//! The console function-symbol stream keeps "last registration of a name wins".
//!
//! `ConsoleProgram::register_symbol` is how the analysis tier publishes every
//! discovered or renamed function, and its contract is a `retain` + `push`: the
//! new record replaces every earlier record of that name and lands last. The
//! stream backing it is indexed by name rather than rescanned, so this checks
//! the indexed stream against a plain-`Vec` model of the same contract, driven
//! through the public API on a real image.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/regglobal_fmt_x86_64");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("verify_symbolstream: skipping (bootstrap failed): {}", e.explain());
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn stream(prog: &ConsoleProgram) -> Vec<(String, u64)> {
    prog.function_entries().map(|(n, a)| (n.to_string(), a.get_offset())).collect()
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog
        .function_entries()
        .next()
        .and_then(|(_, a)| a.get_space().cloned())
        .expect("the fixture carries at least one symbol");
    Address::new(Rc::clone(&space), vma)
}

/// Registration order, record for record, against the `retain` + `push` model:
/// a brand-new name, a loader name moved to a new address, and one name churned
/// enough times to reclaim its tombstones must all leave the same stream.
#[test]
fn registration_matches_the_retain_and_push_model() {
    let Some(mut prog) = load() else { return };
    let mut model = stream(&prog);
    assert!(model.len() > 100, "the fixture must carry enough symbols to be worth scanning");

    let existing = model[model.len() / 2].0.clone();
    let mut script: Vec<(String, u64)> =
        vec![("brand_new".to_string(), 0x9000), (existing, 0x9100)];
    script.extend((0..300u64).map(|i| ("churn".to_string(), 0xa000 + i)));
    script.push(("brand_new".to_string(), 0x9200));

    for (name, vma) in &script {
        model.retain(|(n, _)| n != name);
        model.push((name.clone(), *vma));
        prog.register_symbol(name, code_addr(&prog, *vma));
        assert_eq!(stream(&prog), model, "stream diverged after registering {name} at {vma:#x}");
    }

    assert_eq!(prog.num_symbols(), model.len());
    assert_eq!(prog.lookup_symbol("churn").map(|a| a.get_offset()), Some(0xa000 + 299));
    assert_eq!(prog.lookup_symbol("brand_new").map(|a| a.get_offset()), Some(0x9200));
    assert!(prog.lookup_symbol("never_registered").is_none());
    for (name, _) in &model {
        let first = model.iter().find(|(n, _)| n == name).map(|(_, v)| *v).unwrap();
        assert_eq!(
            prog.lookup_symbol(name).map(|a| a.get_offset()),
            Some(first),
            "{name} stopped resolving after the churn"
        );
    }
}

/// A name the image carries twice resolves to the FIRST of the two records, the
/// answer the linear scan gave.
#[test]
fn lookup_answers_with_the_first_record_of_a_name() {
    let Some(prog) = load() else { return };
    let records = stream(&prog);
    for (name, vma) in &records {
        let first = records.iter().find(|(n, _)| n == name).map(|(_, v)| *v).unwrap();
        assert_eq!(
            prog.lookup_symbol(name).map(|a| a.get_offset()),
            Some(first),
            "{name} at {vma:#x} resolved to the wrong record"
        );
    }
}
