#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSR`"]
pub struct DSR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Reader of field `OUTCLKEN`"]
pub type OUTCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTCLKEN`"]
pub struct OUTCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - PDM Clock enable"]
    #[inline(always)]
    pub fn outclken(&self) -> OUTCLKEN_R {
        OUTCLKEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&mut self) -> DSR_W {
        DSR_W { w: self }
    }
    #[doc = "Bit 31 - PDM Clock enable"]
    #[inline(always)]
    pub fn outclken(&mut self) -> OUTCLKEN_W {
        OUTCLKEN_W { w: self }
    }
}
