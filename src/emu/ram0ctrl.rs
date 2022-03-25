#[doc = "Reader of register RAM0CTRL"]
pub type R = crate::R<u32, super::RAM0CTRL>;
#[doc = "Writer for register RAM0CTRL"]
pub type W = crate::W<u32, super::RAM0CTRL>;
#[doc = "Register RAM0CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM0CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "4: Power down RAM block 3"]
    BLK3 = 4,
    #[doc = "6: Power down RAM blocks 2 and above"]
    BLK2TO3 = 6,
    #[doc = "7: Power down RAM blocks 1 and above"]
    BLK1TO3 = 7,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMPOWERDOWN`"]
pub type RAMPOWERDOWN_R = crate::R<u8, RAMPOWERDOWN_A>;
impl RAMPOWERDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMPOWERDOWN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAMPOWERDOWN_A::NONE),
            4 => Val(RAMPOWERDOWN_A::BLK3),
            6 => Val(RAMPOWERDOWN_A::BLK2TO3),
            7 => Val(RAMPOWERDOWN_A::BLK1TO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK3`"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3
    }
    #[doc = "Checks if the value of the field is `BLK2TO3`"]
    #[inline(always)]
    pub fn is_blk2to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO3
    }
    #[doc = "Checks if the value of the field is `BLK1TO3`"]
    #[inline(always)]
    pub fn is_blk1to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO3
    }
}
#[doc = "Write proxy for field `RAMPOWERDOWN`"]
pub struct RAMPOWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPOWERDOWN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO3)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W { w: self }
    }
}
