//! INDEPENDENT VERIFIER adversarial tests (round 1) — item: w10-symbol-consolidate.
//!
//! Structurally separate from the porter's `*_adversarial.rs` (which covers
//! `name_for_varnode`).  These target the OTHER faithfulness-fragile halves of
//! the global query that the porter's suite does not exercise:
//!
//!   V1. `query_properties` SYMBOL branch — a covering Symbol's `getAllFlags()`
//!       is returned verbatim (C++ `Scope::queryProperties` `res != 0` arm,
//!       database.cc:1274-1275).
//!   V2. `query_properties` OWNED-SCOPE branch — no covering Symbol but the
//!       global scope owns the range → `mapped|addrtied|persist|getProperty`
//!       (C++ database.cc:1276-1282).  This is the keystone that makes a bare
//!       global store survive `ActionDeadCode`.
//!   V3. `query_properties` FLAGBASE-ONLY branch — no Symbol, not owned → just
//!       the range's boolean properties (C++ database.cc:1283-1284), AND the
//!       constant-address guard returns 0 (`stackContainer` `isConstant` skip).
//!   V4. `find_container_flags` smallest-size selection returns the SMALLER
//!       entry's flags (not the larger covering entry's) — the `oldsize` walk,
//!       independent of which name wins.
//!   V5. `sized_type_geometry` typelock gate — a covering Symbol forces its type
//!       ONLY when type-locked (`updateType`'s `symbol->getFlags()&typelock`,
//!       database.cc:139); a non-typelocked covering Symbol returns `None`
//!       (inference stays free).  Plus the `end = addr+size-1` last-byte
//!       containment boundary is inclusive.
//!
//! All assertions are derived from the C++ bodies in scope, not the Rust output.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::space::{spacetype, AddrSpace, ConstantSpace};
use kuna_decomp::dtype::Datatype;
use kuna_decomp::context::{GlobalEntry, GlobalQuery};
use kuna_decomp::varnode::varnode_flags;

fn ram_space(index: i32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        index,
        0,
        0,
        0,
    ))
}

fn the_constant_space() -> Rc<AddrSpace> {
    Rc::new(ConstantSpace::new())
}

fn addr(space: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(space.clone(), off)
}

/// An address-tied global entry covering `[first, last]` with the given flags
/// (the `all_flags` getAllFlags() result) and optional type.
fn entry(
    space: &Rc<AddrSpace>,
    first: u64,
    last: u64,
    all_flags: u32,
    ty: Option<Rc<Datatype>>,
) -> GlobalEntry {
    GlobalEntry {
        space_index: space.get_index(),
        first,
        last,
        size: (last - first + 1) as i32,
        all_flags,
        addrtied: (all_flags & varnode_flags::addrtied) != 0,
        uselimit: RangeList::new(),
        symbol_name: "g".to_string(),
        symbol_offset: 0,
        symbol_type: ty,
        scope_path: Vec::new(),
        is_function: false,
        func_inject_id: -1,
    }
}

// ===========================================================================
// V1 — query_properties SYMBOL branch: getAllFlags() verbatim.
// ===========================================================================
#[test]
fn v1_query_properties_returns_covering_symbol_all_flags() {
    let ram = ram_space(3);
    // A covering entry whose getAllFlags() carries mapped|addrtied|persist|readonly.
    let fl = varnode_flags::mapped
        | varnode_flags::addrtied
        | varnode_flags::persist
        | varnode_flags::readonly;
    let gq = GlobalQuery {
        entries: vec![entry(&ram, 0x4000, 0x4003, fl, None)],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let usepoint = Address::new_invalid();
    let got = gq.query_properties(&addr(&ram, 0x4000), 4, &usepoint);
    // C++ `flags = res->getAllFlags()` — verbatim, NOT OR'd with anything extra.
    assert_eq!(
        got, fl,
        "covering-symbol branch must return getAllFlags() verbatim"
    );
}

// ===========================================================================
// V2 — query_properties OWNED branch: no symbol but owned → mapped|addrtied|
//      persist | getProperty.  The keystone for global-store survival.
// ===========================================================================
#[test]
fn v2_query_properties_owned_no_symbol_is_mapped_addrtied_persist() {
    let ram = ram_space(3);
    let mut owned = RangeList::new();
    owned.insert_range(ram.clone(), 0x5000, 0x5fff);
    // flagbase paints readonly over part of the owned range.
    let mut flagbase = PartMap::new(0u32);
    *flagbase.split(&addr(&ram, 0x5000)) = varnode_flags::readonly;
    *flagbase.split(&addr(&ram, 0x5008)) = 0;
    let gq = GlobalQuery {
        entries: Vec::new(), // NO covering symbol
        owned,
        flagbase,
    };
    let usepoint = Address::new_invalid();
    let got = gq.query_properties(&addr(&ram, 0x5000), 4, &usepoint);
    // C++: flags = mapped|addrtied|persist | getProperty(addr).
    let want = varnode_flags::mapped
        | varnode_flags::addrtied
        | varnode_flags::persist
        | varnode_flags::readonly;
    assert_eq!(
        got, want,
        "owned-scope (no-symbol) branch must add mapped|addrtied|persist to getProperty"
    );
    // A point in the owned range past the readonly split carries no readonly.
    let got2 = gq.query_properties(&addr(&ram, 0x5008), 4, &usepoint);
    assert_eq!(
        got2,
        varnode_flags::mapped | varnode_flags::addrtied | varnode_flags::persist,
        "owned but non-readonly sub-range carries only mapped|addrtied|persist"
    );
}

// ===========================================================================
// V3 — query_properties FLAGBASE-ONLY branch + constant-address guard.
// ===========================================================================
#[test]
fn v3_query_properties_flagbase_only_and_constant_guard() {
    let ram = ram_space(3);
    let mut flagbase = PartMap::new(0u32);
    *flagbase.split(&addr(&ram, 0x9000)) = varnode_flags::readonly;
    *flagbase.split(&addr(&ram, 0x9004)) = 0;
    let gq = GlobalQuery {
        entries: Vec::new(),  // no symbol
        owned: RangeList::new(), // not owned
        flagbase,
    };
    let usepoint = Address::new_invalid();
    // No symbol, not owned: just getProperty (NOT mapped/addrtied/persist).
    let got = gq.query_properties(&addr(&ram, 0x9000), 4, &usepoint);
    assert_eq!(
        got,
        varnode_flags::readonly,
        "no-symbol/no-scope branch returns only the range's boolean properties"
    );
    assert_eq!(
        got & (varnode_flags::mapped | varnode_flags::addrtied | varnode_flags::persist),
        0,
        "flagbase-only branch must NOT paint mapped/addrtied/persist"
    );
    // Constant address: `stackContainer` skips it; query_properties returns 0.
    let cspace = the_constant_space();
    let cval = addr(&cspace, 0x9000);
    assert_eq!(
        gq.query_properties(&cval, 4, &usepoint),
        0,
        "a constant address is never a mapped global (isConstant guard)"
    );
}

// ===========================================================================
// V4 — find_container_flags smallest-size selection returns the SMALLER entry's
//      flags, even when a larger entry also covers the range.
// ===========================================================================
#[test]
fn v4_smallest_containing_flags_win() {
    let ram = ram_space(3);
    // 4-byte entry [0x4000,0x4003] persist; 8-byte entry [0x4000,0x4007] persist
    // + readonly.  A 4-byte query is contained by BOTH; the SMALLER (4-byte)
    // entry's flags must win (no readonly).  Order the larger first to prove the
    // selection is by size, not iteration position.
    let small_fl = varnode_flags::mapped | varnode_flags::addrtied | varnode_flags::persist;
    let big_fl = small_fl | varnode_flags::readonly;
    let gq = GlobalQuery {
        entries: vec![
            entry(&ram, 0x4000, 0x4007, big_fl, None),
            entry(&ram, 0x4000, 0x4003, small_fl, None),
        ],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let usepoint = Address::new_invalid();
    let got4 = gq.query_properties(&addr(&ram, 0x4000), 4, &usepoint);
    assert_eq!(
        got4, small_fl,
        "the smallest containing entry's flags must win (no readonly leak from the wider entry)"
    );
    // An 8-byte query is contained ONLY by the wider entry → its flags (readonly).
    let got8 = gq.query_properties(&addr(&ram, 0x4000), 8, &usepoint);
    assert_eq!(
        got8, big_fl,
        "an 8-byte query is contained only by the 8-byte entry → its flags"
    );
}

// ===========================================================================
// V5 — sized_type_geometry typelock gate + last-byte containment boundary.
// ===========================================================================
#[test]
fn v5_sized_type_only_when_typelocked_and_boundary_inclusive() {
    let ram = ram_space(3);
    let ty: Rc<Datatype> = Rc::new(Datatype::new(
        4,
        kuna_decomp::dtype::type_metatype::TYPE_INT,
    ));
    let usepoint = Address::new_invalid();

    // (a) NON-typelocked covering Symbol → None (inference stays free).
    let gq_nolock = GlobalQuery {
        entries: vec![entry(
            &ram,
            0x6000,
            0x6003,
            varnode_flags::mapped | varnode_flags::addrtied | varnode_flags::persist,
            Some(ty.clone()),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    assert!(
        gq_nolock
            .sized_type_geometry(&addr(&ram, 0x6000), 4, &usepoint)
            .is_none(),
        "a non-typelocked covering Symbol must NOT force a type (updateType gate)"
    );

    // (b) typelocked covering Symbol → Some((type, off=0)).
    let gq_lock = GlobalQuery {
        entries: vec![entry(
            &ram,
            0x6000,
            0x6003,
            varnode_flags::mapped
                | varnode_flags::addrtied
                | varnode_flags::persist
                | varnode_flags::typelock,
            Some(ty.clone()),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let got = gq_lock.sized_type_geometry(&addr(&ram, 0x6000), 4, &usepoint);
    assert!(got.is_some(), "a typelocked covering Symbol forces its type");
    assert_eq!(got.unwrap().1, 0, "whole-symbol access → in-symbol offset 0");

    // (c) last-byte containment is inclusive: a 4-byte query at 0x6000 fits in
    //     [0x6000,0x6003] (end = 0x6003 == last); a 5-byte query does NOT.
    assert!(
        gq_lock
            .sized_type_geometry(&addr(&ram, 0x6000), 4, &usepoint)
            .is_some(),
        "end = addr+size-1 == last is contained (inclusive boundary)"
    );
    assert!(
        gq_lock
            .sized_type_geometry(&addr(&ram, 0x6000), 5, &usepoint)
            .is_none(),
        "a query one byte past the entry's last byte is NOT contained"
    );
}
