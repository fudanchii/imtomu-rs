#![no_std]
#![no_main]

extern crate panic_halt;

// For non-examples (i.e. an actual crate)
// tomu_hal_macros don't need to be explicitly
// imported, tomu_hal can be built with `toboot-custom-config` feature
// which will import and reexport toboot_config automatically
use tomu_hal_macros::toboot_config;

use cortex_m_rt::entry;

use tomu_hal::{prelude::*, tomu::Tomu};

toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    tomu.led.red().off();
    tomu.led.green().on();

    loop {
        tomu.watchdog.feed();
    }
}
