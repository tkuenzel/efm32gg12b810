#[doc = "Reader of register ROUTELOC0"]
pub type R = crate::R<u32, super::ROUTELOC0>;
#[doc = "Writer for register ROUTELOC0"]
pub type W = crate::W<u32, super::ROUTELOC0>;
#[doc = "Register ROUTELOC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSPILOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<QSPILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPILOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QSPILOC`"]
pub type QSPILOC_R = crate::R<u8, QSPILOC_A>;
impl QSPILOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, QSPILOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(QSPILOC_A::LOC0),
            1 => Val(QSPILOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == QSPILOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == QSPILOC_A::LOC1
    }
}
#[doc = "Write proxy for field `QSPILOC`"]
pub struct QSPILOC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPILOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(QSPILOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(QSPILOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSPIRSTLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<QSPIRSTLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPIRSTLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QSPIRSTLOC`"]
pub type QSPIRSTLOC_R = crate::R<u8, QSPIRSTLOC_A>;
impl QSPIRSTLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, QSPIRSTLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(QSPIRSTLOC_A::LOC0),
            1 => Val(QSPIRSTLOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == QSPIRSTLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == QSPIRSTLOC_A::LOC1
    }
}
#[doc = "Write proxy for field `QSPIRSTLOC`"]
pub struct QSPIRSTLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRSTLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPIRSTLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(QSPIRSTLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(QSPIRSTLOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn qspiloc(&self) -> QSPILOC_R {
        QSPILOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - I/O Location"]
    #[inline(always)]
    pub fn qspirstloc(&self) -> QSPIRSTLOC_R {
        QSPIRSTLOC_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn qspiloc(&mut self) -> QSPILOC_W {
        QSPILOC_W { w: self }
    }
    #[doc = "Bits 6:11 - I/O Location"]
    #[inline(always)]
    pub fn qspirstloc(&mut self) -> QSPIRSTLOC_W {
        QSPIRSTLOC_W { w: self }
    }
}
