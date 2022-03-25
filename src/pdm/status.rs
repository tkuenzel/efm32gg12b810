#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOCNT`"]
pub type FIFOCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - PDM is active"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - FIFO FULL Status"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FIFO EMPTY Status"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - FIFO CNT"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
