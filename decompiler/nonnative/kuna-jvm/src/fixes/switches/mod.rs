//! Fix family `switches` (Java output): switch statements and expressions: case
//! order and labels, javac's default, yielded values.

pub(crate) mod switch_case_order;
pub(crate) mod switch_expression_assignments;
pub(crate) mod switch_expression_poly_cast;
pub(crate) mod switch_expression_return;
pub(crate) mod switch_synthetic_default;
pub(crate) mod switch_table_fillers;
pub(crate) mod switch_yield_const_retype;
