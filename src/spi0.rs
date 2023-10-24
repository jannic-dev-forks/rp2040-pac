#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0, SSPCR0 on page 3-4"]
    pub sspcr0: SSPCR0,
    #[doc = "0x04 - Control register 1, SSPCR1 on page 3-5"]
    pub sspcr1: SSPCR1,
    #[doc = "0x08 - Data register, SSPDR on page 3-6"]
    pub sspdr: SSPDR,
    #[doc = "0x0c - Status register, SSPSR on page 3-7"]
    pub sspsr: SSPSR,
    #[doc = "0x10 - Clock prescale register, SSPCPSR on page 3-8"]
    pub sspcpsr: SSPCPSR,
    #[doc = "0x14 - Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub sspimsc: SSPIMSC,
    #[doc = "0x18 - Raw interrupt status register, SSPRIS on page 3-10"]
    pub sspris: SSPRIS,
    #[doc = "0x1c - Masked interrupt status register, SSPMIS on page 3-11"]
    pub sspmis: SSPMIS,
    #[doc = "0x20 - Interrupt clear register, SSPICR on page 3-11"]
    pub sspicr: SSPICR,
    #[doc = "0x24 - DMA control register, SSPDMACR on page 3-12"]
    pub sspdmacr: SSPDMACR,
    _reserved10: [u8; 0x0fb8],
    #[doc = "0xfe0 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid0: SSPPERIPHID0,
    #[doc = "0xfe4 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid1: SSPPERIPHID1,
    #[doc = "0xfe8 - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid2: SSPPERIPHID2,
    #[doc = "0xfec - Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub sspperiphid3: SSPPERIPHID3,
    #[doc = "0xff0 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid0: SSPPCELLID0,
    #[doc = "0xff4 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid1: SSPPCELLID1,
    #[doc = "0xff8 - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid2: SSPPCELLID2,
    #[doc = "0xffc - PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub ssppcellid3: SSPPCELLID3,
}
#[doc = "SSPCR0 (rw) register accessor: Control register 0, SSPCR0 on page 3-4  

You can [`read`](crate::generic::Reg::read) this register and get [`sspcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspcr0`]
module"]
pub type SSPCR0 = crate::Reg<sspcr0::SSPCR0_SPEC>;
#[doc = "Control register 0, SSPCR0 on page 3-4"]
pub mod sspcr0;
#[doc = "SSPCR1 (rw) register accessor: Control register 1, SSPCR1 on page 3-5  

You can [`read`](crate::generic::Reg::read) this register and get [`sspcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspcr1`]
module"]
pub type SSPCR1 = crate::Reg<sspcr1::SSPCR1_SPEC>;
#[doc = "Control register 1, SSPCR1 on page 3-5"]
pub mod sspcr1;
#[doc = "SSPDR (rw) register accessor: Data register, SSPDR on page 3-6  

You can [`read`](crate::generic::Reg::read) this register and get [`sspdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspdr`]
module"]
pub type SSPDR = crate::Reg<sspdr::SSPDR_SPEC>;
#[doc = "Data register, SSPDR on page 3-6"]
pub mod sspdr;
#[doc = "SSPSR (r) register accessor: Status register, SSPSR on page 3-7  

You can [`read`](crate::generic::Reg::read) this register and get [`sspsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspsr`]
module"]
pub type SSPSR = crate::Reg<sspsr::SSPSR_SPEC>;
#[doc = "Status register, SSPSR on page 3-7"]
pub mod sspsr;
#[doc = "SSPCPSR (rw) register accessor: Clock prescale register, SSPCPSR on page 3-8  

You can [`read`](crate::generic::Reg::read) this register and get [`sspcpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspcpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspcpsr`]
module"]
pub type SSPCPSR = crate::Reg<sspcpsr::SSPCPSR_SPEC>;
#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
pub mod sspcpsr;
#[doc = "SSPIMSC (rw) register accessor: Interrupt mask set or clear register, SSPIMSC on page 3-9  

You can [`read`](crate::generic::Reg::read) this register and get [`sspimsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspimsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspimsc`]
module"]
pub type SSPIMSC = crate::Reg<sspimsc::SSPIMSC_SPEC>;
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
pub mod sspimsc;
#[doc = "SSPRIS (r) register accessor: Raw interrupt status register, SSPRIS on page 3-10  

You can [`read`](crate::generic::Reg::read) this register and get [`sspris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspris`]
module"]
pub type SSPRIS = crate::Reg<sspris::SSPRIS_SPEC>;
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
pub mod sspris;
#[doc = "SSPMIS (r) register accessor: Masked interrupt status register, SSPMIS on page 3-11  

You can [`read`](crate::generic::Reg::read) this register and get [`sspmis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspmis`]
module"]
pub type SSPMIS = crate::Reg<sspmis::SSPMIS_SPEC>;
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
pub mod sspmis;
#[doc = "SSPICR (rw) register accessor: Interrupt clear register, SSPICR on page 3-11  

You can [`read`](crate::generic::Reg::read) this register and get [`sspicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspicr`]
module"]
pub type SSPICR = crate::Reg<sspicr::SSPICR_SPEC>;
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
pub mod sspicr;
#[doc = "SSPDMACR (rw) register accessor: DMA control register, SSPDMACR on page 3-12  

You can [`read`](crate::generic::Reg::read) this register and get [`sspdmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspdmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspdmacr`]
module"]
pub type SSPDMACR = crate::Reg<sspdmacr::SSPDMACR_SPEC>;
#[doc = "DMA control register, SSPDMACR on page 3-12"]
pub mod sspdmacr;
#[doc = "SSPPERIPHID0 (r) register accessor: Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspperiphid0`]
module"]
pub type SSPPERIPHID0 = crate::Reg<sspperiphid0::SSPPERIPHID0_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid0;
#[doc = "SSPPERIPHID1 (r) register accessor: Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspperiphid1`]
module"]
pub type SSPPERIPHID1 = crate::Reg<sspperiphid1::SSPPERIPHID1_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid1;
#[doc = "SSPPERIPHID2 (r) register accessor: Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspperiphid2`]
module"]
pub type SSPPERIPHID2 = crate::Reg<sspperiphid2::SSPPERIPHID2_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid2;
#[doc = "SSPPERIPHID3 (r) register accessor: Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sspperiphid3`]
module"]
pub type SSPPERIPHID3 = crate::Reg<sspperiphid3::SSPPERIPHID3_SPEC>;
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
pub mod sspperiphid3;
#[doc = "SSPPCELLID0 (r) register accessor: PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssppcellid0`]
module"]
pub type SSPPCELLID0 = crate::Reg<ssppcellid0::SSPPCELLID0_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid0;
#[doc = "SSPPCELLID1 (r) register accessor: PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssppcellid1`]
module"]
pub type SSPPCELLID1 = crate::Reg<ssppcellid1::SSPPCELLID1_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid1;
#[doc = "SSPPCELLID2 (r) register accessor: PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssppcellid2`]
module"]
pub type SSPPCELLID2 = crate::Reg<ssppcellid2::SSPPCELLID2_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid2;
#[doc = "SSPPCELLID3 (r) register accessor: PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

You can [`read`](crate::generic::Reg::read) this register and get [`ssppcellid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ssppcellid3`]
module"]
pub type SSPPCELLID3 = crate::Reg<ssppcellid3::SSPPCELLID3_SPEC>;
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
pub mod ssppcellid3;
