use crate::{BoardSharedSpiBus, BoardSpiDevice, TouchResources};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    spi::Config,
};
use embedded_hal::spi::{Operation, SpiDevice};

fn spi_config() -> Config {
    let mut config = Config::default();
    config.frequency = 200_000;
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    config
}

pub fn init(spi: &'static BoardSharedSpiBus, resources: TouchResources) -> (Touch, Input<'static>) {
    let cs = Output::new(resources.cs, Level::Low);
    let irq = Input::new(resources.irq, Pull::None);

    let spi = SpiDeviceWithConfig::new(spi, cs, spi_config());

    let touch = Touch::new(spi);

    (touch, irq)
}

struct Calibration {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    sx: i32,
    sy: i32,
}

const CALIBRATION: Calibration = Calibration {
    x1: 3880,
    x2: 340,
    y1: 262,
    y2: 3850,
    sx: 320,
    sy: 240,
};

pub struct Touch {
    spi: BoardSpiDevice,
}

impl Touch {
    pub fn new(spi: BoardSpiDevice) -> Self {
        Self { spi }
    }

    pub fn read(&mut self) -> Option<(i32, i32)> {
        let mut x = [0; 2];
        let mut y = [0; 2];
        self.spi
            .transaction(&mut [
                Operation::Write(&[0x90]),
                Operation::Read(&mut x),
                Operation::Write(&[0xd0]),
                Operation::Read(&mut y),
            ])
            .unwrap();

        let x = (u16::from_be_bytes(x) >> 3) as i32;
        let y = (u16::from_be_bytes(y) >> 3) as i32;

        let cal = &CALIBRATION;

        let x = ((x - cal.x1) * cal.sx / (cal.x2 - cal.x1)).clamp(0, cal.sx);
        let y = ((y - cal.y1) * cal.sy / (cal.y2 - cal.y1)).clamp(0, cal.sy);
        if x == 0 && y == 0 {
            None
        } else {
            Some((x, y))
        }
    }
}
