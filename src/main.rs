#![no_std]
#![no_main]

mod serial;

use arduino_hal::spi::Settings;
use mfrc522::comm::eh02::spi::SpiInterface;

use panic_halt as _;

const CARD_UID: [u8; 4] = [227, 134, 36, 51];
const TAG_UID: [u8; 4] = [106, 3, 45, 105];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    serial::init(arduino_hal::default_serial!(dp, pins, 57600));

    println!("Ready!");

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let sclk = pins.d13.into_output();
    let mosi = pins.d11.into_output();
    let miso = pins.d12.into_pull_up_input();
    let cs = pins.d10.into_output();

    let (spi, nss) = arduino_hal::Spi::new(
        dp.SPI,
        sclk,
        mosi,
        miso,
        cs,
        Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            mode: embedded_hal::spi::MODE_0,
            clock: arduino_hal::spi::SerialClockRate::OscfOver128,
        },
    );

    let comm = SpiInterface::new(spi).with_nss(nss);
    let rfid = mfrc522::Mfrc522::new(comm);
    let mut rfid = rfid.init().unwrap();

    println!("version: {}", rfid.version().unwrap());

    loop {
        // match rfid.new_card_present() {
        //     Ok(_card) => println!("new card"),
        //     Err(_err) => println!("error reading card"),
        // };

        if let Ok(atqa) = rfid.reqa() {
            if let Ok(uid) = rfid.select(&atqa) {
                if uid.as_bytes() == &CARD_UID {
                    println!("CARD");
                } else if uid.as_bytes() == &TAG_UID {
                    println!("TAG");
                } else {
                    println!("Unknown UID: {:?}", uid.as_bytes());
                }

                // handle_authenticate(&mut rfid, &uid, |m| {
                //     let data = m.mf_read(1)?;
                //     println!("read {:?}", data);
                //     Ok(())
                // })
                // .ok();
            }
        }
    }
}
