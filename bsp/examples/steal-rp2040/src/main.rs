#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use peek_o_display_bsp::peripherals::PeekODisplay;
use portable_atomic as _;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let p = unsafe { PeekODisplay::steal() };

    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        embassy_time::block_for(Duration::from_hz(5));
        led.toggle();
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = PeekODisplay::default();

    info!("Hello, world!");

    let mut led = Output::new(p.PIN_25, Level::Low);
    led.set_high();

    Timer::after_secs(5).await;

    panic!("oh dear. how sad. nevermind.");
}
