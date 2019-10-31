#![no_std]
#![no_main]


// For non-examples (i.e. an actual crate)
// tomu_macros don't need to be explicitly
// imported, tomu can be built with `toboot-custom-config` feature
// which will import and reexport toboot_config automatically
use tomu_macros::toboot_config;

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::{prelude::*, Tomu};

// this will cause tomu to always enter user application,
// short the 2 pins on the corner while inserting tomu to
// enter bootloader.
toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    let clk_mgmt = tomu.CMU.constrain().split();
    let gpio = tomu.GPIO.split(clk_mgmt.gpio).pins();

    let leds = led::LEDs::new(gpio.pa0.into(), gpio.pb7.into());

    let mut red = leds.red;
    let mut green = leds.green;

    red.off();
    green.on();

    loop {
        tomu.watchdog.feed();
    }
}
