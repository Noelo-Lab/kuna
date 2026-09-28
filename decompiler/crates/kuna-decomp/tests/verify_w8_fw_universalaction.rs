//! Schedule checks beyond the decompile snapshot: other root filters,
//! container formatting, full-tree order and formerly unported passes.
//! The firstpass expectation retains its original C++ capture; the full tree
//! includes kuna-specific additions.

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
/// zero-padded width-4 index, the ` repeat `/blank, the `!`/`S`/`A` flags, and the indent.
fn name_of(line: &str) -> &str {
    line.split_whitespace().last().unwrap_or("")
}

// The firstpass root keeps only the base group. Its original C++ capture pins
// stacked container separators as well as pass order.
#[test]
fn w8_fw_universalaction_firstpass_drop_and_stacked_blanks_match_cpp() {
    let f = ActionListFilter::from_names(["base"]);
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);

    // Survivors, in order, keeping ONLY group=="base" leaves and the containers
    // that transitively hold one (universal / fullloop / mainloop / stackstall).
    // oppool1 (no base rule) and every non-base leaf are dropped by
    // clone(grouplist).  Indices are zero-padded (`0000`…) to match the C++
    // console's sticky `setfill('0')` — the same padding the decompile snapshot
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

// All groups include registrations omitted from the decompile snapshot.
#[test]
fn w8_fw_universalaction_allgroups_full_order_count_head_tail() {
    let f = ActionListFilter::from_names(ALL_GROUPS.iter().copied());
    let dump = universal_sched(None, None, vec![]).list_action_dump(&f);
    let lines: Vec<&str> = dump.lines().collect();
    let nonblank = lines.iter().filter(|l| !l.is_empty()).count();

    assert_eq!(
        UNPORTED_ALLOWLIST.len(),
        0,
        "all universalAction passes are ported; UNPORTED_ALLOWLIST must be empty"
    );
    assert_eq!(
        nonblank, 284,
        "full kuna schedule registration count changed"
    );

    // These setup passes include groups absent from the decompile snapshot.
    let head: Vec<&str> = lines.iter().take(14).map(|l| name_of(l)).collect();
    assert_eq!(
        head,
        vec![
            "universal", "start", "constbase", "linuxsyscall", "x64syscall", "pebnames",
            "msvcstrappend", "normalizesetup", "defaultparams", "extrapopsetup", "prototypetypes",
            "funclink", "funclink_outonly", "fullloop",
        ]
    );

    let tail: Vec<&str> =
        lines.iter().rev().take(10).map(|l| name_of(l)).collect::<Vec<_>>().into_iter().rev().collect();
    assert_eq!(
        tail,
        vec!["gotoreduce", "taildup", "ifelseflatten", "crossjumprevert", "dedupitetail", "iteregion", "iteboolean", "paramcopyhoist", "prototypewarnings", "stop"]
    );

    // `directwrite` (protorecovery_b) appears 3x total (protorecovery_a twice +
    // protorecovery_b once is 4x by class, but protorecovery_b instances are the
    // 2nd of each pair).  At minimum every directwrite must be present now that
    // protorecovery_b is enabled: 4 occurrences (mainloop pair + fullloop pair).
    let dw = lines.iter().filter(|l| name_of(l) == "directwrite").count();
    assert_eq!(dw, 4, "all four ActionDirectWrite registrations must render");
}

// Keep formerly unported passes registered, with no allowlisted omissions.
#[test]
fn w8_fw_universalaction_allowlist_empty_and_formerly_unported_passes_render() {
    let names: BTreeSet<&str> = UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect();
    assert!(
        UNPORTED_ALLOWLIST.is_empty(),
        "UNPORTED_ALLOWLIST must stay empty (all universalAction passes ported); \
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
