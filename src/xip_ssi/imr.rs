#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `TXEIM` reader - Transmit FIFO empty interrupt mask"]
pub type TXEIM_R = crate::BitReader;
#[doc = "Field `TXEIM` writer - Transmit FIFO empty interrupt mask"]
pub type TXEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOIM` reader - Transmit FIFO overflow interrupt mask"]
pub type TXOIM_R = crate::BitReader;
#[doc = "Field `TXOIM` writer - Transmit FIFO overflow interrupt mask"]
pub type TXOIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUIM` reader - Receive FIFO underflow interrupt mask"]
pub type RXUIM_R = crate::BitReader;
#[doc = "Field `RXUIM` writer - Receive FIFO underflow interrupt mask"]
pub type RXUIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOIM` reader - Receive FIFO overflow interrupt mask"]
pub type RXOIM_R = crate::BitReader;
#[doc = "Field `RXOIM` writer - Receive FIFO overflow interrupt mask"]
pub type RXOIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFIM` reader - Receive FIFO full interrupt mask"]
pub type RXFIM_R = crate::BitReader;
#[doc = "Field `RXFIM` writer - Receive FIFO full interrupt mask"]
pub type RXFIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSTIM` reader - Multi-master contention interrupt mask"]
pub type MSTIM_R = crate::BitReader;
#[doc = "Field `MSTIM` writer - Multi-master contention interrupt mask"]
pub type MSTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txeim(&self) -> TXEIM_R {
        TXEIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn txoim(&self) -> TXOIM_R {
        TXOIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn rxuim(&self) -> RXUIM_R {
        RXUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn rxoim(&self) -> RXOIM_R {
        RXOIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxfim(&self) -> RXFIM_R {
        RXFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn mstim(&self) -> MSTIM_R {
        MSTIM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn txeim(&mut self) -> TXEIM_W<IMR_SPEC, 0> {
        TXEIM_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn txoim(&mut self) -> TXOIM_W<IMR_SPEC, 1> {
        TXOIM_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxuim(&mut self) -> RXUIM_W<IMR_SPEC, 2> {
        RXUIM_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxoim(&mut self) -> RXOIM_W<IMR_SPEC, 3> {
        RXOIM_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxfim(&mut self) -> RXFIM_W<IMR_SPEC, 4> {
        RXFIM_W::new(self)
    }
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mstim(&mut self) -> MSTIM_W<IMR_SPEC, 5> {
        MSTIM_W::new(self)
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
#[doc = "Interrupt mask  

You can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
