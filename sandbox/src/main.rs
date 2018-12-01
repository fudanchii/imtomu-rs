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
};

/// this works too:
/// ```
/// toboot_config! { }
/// ```
toboot_config! {
    config: [autorun_enable],
}

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    p.watchdog.disable();

    p.led.green().on();
    p.led.red().off();

    let mut counter = 0;

    loop {
        if p.cap.c0().is_pressed() {
            p.cap.c0().release();
            p.cap.c1().release();
            p.led.red().on();
            p.led.green().off();
            counter = 2000000;
        }

        if p.cap.c1().is_pressed() {
            p.cap.c0().release();
            p.cap.c1().release();
            p.led.red().off();
            p.led.green().on();
            counter = 2000000;
        }

        if counter > 0 {
            counter = counter - 1;
        }

        if counter == 0 {
            p.cap.c0().hold();
            p.cap.c1().hold();
        }
    }
}
