#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{Board, display::blocking::Display, hal::Timer};
use panic_halt as _;
use embedded_hal::delay::DelayNs;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting up...");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let led = [
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0]
    ];
    loop {
        rprintln!("Looping...");
        display.show(&mut timer, led, 1000);
        display.clear();
        timer.delay_ms(1000);
    }
}
