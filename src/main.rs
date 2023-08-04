#![no_std]
#![no_main]

mod rc522;
mod serial;

use panic_halt as _;

/*
 * For examples (and inspiration), head to
 *
 *     https://github.com/Rahix/avr-hal/tree/main/examples
 *
 * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
 * for a different board can be adapted for yours.  The Arduino Uno currently has the most
 * examples available.
 */

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let spi = dp.SPI;
    let sclk = pins.d13.into_output();
    let mosi = pins.d11.into_output();
    let miso = pins.d12.into_pull_up_input();
    let cs = pins.d10.into_output();

    serial::init(arduino_hal::default_serial!(dp, pins, 57600));
    println!("Serial initialised!");
    arduino_hal::delay_us(200);

    arduino_hal::delay_ms(1000);
    let _ = pins.d9.into_output_high();
    arduino_hal::delay_us(200);

    let mut rfid = rc522::new(spi, sclk, mosi, miso, cs);
    arduino_hal::delay_us(200);
    println!("RC522 version: {}", rfid.version());
    arduino_hal::delay_us(200);

    loop {
        if let Some(found) = rfid.read() {
            println!("{:?}", found);
        };
        arduino_hal::delay_us(200);
    }
}
