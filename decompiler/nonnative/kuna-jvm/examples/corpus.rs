//! The kuna-jvm corpus runner: decompiles every case of a corpus manifest (default
//! `tests/jvm/cases.tsv`) and compares the output with Vineflower's. Run with `--help`.

use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::kuna_corpus::{
    compare_baseline, effective_options, error_head, escape_field, evaluate, expected_as_actual,
    failure_key, format_baseline, histogram, load_cases, next_baseline, parse_baseline, run_case,
    summary_tsv, totals, totals_line, unported_reach, with_engine_stack, Case, CaseResult,
    CaseStatus, EngineRun, JAVADOC_OPTION,
};
use kuna_jvm::rt::try_catch;

const DEFAULT_MANIFEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/jvm/cases.tsv");

const USAGE: &str = "\
usage: corpus [--manifest FILE] [--filter SUBSTR]... [--set NAME]... [--jobs N]
              [--isolate | --in-process] [--timeout SECONDS] [--mem-limit MB] [--out DIR]
              [--baseline FILE] [--save-baseline FILE] [--top N] [--verbose] [--progress]
              [--expected-as-actual] [--dump-options FILE] [--list]

  --manifest FILE       corpus manifest (default: tests/jvm/cases.tsv)
  --filter SUBSTR       run cases whose id contains SUBSTR (repeatable: any of them)
  --set NAME            run cases of test set NAME or slug (repeatable)
  --jobs N              cases in parallel (default: min(16, cpus))
  --isolate             one child process per case, so a stack overflow, abort or hang costs
                        one case (default unless --filter/--set is given)
  --in-process          run cases on fresh threads of this process (default with --filter/--set)
  --timeout SECONDS     per-case limit under --isolate (default 120)
  --mem-limit MB        per-case address-space limit under --isolate (default 8192, 0 = none)
  --out DIR             write <id>.actual.java, <id>.diff, <id>.log per case and summary.tsv,
                        histogram.tsv, unported.tsv (DIR must be new or empty)
  --baseline FILE       regression gate: report cases listed in FILE that no longer match;
                        exit 1 on any regression (other mismatches do not fail the run)
  --save-baseline FILE  write the matching ids (a filtered run keeps FILE's other ids)
  --top N               histogram rows printed (default 40, 0 = all)
  --verbose             one line per case
  --progress            report each finished case on stderr
  --expected-as-actual  self-check: feed each expected text through the comparison as the output
  --actual-dir DIR      compare stored outputs (DIR/<id>.actual.java, the --out layout) instead of
                        running the engine
  --inject-fault KIND:SUBSTR  self-check of --isolate: the child for each case whose id contains
                        SUBSTR fails with KIND (overflow, abort, hang, panic, oom, exit) instead
  --dump-options FILE   write each case's option pairs and the effective option map the engine
                        runs with (records `pairs|effective  id  key  value`) and exit
  --list                print the selected case ids and exit

Exit status: 0 if every case matches (with --baseline: if no case regressed), 1 otherwise,
2 on usage or manifest errors.";

struct Args {
    manifest: PathBuf,
    filters: Vec<String>,
    sets: Vec<String>,
    jobs: usize,
    isolate: Option<bool>,
    timeout: u64,
    mem_limit_mb: u64,
    out: Option<PathBuf>,
    baseline: Option<PathBuf>,
    save_baseline: Option<PathBuf>,
    top: usize,
    verbose: bool,
    progress: bool,
    expected_as_actual: bool,
    actual_dir: Option<PathBuf>,
    dump_options: Option<PathBuf>,
    list: bool,
    faults: Vec<(String, String)>,
    child: Option<String>,
    child_fault: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        manifest: PathBuf::from(DEFAULT_MANIFEST),
        filters: Vec::new(),
        sets: Vec::new(),
        jobs: std::thread::available_parallelism()
            .map_or(1, |n| n.get())
            .min(16),
        isolate: None,
        timeout: 120,
        mem_limit_mb: 8192,
        out: None,
        baseline: None,
        save_baseline: None,
        top: 40,
        verbose: false,
        progress: false,
        expected_as_actual: false,
        actual_dir: None,
        dump_options: None,
        list: false,
        faults: Vec::new(),
        child: None,
        child_fault: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = |name: &str| it.next().ok_or(format!("{name} needs a value"));
        let number = |name: &str, v: String| -> Result<u64, String> {
            v.parse().map_err(|_| format!("{name}: not a number: {v}"))
        };
        match arg.as_str() {
            "--manifest" => a.manifest = PathBuf::from(value("--manifest")?),
            "--filter" => a.filters.push(value("--filter")?),
            "--set" => a.sets.push(value("--set")?),
            "--jobs" => a.jobs = number("--jobs", value("--jobs")?)?.max(1) as usize,
            "--isolate" => a.isolate = Some(true),
            "--in-process" => a.isolate = Some(false),
            "--timeout" => a.timeout = number("--timeout", value("--timeout")?)?.max(1),
            "--mem-limit" => a.mem_limit_mb = number("--mem-limit", value("--mem-limit")?)?,
            "--out" => a.out = Some(PathBuf::from(value("--out")?)),
            "--baseline" => a.baseline = Some(PathBuf::from(value("--baseline")?)),
            "--save-baseline" => a.save_baseline = Some(PathBuf::from(value("--save-baseline")?)),
            "--top" => a.top = number("--top", value("--top")?)? as usize,
            "--verbose" => a.verbose = true,
            "--progress" => a.progress = true,
            "--expected-as-actual" => a.expected_as_actual = true,
            "--actual-dir" => a.actual_dir = Some(PathBuf::from(value("--actual-dir")?)),
            "--dump-options" => a.dump_options = Some(PathBuf::from(value("--dump-options")?)),
            "--list" => a.list = true,
            "--inject-fault" => {
                let spec = value("--inject-fault")?;
                let Some((kind, id)) = spec.split_once(':') else {
                    return Err(format!("--inject-fault expects KIND:SUBSTR, got {spec}"));
                };
                a.faults.push((kind.to_string(), id.to_string()));
            }
            "--child" => a.child = Some(value("--child")?),
            "--child-fault" => a.child_fault = Some(value("--child-fault")?),
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(a)
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let manifest = std::fs::canonicalize(&args.manifest)
        .or_else(|_| std::path::absolute(&args.manifest))
        .unwrap_or(args.manifest.clone());
    let cases = match load_cases(&manifest) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    if let Some(id) = &args.child {
        let Some(case) = cases.iter().find(|c| &c.id == id) else {
            eprintln!("error: no case {id}");
            return ExitCode::from(2);
        };
        if let Some(kind) = &args.child_fault {
            inject_fault(kind);
        }
        let result = with_engine_stack(|| run_case(case));
        let mut out = std::io::stdout().lock();
        let _ = out.write_all(result.to_record().as_bytes());
        let _ = out.flush();
        return ExitCode::SUCCESS;
    }

    let selected: Vec<Case> = cases
        .into_iter()
        .filter(|c| {
            args.filters.is_empty() || args.filters.iter().any(|f| c.id.contains(f.as_str()))
        })
        .filter(|c| {
            args.sets.is_empty() || args.sets.iter().any(|s| *s == c.set || *s == c.set_slug)
        })
        .collect();
    if selected.is_empty() {
        eprintln!("error: no case selected");
        return ExitCode::from(2);
    }
    if args.list {
        for c in &selected {
            println!("{}", c.id);
        }
        return ExitCode::SUCCESS;
    }
    if let Some(path) = &args.dump_options {
        return dump_options(&selected, path);
    }
    if let Some(out) = &args.out {
        let non_empty = std::fs::read_dir(out)
            .map(|mut d| d.next().is_some())
            .unwrap_or(false);
        if non_empty {
            eprintln!("error: --out {} is not empty", out.display());
            return ExitCode::from(2);
        }
        if let Err(e) = std::fs::create_dir_all(out) {
            eprintln!("error: --out {}: {e}", out.display());
            return ExitCode::from(2);
        }
    }
    let baseline = match &args.baseline {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => Some(parse_baseline(&text)),
            Err(e) => {
                eprintln!("error: --baseline {}: {e}", path.display());
                return ExitCode::from(2);
            }
        },
        None => None,
    };

    let filtered = !args.filters.is_empty() || !args.sets.is_empty();
    let offline = args.expected_as_actual || args.actual_dir.is_some();
    let isolate = !offline && args.isolate.unwrap_or(!filtered);
    let started = Instant::now();
    eprintln!(
        "corpus: {} case(s) from {}, {}, {} job(s)",
        selected.len(),
        manifest.display(),
        if args.expected_as_actual {
            "expected-as-actual"
        } else if args.actual_dir.is_some() {
            "stored outputs"
        } else if isolate {
            "isolated"
        } else {
            "in-process"
        },
        args.jobs
    );
    let results = run_all(&args, &manifest, &selected, isolate);
    eprintln!(
        "corpus: finished in {:.1} s",
        started.elapsed().as_secs_f64()
    );

    report(&args, &results, baseline.as_ref())
}

/// Runs every case, `args.jobs` at a time; results come back in case order.
fn run_all(args: &Args, manifest: &Path, cases: &[Case], isolate: bool) -> Vec<CaseResult> {
    let next = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel::<(usize, CaseResult, String)>();
    let mut results: Vec<Option<CaseResult>> = vec![None; cases.len()];
    std::thread::scope(|scope| {
        for _ in 0..args.jobs.min(cases.len()) {
            let tx = tx.clone();
            let next = &next;
            scope.spawn(move || loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(case) = cases.get(i) else { break };
                let (result, output) = if args.expected_as_actual {
                    (self_check(case), String::new())
                } else if let Some(dir) = &args.actual_dir {
                    (stored(case, dir), String::new())
                } else if isolate {
                    run_child(args, manifest, case)
                } else {
                    (with_engine_stack(|| contained(case)), String::new())
                };
                if tx.send((i, result, output)).is_err() {
                    break;
                }
            });
        }
        drop(tx);
        let mut done = 0;
        for (i, result, output) in rx {
            done += 1;
            if args.progress {
                eprintln!(
                    "[{done}/{}] {} {} ({} ms)",
                    cases.len(),
                    result.status.name(),
                    result.id,
                    result.millis
                );
            }
            if let Some(out) = &args.out {
                write_case_files(out, &cases[i], &result, &output);
            }
            results[i] = Some(result);
        }
    });
    results
        .into_iter()
        .map(|r| r.expect("every case ran"))
        .collect()
}

fn contained(case: &Case) -> CaseResult {
    try_catch(|| run_case(case))
        .unwrap_or_else(|e| CaseResult::failed(case, format!("harness: {}", e.message), 0))
}

fn self_check(case: &Case) -> CaseResult {
    match expected_as_actual(case) {
        Ok(run) => evaluate(case, run),
        Err(e) => CaseResult::failed(case, format!("harness: {e}"), 0),
    }
}

fn stored(case: &Case, dir: &Path) -> CaseResult {
    let mut path = dir.join(&case.id).into_os_string();
    path.push(".actual.java");
    let units = match std::fs::read(&path) {
        Ok(bytes) => vec![DecompiledClass {
            qualified_name: String::new(),
            path: case.compare[0].clone(),
            content: String::from_utf8_lossy(&bytes).into_owned(),
            mapping: None,
        }],
        Err(_) => Vec::new(),
    };
    evaluate(
        case,
        EngineRun {
            units,
            ..EngineRun::default()
        },
    )
}

/// Runs one case in a child process of this executable, within the time and memory limits.
fn run_child(args: &Args, manifest: &Path, case: &Case) -> (CaseResult, String) {
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            return (
                CaseResult::failed(case, format!("harness: current_exe: {e}"), 0),
                String::new(),
            )
        }
    };
    let mut child_args: Vec<&std::ffi::OsStr> = vec![
        "--manifest".as_ref(),
        manifest.as_os_str(),
        "--child".as_ref(),
        case.id.as_ref(),
    ];
    if let Some((kind, _)) = args
        .faults
        .iter()
        .find(|(_, id)| case.id.contains(id.as_str()))
    {
        child_args.push("--child-fault".as_ref());
        child_args.push(kind.as_ref());
    }
    let mut cmd = if args.mem_limit_mb > 0 && Path::new("/bin/sh").exists() {
        let mut c = Command::new("/bin/sh");
        c.arg("-c")
            .arg("ulimit -v \"$KUNA_CORPUS_VMEM_KB\" 2>/dev/null; exec \"$0\" \"$@\"")
            .arg(&exe)
            .args(&child_args)
            .env(
                "KUNA_CORPUS_VMEM_KB",
                (args.mem_limit_mb * 1024).to_string(),
            );
        c
    } else {
        let mut c = Command::new(&exe);
        c.args(&child_args);
        c
    };
    if let Some(home) = case
        .jdks
        .iter()
        .find(|j| j.current)
        .and_then(|j| j.home.as_ref())
    {
        cmd.env("JAVA_HOME", home);
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let started = Instant::now();
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            return (
                CaseResult::failed(case, format!("harness: spawn: {e}"), 0),
                String::new(),
            )
        }
    };
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");
    let out_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout.read_to_end(&mut buf);
        buf
    });
    let err_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr.read_to_end(&mut buf);
        buf
    });
    let limit = Duration::from_secs(args.timeout);
    let mut timed_out = false;
    let mut pause = Duration::from_millis(2);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() >= limit => {
                let _ = child.kill();
                timed_out = true;
                break child.wait().ok();
            }
            Ok(None) => {
                std::thread::sleep(pause);
                pause = (pause * 2).min(Duration::from_millis(50));
            }
            Err(_) => break None,
        }
    };
    let millis = started.elapsed().as_millis() as u64;
    let stdout = String::from_utf8_lossy(&out_reader.join().unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&err_reader.join().unwrap_or_default()).into_owned();

    if !timed_out && status.is_some_and(|s| s.success()) {
        if let Ok(result) = CaseResult::from_record(&stdout) {
            return (result, stderr);
        }
    }
    let what = if timed_out {
        format!("timeout: no result after {} s", args.timeout)
    } else if stderr.contains("has overflowed its stack") {
        "crash: stack overflow".to_string()
    } else if stderr.contains("memory allocation of") {
        "crash: out of memory".to_string()
    } else if let Some(at) = panic_location(&stderr) {
        format!("crash: panic outside the engine at {at}")
    } else {
        match status {
            Some(s) => match (s.code(), exit_signal(&s)) {
                (_, Some(6)) => "crash: abort (signal 6)".to_string(),
                (_, Some(sig)) => format!("crash: signal {sig}"),
                (Some(code), _) => format!("crash: exit {code}"),
                _ => "crash: unknown exit".to_string(),
            },
            None => "crash: lost the child process".to_string(),
        }
    };
    let mut output = stderr;
    if !stdout.is_empty() {
        output.push_str("\n[stdout]\n");
        output.push_str(&stdout);
    }
    (CaseResult::failed(case, what, millis), output)
}

/// `file:line` of a `thread '...' panicked at file:line:col:` message.
fn panic_location(stderr: &str) -> Option<String> {
    let rest = &stderr[stderr.find("panicked at ")? + "panicked at ".len()..];
    let loc = rest.lines().next()?.trim_end_matches(':');
    let mut parts = loc.rsplitn(3, ':');
    let (_col, line, file) = (parts.next()?, parts.next()?, parts.next()?);
    Some(format!("{file}:{line}"))
}

fn inject_fault(kind: &str) {
    match kind {
        "overflow" => {
            with_engine_stack(|| overflow(0));
        }
        "abort" => std::process::abort(),
        "hang" => loop {
            std::thread::sleep(Duration::from_secs(1));
        },
        "panic" => panic!("injected panic"),
        "oom" => {
            let v: Vec<u8> = Vec::with_capacity(1 << 40);
            std::hint::black_box(v);
        }
        "exit" => std::process::exit(3),
        other => eprintln!("unknown fault {other}"),
    }
}

fn overflow(depth: u64) -> u64 {
    let frame = std::hint::black_box([depth; 256]);
    if std::hint::black_box(depth) == u64::MAX {
        return 0;
    }
    overflow(depth + 1).wrapping_add(frame[255])
}

#[cfg(unix)]
fn exit_signal(status: &std::process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.signal()
}

#[cfg(not(unix))]
fn exit_signal(_status: &std::process::ExitStatus) -> Option<i32> {
    None
}

fn write_case_files(out: &Path, case: &Case, r: &CaseResult, child_output: &str) {
    let base = out.join(&case.id);
    if let Some(parent) = base.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let with_ext = |ext: &str| {
        let mut p = base.clone().into_os_string();
        p.push(ext);
        PathBuf::from(p)
    };
    let actual_path = with_ext(".actual.java");
    if let Some(actual) = &r.actual {
        let _ = std::fs::write(&actual_path, actual);
        if r.status != CaseStatus::Match {
            let diff = Command::new("diff")
                .arg("-u")
                .arg("--label")
                .arg(case.expected.display().to_string())
                .arg("--label")
                .arg(format!("{}.actual.java", case.id))
                .arg(&case.expected)
                .arg(&actual_path)
                .output();
            let text = match diff {
                Ok(o) => o.stdout,
                Err(e) => format!("diff -u failed to run: {e}\n").into_bytes(),
            };
            let _ = std::fs::write(with_ext(".diff"), text);
        }
    }
    let _ = std::fs::write(with_ext(".log"), case_log(case, r, child_output));
}

fn case_log(case: &Case, r: &CaseResult, child_output: &str) -> String {
    let mut s = String::new();
    let mut line = |text: String| {
        s.push_str(&text);
        s.push('\n');
    };
    line(format!("id: {}", r.id));
    line(format!("set: {}", r.set));
    line(format!("status: {}", r.status.name()));
    line(format!("key: {}", failure_key(r).unwrap_or_default()));
    line(format!("ms: {}", r.millis));
    line(format!("expected: {}", case.expected.display()));
    line(format!(
        "compared unit: {}",
        r.actual_path.as_deref().unwrap_or("(none)")
    ));
    line(format!("units: {}", r.units.join(", ")));
    if let Some(d) = &r.first_diff {
        line(format!("first diff: line {}", d.line));
        line(format!(
            "  expected: {}",
            d.expected.as_deref().unwrap_or("(end of text)")
        ));
        line(format!(
            "  actual:   {}",
            d.actual.as_deref().unwrap_or("(end of text)")
        ));
    }
    line(format!(
        "failed methods: actual {}, expected {}",
        r.failed_methods_actual, r.failed_methods_expected
    ));
    if let Some(t) = &r.error {
        line(format!("error: {}", t.message));
        line(format!("  innermost: {}", t.innermost()));
        line(format!("  at {}", t.location));
    }
    if let Some(f) = &r.failure {
        line(format!("failure: {f}"));
    }
    if let Some(reason) = &r.reason {
        line(format!("unsupported: {reason}"));
    }
    if !r.log.is_empty() {
        line("log:".to_string());
        for l in &r.log {
            line(format!("{}: {}", l.severity.name(), l.message));
            if let Some(t) = &l.throwable {
                line(format!("  {}", t.message));
                line(format!("  at {}", t.location));
            }
        }
    }
    if !child_output.is_empty() {
        line("child output:".to_string());
        line(child_output.to_string());
    }
    s
}

fn dump_options(cases: &[Case], path: &Path) -> ExitCode {
    let mut text = String::new();
    let mut failed = false;
    let mut put = |kind: &str, id: &str, entries: Vec<(String, String)>| {
        for (k, v) in entries {
            let fields = [kind, id, &k, &v].map(escape_field);
            text.push_str(&fields.join("\t"));
            text.push('\n');
        }
    };
    for case in cases {
        let mut pairs = case.options.clone();
        if let Some(style) = &case.javadoc {
            pairs.push((
                JAVADOC_OPTION.to_string(),
                format!("$javadoc_provider:{style}"),
            ));
        }
        pairs.sort();
        put("pairs", &case.id, pairs);
        match with_engine_stack(|| effective_options(case)) {
            Ok(entries) => put("effective", &case.id, entries),
            Err(t) => {
                failed = true;
                eprintln!("{}: {}", case.id, t.message);
            }
        }
    }
    if let Err(e) = std::fs::write(path, text) {
        eprintln!("error: {}: {e}", path.display());
        return ExitCode::from(2);
    }
    eprintln!("wrote {} ({} cases)", path.display(), cases.len());
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn report(args: &Args, results: &[CaseResult], baseline: Option<&BTreeSet<String>>) -> ExitCode {
    let hist = histogram(results);
    let reach = unported_reach(results);
    let shown = |n: usize| if args.top == 0 { n } else { n.min(args.top) };

    if args.verbose || results.len() <= 10 {
        for r in results {
            println!(
                "{:<11} {} {}",
                r.status.name(),
                r.id,
                failure_key(r).unwrap_or_default()
            );
            if let (CaseStatus::Mismatch, Some(d)) = (r.status, &r.first_diff) {
                println!("    line {}", d.line);
                println!(
                    "      expected: {:?}",
                    d.expected.as_deref().unwrap_or("(end of text)")
                );
                println!(
                    "      actual:   {:?}",
                    d.actual.as_deref().unwrap_or("(end of text)")
                );
            }
            if r.status != CaseStatus::Match && !error_head(r).is_empty() {
                println!("    {}", error_head(r));
            }
        }
    }

    let non_matching: usize = hist.iter().map(|(_, ids)| ids.len()).sum();
    if non_matching > 0 {
        println!(
            "failure histogram ({non_matching} non-matching case(s), {} key(s)):",
            hist.len()
        );
        for (key, ids) in hist.iter().take(shown(hist.len())) {
            println!("{:>6}  {key}", ids.len());
        }
        if shown(hist.len()) < hist.len() {
            println!("   ...  {} more key(s)", hist.len() - shown(hist.len()));
        }
    }
    if !reach.is_empty() {
        println!(
            "unported bodies by non-matching cases reaching them ({} bodies):",
            reach.len()
        );
        for (name, n) in reach.iter().take(shown(reach.len())) {
            println!("{n:>6}  {name}");
        }
        if shown(reach.len()) < reach.len() {
            println!("   ...  {} more", reach.len() - shown(reach.len()));
        }
    }

    if let Some(out) = &args.out {
        let _ = std::fs::write(out.join("summary.tsv"), summary_tsv(results));
        let mut h = String::from("count\tkey\tids\n");
        for (key, ids) in &hist {
            h.push_str(&format!("{}\t{}\t{}\n", ids.len(), key, ids.join(" ")));
        }
        let _ = std::fs::write(out.join("histogram.tsv"), h);
        let mut u = String::from("cases\tunported\n");
        for (name, n) in &reach {
            u.push_str(&format!("{n}\t{name}\n"));
        }
        let _ = std::fs::write(out.join("unported.tsv"), u);
    }

    let mut regressed = false;
    if let Some(baseline) = baseline {
        let b = compare_baseline(baseline, results);
        println!(
            "baseline: {} regression(s), {} newly matching, {} baseline id(s) not run",
            b.regressions.len(),
            b.improvements.len(),
            b.not_run.len()
        );
        let by_id = |id: &str| results.iter().find(|r| r.id == id);
        for id in &b.regressions {
            let r = by_id(id).expect("regression was run");
            println!(
                "REGRESSION  {id}  {}  {}",
                r.status.name(),
                failure_key(r).unwrap_or_default()
            );
        }
        for id in &b.improvements {
            println!("newly matching  {id}");
        }
        regressed = !b.regressions.is_empty();
    }

    if let Some(path) = &args.save_baseline {
        let previous = std::fs::read_to_string(path)
            .map(|t| parse_baseline(&t))
            .unwrap_or_default();
        let ids = next_baseline(&previous, results);
        match std::fs::write(path, format_baseline(&ids)) {
            Ok(()) => eprintln!(
                "corpus: saved {} matching id(s) to {}",
                ids.len(),
                path.display()
            ),
            Err(e) => eprintln!("error: --save-baseline {}: {e}", path.display()),
        }
    }

    let t = totals(results);
    println!("{}", totals_line(results));
    let ok = if baseline.is_some() {
        !regressed
    } else {
        t.matched == t.total
    };
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
