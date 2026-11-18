//! Fix family `overloads`: call arguments that pick an overload: varargs,
//! implicit conversions, casts. In this file, `vararg_array_length` spreads a
//! varargs array only when it is created with an initializer or a constant
//! length 0.

pub(crate) mod ambiguous_type_variable_argument;
pub(crate) mod unseen_class_overload;
pub(crate) mod vararg_spread_resolution;

use crate::new_exprent::NewExprent;

/// Whether writing `array`'s elements in its place keeps the call's meaning.
pub fn spreads_as_elements(array: NewExprent) -> bool {
    if !array.get_lst_array_elements().is_empty() {
        return true;
    }
    let dims = array.get_lst_dims();
    dims.is_empty()
        || dims
            .get(0)
            .try_as_const()
            .is_some_and(|c| c.get_int_value() == 0)
}
