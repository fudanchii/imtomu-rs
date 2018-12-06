#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate panic_halt;
extern crate tomu_hal;

// For non-examples (i.e. an actual crate)
// tomu_hal_macros don't need to be explicitly
// imported, tomu_hal can be built with `toboot-custom-config` feature
// which will import and reexport toboot_config automatically
extern crate tomu_hal_macros;
use tomu_hal_macros::toboot_config;

use cortex_m_rt::entry;

use tomu_hal::{peripherals, led::LedTrait};

toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    p.watchdog.disable();

    p.led.red().off();
    p.led.green().on();

    loop {}
}
