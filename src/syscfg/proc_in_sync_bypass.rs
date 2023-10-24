#[doc = "Register `PROC_IN_SYNC_BYPASS` reader"]
pub type R = crate::R<PROC_IN_SYNC_BYPASS_SPEC>;
#[doc = "Register `PROC_IN_SYNC_BYPASS` writer"]
pub type W = crate::W<PROC_IN_SYNC_BYPASS_SPEC>;
#[doc = "Field `PROC_IN_SYNC_BYPASS` reader - "]
pub type PROC_IN_SYNC_BYPASS_R = crate::FieldReader<u32>;
#[doc = "Field `PROC_IN_SYNC_BYPASS` writer - "]
pub type PROC_IN_SYNC_BYPASS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn proc_in_sync_bypass(&self) -> PROC_IN_SYNC_BYPASS_R {
        PROC_IN_SYNC_BYPASS_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn proc_in_sync_bypass(&mut self) -> PROC_IN_SYNC_BYPASS_W<PROC_IN_SYNC_BYPASS_SPEC, 0> {
        PROC_IN_SYNC_BYPASS_W::new(self)
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
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO  
 and the GPIO input register in the SIO. The input synchronizers should  
 generally be unbypassed, to avoid injecting metastabilities into processors.  
 If you're feeling brave, you can bypass to save two cycles of input  
 latency. This register applies to GPIO 0...29.  

You can [`read`](crate::generic::Reg::read) this register and get [`proc_in_sync_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_in_sync_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_IN_SYNC_BYPASS_SPEC;
impl crate::RegisterSpec for PROC_IN_SYNC_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_in_sync_bypass::R`](R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc_in_sync_bypass::W`](W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROC_IN_SYNC_BYPASS to value 0"]
impl crate::Resettable for PROC_IN_SYNC_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
