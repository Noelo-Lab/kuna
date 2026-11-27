//! The parallel driver with renaming and with failing engines: renamed roots
//! are planned and delivered, renaming runs once per engine, an engine whose
//! batch unwound is not reused, a worker that panics outside the engine's catch
//! does not leave the caller waiting, and a method that throws before its
//! body still ends its logger method.

use std::cell::Cell;
use std::collections::BTreeSet;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::time::Duration;

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, Severity};
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_driver::{self, ClassResult, DriverConfig, Jobs, LoggerFactory, Report};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::kuna_partition::RootSelection;
use kuna_jvm::rt::Caught;

use crate::kuna_support::{corpus, decompile, options, RecordingLogger};

fn data(rel: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(rel)
}

fn contents(units: &[DecompiledClass]) -> Vec<(String, String)> {
    units
        .iter()
        .map(|u| (u.qualified_name.clone(), u.content.clone()))
        .collect()
}

fn config(opts: Vec<(String, String)>, jobs: usize, logger: LoggerFactory) -> DriverConfig {
    let mut config = DriverConfig::new(opts, logger);
    config.jobs = Jobs::Exactly(jobs);
    config
}

fn recording() -> LoggerFactory {
    Arc::new(|| Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>)
}

/// Runs the driver on another thread; `None` when it has not returned after
/// `seconds`.
fn drive_within(
    input: &InputSet,
    config: DriverConfig,
    seconds: u64,
) -> Option<(Vec<ClassResult>, Report)> {
    let (tx, rx) = mpsc::channel();
    let input = input.clone();
    std::thread::spawn(move || {
        let mut results = Vec::new();
        let report = kuna_driver::run(&input, &config, &mut |r| results.push(r)).unwrap();
        let _ = tx.send((results, report));
    });
    rx.recv_timeout(Duration::from_secs(seconds)).ok()
}

/// A logger whose engine fails writing any class: the failure escapes the
/// class's own catch, so the batch unwinds out of `decompile_selected`.
struct FailingWriter(Cell<Severity>);

impl IFernflowerLogger for FailingWriter {
    fn severity(&self) -> &Cell<Severity> {
        &self.0
    }

    fn write_message(&self, _message: &str, _severity: Severity) {}

    fn write_message_ex(&self, message: &str, _severity: Severity, _t: &Caught) {
        panic!("test logger: {message}");
    }

    fn start_reading_class(&self, class_name: &str) {
        panic!("test logger: writing {class_name}");
    }
}

fn failing_writer() -> Rc<dyn IFernflowerLogger> {
    Rc::new(FailingWriter(Cell::new(Severity::Error)))
}

/// Every delivered unit equals the single run's; returns the roots that failed.
fn check_against(results: &[ClassResult], reference: &[DecompiledClass]) -> Vec<String> {
    let mut failed = Vec::new();
    for result in results {
        match &result.unit {
            Some(unit) => {
                let theirs = reference
                    .iter()
                    .find(|u| u.qualified_name == unit.qualified_name)
                    .unwrap();
                assert_eq!(unit.content, theirs.content, "{}", unit.qualified_name);
            }
            None => failed.push(result.qualified_name.clone()),
        }
    }
    failed
}

#[test]
fn renamed_roots_are_planned_and_delivered() {
    let input = kuna_input::load(&data("renamed_roots"), &LoadOptions::default()).unwrap();
    let opts = options(&[("ren", "1")]);
    let reference = contents(&decompile(&input, &opts));
    assert_eq!(reference.len(), 7);
    assert!(reference.iter().any(|(name, _)| name == "o/class_0"));
    for jobs in [1, 2] {
        let mut results = Vec::new();
        let report =
            kuna_driver::run(&input, &config(opts.clone(), jobs, recording()), &mut |r| {
                results.push(r)
            })
            .unwrap();
        assert_eq!(report.roots.len(), 7, "jobs {jobs}");
        let got: Vec<(String, String)> = results
            .iter()
            .map(|r| {
                let unit = r.unit.as_ref().unwrap();
                (unit.qualified_name.clone(), unit.content.clone())
            })
            .collect();
        assert_eq!(got, reference, "jobs {jobs}");
    }

    let mut selected = config(opts, 2, recording());
    selected.select = Some(BTreeSet::from(["o/a".to_string()]));
    let mut results = Vec::new();
    let report = kuna_driver::run(&input, &selected, &mut |r| results.push(r)).unwrap();
    assert!(report.failed.is_empty(), "{:?}", report.failed);
    assert_eq!(results.len(), 1);
    let unit = results[0].unit.as_ref().unwrap();
    assert!(reference.contains(&(unit.qualified_name.clone(), unit.content.clone())));
}

/// Renaming reloads the context (`Scanning classes from` per unit); a second
/// batch on the same engine must not rename and reload again.
#[test]
fn an_engine_renames_once_for_all_its_batches() {
    let input = kuna_input::load(&data("renamed_roots"), &LoadOptions::default()).unwrap();
    let opts = options(&[("ren", "1")]);
    let reference = contents(&decompile(&input, &opts));
    let (units, scans) = with_engine_stack(|| {
        reset_engine_thread();
        let logger = Rc::new(RecordingLogger::new());
        let mut fernflower = Fernflower::new(&opts, logger.clone());
        for source in input.sources() {
            fernflower.add_source(source);
        }
        let scanned = || {
            logger
                .records
                .borrow()
                .iter()
                .filter(|r| r.message.starts_with("Scanning classes from"))
                .count()
        };
        fernflower.load();
        let mut scans = vec![scanned()];
        let mut units = Vec::new();
        for (root, _) in &reference[..3] {
            let selection = RootSelection::only([root.clone()]);
            fernflower.decompile_selected(&selection, &mut |u| units.push(u));
            scans.push(scanned());
        }
        fernflower.clear_context();
        reset_engine_thread();
        (units, scans)
    });
    assert_eq!(scans, [2, 2, 2, 2], "added once, reloaded once by the renamer");
    assert_eq!(units.len(), 3);
    for (name, content) in contents(&units) {
        assert!(reference.contains(&(name.clone(), content)), "{name}");
    }
}

#[test]
fn an_engine_whose_batch_unwound_is_not_reused() {
    let input = kuna_input::load(&corpus("java17"), &LoadOptions::default()).unwrap();
    let reference = decompile(&input, &options(&[]));
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    let logger: LoggerFactory = Arc::new(move || {
        if counter.fetch_add(1, Ordering::SeqCst) == 0 {
            failing_writer()
        } else {
            Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>
        }
    });
    let (results, report) = drive_within(&input, config(options(&[]), 1, logger), 300).unwrap();
    assert_eq!(results.len(), reference.len());
    assert!(report.runs > 1);
    assert_eq!(report.instances, 2, "a fresh engine after the failed batch");
    let failed = check_against(&results, &reference);
    assert!(!failed.is_empty() && failed.len() < results.len(), "{failed:?}");
    for result in results.iter().filter(|r| r.unit.is_none()) {
        let error = result.error.as_deref().unwrap_or_default();
        assert!(error.starts_with("the engine failed"), "{error}");
    }
}

#[test]
fn a_worker_panic_outside_the_engine_does_not_block_the_caller() {
    let input = kuna_input::load(&corpus("java17"), &LoadOptions::default()).unwrap();
    let reference = decompile(&input, &options(&[]));
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let opened = Arc::new(AtomicUsize::new(0));
    let (gate_in, opened_in) = (gate.clone(), opened.clone());
    let logger: LoggerFactory = Arc::new(move || {
        let (lock, changed) = &*gate_in;
        if std::thread::current().name() == Some("kuna-jvm-0") {
            if opened_in.fetch_add(1, Ordering::SeqCst) == 0 {
                return failing_writer();
            }
            *lock.lock().unwrap() = true;
            changed.notify_all();
            panic!("test logger factory: the worker panics holding a batch");
        }
        let open = lock.lock().unwrap();
        let _open = changed
            .wait_timeout_while(open, Duration::from_secs(120), |open| !*open)
            .unwrap();
        Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>
    });
    let (results, report) = drive_within(&input, config(options(&[]), 2, logger), 300)
        .expect("the driver returned after a worker panicked");
    assert_eq!(results.len(), reference.len());
    assert_eq!(opened.load(Ordering::SeqCst), 2);
    assert_eq!(report.workers, 2);
    let failed = check_against(&results, &reference);
    assert!(failed.len() < results.len(), "{failed:?}");
    assert!(results
        .iter()
        .any(|r| r.error.as_deref() == Some("the worker thread panicked")));
}

/// Counts the logger's method starts and ends.
struct MethodCounter {
    severity: Cell<Severity>,
    starts: Cell<usize>,
    ends: Cell<usize>,
}

impl IFernflowerLogger for MethodCounter {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, _message: &str, _severity: Severity) {}

    fn write_message_ex(&self, _message: &str, _severity: Severity, _t: &Caught) {}

    fn start_method(&self, _method_name: &str) {
        self.starts.set(self.starts.get() + 1);
    }

    fn end_method(&self) {
        self.ends.set(self.ends.get() + 1);
    }
}

/// `BadDescriptor.java` compiled by javac 21 (`--release 8`), with the
/// descriptor of `first`, `(I)I`, patched to `xI)I`.
#[test]
fn a_method_that_throws_before_its_body_ends_its_logger_method() {
    let input = kuna_input::load(
        &data("malformed_descriptor/BadDescriptor.class"),
        &LoadOptions::default(),
    )
    .unwrap();
    let opts = options(&[]);
    let (starts, ends, units) = with_engine_stack(|| {
        reset_engine_thread();
        let logger = Rc::new(MethodCounter {
            severity: Cell::new(Severity::Error),
            starts: Cell::new(0),
            ends: Cell::new(0),
        });
        let mut fernflower = Fernflower::new(&opts, logger.clone());
        for source in input.sources() {
            fernflower.add_source(source);
        }
        let mut units = Vec::new();
        fernflower.decompile_context(&mut |u| units.push(u));
        fernflower.clear_context();
        reset_engine_thread();
        (logger.starts.get(), logger.ends.get(), units)
    });
    assert_eq!(units.len(), 1);
    assert!(units[0].content.contains("Invalid descriptor: xI)I"));
    assert_eq!(starts, 2, "<init> and first");
    assert_eq!(ends, starts, "a start_method without its end_method");
}
