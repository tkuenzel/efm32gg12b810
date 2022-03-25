#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `DV`"]
pub type DV_R = crate::R<bool, bool>;
#[doc = "Reader of field `DVL`"]
pub type DVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UF`"]
pub type UF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Undeflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
