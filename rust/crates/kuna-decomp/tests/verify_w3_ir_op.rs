//! Verifier adversarial tests for item `w3-ir-op` (op.cc/op.hh).
//!
//! INDEPENDENT of the porter's in-module tests. These drive only the public
//! `op` API and target the hunt-list spots the review flagged as most fragile:
//!
//!  - `getNZMaskLocal` extended-precision INT_RIGHT branch (op.cc:636-654): the
//!    Rust uses a *plain* `>>=` (op.rs:1149) where the C++ relies on x86 shift
//!    masking for counts >= 64. For a varnode wider than 16 bytes with a large
//!    constant shift the Rust shift count exceeds 63 => DEBUG PANIC vs a defined
//!    (platform-masked) C++ result. (BLOCKER candidate.)
//!  - `getCseHash` rotate-left determinism + the const-offset vs create-index
//!    input mixing (op.cc:153-170): order-sensitive, width-sensitive.
//!  - `moveSequenceDead` splice fidelity at the HEAD boundary (move-to-front),
//!    a position the porter's in-module test (move-to-middle) does not cover.
//!  - `markIncidentalCopy` over a range whose endpoints are themselves COPYs and
//!    whose `lastop` is the dead-list tail (enditer == list end).
//!  - `isCollapsible` output-size boundary at exactly sizeof(uintb)==8.
//!
//! Several op.cc methods (collapse/executeSimple/PieceNode/compareOrder/
//! nextOp/.../fallthru) are seam-deferred to W3-block/W6 and not exercised here;
//! their accountability gap is recorded as a LOSS by the verdict, not a test.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintm};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::op::{
    get_cse_hash, get_nz_mask_local, is_collapsible, pcodeop_flags, PcodeOpBank,
};
use kuna_decomp::seams::TypeOp;
use kuna_decomp::varnode::VarnodeBank;

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// const(0), unique(1), ram(2, 8-byte addressable so we can hold wide offsets).
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
    m
}

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(2).unwrap()), off)
}

/// A constant varnode of `size` bytes holding `val`: its nzmask == offset == val
/// (Varnode::new seeds a constant's nzm from its offset).
fn konst(vbank: &mut VarnodeBank, m: &AddrSpaceManager, size: int4, val: u64) -> kuna_decomp::seams::VarnodeId {
    let caddr = Address::new(Rc::clone(m.get_space(0).unwrap()), val);
    vbank.create(size, caddr, dt(size))
}

/// A free (non-constant) varnode in ram: nzm defaults to full mask (u64::MAX).
fn freevn(vbank: &mut VarnodeBank, m: &AddrSpaceManager, size: int4, off: u64) -> kuna_decomp::seams::VarnodeId {
    vbank.create(size, ram(m, off), dt(size))
}

fn no_loop(_: int4) -> bool {
    false
}

/// Build an op (via the bank, the only public opcode-setting path) with a given
/// opcode + cached `extra_flags`, an output varnode, and the listed inputs.
/// Returns the op id; the caller fetches the `&PcodeOp` from the bank.
fn build_op(
    bank: &mut PcodeOpBank,
    opc: OpCode,
    extra_flags: u32,
    out: Option<kuna_decomp::seams::VarnodeId>,
    inputs: &[kuna_decomp::seams::VarnodeId],
    m: &AddrSpaceManager,
) -> kuna_decomp::seams::OpId {
    let id = bank.create_at(inputs.len() as int4, ram(m, 0x100));
    // change_opcode caches TypeOp::flags into op.flags (eval-type bits, etc.)
    bank.change_opcode(id, TypeOp::new(opc, extra_flags, format!("{opc:?}")));
    let op = bank.get_mut(id).unwrap();
    op.set_output(out);
    for (i, &v) in inputs.iter().enumerate() {
        op.set_input(Some(v), i as int4);
    }
    id
}

// ---------------------------------------------------------------------------
// F: getNZMaskLocal extended-precision INT_RIGHT plain-shift panic
// ---------------------------------------------------------------------------

/// op.cc:642-644 — wide (>16 byte) input, large constant shift count: the C++
/// does `resmask >>= (sa - 8*sizeof(uintb))` and relies on x86 masking the
/// count to 6 bits when `sa-64 >= 64`. The Rust `op.rs:1149` plain `>>=` panics
/// in debug for the same inputs. With size=17, sa=130: `sa-64 == 66 >= 64`.
///
/// This is the verifier's required failing-test for the REJECT.
#[test]
fn nzmask_int_right_wide_large_shift_diverges() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 17, 0x1000); // size 17 (> sizeof uintb), nz=full
    let sh = konst(&mut vbank, &m, 17, 130); // constant shift amount 130
    let outp = freevn(&mut vbank, &m, 17, 0x2000); // output size 17
    let id = build_op(&mut bank, OpCode::CPUI_INT_RIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();

    // C++ (x86): sa-64 == 66, masked to 66 & 63 == 2, so result == calc_mask(9) >> 2.
    // We assert the *C++-faithful* result; the Rust port panics here instead,
    // so this test FAILS, evidencing the divergence (verdict REJECT, F-getnz).
    let got = get_nz_mask_local(op, &vbank, false, &no_loop);
    let cpp_expected = u64::MAX >> 2; // calc_mask(9)==full, masked-shift >> 2
    assert_eq!(
        got, cpp_expected,
        "INT_RIGHT wide/large-shift must match x86-masked C++ result"
    );
}

/// Sibling positive case: a wide input with a moderate shift (sa-64 < 64) must
/// already agree with C++ today (no panic). Pins the in-range behavior so a fix
/// to the panic case cannot silently regress this one. size=17, sa=70 ->
/// sa-64 == 6.
#[test]
fn nzmask_int_right_wide_moderate_shift_ok() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 17, 0x1000);
    let sh = konst(&mut vbank, &m, 17, 70);
    let outp = freevn(&mut vbank, &m, 17, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_INT_RIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();
    // sz1=17>8, sa=70: pcode_right(full,70)=0; sa<8*17=136, sa>=64 ->
    // resmask=calc_mask(9)=full; resmask >>= 70-64 == 6.
    let got = get_nz_mask_local(op, &vbank, false, &no_loop);
    assert_eq!(got, u64::MAX >> 6);
}

/// op.cc:657-669 — INT_SRIGHT with a constant shift count >= 64 on a small
/// (<= 8 byte) value. The C++ does an unguarded `(fullmask >> sa) ^ fullmask`
/// (op.cc:668) and relies on x86 masking sa to 6 bits; the Rust `op.rs:1172`
/// plain `>> sa` panics in debug for sa >= 64. Second instance of the same
/// missing-wrapping class as INT_RIGHT.
#[test]
fn nzmask_int_sright_const_shift_ge_64_diverges() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    // 8-byte value with an unknown sign bit (nz=full), constant shift 64.
    let inp = freevn(&mut vbank, &m, 8, 0x1000); // nz=full -> sign bit unknown
    let sh = konst(&mut vbank, &m, 8, 64); // shift amount == 64
    let outp = freevn(&mut vbank, &m, 8, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_INT_SRIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();
    // C++ (x86): sa=64 masks to 0; pcode_right(full,64)=0; (full>>0)^full == 0;
    // resmask == 0 | 0 == 0.
    let got = get_nz_mask_local(op, &vbank, false, &no_loop);
    assert_eq!(got, 0u64, "INT_SRIGHT sa>=64 must match x86-masked C++ result");
}

// ---------------------------------------------------------------------------
// F: getCseHash rotate + const/id mixing
// ---------------------------------------------------------------------------

/// op.cc:153-170 — hash seeds with (outSize<<8)|code, then for each input
/// rotate-left-8 and XOR in (offset for constants / createIndex otherwise).
/// Two INT_ADDs with the SAME constant inputs in the SAME order must hash equal;
/// swapping the input order must (generally) change the hash (rotate is
/// order-sensitive). COPY must hash 0.
#[test]
fn cse_hash_order_sensitive_and_copy_zero() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let out = freevn(&mut vbank, &m, 4, 0x10);
    let c1 = konst(&mut vbank, &m, 4, 0x11);
    let c2 = konst(&mut vbank, &m, 4, 0x22);

    // binary eval-type cached so getCseHash doesn't early-return 0.
    let bin = pcodeop_flags::binary;
    let add = build_op(&mut bank, OpCode::CPUI_INT_ADD, bin, Some(out), &[c1, c2], &m);
    let add2 = build_op(&mut bank, OpCode::CPUI_INT_ADD, bin, Some(out), &[c1, c2], &m);
    let add_swapped =
        build_op(&mut bank, OpCode::CPUI_INT_ADD, bin, Some(out), &[c2, c1], &m);

    let h = get_cse_hash(bank.get(add).unwrap(), &vbank);
    assert_ne!(h, 0, "binary op with constant inputs must be hashable");
    assert_eq!(
        h,
        get_cse_hash(bank.get(add2).unwrap(), &vbank),
        "identical ops hash equal"
    );
    assert_ne!(
        h,
        get_cse_hash(bank.get(add_swapped).unwrap(), &vbank),
        "rotate-left makes input order matter"
    );

    // COPY must hash to 0 even though it is unary (op.cc:158).
    let cpy = build_op(
        &mut bank,
        OpCode::CPUI_COPY,
        pcodeop_flags::unary,
        Some(out),
        &[c1],
        &m,
    );
    assert_eq!(get_cse_hash(bank.get(cpy).unwrap(), &vbank), 0u32 as uintm);
}

// ---------------------------------------------------------------------------
// F: moveSequenceDead splice to the HEAD (move-to-front) boundary
// ---------------------------------------------------------------------------

/// op.cc:1086-1095 — the porter's in-module test moves a middle range after a
/// middle op. This pins the move-to-front edge: moving [3,4] to right after the
/// head element 0 must produce [0,3,4,1,2,5], and the head/tail invariants must
/// hold (subsequent dead-list iteration is exact and re-traversable).
#[test]
fn move_sequence_dead_to_front_boundary() {
    let m = build_manager();
    let mut bank = PcodeOpBank::new();
    let ops: Vec<_> = (0..6).map(|i| bank.create_at(0, ram(&m, 0x10 * i))).collect();
    // dead == [0,1,2,3,4,5]; move [3,4] after 0 -> [0,3,4,1,2,5]
    bank.move_sequence_dead(ops[3], ops[4], ops[0]);
    let want = vec![ops[0], ops[3], ops[4], ops[1], ops[2], ops[5]];
    assert_eq!(bank.iter_dead().collect::<Vec<_>>(), want);
    // Re-iterate to confirm links are internally consistent both directions.
    assert_eq!(bank.iter_dead().collect::<Vec<_>>(), want);
    assert_eq!(bank.num_dead(), 6);
}

// ---------------------------------------------------------------------------
// F: isCollapsible output-size boundary at sizeof(uintb)==8
// ---------------------------------------------------------------------------

/// op.cc:123 — `if (getOut()->getSize() > sizeof(uintb)) return false`. Output
/// size 8 (== sizeof uintb) must remain collapsible; size 9 must not. Both
/// inputs constant, op is an assignment, not nocollapse.
#[test]
fn is_collapsible_output_size_boundary() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let c0 = konst(&mut vbank, &m, 8, 0x1);
    let c1 = konst(&mut vbank, &m, 8, 0x2);

    let mut bank = PcodeOpBank::new();
    let out8 = freevn(&mut vbank, &m, 8, 0x100);
    let op8 = build_op(&mut bank, OpCode::CPUI_INT_ADD, 0, Some(out8), &[c0, c1], &m);
    assert!(is_collapsible(bank.get(op8).unwrap(), &vbank), "size 8 == sizeof(uintb) collapses");

    let out9 = freevn(&mut vbank, &m, 9, 0x200);
    let op9 = build_op(&mut bank, OpCode::CPUI_INT_ADD, 0, Some(out9), &[c0, c1], &m);
    assert!(!is_collapsible(bank.get(op9).unwrap(), &vbank), "size 9 > sizeof(uintb) does not");

    // No inputs -> not collapsible (op.cc:120).
    let out_ni = freevn(&mut vbank, &m, 4, 0x300);
    let op_ni = build_op(&mut bank, OpCode::CPUI_INT_ADD, 0, Some(out_ni), &[], &m);
    assert!(!is_collapsible(bank.get(op_ni).unwrap(), &vbank));
}

// ---------------------------------------------------------------------------
// F: markIncidentalCopy when lastop is the dead-list tail (enditer == end)
// ---------------------------------------------------------------------------

/// op.cc:1101-1113 — the cursor advances before the flag write; when `lastop`
/// is the tail, `++enditer` is the list end (None). All COPYs in the inclusive
/// range get marked; the range endpoints are both COPYs here.
#[test]
fn mark_incidental_copy_through_tail() {
    let m = build_manager();
    let mut bank = PcodeOpBank::new();
    let a = bank.create_at(1, ram(&m, 0x10));
    let b = bank.create_at(1, ram(&m, 0x20));
    let c = bank.create_at(1, ram(&m, 0x30));
    bank.change_opcode(a, TypeOp::new(OpCode::CPUI_COPY, 0, "copy"));
    bank.change_opcode(b, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "+"));
    bank.change_opcode(c, TypeOp::new(OpCode::CPUI_COPY, 0, "copy"));
    // mark [a, c] where c is the tail: a and c (COPY) marked, b (ADD) skipped.
    bank.mark_incidental_copy(a, c);
    assert!(bank.get(a).unwrap().is_incidental_copy());
    assert!(!bank.get(b).unwrap().is_incidental_copy());
    assert!(bank.get(c).unwrap().is_incidental_copy());
}

// ===========================================================================
// ROUND 2 (re-review): the porter's repair commit fixed F1/F2. These tests
// (1) re-pin the F1/F2 fixes from the public API, and (2) probe the rest of
// getNZMaskLocal's plain shifts that round-1 dismissed as "provably bounded".
// One of them — CPUI_SUBPIECE — uses a SIGNED `sz1 < sizeof(uintb)` comparison
// where the C++ promotes int4 to size_t (UNSIGNED). For a negative sz1 (offset
// >= 2^31) the Rust enters the shift branch and `resmask >>= 8*sz1` (a negative
// count) panics in debug, while C++ takes the else arm (resmask = 0). This is
// the same signedness/missing-guard class as F1/F2 but at structurally-bounded
// reachability (a SUBPIECE truncation offset cannot legitimately reach 2^31).
// ===========================================================================

/// F2 (now FIXED): INT_SRIGHT, sign-bit KNOWN ZERO, sa >= 64 — the sign-zero
/// arm uses `pcode_right` (already count-safe), so this never panicked, but it
/// pins the sibling of the F2-fixed unknown-sign arm. nz with top bit clear,
/// sa = 100: pcode_right(nz,100) == 0.
#[test]
fn nzmask_int_sright_signzero_large_shift_ok() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    // 8-byte CONSTANT input with a known small value -> nz == 0x7f (sign bit clear).
    let inp = konst(&mut vbank, &m, 8, 0x7f);
    let sh = konst(&mut vbank, &m, 8, 100);
    let outp = freevn(&mut vbank, &m, 8, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_INT_SRIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();
    // sign bit of fullmask(8) is clear in 0x7f -> sign-zero arm -> pcode_right(0x7f,100)=0.
    assert_eq!(get_nz_mask_local(op, &vbank, false, &no_loop), 0u64);
}

/// F1 (now FIXED): INT_RIGHT, wide (>16B) input, EXACT boundary sa == 8*sizeof
/// (==64): enters the `sa >= 8*sizeof(uintb)` arm with `sa-64 == 0`, so the
/// shifted mask is unchanged. Pins the lower edge of the F1 fix (a wshr by 0).
#[test]
fn nzmask_int_right_wide_shift_exactly_64() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 17, 0x1000); // sz1 == 17 > 8
    let sh = konst(&mut vbank, &m, 17, 64); // sa == 64 == 8*sizeof(uintb)
    let outp = freevn(&mut vbank, &m, 17, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_INT_RIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();
    // sz1=17>8, sa=64: sa<8*17=136, sa>=64 -> resmask=calc_mask(17-8)=calc_mask(9)=full;
    // resmask >>= (64-64)==0 -> full.
    assert_eq!(get_nz_mask_local(op, &vbank, false, &no_loop), u64::MAX);
}

/// Round-1 claimed INT_RIGHT's `sa >= 8*sz1` at op.rs:1144 and `sa >= 8*usize`
/// at 1146 are bounded. They are panic-safe, but op.rs:1146 is a SIGNED i32
/// comparison where C++ op.cc:640 `sa >= 8*sizeof(uintb)` promotes sa to size_t
/// (UNSIGNED). For a *normal positive* shift this agrees; this test pins the
/// well-formed sa==0 case (no shift) on a wide input so a future fix to the
/// negative-sa divergence cannot regress the common path. sz1=17, sa=0.
#[test]
fn nzmask_int_right_wide_zero_shift_ok() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 17, 0x1000); // nz == full (free)
    let sh = konst(&mut vbank, &m, 17, 0); // sa == 0
    let outp = freevn(&mut vbank, &m, 17, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_INT_RIGHT, 0, Some(outp), &[inp, sh], &m);
    let op = bank.get(id).unwrap();
    // sz1=17>8, sa=0: pcode_right(full,0)=full; sa<8*17, sa<64 -> else arm:
    // tmp = -1 << (64-0) == 0 (wshl masks 64->0; -1<<0 == -1? no: 64&63==0 -> <<0 == all ones)
    // C++: tmp <<= (8*8 - 0)==64 -> x86 masks to 0 -> tmp stays all-ones; resmask |= full = full.
    assert_eq!(get_nz_mask_local(op, &vbank, false, &no_loop), u64::MAX);
}

/// SUBPIECE well-formed boundary: offset exactly sizeof(uintb)==8 on a small
/// (<=8B) input. C++ op.cc:701 `sz1 < sizeof(uintb)` is false (8 < 8 false) ->
/// resmask = 0. Rust line 1212 `sz1 < usize_uintb` (8 < 8) also false -> 0.
/// (Agrees because sz1 is non-negative; pins the positive-side boundary.)
#[test]
fn nzmask_subpiece_offset_at_sizeof_boundary() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 8, 0x1000); // size 8 (<= sizeof uintb)
    let off = konst(&mut vbank, &m, 4, 8); // truncation offset == 8 (== sizeof uintb)
    let outp = freevn(&mut vbank, &m, 1, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_SUBPIECE, 0, Some(outp), &[inp, off], &m);
    let op = bank.get(id).unwrap();
    // sz1=8, in_size(0)=8<=8: sz1<8 false -> resmask=0; &= fullmask(1) -> 0.
    assert_eq!(get_nz_mask_local(op, &vbank, false, &no_loop), 0u64);

    // offset 7 (< 8): resmask = full >> 56 == 0xff, & fullmask(1)==0xff -> 0xff.
    let off7 = konst(&mut vbank, &m, 4, 7);
    let id7 = build_op(&mut bank, OpCode::CPUI_SUBPIECE, 0, Some(outp), &[inp, off7], &m);
    assert_eq!(
        get_nz_mask_local(bank.get(id7).unwrap(), &vbank, false, &no_loop),
        0xffu64
    );
}

/// NEW FINDING (round 2): SUBPIECE with a constant truncation offset whose
/// `(int4)offset` is NEGATIVE (offset >= 2^31). C++ op.cc:701 `sz1 < sizeof
/// (uintb)` promotes the negative sz1 to a huge size_t -> the comparison is
/// FALSE -> `resmask = 0` (op.cc:704). The Rust port line 1212 compares two
/// i32s (`sz1 < usize_uintb`), so a negative sz1 is `< 8` -> it ENTERS
/// `resmask >>= 8*sz1` with a negative count -> DEBUG PANIC (and a wrong value
/// in release). This test asserts the C++-faithful result (0); the Rust panics,
/// so it FAILS, evidencing the divergence.
///
/// Reachability note: a SUBPIECE truncation offset cannot legitimately reach
/// 2^31 bytes, so this is a fidelity/robustness divergence at near-nil
/// reachability, NOT a blocker — see the verdict (severity: minor).
#[test]
#[should_panic] // documents the divergence: C++ returns 0, Rust panics in debug
fn nzmask_subpiece_negative_offset_diverges() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    let inp = freevn(&mut vbank, &m, 8, 0x1000); // small input
    // offset 0x80000000 -> (int4) == i32::MIN (negative). 8-byte const can hold it.
    let off = konst(&mut vbank, &m, 8, 0x8000_0000);
    let outp = freevn(&mut vbank, &m, 1, 0x2000);
    let id = build_op(&mut bank, OpCode::CPUI_SUBPIECE, 0, Some(outp), &[inp, off], &m);
    let op = bank.get(id).unwrap();
    // C++-faithful expectation would be 0. The Rust panics first (this is why the
    // test is #[should_panic]); if the port is ever fixed to match C++, replace
    // with assert_eq!(.., 0) and drop should_panic.
    let _ = get_nz_mask_local(op, &vbank, false, &no_loop);
}

/// INT_MULT total-shift boundary: round-1 said op.rs:1263 `resmask >>= 8*size
/// - total` is bounded by `total < 8*size`. Pin the tightest case: size=8,
/// two single-bit inputs (total collapses to 1) so `8*8 - 1 == 63` (the max
/// legal count). Must not panic and must produce a defined mask.
#[test]
fn nzmask_int_mult_total_shift_max_count() {
    let m = build_manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut bank = PcodeOpBank::new();
    // two constants each with a single bit set at position 0 -> nz == 1.
    let a = konst(&mut vbank, &m, 8, 1);
    let b = konst(&mut vbank, &m, 8, 1);
    let outp = freevn(&mut vbank, &m, 8, 0x2000); // size 8 == sizeof uintb
    let id = build_op(&mut bank, OpCode::CPUI_INT_MULT, 0, Some(outp), &[a, b], &m);
    let op = bank.get(id).unwrap();
    // val=1,resmask=1: msb=0,lsb=0 both; sa=0; sz1b=1,sz2b=1 -> total=2, then -=1 ->1.
    // resmask=full; total(1) < 64 -> resmask >>= (64-1)==63 -> 1; << sa(0) & full -> 1.
    assert_eq!(get_nz_mask_local(op, &vbank, false, &no_loop), 1u64);
}
