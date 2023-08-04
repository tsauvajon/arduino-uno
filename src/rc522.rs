//! RFID-RC522
use arduino_hal::{
    hal::port::{PB2, PB3, PB4, PB5},
    port::{
        mode::{Input, Output, PullUp},
        Pin,
    },
    spi::{ChipSelectPin, Settings, Spi},
};
use avr_device::atmega328p::SPI;
use mfrc522::{
    comm::eh02::spi::{DummyDelay, SpiInterface},
    Initialized, Mfrc522, Uid,
};
use ufmt::uDebug;

pub struct Rfid {
    rc522: Mfrc522<SpiInterface<Spi, ChipSelectPin<PB2>, DummyDelay>, Initialized>,
}

const CARD_UID: [u8; 4] = [227, 134, 36, 51];
const TAG_UID: [u8; 4] = [106, 3, 45, 105];

pub fn new(
    spi: SPI,
    sclk: Pin<Output, PB5>,
    mosi: Pin<Output, PB3>,
    miso: Pin<Input<PullUp>, PB4>,
    cs: Pin<Output, PB2>,
) -> Rfid {
    let (spi, nss) = arduino_hal::Spi::new(
        spi,
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
    arduino_hal::delay_us(200);
    let rfid = mfrc522::Mfrc522::new(comm);
    arduino_hal::delay_us(200);

    Rfid {
        rc522: rfid.init().unwrap(),
    }
}

impl Rfid {
    pub fn version(&mut self) -> u8 {
        self.rc522.version().unwrap()
    }

    pub fn read(&mut self) -> Option<Found> {
        if let Ok(atqa) = self.rc522.new_card_present() {
            if let Ok(uid) = self.rc522.select(&atqa) {
                if uid.as_bytes() == CARD_UID {
                    return Some(Found::Card);
                } else if uid.as_bytes() == TAG_UID {
                    return Some(Found::Tag);
                } else {
                    return Some(Found::Unknown(uid));
                }

                // let data = m.mf_read(1)?;
                // println!("read {:?}", data);
            }
        }

        None
    }
}

pub enum Found {
    Tag,
    Card,
    Unknown(Uid),
}

impl uDebug for Found {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Found::Tag => f.write_str("Tag"),
            Found::Card => f.write_str("Card"),
            Found::Unknown(uid) => {
                f.write_str("Unknown(")?;
                ufmt::uwrite!(f, "{:?}", uid.as_bytes())?;
                f.write_str(")")
            }
        }
    }
}
