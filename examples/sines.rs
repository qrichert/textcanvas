//! Cool looking animated sine waves.
//!
//! ```text
//! ⠀⠀i:⠀107⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀Fixed⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠉⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠉⠉⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠉⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠉⠑⢄⠀
//! ⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠊⠀⠀⠀⠀⠀⢣
//! ⠀⠀⠀⠀⢘⡔⠉⠉⠉⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⣣⠊⠉⠉⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⣣⠊⠉⠉⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⡔⠁⠘⡄⠀⠀⠀⠀⠈⣢⡔⠒⠒⠒⠒⢄⡀⡰⠁⠀⠀⠀⠀⠀⠀⡠⠊⠀⢣⠀⠀⠀⠀⠈⣢⡔⠒⠒⠒⠒⢄⡀⢀⠎⠀⠀⠀⠀⠀⠀⡠⠊⠀⢣⠀⠀⠀⠀⠈⡢⡔⠒⠒⠒⠒⢄⡀⢀⠎⠀⠀⠀⠀⠀⢀⠔⠉
//! ⠀⢠⠊⠀⠀⠀⠱⡀⠀⠀⡠⠊⠀⠘⢄⠀⠀⠀⠀⢸⠣⡀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠈⢆⠀⠀⡠⠊⠀⠘⢄⠀⠀⠀⠀⠈⡮⡀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠈⢆⠀⢀⡠⠊⠀⠑⢄⠀⠀⠀⠀⠈⡮⡀⠀⠀⠀⠀⢠⠊⠀⠀
//! ⡰⠁⠀⠀⠀⠀⠀⢣⠔⠉⠀⠀⠀⠀⠈⢢⠀⠀⢠⠃⠀⠈⠑⢄⠀⢀⠎⠀⠀⠀⠀⠀⢘⡔⠉⠀⠀⠀⠀⠈⢢⠀⠀⠀⡜⠀⠈⠒⢄⠀⢠⠊⠀⠀⠀⠀⠀⢘⡔⠁⠀⠀⠀⠀⠈⢢⠀⠀⠀⡜⠀⠈⠒⢄⠀⡔⠁⠀⠀⠀
//! ⠣⣀⣀⣀⣀⡠⠒⠁⢣⠀⠀⠀⠀⠀⠀⠀⠑⢄⠎⠀⠀⠀⠀⢀⠕⠣⣀⣀⢀⣀⡠⠒⠁⠘⡄⠀⠀⠀⠀⠀⠀⠑⢄⡰⠁⠀⠀⠀⢀⠕⠥⣀⣀⣀⣀⡠⠒⠁⠘⡄⠀⠀⠀⠀⠀⠀⠑⢄⡸⠀⠀⠀⠀⡠⠛⠤⣀⣀⣀⣀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⢀⠎⠑⠤⢄⠤⠔⠁⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⡰⠑⠤⠤⠤⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⡰⠑⠤⠤⠤⠊⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠤⠤⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⢄⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⠤⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ```
//!
//! ```text
//! ⠀Δt:⠀0.003731337⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀Variable⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! acc:⠀122.2455052⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⢀⠔⠉⠉⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠉⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠖⠋⠉⠑⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠉⠉
//! ⢠⠃⠀⠀⠀⠀⠈⢢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠈⢲⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀
//! ⢇⠀⠀⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠉⠉⠉⢲⢧⠀⠀⠀⠀⠀⠀⠀⢱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠎⠉⠉⠉⢱⠣⡀⠀⠀⠀⠀⠀⠀⠸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠉⠉⠑⡾⡀⠀⠀⠀
//! ⠀⠱⡀⣀⠤⠔⠒⠒⠚⢦⢄⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⢠⠃⠀⠹⡀⣀⠤⠒⠒⠒⠒⢣⢄⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⢀⠇⠀⠈⢆⢀⡠⠒⠒⠒⠒⠳⣄⠀⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⡜⠀⠘⢄⢀⡠
//! ⠀⢀⠜⢇⠀⠀⠀⠀⠀⠘⡄⠑⠢⡀⠀⠀⡰⠁⠀⠀⠀⠀⢀⠎⠀⠀⢠⠊⢆⠀⠀⠀⠀⠀⠘⡄⠑⠤⡀⠀⠀⡰⠁⠀⠀⠀⠀⢀⠎⠀⠀⢀⠔⠱⡀⠀⠀⠀⠀⠀⢣⠑⠤⡀⠀⠀⢀⠎⠀⠀⠀⠀⠀⡸⠀⠀⢀⠔⠣⡀
//! ⠔⠁⠀⠀⠣⡀⠀⠀⠀⠀⠘⡄⠀⠑⢄⠜⠀⠀⠀⠀⠀⠀⡎⠀⣀⠔⠁⠀⠀⠣⡀⠀⠀⠀⠀⠘⡄⠀⠈⢢⡔⠁⠀⠀⠀⠀⠀⡎⠀⡠⠔⠁⠀⠀⠘⢄⠀⠀⠀⠀⠀⢇⠀⠈⠢⡠⠃⠀⠀⠀⠀⠀⢰⠁⡠⠔⠁⠀⠀⠘
//! ⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠱⡀⡠⠃⠑⠒⠤⣀⣀⣀⣜⠤⠊⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠱⡀⡠⠇⠈⠒⠤⣀⣀⣀⣔⠥⠊⠀⠀⠀⠀⠀⠀⠈⠢⡀⠀⠀⠀⠈⢆⢀⠎⠈⠒⢄⣀⡀⣀⣠⠧⠊⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠑⠢⠤⠤⠔⠻⡀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⠤⠤⠔⠻⡀⠀⠀⠀⠀⠀⠀⡸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⠤⡠⠤⠚⢇⠀⠀⠀⠀⠀⠈⢀⠇⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⡔⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⢠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⢄⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠢⠤⠤⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠓⠤⠤⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ```

use std::time;
use textcanvas::charts::Plot;
use textcanvas::utils::GameLoop;
use textcanvas::TextCanvas;

fn main() {
    for _ in 0..3 {
        with_fixed_time_step();
        with_variable_time_step();
    }
}

/// Will loop at a predefined rate.
fn with_fixed_time_step() {
    let mut canvas = TextCanvas::new(80, 16);

    let mut big_sine = TextCanvas::new(80, 10);
    let mut medium_sine = TextCanvas::new(80, 6);
    let mut small_sine = TextCanvas::new(80, 4);

    let f = |x: f64| x.sin();

    let mut i = 0;
    #[rustfmt::skip]
    GameLoop::loop_fixed(time::Duration::from_millis(30), &mut || {
        if i == 108 {
            // Break out of the loop.
            return None;
        }

        let big_offset = f64::from(i) / 350.0 * 37.0;
        let medium_offset = f64::from(i) / 250.0 * 37.0;
        let small_offset = f64::from(i) / 150.0 * 37.0;

        canvas.clear();

        big_sine.clear();
        medium_sine.clear();
        small_sine.clear();

        Plot::function(&mut big_sine, -10.0 + big_offset, 10.0 + big_offset, &f);
        Plot::function(&mut medium_sine, -10.0 + medium_offset, 10.0 + medium_offset, &f);
        Plot::function(&mut small_sine, -10.0 + small_offset, 10.0 + small_offset, &f);

        canvas.merge_canvas(&big_sine, 0, 12);
        canvas.merge_canvas(&medium_sine, 0, 20);
        canvas.merge_canvas(&small_sine, 0, 25);

        canvas.draw_text(&format!("  i: {i}"), 0, 0);
        canvas.draw_text("Fixed", 37, 0);

        i += 1;

        Some(canvas.to_string())
    });
}

/// Will loop as fast as possible.
fn with_variable_time_step() {
    let mut canvas = TextCanvas::new(80, 16);

    let mut big_sine = TextCanvas::new(80, 10);
    let mut medium_sine = TextCanvas::new(80, 6);
    let mut small_sine = TextCanvas::new(80, 4);

    let f = |x: f64| x.sin();

    let mut accumulator = 0.0;
    #[rustfmt::skip]
    GameLoop::loop_variable(&mut |delta_time| {
        if accumulator >= 333.33 {
            // Break out of the loop.
            return None;
        }

        let big_offset = accumulator / 350.0 * 12.0;
        let medium_offset = accumulator / 250.0 * 12.0;
        let small_offset = accumulator / 150.0 * 12.0;

        canvas.clear();

        big_sine.clear();
        medium_sine.clear();
        small_sine.clear();

        Plot::function(&mut big_sine, -10.0 + big_offset, 10.0 + big_offset, &f);
        Plot::function(&mut medium_sine, -10.0 + medium_offset, 10.0 + medium_offset, &f);
        Plot::function(&mut small_sine, -10.0 + small_offset, 10.0 + small_offset, &f);

        canvas.merge_canvas(&big_sine, 0, 12);
        canvas.merge_canvas(&medium_sine, 0, 20);
        canvas.merge_canvas(&small_sine, 0, 25);

        canvas.draw_text(&format!(" Δt: {delta_time:.9}"), 0, 0);
        canvas.draw_text(&format!("acc: {accumulator:>11.7}"), 0, 1);
        canvas.draw_text("Variable", 36, 0);

        accumulator += 100.0 * delta_time;

        Some(canvas.to_string())
    });
}
