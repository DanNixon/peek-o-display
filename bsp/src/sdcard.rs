use crate::{BoardSharedSpiBus, BoardSpiDevice, SdCardResources};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Level, Output},
    spi::Config,
};
use embassy_time::Delay;

// Re-export
pub use embedded_sdmmc;

pub type SdCard = embedded_sdmmc::SdCard<BoardSpiDevice, Delay>;

fn spi_config() -> Config {
    let mut config = Config::default();
    config.frequency = 1_000_000;
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    config
}

pub fn init(spi: &'static BoardSharedSpiBus, resources: SdCardResources) -> SdCard {
    let sd_cs = Output::new(resources.cs, Level::Low);
    let sd_spi = SpiDeviceWithConfig::new(spi, sd_cs, spi_config());

    SdCard::new(sd_spi, Delay)
}
