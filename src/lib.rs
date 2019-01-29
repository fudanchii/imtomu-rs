#![no_std]

use embedded_hal;

pub use efm32;

#[cfg(feature = "rt")]
pub use crate::efm32::interrupt;

pub mod toboot;

pub mod capacitive;
pub mod clocks;
pub mod delay;
pub mod gpio;
pub mod led;
pub mod time;
pub mod tomu;
pub mod uart;
pub mod usb;
pub mod watchdog;

#[cfg(feature = "toboot-custom-config")]
pub use tomu_hal_macros::toboot_config;

pub mod prelude {
    pub use crate::clocks::ClocksExt;
    pub use crate::delay::DelayExt;
    pub use crate::led::LedTrait;
    pub use crate::time::U32Ext;
    pub use embedded_hal::prelude::*;
    pub use embedded_hal::watchdog::Watchdog;
    pub use embedded_hal::watchdog::WatchdogDisable;
}
