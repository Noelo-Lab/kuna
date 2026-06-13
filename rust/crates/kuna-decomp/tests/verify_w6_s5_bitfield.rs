//! Independent verifier tests for item w6-s5-bitfield (round 1).
//!
//! These are *adversarial* re-derivations of the `BitRange` arithmetic and the
//! `BitFieldNodeState` construction matrix, pinned against ground-truth values
//! produced by compiling the actual C++ (`address.cc:630-868`,
//! `bitfield.cc:21-51`) with `g++ -O0`.  They target boundary/clamping paths
//! that the in-tree unit tests do not cover: negative-shift LSB underflow,
//! non-contiguous `intersectMask` minimal cover, big-endian
//! `minimizeContainer`/`truncate*`, the `overlapTest` big-endian containment
//! branch, and the `getMask` high-shift edge.
//!
//! Hunt list focus: signedness (all four geometry fields are `int4`/i32 and go
//! negative mid-computation before clamping), wrapping (`getMask` uses
//! `wrapping_sub`), off-by-one (the `+7)/8` byte-rounding in
//! `minimizeContainer`), iteration-order / comparator totality (`compare`), and
//! the C++-UB-vs-Rust-shift seam in `getMask`.

use kuna_decomp::bitfield::{BitFieldNodeState, BitRange, FieldRef};
use kuna_decomp::seams::VarnodeId;

fn vn() -> VarnodeId {
    VarnodeId::default()
}

/// `shift` with a negative amount that drives `leastSigBit` below zero must
/// *lose* the bits that fall off the low end (clamp lsb to 0 and subtract the
/// underflow from numBits), not wrap or saturate.  C++ oracle (`/tmp/bitoracle`):
///   [4,8) shift(-6) -> lsb 0 nb 6
///   [4,8) shift(-4) -> lsb 0 nb 8   (exact, no loss)
///   [2,4) shift(-10) -> lsb 0 nb 0  (entirely off low end)
/// Also verifies the high-end clamp branch (`else if most > byteSize*8`) and the
/// 1-byte-container overflow case already pinned in-tree, re-checked here for an
/// asymmetric container.
#[test]
fn shift_signed_underflow_and_high_clamp() {
    let mut a = BitRange::new(0, 8, 4, 8, false);
    a.shift(-6);
    assert_eq!((a.least_sig_bit, a.num_bits), (0, 6), "negative shift must drop low bits");

    let mut a2 = BitRange::new(0, 8, 4, 8, false);
    a2.shift(-4);
    assert_eq!((a2.least_sig_bit, a2.num_bits), (0, 8), "exact down-shift loses nothing");

    let mut a3 = BitRange::new(0, 8, 2, 4, false);
    a3.shift(-10);
    assert_eq!((a3.least_sig_bit, a3.num_bits), (0, 0), "shifted entirely off the low end => empty");

    // High-end clamp: lsb stays >= 0, most exceeds container, numBits trimmed.
    // [60,4) in an 8-byte (64-bit) container, shift left by 2 -> [62, ...] most
    // would be 66 > 64, so numBits drops to 2.
    let mut a4 = BitRange::new(0, 8, 60, 4, false);
    a4.shift(2);
    assert_eq!((a4.least_sig_bit, a4.num_bits), (62, 2), "high-end overflow trims numBits");
}

/// `intersectMask` with a *non-contiguous* mask collapses to the minimal cover
/// (leastsigbit_set .. mostsigbit_set), spanning the gap between set bits.  This
/// is the place a naive port could use popcount or a contiguous assumption.
/// C++ oracle: [0,64) & 0x0100000000000081 -> lsb 0, nb 57 (bit 0 .. bit 56).
#[test]
fn intersect_mask_noncontiguous_minimal_cover() {
    let mut b = BitRange::new(0, 8, 0, 64, false);
    b.intersect_mask(0x0100_0000_0000_0081);
    assert_eq!((b.least_sig_bit, b.num_bits), (0, 57), "minimal cover spans lowest..highest set bit");

    // A mask whose lowest set bit is above the current lsb and highest set bit is
    // below current most, so both clamp branches fire.
    let mut b2 = BitRange::new(0, 8, 8, 16, false); // [8,24)
    b2.intersect_mask(0x00f0_0000); // bits 20..24 only (within [8,24))
    assert_eq!((b2.least_sig_bit, b2.num_bits), (20, 4));
}

/// Big-endian `minimizeContainer` shrinks `byteSize` (not `byteOffset`) when
/// truncating the most-significant side, and adjusts `byteOffset` only for the
/// trailing-byte trim.  C++ oracle: BE [16,8) -> off 5, sz 1, lsb 0, nb 8.
/// Also a little-endian case where the field straddles a byte boundary so the
/// `(lsb+nb+7)/8` rounding matters.  C++ oracle: LE lsb12 nb10 -> off1 sz2 lsb4 nb10.
#[test]
fn minimize_container_endian_and_rounding() {
    let mut c = BitRange::new(0, 8, 16, 8, true);
    c.minimize_container();
    assert_eq!(
        (c.byte_offset, c.byte_size, c.least_sig_bit, c.num_bits),
        (5, 1, 0, 8),
        "BE minimize trims byteSize on the high side"
    );

    let mut d = BitRange::new(0, 8, 12, 10, false);
    d.minimize_container();
    assert_eq!(
        (d.byte_offset, d.byte_size, d.least_sig_bit, d.num_bits),
        (1, 2, 4, 10),
        "LE minimize: +7)/8 rounding keeps the straddling high byte"
    );
}

/// Big-endian `truncateLeastSigBytes` must NOT move `byteOffset` (the `!isBigEndian`
/// guard), only shrink `byteSize` and pull down `leastSigBit`.  And a little-endian
/// case where the truncation eats into the field, clamping numBits.
/// C++ oracle: BE [8,16) truncLSB(2) -> off0 sz6 lsb0 nb8;
///             LE (2,6,4,8) truncLSB(1) -> off3 sz5 lsb0 nb4.
#[test]
fn truncate_least_sig_bytes_endian_and_field_eat() {
    let mut e = BitRange::new(0, 8, 8, 16, true);
    e.truncate_least_sig_bytes(2);
    assert_eq!(
        (e.byte_offset, e.byte_size, e.least_sig_bit, e.num_bits),
        (0, 6, 0, 8),
        "BE truncLSB leaves byteOffset, lsb -= 16, numBits clamped"
    );

    let mut f = BitRange::new(2, 6, 4, 8, false);
    f.truncate_least_sig_bytes(1);
    assert_eq!(
        (f.byte_offset, f.byte_size, f.least_sig_bit, f.num_bits),
        (3, 5, 0, 4),
        "LE truncLSB advances byteOffset and eats into the field"
    );
}

/// `overlapTest` big-endian containment codes (2 = this-in-op2, 3 = op2-in-this)
/// must match little-endian semantics even though the early no-intersection
/// branches differ by endianness.  C++ oracle: BE big.overlap(small)=3,
/// small.overlap(big)=2.  Also re-pins the BE "after" code (1).
#[test]
fn overlap_test_big_endian_containment() {
    let big = BitRange::new(0, 4, 0, 16, true);
    let small = BitRange::new(0, 4, 4, 4, true);
    assert_eq!(big.overlap_test(&small), 3, "op2 contained in this");
    assert_eq!(small.overlap_test(&big), 2, "this contained in op2");

    let be1 = BitRange::new(0, 4, 0, 8, true);
    let be2 = BitRange::new(0, 4, 8, 8, true);
    assert_eq!(be1.overlap_test(&be2), 1, "BE: this after op2 (no intersection)");
    assert_eq!(be1.overlap_test(&be1), 0, "identical");
}

/// `getMask` at the high end of the container: lsb 60 nb 4 must produce
/// 0xf000000000000000 (the top nibble), and numBits 63 must produce
/// 0x7fffffffffffffff (the `wrapping_sub(1)` path with a non-zero shifted 1).
/// C++ oracle: those exact values.
#[test]
fn get_mask_high_shift_and_wrapping() {
    assert_eq!(BitRange::new(0, 8, 60, 4, false).get_mask(), 0xf000_0000_0000_0000);
    assert_eq!(BitRange::new(0, 8, 0, 63, false).get_mask(), 0x7fff_ffff_ffff_ffff);
    // numBits >= 64 takes the res=0 branch, wrapping_sub(1) => all ones, then
    // shifted by lsb.
    assert_eq!(BitRange::new(0, 8, 0, 64, false).get_mask(), 0xffff_ffff_ffff_ffff);
    assert_eq!(BitRange::new(0, 8, 8, 64, false).get_mask(), 0xffff_ffff_ffff_ff00);
}

/// `compare` is a *total order* and ignores `isBigEndian` (matching C++).
/// Verify antisymmetry and the field-priority tie-break chain on a handful of
/// pairs, including the endianness-only difference returning 0.
#[test]
fn compare_total_order_antisymmetry() {
    let pairs = [
        (BitRange::new(1, 2, 3, 4, false), BitRange::new(2, 2, 3, 4, false)),
        (BitRange::new(1, 2, 3, 4, false), BitRange::new(1, 3, 3, 4, false)),
        (BitRange::new(1, 2, 3, 4, false), BitRange::new(1, 2, 5, 4, false)),
        (BitRange::new(1, 2, 3, 4, false), BitRange::new(1, 2, 3, 9, false)),
    ];
    for (a, b) in pairs.iter() {
        assert_eq!(a.compare(b), -1, "a < b in declared field priority");
        assert_eq!(b.compare(a), 1, "antisymmetric");
        assert_eq!(a.compare(a), 0, "reflexive");
    }
    // isBigEndian is not a comparison key.
    let le = BitRange::new(7, 7, 7, 7, false);
    let be = BitRange::new(7, 7, 7, 7, true);
    assert_eq!(le.compare(&be), 0, "endianness is not part of the order");
}

/// `BitFieldNodeState::follow_field` construction matrix: the field's `bits`
/// range is copied into the Varnode's byte container via `in_container`
/// (which itself round-trips through `translateLSB`), `origLeastSigBit` records
/// the translated LSB, and `isSignExtended` is `is_int && isMostSignificant`.
/// Cross-check a NON-top-aligned int field (no sign-ext) and a top-aligned one
/// (sign-ext), plus that `does_sign_extension_match` reads the carried `is_int`.
#[test]
fn node_state_follow_field_signext_matrix() {
    let used = BitRange::byte_range(0, 8, false);

    // Field at byteOffset 2 (LE) -> translateLSB 16, not top of 64-bit container.
    let mid = BitFieldNodeState::follow_field(&used, vn(), &BitRange::new(2, 2, 0, 16, false), true);
    assert_eq!((mid.bits_field.least_sig_bit, mid.bits_field.num_bits), (16, 16));
    assert_eq!(mid.orig_least_sig_bit, 16);
    assert!(!mid.is_sign_extended, "not most-significant => no sign extension");
    assert!(!mid.does_sign_extension_match(), "false == true(is_int) => mismatch");

    // Field at byteOffset 6 (LE) -> translateLSB 48, top of the 64-bit container.
    let top = BitFieldNodeState::follow_field(&used, vn(), &BitRange::new(6, 2, 0, 16, false), true);
    assert!(top.bits_field.is_most_significant());
    assert!(top.is_sign_extended, "int field at the top sign-extends");
    assert!(top.does_sign_extension_match());

    // Same top field but unsigned: never sign-extends; match is true (false==false).
    let topu = BitFieldNodeState::follow_field(&used, vn(), &BitRange::new(6, 2, 0, 16, false), false);
    assert!(!topu.is_sign_extended);
    assert_eq!(topu.field, FieldRef::Field { is_int: false });
    assert!(topu.does_sign_extension_match());

    // A hole carries no field; does_sign_extension_match is defensively false.
    let hole = BitFieldNodeState::hole(&used, vn(), 12, 4);
    assert_eq!(hole.field, FieldRef::Hole);
    assert!(!hole.does_sign_extension_match());
}

/// Big-endian `in_container` / `translateLSB` provenance: a BE field copied into
/// an 8-byte BE container translates its LSB by `8*((thisOff+thisSz)-(opOff+opSz))`.
/// C++ oracle (`/tmp/bitcheck2` mirrored here): BE field (2,2,0,16) into (0,8)
/// -> lsb 32.  This pins that the port did not silently use the LE formula.
#[test]
fn in_container_big_endian_translate_provenance() {
    let be_field = BitRange::new(2, 2, 0, 16, true);
    let be = BitRange::in_container(&be_field, 0, 8);
    assert_eq!(
        (be.byte_offset, be.byte_size, be.least_sig_bit, be.num_bits, be.is_big_endian),
        (0, 8, 32, 16, true)
    );

    // LE counterpart for contrast: 8*(2-0) = 16.
    let le_field = BitRange::new(2, 2, 0, 16, false);
    let le = BitRange::in_container(&le_field, 0, 8);
    assert_eq!(le.least_sig_bit, 16);
}
