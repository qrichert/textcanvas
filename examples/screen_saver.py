import dataclasses
import datetime as dt
from typing import Self

from textcanvas.textcanvas import Color, TextCanvas
from textcanvas.utils import GameLoop

VELOCITY: int = 7
TRAIL_LENGTH: int = 8


@dataclasses.dataclass
class Point:
    x: int
    y: int
    vx: int
    vy: int

    def clone(self) -> Self:
        return dataclasses.replace(self)


type Line = tuple[Point, Point]

canvas = TextCanvas(80, 24)
p1 = Point(0, 0, VELOCITY, VELOCITY)
p2 = Point(canvas.w, canvas.h, VELOCITY, VELOCITY)


lines: list[Line] = []

i: int = 0


def loop() -> str | None:
    global i, p1, p2

    def update_point(p: Point) -> None:
        p.x += p.vx
        p.y += p.vy
        if not 0 <= p.x <= canvas.w:
            p.vx = -p.vx
        if not 0 <= p.y <= canvas.h:
            p.vy = -p.vy

    i += 1
    if i == 1000:
        return None

    update_point(p1)
    update_point(p2)

    lines.append((p1.clone(), p2.clone()))
    if len(lines) > TRAIL_LENGTH:
        lines.pop(0)

    canvas.clear()

    for j, (p1, p2) in enumerate(lines):
        if j == len(lines) - 1:  # Most recent line.
            canvas.set_color(Color().x_royal_blue_1())
        elif j < 3:  # Oldest 3 lines.
            canvas.set_color(Color().x_sky_blue_3())
        else:  # Any line in-between.
            canvas.set_color(Color().x_dodger_blue_1())

        canvas.stroke_line(p1.x, p1.y, p2.x, p2.y)

    return canvas.to_string()


GameLoop.loop_fixed(dt.timedelta(seconds=1 / 14), loop)
