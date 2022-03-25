#[doc = "Reader of register CORECLKCONTROL"]
pub type R = crate::R<u32, super::CORECLKCONTROL>;
#[doc = "Writer for register CORECLKCONTROL"]
pub type W = crate::W<u32, super::CORECLKCONTROL>;
#[doc = "Register CORECLKCONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CORECLKCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CORECLKDIS`"]
pub type CORECLKDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CORECLKDIS`"]
pub struct CORECLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORECLKDIS_W<'a> {
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
#[doc = "Reader of field `CORECLKPRESC`"]
pub type CORECLKPRESC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CORECLKPRESC`"]
pub struct CORECLKPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORECLKPRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Core Clock Disable"]
    #[inline(always)]
    pub fn coreclkdis(&self) -> CORECLKDIS_R {
        CORECLKDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Clock division factor of CORECLKPRESC+1"]
    #[inline(always)]
    pub fn coreclkpresc(&self) -> CORECLKPRESC_R {
        CORECLKPRESC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Core Clock Disable"]
    #[inline(always)]
    pub fn coreclkdis(&mut self) -> CORECLKDIS_W {
        CORECLKDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - Clock division factor of CORECLKPRESC+1"]
    #[inline(always)]
    pub fn coreclkpresc(&mut self) -> CORECLKPRESC_W {
        CORECLKPRESC_W { w: self }
    }
}
