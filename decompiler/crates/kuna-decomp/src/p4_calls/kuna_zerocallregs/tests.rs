//! Unit tests for the zeroing-epilogue return rule.

use super::*;

#[test]
fn flag_ops_are_comparisons() {
    assert!(is_flag_op(OpCode::CPUI_INT_SLESS));
    assert!(is_flag_op(OpCode::CPUI_INT_EQUAL));
    assert!(!is_flag_op(OpCode::CPUI_COPY));
    assert!(!is_flag_op(OpCode::CPUI_INT_ADD));
}
