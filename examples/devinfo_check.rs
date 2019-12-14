#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::{prelude::*, Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    // constrain CMU and split into device clocks
    // so we can enable gpio with its owned clock
    let clk_mgmt = tomu.CMU.constrain().split();
    let gpio = tomu.GPIO.split(clk_mgmt.gpio).pins();

    // create tomu's led instance from gpio pin
    let leds = led::LEDs::new(gpio.pa0.into(), gpio.pb7.into());

    let mut red = leds.red;
    let mut green = leds.green;

    red.off();
    green.off();

    let mut delay = systick::SystickDelay::new(tomu.SYST.constrain(), clk_mgmt.hfcoreclk);

    tomu.watchdog.disable();

    let temp = tomu.DEVINFO.cal.read().temp().bits();
    let flash_size = tomu.DEVINFO.msize.read().flash();
    let ram_size = tomu.DEVINFO.msize.read().sram();

    let correct = temp == 25u8 && flash_size == 64u16 && ram_size == 8u16;

    loop {
        if correct {
            green.on();
            delay.delay_ms(250u16);
            green.off();
            delay.delay_ms(250u16);
            continue;
        }

        red.on();
        delay.delay_ms(250u16);
        red.off();
        delay.delay_ms(250u16);
    }
}
