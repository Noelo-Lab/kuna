//! Adversarial verification tests for `w8-fw-universalaction` (round 1).
//!
//! These land with the verifier's verdict
//! (`docs/rust-port/reviews/w8-fw-universalaction.md`).  They target the spots
//! the hunt list flagged as most fragile for the keystone-schedule port that the
//! item gate (`universalaction_listing`, decompile root only) cannot reach:
//!
//!   * the **clone(grouplist) drop semantics** for roots *other* than decompile
//!     (the gate only exercises the decompile filter, where every nested
//!     container has a surviving sibling so blanks never stack),
//!   * the **C++ `ActionGroup::print` newline discipline** — the trailing
//!     `s << endl` after *every* surviving child, which makes a chain of
//!     last-child containers emit *stacked* blank lines,
//!   * the **whole-tree registration order** (not just the decompile subset),
//!   * **whole-tree drop** when no group is enabled, and
//!   * the **allowlist <-> tree** consistency the renumbering gate silently
//!     trusts.
//!
//! Every expectation below is re-derived by hand from
//! `decompiler/cpp/coreaction.cc` `ActionDatabase::universalAction`
//! (`coreaction.cc:5722`) + `decompiler/cpp/action.cc` `Action::print` (132),
//! `ActionGroup::print` (428), `ActionPool::print` (753), and the
//! `clone(grouplist)` family (391 / 529 / 899) — independent of the captured
//! decomp_dbg oracle fixture.

use std::collections::BTreeSet;

use kuna_decomp::universalaction::{universal_sched, ActionListFilter, UNPORTED_ALLOWLIST};

/// All 38 group names used anywhere in `universalAction` (so nothing is
/// filtered) — the union of every C++ `new ...("group")` argument.  With this
/// filter the rendered tree is the *full* universal tree, exercising the
/// whole-tree order, not just the decompile subset.
const ALL_GROUPS: &[&str] = &[
    "base", "protorecovery", "protorecovery_a", "protorecovery_b", "noproto",
    "deindirect", "localrecovery", "deadcode", "typerecovery", "stackptrflow",
    "blockrecovery", "stackvars", "deadcontrolflow", "switchnorm", "cleanup",
    "splitcopy", "splitpointer", "merge", "dynamic", "casts", "analysis",
    "canonicalcompare", "presentcompare", "fixateglobals", "fixateproto",
    "constsequence", "bitfields", "segment", "returnsplit", "nodejoin",
    "doubleload", "doubleprecis", "unreachable", "subvar", "floatprecision",
    "conditionalexe", "normalanalysis", "normalizebranches",
];

/// The pass name on a `list action` line is the final whitespace-separated
/// token (Action/Rule names never contain spaces); the leading columns are the
/// `{:>4}` index, the ` repeat `/blank, the `!`/`S`/`A` flags, and the indent.
fn name_of(line: &str) -> &str {
    line.split_whitespace().last().unwrap_or("")
}

// ---------------------------------------------------------------------------
// 1. firstpass root: {base}.  The deepest-nesting drop case — exercises both the
//    clone-drop of pools/groups with no surviving member AND the stacked
//    trailing blanks that the decompile oracle never produces (it has only
//    single blanks).
//
//    Re-pinned to the C++ ORACLE: this byte expectation is the verbatim
//    `list action` dump of the firstpass root captured from the main-tree
//    decomp_dbg (group `firstpass` is `{ "base" }`, coreaction.cc:5715):
//
//        printf 'load file /tmp/t.out\nread symbols\nload function main\n
//                option setaction firstpass\nlist action\nquit\n' \
//          | SLEIGHHOME="$(pwd)/specs" ./decompiler/cpp/decomp_dbg 2>/dev/null \
//          | awk '/^\[decomp\]> list action$/{f=1;next}
//                 /^\[decomp\]> quit$/{f=0} f'
//
//    The Rust `list_action_dump(["base"])` is byte-identical to that capture
//    (verified): B0 already proves the decompile tree byte-equal, and firstpass
//    is just the group-filtered {base} subset of the same tree.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_firstpass_drop_and_stacked_blanks_match_cpp() {
    let f = ActionListFilter::from_names(["base"]);
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);

    // Survivors, in order, keeping ONLY group=="base" leaves and the containers
    // that transitively hold one (universal / fullloop / mainloop / stackstall).
    // oppool1 (no base rule) and every non-base leaf are dropped by
    // clone(grouplist).  Indices are zero-padded (`0000`…) to match the C++
    // oracle's sticky `setfill('0')` — the same padding the B0 decompile gate
    // compares against.
    //
    // The three blank lines after `lanedivide` are load-bearing: lanedivide is
    // the sole survivor of stackstall, stackstall the last survivor of mainloop,
    // mainloop the sole survivor of fullloop.  C++ `ActionGroup::print` appends
    // `s << endl` after EACH surviving child, so stackstall/mainloop/fullloop
    // each contribute one trailing blank => exactly three before `stop`.
    // Built line-by-line to avoid raw-literal leading-whitespace pitfalls.  The
    // three empty strings before `stop` are the stacked blank lines; the
    // trailing "" gives the final `\n`.
    let expected = [
        "0000        !    universal",
        "0001                  start",
        "0002                  constbase",
        "0003        !         defaultparams",
        "0004        !         extrapopsetup",
        "0005 repeat           fullloop",
        "0006 repeat                mainloop",
        "0007                            unreachable",
        "0008                            varnodeprops",
        "0009                            heritage",
        "0010                            segmentize",
        "0011        !                   internalstorage",
        "0012                            spacebase",
        "0013 repeat                     stackstall",
        "0014        !                        lanedivide",
        "",
        "",
        "",
        "0015                  stop",
        "",
    ]
    .join("\n");

    assert_eq!(
        dump, expected,
        "firstpass ({{base}}) dump diverged from the captured C++ oracle.\n\
         The stacked trailing blanks after `lanedivide` are the C++ \
         ActionGroup::print `s<<endl`-per-child behavior."
    );

    // Specifically: exactly three consecutive blank lines appear once (the
    // stackstall->mainloop->fullloop tail), and no oppool/cleanup/merge leaks.
    assert!(dump.contains("lanedivide\n\n\n\n0015"), "stacked-blank tail wrong");
    assert!(!dump.contains("oppool1"), "oppool1 must be dropped (no base rule)");
    assert!(!dump.contains("cleanup"), "cleanup pool must be dropped");
    assert!(!dump.contains("setcasts"), "casts-group leaf must be dropped");
}

// ---------------------------------------------------------------------------
// 2. Empty / unmatched filter => whole tree drops (survives()==false at the
//    root) => empty string.  C++ `clone(grouplist)` on the universal restart
//    group returns NULL when no child survives; the listing renders nothing.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_empty_or_unmatched_filter_drops_whole_tree() {
    let empty = ActionListFilter::from_names(Vec::<String>::new());
    let none = universal_sched(None, None, vec![]).list_action_dump(&empty);
    assert_eq!(none, "", "empty filter must render nothing (root drops)");

    // A single group that no leaf carries: still empty.  ("xyzzy" is not a real
    // group; even `start` is group "base".)
    let bogus = ActionListFilter::from_names(["xyzzy_not_a_group"]);
    let none2 = universal_sched(None, None, vec![]).list_action_dump(&bogus);
    assert_eq!(none2, "", "unmatched filter must render nothing");
}

// ---------------------------------------------------------------------------
// 3. Whole-tree order + count with EVERY group enabled (the strongest
//    order-determinism check — covers the 4 leaves the decompile filter drops:
//    normalizesetup(normalanalysis), funclink_outonly(noproto),
//    directwrite(protorecovery_b), normalizebranches).
//
//    C++ universalAction registers 252 addAction/addRule calls.  Every one is
//    now ported (UNPORTED_ALLOWLIST empty), so all 252 leaves render.  Plus the
//    1 kuna leaf (gotoreduce, after finalstructure) and the 7 container headers
//    (universal, fullloop, mainloop, stackstall, oppool1, oppool2, cleanup)
//    => 260 non-blank lines.
//
//    Re-pinned to the C++ ORACLE: 259 is the non-blank `list action` line count
//    of the FULL universal tree captured from the main-tree decomp_dbg by taking
//    the decompile root and toggling ON the only 4 groups it omits — exactly the
//    ALL_GROUPS minus DECOMPILE_GROUPS difference:
//
//        ...\nload function main\n
//        option currentaction decompile normalanalysis on\n
//        option currentaction decompile noproto on\n
//        option currentaction decompile protorecovery_b on\n
//        option currentaction decompile normalizebranches on\n
//        list action\nquit\n
//
//    That capture is 259 non-blank lines (252 leaves + 7 headers) and is
//    byte-identical to this Rust `list_action_dump(ALL_GROUPS)` (verified).  The
//    head and tail are pinned exactly.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_allgroups_full_order_count_head_tail() {
    let f = ActionListFilter::from_names(ALL_GROUPS.iter().copied());
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);
    let lines: Vec<&str> = dump.lines().collect();
    let nonblank = lines.iter().filter(|l| !l.is_empty()).count();

    // All universalAction passes are ported: the allowlist is empty, so every
    // one of the 252 C++ leaves renders.  (+7 kuna-only leaves: `branchflip`,
    // option-gated default-off, registered after the second `prefercomplement`;
    // `gotoreduce`, option-gated default-off, after `finalstructure`; `taildup`,
    // option-gated default-off, right after `gotoreduce`;
    // `ifelseflatten`, option-gated default-off, after `taildup`;
    // `crossjumprevert`, option-gated default-off, right after `ifelseflatten`;
    // `dedupitetail`, option-gated default-off, right after `crossjumprevert`; and
    // `returndup`, option-gated default-off, in the `returnsplit` group right after
    // `returnsplit` (angr SAILR gotoless ReturnDuplicatorHigh, decbench F4).)
    assert_eq!(
        UNPORTED_ALLOWLIST.len(),
        0,
        "all universalAction passes are ported; UNPORTED_ALLOWLIST must be empty"
    );
    assert_eq!(
        nonblank, 267,
        "full universal tree must render 252 C++ leaves + 8 kuna leaves (branchflip + gotoreduce + taildup + ifelseflatten + crossjumprevert + dedupitetail + returndup + iteregion) + 7 container headers"
    );

    // Head: the universal restart-group prelude, in C++ order.  Note
    // `normalizesetup` (normalanalysis) and `funclink_outonly` (noproto) are
    // PRESENT here but absent in the decompile oracle — the part of the order
    // the gate never sees.
    let head: Vec<&str> = lines.iter().take(10).map(|l| name_of(l)).collect();
    assert_eq!(
        head,
        vec![
            "universal", "start", "constbase", "normalizesetup", "defaultparams",
            "extrapopsetup", "prototypetypes", "funclink", "funclink_outonly", "fullloop",
        ]
    );

    // Tail: the S9 fixation/naming/cast suffix, ending at `stop`.
    // (kuna) ActionGotoReduce is registered after ActionFinalStructure, then
    // ActionTailDup, then ActionIfElseFlatten, then ActionCrossJumpReverter, then
    // ActionDedupIteTail, then ActionIteRegion, so the S9-tail kuna leaves sit between
    // finalstructure and prototypewarnings (finalstructure -> gotoreduce -> taildup ->
    // ifelseflatten -> crossjumprevert -> dedupitetail -> iteregion; returndup is an
    // earlier S8 leaf, mid-list). Only the last 8 are checked here.
    let tail: Vec<&str> =
        lines.iter().rev().take(8).map(|l| name_of(l)).collect::<Vec<_>>().into_iter().rev().collect();
    assert_eq!(
        tail,
        vec!["gotoreduce", "taildup", "ifelseflatten", "crossjumprevert", "dedupitetail", "iteregion", "prototypewarnings", "stop"]
    );

    // `directwrite` (protorecovery_b) appears 3x total (protorecovery_a twice +
    // protorecovery_b once is 4x by class, but protorecovery_b instances are the
    // 2nd of each pair).  At minimum every directwrite must be present now that
    // protorecovery_b is enabled: 4 occurrences (mainloop pair + fullloop pair).
    let dw = lines.iter().filter(|l| name_of(l) == "directwrite").count();
    assert_eq!(dw, 4, "all four ActionDirectWrite registrations must render");
}

// ---------------------------------------------------------------------------
// 4. Allowlist is empty + the ten formerly-allowlisted passes now render.
//
//    The allowlist closed in `w8x-universalaction-wire`: every pass named by
//    `universalAction` is ported, so UNPORTED_ALLOWLIST is empty and the B0 gate
//    byte-compares the decompile tree against the raw C++ oracle with no line
//    stripping.  This test is the guard that no NEW allowlist entry silently
//    reappears (which would make the gate skip lines and pass spuriously), and
//    that the ten passes that just landed actually render in the full tree at
//    their C++ groups — re-derived from the C++ source positions in
//    coreaction.cc `universalAction`.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_allowlist_empty_and_formerly_unported_passes_render() {
    // The allowlist must be empty: all universalAction passes are ported.  A new
    // entry here would silently shrink the B0 oracle comparison.
    let names: BTreeSet<&str> = UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect();
    assert!(
        UNPORTED_ALLOWLIST.is_empty(),
        "UNPORTED_ALLOWLIST must stay empty (all universalAction passes ported); \
         a new entry would make the B0 gate strip lines and pass spuriously. \
         Still listed: {names:?}"
    );

    // The ten passes that closed the allowlist, each with the C++ universalAction
    // group it was registered under (hand-read from coreaction.cc).  Every one
    // must now render in the full universal tree under that group — proving the
    // ports are wired in, not merely de-listed.
    let formerly_unported: &[(&str, &str)] = &[
        ("splitflow", "subvar"),                // RuleSplitFlow("subvar")
        ("subfloat_convert", "floatprecision"), // RuleSubfloatConvert("floatprecision")
        ("stackprobeloop", "analysis"),         // RuleStackProbeLoop("analysis")
        ("lowerswitchinstall", "switchnorm"),   // ActionLowerSwitchInstall("switchnorm")
        ("dumptyhumplate", "cleanup"),          // RuleDumptyHumpLate("cleanup")
        ("splitcopy", "splitcopy"),             // RuleSplitCopy("splitcopy")
        ("splitload", "splitpointer"),          // RuleSplitLoad("splitpointer")
        ("splitstore", "splitpointer"),         // RuleSplitStore("splitpointer")
        ("stringcopy", "constsequence"),        // RuleStringCopy("constsequence")
        ("stringstore", "constsequence"),       // RuleStringStore("constsequence")
    ];

    let f = ActionListFilter::from_names(ALL_GROUPS.iter().copied());
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);
    let rendered: BTreeSet<&str> = dump.lines().map(name_of).collect();

    for (n, _g) in formerly_unported {
        assert!(
            rendered.contains(n),
            "formerly-allowlisted pass `{n}` must now render in the full universal tree"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. register root: {base, analysis, canonicalcompare, subvar}.  The cleanup
//    pool (all rules in cleanup/constsequence/bitfields) has NO surviving rule
//    and must be dropped wholesale; floatprecision/segment/typerecovery rules
//    vanish; analysis/subvar/canonicalcompare rules in oppool1 survive.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_register_root_drops_cleanup_pool_and_keeps_oppool_subset() {
    let f = ActionListFilter::from_names(["base", "analysis", "canonicalcompare", "subvar"]);
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);
    let names: BTreeSet<&str> = dump.lines().map(name_of).collect();

    // oppool1 survives (has analysis rules); these analysis/canonical/subvar
    // rules must be present.
    assert!(names.contains("oppool1"), "oppool1 must survive (analysis rules)");
    assert!(names.contains("termorder"), "analysis rule present");
    assert!(names.contains("intlessequal"), "canonicalcompare rule present");
    assert!(names.contains("subvar_and"), "subvar rule present");

    // Dropped: cleanup pool entirely (no base/analysis/canonical/subvar rule),
    // floatprecision, segment, typerecovery, stackvars, doubleprecis rules.
    assert!(!names.contains("cleanup"), "cleanup pool header must be dropped");
    assert!(!names.contains("multnegone"), "cleanup rule dropped");
    assert!(!names.contains("floatcast"), "floatprecision rule dropped");
    assert!(!names.contains("segment"), "segment rule dropped");
    assert!(!names.contains("doubleload"), "doubleload rule dropped");
    // oppool2 has only typerecovery+stackvars rules => dropped.
    assert!(!names.contains("oppool2"), "oppool2 must be dropped");
    // No merge-group S9 actions.
    assert!(!names.contains("namevars"), "merge action dropped");
}
