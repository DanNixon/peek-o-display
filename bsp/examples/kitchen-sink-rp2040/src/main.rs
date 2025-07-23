#![no_std]
#![no_main]

use assign_resources::assign_resources;
use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output};
use embassy_time::{Duration, Ticker, Timer};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::{Primitive, RgbColor},
    primitives::PrimitiveStyleBuilder,
    text::Text,
    Drawable,
};
use panic_probe as _;
use peek_o_display_bsp::{
    display::{Display, Rotation},
    peripherals,
    sdcard::SdCard,
    touch::{Calibration, Touch},
    PeekODisplay,
};
use portable_atomic as _;

assign_resources! {
    led: LedResources {
        led: PIN_25,
    },
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = PeekODisplay::default();
    let p = board.peripherals();
    let r = split_resources!(p);

    let spi = board.board_spi();

    let display_rotation = Rotation::Deg90;

    let (display, backlight) = board.display(spi, display_rotation);

    let (touch, touch_irq) = board.touch(spi, display_rotation, Calibration::default());

    let sd_card = board.sdcard(spi);

    info!("Hello, world!");

    unwrap!(spawner.spawn(led_task(r.led)));
    unwrap!(spawner.spawn(display_task(display, backlight)));
    unwrap!(spawner.spawn(touch_task(touch, touch_irq)));
    unwrap!(spawner.spawn(sdcard_task(sd_card)));
}

#[embassy_executor::task]
async fn led_task(r: LedResources) {
    let mut led = Output::new(r.led, Level::Low);

    let mut ticker = Ticker::every(Duration::from_hz(1));

    loop {
        ticker.next().await;
        led.toggle();
    }
}

#[embassy_executor::task]
async fn display_task(mut display: Display, _backlight: Output<'static>) {
    display.clear(Rgb666::RED).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb666::WHITE);
    Text::new("HI", display.bounding_box().center(), style)
        .draw(&mut display)
        .unwrap();

    display
        .bounding_box()
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Rgb666::YELLOW)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Timer::after_secs(5).await;

    let mut ticker = Ticker::every(Duration::from_hz(1));

    loop {
        ticker.next().await;
        display.clear(Rgb666::RED).unwrap();
        ticker.next().await;
        display.clear(Rgb666::GREEN).unwrap();
        ticker.next().await;
        display.clear(Rgb666::BLUE).unwrap();
    }
}

#[embassy_executor::task]
async fn touch_task(mut touch: Touch, _touch_irq: Input<'static>) {
    let mut ticker = Ticker::every(Duration::from_hz(100));

    loop {
        ticker.next().await;

        if let Some((x, y)) = touch.read() {
            info!("touch at : {},{}", x, y);
        }
    }
}

#[embassy_executor::task]
async fn sdcard_task(sd_card: SdCard) {
    let mut ticker = Ticker::every(Duration::from_secs(5));

    loop {
        ticker.next().await;

        info!("Card size is {} bytes", sd_card.num_bytes());
    }
}
