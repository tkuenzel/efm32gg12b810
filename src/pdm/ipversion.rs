#[doc = "Reader of register IPVERSION"]
pub type R = crate::R<u32, super::IPVERSION>;
#[doc = "Reader of field `IPVERSION`"]
pub type IPVERSION_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP VERSION"]
    #[inline(always)]
    pub fn ipversion(&self) -> IPVERSION_R {
        IPVERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
