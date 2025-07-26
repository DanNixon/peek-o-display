#![no_std]
#![no_main]

use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use embedded_alloc::LlffHeap as Heap;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb666, prelude::RgbColor};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use panic_probe as _;
use peek_o_display_bsp::{
    display::{Display, Rotation},
    embassy_rp::gpio::{Input, Output},
    touch::{Calibration, Touch},
    PeekODisplay,
};
use portable_atomic as _;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Wrap},
    Frame, Terminal,
};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 400_000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let board = PeekODisplay::default();
    let spi = board.board_spi();

    let display_rotation = Rotation::Deg90;

    let (display, backlight) = board.display(spi, display_rotation);

    let (touch, touch_irq) = board.touch(spi, display_rotation, Calibration::default());

    info!("Hello, world!");

    unwrap!(spawner.spawn(display_task(display, backlight)));
    // unwrap!(spawner.spawn(touch_task(touch, touch_irq)));
}

#[embassy_executor::task]
async fn display_task(mut display: Display, _backlight: Output<'static>) {
    display.clear(Rgb666::RED).unwrap();

    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());

    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(draw).unwrap();
    }
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
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
