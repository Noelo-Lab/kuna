//! Round-3 verifier adversarial tests for item `w7-s6-varmap`.
//!
//! Round 1 REJECTed on F1 (sstart scale chain divide-vs-multiply) and F2 (merge
//! resType==2 `diff + b.size` bare int4 add).  Round 2 confirmed F1 fixed and
//! held F2 open as the blocker.  The round-3 repair commit (`25dfd04`) switched
//! both occurrences to `diff.wadd(b.size)` (varmap.rs:495-496).
//!
//! This round re-derives the F2 repair INDEPENDENTLY and additionally hunts the
//! remaining width/wrap spots the prior rounds only flagged as observations:
//!
//!   * F2 repair, the GROWING case: a non-overflowing-but-displaced confuse
//!     frame where `diff + b.size` legitimately grows `size`; assert the exact
//!     grown size (a `wadd` that silently dropped a carry, or that was actually a
//!     `checked_add`, would be caught here — not just "no panic").
//!   * the `isConstAbsorbable` width divergence (varmap.cc:47): C++ computes
//!     `highind * getAlignSize()` as `int4 * int4` (32-bit, truncating), then
//!     widens to `intb`; the Rust widens both to `intb` FIRST then multiplies in
//!     64-bit.  Characterize the (degenerate-only) divergence and pin that the
//!     32-bit-safe regime agrees exactly.
//!   * `contain`'s `b->sstart + b->size - 1 <= sstart + size - 1` at the i64
//!     boundary (varmap.cc:115): wrap-prone `intb` arithmetic.
//!   * `absorb`'s COPY_CONSTANT `b->sstart - sstart + b->size` highind growth
//!     (varmap.cc:238): the `wsub().wadd()` chain must match C++ wrap + the
//!     `/= getAlignSize` and `int4 trialhi` truncation.

use std::rc::Rc;

use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactoryImpl};
use kuna_decomp::varmap::{RangeHint, RangeType, COPY_CONSTANT};

fn space_ws(ws: u32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        8, // address size
        ws,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

/// `new_with_align` pins `align_size = size`, so `get_align_size()` returns
/// `size`.  Tests that need a specific align-size set it via the size argument.
fn base(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

// ===========================================================================
// F2 repair, the GROWING case.
//
// C++ varmap.cc:298-313 resType==2:
//     int4 diff = (int4)(b->sstart - sstart);
//     if (diff + b->size > size) size = diff + b->size;     // int4 + int4
//     if (size not in {1,2,4,8}) { size = 1; rangeType = open; }
//
// Choose a frame where the confuse path is taken and `diff + b.size` genuinely
// exceeds `size`, so the `wadd` result is WRITTEN to `size` (not merely not
// panicking).  diff small + positive, well inside i32:
//   a: sstart 0, size 4, unlocked  (Fixed)
//   b: sstart 4, size 8, unlocked  (Fixed) -> b_end = 4+8-1 = 11 > a_end = 3,
//      and sstart != b.sstart, so contain(b) is FALSE -> a not type-locked ->
//      resType = 2.
//   diff = (int4)(4-0) = 4;  diff + b.size = 4 + 8 = 12 > size(4) -> size = 12.
//   12 is NOT a power of two in {1,2,4,8} -> size = 1, rangeType = open.
// A faithful port lands on size==1 + Open via the exact 12 intermediate; a wadd
// that produced the wrong 12 (e.g. a stray mask) would land on a different
// power-of-two branch and FAIL.
// ===========================================================================
#[test]
fn r3_merge_confuse_grows_size_then_opens() {
    let space = space_ws(1);
    let factory = factory();

    let mut a = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b = RangeHint::new(4, 8, 4, base(8, type_metatype::TYPE_UINT), 0, RangeType::Fixed, -1);

    // Confirm the geometry forces resType==2 (no containment, a unlocked).
    assert!(!a.contain(&b), "b_end 11 > a_end 3 and sstart differ: no containment");

    let ret = a.merge(&b, &space, &factory).unwrap();
    assert!(!ret, "merge always returns false");
    // diff+b.size = 12 -> size=12 -> not a power of 2 -> size=1, Open.
    assert_eq!(a.size, 1, "12 is not in {{1,2,4,8}} so size collapses to 1");
    assert_eq!(a.range_type, RangeType::Open, "non-power-of-two size opens the range");
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UNKNOWN);
    assert_eq!(a.highind, -1, "confuse path sets highind = -1");
    assert_eq!(a.flags, 0, "confuse path clears flags");
}

// A second confuse case where the grown size LANDS on a power of two (stays
// Fixed), so we read the grown `size` directly off the result.
//   a: sstart 0, size 2;  b: sstart 2, size 6 -> b_end 7 > a_end 1, no contain.
//   diff = 2; diff + b.size = 2 + 6 = 8 > size(2) -> size = 8 (a power of two).
//   8 is in {1,2,4,8} -> stays Fixed, UNKNOWN(8).
// This pins the exact arithmetic value 8 that `wadd` must produce.
#[test]
fn r3_merge_confuse_grows_size_to_power_of_two() {
    let space = space_ws(1);
    let factory = factory();

    let mut a = RangeHint::new(0, 2, 0, base(2, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b = RangeHint::new(2, 6, 2, base(6, type_metatype::TYPE_UINT), 0, RangeType::Fixed, -1);

    assert!(!a.contain(&b));
    let _ = a.merge(&b, &space, &factory).unwrap();
    assert_eq!(a.size, 8, "diff(2) + b.size(6) = 8 is written to size");
    assert_eq!(a.range_type, RangeType::Fixed, "8 is a power of two: stays fixed");
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UNKNOWN);
}

// ===========================================================================
// isConstAbsorbable width: 32-bit-safe regime must agree exactly.
//
// C++ varmap.cc:45-50:
//     intb end = sstart;
//     if (highind > 0) end += highind * type->getAlignSize();   // int4 * int4
//     else             end += size;
//     if (b->sstart > end) return false;
//     return true;
//
// In the regime where highind*alignSize stays within i32 (the only regime a
// real stack frame ever reaches), the 32-bit C++ product and the 64-bit Rust
// product are identical, so the absorbability boundary must match to the byte.
// Pin the boundary: end = sstart + highind*alignSize is the LAST absorbable
// b->sstart (`>` is strict), and one past it is the first rejection.
// ===========================================================================
#[test]
fn r3_const_absorbable_highind_mul_boundary_32bit_safe() {
    // self: open primitive int, sstart 0x100, align_size 4, highind 3.
    //   end = 0x100 + 3*4 = 0x100 + 12 = 0x10C.
    let mut me = RangeHint::new(0x100, 4, 0x100, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 3);

    // b must be a COPY_CONSTANT, unlocked, b.size >= self.size, meta unknown/int/uint.
    let mk_b = |sstart: i64| {
        RangeHint::new(
            sstart as u64,
            4, // == self.size, satisfies b.size < size check (not <)
            sstart,
            base(4, type_metatype::TYPE_UNKNOWN),
            COPY_CONSTANT,
            RangeType::Fixed,
            -1,
        )
    };

    // b->sstart exactly at end (0x10C): `b->sstart > end` is FALSE -> absorbable.
    assert!(me.is_const_absorbable(&mk_b(0x10C)), "sstart == end is still absorbable (strict >)");
    // one past end: rejected.
    assert!(!me.is_const_absorbable(&mk_b(0x10D)), "sstart one past end is rejected");
    // well inside: absorbable.
    assert!(me.is_const_absorbable(&mk_b(0x104)));

    // highind <= 0 path uses `end += size` instead of the multiply.
    me.highind = 0;
    // end = 0x100 + size(4) = 0x104.
    assert!(me.is_const_absorbable(&mk_b(0x104)), "highind<=0 uses size, end=0x104");
    assert!(!me.is_const_absorbable(&mk_b(0x105)), "one past size-derived end is rejected");
}

// ===========================================================================
// isConstAbsorbable width DIVERGENCE characterization (degenerate-only).
//
// When highind*alignSize overflows i32, C++ truncates the product to 32 bits
// (and on the int4*int4 == 0x1_0000_0000 case the low 32 bits are 0), while the
// Rust widens to i64 first and keeps the full product.  This flips the
// absorbability decision.  It requires highind*alignSize >= 2^31, which on a
// real frame (alignSize <= 8) needs highind >= 2^28 -- an impossible stack array
// index; both the C++ and Rust only reach it on a synthetic degenerate type.
//
// This test documents the KNOWN, UNREACHABLE divergence rather than asserting
// C++ parity: it records that the Rust takes the WIDE branch.  If a future fix
// narrows the multiply to int4, this test flips (and the verdict's observation
// is resolved).  align_size is forced to 0x10000 via the size argument, and
// highind 0x10000 makes int4*int4 == 0x1_0000_0000 -> low32 == 0.
// ===========================================================================
#[test]
fn r3_const_absorbable_int4_mul_overflow_is_wide_in_rust() {
    // self: "primitive int" of align_size 0x10000 (degenerate), highind 0x10000.
    //   C++ 32-bit product: (int4)(0x10000 * 0x10000) = 0 -> end = sstart + 0.
    //   Rust 64-bit product: 0x10000 * 0x10000 = 0x1_0000_0000 -> end = sstart + 0x1_0000_0000.
    let me = RangeHint::new(0, 0x10000, 0, base(0x10000, type_metatype::TYPE_INT), 0, RangeType::Open, 0x10000);

    // b at sstart 0x100: bigger than the C++ wrapped end (0), smaller than the
    // Rust wide end (0x1_0000_0000).  b.size must be >= self.size (0x10000).
    let b = RangeHint::new(
        0x100,
        0x10000,
        0x100,
        base(0x10000, type_metatype::TYPE_UNKNOWN),
        COPY_CONSTANT,
        RangeType::Fixed,
        -1,
    );

    // Rust (wide): end = 0x1_0000_0000, 0x100 > end is FALSE -> absorbable (true).
    // C++ (narrow): end = 0, 0x100 > 0 is TRUE -> NOT absorbable (false).
    // This asserts the CURRENT Rust behavior and documents the divergence.
    assert!(
        me.is_const_absorbable(&b),
        "Rust widens highind*alignSize to i64 (end = sstart + 0x1_0000_0000) so b is \
         absorbable; C++ truncates the int4*int4 product to 0 and would REJECT. \
         Divergence is unreachable on real frames (alignSize<=8 => highind>=2^28)."
    );
}

// ===========================================================================
// contain boundary arithmetic (varmap.cc:115).
//   if (b->sstart + b->size - 1 <= sstart + size - 1) return true;
// intb arithmetic; b->size/size are int4 promoted to intb.  Test the exact
// boundary: when b ends exactly at a's end, contain is TRUE; one byte past,
// FALSE.  Also the sstart==b.sstart short-circuit.
// ===========================================================================
#[test]
fn r3_contain_end_boundary_exact() {
    // a: sstart 0, size 8 -> a_end = 7.
    let a = RangeHint::new(0, 8, 0, base(8, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);

    // b ends exactly at 7: sstart 4, size 4 -> b_end = 4+4-1 = 7 <= 7 -> contained.
    let b_in = RangeHint::new(4, 4, 4, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    assert!(a.contain(&b_in), "b_end 7 == a_end 7: contained");

    // b ends one past: sstart 4, size 5 -> b_end = 8 > 7 -> NOT contained (and
    // sstart 4 != 0, so no short-circuit).
    let b_out = RangeHint::new(4, 5, 4, base(5, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    assert!(!a.contain(&b_out), "b_end 8 > a_end 7 and sstarts differ: not contained");

    // sstart==b.sstart short-circuits to true regardless of size.
    let b_eq = RangeHint::new(0, 100, 0, base(8, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    assert!(a.contain(&b_eq), "sstart == b.sstart short-circuits to true");
}

// ===========================================================================
// absorb COPY_CONSTANT highind growth (varmap.cc:237-243).
//   if ((b->flags & copy_constant)!=0 && rangeType == open) {
//     intb diffsz = b->sstart - sstart + b->size;
//     if (diffsz > size) {
//       int4 trialhi = diffsz / type->getAlignSize();
//       if (highind < trialhi) highind = trialhi;
//     }
//   }
// `b->sstart - sstart + b->size` is intb (wraps); diffsz/getAlignSize is intb;
// `int4 trialhi` truncates.  Pin the exact grown highind.
//   self: open, sstart 0, size 4, align_size 4, highind 1.
//   b: COPY_CONSTANT, NOT open (Fixed), sstart 0x20, size 4.
//     diffsz = 0x20 - 0 + 4 = 0x24 (36) > size(4) -> trialhi = 36/4 = 9.
//     highind(1) < 9 -> highind = 9.
// ===========================================================================
#[test]
fn r3_absorb_copy_constant_grows_highind() {
    let mut me = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 1);
    // b is Fixed (NOT open) with copy_constant set; takes the else-if branch.
    let b = RangeHint::new(0x20, 4, 0x20, base(4, type_metatype::TYPE_INT), COPY_CONSTANT, RangeType::Fixed, -1);

    me.absorb(&b);
    assert_eq!(me.highind, 9, "diffsz 0x24 / align_size 4 = 9 > old highind 1");
    // copy_constant of b does not propagate when self lacks it AND b has it:
    //   `if ((flags & cc)!=0 && (b->flags & cc)==0)` -- self has no cc, so no xor.
    assert_eq!(me.flags & COPY_CONSTANT, 0, "self never had copy_constant; xor guard requires self to have it");
}

// Companion: the copy_constant XOR-clear (varmap.cc:245-247).
//   if ((flags & copy_constant)!=0 && (b->flags & copy_constant)==0) flags ^= copy_constant;
// self HAS copy_constant, b does NOT -> the bit is cleared.
#[test]
fn r3_absorb_clears_self_copy_constant_when_b_lacks_it() {
    // Make self Fixed so the open/copy-const branches are skipped; only the
    // trailing XOR-clear runs.
    let mut me = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), COPY_CONSTANT, RangeType::Fixed, -1);
    let b = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    me.absorb(&b);
    assert_eq!(me.flags & COPY_CONSTANT, 0, "self had cc, b lacked it -> cleared by xor");
}
