//! `kuna-verify`: differential semantic checking (diffverify, Tier C).
//!
//! ```text
//! kuna-verify check <binary> <function|0xaddr>... | --all  [--jobs N] [--fn-seconds S] [--contract FILE.c] [--option NAME VALUE]...
//! kuna-verify corpus <test.xml|dir>...  [--jobs N] [--file-seconds S] [--baseline FILE] [--save-baseline FILE]
//! kuna-verify localize <test.xml|binary> <function> [--pass N] [--contract FILE.c] [--option NAME VALUE]...
//! kuna-verify step <binary> --regs R,R.. [--flags F,F..] [--states N] [--per-form N] [--limit N] [--canonical]
//! common: [--runs N] [--seed S] [--budget OPS] [--concolic QUERIES] [--json] [--sleighpath DIR]
//! ```
//!
//! `corpus --jobs N` checks N files at a time, each in a worker process of
//! this binary (`shard`) that prints one JSON line, so a file's verdicts do
//! not depend on the files checked before it and a crash costs only its own.
//! A worker still running after `--file-seconds` (1200) is killed and its file
//! is an error, as a crash is.
//! `check --jobs N` splits the functions into fixed chunks and checks N chunks
//! at a time, each in a worker process (`chunk`) that prints a line as each
//! function is done; rows print as they arrive. A function with no verdict
//! after `--fn-seconds` (600) is `UNKNOWN(timeout)`, and the rest of its
//! chunk goes to a fresh worker.
//!
//! Exit status: 1 when a function differs, 3 when a file could not be checked,
//! 2 for a usage error, else 0. With `--baseline`, 1 only for a regression
//! against it: a function that now differs, or a file that now fails.
//! Under GitHub Actions each change is also printed as an annotation.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use kuna_verify::{baseline, check_binary_each, check_datatest, localize, session, Options, Report, Row, Verdict};
use serde_json::{json, Value};

const USAGE: &str = "usage:
  kuna-verify check <binary> <function|0xaddr>... | --all  [--jobs N] [--fn-seconds S] [--contract FILE.c] [--option NAME VALUE]...
  kuna-verify corpus <test.xml|dir>...  [--jobs N] [--file-seconds S] [--baseline FILE] [--save-baseline FILE]
  kuna-verify localize <test.xml|binary> <function> [--pass N] [--contract FILE.c] [--option NAME VALUE]...
  kuna-verify step <binary> --regs R,R.. [--flags F,F..] [--states N] [--per-form N] [--limit N] [--canonical]
  kuna-verify mutate <test.xml|dir>... | <binary> <function|0xaddr>... | --all  [--per-function N (6)] [--jobs N] [--file-seconds S]
options: --runs N (256)  --seed S  --budget OPS (200000)  --concolic QUERIES (0)  --json  --trace  --sleighpath DIR";

struct Args {
    positional: Vec<String>,
    contract: Option<String>,
    options: Vec<(String, String)>,
    opts: Options,
    sleighpath: Option<String>,
    json: bool,
    all: bool,
    jobs: Option<usize>,
    fn_seconds: u64,
    file_seconds: u64,
    /// `--fn-seconds` or `--file-seconds` was given: every check then runs in
    /// a worker process, which is what a time limit can stop.
    timed: bool,
    per_function: usize,
    baseline: Option<String>,
    save_baseline: Option<String>,
    pass: Option<usize>,
    regs: Vec<String>,
    flags: Vec<String>,
    canonical: bool,
    states: u32,
    per_form: usize,
    limit: usize,
    /// Flags a worker process gets again.
    forward: Vec<String>,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut a = Args {
        positional: Vec::new(),
        contract: None,
        options: Vec::new(),
        opts: Options::default(),
        sleighpath: None,
        json: false,
        all: false,
        jobs: None,
        fn_seconds: 600,
        file_seconds: 1200,
        timed: false,
        per_function: 6,
        baseline: None,
        save_baseline: None,
        pass: None,
        regs: Vec::new(),
        flags: Vec::new(),
        canonical: false,
        states: 4,
        per_form: 2,
        limit: 2000,
        forward: Vec::new(),
    };
    let mut it = argv.iter();
    let num = |v: Option<&String>, flag: &str| -> Result<u64, String> {
        let v = v.ok_or(format!("{flag} needs a value"))?;
        let r = match v.strip_prefix("0x") {
            Some(h) => u64::from_str_radix(h, 16),
            None => v.parse(),
        };
        r.map_err(|_| format!("{flag}: not a number: {v}"))
    };
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--contract" => a.contract = Some(it.next().ok_or("--contract needs a file")?.clone()),
            "--option" => {
                let n = it.next().ok_or("--option needs NAME VALUE")?;
                let v = it.next().ok_or("--option needs NAME VALUE")?;
                a.options.push((n.clone(), v.clone()));
            }
            "--concolic" => {
                let v = it.next();
                a.opts.concolic = u32::try_from(num(v, "--concolic")?).map_err(|_| "--concolic: too large")?;
                a.forward.extend(["--concolic".to_string(), v.cloned().unwrap_or_default()]);
            }
            "--runs" | "--seed" | "--budget" => {
                let v = it.next();
                let n = num(v, arg)?;
                match arg.as_str() {
                    "--runs" => {
                        a.opts.runs = u32::try_from(n).ok().filter(|&r| r > 0).ok_or("--runs takes a count from 1 to 4294967295")?
                    }
                    "--seed" => a.opts.seed = n,
                    _ => a.opts.budget = n,
                }
                a.forward.extend([arg.clone(), v.cloned().unwrap_or_default()]);
            }
            "--sleighpath" => {
                let v = it.next().ok_or("--sleighpath needs a directory")?.clone();
                a.forward.extend(["--sleighpath".to_string(), v.clone()]);
                a.sleighpath = Some(v);
            }
            "--jobs" => a.jobs = Some(num(it.next(), "--jobs")? as usize),
            "--fn-seconds" => {
                a.fn_seconds = num(it.next(), "--fn-seconds")?.max(1);
                a.timed = true;
            }
            "--file-seconds" => {
                a.file_seconds = num(it.next(), "--file-seconds")?.max(1);
                a.timed = true;
            }
            "--per-function" => a.per_function = num(it.next(), "--per-function")?.max(1) as usize,
            "--regs" | "--flags" => {
                let v: Vec<String> = it.next().ok_or(format!("{arg} needs names"))?.split(',').filter(|s| !s.is_empty()).map(str::to_string).collect();
                if arg == "--regs" { a.regs = v } else { a.flags = v }
            }
            "--states" => a.states = num(it.next(), "--states")?.max(1) as u32,
            "--canonical" => a.canonical = true,
            "--per-form" => a.per_form = num(it.next(), "--per-form")?.max(1) as usize,
            "--limit" => a.limit = num(it.next(), "--limit")? as usize,
            "--pass" => {
                let p = num(it.next(), "--pass")?;
                a.pass = Some(usize::try_from(p).ok().filter(|&p| p > 0).ok_or("--pass counts from 1")?);
            }
            "--baseline" => a.baseline = Some(it.next().ok_or("--baseline needs a file")?.clone()),
            "--save-baseline" => a.save_baseline = Some(it.next().ok_or("--save-baseline needs a file")?.clone()),
            "--json" => a.json = true,
            "--all" => a.all = true,
            "--trace" => {
                a.opts.trace = true;
                a.forward.push("--trace".to_string());
            }
            s if s.starts_with("--") => return Err(format!("unknown flag {s}")),
            _ => a.positional.push(arg.clone()),
        }
    }
    Ok(a)
}

fn spec_roots(sleighpath: Option<&str>) -> Vec<String> {
    if let Some(p) = sleighpath {
        return vec![p.to_string()];
    }
    let mut roots = Vec::new();
    for var in ["KUNA_SPECS", "SLEIGHHOME"] {
        if let Ok(v) = std::env::var(var) {
            if !v.is_empty() && !roots.contains(&v) {
                roots.push(v);
            }
        }
    }
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    if let Ok(p) = repo.canonicalize() {
        roots.push(p.to_string_lossy().into_owned());
    }
    roots
}

fn run_check(a: &Args) -> Result<ExitCode, String> {
    let [binary, named @ ..] = a.positional.as_slice() else {
        return Err(USAGE.to_string());
    };
    let roots = spec_roots(a.sleighpath.as_deref());
    let cannot = |e: String| {
        eprintln!("kuna-verify: {binary}: {e}");
        Ok(ExitCode::from(3))
    };
    let (functions, chunks) = if a.all {
        let extents = match session::function_extents(binary, &a.options, &roots) {
            Ok(x) => x,
            Err(e) => return cannot(e),
        };
        let sized: Vec<(String, u64)> = extents.into_iter().map(|(at, n)| (format!("{at:#x}"), n)).collect();
        (sized.iter().map(|(f, _)| f.clone()).collect::<Vec<_>>(), chunks_by_size(&sized))
    } else {
        (named.to_vec(), named.chunks(CHUNK).map(<[String]>::to_vec).collect())
    };
    if functions.is_empty() {
        return Err(USAGE.to_string());
    }
    let jobs = a.jobs.unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get())).max(1);
    let started = Instant::now();
    let source = match &a.contract {
        Some(f) => Some((std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?, f.clone())),
        None => None,
    };
    let rows = if !a.timed && (jobs == 1 || functions.len() <= CHUNK) {
        let mut rows = Vec::new();
        let got = check_binary_each(
            binary,
            &functions,
            &a.options,
            &roots,
            source.as_ref().map(|(s, n)| (s.as_str(), n.as_str())),
            &a.opts,
            |_, r| {
                let row = Row::from(&r);
                print_row(a, &row);
                rows.push(row);
            },
        );
        match got {
            Ok(()) => rows,
            Err(e) => return cannot(e),
        }
    } else {
        match run_chunks(a, binary, chunks, jobs) {
            Ok(rows) => rows,
            Err(e) => return cannot(e),
        }
    };
    if a.all {
        let wall = (functions.len() > CHUNK && jobs > 1).then(|| (0, started.elapsed().as_secs_f64()));
        print_summary(a, &[], &rows, wall);
    }
    let crashed = rows.iter().filter(|r| r.label.starts_with("UNKNOWN(harness: worker exited")).count();
    Ok(exit_code(&rows, crashed))
}

/// Functions a `check` worker process takes at a time. Fixed, so which
/// functions share a process does not depend on `--jobs`.
const CHUNK: usize = 32;

fn print_row(a: &Args, row: &Row) {
    let mut out = std::io::stdout().lock();
    let _ = if a.json { writeln!(out, "{}", row.to_json()) } else { write!(out, "{}", row.render()) };
    let _ = out.flush();
}

/// The flags a `chunk` worker needs to check as this process would.
fn chunk_flags(a: &Args) -> Vec<String> {
    let mut v = a.forward.clone();
    for (n, val) in &a.options {
        v.extend(["--option".to_string(), n.clone(), val.clone()]);
    }
    if let Some(c) = &a.contract {
        v.extend(["--contract".to_string(), c.clone()]);
    }
    v
}

/// Check `functions` in chunks across `jobs` worker processes, printing each
/// row as it arrives.
/// Chunks whose functions' total size is about the same, largest first: a
/// big function gets a worker of its own early instead of holding up the end
/// of the run. A chunk still holds at most [`CHUNK`] functions, and the
/// chunks depend only on the functions, never on `--jobs`.
fn chunks_by_size(functions: &[(String, u64)]) -> Vec<Vec<String>> {
    let mut sorted: Vec<&(String, u64)> = functions.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let total: u64 = sorted.iter().map(|f| f.1.max(1)).sum();
    let target = total.div_ceil(functions.len().div_ceil(CHUNK).max(1) as u64);
    let mut out = Vec::new();
    let (mut cur, mut acc) = (Vec::new(), 0u64);
    for (f, n) in sorted {
        cur.push(f.clone());
        acc += (*n).max(1);
        if acc >= target || cur.len() == CHUNK {
            out.push(std::mem::take(&mut cur));
            acc = 0;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn run_chunks(a: &Args, binary: &str, chunks: Vec<Vec<String>>, jobs: usize) -> Result<Vec<Row>, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let flags = chunk_flags(a);
    let workers = jobs.min(chunks.len());
    let queue = std::sync::Mutex::new(chunks.into_iter().collect::<std::collections::VecDeque<_>>());
    let rows = std::sync::Mutex::new(Vec::new());
    let fatal = std::sync::Mutex::new(None);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let Some(chunk) = queue.lock().unwrap_or_else(|e| e.into_inner()).pop_front() else { break };
                let got = run_chunk(&exe, &flags, binary, &chunk, a.fn_seconds, |row| {
                    print_row(a, &row);
                    rows.lock().unwrap_or_else(|e| e.into_inner()).push(row);
                });
                match got {
                    Ok(Some(rest)) => queue.lock().unwrap_or_else(|e| e.into_inner()).push_back(rest),
                    Ok(None) => {}
                    Err(e) => {
                        fatal.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert(e);
                        break;
                    }
                }
            });
        }
    });
    if let Some(e) = fatal.into_inner().unwrap_or_else(|e| e.into_inner()) {
        return Err(e);
    }
    Ok(rows.into_inner().unwrap_or_else(|e| e.into_inner()))
}

/// One worker over `chunk`. `Ok(Some(rest))` when it stopped early: the first
/// function without a verdict is reported, and `rest` is still to check.
fn run_chunk(
    exe: &Path,
    flags: &[String],
    binary: &str,
    chunk: &[String],
    fn_seconds: u64,
    mut on_row: impl FnMut(Row),
) -> Result<Option<Vec<String>>, String> {
    let mut child = Command::new(exe)
        .arg("chunk")
        .args(flags)
        .arg(binary)
        .args(chunk)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    let stdout = child.stdout.take().ok_or("worker without stdout")?;
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in std::io::BufRead::lines(std::io::BufReader::new(stdout)).map_while(Result::ok) {
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let mut seen = vec![false; chunk.len()];
    let stopped = loop {
        match rx.recv_timeout(std::time::Duration::from_secs(fn_seconds)) {
            Ok(line) => {
                let Ok(v) = serde_json::from_str::<Value>(&line) else { continue };
                if let Some(e) = v.get("error").and_then(Value::as_str) {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(e.to_string());
                }
                let pos = v.get("pos").and_then(Value::as_u64).map(|p| p as usize);
                if let (Some(pos), Some(row)) = (pos.filter(|&p| p < chunk.len()), v.get("row").and_then(Row::from_json)) {
                    seen[pos] = true;
                    on_row(row);
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                let _ = child.kill();
                break format!("UNKNOWN(timeout: no verdict in {fn_seconds} s)");
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break String::new(),
        }
    };
    let status = child.wait().map_err(|e| e.to_string())?;
    let _ = reader.join();
    let Some(first) = seen.iter().position(|s| !s) else { return Ok(None) };
    let label = if stopped.is_empty() { format!("UNKNOWN(harness: worker exited ({status}))") } else { stopped };
    let report = Report {
        function: chunk[first].clone(),
        verdict: Verdict::Unknown(label),
        lines: Vec::new(),
        suspects: Vec::new(),
        elapsed: std::time::Duration::ZERO,
        concolic: None,
    };
    on_row(Row::from(&report));
    let rest: Vec<String> = chunk.iter().zip(&seen).skip(first + 1).filter(|(_, s)| !**s).map(|(f, _)| f.clone()).collect();
    Ok((!rest.is_empty()).then_some(rest))
}

/// The `check` worker: print `{"pos", "row"}` as each function is done, or
/// `{"error"}` when the binary cannot be loaded.
fn run_chunk_worker(a: &Args) -> Result<ExitCode, String> {
    let [binary, functions @ ..] = a.positional.as_slice() else {
        return Err(USAGE.to_string());
    };
    let roots = spec_roots(a.sleighpath.as_deref());
    let line = |v: Value| {
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{v}");
        let _ = out.flush();
    };
    let source = match &a.contract {
        Some(f) => match std::fs::read_to_string(f) {
            Ok(s) => Some((s, f.clone())),
            Err(e) => {
                line(json!({"error": format!("{f}: {e}")}));
                return Ok(ExitCode::SUCCESS);
            }
        },
        None => None,
    };
    let got = check_binary_each(
        binary,
        functions,
        &a.options,
        &roots,
        source.as_ref().map(|(s, n)| (s.as_str(), n.as_str())),
        &a.opts,
        |pos, r| line(json!({"pos": pos, "row": Row::from(&r).to_json()})),
    );
    if let Err(e) = got {
        line(json!({"error": e}));
    }
    Ok(ExitCode::SUCCESS)
}

/// 1 when a function differs, 3 when a file could not be checked (a worker
/// crash included), 2 for a usage error, else 0. `UNKNOWN` is not a failure:
/// it says what the model could not check.
fn exit_code(rows: &[Row], file_errors: usize) -> ExitCode {
    if rows.iter().any(|r| r.class == "DIFFERS") {
        ExitCode::from(1)
    } else if file_errors > 0 {
        ExitCode::from(3)
    } else {
        ExitCode::SUCCESS
    }
}

fn collect(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        let mut entries = Vec::new();
        for e in std::fs::read_dir(path).map_err(|e| format!("{}: {e}", path.display()))? {
            entries.push(e.map_err(|e| format!("{}: {e}", path.display()))?.path());
        }
        entries.sort();
        for e in entries {
            collect(&e, out)?;
        }
    } else if path.extension().is_some_and(|x| x == "xml") {
        out.push(path.to_path_buf());
    }
    Ok(())
}

/// One datatest file's verdicts.
struct FileRows {
    rows: Vec<Row>,
    error: Option<String>,
}

fn file_json(index: usize, rows: &[Row], error: Option<&str>, wall_ms: f64) -> Value {
    json!({
        "index": index,
        "rows": rows.iter().map(Row::to_json).collect::<Vec<_>>(),
        "error": error,
        "wall_ms": wall_ms,
    })
}

/// The worker: check each listed `index=path` and print one JSON line per file.
fn run_shard(a: &Args) -> Result<ExitCode, String> {
    let roots = spec_roots(a.sleighpath.as_deref());
    let mut out = std::io::stdout().lock();
    for item in &a.positional {
        let (index, path) = item.split_once('=').ok_or("shard items are index=path")?;
        let index: usize = index.parse().map_err(|_| "bad shard index")?;
        let res = check_datatest(path, &roots, &a.opts);
        let rows: Vec<Row> = res.reports.iter().map(Row::from).collect();
        let line = file_json(index, &rows, res.error.as_deref(), res.wall.as_secs_f64() * 1e3);
        writeln!(out, "{line}").map_err(|e| e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
    }
    Ok(ExitCode::SUCCESS)
}

fn parse_file_line(line: &str) -> Option<(usize, FileRows)> {
    let v: Value = serde_json::from_str(line).ok()?;
    let index = v.get("index")?.as_u64()? as usize;
    let rows = v.get("rows")?.as_array()?.iter().map(Row::from_json).collect::<Option<Vec<_>>>()?;
    let error = v.get("error").and_then(Value::as_str).map(str::to_string);
    Some((index, FileRows { rows, error }))
}

/// Run `files` across `jobs` threads, each file in a worker process of its
/// own: a file's verdicts never depend on what the same process checked
/// before it, and a crash costs only that file.
fn run_workers(a: &Args, files: &[PathBuf], jobs: usize) -> Result<Vec<FileRows>, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: Vec<std::sync::Mutex<Option<FileRows>>> = (0..files.len()).map(|_| std::sync::Mutex::new(None)).collect();
    std::thread::scope(|scope| {
        for _ in 0..jobs {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(file) = files.get(i) else { break };
                let got = check_in_worker(&exe, &a.forward, i, file, a.file_seconds);
                *results[i].lock().unwrap_or_else(|e| e.into_inner()) = Some(got);
            });
        }
    });
    Ok(results
        .into_iter()
        .map(|r| r.into_inner().unwrap_or_else(|e| e.into_inner()).expect("every file is checked"))
        .collect())
}

/// Check one datatest file in a worker process; a worker that has not
/// finished after `seconds` is killed and its file reported as an error.
fn check_in_worker(exe: &Path, forward: &[String], index: usize, file: &Path, seconds: u64) -> FileRows {
    let child = Command::new(exe)
        .arg("shard")
        .args(forward)
        .arg(format!("{index}={}", file.to_string_lossy()))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => return FileRows { rows: Vec::new(), error: Some(e.to_string()) },
    };
    let Some(mut stdout) = child.stdout.take() else {
        let _ = child.kill();
        return FileRows { rows: Vec::new(), error: Some("worker has no output".to_string()) };
    };
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut text = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut text);
        let _ = tx.send(text);
    });
    let Ok(text) = rx.recv_timeout(std::time::Duration::from_secs(seconds)) else {
        let _ = child.kill();
        let _ = child.wait();
        let _ = reader.join();
        return FileRows { rows: Vec::new(), error: Some(format!("timeout: no verdicts in {seconds} s")) };
    };
    let status = child.wait().map(|s| s.to_string()).unwrap_or_else(|e| e.to_string());
    let _ = reader.join();
    text.lines()
        .find_map(parse_file_line)
        .map(|(_, rows)| rows)
        .unwrap_or_else(|| FileRows { rows: Vec::new(), error: Some(format!("worker exited ({status})")) })
}

fn run_corpus(a: &Args) -> Result<ExitCode, String> {
    if a.positional.is_empty() {
        return Err(USAGE.to_string());
    }
    let base = match &a.baseline {
        Some(f) => Some(baseline::parse(&std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?).map_err(|e| format!("{f}: {e}"))?),
        None => None,
    };
    let mut files = Vec::new();
    for p in &a.positional {
        let path = Path::new(p);
        if !path.exists() {
            return Err(format!("{p}: no such file or directory"));
        }
        collect(path, &mut files)?;
    }
    if files.is_empty() {
        return Err("no datatest .xml files under the given paths".to_string());
    }
    let started = Instant::now();
    let jobs = a
        .jobs
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()))
        .clamp(1, files.len().max(1));
    let results: Vec<FileRows> = if jobs == 1 && !a.timed {
        let roots = spec_roots(a.sleighpath.as_deref());
        files
            .iter()
            .map(|f| {
                let res = check_datatest(&f.to_string_lossy(), &roots, &a.opts);
                FileRows { rows: res.reports.iter().map(Row::from).collect(), error: res.error }
            })
            .collect()
    } else {
        run_workers(a, &files, jobs)?
    };
    let names: Vec<String> = files
        .iter()
        .map(|f| f.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default())
        .collect();
    if !a.json {
        println!("| test | function | verdict | check ms |");
        println!("|---|---|---|---:|");
    }
    let mut all_rows = Vec::new();
    let mut errors = Vec::new();
    for (name, res) in names.iter().zip(&results) {
        for r in &res.rows {
            if a.json {
                let mut v = r.to_json();
                v["test"] = json!(name);
                println!("{v}");
            } else {
                println!("| {name} | {} | {} | {:.2} |", r.function, r.label, r.ms);
                if r.class == "DIFFERS" || !r.suspects.is_empty() {
                    for l in r.render().lines().skip(1) {
                        println!("| | | `{}` | |", l.trim());
                    }
                }
            }
            all_rows.push((name.clone(), r.clone()));
        }
        if let Some(e) = &res.error {
            errors.push((name.clone(), e.clone()));
            if a.json {
                println!("{}", json!({"test": name, "error": e}));
            } else {
                println!("| {name} | — | file error: {} | |", e.replace('|', "/"));
            }
        }
    }
    let rows: Vec<Row> = all_rows.iter().map(|(_, r)| r.clone()).collect();
    let slowest: Vec<(f64, String)> = {
        let mut v: Vec<_> = all_rows.iter().map(|(n, r)| (r.ms, format!("{n}/{}", r.function))).collect();
        v.sort_by(|x, y| y.0.total_cmp(&x.0));
        v.truncate(5);
        v
    };
    print_summary(a, &slowest, &rows, Some((files.len(), started.elapsed().as_secs_f64())));
    if !errors.is_empty() && !a.json {
        println!("file errors {}", errors.len());
    }
    let now: baseline::Baseline = files
        .iter()
        .zip(&results)
        .map(|(f, res)| {
            let key = f.to_string_lossy().trim_start_matches("./").to_string();
            let mut rows: Vec<(String, String)> = res.rows.iter().map(|r| (r.function.clone(), baseline::class(r))).collect();
            if res.error.is_some() {
                rows.push((String::new(), baseline::ERROR.to_string()));
            }
            (key, rows)
        })
        .collect();
    if let Some(f) = &a.save_baseline {
        std::fs::write(f, baseline::render(&now)).map_err(|e| format!("{f}: {e}"))?;
        eprintln!("kuna-verify: baseline saved to {f}");
        if base.is_none() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    if let (Some(base), Some(f)) = (base, &a.baseline) {
        return Ok(report_baseline(&baseline::compare(&base, &now), f));
    }
    Ok(exit_code(&rows, errors.len()))
}

/// Print how the run compares with the baseline; 1 on a regression.
fn report_baseline(c: &baseline::Changes, file: &str) -> ExitCode {
    let actions = std::env::var_os("GITHUB_ACTIONS").is_some();
    let groups = [
        ("new DIFFERS or file errors", "error", &c.regressions),
        ("no longer agree or no longer checked", "warning", &c.lost),
        ("other changes", "notice", &c.other),
    ];
    eprintln!();
    for (title, level, lines) in groups {
        if lines.is_empty() {
            continue;
        }
        eprintln!("{title} ({}):", lines.len());
        for l in lines.iter() {
            eprintln!("  {l}");
            if actions {
                println!("::{level} title=diffverify::{l}");
            }
        }
    }
    eprintln!(
        "AGREES {} -> {} of {} -> {} functions",
        c.agreed.0, c.agreed.1, c.functions.0, c.functions.1
    );
    if c.regressions.is_empty() {
        let note = if c.is_empty() { "" } else { "; re-record it if the changes are intended" };
        eprintln!("BASELINE OK against {file}{note}");
        ExitCode::SUCCESS
    } else {
        eprintln!("BASELINE REGRESSION against {file}: {} new", c.regressions.len());
        ExitCode::from(1)
    }
}

/// Name the first rule application after which a decompile differs.
fn run_localize(a: &Args) -> Result<ExitCode, String> {
    let [target, function] = a.positional.as_slice() else {
        return Err(USAGE.to_string());
    };
    let roots = spec_roots(a.sleighpath.as_deref());
    let source = match &a.contract {
        Some(f) => Some((std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?, f.clone())),
        None => None,
    };
    let src = if target.ends_with(".xml") {
        localize::Source::Datatest(target)
    } else {
        let contract = source.as_ref().map(|(s, n)| (s.as_str(), n.as_str()));
        localize::Source::Binary { path: target, functions: vec![function.clone()], options: &a.options, contract }
    };
    let found = localize::localize(&src, &roots, function, a.pass, &a.opts)?;
    print!("{}", found.render());
    Ok(match found.finding {
        localize::Finding::NotDiffering(_) => ExitCode::SUCCESS,
        _ => ExitCode::from(1),
    })
}

/// Tier L's kuna side: one JSON line per instruction and state.
fn run_step(a: &Args) -> Result<ExitCode, String> {
    let [binary] = a.positional.as_slice() else {
        return Err(USAGE.to_string());
    };
    if a.regs.is_empty() {
        return Err("step needs --regs".to_string());
    }
    let roots = spec_roots(a.sleighpath.as_deref());
    let opts =
        kuna_verify::step::Options { seed: a.opts.seed, states: a.states, per_form: a.per_form, limit: a.limit, sweep: 4096, canonical: a.canonical };
    session::with_program(binary, &a.options, &roots, |prog| {
        let regs = kuna_verify::step::registers(prog, &a.regs, &a.flags)?;
        let mut out = std::io::stdout().lock();
        kuna_verify::step::run(prog, &regs, &opts, |v| {
            let _ = writeln!(out, "{v}");
        })
    })??;
    Ok(ExitCode::SUCCESS)
}

/// `corpus`: the file count and the wall time of the whole run; a file count
/// of 0 is a `check` run over workers, which has a wall time but no files.
fn print_summary(a: &Args, slowest: &[(f64, String)], rows: &[Row], corpus: Option<(usize, f64)>) {
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    let mut reasons: BTreeMap<String, u32> = BTreeMap::new();
    for r in rows {
        *counts.entry(r.class.as_str()).or_default() += 1;
        if let Some(k) = r.reason() {
            *reasons.entry(k).or_default() += 1;
        }
    }
    let check_s: f64 = rows.iter().map(|r| r.ms).sum::<f64>() / 1e3;
    if a.json {
        println!(
            "{}",
            json!({"summary": {"files": corpus.map(|c| c.0).filter(|&f| f > 0), "functions": rows.len(), "verdicts": counts,
                   "reasons": reasons, "check_seconds": check_s, "wall_seconds": corpus.map(|c| c.1),
                   "runs": a.opts.runs}})
        );
        return;
    }
    println!();
    match corpus {
        Some((files, _)) if files > 0 => println!("files {files}  functions {}", rows.len()),
        _ => println!("functions {}", rows.len()),
    }
    for (k, v) in &counts {
        println!("  {k:<9} {v}");
    }
    for (k, v) in &reasons {
        println!("    {k}  {v}");
    }
    match corpus {
        Some((_, wall)) => println!("check time {check_s:.3} s  wall time {wall:.3} s  (runs {})", a.opts.runs),
        None => println!("check time {check_s:.3} s  (runs {})", a.opts.runs),
    }
    for (ms, n) in slowest {
        println!("  slowest {n}  {ms:.1} ms");
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let Some((cmd, rest)) = argv.split_first() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let result = parse(rest).and_then(|a| match cmd.as_str() {
        "check" => run_check(&a),
        "corpus" => run_corpus(&a),
        "shard" => run_shard(&a),
        "chunk" => run_chunk_worker(&a),
        "step" => run_step(&a),
        "localize" => run_localize(&a),
        "mutate" => run_mutate(&a),
        _ => Err(USAGE.to_string()),
    });
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("kuna-verify: {e}");
            ExitCode::from(2)
        }
    }
}

fn mutant_json(o: &kuna_verify::mutate::Outcome) -> Value {
    json!({
        "function": o.function,
        "operator": o.operator.name(),
        "what": o.what,
        "verdict": o.verdict.label(),
        "killed": o.killed(),
        "reached": o.reached,
        "ms": o.ms,
    })
}

/// `mutate`: plant mutants in kuna's side of functions that agree and report
/// which the check catches. With `--jobs` above 1, each datatest file or each
/// chunk of a binary's functions runs in a worker process of its own.
fn run_mutate(a: &Args) -> Result<ExitCode, String> {
    let roots = spec_roots(a.sleighpath.as_deref());
    let Some(first) = a.positional.first() else { return Err(USAGE.to_string()) };
    let datatests = first.ends_with(".xml") || Path::new(first).is_dir();
    let jobs = a.jobs.unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get())).max(1);
    let mut tasks: Vec<Vec<String>> = Vec::new();
    if datatests {
        let mut files = Vec::new();
        for p in &a.positional {
            collect(Path::new(p), &mut files)?;
        }
        tasks.extend(files.iter().map(|f| vec![f.to_string_lossy().into_owned()]));
    } else {
        let chunks: Vec<Vec<String>> = if a.all {
            let sized: Vec<(String, u64)> =
                session::function_extents(first, &a.options, &roots)?.into_iter().map(|(at, n)| (format!("{at:#x}"), n)).collect();
            chunks_by_size(&sized)
        } else {
            a.positional[1..].chunks(CHUNK).map(<[String]>::to_vec).collect()
        };
        tasks.extend(chunks.into_iter().map(|c| std::iter::once(first.clone()).chain(c).collect()));
    }
    let rows: std::sync::Mutex<Vec<Value>> = std::sync::Mutex::new(Vec::new());
    let print = |v: Value| {
        if a.json {
            println!("{v}");
        }
        rows.lock().unwrap_or_else(|e| e.into_inner()).push(v);
    };
    if jobs == 1 && !a.timed {
        for t in &tasks {
            let emit = |o: kuna_verify::mutate::Outcome| print(mutant_json(&o));
            if datatests {
                if let Err(e) = kuna_verify::mutate_datatest(&t[0], &roots, &a.opts, a.per_function, emit) {
                    eprintln!("kuna-verify: {}: {e}", t[0]);
                }
            } else {
                kuna_verify::mutate_binary(&t[0], &t[1..], &a.options, &roots, &a.opts, a.per_function, emit)?;
            }
        }
    } else {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let mut flags = chunk_flags(a);
        flags.extend(["--per-function".to_string(), a.per_function.to_string(), "--jobs".to_string(), "1".to_string(), "--json".to_string()]);
        let next = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..jobs.min(tasks.len().max(1)) {
                scope.spawn(|| loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(t) = tasks.get(i) else { break };
                    let Some(out) = output_within(Command::new(&exe).arg("mutate").args(t).args(&flags), a.file_seconds) else {
                        eprintln!("kuna-verify: {}: no mutants in {} s", t[0], a.file_seconds);
                        continue;
                    };
                    for l in out.lines() {
                        if let Ok(mut v) = serde_json::from_str::<Value>(l) {
                            if v.get("operator").is_some() {
                                v["file"] = Value::from(t[0].as_str());
                                print(v);
                            }
                        }
                    }
                });
            }
        });
    }
    let rows = rows.into_inner().unwrap_or_else(|e| e.into_inner());
    print_mutation_summary(&rows, a.json);
    Ok(ExitCode::SUCCESS)
}

/// Run `cmd` and return its standard output, or `None` when it fails to
/// start or has not finished in `seconds`, in which case it is killed.
fn output_within(cmd: &mut Command, seconds: u64) -> Option<String> {
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    let mut stdout = child.stdout.take()?;
    let reader = std::thread::spawn(move || {
        let mut out = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut out);
        out
    });
    let deadline = Instant::now() + std::time::Duration::from_secs(seconds);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(std::time::Duration::from_millis(50)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = reader.join();
                return None;
            }
        }
    }
    reader.join().ok()
}

fn print_mutation_summary(rows: &[Value], json: bool) {
    let mut by: std::collections::BTreeMap<String, [usize; 4]> = std::collections::BTreeMap::new();
    for r in rows {
        let e = by.entry(r["operator"].as_str().unwrap_or("?").to_string()).or_default();
        e[0] += 1;
        if r["reached"].as_bool() == Some(true) {
            e[1] += 1;
            if r["killed"].as_bool() == Some(true) {
                e[2] += 1;
            } else if r["verdict"].as_str().is_some_and(|v| v.starts_with("UNKNOWN")) {
                e[3] += 1;
            }
        }
    }
    let total = by.values().fold([0usize; 4], |acc, e| [acc[0] + e[0], acc[1] + e[1], acc[2] + e[2], acc[3] + e[3]]);
    let recall = |e: &[usize; 4]| if e[1] == 0 { 0.0 } else { 100.0 * e[2] as f64 / e[1] as f64 };
    if json {
        let ops: serde_json::Map<String, Value> = by
            .iter()
            .map(|(k, e)| (k.clone(), json!({"mutants": e[0], "reached": e[1], "killed": e[2], "unknown": e[3], "recall": recall(e)})))
            .collect();
        println!("{}", json!({"mutation_summary": {"operators": ops, "mutants": total[0], "reached": total[1], "killed": total[2], "unknown": total[3], "recall": recall(&total)}}));
        return;
    }
    println!("{:<12} {:>8} {:>8} {:>8} {:>8} {:>7}", "operator", "mutants", "reached", "killed", "unknown", "recall");
    for (k, e) in by.iter().chain(std::iter::once((&"all".to_string(), &total))) {
        println!("{:<12} {:>8} {:>8} {:>8} {:>8} {:>6.1}%", k, e[0], e[1], e[2], e[3], recall(e));
    }
    let survived: Vec<&Value> = rows
        .iter()
        .filter(|r| r["reached"].as_bool() == Some(true) && r["killed"].as_bool() != Some(true) && !r["verdict"].as_str().is_some_and(|v| v.starts_with("UNKNOWN")))
        .collect();
    for r in survived.iter().take(20) {
        println!("survived  {}  {}  {}", r["function"].as_str().unwrap_or(""), r["what"].as_str().unwrap_or(""), r["verdict"].as_str().unwrap_or(""));
    }
}
