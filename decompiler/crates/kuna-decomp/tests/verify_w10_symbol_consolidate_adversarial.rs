//! INDEPENDENT VERIFIER adversarial tests — item: w10-symbol-consolidate.
//!
//! These exercise the faithfulness-fragile spots of the global-SYMBOL naming
//! query [`kuna_decomp::context::GlobalQuery::name_for_varnode`] — the wire for
//! `Funcdata::linkSymbol`'s reach into the global scope (`funcdata_varnode.cc:1190`
//! `localmap->queryProperties` → the covering `SymbolEntry` → its Symbol's
//! display name), transcribed from `Scope::findContainer` + the
//! `name_for_varnode` symbol-offset geometry (`(addr - entry_addr) + entry_off`):
//!
//!   1. exact name resolution + smallest-containing selection — when two global
//!      entries cover a range, the SMALLER (more specific) Symbol's name wins,
//!      matching `findContainer`'s `oldsize` walk;
//!   2. the in-symbol byte offset (`symbol_offset`) for a member access — the
//!      `(access - first) + entry.offset` arithmetic, including a non-zero
//!      `entry.offset` (a struct/array piece);
//!   3. the constant-address guard (no global symbol is ever named for a
//!      constant Varnode — `stackContainer`'s `addr.isConstant()` early out);
//!   4. a non-covering query returns `None` (no name → the `dat_<addr>` /
//!      `Unique` unnamed-location tail), and a query just past the entry's last
//!      byte does NOT match (containment is inclusive on `getLast`);
//!   5. the `inUse(usepoint)` gate — an address-tied entry is valid at every
//!      usepoint (named even at an invalid usepoint); a use-limited (non-tied)
//!      entry is NOT named at an invalid usepoint.
//!
//! All assertions are derived from the C++ body, not the Rust output.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::space::{spacetype, AddrSpace, ConstantSpace};
use kuna_decomp::context::{GlobalEntry, GlobalQuery};

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

fn code_space(index: i32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "code",
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

/// A named, address-tied global Symbol entry covering `[first, last]`, with the
/// given display `name` and in-Symbol `offset`.
fn named_entry(
    space: &Rc<AddrSpace>,
    first: u64,
    last: u64,
    name: &str,
    offset: i32,
) -> GlobalEntry {
    GlobalEntry {
        space_index: space.get_index(),
        first,
        last,
        size: (last - first + 1) as i32,
        all_flags: 0,
        addrtied: true,
        uselimit: RangeList::new(),
        symbol_name: name.to_string(),
        symbol_offset: offset,
        symbol_type: None,
        scope_path: Vec::new(),
        is_function: false,
        func_inject_id: -1,
    }
}

// ===========================================================================
// AV1 — exact name + smallest-containing selection
// ===========================================================================
//
// Two global entries at 0x301014: one 4-byte ("glob1") and one 8-byte
// ("widepair") covering the same start.  `findContainer` selects the SMALLEST
// entry that still contains the query.  A 4-byte query must name "glob1" (the
// 4-byte entry contains it and is smaller); an 8-byte query must name
// "widepair" (only the 8-byte entry contains all 8 bytes).
#[test]
fn av1_smallest_containing_symbol_name_wins() {
    let ram = ram_space(3);
    let gq = GlobalQuery::new(
        vec![
            named_entry(&ram, 0x301014, 0x301017, "glob1", 0),
            named_entry(&ram, 0x301014, 0x30101b, "widepair", 0),
        ],
        RangeList::new(),
        PartMap::new(0u32),
    );
    let up = Address::new_invalid();

    let (name4, off4, _t) = gq
        .name_for_varnode(&addr(&ram, 0x301014), 4, &up)
        .expect("4-byte query must name the 4-byte glob1");
    assert_eq!(name4, "glob1", "smallest entry containing 4 bytes is glob1");
    assert_eq!(off4, 0, "whole-symbol access has offset 0");

    let (name8, _off8, _t8) = gq
        .name_for_varnode(&addr(&ram, 0x301014), 8, &up)
        .expect("8-byte query must name the only 8-byte container");
    assert_eq!(name8, "widepair", "only the 8-byte entry contains all 8 bytes");
}

// ===========================================================================
// AV2 — in-symbol byte offset geometry
// ===========================================================================
//
// `name_for_varnode` returns `(access - entry.first) + entry.offset`.  An array
// `arr` at 0x400 [0x400,0x40f], entry.offset 0: a 4-byte access at 0x408 maps to
// in-symbol offset 8.  A struct piece whose entry.offset is non-zero (a high
// piece of a larger Symbol mapped at 0x500 with entry.offset 4) must ADD that
// base: a 4-byte access at 0x500 returns offset 4, not 0.
#[test]
fn av2_symbol_offset_is_access_minus_first_plus_entry_offset() {
    let ram = ram_space(3);
    let gq = GlobalQuery::new(
        vec![
            named_entry(&ram, 0x400, 0x40f, "arr", 0),
            // A piece mapped at 0x500 that is the OFFSET-4 slice of its Symbol.
            named_entry(&ram, 0x500, 0x503, "structpiece", 4),
        ],
        RangeList::new(),
        PartMap::new(0u32),
    );
    let up = Address::new_invalid();

    let (name, off, _t) = gq
        .name_for_varnode(&addr(&ram, 0x408), 4, &up)
        .expect("0x408 is within arr [0x400,0x40f]");
    assert_eq!(name, "arr");
    assert_eq!(off, 8, "(0x408 - 0x400) + 0 == 8");

    let (pname, poff, _pt) = gq
        .name_for_varnode(&addr(&ram, 0x500), 4, &up)
        .expect("0x500 is the structpiece base");
    assert_eq!(pname, "structpiece");
    assert_eq!(poff, 4, "(0x500 - 0x500) + entry.offset(4) == 4");
}

// ===========================================================================
// AV3 — constant address never names a global symbol
// ===========================================================================
//
// `name_for_varnode` early-outs on a constant address (the `stackContainer`
// `addr.isConstant()` guard).  Even with a (degenerate) entry whose offsets
// would numerically match the constant's value, a constant Varnode is never
// named.
#[test]
fn av3_constant_address_is_never_named() {
    let cst = the_constant_space();
    let ram = ram_space(3);
    // An entry in the constant space's index that would "contain" 0x100.
    let gq = GlobalQuery::new(
        vec![named_entry(&ram, 0x100, 0x103, "phantom", 0)],
        RangeList::new(),
        PartMap::new(0u32),
    );
    let up = Address::new_invalid();
    assert!(
        gq.name_for_varnode(&addr(&cst, 0x100), 4, &up).is_none(),
        "a constant-space address must never resolve to a global symbol name",
    );
}

// ===========================================================================
// AV4 — non-containment: None for a miss / a one-byte-past query
// ===========================================================================
//
// Containment is inclusive on `getLast`.  A 4-byte entry [0x301014,0x301017]:
//   * a query at 0x301018 (the next storage) does NOT match (gap → None),
//   * a 5-byte query at 0x301014 (last byte 0x301018 > 0x301017) does NOT match,
//   * a query at 0x301013 (one byte before `first`) does NOT match.
#[test]
fn av4_no_match_returns_none() {
    let ram = ram_space(3);
    let gq = GlobalQuery::new(
        vec![named_entry(&ram, 0x301014, 0x301017, "glob1", 0)],
        RangeList::new(),
        PartMap::new(0u32),
    );
    let up = Address::new_invalid();

    assert!(
        gq.name_for_varnode(&addr(&ram, 0x301018), 4, &up).is_none(),
        "the adjacent (non-overlapping) address is unnamed",
    );
    assert!(
        gq.name_for_varnode(&addr(&ram, 0x301014), 5, &up).is_none(),
        "a 5-byte query overruns the 4-byte entry's getLast",
    );
    assert!(
        gq.name_for_varnode(&addr(&ram, 0x301013), 4, &up).is_none(),
        "a query beginning before `first` is not contained",
    );
    // The exact 4-byte whole-entry query DOES match (positive control).
    assert!(
        gq.name_for_varnode(&addr(&ram, 0x301014), 4, &up).is_some(),
        "the exact whole-entry query is named",
    );
}

// ===========================================================================
// AV5 — inUse(usepoint): addr-tied always, use-limited only in range
// ===========================================================================
//
// An address-tied entry is valid throughout the scope (named even at an invalid
// usepoint).  A NON-tied, use-limited entry is named only when the usepoint
// falls in its uselimit; at an invalid usepoint it is skipped (`inUse` returns
// false), so a query that only the use-limited entry could cover returns None.
#[test]
fn av5_inuse_usepoint_gate() {
    let ram = ram_space(3);
    let code = code_space(4);

    // Address-tied: named at an invalid usepoint.
    let tied = GlobalQuery::new(
        vec![named_entry(&ram, 0x600, 0x603, "tiedglob", 0)],
        RangeList::new(),
        PartMap::new(0u32),
    );
    assert!(
        tied.name_for_varnode(&addr(&ram, 0x600), 4, &Address::new_invalid()).is_some(),
        "an address-tied global is named regardless of usepoint",
    );

    // Use-limited (not tied), valid only at code 0x800.
    let mut uselimit = RangeList::new();
    uselimit.insert_range(code.clone(), 0x800, 0x800);
    let mut limited = named_entry(&ram, 0x700, 0x703, "limitedglob", 0);
    limited.addrtied = false;
    limited.uselimit = uselimit;
    let gq = GlobalQuery::new(vec![limited], RangeList::new(), PartMap::new(0u32));
    // Invalid usepoint → not inUse → None.
    assert!(
        gq.name_for_varnode(&addr(&ram, 0x700), 4, &Address::new_invalid()).is_none(),
        "a use-limited entry is NOT named at an invalid usepoint",
    );
    // In-range usepoint → named.
    assert!(
        gq.name_for_varnode(&addr(&ram, 0x700), 4, &addr(&code, 0x800)).is_some(),
        "a use-limited entry IS named when the usepoint is in range",
    );
}
