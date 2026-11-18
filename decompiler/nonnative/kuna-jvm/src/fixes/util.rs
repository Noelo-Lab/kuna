//! Helpers several fix families share.

use crate::code_type::CodeType;

/// Java's binary numeric promotion of two written numeric types.
pub fn promoted(a: CodeType, b: CodeType) -> CodeType {
    for wide in [CodeType::Double, CodeType::Float, CodeType::Long] {
        if a == wide || b == wide {
            return wide;
        }
    }
    CodeType::Int
}
