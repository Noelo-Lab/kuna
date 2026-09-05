//! Tests for read-only string block-copy recovery (kuna `rodatastring`): the
//! [`covers_exactly`] tiling predicate that decides whether a gathered run
//! really spells the whole string, the [`readonly_source`] source classifier,
//! plus the [`RuleRodataStringCopy`] gate and the option parse.
//!
//! The end-to-end recovery (the `builtin_strncpy` build and the COPY teardown,
//! both inherited from `StringSequence`) needs a load image, a global symbol
//! table and a type factory, so it is proven by `tests/stages/kuna-rodatastring.xml`
//! rather than here.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::action::Rule;
use crate::context::{ArchContext, TypeOp};
use crate::varnode::varnode_flags;

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

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_copy(fd: &mut Funcdata, bl: BlockId, at: u64) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(2, Address::new(r, at));
    fd.obank_mut().change_opcode(
        op,
        TypeOp::new(OpCode::CPUI_COPY, 0, format!("{:?}", OpCode::CPUI_COPY)),
    );
    fd.op_insert_end(op, bl);
    op
}

fn slice(off: int4, size: int4) -> Slice {
    Slice { off, size, op: OpId::default(), order: 0 }
}

// --- covers_exactly ---------------------------------------------------------

/// The witnessed shape: 8 + 1 + 7 + 9 tiles a 25-byte string exactly.
#[test]
fn split_run_tiles_the_whole_string() {
    let s = vec![slice(0, 8), slice(9, 7), slice(8, 1), slice(16, 9)];
    assert!(covers_exactly(&s, 25));
}

/// A gap in the middle leaves bytes the run never wrote: decline.
#[test]
fn gap_declines() {
    assert!(!covers_exactly(&[slice(0, 8), slice(9, 16)], 25));
}

/// Overlapping writes are ambiguous about which store wins: decline.
#[test]
fn overlap_declines() {
    assert!(!covers_exactly(&[slice(0, 16), slice(8, 17)], 25));
}

/// A run that stops short of the terminator is not the whole string: decline.
#[test]
fn partial_coverage_declines() {
    assert!(!covers_exactly(&[slice(0, 8), slice(8, 8)], 25));
}

/// A run that would write past the symbol: decline.
#[test]
fn overlong_coverage_declines() {
    assert!(!covers_exactly(&[slice(0, 16), slice(16, 16)], 25));
}

/// A single whole-string COPY already renders as one assignment; the filed
/// defect is the SPLIT copy, so a lone op is left alone.
#[test]
fn single_copy_declines() {
    assert!(!covers_exactly(&[slice(0, 25)], 25));
}

/// A run that does not start at byte 0 never spells the string from its head.
#[test]
fn non_zero_start_declines() {
    assert!(!covers_exactly(&[slice(1, 8), slice(9, 16)], 25));
}

// --- readonly_source --------------------------------------------------------

/// A constant input is `RuleStringCopy`'s territory, not ours.
#[test]
fn constant_source_declines() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy(&mut fd, bl, 0);
    let c = fd.new_constant(8, 0x4141414141414141);
    fd.op_set_input(op, c, 0).unwrap();
    assert!(readonly_source(&fd, op).is_none());
}

/// A writable memory source could differ at run time from the image bytes.
#[test]
fn writable_memory_source_declines() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy(&mut fd, bl, 0);
    let r = ram(&fd);
    let vn = fd.new_varnode(8, &Address::new(r, 0x2000), None);
    fd.op_set_input(op, vn, 0).unwrap();
    assert!(readonly_source(&fd, op).is_none());
}

/// A free read-only memory varnode is the source shape the rule claims.
#[test]
fn readonly_memory_source_is_accepted() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy(&mut fd, bl, 0);
    let r = ram(&fd);
    let vn = fd.new_varnode(8, &Address::new(r, 0x2000), None);
    fd.vbank_mut().get_mut(vn).unwrap().set_flags_pub(varnode_flags::readonly);
    fd.op_set_input(op, vn, 0).unwrap();
    assert_eq!(readonly_source(&fd, op), Some(vn));
}

/// A CAST between the memory read and the COPY is looked through.
#[test]
fn cast_between_read_and_copy_is_looked_through() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let r = ram(&fd);
    let cast = fd.new_op(1, Address::new(Rc::clone(&r), 0));
    fd.obank_mut().change_opcode(
        cast,
        TypeOp::new(OpCode::CPUI_CAST, 0, format!("{:?}", OpCode::CPUI_CAST)),
    );
    fd.op_insert_end(cast, bl);
    let src = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x2000), None);
    fd.vbank_mut().get_mut(src).unwrap().set_flags_pub(varnode_flags::readonly);
    fd.op_set_input(cast, src, 0).unwrap();
    let casted = fd.new_unique_out(8, cast).unwrap();
    let op = mk_copy(&mut fd, bl, 4);
    fd.op_set_input(op, casted, 0).unwrap();
    assert_eq!(readonly_source(&fd, op), Some(src));
}

// --- RuleRodataStringCopy gate ----------------------------------------------

/// Gate off: `apply_op` declines regardless of the op.
#[test]
fn rule_gate_off_is_noop() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy(&mut fd, bl, 0);
    let c = fd.new_constant(8, 0);
    fd.op_set_input(op, c, 0).unwrap();
    let mut rule = RuleRodataStringCopy::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

/// Gate on but no covering string symbol: still declines, leaving the op alone.
#[test]
fn rule_gate_on_without_symbol_declines() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_copy(&mut fd, bl, 0);
    let c = fd.new_constant(8, 0);
    fd.op_set_input(op, c, 0).unwrap();
    let mut rule = RuleRodataStringCopy::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

/// The rule matches on CPUI_COPY and clone-filters by group.
#[test]
fn rule_oplist_and_clone() {
    let rule = RuleRodataStringCopy::new(true);
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_COPY]);
    assert!(rule.clone_rule(&ActionGroupList::from_names(["analysis"])).is_some());
    assert!(rule.clone_rule(&ActionGroupList::from_names(["other"])).is_none());
}

// --- option parse -----------------------------------------------------------

#[test]
fn parse_on_sets_flag() {
    let (form, msg) = parse_rodata_string_form("on").unwrap();
    assert_eq!(form, RodataStringForm::On);
    assert!(form.rodata_string());
    assert_eq!(msg, "Read-only string block-copy recovery turned on");
}

#[test]
fn parse_off_clears_flag() {
    let (form, msg) = parse_rodata_string_form("off").unwrap();
    assert_eq!(form, RodataStringForm::Off);
    assert!(!form.rodata_string());
    assert_eq!(msg, "Read-only string block-copy recovery turned off");
}

#[test]
fn parse_empty_defaults_on() {
    let (form, _) = parse_rodata_string_form("").unwrap();
    assert_eq!(form, RodataStringForm::On);
}

#[test]
fn parse_garbage_is_error() {
    assert!(parse_rodata_string_form("maybe").is_err());
}

// --- specs ------------------------------------------------------------------

#[test]
fn specs_ship_default_on() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_COPY]);
    assert!(rule.clone_rule(&ActionGroupList::from_names(["analysis"])).is_some());
}
