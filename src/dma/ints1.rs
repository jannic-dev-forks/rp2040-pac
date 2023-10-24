#[doc = "Register `INTS1` reader"]
pub type R = crate::R<INTS1_SPEC>;
#[doc = "Register `INTS1` writer"]
pub type W = crate::W<INTS1_SPEC>;
#[doc = "Field `INTS1` reader - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS1_R = crate::FieldReader<u16>;
#[doc = "Field `INTS1` writer - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
pub type INTS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    pub fn ints1(&self) -> INTS1_R {
        INTS1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted.  
 Channel interrupts can be cleared by writing a bit mask here."]
    #[inline(always)]
    #[must_use]
    pub fn ints1(&mut self) -> INTS1_W<INTS1_SPEC, 0> {
        INTS1_W::new(self)
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
#[doc = "Interrupt Status (masked) for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`ints1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS1_SPEC;
impl crate::RegisterSpec for INTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints1::R`](R) reader structure"]
impl crate::Readable for INTS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints1::W`](W) writer structure"]
impl crate::Writable for INTS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff;
}
#[doc = "`reset()` method sets INTS1 to value 0"]
impl crate::Resettable for INTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
