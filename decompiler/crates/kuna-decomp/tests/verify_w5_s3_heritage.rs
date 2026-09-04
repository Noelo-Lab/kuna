//! Independent verifier adversarial tests for item `w5-s3-heritage`
//! (`decompiler/cpp/heritage.{cc,hh}`).
//!
//! These target the hunt-list spots flagged as most fragile for the realized
//! algorithmic core (the data-flow *mutation* surface is a documented seam and
//! is exercised by the in-module `*_is_a_seam` panics):
//!
//!   - `LocationMap::add` (heritage.cc:34) — the disjoint-cover union: the
//!     `++iter` skip when the leading candidate does not overlap (heritage.cc
//!     :40-41), the multi-element "swallow" erase loop (heritage.cc:58-66),
//!     and the **pass-min carry** + intersect-code (0/1/2) protocol.  These are
//!     the erase-while-iterating equivalent re-derived against the BTree.
//!   - `buildADT` + `calcMultiequals`/`visitIncr` (heritage.cc:2317/2440/2395)
//!     — the augmented-dominator-tree phi placement: a 3-way join (not just the
//!     diamond the porter tested), and a node reachable by an up-edge whose
//!     `z[]` boundary-ancestor chain is walked (the `while(j<k){k=z[k]}` loop,
//!     heritage.cc:2381).  The iterated dominance frontier is output-affecting.
//!   - `PriorityQueue` (heritage.cc:142) — LIFO-within-depth + the `curdepth`
//!     drain sentinel, and the `reset` idempotence early-return.
//!   - `bumpDeadcodeDelay` (heritage.cc:2572) — the SPACEBASE branch of the
//!     space-type gate and the "install once / suppress on revisit" protocol.
//!
//! Each assertion is re-derived from the C++ oracle, not from the Rust port.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::heritage::{Heritage, LocationMap, PriorityQueue, TaskList};
use kuna_decomp::overrides::Override;
use kuna_decomp::kuna_restartlog::RestartLog;
use kuna_decomp::context::{ArchContext, BlockId};
use kuna_decomp::funcdata::Funcdata;

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
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn raddr(fd: &Funcdata, off: u64) -> Address {
    Address::new(ram(fd), off)
}

fn build_cfg(fd: &mut Funcdata, n: usize, edges: &[(usize, usize)]) -> Vec<BlockId> {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let g = fd.bblocks_mut();
    let mut blocks = Vec::with_capacity(n);
    for _ in 0..n {
        blocks.push(g.new_block_basic(root));
    }
    for (i, &id) in blocks.iter().enumerate() {
        g.block_mut(id).set_index(i as int4);
    }
    for &(a, b) in edges {
        g.add_edge(blocks[a], blocks[b]);
    }
    g.calc_forward_dominator(root, &[blocks[0]]);
    blocks
}

fn merge_indices(h: &Heritage, fd: &Funcdata) -> Vec<int4> {
    let mut v: Vec<int4> =
        h.merge_points().iter().map(|&b| fd.bblocks_ref().block(b).get_index()).collect();
    v.sort_unstable();
    v
}

// ===========================================================================
// 1. LocationMap::add — the ++iter skip path, multi-swallow, pass-min carry
// ===========================================================================

/// The riskiest path in `add`: the leading candidate (after lower_bound-then-
/// step-back) does NOT overlap, so the C++ does `++iter` (heritage.cc:40-41)
/// before the first-block branch.  Build a map with a low element that ends
/// strictly before the new range, then add a range that overlaps ONLY a later
/// element — the leading low element must be skipped, not erased, and the later
/// element must be unioned in with its (older) pass carried.
#[test]
fn location_map_add_skips_nonoverlapping_leading_then_unions_later() {
    let fd = build_fd();
    let mut lm = LocationMap::new();
    // pass 5: [0x100, 0x104) — a low element that will be the lower_bound-1
    //         candidate but does NOT overlap the new range.
    lm.add(raddr(&fd, 0x100), 4, 5);
    // pass 3: [0x200, 0x208) — an older (smaller-pass) element further along.
    lm.add(raddr(&fd, 0x200), 8, 3);

    // pass 7: [0x202, 0x206) — addr=0x202.  lower_bound(0x202) == 0x200's
    //   successor end... in C++: lower_bound finds 0x200 is < 0x202 so the
    //   first key >= 0x202 is end; --iter -> 0x200, which DOES overlap 0x202,
    //   so NO ++iter, first-block branch taken, contained -> intersect 2.
    let (key, intersect, _sz) = lm.add(raddr(&fd, 0x202), 4, 7);
    assert_eq!(key, raddr(&fd, 0x200), "contained sub-range returns containing element");
    assert_eq!(intersect, 2, "fully contained in older range -> intersect 2");
    assert_eq!(lm.get(&key).unwrap().pass, 3, "older pass preserved");
    // The low [0x100,0x104) element must be untouched.
    assert_eq!(lm.get(&raddr(&fd, 0x100)).unwrap().pass, 5);
    assert_eq!(lm.iter().count(), 2);
}

/// Force the genuine `++iter` skip: add a range whose start is BETWEEN two
/// elements, overlapping neither the lower candidate nor reaching the higher
/// one — it lands as a fresh disjoint element (intersect stays 0), and the
/// non-overlapping leading element is skipped, not consumed.
#[test]
fn location_map_add_between_elements_is_fresh_disjoint() {
    let fd = build_fd();
    let mut lm = LocationMap::new();
    lm.add(raddr(&fd, 0x100), 4, 0); // [0x100,0x104)
    lm.add(raddr(&fd, 0x300), 4, 0); // [0x300,0x304)
    // [0x200,0x204) overlaps neither: lower_bound(0x200) -> 0x300; --iter ->
    // 0x100; 0x100 does NOT overlap 0x200 -> ++iter -> 0x300; 0x300 does not
    // overlap 0x200 either (where == -1) -> while loop body never runs;
    // inserted fresh.  intersect must stay 0.
    let (key, intersect, _sz) = lm.add(raddr(&fd, 0x200), 4, 9);
    assert_eq!(key, raddr(&fd, 0x200));
    assert_eq!(intersect, 0, "disjoint insert -> intersect 0");
    assert_eq!(lm.get(&key).unwrap().pass, 9, "fresh element keeps its own pass");
    assert_eq!(lm.iter().count(), 3);
}

/// The multi-element swallow loop (heritage.cc:58-66) with DIFFERENT passes on
/// the swallowed elements: the unified element must carry the MINIMUM pass of
/// any intersecting old element, and intersect must be 1 (partial overlap with
/// something old).  Also checks the size grows to cover the farthest end.
#[test]
fn location_map_add_swallow_carries_global_min_pass() {
    let fd = build_fd();
    let mut lm = LocationMap::new();
    lm.add(raddr(&fd, 0x100), 4, 6); // [0x100,0x104) pass 6
    lm.add(raddr(&fd, 0x108), 4, 2); // [0x108,0x10c) pass 2  (the minimum)
    lm.add(raddr(&fd, 0x110), 4, 4); // [0x110,0x114) pass 4
    // pass 9: [0x100, 0x118) spans all three.  The leading 0x100 overlaps
    // (first-block branch): addr<-0x100, size grows, pass 6<9 -> intersect 1,
    // pass<-6, erase.  Then the while loop swallows 0x108 (pass 2 -> pass<-2)
    // and 0x110 (pass 4, not < 2, no change).  Final element pass == 2.
    let (key, intersect, _sz) = lm.add(raddr(&fd, 0x100), 0x18, 9);
    assert_eq!(key, raddr(&fd, 0x100));
    assert_eq!(intersect, 1, "partial overlap with old -> intersect 1");
    assert_eq!(lm.get(&key).unwrap().pass, 2, "global minimum pass carried");
    assert_eq!(lm.iter().count(), 1, "all three swallowed into one");
    // size must cover up to 0x118 -> at least 0x18.
    assert!(lm.get(&key).unwrap().size >= 0x18);
}

/// `findPass` boundary: an address exactly at the end of a range
/// (`addr == start+size`) is NOT inside it (overlap returns -1), so findPass
/// must be -1 there, while the last in-range byte returns the pass.
#[test]
fn location_map_find_pass_end_boundary_exclusive() {
    let fd = build_fd();
    let mut lm = LocationMap::new();
    lm.add(raddr(&fd, 0x200), 4, 3); // covers 0x200..0x203 inclusive
    assert_eq!(lm.find_pass(&raddr(&fd, 0x203)), 3, "last in-range byte");
    assert_eq!(lm.find_pass(&raddr(&fd, 0x204)), -1, "one past the end is unheritaged");
    assert_eq!(lm.find_pass(&raddr(&fd, 0x1ff)), -1, "one before the start is unheritaged");
}

// ===========================================================================
// 2. TaskList — overlap extend on the LAST element only
// ===========================================================================

/// `TaskList::add` only extends the LAST element; a range overlapping an
/// *earlier* (non-last) element must still append (the C++ only inspects
/// `tasklist.back()`).  This is the off-by-one between TaskList (last-only) and
/// LocationMap (full cover) — a faithful-port trap.
#[test]
fn tasklist_add_only_extends_last_not_earlier() {
    let fd = build_fd();
    let mut tl = TaskList::new();
    tl.add(raddr(&fd, 0x100), 4, 1); // element 0 [0x100,0x104)
    tl.add(raddr(&fd, 0x200), 4, 1); // element 1 [0x200,0x204) (the back)
    // [0x101,0x105) overlaps element 0 but NOT the back; must append fresh.
    tl.add(raddr(&fd, 0x101), 4, 2);
    assert_eq!(tl.len(), 3, "overlap with non-last element does not merge");
    assert_eq!(tl.get(2).addr, raddr(&fd, 0x101));
}

// ===========================================================================
// 3. PriorityQueue — LIFO-within-depth and the drain sentinel
// ===========================================================================

/// The extract drain must walk `curdepth` down past EMPTY intermediate stacks
/// (heritage.cc:171-174): insert at depths 0 and 3 (leaving 1,2 empty), and
/// confirm extract returns depth-3 first, then depth-0, then empties.
#[test]
fn priority_queue_drains_across_empty_intermediate_depths() {
    let mut fd = build_fd();
    let b = build_cfg(&mut fd, 2, &[(0, 1)]);
    let mut pq = PriorityQueue::new();
    pq.reset(3);
    pq.insert(b[0], 0);
    pq.insert(b[1], 3);
    assert!(!pq.empty());
    assert_eq!(pq.extract(), b[1], "highest depth first");
    // curdepth must have drained from 3 down past empty 2,1 to 0.
    assert_eq!(pq.extract(), b[0], "drains across empty intermediate stacks");
    assert!(pq.empty(), "curdepth reached -1 sentinel");
}

// ===========================================================================
// 4. Phi placement — 3-way join and an up-edge z[] chain walk
// ===========================================================================

/// A 3-way join: 0 -> {1,2,3} -> 4.  idom(4) == 0, so a write in ANY one of
/// 1/2/3 needs exactly ONE MULTIEQUAL at block 4 (and the start-block seeding
/// must not spuriously add others).  This stresses `visitIncr`'s `merged_node`
/// dedup across three augmented edges into the same join.
#[test]
fn phi_placement_three_way_join_single_phi() {
    let mut fd = build_fd();
    let b = build_cfg(&mut fd, 5, &[(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]);
    assert_eq!(fd.bblocks_ref().block(b[4]).size_in(), 3, "join has 3 preds");
    assert_eq!(fd.bblocks_ref().block(b[4]).get_immed_dom(), Some(b[0]));

    let mut h = Heritage::new();
    h.build_info_list(&fd);
    h.build_adt(&fd);
    h.calc_multiequals(&fd, &[b[2]]);
    assert_eq!(merge_indices(&h, &fd), vec![4], "3-way join: exactly one phi at the join");
}

/// A write in TWO blocks of the 3-way join still yields exactly one phi (the
/// merged_node flag must prevent a duplicate push for block 4).
#[test]
fn phi_placement_three_way_join_two_writes_still_single_phi() {
    let mut fd = build_fd();
    let b = build_cfg(&mut fd, 5, &[(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]);
    let mut h = Heritage::new();
    h.build_info_list(&fd);
    h.build_adt(&fd);
    h.calc_multiequals(&fd, &[b[1], b[3]]);
    assert_eq!(merge_indices(&h, &fd), vec![4], "two writes, one join -> still one phi");
}

/// Stacked diamonds, write in the FIRST diamond only.  Two diamonds:
///   0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3      (join A at 3, idom 0)
///   3 -> 4, 3 -> 5, 4 -> 6, 5 -> 6      (join B at 6, idom 3)
/// A write in block 1 places a phi at join 3 (DF(1)={3}).  It does NOT force a
/// phi at 6: block 3 *strictly* dominates 6 (idom(6)=3), so 6 is not in DF(3) —
/// both predecessors of 6 carry the single value produced at the phi in 3.  The
/// iterated dominance frontier is therefore {3} only.  (This is the trap: a
/// naive "phi at 3 -> rerun DF -> phi at 6" would over-place; the C++
/// visitIncr/buildADT correctly does NOT, because DF(3) excludes the strictly
/// dominated 6.)
#[test]
fn phi_placement_stacked_diamonds_first_write_no_phi_at_dominated_join() {
    let mut fd = build_fd();
    let b = build_cfg(
        &mut fd,
        7,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 6), (5, 6)],
    );
    assert_eq!(fd.bblocks_ref().block(b[3]).get_immed_dom(), Some(b[0]));
    assert_eq!(fd.bblocks_ref().block(b[6]).get_immed_dom(), Some(b[3]));

    let mut h = Heritage::new();
    h.build_info_list(&fd);
    h.build_adt(&fd);
    h.calc_multiequals(&fd, &[b[1]]);
    assert_eq!(
        merge_indices(&h, &fd),
        vec![3],
        "DF(write in first diamond) is just block 3; 6 is strictly dominated by 3 -> no phi"
    );
}

/// Stacked diamonds, write in block 4 (between the diamonds, inside the second
/// diamond's left arm).  Now block 6 IS a real join for that value: it arrives
/// via 4 but not via 5, so DF(4)={6}.  Block 3 sees no write, so no phi at 3.
/// This is the genuine "phi at the second join only" case, complementing the
/// test above — together they pin that the placement is driven by where the
/// write dominance-frontier actually lands, not by chain position.
#[test]
fn phi_placement_stacked_diamonds_mid_write_phi_at_second_join_only() {
    let mut fd = build_fd();
    let b = build_cfg(
        &mut fd,
        7,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 6), (5, 6)],
    );
    let mut h = Heritage::new();
    h.build_info_list(&fd);
    h.build_adt(&fd);
    h.calc_multiequals(&fd, &[b[4]]);
    assert_eq!(
        merge_indices(&h, &fd),
        vec![6],
        "write in one arm of the second diamond -> phi at its join (6) only"
    );
}

// ===========================================================================
// 5. bumpDeadcodeDelay — SPACEBASE gate + install-once / suppress protocol
// ===========================================================================

/// A SPACEBASE space (the other allowed branch of the space-type gate at
/// heritage.cc:2575) with delay == deadcodedelay must take the install path on
/// the first call and the suppress path (no re-bump, no restart) on the second.
#[test]
fn bump_deadcode_delay_spacebase_installs_once_then_suppresses() {
    // Build a manager that includes a SPACEBASE "stack" space.
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
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    let mut fd = Funcdata::new("f", "f", glb, addr, 0x10000000, 0x40).unwrap();

    let stack = Rc::clone(fd.get_arch().manage().get_space_by_name("stack").unwrap());
    // Precondition for the bump to do work.
    if stack.get_type() == spacetype::IPTR_SPACEBASE
        && stack.get_delay() == stack.get_deadcode_delay()
    {
        let mut h = Heritage::new();
        let mut ovr = Override::new();
        let mut log = RestartLog::new();

        assert!(!ovr.has_deadcode_delay(&stack));
        h.bump_deadcode_delay(&mut fd, &mut ovr, &mut log, &stack);
        assert!(ovr.has_deadcode_delay(&stack), "SPACEBASE: delay installed");
        assert!(fd.has_restart_pending(), "SPACEBASE: restart pending set");
        assert!(!log.is_empty_for(&fd), "SPACEBASE: bump recorded");

        // Second call: already installed -> suppressed, no new restart.
        fd.set_restart_pending(false);
        h.bump_deadcode_delay(&mut fd, &mut ovr, &mut log, &stack);
        assert!(
            !fd.has_restart_pending(),
            "SPACEBASE: suppressed path must NOT re-arm restart"
        );
    }
}
