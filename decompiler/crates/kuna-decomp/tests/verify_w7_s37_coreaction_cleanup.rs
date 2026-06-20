//! Verifier adversarial tests for `w7-s37-coreaction-cleanup`
//! (the S6/S7 merge-group `Action` shells in
//! `decompiler/crates/kuna-decomp/src/coreaction_cleanup.rs`).
//!
//! This item is a SEAM port: 13 of the 14 `Action` classes return 0 with the
//! C++ `apply` body carried as commented pseudocode; only `ActionAssignHigh`
//! is realized (it drives `Funcdata::set_high_level`).  The fragile surface is
//! therefore (a) the exact `name()`/group/`flags` strings vs the C++
//! constructors, (b) the relative schedule order + the lone `"casts"` outlier
//! in `merge_actions()` vs `universalAction` (coreaction.cc:6002-6028), (c)
//! seam *totality* — every seamed body is a pure no-op leaving `count == 0`
//! regardless of `Funcdata` state or how often it is applied, and (d)
//! `clone_filtered` producing an independent action whose counter does not
//! leak from / to the original.
//!
//! These tests are written by the verifier and land with the verdict
//! regardless of outcome (verification.md "Adversarial pass").

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_decomp::action::{ruleflags, Action, ActionContext, ActionGroupList};
use kuna_decomp::coreaction_cleanup::{
    merge_actions, ActionAssignHigh, ActionCopyMarker, ActionDominantCopy, ActionHideShadow,
    ActionMarkExplicit, ActionMarkImplied, ActionMarkIndirectOnly, ActionMergeAdjacent,
    ActionMergeCopy, ActionMergeMultiEntry, ActionMergeRequired, ActionMergeType, ActionNameVars,
    ActionSetCasts,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::seams::Architecture;

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

/// All 14 leaves, each constructed in a fresh group, as `(boxed, expected_name)`.
fn all_leaves(g: &str) -> Vec<(Box<dyn Action>, &'static str)> {
    vec![
        (ActionAssignHigh::boxed(g.to_string()), "assignhigh"),
        (ActionMergeRequired::boxed(g.to_string()), "mergerequired"),
        (ActionMarkExplicit::boxed(g.to_string()), "markexplicit"),
        (ActionMarkImplied::boxed(g.to_string()), "markimplied"),
        (ActionMergeMultiEntry::boxed(g.to_string()), "mergemultientry"),
        (ActionMergeCopy::boxed(g.to_string()), "mergecopy"),
        (ActionDominantCopy::boxed(g.to_string()), "dominantcopy"),
        (ActionMarkIndirectOnly::boxed(g.to_string()), "markindirectonly"),
        (ActionMergeAdjacent::boxed(g.to_string()), "mergeadjacent"),
        (ActionMergeType::boxed(g.to_string()), "mergetype"),
        (ActionHideShadow::boxed(g.to_string()), "hideshadow"),
        (ActionCopyMarker::boxed(g.to_string()), "copymarker"),
        (ActionNameVars::boxed(g.to_string()), "namevars"),
        (ActionSetCasts::boxed(g.to_string()), "setcasts"),
    ]
}

/// (a) Every leaf carries the exact C++ `name()` string, stores the ctor's
/// group verbatim (the C++ `Action(flags,name,g)` ctor), and is
/// `rule_onceperfunc` — checked with a group that is NOT one of the schedule's
/// real group strings (an empty string and an arbitrary token), proving the
/// group is the ctor argument and never a hard-coded literal.
///
/// C++ anchors: coreaction.hh:331/342/353/365/377/388/399/410/441/462/483 and
/// :1007/1018/1029 — all `Action(rule_onceperfunc,"<name>",g)`.
#[test]
fn w7_s37_names_groups_flags_are_ctor_verbatim() {
    for g in ["", "x", "merge", "casts"] {
        for (act, expect) in all_leaves(g) {
            assert_eq!(act.get_name(), expect, "name must match C++ ctor literal");
            assert_eq!(
                act.get_group(),
                g,
                "{expect}: group must be the ctor's `g`, not a literal"
            );
            assert_eq!(
                act.base().flags,
                ruleflags::rule_onceperfunc,
                "{expect}: all 14 are rule_onceperfunc in C++"
            );
            // No leaf is born with a non-zero change count or a non-default
            // status (it is the engine that drives those).
            assert_eq!(act.base().count, 0, "{expect}: fresh count is 0");
        }
    }
}

/// (b) `merge_actions()` reproduces the C++ `universalAction` relative order of
/// the merge/casts-group leaves (coreaction.cc:6002-6028) EXACTLY, including
/// the lone `"casts"` outlier last and "merge" for the other thirteen.  The
/// interleaved non-merge actions (ActionDynamicSymbols@6009/6026,
/// ActionLateDoNothing@6015, ActionBlockStructure@6016, ...,
/// ActionMapGlobals@6025) are intentionally absent.  A reordering or a wrong
/// group on any leaf is silent schedule corruption, so pin the whole vector.
#[test]
fn w7_s37_merge_actions_exact_schedule_and_groups() {
    let acts = merge_actions();
    // Exact (name, group) pairs in C++ schedule order.
    let expect: Vec<(&str, &str)> = vec![
        ("assignhigh", "merge"),     // cc:6002
        ("mergerequired", "merge"),  // cc:6003
        ("markexplicit", "merge"),   // cc:6004
        ("markimplied", "merge"),    // cc:6005 (BEFORE general merging)
        ("mergemultientry", "merge"), // cc:6006
        ("mergecopy", "merge"),      // cc:6007
        ("dominantcopy", "merge"),   // cc:6008
        ("markindirectonly", "merge"), // cc:6010 (after required, before speculative)
        ("mergeadjacent", "merge"),  // cc:6011
        ("mergetype", "merge"),      // cc:6012
        ("hideshadow", "merge"),     // cc:6013
        ("copymarker", "merge"),     // cc:6014
        ("namevars", "merge"),       // cc:6027
        ("setcasts", "casts"),       // cc:6028 — the only non-"merge" group
    ];
    let got: Vec<(&str, &str)> = acts.iter().map(|a| (a.get_name(), a.get_group())).collect();
    assert_eq!(got, expect, "merge_actions must mirror universalAction 6002-6028");
    // Exactly one leaf is in the "casts" group, and it is the last one.
    let casts: Vec<&str> = acts
        .iter()
        .filter(|a| a.get_group() == "casts")
        .map(|a| a.get_name())
        .collect();
    assert_eq!(casts, vec!["setcasts"], "ActionSetCasts is the sole casts-group leaf");
}

/// (c) Seam totality: EVERY seamed `apply` is a pure no-op that leaves
/// `count == 0` and returns 0 — and stays that way across repeated application
/// (the C++ `rule_onceperfunc` bodies are change-signalled by `count`, and a
/// seam that "accidentally" mutated would show up as a non-zero count or a
/// changed Funcdata).  This is checked on a non-empty Funcdata (the high layer
/// turned on first) to make sure the seam bodies do not read uninitialised
/// state into a panic and do not piggy-back on the realized assignhigh.
///
/// The realized `ActionAssignHigh` is checked separately for its idempotence
/// and for signalling NO change (C++ `return 0` with no `count += 1`).
#[test]
fn w7_s37_seam_bodies_are_total_noops_under_repetition() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();

    // Realize the one live body first so the seams run against a "hot" Funcdata.
    let mut assign = ActionAssignHigh::boxed("merge");
    assert!(!fd.is_high_on());
    assert_eq!(assign.apply(&mut fd, &mut ctx), 0);
    assert_eq!(assign.base().count, 0, "setHighLevel signals no change (C++ return 0)");
    assert!(fd.is_high_on(), "high layer on after assignhigh");
    // Idempotent: re-apply changes nothing and never signals.
    assert_eq!(assign.apply(&mut fd, &mut ctx), 0);
    assert_eq!(assign.base().count, 0);
    assert!(fd.is_high_on());

    // The 13 seamed leaves: apply each five times; every call returns 0 and the
    // accumulated count stays 0 (no hidden mutation, no spurious change-signal).
    let mut seams: Vec<Box<dyn Action>> = vec![
        ActionMergeRequired::boxed("merge"),
        ActionMarkExplicit::boxed("merge"),
        ActionMarkImplied::boxed("merge"),
        ActionMergeMultiEntry::boxed("merge"),
        ActionMergeCopy::boxed("merge"),
        ActionDominantCopy::boxed("merge"),
        ActionMarkIndirectOnly::boxed("merge"),
        ActionMergeAdjacent::boxed("merge"),
        ActionMergeType::boxed("merge"),
        ActionHideShadow::boxed("merge"),
        ActionCopyMarker::boxed("merge"),
        ActionNameVars::boxed("merge"),
        ActionSetCasts::boxed("casts"),
    ];
    let high_before = fd.is_high_on();
    for act in &mut seams {
        for _ in 0..5 {
            let r = act.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{} seam must return 0", act.get_name());
        }
        assert_eq!(
            act.base().count,
            0,
            "{} seam must signal no change even after 5 applies",
            act.get_name()
        );
    }
    // No seam touched the one observable flag we can read here.
    assert_eq!(fd.is_high_on(), high_before, "seams must not alter Funcdata state");
}

/// (d) `clone_filtered` is the C++ `clone(grouplist)` group filter: present iff
/// the grouplist contains the leaf's group, dropped (the C++ `(Action *)0`)
/// otherwise — AND the clone is independent (mutating the clone's count does
/// not touch the original, and the clone is born with the original's group).
/// A `casts`-group leaf must survive a {casts} list but be dropped by a {merge}
/// list, and vice-versa, exercising the per-leaf group string (not a shared
/// literal).
#[test]
fn w7_s37_clone_filtered_group_filter_and_independence() {
    let only_merge = ActionGroupList::from_names(["merge"]);
    let only_casts = ActionGroupList::from_names(["casts"]);
    let both = ActionGroupList::from_names(["merge", "casts"]);
    let neither = ActionGroupList::from_names(["analysis"]);

    // A "merge"-group leaf: kept by {merge} and {merge,casts}, dropped by
    // {casts} and {analysis}.
    let mr = ActionMergeRequired::boxed("merge");
    assert!(mr.clone_filtered(&only_merge).is_some());
    assert!(mr.clone_filtered(&both).is_some());
    assert!(mr.clone_filtered(&only_casts).is_none(), "merge leaf dropped by casts-only list");
    assert!(mr.clone_filtered(&neither).is_none());

    // The lone "casts"-group leaf: the mirror image.
    let sc = ActionSetCasts::boxed("casts");
    assert!(sc.clone_filtered(&only_casts).is_some());
    assert!(sc.clone_filtered(&both).is_some());
    assert!(sc.clone_filtered(&only_merge).is_none(), "casts leaf dropped by merge-only list");

    // Independence: the clone carries the original's name+group, and bumping the
    // clone's counter (via apply on the realized action) leaves the original at 0.
    let orig = ActionAssignHigh::boxed("merge");
    let mut cl = orig.clone_filtered(&only_merge).expect("kept");
    assert_eq!(cl.get_name(), "assignhigh");
    assert_eq!(cl.get_group(), "merge");
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let _ = cl.apply(&mut fd, &mut ctx); // assignhigh signals 0 anyway, but mutate the clone path
    cl.base_mut().count += 7; // forcibly diverge the clone's state
    assert_eq!(cl.base().count, 7);
    assert_eq!(orig.base().count, 0, "clone state must not leak back to the original");
}
