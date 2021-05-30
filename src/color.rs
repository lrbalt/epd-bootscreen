#[cfg(not(feature = "e-paper"))]
pub use crate::tri_color::TriColor;
#[cfg(feature = "e-paper")]
pub use epd_waveshare::color::TriColor;
