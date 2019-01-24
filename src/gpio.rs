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

use crate::efm32;

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

/// GPIO pin handler.
/// Currently it doesn't hold any data.
pub struct GPIO;

impl GPIO {
    /// Create new GPIO device, this should `take` GPIO
    /// device exclusively, but currently it doesn't.
    pub fn new(cmu: &mut efm32::CMU) -> Self {
        cmu.hfperclken0.modify(|_, w| w.gpio().bit(true));

        GPIO
    }

    /// Create new pin with specific mode.
    /// For example:
    /// ```no_run
    /// let gpio = GPIO;
    /// let mut a0 = gpio.split<A0<OpenDrain>>();
    ///
    /// a0.set_high();
    /// ```
    pub fn split<MODE: GPIOPinSplitter>(&self) -> MODE::GPIOPin {
        MODE::split()
    }
}

/// Traits to split GPIO into mode specific pin
/// Available pins in tomu/efm32hg309f64 are set
/// to implement this trait.
pub trait GPIOPinSplitter {
    type GPIOPin;

    /// Actual split implementation for each pin
    fn split() -> Self::GPIOPin;
}

macro_rules! gpio_pin_splitter {
    ($pin_struct:ident,
     $io_mode:ident,
     $modegroup:ident,
     $mode:ident,
     $setter:ident) => {
        impl GPIOPinSplitter for $pin_struct<$io_mode> {
            type GPIOPin = $pin_struct<$io_mode>;

            fn split() -> Self::GPIOPin {
                unsafe {
                    (*efm32::GPIO::ptr())
                        .$modegroup
                        .modify(|_, w| w.$mode().$setter())
                };
                $pin_struct { _m: PhantomData }
            }
        }
    };

    ($pin_struct:ident,
     $io_mode:ident,
     $modegroup:ident,
     $mode:ident,
     $setter:ident,
     $shift:expr,
     $outset:ident) => {
        impl GPIOPinSplitter for $pin_struct<$io_mode> {
            type GPIOPin = $pin_struct<$io_mode>;

            fn split() -> Self::GPIOPin {
                unsafe {
                    (*efm32::GPIO::ptr())
                        .$modegroup
                        .modify(|_, w| w.$mode().$setter());
                    (*efm32::GPIO::ptr()).$outset.write(|w| w.bits(1 << $shift));
                };
                $pin_struct { _m: PhantomData }
            }
        }
    };
}

macro_rules! gpio_in_impl {
    ($pin_struct:ident,
     $io_mode:ident,
     $in:ident,
     $shift:expr) => {
        impl InputPin for $pin_struct<$io_mode> {
            fn is_high(&self) -> bool {
                let pos = 1 << $shift;
                unsafe { ((*efm32::GPIO::ptr()).$in.read().bits() & pos) == pos }
            }

            fn is_low(&self) -> bool {
                !self.is_high()
            }
        }
    };
}

macro_rules! gpio_out_impl {
    ($pin_struct:ident,
     $io_mode:ident,
     $shift: expr,
     $outset:ident,
     $outclr:ident,
     $outtgl:ident) => {
        impl OutputPin for $pin_struct<$io_mode> {
            fn set_low(&mut self) {
                unsafe { (*efm32::GPIO::ptr()).$outclr.write(|w| w.bits(1 << $shift)) };
            }

            fn set_high(&mut self) {
                unsafe { (*efm32::GPIO::ptr()).$outset.write(|w| w.bits(1 << $shift)) };
            }
        }

        impl ToggleableOutputPin for $pin_struct<$io_mode> {
            fn toggle(&mut self) {
                unsafe { (*efm32::GPIO::ptr()).$outtgl.write(|w| w.bits(1 << $shift)) };
            }
        }
    };
}

macro_rules! gpio {
    ($pin_struct:ident,
     $mode:ident,
     $shift:expr,
     $ctrl:ident,
     $modegroup:ident,
     $out:ident,
     $outset:ident,
     $outclr:ident,
     $outtgl:ident,
     $in:ident,
     $lock:ident) => {
        pub struct $pin_struct<Mode> {
            _m: PhantomData<Mode>,
        }

        // Disabled pin variants
        gpio_pin_splitter!($pin_struct, Disabled, $modegroup, $mode, disabled);
        gpio_pin_splitter!(
            $pin_struct,
            DisabledPullUp,
            $modegroup,
            $mode,
            disabled,
            $shift,
            $outset
        );

        // Input pin variants
        gpio_pin_splitter!($pin_struct, Input, $modegroup, $mode, input);
        gpio_pin_splitter!(
            $pin_struct,
            InputWithFilter,
            $modegroup,
            $mode,
            input,
            $shift,
            $outset
        );

        gpio_pin_splitter!($pin_struct, InputPullDown, $modegroup, $mode, inputpull);
        gpio_pin_splitter!(
            $pin_struct,
            InputPullUp,
            $modegroup,
            $mode,
            inputpull,
            $shift,
            $outset
        );

        gpio_pin_splitter!(
            $pin_struct,
            InputPullDownWithFilter,
            $modegroup,
            $mode,
            inputpullfilter
        );
        gpio_pin_splitter!(
            $pin_struct,
            InputPullUpWithFilter,
            $modegroup,
            $mode,
            inputpullfilter,
            $shift,
            $outset
        );

        gpio_in_impl!($pin_struct, Input, $in, $shift);
        gpio_in_impl!($pin_struct, InputWithFilter, $in, $shift);
        gpio_in_impl!($pin_struct, InputPullDown, $in, $shift);
        gpio_in_impl!($pin_struct, InputPullUp, $in, $shift);
        gpio_in_impl!($pin_struct, InputPullDownWithFilter, $in, $shift);
        gpio_in_impl!($pin_struct, InputPullUpWithFilter, $in, $shift);

        // Output pin variants
        gpio_pin_splitter!($pin_struct, PushPull, $modegroup, $mode, pushpull);
        gpio_pin_splitter!($pin_struct, PushPullDrive, $modegroup, $mode, pushpulldrive);
        gpio_pin_splitter!($pin_struct, WiredOr, $modegroup, $mode, wiredor);
        gpio_pin_splitter!(
            $pin_struct,
            WiredOrPullDown,
            $modegroup,
            $mode,
            wiredorpulldown
        );
        gpio_pin_splitter!($pin_struct, WiredAnd, $modegroup, $mode, wiredand);
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndWithFilter,
            $modegroup,
            $mode,
            wiredandfilter
        );
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndPullUp,
            $modegroup,
            $mode,
            wiredandpullup
        );
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndPullUpWithFilter,
            $modegroup,
            $mode,
            wiredandpullupfilter
        );
        gpio_pin_splitter!($pin_struct, WiredAndDrive, $modegroup, $mode, wiredanddrive);
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndDriveWithFilter,
            $modegroup,
            $mode,
            wiredanddrivefilter
        );
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndDrivePullUp,
            $modegroup,
            $mode,
            wiredanddrivepullup
        );
        gpio_pin_splitter!(
            $pin_struct,
            WiredAndDrivePullUpWithFilter,
            $modegroup,
            $mode,
            wiredanddrivepullupfilter
        );

        gpio_out_impl!($pin_struct, PushPull, $shift, $outset, $outclr, $outtgl);
        gpio_out_impl!(
            $pin_struct,
            PushPullDrive,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!($pin_struct, WiredOr, $shift, $outset, $outclr, $outtgl);
        gpio_out_impl!(
            $pin_struct,
            WiredOrPullDown,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!($pin_struct, WiredAnd, $shift, $outset, $outclr, $outtgl);
        gpio_out_impl!(
            $pin_struct,
            WiredAndWithFilter,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndPullUp,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndPullUpWithFilter,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndDrive,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndDriveWithFilter,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndDrivePullUp,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
        gpio_out_impl!(
            $pin_struct,
            WiredAndDrivePullUpWithFilter,
            $shift,
            $outset,
            $outclr,
            $outtgl
        );
    };
}

/// Struct for each pin and its traits implementation live in this pin module.
/// Only pin available in efm32hg309f64 listed here.
pub mod pin {
    use super::*;
    use core::marker::PhantomData;
    use embedded_hal::digital::{InputPin, OutputPin, ToggleableOutputPin};

    gpio!(
        A0,
        mode0,
        0,
        pa_ctrl,
        pa_model,
        pa_dout,
        pa_doutset,
        pa_doutclr,
        pa_douttgl,
        pa_din,
        pa_pinlockn
    );
    gpio!(
        B7,
        mode7,
        7,
        pb_ctrl,
        pb_model,
        pb_dout,
        pb_doutset,
        pb_doutclr,
        pb_douttgl,
        pb_din,
        pb_pinlockn
    );
    gpio!(
        B8,
        mode8,
        0,
        pb_ctrl,
        pb_modeh,
        pb_dout,
        pb_doutset,
        pb_doutclr,
        pb_douttgl,
        pb_din,
        pb_pinlockn
    );
    gpio!(
        B11,
        mode11,
        3,
        pb_ctrl,
        pb_modeh,
        pb_dout,
        pb_doutset,
        pb_doutclr,
        pb_douttgl,
        pb_din,
        pb_pinlockn
    );
    gpio!(
        B13,
        mode13,
        5,
        pb_ctrl,
        pb_modeh,
        pb_dout,
        pb_doutset,
        pb_doutclr,
        pb_douttgl,
        pb_din,
        pb_pinlockn
    );
    gpio!(
        B14,
        mode14,
        6,
        pb_ctrl,
        pb_modeh,
        pb_dout,
        pb_doutset,
        pb_doutclr,
        pb_douttgl,
        pb_din,
        pb_pinlockn
    );
    gpio!(
        C0,
        mode0,
        0,
        pc_ctrl,
        pc_model,
        pc_dout,
        pc_doutset,
        pc_doutclr,
        pc_douttgl,
        pc_din,
        pc_pinlockn
    );
    gpio!(
        C1,
        mode1,
        1,
        pc_ctrl,
        pc_model,
        pc_dout,
        pc_doutset,
        pc_doutclr,
        pc_douttgl,
        pc_din,
        pc_pinlockn
    );
    gpio!(
        C14,
        mode14,
        6,
        pc_ctrl,
        pc_modeh,
        pc_dout,
        pc_doutset,
        pc_doutclr,
        pc_douttgl,
        pc_din,
        pc_pinlockn
    );
    gpio!(
        C15,
        mode15,
        7,
        pc_ctrl,
        pc_modeh,
        pc_dout,
        pc_doutset,
        pc_doutclr,
        pc_douttgl,
        pc_din,
        pc_pinlockn
    );
    gpio!(
        E12,
        mode12,
        4,
        pe_ctrl,
        pe_modeh,
        pe_dout,
        pe_doutset,
        pe_doutclr,
        pe_douttgl,
        pe_din,
        pe_pinlockn
    );
    gpio!(
        E13,
        mode13,
        5,
        pe_ctrl,
        pe_modeh,
        pe_dout,
        pe_doutset,
        pe_doutclr,
        pe_douttgl,
        pe_din,
        pe_pinlockn
    );
    gpio!(
        F0,
        mode0,
        0,
        pf_ctrl,
        pf_model,
        pf_dout,
        pf_doutset,
        pf_doutclr,
        pf_douttgl,
        pf_din,
        pf_pinlockn
    );
    gpio!(
        F1,
        mode1,
        1,
        pf_ctrl,
        pf_model,
        pf_dout,
        pf_doutset,
        pf_doutclr,
        pf_douttgl,
        pf_din,
        pf_pinlockn
    );
    gpio!(
        F2,
        mode2,
        2,
        pf_ctrl,
        pf_model,
        pf_dout,
        pf_doutset,
        pf_doutclr,
        pf_douttgl,
        pf_din,
        pf_pinlockn
    );
}
