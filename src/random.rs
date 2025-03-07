/// Very basic (pseudo) Random Number Generator.
///
/// The idea of having this here is to be able to add basic randomness
/// to canvases, without adding new dependencies.
///
/// It can be frustrating to pull in a whole new crate to generate a
/// one-off random number, for which in all likelihood you don't care
/// about perfect uniform distribution.
///
/// <div class="warning">
///
/// This is _very basic_, and does **not** have foolproof distribution.
/// Use the `rand` crate for anything requiring optimal distribution.
///
/// </div>
///
/// Now realistically speaking, it is more than good enough for anything
/// you'd likely do with this library. It's there for convenience. If
/// you don't need cryptographically-secure randomness, and don't want
/// to add a dependency on `rand`, use it. Otherwise, don't.
#[derive(Copy, Clone, Debug)]
pub struct Rng {
    random: u32,
}

impl Rng {
    /// Create a new pseudo-random number generator.
    ///
    /// This uses a non-deterministic seed, which is usually what you
    /// want. If you need determinism, use [`Rng::new_from_seed()`].
    ///
    /// Non-deterministic means you can't predict what the random
    /// numbers wil be.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new();
    ///
    /// let random: u32 = rng.urand();
    /// let random: i32 = rng.irand();
    /// let random: f64 = rng.frand();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            random: Self::seed(), // Original uses fixed `92u32`.
        }
    }

    /// Create a new pseudo-random number generator from a seed.
    ///
    /// This uses a deterministic seed, yielding a deterministic
    /// sequence of numbers. The resulting distribution is _still_
    /// uniform, only the numbers are predictable.
    ///
    /// A seeded RNG is useful every time you need reproductible
    /// results (e.g., an animation that has some randomness to it, but
    /// you always want to play the exact same animation, or just for
    /// testing purposes where you need to know the results in advance).
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// // `rng1` and `rng2` share the same seed, so they will yield the
    /// // same sequence of numbers.
    /// let mut rng1 = Rng::new_from_seed(42);
    /// let mut rng2 = Rng::new_from_seed(42);
    ///
    /// let seq1 = vec![rng1.urand(), rng1.urand(), rng1.urand()];
    /// let seq2 = vec![rng2.urand(), rng2.urand(), rng2.urand()];
    ///
    /// assert_eq!(seq1, seq2);
    /// assert_eq!(seq1, [11355432, 2836018348, 476557059]);
    /// assert_eq!(seq2, [11355432, 2836018348, 476557059]);
    ///
    /// // `rng3` has a different seed, so it will yield a different
    /// // sequence than `rng1` and `rng2`.
    /// let mut rng3 = Rng::new_from_seed(1337);
    ///
    /// let seq3 = vec![rng3.urand(), rng3.urand(), rng3.urand()];
    ///
    /// assert_ne!(seq3, seq1);
    /// assert_eq!(seq3, [339970090, 3449400233, 3849456703]);
    /// ```
    #[must_use]
    pub fn new_from_seed(seed: u32) -> Self {
        Self { random: seed }
    }

    #[cfg(not(tarpaulin_include))]
    #[allow(clippy::cast_possible_truncation)]
    fn seed() -> u32 {
        use std::hash::{BuildHasher, Hasher};
        let seed = std::collections::hash_map::RandomState::new()
            .build_hasher()
            .finish() as u32;
        if seed == 0 {
            1 // Ensure non-zero seed.
        } else {
            seed
        }
    }

    /// Pseudo-random number between `0` and `u32::MAX`.
    ///
    /// From the "Xorshift RNGs" paper by George Marsaglia.
    ///
    /// See:
    ///  - <https://blog.orhun.dev/zero-deps-random-in-rust/>
    ///  - <https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573>
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.urand();
    ///
    /// assert_eq!(random, 24873849);
    /// ```
    #[must_use]
    pub fn urand(&mut self) -> u32 {
        self.random ^= self.random.wrapping_shl(13);
        self.random ^= self.random.wrapping_shr(17);
        self.random ^= self.random.wrapping_shl(5);
        self.random
    }

    /// Pseudo-random number between `i32::MIN` and `i32::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.irand();
    ///
    /// assert_eq!(random, 24873849);
    /// ```
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn irand(&mut self) -> i32 {
        self.urand() as i32
    }

    /// Pseudo-random number between `0.0` and `1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.frand();
    ///
    /// assert_eq!(format!("{random:.4}"), "0.0058");
    /// ```
    #[must_use]
    pub fn frand(&mut self) -> f64 {
        f64::from(self.urand()) / f64::from(u32::MAX)
    }

    /// Pseudo-random number within a range (inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.urand_between(1, 20);
    ///
    /// assert_eq!(random, 10);
    /// ```
    #[must_use]
    pub fn urand_between(&mut self, min: u32, max: u32) -> u32 {
        if min == max {
            return min;
        }
        if min > max {
            return self.urand_between(max, min);
        }
        min + self.urand() % (max - min + 1)
    }

    /// Pseudo-random number within a range (inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.irand_between(-20, -1);
    ///
    /// assert_eq!(random, -11);
    /// ```
    #[must_use]
    pub fn irand_between(&mut self, min: i32, max: i32) -> i32 {
        if min == max {
            return min;
        }
        if min > max {
            return self.irand_between(max, min);
        }

        // `irand() % range` is negative if `irand() < 0`, and
        // `(irand() % range)).abs()` would fold negatives into
        // positives, skewing the distribution (`abs(x)` has bias
        // towards higher numbers if `x < 0`, making them appear
        // more frequently).
        //
        // Here:
        //   1. `self.irand() % range` may be negative.
        //   2. `+ range` makes it `>= 0`.
        //   3. `% range` converts it back to the original, but `>= 0`,
        //      without skewing the distribution.
        min + ((self.irand() % (max - min + 1)) + (max - min + 1)) % (max - min + 1)
    }

    /// Pseudo-random number within a range (inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    ///
    /// let random = rng.frand_between(1.5, 3.0);
    ///
    /// assert_eq!(format!("{random:.4}"), "1.5087");
    /// ```
    #[must_use]
    pub fn frand_between(&mut self, min: f64, max: f64) -> f64 {
        if (min - max).abs() < 0.000_000_1 {
            return min;
        }
        if min > max {
            return self.frand_between(max, min);
        }
        min + self.frand() * (max - min)
    }

    /// Shuffles a vector using the Fisher-Yates algorithm.
    ///
    /// Adapted from the `nois` crate.
    ///
    /// See:
    ///  - <https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle>
    ///  - <https://docs.rs/nois/2.0.0/src/nois/shuffle.rs.html#64-71>
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    /// let mut numbers = vec![0, 1, 2, 3, 4, 5];
    ///
    /// rng.shuffle(&mut numbers);
    ///
    /// assert_eq!(numbers, [2, 5, 4, 1, 0, 3]);
    /// ```
    pub fn shuffle<T>(&mut self, data: &mut [T]) {
        for i in (1..data.len()).rev() {
            let j = (self.urand() as usize) % (i + 1);
            data.swap(i, j);
        }
    }

    /// Uniformly sample `n` elements out of `data`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::random::Rng;
    /// let mut rng = Rng::new_from_seed(92);
    /// let numbers = vec![0, 1, 2, 3, 4, 5];
    ///
    /// let sample = rng.sample(&numbers, 3);
    ///
    /// assert_eq!(sample, [2, 5, 4]);
    ///
    /// // Works for `n > numbers.len()`, too.
    /// let sample = rng.sample(&numbers, 10);
    ///
    /// assert_eq!(sample, [5, 0, 4, 2, 1, 3, 5, 3, 4, 1]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `data` is empty.
    #[must_use]
    pub fn sample<T: Clone>(&mut self, data: &[T], n: usize) -> Vec<T> {
        assert!(!data.is_empty(), "cannot sample from empty data");
        let mut data: Vec<T> = data.to_vec();
        let mut sample = Vec::with_capacity(n);
        // This _could_ be optimized by looping only if `n > data.len()`
        // (which would save the extra `cloned()`), but simplicity is
        // the goal here; no special case.
        while sample.len() < n {
            self.shuffle(&mut data);
            sample.extend(data.iter().take(n - sample.len()).cloned());
        }
        #[cfg(not(tarpaulin_include))] // Wrongly marked uncovered.
        {
            sample
        }
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_new() {
        let rng = Rng::new();
        assert_ne!(rng.random, 0);
    }

    #[test]
    fn rng_default() {
        let rng = Rng::default();
        assert_ne!(rng.random, 0);
    }

    #[test]
    fn rng_seeded() {
        let rng = Rng::new_from_seed(92);
        assert_eq!(rng.random, 92);
    }

    #[test]
    fn rng_seeded_is_deterministic() {
        let mut rng = Rng::new_from_seed(92);
        let mut numbers = Vec::with_capacity(20);

        for _ in 0..20 {
            numbers.push(rng.urand());
        }

        assert_eq!(
            numbers,
            [
                24_873_849,
                1_921_449_235,
                163_429_281,
                1_743_871_077,
                3_284_570_427,
                769_573_035,
                1_286_640_526,
                2_263_802_158,
                61_506_859,
                855_005_484,
                3_282_509_157,
                561_906_016,
                3_407_628_521,
                36_205_769,
                2_593_849_521,
                2_871_452_439,
                2_680_368_661,
                3_124_728_245,
                1_227_596_689,
                2_523_544_221
            ]
        );
    }

    #[test]
    fn rng_urand() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            _ = rng.urand();
        }
    }

    #[test]
    fn rng_irand() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            _ = rng.irand();
        }
    }

    #[test]
    fn rng_frand() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            _ = rng.frand();
        }
    }

    #[test]
    fn rng_urand_between() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.urand_between(10, 20);
            assert!((10..=20).contains(&res), "min=10, max=20, res={res}");
        }
    }

    #[test]
    fn rng_urand_between_min_eq_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.urand_between(20, 20);
            assert_eq!(res, 20, "min=20, max=20, res={res}");
        }
    }

    #[test]
    fn rng_urand_between_min_gt_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.urand_between(20, 10);
            assert!((10..=20).contains(&res), "min=10, max=20, res={res}");
        }
    }

    #[test]
    fn rng_irand_between_positive() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.irand_between(10, 20);
            assert!((10..=20).contains(&res), "min=10, max=20, res={res}");
        }
    }

    #[test]
    fn rng_irand_between_negative() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.irand_between(-20, -10);
            assert!((-20..=-10).contains(&res), "min=-20, max=-10, res={res}");
        }
    }

    #[test]
    fn rng_irand_between_negative_and_positive() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.irand_between(-20, 10);
            assert!((-20..=10).contains(&res), "min=-20, max=10, res={res}");
        }
    }

    #[test]
    fn rng_irand_between_min_eq_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.irand_between(20, 20);
            assert_eq!(res, 20, "min=20, max=20, res={res}");
        }
    }

    #[test]
    fn rng_irand_between_min_gt_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.irand_between(20, 10);
            assert!((10..=20).contains(&res), "min=10, max=20, res={res}");
        }
    }

    #[test]
    fn rng_frand_between_positive() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.frand_between(1.5, 3.5);
            assert!((1.5..=3.5).contains(&res), "min=1.5, max=3.5, res={res:.2}");
        }
    }

    #[test]
    fn rng_frand_between_negative() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.frand_between(-3.5, -1.5);
            assert!(
                (-3.5..=-1.5).contains(&res),
                "min=-3.5, max=-1.5, res={res:.2}"
            );
        }
    }

    #[test]
    fn rng_frand_between_negative_and_positive() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.frand_between(-3.5, 1.5);
            assert!(
                (-3.5..=1.5).contains(&res),
                "min=-3.5, max=1.5, res={res:.2}"
            );
        }
    }

    #[test]
    fn rng_frand_between_min_eq_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.frand_between(3.5, 3.5);
            assert!(
                (res - 3.5).abs() < 0.000_000_1,
                "min=3.5, max=3.5, res={res:.2}"
            );
        }
    }

    #[test]
    fn rng_frand_between_min_gt_max() {
        let mut rng = Rng::new();
        for _ in 0..100_000 {
            let res = rng.frand_between(3.5, 1.5);
            assert!((1.5..=3.5).contains(&res), "min=1.5, max=3.5, res={res:.2}");
        }
    }

    #[test]
    fn rng_shuffle() {
        let mut rng = Rng::new_from_seed(92);
        let mut numbers = vec![0, 1, 2, 3, 4, 5];

        rng.shuffle(&mut numbers);

        assert_ne!(numbers, [0, 1, 2, 3, 4, 5]);
        assert_eq!(numbers.len(), 6);

        // Ensure only order did change, not content.
        numbers.sort_unstable();
        assert_eq!(numbers, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn rng_sample() {
        let mut rng = Rng::new();
        let numbers = vec![0, 1, 2, 3, 4, 5];

        let sample = rng.sample(&numbers, 3);

        assert_eq!(sample.len(), 3);
        for x in sample {
            assert!(numbers.contains(&x));
        }
    }

    #[test]
    fn rng_sample_n_gt_data_len() {
        let mut rng = Rng::new();
        let numbers = vec![0, 1, 2, 3, 4, 5];

        let sample = rng.sample(&numbers, 10_000);

        assert_eq!(sample.len(), 10_000);
        for x in sample {
            assert!(numbers.contains(&x));
        }
    }

    #[test]
    #[should_panic(expected = "cannot sample from empty data")]
    fn rng_sample_empty_data_panics() {
        let mut rng = Rng::new();
        let numbers: Vec<i32> = Vec::new();

        _ = rng.sample(&numbers, 10);
    }
}
