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
    Initialized, Mfrc522,
};

pub fn init(
    spi: SPI,
    sclk: Pin<Output, PB5>,
    mosi: Pin<Output, PB3>,
    miso: Pin<Input<PullUp>, PB4>,
    cs: Pin<Output, PB2>,
) -> Mfrc522<SpiInterface<Spi, ChipSelectPin<PB2>, DummyDelay>, Initialized> {
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

    rfid.init().unwrap()
}
