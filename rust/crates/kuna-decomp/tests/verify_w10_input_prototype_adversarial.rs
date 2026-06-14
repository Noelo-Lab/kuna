//! VERIFIER adversarial tests for item `w10-input-prototype`
//! (`ActionInputPrototype` input-param recovery + typing, the type-plane seed).
//!
//! These drive the *real* gate path: the committed Rust `decomp_test_dbg`
//! harness on a datatest whose `<script>` does `parse line extern <decl>` +
//! `decompile` + `print C`, then compares the bulk `print C` against the C++
//! oracle binary on the same datatest.  They pin two things the diff changed:
//!
//!   1. POSITIVE: the input-prototype recovery genuinely renders a typed,
//!      *named* parameter list (`namespace.xml`: `int4 spam`), matching the
//!      oracle's signature — the params are REAL (flowed through the proto
//!      model's `assign_parameter_storage`), not hardcoded.
//!
//!   2. DIVERGENCE (verdict F2 / LOSS): newly applying the locked prototype on
//!      `condconst2.xml` forces typed input/output Varnodes whose SUBPIECE
//!      truncation (`SUB(r0,0)`) is NOT yet cleaned up by the un-ported
//!      downstream type/cast actions, so the body leaks `SUB`.  The C++ oracle
//!      emits NO `SUB` (it passes the datatest's `SUB`-absent assertion #8); the
//!      Rust branch regresses that one assertion.  This test pins the leak so a
//!      future downstream-action port removes it deliberately.
//!
//! Both are skipped (not failed) when a binary or `.sla` is unavailable, so the
//! suite stays green in a worktree without a built C++/spec tree (honoring the
//! same `KUNA_*` overrides as `printc_parity.rs`).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    std::env::var_os("KUNA_SPECS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("specs"))
}

fn datatests_dir() -> PathBuf {
    std::env::var_os("KUNA_DATATESTS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/datatests"))
}

fn rust_test_bin() -> PathBuf {
    // This crate's own workspace target dir (the branch build under test).
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/decomp_test_dbg")
}

fn cpp_oracle_bin() -> PathBuf {
    std::env::var_os("KUNA_DECOMP_TEST")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/cpp/decomp_test_dbg"))
}

/// Drive a `decomp_test_dbg` binary on one datatest with `KUNA_DUMP=1` and
/// return the first dumped `print C` block (the bulk output).  `None` if the
/// binary is missing or produced no dump (no spec/`.sla`).
fn dump_print_c(bin: &PathBuf, stem: &str) -> Option<String> {
    if !bin.exists() {
        return None;
    }
    let out = Command::new(bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_dir())
        .arg("datatests")
        .arg(format!("{stem}.xml"))
        .env("KUNA_DUMP", "1")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    Some(text[after_begin..mid].to_string())
}

/// POSITIVE: the recovered parameter list is real and named — `namespace.xml`'s
/// `a::b::assign_vals` gains an `int4 spam` parameter (from `parse line extern
/// ... assign_vals(int4 spam)`), matching the oracle's signature.  Before this
/// item the Rust path emitted `...assign_vals(void)`.
#[test]
fn namespace_recovers_named_typed_param_matching_oracle() {
    let rust = match dump_print_c(&rust_test_bin(), "namespace") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    // The recovered, NAMED, typed parameter must appear in the Rust signature.
    assert!(
        rust.contains("assign_vals(int4 spam)"),
        "input-prototype recovery did not render the typed/named param; got:\n{rust}"
    );
    // And it must NOT be the pre-item unrecovered `(void)` form.
    assert!(
        !rust.contains("assign_vals(void)"),
        "signature is still the unrecovered (void) form:\n{rust}"
    );

    // If the oracle is available, the signature LINE must match it byte-for-byte
    // (the param recovery is faithful, not an approximation of the text).
    if let Some(cpp) = dump_print_c(&cpp_oracle_bin(), "namespace") {
        let rust_sig = rust.lines().find(|l| l.contains("assign_vals"));
        let cpp_sig = cpp.lines().find(|l| l.contains("assign_vals"));
        if let (Some(r), Some(c)) = (rust_sig, cpp_sig) {
            assert_eq!(r.trim(), c.trim(), "recovered signature diverges from oracle");
        }
    }
}

/// DIVERGENCE (F2 / LOSS): applying the locked prototype on `condconst2.xml`
/// makes the typed signature appear (good, matches the oracle), but the forced
/// typed output Varnode leaks an un-cleaned `SUB(r0,0)` into the body — which
/// the oracle does not.  This is the cause of the lost `Immediate Conditional
/// #8` (`SUB`-absent) assertion.  Pin both halves.
#[test]
fn condconst2_typed_sig_but_leaks_sub_unlike_oracle() {
    let rust = match dump_print_c(&rust_test_bin(), "condconst2") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    // The typed signature IS now produced (the recovery seed works).
    assert!(
        rust.contains("char zeroprop(int4 *ptrint,int4 val)"),
        "locked prototype not applied to zeroprop signature; got:\n{rust}"
    );
    // DIVERGENCE: the body leaks a SUBPIECE the oracle eliminates.  This is the
    // regression of datatest assertion #8 (`SUB` must be absent).
    assert!(
        rust.contains("SUB("),
        "expected the (currently un-cleaned) SUB leak in the body; got:\n{rust}"
    );

    // Confirm the oracle does NOT leak SUB (so this is a real divergence, not a
    // shared limitation) when the oracle is available.
    if let Some(cpp) = dump_print_c(&cpp_oracle_bin(), "condconst2") {
        assert!(
            !cpp.contains("SUB("),
            "oracle unexpectedly also leaks SUB; the divergence premise is wrong:\n{cpp}"
        );
        // And the oracle's signature is the same typed form — so the SIGNATURE
        // recovery is faithful; only the body cleanup diverges.
        assert!(cpp.contains("char zeroprop(int4 *ptrint,int4 val)"));
    }
}
