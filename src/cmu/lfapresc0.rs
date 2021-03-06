#[doc = "Reader of register LFAPRESC0"]
pub type R = crate::R<u32, super::LFAPRESC0>;
#[doc = "Writer for register LFAPRESC0"]
pub type W = crate::W<u32, super::LFAPRESC0>;
#[doc = "Register LFAPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFAPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low Energy Timer 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LETIMER0_A {
    #[doc = "0: LFACLKLETIMER0 = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLETIMER0 = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLETIMER0 = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLETIMER0 = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKLETIMER0 = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKLETIMER0 = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKLETIMER0 = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKLETIMER0 = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKLETIMER0 = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKLETIMER0 = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<LETIMER0_A> for u8 {
    #[inline(always)]
    fn from(variant: LETIMER0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LETIMER0`"]
pub type LETIMER0_R = crate::R<u8, LETIMER0_A>;
impl LETIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETIMER0_A {
        match self.bits {
            0 => LETIMER0_A::DIV1,
            1 => LETIMER0_A::DIV2,
            2 => LETIMER0_A::DIV4,
            3 => LETIMER0_A::DIV8,
            4 => LETIMER0_A::DIV16,
            5 => LETIMER0_A::DIV32,
            6 => LETIMER0_A::DIV64,
            7 => LETIMER0_A::DIV128,
            8 => LETIMER0_A::DIV256,
            9 => LETIMER0_A::DIV512,
            10 => LETIMER0_A::DIV1024,
            11 => LETIMER0_A::DIV2048,
            12 => LETIMER0_A::DIV4096,
            13 => LETIMER0_A::DIV8192,
            14 => LETIMER0_A::DIV16384,
            15 => LETIMER0_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0_A::DIV32768
    }
}
#[doc = "Write proxy for field `LETIMER0`"]
pub struct LETIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LETIMER0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Low Energy Timer 1 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LETIMER1_A {
    #[doc = "0: LFACLKLETIMER1 = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLETIMER1 = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLETIMER1 = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLETIMER1 = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKLETIMER1 = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKLETIMER1 = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKLETIMER1 = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKLETIMER1 = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKLETIMER1 = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKLETIMER1 = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKLETIMER1 = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKLETIMER1 = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKLETIMER1 = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKLETIMER1 = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKLETIMER1 = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKLETIMER1 = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<LETIMER1_A> for u8 {
    #[inline(always)]
    fn from(variant: LETIMER1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LETIMER1`"]
pub type LETIMER1_R = crate::R<u8, LETIMER1_A>;
impl LETIMER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETIMER1_A {
        match self.bits {
            0 => LETIMER1_A::DIV1,
            1 => LETIMER1_A::DIV2,
            2 => LETIMER1_A::DIV4,
            3 => LETIMER1_A::DIV8,
            4 => LETIMER1_A::DIV16,
            5 => LETIMER1_A::DIV32,
            6 => LETIMER1_A::DIV64,
            7 => LETIMER1_A::DIV128,
            8 => LETIMER1_A::DIV256,
            9 => LETIMER1_A::DIV512,
            10 => LETIMER1_A::DIV1024,
            11 => LETIMER1_A::DIV2048,
            12 => LETIMER1_A::DIV4096,
            13 => LETIMER1_A::DIV8192,
            14 => LETIMER1_A::DIV16384,
            15 => LETIMER1_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER1_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER1_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER1_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER1_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER1_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER1_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER1_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER1_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER1_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER1_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER1_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER1_A::DIV32768
    }
}
#[doc = "Write proxy for field `LETIMER1`"]
pub struct LETIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LETIMER1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKLETIMER1 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV1)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV2)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV4)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV8)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV16)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV32)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV64)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV128)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV256)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV512)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV1024)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV2048)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV4096)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV8192)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV16384)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER1_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Low Energy Sensor Interface Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LESENSE_A {
    #[doc = "0: LFACLKLESENSE = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLESENSE = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLESENSE = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLESENSE = LFACLK/8"]
    DIV8 = 3,
}
impl From<LESENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LESENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LESENSE`"]
pub type LESENSE_R = crate::R<u8, LESENSE_A>;
impl LESENSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LESENSE_A {
        match self.bits {
            0 => LESENSE_A::DIV1,
            1 => LESENSE_A::DIV2,
            2 => LESENSE_A::DIV4,
            3 => LESENSE_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LESENSE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LESENSE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LESENSE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LESENSE_A::DIV8
    }
}
#[doc = "Write proxy for field `LESENSE`"]
pub struct LESENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LESENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LESENSE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKLESENSE = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV1)
    }
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV2)
    }
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV4)
    }
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Liquid Crystal Display Controller Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCD_A {
    #[doc = "0: LFACLKLCD = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLCD = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLCD = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLCD = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKLCD = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKLCD = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKLCD = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKLCD = LFACLK/128"]
    DIV128 = 7,
}
impl From<LCD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCD`"]
pub type LCD_R = crate::R<u8, LCD_A>;
impl LCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_A {
        match self.bits {
            0 => LCD_A::DIV1,
            1 => LCD_A::DIV2,
            2 => LCD_A::DIV4,
            3 => LCD_A::DIV8,
            4 => LCD_A::DIV16,
            5 => LCD_A::DIV32,
            6 => LCD_A::DIV64,
            7 => LCD_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LCD_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LCD_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LCD_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LCD_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LCD_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LCD_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LCD_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LCD_A::DIV128
    }
}
#[doc = "Write proxy for field `LCD`"]
pub struct LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKLCD = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LCD_A::DIV1)
    }
    #[doc = "LFACLKLCD = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LCD_A::DIV2)
    }
    #[doc = "LFACLKLCD = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LCD_A::DIV4)
    }
    #[doc = "LFACLKLCD = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LCD_A::DIV8)
    }
    #[doc = "LFACLKLCD = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LCD_A::DIV16)
    }
    #[doc = "LFACLKLCD = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LCD_A::DIV32)
    }
    #[doc = "LFACLKLCD = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LCD_A::DIV64)
    }
    #[doc = "LFACLKLCD = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LCD_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Real-Time Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: LFACLKRTC = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKRTC = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKRTC = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKRTC = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKRTC = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKRTC = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKRTC = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKRTC = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKRTC = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKRTC = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKRTC = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKRTC = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKRTC = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKRTC = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKRTC = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKRTC = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::DIV1,
            1 => RTC_A::DIV2,
            2 => RTC_A::DIV4,
            3 => RTC_A::DIV8,
            4 => RTC_A::DIV16,
            5 => RTC_A::DIV32,
            6 => RTC_A::DIV64,
            7 => RTC_A::DIV128,
            8 => RTC_A::DIV256,
            9 => RTC_A::DIV512,
            10 => RTC_A::DIV1024,
            11 => RTC_A::DIV2048,
            12 => RTC_A::DIV4096,
            13 => RTC_A::DIV8192,
            14 => RTC_A::DIV16384,
            15 => RTC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RTC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RTC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == RTC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == RTC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == RTC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == RTC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == RTC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == RTC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == RTC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == RTC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == RTC_A::DIV32768
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTC_A::DIV1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTC_A::DIV2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTC_A::DIV4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTC_A::DIV8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTC_A::DIV16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RTC_A::DIV32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RTC_A::DIV64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(RTC_A::DIV128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(RTC_A::DIV256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(RTC_A::DIV512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(RTC_A::DIV1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(RTC_A::DIV2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(RTC_A::DIV4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(RTC_A::DIV8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(RTC_A::DIV16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(RTC_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low Energy Timer 1 Prescaler"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W { w: self }
    }
    #[doc = "Bits 4:7 - Low Energy Timer 1 Prescaler"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> LETIMER1_W {
        LETIMER1_W { w: self }
    }
    #[doc = "Bits 8:9 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W { w: self }
    }
    #[doc = "Bits 12:14 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W { w: self }
    }
    #[doc = "Bits 16:19 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
