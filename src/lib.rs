#![no_std]

pub use efm32;

#[cfg(feature = "rt")]
pub use crate::efm32::interrupt;

pub mod toboot;

// pub mod peripherals;

// pub mod capacitive;
pub mod gpio;
// pub mod led;
pub mod uart;
pub mod usb;

#[cfg(feature = "toboot-custom-config")]
pub use tomu_hal_macros::toboot_config;
