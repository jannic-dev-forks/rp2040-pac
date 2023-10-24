#[doc = "Register `GPIO_HI_OE` reader"]
pub type R = crate::R<GPIO_HI_OE_SPEC>;
#[doc = "Register `GPIO_HI_OE` writer"]
pub type W = crate::W<GPIO_HI_OE_SPEC>;
#[doc = "Field `GPIO_HI_OE` reader - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OE_R = crate::FieldReader;
#[doc = "Field `GPIO_HI_OE` writer - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_oe(&self) -> GPIO_HI_OE_R {
        GPIO_HI_OE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hi_oe(&mut self) -> GPIO_HI_OE_W<GPIO_HI_OE_SPEC, 0> {
        GPIO_HI_OE_W::new(self)
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
#[doc = "QSPI output enable  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_oe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_OE_SPEC;
impl crate::RegisterSpec for GPIO_HI_OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hi_oe::R`](R) reader structure"]
impl crate::Readable for GPIO_HI_OE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_hi_oe::W`](W) writer structure"]
impl crate::Writable for GPIO_HI_OE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OE to value 0"]
impl crate::Resettable for GPIO_HI_OE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
