#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::prelude::*;

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::from(efm32hg::Peripherals::take().unwrap());

    tomu.watchdog.disable();

    loop {
        tomu.leds.red.on();
        tomu.delay.delay_ms(500u16);
        tomu.leds.green.on();
        tomu.delay.delay_ms(500u16);
        tomu.leds.red.off();
        tomu.delay.delay_ms(500u16);
        tomu.leds.green.off();
        tomu.delay.delay_ms(500u16);
    }
}
