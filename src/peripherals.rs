use crate::capacitive;
use crate::efm32;
use crate::gpio;
use crate::led;

/// Watchdog peripheral for tomu board.
pub struct Watchdog;

impl Watchdog {
    /// Disable watchdog, this will prevent the need to refresh
    /// watchdog timer.
    pub fn disable(&mut self) {
        unsafe {
            (*efm32::WDOG::ptr()).ctrl.write(|w| w.bits(0));
        }
    }

    /// By default Tomu boot loader activate watchdog, and it
    /// will need to be refreshed before 9 seconds elapsed.
    /// Call this method to pet the watchdog.
    pub fn refresh(&mut self) {
        unsafe {
            (*efm32::WDOG::ptr()).cmd.write(|w| w.bits(1));
        }
    }

    /// Alias to refresh
    pub fn pet(&mut self) {
        self.refresh();
    }
}

/// Holds all available tomu peripherals
pub struct Peripherals {
    #[allow(dead_code)]
    p: efm32::Peripherals,

    pub gpio: gpio::GPIO,
    pub watchdog: Watchdog,
    pub led: led::LED,
    pub touch: capacitive::Capacitive,
}

/// Take `Peripherals`  instance, this is called `take`
/// since we also take efm32's own `Peripherals` which will
/// cause this method to panic if it's called more than once.
pub fn take() -> Peripherals {
    let mut p = efm32::Peripherals::take().unwrap();

    let mut our_gpio = gpio::GPIO::new(&mut p.CMU);
    let led = led::LED::new(&mut our_gpio);
    let cap = capacitive::Capacitive::new(&mut our_gpio);

    Peripherals {
        p: p,
        gpio: our_gpio,
        led: led,
        watchdog: Watchdog,
        touch: cap,
    }
}
