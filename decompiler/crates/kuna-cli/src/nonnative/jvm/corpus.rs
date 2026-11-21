//! `kuna test --jvm`: the JVM engine's regression corpus (`tests/jvm/`, the
//! engine output expected byte for byte as Vineflower 1.12.0 prints it), run
//! in this process, one case per big-stack engine thread, several at a time.
//! The exit status is nonzero on any case that does not match.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use kuna_jvm::kuna_corpus::{
    failure_key, load_cases, run_case, totals, with_engine_stack, Case, CaseResult, CaseStatus,
};
use kuna_jvm::rt::try_catch;

use crate::jsonfmt::{self, Json};

pub(crate) struct JvmTestArgs {
    pub(crate) dir: Option<String>,
    pub(crate) names: Vec<String>,
    pub(crate) json: bool,
    pub(crate) jobs: Option<usize>,
}

const KEY_PREFIX: &str = "jvm:";

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
    let jobs = args.jobs.unwrap_or_else(|| {
        std::thread::available_parallelism().map_or(1, |n| n.get()).min(16)
    });
    let started = Instant::now();
    let results = run_all(&selected, jobs);
    let seconds = started.elapsed().as_secs_f64();
    let t = totals(&results);
    let failures: Vec<&CaseResult> = results.iter().filter(|r| r.status != CaseStatus::Match).collect();
    let returncode = i32::try_from(failures.len().min(255)).unwrap_or(255);

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
        let doc = vec![
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
    }
    let status = if !selection_errors.is_empty() { 2 } else { i32::from(!failures.is_empty()) };
    crate::output::emit_with_status(&text, status)
}
