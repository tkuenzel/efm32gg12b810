#[doc = "Reader of register PPUFS"]
pub type R = crate::R<u32, super::PPUFS>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIPHID_A {
    #[doc = "0: Analog Comparator 0"]
    ACMP0 = 0,
    #[doc = "1: Analog Comparator 1"]
    ACMP1 = 1,
    #[doc = "2: Analog Comparator 2"]
    ACMP2 = 2,
    #[doc = "3: Analog to Digital Converter 0"]
    ADC0 = 3,
    #[doc = "4: Analog to Digital Converter 0"]
    ADC1 = 4,
    #[doc = "5: CAN 0"]
    CAN0 = 5,
    #[doc = "6: CAN 1"]
    CAN1 = 6,
    #[doc = "7: Clock Management Unit"]
    CMU = 7,
    #[doc = "8: CRYOTIMER"]
    CRYOTIMER = 8,
    #[doc = "9: Advanced Encryption Standard Accelerator"]
    CRYPTO0 = 9,
    #[doc = "10: Capacitive touch sense module"]
    CSEN = 10,
    #[doc = "11: Digital to Analog Converter 0"]
    VDAC0 = 11,
    #[doc = "12: Peripheral Reflex System"]
    PRS = 12,
    #[doc = "13: External Bus Interface"]
    EBI = 13,
    #[doc = "14: Energy Management Unit"]
    EMU = 14,
    #[doc = "15: FPU Exception Handler"]
    FPUEH = 15,
    #[doc = "16: General Purpose CRC"]
    GPCRC = 16,
    #[doc = "17: General purpose Input/Output"]
    GPIO = 17,
    #[doc = "18: I2C 0"]
    I2C0 = 18,
    #[doc = "19: I2C 1"]
    I2C1 = 19,
    #[doc = "20: Current Digital to Analog Converter 0"]
    IDAC0 = 20,
    #[doc = "21: Memory System Controller"]
    MSC = 21,
    #[doc = "22: Liquid Crystal Display Controller"]
    LCD = 22,
    #[doc = "23: Linked Direct Memory Access Controller"]
    LDMA = 23,
    #[doc = "24: Low Energy Sensor Interface"]
    LESENSE = 24,
    #[doc = "25: Low Energy Timer 0"]
    LETIMER0 = 25,
    #[doc = "26: Low Energy Timer 1"]
    LETIMER1 = 26,
    #[doc = "27: Low Energy UART 0"]
    LEUART0 = 27,
    #[doc = "28: Low Energy UART 1"]
    LEUART1 = 28,
    #[doc = "32: Pulse Counter 0"]
    PCNT0 = 32,
    #[doc = "33: Pulse Counter 1"]
    PCNT1 = 33,
    #[doc = "34: Pulse Counter 2"]
    PCNT2 = 34,
    #[doc = "35: PDM Interface "]
    PDM = 35,
    #[doc = "36: Quad-SPI"]
    QSPI0 = 36,
    #[doc = "37: Reset Management Unit"]
    RMU = 37,
    #[doc = "38: Real-Time Counter"]
    RTC = 38,
    #[doc = "39: Real-Time Counter and Calendar"]
    RTCC = 39,
    #[doc = "40: SDIO Controller"]
    SDIO = 40,
    #[doc = "41: Security Management Unit"]
    SMU = 41,
    #[doc = "42: Timer 0"]
    TIMER0 = 42,
    #[doc = "43: Timer 1"]
    TIMER1 = 43,
    #[doc = "44: Timer 2"]
    TIMER2 = 44,
    #[doc = "45: Timer 3"]
    TIMER3 = 45,
    #[doc = "46: True Random Number Generator 0"]
    TRNG0 = 46,
    #[doc = "47: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 47,
    #[doc = "48: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 48,
    #[doc = "49: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 49,
    #[doc = "50: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4 = 53,
    #[doc = "54: Universal Serial Bus Interface"]
    USB = 54,
    #[doc = "55: Watchdog"]
    WDOG0 = 55,
    #[doc = "56: Watchdog"]
    WDOG1 = 56,
    #[doc = "57: Wide Timer 0"]
    WTIMER0 = 57,
    #[doc = "58: Wide Timer 0"]
    WTIMER1 = 58,
}
impl From<PERIPHID_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPHID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIPHID`"]
pub type PERIPHID_R = crate::R<u8, PERIPHID_A>;
impl PERIPHID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERIPHID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERIPHID_A::ACMP0),
            1 => Val(PERIPHID_A::ACMP1),
            2 => Val(PERIPHID_A::ACMP2),
            3 => Val(PERIPHID_A::ADC0),
            4 => Val(PERIPHID_A::ADC1),
            5 => Val(PERIPHID_A::CAN0),
            6 => Val(PERIPHID_A::CAN1),
            7 => Val(PERIPHID_A::CMU),
            8 => Val(PERIPHID_A::CRYOTIMER),
            9 => Val(PERIPHID_A::CRYPTO0),
            10 => Val(PERIPHID_A::CSEN),
            11 => Val(PERIPHID_A::VDAC0),
            12 => Val(PERIPHID_A::PRS),
            13 => Val(PERIPHID_A::EBI),
            14 => Val(PERIPHID_A::EMU),
            15 => Val(PERIPHID_A::FPUEH),
            16 => Val(PERIPHID_A::GPCRC),
            17 => Val(PERIPHID_A::GPIO),
            18 => Val(PERIPHID_A::I2C0),
            19 => Val(PERIPHID_A::I2C1),
            20 => Val(PERIPHID_A::IDAC0),
            21 => Val(PERIPHID_A::MSC),
            22 => Val(PERIPHID_A::LCD),
            23 => Val(PERIPHID_A::LDMA),
            24 => Val(PERIPHID_A::LESENSE),
            25 => Val(PERIPHID_A::LETIMER0),
            26 => Val(PERIPHID_A::LETIMER1),
            27 => Val(PERIPHID_A::LEUART0),
            28 => Val(PERIPHID_A::LEUART1),
            32 => Val(PERIPHID_A::PCNT0),
            33 => Val(PERIPHID_A::PCNT1),
            34 => Val(PERIPHID_A::PCNT2),
            35 => Val(PERIPHID_A::PDM),
            36 => Val(PERIPHID_A::QSPI0),
            37 => Val(PERIPHID_A::RMU),
            38 => Val(PERIPHID_A::RTC),
            39 => Val(PERIPHID_A::RTCC),
            40 => Val(PERIPHID_A::SDIO),
            41 => Val(PERIPHID_A::SMU),
            42 => Val(PERIPHID_A::TIMER0),
            43 => Val(PERIPHID_A::TIMER1),
            44 => Val(PERIPHID_A::TIMER2),
            45 => Val(PERIPHID_A::TIMER3),
            46 => Val(PERIPHID_A::TRNG0),
            47 => Val(PERIPHID_A::UART0),
            48 => Val(PERIPHID_A::UART1),
            49 => Val(PERIPHID_A::USART0),
            50 => Val(PERIPHID_A::USART1),
            51 => Val(PERIPHID_A::USART2),
            52 => Val(PERIPHID_A::USART3),
            53 => Val(PERIPHID_A::USART4),
            54 => Val(PERIPHID_A::USB),
            55 => Val(PERIPHID_A::WDOG0),
            56 => Val(PERIPHID_A::WDOG1),
            57 => Val(PERIPHID_A::WTIMER0),
            58 => Val(PERIPHID_A::WTIMER1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == PERIPHID_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == PERIPHID_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `ACMP2`"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == PERIPHID_A::ACMP2
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == PERIPHID_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == PERIPHID_A::ADC1
    }
    #[doc = "Checks if the value of the field is `CAN0`"]
    #[inline(always)]
    pub fn is_can0(&self) -> bool {
        *self == PERIPHID_A::CAN0
    }
    #[doc = "Checks if the value of the field is `CAN1`"]
    #[inline(always)]
    pub fn is_can1(&self) -> bool {
        *self == PERIPHID_A::CAN1
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == PERIPHID_A::CMU
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == PERIPHID_A::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == PERIPHID_A::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == PERIPHID_A::CSEN
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == PERIPHID_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == PERIPHID_A::PRS
    }
    #[doc = "Checks if the value of the field is `EBI`"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == PERIPHID_A::EBI
    }
    #[doc = "Checks if the value of the field is `EMU`"]
    #[inline(always)]
    pub fn is_emu(&self) -> bool {
        *self == PERIPHID_A::EMU
    }
    #[doc = "Checks if the value of the field is `FPUEH`"]
    #[inline(always)]
    pub fn is_fpueh(&self) -> bool {
        *self == PERIPHID_A::FPUEH
    }
    #[doc = "Checks if the value of the field is `GPCRC`"]
    #[inline(always)]
    pub fn is_gpcrc(&self) -> bool {
        *self == PERIPHID_A::GPCRC
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PERIPHID_A::GPIO
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == PERIPHID_A::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == PERIPHID_A::I2C1
    }
    #[doc = "Checks if the value of the field is `IDAC0`"]
    #[inline(always)]
    pub fn is_idac0(&self) -> bool {
        *self == PERIPHID_A::IDAC0
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == PERIPHID_A::MSC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == PERIPHID_A::LCD
    }
    #[doc = "Checks if the value of the field is `LDMA`"]
    #[inline(always)]
    pub fn is_ldma(&self) -> bool {
        *self == PERIPHID_A::LDMA
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == PERIPHID_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == PERIPHID_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LETIMER1`"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == PERIPHID_A::LETIMER1
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == PERIPHID_A::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == PERIPHID_A::LEUART1
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == PERIPHID_A::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == PERIPHID_A::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == PERIPHID_A::PCNT2
    }
    #[doc = "Checks if the value of the field is `PDM`"]
    #[inline(always)]
    pub fn is_pdm(&self) -> bool {
        *self == PERIPHID_A::PDM
    }
    #[doc = "Checks if the value of the field is `QSPI0`"]
    #[inline(always)]
    pub fn is_qspi0(&self) -> bool {
        *self == PERIPHID_A::QSPI0
    }
    #[doc = "Checks if the value of the field is `RMU`"]
    #[inline(always)]
    pub fn is_rmu(&self) -> bool {
        *self == PERIPHID_A::RMU
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == PERIPHID_A::RTC
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == PERIPHID_A::RTCC
    }
    #[doc = "Checks if the value of the field is `SDIO`"]
    #[inline(always)]
    pub fn is_sdio(&self) -> bool {
        *self == PERIPHID_A::SDIO
    }
    #[doc = "Checks if the value of the field is `SMU`"]
    #[inline(always)]
    pub fn is_smu(&self) -> bool {
        *self == PERIPHID_A::SMU
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == PERIPHID_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == PERIPHID_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == PERIPHID_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == PERIPHID_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `TRNG0`"]
    #[inline(always)]
    pub fn is_trng0(&self) -> bool {
        *self == PERIPHID_A::TRNG0
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == PERIPHID_A::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == PERIPHID_A::UART1
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == PERIPHID_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == PERIPHID_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == PERIPHID_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == PERIPHID_A::USART3
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == PERIPHID_A::USART4
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == PERIPHID_A::USB
    }
    #[doc = "Checks if the value of the field is `WDOG0`"]
    #[inline(always)]
    pub fn is_wdog0(&self) -> bool {
        *self == PERIPHID_A::WDOG0
    }
    #[doc = "Checks if the value of the field is `WDOG1`"]
    #[inline(always)]
    pub fn is_wdog1(&self) -> bool {
        *self == PERIPHID_A::WDOG1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == PERIPHID_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == PERIPHID_A::WTIMER1
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0x7f) as u8)
    }
}
