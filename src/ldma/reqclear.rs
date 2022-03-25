#[doc = "Writer for register REQCLEAR"]
pub type W = crate::W<u32, super::REQCLEAR>;
#[doc = "Register REQCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::REQCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `REQCLEAR`"]
pub struct REQCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REQCLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA Request Clear"]
    #[inline(always)]
    pub fn reqclear(&mut self) -> REQCLEAR_W {
        REQCLEAR_W { w: self }
    }
}
