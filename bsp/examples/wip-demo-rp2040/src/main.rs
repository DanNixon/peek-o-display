#![no_std]
#![no_main]

use assign_resources::assign_resources;
use core::cell::RefCell;
use defmt::info;
use defmt_rtt as _;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    peripherals,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::{Delay, Timer};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    prelude::{Dimensions, Point, Primitive, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
    Drawable,
};
use embedded_graphics::{pixelcolor::Rgb666, prelude::WebColors};
use embedded_sdmmc::SdCard;
use mipidsi::{
    interface::SpiInterface,
    models::ILI9341Rgb666,
    options::{ColorOrder, Orientation, Rotation},
};
use panic_probe as _;
use portable_atomic as _;
use touch::Touch;

assign_resources! {
    board_spi: BoardSpiResources {
        spi: SPI0,
        clk: PIN_2,
        mosi: PIN_3,
        miso: PIN_4,
    }
    display: DisplayResources {
        cs: PIN_5,
        dc: PIN_8,
        reset: PIN_7,
        backlight: PIN_9,
    }
    touch_panel: TouchPanelResources {
        cs: PIN_14,
        irq: PIN_15,
    }
    sd_card: SdCardResources {
        cs: PIN_6,
    }
    led: LedResources {
        led: PIN_25,
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    let mut led = Output::new(r.led.led, Level::Low);

    let mut display_config = embassy_rp::spi::Config::default();
    display_config.frequency = 64_000_000;
    display_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    let mut touch_config = embassy_rp::spi::Config::default();
    touch_config.frequency = 200_000;
    touch_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    touch_config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    let mut sd_config = embassy_rp::spi::Config::default();
    sd_config.frequency = 1_000_000;
    sd_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    sd_config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    let spi = embassy_rp::spi::Spi::new_blocking(
        r.board_spi.spi,
        r.board_spi.clk,
        r.board_spi.mosi,
        r.board_spi.miso,
        touch_config.clone(),
    );
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let display_cs = Output::new(r.display.cs, Level::Low);
    let display_spi = SpiDeviceWithConfig::new(&spi_bus, display_cs, display_config);

    let dc = Output::new(r.display.dc, Level::Low);
    let rst = Output::new(r.display.reset, Level::Low);
    let _backlight = Output::new(r.display.backlight, Level::High);

    let mut buffer = [0_u8; 512];
    let interface = SpiInterface::new(display_spi, dc, &mut buffer);

    let mut display = mipidsi::Builder::new(ILI9341Rgb666, interface)
        .display_size(240, 320)
        .orientation(
            Orientation::default()
                .rotate(Rotation::Deg90)
                .flip_horizontal(),
        )
        .color_order(ColorOrder::Bgr)
        .reset_pin(rst)
        .init(&mut Delay)
        .unwrap();

    display.clear(Rgb666::CSS_RED).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb666::CSS_WHITE);
    Text::new(
        "Hello embedded_graphics \n + embassy + RP2040!",
        Point::new(20, 200),
        style,
    )
    .draw(&mut display)
    .unwrap();

    display
        .bounding_box()
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Rgb666::CSS_YELLOW)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    let touch_cs = Output::new(r.touch_panel.cs, Level::Low);
    let touch_spi = SpiDeviceWithConfig::new(&spi_bus, touch_cs, touch_config);
    let touch_irq = Input::new(r.touch_panel.irq, Pull::None);
    let mut touch = Touch::new(touch_spi);

    let sd_cs = Output::new(r.sd_card.cs, Level::Low);
    let sd_spi = SpiDeviceWithConfig::new(&spi_bus, sd_cs, sd_config);
    let sd_card = SdCard::new(sd_spi, Delay);
    info!("Card size is {} bytes", sd_card.num_bytes());
    // TODO: SD card

    // unwrap!(spawner.spawn(blink_led(led)));

    loop {
        if let Some((x, y)) = touch.read() {
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb666::CSS_BLUE)
                .build();

            Rectangle::new(Point::new(x - 1, y - 1), Size::new(3, 3))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

            Timer::after_millis(50).await;
        }

        led.set_level(touch_irq.get_level());
    }
}

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(2000).await;
    }
}

/// Driver for the XPT2046 resistive touchscreen sensor
mod touch {
    use embedded_hal::spi::{Operation, SpiDevice};

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

    pub struct Touch<SPI: SpiDevice> {
        spi: SPI,
    }

    impl<SPI> Touch<SPI>
    where
        SPI: SpiDevice,
    {
        pub fn new(spi: SPI) -> Self {
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
}
