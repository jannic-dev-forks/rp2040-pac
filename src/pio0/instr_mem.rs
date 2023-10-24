#[doc = "Register `INSTR_MEM%s` writer"]
pub type W = crate::W<INSTR_MEM_SPEC>;
#[doc = "Field `INSTR_MEM0` writer - "]
pub type INSTR_MEM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn instr_mem0(&mut self) -> INSTR_MEM0_W<INSTR_MEM_SPEC, 0> {
        INSTR_MEM0_W::new(self)
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
#[doc = "Write-only access to instruction memory location %s  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instr_mem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTR_MEM_SPEC;
impl crate::RegisterSpec for INSTR_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`instr_mem::W`](W) writer structure"]
impl crate::Writable for INSTR_MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTR_MEM%s to value 0"]
impl crate::Resettable for INSTR_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
