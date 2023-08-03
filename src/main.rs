#![no_std]
#![no_main]

mod rc522;
mod serial;

use panic_halt as _;

const CARD_UID: [u8; 4] = [227, 134, 36, 51];
const TAG_UID: [u8; 4] = [106, 3, 45, 105];

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

    let mut rfid = rc522::init(spi, sclk, mosi, miso, cs);
    arduino_hal::delay_us(200);
    println!("RC522 version: {}", rfid.version().unwrap());
    arduino_hal::delay_us(200);

    loop {
        if let Ok(atqa) = rfid.new_card_present() {
            if let Ok(uid) = rfid.select(&atqa) {
                if uid.as_bytes() == &CARD_UID {
                    println!("CARD");
                } else if uid.as_bytes() == &TAG_UID {
                    println!("TAG");
                } else {
                    println!("Unknown UID: {:?}", uid.as_bytes());
                }

                // let data = m.mf_read(1)?;
                // println!("read {:?}", data);
            }
        }

        arduino_hal::delay_us(200);
    }
}
