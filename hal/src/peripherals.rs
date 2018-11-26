use gpio::GPIO;

use efm32;

pub struct Peripherals {
    pub p: efm32::Peripherals,
    pub gpio: GPIO,
}

pub fn take() -> Peripherals {
    let mut p = efm32::Peripherals::take().unwrap();
    let gpio = GPIO::take(&mut p.CMU);

    Peripherals {
        p: p,
        gpio: gpio,
    }
}

impl Peripherals {
    pub fn watchdog_disable(&self) {
        self.p.WDOG.ctrl.write(|w| unsafe { w.bits(0b00000000) });
    }
}
