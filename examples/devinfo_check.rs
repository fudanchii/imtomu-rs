#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::prelude::*;

#[entry]
fn main() -> ! {
    let efm32 = EFM32HG::take().unwrap();

    let temp = efm32.DEVINFO.cal.read().temp();
    let flash_size = efm32.DEVINFO.msize.read().flash();
    let ram_size = efm32.DEVINFO.msize.read().sram();

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
