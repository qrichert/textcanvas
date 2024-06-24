import doctest
import math
import unittest
from dataclasses import dataclass

import textcanvas.charts
from textcanvas.charts import Chart, Plot
from textcanvas.textcanvas import TextCanvas


def load_tests(
    loader: unittest.TestLoader, tests: unittest.TestSuite, ignore: str
) -> unittest.TestSuite:
    """Add module doctests."""
    tests.addTests(doctest.DocTestSuite(textcanvas.charts))
    return tests


class TestPlot(unittest.TestCase):
    def test_stroke_x_and_y_axes(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_x_axis_at_top_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = list(range(-5, 1))

        Plot.stroke_x_axis(canvas, y)

        self.assertEqual(
            canvas.to_string(),
            "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_x_axis_at_bottom_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = list(range(0, 6))

        Plot.stroke_x_axis(canvas, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n",
        )

    def test_stroke_y_axis_at_left_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(0, 6))

        Plot.stroke_y_axis(canvas, x)

        self.assertEqual(
            canvas.to_string(),
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_y_axis_at_right_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 1))

        Plot.stroke_y_axis(canvas, x)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n",
        )

    def test_stroke_line_at_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))

        Plot.stroke_line_at_x(canvas, -5.0, x)
        Plot.stroke_line_at_x(canvas, -2.5, x)
        Plot.stroke_line_at_x(canvas, 0.0, x)
        Plot.stroke_line_at_x(canvas, 2.5, x)
        Plot.stroke_line_at_x(canvas, 5.0, x)

        self.assertEqual(
            canvas.to_string(),
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n",
        )

    def test_stroke_line_at_x_ignore_empty_values(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = []

        Plot.stroke_line_at_x(canvas, 0.0, x)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_line_at_y(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = list(range(-5, 6))

        Plot.stroke_line_at_y(canvas, -5.0, y)
        Plot.stroke_line_at_y(canvas, -2.5, y)
        Plot.stroke_line_at_y(canvas, 0.0, y)
        Plot.stroke_line_at_y(canvas, 2.5, y)
        Plot.stroke_line_at_y(canvas, 5.0, y)

        self.assertEqual(
            canvas.to_string(),
            "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉\n"
            "⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n",
        )

    def test_stroke_line_at_y_ignore_empty_values(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = []

        Plot.stroke_line_at_y(canvas, 0.0, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_compute_screen_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-10, 11))

        self.assertEqual(0, Plot.compute_screen_x(canvas, -10.0, x))
        self.assertEqual(29, Plot.compute_screen_x(canvas, 10.0, x))
        self.assertEqual(14, Plot.compute_screen_x(canvas, 0.0, x))

    def test_compute_screen_x_input_size_1(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [3.0]

        self.assertEqual(15, Plot.compute_screen_x(canvas, 0.0, x))

    def test_compute_screen_x_empty_input(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = []

        self.assertIsNone(Plot.compute_screen_x(canvas, 0.0, x))

    def test_compute_screen_y(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = list(range(-10, 11))

        self.assertEqual(19, Plot.compute_screen_y(canvas, -10.0, y))
        self.assertEqual(0, Plot.compute_screen_y(canvas, 10.0, y))
        self.assertEqual(10, Plot.compute_screen_y(canvas, 0.0, y))

    def test_compute_screen_y_input_size_1(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = [3.0]

        self.assertEqual(10, Plot.compute_screen_y(canvas, 0.0, y))

    def test_compute_screen_y_empty_input(self) -> None:
        canvas = TextCanvas(15, 5)

        y: list[float] = []

        self.assertIsNone(Plot.compute_screen_y(canvas, 0.0, y))

    def test_stroke_x_and_y_axes_of_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_xy_axes_of_function(canvas, -5.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_x_axis_of_function_at_top_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_x_axis_of_function(canvas, -5.0, 0.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_x_axis_of_function_at_bottom_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_x_axis_of_function(canvas, 0.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n",
        )

    def test_stroke_y_axis_of_function_at_left_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_y_axis_of_function(canvas, 0.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_y_axis_of_function_at_right_boundary(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_y_axis_of_function(canvas, -5.0, 0.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸\n",
        )

    def test_stroke_line_at_x_of_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_line_at_x_of_function(canvas, -5.0, -5.0, 5.0, f)
        Plot.stroke_line_at_x_of_function(canvas, -2.5, -5.0, 5.0, f)
        Plot.stroke_line_at_x_of_function(canvas, 0.0, -5.0, 5.0, f)
        Plot.stroke_line_at_x_of_function(canvas, 2.5, -5.0, 5.0, f)
        Plot.stroke_line_at_x_of_function(canvas, 5.0, -5.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n"
            "⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸\n",
        )

    def test_stroke_line_at_x_of_function_value_out_of_bounds(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_line_at_x_of_function(canvas, -100.0, -5.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_stroke_line_at_y_of_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_line_at_y_of_function(canvas, -5.0, -5.0, 5.0, f)
        Plot.stroke_line_at_y_of_function(canvas, -2.5, -5.0, 5.0, f)
        Plot.stroke_line_at_y_of_function(canvas, 0.0, -5.0, 5.0, f)
        Plot.stroke_line_at_y_of_function(canvas, 2.5, -5.0, 5.0, f)
        Plot.stroke_line_at_y_of_function(canvas, 5.0, -5.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉\n"
            "⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n"
            "⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀\n",
        )

    def test_stroke_line_at_y_of_function_value_out_of_bounds(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        Plot.stroke_line_at_y_of_function(canvas, -100.0, -5.0, 5.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_compute_screen_x_of_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            0, Plot.compute_screen_x_of_function(canvas, -10.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            14, Plot.compute_screen_x_of_function(canvas, 0.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            29, Plot.compute_screen_x_of_function(canvas, 10.0, -10.0, 10.0, f)
        )

    def test_compute_screen_x_of_function_range_0(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            15, Plot.compute_screen_x_of_function(canvas, -10.0, 0.0, 0.0, f)
        )
        self.assertEqual(
            15, Plot.compute_screen_x_of_function(canvas, 0.0, 0.0, 0.0, f)
        )
        self.assertEqual(
            15, Plot.compute_screen_x_of_function(canvas, 10.0, 0.0, 0.0, f)
        )

    def test_compute_screen_x_of_function_canvas_size_1x1(self) -> None:
        canvas = TextCanvas(1, 1)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            0, Plot.compute_screen_x_of_function(canvas, -10.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            0, Plot.compute_screen_x_of_function(canvas, 0.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            1, Plot.compute_screen_x_of_function(canvas, 10.0, -10.0, 10.0, f)
        )

    def test_compute_screen_y_of_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            19,
            Plot.compute_screen_y_of_function(canvas, -10.0, -10.0, 10.0, f),
        )
        self.assertEqual(
            10, Plot.compute_screen_y_of_function(canvas, 0.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            0, Plot.compute_screen_y_of_function(canvas, 10.0, -10.0, 10.0, f)
        )

    def test_compute_screen_y_of_function_range_0(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            10, Plot.compute_screen_y_of_function(canvas, -10.0, 0.0, 0.0, f)
        )
        self.assertEqual(
            10, Plot.compute_screen_y_of_function(canvas, 0.0, 0.0, 0.0, f)
        )
        self.assertEqual(
            10, Plot.compute_screen_y_of_function(canvas, 10.0, 0.0, 0.0, f)
        )

    def test_compute_screen_y_of_function_canvas_size_1x1(self) -> None:
        canvas = TextCanvas(1, 1)

        def f(x: float) -> float:
            return x

        self.assertEqual(
            3, Plot.compute_screen_y_of_function(canvas, -10.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            2, Plot.compute_screen_y_of_function(canvas, 0.0, -10.0, 10.0, f)
        )
        self.assertEqual(
            0, Plot.compute_screen_y_of_function(canvas, 10.0, -10.0, 10.0, f)
        )

    def test_plot_line(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠤⠒⠉\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⢀⠤⠊⠁⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⢤⠤⡯⠥⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⢀⠤⠊⠁⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⡠⠊⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_empty_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = []
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_empty_y(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = []

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_sorts_elements_by_x_before_plotting(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [-5.0, 5.0, -2.5]
        y: list[float] = [5.0, 2.5, -2.5]

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        # Not sorted, it would look like this:
        # ⠉⠑⠒⠒⠤⠤⢄⣇⡀⠀⠀⠀⠀⠀⠀
        # ⠀⠀⠀⠀⠀⠀⠀⡇⠈⠉⠉⠒⠒⢢⡤
        # ⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⣀⠤⠊⠁⠀
        # ⠒⠒⠒⠒⠒⠒⢒⡷⠖⠚⠒⠒⠒⠒⠒
        # ⠀⠀⠀⢀⠤⠒⠁⡇⠀⠀⠀⠀⠀⠀⠀
        self.assertEqual(
            canvas.to_string(),
            "⢣⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠈⢆⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⡠\n"
            "⠀⠘⡄⠀⠀⠀⠀⡇⠀⠀⣀⠤⠊⠁⠀\n"
            "⠒⠒⠳⡒⠒⠒⢒⡷⠖⠛⠒⠒⠒⠒⠒\n"
            "⠀⠀⠀⢣⠤⠒⠁⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_single_value(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0]
        y: list[float] = [0]

        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_range_xy_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0 for _ in range(-5, 6)]
        y: list[float] = [0 for _ in range(-5, 6)]

        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_range_x_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0 for _ in range(-5, 6)]
        y: list[float] = list(range(-5, 6))

        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_range_y_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = [0 for _ in range(-5, 6)]

        Plot.line(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_x_and_y_of_different_lengths_more_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-10, 11))
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        # The scale is correct. At X = 0, Y = 5. To see values on the
        # right, you'd have to increase the range of Y (up to 15, to
        # match X).
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⢀⠔⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡠⠊⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⢤⠴⠥⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⡠⠊⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⡰⠁⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_line_with_x_and_y_of_different_lengths_more_y(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-10, 11))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.line(canvas, x, y)

        # The scale is correct. Y range is [-10;10], (0;10) is just
        # not rendered because X stops when Y = 0. If you'd continue
        # to the right, Y would reach 10 at X = 15.
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⣤⡤⠤⠶\n"
            "⠀⠀⠀⠀⠀⣀⡠⡧⠒⠊⠉⠀⠀⠀⠀\n"
            "⡠⠤⠒⠊⠉⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠀⠂⠈\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⢀⠀⠂⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⢤⠤⡧⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⢀⠀⠂⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⡀⠂⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_empty_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = []
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_empty_y(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = []

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_single_value(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0]
        y: list[float] = [0]

        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_range_xy_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0 for _ in range(-5, 6)]
        y: list[float] = [0 for _ in range(-5, 6)]

        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_range_x_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = [0 for _ in range(-5, 6)]
        y: list[float] = list(range(-5, 6))

        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⢨⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_range_y_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = [0 for _ in range(-5, 6)]

        Plot.scatter(canvas, x, y)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠄⠄⠠⠀⠄⠠⠀⠄⠠⠀⠄⠠⠀⠄⠠\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_x_and_y_of_different_lengths_more_x(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-10, 11))
        y: list[float] = list(range(-5, 6))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.scatter(canvas, x, y)

        # The scale is correct. At X = 0, Y = 5. To see values on the
        # right, you'd have to increase the range of Y (up to 15, to
        # match X).
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⢀⠐⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⡀⠂⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⢤⠴⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⡀⠂⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⡐⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_scatter_with_x_and_y_of_different_lengths_more_y(self) -> None:
        canvas = TextCanvas(15, 5)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-10, 11))

        Plot.stroke_xy_axes(canvas, x, y)
        Plot.scatter(canvas, x, y)

        # The scale is correct. Y range is [-10;10], (0;10) is just
        # not rendered because X stops when Y = 0. If you'd continue
        # to the right, Y would reach 10 at X = 15.
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⢤⠤⠤⠴\n"
            "⠀⠀⠀⠀⠀⢀⠀⡇⠐⠀⠁⠀⠀⠀⠀\n"
            "⡀⠄⠐⠀⠁⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_function(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(x: float) -> float:
            return x * x

        Plot.stroke_xy_axes_of_function(canvas, -10.0, 10.0, f)
        Plot.function(canvas, -10.0, 10.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠱⡀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡜\n"
            "⠀⢣⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⡜⠀\n"
            "⠀⠀⠣⡀⠀⠀⠀⡇⠀⠀⠀⠀⡔⠁⠀\n"
            "⠀⠀⠀⠑⡄⠀⠀⡇⠀⠀⢀⠎⠀⠀⠀\n"
            "⣀⣀⣀⣀⣈⣒⣤⣇⣤⣒⣁⣀⣀⣀⣀\n",
        )

    def test_plot_function_with_single_value(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(_: float) -> float:
            return 0

        Plot.function(canvas, 0.0, 0.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_plot_function_with_range_zero(self) -> None:
        canvas = TextCanvas(15, 5)

        def f(_: float) -> float:
            return 0

        Plot.function(canvas, -10.0, 10.0, f)

        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_compute_function_works_with_structs(self) -> None:
        @dataclass
        class Mock:
            foo: float
            bar: float

        def f(x: float) -> Mock:
            return Mock(foo=x, bar=-x)

        # Compute all values once. Y will contain structs.
        (x, y) = Plot.compute_function(-5.0, 5.0, 5.0, f)

        self.assertEqual(x, [-5.0, -2.5, 0.0, 2.5, 5.0])
        self.assertEqual(
            y,
            [
                Mock(-5.0, 5.0),
                Mock(-2.5, 2.5),
                Mock(0.0, -0.0),
                Mock(2.5, -2.5),
                Mock(5.0, -5.0),
            ],
        )

        # Extract struct fields.
        y_foo: list[float] = [mock.foo for mock in y]
        y_bar: list[float] = [mock.bar for mock in y]

        self.assertEqual(y_foo, [-5.0, -2.5, 0.0, 2.5, 5.0])
        self.assertEqual(y_bar, [5.0, 2.5, -0.0, -2.5, -5.0])


class TestChart(unittest.TestCase):
    def test_chart_x_squared(self) -> None:
        canvas = TextCanvas(71, 19)

        def f(x: float) -> float:
            return x * x

        Chart.function(canvas, -10.0, 10.0, f)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀100⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠋⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠈⢢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠒⠢⠤⠤⢄⡠⠤⠤⠴⠒⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀0.0073⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀-10⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀10\n",
        )

    def test_chart_polynomial(self) -> None:
        canvas = TextCanvas(71, 19)

        def f(x: float) -> float:
            return x**3 - 2 * x**2 + 3 * x

        Chart.function(canvas, -5.0, 5.0, f)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀90⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⡠⠤⠤⠔⠒⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠤⠒⠒⠒⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠔⠚⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀-190⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5\n",
        )

    def test_chart_cos(self) -> None:
        canvas = TextCanvas(71, 19)

        def f(x: float) -> float:
            return math.cos(x)

        Chart.function(canvas, 0.0, 5.0, f)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀1⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠉⠉⠉⠒⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠙⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠒⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠢⠤⠤⢄⠤⠤⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀-1⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀0⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5\n",
        )

    def test_chart_line(self) -> None:
        canvas = TextCanvas(35, 10)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        Chart.line(canvas, x, y)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠒⠉⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠊⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⡠⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢀⡠⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5\n",
        )

    def test_chart_scatter(self) -> None:
        canvas = TextCanvas(35, 10)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        Chart.scatter(canvas, x, y)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠈⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠈⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠠⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n"
            "⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5\n",
        )

    def test_chart_empty(self) -> None:
        canvas = TextCanvas(35, 10)

        x: list[float] = []
        y: list[float] = []

        Chart.line(canvas, x, y)
        Chart.scatter(canvas, x, y)

        # print(f"{canvas}")
        self.assertEqual(
            canvas.to_string(),
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n"
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        )

    def test_chart_canvas_too_small_both_horizontally_and_vertically(self) -> None:
        canvas = TextCanvas(Chart.HORIZONTAL_MARGIN, Chart.VERTICAL_MARGIN)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Canvas too small did not raise exception."
            Chart.scatter(canvas, x, y)

    def test_chart_canvas_too_small_horizontally(self) -> None:
        canvas = TextCanvas(Chart.HORIZONTAL_MARGIN, Chart.VERTICAL_MARGIN + 1)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Canvas too small horizontally did not raise exception."
            Chart.line(canvas, x, y)

    def test_chart_canvas_too_small_vertically(self) -> None:
        canvas = TextCanvas(Chart.HORIZONTAL_MARGIN + 1, Chart.VERTICAL_MARGIN)

        x: list[float] = list(range(-5, 6))
        y: list[float] = list(range(-5, 6))

        with self.assertRaises(ValueError) as ctx:
            ctx.msg = "Canvas too small vertically did not raise exception."
            Chart.line(canvas, x, y)

    def test_chart_pretty_number(self) -> None:
        self.assertEqual(Chart._format_number(1_570_000_000_000.0), "1.6T")
        self.assertEqual(Chart._format_number(1_000_000_000_000.0), "1.0T")

        self.assertEqual(Chart._format_number(1_570_000_000.0), "1.6B")
        self.assertEqual(Chart._format_number(1_000_000_000.0), "1.0B")

        self.assertEqual(Chart._format_number(1_570_000.0), "1.6M")
        self.assertEqual(Chart._format_number(1_000_000.0), "1.0M")

        self.assertEqual(Chart._format_number(100_000.0), "100.0K")

        self.assertEqual(Chart._format_number(10_570.0), "10.6K")
        self.assertEqual(Chart._format_number(10_000.0), "10.0K")

        self.assertEqual(Chart._format_number(1_570.0), "1570")
        self.assertEqual(Chart._format_number(1_000.0), "1000")

        self.assertEqual(Chart._format_number(1.0009), "1")
        self.assertEqual(Chart._format_number(-1.0009), "-1")

        self.assertEqual(Chart._format_number(0.010_57), "0.0106")
        self.assertEqual(Chart._format_number(0.010_00), "0.0100")

        self.assertEqual(Chart._format_number(0.000_001_57), "0")
        self.assertEqual(Chart._format_number(0.000_001_00), "0")

        self.assertEqual(Chart._format_number(-0.000_001_57), "0")
        self.assertEqual(Chart._format_number(-0.000_001_00), "0")

        self.assertEqual(Chart._format_number(-0.001_57), "-0.0016")
        self.assertEqual(Chart._format_number(-0.001_00), "-0.0010")

        self.assertEqual(Chart._format_number(-1_570.0), "-1570")
        self.assertEqual(Chart._format_number(-1_000.0), "-1000")

        self.assertEqual(Chart._format_number(-10_570.0), "-10.6K")
        self.assertEqual(Chart._format_number(-10_000.0), "-10.0K")

        self.assertEqual(Chart._format_number(-100_000.0), "-100.0K")

        self.assertEqual(Chart._format_number(-1_570_000.0), "-1.6M")
        self.assertEqual(Chart._format_number(-1_000_000.0), "-1.0M")

        self.assertEqual(Chart._format_number(-1_570_000_000.0), "-1.6B")
        self.assertEqual(Chart._format_number(-1_000_000_000.0), "-1.0B")

        self.assertEqual(Chart._format_number(-1_570_000_000_000.0), "-1.6T")
        self.assertEqual(Chart._format_number(-1_000_000_000_000.0), "-1.0T")


if __name__ == "__main__":
    unittest.main()
