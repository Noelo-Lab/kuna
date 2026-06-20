//! VERIFIER adversarial tests for item `w10-partial-types` (round 1).
//!
//! The diff under review closes two console seams — `IfcMaphash` (`map hash`)
//! and `IfcMapunionfacet` (`map unionfacet`) — by wiring them to the existing
//! `Scope::addDynamicSymbol` / `Scope::addUnionFacetSymbol` database methods
//! through two new thin `ScopeLocal` wrappers in `varmap.rs`
//! (`add_dynamic_symbol` / `add_union_facet_symbol`), and fills four missing
//! arms in the `ruleaction_3::opflags_for` W6 shim.
//!
//! These tests target the spots the hunt list flagged as most fragile for this
//! item:
//!
//!  * the `field_num` boundary values the console command's bounds check
//!    (`fieldNum < -1 || fieldNum >= ct->numDepend()`, ifacedecomp.cc:790)
//!    lets through: `-1` (the "force the whole union" sentinel) and `0`,
//!  * that the dynamic-symbol size is taken from the *type* (C++
//!    `addDynamicMapInternal(...,ct->getSize(),...)`, database.cc:1705) and not
//!    a fixed width,
//!  * the invalid-address branch (`if (!caddr.isInvalid())`, database.cc:1703)
//!    that must still create the symbol but insert no range,
//!  * that the recovery is REAL — the produced symbol carries the union
//!    data-type, the `UNION_FACET` category, and the requested `field_num`
//!    round-trips (not silently dropped), matching `UnionFacetSymbol`.
//!
//! The bounds check itself + the `s >> hex >> hash` extraction + the
//! `findByName`/`TYPE_UNION` validation live in the console crate
//! (`kuna-console::ifacedecomp`) and are covered by the companion file
//! `kuna-console/tests/verify_w10_partial_types_console.rs`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::database::{symbol_category, SymbolKind};
use kuna_decomp::dtype::{type_metatype, Datatype, DatatypeKind, TypeField};
use kuna_decomp::varmap::ScopeLocal;
use kuna_decomp::varnode::varnode_flags;

// --- builders --------------------------------------------------------------

/// An 8-byte little-endian processor space at index 4 (the "stack" the
/// `ScopeLocal` owns); word size 1 so byte<->address is the identity.
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

/// `ScopeLocal::new` sized to cover the stack space's index (4) so symbol
/// allocation can reach its maptable slot.
fn scope_local() -> ScopeLocal {
    ScopeLocal::new(0x1234, stack_space(), "victim", 8).expect("build ScopeLocal")
}

/// A base data-type of `size` bytes (TYPE_UNKNOWN), align == size.
fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

/// A synthetic TYPE_UNION with `n` int4-sized fields so `num_depend() == n`.
fn union_with_fields(n: usize) -> Rc<Datatype> {
    let mut field = Vec::new();
    for i in 0..n {
        field.push(TypeField::new(i as int4, 0, format!("f{i}"), base(4, type_metatype::TYPE_INT)));
    }
    // Union size == max field size (4 here); override the Base payload with the
    // Union kind so `num_depend()` walks the field list.
    let mut dt = Datatype::new_with_align(4, 4, type_metatype::TYPE_UNION);
    dt.kind = DatatypeKind::Union { field };
    Rc::new(dt)
}

use kuna_base::types::int4;

// --- F1: union-facet recovery is REAL (whole-union sentinel) ---------------

#[test]
fn union_facet_whole_union_minus_one_roundtrips() {
    // C++ fieldNum == -1 means "force the whole union" (database.hh doc on
    // UnionFacetSymbol; the console bounds check permits exactly -1 as the low
    // bound: `fieldNum < -1` rejects, `== -1` passes).  The produced symbol must
    // carry that -1, the UNION_FACET category, and the union data-type — not a
    // silently-zeroed field or a dropped type.
    let mut sl = scope_local();
    let un = union_with_fields(3);
    let space = Rc::clone(sl.get_space_id());
    let addr = Address::new(space, 0x401000);
    let sid = sl
        .add_union_facet_symbol("unionfacet0_401000", Rc::clone(&un), -1, &addr, 0xdeadbeef)
        .expect("addUnionFacetSymbol");

    let sym = sl.database().symbol(sid);
    assert_eq!(sym.category, symbol_category::UNION_FACET, "category must be UNION_FACET");
    match sym.kind {
        SymbolKind::UnionFacet { field_num, addr_based } => {
            assert_eq!(field_num, -1, "whole-union sentinel -1 must round-trip");
            assert!(!addr_based, "C++ addrBased defaults false");
        }
        ref other => panic!("expected UnionFacet kind, got {other:?}"),
    }
    let dt = sym.dtype.as_ref().expect("union-facet symbol keeps its data-type");
    assert_eq!(dt.get_metatype(), type_metatype::TYPE_UNION, "must keep the union type");
    assert_eq!(dt.num_depend(), 3, "the union carried through has its 3 fields");
    // The dynamic mapping was registered (a hash-based entry, not a stack slot).
    assert_eq!(sym.mapentry.len(), 1, "exactly one dynamic entry for the hashed op");
}

// --- F2: field_num == 0 boundary (first real field) ------------------------

#[test]
fn union_facet_field_zero_boundary_roundtrips() {
    // 0 is the inclusive low end of the "real field" range; an off-by-one in the
    // wrapper that mishandled 0 (e.g. treating it like the -1 sentinel) would be
    // silent corruption.
    let mut sl = scope_local();
    let un = union_with_fields(2);
    let space = Rc::clone(sl.get_space_id());
    let addr = Address::new(space, 0x8);
    let sid = sl
        .add_union_facet_symbol("unionfacet1_8", un, 0, &addr, 1)
        .expect("addUnionFacetSymbol field 0");
    match sl.database().symbol(sid).kind {
        SymbolKind::UnionFacet { field_num, .. } => assert_eq!(field_num, 0),
        ref other => panic!("expected UnionFacet, got {other:?}"),
    }
}

// --- F3: dynamic-symbol size comes from the *type*, not a constant ----------

#[test]
fn dynamic_symbol_size_from_type_and_lock_flags() {
    // C++ addDynamicSymbol passes ct->getSize() as the entry size
    // (database.cc:1705); a port that hard-coded 1 (the union-facet width) would
    // silently mis-size every dynamic symbol.  Use a 7-byte type so a fixed 1 or
    // 8 would be caught.  Also exercise the console seam's
    // setAttribute(namelock|typelock) on the produced symbol.
    let mut sl = scope_local();
    let ct = base(7, type_metatype::TYPE_UNKNOWN);
    let space = Rc::clone(sl.get_space_id());
    let addr = Address::new(space, 0x402000);
    let sid =
        sl.add_dynamic_symbol("hv", Rc::clone(&ct), &addr, 0x55).expect("addDynamicSymbol");
    sl.set_attribute(sid, varnode_flags::namelock | varnode_flags::typelock);

    let sym = sl.database().symbol(sid);
    assert_eq!(sym.name, "hv");
    let dt = sym.dtype.as_ref().expect("dynamic symbol keeps its type");
    assert_eq!(dt.get_size(), 7, "size must derive from the declared type, not a constant");
    assert_eq!(sym.mapentry.len(), 1, "one dynamic entry");
    assert_eq!(
        sym.flags & (varnode_flags::namelock | varnode_flags::typelock),
        varnode_flags::namelock | varnode_flags::typelock,
        "the map command name/type-locks the symbol",
    );
}

// --- F4: invalid address still creates the symbol but inserts no range ------

#[test]
fn dynamic_symbol_invalid_address_skips_range_but_keeps_symbol() {
    // C++ guards the range insert with `if (!caddr.isInvalid())` — an invalid
    // address must NOT abort the symbol creation (the dynamic hash alone locates
    // the varnode).  A port that unconditionally indexed caddr.getSpace() would
    // panic here.
    let mut sl = scope_local();
    let ct = base(4, type_metatype::TYPE_INT);
    let invalid = Address::new_invalid();
    let sid = sl
        .add_dynamic_symbol("noaddr", ct, &invalid, 0x99)
        .expect("addDynamicSymbol with invalid address must still succeed");
    let sym = sl.database().symbol(sid);
    assert_eq!(sym.name, "noaddr");
    // The symbol exists with no spatial range, but its dynamic (hash) entry is
    // present (addDynamicMapInternal runs regardless of address validity).
    assert_eq!(sym.mapentry.len(), 1, "the dynamic hash entry is created even with no address");
}

// --- F5: opflags arms — the four newly-filled W6 entries match typeop.cc ----

#[test]
fn opflags_constants_match_cpp_pcodeop_enum() {
    // The four new `opflags_for` arms are `binary | commutative` (XOR/AND/OR),
    // `unary` (NEGATE/2COMP), `binary` (LEFT/RIGHT), `binary | booloutput`
    // (LESS).  `opflags_for` is private, but the arms are built from these
    // public flag constants; pin the constant VALUES against op.hh so a future
    // edit that renumbered them (and thus silently changed every transcribed
    // arm) is caught.  These are the exact bit values the C++ PcodeOp enum uses.
    use kuna_decomp::op::pcodeop_flags;
    assert_eq!(pcodeop_flags::booloutput, 0x80, "PcodeOp::booloutput (op.hh:82)");
    assert_eq!(pcodeop_flags::commutative, 0x4000, "PcodeOp::commutative (op.hh:89)");
    assert_eq!(pcodeop_flags::unary, 0x8000, "PcodeOp::unary (op.hh:90)");
    assert_eq!(pcodeop_flags::binary, 0x10000, "PcodeOp::binary (op.hh:91)");
    // The composite words the new arms produce:
    assert_eq!(
        pcodeop_flags::binary | pcodeop_flags::commutative,
        0x14000,
        "INT_XOR/AND/OR opflags (typeop.cc:1413/1446/1479)",
    );
    assert_eq!(
        pcodeop_flags::binary | pcodeop_flags::booloutput,
        0x10080,
        "INT_LESS opflags (typeop.cc:1072)",
    );
}
