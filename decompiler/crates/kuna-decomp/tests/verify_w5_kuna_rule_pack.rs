//! Independent verifier adversarial tests for item `w5-kuna-rule-pack`
//! (the 7 kuna simplification rules:
//! `kuna_{addcarrychain,booleanmask,ovlesssimplify,flagcompare,stackalias,
//! arraystride,condexeplace}`).
//!
//! Re-derived from the C++ oracle (NOT the Rust port).  These target the
//! hunt-list spots flagged most fragile that the in-crate porter tests do NOT
//! exercise:
//!
//!   - arraystride: SWAPPED SLOT INDEX on the sibling counter phi
//!     (`kuna_arraystride.cc:118`  `if (ci != initIdx || cb != backIdx) continue;`)
//!     and the COMMUTED back-edge add operand order (`a==out` vs `b==out`,
//!     `kuna_arraystride.cc:70-71`).
//!   - addcarrychain: COMMUTED `INT_CARRY(b,a)` operands
//!     (`kuna_addcarrychain.cc:56`) and the const-folded `INT_LESSEQUAL` carry
//!     form (`kuna_addcarrychain.cc:59-72`).
//!   - ovlesssimplify: the `mone` operand of the SLESS-hi limb must be exactly
//!     -1 (`kuna_ovlesssimplify.cc:79  if (mone->getOffset()!=calc_mask(...))`).
//!   - booleanmask: the byte-aligned early-out `if ((sa & 7) == 0) return 0;`
//!     (`kuna_booleanmask.cc:36`) defers to RuleLeftRight even on an otherwise
//!     perfect match.
//!
//! Each assertion cites the C++ line it is derived from.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::Rule;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_addcarrychain::RuleAddCarryChain;
use kuna_decomp::kuna_arraystride::RuleArrayStride;
use kuna_decomp::kuna_booleanmask::RuleBoolSignShift;
use kuna_decomp::kuna_ovlesssimplify::RuleOvLessSimplify;
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

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
    let glb = Rc::new(Architecture::new(manage));
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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(())
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// Create an op on the dead list (NOT marked alive) — for ops the rule reads by
/// id but that are never inserted into the block (matching the arraystride /
/// addcarrychain in-crate harness).
fn mk_op_dead(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

/// Create a live op (for the standalone-rewrite rules that don't go through
/// `op_insert`, matching the flagcompare/ovlesssimplify in-crate harness).
fn mk_op_live(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let op = mk_op_dead(fd, off, inputs, opc);
    fd.obank_mut().mark_alive(op);
    op
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, defop: OpId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn mk_byte(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op_dead(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(1, &Address::new(r, off), Some(unk(1)));
    set_def(fd, vn, defop)
}

// ===========================================================================
// arraystride GH-8724 — swapped sibling edges (swapped-slot-index hunt) and
// commuted back-edge add operand order.
// ===========================================================================

/// Build an induction phi `out = MULTIEQUAL(slot[init]=#0, slot[back]=out+step)`
/// in block `bl`, with `step` placed in add slot `add_self_first` (true: ADD(out,
/// step); false: ADD(step, out) — the commuted order).  Returns (phi, out).
#[allow(clippy::too_many_arguments)]
fn mk_induction_phi(
    fd: &mut Funcdata,
    bl: BlockId,
    base_off: u64,
    sz: int4,
    step: u64,
    init_slot: int4,
    back_slot: int4,
    self_first: bool,
) -> (OpId, VarnodeId) {
    let phi = mk_op_dead(fd, base_off, 2, OpCode::CPUI_MULTIEQUAL);
    let out = fd.new_unique(sz, Some(unk(sz)));
    let out = set_def(fd, out, phi);
    // back-edge add
    let add = mk_op_dead(fd, base_off + 8, 2, OpCode::CPUI_INT_ADD);
    let step_c = fd.new_constant(sz, step);
    if self_first {
        wire(fd, out, add, 0);
        wire(fd, step_c, add, 1);
    } else {
        wire(fd, step_c, add, 0);
        wire(fd, out, add, 1);
    }
    let add_out = fd.new_unique(sz, Some(unk(sz)));
    let add_out = set_def(fd, add_out, add);
    let zero = fd.new_constant(sz, 0);
    wire(fd, zero, phi, init_slot);
    wire(fd, add_out, phi, back_slot);
    fd.op_insert(phi, bl, None);
    (phi, out)
}

#[test]
fn w5_arraystride_swapped_sibling_edges_decline() {
    // accumulator phi:  init at slot 0, back at slot 1  (stride 0x10).
    // counter phi:      init at slot 1, back at slot 0  (step 1)  -> MISALIGNED.
    // C++ kuna_arraystride.cc:118  `if (ci != initIdx || cb != backIdx) continue;`
    // -> no matching sibling -> rule declines (returns 0).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let (acc_phi, acc_out) = mk_induction_phi(&mut fd, bl, 0x100, 4, 0x10, 0, 1, true);
    // External reader of acc so a successful fold would be observable.
    let reader = mk_op_dead(&mut fd, 0x130, 1, OpCode::CPUI_COPY);
    wire(&mut fd, acc_out, reader, 0);
    let rout = fd.new_unique(4, Some(unk(4)));
    let _ = set_def(&mut fd, rout, reader);
    // Sibling counter with SWAPPED edges (init slot 1, back slot 0).
    let (_cnt_phi, _cnt_out) = mk_induction_phi(&mut fd, bl, 0x110, 4, 1, 1, 0, true);

    let mut rule = RuleArrayStride::new(true);
    assert_eq!(
        rule.apply_op(acc_phi, &mut fd),
        0,
        "misaligned sibling edges must NOT fold (kuna_arraystride.cc:118)"
    );
    // The phi survives unchanged and the reader still reads the accumulator.
    assert_eq!(fd.obank().get(acc_phi).unwrap().code(), OpCode::CPUI_MULTIEQUAL);
    assert_eq!(fd.obank().get(reader).unwrap().get_in(0), Some(acc_out));
}

#[test]
fn w5_arraystride_commuted_back_edge_add_folds() {
    // Both phis use the COMMUTED add order `INT_ADD(step, self)` (self in slot 1).
    // C++ kuna_arraystride.cc:71  `else if (b == out) { self = b; cst = a; }`
    // must still recognize the induction shape and fold acc -> cnt*stride.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let (acc_phi, acc_out) =
        mk_induction_phi(&mut fd, bl, 0x100, 4, 0x10, 0, 1, /*self_first=*/ false);
    let reader = mk_op_dead(&mut fd, 0x130, 1, OpCode::CPUI_COPY);
    wire(&mut fd, acc_out, reader, 0);
    let rout = fd.new_unique(4, Some(unk(4)));
    let _ = set_def(&mut fd, rout, reader);
    let (_cnt_phi, cnt_out) =
        mk_induction_phi(&mut fd, bl, 0x110, 4, 1, 0, 1, /*self_first=*/ false);

    let mut rule = RuleArrayStride::new(true);
    assert_eq!(
        rule.apply_op(acc_phi, &mut fd),
        1,
        "commuted add order INT_ADD(step,self) must still fold (kuna_arraystride.cc:71)"
    );
    // reader now reads INT_MULT(cnt, 0x10).
    let multout = fd.obank().get(reader).unwrap().get_in(0).unwrap();
    let multop = fd.vbank().get(multout).unwrap().get_def().unwrap();
    let mo = fd.obank().get(multop).unwrap();
    assert_eq!(mo.code(), OpCode::CPUI_INT_MULT);
    assert_eq!(mo.get_in(0), Some(cnt_out));
    let strv = mo.get_in(1).unwrap();
    assert_eq!(fd.vbank().get(strv).unwrap().get_offset(), 0x10);
}

// ===========================================================================
// addcarrychain GH-8913 — commuted INT_CARRY operands and the const-folded
// INT_LESSEQUAL carry form.
// ===========================================================================

/// Common low-add + PIECE scaffold; the caller supplies the carry varnode.
/// Returns the PIECE op.
fn build_piece_with_carry(
    fd: &mut Funcdata,
    bl: BlockId,
    a: VarnodeId,
    b: VarnodeId,
    hipart: VarnodeId,
    carryvn: VarnodeId,
) -> OpId {
    // lo = INT_ADD(a, b)
    let loadd = mk_op_dead(fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(fd, a, loadd, 0);
    wire(fd, b, loadd, 1);
    let lovn = fd.new_unique(1, Some(unk(1)));
    let lovn = set_def(fd, lovn, loadd);
    // hi = INT_ADD(hipart, carry)
    let hiadd = mk_op_dead(fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(fd, hipart, hiadd, 0);
    wire(fd, carryvn, hiadd, 1);
    let hivn = fd.new_unique(1, Some(unk(1)));
    let hivn = set_def(fd, hivn, hiadd);
    // op = PIECE(hi, lo)
    let op = mk_op_dead(fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(fd, hivn, op, 0);
    wire(fd, lovn, op, 1);
    let out = fd.new_unique(2, Some(unk(2)));
    let _ = set_def(fd, out, op);
    fd.op_insert(op, bl, None);
    op
}

#[test]
fn w5_addcarrychain_commuted_int_carry_operands_fold() {
    // carry = INT_CARRY(b, a)  -- operands SWAPPED relative to lo = a + b.
    // C++ kuna_addcarrychain.cc:56  `if (kunaSameValue(c0,b) && kunaSameValue(c1,a)) return true;`
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_byte(&mut fd, 0x40);
    let b = mk_byte(&mut fd, 0x44);
    let hipart = mk_byte(&mut fd, 0x48);
    let carryop = mk_op_dead(&mut fd, 0x108, 2, OpCode::CPUI_INT_CARRY);
    wire(&mut fd, b, carryop, 0); // swapped: b first
    wire(&mut fd, a, carryop, 1);
    let carryvn = fd.new_unique(1, Some(unk(1)));
    let carryvn = set_def(&mut fd, carryvn, carryop);

    let op = build_piece_with_carry(&mut fd, bl, a, b, hipart, carryvn);
    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(
        rule.apply_op(op, &mut fd),
        1,
        "commuted INT_CARRY(b,a) must still fuse (kuna_addcarrychain.cc:56)"
    );
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_ADD);
    // index operand is `a` (zext'd): in1 def is INT_ZEXT(a).
    let zext_out = fd.obank().get(op).unwrap().get_in(1).unwrap();
    let zextop = fd.vbank().get(zext_out).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(zextop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(zextop).unwrap().get_in(0), Some(a));
}

#[test]
fn w5_addcarrychain_carryelim_const_baselo_dedups_and_completes() {
    // The const-folded carry form (`kunaIsCarryOf` INT_LESSEQUAL branch) is the
    // case where ONE low-add operand is a constant.  The rewrite re-wires
    // `indexvn = a` into the new ZEXT and `baselo = b` into the new base while a/b
    // are STILL read by the original low add — so the constant operand `a` would
    // gain a SECOND descendant.  C++ `opSetInput` re-duplicates a constant that
    // already has a descendant (funcdata_op.cc:108) BEFORE `addDescend`, so the
    // rewrite COMPLETES (it never reaches the "multiple descendants" throw — that
    // path is for free *non-constant* varnodes only).
    //
    // Now that the constant re-duplication guard in `op_set_input` is ported (the
    // funcdata_varnode `newConstant` factory has landed), the port matches C++:
    // the formerly-pinned F1 panic divergence (LOSS-035/055 family for the
    // const-carry path) is RESOLVED — the rule fuses to INT_ADD with the constant
    // re-duplicated into the new ZEXT.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let cval: u64 = 0x30;
    let a_const = fd.new_constant(1, cval);
    let b = mk_byte(&mut fd, 0x44);
    let hipart = mk_byte(&mut fd, 0x48);
    let negc = cval.wrapping_neg() & calc_mask(1);
    let negc_c = fd.new_constant(1, negc);
    let leop = mk_op_dead(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, negc_c, leop, 0);
    wire(&mut fd, b, leop, 1);
    let carryvn = fd.new_unique(1, Some(unk(1)));
    let carryvn = set_def(&mut fd, carryvn, leop);
    let loadd = mk_op_dead(&mut fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, a_const, loadd, 0); // constant low-add operand (a / cAdd)
    wire(&mut fd, b, loadd, 1);
    let lovn = fd.new_unique(1, Some(unk(1)));
    let lovn = set_def(&mut fd, lovn, loadd);
    let hiadd = mk_op_dead(&mut fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, hipart, hiadd, 0);
    wire(&mut fd, carryvn, hiadd, 1);
    let hivn = fd.new_unique(1, Some(unk(1)));
    let hivn = set_def(&mut fd, hivn, hiadd);
    let op = mk_op_dead(&mut fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(&mut fd, hivn, op, 0);
    wire(&mut fd, lovn, op, 1);
    let out = fd.new_unique(2, Some(unk(2)));
    let _ = set_def(&mut fd, out, op);
    fd.op_insert(op, bl, None);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 1, "const-folded carry now fuses (dedup guard)");
    // op fused to INT_ADD; its index operand is the ZEXT of the constant `a`.
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_ADD);
    let zext_out = fd.obank().get(op).unwrap().get_in(1).unwrap();
    let zextop = fd.vbank().get(zext_out).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(zextop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    let zext_in = fd.obank().get(zextop).unwrap().get_in(0).unwrap();
    let zin = fd.vbank().get(zext_in).unwrap();
    assert!(zin.is_constant() && zin.get_offset() == cval, "ZEXT reads the (re-dup'd) constant a");
    // The original constant still has exactly its one (loadd) descendant — the
    // ZEXT read a fresh duplicate.
    assert_eq!(fd.vbank().get(a_const).unwrap().num_descend(), 1, "original constant not over-shared");
}

#[test]
fn w5_addcarrychain_carryelim_wrong_negc_declines() {
    // Same INT_LESSEQUAL shape but the constant is NOT (-c)&mask -> decline.
    // Guards the `cst->getOffset() == negc` test (kuna_addcarrychain.cc:72).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_byte(&mut fd, 0x40);
    let hipart = mk_byte(&mut fd, 0x48);
    let cval: u64 = 0x30;
    let b_const = fd.new_constant(1, cval);
    let wrong = (cval.wrapping_neg() & calc_mask(1)) ^ 0x1; // perturb one bit
    let wrong_c = fd.new_constant(1, wrong);
    let leop = mk_op_dead(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, wrong_c, leop, 0);
    wire(&mut fd, a, leop, 1);
    let carryvn = fd.new_unique(1, Some(unk(1)));
    let carryvn = set_def(&mut fd, carryvn, leop);

    let op = build_piece_with_carry(&mut fd, bl, a, b_const, hipart, carryvn);
    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(
        rule.apply_op(op, &mut fd),
        0,
        "wrong (-c) constant must NOT fuse (kuna_addcarrychain.cc:72)"
    );
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_PIECE);
}

// ===========================================================================
// ovlesssimplify GH-7190 — the SLESS-hi limb's first operand must be exactly -1.
// ===========================================================================

#[test]
fn w5_ovlesssimplify_mone_not_minus_one_declines() {
    // Build NE( SLESS(V+K,0) , BOOL_AND(signTest, SLESS(C, V+K)) ) where C is a
    // constant that is NOT -1.  C++ kuna_ovlesssimplify.cc:79
    //   `if (mone->getOffset() != calc_mask(mone->getSize())) continue;`
    // -> the AND limb is rejected as the SLESS-hi limb -> slessHi stays null ->
    // rule declines.
    let mut fd = build_fd();
    let sz: int4 = 4;
    let r = ram(&fd);
    // V (a written value)
    let vdef = mk_op_dead(&mut fd, 0x40, 1, OpCode::CPUI_COPY);
    let v = fd.new_varnode(sz, &Address::new(Rc::clone(&r), 0x40), Some(unk(sz)));
    let v = set_def(&mut fd, v, vdef);

    // helper: build R = INT_ADD(V, #5).  Each add owns a fresh K constant (a
    // free varnode may have only one descendant); the rule compares K by offset
    // so two #5 constants are equal for its purposes.
    let mk_add = |fd: &mut Funcdata, off: u64| -> VarnodeId {
        let add = mk_op_dead(fd, off, 2, OpCode::CPUI_INT_ADD);
        let kc = fd.new_constant(sz, 5);
        wire(fd, v, add, 0);
        wire(fd, kc, add, 1);
        let o = fd.new_unique(sz, Some(unk(sz)));
        set_def(fd, o, add)
    };
    let r1 = mk_add(&mut fd, 0x100);
    let r2 = mk_add(&mut fd, 0x108);

    // sless0 = INT_SLESS(R1, 0)
    let sless0 = mk_op_dead(&mut fd, 0x110, 2, OpCode::CPUI_INT_SLESS);
    let zero = fd.new_constant(sz, 0);
    wire(&mut fd, r1, sless0, 0);
    wire(&mut fd, zero, sless0, 1);
    let in0 = fd.new_unique(1, Some(unk(1)));
    let in0 = set_def(&mut fd, in0, sless0);

    // slessHi = INT_SLESS(C, R2)  with C = 7 (NOT -1)
    let bad_c = fd.new_constant(sz, 7);
    let slesshi = mk_op_dead(&mut fd, 0x118, 2, OpCode::CPUI_INT_SLESS);
    wire(&mut fd, bad_c, slesshi, 0);
    wire(&mut fd, r2, slesshi, 1);
    let hi = fd.new_unique(1, Some(unk(1)));
    let hi = set_def(&mut fd, hi, slesshi);

    // signTest: any written boolean (unused by the rewrite)
    let sign = mk_op_dead(&mut fd, 0x120, 1, OpCode::CPUI_COPY);
    wire(&mut fd, v, sign, 0);
    let signo = fd.new_unique(1, Some(unk(1)));
    let signo = set_def(&mut fd, signo, sign);

    // andop = BOOL_AND(signTest, slessHi)
    let andop = mk_op_dead(&mut fd, 0x128, 2, OpCode::CPUI_BOOL_AND);
    wire(&mut fd, signo, andop, 0);
    wire(&mut fd, hi, andop, 1);
    let in1 = fd.new_unique(1, Some(unk(1)));
    let in1 = set_def(&mut fd, in1, andop);

    // op = INT_NOTEQUAL(in0, in1)
    let op = mk_op_live(&mut fd, 0x130, 2, OpCode::CPUI_INT_NOTEQUAL);
    wire(&mut fd, in0, op, 0);
    wire(&mut fd, in1, op, 1);
    let oout = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, oout, op);

    let mut rule = RuleOvLessSimplify::new(true);
    assert_eq!(
        rule.apply_op(op, &mut fd),
        0,
        "SLESS-hi limb whose first operand is not -1 must NOT collapse (kuna_ovlesssimplify.cc:79)"
    );
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_NOTEQUAL);
}

// ===========================================================================
// booleanmask GH-1282 — the byte-aligned early-out defers to RuleLeftRight.
// ===========================================================================

#[test]
fn w5_booleanmask_byte_aligned_shift_declines() {
    // sa = 8 (a multiple of 8): everything else matches a perfect mask shape, but
    // C++ kuna_booleanmask.cc:36  `if ((sa & 7) == 0) return 0;` defers to the
    // generic RuleLeftRight -> rule declines even though the NZMask fits.
    let mut fd = build_fd();
    // boolvn: a 2-byte value whose NZMask is 1 (fits the low (16-8)=8 bits).
    let boolvn = fd.new_constant(2, 1);
    let leftshift = mk_op_dead(&mut fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let sa = fd.new_constant(2, 8);
    fd.obank_mut().get_mut(leftshift).unwrap().set_input(Some(boolvn), 0); // shared const
    wire(&mut fd, sa, leftshift, 1);
    let shiftout = fd.new_unique(2, Some(unk(2)));
    let shiftout = set_def(&mut fd, shiftout, leftshift);

    // op = INT_SRIGHT(shiftout, 8)
    let op = mk_op_live(&mut fd, 0x108, 2, OpCode::CPUI_INT_SRIGHT);
    let sa2 = fd.new_constant(2, 8);
    wire(&mut fd, shiftout, op, 0);
    wire(&mut fd, sa2, op, 1);
    let out = fd.new_unique(2, Some(unk(2)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleBoolSignShift::new(true);
    assert_eq!(
        rule.apply_op(op, &mut fd),
        0,
        "byte-aligned shift must defer to RuleLeftRight (kuna_booleanmask.cc:36)"
    );
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SRIGHT);
}

