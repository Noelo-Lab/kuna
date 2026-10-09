use super::*;
use crate::context::ArchContext;
use crate::dtype::Datatype;
use crate::fspec::ParameterPieces;
use crate::merge::MergeContext;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace,
};

#[test]
fn early_backing_queries_keep_an_absent_parameter_contract_unknown() {
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
    context.stack_views = true;
    let mut fd = Funcdata::new(
        "early_views",
        "early_views",
        Rc::new(context),
        Address::new(ram, 0x1000),
        0x1000_0000,
        0x40,
    )
    .unwrap();
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
