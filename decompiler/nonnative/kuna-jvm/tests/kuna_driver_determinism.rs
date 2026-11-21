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
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_driver::{self, DriverConfig, Jobs};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::kuna_partition::{self, DivergentRead, RootPos, RootSelection};

use crate::kuna_support::{decompile, options, RecordingLogger};

fn input(name: &str) -> InputSet {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/driver_determinism")
        .join(name);
    kuna_input::load(&dir, &LoadOptions::default()).unwrap()
}

fn engine(input: &InputSet, opts: &[(String, String)], fixes: Option<KunaFixes>) -> Fernflower {
    reset_engine_thread();
    let logger = Rc::new(RecordingLogger::new());
    let fernflower = match fixes {
        Some(fixes) => Fernflower::new_with_fixes(opts, logger, fixes),
        None => Fernflower::new(opts, logger),
    };
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
        let mut fernflower = engine(input, opts, None);
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
    fixes: Option<KunaFixes>,
    selection: RootSelection,
) -> (
    Vec<DecompiledClass>,
    Vec<DivergentRead>,
    BTreeMap<String, RootPos>,
) {
    with_engine_stack(|| {
        let mut fernflower = engine(input, opts, fixes);
        let order = order(&mut fernflower);
        kuna_partition::watch(Arc::new(order.clone()), &selection);
        let mut units = Vec::new();
        fernflower.decompile_selected(&selection, &mut |u| units.push(u));
        let reads = kuna_partition::unwatch();
        close(fernflower);
        (units, reads, order)
    })
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

/// `partition`: `SortingKt` creates `KeysKt`'s inline-function object, which
/// the full run writes after `KeysKt` itself; whether `KeysKt` was written
/// decides how the object is written (without the fix
/// `kt_reused_inline_object`, which writes the call `byKey(key)` either way).
#[test]
fn a_root_the_single_run_wrote_first_is_written_in_the_batch() {
    let input = input("partition");
    let opts = options(&[]);
    let fixes = Some(KunaFixes::DEFAULT.without(KunaFix::KtReusedInlineObject));
    let full = with_engine_stack(|| {
        let mut fernflower = engine(&input, &opts, fixes);
        let mut units = Vec::new();
        fernflower.decompile_context(&mut |u| units.push(u));
        close(fernflower);
        units
    });
    let reference = content(&full, "pkg/SortingKt");
    let processed_only = RootSelection {
        process: names(&["pkg/KeysKt", "pkg/SortingKt"]),
        deliver: names(&["pkg/SortingKt"]),
        write: BTreeSet::new(),
    };
    let (units, reads, order) = watched_run(&input, &opts, fixes, processed_only);
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
    let (units, reads, _) = watched_run(&input, &opts, fixes, written);
    assert!(reads.is_empty(), "{reads:?}");
    assert_eq!(content(&units, "pkg/SortingKt"), reference);
}

/// `partition`: `Broadcast`'s primary constructor calls the default-arguments
/// constructor of `ZChannel`, which the Kotlin plugin looks up without its
/// marker parameter; the lookup throws in compat mode only while `ZChannel`
/// is processed, as it is in the full run.
#[test]
fn a_constructor_the_root_lacks_makes_its_state_read() {
    let input = input("partition");
    let opts = options(&[("jvm_vineflower_compat", "on")]);
    let full = decompile(&input, &opts);
    let reference = content(&full, "pkg/Broadcast");
    assert!(
        reference.contains(
            "Constructor pkg/ZChannel.<init>(ILkotlin/jvm/functions/Function1;I)V not found"
        ),
        "{reference}"
    );
    let alone = RootSelection::only(names(&["pkg/Broadcast"]));
    let (units, reads, _) = watched_run(&input, &opts, None, alone);
    assert_ne!(content(&units, "pkg/Broadcast"), reference);
    assert!(reads.iter().any(|r| r.owner == "pkg/ZChannel"), "{reads:?}");
    let with_owner = RootSelection {
        process: names(&["pkg/Broadcast", "pkg/ZChannel"]),
        deliver: names(&["pkg/Broadcast"]),
        write: BTreeSet::new(),
    };
    let (units, reads, _) = watched_run(&input, &opts, None, with_owner);
    assert!(reads.is_empty(), "{reads:?}");
    assert_eq!(content(&units, "pkg/Broadcast"), reference);
}

/// `partition`: the companion object of `Gen` overrides `Gen.next(until)`,
/// and `GenKt` declares the extension `Gen.next(range)`, which `User` calls;
/// writing `Gen` marked its `next(Int)` hidden (compat mode still does). The
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
    for opts in [options(&[]), options(&[("jvm_vineflower_compat", "on")])] {
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
