//! `kuna-verify`: differential semantic checking (diffverify, Tier C).
//!
//! ```text
//! kuna-verify check <binary> <function|0xaddr>... | --all  [--contract FILE.c] [--option NAME VALUE]...
//! kuna-verify corpus <test.xml|dir>...  [--jobs N]
//! common: [--runs N] [--seed S] [--budget OPS] [--json] [--sleighpath DIR]
//! ```
//!
//! `corpus --jobs N` runs N worker processes of this binary (`shard`), each
//! printing one JSON line per file, so a file that crashes the engine costs
//! only its own verdicts.
//!
//! Exit status: 1 when a function differs, 3 when a file could not be checked,
//! 2 for a usage error, else 0.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use kuna_verify::{check_binary, check_datatest, session, Options, Row};
use serde_json::{json, Value};

const USAGE: &str = "usage:
  kuna-verify check <binary> <function|0xaddr>... | --all  [--contract FILE.c] [--option NAME VALUE]...
  kuna-verify corpus <test.xml|dir>...  [--jobs N]
options: --runs N (256)  --seed S  --budget OPS (200000)  --json  --trace  --sleighpath DIR";

struct Args {
    positional: Vec<String>,
    contract: Option<String>,
    options: Vec<(String, String)>,
    opts: Options,
    sleighpath: Option<String>,
    json: bool,
    all: bool,
    jobs: Option<usize>,
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
    let functions: Vec<String> = if a.all {
        session::function_entries(binary, &a.options, &roots)?.into_iter().map(|e| format!("{e:#x}")).collect()
    } else {
        named.to_vec()
    };
    if functions.is_empty() {
        return Err(USAGE.to_string());
    }
    let source = match &a.contract {
        Some(f) => Some((std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?, f.clone())),
        None => None,
    };
    let reports = check_binary(
        binary,
        &functions,
        &a.options,
        &roots,
        source.as_ref().map(|(s, n)| (s.as_str(), n.as_str())),
        &a.opts,
    )?;
    let rows: Vec<Row> = reports.iter().map(Row::from).collect();
    for r in &rows {
        if a.json {
            println!("{}", r.to_json());
        } else {
            print!("{}", r.render());
        }
    }
    if a.all {
        print_summary(a, &[], &rows, None);
    }
    Ok(exit_code(&rows, 0))
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

fn collect(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(path)
            .map(|d| d.filter_map(|e| e.ok().map(|e| e.path())).collect())
            .unwrap_or_default();
        entries.sort();
        for e in entries {
            collect(&e, out);
        }
    } else if path.extension().is_some_and(|x| x == "xml") {
        out.push(path.to_path_buf());
    }
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

/// Run `files` across `jobs` worker processes; every file gets a result, a
/// crashed worker's unfinished files an error.
fn run_workers(a: &Args, files: &[PathBuf], jobs: usize) -> Result<Vec<FileRows>, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut results: Vec<Option<FileRows>> = (0..files.len()).map(|_| None).collect();
    let outputs = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..jobs)
            .map(|w| {
                let items: Vec<String> = (w..files.len())
                    .step_by(jobs)
                    .map(|i| format!("{i}={}", files[i].to_string_lossy()))
                    .collect();
                let exe = exe.clone();
                let forward = a.forward.clone();
                scope.spawn(move || -> Result<(Vec<(usize, FileRows)>, String), String> {
                    let mut child = Command::new(exe)
                        .arg("shard")
                        .args(&forward)
                        .args(&items)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::null())
                        .spawn()
                        .map_err(|e| e.to_string())?;
                    let mut got = Vec::new();
                    if let Some(stdout) = child.stdout.take() {
                        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                            if let Some(parsed) = parse_file_line(&line) {
                                got.push(parsed);
                            }
                        }
                    }
                    let status = child.wait().map_err(|e| e.to_string())?;
                    Ok((got, format!("worker exited ({status})")))
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap_or_else(|_| Err("worker thread panicked".into()))).collect::<Vec<_>>()
    });
    let mut exits = Vec::new();
    for out in outputs {
        let (got, exit) = out?;
        for (i, rows) in got {
            if let Some(slot) = results.get_mut(i) {
                *slot = Some(rows);
            }
        }
        exits.push(exit);
    }
    Ok(results
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            r.unwrap_or_else(|| FileRows { rows: Vec::new(), error: Some(exits[i % jobs].clone()) })
        })
        .collect())
}

fn run_corpus(a: &Args) -> Result<ExitCode, String> {
    if a.positional.is_empty() {
        return Err(USAGE.to_string());
    }
    let mut files = Vec::new();
    for p in &a.positional {
        collect(Path::new(p), &mut files);
    }
    let started = Instant::now();
    let jobs = a
        .jobs
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()))
        .clamp(1, files.len().max(1));
    let results: Vec<FileRows> = if jobs == 1 {
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
    Ok(exit_code(&rows, errors.len()))
}

/// `corpus`: the file count and the wall time of the whole run.
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
            json!({"summary": {"files": corpus.map(|c| c.0), "functions": rows.len(), "verdicts": counts,
                   "reasons": reasons, "check_seconds": check_s, "wall_seconds": corpus.map(|c| c.1),
                   "runs": a.opts.runs}})
        );
        return;
    }
    println!();
    match corpus {
        Some((files, _)) => println!("files {files}  functions {}", rows.len()),
        None => println!("functions {}", rows.len()),
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
