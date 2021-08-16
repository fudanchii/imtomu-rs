use efm32_hal::{
    watchdog::{Watchdog, WatchdogExt},
    cmu::CMUExt, gpio::GPIOExt,
    gpio::pins, gpio::common::{Disabled, Floating},
};
use crate::led::LEDs;

pub struct Tomu {
    gpio: TomuFreeGPIO,
    leds: LEDs,
    watchdog: Watchdog,
}

impl Tomu {
    pub fn from(efm32: crate::EFM32HG) -> Self {
        let cmu = efm32.CMU.constrain().freeze();
        let gpio = efm32.GPIO.constrain(cmu.gpio_clock).split();
        Self {
            watchdog: efm32.WDOG.constrain(),
            leds: LEDs::new(gpio.pa0.into(), gpio.pb7.into()),
            gpio: TomuFreeGPIO {
                pb8: gpio.pb8,
                pb11: gpio.pb11,
                pb13: gpio.pb13,
                pb14: gpio.pb14,
                pc0: gpio.pc0,
                pc1: gpio.pc1,
                pc14: gpio.pc14,
                pc15: gpio.pc15,
                pe12: gpio.pe12,
                pe13: gpio.pe13,
                pf0: gpio.pf0,
                pf1: gpio.pf1,
                pf2: gpio.pf2,
            },
        }
    }
}

pub struct TomuFreeGPIO {
    pub pb8: pins::PB8<Disabled<Floating>>,
    pub pb11: pins::PB11<Disabled<Floating>>,
    pub pb13: pins::PB13<Disabled<Floating>>,
    pub pb14: pins::PB14<Disabled<Floating>>,
    pub pc0: pins::PC0<Disabled<Floating>>,
    pub pc1: pins::PC1<Disabled<Floating>>,
    pub pc14: pins::PC14<Disabled<Floating>>,
    pub pc15: pins::PC15<Disabled<Floating>>,
    pub pe12: pins::PE12<Disabled<Floating>>,
    pub pe13: pins::PE13<Disabled<Floating>>,
    pub pf0: pins::PF0<Disabled<Floating>>,
    pub pf1: pins::PF1<Disabled<Floating>>,
    pub pf2: pins::PF2<Disabled<Floating>>,
}
