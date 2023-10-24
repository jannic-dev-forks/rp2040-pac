#[doc = "Register `DMARDLR` reader"]
pub type R = crate::R<DMARDLR_SPEC>;
#[doc = "Register `DMARDLR` writer"]
pub type W = crate::W<DMARDLR_SPEC>;
#[doc = "Field `DMARDL` reader - Receive data watermark level (DMARDLR+1)"]
pub type DMARDL_R = crate::FieldReader;
#[doc = "Field `DMARDL` writer - Receive data watermark level (DMARDLR+1)"]
pub type DMARDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    #[must_use]
    pub fn dmardl(&mut self) -> DMARDL_W<DMARDLR_SPEC, 0> {
        DMARDL_W::new(self)
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
#[doc = "DMA RX data level  

You can [`read`](crate::generic::Reg::read) this register and get [`dmardlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARDLR_SPEC;
impl crate::RegisterSpec for DMARDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardlr::R`](R) reader structure"]
impl crate::Readable for DMARDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmardlr::W`](W) writer structure"]
impl crate::Writable for DMARDLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARDLR to value 0"]
impl crate::Resettable for DMARDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
