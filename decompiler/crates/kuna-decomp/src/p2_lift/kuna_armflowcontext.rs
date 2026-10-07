//! The packed ARM ISA field preserved at an ordinary flow boundary.

#[derive(Clone, Copy)]
pub struct ArmDecodeMode {
    pub word: usize,
    pub mask: u32,
    pub value: u32,
    pub last: u64,
}
