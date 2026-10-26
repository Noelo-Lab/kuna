//! Name lookups reuse the canonical entry list until symbols or declared names
//! change, and an explicit `0x` prototype operand skips the name lookup only when
//! no symbol is spelled that way (issue #1255).
//!
//! Fixture: `kuna-analysis/tests/fixtures/fauxware`, a small unstripped x86-64 ELF
//! whose `authenticate` sits at 0x400664 and calls `strcmp` through its PLT stub.

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn load() -> ConsoleProgram {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware");
    let mut prog = bootstrap_from_object(bin.to_str().expect("UTF-8 fixture path"), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit");
    prog
}

fn named(prog: &ConsoleProgram, name: &str) -> kuna_console::engine::FunctionEntry {
    prog.resolve_entry(&EntrySelector::Name(name.to_string()))
        .unwrap_or_else(|err| panic!("{name} does not resolve: {err}"))
}

/// A name lookup before a declaration must not hide the declaration from the next
/// lookup, and the declared name becomes the entry's reported name.
#[test]
fn lookups_after_a_declaration_see_the_declared_name() {
    let mut prog = load();
    let before = named(&prog, "authenticate");
    assert_eq!(before.name, "authenticate");
    assert!(prog.resolve_entry(&EntrySelector::Name("checked_login".into())).is_err());

    prog.declare_function(before.addr.clone(), Some("checked_login"), 0)
        .expect("declare a second name at authenticate's entry");

    let declared = named(&prog, "checked_login");
    assert_eq!(declared.addr, before.addr);
    let old = named(&prog, "authenticate");
    assert_eq!(old.addr, before.addr);
    assert_eq!(old.name, "checked_login", "the declared name did not become the entry's name");
}

/// A symbol spelled exactly like an address still wins over the address form:
/// the prototype lands on the function carrying that name, not on the function
/// at that address.
#[test]
fn a_symbol_spelled_like_an_address_takes_the_prototype() {
    let mut prog = load();
    let strcmp = named(&prog, "strcmp");
    prog.declare_function(strcmp.addr.clone(), Some("0x400664"), 0)
        .expect("name the strcmp stub like authenticate's address");
    prog.set_assertions(vec![Directive {
        raw: "prototype 0x400664 void *hashit(void *out,void *input)".into(),
        body: Body::Prototype {
            func: "0x400664".into(),
            decl: "void *hashit(void *out,void *input)".into(),
        },
    }]);
    assertions::apply_program_scoped(&mut prog);
    for outcome in prog.assertion_outcomes() {
        assert_eq!(outcome.status, "applied", "{:?}: {:?}", outcome.directive, outcome.detail);
    }
    let authenticate = named(&prog, "authenticate");
    let funcs = decompile_targets(&mut prog, vec![authenticate], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    assert!(
        !code.contains("authenticate(void *out,void *input)"),
        "the prototype reached the function at the address instead of the named symbol:\n{code}"
    );
}

/// An extent declared on its own (`load addr ram,<entry>,<size>` records one
/// without touching any symbol) reaches the next listing's size.
#[test]
fn an_extent_declared_after_a_listing_reaches_the_next_listing() {
    let mut prog = load();
    let entry = named(&prog, "authenticate");
    let vma = entry.addr.get_offset();
    let size_of = |prog: &ConsoleProgram| {
        prog.function_entries_canonical()
            .into_iter()
            .find(|e| e.addr.get_offset() == vma)
            .expect("authenticate in the listing")
            .size
    };
    assert_ne!(size_of(&prog), 3);
    prog.declare_extent(vma, 3);
    assert_eq!(size_of(&prog), 3, "the listing kept the extent from before the declaration");
}
