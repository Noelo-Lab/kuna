//! kuna fix `vararg_array_length` (`kuna_fixes`): `InvocationExprent.toJava`
//! writes the array a varargs call passes last as its elements, so
//! `asList(new T[]{a, b})` is `asList(a, b)`. It does so for any `new T[..]`,
//! and an array created by length alone has no elements to write:
//! `asList(new T[n])` (n nulls) came out `asList()` (none). With the fix the
//! array is spread only when its elements are what it holds: an initializer,
//! or a length of constant 0 (javac's code for a call passing no varargs).

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
