use efm32_hal::gpio::{
    pins::{PA0, PB7},
    Normal, OpenDrain, Output, PullUp,
};
use embedded_hal::digital::v2::OutputPin;

pub struct LED<Out>(Out)
where
    Out: OutputPin + ?Sized;

/// Public trait for leds, All leds can have common behavior
/// that it can be turned on, and turned off. This can be used
/// to set common pins as led type without having to care whether
/// the led is active high or active low.
/// XXX: Likely need to implement toggle when it's available.
pub trait LedTrait {
    /// Turn on the led.
    fn on(&mut self);

    /// Turn off the led.
    fn off(&mut self);
}

/// LED struct stores all leds available
/// in tomu board.
/// It owns all the leds, but access can be moved per led.
pub struct LEDs {
    pub green: LED<PA0<Output<OpenDrain<Normal, PullUp>>>>,
    pub red: LED<PB7<Output<OpenDrain<Normal, PullUp>>>>,
}

impl LEDs {
    /// Take ownership for the respective pin
    pub fn new(
        green: PA0<Output<OpenDrain<Normal, PullUp>>>,
        red: PB7<Output<OpenDrain<Normal, PullUp>>>,
    ) -> Self {
        LEDs {
            green: LED(green),
            red: LED(red),
        }
    }
}

impl<Out: OutputPin> LedTrait for LED<Out> {
    fn on(&mut self) {
        let _ = self.0.set_low();
    }

    fn off(&mut self) {
        let _ = self.0.set_high();
    }
}
