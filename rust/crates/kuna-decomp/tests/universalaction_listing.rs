//! B0 gate for `rport/w8-fw-universalaction` — the keystone schedule.
//!
//! Renders a `list action`-style dump of the derived **decompile** root action
//! tree (every Action/Rule name in registration order, nested by group, in the
//! exact C++ `list action` console format) from the Rust [`SchedNode`] tree, and
//! BYTE-COMPARES it against a fixture captured from the C++ oracle.
//!
//! # Regenerating the oracle fixture
//!
//! `tests/fixtures/list_action_decompile_oracle.txt` is the raw `list action`
//! output of the **main checkout's** `decomp_dbg`, captured with:
//!
//! ```sh
//! cd <repo-root>
//! printf 'int main(){return 0;}\n' | gcc -O0 -x c - -o /tmp/t.out
//! printf 'load file /tmp/t.out\nread symbols\nload function main\nlist action\nquit\n' \
//!     | SLEIGHHOME="$(pwd)/specs" ./decompiler/cpp/decomp_dbg 2>/dev/null \
//!     | awk '/^\[decomp\]> list action$/{f=1;next} /^\[decomp\]> quit$/{f=0} f' \
//!     > crates/kuna-decomp/tests/fixtures/list_action_decompile_oracle.txt
//! ```
//!
//! `load function main` makes `decompile` the current root, so the dump is the
//! filtered decompile tree (the same root this test derives).  The console
//! prompt/echo lines are stripped by the `awk` window; the captured text is the
//! pure `Action::print` output (numbered lines + the blank line each nested
//! group/pool leaves beneath it).
//!
//! # The allowlist
//!
//! A handful of Rules/Actions named by the C++ `universalAction` are not yet
//! ported (their home modules carry SEAM markers; see
//! [`universalaction::UNPORTED_ALLOWLIST`]).  The oracle dump contains them; the
//! Rust dump omits them.  This test removes exactly those lines from the oracle
//! (and renumbers), then requires a *byte-identical* match.  It FAILS if the
//! allowlist is empty (everything should be ported → compare raw) or if any
//! allowlisted name is absent from the oracle (stale allowlist).

use std::collections::BTreeSet;

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

/// Strip the leading 4-digit index from a `list action` line, leaving the flag
/// columns + indent + name (everything that is *position-independent*).  This is
/// the key used to delete allowlisted entries and to renumber.
fn strip_index(line: &str) -> &str {
    if line.is_empty() {
        return line;
    }
    // Lines are `{:>4}` (always 4 chars for our index range) then content.
    &line[4..]
}

/// Re-derive sequential 4-digit indices over a sequence of (already index-less)
/// content lines, skipping blank lines (which carry no index in the C++ dump).
fn renumber(content_lines: &[String]) -> String {
    let mut out = String::new();
    let mut num: u32 = 0;
    for (i, c) in content_lines.iter().enumerate() {
        if c.is_empty() {
            out.push('\n');
        } else {
            out.push_str(&format!("{num:>4}{c}\n"));
            num += 1;
        }
        let _ = i;
    }
    out
}

#[test]
fn decompile_tree_dump_matches_oracle_modulo_allowlist() {
    // --- 1. Render the Rust decompile-root dump ---------------------------
    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    let rust_dump = sched.list_action_dump(&filter);

    // --- 2. Load + sanitize the C++ oracle --------------------------------
    let oracle =
        include_str!("fixtures/list_action_decompile_oracle.txt");

    // The allowlist must be non-trivially used: every entry must appear in the
    // oracle exactly once (a stale allowlist is a bug — fail loudly).
    let allow_names: BTreeSet<&str> = UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect();
    assert!(
        !allow_names.is_empty(),
        "allowlist empty: compare the raw oracle instead (everything is ported)"
    );

    // Split the oracle into content (index-less) lines, dropping any whose
    // trailing token is an allowlisted name.  Track which allowlisted names we
    // actually removed so we can detect a stale allowlist.
    let mut removed: BTreeSet<&str> = BTreeSet::new();
    let mut kept_content: Vec<String> = Vec::new();
    for raw in oracle.lines() {
        if raw.is_empty() {
            kept_content.push(String::new());
            continue;
        }
        let content = strip_index(raw);
        let name = content.trim_start();
        if let Some(&n) = allow_names.get(name) {
            removed.insert(n);
            continue; // drop the allowlisted line (Rust omits it)
        }
        kept_content.push(content.to_string());
    }

    // Every allowlisted name must have been found+removed (no stale entries).
    let missing_from_oracle: Vec<&str> =
        allow_names.iter().filter(|n| !removed.contains(*n)).copied().collect();
    assert!(
        missing_from_oracle.is_empty(),
        "allowlist entries not present in the C++ oracle (stale allowlist): {missing_from_oracle:?}\n\
         The B0 gate requires every allowlisted-missing pass to actually exist in the oracle dump."
    );

    let expected = renumber(&kept_content);

    // --- 3. Byte-compare --------------------------------------------------
    if rust_dump != expected {
        // Produce a focused first-divergence report.
        let r: Vec<&str> = rust_dump.lines().collect();
        let e: Vec<&str> = expected.lines().collect();
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
                "decompile dump diverges from oracle at line {i}:\n\
                 --- rust ---\n{}\n--- oracle (minus allowlist) ---\n{}\n\
                 (rust has {} lines, oracle-minus-allowlist {} lines)",
                ctx(&r, i),
                ctx(&e, i),
                r.len(),
                e.len()
            ),
            None => panic!(
                "decompile dump length differs: rust {} lines vs oracle-minus-allowlist {} lines\n\
                 tail rust:\n{}\ntail oracle:\n{}",
                r.len(),
                e.len(),
                r[r.len().saturating_sub(4)..].join("\n"),
                e[e.len().saturating_sub(4)..].join("\n"),
            ),
        }
    }

    // --- 4. Report the totals (also documents the allowlist) --------------
    let total = rust_dump.lines().filter(|l| !l.is_empty()).count();
    eprintln!(
        "decompile tree: {total} actions+rules (byte-equal to oracle minus {} allowlisted-missing entries)",
        UNPORTED_ALLOWLIST.len()
    );
    for e in UNPORTED_ALLOWLIST {
        eprintln!("  allowlisted-missing: {} (group {}) — {}", e.name, e.group, e.blocked_by);
    }
}
