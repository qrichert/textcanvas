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
    def test_no_color(self) -> None:
        self.assertEqual(Color.NO_COLOR.format("hello, world"), "hello, world")

    def test_color_custom(self) -> None:
        self.assertEqual(
            Color.CUSTOM.format("hello, world", red=45, green=227, blue=61),
            "\x1b[0;38;2;45;227;61mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color.BOLD_CUSTOM.format("hello, world", red=45, green=227, blue=61),
            "\x1b[1;38;2;45;227;61mhello, world\x1b[0m",
        )

        self.assertEqual(
            Color.BG_CUSTOM.format("hello, world", red=45, green=227, blue=61),
            "\x1b[0;48;2;45;227;61mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color.BG_BOLD_CUSTOM.format("hello, world", red=45, green=227, blue=61),
            "\x1b[1;48;2;45;227;61mhello, world\x1b[0m",
        )

    def test_color_predefined(self) -> None:
        self.assertEqual(
            Color.RED.format("hello, world"), "\x1b[0;91mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.YELLOW.format("hello, world"), "\x1b[0;93mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.GREEN.format("hello, world"), "\x1b[0;92mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BLUE.format("hello, world"), "\x1b[0;94mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.CYAN.format("hello, world"), "\x1b[0;96mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.MAGENTA.format("hello, world"), "\x1b[0;95mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.GRAY.format("hello, world"), "\x1b[0;90mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.WHITE.format("hello, world"), "\x1b[0;97mhello, world\x1b[0m"
        )

    def test_color_predefined_bold(self) -> None:
        self.assertEqual(
            Color.BOLD_RED.format("hello, world"), "\x1b[1;91mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_YELLOW.format("hello, world"), "\x1b[1;93mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_GREEN.format("hello, world"), "\x1b[1;92mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_BLUE.format("hello, world"), "\x1b[1;94mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_CYAN.format("hello, world"), "\x1b[1;96mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_MAGENTA.format("hello, world"), "\x1b[1;95mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_GRAY.format("hello, world"), "\x1b[1;90mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BOLD_WHITE.format("hello, world"), "\x1b[1;97mhello, world\x1b[0m"
        )

    def test_color_predefined_background(self) -> None:
        self.assertEqual(
            Color.BG_RED.format("hello, world"), "\x1b[0;101mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_YELLOW.format("hello, world"), "\x1b[0;103mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_GREEN.format("hello, world"), "\x1b[0;102mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BLUE.format("hello, world"), "\x1b[0;104mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_CYAN.format("hello, world"), "\x1b[0;106mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_MAGENTA.format("hello, world"), "\x1b[0;105mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_GRAY.format("hello, world"), "\x1b[0;100mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_WHITE.format("hello, world"), "\x1b[0;107mhello, world\x1b[0m"
        )

    def test_color_predefined_background_bold(self) -> None:
        self.assertEqual(
            Color.BG_BOLD_RED.format("hello, world"), "\x1b[1;101mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BOLD_YELLOW.format("hello, world"),
            "\x1b[1;103mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color.BG_BOLD_GREEN.format("hello, world"), "\x1b[1;102mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BOLD_BLUE.format("hello, world"), "\x1b[1;104mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BOLD_CYAN.format("hello, world"), "\x1b[1;106mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BOLD_MAGENTA.format("hello, world"),
            "\x1b[1;105mhello, world\x1b[0m",
        )
        self.assertEqual(
            Color.BG_BOLD_GRAY.format("hello, world"), "\x1b[1;100mhello, world\x1b[0m"
        )
        self.assertEqual(
            Color.BG_BOLD_WHITE.format("hello, world"), "\x1b[1;107mhello, world\x1b[0m"
        )
