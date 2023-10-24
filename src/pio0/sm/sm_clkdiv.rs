#[doc = "Register `SM_CLKDIV` reader"]
pub type R = crate::R<SM_CLKDIV_SPEC>;
#[doc = "Register `SM_CLKDIV` writer"]
pub type W = crate::W<SM_CLKDIV_SPEC>;
#[doc = "Field `FRAC` reader - Fractional part of clock divisor"]
pub type FRAC_R = crate::FieldReader;
#[doc = "Field `FRAC` writer - Fractional part of clock divisor"]
pub type FRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT` reader - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
pub type INT_R = crate::FieldReader<u16>;
#[doc = "Field `INT` writer - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
pub type INT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 8:15 - Fractional part of clock divisor"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - Fractional part of clock divisor"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<SM_CLKDIV_SPEC, 8> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 16:31 - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<SM_CLKDIV_SPEC, 16> {
        INT_W::new(self)
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
#[doc = "Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_CLKDIV_SPEC;
impl crate::RegisterSpec for SM_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_clkdiv::R`](R) reader structure"]
impl crate::Readable for SM_CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_clkdiv::W`](W) writer structure"]
impl crate::Writable for SM_CLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SM_CLKDIV to value 0x0001_0000"]
impl crate::Resettable for SM_CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
