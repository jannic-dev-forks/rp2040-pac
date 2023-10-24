#[doc = "Register `NVIC_IPR5` reader"]
pub type R = crate::R<NVIC_IPR5_SPEC>;
#[doc = "Register `NVIC_IPR5` writer"]
pub type W = crate::W<NVIC_IPR5_SPEC>;
#[doc = "Field `IP_20` reader - Priority of interrupt 20"]
pub type IP_20_R = crate::FieldReader;
#[doc = "Field `IP_20` writer - Priority of interrupt 20"]
pub type IP_20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_21` reader - Priority of interrupt 21"]
pub type IP_21_R = crate::FieldReader;
#[doc = "Field `IP_21` writer - Priority of interrupt 21"]
pub type IP_21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_22` reader - Priority of interrupt 22"]
pub type IP_22_R = crate::FieldReader;
#[doc = "Field `IP_22` writer - Priority of interrupt 22"]
pub type IP_22_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IP_23` reader - Priority of interrupt 23"]
pub type IP_23_R = crate::FieldReader;
#[doc = "Field `IP_23` writer - Priority of interrupt 23"]
pub type IP_23_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&self) -> IP_20_R {
        IP_20_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&self) -> IP_21_R {
        IP_21_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&self) -> IP_22_R {
        IP_22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&self) -> IP_23_R {
        IP_23_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn ip_20(&mut self) -> IP_20_W<NVIC_IPR5_SPEC, 6> {
        IP_20_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn ip_21(&mut self) -> IP_21_W<NVIC_IPR5_SPEC, 14> {
        IP_21_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn ip_22(&mut self) -> IP_22_W<NVIC_IPR5_SPEC, 22> {
        IP_22_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn ip_23(&mut self) -> IP_23_W<NVIC_IPR5_SPEC, 30> {
        IP_23_W::new(self)
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

You can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR5_SPEC;
impl crate::RegisterSpec for NVIC_IPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr5::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr5::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
