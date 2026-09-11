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
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use kuna_console::engine::{ConsoleProgram, FunctionEntry};
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

/// How long the writer waits for a result before going round its clock again.
const TICK: Duration = Duration::from_millis(500);
/// How often the running README is rewritten, when the counts moved.
const README_EVERY: Duration = Duration::from_secs(5);
/// The `.h` rewrite back-off: never more often than this, and at least this
/// seldom once the prototype count stops doubling.
const HEADER_MIN: Duration = Duration::from_secs(2);
const HEADER_MAX: Duration = Duration::from_secs(30);

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

/// Where the export writes and what it calls itself.
struct Layout {
    binary_path: PathBuf,
    path_label: String,
    file_name: String,
    out_dir: PathBuf,
}

impl Layout {
    /// Resolve the output folder WITHOUT requiring the binary to exist: a
    /// streamed run reports a failed load through its own status file, which it
    /// can only do if it already knows where to write.
    fn resolve(args: &Args, output: Option<&str>) -> Result<Self, String> {
        let binary_path =
            std::fs::canonicalize(&args.binary).unwrap_or_else(|_| PathBuf::from(&args.binary));
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

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Replace `path` in one step: a reader polling the status file or the README
/// must never see a half-written one.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp = path.with_extension("kuna-tmp");
    std::fs::write(&temp, bytes).map_err(|e| format!("cannot write {}: {e}", temp.display()))?;
    std::fs::rename(&temp, path)
        .map_err(|e| format!("cannot replace {}: {e}", path.display()))
}

fn publish_status(dir: &Path, shared: &Mutex<Shared>) -> Result<(), String> {
    let status = shared.lock().unwrap_or_else(|e| e.into_inner()).status.clone();
    write_atomic(&dir.join(STATUS_FILE), format!("{}\n", dumps_compact(&status.to_json())).as_bytes())
}

fn publish_running_readme(dir: &Path, shared: &Mutex<Shared>) -> Result<(), String> {
    let (facts, progress) = {
        let shared = shared.lock().unwrap_or_else(|e| e.into_inner());
        (shared.facts.clone(), shared.status.progress())
    };
    write_atomic(&dir.join("README.md"), render_readme_streaming(&facts, &progress).as_bytes())
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
}

struct SchedState {
    frontier: VecDeque<usize>,
    cursor: usize,
    claimed: Vec<bool>,
    claimed_count: usize,
}

impl Scheduler {
    fn new(specs: &[TargetSpec], jobs: usize, chunk: Option<usize>) -> Self {
        let mut by_addr = BTreeMap::new();
        for (i, spec) in specs.iter().enumerate() {
            by_addr.entry(spec.addr).or_insert(i);
        }
        Self {
            state: Mutex::new(SchedState {
                frontier: VecDeque::new(),
                cursor: 0,
                claimed: vec![false; specs.len()],
                claimed_count: 0,
            }),
            by_addr,
            total: specs.len(),
            jobs: jobs.max(1),
            chunk,
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
        self.next_chunk_of(1).map(|c| c[0])
    }

    /// File every callee hint that names an unclaimed target.
    fn on_results(&self, results: &[FuncResult]) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        for r in results {
            for hint in &r.callee_hints {
                let Some(i) = self.by_addr.get(hint).copied() else { continue };
                if !state.claimed[i] {
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

    fn next_chunk_of(&self, k: usize) -> Option<Vec<usize>> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        while out.len() < k {
            let Some(i) = state.frontier.pop_front() else { break };
            if !state.claimed[i] {
                state.claimed[i] = true;
                state.claimed_count += 1;
                out.push(i);
            }
        }
        // Only when the frontier had nothing left: a short frontier chunk goes
        // out as it is, so the hints a worker is about to return still lead.
        if out.is_empty() {
            while out.len() < k && state.cursor < self.total {
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

    /// Chunk size: small while the frontier is thin, so a deepening frontier is
    /// followed rather than buried under a big address-ordered batch, and large
    /// once there is enough of it (or once the run is down to the fallback).
    fn chunk_size(&self) -> usize {
        if let Some(fixed) = self.chunk {
            return fixed.max(1);
        }
        let thin = {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            state.frontier.len() < 8 * self.jobs
        };
        if thin {
            4
        } else {
            64
        }
    }
}

impl ChunkSource for Scheduler {
    fn next_chunk(&self) -> Option<Vec<usize>> {
        self.next_chunk_of(self.chunk_size())
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
    fn append(&mut self, mut r: FuncResult, shared: &Mutex<Shared>) -> Result<(), String> {
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

        let mut shared = shared.lock().unwrap_or_else(|e| e.into_inner());
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
    fn tick(&mut self, shared: &Mutex<Shared>) -> Result<(), String> {
        if self.last_status.elapsed() < TICK {
            return Ok(());
        }
        publish_status(&self.dir, shared)?;
        self.last_status = Instant::now();
        let done = shared.lock().unwrap_or_else(|e| e.into_inner()).status.done;
        if done != self.last_readme_done && self.last_readme.elapsed() >= README_EVERY {
            publish_running_readme(&self.dir, shared)?;
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
    std::fs::create_dir_all(&layout.out_dir)
        .map_err(|e| format!("cannot create {}: {e}", layout.out_dir.display()))?;
    let shared = Mutex::new(Shared {
        facts: ReadmeFacts::pending(&layout.binary_path, &layout.path_label, &layout.file_name),
        status: StreamStatus::new(args.jobs.max(1)),
    });
    publish_status(&layout.out_dir, &shared)?;
    publish_running_readme(&layout.out_dir, &shared)?;
    if args.jobs <= 1 {
        eprintln!("[kuna --stream] serial run; --jobs auto uses every core");
    }

    match export(args, &layout, &shared) {
        Ok(summary) => Ok(summary),
        Err(error) => {
            {
                let mut shared = shared.lock().unwrap_or_else(|e| e.into_inner());
                shared.status.phase = StreamPhase::Failed;
                shared.status.error = Some(error.clone());
            }
            let _ = publish_status(&layout.out_dir, &shared);
            let _ = publish_running_readme(&layout.out_dir, &shared);
            Err(error)
        }
    }
}

fn export(args: &Args, layout: &Layout, shared: &Mutex<Shared>) -> Result<String, String> {
    let load_started = Instant::now();
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
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

    let scheduler = Scheduler::new(&specs, args.jobs, args.jobs_chunk);
    let seeds = seed_indices(&prog, &scheduler);
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
    let mut asm = BufWriter::new(create(&layout.out_dir.join(format!("{}.asm", layout.file_name)))?);
    asm.write_all(sweep.header_lines().as_bytes())
        .map_err(|e| format!("cannot write {}.asm: {e}", layout.file_name))?;
    let writer = ProjectWriter::create(layout, prelude.clone())?;
    {
        let mut shared = shared.lock().unwrap_or_else(|e| e.into_inner());
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
    publish_status(&layout.out_dir, shared)?;
    publish_running_readme(&layout.out_dir, shared)?;

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

    let (mut writer, types) = std::thread::scope(
        |scope| -> Result<(ProjectWriter, Option<String>), String> {
            let writing = scope.spawn(move || writer_loop(writer, rx, shared));
            let types = if args.jobs > 1 {
                let pooled = run_pooled(
                    args, layout, &mut prog, &targets, &specs, &scheduler, &seeds, &opts, &tx,
                    load_seconds, &mut sweep, &mut asm, shared, scope,
                );
                drop(tx);
                pooled?
            } else {
                let serial = run_serial(
                    &mut prog, &targets, &scheduler, &opts, &tx, &mut sweep, &mut asm, shared,
                );
                drop(tx);
                serial?;
                Some(print_c_types(prog.arch_mut()))
            };
            let writer = writing.join().map_err(|_| "the streaming writer panicked".to_string())??;
            Ok((writer, types))
        },
    )?;

    {
        let mut shared = shared.lock().unwrap_or_else(|e| e.into_inner());
        shared.status.phase = StreamPhase::Finalizing;
    }
    publish_status(&layout.out_dir, shared)?;

    // Reconcile: every target owns exactly one record.  A chunk lost to a dead
    // worker already comes back as `error` records, so this only ever fires on a
    // scheduling gap — and it is written like any other function.
    let seen: BTreeSet<u64> = writer.records.iter().map(|r| r.byte_address).collect();
    for spec in specs.iter().filter(|s| !seen.contains(&s.addr)) {
        writer.append(jobs::missing_result(spec), shared)?;
    }
    jobs::warn_about_streamed_anomalies(&writer.errors, args.max_fn_seconds);

    let types = types.unwrap_or_else(|| print_c_types(prog.arch_mut()));
    writer.write_header(Some(&types))?;
    asm.write_all(render_variables_section(&writer.records).as_bytes())
        .and_then(|()| asm.write_all(render_data_tail(&prog, &writer.dat).as_bytes()))
        .and_then(|()| asm.flush())
        .map_err(|e| format!("cannot write {}.asm: {e}", layout.file_name))?;

    let counts = ReadmeCounts::of(&writer.records);
    let facts = shared.lock().unwrap_or_else(|e| e.into_inner()).facts.clone();
    write_atomic(
        &layout.out_dir.join("README.md"),
        render_readme(&facts, Some(counts), ReadmeLayout::Streamed).as_bytes(),
    )?;
    let _ = std::fs::remove_file(layout.out_dir.join(STATUS_FILE));

    let files = [
        format!("{}.c", layout.file_name),
        format!("{}.h", layout.file_name),
        format!("{}.asm", layout.file_name),
        "README.md".to_string(),
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

/// The image entry point, then `main` — the two starting points an export has
/// without a call graph, each taken only when it is one of THIS run's targets.
fn seed_indices(prog: &ConsoleProgram, scheduler: &Scheduler) -> Vec<usize> {
    let mut seeds: Vec<usize> = Vec::new();
    let entry = prog
        .image_metadata()
        .and_then(|m| m.entry)
        .map(|vma| prog.find_entry_at(vma).map_or(vma, |e| e.addr.get_offset()));
    let named = prog.find_entry_by_name("main").map(|e| e.addr.get_offset());
    for addr in entry.into_iter().chain(named) {
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
    shared: &Mutex<Shared>,
) -> Result<ProjectWriter, String> {
    loop {
        match rx.recv_timeout(TICK) {
            Ok(batch) => {
                for r in batch {
                    writer.append(r, shared)?;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
        writer.tick(shared)?;
    }
    publish_status(&writer.dir, shared)?;
    Ok(writer)
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
    shared: &Mutex<Shared>,
    scope: &'scope std::thread::Scope<'scope, 'env>,
) -> Result<Option<String>, String> {
    decompile_in_process(prog, seeds, targets, scheduler, opts, tx)?;

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
    let pool = scope.spawn(move || {
        let sink = |_indices: &[usize], produced: Vec<FuncResult>| {
            scheduler.on_results(&produced);
            let _ = send.lock().unwrap_or_else(|e| e.into_inner()).send(produced);
        };
        jobs::run_pool_streaming(&cfg, specs, &inventory, scheduler, &sink)
    });

    let swept = sweep_to_end(prog, sweep, asm, shared);
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
    shared: &Mutex<Shared>,
) -> Result<(), String> {
    let step = (code_bytes(prog) / SWEEP_STEPS).max(MIN_SWEEP_STEP_BYTES);
    set_asm_phase(shared, AsmPhase::Sweeping);
    let mut batch = 1usize;
    loop {
        let mut taken = 0usize;
        let mut send_failed = false;
        decompile_pulled(
            prog,
            opts,
            &mut || {
                if taken >= batch {
                    return None;
                }
                scheduler.claim_one().map(|i| {
                    taken += 1;
                    targets[i].clone()
                })
            },
            &mut |r| {
                scheduler.on_results(std::slice::from_ref(&r));
                send_failed |= tx.send(vec![r]).is_err();
            },
        );
        if send_failed {
            return Err("the streaming writer stopped early".into());
        }
        if !sweep.is_done() {
            sweep
                .step(prog, step, asm)
                .and_then(|_| asm.flush())
                .map_err(|e| format!("cannot write the disassembly: {e}"))?;
            if sweep.is_done() {
                set_asm_phase(shared, AsmPhase::Complete);
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
) -> Result<(), String> {
    if indices.is_empty() {
        return Ok(());
    }
    scheduler.claim(indices);
    let mut pending = indices.iter().map(|&i| targets[i].clone()).collect::<Vec<_>>().into_iter();
    let mut send_failed = false;
    decompile_pulled(prog, opts, &mut || pending.next(), &mut |r| {
        scheduler.on_results(std::slice::from_ref(&r));
        send_failed |= tx.send(vec![r]).is_err();
    });
    if send_failed {
        return Err("the streaming writer stopped early".into());
    }
    Ok(())
}

fn sweep_to_end(
    prog: &ConsoleProgram,
    sweep: &mut AsmSweep,
    asm: &mut BufWriter<File>,
    shared: &Mutex<Shared>,
) -> std::io::Result<()> {
    set_asm_phase(shared, AsmPhase::Sweeping);
    while !sweep.is_done() {
        sweep.step(prog, SWEEP_STEP_BYTES, asm)?;
        asm.flush()?;
    }
    set_asm_phase(shared, AsmPhase::Complete);
    Ok(())
}

fn set_asm_phase(shared: &Mutex<Shared>, phase: AsmPhase) {
    shared.lock().unwrap_or_else(|e| e.into_inner()).status.asm = phase;
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

    /// The contract the `.c`'s order rests on: seeds first, then what they
    /// reach, and only then the address-ordered remainder.
    #[test]
    fn the_scheduler_runs_the_frontier_before_the_address_cursor() {
        let specs = specs(8);
        let sched = Scheduler::new(&specs, 1, Some(1));
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
        let sched = Scheduler::new(&specs, 4, None);
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
        let sched = Scheduler::new(&specs, 1, Some(4));
        sched.on_results(&[result(specs[0].addr, vec![0xdead_beef, specs[2].addr])]);
        assert_eq!(sched.next_chunk(), Some(vec![2]));
        assert_eq!(sched.next_chunk(), Some(vec![0, 1]));
        assert_eq!(sched.next_chunk(), None);
    }

    /// `--jobs-chunk N` pins the size; otherwise it follows how much frontier
    /// there is to follow.
    #[test]
    fn the_chunk_size_is_fixed_by_the_flag_and_adaptive_without_it() {
        let specs = specs(600);
        let fixed = Scheduler::new(&specs, 4, Some(7));
        assert_eq!(fixed.next_chunk().unwrap().len(), 7);

        let auto = Scheduler::new(&specs, 4, None);
        assert_eq!(auto.chunk_size(), 4, "a thin frontier is followed closely");
        let hints: Vec<u64> = specs[..200].iter().map(|s| s.addr).collect();
        auto.on_results(&[result(0x1, hints)]);
        assert_eq!(auto.chunk_size(), 64, "a deep frontier batches");
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
