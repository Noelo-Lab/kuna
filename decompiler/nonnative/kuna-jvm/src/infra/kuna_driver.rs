//! The parallel driver: one input's root classes decompiled on N threads, one
//! engine instance per thread, with output identical to a single instance's.
//!
//! Every worker loads its engine as soon as it starts; the first one also
//! plans: it lists the root classes in the
//! order a single run visits them and reads, from the class files, which
//! other roots a root needs pre-processed (the roots whose nested classes'
//! constructors it calls, see `kuna_partition`). The roots are then grouped
//! into batches ([`Strategy`]); worker threads take batches from a shared
//! cursor, largest first, and run each with `Fernflower::decompile_selected`,
//! which pre-processes and writes the batch's roots plus their dependencies in
//! the single run's order and delivers the batch's roots. A worker keeps its
//! engine (the loaded class universe) for its next batch unless that batch
//! needs a root this engine already processed. The divergent-read detector
//! watches every run; a batch that read the state of a root it did not process
//! is run again with that root added (and, after a few rounds, with every root),
//! so its results are delivered only once clean. Results reach the sink on
//! the calling thread in root order, whatever the number of workers; the
//! call returns after the last delivery, while the workers drop their engines.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;

use crate::classes_processor::ClassNode;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fernflower::{DecompiledClass, Fernflower};
use crate::i_fernflower_logger::IFernflowerLogger;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::kuna_corpus::{reset_engine_thread, ENGINE_STACK_BYTES};
use crate::kuna_input::{InputSet, SharedRuntime};
use crate::kuna_inventory;
use crate::kuna_partition::{self, DivergentRead, RootPos, RootSelection};
use crate::rt::try_catch;

/// The most worker threads `Jobs::Auto` picks.
pub const MAX_AUTO_JOBS: usize = 24;

/// Class files per worker `Jobs::Auto` wants at least: a worker costs an
/// engine load whatever its share.
pub const AUTO_CLASSES_PER_JOB: usize = 16;

/// How many engine instances run at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jobs {
    /// `min(cores, MAX_AUTO_JOBS, class files / AUTO_CLASSES_PER_JOB)`, fewer
    /// when [`estimate_memory`] for the count does not fit in 90% of the
    /// available memory.
    Auto,
    Exactly(usize),
}

/// How root classes are grouped into batches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strategy {
    /// Roots joined with the roots they depend on while the unit stays
    /// within the first batch's size, units packed in run order into batches
    /// that shrink as the plan goes; each batch also pre-processes the roots
    /// it depends on outside it, so a root may be processed by several instances.
    Closure,
    /// Roots joined with everything they depend on (connected components),
    /// packed into batches; no root is processed twice, but one large
    /// component limits the parallelism.
    Components,
}

/// Creates the engine logger of each instance (called on the worker thread).
pub type LoggerFactory = Arc<dyn Fn() -> Rc<dyn IFernflowerLogger> + Send + Sync>;
/// Creates the method guard of each worker thread (see `CancelationManager::set_method_guard`).
pub type GuardFactory = Arc<dyn Fn() -> Rc<dyn Fn() -> Option<String>> + Send + Sync>;
pub type ProgressFn = Arc<dyn Fn(&Progress) + Send + Sync>;

pub struct DriverConfig {
    /// Vineflower options as `(key, value)`; later pairs win. The driver sets
    /// `include-runtime`, `include-classpath`, `thread-count` and
    /// `new-line-separator` itself. kuna's `jvm_plugins` pair reaches every
    /// instance (the bundled plugins are on when it is absent).
    pub options: Vec<(String, String)>,
    pub jobs: Jobs,
    pub strategy: Strategy,
    /// The JDK whose classes give the engine context (`None`: no runtime).
    pub runtime: Option<Arc<SharedRuntime>>,
    /// Inputs read for context only, not decompiled.
    pub libraries: Vec<Arc<InputSet>>,
    /// The roots to deliver (internal names); `None` delivers every root.
    pub select: Option<BTreeSet<String>>,
    /// Seconds a method may take (`max-time-per-method`); 0: no limit.
    pub max_method_seconds: u32,
    pub logger: LoggerFactory,
    pub method_guard: Option<GuardFactory>,
    pub progress: Option<ProgressFn>,
    pub stack_size: usize,
    /// Memory to plan with, in bytes (`None`: `MemAvailable`).
    pub available_memory: Option<u64>,
    /// Memory one instance needs for the loaded class universe, in bytes
    /// (`None`: estimated from the input, see [`estimate_memory`]).
    pub instance_memory: Option<u64>,
}

impl DriverConfig {
    pub fn new(options: Vec<(String, String)>, logger: LoggerFactory) -> DriverConfig {
        DriverConfig {
            options,
            jobs: Jobs::Auto,
            strategy: Strategy::Closure,
            runtime: None,
            libraries: Vec::new(),
            select: None,
            max_method_seconds: 0,
            logger,
            method_guard: None,
            progress: None,
            stack_size: ENGINE_STACK_BYTES,
            available_memory: None,
            instance_memory: None,
        }
    }
}

/// What the driver reports while it runs.
#[derive(Clone, Debug)]
pub enum Progress {
    Planned {
        roots: usize,
        batches: usize,
        workers: usize,
    },
    /// A root class's result reached the sink.
    Delivered {
        done: usize,
        total: usize,
        class: String,
    },
    /// A batch is run again because a run read state it had not produced.
    Repeating { roots: usize, read: DivergentRead },
}

/// One root class's outcome, in root order.
#[derive(Clone, Debug)]
pub struct ClassResult {
    /// Position in the run order of the roots.
    pub index: usize,
    pub qualified_name: String,
    /// `None` when the engine produced no text for the class.
    pub unit: Option<DecompiledClass>,
    /// Why there is no unit, or the unit's own error.
    pub error: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Report {
    /// Root classes of the input, in run order.
    pub roots: Vec<String>,
    pub delivered: usize,
    /// Roots selected but without output.
    pub failed: Vec<String>,
    pub workers: usize,
    pub batches: usize,
    /// Engine instances created (the planning one included).
    pub instances: usize,
    /// Root classes pre-processed in total, dependencies included.
    pub processed: usize,
    /// Batches run again after a divergent read.
    pub repeated: usize,
    pub divergent_reads: Vec<DivergentRead>,
    /// Batches after which the engine's thread-local lambda-type map was not
    /// empty: a later class on that thread could have seen the leftover.
    pub leaks: Vec<String>,
    /// Every batch run, in completion order.
    pub runs: Vec<BatchRun>,
    pub plan_millis: u64,
    pub millis: u64,
}

/// One batch run.
#[derive(Clone, Debug, Default)]
pub struct BatchRun {
    pub delivered: usize,
    pub processed: usize,
    /// Repetition round (0: first run).
    pub round: usize,
    /// Whether the run needed a new engine instance (`load_millis` is then its loading time).
    pub fresh_instance: bool,
    pub load_millis: u64,
    pub millis: u64,
}

/// Why nothing could be decompiled.
#[derive(Debug)]
pub struct DriverError(pub String);

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DriverError {}

struct Plan {
    roots: Vec<String>,
    order: Arc<BTreeMap<String, RootPos>>,
    cost: Vec<u64>,
    deps: Vec<BTreeSet<usize>>,
}

#[derive(Clone, Debug)]
struct Batch {
    id: usize,
    deliver: BTreeSet<usize>,
    process: BTreeSet<usize>,
    cost: u64,
    round: usize,
}

struct Outcome {
    batch: Batch,
    units: Vec<DecompiledClass>,
    reads: Vec<DivergentRead>,
    leak: bool,
    failure: Option<String>,
    fresh_instance: bool,
    load_millis: u64,
    millis: u64,
}

/// Decompiles `input` under `config`, handing each selected root class's
/// result to `sink` in root order.
pub fn run(
    input: &InputSet,
    config: &DriverConfig,
    sink: &mut dyn FnMut(ClassResult),
) -> Result<Report, DriverError> {
    let started = Instant::now();
    trace(|| "driver start".into());
    let workers_wanted = worker_count(input, config);
    trace(|| format!("workers {workers_wanted}"));

    let (plan_tx, plan_rx) = mpsc::channel::<Result<Plan, String>>();
    let (out_tx, out_rx) = mpsc::channel::<Outcome>();
    let instances = Arc::new(AtomicUsize::new(0));
    let board = Arc::new(Board::default());
    let env = Arc::new(WorkerEnv::new(input, config));

    let mut report = Report::default();
    let result = (|| -> Result<(), DriverError> {
        let _finish = FinishOnDrop(&board);
        for worker in 0..workers_wanted {
            let out_tx = out_tx.clone();
            let plan_tx = (worker == 0).then(|| plan_tx.clone());
            let instances = instances.clone();
            let board = board.clone();
            let env = env.clone();
            std::thread::Builder::new()
                .name(format!("kuna-jvm-{worker}"))
                .stack_size(config.stack_size)
                .spawn(move || worker_main(&env, plan_tx, &board, &out_tx, &instances))
                .map_err(|e| DriverError(format!("cannot start a worker thread: {e}")))?;
        }
        drop(plan_tx);
        drop(out_tx);

        let plan = match plan_rx.recv() {
            Ok(Ok(plan)) => plan,
            Ok(Err(e)) => {
                board.finish();
                return Err(DriverError(e));
            }
            Err(_) => {
                board.finish();
                return Err(DriverError("the planning engine stopped".to_string()));
            }
        };
        report.plan_millis = started.elapsed().as_millis() as u64;
        trace(|| format!("plan received roots={}", plan.roots.len()));
        report.roots = plan.roots.clone();

        let selected: Vec<usize> = match &config.select {
            None => (0..plan.roots.len()).collect(),
            Some(names) => names
                .iter()
                .filter_map(|n| plan.order.get(n).map(|p| p.index))
                .collect::<BTreeSet<usize>>()
                .into_iter()
                .collect(),
        };
        if let Some(names) = &config.select {
            for name in names {
                if !plan.order.contains_key(name) {
                    report.failed.push(name.clone());
                }
            }
        }

        let batches = make_batches(&plan, &selected, workers_wanted, config.strategy);
        report.batches = batches.len();
        report.workers = workers_wanted.min(batches.len().max(1));
        if let Some(progress) = &config.progress {
            progress(&Progress::Planned {
                roots: selected.len(),
                batches: batches.len(),
                workers: report.workers,
            });
        }
        let shared = Arc::new(SharedPlan {
            order: plan.order.clone(),
            roots: plan.roots.clone(),
            all: (0..plan.roots.len()).collect(),
        });
        let mut sorted = batches.clone();
        sorted.sort_by(|a, b| b.cost.cmp(&a.cost).then(a.id.cmp(&b.id)));
        board.start(shared, sorted);

        let mut slots: BTreeMap<usize, ClassResult> = BTreeMap::new();
        let mut next = 0usize;
        let order_of_selected: Vec<usize> = selected.clone();
        let mut done = 0usize;
        let mut next_id = batches.len();
        let mut outstanding = batches.len();

        while outstanding > 0 {
            let Ok(outcome) = out_rx.recv() else {
                break;
            };
            outstanding -= 1;
            trace(|| {
                format!(
                    "outcome batch {} reads={}",
                    outcome.batch.id,
                    outcome.reads.len()
                )
            });
            report.processed += outcome.batch.process.len();
            report.runs.push(BatchRun {
                delivered: outcome.batch.deliver.len(),
                processed: outcome.batch.process.len(),
                round: outcome.batch.round,
                fresh_instance: outcome.fresh_instance,
                load_millis: outcome.load_millis,
                millis: outcome.millis,
            });
            if outcome.fresh_instance {
                report.instances += 1;
            }
            if outcome.leak {
                report
                    .leaks
                    .push(names_of(&plan.roots, &outcome.batch.deliver).join(" "));
            }
            if !outcome.reads.is_empty() && outcome.failure.is_none() {
                report.repeated += 1;
                let mut process = outcome.batch.process.clone();
                for read in &outcome.reads {
                    if let Some(owner) = plan.order.get(&read.owner) {
                        process.insert(owner.index);
                    }
                }
                let round = outcome.batch.round + 1;
                if round >= MAX_ROUNDS || process == outcome.batch.process {
                    process = (0..plan.roots.len()).collect();
                }
                if let Some(progress) = &config.progress {
                    progress(&Progress::Repeating {
                        roots: outcome.batch.deliver.len(),
                        read: outcome.reads[0].clone(),
                    });
                }
                report.divergent_reads.extend(outcome.reads.iter().cloned());
                let batch = Batch {
                    id: next_id,
                    deliver: outcome.batch.deliver.clone(),
                    cost: process.iter().map(|&i| plan.cost[i]).sum(),
                    process,
                    round,
                };
                next_id += 1;
                outstanding += 1;
                board.push_front(batch);
                continue;
            }
            let mut by_name: BTreeMap<String, DecompiledClass> = BTreeMap::new();
            for unit in outcome.units {
                by_name.entry(unit.qualified_name.clone()).or_insert(unit);
            }
            for &index in &outcome.batch.deliver {
                let name = plan.roots[index].clone();
                let unit = by_name.remove(&name);
                let error = match (&unit, &outcome.failure) {
                    (_, Some(failure)) => Some(failure.clone()),
                    (Some(unit), None) => unit.error.clone(),
                    (None, None) => Some(format!("no output for class {name}")),
                };
                slots.insert(
                    index,
                    ClassResult {
                        index,
                        qualified_name: name,
                        unit: if outcome.failure.is_some() {
                            None
                        } else {
                            unit
                        },
                        error,
                    },
                );
            }
            while next < order_of_selected.len() {
                let index = order_of_selected[next];
                let Some(result) = slots.remove(&index) else {
                    break;
                };
                if result.unit.is_none() {
                    report.failed.push(result.qualified_name.clone());
                } else {
                    report.delivered += 1;
                }
                done += 1;
                if let Some(progress) = &config.progress {
                    progress(&Progress::Delivered {
                        done,
                        total: order_of_selected.len(),
                        class: result.qualified_name.clone(),
                    });
                }
                sink(result);
                next += 1;
            }
        }
        trace(|| "all delivered".into());
        board.finish();
        if next < order_of_selected.len() {
            return Err(DriverError(format!(
                "{} root class(es) were never delivered (a worker stopped)",
                order_of_selected.len() - next
            )));
        }
        Ok(())
    })();
    result?;
    report.instances = report.instances.max(instances.load(Ordering::SeqCst));
    report.millis = started.elapsed().as_millis() as u64;
    Ok(report)
}

/// Rounds of repeating a batch with added roots before it is run with every root.
const MAX_ROUNDS: usize = 3;

/// Queued batches a worker looks through for one its engine can run as is.
const LOOKAHEAD: usize = 8;

struct SharedPlan {
    order: Arc<BTreeMap<String, RootPos>>,
    roots: Vec<String>,
    all: BTreeSet<usize>,
}

/// Ends the run for the workers however the coordinating thread leaves.
struct FinishOnDrop<'a>(&'a Arc<Board>);

impl Drop for FinishOnDrop<'_> {
    fn drop(&mut self) {
        self.0.finish();
    }
}

/// The work queue the workers take batches from.
#[derive(Default)]
struct Board {
    state: Mutex<BoardState>,
    changed: Condvar,
}

#[derive(Default)]
struct BoardState {
    plan: Option<Arc<SharedPlan>>,
    queue: VecDeque<Batch>,
    finished: bool,
}

impl Board {
    fn start(&self, plan: Arc<SharedPlan>, batches: Vec<Batch>) {
        let mut state = self.state.lock().unwrap();
        state.plan = Some(plan);
        state.queue.extend(batches);
        self.changed.notify_all();
    }

    fn push_front(&self, batch: Batch) {
        self.state.lock().unwrap().queue.push_front(batch);
        self.changed.notify_all();
    }

    fn finish(&self) {
        self.state.lock().unwrap().finished = true;
        self.changed.notify_all();
    }

    /// The next batch, or `None` once the run is over: among the next
    /// [`LOOKAHEAD`], the first that processes none of `processed` (the
    /// caller's engine can run it without reloading), else the first.
    fn take(&self, processed: &BTreeSet<usize>) -> Option<(Batch, Arc<SharedPlan>)> {
        let mut state = self.state.lock().unwrap();
        loop {
            if state.finished {
                return None;
            }
            if let Some(plan) = state.plan.clone() {
                let pick = state
                    .queue
                    .iter()
                    .take(LOOKAHEAD)
                    .position(|b| b.process.is_disjoint(processed))
                    .unwrap_or(0);
                if let Some(batch) = state.queue.remove(pick) {
                    return Some((batch, plan));
                }
            }
            state = self.changed.wait(state).unwrap();
        }
    }
}

fn names_of(roots: &[String], indices: &BTreeSet<usize>) -> Vec<String> {
    indices.iter().map(|&i| roots[i].clone()).collect()
}

/// One engine instance on this thread.
struct Session {
    fernflower: Fernflower,
    processed: BTreeSet<usize>,
}

fn engine_options(config: &DriverConfig) -> Vec<(String, String)> {
    let mut options = config.options.clone();
    let mut set = |key: &str, value: String| {
        options.retain(|(k, _)| k != key);
        options.push((key.to_string(), value));
    };
    set(
        IFernflowerPreferences::INCLUDE_JAVA_RUNTIME,
        "0".to_string(),
    );
    set(
        IFernflowerPreferences::INCLUDE_ENTIRE_CLASSPATH,
        "0".to_string(),
    );
    set(IFernflowerPreferences::THREADS, "1".to_string());
    set(IFernflowerPreferences::NEW_LINE_SEPARATOR, "1".to_string());
    set(
        IFernflowerPreferences::MAX_PROCESSING_METHOD,
        config.max_method_seconds.to_string(),
    );
    options
}

/// Whether `KUNA_JVM_DRIVER_TRACE` is set: [`trace`] then prints a timeline to stderr.
pub fn trace_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("KUNA_JVM_DRIVER_TRACE").is_some())
}

/// One timeline line (seconds since the first line, thread, resident set, event).
pub fn trace(what: impl FnOnce() -> String) {
    static T0: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    if trace_on() {
        let t0 = *T0.get_or_init(Instant::now);
        let thread = std::thread::current();
        let rss = std::fs::read_to_string("/proc/self/statm")
            .ok()
            .and_then(|s| {
                s.split_whitespace()
                    .nth(1)
                    .and_then(|p| p.parse::<u64>().ok())
            })
            .unwrap_or(0)
            * 4096;
        eprintln!(
            "TRACE {:9.3} {:12} rss={:6}M {}",
            t0.elapsed().as_secs_f64(),
            thread.name().unwrap_or("?"),
            rss >> 20,
            what()
        );
    }
}

/// What every worker thread reads, owned so that workers can outlive the
/// call: a worker tears its engine down after the last delivery, off the
/// caller's path.
struct WorkerEnv {
    input: InputSet,
    options: Vec<(String, String)>,
    runtime: Option<Arc<SharedRuntime>>,
    libraries: Vec<Arc<InputSet>>,
    logger: LoggerFactory,
    method_guard: Option<GuardFactory>,
}

impl WorkerEnv {
    fn new(input: &InputSet, config: &DriverConfig) -> WorkerEnv {
        WorkerEnv {
            input: input.clone(),
            options: engine_options(config),
            runtime: config.runtime.clone(),
            libraries: config.libraries.clone(),
            logger: config.logger.clone(),
            method_guard: config.method_guard.clone(),
        }
    }
}

fn open_session(env: &WorkerEnv) -> Result<Session, String> {
    trace(|| "open start".into());
    reset_engine_thread();
    let logger = (env.logger)();
    let opened = try_catch(|| {
        let fernflower = Fernflower::new(&env.options, logger);
        if let Some(runtime) = &env.runtime {
            for source in runtime.sources() {
                fernflower.add_library(source);
            }
        }
        trace(|| "open runtime".into());
        for library in &env.libraries {
            for source in library.sources() {
                fernflower.add_library(source);
            }
        }
        for source in env.input.sources() {
            fernflower.add_source(source);
        }
        trace(|| "open sources".into());
        fernflower
    });
    match opened {
        Ok(fernflower) => Ok(Session {
            fernflower,
            processed: BTreeSet::new(),
        }),
        Err(e) => {
            let _ = try_catch(reset_engine_thread);
            Err(format!(
                "cannot load {}: {}",
                env.input.path().display(),
                e.message
            ))
        }
    }
}

fn close_session(session: Option<Session>) {
    if let Some(mut session) = session {
        let _ = try_catch(|| session.fernflower.clear_context());
    }
    let _ = try_catch(reset_engine_thread);
}

fn worker_main(
    env: &WorkerEnv,
    plan_tx: Option<mpsc::Sender<Result<Plan, String>>>,
    board: &Board,
    out_tx: &mpsc::Sender<Outcome>,
    instances: &AtomicUsize,
) {
    if let Some(guard) = &env.method_guard {
        crate::cancelation_manager::CancelationManager::set_method_guard(Some(guard()));
    }
    let mut session: Option<Session> = None;
    let mut fresh = false;
    let mut load_millis = 0u64;
    let loading = Instant::now();
    match (plan_tx, open_session(env)) {
        (Some(plan_tx), Ok(mut s)) => {
            instances.fetch_add(1, Ordering::SeqCst);
            match try_catch(|| plan(&mut s.fernflower, &env.input)) {
                Ok(plan) => {
                    trace(|| "plan done".into());
                    load_millis = loading.elapsed().as_millis() as u64;
                    let _ = plan_tx.send(Ok(plan));
                    session = Some(s);
                    fresh = true;
                }
                Err(e) => {
                    let _ = plan_tx.send(Err(format!(
                        "cannot plan {}: {}",
                        env.input.path().display(),
                        e.message
                    )));
                    close_session(Some(s));
                    return;
                }
            }
        }
        (Some(plan_tx), Err(e)) => {
            let _ = plan_tx.send(Err(e));
            return;
        }
        (None, Ok(mut s)) => {
            instances.fetch_add(1, Ordering::SeqCst);
            let _ = try_catch(|| s.fernflower.load());
            trace(|| "loaded".into());
            load_millis = loading.elapsed().as_millis() as u64;
            session = Some(s);
            fresh = true;
        }
        (None, Err(_)) => close_session(None),
    }
    let none = BTreeSet::new();
    while let Some((batch, shared)) = board.take(session.as_ref().map_or(&none, |s| &s.processed)) {
        let reuse = session
            .as_ref()
            .is_some_and(|s| s.processed.is_disjoint(&batch.process));
        if !reuse {
            close_session(session.take());
            let loading = Instant::now();
            match open_session(env) {
                Ok(mut s) => {
                    instances.fetch_add(1, Ordering::SeqCst);
                    let _ = try_catch(|| s.fernflower.load());
                    trace(|| "loaded".into());
                    load_millis = loading.elapsed().as_millis() as u64;
                    session = Some(s);
                    fresh = true;
                }
                Err(e) => {
                    let _ = out_tx.send(Outcome {
                        batch,
                        units: Vec::new(),
                        reads: Vec::new(),
                        leak: false,
                        failure: Some(e),
                        fresh_instance: true,
                        load_millis: 0,
                        millis: 0,
                    });
                    continue;
                }
            }
        }
        let s = session.as_mut().expect("session");
        let mut outcome = run_batch(s, batch, &shared, fresh);
        outcome.load_millis = std::mem::take(&mut load_millis);
        fresh = false;
        if out_tx.send(outcome).is_err() {
            break;
        }
    }
    close_session(session);
    crate::cancelation_manager::CancelationManager::set_method_guard(None);
}

fn run_batch(session: &mut Session, batch: Batch, plan: &SharedPlan, fresh: bool) -> Outcome {
    let process: BTreeSet<String> = batch
        .process
        .iter()
        .map(|&i| plan.roots[i].clone())
        .collect();
    let selection = RootSelection {
        process,
        deliver: batch
            .deliver
            .iter()
            .map(|&i| plan.roots[i].clone())
            .collect(),
    };
    session.processed.extend(batch.process.iter().copied());
    let watched = batch.process != plan.all;
    if watched {
        kuna_partition::watch(plan.order.clone(), &selection);
    }
    let mut units = Vec::new();
    let started = Instant::now();
    trace(|| {
        format!(
            "batch {} start process={} deliver={} cost={} round={} fresh={}",
            batch.id,
            batch.process.len(),
            batch.deliver.len(),
            batch.cost,
            batch.round,
            fresh
        )
    });
    let result = try_catch(|| {
        session
            .fernflower
            .decompile_selected(&selection, &mut |unit| units.push(unit))
    });
    trace(|| format!("batch {} end", batch.id));
    let reads = if watched {
        kuna_partition::unwatch()
    } else {
        Vec::new()
    };
    let leak = !Exprent::with_inferred_lambda_types(|m| m.is_empty());
    Outcome {
        batch,
        units,
        reads,
        leak,
        failure: result
            .err()
            .map(|e| format!("the engine failed: {}", e.message)),
        fresh_instance: fresh,
        load_millis: 0,
        millis: started.elapsed().as_millis() as u64,
    }
}

/// Lists the roots in run order with their sizes and dependencies.
fn plan(fernflower: &mut Fernflower, input: &InputSet) -> Plan {
    let placed = fernflower.root_classes_in_units();
    let roots: Vec<String> = placed.iter().map(|(_, n)| n.clone()).collect();
    let order: BTreeMap<String, RootPos> = placed
        .iter()
        .enumerate()
        .rev()
        .map(|(index, (unit, n))| (n.clone(), RootPos { unit: *unit, index }))
        .collect();

    let mut root_of: BTreeMap<String, usize> = BTreeMap::new();
    let nodes: Vec<(String, ClassNode)> = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .iter()
        .map(|(name, node)| (name.clone(), *node))
        .collect();
    for (name, node) in nodes {
        let mut top = node;
        while let Some(parent) = top.with(|d| d.parent) {
            top = parent;
        }
        let top_name = top.with(|d| d.class_struct).qualified_name();
        if let Some(pos) = order.get(&top_name) {
            root_of.insert(name, pos.index);
        }
    }

    let mut cost = vec![0u64; roots.len()];
    let mut deps = vec![BTreeSet::new(); roots.len()];
    for (info, refs) in kuna_inventory::parse_all(input).into_iter().flatten() {
        let Some(&root) = root_of.get(&info.internal) else {
            continue;
        };
        cost[root] += 256
            + info
                .methods
                .iter()
                .map(|m| m.code_length.unwrap_or(0) as u64)
                .sum::<u64>();
        for owner in &refs.constructors {
            if let Some(&dep) = root_of.get(owner) {
                if dep != root && !order.contains_key(owner) {
                    deps[root].insert(dep);
                }
            }
        }
    }
    Plan {
        roots,
        order: Arc::new(order),
        cost,
        deps,
    }
}

fn find(parent: &mut [usize], x: usize) -> usize {
    let mut r = x;
    while parent[r] != r {
        r = parent[r];
    }
    let mut y = x;
    while parent[y] != r {
        let next = parent[y];
        parent[y] = r;
        y = next;
    }
    r
}

/// `selected` gathered into units in run order of their first root: a root
/// joins a selected root it depends on (the costliest dependencies first)
/// unless the unit would outgrow `cap`, so that few batches pre-process a
/// root another batch writes.
fn dependency_units(plan: &Plan, selected: &[usize], cap: u64) -> Vec<Vec<usize>> {
    let chosen: BTreeSet<usize> = selected.iter().copied().collect();
    let mut parent: Vec<usize> = (0..plan.roots.len()).collect();
    let mut size: Vec<u64> = plan.cost.iter().map(|&c| c.max(1)).collect();
    let mut edges: Vec<(u64, usize, usize)> = selected
        .iter()
        .flat_map(|&i| {
            plan.deps[i]
                .iter()
                .filter(|d| chosen.contains(d))
                .map(move |&d| (plan.cost[d].max(1), i, d))
        })
        .collect();
    edges.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    for (_, i, d) in edges {
        let (a, b) = (find(&mut parent, i), find(&mut parent, d));
        if a != b && size[a] + size[b] <= cap {
            let (low, high) = (a.min(b), a.max(b));
            parent[high] = low;
            size[low] += size[high];
        }
    }
    let mut units: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &i in selected {
        units.entry(find(&mut parent, i)).or_default().push(i);
    }
    units.into_values().collect()
}

fn make_batches(plan: &Plan, selected: &[usize], workers: usize, strategy: Strategy) -> Vec<Batch> {
    if selected.is_empty() {
        return Vec::new();
    }
    let target = (workers * 4).clamp(1, selected.len());
    let groups: Vec<BTreeSet<usize>> = match strategy {
        Strategy::Closure => {
            let total: u64 = selected.iter().map(|&i| plan.cost[i].max(1)).sum();
            let floor = total.div_ceil((workers * 8) as u64).max(1);
            let share = |remaining: u64| remaining.div_ceil((workers * 2) as u64).max(floor);
            let mut remaining = total;
            let mut groups = Vec::new();
            let mut current = BTreeSet::new();
            let mut weight = 0u64;
            for unit in dependency_units(plan, selected, share(total)) {
                let unit_weight: u64 = unit.iter().map(|&i| plan.cost[i].max(1)).sum();
                if weight > 0 && weight + unit_weight > share(remaining) {
                    groups.push(std::mem::take(&mut current));
                    remaining -= weight;
                    weight = 0;
                }
                current.extend(unit);
                weight += unit_weight;
                if weight >= share(remaining) {
                    groups.push(std::mem::take(&mut current));
                    remaining -= weight;
                    weight = 0;
                }
            }
            if !current.is_empty() {
                groups.push(current);
            }
            groups
        }
        Strategy::Components => {
            let mut parent: Vec<usize> = (0..plan.roots.len()).collect();
            for (root, deps) in plan.deps.iter().enumerate() {
                for &dep in deps {
                    let (a, b) = (find(&mut parent, root), find(&mut parent, dep));
                    if a != b {
                        parent[a.max(b)] = a.min(b);
                    }
                }
            }
            let mut components: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
            for i in 0..plan.roots.len() {
                let r = find(&mut parent, i);
                components.entry(r).or_default().insert(i);
            }
            let wanted: BTreeSet<usize> = selected.iter().map(|&i| find(&mut parent, i)).collect();
            let members: Vec<BTreeSet<usize>> = components
                .into_iter()
                .filter(|(r, _)| wanted.contains(r))
                .map(|(_, s)| s)
                .collect();
            let mut sized: Vec<(u64, BTreeSet<usize>)> = members
                .into_iter()
                .map(|s| (s.iter().map(|&i| plan.cost[i].max(1)).sum(), s))
                .collect();
            sized.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            let mut bins: Vec<(u64, BTreeSet<usize>)> = vec![(0, BTreeSet::new()); target];
            for (weight, set) in sized {
                let bin = bins
                    .iter_mut()
                    .min_by_key(|(w, s)| (*w, s.first().copied()))
                    .expect("bins");
                bin.0 += weight;
                bin.1.extend(set);
            }
            bins.into_iter()
                .map(|(_, s)| s)
                .filter(|s| !s.is_empty())
                .collect()
        }
    };
    let selected_set: BTreeSet<usize> = selected.iter().copied().collect();
    groups
        .into_iter()
        .enumerate()
        .map(|(id, group)| {
            let mut process = group.clone();
            if strategy == Strategy::Closure {
                for &i in &group {
                    process.extend(plan.deps[i].iter().copied());
                }
            }
            let deliver: BTreeSet<usize> = group.intersection(&selected_set).copied().collect();
            Batch {
                id,
                cost: process.iter().map(|&i| plan.cost[i].max(1)).sum(),
                deliver,
                process,
                round: 0,
            }
        })
        .collect()
}

fn worker_count(input: &InputSet, config: &DriverConfig) -> usize {
    let most = config
        .select
        .as_ref()
        .map_or(usize::MAX, |s| s.len().max(1));
    worker_count_unselected(input, config).min(most)
}

fn worker_count_unselected(input: &InputSet, config: &DriverConfig) -> usize {
    match config.jobs {
        Jobs::Exactly(n) => n.max(1),
        Jobs::Auto => {
            let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
            let wanted = cores
                .clamp(1, MAX_AUTO_JOBS)
                .min((input.classes().len() / AUTO_CLASSES_PER_JOB).max(1));
            let Some(available) = config.available_memory.or_else(mem_available) else {
                return wanted;
            };
            let budget = available / 10 * 9;
            (1..=wanted)
                .rev()
                .find(|&n| estimate_memory(input, config, n) <= budget)
                .unwrap_or(1)
        }
    }
}

/// A rough upper bound of a run's memory with `workers` instances: about 90
/// bytes per byte of input class files (the decompiled state and the results,
/// whatever the count), plus per instance 32 MiB, 4 bytes per byte of input
/// class files and 1 per byte of library class files (the class universe, whose
/// classes are read as they are used) unless `instance_memory` says otherwise.
/// The measured peaks it bounds are in `docs/jvm.md` ("Performance and limits").
pub fn estimate_memory(input: &InputSet, config: &DriverConfig, workers: usize) -> u64 {
    let input_bytes = input.class_bytes() as u64;
    let library_bytes = config
        .libraries
        .iter()
        .map(|l| l.class_bytes() as u64)
        .sum::<u64>();
    let per_instance = config
        .instance_memory
        .unwrap_or((32 << 20) + input_bytes * 4 + library_bytes);
    per_instance * workers as u64 + input_bytes * 90
}

/// `MemAvailable` from `/proc/meminfo`, in bytes.
pub fn mem_available() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = text.lines().find(|l| l.starts_with("MemAvailable:"))?;
    let kib: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kib * 1024)
}
