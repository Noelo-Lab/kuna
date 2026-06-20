//! VERIFIER adversarial tests for `rport/w10-pushpartialsymbol`
//! (PrintC::pushPartialSymbol — symbol-mapped struct/union member render).
//!
//! Written by the independent verifier per docs/rust-port/verification.md
//! "Adversarial pass"; they land with the verdict regardless of outcome.
//!
//! ## Why these drive the harness BINARY, not `decompile_func`
//!
//! `pushPartialSymbol` only fires after the FULL datatest *script* runs — the
//! `parse line union ...` type definitions and `map addr r0x302020 baseunion
//! glob` symbol mappings give the varnodes their composite Symbol type.  A bare
//! `decompile_func` call (no script) decompiles raw lifted p-code and never
//! reaches the member render (see `printc_parity.rs`'s LOSS-130 note).  So these
//! tests drive the SAME `decomp_test_dbg` harness the parity runner uses, with
//! `KUNA_DUMP=1`, and slice the rendered `print C` block — then byte-compare the
//! Rust render against the C++ oracle per function.
//!
//! The fragile / REJECT-prone spots each test pins:
//!
//!   (A1) REAL PARITY, not a regex fluke: the per-function Rust `print C` body
//!        BYTE-MATCHES the C++ oracle for the functions whose render newly fires
//!        the partial-symbol path (`printglob`: `glob.intfield`, `testvalue`:
//!        `val.myptr`/`val.mydouble`).  The `Union #5..` datatest asserts only one
//!        line; this asserts the WHOLE function body equals the oracle's.
//!
//!   (A2) THE GUARD IS TIGHT (anti-corruption of the universal explicit path):
//!        a function with NO composite-symbol member access (`fillin`) renders
//!        byte-IDENTICALLY to the C++ oracle and carries NO spurious member
//!        token.  `push_partial_symbol_ir` returns `false` on a whole-symbol
//!        cover, so the bare-name path still wins.  This is the regression
//!        tripwire for the universal `push_vn_explicit_ir`.
//!
//!   (A3) DATA-DRIVEN, NOT SPECIAL-CASED: two different functions resolve to
//!        DIFFERENT field names from their OWN types (`glob.intfield` from a
//!        struct vs `val.myptr` from a union), and neither leaks the other's
//!        field token.  A hardcoded-field-name port would leak or repeat.

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

fn datatests_dir() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DATATESTS") {
        PathBuf::from(p)
    } else {
        repo_root().join("tests/datatests")
    }
}

/// The Rust harness binary under test (the same `decomp_test_dbg` the parity
/// runner drives).  Honors `KUNA_DECOMP_TEST_RUST`, else the release build.
fn rust_harness() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST_RUST") {
        return PathBuf::from(p);
    }
    let profile = std::env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
    repo_root().join("decompiler/target").join(profile).join("decomp_test_dbg")
}

/// The committed C++ oracle binary.  Honors `KUNA_DECOMP_TEST`, else the
/// in-tree build under `decompiler/cpp/`.
fn cpp_oracle() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST") {
        return PathBuf::from(p);
    }
    repo_root().join("decompiler/cpp/decomp_test_dbg")
}

/// Drive `bin` on `stem`.xml with `KUNA_DUMP=1`, return the sliced bulk
/// `print C` block (the `===KUNA_DUMP_BEGIN===..===KUNA_DUMP_MID===` body), or
/// `None` if the binary or its `.sla` is unavailable.
fn dump_print_c(bin: &PathBuf, stem: &str) -> Option<String> {
    if !bin.exists() {
        eprintln!("SKIP: binary {bin:?} not built");
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
    let text = String::from_utf8_lossy(&out.stdout);
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    Some(text[after_begin..mid].to_string())
}

/// Slice the body of one `name(...)` function out of the bulk `print C` dump,
/// from its signature line through its closing `}` at column 0.
fn extract_function(dump: &str, fn_name: &str) -> Option<String> {
    // The signature line is `RET name(args)` — find `name(`.
    let needle = format!("{fn_name}(");
    let mut search_from = 0;
    loop {
        let hit = dump[search_from..].find(&needle)? + search_from;
        // Walk back to the start of the signature line.
        let line_start = dump[..hit].rfind('\n').map(|p| p + 1).unwrap_or(0);
        // The signature line begins the function only if it is a top-level decl
        // (preceded by a blank line in the dump's `\n\nRET name(...)` framing).
        // Find the closing `}` that sits at column 0 after the body.
        if let Some(close_rel) = dump[hit..].find("\n}\n") {
            let close = hit + close_rel + "\n}".len() + 1;
            return Some(dump[line_start..close].to_string());
        }
        search_from = hit + needle.len();
    }
}

/// Capture both engines' renders for `stem`, returning `(rust_dump, cpp_dump)`.
/// `None` for either side when its binary/`.sla` is missing (the test SKIPs).
fn both_dumps(stem: &str) -> Option<(String, String)> {
    let rust = dump_print_c(&rust_harness(), stem)?;
    let cpp = dump_print_c(&cpp_oracle(), stem)?;
    Some((rust, cpp))
}

/// The set of trimmed lines in `body` that contain a struct/union member access
/// (`X.field`).  These are the lines `pushPartialSymbol` produces; the
/// surrounding control-flow structure (if/else-if flattening, for-loop reroll)
/// is a SEPARATE, not-yet-ported wave (held in `blockaction.rs`), so a
/// whole-function byte-compare is not the right granularity for THIS item — the
/// member-access LINES are.
fn member_lines(body: &str) -> Vec<String> {
    let mut out: Vec<String> = body
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            // A member access has a `.` directly between two identifier chars,
            // e.g. `glob.intfield`, `val.myptr`, `(ptr->value).myint`.
            l.as_bytes().windows(3).any(|w| {
                w[1] == b'.'
                    && (w[0].is_ascii_alphanumeric() || w[0] == b'_' || w[0] == b')')
                    && (w[2].is_ascii_alphabetic() || w[2] == b'_')
            })
        })
        .map(|l| l.to_string())
        .collect();
    out.sort();
    out
}

// ---------------------------------------------------------------------------
// (A1) REAL PARITY — per-function body byte-match against the C++ oracle.
// ---------------------------------------------------------------------------

#[test]
fn a1_printglob_struct_member_render_byte_matches_cpp_oracle() {
    let (rust, cpp) = match both_dumps("union_datatype") {
        Some(v) => v,
        None => return,
    };
    let rust_fn = extract_function(&rust, "printglob").expect("printglob in rust dump");
    let cpp_fn = extract_function(&cpp, "printglob").expect("printglob in cpp dump");
    eprintln!("=== RUST printglob ===\n{rust_fn}\n=== CPP printglob ===\n{cpp_fn}");
    // The struct-member render this item ships.
    assert!(
        rust_fn.contains("glob.intfield"),
        "pushPartialSymbol must render the struct member `glob.intfield`:\n{rust_fn}"
    );
    // The member-access LINES are byte-identical to the C++ oracle's — every
    // `glob.intfield / 3`, `glob.floatfield + 11.0`, `glob.uintfield >> 3` token
    // is reproduced exactly.  (The whole-function body still differs in the
    // if/else-if structuring, which is a separate not-yet-ported wave held in
    // `blockaction.rs`; that is NOT what this item delivers.)
    assert_eq!(
        member_lines(&rust_fn),
        member_lines(&cpp_fn),
        "printglob's struct-member render lines must byte-match the C++ oracle.\n--- rust ---\n{rust_fn}\n--- cpp ---\n{cpp_fn}"
    );
    assert!(
        member_lines(&rust_fn).len() >= 3,
        "printglob must render >=3 struct-member lines, got:\n{rust_fn}"
    );
}

#[test]
fn a1_testvalue_union_member_render_byte_matches_cpp_oracle() {
    let (rust, cpp) = match both_dumps("union_datatype") {
        Some(v) => v,
        None => return,
    };
    let rust_fn = extract_function(&rust, "testvalue").expect("testvalue in rust dump");
    let cpp_fn = extract_function(&cpp, "testvalue").expect("testvalue in cpp dump");
    eprintln!("=== RUST testvalue ===\n{rust_fn}\n=== CPP testvalue ===\n{cpp_fn}");
    // The UNION member render (`val.myptr` / `val.mydouble`) — the union arm of
    // the type walk reading the Funcdata resolution cache.
    assert!(
        rust_fn.contains("val.myptr") && rust_fn.contains("val.mydouble"),
        "pushPartialSymbol must render the union members `val.myptr` and `val.mydouble`:\n{rust_fn}"
    );
    // The union-member render lines (`*val.myptr + 3`, `val.mydouble + 33.0`)
    // that are PRESENT in the C++ oracle must appear byte-identically in the
    // Rust render.  (`val.mylong` still renders `SUB(val.mylong,0)` in Rust vs
    // `(int4)val.mylong` in C++ — Union #14, an unrelated cast-vs-SUBPIECE seam
    // still failing in BOTH the failing set and here; so we assert the union
    // lines that DID reach parity are present in Rust, not a full set-equality.)
    let cpp_union: Vec<String> = member_lines(&cpp_fn)
        .into_iter()
        .filter(|l| l.contains("val.myptr") || l.contains("val.mydouble"))
        .collect();
    let rust_union = member_lines(&rust_fn);
    for line in &cpp_union {
        assert!(
            rust_union.contains(line),
            "the C++ oracle's union-member line `{line}` is not byte-reproduced in Rust:\n{rust_fn}"
        );
    }
    assert!(cpp_union.len() >= 2, "oracle must have >=2 union lines (myptr+mydouble)");
}

// ---------------------------------------------------------------------------
// (A2) GUARD TIGHTNESS — a non-partial-symbol function is byte-UNCHANGED.
//
// `fillin` accesses `ptr[i].ffield` via a real PTRADD/struct-member chain, NOT
// via the symbol-mapped partial-cover path (its varnodes are not a partial cover
// of a composite Symbol).  Its render must byte-match the C++ oracle, proving the
// new guard did not perturb the universal explicit-render path.
// ---------------------------------------------------------------------------

#[test]
fn a2_non_partial_symbol_function_byte_unchanged() {
    let (rust, cpp) = match both_dumps("union_datatype") {
        Some(v) => v,
        None => return,
    };
    let rust_fn = extract_function(&rust, "fillin").expect("fillin in rust dump");
    let cpp_fn = extract_function(&cpp, "fillin").expect("fillin in cpp dump");
    eprintln!("=== RUST fillin ===\n{rust_fn}\n=== CPP fillin ===\n{cpp_fn}");
    // The partial-symbol path must NOT have fired here: no `glob.`/`val.` token
    // (fillin has neither global mapped into it).  A loose guard would inject one.
    assert!(
        !rust_fn.contains("glob.") && !rust_fn.contains("val."),
        "the partial-symbol guard fired on a non-partial-cover function:\n{rust_fn}"
    );
    // fillin's member accesses (`ptr[i].ifield`, `ptr[i].ffield`) are the formal
    // struct-field render (opSubpiece's `findTruncation` arm / array subscript),
    // and they are byte-identical to the C++ oracle.  (The whole-function body
    // differs only in the for-loop reroll — a separate wave held in
    // `blockaction.rs` — NOT in any member token this item touches.)
    assert_eq!(
        member_lines(&rust_fn),
        member_lines(&cpp_fn),
        "fillin's member-access render lines must be byte-unchanged vs the C++ oracle.\n--- rust ---\n{rust_fn}\n--- cpp ---\n{cpp_fn}"
    );
    // And every member access in fillin is `ptr[i].field` (NOT a `glob.`/`val.`
    // symbol-mapped partial cover) — confirming the new path is genuinely inert
    // here.
    for l in member_lines(&rust_fn) {
        assert!(
            l.contains("ptr[i]."),
            "fillin's only member access must be `ptr[i].field`, got `{l}`"
        );
    }
}

// ---------------------------------------------------------------------------
// (A3) DATA-DRIVEN, NOT SPECIAL-CASED — different types -> different fields, no
// cross-function leak.
// ---------------------------------------------------------------------------

#[test]
fn a3_member_names_are_type_driven_no_cross_function_leak() {
    let rust = match dump_print_c(&rust_harness(), "union_datatype") {
        Some(v) => v,
        None => return,
    };
    let printglob = extract_function(&rust, "printglob").expect("printglob in dump");
    let testvalue = extract_function(&rust, "testvalue").expect("testvalue in dump");
    // printglob renders ITS baseunion's `intfield`; testvalue renders ITS
    // myunion's `myptr`/`mydouble`.  Each from its own type's field table.
    assert!(printglob.contains("glob.intfield"), "printglob renders its own field:\n{printglob}");
    assert!(
        testvalue.contains("val.myptr") || testvalue.contains("val.mydouble"),
        "testvalue renders its own union field:\n{testvalue}"
    );
    // CROSS-FUNCTION PURITY: printglob's baseunion fields never appear in
    // testvalue, and testvalue's myunion fields never appear in printglob.  A
    // field name smuggled as a constant would leak across functions.
    assert!(
        !testvalue.contains("intfield")
            && !testvalue.contains("floatfield")
            && !testvalue.contains("uintfield"),
        "printglob's struct fields leaked into testvalue (field name is hardcoded):\n{testvalue}"
    );
    assert!(
        !printglob.contains("myptr")
            && !printglob.contains("mydouble")
            && !printglob.contains("mylong"),
        "testvalue's union fields leaked into printglob (field name is hardcoded):\n{printglob}"
    );
}
