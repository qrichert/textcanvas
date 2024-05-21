pub use crate::color::Color;
use std::cmp;
use std::env;
use std::{fmt, fmt::Formatter};

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

macro_rules! to_usize {
    ($value:expr) => {
        usize::try_from($value).expect("the value should be bounds-checked prior to the conversion")
    };
}

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
    fn uwidth(&self) -> usize {
        to_usize!(self.width)
    }

    #[must_use]
    fn uheight(&self) -> usize {
        to_usize!(self.height)
    }
}

#[derive(Debug)]
pub struct IterPixelBuffer {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl IterPixelBuffer {
    fn new(buffer: &PixelBuffer) -> Self {
        Self {
            width: buffer[0].len(),
            height: buffer.len(),
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for IterPixelBuffer {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        let pixel = (self.x, self.y);

        self.x += 1;
        if self.x >= self.width {
            self.y += 1;
            self.x = 0;
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
/// use textcanvas::textcanvas::TextCanvas;
///
/// let mut canvas = TextCanvas::new(15, 5).unwrap();
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
/// canvas.draw_text(1, 2, "hello, world");
/// assert_eq!(
///     canvas.to_string(),
///     "\
/// ⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀hello,⠢world⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄
/// "
/// )
/// ```
///
/// # Todo
/// - `move_to()`, `line_to()`, `stroke()` and other JS Canvas
///   primitives.
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

    color: Color,
}

impl TextCanvas {
    /// Create new `TextCanvas`.
    ///
    /// # Errors
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
    pub fn new(width: i32, height: i32) -> Result<Self, &'static str> {
        if !Self::validate_size(width, height) {
            return Err("TextCanvas' minimal size is 1×1.");
        }

        let mut canvas = Self {
            output: Surface { width, height },
            screen: Surface {
                width: width * 2,
                height: height * 4,
            },
            buffer: Vec::new(),
            color_buffer: Vec::new(),
            text_buffer: Vec::new(),
            color: Color::new(),
        };

        canvas.init_buffer();

        Ok(canvas)
    }

    /// Ensure user `i32s` can safely be cast to internal `usize`.
    fn validate_size(width: i32, height: i32) -> bool {
        width > 0 && width <= MAX_RESOLUTION / 2 && height > 0 && height <= MAX_RESOLUTION / 4
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
    pub fn new_auto() -> Result<Self, &'static str> {
        let Some(width) = env::var("WIDTH").ok().and_then(|w| w.parse().ok()) else {
            return Err("Cannot read terminal width from environment.");
        };

        let Some(height) = env::var("HEIGHT").ok().and_then(|h| h.parse().ok()) else {
            return Err("Cannot read terminal height from environment.");
        };

        Self::new(width, height)
    }

    /// High-level string representation of the canvas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5).unwrap();
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

    /// Shortcut for height of pixel screen (index of last row).
    #[must_use]
    pub fn h(&self) -> i32 {
        self.screen.height() - 1
    }

    /// Shortcut for center-X of pixel screen.
    #[must_use]
    pub fn cx(&self) -> i32 {
        self.screen.width() / 2
    }

    /// Shortcut for center-Y of pixel screen.
    #[must_use]
    pub fn cy(&self) -> i32 {
        self.screen.height() / 2
    }

    /// Turn all pixels off and remove color and text.
    ///
    /// Note: This method does not drop the color and text buffers, it
    /// only clears them. No memory is freed, and all references remain
    /// valid (buffers are cleared in-place, not replaced).
    pub fn clear(&mut self) {
        self.clear_buffer();
        self.clear_color_buffer();
        self.clear_text_buffer();
    }

    fn clear_buffer(&mut self) {
        for (x, y) in self.iter_buffer() {
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

    /// Whether the canvas can contain colors.
    ///
    /// Note: This does not mean that any colors are displayed. This
    /// only means the color buffer is active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::textcanvas::{Color, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(15, 5).unwrap();
    ///
    /// assert!(!canvas.is_colorized());
    ///
    /// canvas.set_color(&Color::new().to_owned());  // Buffer is initialized.
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
    /// use textcanvas::textcanvas::TextCanvas;
    ///
    /// let mut canvas = TextCanvas::new(15, 5).unwrap();
    ///
    /// assert!(!canvas.is_textual());
    ///
    /// canvas.draw_text(0, 0, "");  // Buffer is initialized.
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
    /// use textcanvas::textcanvas::{Color, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(3, 1).unwrap();
    /// let green = Color::new().bright_green().to_owned();
    ///
    /// canvas.set_color(&green);
    /// assert!(canvas.is_colorized());
    ///
    /// canvas.draw_text(0, 0, "foo");
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
        self.color_buffer = Vec::new();
        for _ in 0..self.output.height() {
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
        if x < 0 || x >= self.screen.width() || y < 0 || y >= self.screen.height() {
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
    pub fn set_pixel(&mut self, x: i32, y: i32, state: bool) {
        if x < 0 || x >= self.screen.width() || y < 0 || y >= self.screen.height() {
            return;
        }
        let (x, y) = (to_usize!(x), to_usize!(y));

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
    /// Note: Coordinates outside the screen bounds are ignored.
    ///
    /// Note: Text is rendered on top of pixels, as a separate layer.
    ///
    /// Note: `set_color()` works for text as well, but text does not
    /// share its color buffer with pixels. Rather, color data is stored
    /// with the text characters themselves.
    ///
    /// # Arguments
    ///
    /// - `x` - Output X (true resolution).
    /// - `y` - Output Y (true resolution).
    /// - `text` - The text to draw. Spaces are transparent (you see
    ///   pixels through), and can be used to erase previously drawn
    ///   characters.
    pub fn draw_text(&mut self, mut x: i32, y: i32, text: &str) {
        if !self.is_textual() {
            self.init_text_buffer();
        }

        for char in text.chars() {
            if x < 0 || x >= self.output.width() || y < 0 || y >= self.output.height() {
                x += 1;
                continue;
            }

            let char = if char == ' ' {
                String::new()
            } else {
                self.color.format(&String::from(char))
            };

            let (ux, uy) = (to_usize!(x), to_usize!(y));
            self.text_buffer[uy][ux] = char;

            x += 1;
        }
    }

    fn init_text_buffer(&mut self) {
        self.text_buffer = Vec::new();
        for _ in 0..self.output.height() {
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
        let mut res = String::new();

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
    #[must_use]
    pub fn iter_buffer(&self) -> IterPixelBuffer {
        IterPixelBuffer::new(&self.buffer)
    }

    /// Stroke line using Bresenham's line algorithm.
    pub fn stroke_line(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32) {
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
                self.set_pixel(x, y, ON);
            }
            return;
        } else if dy == 0 {
            let y = y1;
            let from_x = cmp::min(x1, x2);
            let to_x = cmp::max(x1, x2);
            for x in from_x..=to_x {
                self.set_pixel(x, y, ON);
            }
            return;
        }

        #[cfg(not(tarpaulin_include))]
        loop {
            self.set_pixel(x1, y1, ON);
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
}

impl Default for TextCanvas {
    fn default() -> Self {
        Self::new(80, 24).unwrap()
    }
}

impl fmt::Display for TextCanvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

    // Canvas.

    #[test]
    fn test_output_size() {
        let canvas = TextCanvas::new(7, 4).unwrap();

        assert_eq!(canvas.output.width, 7, "Incorrect output width.");
        assert_eq!(canvas.output.height, 4, "Incorrect output height.");
    }

    #[test]
    fn screen_size() {
        let canvas = TextCanvas::new(7, 4).unwrap();

        assert_eq!(canvas.screen.width, 7 * 2, "Incorrect output width.");
        assert_eq!(canvas.screen.height, 4 * 4, "Incorrect output height.");
    }

    #[test]
    fn buffer_size() {
        let canvas = TextCanvas::new(7, 4).unwrap();
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
    fn size_zero_raises_error() {
        assert!(
            TextCanvas::new(0, 1).is_err(),
            "Zero width did not raise error."
        );

        assert!(
            TextCanvas::new(1, 0).is_err(),
            "Zero height did not raise error."
        );

        assert!(
            TextCanvas::new(0, 0).is_err(),
            "Zero width and height did not raise error."
        );
    }

    #[test]
    fn size_negative_raises_error() {
        assert!(
            TextCanvas::new(-1, 1).is_err(),
            "Negative width did not raise error."
        );

        assert!(
            TextCanvas::new(1, -1).is_err(),
            "Negative height did not raise error."
        );

        assert!(
            TextCanvas::new(-1, -1).is_err(),
            "Negative width and height did not raise error."
        );
    }

    #[test]
    fn size_too_big_raises_error() {
        assert!(
            TextCanvas::new(100_000, 1).is_err(),
            "Too large width did not raise error."
        );

        assert!(
            TextCanvas::new(1, 100_000).is_err(),
            "Too large height did not raise error."
        );

        assert!(
            TextCanvas::new(100_000, 100_000).is_err(),
            "Too large width and height did not raise error."
        );
    }

    #[test]
    fn max_i32_does_not_overflow() {
        // There was an error in the bounds checking condition:
        //
        //     if height * 4 <= MAX_RESOLUTION
        //
        // This panics if `size * 4` > `i32::MAX`, with `attempt to
        // multiply with overflow`. The solution is to divide instead:
        //
        //     if height <= MAX_RESOLUTION / 4
        assert!(
            TextCanvas::new(i32::MAX, 1).is_err(),
            "Should raise error, and not panic."
        );
        assert!(
            TextCanvas::new(1, i32::MAX).is_err(),
            "Should raise error, and not panic."
        );
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

        env::set_var("WIDTH", "1");
        env::set_var("HEIGHT", "2147483648");

        assert!(
            TextCanvas::new_auto().is_err(),
            "`HEIGHT` is too large for an `i32`."
        );

        env::set_var("WIDTH", "abc");
        env::set_var("HEIGHT", "1");

        assert!(TextCanvas::new_auto().is_err(), "`WIDTH` is not a number.");

        env::set_var("WIDTH", "1");
        env::set_var("HEIGHT", "abc");

        assert!(TextCanvas::new_auto().is_err(), "`HEIGHT` is not a number.");

        env::set_var("WIDTH", "12");
        env::set_var("HEIGHT", "5");

        let canvas = TextCanvas::new_auto().unwrap();

        assert_eq!(canvas.output.width, 12, "Incorrect auto width.");
        assert_eq!(canvas.output.height, 5, "Incorrect auto height.");
    }

    #[test]
    fn string_representation() {
        let canvas = TextCanvas::new(7, 4).unwrap();

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
        let canvas = TextCanvas::new(7, 4).unwrap();

        assert_eq!(canvas.w(), 13, "Incorrect screen width.");
        assert_eq!(canvas.h(), 15, "Incorrect screen height.");
        assert_eq!(canvas.cx(), 7, "Incorrect screen center-X.");
        assert_eq!(canvas.cy(), 8, "Incorrect screen center-Y.");
    }

    #[test]
    fn turn_all_pixels_on() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        for x in 0..canvas.screen.width() {
            for y in 0..canvas.screen.height() {
                canvas.set_pixel(x, y, true);
            }
        }

        assert_eq!(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not fully on.");
    }

    #[test]
    fn get_pixel() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

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
        let canvas = TextCanvas::new(1, 1).unwrap();

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
        let mut canvas = TextCanvas::new(1, 1).unwrap();

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
        let mut canvas = TextCanvas::new(3, 2).unwrap();
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
        let mut canvas = TextCanvas::new(1, 1).unwrap();

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
        let mut canvas = TextCanvas::new(1, 1).unwrap();

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
        let mut canvas = TextCanvas::new(3, 2).unwrap();
        stroke_line_accros_canvas(&mut canvas);

        assert_eq!(canvas.to_string(), "⠑⢄⠀\n⠀⠀⠑\n", "Incorrect output string.");
    }

    #[test]
    fn clear() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        // Fill canvas.
        for x in 0..canvas.screen.width() {
            for y in 0..canvas.screen.height() {
                canvas.set_pixel(x, y, true);
            }
        }

        canvas.clear();

        assert_eq!(canvas.to_string(), "⠀⠀\n⠀⠀\n", "Output not empty.");
    }

    #[test]
    fn clear_edits_buffer_in_place() {
        let mut canvas = TextCanvas::new(1, 1).unwrap();

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
    fn iter_buffer_by_blocks_lrtb() {
        // This tests a private method, but this method is at the core
        // of the output generation. Testing it helps ensure stability.
        let mut canvas = TextCanvas::new(3, 2).unwrap();
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
        let canvas = TextCanvas::new(3, 2).unwrap();

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
    fn stroke_line() {
        let mut canvas = TextCanvas::new(15, 5).unwrap();

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

    // Color.

    #[test]
    fn color_buffer_size_at_init() {
        let canvas = TextCanvas::new(7, 4).unwrap();

        assert!(
            canvas.color_buffer.is_empty(),
            "Color buffer should be empty."
        );
    }

    #[test]
    fn color_buffer_size_with_color() {
        let mut canvas = TextCanvas::new(7, 4).unwrap();

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
        let mut canvas = TextCanvas::new(2, 2).unwrap();

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
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_blue().to_owned()],
                [Color::new(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn set_color_multiple() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_blue().to_owned()],
                [Color::new().bg_bright_blue().to_owned(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn set_color_override() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        canvas.set_color(Color::new().bg_bright_red());
        canvas.set_pixel(3, 3, true);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new().bg_bright_red().to_owned()],
                [Color::new().bg_bright_blue().to_owned(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn color_is_reset_if_pixel_turned_off() {
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        canvas.set_color(Color::new().bg_bright_blue());
        canvas.set_pixel(3, 3, true);
        canvas.set_pixel(1, 5, true);

        canvas.set_pixel(3, 3, false);

        assert_eq!(
            canvas.color_buffer,
            [
                [Color::new(), Color::new()],
                [Color::new().bg_bright_blue().to_owned(), Color::new()],
            ],
            "Incorrect color buffer.",
        );
    }

    #[test]
    fn get_as_string_colored() {
        let mut canvas = TextCanvas::new(3, 2).unwrap();
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
        let mut canvas = TextCanvas::new(2, 1).unwrap();

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
            [[Color::new().bright_red().to_owned(), Color::new()]],
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
        let mut canvas = TextCanvas::new(2, 2).unwrap();
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
        let canvas = TextCanvas::new(7, 4).unwrap();

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );
    }

    #[test]
    fn text_buffer_size_with_color() {
        let mut canvas = TextCanvas::new(7, 4).unwrap();

        canvas.draw_text(0, 0, "foo");

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
        let mut canvas = TextCanvas::new(2, 2).unwrap();

        assert!(
            !canvas.is_colorized(),
            "Canvas should not be textual by default."
        );

        canvas.draw_text(0, 0, "hi");

        assert!(
            canvas.is_textual(),
            "Canvas should be textual after text is drawn."
        );
    }

    #[test]
    fn draw_text() {
        let mut canvas = TextCanvas::new(5, 1).unwrap();

        canvas.draw_text(1, 0, "bar");

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "a", "r", ""]],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn draw_text_over_text() {
        let mut canvas = TextCanvas::new(5, 1).unwrap();

        canvas.draw_text(1, 0, "bar");
        canvas.draw_text(2, 0, "foo");

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "f", "o", "o"]],
            "Incorrect text buffer."
        );
    }

    #[test]
    fn space_is_transparent() {
        let mut canvas = TextCanvas::new(9, 1).unwrap();

        canvas.draw_text(1, 0, "foo bar");

        assert_eq!(
            canvas.text_buffer,
            [["", "f", "o", "o", "", "b", "a", "r", ""]],
            "Incorrect text buffer.",
        );
    }
    #[test]
    fn space_clears_text() {
        let mut canvas = TextCanvas::new(5, 1).unwrap();

        canvas.draw_text(1, 0, "bar");
        canvas.draw_text(2, 0, "  ");

        assert_eq!(
            canvas.text_buffer,
            [["", "b", "", "", ""]],
            "Incorrect text buffer.",
        );
    }
    #[test]
    fn draw_text_with_overflow() {
        let mut canvas = TextCanvas::new(5, 2).unwrap();

        // Show partially.
        canvas.draw_text(-1, 0, "foo");
        canvas.draw_text(3, 1, "bar");

        // Completely out of bounds.
        canvas.draw_text(-10, -1, "baz1");
        canvas.draw_text(10, -1, "baz2");
        canvas.draw_text(-10, 2, "baz3");
        canvas.draw_text(10, 2, "baz4");

        assert_eq!(
            canvas.text_buffer,
            [["o", "o", "", "", ""], ["", "", "", "b", "a"],],
            "Incorrect text buffer.",
        );
    }
    #[test]
    fn draw_text_on_boundaries() {
        let mut canvas = TextCanvas::new(3, 3).unwrap();

        canvas.draw_text(0, 1, "a");
        canvas.draw_text(1, 0, "b");
        canvas.draw_text(2, 1, "c");
        canvas.draw_text(1, 2, "d");

        assert_eq!(
            canvas.to_string(),
            "⠀b⠀\na⠀c\n⠀d⠀\n",
            "Incorrect text output.",
        );
    }

    #[test]
    fn draw_text_with_color() {
        let mut canvas = TextCanvas::new(3, 1).unwrap();

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );

        canvas.draw_text(0, 0, "hi!");

        assert_eq!(
            canvas.text_buffer,
            [["h", "i", "!"]],
            "Text should not be colorized.",
        );

        canvas.set_color(Color::new().bright_red());
        canvas.draw_text(1, 0, "o!");

        assert_eq!(
            canvas.text_buffer,
            [["h", "\x1b[0;91mo\x1b[0m", "\x1b[0;91m!\x1b[0m"]],
            "'o!' should be red.",
        );
    }

    #[test]
    fn get_text_as_string() {
        let mut canvas = TextCanvas::new(5, 3).unwrap();

        canvas.draw_text(1, 1, "foo");

        assert_eq!(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀\n⠀foo⠀\n⠀⠀⠀⠀⠀\n",
            "Incorrect output string."
        );
    }

    #[test]
    fn get_text_as_string_colored() {
        let mut canvas = TextCanvas::new(5, 3).unwrap();

        canvas.set_color(Color::new().bright_green());
        canvas.draw_text(1, 1, "foo");

        assert_eq!(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀\n⠀\x1b[0;92mf\x1b[0m\x1b[0;92mo\x1b[0m\x1b[0;92mo\x1b[0m⠀\n⠀⠀⠀⠀⠀\n",
            "Incorrect output string.",
        );
    }

    #[test]
    fn clear_clears_text_buffer() {
        let mut canvas = TextCanvas::new(2, 1).unwrap();

        assert!(
            canvas.text_buffer.is_empty(),
            "Text buffer should be empty."
        );

        canvas.set_color(Color::new().bright_red());
        canvas.draw_text(0, 0, "hi");

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
        let mut canvas = TextCanvas::new(2, 2).unwrap();
        canvas.draw_text(0, 0, "hi");

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
}