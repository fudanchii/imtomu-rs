#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::prelude::*;

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::from(efm32hg::Peripherals::take().unwrap());

    tomu.watchdog.disable();

    tomu.leds.red.off();

    loop {
        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.red.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);

        tomu.leds.green.toggle();
        tomu.delay.delay_ms(100u16);
    }
}
