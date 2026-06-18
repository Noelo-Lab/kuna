//! Adversarial tests for the usepoint-scoped register-Symbol binding that backs
//! the Return Structure cluster (retstruct's `type varnode %EAX(0x100010) int4
//! tmp` directive).
//!
//! C++ `Funcdata::linkSymbol` (`funcdata_varnode.cc:1190`) queries the local map
//! with the Varnode's use address:
//!     queryProperties(vn->getAddr(), 1, vn->getUsePoint(*this), fl) -> findContainer
//! and `findContainer` consults `SymbolEntry::inUse` (`database.cc:115`).  A
//! register-storage local whose `uselimit` is a single code address (the
//! directive's usepoint) is only "in use" at that address, so it binds the
//! register read scoped to it; an addr-tied / empty-`uselimit` Symbol is in use
//! everywhere (usepoint-independent).
//!
//! These pin that semantics through the public `Database` API so the change is
//! self-standing against the C++ oracle.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::database::{Database, ScopeId};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::varnode::varnode_flags;

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// const(0), unique(1), ram(2, 8-byte code space), register(3, 4-byte).
fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        0,
        0,
        0,
    )))
    .unwrap();
    m.set_default_code_space(2).unwrap();
    m
}

fn space(m: &AddrSpaceManager, idx: i32) -> Rc<AddrSpace> {
    Rc::clone(m.get_space(idx).unwrap())
}

fn db_with_global(num_spaces: int4) -> (Database, ScopeId) {
    let mut db = Database::new(true);
    let g = db.find_create_scope(0, "", None, num_spaces).unwrap();
    (db, g)
}

// ---------------------------------------------------------------------------
// AT1 (the core regression): a register Symbol created with a usepoint is
// `inUse` ONLY at a usepoint within its range — `findContainer` finds it at the
// scoped use address and NOT at an invalid usepoint.  This is exactly the
// retstruct `tmp` binding (`%EAX(0x100010)`): with an invalid usepoint the EAX
// read keeps the auto-name `vN`; with the real use address it renders `tmp`.
// ---------------------------------------------------------------------------
#[test]
fn at1_usepoint_register_symbol_found_only_at_its_usepoint() {
    let m = build_manager();
    let reg = space(&m, 3);
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());

    let eax = Address::new(Rc::clone(&reg), 0); // %EAX == register:0
    let pc = Address::new(Rc::clone(&ram), 0x100010); // the directive's use addr
    db.add_symbol_mapped(g, "tmp", dt(4), &eax, &pc).unwrap();

    // With the REAL usepoint the register Symbol is in use -> found.
    let e = db
        .find_container(g, &eax, 1, &pc)
        .expect("tmp must bind at its usepoint");
    assert_eq!(db.symbol(db.entry(g, e).symbol).name, "tmp");

    // With an INVALID usepoint (the pre-fix query) `inUse` is false for a
    // non-addr-tied, non-empty-uselimit entry -> NOT found.  This is precisely
    // why threading the real usepoint is load-bearing.
    let inv = Address::new_invalid();
    assert!(
        db.find_container(g, &eax, 1, &inv).is_none(),
        "an invalid usepoint must NOT find the usepoint-scoped register Symbol"
    );

    // And at a code address OUTSIDE the uselimit range it is also not in use.
    let other_pc = Address::new(Rc::clone(&ram), 0x100000);
    assert!(
        db.find_container(g, &eax, 1, &other_pc).is_none(),
        "outside its uselimit the register Symbol is not in use"
    );
}

// ---------------------------------------------------------------------------
// AT2: an addr-tied / empty-uselimit Symbol (the ordinary mapped local — added
// with an INVALID usepoint, which `addMap` turns addr-tied) is `inUse` at EVERY
// usepoint, so threading a real usepoint can never lose it.  This guards the
// no-regression half: the change is a pure ADDITION (usepoint-scoped entries
// become findable) and cannot drop any pre-existing addr-tied binding.
// ---------------------------------------------------------------------------
#[test]
fn at2_addrtied_symbol_found_regardless_of_usepoint() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());

    let loc = Address::new(Rc::clone(&ram), 0x4000);
    let (sym, _) = db
        .add_symbol_mapped(g, "g", dt(4), &loc, &Address::new_invalid())
        .unwrap();
    // An empty-uselimit mapped entry is marked addr-tied.
    assert!(
        (db.symbol(sym).flags & varnode_flags::addrtied) != 0,
        "empty-uselimit mapped symbol must be addr-tied"
    );

    // Found with an invalid usepoint AND with an arbitrary real one — identical.
    let inv = Address::new_invalid();
    let any_pc = Address::new(Rc::clone(&ram), 0x123456);
    assert_eq!(
        db.symbol(db.entry(g, db.find_container(g, &loc, 1, &inv).unwrap()).symbol).name,
        "g"
    );
    assert_eq!(
        db.symbol(db.entry(g, db.find_container(g, &loc, 1, &any_pc).unwrap()).symbol).name,
        "g"
    );
}

// ---------------------------------------------------------------------------
// AT3: the carry-across collector `scope_usepoint_symbol_specs` snapshots EXACTLY
// the usepoint-scoped (register, non-addr-tied) Symbols WITH their use address,
// and excludes the addr-tied stack symbols (which `scope_space_symbol_specs`
// already carries).  This is what makes the console `tmp` survive the IR rebuild.
// ---------------------------------------------------------------------------
#[test]
fn at3_usepoint_specs_carry_register_symbol_with_use_address() {
    let m = build_manager();
    let reg = space(&m, 3);
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());

    let eax = Address::new(Rc::clone(&reg), 0);
    let pc = Address::new(Rc::clone(&ram), 0x100010);
    db.add_symbol_mapped(g, "tmp", dt(4), &eax, &pc).unwrap();
    // An addr-tied stack local — must NOT appear in the usepoint specs.
    db.add_symbol_mapped(
        g,
        "g",
        dt(4),
        &Address::new(Rc::clone(&ram), 0x4000),
        &Address::new_invalid(),
    )
    .unwrap();

    let specs = db.scope_usepoint_symbol_specs(g);
    assert_eq!(specs.len(), 1, "exactly the usepoint-scoped tmp is carried");
    let (name, _ct, addr, _flags, usepoint) = &specs[0];
    assert_eq!(name, "tmp");
    // Carried at its register storage AND its use address (so the re-seed can
    // restore the uselimit and `inUse` matches at the same read).
    assert_eq!(addr.get_offset(), 0);
    assert_eq!(addr.get_space().unwrap().get_name(), "register");
    assert_eq!(usepoint.get_offset(), 0x100010);
    assert_eq!(usepoint.get_space().unwrap().get_name(), "ram");
}
