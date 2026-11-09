//! Driving the engine for one command line: the input and its inventory, the
//! JDK the engine reads, the driver configuration, the per-method memory
//! guard, and the last-resort watchdog for a method no deadline poll stops.
//!
//! The engine stops a method at `--max-fn-seconds` itself, at its poll sites.
//! A method stuck where nothing polls would hold the run forever, so a monitor
//! thread watches the methods in progress and, once one has run `2N+30`
//! seconds, finishes the command with what was delivered (every class not yet
//! delivered is reported as failed) and exits.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use kuna_decomp::kuna_fnmemory;
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, Severity};
use kuna_jvm::kuna_driver::{self, ClassResult, DriverConfig, Jobs, LoggerFactory};
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions, SharedRuntime};
use kuna_jvm::kuna_inventory;
use kuna_jvm::rt::Caught;

use super::args::{Args, RuntimeSpec};
use super::select::Universe;

/// An input with its class inventory.
pub(crate) struct Loaded {
    pub(crate) input: InputSet,
    pub(crate) universe: Universe,
}

impl Loaded {
    /// A single class file's own class (the target a `decompile` without one means).
    pub(crate) fn own_class(&self) -> Option<String> {
        (self.input.kind() == kuna_input::InputKind::ClassFile)
            .then(|| self.input.classes().first().map(|c| c.entry.to_string()))
            .flatten()
    }

    /// Every top-level class, by internal name.
    pub(crate) fn roots(&self) -> Vec<String> {
        self.universe
            .classes
            .iter()
            .filter(|c| self.universe.root_of(&c.internal) == c.internal)
            .map(|c| c.internal.clone())
            .collect()
    }
}

pub(crate) fn load(args: &Args) -> Result<Loaded, String> {
    let options = LoadOptions {
        siblings: args.siblings,
    };
    let input = kuna_input::load(Path::new(&args.input), &options).map_err(|e| e.to_string())?;
    kuna_jvm::kuna_driver::trace(|| "cli input read".into());
    let universe = Universe::new(kuna_inventory::list(&input));
    if universe.classes.is_empty() {
        return Err(format!("{}: no class file could be read", args.input));
    }
    Ok(Loaded { input, universe })
}

/// The JDK `spec` names. A JDK that cannot be found is a note, not an error,
/// unless the caller named its home.
pub(crate) fn open_runtime(spec: &RuntimeSpec) -> Result<Option<Arc<SharedRuntime>>, String> {
    match spec {
        RuntimeSpec::Off => Ok(None),
        RuntimeSpec::Current => match SharedRuntime::open("1") {
            Ok(runtime) => Ok(Some(Arc::new(runtime))),
            Err(_) => {
                eprintln!(
                    "note: no JDK found (JAVA_HOME, java on PATH, the usual install directories); \
                     decompiling without the Java class library. --option jvm_include_runtime <JDK home> \
                     names one; `off` silences this note"
                );
                Ok(None)
            }
        },
        RuntimeSpec::Path(home) => SharedRuntime::open(home)
            .map(|runtime| Some(Arc::new(runtime)))
            .map_err(|e| format!("--option jvm_include_runtime {home}: {e}")),
    }
}

/// The methods in progress on the engine threads, by logger instance.
#[derive(Default)]
pub(crate) struct Watch {
    busy: Mutex<BTreeMap<usize, (Instant, String)>>,
    slots: AtomicUsize,
}

impl Watch {
    fn enter(&self, slot: usize, method: &str) {
        self.lock().insert(slot, (Instant::now(), method.to_string()));
    }

    fn leave(&self, slot: usize) {
        self.lock().remove(&slot);
    }

    /// A method that has run longer than `limit`.
    fn stuck(&self, limit: Duration) -> Option<String> {
        self.lock()
            .values()
            .find(|(since, _)| since.elapsed() >= limit)
            .map(|(_, method)| method.clone())
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BTreeMap<usize, (Instant, String)>> {
        self.busy.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// The engine logger: silent (per-method failures reach the output and the
/// JSON records), it marks method starts and ends for the watchdog and arms
/// the memory limit around each method.
struct KunaLogger {
    severity: Cell<Severity>,
    watch: Arc<Watch>,
    slot: usize,
    memory: bool,
    armed: RefCell<Option<kuna_fnmemory::Armed>>,
}

impl IFernflowerLogger for KunaLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, _message: &str, _severity: Severity) {}

    fn write_message_ex(&self, _message: &str, _severity: Severity, _t: &Caught) {}

    fn start_method(&self, method_name: &str) {
        self.watch.enter(self.slot, method_name);
        if self.memory {
            *self.armed.borrow_mut() = Some(kuna_fnmemory::Armed::enter());
        }
    }

    fn end_method(&self) {
        self.armed.borrow_mut().take();
        self.watch.leave(self.slot);
    }
}

/// Arms `--max-fn-memory` (or the cgroup default); whether a limit is in force.
fn arm_memory_limit(args: &Args) -> bool {
    match args.max_fn_memory {
        Some(mib) => {
            kuna_fnmemory::set_limit_mib(mib);
            mib > 0
        }
        None => kuna_fnmemory::use_cgroup_default(),
    }
}

pub(crate) fn config(
    args: &Args,
    runtime: Option<Arc<SharedRuntime>>,
    select: Option<BTreeSet<String>>,
    watch: &Arc<Watch>,
) -> Result<DriverConfig, String> {
    let memory = arm_memory_limit(args);
    let logger_watch = watch.clone();
    let logger: LoggerFactory = Arc::new(move || {
        let slot = logger_watch.slots.fetch_add(1, Ordering::Relaxed);
        Rc::new(KunaLogger {
            severity: Cell::new(Severity::Error),
            watch: logger_watch.clone(),
            slot,
            memory,
            armed: RefCell::new(None),
        }) as Rc<dyn IFernflowerLogger>
    });
    let mut options = vec![("log-level".to_string(), "warn".to_string())];
    options.extend(args.options.iter().cloned());
    let mut config = DriverConfig::new(options, logger);
    config.jobs = args.jobs.map_or(Jobs::Auto, Jobs::Exactly);
    config.available_memory = kuna_decomp::kuna_hostmem::available().map(|a| a.bytes);
    config.runtime = runtime;
    for lib in &args.libs {
        let set = kuna_input::load_library(Path::new(lib)).map_err(|e| format!("--lib {lib}: {e}"))?;
        config.libraries.push(Arc::new(set));
    }
    config.select = select;
    config.max_method_seconds = u32::try_from(args.max_fn_seconds).unwrap_or(u32::MAX);
    if memory {
        config.method_guard = Some(Arc::new(|| {
            Rc::new(|| {
                kuna_fnmemory::over(Instant::now()).then(|| {
                    format!(
                        "the method exceeded the per-function memory limit ({} MiB; --max-fn-memory)",
                        kuna_fnmemory::limit_mib()
                    )
                })
            }) as Rc<dyn Fn() -> Option<String>>
        }));
    }
    Ok(config)
}

/// Leaves a rendered outcome to the process exit that follows: its text was
/// built on the engine threads, and handing it back to their heaps while they
/// drop their engines costs a quarter second on a large jar.
pub(crate) fn abandon(outcome: Outcome) {
    std::mem::forget(outcome);
}

/// What a finished (or abandoned) run hands its renderer.
pub(crate) struct Outcome {
    /// Delivered root classes, in root order.
    pub(crate) results: Vec<ClassResult>,
    /// Roots asked for but never delivered, with the reason. A selection may
    /// also name a nested class in case the engine takes it for a top-level
    /// one (`Universe::chain`); one it does not is not listed.
    pub(crate) missing: Vec<(String, String)>,
    /// Why the run as a whole stopped early.
    pub(crate) error: Option<String>,
}

/// Runs the driver and hands the outcome to `finish`, whose status is the
/// command's; under a stall `finish` runs on the monitor thread and the process
/// exits with its status.
pub(crate) fn drive(
    loaded: &Loaded,
    args: &Args,
    config: &DriverConfig,
    watch: &Watch,
    finish: &(dyn Fn(Outcome) -> i32 + Sync),
) -> i32 {
    let top_level = |name: &str| loaded.universe.root_of(name) == name;
    let expected: Vec<String> = match &config.select {
        Some(select) => select.iter().filter(|name| top_level(name)).cloned().collect(),
        None => loaded.roots(),
    };
    let results: Mutex<Vec<ClassResult>> = Mutex::new(Vec::new());
    let finishing = AtomicBool::new(false);
    let limit = (args.max_fn_seconds > 0)
        .then(|| Duration::from_secs(args.max_fn_seconds.saturating_mul(2).saturating_add(30)));
    let missing_of = |delivered: &[ClassResult], reason: &str| -> Vec<(String, String)> {
        let seen: BTreeSet<&str> = delivered.iter().map(|r| r.qualified_name.as_str()).collect();
        expected
            .iter()
            .filter(|name| !seen.contains(name.as_str()))
            .map(|name| (name.clone(), reason.to_string()))
            .collect()
    };
    let (done_tx, done_rx) = mpsc::channel::<()>();
    std::thread::scope(|scope| {
        if let Some(limit) = limit {
            let (results, finishing, missing_of) = (&results, &finishing, &missing_of);
            scope.spawn(move || loop {
                match done_rx.recv_timeout(Duration::from_secs(1)) {
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    _ => return,
                }
                let Some(method) = watch.stuck(limit) else {
                    continue;
                };
                if finishing.swap(true, Ordering::SeqCst) {
                    return;
                }
                let delivered = std::mem::take(&mut *results.lock().unwrap_or_else(|e| e.into_inner()));
                let reason = format!(
                    "method {method} ran {} s without reaching a deadline poll (--max-fn-seconds {}); \
                     the run was stopped",
                    limit.as_secs(),
                    args.max_fn_seconds
                );
                let missing = missing_of(&delivered, &reason);
                let status = finish(Outcome {
                    results: delivered,
                    missing,
                    error: Some(reason),
                });
                std::process::exit(status);
            });
        }
        let run = kuna_driver::run(&loaded.input, config, &mut |result| {
            results.lock().unwrap_or_else(|e| e.into_inner()).push(result);
        });
        drop(done_tx);
        if finishing.swap(true, Ordering::SeqCst) {
            loop {
                std::thread::park();
            }
        }
        let delivered = std::mem::take(&mut *results.lock().unwrap_or_else(|e| e.into_inner()));
        let outcome = match run {
            Ok(report) => {
                let delivered_names: BTreeSet<&str> =
                    delivered.iter().map(|r| r.qualified_name.as_str()).collect();
                let missing = report
                    .failed
                    .iter()
                    .filter(|name| !delivered_names.contains(name.as_str()) && top_level(name))
                    .map(|name| (name.clone(), "not a top-level class of the input".to_string()))
                    .collect();
                Outcome {
                    results: delivered,
                    missing,
                    error: None,
                }
            }
            Err(e) => {
                let reason = e.to_string();
                let missing = missing_of(&delivered, &reason);
                Outcome {
                    results: delivered,
                    missing,
                    error: Some(reason),
                }
            }
        };
        finish(outcome)
    })
}

#[cfg(test)]
mod tests {
    use super::Watch;
    use std::time::Duration;

    #[test]
    fn a_method_past_the_limit_is_reported_stuck() {
        let watch = Watch::default();
        watch.enter(0, "loop ()V");
        assert_eq!(watch.stuck(Duration::from_secs(3600)), None);
        assert_eq!(watch.stuck(Duration::ZERO).as_deref(), Some("loop ()V"));
        watch.leave(0);
        assert_eq!(watch.stuck(Duration::ZERO), None);
    }
}
