#![doc = "Peripheral access API for EFM32GG12B810F1024GM64 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn SMU();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn ACMP0();
    fn ADC0();
    fn IDAC0();
    fn I2C0();
    fn I2C1();
    fn GPIO_ODD();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn USART1_RX();
    fn USART1_TX();
    fn USART2_RX();
    fn USART2_TX();
    fn UART0_RX();
    fn UART0_TX();
    fn UART1_RX();
    fn UART1_TX();
    fn LEUART0();
    fn LEUART1();
    fn LETIMER0();
    fn PCNT0();
    fn PCNT1();
    fn PCNT2();
    fn RTCC();
    fn CMU();
    fn MSC();
    fn CRYPTO0();
    fn CRYOTIMER();
    fn FPUEH();
    fn USART3_RX();
    fn USART3_TX();
    fn USART4_RX();
    fn USART4_TX();
    fn WTIMER0();
    fn WTIMER1();
    fn VDAC0();
    fn CSEN();
    fn LESENSE();
    fn EBI();
    fn ACMP2();
    fn ADC1();
    fn LCD();
    fn SDIO();
    fn CAN0();
    fn CAN1();
    fn USB();
    fn RTC();
    fn WDOG1();
    fn LETIMER1();
    fn TRNG0();
    fn QSPI0();
    fn PDM();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: EMU },
    Vector { _handler: WDOG0 },
    Vector { _handler: LDMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: SMU },
    Vector { _handler: TIMER0 },
    Vector {
        _handler: USART0_RX,
    },
    Vector {
        _handler: USART0_TX,
    },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: IDAC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector {
        _handler: USART2_RX,
    },
    Vector {
        _handler: USART2_TX,
    },
    Vector { _handler: UART0_RX },
    Vector { _handler: UART0_TX },
    Vector { _handler: UART1_RX },
    Vector { _handler: UART1_TX },
    Vector { _handler: LEUART0 },
    Vector { _handler: LEUART1 },
    Vector { _handler: LETIMER0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: PCNT1 },
    Vector { _handler: PCNT2 },
    Vector { _handler: RTCC },
    Vector { _handler: CMU },
    Vector { _handler: MSC },
    Vector { _handler: CRYPTO0 },
    Vector {
        _handler: CRYOTIMER,
    },
    Vector { _handler: FPUEH },
    Vector {
        _handler: USART3_RX,
    },
    Vector {
        _handler: USART3_TX,
    },
    Vector {
        _handler: USART4_RX,
    },
    Vector {
        _handler: USART4_TX,
    },
    Vector { _handler: WTIMER0 },
    Vector { _handler: WTIMER1 },
    Vector { _handler: VDAC0 },
    Vector { _handler: CSEN },
    Vector { _handler: LESENSE },
    Vector { _handler: EBI },
    Vector { _handler: ACMP2 },
    Vector { _handler: ADC1 },
    Vector { _handler: LCD },
    Vector { _handler: SDIO },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: USB },
    Vector { _handler: RTC },
    Vector { _handler: WDOG1 },
    Vector { _handler: LETIMER1 },
    Vector { _handler: TRNG0 },
    Vector { _handler: QSPI0 },
    Vector { _handler: PDM },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU = 0,
    #[doc = "1 - WDOG0"]
    WDOG0 = 1,
    #[doc = "2 - LDMA"]
    LDMA = 2,
    #[doc = "3 - GPIO_EVEN"]
    GPIO_EVEN = 3,
    #[doc = "4 - SMU"]
    SMU = 4,
    #[doc = "5 - TIMER0"]
    TIMER0 = 5,
    #[doc = "6 - USART0_RX"]
    USART0_RX = 6,
    #[doc = "7 - USART0_TX"]
    USART0_TX = 7,
    #[doc = "8 - ACMP0"]
    ACMP0 = 8,
    #[doc = "9 - ADC0"]
    ADC0 = 9,
    #[doc = "10 - IDAC0"]
    IDAC0 = 10,
    #[doc = "11 - I2C0"]
    I2C0 = 11,
    #[doc = "12 - I2C1"]
    I2C1 = 12,
    #[doc = "13 - GPIO_ODD"]
    GPIO_ODD = 13,
    #[doc = "14 - TIMER1"]
    TIMER1 = 14,
    #[doc = "15 - TIMER2"]
    TIMER2 = 15,
    #[doc = "16 - TIMER3"]
    TIMER3 = 16,
    #[doc = "17 - USART1_RX"]
    USART1_RX = 17,
    #[doc = "18 - USART1_TX"]
    USART1_TX = 18,
    #[doc = "19 - USART2_RX"]
    USART2_RX = 19,
    #[doc = "20 - USART2_TX"]
    USART2_TX = 20,
    #[doc = "21 - UART0_RX"]
    UART0_RX = 21,
    #[doc = "22 - UART0_TX"]
    UART0_TX = 22,
    #[doc = "23 - UART1_RX"]
    UART1_RX = 23,
    #[doc = "24 - UART1_TX"]
    UART1_TX = 24,
    #[doc = "25 - LEUART0"]
    LEUART0 = 25,
    #[doc = "26 - LEUART1"]
    LEUART1 = 26,
    #[doc = "27 - LETIMER0"]
    LETIMER0 = 27,
    #[doc = "28 - PCNT0"]
    PCNT0 = 28,
    #[doc = "29 - PCNT1"]
    PCNT1 = 29,
    #[doc = "30 - PCNT2"]
    PCNT2 = 30,
    #[doc = "31 - RTCC"]
    RTCC = 31,
    #[doc = "32 - CMU"]
    CMU = 32,
    #[doc = "33 - MSC"]
    MSC = 33,
    #[doc = "34 - CRYPTO0"]
    CRYPTO0 = 34,
    #[doc = "35 - CRYOTIMER"]
    CRYOTIMER = 35,
    #[doc = "36 - FPUEH"]
    FPUEH = 36,
    #[doc = "37 - USART3_RX"]
    USART3_RX = 37,
    #[doc = "38 - USART3_TX"]
    USART3_TX = 38,
    #[doc = "39 - USART4_RX"]
    USART4_RX = 39,
    #[doc = "40 - USART4_TX"]
    USART4_TX = 40,
    #[doc = "41 - WTIMER0"]
    WTIMER0 = 41,
    #[doc = "42 - WTIMER1"]
    WTIMER1 = 42,
    #[doc = "43 - VDAC0"]
    VDAC0 = 43,
    #[doc = "44 - CSEN"]
    CSEN = 44,
    #[doc = "45 - LESENSE"]
    LESENSE = 45,
    #[doc = "46 - EBI"]
    EBI = 46,
    #[doc = "47 - ACMP2"]
    ACMP2 = 47,
    #[doc = "48 - ADC1"]
    ADC1 = 48,
    #[doc = "49 - LCD"]
    LCD = 49,
    #[doc = "50 - SDIO"]
    SDIO = 50,
    #[doc = "51 - CAN0"]
    CAN0 = 51,
    #[doc = "52 - CAN1"]
    CAN1 = 52,
    #[doc = "53 - USB"]
    USB = 53,
    #[doc = "54 - RTC"]
    RTC = 54,
    #[doc = "55 - WDOG1"]
    WDOG1 = 55,
    #[doc = "56 - LETIMER1"]
    LETIMER1 = 56,
    #[doc = "57 - TRNG0"]
    TRNG0 = 57,
    #[doc = "58 - QSPI0"]
    QSPI0 = 58,
    #[doc = "59 - PDM"]
    PDM = 59,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const msc::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MSC::ptr() }
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emu::RegisterBlock {
        0x400e_3000 as *const _
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMU::ptr() }
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmu::RegisterBlock {
        0x400e_5000 as *const _
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RMU::ptr() }
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmu::RegisterBlock {
        0x400e_4000 as *const _
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMU::ptr() }
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "CRYPTO0"]
pub struct CRYPTO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO0 {}
impl CRYPTO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto0::RegisterBlock {
        0x400f_0000 as *const _
    }
}
impl Deref for CRYPTO0 {
    type Target = crypto0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTO0::ptr() }
    }
}
#[doc = "CRYPTO0"]
pub mod crypto0;
#[doc = "LESENSE"]
pub struct LESENSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LESENSE {}
impl LESENSE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lesense::RegisterBlock {
        0x4005_5000 as *const _
    }
}
impl Deref for LESENSE {
    type Target = lesense::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LESENSE::ptr() }
    }
}
#[doc = "LESENSE"]
pub mod lesense;
#[doc = "EBI"]
pub struct EBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EBI {}
impl EBI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ebi::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for EBI {
    type Target = ebi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EBI::ptr() }
    }
}
#[doc = "EBI"]
pub mod ebi;
#[doc = "SDIO"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdio::RegisterBlock {
        0x400f_1000 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "SDIO"]
pub mod sdio;
#[doc = "PDM"]
pub struct PDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM {}
impl PDM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdm::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for PDM {
    type Target = pdm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDM::ptr() }
    }
}
#[doc = "PDM"]
pub mod pdm;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prs::RegisterBlock {
        0x400e_6000 as *const _
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PRS::ptr() }
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "LDMA"]
pub struct LDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LDMA {}
impl LDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ldma::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for LDMA {
    type Target = ldma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LDMA::ptr() }
    }
}
#[doc = "LDMA"]
pub mod ldma;
#[doc = "FPUEH"]
pub struct FPUEH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPUEH {}
impl FPUEH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpueh::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for FPUEH {
    type Target = fpueh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPUEH::ptr() }
    }
}
#[doc = "FPUEH"]
pub mod fpueh;
#[doc = "GPCRC"]
pub struct GPCRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPCRC {}
impl GPCRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpcrc::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for GPCRC {
    type Target = gpcrc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPCRC::ptr() }
    }
}
#[doc = "GPCRC"]
pub mod gpcrc;
#[doc = "CAN0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "CAN0"]
pub mod can0;
#[doc = "CAN1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "CAN1"]
pub mod can1;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        0x4001_8400 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer2::RegisterBlock {
        0x4001_8800 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER2"]
pub mod timer2;
#[doc = "TIMER3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer3::RegisterBlock {
        0x4001_8c00 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "TIMER3"]
pub mod timer3;
#[doc = "WTIMER0"]
pub struct WTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER0 {}
impl WTIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        0x4001_a000 as *const _
    }
}
impl Deref for WTIMER0 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER0::ptr() }
    }
}
#[doc = "WTIMER0"]
pub mod wtimer0;
#[doc = "WTIMER1"]
pub struct WTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER1 {}
impl WTIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer1::RegisterBlock {
        0x4001_a400 as *const _
    }
}
impl Deref for WTIMER1 {
    type Target = wtimer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WTIMER1::ptr() }
    }
}
#[doc = "WTIMER1"]
pub mod wtimer1;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart2::RegisterBlock {
        0x4001_0800 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART2"]
pub mod usart2;
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart3::RegisterBlock {
        0x4001_0c00 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USART3"]
pub mod usart3;
#[doc = "USART4"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart4::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "USART4"]
pub mod usart4;
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "QSPI0"]
pub struct QSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI0 {}
impl QSPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        0x4001_c400 as *const _
    }
}
impl Deref for QSPI0 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI0::ptr() }
    }
}
#[doc = "QSPI0"]
pub mod qspi0;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const leuart0::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEUART0::ptr() }
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "LEUART1"]
pub struct LEUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART1 {}
impl LEUART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const leuart1::RegisterBlock {
        0x4006_a400 as *const _
    }
}
impl Deref for LEUART1 {
    type Target = leuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEUART1::ptr() }
    }
}
#[doc = "LEUART1"]
pub mod leuart1;
#[doc = "LETIMER0"]
pub struct LETIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER0 {}
impl LETIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const letimer0::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for LETIMER0 {
    type Target = letimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LETIMER0::ptr() }
    }
}
#[doc = "LETIMER0"]
pub mod letimer0;
#[doc = "LETIMER1"]
pub struct LETIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER1 {}
impl LETIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const letimer1::RegisterBlock {
        0x4006_6400 as *const _
    }
}
impl Deref for LETIMER1 {
    type Target = letimer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LETIMER1::ptr() }
    }
}
#[doc = "LETIMER1"]
pub mod letimer1;
#[doc = "CRYOTIMER"]
pub struct CRYOTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYOTIMER {}
impl CRYOTIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryotimer::RegisterBlock {
        0x4008_f000 as *const _
    }
}
impl Deref for CRYOTIMER {
    type Target = cryotimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYOTIMER::ptr() }
    }
}
#[doc = "CRYOTIMER"]
pub mod cryotimer;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt0::RegisterBlock {
        0x4006_e000 as *const _
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCNT0::ptr() }
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "PCNT1"]
pub struct PCNT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT1 {}
impl PCNT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt1::RegisterBlock {
        0x4006_e400 as *const _
    }
}
impl Deref for PCNT1 {
    type Target = pcnt1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCNT1::ptr() }
    }
}
#[doc = "PCNT1"]
pub mod pcnt1;
#[doc = "PCNT2"]
pub struct PCNT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT2 {}
impl PCNT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt2::RegisterBlock {
        0x4006_e800 as *const _
    }
}
impl Deref for PCNT2 {
    type Target = pcnt2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCNT2::ptr() }
    }
}
#[doc = "PCNT2"]
pub mod pcnt2;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4008_9400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C1"]
pub mod i2c1;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4008_2000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "ADC1"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4008_2400 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "ADC1"]
pub mod adc1;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp0::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACMP0::ptr() }
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "ACMP1"]
pub struct ACMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP1 {}
impl ACMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp1::RegisterBlock {
        0x4008_0400 as *const _
    }
}
impl Deref for ACMP1 {
    type Target = acmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACMP1::ptr() }
    }
}
#[doc = "ACMP1"]
pub mod acmp1;
#[doc = "ACMP2"]
pub struct ACMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP2 {}
impl ACMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp2::RegisterBlock {
        0x4008_0800 as *const _
    }
}
impl Deref for ACMP2 {
    type Target = acmp2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACMP2::ptr() }
    }
}
#[doc = "ACMP2"]
pub mod acmp2;
#[doc = "VDAC0"]
pub struct VDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VDAC0 {}
impl VDAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vdac0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for VDAC0 {
    type Target = vdac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VDAC0::ptr() }
    }
}
#[doc = "VDAC0"]
pub mod vdac0;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "IDAC0"]
pub struct IDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAC0 {}
impl IDAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const idac0::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for IDAC0 {
    type Target = idac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IDAC0::ptr() }
    }
}
#[doc = "IDAC0"]
pub mod idac0;
#[doc = "CSEN"]
pub struct CSEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSEN {}
impl CSEN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csen::RegisterBlock {
        0x4008_e000 as *const _
    }
}
impl Deref for CSEN {
    type Target = csen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CSEN::ptr() }
    }
}
#[doc = "CSEN"]
pub mod csen;
#[doc = "LCD"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd::RegisterBlock {
        0x4005_4000 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "LCD"]
pub mod lcd;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "RTCC"]
pub struct RTCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCC {}
impl RTCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtcc::RegisterBlock {
        0x4006_2000 as *const _
    }
}
impl Deref for RTCC {
    type Target = rtcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTCC::ptr() }
    }
}
#[doc = "RTCC"]
pub mod rtcc;
#[doc = "WDOG0"]
pub struct WDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG0 {}
impl WDOG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog0::RegisterBlock {
        0x4005_2000 as *const _
    }
}
impl Deref for WDOG0 {
    type Target = wdog0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG0::ptr() }
    }
}
#[doc = "WDOG0"]
pub mod wdog0;
#[doc = "WDOG1"]
pub struct WDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG1 {}
impl WDOG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog1::RegisterBlock {
        0x4005_2400 as *const _
    }
}
impl Deref for WDOG1 {
    type Target = wdog1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG1::ptr() }
    }
}
#[doc = "WDOG1"]
pub mod wdog1;
#[doc = "ETM"]
pub struct ETM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETM {}
impl ETM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const etm::RegisterBlock {
        0xe004_1000 as *const _
    }
}
impl Deref for ETM {
    type Target = etm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETM::ptr() }
    }
}
#[doc = "ETM"]
pub mod etm;
#[doc = "SMU"]
pub struct SMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMU {}
impl SMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smu::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for SMU {
    type Target = smu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMU::ptr() }
    }
}
#[doc = "SMU"]
pub mod smu;
#[doc = "TRNG0"]
pub struct TRNG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG0 {}
impl TRNG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng0::RegisterBlock {
        0x4001_d000 as *const _
    }
}
impl Deref for TRNG0 {
    type Target = trng0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG0::ptr() }
    }
}
#[doc = "TRNG0"]
pub mod trng0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "CRYPTO0"]
    pub CRYPTO0: CRYPTO0,
    #[doc = "LESENSE"]
    pub LESENSE: LESENSE,
    #[doc = "EBI"]
    pub EBI: EBI,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "PDM"]
    pub PDM: PDM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "LDMA"]
    pub LDMA: LDMA,
    #[doc = "FPUEH"]
    pub FPUEH: FPUEH,
    #[doc = "GPCRC"]
    pub GPCRC: GPCRC,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "WTIMER0"]
    pub WTIMER0: WTIMER0,
    #[doc = "WTIMER1"]
    pub WTIMER1: WTIMER1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "QSPI0"]
    pub QSPI0: QSPI0,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "LEUART1"]
    pub LEUART1: LEUART1,
    #[doc = "LETIMER0"]
    pub LETIMER0: LETIMER0,
    #[doc = "LETIMER1"]
    pub LETIMER1: LETIMER1,
    #[doc = "CRYOTIMER"]
    pub CRYOTIMER: CRYOTIMER,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "PCNT1"]
    pub PCNT1: PCNT1,
    #[doc = "PCNT2"]
    pub PCNT2: PCNT2,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "ACMP1"]
    pub ACMP1: ACMP1,
    #[doc = "ACMP2"]
    pub ACMP2: ACMP2,
    #[doc = "VDAC0"]
    pub VDAC0: VDAC0,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "IDAC0"]
    pub IDAC0: IDAC0,
    #[doc = "CSEN"]
    pub CSEN: CSEN,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RTCC"]
    pub RTCC: RTCC,
    #[doc = "WDOG0"]
    pub WDOG0: WDOG0,
    #[doc = "WDOG1"]
    pub WDOG1: WDOG1,
    #[doc = "ETM"]
    pub ETM: ETM,
    #[doc = "SMU"]
    pub SMU: SMU,
    #[doc = "TRNG0"]
    pub TRNG0: TRNG0,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            CRYPTO0: CRYPTO0 {
                _marker: PhantomData,
            },
            LESENSE: LESENSE {
                _marker: PhantomData,
            },
            EBI: EBI {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            PDM: PDM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            LDMA: LDMA {
                _marker: PhantomData,
            },
            FPUEH: FPUEH {
                _marker: PhantomData,
            },
            GPCRC: GPCRC {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            WTIMER0: WTIMER0 {
                _marker: PhantomData,
            },
            WTIMER1: WTIMER1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            QSPI0: QSPI0 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            LEUART1: LEUART1 {
                _marker: PhantomData,
            },
            LETIMER0: LETIMER0 {
                _marker: PhantomData,
            },
            LETIMER1: LETIMER1 {
                _marker: PhantomData,
            },
            CRYOTIMER: CRYOTIMER {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            PCNT1: PCNT1 {
                _marker: PhantomData,
            },
            PCNT2: PCNT2 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            ACMP1: ACMP1 {
                _marker: PhantomData,
            },
            ACMP2: ACMP2 {
                _marker: PhantomData,
            },
            VDAC0: VDAC0 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            IDAC0: IDAC0 {
                _marker: PhantomData,
            },
            CSEN: CSEN {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RTCC: RTCC {
                _marker: PhantomData,
            },
            WDOG0: WDOG0 {
                _marker: PhantomData,
            },
            WDOG1: WDOG1 {
                _marker: PhantomData,
            },
            ETM: ETM {
                _marker: PhantomData,
            },
            SMU: SMU {
                _marker: PhantomData,
            },
            TRNG0: TRNG0 {
                _marker: PhantomData,
            },
        }
    }
}
