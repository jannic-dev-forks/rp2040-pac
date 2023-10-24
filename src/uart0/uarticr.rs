#[doc = "Register `UARTICR` reader"]
pub type R = crate::R<UARTICR_SPEC>;
#[doc = "Register `UARTICR` writer"]
pub type W = crate::W<UARTICR_SPEC>;
#[doc = "Field `RIMIC` reader - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
pub type RIMIC_R = crate::BitReader;
#[doc = "Field `RIMIC` writer - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
pub type RIMIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CTSMIC` reader - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub type CTSMIC_R = crate::BitReader;
#[doc = "Field `CTSMIC` writer - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub type CTSMIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `DCDMIC` reader - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub type DCDMIC_R = crate::BitReader;
#[doc = "Field `DCDMIC` writer - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub type DCDMIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `DSRMIC` reader - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub type DSRMIC_R = crate::BitReader;
#[doc = "Field `DSRMIC` writer - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub type DSRMIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RXIC` reader - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub type RXIC_R = crate::BitReader;
#[doc = "Field `RXIC` writer - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub type RXIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `TXIC` reader - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub type TXIC_R = crate::BitReader;
#[doc = "Field `TXIC` writer - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub type TXIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RTIC` reader - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub type RTIC_R = crate::BitReader;
#[doc = "Field `RTIC` writer - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub type RTIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `FEIC` reader - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub type FEIC_R = crate::BitReader;
#[doc = "Field `FEIC` writer - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub type FEIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `PEIC` reader - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub type PEIC_R = crate::BitReader;
#[doc = "Field `PEIC` writer - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub type PEIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `BEIC` reader - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub type BEIC_R = crate::BitReader;
#[doc = "Field `BEIC` writer - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub type BEIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `OEIC` reader - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
pub type OEIC_R = crate::BitReader;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
pub type OEIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rimic(&self) -> RIMIC_R {
        RIMIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmic(&self) -> DCDMIC_R {
        DCDMIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmic(&self) -> DSRMIC_R {
        DSRMIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rimic(&mut self) -> RIMIC_W<UARTICR_SPEC, 0> {
        RIMIC_W::new(self)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CTSMIC_W<UARTICR_SPEC, 1> {
        CTSMIC_W::new(self)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dcdmic(&mut self) -> DCDMIC_W<UARTICR_SPEC, 2> {
        DCDMIC_W::new(self)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsrmic(&mut self) -> DSRMIC_W<UARTICR_SPEC, 3> {
        DSRMIC_W::new(self)
    }
    #[doc = "Bit 4 - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RXIC_W<UARTICR_SPEC, 4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TXIC_W<UARTICR_SPEC, 5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<UARTICR_SPEC, 6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FEIC_W<UARTICR_SPEC, 7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PEIC_W<UARTICR_SPEC, 8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BEIC_W<UARTICR_SPEC, 9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OEIC_W<UARTICR_SPEC, 10> {
        OEIC_W::new(self)
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
#[doc = "Interrupt Clear Register, UARTICR  

You can [`read`](crate::generic::Reg::read) this register and get [`uarticr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uarticr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTICR_SPEC;
impl crate::RegisterSpec for UARTICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uarticr::R`](R) reader structure"]
impl crate::Readable for UARTICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uarticr::W`](W) writer structure"]
impl crate::Writable for UARTICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07ff;
}
#[doc = "`reset()` method sets UARTICR to value 0"]
impl crate::Resettable for UARTICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
