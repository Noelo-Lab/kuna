//! Round-2 independent verifier adversarial tests for item
//! `w5-s3-coreaction-early` (`decompiler/cpp/coreaction.{cc,hh}`).
//!
//! Round 1 REJECTed on F1: `ActionStartCleanUp::apply` no-op'd a *realized* side
//! effect (`Funcdata::start_clean_up`, which snapshots the clean-up create
//! index).  The repair commit wires `data.start_clean_up()` into the body.  This
//! round re-derives the F1 fix directly from the C++ oracle and stresses the two
//! realized control-flow rules that round-1's tests touched only on their happy
//! path:
//!
//!   - `ActionStartCleanUp::apply` (coreaction.hh:65 -> funcdata.hh:194):
//!     `clean_up_index = vbank.getCreateIndex()`.  The snapshot must equal the
//!     *current* create index at call time (after any varnodes already created),
//!     and must update on a *second* apply once more varnodes exist.
//!   - `ActionRedundBranch::apply` n-way "all exits to same block" detection
//!     (coreaction.cc:3673-3675): when a multi-out block's exits do NOT all go to
//!     `getOut(0)`, the `j != sizeOut` early-out must fire and NO branch is
//!     removed and NO splice happens (the single-out arm is not entered).
//!   - `ActionRedundBranch::apply` single-out arm is also not entered for a
//!     multi-out block even when every exit IS the same block (the duplicate
//!     n-way edge case): that path only reaches the *seamed* `removeBranch`, so
//!     count stays 0 (no spurious splice, no panic).
//!
//! Every assertion is re-derived from the C++ source, not the Rust port.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_decomp::action::ActionContext;
use kuna_decomp::coreaction_early::{ActionRedundBranch, ActionStartCleanUp};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::ArchContext;

// ---- fixtures (mirror the in-module / round-1 ones) -----------------------

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

// ===========================================================================
// F1 fix: ActionStartCleanUp::apply snapshots the *current* create index.
// C++: data.startCleanUp();  // clean_up_index = vbank.getCreateIndex()
// ===========================================================================

/// The clean-up index snapshot must equal the create index that is *current*
/// at the moment `apply` runs — i.e. after varnodes have already been created
/// the snapshot is non-trivial, and a *second* apply after more creation moves
/// it forward.  A no-op `apply` (the round-1 bug) would leave the index pinned
/// at its constructor default (0) forever.
#[test]
fn r2_startcleanup_snapshots_live_create_index() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionStartCleanUp::boxed("base");

    // Create a couple of varnodes so the create index has advanced past 0.
    let _c0 = fd.new_constant(4, 0);
    let _c1 = fd.new_constant(4, 1);
    let live = fd.vbank().get_create_index();
    assert!(
        live > 0,
        "precondition: creating varnodes advances the create index past the \
         constructor default"
    );

    // First apply: snapshot must capture the *live* index, not the default 0.
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    let snap1 = fd.get_clean_up_index();
    assert_eq!(
        snap1, live,
        "startCleanUp must snapshot vbank.getCreateIndex() at call time, not 0"
    );

    // Create more varnodes, then re-apply: the snapshot must move forward
    // (proving the body re-reads the live index every call, like C++).
    let _c2 = fd.new_constant(4, 2);
    let _c3 = fd.new_constant(4, 3);
    let live2 = fd.vbank().get_create_index();
    assert!(live2 > live, "more creation advances the index further");

    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    let snap2 = fd.get_clean_up_index();
    assert_eq!(snap2, live2, "second startCleanUp re-snapshots the new live index");
    assert!(snap2 > snap1, "the re-snapshot strictly moved forward");
}

// ===========================================================================
// ActionRedundBranch: n-way detection early-out (coreaction.cc:3673-3675).
// for(j=1;j<sizeOut;++j) if (getOut(j) != bl) break;  if (j != sizeOut) continue;
// ===========================================================================

/// A block with two *distinct* successors must NOT have a branch removed: the
/// `j != sizeOut` early-out fires because `getOut(1) != getOut(0)`.  Removal is
/// seamed anyway, but the realized detection must *reject* this shape, and the
/// single-out splice arm must not be entered (no splice, no count, no panic, no
/// block removed).
#[test]
fn r2_redundbranch_multi_out_distinct_targets_not_removed() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let head = fd.bblocks_mut().new_block_basic(root);
    let a = fd.bblocks_mut().new_block_basic(root);
    let b = fd.bblocks_mut().new_block_basic(root);
    // head has two distinct out-edges: head->a and head->b.
    fd.bblocks_mut().add_edge(head, a);
    fd.bblocks_mut().add_edge(head, b);

    assert_eq!(fd.bblocks_ref().block(head).size_out(), 2);
    let size_before = fd.bblocks_get_size();

    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // getOut(1) != getOut(0) -> j stops at 1 != sizeOut(2) -> `continue`.
    // No removeBranch (seamed) AND no splice -> count stays 0, nothing removed.
    assert_eq!(act.base().count, 0, "distinct multi-out targets are not redundant");
    assert_eq!(fd.bblocks_get_size(), size_before, "no block removed");
}

/// A block whose multi-out edges *all* lead to the same successor (a duplicate
/// n-way branch) passes the realized detection (`j == sizeOut`); the
/// `removeBranch` seam is now closed, so one duplicate edge is severed and the
/// branch is reported as a change (`count == 1`).  The single-out splice arm is
/// never entered (sizeOut==2), and no whole block is removed — only one of the
/// two parallel edges is dropped (sizeOut(head) drops 2 -> 1).
#[test]
fn r2_redundbranch_all_exits_same_block_removes_one_edge() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let head = fd.bblocks_mut().new_block_basic(root);
    let tgt = fd.bblocks_mut().new_block_basic(root);
    // Two parallel edges head=>tgt: sizeOut(head)==2, both getOut(j)==tgt.
    fd.bblocks_mut().add_edge(head, tgt);
    fd.bblocks_mut().add_edge(head, tgt);

    assert_eq!(fd.bblocks_ref().block(head).size_out(), 2);
    assert_eq!(fd.bblocks_ref().block(head).get_out(0), tgt);
    assert_eq!(fd.bblocks_ref().block(head).get_out(1), tgt);
    let size_before = fd.bblocks_get_size();

    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // sizeOut==2 so the single-out splice arm is never entered; the all-same
    // detection now reaches the realized removeBranch -> one edge severed.
    assert_eq!(
        act.base().count,
        1,
        "duplicate n-way edge removal now realized (removeBranch seam closed)"
    );
    assert_eq!(
        fd.bblocks_ref().block(head).size_out(),
        1,
        "one of the two parallel edges is severed"
    );
    assert_eq!(fd.bblocks_get_size(), size_before, "no whole block removed, only an edge");
}

/// Sanity: an empty graph (no blocks) makes `ActionRedundBranch::apply` a clean
/// no-op — the `while i < size` loop never runs (boundary: getSize()==0).
#[test]
fn r2_redundbranch_empty_graph_is_noop() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    assert_eq!(fd.bblocks_get_size(), 0);

    let mut act = ActionRedundBranch::boxed("base");
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    assert_eq!(act.base().count, 0);
    assert_eq!(fd.bblocks_get_size(), 0);
}
