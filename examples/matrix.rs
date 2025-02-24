use textcanvas::TextCanvas;

const NB_STREAMS: i32 = 80;

// /// One output column.
// ///
// /// A stream can contain, 0, 1 or more droplets.
// struct Stream;

/// One continuous text string.
///
/// Droplets run down the screen in a stream.
#[derive(Debug)]
struct Droplet {
    x: i32,
    y: i32,
}

fn main() {
    let mut canvas = TextCanvas::new(NB_STREAMS, 24);
    let mut rng = Rng::new();

    let droplets: Vec<Droplet> = (0..NB_STREAMS)
        .map(|_| Droplet {
            x: rng.next() % NB_STREAMS,
            y: 0,
        })
        .collect();

    draw_droplets(&mut canvas, &droplets);

    dbg!(&droplets);

    println!("{canvas}");
}

fn draw_droplets(canvas: &mut TextCanvas, droplets: &[Droplet]) {
    for droplet in droplets {
        canvas.draw_text_vertical("hello", droplet.x, droplet.y);
    }
}

pub struct Rng {
    inner: Box<dyn Iterator<Item = u32>>,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            inner: Box::new(Self::new_rng()),
        }
    }

    /// Pseudorandom number generator.
    ///
    /// From the "Xorshift RNGs" paper by George Marsaglia.
    ///
    /// See:
    /// - <https://blog.orhun.dev/zero-deps-random-in-rust/>
    /// - <https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573>
    fn new_rng() -> impl Iterator<Item = u32> {
        let mut random = Self::seed();
        // let mut random = 92u32;
        std::iter::repeat_with(move || {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            random
        })
    }

    fn seed() -> u32 {
        use std::hash::{Hasher, BuildHasher};
        std::collections::hash_map::RandomState::new().build_hasher().finish() as u32
    }

    pub fn next(self: &mut Self) -> i32 {
        (self.inner.next().expect("infinite") as i32).abs()
    }
}
