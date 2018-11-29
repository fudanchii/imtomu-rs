use gpio;
use led;
use efm32;

/// Watchdog peripheral for tomu board.
pub struct Watchdog;

impl Watchdog {
    /// Disable watchdog, this will prevent the need to refresh
    /// watchdog timer.
    pub fn disable(&self) {
        unsafe {
            (*efm32::WDOG::ptr()).ctrl.write(|w| w.bits(0));
        }
    }

    /// By default Tomu boot loader activate watchdog, and it
    /// will need to be refreshed before 9 seconds elapsed.
    /// Call this method to pet the watchdog.
    pub fn refresh(&self) {
        unsafe {
            (*efm32::WDOG::ptr()).cmd.write(|w| w.bits(1));
        }
    }
}

/// Holds all available tomu peripherals
pub struct Peripherals {
    pub p: efm32::Peripherals,
    pub gpio: gpio::GPIO,
    pub watchdog: Watchdog,
    pub led: led::LED,
}

/// Take `Peripherals`  instance, this is called `take`
/// since we also take efm32's own `Peripherals` which will
/// cause this method to panic if it's called more than once.
pub fn take() -> Peripherals {
    let mut p = efm32::Peripherals::take().unwrap();

    let mut our_gpio = gpio::GPIO::take(&mut p.CMU);
    let led = led::LED::new(&mut our_gpio);

    Peripherals {
        p: p,
        gpio: our_gpio,
        led: led,
        watchdog: Watchdog,
    }
}
