#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate nb;
extern crate panic_halt;
extern crate tomu_hal;

use cortex_m_rt::entry;
use embedded_hal::timer::CountDown;
use nb::block;
use tomu_hal::{led::LedTrait, peripherals};

#[entry]
fn main() -> ! {
    let mut p = peripherals::take();

    p.watchdog.disable();

    p.rtc.default_setup();

    p.rtc.start(3);

    p.led.green().off();
    p.led.red().off();

    loop {
        p.led.green().on();
        block!(p.rtc.wait()).unwrap();

        p.led.green().off();
        block!(p.rtc.wait()).unwrap();

        p.led.red().on();
        block!(p.rtc.wait()).unwrap();

        p.led.red().off();
        block!(p.rtc.wait()).unwrap();
    }
}
