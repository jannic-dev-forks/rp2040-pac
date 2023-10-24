#[doc = "Register `VOLTAGE_SELECT` reader"]
pub type R = crate::R<VOLTAGE_SELECT_SPEC>;
#[doc = "Register `VOLTAGE_SELECT` writer"]
pub type W = crate::W<VOLTAGE_SELECT_SPEC>;
#[doc = "Field `VOLTAGE_SELECT` reader - "]
pub type VOLTAGE_SELECT_R = crate::BitReader<VOLTAGE_SELECT_A>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOLTAGE_SELECT_A {
    #[doc = "0: Set voltage to 3.3V (DVDD >= 2V5)"]
    _3V3 = 0,
    #[doc = "1: Set voltage to 1.8V (DVDD &lt;= 1V8)"]
    _1V8 = 1,
}
impl From<VOLTAGE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl VOLTAGE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOLTAGE_SELECT_A {
        match self.bits {
            false => VOLTAGE_SELECT_A::_3V3,
            true => VOLTAGE_SELECT_A::_1V8,
        }
    }
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == VOLTAGE_SELECT_A::_3V3
    }
    #[doc = "Set voltage to 1.8V (DVDD &lt;= 1V8)"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == VOLTAGE_SELECT_A::_1V8
    }
}
#[doc = "Field `VOLTAGE_SELECT` writer - "]
pub type VOLTAGE_SELECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VOLTAGE_SELECT_A>;
impl<'a, REG, const O: u8> VOLTAGE_SELECT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut crate::W<REG> {
        self.variant(VOLTAGE_SELECT_A::_3V3)
    }
    #[doc = "Set voltage to 1.8V (DVDD &lt;= 1V8)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(VOLTAGE_SELECT_A::_1V8)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&self) -> VOLTAGE_SELECT_R {
        VOLTAGE_SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn voltage_select(&mut self) -> VOLTAGE_SELECT_W<VOLTAGE_SELECT_SPEC, 0> {
        VOLTAGE_SELECT_W::new(self)
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
#[doc = "Voltage select. Per bank control  

You can [`read`](crate::generic::Reg::read) this register and get [`voltage_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`voltage_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VOLTAGE_SELECT_SPEC;
impl crate::RegisterSpec for VOLTAGE_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`voltage_select::R`](R) reader structure"]
impl crate::Readable for VOLTAGE_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`voltage_select::W`](W) writer structure"]
impl crate::Writable for VOLTAGE_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VOLTAGE_SELECT to value 0"]
impl crate::Resettable for VOLTAGE_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
