#[doc = "Reader of register DATA3"]
pub type R = crate::R<u32, super::DATA3>;
#[doc = "Writer for register DATA3"]
pub type W = crate::W<u32, super::DATA3>;
#[doc = "Register DATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA3`"]
pub type DATA3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA3`"]
pub struct DATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W { w: self }
    }
}
