//! Unit tests for the `paramrefdecl` Symbol-identity test.
//!
//! The frame modelled is the witness's: an i386 cdecl stack parameter at
//! `[EBP + 8]` whose address the body takes, with a plain local behind it.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};

use crate::dtype::{type_metatype, TypeFactory, TypeFactoryImpl};

use super::*;

fn manager() -> Rc<AddrSpaceManager> {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    Rc::new(m)
}

fn types_with_core() -> Rc<TypeFactoryImpl> {
    let t = TypeFactoryImpl::new();
    t.setup_sizes(Some(4), 4, 4);
    t.set_core_type("void", 1, type_metatype::TYPE_VOID, false).unwrap();
    t.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    t.cache_core_types().unwrap();
    Rc::new(t)
}

fn ram(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("ram").unwrap())
}

/// A scope holding a cat-0 parameter at frame offset 4 and a `no_category` local
/// at frame offset -0x10, plus the two symbol ids.
fn scope_with_param_and_local() -> (ScopeLocal, Rc<AddrSpace>, SymbolId, SymbolId) {
    let m = manager();
    let types = types_with_core();
    let int4_t = TypeFactory::find_by_name(types.as_ref(), "int4").unwrap().unwrap();
    let mut lm = ScopeLocal::new(0x401571, ram(&m), "sub_401571", m.num_spaces()).unwrap();
    let param_addr = Address::new(ram(&m), 4);
    let usepoint = Address::new(ram(&m), 0x401570);
    let param = lm
        .add_param_symbol(0, "a0", Rc::clone(&int4_t), &param_addr, &usepoint)
        .unwrap()
        .expect("param created");
    let local_addr = Address::new(ram(&m), 0x100);
    let local = lm
        .add_symbol("v7", Rc::clone(&int4_t), &local_addr, &Address::new_invalid())
        .unwrap();
    (lm, ram(&m), param, local)
}

#[test]
fn parameter_symbol_is_recognized_and_a_local_is_not() {
    let (lm, _space, param, local) = scope_with_param_and_local();
    assert!(is_parameter_symbol(&lm, param));
    assert!(!is_parameter_symbol(&lm, local));
}

/// The storage query the JSON surface uses: the parameter's own slot resolves to
/// the parameter Symbol, the local's slot resolves to nothing (it is not a
/// parameter), and unclaimed storage resolves to nothing.
#[test]
fn parameter_symbol_at_resolves_only_the_parameter_slot() {
    let (lm, space, param, _local) = scope_with_param_and_local();
    assert_eq!(parameter_symbol_at(&lm, &Address::new(Rc::clone(&space), 4)), Some(param));
    assert_eq!(parameter_symbol_at(&lm, &Address::new(Rc::clone(&space), 0x100)), None);
    assert_eq!(parameter_symbol_at(&lm, &Address::new(space, 0x800)), None);
}
