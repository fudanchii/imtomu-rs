#![no_std]
#![no_main]

/// imtomu-rs examples: pac_capsense.rs
///
/// This is an example on how to configure touch sense (capsense) in
/// efm32hg309 (imtomu) board. Capsense enabled by configuring ACMP0's
/// negative signal in `capsense` mode, and route the generated pulse
/// to TIMER0 via PRS channel.
///
/// Another timer, TIMER1, acted as timekeeper and record the pulse counted by
/// TIMER0, and when ACMP0's ch0 pad (GPIO PC0) sensing finger touch, the pulse
/// periode will be slower and so resulting in smaller counter in TIMER0.
///
/// In this example we normalize the counter and use it as a blink period.
/// The leds will blink faster when the capsense is touched.
use panic_halt as _;

use core::cell::{Cell, RefCell};
use core::ops::DerefMut;

use cortex_m::asm;
use cortex_m_rt::entry;

use critical_section::Mutex;

use efm32_hal::delay;
use tomu::{interrupt, prelude::*};

// NOTE: the following value follows the mcu frequency divided by 1024
// e.g.
// 1367 -> 14 MHz
// 2051 -> 21 MHz
const TIMER1_TOP_VALUE: u16 = 1367u16;

const CAPSENSE_THRESHOLD_VALUE: u16 = 400;

static _ACMP0: Mutex<RefCell<Option<efm32::ACMP0>>> = Mutex::new(RefCell::new(None));
static _TIMER0: Mutex<RefCell<Option<efm32::TIMER0>>> = Mutex::new(RefCell::new(None));
static _TIMER1: Mutex<RefCell<Option<efm32::TIMER1>>> = Mutex::new(RefCell::new(None));

static DELAY: Mutex<RefCell<Option<delay::Delay>>> = Mutex::new(RefCell::new(None));

static GREENLED: Mutex<RefCell<Option<led::GreenLED>>> = Mutex::new(RefCell::new(None));
static REDLED: Mutex<RefCell<Option<led::RedLED>>> = Mutex::new(RefCell::new(None));

static CHSWITCH: Mutex<Cell<u8>> = Mutex::new(Cell::new(1));

#[entry]
fn main() -> ! {
    let efm32 = efm32hg::Peripherals::take().unwrap();

    clock_setup(&efm32);

    acmp0_setup(&efm32);

    timer_setup(&efm32);

    let acmp0 = efm32.ACMP0;
    let timer0 = efm32.TIMER0;
    let timer1 = efm32.TIMER1;

    let mut tomu = Tomu::from_parts(efm32.CMU, efm32.WDOG, efm32.GPIO, efm32.SYST);
    tomu.watchdog.disable();

    // NOTE: toboot v2.0-rc7 and below has issues with GPIO pin reset
    // you may want to call the following
    // tomu.gpio.pc1.into_input();

    // NOTE: toboot v2.0-rc7 and earlier has issues with GPIO pin reset
    // you may want to call the following
    //
    // tomu.leds.red.off();
    //

    critical_section::with(|lock| {
        efm32::NVIC::unpend(interrupt::TIMER1);
        unsafe {
            efm32::NVIC::unmask(interrupt::TIMER1);
        }

        _ACMP0.borrow(lock).replace(Some(acmp0));
        _TIMER0.borrow(lock).replace(Some(timer0));
        _TIMER1.borrow(lock).replace(Some(timer1));
        GREENLED.borrow(lock).replace(Some(tomu.leds.green));
        REDLED.borrow(lock).replace(Some(tomu.leds.red));
        DELAY.borrow(lock).replace(Some(tomu.delay));
    });

    measure_start();

    loop {
        asm::wfi()
    }
}

#[interrupt]
fn TIMER1() {
    let count = measure_stop();

    // NOTE: Capacitance will be lower if capsense get touched, so the led will blinks
    // faster. With the counter threshold set, no leds will be on if the capsense
    // is not touched.
    critical_section::with(|lock| {
        if let (&mut Some(ref mut green), &mut Some(ref mut red), &mut Some(ref mut delay), ch) = (
            GREENLED.borrow(lock).borrow_mut().deref_mut(),
            REDLED.borrow(lock).borrow_mut().deref_mut(),
            DELAY.borrow(lock).borrow_mut().deref_mut(),
            CHSWITCH.borrow(lock).get(),
        ) {
            let scaled_count = count / 100;
            if scaled_count > CAPSENSE_THRESHOLD_VALUE {
                // not touched
                match ch {
                    0 => green.off(),
                    _ => red.off(),
                }
                return;
            }
            match ch {
                0 => {
                    // channel 0 touched
                    green.on();
                    delay.delay_ms(scaled_count);
                    green.off();
                }
                _ => {
                    // channel 1 touched
                    red.on();
                    delay.delay_ms(scaled_count);
                    red.off();
                }
            }
        }
    });

    measure_start();
}

fn measure_start() {
    critical_section::with(|lock| {
        let ch = CHSWITCH.borrow(lock).get();
        CHSWITCH.borrow(lock).replace(ch ^ 1);

        if let (
            &mut Some(ref mut timer0),
            &mut Some(ref mut timer1),
            &mut Some(ref mut acmp0),
            ch,
        ) = (
            _TIMER0.borrow(lock).borrow_mut().deref_mut(),
            _TIMER1.borrow(lock).borrow_mut().deref_mut(),
            _ACMP0.borrow(lock).borrow_mut().deref_mut(),
            CHSWITCH.borrow(lock).get(),
        ) {
            if ch == 0 {
                acmp0.inputsel.modify(|_, w| w.possel().ch0());
            } else {
                acmp0.inputsel.modify(|_, w| w.possel().ch1());
            }
            timer0.cnt.reset();
            timer1.cnt.reset();
            timer0.cmd.write(|w| w.start().set_bit());
            timer1.cmd.write(|w| w.start().set_bit());
        }
    });
}

fn measure_stop() -> u16 {
    critical_section::with(|lock| {
        if let (&mut Some(ref mut timer0), &mut Some(ref mut timer1)) = (
            _TIMER0.borrow(lock).borrow_mut().deref_mut(),
            _TIMER1.borrow(lock).borrow_mut().deref_mut(),
        ) {
            timer0.cmd.write(|w| w.stop().set_bit());
            timer1.cmd.write(|w| w.stop().set_bit());
            timer1.ifc.write(|w| w.of().set_bit());
            timer0.cnt.read().cnt().bits()
        } else {
            0
        }
    })
}

fn clock_setup(efm32: &efm32hg::Peripherals) {
    efm32.CMU.hfperclken0.modify(|_, w| {
        w.acmp0().set_bit()
         .timer0().set_bit()
         .timer1().set_bit()
         .prs().set_bit()
    });
}

fn acmp0_setup(efm32: &efm32hg::Peripherals) {
    efm32.ACMP0.ctrl.write(|w| unsafe {
        w.fullbias().clear_bit()
         .halfbias().clear_bit()
         .biasprog().bits(7u8)
         .warmtime()._512cycles()
         .hystsel().hyst5()
    });

    efm32.ACMP0.inputsel.write(|w| unsafe {
        w.csressel().res3()
         .csresen().set_bit()
         .lpref().clear_bit()
         .vddlevel().bits(0x3du8)
         .negsel().capsense()
    });

    efm32.ACMP0.ctrl.modify(|_, w| w.en().set_bit());

    loop {
        if efm32.ACMP0.status.read().acmpact().bit_is_set() {
            break;
        }
    }
}

fn timer_setup(efm32: &efm32hg::Peripherals) {
    efm32.TIMER0
        .ctrl
        .write(|w| w.presc().div1024().clksel().cc1());

    efm32.TIMER0.top.reset();

    efm32.TIMER0.cc1_ctrl.write(|w| {
        w.mode().inputcapture()
         .prssel().prsch0()
         .insel().set_bit()
         .icevctrl().rising()
         .icedge().both()
    });

    efm32.PRS
        .ch0_ctrl
        .write(|w| unsafe { w.edsel().posedge().sourcesel().acmp0().sigsel().bits(0u8) });

    efm32.TIMER1.ctrl.write(|w| w.presc().div1024());

    // scan time 100ms
    efm32.TIMER1
        .top
        .write(|w| unsafe { w.top().bits(TIMER1_TOP_VALUE) });
    efm32.TIMER1.ien.write(|w| w.of().set_bit());
}
