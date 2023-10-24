#[doc = "Register `IC_DMA_RDLR` reader"]
pub type R = crate::R<IC_DMA_RDLR_SPEC>;
#[doc = "Register `IC_DMA_RDLR` writer"]
pub type W = crate::W<IC_DMA_RDLR_SPEC>;
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
pub type DMARDL_R = crate::FieldReader;
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
pub type DMARDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn dmardl(&mut self) -> DMARDL_W<IC_DMA_RDLR_SPEC, 0> {
        DMARDL_W::new(self)
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
#[doc = "I2C Receive Data Level Register  

You can [`read`](crate::generic::Reg::read) this register and get [`ic_dma_rdlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic_dma_rdlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_DMA_RDLR_SPEC;
impl crate::RegisterSpec for IC_DMA_RDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_rdlr::R`](R) reader structure"]
impl crate::Readable for IC_DMA_RDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_rdlr::W`](W) writer structure"]
impl crate::Writable for IC_DMA_RDLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_DMA_RDLR to value 0"]
impl crate::Resettable for IC_DMA_RDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
