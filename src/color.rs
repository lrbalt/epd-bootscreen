use std::convert::From;

use embedded_graphics::pixelcolor::{PixelColor, Rgb888, RgbColor};

/// Only for the Black/White/Color-Displays
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TriColor {
    /// Black color
    Black,
    /// White color
    White,
    /// Chromatic color
    Chromatic,
}

/// The `Raw` can be is set to `()` because `EpdColor` doesn't need to be
/// converted to raw data for the display and isn't stored in images.
impl PixelColor for TriColor {
    type Raw = ();
}

impl From<TriColor> for Rgb888 {
    fn from(color: TriColor) -> Rgb888 {
        match color {
            TriColor::Black => Rgb888::BLACK,
            TriColor::White => Rgb888::WHITE,
            TriColor::Chromatic => Rgb888::RED,
        }
    }
}