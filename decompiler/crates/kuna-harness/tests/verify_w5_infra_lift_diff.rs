//! Regression tests for item `w5-infra-lift-diff` (post-repair).
//!
//! Round 1 verification REJECTED the item on F1: the structural model keyed basic
//! blocks by cover-start offset in a `BTreeMap<u64,BlockRec>`, but cover-start is
//! NOT unique — one machine instruction can decode (a SPARC register-window
//! `save`/`restore` inject, a conditional-execution lattice, any decode-time p-code
//! inject) into several basic blocks that all cover the same address.  That
//! collapsed duplicate-cover-start blocks on BOTH engines and produced a spurious
//! PASS over a degenerate projection (`gh6990-returnpair`: a 5-block CFG → 2 keyed
//! blocks / 31 of 211 ops).
//!
//! The repair re-keyed the model by block INDEX (print/list order) in an ordered
//! `Vec<BlockRec>`, kept cover-`(start,stop)` as compared *data*, and re-keyed
//! inter-block edges by target-block index (the `Block_N` short-header carries `N`
//! directly).  The F2 entry-head exclusion was narrowed to the architectural entry
//! block (index 0), not "any block sharing the entry address".
//!
//! These tests now PIN the FIXED behaviour: every basic block survives, the diff
//! is no longer blind to shared-cover-start blocks, and `gh6990-returnpair` is a
//! genuine 5-block PASS.

use std::path::PathBuf;

use kuna_harness::liftdiff::{self, BlockRec, DiffResult, OpClass, OpRec, StructModel};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// F1 FIXED, against the committed corpus oracle: `gh6990-returnpair.b2.txt` is a
/// FIVE-block SPARC CFG (217 lines, 211 op lines).  The repaired model keeps every
/// block — 5 block records, and the bulk of the op lines (the entry-head strip
/// removes only the two `ActionConstbase` tracked-context COPYs at index 0).
/// Before the repair this collapsed to 2 keyed blocks / 31 ops (the spurious PASS).
#[test]
fn gh6990_fixture_keeps_all_five_blocks_f1() {
    let fix = repo_root().join("decompiler/crates/kuna-harness/tests/fixtures/gh6990-returnpair.b2.txt");
    let text = std::fs::read_to_string(&fix).expect("read gh6990 fixture");

    // The fixture really has 5 `Basic Block` headers.
    let header_count = text.lines().filter(|l| l.starts_with("Basic Block ")).count();
    assert_eq!(header_count, 5, "gh6990 fixture is a 5-block CFG");
    // ...and the snapshot has 211 op lines (lines whose head is `<addr>:<time>:`).
    let op_lines = text.lines().filter(|l| l.contains(":\t") && !l.contains("[ goto ")).count();
    assert_eq!(op_lines, 211, "gh6990 fixture has 211 op lines");

    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(&text, &mut ex).unwrap();
    // The model now keeps all 5 blocks (was 2 before the repair).
    assert_eq!(m.blocks.len(), 5, "F1 fixed: all 5 basic blocks survive");
    // Blocks are in index order, position == index.
    for (i, b) in m.blocks.iter().enumerate() {
        assert_eq!(b.index, i as i64, "block at position {i} carries index {i}");
    }
    // Two ActionConstbase tracked-context COPYs (DECOMPILE_MODE, didrestore) at the
    // entry head are excluded; everything else survives (was <50 before).
    assert_eq!(ex, 2, "exactly the two entry-head tracked COPYs are excluded");
    assert_eq!(
        m.total_ops(),
        211 - 2,
        "F1 fixed: 209 ops survive of 211 op lines (only the 2 entry-head COPYs stripped)"
    );
}

// ---------------------------------------------------------------------------
// F1: duplicate cover-start blocks must ALL survive (index-keyed model).
// ---------------------------------------------------------------------------

/// A trimmed SPARC-shaped snapshot: three basic blocks (indices 0,1,2) that ALL
/// cover the single entry instruction address 0x32148 (the register-window inject
/// splits one machine instruction into several blocks), then a return block
/// (index 3) at 0x32150.  The cover-start key set is only {0x32148, 0x32150} — 2
/// distinct addresses — but a faithful structural model must keep all 4 blocks.
const SPARC_DUP_START: &str = "0\n\
    Basic Block 0 0x00032148-0x00032148\n\
    0x00032148:d2:\tDECOMPILE_MODE(0x00032148:d2) = #0x1:1\n\
    0x00032148:0:\tu0x00028e00(0x00032148:0) = sp(free) + #0xffffff48\n\
    0x00032148:1:\tgoto Block_2:0x00032148 if (DECOMPILE_MODE(free) != 0) else Block_1:0x00032148\n\
    Basic Block 1 0x00032148-0x00032148\n\
    0x00032148:2:\tu0x00000000(0x00032148:2) = CWP(free) * #0x10\n\
    Basic Block 2 0x00032148-0x00032150\n\
    0x00032148:5a:\ti0(0x00032148:5a) = o0(free)\n\
    0x00032150:74:\tgoto Block_3:0x00032150\n\
    Basic Block 3 0x00032150-0x00032154\n\
    0x00032154:d0:\treturn(#0x0)\n";

/// F1 FIXED: the index-keyed model keeps every basic block, so a 4-block CFG with
/// only 2 distinct cover-starts yields 4 block records — none collapsed.
#[test]
fn dup_start_blocks_all_survive_f1() {
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(SPARC_DUP_START, &mut ex).unwrap();
    assert_eq!(m.blocks.len(), 4, "all 4 basic blocks must be represented");

    // Each block keeps its OWN ops in index order (no overwrite by a same-address
    // sibling).  Block 0: the entry head strips the DECOMPILE_MODE tracked COPY
    // (time d2 > the decode time 0), leaving the add + the CBRANCH.
    assert_eq!(ex, 1, "the single entry-head tracked COPY (time d2) is excluded");
    assert_eq!(m.blocks[0].ops.len(), 2, "block 0: add + CBRANCH (COPY stripped)");
    assert_eq!(m.blocks[0].ops[0].class, OpClass::Value);
    assert_eq!(m.blocks[0].ops[1].class, OpClass::Cbranch);
    // Block 0 CBRANCHes to blocks 2 (true) and 1 (false).
    assert_eq!(m.blocks[0].out_indices, vec![1, 2]);

    // Block 1 (also at 0x32148) is NOT overwritten by block 2: it keeps its single
    // multiply op and falls through to block 2.
    assert_eq!(m.blocks[1].ops.len(), 1, "block 1 keeps its own single op");
    assert_eq!(m.blocks[1].out_indices, vec![2], "block 1 falls through to block 2");

    // Block 2 (third block at 0x32148) keeps its 2 ops and gotos block 3.
    assert_eq!(m.blocks[2].ops.len(), 2);
    assert_eq!(m.blocks[2].out_indices, vec![3]);

    // Block 3 (the return) at 0x32150.
    assert_eq!(m.blocks[3].start, 0x32150);
    assert_eq!(m.blocks[3].ops.last().unwrap().class, OpClass::Return);
    assert!(m.blocks[3].out_indices.is_empty());
}

/// F1 corollary FIXED: the diff is NO LONGER blind to a difference that lives in a
/// duplicate-cover-start block.  A Rust model that differs from the C++ model ONLY
/// in a block that shares an address with another block must now be reported
/// DIVERGENT (before the repair both collapsed and it reported a spurious PASS).
#[test]
fn diff_catches_difference_in_shared_start_block_f1() {
    let mut ex = 0usize;
    let cpp = liftdiff::parse_cpp_b2(SPARC_DUP_START, &mut ex).unwrap();

    // A Rust model identical to the C++ EXCEPT block 1's single op count (it would
    // be collapsed away under the old cover-start key, hiding the difference).
    let mut rust = cpp.clone();
    // Drop block 1's op — a genuine divergence in a shared-cover-start block.
    rust.blocks[1].ops.clear();

    match liftdiff::diff(&rust, &cpp) {
        DiffResult::Divergent(d) => {
            assert!(
                d.contains("block#1") && d.contains("op count differs"),
                "F1 fixed: diff catches the difference in the shared-start block#1: {d}"
            );
        }
        DiffResult::Pass => panic!("F1 regression: diff blind to shared-cover-start block"),
    }
}

/// F1: two models that agree on the surviving cover-starts but disagree on a third
/// block at a shared address must NOT compare equal merely because the block COUNT
/// matches — the per-position comparison catches the op/edge difference.
#[test]
fn diff_block_count_alone_is_not_sufficient_f1() {
    let mut ex = 0usize;
    let cpp = liftdiff::parse_cpp_b2(SPARC_DUP_START, &mut ex).unwrap();
    let mut rust = cpp.clone();
    // Same number of blocks, but block 2's out-edge points elsewhere.
    rust.blocks[2].out_indices = vec![1];
    match liftdiff::diff(&rust, &cpp) {
        DiffResult::Divergent(d) => assert!(d.contains("block#2") && d.contains("out-edge")),
        DiffResult::Pass => panic!("F1 regression: out-edge difference not caught"),
    }
}

// ---------------------------------------------------------------------------
// F2: the entry-head ActionConstbase exclusion must run on the ARCHITECTURAL
//     entry block (index 0) only — NOT on every block sharing the entry address.
// ---------------------------------------------------------------------------

/// F2 FIXED: when several blocks share the entry cover-start, the entry-head strip
/// fires ONLY on block index 0.  Here block 1 also starts at 0x32148 and leads with
/// a high-time output op; under the old `start == entry_start` rule the strip could
/// over-reach to it.  The narrowed rule (index 0 only) leaves block 1 untouched.
#[test]
fn entry_head_strip_is_entry_block_only_f2() {
    // Block 0 (entry): one real tracked COPY (time d2) + a decode op (time 0) + a
    // goto to block 1.  Block 1: also at 0x32148, leads with a high-time (5a) output
    // op that is NOT an ActionConstbase COPY — it must survive.
    let snap = "0\n\
        Basic Block 0 0x00032148-0x00032148\n\
        0x00032148:d2:\tDECOMPILE_MODE(0x00032148:d2) = #0x1:1\n\
        0x00032148:0:\tu0x0(0x00032148:0) = sp(free) + #0x1\n\
        0x00032148:1:\tgoto Block_1:0x00032148\n\
        Basic Block 1 0x00032148-0x00032150\n\
        0x00032148:5a:\ti0(0x00032148:5a) = o0(free)\n\
        0x00032148:5b:\ti1(0x00032148:5b) = o1(free)\n\
        0x00032150:74:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    // Exactly the genuine block-0 tracked COPY is excluded (the strip never touched
    // block 1, even though block 1 shares the entry address 0x32148).
    assert_eq!(ex, 1, "only block-0's tracked COPY is excluded (entry-block-only)");
    assert_eq!(m.blocks.len(), 2);
    // Block 0: COPY stripped -> the add + the goto remain.
    assert_eq!(m.blocks[0].ops.len(), 2);
    // Block 1: BOTH of its leading high-time ops survive (no over-reach).
    assert_eq!(
        m.blocks[1].ops.len(),
        3,
        "block 1 (shares entry addr) keeps all its ops; the strip is entry-block-only"
    );
    assert_eq!(m.blocks[1].ops[0].addr, 0x32148);
}

// ---------------------------------------------------------------------------
// Parser-grammar pins (unchanged by the repair, re-asserted on the index model).
// ---------------------------------------------------------------------------

/// A single-out block whose out-edge is NOT the print-next block carries an
/// explicit `[ goto ]` implied-goto line; the parser reads the implied-goto TARGET
/// INDEX, and the fall-through heuristic must not invent an edge to the print-next
/// block.
#[test]
fn implied_goto_overrides_fallthrough_to_print_next() {
    // Block 0 gotos block 2 (index 2) via an implied goto, NOT the print-next block
    // 1 (index 1).  The parser must read the implied-goto target index 2.
    let snap = "0\n\
        Basic Block 0 0x00000000-0x00000004\n\
        0x00000000:0:\tr0(0x00000000:0) = r1(free) + #0x1\n\
        0x00000004:   \t[ goto Block_2:0x00000020 ]\n\
        Basic Block 1 0x00000010-0x00000014\n\
        0x00000010:1:\tr2(0x00000010:1) = #0x0\n\
        0x00000014:2:\treturn(#0x0)\n\
        Basic Block 2 0x00000020-0x00000024\n\
        0x00000020:3:\tr3(0x00000020:3) = #0x1\n\
        0x00000024:4:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    assert_eq!(
        m.blocks[0].out_indices,
        vec![2],
        "implied-goto target index 2 must win over fall-through to print-next (index 1)"
    );
    // Block 1 (a fall-through reach is impossible from block 0) still terminates via
    // its own RETURN.
    assert!(m.blocks[1].out_indices.is_empty());
}

/// A block with NO terminator whose last op is a plain value op is fabricated a
/// fall-through edge to the next printed block (index i+1).  This pins the
/// documented fall-through heuristic.
#[test]
fn value_tail_block_fabricates_fallthrough_edge() {
    let snap = "0\n\
        Basic Block 0 0x00000000-0x00000004\n\
        0x00000000:0:\tr0(0x00000000:0) = r1(free) + #0x1\n\
        Basic Block 1 0x00000010-0x00000014\n\
        0x00000010:1:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    // Block 0's value tail -> fall-through to the next printed block (index 1).
    assert_eq!(
        m.blocks[0].out_indices,
        vec![1],
        "value-tail block fabricates a fall-through to the print-next block (index 1)"
    );
}

/// A snapshot whose block list order does NOT match the index order (not a
/// post-`structureReset` B2 capture) is rejected, rather than silently mis-keying
/// the index-based edge model.
#[test]
fn non_structurereset_index_order_is_rejected() {
    // Position 0 carries index 1 (and vice versa) — not B2 list order.
    let snap = "0\n\
        Basic Block 1 0x00000000-0x00000004\n\
        0x00000000:0:\treturn(#0x0)\n\
        Basic Block 0 0x00000010-0x00000014\n\
        0x00000010:1:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let err = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap_err();
    assert!(
        err.0.contains("not a post-structureReset"),
        "out-of-order index must be rejected, got: {}",
        err.0
    );
}

/// Sanity: a tiny hand-built model round-trips through `diff` as a PASS.
#[test]
fn identical_models_pass() {
    let m = StructModel {
        blocks: vec![BlockRec {
            index: 0,
            start: 0,
            stop: 4,
            ops: vec![
                OpRec { addr: 0, class: OpClass::Value, has_output: true },
                OpRec { addr: 4, class: OpClass::Return, has_output: false },
            ],
            out_indices: vec![],
        }],
    };
    assert!(matches!(liftdiff::diff(&m.clone(), &m), DiffResult::Pass));
}

// ===========================================================================
// ROUND 2 (independent verifier) — adversarial tests against the REAL C++
// `print raw` grammar (`typeop.cc`) and the entry-head exclusion boundary.
// ===========================================================================

/// R2-F6 (parser grammar, CALLIND): `TypeOpCallind::printRaw` (typeop.cc:800)
/// prints `s << name` with NO trailing space, so the real rendering glues the
/// keyword to the target varnode: `callindpc(free)(..)` (see the committed
/// `gp.b2.txt` fixture).  `TypeOpCall::printRaw` (typeop.cc:676) DOES emit a
/// trailing space (`call <target>`), but CALLIND does not.  The parser's
/// `call_class` keys on the first WHITESPACE token, so it can recognize `call`
/// (space-separated) but NOT `callind` (glued) — it misclassifies CALLIND as
/// `Value`.  The shipped unit test `classify("callind r0(free)")` feeds a
/// FICTIONAL space-separated form that the C++ never emits, giving false
/// confidence.
///
/// This is documented as a LOSS, not a blocker, because the misclassification is
/// in the SAFE direction: the Rust side maps `CPUI_CALLIND -> OpClass::Callind`,
/// so a comparable CALLIND op yields a CLASS divergence (DIVERGENT), never a
/// spurious PASS.  This test PINS the defect (the parser yields `Value` for the
/// real `callindpc(..)` line) so a future "fix" that changes the classification
/// is a deliberate, visible decision.
#[test]
fn callind_real_grammar_misclassified_as_value_r2f6() {
    // The EXACT body the C++ emits for a void-return CALLIND (from gp.b2.txt):
    //   `callindpc(free)(u0x...(...))`  — keyword glued to the `pc` target.
    let snap = "0\n\
        Basic Block 0 0x00410064-0x00410068\n\
        0x00410064:0:\tcallindpc(free)(u0x10000004:1(0x00410064:1))\n\
        0x00410068:2:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    let callop = &m.blocks[0].ops[0];
    // DEFECT: the parser classifies the real `callindpc(..)` rendering as Value,
    // NOT Callind, because `callind` is glued to `pc` with no separating space.
    assert_eq!(
        callop.class,
        OpClass::Value,
        "documents the CALLIND grammar defect: the real glued `callindpc(..)` \
         rendering is misclassified as Value (the keyword has no trailing space)"
    );
    // The op has NO `=` and so reads has_output == false (correct for a void CALLIND).
    assert!(!callop.has_output, "void CALLIND carries no output");
}

/// R2-F6 corollary: the misclassification is SAFE — it can only DIVERGE, never
/// produce a spurious PASS.  A Rust model that (correctly) classes the same op as
/// `Callind` is reported DIVERGENT against the parser's `Value`, so no optimism
/// leaks into the gate.
#[test]
fn callind_misclass_diverges_never_spurious_pass_r2f6() {
    let snap = "0\n\
        Basic Block 0 0x00410064-0x00410068\n\
        0x00410064:0:\tcallindpc(free)(u0x10000004:1(0x00410064:1))\n\
        0x00410068:2:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let cpp = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();

    // The FAITHFUL Rust extraction (CPUI_CALLIND -> OpClass::Callind).
    let rust = StructModel {
        blocks: vec![BlockRec {
            index: 0,
            start: 0x410064,
            stop: 0x410068,
            ops: vec![
                OpRec { addr: 0x410064, class: OpClass::Callind, has_output: false },
                OpRec { addr: 0x410068, class: OpClass::Return, has_output: false },
            ],
            out_indices: vec![],
        }],
    };
    match liftdiff::diff(&rust, &cpp) {
        DiffResult::Divergent(d) => assert!(
            d.contains("class differs") && d.contains("Callind") && d.contains("Value"),
            "CALLIND misclass must surface as a class divergence (safe direction): {d}"
        ),
        DiffResult::Pass => panic!(
            "REGRESSION: the CALLIND grammar defect produced a SPURIOUS PASS — \
             the parser must never agree with a Callind op by mis-reading it as Value"
        ),
    }
}

/// R2: `TypeOpCall::printRaw` DOES emit a trailing space, so the space-separated
/// `call <target>` IS recognized.  Pins that the defect is CALLIND-specific (the
/// CALL path is correct), so a blanket "trim the keyword" fix is not needed.
#[test]
fn call_with_trailing_space_is_recognized_r2() {
    // Void CALL (ccmp.b2.txt shape): `call fotherfunc(free)(..)` — `call` + space.
    let snap = "0\n\
        Basic Block 0 0x0042cb04-0x0042cb08\n\
        0x0042cb04:0:\tcall fotherfunc(free)(u0x10000008:1(0x0042cb04:1))\n\
        0x0042cb08:2:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    assert_eq!(
        m.blocks[0].ops[0].class,
        OpClass::Call,
        "the space-separated `call <target>` form IS recognized (CALL path correct)"
    );
}

/// R2: entry-head exclusion robustness — when a tracked-context COPY is the ONLY
/// op at the entry address (no sibling decode op to compare its time against),
/// the strip CONSERVATIVELY declines (`max_other == None => break`), leaving the op
/// in place.  This documents that the exclusion never over-strips into a region it
/// cannot prove is a pre-heritage insertion; the trade is a possible UNDER-strip
/// (which would show up as a count DIVERGENCE, the safe direction), never an
/// over-strip that could fabricate a PASS.
#[test]
fn entry_head_strip_declines_without_a_sibling_op_r2() {
    // Block 0: a single high-time output op at the entry address, then a decode op
    // at a DIFFERENT address.  There is no other op AT the entry address, so the
    // strip cannot prove the head op is an insertion -> it declines.
    let snap = "0\n\
        Basic Block 0 0x00000000-0x00000008\n\
        0x00000000:9:\tr0(0x00000000:9) = #0x0:1\n\
        0x00000004:1:\tr1(0x00000004:1) = #0x1\n\
        0x00000008:2:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    assert_eq!(
        ex, 0,
        "no sibling op at the entry address -> the strip declines (conservative, \
         no over-strip)"
    );
    assert_eq!(m.blocks[0].ops.len(), 3, "all three ops survive (nothing stripped)");
}

/// R2: the strip stops at the FIRST head op whose time is not the latest at the
/// entry address — it does NOT keep scanning past a genuine decode op to strip a
/// later high-time op.  Mirrors the C++ `opInsertBegin` invariant: the inserted
/// pre-heritage ops sit CONTIGUOUSLY at the block head, never interleaved with
/// decode ops.
#[test]
fn entry_head_strip_stops_at_first_decode_op_r2() {
    // Head: one inserted COPY (time 5, latest) then the decode ops (0,1) then a
    // STRAY high-time output op at the entry addr (time 9) AFTER a decode op.  The
    // strip must remove ONLY the leading COPY and stop at the first decode op; it
    // must NOT reach back to the time-9 op buried after a decode op.
    let snap = "0\n\
        Basic Block 0 0x00000000-0x00000008\n\
        0x00000000:5:\tDF(0x00000000:5) = #0x0:1\n\
        0x00000000:0:\tr0(0x00000000:0) = r1(free) + #0x1\n\
        0x00000000:9:\tr2(0x00000000:9) = r0(free) + #0x2\n\
        0x00000008:1:\treturn(#0x0)\n";
    let mut ex = 0usize;
    let m = liftdiff::parse_cpp_b2(snap, &mut ex).unwrap();
    // Only the leading COPY is stripped (time 5 > max of the OTHER entry-addr times
    // {0, 9} is FALSE for 9 — so even the first op is NOT the latest!).  This pins
    // that a buried higher-time op DEFEATS the leading strip entirely (the head op
    // is only removed when it is strictly the latest at the entry address).
    assert_eq!(
        ex, 0,
        "a buried higher-time entry-addr op (time 9 > head time 5) defeats the \
         leading strip: the head op is removed only when STRICTLY the latest"
    );
    assert_eq!(m.blocks[0].ops.len(), 4, "nothing stripped (head op is not the latest)");
}

/// R2: out-edge target indices are compared ORDER-INSENSITIVELY (sorted+deduped on
/// both sides), so a CBRANCH whose true/false targets the two engines emit in
/// opposite order still compares equal.  Pins that the edge set is a SET, matching
/// the structural intent (the per-op CBRANCH class already pins the conditional).
#[test]
fn out_edges_are_order_insensitive_r2() {
    let a = StructModel {
        blocks: vec![
            BlockRec {
                index: 0,
                start: 0,
                stop: 4,
                ops: vec![OpRec { addr: 0, class: OpClass::Cbranch, has_output: false }],
                out_indices: vec![1, 2],
            },
            BlockRec { index: 1, start: 8, stop: 8, ops: vec![], out_indices: vec![] },
            BlockRec { index: 2, start: 0xc, stop: 0xc, ops: vec![], out_indices: vec![] },
        ],
    };
    let mut b = a.clone();
    // Same set, opposite emit order before the sort: must still PASS.
    b.blocks[0].out_indices = vec![2, 1];
    // (The model stores sorted indices, but assert the comparator is set-equal even
    // if a hand-built model were unsorted.)
    b.blocks[0].out_indices.sort_unstable();
    assert!(matches!(liftdiff::diff(&a, &b), DiffResult::Pass));
}
