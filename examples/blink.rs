#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use tomu::{delay::Delay, prelude::*, Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    tomu.watchdog.disable();

    let clocks = tomu.CMU.constrain().freeze();
    let mut timer = Delay::new(tomu.SYST, clocks);

    loop {
        tomu.led.red().off();
        timer.delay_ms(1000_u32);
        tomu.led.red().on();
        timer.delay_ms(1000_u32);
    }
}
