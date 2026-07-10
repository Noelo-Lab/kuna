//! Tests for the early S2/S3 coreaction classes (`coreaction_early.rs`).
//!
//! Two kinds of checks:
//!
//!   1. **Identity** — every ported action carries the exact `name()` /
//!      `getGroup()` / flags the C++ constructor set, clones only when its group
//!      is in the [`ActionGroupList`], and `early_actions` enumerates the full
//!      set in C++ definition order.
//!   2. **Behavior on realized surfaces** — the parts of each `apply`/`reset`
//!      body that bind to primitives present in the merged tree are exercised on
//!      a hand-built [`Funcdata`]: `ActionStartTypes`/`ActionNormalizeSetup`
//!      flag-setting, the `ActionRedundBranch` single-out splice, the
//!      `ActionDeterminedBranch` constant-condition detection walk,
//!      `ActionLateDoNothing::removing_creates_redundancy`, the
//!      `ActionDoNothing` delayed-donothing sweep, and `ActionConstbase`'s
//!      no-blocks early return.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::action::{ruleflags, ActionContext, ActionGroupList};
use crate::context::{ArchContext, BlockId, OpId, TypeOp};

// -----------------------------------------------------------------------------
// Harness (mirrors funcdata_block.rs / action/tests.rs fixtures)
// -----------------------------------------------------------------------------

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

fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(ramspace(fd), off)
}

/// A grouplist containing exactly the named group.
fn glist(name: &str) -> ActionGroupList {
    ActionGroupList::from_names([name])
}

// -----------------------------------------------------------------------------
// Identity: name / group / flags
// -----------------------------------------------------------------------------

#[test]
fn names_groups_and_flags_match_cpp() {
    // (constructor, expected name, expected flags)
    let cases: Vec<(Box<dyn Action>, &str, u32)> = vec![
        (ActionStart::boxed("g"), "start", 0),
        (ActionStop::boxed("g"), "stop", 0),
        (ActionStartCleanUp::boxed("g"), "startcleanup", 0),
        (ActionStartTypes::boxed("g"), "starttypes", 0),
        (ActionConstbase::boxed("g"), "constbase", 0),
        (ActionSpacebase::boxed("g"), "spacebase", 0),
        (ActionHeritage::boxed("g"), "heritage", 0),
        (ActionNonzeroMask::boxed("g"), "nonzeromask", 0),
        (ActionVarnodeProps::boxed("g"), "varnodeprops", 0),
        (ActionUnreachable::boxed("g"), "unreachable", 0),
        (ActionDoNothing::boxed("g"), "donothing", ruleflags::rule_repeatapply),
        (ActionLateDoNothing::boxed("g"), "latedonothing", 0),
        (ActionRedundBranch::boxed("g"), "redundbranch", 0),
        (ActionDeterminedBranch::boxed("g"), "determinedbranch", 0),
        (ActionNormalizeSetup::boxed("g"), "normalizesetup", ruleflags::rule_onceperfunc),
    ];
    for (act, name, flags) in cases {
        assert_eq!(act.get_name(), name, "name mismatch for {name}");
        assert_eq!(act.get_group(), "g", "group mismatch for {name}");
        assert_eq!(act.base().flags, flags, "flags mismatch for {name}");
    }
}

#[test]
fn clone_filtered_respects_grouplist() {
    let act = ActionHeritage::boxed("ssa");
    // In-group => clone present and preserves name+group.
    let cloned = act.clone_filtered(&glist("ssa")).expect("should clone in-group");
    assert_eq!(cloned.get_name(), "heritage");
    assert_eq!(cloned.get_group(), "ssa");
    // Out-of-group => filtered out.
    assert!(act.clone_filtered(&glist("other")).is_none());
}

#[test]
fn early_actions_enumerates_full_set_in_cpp_order() {
    let acts = early_actions("base");
    let names: Vec<&str> = acts.iter().map(|a| a.get_name()).collect();
    assert_eq!(
        names,
        vec![
            "start",
            "stop",
            "startcleanup",
            "starttypes",
            "constbase",
            "spacebase",
            "heritage",
            "nonzeromask",
            "varnodeprops",
            "unreachable",
            "donothing",
            "latedonothing",
            "redundbranch",
            "determinedbranch",
            "normalizesetup",
        ]
    );
    for a in &acts {
        assert_eq!(a.get_group(), "base");
    }
}

// -----------------------------------------------------------------------------
// ActionStartTypes (realized): reset sets type recovery, apply flags the start
// -----------------------------------------------------------------------------

#[test]
fn starttypes_reset_and_apply() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionStartTypes::boxed("base");

    assert!(!fd.is_type_recovery_on());
    act.reset(&mut fd);
    assert!(fd.is_type_recovery_on(), "reset must setTypeRecovery(true)");

    // First apply: startTypeRecovery() returns true => count increments.
    assert!(!fd.has_type_recovery_started());
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert!(fd.has_type_recovery_started());
    assert_eq!(act.base().count, 1, "first start counts as a change");

    // Second apply: already started => no change.
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 1, "already-started start makes no change");
}

// -----------------------------------------------------------------------------
// ActionNormalizeSetup (realized reset): flips the normalization flag
// -----------------------------------------------------------------------------

#[test]
fn normalizesetup_reset_sets_normalization() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionNormalizeSetup::boxed("base");

    assert!(!fd.is_normalization_on());
    act.reset(&mut fd);
    assert!(fd.is_normalization_on(), "reset must setNormalization(true)");

    // apply body is the W4-stubbed lock-stripping => no change count.
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0);
}

// -----------------------------------------------------------------------------
// ActionConstbase (realized guard): no blocks => early return, no change
// -----------------------------------------------------------------------------

#[test]
fn constbase_no_blocks_early_return() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionConstbase::boxed("base");

    assert_eq!(fd.bblocks_get_size(), 0);
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0, "no blocks => no injected COPYs");

    // With a block present the W4 inject/track surface is stubbed => still 0.
    let root = fd.bblocks_root_pub();
    let _bb = fd.bblocks_mut().new_block_basic(root);
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0);
}

// -----------------------------------------------------------------------------
// ActionDoNothing (realized): the clearDelayedDonothing sweep runs every call
// -----------------------------------------------------------------------------

#[test]
fn donothing_clears_delayed_flag_on_every_block() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_root_pub();
    let a = fd.bblocks_mut().new_block_basic(root);
    let b = fd.bblocks_mut().new_block_basic(root);
    // Pre-set the delayed-donothing flag the sweep must clear.
    fd.bblocks_mut().block_mut(a).set_delayed_donothing();
    fd.bblocks_mut().block_mut(b).set_delayed_donothing();
    assert!(fd.bblocks_ref().block(a).is_delayed_donothing());

    let mut act = ActionDoNothing::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // The realized clearDelayedDonothing() sweep cleared both flags.
    assert!(!fd.bblocks_ref().block(a).is_delayed_donothing());
    assert!(!fd.bblocks_ref().block(b).is_delayed_donothing());
    // No do-nothing predicate / removal primitive => no block removed.
    assert_eq!(act.base().count, 0);
    assert_eq!(fd.bblocks_get_size(), 2);
}

// -----------------------------------------------------------------------------
// ActionLateDoNothing::removing_creates_redundancy (realized pure graph read)
// -----------------------------------------------------------------------------

#[test]
fn late_donothing_redundancy_detection() {
    // pred --(many)--> bl --> out, and pred also branches straight to out.
    // Removing bl would make pred->out redundant.
    let mut fd = build_fd();
    let root = fd.bblocks_root_pub();
    let pred = fd.bblocks_mut().new_block_basic(root);
    let bl = fd.bblocks_mut().new_block_basic(root);
    let out = fd.bblocks_mut().new_block_basic(root);
    // pred has two out edges: one to bl, one straight to out (both "lead to out").
    fd.bblocks_mut().add_edge(pred, bl);
    fd.bblocks_mut().add_edge(pred, out);
    fd.bblocks_mut().add_edge(bl, out);

    assert!(
        ActionLateDoNothing::removing_creates_redundancy(&fd, bl),
        "all of pred's other out edges lead to out => removing bl is redundant"
    );

    // A second predecessor whose other out-edge goes elsewhere is not redundant.
    let mut fd2 = build_fd();
    let root = fd2.bblocks_root_pub();
    let pred = fd2.bblocks_mut().new_block_basic(root);
    let bl = fd2.bblocks_mut().new_block_basic(root);
    let out = fd2.bblocks_mut().new_block_basic(root);
    let elsewhere = fd2.bblocks_mut().new_block_basic(root);
    fd2.bblocks_mut().add_edge(pred, bl);
    fd2.bblocks_mut().add_edge(pred, elsewhere); // not -> out
    fd2.bblocks_mut().add_edge(bl, out);
    assert!(
        !ActionLateDoNothing::removing_creates_redundancy(&fd2, bl),
        "pred has an out edge that does not lead to out => not redundant"
    );
}

#[test]
fn late_donothing_apply_skips_non_delayed_blocks() {
    // No block carries the delayed-donothing flag => the apply walk does nothing.
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_root_pub();
    let a = fd.bblocks_mut().new_block_basic(root);
    let b = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(a, b);

    let mut act = ActionLateDoNothing::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0);
    assert_eq!(fd.bblocks_get_size(), 2);
}

// -----------------------------------------------------------------------------
// ActionRedundBranch (realized splice path)
// -----------------------------------------------------------------------------

#[test]
fn redundbranch_splices_single_in_single_out_chain() {
    // entry -> bl -> out, with bl single-out and out single-in, no branch op in
    // bl: the C++ single-out arm calls spliceBlockBasic which merges out into bl
    // and counts a change.  (entry guards `out` from being an entry point.)
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_root_pub();
    let entry = fd.bblocks_mut().new_block_basic(root);
    let bl = fd.bblocks_mut().new_block_basic(root);
    let out = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(entry, bl);
    fd.bblocks_mut().add_edge(bl, out);
    fd.bblocks_mut().set_start_block(root, entry);

    let size_before = fd.bblocks_get_size();
    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0, "ActionRedundBranch returns 0 (full rule applied)");
    // The bl->out splice merged out into bl, removing one block and counting it.
    assert!(act.base().count >= 1, "single-out chain should splice at least once");
    assert!(
        fd.bblocks_get_size() < size_before,
        "splice removed at least one block"
    );
}

// -----------------------------------------------------------------------------
// ActionDeterminedBranch (realized detection walk)
// -----------------------------------------------------------------------------

/// Build a basic block whose terminating op is `CBRANCH cond` with the given
/// constant condition in slot 1; returns the op id.
fn block_with_const_cbranch(fd: &mut Funcdata, bl: BlockId, off: u64, cond: u64) -> OpId {
    let pc = ram_addr(fd, off);
    let op = fd.new_op(2, pc);
    let cvn = fd.new_constant(1, cond);
    fd.op_set_input(op, cvn, 1).unwrap();
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
    fd.op_insert_end(op, bl);
    op
}

#[test]
fn determinedbranch_detects_constant_condition_without_panicking() {
    // A block whose last op is CBRANCH on a constant: the realized detection walk
    // (lastOp == CBRANCH, condition isConstant) runs to completion and the dead
    // edge is severed by the now-ported `removeBranch` (the constant condition `1`
    // with no boolean-flip => `num = 1`, so out-edge 1 is removed and the CBRANCH
    // collapses to an unconditional BRANCH).  One change is counted.
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let t0 = fd.bblocks_mut().new_block_basic(root);
    let t1 = fd.bblocks_mut().new_block_basic(root);
    // Two out-edges so the CBRANCH has a real binary decision to fold.
    fd.bblocks_mut().add_edge(bl, t0);
    fd.bblocks_mut().add_edge(bl, t1);
    let _op = block_with_const_cbranch(&mut fd, bl, 0x1000, 1);

    let mut act = ActionDeterminedBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // removeBranch is now wired: the determined (constant) branch folds, count==1.
    assert_eq!(act.base().count, 1);
    // The block is now an unconditional branch (one out-edge severed).
    assert_eq!(fd.bblocks_ref().block(bl).size_out(), 1);
}

#[test]
fn determinedbranch_ignores_empty_and_non_cbranch_blocks() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_root_pub();
    // Empty block (lastOp == null).
    let _empty = fd.bblocks_mut().new_block_basic(root);
    // Block ending in a non-CBRANCH op.
    let other = fd.bblocks_mut().new_block_basic(root);
    let pc = ram_addr(&fd, 0x2000);
    let op = fd.new_op(1, pc);
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "CPUI_COPY"));
    fd.op_insert_end(op, other);

    let mut act = ActionDeterminedBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0);
}

// -----------------------------------------------------------------------------
// Stubbed wrappers: apply is a clean no-op returning 0 (no panic)
// -----------------------------------------------------------------------------

#[test]
fn stubbed_wrappers_apply_cleanly() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    for mut act in [
        ActionStart::boxed("base"),
        ActionStop::boxed("base"),
        ActionStartCleanUp::boxed("base"),
        ActionSpacebase::boxed("base"),
        ActionHeritage::boxed("base"),
        ActionNonzeroMask::boxed("base"),
        ActionVarnodeProps::boxed("base"),
        ActionUnreachable::boxed("base"),
    ] {
        let res = act.apply(&mut fd, &mut ctx);
        assert_eq!(res, 0, "{} should apply cleanly", act.get_name());
        assert_eq!(act.base().count, 0, "{} should make no change yet", act.get_name());
    }
}
