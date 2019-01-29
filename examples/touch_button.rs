#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use tomu_hal::{prelude::*, tomu::Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    tomu.led.green().on();
    tomu.led.red().off();

    let mut counter = 0;

    loop {
        if tomu.touch.cap0().is_pressed() {
            tomu.touch.cap0().release();
            tomu.touch.cap1().release();
            tomu.led.red().on();
            tomu.led.green().off();
            counter = 2000000;
        }

        if tomu.touch.cap1().is_pressed() {
            tomu.touch.cap0().release();
            tomu.touch.cap1().release();
            tomu.led.red().off();
            tomu.led.green().on();
            counter = 2000000;
        }

        if counter > 0 {
            counter = counter - 1;
        }

        if counter == 0 {
            tomu.touch.cap0().hold();
            tomu.touch.cap1().hold();
        }

        tomu.watchdog.feed();
    }
}
