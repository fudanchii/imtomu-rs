#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate embedded_hal;
extern crate tomu_hal;

use cortex_m_rt::entry;
use tomu_hal::{
    peripherals,
    toboot_config,
    led::LedTrait,
    gpio::{pin, OpenDrain, InputPullDown},
};

use embedded_hal::digital::{OutputPin, InputPin};

/// this works too:
/// ```
/// toboot_config! { }
/// ```
toboot_config! {
    config: [autorun_enable],
    lock_entry: false,
    erase_mask_lo: 0,
    erase_mask_hi: 0,
}

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    let pin_c0      = p.gpio.split::<pin::C0<InputPullDown>>();
    let mut pin_e12 = p.gpio.split::<pin::E12<OpenDrain>>();

    p.watchdog.disable();

    p.led.green().off();
    p.led.red().on();

    pin_e12.set_high();

    let mut counter = 0;

    loop {
        if pin_c0.is_high() {
            pin_e12.set_low();
            p.led.green().on();
            p.led.red().off();
            counter = 2000000;
        }

        if counter > 0 {
            counter = counter - 1;
        }

        if counter == 0 {
            pin_e12.set_high();
            p.led.green().off();
            p.led.red().on();
        }
    }
}
