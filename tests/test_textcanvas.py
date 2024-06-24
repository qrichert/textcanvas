import doctest
import math
import os
import unittest

import textcanvas.textcanvas
from textcanvas.color import Color
from textcanvas.textcanvas import Surface, TextCanvas


def load_tests(
    loader: unittest.TestLoader, tests: unittest.TestSuite, ignore: str
) -> unittest.TestSuite:
    """Add module doctests."""
    tests.addTests(doctest.DocTestSuite(textcanvas.textcanvas))
    return tests


def stroke_line_accros_canvas(canvas: TextCanvas) -> None:
    y = 0
    for x in range(canvas.screen.width):
        canvas.set_pixel(x, y, True)
        y += 1


class TestSurface(unittest.TestCase):
    def test_size(self):
        surface = Surface(width=15, height=9)

        self.assertEqual(surface.width, 15)
        self.assertEqual(surface.height, 9)


class TestTextCanvas(unittest.TestCase):
    def test_output_size(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(canvas.output.width, 7, "Incorrect output width.")
        self.assertEqual(canvas.output.height, 4, "Incorrect output height.")

    def test_screen_size(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(canvas.screen.width, 7 * 2, "Incorrect output width.")
        self.assertEqual(canvas.screen.height, 4 * 4, "Incorrect output height.")

    def test_buffer_size(self) -> None:
        canvas = TextCanvas(7, 4)
        buffer_width = len(canvas.buffer[0])
        buffer_height = len(canvas.buffer)

        self.assertEqual(buffer_width, 7 * 2, "Incorrect number of rows in buffer.")
        self.assertEqual(buffer_height, 4 * 4, "Incorrect number of columns in buffer.")

    def test_default_size(self) -> None:
        canvas = TextCanvas()

        self.assertEqual(canvas.output.width, 80, "Incorrect default width.")
        self.assertEqual(canvas.output.height, 24, "Incorrect default height.")

    def test_get_default_size(self) -> None:
        (width, height) = TextCanvas.get_default_size()

        self.assertEqual(width, 80, "Incorrect default width.")
        self.assertEqual(height, 24, "Incorrect default width.")

    def test_constructor_default_size_eq_get_default_size(self) -> None:
        canvas = TextCanvas()
        (width, height) = TextCanvas.get_default_size()

        self.assertEqual(canvas.output.width, width, "Incorrect default width.")
        self.assertEqual(canvas.output.height, height, "Incorrect default height.")

    def test_size_zero_raises_error(self) -> None:
        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Zero width did not raise error."
            TextCanvas(0, 1)

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Zero height did not raise error."
            TextCanvas(1, 0)

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Zero width and height did not raise error."
            TextCanvas(0, 0)

    def test_size_negative_raises_error(self) -> None:
        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Negative width did not raise error."
            TextCanvas(-1, 1)

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Negative height did not raise error."
            TextCanvas(1, -1)

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Negative width and height did not raise error."
            TextCanvas(-1, -1)

    def test_auto_size(self) -> None:
        os.environ["WIDTH"] = "12"
        os.environ["HEIGHT"] = "5"

        canvas = TextCanvas.auto()

        self.assertEqual(canvas.output.width, 12, "Incorrect auto width.")
        self.assertEqual(canvas.output.height, 5, "Incorrect auto height.")

        self.assertEqual(TextCanvas.get_auto_size(), (12, 5))

    def test_auto_size_width_and_height_variables_dont_exist(self) -> None:
        os.environ.pop("WIDTH", None)
        os.environ.pop("HEIGHT", None)

        with self.assertRaises(LookupError) as ctx:
            ctx.msg = "`WIDTH` and `HEIGHT` don't exist."
            TextCanvas.auto()

        with self.assertRaises(LookupError):
            TextCanvas.get_auto_size()

    def test_auto_size_cannot_parse_width_variable(self) -> None:
        os.environ["WIDTH"] = "abc"
        os.environ["HEIGHT"] = "1"

        with self.assertRaises(LookupError) as ctx:
            ctx.msg = "`WIDTH` is not a number."
            TextCanvas.auto()

        with self.assertRaises(LookupError):
            TextCanvas.get_auto_size()

    def test_auto_size_cannot_parse_height_variable(self) -> None:
        os.environ["WIDTH"] = "1"
        os.environ["HEIGHT"] = "abc"

        with self.assertRaises(LookupError) as ctx:
            ctx.msg = "`HEIGHT` is not a number."
            TextCanvas.auto()

        with self.assertRaises(LookupError):
            TextCanvas.get_auto_size()

    def test_string_representation(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(
            canvas.to_string(), str(canvas), "Incorrect string representation."
        )

        self.assertEqual(
            repr(canvas),
            "Canvas(output=(7×4), screen=(14×16)))",
            "Incorrect string representation.",
        )

    def test_shortcuts(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(canvas.w, 13, "Incorrect screen width.")
        self.assertEqual(canvas.h, 15, "Incorrect screen height.")
        self.assertEqual(canvas.cx, 7, "Incorrect screen center-X.")
        self.assertEqual(canvas.cy, 8, "Incorrect screen center-Y.")

    def test_check_output_bounds(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertTrue(canvas._check_output_bounds(0, 0))
        self.assertTrue(canvas._check_output_bounds(6, 0))
        self.assertTrue(canvas._check_output_bounds(6, 3))
        self.assertTrue(canvas._check_output_bounds(0, 3))

        self.assertFalse(canvas._check_output_bounds(0, -1))
        self.assertFalse(canvas._check_output_bounds(7, 0))
        self.assertFalse(canvas._check_output_bounds(6, 4))
        self.assertFalse(canvas._check_output_bounds(-1, 3))

    def test_check_screen_bounds(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertTrue(canvas._check_screen_bounds(0, 0))
        self.assertTrue(canvas._check_screen_bounds(13, 0))
        self.assertTrue(canvas._check_screen_bounds(13, 15))
        self.assertTrue(canvas._check_screen_bounds(0, 15))

        self.assertFalse(canvas._check_screen_bounds(0, -1))
        self.assertFalse(canvas._check_screen_bounds(14, 0))
        self.assertFalse(canvas._check_screen_bounds(13, 16))
        self.assertFalse(canvas._check_screen_bounds(-1, 15))

    def test_turn_all_pixels_on(self) -> None:
        canvas = TextCanvas(2, 2)

        for x in range(canvas.screen.width):
            for y in range(canvas.screen.height):
                canvas.set_pixel(x, y, True)

        self.assertEqual(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not fully on.")

    def test_get_pixel(self) -> None:
        canvas = TextCanvas(2, 2)

        self.assertEqual(canvas.get_pixel(3, 2), False, "Pixel should be turned off.")

        canvas.set_pixel(3, 2, True)

        self.assertEqual(canvas.get_pixel(3, 2), True, "Pixel should be turned on.")

    def test_get_pixel_with_overflow(self) -> None:
        canvas = TextCanvas(1, 1)

        self.assertIsNone(canvas.get_pixel(-1, 0), "Overflow should be None.")
        self.assertIsNone(canvas.get_pixel(0, -1), "Overflow should be None.")
        self.assertIsNone(canvas.get_pixel(-1, -1), "Overflow should be None.")

        self.assertIsNone(
            canvas.get_pixel(canvas.screen.width, 0), "Overflow should be None."
        )
        self.assertIsNone(
            canvas.get_pixel(0, canvas.screen.height), "Overflow should be None."
        )
        self.assertIsNone(
            canvas.get_pixel(canvas.screen.width, canvas.screen.height),
            "Overflow should be None.",
        )

    def test_get_pixel_on_boundaries(self) -> None:
        canvas = TextCanvas(1, 1)

        canvas.buffer = [
            [True, False],
            [False, False],
            [False, False],
            [False, True],
        ]

        self.assertEqual(canvas.get_pixel(0, 0), True, "Incorrect pixel value.")
        self.assertEqual(
            canvas.get_pixel(canvas.screen.width - 1, canvas.screen.height - 1),
            True,
            "Incorrect pixel value.",
        )

    def test_set_pixel(self) -> None:
        canvas = TextCanvas(3, 2)
        stroke_line_accros_canvas(canvas)

        self.assertEqual(
            canvas.buffer,
            [
                [True, False, False, False, False, False],
                [False, True, False, False, False, False],
                [False, False, True, False, False, False],
                [False, False, False, True, False, False],
                [False, False, False, False, True, False],
                [False, False, False, False, False, True],
                [False, False, False, False, False, False],
                [False, False, False, False, False, False],
            ],
            "Incorrect buffer content.",
        )

    def test_set_pixel_with_overflow(self) -> None:
        canvas = TextCanvas(1, 1)

        canvas.set_pixel(-1, 0, True)
        canvas.set_pixel(0, -1, True)
        canvas.set_pixel(-1, -1, True)

        canvas.set_pixel(canvas.screen.width, 0, True)
        canvas.set_pixel(0, canvas.screen.height, True)
        canvas.set_pixel(canvas.screen.width, canvas.screen.height, True)

        self.assertEqual(
            canvas.buffer,
            [
                [False, False],
                [False, False],
                [False, False],
                [False, False],
            ],
            "No pixel should be turned on.",
        )

    def test_set_pixel_on_boundaries(self) -> None:
        canvas = TextCanvas(1, 1)

        canvas.set_pixel(0, 0, True)
        canvas.set_pixel(canvas.screen.width - 1, canvas.screen.height - 1, True)

        self.assertEqual(
            canvas.buffer,
            [
                [True, False],
                [False, False],
                [False, False],
                [False, True],
            ],
            "Incorrect buffer content.",
        )

    def test_get_as_string(self) -> None:
        canvas = TextCanvas(3, 2)
        stroke_line_accros_canvas(canvas)

        self.assertEqual(canvas.to_string(), "⠑⢄⠀\n⠀⠀⠑\n", "Incorrect output string.")

    def test_clear(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.fill()

        canvas.clear()

        self.assertEqual(canvas.to_string(), "⠀⠀\n⠀⠀\n", "Output not empty.")

    def test_clear_edits_buffer_in_place(self) -> None:
        canvas = TextCanvas(1, 1)

        buffer = canvas.buffer
        row_0 = canvas.buffer[0]
        row_1 = canvas.buffer[1]
        row_2 = canvas.buffer[2]
        row_3 = canvas.buffer[3]

        canvas.clear()

        self.assertIs(buffer, canvas.buffer, "Container should be the same as before.")
        self.assertIs(
            row_0, canvas.buffer[0], "Container should be the same as before."
        )
        self.assertIs(
            row_1, canvas.buffer[1], "Container should be the same as before."
        )
        self.assertIs(
            row_2, canvas.buffer[2], "Container should be the same as before."
        )
        self.assertIs(
            row_3, canvas.buffer[3], "Container should be the same as before."
        )

    def test_fill(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.fill()

        self.assertEqual(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not full.")

    def test_invert(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.fill_rect(6, 3, 20, 15)

        self.assertFalse(canvas.is_inverted)

        canvas.invert()
        canvas.fill_rect(9, 6, 14, 9)

        self.assertTrue(canvas.is_inverted)
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀\n"
            "⠀⠀⠀⣿⡟⠛⠛⠛⠛⠛⠛⢻⣿⠀⠀\n"
            "⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀\n"
            "⠀⠀⠀⣿⣇⣀⣀⣀⣀⣀⣀⣸⣿⠀⠀\n"
            "⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀\n",
        )

    def test_double_invert(self) -> None:
        canvas = TextCanvas(15, 5)

        self.assertFalse(canvas.is_inverted)

        canvas.invert()
        self.assertTrue(canvas.is_inverted)

        canvas.invert()
        self.assertFalse(canvas.is_inverted)

    def test_clear_not_affected_by_invert(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.invert()
        canvas.clear()

        self.assertEqual(canvas.to_string(), "⠀⠀\n⠀⠀\n", "Output not empty.")

    def test_fill_not_affected_by_invert(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.invert()
        canvas.fill()

        self.assertEqual(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not full.")

    def test_iter_buffer_by_blocks_lrtb(self) -> None:
        # This tests a private method, but this method is at the core
        # of the output generation. Testing it helps ensure stability.
        canvas = TextCanvas(3, 2)
        stroke_line_accros_canvas(canvas)

        self.assertEqual(
            list(canvas._iter_buffer_by_blocks_lrtb()),
            [
                (
                    (True, False),
                    (False, True),
                    (False, False),
                    (False, False),
                ),
                (
                    (False, False),
                    (False, False),
                    (True, False),
                    (False, True),
                ),
                (
                    (False, False),
                    (False, False),
                    (False, False),
                    (False, False),
                ),
                (
                    (False, False),
                    (False, False),
                    (False, False),
                    (False, False),
                ),
                (
                    (False, False),
                    (False, False),
                    (False, False),
                    (False, False),
                ),
                (
                    (True, False),
                    (False, True),
                    (False, False),
                    (False, False),
                ),
            ],
            "Incorrect list of blocks.",
        )

    def test_iter_buffer(self) -> None:
        canvas = TextCanvas(3, 2)

        # fmt: off
        self.assertEqual(list(canvas.iter_buffer()), [
            (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
            (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
            (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
            (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
            (0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5),
            (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6),
            (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
        ], "Incorrect X and Y pairs, or in wrong order.")
        # fmt: on


class TestTextCanvasColor(unittest.TestCase):
    def test_color_buffer_size_at_init(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(len(canvas.color_buffer), 0, "Color buffer should be empty.")

    def test_color_buffer_size_with_color(self) -> None:
        canvas = TextCanvas(7, 4)

        canvas.set_color(Color().bg_bright_blue())

        buffer_width = len(canvas.color_buffer[0])
        buffer_height = len(canvas.color_buffer)

        self.assertEqual(
            buffer_width, 7, "Color buffer width should match output buffer width."
        )
        self.assertEqual(
            buffer_height, 4, "Color buffer height should match output buffer height."
        )

    def test_is_colorized(self) -> None:
        canvas = TextCanvas(2, 2)

        self.assertFalse(
            canvas.is_colorized, "Canvas should not be colorized by default."
        )

        canvas.set_color(Color().bg_bright_blue())

        self.assertTrue(
            canvas.is_colorized, "Canvas should be colorized after a color is set."
        )

    def test_set_color(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color().bg_bright_blue())
        canvas.set_pixel(3, 3, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color(), Color().bg_bright_blue()],
                [Color(), Color()],
            ],
            "Incorrect color buffer.",
        )

    def test_set_color_multiple(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color().bg_bright_blue())
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color(), Color().bg_bright_blue()],
                [Color().bg_bright_blue(), Color()],
            ],
            "Incorrect color buffer.",
        )

    def test_set_color_override(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color().bg_bright_blue())
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        canvas.set_color(Color().bg_bright_red())
        canvas.set_pixel(3, 3, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color(), Color().bg_bright_red()],
                [Color().bg_bright_blue(), Color()],
            ],
            "Incorrect color buffer.",
        )

    def test_color_is_reset_if_pixel_turned_off(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color().bg_bright_blue())
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        canvas.set_pixel(3, 3, False)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color(), Color()],
                [Color().bg_bright_blue(), Color()],
            ],
            "Incorrect color buffer.",
        )

    def test_get_as_string_colored(self) -> None:
        canvas = TextCanvas(3, 2)
        canvas.set_color(Color().bright_green())
        stroke_line_accros_canvas(canvas)

        self.assertEqual(
            canvas.to_string(),
            "\x1b[0;92m⠑\x1b[0m\x1b[0;92m⢄\x1b[0m⠀\n⠀⠀\x1b[0;92m⠑\x1b[0m\n",
            "Incorrect output string.",
        )

    def test_clear_clears_color_buffer(self) -> None:
        canvas = TextCanvas(2, 1)

        self.assertEqual(canvas.color_buffer, [], "Color buffer should be empty.")

        canvas.set_color(Color().bright_red())

        self.assertEqual(
            canvas.color_buffer,
            [[Color(), Color()]],
            "Color buffer should be full of no-color.",
        )

        canvas.set_pixel(0, 0, True)

        self.assertEqual(
            canvas.color_buffer,
            [[Color().bright_red(), Color()]],
            "First pixel should be red.",
        )

        canvas.clear()

        self.assertEqual(
            canvas.color_buffer,
            [[Color(), Color()]],
            "Color buffer should be full of no-color.",
        )

    def test_clear_edits_color_buffer_in_place(self) -> None:
        canvas = TextCanvas(2, 2)
        canvas.set_color(Color().bright_red())

        color_buffer = canvas.color_buffer
        row_0 = canvas.color_buffer[0]
        row_1 = canvas.color_buffer[1]

        canvas.clear()

        self.assertIs(
            color_buffer, canvas.color_buffer, "Container should be the same as before."
        )
        self.assertIs(
            row_0, canvas.color_buffer[0], "Container should be the same as before."
        )
        self.assertIs(
            row_1, canvas.color_buffer[1], "Container should be the same as before."
        )


class TestTextCanvasText(unittest.TestCase):
    def test_text_buffer_size_at_init(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(len(canvas.text_buffer), 0, "Text buffer should be empty.")

    def test_text_buffer_size_with_color(self) -> None:
        canvas = TextCanvas(7, 4)

        canvas.draw_text("foo", 0, 0)

        buffer_width = len(canvas.text_buffer[0])
        buffer_height = len(canvas.text_buffer)

        self.assertEqual(
            buffer_width, 7, "Text buffer width should match output buffer width."
        )
        self.assertEqual(
            buffer_height, 4, "Text buffer height should match output buffer height."
        )

    def test_is_textual(self) -> None:
        canvas = TextCanvas(2, 2)

        self.assertFalse(
            canvas.is_colorized, "Canvas should not be textual by default."
        )

        canvas.draw_text("hi", 0, 0)

        self.assertTrue(
            canvas.is_textual, "Canvas should be textual after text is drawn."
        )

    def test_draw_text(self) -> None:
        canvas = TextCanvas(5, 1)

        canvas.draw_text("bar", 1, 0)

        self.assertEqual(
            canvas.text_buffer, [["", "b", "a", "r", ""]], "Incorrect text buffer."
        )

    def test_draw_text_vertical(self) -> None:
        canvas = TextCanvas(1, 5)

        self.assertFalse(canvas.is_textual)

        canvas.draw_text_vertical("bar", 0, 1)

        self.assertTrue(canvas.is_textual)

        self.assertEqual(
            canvas.text_buffer,
            [
                [""],
                ["b"],
                ["a"],
                ["r"],
                [""],
            ],
            "Incorrect text buffer.",
        )

    def test_draw_text_over_text(self) -> None:
        canvas = TextCanvas(5, 1)

        canvas.draw_text("bar", 1, 0)
        canvas.draw_text("foo", 2, 0)

        self.assertEqual(
            canvas.text_buffer, [["", "b", "f", "o", "o"]], "Incorrect text buffer."
        )

    def test_draw_text_space_is_transparent(self) -> None:
        canvas = TextCanvas(9, 1)

        canvas.draw_text("foo bar", 1, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["", "f", "o", "o", "", "b", "a", "r", ""]],
            "Incorrect text buffer.",
        )

    def test_draw_text_space_clears_text(self) -> None:
        canvas = TextCanvas(5, 1)

        canvas.draw_text("bar", 1, 0)
        canvas.draw_text("  ", 2, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["", "b", "", "", ""]],
            "Incorrect text buffer.",
        )

    def test_draw_text_with_overflow(self) -> None:
        canvas = TextCanvas(5, 2)

        # Show partially.
        canvas.draw_text("foo", -1, 0)
        canvas.draw_text("bar", 3, 1)

        # Completely out of bounds.
        canvas.draw_text("baz1", -10, -1)
        canvas.draw_text("baz2", 10, -1)
        canvas.draw_text("baz3", -10, 2)
        canvas.draw_text("baz4", 10, 2)

        self.assertEqual(
            canvas.text_buffer,
            [
                ["o", "o", "", "", ""],
                ["", "", "", "b", "a"],
            ],
            "Incorrect text buffer.",
        )

    def test_draw_text_on_boundaries(self) -> None:
        canvas = TextCanvas(3, 3)

        canvas.draw_text("a", 0, 1)
        canvas.draw_text("b", 1, 0)
        canvas.draw_text("c", 2, 1)
        canvas.draw_text("d", 1, 2)

        self.assertEqual(
            canvas.to_string(),
            "⠀b⠀\na⠀c\n⠀d⠀\n",
            "Incorrect text output.",
        )

    def test_draw_text_with_color(self) -> None:
        canvas = TextCanvas(3, 1)

        self.assertEqual(canvas.text_buffer, [], "Text buffer should be empty.")

        canvas.draw_text("hi!", 0, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["h", "i", "!"]],
            "Text should not be colorized.",
        )

        canvas.set_color(Color().bright_red())
        canvas.draw_text("o!", 1, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["h", "\x1b[0;91mo\x1b[0m", "\x1b[0;91m!\x1b[0m"]],
            "'o!' should be red.",
        )

    def test_merge_text_space_does_not_clear_text(self) -> None:
        canvas = TextCanvas(5, 1)

        canvas.merge_text("bar", 1, 0)
        canvas.merge_text(" z", 2, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["", "b", "a", "z", ""]],
            "Incorrect text buffer.",
        )

    def test_merge_text_vertical(self) -> None:
        canvas = TextCanvas(1, 5)

        self.assertFalse(canvas.is_textual)

        canvas.merge_text_vertical("bar", 0, 1)
        canvas.merge_text_vertical(" z", 0, 2)

        self.assertTrue(canvas.is_textual)

        self.assertEqual(
            canvas.text_buffer,
            [
                [""],
                ["b"],
                ["a"],
                ["z"],
                [""],
            ],
            "Incorrect text buffer.",
        )

    def test_get_text_as_string(self) -> None:
        canvas = TextCanvas(5, 3)

        canvas.draw_text("foo", 1, 1)

        self.assertEqual(
            canvas.to_string(), "⠀⠀⠀⠀⠀\n⠀foo⠀\n⠀⠀⠀⠀⠀\n", "Incorrect output string."
        )

    def test_get_text_as_string_colored(self) -> None:
        canvas = TextCanvas(5, 3)

        canvas.set_color(Color().bright_green())
        canvas.draw_text("foo", 1, 1)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀\n⠀\x1b[0;92mf\x1b[0m\x1b[0;92mo\x1b[0m\x1b[0;92mo\x1b[0m⠀\n⠀⠀⠀⠀⠀\n",
            "Incorrect output string.",
        )

    def test_clear_clears_text_buffer(self) -> None:
        canvas = TextCanvas(2, 1)

        self.assertEqual(canvas.text_buffer, [], "Text buffer should be empty.")

        canvas.set_color(Color().bright_red())
        canvas.draw_text("hi", 0, 0)

        self.assertEqual(
            canvas.text_buffer,
            [["\x1b[0;91mh\x1b[0m", "\x1b[0;91mi\x1b[0m"]],
            "Text should be colorized.",
        )

        canvas.clear()

        self.assertEqual(
            canvas.text_buffer,
            [["", ""]],
            "Text buffer should be full of no-colored empty chars.",
        )

    def test_clear_edits_text_buffer_in_place(self) -> None:
        canvas = TextCanvas(2, 2)
        canvas.draw_text("hi", 0, 0)

        text_buffer = canvas.text_buffer
        row_0 = canvas.text_buffer[0]
        row_1 = canvas.text_buffer[1]

        canvas.clear()

        self.assertIs(
            text_buffer, canvas.text_buffer, "Container should be the same as before."
        )
        self.assertIs(
            row_0, canvas.text_buffer[0], "Container should be the same as before."
        )
        self.assertIs(
            row_1, canvas.text_buffer[1], "Container should be the same as before."
        )


class TestTextCanvasDrawingPrimitives(unittest.TestCase):
    def test_stroke_line(self) -> None:
        canvas = TextCanvas(15, 5)

        top_left = (0, 0)
        top_right = (canvas.w, 0)
        bottom_right = (canvas.w, canvas.h)
        bottom_left = (0, canvas.h)
        center = (canvas.cx, canvas.cy)
        center_top = (canvas.cx, 0)
        center_right = (canvas.w, canvas.cy)
        center_bottom = (canvas.cx, canvas.h)
        center_left = (0, canvas.cy)

        canvas.stroke_line(*center, *top_left)
        canvas.stroke_line(*center, *top_right)
        canvas.stroke_line(*center, *bottom_right)
        canvas.stroke_line(*center, *bottom_left)
        canvas.stroke_line(*center, *center_top)
        canvas.stroke_line(*center, *center_right)
        canvas.stroke_line(*center, *center_bottom)
        canvas.stroke_line(*center, *center_left)

        self.assertEqual(
            canvas.to_string(),
            "⠑⠢⣀⠀⠀⠀⠀⢸⠀⠀⠀⠀⢀⠔⠊\n"
            "⠀⠀⠀⠑⠢⣀⠀⢸⠀⢀⠤⠊⠁⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⢵⣾⣶⠥⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⣀⠤⠊⠁⢸⠀⠑⠢⣀⠀⠀⠀\n"
            "⡠⠔⠊⠀⠀⠀⠀⢸⠀⠀⠀⠀⠉⠢⢄\n",
            "Lines not drawn correctly.",
        )

    def test_stroke_line_from_outside_to_outside(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_line(-10, -10, canvas.w + 10, canvas.h + 10)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠈⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠉⠢⣀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⡀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⣀⠀\n",
            "Line not drawn correctly.",
        )

    def test_erase_line(self) -> None:
        canvas = TextCanvas(15, 5)

        # Fill canvas.
        canvas.fill()

        top_left = (0, 0)
        bottom_right = (canvas.w, canvas.h)

        canvas.invert()
        canvas.stroke_line(*top_left, *bottom_right)

        self.assertEqual(
            canvas.to_string(),
            "⣮⣝⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n"
            "⣿⣿⣿⣮⣝⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n"
            "⣿⣿⣿⣿⣿⣿⣮⣝⡻⣿⣿⣿⣿⣿⣿\n"
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣝⡻⣿⣿⣿\n"
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣝⡻\n",
            "Line not erased correctly.",
        )

    def test_stroke_rect(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_rect(6, 3, 20, 15)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀\n"
            "⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀\n"
            "⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀\n"
            "⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀\n"
            "⠀⠀⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀⠀\n",
        )

    def test_frame(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.frame()

        self.assertEqual(
            canvas.to_string(),
            "⡏⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣸\n",
        )

    def test_fill_rect(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.fill_rect(6, 3, 20, 15)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀\n"
            "⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀\n"
            "⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀\n"
            "⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀\n"
            "⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀\n",
        )

    def test_stroke_triangle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_triangle(6, 3, 20, 2, 23, 18)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⣀⣀⣀⡠⠤⠤⠤⡄⠀⠀⠀⠀\n"
            "⠀⠀⠀⠈⠢⣀⠀⠀⠀⠀⢱⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠘⡄⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠑⠤⡀⡇⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠺⠀⠀⠀\n",
        )

    def test_fill_triangle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.fill_triangle(6, 3, 20, 2, 23, 18)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⣀⣀⣀⣠⣤⣤⣤⡄⠀⠀⠀⠀\n"
            "⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⡄⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⣿⡇⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⠀⠀⠀\n",
        )

    def test_stroke_circle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_circle(15, 10, 7)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡠⠊⠀⠀⠀⠈⠢⡀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀\n"
            "⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⡠⠃⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠈⠒⠒⠒⠊⠀⠀⠀⠀⠀\n",
        )

    def test_fill_circle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.fill_circle(15, 10, 7)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣦⡀⠀⠀⠀\n"
            "⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀\n"
            "⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⡿⠃⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠈⠛⠛⠛⠋⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_ngon(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_ngon(canvas.cx, canvas.cy, 7, 6, 0.0)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⡰⠉⠉⠉⠱⡀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⢜⠀⠀⠀⠀⠀⢘⠄⠀⠀⠀\n"
            "⠀⠀⠀⠀⠈⢆⠀⠀⠀⢠⠊⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_ngon_at_angle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_ngon(canvas.cx, canvas.cy, 7, 6, math.pi / 2.0)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⢠⠔⠊⠁⠉⠢⢄⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠘⠤⡀⠀⠀⣀⠼⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠈⠑⠉⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_ngon_radius_matches_circle(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.stroke_ngon(canvas.cx, canvas.cy, 7, 3, math.pi / 2.0)

        canvas.stroke_circle(15, 10, 7)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⣀⣀⣀⡀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡠⠊⢠⠃⢣⠈⠢⡀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡇⡰⠁⠀⠀⢣⠀⡇⠀⠀⠀\n"
            "⠀⠀⠀⠀⠳⡓⠒⠢⠤⠤⡧⠃⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠈⠒⠒⠒⠊⠀⠀⠀⠀⠀\n",
        )

    def test_fill_ngon(self) -> None:
        canvas = TextCanvas(15, 5)

        canvas.fill_ngon(canvas.cx, canvas.cy, 7, 6, 0.0)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⣰⣿⣿⣿⣷⡀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⢼⣿⣿⣿⣿⣿⣿⠄⠀⠀⠀\n"
            "⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⠋⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀\n",
        )

    def test_fill_ngon_not_enough_sides(self) -> None:
        canvas = TextCanvas(15, 5)

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Number of sides less than 3 did not raise error."
            canvas.fill_ngon(canvas.cx, canvas.cy, 7, 2, 0.0)

    def test_draw_canvas(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.stroke_line(0, 0, canvas.w, canvas.h)
        canvas.frame()

        overlay = TextCanvas(7, 3)
        overlay.stroke_line(0, overlay.h, overlay.w, 0)
        overlay.frame()

        canvas.draw_canvas(overlay, 8, 4)

        self.assertEqual(
            canvas.to_string(),
            "⡟⠫⣉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹\n"
            "⡇⠀⠀⠑⡏⠉⠉⠉⢉⡩⢻⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⡇⠀⢀⠔⠁⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⣧⣊⣁⣀⣀⣀⣸⢄⠀⠀⢸\n"
            "⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣉⣢⣼\n",
        )

    def test_draw_canvas_with_overflow(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.stroke_line(0, 0, canvas.w, canvas.h)
        canvas.frame()

        overlay = TextCanvas(7, 3)
        overlay.stroke_line(0, overlay.h, overlay.w, 0)
        overlay.frame()

        canvas.draw_canvas(overlay, -8, -4)
        canvas.draw_canvas(overlay, 24, -4)
        canvas.draw_canvas(overlay, 24, 12)
        canvas.draw_canvas(overlay, -8, 12)

        self.assertEqual(
            canvas.to_string(),
            "⠁⠀⢸⠉⠉⠉⠉⠉⠉⠉⠉⠉⡇⠀⢀\n"
            "⣀⣀⣸⠑⠢⣀⠀⠀⠀⠀⠀⠀⣧⣊⣁\n"
            "⡇⠀⠀⠀⠀⠀⠑⠢⢄⠀⠀⠀⠀⠀⢸\n"
            "⢉⡩⢻⠀⠀⠀⠀⠀⠀⠉⠢⢄⡏⠉⠉\n"
            "⠁⠀⢸⣀⣀⣀⣀⣀⣀⣀⣀⣀⡇⠀⢀\n",
        )

    def test_draw_canvas_with_color(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.set_color(Color().red())
        canvas.fill_rect(3, 3, 10, 10)

        overlay = TextCanvas(15, 5)
        overlay.set_color(Color().green())
        overlay.fill_rect(8, 8, 10, 10)

        canvas.draw_canvas(overlay, 8, 8)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀\x1b[0;31m⢀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⡀\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⡇\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⠈\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;31m⠉\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀\n",
        )

    def test_draw_canvas_with_color_onto_non_colorized_canvas(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.fill_rect(3, 3, 10, 10)

        self.assertFalse(canvas.is_colorized)

        overlay = TextCanvas(15, 5)
        overlay.set_color(Color().green())
        overlay.fill_rect(8, 8, 10, 10)

        canvas.draw_canvas(overlay, 8, 8)

        self.assertTrue(canvas.is_colorized)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⢸⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⢸⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀\n",
        )

    def test_draw_canvas_with_text(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.draw_text("abcde", 1, 1)

        overlay = TextCanvas(7, 3)
        overlay.draw_text("012", 2, 1)

        canvas.draw_canvas(overlay, 5, 0)

        # print(f"{canvas}")

        self.assertEqual(canvas.to_string(), "⠀⠀⠀⠀⠀⠀⠀\n" "⠀a⠀⠀012\n" "⠀⠀⠀⠀⠀⠀⠀\n")

    def test_draw_canvas_with_colored_text(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.set_color(Color().red())
        canvas.draw_text("abcde", 1, 1)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.draw_canvas(overlay, 5, 0)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31ma\x1b[0m⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_draw_canvas_with_colored_text_onto_non_textual_canvas(self) -> None:
        canvas = TextCanvas(7, 3)

        self.assertFalse(canvas.is_colorized)
        self.assertFalse(canvas.is_textual)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.draw_canvas(overlay, 5, 0)

        self.assertTrue(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_draw_canvas_with_colored_text_onto_non_colorized_canvas(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.draw_text("abcde", 1, 1)

        self.assertFalse(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.draw_canvas(overlay, 5, 0)

        self.assertTrue(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀a⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.stroke_line(0, 0, canvas.w, canvas.h)
        canvas.frame()

        overlay = TextCanvas(7, 3)
        overlay.stroke_line(0, overlay.h, overlay.w, 0)
        overlay.frame()

        canvas.merge_canvas(overlay, 8, 4)

        self.assertEqual(
            canvas.to_string(),
            "⡟⠫⣉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢹\n"
            "⡇⠀⠀⠑⡯⣉⠉⠉⢉⡩⢻⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⡇⠀⢑⠶⢅⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⠀⣧⣊⣁⣀⣀⣉⣺⢄⠀⠀⢸\n"
            "⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣉⣢⣼\n",
        )

    def test_merge_canvas_with_overflow(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.stroke_line(0, 0, canvas.w, canvas.h)
        canvas.frame()

        overlay = TextCanvas(7, 3)
        overlay.stroke_line(0, overlay.h, overlay.w, 0)
        overlay.frame()

        canvas.merge_canvas(overlay, -8, -4)
        canvas.merge_canvas(overlay, 24, -4)
        canvas.merge_canvas(overlay, 24, 12)
        canvas.merge_canvas(overlay, -8, 12)

        self.assertEqual(
            canvas.to_string(),
            "⡟⠫⣹⠉⠉⠉⠉⠉⠉⠉⠉⠉⡏⠉⢹\n"
            "⣇⣀⣸⠑⠢⣀⠀⠀⠀⠀⠀⠀⣧⣊⣹\n"
            "⡇⠀⠀⠀⠀⠀⠑⠢⢄⠀⠀⠀⠀⠀⢸\n"
            "⣏⡩⢻⠀⠀⠀⠀⠀⠀⠉⠢⢄⡏⠉⢹\n"
            "⣇⣀⣸⣀⣀⣀⣀⣀⣀⣀⣀⣀⣏⣢⣼\n",
        )

    def test_merge_canvas_with_color(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.set_color(Color().red())
        canvas.fill_rect(3, 3, 10, 10)

        overlay = TextCanvas(15, 5)
        overlay.set_color(Color().green())
        overlay.fill_rect(8, 8, 10, 10)

        canvas.merge_canvas(overlay, 0, 0)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀\x1b[0;31m⢀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⣀\x1b[0m\x1b[0;31m⡀\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⡇\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⢸\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;31m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31m⠈\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;31m⠉\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas_with_color_onto_non_colorized_canvas(self) -> None:
        canvas = TextCanvas(15, 5)
        canvas.fill_rect(3, 3, 10, 10)

        self.assertFalse(canvas.is_colorized)

        overlay = TextCanvas(15, 5)
        overlay.set_color(Color().green())
        overlay.fill_rect(8, 8, 10, 10)

        canvas.merge_canvas(overlay, 0, 0)

        self.assertTrue(canvas.is_colorized)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⢸⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⢸⣿⣿\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀\n"
            "⠀⠈⠉⠉\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m\x1b[0;32m⣿\x1b[0m⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m\x1b[0;32m⠛\x1b[0m⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas_with_text(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.draw_text("abcde", 1, 1)

        overlay = TextCanvas(7, 3)
        overlay.draw_text("012", 2, 1)

        canvas.merge_canvas(overlay, 0, 0)

        # print(f"{canvas}")

        self.assertEqual(canvas.to_string(), "⠀⠀⠀⠀⠀⠀⠀\n" "⠀a012e⠀\n" "⠀⠀⠀⠀⠀⠀⠀\n")

    def test_merge_canvas_with_colored_text(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.set_color(Color().red())
        canvas.draw_text("abcde", 1, 1)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.merge_canvas(overlay, 5, 0)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀\x1b[0;31ma\x1b[0m\x1b[0;31mb\x1b[0m\x1b[0;31mc\x1b[0m\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas_with_colored_text_onto_non_textual_canvas(self) -> None:
        canvas = TextCanvas(7, 3)

        self.assertFalse(canvas.is_colorized)
        self.assertFalse(canvas.is_textual)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.merge_canvas(overlay, 5, 0)

        self.assertTrue(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas_with_colored_text_onto_non_colorized_canvas(self) -> None:
        canvas = TextCanvas(7, 3)
        canvas.draw_text("abcde", 1, 1)

        self.assertFalse(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        overlay = TextCanvas(7, 3)
        overlay.set_color(Color().green())
        overlay.draw_text("012", 2, 1)

        canvas.merge_canvas(overlay, 5, 0)

        self.assertTrue(canvas.is_colorized)
        self.assertTrue(canvas.is_textual)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀abc\x1b[0;32m0\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;32m2\x1b[0m\n"
            "⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_merge_canvas_with_pixels_color_and_text(self) -> None:
        canvas = TextCanvas(7, 5)
        canvas.set_color(Color().red())
        canvas.draw_text("abcdefg", 0, 2)
        canvas.set_color(Color().blue())
        canvas.stroke_line(0, 13, canvas.w, 13)

        overlay = TextCanvas(7, 5)
        overlay.set_color(Color().green())
        overlay.draw_text_vertical("012", canvas.cx // 2, 1)
        overlay.set_color(Color().yellow())
        overlay.stroke_line(overlay.cx, 0, overlay.cx, overlay.h)

        canvas.merge_canvas(overlay, 0, 0)

        # print(f"{canvas}")

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀\x1b[0;33m⢸\x1b[0m⠀⠀⠀\n"
            "⠀⠀⠀\x1b[0;32m0\x1b[0m⠀⠀⠀\n"
            "\x1b[0;31ma\x1b[0m\x1b[0;31mb\x1b[0m\x1b[0;31mc\x1b[0m\x1b[0;32m1\x1b[0m\x1b[0;31me\x1b[0m\x1b[0;31mf\x1b[0m\x1b[0;31mg\x1b[0m\n"
            "\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;32m2\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\x1b[0;34m⠒\x1b[0m\n"
            "⠀⠀⠀\x1b[0;33m⢸\x1b[0m⠀⠀⠀\n",
        )


if __name__ == "__main__":
    unittest.main()
