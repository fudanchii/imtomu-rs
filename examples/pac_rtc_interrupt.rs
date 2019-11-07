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

use cortex_m_rt::entry;
use efm32_hal::gpio::{pins::*, *};
use panic_halt as _;
use tomu::{efm32::interrupt, prelude::*, Tomu};

// TODO(lucab): consider a safer type with interior mutability.
static mut GREEN: Option<led::LED<PA0<Output<OpenDrain<Normal, PullUp>>>>> = None;

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();
    tomu.watchdog.disable();

    // Configure clock to RTC:
    //  * LFRCO ticks at 32768 Hz
    //  * No clock divider
    tomu.CMU.hfcoreclken0.write(|w| w.le().set_bit());
    tomu.CMU.oscencmd.write(|w| w.lfrcoen().set_bit());
    tomu.CMU.lfapresc0.reset();
    tomu.CMU.lfclksel.write(|w| w.lfa().lfrco());

    // Enable clock to RTC, ticking at 32 KiHz.
    tomu.CMU.lfaclken0.write(|w| w.rtc().set_bit());

    // constrain CMU and split into device clocks
    // so we can enable gpio with its owned clock
    let clk_mgmt = tomu.CMU.constrain().split();
    let gpio = tomu.GPIO.split(clk_mgmt.gpio).pins();

    // Turn off leds, move the green one for the interrupt handler.
    let mut leds = led::LEDs::new(gpio.pa0.into(), gpio.pb7.into());
    leds.red.off();
    leds.green.off();
    unsafe {
        GREEN = Some(leds.green);
    };

    // Reset RTC
    tomu.RTC.freeze.reset();
    tomu.RTC.ctrl.reset();
    tomu.RTC.ien.reset();
    tomu.RTC
        .ifc
        .write(|w| w.comp0().set_bit().comp1().set_bit().of().set_bit());
    tomu.RTC.comp0.reset();
    tomu.RTC.comp1.reset();

    // Interrupt when matching custom compare value:
    // 65536 / 32768 Hz = 2 secs
    tomu.RTC.comp0.write(|w| unsafe { w.comp0().bits(65_536) });
    tomu.RTC.ien.modify(|_, w| w.comp0().set_bit());

    // Cap counter at `comp0` value.
    tomu.RTC.ctrl.modify(|_, w| w.comp0top().set_bit());

    // Enable RTC interrupts.
    efm32::NVIC::unpend(efm32::Interrupt::RTC);
    tomu.NVIC.enable(efm32::Interrupt::RTC);

    // Start RTC.
    tomu.RTC.ctrl.modify(|_, w| w.en().set_bit());

    // Nothing else to do here, just wait and process interrupts.
    loop {
        cortex_m::asm::wfi();
    }
}

/// Interrupt handler for RTC events (comp0 match).
#[interrupt]
fn RTC() {
    let rtc = tomu::efm32::RTC::ptr();
    cortex_m::interrupt::free(|_| {
        unsafe {
            // Clear interrupt.
            (*rtc).ifc.write(|w| w.comp0().set_bit());

            // Toggle green LED.
            if let Some(ref mut green) = GREEN {
                green.toggle();
            };
        };
    });
}
