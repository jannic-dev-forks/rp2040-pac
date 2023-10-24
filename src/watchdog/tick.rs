#[doc = "Register `TICK` reader"]
pub type R = crate::R<TICK_SPEC>;
#[doc = "Register `TICK` writer"]
pub type W = crate::W<TICK_SPEC>;
#[doc = "Field `CYCLES` reader - Total number of clk_tick cycles before the next tick."]
pub type CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `CYCLES` writer - Total number of clk_tick cycles before the next tick."]
pub type CYCLES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `ENABLE` reader - start / stop tick generation"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - start / stop tick generation"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNNING` reader - Is the tick generator running?"]
pub type RUNNING_R = crate::BitReader;
#[doc = "Field `COUNT` reader - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Is the tick generator running?"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:19 - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CYCLES_W<TICK_SPEC, 0> {
        CYCLES_W::new(self)
    }
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<TICK_SPEC, 9> {
        ENABLE_W::new(self)
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
#[doc = "Controls the tick generator  

You can [`read`](crate::generic::Reg::read) this register and get [`tick::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TICK_SPEC;
impl crate::RegisterSpec for TICK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tick::R`](R) reader structure"]
impl crate::Readable for TICK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tick::W`](W) writer structure"]
impl crate::Writable for TICK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICK to value 0x0200"]
impl crate::Resettable for TICK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
