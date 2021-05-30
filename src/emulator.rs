use std::{thread::sleep, time::Duration};

use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use log::info;

use crate::{color::TriColor, draw::draw_boot_screen};

pub fn emulator() {
    info!("Creating display and window 212x104 using emulator");
    let mut display: SimulatorDisplay<TriColor> =
        SimulatorDisplay::with_default_color(Size::new(212, 104), TriColor::White);
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .pixel_spacing(0)
        .build();
    let mut window = Window::new("2.13\" 212x104", &output_settings);

    info!("Drawing boot screen");
    draw_boot_screen(&mut display);

    info!("Waiting for Ctrl-C or close of window");
    'outer: loop {
        window.update(&display);
        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'outer;
            }
        }
        sleep(Duration::from_millis(500u64));
    }
}
