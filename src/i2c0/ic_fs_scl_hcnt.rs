#[doc = "Register `IC_FS_SCL_HCNT` reader"]
pub type R = crate::R<IC_FS_SCL_HCNT_SPEC>;
#[doc = "Register `IC_FS_SCL_HCNT` writer"]
pub type W = crate::W<IC_FS_SCL_HCNT_SPEC>;
#[doc = "Field `IC_FS_SCL_HCNT` reader - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
pub type IC_FS_SCL_HCNT_R = crate::FieldReader<u16>;
#[doc = "Field `IC_FS_SCL_HCNT` writer - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
pub type IC_FS_SCL_HCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(&self) -> IC_FS_SCL_HCNT_R {
        IC_FS_SCL_HCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ic_fs_scl_hcnt(&mut self) -> IC_FS_SCL_HCNT_W<IC_FS_SCL_HCNT_SPEC> {
        IC_FS_SCL_HCNT_W::new(self, 0)
    }
}
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_fs_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_fs_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_FS_SCL_HCNT_SPEC;
impl crate::RegisterSpec for IC_FS_SCL_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_fs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for IC_FS_SCL_HCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_fs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for IC_FS_SCL_HCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_FS_SCL_HCNT to value 0x06"]
impl crate::Resettable for IC_FS_SCL_HCNT_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
