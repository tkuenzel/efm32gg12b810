#[doc = "Reader of register INDIRECTWRITEXFERSTART"]
pub type R = crate::R<u32, super::INDIRECTWRITEXFERSTART>;
#[doc = "Writer for register INDIRECTWRITEXFERSTART"]
pub type W = crate::W<u32, super::INDIRECTWRITEXFERSTART>;
#[doc = "Register INDIRECTWRITEXFERSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::INDIRECTWRITEXFERSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
