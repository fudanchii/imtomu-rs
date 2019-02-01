#![no_std]
#![no_main]

extern crate panic_halt;

// For non-examples (i.e. an actual crate)
// tomu_macros don't need to be explicitly
// imported, tomu can be built with `toboot-custom-config` feature
// which will import and reexport toboot_config automatically
use tomu_macros::toboot_config;

use cortex_m_rt::entry;

use tomu::{prelude::*, Tomu};

toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    tomu.leds.red.off();
    tomu.leds.green.on();

    loop {
        tomu.watchdog.feed();
    }
}
