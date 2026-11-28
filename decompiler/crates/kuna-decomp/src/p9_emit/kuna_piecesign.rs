//! (kuna) A read through a symbol's member keeps the signedness of the value
//! read.
//!
//! A variable that is one element or field of a larger symbol prints as the
//! member (`buf[5]`), and in C the member has the type the symbol declares
//! for it. The casts of the function were set against the variable's own
//! type, which differs when a temporary of the other signedness was merged
//! into it: an unsigned byte kept in a `char` array printed `buf[5] >> 4`
//! for a logical shift, which C evaluates as an arithmetic one. Such a read
//! is cast to the variable's type, as the truncating read of a member
//! already is (`PrintC::pushPartialSymbol`, the `finalcast` of
//! `printc.cc:2094`).

use crate::dtype::{type_metatype, Datatype};

/// Does a read of a member declared `member` yield the other signedness than
/// the integer type `value` of the variable printed through it?
pub(crate) fn changes_sign(value: &Datatype, member: &Datatype) -> bool {
    use type_metatype::{TYPE_INT, TYPE_UINT};
    let (v, m) = (value.get_metatype(), member.get_metatype());
    value.get_size() == member.get_size() && matches!((v, m), (TYPE_INT, TYPE_UINT) | (TYPE_UINT, TYPE_INT))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_integer_of_the_other_signedness_and_the_same_size_changes_sign() {
        use type_metatype::{TYPE_INT, TYPE_UINT, TYPE_UNKNOWN};
        let (schar, uchar) = (Datatype::new(1, TYPE_INT), Datatype::new(1, TYPE_UINT));
        let (uint, unknown) = (Datatype::new(4, TYPE_UINT), Datatype::new(1, TYPE_UNKNOWN));
        assert!(changes_sign(&uchar, &schar));
        assert!(changes_sign(&schar, &uchar));
        assert!(!changes_sign(&schar, &schar));
        assert!(!changes_sign(&uint, &schar));
        assert!(!changes_sign(&unknown, &schar));
        assert!(!changes_sign(&uchar, &unknown));
    }
}
