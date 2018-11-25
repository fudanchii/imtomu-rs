#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate efm32hg309f64 as efm32hg;
extern crate panic_halt;
extern crate tomu_hal;

use cortex_m_rt::entry;
use tomu_hal::toboot_config;

/// this works too:
/// ```
/// toboot_config! { }
/// ```
toboot_config! {
    // enable autorun
    config: [autorun_enable],
    lock_entry: false,
    erase_mask_lo: 0,
    erase_mask_hi: 0,
}

#[entry]
fn main() -> ! {
    let p = efm32hg::Peripherals::take().unwrap();

    p.WDOG.ctrl.write(|w| unsafe { w.bits(0b00000000) });

    p.CMU.hfperclken0.modify(|_, w| w.gpio().bit(true));

    p.GPIO.pa_model.modify(|_, w| w.mode0().wiredand());
    p.GPIO.pb_model.modify(|_, w| w.mode7().wiredand());

    // tomu use pullup resistor for its leds,
    // need to set low to turn the led on
    p.GPIO.pa_doutclr.write(|w| unsafe {w.bits(0b00000001) });
    p.GPIO.pb_doutclr.write(|w| unsafe {w.bits(0b10000000) });

    loop {}
}
