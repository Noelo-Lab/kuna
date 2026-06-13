//! Unit tests for the universal-action keystone (in-crate, white-box).
//!
//! The byte-for-byte `list action` oracle comparison lives in the integration
//! test `tests/universalaction_listing.rs` (the B0 gate); these cover the
//! materializer's structural invariants and the install hook.

use super::*;
use crate::action::{build_default_groups, ActionDatabase};

/// The C++ `decompile` group members (same list as `build_default_groups`).
const DECOMPILE_GROUPS: &[&str] = &[
    "base", "protorecovery", "protorecovery_a", "deindirect", "localrecovery",
    "deadcode", "typerecovery", "stackptrflow",
    "blockrecovery", "stackvars", "deadcontrolflow", "switchnorm",
    "cleanup", "splitcopy", "splitpointer", "merge", "dynamic", "casts", "analysis",
    "canonicalcompare", "presentcompare",
    "fixateglobals", "fixateproto", "constsequence", "bitfields",
    "segment", "returnsplit", "nodejoin", "doubleload", "doubleprecis",
    "unreachable", "subvar", "floatprecision",
    "conditionalexe",
];

#[test]
fn materializes_without_panic_and_names_universal_root() {
    let act = build_universal_action(None, None, vec![]);
    // The C++ outer node is the universal restart group, name "universal",
    // carrying rule_onceperfunc.
    assert_eq!(act.get_name(), UNIVERSAL_NAME);
    assert_ne!(act.base().flags & ruleflags::rule_onceperfunc, 0);
}

#[test]
fn install_into_database_derives_decompile_root() {
    let mut db = ActionDatabase::new();
    install_universal(&mut db, None, None, vec![]);
    // The default groups must be present and the universal registered.
    db.set_current("decompile").expect("derive decompile root");
    assert_eq!(db.get_current_name(), "decompile");
    let root = db.get_current().expect("decompile root present");
    // The derived root keeps the universal name (clone copies the base).
    assert_eq!(root.get_name(), UNIVERSAL_NAME);
}

#[test]
fn unported_allowlist_entries_are_unique_and_nonempty() {
    assert!(!UNPORTED_ALLOWLIST.is_empty());
    let mut names: Vec<&str> = UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect();
    let n = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), n, "duplicate names in UNPORTED_ALLOWLIST");
    for e in UNPORTED_ALLOWLIST {
        assert!(!e.blocked_by.is_empty(), "{} missing a blocking item", e.name);
        assert!(!e.group.is_empty());
    }
}

#[test]
fn dump_filters_out_excluded_group_actions() {
    // normalizesetup (group "normalanalysis"), funclink_outonly (group
    // "noproto") and normalizebranches (group "normalizebranches") are NOT in
    // the decompile grouplist, so the decompile dump must omit them.
    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    let dump = sched.list_action_dump(&filter);
    for excluded in ["normalizesetup", "funclink_outonly"] {
        assert!(
            !dump.lines().any(|l| l.trim_start_matches(|c: char| !c.is_alphabetic()) == excluded),
            "decompile dump unexpectedly contains group-filtered action {excluded}"
        );
    }

    // ...but they survive in a grouplist that includes their groups.
    let mut groups: Vec<&str> = DECOMPILE_GROUPS.to_vec();
    groups.extend(["normalanalysis", "noproto", "normalizebranches"]);
    let wide = ActionListFilter::from_names(groups);
    let wide_dump = sched.list_action_dump(&wide);
    for present in ["normalizesetup", "funclink_outonly", "normalizebranches"] {
        assert!(wide_dump.contains(present), "wider grouplist dump should contain {present}");
    }
}

#[test]
fn build_default_groups_decompile_matches_test_constant() {
    // Guard against the test's DECOMPILE_GROUPS drifting from the real group
    // map built by action.rs::build_default_groups.
    let mut db = ActionDatabase::new();
    build_default_groups(&mut db);
    let grp = db.get_group("decompile").expect("decompile group");
    for g in DECOMPILE_GROUPS {
        assert!(grp.contains(g), "decompile grouplist missing {g}");
    }
}

#[test]
fn dump_indices_are_sequential_and_dense() {
    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    let dump = sched.list_action_dump(&filter);
    let mut expect = 0u32;
    for line in dump.lines() {
        if line.is_empty() {
            continue; // blank container separator carries no index
        }
        let idx: u32 = line[..4].trim().parse().expect("4-digit index prefix");
        assert_eq!(idx, expect, "non-sequential index in dump");
        expect += 1;
    }
    assert!(expect > 200, "decompile dump unexpectedly small ({expect} entries)");
}
