#[doc = "Register `NVIC_IPR1` reader"]
pub type R = crate::R<NVIC_IPR1_SPEC>;
#[doc = "Register `NVIC_IPR1` writer"]
pub type W = crate::W<NVIC_IPR1_SPEC>;
#[doc = "Field `IP_4` reader - Priority of interrupt 4"]
pub type IP_4_R = crate::FieldReader;
#[doc = "Field `IP_4` writer - Priority of interrupt 4"]
pub type IP_4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_5` reader - Priority of interrupt 5"]
pub type IP_5_R = crate::FieldReader;
#[doc = "Field `IP_5` writer - Priority of interrupt 5"]
pub type IP_5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_6` reader - Priority of interrupt 6"]
pub type IP_6_R = crate::FieldReader;
#[doc = "Field `IP_6` writer - Priority of interrupt 6"]
pub type IP_6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_7` reader - Priority of interrupt 7"]
pub type IP_7_R = crate::FieldReader;
#[doc = "Field `IP_7` writer - Priority of interrupt 7"]
pub type IP_7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&self) -> IP_4_R {
        IP_4_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&self) -> IP_5_R {
        IP_5_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&self) -> IP_6_R {
        IP_6_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&self) -> IP_7_R {
        IP_7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn ip_4(&mut self) -> IP_4_W<NVIC_IPR1_SPEC, 6> {
        IP_4_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn ip_5(&mut self) -> IP_5_W<NVIC_IPR1_SPEC, 14> {
        IP_5_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn ip_6(&mut self) -> IP_6_W<NVIC_IPR1_SPEC, 22> {
        IP_6_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn ip_7(&mut self) -> IP_7_W<NVIC_IPR1_SPEC, 30> {
        IP_7_W::new(self)
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
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR1_SPEC;
impl crate::RegisterSpec for NVIC_IPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr1::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr1::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR1 to value 0"]
impl crate::Resettable for NVIC_IPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
