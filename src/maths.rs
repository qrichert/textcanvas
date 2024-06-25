// `x`, `y`, `u`, `v`, etc. are standard notation.
#![allow(clippy::many_single_char_names)]

use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2D {
    x: f64,
    y: f64,
}

impl Vec2D {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn from_i32(x: i32, y: i32) -> Self {
        Self::new(f64::from(x), f64::from(y))
    }

    #[must_use]
    pub fn from_segment(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::new(x2 - x1, y2 - y1)
    }

    #[must_use]
    pub fn from_segment_i32(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self::from_i32(x2 - x1, y2 - y1)
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn to_i32(&self) -> (i32, i32) {
        (self.x.trunc() as i32, self.y.trunc() as i32)
    }

    #[must_use]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    #[must_use]
    pub fn one() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl Default for Vec2D {
    fn default() -> Self {
        Self::zero()
    }
}

impl ops::Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Neg for Vec2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl ops::Mul for Vec2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> ops::Mul<T> for Vec2D
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs.into(), self.y * rhs.into())
    }
}

impl ops::Mul<Vec2D> for f64 {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec2D> for i32 {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign for Vec2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> ops::MulAssign<T> for Vec2D
where
    T: Into<f64> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into();
        self.y *= rhs.into();
    }
}

impl ops::Div for Vec2D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> ops::Div<T> for Vec2D
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs.into(), self.y / rhs.into())
    }
}

impl ops::DivAssign for Vec2D {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> ops::DivAssign<T> for Vec2D
where
    T: Into<f64> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.into();
        self.y /= rhs.into();
    }
}

impl Vec2D {
    #[must_use]
    pub fn sum(vectors: &[Self]) -> Self {
        let mut acc = Self::zero();
        for vec in vectors {
            acc += *vec;
        }
        acc
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn mean(vectors: &[Self]) -> Self {
        let sum = Self::sum(vectors);
        sum / vectors.len() as f64
    }

    #[must_use]
    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y)
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let length = self.magnitude();
        *self / length
    }

    #[must_use]
    pub fn normal(&self) -> Self {
        let Self { x, y } = *self;
        Self::new(y, -x)
    }

    #[must_use]
    pub fn dot_product(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[must_use]
    pub fn projection_onto(&self, b: Self) -> f64 {
        // (a⋅b)/∥b∥^2
        let dot_product = self.dot_product(b);
        // magnitude(b) ** 2 involves a square root, canceled by "** 2".
        // It is more efficient to do it manually and avoid the sqrt().
        let squared_magnitude_of_b = b.x * b.x + b.y * b.y;
        dot_product / squared_magnitude_of_b
    }
}

pub struct Interpolation;

impl Interpolation {
    /// Linear.
    ///
    /// Find value given time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::lerp(10.0, 20.0, 0.5), 15.0);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `t` - Time [0; 1]
    #[must_use]
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        (1.0 - t) * a + t * b
    }

    /// Reverse Linear.
    ///
    /// Find time given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::rlerp(10.0, 20.0, 15.0), 0.5);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `v` - Value [start; end]
    #[must_use]
    pub fn rlerp(a: f64, b: f64, v: f64) -> f64 {
        (v - a) / (b - a)
    }

    /// Ease In Quad (^2) - Start slow, accelerate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.00), 0.0);
    /// assert_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.25), 6.25);
    /// assert_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.50), 25.0);
    /// assert_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.75), 56.25);
    /// assert_eq!(Interpolation::ease_in_quad(0.0, 100.0, 1.00), 100.0);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `t` - Time [0; 1]
    #[must_use]
    pub fn ease_in_quad(a: f64, b: f64, mut t: f64) -> f64 {
        t = t * t;
        Self::lerp(a, b, t)
    }

    /// Ease Out Quad (^2) - Start fast, decelerate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.00), 0.0);
    /// assert_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.25), 43.75);
    /// assert_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.50), 75.0);
    /// assert_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.75), 93.75);
    /// assert_eq!(Interpolation::ease_out_quad(0.0, 100.0, 1.00), 100.0);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `t` - Time [0; 1]
    #[must_use]
    pub fn ease_out_quad(a: f64, b: f64, mut t: f64) -> f64 {
        t = 1.0 - (1.0 - t) * (1.0 - t);
        Self::lerp(a, b, t)
    }

    /// Ease In-Out Quad (^2) - Start slow, accelerate, end slow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.00), 0.0);
    /// assert_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.25), 12.5);
    /// assert_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.50), 50.0);
    /// assert_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.75), 87.5);
    /// assert_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 1.00), 100.0);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `t` - Time [0; 1]
    #[must_use]
    pub fn ease_in_out_quad(a: f64, b: f64, mut t: f64) -> f64 {
        if t < 0.5 {
            t = 2.0 * t * t;
        } else {
            t = 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0;
        }
        Self::lerp(a, b, t)
    }

    /// Smoothstep (Smooth ease in-out).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::maths::Interpolation;
    /// assert_eq!(Interpolation::smoothstep(0.0, 100.0, 0.00), 0.0);
    /// assert_eq!(Interpolation::smoothstep(0.0, 100.0, 0.25), 15.625);
    /// assert_eq!(Interpolation::smoothstep(0.0, 100.0, 0.50), 50.0);
    /// assert_eq!(Interpolation::smoothstep(0.0, 100.0, 0.75), 84.375);
    /// assert_eq!(Interpolation::smoothstep(0.0, 100.0, 1.00), 100.0);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `a` - Start
    /// - `b` - End
    /// - `t` - Time [0; 1]
    #[must_use]
    pub fn smoothstep(a: f64, b: f64, mut t: f64) -> f64 {
        t = t * t * (3.0 - 2.0 * t);
        Self::lerp(a, b, t)
    }

    /// Catmull-Rom.
    ///
    /// ```text
    ///       P1              - P3
    ///       -  *          -
    ///     -     *      -
    /// P0 -        *  *
    ///                P2
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use textcanvas::maths::{Interpolation, Vec2D};
    /// let p0 = Vec2D::new(0.0, 0.25);
    /// let p1 = Vec2D::new(0.33, 0.85);
    /// let p2 = Vec2D::new(0.67, 0.15);
    /// let p3 = Vec2D::new(1.0, 0.75);
    ///
    /// assert_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 0.00, 0.5), p1);
    /// assert_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 0.25, 0.5), Vec2D::new(0.416, 0.740));
    /// assert_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 0.50, 0.5), Vec2D::new(0.5, 0.5));
    /// assert_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 0.75, 0.5), Vec2D::new(0.584, 0.260));
    /// assert_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 1.00, 0.5), p2);
    /// ```
    ///
    /// # Arguments
    ///
    /// - `p0` - Control point 1
    /// - `p1` - Spline start
    /// - `p2` - Spline end
    /// - `p3` - Control point 2
    /// - `t` - Time [0; 1]
    /// - `alpha` - 0.5 = centripetal, 0 = uniform, 1 = chordal
    #[must_use]
    pub fn catmull_rom(
        p0: Vec2D,
        p1: Vec2D,
        p2: Vec2D,
        p3: Vec2D,
        mut t: f64,
        alpha: f64,
    ) -> Vec2D {
        let get_t = |t: f64, alpha: f64, p0: Vec2D, p1: Vec2D| -> f64 {
            let d = p1 - p0;
            let a = d.dot_product(d);
            let b = a.powf(alpha * 0.5);
            b + t
        };

        let t0 = 0.0;
        let t1 = get_t(t0, alpha, p0, p1);
        let t2 = get_t(t1, alpha, p1, p2);
        let t3 = get_t(t2, alpha, p2, p3);
        t = Self::lerp(t1, t2, t);

        let a1 = ((t1 - t) / (t1 - t0) * p0) + ((t - t0) / (t1 - t0) * p1);
        let a2 = ((t2 - t) / (t2 - t1) * p1) + ((t - t1) / (t2 - t1) * p2);
        let a3 = ((t3 - t) / (t3 - t2) * p2) + ((t - t2) / (t3 - t2) * p3);
        let b1 = ((t2 - t) / (t2 - t0) * a1) + ((t - t0) / (t2 - t0) * a2);
        let b2 = ((t3 - t) / (t3 - t1) * a2) + ((t - t1) / (t3 - t1) * a3);

        ((t2 - t) / (t2 - t1) * b1) + ((t - t1) / (t2 - t1) * b2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr) => {
            assert!(($a - $b).abs() < f64::EPSILON, "{} != {}", $a, $b);
        };
    }

    // Vec2D.

    #[test]
    fn new() {
        let v = Vec2D::new(3.0, 6.0);

        assert_almost_eq!(v.x, 3.0);
        assert_almost_eq!(v.y, 6.0);
    }

    #[test]
    fn from_i32() {
        let v = Vec2D::from_i32(3, 6);

        assert_almost_eq!(v.x, 3.0);
        assert_almost_eq!(v.y, 6.0);
    }

    #[test]
    fn from_segment() {
        let v = Vec2D::from_segment(9.0, 2.0, 5.0, 7.0);

        assert_eq!(v, Vec2D::new(-4.0, 5.0));
    }

    #[test]
    fn from_segment_i32() {
        let v = Vec2D::from_segment_i32(9, 2, 5, 7);

        assert_eq!(v, Vec2D::new(-4.0, 5.0));
    }

    #[test]
    fn to_i32() {
        let v = Vec2D::new(3.0, 6.0);

        let (x, y) = v.to_i32();

        assert_eq!(x, 3);
        assert_eq!(y, 6);
    }

    #[test]
    fn zero() {
        assert_eq!(Vec2D::zero(), Vec2D::new(0.0, 0.0));
    }

    #[test]
    fn one() {
        assert_eq!(Vec2D::one(), Vec2D::new(1.0, 1.0));
    }

    #[test]
    fn default() {
        assert_eq!(Vec2D::default(), Vec2D::new(0.0, 0.0));
    }

    #[test]
    fn vec_add() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(u + v, Vec2D::new(3.0, 3.0));
        assert_eq!(v + v, Vec2D::new(4.0, 6.0));
    }

    #[test]
    fn vec_add_assign() {
        let mut u = Vec2D::new(1.0, 0.0);
        let mut v = Vec2D::new(2.0, 3.0);

        u += v;
        v += v;

        assert_eq!(u, Vec2D::new(3.0, 3.0));
        assert_eq!(v, Vec2D::new(4.0, 6.0));
    }

    #[test]
    fn vec_subtract() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(u - v, Vec2D::new(-1.0, -3.0));
        assert_eq!(v - u, Vec2D::new(1.0, 3.0));
        assert_eq!(v - v, Vec2D::new(0.0, 0.0));
    }

    #[test]
    fn vec_subtract_assign() {
        let mut u = Vec2D::new(1.0, 0.0);
        let mut v = Vec2D::new(2.0, 3.0);
        let mut w = Vec2D::new(2.0, 3.0);

        u -= v;
        v -= Vec2D::new(1.0, 0.0);
        w -= w;

        assert_eq!(u, Vec2D::new(-1.0, -3.0));
        assert_eq!(v, Vec2D::new(1.0, 3.0));
        assert_eq!(w, Vec2D::new(0.0, 0.0));
    }

    #[test]
    fn vec_negative() {
        let v = Vec2D::new(6.0, 9.0);

        assert_eq!(-v, Vec2D::new(-6.0, -9.0));
    }

    #[test]
    fn vec_multiply() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(u * v, Vec2D::new(2.0, 0.0));
        assert_eq!(v * v, Vec2D::new(4.0, 9.0));
    }

    #[test]
    fn vec_multiply_by_scalar() {
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(v * 3.0, Vec2D::new(6.0, 9.0));
        assert_eq!(v * 3, Vec2D::new(6.0, 9.0));
    }

    #[test]
    fn vec_multiply_scalar_by_vec() {
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(3.0 * v, Vec2D::new(6.0, 9.0));
        assert_eq!(3 * v, Vec2D::new(6.0, 9.0));
    }

    #[test]
    fn vec_multiply_assign() {
        let mut u = Vec2D::new(1.0, 0.0);
        let mut v = Vec2D::new(2.0, 3.0);

        u *= v;
        v *= v;

        assert_eq!(u, Vec2D::new(2.0, 0.0));
        assert_eq!(v, Vec2D::new(4.0, 9.0));
    }

    #[test]
    fn vec_multiply_by_scalar_assign() {
        let mut u = Vec2D::new(2.0, 3.0);
        let mut v = Vec2D::new(2.0, 3.0);

        u *= 3.0;
        v *= 3;

        assert_eq!(u, Vec2D::new(6.0, 9.0));
        assert_eq!(v, Vec2D::new(6.0, 9.0));
    }

    #[test]
    fn vec_divide() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(2.0, 3.0);

        assert_eq!(u / v, Vec2D::new(0.5, 0.0));
        assert_eq!(v / v, Vec2D::new(1.0, 1.0));
    }

    #[test]
    fn vec_divide_by_scalar() {
        let v = Vec2D::new(6.0, 9.0);

        assert_eq!(v / 3.0, Vec2D::new(2.0, 3.0));
        assert_eq!(v / 3, Vec2D::new(2.0, 3.0));
    }

    #[test]
    fn vec_divide_assign() {
        let mut u = Vec2D::new(1.0, 0.0);
        let mut v = Vec2D::new(2.0, 3.0);

        u /= v;
        v /= v;

        assert_eq!(u, Vec2D::new(0.5, 0.0));
        assert_eq!(v, Vec2D::new(1.0, 1.0));
    }

    #[test]
    fn vec_divide_by_scalar_assign() {
        let mut u = Vec2D::new(6.0, 9.0);
        let mut v = Vec2D::new(6.0, 9.0);

        u /= 3.0;
        v /= 3;

        assert_eq!(u, Vec2D::new(2.0, 3.0));
        assert_eq!(v, Vec2D::new(2.0, 3.0));
    }

    #[test]
    fn sum() {
        let vectors = [
            Vec2D::new(1.0, 0.0),
            Vec2D::new(2.0, 3.0),
            Vec2D::new(-1.0, -0.5),
        ];

        let sum = Vec2D::sum(&vectors);

        assert_eq!(sum, Vec2D::new(2.0, 2.5));
    }

    #[test]
    fn mean() {
        let vectors = [
            Vec2D::new(5.0, -9.5),
            Vec2D::new(2.0, 1.0),
            Vec2D::new(-1.0, -0.5),
        ];

        let mean = Vec2D::mean(&vectors);

        assert_eq!(mean, Vec2D::new(2.0, -3.0));
    }

    #[test]
    fn magnitude() {
        let v = Vec2D::new(3.0, 4.0);

        assert_almost_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn normalize() {
        let v = Vec2D::new(3.0, 4.0);

        assert_eq!(v.normalize(), Vec2D::new(0.6, 0.8));
    }

    #[test]
    fn normal() {
        let v = Vec2D::new(3.0, 4.0);

        assert_eq!(v.normal(), Vec2D::new(4.0, -3.0));
    }

    #[test]
    fn dot_product() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(-1.0, 0.0);
        let w = Vec2D::new(0.0, 1.0);
        let x = Vec2D::new(0.5, 0.5);
        let y = Vec2D::new(-0.5, -0.5);

        assert_almost_eq!(u.dot_product(u), 1.0);
        assert_almost_eq!(u.dot_product(v), -1.0);
        assert_almost_eq!(u.dot_product(w), 0.0);
        assert_almost_eq!(u.dot_product(x), 0.5);
        assert_almost_eq!(u.dot_product(y), -0.5);
    }

    #[test]
    fn projection_onto() {
        let u = Vec2D::new(1.0, 0.0);
        let v = Vec2D::new(-1.0, 0.0);
        let w = Vec2D::new(0.0, 1.0);
        let x = Vec2D::new(0.5, 0.5);
        let y = Vec2D::new(-0.5, -0.5);
        let z = Vec2D::new(2.0, 0.0);

        assert_almost_eq!(u.projection_onto(u), 1.0);
        assert_almost_eq!(v.projection_onto(u), -1.0);
        assert_almost_eq!(w.projection_onto(u), 0.0);
        assert_almost_eq!(x.projection_onto(u), 0.5);
        assert_almost_eq!(y.projection_onto(u), -0.5);
        assert_almost_eq!(z.projection_onto(u), 2.0);
    }

    // Interpolation.

    macro_rules! assert_vec_almost_eq {
        ($a:expr, $b:expr) => {
            assert_almost_eq!($a.x, $b.x);
            assert_almost_eq!($a.y, $b.y);
        };
    }

    #[test]
    fn lerp() {
        assert_almost_eq!(Interpolation::lerp(10.0, 20.0, 0.5), 15.0);
    }

    #[test]
    fn rlerp() {
        assert_almost_eq!(Interpolation::rlerp(10.0, 20.0, 15.0), 0.5);
    }

    #[test]
    fn ease_in_quad() {
        assert_almost_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.00), 0.0);
        assert_almost_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.25), 6.25);
        assert_almost_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.50), 25.0);
        assert_almost_eq!(Interpolation::ease_in_quad(0.0, 100.0, 0.75), 56.25);
        assert_almost_eq!(Interpolation::ease_in_quad(0.0, 100.0, 1.00), 100.0);
    }

    #[test]
    fn ease_out_quad() {
        assert_almost_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.00), 0.0);
        assert_almost_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.25), 43.75);
        assert_almost_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.50), 75.0);
        assert_almost_eq!(Interpolation::ease_out_quad(0.0, 100.0, 0.75), 93.75);
        assert_almost_eq!(Interpolation::ease_out_quad(0.0, 100.0, 1.00), 100.0);
    }

    #[test]
    fn ease_in_out_quad() {
        assert_almost_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.00), 0.0);
        assert_almost_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.25), 12.5);
        assert_almost_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.50), 50.0);
        assert_almost_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 0.75), 87.5);
        assert_almost_eq!(Interpolation::ease_in_out_quad(0.0, 100.0, 1.00), 100.0);
    }

    #[test]
    fn smoothstep() {
        assert_almost_eq!(Interpolation::smoothstep(0.0, 100.0, 0.00), 0.0);
        assert_almost_eq!(Interpolation::smoothstep(0.0, 100.0, 0.25), 15.625);
        assert_almost_eq!(Interpolation::smoothstep(0.0, 100.0, 0.50), 50.0);
        assert_almost_eq!(Interpolation::smoothstep(0.0, 100.0, 0.75), 84.375);
        assert_almost_eq!(Interpolation::smoothstep(0.0, 100.0, 1.00), 100.0);
    }

    #[test]
    fn catmull_rom() {
        let p0 = Vec2D::new(0.0, 0.25);
        let p1 = Vec2D::new(0.33, 0.85);
        let p2 = Vec2D::new(0.67, 0.15);
        let p3 = Vec2D::new(1.0, 0.75);

        assert_vec_almost_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 0.00, 0.5), p1);
        assert_vec_almost_eq!(
            Interpolation::catmull_rom(p0, p1, p2, p3, 0.25, 0.5),
            Vec2D::new(0.415_570_592_232_469_2, 0.739_802_500_912_719_5)
        );
        assert_vec_almost_eq!(
            Interpolation::catmull_rom(p0, p1, p2, p3, 0.50, 0.5),
            Vec2D::new(0.5, 0.5)
        );
        assert_vec_almost_eq!(
            Interpolation::catmull_rom(p0, p1, p2, p3, 0.75, 0.5),
            Vec2D::new(0.584_429_407_767_530_7, 0.260_197_499_087_280_35)
        );
        assert_vec_almost_eq!(Interpolation::catmull_rom(p0, p1, p2, p3, 1.00, 0.5), p2);
    }
}
