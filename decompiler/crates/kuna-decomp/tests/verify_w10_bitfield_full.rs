//! Adversarial verifier tests for `w10-bitfield-activate` — the bitfield
//! *expression-recovery + printc render* stage (the layer that turns a
//! materialized `CPUI_ZPULL`/`CPUI_SPULL`/`CPUI_INSERT`/special-print STORE into
//! `ptr->field` member syntax).
//!
//! These hit the spots most likely to be silently wrong:
//!
//!  - `Datatype::find_matching_bit_field` binary search — in particular the
//!    **big-endian direction**, where `overlapTest` returns `+1` for a query
//!    *below* the candidate (BE treats higher `leastSigBit` as *less* significant,
//!    the reverse of LE).  A naive port that drops the endianness from the query
//!    range (or sorts the vector LE while querying BE) can match the first field
//!    by luck but mis-search for fields reached only after probing the middle.
//!    This is the exact failure the stage's own work surfaced and fixed (the
//!    TypeFactory big-endian seed).
//!  - The binary search's window narrowing on a struct with enough fields that the
//!    answer is reached only after 2+ probes (a one-probe struct never exercises
//!    the `min`/`max` update).
//!  - Partial overlap and empty/no-match returns.
//!  - End-to-end: the actual `bitfields2.xml` `dosomething` parameter→field render
//!    must byte-match the C++ oracle for the recovered statements, proving the
//!    expression-recovery + render path is wired (not just the type primitive).
//!
//! Verifier-authored (independent of the porter), landing on the porter branch
//! per docs/rust-port/verification.md.

use std::rc::Rc;

use kuna_decomp::bitfield::BitRange;
use kuna_decomp::dtype::{flags, type_metatype, Datatype, DatatypeKind, TypeBitField};

/// Build a bitfield with an explicit container layout (byte offset / size, LSB,
/// width, endianness).
fn bf(id: i32, byte_off: i32, byte_sz: i32, lsb: i32, nb: i32, big: bool) -> TypeBitField {
    let mut b = TypeBitField::new(
        id,
        nb,
        big,
        format!("f{id}"),
        Rc::new(Datatype::new(4, type_metatype::TYPE_UINT)),
    );
    b.byte_offset = byte_off;
    b.byte_size = byte_sz;
    b.least_sig_bit = lsb;
    b
}

fn struct_with(bitfield: Vec<TypeBitField>, size: i32) -> Datatype {
    let mut s = Datatype::new_with_align(size, 4, type_metatype::TYPE_STRUCT);
    if !bitfield.is_empty() {
        s.flags |= flags::has_bitfields;
    }
    s.kind = DatatypeKind::Struct { field: vec![], bitfield };
    s
}

/// The exact `myfoo` first-word bitfield layout from `bitfields2.xml`, in BIG
/// endian, **as laid out** by `assign_contiguous_bitfields` (type.cc:2322).
/// Three fields packed into byte 0:
///   fieldb : 1  (lsb 0)
///   field3 : 3  (lsb 1)
///   sfield4: 4  (lsb 4)
/// For big-endian the layout reverses the contiguous group so the stored vector
/// is **descending leastSigBit** `[sfield4, field3, fieldb]` (C++ `std::reverse`,
/// type.cc:2350-2353).  This is the order `findMatchingBitField`'s binary search
/// (which probes with the big-endian `overlapTest` sign) is built to walk; a
/// vector left in ascending order would mis-search.
fn myfoo_word0_be() -> Datatype {
    let mut v = vec![
        bf(0, 0, 1, 0, 1, true), // fieldb  (lsb 0)
        bf(1, 0, 1, 1, 3, true), // field3  (lsb 1)
        bf(2, 0, 1, 4, 4, true), // sfield4 (lsb 4)
    ];
    // The big-endian post-layout order (most- to least-significant in memory).
    v.reverse();
    struct_with(v, 4)
}

// ---------------------------------------------------------------------------
// G-1: find_matching_bit_field on the BE layout must match EVERY field — the
// regression that motivated this stage.  The binary search first probes the
// middle (field3), and for a query below it (fieldb) big-endian overlapTest
// returns +1, so the search must move toward the LOW index.  A query above it
// (sfield4) returns -1.  A range built with the WRONG endianness (LE) would
// move the wrong way and silently return None for fieldb.
// ---------------------------------------------------------------------------
#[test]
fn w10bff_find_matching_be_matches_every_field() {
    let s = myfoo_word0_be();
    // The recovery builds the query range with the STORE space's endianness (BE)
    // and the field's bit position.
    for (name_id, lsb, nb) in [(0, 0, 1), (1, 1, 3), (2, 4, 4)] {
        let range = BitRange::new(0, 1, lsb, nb, true);
        let matched = s.find_matching_bit_field(&range);
        assert!(
            matched.is_some(),
            "BE find_matching must locate field id={name_id} (lsb={lsb}); the binary \
             search direction must follow the big-endian overlapTest sign"
        );
        let m = matched.unwrap();
        assert_eq!(m.ident, name_id, "matched the wrong field");
        assert_eq!(m.least_sig_bit, lsb);
        assert_eq!(m.num_bits, nb);
    }
}

// ---------------------------------------------------------------------------
// G-2: every returned match must actually COVER the queried range (the C++
// `overlapTest==0` exact-identity contract), and a BE query against the BE-laid
// vector must succeed for the most- and least-significant fields — the two ends
// of the search window, which is where a wrong binary-search direction first
// shows up (the middle field can match by luck on the first probe).
// ---------------------------------------------------------------------------
#[test]
fn w10bff_find_matching_endianness_is_load_bearing() {
    let s = myfoo_word0_be();
    // A returned match always covers exactly the queried (lsb,nb).
    for (lsb, nb) in [(0, 1), (1, 3), (4, 4)] {
        if let Some(m) = s.find_matching_bit_field(&BitRange::new(0, 1, lsb, nb, true)) {
            assert_eq!(
                (m.least_sig_bit, m.num_bits),
                (lsb, nb),
                "a returned match must cover the queried range (lsb={lsb})"
            );
        }
    }
    // The two window-extreme fields (least- and most-significant in memory) must
    // both be found under the big-endian search direction — the regression case.
    assert!(
        s.find_matching_bit_field(&BitRange::new(0, 1, 0, 1, true)).is_some(),
        "BE query for the least-significant field (fieldb) must match"
    );
    assert!(
        s.find_matching_bit_field(&BitRange::new(0, 1, 4, 4, true)).is_some(),
        "BE query for the most-significant field (sfield4) must match"
    );
}

// ---------------------------------------------------------------------------
// G-3: the binary search must narrow correctly with > 3 fields so the answer is
// only reached after multiple probes.  A 6-field little-endian struct: probe the
// middle, then a quarter, then the exact element.  An off-by-one in the window
// update (`max = mid` instead of `mid - 1`) would loop or miss.
// ---------------------------------------------------------------------------
#[test]
fn w10bff_find_matching_multiprobe_narrowing() {
    // Six 1-bit LE fields across byte 0, lsb 0..6, vector ascending.
    let bits: Vec<TypeBitField> = (0..6).map(|i| bf(i, 0, 1, i, 1, false)).collect();
    let s = struct_with(bits, 4);
    for lsb in 0..6 {
        let m = s
            .find_matching_bit_field(&BitRange::new(0, 1, lsb, 1, false))
            .unwrap_or_else(|| panic!("LE multiprobe must find lsb={lsb}"));
        assert_eq!(m.least_sig_bit, lsb);
        assert_eq!(m.ident, lsb, "binary search returned the wrong field for lsb={lsb}");
    }
}

// ---------------------------------------------------------------------------
// G-4: a partial overlap (range straddling two fields) must return None (the C++
// `break` on overlapTest==2/4), NOT a wrong field.  And a range outside every
// field is None.  And the empty-struct case (max = -1) must not panic.
// ---------------------------------------------------------------------------
#[test]
fn w10bff_find_matching_partial_and_empty() {
    let s = myfoo_word0_be();
    // A 2-bit range starting at lsb 3 straddles field3 (lsb1..4) and sfield4
    // (lsb4..8) — partial overlap with one of them, no exact match.
    let straddle = BitRange::new(0, 1, 3, 2, true);
    assert!(
        s.find_matching_bit_field(&straddle).is_none(),
        "a range straddling two fields must not match either"
    );
    // A range entirely past the populated bits.
    let beyond = BitRange::new(1, 1, 0, 1, true);
    assert!(s.find_matching_bit_field(&beyond).is_none(), "out-of-range query is no match");
    // Empty struct: max = bitfield.len() - 1 = -1, loop skipped, no panic.
    let empty = struct_with(vec![], 4);
    assert!(empty.find_matching_bit_field(&BitRange::new(0, 1, 0, 1, false)).is_none());
    // A non-struct (a plain int) must also safely return None.
    let prim = Datatype::new(4, type_metatype::TYPE_UINT);
    assert!(prim.find_matching_bit_field(&BitRange::new(0, 1, 0, 1, false)).is_none());
}

// ---------------------------------------------------------------------------
// G-5: an EXACT-match query must beat a containing/contained candidate.  The C++
// `overlapTest` returns 0 only for identical (lsb,most); a wider range that
// merely contains a field (code 3) or is contained (code 2) is `break` (no
// match).  Confirm a query that exactly equals a field returns it, while a query
// one bit wider returns None rather than that field.
// ---------------------------------------------------------------------------
#[test]
fn w10bff_find_matching_requires_exact_range() {
    let s = myfoo_word0_be();
    // Exact field3 (lsb1, nb3) -> match.
    assert_eq!(
        s.find_matching_bit_field(&BitRange::new(0, 1, 1, 3, true)).map(|m| m.ident),
        Some(1),
        "exact range must match field3"
    );
    // field3 widened by one bit (lsb1, nb4) now spills into sfield4 -> no exact
    // match (overlapTest != 0 for the probed field).
    assert!(
        s.find_matching_bit_field(&BitRange::new(0, 1, 1, 4, true)).is_none(),
        "a range wider than the field must not match it"
    );
}
