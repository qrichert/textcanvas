"""Conway's Game of Life.

```shell-session
$ python -m examples.game_of_life
```
"""

import datetime as dt
import os
import random
import sys
from typing import Literal

from textcanvas.textcanvas import PixelBuffer, TextCanvas
from textcanvas.utils import GameLoop

CLEAR_CMD: Literal["clear", "cls"] = "cls" if os.name == "nt" else "clear"
CLEAR_SEQUENCE: str = "\x1b[2J\x1b[1;1H"
HIDE_TEXT_CURSOR_SEQUENCE: str = "\x1b[?25l"
SHOW_TEXT_CURSOR_SEQUENCE: str = "\x1b[?25h"


class GameOfLife:
    def __init__(self) -> None:
        self.canvas: TextCanvas = TextCanvas(80, 24)
        self._draw_title()
        self.old_buffer: PixelBuffer = []
        self._init_game()

    def _draw_title(self) -> None:
        title: str = "Conway's Game of Life"
        self.canvas.draw_text(
            title,
            self.canvas.output.width // 2 - len(title) // 2,
            self.canvas.output.height // 2 - 1,
        )

    def _init_game(self) -> None:
        for x, y in self.canvas.iter_buffer():
            self.set_cell_state(x, y, random.random() > 0.9)

    def exec(self, nb_iterations: int = 1000) -> None:
        print(HIDE_TEXT_CURSOR_SEQUENCE, end="")
        try:
            self._game_loop(nb_iterations)
        except KeyboardInterrupt:
            return
        finally:
            print(SHOW_TEXT_CURSOR_SEQUENCE)

    def _game_loop(self, nb_iterations: int) -> None:
        i: int = 0

        def loop() -> str | None:
            nonlocal i
            i += 1
            if i > nb_iterations:
                return None
            self.update()
            return self.canvas.to_string()

        GameLoop.loop_fixed(dt.timedelta(seconds=1 / 24), loop)

    def update(self) -> None:
        self.copy_buffer()
        for x, y in self.canvas.iter_buffer():
            self.update_cell(x, y)

    def copy_buffer(self) -> None:
        self.old_buffer = [[cell for cell in row] for row in self.canvas.buffer]

    def update_cell(self, x: int, y: int) -> None:
        is_alive: bool = self.is_cell_alive(x, y)
        is_dead: bool = not is_alive
        nb_neighbors: int = self.count_neighbors(x, y)

        if is_alive and (nb_neighbors < 2 or nb_neighbors > 3):
            # Under / over-population.
            self.set_cell_dead(x, y)
        elif is_dead and nb_neighbors == 3:
            # Reproduction.
            self.set_cell_alive(x, y)

    def count_neighbors(self, x: int, y: int) -> int:
        nb_neighbors: int = 0
        for i in range(x - 1, x + 2):
            for j in range(y - 1, y + 2):
                if i == x and j == y:
                    continue
                if self.is_cell_alive(i, j):
                    nb_neighbors += 1
        return nb_neighbors

    def is_cell_alive(self, x: int, y: int) -> bool:
        width, height = self.canvas.screen.width, self.canvas.screen.height
        # Cells outside of bounds move to the other side (taurus space).
        # If you want them dead instead, remove the modulo. As out of
        # bound pixels return None.
        xmod: int = x % width
        ymod: int = y % height
        return bool(self.old_buffer[ymod][xmod])

    def set_cell_alive(self, x: int, y: int) -> None:
        self.set_cell_state(x, y, True)

    def set_cell_dead(self, x: int, y: int) -> None:
        self.set_cell_state(x, y, False)

    def set_cell_state(self, x: int, y: int, state: bool) -> None:
        width, height = self.canvas.screen.width, self.canvas.screen.height
        xmod: int = x % width
        ymod: int = y % height
        self.canvas.set_pixel(xmod, ymod, state)


if __name__ == "__main__":
    game_of_life: GameOfLife = GameOfLife()

    if len(sys.argv) > 1:
        nb_iterations: int = int(sys.argv[1])
        game_of_life.exec(nb_iterations)
    else:
        game_of_life.exec()
