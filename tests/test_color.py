import doctest
import unittest

import textcanvas.color
from textcanvas.color import Color


def load_tests(
    loader: unittest.TestLoader, tests: unittest.TestSuite, ignore: str
) -> unittest.TestSuite:
    """Add module doctests."""
    tests.addTests(doctest.DocTestSuite(textcanvas.color))
    return tests


class TestColor(unittest.TestCase):
    # General.

    def test_format(self) -> None:
        string = Color().magenta().format("foo")

        self.assertEqual(string, "\x1b[0;35mfoo\x1b[0m")

    def test_to_string(self) -> None:
        string = Color().magenta().to_string()

        self.assertEqual(string, "\x1b[0;35m{}\x1b[0m")

    # Special Cases.

    def test_no_color(self) -> None:
        self.assertEqual(
            Color().format("hello, world"),
            "hello, world",
        )

    # Display Attributes

    def test_no_color_bold(self) -> None:
        self.assertEqual(
            Color().bold().format("hello, world"),
            "\x1b[1mhello, world\x1b[0m",
        )

    def test_no_color_italic(self) -> None:
        self.assertEqual(
            Color().italic().format("hello, world"),
            "\x1b[3mhello, world\x1b[0m",
        )

    def test_no_color_underline(self) -> None:
        self.assertEqual(
            Color().underline().format("hello, world"),
            "\x1b[4mhello, world\x1b[0m",
        )

    def test_no_color_bold_italic_underline(self) -> None:
        self.assertEqual(
            Color().bold().italic().underline().format("hello, world"),
            "\x1b[1;3;4mhello, world\x1b[0m",
        )

    def test_color_bold(self) -> None:
        self.assertEqual(
            Color().bold().bg_bright_green().format("hello, world"),
            "\x1b[1;102mhello, world\x1b[0m",
        )

    def test_color_bold_after_color(self) -> None:
        # Order should not matter.
        self.assertEqual(
            Color().bg_bright_green().bold().format("hello, world"),
            "\x1b[1;102mhello, world\x1b[0m",
        )

    # RGB (24-bit)

    def test_color_rgb(self) -> None:
        self.assertEqual(
            Color().rgb(45, 227, 61).format("hello, world"),
            "\x1b[0;38;2;45;227;61mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color().bold().rgb(45, 227, 61).format("hello, world"),
            "\x1b[1;38;2;45;227;61mhello, world\x1b[0m",
        )

    def test_color_bg_rgb(self) -> None:
        self.assertEqual(
            Color().bg_rgb(45, 227, 61).format("hello, world"),
            "\x1b[0;48;2;45;227;61mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color().bold().bg_rgb(45, 227, 61).format("hello, world"),
            "\x1b[1;48;2;45;227;61mhello, world\x1b[0m",
        )

    def test_color_rgb_with_bg(self) -> None:
        self.assertEqual(
            Color().rgb(45, 227, 61).bg_rgb(13, 10, 40).format("hello, world"),
            "\x1b[0;38;2;45;227;61m\x1b[48;2;13;10;40mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color().bold().rgb(45, 227, 61).bg_rgb(13, 10, 40).format("hello, world"),
            "\x1b[1;38;2;45;227;61m\x1b[48;2;13;10;40mhello, world\x1b[0m",
        )

    def test_color_rgb_from_hex(self) -> None:
        self.assertEqual(
            Color().rbg_from_hex("#1f2c3b").format("hello, world"),
            "\x1b[0;38;2;31;44;59mhello, world\x1b[0m",
        )

    def test_color_rgb_from_hex_without_hash(self) -> None:
        self.assertEqual(
            Color().rbg_from_hex("1f2c3b").format("hello, world"),
            "\x1b[0;38;2;31;44;59mhello, world\x1b[0m",
        )

    def test_color_rgb_from_hex_invalid(self) -> None:
        self.assertEqual(
            Color()
            .rbg_from_hex("#012345678901234567890123456789")
            .format("hello, world"),
            "\x1b[0;38;2;0;0;0mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color().rbg_from_hex("#zzzzzz").format("hello, world"),
            "\x1b[0;38;2;0;0;0mhello, world\x1b[0m",
        )

    def test_color_rgb_from_hex_only_one_invalid(self) -> None:
        self.assertEqual(
            Color().rbg_from_hex("#ffggff").format("hello, world"),
            "\x1b[0;38;2;255;0;255mhello, world\x1b[0m",
        )

    def test_color_bg_rgb_from_hex(self) -> None:
        self.assertEqual(
            Color().bg_rbg_from_hex("#1f2c3b").format("hello, world"),
            "\x1b[0;48;2;31;44;59mhello, world\x1b[0m",
        )

    # 4-bit.

    def test_color_4bit(self) -> None:
        self.assertEqual(
            Color().green().format("hello, world"),
            "\x1b[0;32mhello, world\x1b[0m",
        )

    def test_color_4bit_bright(self) -> None:
        self.assertEqual(
            Color().bright_green().format("hello, world"),
            "\x1b[0;92mhello, world\x1b[0m",
        )

    def test_color_4bit_bg(self) -> None:
        self.assertEqual(
            Color().bg_green().format("hello, world"),
            "\x1b[0;42mhello, world\x1b[0m",
        )

    def test_color_4bit_bg_bright(self) -> None:
        self.assertEqual(
            Color().bg_bright_green().format("hello, world"),
            "\x1b[0;102mhello, world\x1b[0m",
        )

    def test_color_4bit_with_bg(self) -> None:
        self.assertEqual(
            Color().red().bg_green().format("hello, world"),
            "\x1b[0;31;42mhello, world\x1b[0m",
        )

    # 8-bit.

    def test_color_8bit(self) -> None:
        self.assertEqual(
            Color().x_cadet_blue_a().format("hello, world"),
            "\x1b[0;38;5;72mhello, world\x1b[0m",
        )

    def test_color_8bit_bg(self) -> None:
        self.assertEqual(
            Color().bg_x_salmon_1().format("hello, world"),
            "\x1b[0;48;5;209mhello, world\x1b[0m",
        )

    def test_color_8bit_with_bg(self) -> None:
        self.assertEqual(
            Color().x_dark_sea_green().bg_x_spring_green_2a().format("hello, world"),
            "\x1b[0;38;5;108m\x1b[48;5;42mhello, world\x1b[0m",
        )


if __name__ == "__main__":
    unittest.main()
