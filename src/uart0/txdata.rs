#[doc = "Reader of register TXDATA"]
pub type R = crate::R<u32, super::TXDATA>;
#[doc = "Writer for register TXDATA"]
pub type W = crate::W<u32, super::TXDATA>;
#[doc = "Register TXDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
}
