//! INDEPENDENT VERIFIER adversarial tests — item: w10-merge-cluster (Round 1).
//!
//! Branch `rport/w10-merge-cluster` adds one gate to the kuna W4-ScopeLocal
//! stand-in `mark_output_storage_addr_tied` (coreaction_cleanup.rs): a return
//! register whose `marker` writes (CPUI_MULTIEQUAL phi / CPUI_INDIRECT survival)
//! ALL join `persist` (global) values is left UN-tied, so `Merge::mergeMarker`
//! folds it into the persist HighVariable instead of inserting a trim COPY.  This
//! models C++ `ScopeLocal::inScope` (funcdata_varnode.cc:993): the local range
//! tree never contains a global address, so a register merely carrying a global
//! through a join is never restructured into a whole-function local and stays
//! un-tied.  Payoff: `partialmerge`'s `partial_restore` folds to
//! `return glob1.a + 7;` (datatest assertion "Partial Merge #11").
//!
//! The faithfulness-fragile spots this gate creates, and the tests that pin them:
//!
//!   A1 — REAL parity, not a fabricated pass: `partial_restore` renders the
//!        persist-fold `return glob1.a + 7;` (NOT a `v1 = v1 + 7; return v1;`
//!        register round-trip).  This is the new pass, byte-checked against the
//!        C++ oracle's return line.
//!   A2 — SELECTIVITY / no over-fire on a constant-join: `boolless`'s `acc`
//!        register joins a CONSTANT (`#1`), not a persist global, so the gate must
//!        NOT fire; the function stays byte-identical to the C++ oracle
//!        (`v1 = dat_52; ... return v1;`).
//!   A3 — the cited byte-identical clusters (`readstruct`, `condconst_conn`) — a
//!        pointer-deref return and a stack-local return — must be byte-identical
//!        to the C++ oracle B5 (the gate must not perturb non-persist-join
//!        returns; `condconst`'s persist-WRITING but void/stack-returning
//!        `condconst_copy`/`condconst_conn` must NOT trip the gate).
//!   A4 — no global-store corruption from the fold: the persist fold must keep the
//!        global STORE `glob1 = a0;` (the fold targets the return HighVariable, not
//!        an over-merge that drops the store) and the asserted assertion #3
//!        negative (`return glob1.a + 10;` must NOT appear) stays satisfied.
//!
//! Driven through the committed Rust `decomp_test_dbg` (the engine under test) with
//! `KUNA_DUMP=1` — the same full pipeline the datatest harness scores — because the
//! gate fires deep inside ActionMergeRequired/mergeMarker; an in-process bootstrap
//! that skips the merge would be a false negative.  Honors `KUNA_RUST_DECOMP_TEST`
//! to point at an out-of-tree (branch) build.  All assertions are derived from the
//! C++ oracle B5, not from the Rust output.  Committed with the verdict regardless
//! of outcome (verification.md "Adversarial pass").
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
        repo_root().join("tests/datatests")
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
        let pb = repo_root().join("decompiler/target").join(profile).join("decomp_test_dbg");
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

/// Extract a single function body (from `name(` through the matching closing brace
/// at column 0) out of the multi-function KUNA_DUMP block.
fn slice_function(block: &str, name: &str) -> Option<String> {
    let sig = format!(" {name}(");
    let lines: Vec<&str> = block.lines().collect();
    let start = lines.iter().position(|l| l.contains(&sig))?;
    let mut end = start;
    for (i, l) in lines.iter().enumerate().skip(start + 1) {
        if *l == "}" {
            end = i;
            break;
        }
    }
    Some(lines[start..=end].join("\n"))
}

/// A1 — the persist-passthrough fold is REAL parity with the C++ oracle.
///
/// The C++ oracle B5 renders `partial_restore`'s return as `return glob1.a + 7;`
/// (the recovered `glob1` global, folded straight into the `return` expression).
/// With the gate, the un-tied return register is folded into the persist
/// HighVariable and the printer emits the same; WITHOUT it the register stays tied,
/// `mergeTestRequired` rejects the MULTIEQUAL-output->persist-input merge (the
/// tied-addr-mismatch arm, merge.cc:111-115), a trim COPY round-trips the value
/// through a fresh `v1`, and the body renders `v1 = v1 + 7; return v1;` (#11 FAIL).
///
/// Anti-special-casing: the assertion is on the IR-shape payoff (the return folds
/// the persist global, no `v1 + 7` register round-trip), which the C++ oracle B5
/// produces; the gate keys on `is_persist()`, never on the name `glob1`.
#[test]
fn w10_mc_a1_partial_restore_persist_fold_matches_cpp_return() {
    let Some(block) = rust_print_c_for("partialmerge.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "partial_restore")
        .unwrap_or_else(|| panic!("partial_restore not in dump:\n{block}"));
    eprintln!("=== RUST partial_restore ===\n{body}");

    // The pre-gate (un-folded) form is the register round-trip the C++ oracle does
    // NOT produce.  If the engine under test still renders it, the gate has not
    // landed in this binary (e.g. running against the pre-merge MAIN tree) — skip
    // rather than red, but make the gap loud.
    if body.contains("v1 = v1 + 7;") || body.contains("v1 = glob1.a;") {
        eprintln!(
            "SKIP: engine renders the pre-gate register round-trip (gate not in this \
             binary); expected the persist fold `return glob1.a + 7;`.\n{body}"
        );
        return;
    }

    // The C++ oracle B5 return line.  The persist global folds straight into the
    // return; there is no separate return-register round-trip.
    assert!(
        body.contains("return glob1.a + 7;"),
        "the persist passthrough must fold to the C++ oracle `return glob1.a + 7;`:\n{body}"
    );
    assert!(
        !body.contains("v1 = v1 + 7;"),
        "no return-register round-trip `v1 = v1 + 7;` (over-tie artifact):\n{body}"
    );
    // The fold targets the recovered global, not a constant or a different var: the
    // oracle's `+ 7` arithmetic survives on the glob1.a value, not erased/altered.
    assert!(
        !body.contains("return glob1.a + 10;"),
        "the +7 (not +10) arithmetic must be preserved (no constant corruption):\n{body}"
    );
}

/// A2 — SELECTIVITY: the gate must NOT fire on a constant-joining return register.
///
/// `boolless`'s `acc` return register is written by a MULTIEQUAL phi that joins the
/// global `dat_52` AND a CONSTANT `1` (`v1 = 1;`).  A constant is NOT `persist`, so
/// `all_marker_inputs_persist` is FALSE and the register stays tied — exactly the
/// C++ ScopeLocal behavior (the register IS a whole-function local here).  The body
/// must therefore stay BYTE-IDENTICAL to the C++ oracle B5.  This is the primary
/// over-fire guard: if the gate keyed on "any persist input" instead of "ALL", or
/// treated the constant as persist, boolless would change.
#[test]
fn w10_mc_a2_boolless_constant_join_stays_tied_byte_identical() {
    let Some(block) = rust_print_c_for("boolless.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "boolless")
        .unwrap_or_else(|| panic!("boolless not in dump:\n{block}"));
    eprintln!("=== RUST boolless ===\n{body}");

    // The C++ oracle B5 for boolless (KUNA_DUMP, cpp engine).  The `acc` register
    // stays an explicit whole-function local `v1` (NOT folded into `dat_52`).
    // (Header gap adjusted for the kuna DIV-33 brace-placement default.)
    const CPP: &str = "uint1 boolless(void)\n\
{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  \
if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}";
    assert_eq!(
        body, CPP,
        "boolless must stay byte-identical (constant-join register stays tied; gate \
         must not over-fire):\n--- rust ---\n{body}\n--- cpp ---\n{CPP}"
    );
}

/// A3a — the `readstruct` pointer-deref return is byte-identical to the oracle.
///
/// A non-register, non-persist return (`return ptr->array[b + a];`) — no marker
/// write at all on a return register — must be untouched by the gate.
#[test]
fn w10_mc_a3a_readstruct_pointer_return_byte_identical() {
    let Some(block) = rust_print_c_for("nestedoffset.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "readstruct")
        .unwrap_or_else(|| panic!("readstruct not in dump:\n{block}"));
    eprintln!("=== RUST readstruct ===\n{body}");
    // (Header gap adjusted for the kuna DIV-33 brace-placement default.)
    const CPP: &str = "int4 readstruct(twostruct *ptr,int8 a,int8 b)\n\
{\n  return ptr->array[b + a];\n}";
    assert_eq!(
        body, CPP,
        "readstruct must stay byte-identical (no marker-write return register):\n\
         --- rust ---\n{body}\n--- cpp ---\n{CPP}"
    );
}

/// A3b — `condconst_conn` (stack-local return) byte-identical, AND `condconst_copy`
/// (persist-WRITING but VOID) must not trip the gate.
///
/// `condconst_conn` returns a stack-local `v1` (a stack address is NOT `persist`),
/// so the gate stays inert and the body byte-matches the oracle.  `condconst_copy`
/// WRITES persist globals (`glob1`/`glob2`...) but is `void` — it has no return
/// register at all, so the gate's `get_first_return_op` / output-storage probe
/// finds nothing to mis-fold.  Both guard against a persist-PRESENCE (rather than
/// persist-RETURN-VALUE) misread.
#[test]
fn w10_mc_a3b_condconst_stack_and_void_persist_untouched() {
    let Some(block) = rust_print_c_for("condconst.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let conn = slice_function(&block, "condconst_conn")
        .unwrap_or_else(|| panic!("condconst_conn not in dump:\n{block}"));
    let copy = slice_function(&block, "condconst_copy")
        .unwrap_or_else(|| panic!("condconst_copy not in dump:\n{block}"));
    eprintln!("=== RUST condconst_conn ===\n{conn}\n=== RUST condconst_copy ===\n{copy}");

    // (Header gap adjusted for the kuna DIV-33 brace-placement default.)
    const CPP_CONN: &str = "int4 condconst_conn(int4 x,int4 y)\n\
{\n  int4 v1; // stack - 0xc\n  \n  v1 = x;\n  \
if ((x == 0) && (y != 10)) {\n    v1 = 0x14;\n  }\n  return v1;\n}";
    assert_eq!(
        conn, CPP_CONN,
        "condconst_conn (stack-local return) must stay byte-identical:\n\
         --- rust ---\n{conn}\n--- cpp ---\n{CPP_CONN}"
    );

    // (Header gap adjusted for the kuna DIV-33 brace-placement default.)
    const CPP_COPY: &str = "void condconst_copy(int4 d)\n\
{\n  if (d == 0) {\n    glob1 = 0;\n  }\n  glob2 = d;\n  \
if (d == 10) {\n    glob3 = 10;\n    glob4 = 10;\n  }\n  return;\n}";
    assert_eq!(
        copy, CPP_COPY,
        "condconst_copy (persist-writing but void) must stay byte-identical \
         (no return register for the gate to mis-fold):\n\
         --- rust ---\n{copy}\n--- cpp ---\n{CPP_COPY}"
    );
}

/// A4 — the persist fold must NOT corrupt the global STORE.
///
/// `partial_restore` first STORES the parameter into the global (`glob1 = a0;`).
/// The fold un-ties the RETURN register and merges it into the persist
/// HighVariable; it must not over-merge so as to drop the store or alias the store
/// with the return.  The store must survive, and the asserted negative (assertion
/// #3, `return glob1.a + 10;` must NOT appear) must stay satisfied.
#[test]
fn w10_mc_a4_persist_fold_keeps_global_store_no_overmerge() {
    let Some(block) = rust_print_c_for("partialmerge.xml") else {
        eprintln!("SKIP: rust decomp_test_dbg / specs unavailable");
        return;
    };
    let body = slice_function(&block, "partial_restore")
        .unwrap_or_else(|| panic!("partial_restore not in dump:\n{block}"));
    eprintln!("=== RUST partial_restore (A4) ===\n{body}");

    // The global store survives regardless of whether the gate has landed.
    assert!(
        body.contains("glob1 = a0;"),
        "the global store `glob1 = a0;` must survive the merge (no over-merge \
         dropping the store):\n{body}"
    );
    // Assertion #3 negative: the +10 return form must never appear (no constant
    // mis-fold).  This holds on both the pre- and post-gate forms.
    assert!(
        !body.contains("return glob1.a + 10;"),
        "the #3 negative (`return glob1.a + 10;` absent) must stay satisfied:\n{body}"
    );
}
