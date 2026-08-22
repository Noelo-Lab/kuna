//! End-to-end gate for **`option setlanguage rust-language`**: the emitted
//! document must be syntactically valid Rust.
//!
//! # Why a parser and not string assertions
//!
//! String assertions can only fail on a construct someone remembered to assert.
//! A parser fails on a *category* -- an unbalanced brace, a stray `;` before an
//! arm, an elided block body, a leaked `->`, `~`, `goto` or `do { } while`. That
//! matters here because the reference implementation this back-end is modelled
//! on (SEFCOM Oxidizer, merged into angr as `structured_codegen/rust.py`) emits
//! all four of those into output it calls Rust, and its own showcase snippet
//! does not parse. `syn::parse_file` is the check that keeps kuna honest.
//!
//! `syn` is a token-level oracle: it accepts some things rustc rejects on
//! semantic grounds. The C-variadic case below is exactly that — `unsafe fn
//! f(a: i32, ...)` parses and does not compile — so the shape assertions here
//! carry the rules a parser alone would miss.
//!
//! # Parseable, not compilable
//!
//! The scope boundary, deliberately: decompiled output calls functions that have
//! no definition, `CARRY4(a, b)` has no Rust spelling, `>>` signedness follows
//! the operand type, and `[u8; 3]` does not do arithmetic. Making the output
//! *compile* is a different and much larger project. This gate asserts it
//! parses.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The stripped shadow `faillog` (23 KB): small enough to decompile whole in a
/// unit-test budget, and it still carries loops, switches, calls and casts.
fn faillog() -> PathBuf {
    repo_root().join("tests/bug-repro/faillog")
}

/// `grep` (170 KB): the breadth arm. Its bodies reach the constructs `faillog`
/// does not -- nested loops, jump tables, deep member chains.
fn grep() -> PathBuf {
    repo_root().join("tests/bug-repro/grep")
}

/// Decompile `func` from `bin` under `lang`, returning the emitted document
/// (`None` => specs-less skip).
fn decompile_as(bin: &PathBuf, func: &str, lang: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = bin.to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_outlang_rust_syntax: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> = vec![
        format!("option setlanguage {lang}"),
        format!("load function {func}"),
        "decompile".into(),
        "print C".into(),
    ];
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(strip_console_chatter(&status.optr))
}

/// The raw console transcript, for the arms that assert on a console message
/// rather than on the document.
fn console_transcript(bin: &PathBuf, func: &str, lang: &str) -> Option<String> {
    RAW.with(|r| r.set(true));
    let out = decompile_as(bin, func, lang);
    RAW.with(|r| r.set(false));
    out
}

thread_local! {
    static RAW: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The console interleaves its own progress lines (`Decompiling <f>`, ...) with
/// the document; the parser needs the document alone.
fn strip_console_chatter(out: &str) -> String {
    if RAW.with(|r| r.get()) {
        return out.to_string();
    }
    match out.rfind("Decompilation complete") {
        Some(i) => out[i..].splitn(2, '\n').nth(1).unwrap_or("").trim_start_matches('\n').to_string(),
        None => out.to_string(),
    }
}

/// The emitted document is a function item, so it parses as a file on its own.
fn assert_parses_as_rust(code: &str, what: &str) {
    match syn::parse_file(code) {
        Ok(_) => {}
        Err(e) => panic!("{what}: emitted document is not valid Rust: {e}\n--- document ---\n{code}"),
    }
}

#[test]
fn rust_output_parses() {
    for func in ["sub_2f60", "sub_3050", "sub_31a0"] {
        let Some(code) = decompile_as(&faillog(), func, "rust-language") else { return };
        assert_parses_as_rust(&code, func);
    }
}

#[test]
fn rust_output_parses_on_a_wider_body() {
    let Some(code) = decompile_as(&grep(), "sub_144a0", "rust-language") else { return };
    assert_parses_as_rust(&code, "grep/sub_144a0");
}

/// Jump tables are the construct with the least overlap between the two
/// languages: C's `switch` falls through, allows no `default`, and needs an
/// explicit `break`, while `match` does none of those. These four bodies carry
/// 6, 13, 17 and 30 recovered cases between them.
#[test]
fn rust_match_output_parses() {
    for func in ["sub_eda0", "sub_f9c0", "sub_15020", "sub_1f450"] {
        let Some(code) = decompile_as(&grep(), func, "rust-language") else { return };
        assert_parses_as_rust(&code, &format!("grep/{func}"));
        assert!(code.contains("match "), "expected a match in {func}:\n{code}");
        assert!(code.contains("_ => "), "a match on an integer needs a wildcard arm in {func}");
        assert!(!code.contains("case "), "`case` has no match form; got:\n{code}");
    }
}

/// The multi-label shape is the common one and must not be mistaken for a
/// fall-through chain: `case A: case B: body` is ONE recovered case with two
/// jump-table indices, which is exactly Rust's `A | B =>`.
#[test]
fn multi_label_arms_use_pattern_alternation() {
    let Some(code) = decompile_as(&grep(), "sub_f9c0", "rust-language") else { return };
    let re = regex::Regex::new(r"(?m)^\s*0x[0-9a-f]+( \| 0x[0-9a-f]+)+ => \{").unwrap();
    assert!(re.is_match(&code), "expected an `A | B => {{` arm; got:\n{code}");
}

/// The four C-isms the reference implementation leaks into its Rust output.
/// Each has a `LangForms`/`LangCaps` member that is supposed to prevent it, so
/// this is the regression test for those members reaching the emitters.
#[test]
fn rust_output_carries_no_c_isms() {
    let Some(code) = decompile_as(&grep(), "sub_144a0", "rust-language") else { return };
    // The scan is over CODE, not prose: the residual-goto marker deliberately
    // says "unstructured goto", and the signature's `) -> T` is a return arrow.
    let body: String = code
        .lines()
        .map(|l| match l.find("/*") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace(") -> ", ") RETURNS ");
    for (pattern, why) in [
        (r"->", "Rust raw pointers have no auto-deref; expected `(*p).f`"),
        (r"~", "Rust spells bitwise complement `!`"),
        (r"(?m)^\s*goto\s+\w+\s*;", "Rust has no goto"),
        (r"\}\s*while", "Rust has no do/while"),
        (r"\bfor\s*\(", "Rust has no C-style for header"),
        (r"\b\d+[UL]+\b", "Rust has no C integer suffixes"),
    ] {
        let re = regex::Regex::new(pattern).unwrap();
        assert!(!re.is_match(&body), "rust output matches `{pattern}`: {why}\n{code}");
    }
}

/// The C arm of the same path is untouched: selecting C still emits C. Pins that
/// the language is a real selection rather than a global mode change.
#[test]
fn c_output_is_unchanged_by_the_language_seam() {
    let Some(code) = decompile_as(&faillog(), "sub_3050", "c-language") else { return };
    assert!(code.contains("void sub_3050("), "expected the C prototype form; got:\n{code}");
    assert!(!code.contains("unsafe fn"), "C output must not carry the Rust shell:\n{code}");
    assert!(!code.contains("let mut"), "C output must not carry Rust declarations:\n{code}");
}

/// A name no back-end claims is an error rather than a silent fallback to C --
/// otherwise a typo would keep emitting C under a name that says otherwise.
#[test]
fn an_unknown_language_is_rejected() {
    let Some(out) = console_transcript(&faillog(), "sub_3050", "not-a-language") else { return };
    assert!(
        out.contains("Unknown print language"),
        "expected the unknown-language error; got:\n{out}"
    );
}

/// A C-variadic parameter is only legal on an `unsafe extern "C" fn`. `syn`
/// accepts the shorter `unsafe fn f(a, ...)` (the restriction is semantic, not
/// grammatical), so this is asserted directly rather than left to the parser --
/// it was the single largest source of non-compiling output before the fix.
#[test]
fn variadic_prototypes_carry_extern_c() {
    let Some(code) = decompile_as(&grep(), "__printf_chk", "rust-language") else { return };
    assert!(code.contains(", ...)"), "expected a recovered variadic; got:\n{code}");
    assert!(
        code.contains("unsafe extern \"C\" fn"),
        "a C-variadic needs `unsafe extern \"C\" fn`; got:\n{code}"
    );
}

/// The non-variadic majority keeps the shorter shell.
#[test]
fn non_variadic_prototypes_stay_plain_unsafe_fn() {
    let Some(code) = decompile_as(&faillog(), "sub_3050", "rust-language") else { return };
    assert!(code.contains("unsafe fn sub_3050("), "got:\n{code}");
    assert!(!code.contains("extern \"C\""), "no variadic here; got:\n{code}");
}
