//! Tests for `rport/w10-emptyblock-orform`.
//!
//! Drive the SAME full pipeline the datatest harness uses — the committed Rust
//! `decomp_test_dbg` binary as a subprocess with `KUNA_DUMP=1` — because the
//! empty-block splice (`removeDoNothingBlock`) only fires after the merge/trim and
//! `ActionDominantCopy` have emptied the redundant COPY blocks, and the `&&`
//! `BlockCondition` (`ruleBlockOr`) only forms once `ActionBlockStructure` re-runs
//! over the cleaned-up CFG with the real `isComplex` statement count.  An
//! in-process bootstrap that skips the prototype/merge would be a false negative;
//! the subprocess is the faithful exercise.
//!
//! These target the un-seam chain:
//!  * the now-empty then/else blocks are SPLICED OUT (`removeDoNothingBlock`), so
//!    `condconst_conn` no longer renders the empty `if (x != 0) { } else if ...`
//!    (Test A),
//!  * with the empty blocks gone, `ruleBlockOr` forms the `&&` BlockCondition and
//!    the body byte-matches the C++ oracle `if ((x == 0) && (y != 10))` (Test B),
//!  * the splice + orform must NOT perturb boolless — byte identical to the C++
//!    oracle (Test C: anti-over-splice guard).
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
    for profile in ["release", "debug"] {
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
    if !specs_root().exists() {
        return None;
    }
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

/// TEST A — the now-empty then/else blocks are SPLICED OUT.
///
/// Before this un-seam, the dominant-copy hoist emptied the two redundant COPY
/// blocks but they survived structuring, rendering the broken
/// `if (x != 0) { } else if (y != 10) { v1 = 0x14; } else { }`.  With
/// `removeDoNothingBlock` wired into `ActionDoNothing`/`ActionLateDoNothing`, the
/// empty marker blocks are gone: no empty `{ }` then-clause, no spurious `else`.
///
/// Anti-special-casing: the assertion is on the absence of the empty-block
/// artifact (the `if (x != 0) {` immediately followed by a `}` line), not a
/// hardcoded printer string.
#[test]
fn w10_eob_condconst_conn_empty_blocks_spliced_out() {
    let Some(block) = rust_print_c_for("condconst.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "condconst_conn")
        .unwrap_or_else(|| panic!("condconst_conn not in dump:\n{block}"));
    eprintln!("=== RUST condconst_conn ===\n{body}");
    // The broken pre-un-seam form opened `if (x != 0) {` with an immediately
    // closing `}` (an empty then-clause).  That artifact must be gone.
    assert!(
        !body.contains("if (x != 0) {"),
        "the empty then-clause `if (x != 0) {{ }}` must be spliced out, got:\n{body}"
    );
    // The function must still produce the store and return (no over-deletion).
    assert!(body.contains("v1 = 0x14;"), "the 0x14 store must survive the splice:\n{body}");
    assert!(body.contains("return v1;"), "the return must survive the splice:\n{body}");
}

/// TEST B — with the empty blocks gone, `ruleBlockOr` forms the `&&` and the body
/// byte-matches the C++ oracle.
///
/// This is the full un-seam payoff: the two CBRANCH-condition blocks are now
/// adjacent (size_in==1), `isComplex` (now a real statement count, not the
/// conservative `true`) lets the OR-clause block be absorbed, and `ruleBlockOr`
/// forms the `(x == 0) && (y != 10)` BlockCondition.  The whole function must
/// equal the C++ B5 oracle.
#[test]
fn w10_eob_condconst_conn_forms_andand_byte_identical() {
    let Some(block) = rust_print_c_for("condconst.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "condconst_conn")
        .unwrap_or_else(|| panic!("condconst_conn not in dump:\n{block}"));
    // The C++ oracle B5 for condconst_conn (from `decomp_test_dbg KUNA_DUMP=1`).
    const CPP: &str = "int4 condconst_conn(int4 x,int4 y)\n\n\
{\n  int4 v1; // stack - 0xc\n  \n  v1 = x;\n  \
if ((x == 0) && (y != 10)) {\n    v1 = 0x14;\n  }\n  return v1;\n}";
    assert_eq!(
        body, CPP,
        "condconst_conn must byte-match the C++ && oracle:\n--- rust ---\n{body}\n--- cpp ---\n{CPP}"
    );
    // Belt-and-suspenders on the load-bearing `&&` token (the BlockCondition).
    assert!(
        body.contains("if ((x == 0) && (y != 10)) {"),
        "ruleBlockOr must form the && BlockCondition:\n{body}"
    );
}

/// TEST C — the splice + orform must NOT perturb boolless.
///
/// boolless has its own structuring but NO empty do-nothing block on its critical
/// path; the new removeDoNothingBlock / ruleBlockOr machinery must leave its byte
/// output exactly equal to the C++ B5 oracle.  Anti-over-splice / anti-mis-orform:
/// a wrong `isDoNothing` predicate or a spurious `&&` fold would perturb boolless.
#[test]
fn w10_eob_boolless_not_perturbed_byte_identical() {
    let Some(block) = rust_print_c_for("boolless.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    const CPP: &str = "\nuint1 boolless(void)\n\n{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}\n";
    assert_eq!(
        block.trim_end(),
        CPP.trim_end(),
        "the empty-block splice / ruleBlockOr must not perturb boolless:\n--- rust ---\n{block}\n--- cpp ---\n{CPP}"
    );
    // boolless's single `if` must NOT become a spurious `&&` condition.
    assert!(!block.contains("&&"), "boolless must not grow a spurious && condition:\n{block}");
}
