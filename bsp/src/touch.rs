use crate::{
    BoardSharedSpiBus, BoardSpiDevice, TouchResources,
    display::{HEIGHT, WIDTH},
};
use defmt::debug;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    spi::Config,
};
use embedded_hal::spi::{Operation, SpiDevice};
use mipidsi::options::Rotation;

fn spi_config() -> Config {
    let mut config = Config::default();
    config.frequency = 200_000;
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    config
}

pub fn init(
    spi: &'static BoardSharedSpiBus,
    resources: TouchResources,
    rotation: Rotation,
    calibration: Calibration,
) -> (Touch, Input<'static>) {
    let cs = Output::new(resources.cs, Level::Low);
    let irq = Input::new(resources.irq, Pull::None);

    let spi = SpiDeviceWithConfig::new(spi, cs, spi_config());

    let touch = Touch::new(spi, rotation, calibration);

    (touch, irq)
}

pub struct Calibration {
    x1: i32,
    x2: i32,

    y1: i32,
    y2: i32,
}

impl Default for Calibration {
    fn default() -> Self {
        Self {
            x1: 330,
            x2: 3780,

            y1: 3780,
            y2: 250,
        }
    }
}

pub struct Touch {
    spi: BoardSpiDevice,
    rotation: Rotation,
    calibration: Calibration,
}

impl Touch {
    pub fn new(spi: BoardSpiDevice, rotation: Rotation, calibration: Calibration) -> Self {
        Self {
            spi,
            rotation,
            calibration,
        }
    }

    pub fn set_calibration(&mut self, calibration: Calibration) {
        self.calibration = calibration;
    }

    pub fn read(&mut self) -> Option<(i32, i32)> {
        let mut x = [0; 2];
        let mut y = [0; 2];

        self.spi
            .transaction(&mut [
                Operation::Write(&[0xd0]),
                Operation::Read(&mut x),
                Operation::Write(&[0x90]),
                Operation::Read(&mut y),
            ])
            .unwrap();

        let x = (u16::from_be_bytes(x) >> 3) as i32;
        let y = (u16::from_be_bytes(y) >> 3) as i32;

        debug!("Raw: x = {}, y = {}", x, y);

        let cal = &self.calibration;

        let width = WIDTH as i32;
        let height = HEIGHT as i32;

        let x = ((x - cal.x1) * width / (cal.x2 - cal.x1)).clamp(0, width);
        let y = ((y - cal.y1) * height / (cal.y2 - cal.y1)).clamp(0, height);

        if x == 0 || y == 0 {
            None
        } else {
            Some(apply_rotation((x, y), self.rotation))
        }
    }
}

fn apply_rotation(p: (i32, i32), rot: Rotation) -> (i32, i32) {
    let width = WIDTH as i32;
    let height = HEIGHT as i32;

    match rot {
        Rotation::Deg0 => p,
        Rotation::Deg90 => (height - p.1, p.0),
        Rotation::Deg180 => (width - p.0, height - p.1),
        Rotation::Deg270 => (p.1, width - p.0),
    }
}
