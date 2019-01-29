#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use tomu::{prelude::*, Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();
    let mut timer = tomu.delay;
    let red = tomu.led.red();

    tomu.watchdog.disable();

    loop {
        red.off();
        timer.delay_ms(1000_u32);
        red.on();
        timer.delay_ms(1000_u32);
    }
}
