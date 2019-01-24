#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::watchdog::WatchdogDisable;
use tomu_hal::gpio::{self, pin::B7, OpenDrain};
use tomu_hal::{delay::Delay, prelude::*, watchdog::Watchdog};

#[entry]
fn main() -> ! {
    let mut p = efm32::Peripherals::take().unwrap();
    let cp = efm32::CorePeripherals::take().unwrap();

    Watchdog::new(p.WDOG).disable();

    let g = gpio::GPIO::new(&mut p.CMU);
    let mut red = g.split::<B7<OpenDrain>>();

    let clocks = p.CMU.constrain().freeze();
    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        red.set_high();
        timer.delay_ms(1000_u32);
        red.set_low();
        timer.delay_ms(1000_u32);
    }
}
