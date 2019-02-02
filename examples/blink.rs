#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use tomu::{prelude::*, Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();
    let mut red = tomu.leds.red;
    let mut green = tomu.leds.green;
    let mut delay = tomu.delay;

    tomu.watchdog.disable();

    loop {
        red.on();
        delay.delay_ms(500u16);
        green.on();
        delay.delay_ms(500u16);
        red.off();
        delay.delay_ms(500u16);
        green.off();
        delay.delay_ms(500u16);
    }
}
