use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactoryImpl};
use kuna_decomp::varmap::ScopeLocal;
use kuna_decomp::varnode::varnode_flags;

fn space(name: &str, index: i32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        name,
        false,
        8,
        1,
        index,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn ty(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

#[test]
fn register_symbols_resolve_by_usepoint_for_name_and_type() {
    let mut scope = ScopeLocal::new(0x1234, space("stack", 4), "identity", 8).unwrap();
    let register = Address::new(space("register", 1), 0x18);
    let code = space("ram", 2);
    let first_use = Address::new(Rc::clone(&code), 0x100);
    let second_use = Address::new(Rc::clone(&code), 0x200);
    let unrelated_use = Address::new(Rc::clone(&code), 0x300);
    let first = scope
        .add_symbol(
            "$$undef00000000",
            ty(4, type_metatype::TYPE_INT),
            &register,
            &first_use,
        )
        .unwrap();
    let second = scope
        .add_symbol(
            "$$undef00000001",
            ty(4, type_metatype::TYPE_UINT),
            &register,
            &second_use,
        )
        .unwrap();
    scope.set_attribute(first, varnode_flags::typelock);
    scope.set_attribute(second, varnode_flags::typelock);

    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);

    let first_entry = scope
        .query_container_for_link(&register, &first_use)
        .unwrap();
    let second_entry = scope
        .query_container_for_link(&register, &second_use)
        .unwrap();
    assert_eq!(first_entry.symbol, first);
    assert_eq!(second_entry.symbol, second);
    assert_eq!(
        scope
            .build_localtype_seed_at(&register, 4, &first_use, &types)
            .unwrap()
            .get_metatype(),
        type_metatype::TYPE_INT
    );
    assert_eq!(
        scope
            .build_localtype_seed_at(&register, 4, &second_use, &types)
            .unwrap()
            .get_metatype(),
        type_metatype::TYPE_UINT
    );

    let mut base = 1;
    assert_eq!(
        scope
            .resolve_default_name_for_link(&first_entry, 4, &mut base, None)
            .unwrap()
            .0,
        "v1"
    );
    assert_eq!(
        scope
            .resolve_default_name_for_link(&second_entry, 4, &mut base, None)
            .unwrap()
            .0,
        "v2"
    );
    assert_eq!(scope.database().symbol(first).get_name(), "v1");
    assert_eq!(scope.database().symbol(second).get_name(), "v2");

    assert!(scope
        .query_container_for_link(&register, &unrelated_use)
        .is_none());
    assert!(scope
        .build_localtype_seed_at(&register, 4, &unrelated_use, &types)
        .is_none());

    let whole_function = Address::new(register.get_space().unwrap().clone(), 0x28);
    let unscoped = Address::new_invalid();
    let whole = scope
        .add_symbol(
            "whole",
            ty(4, type_metatype::TYPE_INT),
            &whole_function,
            &unscoped,
        )
        .unwrap();
    scope.set_attribute(whole, varnode_flags::typelock);
    let whole_entry = scope
        .query_container_for_link(&whole_function, &unrelated_use)
        .unwrap();
    assert_eq!(whole_entry.symbol, whole);
    assert_eq!(
        scope
            .resolve_default_name_for_link(&whole_entry, 4, &mut base, None)
            .unwrap()
            .0,
        "whole"
    );
    assert_eq!(
        scope
            .build_localtype_seed_at(&whole_function, 4, &unrelated_use, &types)
            .unwrap()
            .get_metatype(),
        type_metatype::TYPE_INT
    );
}
