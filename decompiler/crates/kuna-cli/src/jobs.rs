//! Persistent subprocess workers for whole-binary decompilation.
//!
//! The parent fixes loader policy and inventory, schedules chunks dynamically,
//! and merges results into target-order slots. Workers keep one program load
//! across chunks; their assignment pipe also detects a dead parent.
//!
//! This module owns scheduling, recovery and synthesized-structure replay.
//! [`wire`] owns the versioned chunk and result records. The complete behavior
//! and its serial-equivalence limits are specified in `docs/spec/00-overview.md`.

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use kuna_console::engine::{EntryProvenance, ObjectLocation};
use kuna_console::project::FuncResult;
use kuna_decomp::kuna_structsynth::shard::{self, FunctionRecord, Replay, SynthRequest};

mod type_blocks;
mod wire;

pub(crate) use type_blocks::merge_type_definitions;
use wire::{decode_results, encode_spec};
pub(crate) use wire::{read_spec, ResultWriter};

/// Ceiling on an automatically planned chunk, when `--jobs-chunk` is omitted.
/// A chunk is only a scheduling unit here, so this exists to keep the tail of a
/// run from being one indivisible lump, not to bound memory.
const MAX_AUTO_CHUNK: usize = 512;

/// How many chunks the planner aims to give each worker.  More chunks balance
/// the end of a run better and cost nothing but a spec file, since a worker is
/// not restarted between them.
const CHUNKS_PER_JOB: usize = 8;

/// How many functions one worker process handles before the pool retires it.
///
/// This is the memory ceiling, and what it bounds is the allocator arena, not a
/// leak: a counting `#[global_allocator]` over 587 functions of a 274,000-function
/// IL2CPP `.so` shows the LIVE heap flat at 403 -> 411 MB (~14 KB per function
/// retained) while RSS climbs non-monotonically to 3.3 GB and does not come back
/// down.  Per-function transients are freed, but the process keeps their
/// high-water mark, and only a fresh worker returns to the floor.  RSS was
/// measured flat over the 3,000 functions one worker decompiled after its load on
/// an 18 MB PE, so the ceiling is high enough that an ordinary binary never
/// reaches it and pays no second load.
const RECYCLE_AFTER: usize = 4096;

/// Ceiling on `--jobs auto`.  Each worker pays a whole program load, so the
/// pool's memory is `jobs x worker RSS` and the machine that runs out first is a
/// desktop with many cores and ordinary RAM — the shape of the machine in the
/// report this exists for (12 cores, one 18 MB PE).  Beyond this the wall-clock
/// return is small anyway, because the parent's own load is serial.
/// [`affordable_jobs`] trims it further to what the machine's free memory holds.
const MAX_AUTO_JOBS: usize = 16;

/// The decode-lane cap, which is the engine's, not the pool's.
const MAX_DECODE_LANES: usize = kuna_analysis::listing::kuna_pdecode::MAX_DECODE_LANES;

/// The parent's word for "no more chunks", and the worker's for "chunk done".
/// Both travel as whole lines over the pipes the pool already needs for
/// liveness, so neither costs a file descriptor or a dependency.  `quit` carries
/// a token, which names the file a retiring worker leaves its type block in.
const QUIT_PREFIX: &str = "quit ";
const ACK_PREFIX: &str = "done ";

/// The parent's word for "install the replayed structures and answer from now
/// on" ([`Worker::force`]).
const SYNTH_LINE: &str = "synth";


/// Names every pool scratch directory, so a worker and the next run's sweep can
/// both recognize one without being told.
const SCRATCH_PREFIX: &str = "kuna-jobs-";

/// A worker's exit code when it outlived its parent.  Distinct from anything the
/// engine returns, and it never reaches a user: whoever would have read it is the
/// process that just died.
const EXIT_PARENT_GONE: i32 = 70;

/// A chunk stops re-running once this many of its bystanders have failed again
/// on their own and they outnumber the ones recovered.
const CHUNK_RERUN_FAILURES: usize = 8;

/// A chunk stops re-running once this many of its re-runs have stalled, each of
/// which waited out a whole stall window.
const CHUNK_RERUN_STALLS: usize = 2;

/// No re-run starts while this many re-runs have failed and they outnumber
/// every record the run's workers have delivered.
const RUN_RERUN_FAILURES: usize = 16;

/// Test-only fault injection, comma-separated: `panic:<addr>` panics a worker
/// as it starts the target at `<addr>`, `panic-once:<addr>` only the first time
/// in a run, `stall:<addr>` wedges it there until the stall watchdog kills it,
/// `spawn:<n>` refuses every worker spawn after the first `n`, `synth:serial`
/// names synthesized structures by the one-worker serial path, `synth:force`
/// renames no function and `synth:noinstall` makes a worker refuse the
/// replayed structures.  `<addr>` is a byte address (`0x` hex or
/// decimal), or `*` for every target.
pub(crate) const JOBS_FAULT_ENV: &str = "KUNA_JOBS_FAULT";

/// One decompile target, flattened out of a [`kuna_console::engine::FunctionEntry`]
/// so it can cross a thread and a process boundary (`Address` holds an `Rc`).
///
/// Every field of the entry travels, not just the address: the name, aliases,
/// extent and object coordinates a function is reported under are inventory
/// facts the PARENT resolved, and re-deriving them in a worker whose load skipped
/// discovery is exactly how a sharded document would drift from a serial one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct TargetSpec {
    pub(crate) addr: u64,
    pub(crate) space: String,
    pub(crate) name: String,
    pub(crate) aliases: Vec<String>,
    pub(crate) size: u64,
    pub(crate) object_location: Option<ObjectLocation>,
    pub(crate) provenance: EntryProvenance,
    pub(crate) binding: Option<String>,
    /// (kuna `structsynth`) The name each of this target's ledger lookups is
    /// answered with, in order, when a [`SynthWorker::Force`] worker runs it.
    pub(crate) synth: Option<Vec<Option<String>>>,
}

/// What a worker does with the synthesized-structure ledger (`--jobs-synth`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SynthWorker {
    /// `structsynth off`: the `--stream` pool, whose serial run keeps no
    /// convergence to reproduce either.
    Off,
    /// Record every lookup and let the worker's own ledger answer.
    Record,
    /// Install the parent's replayed structures and answer every lookup with
    /// the name the chunk spec gives it.
    Force,
    /// Decompile the chunk in spec order with the worker's own ledger, then run
    /// the serial convergence sweep over it.
    Serial,
}

impl SynthWorker {
    fn flag(self) -> Option<&'static str> {
        match self {
            SynthWorker::Off => None,
            SynthWorker::Record => Some("record"),
            SynthWorker::Force => Some("force"),
            SynthWorker::Serial => Some("serial"),
        }
    }

    /// Parse the worker's `--jobs-synth` value.
    pub(crate) fn parse(v: &str) -> Result<Self, String> {
        match v {
            "record" => Ok(SynthWorker::Record),
            "force" => Ok(SynthWorker::Force),
            "serial" => Ok(SynthWorker::Serial),
            _ => Err(format!("invalid --jobs-synth value {v:?}")),
        }
    }
}

/// Everything a worker needs on its command line, resolved once by the parent.
pub(crate) struct PoolConfig<'a> {
    pub(crate) jobs: usize,
    /// `--jobs auto` rather than an explicit count, so [`affordable_jobs`] knows
    /// whether it may trim the number or only warn about it.
    pub(crate) jobs_auto: bool,
    pub(crate) chunk: Option<usize>,
    pub(crate) binary: &'a str,
    pub(crate) mode: &'static str,
    pub(crate) options: &'a [(String, String)],
    pub(crate) func_decls: Vec<String>,
    pub(crate) no_vars: bool,
    pub(crate) want_proto: bool,
    pub(crate) want_provenance: bool,
    /// Ask each worker for its rendered user-defined type definitions — the
    /// `decompile-project` `.h` block, which is a whole-program artifact built
    /// after the loop from types the loop itself interns.
    pub(crate) want_types: bool,
    /// Ask each worker for [`FuncResult::callee_hints`] — the `--stream`
    /// scheduler's frontier, and the only reason a worker reports one.
    pub(crate) want_callee_hints: bool,
    pub(crate) max_fn_seconds: u64,
    pub(crate) full_load: bool,
    /// How long the PARENT's own load took.  The stall watchdog cannot fire
    /// before a worker could plausibly have finished loading, and this is the
    /// only measurement of that cost anyone has.
    pub(crate) load_seconds: f64,
    pub(crate) isa: Option<&'a str>,
    pub(crate) slice: Option<&'a str>,
    pub(crate) target: Option<&'a str>,
    pub(crate) sleighpath: Option<&'a str>,
    /// (kuna `structsynth`) The parent's ledger before any decompile, when this
    /// run synthesizes structures: the state a [`run_pool`] replays the
    /// workers' lookups from.  `None` runs every worker with `structsynth off`.
    pub(crate) synth_base: Option<Replay>,
    /// (kuna `protoorder`) The serial run this pool replays would have
    /// decompiled callees first, an order no pool takes: the structures a
    /// function mints there are the ones its callees' recovered prototypes lead
    /// it to, so the serial run the replayed names belong to is
    /// `--jobs 1 --option protoorder off` and the report says so.
    pub(crate) serial_callee_first: bool,
    /// (kuna `elemptr`) The pool decompiles a batch of one function, whose
    /// worker types globals and tables as a serial batch of one does; the
    /// workers of any larger pool type neither, as a serial run that is not
    /// callee-first does not.
    pub(crate) elem_objects: bool,
}

/// How many decode lanes `--jobs` asks the discovery walk for.
///
/// Deliberately NOT the pool count: a pool worker pays a whole program load, so
/// `auto` caps it at [`MAX_AUTO_JOBS`]; a decode lane costs one SLEIGH engine
/// (~48 MB) and saturates much later, so `auto` there is this machine's
/// parallelism capped at [`MAX_DECODE_LANES`]. An explicit `--jobs N` is honoured
/// up to the same cap.
pub(crate) fn decode_lanes(args: &crate::decompile_all::Args) -> usize {
    if args.jobs_auto {
        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        return cores.clamp(1, MAX_DECODE_LANES);
    }
    args.jobs.min(MAX_DECODE_LANES)
}

/// `--jobs N|auto`: `auto` is this machine's parallelism, capped at
/// [`MAX_AUTO_JOBS`] because every worker pays a whole program load, and trimmed
/// again by [`affordable_jobs`] once the load has shown what one worker costs.
/// The flag is returned alongside the count because it decides whether that
/// later trim may lower the number or must only warn.
pub(crate) fn parse_jobs(value: &str) -> Result<(usize, bool), String> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("auto") {
        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        return Ok((cores.clamp(1, MAX_AUTO_JOBS), true));
    }
    match v.parse::<usize>() {
        Ok(n) if n >= 1 => Ok((n, false)),
        _ => Err(format!("invalid --jobs value {value:?} (expected a positive integer or `auto`)")),
    }
}


// --- the pool ----------------------------------------------------------------

/// What a pool run produced: one [`FuncResult`] per target in target order, plus
/// the type-definition block if the caller asked for one.
pub(crate) struct PoolOutput {
    pub(crate) results: Vec<FuncResult>,
    pub(crate) types: Option<String>,
}

/// How a pool run brands its stderr: `--jobs` for the static plan, `--stream`
/// for the dynamic frontier, so every line of one run reads as one run.
pub(crate) const JOBS_TAG: &str = "--jobs";
pub(crate) const STREAM_TAG: &str = "--stream";

/// Where a pool thread gets its next chunk.
///
/// The static plan a `--jobs N` run computes up front and the dynamic,
/// result-steered frontier `--stream` schedules from are the same thing to the
/// pool: a source of slot-index lists that eventually runs dry.  Each entry is a
/// list of **slot indices** into the caller's target list, so output order
/// travels independently of the order work is done in.
pub(crate) trait ChunkSource: Sync {
    /// The next chunk, or `None` once no work is left.  Called concurrently by
    /// every pool thread.
    fn next_chunk(&self) -> Option<Vec<usize>>;
}

/// The static `--jobs` plan, handed out through a shared cursor.
struct PlannedChunks {
    plan: Vec<Vec<usize>>,
    cursor: AtomicUsize,
}

impl ChunkSource for PlannedChunks {
    fn next_chunk(&self) -> Option<Vec<usize>> {
        let idx = self.cursor.fetch_add(1, Ordering::SeqCst);
        self.plan.get(idx).cloned()
    }
}

/// Run `targets` across a pool of `cfg.jobs` worker processes.  `inventory` is
/// the parent's full canonical function inventory, replayed into every worker so
/// a discovery-free worker load still resolves every call the serial run did.
///
/// A run that synthesizes structures ([`PoolConfig::synth_base`]) goes on to
/// [`name_structs_serially`] once every function has come back, so each
/// `struct_N` is the name `--jobs 1` gives it.
pub(crate) fn run_pool(
    cfg: &PoolConfig,
    targets: &[TargetSpec],
    inventory: &[TargetSpec],
) -> Result<PoolOutput, String> {
    let total = targets.len();
    if total == 0 {
        return Ok(PoolOutput { results: Vec::new(), types: None });
    }
    let jobs = affordable_jobs(cfg, JOBS_TAG);
    let session = Session::open(cfg, inventory)?;
    let first = Phase {
        synth: if cfg.synth_base.is_some() { SynthWorker::Record } else { SynthWorker::Off },
        table: None,
    };
    // Dispatch order is NOT output order: work is planned longest-first into
    // equal-work chunks, and every result is filed by its slot index, so the
    // merged document does not depend on how the pool scheduled anything.
    let plan = plan_chunks(targets, cfg.chunk, jobs);
    let banner = |plan: &[Vec<usize>], workers: usize| {
        format!(
            "[kuna --jobs] {total} functions, {} chunk(s) of {}..{}, {workers} worker process(es)",
            plan.len(),
            plan.iter().map(Vec::len).min().unwrap_or(0),
            plan.iter().map(Vec::len).max().unwrap_or(0)
        )
    };
    let mut run = run_planned(cfg, &session, jobs, targets, plan, &first, &banner)?;
    let named = match &cfg.synth_base {
        Some(base) => name_structs_serially(cfg, &session, jobs, targets, base.clone(), &mut run)?,
        None => Named { kind: first.synth, table: None },
    };
    let blocks = session.close(cfg, &named);
    let PlannedRun { results, retries } = run;
    warn_about_anomalies(&results, cfg.max_fn_seconds, retries);
    // After the anomaly warnings, which report the run itself: the type-shard
    // disagreement is a note about one artifact.
    let types =
        (cfg.want_types && !blocks.is_empty()).then(|| merge_type_definitions(&blocks, JOBS_TAG));
    Ok(PoolOutput { results, types })
}

/// What one pool over a fixed plan brought back: a result per target, in
/// target order, and what its re-runs did.
struct PlannedRun {
    results: Vec<FuncResult>,
    retries: Retries,
}

/// Serve `plan` over `targets` with at most `jobs` workers of kind `phase`,
/// filing every result by its slot.
fn run_planned(
    cfg: &PoolConfig,
    session: &Session,
    jobs: usize,
    targets: &[TargetSpec],
    plan: Vec<Vec<usize>>,
    phase: &Phase,
    banner: &dyn Fn(&[Vec<usize>], usize) -> String,
) -> Result<PlannedRun, String> {
    let total = targets.len();
    let workers = jobs.min(plan.len());
    let banner = banner(&plan, workers);
    let source = PlannedChunks { plan, cursor: AtomicUsize::new(0) };
    let slots: Mutex<Vec<Option<FuncResult>>> = Mutex::new((0..total).map(|_| None).collect());
    let retries = run_pool_with(
        cfg,
        session,
        phase,
        targets,
        &source,
        &PoolReport { tag: JOBS_TAG, banner, workers, finish_delivered: false },
        &|indices: &[usize], produced: Vec<FuncResult>| {
            let mut slots = slots.lock().unwrap_or_else(|e| e.into_inner());
            for (&slot, r) in indices.iter().zip(produced) {
                slots[slot] = Some(r);
            }
        },
    )?;
    let slots = slots.into_inner().unwrap_or_else(|e| e.into_inner());
    let results = slots
        .into_iter()
        .zip(targets)
        .map(|(slot, t)| slot.unwrap_or_else(|| lost_result(t, NO_RECORD)))
        .collect();
    Ok(PlannedRun { results, retries })
}

/// Which worker a pool spawns, and the replayed structures a
/// [`SynthWorker::Force`] worker installs before its first chunk.
struct Phase<'t> {
    synth: SynthWorker,
    table: Option<&'t [u8]>,
}

/// How a sharded run's synthesized structures were named: the kind of worker
/// whose type blocks speak for the document, and the replayed table when the
/// names are the replay's.
struct Named {
    kind: SynthWorker,
    table: Option<Vec<u8>>,
}

/// (kuna `structsynth`) Give every synthesized structure of a sharded run the
/// name, the layout and the reuse `--jobs 1` gives it.
///
/// A serial run names `struct_N` in decompile order, each ledger lookup reading
/// what the functions before it minted, and then decompiles once more every
/// function that names a structure a later, larger one superseded
/// (`converge_synthesized_structs`). A worker sees only its own share of the
/// functions, so its answers are its own. What a function ASKS the ledger does
/// not depend on the answers it got before, as a rule, so the first pool's
/// workers record every lookup, and this replays them in target order through
/// the ledger's own decision ([`Replay`]): every answer, every mint, the
/// superseded set and the answers the convergence sweep will get.
///
/// A function whose own worker answered every lookup with a structure that has
/// exactly the members of the serial answer's -- which a function that mints
/// what it measured usually did -- keeps its first decompile with the numbers
/// renamed ([`shard::renaming`]). Only the others are decompiled again, by the
/// first pool's workers: each forgets the structures it minted itself,
/// installs the replayed ones and answers every lookup with its replayed name
/// (`synth` on its assignment pipe); the ones the sweep will redo with a
/// different answer go out in the same pool.
///
/// The second decompile records its lookups too, and the exception to the rule
/// shows up there: a function whose first answers change what it does next --
/// a restart, a second drive -- asks something else once it is answered as the
/// serial run answers it. Its questions up to the first difference were answered
/// right, so the replay takes the new record and runs again, and only the
/// functions whose answers moved are renamed or decompiled once more. When that
/// does not settle within [`SYNTH_ROUNDS`], when the sweep would ask something
/// new, when a structure cannot be rebuilt in another process, or when a
/// second decompile fails where the first did not (a worker that died, a
/// table it could not install), the functions that asked are decompiled again
/// in target order by ONE worker running the ledger and the sweep itself: the
/// serial computation, over the only functions that take part in it.
fn name_structs_serially(
    cfg: &PoolConfig,
    session: &Session,
    jobs: usize,
    targets: &[TargetSpec],
    base: Replay,
    run: &mut PlannedRun,
) -> Result<Named, String> {
    let records: Vec<Option<FunctionRecord>> = run.results.iter_mut().map(|r| r.synth.take()).collect();
    let mut asked: Vec<Vec<SynthRequest>> =
        records.iter().map(|r| r.as_ref().map(|r| r.requests.clone()).unwrap_or_default()).collect();
    let askers: Vec<usize> = (0..asked.len()).filter(|&i| !asked[i].is_empty()).collect();
    if askers.is_empty() {
        return Ok(Named { kind: SynthWorker::Record, table: None });
    }
    let fallback = |run: &mut PlannedRun, why: &str| serial_fallback(cfg, session, targets, &askers, run, why);
    let faults = Faults::from_env();
    if faults.synth_serial {
        return fallback(run, &format!("{JOBS_FAULT_ENV} asked for the serial path"));
    }
    // A function the watchdog or a dead worker cut short asks a different
    // number of questions each time it runs; its record is taken as it comes.
    let cut_short: Vec<bool> = run.results.iter().map(|r| r.error.is_some()).collect();
    // What each function's own worker answered, while its record stands.
    let mut own: Vec<Option<Vec<shard::Answer>>> = records
        .iter()
        .zip(&cut_short)
        .map(|(r, &cut)| if cut || faults.synth_force { None } else { r.as_ref()?.own_answers() })
        .collect();
    let held = base.held();

    // Every kept first and sweep decompile, with the answers it was given.
    let mut firsts: HashMap<usize, KeptFirst> = HashMap::new();
    let mut sweeps: HashMap<usize, (AnswerKey, FuncResult)> = HashMap::new();
    let mut forced = 0usize;
    let mut round = 0;
    let plan = loop {
        let plan = match SynthPlan::replay(&mut base.clone(), &asked) {
            Ok(plan) => plan,
            Err(why) => return fallback(run, &why),
        };
        for &i in &askers {
            if firsts.get(&i).is_none_or(|kept| !plan.matches(&plan.first[i], &kept.key)) {
                let first = plan.first_key(i);
                if let Some(r) = rename_result(&run.results[i], own[i].as_deref(), &first, &held) {
                    firsts.insert(i, KeptFirst { key: first, result: r, renamed: true });
                }
            }
            let Some(answers) = plan.sweep[i].as_ref().filter(|_| plan.redo_predicted(i)) else { continue };
            if sweeps.get(&i).is_none_or(|(kept, _)| !plan.matches(answers, kept)) {
                let key = plan.key(answers);
                if let Some(r) = rename_result(&run.results[i], own[i].as_deref(), &key, &held) {
                    sweeps.insert(i, (key, r));
                }
            }
        }
        let stale_first = |i: usize| {
            firsts.get(&i).is_none_or(|kept| !plan.matches(&plan.first[i], &kept.key))
        };
        let stale_sweep = |i: usize| {
            plan.redo_predicted(i) && sweeps.get(&i).is_none_or(|(key, _)| {
                plan.sweep[i].as_ref().is_none_or(|answers| !plan.matches(answers, key))
            })
        };
        let list: Vec<(usize, bool)> = askers
            .iter()
            .filter(|&&i| stale_first(i))
            .map(|&i| (i, false))
            .chain(askers.iter().filter(|&&i| stale_sweep(i)).map(|&i| (i, true)))
            .collect();
        if list.is_empty() {
            break plan;
        }
        if round == SYNTH_ROUNDS {
            return fallback(run, "the replayed questions did not settle");
        }
        round += 1;
        forced += list.len();
        for (i, sweep, r, record) in run_forced(cfg, session, jobs, targets, &plan, &list)? {
            if r.error.is_some() && !cut_short[i] {
                return fallback(run, "a function failed when decompiled again with the serial names");
            }
            let agrees = record.as_ref().is_some_and(|rec| asks_the_same(rec, &asked[i]));
            if agrees || cut_short[i] {
                if sweep {
                    sweeps.insert(i, (plan.sweep_key(i).unwrap_or_default(), r));
                } else {
                    firsts.insert(i, KeptFirst { key: plan.first_key(i), result: r, renamed: false });
                }
                continue;
            }
            match record {
                Some(rec) if !sweep => {
                    asked[i] = rec.requests;
                    own[i] = None;
                    firsts.remove(&i);
                }
                _ => return fallback(run, "a function asked the ledger something new in the sweep"),
            }
        }
    };
    let mut renamed = 0;
    for (i, kept) in firsts {
        renamed += usize::from(kept.renamed);
        run.results[i] = kept.result;
    }

    // The serial sweep: decided on the first-pass text, then each redo in
    // target order.
    let mut again: Vec<(usize, FuncResult)> = Vec::new();
    let mut leftover: Vec<(usize, bool)> = Vec::new();
    for i in (0..run.results.len())
        .filter(|&i| kuna_console::project::names_any_type(&run.results[i], &plan.stale))
    {
        if asked[i].is_empty() {
            continue;
        }
        match &plan.sweep[i] {
            None => return fallback(run, "the convergence sweep would mint a structure"),
            Some(s) if *s == plan.first[i] => {}
            Some(_) => match sweeps.remove(&i) {
                Some((key, r)) if plan.sweep[i].as_ref().is_some_and(|answers| plan.matches(answers, &key)) => {
                    again.push((i, r));
                }
                _ => leftover.push((i, true)),
            },
        }
    }
    if !leftover.is_empty() {
        forced += leftover.len();
        for (i, _, r, record) in run_forced(cfg, session, jobs, targets, &plan, &leftover)? {
            if r.error.is_some() && !cut_short[i] {
                return fallback(run, "a function failed when decompiled again with the serial names");
            }
            let agrees = record.as_ref().is_some_and(|rec| asks_the_same(rec, &asked[i]));
            if !(agrees || cut_short[i]) {
                return fallback(run, "a function asked the ledger something new in the sweep");
            }
            again.push((i, r));
        }
    }
    // The `.h` renders the structures from the workers that hold them, each
    // with the other types its own functions interned.
    if cfg.want_types && !plan.minted.is_empty() {
        if let Err(why) = install_on_idle_workers(cfg, session, &plan.table) {
            return fallback(run, &why);
        }
    }
    again.sort_by_key(|(i, _)| *i);
    for (i, r) in again {
        if kuna_console::project::redo_replaces(&run.results[i], &r) {
            run.results[i] = r;
        }
    }
    eprintln!(
        "[kuna --jobs] structsynth: {} function(s) with synthesized structures named as {} names \
         them: {renamed} renamed, {forced} decompile(s) again",
        askers.len(),
        serial_run(cfg)
    );
    Ok(Named { kind: SynthWorker::Force, table: Some(plan.table) })
}

/// Whether every name `map` covers in `r` is a TYPE name.
///
/// The substitution is textual, so a function, alias or variable literally
/// called `struct_3` would be rewritten along with the structure of that name.
/// Nothing kuna names spells one today; a binary whose symbols do takes the
/// second decompile rather than a wrong rename.
fn only_types_are_renamed(r: &FuncResult, map: &[(String, String)]) -> bool {
    let covered = |name: &str| map.iter().any(|(own, _)| own == name);
    !covered(&r.name)
        && !r.aliases.iter().any(|a| covered(a))
        && !r.variables.iter().any(|v| covered(&v.name))
}

/// `r` with its structures renamed from `own` to `serial`, or `None` when the
/// two answer lists do not name structures with the same members, or the
/// function's text names one the renaming does not cover.
fn rename_result(
    r: &FuncResult,
    own: Option<&[shard::Answer]>,
    serial: &[shard::Answer],
    held: &[String],
) -> Option<FuncResult> {
    let map = shard::renaming(own?, serial)?;
    if !only_types_are_renamed(r, &map) {
        return None;
    }
    let text = |s: &str| shard::rename_identifiers(s, &map, held);
    let mut out = r.clone();
    out.code = match r.code.as_deref() {
        Some(code) => Some(text(code)?),
        None => None,
    };
    out.proto = match r.proto.as_deref() {
        Some(proto) => Some(text(proto)?),
        None => None,
    };
    for v in &mut out.variables {
        v.type_name = text(&v.type_name)?;
    }
    for t in &mut out.types {
        t.name = text(&t.name)?;
        t.definition = text(&t.definition)?;
    }
    Some(out)
}

/// Did a forced decompile ask what the replay was told it asks, repeats aside,
/// and take exactly the answers it was given?
fn asks_the_same(record: &FunctionRecord, asked: &[SynthRequest]) -> bool {
    !record.off_script && shard::distinct(&record.requests) == shard::distinct(asked)
}

/// How many times the replay takes a function's corrected questions and runs
/// again before the run falls back to one ordered worker.
const SYNTH_ROUNDS: usize = 4;

/// What a function's lookups were answered with, each name paired with the
/// definition the replay minted under it (a name held before the run stands for
/// itself), so a kept decompile is reused only when both still hold.
type AnswerKey = Vec<shard::Answer>;

struct KeptFirst {
    key: AnswerKey,
    result: FuncResult,
    renamed: bool,
}

/// The replayed ledger of one run: the answers each function's first decompile
/// gets, the answers its redo would get, the superseded names, and the table
/// every forced worker installs.
struct SynthPlan {
    first: Vec<Vec<Option<String>>>,
    /// `None` where the sweep's lookup would mint, which the serial sweep never
    /// does for a function that asks what it asked the first time.
    sweep: Vec<Option<Vec<Option<String>>>>,
    stale: Vec<String>,
    table: Vec<u8>,
    minted: HashMap<String, SynthRequest>,
}

impl SynthPlan {
    fn replay(replay: &mut Replay, asked: &[Vec<SynthRequest>]) -> Result<SynthPlan, String> {
        let mut first = Vec::with_capacity(asked.len());
        for requests in asked {
            // Every asking goes through the ledger, as it does serially; a
            // forced worker answers a repeat as it answered the first asking,
            // so only the distinct questions carry answers.
            let mut answers: Vec<(&SynthRequest, Option<String>)> = Vec::new();
            for q in requests {
                let before = replay.table().len();
                let answer = replay.lookup_or_mint(q);
                if replay.table().len() > before && !q.portable() {
                    return Err("a synthesized structure has a field type another process \
                                cannot rebuild"
                        .into());
                }
                match answers.iter().find(|(asked, _)| *asked == q) {
                    Some((_, earlier)) if *earlier != answer => {
                        return Err("a function's repeated question got another answer".into())
                    }
                    Some(_) => {}
                    None => answers.push((q, answer)),
                }
            }
            // Whether the function repeats a question can depend on what its
            // process decompiled before, so every question must get the same
            // answer at the end of the function as when it was first asked.
            if answers.iter().any(|(q, a)| replay.lookup(q) != Ok(a.clone())) {
                return Err("a function's own structures would answer its earlier question \
                            differently"
                    .into());
            }
            first.push(answers.into_iter().map(|(_, a)| a).collect());
        }
        let sweep = asked
            .iter()
            .map(|requests| {
                shard::distinct(requests).iter().map(|q| replay.lookup(q)).collect::<Result<_, _>>().ok()
            })
            .collect();
        Ok(SynthPlan {
            first,
            sweep,
            stale: replay.superseded_names(),
            table: shard::encode_table(replay.table()),
            minted: replay.table().iter().cloned().collect(),
        })
    }

    fn key(&self, answers: &[Option<String>]) -> AnswerKey {
        answers
            .iter()
            .map(|a| a.as_ref().map(|n| (n.clone(), self.minted.get(n).cloned())))
            .collect()
    }

    fn first_key(&self, i: usize) -> AnswerKey {
        self.key(&self.first[i])
    }

    fn matches(&self, answers: &[Option<String>], key: &[shard::Answer]) -> bool {
        answers
            .iter()
            .map(|answer| answer.as_deref().map(|name| (name, self.minted.get(name))))
            .eq(key.iter().map(|answer| {
                answer.as_ref().map(|(name, request)| (name.as_str(), request.as_ref()))
            }))
    }

    fn sweep_key(&self, i: usize) -> Option<AnswerKey> {
        self.sweep[i].as_ref().map(|s| self.key(s))
    }

    /// Will the sweep, as far as the answers tell, decompile `i` again with
    /// different answers? The text decides whether it really does.
    fn redo_predicted(&self, i: usize) -> bool {
        self.first[i].iter().flatten().any(|n| self.stale.contains(n))
            && self.sweep[i].as_ref().is_some_and(|s| *s != self.first[i])
    }
}

/// One forced decompile: its target slot, whether it was the sweep's, its
/// result, and what it asked the ledger.
type ForcedRun = (usize, bool, FuncResult, Option<FunctionRecord>);

/// Decompile each `(slot, sweep)` of `list` again on workers holding the
/// replayed table, each lookup answered from `plan`.
fn run_forced(
    cfg: &PoolConfig,
    session: &Session,
    jobs: usize,
    targets: &[TargetSpec],
    plan: &SynthPlan,
    list: &[(usize, bool)],
) -> Result<Vec<ForcedRun>, String> {
    let specs: Vec<TargetSpec> = list
        .iter()
        .map(|&(i, sweep)| {
            let answers = if sweep { plan.sweep[i].clone() } else { Some(plan.first[i].clone()) };
            TargetSpec { synth: answers, ..targets[i].clone() }
        })
        .collect();
    // A chunk never holds one function twice: records are matched back by
    // address, and a function the sweep redoes is here twice.
    let split = list.iter().position(|&(_, sweep)| sweep).unwrap_or(list.len());
    let mut chunks = plan_chunks(&specs[..split], cfg.chunk, jobs);
    chunks.extend(
        plan_chunks(&specs[split..], cfg.chunk, jobs)
            .into_iter()
            .map(|c| c.into_iter().map(|k| k + split).collect()),
    );
    let n = specs.len();
    let banner = |plan: &[Vec<usize>], workers: usize| {
        format!(
            "[kuna --jobs] structsynth: {n} decompile(s) of the functions with synthesized \
             structures again, with the serial names, {} chunk(s), {workers} worker process(es)",
            plan.len()
        )
    };
    let phase = Phase { synth: SynthWorker::Force, table: Some(&plan.table) };
    let run = run_planned(cfg, session, jobs, &specs, chunks, &phase, &banner)?;
    Ok(run
        .results
        .into_iter()
        .zip(list)
        .map(|(mut r, &(i, sweep))| {
            let record = r.synth.take();
            (i, sweep, r, record)
        })
        .collect())
}

/// Make every idle worker hold `table` -- a new one, when no worker is left
/// that could -- and wait until each says it installed it: the `.h` renders
/// the replayed structures from the workers that hold them, and a worker whose
/// every function kept its first decompile never took the table.
fn install_on_idle_workers(cfg: &PoolConfig, session: &Session, table: &[u8]) -> Result<(), String> {
    let id = session.write_table(table)?;
    let idle = std::mem::take(&mut *session.idle.lock().unwrap_or_else(|e| e.into_inner()));
    let (ready, mut stale): (Vec<Worker>, Vec<Worker>) =
        idle.into_iter().partition(|w| w.synth == SynthWorker::Force && w.table == id);
    if ready.is_empty() && stale.is_empty() && !session.holds_table(table) {
        let exe = std::env::current_exe().map_err(|e| format!("cannot locate the kuna binary: {e}"))?;
        let mut w = Worker::spawn(cfg, SynthWorker::Force, &exe, session.path())
            .map_err(|e| format!("{SPAWN_FAILED}: {e}"))?;
        w.table = id;
        stale.push(w);
    }
    session.idle.lock().unwrap_or_else(|e| e.into_inner()).extend(ready);
    let failed = AtomicBool::new(false);
    std::thread::scope(|scope| {
        for mut w in stale {
            let failed = &failed;
            scope.spawn(move || {
                let idx = session.chunk_ids.fetch_add(1, Ordering::SeqCst);
                if std::fs::write(session.path().join(spec_name(idx)), encode_spec(&[])).is_err() {
                    session.retire(cfg, w);
                    failed.store(true, Ordering::SeqCst);
                    return;
                }
                // An empty chunk acknowledged after the `synth` line is the
                // table installed; a worker that died instead is gone.
                if (w.stale_for(id) && !w.force(id)) || w.run_chunk(cfg, session.path(), idx, 0).2 != Ending::Finished {
                    failed.store(true, Ordering::SeqCst);
                    return;
                }
                session.idle.lock().unwrap_or_else(|e| e.into_inner()).push(w);
            });
        }
    });
    if failed.load(Ordering::SeqCst) {
        return Err("a worker could not install the synthesized structures".into());
    }
    Ok(())
}

/// The serial run a sharded one is replaying, spelled as a command line.
///
/// (kuna `protoorder`) On `decompile-all` the serial default decompiles callees
/// first, which decides what a function mints as much as the ledger does; a pool
/// cannot take that order ([`crate::decompile_all`] says so on its own line), so
/// the run whose names these are is the one with the order turned off.
fn serial_run(cfg: &PoolConfig) -> &'static str {
    if cfg.serial_callee_first {
        "--jobs 1 --option protoorder off"
    } else {
        "--jobs 1"
    }
}

/// The functions that asked the ledger, decompiled again in target order by one
/// worker running the ledger and the convergence sweep itself.
fn serial_fallback(
    cfg: &PoolConfig,
    session: &Session,
    targets: &[TargetSpec],
    askers: &[usize],
    run: &mut PlannedRun,
    why: &str,
) -> Result<Named, String> {
    eprintln!(
        "[kuna --jobs] note: {why}, so the {} function(s) with synthesized structures are \
         decompiled again in order by one worker process, as {} would.",
        askers.len(),
        serial_run(cfg)
    );
    let specs: Vec<TargetSpec> = askers.iter().map(|&i| targets[i].clone()).collect();
    let n = specs.len();
    let banner = |_: &[Vec<usize>], _: usize| {
        format!("[kuna --jobs] structsynth: {n} function(s) in one ordered chunk, 1 worker process")
    };
    let phase = Phase { synth: SynthWorker::Serial, table: None };
    let serial = run_planned(cfg, session, 1, &specs, vec![(0..n).collect()], &phase, &banner)?;
    for (&i, r) in askers.iter().zip(serial.results) {
        run.results[i] = r;
    }
    run.retries.recovered += serial.retries.recovered;
    run.retries.failed_alone += serial.retries.failed_alone;
    Ok(Named { kind: SynthWorker::Serial, table: None })
}

/// The `--stream` pool: the same workers over a caller-supplied dynamic
/// [`ChunkSource`], with each finished chunk handed to `sink` as it lands
/// instead of being filed into a slot table.  Returns each retired worker's
/// type block, unmerged — the streamed caller merges its own factory in as one
/// more shard — and what re-running a dead worker's targets recovered, for the
/// caller's anomaly warnings.
///
/// `workers_out` reports how many workers the memory trim actually left, which
/// is what `.streaming` publishes.
///
/// The caller owns the record keeping a streamed run needs: one record per
/// target, the anomaly warnings and the end-of-run reconciliation.
pub(crate) fn run_pool_streaming(
    cfg: &PoolConfig,
    targets: &[TargetSpec],
    inventory: &[TargetSpec],
    source: &dyn ChunkSource,
    workers_out: &AtomicUsize,
    sink: &(dyn Fn(&[usize], Vec<FuncResult>) + Sync),
) -> Result<(Vec<String>, Retries), String> {
    let total = targets.len();
    if total == 0 {
        return Ok((Vec::new(), Retries::default()));
    }
    let workers = affordable_jobs(cfg, STREAM_TAG).min(total);
    workers_out.store(workers, Ordering::SeqCst);
    let banner = format!(
        "[kuna --stream] {total} functions, entry-point-first dynamic scheduling, \
         {workers} worker process(es)"
    );
    let session = Session::open(cfg, inventory)?;
    let retries = run_pool_with(
        cfg,
        &session,
        &Phase { synth: SynthWorker::Off, table: None },
        targets,
        source,
        &PoolReport { tag: STREAM_TAG, banner, workers, finish_delivered: true },
        sink,
    )?;
    let blocks = session.close(cfg, &Named { kind: SynthWorker::Off, table: None });
    Ok((blocks, retries))
}

/// What the pools of one run share: the scratch directory every worker was
/// started on, the chunk and retirement counters, the live workers one pool
/// hands to the next, and every retired worker's type blocks with the kind of
/// worker it was when it rendered them.
struct Session {
    scratch: ScratchDir,
    chunk_ids: AtomicUsize,
    retired: AtomicUsize,
    idle: Mutex<Vec<Worker>>,
    blocks: Mutex<Vec<RetiredBlocks>>,
    /// Which replayed table sits in the scratch directory, 0 for none and one
    /// more each time a different one is written, and its bytes.
    table: Mutex<(usize, Vec<u8>)>,
}

/// One retired worker's type blocks: rendered with everything its factory
/// held, and (a worker that took part in naming structures) once more
/// without the structures it minted or installed.
struct RetiredBlocks {
    kind: SynthWorker,
    table: usize,
    full: String,
    rest: Option<String>,
}

impl Session {
    fn open(cfg: &PoolConfig, inventory: &[TargetSpec]) -> Result<Session, String> {
        let scratch = ScratchDir::create()?;
        if !cfg.full_load {
            let path = scratch.path().join(INVENTORY_FILE);
            std::fs::write(&path, encode_spec(inventory))
                .map_err(|e| format!("cannot write the worker inventory {}: {e}", path.display()))?;
        }
        Ok(Session {
            scratch,
            chunk_ids: AtomicUsize::new(0),
            retired: AtomicUsize::new(0),
            idle: Mutex::new(Vec::new()),
            blocks: Mutex::new(Vec::new()),
            table: Mutex::new((0, Vec::new())),
        })
    }

    fn path(&self) -> &Path {
        self.scratch.path()
    }

    /// Put `table` where a forcing worker reads it, unless it is there
    /// already, and return its number.
    fn write_table(&self, table: &[u8]) -> Result<usize, String> {
        let mut held = self.table.lock().unwrap_or_else(|e| e.into_inner());
        if held.0 > 0 && held.1 == table {
            return Ok(held.0);
        }
        let path = self.path().join(SYNTH_TABLE_FILE);
        std::fs::write(&path, table)
            .map_err(|e| format!("cannot write the synthesized structures {}: {e}", path.display()))?;
        *held = (held.0 + 1, table.to_vec());
        Ok(held.0)
    }

    /// The number of the table in the scratch directory, when it is `table`.
    fn table_id(&self, table: &[u8]) -> Option<usize> {
        let held = self.table.lock().unwrap_or_else(|e| e.into_inner());
        (held.0 > 0 && held.1 == table).then_some(held.0)
    }

    /// The number of the latest table.
    fn latest_table(&self) -> usize {
        self.table.lock().unwrap_or_else(|e| e.into_inner()).0
    }

    /// Does a live or retired worker hold `table`?
    fn holds_table(&self, table: &[u8]) -> bool {
        let Some(id) = self.table_id(table) else { return false };
        let holds = |kind: SynthWorker, t: usize| kind == SynthWorker::Force && t == id;
        self.idle.lock().unwrap_or_else(|e| e.into_inner()).iter().any(|w| holds(w.synth, w.table))
            || self.blocks.lock().unwrap_or_else(|e| e.into_inner()).iter().any(|b| holds(b.kind, b.table))
    }

    /// Retiring a worker is where its type blocks come from.
    fn retire(&self, cfg: &PoolConfig, w: Worker) {
        let token = self.retired.fetch_add(1, Ordering::SeqCst);
        let (kind, table) = (w.synth, w.table);
        if let Some((full, rest)) = w.quit(cfg, self.path(), token) {
            self.blocks.lock().unwrap_or_else(|e| e.into_inner()).push(RetiredBlocks { kind, table, full, rest });
        }
    }

    /// Retire every worker still alive, in parallel, and return the type
    /// blocks that speak for the document: in full from each worker that
    /// ended as `named.kind` (holding `named.table`, when there is one), first;
    /// then every other worker's without the structures it minted, for the
    /// types its functions interned.
    fn close(&self, cfg: &PoolConfig, named: &Named) -> Vec<String> {
        let idle = std::mem::take(&mut *self.idle.lock().unwrap_or_else(|e| e.into_inner()));
        std::thread::scope(|scope| {
            for w in idle {
                scope.spawn(move || self.retire(cfg, w));
            }
        });
        let table = named.table.as_deref().and_then(|t| self.table_id(t));
        let speaks = |b: &RetiredBlocks| {
            b.kind == named.kind && (named.kind != SynthWorker::Force || Some(b.table) == table)
        };
        let blocks = std::mem::take(&mut *self.blocks.lock().unwrap_or_else(|e| e.into_inner()));
        let (full, other): (Vec<RetiredBlocks>, Vec<RetiredBlocks>) = blocks.into_iter().partition(|b| speaks(b));
        // A worker that numbered structures of its own and could not say what
        // it held without them says nothing.
        let rest = other.into_iter().filter_map(|b| match b.kind {
            SynthWorker::Record | SynthWorker::Force => b.rest,
            SynthWorker::Off | SynthWorker::Serial => Some(b.full),
        });
        full.into_iter().map(|b| b.full).chain(rest).collect()
    }
}

/// The stderr brand of a pool run.
struct PoolReport {
    /// `--jobs` or `--stream`: every line this run prints is `[kuna <tag>]`.
    tag: &'static str,
    banner: String,
    workers: usize,
    /// Does the closing line count the targets asked for, or the results that
    /// came back?  A `--jobs` run serves every chunk it planned, so the two are
    /// the same; a streamed one stops where its writer died, and `done: <every
    /// target>` above the error that stopped it reads as a finished export.
    finish_delivered: bool,
}

/// The pool itself: `report.workers` threads, each driving one worker process
/// through chunk after chunk of `source` until it runs dry, with every target's
/// final record handed to `sink` exactly once as `(slot indices, results)`.
/// Returns one rendered user-defined type block per retired worker when
/// `cfg.want_types` asked for them, and what the re-runs recovered.
fn run_pool_with(
    cfg: &PoolConfig,
    session: &Session,
    phase: &Phase,
    targets: &[TargetSpec],
    source: &dyn ChunkSource,
    report: &PoolReport,
    sink: &(dyn Fn(&[usize], Vec<FuncResult>) + Sync),
) -> Result<Retries, String> {
    let workers = report.workers;
    let total = targets.len();
    let exe = std::env::current_exe().map_err(|e| format!("cannot locate the kuna binary: {e}"))?;
    if let Some(table) = phase.table {
        session.write_table(table)?;
    }
    eprintln!("{}", report.banner);
    if phase.synth == SynthWorker::Off {
        if let Some(note) = structsynth_shard_note(cfg, report.tag) {
            eprintln!("{note}");
        }
    }

    let pool = Pool {
        cfg,
        session,
        synth: phase.synth,
        exe: &exe,
        scratch: session.path(),
        tag: report.tag,
        spawned: AtomicUsize::new(0),
        faults: Faults::from_env_reporting(report.tag),
        gate: RetryGate::default(),
        recovered: AtomicUsize::new(0),
        failed_alone: AtomicUsize::new(0),
    };
    let completed = AtomicUsize::new(0);
    let worker_ids = AtomicUsize::new(0);
    let progress = Progress::new(total, workers, report.tag);
    let start = Instant::now();

    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                let worker_id = worker_ids.fetch_add(1, Ordering::SeqCst);
                // A worker an earlier pool of this run left alive is taken
                // before a new one is spawned: its load is already paid.
                let mut worker: Option<Worker> =
                    session.idle.lock().unwrap_or_else(|e| e.into_inner()).pop();
                let deliver = |indices: &[usize], results: Vec<FuncResult>| {
                    sink(indices, results);
                    let done = completed.fetch_add(indices.len(), Ordering::SeqCst) + indices.len();
                    progress.report(worker_id, done, start);
                };
                loop {
                    let Some(indices) = source.next_chunk() else { break };
                    pool.serve_with_retries(&mut worker, targets, &indices, &deliver);
                    // Recycling returns a worker to the memory floor a process
                    // cannot reach on its own; it costs a whole program load, so
                    // it is a ceiling rather than a rhythm.
                    if worker.as_ref().is_some_and(|w| w.functions_done >= RECYCLE_AFTER) {
                        if let Some(w) = worker.take() {
                            session.retire(cfg, w);
                        }
                    }
                }
                // Parked for the run's next pool; [`Session::close`] retires it.
                if let Some(w) = worker.take() {
                    session.idle.lock().unwrap_or_else(|e| e.into_inner()).push(w);
                }
            });
        }
    });

    let delivered = completed.load(Ordering::SeqCst);
    progress.finish(if report.finish_delivered { delivered } else { total }, start);
    Ok(pool.retries())
}

/// What every pool thread shares besides the caller's source and sink.
struct Pool<'r> {
    cfg: &'r PoolConfig<'r>,
    session: &'r Session,
    synth: SynthWorker,
    exe: &'r Path,
    scratch: &'r Path,
    tag: &'static str,
    spawned: AtomicUsize,
    faults: Faults,
    gate: RetryGate,
    recovered: AtomicUsize,
    failed_alone: AtomicUsize,
}

impl Pool<'_> {
    /// Serve the chunk at `indices`, then re-run alone each target its worker
    /// died before delivering, in [`retry_order`].  What will not be re-run
    /// reaches `deliver` at once and each re-run as it lands, so every target
    /// is delivered exactly once whatever happens to the re-runs; a target the
    /// chunk stops before keeps its record, marked as not re-run.
    fn serve_with_retries(
        &self,
        worker: &mut Option<Worker>,
        targets: &[TargetSpec],
        indices: &[usize],
        deliver: &dyn Fn(&[usize], Vec<FuncResult>),
    ) {
        let chunk: Vec<TargetSpec> = indices.iter().map(|&i| targets[i].clone()).collect();
        let served = self.serve(worker, &chunk);
        let reruns = retry_order(served.ending, &served.delivered);
        if reruns.is_empty() {
            deliver(indices, served.results);
            return;
        }
        let mut results: Vec<Option<FuncResult>> = served.results.into_iter().map(Some).collect();
        let order: Vec<(usize, bool)> = reruns
            .suspect
            .map(|p| (p, true))
            .into_iter()
            .chain(reruns.bystanders.iter().map(|&p| (p, false)))
            .collect();
        let mut rerun = vec![false; chunk.len()];
        for &(p, _) in &order {
            rerun[p] = true;
        }
        let (now, landed): (Vec<usize>, Vec<FuncResult>) = (0..chunk.len())
            .filter(|&p| !rerun[p])
            .filter_map(|p| Some((indices[p], results[p].take()?)))
            .unzip();
        if !now.is_empty() {
            deliver(&now, landed);
        }

        let (mut kept, mut kept_results) = (Vec::new(), Vec::new());
        let mut stopped: Option<String> = None;
        let mut tally = ChunkReruns::default();
        for (p, suspect) in order {
            let Some(original) = results[p].take() else { continue };
            if let Some(why) = &stopped {
                kept.push(indices[p]);
                kept_results.push(not_rerun(original, why));
                continue;
            }
            if !self.gate.allows() {
                if self.gate.first_refusal() {
                    eprintln!(
                        "[kuna {}] warning: {RUN_RERUN_FAILURES} or more functions have failed \
                         again when re-run on their own, more than the workers have delivered, \
                         so functions a worker leaves unfinished are not re-run while that lasts.",
                        self.tag
                    );
                }
                kept.push(indices[p]);
                kept_results.push(not_rerun(original, NOT_RERUN_RUN));
                continue;
            }
            let solo = self.serve(worker, std::slice::from_ref(&chunk[p]));
            if !solo.ending.started() {
                let cause = solo.results[0].error.clone().unwrap_or_default();
                let why = format!("its re-run could not start ({cause})");
                kept.push(indices[p]);
                kept_results.push(not_rerun(original, &why));
                stopped = Some(why);
                continue;
            }
            let recovered = solo.delivered[0];
            if recovered {
                self.recovered.fetch_add(1, Ordering::SeqCst);
            } else {
                self.failed_alone.fetch_add(1, Ordering::SeqCst);
                self.gate.failed();
            }
            let stalled = matches!(solo.ending, Ending::Died { stalled: true, .. });
            stopped = tally.record(suspect, recovered, stalled).map(str::to_string);
            deliver(&[indices[p]], solo.results);
        }
        if !kept.is_empty() {
            deliver(&kept, kept_results);
        }
    }

    /// Hand one chunk to this thread's worker, starting one first if the thread
    /// has none (its first chunk, or the one after a crash or a recycle).
    fn serve(&self, worker: &mut Option<Worker>, chunk: &[TargetSpec]) -> Served {
        // Names each chunk's spec and result files, so a re-run takes a fresh one.
        let idx = self.session.chunk_ids.fetch_add(1, Ordering::SeqCst);
        // A recording worker, or a forcing one holding an older table, takes the
        // latest table on the spot; any other worker of the wrong kind is
        // retired and replaced.
        if let Some(w) = worker.as_mut() {
            let table = self.session.latest_table();
            let usable = if self.synth == SynthWorker::Force {
                !w.stale_for(table) || w.force(table)
            } else {
                w.synth == self.synth
            };
            if !usable {
                if let Some(w) = worker.take() {
                    self.session.retire(self.cfg, w);
                }
            }
        }
        if let Err(e) = std::fs::write(self.scratch.join(spec_name(idx)), encode_spec(chunk)) {
            return Served::not_run(chunk, &format!("{SPEC_WRITE_FAILED}: {e}"));
        }
        if worker.is_none() {
            match self.spawn() {
                Ok(w) => *worker = Some(w),
                Err(e) => {
                    let _ = std::fs::remove_file(self.scratch.join(spec_name(idx)));
                    return Served::not_run(chunk, &format!("{SPAWN_FAILED}: {e}"));
                }
            }
        }
        let w = worker.as_mut().expect("just spawned");
        let (produced, reason, ending) = w.run_chunk(self.cfg, self.scratch, idx, chunk.len());
        if ending != Ending::Finished {
            *worker = None;
        }
        let (results, delivered) = merge_chunk(chunk, produced, &reason);
        self.gate.delivered(delivered.iter().filter(|&&d| d).count());
        Served { results, delivered, ending }
    }

    fn spawn(&self) -> std::io::Result<Worker> {
        let n = self.spawned.fetch_add(1, Ordering::SeqCst);
        if self.faults.refuses_spawn(n) {
            return Err(std::io::Error::other(format!("{JOBS_FAULT_ENV} refused spawn {n}")));
        }
        let mut w = Worker::spawn(self.cfg, self.synth, self.exe, self.scratch)?;
        if self.synth == SynthWorker::Force {
            w.table = self.session.latest_table();
        }
        Ok(w)
    }

    fn retries(&self) -> Retries {
        Retries {
            recovered: self.recovered.load(Ordering::SeqCst),
            failed_alone: self.failed_alone.load(Ordering::SeqCst),
        }
    }
}

/// What re-running a dead worker's targets came to over a run.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Retries {
    /// Re-runs that came back with the worker's own record for the function.
    pub(crate) recovered: usize,
    /// Re-runs whose worker failed again, leaving that re-run's `error` record.
    pub(crate) failed_alone: usize,
}

/// How a chunk handed to a worker ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    /// The worker acknowledged it.
    Finished,
    /// No worker ran it: its spec could not be written or no worker started.
    NotRun,
    /// The worker exited, or the stall watchdog killed it, first.  `started`:
    /// it had opened this chunk's result file, which it does after its load,
    /// its spec and its rehydration, just before its first target.  `warm`: it
    /// had finished a chunk before this one.
    Died { stalled: bool, started: bool, warm: bool },
}

impl Ending {
    /// Did a worker get as far as the chunk's first target?
    fn started(self) -> bool {
        matches!(self, Ending::Finished | Ending::Died { started: true, .. })
    }
}

/// One served chunk: a record per target in chunk order, and which of them a
/// worker produced rather than the pool filling the gap.
struct Served {
    results: Vec<FuncResult>,
    delivered: Vec<bool>,
    ending: Ending,
}

impl Served {
    fn not_run(chunk: &[TargetSpec], reason: &str) -> Self {
        Served {
            results: lost_chunk(chunk, reason),
            delivered: vec![false; chunk.len()],
            ending: Ending::NotRun,
        }
    }
}

/// Why the functions a chunk stops before are not re-run, appended to the
/// record their chunk left them so [`count_anomalies`] still classifies it.
const NOT_RERUN_STALLED: &str = "two functions re-run from its chunk stalled";
const NOT_RERUN_CRASHING: &str = "most functions re-run from its chunk failed again";
const NOT_RERUN_RUN: &str =
    "re-runs in this run had failed more often than workers had delivered when it came up";

fn not_rerun(mut r: FuncResult, why: &str) -> FuncResult {
    if let Some(error) = r.error.as_mut() {
        error.push_str("; not re-run: ");
        error.push_str(why);
    }
    r
}

/// The positions of a chunk to re-run alone: the function its worker died in,
/// then the ones that never started.
#[derive(Debug, Default, PartialEq, Eq)]
struct Reruns {
    suspect: Option<usize>,
    bystanders: Vec<usize>,
}

impl Reruns {
    fn is_empty(&self) -> bool {
        self.suspect.is_none() && self.bystanders.is_empty()
    }
}

/// What to re-run of a chunk whose worker ended as `ending`.
///
/// Only a worker that died leaves anything to re-run.  Its records are a prefix
/// of the chunk, so the first position without one is the function it was on
/// (the suspect) and every later one never started.  The suspect goes first,
/// onto the fresh worker the thread spawns next, so a crash it repeats is its
/// own; the bystanders follow in [`spread`] order.  A stalled suspect is not
/// re-run: it already ran past four times the per-function budget, and a second
/// attempt would cost the same stall window again.  A worker that never opened
/// this chunk ran none of it, so its targets are all bystanders if it had served
/// a chunk before — and none are re-run if it had not, because then it may have
/// died in its own load.
fn retry_order(ending: Ending, delivered: &[bool]) -> Reruns {
    let Ending::Died { stalled, started, warm } = ending else { return Reruns::default() };
    let missing: Vec<usize> = (0..delivered.len()).filter(|&p| !delivered[p]).collect();
    let (suspect, rest) = match (started, missing.split_first()) {
        (false, _) if !warm => return Reruns::default(),
        (false, _) => (None, &missing[..]),
        (true, None) => return Reruns::default(),
        (true, Some((&first, rest))) => ((!stalled).then_some(first), rest),
    };
    Reruns { suspect, bystanders: spread(rest.len()).into_iter().map(|k| rest[k]).collect() }
}

/// `0..count` in bit-reversed order, so every prefix of it is spread across the
/// whole range: a chunk's first re-runs sample all of it instead of one run of
/// size-sorted neighbours.  The pattern it serves worst is a crasher at every
/// other place, since its first half is the even offsets.
fn spread(count: usize) -> Vec<usize> {
    let span = count.next_power_of_two();
    let bits = span.trailing_zeros();
    (0..span)
        .map(|k| if bits == 0 { k } else { k.reverse_bits() >> (usize::BITS - bits) })
        .filter(|&k| k < count)
        .collect()
}

/// One chunk's re-runs so far.  Stalls count whoever stalled, since each cost a
/// window; failures and recoveries count bystanders only, since the function
/// that was running is expected to fail again.
#[derive(Default)]
struct ChunkReruns {
    failed: usize,
    recovered: usize,
    stalled: usize,
}

impl ChunkReruns {
    /// Record one re-run; why the chunk stops re-running, once it does.
    fn record(&mut self, suspect: bool, recovered: bool, stalled: bool) -> Option<&'static str> {
        self.stalled += usize::from(stalled);
        if !suspect {
            if recovered {
                self.recovered += 1;
            } else {
                self.failed += 1;
            }
        }
        if self.stalled >= CHUNK_RERUN_STALLS {
            return Some(NOT_RERUN_STALLED);
        }
        (self.failed >= CHUNK_RERUN_FAILURES && self.failed > self.recovered)
            .then_some(NOT_RERUN_CRASHING)
    }
}

/// The run-wide check on re-running, asked each time a re-run would start: no
/// while [`RUN_RERUN_FAILURES`] or more re-runs have failed and they outnumber
/// every record the workers delivered, planned or re-run.  Workers that die
/// whatever they are given deliver nothing, so they pay a bounded number of
/// extra loads; since held-back re-runs cannot fail, deliveries catch up and
/// re-running resumes in a run whose workers mostly work.
#[derive(Default)]
struct RetryGate {
    failed: AtomicUsize,
    delivered: AtomicUsize,
    refused: AtomicBool,
}

impl RetryGate {
    fn allows(&self) -> bool {
        let failed = self.failed.load(Ordering::SeqCst);
        failed < RUN_RERUN_FAILURES || failed <= self.delivered.load(Ordering::SeqCst)
    }

    fn delivered(&self, records: usize) {
        self.delivered.fetch_add(records, Ordering::SeqCst);
    }

    fn failed(&self) {
        self.failed.fetch_add(1, Ordering::SeqCst);
    }

    /// `true` only the first time a re-run is held back, so the run warns once.
    fn first_refusal(&self) -> bool {
        !self.refused.swap(true, Ordering::SeqCst)
    }
}

/// One live worker process, its assignment pipe and its acknowledgement channel.
///
/// A worker outlives the chunk it is working on, which is the whole point: on an
/// 18 MB PE a worker's load costs 17 s, against ~70 ms for the average function,
/// so paying it per chunk made a 12-way run **slower** than the serial one
/// (1,500 `mpengine.dll` functions: 113 s serial, 327 s across 34 single-load
/// chunks).  Loading once and taking chunk after chunk down the pipe pays it
/// `--jobs` times for the whole run, and leaves chunks free to be small enough
/// that no worker sits idle at the end.
struct Worker {
    child: std::process::Child,
    /// The assignment pipe, and by construction the liveness pipe: the parent
    /// holds the only write end for as long as this worker should live.
    stdin: std::process::ChildStdin,
    /// Chunk indices the worker has acknowledged.  Disconnects when the worker's
    /// stdout closes, which is how a crash is noticed.
    acks: std::sync::mpsc::Receiver<usize>,
    /// Functions handed to this process so far — what [`RECYCLE_AFTER`] bounds.
    functions_done: usize,
    /// Has this worker finished a chunk?  Until it has, the stall watchdog must
    /// allow for a whole program load on top of the per-function budget.
    warm: bool,
    /// What it does with the structure ledger now.
    synth: SynthWorker,
    /// Which of the session's tables it holds, when it forces.
    table: usize,
}

impl Worker {
    fn spawn(
        cfg: &PoolConfig,
        synth: SynthWorker,
        exe: &Path,
        scratch: &Path,
    ) -> std::io::Result<Self> {
        let mut cmd = Command::new(exe);
        cmd.arg("decompile-all")
            .arg(cfg.binary)
            .arg("--jobs-worker")
            .arg(scratch)
            .arg("--mode")
            .arg(cfg.mode)
            .arg("--max-fn-seconds")
            .arg(cfg.max_fn_seconds.to_string());
        if cfg.no_vars {
            cmd.arg("--no-vars");
        }
        if cfg.elem_objects {
            cmd.arg("--jobs-objects");
        }
        if cfg.want_proto {
            cmd.arg("--jobs-proto");
        }
        if cfg.want_provenance {
            cmd.arg("--jobs-provenance");
        }
        if cfg.want_types {
            cmd.arg("--jobs-types");
        }
        if cfg.want_callee_hints {
            cmd.arg("--jobs-callees");
        }
        if cfg.full_load {
            cmd.arg("--jobs-full-load");
        }
        for (flag, value) in [
            ("--isa", cfg.isa),
            ("--slice", cfg.slice),
            ("--target", cfg.target),
            ("--sleighpath", cfg.sleighpath),
        ] {
            if let Some(v) = value {
                cmd.arg(flag).arg(v);
            }
        }
        for decl in &cfg.func_decls {
            cmd.arg("--define-function").arg(decl);
        }
        // The parent's resolved options, verbatim — except the one decision a
        // worker must NOT inherit.  `fast_funcdisc` is the whole-binary function
        // discovery the parent has already run and is about to hand over; leaving
        // it on makes every worker redo it (31 s and 1.97 GB each on an 18 MB PE,
        // against 17 s and 469 MB without).  The parent's list is the merged one,
        // so its `--mode` preset spells this option out and a worker-side default
        // would read as an explicit choice and lose.
        for (name, value) in cfg.options {
            if !cfg.full_load && name == "fast_funcdisc" {
                continue;
            }
            cmd.arg("--option").arg(name).arg(value);
        }
        if !cfg.full_load {
            cmd.arg("--option").arg("fast_funcdisc").arg("off");
        }
        // See [`structsynth_shard_note`] and [`name_structs_serially`].
        match synth.flag() {
            Some(flag) => {
                cmd.arg("--jobs-synth").arg(flag);
            }
            None => {
                cmd.arg("--option").arg("structsynth").arg("off");
            }
        }
        // Same reasoning one flag further in: the parent's own load ran the
        // discovery walk on N decode lanes and is handing the inventory over, so
        // a worker must not run N more of them. Forced rather than merely left
        // unset, because the variable can also reach a worker from the user's
        // environment or through `--jobs-full-load`.
        cmd.env(kuna_analysis::listing::kuna_pdecode::DECODE_JOBS_ENV, "1");
        // stdin carries the assignments AND the liveness signal; stdout carries
        // the acknowledgements; stderr is the user's, shared with the parent.
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());

        let mut child = cmd.spawn()?;
        let stdin = child.stdin.take().expect("stdin was piped");
        let stdout = child.stdout.take().expect("stdout was piped");
        let (tx, acks) = std::sync::mpsc::channel();
        // A line the engine itself printed is not an acknowledgement: only the
        // exact `done <n>` shape counts, so nothing on stdout can fake progress.
        std::thread::spawn(move || {
            for line in std::io::BufReader::new(stdout).lines().map_while(Result::ok) {
                if let Some(rest) = line.strip_prefix(ACK_PREFIX) {
                    if let Ok(idx) = rest.trim().parse::<usize>() {
                        if tx.send(idx).is_err() {
                            return;
                        }
                    }
                }
            }
        });
        Ok(Self { child, stdin, acks, functions_done: 0, warm: false, synth, table: 0 })
    }

    /// Does this worker lack table `table` for a forced chunk?
    fn stale_for(&self, table: usize) -> bool {
        self.synth == SynthWorker::Record || (self.synth == SynthWorker::Force && self.table != table)
    }

    /// Make this worker a forcing one holding table `table`: it forgets the
    /// structures it minted or installed and installs the one in the scratch
    /// directory before its next chunk. `false` when the pipe is gone, which
    /// leaves the worker for the caller to replace.
    fn force(&mut self, table: usize) -> bool {
        let sent = writeln!(self.stdin, "{SYNTH_LINE}").and_then(|()| self.stdin.flush()).is_ok();
        if sent {
            self.synth = SynthWorker::Force;
            self.table = table;
        }
        sent
    }

    /// Decompile chunk `idx`, returning what the worker delivered, the reason any
    /// target of it is missing, and how the chunk ended — anything but
    /// [`Ending::Finished`] leaves the worker unusable.
    fn run_chunk(
        &mut self,
        cfg: &PoolConfig,
        scratch: &Path,
        idx: usize,
        len: usize,
    ) -> (Vec<FuncResult>, String, Ending) {
        self.functions_done += len;
        let warm = self.warm;
        let out_path = scratch.join(result_name(idx));
        let assigned = writeln!(self.stdin, "{idx}").and_then(|()| self.stdin.flush());
        let outcome = match assigned {
            Ok(()) => self.await_chunk(cfg, idx, &out_path),
            Err(e) => {
                let _ = self.child.kill();
                let _ = self.child.wait();
                Wait::Failed(format!("cannot assign the chunk: {e}"))
            }
        };
        // Read only once the worker is acknowledged or gone, so whether the file
        // exists is final: the worker creates it just before its first target.
        let file = std::fs::read(&out_path);
        let started = file.is_ok();
        let produced = file.ok().and_then(|b| decode_results(&b)).unwrap_or_default();
        let _ = std::fs::remove_file(scratch.join(spec_name(idx)));
        let _ = std::fs::remove_file(&out_path);
        match outcome {
            Wait::Done => {
                self.warm = true;
                (produced, format!("{NO_RECORD} for this function"), Ending::Finished)
            }
            Wait::Stalled => (
                produced,
                format!("{STALLED} ({}s); the worker was killed", cfg.max_fn_seconds),
                Ending::Died { stalled: true, started, warm },
            ),
            Wait::Failed(why) => (
                produced,
                format!("{CHUNK_FAILED} ({why})"),
                Ending::Died { stalled: false, started, warm },
            ),
        }
    }

    /// Wait for this chunk's acknowledgement, killing the worker if it stops
    /// producing records for longer than [`stall_deadline`] allows.
    ///
    /// The in-process watchdog is **cooperative**: the decompile drive polls a
    /// deadline, so a function wedged where nothing polls runs straight through
    /// it — which is how one function can hold a whole export open.  A pool is
    /// the first arrangement that can enforce the budget for real, because the
    /// enforcer is not the process that is stuck.  Progress is measured by the
    /// result file growing, since a worker flushes one frame per finished
    /// function.
    fn await_chunk(&mut self, cfg: &PoolConfig, idx: usize, out_path: &Path) -> Wait {
        let mut last_progress = Instant::now();
        let mut last_size = 0u64;
        let mut saw_record = false;
        loop {
            match self.acks.recv_timeout(Duration::from_millis(250)) {
                Ok(acked) if acked == idx => return Wait::Done,
                // An acknowledgement for another chunk cannot happen (a worker
                // serves one at a time) but is not a reason to give up on this one.
                Ok(_) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    let status = self.child.wait();
                    return Wait::Failed(match status {
                        Ok(s) => format!("worker exited: {s}"),
                        Err(e) => format!("cannot wait for the worker: {e}"),
                    });
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            }
            let size = std::fs::metadata(out_path).map(|m| m.len()).unwrap_or(0);
            if size > last_size {
                last_size = size;
                saw_record = true;
                last_progress = Instant::now();
            }
            let Some(allowed) = stall_deadline(cfg, self.warm || saw_record) else { continue };
            if last_progress.elapsed() > allowed {
                let _ = self.child.kill();
                let _ = self.child.wait();
                return Wait::Stalled;
            }
        }
    }

    /// Retire a worker that is between chunks: `quit <token>` on the assignment
    /// pipe, so it exits without mistaking the closed pipe for a dead parent and
    /// sweeping the scratch directory its siblings are still using.
    ///
    /// `quit` is also when a `--jobs-types` worker renders its type block, and
    /// the token names the file it leaves it in.  Per WORKER, not per chunk: a
    /// worker's type factory accumulates over every chunk it served, so its final
    /// rendering is the only one that speaks for all of them.
    fn quit(self, cfg: &PoolConfig, scratch: &Path, token: usize) -> Option<(String, Option<String>)> {
        let Self { mut child, mut stdin, .. } = self;
        let _ = writeln!(stdin, "{QUIT_PREFIX}{token}");
        let _ = stdin.flush();
        drop(stdin);
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match child.try_wait() {
                Ok(Some(_)) | Err(_) => break,
                Ok(None) if Instant::now() >= deadline => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
        if !cfg.want_types {
            return None;
        }
        let read = |rest: bool| {
            let path = scratch.join(types_name(token, rest));
            let block = std::fs::read_to_string(&path).ok();
            let _ = std::fs::remove_file(&path);
            block
        };
        let rest = read(true);
        Some((read(false)?, rest))
    }
}

enum Wait {
    Done,
    Stalled,
    Failed(String),
}

/// How long a worker may produce nothing before the parent kills it, or `None`
/// when `--max-fn-seconds 0` has disabled the budget and therefore this too.
///
/// Deliberately loose: `4x` the per-function budget, and before the worker's
/// first record a whole program load on top of it, because it is competing with
/// `jobs-1` siblings for the machine and the parent's own load time is the only
/// measurement of that cost anyone has.  The point is to bound a function wedged
/// where nothing probes the cooperative deadline, not to second-guess a slow one.
fn stall_deadline(cfg: &PoolConfig, warm: bool) -> Option<Duration> {
    if cfg.max_fn_seconds == 0 {
        return None;
    }
    let budget = Duration::from_secs_f64(4.0 * cfg.max_fn_seconds as f64);
    if warm {
        return Some(budget);
    }
    Some(budget + Duration::from_secs_f64(60.0 + 3.0 * cfg.load_seconds))
}

/// Why a pool gives its workers `structsynth off`, or `None` when the run had
/// turned it off anyway: the `--stream` pool, whose serial run writes each body
/// as it lands and runs no convergence sweep for [`name_structs_serially`] to
/// reproduce.
///
/// A synthesized `struct_N` is named by the process that minted it, so two
/// workers can each define a different `struct_0`: one document would use a name
/// for two layouts, and one `.h` cannot declare both.
fn structsynth_shard_note(cfg: &PoolConfig, tag: &str) -> Option<String> {
    let explicit = cfg.options.iter().rev().find(|(name, _)| name == "structsynth");
    if explicit.is_some_and(|(_, value)| value == "off") {
        return None;
    }
    Some(format!(
        "[kuna {tag}] note: structsynth is off in the worker processes: each process would \
         number its own struct_N, so one name could stand for two layouts. Re-run with \
         --jobs 1 for synthesized structures."
    ))
}

/// Every `error` record the POOL itself produced, as opposed to one the engine
/// produced for a function it genuinely could not decompile.  Kept as constants
/// because [`count_anomalies`] classifies by them.
const NO_RECORD: &str = "worker produced no record";
const CHUNK_FAILED: &str = "worker chunk failed";
const SPAWN_FAILED: &str = "cannot spawn worker";
const SPEC_WRITE_FAILED: &str = "chunk spec write failed";
const STALLED: &str = "worker stalled past the per-function watchdog";

/// `(watchdog trips, functions lost to a failed worker)`.
fn count_anomalies(results: &[FuncResult]) -> (usize, usize) {
    count_error_anomalies(results.iter().filter_map(|r| r.error.as_deref()))
}

fn count_error_anomalies<'a>(errors: impl Iterator<Item = &'a str>) -> (usize, usize) {
    let mut tripped = 0;
    let mut lost = 0;
    for e in errors {
        if e.contains("budget exceeded") {
            tripped += 1;
        }
        if [NO_RECORD, CHUNK_FAILED, SPAWN_FAILED, SPEC_WRITE_FAILED, STALLED]
            .iter()
            .any(|m| e.starts_with(m))
        {
            lost += 1;
        }
    }
    (tripped, lost)
}

/// The two ways `--jobs N` can end up disagreeing with `--jobs 1`, both of which
/// would otherwise be silent.
///
/// The per-function watchdog is a **wall-clock** deadline, so a function that
/// finished just inside it serially can miss it under N-way CPU contention and
/// degrade to an `error`.  The budget itself is resolved once by the parent and
/// passed to every worker, so the policy is identical — only the wall clock
/// moves.
///
/// A worker can also simply die (OOM killer, SIGSEGV, an operator's `kill`, or
/// the parent's own stall kill).  The functions of its chunk it had not flushed
/// are re-run on their own ([`retry_order`]), and whatever still has no result
/// becomes an `error` record.  The run legitimately continues — one bad function
/// must not kill a 33,000-function export — but exiting 0 without a word about
/// it is not reporting, and neither is hiding that a worker died when every
/// function it left behind was recovered.
fn warn_about_anomalies(results: &[FuncResult], max_fn_seconds: u64, retries: Retries) {
    for line in anomaly_lines(count_anomalies(results), max_fn_seconds, JOBS_TAG, retries) {
        eprintln!("{line}");
    }
}

/// The same warnings for a run whose results were consumed as they landed
/// (`--stream`), which keeps only the error strings.
pub(crate) fn warn_about_streamed_anomalies(
    errors: &[String],
    max_fn_seconds: u64,
    retries: Retries,
) {
    let counts = count_error_anomalies(errors.iter().map(String::as_str));
    for line in anomaly_lines(counts, max_fn_seconds, STREAM_TAG, retries) {
        eprintln!("{line}");
    }
}

fn anomaly_lines(
    (tripped, lost): (usize, usize),
    max_fn_seconds: u64,
    tag: &str,
    retries: Retries,
) -> Vec<String> {
    let mut lines = Vec::new();
    if tripped > 0 && max_fn_seconds > 0 {
        lines.push(format!(
            "[kuna {tag}] warning: {tripped} function(s) hit the {max_fn_seconds}s per-function \
             watchdog. It is wall-clock, so heavy functions can trip it under parallel load that \
             would pass serially — re-run with a larger --max-fn-seconds (or 0) if you need them."
        ));
    }
    if retries.recovered > 0 {
        lines.push(format!(
            "[kuna {tag}] {} function(s) left unfinished by a failed worker process were re-run \
             one at a time and recovered.",
            retries.recovered
        ));
    }
    if lost > 0 {
        let again = match retries.failed_alone {
            0 => String::new(),
            n => format!(" {n} of them failed again when re-run on their own."),
        };
        lines.push(format!(
            "[kuna {tag}] warning: {lost} function(s) have no result because their worker process \
             failed (crash, OOM kill, an external signal, or the stall watchdog); they are `error` \
             records in the output.{again} Re-run those functions, with fewer --jobs if the \
             machine ran out of memory."
        ));
    }
    lines
}

// --- how many workers, and what each one gets --------------------------------

/// Trim `--jobs auto` to what this machine's free memory can hold, and say so
/// when an explicit `--jobs N` asks for more than that.
///
/// `auto` is a promise not to wreck the machine, so it yields; an explicit
/// number is an instruction, so it is obeyed with a warning.
fn affordable_jobs(cfg: &PoolConfig, tag: &str) -> usize {
    let (Some(per_worker), Some(available)) =
        (worker_estimate(cfg.full_load), available_memory_bytes())
    else {
        return cfg.jobs;
    };
    // Two thirds of what is free: the parent keeps its own load on several of
    // these surfaces, and a worker's peak is an estimate, not a bound.
    let affordable = (available.saturating_mul(2) / 3 / per_worker.max(1)).max(1) as usize;
    if cfg.jobs <= affordable {
        return cfg.jobs;
    }
    let gb = |b: u64| b as f64 / (1024.0 * 1024.0 * 1024.0);
    if cfg.jobs_auto {
        eprintln!(
            "[kuna {tag}] auto: {affordable} worker(s), not {} — one worker needs about {:.1} GB \
             here and {:.1} GB is free. Pass --jobs N to override.",
            cfg.jobs,
            gb(per_worker),
            gb(available)
        );
        return affordable;
    }
    eprintln!(
        "[kuna {tag}] warning: {} workers at about {:.1} GB each is more than the {:.1} GB free \
         on this machine; expect swapping or an OOM kill. {affordable} would fit.",
        cfg.jobs,
        gb(per_worker),
        gb(available)
    );
    cfg.jobs
}

/// What one worker will cost, from the only measurement the parent has: its own
/// peak resident size, which is only knowable after the load.
///
/// A `--jobs-full-load` worker runs the parent's exact load, so that peak IS the
/// estimate.  A default worker skips the whole-binary discovery the parent ran
/// and takes its inventory instead, which is most of a load's memory — 469 MB
/// against the parent's 1.99 GB on an 18 MB PE with 33,214 functions.  A quarter
/// rounds that ratio against us, with a floor for small programs where the
/// constant costs dominate and the ratio means nothing.
fn worker_estimate(full_load: bool) -> Option<u64> {
    Some(worker_estimate_from(
        peak_rss_bytes()?,
        kuna_analysis::listing::kuna_pdecode::lane_peak_excess_bytes(),
        full_load,
    ))
}

/// The estimate's arithmetic. `lane_excess` is what the parent's own decode
/// lanes added to its peak (`--jobs N` runs them during the load) — a cost no
/// worker pays, because every worker is forced back to one lane, so leaving it
/// in prices a worker ~1.5x too high and shrinks the pool that does 96% of the
/// work.
fn worker_estimate_from(parent_peak: u64, lane_excess: u64, full_load: bool) -> u64 {
    const FLOOR: u64 = 256 * 1024 * 1024;
    let parent = parent_peak.saturating_sub(lane_excess);
    if full_load {
        parent
    } else {
        (parent / 4).max(FLOOR)
    }
}

/// This process's peak resident size (Linux `VmHWM`).
fn peak_rss_bytes() -> Option<u64> {
    proc_kb("/proc/self/status", "VmHWM:")
}

/// Memory that can be handed out without swapping (Linux `MemAvailable`).
fn available_memory_bytes() -> Option<u64> {
    proc_kb("/proc/meminfo", "MemAvailable:")
}

fn proc_kb(path: &str, key: &str) -> Option<u64> {
    let text = std::fs::read_to_string(path).ok()?;
    let line = text.lines().find(|l| l.starts_with(key))?;
    Some(line.split_whitespace().nth(1)?.parse::<u64>().ok()? * 1024)
}

/// Plan the chunks: which targets each worker invocation gets, in dispatch
/// order. Each entry is a list of **slot indices**, so output order travels
/// independently of the order work is done in.
///
/// A chunk is a unit of *scheduling*, not of process lifetime — a worker takes
/// the next one the moment it finishes the last — so the packing only has to
/// balance the end of the run.  Chunks are packed to roughly equal estimated
/// work over a longest-first order, aiming at [`CHUNKS_PER_JOB`] per worker: the
/// expensive functions go out first and mostly alone, the long cheap tail
/// batches, and a worker that draws a slow chunk simply draws fewer of them.
/// An explicit `--jobs-chunk` overrides the packing with a fixed size, keeping
/// the ordering.  Depends only on the targets and `jobs`, so the plan is
/// deterministic.
fn plan_chunks(targets: &[TargetSpec], explicit: Option<usize>, jobs: usize) -> Vec<Vec<usize>> {
    let total = targets.len();
    if total == 0 {
        return Vec::new();
    }
    let cost: Vec<u64> = targets.iter().map(|t| t.size.clamp(1, EXTENT_CAP)).collect();
    let mut order: Vec<usize> = (0..total).collect();
    order.sort_by(|&a, &b| cost[b].cmp(&cost[a]).then(a.cmp(&b)));

    if let Some(size) = explicit {
        return order.chunks(size.max(1)).map(<[usize]>::to_vec).collect();
    }

    let quantum = (cost.iter().sum::<u64>() / (jobs.max(1) * CHUNKS_PER_JOB) as u64).max(1);
    let mut plan: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut work: u64 = 0;
    for &i in &order {
        current.push(i);
        work = work.saturating_add(cost[i]);
        // No floor on the size: a function estimated to be a long pole is worth a
        // chunk to itself, while cheap functions never reach the quantum alone
        // and batch up anyway.
        if work >= quantum || current.len() >= MAX_AUTO_CHUNK {
            plan.push(std::mem::take(&mut current));
            work = 0;
        }
    }
    if !current.is_empty() {
        plan.push(current);
    }
    plan
}

/// Per-target cost estimate cap, in bytes.  The inventory's own extent is the
/// estimate; the cap keeps one enormous (or unmeasured, hence `0` → `1`) entry
/// from swallowing a chunk's whole budget.
const EXTENT_CAP: u64 = 0x8000;

/// Line the worker's records back up with the chunk it was given, one result per
/// target, and say which of them the worker produced.  A worker that died hard
/// (SIGSEGV/OOM) delivers only the prefix it flushed, so the rest degrade to
/// `error` records — which [`retry_order`] then decides whether to re-run — and
/// the run continues: one bad function must not kill a 33,000-function export.
fn merge_chunk(
    chunk: &[TargetSpec],
    produced: Vec<FuncResult>,
    reason: &str,
) -> (Vec<FuncResult>, Vec<bool>) {
    let mut by_addr: std::collections::HashMap<u64, FuncResult> =
        produced.into_iter().map(|r| (r.byte_address, r)).collect();
    chunk
        .iter()
        .map(|t| match by_addr.remove(&t.addr) {
            Some(r) => (r, true),
            None => (lost_result(t, reason), false),
        })
        .unzip()
}

/// The `error` record a target gets when nothing produced one for it — the
/// `--stream` reconciliation's filler, classified as a lost function by
/// [`warn_about_streamed_anomalies`] exactly as the pool's own gap is.
pub(crate) fn missing_result(t: &TargetSpec) -> FuncResult {
    lost_result(t, NO_RECORD)
}

fn lost_chunk(chunk: &[TargetSpec], reason: &str) -> Vec<FuncResult> {
    chunk.iter().map(|t| lost_result(t, reason)).collect()
}

fn lost_result(t: &TargetSpec, reason: &str) -> FuncResult {
    FuncResult {
        name: t.name.clone(),
        address: t.addr,
        byte_address: t.addr,
        size: t.size as i64,
        code: None,
        error: Some(reason.to_string()),
        proto: None,
        variables: Vec::new(),
        types: Vec::new(),
        globals: Vec::new(),
        line_mappings: Vec::new(),
        aliases: t.aliases.clone(),
        object_location: t.object_location.clone(),
        callee_hints: Vec::new(),
        synth: None,
        detail: None,
    }
}

// --- the scratch directory ----------------------------------------------------

/// The pool's temp directory, owned by its `Drop`.  Cleanup cannot live on
/// [`run_pool`]'s happy path: the early `Err` returns and a panicking pool thread
/// leave by other doors, and the directory carries every worker's decompiled C
/// plus the whole-program symbol inventory.
struct ScratchDir {
    path: PathBuf,
}

impl ScratchDir {
    fn create() -> Result<Self, String> {
        let temp = std::env::temp_dir();
        sweep_stale_scratch(&temp);
        let path = temp.join(format!(
            "{SCRATCH_PREFIX}{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("cannot create the worker scratch dir {}: {e}", path.display()))?;
        let dir = Self { path };
        // 0700, not the ambient umask: the whole program's symbol inventory and
        // every function's decompiled C transit this directory.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dir.path, std::fs::Permissions::from_mode(0o700)).map_err(
                |e| format!("cannot restrict the worker scratch dir {}: {e}", dir.path.display()),
            )?;
        }
        Ok(dir)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Sweep scratch directories left behind by a run whose parent died in the one
/// window nothing else covers — killed after creating the directory but before a
/// worker existed to notice.  A directory is only removed once its owning pid is
/// provably gone; on Linux `/proc` answers that exactly, and elsewhere the only
/// std-visible evidence is age, so the fallback waits a week rather than risk
/// deleting a live multi-hour run's directory.
fn sweep_stale_scratch(temp: &Path) {
    let Ok(entries) = std::fs::read_dir(temp) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(pid) = name.to_str().and_then(scratch_owner) else { continue };
        if !owner_is_gone(pid, &entry.path()) {
            continue;
        }
        let _ = std::fs::remove_dir_all(entry.path());
    }
}

/// The pid encoded in a scratch directory name (`kuna-jobs-<pid>-<nanos>`).
fn scratch_owner(name: &str) -> Option<u32> {
    name.strip_prefix(SCRATCH_PREFIX)?.split('-').next()?.parse().ok()
}

fn owner_is_gone(pid: u32, path: &Path) -> bool {
    if Path::new("/proc/self/stat").exists() {
        return !Path::new(&format!("/proc/{pid}")).exists();
    }
    const WEEK: Duration = Duration::from_secs(7 * 24 * 3600);
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .and_then(|t| t.elapsed().map_err(|_| std::io::ErrorKind::Other.into()))
        .is_ok_and(|age| age > WEEK)
}

// --- worker-side assignments and parent liveness ------------------------------

/// The scratch directory a worker was pointed at, but only when it is
/// recognizably one of ours: a worker must never delete a directory the pool did
/// not create.
fn pool_scratch(dir: &str) -> Option<PathBuf> {
    let path = Path::new(dir);
    path.file_name()?.to_str()?.starts_with(SCRATCH_PREFIX).then(|| path.to_path_buf())
}

/// The chunk a worker should decompile next, and — by the same pipe — whether it
/// still has a parent to decompile it for.
///
/// A worker's stdin is a pipe whose only write end is a
/// [`std::process::ChildStdin`] the parent holds for the worker's lifetime.  The
/// parent writes one chunk index per line down it and finishes with
/// [`QUIT_PREFIX`]; the returned channel closes when either that line or the end
/// of the pipe arrives, which is what ends the worker's loop.
///
/// The distinction between the two matters.  `quit` is the parent saying it is
/// done with this worker while its siblings run on, so the worker exits and
/// touches nothing.  End-of-pipe without it means every write end is closed,
/// which happens when the parent goes away *however* it goes away — normally,
/// panicking, SIGINT, SIGTERM or SIGKILL, the last of which no in-process
/// handler could ever cover.  Then there is no one left to consume any result,
/// so the worker removes the pool's scratch directory (with the whole program's
/// symbol inventory and every function's C in it) and exits, instead of running
/// to completion reparented to init.
pub(crate) fn listen_for_assignments(scratch: &str) -> std::sync::mpsc::Receiver<Assignment> {
    let dir = pool_scratch(scratch);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in std::io::BufReader::new(std::io::stdin()).lines().map_while(Result::ok) {
            let line = line.trim();
            if line == SYNTH_LINE {
                if tx.send(Assignment::Force).is_err() {
                    return;
                }
                continue;
            }
            let sent = match line.strip_prefix(QUIT_PREFIX) {
                Some(token) => match token.parse::<usize>() {
                    Ok(token) => {
                        let _ = tx.send(Assignment::Quit(token));
                        return;
                    }
                    Err(_) => return,
                },
                None => match line.parse::<usize>() {
                    Ok(idx) => tx.send(Assignment::Chunk(idx)).is_ok(),
                    Err(_) => continue,
                },
            };
            if !sent {
                return;
            }
        }
        if let Some(dir) = dir {
            let _ = std::fs::remove_dir_all(dir);
        }
        std::process::exit(EXIT_PARENT_GONE);
    });
    rx
}

/// What the parent just told this worker to do.
pub(crate) enum Assignment {
    /// Decompile the targets in this chunk's spec file.
    Chunk(usize),
    /// Stop.  The token names the file to leave a `--jobs-types` block in.
    Quit(usize),
    /// Forget the structures this worker minted, install the replayed table and
    /// answer every lookup from the chunk specs from now on.
    Force,
}

/// A worker's chunk-spec / result / type-block paths inside the pool scratch
/// directory.
pub(crate) fn spec_name(idx: usize) -> String {
    format!("chunk{idx}.spec")
}

pub(crate) fn result_name(idx: usize) -> String {
    format!("chunk{idx}.res")
}

fn types_name(token: usize, rest: bool) -> String {
    format!("types{token}{}.blk", if rest { ".rest" } else { "" })
}

/// Leave this worker's rendered user-defined type definitions where the parent's
/// `quit` said to (see [`merge_type_definitions`]); `rest` is the rendering
/// without the synthesized structures ([`Session::close`]).
pub(crate) fn write_type_block(scratch: &str, token: usize, rest: bool, text: &str) -> Result<(), String> {
    let path = Path::new(scratch).join(types_name(token, rest));
    std::fs::write(&path, text)
        .map_err(|e| format!("cannot write the worker type block {}: {e}", path.display()))
}

/// The inventory hand-off file, written once by the parent and read by every
/// worker that did not run its own discovery.
pub(crate) const INVENTORY_FILE: &str = "inventory.spec";

/// The replayed synthesized structures a [`SynthWorker::Force`] worker installs.
pub(crate) const SYNTH_TABLE_FILE: &str = "synth.table";

/// Read the table [`name_structs_serially`] left for a forced worker.
pub(crate) fn read_synth_table(scratch: &str) -> Result<Vec<(String, SynthRequest)>, String> {
    let path = Path::new(scratch).join(SYNTH_TABLE_FILE);
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("cannot read the synthesized structures {}: {e}", path.display()))?;
    shard::decode_table(&bytes)
        .ok_or_else(|| format!("malformed synthesized-structure table {}", path.display()))
}

/// Tell the parent chunk `idx` is written and closed.  The only line on a
/// worker's stdout the parent acts on.
pub(crate) fn ack_chunk(idx: usize) {
    let mut out = std::io::stdout();
    let _ = writeln!(out, "{ACK_PREFIX}{idx}");
    let _ = out.flush();
}

// --- test-only fault injection -------------------------------------------------

/// The failures [`JOBS_FAULT_ENV`] asks for.  Empty in every real run, where
/// each check below is a loop over nothing.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Faults {
    at: Vec<(Fault, Option<u64>)>,
    spawn_after: Option<usize>,
    /// `synth:serial`: name the synthesized structures by the one-worker serial
    /// path even when the replay holds, so a test can reach it.
    synth_serial: bool,
    /// `synth:force`: rename no function, so every one that asked the ledger
    /// is decompiled again with the replayed names.
    synth_force: bool,
    /// `synth:noinstall`: a worker refuses the replayed structures, which is
    /// what a field type it cannot rebuild would do to it.
    synth_noinstall: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fault {
    Panic,
    PanicOnce,
    Stall,
}

impl Faults {
    pub(crate) fn from_env() -> Faults {
        std::env::var(JOBS_FAULT_ENV).map(|raw| Faults::parse(&raw).0).unwrap_or_default()
    }

    /// The parent's copy, which also says which directives it could not read,
    /// once, so a mistyped fault cannot pass for a test that injected nothing.
    fn from_env_reporting(tag: &str) -> Faults {
        let Ok(raw) = std::env::var(JOBS_FAULT_ENV) else { return Faults::default() };
        let (faults, rejected) = Faults::parse(&raw);
        for directive in rejected {
            eprintln!(
                "[kuna {tag}] warning: ignoring unreadable {JOBS_FAULT_ENV} directive {directive:?}"
            );
        }
        faults
    }

    fn parse(raw: &str) -> (Faults, Vec<String>) {
        let mut faults = Faults::default();
        let mut rejected = Vec::new();
        for directive in raw.split(',').map(str::trim).filter(|d| !d.is_empty()) {
            let parsed = directive.split_once(':').and_then(|(kind, arg)| {
                let arg = arg.trim();
                let fault = match kind.trim() {
                    "spawn" => {
                        faults.spawn_after = Some(arg.parse().ok()?);
                        return Some(());
                    }
                    "synth" if arg == "serial" => {
                        faults.synth_serial = true;
                        return Some(());
                    }
                    "synth" if arg == "force" => {
                        faults.synth_force = true;
                        return Some(());
                    }
                    "synth" if arg == "noinstall" => {
                        faults.synth_noinstall = true;
                        return Some(());
                    }
                    "panic" => Fault::Panic,
                    "panic-once" => Fault::PanicOnce,
                    "stall" => Fault::Stall,
                    _ => return None,
                };
                let addr = match arg.strip_prefix("0x").or_else(|| arg.strip_prefix("0X")) {
                    _ if arg == "*" => None,
                    Some(hex) => Some(u64::from_str_radix(hex, 16).ok()?),
                    None => Some(arg.parse().ok()?),
                };
                faults.at.push((fault, addr));
                Some(())
            });
            if parsed.is_none() {
                rejected.push(directive.to_string());
            }
        }
        (faults, rejected)
    }

    /// Worker side, as the target at `addr` starts.  `panic-once` remembers
    /// firing with a marker file in the pool's scratch directory, which every
    /// worker of the run shares.
    pub(crate) fn before_target(&self, scratch: &Path, addr: u64) {
        for &(fault, at) in &self.at {
            if at.is_some_and(|at| at != addr) {
                continue;
            }
            match fault {
                Fault::Panic => panic!("{JOBS_FAULT_ENV}: injected panic at {addr:#x}"),
                Fault::PanicOnce => {
                    let marker = scratch.join(format!("fault-once-{addr:x}"));
                    if std::fs::OpenOptions::new().write(true).create_new(true).open(marker).is_ok()
                    {
                        panic!("{JOBS_FAULT_ENV}: injected panic at {addr:#x}");
                    }
                }
                Fault::Stall => {
                    eprintln!("{JOBS_FAULT_ENV}: injected stall at {addr:#x}");
                    loop {
                        std::thread::sleep(Duration::from_secs(3600));
                    }
                }
            }
        }
    }

    /// Worker side: does this worker refuse the replayed structures?
    pub(crate) fn refuses_synth_install(&self) -> bool {
        self.synth_noinstall
    }

    /// Parent side: does spawn number `n` (from 0) fail?
    fn refuses_spawn(&self, n: usize) -> bool {
        self.spawn_after.is_some_and(|after| n >= after)
    }
}

// --- progress ----------------------------------------------------------------

/// Throttled `functions done / total, elapsed, ETA` on **stderr** (stdout stays
/// byte-clean for `--json`).  A 33,000-function export runs for many minutes and
/// otherwise prints nothing at all until it finishes.
struct Progress {
    state: Mutex<ProgressState>,
    total: usize,
    tag: &'static str,
}

struct ProgressState {
    last: Instant,
    warmed: Vec<bool>,
    warm_count: usize,
    observed_done: usize,
    baseline: Option<(usize, Instant)>,
}

impl ProgressState {
    fn new(workers: usize, now: Instant) -> Self {
        Self {
            last: now,
            warmed: vec![false; workers],
            warm_count: 0,
            observed_done: 0,
            baseline: None,
        }
    }

    fn observe(
        &mut self,
        worker_id: usize,
        done: usize,
        now: Instant,
    ) -> (usize, Option<(usize, f64)>) {
        self.observed_done = self.observed_done.max(done);
        if !self.warmed[worker_id] {
            self.warmed[worker_id] = true;
            self.warm_count += 1;
            if self.warm_count == self.warmed.len() {
                self.baseline = Some((self.observed_done, now));
            }
            return (self.observed_done, None);
        }
        let sample = self.baseline.and_then(|(baseline_done, baseline_at)| {
            self.observed_done
                .checked_sub(baseline_done)
                .map(|n| (n, now.duration_since(baseline_at).as_secs_f64()))
        });
        (self.observed_done, sample)
    }
}

impl Progress {
    fn new(total: usize, workers: usize, tag: &'static str) -> Self {
        Self {
            state: Mutex::new(ProgressState::new(workers, Instant::now())),
            total,
            tag,
        }
    }

    fn report(&self, worker_id: usize, done: usize, start: Instant) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let (done, sample) = state.observe(worker_id, done, now);
        if now.duration_since(state.last) < Duration::from_secs(2) && done < self.total {
            return;
        }
        state.last = now;
        let elapsed = start.elapsed().as_secs_f64();
        let frac = done as f64 / self.total.max(1) as f64;
        let eta = sample
            .and_then(|(sample_done, sample_seconds)| {
                progress_eta(self.total, done, sample_done, sample_seconds)
            })
            .map(|seconds| format!(", ETA {}", hms(seconds)))
            .unwrap_or_default();
        eprintln!(
            "[kuna {}] {done}/{} functions ({:.1}%), {} elapsed{eta}",
            self.tag,
            self.total,
            frac * 100.0,
            hms(elapsed)
        );
    }

    /// The closing line.  `count` is the run's own answer to "how many": every
    /// target for a plan that was served in full, and the results that actually
    /// came back for a run that stopped early.
    fn finish(&self, count: usize, start: Instant) {
        eprintln!(
            "[kuna {}] done: {count} functions in {}",
            self.tag,
            hms(start.elapsed().as_secs_f64())
        );
    }
}

fn progress_eta(
    total: usize,
    done: usize,
    sample_done: usize,
    sample_seconds: f64,
) -> Option<f64> {
    if sample_done == 0 || !sample_seconds.is_finite() || sample_seconds <= 0.0 {
        return None;
    }
    Some(total.saturating_sub(done) as f64 * sample_seconds / sample_done as f64)
}

fn hms(seconds: f64) -> String {
    let s = seconds.max(0.0) as u64;
    if s >= 3600 {
        format!("{}h{:02}m{:02}s", s / 3600, (s % 3600) / 60, s % 60)
    } else if s >= 60 {
        format!("{}m{:02}s", s / 60, s % 60)
    } else {
        format!("{s}s")
    }
}

#[cfg(test)]
#[path = "jobs/replay_tests.rs"]
mod replay_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use super::wire::{decode_spec, RESULT_MAGIC};
    use kuna_decomp::decompile_drive::{GlobalInfo, LineMapping, TypeInfo, VarInfo};

    fn sample_result() -> FuncResult {
        FuncResult {
            name: "main".into(),
            address: 0x401000,
            byte_address: 0x401000,
            size: 42,
            // Deliberately hostile to a text protocol: quotes, backslashes,
            // newlines, a control char and non-ASCII.
            code: Some("void main(void)\n{\n  \"q\\\\\" /* \u{1}é */\n}".into()),
            error: None,
            proto: Some("void main(void);".into()),
            variables: vec![
                VarInfo {
                    name: "param_1".into(),
                    type_name: "char *".into(),
                    stack_offset: None,
                    size: 8,
                    is_param: true,
                    arg_index: Some(0),
                    line_numbers: vec![3, 4],
                    addresses: vec![0x401004, 0x401008],
                },
                VarInfo {
                    name: "local_18".into(),
                    type_name: "undefined8".into(),
                    stack_offset: Some(-0x18),
                    size: 8,
                    is_param: false,
                    arg_index: None,
                    line_numbers: Vec::new(),
                    addresses: Vec::new(),
                },
            ],
            // (kuna `structdefs`) One recovered type definition, so the frame
            // codec's `types` block is exercised by the round-trip below.
            types: vec![TypeInfo {
                name: "mystruct".into(),
                definition: "struct mystruct {\n    int a;\n};\n".into(),
                size: 4,
            }],
            // (kuna `globalref`) One global, so the codec's flag byte is exercised.
            globals: vec![GlobalInfo {
                address: 0x404010,
                name: "dat_404010".into(),
                declaration: "struct_0 dat_404010".into(),
                size: 16,
                unknown: false,
                direct: true,
                aggregate: true,
                elem: true,
            }],
            line_mappings: vec![LineMapping { line_number: 3, addresses: vec![0x401004] }],
            aliases: vec!["_main".into()],
            object_location: Some(ObjectLocation {
                section_index: 2,
                section: ".text".into(),
                offset: 0x40,
            }),
            callee_hints: vec![0x401200, 0x401340, 0xffff_ffff_ffff_fff0],
            synth: None,
            detail: None,
        }
    }

    fn same(a: &FuncResult, b: &FuncResult) -> bool {
        a.name == b.name
            && a.address == b.address
            && a.byte_address == b.byte_address
            && a.size == b.size
            && a.code == b.code
            && a.error == b.error
            && a.proto == b.proto
            && a.aliases == b.aliases
            && a.object_location == b.object_location
            && a.callee_hints == b.callee_hints
            && a.line_mappings == b.line_mappings
            && a.globals == b.globals
            && a.types.len() == b.types.len()
            && a.types.iter().zip(&b.types).all(|(x, y)| {
                x.name == y.name && x.definition == y.definition && x.size == y.size
            })
            && a.variables.len() == b.variables.len()
            && a.variables.iter().zip(&b.variables).all(|(x, y)| {
                x.name == y.name
                    && x.type_name == y.type_name
                    && x.stack_offset == y.stack_offset
                    && x.size == y.size
                    && x.is_param == y.is_param
                    && x.arg_index == y.arg_index
                    && x.line_numbers == y.line_numbers
                    && x.addresses == y.addresses
            })
    }

    #[test]
    fn progress_waits_for_every_worker_then_resets_its_rate_baseline() {
        let start = Instant::now();
        let mut state = ProgressState::new(3, start);

        assert_eq!(state.observe(0, 10, start + Duration::from_secs(10)).1, None);
        assert_eq!(state.observe(0, 20, start + Duration::from_secs(12)).1, None);
        assert_eq!(state.observe(1, 30, start + Duration::from_secs(20)).1, None);
        assert_eq!(state.observe(0, 40, start + Duration::from_secs(22)).1, None);

        let (done, sample) = state.observe(2, 50, start + Duration::from_secs(40));
        assert_eq!((done, sample), (50, None));
        let (done, sample) = state.observe(0, 60, start + Duration::from_secs(42));
        assert_eq!((done, sample), (60, Some((10, 2.0))));
        assert_eq!(progress_eta(1000, done, 10, 2.0), Some(188.0));
    }

    #[test]
    fn result_frames_round_trip_byte_exactly() {
        let dir = ScratchDir::create().unwrap();
        let path = dir.path().join("r.bin").to_string_lossy().into_owned();
        let a = sample_result();
        let b = FuncResult {
            name: "sub_1234".into(),
            address: 0x1234,
            byte_address: 0x1234,
            size: 0,
            code: None,
            error: Some("budget exceeded".into()),
            proto: None,
            variables: Vec::new(),
            types: Vec::new(),
            globals: Vec::new(),
            line_mappings: Vec::new(),
            aliases: Vec::new(),
            object_location: None,
            callee_hints: Vec::new(),
            synth: None,
            detail: None,
        };
        {
            let mut w = ResultWriter::create(&path).unwrap();
            w.push(&a).unwrap();
            w.push(&b).unwrap();
        }
        let decoded = decode_results(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(decoded.len(), 2);
        assert!(same(&decoded[0], &a));
        assert!(same(&decoded[1], &b));
        assert_eq!(
            decoded[0].callee_hints,
            vec![0x401200, 0x401340, 0xffff_ffff_ffff_fff0],
            "the --stream scheduler's frontier has to survive the wire"
        );
        assert!(decoded[1].callee_hints.is_empty());
    }

    #[test]
    fn result_encoding_matches_the_version_four_wire_layout() {
        let dir = ScratchDir::create().unwrap();
        let path = dir.path().join("wire.bin").to_string_lossy().into_owned();
        let result = FuncResult {
            name: "f".into(),
            address: 0x10,
            byte_address: 0x20,
            size: -1,
            code: None,
            error: Some("e".into()),
            proto: None,
            variables: Vec::new(),
            types: Vec::new(),
            globals: Vec::new(),
            line_mappings: Vec::new(),
            aliases: Vec::new(),
            object_location: None,
            callee_hints: Vec::new(),
            synth: None,
            detail: None,
        };
        let expected = b"KUNAJOBRES04\x01\x3f\0\0\0\
            \x10\0\0\0\0\0\0\0\x20\0\0\0\0\0\0\0\
            \xff\xff\xff\xff\xff\xff\xff\xff\
            \x01\0\0\0f\0\x01\x01\0\0\0e\0\0\
            \0\0\0\0\0\0\0\0\0\0\0\0\
            \0\0\0\0\0\0\0\0\0\0\0\0\0";
        ResultWriter::create(&path).unwrap().push(&result).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), expected);
        let decoded = decode_results(expected).unwrap();
        assert_eq!(decoded.len(), 1);
        assert!(same(&decoded[0], &result));
    }

    /// The dynamic source is the only thing `--stream` changes about the pool:
    /// every chunk it hands out is served once, and a drained source ends the
    /// thread's loop.
    #[test]
    fn a_chunk_source_is_drained_exactly_once() {
        let source = PlannedChunks {
            plan: vec![vec![0, 1], vec![2], vec![3, 4, 5]],
            cursor: AtomicUsize::new(0),
        };
        let mut seen: Vec<usize> = Vec::new();
        while let Some(chunk) = source.next_chunk() {
            seen.extend(chunk);
        }
        assert_eq!(seen, (0..6).collect::<Vec<_>>());
        assert!(source.next_chunk().is_none(), "a drained source stays drained");
    }

    #[test]
    fn a_truncated_result_file_keeps_every_complete_frame() {
        let dir = ScratchDir::create().unwrap();
        let path = dir.path().join("t.bin").to_string_lossy().into_owned();
        let a = sample_result();
        {
            let mut w = ResultWriter::create(&path).unwrap();
            w.push(&a).unwrap();
            w.push(&a).unwrap();
        }
        let full = std::fs::read(&path).unwrap();
        for cut in [full.len() - 1, full.len() / 2, RESULT_MAGIC.len() + 3] {
            let decoded = decode_results(&full[..cut]).unwrap();
            assert!(decoded.len() < 2, "a truncated tail must be dropped, not guessed");
            for r in &decoded {
                assert!(same(r, &a));
            }
        }
        assert!(decode_results(b"not a kuna stream").is_none());
    }

    fn target(addr: u64) -> TargetSpec {
        TargetSpec {
            addr,
            space: "ram".into(),
            name: format!("sub_{addr:x}"),
            aliases: Vec::new(),
            size: 0x20,
            object_location: None,
            provenance: EntryProvenance::Mapped,
            binding: None,
            synth: None,
        }
    }

    #[test]
    fn spec_round_trips_and_rejects_garbage() {
        let targets = vec![
            TargetSpec {
                addr: 0x1b8c4f0,
                space: "ram".into(),
                name: "sub_1b8c4f0".into(),
                aliases: vec![],
                size: 0,
                object_location: None,
                provenance: EntryProvenance::UndefinedExternal,
                binding: Some("weak".into()),
                synth: Some(vec![Some("struct_3".into()), None]),
            },
            TargetSpec {
                addr: 0x401000,
                space: "ram".into(),
                name: "main".into(),
                aliases: vec!["_main".into(), "__libc_main".into()],
                size: 0x120,
                object_location: Some(ObjectLocation {
                    section_index: 1,
                    section: ".text".into(),
                    offset: 0,
                }),
                provenance: EntryProvenance::DefinedObject,
                binding: Some("global".into()),
                synth: None,
            },
        ];
        assert_eq!(decode_spec(&encode_spec(&targets)).unwrap(), targets);
        assert!(decode_spec(b"KUNAJOBSPEC3\xff\xff\xff\xff").is_none());
    }

    #[test]
    fn spec_encoding_matches_the_version_three_wire_layout() {
        let expected = b"KUNAJOBSPEC3\x01\0\0\0\
            \0\x10\0\0\0\0\0\0\
            \x03\0\0\0ram\x08\0\0\0sub_1000\
            \0\0\0\0\x20\0\0\0\0\0\0\0\
            \0\0\0\0";
        assert_eq!(encode_spec(&[target(0x1000)]), expected);
        assert_eq!(decode_spec(expected).unwrap(), vec![target(0x1000)]);
    }

    /// A worker killed mid-chunk keeps its flushed prefix; the rest of ITS chunk
    /// becomes `error` records, still one per target and still in target order,
    /// so the surrounding run is unaffected.
    #[test]
    fn a_dead_worker_degrades_only_its_own_missing_functions() {
        let chunk: Vec<TargetSpec> =
            [0x1000u64, 0x2000, 0x3000].iter().map(|&a| target(a)).collect();
        let mut survived = sample_result();
        survived.address = 0x2000;
        survived.byte_address = 0x2000;
        survived.name = "sub_2000".into();

        let (merged, delivered) =
            merge_chunk(&chunk, vec![survived], "worker chunk failed (signal: 11)");
        assert_eq!(merged.len(), 3);
        assert_eq!(delivered, vec![false, true, false], "only the flushed record is the worker's");
        assert_eq!(
            merged.iter().map(|r| r.address).collect::<Vec<_>>(),
            vec![0x1000, 0x2000, 0x3000],
            "target order must survive a crashed worker"
        );
        assert!(merged[1].code.is_some() && merged[1].error.is_none(), "flushed record kept");
        for lost in [&merged[0], &merged[2]] {
            assert!(lost.code.is_none());
            assert_eq!(lost.error.as_deref(), Some("worker chunk failed (signal: 11)"));
            // The parent's own inventory facts still describe the function.
            assert_eq!(lost.size, 0x20);
        }
        let (none, delivered) = merge_chunk(&chunk, Vec::new(), "boom");
        assert_eq!(none.len(), 3);
        assert_eq!(delivered, vec![false; 3]);
        assert!(none.iter().all(|r| r.error.as_deref() == Some("boom")));
    }

    fn died(stalled: bool, started: bool, warm: bool) -> Ending {
        Ending::Died { stalled, started, warm }
    }

    fn reruns(suspect: Option<usize>, bystanders: &[usize]) -> Reruns {
        Reruns { suspect, bystanders: bystanders.to_vec() }
    }

    /// The re-run policy, case by case.  A crash re-runs the function it
    /// happened in first and then everything that never started; a stall
    /// re-runs only what never started; a worker that ran none of the chunk
    /// re-runs all of it, unless it may have died in its own load.
    #[test]
    fn a_dead_worker_reruns_its_suspect_first_and_its_bystanders_after() {
        let prefix = [true, true, false, false, false];
        assert_eq!(retry_order(died(false, true, true), &prefix), reruns(Some(2), &[3, 4]));
        assert_eq!(retry_order(died(false, true, false), &prefix), reruns(Some(2), &[3, 4]));
        assert_eq!(
            retry_order(died(true, true, true), &prefix),
            reruns(None, &[3, 4]),
            "a stalled suspect already ran four budgets; only its bystanders are re-run"
        );
        assert!(retry_order(died(true, true, false), &[false]).is_empty());
        assert_eq!(
            retry_order(died(false, true, false), &[false]),
            reruns(Some(0), &[]),
            "a planned singleton that crashed is run once more on a fresh worker"
        );

        let untouched = [false; 5];
        assert_eq!(
            retry_order(died(false, false, true), &untouched),
            reruns(None, &[0, 4, 2, 1, 3]),
            "a warm worker that died between chunks ran none of this one"
        );
        let everything = reruns(None, &[0, 4, 2, 1, 3]);
        assert_eq!(retry_order(died(true, false, true), &untouched), everything);
        assert!(
            retry_order(died(false, false, false), &untouched).is_empty(),
            "a worker that never finished a chunk may have died in its load"
        );
        assert!(retry_order(died(true, false, false), &untouched).is_empty());

        for ending in [Ending::Finished, Ending::NotRun] {
            assert!(retry_order(ending, &untouched).is_empty(), "{ending:?} re-runs nothing");
        }
        assert!(retry_order(died(false, true, true), &[true; 4]).is_empty());

        // A re-run counts as run only once its worker reached the target; one
        // that did not ends its chunk's re-runs instead of spawning for the rest.
        assert!(Ending::Finished.started());
        assert!(died(false, true, false).started() && died(true, true, true).started());
        assert!(!Ending::NotRun.started());
        assert!(!died(false, false, false).started() && !died(true, false, true).started());
    }

    /// Bystanders are visited in bit-reversed order, so any prefix of the visit
    /// samples the whole chunk: a run of adjacent crashers is spread out among
    /// the recoveries instead of arriving first.
    #[test]
    fn bystanders_are_visited_spread_across_the_chunk() {
        for count in [0usize, 1, 2, 3, 7, 20, 64, 219, 512] {
            let order = spread(count);
            let mut sorted = order.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, (0..count).collect::<Vec<_>>(), "a permutation of {count}");
        }
        assert_eq!(spread(8), vec![0, 4, 2, 6, 1, 5, 3, 7]);
        let first: Vec<usize> = spread(219).into_iter().take(16).collect();
        assert!(first.iter().filter(|&&k| k < 16).count() <= 2, "the first 16 re-runs: {first:?}");
    }

    /// A chunk stops once enough of its bystanders failed and they outnumber
    /// the recovered ones, or once two of its re-runs stalled; neighbouring
    /// crashers, the expected failure of the function that was running, and a
    /// single stall never stop it.
    #[test]
    fn a_chunk_stops_rerunning_only_when_failures_dominate() {
        let mut adjacent = ChunkReruns::default();
        assert_eq!(adjacent.record(true, false, false), None, "the suspect does not count");
        for _ in 0..CHUNK_RERUN_FAILURES - 1 {
            assert_eq!(adjacent.record(false, false, false), None);
        }
        assert_eq!(adjacent.record(false, false, false), Some(NOT_RERUN_CRASHING));

        let mut alternating = ChunkReruns::default();
        for _ in 0..100 {
            assert_eq!(alternating.record(false, true, false), None);
            assert_eq!(alternating.record(false, false, false), None, "a tie never stops");
        }

        let mut mostly_failing = ChunkReruns::default();
        let stops: Vec<_> =
            (0..30).map(|i| mostly_failing.record(false, i % 3 == 0, false)).collect();
        assert!(stops.contains(&Some(NOT_RERUN_CRASHING)), "two failures per recovery stop it");

        let mut stalls = ChunkReruns::default();
        assert_eq!(stalls.record(true, false, true), None, "one stall is a function");
        for _ in 0..20 {
            assert_eq!(stalls.record(false, true, false), None);
        }
        assert_eq!(stalls.record(false, false, true), Some(NOT_RERUN_STALLED), "the second stall");
    }

    /// The run-wide check needs both a floor of failed re-runs and more failures
    /// than delivered records, and it is asked afresh: deliveries that catch up
    /// let re-runs start again, and the warning is claimed once.
    #[test]
    fn the_retry_gate_holds_back_reruns_only_while_failures_outnumber_deliveries() {
        let gate = RetryGate::default();
        gate.delivered(3);
        for _ in 0..RUN_RERUN_FAILURES - 1 {
            gate.failed();
        }
        assert!(gate.allows(), "below the floor");
        gate.failed();
        assert!(!gate.allows(), "{RUN_RERUN_FAILURES} failures against 3 records");
        assert!(gate.first_refusal() && !gate.first_refusal(), "warned once");
        gate.delivered(RUN_RERUN_FAILURES - 3);
        assert!(gate.allows(), "as many records as failures: re-running resumes");
        gate.failed();
        assert!(!gate.allows());

        let working = RetryGate::default();
        working.delivered(40);
        for _ in 0..40 {
            working.failed();
        }
        assert!(working.allows(), "workers that deliver as much as fails keep re-running");
        working.failed();
        assert!(!working.allows());
    }

    #[test]
    fn the_fault_hook_parses_what_the_tests_inject_and_reports_the_rest() {
        assert_eq!(Faults::parse(""), (Faults::default(), Vec::new()));
        let (none, rejected) = Faults::parse("nonsense,panic:,stall:0xzz,spawn:x,boom:0x1");
        assert_eq!(none, Faults::default());
        assert_eq!(rejected, vec!["nonsense", "panic:", "stall:0xzz", "spawn:x", "boom:0x1"]);
        let (f, rejected) =
            Faults::parse(" panic:0x40071d , panic-once:4196064,stall:*,spawn:2,panic:0X10");
        assert!(rejected.is_empty(), "{rejected:?}");
        assert_eq!(
            f.at,
            vec![
                (Fault::Panic, Some(0x40071d)),
                (Fault::PanicOnce, Some(4196064)),
                (Fault::Stall, None),
                (Fault::Panic, Some(0x10))
            ]
        );
        assert!(!f.refuses_spawn(1) && f.refuses_spawn(2) && f.refuses_spawn(9));
        assert!(!Faults::default().refuses_spawn(0));
        let (f, rejected) = Faults::parse("synth:serial,synth:force,synth:noinstall,synth:other");
        assert!(f.synth_serial && f.synth_force && f.refuses_synth_install());
        assert_eq!(rejected, vec!["synth:other"]);

        let dir = ScratchDir::create().unwrap();
        Faults::default().before_target(dir.path(), 0x1000);
        Faults::parse("panic:0x2000").0.before_target(dir.path(), 0x1000);
        let once = Faults::parse("panic-once:0x1000").0;
        assert!(std::panic::catch_unwind(|| once.before_target(dir.path(), 0x1000)).is_err());
        once.before_target(dir.path(), 0x1000);

        let lost = lost_result(&target(0x1000), "worker stalled past the per-function watchdog");
        let marked = not_rerun(lost, NOT_RERUN_STALLED);
        let error = marked.error.as_deref().unwrap();
        assert!(error.ends_with(&format!("; not re-run: {NOT_RERUN_STALLED}")), "{error}");
        assert_eq!(count_error_anomalies(std::iter::once(error)), (0, 1), "still classified");
    }

    /// The closing lines say how many functions the re-runs recovered and how
    /// many are still lost, and a run whose re-runs recovered everything still
    /// says a worker died.
    #[test]
    fn the_anomaly_lines_separate_recovered_from_lost() {
        let none = Retries::default();
        assert!(anomaly_lines((0, 0), 10, JOBS_TAG, none).is_empty());

        let all_back =
            anomaly_lines((0, 0), 10, JOBS_TAG, Retries { recovered: 20, failed_alone: 0 });
        assert_eq!(all_back.len(), 1);
        assert!(all_back[0].starts_with("[kuna --jobs] 20 function(s) left unfinished"));
        assert!(!all_back[0].contains("warning"), "nothing is missing: {all_back:?}");

        let mixed =
            anomaly_lines((1, 3), 10, STREAM_TAG, Retries { recovered: 18, failed_alone: 2 });
        assert_eq!(mixed.len(), 3, "{mixed:?}");
        assert!(mixed[0].contains("1 function(s) hit the 10s per-function watchdog"));
        assert!(mixed[1].starts_with("[kuna --stream] 18 function(s)"));
        assert!(mixed[2].starts_with("[kuna --stream] warning: 3 function(s) have no result"));
        assert!(mixed[2].contains(" 2 of them failed again when re-run on their own."));

        let unretried = anomaly_lines((0, 5), 0, JOBS_TAG, none);
        assert_eq!(unretried.len(), 1);
        assert!(!unretried[0].contains("re-run on their own"), "{unretried:?}");
    }

    /// Cleanup is a `Drop`, not a step on the happy path: the directory holds the
    /// whole program's symbol inventory and every worker's decompiled C, and
    /// [`run_pool`] can also leave by an early `Err` or a panicking pool thread.
    #[test]
    fn the_scratch_dir_is_private_and_removed_on_every_path() {
        let kept = {
            let dir = ScratchDir::create().unwrap();
            let path = dir.path().to_path_buf();
            assert!(path.is_dir());
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
                assert_eq!(
                    mode, 0o700,
                    "worker C and symbols must not transit a world-readable dir"
                );
            }
            std::fs::write(path.join("inventory.spec"), b"payload").unwrap();
            path
        };
        assert!(!kept.exists(), "a non-empty scratch dir must go with its guard");

        let leaked = std::sync::Mutex::new(PathBuf::new());
        let _ = std::panic::catch_unwind(|| {
            let dir = ScratchDir::create().unwrap();
            *leaked.lock().unwrap() = dir.path().to_path_buf();
            panic!("pool thread died");
        });
        assert!(!leaked.lock().unwrap().exists());
    }

    /// A worker only ever deletes a directory the pool itself named.
    #[test]
    fn a_worker_recognizes_only_its_own_scratch_dir() {
        assert_eq!(
            pool_scratch("/tmp/kuna-jobs-4242-99"),
            Some(PathBuf::from("/tmp/kuna-jobs-4242-99"))
        );
        for foreign in ["/tmp", "/home/u/out", "kuna-jobs-1/nested", "/"] {
            assert_eq!(pool_scratch(foreign), None, "{foreign} is not a pool scratch dir");
        }
        assert_eq!(scratch_owner("kuna-jobs-4242-17384"), Some(4242));
        for bad in ["kuna-jobs-", "kuna-jobs-abc-1", "kunajobs-1-2", "tmpdir"] {
            assert_eq!(scratch_owner(bad), None, "{bad}");
        }
    }

    /// The sweep is the last resort for the one window a dying process cannot
    /// cover, so it must be exact about ownership: this process is alive, so its
    /// own directory is never a candidate.
    #[test]
    fn the_sweep_spares_a_live_owner() {
        let dir = ScratchDir::create().unwrap();
        let mine = dir.path().to_path_buf();
        let temp = mine.parent().unwrap().to_path_buf();
        let dead = temp.join(format!("{SCRATCH_PREFIX}{}-11", u32::MAX));
        let unrelated = temp.join(format!("kuna-not-a-job-{}", std::process::id()));
        std::fs::create_dir_all(&dead).unwrap();
        std::fs::create_dir_all(&unrelated).unwrap();

        sweep_stale_scratch(&temp);

        assert!(mine.is_dir(), "a live run's directory must survive the sweep");
        assert!(unrelated.is_dir(), "the sweep must not touch directories it did not create");
        if Path::new("/proc/self/stat").exists() {
            assert!(!dead.exists(), "a dead owner's directory must be swept");
        }
        let _ = std::fs::remove_dir_all(&dead);
        let _ = std::fs::remove_dir_all(&unrelated);
    }

    /// Both silent-difference classes are counted, and an ordinary decompile
    /// failure is neither of them.
    #[test]
    fn worker_failures_are_counted_not_just_budget_trips() {
        let err = |e: &str| FuncResult {
            name: "f".into(),
            address: 0,
            byte_address: 0,
            size: 0,
            code: None,
            error: Some(e.to_string()),
            proto: None,
            variables: Vec::new(),
            types: Vec::new(),
            globals: Vec::new(),
            line_mappings: Vec::new(),
            aliases: Vec::new(),
            object_location: None,
            callee_hints: Vec::new(),
            synth: None,
            detail: None,
        };
        let results = vec![
            err("budget exceeded (30s)"),
            err(&format!("{CHUNK_FAILED} (signal: 9 (SIGKILL))")),
            err(&format!("{NO_RECORD} for this function")),
            err(&format!("{SPAWN_FAILED}: No such file or directory")),
            err(&format!("{SPEC_WRITE_FAILED}: No space left on device")),
            err(&format!("{STALLED} (10s); the worker was killed")),
            err("unable to load 8 bytes at 0x1000"),
            sample_result(),
        ];
        assert_eq!(count_anomalies(&results), (1, 5));
        assert_eq!(count_anomalies(&[sample_result()]), (0, 0));
    }

    fn cfg(max_fn_seconds: u64, load_seconds: f64) -> PoolConfig<'static> {
        PoolConfig {
            jobs: 4,
            jobs_auto: false,
            chunk: None,
            binary: "/bin/true",
            mode: "fast",
            options: &[],
            func_decls: Vec::new(),
            no_vars: false,
            want_proto: false,
            want_provenance: false,
            want_types: false,
            want_callee_hints: false,
            max_fn_seconds,
            full_load: false,
            load_seconds,
            isa: None,
            slice: None,
            target: None,
            sleighpath: None,
            synth_base: None,
            serial_callee_first: false,
            elem_objects: false,
        }
    }

    /// A pool that runs its workers with `structsynth off` (the `--stream` one)
    /// says why it has no `struct_N`, and stays quiet where the run had turned
    /// synthesis off itself.
    #[test]
    fn structsynth_shard_note_for_every_run_that_would_synthesize() {
        let note = structsynth_shard_note(&cfg(0, 0.0), JOBS_TAG).expect("default param");
        assert!(note.starts_with("[kuna --jobs] note: structsynth is off"), "{note}");
        let project = PoolConfig { want_types: true, ..cfg(0, 0.0) };
        assert!(structsynth_shard_note(&project, STREAM_TAG).is_some());
        let off = [("structsynth".to_string(), "off".to_string())];
        let turned_off = PoolConfig { options: &off, ..cfg(0, 0.0) };
        assert!(structsynth_shard_note(&turned_off, JOBS_TAG).is_none());
        let param = [("structsynth".to_string(), "param".to_string())];
        let asked = PoolConfig { options: &param, ..cfg(0, 0.0) };
        assert!(structsynth_shard_note(&asked, JOBS_TAG).is_some());
    }

    /// A `struct_N` that is a symbol's name, not a type's, must not be rewritten
    /// with the type: such a function is decompiled again instead.
    #[test]
    fn a_rename_covering_a_symbol_name_is_refused() {
        let map = [("struct_3".to_string(), "struct_0".to_string())];
        let mut r = sample_result();
        assert!(only_types_are_renamed(&r, &map));
        r.variables[0].name = "struct_3".into();
        assert!(!only_types_are_renamed(&r, &map));
        r.variables[0].name = "param_1".into();
        r.aliases.push("struct_3".into());
        assert!(!only_types_are_renamed(&r, &map));
        r.aliases.clear();
        r.name = "struct_3".into();
        assert!(!only_types_are_renamed(&r, &map));
    }

    /// (kuna `protoorder`) The names a pool replays are a serial run's, and on
    /// `decompile-all` that run is the one without the callee-first order: the
    /// report has to name it, not `--jobs 1`.
    #[test]
    fn the_report_names_the_serial_run_it_can_actually_replay() {
        assert_eq!(serial_run(&cfg(0, 0.0)), "--jobs 1");
        let callee_first = PoolConfig { serial_callee_first: true, ..cfg(0, 0.0) };
        assert_eq!(serial_run(&callee_first), "--jobs 1 --option protoorder off");
    }

    /// The pool is the only thing that can enforce the per-function budget on a
    /// function wedged where nothing probes the cooperative deadline, so the
    /// window has to be loose enough never to fire on a merely slow one — and
    /// absent entirely when the budget itself is off.
    #[test]
    fn the_stall_window_allows_a_load_then_tightens() {
        assert_eq!(stall_deadline(&cfg(0, 70.0), false), None, "no budget, no stall kill");
        assert_eq!(stall_deadline(&cfg(0, 70.0), true), None);

        let cold = stall_deadline(&cfg(10, 70.0), false).unwrap();
        let warm = stall_deadline(&cfg(10, 70.0), true).unwrap();
        assert_eq!(warm, Duration::from_secs(40), "4x the budget once the worker is loaded");
        assert!(
            cold > warm + Duration::from_secs(200),
            "a cold worker still has to load: {cold:?}"
        );
        assert!(
            stall_deadline(&cfg(10, 0.0), false).unwrap() < cold,
            "the grace tracks the parent's own load time"
        );
        assert!(
            stall_deadline(&cfg(120, 70.0), true).unwrap() > warm,
            "a bigger budget is a bigger window"
        );
    }

    #[test]
    fn jobs_values_parse() {
        assert_eq!(parse_jobs("1").unwrap(), (1, false));
        assert_eq!(parse_jobs(" 32 ").unwrap(), (32, false));
        let (auto, is_auto) = parse_jobs("auto").unwrap();
        assert!(is_auto, "`auto` must be reported as auto, so the memory trim may lower it");
        assert!((1..=MAX_AUTO_JOBS).contains(&auto), "auto must stay inside the cap: {auto}");
        assert_eq!(parse_jobs("AUTO").unwrap(), (auto, true));
        for bad in ["0", "-4", "", "many"] {
            assert!(parse_jobs(bad).is_err(), "{bad:?} must be rejected");
        }
    }

    /// The memory trim runs on every pool start, so it decides the worker count
    /// as much as `--jobs` does.  `auto` is a promise not to wreck the machine
    /// and yields to what free memory holds; an explicit count is an instruction
    /// and is obeyed with a warning.  A request the machine can hold is never
    /// touched either way.
    #[cfg(target_os = "linux")]
    #[test]
    fn auto_yields_to_free_memory_and_an_explicit_count_does_not() {
        let mut c = cfg(0, 0.0);
        c.jobs = 1;
        assert_eq!(affordable_jobs(&c, JOBS_TAG), 1, "one worker fits on any machine that can run this");

        // Far past what any machine holds, so the trim is reached wherever this
        // runs rather than only on a loaded box.
        c.jobs = 1_000_000;
        c.jobs_auto = true;
        let trimmed = affordable_jobs(&c, JOBS_TAG);
        assert!(trimmed >= 1, "the trim must still leave a pool: {trimmed}");
        assert!(trimmed < c.jobs, "`auto` must come down to what fits: {trimmed}");

        c.jobs_auto = false;
        assert_eq!(
            affordable_jobs(&c, JOBS_TAG),
            1_000_000,
            "an explicit --jobs N is obeyed, not lowered"
        );
    }

    /// Whatever the planning policy, the plan must cover every slot exactly once
    /// — that is what makes the positional merge total.
    #[test]
    fn the_plan_covers_every_target_exactly_once() {
        for explicit in [None, Some(1), Some(7), Some(0), Some(100_000)] {
            for jobs in [1, 8, 64] {
                for count in [1usize, 3, 17, 2000] {
                    let targets: Vec<TargetSpec> = (0..count as u64)
                        .map(|i| {
                            let mut t = target(0x1000 + i * 0x20);
                            t.size = if i % 7 == 0 { 0x4000 } else { 0x20 };
                            t
                        })
                        .collect();
                    let plan = plan_chunks(&targets, explicit, jobs);
                    let mut seen: Vec<usize> = plan.iter().flatten().copied().collect();
                    seen.sort_unstable();
                    assert_eq!(
                        seen,
                        (0..count).collect::<Vec<_>>(),
                        "{explicit:?} jobs={jobs} count={count}"
                    );
                    let cap = MAX_AUTO_CHUNK.max(explicit.unwrap_or(0));
                    assert!(plan.iter().all(|c| !c.is_empty() && c.len() <= cap));
                }
            }
        }
        assert!(plan_chunks(&[], None, 8).is_empty());
    }

    /// The scheduling contract: the long poles are dispatched first and are not
    /// buried in a fat chunk, while the cheap majority is batched so the worker
    /// load is amortized.
    #[test]
    fn expensive_functions_go_first_and_alone_cheap_ones_batch() {
        let mut targets: Vec<TargetSpec> = Vec::new();
        for i in 0..1000u64 {
            let mut t = target(0x1000 + i * 0x20);
            t.size = if i % 400 == 399 { 0x8000 } else { 0x20 };
            targets.push(t);
        }
        let plan = plan_chunks(&targets, None, 16);
        let whales: Vec<usize> =
            (0..targets.len()).filter(|&i| targets[i].size >= 0x8000).collect();
        assert!(whales.len() >= 2, "fixture must contain whales");

        let first_chunk_of = |i: usize| plan.iter().position(|c| c.contains(&i)).unwrap();
        for &w in &whales {
            assert!(first_chunk_of(w) < whales.len(), "whale {w} dispatched late");
            assert_eq!(plan[first_chunk_of(w)].len(), 1, "whale {w} shares a chunk");
        }
        let tail: Vec<usize> = plan[whales.len()..].iter().map(Vec::len).collect();
        assert!(tail.iter().all(|&n| n > 1), "cheap functions must batch: {tail:?}");
        assert_eq!(plan, plan_chunks(&targets, None, 16), "planning is deterministic");

        let fixed = plan_chunks(&targets, Some(7), 16);
        assert!(fixed[..fixed.len() - 1].iter().all(|c| c.len() == 7));
        assert!(whales.contains(&fixed[0][0]));
    }

    /// The `.h` merge only claims identity when one shard holds every
    /// definition; when none does it still declares everything once, and says
    /// so.
    #[test]
    fn the_type_block_is_the_shard_that_holds_them_all_or_their_union() {
        let a = "typedef struct s s;\n\nstruct s {\n    int x;\n};\n".to_string();
        assert_eq!(merge_type_definitions(&[a.clone(), a.clone()], JOBS_TAG), a);
        assert_eq!(merge_type_definitions(&[], JOBS_TAG), "");
        assert_eq!(merge_type_definitions(std::slice::from_ref(&a), JOBS_TAG), a);

        // A worker's block without its structures holds nothing the full one
        // lacks, wherever it comes.
        let full = "typedef struct s s;\ntypedef struct struct_0 struct_0;\n\nstruct s {\n    int x;\n};\n\n\
                    struct struct_0 {\n    int x;\n};\n"
            .to_string();
        assert_eq!(merge_type_definitions(&[a.clone(), full.clone()], JOBS_TAG), full);
        assert_eq!(merge_type_definitions(&[full.clone(), a.clone(), String::new()], JOBS_TAG), full);

        // Two blocks that each hold a type the other does not.
        let t = "typedef struct t t;\n\nstruct t {\n    int x;\n};\n".to_string();
        let merged = merge_type_definitions(&[full.clone(), t], JOBS_TAG);
        assert_eq!(
            merged,
            format!("{full}typedef struct t t;\n\nstruct t {{\n    int x;\n}};\n"),
            "one blank line before a spaced definition, none repeated"
        );
        for item in ["struct s {\n    int x;\n};", "struct t {\n    int x;\n};", "struct struct_0 {\n    int x;\n};"] {
            assert_eq!(merged.matches(item).count(), 1, "{item} must appear exactly once");
        }
    }

    /// The parent's own decode lanes must not price its workers. Measured on a
    /// 147 MB binary: 6.02 GB of peak serial against 9.09 GB at `--jobs 16`, of
    /// which 3.07 GB is lanes -- a worker, forced back to one lane, pays none of
    /// it and must be estimated at the serial number.
    #[test]
    fn the_worker_estimate_ignores_what_the_decode_lanes_added() {
        let gb = |n: u64| n * 1024 * 1024 * 1024;
        let serial = worker_estimate_from(6_167_417_856, 0, false);
        let laned = worker_estimate_from(9_305_874_432, 3_138_456_576, false);
        assert_eq!(laned, serial, "a laned load must estimate a worker as a serial one does");
        assert_eq!(worker_estimate_from(gb(8), gb(2), true), gb(6), "--jobs-full-load too");
        // The floor still holds, and nothing underflows when the excess is stale
        // and larger than this load's peak.
        assert_eq!(worker_estimate_from(gb(1), gb(4), false), 256 * 1024 * 1024);
        assert_eq!(worker_estimate_from(gb(4), 0, false), gb(1));
    }

    /// ...and it must not price them LOW either, which is the direction that
    /// ends in an OOM kill rather than a narrow pool. Measured on the same
    /// binary under `--option listing on` (what `--mode aggressive|reliable`
    /// injects), where the walk's map costs ~713 B an instruction instead of the
    /// 298 the excess subtraction is calibrated on: VmHWM 14.08 GB serial
    /// against 17.73 GB at 16 lanes, so the lanes added 3.65 GB -- but an
    /// unbounded subtraction reported 11.19 GB and priced a worker at 1.6 GB
    /// where the honest number is 3.4 GB.
    #[test]
    fn the_worker_estimate_is_not_priced_low_by_an_over_reported_lane_excess() {
        let kb = |n: u64| n * 1024;
        let serial = worker_estimate_from(kb(14_079_200), 0, false);
        let over_reported = worker_estimate_from(kb(17_727_772), kb(11_730_186), false);
        assert!(
            over_reported * 2 < serial,
            "this is the defect: {over_reported} against {serial}"
        );
        // Bounded by the lanes' own footprint (`kuna_pdecode::lane_footprint`),
        // the same load reports what the lanes hold and the estimate lands
        // within a tenth of the serial one.
        let bounded = worker_estimate_from(kb(17_727_772), kb(4_257_000), false);
        let ratio = bounded as f64 / serial as f64;
        assert!(ratio > 0.85 && ratio < 1.15, "bounded estimate is {ratio:.2}x the serial one");
    }
}
