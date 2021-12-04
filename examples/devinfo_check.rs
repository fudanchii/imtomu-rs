#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::prelude::*;

#[entry]
fn main() -> ! {
    let efm32 = efm32hg::Peripherals::take().unwrap();

    let temp = efm32.DEVINFO.cal.read().temp().bits();
    let flash_size = efm32.DEVINFO.msize.read().flash().bits();
    let ram_size = efm32.DEVINFO.msize.read().sram().bits();

    let mut tomu = Tomu::from(efm32);

    tomu.leds.red.off();
    tomu.leds.green.off();

    tomu.watchdog.disable();

    let correct = temp == 25u8 && flash_size == 64u16 && ram_size == 8u16;

    loop {
        if correct {
            tomu.leds.green.on();
            tomu.delay.delay_ms(250u16);
            tomu.leds.green.off();
            tomu.delay.delay_ms(250u16);
            continue;
        }

        tomu.leds.red.on();
        tomu.delay.delay_ms(250u16);
        tomu.leds.red.off();
        tomu.delay.delay_ms(250u16);
    }
}
