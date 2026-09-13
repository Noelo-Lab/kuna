//! Tests for the lowered comparison-cascade -> switch recovery sub-stage (kuna,
//! angr `LoweredSwitchSimplifier`, S2): the read-only detection
//! ([`ActionLowerSwitchDetect::detect`] / [`recover_cascade`]) on a hand-built
//! lowered binary-search cascade, the default (gate-off) behavior, the sticky
//! side table, and the [`OptionLowerSwitch::apply`] parse.
//!
//! The install half (`Funcdata::kuna_install_lowered_switch`) is exercised
//! end-to-end by the `ghangr-loweredswitch` / `ghangr-loweredswitch-mv`
//! stage-model datatests; the unit tests below cover the install Action's gate /
//! store-lookup structure (the surgery declines gracefully on the empty unit `fd`,
//! which has no block at the recorded branch address).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::action::ActionContext;
use crate::context::{ArchContext, BlockId, TypeOp, VarnodeId};

// -----------------------------------------------------------------------------
// Harness (mirrors coreaction_early/tests.rs fixtures)
// -----------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
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
    build_fd_with_label_signedness(true)
}

fn build_fd_with_label_signedness(enabled: bool) -> Funcdata {
    let manage = build_manager();
    let mut context = ArchContext::new(manage);
    context.lowered_switch_labels = enabled;
    let glb = Rc::new(context);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(ramspace(fd), off)
}

/// Create a fresh basic block in the root graph and give it a one-byte cover at
/// `off` so `block_get_start` (the `targetStart` source) returns `off`.
fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let beg = ram_addr(fd, off);
    let end = ram_addr(fd, off);
    fd.set_basic_block_range(bl, &beg, &end);
    bl
}

/// Append a `CBRANCH( bool(cmp_op(V, c)) )` to block `bl` at op address `off`,
/// where `cmp_op` is an integer comparison.  Slot 0 of the comparison is the
/// switch variable `v`, slot 1 the constant `c`.  Out-edge 0 is the \b false
/// branch, out-edge 1 the \b true branch (must be wired by the caller via
/// `add_edge` in false-then-true order, matching `getFalseOut`/`getTrueOut`).
fn append_cmp_cbranch(
    fd: &mut Funcdata,
    bl: BlockId,
    off: u64,
    cmp_code: OpCode,
    v: VarnodeId,
    c: u64,
) {
    // bool = cmp_code(V, c)   (a written boolean def in a unique varnode)
    let cmp = fd.new_op(2, ram_addr(fd, off));
    fd.op_set_opcode(cmp, TypeOp::new(cmp_code, 0, "cmp"));
    fd.op_set_input(cmp, v, 0).unwrap();
    let cvn = fd.new_constant(vn_size_pub(fd, v), c);
    fd.op_set_input(cmp, cvn, 1).unwrap();
    let boolvn = fd.new_unique_out(1, cmp).unwrap();
    fd.op_insert_end(cmp, bl);

    // CBRANCH(<dest>, bool)
    let cb = fd.new_op(2, ram_addr(fd, off + 1));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
    // slot 0 is the (annotation) destination; only slot 1 (the condition) is read.
    let dest = fd.new_constant(8, 0);
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, boolvn, 1).unwrap();
    fd.op_insert_end(cb, bl);
}

fn vn_size_pub(fd: &Funcdata, vn: VarnodeId) -> int4 {
    fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

/// The switch variable `V` — a single SSA Varnode defined once and read by every
/// comparison in the cascade (the pre-merge identity the detection keys on).
///
/// The IR forbids a *free* (undefined, non-constant) Varnode from having more
/// than one descendant, so `V` must be a *written* value to be read by many
/// comparison ops — exactly the post-simplification shape (the dispatch value is
/// defined once and not redefined between comparisons).  We define `V` at
/// register `0x4000` as the output of a `LOAD` (a non-transparent op, so
/// `canonSwitchVar` keeps it as-is and every comparison canonicalizes to the
/// same `V` id).  The defining op lives in its own predecessor block so it does
/// not contaminate the pure-compare spine blocks.
fn switch_var(fd: &mut Funcdata) -> VarnodeId {
    switch_var_sized(fd, 4)
}

fn switch_var_sized(fd: &mut Funcdata, size: int4) -> VarnodeId {
    let def_bl = new_block(fd, 0x0F00);
    // V = LOAD(ram, ptr)   — a written value at register 0x4000.
    let load = fd.new_op(2, ram_addr(fd, 0x0F00));
    fd.op_set_opcode(load, TypeOp::new(OpCode::CPUI_LOAD, 0, "LOAD"));
    let spaceid = fd.new_constant(4, 0);
    let ptr = fd.new_constant(8, 0x5000);
    fd.op_set_input(load, spaceid, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let vaddr = ram_addr(fd, 0x4000);
    let v = fd.new_varnode_out(size, &vaddr, load).unwrap();
    fd.op_insert_end(load, def_bl);
    v
}

fn extend_switch_var(
    fd: &mut Funcdata,
    v: VarnodeId,
    code: OpCode,
    out_size: int4,
    off: u64,
) -> VarnodeId {
    assert!(code == OpCode::CPUI_INT_ZEXT || code == OpCode::CPUI_INT_SEXT);
    let def_bl = new_block(fd, off);
    let ext = fd.new_op(1, ram_addr(fd, off));
    fd.op_set_opcode(ext, TypeOp::new(code, 0, "extension"));
    fd.op_set_input(ext, v, 0).unwrap();
    let out = fd.new_unique_out(out_size, ext).unwrap();
    fd.op_insert_end(ext, def_bl);
    out
}

fn truncate_switch_var(fd: &mut Funcdata, v: VarnodeId, out_size: int4, off: u64) -> VarnodeId {
    assert!(out_size > 0 && out_size < vn_size_pub(fd, v));
    let def_bl = new_block(fd, off);
    let subpiece = fd.new_op(2, ram_addr(fd, off));
    fd.op_set_opcode(subpiece, TypeOp::new(OpCode::CPUI_SUBPIECE, 0, "SUBPIECE"));
    fd.op_set_input(subpiece, v, 0).unwrap();
    let zero = fd.new_constant(4, 0);
    fd.op_set_input(subpiece, zero, 1).unwrap();
    let out = fd.new_unique_out(out_size, subpiece).unwrap();
    fd.op_insert_end(subpiece, def_bl);
    out
}

fn build_signedness_cascade_for_vars(
    fd: &mut Funcdata,
    range_v: VarnodeId,
    equality_vars: [VarnodeId; 3],
    range_code: OpCode,
    cases: [u64; 3],
) -> BlockId {
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let case_a = new_block(fd, 0x2000);
    let case_b = new_block(fd, 0x2100);
    let default = new_block(fd, 0x3000);

    append_cmp_cbranch(fd, b0, 0x1000, range_code, range_v, cases[1]);
    fd.bblocks_mut().add_edge(b0, b2);
    fd.bblocks_mut().add_edge(b0, b1);
    append_cmp_cbranch(
        fd,
        b1,
        0x1100,
        OpCode::CPUI_INT_EQUAL,
        equality_vars[0],
        cases[0],
    );
    fd.bblocks_mut().add_edge(b1, default);
    fd.bblocks_mut().add_edge(b1, case_a);
    append_cmp_cbranch(
        fd,
        b2,
        0x1200,
        OpCode::CPUI_INT_EQUAL,
        equality_vars[1],
        cases[1],
    );
    fd.bblocks_mut().add_edge(b2, b3);
    fd.bblocks_mut().add_edge(b2, case_b);
    append_cmp_cbranch(
        fd,
        b3,
        0x1300,
        OpCode::CPUI_INT_EQUAL,
        equality_vars[2],
        cases[2],
    );
    fd.bblocks_mut().add_edge(b3, default);
    fd.bblocks_mut().add_edge(b3, case_a);
    b0
}

fn build_signedness_cascade(fd: &mut Funcdata, size: int4, range_code: OpCode, cases: [u64; 3]) {
    let v = switch_var_sized(fd, size);
    let _ = build_signedness_cascade_for_vars(fd, v, [v; 3], range_code, cases);
}

/// Put an ambiguous byte projection in front of an otherwise-valid wide-value
/// range cascade. If detection were to discard the outer node instead of keeping
/// it in the structural map, the inner three-case subtree could recover alone.
fn build_shrinking_subpiece_outer_with_inner_cascade(
    fd: &mut Funcdata,
) -> (VarnodeId, BlockId, BlockId) {
    // The LOAD leaves all upper 24 bits capable of being nonzero. Replacing the
    // byte projection with this four-byte storage would therefore let those bits
    // select a different BRANCHIND target than the outer comparison.
    let wide = switch_var_sized(fd, 4);
    let low_byte = truncate_switch_var(fd, wide, 1, 0x0F10);
    let inner = build_signedness_cascade_for_vars(
        fd,
        wide,
        [wide; 3],
        OpCode::CPUI_INT_LESS,
        [0x1e, 0x7c, 0x8b],
    );

    let outer = new_block(fd, 0x0FF0);
    let projected_case = new_block(fd, 0x3100);
    append_cmp_cbranch(fd, outer, 0x0FF0, OpCode::CPUI_INT_EQUAL, low_byte, 0xfe);
    fd.bblocks_mut().add_edge(outer, inner);
    fd.bblocks_mut().add_edge(outer, projected_case);
    (wide, outer, inner)
}

/// Build a 4-block GCC-lowered binary-search cascade dispatching on `V`:
///
/// ```text
///   B0 (range):  if (V < 2)        true -> B1   false -> B2
///   B1 (equal):  if (V == 0)       match-> A    cont -> DEFAULT
///   B2 (equal):  if (V == 2)       match-> B    cont -> B3
///   B3 (equal):  if (V == 3)       match-> A    cont -> DEFAULT
/// ```
///
/// => 3 distinct case values {0->A, 2->B, 3->A}, 2 distinct targets {A,B}, a
/// range node (so `saw_range`), and DEFAULT voted twice.  Returns the head id.
fn build_lowered_cascade(fd: &mut Funcdata) -> BlockId {
    let v = switch_var(fd);

    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let case_a = new_block(fd, 0x2000);
    let case_b = new_block(fd, 0x2100);
    let default = new_block(fd, 0x3000);

    // B0: range  if (V < 2)  -> false: B2, true: B1   (INT_LESS, not flipped)
    append_cmp_cbranch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    fd.bblocks_mut().add_edge(b0, b2); // out 0 = false (V >= 2)
    fd.bblocks_mut().add_edge(b0, b1); // out 1 = true  (V < 2)

    // B1: equal  if (V == 0)  -> false: DEFAULT(cont), true: A(match)
    append_cmp_cbranch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    fd.bblocks_mut().add_edge(b1, default); // out 0 = false = contA
    fd.bblocks_mut().add_edge(b1, case_a); // out 1 = true  = matchOut

    // B2: equal  if (V == 2)  -> false: B3(cont), true: B(match)
    append_cmp_cbranch(fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 2);
    fd.bblocks_mut().add_edge(b2, b3); // out 0 = false = contA
    fd.bblocks_mut().add_edge(b2, case_b); // out 1 = true  = matchOut

    // B3: equal  if (V == 3)  -> false: DEFAULT(cont), true: A(match)
    append_cmp_cbranch(fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 3);
    fd.bblocks_mut().add_edge(b3, default); // out 0 = false = contA
    fd.bblocks_mut().add_edge(b3, case_a); // out 1 = true  = matchOut

    b0
}

// -----------------------------------------------------------------------------
// ElementId
// -----------------------------------------------------------------------------

#[test]
fn element_id_matches_cpp() {
    assert_eq!(ELEM_LOWEREDSWITCH.get_name(), "loweredswitch");
    assert_eq!(ELEM_LOWEREDSWITCH.get_id(), 4019);
}

// -----------------------------------------------------------------------------
// Detection on the hand-built cascade (the core mechanism)
// -----------------------------------------------------------------------------

#[test]
fn detect_recovers_lowered_switch() {
    let mut fd = build_fd();
    let _head = build_lowered_cascade(&mut fd);

    let mut act = ActionLowerSwitchDetect::new(true, "base"); // gate on
    let added = act.detect(&mut fd);
    assert_eq!(added, 1, "one cascade should be recorded");

    // The restart was requested (mechanism-c restart idiom).
    assert!(
        fd.has_restart_pending(),
        "detect should set restart-pending"
    );

    // The sticky side table now reports a record (kunaLoweredSwitchHasRecord).
    assert!(act.store().borrow().has_record(&fd));
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs.len(), 1);
    let rec = &recs[0];

    // 3 distinct case values {0,2,3}, ordered by the BTreeMap (== std::map) key.
    assert_eq!(rec.case_vals, vec![0u64, 2u64, 3u64]);
    // Parallel targets: 0->A(0x2000), 2->B(0x2100), 3->A(0x2000).
    let off = |a: &Address| a.get_offset();
    assert_eq!(off(&rec.case_targets[0]), 0x2000); // case 0 -> A
    assert_eq!(off(&rec.case_targets[1]), 0x2100); // case 2 -> B
    assert_eq!(off(&rec.case_targets[2]), 0x2000); // case 3 -> A
                                                   // Default is the most-voted common sink (DEFAULT, voted by B1 and B3).
    assert_eq!(off(&rec.default_target), 0x3000);
    // The synthetic BRANCHIND will host at the head's CBRANCH address (0x1001).
    assert_eq!(off(&rec.branch_addr), 0x1001);
    // The switch variable storage (the 4-byte register at 0x4000).
    assert_eq!(off(&rec.var_addr), 0x4000);
    assert_eq!(rec.var_size, 4);
    assert!(!rec.signed_labels);
}

#[test]
fn signed_byte_high_case_keeps_signed_labels() {
    let mut fd = build_fd();
    build_signedness_cascade(&mut fd, 1, OpCode::CPUI_INT_SLESS, [0x1e, 0x7c, 0x8b]);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![0x1e, 0x7c, 0x8b]);
    assert!(
        recs[0].signed_labels,
        "0x8b is -117 under signed byte comparisons"
    );
}

#[test]
fn signed_int_minus_one_keeps_signed_labels() {
    let mut fd = build_fd();
    build_signedness_cascade(
        &mut fd,
        4,
        OpCode::CPUI_INT_SLESS,
        [0x7c, 0x8000_0000, 0xffff_ffff],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![0x7c, 0x8000_0000, 0xffff_ffff]);
    assert!(
        recs[0].signed_labels,
        "0xffffffff is -1 for a signed int selector"
    );
}

#[test]
fn unsigned_u32_high_case_keeps_unsigned_labels() {
    let mut fd = build_fd();
    build_signedness_cascade(
        &mut fd,
        4,
        OpCode::CPUI_INT_LESS,
        [1, 0x8000_0000, 0xffff_ffff],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![1, 0x8000_0000, 0xffff_ffff]);
    assert!(!recs[0].signed_labels, "0xffffffff remains unsigned");
}

#[test]
fn zext_selector_with_signed_range_declines_recovery() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let zext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_ZEXT, 4, 0x0F10);
    let head = build_signedness_cascade_for_vars(
        &mut fd,
        zext,
        [zext; 3],
        OpCode::CPUI_INT_SLESS,
        [0x1e, 200, 0x8b],
    );

    let head_cmp = analyze_cmp(&fd, head);
    assert!(
        head_cmp.valid,
        "ambiguous head must remain in the structural map"
    );
    assert!(head_cmp.evidence_ambiguous);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        0,
        "ZEXT and SLESS give conflicting evidence"
    );
    assert!(!act.store().borrow().has_record(&fd));
}

#[test]
fn option_off_zext_signed_range_retains_legacy_recovery() {
    let mut fd = build_fd_with_label_signedness(false);
    let v = switch_var_sized(&mut fd, 1);
    let zext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_ZEXT, 4, 0x0F10);
    build_signedness_cascade_for_vars(
        &mut fd,
        zext,
        [zext; 3],
        OpCode::CPUI_INT_SLESS,
        [0x1e, 200, 0x8b],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        1,
        "option off must ignore extension conflicts"
    );
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![0x1e, 0x8b, 0xc8]);
    assert!(
        recs[0].signed_labels,
        "legacy high-bit heuristic remains authoritative"
    );
}

#[test]
fn sext_selector_with_unsigned_range_declines_recovery() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let sext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_SEXT, 4, 0x0F10);
    build_signedness_cascade_for_vars(
        &mut fd,
        sext,
        [sext; 3],
        OpCode::CPUI_INT_LESS,
        [0x1e, 0x7c, 0xffff_ff8b],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        0,
        "SEXT and LESS give conflicting evidence"
    );
    assert!(!act.store().borrow().has_record(&fd));
}

#[test]
fn zext_selector_with_unsigned_range_recovers_unsigned_labels() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let zext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_ZEXT, 4, 0x0F10);
    build_signedness_cascade_for_vars(
        &mut fd,
        zext,
        [zext; 3],
        OpCode::CPUI_INT_LESS,
        [0x1e, 0x7c, 0x8b],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![0x1e, 0x7c, 0x8b]);
    assert_eq!(recs[0].var_size, 1);
    assert!(!recs[0].signed_labels);
}

#[test]
fn sext_selector_with_signed_range_recovers_signed_labels() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let sext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_SEXT, 4, 0x0F10);
    build_signedness_cascade_for_vars(
        &mut fd,
        sext,
        [sext; 3],
        OpCode::CPUI_INT_SLESS,
        [0x1e, 0x7c, 0xffff_ff8b],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![0x1e, 0x7c, 0x8b]);
    assert_eq!(recs[0].var_size, 1);
    assert!(recs[0].signed_labels);
}

#[test]
fn mixed_extension_paths_decline_recovery() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let zext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_ZEXT, 4, 0x0F10);
    let sext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_SEXT, 4, 0x0F20);
    build_signedness_cascade_for_vars(
        &mut fd,
        zext,
        [zext, zext, sext],
        OpCode::CPUI_INT_LESS,
        [0x1e, 0x3c, 0x6b],
    );

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        0,
        "ZEXT and SEXT paths must not be merged"
    );
    assert!(!act.store().borrow().has_record(&fd));
}

#[test]
fn shrinking_subpiece_declines_without_recovering_inner_cascade() {
    let mut fd = build_fd();
    let (wide, outer, inner) = build_shrinking_subpiece_outer_with_inner_cascade(&mut fd);

    let head_cmp = analyze_cmp(&fd, outer);
    assert!(
        head_cmp.valid,
        "ambiguous outer head must keep ownership of the inner subtree"
    );
    assert_eq!(head_cmp.var, Some(wide));
    assert_eq!(head_cmp.effective_size, 1);
    assert!(head_cmp.evidence_ambiguous);
    let inner_cmp = analyze_cmp(&fd, inner);
    assert!(inner_cmp.valid);
    assert_eq!(inner_cmp.var, Some(wide));
    assert!(!inner_cmp.evidence_ambiguous);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        0,
        "strict mode cannot install a wide selector for a byte projection"
    );
    assert!(
        !act.store().borrow().has_record(&fd),
        "neither the outer head nor its inner three-case/range subtree may recover"
    );
}

#[test]
fn option_off_shrinking_subpiece_retains_legacy_wide_selector_peel() {
    let mut fd = build_fd_with_label_signedness(false);
    let (_wide, _outer, _inner) = build_shrinking_subpiece_outer_with_inner_cascade(&mut fd);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        1,
        "option off must retain the historical shrinking-SUBPIECE peel"
    );
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].var_size, 4);
    assert_eq!(recs[0].case_vals, vec![0x1e, 0x7c, 0x8b, 0xfe]);
    assert!(!recs[0].signed_labels);
}

#[test]
fn explicit_ablation_retains_legacy_case_bit_guess() {
    let mut fd = build_fd_with_label_signedness(false);
    build_signedness_cascade(&mut fd, 1, OpCode::CPUI_INT_LESS, [0x1e, 0x7c, 0x8b]);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    assert!(act.store().borrow().records(&fd)[0].signed_labels);
}

#[test]
fn mixed_signed_and_unsigned_range_comparisons_decline_recovery() {
    let mut fd = build_fd();
    build_mixed_range_cascade(&mut fd);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 0);
    assert!(!act.store().borrow().has_record(&fd));
}

#[test]
fn option_off_mixed_range_cascade_retains_legacy_recovery() {
    let mut fd = build_fd_with_label_signedness(false);
    build_mixed_range_cascade(&mut fd);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        1,
        "option off must not apply new conflict checks"
    );
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs[0].case_vals, vec![2, 7, 12]);
    assert!(
        !recs[0].signed_labels,
        "legacy high-bit heuristic remains authoritative"
    );
}

fn build_mixed_range_cascade(fd: &mut Funcdata) {
    let v = switch_var_sized(fd, 1);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let b4 = new_block(fd, 0x1400);
    let case_a = new_block(fd, 0x2000);
    let case_b = new_block(fd, 0x2100);
    let case_c = new_block(fd, 0x2200);
    let default = new_block(fd, 0x3000);

    append_cmp_cbranch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 10);
    fd.bblocks_mut().add_edge(b0, b2);
    fd.bblocks_mut().add_edge(b0, b1);
    append_cmp_cbranch(fd, b1, 0x1100, OpCode::CPUI_INT_SLESS, v, 5);
    fd.bblocks_mut().add_edge(b1, b4);
    fd.bblocks_mut().add_edge(b1, b3);

    for (block, off, value, target) in [
        (b2, 0x1200, 12, case_a),
        (b3, 0x1300, 2, case_b),
        (b4, 0x1400, 7, case_c),
    ] {
        append_cmp_cbranch(fd, block, off, OpCode::CPUI_INT_EQUAL, v, value);
        fd.bblocks_mut().add_edge(block, default);
        fd.bblocks_mut().add_edge(block, target);
    }
}

// (restart-pending is read via `has_restart_pending`).

#[test]
fn detect_is_sticky_and_idempotent() {
    let mut fd = build_fd();
    let _head = build_lowered_cascade(&mut fd);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
    // A second detect on the same function is a no-op (already discovered).
    assert_eq!(act.detect(&mut fd), 0, "sticky: no second record");
    assert_eq!(act.store().borrow().records(&fd).len(), 1);
}

// -----------------------------------------------------------------------------
// Gate-off and other early returns
// -----------------------------------------------------------------------------

#[test]
fn gate_off_records_nothing() {
    let mut fd = build_fd();
    let _head = build_lowered_cascade(&mut fd);

    // option loweredswitch off => the Detect Action is inert.
    let mut act = ActionLowerSwitchDetect::new(false, "base");
    assert_eq!(act.detect(&mut fd), 0);
    assert!(!act.store().borrow().has_record(&fd));
    assert!(!fd.has_restart_pending());
}

#[test]
fn apply_returns_zero_and_leaves_count_untouched() {
    // The C++ apply returns 0 (no op change) so the fullloop quiesces; the
    // detection's `count` stays 0 even when a record is added.
    let mut fd = build_fd();
    let _head = build_lowered_cascade(&mut fd);
    let mut ctx = ActionContext::new();

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0, "apply returns 0 to quiesce the loop");
    assert_eq!(act.base().count, 0, "apply does not count a change");
    // The record was still stored and the restart requested.
    assert!(act.store().borrow().has_record(&fd));
    assert!(fd.has_restart_pending());
}

#[test]
fn linear_equality_chain_is_not_a_switch() {
    // A purely linear equality chain (no range/binary-search split) is a
    // hand-written if/else-if, not a lowered switch: `!sawRange` => bail.
    let mut fd = build_fd();
    let v = switch_var(&mut fd);

    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let b2 = new_block(&mut fd, 0x1200);
    let case_a = new_block(&mut fd, 0x2000);
    let case_b = new_block(&mut fd, 0x2100);
    let case_c = new_block(&mut fd, 0x2200);
    let default = new_block(&mut fd, 0x3000);

    // B0: if (V == 0) -> A else B1
    append_cmp_cbranch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 0);
    fd.bblocks_mut().add_edge(b0, b1); // false = cont
    fd.bblocks_mut().add_edge(b0, case_a); // true = match
                                           // B1: if (V == 1) -> B else B2
    append_cmp_cbranch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 1);
    fd.bblocks_mut().add_edge(b1, b2);
    fd.bblocks_mut().add_edge(b1, case_b);
    // B2: if (V == 2) -> C else DEFAULT
    append_cmp_cbranch(&mut fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 2);
    fd.bblocks_mut().add_edge(b2, default);
    fd.bblocks_mut().add_edge(b2, case_c);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(
        act.detect(&mut fd),
        0,
        "linear equality chain rejected (no range node)"
    );
    assert!(!act.store().borrow().has_record(&fd));
}

#[test]
fn too_few_cases_rejected() {
    // Two equality cases + a range node: fewer than the RULE3 minimum of 3 cases.
    let mut fd = build_fd();
    let v = switch_var(&mut fd);

    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let case_a = new_block(&mut fd, 0x2000);
    let case_b = new_block(&mut fd, 0x2100);
    let default = new_block(&mut fd, 0x3000);

    // B0: range  if (V < 1)  -> false: B1, true: A   (one case via the range edge)
    append_cmp_cbranch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 1);
    fd.bblocks_mut().add_edge(b0, b1); // false = contB
    fd.bblocks_mut().add_edge(b0, case_a); // true  = contA (a non-cascade sink)
                                           // B1: equal  if (V == 2)  -> false: DEFAULT, true: B
    append_cmp_cbranch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 2);
    fd.bblocks_mut().add_edge(b1, default);
    fd.bblocks_mut().add_edge(b1, case_b);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 0, "fewer than 3 cases rejected");
    assert!(!act.store().borrow().has_record(&fd));
}

// -----------------------------------------------------------------------------
// canon_switch_var: peels COPY/CAST/ZEXT/SEXT/zero-SUBPIECE
// -----------------------------------------------------------------------------

#[test]
fn canon_switch_var_peels_transparent_ops() {
    let mut fd = build_fd();
    // base leaf register V (size 4)
    let v = switch_var(&mut fd);
    // w = ZEXT(V)  (size 8)
    let bl = new_block(&mut fd, 0x1000);
    let zext = fd.new_op(1, ram_addr(&fd, 0x1000));
    fd.op_set_opcode(zext, TypeOp::new(OpCode::CPUI_INT_ZEXT, 0, "ZEXT"));
    fd.op_set_input(zext, v, 0).unwrap();
    let w = fd.new_unique_out(8, zext).unwrap();
    fd.op_insert_end(zext, bl);

    // A same-width zero-offset SUBPIECE is still transparent and must not hide
    // the widening evidence below it.
    let subpiece = fd.new_op(2, ram_addr(&fd, 0x1001));
    fd.op_set_opcode(subpiece, TypeOp::new(OpCode::CPUI_SUBPIECE, 0, "SUBPIECE"));
    fd.op_set_input(subpiece, w, 0).unwrap();
    let zero = fd.new_constant(4, 0);
    fd.op_set_input(subpiece, zero, 1).unwrap();
    let same_width = fd.new_unique_out(8, subpiece).unwrap();
    fd.op_insert_end(subpiece, bl);

    // canonSwitchVar(w) peels the ZEXT back to V without discarding its
    // unsigned interpretation or the byte selector's effective width.
    assert_eq!(
        canon_switch_var(&fd, w),
        CanonSwitchVar {
            var: v,
            effective_size: 4,
            extension_signedness: Some(LoweredSwitchSignedness::Unsigned),
            evidence_ambiguous: false,
        }
    );
    assert_eq!(canon_switch_var(&fd, same_width), canon_switch_var(&fd, w));
    // A leaf (unwritten) varnode is returned as-is.
    assert_eq!(
        canon_switch_var(&fd, v),
        CanonSwitchVar {
            var: v,
            effective_size: 4,
            extension_signedness: None,
            evidence_ambiguous: false,
        }
    );
}

#[test]
fn canon_switch_var_rejects_conflicting_extension_chain() {
    let mut fd = build_fd();
    let v = switch_var_sized(&mut fd, 1);
    let zext = extend_switch_var(&mut fd, v, OpCode::CPUI_INT_ZEXT, 2, 0x0F10);
    let sext = extend_switch_var(&mut fd, zext, OpCode::CPUI_INT_SEXT, 4, 0x0F20);

    let canon = canon_switch_var(&fd, sext);
    assert_eq!(canon.var, v);
    assert!(canon.evidence_ambiguous);
}

// -----------------------------------------------------------------------------
// OptionLowerSwitch::apply (the gate parse + message)
// -----------------------------------------------------------------------------

#[test]
fn option_apply_on() {
    let (val, msg) = OptionLowerSwitch.apply("on").unwrap();
    assert!(val);
    assert_eq!(msg, "Lowered comparison-cascade switch recovery turned on");
}

#[test]
fn option_apply_off() {
    let (val, msg) = OptionLowerSwitch.apply("off").unwrap();
    assert!(!val);
    assert_eq!(msg, "Lowered comparison-cascade switch recovery turned off");
}

#[test]
fn option_apply_empty_is_on() {
    // C++ onOrOff: empty string => true (on).
    let (val, _msg) = OptionLowerSwitch.apply("").unwrap();
    assert!(val);
}

#[test]
fn option_apply_bad_value_errors() {
    assert!(OptionLowerSwitch.apply("maybe").is_err());
    assert_eq!(OptionLowerSwitch::NAME, "loweredswitch");
}

// -----------------------------------------------------------------------------
// key_for_func / store basics
// -----------------------------------------------------------------------------

#[test]
fn key_for_func_uses_entry_address() {
    let fd = build_fd();
    let key = key_for_func(&fd);
    assert_eq!(key.offset, 0x1000);
    assert!(key.space_index >= 0);
}

#[test]
fn empty_store_has_no_record() {
    let fd = build_fd();
    let store = KunaLoweredSwitchStore::new();
    assert!(!store.has_record(&fd));
    assert!(store.records(&fd).is_empty());
}

// -----------------------------------------------------------------------------
// ActionLowerSwitchInstall (the install half).
//
// The CFG surgery (Funcdata::kunaInstallLoweredSwitch) is a documented
// STUB(W7/W4) — getHeritagePass, a real JumpTable registry, kunaSetTrivialModel,
// and removeUnreachableBlocks are all unported — so `install` always declines
// (0).  These cover the ported structure: gate-off, no-record short-circuit, the
// has-record path still declining at the surgery seam, and clone filtering.
// -----------------------------------------------------------------------------

/// A minimal recovered record for `fd` (the shape the Detect half produces).
fn sample_record(fd: &Funcdata) -> KunaLoweredSwitchRecord {
    let a = |off: u64| ram_addr(fd, off);
    KunaLoweredSwitchRecord {
        branch_addr: a(0x1001),
        var_addr: a(0x4000),
        var_size: 4,
        case_vals: vec![0, 2, 3],
        case_targets: vec![a(0x2000), a(0x2100), a(0x2000)],
        default_target: a(0x3000),
        signed_labels: false,
        value: crate::p2_lift::kuna_loweredswitchvalue::ValueCheck::default(),
    }
}

#[test]
fn install_gate_off_is_inert() {
    let mut fd = build_fd();
    let mut act = ActionLowerSwitchInstall::new(false, "base"); // gate off
    let rec = sample_record(&fd);
    act.store_mut().borrow_mut().push(&fd, rec);
    // Even with a record present, the gate-off install declines.
    assert_eq!(act.install(&mut fd), 0);
}

#[test]
fn install_no_record_short_circuits() {
    let mut fd = build_fd();
    let mut act = ActionLowerSwitchInstall::new(true, "base"); // gate on, empty store
    assert!(!act.store().borrow().has_record(&fd));
    assert_eq!(act.install(&mut fd), 0);
}

#[test]
fn install_with_record_declines_when_cfg_does_not_match() {
    let mut fd = build_fd();
    let mut act = ActionLowerSwitchInstall::new(true, "base");
    let rec = sample_record(&fd);
    act.store_mut().borrow_mut().push(&fd, rec);
    assert!(
        act.store().borrow().has_record(&fd),
        "the record is present"
    );
    // The store lookup succeeds, but the empty test `fd` has no block whose
    // terminator is at the recorded branch address, so the surgery declines
    // gracefully (Ok(None)) rather than corrupting the CFG.
    assert_eq!(act.install(&mut fd), 0);
}

#[test]
fn install_apply_leaves_count_zero() {
    let mut fd = build_fd();
    let mut act = ActionLowerSwitchInstall::new(true, "base");
    let rec = sample_record(&fd);
    act.store_mut().borrow_mut().push(&fd, rec);
    let mut ctx = ActionContext::default();
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0, "no install => count untouched");
}

#[test]
fn install_clone_filters_by_group() {
    let act = ActionLowerSwitchInstall::new(true, "switchnorm");
    let inc = crate::action::ActionGroupList::from_names(["switchnorm"]);
    let exc = crate::action::ActionGroupList::from_names(["cleanup"]);
    assert!(act.clone_filtered(&inc).is_some());
    assert!(act.clone_filtered(&exc).is_none());
}
