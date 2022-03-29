#[doc = "Reader of register INPUT"]
pub type R = crate::R<u32, super::INPUT>;
#[doc = "Writer for register INPUT"]
pub type W = crate::W<u32, super::INPUT>;
#[doc = "Register INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S0PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    PRSCH15 = 15,
}
impl From<S0PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S0PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S0PRSSEL`"]
pub type S0PRSSEL_R = crate::R<u8, S0PRSSEL_A>;
impl S0PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0PRSSEL_A {
        match self.bits {
            0 => S0PRSSEL_A::PRSCH0,
            1 => S0PRSSEL_A::PRSCH1,
            2 => S0PRSSEL_A::PRSCH2,
            3 => S0PRSSEL_A::PRSCH3,
            4 => S0PRSSEL_A::PRSCH4,
            5 => S0PRSSEL_A::PRSCH5,
            6 => S0PRSSEL_A::PRSCH6,
            7 => S0PRSSEL_A::PRSCH7,
            8 => S0PRSSEL_A::PRSCH8,
            9 => S0PRSSEL_A::PRSCH9,
            10 => S0PRSSEL_A::PRSCH10,
            11 => S0PRSSEL_A::PRSCH11,
            12 => S0PRSSEL_A::PRSCH12,
            13 => S0PRSSEL_A::PRSCH13,
            14 => S0PRSSEL_A::PRSCH14,
            15 => S0PRSSEL_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH15
    }
}
#[doc = "Write proxy for field `S0PRSSEL`"]
pub struct S0PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0PRSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `S0PRSEN`"]
pub type S0PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0PRSEN`"]
pub struct S0PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S1PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    PRSCH15 = 15,
}
impl From<S1PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S1PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S1PRSSEL`"]
pub type S1PRSSEL_R = crate::R<u8, S1PRSSEL_A>;
impl S1PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1PRSSEL_A {
        match self.bits {
            0 => S1PRSSEL_A::PRSCH0,
            1 => S1PRSSEL_A::PRSCH1,
            2 => S1PRSSEL_A::PRSCH2,
            3 => S1PRSSEL_A::PRSCH3,
            4 => S1PRSSEL_A::PRSCH4,
            5 => S1PRSSEL_A::PRSCH5,
            6 => S1PRSSEL_A::PRSCH6,
            7 => S1PRSSEL_A::PRSCH7,
            8 => S1PRSSEL_A::PRSCH8,
            9 => S1PRSSEL_A::PRSCH9,
            10 => S1PRSSEL_A::PRSCH10,
            11 => S1PRSSEL_A::PRSCH11,
            12 => S1PRSSEL_A::PRSCH12,
            13 => S1PRSSEL_A::PRSCH13,
            14 => S1PRSSEL_A::PRSCH14,
            15 => S1PRSSEL_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH15
    }
}
#[doc = "Write proxy for field `S1PRSSEL`"]
pub struct S1PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1PRSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `S1PRSEN`"]
pub type S1PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1PRSEN`"]
pub struct S1PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0PRSSEL_R {
        S0PRSSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0PRSEN_R {
        S0PRSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1PRSSEL_R {
        S1PRSSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1PRSEN_R {
        S1PRSEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&mut self) -> S0PRSSEL_W {
        S0PRSSEL_W { w: self }
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&mut self) -> S0PRSEN_W {
        S0PRSEN_W { w: self }
    }
    #[doc = "Bits 6:9 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&mut self) -> S1PRSSEL_W {
        S1PRSSEL_W { w: self }
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&mut self) -> S1PRSEN_W {
        S1PRSEN_W { w: self }
    }
}
