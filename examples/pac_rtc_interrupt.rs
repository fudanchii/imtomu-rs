//! PAC example: RTC interrupt toggling green LED.
//!
//! This examples shows:
//!  * how to configure clock to RTC.
//!  * how to configure RTC parameters.
//!  * how to configure and handle RTC interrupts.
//!
//! It requires the "unproven" feature for LED toggling.

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use tomu::{efm32, efm32::interrupt, prelude::*};

static GREEN: Mutex<RefCell<Option<led::GreenLED>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let efm32hg = EFM32HG::take().unwrap();

    // Configure clock to RTC:
    //  * LFRCO ticks at 32768 Hz
    //  * No clock divider
    efm32hg.CMU.hfcoreclken0.write(|w| w.le().set_bit());
    efm32hg.CMU.oscencmd.write(|w| w.lfrcoen().set_bit());
    efm32hg.CMU.lfapresc0.reset();
    efm32hg.CMU.lfclksel.write(|w| w.lfa().lfrco());

    // Enable clock to RTC, ticking at 32 KiHz.
    efm32hg.CMU.lfaclken0.write(|w| w.rtc().set_bit());

    // Reset RTC
    efm32hg.RTC.freeze.reset();
    efm32hg.RTC.ctrl.reset();
    efm32hg.RTC.ien.reset();
    efm32hg.RTC.ifc
        .write(|w| w.comp0().set_bit().comp1().set_bit().of().set_bit());
    efm32hg.RTC.comp0.reset();
    efm32hg.RTC.comp1.reset();

    // Interrupt when matching custom compare value:
    // 65536 / 32768 Hz = 2 secs
    efm32hg.RTC.comp0.write(|w| unsafe { w.comp0().bits(65_536) });
    efm32hg.RTC.ien.modify(|_, w| w.comp0().set_bit());

    // Cap counter at `comp0` value.
    efm32hg.RTC.ctrl.modify(|_, w| w.comp0top().set_bit());

    // Enable RTC interrupts.
    efm32::NVIC::unpend(efm32::Interrupt::RTC);
    unsafe { efm32::NVIC::unmask(efm32::Interrupt::RTC) };

    // Start RTC.
    efm32hg.RTC.ctrl.modify(|_, w| w.en().set_bit());

    let mut tomu = Tomu::from(efm32hg);
    tomu.watchdog.disable();

    tomu.leds.red.off();
    tomu.leds.green.off();

    cortex_m::interrupt::free(|lock| {
        GREEN.borrow(lock).replace(Some(tomu.leds.green));
    });

    // Nothing else to do here, just wait and process interrupts.
    loop {
        cortex_m::asm::wfi();
    }
}

/// Interrupt handler for RTC events (comp0 match).
#[interrupt]
fn RTC() {
    let rtc = unsafe { &*tomu::efm32::RTC::ptr() };
    cortex_m::interrupt::free(|lock| {
        // Clear interrupt.
        rtc.ifc.write(|w| w.comp0().set_bit());

        // Toggle green LED.
        if let Some(ref mut green) = GREEN.borrow(lock).borrow_mut().deref_mut() {
            green.toggle();
        };
    });
}
