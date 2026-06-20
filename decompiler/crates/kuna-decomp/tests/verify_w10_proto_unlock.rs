//! VERIFIER adversarial tests for item `w10-proto-unlock`.
//!
//! This branch delivers two coupled changes (independent VERIFIER review,
//! round 2):
//!
//!   1. `Architecture::decode_join_addr` + the `IPTR_JOIN` dispatch in
//!      `decode_pentry_storage` (`architecture.rs`) — a faithful port of
//!      `JoinSpace::decodeAttributes` (space.cc:539) reached through the C++
//!      `VarnodeData::decodeFromAttributes` space dispatch (pcoderaw.cc:33).
//!      It resolves the x86 struct-return output pentry
//!      `<addr space="join" piece1="EDX" piece2="EAX"/>` to a real unified
//!      offset instead of the offset-0 fallback that made the default proto
//!      model build empty.
//!
//!   2. The `all_writes_const_copy` over-tie guard in
//!      `mark_output_storage_addr_tied` (`coreaction_cleanup.rs`) — a return
//!      register written ONLY by `CPUI_COPY`s of a constant (the `EAX = #0` /
//!      `EAX = #1` constant returns) is left un-tied, so the printer collapses
//!      `EAX = #N; return EAX;` to `return N;` exactly as C++ `inScope` does,
//!      and the overlapping AL `(uint1)` promotion cast is not dropped into a
//!      tied EAX group (Promotion on compare #2).
//!
//! These are END-TO-END tests: they drive the committed Rust `decomp_test_dbg`
//! on the real datatests, slice the `KUNA_DUMP` `print C` block, and where the
//! built C++ oracle + `.sla` are present, run the SAME datatest through the C++
//! `decomp_test_dbg` for a direction check (the recovered shape must move
//! TOWARD the oracle, never special-cased away from it).  They SKIP (never
//! fail) when a binary or `.sla` is unavailable so the suite stays green in a
//! bare worktree.
//!
//! The hunt-list spots these target (most fragile for this diff):
//!   A. the join-pentry `space:offset:size` triple / `pieceN`->index decode is
//!      REAL — the EDX:EAX struct-return output recovers a non-void `foo`
//!      return type (was `void` while the pentry decoded to offset 0).
//!   B. the const-return collapse fires on the all-const-COPY shape
//!      (promotecompare's `return 0;` / `return 1;`, NO `EAX = 0; return EAX;`
//!      round-trip) AND matches the oracle direction.
//!   C. the over-tie guard does NOT spuriously un-tie a register written by a
//!      NON-constant value: `partialmerge`'s already-passing `Partial Merge #3`
//!      rendering (`readpartial`'s non-constant return) is preserved.
//!   D. determinism: the join decode + un-tie are stable run-to-run.

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
        .unwrap_or_else(|| repo_root().join("tests/datatests"))
}

fn rust_test_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/decomp_test_dbg")
}

fn cpp_oracle_bin() -> PathBuf {
    std::env::var_os("KUNA_DECOMP_TEST")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/cpp/decomp_test_dbg"))
}

/// Drive a `decomp_test_dbg` binary on one datatest with `KUNA_DUMP=1` and
/// return the first dumped `print C` block (header line stripped).  `None` if
/// the binary is missing or produced no dump (no spec/`.sla`).
fn dump_body(bin: &PathBuf, stem: &str) -> Option<String> {
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

/// (A) JOIN-PENTRY DECODE IS REAL.  The `retstruct` datatest's output pentry is
/// `<addr space="join" piece1="EDX" piece2="EAX"/>`.  If `decode_join_addr`
/// were absent (the offset-0 fallback the diff replaced), the default proto
/// model would build empty and the function would recover as `void
/// retstruct(void)`.  With the join decode it recovers the oracle's `foo
/// retstruct(int4 x,int4 y,int4 z)` and the join-dependent body assertion
/// `y = 0x1e;` materializes.  This is the new datatest pass ("Return Structure
/// #3") proven REAL by inspection — not a weakened assertion.
#[test]
fn w10_proto_unlock_join_struct_return_is_real_not_void() {
    let a = match dump_body(&rust_test_bin(), "retstruct") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable (retstruct)");
            return;
        }
    };
    // The join pentry decoded -> a non-void struct return with the three int4
    // params (the empty-model fallback would be `void retstruct(void)`).
    assert!(
        a.contains("foo retstruct(int4 x,int4 y,int4 z)"),
        "retstruct lost its join-recovered struct-return signature \
         (decode_join_addr regression -> empty proto model?):\n{a}"
    );
    assert!(
        !a.contains("retstruct(void)"),
        "retstruct degenerated to a void proto (join pentry decoded to the \
         offset-0 fallback):\n{a}"
    );
    // The join-dependent body assertion that newly passes.
    assert!(
        a.contains("y = 0x1e;"),
        "retstruct lost the join-recovered `y = 0x1e;` (Return Structure #3):\n{a}"
    );

    // Direction check against the C++ oracle on the SAME datatest: the recovered
    // signature must match the oracle's, not be special-cased.
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "retstruct") {
        let csig = cpp.lines().find(|l| l.contains("retstruct(")).unwrap_or("");
        assert!(
            csig.contains("foo retstruct(int4 x,int4 y,int4 z)"),
            "oracle retstruct signature drifted from the pinned shape: {csig}"
        );
        assert!(
            cpp.contains("y = 0x1e;"),
            "oracle retstruct lost `y = 0x1e;` — the Rust pass would be fabricated:\n{cpp}"
        );
    }
}

/// (B) CONST-RETURN COLLAPSE FIRES (the over-tie fix).  `promote_compare`'s
/// return register EAX is written only by `EAX = #0` / `EAX = #1`.  With the
/// `all_writes_const_copy` guard it stays un-tied and the printer collapses the
/// round-trip to `return 0;` / `return 1;`.  Without the guard EAX is tied,
/// renders an explicit `EAX = 0; return EAX;` (a `uVarN = 0; ... return uVarN;`
/// shape), and the overlapping AL `(uint1)` promotion cast is dropped.  We
/// assert BOTH the literal returns appear and NO tied-register round-trip
/// remains, and that the recovered return type matches the oracle.
#[test]
fn w10_proto_unlock_const_return_collapses_no_tied_roundtrip() {
    let a = match dump_body(&rust_test_bin(), "promotecompare") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable (promotecompare)");
            return;
        }
    };
    // The join pentry decoded -> a non-empty model with a `char *` parameter and
    // the oracle's `xunknown4` return type (was `void promote_compare(void)`
    // while the join pentry failed).
    assert!(
        a.contains("xunknown4 promote_compare(char *"),
        "promote_compare lost its join-recovered signature:\n{a}"
    );
    // The constant returns collapsed (the over-tie fix): both literal returns
    // are present.
    assert!(
        a.contains("return 0") && a.contains("return 1"),
        "promote_compare lost a collapsed constant return (over-tie fix \
         regression?):\n{a}"
    );
    // And NO tied-EAX round-trip survives: there must be no `<var> = 0;` line
    // whose variable is then returned (the round-trip the un-tie collapses).
    // We check there is no assignment of a bare `0`/`1` constant to a local that
    // is the return value — i.e. no `= 0;` line co-occurring with `return <that
    // local>;`.  The collapsed form has the constant inline in `return`.
    let has_const_assign_to_eax = a.lines().any(|l| {
        let t = l.trim();
        // an explicit `<ident> = 0;` / `<ident> = 1;` constant assignment whose
        // ident is NOT a struct/array store (no `.`/`[`), i.e. a register copy.
        (t.ends_with("= 0;") || t.ends_with("= 1;"))
            && !t.contains('.')
            && !t.contains('[')
            && t.split('=').next().map(|lhs| lhs.trim().split_whitespace().count() == 1).unwrap_or(false)
    });
    assert!(
        !has_const_assign_to_eax,
        "promote_compare still emits a tied-register constant round-trip \
         (`reg = N;` then `return reg;`) — the all_writes_const_copy un-tie did \
         not fire:\n{a}"
    );

    // Direction check: the oracle also collapses to literal `return 0;` /
    // `return 1;` and has the `xunknown4` return type.
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "promotecompare") {
        assert!(
            cpp.contains("xunknown4 promote_compare(char *"),
            "oracle promote_compare signature drifted:\n{cpp}"
        );
        assert!(
            cpp.contains("return 0;") && cpp.contains("return 1;"),
            "oracle promote_compare does not collapse the constant returns — \
             the Rust collapse would diverge from it:\n{cpp}"
        );
    }
}

/// (C) THE GUARD IS NARROW — a NON-constant return register stays handled by the
/// marker/output_locked arm.  `partialmerge`'s `readpartial` returns
/// `glob1.a + 10` (a non-constant COPY source), and its already-passing
/// rendering ("Partial Merge #3" => `return uVar + 10;` shape) must be
/// preserved: the `all_writes_const_copy` predicate is all-or-nothing over the
/// writes, so a register whose def is `COPY(non-constant)` is NOT un-tied here.
/// This pins that the over-tie fix did not over-reach into the non-constant
/// path.
#[test]
fn w10_proto_unlock_nonconst_return_register_unaffected() {
    let a = match dump_body(&rust_test_bin(), "partialmerge") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable (partialmerge)");
            return;
        }
    };
    // The Partial Merge #3 assertion is `readpartial`'s non-constant return
    // expression `<expr> + 10`.  It must still render (the guard did not un-tie
    // a non-constant register and perturb the merge).
    assert!(
        a.contains("+ 10;"),
        "partialmerge lost the non-constant `... + 10;` return (Partial Merge \
         #3) — the over-tie guard over-reached into a non-constant register:\n{a}"
    );
    // Direction check: the oracle renders the same non-constant return.
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "partialmerge") {
        assert!(
            cpp.contains("+ 10;"),
            "oracle partialmerge lost `... + 10;` — pin drift:\n{cpp}"
        );
    }
}

/// (D) DETERMINISM.  The join decode (`find_add_join` allocates a unified
/// offset off a per-architecture allocator) and the un-tie must be stable
/// run-to-run; an order- or hash-dependent join offset would surface as a
/// nondeterministic body.  We dump twice and require byte-identical output for
/// both the join-pentry function and the const-return function.
#[test]
fn w10_proto_unlock_join_and_untie_are_deterministic() {
    for stem in ["retstruct", "promotecompare"] {
        let a = match dump_body(&rust_test_bin(), stem) {
            Some(t) => t,
            None => {
                eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable ({stem})");
                return;
            }
        };
        let b = dump_body(&rust_test_bin(), stem).expect("second dump");
        assert_eq!(a, b, "{stem} body is nondeterministic across runs (join/un-tie order leak)");
    }
}
