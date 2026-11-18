//! Fix family `kt_operators` (Kotlin output): operators: precedence, equality,
//! bitwise operators, numeric promotion.

pub(crate) mod kt_binary_numeric_promotion;
pub(crate) mod kt_compound_bitwise_assignment;
pub(crate) mod kt_copied_equality;
pub(crate) mod kt_implicit_widening;
pub(crate) mod kt_narrow_bitwise_operand;
pub(crate) mod kt_negated_equality;
pub(crate) mod kt_operator_line_break;
pub(crate) mod kt_operator_precedence;
pub(crate) mod kt_reference_inequality;
