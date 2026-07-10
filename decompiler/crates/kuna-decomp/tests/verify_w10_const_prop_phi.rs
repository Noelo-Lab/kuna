//! VERIFIER adversarial tests for `rport/w10-const-prop-phi`
//! (`ActionConditionalConst`, constant propagation through MULTIEQUAL/phi edges;
//! `decompiler/cpp/coreaction.cc:4292-4780`).
//!
//! Written by the *independent verifier* (not the porter), per
//! `docs/rust-port/verification.md` (the >= 3 adversarial-test requirement).
//!
//! Two families:
//!
//!  1. **Unit tests of `FlowBlock::restrictedByConditional`** (`block.cc:405`),
//!     the predicate that computes `blockDom[0]`/`blockDom[1]` in
//!     `ActionConditionalConst::apply` — the single most fragile helper the port
//!     introduced (a dominator-tree walk with a `do/while`-flavoured inner loop
//!     and a `seenCond` multiple-direct-edge guard).  Each graph is wired with
//!     real edges and a real Cooper/Harvey/Kennedy dominator tree
//!     (`calc_forward_dominator`), so the immediate-dominator chain the predicate
//!     walks is genuine, not hand-set.  Expected values are re-derived from the
//!     C++ source, not from the Rust port.
//!
//!  2. **End-to-end real-recovery tests** driving the committed Rust
//!     `decomp_test_dbg` over `condconst.xml` (the full pipeline).  These pin the
//!     *named* constant-propagation-through-phi recoveries (`*ptr = b`,
//!     `ptr[2] = 10`, `ptr[4] = 0x10` — datatest #1/#3/#5, which FAIL on the
//!     `rust-port` base and PASS on this branch) and the GH-9203/DIV-3 loop-block
//!     decline (`iStack_c = 10` must be ABSENT — datatest #12).  Anti-special-
//!     casing: each assertion is over the recovered IR rendering, and the e2e
//!     proves the rewrite happened in the data-flow, not in the printer.

use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_decomp::block::{BlockGraph, BlockKind, FlowBlock};
use kuna_decomp::context::BlockId;

// ===========================================================================
// Family 1: restrictedByConditional unit tests
// ===========================================================================

/// Manager with const(0)/ram(1) so block covers have a space (mirrors the
/// in-module `block.rs` test harness).
fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.set_default_code_space(1).unwrap();
    m
}

/// A graph + root holding `n` plain components.
fn build_graph(n: usize) -> (BlockGraph, BlockId, Vec<BlockId>) {
    let _m = build_manager(); // construct so the space registry exists in scope
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let mut blocks = Vec::new();
    for _ in 0..n {
        blocks.push(g.new_block(root));
    }
    (g, root, blocks)
}

/// Run loop structuring + forward dominators so each block gets a real RPO index
/// and a real immediate dominator (the chain `restricted_by_conditional` walks).
fn finalize(g: &mut BlockGraph, root: BlockId, start: BlockId) {
    g.set_start_block(root, start);
    let mut rootlist = Vec::new();
    g.structure_loops(root, &mut rootlist);
    g.calc_forward_dominator(root, &rootlist);
}

/// HUNT (off-by-one / do-while + sibling walk): the DIAMOND.
///
/// `0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3`.  Block 3 is the merge with two in-edges and
/// `idom(3) == 0 == cond`.  Per the C++ (`block.cc:405`), walking in-edge `1`
/// backtracks `1 -> idom(1)=0 == cond` BEFORE reaching `this`, so the condition
/// does NOT hold throughout block 3 (it is reachable through the sibling edge).
/// Expected: restrictedByConditional(3, 0) == false.
///
/// The two arm blocks each have sizeIn()==1, so the fast path returns true.
#[test]
fn w10cpp_restricted_diamond_merge_is_not_restricted() {
    let (mut g, root, b) = build_graph(4);
    let (cond, arm1, arm2, merge) = (b[0], b[1], b[2], b[3]);
    g.add_edge(cond, arm1);
    g.add_edge(cond, arm2);
    g.add_edge(arm1, merge);
    g.add_edge(arm2, merge);
    finalize(&mut g, root, cond);

    // idom(merge) must be cond for the test to exercise the walk (not the early
    // `idom != cond` return).
    assert_eq!(
        g.block(merge).get_immed_dom(),
        Some(cond),
        "diamond merge must be immediately dominated by the split"
    );
    assert_eq!(g.block(merge).size_in(), 2, "merge has two in-edges");

    // The merge is reachable through the sibling: NOT restricted.
    assert!(
        !g.restricted_by_conditional(merge, cond),
        "diamond merge is reachable through the sibling edge => not restricted"
    );
    // Each arm has a unique in-edge from cond => sizeIn()==1 fast path => restricted.
    assert!(
        g.restricted_by_conditional(arm1, cond),
        "arm with sizeIn()==1 is trivially restricted"
    );
    assert!(g.restricted_by_conditional(arm2, cond));
}

/// HUNT (do-while at-least-once / `inBlock == this` immediate loop exit): a
/// SELF-LOOP HEADER.
///
/// `0 -> 1, 1 -> 1 (self), 1 -> 2 (exit)`.  Block 1 has in-edges `[0, 1]` and
/// `idom(1) == 0 == cond`.  The C++ walk: in-edge `0` is the direct `cond` edge
/// (seenCond := true); in-edge `1` equals `this`, so `while (inBlock != this)`
/// runs ZERO times and no sibling is found.  Expected: restrictedByConditional
/// (1, 0) == true.  This pins the Rust `while walk != Some(this_id)` against the
/// C++ `while (inBlock != this)` — the back-edge from the block to itself must NOT
/// be mistaken for a sibling path.
#[test]
fn w10cpp_restricted_self_loop_header_is_restricted() {
    let (mut g, root, b) = build_graph(3);
    let (cond, hdr, exit) = (b[0], b[1], b[2]);
    g.add_edge(cond, hdr);
    g.add_edge(hdr, hdr); // self back-edge
    g.add_edge(hdr, exit);
    finalize(&mut g, root, cond);

    assert_eq!(
        g.block(hdr).get_immed_dom(),
        Some(cond),
        "self-loop header is immediately dominated by its entry"
    );
    assert_eq!(g.block(hdr).size_in(), 2, "header in-edges: [entry, self back-edge]");
    assert!(
        g.restricted_by_conditional(hdr, cond),
        "the self back-edge equals `this`, so no sibling path exists => restricted"
    );
}

/// HUNT (the early `getImmedDom() != cond` short-circuit): a merge NOT dominated
/// by the conditional.
///
/// `0 -> 1, 0 -> 2, 1 -> 3, 1 -> 4, 4 -> 3` makes block 3 reachable only through
/// block 1, so `idom(3) == 1`, not `cond == 0`.  Per the C++ second line,
/// `getImmedDom() != cond => return false` BEFORE any walk.  Expected:
/// restrictedByConditional(3, 0) == false (early `idom != cond` exit).
///
/// And restrictedByConditional(3, 1) == false too — even though `mid` IS the
/// immediate dominator: the non-direct in-edge from `between` walks
/// `between -> idom(between)=mid` and `mid == cond` is hit BEFORE reaching
/// `this`, so the C++ inner `if (inBlock == cond) return false` fires.  This is
/// the conservative C++ behaviour (a value known down `mid -> 3` is NOT proven to
/// hold throughout 3 because flow also arrives via `mid -> 4 -> 3`); the port
/// must reproduce the `false`, NOT "fix" it to `true`.
#[test]
fn w10cpp_restricted_idom_mismatch_short_circuits_false() {
    let (mut g, root, b) = build_graph(5);
    let (entry, mid, sib, merge, between) = (b[0], b[1], b[2], b[3], b[4]);
    g.add_edge(entry, mid);
    g.add_edge(entry, sib);
    g.add_edge(mid, merge);
    g.add_edge(mid, between);
    g.add_edge(between, merge);
    finalize(&mut g, root, entry);

    assert_eq!(
        g.block(merge).get_immed_dom(),
        Some(mid),
        "merge is dominated by `mid`, not the entry"
    );
    assert_eq!(g.block(between).get_immed_dom(), Some(mid), "between is dominated by mid");
    // Wrong conditional block: idom(merge) != entry => early false.
    assert!(
        !g.restricted_by_conditional(merge, entry),
        "idom(merge) != entry => early `getImmedDom() != cond` return false"
    );
    // Correct immediate dominator, but the `between` in-edge's idom chain hits
    // `mid == cond` before `this`: conservative C++ returns false.
    assert_eq!(g.block(merge).size_in(), 2);
    assert!(
        !g.restricted_by_conditional(merge, mid),
        "the `between -> mid(==cond)` walk hits cond before `this` => not restricted \
         (faithful conservative C++ behaviour, not 'fixed' to true)"
    );
}

// ===========================================================================
// Family 2: end-to-end real-recovery (full pipeline, condconst.xml)
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    std::env::var_os("KUNA_SPECS").map(PathBuf::from).unwrap_or_else(|| repo_root().join("specs"))
}

fn datatests_root() -> PathBuf {
    std::env::var_os("KUNA_DATATESTS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("tests/datatests"))
}

/// The committed Rust `decomp_test_dbg`; `None` => skip.
fn rust_oracle() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("KUNA_RUST_DECOMP_TEST") {
        let pb = PathBuf::from(p);
        return pb.exists().then_some(pb);
    }
    for profile in ["release", "debug"] {
        let pb = repo_root().join("decompiler/target").join(profile).join("decomp_test_dbg");
        if pb.exists() {
            return Some(pb);
        }
    }
    None
}

/// Run the harness over `condconst.xml` in assertion mode and return the raw
/// per-assertion result lines (`Success -- ...` / `FAIL -- ...`).  This is the
/// faithful exercise: the harness applies the stringmatch regexes against the
/// engine's rendered C, so a pass means the constant was really propagated.
fn condconst_assertions() -> Option<String> {
    let bin = rust_oracle()?;
    let out = Command::new(&bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_root())
        .arg("datatests")
        .arg("condconst.xml")
        .output()
        .ok()?;
    if !specs_root().join("Ghidra/Processors/x86/data/languages/x86-64.sla").exists() {
        // specs not compiled in this tree
        let txt = String::from_utf8_lossy(&out.stdout);
        if txt.contains("Could not find .sla") {
            return None;
        }
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn assertion_passed(report: &str, name: &str) -> bool {
    report.lines().any(|l| l.trim() == format!("Success -- {name}"))
}

fn assertion_failed(report: &str, name: &str) -> bool {
    report.lines().any(|l| l.trim() == format!("FAIL -- {name}"))
}

/// TEST (real phi recovery): the three named constant-propagation-through-phi
/// recoveries `*ptr = b` (#1), `ptr[2] = 10` (#3), `ptr[4] = 0x10` (#5) — the
/// ones that FAIL on the base branch (stub `apply`) and PASS once
/// `handlePhiNodes`/`placeCopy` materialize the constant COPY.  This is the
/// REAL parity claim of the item.
#[test]
fn w10cpp_phi_recovery_constants_are_propagated() {
    let Some(report) = condconst_assertions() else {
        eprintln!("SKIP: rust decomp_test_dbg / compiled specs unavailable");
        return;
    };
    eprintln!("=== condconst.xml assertion report ===\n{report}");
    for name in [
        "Conditional Constant #1", // *ptr = b;  (phi recovery)
        "Conditional Constant #3", // ptr[2] = 10;
        "Conditional Constant #5", // ptr[4] = 0x10;
    ] {
        assert!(
            assertion_passed(&report, name),
            "{name} (constant-propagation-through-phi) must PASS — it is the named \
             recovery of this item and FAILS on the rust-port base:\n{report}"
        );
    }
}

/// TEST (GH-9203 / DIV-3 loop decline, anti-special-casing): the spurious
/// `iStack_c = 10` materialization is DECLINED inside a loop predecessor block.
/// The datatest pins this with `min=0 max=0` (the string must be ABSENT); the
/// `condexe_block_placement && hasLoopIn()` guard in `handlePhiNodes` is the
/// only thing producing it.  A faithful port keeps #12 passing (absent) while
/// #1/#3/#5 still recover — i.e. the loop guard is value/loop-shape driven, NOT
/// a hardcoded suppression of one variable name.
#[test]
fn w10cpp_loop_block_copy_is_declined_div3() {
    let Some(report) = condconst_assertions() else {
        eprintln!("SKIP: rust decomp_test_dbg / compiled specs unavailable");
        return;
    };
    // #12: `iStack_c = 10;` must be absent (min=0 max=0) => the harness reports
    // "Success" when it is correctly absent.
    assert!(
        assertion_passed(&report, "Conditional Constant #12"),
        "the GH-9203/DIV-3 loop-block COPY decline must hold (`iStack_c = 10;` \
         absent):\n{report}"
    );
    // And the decline must NOT have suppressed the legitimate recoveries: #1 still
    // recovers.  (If the loop guard were a blanket `return` it would kill #1 too.)
    assert!(
        assertion_passed(&report, "Conditional Constant #1"),
        "the loop-block decline must be loop-shape-scoped, not a blanket suppression:\n{report}"
    );
}

/// TEST (no regression / anti-over-propagation): the non-phi recoveries and the
/// `condconst_conn` reads that must SURVIVE all still hold, and nothing the base
/// already passed now fails.  In particular #2 (`ptr[1] = a;`) and #11
/// (`v1 = 0x14;`) are non-phi paths that must remain green; #10 (`v1 = x;`) is
/// the `condconst_conn` no-replace case — the variable must NOT be wrongly
/// replaced by a constant.  This guards against the propagation being too
/// aggressive (a faithful `restrictedByConditional`/`testAlternatePath` keeps it
/// scoped).
#[test]
fn w10cpp_non_phi_paths_unregressed_and_no_over_propagation() {
    let Some(report) = condconst_assertions() else {
        eprintln!("SKIP: rust decomp_test_dbg / compiled specs unavailable");
        return;
    };
    for name in [
        "Conditional Constant #2",  // ptr[1] = a;
        "Conditional Constant #4",  // ptr[3] = c + 8;
        "Conditional Constant #10", // v1 = x;  (must NOT be over-replaced)
        "Conditional Constant #11", // v1 = 0x14;
    ] {
        assert!(
            assertion_passed(&report, name),
            "{name} must stay passing (no over-propagation / no regression):\n{report}"
        );
        assert!(
            !assertion_failed(&report, name),
            "{name} must not be reported as a failure:\n{report}"
        );
    }
}
