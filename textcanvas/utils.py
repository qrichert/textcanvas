import datetime as dt
import sys
import time
from typing import Callable


class GameLoop:
    """Help animate `TextCanvas`.

    The main functions of interest are `GameLoop.loop_fixed()`, and
    `GameLoop.loop_variable()`.

    Note:
        The other functions constitute the lower-level machinery. They
        are made available in case one wanted to be more hands on. But
        for normal use, use the `loop_*` functions.
    """

    @staticmethod
    def loop_fixed(
        time_step: dt.timedelta,
        render_frame: Callable[[], str | None],
    ) -> None:
        """Run a game loop with fixed time step.

        In this mode, the time step is fixed to a given duration. This
        means the frame rate is constant. This is simple to implement,
        and better for physics.

        <div class="warning">

        The given time step must always be greater than the time it
        takes to render a frame. If rendering a frame takes longer than
        the time step, the frame rate will obviously be affected, and
        the "fixed" nature of it will not hold.

        </div>

        Returns:
            The `render_frame` closure must return an `str | None`. If
            it returns `str`, then the string will be rendered, and on
            to the next iteration. If it returns `None`, the game loop
            stops right there.

        <div class="warning">

        The screen is never cleared between frames; the new frame only
        overwrites the old one in-place. This is to reduce the risk of
        flickering. Thus, the string should never get smaller from one
        iteration to the next, else it would not completely erase the
        old frame.

        </div>
        """
        game_loop = GameLoop()

        game_loop.set_up()

        try:
            while True:
                start: int = time.perf_counter_ns()

                if (frame := render_frame()) is None:
                    break

                game_loop.update(frame)

                time_elapsed: float = (time.perf_counter_ns() - start) / 1000  # μs
                time_to_next_frame = time_step - dt.timedelta(microseconds=time_elapsed)
                # If not, rendering took longer than time step. In which
                # case, we want to render the next frame immediately.
                if time_to_next_frame > dt.timedelta(0):
                    game_loop.sleep(time_to_next_frame)
        except KeyboardInterrupt:
            pass

        game_loop.tear_down()

    @staticmethod
    def loop_variable(render_frame: Callable[[float], str | None]) -> None:
        """Run a game loop with variable time step.

        In this mode, the time step is whatever time it takes to render
        a frame. There is no artificial delay between frames, it's just
        one after the other as fast as possible (although it _is_
        possible for the user to `sleep()` manually inside the loop, to
        save on some CPU cycles).

        The render duration of the last frame is passed to the loop as
        `delta_time`. `delta_time` should be used to modulate values to
        make the speed of the animations independent of the frame rate.

        Returns:
            The `render_frame` closure must return an `str | None`. If
            it returns `str`, then the string will be rendered, and on
            to the next iteration. If it returns `None`, the game loop
            stops right there.

        <div class="warning">

        The screen is never cleared between frames; the new frame only
        overwrites the old one in-place. This is to reduce the risk of
        flickering. Thus, the string should never get smaller from one
        iteration to the next, else it would not completely erase the
        old frame.

        </div>
        """
        game_loop = GameLoop()

        game_loop.set_up()

        # The first one is a bit... arbitrary. Close to 60fps.
        delta_time = dt.timedelta(milliseconds=17)

        try:
            while True:
                start: int = time.perf_counter_ns()

                if (frame := render_frame(delta_time.total_seconds())) is None:
                    break

                game_loop.update(frame)

                time_elapsed: float = (time.perf_counter_ns() - start) / 1000  # μs
                delta_time = dt.timedelta(microseconds=time_elapsed)
        except KeyboardInterrupt:
            pass

        game_loop.tear_down()

    # Lower-level machinery, made available if one wants to be more
    # hands on. For normal use, use the loop functions.

    def set_up(self) -> None:
        """Set the stage for the render loop.

        This function should be called once before the loop starts.

        This effectively hides the blinking text cursor and clears the
        screen.
        """
        self.hide_text_cursor()
        self.clear_screen()

    def update(self, frame: str) -> None:
        """Update the screen buffer with a new render.

        This function should be called on every iteration of the loop.

        This overwrites the current frame in-place, without clearing the
        screen (to prevent flickering).

        Any terminating newline character is stripped to ensure the
        output exactly matches the canvas' height.
        """
        # Always do extra operations _before_ drawing, to help keep
        # drawing time to a minimum and help reduce flickering.
        frame = frame.removesuffix("\n")
        self.move_cursor_top_left()
        print(f"{frame}", end="")
        # Flush because output may not contain newline.
        self.flush()

    def tear_down(self) -> None:
        """Clean up after the render loop.

        This function should be called once after the loop ends.

        This restores the blinking text cursor, prints an end-of-line
        (`\n`) character not to mess with the system prompt, and ensures
        the output buffer is flushed.
        """
        self.show_text_cursor()
        # The newline we've been omitting in the rendered frames.
        print(flush=True)

    # Even-lower-level machinery.

    @staticmethod
    def clear() -> None:
        """Clear the screen (soft).

        This is a "soft" clear. Since it does not flush, it may not be
        applied immediately (because of output buffering). It also won't
        reset the cursor to the top-left.

        For something more analogous to the system "clear" command, see
        `clear_screen()`.
        """
        print("\x1b[2J", end="", flush=False)

    def clear_screen(self) -> None:
        """Clear the screen (hard).

        This is a "hard" clear. Since it flushes, it will force the
        screen to be empty.

        <div class="warning">

        This is meant to be used during setup, before the render loop.
        To clear between frames, you're better off _overwriting_ instead
        of clearing. See `GameLoop.move_cursor_top_left()`.

        </div>
        """
        self.clear()
        self.move_cursor_top_left()
        self.flush()

    @staticmethod
    def hide_text_cursor() -> None:
        """Hide the blinking text cursor."""
        print("\x1b[?25l", end="", flush=False)

    @staticmethod
    def show_text_cursor() -> None:
        """Restore the blinking text cursor."""
        print("\x1b[?25h", end="", flush=False)

    @staticmethod
    def move_cursor_top_left() -> None:
        """Move the cursor to the top-left corner of the screen.

        That is, first row, first column.

        This function is meant to be used as a screen-wide carriage
        return (`\r`). With a regular `\r`, you move the cursor to the
        start of the line, and anything you write will overwrite the
        existing line. Here it is the same, but at the level of the
        entire screen. Since the render always has the same dimension,
        instead of clearing the screen, you can simply move the caret to
        the start and overwrite the exising characters.

        The benefit over a full-fledged `clear()` is that you never have
        an intermediate state where the screen is empty. Not only does
        this reduces the number of screen updates, it also prevents
        flickering (no blank screen between two frames).
        """
        print("\x1b[1;1H", end="", flush=False)

    @staticmethod
    def flush() -> None:
        """Flush stdout.

        Most often stdout is line-buffered. This means, what you write
        to stdout will be kept in memory and not be displayed until it
        sees and end-of-line `\n` character.

        Flushing stdout forces the immediate display of what's in the
        buffer, even if there is no `\n`.
        """
        sys.stdout.flush()

    @staticmethod
    def sleep(duration: dt.timedelta) -> None:
        """Sleep for some duration."""
        time.sleep(duration.total_seconds())
