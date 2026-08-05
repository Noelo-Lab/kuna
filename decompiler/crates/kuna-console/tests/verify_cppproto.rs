//! End-to-end gate for the DWARF **C++ prototype** arm (`--option cppproto`).
//!
//! The bug: `DwarfPass` keyed every recovery off a subprogram DIE's own
//! `DW_AT_name`. C++ splits a definition from its declaration — an out-of-line
//! member or namespace definition is emitted with only `DW_AT_specification`, and
//! a concrete out-of-line instance of an inlined function with only
//! `DW_AT_abstract_origin` — so those DIEs carry no name and were dropped whole,
//! taking the typed signature and the named stack locals with them. On top of
//! that the prototype was parked by NAME in the global scope while it is read
//! back by ADDRESS, which C++ breaks twice over (a demangled template name is
//! normalized to `maxof`; a qualified name lives in a nested scope).
//!
//! The fixture `cppproto_x86_64` (source vendored alongside) is a `-g` C++ binary
//! whose every interesting function exercises one of those shapes. Each test is
//! TWO-PASS: `option cppproto off` reproduces the bug (`a0`/`a1` parameters,
//! `void *` for a class pointer), the default reproduces the fix.
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

/// Bootstrap the C++ fixture with `cppproto` set to `on`/`off`, commit the DWARF
/// facts under that gate, decompile `func` and return the captured C.
/// `None` => specs-less skip.
fn decompile(func: &str, cppproto: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cppproto_x86_64")
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_cppproto: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The gate is read at the analysis COMMIT boundary (the producing pass runs at
    // `load file`, upstream of any `option` command), so it must be set here.
    prog.arch_mut()
        .set_kuna_option("cppproto", if cppproto { "on" } else { "off" })
        .expect("cppproto is a registered option");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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
    Some(status.optr.clone())
}

/// The headline: an out-of-line member definition (`DW_AT_specification`, plus an
/// artificial `this` typed by a `DW_TAG_class_type`).
#[test]
fn out_of_line_member_recovers_this_and_parameter_names() {
    let Some(off) = decompile("Account::deposit", false) else { return };
    assert!(
        off.contains("Account::deposit(int4 *a0,int4 a1)"),
        "gate off must reproduce the name-only walk's a0/a1 signature, got:\n{off}"
    );

    let on = decompile("Account::deposit", true).expect("second pass bootstraps");
    assert!(
        on.contains("Account::deposit(Account *this,int4 amount)"),
        "gate on must recover `Account *this` + the DWARF parameter name, got:\n{on}"
    );
}

/// A `const` member: `this` is `const Account *const`, a four-DIE qualifier chain
/// that used to exceed the type mapper's depth budget and degrade to `void *`.
#[test]
fn const_member_this_survives_the_qualifier_chain() {
    let Some(off) = decompile("Account::available", false) else { return };
    assert!(
        off.contains("Account::available(int4 *a0)"),
        "gate off signature, got:\n{off}"
    );

    let on = decompile("Account::available", true).expect("second pass bootstraps");
    assert!(
        on.contains("Account::available(Account *this)"),
        "a const method's `this` must still resolve to `Account *`, got:\n{on}"
    );
}

/// A C++ REFERENCE parameter (`DW_TAG_reference_type`) maps to a pointer.
#[test]
fn reference_parameter_maps_to_a_pointer() {
    let Some(on) = decompile("Account::bump", true) else { return };
    assert!(
        on.contains("Account::bump(Account *this,int4 *out)"),
        "`int &out` must render as `int4 *out`, got:\n{on}"
    );
}

/// A `static` member has no artificial `this` and is still an out-of-line
/// definition — proof the drop was never about the `this` DIE.
#[test]
fn static_member_recovers_its_parameter_name() {
    let Some(off) = decompile("Account::make_id", false) else { return };
    assert!(off.contains("Account::make_id(int4 a0)"), "gate off signature, got:\n{off}");

    let on = decompile("Account::make_id", true).expect("second pass bootstraps");
    assert!(
        on.contains("Account::make_id(int4 seed)"),
        "a static member must recover its DWARF parameter name, got:\n{on}"
    );
}

/// A namespace function defined out of line.
#[test]
fn namespaced_function_recovers_its_prototype() {
    let Some(off) = decompile("db::inner::scaled_add", false) else { return };
    assert!(
        off.contains("db::inner::scaled_add(int4 a0,int4 a1)"),
        "gate off signature, got:\n{off}"
    );

    let on = decompile("db::inner::scaled_add", true).expect("second pass bootstraps");
    assert!(
        on.contains("db::inner::scaled_add(int4 a,int4 b)"),
        "a namespaced function must recover its DWARF parameter names, got:\n{on}"
    );
}

/// A template instantiation DOES carry its own `DW_AT_name` (`maxof<int>`), but
/// kuna's symbol table files it as `maxof` — so only the ADDRESS-keyed prototype
/// park reaches it.
#[test]
fn template_instantiation_binds_by_address_not_name() {
    let Some(off) = decompile("maxof<int>", false) else { return };
    assert!(off.contains("maxof<int>(int4 a0,int4 a1)"), "gate off signature, got:\n{off}");

    let on = decompile("maxof<int>", true).expect("second pass bootstraps");
    assert!(
        on.contains("maxof<int>(int4 a,int4 b)"),
        "a template instantiation's prototype must bind by entry address, got:\n{on}"
    );
}

/// A pointer to a `DW_TAG_class_type` — the tag that fell through to `None`, so
/// every `Foo *` parameter degraded to `void *`.
#[test]
fn class_typed_pointer_parameter_keeps_its_class() {
    let Some(off) = decompile("probe_virtual_call", false) else { return };
    assert!(
        off.contains("probe_virtual_call(void *s,int4 scale)"),
        "gate off must degrade the class pointer to `void *`, got:\n{off}"
    );

    let on = decompile("probe_virtual_call", true).expect("second pass bootstraps");
    assert!(
        on.contains("probe_virtual_call(Shape *s,int4 scale)"),
        "`DW_TAG_class_type` must map like a structure, got:\n{on}"
    );
}
