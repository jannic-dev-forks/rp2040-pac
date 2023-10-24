#[doc = "Register `ALARM2` reader"]
pub type R = crate::R<ALARM2_SPEC>;
#[doc = "Register `ALARM2` writer"]
pub type W = crate::W<ALARM2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ALARM2_SPEC> {
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
#[doc = "Arm alarm 2, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::generic::Reg::read) this register and get [`alarm2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM2_SPEC;
impl crate::RegisterSpec for ALARM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm2::R`](R) reader structure"]
impl crate::Readable for ALARM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm2::W`](W) writer structure"]
impl crate::Writable for ALARM2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARM2 to value 0"]
impl crate::Resettable for ALARM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
