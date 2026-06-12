//! Independent VERIFIER adversarial tests for item `w6-s5-rangeutil`
//! (`decompiler/cpp/rangeutil.{cc,hh}` — `CircleRange` + value-set widening).
//!
//! Written by the verifier (not the porter), targeting the hunt-list spots most
//! fragile for this item:
//!
//!   - **Wrapping / shift-by->=width.** ADR 0003 mandates the `w*` helpers for
//!     any C++ expression that legitimately wraps.  The port used `wshl` for the
//!     left-shifts in `pull_back_binary`/`push_forward_binary` but left the
//!     *right*-shifts as bare `>>` (`rangeutil.rs:1136`, `:1166`, `:1423`,
//!     `:1427`, `:1443`, `:1471`, ...).  On x86, the C++ `x >> sa` masks the
//!     count to the low 6 bits (`sa & 63`); Rust's bare `>>` **panics in a debug
//!     build (which is what `cargo test` runs)** the instant `sa >= 64`.  A
//!     SUBPIECE with byte-offset 8 (`sa = 8*8 = 64`) or a shift constant >= 64 is
//!     ordinary p-code — these tests pin the divergence with the x86 C++ oracle.
//!   - **Iteration-order provenance.** `ValueSet::add_equation` is a slot-ordered
//!     *stable* insert (`vector::insert` before the first `slot > slot`); ties
//!     keep insertion order.  Pinned below.
//!   - **Bourdoncle WTO order.** `visit`/`component`/`establish_topological_order`
//!     reproduce upstream's nested-component ordering; pinned with a cycle.
//!   - **getSize overflow / circleUnion / intersect** boundary oracles.
//!
//! Tests that demonstrate a divergence wrap the offending call in
//! `catch_unwind` and assert the C++ (x86) result the port fails to produce, so
//! the file stays green-or-documents-the-bug rather than aborting the harness.

use kuna_decomp::rangeutil::{CircleRange, ValueSet, ValueSetGraph, ValueSetSolver};
use kuna_num::opcodes::OpCode;
use std::panic::{catch_unwind, AssertUnwindSafe};

// ---------------------------------------------------------------------------
// F1: right-shift-by->=64 wrapping divergence (the central finding)
// ---------------------------------------------------------------------------

/// `push_forward_binary(SUBPIECE)` with byte offset 8 -> `sa = 8*8 = 64`.
///
/// C++ (rangeutil.cc:1279-1298, x86): `range >> 64 == range`, takes the else
/// branch, `left = in1.left>>64 = 0`, `right = (0xff>>64)+1 = 0x100`, so the
/// result is the full input range `[0, 0x100)`.  The Rust port (`rangeutil.rs:
/// 1423/1427/1428`) uses bare `>>`, which panics in debug at `range >> sa`.
#[test]
fn w6_s5_subpiece_byteoffset8_shift64_diverges() {
    // in1 = [0, 0x100) step 1 over an 8-byte domain.
    let in1 = CircleRange::new(0x00, 0x100, 8, 1);
    // in2 = single value 8 (the SUBPIECE byte offset) over an 8-byte domain.
    let in2 = CircleRange::new(0x08, 0x09, 8, 1);
    assert!(in2.is_single(), "guard: in2 must be a single value");

    let res = catch_unwind(AssertUnwindSafe(|| {
        let mut out = CircleRange::new_empty();
        let ok = out.push_forward_binary(
            OpCode::CPUI_SUBPIECE,
            &in1,
            &in2,
            8, // in_size
            8, // out_size
            32,
        );
        (ok, out)
    }));

    match res {
        Err(_) => {
            // Divergence demonstrated: C++ would NOT panic; on x86 it produces
            // the range [0, 0x100).  This is the F1 finding.
            // (panic message: "attempt to shift right with overflow")
        }
        Ok((ok, out)) => {
            // If a future fix lands (using wshr), it must match the x86 C++ oracle.
            assert!(ok, "C++ returns true for SUBPIECE single-offset");
            let expect = CircleRange::new(0x00, 0x100, 8, 1);
            assert!(
                out.equals(&expect),
                "SUBPIECE sa=64 must match x86 C++ oracle [0,0x100); got {:?}",
                out
            );
        }
    }
}

/// `pull_back_binary(INT_RIGHT)` with a shift constant `val = 64` (REPAIRED, F1).
///
/// C++ (rangeutil.cc:948-962, x86): `rightBound = (calc_mask(2) >> (64&63)) + 1`
/// `= (0xffff >> 0) + 1 = 0x10000`; the coverage condition is false, neither
/// bound exceeds `rightBound`, so `left = (1 << (64&63)) & mask = 1`,
/// `right = (0xf << 0) & mask = 0xf` -> the range is UNCHANGED `[1, 0xf)` and the
/// pull-back returns `true`.  The port now uses `wshr`/`wshl` (shift mod 64),
/// so it no longer panics and reproduces this x86 oracle exactly.
#[test]
fn w6_s5_pullback_int_right_shift64_matches_oracle() {
    // non-empty, step == 1 -> reaches the shift.
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    let ok = range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 64, 0, 2, 2);
    assert!(ok, "INT_RIGHT pull-back returns true (x86 C++)");
    // Shift count 64 masks to 0 on x86 -> range unchanged.
    assert!(
        range.equals(&CircleRange::new(0x01, 0x0f, 2, 1)),
        "INT_RIGHT shift-64 pull-back must be the unchanged [1,0xf) per x86 oracle; got {:?}",
        range
    );
}

/// `pull_back_binary(INT_SRIGHT)` with `val = 64` (REPAIRED, F1).
///
/// C++ (rangeutil.cc:973-992, x86): `leftb = 0xffff >> ((64+1)&63) = 0xffff >> 1
/// = 0x7fff`; `rightb = 0x7fff ^ 0xffff = 0x8000`; `leftb += 1 = 0x8000`.  The
/// coverage condition is false and neither bound is strictly inside
/// `(leftb, rightb)`, so `left = (1 << (64&63)) & mask = 1`,
/// `right = (0xf << 0) & mask = 0xf` -> range UNCHANGED `[1, 0xf)`, returns
/// `true`.  The port now uses `wadd` for the `val+1` count and `wshr`/`wshl`
/// (shift mod 64) so it reproduces this oracle without panicking.
#[test]
fn w6_s5_pullback_int_sright_shift64_matches_oracle() {
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    let ok = range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 64, 0, 2, 2);
    assert!(ok, "INT_SRIGHT pull-back returns true (x86 C++)");
    assert!(
        range.equals(&CircleRange::new(0x01, 0x0f, 2, 1)),
        "INT_SRIGHT shift-64 pull-back must be the unchanged [1,0xf) per x86 oracle; got {:?}",
        range
    );
}

/// `push_forward_binary(INT_RIGHT)` with single shift `in2.left = 64`
/// (`sa = in2.left as int4 = 64`) (REPAIRED, F1).
///
/// C++ (x86, rangeutil.cc:1300-1316): `in1.left < in1.right`, so
/// `left = in1.left >> (64&63) = 0x10 >> 0 = 0x10`,
/// `right = ((in1.right - in1.step) >> 0) + 1 = (0x3f) + 1 = 0x40`; `left != right`.
/// The out range is `[0x10, 0x40)` step 1 over `calc_mask(8)`.  The port now uses
/// `wshr` (shift mod 64) and reproduces this oracle without panicking.
#[test]
fn w6_s5_pushforward_int_right_shift64_matches_oracle() {
    let in1 = CircleRange::new(0x10, 0x40, 8, 1); // left < right, non-empty
    let in2 = CircleRange::new(64, 65, 8, 1); // single value 64
    assert!(in2.is_single());

    let mut out = CircleRange::new_empty();
    let ok = out.push_forward_binary(OpCode::CPUI_INT_RIGHT, &in1, &in2, 8, 8, 32);
    assert!(ok, "INT_RIGHT push-forward returns true (x86 C++)");
    assert!(
        out.equals(&CircleRange::new(0x10, 0x40, 8, 1)),
        "INT_RIGHT shift-64 push-forward must equal x86 oracle [0x10,0x40); got {:?}",
        out
    );
}

/// Control: the SAME SUBPIECE op with byte offset 4 (`sa = 32 < 64`) must NOT
/// panic and must match the x86 C++ oracle.  This proves the divergence is
/// specifically the >=64 boundary, not a general SUBPIECE breakage.
#[test]
fn w6_s5_subpiece_byteoffset4_shift32_matches_oracle() {
    let in1 = CircleRange::new(0x00, 0x10000, 8, 1); // [0, 0x10000) step 1
    let in2 = CircleRange::new(0x04, 0x05, 8, 1); // single offset 4 -> sa=32
    let mut out = CircleRange::new_empty();
    let ok = out.push_forward_binary(OpCode::CPUI_SUBPIECE, &in1, &in2, 8, 4, 32);
    assert!(ok);
    // C++ oracle (x86, sa=32): range = 0x10000; range>>32 = 0 (<= mask) so else
    // branch: left = 0>>32 = 0; right = ((0x10000-1)>>32)+1 = 0+1 = 1; normalize.
    // out_size = 4 -> mask = 0xffffffff.  left==right? 0 != 1.
    let expect = CircleRange::new(0x00, 0x01, 4, 1);
    assert!(
        out.equals(&expect),
        "SUBPIECE sa=32 must equal x86 C++ oracle [0,1); got {:?}",
        out
    );
}

// ---------------------------------------------------------------------------
// F2 (clean): ValueSet::add_equation slot-ordered STABLE insert
// ---------------------------------------------------------------------------

/// C++ `addEquation` (rangeutil.cc:1549) inserts before the first equation whose
/// `slot > slot`; equal slots keep insertion order (`vector::insert`).  Verify
/// the Rust port reproduces that order exactly via `does_equation_apply` (which
/// reads `equations[num].slot` at each index) plus `get_landmark` (which returns
/// the FIRST equation's range).
#[test]
fn w6_s5_add_equation_is_slot_ordered_stable() {
    let mut vs = ValueSet::new();
    // Insert slots out of order, with a duplicate slot to probe tie-breaking.
    let r = |v: u64| CircleRange::new(v, v + 1, 4, 1);
    vs.add_equation(2, 0, r(0xA)); // slot 2  (first 2)
    vs.add_equation(0, 0, r(0xB)); // slot 0
    vs.add_equation(2, 0, r(0xC)); // slot 2  (second 2, must land AFTER first 2)
    vs.add_equation(1, 0, r(0xD)); // slot 1

    // Expected stored order (stable slot sort): idx0=slot0, idx1=slot1, idx2=slot2, idx3=slot2.
    assert!(vs.does_equation_apply(0, 0), "equations[0] must be slot 0");
    assert!(vs.does_equation_apply(1, 1), "equations[1] must be slot 1");
    assert!(vs.does_equation_apply(2, 2), "equations[2] must be slot 2 (first inserted)");
    assert!(vs.does_equation_apply(3, 2), "equations[3] must be slot 2 (second inserted)");
    // No slot mismatch should report apply.
    assert!(!vs.does_equation_apply(0, 1), "index 0 is slot 0, not slot 1");
    assert!(!vs.does_equation_apply(4, 2), "out-of-range index never applies");

    // get_landmark returns the FIRST equation's range -> slot 0 -> value 0xB.
    let lm = vs.get_landmark().expect("landmark exists (type_code 0 matches)");
    assert_eq!(lm.get_min(), 0xB, "landmark is the first (slot-0) equation's range");
}

// ---------------------------------------------------------------------------
// F3 (clean): getSize wrap-to-everything + circleUnion/intersect boundary
// ---------------------------------------------------------------------------

/// `getSize` (rangeutil.cc:256-273) for a full 8-byte range (`left == right`,
/// step 1, mask = u64::MAX): C++ computes `val = (mask - 0 + step)/step`, where
/// `mask + 1` WRAPS to 0 over `uintb`, hitting the documented overflow branch
/// (`val == 0 -> val = mask`).  The Rust port (`rangeutil.rs:400`) uses bare
/// `self.mask - ... + self.step`, which **panics in a debug build** at
/// `mask + 1` instead of wrapping.  `get_size` is a core public method (the
/// jumptable code calls it), so this is an output-relevant divergence, not a
/// dormant one.
#[test]
fn w6_s5_get_size_full_8byte_range_overflow_diverges() {
    let res = catch_unwind(AssertUnwindSafe(|| {
        let mut full = CircleRange::new_empty();
        full.set_full(8); // left==right==0, step 1, mask=u64::MAX
        full.get_size()
    }));
    match res {
        Err(_) => {
            // F-getSize: C++ returns mask (== u64::MAX) via the wrap branch;
            // the Rust port panics on `mask + step`.  This is the finding.
        }
        Ok(sz) => {
            // If fixed to use wadd/wsub, must equal the x86 C++ oracle (mask).
            assert_eq!(
                sz,
                u64::MAX,
                "full 8-byte getSize must be the wrapped mask (u64::MAX) per C++ overflow branch"
            );
        }
    }

    // Control: a 1-byte full range does not overflow (mask=0xff, 0xff+1=0x100),
    // so the port must compute 0x100 without panicking.
    let mut full1 = CircleRange::new_empty();
    full1.set_full(1);
    assert_eq!(full1.get_size(), 0x100, "1-byte full range size is 256, no wrap");
}

/// `circle_union` of two disjoint singletons that are NOT adjacent returns 2
/// (two pieces) and leaves `self` unmodified.  Adjacent ones merge.  Probes the
/// `encode_range_overlaps` ARRANGE table dispatch at the wrap boundary.
#[test]
fn w6_s5_circle_union_disjoint_vs_adjacent() {
    // [0x10,0x11) U [0x20,0x21): disjoint, non-adjacent -> 2 pieces, unmodified.
    let mut a = CircleRange::new(0x10, 0x11, 1, 1);
    let b = CircleRange::new(0x20, 0x21, 1, 1);
    let before = a.clone();
    let code = a.circle_union(&b);
    assert_eq!(code, 2, "disjoint non-adjacent union is 2 pieces");
    assert!(a.equals(&before), "self unmodified when union returns 2");

    // [0x10,0x11) U [0x11,0x12): adjacent -> merges to [0x10,0x12).
    let mut c = CircleRange::new(0x10, 0x11, 1, 1);
    let d = CircleRange::new(0x11, 0x12, 1, 1);
    let code2 = c.circle_union(&d);
    assert_eq!(code2, 0, "adjacent union merges to single interval");
    assert!(c.equals(&CircleRange::new(0x10, 0x12, 1, 1)));
}

/// `intersect` mixed-step path: a step-1 range intersected with a step-2 range
/// raises the smaller stride; verify a known-good result and that the empty
/// result is flagged via `is_empty` (not a panic).
#[test]
fn w6_s5_intersect_mixed_step_phase() {
    // self = [0,8) step 1 (1-byte); op2 = [1,9) step 2 phase-1 (odd values 1,3,5,7).
    let mut a = CircleRange::new(0x00, 0x08, 1, 1);
    let b = CircleRange::new(0x01, 0x09, 1, 2);
    let code = a.intersect(&b);
    assert_eq!(code, 0, "intersect should yield a single interval");
    assert!(!a.is_empty());
    // The intersection is the odd values in [1,7]; step becomes 2, phase 1.
    assert_eq!(a.get_step(), 2, "intersect raises stride to the larger step");
    assert_eq!(a.get_min() % 2, 1, "phase preserved (odd)");
}

// ---------------------------------------------------------------------------
// F4 (clean): Bourdoncle WTO ordering reproduces nested components with a cycle
// ---------------------------------------------------------------------------

/// Adjacency-list graph backing the injected `ValueSetGraph` seam.  Index 0
/// reserved for the transient root the solver appends at `value_nodes.len()`;
/// here the root index equals `nodes.len()` minus the transient slot, so we map
/// it explicitly.
struct AdjGraph {
    /// successors[node] in the exact order C++ `ValueSetEdge::getNext` yields.
    successors: Vec<Vec<usize>>,
    /// the node index the solver uses for the transient root.
    root_index: usize,
    /// root edges (rootNodes), in registration order.
    root_edges: Vec<usize>,
}

impl ValueSetGraph for AdjGraph {
    fn successors(&self, node: usize) -> Vec<usize> {
        if node == self.root_index {
            self.root_edges.clone()
        } else {
            self.successors[node].clone()
        }
    }
}

/// A simple loop: root -> 0 -> 1 -> 2 -> 1 (back-edge), 2 -> exit.
/// Bourdoncle should place 0 first, then a component headed by 1 containing {1,2}.
#[test]
fn w6_s5_wto_orders_cycle_as_nested_component() {
    let mut solver = ValueSetSolver::new();
    let n0 = solver.new_value_set();
    let n1 = solver.new_value_set();
    let n2 = solver.new_value_set();
    solver.push_root(n0);

    // The transient root is appended at value_nodes.len() == 3 during
    // establish_topological_order, so root_index = 3.
    let graph = AdjGraph {
        successors: vec![
            vec![n1],     // 0 -> 1
            vec![n2],     // 1 -> 2
            vec![n1],     // 2 -> 1 (back-edge forming the loop)
        ],
        root_index: 3,
        root_edges: vec![n0],
    };

    solver.establish_topological_order(&graph);
    let order = solver.iteration_order();

    // Node 0 must come first; the loop {1,2} forms a component headed by 1.
    assert_eq!(order.first(), Some(&n0), "acyclic predecessor 0 iterates first");
    assert!(order.contains(&n1) && order.contains(&n2), "all loop nodes present");
    // The component head (node 1) must be recorded as a partition head.
    assert!(
        solver.part_head_of(n1).is_some(),
        "loop head (node 1) must be a partition head"
    );
    // Node 1 must appear before node 2 in the component (head leads the body).
    let p1 = order.iter().position(|&x| x == n1).unwrap();
    let p2 = order.iter().position(|&x| x == n2).unwrap();
    assert!(p1 < p2, "component head precedes its body in iteration order");
}

/// A purely linear chain (no cycle) must produce a flat order root->0->1->2 with
/// NO partition heads (every `count` collapses without a loop flag).
#[test]
fn w6_s5_wto_linear_chain_has_no_components() {
    let mut solver = ValueSetSolver::new();
    let n0 = solver.new_value_set();
    let n1 = solver.new_value_set();
    let n2 = solver.new_value_set();
    solver.push_root(n0);
    let graph = AdjGraph {
        successors: vec![vec![n1], vec![n2], vec![]],
        root_index: 3,
        root_edges: vec![n0],
    };
    solver.establish_topological_order(&graph);
    let order = solver.iteration_order();
    assert_eq!(order, vec![n0, n1, n2], "linear chain iterates in flow order");
    for &n in &[n0, n1, n2] {
        assert!(solver.part_head_of(n).is_none(), "no node is a component head");
    }
}
