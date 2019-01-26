use crate::gpio::{
    pin::{C0, C1, E12, E13},
    InputPullDown, OpenDrain, GPIO,
};
use embedded_hal::digital::{InputPin, OutputPin};

pub struct Button<Sink, Source> {
    sink: Sink,
    source: Source,
}

impl<Sink: InputPin, Source: OutputPin> Button<Sink, Source> {
    /// Button state, true if pressed
    pub fn is_pressed(&self) -> bool {
        self.sink.is_high()
    }

    /// Button need to be in hold before pressed,
    /// this set source pin to high.
    pub fn hold(&mut self) {
        self.source.set_high();
    }

    /// Release button, `is_pressed` will be always
    /// false if button is in released state.
    /// Need to call hold before checking `is_pressed` again
    pub fn release(&mut self) {
        self.source.set_low();
    }
}

/// List all capacitive buttons available in tomu board.
pub struct Capacitive {
    c0: Button<C0<InputPullDown>, E12<OpenDrain>>,
    c1: Button<C1<InputPullDown>, E13<OpenDrain>>,
}

impl Capacitive {
    /// Create new capacitive buttons
    pub fn new(gpio: &mut GPIO) -> Capacitive {
        Capacitive {
            c0: Button {
                sink: gpio.split::<C0<InputPullDown>>(),
                source: gpio.split::<E12<OpenDrain>>(),
            },
            c1: Button {
                sink: gpio.split::<C1<InputPullDown>>(),
                source: gpio.split::<E13<OpenDrain>>(),
            },
        }
    }

    /// Mutably borrow first button (on the right)
    pub fn cap0(&mut self) -> &mut Button<C0<InputPullDown>, E12<OpenDrain>> {
        &mut self.c0
    }

    /// Mutably borrow second button (on the left)
    pub fn cap1(&mut self) -> &mut Button<C1<InputPullDown>, E13<OpenDrain>> {
        &mut self.c1
    }
}
