#![no_std]

extern crate efm32;
extern crate embedded_hal;

#[cfg(feature = "toboot-custom-config")]
extern crate tomu_hal_macros;

pub mod toboot;

pub mod peripherals;

pub mod capacitive;
pub mod gpio;
pub mod led;
pub mod uart;
pub mod usb;

#[cfg(feature = "toboot-custom-config")]
pub use tomu_hal_macros::toboot_config;
