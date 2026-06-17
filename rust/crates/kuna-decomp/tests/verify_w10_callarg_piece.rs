//! Adversarial tests for item `w10-callarg-piece` — call-argument PIECE recovery
//! (the INPUT mirror of the struct-return field-binding `RulePieceStructure` path).
//!
//! ROOT CAUSE this item closed: a by-value struct (or any callee-typed
//! aggregate) passed as a CALL argument was rendered with the forming CONCAT
//! inlined (`structparam(CONCAT(regp2,regp1))`) instead of split into per-field
//! writes (`v1.A = regp1; v1.B = regp2; structparam(v1);`).  The split is done by
//! `RulePieceStructure`, which fires only when the CONCAT output Varnode carries
//! a *structured* data-type.  On the call-arg side that type comes from the
//! callee parameter (`TypeOpCall::getInputLocal`, typeop.cc:689), but the callee
//! `FuncProto` was never copied into the call's `FuncCallSpecs`
//! (`ActionDefaultParams`, coreaction.cc:2385: `fc->copy(otherfunc->getFuncProto())`)
//! because the declared callee's locked prototype (`parse line extern <sig>`,
//! `Architecture::setPrototype`, architecture.cc:393) was never applied to the
//! callee FunctionSymbol's `TypeCode`.  This item:
//!   * applies a parsed `extern` prototype to the matching FunctionSymbol
//!     (`apply_prototype_to_symbol` → `retype_symbol` with `getTypeCode(pieces)`);
//!   * surfaces the callee proto to `ActionDefaultParams` via the global-query
//!     snapshot (`Architecture::query_callee_proto` reading `TypeCode::getPrototype`);
//!   * copies it into the call spec (`callee_func_proto` + `fc.copy`).
//!
//! These drive the *real* gate path: the committed Rust `decomp_test_dbg`
//! harness on `concat.xml` / `longdouble.xml` whose `<script>` runs
//! `parse line extern` + `lo fu` + `decompile` + `print C`, then compares the
//! bulk `print C` against the C++ oracle on the same datatest.  Each assertion is
//! keyed to generic structure geometry (field-write FORM, no struct/field/
//! function name as the discriminator) so it cannot be met by a special-cased
//! decode.  All tests SKIP (not fail) when a binary or `.sla` is unavailable,
//! honoring the same pattern as `verify_w10_input_prototype_adversarial.rs`.

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/decomp_test_dbg")
}

fn cpp_oracle_bin() -> PathBuf {
    std::env::var_os("KUNA_DECOMP_TEST")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/cpp/decomp_test_dbg"))
}

/// Drive a `decomp_test_dbg` binary on one datatest with `KUNA_DUMP=1` and
/// return the dumped `print C` block.  `None` if the binary is missing or
/// produced no dump (no spec/`.sla`).
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

/// Extract the body of a named function from a `print C` dump (the lines from the
/// `<sig> {`-introducing signature line through the matching closing brace).
fn function_block<'a>(dump: &'a str, name: &str) -> Option<&'a str> {
    let sig = dump.lines().position(|l| l.contains(name) && l.contains('('))?;
    let bytes = dump.as_bytes();
    // Find the byte offset of that signature line.
    let mut off = 0usize;
    for (i, line) in dump.lines().enumerate() {
        if i == sig {
            break;
        }
        off += line.len() + 1;
    }
    // Walk to the closing brace that returns depth to 0.
    let mut depth = 0i32;
    let mut seen_open = false;
    let mut end = off;
    for (i, &b) in bytes[off..].iter().enumerate() {
        if b == b'{' {
            depth += 1;
            seen_open = true;
        } else if b == b'}' {
            depth -= 1;
            if seen_open && depth == 0 {
                end = off + i + 1;
                break;
            }
        }
    }
    Some(&dump[off..end])
}

/// AT1 — POSITIVE: a by-value struct passed as a CALL argument is gathered and
/// rendered as per-field write statements (the INPUT mirror of the return-side
/// `v1.A = retp1; v1.B = retp2`), and the CONCAT no longer survives inline.
///
/// `concatregparam` calls `structparam(foo)` by value; the oracle renders
/// `v1.A = regp1; v1.B = regp2; structparam(v1);`.  Pre-item the Rust path
/// emitted `structparam(CONCAT(regp2,regp1));`.
#[test]
fn at1_by_value_struct_arg_renders_field_writes_not_inline_concat() {
    let rust = match dump_print_c(&rust_test_bin(), "concat") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "concatregparam")
        .unwrap_or_else(|| panic!("no concatregparam block in:\n{rust}"));

    // The forming CONCAT must NOT survive as an inline call argument.
    assert!(
        !body.contains("structparam(CONCAT"),
        "by-value struct call argument still renders the inline CONCAT (RulePieceStructure \
         did not split the call-arg PIECE tree); got:\n{body}"
    );
    // The argument is passed as the gathered whole-struct variable.
    assert!(
        body.contains("structparam(v1)"),
        "the call argument is not the gathered whole-struct variable `v1`; got:\n{body}"
    );
    // Both fields are recovered as separate write statements (the per-field render).
    assert!(
        body.contains("v1.A = regp1;") && body.contains("v1.B = regp2;"),
        "the two struct fields are not rendered as separate writes; got:\n{body}"
    );

    // If the oracle is available, the statement content must match it exactly
    // (the gather is faithful, not an approximation of the field text); compare
    // with the variable-storage `// <reg>` comment stripped (the orthogonal
    // variable-comment plane).
    if let Some(cpp) = dump_print_c(&cpp_oracle_bin(), "concat") {
        if let Some(cpp_body) = function_block(&cpp, "concatregparam") {
            assert_eq!(
                strip_line_comments(body),
                strip_line_comments(cpp_body),
                "concatregparam statement content diverges from the C++ oracle"
            );
        }
    }
}

/// AT2 — GENERIC (no name-keying): the same call-arg split fires for a DIFFERENT
/// struct (`fooshort`) with DIFFERENT field names (`a`/`b`/`c`/`d`) and a partial
/// in-place update.  `replaceshort` calls `structshort(fooshort)`; the oracle
/// gathers the struct from a mix of a replaced field and globals and renders
/// `v1.b = replace1; v1.a = globshort.a; ...; structshort(v1);`.
///
/// Because the recovery is keyed only on the callee parameter's *structured
/// data-type* (not on any name), a special-cased `foo`/`A`/`B`/`structparam`
/// decode could not satisfy this — the names here are entirely different.
#[test]
fn at2_generic_struct_arg_split_no_name_keying() {
    let rust = match dump_print_c(&rust_test_bin(), "concat") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "replaceshort")
        .unwrap_or_else(|| panic!("no replaceshort block in:\n{rust}"));

    // No inline CONCAT survives as the call argument (a different struct/fields).
    assert!(
        !body.contains("structshort(CONCAT"),
        "fooshort by-value call argument still renders the inline CONCAT; got:\n{body}"
    );
    assert!(
        body.contains("structshort(v1)"),
        "the fooshort argument is not the gathered whole-struct variable; got:\n{body}"
    );
    // The four DIFFERENTLY-NAMED fields are recovered as separate writes — the
    // recovery walked the struct's own geometry, not a hardcoded `A`/`B` pair.
    for fw in ["v1.b = replace1;", "v1.a = globshort.a;", "v1.c = globshort.c;", "v1.d = globshort.d;"] {
        assert!(
            body.contains(fw),
            "missing generic field write `{fw}` (struct geometry not walked generically); \
             got:\n{body}"
        );
    }

    // Oracle parity on the STATEMENT content (the field-writes + the call): the
    // only residual gap is the declaration-line trailing `// <storage>` comment
    // (`fooshort v1;` vs `fooshort v1; // tmp`), a pre-existing variable-comment
    // plane divergence orthogonal to the call-arg gather.  Compare with trailing
    // `//` comments stripped so the load-bearing statements are checked exactly.
    if let Some(cpp) = dump_print_c(&cpp_oracle_bin(), "concat") {
        if let Some(cpp_body) = function_block(&cpp, "replaceshort") {
            assert_eq!(
                strip_line_comments(body),
                strip_line_comments(cpp_body),
                "replaceshort statement content diverges from the C++ oracle"
            );
        }
    }
}

/// Drop the trailing `// ...` comment from every line (the variable-storage
/// annotation plane), keeping the statement content for an exact comparison.
fn strip_line_comments(block: &str) -> String {
    block
        .lines()
        .map(|l| match l.find("//") {
            Some(i) => l[..i].trim_end(),
            None => l.trim_end(),
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// AT3 — the split is DRIVEN BY THE CALLEE PROTO, not a same-function default.
/// The return-side `concatreturn` (its struct type comes from the FUNCTION's
/// recovered output) already split before this item; the call-arg `concatregparam`
/// (its struct type must come from the CALLEE param) did not.  Both now split, and
/// the call-arg block must show the *call* form (`structparam(v1)`), proving the
/// callee parameter type — not the function's own output type — drove it.
#[test]
fn at3_callarg_split_uses_callee_param_type() {
    let rust = match dump_print_c(&rust_test_bin(), "concat") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    // The call-arg function gathers and CALLS with the whole struct.
    let call_body = function_block(&rust, "concatregparam")
        .unwrap_or_else(|| panic!("no concatregparam block in:\n{rust}"));
    assert!(
        call_body.contains("structparam(v1)") && !call_body.contains("CONCAT"),
        "call-arg struct not gathered from the callee param type; got:\n{call_body}"
    );
    // The return-side function still splits (regression guard for struct-return-v2).
    let ret_body = function_block(&rust, "concatreturn")
        .unwrap_or_else(|| panic!("no concatreturn block in:\n{rust}"));
    assert!(
        ret_body.contains("v1.A = retp1;")
            && ret_body.contains("v1.B = retp2;")
            && ret_body.contains("return v1;")
            && !ret_body.contains("CONCAT"),
        "return-side split regressed; got:\n{ret_body}"
    );
}

/// AT4 — NEGATIVE GUARD (base_explicit-v2 follow-on): a wide-float (`float10`)
/// CALL argument is NOT a piece-structured type (`metatype == TYPE_FLOAT`, not
/// `<= TYPE_ARRAY`), so `RulePieceStructure` cannot — and must not — gather it.
/// Its x87 register-pair reassembly is the separate, shelved base_explicit-v2
/// path (`f2c3e72`).  This pins the current state for that fold-in: making the
/// callee proto visible (this item) does NOT by itself build the float10 arg
/// CONCAT — `passmany` still drops the `x` argument (`writeLongDouble(ldarr)`),
/// matching the pre-base_explicit-v2 baseline.  When base_explicit-v2 lands and
/// the arg is reconstructed, THIS guard flips and is updated alongside it.
#[test]
fn at4_widefloat_callarg_concat_not_built_without_base_explicit() {
    let rust = match dump_print_c(&rust_test_bin(), "longdouble") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "passmany")
        .unwrap_or_else(|| panic!("no passmany block in:\n{rust}"));
    // The float10 argument is still dropped: the wide-float gather is NOT this
    // item's RulePieceStructure path (it is base_explicit-v2's reassembly).
    assert!(
        body.contains("writeLongDouble(ldarr)") && !body.contains("writeLongDouble(ldarr,x)"),
        "the float10 call-arg was reconstructed without base_explicit-v2 — this guard \
         (and the base_explicit-v2 fold-in note) must be revisited; got:\n{body}"
    );
}
