#[doc = "Reader of register OPA1_CTRL"]
pub type R = crate::R<u32, super::OPA1_CTRL>;
#[doc = "Writer for register OPA1_CTRL"]
pub type W = crate::W<u32, super::OPA1_CTRL>;
#[doc = "Register OPA1_CTRL `reset()`'s with value 0x0e"]
impl crate::ResetValue for super::OPA1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e
    }
}
#[doc = "OPAx Operation Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVESTRENGTH_A {
    #[doc = "0: Lower accuracy with Low drive strength."]
    _0 = 0,
    #[doc = "1: Low accuracy with Low drive strength."]
    _1 = 1,
    #[doc = "2: High accuracy with High drive strength."]
    _2 = 2,
    #[doc = "3: Higher accuracy with High drive strength."]
    _3 = 3,
}
impl From<DRIVESTRENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVESTRENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVESTRENGTH`"]
pub type DRIVESTRENGTH_R = crate::R<u8, DRIVESTRENGTH_A>;
impl DRIVESTRENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVESTRENGTH_A {
        match self.bits {
            0 => DRIVESTRENGTH_A::_0,
            1 => DRIVESTRENGTH_A::_1,
            2 => DRIVESTRENGTH_A::_2,
            3 => DRIVESTRENGTH_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRIVESTRENGTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRIVESTRENGTH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DRIVESTRENGTH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == DRIVESTRENGTH_A::_3
    }
}
#[doc = "Write proxy for field `DRIVESTRENGTH`"]
pub struct DRIVESTRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVESTRENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVESTRENGTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_0)
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_1)
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_2)
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INCBW`"]
pub type INCBW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCBW`"]
pub struct INCBW_W<'a> {
    w: &'a mut W,
}
impl<'a> INCBW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `HCMDIS`"]
pub type HCMDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCMDIS`"]
pub struct HCMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCMDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OUTSCALE`"]
pub type OUTSCALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTSCALE`"]
pub struct OUTSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRSEN`"]
pub type PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSEN`"]
pub struct PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRSMODE`"]
pub type PRSMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSMODE`"]
pub struct PRSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "OPAx PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers OPA."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers OPA."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers OPA."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers OPA."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers OPA."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers OPA."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers OPA."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers OPA."]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers OPA."]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers OPA."]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers OPA."]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers OPA."]
    PRSCH11 = 11,
    #[doc = "12: PRS ch 12 triggers OPA."]
    PRSCH12 = 12,
    #[doc = "13: PRS ch 13 triggers OPA."]
    PRSCH13 = 13,
    #[doc = "14: PRS ch 14 triggers OPA."]
    PRSCH14 = 14,
    #[doc = "15: PRS ch 15 triggers OPA."]
    PRSCH15 = 15,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRSSEL`"]
pub type PRSSEL_R = crate::R<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL_A {
        match self.bits {
            0 => PRSSEL_A::PRSCH0,
            1 => PRSSEL_A::PRSCH1,
            2 => PRSSEL_A::PRSCH2,
            3 => PRSSEL_A::PRSCH3,
            4 => PRSSEL_A::PRSCH4,
            5 => PRSSEL_A::PRSCH5,
            6 => PRSSEL_A::PRSCH6,
            7 => PRSSEL_A::PRSCH7,
            8 => PRSSEL_A::PRSCH8,
            9 => PRSSEL_A::PRSCH9,
            10 => PRSSEL_A::PRSCH10,
            11 => PRSSEL_A::PRSCH11,
            12 => PRSSEL_A::PRSCH12,
            13 => PRSSEL_A::PRSCH13,
            14 => PRSSEL_A::PRSCH14,
            15 => PRSSEL_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
}
#[doc = "Write proxy for field `PRSSEL`"]
pub struct PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRSOUTMODE`"]
pub type PRSOUTMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSOUTMODE`"]
pub struct PRSOUTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSOUTMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `APORTXMASTERDIS`"]
pub type APORTXMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTXMASTERDIS`"]
pub struct APORTXMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTXMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `APORTYMASTERDIS`"]
pub type APORTYMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTYMASTERDIS`"]
pub struct APORTYMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTYMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    pub fn incbw(&self) -> INCBW_R {
        INCBW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    pub fn hcmdis(&self) -> HCMDIS_R {
        HCMDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    pub fn outscale(&self) -> OUTSCALE_R {
        OUTSCALE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - OPAx PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    pub fn prsoutmode(&self) -> PRSOUTMODE_R {
        PRSOUTMODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDIS_R {
        APORTXMASTERDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> APORTYMASTERDIS_R {
        APORTYMASTERDIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W {
        DRIVESTRENGTH_W { w: self }
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    pub fn incbw(&mut self) -> INCBW_W {
        INCBW_W { w: self }
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    pub fn hcmdis(&mut self) -> HCMDIS_W {
        HCMDIS_W { w: self }
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    pub fn outscale(&mut self) -> OUTSCALE_W {
        OUTSCALE_W { w: self }
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PRSEN_W {
        PRSEN_W { w: self }
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&mut self) -> PRSMODE_W {
        PRSMODE_W { w: self }
    }
    #[doc = "Bits 10:13 - OPAx PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    pub fn prsoutmode(&mut self) -> PRSOUTMODE_W {
        PRSOUTMODE_W { w: self }
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&mut self) -> APORTXMASTERDIS_W {
        APORTXMASTERDIS_W { w: self }
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&mut self) -> APORTYMASTERDIS_W {
        APORTYMASTERDIS_W { w: self }
    }
}
