import unittest

from textcanvas.textcanvas import Color, TextCanvas


def stroke_line_accros_canvas(canvas: TextCanvas) -> None:
    y = 0
    for x in range(canvas.screen.width):
        canvas.set_pixel(x, y, True)
        y += 1


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

    def test_color_buffer_size_at_init(self) -> None:
        canvas = TextCanvas(7, 4)

        self.assertEqual(len(canvas.color_buffer), 0, "Color buffer should be empty.")

    def test_color_buffer_size_with_color(self) -> None:
        canvas = TextCanvas(7, 4)

        canvas.set_color(Color.BG_BLUE)

        buffer_width = len(canvas.color_buffer[0])
        buffer_height = len(canvas.color_buffer)

        self.assertEqual(
            buffer_width, 7, "Color buffer width should match output buffer width."
        )
        self.assertEqual(
            buffer_height, 4, "Color buffer height should match output buffer height."
        )

    def test_default_size(self) -> None:
        canvas = TextCanvas()

        self.assertEqual(canvas.output.width, 80, "Incorrect default width.")
        self.assertEqual(canvas.output.height, 24, "Incorrect default height.")

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

    def test_clear(self) -> None:
        canvas = TextCanvas(2, 2)

        # Fill canvas.
        for x in range(canvas.screen.width):
            for y in range(canvas.screen.height):
                canvas.set_pixel(x, y, True)

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

    def test_turn_all_pixels_on(self) -> None:
        canvas = TextCanvas(2, 2)

        for x in range(canvas.screen.width):
            for y in range(canvas.screen.height):
                canvas.set_pixel(x, y, True)

        self.assertEqual(canvas.to_string(), "⣿⣿\n⣿⣿\n", "Output not fully on.")

    def test_is_colorized(self) -> None:
        canvas = TextCanvas(2, 2)

        self.assertFalse(
            canvas.is_colorized, "Canvas should not be colorized by default."
        )

        canvas.set_color(Color.BG_BLUE)

        self.assertTrue(
            canvas.is_colorized, "Canvas should be colorized after a color is set."
        )

    def test_set_color(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color.BG_BLUE)
        canvas.set_pixel(3, 3, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color.NO_COLOR, Color.BG_BLUE],
                [Color.NO_COLOR, Color.NO_COLOR],
            ],
            "Incorrect color buffer.",
        )

    def test_set_color_multiple(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color.BG_BLUE)
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color.NO_COLOR, Color.BG_BLUE],
                [Color.BG_BLUE, Color.NO_COLOR],
            ],
            "Incorrect color buffer.",
        )

    def test_set_color_override(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color.BG_BLUE)
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        canvas.set_color(Color.BG_RED)
        canvas.set_pixel(3, 3, True)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color.NO_COLOR, Color.BG_RED],
                [Color.BG_BLUE, Color.NO_COLOR],
            ],
            "Incorrect color buffer.",
        )

    def test_color_is_reset_if_pixel_turned_off(self) -> None:
        canvas = TextCanvas(2, 2)

        canvas.set_color(Color.BG_BLUE)
        canvas.set_pixel(3, 3, True)
        canvas.set_pixel(1, 5, True)

        canvas.set_pixel(3, 3, False)

        self.assertEqual(
            canvas.color_buffer,
            [
                [Color.NO_COLOR, Color.NO_COLOR],
                [Color.BG_BLUE, Color.NO_COLOR],
            ],
            "Incorrect color buffer.",
        )

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

    def test_get_as_string_colored(self) -> None:
        canvas = TextCanvas(3, 2)
        canvas.set_color(Color.GREEN)
        stroke_line_accros_canvas(canvas)

        self.assertEqual(
            canvas.to_string(),
            "\x1b[0;92m⠑\x1b[0m\x1b[0;92m⢄\x1b[0m⠀\n⠀⠀\x1b[0;92m⠑\x1b[0m\n",
            "Incorrect output string.",
        )

    def test_iter_buffer_by_blocks_lrtb(self) -> None:
        # This tests a private method, but this method is at the core of
        # the output generation. Testing it helps to ensure stability.
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

    def test_stroke_line(self) -> None:
        canvas = TextCanvas(15, 5)

        xw = canvas.screen.width - 1
        yh = canvas.screen.height - 1
        cx = canvas.screen.width // 2
        cy = canvas.screen.height // 2

        top_left = (0, 0)
        top_right = (xw, 0)
        bottom_right = (xw, yh)
        bottom_left = (0, yh)
        center = (cx, cy)
        center_top = (cx, 0)
        center_right = (xw, cy)
        center_bottom = (cx, yh)
        center_left = (0, cy)

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
