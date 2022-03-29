#[doc = "Writer for register LINKLOAD"]
pub type W = crate::W<u32, super::LINKLOAD>;
#[doc = "Register LINKLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::LINKLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LINKLOAD`"]
pub struct LINKLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LINKLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA Link Loads"]
    #[inline(always)]
    pub fn linkload(&mut self) -> LINKLOAD_W {
        LINKLOAD_W { w: self }
    }
}
