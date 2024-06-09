use textcanvas::{Color, TextCanvas};

#[cfg(not(tarpaulin_include))]
fn main() {
    // TextCanvas.

    let mut canvas = TextCanvas::new(15, 5);

    let top_left = (0, 0);
    let top_right = (canvas.w(), 0);
    let bottom_right = (canvas.w(), canvas.h());
    let bottom_left = (0, canvas.h());
    let center = (canvas.cx(), canvas.cy());
    let center_top = (canvas.cx(), 0);
    let center_right = (canvas.w(), canvas.cy());
    let center_bottom = (canvas.cx(), canvas.h());
    let center_left = (0, canvas.cy());

    let red = Color::new().bright_red().fix();
    let yellow = Color::new().bright_yellow().fix();
    let green = Color::new().bright_green().fix();
    let blue = Color::new().bright_blue().fix();
    let cyan = Color::new().bright_cyan().fix();
    let magenta = Color::new().bright_magenta().fix();
    let gray = Color::new().bright_gray().fix();
    let no_color = Color::new();

    canvas.set_color(&red);
    canvas.stroke_line(center.0, center.1, top_left.0, top_left.1);
    canvas.set_color(&yellow);
    canvas.stroke_line(center.0, center.1, top_right.0, top_right.1);
    canvas.set_color(&green);
    canvas.stroke_line(center.0, center.1, bottom_right.0, bottom_right.1);
    canvas.set_color(&blue);
    canvas.stroke_line(center.0, center.1, bottom_left.0, bottom_left.1);
    canvas.set_color(&cyan);
    canvas.stroke_line(center.0, center.1, center_top.0, center_top.1);
    canvas.set_color(&magenta);
    canvas.stroke_line(center.0, center.1, center_right.0, center_right.1);
    canvas.set_color(&gray);
    canvas.stroke_line(center.0, center.1, center_bottom.0, center_bottom.1);
    canvas.set_color(&no_color);
    canvas.stroke_line(center.0, center.1, center_left.0, center_left.1);

    print!("{canvas}");

    // Colors.

    println!(
        "{} {} {}",
        Color::new().bright_red().format("hello, world"),
        Color::new().bold().bright_green().format("hello, world"),
        Color::new()
            .underline()
            .bright_blue()
            .format("hello, world")
    );
    println!(
        "{} {} {}",
        Color::new().italic().bright_cyan().format("hello, world"),
        Color::new().italic().cyan().format("hello, world"),
        Color::new()
            .bold()
            .underline()
            .bright_magenta()
            .format("hello, world")
    );
    println!(
        "{} {} {}",
        Color::new().bright_gray().format("hello, world"),
        Color::new().bright_yellow().format("hello, world"),
        Color::new().bold().bg_bright_gray().format("hello, world")
    );
    println!(
        "{} {} {}",
        Color::new()
            .bold()
            .underline()
            .red()
            .bg_green()
            .format("hello, world"),
        Color::new()
            .bold()
            .rgb(0, 255, 0)
            .bg_rgb(255, 0, 0)
            .format("hello, world"),
        Color::new()
            .x_magenta_3b()
            .bg_x_cadet_blue_b()
            .format("hello, world")
    );
}
