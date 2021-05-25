use crate::draw::draw_boot_screen;
use epd_waveshare::{
    color::*,
    epd2in13bc::{Display2in13bc, Epd2in13bc},
    graphics::{Display, DisplayRotation},
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};
use log::{debug, info, LevelFilter};

fn split_buffer(buffer: &Display2in13bc) -> (Display2in13bc, Display2in13bc) {
    let mut bw = Display2in13bc::default();
    let mut cr = Display2in13bc::default();
    for (i, &v) in buffer.buffer().iter().enumerate() {
        bw.get_mut_buffer()[i] = v;
        cr.get_mut_buffer()[i] = 0xFF;
    }
    (bw, cr)
}

pub fn epd() {
    debug!("Configuring pins");

    let busy = Pin::new(24); // GPIO 24, board J-18
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(25); // GPIO 25, board J-22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");

    let rst = Pin::new(17); // GPIO 17, board J-11
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs = Pin::new(26); // CE0, board J-24, GPIO 8 -> doesn work. use this from 2in19 example which works
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    debug!("Configure SPI");
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(10_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    let mut delay = Delay {};

    debug!("Initializing i-ink screen");
    let epd =
        Epd2in13bc::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");
    debug!("Initializing done");

    debug!("Creating display for e-paper screen with 90 degrees rotation");
    let mut display = Display2in13bc::default();
    display.set_rotation(DisplayRotation::Rotate90);
    display.clear_buffer(Color::White);

    draw_boot_screen(&mut display).unwrap();

    debug!("Update display");
    let (buffer, chromatic_buffer) = split_buffer(&display);
    epd.update_color_frame(&mut spi, &buffer, &chromatic_buffer)
        .unwrap();
    epd.display_frame(&mut self.spi, &mut self.delay).unwrap();

    screen.update().unwrap();
}
