//! Adversarial verifier tests for item `w6-kuna-s4s5-pack` — the kuna S4/S5
//! sub-stage pack (`kuna_returnpair`, `kuna_memsetsequence`, `kuna_compareform`).
//!
//! The hunt list flagged these as the most fragile spots for this item:
//!
//!   * **Overflow guards / signedness (compareform).** `restoreLessequal`
//!     (`decompiler/cpp/kuna_compareform.cc:42-52`) is the exact inverse of
//!     `Funcdata::replaceLessequal` and mirrors its signed/unsigned overflow
//!     guards in the *inverse* direction (`calc_int_min`/`calc_int_max`/
//!     `calc_uint_max`).  A boundary error here silently rewrites a comparison
//!     that should have been left canonical (or refuses one that should round
//!     trip).  These drive a real `replace_lessequal` -> `ActionPresentCompareForm`
//!     round trip *at* the int-max/int-min/uint-max boundary, across 4- and
//!     8-byte widths.
//!   * **Off-by-one (returnpair gate).** `keep_single_return`
//!     (`kuna_returnpair`, the `buildReturnOutput` resize predicate
//!     `return_single && newparam.size() > 2`) has its sole boundary at size 2
//!     vs 3.  Exhaustively pinned.
//!   * **Comparator totality / exact-token parse.** `OptionReturnPair::apply`
//!     (`kuna_returnpair.cc:20-25`) and `OptionMemsetRecover::apply`
//!     (`kuna_memsetsequence.cc:211-217`) compare `p1` by *exact* string
//!     (`p1 == "single"` / `onOrOff`); no trimming, no case fold.  Adversarial
//!     near-miss tokens confirm the port did not "helpfully" normalize.
//!
//! C++ oracle anchors are cited inline.

use std::rc::Rc;

use kuna_base::address::{calc_int_max, calc_int_min, calc_uint_max, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::{int4, uintb};

use kuna_decomp::action::{Action, ActionContext};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_compareform::{parse_compare_form, ActionPresentCompareForm, CompareForm};
use kuna_decomp::kuna_memsetsequence::{parse_memset_recover_form, MemsetRecoverForm};
use kuna_decomp::kuna_returnpair::{keep_single_return, parse_return_pair_form, ReturnPairForm};
use kuna_decomp::context::{ArchContext, TypeOp};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// Funcdata-construction scaffolding (mirrors the in-crate compareform tests).
// ---------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn no_replace(
) -> impl FnMut(&mut VarnodeBank, kuna_decomp::context::VarnodeId, kuna_decomp::context::VarnodeId) -> kuna_base::error::KunaResult<()>
{
    |_: &mut VarnodeBank, _, _| Ok(())
}

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> kuna_decomp::context::OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn set_def(
    fd: &mut Funcdata,
    vn: kuna_decomp::context::VarnodeId,
    defop: kuna_decomp::context::OpId,
) -> kuna_decomp::context::VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: kuna_decomp::context::VarnodeId, op: kuna_decomp::context::OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn mk_var(fd: &mut Funcdata, off: u64, size: int4) -> kuna_decomp::context::VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    set_def(fd, vn, defop)
}

fn run_present(fd: &mut Funcdata) -> int4 {
    let mut action = ActionPresentCompareForm::new(true, "analysis");
    let mut ctx = ActionContext::new();
    action.apply(fd, &mut ctx);
    action.base().count
}

fn code_of(fd: &Funcdata, op: kuna_decomp::context::OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}
fn const_in(fd: &Funcdata, op: kuna_decomp::context::OpId, slot: int4) -> uintb {
    let v = fd.obank().get(op).unwrap().get_in(slot).unwrap();
    fd.vbank().get(v).unwrap().get_offset()
}

/// Build a binary comparison `op(in0, in1)` where exactly one side is the given
/// constant; returns the op id.  `const_left=true` puts the constant on slot 0.
fn mk_cmp_with_const(
    fd: &mut Funcdata,
    width: int4,
    opc: OpCode,
    cval: uintb,
    const_left: bool,
) -> kuna_decomp::context::OpId {
    let v = mk_var(fd, 0x100, width);
    let c = fd.new_constant(width, cval);
    let op = mk_op(fd, 0x108, 2, opc);
    if const_left {
        wire(fd, c, op, 0);
        wire(fd, v, op, 1);
    } else {
        wire(fd, v, op, 0);
        wire(fd, c, op, 1);
    }
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(fd, out, op);
    op
}

// ===========================================================================
// T1 — compareform: signed round trip AT the int-max boundary, both widths.
//
// `V s<= (INT_MAX-1)` canonicalizes to `V s< INT_MAX` (replaceLessequal, diff
// = +1 on the right constant: INT_MAX-1 + 1 = INT_MAX, and the guard
// `val == calc_int_max` is NOT hit because val is INT_MAX-1).  The inverse
// (restoreLessequal, i=1, diff=-1) must restore exactly `V s<= INT_MAX-1`:
// its guard `diff==-1 && val==calc_int_min` is NOT hit (val is INT_MAX), so
// the restore fires and lands back on the original constant.
//
// This is the closest-to-the-edge case that must still round trip; an
// off-by-one in either guard (e.g. comparing against int_max instead of
// int_min on the restore side) would wrongly decline it.
// Oracle: kuna_compareform.cc:42-46 ; funcdata_op.cc:1049-1053.
// ===========================================================================
#[test]
fn t1_signed_roundtrip_at_intmax_boundary_4_and_8() {
    for &width in &[4i32, 8i32] {
        let int_max = calc_int_max(width); // 0x7fff_ffff (w=4)
        let start = int_max - 1; // INT_MAX-1
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, width, OpCode::CPUI_INT_SLESSEQUAL, start, false);

        // canonicalize: V s<= (INT_MAX-1)  =>  V s< INT_MAX  (marked)
        assert!(fd.replace_lessequal(op).unwrap(), "width {width}: canon should fire");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESS);
        assert_eq!(const_in(&fd, op, 1), int_max, "width {width}: canon constant");
        assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());

        // restore: V s< INT_MAX  =>  V s<= (INT_MAX-1)
        assert_eq!(run_present(&mut fd), 1, "width {width}: restore should fire");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESSEQUAL);
        assert_eq!(const_in(&fd, op, 1), start, "width {width}: restored constant");
        assert!(!fd.obank().get(op).unwrap().is_canonical_lessequal());
    }
}

// ===========================================================================
// T2 — compareform: the restore-side guards actually DECLINE at the edge.
//
// Construct a *marked* `INT_SLESS` whose left constant is exactly INT_MIN.
// This is the shape `c-1 < V` with c-1 = INT_MIN, so the inverse would be
// `c <= V` with c = INT_MIN+1 — but the guard is `diff==1 && val==int_max`
// for the left-constant branch, which is NOT INT_MIN, so it actually *would*
// restore.  The genuinely-declining edges are:
//   (a) unsigned right-constant == 0      (diff=-1, val==0)        -> decline
//   (b) signed   right-constant == INT_MIN(diff=-1, val==int_min)  -> decline
// Both must leave the op untouched AND keep the provenance mark (C++ only
// clears on a successful restore).
// Oracle: kuna_compareform.cc:44 (signed int_min), :49 (unsigned 0).
// ===========================================================================
#[test]
fn t2_restore_guards_decline_and_keep_mark() {
    // (a) unsigned, right constant 0: `V < 0` marked -> would be `V <= -1`(wrap)
    {
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, 4, OpCode::CPUI_INT_LESS, 0, false);
        fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();
        assert_eq!(run_present(&mut fd), 0, "unsigned-0 must decline");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
        assert_eq!(const_in(&fd, op, 1), 0);
        assert!(
            fd.obank().get(op).unwrap().is_canonical_lessequal(),
            "declined restore must NOT clear the mark"
        );
    }
    // (b) signed, right constant INT_MIN: `V s< INT_MIN` marked -> guard kills it
    {
        let width = 4;
        let int_min = calc_int_min(width); // 0x8000_0000
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, width, OpCode::CPUI_INT_SLESS, int_min, false);
        fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();
        assert_eq!(run_present(&mut fd), 0, "signed-INT_MIN must decline");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESS);
        assert_eq!(const_in(&fd, op, 1), int_min);
        assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
    }
    // (c) unsigned, left constant == UINT_MAX: `UINT_MAX < V` marked, diff=+1,
    //     guard `val==calc_uint_max` -> decline (would wrap to 0 on +1).
    {
        let width = 4;
        let umax = calc_uint_max(width); // 0xffff_ffff
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, width, OpCode::CPUI_INT_LESS, umax, true);
        fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();
        assert_eq!(run_present(&mut fd), 0, "unsigned-UINT_MAX-left must decline");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
        assert_eq!(const_in(&fd, op, 0), umax);
        assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
    }
}

// ===========================================================================
// T3 — returnpair gate off-by-one + parse exact-token totality.
//
// keep_single_return == (return_single && newparam_size > 2): the only
// boundary is at size 2 (allow) vs 3 (truncate).  Pin both, and that `pair`
// (return_single=false) never truncates regardless of size.
// Parse: exact-string match; near-miss tokens are errors, not silently
// accepted (no case fold, no trim).
// Oracle: kuna_returnpair (gate predicate) ; kuna_returnpair.cc:20-25.
// ===========================================================================
#[test]
fn t3_returnpair_gate_boundary_and_exact_token_parse() {
    // gate: single + sizes around the size>2 boundary
    assert!(!keep_single_return(true, 0));
    assert!(!keep_single_return(true, 1));
    assert!(!keep_single_return(true, 2), "size==2 is NOT > 2: must keep");
    assert!(keep_single_return(true, 3), "size==3 IS > 2: must truncate");
    assert!(keep_single_return(true, usize::MAX));
    // pair never truncates
    for sz in 0..6usize {
        assert!(!keep_single_return(false, sz), "pair must never truncate (sz={sz})");
    }

    // exact-token parse: only "single"/"pair" accepted
    assert_eq!(parse_return_pair_form("single").unwrap().0, ReturnPairForm::Single);
    assert_eq!(parse_return_pair_form("pair").unwrap().0, ReturnPairForm::Pair);
    // adversarial near-misses must error (C++ `p1 == "single"` is exact)
    for bad in ["Single", "PAIR", " pair", "pair ", "single\n", "", "p", "join", "0"] {
        assert!(
            parse_return_pair_form(bad).is_err(),
            "near-miss token {bad:?} must be rejected (exact match only)"
        );
    }

    // memsetrecover parse: onOrOff semantics — "" and "on" -> On, "off" -> Off,
    // anything else -> error (no case fold).  Oracle: kuna_memsetsequence.cc:214.
    assert_eq!(parse_memset_recover_form("on").unwrap().0, MemsetRecoverForm::On);
    assert_eq!(parse_memset_recover_form("off").unwrap().0, MemsetRecoverForm::Off);
    assert_eq!(parse_memset_recover_form("").unwrap().0, MemsetRecoverForm::On);
    for bad in ["ON", "Off", "yes", "true", "1", " on"] {
        assert!(parse_memset_recover_form(bad).is_err(), "memset near-miss {bad:?} must error");
    }
}

// ===========================================================================
// ROUND 2 — verifier re-derivation of the F1 repair (parse_compare_form) and
// two compareform paths the round-1 suite did not pin.
// ===========================================================================

// ---------------------------------------------------------------------------
// R2-T1 — compareform option-parse (the F1 fix) through the PUBLIC API.
//
// `OptionCompareForm::apply` (kuna_compareform.cc:85-95):
//   if (p1 == "original")       glb->present_lessequal = true;
//   else if (p1 == "canonical") glb->present_lessequal = false;
//   else throw ParseError("Must specify compareform as 'canonical' or 'original'");
//   return "Comparison presentation set to " + p1 + " form";
//
// `original` (kuna DIV-2 default) must map to present_lessequal=TRUE (the gate
// `ActionPresentCompareForm` consumes), `canonical` to FALSE.  The parse is an
// EXACT string match (no case fold, no trim); near-miss tokens are parse errors,
// not silently coerced.  The confirmation string is byte-identical to the C++
// `"Comparison presentation set to " + p1 + " form"`.
// ===========================================================================
#[test]
fn r2_t1_compareform_parse_public_api() {
    // original -> Original, present_lessequal = true, exact message
    let (form, msg) = parse_compare_form("original").unwrap();
    assert_eq!(form, CompareForm::Original);
    assert!(form.present_lessequal(), "original => present_lessequal=true (gate ON)");
    assert_eq!(msg, "Comparison presentation set to original form");
    assert_eq!(form.as_str(), "original");

    // canonical -> Canonical, present_lessequal = false
    let (form, msg) = parse_compare_form("canonical").unwrap();
    assert_eq!(form, CompareForm::Canonical);
    assert!(!form.present_lessequal(), "canonical => present_lessequal=false (gate OFF)");
    assert_eq!(msg, "Comparison presentation set to canonical form");
    assert_eq!(form.as_str(), "canonical");

    // EXACT match only — every near miss is an error carrying the C++ message.
    for bad in ["Original", "CANONICAL", " original", "canonical ", "orig", "", "default", "1"] {
        let e = parse_compare_form(bad).unwrap_err();
        assert!(
            e.to_string().contains("Must specify compareform as 'canonical' or 'original'"),
            "near-miss {bad:?} must surface the C++ ParseError text, got {e}"
        );
    }
}

// ===========================================================================
// R2-T2 — compareform: UNSIGNED, LEFT-constant round trip (`c-1 < V` from
// `c <= V`), the i=0 / diff path round-1 T1 did not exercise.
//
// Forward replaceLessequal on `c <= V` (INT_LESSEQUAL, in0 constant): diff=-1,
// i=0 -> `(c-1) < V`, marked.  Inverse restoreLessequal (in0 constant): diff=+1,
// i=0 -> `c <= V`.  Pick c = 0x40 (mid-range, no boundary) so neither the
// forward `val==0` guard nor the inverse `val==calc_uint_max` guard fires; the
// constant must land back EXACTLY on 0x40 with the LESSEQUAL opcode restored and
// the provenance mark cleared.
// Oracle: funcdata_op.cc:1037-1058 (forward), kuna_compareform.cc:30-51 (inverse).
// ===========================================================================
#[test]
fn r2_t2_unsigned_left_const_roundtrip() {
    let width = 4i32;
    let c: uintb = 0x40;
    let mut fd = build_fd();
    // `c <= V` : constant on the LEFT (slot 0).
    let op = mk_cmp_with_const(&mut fd, width, OpCode::CPUI_INT_LESSEQUAL, c, true);

    // canonicalize: `0x40 <= V` => `0x3f < V`  (INT_LESS, in0 = 0x3f, marked)
    assert!(fd.replace_lessequal(op).unwrap(), "forward canon should fire");
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 0), c - 1, "canon left constant = c-1");
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());

    // restore: `0x3f < V` => `0x40 <= V`  (INT_LESSEQUAL, in0 = 0x40, unmarked)
    assert_eq!(run_present(&mut fd), 1, "inverse restore should fire");
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
    assert_eq!(const_in(&fd, op, 0), c, "restored left constant = c (exact round trip)");
    assert!(!fd.obank().get(op).unwrap().is_canonical_lessequal(), "mark cleared on success");
}

// ===========================================================================
// R2-T3 — compareform: idempotency + provenance gating.
//
// (a) A second `ActionPresentCompareForm::apply` after a successful restore must
//     be a NO-OP: restoreLessequal clears the mark, and the now-LESSEQUAL op is
//     neither INT_LESS nor INT_SLESS, so the loop skips it.  A double-restore
//     would corrupt the constant (decrement it again).
// (b) An INT_LESS that was NEVER marked (no canonicalization provenance) is left
//     untouched — the mark is the sole trigger (`if (!op->isCanonicalLessequal())
//     continue;`), so an organically-`<` comparison is never rewritten.
// Oracle: kuna_compareform.cc:72-78 (the opcode + mark guards), :57 (clear).
// ===========================================================================
#[test]
fn r2_t3_idempotent_and_mark_gated() {
    // (a) round trip then re-run: second apply must not fire and must not move
    //     the constant.
    {
        let width = 4i32;
        let int_max = calc_int_max(width);
        let start = int_max - 1;
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, width, OpCode::CPUI_INT_SLESSEQUAL, start, false);
        assert!(fd.replace_lessequal(op).unwrap());
        assert_eq!(run_present(&mut fd), 1, "first restore fires");
        assert_eq!(const_in(&fd, op, 1), start);
        // Re-run on the already-restored function: nothing left marked / matching.
        let mut again = ActionPresentCompareForm::new(true, "analysis");
        let mut ctx = ActionContext::new();
        again.apply(&mut fd, &mut ctx);
        assert_eq!(again.base().count, 0, "second restore must be a no-op (idempotent)");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESSEQUAL);
        assert_eq!(const_in(&fd, op, 1), start, "constant must NOT be decremented twice");
    }
    // (b) an UNMARKED INT_LESS is never touched even with the gate ON.
    {
        let mut fd = build_fd();
        let op = mk_cmp_with_const(&mut fd, 4, OpCode::CPUI_INT_LESS, 0x10, false);
        // deliberately do NOT set_canonical_lessequal.
        assert!(!fd.obank().get(op).unwrap().is_canonical_lessequal());
        assert_eq!(run_present(&mut fd), 0, "unmarked op must be skipped");
        assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS, "unmarked op unchanged");
        assert_eq!(const_in(&fd, op, 1), 0x10);
    }
}
