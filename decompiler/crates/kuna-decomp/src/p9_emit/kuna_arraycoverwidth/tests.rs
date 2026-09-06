//! (kuna) `arraycoverwidth` option + width-predicate tests.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use kuna_base::types::int4;

use super::*;

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

#[test]
fn apply_parses_on_off_and_rejects_garbage() {
    let o = OptionArrayCoverWidth;
    assert!(o.apply("on").unwrap().0);
    assert!(!o.apply("off").unwrap().0);
    assert!(o.apply("maybe").is_err());
}

#[test]
fn element_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_ARRAYCOVERWIDTH.get_id(), 4145);
    assert_eq!(ELEM_ARRAYCOVERWIDTH.get_name(), "arraycoverwidth");
}

#[test]
fn spans_multiple_elements_only_for_a_wide_array_cover() {
    let f = factory();
    // char[16] (the VM register bank): a 16-byte movaps cover spans 16 elements,
    // a 2-byte access spans 2, and a 1-byte access fits its element.
    let arr16 = f
        .get_type_array(16, base(1, type_metatype::TYPE_INT))
        .expect("char[16]");
    assert!(spans_multiple_elements(&arr16, 16));
    assert!(spans_multiple_elements(&arr16, 2));
    assert!(!spans_multiple_elements(&arr16, 1));

    // int4[4]: a 4-byte access fits one element; 8 and 16 do not.
    let arr4x4 = f
        .get_type_array(4, base(4, type_metatype::TYPE_INT))
        .expect("int4[4]");
    assert!(!spans_multiple_elements(&arr4x4, 4));
    assert!(spans_multiple_elements(&arr4x4, 8));
    assert!(spans_multiple_elements(&arr4x4, 16));

    // A scalar leaf is never a wide array cover, whatever width is asked for.
    assert!(!spans_multiple_elements(&base(4, type_metatype::TYPE_INT), 16));
}
