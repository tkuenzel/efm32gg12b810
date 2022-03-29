#[doc = "Reader of register DIEP0CTL"]
pub type R = crate::R<u32, super::DIEP0CTL>;
#[doc = "Writer for register DIEP0CTL"]
pub type W = crate::W<u32, super::DIEP0CTL>;
#[doc = "Register DIEP0CTL `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::DIEP0CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPS_A {
    #[doc = "0: 64 bytes."]
    _64B = 0,
    #[doc = "1: 32 bytes."]
    _32B = 1,
    #[doc = "2: 16 bytes."]
    _16B = 2,
    #[doc = "3: 8 bytes."]
    _8B = 3,
}
impl From<MPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPS`"]
pub type MPS_R = crate::R<u8, MPS_A>;
impl MPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPS_A {
        match self.bits {
            0 => MPS_A::_64B,
            1 => MPS_A::_32B,
            2 => MPS_A::_16B,
            3 => MPS_A::_8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64B`"]
    #[inline(always)]
    pub fn is_64b(&self) -> bool {
        *self == MPS_A::_64B
    }
    #[doc = "Checks if the value of the field is `_32B`"]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == MPS_A::_32B
    }
    #[doc = "Checks if the value of the field is `_16B`"]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == MPS_A::_16B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == MPS_A::_8B
    }
}
#[doc = "Write proxy for field `MPS`"]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn _64b(self) -> &'a mut W {
        self.variant(MPS_A::_64B)
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut W {
        self.variant(MPS_A::_32B)
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut W {
        self.variant(MPS_A::_16B)
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(MPS_A::_8B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `USBACTEP`"]
pub type USBACTEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKSTS`"]
pub type NAKSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTYPE`"]
pub type EPTYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Reader of field `TXFNUM`"]
pub type TXFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFNUM`"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `CNAK`"]
pub struct CNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `SNAK`"]
pub struct SNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `EPDIS`"]
pub type EPDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDIS`"]
pub struct EPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `EPENA`"]
pub type EPENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPENA`"]
pub struct EPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W { w: self }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W {
        EPENA_W { w: self }
    }
}
