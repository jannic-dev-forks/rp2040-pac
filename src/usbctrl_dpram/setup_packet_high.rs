#[doc = "Register `SETUP_PACKET_HIGH` reader"]
pub type R = crate::R<SETUP_PACKET_HIGH_SPEC>;
#[doc = "Register `SETUP_PACKET_HIGH` writer"]
pub type W = crate::W<SETUP_PACKET_HIGH_SPEC>;
#[doc = "Field `WINDEX` reader - "]
pub type WINDEX_R = crate::FieldReader<u16>;
#[doc = "Field `WINDEX` writer - "]
pub type WINDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `WLENGTH` reader - "]
pub type WLENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `WLENGTH` writer - "]
pub type WLENGTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn windex(&mut self) -> WINDEX_W<SETUP_PACKET_HIGH_SPEC, 0> {
        WINDEX_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn wlength(&mut self) -> WLENGTH_W<SETUP_PACKET_HIGH_SPEC, 16> {
        WLENGTH_W::new(self)
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
#[doc = "Bytes 4-7 of the setup packet from the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`setup_packet_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup_packet_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP_PACKET_HIGH_SPEC;
impl crate::RegisterSpec for SETUP_PACKET_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup_packet_high::R`](R) reader structure"]
impl crate::Readable for SETUP_PACKET_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup_packet_high::W`](W) writer structure"]
impl crate::Writable for SETUP_PACKET_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP_PACKET_HIGH to value 0"]
impl crate::Resettable for SETUP_PACKET_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
