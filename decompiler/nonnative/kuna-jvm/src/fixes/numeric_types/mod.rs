//! Fix family `numeric_types` (Java output): the types of numeric and boolean
//! constants and operands, widening and boxing.

pub(crate) mod compound_assignment_shape;
pub(crate) mod concat_operand_type;
pub(crate) mod implicit_widening_arithmetic;
pub(crate) mod mixed_type_reassociation;
pub(crate) mod reference_unboxing;
pub(crate) mod ternary_numeric_arms;
