#[doc = "Register `EP_CONTROL%s` reader"]
pub type R = crate::R<EP_CONTROL_SPEC>;
#[doc = "Register `EP_CONTROL%s` writer"]
pub type W = crate::W<EP_CONTROL_SPEC>;
#[doc = "Field `BUFFER_ADDRESS` reader - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub type BUFFER_ADDRESS_R = crate::FieldReader<u16>;
#[doc = "Field `BUFFER_ADDRESS` writer - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
pub type BUFFER_ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `INTERRUPT_ON_NAK` reader - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub type INTERRUPT_ON_NAK_R = crate::BitReader;
#[doc = "Field `INTERRUPT_ON_NAK` writer - Trigger an interrupt if a NAK is sent. Intended for debug only."]
pub type INTERRUPT_ON_NAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_POLL_INTERVAL` reader - The interval the host controller should poll this endpoint. Only applicable for interrupt endpoints. Specified in ms - 1. For example: a value of 9 would poll the endpoint every 10ms."]
pub type HOST_POLL_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_POLL_INTERVAL` writer - The interval the host controller should poll this endpoint. Only applicable for interrupt endpoints. Specified in ms - 1. For example: a value of 9 would poll the endpoint every 10ms."]
pub type HOST_POLL_INTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `INTERRUPT_ON_STALL` reader - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub type INTERRUPT_ON_STALL_R = crate::BitReader;
#[doc = "Field `INTERRUPT_ON_STALL` writer - Trigger an interrupt if a STALL is sent. Intended for debug only."]
pub type INTERRUPT_ON_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDPOINT_TYPE` reader - "]
pub type ENDPOINT_TYPE_R = crate::FieldReader<ENDPOINT_TYPE_A>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENDPOINT_TYPE_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<ENDPOINT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDPOINT_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENDPOINT_TYPE_A {
    type Ux = u8;
}
impl ENDPOINT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDPOINT_TYPE_A {
        match self.bits {
            0 => ENDPOINT_TYPE_A::CONTROL,
            1 => ENDPOINT_TYPE_A::ISOCHRONOUS,
            2 => ENDPOINT_TYPE_A::BULK,
            3 => ENDPOINT_TYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == ENDPOINT_TYPE_A::CONTROL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == ENDPOINT_TYPE_A::ISOCHRONOUS
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == ENDPOINT_TYPE_A::BULK
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ENDPOINT_TYPE_A::INTERRUPT
    }
}
#[doc = "Field `ENDPOINT_TYPE` writer - "]
pub type ENDPOINT_TYPE_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, ENDPOINT_TYPE_A>;
impl<'a, REG, const O: u8> ENDPOINT_TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(ENDPOINT_TYPE_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut crate::W<REG> {
        self.variant(ENDPOINT_TYPE_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(ENDPOINT_TYPE_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(ENDPOINT_TYPE_A::INTERRUPT)
    }
}
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` reader - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub type INTERRUPT_PER_DOUBLE_BUFF_R = crate::BitReader;
#[doc = "Field `INTERRUPT_PER_DOUBLE_BUFF` writer - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
pub type INTERRUPT_PER_DOUBLE_BUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRUPT_PER_BUFF` reader - Trigger an interrupt each time a buffer is done."]
pub type INTERRUPT_PER_BUFF_R = crate::BitReader;
#[doc = "Field `INTERRUPT_PER_BUFF` writer - Trigger an interrupt each time a buffer is done."]
pub type INTERRUPT_PER_BUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOUBLE_BUFFERED` reader - This endpoint is double buffered."]
pub type DOUBLE_BUFFERED_R = crate::BitReader;
#[doc = "Field `DOUBLE_BUFFERED` writer - This endpoint is double buffered."]
pub type DOUBLE_BUFFERED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_nak(&self) -> INTERRUPT_ON_NAK_R {
        INTERRUPT_ON_NAK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 16:25 - The interval the host controller should poll this endpoint. Only applicable for interrupt endpoints. Specified in ms - 1. For example: a value of 9 would poll the endpoint every 10ms."]
    #[inline(always)]
    pub fn host_poll_interval(&self) -> HOST_POLL_INTERVAL_R {
        HOST_POLL_INTERVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub fn interrupt_on_stall(&self) -> INTERRUPT_ON_STALL_R {
        INTERRUPT_ON_STALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn endpoint_type(&self) -> ENDPOINT_TYPE_R {
        ENDPOINT_TYPE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub fn interrupt_per_double_buff(&self) -> INTERRUPT_PER_DOUBLE_BUFF_R {
        INTERRUPT_PER_DOUBLE_BUFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub fn interrupt_per_buff(&self) -> INTERRUPT_PER_BUFF_R {
        INTERRUPT_PER_BUFF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    pub fn double_buffered(&self) -> DOUBLE_BUFFERED_R {
        DOUBLE_BUFFERED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    #[must_use]
    pub fn buffer_address(&mut self) -> BUFFER_ADDRESS_W<EP_CONTROL_SPEC, 0> {
        BUFFER_ADDRESS_W::new(self)
    }
    #[doc = "Bit 16 - Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_nak(&mut self) -> INTERRUPT_ON_NAK_W<EP_CONTROL_SPEC, 16> {
        INTERRUPT_ON_NAK_W::new(self)
    }
    #[doc = "Bits 16:25 - The interval the host controller should poll this endpoint. Only applicable for interrupt endpoints. Specified in ms - 1. For example: a value of 9 would poll the endpoint every 10ms."]
    #[inline(always)]
    #[must_use]
    pub fn host_poll_interval(&mut self) -> HOST_POLL_INTERVAL_W<EP_CONTROL_SPEC, 16> {
        HOST_POLL_INTERVAL_W::new(self)
    }
    #[doc = "Bit 17 - Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_stall(&mut self) -> INTERRUPT_ON_STALL_W<EP_CONTROL_SPEC, 17> {
        INTERRUPT_ON_STALL_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint_type(&mut self) -> ENDPOINT_TYPE_W<EP_CONTROL_SPEC, 26> {
        ENDPOINT_TYPE_W::new(self)
    }
    #[doc = "Bit 28 - Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_per_double_buff(
        &mut self,
    ) -> INTERRUPT_PER_DOUBLE_BUFF_W<EP_CONTROL_SPEC, 28> {
        INTERRUPT_PER_DOUBLE_BUFF_W::new(self)
    }
    #[doc = "Bit 29 - Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_per_buff(&mut self) -> INTERRUPT_PER_BUFF_W<EP_CONTROL_SPEC, 29> {
        INTERRUPT_PER_BUFF_W::new(self)
    }
    #[doc = "Bit 30 - This endpoint is double buffered."]
    #[inline(always)]
    #[must_use]
    pub fn double_buffered(&mut self) -> DOUBLE_BUFFERED_W<EP_CONTROL_SPEC, 30> {
        DOUBLE_BUFFERED_W::new(self)
    }
    #[doc = "Bit 31 - Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<EP_CONTROL_SPEC, 31> {
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
#[doc = "-  

You can [`read`](crate::generic::Reg::read) this register and get [`ep_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_CONTROL_SPEC;
impl crate::RegisterSpec for EP_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_control::R`](R) reader structure"]
impl crate::Readable for EP_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_control::W`](W) writer structure"]
impl crate::Writable for EP_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_CONTROL%s to value 0"]
impl crate::Resettable for EP_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
