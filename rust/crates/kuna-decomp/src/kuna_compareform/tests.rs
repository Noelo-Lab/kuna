//! Tests for the comparison-form presentation sub-stage (kuna GH-558):
//! [`ActionPresentCompareForm`] restores a canonicalized `V < c+1` / `c-1 < V`
//! comparison to its source `V <= c` / `c <= V` LESSEQUAL form when the
//! `canonical_lessequal` provenance mark is set, plus the gate-off, no-mark,
//! shape-changed, and overflow-guard decline paths.
//!
//! These exercise [`restore_lessequal`] as the exact inverse of
//! `Funcdata::replace_lessequal`: we canonicalize with the ported
//! `replace_lessequal` (which sets the mark), then run the action and check it
//! lands back on the original op-code and constant.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::{int4, uintb};

use crate::action::ActionContext;
use crate::dtype::{type_metatype, Datatype};
use crate::seams::{Architecture, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

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

fn no_replace(
) -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(())
}

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
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

/// A non-constant value (output of a COPY) used as the variable operand `V`.
fn mk_var(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    set_def(fd, vn, defop)
}

fn run_action(fd: &mut Funcdata, enabled: bool) -> int4 {
    let mut action = ActionPresentCompareForm::new(enabled, "analysis");
    let mut ctx = ActionContext::new();
    action.apply(fd, &mut ctx);
    action.base().count
}

fn op_code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}
fn const_in(fd: &Funcdata, op: OpId, slot: int4) -> uintb {
    let v = fd.obank().get(op).unwrap().get_in(slot).unwrap();
    fd.vbank().get(v).unwrap().get_offset()
}

// --- V <= c  (constant on the right) -----------------------------------------

/// `V <= 5`  --canonicalize-->  `V < 6` (marked)  --restore-->  `V <= 5`.
#[test]
fn restores_unsigned_le_right() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let c = fd.new_constant(4, 5);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, v, op, 0);
    wire(&mut fd, c, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    // Canonicalize: V <= 5  =>  V < 6, with the provenance mark set.
    assert!(fd.replace_lessequal(op).unwrap());
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 1), 6);
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());

    // Restore.
    assert_eq!(run_action(&mut fd, true), 1);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
    assert_eq!(const_in(&fd, op, 1), 5);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    assert!(!fd.obank().get(op).unwrap().is_canonical_lessequal());
}

/// `V s<= 5`  --canonicalize-->  `V s< 6` (marked)  --restore-->  `V s<= 5`.
#[test]
fn restores_signed_le_right() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let c = fd.new_constant(4, 5);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_SLESSEQUAL);
    wire(&mut fd, v, op, 0);
    wire(&mut fd, c, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    assert!(fd.replace_lessequal(op).unwrap());
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_SLESS);
    assert_eq!(const_in(&fd, op, 1), 6);

    assert_eq!(run_action(&mut fd, true), 1);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_SLESSEQUAL);
    assert_eq!(const_in(&fd, op, 1), 5);
}

// --- c <= V  (constant on the left) ------------------------------------------

/// `5 <= V`  --canonicalize-->  `4 < V` (marked)  --restore-->  `5 <= V`.
#[test]
fn restores_unsigned_le_left() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let c = fd.new_constant(4, 5);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, c, op, 0);
    wire(&mut fd, v, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    // Canonicalize: 5 <= V  =>  4 < V.
    assert!(fd.replace_lessequal(op).unwrap());
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 0), 4);
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());

    // Restore: 4 < V  =>  5 <= V.
    assert_eq!(run_action(&mut fd, true), 1);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
    assert_eq!(const_in(&fd, op, 0), 5);
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v));
}

// --- gate off ----------------------------------------------------------------

/// With `present_lessequal` off the action is a no-op: the canonical form stays.
#[test]
fn gate_off_leaves_canonical_form() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let c = fd.new_constant(4, 5);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, v, op, 0);
    wire(&mut fd, c, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    assert!(fd.replace_lessequal(op).unwrap());

    assert_eq!(run_action(&mut fd, false), 0);
    // Untouched: still the canonical INT_LESS with the +1 constant and the mark.
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 1), 6);
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
}

// --- no provenance mark ------------------------------------------------------

/// An `INT_LESS` that was NOT produced by canonicalization (no mark) is skipped
/// even when the gate is on.
#[test]
fn unmarked_less_is_skipped() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let c = fd.new_constant(4, 6);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESS);
    wire(&mut fd, v, op, 0);
    wire(&mut fd, c, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    // No set_canonical_lessequal() here.

    assert_eq!(run_action(&mut fd, true), 0);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 1), 6);
}

// --- shape changed since canonicalization ------------------------------------

/// A marked op whose constant operand was replaced by a non-constant (shape
/// changed under us) declines gracefully and clears nothing.
#[test]
fn shape_changed_declines() {
    let mut fd = build_fd();
    let v0 = mk_var(&mut fd, 0x100, 4);
    let v1 = mk_var(&mut fd, 0x110, 4);
    // Both operands non-constant, but the op carries the provenance mark.
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESS);
    wire(&mut fd, v0, op, 0);
    wire(&mut fd, v1, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();

    // No rewrite (restore_lessequal returns false): count stays 0, op unchanged,
    // and the mark is NOT cleared (the C++ only clears on a successful restore).
    assert_eq!(run_action(&mut fd, true), 0);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
}

// --- overflow guards ---------------------------------------------------------

/// Unsigned guard: a marked `V < c+1` whose right constant is 0 would underflow
/// on restore (`c = -1`); the inverse declines.  (`diff == -1, val == 0`.)
#[test]
fn unsigned_underflow_guard_declines() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let zero = fd.new_constant(4, 0);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_LESS);
    wire(&mut fd, v, op, 0);
    wire(&mut fd, zero, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();

    assert_eq!(run_action(&mut fd, true), 0);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(const_in(&fd, op, 1), 0);
    // mark left intact since restore declined.
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
}

/// Signed guard: a marked `c-1 < V` whose left constant is INT_MAX would overflow
/// on restore (`c = INT_MAX+1`); the inverse declines.  (`diff == 1, val == INT_MAX`.)
#[test]
fn signed_overflow_guard_declines() {
    let mut fd = build_fd();
    let v = mk_var(&mut fd, 0x100, 4);
    let cmax = fd.new_constant(4, kuna_base::address::calc_int_max(4)); // 0x7fffffff
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_SLESS);
    wire(&mut fd, cmax, op, 0);
    wire(&mut fd, v, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    fd.obank_mut().get_mut(op).unwrap().set_canonical_lessequal();

    assert_eq!(run_action(&mut fd, true), 0);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_SLESS);
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
}

// --- option parse (OptionCompareForm::apply) ---------------------------------

/// `original` => `present_lessequal = true` (restore the source LESSEQUAL form);
/// the confirmation echoes the upstream `"Comparison presentation set to ... form"`.
#[test]
fn parse_original_sets_present_lessequal() {
    let (form, msg) = parse_compare_form("original").unwrap();
    assert_eq!(form, CompareForm::Original);
    assert!(form.present_lessequal()); // glb->present_lessequal = true
    assert_eq!(form.as_str(), "original");
    assert_eq!(msg, "Comparison presentation set to original form");
}

/// `canonical` => `present_lessequal = false` (leave the canonical `<`/`s<` form).
#[test]
fn parse_canonical_clears_present_lessequal() {
    let (form, msg) = parse_compare_form("canonical").unwrap();
    assert_eq!(form, CompareForm::Canonical);
    assert!(!form.present_lessequal()); // glb->present_lessequal = false
    assert_eq!(form.as_str(), "canonical");
    assert_eq!(msg, "Comparison presentation set to canonical form");
}

#[test]
fn parse_unknown_is_error() {
    // C++ `throw ParseError("Must specify compareform as 'canonical' or 'original'")`;
    // the Rust port surfaces it as a parse error carrying the same text.
    let err = parse_compare_form("neither").unwrap_err();
    assert!(format!("{err}").contains("Must specify compareform as 'canonical' or 'original'"));
    // empty string is also rejected (not "original"/"canonical").
    assert!(parse_compare_form("").is_err());
    // no case-folding: only the exact lowercase tokens match.
    assert!(parse_compare_form("Original").is_err());
    assert!(parse_compare_form("CANONICAL").is_err());
}

/// The parsed form's `present_lessequal` flag is exactly the gate
/// [`ActionPresentCompareForm`] consumes: `original` runs the restore, `canonical`
/// makes it a no-op.
#[test]
fn parsed_form_drives_gate() {
    let mk = |fd: &mut Funcdata| -> OpId {
        let v = mk_var(fd, 0x100, 4);
        let c = fd.new_constant(4, 5);
        let op = mk_op(fd, 0x108, 2, OpCode::CPUI_INT_LESSEQUAL);
        wire(fd, v, op, 0);
        wire(fd, c, op, 1);
        let out = fd.new_unique(1, Some(unk(1)));
        let _ = set_def(fd, out, op);
        assert!(fd.replace_lessequal(op).unwrap()); // V <= 5  =>  V < 6 (marked)
        op
    };

    // original: gate on => restore back to V <= 5.
    let mut fd = build_fd();
    let op = mk(&mut fd);
    let (orig, _) = parse_compare_form("original").unwrap();
    assert_eq!(run_action(&mut fd, orig.present_lessequal()), 1);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);

    // canonical: gate off => leave the canonical V < 6 in place.
    let mut fd = build_fd();
    let op = mk(&mut fd);
    let (canon, _) = parse_compare_form("canonical").unwrap();
    assert_eq!(run_action(&mut fd, canon.present_lessequal()), 0);
    assert_eq!(op_code_of(&fd, op), OpCode::CPUI_INT_LESS);
}

// --- clone filtering ---------------------------------------------------------

#[test]
fn clone_filtered_by_group() {
    let action = ActionPresentCompareForm::new(true, "analysis");
    let in_list = crate::action::ActionGroupList::from_names(["analysis"]);
    let out_list = crate::action::ActionGroupList::from_names(["other"]);
    assert!(action.clone_filtered(&in_list).is_some());
    assert!(action.clone_filtered(&out_list).is_none());
}
