//! The parallel driver's output does not depend on how its batches fall on
//! engines: a reused engine runs a batch as a fresh one does, and a batch
//! repeated after a divergent read delivers what the single run writes.
//!
//! Fixtures: `tests/data/driver_determinism/<name>`, compiled by kotlinc 2.0.0
//! (`-jvm-target 17 -no-reflect`) from the sources in its `src/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::kuna_partition::{self, DivergentRead, RootPos, RootSelection};

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

fn names(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|s| s.to_string()).collect()
}

fn order(fernflower: &mut Fernflower) -> BTreeMap<String, RootPos> {
    fernflower
        .root_classes_in_units()
        .into_iter()
        .enumerate()
        .map(|(index, (unit, name))| (name, RootPos { unit, index }))
        .collect()
}

/// One selected run under the divergent-read detector, with the full run's order.
fn watched_run(
    input: &InputSet,
    opts: &[(String, String)],
    selection: RootSelection,
) -> (
    Vec<DecompiledClass>,
    Vec<DivergentRead>,
    BTreeMap<String, RootPos>,
) {
    with_engine_stack(|| {
        let mut fernflower = engine(input, opts);
        let order = order(&mut fernflower);
        kuna_partition::watch(Arc::new(order.clone()), &selection);
        let mut units = Vec::new();
        fernflower.decompile_selected(&selection, &mut |u| units.push(u));
        let reads = kuna_partition::unwatch();
        close(fernflower);
        (units, reads, order)
    })
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

/// `partition`: `SortingKt` creates `KeysKt`'s inline-function object, which
/// the full run writes after `KeysKt` itself; whether `KeysKt` was written
/// decides how the object is written.
#[test]
fn a_root_the_single_run_wrote_first_is_written_in_the_batch() {
    let input = input("partition");
    let opts = options(&[]);
    let full = decompile(&input, &opts);
    let reference = content(&full, "pkg/SortingKt");
    let processed_only = RootSelection {
        process: names(&["pkg/KeysKt", "pkg/SortingKt"]),
        deliver: names(&["pkg/SortingKt"]),
        write: BTreeSet::new(),
    };
    let (units, reads, order) = watched_run(&input, &opts, processed_only);
    assert_ne!(content(&units, "pkg/SortingKt"), reference);
    assert!(!reads.is_empty());
    assert!(
        reads.iter().all(
            |r| r.owner == "pkg/KeysKt" && kuna_partition::owner_written_in_full_run(&order, r)
        ),
        "{reads:?}"
    );
    let written = RootSelection {
        process: names(&["pkg/KeysKt", "pkg/SortingKt"]),
        deliver: names(&["pkg/SortingKt"]),
        write: names(&["pkg/KeysKt"]),
    };
    let (units, reads, _) = watched_run(&input, &opts, written);
    assert!(reads.is_empty(), "{reads:?}");
    assert_eq!(content(&units, "pkg/SortingKt"), reference);
}
