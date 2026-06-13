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
//    single blanks).  This byte expectation is hand-derived from the C++
//    clone+print, NOT from any captured fixture.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_firstpass_drop_and_stacked_blanks_match_cpp() {
    let f = ActionListFilter::from_names(["base"]);
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);

    // Hand-derived survivors, in order, from coreaction.cc:5734-6031 keeping
    // ONLY group=="base" leaves and the containers that transitively hold one
    // (universal / fullloop / mainloop / stackstall).  oppool1 (no base rule)
    // and every non-base leaf are dropped by clone(grouplist).
    //
    // The three blank lines after `lanedivide` are load-bearing: lanedivide is
    // the sole survivor of stackstall, stackstall the last survivor of mainloop,
    // mainloop the sole survivor of fullloop.  C++ `ActionGroup::print` appends
    // `s << endl` after EACH surviving child, so stackstall/mainloop/fullloop
    // each contribute one trailing blank => exactly three before `stop`.
    // Built line-by-line (with the exact leading `{:>4}` index spaces) to avoid
    // raw-literal leading-whitespace pitfalls.  The three empty strings before
    // `stop` are the stacked blank lines; the trailing "" gives the final `\n`.
    let expected = [
        "   0        !    universal",
        "   1                  start",
        "   2                  constbase",
        "   3        !         defaultparams",
        "   4        !         extrapopsetup",
        "   5 repeat           fullloop",
        "   6 repeat                mainloop",
        "   7                            unreachable",
        "   8                            varnodeprops",
        "   9                            heritage",
        "  10                            segmentize",
        "  11        !                   internalstorage",
        "  12                            spacebase",
        "  13 repeat                     stackstall",
        "  14        !                        lanedivide",
        "",
        "",
        "",
        "  15                  stop",
        "",
    ]
    .join("\n");

    assert_eq!(
        dump, expected,
        "firstpass ({{base}}) dump diverged from the hand-derived C++ clone+print.\n\
         The stacked trailing blanks after `lanedivide` are the C++ \
         ActionGroup::print `s<<endl`-per-child behavior."
    );

    // Specifically: exactly three consecutive blank lines appear once (the
    // stackstall->mainloop->fullloop tail), and no oppool/cleanup/merge leaks.
    assert!(dump.contains("lanedivide\n\n\n\n  15"), "stacked-blank tail wrong");
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
//    C++ universalAction registers 252 addAction/addRule calls.  This Rust port
//    omits 10 allowlisted-unported passes, so 242 leaves render.  Plus the 7
//    container headers (universal, fullloop, mainloop, stackstall, oppool1,
//    oppool2, cleanup) => 249 non-blank lines.  The head and tail are pinned
//    exactly.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_allgroups_full_order_count_head_tail() {
    let f = ActionListFilter::from_names(ALL_GROUPS.iter().copied());
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);
    let lines: Vec<&str> = dump.lines().collect();
    let nonblank = lines.iter().filter(|l| !l.is_empty()).count();

    // 242 leaves (252 C++ - 10 allowlisted) + 7 container headers.
    assert_eq!(
        UNPORTED_ALLOWLIST.len(),
        10,
        "this count assertion assumes exactly 10 allowlisted-unported passes"
    );
    assert_eq!(
        nonblank, 249,
        "full universal tree must render 242 leaves + 7 container headers"
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
    let tail: Vec<&str> =
        lines.iter().rev().take(4).map(|l| name_of(l)).collect::<Vec<_>>().into_iter().rev().collect();
    assert_eq!(tail, vec!["setcasts", "finalstructure", "prototypewarnings", "stop"]);

    // `directwrite` (protorecovery_b) appears 3x total (protorecovery_a twice +
    // protorecovery_b once is 4x by class, but protorecovery_b instances are the
    // 2nd of each pair).  At minimum every directwrite must be present now that
    // protorecovery_b is enabled: 4 occurrences (mainloop pair + fullloop pair).
    let dw = lines.iter().filter(|l| name_of(l) == "directwrite").count();
    assert_eq!(dw, 4, "all four ActionDirectWrite registrations must render");
}

// ---------------------------------------------------------------------------
// 4. Allowlist <-> tree consistency.  The gate's renumber-and-compare silently
//    trusts that (a) every allowlist name is unique and (b) its declared group
//    is the group the tree would have filtered on.  A stale or mis-grouped
//    allowlist entry would make the gate pass spuriously.  Re-derive each
//    entry's group from the C++ source position.
// ---------------------------------------------------------------------------
#[test]
fn w8_fw_universalaction_allowlist_is_unique_and_correctly_grouped() {
    // (name -> C++ universalAction group argument), hand-read from coreaction.cc.
    let expected_group: &[(&str, &str)] = &[
        ("splitflow", "subvar"),                 // 5893 RuleSplitFlow("subvar")
        ("subfloat_convert", "floatprecision"),  // 5903 RuleSubfloatConvert("floatprecision")
        ("stackprobeloop", "analysis"),          // 5917 RuleStackProbeLoop("analysis")
        ("lowerswitchinstall", "switchnorm"),    // 5755 ActionLowerSwitchInstall("switchnorm")
        ("dumptyhumplate", "cleanup"),           // 5977 RuleDumptyHumpLate("cleanup")
        ("splitcopy", "splitcopy"),              // 5984 RuleSplitCopy("splitcopy")
        ("splitload", "splitpointer"),           // 5985 RuleSplitLoad("splitpointer")
        ("splitstore", "splitpointer"),          // 5986 RuleSplitStore("splitpointer")
        ("stringcopy", "constsequence"),         // 5987 RuleStringCopy("constsequence")
        ("stringstore", "constsequence"),        // 5988 RuleStringStore("constsequence")
    ];

    // Uniqueness of allowlisted names.
    let names: BTreeSet<&str> = UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect();
    assert_eq!(
        names.len(),
        UNPORTED_ALLOWLIST.len(),
        "duplicate name in UNPORTED_ALLOWLIST"
    );

    // Same set as the hand-derived expectation, same group on each.
    let exp_names: BTreeSet<&str> = expected_group.iter().map(|(n, _)| *n).collect();
    assert_eq!(names, exp_names, "allowlist name set drifted from the C++ tree");

    for (n, g) in expected_group {
        let entry = UNPORTED_ALLOWLIST.iter().find(|e| e.name == *n).unwrap();
        assert_eq!(
            entry.group, *g,
            "allowlist `{n}` group must equal its C++ universalAction group `{g}`"
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
