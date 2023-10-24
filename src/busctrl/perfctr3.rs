#[doc = "Register `PERFCTR3` reader"]
pub type R = crate::R<PERFCTR3_SPEC>;
#[doc = "Register `PERFCTR3` writer"]
pub type W = crate::W<PERFCTR3_SPEC>;
#[doc = "Field `PERFCTR3` reader - Busfabric saturating performance counter 3  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL3"]
pub type PERFCTR3_R = crate::FieldReader<u32>;
#[doc = "Field `PERFCTR3` writer - Busfabric saturating performance counter 3  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL3"]
pub type PERFCTR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    pub fn perfctr3(&self) -> PERFCTR3_R {
        PERFCTR3_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Busfabric saturating performance counter 3  
 Count some event signal from the busfabric arbiters.  
 Write any value to clear. Select an event to count using PERFSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn perfctr3(&mut self) -> PERFCTR3_W<PERFCTR3_SPEC, 0> {
        PERFCTR3_W::new(self)
    }
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
#[doc = "Bus fabric performance counter 3  

You can [`read`](crate::generic::Reg::read) this register and get [`perfctr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfctr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFCTR3_SPEC;
impl crate::RegisterSpec for PERFCTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfctr3::R`](R) reader structure"]
impl crate::Readable for PERFCTR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfctr3::W`](W) writer structure"]
impl crate::Writable for PERFCTR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PERFCTR3 to value 0"]
impl crate::Resettable for PERFCTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
