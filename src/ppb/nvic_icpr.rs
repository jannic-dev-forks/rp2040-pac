#[doc = "Register `NVIC_ICPR` reader"]
pub type R = crate::R<NVIC_ICPR_SPEC>;
#[doc = "Register `NVIC_ICPR` writer"]
pub type W = crate::W<NVIC_ICPR_SPEC>;
#[doc = "Field `CLRPEND` reader - Interrupt clear-pending bits.  
 Write:  
 0 = No effect.  
 1 = Removes pending state and interrupt.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending."]
pub type CLRPEND_R = crate::FieldReader<u32>;
#[doc = "Field `CLRPEND` writer - Interrupt clear-pending bits.  
 Write:  
 0 = No effect.  
 1 = Removes pending state and interrupt.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending."]
pub type CLRPEND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits.  
 Write:  
 0 = No effect.  
 1 = Removes pending state and interrupt.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending."]
    #[inline(always)]
    pub fn clrpend(&self) -> CLRPEND_R {
        CLRPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits.  
 Write:  
 0 = No effect.  
 1 = Removes pending state and interrupt.  
 Read:  
 0 = Interrupt is not pending.  
 1 = Interrupt is pending."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend(&mut self) -> CLRPEND_W<NVIC_ICPR_SPEC, 0> {
        CLRPEND_W::new(self)
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
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ICPR_SPEC;
impl crate::RegisterSpec for NVIC_ICPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icpr::R`](R) reader structure"]
impl crate::Readable for NVIC_ICPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_icpr::W`](W) writer structure"]
impl crate::Writable for NVIC_ICPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ICPR to value 0"]
impl crate::Resettable for NVIC_ICPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
