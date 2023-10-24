#[doc = "Register `SCRATCH6` reader"]
pub type R = crate::R<SCRATCH6_SPEC>;
#[doc = "Register `SCRATCH6` writer"]
pub type W = crate::W<SCRATCH6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCRATCH6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "Scratch register. Information persists through soft reset of the chip.  

You can [`read`](crate::generic::Reg::read) this register and get [`scratch6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH6_SPEC;
impl crate::RegisterSpec for SCRATCH6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch6::R`](R) reader structure"]
impl crate::Readable for SCRATCH6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch6::W`](W) writer structure"]
impl crate::Writable for SCRATCH6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRATCH6 to value 0"]
impl crate::Resettable for SCRATCH6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
