#[doc = "Register `SCRATCH2` reader"]
pub type R = crate::R<SCRATCH2_SPEC>;
#[doc = "Register `SCRATCH2` writer"]
pub type W = crate::W<SCRATCH2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCRATCH2_SPEC> {
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

You can [`read`](crate::generic::Reg::read) this register and get [`scratch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRATCH2_SPEC;
impl crate::RegisterSpec for SCRATCH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratch2::R`](R) reader structure"]
impl crate::Readable for SCRATCH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scratch2::W`](W) writer structure"]
impl crate::Writable for SCRATCH2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRATCH2 to value 0"]
impl crate::Resettable for SCRATCH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
