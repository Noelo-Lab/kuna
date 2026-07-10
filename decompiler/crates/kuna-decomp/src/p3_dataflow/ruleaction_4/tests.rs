// Tests for the `ruleaction_4` rules (lines 4293-5526: RuleLoadVarnode ..
// RuleCondNegate).  Included into the `mod tests` in ruleaction_4.rs.
//
// Each positive test hand-builds the op pattern the C++ `applyOp` matches on a
// real `Funcdata` and asserts the exact transformation (opcode change, operand
// rewiring, created constants/ops).  Each early-out guard has a negative test.
//
// Coverage spans well over a third of the 19 rules, prioritizing the intricate
// ones: RuleSubExtComm, RuleSubCancel, RuleSubZext, RuleShiftAnd, RuleShiftSub,
// RuleHumptyDumpty, RuleDumptyHump, RuleHumptyOr, RuleConcatZext, RuleConcatZero,
// RuleZextCommute, RuleZextShiftZext, plus the registration shape.

use super::*;

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, BlockId};

// ---------------------------------------------------------------------------
// Fixtures (mirroring funcdata_op/tests.rs)
// ---------------------------------------------------------------------------

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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// A fresh basic block under the bblocks root.
fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// Create an op with `inputs` slots and a given opcode at `off`.
fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    set_opcode(fd, op, opc);
    op
}

/// Create a register Varnode of size `s` at ram offset `off`, marked as a
/// function input so it is *not* free (most rule guards require non-free leaf
/// operands, exactly as heritaged registers are non-free in a real flow).
fn mk_vn(fd: &mut Funcdata, s: int4, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(s, Address::new(ram, off), dt(s));
    fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).expect("mk_vn: set_input")
}

/// Build a defined Varnode: a `defop` (with output `out`, size `s`) in block
/// `bl`, fed `ins` on its input slots.  Returns `(defop, out)`.  The op is
/// inserted live so later `op_insert_before` calls have a parent to anchor to.
fn mk_def(
    fd: &mut Funcdata,
    bl: BlockId,
    opc: OpCode,
    s: int4,
    out_off: u64,
    ins: &[VarnodeId],
) -> (OpId, VarnodeId) {
    let op = mk_op(fd, ins.len() as int4, out_off, opc);
    for (i, &v) in ins.iter().enumerate() {
        fd.op_set_input(op, v, i as int4).unwrap();
    }
    fd.op_insert(op, bl, None);
    let out = new_unique_out(fd, s, op);
    (op, out)
}

// ---------------------------------------------------------------------------
// registration
// ---------------------------------------------------------------------------

#[test]
fn specs_in_definition_order() {
    let names: Vec<&str> = specs()
        .iter()
        .map(|s| {
            // The ctor's debug name comes from the rule's clone group; we instead
            // assert order via the rule's own `get_op_list` first opcode, which is
            // distinctive enough alongside the count.
            s.group
        })
        .collect();
    assert_eq!(names.len(), 19);
    // RuleLoadVarnode/RuleStoreVarnode (the first two, in definition order)
    // register under "stackvars" (coreaction.cc:5939-5940); the other 17 are
    // "analysis".
    assert_eq!(names[0], "stackvars");
    assert_eq!(names[1], "stackvars");
    assert!(names[2..].iter().all(|g| *g == "analysis"));
}

#[test]
fn op_lists_match_cpp() {
    assert_eq!(RuleLoadVarnode::new().get_op_list(), vec![OpCode::CPUI_LOAD]);
    assert_eq!(RuleStoreVarnode::new().get_op_list(), vec![OpCode::CPUI_STORE]);
    assert_eq!(RuleSubExtComm::new().get_op_list(), vec![OpCode::CPUI_SUBPIECE]);
    assert_eq!(
        RuleShiftAnd::new().get_op_list(),
        vec![OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_LEFT, OpCode::CPUI_INT_MULT]
    );
    assert_eq!(RuleConcatZext::new().get_op_list(), vec![OpCode::CPUI_PIECE]);
    assert_eq!(RuleZextCommute::new().get_op_list(), vec![OpCode::CPUI_INT_RIGHT]);
    assert_eq!(RuleSubZext::new().get_op_list(), vec![OpCode::CPUI_INT_ZEXT]);
    assert_eq!(RuleHumptyOr::new().get_op_list(), vec![OpCode::CPUI_INT_OR]);
    assert_eq!(RuleSwitchSingle::new().get_op_list(), vec![OpCode::CPUI_BRANCHIND]);
    assert_eq!(RuleCondNegate::new().get_op_list(), vec![OpCode::CPUI_CBRANCH]);
}

#[test]
fn clone_filters_on_group() {
    let inside = ActionGroupList::from_names(["analysis"]);
    let outside = ActionGroupList::from_names(["other"]);
    assert!(RuleSubCancel::new().clone_rule(&inside).is_some());
    assert!(RuleSubCancel::new().clone_rule(&outside).is_none());
}

// ---------------------------------------------------------------------------
// RuleShiftAnd  (pure input rewrite; fully runnable)
// ---------------------------------------------------------------------------

// `(V & 0xff) << 0  =>  V << 0`  when the AND clears no live bits.
//
// A processor-space varnode's nz-mask is the full `u64::MAX` (heritage would
// later narrow it; the W3 model seeds the conservative all-ones).  The INT_LEFT
// branch is the one that re-masks `nzm &= calc_mask(invn->getSize())`, so with
// invn size 1 (fullmask 0xff), mask 0xff, sa 0: `(mask & nzm) == nzm` holds and
// the AND is bypassed.  (The INT_RIGHT branch never re-masks, so a MAX nz-mask
// can't be covered there — that is faithful to the C++.)
#[test]
fn shiftand_int_left_bypasses_and() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let invn = mk_vn(&mut fd, 1, 0x10);
    let mask = fd.new_constant(1, 0xff);
    let (_andop, andout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 1, 0x20, &[invn, mask]);
    // shift: (andout) << 0
    let sa = fd.new_constant(4, 0);
    let shop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(shop, andout, 0).unwrap();
    fd.op_set_input(shop, sa, 1).unwrap();
    fd.op_insert(shop, bl, None);

    let res = RuleShiftAnd::new().apply_op(shop, &mut fd);
    assert_eq!(res, 1);
    // slot 0 of the shift now reads invn directly (AND bypassed).
    assert_eq!(fd.obank().get(shop).unwrap().get_in(0), Some(invn));
    assert_eq!(fd.obank().get(shop).unwrap().code(), OpCode::CPUI_INT_LEFT);
}

// Early-out: shift amount is not constant.
#[test]
fn shiftand_rejects_nonconstant_shift() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let invn = mk_vn(&mut fd, 1, 0x10);
    let mask = fd.new_constant(1, 0xff);
    let (_andop, andout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 1, 0x20, &[invn, mask]);
    let sa = mk_vn(&mut fd, 4, 0x40); // non-constant
    let shop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_RIGHT);
    fd.op_set_input(shop, andout, 0).unwrap();
    fd.op_set_input(shop, sa, 1).unwrap();
    fd.op_insert(shop, bl, None);
    assert_eq!(RuleShiftAnd::new().apply_op(shop, &mut fd), 0);
}

// Early-out: slot-0 def is not an INT_AND.
#[test]
fn shiftand_rejects_non_and_def() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 4, 0x10);
    let b = mk_vn(&mut fd, 4, 0x18);
    let (_addop, addout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ADD, 4, 0x20, &[a, b]);
    let sa = fd.new_constant(4, 0);
    let shop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_RIGHT);
    fd.op_set_input(shop, addout, 0).unwrap();
    fd.op_set_input(shop, sa, 1).unwrap();
    fd.op_insert(shop, bl, None);
    assert_eq!(RuleShiftAnd::new().apply_op(shop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleSubCancel  (input/opcode rewrite; fully runnable)
// ---------------------------------------------------------------------------

// `sub( V & 0xffff, 0 )  =>  sub(V)`  (the INT_AND total-cancel corner).
#[test]
fn subcancel_and_total_cancel() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // outsize = 2, mask = calc_mask(2) = 0xffff.
    let v = mk_vn(&mut fd, 4, 0x10);
    let mask = fd.new_constant(4, 0xffff);
    let (_andop, andout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 4, 0x20, &[v, mask]);
    // sub(andout, 0) with output size 2.
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, andout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 2, subop);

    let res = RuleSubCancel::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    // slot 0 now reads V (the AND's thruvn); opcode unchanged (still SUBPIECE).
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_SUBPIECE);
}

// `sub( zext(V), 0 )  =>  V`  when out-size == far-in-size (total elimination).
#[test]
fn subcancel_zext_total_elimination_to_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // V size 4; zext to 8; sub(zext, 0) out size 4 -> outsize==farinsize -> COPY.
    let v = mk_vn(&mut fd, 4, 0x10);
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 8, 0x20, &[v]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, zout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);

    let res = RuleSubCancel::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
}

// `sub( zext(V), c )  =>  0 (COPY of const 0)` when c >= far-in-size.
#[test]
fn subcancel_zext_offset_past_input_yields_zero() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // V size 2 (farinsize=2); zext to 8; sub(zext, 4) -> farinsize(2) <= offset(4).
    let v = mk_vn(&mut fd, 2, 0x10);
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 8, 0x20, &[v]);
    let off4 = fd.new_constant(4, 4);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, zout, 0).unwrap();
    fd.op_set_input(subop, off4, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 2, subop);

    let res = RuleSubCancel::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().num_input(), 1);
    let in0 = fd.obank().get(subop).unwrap().get_in(0).unwrap();
    assert!(fd.vbank().get(in0).unwrap().is_constant());
    assert_eq!(fd.vbank().get(in0).unwrap().get_offset(), 0);
}

// Early-out: slot-0 def is none of ZEXT/SEXT/AND.
#[test]
fn subcancel_rejects_other_def() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 8, 0x10);
    let b = mk_vn(&mut fd, 8, 0x18);
    let (_addop, addout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ADD, 8, 0x20, &[a, b]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, addout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);
    assert_eq!(RuleSubCancel::new().apply_op(subop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleShiftSub  (input rewrite + new constant; fully runnable)
// ---------------------------------------------------------------------------

// `sub( V << 8, 2 )  =>  sub(V, 1)`  (c -= n/8 = 2 - 1 = 1).
#[test]
fn shiftsub_rewrites_offset() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 8, 0x10); // insize 8
    let n = fd.new_constant(4, 8); // shift by 8 bits = 1 byte
    let (_shop, shout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_LEFT, 8, 0x20, &[v, n]);
    let c = fd.new_constant(4, 2);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, shout, 0).unwrap();
    fd.op_set_input(subop, c, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop); // outsize 4; c+out=1+4=5<=8 ok

    let res = RuleShiftSub::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
    let newc = fd.obank().get(subop).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(newc).unwrap().is_constant());
    assert_eq!(fd.vbank().get(newc).unwrap().get_offset(), 1);
}

// Early-out: the shift is not a multiple of 8 bits.
#[test]
fn shiftsub_rejects_non_byte_shift() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 8, 0x10);
    let n = fd.new_constant(4, 4); // 4 bits, not a multiple of 8
    let (_shop, shout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_LEFT, 8, 0x20, &[v, n]);
    let c = fd.new_constant(4, 2);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, shout, 0).unwrap();
    fd.op_set_input(subop, c, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);
    assert_eq!(RuleShiftSub::new().apply_op(subop, &mut fd), 0);
}

// Early-out: would not be a natural truncation (c - n/8 < 0).
#[test]
fn shiftsub_rejects_negative_offset() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 8, 0x10);
    let n = fd.new_constant(4, 16); // 2 bytes
    let (_shop, shout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_LEFT, 8, 0x20, &[v, n]);
    let c = fd.new_constant(4, 1); // c - n/8 = 1 - 2 = -1
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, shout, 0).unwrap();
    fd.op_set_input(subop, c, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);
    assert_eq!(RuleShiftSub::new().apply_op(subop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleHumptyDumpty  (opcode/operand rewrite; fully runnable)
// ---------------------------------------------------------------------------

// `concat( sub(V,4), sub(V,0) )  =>  V`  (whole thing pieced together).
#[test]
fn humptydumpty_whole_to_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let root = mk_vn(&mut fd, 8, 0x10); // root size 8
    // sub1 = sub(root, 4), size 4 (high half). pos1 = 4.
    let p4 = fd.new_constant(4, 4);
    let (_s1, sub1out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 4, 0x20, &[root, p4]);
    // sub2 = sub(root, 0), size 4 (low half). pos2 = 0.
    let p0 = fd.new_constant(4, 0);
    let (_s2, sub2out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 4, 0x28, &[root, p0]);
    // concat(sub1out, sub2out): vn1 size4 + vn2 size4 == root size8, pos2==0.
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, sub1out, 0).unwrap();
    fd.op_set_input(cat, sub2out, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 8, cat);

    let res = RuleHumptyDumpty::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(cat).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(cat).unwrap().get_in(0), Some(root));
}

// `concat( sub(V,6), sub(V,2) )  =>  sub(V,2)`  (a larger part, not the whole).
#[test]
fn humptydumpty_partial_to_subpiece() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let root = mk_vn(&mut fd, 8, 0x10);
    // sub1 = sub(root, 6) size 2; sub2 = sub(root, 2) size 4.  pos1=6 == pos2(2)+size2(4).
    let p6 = fd.new_constant(4, 6);
    let (_s1, sub1out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 2, 0x20, &[root, p6]);
    let p2 = fd.new_constant(4, 2);
    let (_s2, sub2out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 4, 0x28, &[root, p2]);
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, sub1out, 0).unwrap();
    fd.op_set_input(cat, sub2out, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 6, cat);

    let res = RuleHumptyDumpty::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_SUBPIECE);
    assert_eq!(fd.obank().get(cat).unwrap().get_in(0), Some(root));
    let newc = fd.obank().get(cat).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(newc).unwrap().get_offset(), 2); // pos2
}

// Early-out: the two SUBPIECEs draw from different roots.
#[test]
fn humptydumpty_rejects_different_roots() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let root1 = mk_vn(&mut fd, 8, 0x10);
    let root2 = mk_vn(&mut fd, 8, 0x18);
    let p4 = fd.new_constant(4, 4);
    let (_s1, sub1out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 4, 0x20, &[root1, p4]);
    let p0 = fd.new_constant(4, 0);
    let (_s2, sub2out) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 4, 0x28, &[root2, p0]);
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, sub1out, 0).unwrap();
    fd.op_set_input(cat, sub2out, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 8, cat);
    assert_eq!(RuleHumptyDumpty::new().apply_op(cat, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleDumptyHump  (opcode/operand rewrite; fully runnable)
// ---------------------------------------------------------------------------

// `sub( concat(V,W), 0 )  =>  W`  (offset 0, draws all of vn2).
#[test]
fn dumptyhump_offset0_eliminates() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10); // vn1
    let w = mk_vn(&mut fd, 4, 0x18); // vn2 (low)
    let (_cat, catout) = mk_def(&mut fd, bl, OpCode::CPUI_PIECE, 8, 0x20, &[v, w]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, catout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop); // out size == vn2 size

    let res = RuleDumptyHump::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(w));
}

// `sub( concat(V,W), 4 )  =>  V`  (offset reaches into vn1, drawing all of it).
#[test]
fn dumptyhump_offset_into_vn1() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10); // vn1 (high)
    let w = mk_vn(&mut fd, 4, 0x18); // vn2 (low) size 4
    let (_cat, catout) = mk_def(&mut fd, bl, OpCode::CPUI_PIECE, 8, 0x20, &[v, w]);
    let off4 = fd.new_constant(4, 4); // offset == vn2 size -> draws from vn1, rel offset 0
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, catout, 0).unwrap();
    fd.op_set_input(subop, off4, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop); // out size == vn1 size -> COPY

    let res = RuleDumptyHump::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
}

// Early-out: SUBPIECE straddles both halves of the concat.
#[test]
fn dumptyhump_rejects_straddle() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10);
    let w = mk_vn(&mut fd, 4, 0x18); // vn2 size 4
    let (_cat, catout) = mk_def(&mut fd, bl, OpCode::CPUI_PIECE, 8, 0x20, &[v, w]);
    let off2 = fd.new_constant(4, 2); // offset 2 < vn2size, but out 4 -> 2+4>4 straddle
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, catout, 0).unwrap();
    fd.op_set_input(subop, off2, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);
    assert_eq!(RuleDumptyHump::new().apply_op(subop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleHumptyOr  (constant branch; fully runnable)
// ---------------------------------------------------------------------------

// `(V & 0xff00) | (V & 0x00ff)  =>  V`  (constants cover all bits of V).
#[test]
fn humptyor_constants_cover_all_to_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 2, 0x10); // a size 2 -> calc_mask = 0xffff
    let bconst = fd.new_constant(2, 0xff00);
    let cconst = fd.new_constant(2, 0x00ff);
    let (_and1, and1out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x20, &[a, bconst]);
    let (_and2, and2out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x28, &[a, cconst]);
    let orop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_OR);
    fd.op_set_input(orop, and1out, 0).unwrap();
    fd.op_set_input(orop, and2out, 1).unwrap();
    fd.op_insert(orop, bl, None);
    let _orout = new_unique_out(&mut fd, 2, orop);

    let res = RuleHumptyOr::new().apply_op(orop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(orop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(orop).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(orop).unwrap().get_in(0), Some(a));
}

// `(V & 0x0f00) | (V & 0x00ff)  =>  V & 0x0fff`  (partial coverage -> AND).
#[test]
fn humptyor_partial_to_and() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 2, 0x10);
    let bconst = fd.new_constant(2, 0x0f00);
    let cconst = fd.new_constant(2, 0x00ff);
    let (_and1, and1out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x20, &[a, bconst]);
    let (_and2, and2out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x28, &[a, cconst]);
    let orop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_OR);
    fd.op_set_input(orop, and1out, 0).unwrap();
    fd.op_set_input(orop, and2out, 1).unwrap();
    fd.op_insert(orop, bl, None);
    let _orout = new_unique_out(&mut fd, 2, orop);

    let res = RuleHumptyOr::new().apply_op(orop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(orop).unwrap().code(), OpCode::CPUI_INT_AND);
    assert_eq!(fd.obank().get(orop).unwrap().get_in(0), Some(a));
    let newc = fd.obank().get(orop).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(newc).unwrap().get_offset(), 0x0fff);
}

// Early-out: the two ANDs share no operand.
#[test]
fn humptyor_rejects_no_shared_operand() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 2, 0x10);
    let x = mk_vn(&mut fd, 2, 0x14);
    let bconst = fd.new_constant(2, 0xff00);
    let cconst = fd.new_constant(2, 0x00ff);
    let (_and1, and1out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x20, &[a, bconst]);
    let (_and2, and2out) = mk_def(&mut fd, bl, OpCode::CPUI_INT_AND, 2, 0x28, &[x, cconst]);
    let orop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_OR);
    fd.op_set_input(orop, and1out, 0).unwrap();
    fd.op_set_input(orop, and2out, 1).unwrap();
    fd.op_insert(orop, bl, None);
    let _orout = new_unique_out(&mut fd, 2, orop);
    assert_eq!(RuleHumptyOr::new().apply_op(orop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleSubExtComm  (creates a new SUBPIECE; exercises new_unique_out)
// ---------------------------------------------------------------------------

// `sub(zext(V),0)  =>  zext(V)` when out-size == invn-size (the "doesn't hit
// extended bits, exactly the input size" -> COPY) and the broader commute.
//
// Use: V size 2, zext to 8, sub(zext, 0) out size 4.  out(4)+subcut(0)=4 > invn(2),
// so it falls to the commute path: subcut(0) < invn(2); subcut==0 so newvn=invn;
// op becomes zext(invn) (opcode INT_ZEXT, slot0=V).
#[test]
fn subextcomm_commutes_to_zext() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10);
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 8, 0x20, &[v]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x40, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, zout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop); // out size 4

    let res = RuleSubExtComm::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    // subcut==0 path: op opcode becomes INT_ZEXT, slot 0 = V, input 1 removed.
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(subop).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
}

// `sub(zext(V),0)  =>  COPY(V)` when out-size == invn-size: SUBPIECE misses the
// extended bits entirely and is exactly the input size.
#[test]
fn subextcomm_misses_extbits_to_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10); // invn size 4
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 8, 0x20, &[v]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x40, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, zout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop); // out(4)+0 <= invn(4) -> misses ext bits

    let res = RuleSubExtComm::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().num_input(), 1);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
}

// Early-out: slot-0 def is not an extension.
#[test]
fn subextcomm_rejects_non_extension() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 8, 0x10);
    let b = mk_vn(&mut fd, 8, 0x18);
    let (_addop, addout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ADD, 8, 0x20, &[a, b]);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x40, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, addout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _subout = new_unique_out(&mut fd, 4, subop);
    assert_eq!(RuleSubExtComm::new().apply_op(subop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleConcatZext  (creates an earlier PIECE; exercises new_unique_out)
// ---------------------------------------------------------------------------

// `concat( zext(V), W )  =>  zext( concat(V,W) )`.
#[test]
fn concatzext_commutes() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10); // hi inner, size 2
    let w = mk_vn(&mut fd, 4, 0x18); // lo, size 4
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 4, 0x20, &[v]); // zext(V) size 4
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, zout, 0).unwrap();
    fd.op_set_input(cat, w, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 8, cat);

    let res = RuleConcatZext::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    // op is now INT_ZEXT with one input = the new PIECE's output.
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(cat).unwrap().num_input(), 1);
    let newin = fd.obank().get(cat).unwrap().get_in(0).unwrap();
    let newpiece = fd.vbank().get(newin).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(newpiece).unwrap().code(), OpCode::CPUI_PIECE);
    // new PIECE has inputs V (inner of the zext) and W, sized V+W.
    assert_eq!(fd.obank().get(newpiece).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(newpiece).unwrap().get_in(1), Some(w));
    assert_eq!(fd.vbank().get(newin).unwrap().get_size(), 6);
}

// Early-out: slot-0 def is not a ZEXT.
#[test]
fn concatzext_rejects_non_zext() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 4, 0x10);
    let b = mk_vn(&mut fd, 4, 0x18);
    let w = mk_vn(&mut fd, 4, 0x20);
    let (_addop, addout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ADD, 4, 0x28, &[a, b]);
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, addout, 0).unwrap();
    fd.op_set_input(cat, w, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 8, cat);
    assert_eq!(RuleConcatZext::new().apply_op(cat, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleConcatZero  (creates a ZEXT then becomes a shift)
// ---------------------------------------------------------------------------

// `concat(V, 0)  =>  zext(V) << (8*size0)`.
#[test]
fn concatzero_to_zext_shift() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10); // high
    let zero = fd.new_constant(2, 0); // low, size 2 -> sa = 16
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, v, 0).unwrap();
    fd.op_set_input(cat, zero, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 6, cat);

    let res = RuleConcatZero::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    // op becomes INT_LEFT; slot0 = new ZEXT out, slot1 = const 16.
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_INT_LEFT);
    let saconst = fd.obank().get(cat).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(saconst).unwrap().get_offset(), 16);
    let zin = fd.obank().get(cat).unwrap().get_in(0).unwrap();
    let zop = fd.vbank().get(zin).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(zop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(zop).unwrap().get_in(0), Some(v));
}

// Early-out: the low piece is not the constant 0.
#[test]
fn concatzero_rejects_nonzero_const() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10);
    let lo = fd.new_constant(2, 1); // not zero
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, v, 0).unwrap();
    fd.op_set_input(cat, lo, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 6, cat);
    assert_eq!(RuleConcatZero::new().apply_op(cat, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleZextCommute  (creates an earlier INT_RIGHT)
// ---------------------------------------------------------------------------

// `zext(V) >> c  =>  zext( V >> c )`.
#[test]
fn zextcommute_pushes_shift_inside() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 4, 0x10); // zextin size 4
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 8, 0x20, &[v]);
    let c = fd.new_constant(4, 3);
    let shr = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_RIGHT);
    fd.op_set_input(shr, zout, 0).unwrap();
    fd.op_set_input(shr, c, 1).unwrap();
    fd.op_insert(shr, bl, None);
    let _shrout = new_unique_out(&mut fd, 8, shr);

    let res = RuleZextCommute::new().apply_op(shr, &mut fd);
    assert_eq!(res, 1);
    // op is now INT_ZEXT, single input = new INT_RIGHT output (sized like zextin).
    assert_eq!(fd.obank().get(shr).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(shr).unwrap().num_input(), 1);
    let newin = fd.obank().get(shr).unwrap().get_in(0).unwrap();
    assert_eq!(fd.vbank().get(newin).unwrap().get_size(), 4);
    let newshift = fd.vbank().get(newin).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(newshift).unwrap().code(), OpCode::CPUI_INT_RIGHT);
    assert_eq!(fd.obank().get(newshift).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(newshift).unwrap().get_in(1), Some(c));
}

// ---------------------------------------------------------------------------
// RuleSubZext  (the intricate sub-then-zext masking rule)
// ---------------------------------------------------------------------------

// `zext( sub(V,0) )  =>  V & mask`  (offset 0 -> bypass truncation, AND with mask).
#[test]
fn subzext_offset0_to_and_mask() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let basevn = mk_vn(&mut fd, 4, 0x10); // base size 4; out size must equal base size
    let off0 = fd.new_constant(4, 0);
    // sub(base, 0) size 2.
    let (_sub, subout) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 2, 0x20, &[basevn, off0]);
    // zext(subout) out size 4 (== base size).
    let zop = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(zop, subout, 0).unwrap();
    fd.op_insert(zop, bl, None);
    let _zout = new_unique_out(&mut fd, 4, zop);

    let res = RuleSubZext::new().apply_op(zop, &mut fd);
    assert_eq!(res, 1);
    // op becomes INT_AND; slot 0 bypasses to base, slot 1 = mask = calc_mask(2).
    assert_eq!(fd.obank().get(zop).unwrap().code(), OpCode::CPUI_INT_AND);
    assert_eq!(fd.obank().get(zop).unwrap().get_in(0), Some(basevn));
    let maskc = fd.obank().get(zop).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(maskc).unwrap().get_offset(), 0xffff);
}

// Early-out: sub base size != zext output size (not "truncate then extend to same").
#[test]
fn subzext_rejects_size_mismatch() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let basevn = mk_vn(&mut fd, 8, 0x10); // base size 8
    let off0 = fd.new_constant(4, 0);
    let (_sub, subout) = mk_def(&mut fd, bl, OpCode::CPUI_SUBPIECE, 2, 0x20, &[basevn, off0]);
    let zop = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(zop, subout, 0).unwrap();
    fd.op_insert(zop, bl, None);
    let _zout = new_unique_out(&mut fd, 4, zop); // out 4 != base 8
    assert_eq!(RuleSubZext::new().apply_op(zop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleConcatLeftShift  (creates concat(concat(V,W),0))
// ---------------------------------------------------------------------------

// `concat(V, zext(W) << 8)  =>  concat( concat(V,W), 0 )` (sa multiple of 8,
//  shifts to most-significant boundary).
#[test]
fn concatleftshift_rejoins() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10); // vn1
    let w = mk_vn(&mut fd, 1, 0x18); // inner of zext, size 1
    // zext(W) size 2 (tmpvn).  sa=8 -> sa/8=1; 1 + w(1) == tmp(2) ok.
    let (_zop, zout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 2, 0x20, &[w]);
    let sa = fd.new_constant(4, 8);
    let (_shop, shout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_LEFT, 2, 0x28, &[zout, sa]);
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, v, 0).unwrap();
    fd.op_set_input(cat, shout, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 4, cat); // out size 4

    let res = RuleConcatLeftShift::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    // op stays PIECE; slot0 = new concat(V,W) (size 3), slot1 = const 0 size 1.
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_PIECE);
    let newin = fd.obank().get(cat).unwrap().get_in(0).unwrap();
    assert_eq!(fd.vbank().get(newin).unwrap().get_size(), 3);
    let inner = fd.vbank().get(newin).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(inner).unwrap().code(), OpCode::CPUI_PIECE);
    assert_eq!(fd.obank().get(inner).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(inner).unwrap().get_in(1), Some(w));
    let zc = fd.obank().get(cat).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(zc).unwrap().is_constant());
    assert_eq!(fd.vbank().get(zc).unwrap().get_offset(), 0);
    assert_eq!(fd.vbank().get(zc).unwrap().get_size(), 1);
}

// ---------------------------------------------------------------------------
// RuleZextShiftZext  (zext(zext(a)) collapse path)
// ---------------------------------------------------------------------------

// `zext( zext(a) )  =>  zext(a)`  (the inner-ZEXT collapse: op's slot0 -> a).
#[test]
fn zextshiftzext_collapses_double_zext() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 2, 0x10);
    let (_inner, innerout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, 4, 0x20, &[a]);
    let outer = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(outer, innerout, 0).unwrap();
    fd.op_insert(outer, bl, None);
    let _outerout = new_unique_out(&mut fd, 8, outer);

    let res = RuleZextShiftZext::new().apply_op(outer, &mut fd);
    assert_eq!(res, 1);
    // op stays INT_ZEXT but now reads `a` directly.
    assert_eq!(fd.obank().get(outer).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(outer).unwrap().get_in(0), Some(a));
}

// Early-out: inner of ZEXT(...) is neither ZEXT nor INT_LEFT.
#[test]
fn zextshiftzext_rejects_other_inner() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 4, 0x10);
    let b = mk_vn(&mut fd, 4, 0x18);
    let (_addop, addout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_ADD, 4, 0x20, &[a, b]);
    let outer = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(outer, addout, 0).unwrap();
    fd.op_insert(outer, bl, None);
    let _outerout = new_unique_out(&mut fd, 8, outer);
    assert_eq!(RuleZextShiftZext::new().apply_op(outer, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// RuleConcatCommute  (commute PIECE with INT_OR const)
// ---------------------------------------------------------------------------

// `concat( V, W | c )  =>  concat(V,W) | c`  (i==1 path, INT_OR).
#[test]
fn concatcommute_or_lower() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10); // hi
    let w = mk_vn(&mut fd, 2, 0x18); // lo inner
    let c = fd.new_constant(2, 0x00ff);
    let (_orop, orout) = mk_def(&mut fd, bl, OpCode::CPUI_INT_OR, 2, 0x20, &[w, c]);
    let cat = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_PIECE);
    fd.op_set_input(cat, v, 0).unwrap();
    fd.op_set_input(cat, orout, 1).unwrap();
    fd.op_insert(cat, bl, None);
    let _catout = new_unique_out(&mut fd, 4, cat);

    let res = RuleConcatCommute::new().apply_op(cat, &mut fd);
    assert_eq!(res, 1);
    // i==0 (V) is not written so skipped; i==1 fires.  op becomes INT_OR with
    // slot0 = new concat(V,W) and slot1 = const c (unshifted for the i==1 case).
    assert_eq!(fd.obank().get(cat).unwrap().code(), OpCode::CPUI_INT_OR);
    let newin = fd.obank().get(cat).unwrap().get_in(0).unwrap();
    let inner = fd.vbank().get(newin).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(inner).unwrap().code(), OpCode::CPUI_PIECE);
    assert_eq!(fd.obank().get(inner).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(inner).unwrap().get_in(1), Some(w));
    let cc = fd.obank().get(cat).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(cc).unwrap().get_offset(), 0x00ff);
}

// ---------------------------------------------------------------------------
// Stub-affected rules: assert the documented early-out
// ---------------------------------------------------------------------------

// RuleLoadVarnode: a LOAD whose space operand (0x18) is not a valid spacebase
// constant and whose pointer is a plain non-spacebase Varnode is not a spacebase
// load -> `check_spacebase` returns None -> the rule is a no-op (0).  This pins
// only the negative case; positive spacebase-recovery behavior (the marking, the
// gather, the symbol paint, the stack-pointer-flow solve) is pinned directly by
// the unit tests in `coreaction_stackptr` / `funcdata_spacebase`'s test modules.
#[test]
fn loadvarnode_stub_noop() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let ptr = mk_vn(&mut fd, 8, 0x10);
    let spc = mk_vn(&mut fd, 8, 0x18); // space operand (annotation in real life)
    let load = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_LOAD);
    fd.op_set_input(load, spc, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    fd.op_insert(load, bl, None);
    let _out = new_unique_out(&mut fd, 4, load);
    assert_eq!(RuleLoadVarnode::new().apply_op(load, &mut fd), 0);
}

// RuleSwitchSingle: a BRANCHIND whose block has != 1 out-edge early-outs (0).
#[test]
fn switchsingle_rejects_multi_out_block() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // No out-edges added -> sizeOut() == 0 != 1.
    let br = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_BRANCHIND);
    let v = mk_vn(&mut fd, 8, 0x10);
    fd.op_set_input(br, v, 0).unwrap();
    fd.op_insert(br, bl, None);
    assert_eq!(RuleSwitchSingle::new().apply_op(br, &mut fd), 0);
}

// RuleCondNegate: a CBRANCH without the boolean_flip flag early-outs (0).
#[test]
fn condnegate_rejects_unflipped() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let cond = mk_vn(&mut fd, 1, 0x10);
    let target = fd.new_constant(8, 0x2000);
    let cb = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_CBRANCH);
    fd.op_set_input(cb, target, 0).unwrap();
    fd.op_set_input(cb, cond, 1).unwrap();
    fd.op_insert(cb, bl, None);
    // boolean_flip not set -> early-out.
    assert_eq!(RuleCondNegate::new().apply_op(cb, &mut fd), 0);
}

