#[doc = "Reader of register REQDIS"]
pub type R = crate::R<u32, super::REQDIS>;
#[doc = "Writer for register REQDIS"]
pub type W = crate::W<u32, super::REQDIS>;
#[doc = "Register REQDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::REQDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REQDIS`"]
pub type REQDIS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REQDIS`"]
pub struct REQDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> REQDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&self) -> REQDIS_R {
        REQDIS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&mut self) -> REQDIS_W {
        REQDIS_W { w: self }
    }
}
