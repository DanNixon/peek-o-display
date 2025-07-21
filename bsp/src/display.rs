use crate::{BoardSharedSpiBus, BoardSpiDevice, DisplayResources};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Level, Output},
    spi::Config,
};
use embassy_time::Delay;
pub use mipidsi::options::Rotation;
use mipidsi::{
    interface::SpiInterface,
    models::ILI9341Rgb666,
    options::{ColorOrder, Orientation},
};

pub type Display = mipidsi::Display<
    SpiInterface<'static, BoardSpiDevice, Output<'static>>,
    ILI9341Rgb666,
    Output<'static>,
>;

fn spi_config() -> Config {
    let mut config = Config::default();
    config.frequency = 64_000_000;
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    config
}

pub fn init(
    spi: &'static BoardSharedSpiBus,
    resources: DisplayResources,
    buffer: &'static mut [u8],
    rotation: Rotation,
) -> (Display, Output<'static>) {
    let cs = Output::new(resources.cs, Level::Low);
    let dc = Output::new(resources.dc, Level::Low);
    let rst = Output::new(resources.reset, Level::Low);
    let backlight = Output::new(resources.backlight, Level::High);

    let spi = SpiDeviceWithConfig::new(spi, cs, spi_config());

    let interface = SpiInterface::new(spi, dc, buffer);

    let display = mipidsi::Builder::new(ILI9341Rgb666, interface)
        .display_size(240, 320)
        .orientation(Orientation::default().rotate(rotation).flip_horizontal())
        .color_order(ColorOrder::Bgr)
        .reset_pin(rst)
        .init(&mut Delay)
        .unwrap();

    (display, backlight)
}
