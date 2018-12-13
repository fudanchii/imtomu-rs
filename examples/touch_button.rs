#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate panic_halt;
extern crate tomu_hal;

use cortex_m_rt::entry;
use tomu_hal::{led::LedTrait, peripherals};

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    p.led.green().on();
    p.led.red().off();

    let mut counter = 0;

    loop {
        if p.touch.cap0().is_pressed() {
            p.touch.cap0().release();
            p.touch.cap1().release();
            p.led.red().on();
            p.led.green().off();
            counter = 2000000;
        }

        if p.touch.cap1().is_pressed() {
            p.touch.cap0().release();
            p.touch.cap1().release();
            p.led.red().off();
            p.led.green().on();
            counter = 2000000;
        }

        if counter > 0 {
            counter = counter - 1;
        }

        if counter == 0 {
            p.touch.cap0().hold();
            p.touch.cap1().hold();
        }

        p.watchdog.pet();
    }
}
