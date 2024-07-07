use std::time;
use std::{io, io::Write};

/// Help animate [`TextCanvas`](crate::TextCanvas).
///
/// The main functions of interest are [`GameLoop::loop_fixed()`], and
/// [`GameLoop::loop_variable()`].
///
/// # Note
///
/// The other functions constitute the lower-level machinery. They are
/// made available in case one wanted to be more hands on. But for
/// normal use, use the `loop_*` functions.
pub struct GameLoop<'a> {
    stdout: io::StdoutLock<'a>,
}

impl GameLoop<'_> {
    /// Run a game loop with fixed time step.
    ///
    /// In this mode, the time step is fixed to a given duration. This
    /// means the frame rate is constant. This is simple to implement,
    /// and better for physics.
    ///
    /// <div class="warning">
    ///
    /// The given time step must always be greater than the time it
    /// takes to render a frame. If rendering a frame takes longer than
    /// the time step, the frame rate will obviously be affected, and
    /// the "fixed" nature of it will not hold.
    ///
    /// </div>
    ///
    /// # Return values
    ///
    /// The `render_frame` closure must return an `Option<String>`. If
    /// it returns `Some<String>`, then `String` will be rendered, and
    /// on to the next iteration. If it returns `None`, the game loop
    /// stops right there.
    ///
    /// <div class="warning">
    ///
    /// The screen is never cleared between frames; the new frame only
    /// overwrites the old one in-place. This is to reduce the risk of
    /// flickering. Thus, the string should never get smaller from one
    /// iteration to the next, else it would not completely erase the
    /// old frame.
    ///
    /// </div>
    pub fn loop_fixed(
        time_step: time::Duration,
        render_frame: &mut impl FnMut() -> Option<String>,
    ) {
        let mut game_loop = Self::new();

        game_loop.set_up();

        loop {
            let start = time::Instant::now();

            let Some(frame) = render_frame() else {
                break;
            };
            #[cfg(not(tarpaulin_include))] // Wrongly marked uncovered.
            game_loop.update(&frame);

            #[cfg(not(tarpaulin_include))] // Wrongly marked uncovered.
            let delta_time = start.elapsed();
            #[cfg(not(tarpaulin_include))] // Wrongly marked uncovered.
            let time_to_next_frame = time_step.checked_sub(delta_time);
            // If not, rendering took longer than time step. In which
            // case, we want to render the next frame immediately.
            if let Some(time_to_next_frame) = time_to_next_frame {
                game_loop.sleep(time_to_next_frame);
            }
        }

        game_loop.tear_down();
    }

    /// Run a game loop with variable time step.
    ///
    /// In this mode, the time step is whatever time it takes to render
    /// a frame. There is no artificial delay between frames, it's just
    /// one after the other as fast as possible (although it _is_
    /// possible for the user to `sleep()` manually inside the loop, to
    /// save on some CPU cycles).
    ///
    /// The render duration of the last frame is passed to the loop as
    /// `delta_time`. `delta_time` should be used to modulate values to
    /// make the speed of the animations independent of the frame rate.
    ///
    /// # Return values
    ///
    /// The `render_frame` closure must return an `Option<String>`. If
    /// it returns `Some<String>`, then `String` will be rendered, and
    /// on to the next iteration. If it returns `None`, the game loop
    /// stops right there.
    ///
    /// <div class="warning">
    ///
    /// The screen is never cleared between frames; the new frame only
    /// overwrites the old one in-place. This is to reduce the risk of
    /// flickering. Thus, the string should never get smaller from one
    /// iteration to the next, else it would not completely erase the
    /// old frame.
    ///
    /// </div>
    pub fn loop_variable(render_frame: &mut impl FnMut(f64) -> Option<String>) {
        let mut game_loop = Self::new();

        game_loop.set_up();

        // The first one is a bit... arbitrary. Close to 60fps.
        let mut delta_time = time::Duration::from_millis(17);

        loop {
            let start = time::Instant::now();

            let Some(frame) = render_frame(delta_time.as_secs_f64()) else {
                break;
            };
            game_loop.update(&frame);

            delta_time = start.elapsed();
        }

        #[cfg(not(tarpaulin_include))] // Wrongly marked uncovered.
        game_loop.tear_down();
    }

    // Lower-level machinery, made available if one wants to be more
    // hands on. For normal use, use the loop functions.

    /// Create new `GameLoop` instance, acquiring a lock to stdout.
    ///
    /// The lock will be valid for the entire lifetime of the loop.
    #[must_use]
    pub fn new() -> Self {
        // Acquire the lock once (instead of on every call to `print!`).
        let stdout = io::stdout().lock();
        Self { stdout }
    }

    /// Set the stage for the render loop.
    ///
    /// This function should be called once before the loop starts.
    ///
    /// This effectively hides the blinking text cursor and clears the
    /// screen.
    pub fn set_up(&mut self) {
        self.hide_text_cursor();
        self.clear_screen();
    }

    /// Update the screen buffer with a new render.
    ///
    /// This function should be called on every iteration of the loop.
    ///
    /// This overwrites the current frame in-place, without clearing the
    /// screen (to prevent flickering).
    ///
    /// Any terminating newline character is stripped to ensure the
    /// output exactly matches the canvas' height.
    pub fn update(&mut self, frame: &str) {
        // Always do extra operations _before_ drawing, to help keep
        // drawing time to a minimum and help reduce flickering.
        let frame = frame.strip_suffix('\n').unwrap_or(frame);
        self.move_cursor_top_left();
        self.print(frame);
        // Flush because output may not contain newline.
        self.flush();
    }

    /// Clean up after the render loop.
    ///
    /// This function should be called once after the loop ends.
    ///
    /// This restores the blinking text cursor, prints an end-of-line
    /// (`\n`) character not to mess with the system prompt, and ensures
    /// the output buffer is flushed.
    pub fn tear_down(&mut self) {
        self.show_text_cursor();
        // The newline we've been omitting in the rendered frames.
        self.print("\n");
        self.flush();
    }

    // Even-lower-level machinery.

    /// Clear the screen (soft).
    ///
    /// This is a "soft" clear. Since it does not flush, it may not be
    /// applied immediately (because of output buffering). It also won't
    /// reset the cursor to the top-left.
    ///
    /// For something more analogous to the system "clear" command, see
    /// [`clear_screen()`](GameLoop::clear_screen).
    pub fn clear(&mut self) {
        self.print("\x1b[2J");
    }

    /// Clear the screen (hard).
    ///
    /// This is a "hard" clear. Since it flushes, it will force the
    /// screen to be empty.
    ///
    /// <div class="warning">
    ///
    /// This is meant to be used during setup, before the render loop.
    /// To clear between frames, you're better off _overwriting_ instead
    /// of clearing. See [`GameLoop::move_cursor_top_left()`].
    ///
    /// </div>
    pub fn clear_screen(&mut self) {
        self.clear();
        self.move_cursor_top_left();
        self.flush();
    }

    /// Hide the blinking text cursor.
    pub fn hide_text_cursor(&mut self) {
        self.print("\x1b[?25l");
    }

    /// Restore the blinking text cursor.
    pub fn show_text_cursor(&mut self) {
        self.print("\x1b[?25h");
    }

    /// Move the cursor to the top-left corner of the screen.
    ///
    /// That is, first row, first column.
    ///
    /// This function is meant to be used as a screen-wide carriage
    /// return (`\r`). With a regular `\r`, you move the cursor to the
    /// start of the line, and anything you write will overwrite the
    /// existing line. Here it is the same, but at the level of the
    /// entire screen. Since the render always has the same dimension,
    /// instead of clearing the screen, you can simply move the caret to
    /// the start and overwrite the exising characters.
    ///
    /// The benefit over a full-fledged `clear()` is that you never have
    /// an intermediate state where the screen is empty. Not only does
    /// this reduces the number of screen updates, it also prevents
    /// flickering (no blank screen between two frames).
    pub fn move_cursor_top_left(&mut self) {
        self.print("\x1b[1;1H");
    }

    /// Print to the locked stdout.
    #[inline]
    pub fn print(&mut self, string: &str) {
        #![allow(unreachable_code)]
        #[cfg(test)]
        {
            // `print!` is silenced in tests, while stdout is not.
            print!("{string}");
            return;
        }
        let _ = write!(self.stdout, "{string}");
    }

    /// Flush stdout.
    ///
    /// Most often stdout is line-buffered. This means, what you write
    /// to stdout will be kept in memory and not be displayed until it
    /// sees and end-of-line `\n` character.
    ///
    /// Flushing stdout forces the immediate display of what's in the
    /// buffer, even if there is no `\n`.
    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }

    /// Sleep for some duration.
    pub fn sleep(&self, duration: time::Duration) {
        std::thread::sleep(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextCanvas;

    #[test]
    fn loop_fixed() {
        let mut canvas = TextCanvas::new(3, 2);

        let mut i = 0;

        GameLoop::loop_fixed(time::Duration::from_millis(1), &mut || {
            i += 1;
            if i == 4 {
                return None;
            }
            canvas.draw_text(&format!("{i}"), i - 1, 0);
            Some(canvas.to_string())
        });

        assert_eq!(canvas.to_string(), "123\n⠀⠀⠀\n");
        assert_eq!(i, 4);
    }

    #[test]
    fn loop_fixed_exits_on_none() {
        let mut canvas = TextCanvas::new(3, 2);

        let mut i = 0;

        GameLoop::loop_fixed(time::Duration::from_millis(1), &mut || {
            i += 1;
            if i == 2 {
                return None;
            }
            canvas.draw_text(&format!("{i}"), 0, 0);
            Some(canvas.to_string())
        });

        // First iteration passed.
        assert_eq!(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n");
        // Second iteration stopped.
        assert_eq!(i, 2);
    }

    #[test]
    fn loop_fixed_takes_longer_than_time_step() {
        let mut i = 0;
        GameLoop::loop_fixed(time::Duration::from_millis(1), &mut || {
            i += 1;
            if i == 2 {
                return None;
            }
            // 10ms > 1ms
            std::thread::sleep(time::Duration::from_millis(10));
            Some(String::new())
        });
    }

    #[test]
    fn loop_variable() {
        let mut canvas = TextCanvas::new(3, 2);

        let mut i = 0;

        GameLoop::loop_variable(&mut |_| {
            i += 1;
            if i == 4 {
                return None;
            }
            canvas.draw_text(&format!("{i}"), i - 1, 0);
            Some(canvas.to_string())
        });

        assert_eq!(canvas.to_string(), "123\n⠀⠀⠀\n");
        assert_eq!(i, 4);
    }

    #[test]
    fn loop_variable_exits_on_none() {
        let mut canvas = TextCanvas::new(3, 2);

        let mut i = 0;

        GameLoop::loop_variable(&mut |_| {
            i += 1;
            if i == 2 {
                return None;
            }
            canvas.draw_text(&format!("{i}"), 0, 0);
            Some(canvas.to_string())
        });

        // First iteration passed.
        assert_eq!(canvas.to_string(), "1⠀⠀\n⠀⠀⠀\n");
        // Second iteration stopped.
        assert_eq!(i, 2);
    }
}
