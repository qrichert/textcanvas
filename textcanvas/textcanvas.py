"""TextCanvas.

TextCanvas is an HTML Canvas-like surface that can be used to draw to
the terminal. Other use cases include visual checks for mathematical
computations (i.e. does the graph at least look correct?), or snapshot
testing (may not be the most accurate, but can have great documentation
value).

It is inspired by drawille[^1], which uses Braille Unicode characters to
increase the resolution of the terminal by a factor of 8 (8 Braille dots
in one terminal character).

The API is inspired by JavaScript Canvas's API, but has barely any
features.

# How It Works

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

This works in binary as well:

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

For example, to turn on the first two rows, we would activate bit 1, 4,
2, and 5:

    0 0 0 1 1 0 1 1

    Note that: 0b11011 = 0x1b = 0x1 + 0x8 + 0x2 + 0x10 (see hex chart)

Carrying on with this example, we could turn off the first row and turn
on the last row like so:

    Current pattern:  00011011
    First row (1, 4): 00001001
    Last row (7, 8):  11000000

    0b11011 - 0b1001 + 0b11000000 = 0b11010010
      0x1b  -   0x9  +    0xc0    =    0xd2

    0x2800 + 0b11010010 = 0x28d2 (⣒)

# See Also

- https://en.wikipedia.org/wiki/Braille_Patterns
- https://www.unicode.org/charts/PDF/U2800.pdf

[^1]: https://github.com/asciimoo/drawille
"""

from dataclasses import dataclass
from typing import Generator

from .color import Color

type PixelBuffer = list[list[bool]]
type ColorBuffer = list[list[Color]]
type TextBuffer = list[list[str]]
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


class TextCanvas:
    """Draw to the terminal like an HTML Canvas.

    Examples:
        >>> canvas = TextCanvas(15, 5)
        >>> repr(canvas)
        'Canvas(output=(15×5), screen=(30×20)))'
        >>> canvas.w, canvas.h, canvas.cx, canvas.cy
        (29, 19, 15, 10)
        >>> canvas.stroke_line(0, 0, canvas.w, canvas.h)
        >>> canvas.draw_text(1, 2, "hello, world")
        >>> print(canvas, end="")
        ⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀hello,⠢world⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄

    Attributes:
        output (Surface): Properties of the output surface, whose size
            is given as parameter to the constructor. One unit in width
            and in height represents exactly one character on the
            terminal.
        screen (Surface): Properties of the virtual output surface. This
            surface benefits from the increase in resolution. One unit
            in width and in height represents one Braille dot. There are
            2 dots per character in width, and 4 per character in
            height. So this surface is 2× wider and 4× higher than the
            output surface.
        buffer (PixelBuffer): The in-memory pixel buffer. This maps
            1-to-1 to the virtual screen. Each pixel is this buffer is
            either _on_ or _off_.
        color_buffer (ColorBuffer): The in-memory color buffer. This
            maps 1-to-1 to the output buffer (and so to the physical
            screen). This contains color data for characters. Screen
            dots, being part of one output character, cannot be colored
            individually. Color is thus less precise than screen pixels.
            Note that a call to `set_color()` is valid for both pixels
            and text, but text embeds color in its own buffer, and does
            not use this buffer at all. Note also that this buffer is
            empty until the first call to `set_color()`. The first call
            to `set_color()` initializes the buffer and sets
            `is_colorized` to `True`.
        text_buffer (TextBuffer): The in-memory text buffer. This maps
            1-to-1 to the output buffer (and so to the physical screen).
            This contains regular text characters. Text is drawn on top
            of the pixel buffer on a separate layer. Drawing text does
            not affect pixels. Pixels and text do not share the same
            color buffer either. Color info is embedded in the text
            buffer with each character directly. Note also that this
            buffer is empty until the first call to `draw_text()`. The
            first call to `draw_text()` initializes the buffer and sets
            `is_textual` to `True`.

    Raises:
        ValueError: If width and height of canvas are < 1×1.

    Todo:
        - `move_to()`, `line_to()`, `stroke()` and other JS Canvas
          primitives.
    """

    def __init__(self, width: int = 80, height: int = 24) -> None:
        self._validate_size(width, height)

        self.output: Surface = Surface(width, height)
        self.screen: Surface = Surface(width * 2, height * 4)
        self.buffer: PixelBuffer
        self.color_buffer: ColorBuffer = []
        self.text_buffer: TextBuffer = []

        self._color: Color = Color()

        self._init_buffer()

    @staticmethod
    def _validate_size(width: int, height: int) -> None:
        if width <= 0 or height <= 0:
            raise ValueError("TextCanvas' minimal size is 1×1.")

    def _init_buffer(self) -> None:
        self.buffer = [
            [OFF for _ in range(self.screen.width)] for _ in range(self.screen.height)
        ]

    def __repr__(self) -> str:
        out_w: int = self.output.width
        out_h: int = self.output.height
        screen_w: int = self.screen.width
        screen_h: int = self.screen.height
        return f"Canvas(output=({out_w}×{out_h}), screen=({screen_w}×{screen_h})))"

    def __str__(self) -> str:
        return self.to_string()

    @property
    def w(self) -> int:
        """Shortcut for width of pixel screen (index of last column)."""
        return self.screen.width - 1

    @property
    def h(self) -> int:
        """Shortcut for height of pixel screen (index of last row)."""
        return self.screen.height - 1

    @property
    def cx(self) -> int:
        """Shortcut for center-X of pixel screen."""
        return self.screen.width // 2

    @property
    def cy(self) -> int:
        """Shortcut for center-Y of pixel screen."""
        return self.screen.height // 2

    def clear(self) -> None:
        """Turn all pixels off and remove color and text.

        Note:
            This method does not drop the color and text buffers, it
            only clears them. No memory is freed, and all references
            remain valid (buffers are cleared in-place, not replaced).
        """
        self._clear_buffer()
        self._clear_color_buffer()
        self._clear_text_buffer()

    def _clear_buffer(self) -> None:
        for x, y in self.iter_buffer():
            self.buffer[y][x] = False

    def _clear_color_buffer(self) -> None:
        if self.color_buffer:
            for y, _ in enumerate(self.color_buffer):
                for x, _ in enumerate(self.color_buffer[y]):
                    self.color_buffer[y][x] = Color()

    def _clear_text_buffer(self) -> None:
        if self.text_buffer:
            for y, _ in enumerate(self.text_buffer):
                for x, _ in enumerate(self.text_buffer[y]):
                    self.text_buffer[y][x] = ""

    @property
    def is_colorized(self) -> bool:
        """Whether the canvas can contain colors.

        Note:
            This does not mean that any colors are displayed. This only
            means the color buffer is active.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> canvas.is_colorized
            False
            >>> canvas.set_color(Color())  # Buffer is initialized.
            >>> canvas.is_colorized
            True
        """
        return bool(self.color_buffer)

    @property
    def is_textual(self) -> bool:
        """Whether the canvas can contain text.

        Note:
            This does not mean that any text is displayed. This only
            means the text buffer is active.

        Examples:
            >>> canvas = TextCanvas(15, 5)
            >>> canvas.is_textual
            False
            >>> canvas.draw_text(0, 0, "")  # Buffer is initialized.
            >>> canvas.is_textual
            True
        """
        return bool(self.text_buffer)

    def set_color(self, color: Color) -> None:
        """Set context color.

        Examples:
            >>> canvas = TextCanvas(3, 1)
            >>> canvas.set_color(Color().bright_green())
            >>> canvas.is_colorized
            True
            >>> canvas.draw_text(0, 0, "foo")
            >>> print(canvas, end="")
            \x1b[0;92mf\x1b[0m\x1b[0;92mo\x1b[0m\x1b[0;92mo\x1b[0m
        """
        if not self.is_colorized:
            self._init_color_buffer()
        self._color = color

    def _init_color_buffer(self) -> None:
        self.color_buffer = [
            [Color() for _ in range(self.output.width)]
            for _ in range(self.output.height)
        ]

    def get_pixel(self, x: int, y: int) -> bool | None:
        """Get the state of a screen pixel.

        Args:
            x (int): Screen X (high resolution).
            y (int): Screen Y (high resolution).

        Returns:
            `True` if the pixel is turned _on_, `False` if it is turned
            _off_, and `None` if the coordinates are outside the bounds
            of the buffer.
        """
        if not 0 <= x < self.screen.width or not 0 <= y < self.screen.height:
            return None
        return self.buffer[y][x]

    def set_pixel(self, x: int, y: int, state: bool) -> None:
        """Set the state of a screen pixel.

        Note:
            Coordinates outside the screen bounds are ignored.

        Note:
            Turning a pixel _off_ also removes color. This side effect
            does not affect text, as text has a separate color buffer.

        Args:
            x (int): Screen X (high resolution).
            y (int): Screen Y (high resolution).
            state (bool): `True` means _on_, `False` means _off_.
        """
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
        self.color_buffer[y // 4][x // 2] = Color()

    def draw_text(self, x: int, y: int, text: str) -> None:
        """Draw text onto the canvas.

        Note:
            Coordinates outside the screen bounds are ignored.

        Note:
            Text is rendered on top of pixels, as a separate layer.

        Note:
            `set_color()` works for text as well, but text does not
            share its color buffer with pixels. Rather, color data is
            stored with the text characters themselves.

        Args:
            x (int): Output X (true resolution).
            y (int): Output Y (true resolution).
            text (str): The text to draw. Spaces are transparent (you
                see pixels through), and can be used to erase previously
                drawn characters.
        """
        if not self.is_textual:
            self._init_text_buffer()
        for char in text:
            if not 0 <= x < self.output.width or not 0 <= y < self.output.height:
                x += 1
                continue
            if char == " ":
                char = ""
            self.text_buffer[y][x] = self._color.format(char)
            x += 1

    def _init_text_buffer(self) -> None:
        self.text_buffer = [
            ["" for _ in range(self.output.width)] for _ in range(self.output.height)
        ]

    def to_string(self) -> str:
        """Render canvas as a `print()`-able string.

        Note:
            This is used by `str()`, and so is also what's printed to
            the screen by a call to `print(canvas)`.

        Returns:
            Rendered canvas, with pixels, text and colors. Each canvas
            row becomes a line of text (lines are separated by `\n`s),
            and each canvas column becomes a single character in each
            line. What you would expect. It can be printed as-is.
        """
        res: str = ""
        for i, pixel_block in enumerate(self._iter_buffer_by_blocks_lrtb()):
            x: int = i % self.output.width
            y: int = i // self.output.width

            # Text layer.
            if (text_char := self._get_text_char(x, y)) != "":
                res += text_char
            # Pixel layer.
            else:
                braille_char: str = self._pixel_block_to_braille_char(pixel_block)
                res += self._color_pixel_char(x, y, braille_char)

            # If end of line is reached, go to next line.
            if (i + 1) % self.output.width == 0:
                res += "\n"
        return res

    def _get_text_char(self, x: int, y: int) -> str:
        if self.is_textual:
            return self.text_buffer[y][x]
        return ""

    @staticmethod
    def _pixel_block_to_braille_char(pixel_block: PixelBlock) -> str:
        braille_char: BrailleChar = BRAILLE_UNICODE_0
        # Iterate over individual pixels to turn them on or off.
        for y, _ in enumerate(pixel_block):
            for x, _ in enumerate(pixel_block[y]):
                if pixel_block[y][x] is ON:
                    braille_char += BRAILLE_UNICODE_OFFSET_MAP[y][x]
        # Convert Unicode integer value to string.
        return chr(braille_char)

    def _color_pixel_char(self, x: int, y: int, pixel_char: str) -> str:
        if self.is_colorized:
            color: Color = self.color_buffer[y][x]
            return color.format(pixel_char)
        return pixel_char

    def _iter_buffer_by_blocks_lrtb(self) -> Generator[PixelBlock, None, None]:
        """Advance block by block (2x4), left-right, top-bottom."""
        for y in range(0, self.screen.height, 4):
            for x in range(0, self.screen.width, 2):
                yield (
                    (self.buffer[y + 0][x + 0], self.buffer[y + 0][x + 1]),
                    (self.buffer[y + 1][x + 0], self.buffer[y + 1][x + 1]),
                    (self.buffer[y + 2][x + 0], self.buffer[y + 2][x + 1]),
                    (self.buffer[y + 3][x + 0], self.buffer[y + 3][x + 1]),
                )

    def iter_buffer(self) -> Generator[tuple[int, int], None, None]:
        for y in range(self.screen.height):
            for x in range(self.screen.width):
                yield x, y

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

    top_left = (0, 0)
    top_right = (canvas.w, 0)
    bottom_right = (canvas.w, canvas.h)
    bottom_left = (0, canvas.h)
    center = (canvas.cx, canvas.cy)
    center_top = (canvas.cx, 0)
    center_right = (canvas.w, canvas.cy)
    center_bottom = (canvas.cx, canvas.h)
    center_left = (0, canvas.cy)

    canvas.set_color(Color().bright_red())
    canvas.stroke_line(*center, *top_left)
    canvas.set_color(Color().bright_yellow())
    canvas.stroke_line(*center, *top_right)
    canvas.set_color(Color().bright_green())
    canvas.stroke_line(*center, *bottom_right)
    canvas.set_color(Color().bright_blue())
    canvas.stroke_line(*center, *bottom_left)
    canvas.set_color(Color().bright_cyan())
    canvas.stroke_line(*center, *center_top)
    canvas.set_color(Color().bright_magenta())
    canvas.stroke_line(*center, *center_right)
    canvas.set_color(Color().bright_gray())
    canvas.stroke_line(*center, *center_bottom)
    canvas.set_color(Color())
    canvas.stroke_line(*center, *center_left)

    print(canvas)
