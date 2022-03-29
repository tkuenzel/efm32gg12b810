#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPUPRIV`"]
pub type PPUPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPUPRIV`"]
pub struct PPUPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PPUPRIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PPUPRIV Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PPUPRIV_R {
        PPUPRIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPUPRIV Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&mut self) -> PPUPRIV_W {
        PPUPRIV_W { w: self }
    }
}
