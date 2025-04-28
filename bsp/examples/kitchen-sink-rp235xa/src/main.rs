#![no_std]
#![no_main]

use assign_resources::assign_resources;
use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Ticker};
use panic_probe as _;
use peek_o_display_bsp::peripherals::{self, PeekODisplay};
use portable_atomic as _;

assign_resources! {
    led: LedResources {
        led: PIN_25,
    },
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = PeekODisplay::default();
    let r = split_resources!(p);

    info!("Hello, world!");

    unwrap!(spawner.spawn(blink_led(r.led)));
}

#[embassy_executor::task]
async fn blink_led(r: LedResources) {
    let mut led = Output::new(r.led, Level::Low);

    let mut ticker = Ticker::every(Duration::from_hz(1));

    loop {
        ticker.next().await;
        led.toggle();
    }
}
