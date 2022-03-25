#[doc = "Reader of register HPRT"]
pub type R = crate::R<u32, super::HPRT>;
#[doc = "Writer for register HPRT"]
pub type W = crate::W<u32, super::HPRT>;
#[doc = "Register HPRT `reset()`'s with value 0"]
impl crate::ResetValue for super::HPRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRTCONNSTS`"]
pub type PRTCONNSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRTCONNDET`"]
pub type PRTCONNDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTCONNDET`"]
pub struct PRTCONNDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCONNDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PRTENA`"]
pub type PRTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTENA`"]
pub struct PRTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENA_W<'a> {
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
#[doc = "Reader of field `PRTENCHNG`"]
pub type PRTENCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTENCHNG`"]
pub struct PRTENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENCHNG_W<'a> {
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
#[doc = "Reader of field `PRTOVRCURRACT`"]
pub type PRTOVRCURRACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRTOVRCURRCHNG`"]
pub type PRTOVRCURRCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTOVRCURRCHNG`"]
pub struct PRTOVRCURRCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOVRCURRCHNG_W<'a> {
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
#[doc = "Reader of field `PRTRES`"]
pub type PRTRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTRES`"]
pub struct PRTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRTSUSP`"]
pub type PRTSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTSUSP`"]
pub struct PRTSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTSUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PRTRST`"]
pub type PRTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTRST`"]
pub struct PRTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRST_W<'a> {
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
#[doc = "Reader of field `PRTLNSTS`"]
pub type PRTLNSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRTPWR`"]
pub type PRTPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTPWR`"]
pub struct PRTPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTTSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK = 3,
    #[doc = "4: Test_Packet mode."]
    PACKET = 4,
    #[doc = "5: Test_Force_Enable."]
    FORCE = 5,
}
impl From<PRTTSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTTSTCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRTTSTCTL`"]
pub type PRTTSTCTL_R = crate::R<u8, PRTTSTCTL_A>;
impl PRTTSTCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRTTSTCTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRTTSTCTL_A::DISABLE),
            1 => Val(PRTTSTCTL_A::J),
            2 => Val(PRTTSTCTL_A::K),
            3 => Val(PRTTSTCTL_A::SE0NAK),
            4 => Val(PRTTSTCTL_A::PACKET),
            5 => Val(PRTTSTCTL_A::FORCE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRTTSTCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == PRTTSTCTL_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == PRTTSTCTL_A::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == PRTTSTCTL_A::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == PRTTSTCTL_A::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == PRTTSTCTL_A::FORCE
    }
}
#[doc = "Write proxy for field `PRTTSTCTL`"]
pub struct PRTTSTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTTSTCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTTSTCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTSPD_A {
    #[doc = "1: Full speed."]
    FS = 1,
    #[doc = "2: Low speed."]
    LS = 2,
}
impl From<PRTSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRTSPD`"]
pub type PRTSPD_R = crate::R<u8, PRTSPD_A>;
impl PRTSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRTSPD_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PRTSPD_A::FS),
            2 => Val(PRTSPD_A::LS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == PRTSPD_A::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == PRTSPD_A::LS
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W {
        PRTCONNDET_W { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&mut self) -> PRTENA_W {
        PRTENA_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W {
        PRTENCHNG_W { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W {
        PRTOVRCURRCHNG_W { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&mut self) -> PRTRES_W {
        PRTRES_W { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&mut self) -> PRTSUSP_W {
        PRTSUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&mut self) -> PRTRST_W {
        PRTRST_W { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&mut self) -> PRTPWR_W {
        PRTPWR_W { w: self }
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W {
        PRTTSTCTL_W { w: self }
    }
}
