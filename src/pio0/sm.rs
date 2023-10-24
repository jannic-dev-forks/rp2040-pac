#[doc = r"Register block"]
#[repr(C)]
pub struct SM {
    #[doc = "0x00 - Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub sm_clkdiv: SM_CLKDIV,
    #[doc = "0x04 - Execution/behavioural settings for state machine 0"]
    pub sm_execctrl: SM_EXECCTRL,
    #[doc = "0x08 - Control behaviour of the input/output shift registers for state machine 0"]
    pub sm_shiftctrl: SM_SHIFTCTRL,
    #[doc = "0x0c - Current instruction address of state machine 0"]
    pub sm_addr: SM_ADDR,
    #[doc = "0x10 - Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub sm_instr: SM_INSTR,
    #[doc = "0x14 - State machine pin control"]
    pub sm_pinctrl: SM_PINCTRL,
}
#[doc = "SM_CLKDIV (rw) register accessor: Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_clkdiv`]
module"]
pub type SM_CLKDIV = crate::Reg<sm_clkdiv::SM_CLKDIV_SPEC>;
#[doc = "Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
pub mod sm_clkdiv;
#[doc = "SM_EXECCTRL (rw) register accessor: Execution/behavioural settings for state machine 0  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_execctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_execctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_execctrl`]
module"]
pub type SM_EXECCTRL = crate::Reg<sm_execctrl::SM_EXECCTRL_SPEC>;
#[doc = "Execution/behavioural settings for state machine 0"]
pub mod sm_execctrl;
#[doc = "SM_SHIFTCTRL (rw) register accessor: Control behaviour of the input/output shift registers for state machine 0  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_shiftctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_shiftctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_shiftctrl`]
module"]
pub type SM_SHIFTCTRL = crate::Reg<sm_shiftctrl::SM_SHIFTCTRL_SPEC>;
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
pub mod sm_shiftctrl;
#[doc = "SM_ADDR (r) register accessor: Current instruction address of state machine 0  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_addr`]
module"]
pub type SM_ADDR = crate::Reg<sm_addr::SM_ADDR_SPEC>;
#[doc = "Current instruction address of state machine 0"]
pub mod sm_addr;
#[doc = "SM_INSTR (rw) register accessor: Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution.  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_instr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_instr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_instr`]
module"]
pub type SM_INSTR = crate::Reg<sm_instr::SM_INSTR_SPEC>;
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution."]
pub mod sm_instr;
#[doc = "SM_PINCTRL (rw) register accessor: State machine pin control  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_pinctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_pinctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sm_pinctrl`]
module"]
pub type SM_PINCTRL = crate::Reg<sm_pinctrl::SM_PINCTRL_SPEC>;
#[doc = "State machine pin control"]
pub mod sm_pinctrl;
