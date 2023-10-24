#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JEDEC JEP-106 compliant chip identifier."]
    pub chip_id: CHIP_ID,
    #[doc = "0x04 - Platform register. Allows software to know what environment it is running in."]
    pub platform: PLATFORM,
    _reserved2: [u8; 0x38],
    #[doc = "0x40 - Git hash of the chip source. Used to identify chip version."]
    pub gitref_rp2040: GITREF_RP2040,
}
#[doc = "CHIP_ID (r) register accessor: JEDEC JEP-106 compliant chip identifier.  

You can [`read`](crate::generic::Reg::read) this register and get [`chip_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_id`]
module"]
pub type CHIP_ID = crate::Reg<chip_id::CHIP_ID_SPEC>;
#[doc = "JEDEC JEP-106 compliant chip identifier."]
pub mod chip_id;
#[doc = "PLATFORM (r) register accessor: Platform register. Allows software to know what environment it is running in.  

You can [`read`](crate::generic::Reg::read) this register and get [`platform::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@platform`]
module"]
pub type PLATFORM = crate::Reg<platform::PLATFORM_SPEC>;
#[doc = "Platform register. Allows software to know what environment it is running in."]
pub mod platform;
#[doc = "GITREF_RP2040 (r) register accessor: Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::generic::Reg::read) this register and get [`gitref_rp2040::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gitref_rp2040`]
module"]
pub type GITREF_RP2040 = crate::Reg<gitref_rp2040::GITREF_RP2040_SPEC>;
#[doc = "Git hash of the chip source. Used to identify chip version."]
pub mod gitref_rp2040;
