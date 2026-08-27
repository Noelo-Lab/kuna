//! End-to-end gate for the `symbolnamechars` option (GH-340), all three arms.
//!
//! A symbol name's bytes reach emitted C with nothing between `.strtab` and the
//! printer that validates them. `a*/b` closes the `// Function:` header comment,
//! `a//b` comments out the rest of the line, a raw `0x0a` splits every rendering
//! of the name in two, and `String::from_utf8_lossy` maps every invalid byte to
//! U+FFFD so `a\x80b` and `a\x81b` — two symbols at two addresses — become one
//! `String` and the export carries a C redefinition.
//!
//! The option sanitizes at the MINT (the loader's symbol walks), so one string
//! serves `prog.symbols`, `kuna functions`, `kuna decompile <name>`, the DB scope
//! path and the emitted C. That is what this file's round-trip assertion is
//! about: every name the symbol table now reports must still be a name the
//! symbol table can be asked for.
//!
//! ## Why the two-pass coverage lives here and not in a stage XML
//!
//! `symbolnamechars` is a LOADER-tier gate: the names are minted inside `load
//! file`, and a stage script's `option` command requires a live image, so no
//! script can set the mode before the load it has to affect.
//! `tests/stages/kuna-symbolnamechars.xml` therefore pins the DEFAULT arm only
//! and the off/ident arms live here, where the env var can be set around each
//! load. Same limit, same reason, as `verify_symbolnamerepair.rs`.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early — a specs-less CI is a visible skip, never a false green.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::kuna_symbolnamechars::{NameChars, SYMBOLNAMECHARS_ENV};

/// Serializes the env-var flip + bootstrap across the concurrently-run tests
/// (the guard `verify_symbolnamerepair.rs` / `verify_typedepth.rs` use): the
/// gate is a process-global env var, and every object load reads it.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

/// Bootstrap `bin` with the sanitizer forced to `mode` and return every function
/// name the symbol table installed, or the error text the load failed with.
fn load_function_names(bin: &str, mode: NameChars) -> Result<Vec<String>, String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    std::env::set_var(SYMBOLNAMECHARS_ENV, mode.as_str());
    let prog = bootstrap_from_object(bin, "", &spec_roots);
    std::env::remove_var(SYMBOLNAMECHARS_ENV);
    let prog = prog.map_err(|e| e.explain().to_string())?;
    Ok(prog.function_entries().map(|(name, _)| name.to_string()).collect())
}

/// Whether the `.sla` is present, so a bootstrap failure means what it says.
/// Returns `false` (and prints) when it is not — a visible skip.
fn specs_available() -> bool {
    match load_function_names(&fixture("hostile_symname_x86_64"), NameChars::Safe) {
        Err(e) if e.contains("sleigh specification") || e.contains(".sla") => {
            eprintln!("verify_symbolnamechars: skipping (no `.sla`, build with `make specs`): {e}");
            false
        }
        _ => true,
    }
}

/// The broken arm. `off` is today's verbatim behavior, kept reachable for
/// someone auditing what a binary literally claims — and it still carries every
/// half of the defect, which is what makes it the paired proof that the default
/// is doing the work.
#[test]
fn off_restores_every_half_of_the_defect() {
    if !specs_available() {
        return;
    }
    let names = load_function_names(&fixture("hostile_symname_x86_64"), NameChars::Off)
        .expect("the fixture must still load with the sanitizer off");

    // The structural names arrive verbatim, comment delimiters and all.
    for raw in ["a*/b", "a//b", "a\nb"] {
        assert!(names.iter().any(|n| n == raw), "expected the raw {raw:?} in {names:?}");
    }
    // And the two invalid-UTF-8 names collapse onto ONE string at two addresses:
    // the export would carry two definitions and two prototypes with one name.
    let lossy = "a\u{fffd}b";
    assert_eq!(
        names.iter().filter(|n| *n == lossy).count(),
        2,
        "0x80 and 0x81 must both have become U+FFFD with the gate off; got {names:?}"
    );
}

/// The default arm. Every hostile shape is neutralized, the two invalid-UTF-8
/// names stay two names, and nothing else about the binary changes.
#[test]
fn safe_neutralizes_every_half_and_keeps_distinct_names_distinct() {
    if !specs_available() {
        return;
    }
    let names = load_function_names(&fixture("hostile_symname_x86_64"), NameChars::Safe)
        .expect("the fixture must load under the shipped default");

    for expected in ["main", "a_x2a_x2fb", "a_x2f_x2fb", "a_x0ab", "a_x80b", "a_x81b"] {
        assert!(names.iter().any(|n| n == expected), "missing {expected:?}; got {names:?}");
    }
    // No name may still be able to restructure the document it is printed into.
    for n in &names {
        assert!(!n.contains("*/"), "a comment delimiter survived: {n:?}");
        assert!(!n.contains("/*"), "a comment delimiter survived: {n:?}");
        assert!(!n.contains("//"), "a line comment survived: {n:?}");
        assert!(!n.chars().any(|c| c.is_control()), "a control character survived: {n:?}");
        assert!(!n.contains('\u{fffd}'), "a lossy replacement survived: {n:?}");
    }
    // The identity half: six symbols in, six DISTINCT names out.
    let mut sorted = names.clone();
    sorted.sort();
    let before = sorted.len();
    sorted.dedup();
    assert_eq!(sorted.len(), before, "two names collapsed onto one: {names:?}");
}

/// `safe` must be a NO-OP on the names a real optimizing toolchain emits — that
/// is the whole reason `safe` and `ident` are separate values. `ident` rewrites
/// exactly those, which is why it cannot be the default.
#[test]
fn safe_leaves_gcc_clone_suffixes_alone_and_ident_folds_them() {
    if !specs_available() {
        return;
    }
    // A real gcc -O2 binary whose `.symtab` carries `.constprop.0` clone names.
    let bin = fixture("noreturn_error_x86_64");
    let safe = load_function_names(&bin, NameChars::Safe).expect("safe load");
    let off = load_function_names(&bin, NameChars::Off).expect("off load");
    assert_eq!(safe, off, "safe must not rewrite a single name on a real gcc binary");
    assert!(
        safe.iter().any(|n| n == "err_fatal.constprop.0"),
        "the fixture must actually carry a clone suffix; got {safe:?}"
    );

    let ident = load_function_names(&bin, NameChars::Ident).expect("ident load");
    assert!(
        ident.iter().any(|n| n == "err_fatal_constprop_0"),
        "ident must fold the clone suffix; got {ident:?}"
    );
    assert!(
        !ident.iter().any(|n| n.contains('.')),
        "ident must leave no non-identifier character behind; got {ident:?}"
    );
}

/// The risk sanitizing at the mint introduces: it changes the KEY in
/// `prog.symbols`, which is what `kuna decompile <name>` and the console's `load
/// function` are passed.
///
/// `IfcFuncload` reads its argument with `CommandStream::read_token`, which
/// splits on WHITESPACE, and then resolves it with `ConsoleProgram::lookup_symbol`.
/// So a reported name is addressable iff it survives tokenization and is unique —
/// the resolve itself reads the same vector `function_entries` iterates, so
/// asserting only that would be tautological. What is asserted here instead is
/// the pair of properties that are NOT free: no reported name carries whitespace
/// or a control byte (under `off`, `a\nb` fails exactly this and is unaddressable
/// from the CLI, the console and every stage script), and no two functions at
/// different addresses share a name. Checked on the hostile fixture AND on two
/// real unstripped C++ binaries, where the names are `::`-qualified and mangled.
///
/// The end-to-end half — actually running `load function <sanitized name>` and
/// getting a body — is `tests/stages/kuna-symbolnamechars.xml`, which does it
/// five times.
#[test]
fn every_reported_name_is_addressable_and_unique() {
    if !specs_available() {
        return;
    }
    for bin in ["hostile_symname_x86_64", "anon_namespace_x86_64", "cpp_mangled_x86_64"] {
        let path = fixture(bin);
        let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
        let root = repo_root();
        let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
        std::env::remove_var(SYMBOLNAMECHARS_ENV); // the shipped default
        let prog = bootstrap_from_object(&path, "", &spec_roots)
            .unwrap_or_else(|e| panic!("{bin} must load: {}", e.explain()));
        let mut seen: std::collections::HashMap<&str, u64> = std::collections::HashMap::new();
        let mut count = 0usize;
        for (name, addr) in prog.function_entries() {
            count += 1;
            assert!(
                !name.is_empty() && !name.chars().any(|c| c.is_whitespace() || c.is_control()),
                "{bin}: {name:?} cannot be passed to `load function` -- read_token splits \
                 on whitespace, so this name is unaddressable from the CLI and the console"
            );
            assert!(
                prog.lookup_symbol(name).is_some(),
                "{bin}: {name:?} does not resolve through lookup_symbol"
            );
            if let Some(prev) = seen.insert(name, addr.get_offset()) {
                assert_eq!(
                    prev,
                    addr.get_offset(),
                    "{bin}: two DIFFERENT functions share the name {name:?} -- the \
                     export would carry a C redefinition and `load function {name}` \
                     could only ever reach one of them"
                );
            }
        }
        assert!(count > 0, "{bin} reported no functions");
    }
}
