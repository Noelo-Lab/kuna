//! VERIFIER adversarial tests for `rport/w10-dominant-copy`.
//!
//! Written by the *verifier* (not the porter) per `docs/rust-port/verification.md`
//! (the >= 3 adversarial-test requirement).  These drive the SAME full pipeline
//! the datatest harness uses — the committed Rust `decomp_test_dbg` binary as a
//! subprocess with `KUNA_DUMP=1` — because the dominant-copy hoist only fires
//! once input-prototype recovery (the harness's cspec wiring) has produced the
//! parameter-into-local trim COPYs.  An in-process `decompile_func` bootstrap
//! that skips the prototype does NOT exercise the hoist, so it would be a false
//! negative; the subprocess is the faithful exercise.
//!
//! They target the hunt-list-fragile spots for the hoist:
//!  * the hoist is a REAL IR rewrite matching the C++ (the `v1 = x;` dominant
//!    COPY), not a printer special-case (Test A),
//!  * a NON-dominant single-source body (boolless) is NOT over-merged — byte
//!    identical to the C++ oracle (Test B),
//!  * distinct sources into the merged local both survive the per-source grouping
//!    (Test C).
use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_SPECS") {
        PathBuf::from(p)
    } else {
        repo_root().join("specs")
    }
}

fn datatests_root() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DATATESTS") {
        PathBuf::from(p)
    } else {
        repo_root().join("decompiler/datatests")
    }
}

/// The committed Rust `decomp_test_dbg` (the engine under test).  Honors
/// `KUNA_RUST_DECOMP_TEST`, else the workspace target dir for the active profile.
fn rust_oracle() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("KUNA_RUST_DECOMP_TEST") {
        let pb = PathBuf::from(p);
        return pb.exists().then_some(pb);
    }
    for profile in ["debug", "release"] {
        let pb = repo_root().join("rust/target").join(profile).join("decomp_test_dbg");
        if pb.exists() {
            return Some(pb);
        }
    }
    None
}

/// Drive the Rust engine over one datatest file and slice the first KUNA_DUMP
/// `print C` block.  `None` when the binary/specs are unavailable (test skips).
fn rust_print_c_for(file: &str) -> Option<String> {
    let bin = rust_oracle()?;
    let out = Command::new(&bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_root())
        .arg("datatests")
        .arg(file)
        .env("KUNA_DUMP", "1")
        .env("KUNA_ENGINE", "rust")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    Some(text[after_begin..mid].to_string())
}

/// Extract a single function body (from `name(` through the matching closing
/// brace at column 0) out of the multi-function KUNA_DUMP block.
fn slice_function(block: &str, name: &str) -> Option<String> {
    let sig = format!(" {name}(");
    let start = block.lines().position(|l| l.contains(&sig))?;
    let lines: Vec<&str> = block.lines().collect();
    let mut end = start;
    for (i, l) in lines.iter().enumerate().skip(start + 1) {
        if *l == "}" {
            end = i;
            break;
        }
    }
    Some(lines[start..=end].join("\n"))
}

/// TEST A — the dominant-copy hoist is a REAL IR artifact rendering as `v1 = x;`.
///
/// `condconst_conn` assigns the parameter into a stack local across multiple
/// predecessors; the merge-trim pass inserts a COPY of the parameter on each
/// path and `ActionDominantCopy` (`processCopyTrims` -> `buildDominantCopy`) must
/// replace them with ONE dominating COPY hoisted to the common dominator,
/// rendering as the C++ oracle's `v1 = x;`.  This is the `Conditional Constant
/// #10` datatest assertion (FAIL on the base branch, PASS with the hoist).
///
/// Anti-special-casing: the assertion is `v1 = x;` over the recovered parameter
/// name `x` and merged local `v1`, NOT a hardcoded printer string.
#[test]
fn w10_dc_condconst_conn_hoist_is_real_v1_eq_x() {
    let Some(block) = rust_print_c_for("condconst.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "condconst_conn")
        .unwrap_or_else(|| panic!("condconst_conn not in dump:\n{block}"));
    eprintln!("=== RUST condconst_conn ===\n{body}");
    assert!(
        body.contains("v1 = x;"),
        "the dominant COPY of the parameter must hoist to a single `v1 = x;` \
         (Conditional Constant #10), got:\n{body}"
    );
    // The hoist folded into the SAME merged local (also assigned 0x14, returned).
    assert!(body.contains("v1 = 0x14;"), "merged local must receive the 0x14 store:\n{body}");
    assert!(body.contains("return v1;"), "merged local must be returned:\n{body}");
}

/// TEST B — a NON-dominant single-source body is NOT over-merged.
///
/// boolless has its own merge-trim COPY (`v1 = dat_52;`) but NO multi-source
/// dominant-copy group; the hoist must leave its byte output exactly equal to the
/// C++ oracle.  Anti-mis-hoist: a wrong `count <= 1` guard or a spurious fold
/// would perturb boolless.  It must stay byte-identical to the C++ B5 oracle.
#[test]
fn w10_dc_boolless_not_over_merged_byte_identical() {
    let Some(block) = rust_print_c_for("boolless.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    const CPP: &str = "\nuint1 boolless(void)\n\n{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}\n";
    assert_eq!(
        block.trim_end(),
        CPP.trim_end(),
        "the dominant-copy hoist must not perturb boolless (no over-merge):\n--- rust ---\n{block}\n--- cpp ---\n{CPP}"
    );
    assert!(block.contains("dat_52"), "the global must stay dat_52:\n{block}");
    assert!(!block.contains("dat_52 = "), "dat_52 must remain read-only (not a hoist target):\n{block}");
}

/// TEST C — distinct sources into the merged local both survive grouping.
///
/// `condconst_conn`'s local `v1` receives the parameter (`v1 = x;`, the hoisted
/// dominant COPY) AND the constant `0x14` (`v1 = 0x14;`) from DIFFERENT source
/// Varnodes — two separate groups in `processHighDominantCopy`.  If the
/// per-source grouping (`while next_vn == in_vn`) were wrong, one would vanish.
/// Both must survive, proving the grouping keys on source-Varnode identity.
#[test]
fn w10_dc_condconst_conn_distinct_sources_both_survive() {
    let Some(block) = rust_print_c_for("condconst.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "condconst_conn")
        .unwrap_or_else(|| panic!("condconst_conn not in dump:\n{block}"));
    let has_param = body.contains("v1 = x;");
    let has_const = body.contains("v1 = 0x14;");
    assert!(
        has_param && has_const,
        "distinct-source assignments to the merged local must both survive the per-source \
         grouping (param `v1 = x;` AND const `v1 = 0x14;`), got param={has_param} const={has_const}:\n{body}"
    );
}
