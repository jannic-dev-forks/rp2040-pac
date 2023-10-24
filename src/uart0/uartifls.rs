#[doc = "Register `UARTIFLS` reader"]
pub type R = crate::R<UARTIFLS_SPEC>;
#[doc = "Register `UARTIFLS` writer"]
pub type W = crate::W<UARTIFLS_SPEC>;
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes &lt;= 1 / 8 full b001 = Transmit FIFO becomes &lt;= 1 / 4 full b010 = Transmit FIFO becomes &lt;= 1 / 2 full b011 = Transmit FIFO becomes &lt;= 3 / 4 full b100 = Transmit FIFO becomes &lt;= 7 / 8 full b101-b111 = reserved."]
pub type TXIFLSEL_R = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes &lt;= 1 / 8 full b001 = Transmit FIFO becomes &lt;= 1 / 4 full b010 = Transmit FIFO becomes &lt;= 1 / 2 full b011 = Transmit FIFO becomes &lt;= 3 / 4 full b100 = Transmit FIFO becomes &lt;= 7 / 8 full b101-b111 = reserved."]
pub type TXIFLSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
pub type RXIFLSEL_R = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
pub type RXIFLSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes &lt;= 1 / 8 full b001 = Transmit FIFO becomes &lt;= 1 / 4 full b010 = Transmit FIFO becomes &lt;= 1 / 2 full b011 = Transmit FIFO becomes &lt;= 3 / 4 full b100 = Transmit FIFO becomes &lt;= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes &lt;= 1 / 8 full b001 = Transmit FIFO becomes &lt;= 1 / 4 full b010 = Transmit FIFO becomes &lt;= 1 / 2 full b011 = Transmit FIFO becomes &lt;= 3 / 4 full b100 = Transmit FIFO becomes &lt;= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W<UARTIFLS_SPEC, 0> {
        TXIFLSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W<UARTIFLS_SPEC, 3> {
        RXIFLSEL_W::new(self)
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
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS  

You can [`read`](crate::generic::Reg::read) this register and get [`uartifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTIFLS_SPEC;
impl crate::RegisterSpec for UARTIFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartifls::R`](R) reader structure"]
impl crate::Readable for UARTIFLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartifls::W`](W) writer structure"]
impl crate::Writable for UARTIFLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTIFLS to value 0x12"]
impl crate::Resettable for UARTIFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0x12;
}
