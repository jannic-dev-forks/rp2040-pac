#[doc = "Register `DBG_PADOE` reader"]
pub type R = crate::R<DBG_PADOE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DBG_PADOE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_padoe::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PADOE_SPEC;
impl crate::RegisterSpec for DBG_PADOE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_padoe::R`](R) reader structure"]
impl crate::Readable for DBG_PADOE_SPEC {}
#[doc = "`reset()` method sets DBG_PADOE to value 0"]
impl crate::Resettable for DBG_PADOE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
