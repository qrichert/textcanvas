"""Graph of sin(x) and cos(x).

```shell-session
$ python -m examples.graph
```
"""

import math
from typing import Callable

from textcanvas.textcanvas import Color, TextCanvas

canvas: TextCanvas = TextCanvas(80, 24)

width: int = canvas.screen.width
height: int = canvas.screen.height

half_screen_x: int = width // 2
half_screen_y: int = height // 2

scale_x: float = half_screen_x / ((math.pi * 2) * 1.0065)
scale_y: float = 0.618


def draw_x_and_y_axes() -> None:
    half_output_x: int = canvas.output.width // 2
    half_output_y: int = canvas.output.height // 2

    canvas.stroke_line(0, half_screen_y, width, half_screen_y)
    canvas.stroke_line(half_screen_x, 0, half_screen_x, height)

    canvas.draw_text("-π", half_output_x - int(scale_x * math.pi), half_output_y + 1)
    canvas.draw_text(
        "-π/2", half_output_x - int(scale_x * math.pi / 2) - 1, half_output_y + 1
    )
    canvas.draw_text("0", half_output_x - 2, half_output_y + 1)
    canvas.draw_text(
        "π/2", half_output_x + int(scale_x * math.pi / 2) - 1, half_output_y + 1
    )
    canvas.draw_text("π", half_output_x + int(scale_x * math.pi) - 1, half_output_y + 1)

    canvas.draw_text(
        "1", half_output_x - 2, half_output_y - 1 - int(scale_y * half_output_y)
    )
    canvas.draw_text(
        "-1", half_output_x - 3, half_output_y + int(scale_y * half_output_y)
    )


def graph_function(func: Callable) -> None:
    x_start: int = -half_screen_x
    x_end: int = half_screen_x

    graph_x_to_screen = lambda x: x + half_screen_x  # noqa: E731
    graph_y_to_screen = lambda y: height - 1 - int((y + 1) * half_screen_y)  # noqa: E731

    prev_x: int | None = None
    prev_y: int | None = None
    for x in range(x_start, x_end):
        y: float = func(x * scale_x) * scale_y

        screen_x: int = graph_x_to_screen(x)
        screen_y: int = graph_y_to_screen(y)

        if prev_x and prev_y:
            canvas.stroke_line(prev_x, prev_y, screen_x, screen_y)
        prev_x = screen_x
        prev_y = screen_y


draw_x_and_y_axes()

canvas.set_color(Color().bold().bright_blue())
graph_function(math.cos)

canvas.set_color(Color().bold().bright_red())
graph_function(math.sin)

print(canvas)
