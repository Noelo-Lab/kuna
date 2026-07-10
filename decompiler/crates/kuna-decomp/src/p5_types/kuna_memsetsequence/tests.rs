//! Tests for the constant-fill (memset/bzero) recovery sub-stage (kuna GH-9230):
//! [`MemsetSequence::form_fill_run`] — the single-value-fill detection that
//! distinguishes a memset from a string or a lone NUL terminator — plus the
//! [`RuleMemsetCopy`] gate and the `OptionMemsetRecover` parse.
//!
//! The collection (`collect_fill_run`), build (`build_memset`), transform, and
//! the full `RuleMemsetCopy::apply_op` body reach W4/W6 stubs (symbol table,
//! type-facing factory, address-only loc-set overload) and are exercised only at
//! their gate/structure level here — see the module docs.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::action::Rule;
use crate::constseq::WriteNode;
use crate::dtype::type_metatype;
use crate::context::{ArchContext, BlockId, TypeOp};

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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn char1(_fd: &Funcdata) -> Rc<Datatype> {
    // A 1-byte char-like element (align_size 1) — matches the C++ array element.
    Rc::new(Datatype::new(1, type_metatype::TYPE_INT))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op_in_block(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(2, Address::new(r, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

fn order_of(fd: &Funcdata, op: OpId) -> u32 {
    fd.obank().get(op).unwrap().get_seq_num().get_order()
}

/// A COPY of byte-size `sz` whose input-0 is the constant `cval`.
fn mk_copy_const(fd: &mut Funcdata, bl: BlockId, off: u64, sz: int4, cval: u64) -> OpId {
    let op = mk_op_in_block(fd, bl, off, OpCode::CPUI_COPY);
    let c = fd.new_constant(sz, cval);
    fd.op_set_input(op, c, 0).unwrap();
    op
}

/// Build a `WriteNode` for a collected COPY: `offset` is the output offset,
/// `slot` holds the COPY byte-size (the C++ `collectFillRun` convention).
fn node(fd: &Funcdata, off: u64, op: OpId, byte_size: int4) -> WriteNode {
    WriteNode::new(off, op, byte_size, order_of(fd, op))
}

// --- form_fill_run: positive detection ---------------------------------------

/// Sixteen 1-byte zero COPYs (an unrolled bzero) tile [0,16) with fill byte 0
/// -> a memset of 16 bytes.
#[test]
fn detects_zero_fill_16_single_byte() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let mut nodes = Vec::new();
    for i in 0..16u64 {
        let op = mk_copy_const(&mut fd, bl, i, 1, 0x00);
        nodes.push(node(&fd, i, op, 1));
    }
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(seq.is_valid_fill());
    assert_eq!(seq.fill_value(), 0x00);
    assert_eq!(seq.fill_count(), 16);
    assert_eq!(seq.move_ops().len(), 16);
}

/// Two 8-byte COPYs of the uniform byte 0xAA (clang's wide stores) tile [0,16)
/// -> a memset of 16 bytes with fill 0xAA.  (The low byte of each 8-byte const
/// is what `form_fill_run` compares.)
#[test]
fn detects_wide_uniform_fill_two_8byte() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // 0xAAAAAAAAAAAAAAAA: every byte is 0xAA.
    let wide = 0xAAAA_AAAA_AAAA_AAAAu64;
    let op0 = mk_copy_const(&mut fd, bl, 0, 8, wide);
    let op1 = mk_copy_const(&mut fd, bl, 8, 8, wide);
    let nodes = vec![node(&fd, 0, op0, 8), node(&fd, 8, op1, 8)];
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(seq.is_valid_fill());
    assert_eq!(seq.fill_value(), 0xAA);
    assert_eq!(seq.fill_count(), 16);
    assert_eq!(seq.move_ops().len(), 2);
}

/// Collected COPYs given out of offset order are sorted first; the run is then
/// recognized.
#[test]
fn sorts_collected_ops_before_tiling() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // 4 x 4-byte uniform 0x01010101 at offsets 0,4,8,12 = 16 bytes, fed reversed.
    let val = 0x0101_0101u64;
    let mut ops = Vec::new();
    for i in 0..4u64 {
        ops.push(mk_copy_const(&mut fd, bl, i * 4, 4, val));
    }
    let nodes = vec![
        node(&fd, 12, ops[3], 4),
        node(&fd, 4, ops[1], 4),
        node(&fd, 0, ops[0], 4),
        node(&fd, 8, ops[2], 4),
    ];
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(seq.is_valid_fill());
    assert_eq!(seq.fill_value(), 0x01);
    assert_eq!(seq.fill_count(), 16);
}

/// Trailing ops past the contiguous run are dropped (a gap ends the tiling).
#[test]
fn drops_ops_past_contiguous_run() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // 16 bytes of 0x00 at [0,16), then a 17th COPY at offset 0x20 (gap) of 0x00.
    let mut nodes = Vec::new();
    for i in 0..16u64 {
        let op = mk_copy_const(&mut fd, bl, i, 1, 0x00);
        nodes.push(node(&fd, i, op, 1));
    }
    let strayop = mk_copy_const(&mut fd, bl, 0x20, 1, 0x00);
    nodes.push(node(&fd, 0x20, strayop, 1));
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(seq.is_valid_fill());
    assert_eq!(seq.fill_count(), 16);
    // The stray op past the [0,16) run is dropped.
    assert_eq!(seq.move_ops().len(), 16);
}

// --- form_fill_run: negative / decline paths ---------------------------------

/// A different fill byte mid-run ends the contiguous run; if what precedes it is
/// under 16 bytes, no memset.
#[test]
fn different_fill_byte_ends_run() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // 8 bytes of 0x00 then a 0x01 -> the run is only 8 bytes (< 16) -> declined.
    let mut nodes = Vec::new();
    for i in 0..8u64 {
        let op = mk_copy_const(&mut fd, bl, i, 1, 0x00);
        nodes.push(node(&fd, i, op, 1));
    }
    let op8 = mk_copy_const(&mut fd, bl, 8, 1, 0x01);
    nodes.push(node(&fd, 8, op8, 1));
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(!seq.is_valid_fill());
    assert_eq!(seq.fill_count(), 0);
}

/// A footprint under 16 bytes is rejected even if uniform and contiguous
/// (`totalBytes < 16`).
#[test]
fn too_short_footprint_declined() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // 4 x 1-byte 0x00 = 4 bytes < 16.
    let mut nodes = Vec::new();
    for i in 0..4u64 {
        let op = mk_copy_const(&mut fd, bl, i, 1, 0x00);
        nodes.push(node(&fd, i, op, 1));
    }
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(!seq.is_valid_fill());
}

/// A LONE store (one COPY) is never claimed as a memset, even a 16-byte one
/// (`lastIdx < 1` requires at least 2 COPYs) — so a string's NUL terminator is
/// never stolen.
#[test]
fn lone_store_never_claimed() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy_const(&mut fd, bl, 0, 16, 0x0000_0000_0000_0000);
    let nodes = vec![node(&fd, 0, op, 16)];
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(!seq.is_valid_fill());
}

/// A gap at the very start (the first two offsets are not contiguous) stops the
/// run immediately -> `lastIdx < 1` -> declined.
#[test]
fn leading_gap_declines() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // offset 0 (1 byte) then offset 4 (gap of 3) -> run ends after the first op.
    let op0 = mk_copy_const(&mut fd, bl, 0, 1, 0x00);
    let op4 = mk_copy_const(&mut fd, bl, 4, 1, 0x00);
    let nodes = vec![node(&fd, 0, op0, 1), node(&fd, 4, op4, 1)];
    let ct = char1(&fd);
    let seq = MemsetSequence::from_collected(ct, nodes, &fd);
    assert!(!seq.is_valid_fill());
}

// --- RuleMemsetCopy gate -----------------------------------------------------

/// Gate off: `apply_op` declines (returns 0) regardless of the op.
#[test]
fn rule_gate_off_is_noop() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy_const(&mut fd, bl, 0, 1, 0x00);
    let mut rule = RuleMemsetCopy::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

/// Gate on, but the driver body is stub-blocked (W4 symbol table / W6 type
/// facing) -> still declines today, byte-identical to the option being off.
#[test]
fn rule_gate_on_declines_until_stubs_land() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy_const(&mut fd, bl, 0, 1, 0x00);
    let mut rule = RuleMemsetCopy::new(true);
    // getTypeDefFacing/queryContainer are not reachable yet (see module docs).
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

/// The rule matches on CPUI_COPY and clone-filters by group.
#[test]
fn rule_oplist_and_clone() {
    let rule = RuleMemsetCopy::new(true);
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_COPY]);
    let in_list = ActionGroupList::from_names(["analysis"]);
    let out_list = ActionGroupList::from_names(["other"]);
    assert!(rule.clone_rule(&in_list).is_some());
    assert!(rule.clone_rule(&out_list).is_none());
}

// --- OptionMemsetRecover parse -----------------------------------------------

#[test]
fn parse_on_sets_flag() {
    let (form, msg) = parse_memset_recover_form("on").unwrap();
    assert_eq!(form, MemsetRecoverForm::On);
    assert!(form.memset_recover());
    assert_eq!(msg, "Constant-fill (memset) recovery turned on");
}

#[test]
fn parse_off_clears_flag() {
    let (form, msg) = parse_memset_recover_form("off").unwrap();
    assert_eq!(form, MemsetRecoverForm::Off);
    assert!(!form.memset_recover());
    assert_eq!(msg, "Constant-fill (memset) recovery turned off");
}

#[test]
fn parse_empty_defaults_on() {
    // onOrOff("") == true (the shared parser default).
    let (form, _) = parse_memset_recover_form("").unwrap();
    assert_eq!(form, MemsetRecoverForm::On);
}

#[test]
fn parse_garbage_is_error() {
    assert!(parse_memset_recover_form("maybe").is_err());
}

// --- specs -------------------------------------------------------------------

#[test]
fn specs_ship_default_on() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    // The shipped ctor builds an enabled rule (memset_recover = true, DIV-2).
    let rule = (s[0].ctor)();
    // A real apply still declines (stub-blocked), but the rule is constructed
    // in the enabled state — exercise its op list to confirm it is RuleMemsetCopy.
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_COPY]);
    // gate is on: clone in the analysis group survives.
    assert!(rule.clone_rule(&ActionGroupList::from_names(["analysis"])).is_some());
}
