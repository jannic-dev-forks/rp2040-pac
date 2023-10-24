#[doc = "Register `NVIC_ISER` reader"]
pub type R = crate::R<NVIC_ISER_SPEC>;
#[doc = "Register `NVIC_ISER` writer"]
pub type W = crate::W<NVIC_ISER_SPEC>;
#[doc = "Field `SETENA` reader - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub type SETENA_R = crate::FieldReader<u32>;
#[doc = "Field `SETENA` writer - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub type SETENA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-enable bits.  
 Write:  
 0 = No effect.  
 1 = Enable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn setena(&mut self) -> SETENA_W<NVIC_ISER_SPEC, 0> {
        SETENA_W::new(self)
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
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled.  
 If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISER_SPEC;
impl crate::RegisterSpec for NVIC_ISER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser::R`](R) reader structure"]
impl crate::Readable for NVIC_ISER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser::W`](W) writer structure"]
impl crate::Writable for NVIC_ISER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISER to value 0"]
impl crate::Resettable for NVIC_ISER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
