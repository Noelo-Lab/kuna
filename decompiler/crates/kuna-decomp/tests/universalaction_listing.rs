//! Byte-for-byte regression check of kuna's decompile action-tree listing.
//! The fixture includes kuna-specific passes; it is not a raw C++ oracle.
//! See `fixtures/README.md` for its provenance and update policy.

use kuna_decomp::universalaction::{universal_sched, ActionListFilter, UNPORTED_ALLOWLIST};

/// The C++ `decompile` group members (verbatim from
/// `coreaction.cc` `buildDefaultGroups` / `action.rs::build_default_groups`).
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
fn decompile_tree_dump_is_byte_equal_to_snapshot() {
    assert!(
        UNPORTED_ALLOWLIST.is_empty(),
        "UNPORTED_ALLOWLIST must be empty (still missing: {:?})",
        UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect::<Vec<_>>()
    );

    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    let rust_dump = sched.list_action_dump(&filter);

    let snapshot = include_str!("fixtures/list_action_decompile_snapshot.txt");

    if rust_dump != snapshot {
        // Produce a focused first-divergence report.
        let r: Vec<&str> = rust_dump.lines().collect();
        let e: Vec<&str> = snapshot.lines().collect();
        let n = r.len().min(e.len());
        let mut first = None;
        for i in 0..n {
            if r[i] != e[i] {
                first = Some(i);
                break;
            }
        }
        let ctx = |v: &[&str], i: usize| {
            let lo = i.saturating_sub(3);
            let hi = (i + 4).min(v.len());
            v[lo..hi].join("\n")
        };
        match first {
            Some(i) => panic!(
                "decompile dump diverges from snapshot at line {i}:\n\
                 --- rust ---\n{}\n--- snapshot ---\n{}\n\
                 (rust has {} lines, snapshot {} lines)",
                ctx(&r, i),
                ctx(&e, i),
                r.len(),
                e.len()
            ),
            None => panic!(
                "decompile dump length differs: rust {} lines vs snapshot {} lines\n\
                 tail rust:\n{}\ntail snapshot:\n{}",
                r.len(),
                e.len(),
                r[r.len().saturating_sub(4)..].join("\n"),
                e[e.len().saturating_sub(4)..].join("\n"),
            ),
        }
    }

    let total = rust_dump.lines().filter(|l| !l.is_empty()).count();
    eprintln!("decompile tree: {total} actions+rules (byte-equal to the kuna snapshot, allowlist empty)");
}
