from enum import StrEnum
from typing import Literal


class Color(StrEnum):
    """Color.

    TODO:

    Examples:
        >>> Color.RED.format("hello, world")
        '\\x1b[0;91mhello, world\\x1b[0m'
        >>> Color.BG_CUSTOM.format("hello, world", red=45, green=227, blue=61)
        '\\x1b[0;48;2;45;227;61mhello, world\\x1b[0m'

    - ESC = 0o33 = 0x1b = Escape character, start of escape sequence.
    - ESC[ = Control sequence.
    - ESC[0m = Reset sequence.
    - ESC[0;⟨n⟩m] = ⟨n⟩ is one of 16 color codes (4-bit)
    - ESC[38;2;⟨r⟩;⟨g⟩;⟨b⟩m = [0;255] Select RGB foreground color
    - ESC[48;2;⟨r⟩;⟨g⟩;⟨b⟩m = [0;255] Select RGB background color

    24-bit custom color do not work on every terminal.

    https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
    https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit
    """

    NO_COLOR = "{}"

    # 24-bit
    CUSTOM = "\033[0;38;2;{red:d};{green:d};{blue:d}m{}\033[0m"
    BOLD_CUSTOM = "\033[1;38;2;{red:d};{green:d};{blue:d}m{}\033[0m"

    BG_CUSTOM = "\033[0;48;2;{red:d};{green:d};{blue:d}m{}\033[0m"
    BG_BOLD_CUSTOM = "\033[1;48;2;{red:d};{green:d};{blue:d}m{}\033[0m"

    # 4-bit colors.
    # 0; is optional () 1; bold.
    RED = "\033[0;91m{}\033[0m"
    YELLOW = "\033[0;93m{}\033[0m"
    GREEN = "\033[0;92m{}\033[0m"
    BLUE = "\033[0;94m{}\033[0m"
    CYAN = "\033[0;96m{}\033[0m"
    MAGENTA = "\033[0;95m{}\033[0m"
    GRAY = "\033[0;90m{}\033[0m"
    WHITE = "\033[0;97m{}\033[0m"

    BOLD_RED = "\033[1;91m{}\033[0m"
    BOLD_YELLOW = "\033[1;93m{}\033[0m"
    BOLD_GREEN = "\033[1;92m{}\033[0m"
    BOLD_BLUE = "\033[1;94m{}\033[0m"
    BOLD_CYAN = "\033[1;96m{}\033[0m"
    BOLD_MAGENTA = "\033[1;95m{}\033[0m"
    BOLD_GRAY = "\033[1;90m{}\033[0m"
    BOLD_WHITE = "\033[1;97m{}\033[0m"

    BG_RED = "\033[0;101m{}\033[0m"
    BG_YELLOW = "\033[0;103m{}\033[0m"
    BG_GREEN = "\033[0;102m{}\033[0m"
    BG_BLUE = "\033[0;104m{}\033[0m"
    BG_CYAN = "\033[0;106m{}\033[0m"
    BG_MAGENTA = "\033[0;105m{}\033[0m"
    BG_GRAY = "\033[0;100m{}\033[0m"
    BG_WHITE = "\033[0;107m{}\033[0m"

    BG_BOLD_RED = "\033[1;101m{}\033[0m"
    BG_BOLD_YELLOW = "\033[1;103m{}\033[0m"
    BG_BOLD_GREEN = "\033[1;102m{}\033[0m"
    BG_BOLD_BLUE = "\033[1;104m{}\033[0m"
    BG_BOLD_CYAN = "\033[1;106m{}\033[0m"
    BG_BOLD_MAGENTA = "\033[1;105m{}\033[0m"
    BG_BOLD_GRAY = "\033[1;100m{}\033[0m"
    BG_BOLD_WHITE = "\033[1;107m{}\033[0m"


type CustomColor = Literal[
    Color.CUSTOM,
    Color.BOLD_CUSTOM,
    Color.BG_CUSTOM,
    Color.BG_BOLD_CUSTOM,
]


def custom_color_from_rgb(
    red: int, green: int, blue: int, base: CustomColor = Color.CUSTOM
) -> str:
    # Replace '{}' by '{}' to keep it as is.
    return base.format("{}", red=red, green=green, blue=blue)


def custom_color_from_hex(hex_color: str, base: CustomColor = Color.CUSTOM) -> str:
    hex_color = hex_color.lstrip("#")
    return custom_color_from_rgb(
        red=int(hex_color[:2], 16),
        green=int(hex_color[2:4], 16),
        blue=int(hex_color[4:], 16),
        base=base,
    )


if __name__ == "__main__":
    print(Color.CUSTOM.format("hello, world", red=45, green=227, blue=61))
    print(Color.BOLD_CUSTOM.format("hello, world", red=45, green=227, blue=61))
    print(Color.BG_CUSTOM.format("hello, world", red=45, green=227, blue=61))
    print(Color.BG_BOLD_CUSTOM.format("hello, world", red=45, green=227, blue=61))

    print(Color.RED.format("hello, world"))
    print(Color.YELLOW.format("hello, world"))
    print(Color.GREEN.format("hello, world"))
    print(Color.BLUE.format("hello, world"))
    print(Color.CYAN.format("hello, world"))
    print(Color.MAGENTA.format("hello, world"))
    print(Color.GRAY.format("hello, world"))
    print(Color.WHITE.format("hello, world"))

    print(Color.BOLD_RED.format("hello, world"))
    print(Color.BOLD_YELLOW.format("hello, world"))
    print(Color.BOLD_GREEN.format("hello, world"))
    print(Color.BOLD_BLUE.format("hello, world"))
    print(Color.BOLD_CYAN.format("hello, world"))
    print(Color.BOLD_MAGENTA.format("hello, world"))
    print(Color.BOLD_GRAY.format("hello, world"))
    print(Color.BOLD_WHITE.format("hello, world"))

    print(Color.BG_RED.format("hello, world"))
    print(Color.BG_YELLOW.format("hello, world"))
    print(Color.BG_GREEN.format("hello, world"))
    print(Color.BG_BLUE.format("hello, world"))
    print(Color.BG_CYAN.format("hello, world"))
    print(Color.BG_MAGENTA.format("hello, world"))
    print(Color.BG_GRAY.format("hello, world"))
    print(Color.BG_WHITE.format("hello, world"))

    print(Color.BG_BOLD_RED.format("hello, world"))
    print(Color.BG_BOLD_YELLOW.format("hello, world"))
    print(Color.BG_BOLD_GREEN.format("hello, world"))
    print(Color.BG_BOLD_BLUE.format("hello, world"))
    print(Color.BG_BOLD_CYAN.format("hello, world"))
    print(Color.BG_BOLD_MAGENTA.format("hello, world"))
    print(Color.BG_BOLD_GRAY.format("hello, world"))
    print(Color.BG_BOLD_WHITE.format("hello, world"))
