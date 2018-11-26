#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate tomu_hal;

use cortex_m_rt::entry;
use tomu_hal::{
    peripherals,
    toboot_config,
    gpio::{OpenDrain, pin},
};

/// this works too:
/// ```
/// toboot_config! { }
/// ```
toboot_config! {
    // enable autorun
    config: [],
    lock_entry: false,
    erase_mask_lo: 0,
    erase_mask_hi: 0,
}

#[entry]
fn main() -> ! {
    let p = peripherals::take();

    p.watchdog_disable();

    // or
    // ```
    // let pa0 = p.gpio.split::<pin::A0<WiredAnd>>();
    // let pb7 = p.gpio.split::<pin::B7<WiredAnd>>();
    // ```
    let mut pa0 = p.gpio.split::<pin::A0<OpenDrain>>();
    let mut pb7 = p.gpio.split::<pin::B7<OpenDrain>>();

    // or, if using led hal
    // ```
    // use tomu_hal::led;
    //
    // // ...
    //
    // let green_led = p.led.green();
    // let red_led = p.led.red();
    //
    // green_led.on();
    // red_led.off();
    pa0.set_low();  // on
    pb7.set_high(); // off


    loop {}
}
