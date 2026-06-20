//! B0 gate for `rport/w8x-universalaction-wire` — the keystone schedule.
//!
//! Renders a `list action`-style dump of the derived **decompile** root action
//! tree (every Action/Rule name in registration order, nested by group, in the
//! exact C++ `list action` console format) from the Rust [`SchedNode`] tree, and
//! BYTE-COMPARES it against a fixture captured from the C++ oracle.
//!
//! # The empty-allowlist gate (B0)
//!
//! Earlier waves omitted a handful of un-ported Rules/Actions from the
//! materialized tree, recorded in [`universalaction::UNPORTED_ALLOWLIST`], and
//! this test stripped exactly those lines from the oracle before comparing.  As
//! of `w8x-universalaction-wire` every one of those passes is ported and wired in
//! at its exact C++ registration position, so the allowlist is **empty** and the
//! Rust dump must be **byte-identical** to the raw C++ oracle dump.  This test now
//! asserts the allowlist is empty and compares the two dumps verbatim, with no
//! line stripping or renumbering.
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
fn decompile_tree_dump_is_byte_equal_to_oracle() {
    // --- 1. The B0 empty-allowlist gate -----------------------------------
    // Every pass named by the C++ universalAction is now ported and wired in;
    // nothing may remain allowlisted-missing.  If a genuinely-still-missing pass
    // were re-introduced here, this assertion would fire loudly rather than let
    // the dump silently diverge.
    assert!(
        UNPORTED_ALLOWLIST.is_empty(),
        "B0 gate: UNPORTED_ALLOWLIST must be empty after wiring (still missing: {:?})",
        UNPORTED_ALLOWLIST.iter().map(|e| e.name).collect::<Vec<_>>()
    );

    // --- 2. Render the Rust decompile-root dump ---------------------------
    let sched = universal_sched(None, None, vec![]);
    let filter = ActionListFilter::from_names(DECOMPILE_GROUPS.iter().copied());
    let rust_dump = sched.list_action_dump(&filter);

    // --- 3. Load the C++ oracle (raw — no stripping, no renumbering) -------
    let oracle = include_str!("fixtures/list_action_decompile_oracle.txt");

    // --- 4. Byte-compare verbatim -----------------------------------------
    if rust_dump != oracle {
        // Produce a focused first-divergence report.
        let r: Vec<&str> = rust_dump.lines().collect();
        let e: Vec<&str> = oracle.lines().collect();
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
                 --- rust ---\n{}\n--- oracle ---\n{}\n\
                 (rust has {} lines, oracle {} lines)",
                ctx(&r, i),
                ctx(&e, i),
                r.len(),
                e.len()
            ),
            None => panic!(
                "decompile dump length differs: rust {} lines vs oracle {} lines\n\
                 tail rust:\n{}\ntail oracle:\n{}",
                r.len(),
                e.len(),
                r[r.len().saturating_sub(4)..].join("\n"),
                e[e.len().saturating_sub(4)..].join("\n"),
            ),
        }
    }

    // --- 5. Report the total ----------------------------------------------
    let total = rust_dump.lines().filter(|l| !l.is_empty()).count();
    eprintln!("decompile tree: {total} actions+rules (byte-equal to the C++ oracle, allowlist empty)");
}
