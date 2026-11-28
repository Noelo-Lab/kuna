//! The parallel driver's output does not depend on how its batches fall on
//! engines: a reused engine runs a batch as a fresh one does, and every worker
//! count delivers what the single run writes.
//!
//! Fixtures: `tests/data/driver_determinism/<name>`, compiled by kotlinc 2.0.0
//! (`-jvm-target 17 -no-reflect`) from the sources in its `src/`.

use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_driver::{self, DriverConfig, Jobs};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::kuna_partition::RootSelection;

use crate::kuna_support::{decompile, options, RecordingLogger};

fn input(name: &str) -> InputSet {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/driver_determinism")
        .join(name);
    kuna_input::load(&dir, &LoadOptions::default()).unwrap()
}

fn engine(input: &InputSet, opts: &[(String, String)]) -> Fernflower {
    reset_engine_thread();
    let fernflower = Fernflower::new(opts, Rc::new(RecordingLogger::new()));
    for source in input.sources() {
        fernflower.add_source(source);
    }
    fernflower
}

fn close(mut fernflower: Fernflower) {
    fernflower.clear_context();
    reset_engine_thread();
}

/// Runs `batches` one after the other on one engine, as a reused driver worker does.
fn reused_engine(
    input: &InputSet,
    opts: &[(String, String)],
    batches: &[&[&str]],
) -> Vec<Vec<DecompiledClass>> {
    with_engine_stack(|| {
        let mut fernflower = engine(input, opts);
        let mut out = Vec::new();
        for batch in batches {
            let mut units = Vec::new();
            let selection = RootSelection::only(batch.iter().map(|s| s.to_string()));
            fernflower.decompile_selected(&selection, &mut |u| units.push(u));
            out.push(units);
        }
        close(fernflower);
        out
    })
}

fn content<'a>(units: &'a [DecompiledClass], name: &str) -> &'a str {
    &units
        .iter()
        .find(|u| u.qualified_name == name)
        .unwrap_or_else(|| panic!("{name} not delivered"))
        .content
}

fn driven(input: &InputSet, opts: &[(String, String)], jobs: usize) -> Vec<(String, String)> {
    let mut config = DriverConfig::new(
        opts.to_vec(),
        Arc::new(|| Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>),
    );
    config.jobs = Jobs::Exactly(jobs);
    let mut out = Vec::new();
    kuna_driver::run(input, &config, &mut |r| {
        let unit = r
            .unit
            .unwrap_or_else(|| panic!("{}: {:?}", r.qualified_name, r.error));
        out.push((unit.qualified_name, unit.content));
    })
    .unwrap();
    out
}

/// `reuse`: `Shifted` has an inline-class constructor parameter, so kotlinc
/// adds a synthetic accessor constructor, which `TestStaleSequence.make`
/// calls; recognising the accessor reads its code, which the engine expanded
/// in the batch that decompiled `Shifted` (the class scope since reset).
#[test]
fn a_method_sequence_from_an_earlier_batch_is_not_reused() {
    let input = input("reuse");
    let opts = options(&[]);
    let full = decompile(&input, &opts);
    let reference = content(&full, "pkg/TestStaleSequence");
    assert!(reference.contains("Shifted(base, span)"), "{reference}");

    let runs = reused_engine(
        &input,
        &opts,
        &[&["pkg/Shifted"], &["pkg/TestStaleSequence"]],
    );
    assert_eq!(
        content(&runs[0], "pkg/Shifted"),
        content(&full, "pkg/Shifted")
    );
    assert_eq!(content(&runs[1], "pkg/TestStaleSequence"), reference);
}

/// `partition`: the companion object of `Gen` overrides `Gen.next(until)`,
/// and `GenKt` declares the extension `Gen.next(range)`, which `User` calls;
/// writing `Gen` marked its `next(Int)` hidden (Vineflower still does). The
/// member cannot take an `IntRange`, so the call is written on `gen`.
#[test]
fn a_companion_override_does_not_hide_the_member_from_other_classes() {
    let input = input("partition");
    let opts = options(&[]);
    let full = decompile(&input, &opts);
    let reference = content(&full, "pkg/User");
    assert!(reference.contains("gen.next(range)"), "{reference}");
    let after_gen = reused_engine(&input, &opts, &[&["pkg/Gen", "pkg/User"]]);
    assert_eq!(content(&after_gen[0], "pkg/User"), reference);
    let alone = reused_engine(&input, &opts, &[&["pkg/User"]]);
    assert_eq!(content(&alone[0], "pkg/User"), reference);
}

#[test]
fn every_worker_count_delivers_the_single_run() {
    let input = input("partition");
    for opts in [options(&[])] {
        let reference: Vec<(String, String)> = decompile(&input, &opts)
            .into_iter()
            .map(|u| (u.qualified_name, u.content))
            .collect();
        assert_eq!(reference.len(), 7);
        for jobs in [1, 2, 4] {
            assert_eq!(
                driven(&input, &opts, jobs),
                reference,
                "jobs {jobs}, {opts:?}"
            );
        }
    }
}
