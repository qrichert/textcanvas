import datetime as dt
import doctest
import time
import unittest
from unittest.mock import Mock

import textcanvas.utils
from textcanvas.textcanvas import TextCanvas
from textcanvas.utils import GameLoop

textcanvas.utils.print = Mock()  # type: ignore


def load_tests(
    loader: unittest.TestLoader, tests: unittest.TestSuite, ignore: str
) -> unittest.TestSuite:
    """Add module doctests."""
    tests.addTests(doctest.DocTestSuite(textcanvas.utils))
    return tests


class TestGameLoop(unittest.TestCase):
    def test_loop_fixed(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop() -> str | None:
            nonlocal i
            i += 1
            if i == 4:
                return None
            canvas.draw_text(f"{i}", i - 1, 0)
            return canvas.to_string()

        GameLoop.loop_fixed(dt.timedelta(milliseconds=1), loop)

        self.assertEqual(canvas.to_string(), "123\n⠀⠀⠀\n")
        self.assertEqual(i, 4)

    def test_loop_fixed_exits_on_none(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop() -> str | None:
            nonlocal i
            i += 1
            if i == 2:
                return None
            canvas.draw_text(f"{i}", 0, 0)
            return canvas.to_string()

        GameLoop.loop_fixed(dt.timedelta(milliseconds=1), loop)

        # First iteration passed.
        self.assertEqual(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n")
        # Second iteration stopped.
        self.assertEqual(i, 2)

    def test_loop_fixed_exits_on_keyboard_interrupt(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop() -> str | None:
            nonlocal i
            i += 1
            if i == 2:
                raise KeyboardInterrupt
            canvas.draw_text(f"{i}", 0, 0)
            return canvas.to_string()

        GameLoop.loop_fixed(dt.timedelta(milliseconds=1), loop)

        # First iteration passed.
        self.assertEqual(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n")
        # Second iteration stopped.
        self.assertEqual(i, 2)

    def test_loop_fixed_takes_longer_than_time_step(self) -> None:
        i = 0

        def loop() -> str | None:
            nonlocal i
            i += 1
            if i == 2:
                return None
            # 10ms > 1ms
            time.sleep(dt.timedelta(milliseconds=10).total_seconds())
            return ""

        GameLoop.loop_fixed(dt.timedelta(milliseconds=1), loop)

    def test_loop_variable(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop(_: float) -> str | None:
            nonlocal i
            i += 1
            if i == 4:
                return None
            canvas.draw_text(f"{i}", i - 1, 0)
            return canvas.to_string()

        GameLoop.loop_variable(loop)

        self.assertEqual(canvas.to_string(), "123\n⠀⠀⠀\n")
        self.assertEqual(i, 4)

    def test_loop_variable_exits_on_none(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop(_: float) -> str | None:
            nonlocal i
            i += 1
            if i == 2:
                return None
            canvas.draw_text(f"{i}", 0, 0)
            return canvas.to_string()

        GameLoop.loop_variable(loop)

        # First iteration passed.
        self.assertEqual(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n")
        # Second iteration stopped.
        self.assertEqual(i, 2)

    def test_loop_variable_exits_on_keyboard_interrupt(self) -> None:
        canvas = TextCanvas(3, 2)

        i = 0

        def loop(_: float) -> str | None:
            nonlocal i
            i += 1
            if i == 2:
                raise KeyboardInterrupt
            canvas.draw_text(f"{i}", 0, 0)
            return canvas.to_string()

        GameLoop.loop_variable(loop)

        # First iteration passed.
        self.assertEqual(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n")
        # Second iteration stopped.
        self.assertEqual(i, 2)


if __name__ == "__main__":
    unittest.main()
