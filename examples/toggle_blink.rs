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

    let mut delay = systick::SystickDelay::new(tomu.SYST.constrain(), clk_mgmt.hfcoreclk);

    tomu.watchdog.disable();

    // by default red led is on, toggle it to off first.
    red.toggle();

    loop {
        red.toggle();
        delay.delay_ms(100u16);

        red.toggle();
        delay.delay_ms(100u16);

        red.toggle();
        delay.delay_ms(100u16);

        red.toggle();
        delay.delay_ms(100u16);

        red.toggle();
        delay.delay_ms(100u16);

        red.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);

        green.toggle();
        delay.delay_ms(100u16);
    }
}
