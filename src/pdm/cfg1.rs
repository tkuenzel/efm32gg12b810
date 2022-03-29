#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
}
