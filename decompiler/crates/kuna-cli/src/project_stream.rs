//! `kuna decompile-project --stream` — the **streamed** project export.
//!
//! ```text
//!   kuna decompile-project <binary> --stream [--jobs N] [-o DIR]
//! ```
//!
//! A non-stream export writes nothing until the last function is decompiled, so
//! an agent studying a large binary waits out the whole run before it can read a
//! line of C.  A streamed export writes the folder as it goes, entry-point-first:
//!
//! * before the load — `README.md` and `.streaming` (phase `loading`), and
//!   nothing else, so a previous export's `.c`/`.h`/`.asm` survive a failed load;
//! * once the program is loaded — the four artifacts plus `index.jsonl` are
//!   created, the image entry point and `main` are decompiled in-process as
//!   SEEDS, and their callee hints become the scheduler's frontier;
//! * then the decompile (a worker pool at `--jobs N`, the main thread at
//!   `--jobs 1`), the `.asm` sweep and the writer run concurrently, each function
//!   appended to the `.c` and then announced in `index.jsonl`;
//! * finally the type block, the `.h`, the `.asm` tails and the README are
//!   written and `.streaming` is removed.
//!
//! Order is the deliberate difference from a non-stream run: the `.c` is in
//! DECOMPILE order (seeds, their callees breadth-first, then the remaining
//! targets address-ordered) and `index.jsonl` is the address-to-offset index that
//! replaces it.  The function SET is identical.
//!
//! The resumable sweep, the `.asm` tails and the README renderers live in
//! `kuna_console::project_stream` / `kuna_console::project`; this module is the
//! orchestration — the scheduler, the writer and the timeline.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use kuna_console::engine::{ConsoleProgram, EntrySelector, FunctionEntry};
use kuna_console::project::{
    build_header, collect_dat_addrs, decompile_pulled, render_c, render_readme,
    render_readme_streaming, AsmPhase, DecompileOptions, FuncResult, ReadmeCounts, ReadmeFacts,
    ReadmeLayout, StreamPhase, StreamProgress,
};
use kuna_console::project_stream::{
    asm_labels_from_entries, render_data_tail, render_variables_section, AsmSweep,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};
use kuna_sleigh::loadimage::section_flags;

use crate::decompile_all::{
    flatten_targets, load_program, pool_config, resolve_targets, Args, DriverDefaults,
};
use crate::jobs::{self, ChunkSource, TargetSpec};
use crate::jsonfmt::{dumps_compact, Json};

/// The status file, present only while an export is running.
const STATUS_FILE: &str = ".streaming";
const INDEX_FILE: &str = "index.jsonl";
const README_FILE: &str = "README.md";

/// How long the writer waits for a result before going round its clock again.
const TICK: Duration = Duration::from_millis(500);
/// How often the load clock looks at whether the load has finished.
const CLOCK_GRANULARITY: Duration = Duration::from_millis(100);
/// How often the running README is rewritten, when the counts moved.
const README_EVERY: Duration = Duration::from_secs(5);
/// The `.h` rewrite back-off: never more often than this, and at least this
/// seldom once the prototype count stops doubling.
const HEADER_MIN: Duration = Duration::from_secs(2);
const HEADER_MAX: Duration = Duration::from_secs(60);

/// One sweep step's budget when the sweep has a thread to itself (`--jobs N`).
/// Only a flush rhythm: it decides how often a reader sees the `.asm` grow.
const SWEEP_STEP_BYTES: u64 = 4 * 1024 * 1024;
/// How many interleaved steps the serial (`--jobs 1`) sweep is cut into, and the
/// floor under one step.  The `.asm` therefore completes within the first
/// `SWEEP_STEPS` decompile batches rather than at the end of the run.
const SWEEP_STEPS: u64 = 16;
const MIN_SWEEP_STEP_BYTES: u64 = 1024 * 1024;
/// The serial interleave's decompile batch, growing from one function (so the
/// first `.c` block lands immediately) to this ceiling.
const SERIAL_BATCH_MAX: usize = 64;
const SERIAL_BATCH_GROWTH: usize = 8;
/// Targets below which the `--jobs auto` hint is noise: a handful of functions
/// is not a run anyone would parallelise.
const SERIAL_HINT_MIN: usize = 32;

/// What a producer reports when all it saw was its channel close — the writer's
/// own error is what the run reports whenever it has one.
const WRITER_STOPPED: &str = "the streaming writer stopped early";

/// Where the export writes and what it calls itself.
struct Layout {
    binary_path: PathBuf,
    path_label: String,
    file_name: String,
    out_dir: PathBuf,
}

impl Layout {
    /// Resolve the output folder.  The binary has to exist first: a typo'd path
    /// must fail the way the non-stream export fails it, before a folder, a
    /// status file or a README is created anywhere.
    fn resolve(args: &Args, output: Option<&str>) -> Result<Self, String> {
        let binary_path = std::fs::canonicalize(&args.binary)
            .map_err(|_| format!("binary not found: {}", args.binary))?;
        let file_name = binary_path
            .file_name()
            .ok_or_else(|| format!("binary has no file name: {}", args.binary))?
            .to_string_lossy()
            .into_owned();
        let out_dir = match output {
            Some(dir) => PathBuf::from(dir),
            None => binary_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(format!("{file_name}.kuna")),
        };
        Ok(Self { path_label: binary_path.display().to_string(), binary_path, file_name, out_dir })
    }
}

/// `.streaming`, as a value.  The one place a reader learns how far along the
/// export is, and the only artifact that is removed rather than finished.
#[derive(Debug, Clone)]
struct StreamStatus {
    phase: StreamPhase,
    pid: u32,
    started: Instant,
    started_at: u64,
    last_result: Instant,
    jobs: usize,
    /// Seed functions the scheduler started from; `0` means no seed was a
    /// target, so the order degenerates to address order.
    seeds: usize,
    total: Option<usize>,
    done: usize,
    failed: usize,
    c_bytes: u64,
    asm: AsmPhase,
    error: Option<String>,
}

impl StreamStatus {
    fn new(jobs: usize) -> Self {
        let now = Instant::now();
        Self {
            phase: StreamPhase::Loading,
            pid: std::process::id(),
            started: now,
            started_at: unix_now(),
            last_result: now,
            jobs,
            seeds: 0,
            total: None,
            done: 0,
            failed: 0,
            c_bytes: 0,
            asm: AsmPhase::Pending,
            error: None,
        }
    }

    fn progress(&self) -> StreamProgress {
        StreamProgress {
            phase: self.phase,
            jobs: self.jobs,
            total: self.total,
            done: self.done,
            failed: self.failed,
            asm: self.asm,
            error: self.error.clone(),
            elapsed_s: self.started.elapsed().as_secs(),
        }
    }

    fn to_json(&self) -> Json {
        let num = |v: u64| Json::Number(v.to_string());
        let count = |v: Option<usize>| match v {
            Some(v) => Json::Number(v.to_string()),
            None => Json::Null,
        };
        Json::Object(vec![
            ("schema".into(), num(1)),
            ("phase".into(), Json::Str(self.phase.as_str().into())),
            ("pid".into(), num(u64::from(self.pid))),
            ("started_at".into(), num(self.started_at)),
            ("updated_at".into(), num(unix_now())),
            ("elapsed_s".into(), num(self.started.elapsed().as_secs())),
            ("jobs".into(), num(self.jobs as u64)),
            ("seeds".into(), num(self.seeds as u64)),
            ("functions_total".into(), count(self.total)),
            ("functions_done".into(), num(self.done as u64)),
            ("functions_failed".into(), num(self.failed as u64)),
            ("seconds_since_last_result".into(), num(self.last_result.elapsed().as_secs())),
            ("c_bytes".into(), num(self.c_bytes)),
            ("asm".into(), Json::Str(self.asm.as_str().into())),
            (
                "error".into(),
                self.error.clone().map_or(Json::Null, Json::Str),
            ),
        ])
    }
}

/// Everything both the writer thread and the main thread report from.
struct Shared {
    facts: ReadmeFacts,
    status: StreamStatus,
}

/// The run's stop signal.
///
/// The writer thread owns every artifact a reader polls, so when it dies the
/// producers have to stop at the function they are on instead of decompiling
/// the rest of the binary into a closed channel — on a 392,814-function image
/// that is hours of full-rate work nobody will ever read.
#[derive(Default)]
struct Abort {
    stopped: AtomicBool,
    /// The writer's own error, which is the run's real cause; a producer only
    /// ever sees its channel close.
    error: Mutex<Option<String>>,
}

impl Abort {
    fn stop(&self, error: Option<String>) {
        if let Some(error) = error {
            let mut slot = self.error.lock().unwrap_or_else(|e| e.into_inner());
            slot.get_or_insert(error);
        }
        self.stopped.store(true, Ordering::SeqCst);
    }

    fn stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

    /// Why the run is stopping, preferring the writer's own message.
    fn reason(&self) -> String {
        self.error
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .unwrap_or_else(|| WRITER_STOPPED.to_string())
    }
}

/// The run as every thread sees it: the status value, the stop signal, and the
/// folder they all report into.
struct StreamRun {
    dir: PathBuf,
    state: Mutex<Shared>,
    /// Shared with the scheduler, so a pool thread stops pulling chunks.
    abort: Arc<Abort>,
    /// Workers the pool actually spawned: `--jobs` asks, the memory trim
    /// answers, and `.streaming` reports what is running.
    workers: AtomicUsize,
    /// Has a failed status/README publish already been reported?  One line per
    /// outage, not one per tick.
    publish_warned: AtomicBool,
    /// Has this run truncated an artifact of its own yet?  Until it has, a
    /// failure must leave the folder as it found it.
    artifacts: AtomicBool,
}

impl StreamRun {
    fn new(dir: PathBuf, facts: ReadmeFacts, jobs: usize) -> Self {
        Self {
            dir,
            state: Mutex::new(Shared { facts, status: StreamStatus::new(jobs) }),
            abort: Arc::new(Abort::default()),
            workers: AtomicUsize::new(jobs),
            publish_warned: AtomicBool::new(false),
            artifacts: AtomicBool::new(false),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Shared> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The facts and the status as a reader sees them — `jobs` reporting the
    /// workers the pool really spawned rather than the number asked for.
    fn snapshot(&self) -> (ReadmeFacts, StreamStatus) {
        let shared = self.lock();
        let mut status = shared.status.clone();
        status.jobs = self.workers.load(Ordering::SeqCst);
        (shared.facts.clone(), status)
    }

    fn publish_status(&self) -> Result<(), String> {
        let text = format!("{}\n", dumps_compact(&self.snapshot().1.to_json()));
        write_atomic(&self.dir.join(STATUS_FILE), text.as_bytes())
    }

    fn publish_readme(&self) -> Result<(), String> {
        let (facts, status) = self.snapshot();
        write_atomic(
            &self.dir.join(README_FILE),
            render_readme_streaming(&facts, &status.progress()).as_bytes(),
        )
    }

    /// The clock's publishes.  `.streaming` and the running README report on the
    /// export rather than being it, so a failed one warns once and is retried on
    /// the next tick instead of ending an otherwise healthy run — the `.c`, the
    /// `index.jsonl` and the `.h` are the export, and those stay fatal.
    fn publish_progress(&self, readme: bool) {
        let published = self
            .publish_status()
            .and_then(|()| if readme { self.publish_readme() } else { Ok(()) });
        match published {
            Ok(()) => self.publish_warned.store(false, Ordering::SeqCst),
            Err(e) => {
                if !self.publish_warned.swap(true, Ordering::SeqCst) {
                    eprintln!("[kuna --stream] warning: {e}; retrying on the next tick");
                }
            }
        }
    }

    fn set_asm_phase(&self, phase: AsmPhase) {
        self.lock().status.asm = phase;
    }

    fn fail(&self, error: &str) {
        let mut shared = self.lock();
        shared.status.phase = StreamPhase::Failed;
        shared.status.error = Some(error.to_string());
    }
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Replace `path` in one step: a reader polling the status file or the README
/// must never see a half-written one.  The temp name carries this process's pid
/// so two exports into one folder cannot fight over it.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let name = path.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let temp = path.with_file_name(format!("{name}.kuna-tmp.{}", std::process::id()));
    std::fs::write(&temp, bytes).map_err(|e| format!("cannot write {}: {e}", temp.display()))?;
    std::fs::rename(&temp, path)
        .map_err(|e| format!("cannot replace {}: {e}", path.display()))
}

/// Refuse to stream into a folder another live export is streaming into: the
/// second run truncates the first's `.c` and `index.jsonl` mid-flight, which
/// silently invalidates every offset a reader has taken from them.
fn refuse_a_live_export(dir: &Path) -> Result<(), String> {
    // /proc is how liveness is checked, so elsewhere the folder is the user's
    // to manage.
    if !cfg!(target_os = "linux") {
        return Ok(());
    }
    let Ok(status) = std::fs::read_to_string(dir.join(STATUS_FILE)) else { return Ok(()) };
    let Some(pid) = json_number(&status, "pid") else { return Ok(()) };
    if pid == u64::from(std::process::id()) || !Path::new(&format!("/proc/{pid}")).exists() {
        return Ok(());
    }
    Err(format!(
        "a streamed export (pid {pid}) is already writing {}: two exports into one folder \
         truncate each other's .c. Wait for it, or pass -o to export somewhere else.",
        dir.display()
    ))
}

/// The value of a compact-JSON number field, for the one field this module has
/// to read back out of a status file it did not write.
fn json_number(line: &str, key: &str) -> Option<u64> {
    let at = line.find(&format!("\"{key}\":"))? + key.len() + 3;
    let digits: String = line[at..].chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

// --- the scheduler -----------------------------------------------------------

/// The dynamic, result-steered work order that replaces the static
/// longest-first plan in stream mode.
///
/// A function's callee hints ([`FuncResult::callee_hints`]) come back with its
/// result, so the frontier deepens as the export runs: the seeds first, then
/// what they call, and only then the rest of the target list in address order.
/// Hints are intersected with the RESOLVED TARGETS, never the inventory — an
/// `--addr`/`--functions` export must not expand to callees it was not asked for.
pub(crate) struct Scheduler {
    state: Mutex<SchedState>,
    by_addr: BTreeMap<u64, usize>,
    total: usize,
    jobs: usize,
    chunk: Option<usize>,
    abort: Arc<Abort>,
}

struct SchedState {
    frontier: VecDeque<usize>,
    /// Is this target already sitting in the frontier?  Without it a function
    /// with many callers is queued once per caller, which inflates the frontier
    /// length the chunk size reads and makes a drained frontier look deep.
    queued: Vec<bool>,
    cursor: usize,
    claimed: Vec<bool>,
    claimed_count: usize,
}

/// The frontier is followed closely while it is thin, so the order stays
/// entry-point-first; the address-order fallback is the bulk of a large export
/// and goes out in full chunks, because every chunk costs a spec write, a pipe
/// assignment, a result file and an ack.
const FRONTIER_CHUNK_THIN: usize = 4;
const CHUNK_MAX: usize = 64;

impl Scheduler {
    fn new(specs: &[TargetSpec], jobs: usize, chunk: Option<usize>, abort: Arc<Abort>) -> Self {
        let mut by_addr = BTreeMap::new();
        for (i, spec) in specs.iter().enumerate() {
            by_addr.entry(spec.addr).or_insert(i);
        }
        Self {
            state: Mutex::new(SchedState {
                frontier: VecDeque::new(),
                queued: vec![false; specs.len()],
                cursor: 0,
                claimed: vec![false; specs.len()],
                claimed_count: 0,
            }),
            by_addr,
            total: specs.len(),
            jobs: jobs.max(1),
            chunk,
            abort,
        }
    }

    /// The target index at `addr`, if this export selected it.
    fn index_of(&self, addr: u64) -> Option<usize> {
        self.by_addr.get(&addr).copied()
    }

    /// Put `indices` at the head of the frontier, in order.
    fn seed(&self, indices: &[usize]) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        for (n, &i) in indices.iter().enumerate() {
            state.frontier.insert(n, i);
            state.queued[i] = true;
        }
    }

    /// Claim `indices` for a caller that will decompile them itself.
    fn claim(&self, indices: &[usize]) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        for &i in indices {
            if !state.claimed[i] {
                state.claimed[i] = true;
                state.claimed_count += 1;
            }
        }
    }

    /// One target, frontier first — the serial loop's pull.
    fn claim_one(&self) -> Option<usize> {
        if self.abort.stopped() {
            return None;
        }
        self.next_chunk_of(1, 1).map(|c| c[0])
    }

    /// File every callee hint that names an unclaimed, unqueued target.
    fn on_results(&self, results: &[FuncResult]) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        for r in results {
            for hint in &r.callee_hints {
                let Some(i) = self.by_addr.get(hint).copied() else { continue };
                if !state.claimed[i] && !state.queued[i] {
                    state.queued[i] = true;
                    state.frontier.push_back(i);
                }
            }
        }
    }

    #[cfg(test)]
    fn claimed_all(&self) -> bool {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.claimed_count >= self.total
    }

    #[cfg(test)]
    fn frontier_len(&self) -> usize {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).frontier.len()
    }

    fn next_chunk_of(&self, frontier_k: usize, cursor_k: usize) -> Option<Vec<usize>> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        while out.len() < frontier_k {
            let Some(i) = state.frontier.pop_front() else { break };
            state.queued[i] = false;
            if !state.claimed[i] {
                state.claimed[i] = true;
                state.claimed_count += 1;
                out.push(i);
            }
        }
        // Only when the frontier had nothing left: a short frontier chunk goes
        // out as it is, so the hints a worker is about to return still lead.
        if out.is_empty() {
            while out.len() < cursor_k && state.cursor < self.total {
                let i = state.cursor;
                state.cursor += 1;
                if !state.claimed[i] {
                    state.claimed[i] = true;
                    state.claimed_count += 1;
                    out.push(i);
                }
            }
        }
        (!out.is_empty()).then_some(out)
    }

    /// `(frontier chunk, address-cursor chunk)`.
    fn chunk_sizes(&self) -> (usize, usize) {
        if let Some(fixed) = self.chunk {
            let fixed = fixed.max(1);
            return (fixed, fixed);
        }
        let thin = {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            state.frontier.len() < 8 * self.jobs
        };
        (if thin { FRONTIER_CHUNK_THIN } else { CHUNK_MAX }, CHUNK_MAX)
    }
}

impl ChunkSource for Scheduler {
    fn next_chunk(&self) -> Option<Vec<usize>> {
        // A stopped run hands out no more work: each pool thread finishes the
        // chunk it is on and leaves.
        if self.abort.stopped() {
            return None;
        }
        let (frontier, cursor) = self.chunk_sizes();
        self.next_chunk_of(frontier, cursor)
    }
}

// --- the writer --------------------------------------------------------------

/// Owns `<name>.c`, `<name>.h`, `index.jsonl`, `README.md` and `.streaming`.
///
/// A function's `.c` block is appended with ONE write and only then announced in
/// `index.jsonl`, which is what makes a line there the proof its block is whole.
/// Blocks are never rewritten, so an offset a reader took stays valid; the two
/// whole-file artifacts are replaced atomically.
struct ProjectWriter {
    dir: PathBuf,
    file_name: String,
    prelude: String,
    c: File,
    index: File,
    c_bytes: u64,
    seq: usize,
    /// One per finished target, with the code dropped — what the `.h`, the
    /// `.asm` variables section and the final README are built from.
    records: Vec<FuncResult>,
    errors: Vec<String>,
    dat: BTreeSet<u64>,
    protos: usize,
    protos_at_last_header: usize,
    last_status: Instant,
    last_header: Instant,
    last_readme: Instant,
    last_readme_done: usize,
}

impl ProjectWriter {
    /// Truncate and create the streamed artifacts.  Called only once the load,
    /// the language check and the target set have all succeeded — before that a
    /// previous export's artifacts are still the best answer in the folder.
    fn create(layout: &Layout, prelude: String) -> Result<Self, String> {
        let dir = layout.out_dir.clone();
        let file_name = layout.file_name.clone();
        let mut c = create(&dir.join(format!("{file_name}.c")))?;
        c.write_all(format!("#include \"{file_name}.h\"\n\n").as_bytes())
            .map_err(|e| format!("cannot write {file_name}.c: {e}"))?;
        let c_bytes = format!("#include \"{file_name}.h\"\n\n").len() as u64;
        let index = create(&dir.join(INDEX_FILE))?;
        let now = Instant::now();
        let writer = Self {
            dir,
            file_name,
            prelude,
            c,
            index,
            c_bytes,
            seq: 0,
            records: Vec::new(),
            errors: Vec::new(),
            dat: BTreeSet::new(),
            protos: 0,
            protos_at_last_header: 0,
            last_status: now,
            last_header: now,
            last_readme: now,
            last_readme_done: 0,
        };
        writer.write_header(None)?;
        Ok(writer)
    }

    /// Append one finished function: its `.c` block, then its `index.jsonl`
    /// line, then the compact record the tail artifacts are built from.
    fn append(&mut self, mut r: FuncResult, run: &StreamRun) -> Result<(), String> {
        let block = render_c(std::slice::from_ref(&r));
        self.c
            .write_all(block.as_bytes())
            .map_err(|e| format!("cannot append to {}.c: {e}", self.file_name))?;
        let offset = self.c_bytes;
        self.c_bytes += block.len() as u64;
        self.dat.extend(collect_dat_addrs(std::slice::from_ref(&r)));
        let line = index_line(self.seq, &r, offset, block.len());
        self.index
            .write_all(line.as_bytes())
            .map_err(|e| format!("cannot append to {INDEX_FILE}: {e}"))?;
        self.seq += 1;

        if let Some(error) = &r.error {
            self.errors.push(error.clone());
        }
        if r.proto.is_some() {
            self.protos += 1;
        }
        let failed = r.error.is_some();
        r.code = None;
        r.line_mappings.clear();
        r.aliases.clear();
        r.callee_hints.clear();
        r.object_location = None;
        self.records.push(r);

        let mut shared = run.lock();
        shared.status.done += 1;
        shared.status.failed += usize::from(failed);
        shared.status.c_bytes = self.c_bytes;
        shared.status.last_result = Instant::now();
        Ok(())
    }

    /// The clock-driven half — driven by TIME, not by results, so a run whose
    /// functions land in bursts still reports at a steady rate: the status file
    /// every tick, the README when the counts have moved, the `.h` on a doubling
    /// back-off.
    fn tick(&mut self, run: &StreamRun) -> Result<(), String> {
        if self.last_status.elapsed() < TICK {
            return Ok(());
        }
        let done = run.lock().status.done;
        let readme = done != self.last_readme_done && self.last_readme.elapsed() >= README_EVERY;
        run.publish_progress(readme);
        self.last_status = Instant::now();
        if readme {
            self.last_readme = Instant::now();
            self.last_readme_done = done;
        }
        let doubled = self.protos >= self.protos_at_last_header.saturating_mul(2).max(1);
        let since = self.last_header.elapsed();
        if since >= HEADER_MIN && (doubled || since >= HEADER_MAX) {
            self.write_header(None)?;
            self.protos_at_last_header = self.protos;
            self.last_header = Instant::now();
        }
        Ok(())
    }

    /// `<name>.h`.  `types` is `None` until the run finishes: the user-defined
    /// type block is a whole-program artifact the decompile itself produces, so
    /// a mid-run `.h` says so rather than declaring half of it.
    fn write_header(&self, types: Option<&str>) -> Result<(), String> {
        let types =
            types.unwrap_or("/* pending until the export completes */\n");
        let text = build_header(&self.file_name, &self.prelude, types, &self.records);
        write_atomic(&self.dir.join(format!("{}.h", self.file_name)), text.as_bytes())
    }
}

fn create(path: &Path) -> Result<File, String> {
    File::create(path).map_err(|e| format!("cannot create {}: {e}", path.display()))
}

/// One `index.jsonl` line: what the function is, and the exact span of the `.c`
/// its block occupies.
fn index_line(seq: usize, r: &FuncResult, offset: u64, len: usize) -> String {
    let num = |v: u64| Json::Number(v.to_string());
    let line = Json::Object(vec![
        ("seq".into(), num(seq as u64)),
        ("name".into(), Json::Str(r.name.clone())),
        ("addr".into(), Json::Str(format!("0x{:x}", r.address))),
        ("size".into(), Json::Number(r.size.to_string())),
        ("c_offset".into(), num(offset)),
        ("c_len".into(), num(len as u64)),
        ("error".into(), r.error.clone().map_or(Json::Null, Json::Str)),
    ]);
    format!("{}\n", dumps_compact(&line))
}

// --- the export --------------------------------------------------------------

/// `kuna decompile-project --stream`: the whole timeline.
pub(crate) fn run(args: &Args, output: Option<&str>) -> Result<String, String> {
    let layout = Layout::resolve(args, output)?;
    refuse_a_live_export(&layout.out_dir)?;
    // A previous export's README is the only file in the folder that says which
    // binary the artifacts around it describe.  This run replaces it at t=0 and
    // puts it back byte for byte if it fails before truncating anything.
    let prior_readme = std::fs::read(layout.out_dir.join(README_FILE)).ok();
    std::fs::create_dir_all(&layout.out_dir)
        .map_err(|e| format!("cannot create {}: {e}", layout.out_dir.display()))?;
    let run = StreamRun::new(
        layout.out_dir.clone(),
        ReadmeFacts::pending(&layout.binary_path, &layout.path_label, &layout.file_name),
        args.jobs.max(1),
    );

    // The two t=0 writes are fatal: they are the proof the folder can be written
    // at all, and there is no next tick to retry them on yet.
    let opened = run.publish_status().and_then(|()| run.publish_readme());
    match opened.and_then(|()| export(args, &layout, &run)) {
        Ok(summary) => Ok(summary),
        Err(error) => {
            run.fail(&error);
            let _ = run.publish_status();
            match prior_readme {
                Some(bytes) if !run.artifacts.load(Ordering::SeqCst) => {
                    let _ = write_atomic(&layout.out_dir.join(README_FILE), &bytes);
                }
                _ => {
                    let _ = run.publish_readme();
                }
            }
            Err(error)
        }
    }
}

fn export(args: &Args, layout: &Layout, run: &StreamRun) -> Result<String, String> {
    let load_started = Instant::now();
    let mut prog = load_with_heartbeat(args, run)?;
    let load_seconds = load_started.elapsed().as_secs_f64();
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget = Some(Duration::from_secs(args.max_fn_seconds));
    }
    if prog.arch().print().get_name() != "c-language" {
        return Err(format!(
            "project export is C-only in this release (got {}); use `kuna decompile` or \
             `kuna decompile-all --json` for other output languages",
            prog.arch().print().get_name()
        ));
    }
    let mut targets = resolve_targets(&prog, args)?;
    if targets.is_empty() {
        return Err(format!("no functions selected/discovered in {}", args.binary));
    }
    // The scheduler's fallback cursor, the `.asm` labels and the reconciliation
    // all read one order, so the target list is sorted once here.
    targets.sort_by(|a, b| {
        a.addr.get_offset().cmp(&b.addr.get_offset()).then_with(|| a.name.cmp(&b.name))
    });
    let specs = flatten_targets(&targets);
    if args.jobs <= 1 && targets.len() > SERIAL_HINT_MIN {
        eprintln!("[kuna --stream] serial run; --jobs auto uses every core");
    }

    let scheduler = Scheduler::new(&specs, args.jobs, args.jobs_chunk, Arc::clone(&run.abort));
    let seeds = seed_indices(&prog, args, &scheduler);
    if seeds.is_empty() {
        eprintln!(
            "[kuna --stream] warning: neither the image entry point nor `main` is in this \
             export's target set, so functions are written in address order."
        );
    }
    scheduler.seed(&seeds);

    let prelude = print_c_recompile_prelude(prog.arch());
    let mut sweep =
        AsmSweep::new(&prog, &layout.file_name, asm_labels_from_entries(&prog, &targets));
    // An image with no CODE section has nothing to sweep, so its `.asm` is final
    // before the first function is decompiled: `pending` means the sweep has not
    // started, and an agent waiting for `complete` must not wait for the export.
    if sweep.is_done() {
        run.set_asm_phase(AsmPhase::Complete);
    }
    // From here the folder is this run's: everything below truncates.
    run.artifacts.store(true, Ordering::SeqCst);
    let mut asm = BufWriter::new(create(&layout.out_dir.join(format!("{}.asm", layout.file_name)))?);
    asm.write_all(sweep.header_lines().as_bytes())
        .map_err(|e| format!("cannot write {}.asm: {e}", layout.file_name))?;
    let writer = ProjectWriter::create(layout, prelude.clone())?;
    {
        let mut shared = run.lock();
        shared.facts = ReadmeFacts::snapshot(
            &layout.binary_path,
            &layout.path_label,
            &layout.file_name,
            &prog,
        );
        shared.status.phase = StreamPhase::Decompiling;
        shared.status.total = Some(targets.len());
        shared.status.seeds = seeds.len();
        shared.status.last_result = Instant::now();
    }
    run.publish_progress(true);

    let (tx, rx) = mpsc::channel::<Vec<FuncResult>>();
    let opts = DecompileOptions {
        no_vars: false,
        want_proto: true,
        want_provenance: false,
        want_callee_hints: true,
        // A streamed run is never a single-target run: `--stream` and `--assert`
        // are exclusive precisely so an unqualified directive cannot bind to
        // every function of the export.
        single_target: false,
    };

    let (mut writer, mut type_blocks) = std::thread::scope(
        |scope| -> Result<(ProjectWriter, Vec<String>), String> {
            let writing = scope.spawn(move || writer_loop(writer, rx, run));
            let produced = if args.jobs > 1 {
                let pooled = run_pooled(
                    args, layout, &mut prog, &targets, &specs, &scheduler, &seeds, &opts, &tx,
                    load_seconds, &mut sweep, &mut asm, run, scope,
                );
                drop(tx);
                pooled
            } else {
                let serial = run_serial(
                    &mut prog, &targets, &scheduler, &opts, &tx, &mut sweep, &mut asm, run,
                );
                drop(tx);
                serial.map(|()| Vec::new())
            };
            // The writer owns every artifact, so ITS error is why the run
            // stopped; a producer that only saw its channel close carries the
            // sentinel, and the writer is joined before either is propagated.
            let written = writing.join().map_err(|_| "the streaming writer panicked".to_string())?;
            match (produced, written) {
                (Ok(blocks), Ok(writer)) => Ok((writer, blocks)),
                (_, Err(writer)) => Err(writer),
                (Err(produced), Ok(_)) => Err(produced),
            }
        },
    )?;

    // Reconcile: every target owns exactly one record.  A chunk lost to a dead
    // worker already comes back as `error` records, so this only ever fires on a
    // scheduling gap — and it is written like any other function.  The
    // `finalizing` status goes out after them, so the last counts a reader sees
    // agree with README.md and index.jsonl.
    let seen: BTreeSet<u64> = writer.records.iter().map(|r| r.byte_address).collect();
    for spec in specs.iter().filter(|s| !seen.contains(&s.addr)) {
        writer.append(jobs::missing_result(spec), run)?;
    }
    run.lock().status.phase = StreamPhase::Finalizing;
    run.publish_progress(false);
    jobs::warn_about_streamed_anomalies(&writer.errors, args.max_fn_seconds);

    // The parent is one more type shard: at `--jobs N` it decompiled the seeds
    // itself, and whatever those interned lives only in its own factory.
    type_blocks.push(print_c_types(prog.arch_mut()));
    let types = jobs::merge_type_definitions(&type_blocks, jobs::STREAM_TAG);
    writer.write_header(Some(&types))?;
    asm.write_all(render_variables_section(&writer.records).as_bytes())
        .and_then(|()| asm.write_all(render_data_tail(&prog, &writer.dat).as_bytes()))
        .and_then(|()| asm.flush())
        .map_err(|e| format!("cannot write {}.asm: {e}", layout.file_name))?;

    let counts = ReadmeCounts::of(&writer.records);
    let facts = run.lock().facts.clone();
    write_atomic(
        &layout.out_dir.join(README_FILE),
        render_readme(&facts, Some(counts), ReadmeLayout::Streamed).as_bytes(),
    )?;
    // A `.streaming` left behind means "killed" to every reader, so failing to
    // remove it is a failed export, not a detail.
    let status = layout.out_dir.join(STATUS_FILE);
    std::fs::remove_file(&status)
        .map_err(|e| format!("cannot remove {}: {e}", status.display()))?;

    let files = [
        format!("{}.c", layout.file_name),
        format!("{}.h", layout.file_name),
        format!("{}.asm", layout.file_name),
        README_FILE.to_string(),
        INDEX_FILE.to_string(),
    ]
    .iter()
    .map(|name| {
        let size = std::fs::metadata(layout.out_dir.join(name)).map(|m| m.len()).unwrap_or(0);
        format!("{name} ({size} bytes)")
    })
    .collect::<Vec<_>>()
    .join(", ");
    Ok(format!(
        "wrote {}: {files}; functions: {} ok, {} failed\n",
        layout.out_dir.display(),
        counts.ok,
        counts.failed
    ))
}

/// Load the program while the status file keeps ticking.
///
/// The load is the longest single wait in a large export — 96 s on the 147 MB
/// image this flag was built for — and it happens before any other thread
/// exists.  Without a clock, `.streaming` holds `elapsed_s: 0` and
/// `seconds_since_last_result: 0` with a live pid for all of it, which is
/// exactly what a hung process looks like to a poller.
fn load_with_heartbeat(args: &Args, run: &StreamRun) -> Result<ConsoleProgram, String> {
    /// Stops the clock however the load ends — a panicking load would otherwise
    /// leave the thread spinning and the scope waiting on it forever.
    struct Stopper<'a>(&'a AtomicBool);
    impl Drop for Stopper<'_> {
        fn drop(&mut self) {
            self.0.store(false, Ordering::SeqCst);
        }
    }

    let loading = AtomicBool::new(true);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let mut last_status = Instant::now();
            let mut last_readme = Instant::now();
            while loading.load(Ordering::SeqCst) {
                std::thread::sleep(CLOCK_GRANULARITY);
                if !loading.load(Ordering::SeqCst) || last_status.elapsed() < TICK {
                    continue;
                }
                let readme = last_readme.elapsed() >= README_EVERY;
                run.publish_progress(readme);
                last_status = Instant::now();
                if readme {
                    last_readme = Instant::now();
                }
            }
        });
        let _stop = Stopper(&loading);
        load_program(args, DriverDefaults::Decompile)
    })
}

/// The image entry point, then `main` — the two starting points an export has
/// without a call graph, each taken only when it is one of THIS run's targets.
///
/// A headerless image has no entry in its metadata, so there the caller's own
/// `--entry`/`--addr` are the entry points it named.
fn seed_indices(prog: &ConsoleProgram, args: &Args, scheduler: &Scheduler) -> Vec<usize> {
    let entry = prog.image_metadata().and_then(|m| m.entry);
    let mut addrs: Vec<u64> = match entry {
        Some(vma) => vec![vma],
        None => args
            .addrs
            .iter()
            .filter_map(|selector| match selector {
                EntrySelector::Numeric(vma) if args.raw_image => prog.input_code_offset(*vma).ok(),
                EntrySelector::Numeric(vma) => Some(*vma),
                _ => None,
            })
            .collect(),
    };
    addrs = addrs
        .into_iter()
        .map(|vma| prog.find_entry_at(vma).map_or(vma, |e| e.addr.get_offset()))
        .collect();
    if let Some(named) = prog.find_entry_by_name("main") {
        addrs.push(named.addr.get_offset());
    }
    let mut seeds: Vec<usize> = Vec::new();
    for addr in addrs {
        if let Some(i) = scheduler.index_of(addr) {
            if !seeds.contains(&i) {
                seeds.push(i);
            }
        }
    }
    seeds
}

/// The writer thread: drain results as they land, and keep the whole-file
/// artifacts current on the clock even while nothing lands.
fn writer_loop(
    mut writer: ProjectWriter,
    rx: mpsc::Receiver<Vec<FuncResult>>,
    run: &StreamRun,
) -> Result<ProjectWriter, String> {
    loop {
        match rx.recv_timeout(TICK) {
            Ok(batch) => {
                for r in batch {
                    writer.append(r, run).map_err(|e| writer_died(run, e))?;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
        writer.tick(run).map_err(|e| writer_died(run, e))?;
    }
    run.publish_progress(false);
    Ok(writer)
}

/// The writer reports its own death: it is the only thread that writes
/// `.streaming`, so the status file has to say why before it stops — a poller
/// would otherwise read a frozen-but-healthy export for as long as the
/// producers took to notice.
fn writer_died(run: &StreamRun, error: String) -> String {
    run.abort.stop(Some(error.clone()));
    run.fail(&error);
    let _ = run.publish_status();
    error
}

/// `--jobs N`: the seeds in-process, then the worker pool on its own thread
/// while the main thread sweeps the `.asm`.
#[allow(clippy::too_many_arguments)]
fn run_pooled<'scope, 'env>(
    args: &'env Args,
    layout: &'env Layout,
    prog: &mut ConsoleProgram,
    targets: &[FunctionEntry],
    specs: &'env [TargetSpec],
    scheduler: &'env Scheduler,
    seeds: &[usize],
    opts: &DecompileOptions,
    tx: &mpsc::Sender<Vec<FuncResult>>,
    load_seconds: f64,
    sweep: &mut AsmSweep,
    asm: &mut BufWriter<File>,
    run: &'env StreamRun,
    scope: &'scope std::thread::Scope<'scope, 'env>,
) -> Result<Vec<String>, String> {
    decompile_in_process(prog, seeds, targets, scheduler, opts, tx, run)?;

    let cfg = pool_config(
        args,
        /* want_proto= */ true,
        /* want_provenance= */ false,
        /* want_types= */ true,
        /* want_callee_hints= */ true,
        load_seconds,
    );
    let inventory = flatten_targets(&prog.function_entries_canonical());
    let send = Mutex::new(tx.clone());
    let done_base = seeds.len();
    let pool = scope.spawn(move || {
        let sink = |_indices: &[usize], produced: Vec<FuncResult>| {
            scheduler.on_results(&produced);
            if send.lock().unwrap_or_else(|e| e.into_inner()).send(produced).is_err() {
                run.abort.stop(None);
            }
        };
        jobs::run_pool_streaming(
            &cfg,
            specs,
            &inventory,
            scheduler,
            done_base,
            &run.workers,
            &sink,
        )
    });

    let swept = sweep_to_end(prog, sweep, asm, run);
    let types = pool.join().map_err(|_| "the --stream worker pool panicked".to_string())?;
    swept.map_err(|e| format!("cannot write {}.asm: {e}", layout.file_name))?;
    types
}

/// `--jobs 1`: one thread alternating decompile batches with sweep steps, so
/// the `.c` starts after the first function and the `.asm` still completes
/// early instead of at the end of the run.
#[allow(clippy::too_many_arguments)]
fn run_serial(
    prog: &mut ConsoleProgram,
    targets: &[FunctionEntry],
    scheduler: &Scheduler,
    opts: &DecompileOptions,
    tx: &mpsc::Sender<Vec<FuncResult>>,
    sweep: &mut AsmSweep,
    asm: &mut BufWriter<File>,
    run: &StreamRun,
) -> Result<(), String> {
    let step = (code_bytes(prog) / SWEEP_STEPS).max(MIN_SWEEP_STEP_BYTES);
    if sweep.is_done() {
        run.set_asm_phase(AsmPhase::Complete);
    }
    let mut batch = 1usize;
    loop {
        // The callee-hint context is built once per `decompile_pulled` call, so
        // the interleave is batched rather than per function, and once the sweep
        // is out of the way the rest of the run is ONE call.
        let limit = if sweep.is_done() { usize::MAX } else { batch };
        let mut taken = 0usize;
        decompile_pulled(
            prog,
            opts,
            &mut || {
                if taken >= limit || run.abort.stopped() {
                    return None;
                }
                scheduler.claim_one().map(|i| {
                    taken += 1;
                    targets[i].clone()
                })
            },
            &mut |r| {
                scheduler.on_results(std::slice::from_ref(&r));
                if tx.send(vec![r]).is_err() {
                    run.abort.stop(None);
                }
            },
        );
        if run.abort.stopped() {
            return Err(run.abort.reason());
        }
        if !sweep.is_done() {
            run.set_asm_phase(AsmPhase::Sweeping);
            sweep
                .step(prog, step, asm)
                .and_then(|_| asm.flush())
                .map_err(|e| format!("cannot write the disassembly: {e}"))?;
            if sweep.is_done() {
                run.set_asm_phase(AsmPhase::Complete);
            }
        }
        if taken == 0 && sweep.is_done() {
            return Ok(());
        }
        if taken > 0 {
            batch = (batch * SERIAL_BATCH_GROWTH).min(SERIAL_BATCH_MAX);
        }
    }
}

/// Decompile `indices` on this thread, claiming them first so the pool never
/// repeats them, and file their hints as the scheduler's opening frontier.
fn decompile_in_process(
    prog: &mut ConsoleProgram,
    indices: &[usize],
    targets: &[FunctionEntry],
    scheduler: &Scheduler,
    opts: &DecompileOptions,
    tx: &mpsc::Sender<Vec<FuncResult>>,
    run: &StreamRun,
) -> Result<(), String> {
    if indices.is_empty() {
        return Ok(());
    }
    scheduler.claim(indices);
    let mut pending = indices.iter().map(|&i| targets[i].clone()).collect::<Vec<_>>().into_iter();
    decompile_pulled(
        prog,
        opts,
        &mut || if run.abort.stopped() { None } else { pending.next() },
        &mut |r| {
            scheduler.on_results(std::slice::from_ref(&r));
            if tx.send(vec![r]).is_err() {
                run.abort.stop(None);
            }
        },
    );
    if run.abort.stopped() {
        return Err(run.abort.reason());
    }
    Ok(())
}

fn sweep_to_end(
    prog: &ConsoleProgram,
    sweep: &mut AsmSweep,
    asm: &mut BufWriter<File>,
    run: &StreamRun,
) -> std::io::Result<()> {
    if sweep.is_done() {
        run.set_asm_phase(AsmPhase::Complete);
        return Ok(());
    }
    run.set_asm_phase(AsmPhase::Sweeping);
    while !sweep.is_done() {
        // The run is over: a finished `.asm` is no use to an export that is
        // about to report why it stopped.
        if run.abort.stopped() {
            return Ok(());
        }
        sweep.step(prog, SWEEP_STEP_BYTES, asm)?;
        asm.flush()?;
    }
    run.set_asm_phase(AsmPhase::Complete);
    Ok(())
}

fn code_bytes(prog: &ConsoleProgram) -> u64 {
    prog.sections()
        .iter()
        .filter(|(_, _, flags)| flags & section_flags::CODE != 0)
        .map(|&(_, size, _)| size)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_console::engine::EntryProvenance;

    fn spec(addr: u64) -> TargetSpec {
        TargetSpec {
            addr,
            space: "ram".into(),
            name: format!("sub_{addr:x}"),
            aliases: Vec::new(),
            size: 0x20,
            object_location: None,
            provenance: EntryProvenance::Mapped,
            binding: None,
        }
    }

    fn result(addr: u64, hints: Vec<u64>) -> FuncResult {
        FuncResult {
            name: format!("sub_{addr:x}"),
            address: addr,
            byte_address: addr,
            size: 0x20,
            code: Some(format!("void sub_{addr:x}(void)\n{{\n  return;\n}}\n")),
            error: None,
            proto: Some(format!("void sub_{addr:x}(void);")),
            variables: Vec::new(),
            line_mappings: Vec::new(),
            aliases: Vec::new(),
            object_location: None,
            callee_hints: hints,
        }
    }

    fn specs(n: u64) -> Vec<TargetSpec> {
        (0..n).map(|i| spec(0x1000 + i * 0x20)).collect()
    }

    fn scheduler(specs: &[TargetSpec], jobs: usize, chunk: Option<usize>) -> Scheduler {
        Scheduler::new(specs, jobs, chunk, Arc::new(Abort::default()))
    }

    /// The contract the `.c`'s order rests on: seeds first, then what they
    /// reach, and only then the address-ordered remainder.
    #[test]
    fn the_scheduler_runs_the_frontier_before_the_address_cursor() {
        let specs = specs(8);
        let sched = scheduler(&specs, 1, Some(1));
        sched.seed(&[5]);
        assert_eq!(sched.claim_one(), Some(5), "a seed leads");

        // sub_10a0 (index 5) calls index 7 and index 1, in that order.
        sched.on_results(&[result(specs[5].addr, vec![specs[7].addr, specs[1].addr])]);
        assert_eq!(sched.claim_one(), Some(7));
        assert_eq!(sched.claim_one(), Some(1));
        // Frontier drained: the fallback resumes at the bottom of the address
        // order, skipping what the frontier already took.
        assert_eq!(sched.claim_one(), Some(0));
        assert_eq!(sched.claim_one(), Some(2));
    }

    /// Every target exactly once, whatever the hints claim.
    #[test]
    fn every_target_is_claimed_exactly_once() {
        let specs = specs(40);
        let sched = scheduler(&specs, 4, None);
        sched.seed(&[3, 9]);
        let mut seen: Vec<usize> = Vec::new();
        while let Some(chunk) = sched.next_chunk() {
            // Re-announce everything as a callee of everything: a hint that
            // names a claimed target must not produce a second claim.
            let hints: Vec<u64> = specs.iter().map(|s| s.addr).collect();
            for &i in &chunk {
                sched.on_results(&[result(specs[i].addr, hints.clone())]);
            }
            seen.extend(chunk);
        }
        let mut sorted = seen.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..40).collect::<Vec<_>>());
        assert_eq!(seen.len(), 40, "a target was handed out twice");
        assert!(sched.claimed_all());
        assert_eq!(seen[0], 3, "the seeds still lead");
        assert_eq!(seen[1], 9);
    }

    /// A hint to something this export did not select is scheduling noise, not
    /// a reason to decompile it — the `--addr`/`--functions` guarantee.
    #[test]
    fn a_hint_outside_the_target_set_is_ignored() {
        let specs = specs(3);
        let sched = scheduler(&specs, 1, Some(4));
        sched.on_results(&[result(specs[0].addr, vec![0xdead_beef, specs[2].addr])]);
        assert_eq!(sched.next_chunk(), Some(vec![2]));
        assert_eq!(sched.next_chunk(), Some(vec![0, 1]));
        assert_eq!(sched.next_chunk(), None);
    }

    /// `--jobs-chunk N` pins the size; otherwise it follows how much frontier
    /// there is to follow — and the address-order fallback, which is where most
    /// of a large export runs, takes full chunks whatever the frontier says.
    #[test]
    fn the_chunk_size_is_fixed_by_the_flag_and_adaptive_without_it() {
        let specs = specs(600);
        let fixed = scheduler(&specs, 4, Some(7));
        assert_eq!(fixed.next_chunk().unwrap().len(), 7);
        assert_eq!(fixed.chunk_sizes(), (7, 7), "--jobs-chunk pins both halves");

        let auto = scheduler(&specs, 4, None);
        assert_eq!(auto.chunk_sizes(), (4, CHUNK_MAX), "a thin frontier is followed closely");
        assert_eq!(
            auto.next_chunk().unwrap().len(),
            CHUNK_MAX,
            "an empty frontier IS the fallback: the cursor goes out in full chunks"
        );
        let hints: Vec<u64> = specs[100..300].iter().map(|s| s.addr).collect();
        auto.on_results(&[result(0x1, hints)]);
        assert_eq!(auto.chunk_sizes(), (CHUNK_MAX, CHUNK_MAX), "a deep frontier batches");
    }

    /// A callee with many callers is queued once: the frontier length decides
    /// the chunk size, so duplicates would make a drained frontier read as deep.
    #[test]
    fn a_repeated_hint_is_queued_once() {
        let specs = specs(8);
        let sched = scheduler(&specs, 1, Some(1));
        let callee = specs[5].addr;
        for caller in 0..4 {
            sched.on_results(&[result(specs[caller].addr, vec![callee, callee])]);
        }
        assert_eq!(sched.frontier_len(), 1, "one entry per queued target");
        assert_eq!(sched.claim_one(), Some(5));
        assert_eq!(sched.frontier_len(), 0, "and it leaves when it is claimed");
        assert_eq!(sched.claim_one(), Some(0), "the cursor takes over");
    }

    /// The writer owns every artifact, so when it dies the pool must stop
    /// pulling chunks rather than decompile the rest of the binary into a
    /// closed channel.
    #[test]
    fn an_aborted_run_hands_out_no_more_work() {
        let specs = specs(40);
        let abort = Arc::new(Abort::default());
        let sched = Scheduler::new(&specs, 4, Some(4), Arc::clone(&abort));
        assert_eq!(sched.next_chunk().map(|c| c.len()), Some(4));
        abort.stop(Some("cannot replace x.h: Is a directory (os error 21)".into()));
        assert_eq!(sched.next_chunk(), None, "a stopped run schedules nothing");
        assert_eq!(sched.claim_one(), None);
        assert!(abort.reason().contains("Is a directory"), "the writer's error is the reason");
    }

    /// A producer that only saw its channel close still reports something.
    #[test]
    fn an_abort_without_a_writer_error_reports_the_sentinel() {
        let abort = Abort::default();
        abort.stop(None);
        assert!(abort.stopped());
        assert_eq!(abort.reason(), WRITER_STOPPED);
        abort.stop(Some("second".into()));
        assert_eq!(abort.reason(), "second", "the first real error wins over the sentinel");
        abort.stop(Some("third".into()));
        assert_eq!(abort.reason(), "second", "and is not overwritten by a later one");
    }

    /// Two streamed exports into one folder truncate each other's `.c` and
    /// invalidate every offset a reader took, so the second one refuses while
    /// the first is alive — and takes the folder once it is not.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_second_export_refuses_a_folder_a_live_one_is_writing() {
        let dir = std::env::temp_dir().join(format!("kuna_stream_live_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(refuse_a_live_export(&dir).is_ok(), "an empty folder is free");

        // pid 1 is alive on any Linux this runs on.
        std::fs::write(dir.join(STATUS_FILE), "{\"schema\":1,\"phase\":\"decompiling\",\"pid\":1}\n")
            .unwrap();
        let refused = refuse_a_live_export(&dir).unwrap_err();
        assert!(refused.contains("pid 1"), "{refused}");
        assert!(refused.contains("-o"), "the refusal has to say what to do instead: {refused}");

        std::fs::write(
            dir.join(STATUS_FILE),
            format!("{{\"schema\":1,\"pid\":{}}}\n", std::process::id()),
        )
        .unwrap();
        assert!(refuse_a_live_export(&dir).is_ok(), "our own pid is not a second export");

        // Past `pid_max`, so no process can hold it: the run was killed and the
        // folder is free.
        let dead: u64 = std::fs::read_to_string("/proc/sys/kernel/pid_max")
            .map(|s| s.trim().parse::<u64>().unwrap_or(4_194_304))
            .unwrap_or(4_194_304)
            + 1;
        std::fs::write(dir.join(STATUS_FILE), format!("{{\"pid\":{dead}}}\n")).unwrap();
        assert!(refuse_a_live_export(&dir).is_ok(), "a dead pid does not hold the folder");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The status file's `pid` is what a second export reads to decide whether
    /// the first is still alive.
    #[test]
    fn the_status_pid_is_readable_back_out_of_the_line() {
        let status = StreamStatus::new(2);
        let line = dumps_compact(&status.to_json());
        assert_eq!(json_number(&line, "pid"), Some(u64::from(std::process::id())));
        assert_eq!(json_number(&line, "functions_done"), Some(0));
        assert_eq!(json_number(&line, "functions_total"), None, "a null is not a number");
        assert_eq!(json_number(&line, "nope"), None);
    }

    /// Two exports into one folder must not fight over one temp path.
    #[test]
    fn an_atomic_write_names_its_temp_after_the_process() {
        let dir = std::env::temp_dir().join(format!("kuna_stream_atomic_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("x.h");
        write_atomic(&path, b"one").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"one");
        let leftovers: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| Some(e.ok()?.file_name().to_string_lossy().into_owned()))
            .filter(|n| n.contains("kuna-tmp"))
            .collect();
        assert!(leftovers.is_empty(), "the temp file is renamed away: {leftovers:?}");
        assert!(
            write_atomic(&dir.join("nested/x.h"), b"two").is_err(),
            "a path that cannot be written is an error, not a panic"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The status file is the run's only contract with a polling reader.
    #[test]
    fn the_status_file_carries_the_documented_schema() {
        let mut status = StreamStatus::new(4);
        let json = dumps_compact(&status.to_json());
        for field in [
            "\"schema\":1",
            "\"phase\":\"loading\"",
            "\"functions_total\":null",
            "\"functions_done\":0",
            "\"asm\":\"pending\"",
            "\"error\":null",
        ] {
            assert!(json.contains(field), "{field} missing from {json}");
        }
        status.phase = StreamPhase::Failed;
        status.total = Some(12);
        status.done = 3;
        status.failed = 1;
        status.asm = AsmPhase::Complete;
        status.error = Some("boom \"quoted\"".into());
        let json = dumps_compact(&status.to_json());
        assert!(json.contains("\"phase\":\"failed\""));
        assert!(json.contains("\"functions_total\":12"));
        assert!(json.contains("\"error\":\"boom \\\"quoted\\\"\""));
        assert_eq!(json.lines().count(), 1, "a status file is one line");
    }

    /// An index line has to slice the `.c` to exactly its own block.
    #[test]
    fn an_index_line_slices_the_c_to_its_block() {
        let ok = result(0x401000, Vec::new());
        let block = render_c(std::slice::from_ref(&ok));
        let prelude = "#include \"x.h\"\n\n";
        let c = format!("{prelude}{block}");
        let offset = prelude.len();
        let line = index_line(0, &ok, offset as u64, block.len());
        assert!(line.ends_with('\n'));
        assert!(line.contains("\"seq\":0"));
        assert!(line.contains("\"addr\":\"0x401000\""));
        assert!(line.contains("\"error\":null"));
        assert!(line.contains(&format!("\"c_offset\":{offset}")));
        assert_eq!(&c[offset..offset + block.len()], block);

        let mut bad = ok;
        bad.code = None;
        bad.error = Some("budget exceeded".into());
        assert!(index_line(1, &bad, 0, 0).contains("\"error\":\"budget exceeded\""));
    }
}
