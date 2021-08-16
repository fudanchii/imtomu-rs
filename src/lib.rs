#![no_std]

pub use efm32;
pub use efm32_hal::{systick, systick::SystickExt, watchdog::WatchdogExt};

#[cfg(feature = "rt")]
pub use crate::efm32::interrupt;

pub mod toboot;

pub mod led;
pub mod uart;
pub mod usb;
pub mod efm32hg;
pub use efm32hg::EFM32HG;
pub mod tomu;
pub use tomu::Tomu;

#[cfg(feature = "toboot-custom-config")]
pub use tomu_macros::toboot_config;

pub mod prelude {
    pub use embedded_hal::prelude::*;
    pub use embedded_hal::watchdog::Watchdog;
    pub use embedded_hal::watchdog::WatchdogDisable;

    pub use efm32_hal::systick;

    pub use efm32_hal::cmu::CMUExt;
    pub use efm32_hal::gpio::GPIOExt;
    pub use efm32_hal::systick::SystickExt;
    pub use efm32_hal::watchdog::WatchdogExt;

    pub use crate::led;
    pub use crate::led::LedTrait;
}
