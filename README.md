# TextCanvas

TextCanvas is JavaScript Canvas-like surface that can be used to draw to
the console. Other use cases include visual checks of mathematical
computations (i.e. does the graph at least look correct?), or even
snapshot testing (may not be the most accurate, but can have great
documentation value).

It is inspired by drawille[^1], which uses Braille Unicode characters to
increase the resolution of the console by a factor of 8 (8 Braille dots
in one console character).

The API is inspired by JavaScript's Canvas, but has almost no features.

## How It Works

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

## See Also

- https://en.wikipedia.org/wiki/Braille_Patterns
- https://www.unicode.org/charts/PDF/U2800.pdf

[^1]: https://github.com/asciimoo/drawille
