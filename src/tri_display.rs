use embedded_graphics::{drawable::Pixel, prelude::Size, DrawTarget};

use crate::color::TriColor;

pub struct TriDisplay<const BUFFER_SIZE: usize, const WIDTH: usize, const HEIGHT: usize> {
    buffer: [TriColor; BUFFER_SIZE],
}

impl<const BUFFER_SIZE: usize, const WIDTH: usize, const HEIGHT: usize> Default
    for TriDisplay<BUFFER_SIZE, WIDTH, HEIGHT>
{
    fn default() -> Self {
        TriDisplay {
            buffer: [TriColor::White; BUFFER_SIZE],
        }
    }
}

impl<const BUFFER_SIZE: usize, const WIDTH: usize, const HEIGHT: usize> DrawTarget<TriColor>
    for TriDisplay<BUFFER_SIZE, WIDTH, HEIGHT>
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, item: Pixel<TriColor>) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}
