#[doc = "Register `PROC_CONFIG` reader"]
pub type R = crate::R<PROC_CONFIG_SPEC>;
#[doc = "Register `PROC_CONFIG` writer"]
pub type W = crate::W<PROC_CONFIG_SPEC>;
#[doc = "Field `PROC0_HALTED` reader - Indication that proc0 has halted"]
pub type PROC0_HALTED_R = crate::BitReader;
#[doc = "Field `PROC1_HALTED` reader - Indication that proc1 has halted"]
pub type PROC1_HALTED_R = crate::BitReader;
#[doc = "Field `PROC0_DAP_INSTID` reader - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC0_DAP_INSTID_R = crate::FieldReader;
#[doc = "Field `PROC0_DAP_INSTID` writer - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC0_DAP_INSTID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PROC1_DAP_INSTID` reader - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC1_DAP_INSTID_R = crate::FieldReader;
#[doc = "Field `PROC1_DAP_INSTID` writer - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
pub type PROC1_DAP_INSTID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Indication that proc0 has halted"]
    #[inline(always)]
    pub fn proc0_halted(&self) -> PROC0_HALTED_R {
        PROC0_HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indication that proc1 has halted"]
    #[inline(always)]
    pub fn proc1_halted(&self) -> PROC1_HALTED_R {
        PROC1_HALTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc0_dap_instid(&self) -> PROC0_DAP_INSTID_R {
        PROC0_DAP_INSTID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn proc1_dap_instid(&self) -> PROC1_DAP_INSTID_R {
        PROC1_DAP_INSTID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - Configure proc0 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    #[must_use]
    pub fn proc0_dap_instid(&mut self) -> PROC0_DAP_INSTID_W<PROC_CONFIG_SPEC, 24> {
        PROC0_DAP_INSTID_W::new(self)
    }
    #[doc = "Bits 28:31 - Configure proc1 DAP instance ID.  
 Recommend that this is NOT changed until you require debug access in multi-chip environment  
 WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    #[must_use]
    pub fn proc1_dap_instid(&mut self) -> PROC1_DAP_INSTID_W<PROC_CONFIG_SPEC, 28> {
        PROC1_DAP_INSTID_W::new(self)
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
#[doc = "Configuration for processors  

You can [`read`](crate::generic::Reg::read) this register and get [`proc_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_CONFIG_SPEC;
impl crate::RegisterSpec for PROC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_config::R`](R) reader structure"]
impl crate::Readable for PROC_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc_config::W`](W) writer structure"]
impl crate::Writable for PROC_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROC_CONFIG to value 0x1000_0000"]
impl crate::Resettable for PROC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
