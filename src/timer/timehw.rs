#[doc = "Register `TIMEHW` writer"]
pub type W = crate::W<TIMEHW_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TIMEHW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timehw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHW_SPEC;
impl crate::RegisterSpec for TIMEHW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timehw::W`](W) writer structure"]
impl crate::Writable for TIMEHW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEHW to value 0"]
impl crate::Resettable for TIMEHW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
