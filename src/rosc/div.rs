#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIV` reader - set to 0xaa0 + div where  
 div = 0 divides by 32  
 div = 1-31 divides by div  
 any other value sets div=31  
 this register resets to div=16"]
pub type DIV_R = crate::FieldReader<DIV_A>;
#[doc = "set to 0xaa0 + div where  
 div = 0 divides by 32  
 div = 1-31 divides by div  
 any other value sets div=31  
 this register resets to div=16  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DIV_A {
    #[doc = "2720: `101010100000`"]
    PASS = 2720,
}
impl From<DIV_A> for u16 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIV_A {
    type Ux = u16;
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIV_A> {
        match self.bits {
            2720 => Some(DIV_A::PASS),
            _ => None,
        }
    }
    #[doc = "`101010100000`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == DIV_A::PASS
    }
}
#[doc = "Field `DIV` writer - set to 0xaa0 + div where  
 div = 0 divides by 32  
 div = 1-31 divides by div  
 any other value sets div=31  
 this register resets to div=16"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, DIV_A>;
impl<'a, REG, const O: u8> DIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`101010100000`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::PASS)
    }
}
impl R {
    #[doc = "Bits 0:11 - set to 0xaa0 + div where  
 div = 0 divides by 32  
 div = 1-31 divides by div  
 any other value sets div=31  
 this register resets to div=16"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - set to 0xaa0 + div where  
 div = 0 divides by 32  
 div = 1-31 divides by div  
 any other value sets div=31  
 this register resets to div=16"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIV_SPEC, 0> {
        DIV_W::new(self)
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
#[doc = "Controls the output divider  

You can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
