//! Partitioned runs and the parallel driver: the same output for any number of workers.

use std::collections::BTreeSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_driver::{self, ClassResult, DriverConfig, Jobs, Progress, Report, Strategy};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::kuna_partition::{self, DivergentRead, RootPos, RootSelection};

use crate::kuna_support::{copy_classes, corpus, decompile, options, scratch, RecordingLogger};

fn config(jobs: usize) -> DriverConfig {
    let mut config = DriverConfig::new(
        options(&[]),
        Arc::new(|| Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>),
    );
    config.jobs = Jobs::Exactly(jobs);
    config
}

fn drive(input: &InputSet, config: &DriverConfig) -> (Vec<ClassResult>, Report) {
    let mut results = Vec::new();
    let report = kuna_driver::run(input, config, &mut |r| results.push(r)).unwrap();
    (results, report)
}

fn contents(units: &[DecompiledClass]) -> Vec<(String, String)> {
    units
        .iter()
        .map(|u| (u.qualified_name.clone(), u.content.clone()))
        .collect()
}

#[test]
fn output_is_identical_for_any_worker_count() {
    let input = kuna_input::load(&corpus("java16"), &LoadOptions::default()).unwrap();
    let reference = contents(&decompile(&input, &options(&[])));
    assert!(reference.len() > 30);
    for (jobs, strategy) in [
        (1, Strategy::Closure),
        (2, Strategy::Closure),
        (4, Strategy::Closure),
        (3, Strategy::Components),
    ] {
        let mut config = config(jobs);
        config.strategy = strategy;
        let (results, report) = drive(&input, &config);
        let indices: Vec<usize> = results.iter().map(|r| r.index).collect();
        assert_eq!(indices, (0..results.len()).collect::<Vec<_>>());
        let got: Vec<(String, String)> = results
            .iter()
            .map(|r| {
                let unit = r
                    .unit
                    .as_ref()
                    .unwrap_or_else(|| panic!("{}: {:?}", r.qualified_name, r.error));
                (unit.qualified_name.clone(), unit.content.clone())
            })
            .collect();
        assert_eq!(got.len(), reference.len(), "jobs {jobs}");
        for (mine, theirs) in got.iter().zip(&reference) {
            assert_eq!(mine, theirs, "jobs {jobs} {strategy:?}: {} differs", mine.0);
        }
        assert!(
            report.divergent_reads.is_empty(),
            "{:?}",
            report.divergent_reads
        );
        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert_eq!(report.delivered, reference.len());
        assert_eq!(report.roots.len(), reference.len());
    }
}

fn super_inner_input() -> InputSet {
    let dir = scratch("driver-super-inner");
    copy_classes(
        &dir,
        &[
            "java8/pkg/TestSuperInner.class",
            "java8/pkg/TestSuperInner$Inner2.class",
            "java8/pkg/TestSuperInnerBase.class",
            "java8/pkg/TestSuperInnerBase$Inner.class",
        ],
    );
    kuna_input::load(&dir, &LoadOptions::default()).unwrap()
}

/// One selected run over `input` under the divergent-read detector.
fn selected_run(
    input: &InputSet,
    selection: RootSelection,
) -> (Vec<DecompiledClass>, Vec<DivergentRead>) {
    let opts = options(&[]);
    with_engine_stack(|| {
        reset_engine_thread();
        let mut fernflower = Fernflower::new(&opts, Rc::new(RecordingLogger::new()));
        for source in input.sources() {
            fernflower.add_source(source);
        }
        let order = fernflower
            .root_classes_in_units()
            .into_iter()
            .enumerate()
            .map(|(index, (unit, name))| (name, RootPos { unit, index }))
            .collect();
        kuna_partition::watch(Arc::new(order), &selection);
        let mut units = Vec::new();
        fernflower.decompile_selected(&selection, &mut |u| units.push(u));
        let reads = kuna_partition::unwatch();
        fernflower.clear_context();
        reset_engine_thread();
        (units, reads)
    })
}

#[test]
fn a_cross_root_constructor_read_is_detected_and_resolved() {
    let input = super_inner_input();
    let full = decompile(&input, &options(&[]));
    let reference = full
        .iter()
        .find(|u| u.qualified_name == "pkg/TestSuperInner")
        .unwrap();

    let alone = RootSelection::only(["pkg/TestSuperInner".to_string()]);
    let (units, reads) = selected_run(&input, alone);
    assert_eq!(units.len(), 1);
    assert_eq!(reads.len(), 1, "{reads:?}");
    assert_eq!(reads[0].reader, "pkg/TestSuperInner");
    assert_eq!(reads[0].owner, "pkg/TestSuperInnerBase");
    assert_eq!(reads[0].class, "pkg/TestSuperInnerBase$Inner");

    let with_dependency = RootSelection {
        process: ["pkg/TestSuperInner", "pkg/TestSuperInnerBase"]
            .map(String::from)
            .into(),
        deliver: BTreeSet::from(["pkg/TestSuperInner".to_string()]),
    };
    let (units, reads) = selected_run(&input, with_dependency);
    assert!(reads.is_empty(), "{reads:?}");
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].content, reference.content);

    let mut config = config(2);
    config.select = Some(BTreeSet::from(["pkg/TestSuperInner".to_string()]));
    let (results, report) = drive(&input, &config);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].unit.as_ref().unwrap().content, reference.content);
    assert_eq!(report.processed, 2, "the dependency is pre-processed too");
    assert_eq!(report.repeated, 0);
}

#[test]
fn several_units_keep_their_order() {
    let classes = [
        "java8/pkg/TestSuperInner.class",
        "java8/pkg/TestSuperInner$Inner2.class",
        "java8/pkg/TestSuperInnerBase.class",
        "java8/pkg/TestSuperInnerBase$Inner.class",
    ];
    let dir = scratch("driver-units");
    copy_classes(&dir, &classes);
    let main = dir.join("pkg/TestSuperInner.class");
    let input = kuna_input::load(&main, &LoadOptions::default()).unwrap();
    assert_eq!(input.sources().len(), 2, "the class and its sibling");
    let reference = contents(&decompile(&input, &options(&[])));
    for jobs in [1, 2] {
        let (results, report) = drive(&input, &config(jobs));
        let got: Vec<(String, String)> = results
            .iter()
            .map(|r| {
                let u = r.unit.as_ref().unwrap();
                (u.qualified_name.clone(), u.content.clone())
            })
            .collect();
        assert_eq!(got, reference);
        assert!(report.divergent_reads.is_empty());
    }
}

#[test]
fn progress_selection_and_failure_containment() {
    let input = kuna_input::load(&corpus("java17"), &LoadOptions::default()).unwrap();
    let reference = decompile(&input, &options(&[]));
    let pick: Vec<String> = reference
        .iter()
        .step_by(5)
        .map(|u| u.qualified_name.clone())
        .collect();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut config = config(3);
    config.select = Some(
        pick.iter()
            .cloned()
            .chain(["no/Such".to_string()])
            .collect(),
    );
    let log = seen.clone();
    config.progress = Some(Arc::new(move |p: &Progress| {
        if let Progress::Delivered { class, .. } = p {
            log.lock().unwrap().push(class.clone());
        }
    }));
    let victim = pick
        .iter()
        .find(|name| {
            let unit = reference
                .iter()
                .find(|u| &u.qualified_name == *name)
                .unwrap();
            unit.methods
                .iter()
                .any(|m| &m.class == *name && !m.lines.is_empty())
        })
        .expect("a picked class with code")
        .clone();
    let target = victim.clone();
    config.method_guard = Some(Arc::new(move || {
        let target = target.clone();
        Rc::new(move || {
            let class = kuna_jvm::decompiler_context::DecompilerContext::get_current_context()
                .and_then(|_| kuna_jvm::decompiler_context::DecompilerContext::get_current_class())
                .map(|c| c.qualified_name());
            (class.as_deref() == Some(target.as_str())).then(|| "aborted by test".to_string())
        }) as Rc<dyn Fn() -> Option<String>>
    }));
    let (results, report) = drive(&input, &config);
    let names: Vec<&str> = results.iter().map(|r| r.qualified_name.as_str()).collect();
    let mut expected: Vec<&str> = pick.iter().map(String::as_str).collect();
    expected.sort_by_key(|n| reference.iter().position(|u| u.qualified_name == *n));
    assert_eq!(names, expected);
    assert_eq!(*seen.lock().unwrap(), expected);
    assert_eq!(report.failed, ["no/Such"]);
    for result in &results {
        let unit = result.unit.as_ref().unwrap();
        let theirs = reference
            .iter()
            .find(|u| u.qualified_name == unit.qualified_name)
            .unwrap();
        if unit.qualified_name == victim {
            assert!(unit
                .methods
                .iter()
                .filter(|m| m.error.is_some())
                .all(|m| m.error.as_deref() == Some("MethodAbortedException: aborted by test")));
            assert!(unit.methods.iter().any(|m| m.error.is_some()));
        } else {
            assert_eq!(unit.content, theirs.content);
        }
    }
}

#[test]
fn auto_jobs_fit_the_memory() {
    let input = kuna_input::load(&corpus("java11"), &LoadOptions::default()).unwrap();
    let mut config = config(1);
    config.jobs = Jobs::Auto;
    config.available_memory = Some(1);
    let (results, report) = drive(&input, &config);
    assert_eq!(report.workers, 1);
    assert!(results.iter().all(|r| r.unit.is_some()));
    let small = kuna_driver::estimate_memory(&input, &config, 1);
    assert!(kuna_driver::estimate_memory(&input, &config, 8) > small);
    config.available_memory = Some(small * 100);
    let (_, report) = drive(&input, &config);
    assert!(report.workers >= 1 && report.workers <= kuna_driver::MAX_AUTO_JOBS);
}
