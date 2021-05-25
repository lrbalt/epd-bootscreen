use std::{thread::sleep, time::Duration};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use log::info;

use crate::draw::draw_boot_screen;

pub fn emulator() {
    info!("Creating display and window 212x104 using emulator");
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(212, 104));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
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
