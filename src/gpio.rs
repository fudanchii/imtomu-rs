//! GPIO is a higher abstraction for gpio driver available from
//! svd2rust derived efm32hg309f64 board supprt code. This hal
//! also unify access for all I/O mode as disctinct type, so instead of
//! having to fiddle with OUT register for filter and pullup configuration
//! to setup specific config, we can just call `split` method with the desired
//! I/O type.
//!
//! ```
//! use gpio;
//! use gpio::{pin::{A0, C0, E12}, OpenDrain, InputPullDown};
//!
//! let p = efm32::Peripherals::take().unwrap();
//! let g = gpio::GPIO::new(&p.CMU);
//!
//! let mut pinA0  : A0<OpenDrain>     = g.split();
//! let mut pinC0  : C0<InputPullDown> = g.split();
//! let mut pinE12 : E12<OpenDrain>    = g.split();
//!
//! pinE12.set_high();
//! loop {
//!     if pinC0.is_high() {
//!         pinA0.toggle();
//!     }
//! }
//!
//! ```

use core::marker::PhantomData;
use crate::efm32;
use crate::efm32::{CMU, GPIO};
use embedded_hal::digital::{OutputPin, ToggleableOutputPin};

pub trait OutputMode {}

/// Disable Input/Output no pullup.
pub struct Disabled;

/// Disable Input/Output with pullup.
pub struct DisabledPullUp;

/// Enable Input without glitch filter.
pub struct Input;

/// Enable Input with glitch filter.
pub struct InputWithFilter;

/// Enable Input with pull-down resistor (without glitch filter).
pub struct InputPullDown;

/// Enable Input with pull-up resistor (without glitch filter).
pub struct InputPullUp;

/// Enable Input with pull-down resistor and glitch filter.
pub struct InputPullDownWithFilter;

/// Enable Input with pull-up resistor and glitch filter.
pub struct InputPullUpWithFilter;

/// Enable Input and Output in push-pull mode.
pub struct PushPull;

/// Enable Input and Output in push-pull mode and enable current drive.
pub struct PushPullDrive;

/// Enable Input and Output in open-source mode.
pub struct WiredOr;
pub type OpenSource = WiredOr;

/// Enable Input and Output in open-source mode and pull-down resistor.
pub struct WiredOrPullDown;
pub type OpenSourcePullDown = WiredOrPullDown;

/// Enable Input and Output in open-drain mode.
pub struct WiredAnd;
pub type OpenDrain = WiredAnd;
impl OutputMode for WiredAnd {}

/// Enable Input and Output in open-drain mode with glitch filter for input.
pub struct WiredAndWithFilter;
pub type OpenDrainWithFilter = WiredAndWithFilter;

/// Enable Input and Output in open-drain mode and pull-up resistor without
/// glitch filter for input.
pub struct WiredAndPullUp;
pub type OpenDrainPullUp = WiredAndPullUp;

/// Enable Input and Output in open-drain mode and pull-up resistor with
/// glitch filter for input.
pub struct WiredAndPullUpWithFilter;
pub type OpenDrainPullUpWithFilter = WiredAndPullUpWithFilter;

/// Enable Input and Output in open-drain mode and enable current drive without
/// glitch filter for input.
pub struct WiredAndDrive;
pub type OpenDrainDrive = WiredAndDrive;

/// Enable Input and Output in open-drain mode and enable current drive with
/// glitch filter for input.
pub struct WiredAndDriveWithFilter;
pub type OpenDrainDriveWithFilter = WiredAndDriveWithFilter;

/// Enable Input and Output in open-drain mode and pull-up resister and also
/// enable current drive without glitch filter for input.
pub struct WiredAndDrivePullUp;
pub type OpenDrainDrivePullUp = WiredAndDrivePullUp;

/// Enable Input and Output in open-drain mode and pull-up resistor and also
/// enable current drive with glitch filter for input.
pub struct WiredAndDrivePullUpWithFilter;
pub type OpenDrainDrivePullUpWithFilter = WiredAndDrivePullUpWithFilter;


/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, cmu: &mut CMU) -> Self::Parts;
}

macro_rules! gpio_pin { 
    ($PXi:ident, $i:expr, $mode: ident, $modegroup: ident, $outset:ident, $outclr: ident, $outtgl: ident) => {
        pub struct $PXi<MODE> {
            _mode: PhantomData<MODE>,
        }
        
        impl<MODE: OutputMode> OutputPin for $PXi<MODE> {
            fn set_high(&mut self) {
                // NOTE(unsafe) atomic write to a stateless register
                unsafe { (*efm32::GPIO::ptr()).$outset.write(|w| w.bits(1 << $i)) };
            }

            fn set_low(&mut self) {
                // NOTE(unsafe) atomic write to a stateless register
               unsafe { (*efm32::GPIO::ptr()).$outclr.write(|w| w.bits(1 << $i)) };
            }
        }

        impl<MODE: OutputMode> ToggleableOutputPin for $PXi<MODE> {
            fn toggle(&mut self) {
                unsafe { (*efm32::GPIO::ptr()).$outtgl.write(|w| w.bits(1 << $i)) };
            }
        }

        impl<MODE> $PXi<MODE> {
            pub fn into_open_drain(self) -> $PXi<OpenDrain> {
                unsafe {
                    (*efm32::GPIO::ptr())
                        .$modegroup
                        .modify(|_, w| w.$mode().wiredand())
                };
                $PXi { _mode: PhantomData }
            }
        }
    }
}

gpio_pin!(A0, 0, mode0, pa_model, pa_doutset, pa_doutclr, pa_douttgl);
gpio_pin!(B7, 7, mode7, pb_model, pb_doutset, pb_doutclr, pb_douttgl);
gpio_pin!(B8, 0, mode8, pb_modeh, pb_doutset, pb_doutclr, pb_douttgl);
gpio_pin!(B11, 3, mode11, pb_modeh, pb_doutset, pb_doutclr, pb_douttgl);
gpio_pin!(B13, 5, mode13, pb_modeh, pb_doutset, pb_doutclr, pb_douttgl);
gpio_pin!(B14, 6, mode14, pb_modeh, pb_doutset, pb_doutclr, pb_douttgl);
gpio_pin!(C0, 0, mode0, pc_model, pc_doutset, pc_doutclr, pc_douttgl);
gpio_pin!(C1, 1, mode1, pc_model, pc_doutset, pc_doutclr, pc_douttgl);
gpio_pin!(C14, 6, mode14, pc_modeh, pc_doutset, pc_doutclr, pc_douttgl);
gpio_pin!(C15, 7, mode15, pc_modeh, pc_doutset, pc_doutclr, pc_douttgl);
gpio_pin!(E12, 4, mode12, pe_modeh, pe_doutset, pe_doutclr, pe_douttgl);
gpio_pin!(E13, 5, mode13, pe_modeh, pe_doutset, pe_doutclr, pe_douttgl);
gpio_pin!(F0, 0, mode0, pf_model, pf_doutset, pf_doutclr, pf_douttgl);
gpio_pin!(F1, 1, mode1, pf_model, pf_doutset, pf_doutclr, pf_douttgl);
gpio_pin!(F2, 2, mode2, pf_model, pf_doutset, pf_doutclr, pf_douttgl);

/// GPIO parts
pub struct Parts {
    /// Pin
    pub a0: A0<Disabled>,
    pub b7: B7<Disabled>,
    pub b8: B8<Disabled>,
    pub b11: B11<Disabled>,
    pub b13: B13<Disabled>,
    pub b14: B14<Disabled>,
    pub c0: C0<Disabled>,
    pub c1: C1<Disabled>,
    pub c14: C14<Disabled>,
    pub c15: C15<Disabled>,
    pub e12: E12<Disabled>,
    pub e13: E13<Disabled>,
    pub f0: F0<Disabled>,
    pub f1: F1<Disabled>,
    pub f2: F2<Disabled>,
}

impl GpioExt for GPIO {
    type Parts = Parts;

    fn split(self, cmu: &mut CMU) -> Self::Parts {
        cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));

        Parts {
            a0: A0 { _mode: PhantomData },
            b7: B7 { _mode: PhantomData },
            b8: B8 { _mode: PhantomData },
            b11: B11 { _mode: PhantomData },
            b13: B13 { _mode: PhantomData },
            b14: B14 { _mode: PhantomData },
            c0: C0 { _mode: PhantomData },
            c1: C1 { _mode: PhantomData },
            c14: C14 { _mode: PhantomData },
            c15: C15 { _mode: PhantomData },
            e12: E12 { _mode: PhantomData },
            e13: E13 { _mode: PhantomData },
            f0: F0 { _mode: PhantomData },
            f1: F1 { _mode: PhantomData },
            f2: F2 { _mode: PhantomData },
        }
    }
}