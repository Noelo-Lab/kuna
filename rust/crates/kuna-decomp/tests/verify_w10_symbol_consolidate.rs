//! INDEPENDENT VERIFIER adversarial tests — item: w10-symbol-consolidate
//! (the type-FORCE half of the global query, driving the displayformat render).
//!
//! These exercise the faithfulness-fragile spots of
//! [`kuna_decomp::seams::GlobalQuery::sized_type_geometry`] — the wire for the
//! type-force half of `Funcdata::setVarnodeProperties`
//! (`funcdata_varnode.cc:31`) → `Varnode::setSymbolProperties` (`varnode.cc:429`)
//! → `SymbolEntry::updateType` (`database.cc:136`):
//!
//!     if ((symbol->getFlags() & typelock) != 0) {
//!       dt = getSizedType(addr, size);             // database.cc:151
//!       if (dt) vn->updateType(dt, true, true);
//!     }
//!     // getSizedType: off = (inaddr - addr.getOffset()) + entry.offset;
//!     //               return getExactPiece(symbol->getType(), off, size);
//!
//! The result (`symbol_type`, in-symbol `off`) is exactly what the caller feeds
//! `TypeFactory::getExactPiece` before applying the type-lock; getting either
//! wrong mis-seeds `ActionInferTypes` and the forced display format (`oct`/`hex`)
//! never reaches the stored constant (`globaloct = 05555`).
//!
//! The TYPELOCK GATE is the load-bearing distinction these tests pin: a mapped
//! global with a non-type-locked Symbol must NOT force a type (C++ returns false
//! from `updateType`, leaving inference free), and the in-symbol offset must add
//! the entry's own `offset` (a struct/array piece), not just `access - first`.
//!
//! All assertions are derived from the C++ body, not the Rust output.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::seams::{GlobalEntry, GlobalQuery};
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

fn addr(space: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(space.clone(), off)
}

/// A named integer Datatype with a forced display format (the `octint4`/`uint8_t`
/// of the `displayformat` datatest: an int4 carrying a forced `oct`/`hex` format).
fn fmt_int(size: i32, name: &str, format: u32) -> Rc<Datatype> {
    let mut dt = Datatype::new(size, type_metatype::TYPE_INT);
    dt.name = name.to_string();
    dt.display_name = name.to_string();
    dt.set_display_format(format);
    Rc::new(dt)
}

/// A global Symbol entry covering `[first, last]` carrying `symbol_type`, with
/// `all_flags` controlling the typelock bit (`getAllFlags()` = the Symbol's
/// flags) and an in-Symbol `offset`.
fn typed_entry(
    space: &Rc<AddrSpace>,
    first: u64,
    last: u64,
    name: &str,
    offset: i32,
    all_flags: u32,
    symbol_type: Option<Rc<Datatype>>,
) -> GlobalEntry {
    GlobalEntry {
        space_index: space.get_index(),
        first,
        last,
        size: (last - first + 1) as i32,
        all_flags,
        addrtied: true,
        uselimit: RangeList::new(),
        symbol_name: name.to_string(),
        symbol_offset: offset,
        symbol_type,
        scope_path: Vec::new(),
    }
}

// ===========================================================================
// TF1 — a TYPE-LOCKED Symbol forces its (formatted) data-type
// ===========================================================================
//
// `map addr r0x301018 octint4 globaloct` creates a type-locked global Symbol of
// type `octint4` (an int4 with a forced `oct` display format).  A whole-symbol
// 4-byte query returns that very type at in-symbol offset 0 — the seed that
// makes `ActionInferTypes` flow `octint4` onto the store and its COPY input so
// `globaloct = 05555` renders octal.  `getDisplayFormat()` on the returned type
// must be the forced `oct` (3).
#[test]
fn tf1_typelocked_symbol_forces_its_formatted_type() {
    let ram = ram_space(3);
    let octint4 = fmt_int(4, "octint4", 3 /* Symbol::force_oct */);
    let gq = GlobalQuery {
        entries: vec![typed_entry(
            &ram,
            0x301018,
            0x30101b,
            "globaloct",
            0,
            varnode_flags::typelock | varnode_flags::addrtied | varnode_flags::mapped,
            Some(octint4),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let up = Address::new_invalid();

    let (dt, off) = gq
        .sized_type_geometry(&addr(&ram, 0x301018), 4, &up)
        .expect("a type-locked global must force its data-type");
    assert_eq!(dt.get_name(), "octint4", "the Symbol's data-type flows");
    assert_eq!(off, 0, "(0x301018 - 0x301018) + entry.offset(0) == 0");
    assert_eq!(
        dt.get_display_format(),
        3,
        "the forced oct display format rides on the type"
    );
}

// ===========================================================================
// TF2 — a NON-type-locked Symbol forces NOTHING (the typelock gate)
// ===========================================================================
//
// `SymbolEntry::updateType` only forces when `(symbol->getFlags() & typelock)`.
// A mapped/addrtied global whose Symbol is NOT type-locked must return `None`
// (the C++ `updateType` returns false, leaving `ActionInferTypes` free to seed
// the Varnode from dataflow).  Forcing a type here would wrongly pin an
// unlocked global and could regress inference on every non-typed `map addr`.
#[test]
fn tf2_non_typelocked_symbol_forces_no_type() {
    let ram = ram_space(3);
    let some_int = fmt_int(4, "int4", 0);
    let gq = GlobalQuery {
        entries: vec![typed_entry(
            &ram,
            0x301014,
            0x301017,
            "globalfree",
            0,
            // mapped | addrtied but NO typelock bit.
            varnode_flags::addrtied | varnode_flags::mapped,
            Some(some_int),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let up = Address::new_invalid();

    assert!(
        gq.sized_type_geometry(&addr(&ram, 0x301014), 4, &up).is_none(),
        "a non-type-locked Symbol does not force its type"
    );
}

// ===========================================================================
// TF3 — the in-symbol byte offset adds the entry's own offset (sub-piece)
// ===========================================================================
//
// `getSizedType` computes `off = (inaddr - addr.getOffset()) + entry.offset`.
// A type-locked Symbol whose mapped piece begins at 0x500 with `entry.offset`
// 4 (the offset-4 slice of a larger Symbol): a 4-byte query at 0x504 must
// return in-symbol offset `(0x504 - 0x500) + 4 == 8`, not 4.  The base offset
// MUST be added so `getExactPiece` indexes the right field of the Symbol type.
#[test]
fn tf3_sized_offset_is_access_minus_first_plus_entry_offset() {
    let ram = ram_space(3);
    let wide = fmt_int(8, "wide_struct", 1 /* force_hex, irrelevant here */);
    let gq = GlobalQuery {
        entries: vec![typed_entry(
            &ram,
            0x500,
            0x507,
            "structpiece",
            4,
            varnode_flags::typelock | varnode_flags::mapped,
            Some(wide),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let up = Address::new_invalid();

    let (_dt, off) = gq
        .sized_type_geometry(&addr(&ram, 0x504), 4, &up)
        .expect("0x504 is within the type-locked piece");
    assert_eq!(off, 8, "(0x504 - 0x500) + entry.offset(4) == 8");
}

// ===========================================================================
// TF4 — a constant Varnode is never type-forced
// ===========================================================================
//
// `sized_type_geometry` shares the `addr.isConstant()` early-out of the global
// query (a constant's offset is a value, not an address).  Even a type-locked
// entry whose numeric range would "contain" the constant value must not force a
// type onto a constant Varnode.
#[test]
fn tf4_constant_address_is_never_type_forced() {
    let ram = ram_space(3);
    let octint4 = fmt_int(4, "octint4", 3);
    let gq = GlobalQuery {
        entries: vec![typed_entry(
            &ram,
            0x100,
            0x103,
            "phantom",
            0,
            varnode_flags::typelock | varnode_flags::mapped,
            Some(octint4),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let cst = kuna_base::address::Address::new(
        Rc::new(kuna_base::space::ConstantSpace::new()),
        0x100,
    );
    let up = Address::new_invalid();

    assert!(
        gq.sized_type_geometry(&cst, 4, &up).is_none(),
        "a constant Varnode is never type-forced"
    );
}

// ===========================================================================
// TF5 — a type-locked entry with no data-type yields None (NULL getSizedType)
// ===========================================================================
//
// `getSizedType` returns NULL when there is no matching sub-type, and C++ then
// skips `updateType`.  An entry flagged type-locked but carrying no
// `symbol_type` (a degenerate snapshot) must yield `None` rather than panic —
// the faithful "no exact piece, no force" outcome.
#[test]
fn tf5_typelocked_entry_without_type_yields_none() {
    let ram = ram_space(3);
    let gq = GlobalQuery {
        entries: vec![typed_entry(
            &ram,
            0x700,
            0x703,
            "notype",
            0,
            varnode_flags::typelock | varnode_flags::mapped,
            None,
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let up = Address::new_invalid();

    assert!(
        gq.sized_type_geometry(&addr(&ram, 0x700), 4, &up).is_none(),
        "a type-locked entry with no data-type forces nothing"
    );
}

// ===========================================================================
// TF6 — a non-covering query returns None (containment is inclusive on getLast)
// ===========================================================================
//
// A 4-byte query one byte past the entry's `last` does NOT match (the entry
// covers `[first, last]` inclusive; `find_container_entry` requires
// `e.last >= addr + size - 1`).  No covering entry → no forced type.
#[test]
fn tf6_query_past_last_byte_forces_no_type() {
    let ram = ram_space(3);
    let octint4 = fmt_int(4, "octint4", 3);
    let gq = GlobalQuery {
        // Covers [0x600, 0x603].
        entries: vec![typed_entry(
            &ram,
            0x600,
            0x603,
            "g",
            0,
            varnode_flags::typelock | varnode_flags::mapped,
            Some(octint4),
        )],
        owned: RangeList::new(),
        flagbase: PartMap::new(0u32),
    };
    let up = Address::new_invalid();

    // A 4-byte access at 0x601 needs [0x601,0x604]; 0x604 > last(0x603): no match.
    assert!(
        gq.sized_type_geometry(&addr(&ram, 0x601), 4, &up).is_none(),
        "a query whose last byte is past the entry's last does not match"
    );
    // The exact whole-symbol query DOES match (sanity that the gate is the bound,
    // not a blanket None).
    assert!(
        gq.sized_type_geometry(&addr(&ram, 0x600), 4, &up).is_some(),
        "the in-bounds whole-symbol query still forces the type"
    );
}
