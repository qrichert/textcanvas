#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::fmt;
use std::time;

use textcanvas::random::Rng;
use textcanvas::utils::GameLoop;
use textcanvas::{Color, TextCanvas};

const NB_STREAMS: i32 = 80;
const STREAM_LENGTH: i32 = 24;
const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const GLITCHES: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

enum Shade {
    BrightGreen,
    DimGreen,
    PreTip,
    Tip,
}

impl From<Shade> for Color {
    fn from(shade: Shade) -> Self {
        match shade {
            Shade::BrightGreen => Self::new().bright_green().fix(),
            Shade::DimGreen => Self::new().green().fix(),
            Shade::PreTip => Self::new().white().fix(),
            Shade::Tip => Self::new().bright_white().fix(),
        }
    }
}

/// One continuous text string.
///
/// Droplets run down the screen in a stream.
#[derive(Debug)]
pub struct Droplet {
    x: i32,
    y: f64,
    length: i32,
    chars: String,
    speed: f64,
}

impl Droplet {
    pub fn new(rng: &mut Rng) -> Self {
        let x = rng.irand_between(0, NB_STREAMS - 1);
        let length = rng.irand_between(STREAM_LENGTH / 2, STREAM_LENGTH * 3 / 2);
        let y = f64::from(-length) * 1.5; // Just out-of-bounds, and some.

        let mut chars: Vec<char> = CHARS.chars().collect();
        rng.shuffle(&mut chars);
        chars.truncate(STREAM_LENGTH as usize);
        let chars = chars.into_iter().collect();

        let speed = rng.frand_between(0.3, 0.8);

        Self {
            x,
            y,
            length,
            chars,
            speed,
        }
    }

    pub fn recycle(&mut self, rng: &mut Rng) {
        let droplet = Self::new(rng);
        *self = droplet;

        // Make one very fast at random. Doing it in `recycle()` instead
        // of `new()` keeps the initial "curtain fall" homogeneous.
        if rng.frand() > 0.99 {
            self.speed = (self.speed * 2.0).max(1.3);
        }
    }

    /// `self.y` as drawable integer.
    ///
    /// We keep the original `y` as fractional, it makes it easier to
    /// modulate falling speed.
    fn iy(&self) -> i32 {
        self.y.trunc() as i32
    }

    pub fn fall(&mut self) {
        self.y += self.speed;
    }

    #[must_use]
    pub fn has_fallen_out_of_screen(&self) -> bool {
        self.iy() >= STREAM_LENGTH
    }

    pub fn maybe_glitch(&mut self, rng: &mut Rng) {
        if rng.frand() <= 0.999 {
            return;
        }

        let tip = self.iy() + self.length;
        if tip - 2 < 0 {
            // No green chars visible.
            return;
        }

        // `-3` to exclude tip and pre-tip.
        let pos = rng.irand_between(0, tip - 3) as usize;

        let mut chars: Vec<char> = self.chars.chars().collect();
        for (i, char) in chars.iter_mut().enumerate() {
            if i == pos {
                let glitch = rng.sample(&GLITCHES.chars().collect::<Vec<char>>(), 1)[0];
                *char = glitch;
            }
        }
        self.chars = chars.into_iter().collect();
    }

    pub fn draw_onto(&mut self, canvas: &mut TextCanvas) {
        let chars = self.to_string();
        debug_assert!(chars.chars().count() == STREAM_LENGTH as usize);

        let i_tip = self.iy() + self.length - 1;

        // Start at `y=0`, NOT `droplet.y`. The droplet is already
        // rendered, including spacing, etc.
        for (i, char_) in chars.chars().enumerate() {
            let i = i as i32;
            canvas.set_color(&self.determine_char_color(i, i_tip));
            // `merge_text()` ignores spaces.
            canvas.merge_text(&char_.to_string(), self.x, i);
        }
    }

    fn determine_char_color(&self, i: i32, i_tip: i32) -> Color {
        if i == i_tip {
            return Shade::Tip.into();
        }
        if i == i_tip - 1 {
            return Shade::PreTip.into();
        }

        // Use `self.x` and `self.length` to deterministically randomize
        // bucket size and bright spots distribution.
        let s = f64::from(self.x).sin().abs(); // [0; 1]
        let d = f64::from(self.length).sin() - 0.3; // [-1.3; 0.7] (slightly skewed towards dim).

        if f64::from(i_tip - i).sin() * s <= d {
            // `sin(x) * S >= D`
            // Deterministic way (`i_tip - i`) to modulate shade.
            // `S` influences the size of the (base) buckets.
            // `D` (`[-1; 1]`) affects the distribution. Higher means
            //     bias towards bright, lower means bias towards dim,
            //     `0.0` being neutral.
            Shade::BrightGreen.into()
        } else {
            Shade::DimGreen.into()
        }
    }
}

impl fmt::Display for Droplet {
    /// Render droplet as part of a stream, with leading and trailing whitespace.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Not yet visible (above screen).
        if self.iy() + self.length <= 0 {
            return write!(f, "{}", " ".repeat(STREAM_LENGTH as usize));
        }
        // No longer visible (below screen).
        if self.iy() >= STREAM_LENGTH {
            return write!(f, "{}", " ".repeat(STREAM_LENGTH as usize));
        }
        let window_start = self.iy().clamp(0, STREAM_LENGTH - 1) as usize;
        let window_end = (self.iy() + self.length - 1).clamp(0, STREAM_LENGTH - 1) as usize;

        write!(
            f,
            "{}{}{}",
            " ".repeat(window_start),
            // Equivalent to `&self.chars[window_start..=window_end]`, but with `chars()`.
            &self
                .chars
                .chars()
                .skip(window_start)
                .take(window_end - window_start + 1)
                .collect::<String>(),
            " ".repeat(STREAM_LENGTH as usize - window_end - 1)
        )
    }
}

fn main() {
    debug_assert!(CHARS.chars().count() > STREAM_LENGTH as usize);

    if std::env::args().any(|arg| arg == "-i" || arg == "--with-intro") {
        do_intro();
    }

    let mut canvas = TextCanvas::new(NB_STREAMS, STREAM_LENGTH);
    let mut rng = Rng::new();

    let mut droplets: Vec<Droplet> = (0..(NB_STREAMS * 11 / 10))
        .map(|_| Droplet::new(&mut rng))
        .collect();

    GameLoop::loop_fixed(time::Duration::from_millis(30), &mut || {
        canvas.clear();

        for droplet in &mut droplets {
            droplet.fall();
            if droplet.has_fallen_out_of_screen() {
                droplet.recycle(&mut rng);
            }
            droplet.maybe_glitch(&mut rng);
            droplet.draw_onto(&mut canvas);
        }

        Some(canvas.to_string())
    });
}

fn do_intro() {
    let sleep = |duration| std::thread::sleep(time::Duration::from_millis(duration));

    let mut game_loop = GameLoop::new();
    game_loop.set_up();

    let mut canvas = TextCanvas::new(NB_STREAMS, STREAM_LENGTH);
    let mut rng = Rng::new_from_seed(42);

    canvas.set_color(Color::new().bright_green());

    // Wake up, Neo...
    for (x, c) in "Wake up, Neo...".chars().enumerate() {
        canvas.draw_text(&c.to_string(), x as i32 + 3, 1);
        game_loop.update(&canvas.to_string());
        sleep(if c == ',' {
            300
        } else if c == ' ' {
            100
        } else {
            50
        });
    }
    sleep(2000);

    // The Matrix has you...
    canvas.clear();
    for (x, c) in "The Matrix has you...".chars().enumerate() {
        canvas.draw_text(&c.to_string(), x as i32 + 3, 1);
        game_loop.update(&canvas.to_string());

        sleep(if x < 3 {
            400
        } else {
            u64::from(rng.urand_between(150, 300))
        });
    }
    sleep(2000);

    // Follow the white rabbit.
    canvas.clear();
    for (x, c) in "Follow the white rabbit.".chars().enumerate() {
        canvas.draw_text(&c.to_string(), x as i32 + 3, 1);
        game_loop.update(&canvas.to_string());

        sleep(if x < 4 { 100 } else { 50 });
    }
    sleep(3000);

    // Knock, knock, Neo.
    canvas.clear();
    game_loop.update(&canvas.to_string());
    sleep(70);
    canvas.draw_text("Knock, knock, Neo.", 3, 1);
    game_loop.update(&canvas.to_string());

    // Don't tear down.
    // game_loop.tear_down();

    sleep(4000);
}
