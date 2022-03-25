#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `STARTBUSY`"]
pub type STARTBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPBUSY`"]
pub type STOPBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLEARBUSY`"]
pub type CLEARBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOFLBUSY`"]
pub type FIFOFLBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRESCBUSY`"]
pub type PRESCBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTRLREGBUSY`"]
pub type CTRLREGBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - START sync busy"]
    #[inline(always)]
    pub fn startbusy(&self) -> STARTBUSY_R {
        STARTBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STOP sync busy"]
    #[inline(always)]
    pub fn stopbusy(&self) -> STOPBUSY_R {
        STOPBUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLEAR sync busy"]
    #[inline(always)]
    pub fn clearbusy(&self) -> CLEARBUSY_R {
        CLEARBUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Flush Sync busy"]
    #[inline(always)]
    pub fn fifoflbusy(&self) -> FIFOFLBUSY_R {
        FIFOFLBUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PRESC Sync busy"]
    #[inline(always)]
    pub fn prescbusy(&self) -> PRESCBUSY_R {
        PRESCBUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTRLREGBUSY busy"]
    #[inline(always)]
    pub fn ctrlregbusy(&self) -> CTRLREGBUSY_R {
        CTRLREGBUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
