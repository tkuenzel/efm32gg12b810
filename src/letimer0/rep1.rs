#[doc = "Reader of register REP1"]
pub type R = crate::R<u32, super::REP1>;
#[doc = "Writer for register REP1"]
pub type W = crate::W<u32, super::REP1>;
#[doc = "Register REP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REP1`"]
pub type REP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REP1`"]
pub struct REP1_W<'a> {
    w: &'a mut W,
}
impl<'a> REP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    pub fn rep1(&mut self) -> REP1_W {
        REP1_W { w: self }
    }
}
