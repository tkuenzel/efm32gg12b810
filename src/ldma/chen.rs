#[doc = "Reader of register CHEN"]
pub type R = crate::R<u32, super::CHEN>;
#[doc = "Writer for register CHEN"]
pub type W = crate::W<u32, super::CHEN>;
#[doc = "Register CHEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Channel Enables"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel Enables"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
}
