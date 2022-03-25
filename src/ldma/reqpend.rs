#[doc = "Reader of register REQPEND"]
pub type R = crate::R<u32, super::REQPEND>;
#[doc = "Reader of field `REQPEND`"]
pub type REQPEND_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new((self.bits & 0x0fff) as u16)
    }
}
