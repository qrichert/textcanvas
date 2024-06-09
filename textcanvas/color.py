import enum
from typing import Any, Self

ESC: str = "\x1b["
RESET: str = "\x1b[0m"
PLACEHOLDER: str = "{}"


class ColorMode(enum.Enum):
    NO_COLOR = "NO_COLOR"
    COLOR_RGB = "COLOR_RGB"
    COLOR_4BIT = "COLOR_4BIT"
    COLOR_8BIT = "COLOR_8BIT"


class Color:
    """Color for the terminal.

    Three color modes are available:

    - 4-bit colors.
    - 8-bit Xterm colors (prefixed by `x_`).
    - 24-bit RGB colors (`rgb` methods).

    Examples:
        >>> # Bright red text.
        >>> Color().bright_red().format("hello, world")
        '\\x1b[0;91mhello, world\\x1b[0m'

        >>> # Bold-underlined red text over green background.
        >>> Color().bold().underline().red().bg_green().format("hello, world")
        '\\x1b[1;4;31;42mhello, world\\x1b[0m'

        >>> # 8-bit foreground and background.
        >>> Color().x_dark_goldenrod().bg_x_aquamarine_3().format("hello, world")
        '\\x1b[0;38;5;136m\\x1b[48;5;79mhello, world\\x1b[0m'

        >>> # RGB background.
        >>> Color().bg_rgb(45, 227, 61).format("hello, world")
        '\\x1b[0;48;2;45;227;61mhello, world\\x1b[0m'

        >>> # RGB text from hex value.
        >>> Color().rbg_from_hex("#1f2c3b").format("hello, world")
        '\\x1b[0;38;2;31;44;59mhello, world\\x1b[0m'

    Limitations:
        <div class="warning">

        RGB (24-bit) colors do not work on every terminal.

        </div>

        <div class="warning">

        `italic()` isn't widely supported in terminal implementations,
        and is sometimes treated as inverse or blink.

        </div>

        <div class="warning">

        Mixing foreground and background colors of different modes
        doesn't work. For example, if you have an 8-bit background, use
        an 8-bit foreground as well.

        </div>

    Technical Details:
        - `ESC` = `0o33` = `0x1b` = Escape character, start of escape sequence.
        - `ESC[` = Control sequence.
        - `ESC[0m` = Reset sequence.

        - `ESC[0⟨n⟩m]` = `⟨n⟩` is one of 16 color codes (4-bit)

        - `ESC[38;5;⟨n⟩m` Select 8-bit foreground color
        - `ESC[48;5;⟨n⟩m` Select 8-bit background color

        - `ESC[382⟨r⟩⟨g⟩⟨b⟩m` = `[0255]` Select RGB foreground color
        - `ESC[482⟨r⟩⟨g⟩⟨b⟩m` = `[0255]` Select RGB background color

        See Also:
            - https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
    """

    def __init__(self) -> None:
        self._mode: ColorMode = ColorMode.NO_COLOR
        self._color_rgb: tuple[int, int, int] | None = None
        self._bg_color_rgb: tuple[int, int, int] | None = None
        self._color_4bit: int | None = None
        self._bg_color_4bit: int | None = None
        self._color_8bit: int | None = None
        self._bg_color_8bit: int | None = None
        self._is_bold: bool = False
        self._is_italic: bool = False
        self._is_underlined: bool = False

    def __eq__(self, other: Any) -> bool:
        return self.to_string() == other

    def to_string(self) -> str:
        if self._is_empty():
            return PLACEHOLDER

        res: str = ESC
        res += self._format_display_attributes()

        if self._has_colors():
            res += ";"
        match self._mode:
            case ColorMode.COLOR_RGB:
                res += self._format_colors_rgb()
            case ColorMode.COLOR_4BIT:
                res += self._format_colors_4bit()
            case ColorMode.COLOR_8BIT:
                res += self._format_colors_8bit()

        return res + f"m{PLACEHOLDER}{RESET}"

    def format(self, string: str) -> str:
        return self.to_string().replace(PLACEHOLDER, string)

    def _is_empty(self) -> bool:
        return self._mode == ColorMode.NO_COLOR and not self._has_display_attributes()

    def _has_colors(self) -> bool:
        return not self._mode == ColorMode.NO_COLOR

    # Display Attributes.

    def bold(self) -> Self:
        self._is_bold = True
        return self

    def italic(self) -> Self:
        self._is_italic = True
        return self

    def underline(self) -> Self:
        self._is_underlined = True
        return self

    def _format_display_attributes(self) -> str:
        if not self._has_display_attributes():
            return "0"
        attributes: list[str] = []
        if self._is_bold:
            attributes.append("1")
        if self._is_italic:
            attributes.append("3")
        if self._is_underlined:
            attributes.append("4")
        return ";".join(attributes)

    def _has_display_attributes(self) -> bool:
        return self._is_bold or self._is_italic or self._is_underlined

    # RGB colors (24-bit).

    def _apply_color_rgb(self, red: int, green: int, blue: int) -> Self:
        self._mode = ColorMode.COLOR_RGB
        self._color_rgb = (red, green, blue)
        return self

    def _apply_bg_color_rgb(self, red: int, green: int, blue: int) -> Self:
        self._mode = ColorMode.COLOR_RGB
        self._bg_color_rgb = (red, green, blue)
        return self

    def rgb(self, red: int, green: int, blue: int) -> Self:
        return self._apply_color_rgb(red, green, blue)

    def bg_rgb(self, red: int, green: int, blue: int) -> Self:
        return self._apply_bg_color_rgb(red, green, blue)

    def rbg_from_hex(self, hex_color: str) -> Self:
        r, g, b = self._hex_to_rgb(hex_color)
        return self.rgb(r, g, b)

    def bg_rbg_from_hex(self, hex_color: str) -> Self:
        r, g, b = self._hex_to_rgb(hex_color)
        return self.bg_rgb(r, g, b)

    @staticmethod
    def _hex_to_rgb(hex_color: str) -> tuple[int, int, int]:
        if hex_color.startswith("#"):
            hex_color = hex_color[1:]

        if len(hex_color) != 6:
            return 0, 0, 0

        red: int = 0
        green: int = 0
        blue: int = 0

        try:
            red = int(hex_color[:2], 16)
        except ValueError:
            pass
        try:
            green = int(hex_color[2:4], 16)
        except ValueError:
            pass
        try:
            blue = int(hex_color[4:], 16)
        except ValueError:
            pass

        return red, green, blue

    def _format_colors_rgb(self) -> str:
        colors: str = ""
        # Foreground.
        if self._color_rgb is not None:
            red, green, blue = self._color_rgb
            colors += f"38;2;{red};{green};{blue}"

            # Foreground AND Background.
            if self._bg_color_rgb is not None:
                # Close foreground sequence and start new one.
                colors += f"m{ESC}"

        # Background.
        if self._bg_color_rgb is not None:
            red, green, blue = self._bg_color_rgb
            colors += f"48;2;{red};{green};{blue}"

        return colors

    # 4-bit colors.

    def _apply_color_4bit(self, color: int) -> Self:
        self._mode = ColorMode.COLOR_4BIT
        self._color_4bit = color
        return self

    def _apply_bg_color_4bit(self, color: int) -> Self:
        self._mode = ColorMode.COLOR_4BIT
        self._bg_color_4bit = color
        return self

    def _format_colors_4bit(self) -> str:
        colors: str = ""
        # Foreground.
        if self._color_4bit is not None:
            colors += f"{self._color_4bit}"

            # Foreground AND Background.
            if self._bg_color_4bit is not None:
                colors += ";"

        # Background.
        if self._bg_color_4bit is not None:
            colors += f"{self._bg_color_4bit}"

        return colors

    # fmt: off
    def red(self) -> Self: return self._apply_color_4bit(31)
    def yellow(self) -> Self: return self._apply_color_4bit(33)
    def green(self) -> Self: return self._apply_color_4bit(32)
    def blue(self) -> Self: return self._apply_color_4bit(34)
    def cyan(self) -> Self: return self._apply_color_4bit(36)
    def magenta(self) -> Self: return self._apply_color_4bit(35)
    def gray(self) -> Self: return self._apply_color_4bit(30)
    def white(self) -> Self: return self._apply_color_4bit(37)

    def bright_red(self) -> Self: return self._apply_color_4bit(91)
    def bright_yellow(self) -> Self: return self._apply_color_4bit(93)
    def bright_green(self) -> Self: return self._apply_color_4bit(92)
    def bright_blue(self) -> Self: return self._apply_color_4bit(94)
    def bright_cyan(self) -> Self: return self._apply_color_4bit(96)
    def bright_magenta(self) -> Self: return self._apply_color_4bit(95)
    def bright_gray(self) -> Self: return self._apply_color_4bit(90)
    def bright_white(self) -> Self: return self._apply_color_4bit(97)

    def bg_red(self) -> Self: return self._apply_bg_color_4bit(41)
    def bg_yellow(self) -> Self: return self._apply_bg_color_4bit(43)
    def bg_green(self) -> Self: return self._apply_bg_color_4bit(42)
    def bg_blue(self) -> Self: return self._apply_bg_color_4bit(44)
    def bg_cyan(self) -> Self: return self._apply_bg_color_4bit(46)
    def bg_magenta(self) -> Self: return self._apply_bg_color_4bit(45)
    def bg_gray(self) -> Self: return self._apply_bg_color_4bit(40)
    def bg_white(self) -> Self: return self._apply_bg_color_4bit(47)

    def bg_bright_red(self) -> Self: return self._apply_bg_color_4bit(101)
    def bg_bright_yellow(self) -> Self: return self._apply_bg_color_4bit(103)
    def bg_bright_green(self) -> Self: return self._apply_bg_color_4bit(102)
    def bg_bright_blue(self) -> Self: return self._apply_bg_color_4bit(104)
    def bg_bright_cyan(self) -> Self: return self._apply_bg_color_4bit(106)
    def bg_bright_magenta(self) -> Self: return self._apply_bg_color_4bit(105)
    def bg_bright_gray(self) -> Self: return self._apply_bg_color_4bit(100)
    def bg_bright_white(self) -> Self: return self._apply_bg_color_4bit(107)
    # fmt: on

    # 8-bit colors.

    def _apply_color_8bit(self, color: int) -> Self:
        self._mode = ColorMode.COLOR_8BIT
        self._color_8bit = color
        return self

    def _apply_bg_color_8bit(self, color: int) -> Self:
        self._mode = ColorMode.COLOR_8BIT
        self._bg_color_8bit = color
        return self

    def _format_colors_8bit(self) -> str:
        colors: str = ""
        # Foreground.
        if self._color_8bit is not None:
            colors += f"38;5;{self._color_8bit}"

            # Foreground AND Background.
            if self._bg_color_8bit is not None:
                # Close foreground sequence and start new one.
                colors += f"m{ESC}"

        # Background.
        if self._bg_color_8bit is not None:
            colors += f"48;5;{self._bg_color_8bit}"

        return colors

    # fmt: off
    def x_black(self) -> Self: return self._apply_color_8bit(0)
    def x_maroon(self) -> Self: return self._apply_color_8bit(1)
    def x_green(self) -> Self: return self._apply_color_8bit(2)
    def x_olive(self) -> Self: return self._apply_color_8bit(3)
    def x_navy(self) -> Self: return self._apply_color_8bit(4)
    def x_purple(self) -> Self: return self._apply_color_8bit(5)
    def x_teal(self) -> Self: return self._apply_color_8bit(6)
    def x_silver(self) -> Self: return self._apply_color_8bit(7)
    def x_grey(self) -> Self: return self._apply_color_8bit(8)
    def x_red(self) -> Self: return self._apply_color_8bit(9)
    def x_lime(self) -> Self: return self._apply_color_8bit(10)
    def x_yellow(self) -> Self: return self._apply_color_8bit(11)
    def x_blue(self) -> Self: return self._apply_color_8bit(12)
    def x_fuchsia(self) -> Self: return self._apply_color_8bit(13)
    def x_aqua(self) -> Self: return self._apply_color_8bit(14)
    def x_white(self) -> Self: return self._apply_color_8bit(15)
    def x_grey_0(self) -> Self: return self._apply_color_8bit(16)
    def x_navy_blue(self) -> Self: return self._apply_color_8bit(17)
    def x_dark_blue(self) -> Self: return self._apply_color_8bit(18)
    def x_blue_3a(self) -> Self: return self._apply_color_8bit(19)
    def x_blue_3b(self) -> Self: return self._apply_color_8bit(20)
    def x_blue_1(self) -> Self: return self._apply_color_8bit(21)
    def x_dark_green(self) -> Self: return self._apply_color_8bit(22)
    def x_deep_sky_blue_4a(self) -> Self: return self._apply_color_8bit(23)
    def x_deep_sky_blue_4b(self) -> Self: return self._apply_color_8bit(24)
    def x_deep_sky_blue_4c(self) -> Self: return self._apply_color_8bit(25)
    def x_dodger_blue_3(self) -> Self: return self._apply_color_8bit(26)
    def x_dodger_blue_2(self) -> Self: return self._apply_color_8bit(27)
    def x_green_4(self) -> Self: return self._apply_color_8bit(28)
    def x_spring_green_4(self) -> Self: return self._apply_color_8bit(29)
    def x_turquoise_4(self) -> Self: return self._apply_color_8bit(30)
    def x_deep_sky_blue_3a(self) -> Self: return self._apply_color_8bit(31)
    def x_deep_sky_blue_3b(self) -> Self: return self._apply_color_8bit(32)
    def x_dodger_blue_1(self) -> Self: return self._apply_color_8bit(33)
    def x_green_3a(self) -> Self: return self._apply_color_8bit(34)
    def x_spring_green_3a(self) -> Self: return self._apply_color_8bit(35)
    def x_dark_cyan(self) -> Self: return self._apply_color_8bit(36)
    def x_light_sea_green(self) -> Self: return self._apply_color_8bit(37)
    def x_deep_sky_blue_2(self) -> Self: return self._apply_color_8bit(38)
    def x_deep_sky_blue_1(self) -> Self: return self._apply_color_8bit(39)
    def x_green_3b(self) -> Self: return self._apply_color_8bit(40)
    def x_spring_green_3b(self) -> Self: return self._apply_color_8bit(41)
    def x_spring_green_2a(self) -> Self: return self._apply_color_8bit(42)
    def x_cyan_3(self) -> Self: return self._apply_color_8bit(43)
    def x_dark_turquoise(self) -> Self: return self._apply_color_8bit(44)
    def x_turquoise_2(self) -> Self: return self._apply_color_8bit(45)
    def x_green_1(self) -> Self: return self._apply_color_8bit(46)
    def x_spring_green_2b(self) -> Self: return self._apply_color_8bit(47)
    def x_spring_green_1(self) -> Self: return self._apply_color_8bit(48)
    def x_medium_spring_green(self) -> Self: return self._apply_color_8bit(49)
    def x_cyan_2(self) -> Self: return self._apply_color_8bit(50)
    def x_cyan_1(self) -> Self: return self._apply_color_8bit(51)
    def x_dark_red_a(self) -> Self: return self._apply_color_8bit(52)
    def x_deep_pink_4a(self) -> Self: return self._apply_color_8bit(53)
    def x_purple_4a(self) -> Self: return self._apply_color_8bit(54)
    def x_purple_4b(self) -> Self: return self._apply_color_8bit(55)
    def x_purple_3(self) -> Self: return self._apply_color_8bit(56)
    def x_blue_violet(self) -> Self: return self._apply_color_8bit(57)
    def x_orange_4a(self) -> Self: return self._apply_color_8bit(58)
    def x_grey_37(self) -> Self: return self._apply_color_8bit(59)
    def x_medium_purple_4(self) -> Self: return self._apply_color_8bit(60)
    def x_slate_blue_3a(self) -> Self: return self._apply_color_8bit(61)
    def x_slate_blue_3b(self) -> Self: return self._apply_color_8bit(62)
    def x_royal_blue_1(self) -> Self: return self._apply_color_8bit(63)
    def x_chartreuse_4(self) -> Self: return self._apply_color_8bit(64)
    def x_dark_sea_green_4a(self) -> Self: return self._apply_color_8bit(65)
    def x_pale_turquoise_4(self) -> Self: return self._apply_color_8bit(66)
    def x_steel_blue(self) -> Self: return self._apply_color_8bit(67)
    def x_steel_blue_3(self) -> Self: return self._apply_color_8bit(68)
    def x_cornflower_blue(self) -> Self: return self._apply_color_8bit(69)
    def x_chartreuse_3a(self) -> Self: return self._apply_color_8bit(70)
    def x_dark_sea_green_4b(self) -> Self: return self._apply_color_8bit(71)
    def x_cadet_blue_a(self) -> Self: return self._apply_color_8bit(72)
    def x_cadet_blue_b(self) -> Self: return self._apply_color_8bit(73)
    def x_sky_blue_3(self) -> Self: return self._apply_color_8bit(74)
    def x_steel_blue_1a(self) -> Self: return self._apply_color_8bit(75)
    def x_chartreuse_3b(self) -> Self: return self._apply_color_8bit(76)
    def x_pale_green_3a(self) -> Self: return self._apply_color_8bit(77)
    def x_sea_green_3(self) -> Self: return self._apply_color_8bit(78)
    def x_aquamarine_3(self) -> Self: return self._apply_color_8bit(79)
    def x_medium_turquoise(self) -> Self: return self._apply_color_8bit(80)
    def x_steel_blue_1b(self) -> Self: return self._apply_color_8bit(81)
    def x_chartreuse_2a(self) -> Self: return self._apply_color_8bit(82)
    def x_sea_green_2(self) -> Self: return self._apply_color_8bit(83)
    def x_sea_green_1a(self) -> Self: return self._apply_color_8bit(84)
    def x_sea_green_1b(self) -> Self: return self._apply_color_8bit(85)
    def x_aquamarine_1a(self) -> Self: return self._apply_color_8bit(86)
    def x_dark_slate_gray_2(self) -> Self: return self._apply_color_8bit(87)
    def x_dark_red_b(self) -> Self: return self._apply_color_8bit(88)
    def x_deep_pink_4b(self) -> Self: return self._apply_color_8bit(89)
    def x_dark_magenta_a(self) -> Self: return self._apply_color_8bit(90)
    def x_dark_magenta_b(self) -> Self: return self._apply_color_8bit(91)
    def x_dark_violet_a(self) -> Self: return self._apply_color_8bit(92)
    def x_purple_a(self) -> Self: return self._apply_color_8bit(93)
    def x_orange_4b(self) -> Self: return self._apply_color_8bit(94)
    def x_light_pink_4(self) -> Self: return self._apply_color_8bit(95)
    def x_plum_4(self) -> Self: return self._apply_color_8bit(96)
    def x_medium_purple_3a(self) -> Self: return self._apply_color_8bit(97)
    def x_medium_purple_3b(self) -> Self: return self._apply_color_8bit(98)
    def x_slate_blue_1(self) -> Self: return self._apply_color_8bit(99)
    def x_yellow_4a(self) -> Self: return self._apply_color_8bit(100)
    def x_wheat_4(self) -> Self: return self._apply_color_8bit(101)
    def x_grey_53(self) -> Self: return self._apply_color_8bit(102)
    def x_light_slate_grey(self) -> Self: return self._apply_color_8bit(103)
    def x_medium_purple(self) -> Self: return self._apply_color_8bit(104)
    def x_light_slate_blue(self) -> Self: return self._apply_color_8bit(105)
    def x_yellow_4b(self) -> Self: return self._apply_color_8bit(106)
    def x_dark_olive_green_3a(self) -> Self: return self._apply_color_8bit(107)
    def x_dark_sea_green(self) -> Self: return self._apply_color_8bit(108)
    def x_light_sky_blue_3a(self) -> Self: return self._apply_color_8bit(109)
    def x_light_sky_blue_3b(self) -> Self: return self._apply_color_8bit(110)
    def x_sky_blue_2(self) -> Self: return self._apply_color_8bit(111)
    def x_chartreuse_2b(self) -> Self: return self._apply_color_8bit(112)
    def x_dark_olive_green_3b(self) -> Self: return self._apply_color_8bit(113)
    def x_pale_green_3b(self) -> Self: return self._apply_color_8bit(114)
    def x_dark_sea_green_3a(self) -> Self: return self._apply_color_8bit(115)
    def x_dark_slate_gray_3(self) -> Self: return self._apply_color_8bit(116)
    def x_sky_blue_1(self) -> Self: return self._apply_color_8bit(117)
    def x_chartreuse_1(self) -> Self: return self._apply_color_8bit(118)
    def x_light_green_a(self) -> Self: return self._apply_color_8bit(119)
    def x_light_green_b(self) -> Self: return self._apply_color_8bit(120)
    def x_pale_green_1a(self) -> Self: return self._apply_color_8bit(121)
    def x_aquamarine_1b(self) -> Self: return self._apply_color_8bit(122)
    def x_dark_slate_gray_1(self) -> Self: return self._apply_color_8bit(123)
    def x_red_3a(self) -> Self: return self._apply_color_8bit(124)
    def x_deep_pink_4c(self) -> Self: return self._apply_color_8bit(125)
    def x_medium_violet_red(self) -> Self: return self._apply_color_8bit(126)
    def x_magenta_3a(self) -> Self: return self._apply_color_8bit(127)
    def x_dark_violet_b(self) -> Self: return self._apply_color_8bit(128)
    def x_purple_b(self) -> Self: return self._apply_color_8bit(129)
    def x_dark_orange_3a(self) -> Self: return self._apply_color_8bit(130)
    def x_indian_red_a(self) -> Self: return self._apply_color_8bit(131)
    def x_hot_pink_3a(self) -> Self: return self._apply_color_8bit(132)
    def x_medium_orchid_3(self) -> Self: return self._apply_color_8bit(133)
    def x_medium_orchid(self) -> Self: return self._apply_color_8bit(134)
    def x_medium_purple_2a(self) -> Self: return self._apply_color_8bit(135)
    def x_dark_goldenrod(self) -> Self: return self._apply_color_8bit(136)
    def x_light_salmon_3a(self) -> Self: return self._apply_color_8bit(137)
    def x_rosy_brown(self) -> Self: return self._apply_color_8bit(138)
    def x_grey_63(self) -> Self: return self._apply_color_8bit(139)
    def x_medium_purple_2b(self) -> Self: return self._apply_color_8bit(140)
    def x_medium_purple_1(self) -> Self: return self._apply_color_8bit(141)
    def x_gold_3a(self) -> Self: return self._apply_color_8bit(142)
    def x_dark_khaki(self) -> Self: return self._apply_color_8bit(143)
    def x_navajo_white_3(self) -> Self: return self._apply_color_8bit(144)
    def x_grey_69(self) -> Self: return self._apply_color_8bit(145)
    def x_light_steel_blue_3(self) -> Self: return self._apply_color_8bit(146)
    def x_light_steel_blue(self) -> Self: return self._apply_color_8bit(147)
    def x_yellow_3a(self) -> Self: return self._apply_color_8bit(148)
    def x_dark_olive_green_3c(self) -> Self: return self._apply_color_8bit(149)
    def x_dark_sea_green_3b(self) -> Self: return self._apply_color_8bit(150)
    def x_dark_sea_green_2a(self) -> Self: return self._apply_color_8bit(151)
    def x_light_cyan_3(self) -> Self: return self._apply_color_8bit(152)
    def x_light_sky_blue_1(self) -> Self: return self._apply_color_8bit(153)
    def x_green_yellow(self) -> Self: return self._apply_color_8bit(154)
    def x_dark_olive_green_2(self) -> Self: return self._apply_color_8bit(155)
    def x_pale_green_1b(self) -> Self: return self._apply_color_8bit(156)
    def x_dark_sea_green_2b(self) -> Self: return self._apply_color_8bit(157)
    def x_dark_sea_green_1a(self) -> Self: return self._apply_color_8bit(158)
    def x_pale_turquoise_1(self) -> Self: return self._apply_color_8bit(159)
    def x_red_3b(self) -> Self: return self._apply_color_8bit(160)
    def x_deep_pink_3a(self) -> Self: return self._apply_color_8bit(161)
    def x_deep_pink_3b(self) -> Self: return self._apply_color_8bit(162)
    def x_magenta_3b(self) -> Self: return self._apply_color_8bit(163)
    def x_magenta_3c(self) -> Self: return self._apply_color_8bit(164)
    def x_magenta_2a(self) -> Self: return self._apply_color_8bit(165)
    def x_dark_orange_3b(self) -> Self: return self._apply_color_8bit(166)
    def x_indian_red_b(self) -> Self: return self._apply_color_8bit(167)
    def x_hot_pink_3b(self) -> Self: return self._apply_color_8bit(168)
    def x_hot_pink_2(self) -> Self: return self._apply_color_8bit(169)
    def x_orchid(self) -> Self: return self._apply_color_8bit(170)
    def x_medium_orchid_1a(self) -> Self: return self._apply_color_8bit(171)
    def x_orange_3(self) -> Self: return self._apply_color_8bit(172)
    def x_light_salmon_3b(self) -> Self: return self._apply_color_8bit(173)
    def x_light_pink_3(self) -> Self: return self._apply_color_8bit(174)
    def x_pink_3(self) -> Self: return self._apply_color_8bit(175)
    def x_plum_3(self) -> Self: return self._apply_color_8bit(176)
    def x_violet(self) -> Self: return self._apply_color_8bit(177)
    def x_gold_3b(self) -> Self: return self._apply_color_8bit(178)
    def x_light_goldenrod_3(self) -> Self: return self._apply_color_8bit(179)
    def x_tan(self) -> Self: return self._apply_color_8bit(180)
    def x_misty_rose_3(self) -> Self: return self._apply_color_8bit(181)
    def x_thistle_3(self) -> Self: return self._apply_color_8bit(182)
    def x_plum_2(self) -> Self: return self._apply_color_8bit(183)
    def x_yellow_3b(self) -> Self: return self._apply_color_8bit(184)
    def x_khaki_3(self) -> Self: return self._apply_color_8bit(185)
    def x_light_goldenrod_2a(self) -> Self: return self._apply_color_8bit(186)
    def x_light_yellow_3(self) -> Self: return self._apply_color_8bit(187)
    def x_grey_84(self) -> Self: return self._apply_color_8bit(188)
    def x_light_steel_blue_1(self) -> Self: return self._apply_color_8bit(189)
    def x_yellow_2(self) -> Self: return self._apply_color_8bit(190)
    def x_dark_olive_green_1a(self) -> Self: return self._apply_color_8bit(191)
    def x_dark_olive_green_1b(self) -> Self: return self._apply_color_8bit(192)
    def x_dark_sea_green_1b(self) -> Self: return self._apply_color_8bit(193)
    def x_honeydew_2(self) -> Self: return self._apply_color_8bit(194)
    def x_light_cyan_1(self) -> Self: return self._apply_color_8bit(195)
    def x_red_1(self) -> Self: return self._apply_color_8bit(196)
    def x_deep_pink_2(self) -> Self: return self._apply_color_8bit(197)
    def x_deep_pink_1a(self) -> Self: return self._apply_color_8bit(198)
    def x_deep_pink_1b(self) -> Self: return self._apply_color_8bit(199)
    def x_magenta_2b(self) -> Self: return self._apply_color_8bit(200)
    def x_magenta_1(self) -> Self: return self._apply_color_8bit(201)
    def x_orange_red_1(self) -> Self: return self._apply_color_8bit(202)
    def x_indian_red_1a(self) -> Self: return self._apply_color_8bit(203)
    def x_indian_red_1b(self) -> Self: return self._apply_color_8bit(204)
    def x_hot_pink_a(self) -> Self: return self._apply_color_8bit(205)
    def x_hot_pink_b(self) -> Self: return self._apply_color_8bit(206)
    def x_medium_orchid_1b(self) -> Self: return self._apply_color_8bit(207)
    def x_dark_orange(self) -> Self: return self._apply_color_8bit(208)
    def x_salmon_1(self) -> Self: return self._apply_color_8bit(209)
    def x_light_coral(self) -> Self: return self._apply_color_8bit(210)
    def x_pale_violet_red_1(self) -> Self: return self._apply_color_8bit(211)
    def x_orchid_2(self) -> Self: return self._apply_color_8bit(212)
    def x_orchid_1(self) -> Self: return self._apply_color_8bit(213)
    def x_orange_1(self) -> Self: return self._apply_color_8bit(214)
    def x_sandy_brown(self) -> Self: return self._apply_color_8bit(215)
    def x_light_salmon_1(self) -> Self: return self._apply_color_8bit(216)
    def x_light_pink_1(self) -> Self: return self._apply_color_8bit(217)
    def x_pink_1(self) -> Self: return self._apply_color_8bit(218)
    def x_plum_1(self) -> Self: return self._apply_color_8bit(219)
    def x_gold_1(self) -> Self: return self._apply_color_8bit(220)
    def x_light_goldenrod_2b(self) -> Self: return self._apply_color_8bit(221)
    def x_light_goldenrod_2c(self) -> Self: return self._apply_color_8bit(222)
    def x_navajo_white_1(self) -> Self: return self._apply_color_8bit(223)
    def x_misty_rose_1(self) -> Self: return self._apply_color_8bit(224)
    def x_thistle_1(self) -> Self: return self._apply_color_8bit(225)
    def x_yellow_1(self) -> Self: return self._apply_color_8bit(226)
    def x_light_goldenrod_1(self) -> Self: return self._apply_color_8bit(227)
    def x_khaki_1(self) -> Self: return self._apply_color_8bit(228)
    def x_wheat_1(self) -> Self: return self._apply_color_8bit(229)
    def x_cornsilk_1(self) -> Self: return self._apply_color_8bit(230)
    def x_grey_100(self) -> Self: return self._apply_color_8bit(231)
    def x_grey_3(self) -> Self: return self._apply_color_8bit(232)
    def x_grey_7(self) -> Self: return self._apply_color_8bit(233)
    def x_grey_11(self) -> Self: return self._apply_color_8bit(234)
    def x_grey_15(self) -> Self: return self._apply_color_8bit(235)
    def x_grey_19(self) -> Self: return self._apply_color_8bit(236)
    def x_grey_23(self) -> Self: return self._apply_color_8bit(237)
    def x_grey_27(self) -> Self: return self._apply_color_8bit(238)
    def x_grey_30(self) -> Self: return self._apply_color_8bit(239)
    def x_grey_35(self) -> Self: return self._apply_color_8bit(240)
    def x_grey_39(self) -> Self: return self._apply_color_8bit(241)
    def x_grey_42(self) -> Self: return self._apply_color_8bit(242)
    def x_grey_46(self) -> Self: return self._apply_color_8bit(243)
    def x_grey_50(self) -> Self: return self._apply_color_8bit(244)
    def x_grey_54(self) -> Self: return self._apply_color_8bit(245)
    def x_grey_58(self) -> Self: return self._apply_color_8bit(246)
    def x_grey_62(self) -> Self: return self._apply_color_8bit(247)
    def x_grey_66(self) -> Self: return self._apply_color_8bit(248)
    def x_grey_70(self) -> Self: return self._apply_color_8bit(249)
    def x_grey_74(self) -> Self: return self._apply_color_8bit(250)
    def x_grey_78(self) -> Self: return self._apply_color_8bit(251)
    def x_grey_82(self) -> Self: return self._apply_color_8bit(252)
    def x_grey_85(self) -> Self: return self._apply_color_8bit(253)
    def x_grey_89(self) -> Self: return self._apply_color_8bit(254)
    def x_grey_93(self) -> Self: return self._apply_color_8bit(255)

    def bg_x_black(self) -> Self: return self._apply_bg_color_8bit(0)
    def bg_x_maroon(self) -> Self: return self._apply_bg_color_8bit(1)
    def bg_x_green(self) -> Self: return self._apply_bg_color_8bit(2)
    def bg_x_olive(self) -> Self: return self._apply_bg_color_8bit(3)
    def bg_x_navy(self) -> Self: return self._apply_bg_color_8bit(4)
    def bg_x_purple(self) -> Self: return self._apply_bg_color_8bit(5)
    def bg_x_teal(self) -> Self: return self._apply_bg_color_8bit(6)
    def bg_x_silver(self) -> Self: return self._apply_bg_color_8bit(7)
    def bg_x_grey(self) -> Self: return self._apply_bg_color_8bit(8)
    def bg_x_red(self) -> Self: return self._apply_bg_color_8bit(9)
    def bg_x_lime(self) -> Self: return self._apply_bg_color_8bit(10)
    def bg_x_yellow(self) -> Self: return self._apply_bg_color_8bit(11)
    def bg_x_blue(self) -> Self: return self._apply_bg_color_8bit(12)
    def bg_x_fuchsia(self) -> Self: return self._apply_bg_color_8bit(13)
    def bg_x_aqua(self) -> Self: return self._apply_bg_color_8bit(14)
    def bg_x_white(self) -> Self: return self._apply_bg_color_8bit(15)
    def bg_x_grey_0(self) -> Self: return self._apply_bg_color_8bit(16)
    def bg_x_navy_blue(self) -> Self: return self._apply_bg_color_8bit(17)
    def bg_x_dark_blue(self) -> Self: return self._apply_bg_color_8bit(18)
    def bg_x_blue_3a(self) -> Self: return self._apply_bg_color_8bit(19)
    def bg_x_blue_3b(self) -> Self: return self._apply_bg_color_8bit(20)
    def bg_x_blue_1(self) -> Self: return self._apply_bg_color_8bit(21)
    def bg_x_dark_green(self) -> Self: return self._apply_bg_color_8bit(22)
    def bg_x_deep_sky_blue_4a(self) -> Self: return self._apply_bg_color_8bit(23)
    def bg_x_deep_sky_blue_4b(self) -> Self: return self._apply_bg_color_8bit(24)
    def bg_x_deep_sky_blue_4c(self) -> Self: return self._apply_bg_color_8bit(25)
    def bg_x_dodger_blue_3(self) -> Self: return self._apply_bg_color_8bit(26)
    def bg_x_dodger_blue_2(self) -> Self: return self._apply_bg_color_8bit(27)
    def bg_x_green_4(self) -> Self: return self._apply_bg_color_8bit(28)
    def bg_x_spring_green_4(self) -> Self: return self._apply_bg_color_8bit(29)
    def bg_x_turquoise_4(self) -> Self: return self._apply_bg_color_8bit(30)
    def bg_x_deep_sky_blue_3a(self) -> Self: return self._apply_bg_color_8bit(31)
    def bg_x_deep_sky_blue_3b(self) -> Self: return self._apply_bg_color_8bit(32)
    def bg_x_dodger_blue_1(self) -> Self: return self._apply_bg_color_8bit(33)
    def bg_x_green_3a(self) -> Self: return self._apply_bg_color_8bit(34)
    def bg_x_spring_green_3a(self) -> Self: return self._apply_bg_color_8bit(35)
    def bg_x_dark_cyan(self) -> Self: return self._apply_bg_color_8bit(36)
    def bg_x_light_sea_green(self) -> Self: return self._apply_bg_color_8bit(37)
    def bg_x_deep_sky_blue_2(self) -> Self: return self._apply_bg_color_8bit(38)
    def bg_x_deep_sky_blue_1(self) -> Self: return self._apply_bg_color_8bit(39)
    def bg_x_green_3b(self) -> Self: return self._apply_bg_color_8bit(40)
    def bg_x_spring_green_3b(self) -> Self: return self._apply_bg_color_8bit(41)
    def bg_x_spring_green_2a(self) -> Self: return self._apply_bg_color_8bit(42)
    def bg_x_cyan_3(self) -> Self: return self._apply_bg_color_8bit(43)
    def bg_x_dark_turquoise(self) -> Self: return self._apply_bg_color_8bit(44)
    def bg_x_turquoise_2(self) -> Self: return self._apply_bg_color_8bit(45)
    def bg_x_green_1(self) -> Self: return self._apply_bg_color_8bit(46)
    def bg_x_spring_green_2b(self) -> Self: return self._apply_bg_color_8bit(47)
    def bg_x_spring_green_1(self) -> Self: return self._apply_bg_color_8bit(48)
    def bg_x_medium_spring_green(self) -> Self: return self._apply_bg_color_8bit(49)
    def bg_x_cyan_2(self) -> Self: return self._apply_bg_color_8bit(50)
    def bg_x_cyan_1(self) -> Self: return self._apply_bg_color_8bit(51)
    def bg_x_dark_red_a(self) -> Self: return self._apply_bg_color_8bit(52)
    def bg_x_deep_pink_4a(self) -> Self: return self._apply_bg_color_8bit(53)
    def bg_x_purple_4a(self) -> Self: return self._apply_bg_color_8bit(54)
    def bg_x_purple_4b(self) -> Self: return self._apply_bg_color_8bit(55)
    def bg_x_purple_3(self) -> Self: return self._apply_bg_color_8bit(56)
    def bg_x_blue_violet(self) -> Self: return self._apply_bg_color_8bit(57)
    def bg_x_orange_4a(self) -> Self: return self._apply_bg_color_8bit(58)
    def bg_x_grey_37(self) -> Self: return self._apply_bg_color_8bit(59)
    def bg_x_medium_purple_4(self) -> Self: return self._apply_bg_color_8bit(60)
    def bg_x_slate_blue_3a(self) -> Self: return self._apply_bg_color_8bit(61)
    def bg_x_slate_blue_3b(self) -> Self: return self._apply_bg_color_8bit(62)
    def bg_x_royal_blue_1(self) -> Self: return self._apply_bg_color_8bit(63)
    def bg_x_chartreuse_4(self) -> Self: return self._apply_bg_color_8bit(64)
    def bg_x_dark_sea_green_4a(self) -> Self: return self._apply_bg_color_8bit(65)
    def bg_x_pale_turquoise_4(self) -> Self: return self._apply_bg_color_8bit(66)
    def bg_x_steel_blue(self) -> Self: return self._apply_bg_color_8bit(67)
    def bg_x_steel_blue_3(self) -> Self: return self._apply_bg_color_8bit(68)
    def bg_x_cornflower_blue(self) -> Self: return self._apply_bg_color_8bit(69)
    def bg_x_chartreuse_3a(self) -> Self: return self._apply_bg_color_8bit(70)
    def bg_x_dark_sea_green_4b(self) -> Self: return self._apply_bg_color_8bit(71)
    def bg_x_cadet_blue_a(self) -> Self: return self._apply_bg_color_8bit(72)
    def bg_x_cadet_blue_b(self) -> Self: return self._apply_bg_color_8bit(73)
    def bg_x_sky_blue_3(self) -> Self: return self._apply_bg_color_8bit(74)
    def bg_x_steel_blue_1a(self) -> Self: return self._apply_bg_color_8bit(75)
    def bg_x_chartreuse_3b(self) -> Self: return self._apply_bg_color_8bit(76)
    def bg_x_pale_green_3a(self) -> Self: return self._apply_bg_color_8bit(77)
    def bg_x_sea_green_3(self) -> Self: return self._apply_bg_color_8bit(78)
    def bg_x_aquamarine_3(self) -> Self: return self._apply_bg_color_8bit(79)
    def bg_x_medium_turquoise(self) -> Self: return self._apply_bg_color_8bit(80)
    def bg_x_steel_blue_1b(self) -> Self: return self._apply_bg_color_8bit(81)
    def bg_x_chartreuse_2a(self) -> Self: return self._apply_bg_color_8bit(82)
    def bg_x_sea_green_2(self) -> Self: return self._apply_bg_color_8bit(83)
    def bg_x_sea_green_1a(self) -> Self: return self._apply_bg_color_8bit(84)
    def bg_x_sea_green_1b(self) -> Self: return self._apply_bg_color_8bit(85)
    def bg_x_aquamarine_1a(self) -> Self: return self._apply_bg_color_8bit(86)
    def bg_x_dark_slate_gray_2(self) -> Self: return self._apply_bg_color_8bit(87)
    def bg_x_dark_red_b(self) -> Self: return self._apply_bg_color_8bit(88)
    def bg_x_deep_pink_4b(self) -> Self: return self._apply_bg_color_8bit(89)
    def bg_x_dark_magenta_a(self) -> Self: return self._apply_bg_color_8bit(90)
    def bg_x_dark_magenta_b(self) -> Self: return self._apply_bg_color_8bit(91)
    def bg_x_dark_violet_a(self) -> Self: return self._apply_bg_color_8bit(92)
    def bg_x_purple_a(self) -> Self: return self._apply_bg_color_8bit(93)
    def bg_x_orange_4b(self) -> Self: return self._apply_bg_color_8bit(94)
    def bg_x_light_pink_4(self) -> Self: return self._apply_bg_color_8bit(95)
    def bg_x_plum_4(self) -> Self: return self._apply_bg_color_8bit(96)
    def bg_x_medium_purple_3a(self) -> Self: return self._apply_bg_color_8bit(97)
    def bg_x_medium_purple_3b(self) -> Self: return self._apply_bg_color_8bit(98)
    def bg_x_slate_blue_1(self) -> Self: return self._apply_bg_color_8bit(99)
    def bg_x_yellow_4a(self) -> Self: return self._apply_bg_color_8bit(100)
    def bg_x_wheat_4(self) -> Self: return self._apply_bg_color_8bit(101)
    def bg_x_grey_53(self) -> Self: return self._apply_bg_color_8bit(102)
    def bg_x_light_slate_grey(self) -> Self: return self._apply_bg_color_8bit(103)
    def bg_x_medium_purple(self) -> Self: return self._apply_bg_color_8bit(104)
    def bg_x_light_slate_blue(self) -> Self: return self._apply_bg_color_8bit(105)
    def bg_x_yellow_4b(self) -> Self: return self._apply_bg_color_8bit(106)
    def bg_x_dark_olive_green_3a(self) -> Self: return self._apply_bg_color_8bit(107)
    def bg_x_dark_sea_green(self) -> Self: return self._apply_bg_color_8bit(108)
    def bg_x_light_sky_blue_3a(self) -> Self: return self._apply_bg_color_8bit(109)
    def bg_x_light_sky_blue_3b(self) -> Self: return self._apply_bg_color_8bit(110)
    def bg_x_sky_blue_2(self) -> Self: return self._apply_bg_color_8bit(111)
    def bg_x_chartreuse_2b(self) -> Self: return self._apply_bg_color_8bit(112)
    def bg_x_dark_olive_green_3b(self) -> Self: return self._apply_bg_color_8bit(113)
    def bg_x_pale_green_3b(self) -> Self: return self._apply_bg_color_8bit(114)
    def bg_x_dark_sea_green_3a(self) -> Self: return self._apply_bg_color_8bit(115)
    def bg_x_dark_slate_gray_3(self) -> Self: return self._apply_bg_color_8bit(116)
    def bg_x_sky_blue_1(self) -> Self: return self._apply_bg_color_8bit(117)
    def bg_x_chartreuse_1(self) -> Self: return self._apply_bg_color_8bit(118)
    def bg_x_light_green_a(self) -> Self: return self._apply_bg_color_8bit(119)
    def bg_x_light_green_b(self) -> Self: return self._apply_bg_color_8bit(120)
    def bg_x_pale_green_1a(self) -> Self: return self._apply_bg_color_8bit(121)
    def bg_x_aquamarine_1b(self) -> Self: return self._apply_bg_color_8bit(122)
    def bg_x_dark_slate_gray_1(self) -> Self: return self._apply_bg_color_8bit(123)
    def bg_x_red_3a(self) -> Self: return self._apply_bg_color_8bit(124)
    def bg_x_deep_pink_4c(self) -> Self: return self._apply_bg_color_8bit(125)
    def bg_x_medium_violet_red(self) -> Self: return self._apply_bg_color_8bit(126)
    def bg_x_magenta_3a(self) -> Self: return self._apply_bg_color_8bit(127)
    def bg_x_dark_violet_b(self) -> Self: return self._apply_bg_color_8bit(128)
    def bg_x_purple_b(self) -> Self: return self._apply_bg_color_8bit(129)
    def bg_x_dark_orange_3a(self) -> Self: return self._apply_bg_color_8bit(130)
    def bg_x_indian_red_a(self) -> Self: return self._apply_bg_color_8bit(131)
    def bg_x_hot_pink_3a(self) -> Self: return self._apply_bg_color_8bit(132)
    def bg_x_medium_orchid_3(self) -> Self: return self._apply_bg_color_8bit(133)
    def bg_x_medium_orchid(self) -> Self: return self._apply_bg_color_8bit(134)
    def bg_x_medium_purple_2a(self) -> Self: return self._apply_bg_color_8bit(135)
    def bg_x_dark_goldenrod(self) -> Self: return self._apply_bg_color_8bit(136)
    def bg_x_light_salmon_3a(self) -> Self: return self._apply_bg_color_8bit(137)
    def bg_x_rosy_brown(self) -> Self: return self._apply_bg_color_8bit(138)
    def bg_x_grey_63(self) -> Self: return self._apply_bg_color_8bit(139)
    def bg_x_medium_purple_2b(self) -> Self: return self._apply_bg_color_8bit(140)
    def bg_x_medium_purple_1(self) -> Self: return self._apply_bg_color_8bit(141)
    def bg_x_gold_3a(self) -> Self: return self._apply_bg_color_8bit(142)
    def bg_x_dark_khaki(self) -> Self: return self._apply_bg_color_8bit(143)
    def bg_x_navajo_white_3(self) -> Self: return self._apply_bg_color_8bit(144)
    def bg_x_grey_69(self) -> Self: return self._apply_bg_color_8bit(145)
    def bg_x_light_steel_blue_3(self) -> Self: return self._apply_bg_color_8bit(146)
    def bg_x_light_steel_blue(self) -> Self: return self._apply_bg_color_8bit(147)
    def bg_x_yellow_3a(self) -> Self: return self._apply_bg_color_8bit(148)
    def bg_x_dark_olive_green_3c(self) -> Self: return self._apply_bg_color_8bit(149)
    def bg_x_dark_sea_green_3b(self) -> Self: return self._apply_bg_color_8bit(150)
    def bg_x_dark_sea_green_2a(self) -> Self: return self._apply_bg_color_8bit(151)
    def bg_x_light_cyan_3(self) -> Self: return self._apply_bg_color_8bit(152)
    def bg_x_light_sky_blue_1(self) -> Self: return self._apply_bg_color_8bit(153)
    def bg_x_green_yellow(self) -> Self: return self._apply_bg_color_8bit(154)
    def bg_x_dark_olive_green_2(self) -> Self: return self._apply_bg_color_8bit(155)
    def bg_x_pale_green_1b(self) -> Self: return self._apply_bg_color_8bit(156)
    def bg_x_dark_sea_green_2b(self) -> Self: return self._apply_bg_color_8bit(157)
    def bg_x_dark_sea_green_1a(self) -> Self: return self._apply_bg_color_8bit(158)
    def bg_x_pale_turquoise_1(self) -> Self: return self._apply_bg_color_8bit(159)
    def bg_x_red_3b(self) -> Self: return self._apply_bg_color_8bit(160)
    def bg_x_deep_pink_3a(self) -> Self: return self._apply_bg_color_8bit(161)
    def bg_x_deep_pink_3b(self) -> Self: return self._apply_bg_color_8bit(162)
    def bg_x_magenta_3b(self) -> Self: return self._apply_bg_color_8bit(163)
    def bg_x_magenta_3c(self) -> Self: return self._apply_bg_color_8bit(164)
    def bg_x_magenta_2a(self) -> Self: return self._apply_bg_color_8bit(165)
    def bg_x_dark_orange_3b(self) -> Self: return self._apply_bg_color_8bit(166)
    def bg_x_indian_red_b(self) -> Self: return self._apply_bg_color_8bit(167)
    def bg_x_hot_pink_3b(self) -> Self: return self._apply_bg_color_8bit(168)
    def bg_x_hot_pink_2(self) -> Self: return self._apply_bg_color_8bit(169)
    def bg_x_orchid(self) -> Self: return self._apply_bg_color_8bit(170)
    def bg_x_medium_orchid_1a(self) -> Self: return self._apply_bg_color_8bit(171)
    def bg_x_orange_3(self) -> Self: return self._apply_bg_color_8bit(172)
    def bg_x_light_salmon_3b(self) -> Self: return self._apply_bg_color_8bit(173)
    def bg_x_light_pink_3(self) -> Self: return self._apply_bg_color_8bit(174)
    def bg_x_pink_3(self) -> Self: return self._apply_bg_color_8bit(175)
    def bg_x_plum_3(self) -> Self: return self._apply_bg_color_8bit(176)
    def bg_x_violet(self) -> Self: return self._apply_bg_color_8bit(177)
    def bg_x_gold_3b(self) -> Self: return self._apply_bg_color_8bit(178)
    def bg_x_light_goldenrod_3(self) -> Self: return self._apply_bg_color_8bit(179)
    def bg_x_tan(self) -> Self: return self._apply_bg_color_8bit(180)
    def bg_x_misty_rose_3(self) -> Self: return self._apply_bg_color_8bit(181)
    def bg_x_thistle_3(self) -> Self: return self._apply_bg_color_8bit(182)
    def bg_x_plum_2(self) -> Self: return self._apply_bg_color_8bit(183)
    def bg_x_yellow_3b(self) -> Self: return self._apply_bg_color_8bit(184)
    def bg_x_khaki_3(self) -> Self: return self._apply_bg_color_8bit(185)
    def bg_x_light_goldenrod_2a(self) -> Self: return self._apply_bg_color_8bit(186)
    def bg_x_light_yellow_3(self) -> Self: return self._apply_bg_color_8bit(187)
    def bg_x_grey_84(self) -> Self: return self._apply_bg_color_8bit(188)
    def bg_x_light_steel_blue_1(self) -> Self: return self._apply_bg_color_8bit(189)
    def bg_x_yellow_2(self) -> Self: return self._apply_bg_color_8bit(190)
    def bg_x_dark_olive_green_1a(self) -> Self: return self._apply_bg_color_8bit(191)
    def bg_x_dark_olive_green_1b(self) -> Self: return self._apply_bg_color_8bit(192)
    def bg_x_dark_sea_green_1b(self) -> Self: return self._apply_bg_color_8bit(193)
    def bg_x_honeydew_2(self) -> Self: return self._apply_bg_color_8bit(194)
    def bg_x_light_cyan_1(self) -> Self: return self._apply_bg_color_8bit(195)
    def bg_x_red_1(self) -> Self: return self._apply_bg_color_8bit(196)
    def bg_x_deep_pink_2(self) -> Self: return self._apply_bg_color_8bit(197)
    def bg_x_deep_pink_1a(self) -> Self: return self._apply_bg_color_8bit(198)
    def bg_x_deep_pink_1b(self) -> Self: return self._apply_bg_color_8bit(199)
    def bg_x_magenta_2b(self) -> Self: return self._apply_bg_color_8bit(200)
    def bg_x_magenta_1(self) -> Self: return self._apply_bg_color_8bit(201)
    def bg_x_orange_red_1(self) -> Self: return self._apply_bg_color_8bit(202)
    def bg_x_indian_red_1a(self) -> Self: return self._apply_bg_color_8bit(203)
    def bg_x_indian_red_1b(self) -> Self: return self._apply_bg_color_8bit(204)
    def bg_x_hot_pink_a(self) -> Self: return self._apply_bg_color_8bit(205)
    def bg_x_hot_pink_b(self) -> Self: return self._apply_bg_color_8bit(206)
    def bg_x_medium_orchid_1b(self) -> Self: return self._apply_bg_color_8bit(207)
    def bg_x_dark_orange(self) -> Self: return self._apply_bg_color_8bit(208)
    def bg_x_salmon_1(self) -> Self: return self._apply_bg_color_8bit(209)
    def bg_x_light_coral(self) -> Self: return self._apply_bg_color_8bit(210)
    def bg_x_pale_violet_red_1(self) -> Self: return self._apply_bg_color_8bit(211)
    def bg_x_orchid_2(self) -> Self: return self._apply_bg_color_8bit(212)
    def bg_x_orchid_1(self) -> Self: return self._apply_bg_color_8bit(213)
    def bg_x_orange_1(self) -> Self: return self._apply_bg_color_8bit(214)
    def bg_x_sandy_brown(self) -> Self: return self._apply_bg_color_8bit(215)
    def bg_x_light_salmon_1(self) -> Self: return self._apply_bg_color_8bit(216)
    def bg_x_light_pink_1(self) -> Self: return self._apply_bg_color_8bit(217)
    def bg_x_pink_1(self) -> Self: return self._apply_bg_color_8bit(218)
    def bg_x_plum_1(self) -> Self: return self._apply_bg_color_8bit(219)
    def bg_x_gold_1(self) -> Self: return self._apply_bg_color_8bit(220)
    def bg_x_light_goldenrod_2b(self) -> Self: return self._apply_bg_color_8bit(221)
    def bg_x_light_goldenrod_2c(self) -> Self: return self._apply_bg_color_8bit(222)
    def bg_x_navajo_white_1(self) -> Self: return self._apply_bg_color_8bit(223)
    def bg_x_misty_rose_1(self) -> Self: return self._apply_bg_color_8bit(224)
    def bg_x_thistle_1(self) -> Self: return self._apply_bg_color_8bit(225)
    def bg_x_yellow_1(self) -> Self: return self._apply_bg_color_8bit(226)
    def bg_x_light_goldenrod_1(self) -> Self: return self._apply_bg_color_8bit(227)
    def bg_x_khaki_1(self) -> Self: return self._apply_bg_color_8bit(228)
    def bg_x_wheat_1(self) -> Self: return self._apply_bg_color_8bit(229)
    def bg_x_cornsilk_1(self) -> Self: return self._apply_bg_color_8bit(230)
    def bg_x_grey_100(self) -> Self: return self._apply_bg_color_8bit(231)
    def bg_x_grey_3(self) -> Self: return self._apply_bg_color_8bit(232)
    def bg_x_grey_7(self) -> Self: return self._apply_bg_color_8bit(233)
    def bg_x_grey_11(self) -> Self: return self._apply_bg_color_8bit(234)
    def bg_x_grey_15(self) -> Self: return self._apply_bg_color_8bit(235)
    def bg_x_grey_19(self) -> Self: return self._apply_bg_color_8bit(236)
    def bg_x_grey_23(self) -> Self: return self._apply_bg_color_8bit(237)
    def bg_x_grey_27(self) -> Self: return self._apply_bg_color_8bit(238)
    def bg_x_grey_30(self) -> Self: return self._apply_bg_color_8bit(239)
    def bg_x_grey_35(self) -> Self: return self._apply_bg_color_8bit(240)
    def bg_x_grey_39(self) -> Self: return self._apply_bg_color_8bit(241)
    def bg_x_grey_42(self) -> Self: return self._apply_bg_color_8bit(242)
    def bg_x_grey_46(self) -> Self: return self._apply_bg_color_8bit(243)
    def bg_x_grey_50(self) -> Self: return self._apply_bg_color_8bit(244)
    def bg_x_grey_54(self) -> Self: return self._apply_bg_color_8bit(245)
    def bg_x_grey_58(self) -> Self: return self._apply_bg_color_8bit(246)
    def bg_x_grey_62(self) -> Self: return self._apply_bg_color_8bit(247)
    def bg_x_grey_66(self) -> Self: return self._apply_bg_color_8bit(248)
    def bg_x_grey_70(self) -> Self: return self._apply_bg_color_8bit(249)
    def bg_x_grey_74(self) -> Self: return self._apply_bg_color_8bit(250)
    def bg_x_grey_78(self) -> Self: return self._apply_bg_color_8bit(251)
    def bg_x_grey_82(self) -> Self: return self._apply_bg_color_8bit(252)
    def bg_x_grey_85(self) -> Self: return self._apply_bg_color_8bit(253)
    def bg_x_grey_89(self) -> Self: return self._apply_bg_color_8bit(254)
    def bg_x_grey_93(self) -> Self: return self._apply_bg_color_8bit(255)
    # fmt: on


if __name__ == "__main__":
    print(
        Color().bright_red().format("hello, world"),
        Color().bold().bright_green().format("hello, world"),
        Color().underline().bright_blue().format("hello, world"),
    )
    print(
        Color().italic().bright_cyan().format("hello, world"),
        Color().italic().cyan().format("hello, world"),
        Color().bold().underline().bright_magenta().format("hello, world"),
    )
    print(
        Color().bright_gray().format("hello, world"),
        Color().bright_yellow().format("hello, world"),
        Color().bold().bg_bright_gray().format("hello, world"),
    )
    print(
        Color().bold().underline().red().bg_green().format("hello, world"),
        Color().bold().rgb(0, 255, 0).bg_rgb(255, 0, 0).format("hello, world"),
        Color().x_magenta_3b().bg_x_cadet_blue_b().format("hello, world"),
    )
