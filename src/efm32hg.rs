use efm32;
use crate::tomu::Tomu;

/// Holds all available tomu peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Core peripheral: Cache and branch predictor maintenance operations
    pub CBP: efm32::CBP,

    /// Core peripheral: CPUID
    pub CPUID: efm32::CPUID,

    /// Core peripheral: Debug Control Block
    pub DCB: efm32::DCB,

    /// Core peripheral: Data Watchpoint and Trace unit
    pub DWT: efm32::DWT,

    /// Core peripheral: Flash Patch and Breakpoint unit
    pub FPB: efm32::FPB,

    /// Core peripheral: Floating Point Unit
    pub FPU: efm32::FPU,

    /// Core peripheral: Instrumentation Trace Macrocell
    pub ITM: efm32::ITM,

    /// Core peripheral: Memory Protection Unit
    pub MPU: efm32::MPU,

    /// Core peripheral: Nested Vector Interrupt Controller
    pub NVIC: efm32::NVIC,

    /// Core peripheral: System Control Block
    pub SCB: efm32::SCB,

    /// Core peripheral: Trace Port Interface Unit
    pub TPIU: efm32::TPIU,

    /// efm32 peripheral: ACMP0
    pub ACMP0: efm32::ACMP0,

    /// efm32 peripheral: ADC0
    pub ADC0: efm32::ADC0,

    /// efm32 peripheral: AES
    pub AES: efm32::AES,

    /// efm32 peripheral: CMU
    pub CMU: efm32::CMU,

    /// efm32 peripheral: Device Info read only pages
    pub DEVINFO: efm32::DEVINFO,

    /// efm32 peripheral: DMA
    pub DMA: efm32::DMA,

    /// efm32 peripheral: EMU
    pub EMU: efm32::EMU,

    /// efm32 peripheral: GPIO
    pub GPIO: efm32::GPIO,

    /// efm32 peripheral: I2C0
    pub I2C0: efm32::I2C0,

    /// efm32 peripheral: IDAC0
    pub IDAC0: efm32::IDAC0,

    /// efm32 peripheral: LEUART0
    pub LEUART0: efm32::LEUART0,

    /// efm32 peripheral: MSC
    pub MSC: efm32::MSC,

    /// efm32 peripheral: MTB
    pub MTB: efm32::MTB,

    /// efm32 peripheral: PCNT0
    pub PCNT0: efm32::PCNT0,

    /// efm32 peripheral: PRS
    pub PRS: efm32::PRS,

    /// efm32 peripheral: RMU
    pub RMU: efm32::RMU,

    /// efm32 peripheral: RTC
    pub RTC: efm32::RTC,

    /// efm32 peripheral: SYSTICK
    pub SYST: efm32::SYST,

    /// efm32 peripheral: TIMER0
    pub TIMER0: efm32::TIMER0,

    /// efm32 peripheral: TIMER1
    pub TIMER1: efm32::TIMER1,

    /// efm32 peripheral: TIMER2
    pub TIMER2: efm32::TIMER2,

    /// efm32 peripheral: USART0
    pub USART0: efm32::USART0,

    /// efm32 peripheral: USART1
    pub USART1: efm32::USART1,

    /// efm32 peripheral: USB
    pub USB: efm32::USB,

    /// efm32 peripheral: VCMP
    pub VCMP: efm32::VCMP,

    /// efm32 peripheral: WDOG
    pub WDOG: efm32::WDOG,
}

impl Peripherals {
    /// Take `Peripherals`  instance, this is called `take`
    /// since we also take efm32's own `Peripherals` which will
    /// cause this method to panic if it's called more than once.
    pub fn take() -> Option<Self> {
        let p = efm32::Peripherals::take()?;
        let cp = efm32::CorePeripherals::take()?;

        Some(Self {
            // Core peripherals
            CBP: cp.CBP,
            CPUID: cp.CPUID,
            DCB: cp.DCB,
            DWT: cp.DWT,
            FPB: cp.FPB,
            FPU: cp.FPU,
            ITM: cp.ITM,
            MPU: cp.MPU,
            NVIC: cp.NVIC,
            SCB: cp.SCB,
            SYST: cp.SYST,
            TPIU: cp.TPIU,

            // efm32 peripherals
            ACMP0: p.ACMP0,
            ADC0: p.ADC0,
            AES: p.AES,
            CMU: p.CMU,
            DEVINFO: p.DEVINFO,
            DMA: p.DMA,
            EMU: p.EMU,
            GPIO: p.GPIO,
            I2C0: p.I2C0,
            IDAC0: p.IDAC0,
            LEUART0: p.LEUART0,
            MSC: p.MSC,
            MTB: p.MTB,
            PCNT0: p.PCNT0,
            PRS: p.PRS,
            RMU: p.RMU,
            RTC: p.RTC,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            USART0: p.USART0,
            USART1: p.USART1,
            USB: p.USB,
            VCMP: p.VCMP,
            WDOG: p.WDOG,
        })
    }

    pub fn constrain(self) -> Tomu {
        Tomu::from(self)
    }

}
