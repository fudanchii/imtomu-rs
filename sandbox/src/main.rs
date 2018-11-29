#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate tomu_hal;

use cortex_m_rt::entry;
use tomu_hal::{
    peripherals,
    toboot_config,
    led::LedTrait,
};

/// this works too:
/// ```
/// toboot_config! { }
/// ```
toboot_config! {
    config: [],
    lock_entry: false,
    erase_mask_lo: 0,
    erase_mask_hi: 0,
}

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    p.watchdog.disable();

    p.led.green().on();
    p.led.red().off();

    loop {}
}
