use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::style::PrimitiveStyleBuilder;

use st7735_lcd::{Orientation, ST7735};

use gpio_cdev::{Chip, LineRequestFlags};

use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{CdevPin, Spidev};

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.1").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(10_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    let mut chip = Chip::new("/dev/gpiochip0").expect("chip");
    let dc = CdevPin::new(
        chip.get_line(9)
            .expect("dc line")
            .request(LineRequestFlags::OUTPUT, 1, "dc export")
            .expect("dc request"),
    )
    .expect("dc pin");
    let reset = CdevPin::new(
        chip.get_line(16)  // Unused! Empty pin according to the pinout
            .expect("reset line")
            .request(LineRequestFlags::OUTPUT, 1, "reset export")
            .expect("reset request"),
    )
    .expect("reset pin");
    let _backlight = CdevPin::new(
        chip.get_line(12)
            .expect("backlight line")
            .request(LineRequestFlags::OUTPUT, 1, "backlight export")
            .expect("backlight request"),
    )
    .expect("backlight pin");
    let mut delay = Delay {};

    let mut display = ST7735::new(spi, dc, reset, false, true, 160, 80);
    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();

    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::GREEN).build();
    let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(160, 80)).into_styled(style);
    display.set_offset(0, 25);
    black_backdrop.draw(&mut display).unwrap();

    loop {
        continue;
    }
}
