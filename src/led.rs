use efm32_hal::gpio::{
    pins::{PA0, PB7},
    Normal, OpenDrain, Output, PullUp,
};
use embedded_hal::digital::OutputPin;

pub struct LED<Out>(Out)
where
    Out: OutputPin + ?Sized;

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

impl<Out> LED<Out>
where
    Out: OutputPin + ?Sized,
{
    pub fn on(&mut self) {
        self.0.set_low();
    }

    pub fn off(&mut self) {
        self.0.set_high();
    }
}
