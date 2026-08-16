//! Round-3 independent VERIFIER adversarial tests for `w6-s5-rangeutil`.
//!
//! These are *fresh* tests authored by the verifier (not the porter), distinct
//! from `verify_w6_s5_rangeutil.rs`.  Round 2 found F3 — a bare `val_right + 1`
//! over `intb` in `push_forward_binary(CPUI_INT_SRIGHT)` that overflows /
//! debug-panics when `out_size == 8`.  Round 3's repair changed it to
//! `val_right.wadd(1)` (rangeutil.rs:1488).  This file re-derives the affected
//! and adjacent paths with hand-computed x86 oracles and pins them:
//!
//!   - F3 fix: the SRIGHT canonical-rebuild branch at out_size==8 (the panic
//!     boundary) now produces the wrapped two's-complement range, NOT a panic.
//!   - The SRIGHT path at a *smaller* output width (out_size==2 -> bitPos 15),
//!     where the `valRight+1` does NOT itself overflow but the sign_extend and
//!     arithmetic-shift bookkeeping still must match.
//!   - The SRIGHT *non-wrapping* common path (valLeft < valRight) must skip the
//!     rebuild entirely — proving the fix did not perturb the hot path.
//!   - `get_size` wrapping overflow branch with step>1 at a step OTHER than the
//!     porter's (step 4), plus a precise non-wrapping count.
//!   - Bourdoncle WTO determinism across two independent sibling loops.
//!
//! No `catch_unwind` here: after the F3 repair these calls must return cleanly,
//! so a panic = a hard test failure (the desired regression signal).

use kuna_decomp::rangeutil::{CircleRange, ValueSetGraph, ValueSetSolver};
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// F3 fix — INT_SRIGHT push-forward at the out_size==8 panic boundary.
// ---------------------------------------------------------------------------

/// Re-derivation of the F3 site (rangeutil.cc:1319-1338) for out_size==8, with
/// a hand-computed x86 oracle.
///
/// in1 = [0x10, 0x05) over an 8-byte domain (left > right -> the sign-extended
/// range wraps, so valLeft(0x10) >= valRight(0x05) and the canonical rebuild
/// fires).  in2 = single shift amount 1.  bitPos = 8*8 - 1 = 63, mask = u64::MAX.
///
///   valRight = (intb)(mask >> 1)         = i64::MAX = 0x7FFF_FFFF_FFFF_FFFF
///   valLeft  = valRight + 1 (TWO's-comp) = i64::MIN = 0x8000_0000_0000_0000
///   valLeft  = sign_extend(valLeft, 63)  = i64::MIN (bit 63 already set)
///   left  = (i64::MIN >> 1) & mask                   = 0xC000_0000_0000_0000
///   right = (((i64::MAX - 1) >> 1) + 1) & mask        = 0x4000_0000_0000_0000
///   left != right -> no fix-up.
///
/// Before the fix, `val_right + 1` panicked ("attempt to add with overflow").
#[test]
fn r3_sright_out8_canonical_rebuild_matches_x86_oracle() {
    let in1 = CircleRange::new(0x10, 0x05, 8, 1);
    let in2 = CircleRange::new(1, 2, 8, 1);
    assert!(in2.is_single());

    let mut out = CircleRange::new_empty();
    // Must NOT panic now (F3 repaired).
    let ok = out.push_forward_binary(OpCode::CPUI_INT_SRIGHT, &in1, &in2, 8, 8, 32);
    assert!(ok, "INT_SRIGHT push-forward returns true (x86 C++)");
    assert_eq!(
        out.get_min(),
        0xC000_0000_0000_0000u64,
        "left = (i64::MIN >> 1) & mask"
    );
    assert_eq!(
        out.get_end(),
        0x4000_0000_0000_0000u64,
        "right = (((i64::MAX - 1) >> 1) + 1) & mask"
    );
    assert_eq!(out.get_step(), 1, "INT_SRIGHT loses any step -> 1");
}

/// The SRIGHT canonical-rebuild path at a SMALLER output width (out_size==2 ->
/// bitPos 15, mask 0xffff).  Here `valRight + 1` (0x7fff + 1 = 0x8000) does NOT
/// overflow i64, so this isolates the rebuild's sign-extend / arithmetic-shift
/// bookkeeping from the F3 wrap.  Hand oracle (rangeutil.cc:1319-1338):
///
///   valRight = mask>>1 = 0x7fff; valLeft = 0x8000;
///   valLeft  = sign_extend(0x8000, 15) = 0xFFFF_FFFF_FFFF_8000 (bit15 set)
///   sa = 1:
///     left  = (0xFFFF_FFFF_FFFF_8000 >> 1) & 0xffff = 0xC000  (arith shr)
///     right = (((0x7fff - 1) >> 1) + 1) & 0xffff    = 0x4000
///   left != right -> no fix-up.
#[test]
fn r3_sright_out2_canonical_rebuild_matches_x86_oracle() {
    let in1 = CircleRange::new(0x10, 0x05, 2, 1); // wraps -> rebuild fires
    let in2 = CircleRange::new(1, 2, 2, 1);
    let mut out = CircleRange::new_empty();
    let ok = out.push_forward_binary(OpCode::CPUI_INT_SRIGHT, &in1, &in2, 2, 2, 32);
    assert!(ok);
    assert_eq!(out.get_min(), 0xC000u64, "out2 left = 0xC000 (x86 oracle)");
    assert_eq!(out.get_end(), 0x4000u64, "out2 right = 0x4000 (x86 oracle)");
}

/// The SRIGHT *non-wrapping* common path: valLeft < valRight, so the canonical
/// rebuild (and therefore the `wadd`) is SKIPPED.  Proves the F3 fix left the
/// hot path untouched.  in1 = [0x05, 0x10) over 8 bytes (both small positive),
/// in2 = single 1, out_size 8.  Oracle (rangeutil.cc:1326-1337):
///
///   valLeft  = 0x05, valRight = 0x10 ; 0x05 < 0x10 -> NO rebuild.
///   sa = 1: left  = (0x05 >> 1) & mask              = 2
///           right = (((0x10 - 1) >> 1) + 1) & mask  = 8
///   left != right -> no fix-up.
#[test]
fn r3_sright_out8_nonwrapping_skips_rebuild() {
    let in1 = CircleRange::new(0x05, 0x10, 8, 1);
    let in2 = CircleRange::new(1, 2, 8, 1);
    let mut out = CircleRange::new_empty();
    let ok = out.push_forward_binary(OpCode::CPUI_INT_SRIGHT, &in1, &in2, 8, 8, 32);
    assert!(ok);
    assert_eq!(out.get_min(), 2u64, "non-wrapping left = 0x05>>1 = 2");
    assert_eq!(out.get_end(), 8u64, "non-wrapping right = ((0xf>>1)+1) = 8");
}

// ---------------------------------------------------------------------------
// get_size overflow branch — step OTHER than the porter's, plus a precise count.
// ---------------------------------------------------------------------------

/// `getSize` (rangeutil.cc:256-273) full 8-byte range, step 4.  left==right==0,
/// mask=u64::MAX.  `val = (mask - 0 + 4)/4`; `mask + 4` wraps to 3 over uintb,
/// `3/4 = 0` -> overflow branch: `val = mask`; step>1: `val = mask/4 + 1`.
///   mask/4 = 0x3FFF_FFFF_FFFF_FFFF ; +1 = 0x4000_0000_0000_0000.
/// (Independent of the porter's step-2 case.)
#[test]
fn r3_get_size_full_8byte_step4_overflow_oracle() {
    let full_s4 = CircleRange::new(0, 0, 8, 4);
    assert_eq!(
        full_s4.get_size(),
        0x4000_0000_0000_0000u64,
        "full 8-byte step-4 getSize = mask/4 + 1 (overflow branch)"
    );
}

/// `getSize` precise non-wrapping count: [0x10, 0x20) step 2 -> (0x20-0x10)/2 = 8.
#[test]
fn r3_get_size_nonwrapping_precise_count() {
    let r = CircleRange::new(0x10, 0x20, 8, 2);
    assert_eq!(r.get_size(), 8u64, "(0x20-0x10)/2 = 8");
    // step 1 over a sub-byte range: [3, 9) -> 6.
    let r2 = CircleRange::new(3, 9, 1, 1);
    assert_eq!(r2.get_size(), 6u64, "(9-3)/1 = 6");
}

// ---------------------------------------------------------------------------
// Bourdoncle WTO determinism across two independent sibling loops.
// ---------------------------------------------------------------------------

struct AdjGraph {
    successors: Vec<Vec<usize>>,
    root_index: usize,
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

/// root -> 0 ; 0 -> {1, 3} ; 1 -> 2 -> 1 (loop A head=1) ; 3 -> 4 -> 3 (loop B
/// head=3).  Two independent sub-loops.  WTO must (a) be deterministic across
/// repeated runs (no HashSet/HashMap nondeterminism leaking into iteration
/// order), (b) mark BOTH loop heads (1 and 3) as partition heads, and (c) order
/// each head before its body.
#[test]
fn r3_wto_two_sibling_loops_deterministic_and_nested() {
    let build = || {
        let mut solver = ValueSetSolver::new();
        let n0 = solver.alloc_value_set();
        let n1 = solver.alloc_value_set();
        let n2 = solver.alloc_value_set();
        let n3 = solver.alloc_value_set();
        let n4 = solver.alloc_value_set();
        solver.push_root(n0);
        // transient root appended at value_nodes.len() == 5.
        let graph = AdjGraph {
            successors: vec![
                vec![n1, n3], // 0 -> {1,3}
                vec![n2],     // 1 -> 2
                vec![n1],     // 2 -> 1   (loop A back-edge)
                vec![n4],     // 3 -> 4
                vec![n3],     // 4 -> 3   (loop B back-edge)
            ],
            root_index: 5,
            root_edges: vec![n0],
        };
        solver.establish_topological_order(&graph);
        (solver, [n0, n1, n2, n3, n4])
    };

    let (s1, ids) = build();
    let order1 = s1.iteration_order();
    let (s2, _) = build();
    let order2 = s2.iteration_order();
    assert_eq!(order1, order2, "WTO iteration order must be deterministic");

    let [n0, n1, n2, n3, n4] = ids;
    // All five nodes present.
    for n in [n0, n1, n2, n3, n4] {
        assert!(order1.contains(&n), "node {n} present in iteration order");
    }
    // Both loop heads are partition heads.
    assert!(s1.part_head_of(n1).is_some(), "loop-A head (1) is a partition head");
    assert!(s1.part_head_of(n3).is_some(), "loop-B head (3) is a partition head");
    // Non-head loop bodies are NOT partition heads (single-entry loops).
    assert!(s1.part_head_of(n0).is_none(), "node 0 is acyclic, not a head");
    // Each head precedes its body.
    let pos = |x: usize| order1.iter().position(|&y| y == x).unwrap();
    assert!(pos(n1) < pos(n2), "loop-A head precedes its body");
    assert!(pos(n3) < pos(n4), "loop-B head precedes its body");
    // The acyclic predecessor 0 comes before both loops.
    assert!(pos(n0) < pos(n1) && pos(n0) < pos(n3), "0 iterates before both loops");
}
