#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT0PEN`"]
pub type DAT0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT0PEN`"]
pub struct DAT0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT0PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DAT1PEN`"]
pub type DAT1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT1PEN`"]
pub struct DAT1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT1PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DAT2PEN`"]
pub type DAT2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT2PEN`"]
pub struct DAT2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT2PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DAT3PEN`"]
pub type DAT3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT3PEN`"]
pub struct DAT3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT3PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CLKPEN`"]
pub type CLKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPEN`"]
pub struct CLKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DAT0 I/O Enable"]
    #[inline(always)]
    pub fn dat0pen(&self) -> DAT0PEN_R {
        DAT0PEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAT1 I/O Enable"]
    #[inline(always)]
    pub fn dat1pen(&self) -> DAT1PEN_R {
        DAT1PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT2 I/O Enable"]
    #[inline(always)]
    pub fn dat2pen(&self) -> DAT2PEN_R {
        DAT2PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAT3 I/O Enable"]
    #[inline(always)]
    pub fn dat3pen(&self) -> DAT3PEN_R {
        DAT3PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAT0 I/O Enable"]
    #[inline(always)]
    pub fn dat0pen(&mut self) -> DAT0PEN_W {
        DAT0PEN_W { w: self }
    }
    #[doc = "Bit 1 - DAT1 I/O Enable"]
    #[inline(always)]
    pub fn dat1pen(&mut self) -> DAT1PEN_W {
        DAT1PEN_W { w: self }
    }
    #[doc = "Bit 2 - DAT2 I/O Enable"]
    #[inline(always)]
    pub fn dat2pen(&mut self) -> DAT2PEN_W {
        DAT2PEN_W { w: self }
    }
    #[doc = "Bit 3 - DAT3 I/O Enable"]
    #[inline(always)]
    pub fn dat3pen(&mut self) -> DAT3PEN_W {
        DAT3PEN_W { w: self }
    }
    #[doc = "Bit 8 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W { w: self }
    }
}
