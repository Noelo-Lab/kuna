use super::*;
use crate::dtype::DatatypeKind;
use crate::fspec::FuncProto;

fn pointer(to: Rc<Datatype>) -> Datatype {
    let mut ty = Datatype::new(8, type_metatype::TYPE_PTR);
    ty.kind = DatatypeKind::Pointer { ptrto: to, spaceid: None, truncate: None, wordsize: 1 };
    ty
}

fn code(proto: Option<FuncProto>) -> Rc<Datatype> {
    let mut ty = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
    ty.kind = DatatypeKind::Code { proto: proto.map(Rc::new) };
    Rc::new(ty)
}

/// Only a pointer to the prototype-less `code` or to `void` is a target the
/// printer neither calls nor casts.
#[test]
fn uncallable_pointers_are_generic_code_and_void() {
    assert!(uncallable_pointer(&pointer(code(None))));
    assert!(uncallable_pointer(&pointer(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)))));
    assert!(!uncallable_pointer(&pointer(code(Some(FuncProto::new())))));
    assert!(!uncallable_pointer(&pointer(Rc::new(Datatype::new(8, type_metatype::TYPE_UINT)))));
    assert!(!uncallable_pointer(&Datatype::new(8, type_metatype::TYPE_UINT)));
}

#[test]
fn a_function_pointer_carries_a_prototype() {
    assert!(function_pointer(&pointer(code(Some(FuncProto::new())))));
    assert!(!function_pointer(&pointer(code(None))));
    assert!(!function_pointer(&code(Some(FuncProto::new()))));
}

/// A parameter or return type must have the value's width and a C spelling.
#[test]
fn spellable_types_have_the_width_and_a_c_name() {
    assert!(spellable(&Datatype::new(4, type_metatype::TYPE_INT), 4));
    assert!(spellable(&Datatype::new(8, type_metatype::TYPE_UNKNOWN), 8));
    assert!(spellable(&Datatype::new(10, type_metatype::TYPE_FLOAT), 10));
    assert!(spellable(&pointer(code(None)), 8));
    assert!(!spellable(&Datatype::new(4, type_metatype::TYPE_INT), 8));
    assert!(!spellable(&Datatype::new(3, type_metatype::TYPE_UNKNOWN), 3));
    assert!(!spellable(&Datatype::new(6, type_metatype::TYPE_FLOAT), 6));
    assert!(!spellable(&Datatype::new(0, type_metatype::TYPE_VOID), 0));
    assert!(!spellable(&code(None), 1));
    assert!(!spellable(&Datatype::new(8, type_metatype::TYPE_ARRAY), 8));
}

/// A library callback slot keeps its header's own function-pointer type, so a
/// local handed to one is not retyped.
#[test]
fn library_callback_slots_are_known() {
    use crate::kuna_callbacktype::is_callback_slot;
    assert!(is_callback_slot("qsort", 3));
    assert!(is_callback_slot("signal", 1));
    assert!(!is_callback_slot("qsort", 0));
    assert!(!is_callback_slot("memcpy", 0));
}
