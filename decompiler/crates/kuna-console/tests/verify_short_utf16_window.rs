//! Regression fence for an explicit two-byte `wchar_t` data declaration at a
//! short UTF-16 string. Operand-reference analysis once mapped the string's first
//! unit as a one-byte short string; it now declines a run that opens an array of
//! 2-byte character codes, and `tests/stages/kuna-short-utf16-window.xml` keeps
//! the collision of the two mappings.

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn load() -> ConsoleProgram {
    let root = repo_root();
    let specs = vec![root.join("specs").to_str().unwrap().to_string()];
    let fixture = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/short_utf16_window_pe_x86_64.exe");
    let mut prog = bootstrap_from_object(fixture.to_str().unwrap(), "", &specs)
        .unwrap_or_else(|e| panic!("bootstrap short UTF-16 PE fixture: {}", e.explain()));
    prog.arch_mut()
        .set_kuna_option("operand_refs", "on")
        .expect("operand_refs is registered");
    prog.commit_pending_analysis().expect("analysis commit");
    prog
}

fn directive(raw: &str, body: Body) -> Directive {
    Directive {
        raw: raw.to_string(),
        body,
    }
}

fn decompile(mut prog: ConsoleProgram) -> String {
    let entry = prog
        .resolve_entry(&EntrySelector::Numeric(0x401000))
        .expect("fixture entry");
    decompile_targets(&mut prog, vec![entry], false, false, false)[0]
        .code
        .clone()
        .unwrap_or_default()
}

#[test]
fn explicit_wchar_data_types_a_short_utf16_window_and_preserves_controls() {
    let baseline = decompile(load());
    assert!(
        baseline.contains("sub_401040(0x402000,0)"),
        "the short UTF-16 window read as a narrow string:\n{baseline}"
    );
    assert!(
        baseline.contains("L\"OLLYDBG\""),
        "adjacent wide control regressed:\n{baseline}"
    );
    assert!(
        baseline.contains("\"ASCII\""),
        "ASCII control regressed:\n{baseline}"
    );

    let mut asserted = load();
    asserted.set_assertions(vec![directive(
        "data 0x402000 wchar_t window_class[3]",
        Body::Data {
            addr: 0x402000,
            decl: "wchar_t window_class[3]".into(),
        },
    )]);
    assertions::apply_program_scoped(&mut asserted);
    let outcomes = asserted.assertion_outcomes();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(
        outcomes[0].status, "applied",
        "assertion report: {outcomes:?}"
    );
    let code = decompile(asserted);

    assert!(
        code.contains("sub_401040(L\"ID\",0)"),
        "wide declaration was ignored:\n{code}"
    );
    assert!(
        !code.contains("\"I\""),
        "narrow short string survived:\n{code}"
    );
    assert!(
        code.contains("L\"OLLYDBG\""),
        "adjacent wide control changed:\n{code}"
    );
    assert!(code.contains("\"ASCII\""), "ASCII control changed:\n{code}");
}
