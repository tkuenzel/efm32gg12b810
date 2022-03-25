#[doc = "Reader of register PDMCTRL"]
pub type R = crate::R<u32, super::PDMCTRL>;
#[doc = "Writer for register PDMCTRL"]
pub type W = crate::W<u32, super::PDMCTRL>;
#[doc = "Register PDMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDM Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDMCLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock PDM"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock PDM"]
    HFXO = 1,
    #[doc = "2: USHFRCO is used to clock PDM"]
    USHFRCO = 2,
    #[doc = "3: CLKIN0 is selected as HFCLK clock source"]
    CLKIN0 = 3,
}
impl From<PDMCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PDMCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PDMCLKSEL`"]
pub type PDMCLKSEL_R = crate::R<u8, PDMCLKSEL_A>;
impl PDMCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCLKSEL_A {
        match self.bits {
            0 => PDMCLKSEL_A::HFRCO,
            1 => PDMCLKSEL_A::HFXO,
            2 => PDMCLKSEL_A::USHFRCO,
            3 => PDMCLKSEL_A::CLKIN0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == PDMCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == PDMCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == PDMCLKSEL_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == PDMCLKSEL_A::CLKIN0
    }
}
#[doc = "Write proxy for field `PDMCLKSEL`"]
pub struct PDMCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMCLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFRCO clock is used to clock PDM"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock PDM"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::HFXO)
    }
    #[doc = "USHFRCO is used to clock PDM"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::USHFRCO)
    }
    #[doc = "CLKIN0 is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::CLKIN0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PDMCLKEN`"]
pub type PDMCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMCLKEN`"]
pub struct PDMCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCLKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PDM Core Clock Select"]
    #[inline(always)]
    pub fn pdmclksel(&self) -> PDMCLKSEL_R {
        PDMCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - PDM Core Clock Enable"]
    #[inline(always)]
    pub fn pdmclken(&self) -> PDMCLKEN_R {
        PDMCLKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDM Core Clock Select"]
    #[inline(always)]
    pub fn pdmclksel(&mut self) -> PDMCLKSEL_W {
        PDMCLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - PDM Core Clock Enable"]
    #[inline(always)]
    pub fn pdmclken(&mut self) -> PDMCLKEN_W {
        PDMCLKEN_W { w: self }
    }
}
