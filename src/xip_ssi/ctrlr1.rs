#[doc = "Register `CTRLR1` reader"]
pub type R = crate::R<CTRLR1_SPEC>;
#[doc = "Register `CTRLR1` writer"]
pub type W = crate::W<CTRLR1_SPEC>;
#[doc = "Field `NDF` reader - Number of data frames"]
pub type NDF_R = crate::FieldReader<u16>;
#[doc = "Field `NDF` writer - Number of data frames"]
pub type NDF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    #[must_use]
    pub fn ndf(&mut self) -> NDF_W<CTRLR1_SPEC, 0> {
        NDF_W::new(self)
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
#[doc = "Master Control register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`ctrlr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLR1_SPEC;
impl crate::RegisterSpec for CTRLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr1::R`](R) reader structure"]
impl crate::Readable for CTRLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlr1::W`](W) writer structure"]
impl crate::Writable for CTRLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLR1 to value 0"]
impl crate::Resettable for CTRLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
