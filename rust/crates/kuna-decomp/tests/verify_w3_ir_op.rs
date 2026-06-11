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
