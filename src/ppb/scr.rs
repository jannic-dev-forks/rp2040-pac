#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Indicates sleep-on-exit when returning from Handler mode to Thread mode:  
 0 = Do not sleep when returning to Thread mode.  
 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.  
 Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
pub type SLEEPONEXIT_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPONEXIT` writer - Indicates sleep-on-exit when returning from Handler mode to Thread mode:  
 0 = Do not sleep when returning to Thread mode.  
 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.  
 Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
pub type SLEEPONEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SLEEPDEEP` reader - Controls whether the processor uses sleep or deep sleep as its low power mode:  
 0 = Sleep.  
 1 = Deep sleep."]
pub type SLEEPDEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEP` writer - Controls whether the processor uses sleep or deep sleep as its low power mode:  
 0 = Sleep.  
 1 = Deep sleep."]
pub type SLEEPDEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit:  
 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.  
 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.  
 When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the  
 processor is not waiting for an event, the event is registered and affects the next WFE.  
 The processor also wakes up on execution of an SEV instruction or an external event."]
pub type SEVONPEND_R = crate::BitReader<bool>;
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit:  
 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.  
 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.  
 When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the  
 processor is not waiting for an event, the event is registered and affects the next WFE.  
 The processor also wakes up on execution of an SEV instruction or an external event."]
pub type SEVONPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode:  
 0 = Do not sleep when returning to Thread mode.  
 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.  
 Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode:  
 0 = Sleep.  
 1 = Deep sleep."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit:  
 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.  
 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.  
 When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the  
 processor is not waiting for an event, the event is registered and affects the next WFE.  
 The processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode:  
 0 = Do not sleep when returning to Thread mode.  
 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode.  
 Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode:  
 0 = Sleep.  
 1 = Deep sleep."]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 4 - Send Event on Pending bit:  
 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded.  
 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor.  
 When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the  
 processor is not waiting for an event, the event is registered and affects the next WFE.  
 The processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
