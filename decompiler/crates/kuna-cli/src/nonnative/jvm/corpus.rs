//! `kuna test --jvm`: the JVM engine's regression corpus (`tests/jvm/`, the
//! engine output expected byte for byte as Vineflower 1.12.0 prints it), run
//! in this process, one case per big-stack engine thread, several at a time.
//!
//! The baseline is the corpus's own `baseline.txt` (one matching case id per
//! line) or a JSON record (`passing` keys `jvm:<id>`, as `kuna test` saves
//! them). The exit status is nonzero on any regression against `--baseline`,
//! or, without one, on any case that does not match.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use kuna_jvm::kuna_corpus::{
    compare_baseline, failure_key, format_baseline, load_cases, next_baseline, parse_baseline,
    run_case, totals, with_engine_stack, Case, CaseResult, CaseStatus,
};
use kuna_jvm::rt::try_catch;

use crate::jsonfmt::{self, Json};

pub(crate) struct JvmTestArgs {
    pub(crate) dir: Option<String>,
    pub(crate) names: Vec<String>,
    pub(crate) baseline: Option<String>,
    pub(crate) save_baseline: Option<String>,
    pub(crate) json: bool,
    pub(crate) jobs: Option<usize>,
}

const KEY_PREFIX: &str = "jvm:";

fn read_baseline(path: &str) -> Result<BTreeSet<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("could not read baseline {path}: {e}"))?;
    if !text.trim_start().starts_with('{') {
        return Ok(parse_baseline(&text));
    }
    let parsed = jsonfmt::parse(&text).ok_or_else(|| format!("could not parse baseline JSON {path}"))?;
    let Json::Object(fields) = parsed else {
        return Err(format!("baseline {path} is not a JSON object"));
    };
    let passing = fields
        .into_iter()
        .find(|(k, _)| k == "passing")
        .map(|(_, v)| v)
        .ok_or_else(|| format!("baseline {path} has no `passing` array"))?;
    let Json::Array(items) = passing else {
        return Err(format!("baseline {path}: `passing` is not an array"));
    };
    Ok(items
        .into_iter()
        .filter_map(|item| match item {
            Json::Str(key) => key.strip_prefix(KEY_PREFIX).map(str::to_string),
            _ => None,
        })
        .collect())
}

fn baseline_json(ids: &BTreeSet<String>, matched: usize, total: usize, returncode: i32) -> String {
    let doc = Json::Object(vec![
        (
            "jvm_footer".into(),
            Json::Array(vec![Json::Number(total.to_string()), Json::Number(matched.to_string())]),
        ),
        (
            "passing".into(),
            Json::Array(ids.iter().map(|id| Json::Str(format!("{KEY_PREFIX}{id}"))).collect()),
        ),
        ("returncode".into(), Json::Number(returncode.to_string())),
    ]);
    jsonfmt::dumps_indent2_sorted(&doc)
}

/// Runs `cases` on `jobs` threads; results in case order.
fn run_all(cases: &[Case], jobs: usize) -> Vec<CaseResult> {
    let next = AtomicUsize::new(0);
    let slots: Mutex<Vec<Option<CaseResult>>> = Mutex::new(vec![None; cases.len()]);
    std::thread::scope(|scope| {
        for _ in 0..jobs.clamp(1, cases.len().max(1)) {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(case) = cases.get(i) else { break };
                let result = with_engine_stack(|| {
                    try_catch(|| run_case(case))
                        .unwrap_or_else(|e| CaseResult::failed(case, format!("harness: {}", e.message), 0))
                });
                slots.lock().unwrap_or_else(|e| e.into_inner())[i] = Some(result);
            });
        }
    });
    slots
        .into_inner()
        .unwrap_or_else(|e| e.into_inner())
        .into_iter()
        .zip(cases)
        .map(|(r, case)| r.unwrap_or_else(|| CaseResult::failed(case, "harness: no result".into(), 0)))
        .collect()
}

pub(crate) fn run(args: &JvmTestArgs) -> i32 {
    let dir: PathBuf = match &args.dir {
        Some(d) => PathBuf::from(d),
        None => crate::paths::repo_root().join("tests").join("jvm"),
    };
    let manifest = dir.join("cases.tsv");
    let manifest = std::fs::canonicalize(&manifest).unwrap_or(manifest);
    let cases = match load_cases(&manifest) {
        Ok(cases) => cases,
        Err(e) => {
            eprintln!("error: {}: {e}", manifest.display());
            return 2;
        }
    };
    let mut selection_errors = Vec::new();
    for name in &args.names {
        if !cases.iter().any(|c| c.id == *name || c.id.contains(name.as_str())) {
            selection_errors.push(format!("no JVM corpus case matches {name:?}"));
        }
    }
    let selected: Vec<Case> = cases
        .into_iter()
        .filter(|c| args.names.is_empty() || args.names.iter().any(|n| c.id == *n || c.id.contains(n.as_str())))
        .collect();
    if selected.is_empty() {
        selection_errors.push(format!("no JVM corpus case selected from {}", manifest.display()));
    }
    for e in &selection_errors {
        eprintln!("error: {e}");
    }
    let baseline = match args.baseline.as_deref().map(read_baseline).transpose() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };

    let jobs = args.jobs.unwrap_or_else(|| {
        std::thread::available_parallelism().map_or(1, |n| n.get()).min(16)
    });
    let started = Instant::now();
    let results = run_all(&selected, jobs);
    let seconds = started.elapsed().as_secs_f64();
    let t = totals(&results);
    let failures: Vec<&CaseResult> = results.iter().filter(|r| r.status != CaseStatus::Match).collect();
    let returncode = i32::try_from(failures.len().min(255)).unwrap_or(255);

    if let Some(path) = &args.save_baseline {
        if !selection_errors.is_empty() {
            eprintln!("error: not writing baseline {path}: the test selection is incomplete");
        } else {
            let previous = read_baseline(path).unwrap_or_default();
            let ids = next_baseline(&previous, &results);
            let body = if Path::new(path).extension().is_some_and(|e| e == "json") {
                baseline_json(&ids, t.matched, t.total, returncode)
            } else {
                format_baseline(&ids)
            };
            match std::fs::write(path, body) {
                Ok(()) => eprintln!("baseline written to {path} ({} matching cases)", ids.len()),
                Err(e) => {
                    eprintln!("error: could not write baseline {path}: {e}");
                    return 2;
                }
            }
        }
    }

    let parity = baseline.as_ref().map(|b| compare_baseline(b, &results));
    let parity_fail = parity.as_ref().is_some_and(|p| !p.regressions.is_empty());

    let mut text = String::new();
    if args.json {
        let failure_rows = failures
            .iter()
            .map(|r| {
                Json::Array(vec![
                    Json::Str(r.id.clone()),
                    Json::Str(r.status.name().into()),
                    Json::Str(failure_key(r).unwrap_or_default()),
                ])
            })
            .collect();
        let mut doc = vec![
            (
                "jvm_footer".to_string(),
                Json::Array(vec![Json::Number(t.total.to_string()), Json::Number(t.matched.to_string())]),
            ),
            (
                "passing".to_string(),
                Json::Array(
                    results
                        .iter()
                        .filter(|r| r.status == CaseStatus::Match)
                        .map(|r| Json::Str(format!("{KEY_PREFIX}{}", r.id)))
                        .collect(),
                ),
            ),
            ("returncode".to_string(), Json::Number(returncode.to_string())),
            ("failures".to_string(), Json::Object(vec![("jvm".into(), Json::Array(failure_rows))])),
            (
                "selection".to_string(),
                Json::Object(vec![
                    ("complete".into(), Json::Bool(selection_errors.is_empty())),
                    (
                        "errors".into(),
                        Json::Array(selection_errors.iter().map(|e| Json::Str(e.clone())).collect()),
                    ),
                ]),
            ),
        ];
        if let Some(p) = &parity {
            let keys = |ids: &[String]| Json::Array(ids.iter().map(|id| Json::Str(format!("{KEY_PREFIX}{id}"))).collect());
            doc.push((
                "parity".to_string(),
                Json::Object(vec![
                    ("regressed".into(), keys(&p.regressions)),
                    ("newly_passing".into(), keys(&p.improvements)),
                    ("verdict".into(), Json::Str(if parity_fail { "PARITY FAIL" } else { "PARITY OK" }.into())),
                ]),
            ));
        }
        let _ = writeln!(text, "{}", jsonfmt::dumps_indent2_sorted(&Json::Object(doc)));
    } else {
        let _ = writeln!(
            text,
            "jvm: {}/{} cases matched ({} mismatch, {} error, {} unsupported) in {seconds:.1} s",
            t.matched, t.total, t.mismatch, t.error, t.unsupported
        );
        if !failures.is_empty() || !selection_errors.is_empty() {
            let _ = writeln!(text, "FAILURES:");
            for r in &failures {
                let _ = writeln!(
                    text,
                    "  [jvm] {}: {} {}",
                    r.id,
                    r.status.name(),
                    failure_key(r).unwrap_or_default()
                );
            }
            for e in &selection_errors {
                let _ = writeln!(text, "  [selection] {e}");
            }
        }
        let _ = writeln!(text, "exit: {returncode}");
        if let Some(p) = &parity {
            let _ = writeln!(text, "\n=== baseline parity ===");
            if !p.regressions.is_empty() {
                let _ = writeln!(text, "REGRESSED ({}):", p.regressions.len());
                for id in &p.regressions {
                    let _ = writeln!(text, "  {KEY_PREFIX}{id}");
                }
            }
            if !p.improvements.is_empty() {
                let shown: Vec<String> = p.improvements.iter().take(10).map(|id| format!("{KEY_PREFIX}{id}")).collect();
                let more = if p.improvements.len() > 10 { " ..." } else { "" };
                let _ = writeln!(text, "newly passing ({}): {}{more}", p.improvements.len(), shown.join(", "));
            }
            let _ = writeln!(text, "{}", if parity_fail { "PARITY FAIL" } else { "PARITY OK" });
        }
    }
    let status = if !selection_errors.is_empty() {
        2
    } else if baseline.is_some() {
        i32::from(parity_fail)
    } else {
        i32::from(!failures.is_empty())
    };
    crate::output::emit_with_status(&text, status)
}
