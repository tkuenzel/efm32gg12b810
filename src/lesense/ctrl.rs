#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configure Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCANMODE_A {
    #[doc = "0: A new scan is started each time the period counter overflows"]
    PERIODIC = 0,
    #[doc = "1: A single scan is performed when START in CMD is set"]
    ONESHOT = 1,
    #[doc = "2: Pulse on PRS channel"]
    PRS = 2,
}
impl From<SCANMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCANMODE`"]
pub type SCANMODE_R = crate::R<u8, SCANMODE_A>;
impl SCANMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCANMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCANMODE_A::PERIODIC),
            1 => Val(SCANMODE_A::ONESHOT),
            2 => Val(SCANMODE_A::PRS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == SCANMODE_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == SCANMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SCANMODE_A::PRS
    }
}
#[doc = "Write proxy for field `SCANMODE`"]
pub struct SCANMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCANMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(SCANMODE_A::PERIODIC)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(SCANMODE_A::ONESHOT)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(SCANMODE_A::PRS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Scan Start PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Select Scan Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCANCONF_A {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP = 0,
    #[doc = "1: The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    INVMAP = 1,
    #[doc = "2: The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    TOGGLE = 2,
    #[doc = "3: The decoder state defines the CONF registers to be used."]
    DECDEF = 3,
}
impl From<SCANCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCANCONF`"]
pub type SCANCONF_R = crate::R<u8, SCANCONF_A>;
impl SCANCONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANCONF_A {
        match self.bits {
            0 => SCANCONF_A::DIRMAP,
            1 => SCANCONF_A::INVMAP,
            2 => SCANCONF_A::TOGGLE,
            3 => SCANCONF_A::DECDEF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIRMAP`"]
    #[inline(always)]
    pub fn is_dirmap(&self) -> bool {
        *self == SCANCONF_A::DIRMAP
    }
    #[doc = "Checks if the value of the field is `INVMAP`"]
    #[inline(always)]
    pub fn is_invmap(&self) -> bool {
        *self == SCANCONF_A::INVMAP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == SCANCONF_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `DECDEF`"]
    #[inline(always)]
    pub fn is_decdef(&self) -> bool {
        *self == SCANCONF_A::DECDEF
    }
}
#[doc = "Write proxy for field `SCANCONF`"]
pub struct SCANCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCANCONF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::DIRMAP)
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::INVMAP)
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(SCANCONF_A::TOGGLE)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn decdef(self) -> &'a mut W {
        self.variant(SCANCONF_A::DECDEF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `ALTEXMAP`"]
pub type ALTEXMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEXMAP`"]
pub struct ALTEXMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEXMAP_W<'a> {
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
#[doc = "Reader of field `DUALSAMPLE`"]
pub type DUALSAMPLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALSAMPLE`"]
pub struct DUALSAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALSAMPLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BUFOW`"]
pub type BUFOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFOW`"]
pub struct BUFOW_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFOW_W<'a> {
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
#[doc = "Reader of field `STRSCANRES`"]
pub type STRSCANRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRSCANRES`"]
pub struct STRSCANRES_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSCANRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BUFIDL`"]
pub type BUFIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFIDL`"]
pub struct BUFIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFIDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "DMA Wake-up From EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAWU_A {
    #[doc = "0: No DMA wake-up from EM2"]
    DISABLE = 0,
    #[doc = "1: DMA wake-up from EM2 when data is valid in the result buffer"]
    BUFDATAV = 1,
    #[doc = "2: DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    BUFLEVEL = 2,
}
impl From<DMAWU_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAWU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMAWU`"]
pub type DMAWU_R = crate::R<u8, DMAWU_A>;
impl DMAWU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMAWU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAWU_A::DISABLE),
            1 => Val(DMAWU_A::BUFDATAV),
            2 => Val(DMAWU_A::BUFLEVEL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAWU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BUFDATAV`"]
    #[inline(always)]
    pub fn is_bufdatav(&self) -> bool {
        *self == DMAWU_A::BUFDATAV
    }
    #[doc = "Checks if the value of the field is `BUFLEVEL`"]
    #[inline(always)]
    pub fn is_buflevel(&self) -> bool {
        *self == DMAWU_A::BUFLEVEL
    }
}
#[doc = "Write proxy for field `DMAWU`"]
pub struct DMAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAWU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAWU_A::DISABLE)
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline(always)]
    pub fn bufdatav(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFDATAV)
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline(always)]
    pub fn buflevel(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFLEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DEBUGRUN`"]
pub type DEBUGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGRUN`"]
pub struct DEBUGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> SCANMODE_R {
        SCANMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> SCANCONF_R {
        SCANCONF_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&self) -> ALTEXMAP_R {
        ALTEXMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DUALSAMPLE_R {
        DUALSAMPLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&self) -> BUFOW_R {
        BUFOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> STRSCANRES_R {
        STRSCANRES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&self) -> BUFIDL_R {
        BUFIDL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DMAWU_R {
        DMAWU_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&mut self) -> SCANMODE_W {
        SCANMODE_W { w: self }
    }
    #[doc = "Bits 2:5 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&mut self) -> SCANCONF_W {
        SCANCONF_W { w: self }
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&mut self) -> ALTEXMAP_W {
        ALTEXMAP_W { w: self }
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&mut self) -> DUALSAMPLE_W {
        DUALSAMPLE_W { w: self }
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&mut self) -> BUFOW_W {
        BUFOW_W { w: self }
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&mut self) -> STRSCANRES_W {
        STRSCANRES_W { w: self }
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&mut self) -> BUFIDL_W {
        BUFIDL_W { w: self }
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DMAWU_W {
        DMAWU_W { w: self }
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
}
