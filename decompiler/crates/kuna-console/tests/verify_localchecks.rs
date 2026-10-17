//! A `name`/`type` directive is checked against the C its replay printed
//! (`assertions::check_printed`): a directive whose local the output does not
//! declare under its name, or with its type, is reported, never `applied`.
//!
//! Fixture: `kuna-analysis/tests/fixtures/stablenames_x86_64`'s `mix`. The
//! check is run against the C of the pass the directives were applied to,
//! which still prints the old names and types, so those expectations are unmet.

use std::path::PathBuf;

use kuna_console::assertions;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, EntrySelector};
use kuna_console::grammar::{parse_type, DataOrg};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn refusals(
    prog: &kuna_console::engine::ConsoleProgram,
    fd: &kuna_decomp::funcdata::Funcdata,
    code: &str,
    expected: &[assertions::Expected],
) -> Vec<(usize, String)> {
    assertions::check_printed(prog.arch(), fd, code, expected)
        .into_iter()
        .filter_map(|(i, shown)| match shown {
            assertions::Shown::Refused(reason) => Some((i, reason)),
            assertions::Shown::AppliedAs(_) => None,
        })
        .collect()
}

#[test]
fn a_directive_the_output_does_not_show_is_reported() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/stablenames_x86_64");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .unwrap_or_else(|e| panic!("bootstrap stablenames_x86_64: {}", e.explain()));
    prog.commit_pending_analysis().expect("analysis commit");
    let entry = prog.resolve_entry(&EntrySelector::Name("mix".into())).expect("the fixture has a `mix`");
    let mut fd = decompile_one(
        prog.arch_mut(),
        &entry.name,
        entry.addr.clone(),
        entry.size as i32,
        &DecompileSeed::plain(&[], &[]),
        &[],
    )
    .result
    .expect("mix decompiles");
    let (addr_size, word_size) = prog.arch().data_org();
    let (float8, _) = parse_type("double", prog.arch().types(), DataOrg { addr_size, word_size }).expect("parses");

    let renamed = kuna_console::kuna_hightarget::apply_local(&mut fd, "v5", "hash", None, 8, None)
        .expect("applies")
        .expect("binds a Symbol");
    let retyped = kuna_console::kuna_hightarget::apply_local(&mut fd, "v7", "", Some(float8.clone()), 8, None)
        .expect("applies")
        .expect("binds a Symbol");
    let expected = assertions::expected_locals(&fd, &[(0, renamed, None), (1, retyped, Some(float8))]);
    let code = kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd);
    let unapplied = refusals(&prog, &fd, &code, &expected);
    assert_eq!(unapplied.len(), 2, "{unapplied:?}");
    assert!(unapplied.contains(&(0, "nothing is declared as `hash` after it".into())), "{unapplied:?}");
    assert!(unapplied.iter().any(|(i, reason)| *i == 1 && reason.starts_with("`v7` is declared as")), "{unapplied:?}");

    let untouched = kuna_console::kuna_hightarget::apply_local(&mut fd, "v2", "v2", None, 8, None)
        .expect("applies")
        .expect("binds a Symbol");
    let expected = assertions::expected_locals(&fd, &[(2, untouched, None)]);
    assert!(refusals(&prog, &fd, &code, &expected).is_empty());
}
