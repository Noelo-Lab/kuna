//! P-code values and their semantics, shared by both sides: `value` evaluates
//! an op, `x87` the 80-bit floats, `storage` names a varnode's place.

pub mod storage;
pub mod value;
pub mod x87;
