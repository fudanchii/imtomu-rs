#![no_std]
#![no_main]

// For non-examples (i.e. an actual crate)
// tomu_macros don't need to be explicitly
// imported, tomu can be built with `toboot-custom-config` feature
// which will import and reexport toboot_config automatically
use tomu_macros::toboot_config;

use cortex_m_rt::entry;
use panic_halt as _;
use tomu::prelude::*;

// this will cause tomu to always enter user application,
// short the 2 pins on the corner while inserting tomu to
// enter bootloader.
toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::from(efm32hg::Peripherals::take().unwrap());

    tomu.leds.red.off();
    tomu.leds.green.on();

    loop {
        tomu.watchdog.feed();
    }
}
