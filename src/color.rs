use std::fmt;
use std::fmt::{Formatter, Write};

const ESC: &str = "\x1b[";
const RESET: &str = "\x1b[0m";
const PLACEHOLDER: &str = "{}";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum ColorMode {
    #[default]
    NoColor,
    ColorRGB,
    Color4bit,
}

/// Color for the terminal.
///
/// # Examples
///
/// ```rust
/// use textcanvas::Color;
///
/// // Bright red text.
/// assert_eq!(
///     Color::new().bright_red().format("hello, world"),
///     "\x1b[0;91mhello, world\x1b[0m",
/// );
///
/// // Bold-underlined red text over green background.
/// assert_eq!(
///     Color::new().bold().underline().red().bg_green().format("hello, world"),
///     "\x1b[1;4;31;42mhello, world\x1b[0m",
/// );
///
/// // RGB background.
/// assert_eq!(
///     Color::new().bg_rgb(45, 227, 61).format("hello, world"),
///     "\x1b[0;48;2;45;227;61mhello, world\x1b[0m",
/// );
///
/// // RGB text from hex value.
/// assert_eq!(
///     Color::new().rbg_from_hex("#1f2c3b").format("hello, world"),
///     "\x1b[0;38;2;31;44;59mhello, world\x1b[0m",
/// );
/// ```
///
/// # Limitations
///
/// <div class="warning">
///
/// RGB (24-bit) colors do not work on every terminal.
///
/// </div>
///
/// <div class="warning">
///
/// `italic()` isn't widely supported in terminal implementations, and
/// is sometimes treated as inverse or blink.
///
/// </div>
///
/// # Technical Details
///
/// - `ESC` = `0o33` = `0x1b` = Escape character, start of escape sequence.
/// - `ESC[` = Control sequence.
/// - `ESC[0m` = Reset sequence.
/// - `ESC[0;⟨n⟩m]` = `⟨n⟩` is one of 16 color codes (4-bit)
/// - `ESC[38;2;⟨r⟩;⟨g⟩;⟨b⟩m` = `[0;255]` Select RGB foreground color
/// - `ESC[48;2;⟨r⟩;⟨g⟩;⟨b⟩m` = `[0;255]` Select RGB background color
///
/// ## See Also
///
/// - <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
///
/// # Todo
///
/// 8-bit colors.
/// - <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
/// - They have official(?) names: <https://www.ditig.com/publications/256-colors-cheat-sheet>
/// - Looks like they are all present in Python's colored.library.py
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Color {
    mode: ColorMode,
    color_rgb: Option<(u8, u8, u8)>,
    bg_color_rgb: Option<(u8, u8, u8)>,
    color_4bit: Option<u8>,
    bg_color_4bit: Option<u8>,
    is_bold: bool,
    is_italic: bool,
    is_underlined: bool,
}

macro_rules! color_4bit {
    ($name:ident, $value:literal) => {
        pub fn $name(&mut self) -> &mut Self {
            self.apply_color_4bit($value)
        }
    };
}

macro_rules! bg_color_4bit {
    ($name:ident, $value:literal) => {
        pub fn $name(&mut self) -> &mut Self {
            self.apply_bg_color_4bit($value)
        }
    };
}

impl Color {
    #[must_use]
    pub fn new() -> Self {
        Self {
            mode: ColorMode::NoColor,
            color_rgb: None,
            bg_color_rgb: None,
            color_4bit: None,
            bg_color_4bit: None,
            is_bold: false,
            is_italic: false,
            is_underlined: false,
        }
    }

    #[must_use]
    pub fn format(&self, string: &str) -> String {
        self.to_string().replace(PLACEHOLDER, string)
    }

    fn is_empty(&self) -> bool {
        matches!(self.mode, ColorMode::NoColor) && !self.has_display_attributes()
    }

    fn has_colors(&self) -> bool {
        !matches!(self.mode, ColorMode::NoColor)
    }

    // Display Attributes.

    pub fn bold(&mut self) -> &mut Self {
        self.is_bold = true;
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.is_italic = true;
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.is_underlined = true;
        self
    }

    fn format_display_attributes(&self) -> String {
        if !self.has_display_attributes() {
            return String::from("0");
        }

        let mut attributes = Vec::new();
        if self.is_bold {
            attributes.push("1");
        }
        if self.is_italic {
            attributes.push("3");
        }
        if self.is_underlined {
            attributes.push("4");
        }
        attributes.join(";")
    }

    fn has_display_attributes(&self) -> bool {
        self.is_bold || self.is_italic || self.is_underlined
    }

    // RGB colors (24-bit).

    fn apply_color_rgb(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.mode = ColorMode::ColorRGB;
        self.color_rgb = Some((red, green, blue));
        self
    }

    fn apply_bg_color_rgb(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.mode = ColorMode::ColorRGB;
        self.bg_color_rgb = Some((red, green, blue));
        self
    }

    pub fn rgb(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.apply_color_rgb(red, green, blue)
    }

    pub fn bg_rgb(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.apply_bg_color_rgb(red, green, blue)
    }

    pub fn rbg_from_hex(&mut self, hex_color: &str) -> &mut Self {
        let (r, g, b) = Self::hex_to_rgb(hex_color);
        self.rgb(r, g, b)
    }

    pub fn bg_rbg_from_hex(&mut self, hex_color: &str) -> &mut Self {
        let (r, g, b) = Self::hex_to_rgb(hex_color);
        self.bg_rgb(r, g, b)
    }

    fn hex_to_rgb(mut hex_color: &str) -> (u8, u8, u8) {
        if hex_color.starts_with('#') {
            hex_color = &hex_color[1..];
        }

        if hex_color.len() != 6 {
            return (0, 0, 0);
        }

        let red = &hex_color[..2];
        let green = &hex_color[2..4];
        let blue = &hex_color[4..];

        (
            u8::from_str_radix(red, 16).unwrap_or(0),
            u8::from_str_radix(green, 16).unwrap_or(0),
            u8::from_str_radix(blue, 16).unwrap_or(0),
        )
    }

    fn format_colors_rgb(&self) -> String {
        let mut colors = String::new();
        // Foreground.
        if let Some(color) = self.color_rgb {
            let (red, green, blue) = color;
            write!(colors, "38;2;{red};{green};{blue}").unwrap_or(());

            // Foreground AND Background.
            if self.bg_color_rgb.is_some() {
                // Close foreground sequence and start new one.
                write!(colors, "m{ESC}").unwrap_or(());
            }
        }
        // Background.
        if let Some(bg_color) = self.bg_color_rgb {
            let (red, green, blue) = bg_color;
            write!(colors, "48;2;{red};{green};{blue}").unwrap_or(());
        }
        colors
    }

    // 4-bit colors.

    fn apply_color_4bit(&mut self, color: u8) -> &mut Self {
        self.mode = ColorMode::Color4bit;
        self.color_4bit = Some(color);
        self
    }

    fn apply_bg_color_4bit(&mut self, color: u8) -> &mut Self {
        self.mode = ColorMode::Color4bit;
        self.bg_color_4bit = Some(color);
        self
    }

    fn format_colors_4bit(&self) -> String {
        let mut colors = String::new();
        // Foreground.
        if let Some(color) = self.color_4bit {
            write!(colors, "{color}").unwrap_or(());

            // Foreground AND Background.
            if self.bg_color_4bit.is_some() {
                write!(colors, ";").unwrap_or(());
            }
        }
        // Background.
        if let Some(bg_color) = self.bg_color_4bit {
            write!(colors, "{bg_color}").unwrap_or(());
        }
        colors
    }

    color_4bit!(red, 31);
    color_4bit!(yellow, 33);
    color_4bit!(green, 32);
    color_4bit!(blue, 34);
    color_4bit!(cyan, 36);
    color_4bit!(magenta, 35);
    color_4bit!(gray, 30);
    color_4bit!(white, 37);

    color_4bit!(bright_red, 91);
    color_4bit!(bright_yellow, 93);
    color_4bit!(bright_green, 92);
    color_4bit!(bright_blue, 94);
    color_4bit!(bright_cyan, 96);
    color_4bit!(bright_magenta, 95);
    color_4bit!(bright_gray, 90);
    color_4bit!(bright_white, 97);

    bg_color_4bit!(bg_red, 41);
    bg_color_4bit!(bg_yellow, 43);
    bg_color_4bit!(bg_green, 42);
    bg_color_4bit!(bg_blue, 44);
    bg_color_4bit!(bg_cyan, 46);
    bg_color_4bit!(bg_magenta, 45);
    bg_color_4bit!(bg_gray, 40);
    bg_color_4bit!(bg_white, 47);

    bg_color_4bit!(bg_bright_red, 101);
    bg_color_4bit!(bg_bright_yellow, 103);
    bg_color_4bit!(bg_bright_green, 102);
    bg_color_4bit!(bg_bright_blue, 104);
    bg_color_4bit!(bg_bright_cyan, 106);
    bg_color_4bit!(bg_bright_magenta, 105);
    bg_color_4bit!(bg_bright_gray, 100);
    bg_color_4bit!(bg_bright_white, 107);
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "{PLACEHOLDER}");
        }

        write!(f, "{ESC}").unwrap_or(());

        write!(f, "{}", self.format_display_attributes()).unwrap_or(());

        if self.has_colors() {
            write!(f, ";").unwrap_or(());
        }
        match self.mode {
            ColorMode::ColorRGB => {
                write!(f, "{}", self.format_colors_rgb()).unwrap_or(());
            }
            ColorMode::Color4bit => {
                write!(f, "{}", self.format_colors_4bit()).unwrap_or(());
            }
            ColorMode::NoColor => {}
        }

        write!(f, "m{PLACEHOLDER}{RESET}")
    }
}

impl From<&mut Color> for String {
    fn from(color: &mut Color) -> Self {
        color.to_string()
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        color.to_string()
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // General.

    #[test]
    fn format() {
        let str = Color::new().magenta().format("foo");

        assert_eq!(str, "\x1b[0;35mfoo\x1b[0m");
    }

    #[test]
    fn to_string() {
        let str = Color::new().magenta().to_string();

        assert_eq!(str, "\x1b[0;35m{}\x1b[0m");
    }

    #[test]
    fn string_from_mut_builder() {
        let str = String::from(Color::new().magenta());

        assert_eq!(str, "\x1b[0;35m{}\x1b[0m");
    }

    #[test]
    fn string_from_ref() {
        let str = String::from(&Color::new().magenta().to_owned());

        assert_eq!(str, "\x1b[0;35m{}\x1b[0m");
    }

    #[test]
    fn string_from_owned() {
        let str = String::from(Color::new().magenta().to_owned());

        assert_eq!(str, "\x1b[0;35m{}\x1b[0m");
    }

    #[test]
    fn to_owned() {
        let owned_color = Color::new().magenta().to_owned();

        assert_eq!(
            owned_color.color_4bit, // Do something with the owned color.
            Color::new().magenta().color_4bit
        );
    }

    // Special Cases.

    #[test]
    fn no_color() {
        assert_eq!(Color::new().format("hello, world"), "hello, world");
    }

    // Display Attributes

    #[test]
    fn no_color_bold() {
        assert_eq!(
            Color::new().bold().format("hello, world"),
            "\x1b[1mhello, world\x1b[0m"
        );
    }

    #[test]
    fn no_color_italic() {
        assert_eq!(
            Color::new().italic().format("hello, world"),
            "\x1b[3mhello, world\x1b[0m"
        );
    }

    #[test]
    fn no_color_underline() {
        assert_eq!(
            Color::new().underline().format("hello, world"),
            "\x1b[4mhello, world\x1b[0m"
        );
    }

    #[test]
    fn no_color_bold_italic_underline() {
        assert_eq!(
            Color::new()
                .bold()
                .italic()
                .underline()
                .format("hello, world"),
            "\x1b[1;3;4mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_bold() {
        assert_eq!(
            Color::new().bold().bg_bright_green().format("hello, world"),
            "\x1b[1;102mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_bold_after_color() {
        // Order should not matter.
        assert_eq!(
            Color::new().bg_bright_green().bold().format("hello, world"),
            "\x1b[1;102mhello, world\x1b[0m"
        );
    }

    // RGB (24-bit)

    #[test]
    fn color_rgb() {
        assert_eq!(
            Color::new().rgb(45, 227, 61).format("hello, world"),
            "\x1b[0;38;2;45;227;61mhello, world\x1b[0m",
        );
        assert_eq!(
            Color::new().bold().rgb(45, 227, 61).format("hello, world"),
            "\x1b[1;38;2;45;227;61mhello, world\x1b[0m",
        );
    }

    #[test]
    fn color_bg_rgb() {
        assert_eq!(
            Color::new().bg_rgb(45, 227, 61).format("hello, world"),
            "\x1b[0;48;2;45;227;61mhello, world\x1b[0m",
        );
        assert_eq!(
            Color::new()
                .bold()
                .bg_rgb(45, 227, 61)
                .format("hello, world"),
            "\x1b[1;48;2;45;227;61mhello, world\x1b[0m",
        );
    }

    #[test]
    fn color_rgb_with_bg() {
        assert_eq!(
            Color::new()
                .rgb(45, 227, 61)
                .bg_rgb(13, 10, 40)
                .format("hello, world"),
            "\x1b[0;38;2;45;227;61m\x1b[48;2;13;10;40mhello, world\x1b[0m",
        );
        assert_eq!(
            Color::new()
                .bold()
                .rgb(45, 227, 61)
                .bg_rgb(13, 10, 40)
                .format("hello, world"),
            "\x1b[1;38;2;45;227;61m\x1b[48;2;13;10;40mhello, world\x1b[0m",
        );
    }

    #[test]
    fn color_rgb_from_hex() {
        assert_eq!(
            Color::new().rbg_from_hex("#1f2c3b").format("hello, world"),
            "\x1b[0;38;2;31;44;59mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_rgb_from_hex_without_hash() {
        assert_eq!(
            Color::new().rbg_from_hex("1f2c3b").format("hello, world"),
            "\x1b[0;38;2;31;44;59mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_rgb_from_hex_invalid() {
        assert_eq!(
            Color::new()
                .rbg_from_hex("#012345678901234567890123456789")
                .format("hello, world"),
            "\x1b[0;38;2;0;0;0mhello, world\x1b[0m"
        );
        assert_eq!(
            Color::new().rbg_from_hex("#zzzzzz").format("hello, world"),
            "\x1b[0;38;2;0;0;0mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_rgb_from_hex_only_one_invalid() {
        assert_eq!(
            Color::new().rbg_from_hex("#ffggff").format("hello, world"),
            "\x1b[0;38;2;255;0;255mhello, world\x1b[0m",
        );
    }

    #[test]
    fn color_bg_rgb_from_hex() {
        assert_eq!(
            Color::new()
                .bg_rbg_from_hex("#1f2c3b")
                .format("hello, world"),
            "\x1b[0;48;2;31;44;59mhello, world\x1b[0m"
        );
    }

    // 4-bit.

    #[test]
    fn color_4bit() {
        assert_eq!(
            Color::new().green().format("hello, world"),
            "\x1b[0;32mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_4bit_bright() {
        assert_eq!(
            Color::new().bright_green().format("hello, world"),
            "\x1b[0;92mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_4bit_bg() {
        assert_eq!(
            Color::new().bg_green().format("hello, world"),
            "\x1b[0;42mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_4bit_bg_bright() {
        assert_eq!(
            Color::new().bg_bright_green().format("hello, world"),
            "\x1b[0;102mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_4bit_with_bg() {
        assert_eq!(
            Color::new().red().bg_green().format("hello, world"),
            "\x1b[0;31;42mhello, world\x1b[0m"
        );
    }
}
