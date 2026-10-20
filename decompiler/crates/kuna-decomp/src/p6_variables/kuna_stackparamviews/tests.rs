use super::*;
use crate::context::ArchContext;
use crate::dtype::Datatype;
use crate::fspec::ParameterPieces;
use crate::merge::MergeContext;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace,
};

fn frame_data() -> (Funcdata, Rc<AddrSpace>) {
    frame_data_with_views(true)
}

fn frame_data_with_views(stack_views: bool) -> (Funcdata, Rc<AddrSpace>) {
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&register)).unwrap();
    let stack = Rc::new(SpacebaseSpace::new(
        "stack", 3, 8, &register, 1, true, false,
    ));
    manage.insert_space(Rc::clone(&stack)).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    let mut context = ArchContext::new(manage);
    context.stack_views = stack_views;
    let fd = Funcdata::new(
        "early_views",
        "early_views",
        Rc::new(context),
        Address::new(ram, 0x1000),
        0x1000_0000,
        0x40,
    )
    .unwrap();
    (fd, stack)
}

#[test]
fn native_parameter_store_extents_preserve_only_the_reused_bytes() {
    let (mut fd, stack) = frame_data_with_views(false);
    let address = Address::new(Rc::clone(&stack), 8);
    record_store(&mut fd, &address, 4);
    assert!(fd.stack_parameter_store_extents.is_empty());
    fd.get_func_proto_mut()
        .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
    fd.get_func_proto_mut().set_param(
        0,
        "flag",
        &ParameterPieces {
            addr: address.clone(),
            type_: Some(Rc::new(Datatype::new(1, type_metatype::TYPE_BOOL))),
            flags: 0,
        },
    );
    record_store(&mut fd, &address, 4);
    assert!(fd.stack_parameter_store_extents.is_empty());
    fd.get_func_proto_mut().set_input_lock(true);
    record_store(&mut fd, &address, 1);
    record_store(&mut fd, &Address::new(Rc::clone(&stack), 9), 4);
    assert!(fd.stack_parameter_store_extents.is_empty());
    record_store(&mut fd, &address, 4);
    assert_eq!(wider_scalar_parameters(&fd), vec![(8, 4)]);
    for (offset, size, expected) in [
        (7, 1, false),
        (7, 2, true),
        (8, 4, true),
        (9, 1, true),
        (11, 1, true),
        (12, 4, false),
    ] {
        assert_eq!(
            preserves_range(&fd, &Address::new(Rc::clone(&stack), offset), size),
            expected
        );
    }
    let register = fd
        .get_arch()
        .manage()
        .get_space_by_name("register")
        .unwrap()
        .clone();
    assert!(!preserves_range(&fd, &Address::new(register, 8), 4));
    record_store(&mut fd, &address, 8);
    assert!(wider_scalar_parameters(&fd).is_empty());
    assert!(!preserves_range(&fd, &address, 4));
    record_store(&mut fd, &address, 4);
    assert!(wider_scalar_parameters(&fd).is_empty());
    assert!(!preserves_range(&fd, &address, 8));
    fd.clear();
    assert!(fd.stack_parameter_store_extents.is_empty());
}

#[test]
fn native_parameter_store_extent_declines_another_incoming_parameter() {
    for (offset, size, allowed) in [(10, 2, false), (4, 8, false), (12, 4, true)] {
        let (mut fd, stack) = frame_data_with_views(false);
        let address = Address::new(Rc::clone(&stack), 8);
        fd.get_func_proto_mut()
            .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        for (index, addr, width, meta) in [
            (0, address.clone(), 1, type_metatype::TYPE_BOOL),
            (
                1,
                Address::new(stack, offset),
                size,
                type_metatype::TYPE_INT,
            ),
        ] {
            fd.get_func_proto_mut().set_param(
                index,
                "arg",
                &ParameterPieces {
                    addr,
                    type_: Some(Rc::new(Datatype::new(width, meta))),
                    flags: 0,
                },
            );
        }
        fd.get_func_proto_mut().set_input_lock(true);
        record_store(&mut fd, &address, 4);
        assert_eq!(!wider_scalar_parameters(&fd).is_empty(), allowed);
        assert_eq!(preserves_range(&fd, &address, 4), allowed);
        record_store(&mut fd, &address, 8);
        record_store(&mut fd, &address, 4);
        assert!(wider_scalar_parameters(&fd).is_empty());
        assert!(!preserves_range(&fd, &address, 4));
    }
}

#[test]
fn larger_native_stores_exclude_later_narrow_parameter_recovery() {
    for (offset, size) in [(8, 8), (8, 16), (8, 32), (9, 16), (4, 16)] {
        let (mut fd, stack) = frame_data_with_views(false);
        let address = Address::new(Rc::clone(&stack), 8);
        fd.get_func_proto_mut()
            .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        fd.get_func_proto_mut().set_param(
            0,
            "flag",
            &ParameterPieces {
                addr: address.clone(),
                type_: Some(Rc::new(Datatype::new(1, type_metatype::TYPE_BOOL))),
                flags: 0,
            },
        );
        fd.get_func_proto_mut().set_input_lock(true);
        record_store(&mut fd, &Address::new(stack, offset), size);
        record_store(&mut fd, &address, 4);
        assert!(
            wider_scalar_parameters(&fd).is_empty(),
            "store at {offset} size {size}"
        );
        assert!(!preserves_range(&fd, &address, 4));
    }
}

#[test]
fn early_backing_queries_keep_an_absent_parameter_contract_unknown() {
    let (mut fd, stack) = frame_data();
    let address = Address::new(stack, 8);
    let mut datatype = Datatype::new(8, type_metatype::TYPE_UNION);
    datatype.name = "stack_views_early".to_string();
    fd.get_scope_local_mut()
        .unwrap()
        .add_symbol(
            "backing",
            Rc::new(datatype),
            &address,
            &Address::new_invalid(),
        )
        .unwrap();
    let input = fd.new_varnode(4, &address, None);
    fd.set_input_varnode(input).unwrap();
    fd.set_high_level();

    assert!(!fd.get_func_proto().has_store());
    assert!(parameter_types(&fd, 8, 8).is_empty());
    assert!(backing_for_address(&fd, &address, 4).is_none());
    let _ = fd.addr_tied_ranges();
    assert!(!fd.get_func_proto().has_store());

    let key_type = Rc::new(Datatype::new(4, type_metatype::TYPE_UINT));
    fd.get_func_proto_mut()
        .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
    fd.get_func_proto_mut().set_param(
        0,
        "key",
        &ParameterPieces {
            addr: address.clone(),
            type_: Some(Rc::clone(&key_type)),
            flags: 0,
        },
    );
    let views = parameter_types(&fd, 8, 8);
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].0, 0);
    assert!(Rc::ptr_eq(&views[0].1, &key_type));
    assert!(backing_for_address(&fd, &address, 4).is_some());
}

#[test]
fn parameter_backing_declines_partial_local_mappings() {
    for (offset, width, allowed) in [(8, 1, false), (12, 4, false), (4, 8, false), (16, 4, true)] {
        let (mut fd, stack) = frame_data();
        let address = Address::new(Rc::clone(&stack), 8);
        let incoming = Rc::new(Datatype::new(1, type_metatype::TYPE_BOOL));
        fd.get_func_proto_mut()
            .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        fd.get_func_proto_mut().set_param(
            0,
            "flag",
            &ParameterPieces {
                addr: address.clone(),
                type_: Some(Rc::clone(&incoming)),
                flags: 0,
            },
        );
        let mut range = kuna_base::address::RangeList::new();
        range.insert_range(Rc::clone(&stack), 8, 31);
        let scope = fd.get_scope_local_mut().unwrap();
        scope.reset_local_window(&kuna_base::address::RangeList::new(), &range, true);
        scope
            .add_param_symbol(0, "flag", incoming, &address, &Address::new_invalid())
            .unwrap();
        assert!(can_map_backing(&fd, 8, 8));
        fd.get_scope_local_mut()
            .unwrap()
            .add_symbol(
                "other",
                Rc::new(Datatype::new(width, type_metatype::TYPE_UINT)),
                &Address::new(stack, offset),
                &Address::new_invalid(),
            )
            .unwrap();
        assert_eq!(
            can_map_backing(&fd, 8, 8),
            allowed,
            "local at {offset} width {width}"
        );
    }
}
