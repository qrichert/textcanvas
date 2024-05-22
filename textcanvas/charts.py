import enum
from typing import Callable

from .textcanvas import TextCanvas


class PlotType(enum.Enum):
    LINE = "LINE"
    SCATTER = "SCATTER"


class Plot:
    """Helper functions to plot data on a `TextCanvas`.

    `Plot` does nothing magical. Calling functions on `Plot` is
    exactly like drawing manually on the canvas. This entails that
    nothing changes in the way you use the canvas before or after
    plotting. Nor does it change the way you apply colors.

    There are two classes of functions in `Plot`:

    - Functions that take a discrete set of values as input.
    - Functions that take a function as input (they all have `function`
      in their name).

    The main difference is that for those that take a discrete set as
    input, `Plot` does nothing in particular. But for those that take a
    function as input, `Plot` will be able to compute any value it needs
    to plot the function with the highest precision possible.

    Note:
        All the helper functions auto-scale the input data. The purpose
        of this is to have a _quick_ and _simple_ way to graph things
        out.

        Auto-scaling in this context means the lowest X value will be
        plotted on the left border of the canvas, and the highest X
        value will be plotted on the right side of the canvas, and all
        the values in-between will be distributed uniformly. Same for Y.

        If you absolutely need the plot to be smaller than the canvas,
        you need to plot it to a _different_ canvas that has the target
        size, and then draw the smaller canvas with the graph onto the
        parent canvas. Use `draw_canvas()` or `merge_canvas()` from
        `TextCanvas` to do this easily.
    """

    @staticmethod
    def stroke_xy_axes(canvas: TextCanvas, x: list[float], y: list[float]) -> None:
        """Stroke X and Y axes.

        If 0 is not visible on an axis, the axis will not be drawn.

        <div class="warning">

        `x` and `y` _should_ match in length,

        If `x` and `y` are not the same length, plotting will stop once
        the smallest of the two collections is consumed.

        </div>

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> x: list[float] = list(range(-5, 6))
            >>> y: list[float] = list(range(-5, 6))
            >>> Plot.stroke_xy_axes(canvas, x, y)
            >>> Plot.line(canvas, x, y)
            >>> print(canvas, end="")
            ⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠤⠒⠉
            ⠀⠀⠀⠀⠀⠀⠀⡇⢀⠤⠊⠁⠀⠀⠀
            ⠤⠤⠤⠤⠤⢤⠤⡯⠥⠤⠤⠤⠤⠤⠤
            ⠀⠀⢀⠤⠊⠁⠀⡇⠀⠀⠀⠀⠀⠀⠀
            ⡠⠊⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
        """
        Plot.stroke_x_axis(canvas, y)
        Plot.stroke_y_axis(canvas, x)

    @staticmethod
    def stroke_x_axis(canvas: TextCanvas, y: list[float]) -> None:
        """Stroke X axis.

        See `stroke_xy_axes()` which has the same API for an example.

        Args:
            y: Values of the Y axis, used to determine where Y = 0 is.
        """
        Plot.stroke_line_at_y(canvas, 0.0, y)

    @staticmethod
    def stroke_y_axis(canvas: TextCanvas, x: list[float]) -> None:
        """Stroke Y axis.

        See `stroke_xy_axes()` which has the same API for an example.

        Args:
            x: Values of the X axis, used to determine where X = 0 is.
        """
        Plot.stroke_line_at_x(canvas, 0.0, x)

    @staticmethod
    def stroke_line_at_x(canvas: TextCanvas, value: float, x: list[float]) -> None:
        """Stroke vertical line at X = value.

        If the value is out of the range of Y values, nothing will be
        drawn.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> x: list[float] = list(range(-5, 6))
            >>> Plot.stroke_line_at_x(canvas, -5.0, x)
            >>> Plot.stroke_line_at_x(canvas, -2.5, x)
            >>> Plot.stroke_line_at_x(canvas, 0.0, x)
            >>> Plot.stroke_line_at_x(canvas, 2.5, x)
            >>> Plot.stroke_line_at_x(canvas, 5.0, x)
            >>> print(canvas, end="")
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
        """
        if (screen_x := Plot.compute_screen_x(canvas, value, x)) is None:
            return
        canvas.stroke_line(screen_x, 0, screen_x, canvas.h)

    @staticmethod
    def stroke_line_at_y(canvas: TextCanvas, value: float, y: list[float]) -> None:
        """Stroke horizontal line at Y = value.

        If the value is out of the range of Y values, nothing will be
        drawn.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> y: list[float] = list(range(-5, 6))
            >>> Plot.stroke_line_at_y(canvas, -5.0, y)
            >>> Plot.stroke_line_at_y(canvas, -2.5, y)
            >>> Plot.stroke_line_at_y(canvas, 0.0, y)
            >>> Plot.stroke_line_at_y(canvas, 2.5, y)
            >>> Plot.stroke_line_at_y(canvas, 5.0, y)
            >>> print(canvas, end="")
            ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
            ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
            ⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
            ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
            ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
        """
        if (screen_y := Plot.compute_screen_y(canvas, value, y)) is None:
            return
        canvas.stroke_line(0, screen_y, canvas.w, screen_y)

    @staticmethod
    def compute_screen_x(
        canvas: TextCanvas, value: float, x: list[float]
    ) -> int | None:
        """Compute X position of a value on the canvas.

        Remember, values are auto-scaled to fit the canvas. If X goes
        from _-10_ to _10_, then:

        - Screen X of _-10_ will be 0
        - Screen X of _10_ will be canvas width
        - Screen X of _0_ will be canvas center X

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> x: list[float] = list(range(-10, 11))
            >>> assert 0 == Plot.compute_screen_x(canvas, -10.0, x)
            >>> assert 29 == Plot.compute_screen_x(canvas, 10.0, x)
            >>> assert 14 == Plot.compute_screen_x(canvas, 0.0, x)
        """
        if not x:
            return None

        min_x: float = min(x)
        max_x: float = max(x)
        range_x: float = max_x - min_x

        try:
            scale_x: float = canvas.w / range_x
        except ZeroDivisionError:
            return canvas.cx

        # Shift data left, so that `min_x` would = 0, then scale so
        # that `max_x` would = width.
        return int((value - min_x) * scale_x)

    @staticmethod
    def compute_screen_y(
        canvas: TextCanvas, value: float, y: list[float]
    ) -> int | None:
        """Compute Y position of a value on the canvas.

        Remember, values are auto-scaled to fit the canvas. If Y goes
        from _-10_ to _10_, then:

        - Screen X of _-10_ will be canvas height
        - Screen X of _10_ will be 0
        - Screen X of _0_ will be canvas center Y

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> y: list[float] = list(range(-10, 11))
            >>> assert 19 == Plot.compute_screen_y(canvas, -10.0, y)
            >>> assert 0 == Plot.compute_screen_y(canvas, 10.0, y)
            >>> assert 10 == Plot.compute_screen_y(canvas, 0.0, y)
        """
        if not y:
            return None

        min_y: float = min(y)
        max_y: float = max(y)
        range_y: float = max_y - min_y

        try:
            scale_y: float = canvas.h / range_y
        except ZeroDivisionError:
            return canvas.cy

        # Shift data down, so that `min_y` would = 0, then scale so
        # that `max_y` would = height.
        return canvas.h - int((value - min_y) * scale_y)  # Y-axis is inverted.

    @staticmethod
    def stroke_xy_axes_of_function(
        canvas: TextCanvas,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> None:
        """Stroke X and Y axes, given a function.

        The function is scaled to take up the entire canvas. The axes
        are then placed where _X_ and _Y_ = _0_;

        If 0 is not visible on an axis, the axis will not be drawn.

        Examples:
            >>> import math
            >>> canvas = TextCanvas(15, 5)
            >>> f = lambda x: math.sin(x)
            >>> Plot.stroke_xy_axes_of_function(canvas, -3.0, 7.0, f)
            >>> Plot.function(canvas, -3.0, 7.0, f)
            >>> print(canvas, end="")
            ⠀⠀⠀⠀⡇⢠⠋⠑⡄⠀⠀⠀⠀⠀⢀
            ⠀⠀⠀⠀⣇⠇⠀⠀⢱⠀⠀⠀⠀⠀⡎
            ⡤⠤⠤⠤⡿⠤⠤⠤⠤⡧⠤⠤⠤⡼⠤
            ⠸⡀⠀⢰⡇⠀⠀⠀⠀⠸⡀⠀⢠⠃⠀
            ⠀⠱⡠⠃⡇⠀⠀⠀⠀⠀⠑⠤⠊⠀⠀
        """
        # `stroke_(x|y)_axis_of_function()` methods would both compute
        # the values of `f()`. It is more efficient to compute these
        # values once, and use the regular `stroke_(x|y)_axis()`
        # methods instead.
        nb_values: int = canvas.screen.width
        (x, y) = Plot.compute_function(from_x, to_x, nb_values, f)
        Plot.stroke_x_axis(canvas, y)
        Plot.stroke_y_axis(canvas, x)

    @staticmethod
    def stroke_x_axis_of_function(
        canvas: TextCanvas,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> None:
        """Stroke X axis, given a function.

        See `stroke_xy_axes_of_function()` which has the same API for an
        example.
        """
        Plot.stroke_line_at_y_of_function(canvas, 0.0, from_x, to_x, f)

    @staticmethod
    def stroke_y_axis_of_function(
        canvas: TextCanvas,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> None:
        """Stroke Y axis, given a function.

        See `stroke_xy_axes_of_function()` which has the same API for an
        example.
        """
        Plot.stroke_line_at_x_of_function(canvas, 0.0, from_x, to_x, f)

    @staticmethod
    def stroke_line_at_x_of_function(
        canvas: TextCanvas,
        value: float,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> None:
        """Stroke vertical line at X = value, given a function.

        Same as `stroke_line_at_x()`, but for a function.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> f = lambda x: x
            >>> Plot.stroke_line_at_x_of_function(canvas, -5.0, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_x_of_function(canvas, -2.5, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_x_of_function(canvas, 0.0, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_x_of_function(canvas, 2.5, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_x_of_function(canvas, 5.0, -5.0, 5.0, f)
            >>> print(canvas, end="")
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
            ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
        """
        nb_values: int = canvas.screen.width
        (x, _) = Plot.compute_function(from_x, to_x, nb_values, f)
        Plot.stroke_line_at_x(canvas, value, x)

    @staticmethod
    def stroke_line_at_y_of_function(
        canvas: TextCanvas,
        value: float,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> None:
        """Stroke horizontal line at Y = value, given a function.

        Same as `stroke_line_at_y()`, but for a function.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> f = lambda x: x
            >>> Plot.stroke_line_at_y_of_function(canvas, -5.0, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_y_of_function(canvas, -2.5, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_y_of_function(canvas, 0.0, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_y_of_function(canvas, 2.5, -5.0, 5.0, f)
            >>> Plot.stroke_line_at_y_of_function(canvas, 5.0, -5.0, 5.0, f)
            >>> print(canvas, end="")
            ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
            ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
            ⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
            ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
            ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
        """
        nb_values: int = canvas.screen.width
        (_, y) = Plot.compute_function(from_x, to_x, nb_values, f)
        Plot.stroke_line_at_y(canvas, value, y)

    @staticmethod
    def compute_screen_x_of_function(
        canvas: TextCanvas,
        value: float,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> int | None:
        """Compute X position of a value on the canvas, given a
        function.

        Same as `compute_screen_x()`, but for a function.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> f = lambda x: x
            >>> assert 0 == Plot.compute_screen_x_of_function(canvas, -10.0, -10.0, 10.0, f)
            >>> assert 14 == Plot.compute_screen_x_of_function(canvas, 0.0, -10.0, 10.0, f)
            >>> assert 29 == Plot.compute_screen_x_of_function(canvas, 10.0, -10.0, 10.0, f)
        """
        nb_values: int = canvas.screen.width
        (x, _) = Plot.compute_function(from_x, to_x, nb_values, f)
        return Plot.compute_screen_x(canvas, value, x)

    @staticmethod
    def compute_screen_y_of_function(
        canvas: TextCanvas,
        value: float,
        from_x: float,
        to_x: float,
        f: Callable[[float], float],
    ) -> int | None:
        """Compute Y position of a value on the canvas, given a
        function.

        Same as `compute_screen_y()`, but for a function.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> f = lambda x: x
            >>> assert 19 == Plot.compute_screen_y_of_function(canvas, -10.0, -10.0, 10.0, f)
            >>> assert 10 == Plot.compute_screen_y_of_function(canvas, 0.0, -10.0, 10.0, f)
            >>> assert 0 == Plot.compute_screen_y_of_function(canvas, 10.0, -10.0, 10.0, f)
        """
        nb_values: int = canvas.screen.width
        (_, y) = Plot.compute_function(from_x, to_x, nb_values, f)
        return Plot.compute_screen_y(canvas, value, y)

    @staticmethod
    def line(canvas: TextCanvas, x: list[float], y: list[float]) -> None:
        """Plot line-joined points.

        The data is scaled to take up the entire canvas.

        <div class="warning">

        `x` and `y` _should_ match in length,

        If `x` and `y` are not the same length, plotting will stop once
        the smallest of the two collections is consumed.

        </div>

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> x: list[float] = list(range(-5, 6))
            >>> y: list[float] = list(range(-5, 6))
            >>> Plot.line(canvas, x, y)
            >>> print(canvas, end="")
            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠒⠉
            ⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀
            ⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀
            ⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
            ⡠⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        """
        Plot._plot(canvas, x, y, PlotType.LINE)

    @staticmethod
    def scatter(canvas: TextCanvas, x: list[float], y: list[float]) -> None:
        """Plot scattered points.

        The data is scaled to take up the entire canvas.

        <div class="warning">

        `x` and `y` _should_ match in length,

        If `x` and `y` are not the same length, plotting will stop once
        the smallest of the two collections is consumed.

        </div>

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> x: list[float] = list(range(-5, 6))
            >>> y: list[float] = list(range(-5, 6))
            >>> Plot.scatter(canvas, x, y)
            >>> print(canvas, end="")
            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠂⠈
            ⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠂⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⢀⠀⠂⠀⠀⠀⠀⠀⠀⠀
            ⠀⠀⢀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
            ⡀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        """
        Plot._plot(canvas, x, y, PlotType.SCATTER)

    @staticmethod
    def _plot(
        canvas: TextCanvas,
        x_vals: list[float],
        y_vals: list[float],
        plot_type: PlotType,
    ) -> None:
        if not x_vals or not y_vals:
            return

        pairs: list[tuple[float, float]] = list(zip(x_vals, y_vals))
        if plot_type == PlotType.LINE:
            # Sort by `x`
            pairs.sort(key=lambda pair: pair[0])

        min_x: float = min(x_vals)
        max_x: float = max(x_vals)
        range_x: float = max_x - min_x

        scale_x_is_infinite: bool = False
        try:
            scale_x: float = canvas.w / range_x
        except ZeroDivisionError:
            scale_x_is_infinite = True
            scale_x = float("+inf")

        min_y: float = min(y_vals)
        max_y: float = max(y_vals)
        range_y: float = max_y - min_y

        scale_y_is_infinite: bool = False
        try:
            scale_y: float = canvas.h / range_y
        except ZeroDivisionError:
            scale_y_is_infinite = True
            scale_y = float("+inf")

        if scale_x_is_infinite or scale_y_is_infinite:
            # One or both axis have no range. This doesn't make sense
            # for plotting with auto-scale.
            return Plot._handle_axes_without_range(
                canvas,
                x_vals,
                y_vals,
                plot_type,
                scale_x_is_infinite,
                scale_y_is_infinite,
            )

        previous: tuple[int, int] | None = None  # For line plot.
        for x, y in pairs:
            # Shift data left so that `min_x` = 0, then scale so that
            # `max_x` = width.
            x = (x - min_x) * scale_x
            x = int(x)

            y = (y - min_y) * scale_y
            y = canvas.h - y  # Y-axis is inverted.
            y = int(y)

            match plot_type:
                case PlotType.LINE:
                    pair = (x, y)
                    if previous is not None:
                        canvas.stroke_line(previous[0], previous[1], pair[0], pair[1])
                    previous = pair
                case PlotType.SCATTER:
                    canvas.set_pixel(x, y, True)

    @staticmethod
    def _handle_axes_without_range(
        canvas: TextCanvas,
        x_vals: list[float],
        y_vals: list[float],
        plot_type: PlotType,
        x_has_no_range: bool,
        y_has_no_range: bool,
    ) -> None:
        x_has_range_but_not_y: bool = not x_has_no_range and y_has_no_range
        y_has_range_but_not_x: bool = x_has_no_range and not y_has_no_range
        both_have_no_range: bool = x_has_no_range and y_has_no_range

        if x_has_range_but_not_y:
            # Y is a constant, draw a single centered line.
            Plot._draw_horizontally_centered_line(canvas, x_vals, plot_type)
        elif y_has_range_but_not_x:
            # Compress all Ys into a single centered line.
            Plot._draw_vertically_centered_line(canvas, y_vals, plot_type)
        elif both_have_no_range:
            # Draw a dot in the middle to show the user we tried to do
            # something, but the values are off.
            canvas.set_pixel(canvas.cx, canvas.cy, True)

    @staticmethod
    def _draw_horizontally_centered_line(
        canvas: TextCanvas, x_vals: list[float], plot_type: PlotType
    ) -> None:
        match plot_type:
            case PlotType.LINE:
                canvas.stroke_line(0, canvas.cy, canvas.w, canvas.cy)
            case PlotType.SCATTER:
                for x_val in x_vals:
                    if (x := Plot.compute_screen_x(canvas, x_val, x_vals)) is not None:
                        canvas.set_pixel(x, canvas.cy, True)

    @staticmethod
    def _draw_vertically_centered_line(
        canvas: TextCanvas, y_vals: list[float], plot_type: PlotType
    ) -> None:
        match plot_type:
            case PlotType.LINE:
                canvas.stroke_line(canvas.cx, 0, canvas.cx, canvas.h)
            case PlotType.SCATTER:
                for y_val in y_vals:
                    if (y := Plot.compute_screen_y(canvas, y_val, y_vals)) is not None:
                        canvas.set_pixel(canvas.cx, y, True)

    @staticmethod
    def function(
        canvas: TextCanvas, from_x: float, to_x: float, f: Callable[[float], float]
    ) -> None:
        """Plot a function.

        The function is scaled to take up the entire canvas, and is
        assumed to be continuous (points will be line-joined together).

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> Plot.function(canvas, -10.0, 10.0, lambda x: x ** 2)
            >>> print(canvas, end="")
            ⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜
            ⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀
            ⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⡔⠁⠀
            ⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀
            ⠀⠀⠀⠀⠈⠒⠤⣀⠤⠒⠁⠀⠀⠀⠀
        """
        nb_values: int = canvas.screen.width
        (x, y) = Plot.compute_function(from_x, to_x, nb_values, f)
        Plot.line(canvas, x, y)

    @staticmethod
    def compute_function(
        from_x: float,
        to_x: float,
        nb_values: float,
        f: Callable[[float], float],
    ) -> tuple[list[float], list[float]]:
        """Compute the values of a function.

        This is mainly used internally to compute values for functions.

        However, it may also be useful in case one wants to pre-compute
        values.

        Examples:
            >>> import math
            >>> canvas = TextCanvas(15, 5)
            >>> canvas2 = TextCanvas(15, 5)
            >>> f = lambda x: math.sin(x)
            >>> # This is inefficient, because `f()` will be computed twice.
            >>> Plot.stroke_xy_axes_of_function(canvas, -3.0, 7.0, f)
            >>> Plot.function(canvas, -3.0, 7.0, f)
            >>> # This is better, the values are computed only once.
            >>> (x, y) = Plot.compute_function(-3.0, 7.0, canvas2.screen.width, f)
            >>> Plot.stroke_xy_axes(canvas2, x, y)
            >>> Plot.line(canvas2, x, y)
            >>> assert canvas.to_string() == canvas2.to_string()

        Note that the "inefficient" solution is unlikely to cause a
        noticeable performance hit. The simpler approach is most often
        the better approach.
        """
        range: float = to_x - from_x
        # If we want 5 values in a range including bounds, we need to
        # divide the range into 4 equal pieces:
        #   1   2   3   4
        # |   |   |   |   |
        # 1   2   3   4   5
        step: float = range / (nb_values - 1)

        px: list[float] = []
        py: list[float] = []

        # Always add first value.
        px.append(from_x)
        py.append(f(from_x))

        x = from_x + step
        while x < to_x:
            px.append(x)
            py.append(f(x))
            x += step

        # Always add last value.
        px.append(to_x)
        py.append(f(to_x))

        return (px, py)
