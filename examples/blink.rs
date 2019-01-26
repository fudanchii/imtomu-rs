#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use tomu_hal::{led::LedTrait, peripherals};

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    let mut counter = 0;

    loop {
        if counter == 400000 {
            p.led.green().off();
            p.led.red().off();
        } else if counter == 300000 {
            p.led.green().on();
            p.led.red().off();
        } else if counter == 200000 {
            p.led.green().off();
            p.led.red().off();
        }
        if counter == 100000 {
            p.led.green().off();
            p.led.red().on();
        }

        if counter > 0 {
            counter = counter - 1;
        } else {
            counter = 400000;
        }

        p.watchdog.pet();
    }
}
