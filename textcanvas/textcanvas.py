"""TextCanvas.

TextCanvas is JavaScript Canvas-like surface that can be used to draw to
the console. Other use cases include visual checks of mathematical
computations (i.e. does the graph at least look correct?), or even
snapshot testing (may not be the most accurate, but can have great
documentation value).

It is inspired by drawille[^1], which uses Braille Unicode characters to
increase the resolution of the console by a factor of 8 (8 Braille dots
in one console character).

The API is inspired by JavaScript's Canvas, but has almost no features.

How It Works:

Braille characters start at Unicode offset `U2800` (hexadecimal), and
work by addition (binary flags really, just like chmod):

    2800 + (1 + 2) = 2803 <=> U2801 (⠁) + U2802 (⠂) = U2803 (⠃)

    2800 + (3 + 4) = 2807 <=> U2803 (⠃) + U2804 (⠄) = U2807 (⠇)

One character is 8 pixels, and we individually turn pixels on or off by
adding or subtracting the value of the dot we want.

Each dot has its value (again, this is hexadecimal):

    ┌──────┐  ┌────────────┐
    │ •  • │  │  0x1   0x8 │
    │ •  • │  │  0x2  0x10 │
    │ •  • │  │  0x4  0x20 │
    │ •  • │  │ 0x40  0x80 │
    └──────┘  └────────────┘

For example, to turn off the right pixel from the second row:

    0x28FF (⣿) - 0x10 (⠐) = 0x28ef (⣯)

Or the whole second row:

    0x28FF (⣿) - 0x12 (⠒) = 0x28ed (⣭)

This works in binary as well :

    ┌──────┐  ┌──────┐
    │ •  • │  │ 1  4 │
    │ •  • │  │ 2  5 │
    │ •  • │  │ 3  6 │
    │ •  • │  │ 7  8 │
    └──────┘  └──────┘

These numbers define how dots are mapped to a bit array (ordering is
historical, 7 and 8 were added later):

    Bits: 0 0 0 0 0 0 0 0
    Dots: 8 7 6 5 4 3 2 1

For example, to turn on the first two rows, we would activate bit 1, 2,
4, and 5:

    0 0 0 1 1 0 1 1

    Note that: 0b11011 = 0x1b = 0x1 + 0x2 + 0x8 + 0x10

Carrying on with this example, we could turn off the first row and turn
on the last row like so:

    Current pattern:  00011011
    First row (1, 4): 00001001
    Last row (7, 8):  11000000

    0b11011 - 0b1001 + 0b11000000 = 0b11010010
      0x1b  -   0x9  +    0xc0    =    0xd2

    0x2800 + 0b11010010 = 0x28d2 (⣒)

See Also:

- https://en.wikipedia.org/wiki/Braille_Patterns
- https://www.unicode.org/charts/PDF/U2800.pdf

[^1]: https://github.com/asciimoo/drawille
"""

from dataclasses import dataclass
from typing import Generator

from .color import Color

type PixelBuffer = list[list[bool]]
type ColorBuffer = list[list[str]]
type BrailleChar = int
type PixelBlock = tuple[
    tuple[bool, bool],
    tuple[bool, bool],
    tuple[bool, bool],
    tuple[bool, bool],
]
type BrailleMap = tuple[
    tuple[int, int],
    tuple[int, int],
    tuple[int, int],
    tuple[int, int],
]

ON: bool = True
OFF: bool = False

BRAILLE_UNICODE_0: int = 0x2800
BRAILLE_UNICODE_OFFSET_MAP: BrailleMap = (
    (0x1, 0x8),
    (0x2, 0x10),
    (0x4, 0x20),
    (0x40, 0x80),
)


@dataclass
class Surface:
    width: int
    height: int


# TODO:
#  move_to(), line_to(), stroke()
#  canvas.x,xw,cx, y,yh,cy
#  draw text (text buffer on top?)
class TextCanvas:
    def __init__(self, width: int = 80, height: int = 24) -> None:
        self._validate_size(width, height)

        self.output: Surface = Surface(width, height)
        self.screen: Surface = Surface(width * 2, height * 4)
        self.buffer: PixelBuffer
        self.color_buffer: ColorBuffer = []

        self._color: Color = Color.NO_COLOR

        self.clear()

    def _validate_size(self, width: int, height: int) -> None:
        if width <= 0 or height <= 0:
            raise ValueError("TextCanvas' minimal size is 1×1.")

    def __repr__(self) -> str:
        out_w: int = self.output.width
        out_h: int = self.output.height
        screen_w: int = self.screen.width
        screen_h: int = self.screen.height
        return f"Canvas(output=({out_w}×{out_h}), screen=({screen_w}×{screen_h})))"

    def __str__(self) -> str:
        return self.to_string()

    def clear(self) -> None:
        """
        TODO: This is NOT the same buffer anymore, old references will be
         invalid.
        """
        if hasattr(self, "buffer"):
            for x, y in self.iter_buffer():
                self.buffer[y][x] = False
        else:
            self.buffer = [
                [OFF for _ in range(self.screen.width)]
                for _ in range(self.screen.height)
            ]

    @property
    def is_colorized(self) -> bool:
        return bool(self.color_buffer)

    def set_color(self, color: Color) -> None:
        if not self.is_colorized:
            self.color_buffer = [
                [Color.NO_COLOR for _ in range(self.output.width)]
                for _ in range(self.output.height)
            ]
        self._color = color

    def get_pixel(self, x: int, y: int) -> bool | None:
        if not 0 <= x < self.screen.width or not 0 <= y < self.screen.height:
            return None
        return self.buffer[y][x]

    def set_pixel(self, x: int, y: int, state: bool) -> None:
        # TODO[doc]: In screen coordinates, part of public API.

        if not 0 <= x < self.screen.width or not 0 <= y < self.screen.height:
            return

        self.buffer[y][x] = state

        if self.is_colorized:
            if state is True:
                self._color_pixel(x, y)
            else:
                self._decolor_pixel(x, y)

    def _color_pixel(self, x: int, y: int) -> None:
        self.color_buffer[y // 4][x // 2] = self._color

    def _decolor_pixel(self, x: int, y: int) -> None:
        self.color_buffer[y // 4][x // 2] = Color.NO_COLOR

    def to_string(self) -> str:
        res: str = ""
        for i, pixel_block in enumerate(self._iter_buffer_by_blocks_lrtb()):
            output_char: BrailleChar = BRAILLE_UNICODE_0
            # Iterate over individual pixels to turn them on or off.
            for row in range(len(pixel_block)):
                for col in range(len(pixel_block[row])):
                    if pixel_block[row][col] is ON:
                        output_char += BRAILLE_UNICODE_OFFSET_MAP[row][col]
            # Convert Unicode value to string and append to output.
            final_char: str = chr(output_char)
            if self.is_colorized:
                # TODO: cleanup
                final_char = self.color_buffer[i // self.output.width][
                    i % self.output.width
                ].format(final_char)
            res += final_char
            # If end of line is reached, go to next line.
            if (i + 1) % self.output.width == 0:
                res += "\n"
        return res

    def _iter_buffer_by_blocks_lrtb(self) -> Generator[PixelBlock, None, None]:
        """Advance block by block (2x4), left-right, top-bottom."""
        for row in range(0, self.screen.height, 4):
            for col in range(0, self.screen.width, 2):
                yield (
                    (self.buffer[row + 0][col + 0], self.buffer[row + 0][col + 1]),
                    (self.buffer[row + 1][col + 0], self.buffer[row + 1][col + 1]),
                    (self.buffer[row + 2][col + 0], self.buffer[row + 2][col + 1]),
                    (self.buffer[row + 3][col + 0], self.buffer[row + 3][col + 1]),
                )

    def iter_buffer(self) -> Generator[tuple[int, int], None, None]:
        for y in range(self.screen.height):
            for x in range(self.screen.width):
                yield (x, y)

    def stroke_line(self, x1: int, y1: int, x2: int, y2: int) -> None:
        """Stroke line using Bresenham's line algorithm."""

        dx = abs(x2 - x1)
        sx = 1 if x1 < x2 else -1
        dy = -abs(y2 - y1)
        sy = 1 if y1 < y2 else -1
        error = dx + dy

        # Treat vertical and horizontal lines as special cases.
        if dx == 0:
            x = x1
            from_y = min(y1, y2)
            to_y = max(y1, y2)
            for y in range(from_y, to_y + 1):
                self.set_pixel(x, y, True)
            return
        elif dy == 0:
            y = y1
            from_x = min(x1, x2)
            to_x = max(x1, x2)
            for x in range(from_x, to_x + 1):
                self.set_pixel(x, y, True)
            return

        while True:
            self.set_pixel(x1, y1, True)
            if x1 == x2 and y1 == y2:
                break
            e2 = 2 * error
            if e2 >= dy:
                if x1 == x2:
                    break  # pragma: no cover
                error = error + dy
                x1 = x1 + sx
            if e2 <= dx:
                if y1 == y2:
                    break  # pragma: no cover
                error = error + dx
                y1 = y1 + sy


if __name__ == "__main__":
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

    canvas.set_color(Color.RED)
    canvas.stroke_line(*center, *top_left)
    canvas.set_color(Color.YELLOW)
    canvas.stroke_line(*center, *top_right)
    canvas.set_color(Color.GREEN)
    canvas.stroke_line(*center, *bottom_right)
    canvas.set_color(Color.BLUE)
    canvas.stroke_line(*center, *bottom_left)
    canvas.set_color(Color.CYAN)
    canvas.stroke_line(*center, *center_top)
    canvas.set_color(Color.MAGENTA)
    canvas.stroke_line(*center, *center_right)
    canvas.set_color(Color.GRAY)
    canvas.stroke_line(*center, *center_bottom)
    canvas.set_color(Color.NO_COLOR)
    canvas.stroke_line(*center, *center_left)

    print(canvas)

    # TODO: Examples + use canvas.move_to(), line_to
    canvas = TextCanvas()
    canvas.stroke_line(
        canvas.screen.width // 2, 0, canvas.screen.width // 2, canvas.screen.height - 1
    )
    canvas.stroke_line(
        0, canvas.screen.height // 2, canvas.screen.width - 1, canvas.screen.height // 2
    )
    canvas.set_color(Color.BOLD_RED)
    import math

    x_offset = canvas.screen.width // 2 - 1.45
    y_offset = canvas.screen.height // 2
    x_scale = 17
    y_scale = 23
    prevx = None
    prevy = None
    for x in range(canvas.screen.width):
        y = math.sin(x / x_scale + x_offset) * y_scale + y_offset
        x = int(x)
        y = canvas.screen.height - 1 - int(y)
        if prevx and prevy:
            canvas.stroke_line(prevx, prevy, x, y)
        prevx = x
        prevy = y
    prevx = None
    prevy = None
    canvas.set_color(Color.BOLD_BLUE)
    for x in range(canvas.screen.width):
        y = math.cos(x / x_scale + x_offset) * y_scale + y_offset
        x = int(x)
        y = canvas.screen.height - 1 - int(y)
        if prevx and prevy:
            canvas.stroke_line(prevx, prevy, x, y)
        prevx = x
        prevy = y
    print(canvas)
