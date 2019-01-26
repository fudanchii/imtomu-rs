use crate::gpio::{
    pin::{A0, B7},
    OpenDrain, GPIO,
};
use embedded_hal::digital::OutputPin;

/// LED struct stores all leds available
/// in tomu board.
/// It owns all the leds, and all access to
/// leds are treated via mutable borrow.
pub struct LED {
    green: A0<OpenDrain>,
    red: B7<OpenDrain>,
}

impl LED {
    /// Create new LED instance, this is supposed
    /// to create singleton, but it's not as of currently.
    /// As we don't have singleton support in GPIO package
    /// yet.
    pub fn new(g: &mut GPIO) -> Self {
        LED {
            green: g.split::<A0<OpenDrain>>(),
            red: g.split::<B7<OpenDrain>>(),
        }
    }

    /// Mutably borrow green led.
    /// As per https://github.com/rust-lang/rfcs/issues/1215
    /// partial borrow for struct field via method call
    /// is not supported yet as the borrowed field were
    /// expected to live as long as the borrowed struct.
    /// So in this case, with binding, green led cannot
    /// be used with red led in the same scope.
    ///
    /// ```no_run
    /// // this is ok
    /// p.led.green().on();
    /// p.led.red().on();
    ///
    /// // this is not ok
    /// let red = p.led.red();
    /// let green = p.led.green();
    ///
    /// // this is ok
    /// {
    ///     let red = p.led.red();
    ///     red.on();
    /// }
    /// let green = p.led.green();
    /// green.on();
    /// ```
    pub fn green(&mut self) -> &mut A0<OpenDrain> {
        &mut self.green
    }

    /// Mutably borrow red led.
    /// see `green(&mut self)` documentation
    /// for more info.
    pub fn red(&mut self) -> &mut B7<OpenDrain> {
        &mut self.red
    }
}

/// Common trait for leds
pub trait LedTrait {
    /// turn on led.
    fn on(&mut self);

    /// turn off led.
    fn off(&mut self);
}

impl<T: OutputPin> LedTrait for T {
    fn on(&mut self) {
        self.set_low();
    }

    fn off(&mut self) {
        self.set_high();
    }
}
