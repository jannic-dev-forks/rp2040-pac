#[doc = "Register `SHPR3` reader"]
pub type R = crate::R<SHPR3_SPEC>;
#[doc = "Register `SHPR3` writer"]
pub type W = crate::W<SHPR3_SPEC>;
#[doc = "Field `PRI_14` reader - Priority of system handler 14, PendSV"]
pub type PRI_14_R = crate::FieldReader;
#[doc = "Field `PRI_14` writer - Priority of system handler 14, PendSV"]
pub type PRI_14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRI_15` reader - Priority of system handler 15, SysTick"]
pub type PRI_15_R = crate::FieldReader;
#[doc = "Field `PRI_15` writer - Priority of system handler 15, SysTick"]
pub type PRI_15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 22:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of system handler 15, SysTick"]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> PRI_14_W<SHPR3_SPEC, 22> {
        PRI_14_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of system handler 15, SysTick"]
    #[inline(always)]
    #[must_use]
    pub fn pri_15(&mut self) -> PRI_15_W<SHPR3_SPEC, 30> {
        PRI_15_W::new(self)
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
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick.  

You can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR3_SPEC;
impl crate::RegisterSpec for SHPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr3::R`](R) reader structure"]
impl crate::Readable for SHPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr3::W`](W) writer structure"]
impl crate::Writable for SHPR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for SHPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
