use super::*;
use crate::dtype::Datatype;

#[test]
fn code_pointee_is_blocked() {
    let code = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
    assert!(blocks_value_type(&code));
}

#[test]
fn ordinary_pointees_are_untouched() {
    for meta in [
        type_metatype::TYPE_UNKNOWN,
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT,
        type_metatype::TYPE_PTR,
        type_metatype::TYPE_STRUCT,
    ] {
        let t = Datatype::new(1, meta);
        assert!(!blocks_value_type(&t), "{meta:?} must not be blocked");
    }
}
