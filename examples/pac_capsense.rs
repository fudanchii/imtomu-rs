#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::DerefMut;
use core::cell::RefCell;

use cortex_m_rt::entry;
use cortex_m::{asm, interrupt as intr, interrupt::Mutex};

use efm32_hal::gpio::*;
use tomu::{interrupt, Tomu, prelude::*};

static _ACMP0: Mutex<RefCell<Option<efm32::ACMP0>>> = Mutex::new(RefCell::new(None));
static _TIMER0: Mutex<RefCell<Option<efm32::TIMER0>>> = Mutex::new(RefCell::new(None));
static _TIMER1: Mutex<RefCell<Option<efm32::TIMER1>>> = Mutex::new(RefCell::new(None));
static GREENLED: Mutex<RefCell<Option<led::LED<pins::PA0<Output<OpenDrain<Normal, PullUp>>>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let tomu = Tomu::take().unwrap();

    clock_setup(&tomu);

    acmp0_setup(&tomu);

    timer_setup(&tomu);

    // constrain CMU and split into device clocks
    // so we can enable gpio with its owned clock
    let clk_mgmt = tomu.CMU.constrain().split();
    let gpio = tomu.GPIO.split(clk_mgmt.gpio).pins();

    // create tomu's led instance from gpio pin
    let leds = led::LEDs::new(gpio.pa0.into(), gpio.pb7.into());

    let mut red = leds.red;
    let green = leds.green;

    red.off();

    efm32::NVIC::unpend(interrupt::TIMER1);
    tomu.NVIC.enable(interrupt::TIMER1);

    let acmp0 = tomu.ACMP0;
    let timer0 = tomu.TIMER0;
    let timer1 = tomu.TIMER1;
    intr::free(|lock| {
        _ACMP0.borrow(lock).replace(Some(acmp0));
        _TIMER0.borrow(lock).replace(Some(timer0));
        _TIMER1.borrow(lock).replace(Some(timer1));
        GREENLED.borrow(lock).replace(Some(green));
    });

    loop { asm::wfi() }
}

#[interrupt]
fn TIMER1() {
    let _ = measure_stop();

    intr::free(|lock| if let &mut Some(ref mut green) = GREENLED.borrow(lock).borrow_mut().deref_mut() {
        green.toggle();
    });

    measure_start();
}

fn measure_start() {
    intr::free(|lock| if let (&mut Some(ref mut timer0), &mut Some(ref mut timer1)) = (
            _TIMER0.borrow(lock).borrow_mut().deref_mut(),
            _TIMER1.borrow(lock).borrow_mut().deref_mut(),
    ) {
        timer0.cnt.write(|w| unsafe { w.cnt().bits(0u16) });
        timer1.cnt.write(|w| unsafe { w.cnt().bits(0u16) });
        timer0.cmd.write(|w| w.start().set_bit());
        timer1.cmd.write(|w| w.start().set_bit());
    });
}

fn measure_stop() -> u16 {
    intr::free(|lock| if let (&mut Some(ref mut timer0), &mut Some(ref mut timer1)) = (
            _TIMER0.borrow(lock).borrow_mut().deref_mut(),
            _TIMER1.borrow(lock).borrow_mut().deref_mut(),
    ) {
        timer0.cmd.write(|w| w.stop().set_bit());
        timer1.cmd.write(|w| w.stop().set_bit());
        timer1.ifc.write(|w| w.of().set_bit());
        timer0.cnt.read().cnt().bits()
    } else { 0 })
}

fn clock_setup(tomu: &Tomu) {
    tomu.CMU.hfperclken0.write(|w| w
        .acmp0().set_bit()
        .timer0().set_bit()
        .timer1().set_bit()
        .prs().set_bit()
    );
}

fn acmp0_setup(tomu: &Tomu) {
    tomu.ACMP0.ctrl.write(|w| unsafe {
        w
            .fullbias().clear_bit()
            .halfbias().clear_bit()
            .biasprog().bits(7u8)
            .warmtime()._512cycles()
            .hystsel().hyst5()
    });

    tomu.ACMP0.inputsel.write(|w| unsafe {
        w
            .csressel().res3()
            .csresen().set_bit()
            .lpref().clear_bit()
            .vddlevel().bits(0x3du8)
            .negsel().capsense()
    });

    tomu.ACMP0.ctrl.modify(|_, w|
        w.en().set_bit()
    );

    tomu.ACMP0.inputsel.modify(|_, w|
        w.possel().ch0()
    );

    while !tomu.ACMP0.status.read().acmpact().bit_is_set() {
        asm::nop();
    }
}

fn timer_setup(tomu: &Tomu) {
    tomu.TIMER0.ctrl.write(|w| w
        .presc().div1024()
        .clksel().cc1()
    );

    tomu.TIMER0.top.write(|w| unsafe { w.bits(0xffffu32) });

    tomu.TIMER0.cc1_ctrl.write(|w| w
        .mode().inputcapture()
        .prssel().prsch0()
        .insel().set_bit()
        .icevctrl().rising()
        .icedge().both()
    );

    tomu.PRS.ch0_ctrl.write(|w| unsafe {
        w
            .edsel().posedge()
            .sourcesel().acmp0()
            .sigsel().bits(0u8)
    });

    tomu.TIMER1.ctrl.write(|w| w.presc().div1024());
    tomu.TIMER1.top.write(|w| unsafe { w.top().bits(20508u16) });
    tomu.TIMER1.ien.write(|w| w.of().set_bit() );
    tomu.TIMER1.cnt.write(|w| unsafe { w.cnt().bits(0u16) });
}
