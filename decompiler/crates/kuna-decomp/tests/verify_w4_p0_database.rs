//! Adversarial verifier tests for item `w4-p0-database` (round 1).
//!
//! These target the hunt-list spots flagged as most fragile for the
//! `database.{cc,hh}` port: address-space-mask overflow in `addMapInternal`,
//! the `makeNameUnique` do-while sequence at the 2-digit/x-form boundary,
//! name-dedup collision ordering, the near-max `buildUndefinedName` hex bump,
//! and the reverse subsort walk of `findContainer`/`findClosestFit` when
//! several entries share a starting address.
//!
//! Driven entirely through the public `Database` API so the diff stands on its
//! own against the C++ source.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::database::{
    kuna_global_data_name, Database, DatabaseArch, ScopeId, TranslateAccess, TypeFactoryAccess,
};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::varnode::varnode_flags;
use kuna_base::types::{uintb, uint4};

/// Minimal angr-style architecture access impl: no register names, angr naming on.
/// `buildVariableName` with the `persist` flag and an empty register name then
/// renders `dat_<addr>` and feeds it through `makeNameUnique` — the path under
/// test for the x-form boundary.
struct AngrArch {
    num_spaces: int4,
}
struct NoTypes;
impl TypeFactoryAccess for NoTypes {
    fn get_base(&self, size: int4, meta: type_metatype) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, meta))
    }
    fn get_type_code(&self) -> Rc<Datatype> {
        Rc::new(Datatype::new(1, type_metatype::TYPE_CODE))
    }
}
impl TranslateAccess for AngrArch {
    fn get_register_name(&self, _sp: &Rc<AddrSpace>, _off: uintb, _size: int4) -> String {
        String::new() // no register backs any storage in these tests
    }
}
impl DatabaseArch for AngrArch {
    fn num_spaces(&self) -> int4 {
        self.num_spaces
    }
    fn types(&self) -> &dyn TypeFactoryAccess {
        &NoTypes
    }
    fn translate(&self) -> &dyn TranslateAccess {
        self
    }
    fn min_funcsymbol_size(&self) -> int4 {
        1
    }
    fn name_style_angr(&self) -> bool {
        true
    }
    fn type_name_base(&self, _dt: &Datatype) -> String {
        String::new()
    }
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// const(0), unique(1), ram(2, 8-byte), register(3, 4-byte).
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
    // 4-byte "register" space: highest offset is 0xffffffff.
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
    // Global scope: empty name, id 0 (find_create_scope builds + attaches it).
    let g = db.find_create_scope(0, "", None, num_spaces).unwrap();
    (db, g)
}

// ---------------------------------------------------------------------------
// F1: addMapInternal overflow check must wrap at the address-space mask, not
// at 64 bits.  C++ `Address lastaddress = addr + (sz-1)` uses
// `wrapOffset` (masks to the space's `highest`); a 4-byte space wraps at
// 0xffffffff, so a symbol straddling the top of the space throws
// "extends beyond the end of the address space".  The port computes
// `addr.get_offset().wrapping_add((sz-1) as uintb)` (raw 64-bit), which never
// detects the wrap for a sub-64-bit space.
// ---------------------------------------------------------------------------
#[test]
fn w4_p0_database_addmap_overflow_uses_space_mask() {
    let m = build_manager();
    let reg = space(&m, 3); // 4-byte space, highest = 0xffffffff
    let (mut db, g) = db_with_global(m.num_spaces());

    // 8-byte symbol starting at register:0xfffffffe.  In the 4-byte space this
    // straddles the top: last byte = wrapOffset(0xfffffffe + 7) wraps below the
    // start, so C++ addMapInternal throws "extends beyond the end of the
    // address space".
    let top = Address::new(Rc::clone(&reg), 0xffff_fffe);
    let res = db.add_symbol_mapped(g, "topsym", dt(8), &top, &Address::new_invalid());

    assert!(
        res.is_err(),
        "FINDING F1: a symbol straddling the top of a 4-byte address space must \
         be rejected (C++ uses wrapOffset on the space mask); the port's raw \
         64-bit wrapping_add accepted it: {res:?}"
    );
}

// A symbol that fits entirely within the 4-byte space must still be accepted
// (guards against an over-eager fix that rejects valid sub-64-bit symbols).
#[test]
fn w4_p0_database_addmap_inbounds_4byte_ok() {
    let m = build_manager();
    let reg = space(&m, 3);
    let (mut db, g) = db_with_global(m.num_spaces());
    // 4 bytes at register:0x0 — well within the space.
    let a = Address::new(Rc::clone(&reg), 0x0);
    let r = db.add_symbol_mapped(g, "ok", dt(4), &a, &Address::new_invalid());
    assert!(r.is_ok(), "in-bounds 4-byte symbol must be accepted: {r:?}");
    // And it is findable.
    let inv = Address::new_invalid();
    assert!(db.find_addr(g, &a, &inv).is_some());
}

// The 8-byte (full-width) ram space genuinely overflows at 64 bits, and both
// C++ and the port detect it identically — locks in the common-path agreement.
#[test]
fn w4_p0_database_addmap_overflow_full_width_space() {
    let m = build_manager();
    let ram = space(&m, 2); // 8-byte space, highest = 0xffffffffffffffff
    let (mut db, g) = db_with_global(m.num_spaces());
    let top = Address::new(Rc::clone(&ram), 0xffff_ffff_ffff_fffe);
    let res = db.add_symbol_mapped(g, "wrap8", dt(8), &top, &Address::new_invalid());
    assert!(
        res.is_err(),
        "8-byte space symbol wrapping past 2^64 must be rejected: {res:?}"
    );
}

// ---------------------------------------------------------------------------
// makeNameUnique: the C++ do-while sequence walks back over `<nm>_NN` and
// `<nm>_xNNNNN` forms.  The 2-digit form covers _00.._99; at 100 it must
// switch to the 5-digit x-form (_x00100).  This exercises the boundary the
// `do {} while` guards (digCount == 2 vs == 5).
// ---------------------------------------------------------------------------
// NOTE: C++ `addSymbol` does NOT uniquify — duplicate names are allowed via
// `nameDedup`.  `makeNameUnique` is reached through `buildVariableName`; the
// angr `persist` path renders `dat_<addr>` and uniquifies it, which is what
// these tests drive (a faithful end-to-end exercise of the do-while sequence).
#[test]
fn w4_p0_database_make_name_unique_xform_boundary() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());
    let arch = AngrArch { num_spaces: m.num_spaces() };

    let addr = Address::new(Rc::clone(&ram), 0x4000);
    let basebare = kuna_global_data_name(&addr); // dat_4000

    // Seed dat_4000 plus the full two-digit run dat_4000_00..dat_4000_99.
    db.add_symbol(g, &basebare, dt(4)).unwrap();
    for i in 0..=99u32 {
        db.add_symbol(g, &format!("{basebare}_{i:02}"), dt(4)).unwrap();
    }

    // buildVariableName(persist, empty regname) -> makeNameUnique("dat_4000").
    // The two-digit run is exhausted at _99 so uniqid 99 -> +1 -> 100, which is
    // >= 100 and renders as the 5-digit x-form dat_4000_x00100.
    let mut index: int4 = 0;
    let nm = db
        .build_variable_name(
            g,
            &addr,
            &Address::new_invalid(),
            Some(&Datatype::new(4, type_metatype::TYPE_UNKNOWN)),
            &mut index,
            varnode_flags::persist,
            &arch,
        )
        .unwrap();
    assert_eq!(
        nm, "dat_4000_x00100",
        "two-digit run exhausted at _99 must roll to the 5-digit x-form"
    );
}

// A `<nm>_<3 digits>` name is NOT in the 2-digit form and must be ignored by
// the digCount==2 guard, so the sequence starts a fresh _00.
#[test]
fn w4_p0_database_make_name_unique_ignores_wrong_digit_count() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());
    let arch = AngrArch { num_spaces: m.num_spaces() };

    let addr = Address::new(Rc::clone(&ram), 0x5000);
    let basebare = kuna_global_data_name(&addr); // dat_5000
    db.add_symbol(g, &basebare, dt(4)).unwrap();
    // dat_5000_123 has three digits: not the 2-digit form, not the x-form.
    db.add_symbol(g, &format!("{basebare}_123"), dt(4)).unwrap();

    // makeNameUnique must skip dat_5000_123 and start a fresh sequence at _00.
    let mut index: int4 = 0;
    let nm = db
        .build_variable_name(
            g,
            &addr,
            &Address::new_invalid(),
            Some(&Datatype::new(4, type_metatype::TYPE_UNKNOWN)),
            &mut index,
            varnode_flags::persist,
            &arch,
        )
        .unwrap();
    assert_eq!(nm, format!("{basebare}_00"));
}

// Suppress unused-import warning for uint4 (kept for symmetry with the access-trait decls).
const _: uint4 = 0;

// ---------------------------------------------------------------------------
// Name dedup: repeated identical names get increasing nameDedup ids, and
// findByName returns them in (name, dedup) order.  Exercises the
// insertNameTree upper_bound/--iter collision path under many collisions.
// ---------------------------------------------------------------------------
#[test]
fn w4_p0_database_name_dedup_many_collisions_ordered() {
    let (mut db, g) = db_with_global(4);
    let mut ids = Vec::new();
    for _ in 0..50 {
        ids.push(db.add_symbol(g, "dup", dt(4)).unwrap());
    }
    // Dedup ids are 0..50 in insertion order.
    for (i, &id) in ids.iter().enumerate() {
        assert_eq!(db.symbol(id).name_dedup, i as u32, "dedup id at {i}");
    }
    // findByName preserves dedup order and returns exactly the 50 symbols.
    let found = db.find_by_name(g, "dup");
    assert_eq!(found, ids, "findByName must return dedup-ordered ids");
}

// ---------------------------------------------------------------------------
// buildUndefinedName: an empty-named symbol is assigned $$undef<8 hex>.  The
// next one increments the hex; a near-max existing value must still bump
// correctly (and only u32::MAX itself is the error case).
// ---------------------------------------------------------------------------
#[test]
fn w4_p0_database_build_undefined_name_increments_from_existing() {
    let (mut db, g) = db_with_global(4);
    // First empty-named symbol -> $$undef00000000.
    let a = db.add_symbol(g, "", dt(4)).unwrap();
    assert_eq!(db.symbol(a).name, "$$undef00000000");
    // Second -> $$undef00000001 (lower_bound("$$undefz") then --iter sees the
    // largest existing undef name).
    let b = db.add_symbol(g, "", dt(4)).unwrap();
    assert_eq!(db.symbol(b).name, "$$undef00000001");

    // Seed a high-valued undef name and confirm the next bump is +1, not a
    // restart (proves the hex parse reads all 8 digits, not a prefix).
    db.add_symbol(g, "$$undef0000fffe", dt(4)).unwrap();
    let c = db.add_symbol(g, "", dt(4)).unwrap();
    assert_eq!(db.symbol(c).name, "$$undef0000ffff");
}

// ---------------------------------------------------------------------------
// findContainer / findClosestFit reverse subsort walk: several entries start
// at the same address.  findContainer must return the SMALLEST entry that
// still contains the requested range; findClosestFit must pick the tightest
// fit by signed size difference (negative diffs lose to any non-negative).
// ---------------------------------------------------------------------------
#[test]
fn w4_p0_database_find_container_smallest_enclosing() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());
    let base = Address::new(Rc::clone(&ram), 0x2000);
    // Three nested symbols at the same start: 16, 8, 4 bytes.
    let (s16, _) = db
        .add_symbol_mapped(g, "s16", dt(16), &base, &Address::new_invalid())
        .unwrap();
    let (s8, _) = db
        .add_symbol_mapped(g, "s8", dt(8), &base, &Address::new_invalid())
        .unwrap();
    let (s4, _) = db
        .add_symbol_mapped(g, "s4", dt(4), &base, &Address::new_invalid())
        .unwrap();
    let inv = Address::new_invalid();

    // A 4-byte access -> smallest enclosing is s4.
    let e = db.find_container(g, &base, 4, &inv).expect("4-byte container");
    assert_eq!(db.entry(g, e).symbol, s4);
    // A 5-byte access -> s4 too small, smallest enclosing is s8.
    let e = db.find_container(g, &base, 5, &inv).expect("5-byte container");
    assert_eq!(db.entry(g, e).symbol, s8);
    // A 9-byte access -> only s16 contains it.
    let e = db.find_container(g, &base, 9, &inv).expect("9-byte container");
    assert_eq!(db.entry(g, e).symbol, s16);
    // A 17-byte access -> none contain it.
    assert!(db.find_container(g, &base, 17, &inv).is_none());
}

#[test]
fn w4_p0_database_find_closest_fit_negative_diff_loses() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());
    let base = Address::new(Rc::clone(&ram), 0x3000);
    // 4-byte and 16-byte symbols at the same start.
    let (s4, _) = db
        .add_symbol_mapped(g, "f4", dt(4), &base, &Address::new_invalid())
        .unwrap();
    let (s16, _) = db
        .add_symbol_mapped(g, "f16", dt(16), &base, &Address::new_invalid())
        .unwrap();
    let inv = Address::new_invalid();

    // Exact 4-byte fit -> s4 (newdiff 0 wins immediately).
    let e = db.find_closest_fit(g, &base, 4, &inv).expect("exact fit");
    assert_eq!(db.entry(g, e).symbol, s4);

    // 8-byte access: s4 gives newdiff -4 (negative), s16 gives +8.  A
    // non-negative diff always beats a negative one, so s16 wins even though
    // its magnitude is larger.
    let e = db.find_closest_fit(g, &base, 8, &inv).expect("8-byte fit");
    assert_eq!(
        db.entry(g, e).symbol,
        s16,
        "non-negative size diff must beat a negative one"
    );
}

// ===========================================================================
// Round-2 verifier additions (independent re-derivation).
//
// These target spots the round-2 re-derivation flagged as still-fragile after
// round 1's overflow fix:
//   - makeNameUnique's lower_bound boundary: a `<nm>_xNNNNN` (5-digit) form is
//     parsed as the x-branch, so the next id rolls forward from it, NOT from a
//     fresh _00 (exercises the `do {} while` digCount==5 arm and the +1 bump);
//   - removeSymbol category-tail trimming followed by a re-add: the category
//     slot index must be recomputed (the C++ `pop_back` of trailing nulls
//     means the next FAKE_INPUT/EQUATE-category symbol reuses the freed index);
//   - query_properties' finalscope arm: when no SymbolEntry contains the range
//     but the (global) scope owns it, the returned flags must be
//     mapped|addrtied|persist | get_property(addr) — not the symbol-entry flags.
// ===========================================================================

// makeNameUnique: an existing 5-digit x-form (`dat_6000_x00100`) is recognized
// by the digCount==5 arm; the produced id is x-form value + 1 = 101, which is
// still >= 100 so it renders as the 5-digit x-form `dat_6000_x00101`.
#[test]
fn w4_p0_database_make_name_unique_continues_xform() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());
    let arch = AngrArch { num_spaces: m.num_spaces() };

    let addr = Address::new(Rc::clone(&ram), 0x6000);
    let basebare = kuna_global_data_name(&addr); // dat_6000

    // Seed the base name plus a single x-form entry at 100.
    db.add_symbol(g, &basebare, dt(4)).unwrap();
    db.add_symbol(g, &format!("{basebare}_x00100"), dt(4)).unwrap();

    let mut index: int4 = 0;
    let nm = db
        .build_variable_name(
            g,
            &addr,
            &Address::new_invalid(),
            Some(&Datatype::new(4, type_metatype::TYPE_UNKNOWN)),
            &mut index,
            varnode_flags::persist,
            &arch,
        )
        .unwrap();
    // The largest matching name is the x-form 100; +1 -> 101 -> still x-form.
    assert_eq!(
        nm, "dat_6000_x00101",
        "an existing 5-digit x-form must roll forward (digCount==5 arm)"
    );
}

// removeSymbol trims trailing nulls from the category tail; a subsequent
// category-1 (EQUATE) add must reuse the freed index (C++ catindex = list.size()
// after pop_back).  Drives the addSymbolInternal `catindex = list.size()` path
// for category > 0 through the public equate factory + removal.
#[test]
fn w4_p0_database_category_index_reused_after_tail_removal() {
    let (mut db, g) = db_with_global(4);
    let inv = Address::new_invalid();
    // Three EQUATE-category symbols get catindex 0,1,2 (category 1 > 0, so the
    // index is auto-assigned to the current list length).
    let e0 = db
        .add_equate_symbol(g, "e0", 0, 10, &inv, 0, dt(1))
        .unwrap();
    let e1 = db
        .add_equate_symbol(g, "e1", 0, 11, &inv, 0, dt(1))
        .unwrap();
    let e2 = db
        .add_equate_symbol(g, "e2", 0, 12, &inv, 0, dt(1))
        .unwrap();
    assert_eq!(db.symbol(e0).get_category_index(), 0);
    assert_eq!(db.symbol(e1).get_category_index(), 1);
    assert_eq!(db.symbol(e2).get_category_index(), 2);

    // Remove the tail (e2): the category list pops its trailing slot back to
    // length 2.  A new equate must then take catindex 2 again.
    db.remove_symbol(e2);
    let e3 = db
        .add_equate_symbol(g, "e3", 0, 13, &inv, 0, dt(1))
        .unwrap();
    assert_eq!(
        db.symbol(e3).get_category_index(),
        2,
        "after tail removal the freed category index must be reused"
    );

    // Removing a MIDDLE entry leaves a hole (not popped): the next add grows
    // the list to index 3, it does NOT backfill the hole at index 1.
    db.remove_symbol(e1); // hole at index 1, tail (e3) still at 2
    let e4 = db
        .add_equate_symbol(g, "e4", 0, 14, &inv, 0, dt(1))
        .unwrap();
    assert_eq!(
        db.symbol(e4).get_category_index(),
        3,
        "a middle hole is not backfilled; new index is the current length"
    );
}

// query_properties finalscope arm: a range owned by the (global) scope's
// rangetree but covered by NO SymbolEntry returns the synthesized owner flags
// mapped|addrtied|persist, OR'd with the address property bits — NOT zero and
// NOT any symbol's flags.
#[test]
fn w4_p0_database_query_properties_finalscope_flags() {
    let m = build_manager();
    let ram = space(&m, 2);
    let (mut db, g) = db_with_global(m.num_spaces());

    // Give the global scope ownership of [0x8000, 0x80ff] and mark part of it
    // read-only, but place NO symbol there.
    db.add_range(g, Rc::clone(&ram), 0x8000, 0x80ff);
    let a1 = Address::new(Rc::clone(&ram), 0x8000);
    let a2 = Address::new(Rc::clone(&ram), 0x8010); // one-past-end of the ro run
    db.set_property_range(varnode_flags::readonly, &a1, &a2);

    let query = Address::new(Rc::clone(&ram), 0x8004);
    let (res, flags) = db.query_properties(g, &query, 4, &Address::new_invalid());
    assert!(res.is_none(), "no SymbolEntry should contain the range");
    // global owner => mapped|addrtied|persist, plus the readonly property bit.
    let expected = varnode_flags::mapped
        | varnode_flags::addrtied
        | varnode_flags::persist
        | varnode_flags::readonly;
    assert_eq!(
        flags, expected,
        "finalscope flags must be the synthesized global-owner set | property bits"
    );

    // Outside the scope's owned range and with no property: flags are just the
    // (zero) property bits, since there is no finalscope and no entry.
    let outside = Address::new(Rc::clone(&ram), 0x9000);
    let (res2, flags2) = db.query_properties(g, &outside, 4, &Address::new_invalid());
    assert!(res2.is_none());
    assert_eq!(flags2, 0, "unowned, unpropertied range yields zero flags");
}
