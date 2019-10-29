#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m::asm;

use tomu::Tomu;

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

    loop { asm::nop(); }
}
