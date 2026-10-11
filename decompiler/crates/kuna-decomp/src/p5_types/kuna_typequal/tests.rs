use super::*;
use crate::dtype::{type_metatype, TypeFactory, TypeFactoryImpl};
use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};

#[test]
fn qualifiers_intern_and_roundtrip_at_each_pointer_level() {
    let factory = TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    factory
        .set_core_type("uint4", 4, type_metatype::TYPE_UINT, false)
        .unwrap();
    factory
        .set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false)
        .unwrap();
    factory.cache_core_types().unwrap();
    let scalar = factory.find_by_name("uint4").unwrap().unwrap();
    let qualified = factory
        .get_qualified_type(scalar.clone(), VOLATILE)
        .unwrap();
    let pointer = factory.get_type_pointer(4, scalar.clone(), 1).unwrap();
    let pointee = factory.get_type_pointer(4, qualified.clone(), 1).unwrap();
    let volatile_pointer = factory
        .get_qualified_type(pointer.clone(), VOLATILE)
        .unwrap();
    assert!(qualified.type_order(&scalar).unwrap() < 0);
    assert!(pointee.type_order(&pointer).unwrap() < 0);
    assert!(volatile_pointer.type_order(&pointer).unwrap() < 0);
    assert_ne!(volatile_pointer.compare(&pointee, 10).unwrap(), 0);
    let manager = kuna_base::space::AddrSpaceManager::new();
    for ty in [qualified, pointee.clone(), volatile_pointer.clone()] {
        let mut bytes = Vec::new();
        ty.encode_ref(&mut PackedEncode::new(&mut bytes)).unwrap();
        let mut decoder = PackedDecode::new(&manager);
        decoder.ingest_stream(&bytes).unwrap();
        let decoded = factory.decode_type(&mut decoder).unwrap();
        assert!(Rc::ptr_eq(&ty, &decoded));
    }
    let combined = factory
        .get_qualified_type(scalar.clone(), CONST | VOLATILE)
        .unwrap();
    let twice = factory
        .get_qualified_type(
            factory.get_qualified_type(scalar.clone(), CONST).unwrap(),
            VOLATILE,
        )
        .unwrap();
    assert!(Rc::ptr_eq(&combined, &twice));
    let alias = factory.get_typedef(&combined, "Register", 0, 0).unwrap();
    assert!(alias.type_order(&scalar).unwrap() < 0);
    assert!(Rc::ptr_eq(&unqualified_value(alias), &scalar));
    assert!(Rc::ptr_eq(&unqualified_value(pointer.clone()), &pointer));
    assert!(Rc::ptr_eq(&unqualified_value(volatile_pointer), &pointer));
    let object = factory.get_qualified_type(pointee.clone(), CONST).unwrap();
    assert!(Rc::ptr_eq(&unqualified_value(object), &pointee));
}
