use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace,
    SpacebaseSpace, UniqueSpace, VarnodeStorage,
};
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

#[test]
fn register_type_seeds_select_the_whole_requested_storage_width() {
    let mut scope = ScopeLocal::new(0x1234, space("stack", 4), "widths", 8).unwrap();
    let register = Address::new(space("register", 1), 0x18);
    let usepoint = Address::new(space("ram", 2), 0x100);
    let wide = scope
        .add_symbol("wide", ty(8, type_metatype::TYPE_INT), &register, &usepoint)
        .unwrap();
    let narrow = scope
        .add_symbol(
            "narrow",
            ty(4, type_metatype::TYPE_UINT),
            &register,
            &usepoint,
        )
        .unwrap();
    scope.set_attribute(wide, varnode_flags::typelock);
    scope.set_attribute(narrow, varnode_flags::typelock);

    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);

    let wide_type = scope
        .build_localtype_seed_at(&register, 8, &usepoint, &types)
        .expect("the narrow entry must not hide the independently asserted wide type");
    assert_eq!(wide_type.get_size(), 8);
    assert_eq!(wide_type.get_metatype(), type_metatype::TYPE_INT);
    let narrow_type = scope
        .build_localtype_seed_at(&register, 4, &usepoint, &types)
        .unwrap();
    assert_eq!(narrow_type.get_size(), 4);
    assert_eq!(narrow_type.get_metatype(), type_metatype::TYPE_UINT);
    assert!(scope
        .build_localtype_seed_at(&register, 16, &usepoint, &types)
        .is_none());
}

#[test]
fn full_width_type_seeds_preserve_wrapped_stack_precision_entries() {
    for big in [false, true] {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            big,
            4,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager
            .insert_space(Rc::new(JoinSpace::new(3, big)))
            .unwrap();
        let stack = Rc::new(SpacebaseSpace::new("stack", 4, 4, &ram, 1, true, big));
        manager.insert_space(Rc::clone(&stack)).unwrap();
        let mut pieces = [
            VarnodeStorage {
                space: Some(Rc::clone(&stack)),
                offset: 0xffff_fffc,
                size: 4,
            },
            VarnodeStorage {
                space: Some(Rc::clone(&stack)),
                offset: 0,
                size: 4,
            },
        ];
        if !big {
            pieces.reverse();
        }
        let join = manager.find_add_join(&pieces, 0).unwrap();
        let mut scope = ScopeLocal::new(0x1234, Rc::clone(&stack), "wrapped", 5).unwrap();
        let invalid = Address::new_invalid();
        let symbol = scope
            .add_symbol(
                "wrapped_word",
                ty(8, type_metatype::TYPE_UINT),
                &join.get_unified().get_addr(),
                &invalid,
            )
            .unwrap();
        scope.set_attribute(symbol, varnode_flags::typelock);
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        let start = Address::new(stack, 0xffff_fffc);

        assert!(scope
            .query_container_for_link_width(&start, 8, &invalid)
            .is_none());
        let dtype = scope
            .build_localtype_seed_at(&start, 8, &invalid, &types)
            .expect("a wrapped scalar uses its whole symbol type through a precision entry");
        assert_eq!(dtype.get_size(), 8);
        assert_eq!(dtype.get_metatype(), type_metatype::TYPE_UINT);
    }
}
