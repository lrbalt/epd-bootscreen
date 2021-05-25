use std::convert::Infallible;

use chrono::Local;
use embedded_graphics::{
    egrectangle, egtext, pixelcolor::BinaryColor, prelude::*, primitive_style, text_style,
};
use profont::{ProFont12Point, ProFont18Point};

fn draw_centered_large<DT>(
    display: &mut DT,
    text: &str,
    top: i32,
    left: i32,
    width: i32,
) -> Result<(), Infallible>
where
    DT: DrawTarget<BinaryColor, Error = Infallible>,
{
    let mut text = egtext!(
        text = &text,
        top_left = (left, top),
        style = text_style!(font = ProFont18Point, text_color = BinaryColor::On)
    );
    text = text.translate(Point::new((width - text.size().width as i32) / 2, 0));
    text.draw(display)
}

fn draw_centered_medium<DT>(
    display: &mut DT,
    text: &str,
    top: i32,
    left: i32,
    width: i32,
) -> Result<(), Infallible>
where
    DT: DrawTarget<BinaryColor, Error = Infallible>,
{
    let mut text = egtext!(
        text = &text,
        top_left = (left, top),
        style = text_style!(font = ProFont12Point, text_color = BinaryColor::On)
    );
    text = text.translate(Point::new((width - text.size().width as i32) / 2, 0));
    text.draw(display)
}

pub fn draw_boot_screen<DT>(display: &mut DT)
where
    DT: DrawTarget<BinaryColor, Error = Infallible>,
{
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (212, 104),
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            fill_color = BinaryColor::Off,
            stroke_width = 1
        )
    )
    .draw(display)
    .unwrap();

    draw_centered_large(display, "Booting Solar Pi", 10, 0, 212).unwrap();

    let now = Local::now();
    draw_centered_medium(
        display,
        &format!("{}", now.date().format("%d-%m-%Y")),
        55,
        0,
        212,
    )
    .unwrap();

    draw_centered_medium(
        display,
        &format!("{}", now.time().format("%H:%M:%S")),
        75,
        0,
        212,
    )
    .unwrap();
}
