//! Independent verifier adversarial tests for item `w5-s3-coreaction-early`
//! (`decompiler/cpp/coreaction.{cc,hh}` — early S2/S3 Action classes).
//!
//! These target the *realized, publicly observable* spots the hunt list flagged
//! as most fragile.  The block-graph surgery primitives (removeBranch /
//! removeDoNothingBlock / removeUnreachableBlocks) and the do-nothing predicate
//! are documented seams, so they cannot be exercised end-to-end here; the
//! realized output-affecting behavior is:
//!
//!   - `ActionRedundBranch::apply` single-out **splice gating**
//!     (coreaction.cc:3663-3671): the three guards `sizeIn()==1`,
//!     `!isEntryPoint()`, `!isSwitchOut()` must each independently *suppress* the
//!     splice, and the `i = -1` scan-reset must re-walk from the top so a chain
//!     collapses fully in one `apply` call.
//!   - `ActionDeterminedBranch::apply` detection walk (coreaction.cc:3696-3705):
//!     last-op == CBRANCH, slot-1 constant gate, and the `num` boolean-flip XOR
//!     — must not panic and must skip non-constant / non-CBRANCH / empty blocks.
//!   - `ActionStartTypes` (coreaction.hh:82) `startTypeRecovery()` once-only
//!     transition: `count` increments exactly on the first apply.
//!
//! Every assertion is re-derived from the C++ oracle, not the Rust port.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_num::opcodes::OpCode;

use kuna_decomp::action::ActionContext;
use kuna_decomp::coreaction_early::{
    ActionDeterminedBranch, ActionRedundBranch, ActionStartCleanUp, ActionStartTypes,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::seams::{Architecture, BlockId, TypeOp};

// ---- fixtures (mirroring the in-module ones) ------------------------------

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
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    Address::new(ram, off)
}

// ===========================================================================
// ActionRedundBranch: single-out splice gating (realized, output-affecting)
// ===========================================================================

/// C++ guard `!bl->isEntryPoint()` (coreaction.cc:3664): a single-out chain
/// `bl -> out` must NOT splice when `out` is the function entry point — even
/// though `out` is single-in.  (Reverse the start-block assignment from the
/// in-module happy-path test: make `out` the entry instead of a separate head.)
#[test]
fn redundbranch_does_not_splice_into_entry_point() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    let out = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(bl, out);
    // Mark `out` (the splice target `bl` in C++ terms) as the entry point.
    fd.bblocks_mut().set_start_block(root, out);
    assert!(fd.bblocks_ref().block(out).is_entry_point());

    let size_before = fd.bblocks_get_size();
    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // The `!isEntryPoint()` guard suppresses the splice -> no change, no removal.
    assert_eq!(act.base().count, 0, "splice into an entry point is forbidden");
    assert_eq!(fd.bblocks_get_size(), size_before, "no block removed");
}

/// C++ guard `bl->sizeIn() == 1` (coreaction.cc:3664): when the successor has
/// two predecessors, the single-out arm must NOT splice.  Shape:
/// `head -> mid -> out`, plus `head2 -> out`, so `out` has sizeIn == 2.
#[test]
fn redundbranch_does_not_splice_when_successor_has_two_in_edges() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let head = fd.bblocks_mut().new_block_basic(root);
    let mid = fd.bblocks_mut().new_block_basic(root);
    let out = fd.bblocks_mut().new_block_basic(root);
    let head2 = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(head, mid);
    fd.bblocks_mut().add_edge(mid, out); // mid single-out -> out
    fd.bblocks_mut().add_edge(head2, out); // out now has sizeIn == 2
    fd.bblocks_mut().set_start_block(root, head);
    assert_eq!(fd.bblocks_ref().block(out).size_in(), 2);

    let size_before = fd.bblocks_get_size();
    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // `mid -> out` is single-out but out.sizeIn()==2 => no splice.
    // (`head -> mid` is also single-out, but mid.sizeIn()==1 and mid->out has a
    //  trailing-branch-free body; however head is the entry so its only out edge
    //  goes to mid whose sizeIn==1 -> that *could* splice.  Guard against a
    //  false positive by asserting the out-edge target of the merged block is
    //  preserved: the key invariant is that `out` is never absorbed.)
    // Re-derive: head->mid is spliceable (mid sizeIn 1, mid not entry, head not
    //  switch) so the count may be >=1, but `out` (sizeIn 2) must survive.
    let _ = size_before;
    // `out` must still exist with both its in-edges (never spliced away).
    // Find a block that still has two in-edges == the surviving `out`.
    let n = fd.bblocks_get_size();
    let mut found_two_in = false;
    for i in 0..n {
        let b = fd.bblocks_get_block(i);
        if fd.bblocks_ref().block(b).size_in() == 2 {
            found_two_in = true;
        }
    }
    assert!(
        found_two_in,
        "the two-in-edge successor must never be spliced away (sizeIn==1 guard)"
    );
}

/// C++ scan-reset `i = -1` (coreaction.cc:3669): a straight chain
/// `entry -> a -> b -> c` (each single-out, each successor single-in, none an
/// entry/switch) must collapse fully in a single `apply` because each splice
/// resets the scan to index 0.  Observe: at least two blocks removed and
/// `count` advanced by the same number of splices.
#[test]
fn redundbranch_scan_reset_collapses_full_chain_in_one_pass() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let entry = fd.bblocks_mut().new_block_basic(root);
    let a = fd.bblocks_mut().new_block_basic(root);
    let b = fd.bblocks_mut().new_block_basic(root);
    let c = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(a, b);
    fd.bblocks_mut().add_edge(b, c);
    fd.bblocks_mut().set_start_block(root, entry);

    let size_before = fd.bblocks_get_size(); // 4
    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    let removed = size_before - fd.bblocks_get_size();
    // Each successful splice removes exactly one block and bumps count by one;
    // the scan-reset guarantees a multi-link chain collapses in one apply call.
    assert!(removed >= 2, "scan-reset must collapse more than one link per call");
    assert_eq!(
        act.base().count, removed,
        "every removed block corresponds to exactly one counted splice"
    );
}

// ===========================================================================
// ActionDeterminedBranch: detection walk robustness (realized)
// ===========================================================================

fn block_with_cbranch_cond(fd: &mut Funcdata, bl: BlockId, off: u64, cond_const: Option<u64>) {
    let pc = ram_addr(fd, off);
    let op = fd.new_op(2, pc);
    if let Some(c) = cond_const {
        let cvn = fd.new_constant(1, c);
        fd.op_set_input(op, cvn, 1).unwrap();
    }
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
    fd.op_insert_end(op, bl);
}

/// C++ `if (!cbranch->getIn(1)->isConstant()) continue;` (coreaction.cc:3700):
/// a CBRANCH whose condition slot is *not* constant must be skipped — the walk
/// must run to completion with no change and no panic, even across a mix of
/// constant and non-constant CBRANCH blocks and an empty block.
#[test]
fn determinedbranch_skips_nonconstant_and_walks_mixed_graph() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");

    // Block 0: empty (lastOp == null) -> continue.
    let _empty = fd.bblocks_mut().new_block_basic(root);
    // Block 1: CBRANCH on a *non-constant* condition (a fresh non-const varnode).
    let nonconst = fd.bblocks_mut().new_block_basic(root);
    {
        let pc = ram_addr(&fd, 0x3000);
        let op = fd.new_op(2, pc);
        // slot-1 input is a plain (non-constant) varnode in ram.
        let vn_addr = ram_addr(&fd, 0x9000);
        let vn = fd.new_varnode(1, &vn_addr, None);
        fd.op_set_input(op, vn, 1).unwrap();
        fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
        fd.op_insert_end(op, nonconst);
    }
    // Block 2: CBRANCH on constant 0.
    let const0 = fd.bblocks_mut().new_block_basic(root);
    block_with_cbranch_cond(&mut fd, const0, 0x4000, Some(0));
    // Block 3: CBRANCH on constant 1.
    let const1 = fd.bblocks_mut().new_block_basic(root);
    block_with_cbranch_cond(&mut fd, const1, 0x5000, Some(1));

    let mut act = ActionDeterminedBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // removeBranch is the W3-block seam, so even the two constant blocks count 0;
    // the point is the non-constant block is *skipped* (no panic on getOffset of
    // a non-constant) and the whole walk completes.
    assert_eq!(act.base().count, 0);
    assert_eq!(fd.bblocks_get_size(), 4, "detection walk mutates nothing (seam)");
}

// ===========================================================================
// ActionStartTypes: once-only startTypeRecovery transition (realized)
// ===========================================================================

/// C++ `if (data.startTypeRecovery()) count+=1;` (coreaction.hh:83) over
/// `startTypeRecovery()` which returns true only on the first call
/// (funcdata.cc:180).  Three applies must bump `count` exactly once total, and
/// the type-recovery-started flag latches.
#[test]
fn starttypes_apply_counts_exactly_once_across_repeats() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionStartTypes::boxed("base");

    assert!(!fd.has_type_recovery_started());
    for _ in 0..3 {
        let res = act.apply(&mut fd, &mut ctx);
        assert_eq!(res, 0);
    }
    assert!(fd.has_type_recovery_started(), "flag latches after first apply");
    assert_eq!(
        act.base().count,
        1,
        "startTypeRecovery transitions once => count increments exactly once"
    );
}

// ===========================================================================
// ActionStartCleanUp: FINDING F1 — the realized startCleanUp side effect
// (the clean-up create-index snapshot) is dropped by the no-op apply.
// ===========================================================================

/// C++ `ActionStartCleanUp::apply` (coreaction.hh:65) is
/// `data.startCleanUp(); return 0;`, and `startCleanUp()` (funcdata.hh:194) is
/// `clean_up_index = vbank.getCreateIndex();` — a side effect that IS realized
/// in the merged tree as `Funcdata::start_clean_up` (funcdata.rs:429).  The
/// faithful body must call it, snapshotting the current create index into
/// `clean_up_index`.
///
/// The ported `apply` instead no-ops with a (factually wrong) comment claiming
/// `startCleanUp` "is not in the merged tree", so the snapshot never happens.
/// This test asserts the C++-faithful behavior and therefore **fails** against
/// the current port — it is the divergence trace for the REJECT.
#[test]
fn startcleanup_apply_snapshots_clean_up_index() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();

    // Advance the varnode create index by allocating two free varnodes.
    let a0 = ram_addr(&fd, 0x40);
    let a1 = ram_addr(&fd, 0x44);
    let _ = fd.new_varnode(4, &a0, None);
    let _ = fd.new_varnode(4, &a1, None);
    let create_index_now = fd.get_clean_up_index(); // still 0 before the action
    assert_eq!(create_index_now, 0, "clean_up_index starts at 0");

    let mut act = ActionStartCleanUp::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);

    // C++ would now have clean_up_index == the current create index (> 0).
    assert!(
        fd.get_clean_up_index() > 0,
        "ActionStartCleanUp::apply must call startCleanUp() to snapshot the \
         clean-up create-index (realized as start_clean_up); the no-op port \
         leaves it at 0 (FINDING F1)"
    );
}
