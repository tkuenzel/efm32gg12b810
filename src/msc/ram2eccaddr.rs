#[doc = "Reader of register RAM2ECCADDR"]
pub type R = crate::R<u32, super::RAM2ECCADDR>;
#[doc = "Reader of field `RAM2ECCADDR`"]
pub type RAM2ECCADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM2 ECC Error Address"]
    #[inline(always)]
    pub fn ram2eccaddr(&self) -> RAM2ECCADDR_R {
        RAM2ECCADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
