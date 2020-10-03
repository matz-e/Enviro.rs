use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::style::{PrimitiveStyleBuilder, TextStyle};
use embedded_graphics::fonts::{Font8x16, Text};

use embedded_hal::prelude::*;
use gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::{CdevPin, Delay, I2cdev, Spidev};

use bme280::BME280;
use st7735_lcd::{Orientation, ST7735};

fn main() {
    let i2c_bus = I2cdev::new("/dev/i2c-1").expect("i2c bus");
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().expect("bme280 init");

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
    display.set_orientation(&Orientation::LandscapeSwapped).unwrap();
    display.set_offset(0, 25);

    let bg_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let text_style = TextStyle::new(Font8x16, Rgb565::WHITE);
    let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(160, 80)).into_styled(bg_style);
    black_backdrop.draw(&mut display).unwrap();

    loop {
        if let Ok(measurements) = bme280.measure() {
            let tmp = format!("Temp: {:.1}°", measurements.temperature);
            println!("Temp: {:.3}°", measurements.temperature);
            let area = Rectangle::new(Point::new(0,0), Point::new(160, 16)).into_styled(bg_style);
            let text = Text::new(&tmp, Point::new(0, 0)).into_styled(text_style);
            area.draw(&mut display).unwrap();
            text.draw(&mut display).unwrap();
        } else {
            println!("Could not grab temperature!");
        }
        delay.delay_ms(1000u16);
    }
}
