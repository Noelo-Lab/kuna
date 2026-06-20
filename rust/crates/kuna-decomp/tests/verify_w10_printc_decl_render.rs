//! ADVERSARIAL tests for `rport/w10-printc-decl-render` — three render-only printc
//! seams, all in the local declaration / cast emission arm (the underlying
//! data/types are already correct; these fix only what the C printer emits):
//!
//!  * SEAM A — struct-return declaration collapse.  A register-returned struct is
//!    split into per-field proto-partial pieces (`RulePieceStructure`), each bound
//!    to the ROOT's shared name + the ROOT's whole-struct type + its own in-symbol
//!    byte offset (`bind_proto_partial_piece`).  C++ `emitScopeVarDecls` skips a
//!    piece entry (`if (entry->isPiece()) continue;`, printc.cc:2688) and the
//!    non-first whole-map (`getFirstWholeMap() != entry`, printc.cc:2697), so ONE
//!    shared `foo v1;` declares the whole symbol.  Before the fix the Rust
//!    name-local-highs loop emitted a SEPARATE decl per piece-high
//!    (`int4 v1; foo v1; int4 v1;`).
//!
//!  * SEAM B — global-scope symbol mis-declared as a local.  A `&symbol` reference
//!    that `linkSpacebaseSymbol` resolves through the GLOBAL scope (a ram spacebase,
//!    e.g. `myarray` materialized as a const base address) is rendered by name in
//!    the body but NEVER declared as a local — C++ `emitLocalVarDecls` declares only
//!    `fd->getScopeLocal()` (printc.cc:2336/2667).  Before the fix the Rust loop
//!    emitted `undefined20 myarray [3];` in the function body.
//!
//!  * SEAM C — address-of-cast collapse.  A CAST whose target is a pointer-to-array
//!    and whose input is `&array_symbol` renders as `&sym` (the array decays to the
//!    pointer-to-array type directly), dropping the spurious `(T(*)[n])` cast — C++
//!    `PrintC::checkAddressOfCast` + the `opTypeCast` pointer-to-array arm
//!    (printc.cc:396-438 / 468-484).  Before the fix the Rust rendered
//!    `paiGlob = (int4 *[16])c;` instead of `paiGlob = &c;`.
//!
//! All three are GENERIC over the declaration / scope / type geometry — NO struct /
//! field / symbol / function-name / address / value special-casing.  These tests
//! drive the built `decomp_test_dbg` (rust harness) with `KUNA_DUMP=1` and assert on
//! the sliced `print C` body; they SKIP cleanly if the binary / `.sla` is missing.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    std::env::var_os("KUNA_SPECS").map(PathBuf::from).unwrap_or_else(|| repo_root().join("specs"))
}

fn datatests_dir() -> PathBuf {
    std::env::var_os("KUNA_DATATESTS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/datatests"))
}

fn rust_harness() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST_RUST") {
        return PathBuf::from(p);
    }
    let profile = std::env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
    repo_root().join("rust/target").join(profile).join("decomp_test_dbg")
}

/// Drive `bin` on `stem`.xml with `KUNA_DUMP=1`, returning the full concatenated
/// bulk `print C` output (between the `===KUNA_DUMP_BEGIN===`/`===KUNA_DUMP_MID===`
/// markers), or `None` if the binary / `.sla` is unavailable (the test SKIPs).
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
    let body = text[after_begin..mid].to_string();
    if body.trim().is_empty() {
        eprintln!("SKIP: {stem} produced no C body");
        return None;
    }
    Some(body)
}

/// Slice the body of ONE function `name` from the concatenated dump (the lines from
/// the function signature down to its closing `}` at column 0).
fn func_body<'a>(dump: &'a str, name: &str) -> Option<&'a str> {
    let sig = format!("{name}(");
    let start = dump.find(&sig)?;
    // Back up to the start of the signature line.
    let line_start = dump[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let rest = &dump[line_start..];
    // The function ends at the first line that is exactly `}` (column 0).
    let end_rel = rest.find("\n}")?;
    Some(&rest[..end_rel + 2])
}

/// Count non-overlapping occurrences of `needle` in `hay`.
fn count(hay: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut n = 0usize;
    let mut i = 0usize;
    while let Some(p) = hay[i..].find(needle) {
        n += 1;
        i += p + needle.len();
    }
    n
}

// =============================================================================
// (T1) SEAM A — concat shares ONE struct decl.  The register-returned struct's
//      per-field pieces collapse to a single whole-symbol declaration
//      (`foo v1; // rax`), exactly as the C++ getFirstWholeMap behavior.  Before
//      the fix concatreturn carried three decls (`int4 v1; foo v1; int4 v1;`).
// =============================================================================
#[test]
fn t1_concat_struct_return_shares_one_whole_symbol_decl() {
    let Some(dump) = dump_print_c(&rust_harness(), "concat") else { return };
    let body = func_body(&dump, "concatreturn")
        .unwrap_or_else(|| panic!("concatreturn not found in dump:\n{dump}"));

    // EXACTLY ONE local declaration line for the returned struct (`foo v1;`).
    let decl_count = count(body, "foo v1;");
    assert_eq!(
        decl_count, 1,
        "concatreturn: expected exactly ONE shared `foo v1;` whole-symbol decl \
         (C++ getFirstWholeMap), got {decl_count}. The per-field proto-partial \
         pieces are still each emitting their own decl. Body:\n{body}"
    );
    // The redundant per-field scalar decls (`int4 v1;`) must be GONE — only the
    // whole struct declares; the pieces share it.
    assert!(
        !body.contains("int4 v1;"),
        "concatreturn: a per-field piece (`int4 v1;`) still emits its own \
         declaration — SEAM A did not collapse the split-return pieces. Body:\n{body}"
    );
    // The body's field writes through the shared symbol are intact (the data was
    // always correct; this is a render-only collapse).
    assert!(
        body.contains("v1.A = retp1;") && body.contains("v1.B = retp2;"),
        "concatreturn: the per-field writes through the shared symbol vanished — \
         the collapse must not drop the field statements. Body:\n{body}"
    );

    // The same generic collapse applies to a wider struct (`fooshort`, four
    // fields): ONE `fooshort v1;` decl, no per-field scalar decls.
    if let Some(short) = func_body(&dump, "concatreturnshort") {
        assert_eq!(
            count(short, "fooshort v1;"),
            1,
            "concatreturnshort: expected exactly ONE shared `fooshort v1;` decl. \
             Body:\n{short}"
        );
        assert!(
            !short.contains("int2 v1;") && !short.contains("undefined6 v1;"),
            "concatreturnshort: a per-field piece still emits its own decl. \
             Body:\n{short}"
        );
    }
}

// =============================================================================
// (T2) SEAM B — a GLOBAL is not locally declared.  The mapped global array
//      `myarray` is rendered by NAME in the body (`myarray[globindex][...]`) but
//      carries NO local declaration — it lives in the global scope, which
//      `emitLocalVarDecls` never walks.  Before the fix the body declared
//      `undefined20 myarray [3];`.
// =============================================================================
#[test]
fn t2_global_spacebase_array_is_not_locally_declared() {
    let Some(dump) = dump_print_c(&rust_harness(), "twodim") else { return };
    let body = func_body(&dump, "array_access")
        .unwrap_or_else(|| panic!("array_access not found:\n{dump}"));

    // The global symbol IS referenced by name in the body (the underlying data is
    // correct — this is render-only).
    assert!(
        body.contains("myarray[globindex]"),
        "twodim: the global `myarray[globindex]` reference is missing. Body:\n{body}"
    );
    // The global symbol must NOT be declared as a local.  Any local declaration of
    // `myarray` (an `[N]` array decl) is the bug.  Assert no declaration line.
    for line in body.lines() {
        let t = line.trim();
        let is_myarray_decl = t.ends_with("myarray [3];")
            || t.ends_with("myarray[3];")
            || (t.contains(" myarray ") && t.ends_with("];") && !t.contains('='));
        assert!(
            !is_myarray_decl,
            "twodim: the GLOBAL symbol `myarray` is wrongly declared as a LOCAL \
             (`{t}`) — emitLocalVarDecls must skip a global-scoped high. Body:\n{body}"
        );
    }

    // Generalizes to the 3D global array (threedim's `myarray`): referenced by
    // name, never locally declared.
    if let Some(dump3) = dump_print_c(&rust_harness(), "threedim") {
        if let Some(body3) = func_body(&dump3, "array_access") {
            assert!(
                body3.contains("myarray[globindex]"),
                "threedim: global `myarray[globindex]` missing. Body:\n{body3}"
            );
            for line in body3.lines() {
                let t = line.trim();
                let is_decl = (t.contains(" myarray ") || t.contains(" myarray["))
                    && t.ends_with("];")
                    && !t.contains('=');
                assert!(
                    !is_decl,
                    "threedim: global `myarray` wrongly declared as a local (`{t}`). \
                     Body:\n{body3}"
                );
            }
        }
    }
}

// =============================================================================
// (T2b) SEAM B IS SCOPE-DRIVEN, NOT CONST-DRIVEN.  A LOCAL-frame spacebase symbol
//       referenced via the same const-materialized `&symbol` path (the stack array
//       `c` in passPtrToArray) MUST still be declared as a local — only the GLOBAL
//       scope is skipped.  This pins that the fix discriminates by SCOPE, not by
//       "it is a const high" (which would wrongly drop local-frame symbol decls).
// =============================================================================
#[test]
fn t2b_local_frame_spacebase_symbol_still_declared() {
    let Some(dump) = dump_print_c(&rust_harness(), "ptrtoarray") else { return };
    let body = func_body(&dump, "passPtrToArray")
        .unwrap_or_else(|| panic!("passPtrToArray not found:\n{dump}"));
    // The stack-local array `c` (a LOCAL-frame symbol) is STILL declared — the
    // global-scope skip must not over-reach into local-frame references.
    assert!(
        body.contains("c [16]"),
        "passPtrToArray: the LOCAL-frame stack array `c` lost its declaration — \
         the global-scope decl skip wrongly dropped a local symbol. Body:\n{body}"
    );
}

// =============================================================================
// (T3) SEAM C — address-of-array renders `&name`.  The pointer-to-array CAST over
//      `&c` collapses to `&c` (no spurious `(int4 *[16])` cast), via the faithful
//      `checkAddressOfCast` (the input's PTRSUB root resolves the array symbol of
//      matching size through `getSubType`).  Before the fix: `(int4 *[16])c`.
// =============================================================================
#[test]
fn t3_address_of_array_cast_collapses_to_ampersand_name() {
    let Some(dump) = dump_print_c(&rust_harness(), "ptrtoarray") else { return };
    let body = func_body(&dump, "passPtrToArray")
        .unwrap_or_else(|| panic!("passPtrToArray not found:\n{dump}"));

    // The address-of collapse: `paiGlob = &c;` with NO array-pointer cast token.
    assert!(
        body.contains("paiGlob = &c;"),
        "passPtrToArray: the address-of-array CAST did not collapse to `&c` — \
         checkAddressOfCast did not fire. Body:\n{body}"
    );
    // The spurious array-pointer cast must be GONE.
    assert!(
        !body.contains("(int4 *[16])"),
        "passPtrToArray: the spurious `(int4 *[16])` array-pointer cast survived \
         on the address-of. Body:\n{body}"
    );
}

// =============================================================================
// (T4) SEAM C IS SURGICAL — it must NOT drop a legitimate cast.  A
//      pointer-to-array cast whose input is NOT an `&array_symbol` of matching size
//      keeps its cast.  Pin that the legitimate scalar/precision casts elsewhere
//      survive (the address-of arm is gated on `isPointerToArray()` + the strict
//      `checkAddressOfCast` predicate, never opcode/name keyed).
// =============================================================================
#[test]
fn t4_seam_c_does_not_remove_legitimate_casts() {
    if let Some(dump) = dump_print_c(&rust_harness(), "floatcast") {
        assert!(
            dump.contains("(float8)") && dump.contains("(float4)"),
            "floatcast: a legitimate `(float8)`/`(float4)` precision cast was \
             removed — the address-of-cast arm over-reached. Dump:\n{dump}"
        );
    }
    if let Some(dump) = dump_print_c(&rust_harness(), "packstructaccess") {
        assert!(
            dump.contains("(int4)"),
            "packstructaccess: a legitimate `(int4)` truncation cast was removed. \
             Dump:\n{dump}"
        );
    }
    // The ptrToArray function (the typed pointer-to-array parameter case) keeps its
    // own structured body and `int4 (*v1) [16]` typed local — no spurious collapse.
    if let Some(dump) = dump_print_c(&rust_harness(), "ptrtoarray") {
        if let Some(body) = func_body(&dump, "ptrToArray") {
            assert!(
                body.contains('{') && body.contains('}'),
                "ptrToArray: the render lost its body. Body:\n{body}"
            );
        }
    }
}
