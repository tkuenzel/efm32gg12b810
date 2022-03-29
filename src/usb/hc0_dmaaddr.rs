#[doc = "Reader of register HC0_DMAADDR"]
pub type R = crate::R<u32, super::HC0_DMAADDR>;
#[doc = "Writer for register HC0_DMAADDR"]
pub type W = crate::W<u32, super::HC0_DMAADDR>;
#[doc = "Register HC0_DMAADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::HC0_DMAADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAADDR`"]
pub type DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMAADDR`"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
}
