#[doc = "Register `SHPR2` reader"]
pub type R = crate::R<SHPR2_SPEC>;
#[doc = "Register `SHPR2` writer"]
pub type W = crate::W<SHPR2_SPEC>;
#[doc = "Field `PRI_11` reader - Priority of system handler 11, SVCall"]
pub type PRI_11_R = crate::FieldReader;
#[doc = "Field `PRI_11` writer - Priority of system handler 11, SVCall"]
pub type PRI_11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 30:31 - Priority of system handler 11, SVCall"]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of system handler 11, SVCall"]
    #[inline(always)]
    #[must_use]
    pub fn pri_11(&mut self) -> PRI_11_W<SHPR2_SPEC, 30> {
        PRI_11_W::new(self)
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
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall.  

You can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR2_SPEC;
impl crate::RegisterSpec for SHPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr2::R`](R) reader structure"]
impl crate::Readable for SHPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr2::W`](W) writer structure"]
impl crate::Writable for SHPR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHPR2 to value 0"]
impl crate::Resettable for SHPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
