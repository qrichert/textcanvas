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
    Color8bit,
}

/// Color for the terminal.
///
/// Three color modes are available:
///
/// - 4-bit colors.
/// - 8-bit Xterm colors (prefixed by `x_`).
/// - 24-bit RGB colors (`rgb` methods).
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
/// // 8-bit foreground and background.
/// assert_eq!(
///     Color::new().x_dark_goldenrod().bg_x_aquamarine_3().format("hello, world"),
///     "\x1b[0;38;5;136m\x1b[48;5;79mhello, world\x1b[0m",
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
/// <div class="warning">
///
/// Mixing foreground and background colors of different modes doesn't
/// work. For example, if you have an 8-bit background, use an 8-bit
/// foreground as well.
///
/// </div>
///
/// # Technical Details
///
/// - `ESC` = `0o33` = `0x1b` = Escape character, start of escape sequence.
/// - `ESC[` = Control sequence.
/// - `ESC[0m` = Reset sequence.
///
/// - `ESC[0;⟨n⟩m]` = `⟨n⟩` is one of 16 color codes (4-bit)
///
/// - `ESC[38;5;⟨n⟩m` Select 8-bit foreground color
/// - `ESC[48;5;⟨n⟩m` Select 8-bit background color
///
/// - `ESC[38;2;⟨r⟩;⟨g⟩;⟨b⟩m` = `[0;255]` Select RGB foreground color
/// - `ESC[48;2;⟨r⟩;⟨g⟩;⟨b⟩m` = `[0;255]` Select RGB background color
///
/// ## See Also
///
/// - <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Color {
    mode: ColorMode,
    color_rgb: Option<(u8, u8, u8)>,
    bg_color_rgb: Option<(u8, u8, u8)>,
    color_4bit: Option<u8>,
    bg_color_4bit: Option<u8>,
    color_8bit: Option<u8>,
    bg_color_8bit: Option<u8>,
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

macro_rules! color_8bit {
    ($name:ident, $value:literal) => {
        pub fn $name(&mut self) -> &mut Self {
            self.apply_color_8bit($value)
        }
    };
}

macro_rules! bg_color_8bit {
    ($name:ident, $value:literal) => {
        pub fn $name(&mut self) -> &mut Self {
            self.apply_bg_color_8bit($value)
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
            color_8bit: None,
            bg_color_8bit: None,
            is_bold: false,
            is_italic: false,
            is_underlined: false,
        }
    }

    /// Transform mutable builder into the final color.
    ///
    /// This is equivalent to calling `to_owned()`, but is shorter, and
    /// feels more natural at the end of a building sequence. You stop
    /// "mixing", and "fix" the color in its current state.
    ///
    ///
    /// # Examples
    ///
    /// This won't compile, because `magenta()` returns a mutable
    /// reference to the `Color`, but nothing owns the `Color`.
    ///
    /// ```rust,compile_fail
    /// # use textcanvas::Color;
    /// // Temporary value is freed at the end of this statement:
    /// let mut color = Color::new().magenta();
    /// // Borrow later used here:
    /// color.bold();
    /// ```
    ///
    /// The easy solution is to call `fix()`, which really just calls
    /// `to_owned()`, so that the mutable reference is transformed into
    /// an owned value.
    ///
    /// ```rust
    /// # use textcanvas::Color;
    /// let mut color = Color::new().magenta().fix();
    /// color.bold();
    /// ```
    #[must_use]
    pub fn fix(&self) -> Self {
        self.to_owned()
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

        let mut attributes = Vec::with_capacity(3);
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

    // 8-bit colors.

    fn apply_color_8bit(&mut self, color: u8) -> &mut Self {
        self.mode = ColorMode::Color8bit;
        self.color_8bit = Some(color);
        self
    }

    fn apply_bg_color_8bit(&mut self, color: u8) -> &mut Self {
        self.mode = ColorMode::Color8bit;
        self.bg_color_8bit = Some(color);
        self
    }

    fn format_colors_8bit(&self) -> String {
        let mut colors = String::new();
        // Foreground.
        if let Some(color) = self.color_8bit {
            write!(colors, "38;5;{color}").unwrap_or(());

            // Foreground AND Background.
            if self.bg_color_8bit.is_some() {
                // Close foreground sequence and start new one.
                write!(colors, "m{ESC}").unwrap_or(());
            }
        }
        // Background.
        if let Some(bg_color) = self.bg_color_8bit {
            write!(colors, "48;5;{bg_color}").unwrap_or(());
        }
        colors
    }

    color_8bit!(x_black, 0);
    color_8bit!(x_maroon, 1);
    color_8bit!(x_green, 2);
    color_8bit!(x_olive, 3);
    color_8bit!(x_navy, 4);
    color_8bit!(x_purple, 5);
    color_8bit!(x_teal, 6);
    color_8bit!(x_silver, 7);
    color_8bit!(x_grey, 8);
    color_8bit!(x_red, 9);
    color_8bit!(x_lime, 10);
    color_8bit!(x_yellow, 11);
    color_8bit!(x_blue, 12);
    color_8bit!(x_fuchsia, 13);
    color_8bit!(x_aqua, 14);
    color_8bit!(x_white, 15);
    color_8bit!(x_grey_0, 16);
    color_8bit!(x_navy_blue, 17);
    color_8bit!(x_dark_blue, 18);
    color_8bit!(x_blue_3a, 19);
    color_8bit!(x_blue_3b, 20);
    color_8bit!(x_blue_1, 21);
    color_8bit!(x_dark_green, 22);
    color_8bit!(x_deep_sky_blue_4a, 23);
    color_8bit!(x_deep_sky_blue_4b, 24);
    color_8bit!(x_deep_sky_blue_4c, 25);
    color_8bit!(x_dodger_blue_3, 26);
    color_8bit!(x_dodger_blue_2, 27);
    color_8bit!(x_green_4, 28);
    color_8bit!(x_spring_green_4, 29);
    color_8bit!(x_turquoise_4, 30);
    color_8bit!(x_deep_sky_blue_3a, 31);
    color_8bit!(x_deep_sky_blue_3b, 32);
    color_8bit!(x_dodger_blue_1, 33);
    color_8bit!(x_green_3a, 34);
    color_8bit!(x_spring_green_3a, 35);
    color_8bit!(x_dark_cyan, 36);
    color_8bit!(x_light_sea_green, 37);
    color_8bit!(x_deep_sky_blue_2, 38);
    color_8bit!(x_deep_sky_blue_1, 39);
    color_8bit!(x_green_3b, 40);
    color_8bit!(x_spring_green_3b, 41);
    color_8bit!(x_spring_green_2a, 42);
    color_8bit!(x_cyan_3, 43);
    color_8bit!(x_dark_turquoise, 44);
    color_8bit!(x_turquoise_2, 45);
    color_8bit!(x_green_1, 46);
    color_8bit!(x_spring_green_2b, 47);
    color_8bit!(x_spring_green_1, 48);
    color_8bit!(x_medium_spring_green, 49);
    color_8bit!(x_cyan_2, 50);
    color_8bit!(x_cyan_1, 51);
    color_8bit!(x_dark_red_a, 52);
    color_8bit!(x_deep_pink_4a, 53);
    color_8bit!(x_purple_4a, 54);
    color_8bit!(x_purple_4b, 55);
    color_8bit!(x_purple_3, 56);
    color_8bit!(x_blue_violet, 57);
    color_8bit!(x_orange_4a, 58);
    color_8bit!(x_grey_37, 59);
    color_8bit!(x_medium_purple_4, 60);
    color_8bit!(x_slate_blue_3a, 61);
    color_8bit!(x_slate_blue_3b, 62);
    color_8bit!(x_royal_blue_1, 63);
    color_8bit!(x_chartreuse_4, 64);
    color_8bit!(x_dark_sea_green_4a, 65);
    color_8bit!(x_pale_turquoise_4, 66);
    color_8bit!(x_steel_blue, 67);
    color_8bit!(x_steel_blue_3, 68);
    color_8bit!(x_cornflower_blue, 69);
    color_8bit!(x_chartreuse_3a, 70);
    color_8bit!(x_dark_sea_green_4b, 71);
    color_8bit!(x_cadet_blue_a, 72);
    color_8bit!(x_cadet_blue_b, 73);
    color_8bit!(x_sky_blue_3, 74);
    color_8bit!(x_steel_blue_1a, 75);
    color_8bit!(x_chartreuse_3b, 76);
    color_8bit!(x_pale_green_3a, 77);
    color_8bit!(x_sea_green_3, 78);
    color_8bit!(x_aquamarine_3, 79);
    color_8bit!(x_medium_turquoise, 80);
    color_8bit!(x_steel_blue_1b, 81);
    color_8bit!(x_chartreuse_2a, 82);
    color_8bit!(x_sea_green_2, 83);
    color_8bit!(x_sea_green_1a, 84);
    color_8bit!(x_sea_green_1b, 85);
    color_8bit!(x_aquamarine_1a, 86);
    color_8bit!(x_dark_slate_gray_2, 87);
    color_8bit!(x_dark_red_b, 88);
    color_8bit!(x_deep_pink_4b, 89);
    color_8bit!(x_dark_magenta_a, 90);
    color_8bit!(x_dark_magenta_b, 91);
    color_8bit!(x_dark_violet_a, 92);
    color_8bit!(x_purple_a, 93);
    color_8bit!(x_orange_4b, 94);
    color_8bit!(x_light_pink_4, 95);
    color_8bit!(x_plum_4, 96);
    color_8bit!(x_medium_purple_3a, 97);
    color_8bit!(x_medium_purple_3b, 98);
    color_8bit!(x_slate_blue_1, 99);
    color_8bit!(x_yellow_4a, 100);
    color_8bit!(x_wheat_4, 101);
    color_8bit!(x_grey_53, 102);
    color_8bit!(x_light_slate_grey, 103);
    color_8bit!(x_medium_purple, 104);
    color_8bit!(x_light_slate_blue, 105);
    color_8bit!(x_yellow_4b, 106);
    color_8bit!(x_dark_olive_green_3a, 107);
    color_8bit!(x_dark_sea_green, 108);
    color_8bit!(x_light_sky_blue_3a, 109);
    color_8bit!(x_light_sky_blue_3b, 110);
    color_8bit!(x_sky_blue_2, 111);
    color_8bit!(x_chartreuse_2b, 112);
    color_8bit!(x_dark_olive_green_3b, 113);
    color_8bit!(x_pale_green_3b, 114);
    color_8bit!(x_dark_sea_green_3a, 115);
    color_8bit!(x_dark_slate_gray_3, 116);
    color_8bit!(x_sky_blue_1, 117);
    color_8bit!(x_chartreuse_1, 118);
    color_8bit!(x_light_green_a, 119);
    color_8bit!(x_light_green_b, 120);
    color_8bit!(x_pale_green_1a, 121);
    color_8bit!(x_aquamarine_1b, 122);
    color_8bit!(x_dark_slate_gray_1, 123);
    color_8bit!(x_red_3a, 124);
    color_8bit!(x_deep_pink_4c, 125);
    color_8bit!(x_medium_violet_red, 126);
    color_8bit!(x_magenta_3a, 127);
    color_8bit!(x_dark_violet_b, 128);
    color_8bit!(x_purple_b, 129);
    color_8bit!(x_dark_orange_3a, 130);
    color_8bit!(x_indian_red_a, 131);
    color_8bit!(x_hot_pink_3a, 132);
    color_8bit!(x_medium_orchid_3, 133);
    color_8bit!(x_medium_orchid, 134);
    color_8bit!(x_medium_purple_2a, 135);
    color_8bit!(x_dark_goldenrod, 136);
    color_8bit!(x_light_salmon_3a, 137);
    color_8bit!(x_rosy_brown, 138);
    color_8bit!(x_grey_63, 139);
    color_8bit!(x_medium_purple_2b, 140);
    color_8bit!(x_medium_purple_1, 141);
    color_8bit!(x_gold_3a, 142);
    color_8bit!(x_dark_khaki, 143);
    color_8bit!(x_navajo_white_3, 144);
    color_8bit!(x_grey_69, 145);
    color_8bit!(x_light_steel_blue_3, 146);
    color_8bit!(x_light_steel_blue, 147);
    color_8bit!(x_yellow_3a, 148);
    color_8bit!(x_dark_olive_green_3c, 149);
    color_8bit!(x_dark_sea_green_3b, 150);
    color_8bit!(x_dark_sea_green_2a, 151);
    color_8bit!(x_light_cyan_3, 152);
    color_8bit!(x_light_sky_blue_1, 153);
    color_8bit!(x_green_yellow, 154);
    color_8bit!(x_dark_olive_green_2, 155);
    color_8bit!(x_pale_green_1b, 156);
    color_8bit!(x_dark_sea_green_2b, 157);
    color_8bit!(x_dark_sea_green_1a, 158);
    color_8bit!(x_pale_turquoise_1, 159);
    color_8bit!(x_red_3b, 160);
    color_8bit!(x_deep_pink_3a, 161);
    color_8bit!(x_deep_pink_3b, 162);
    color_8bit!(x_magenta_3b, 163);
    color_8bit!(x_magenta_3c, 164);
    color_8bit!(x_magenta_2a, 165);
    color_8bit!(x_dark_orange_3b, 166);
    color_8bit!(x_indian_red_b, 167);
    color_8bit!(x_hot_pink_3b, 168);
    color_8bit!(x_hot_pink_2, 169);
    color_8bit!(x_orchid, 170);
    color_8bit!(x_medium_orchid_1a, 171);
    color_8bit!(x_orange_3, 172);
    color_8bit!(x_light_salmon_3b, 173);
    color_8bit!(x_light_pink_3, 174);
    color_8bit!(x_pink_3, 175);
    color_8bit!(x_plum_3, 176);
    color_8bit!(x_violet, 177);
    color_8bit!(x_gold_3b, 178);
    color_8bit!(x_light_goldenrod_3, 179);
    color_8bit!(x_tan, 180);
    color_8bit!(x_misty_rose_3, 181);
    color_8bit!(x_thistle_3, 182);
    color_8bit!(x_plum_2, 183);
    color_8bit!(x_yellow_3b, 184);
    color_8bit!(x_khaki_3, 185);
    color_8bit!(x_light_goldenrod_2a, 186);
    color_8bit!(x_light_yellow_3, 187);
    color_8bit!(x_grey_84, 188);
    color_8bit!(x_light_steel_blue_1, 189);
    color_8bit!(x_yellow_2, 190);
    color_8bit!(x_dark_olive_green_1a, 191);
    color_8bit!(x_dark_olive_green_1b, 192);
    color_8bit!(x_dark_sea_green_1b, 193);
    color_8bit!(x_honeydew_2, 194);
    color_8bit!(x_light_cyan_1, 195);
    color_8bit!(x_red_1, 196);
    color_8bit!(x_deep_pink_2, 197);
    color_8bit!(x_deep_pink_1a, 198);
    color_8bit!(x_deep_pink_1b, 199);
    color_8bit!(x_magenta_2b, 200);
    color_8bit!(x_magenta_1, 201);
    color_8bit!(x_orange_red_1, 202);
    color_8bit!(x_indian_red_1a, 203);
    color_8bit!(x_indian_red_1b, 204);
    color_8bit!(x_hot_pink_a, 205);
    color_8bit!(x_hot_pink_b, 206);
    color_8bit!(x_medium_orchid_1b, 207);
    color_8bit!(x_dark_orange, 208);
    color_8bit!(x_salmon_1, 209);
    color_8bit!(x_light_coral, 210);
    color_8bit!(x_pale_violet_red_1, 211);
    color_8bit!(x_orchid_2, 212);
    color_8bit!(x_orchid_1, 213);
    color_8bit!(x_orange_1, 214);
    color_8bit!(x_sandy_brown, 215);
    color_8bit!(x_light_salmon_1, 216);
    color_8bit!(x_light_pink_1, 217);
    color_8bit!(x_pink_1, 218);
    color_8bit!(x_plum_1, 219);
    color_8bit!(x_gold_1, 220);
    color_8bit!(x_light_goldenrod_2b, 221);
    color_8bit!(x_light_goldenrod_2c, 222);
    color_8bit!(x_navajo_white_1, 223);
    color_8bit!(x_misty_rose_1, 224);
    color_8bit!(x_thistle_1, 225);
    color_8bit!(x_yellow_1, 226);
    color_8bit!(x_light_goldenrod_1, 227);
    color_8bit!(x_khaki_1, 228);
    color_8bit!(x_wheat_1, 229);
    color_8bit!(x_cornsilk_1, 230);
    color_8bit!(x_grey_100, 231);
    color_8bit!(x_grey_3, 232);
    color_8bit!(x_grey_7, 233);
    color_8bit!(x_grey_11, 234);
    color_8bit!(x_grey_15, 235);
    color_8bit!(x_grey_19, 236);
    color_8bit!(x_grey_23, 237);
    color_8bit!(x_grey_27, 238);
    color_8bit!(x_grey_30, 239);
    color_8bit!(x_grey_35, 240);
    color_8bit!(x_grey_39, 241);
    color_8bit!(x_grey_42, 242);
    color_8bit!(x_grey_46, 243);
    color_8bit!(x_grey_50, 244);
    color_8bit!(x_grey_54, 245);
    color_8bit!(x_grey_58, 246);
    color_8bit!(x_grey_62, 247);
    color_8bit!(x_grey_66, 248);
    color_8bit!(x_grey_70, 249);
    color_8bit!(x_grey_74, 250);
    color_8bit!(x_grey_78, 251);
    color_8bit!(x_grey_82, 252);
    color_8bit!(x_grey_85, 253);
    color_8bit!(x_grey_89, 254);
    color_8bit!(x_grey_93, 255);

    bg_color_8bit!(bg_x_black, 0);
    bg_color_8bit!(bg_x_maroon, 1);
    bg_color_8bit!(bg_x_green, 2);
    bg_color_8bit!(bg_x_olive, 3);
    bg_color_8bit!(bg_x_navy, 4);
    bg_color_8bit!(bg_x_purple, 5);
    bg_color_8bit!(bg_x_teal, 6);
    bg_color_8bit!(bg_x_silver, 7);
    bg_color_8bit!(bg_x_grey, 8);
    bg_color_8bit!(bg_x_red, 9);
    bg_color_8bit!(bg_x_lime, 10);
    bg_color_8bit!(bg_x_yellow, 11);
    bg_color_8bit!(bg_x_blue, 12);
    bg_color_8bit!(bg_x_fuchsia, 13);
    bg_color_8bit!(bg_x_aqua, 14);
    bg_color_8bit!(bg_x_white, 15);
    bg_color_8bit!(bg_x_grey_0, 16);
    bg_color_8bit!(bg_x_navy_blue, 17);
    bg_color_8bit!(bg_x_dark_blue, 18);
    bg_color_8bit!(bg_x_blue_3a, 19);
    bg_color_8bit!(bg_x_blue_3b, 20);
    bg_color_8bit!(bg_x_blue_1, 21);
    bg_color_8bit!(bg_x_dark_green, 22);
    bg_color_8bit!(bg_x_deep_sky_blue_4a, 23);
    bg_color_8bit!(bg_x_deep_sky_blue_4b, 24);
    bg_color_8bit!(bg_x_deep_sky_blue_4c, 25);
    bg_color_8bit!(bg_x_dodger_blue_3, 26);
    bg_color_8bit!(bg_x_dodger_blue_2, 27);
    bg_color_8bit!(bg_x_green_4, 28);
    bg_color_8bit!(bg_x_spring_green_4, 29);
    bg_color_8bit!(bg_x_turquoise_4, 30);
    bg_color_8bit!(bg_x_deep_sky_blue_3a, 31);
    bg_color_8bit!(bg_x_deep_sky_blue_3b, 32);
    bg_color_8bit!(bg_x_dodger_blue_1, 33);
    bg_color_8bit!(bg_x_green_3a, 34);
    bg_color_8bit!(bg_x_spring_green_3a, 35);
    bg_color_8bit!(bg_x_dark_cyan, 36);
    bg_color_8bit!(bg_x_light_sea_green, 37);
    bg_color_8bit!(bg_x_deep_sky_blue_2, 38);
    bg_color_8bit!(bg_x_deep_sky_blue_1, 39);
    bg_color_8bit!(bg_x_green_3b, 40);
    bg_color_8bit!(bg_x_spring_green_3b, 41);
    bg_color_8bit!(bg_x_spring_green_2a, 42);
    bg_color_8bit!(bg_x_cyan_3, 43);
    bg_color_8bit!(bg_x_dark_turquoise, 44);
    bg_color_8bit!(bg_x_turquoise_2, 45);
    bg_color_8bit!(bg_x_green_1, 46);
    bg_color_8bit!(bg_x_spring_green_2b, 47);
    bg_color_8bit!(bg_x_spring_green_1, 48);
    bg_color_8bit!(bg_x_medium_spring_green, 49);
    bg_color_8bit!(bg_x_cyan_2, 50);
    bg_color_8bit!(bg_x_cyan_1, 51);
    bg_color_8bit!(bg_x_dark_red_a, 52);
    bg_color_8bit!(bg_x_deep_pink_4a, 53);
    bg_color_8bit!(bg_x_purple_4a, 54);
    bg_color_8bit!(bg_x_purple_4b, 55);
    bg_color_8bit!(bg_x_purple_3, 56);
    bg_color_8bit!(bg_x_blue_violet, 57);
    bg_color_8bit!(bg_x_orange_4a, 58);
    bg_color_8bit!(bg_x_grey_37, 59);
    bg_color_8bit!(bg_x_medium_purple_4, 60);
    bg_color_8bit!(bg_x_slate_blue_3a, 61);
    bg_color_8bit!(bg_x_slate_blue_3b, 62);
    bg_color_8bit!(bg_x_royal_blue_1, 63);
    bg_color_8bit!(bg_x_chartreuse_4, 64);
    bg_color_8bit!(bg_x_dark_sea_green_4a, 65);
    bg_color_8bit!(bg_x_pale_turquoise_4, 66);
    bg_color_8bit!(bg_x_steel_blue, 67);
    bg_color_8bit!(bg_x_steel_blue_3, 68);
    bg_color_8bit!(bg_x_cornflower_blue, 69);
    bg_color_8bit!(bg_x_chartreuse_3a, 70);
    bg_color_8bit!(bg_x_dark_sea_green_4b, 71);
    bg_color_8bit!(bg_x_cadet_blue_a, 72);
    bg_color_8bit!(bg_x_cadet_blue_b, 73);
    bg_color_8bit!(bg_x_sky_blue_3, 74);
    bg_color_8bit!(bg_x_steel_blue_1a, 75);
    bg_color_8bit!(bg_x_chartreuse_3b, 76);
    bg_color_8bit!(bg_x_pale_green_3a, 77);
    bg_color_8bit!(bg_x_sea_green_3, 78);
    bg_color_8bit!(bg_x_aquamarine_3, 79);
    bg_color_8bit!(bg_x_medium_turquoise, 80);
    bg_color_8bit!(bg_x_steel_blue_1b, 81);
    bg_color_8bit!(bg_x_chartreuse_2a, 82);
    bg_color_8bit!(bg_x_sea_green_2, 83);
    bg_color_8bit!(bg_x_sea_green_1a, 84);
    bg_color_8bit!(bg_x_sea_green_1b, 85);
    bg_color_8bit!(bg_x_aquamarine_1a, 86);
    bg_color_8bit!(bg_x_dark_slate_gray_2, 87);
    bg_color_8bit!(bg_x_dark_red_b, 88);
    bg_color_8bit!(bg_x_deep_pink_4b, 89);
    bg_color_8bit!(bg_x_dark_magenta_a, 90);
    bg_color_8bit!(bg_x_dark_magenta_b, 91);
    bg_color_8bit!(bg_x_dark_violet_a, 92);
    bg_color_8bit!(bg_x_purple_a, 93);
    bg_color_8bit!(bg_x_orange_4b, 94);
    bg_color_8bit!(bg_x_light_pink_4, 95);
    bg_color_8bit!(bg_x_plum_4, 96);
    bg_color_8bit!(bg_x_medium_purple_3a, 97);
    bg_color_8bit!(bg_x_medium_purple_3b, 98);
    bg_color_8bit!(bg_x_slate_blue_1, 99);
    bg_color_8bit!(bg_x_yellow_4a, 100);
    bg_color_8bit!(bg_x_wheat_4, 101);
    bg_color_8bit!(bg_x_grey_53, 102);
    bg_color_8bit!(bg_x_light_slate_grey, 103);
    bg_color_8bit!(bg_x_medium_purple, 104);
    bg_color_8bit!(bg_x_light_slate_blue, 105);
    bg_color_8bit!(bg_x_yellow_4b, 106);
    bg_color_8bit!(bg_x_dark_olive_green_3a, 107);
    bg_color_8bit!(bg_x_dark_sea_green, 108);
    bg_color_8bit!(bg_x_light_sky_blue_3a, 109);
    bg_color_8bit!(bg_x_light_sky_blue_3b, 110);
    bg_color_8bit!(bg_x_sky_blue_2, 111);
    bg_color_8bit!(bg_x_chartreuse_2b, 112);
    bg_color_8bit!(bg_x_dark_olive_green_3b, 113);
    bg_color_8bit!(bg_x_pale_green_3b, 114);
    bg_color_8bit!(bg_x_dark_sea_green_3a, 115);
    bg_color_8bit!(bg_x_dark_slate_gray_3, 116);
    bg_color_8bit!(bg_x_sky_blue_1, 117);
    bg_color_8bit!(bg_x_chartreuse_1, 118);
    bg_color_8bit!(bg_x_light_green_a, 119);
    bg_color_8bit!(bg_x_light_green_b, 120);
    bg_color_8bit!(bg_x_pale_green_1a, 121);
    bg_color_8bit!(bg_x_aquamarine_1b, 122);
    bg_color_8bit!(bg_x_dark_slate_gray_1, 123);
    bg_color_8bit!(bg_x_red_3a, 124);
    bg_color_8bit!(bg_x_deep_pink_4c, 125);
    bg_color_8bit!(bg_x_medium_violet_red, 126);
    bg_color_8bit!(bg_x_magenta_3a, 127);
    bg_color_8bit!(bg_x_dark_violet_b, 128);
    bg_color_8bit!(bg_x_purple_b, 129);
    bg_color_8bit!(bg_x_dark_orange_3a, 130);
    bg_color_8bit!(bg_x_indian_red_a, 131);
    bg_color_8bit!(bg_x_hot_pink_3a, 132);
    bg_color_8bit!(bg_x_medium_orchid_3, 133);
    bg_color_8bit!(bg_x_medium_orchid, 134);
    bg_color_8bit!(bg_x_medium_purple_2a, 135);
    bg_color_8bit!(bg_x_dark_goldenrod, 136);
    bg_color_8bit!(bg_x_light_salmon_3a, 137);
    bg_color_8bit!(bg_x_rosy_brown, 138);
    bg_color_8bit!(bg_x_grey_63, 139);
    bg_color_8bit!(bg_x_medium_purple_2b, 140);
    bg_color_8bit!(bg_x_medium_purple_1, 141);
    bg_color_8bit!(bg_x_gold_3a, 142);
    bg_color_8bit!(bg_x_dark_khaki, 143);
    bg_color_8bit!(bg_x_navajo_white_3, 144);
    bg_color_8bit!(bg_x_grey_69, 145);
    bg_color_8bit!(bg_x_light_steel_blue_3, 146);
    bg_color_8bit!(bg_x_light_steel_blue, 147);
    bg_color_8bit!(bg_x_yellow_3a, 148);
    bg_color_8bit!(bg_x_dark_olive_green_3c, 149);
    bg_color_8bit!(bg_x_dark_sea_green_3b, 150);
    bg_color_8bit!(bg_x_dark_sea_green_2a, 151);
    bg_color_8bit!(bg_x_light_cyan_3, 152);
    bg_color_8bit!(bg_x_light_sky_blue_1, 153);
    bg_color_8bit!(bg_x_green_yellow, 154);
    bg_color_8bit!(bg_x_dark_olive_green_2, 155);
    bg_color_8bit!(bg_x_pale_green_1b, 156);
    bg_color_8bit!(bg_x_dark_sea_green_2b, 157);
    bg_color_8bit!(bg_x_dark_sea_green_1a, 158);
    bg_color_8bit!(bg_x_pale_turquoise_1, 159);
    bg_color_8bit!(bg_x_red_3b, 160);
    bg_color_8bit!(bg_x_deep_pink_3a, 161);
    bg_color_8bit!(bg_x_deep_pink_3b, 162);
    bg_color_8bit!(bg_x_magenta_3b, 163);
    bg_color_8bit!(bg_x_magenta_3c, 164);
    bg_color_8bit!(bg_x_magenta_2a, 165);
    bg_color_8bit!(bg_x_dark_orange_3b, 166);
    bg_color_8bit!(bg_x_indian_red_b, 167);
    bg_color_8bit!(bg_x_hot_pink_3b, 168);
    bg_color_8bit!(bg_x_hot_pink_2, 169);
    bg_color_8bit!(bg_x_orchid, 170);
    bg_color_8bit!(bg_x_medium_orchid_1a, 171);
    bg_color_8bit!(bg_x_orange_3, 172);
    bg_color_8bit!(bg_x_light_salmon_3b, 173);
    bg_color_8bit!(bg_x_light_pink_3, 174);
    bg_color_8bit!(bg_x_pink_3, 175);
    bg_color_8bit!(bg_x_plum_3, 176);
    bg_color_8bit!(bg_x_violet, 177);
    bg_color_8bit!(bg_x_gold_3b, 178);
    bg_color_8bit!(bg_x_light_goldenrod_3, 179);
    bg_color_8bit!(bg_x_tan, 180);
    bg_color_8bit!(bg_x_misty_rose_3, 181);
    bg_color_8bit!(bg_x_thistle_3, 182);
    bg_color_8bit!(bg_x_plum_2, 183);
    bg_color_8bit!(bg_x_yellow_3b, 184);
    bg_color_8bit!(bg_x_khaki_3, 185);
    bg_color_8bit!(bg_x_light_goldenrod_2a, 186);
    bg_color_8bit!(bg_x_light_yellow_3, 187);
    bg_color_8bit!(bg_x_grey_84, 188);
    bg_color_8bit!(bg_x_light_steel_blue_1, 189);
    bg_color_8bit!(bg_x_yellow_2, 190);
    bg_color_8bit!(bg_x_dark_olive_green_1a, 191);
    bg_color_8bit!(bg_x_dark_olive_green_1b, 192);
    bg_color_8bit!(bg_x_dark_sea_green_1b, 193);
    bg_color_8bit!(bg_x_honeydew_2, 194);
    bg_color_8bit!(bg_x_light_cyan_1, 195);
    bg_color_8bit!(bg_x_red_1, 196);
    bg_color_8bit!(bg_x_deep_pink_2, 197);
    bg_color_8bit!(bg_x_deep_pink_1a, 198);
    bg_color_8bit!(bg_x_deep_pink_1b, 199);
    bg_color_8bit!(bg_x_magenta_2b, 200);
    bg_color_8bit!(bg_x_magenta_1, 201);
    bg_color_8bit!(bg_x_orange_red_1, 202);
    bg_color_8bit!(bg_x_indian_red_1a, 203);
    bg_color_8bit!(bg_x_indian_red_1b, 204);
    bg_color_8bit!(bg_x_hot_pink_a, 205);
    bg_color_8bit!(bg_x_hot_pink_b, 206);
    bg_color_8bit!(bg_x_medium_orchid_1b, 207);
    bg_color_8bit!(bg_x_dark_orange, 208);
    bg_color_8bit!(bg_x_salmon_1, 209);
    bg_color_8bit!(bg_x_light_coral, 210);
    bg_color_8bit!(bg_x_pale_violet_red_1, 211);
    bg_color_8bit!(bg_x_orchid_2, 212);
    bg_color_8bit!(bg_x_orchid_1, 213);
    bg_color_8bit!(bg_x_orange_1, 214);
    bg_color_8bit!(bg_x_sandy_brown, 215);
    bg_color_8bit!(bg_x_light_salmon_1, 216);
    bg_color_8bit!(bg_x_light_pink_1, 217);
    bg_color_8bit!(bg_x_pink_1, 218);
    bg_color_8bit!(bg_x_plum_1, 219);
    bg_color_8bit!(bg_x_gold_1, 220);
    bg_color_8bit!(bg_x_light_goldenrod_2b, 221);
    bg_color_8bit!(bg_x_light_goldenrod_2c, 222);
    bg_color_8bit!(bg_x_navajo_white_1, 223);
    bg_color_8bit!(bg_x_misty_rose_1, 224);
    bg_color_8bit!(bg_x_thistle_1, 225);
    bg_color_8bit!(bg_x_yellow_1, 226);
    bg_color_8bit!(bg_x_light_goldenrod_1, 227);
    bg_color_8bit!(bg_x_khaki_1, 228);
    bg_color_8bit!(bg_x_wheat_1, 229);
    bg_color_8bit!(bg_x_cornsilk_1, 230);
    bg_color_8bit!(bg_x_grey_100, 231);
    bg_color_8bit!(bg_x_grey_3, 232);
    bg_color_8bit!(bg_x_grey_7, 233);
    bg_color_8bit!(bg_x_grey_11, 234);
    bg_color_8bit!(bg_x_grey_15, 235);
    bg_color_8bit!(bg_x_grey_19, 236);
    bg_color_8bit!(bg_x_grey_23, 237);
    bg_color_8bit!(bg_x_grey_27, 238);
    bg_color_8bit!(bg_x_grey_30, 239);
    bg_color_8bit!(bg_x_grey_35, 240);
    bg_color_8bit!(bg_x_grey_39, 241);
    bg_color_8bit!(bg_x_grey_42, 242);
    bg_color_8bit!(bg_x_grey_46, 243);
    bg_color_8bit!(bg_x_grey_50, 244);
    bg_color_8bit!(bg_x_grey_54, 245);
    bg_color_8bit!(bg_x_grey_58, 246);
    bg_color_8bit!(bg_x_grey_62, 247);
    bg_color_8bit!(bg_x_grey_66, 248);
    bg_color_8bit!(bg_x_grey_70, 249);
    bg_color_8bit!(bg_x_grey_74, 250);
    bg_color_8bit!(bg_x_grey_78, 251);
    bg_color_8bit!(bg_x_grey_82, 252);
    bg_color_8bit!(bg_x_grey_85, 253);
    bg_color_8bit!(bg_x_grey_89, 254);
    bg_color_8bit!(bg_x_grey_93, 255);
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
            ColorMode::Color8bit => {
                write!(f, "{}", self.format_colors_8bit()).unwrap_or(());
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
        let str = String::from(&Color::new().magenta().fix());

        assert_eq!(str, "\x1b[0;35m{}\x1b[0m");
    }

    #[test]
    fn string_from_owned() {
        let str = String::from(Color::new().magenta().fix());

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

    #[test]
    fn fix() {
        let owned_color = Color::new().magenta().fix();

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

    // 8-bit.

    #[test]
    fn color_8bit() {
        assert_eq!(
            Color::new().x_cadet_blue_a().format("hello, world"),
            "\x1b[0;38;5;72mhello, world\x1b[0m"
        );
    }

    #[test]
    fn color_8bit_bg() {
        assert_eq!(
            Color::new().bg_x_salmon_1().format("hello, world"),
            "\x1b[0;48;5;209mhello, world\x1b[0m"
        );
    }
    #[test]
    fn color_8bit_with_bg() {
        assert_eq!(
            Color::new()
                .x_dark_sea_green()
                .bg_x_spring_green_2a()
                .format("hello, world"),
            "\x1b[0;38;5;108m\x1b[48;5;42mhello, world\x1b[0m"
        );
    }
}
