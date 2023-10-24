#[doc = "Register `NVIC_ICER` reader"]
pub type R = crate::R<NVIC_ICER_SPEC>;
#[doc = "Register `NVIC_ICER` writer"]
pub type W = crate::W<NVIC_ICER_SPEC>;
#[doc = "Field `CLRENA` reader - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub type CLRENA_R = crate::FieldReader<u32>;
#[doc = "Field `CLRENA` writer - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
pub type CLRENA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits.  
 Write:  
 0 = No effect.  
 1 = Disable interrupt.  
 Read:  
 0 = Interrupt disabled.  
 1 = Interrupt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn clrena(&mut self) -> CLRENA_W<NVIC_ICER_SPEC, 0> {
        CLRENA_W::new(self)
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
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ICER_SPEC;
impl crate::RegisterSpec for NVIC_ICER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icer::R`](R) reader structure"]
impl crate::Readable for NVIC_ICER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_icer::W`](W) writer structure"]
impl crate::Writable for NVIC_ICER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ICER to value 0"]
impl crate::Resettable for NVIC_ICER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
