#[doc = "Register `PRIM` reader"]
pub type R = crate::R<PRIM_SPEC>;
#[doc = "Register `PRIM` writer"]
pub type W = crate::W<PRIM_SPEC>;
#[doc = "Field `POSTDIV2` reader - divide by 1-7"]
pub type POSTDIV2_R = crate::FieldReader;
#[doc = "Field `POSTDIV2` writer - divide by 1-7"]
pub type POSTDIV2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `POSTDIV1` reader - divide by 1-7"]
pub type POSTDIV1_R = crate::FieldReader;
#[doc = "Field `POSTDIV1` writer - divide by 1-7"]
pub type POSTDIV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv2(&mut self) -> POSTDIV2_W<PRIM_SPEC, 12> {
        POSTDIV2_W::new(self)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv1(&mut self) -> POSTDIV1_W<PRIM_SPEC, 16> {
        POSTDIV1_W::new(self)
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
#[doc = "Controls the PLL post dividers for the primary output  
 (note: this PLL does not have a secondary output)  
 the primary output is driven from VCO divided by postdiv1*postdiv2  

You can [`read`](crate::generic::Reg::read) this register and get [`prim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIM_SPEC;
impl crate::RegisterSpec for PRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prim::R`](R) reader structure"]
impl crate::Readable for PRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prim::W`](W) writer structure"]
impl crate::Writable for PRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIM to value 0x0007_7000"]
impl crate::Resettable for PRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_7000;
}
