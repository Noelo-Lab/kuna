//! `kuna-verify`: differential semantic checking (diffverify, Tier C slice C1).
//!
//! ```text
//! kuna-verify check <binary> <function>... [--contract FILE.c] [--option NAME VALUE]...
//! kuna-verify corpus <test.xml|dir>...
//! common: [--runs N] [--seed S] [--budget OPS] [--sleighpath DIR]
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use kuna_verify::{check_binary, check_datatest, Options, Verdict};

const USAGE: &str = "usage:
  kuna-verify check <binary> <function>... [--contract FILE.c] [--option NAME VALUE]...
  kuna-verify corpus <test.xml|dir>...
options: --runs N (256)  --seed S  --budget OPS (200000)  --sleighpath DIR";

struct Args {
    positional: Vec<String>,
    contract: Option<String>,
    options: Vec<(String, String)>,
    opts: Options,
    sleighpath: Option<String>,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut a = Args { positional: Vec::new(), contract: None, options: Vec::new(), opts: Options::default(), sleighpath: None };
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
            "--runs" => a.opts.runs = num(it.next(), "--runs")? as u32,
            "--seed" => a.opts.seed = num(it.next(), "--seed")?,
            "--budget" => a.opts.budget = num(it.next(), "--budget")?,
            "--sleighpath" => a.sleighpath = Some(it.next().ok_or("--sleighpath needs a directory")?.clone()),
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
    let [binary, functions @ ..] = a.positional.as_slice() else {
        return Err(USAGE.to_string());
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
        functions,
        &a.options,
        &spec_roots(a.sleighpath.as_deref()),
        source.as_ref().map(|(s, n)| (s.as_str(), n.as_str())),
        &a.opts,
    )?;
    let mut differs = false;
    for r in &reports {
        print!("{}", r.render());
        differs |= matches!(r.verdict, Verdict::Differs { .. });
    }
    Ok(if differs { ExitCode::from(1) } else { ExitCode::SUCCESS })
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

fn run_corpus(a: &Args) -> Result<ExitCode, String> {
    if a.positional.is_empty() {
        return Err(USAGE.to_string());
    }
    let mut files = Vec::new();
    for p in &a.positional {
        collect(Path::new(p), &mut files);
    }
    let roots = spec_roots(a.sleighpath.as_deref());
    let started = Instant::now();
    let mut counts: BTreeMap<&'static str, u32> = BTreeMap::new();
    let mut reasons: BTreeMap<String, u32> = BTreeMap::new();
    let mut check_time = Duration::ZERO;
    let mut slowest: Vec<(Duration, String)> = Vec::new();
    let mut file_errors = 0;
    println!("| test | function | verdict | check ms |");
    println!("|---|---|---|---:|");
    for f in &files {
        let name = f.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let res = check_datatest(&f.to_string_lossy(), &roots, &a.opts);
        for r in &res.reports {
            *counts.entry(r.verdict.class()).or_default() += 1;
            if matches!(r.verdict, Verdict::Unknown(_) | Verdict::Skipped(_)) {
                let label = r.verdict.label();
                let key = label.split(':').next().unwrap_or(&label).trim_end_matches(')').to_string();
                *reasons.entry(key).or_default() += 1;
            }
            check_time += r.elapsed;
            slowest.push((r.elapsed, format!("{name}/{}", r.function)));
            println!("| {name} | {} | {} | {:.2} |", r.function, r.verdict.label(), r.elapsed.as_secs_f64() * 1e3);
            if matches!(r.verdict, Verdict::Differs { .. }) || !r.suspects.is_empty() {
                for l in r.render().lines().skip(1) {
                    println!("| | | `{}` | |", l.trim());
                }
            }
        }
        if let Some(e) = &res.error {
            file_errors += 1;
            println!("| {name} | — | file error: {} | |", e.replace('|', "/"));
        }
    }
    slowest.sort_by_key(|x| std::cmp::Reverse(x.0));
    println!();
    println!("files {}  file errors {}  functions {}", files.len(), file_errors, counts.values().sum::<u32>());
    for (k, v) in &counts {
        println!("  {k:<9} {v}");
    }
    for (k, v) in &reasons {
        println!("    {k}  {v}");
    }
    println!(
        "check time {:.3} s  wall time {:.3} s  (runs {})",
        check_time.as_secs_f64(),
        started.elapsed().as_secs_f64(),
        a.opts.runs
    );
    for (d, n) in slowest.iter().take(5) {
        println!("  slowest {n}  {:.1} ms", d.as_secs_f64() * 1e3);
    }
    Ok(if counts.contains_key("DIFFERS") { ExitCode::from(1) } else { ExitCode::SUCCESS })
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
