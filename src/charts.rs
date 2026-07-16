use crate::TextCanvas;

/// Find the minimum value of a `&[f64]`, concisely.
#[inline]
fn min_of(arr: &[f64]) -> Option<f64> {
    arr.iter().copied().min_by(f64::total_cmp)
}

/// Find the maximum value of a `&[f64]`, concisely.
#[inline]
fn max_of(arr: &[f64]) -> Option<f64> {
    arr.iter().copied().max_by(f64::total_cmp)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PlotType {
    Line,
    Scatter,
    Bars,
}

/// Helper functions to plot data on a [`TextCanvas`].
///
/// [`Plot`] does nothing magical. Calling functions on [`Plot`] is
/// exactly like drawing manually on the canvas. This entails that
/// nothing changes in the way you use the canvas before or after
/// plotting. Nor does it change the way you apply colors.
///
/// There are two classes of functions in `Plot`:
///
/// - Functions that take a discrete set of values as input.
/// - Functions that take a function as input (they all have `function`
///   in their name).
///
/// The main difference is that for those that take a discrete set as
/// input, `Plot` does nothing in particular. But for those that take a
/// function as input, `Plot` will be able to compute any value it needs
/// to plot the function with the highest precision possible.
///
/// # Note on auto-scaling
///
/// All the helper functions auto-scale the input data. The purpose of
/// this is to have a _quick_ and _simple_ way to graph things out.
///
/// Auto-scaling in this context means the lowest X value will be
/// plotted on the left border of the canvas, and the highest X value
/// will be plotted on the right side of the canvas, and all the values
/// in-between will be distributed uniformly. Same for Y.
///
/// If you absolutely need the plot to be smaller than the canvas, you
/// need to plot it to a _different_ canvas that has the target size,
/// and then draw the smaller canvas with the graph onto the parent
/// canvas. Use [`draw_canvas()`](TextCanvas::draw_canvas) or
/// [`merge_canvas()`](TextCanvas::merge_canvas) from [`TextCanvas`] to
/// do this easily.
pub struct Plot;

impl Plot {
    /// Stroke X and Y axes.
    ///
    /// If 0 is not visible on an axis, the axis will not be drawn.
    ///
    /// <div class="warning">
    ///
    /// `x` and `y` _should_ match in length,
    ///
    /// If `x` and `y` are not the same length, plotting will stop once
    /// the smallest of the two collections is consumed.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::stroke_xy_axes(&mut canvas, &x, &y);
    /// Plot::line(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠤⠒⠉
    /// ⠀⠀⠀⠀⠀⠀⠀⡇⢀⠤⠊⠁⠀⠀⠀
    /// ⠤⠤⠤⠤⠤⢤⠤⡯⠥⠤⠤⠤⠤⠤⠤
    /// ⠀⠀⢀⠤⠊⠁⠀⡇⠀⠀⠀⠀⠀⠀⠀
    /// ⡠⠊⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_xy_axes(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::stroke_x_axis(canvas, y);
        Self::stroke_y_axis(canvas, x);
    }

    /// Stroke X axis.
    ///
    /// See [`stroke_xy_axes()`](Self::stroke_xy_axes()) which has the
    /// same API for an example.
    ///
    /// # Arguments
    ///
    /// - `y` - Values of the Y axis, used to determine where Y = 0 is.
    pub fn stroke_x_axis(canvas: &mut TextCanvas, y: &[f64]) {
        Self::stroke_line_at_y(canvas, 0.0, y);
    }

    /// Stroke Y axis.
    ///
    /// See [`stroke_xy_axes()`](Self::stroke_xy_axes()) which has the
    /// same API for an example.
    ///
    /// # Arguments
    ///
    /// - `x` - Values of the X axis, used to determine where X = 0 is.
    pub fn stroke_y_axis(canvas: &mut TextCanvas, x: &[f64]) {
        Self::stroke_line_at_x(canvas, 0.0, x);
    }

    /// Stroke vertical line at X = value.
    ///
    /// If the value is out of the range of Y values, nothing will be
    /// drawn.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::stroke_line_at_x(&mut canvas, -5.0, &x);
    /// Plot::stroke_line_at_x(&mut canvas, -2.5, &x);
    /// Plot::stroke_line_at_x(&mut canvas, 0.0, &x);
    /// Plot::stroke_line_at_x(&mut canvas, 2.5, &x);
    /// Plot::stroke_line_at_x(&mut canvas, 5.0, &x);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// "
    /// );
    /// ```
    pub fn stroke_line_at_x(canvas: &mut TextCanvas, value: f64, x: &[f64]) {
        let Some(x) = Self::compute_screen_x(canvas, value, x) else {
            return;
        };
        canvas.stroke_line(x, 0, x, canvas.h());
    }

    /// Stroke horizontal line at Y = value.
    ///
    /// If the value is out of the range of Y values, nothing will be
    /// drawn.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::stroke_line_at_y(&mut canvas, -5.0, &y);
    /// Plot::stroke_line_at_y(&mut canvas, -2.5, &y);
    /// Plot::stroke_line_at_y(&mut canvas, 0.0, &y);
    /// Plot::stroke_line_at_y(&mut canvas, 2.5, &y);
    /// Plot::stroke_line_at_y(&mut canvas, 5.0, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
    /// ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
    /// ⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
    /// ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
    /// ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
    /// "
    /// );
    /// ```
    pub fn stroke_line_at_y(canvas: &mut TextCanvas, value: f64, y: &[f64]) {
        let Some(y) = Self::compute_screen_y(canvas, value, y) else {
            return;
        };
        canvas.stroke_line(0, y, canvas.w(), y);
    }

    /// Compute X position of a value on the canvas.
    ///
    /// Remember, values are auto-scaled to fit the canvas. If X goes
    /// from _-10_ to _10_, then:
    ///
    /// - Screen X of _-10_ will be 0
    /// - Screen X of _10_ will be canvas width
    /// - Screen X of _0_ will be canvas center X
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::{TextCanvas, charts::Plot};
    /// let canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-10..=10).map(f64::from).collect();
    ///
    /// assert_eq!(0, Plot::compute_screen_x(&canvas, -10.0, &x).unwrap());
    /// assert_eq!(29, Plot::compute_screen_x(&canvas, 10.0, &x).unwrap());
    /// assert_eq!(14, Plot::compute_screen_x(&canvas, 0.0, &x).unwrap());
    /// ```
    #[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
    #[must_use]
    pub fn compute_screen_x(canvas: &TextCanvas, value: f64, x: &[f64]) -> Option<i32> {
        if x.is_empty() {
            return None;
        }

        let min_x = min_of(x).expect("cannot be empty");
        let max_x = max_of(x).expect("cannot be empty");
        let range_x = max_x - min_x;
        let scale_x = canvas.fw() / range_x;

        // If `range = 0`. Division of a positive number by zero
        // results in +Inf.
        if scale_x.is_infinite() {
            return Some(canvas.cx());
        }

        // Shift data left, so that `min_x` would = 0, then scale so
        // that `max_x` would = width.
        let x = ((value - min_x) * scale_x).trunc() as i32;

        Some(x)
    }

    /// Compute Y position of a value on the canvas.
    ///
    /// Remember, values are auto-scaled to fit the canvas. If Y goes
    /// from _-10_ to _10_, then:
    ///
    /// - Screen X of _-10_ will be canvas height
    /// - Screen X of _10_ will be 0
    /// - Screen X of _0_ will be canvas center Y
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::{TextCanvas, charts::Plot};
    /// let canvas = TextCanvas::new(15, 5);
    ///
    /// let y: Vec<f64> = (-10..=10).map(f64::from).collect();
    ///
    /// assert_eq!(19, Plot::compute_screen_y(&canvas, -10.0, &y).unwrap());
    /// assert_eq!(0, Plot::compute_screen_y(&canvas, 10.0, &y).unwrap());
    /// assert_eq!(10, Plot::compute_screen_y(&canvas, 0.0, &y).unwrap());
    /// ```
    #[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
    #[must_use]
    pub fn compute_screen_y(canvas: &TextCanvas, value: f64, y: &[f64]) -> Option<i32> {
        if y.is_empty() {
            return None;
        }

        let min_y = min_of(y).expect("cannot be empty");
        let max_y = max_of(y).expect("cannot be empty");
        let range_y = max_y - min_y;
        let scale_y = canvas.fh() / range_y;

        // If `range = 0`. Division of a positive number by zero
        // results in +Inf.
        if scale_y.is_infinite() {
            return Some(canvas.cy());
        }

        // Shift data down, so that `min_y` would = 0, then scale so
        // that `max_y` would = height.
        let mut y = ((value - min_y) * scale_y).trunc() as i32;
        y = canvas.h() - y; // Y-axis is inverted.

        Some(y)
    }

    /// Stroke X and Y axes, given a function.
    ///
    /// The function is scaled to take up the entire canvas. The axes
    /// are then placed where _X_ and _Y_ = _0_.
    ///
    /// If 0 is not visible on an axis, the axis will not be drawn.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let f = |x: f64| x.sin();
    ///
    /// Plot::stroke_xy_axes_of_function(&mut canvas, -3.0, 7.0, &f);
    /// Plot::function(&mut canvas, -3.0, 7.0, &f);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⡇⢠⠋⠑⡄⠀⠀⠀⠀⠀⢀
    /// ⠀⠀⠀⠀⣇⠇⠀⠀⢱⠀⠀⠀⠀⠀⡎
    /// ⡤⠤⠤⠤⡿⠤⠤⠤⠤⡧⠤⠤⠤⡼⠤
    /// ⠸⡀⠀⢰⡇⠀⠀⠀⠀⠸⡀⠀⢠⠃⠀
    /// ⠀⠱⡠⠃⡇⠀⠀⠀⠀⠀⠑⠤⠊⠀⠀
    /// "
    /// );
    /// ```
    pub fn stroke_xy_axes_of_function(
        canvas: &mut TextCanvas,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        // `stroke_(x|y)_axis_of_function()` methods would both compute
        // the values of `f()`. It is more efficient to compute these
        // values once, and use the regular `stroke_(x|y)_axis()`
        // methods instead.
        let nb_values = canvas.screen.fwidth();
        let (x, y) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::stroke_x_axis(canvas, &y);
        Self::stroke_y_axis(canvas, &x);
    }

    /// Stroke X axis, given a function.
    ///
    /// See [`stroke_xy_axes_of_function()`](Self::stroke_xy_axes_of_function())
    /// which has the same API for an example.
    pub fn stroke_x_axis_of_function(
        canvas: &mut TextCanvas,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        Self::stroke_line_at_y_of_function(canvas, 0.0, from_x, to_x, f);
    }

    /// Stroke Y axis, given a function.
    ///
    /// See [`stroke_xy_axes_of_function()`](Self::stroke_xy_axes_of_function())
    /// which has the same API for an example.
    pub fn stroke_y_axis_of_function(
        canvas: &mut TextCanvas,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        Self::stroke_line_at_x_of_function(canvas, 0.0, from_x, to_x, f);
    }

    /// Stroke vertical line at X = value, given a function.
    ///
    /// Same as [`stroke_line_at_x()`](Self::stroke_line_at_x()), but
    /// for a function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let f = |x| x;
    ///
    /// Plot::stroke_line_at_x_of_function(&mut canvas, -5.0, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_x_of_function(&mut canvas, -2.5, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_x_of_function(&mut canvas, 0.0, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_x_of_function(&mut canvas, 2.5, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_x_of_function(&mut canvas, 5.0, -5.0, 5.0, &f);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// ⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
    /// "
    /// );
    /// ```
    pub fn stroke_line_at_x_of_function(
        canvas: &mut TextCanvas,
        value: f64,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        let nb_values = canvas.screen.fwidth();
        let (x, _) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::stroke_line_at_x(canvas, value, &x);
    }

    /// Stroke horizontal line at Y = value, given a function.
    ///
    /// Same as [`stroke_line_at_y()`](Self::stroke_line_at_y()), but
    /// for a function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let f = |x| x;
    ///
    /// Plot::stroke_line_at_y_of_function(&mut canvas, -5.0, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_y_of_function(&mut canvas, -2.5, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_y_of_function(&mut canvas, 0.0, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_y_of_function(&mut canvas, 2.5, -5.0, 5.0, &f);
    /// Plot::stroke_line_at_y_of_function(&mut canvas, 5.0, -5.0, 5.0, &f);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
    /// ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
    /// ⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
    /// ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
    /// ⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
    /// "
    /// );
    /// ```
    pub fn stroke_line_at_y_of_function(
        canvas: &mut TextCanvas,
        value: f64,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        let nb_values = canvas.screen.fwidth();
        let (_, y) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::stroke_line_at_y(canvas, value, &y);
    }

    /// Compute X position of a value on the canvas, given a function.
    ///
    /// Same as [`compute_screen_x()`](Self::compute_screen_x), but for
    /// a function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::{TextCanvas, charts::Plot};
    /// let canvas = TextCanvas::new(15, 5);
    ///
    /// let f = |x| x;
    ///
    /// assert_eq!(0, Plot::compute_screen_x_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap());
    /// assert_eq!(14, Plot::compute_screen_x_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap());
    /// assert_eq!(29, Plot::compute_screen_x_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap());
    /// ```
    pub fn compute_screen_x_of_function(
        canvas: &TextCanvas,
        value: f64,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) -> Option<i32> {
        let nb_values = canvas.screen.fwidth();
        let (x, _) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::compute_screen_x(canvas, value, &x)
    }

    /// Compute Y position of a value on the canvas, given a function.
    ///
    /// Same as [`compute_screen_y()`](Self::compute_screen_y), but for
    /// a function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::{TextCanvas, charts::Plot};
    /// let canvas = TextCanvas::new(15, 5);
    ///
    /// let f = |x| x;
    ///
    /// assert_eq!(19, Plot::compute_screen_y_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap());
    /// assert_eq!(10, Plot::compute_screen_y_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap());
    /// assert_eq!(0, Plot::compute_screen_y_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap());
    /// ```
    pub fn compute_screen_y_of_function(
        canvas: &TextCanvas,
        value: f64,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) -> Option<i32> {
        let nb_values = canvas.screen.fwidth();
        let (_, y) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::compute_screen_y(canvas, value, &y)
    }

    /// Plot line-joined points.
    ///
    /// The data is scaled to take up the entire canvas.
    ///
    /// <div class="warning">
    ///
    /// `x` and `y` _should_ match in length,
    ///
    /// If `x` and `y` are not the same length, plotting will stop once
    /// the smallest of the two collections is consumed.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::line(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    ///⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠒⠉
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⡠⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn line(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::plot(canvas, x, y, PlotType::Line);
    }

    /// Plot scattered points.
    ///
    /// The data is scaled to take up the entire canvas.
    ///
    /// <div class="warning">
    ///
    /// `x` and `y` _should_ match in length,
    ///
    /// If `x` and `y` are not the same length, plotting will stop once
    /// the smallest of the two collections is consumed.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::scatter(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠂⠈
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠂⠀⠀⠀⠀
    /// ⠀⠀⠀⠀⠀⢀⠀⠂⠀⠀⠀⠀⠀⠀⠀
    /// ⠀⠀⢀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// ⡀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn scatter(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::plot(canvas, x, y, PlotType::Scatter);
    }

    /// Plot bars.
    ///
    /// The data is scaled to take up the entire canvas.
    ///
    /// <div class="warning">
    ///
    /// `x` and `y` _should_ match in length,
    ///
    /// If `x` and `y` are not the same length, plotting will stop once
    /// the smallest of the two collections is consumed.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Plot::bars(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⡆⢸
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⡆⢸⠀⡇⢸
    /// ⠀⠀⠀⠀⠀⢀⠀⡆⢸⠀⡇⢸⠀⡇⢸
    /// ⠀⠀⢀⠀⡆⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
    /// ⡀⡆⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
    /// "
    /// );
    /// ```
    pub fn bars(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::plot(canvas, x, y, PlotType::Bars);
    }

    #[allow(clippy::cast_possible_truncation)]
    fn plot(canvas: &mut TextCanvas, x: &[f64], y: &[f64], plot_type: PlotType) {
        if x.is_empty() || y.is_empty() {
            return;
        }

        // `.copied()` is necessary to get `(f64, f64)` instead of `(&f64, &f64)`.
        let mut points: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
        if plot_type == PlotType::Line {
            // Sort by `x`.
            points.sort_by(|a, b| f64::total_cmp(&a.0, &b.0));
        }

        let min_x = min_of(x).expect("cannot be empty");
        let max_x = max_of(x).expect("cannot be empty");
        let range_x = max_x - min_x;
        let scale_x = canvas.fw() / range_x;

        let min_y = min_of(y).expect("cannot be empty");
        let max_y = max_of(y).expect("cannot be empty");
        let range_y = max_y - min_y;
        let scale_y = canvas.fh() / range_y;

        // If `range = 0`. Division of a positive number by zero
        // results in +Inf.
        if scale_x.is_infinite() || scale_y.is_infinite() {
            // One or both axis have no range. This doesn't make sense
            // for plotting with auto-scale.
            return Self::handle_axes_without_range(
                canvas,
                x,
                y,
                plot_type,
                scale_x.is_infinite(),
                scale_y.is_infinite(),
            );
        }

        let mut previous: Option<(i32, i32)> = None; // For line plot.
        for (x, y) in points {
            let mut x = x;
            // Shift data left so that `min_x` = 0, then scale so that
            // `max_x` = width.
            x = (x - min_x) * scale_x;
            let x = x.trunc() as i32;

            let mut y = y;
            y = (y - min_y) * scale_y;
            y = canvas.fh() - y; // Y-axis is inverted.
            let y = y.trunc() as i32;

            match plot_type {
                PlotType::Line => {
                    let pair = (x, y);

                    if let Some(previous) = previous {
                        canvas.stroke_line(previous.0, previous.1, pair.0, pair.1);
                    }

                    previous = Some(pair);
                }
                PlotType::Scatter => {
                    canvas.set_pixel(x, y, true);
                }
                PlotType::Bars => {
                    canvas.stroke_line(x, y, x, canvas.h());
                }
            }
        }
    }

    fn handle_axes_without_range(
        canvas: &mut TextCanvas,
        x: &[f64],
        y: &[f64],
        plot_type: PlotType,
        x_has_no_range: bool,
        y_has_no_range: bool,
    ) {
        let x_has_range_but_not_y = !x_has_no_range && y_has_no_range;
        let y_has_range_but_not_x = x_has_no_range && !y_has_no_range;
        let both_have_no_range = x_has_no_range && y_has_no_range;

        if x_has_range_but_not_y {
            // Y is a constant, draw a single centered line.
            Self::draw_horizontally_centered_line(canvas, x, plot_type);
        } else if y_has_range_but_not_x {
            // Compress all Ys into a single centered line.
            Self::draw_vertically_centered_line(canvas, y, plot_type);
        } else if both_have_no_range {
            // Draw a dot in the middle to show the user we tried to do
            // something, but the values are off.
            canvas.set_pixel(canvas.cx(), canvas.cy(), true);

            if plot_type == PlotType::Bars {
                // Add the bar for bar plots.
                canvas.stroke_line(canvas.cx(), canvas.cy(), canvas.cx(), canvas.h());
            }
        }
    }

    /// Draw all points at the same Y coordinate.
    ///
    /// This is a fallback for when the data has no range on the Y axis.
    fn draw_horizontally_centered_line(canvas: &mut TextCanvas, x: &[f64], plot_type: PlotType) {
        match plot_type {
            PlotType::Line => {
                canvas.stroke_line(0, canvas.cy(), canvas.w(), canvas.cy());
            }
            PlotType::Scatter => {
                for &x_val in x {
                    if let Some(x) = Self::compute_screen_x(canvas, x_val, x) {
                        canvas.set_pixel(x, canvas.cy(), true);
                    }
                }
            }
            PlotType::Bars => {
                for &x_val in x {
                    if let Some(x) = Self::compute_screen_x(canvas, x_val, x) {
                        canvas.stroke_line(x, canvas.cy(), x, canvas.h());
                    }
                }
            }
        }
    }

    /// Draw all points at the same X coordinate.
    ///
    /// This is a fallback for when the data has no range on the X axis.
    fn draw_vertically_centered_line(canvas: &mut TextCanvas, y: &[f64], plot_type: PlotType) {
        match plot_type {
            PlotType::Line | PlotType::Bars => {
                canvas.stroke_line(canvas.cx(), 0, canvas.cx(), canvas.h());
            }
            PlotType::Scatter => {
                for &y_val in y {
                    if let Some(y) = Self::compute_screen_y(canvas, y_val, y) {
                        canvas.set_pixel(canvas.cx(), y, true);
                    }
                }
            }
        }
    }

    /// Plot a function.
    ///
    /// The function is scaled to take up the entire canvas, and is
    /// assumed to be continuous (points will be line-joined together).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// Plot::function(&mut canvas, -10.0, 10.0, &|x| x * x);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜
    /// ⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀
    /// ⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⡔⠁⠀
    /// ⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀
    /// ⠀⠀⠀⠀⠈⠒⠤⣀⠤⠒⠁⠀⠀⠀⠀
    /// "
    /// );
    /// ```
    pub fn function(canvas: &mut TextCanvas, from_x: f64, to_x: f64, f: &impl Fn(f64) -> f64) {
        let nb_values = canvas.screen.fwidth();
        let (x, y) = Self::compute_function(from_x, to_x, nb_values, f);
        Self::line(canvas, &x, &y);
    }

    /// Plot a function, and fill the area under the curve.
    ///
    /// The function is scaled to take up the entire canvas, and is
    /// assumed to be continuous (points will be line-joined together).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{TextCanvas, charts::Plot};
    ///
    /// let mut canvas = TextCanvas::new(15, 5);
    ///
    /// Plot::function_filled(&mut canvas, -10.0, 10.0, &|x| x * x);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿
    /// ⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿
    /// ⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿
    /// ⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿
    /// ⣿⣿⣿⣿⣿⣦⣤⣠⣤⣾⣿⣿⣿⣿⣿
    /// "
    /// );
    /// ```
    pub fn function_filled(
        canvas: &mut TextCanvas,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        let nb_values = canvas.screen.fwidth();
        // Increase density to prevent "holes" due to rounding (missing
        // values because one would round lower, and the other higher).
        let nb_values = nb_values * 1.07;
        let (x, y) = Self::compute_function(from_x, to_x, nb_values, f);
        // This is a "trick". Since we've just computed the value of the
        // function for every horizontal pixel, we can now plot the
        // points as bars to fill up the whole area under the curve.
        Self::bars(canvas, &x, &y);
    }

    /// Compute the values of a function.
    ///
    /// This is mainly used internally to compute values for functions.
    ///
    /// However, it may also be useful in case one wants to pre-compute
    /// values.
    ///
    /// # Note
    ///
    /// The return value of the function is generic. You can use
    /// [`compute_function()`](Plot::compute_function) to compute
    /// anything, but if the values of Y are not `f64`s, you will need
    /// to adapt them before use.
    ///
    /// This is useful for optimisation. Say you have an expensive
    /// function that returns a `struct` with multiple fields. If only
    /// `f64`s were allowed, you would have to re-compute the exact same
    /// function for each field of the struct. But thanks to the generic
    /// return type, you can compute the function _once_, and extract
    /// the fields into separate vectors by `map()`ping the values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use textcanvas::{TextCanvas, charts::Plot};
    /// # let mut canvas = TextCanvas::new(15, 5);
    /// # let mut canvas2 = TextCanvas::new(15, 5);
    /// #
    /// let f = |x: f64| x.sin();
    ///
    /// // This is inefficient, because `f()` will be computed twice.
    /// Plot::stroke_xy_axes_of_function(&mut canvas, -3.0, 7.0, &f);
    /// Plot::function(&mut canvas, -3.0, 7.0, &f);
    ///
    /// // This is better, the values are computed only once.
    /// let (x, y) = Plot::compute_function(-3.0, 7.0, canvas2.screen.fwidth(), &f);
    /// Plot::stroke_xy_axes(&mut canvas2, &x, &y);
    /// Plot::line(&mut canvas2, &x, &y);
    ///
    /// assert_eq!(canvas.to_string(), canvas2.to_string());
    /// ```
    ///
    /// Note that the "inefficient" solution is unlikely to cause a
    /// noticeable performance hit. The simpler approach is most often
    /// the better approach.
    pub fn compute_function<T>(
        from_x: f64,
        to_x: f64,
        nb_values: f64,
        f: &impl Fn(f64) -> T,
    ) -> (Vec<f64>, Vec<T>) {
        let range = to_x - from_x;
        // If we want 5 values in a range including bounds, we need to
        // divide the range into 4 equal pieces:
        //   1   2   3   4
        // |   |   |   |   |
        // 1   2   3   4   5
        let step = range / (nb_values - 1.0);

        // This is fine. `nb_values` will realistically never be big
        // enough to overflow `usize`, and even then, this is just for
        // pre-allocation.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let nb_values = nb_values.ceil() as usize;
        let mut px: Vec<f64> = Vec::with_capacity(nb_values);
        let mut py: Vec<T> = Vec::with_capacity(nb_values);

        let mut x = from_x;
        for _ in 0..(nb_values - 1) {
            px.push(x);
            py.push(f(x));

            x += step;
        }

        // Add exact last value to compensate for errors accumulated by
        // `+= step` over many iterations (hence `0..(nb_values - 1)`).
        px.push(to_x);
        py.push(f(to_x));

        debug_assert_eq!(px.len(), nb_values);
        debug_assert_eq!(px.len(), py.len());

        (px, py)
    }
}

/// Helper functions to render charts on a [`TextCanvas`].
///
/// Basically, this renders a [`Plot`] and makes it pretty.
///
/// The idea comes from <https://github.com/sunetos/TextPlots.jl>.
pub struct Chart;

impl Chart {
    const MARGIN_TOP: i32 = 1;
    const MARGIN_RIGHT: i32 = 2;
    const MARGIN_BOTTOM: i32 = 2;
    const MARGIN_LEFT: i32 = 10;

    const HORIZONTAL_MARGIN: i32 = Self::MARGIN_LEFT + Self::MARGIN_RIGHT;
    const VERTICAL_MARGIN: i32 = Self::MARGIN_TOP + Self::MARGIN_BOTTOM;

    /// Render chart with a line plot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{charts::Chart, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(35, 10);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Chart::line(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠒⠉⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠊⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⡠⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢀⡠⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if chart is < 13×4, because it would make plot < 1×1.
    pub fn line(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::chart(canvas, x, y, PlotType::Line);
    }

    /// Render chart with a scatter plot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{charts::Chart, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(35, 10);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Chart::scatter(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠈⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠈⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠠⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if chart is < 13×4, because it would make plot < 1×1.
    pub fn scatter(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::chart(canvas, x, y, PlotType::Scatter);
    }

    /// Render chart with a bars plot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{charts::Chart, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(35, 10);
    ///
    /// let x: Vec<f64> = (-5..=5).map(f64::from).collect();
    /// let y: Vec<f64> = (-5..=5).map(f64::from).collect();
    ///
    /// Chart::bars(&mut canvas, &x, &y);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡄⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⢠⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢰⠀⢸⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡀⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
    /// ⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if chart is < 13×4, because it would make plot < 1×1.
    pub fn bars(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        Self::chart(canvas, x, y, PlotType::Bars);
    }

    fn chart(canvas: &mut TextCanvas, x: &[f64], y: &[f64], plot_type: PlotType) {
        if x.is_empty() || y.is_empty() {
            return;
        }
        Self::check_canvas_size(canvas);
        Self::plot_values(canvas, x, y, plot_type);
        Self::stroke_plot_border(canvas);
        Self::draw_min_and_max_values(canvas, x, y);
    }

    fn check_canvas_size(canvas: &TextCanvas) {
        let width = canvas.output.width();
        let height = canvas.output.height();
        let min_width = Self::HORIZONTAL_MARGIN + 1;
        let min_height = Self::VERTICAL_MARGIN + 1;
        assert!(
            width >= min_width && height >= min_height,
            "Canvas size is {width}×{height}, but must be at least {min_width}×{min_height} to accommodate for plot."
        );
    }

    fn plot_values(canvas: &mut TextCanvas, x: &[f64], y: &[f64], plot_type: PlotType) {
        let width = canvas.output.width() - Self::HORIZONTAL_MARGIN;
        let height = canvas.output.height() - Self::VERTICAL_MARGIN;

        let mut plot = TextCanvas::new(width, height);

        match plot_type {
            PlotType::Line => {
                Plot::line(&mut plot, x, y);
            }
            PlotType::Scatter => {
                Plot::scatter(&mut plot, x, y);
            }
            PlotType::Bars => {
                Plot::bars(&mut plot, x, y);
            }
        }

        canvas.draw_canvas(&plot, Self::MARGIN_LEFT * 2, Self::MARGIN_TOP * 4);
    }

    fn stroke_plot_border(canvas: &mut TextCanvas) {
        let top = (Self::MARGIN_TOP - 1) * 4 + 2;
        let right = canvas.w() - (Self::MARGIN_RIGHT - 1) * 2;
        let bottom = canvas.h() - ((Self::MARGIN_BOTTOM - 1) * 4 + 2);
        let left = (Self::MARGIN_LEFT - 1) * 2;

        canvas.stroke_line(left, top, right, top);
        canvas.stroke_line(right, top, right, bottom);
        canvas.stroke_line(right, bottom, left, bottom);
        canvas.stroke_line(left, bottom, left, top);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn draw_min_and_max_values(canvas: &mut TextCanvas, x: &[f64], y: &[f64]) {
        let min_x = Self::format_number(min_of(x).expect("cannot be empty"));
        let max_x = Self::format_number(max_of(x).expect("cannot be empty"));
        let min_y = Self::format_number(min_of(y).expect("cannot be empty"));
        let max_y = Self::format_number(max_of(y).expect("cannot be empty"));

        canvas.draw_text(
            &min_x,
            Self::MARGIN_LEFT - (min_x.len() as i32),
            canvas.output.height() - Self::MARGIN_TOP,
        );
        canvas.draw_text(
            &max_x,
            canvas.output.width() - Self::MARGIN_RIGHT + 2 - (max_x.len() as i32),
            canvas.output.height() - Self::MARGIN_TOP,
        );
        canvas.draw_text(
            &min_y,
            Self::MARGIN_LEFT - 2 - (min_y.len() as i32),
            canvas.output.height() - Self::MARGIN_TOP - 1,
        );
        canvas.draw_text(
            &max_y,
            Self::MARGIN_LEFT - 2 - (max_y.len() as i32),
            Self::MARGIN_TOP - 1,
        );
    }

    fn format_number(mut number: f64) -> String {
        let mut precision = 1;
        let mut suffix = "";
        if number.abs() >= 1_000_000_000_000.0 {
            number /= 1_000_000_000_000.0;
            suffix = "T";
        } else if number.abs() >= 1_000_000_000.0 {
            number /= 1_000_000_000.0;
            suffix = "B";
        } else if number.abs() >= 1_000_000.0 {
            number /= 1_000_000.0;
            suffix = "M";
        } else if number.abs() >= 10_000.0 {
            number /= 1000.0;
            suffix = "K";
        } else if (number - number.round()).abs() < 0.001 {
            precision = 0; // Close enough to being round for display.
            if number.abs() < 0.000_1 {
                number = 0.0; // Prevent "-0".
            }
        } else if number.abs() < 1.0 {
            precision = 4; // Sub-1 decimals matter a lot.
        }

        format!("{number:.precision$}{suffix}")
    }

    /// Render chart with a function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{charts::Chart, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(35, 10);
    ///
    /// let f = |x: f64| x.cos();
    ///
    /// Chart::function(&mut canvas, 0.0, 5.0, &f);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀1⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠉⠉⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠃⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⡀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⠤⡠⠤⠒⠁⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀-1⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀0⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if chart is < 13×4, because it would make plot < 1×1.
    pub fn function(canvas: &mut TextCanvas, from_x: f64, to_x: f64, f: &impl Fn(f64) -> f64) {
        let nb_values = f64::from((canvas.output.width() - Self::HORIZONTAL_MARGIN) * 2);
        let (x, y) = Plot::compute_function(from_x, to_x, nb_values, f);
        Self::line(canvas, &x, &y);
    }

    /// Render chart with a function, and fill the area under the curve.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use textcanvas::{charts::Chart, TextCanvas};
    ///
    /// let mut canvas = TextCanvas::new(35, 10);
    ///
    /// let f = |x: f64| x.cos();
    ///
    /// Chart::function_filled(&mut canvas, 0.0, 5.0, &f);
    ///
    /// assert_eq!(
    ///     canvas.to_string(),
    ///     "\
    /// ⠀⠀⠀⠀⠀⠀⠀1⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⢸⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣤⣤⣤⣾⣿⣿⣿⣿⣿⣿⢸⠀
    /// ⠀⠀⠀⠀⠀⠀-1⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
    /// ⠀⠀⠀⠀⠀⠀⠀⠀⠀0⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
    /// "
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if chart is < 13×4, because it would make plot < 1×1.
    pub fn function_filled(
        canvas: &mut TextCanvas,
        from_x: f64,
        to_x: f64,
        f: &impl Fn(f64) -> f64,
    ) {
        let nb_values = f64::from((canvas.output.width() - Self::HORIZONTAL_MARGIN) * 2);
        // Increase density to prevent "holes" due to rounding (missing
        // values because one would round lower, and the other higher).
        let nb_values = nb_values * 1.07;
        let (x, y) = Plot::compute_function(from_x, to_x, nb_values, f);
        // This is a "trick". Since we've just computed the value of the
        // function for every horizontal pixel, we can now plot the
        // points as bars to fill up the whole area under the curve.
        Self::bars(canvas, &x, &y);
    }
}

/// Helper functions to resample data.
///
/// Rendering too many data points can quickly lead to messy charts.
/// Downsampling aims at reducing the number of data points, while
/// trying to preserve the essence of the data (e.g., curve and
/// distribution should look similar).
///
/// Resampling is very idiosyncratic to the dataset, and so is not done
/// automatically by [`Plot`].
pub struct Resampling;

impl Resampling {
    /// Downsample data using the mean technique.
    ///
    /// Mean downsampling reduces the number of values by averaging them
    /// out. The data points are split into `n` buckets (where `n` is
    /// the target resolution), and for each bucket we keep the mean of
    /// the values.
    ///
    /// Compared to min/max downsampling for instance, mean will
    /// smoothen the data, and lose information about local minima and
    /// maxima in the process.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::charts::Resampling;
    /// let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    /// let y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    ///
    /// let (x, y) = Resampling::downsample_mean(&x, &y, 4);
    ///
    /// assert_eq!(x, [0.0, 1.5, 3.5, 5.0]);
    /// assert_eq!(y, [1.0, 2.5, 4.5, 6.0]);
    /// ```
    ///
    /// # Notes
    ///
    /// This implementation keeps the first and last points in the data
    /// unchanged. Thus, the resulting graphs will always start and end
    /// at the exact same values.
    ///
    /// # Pitfalls
    ///
    /// The caller _should_ ensure the data is sorted, otherwise he will
    /// probably get inconsistent results.
    ///
    /// # Panics
    ///
    /// This function panics if `max_nb_points` is `< 2`.
    #[must_use]
    pub fn downsample_mean(x: &[f64], y: &[f64], max_nb_points: usize) -> (Vec<f64>, Vec<f64>) {
        let points: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
        let points = Self::downsample_points_mean(&points, max_nb_points);
        let (x, y) = points.into_iter().unzip();
        (x, y)
    }

    /// Downsample data using the mean technique.
    ///
    /// Same as [`Self::downsample_mean()`], with another signature.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::charts::Resampling;
    /// let points = [
    ///     (0.0, 1.0), (1.0, 2.0), (2.0, 3.0),
    ///     (3.0, 4.0), (4.0, 5.0), (5.0, 6.0),
    /// ];
    ///
    /// let downsampled = Resampling::downsample_points_mean(&points, 4);
    ///
    /// assert_eq!(downsampled, [(0.0, 1.0), (1.5, 2.5), (3.5, 4.5), (5.0, 6.0)]);
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if `max_nb_points` is `< 2`.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn downsample_points_mean(points: &[(f64, f64)], max_nb_points: usize) -> Vec<(f64, f64)> {
        assert!(
            max_nb_points >= 2,
            "minimum two points are required as output"
        );

        if points.len() <= max_nb_points {
            return points.to_owned();
        }
        // Prevent divide-by-zero issues.
        if max_nb_points - 2 == 0 {
            return vec![points[0], points[points.len() - 1]];
        }

        // `- 2` to exclude first and last.
        let nb_points = (points.len() - 2) as f64;
        let nb_buckets = (max_nb_points - 2) as f64;

        // _ceil_ so `bucket_size` is large enough to never leave rest.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bucket_size = (nb_points / nb_buckets).ceil() as usize;

        let mut downsampled_points = Vec::with_capacity(max_nb_points);
        downsampled_points.push(points.first().copied().expect("min 2 points"));

        for bucket in points[1..points.len() - 1].chunks(bucket_size) {
            let mean_x = bucket.iter().fold(0.0, |acc, &(x, _)| acc + x) / (bucket.len() as f64);
            let mean_y = bucket.iter().fold(0.0, |acc, &(_, y)| acc + y) / (bucket.len() as f64);
            downsampled_points.push((mean_x, mean_y));
        }

        downsampled_points.push(points.last().copied().expect("min 2 points"));

        downsampled_points
    }

    /// Downsample data using the min/max technique.
    ///
    /// The idea behind min/max downsampling is to preserve the local
    /// peaks and trophes in the data. The data points are split into
    /// `n` buckets (where `n` is the target resolution divided by 2),
    /// and for each bucket we keep the minimum and maximum values.
    ///
    /// Compared to mean downsampling for instance, min/max will render
    /// the noise, while mean would smooth it out, losing information
    /// about local minima and maxima.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::charts::Resampling;
    /// let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    /// let y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    ///
    /// let (x, y) = Resampling::downsample_min_max(&x, &y, 4);
    ///
    /// assert_eq!(x, [0.0, 1.0, 4.0, 5.0]);
    /// assert_eq!(y, [1.0, 2.0, 5.0, 6.0]);
    /// ```
    ///
    /// # Notes
    ///
    /// This implementation keeps the first and last points in the data
    /// unchanged. Thus, the resulting graphs will always start and end
    /// at the exact same values.
    ///
    /// This implementation also preserves the ordering of the minimum
    /// and maximum values in a bucket. This means that if the minimum
    /// comes before the maximum in the input, it will also come before
    /// it in the output, same the other way around.
    ///
    /// # Pitfalls
    ///
    /// The caller _should_ ensure the data is sorted, otherwise he will
    /// probably get inconsistent results.
    ///
    /// `max_nb_points` _must_ be even. Points always come in pairs
    /// (min/max), it doesn't make sense to cap the data at an odd
    /// length.
    ///
    /// # Panics
    ///
    /// This function panics if `max_nb_points` is `< 2` or is odd.
    #[must_use]
    pub fn downsample_min_max(x: &[f64], y: &[f64], max_nb_points: usize) -> (Vec<f64>, Vec<f64>) {
        let points: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
        let points = Self::downsample_points_min_max(&points, max_nb_points);
        let (x, y) = points.into_iter().unzip();
        (x, y)
    }

    /// Downsample data using the min/max technique.
    ///
    /// Same as [`Self::downsample_min_max()`], with another signature.
    ///
    /// # Examples
    ///
    /// ```
    /// # use textcanvas::charts::Resampling;
    /// let points = [
    ///     (0.0, 1.0), (1.0, 2.0), (2.0, 3.0),
    ///     (3.0, 4.0), (4.0, 5.0), (5.0, 6.0),
    /// ];
    ///
    /// let downsampled = Resampling::downsample_points_min_max(&points, 4);
    ///
    /// assert_eq!(downsampled, [(0.0, 1.0), (1.0, 2.0), (4.0, 5.0), (5.0, 6.0)]);
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if `max_nb_points` is `< 2` or is odd.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn downsample_points_min_max(
        points: &[(f64, f64)],
        max_nb_points: usize,
    ) -> Vec<(f64, f64)> {
        assert!(
            max_nb_points >= 2,
            "minimum two points are required as output"
        );
        assert_eq!(max_nb_points % 2, 0, "number of output points must be even");

        if points.len() <= max_nb_points {
            return points.to_owned();
        }
        // Prevent divide-by-zero issues.
        if max_nb_points - 2 == 0 {
            return vec![points[0], points[points.len() - 1]];
        }

        // `- 2` to exclude first and last.
        let nb_points = (points.len() - 2) as f64;
        let nb_buckets = (max_nb_points - 2) as f64 / 2.0; // Buckets yield 2 points: min/max

        // _ceil_ so `bucket_size` is large enough to never leave rest.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bucket_size = (nb_points / nb_buckets).ceil() as usize;

        let mut downsampled_points = Vec::with_capacity(max_nb_points);
        downsampled_points.push(points.first().copied().expect("min 2 points"));

        for bucket in points[1..points.len() - 1].chunks(bucket_size) {
            let mut bucket = bucket.iter();
            let &first_point = bucket.next().expect("bucket is non-empty");

            let (mut min, mut max) = (first_point, first_point);

            for &point in bucket {
                if point.1 < min.1 {
                    min = point;
                }
                if point.1 > max.1 {
                    max = point;
                }
            }

            // Preserve original order based on X value.
            if min.0 <= max.0 {
                downsampled_points.extend([min, max]);
            } else {
                // `nursery` lint error, may get fixed sometime.
                #[allow(clippy::tuple_array_conversions)]
                downsampled_points.extend([max, min]);
            }
        }

        downsampled_points.push(points.last().copied().expect("min 2 points"));

        downsampled_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stroke_x_and_y_axes() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_x_axis_at_top_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = (-5..=0).map(f64::from).collect();

        Plot::stroke_x_axis(&mut canvas, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_x_axis_at_bottom_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = (0..=5).map(f64::from).collect();

        Plot::stroke_x_axis(&mut canvas, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
"
        );
    }

    #[test]
    fn stroke_y_axis_at_left_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (0..=5).map(f64::from).collect();

        Plot::stroke_y_axis(&mut canvas, &x);

        assert_eq!(
            canvas.to_string(),
            "\
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_y_axis_at_right_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=0).map(f64::from).collect();

        Plot::stroke_y_axis(&mut canvas, &x);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
"
        );
    }

    #[test]
    fn stroke_line_at_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_line_at_x(&mut canvas, -5.0, &x);
        Plot::stroke_line_at_x(&mut canvas, -2.5, &x);
        Plot::stroke_line_at_x(&mut canvas, 0.0, &x);
        Plot::stroke_line_at_x(&mut canvas, 2.5, &x);
        Plot::stroke_line_at_x(&mut canvas, 5.0, &x);

        assert_eq!(
            canvas.to_string(),
            "\
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
"
        );
    }

    #[test]
    fn stroke_line_at_x_ignore_empty_values() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![];

        Plot::stroke_line_at_x(&mut canvas, 0.0, &x);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_line_at_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_line_at_y(&mut canvas, -5.0, &y);
        Plot::stroke_line_at_y(&mut canvas, -2.5, &y);
        Plot::stroke_line_at_y(&mut canvas, 0.0, &y);
        Plot::stroke_line_at_y(&mut canvas, 2.5, &y);
        Plot::stroke_line_at_y(&mut canvas, 5.0, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
"
        );
    }

    #[test]
    fn stroke_line_at_y_ignore_empty_values() {
        let mut canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = vec![];

        Plot::stroke_line_at_y(&mut canvas, 0.0, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn compute_screen_x() {
        let canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-10..=10).map(f64::from).collect();

        assert_eq!(0, Plot::compute_screen_x(&canvas, -10.0, &x).unwrap());
        assert_eq!(29, Plot::compute_screen_x(&canvas, 10.0, &x).unwrap());
        assert_eq!(14, Plot::compute_screen_x(&canvas, 0.0, &x).unwrap());
    }

    #[test]
    fn compute_screen_x_input_size_1() {
        let canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![3.0];

        assert_eq!(15, Plot::compute_screen_x(&canvas, 0.0, &x).unwrap());
    }

    #[test]
    fn compute_screen_x_empty_input() {
        let canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![];

        assert!(Plot::compute_screen_x(&canvas, 0.0, &x).is_none());
    }

    #[test]
    fn compute_screen_y() {
        let canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = (-10..=10).map(f64::from).collect();

        assert_eq!(19, Plot::compute_screen_y(&canvas, -10.0, &y).unwrap());
        assert_eq!(0, Plot::compute_screen_y(&canvas, 10.0, &y).unwrap());
        assert_eq!(10, Plot::compute_screen_y(&canvas, 0.0, &y).unwrap());
    }

    #[test]
    fn compute_screen_y_input_size_1() {
        let canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = vec![3.0];

        assert_eq!(10, Plot::compute_screen_y(&canvas, 0.0, &y).unwrap());
    }

    #[test]
    fn compute_screen_y_empty_input() {
        let canvas = TextCanvas::new(15, 5);

        let y: Vec<f64> = vec![];

        assert!(Plot::compute_screen_y(&canvas, 0.0, &y).is_none());
    }

    #[test]
    fn stroke_x_and_y_axes_of_function() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_xy_axes_of_function(&mut canvas, -5.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_x_axis_of_function_at_top_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_x_axis_of_function(&mut canvas, -5.0, 0.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_x_axis_of_function_at_bottom_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_x_axis_of_function(&mut canvas, 0.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
"
        );
    }

    #[test]
    fn stroke_y_axis_of_function_at_left_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_y_axis_of_function(&mut canvas, 0.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_y_axis_of_function_at_right_boundary() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_y_axis_of_function(&mut canvas, -5.0, 0.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
"
        );
    }

    #[test]
    fn stroke_line_at_x_of_function() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_line_at_x_of_function(&mut canvas, -5.0, -5.0, 5.0, &f);
        Plot::stroke_line_at_x_of_function(&mut canvas, -2.5, -5.0, 5.0, &f);
        Plot::stroke_line_at_x_of_function(&mut canvas, 0.0, -5.0, 5.0, &f);
        Plot::stroke_line_at_x_of_function(&mut canvas, 2.5, -5.0, 5.0, &f);
        Plot::stroke_line_at_x_of_function(&mut canvas, 5.0, -5.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
⡇⠀⠀⢸⠀⠀⠀⡇⠀⠀⢸⠀⠀⠀⢸
"
        );
    }

    #[test]
    fn stroke_line_at_x_of_function_value_out_of_bounds() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_line_at_x_of_function(&mut canvas, -100.0, -5.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn stroke_line_at_y_of_function() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_line_at_y_of_function(&mut canvas, -5.0, -5.0, 5.0, &f);
        Plot::stroke_line_at_y_of_function(&mut canvas, -2.5, -5.0, 5.0, &f);
        Plot::stroke_line_at_y_of_function(&mut canvas, 0.0, -5.0, 5.0, &f);
        Plot::stroke_line_at_y_of_function(&mut canvas, 2.5, -5.0, 5.0, &f);
        Plot::stroke_line_at_y_of_function(&mut canvas, 5.0, -5.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
"
        );
    }

    #[test]
    fn stroke_line_at_y_of_function_value_out_of_bounds() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        Plot::stroke_line_at_y_of_function(&mut canvas, -100.0, -5.0, 5.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn compute_screen_x_of_function() {
        let canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        assert_eq!(
            0,
            Plot::compute_screen_x_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            14,
            Plot::compute_screen_x_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            29,
            Plot::compute_screen_x_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap()
        );
    }

    #[test]
    fn compute_screen_x_of_function_range_0() {
        let canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        assert_eq!(
            15,
            Plot::compute_screen_x_of_function(&canvas, -10.0, 0.0, 0.0, &f).unwrap()
        );
        assert_eq!(
            15,
            Plot::compute_screen_x_of_function(&canvas, 0.0, 0.0, 0.0, &f).unwrap()
        );
        assert_eq!(
            15,
            Plot::compute_screen_x_of_function(&canvas, 10.0, 0.0, 0.0, &f).unwrap()
        );
    }

    #[test]
    fn compute_screen_x_of_function_canvas_size_1x1() {
        let canvas = TextCanvas::new(1, 1);

        let f = |x| x;

        assert_eq!(
            0,
            Plot::compute_screen_x_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            0,
            Plot::compute_screen_x_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            1,
            Plot::compute_screen_x_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap()
        );
    }

    #[test]
    fn compute_screen_y_of_function() {
        let canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        assert_eq!(
            19,
            Plot::compute_screen_y_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            10,
            Plot::compute_screen_y_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            0,
            Plot::compute_screen_y_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap()
        );
    }

    #[test]
    fn compute_screen_y_of_function_range_0() {
        let canvas = TextCanvas::new(15, 5);

        let f = |x| x;

        assert_eq!(
            10,
            Plot::compute_screen_y_of_function(&canvas, -10.0, 0.0, 0.0, &f).unwrap()
        );
        assert_eq!(
            10,
            Plot::compute_screen_y_of_function(&canvas, 0.0, 0.0, 0.0, &f).unwrap()
        );
        assert_eq!(
            10,
            Plot::compute_screen_y_of_function(&canvas, 10.0, 0.0, 0.0, &f).unwrap()
        );
    }

    #[test]
    fn compute_screen_y_of_function_canvas_size_1x1() {
        let canvas = TextCanvas::new(1, 1);

        let f = |x| x;

        assert_eq!(
            3,
            Plot::compute_screen_y_of_function(&canvas, -10.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            2,
            Plot::compute_screen_y_of_function(&canvas, 0.0, -10.0, 10.0, &f).unwrap()
        );
        assert_eq!(
            0,
            Plot::compute_screen_y_of_function(&canvas, 10.0, -10.0, 10.0, &f).unwrap()
        );
    }

    #[test]
    fn plot_line() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠤⠒⠉
⠀⠀⠀⠀⠀⠀⠀⡇⢀⠤⠊⠁⠀⠀⠀
⠤⠤⠤⠤⠤⢤⠤⡯⠥⠤⠤⠤⠤⠤⠤
⠀⠀⢀⠤⠊⠁⠀⡇⠀⠀⠀⠀⠀⠀⠀
⡠⠊⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_empty_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![];
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_empty_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = vec![];

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_sorts_elements_by_x_before_plotting() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![-5.0, 5.0, -2.5];
        let y: Vec<f64> = vec![5.0, 2.5, -2.5];

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        // Not sorted, it would look like this:
        // ⠉⠑⠒⠒⠤⠤⢄⣇⡀⠀⠀⠀⠀⠀⠀
        // ⠀⠀⠀⠀⠀⠀⠀⡇⠈⠉⠉⠒⠒⢢⡤
        // ⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⣀⠤⠊⠁⠀
        // ⠒⠒⠒⠒⠒⠒⢒⡷⠖⠚⠒⠒⠒⠒⠒
        // ⠀⠀⠀⢀⠤⠒⠁⡇⠀⠀⠀⠀⠀⠀⠀
        assert_eq!(
            canvas.to_string(),
            "\
⢣⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠈⢆⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⡠
⠀⠘⡄⠀⠀⠀⠀⡇⠀⠀⣀⠤⠊⠁⠀
⠒⠒⠳⡒⠒⠒⢒⡷⠖⠛⠒⠒⠒⠒⠒
⠀⠀⠀⢣⠤⠒⠁⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_single_value() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![0.0];
        let y: Vec<f64> = vec![0.0];

        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_range_xy_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_range_x_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_range_y_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::line(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_x_and_y_of_different_lengths_more_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-10..=10).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        // The scale is correct. At X = 0, Y = 5. To see values on the
        // right, you'd have to increase the range of Y (up to 15, to
        // match X).
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⢀⠔⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡠⠊⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⢤⠴⠥⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤
⠀⡠⠊⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⡰⠁⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_line_with_x_and_y_of_different_lengths_more_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-10..=10).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::line(&mut canvas, &x, &y);

        // The scale is correct. Y range is [-10;10], (0;10) is just
        // not rendered because X stops when Y = 0. If you'd continue
        // to the right, Y would reach 10 at X = 15.
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⣤⡤⠤⠶
⠀⠀⠀⠀⠀⣀⡠⡧⠒⠊⠉⠀⠀⠀⠀
⡠⠤⠒⠊⠉⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠀⠂⠈
⠀⠀⠀⠀⠀⠀⠀⡇⢀⠀⠂⠀⠀⠀⠀
⠤⠤⠤⠤⠤⢤⠤⡧⠤⠤⠤⠤⠤⠤⠤
⠀⠀⢀⠀⠂⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⡀⠂⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_empty_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![];
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_empty_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = vec![];

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_single_value() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![0.0];
        let y: Vec<f64> = vec![0.0];

        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_range_xy_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_range_x_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠨⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢨⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_range_y_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⠄⠠⠀⠄⠠⠀⠄⠠⠀⠄⠠⠀⠄⠠
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_x_and_y_of_different_lengths_more_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-10..=10).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::scatter(&mut canvas, &x, &y);

        // The scale is correct. At X = 0, Y = 5. To see values on the
        // right, you'd have to increase the range of Y (up to 15, to
        // match X).
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⢀⠐⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡀⠂⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⢤⠴⠤⠤⠤⡧⠤⠤⠤⠤⠤⠤⠤
⠀⡀⠂⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⡐⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_scatter_with_x_and_y_of_different_lengths_more_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-10..=10).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::scatter(&mut canvas, &x, &y);

        // The scale is correct. Y range is [-10;10], (0;10) is just
        // not rendered because X stops when Y = 0. If you'd continue
        // to the right, Y would reach 10 at X = 15.
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⢤⠤⠤⠴
⠀⠀⠀⠀⠀⢀⠀⡇⠐⠀⠁⠀⠀⠀⠀
⡀⠄⠐⠀⠁⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⠀⡆⢸
⠀⠀⠀⠀⠀⠀⠀⡇⢀⠀⡆⢸⠀⡇⢸
⠤⠤⠤⠤⠤⢤⠤⡧⢼⠤⡧⢼⠤⡧⢼
⠀⠀⢀⠀⡆⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
⡀⡆⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
"
        );
    }

    #[test]
    fn plot_bars_with_empty_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![];
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_empty_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = vec![];

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_single_value() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = vec![0.0];
        let y: Vec<f64> = vec![0.0];

        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_range_xy_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_range_x_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(|_| 0.0).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_range_y_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(|_| 0.0).collect();

        Plot::bars(&mut canvas, &x, &y);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡄⡄⢠⠀⡄⢠⠀⡄⢠⠀⡄⢠⠀⡄⢠
⡇⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
⡇⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
"
        );
    }

    #[test]
    fn plot_bars_with_x_and_y_of_different_lengths_more_x() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-10..=10).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::bars(&mut canvas, &x, &y);

        // The scale is correct. At X = 0, Y = 5. To see values on the
        // right, you'd have to increase the range of Y (up to 15, to
        // match X).
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⢀⢰⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡀⣾⢸⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⢤⢴⡧⣿⢼⡧⠤⠤⠤⠤⠤⠤⠤
⠀⡀⣾⢸⡇⣿⢸⡇⠀⠀⠀⠀⠀⠀⠀
⣰⡇⣿⢸⡇⣿⢸⡇⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_bars_with_x_and_y_of_different_lengths_more_y() {
        let mut canvas = TextCanvas::new(15, 5);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-10..=10).map(f64::from).collect();

        Plot::stroke_xy_axes(&mut canvas, &x, &y);
        Plot::bars(&mut canvas, &x, &y);

        // The scale is correct. Y range is [-10;10], (0;10) is just
        // not rendered because X stops when Y = 0. If you'd continue
        // to the right, Y would reach 10 at X = 15.
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⡧⠤⠤⠤⢤⠤⡤⢴
⠀⠀⠀⠀⠀⢀⠀⡇⢰⠀⡇⢸⠀⡇⢸
⡀⡄⢰⠀⡇⢸⠀⡇⢸⠀⡇⢸⠀⡇⢸
"
        );
    }

    #[test]
    fn plot_function() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x * x;

        Plot::stroke_xy_axes_of_function(&mut canvas, -10.0, 10.0, &f);
        Plot::function(&mut canvas, -10.0, 10.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠱⡀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡜
⠀⢣⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⡜⠀
⠀⠀⠣⡀⠀⠀⠀⡇⠀⠀⠀⠀⡔⠁⠀
⠀⠀⠀⠑⡄⠀⠀⡇⠀⠀⢀⠎⠀⠀⠀
⣀⣀⣀⣀⣈⣒⣤⣇⣤⣒⣁⣀⣀⣀⣀
"
        );
    }

    #[test]
    fn plot_function_with_single_value() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |_| 0.0;

        Plot::function(&mut canvas, 0.0, 0.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_function_with_range_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |_| 0.0;

        Plot::function(&mut canvas, -10.0, 10.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_function_filled() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |x| x * x;

        Plot::stroke_xy_axes_of_function(&mut canvas, -10.0, 10.0, &f);
        Plot::function_filled(&mut canvas, -10.0, 10.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢠⣿
⣿⡄⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⣼⣿
⣿⣿⣄⠀⠀⠀⠀⡇⠀⠀⠀⢀⣼⣿⣿
⣿⣿⣿⣦⡀⠀⠀⡇⠀⠀⣠⣾⣿⣿⣿
⣿⣿⣿⣿⣿⣦⣤⣧⣤⣾⣿⣿⣿⣿⣿
"
        );
    }

    #[test]
    fn plot_function_filled_with_single_value() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |_| 0.0;

        Plot::function_filled(&mut canvas, 0.0, 0.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    fn plot_function_filled_with_range_zero() {
        let mut canvas = TextCanvas::new(15, 5);

        let f = |_| 0.0;

        Plot::function_filled(&mut canvas, -10.0, 10.0, &f);

        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"
        );
    }

    #[test]
    fn compute_function_works_with_structs() {
        #[derive(Debug, PartialEq)]
        struct Mock {
            foo: f64,
            bar: f64,
        }

        let f = |x: f64| Mock { foo: x, bar: -x };

        // Compute all values once. Y will contain structs.
        let (x, y) = Plot::compute_function(-5.0, 5.0, 5.0, &f);

        assert_eq!(x, vec![-5.0, -2.5, 0.0, 2.5, 5.0]);
        assert_eq!(
            y,
            vec![
                Mock {
                    foo: -5.0,
                    bar: 5.0
                },
                Mock {
                    foo: -2.5,
                    bar: 2.5
                },
                Mock {
                    foo: 0.0,
                    bar: -0.0
                },
                Mock {
                    foo: 2.5,
                    bar: -2.5
                },
                Mock {
                    foo: 5.0,
                    bar: -5.0
                }
            ]
        );

        // Extract struct fields.
        let y_foo: Vec<f64> = y.iter().map(|mock| mock.foo).collect();
        let y_bar: Vec<f64> = y.iter().map(|mock| mock.bar).collect();

        assert_eq!(y_foo, vec![-5.0, -2.5, 0.0, 2.5, 5.0]);
        assert_eq!(y_bar, vec![5.0, 2.5, -0.0, -2.5, -5.0]);
    }

    #[test]
    fn chart_function_x_squared() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x| x * x;

        Chart::function(&mut canvas, -10.0, 10.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀100⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠊⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠈⢢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠑⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠒⠢⠤⠤⢄⡠⠤⠤⠴⠒⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀0.0073⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀-10⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀10
"
        );
    }

    #[test]
    fn chart_function_polynomial() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x: f64| x.powi(3) - 2.0 * x.powi(2) + 3.0 * x;

        Chart::function(&mut canvas, -5.0, 5.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀90⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⡠⠤⠤⠔⠒⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠤⠒⠒⠒⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠔⠚⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀-190⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_function_cos() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x: f64| x.cos();

        Chart::function(&mut canvas, 0.0, 5.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀1⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠉⠉⠉⠒⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠙⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠒⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠢⠤⠤⢄⠤⠤⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀-1⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀0⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_function_filled_x_squared() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x| x * x;

        Chart::function_filled(&mut canvas, -10.0, 10.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀100⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⣤⣤⣤⣠⣤⣤⣤⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀0.0035⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀-10⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀10
"
        );
    }

    #[test]
    fn chart_function_filled_polynomial() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x: f64| x.powi(3) - 2.0 * x.powi(2) + 3.0 * x;

        Chart::function_filled(&mut canvas, -5.0, 5.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀90⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣾⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣠⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣤⣴⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀-190⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_function_filled_cos() {
        let mut canvas = TextCanvas::new(71, 19);

        let f = |x: f64| x.cos();

        Chart::function_filled(&mut canvas, 0.0, 5.0, &f);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀1⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣶⣤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣤⣤⣄⣤⣤⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢸⠀
⠀⠀⠀⠀⠀⠀-1⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀0⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_line() {
        let mut canvas = TextCanvas::new(35, 10);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::line(&mut canvas, &x, &y);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠒⠉⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠊⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠒⠉⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⡠⠔⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢀⡠⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_scatter() {
        let mut canvas = TextCanvas::new(35, 10);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::scatter(&mut canvas, &x, &y);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠈⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠈⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠠⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_bars() {
        let mut canvas = TextCanvas::new(35, 10);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::bars(&mut canvas, &x, &y);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀5⠀⡤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⢤⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡄⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⢠⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢰⠀⢸⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡀⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⠀⢸⠀⠀⡇⠀⡇⠀⢸⢸⠀
⠀⠀⠀⠀⠀⠀-5⠀⠓⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠀
⠀⠀⠀⠀⠀⠀⠀⠀-5⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀5
"
        );
    }

    #[test]
    fn chart_empty() {
        let mut canvas = TextCanvas::new(35, 10);

        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];

        Chart::line(&mut canvas, &x, &y);
        Chart::scatter(&mut canvas, &x, &y);

        println!("{canvas}");
        assert_eq!(
            canvas.to_string(),
            "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"
        );
    }

    #[test]
    #[should_panic(
        expected = "Canvas size is 12×3, but must be at least 13×4 to accommodate for plot."
    )]
    fn chart_canvas_too_small_both_horizontally_and_vertically() {
        let mut canvas = TextCanvas::new(Chart::HORIZONTAL_MARGIN, Chart::VERTICAL_MARGIN);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::scatter(&mut canvas, &x, &y);
    }

    #[test]
    #[should_panic(
        expected = "Canvas size is 12×4, but must be at least 13×4 to accommodate for plot."
    )]
    fn chart_canvas_too_small_horizontally() {
        let mut canvas = TextCanvas::new(Chart::HORIZONTAL_MARGIN, Chart::VERTICAL_MARGIN + 1);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::line(&mut canvas, &x, &y);
    }

    #[test]
    #[should_panic(
        expected = "Canvas size is 13×3, but must be at least 13×4 to accommodate for plot."
    )]
    fn chart_canvas_too_small_vertically() {
        let mut canvas = TextCanvas::new(Chart::HORIZONTAL_MARGIN + 1, Chart::VERTICAL_MARGIN);

        let x: Vec<f64> = (-5..=5).map(f64::from).collect();
        let y: Vec<f64> = (-5..=5).map(f64::from).collect();

        Chart::line(&mut canvas, &x, &y);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn chart_pretty_number() {
        assert_eq!(Chart::format_number(1_570_000_000_000.0), "1.6T");
        assert_eq!(Chart::format_number(1_000_000_000_000.0), "1.0T");

        assert_eq!(Chart::format_number(1_570_000_000.0), "1.6B");
        assert_eq!(Chart::format_number(1_000_000_000.0), "1.0B");

        assert_eq!(Chart::format_number(1_570_000.0), "1.6M");
        assert_eq!(Chart::format_number(1_000_000.0), "1.0M");

        assert_eq!(Chart::format_number(100_000.0), "100.0K");

        assert_eq!(Chart::format_number(10_570.0), "10.6K");
        assert_eq!(Chart::format_number(10_000.0), "10.0K");

        assert_eq!(Chart::format_number(1_570.0), "1570");
        assert_eq!(Chart::format_number(1_000.0), "1000");

        assert_eq!(Chart::format_number(1.0009), "1");
        assert_eq!(Chart::format_number(-1.0009), "-1");

        assert_eq!(Chart::format_number(0.010_57), "0.0106");
        assert_eq!(Chart::format_number(0.010_00), "0.0100");

        assert_eq!(Chart::format_number(0.000_001_57), "0");
        assert_eq!(Chart::format_number(0.000_001_00), "0");

        assert_eq!(Chart::format_number(-0.000_001_57), "0");
        assert_eq!(Chart::format_number(-0.000_001_00), "0");

        assert_eq!(Chart::format_number(-0.001_57), "-0.0016");
        assert_eq!(Chart::format_number(-0.001_00), "-0.0010");

        assert_eq!(Chart::format_number(-1_570.0), "-1570");
        assert_eq!(Chart::format_number(-1_000.0), "-1000");

        assert_eq!(Chart::format_number(-10_570.0), "-10.6K");
        assert_eq!(Chart::format_number(-10_000.0), "-10.0K");

        assert_eq!(Chart::format_number(-100_000.0), "-100.0K");

        assert_eq!(Chart::format_number(-1_570_000.0), "-1.6M");
        assert_eq!(Chart::format_number(-1_000_000.0), "-1.0M");

        assert_eq!(Chart::format_number(-1_570_000_000.0), "-1.6B");
        assert_eq!(Chart::format_number(-1_000_000_000.0), "-1.0B");

        assert_eq!(Chart::format_number(-1_570_000_000_000.0), "-1.6T");
        assert_eq!(Chart::format_number(-1_000_000_000_000.0), "-1.0T");
    }

    #[test]
    fn downsample_mean_regular() {
        let points = [
            // 1 point.
            (0.0, 0.0),
            // 1 point.
            (1.0, 3.0),
            (2.0, -1.0),
            (3.0, -4.0),
            (4.0, 6.0),
            (5.0, 1.0),
            // 1 point.
            (6.0, 7.0),
            (7.0, -4.0),
            (8.0, -2.0),
            (9.0, 2.5),
            // 1 point.
            (10.0, 0.0),
        ];
        let (x, y): (Vec<f64>, Vec<f64>) = points.iter().copied().unzip();

        let res = Resampling::downsample_mean(&x, &y, 4);

        let res_points = Resampling::downsample_points_mean(&points, 4);
        let res_points: (Vec<f64>, Vec<f64>) = res_points.into_iter().unzip();

        // `downsample_mean()` uses `downsample_points_mean()` under
        // the hood. We just ensure they are equal.
        assert_eq!(res, res_points);
    }

    #[test]
    fn downsample_points_mean_regular() {
        let points = [
            // 1 point.
            (0.0, 0.0),
            // 1 point.
            (1.0, 3.0),
            (2.0, -1.0),
            (3.0, -4.0),
            (4.0, 6.0),
            (5.0, 1.0),
            // 1 point.
            (6.0, 7.0),
            (7.0, -4.0),
            (8.0, -2.0),
            (9.0, 2.5),
            // 1 point.
            (10.0, 0.0),
        ];

        let res = Resampling::downsample_points_mean(&points, 4);

        assert_eq!(
            res,
            [
                // First.
                (0.0, 0.0),
                // Bucket 1.
                (3.0, 1.0),
                // Bucket 2.
                (7.5, 0.875),
                // Last.
                (10.0, 0.0),
            ]
        );
    }

    #[test]
    fn downsample_points_mean_no_op_nb_points_lt_max_nb_points() {
        let points = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (4.0, 0.0)];

        let res = Resampling::downsample_points_mean(&points, 6);

        assert_eq!(res, points);
    }

    #[test]
    fn downsample_points_mean_keep_only_first_and_last() {
        let points = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)];

        let res = Resampling::downsample_points_mean(&points, 2);

        assert_eq!(res, [(0.0, 0.0), (3.0, 0.0)]);
    }

    #[test]
    #[should_panic(expected = "minimum two points are required as output")]
    fn downsample_points_mean_error_max_nb_points_lt_2() {
        _ = Resampling::downsample_points_mean(&[], 2); // OK
        _ = Resampling::downsample_points_mean(&[], 1);
    }

    #[test]
    fn plot_data_with_downsampling_mean() {
        let f = |x: f64| x.sin();

        // Compute lots of values.
        let (x, y) = Plot::compute_function(0.0, std::f64::consts::TAU, 1000.0, &f);

        let mut canvas = TextCanvas::new(15, 5);
        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(x.len(), 1000);
        assert_eq!(
            canvas.to_string(),
            "\
⠀⣠⠞⠉⠙⢦⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣰⠃⠀⠀⠀⠈⢧⠀⠀⠀⠀⠀⠀⠀⠀
⠃⠀⠀⠀⠀⠀⠈⢧⠀⠀⠀⠀⠀⢀⡖
⠀⠀⠀⠀⠀⠀⠀⠈⢧⠀⠀⠀⢀⡞⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⢤⠴⠋⠀⠀
"
        );

        let mut canvas_downsampled = TextCanvas::new(15, 5);

        let points: Vec<(f64, f64)> = x.iter().copied().zip(y).collect();
        let points = Resampling::downsample_points_mean(&points, 30);
        let (x, y): (Vec<f64>, Vec<f64>) = points.into_iter().unzip();

        Plot::scatter(&mut canvas_downsampled, &x, &y);

        // 1000 points downsampled to 30.
        assert_eq!(x.len(), 30);
        assert_eq!(
            canvas_downsampled.to_string(),
            "\
⠀⠠⠊⠉⠑⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠠⠁⠀⠀⠀⠀⠡⠀⠀⠀⠀⠀⠀⠀⠀
⠃⠀⠀⠀⠀⠀⠀⠡⠀⠀⠀⠀⠀⠀⠔
⠀⠀⠀⠀⠀⠀⠀⠀⠡⠀⠀⠀⢀⠌⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠤⠂⠀⠀
"
        );
    }

    #[test]
    fn downsample_min_max_regular() {
        let points = [
            // 1 point.
            (0.0, 0.0),
            // 2 points (min/max).
            (1.0, 3.0),
            (2.0, -1.0),
            (3.0, -4.0),
            (4.0, 6.0),
            (5.0, 1.0),
            // 2 points (min/max).
            (6.0, 7.0),
            (7.0, -4.0),
            (8.0, -2.0),
            (9.0, 2.5),
            // 1 point.
            (10.0, 0.0),
        ];
        let (x, y): (Vec<f64>, Vec<f64>) = points.iter().copied().unzip();

        let res = Resampling::downsample_min_max(&x, &y, 4);

        let res_points = Resampling::downsample_points_min_max(&points, 4);
        let res_points: (Vec<f64>, Vec<f64>) = res_points.into_iter().unzip();

        // `downsample_min_max()` uses `downsample_points_min_max()`
        // under the hood. We just ensure they are equal.
        assert_eq!(res, res_points);
    }

    #[test]
    fn downsample_points_min_max_regular() {
        let points = [
            // 1 point.
            (0.0, 0.0),
            // 2 points (min/max).
            (1.0, 3.0),
            (2.0, -1.0),
            (3.0, -4.0),
            (4.0, 6.0),
            (5.0, 1.0),
            // 2 points (min/max).
            (6.0, 7.0),
            (7.0, -4.0),
            (8.0, -2.0),
            (9.0, 2.5),
            // 1 point.
            (10.0, 0.0),
        ];

        let res = Resampling::downsample_points_min_max(&points, 6);

        assert_eq!(
            res,
            [
                // First.
                (0.0, 0.0),
                // Bucket 1.
                (3.0, -4.0),
                (4.0, 6.0),
                // Bucket 2.
                (6.0, 7.0),
                (7.0, -4.0),
                // Last.
                (10.0, 0.0),
            ]
        );
    }

    #[test]
    fn downsample_points_min_max_no_op_nb_points_lt_max_nb_points() {
        let points = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (4.0, 0.0)];

        let res = Resampling::downsample_points_min_max(&points, 6);

        assert_eq!(res, points);
    }

    #[test]
    fn downsample_points_min_max_keep_only_first_and_last() {
        let points = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)];

        let res = Resampling::downsample_points_min_max(&points, 2);

        assert_eq!(res, [(0.0, 0.0), (3.0, 0.0)]);
    }

    #[test]
    #[should_panic(expected = "minimum two points are required as output")]
    fn downsample_points_min_max_error_max_nb_points_lt_2() {
        _ = Resampling::downsample_points_min_max(&[], 2); // OK
        _ = Resampling::downsample_points_min_max(&[], 1);
    }

    #[test]
    #[should_panic(expected = "number of output points must be even")]
    fn downsample_points_min_max_error_max_nb_points_is_odd() {
        _ = Resampling::downsample_points_min_max(&[], 3);
    }

    #[test]
    fn plot_data_with_downsampling_min_max() {
        let f = |x: f64| x.sin();

        // Compute lots of values.
        let (x, y) = Plot::compute_function(0.0, std::f64::consts::TAU, 1000.0, &f);

        let mut canvas = TextCanvas::new(15, 5);
        Plot::scatter(&mut canvas, &x, &y);

        assert_eq!(x.len(), 1000);
        assert_eq!(
            canvas.to_string(),
            "\
⠀⣠⠞⠉⠙⢦⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣰⠃⠀⠀⠀⠈⢧⠀⠀⠀⠀⠀⠀⠀⠀
⠃⠀⠀⠀⠀⠀⠈⢧⠀⠀⠀⠀⠀⢀⡖
⠀⠀⠀⠀⠀⠀⠀⠈⢧⠀⠀⠀⢀⡞⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⢤⠴⠋⠀⠀
"
        );

        let mut canvas_downsampled = TextCanvas::new(15, 5);

        let points: Vec<(f64, f64)> = x.iter().copied().zip(y).collect();
        let points = Resampling::downsample_points_min_max(&points, 60);
        let (x, y): (Vec<f64>, Vec<f64>) = points.into_iter().unzip();

        Plot::scatter(&mut canvas_downsampled, &x, &y);

        // 1000 points downsampled to 60.
        assert_eq!(x.len(), 60);
        assert_eq!(
            canvas_downsampled.to_string(),
            "\
⠀⢀⠔⠉⠉⢂⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢀⠂⠀⠀⠀⠀⠡⠀⠀⠀⠀⠀⠀⠀⠀
⠂⠀⠀⠀⠀⠀⠀⢁⠀⠀⠀⠀⠀⠀⠖
⠀⠀⠀⠀⠀⠀⠀⠀⠢⠀⠀⠀⠀⠌⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢤⠤⠊⠀⠀
"
        );
    }
}
