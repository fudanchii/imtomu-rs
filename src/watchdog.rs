use efm32::WDOG;
use embedded_hal::watchdog;

/// Watchdog peripheral for tomu board.
pub struct Watchdog {
    wdog: WDOG,
}

impl Watchdog {
    pub fn new(wdog: WDOG) -> Self {
        Self { wdog }
    }
}

impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        unsafe {
            self.wdog.cmd.write(|w| w.bits(1));
        }
    }
}

impl watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        unsafe {
            self.wdog.ctrl.write(|w| w.bits(0));
        }
    }
}
