use crate::Color;

use std::cmp;
use std::env;
use std::error::Error;
use std::fmt;

pub type PixelBuffer = Vec<Vec<bool>>;
pub type ColorBuffer = Vec<Vec<Color>>;
pub type TextBuffer = Vec<Vec<String>>;
type BrailleChar = char;
type PixelBlock = [[bool; 2]; 4];
type BrailleMap = [[u32; 2]; 4];

const ON: bool = true;
const OFF: bool = false;

/// Arbitrary upper bound: max `usize` on a 16-bit system.
/// This way, our `i32 as usize` conversions (userland to internal) are
/// guaranteed to work (min resolution is 1×1).
const MAX_RESOLUTION: i32 = u16::MAX as i32; // 65_535 < 2_147_483_647

const BRAILLE_UNICODE_0: u32 = 0x2800;
#[rustfmt::skip]
const BRAILLE_UNICODE_OFFSET_MAP: BrailleMap = [
    [0x1, 0x8],
    [0x2, 0x10],
    [0x4, 0x20],
    [0x40, 0x80],
];

/// Helper to convert user-facing `i32`s to internal `usize`s.
macro_rules! to_usize {
    ($value:expr) => {
        // User-facing values should always be bounds-checked before
        // doing anything.
        usize::try_from($value).expect("the value should be bounds-checked prior to the conversion")
    };
}

/// Helper to convert internal `usize`s to user-facing `i32`s.
macro_rules! to_i32 {
    ($value:expr) => {
        // `usize` cannot be less than 0, so if the conversion fails,
        // the value is too large towards +inf. This doesn't make sense,
        // because `MAX_RESOLUTION` is capped at `u16`, which is smaller
        // than `i32`. And `usize` is internal, this macro should never
        // be called on user-provided values.
        i32::try_from($value).expect("the value should not exceed max resolution.")
    };
}

#[derive(Debug)]
pub struct TextCanvasError(pub &'static str);

impl fmt::Display for TextCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TextCanvasError {}

/// Grid-like area with a width and a height.
///
/// This is an abstract way to define the renderable buffers.
#[derive(Debug)]
pub struct Surface {
    width: i32,
    height: i32,
}

impl Surface {
    #[must_use]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[must_use]
    pub fn uwidth(&self) -> usize {
        to_usize!(self.width)
    }

    #[must_use]
    pub fn uheight(&self) -> usize {
        to_usize!(self.height)
    }

    #[must_use]
    pub fn fwidth(&self) -> f64 {
        f64::from(self.width)
    }

    #[must_use]
    pub fn fheight(&self) -> f64 {
        f64::from(self.height)
    }
}

#[derive(Debug)]
pub struct IterPixelBuffer<T> {
    width: T,
    height: T,
    x: T,
    y: T,
}

impl IterPixelBuffer<usize> {
    fn new(buffer: &PixelBuffer) -> Self {
        Self {
            width: buffer[0].len(),
            height: buffer.len(),
            x: 0usize,
            y: 0usize,
        }
    }
}

impl IterPixelBuffer<i32> {
    fn new(buffer: &PixelBuffer) -> Self {
        Self {
            width: to_i32!(buffer[0].len()),
            height: to_i32!(buffer.len()),
            x: 0i32,
            y: 0i32,
        }
    }
}

impl<T> Iterator for IterPixelBuffer<T>
where
    T: From<u8> + Copy + PartialOrd + std::ops::AddAssign,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        let pixel = (self.x, self.y);

        self.x += T::from(1);
        if self.x >= self.width {
            self.y += T::from(1);
            self.x = T::from(0);
        }

        Some(pixel)
    }
}

#[derive(Debug)]
struct IterPixelBufferByBlocksLRTB<'a> {
    buffer: &'a PixelBuffer,
    screen: &'a Surface,
    x: usize,
    y: usize,
}

impl<'a> IterPixelBufferByBlocksLRTB<'a> {
    fn new(buffer: &'a PixelBuffer, screen: &'a Surface) -> Self {
        Self {
            buffer,
            screen,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for IterPixelBufferByBlocksLRTB<'_> {
    type Item = PixelBlock;

    /// Advance block by block (2x4), left-right, top-bottom.
    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.screen.uheight() {
            return None;
        }

        #[rustfmt::skip]
        #[allow(clippy::identity_op)]
        let block = [
            [self.buffer[self.y + 0][self.x + 0], self.buffer[self.y + 0][self.x + 1]],
            [self.buffer[self.y + 1][self.x + 0], self.buffer[self.y + 1][self.x + 1]],
            [self.buffer[self.y + 2][self.x + 0], self.buffer[self.y + 2][self.x + 1]],
            [self.buffer[self.y + 3][self.x + 0], self.buffer[self.y + 3][self.x + 1]],
        ];

        self.x += 2;
        if self.x >= self.screen.uwidth() {
            self.y += 4;
            self.x = 0;
        }

        Some(block)
    }
}

/// Draw to the terminal like an HTML Canvas.
///
/// # Examples
///
/// ```rust
/// use textcanvas::TextCanvas;
///
/// let mut canvas = TextCanvas::new(15, 5);
///
/// assert_eq!(
///     canvas.repr(),
///     "Canvas(output=(15×5), screen=(30×20)))"
/// );
///
/// assert_eq!(
///     (canvas.w(), canvas.h(), canvas.cx(), canvas.cy()),
///     (29, 19, 15, 10),
/// );
///
/// canvas.stroke_line(0, 0, canvas.w(), canvas.h());
/// canvas.draw_text("hello, world", 1, 2);
/// assert_eq!(
///     canvas.to_string(),
///     "\
/// ⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀hello,⠢world⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄
/// "
/// );
/// ```
#[derive(Debug)]
pub struct TextCanvas {
    /// Properties of the output surface, whose size is given as
    /// parameter to the constructor. One unit in width and in height
    /// represents exactly one character on the terminal.
    pub output: Surface,
    /// Properties of the virtual output surface. This surface benefits
    /// from the increase in resolution. One unit in width and in height
    /// represents one Braille dot. There are 2 dots per character in
    /// width, and 4 per character in height. So this surface is 2×
    /// wider and 4× higher than the output surface.
    pub screen: Surface,
    /// The in-memory pixel buffer. This maps 1-to-1 to the virtual
    /// screen. Each pixel is this buffer is either _on_ or _off_.
    pub buffer: PixelBuffer,
    /// The in-memory color buffer. This maps 1-to-1 to the output
    /// buffer (and so to the physical screen). This contains color data
    /// for characters. Screen dots, being part of one output character,
    /// cannot be colored individually. Color is thus less precise than
    /// screen pixels. Note that a call to `set_color()` is valid for
    /// both pixels and text, but text embeds color in its own buffer,
    /// and does not use this buffer at all. Note also that this buffer
    /// is empty until the first call to `set_color()`. The first call
    /// to `set_color()` initializes the buffer and sets
    /// `is_colorized()` to `true`.
    pub color_buffer: ColorBuffer,
    /// The in-memory text buffer. This maps 1-to-1 to the output buffer
    /// (and so to the physical screen). This contains regular text
    /// characters. Text is drawn on top of the pixel buffer on a
    /// separate layer. Drawing text does not affect pixels. Pixels and
    /// text do not share the same color buffer either. Color info is
    /// embedded in the text buffer with each character directly. Note
    /// also that this buffer is empty until the first call to
    /// `draw_text()`. The first call to `draw_text()` initializes the
    /// buffer and sets `is_textual()` to `True`.
    pub text_buffer: TextBuffer,
    /// Inverted drawing mode. In inverted mode, functions which usually
    /// turn pixels _on_, will turn them _off_, and vice-versa.
    pub is_inverted: bool,

    color: Color,
}

impl TextCanvas {
    /// Create new `TextCanvas`.
    ///
    /// # Panics
    ///
    /// If width and height of canvas are < 1×1.
    ///
    /// Also, if the pixel resolution of width or height is larger than
    /// `65_535` (realistically, should not happen). This is because we
    /// let the user interact with the canvas with `i32`s. `i32` is a
    /// more likely user-type (maths, etc.), and allows for negative
    /// values. Even though they are never displayed, allowing negative
    /// values makes for a nicer API. _We_ handle the complexity for the
    /// user.
    #[must_use]
    pub fn new(width: i32, height: i32) -> Self {
        assert!(
            Self::check_canvas_size(width, height),
            "TextCanvas' minimal size is 1×1."
        );

        let mut canvas = Self {
            output: Surface { width, height },
            screen: Surface {
                width: width * 2,
                height: height * 4,
            },
            buffer: Vec::new(),
            color_buffer: Vec::new(),
            text_buffer: Vec::new(),
            is_inverted: false,
            color: Color::new(),
        };

        canvas.init_buffer();

        canvas
    }

    /// Ensure user `i32s` can safely be cast to internal `usize`.
    fn check_canvas_size(width: i32, height: i32) -> bool {
        width > 0 && width <= MAX_RESOLUTION / 2 && height > 0 && height <= MAX_RESOLUTION / 4
    }

    fn check_output_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.output.width() && y >= 0 && y < self.output.height()
    }

    fn check_screen_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.screen.width() && y >= 0 && y < self.screen.height()
    }

    fn init_buffer(&mut self) {
        let empty_row = vec![OFF; self.screen.uwidth()];
        self.buffer = vec![empty_row; self.screen.uheight()];
    }

    /// Create new `TextCanvas` by reading size from environment.
    ///
    /// # Errors
    ///
    /// If either or both `WIDTH` and `HEIGHT` variables cannot be read
    /// from the environment.
    pub fn new_auto() -> Result<Self, TextCanvasError> {
        let (width, height) = Self::get_auto_size()?;
        Ok(Self::new(width, height))
    }

    /// Default canvas size.
    ///
    /// This value is used by [`TextCanvas::default()`], but it may be
    /// useful to query it separately.
    #[must_use]
    pub fn get_default_size() -> (i32, i32) {
        (80, 24)
    }

    /// Read canvas size from `WIDTH` and `HEIGHT` env variables.
    ///
    /// This value is used by [`TextCanvas::new_auto()`], but it may be
    /// useful to query it separately.
    ///
    /// # Errors
    ///
    /// If either or both `WIDTH` and `HEIGHT` variables cannot be read
    /// from the environment.
    pub fn get_auto_size() -> Result<(i32, i32), TextCanvasError> {
        let Some(width) = env::var("WIDTH").ok().and_then(|w| w.parse().ok()) else {
            return Err(TextCanvasError(
                "cannot read terminal width from environment",
            ));
        };

        let Some(height) = env::var("HEIGHT").ok().and_then(|h| h.parse().ok()) else {
            return Err(TextCanvasError(
                "cannot read terminal height from environment",
            ));
        };

        Ok((width, height))
    }

    /// High-level string representation of the canvas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// assert_eq!(
    ///     canvas.repr(),
    ///     "Canvas(output=(15×5), screen=(30×20)))"
    /// );
    /// ```
    #[must_use]
    pub fn repr(&self) -> String {
        let out_w = self.output.width();
        let out_h = self.output.height();
        let screen_w = self.screen.width();
        let screen_h = self.screen.height();
        format!("Canvas(output=({out_w}×{out_h}), screen=({screen_w}×{screen_h})))")
    }

    /// Shortcut for width of pixel screen (index of last column).
    #[must_use]
    pub fn w(&self) -> i32 {
        self.screen.width() - 1
    }

    /// Shortcut for width of pixel screen (index of last column).
    #[must_use]
    pub fn uw(&self) -> usize {
        to_usize!(self.w())
    }

    /// Shortcut for width of pixel screen (index of last column).
    #[must_use]
    pub fn fw(&self) -> f64 {
        f64::from(self.w())
    }

    /// Shortcut for height of pixel screen (index of last row).
    #[must_use]
    pub fn h(&self) -> i32 {
        self.screen.height() - 1
    }

    /// Shortcut for height of pixel screen (index of last row).
    #[must_use]
    pub fn uh(&self) -> usize {
        to_usize!(self.h())
    }

    /// Shortcut for height of pixel screen (index of last row).
    #[must_use]
    pub fn fh(&self) -> f64 {
        f64::from(self.h())
    }

    /// Shortcut for center-X of pixel screen.
    #[must_use]
    pub fn cx(&self) -> i32 {
        self.screen.width() / 2
    }

    /// Shortcut for center-X of pixel screen.
    #[must_use]
    pub fn ucx(&self) -> usize {
        to_usize!(self.cx())
    }

    /// Shortcut for center-X of pixel screen.
    #[must_use]
    pub fn fcx(&self) -> f64 {
        f64::from(self.cx())
    }

    /// Shortcut for center-Y of pixel screen.
    #[must_use]
    pub fn cy(&self) -> i32 {
        self.screen.height() / 2
    }

    /// Shortcut for center-Y of pixel screen.
    #[must_use]
    pub fn ucy(&self) -> usize {
        to_usize!(self.cy())
    }

    /// Shortcut for center-Y of pixel screen.
    #[must_use]
    pub fn fcy(&self) -> f64 {
        f64::from(self.cy())
    }

    /// Turn all pixels off and remove color and text.
    ///
    /// Note: This method does not drop the color and text buffers, it
    /// only clears them. No memory is freed, and all references remain
    /// valid (buffers are cleared in-place, not replaced).
    ///
    /// Note: `clear()` is not affected by inverted mode, it works on a
    /// lower level.
    pub fn clear(&mut self) {
        self.clear_buffer();
        self.clear_color_buffer();
        self.clear_text_buffer();
    }

    fn clear_buffer(&mut self) {
        for (x, y) in self.uiter_buffer() {
            self.buffer[y][x] = OFF;
        }
    }

    fn clear_color_buffer(&mut self) {
        if self.is_colorized() {
            for y in 0..self.color_buffer.len() {
                for x in 0..self.color_buffer[0].len() {
                    self.color_buffer[y][x] = Color::new();
                }
            }
        }
    }

    fn clear_text_buffer(&mut self) {
        if self.is_textual() {
            for y in 0..self.text_buffer.len() {
                for x in 0..self.text_buffer[0].len() {
                    self.text_buffer[y][x] = String::new();
                }
            }
        }
    }

    /// Turn all pixels on.
    ///
    /// This does not affect the color and text buffers.
    ///
    /// Note: `fill()` is not affected by inverted mode, it works on a
    /// lower level.
    pub fn fill(&mut self) {
        for (x, y) in self.uiter_buffer() {
            self.buffer[y][x] = ON;
        }
    }

    /// Invert drawing mode.
    ///
    /// In inverted mode, functions that usually turn pixels _on_, will
    /// turn them _off_, and vice versa. This can be used to cut out
    /// shapes for instance.
    pub fn invert(&mut self) {
        self.is_inverted = !self.is_inverted;
    }

    /// Whether the canvas can contain colors.
    ///
    /// Note: This does not mean that any colors are displayed. This
    /// only means the color buffer is active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{Color, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// assert!(!canvas.is_colorized());
    ///
    /// canvas.set_color(&Color::new().fix());  // Buffer is initialized.
    /// assert!(canvas.is_colorized());
    /// ```
    #[must_use]
    pub fn is_colorized(&self) -> bool {
        !self.color_buffer.is_empty()
    }

    /// Whether the canvas can contain text.
    ///
    /// Note: This does not mean that any text is displayed. This only
    /// means the text buffer is active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// assert!(!canvas.is_textual());
    ///
    /// canvas.draw_text("", 0, 0);  // Buffer is initialized.
    /// assert!(canvas.is_textual());
    /// ```
    #[must_use]
    pub fn is_textual(&self) -> bool {
        !self.text_buffer.is_empty()
    }

    /// Set context color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{Color, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(3, 1);
    /// let green = Color::new().bright_green().fix();
    ///
    /// canvas.set_color(&green);
    /// assert!(canvas.is_colorized());
    ///
    /// canvas.draw_text("foo", 0, 0);
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\x1b[0;92mf\x1b[0m\x1b[0;92mo\x1b[0m\x1b[0;92mo\x1b[0m\n"
    /// );
    /// ```
    pub fn set_color(&mut self, color: &Color) {
        if !self.is_colorized() {
            self.init_color_buffer();
        }
        self.color = color.clone();
    }

    fn init_color_buffer(&mut self) {
        self.color_buffer = Vec::with_capacity(self.output.uheight());
        for _ in 0..self.output.uheight() {
            let row = vec![Color::new(); self.output.uwidth()];
            self.color_buffer.push(row);
        }
    }

    /// Get the state of a screen pixel.
    ///
    /// `Some(true)` if the pixel is turned _on_, `Some(false)` if it is
    /// turned _off_, and `None` if the coordinates are outside the
    /// bounds of the buffer.
    ///
    /// # Arguments
    ///
    /// - `x` - Screen X (high resolution).
    /// - `y` - Screen Y (high resolution).
    #[must_use]
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<bool> {
        if !self.check_screen_bounds(x, y) {
            return None;
        }
        let (x, y) = (to_usize!(x), to_usize!(y));
        Some(self.buffer[y][x])
    }

    /// Set the state of a screen pixel.
    ///
    /// Note: Coordinates outside the screen bounds are ignored.
    ///
    /// Note: Turning a pixel _off_ also removes color. This side effect
    /// does not affect text, as text has a separate color buffer.
    ///
    /// # Arguments
    ///
    /// - `x` - Screen X (high resolution).
    /// - `y` - Screen Y (high resolution).
    /// - `state` - `true` means _on_, `false` means _off_.
    pub fn set_pixel(&mut self, x: i32, y: i32, mut state: bool) {
        if !self.check_screen_bounds(x, y) {
            return;
        }
        let (x, y) = (to_usize!(x), to_usize!(y));

        if self.is_inverted {
            state = !state;
        }

        self.buffer[y][x] = state;

        if self.is_colorized() {
            if state == ON {
                self.color_pixel(x, y);
            } else {
                self.decolor_pixel(x, y);
            }
        }
    }

    fn color_pixel(&mut self, x: usize, y: usize) {
        self.color_buffer[y / 4][x / 2] = self.color.clone();
    }

    fn decolor_pixel(&mut self, x: usize, y: usize) {
        self.color_buffer[y / 4][x / 2] = Color::new();
    }

    /// Draw text onto the canvas.
    ///
    /// Note: Spaces are transparent (you see pixels through). But
    /// drawing spaces over text erases the text beneath. If you want to
    /// keep the text, use the [`merge_text()`](TextCanvas::merge_text)
    /// method.
    ///
    /// Note: Coordinates outside the screen bounds are ignored.
    ///
    /// Note: Text is rendered on top of pixels, as a separate layer.
    ///
    /// Note: `set_color()` works for text as well, but text does not
    /// share its color buffer with pixels.
    pub fn draw_text(&mut self, text: &str, mut x: i32, y: i32) {
        if !self.is_textual() {
            self.init_text_buffer();
        }

        for char in text.chars() {
            self.draw_char(char, x, y, false);
            x += 1;
        }
    }

    pub fn draw_text_vertical(&mut self, text: &str, x: i32, mut y: i32) {
        if !self.is_textual() {
            self.init_text_buffer();
        }

        for char in text.chars() {
            self.draw_char(char, x, y, false);
            y += 1;
        }
    }

    /// Merge text onto the canvas.
    ///
    /// This is the same as [`draw_text()`](TextCanvas::draw_text), but
    /// spaces do not erase text underneath.
    pub fn merge_text(&mut self, text: &str, mut x: i32, y: i32) {
        if !self.is_textual() {
            self.init_text_buffer();
        }

        for char in text.chars() {
            self.draw_char(char, x, y, true);
            x += 1;
        }
    }

    pub fn merge_text_vertical(&mut self, text: &str, x: i32, mut y: i32) {
        if !self.is_textual() {
            self.init_text_buffer();
        }

        for char in text.chars() {
            self.draw_char(char, x, y, true);
            y += 1;
        }
    }

    fn draw_char(&mut self, char: char, x: i32, y: i32, merge: bool) {
        if !self.check_output_bounds(x, y) {
            return;
        }

        let char = if char == ' ' {
            if merge {
                return;
            }
            String::new()
        } else {
            self.color.format(&String::from(char))
        };

        let (ux, uy) = (to_usize!(x), to_usize!(y));
        self.text_buffer[uy][ux] = char;
    }

    fn init_text_buffer(&mut self) {
        self.text_buffer = Vec::with_capacity(self.output.uheight());
        for _ in 0..self.output.uheight() {
            let row = vec![String::new(); self.output.uwidth()];
            self.text_buffer.push(row);
        }
    }

    /// Render canvas as a `print!()`-able string.
    ///
    /// Returns rendered canvas, with pixels, text and colors. Each
    /// canvas row becomes a line of text (lines are separated by
    /// `\n`s), and each canvas column becomes a single character in
    /// each line. What you would expect. It can be printed as-is.
    fn render(&self) -> String {
        let nb_output_chars = (self.output.uwidth() + 1) * self.output.uheight();
        let mut res = String::with_capacity(nb_output_chars);

        for (i, pixel_block) in self.iter_buffer_by_blocks_lrtb().enumerate() {
            let x = i % self.output.uwidth();
            let y = i / self.output.uwidth();

            let text_char = self.get_text_char(x, y);
            // Pixel layer.
            if text_char.is_empty() {
                let braille_char = Self::pixel_block_to_braille_char(pixel_block);
                let braille_char = self.color_pixel_char(x, y, braille_char);
                res.push_str(&braille_char);
            }
            // Text layer.
            else {
                res.push_str(&text_char);
            }

            // If end of line is reached, go to next line.
            if (i + 1) % self.output.uwidth() == 0 {
                res.push('\n');
            }
        }

        res
    }

    fn get_text_char(&self, x: usize, y: usize) -> String {
        if self.is_textual() {
            return self.text_buffer[y][x].clone();
        }
        String::new()
    }

    fn pixel_block_to_braille_char(pixel_block: PixelBlock) -> BrailleChar {
        let mut braille_char = BRAILLE_UNICODE_0;
        // Iterate over individual pixels to turn them on or off.
        for y in 0..4 {
            for x in 0..2 {
                if pixel_block[y][x] == ON {
                    braille_char += BRAILLE_UNICODE_OFFSET_MAP[y][x];
                }
            }
        }
        // Convert Unicode integer value to char.
        char::from_u32(braille_char).expect("from valid hardcoded values")
    }

    fn color_pixel_char(&self, x: usize, y: usize, pixel_char: char) -> String {
        let pixel_char = String::from(pixel_char);
        if self.is_colorized() {
            let color = &self.color_buffer[y][x];
            return color.format(&pixel_char);
        }
        pixel_char
    }

    fn iter_buffer_by_blocks_lrtb(&self) -> IterPixelBufferByBlocksLRTB {
        IterPixelBufferByBlocksLRTB::new(&self.buffer, &self.screen)
    }

    /// Iterate over all cells of the pixel buffer.
    ///
    /// Yields all X/Y coordinates, left-right, top-bottom.
    ///
    /// Values are `i32`, this is meant to be used with the public API.
    #[must_use]
    pub fn iter_buffer(&self) -> IterPixelBuffer<i32> {
        IterPixelBuffer::<i32>::new(&self.buffer)
    }

    /// Iterate over all cells of the pixel buffer.
    ///
    /// Yields all X/Y coordinates, left-right, top-bottom.
    ///
    /// Values are `usize`, this is meant to index directly into the
    /// buffer.
    #[must_use]
    pub fn uiter_buffer(&self) -> IterPixelBuffer<usize> {
        IterPixelBuffer::<usize>::new(&self.buffer)
    }
}

/// Implementation of drawing primitives.
impl TextCanvas {
    /// Stroke line.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.stroke_line(5, 5, 25, 15);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠐⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠉⠒⠤⣀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⠤⣀⠀⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.bresenham_line(x1, y1, x2, y2);
    }

    /// Stroke line using Bresenham's line algorithm.
    fn bresenham_line(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32) {
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut error = dx + dy;

        // Treat vertical and horizontal lines as special cases.
        if dx == 0 {
            let x = x1;
            let from_y = cmp::min(y1, y2);
            let to_y = cmp::max(y1, y2);
            for y in from_y..=to_y {
                self.set_pixel(x, y, true);
            }
            return;
        } else if dy == 0 {
            let y = y1;
            let from_x = cmp::min(x1, x2);
            let to_x = cmp::max(x1, x2);
            for x in from_x..=to_x {
                self.set_pixel(x, y, true);
            }
            return;
        }

        #[cfg(not(tarpaulin_include))]
        loop {
            self.set_pixel(x1, y1, true);
            if x1 == x2 && y1 == y2 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x1 == x2 {
                    break;
                }
                error += dy;
                x1 += sx;
            }
            if e2 <= dx {
                if y1 == y2 {
                    break;
                }
                error += dx;
                y1 += sy;
            }
        }
    }

    /// Stroke rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.stroke_rect(5, 5, 20, 10);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢰⠒⠒⠒⠒⠒⠒⠒⠒⠒⡆⠀⠀
    /// ⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀
    /// ⠀⠀⠸⠤⠤⠤⠤⠤⠤⠤⠤⠤⠇⠀⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let (width, height) = (width - 1, height - 1);
        self.stroke_line(x, y, x + width, y);
        self.stroke_line(x + width, y, x + width, y + height);
        self.stroke_line(x + width, y + height, x, y + height);
        self.stroke_line(x, y + height, x, y);
    }

    /// Draw a border around the canvas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.frame();
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⡏⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹
    /// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
    /// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
    /// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
    /// ⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣸
    /// "
    /// );
    /// ```
    pub fn frame(&mut self) {
        self.stroke_rect(0, 0, self.screen.width(), self.screen.height());
    }

    /// Fill rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.fill_rect(5, 5, 20, 10);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢰⣶⣶⣶⣶⣶⣶⣶⣶⣶⡆⠀⠀
    /// ⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀
    /// ⠀⠀⠸⠿⠿⠿⠿⠿⠿⠿⠿⠿⠇⠀⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        for y in y..y + height {
            self.stroke_line(x, y, x + width - 1, y);
        }
    }

    /// Stroke triangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.stroke_triangle(5, 5, 20, 10, 4, 17);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢰⠢⠤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢸⠀⠀⠀⠈⠉⢒⡢⠄⠀⠀⠀⠀
    /// ⠀⠀⡇⠀⣀⠤⠔⠊⠁⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠓⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        self.stroke_line(x1, y1, x2, y2);
        self.stroke_line(x2, y2, x3, y3);
        self.stroke_line(x3, y3, x1, y1);
    }

    /// Fill triangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.fill_triangle(5, 5, 20, 10, 4, 17);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢰⣦⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢸⣿⣿⣿⣿⣿⣶⡦⠄⠀⠀⠀⠀
    /// ⠀⠀⣿⣿⣿⠿⠟⠋⠁⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        // This makes for neater edges.
        self.stroke_triangle(x1, y1, x2, y2, x3, y3);

        // Barycentric Algorithm: Compute the bounding box of the
        // triangle. Then for each point in the box, determine if it
        // lies inside or outside the triangle.

        // Bounding box.
        let min_x = cmp::min(x1, cmp::min(x2, x3));
        let max_x = cmp::max(x1, cmp::max(x2, x3));
        let min_y = cmp::min(y1, cmp::min(y2, y3));
        let max_y = cmp::max(y1, cmp::max(y2, y3));

        let p1 = (f64::from(x1), f64::from(y1));
        let p2 = (f64::from(x2), f64::from(y2));
        let p3 = (f64::from(x3), f64::from(y3));
        let triangle = (p1, p2, p3);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = (f64::from(x), f64::from(y));
                if Self::is_point_in_triangle(point, triangle) {
                    self.set_pixel(x, y, true);
                }
            }
        }
    }

    #[allow(clippy::similar_names)]
    fn is_point_in_triangle(
        (px, py): (f64, f64),
        ((p0x, p0y), (p1x, p1y), (p2x, p2y)): ((f64, f64), (f64, f64), (f64, f64)),
    ) -> bool {
        // This version correctly handles triangles specified in either
        // winding direction (clockwise vs. counterclockwise).
        // https://stackoverflow.com/a/20861130 — Glenn Slayden
        let s = (p0x - p2x) * (py - p2y) - (p0y - p2y) * (px - p2x);
        let t = (p1x - p0x) * (py - p0y) - (p1y - p0y) * (px - p0x);

        if (s < 0.0) != (t < 0.0) && s != 0.0 && t != 0.0 {
            return false;
        }

        let d = (p2x - p1x) * (py - p1y) - (p2y - p1y) * (px - p1x);

        d == 0.0 || (d < 0.0) == (s + t <= 0.0)
    }

    /// Stroke circle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.stroke_circle(canvas.cx(), canvas.cy(), 7);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⡠⠊⠀⠀⠀⠈⠢⡀⠀⠀⠀
    /// ⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀
    /// ⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⡠⠃⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠈⠒⠒⠒⠊⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_circle(&mut self, x: i32, y: i32, radius: i32) {
        self.bresenham_circle(x, y, radius, false);
    }

    /// Fill circle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.fill_circle(canvas.cx(), canvas.cy(), 7);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣦⡀⠀⠀⠀
    /// ⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
    /// ⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⡿⠃⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠈⠛⠛⠛⠋⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn fill_circle(&mut self, x: i32, y: i32, radius: i32) {
        self.bresenham_circle(x, y, radius, true);
    }

    /// Draw circle using Jesko's Method of the Bresenham's circle
    /// algorithm.
    fn bresenham_circle(&mut self, x: i32, y: i32, radius: i32, fill: bool) {
        let (cx, cy) = (x, y);
        let mut t1 = radius / 16;
        let mut x = radius;
        let mut y = 0;
        while x >= y {
            if fill {
                // Connect each pair of points with the same `y`.
                self.stroke_line(cx - x, cy - y, cx + x, cy - y);
                self.stroke_line(cx + x, cy + y, cx - x, cy + y);
                self.stroke_line(cx - y, cy - x, cx + y, cy - x);
                self.stroke_line(cx + y, cy + x, cx - y, cy + x);
            } else {
                self.set_pixel(cx - x, cy - y, true);
                self.set_pixel(cx + x, cy - y, true);
                self.set_pixel(cx + x, cy + y, true);
                self.set_pixel(cx - x, cy + y, true);
                self.set_pixel(cx - y, cy - x, true);
                self.set_pixel(cx + y, cy - x, true);
                self.set_pixel(cx + y, cy + x, true);
                self.set_pixel(cx - y, cy + x, true);
            }

            y += 1;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0 {
                t1 = t2;
                x -= 1;
            }
        }
    }

    /// Stroke n-gon.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    /// use std::f64::consts::PI;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.stroke_ngon(canvas.cx(), canvas.cy(), 7, 5, PI / 2.0);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⢀⡠⠊⠁⠉⠢⣀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⢇⠀⠀⠀⢀⠎⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠈⠉⠉⠉⠉⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `sides` < 3.
    pub fn stroke_ngon(&mut self, x: i32, y: i32, radius: i32, sides: i32, angle: f64) {
        self.ngon(x, y, radius, sides, angle, false);
    }

    /// Fill n-gon.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// canvas.fill_ngon(canvas.cx(), canvas.cy(), 7, 4, 0.0);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⢀⣴⣿⣷⣄⠀⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⢴⣿⣿⣿⣿⣿⣷⠄⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠙⢿⣿⣿⠟⠁⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠙⠁⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `sides` < 3.
    pub fn fill_ngon(&mut self, x: i32, y: i32, radius: i32, sides: i32, angle: f64) {
        self.ngon(x, y, radius, sides, angle, true);
    }

    fn ngon(&mut self, x: i32, y: i32, radius: i32, sides: i32, angle: f64, fill: bool) {
        assert!(
            sides >= 3,
            "Minimum 3 sides needed to draw an n-gon, but only {sides} requested."
        );

        let mut join_vertices = |from: &(i32, i32), to: &(i32, i32)| {
            if fill {
                self.fill_triangle(self.cx(), self.cy(), from.0, from.1, to.0, to.1);
            } else {
                self.stroke_line(from.0, from.1, to.0, to.1);
            }
        };

        let vertices = Self::compute_ngon_vertices(x, y, radius, sides, angle);
        let mut vertices = vertices.iter();

        let first = vertices.next().expect("there are at least 3 vertex");
        let mut previous = first;
        for vertex in vertices {
            join_vertices(previous, vertex);
            previous = vertex;
        }
        join_vertices(previous, first);
    }

    #[allow(clippy::cast_possible_truncation)]
    fn compute_ngon_vertices(
        x: i32,
        y: i32,
        radius: i32,
        sides: i32,
        angle: f64,
    ) -> Vec<(i32, i32)> {
        let cx = f64::from(x);
        let cy = f64::from(y);
        let radius = f64::from(radius);
        let slice = (2.0 * std::f64::consts::PI) / f64::from(sides);

        let mut vertices: Vec<(i32, i32)> = Vec::with_capacity(to_usize!(sides));
        for vertex in 0..sides {
            let theta = f64::from(vertex) * slice + angle;
            let x = cx + (theta.cos() * radius);
            let y = cy - (theta.sin() * radius); // Screen Y coordinates are inverted.
            let point = (x.round() as i32, y.round() as i32);
            vertices.push(point);
        }
        vertices
    }

    /// Draw another canvas onto the current canvas.
    ///
    /// The other canvas completely overrides the current canvas where
    /// it is drawn (but it does not affect the portions where it is
    /// _not_ drawn).
    ///
    /// Note: Inverted mode has no effect here, this is a low level
    /// copy-paste.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    /// canvas.stroke_line(0, 0, canvas.w(), canvas.h());
    /// canvas.stroke_line(0, canvas.h(), canvas.w(), 0);
    ///
    /// let mut overlay = TextCanvas::new(7, 3);
    /// overlay.frame();
    ///
    /// canvas.draw_canvas(&overlay, 8, 4);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠔⠊
    /// ⠀⠀⠀⠑⡏⠉⠉⠉⠉⠉⢹⠊⠀⠀⠀
    /// ⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀
    /// ⠀⠀⠀⡠⣇⣀⣀⣀⣀⣀⣸⢄⠀⠀⠀
    /// ⡠⠔⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄
    /// "
    /// );
    /// ```
    pub fn draw_canvas(&mut self, canvas: &Self, dx: i32, dy: i32) {
        self.draw_canvas_onto_canvas(canvas, dx, dy, false);
    }

    /// Merge another canvas with the current canvas.
    ///
    /// The other canvas is merged with the current canvas. That is,
    /// pixels that are turned on get draw, but those that are off are
    /// ignored.
    ///
    /// Note: Inverted mode has no effect here, this is a low level
    /// copy-paste.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    /// canvas.stroke_line(0, 0, canvas.w(), canvas.h());
    /// canvas.stroke_line(0, canvas.h(), canvas.w(), 0);
    ///
    /// let mut overlay = TextCanvas::new(7, 3);
    /// overlay.frame();
    ///
    /// canvas.merge_canvas(&overlay, 8, 4);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠔⠊
    /// ⠀⠀⠀⠑⡯⣉⠉⠉⠉⣉⢽⠊⠀⠀⠀
    /// ⠀⠀⠀⠀⡇⠀⡱⠶⢎⠀⢸⠀⠀⠀⠀
    /// ⠀⠀⠀⡠⣗⣉⣀⣀⣀⣉⣺⢄⠀⠀⠀
    /// ⡠⠔⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄
    /// "
    /// );
    /// ```
    pub fn merge_canvas(&mut self, canvas: &Self, dx: i32, dy: i32) {
        self.draw_canvas_onto_canvas(canvas, dx, dy, true);
    }

    fn draw_canvas_onto_canvas(&mut self, canvas: &Self, dx: i32, dy: i32, merge: bool) {
        if !self.is_colorized() && canvas.is_colorized() {
            self.init_color_buffer();
        }

        if !self.is_textual() && canvas.is_textual() {
            self.init_text_buffer();
        }

        // We cannot convert `offset_x` and `offset_y` to `usize` yet,
        // because negative values are possible here (you can draw the
        // canvas out of bounds). The conversion must be made at the
        // pixel level, because we can't draw pixels out of bound.
        let (offset_x, offset_y) = (dx, dy);

        for (x, y) in canvas.uiter_buffer() {
            // Source coordinates of pixel.
            // x, y

            // Destination coordinates of pixel.
            let (dx, dy) = (offset_x + to_i32!(x), offset_y + to_i32!(y));
            if !self.check_screen_bounds(dx, dy) {
                continue;
            }
            // Here we are safe. If a pixel is within the screen bounds,
            // it can safely be converted to buffer coordinates in
            // `usize`. And, we can also safely convert it to output
            // coordinates (`x / 2`, `y / 4`) later.
            let (dx, dy) = (to_usize!(dx), to_usize!(dy));

            // Pixels.
            let pixel = canvas.buffer[y][x];
            // In merge mode, only draw if pixel is on, treating off
            // pixels as transparent.
            if !merge || pixel == ON {
                self.buffer[dy][dx] = pixel;

                if canvas.is_colorized() {
                    let color = canvas.color_buffer[y / 4][x / 2].clone();
                    self.color_buffer[dy / 4][dx / 2] = color;
                }
            }

            // Text.
            if canvas.is_textual() {
                // Text buffer has color embedded into the String.
                let text = canvas.text_buffer[y / 4][x / 2].clone();

                if !merge || !text.is_empty() {
                    self.text_buffer[dy / 4][dx / 2] = text;
                }
            }
        }
    }
}

impl Default for TextCanvas {
    fn default() -> Self {
        let (width, heigt) = Self::get_default_size();
        Self::new(width, heigt)
    }
}

impl fmt::Display for TextCanvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::explicit_counter_loop)]
    fn stroke_line_accros_canvas(canvas: &mut TextCanvas) {
        let mut y = 0;
        for x in 0..canvas.screen.width() {
            canvas.set_pixel(x, y, true);
            y += 1;
        }
    }

    // Errors.

    #[test]
    fn textcanvaserror_format() {
        let error = TextCanvasError("an error has occurred");

        assert_eq!(error.to_string(), "an error has occurred");
    }

    // Surface.

    #[test]
    fn size() {
        let surface = Surface {
            width: 15,
            height: 9,
        };

        assert_eq!(surface.width(), 15);
        assert_eq!(surface.height(), 9);
    }

    #[test]
    fn size_unsigned() {
        let surface = Surface {
            width: 15,
            height: 9,
        };

        assert_eq!(surface.uwidth(), 15);
        assert_eq!(surface.uheight(), 9);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn size_float() {
        let surface = Surface {
            width: 15,
            height: 9,
        };

        assert_eq!(surface.fwidth(), 15.0);
        assert_eq!(surface.fheight(), 9.0);
    }

    // Canvas.

    #[test]
    fn output_size() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(canvas.output.width, 7, "Incorrect output width.");
        assert_eq!(canvas.output.height, 4, "Incorrect output height.");
    }

    #[test]
    fn screen_size() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(canvas.screen.width, 7 * 2, "Incorrect output width.");
        assert_eq!(canvas.screen.height, 4 * 4, "Incorrect output height.");
    }

    #[test]
    fn buffer_size() {
        let canvas = TextCanvas::new(7, 4);
        let buffer_width = canvas.buffer[0].len();
        let buffer_height = canvas.buffer.len();

        assert_eq!(buffer_width, 7 * 2, "Incorrect number of rows in buffer.");
        assert_eq!(
            buffer_height,
            4 * 4,
            "Incorrect number of columns in buffer."
        );
    }

    #[test]
    fn default_size() {
        let canvas = TextCanvas::default();

        assert_eq!(canvas.output.width, 80, "Incorrect default width.");
        assert_eq!(canvas.output.height, 24, "Incorrect default height.");
    }

    #[test]
    fn get_default_size() {
        let (width, height) = TextCanvas::get_default_size();

        assert_eq!(width, 80, "Incorrect default width.");
        assert_eq!(height, 24, "Incorrect default height.");
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_zero_panics_for_width() {
        let _ = TextCanvas::new(0, 1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_zero_panics_for_height() {
        let _ = TextCanvas::new(1, 0);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_zero_panics_for_width_and_height() {
        let _ = TextCanvas::new(0, 0);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_negative_panics_for_width() {
        let _ = TextCanvas::new(-1, 1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_negative_panics_for_height() {
        let _ = TextCanvas::new(1, -1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_negative_panics_for_width_and_height() {
        let _ = TextCanvas::new(-1, -1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_too_big_panics_for_width() {
        let _ = TextCanvas::new(100_000, 1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_too_big_panics_for_height() {
        let _ = TextCanvas::new(1, 100_000);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn size_too_big_panics_for_width_and_height() {
        let _ = TextCanvas::new(100_000, 100_000);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn max_i32_does_not_overflow_width() {
        // There was an error in the bounds checking condition:
        //
        //     if width * 2 <= MAX_RESOLUTION
        //
        // This panics if `size * 2` > `i32::MAX`, with `attempt to
        // multiply with overflow`. The solution is to divide instead:
        //
        //     if width <= MAX_RESOLUTION / 2
        let _ = TextCanvas::new(i32::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "TextCanvas' minimal size is 1×1.")]
    fn max_i32_does_not_overflow_height() {
        // There was an error in the bounds checking condition:
        //
        //     if height * 4 <= MAX_RESOLUTION
        //
        // This panics if `size * 4` > `i32::MAX`, with `attempt to
        // multiply with overflow`. The solution is to divide instead:
        //
        //     if height <= MAX_RESOLUTION / 4
        let _ = TextCanvas::new(1, i32::MAX);
    }

    #[test]
    fn auto_size() {
        // This is fine, as long as this is the only test that modifies
        // the environment.
        env::remove_var("WIDTH");
        env::remove_var("HEIGHT");

        assert!(
            TextCanvas::new_auto().is_err(),
            "`WIDTH` and `HEIGHT` don't exist."
        );
        assert!(TextCanvas::get_auto_size().is_err());

        env::set_var("WIDTH", "1");
        env::set_var("HEIGHT", "2147483648");

        assert!(
            TextCanvas::new_auto().is_err(),
            "`HEIGHT` is too large for an `i32`."
        );
        assert!(TextCanvas::get_auto_size().is_err());

        env::set_var("WIDTH", "abc");
        env::set_var("HEIGHT", "1");

        assert!(TextCanvas::new_auto().is_err(), "`WIDTH` is not a number.");
        assert!(TextCanvas::get_auto_size().is_err());

        env::set_var("WIDTH", "1");
        env::set_var("HEIGHT", "abc");

        assert!(TextCanvas::new_auto().is_err(), "`HEIGHT` is not a number.");
        assert!(TextCanvas::get_auto_size().is_err());

        env::set_var("WIDTH", "12");
        env::set_var("HEIGHT", "5");

        let canvas = TextCanvas::new_auto().unwrap();

        assert_eq!(canvas.output.width, 12, "Incorrect auto width.");
        assert_eq!(canvas.output.height, 5, "Incorrect auto height.");
        assert_eq!(TextCanvas::get_auto_size().unwrap(), (12, 5));
    }

    #[test]
    fn string_representation() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(
            canvas.to_string(),
            format!("{canvas}"),
            "Incorrect string representation."
        );

        assert_eq!(
            canvas.repr(),
            "Canvas(output=(7×4), screen=(14×16)))",
            "Incorrect string representation.",
        );
    }

    #[test]
    fn shortcuts() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(canvas.w(), 13, "Incorrect screen width.");
        assert_eq!(canvas.h(), 15, "Incorrect screen height.");
        assert_eq!(canvas.cx(), 7, "Incorrect screen center-X.");
        assert_eq!(canvas.cy(), 8, "Incorrect screen center-Y.");
    }

    #[test]
    fn shortcuts_unsigned() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(canvas.uw(), 13, "Incorrect screen width.");
        assert_eq!(canvas.uh(), 15, "Incorrect screen height.");
        assert_eq!(canvas.ucx(), 7, "Incorrect screen center-X.");
        assert_eq!(canvas.ucy(), 8, "Incorrect screen center-Y.");
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn shortcuts_float() {
        let canvas = TextCanvas::new(7, 4);

        assert_eq!(canvas.fw(), 13.0, "Incorrect screen width.");
        assert_eq!(canvas.fh(), 15.0, "Incorrect screen height.");
        assert_eq!(canvas.fcx(), 7.0, "Incorrect screen center-X.");
        assert_eq!(canvas.fcy(), 8.0, "Incorrect screen center-Y.");
    }

    #[test]
    fn check_output_bounds() {
        let canvas = TextCanvas::new(7, 4);

        assert!(canvas.check_output_bounds(0, 0));
        assert!(canvas.check_output_bounds(6, 0));
        assert!(canvas.check_output_bounds(6, 3));
        assert!(canvas.check_output_bounds(0, 3));

        assert!(!canvas.check_output_bounds(0, -1));
        assert!(!canvas.check_output_bounds(7, 0));
        assert!(!canvas.check_output_bounds(6, 4));
        assert!(!canvas.check_output_bounds(-1, 3));
    }

    #[test]
    fn check_screen_bounds() {
        let canvas = TextCanvas::new(7, 4);

        assert!(canvas.check_screen_bounds(0, 0));
        assert!(canvas.check_screen_bounds(13, 0));
        assert!(canvas.check_screen_bounds(13, 15));
        assert!(canvas.check_screen_bounds(0, 15));

        assert!(!canvas.check_screen_bounds(0, -1));
        assert!(!canvas.check_screen_bounds(14, 0));
        assert!(!canvas.check_screen_bounds(13, 16));
        assert!(!canvas.check_screen_bounds(-1, 15));
    }

    #[test]
    fn turn_all_pixels_on() {
        let mut canvas = TextCanvas::new(2, 2);

        for x in 0..canvas.screen.width() {
            for y in 0..canvas.screen.height() {
                canvas.set_pixel(x, y, true);
            }
        }

        assert_eq!(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not fully on.");
    }

    #[test]
    fn get_pixel() {
        let mut canvas = TextCanvas::new(2, 2);

        assert_eq!(
            canvas.get_pixel(3, 2),
            Some(false),
            "Pixel should be turned off."
        );

        canvas.set_pixel(3, 2, true);

        assert_eq!(
            canvas.get_pixel(3, 2),
            Some(true),
            "Pixel should be turned on."
        );
    }

    #[test]
    fn get_pixel_with_overflow() {
        let canvas = TextCanvas::new(1, 1);

        assert_eq!(canvas.get_pixel(-1, 0), None, "Overflow should be None.");
        assert_eq!(canvas.get_pixel(0, -1), None, "Overflow should be None.");
        assert_eq!(canvas.get_pixel(-1, -1), None, "Overflow should be None.");

        assert_eq!(
            canvas.get_pixel(canvas.screen.width(), 0),
            None,
            "Overflow should be None."
        );
        assert_eq!(
            canvas.get_pixel(0, canvas.screen.height()),
            None,
            "Overflow should be None."
        );
        assert_eq!(
            canvas.get_pixel(canvas.screen.width(), canvas.screen.height()),
            None,
            "Overflow should be None.",
        );
    }

    #[test]
    fn get_pixel_on_boundaries() {
        let mut canvas = TextCanvas::new(1, 1);

        canvas.buffer = vec![
            vec![true, false],
            vec![false, false],
            vec![false, false],
            vec![false, true],
        ];

        assert_eq!(canvas.get_pixel(0, 0), Some(true), "Incorrect pixel value.");
        assert_eq!(
            canvas.get_pixel(canvas.screen.width() - 1, canvas.screen.height() - 1),
            Some(true),
            "Incorrect pixel value.",
        );
    }

    #[test]
    fn set_pixel() {
        let mut canvas = TextCanvas::new(3, 2);
        stroke_line_accros_canvas(&mut canvas);

        assert_eq!(
            canvas.buffer,
            [
                [true, false, false, false, false, false],
                [false, true, false, false, false, false],
                [false, false, true, false, false, false],
                [false, false, false, true, false, false],
                [false, false, false, false, true, false],
                [false, false, false, false, false, true],
                [false, false, false, false, false, false],
                [false, false, false, false, false, false],
            ],
            "Incorrect buffer content.",
        );
    }

    #[test]
    fn set_pixel_with_overflow() {
        let mut canvas = TextCanvas::new(1, 1);

        canvas.set_pixel(-1, 0, true);
        canvas.set_pixel(0, -1, true);
        canvas.set_pixel(-1, -1, true);

        canvas.set_pixel(canvas.screen.width(), 0, true);
        canvas.set_pixel(0, canvas.screen.height(), true);
        canvas.set_pixel(canvas.screen.width(), canvas.screen.height(), true);

        assert_eq!(
            canvas.buffer,
            [
                [false, false],
                [false, false],
                [false, false],
                [false, false],
            ],
            "No pixel should be turned on.",
        );
    }

    #[test]
    fn set_pixel_on_boundaries() {
        let mut canvas = TextCanvas::new(1, 1);

        canvas.set_pixel(0, 0, true);
        canvas.set_pixel(canvas.screen.width() - 1, canvas.screen.height() - 1, true);

        assert_eq!(
            canvas.buffer,
            [[true, false], [false, false], [false, false], [false, true],],
            "Incorrect buffer content.",
        );
    }

    #[test]
    fn get_as_string() {
        let mut canvas = TextCanvas::new(3, 2);
        stroke_line_accros_canvas(&mut canvas);

        assert_eq!(canvas.to_string(), "⠑⢄⠀\n⠀⠀⠑\n", "Incorrect output string.");
    }

    #[test]
    fn clear() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.fill();

        canvas.clear();

        assert_eq!(canvas.to_string(), "⠀⠀\n⠀⠀\n", "Output not empty.");
    }

    #[test]
    fn clear_edits_buffer_in_place() {
        let mut canvas = TextCanvas::new(1, 1);

        let buffer = canvas.buffer.as_ptr();
        let row_0 = canvas.buffer[0].as_ptr();
        let row_1 = canvas.buffer[1].as_ptr();
        let row_2 = canvas.buffer[2].as_ptr();
        let row_3 = canvas.buffer[3].as_ptr();

        canvas.clear();

        assert_eq!(
            buffer,
            canvas.buffer.as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_0,
            canvas.buffer[0].as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_1,
            canvas.buffer[1].as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_2,
            canvas.buffer[2].as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_3,
            canvas.buffer[3].as_ptr(),
            "Container should be the same as before."
        );
    }

    #[test]
    fn fill() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.fill();

        assert_eq!(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not full.");
    }

    #[test]
    fn invert() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_rect(6, 3, 20, 15);

        assert!(!canvas.is_inverted);

        canvas.invert();
        canvas.fill_rect(9, 6, 14, 9);

        assert!(canvas.is_inverted);
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀
⠀⠀⠀⣿⡟⠛⠛⠛⠛⠛⠛⢻⣿⠀⠀
⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀
⠀⠀⠀⣿⣇⣀⣀⣀⣀⣀⣀⣸⣿⠀⠀
⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀
"
        );
    }

    #[test]
    fn double_invert() {
        let mut canvas = TextCanvas::new(15, 5);

        assert!(!canvas.is_inverted);

        canvas.invert();
        assert!(canvas.is_inverted);

        canvas.invert();
        assert!(!canvas.is_inverted);
    }

    #[test]
    fn clear_not_affected_by_invert() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.invert();
        canvas.clear();

        assert_eq!(canvas.to_string(), "⠀⠀\n⠀⠀\n", "Output not empty.");
    }

    #[test]
    fn fill_not_affected_by_invert() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.invert();
        canvas.fill();

        assert_eq!(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not full.");
    }

    #[test]
    fn iter_buffer_by_blocks_lrtb() {
        // This tests a private method, but this method is at the core
        // of the output generation. Testing it helps ensure stability.
        let mut canvas = TextCanvas::new(3, 2);
        stroke_line_accros_canvas(&mut canvas);

        assert_eq!(
            canvas.iter_buffer_by_blocks_lrtb().collect::<Vec<_>>(),
            [
                [[true, false], [false, true], [false, false], [false, false],],
                [[false, false], [false, false], [true, false], [false, true],],
                [
                    [false, false],
                    [false, false],
                    [false, false],
                    [false, false],
                ],
                [
                    [false, false],
                    [false, false],
                    [false, false],
                    [false, false],
                ],
                [
                    [false, false],
                    [false, false],
                    [false, false],
                    [false, false],
                ],
                [[true, false], [false, true], [false, false], [false, false],],
            ],
            "Incorrect list of blocks.",
        );
    }

    #[test]
    fn iter_buffer() {
        let canvas = TextCanvas::new(3, 2);

        #[rustfmt::skip]
        assert_eq!(canvas.iter_buffer().collect::<Vec<_>>(), [
            (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
            (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
            (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
            (0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5),
            (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6),
            (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
        ], "Incorrect X and Y pairs, or in wrong order.");
    }

    #[test]
    fn uiter_buffer() {
        let canvas = TextCanvas::new(3, 2);

        #[rustfmt::skip]
        assert_eq!(canvas.uiter_buffer().collect::<Vec<_>>(), [
            (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
            (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
            (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
            (0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5),
            (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6),
            (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
        ], "Incorrect X and Y pairs, or in wrong order.");
    }

    // Color.

    #[test]
    fn color_buffer_size_at_init() {
        let canvas = TextCanvas::new(7, 4);

        assert!(
            canvas.color_buffer.is_empty(),
            "Color buffer should be empty."
        );
    }

    #[test]
    fn color_buffer_size_with_color() {
        let mut canvas = TextCanvas::new(7, 4);

        canvas.set_color(Color::new().bg_bright_blue());

        let buffer_width = canvas.color_buffer[0].len();
        let buffer_height = canvas.color_buffer.len();

        assert_eq!(
            buffer_width, 7,
            "Color buffer width should match output buffer width."
        );
        assert_eq!(
            buffer_height, 4,
            "Color buffer height should match output buffer height."
        );
    }

    #[test]
    fn is_colorized() {
        let mut canvas = TextCanvas::new(2, 2);

        assert!(
            !canvas.is_colorized(),
            "Canvas should not be colorized by default."
        );

        canvas.set_color(Color::new().bg_bright_blue());

        assert!(
            canvas.is_colorized(),
            "Canvas should be colorized after a color is set."
        );
    }

    #[test]
    fn set_color() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_blue().fix()],
                [Color::new(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn set_color_multiple() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_blue().fix()],
                [Color::new().bg_bright_blue().fix(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn set_color_override() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        canvas.set_color(Color::new().bg_bright_red());
        canvas.set_pixel(3, 3, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_red().fix()],
                [Color::new().bg_bright_blue().fix(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn color_is_reset_if_pixel_turned_off() {
        let mut canvas = TextCanvas::new(2, 2);

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        canvas.set_pixel(3, 3, false);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new()],
                [Color::new().bg_bright_blue().fix(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn get_as_string_colored() {
        let mut canvas = TextCanvas::new(3, 2);
        canvas.set_color(Color::new().bright_green());
        stroke_line_accros_canvas(&mut canvas);

        assert_eq!(
            canvas.to_string(),
            "\x1b[0;92m⠑\x1b[0m\x1b[0;92m⢄\x1b[0m⠀\n⠀⠀\x1b[0;92m⠑\x1b[0m\n",
            "Incorrect output string.",
        );
    }

    #[test]
    fn clear_clears_color_buffer() {
        let mut canvas = TextCanvas::new(2, 1);

        assert!(
            canvas.color_buffer.is_empty(),
            "Color buffer should be empty."
        );

        canvas.set_color(Color::new().bright_red());

        assert_eq!(
            canvas.color_buffer,
            [[Color::new(), Color::new()]],
            "Color buffer should be full of no-color.",
        );

        canvas.set_pixel(0, 0, true);

        assert_eq!(
            canvas.color_buffer,
            [[Color::new().bright_red().fix(), Color::new()]],
            "First pixel should be red.",
        );

        canvas.clear();

        assert_eq!(
            canvas.color_buffer,
            [[Color::new(), Color::new()]],
            "Color buffer should be full of no-color.",
        );
    }

    #[test]
    fn clear_edits_color_buffer_in_place() {
        let mut canvas = TextCanvas::new(2, 2);
        canvas.set_color(Color::new().bright_red());

        let color_buffer = canvas.color_buffer.as_ptr();
        let row_0 = canvas.color_buffer[0].as_ptr();
        let row_1 = canvas.color_buffer[1].as_ptr();

        canvas.clear();

        assert_eq!(
            color_buffer,
            canvas.color_buffer.as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_0,
            canvas.color_buffer[0].as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_1,
            canvas.color_buffer[1].as_ptr(),
            "Container should be the same as before."
        );
    }

    // Text.

    #[test]
    fn text_buffer_size_at_init() {
        let canvas = TextCanvas::new(7, 4);

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );
    }

    #[test]
    fn text_buffer_size_with_color() {
        let mut canvas = TextCanvas::new(7, 4);

        canvas.draw_text("foo", 0, 0);

        let buffer_width = canvas.text_buffer[0].len();
        let buffer_height = canvas.text_buffer.len();

        assert_eq!(
            buffer_width, 7,
            "Text buffer width should match output buffer width."
        );
        assert_eq!(
            buffer_height, 4,
            "Text buffer height should match output buffer height."
        );
    }

    #[test]
    fn is_textual() {
        let mut canvas = TextCanvas::new(2, 2);

        assert!(
            !canvas.is_colorized(),
            "Canvas should not be textual by default."
        );

        canvas.draw_text("hi", 0, 0);

        assert!(
            canvas.is_textual(),
            "Canvas should be textual after text is drawn."
        );
    }

    #[test]
    fn draw_text() {
        let mut canvas = TextCanvas::new(5, 1);

        canvas.draw_text("bar", 1, 0);

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "a", "r", ""]],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn draw_text_vertical() {
        let mut canvas = TextCanvas::new(1, 5);

        assert!(!canvas.is_textual());

        canvas.draw_text_vertical("bar", 0, 1);

        assert!(canvas.is_textual());

        assert_eq!(
            canvas.text_buffer,
            [[""], ["b"], ["a"], ["r"], [""],],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn draw_text_over_text() {
        let mut canvas = TextCanvas::new(5, 1);

        canvas.draw_text("bar", 1, 0);
        canvas.draw_text("foo", 2, 0);

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "f", "o", "o"]],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn draw_text_space_is_transparent() {
        let mut canvas = TextCanvas::new(9, 1);

        canvas.draw_text("foo bar", 1, 0);

        assert_eq!(
            canvas.text_buffer,
            [["", "f", "o", "o", "", "b", "a", "r", ""]],
            "Incorrect text buffer.",
        );
    }

    #[test]
    fn draw_text_space_clears_text() {
        let mut canvas = TextCanvas::new(5, 1);

        canvas.draw_text("bar", 1, 0);
        canvas.draw_text("  ", 2, 0);

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "", "", ""]],
            "Incorrect text buffer.",
        );
    }

    #[test]
    fn draw_text_with_overflow() {
        let mut canvas = TextCanvas::new(5, 2);

        // Show partially.
        canvas.draw_text("foo", -1, 0);
        canvas.draw_text("bar", 3, 1);

        // Completely out of bounds.
        canvas.draw_text("baz1", -10, -1);
        canvas.draw_text("baz2", 10, -1);
        canvas.draw_text("baz3", -10, 2);
        canvas.draw_text("baz4", 10, 2);

        assert_eq!(
            canvas.text_buffer,
            [["o", "o", "", "", ""], ["", "", "", "b", "a"],],
            "Incorrect text buffer.",
        );
    }

    #[test]
    fn draw_text_on_boundaries() {
        let mut canvas = TextCanvas::new(3, 3);

        canvas.draw_text("a", 0, 1);
        canvas.draw_text("b", 1, 0);
        canvas.draw_text("c", 2, 1);
        canvas.draw_text("d", 1, 2);

        assert_eq!(
            canvas.to_string(),
            "⠀b⠀\na⠀c\n⠀d⠀\n",
            "Incorrect text output.",
        );
    }

    #[test]
    fn draw_text_with_color() {
        let mut canvas = TextCanvas::new(3, 1);

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );

        canvas.draw_text("hi!", 0, 0);

        assert_eq!(
            canvas.text_buffer,
            [["h", "i", "!"]],
            "Text should not be colorized.",
        );

        canvas.set_color(Color::new().bright_red());
        canvas.draw_text("o!", 1, 0);

        assert_eq!(
            canvas.text_buffer,
            [["h", "\x1b[0;91mo\x1b[0m", "\x1b[0;91m!\x1b[0m"]],
            "'o!' should be red.",
        );
    }

    #[test]
    fn merge_text_space_does_not_clear_text() {
        let mut canvas = TextCanvas::new(5, 1);

        canvas.merge_text("bar", 1, 0);
        canvas.merge_text(" z", 2, 0);

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "a", "z", ""]],
            "Incorrect text buffer.",
        );
    }

    #[test]
    fn merge_text_vertical() {
        let mut canvas = TextCanvas::new(1, 5);

        assert!(!canvas.is_textual());

        canvas.merge_text_vertical("bar", 0, 1);
        canvas.merge_text_vertical(" z", 0, 2);

        assert!(canvas.is_textual());

        assert_eq!(
            canvas.text_buffer,
            [[""], ["b"], ["a"], ["z"], [""],],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn get_text_as_string() {
        let mut canvas = TextCanvas::new(5, 3);

        canvas.draw_text("foo", 1, 1);

        assert_eq!(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀\n⠀foo⠀\n⠀⠀⠀⠀⠀\n",
            "Incorrect output string."
        );
    }

    #[test]
    fn get_text_as_string_colored() {
        let mut canvas = TextCanvas::new(5, 3);

        canvas.set_color(Color::new().bright_green());
        canvas.draw_text("foo", 1, 1);

        assert_eq!(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀\n⠀\x1b[0;92mf\x1b[0m\x1b[0;92mo\x1b[0m\x1b[0;92mo\x1b[0m⠀\n⠀⠀⠀⠀⠀\n",
            "Incorrect output string.",
        );
    }

    #[test]
    fn clear_clears_text_buffer() {
        let mut canvas = TextCanvas::new(2, 1);

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );

        canvas.set_color(Color::new().bright_red());
        canvas.draw_text("hi", 0, 0);

        assert_eq!(
            canvas.text_buffer,
            [["\x1b[0;91mh\x1b[0m", "\x1b[0;91mi\x1b[0m"]],
            "Text should be colorized.",
        );

        canvas.clear();

        assert_eq!(
            canvas.text_buffer,
            [["", ""]],
            "Text buffer should be full of no-colored empty chars.",
        );
    }

    #[test]
    fn clear_edits_text_buffer_in_place() {
        let mut canvas = TextCanvas::new(2, 2);
        canvas.draw_text("hi", 0, 0);

        let text_buffer = canvas.text_buffer.as_ptr();
        let row_0 = canvas.text_buffer[0].as_ptr();
        let row_1 = canvas.text_buffer[1].as_ptr();

        canvas.clear();

        assert_eq!(
            text_buffer,
            canvas.text_buffer.as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_0,
            canvas.text_buffer[0].as_ptr(),
            "Container should be the same as before."
        );
        assert_eq!(
            row_1,
            canvas.text_buffer[1].as_ptr(),
            "Container should be the same as before."
        );
    }

    // Drawing primitives.

    #[test]
    fn stroke_line() {
        let mut canvas = TextCanvas::new(15, 5);

        let top_left = (0, 0);
        let top_right = (canvas.w(), 0);
        let bottom_right = (canvas.w(), canvas.h());
        let bottom_left = (0, canvas.h());
        let center = (canvas.cx(), canvas.cy());
        let center_top = (canvas.cx(), 0);
        let center_right = (canvas.w(), canvas.cy());
        let center_bottom = (canvas.cx(), canvas.h());
        let center_left = (0, canvas.cy());

        canvas.stroke_line(center.0, center.1, top_left.0, top_left.1);
        canvas.stroke_line(center.0, center.1, top_right.0, top_right.1);
        canvas.stroke_line(center.0, center.1, bottom_right.0, bottom_right.1);
        canvas.stroke_line(center.0, center.1, bottom_left.0, bottom_left.1);
        canvas.stroke_line(center.0, center.1, center_top.0, center_top.1);
        canvas.stroke_line(center.0, center.1, center_right.0, center_right.1);
        canvas.stroke_line(center.0, center.1, center_bottom.0, center_bottom.1);
        canvas.stroke_line(center.0, center.1, center_left.0, center_left.1);

        assert_eq!(
            canvas.to_string(),
            "\
⠑⠢⣀⠀⠀⠀⠀⢸⠀⠀⠀⠀⢀⠔⠊
⠀⠀⠀⠑⠢⣀⠀⢸⠀⢀⠤⠊⠁⠀⠀
⠤⠤⠤⠤⠤⠤⢵⣾⣶⠥⠤⠤⠤⠤⠤
⠀⠀⠀⣀⠤⠊⠁⢸⠀⠑⠢⣀⠀⠀⠀
⡠⠔⠊⠀⠀⠀⠀⢸⠀⠀⠀⠀⠉⠢⢄
",
            "Lines not drawn correctly.",
        );
    }

    #[test]
    fn stroke_line_from_outside_to_outside() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_line(-10, -10, canvas.w() + 10, canvas.h() + 10);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠈⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠉⠢⣀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⡀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⣀⠀
",
            "Line not drawn correctly.",
        );
    }

    #[test]
    fn erase_line() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill();

        let top_left = (0, 0);
        let bottom_right = (canvas.w(), canvas.h());

        canvas.invert();
        canvas.stroke_line(top_left.0, top_left.1, bottom_right.0, bottom_right.1);

        assert_eq!(
            canvas.to_string(),
            "\
⣮⣝⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣮⣝⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣮⣝⡻⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣝⡻⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣝⡻
",
            "Line not erased correctly.",
        );
    }

    #[test]
    fn stroke_rect() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_rect(6, 3, 20, 15);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀
⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀
⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀
⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀
⠀⠀⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀⠀
",
        );
    }

    #[test]
    fn frame() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.frame();

        assert_eq!(
            canvas.to_string(),
            "\
⡏⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣸
",
        );
    }

    #[test]
    fn fill_rect() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_rect(6, 3, 20, 15);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀
⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀
⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀
⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀
⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀
",
        );
    }

    #[test]
    fn stroke_triangle() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_triangle(6, 3, 20, 2, 23, 18);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⣀⣀⣀⡠⠤⠤⠤⡄⠀⠀⠀⠀
⠀⠀⠀⠈⠢⣀⠀⠀⠀⠀⢱⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠘⡄⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠑⠤⡀⡇⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠺⠀⠀⠀
",
        );
    }

    #[test]
    fn fill_triangle() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_triangle(6, 3, 20, 2, 23, 18);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⣀⣀⣀⣠⣤⣤⣤⡄⠀⠀⠀⠀
⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⡄⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⣿⡇⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⠀⠀⠀
",
        );
    }

    #[test]
    fn stroke_circle() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_circle(15, 10, 7);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡠⠊⠀⠀⠀⠈⠢⡀⠀⠀⠀
⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀
⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⡠⠃⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠒⠒⠒⠊⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    fn fill_circle() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_circle(15, 10, 7);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣦⡀⠀⠀⠀
⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⡿⠃⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠛⠛⠛⠋⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    fn stroke_ngon() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_ngon(canvas.cx(), canvas.cy(), 7, 6, 0.0);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⡰⠉⠉⠉⠙⡄⠀⠀⠀⠀
⠀⠀⠀⠀⢜⠀⠀⠀⠀⠀⢘⠄⠀⠀⠀
⠀⠀⠀⠀⠈⢆⠀⠀⠀⢠⠊⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    fn stroke_ngon_at_angle() {
        use std::f64::consts::PI;

        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_ngon(canvas.cx(), canvas.cy(), 7, 6, PI / 2.0);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢠⠔⠊⠁⠉⠢⢄⠀⠀⠀⠀
⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀
⠀⠀⠀⠀⠘⠤⡀⠀⠀⣀⠼⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠑⠉⠀⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    fn stroke_ngon_radius_matches_circle() {
        use std::f64::consts::PI;

        let mut canvas = TextCanvas::new(15, 5);

        canvas.stroke_ngon(canvas.cx(), canvas.cy(), 7, 3, PI / 2.0);

        canvas.stroke_circle(15, 10, 7);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡠⠊⢠⠃⢣⠈⠢⡀⠀⠀⠀
⠀⠀⠀⠀⡇⡰⠁⠀⠀⢣⠀⡇⠀⠀⠀
⠀⠀⠀⠀⠳⡓⠒⠢⠤⠤⡧⠃⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠒⠒⠒⠊⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    fn fill_ngon() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_ngon(canvas.cx(), canvas.cy(), 7, 6, 0.0);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿⡄⠀⠀⠀⠀
⠀⠀⠀⠀⢼⣿⣿⣿⣿⣿⣿⠄⠀⠀⠀
⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⠋⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀
",
        );
    }

    #[test]
    #[should_panic(expected = "Minimum 3 sides needed to draw an n-gon, but only 2 requested.")]
    fn fill_ngon_not_enough_sides() {
        let mut canvas = TextCanvas::new(15, 5);

        canvas.fill_ngon(canvas.cx(), canvas.cy(), 7, 2, 0.0);
    }

    #[test]
    fn draw_canvas() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.stroke_line(0, 0, canvas.w(), canvas.h());
        canvas.frame();

        let mut overlay = TextCanvas::new(7, 3);
        overlay.stroke_line(0, overlay.h(), overlay.w(), 0);
        overlay.frame();

        canvas.draw_canvas(&overlay, 8, 4);

        assert_eq!(
            canvas.to_string(),
            "\
⡟⠫⣉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹
⡇⠀⠀⠑⡏⠉⠉⠉⢉⡩⢻⠀⠀⠀⢸
⡇⠀⠀⠀⡇⠀⢀⠔⠁⠀⢸⠀⠀⠀⢸
⡇⠀⠀⠀⣧⣊⣁⣀⣀⣀⣸⢄⠀⠀⢸
⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣉⣢⣼
",
        );
    }

    #[test]
    fn draw_canvas_with_overflow() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.stroke_line(0, 0, canvas.w(), canvas.h());
        canvas.frame();

        let mut overlay = TextCanvas::new(7, 3);
        overlay.stroke_line(0, overlay.h(), overlay.w(), 0);
        overlay.frame();

        canvas.draw_canvas(&overlay, -8, -4);
        canvas.draw_canvas(&overlay, 24, -4);
        canvas.draw_canvas(&overlay, 24, 12);
        canvas.draw_canvas(&overlay, -8, 12);

        assert_eq!(
            canvas.to_string(),
            "\
⠁⠀⢸⠉⠉⠉⠉⠉⠉⠉⠉⠉⡇⠀⢀
⣀⣀⣸⠑⠢⣀⠀⠀⠀⠀⠀⠀⣧⣊⣁
⡇⠀⠀⠀⠀⠀⠑⠢⢄⠀⠀⠀⠀⠀⢸
⢉⡩⢻⠀⠀⠀⠀⠀⠀⠉⠢⢄⡏⠉⠉
⠁⠀⢸⣀⣀⣀⣀⣀⣀⣀⣀⣀⡇⠀⢀
",
        );
    }

    #[test]
    fn draw_canvas_with_color() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.set_color(Color::new().red());
        canvas.fill_rect(3, 3, 10, 10);

        let mut overlay = TextCanvas::new(15, 5);
        overlay.set_color(Color::new().green());
        overlay.fill_rect(8, 8, 10, 10);

        canvas.draw_canvas(&overlay, 8, 8);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀\x1b[0;31m⢀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⡀\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⡇\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⠈\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;31m⠉\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀
"
        );
    }

    #[test]
    fn draw_canvas_with_color_onto_non_colorized_canvas() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.fill_rect(3, 3, 10, 10);

        assert!(!canvas.is_colorized());

        let mut overlay = TextCanvas::new(15, 5);
        overlay.set_color(Color::new().green());
        overlay.fill_rect(8, 8, 10, 10);

        canvas.draw_canvas(&overlay, 8, 8);

        assert!(canvas.is_colorized());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢸⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢸⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀
"
        );
    }

    #[test]
    fn draw_canvas_with_text() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.draw_text("abcde", 1, 1);

        let mut overlay = TextCanvas::new(7, 3);
        overlay.draw_text("012", 2, 1);

        canvas.draw_canvas(&overlay, 5, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀a⠀⠀012
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn draw_canvas_with_colored_text() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.set_color(Color::new().red());
        canvas.draw_text("abcde", 1, 1);

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.draw_canvas(&overlay, 5, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31ma\x1b[0m⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn draw_canvas_with_colored_text_onto_non_textual_canvas() {
        let mut canvas = TextCanvas::new(7, 3);

        assert!(!canvas.is_colorized());
        assert!(!canvas.is_textual());

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.draw_canvas(&overlay, 5, 0);

        assert!(canvas.is_colorized());
        assert!(canvas.is_textual());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn draw_canvas_with_colored_text_onto_non_colorized_canvas() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.draw_text("abcde", 1, 1);

        assert!(!canvas.is_colorized());
        assert!(canvas.is_textual());

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.draw_canvas(&overlay, 5, 0);

        assert!(canvas.is_colorized());
        assert!(canvas.is_textual());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀a⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.stroke_line(0, 0, canvas.w(), canvas.h());
        canvas.frame();

        let mut overlay = TextCanvas::new(7, 3);
        overlay.stroke_line(0, overlay.h(), overlay.w(), 0);
        overlay.frame();

        canvas.merge_canvas(&overlay, 8, 4);

        assert_eq!(
            canvas.to_string(),
            "\
⡟⠫⣉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹
⡇⠀⠀⠑⡯⣉⠉⠉⢉⡩⢻⠀⠀⠀⢸
⡇⠀⠀⠀⡇⠀⢑⠶⢅⠀⢸⠀⠀⠀⢸
⡇⠀⠀⠀⣧⣊⣁⣀⣀⣉⣺⢄⠀⠀⢸
⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣉⣢⣼
",
        );
    }

    #[test]
    fn merge_canvas_with_overflow() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.stroke_line(0, 0, canvas.w(), canvas.h());
        canvas.frame();

        let mut overlay = TextCanvas::new(7, 3);
        overlay.stroke_line(0, overlay.h(), overlay.w(), 0);
        overlay.frame();

        canvas.merge_canvas(&overlay, -8, -4);
        canvas.merge_canvas(&overlay, 24, -4);
        canvas.merge_canvas(&overlay, 24, 12);
        canvas.merge_canvas(&overlay, -8, 12);

        assert_eq!(
            canvas.to_string(),
            "\
⡟⠫⣹⠉⠉⠉⠉⠉⠉⠉⠉⠉⡏⠉⢹
⣇⣀⣸⠑⠢⣀⠀⠀⠀⠀⠀⠀⣧⣊⣹
⡇⠀⠀⠀⠀⠀⠑⠢⢄⠀⠀⠀⠀⠀⢸
⣏⡩⢻⠀⠀⠀⠀⠀⠀⠉⠢⢄⡏⠉⢹
⣇⣀⣸⣀⣀⣀⣀⣀⣀⣀⣀⣀⣏⣢⣼
",
        );
    }

    #[test]
    fn merge_canvas_with_color() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.set_color(Color::new().red());
        canvas.fill_rect(3, 3, 10, 10);

        let mut overlay = TextCanvas::new(15, 5);
        overlay.set_color(Color::new().green());
        overlay.fill_rect(8, 8, 10, 10);

        canvas.merge_canvas(&overlay, 0, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀\x1b[0;31m⢀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⡀\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⡇\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31m⠈\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_color_onto_non_colorized_canvas() {
        let mut canvas = TextCanvas::new(15, 5);
        canvas.fill_rect(3, 3, 10, 10);

        assert!(!canvas.is_colorized());

        let mut overlay = TextCanvas::new(15, 5);
        overlay.set_color(Color::new().green());
        overlay.fill_rect(8, 8, 10, 10);

        canvas.merge_canvas(&overlay, 0, 0);

        assert!(canvas.is_colorized());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢸⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢸⣿⣿\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀
⠀⠈⠉⠉\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_text() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.draw_text("abcde", 1, 1);

        let mut overlay = TextCanvas::new(7, 3);
        overlay.draw_text("012", 2, 1);

        canvas.merge_canvas(&overlay, 0, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀a012e⠀
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_colored_text() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.set_color(Color::new().red());
        canvas.draw_text("abcde", 1, 1);

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.merge_canvas(&overlay, 5, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀\x1b[0;31ma\x1b[0m\x1b[0;31mb\x1b[0m\x1b[0;31mc\x1b[0m\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_colored_text_onto_non_textual_canvas() {
        let mut canvas = TextCanvas::new(7, 3);

        assert!(!canvas.is_colorized());
        assert!(!canvas.is_textual());

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.merge_canvas(&overlay, 5, 0);

        assert!(canvas.is_colorized());
        assert!(canvas.is_textual());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_colored_text_onto_non_colorized_canvas() {
        let mut canvas = TextCanvas::new(7, 3);
        canvas.draw_text("abcde", 1, 1);

        assert!(!canvas.is_colorized());
        assert!(canvas.is_textual());

        let mut overlay = TextCanvas::new(7, 3);
        overlay.set_color(Color::new().green());
        overlay.draw_text("012", 2, 1);

        canvas.merge_canvas(&overlay, 5, 0);

        assert!(canvas.is_colorized());
        assert!(canvas.is_textual());

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀
⠀abc\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m
⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn merge_canvas_with_pixels_color_and_text() {
        let mut canvas = TextCanvas::new(7, 5);
        canvas.set_color(Color::new().red());
        canvas.draw_text("abcdefg", 0, 2);
        canvas.set_color(Color::new().blue());
        canvas.stroke_line(0, 13, canvas.w(), 13);

        let mut overlay = TextCanvas::new(7, 5);
        overlay.set_color(Color::new().green());
        overlay.draw_text_vertical("012", canvas.cx() / 2, 1);
        overlay.set_color(Color::new().yellow());
        overlay.stroke_line(overlay.cx(), 0, overlay.cx(), overlay.h());

        canvas.merge_canvas(&overlay, 0, 0);

        print!("{canvas}");

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀\x1b[0;33m⢸\x1b[0m⠀⠀⠀
⠀⠀⠀\x1b[0;32m0\x1b[0m⠀⠀⠀
\x1b[0;31ma\x1b[0m\x1b[0;31mb\x1b[0m\x1b[0;31mc\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;31me\x1b[0m\x1b[0;31mf\x1b[0m\x1b[0;31mg\x1b[0m
\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;32m2\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m
⠀⠀⠀\x1b[0;33m⢸\x1b[0m⠀⠀⠀
"
        );
    }
}
