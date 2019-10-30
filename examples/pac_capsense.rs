#![no_std]
#![no_main]

extern crate panic_halt;

use core::cell::Cell;

use cortex_m_rt::entry;
use cortex_m::{asm, interrupt as intr, interrupt::Mutex};

use tomu::{interrupt, Tomu};

static TIMER1READY: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    let tomu = Tomu::take().unwrap();
    
    tomu.CMU.hfperclken0.write(|w|
        w
            .acmp0().set_bit()
            .timer0().set_bit()
            .timer1().set_bit()
            .prs().set_bit()
    );

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

    tomu.TIMER0.ctrl.write(|w|
        w
            .presc().div1024()
            .clksel().cc1()
    );

    tomu.TIMER0.top.write(|w| unsafe { w.bits(0xffffu32) });

    tomu.TIMER0.cc1_ctrl.write(|w|
        w
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

    tomu.TIMER1.ctrl.write(|w| w.presc().div1024() );
    tomu.TIMER1.top.write(|w| unsafe { w.bits(40 * 21u32) });
    tomu.TIMER1.ien.write(|w| w.of().set_bit() );
    tomu.TIMER1.cnt.write(|w| unsafe { w.cnt().bits(0u16) });

    let mut ch: usize = 0;
    let mut touch = 0;
    let mut present = 0;
    let mut since_last_touch = 0;
    let mut count_max: [u16; 2] = [0, 0];

    loop {
        let ready = intr::free(|cs| TIMER1READY.borrow(cs).get());

        if !ready {
            asm::wfi();
            continue;
        }

        let count = measure_stop(&tomu);

        let threshold = count_max[ch] - count_max[ch] / 2;
        if count > 0 && count < threshold.into() {
            touch |= 1 << ch;
        } else {
            touch &= !(1 << ch);
        }

        if count > threshold.into() {
            count_max[ch] = (count_max[ch] + count as u16) / 2;
        }

        if present > 0 {
            present -= 1;
        }

        if touch > 0 {
            if since_last_touch > 10 {
                if present > 0 {
                    present = 0;
                } else {
                    present = 500;
                }
            }
            since_last_touch = 0;
        } else {
            since_last_touch += 1;
        }

        if since_last_touch > 1000 {
            since_last_touch = 1000;
        }

        ch ^= 1;
        measure_start(ch as u8, &tomu);
    }
}

fn measure_start(ch: u8, tomu: &Tomu) {
    tomu.ACMP0.inputsel.modify(|_, w| w.possel().bits(ch));
    tomu.TIMER0.cnt.write(|w| unsafe { w.cnt().bits(0u16) });
    tomu.TIMER1.cnt.write(|w| unsafe { w.cnt().bits(0u16) });
    tomu.TIMER0.cmd.write(|w| w.start().set_bit());
    tomu.TIMER1.cmd.write(|w| w.start().set_bit());
}

fn measure_stop(tomu: &Tomu) -> u32 {
    tomu.TIMER0.cmd.write(|w| w.stop().set_bit());
    tomu.TIMER1.cmd.write(|w| w.stop().set_bit());
    tomu.TIMER1.ifc.write(|w| w.of().set_bit());
    tomu.TIMER0.cnt.read().bits()
}

#[interrupt]
fn TIMER1() {
    intr::free(|cs| TIMER1READY.borrow(cs).set(true));
}
