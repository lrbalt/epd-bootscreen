use crate::color::TriColor;
use chrono::Local;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text, TextStyleBuilder},
};
use std::convert::Infallible;

fn draw_centered_large<DT>(
    display: &mut DT,
    text: &str,
    top: i32,
    left: i32,
    width: i32,
    color: TriColor,
) -> Result<Point, Infallible>
where
    DT: DrawTarget<Color = TriColor, Error = Infallible>,
{
    let style = MonoTextStyle::new(&profont::PROFONT_18_POINT, color);
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let mut text = Text::with_text_style(text, Point::new(left, top), style, text_style);

    let size = text.bounding_box().size;
    text = text.translate(Point::new((width - size.width as i32) / 2, 0));
    text.draw(display)
}

fn draw_centered_medium<DT>(
    display: &mut DT,
    text: &str,
    top: i32,
    left: i32,
    width: i32,
    color: TriColor,
) -> Result<Point, Infallible>
where
    DT: DrawTarget<Color = TriColor, Error = Infallible>,
{
    let style = MonoTextStyle::new(&profont::PROFONT_12_POINT, color);
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let mut text = Text::with_text_style(text, Point::new(left, top), style, text_style);

    let size = text.bounding_box().size;
    text = text.translate(Point::new((width - size.width as i32) / 2, 0));
    text.draw(display)
}

pub fn draw_boot_screen<DT>(display: &mut DT)
where
    DT: DrawTarget<Color = TriColor, Error = Infallible>,
{
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(TriColor::White)
        .fill_color(TriColor::White)
        .stroke_width(1)
        .build();

    Rectangle::new(Point::new(0, 0), Size::new(212, 104))
        .into_styled(style)
        .draw(display)
        .unwrap();

    draw_centered_large(display, "Booting Solar Pi", 10, 0, 212, TriColor::Chromatic).unwrap();

    let now = Local::now();
    draw_centered_medium(
        display,
        &format!("{}", now.date_naive().format("%d-%m-%Y")),
        55,
        0,
        212,
        TriColor::Black,
    )
    .unwrap();

    draw_centered_medium(
        display,
        &format!("{}", now.time().format("%H:%M:%S")),
        75,
        0,
        212,
        TriColor::Black,
    )
    .unwrap();
}
