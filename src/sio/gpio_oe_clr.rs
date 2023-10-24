#[doc = "Register `GPIO_OE_CLR` writer"]
pub type W = crate::W<GPIO_OE_CLR_SPEC>;
#[doc = "Field `GPIO_OE_CLR` writer - Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &amp;= ~wdata`"]
pub type GPIO_OE_CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &amp;= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_oe_clr(&mut self) -> GPIO_OE_CLR_W<GPIO_OE_CLR_SPEC, 0> {
        GPIO_OE_CLR_W::new(self)
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
#[doc = "GPIO output enable clear  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OE_CLR_SPEC;
impl crate::RegisterSpec for GPIO_OE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_oe_clr::W`](W) writer structure"]
impl crate::Writable for GPIO_OE_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OE_CLR to value 0"]
impl crate::Resettable for GPIO_OE_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
