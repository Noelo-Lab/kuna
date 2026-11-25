//! Each function of a project export carries only the helpers its own C calls:
//! the printer's set starts empty with every function it prints, so `_start`,
//! printed after `scale`, does not inherit `scale`'s SCARRY4.
use kuna_console::engine::bootstrap_from_object;
use kuna_console::project::{decompile_export_targets, FuncResult};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn helpers<'r>(results: &'r [FuncResult], name: &str) -> Vec<&'r str> {
    let result = results.iter().find(|r| r.name == name).unwrap_or_else(|| panic!("no {name}"));
    result.call_declarations.iter().map(String::as_str).filter(|d| d.ends_with("(KUNA_ANY_ARGS);")).collect()
}

#[test]
fn each_function_declares_only_the_helpers_its_own_c_calls() {
    let root = repo_root();
    let specs = vec![root.join("specs").to_str().unwrap().to_string()];
    let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/checked_arith_x86_64");
    for reversed in [false, true] {
        let mut prog = bootstrap_from_object(binary.to_str().unwrap(), "", &specs)
            .expect("bootstrap fixture with built processor specs");
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        let mut targets = prog.function_entries_executable();
        if reversed {
            targets.reverse();
        }
        let results = decompile_export_targets(&mut prog, targets);
        let scale = helpers(&results, "scale");
        assert!(scale.contains(&"bool SCARRY4(KUNA_ANY_ARGS);"), "{scale:?}");
        assert_eq!(helpers(&results, "_start"), Vec::<&str>::new(), "reversed={reversed}");
    }
}
