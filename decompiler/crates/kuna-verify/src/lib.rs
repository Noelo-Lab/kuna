//! diffverify: does kuna's decompiled function compute what the machine code
//! computes? `docs/features/diffverify/README.md` is the guide.
//!
//! Both sides start from the same raw machine state. The machine side runs the
//! function's SLEIGH p-code (`machine`); the decompiled side runs the final
//! `Funcdata` (`graph`). Calls run the same stub on both (`calls`). Each call,
//! the memory outside the stack, and the return value are compared, the last
//! under the physical ABI and, when one is given, a typed contract (`check`).
//! Design and scope:
//! `docs/features/diffverify/proposal.md`.
//!
//! Development only: the release binary does not link this crate.

pub mod abi;
pub mod compare;
pub mod graph;
pub mod machine;
pub mod oracle;
pub mod report;
pub mod semantics;
pub mod session;
pub mod state;
pub mod symbolic;
pub mod tools;

pub use abi::{calls, contract, divergence};
pub use compare::check;
pub use graph::program;
pub use report::baseline;
pub use semantics::{storage, value, x87};
pub use tools::{concolic, localize, mutate, step};

use std::time::{Duration, Instant};

pub use check::{check_function, Options};
pub use report::{Report, Row, Verdict};

/// Check `functions` (names, or `0x` addresses) in an object file.
/// `contract_source` is C source to read each function's return type from. A
/// function whose decompile fails is reported as `UNKNOWN`, and the rest go on.
pub fn check_binary(
    path: &str,
    functions: &[String],
    options: &[(String, String)],
    spec_roots: &[String],
    contract_source: Option<(&str, &str)>,
    opts: &Options,
) -> Result<Vec<Report>, String> {
    let mut found = Vec::new();
    check_binary_each(path, functions, options, spec_roots, contract_source, opts, |pos, r| found.push((pos, r)))?;
    found.sort_by_key(|(pos, _)| *pos);
    Ok(found.into_iter().map(|(_, r)| r).collect())
}

/// [`check_binary`], handing each function's report to `emit` as soon as it is
/// known, with the function's position in `functions`.
pub fn check_binary_each(
    path: &str,
    functions: &[String],
    options: &[(String, String)],
    spec_roots: &[String],
    contract_source: Option<(&str, &str)>,
    opts: &Options,
    mut emit: impl FnMut(usize, Report),
) -> Result<(), String> {
    let script = session::binary_script(path, functions, options);
    let first = session::load_lines(path, options).len();
    let mut load_error = None;
    let mut failure = None;
    let mut done = std::collections::BTreeSet::new();
    session::run_steps(script, None, spec_roots, true, None, |step| match step {
        session::Step::Failed { line, msg } if line < first => {
            load_error.get_or_insert_with(|| msg.to_string());
        }
        session::Step::Failed { line, msg } => {
            let pos = (line - first) / 2;
            if done.insert(pos) {
                let report = Report {
                    function: functions.get(pos).cloned().unwrap_or_default(),
                    verdict: Verdict::Unknown(format!("UNKNOWN(decompile: {msg})")),
                    lines: Vec::new(),
                    suspects: Vec::new(),
                    elapsed: Duration::ZERO,
                    concolic: None,
                };
                emit(pos, report);
            }
        }
        session::Step::Decompiled { line, prog, fd } => {
            if line < first || load_error.is_some() {
                return;
            }
            let contract = match contract_source {
                Some((src, name)) => match contract::from_source(src, fd.get_name(), name, abi::c_sizes(prog)) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        failure.get_or_insert(e);
                        return;
                    }
                },
                None => None,
            };
            let pos = (line - first) / 2;
            done.insert(pos);
            emit(pos, check_function(prog, fd, contract.as_ref(), opts));
        }
    })?;
    if let Some(e) = load_error.or(failure) {
        return Err(e);
    }
    Ok(())
}

/// One datatest file's reports, and the time its decompiles and checks took.
pub struct FileResult {
    pub reports: Vec<Report>,
    pub error: Option<String>,
    pub wall: Duration,
}

/// Check every function a datatest's script decompiles, under the options its
/// script sets.
/// A command the script expects to fail (a datatest that checks an error
/// message) is passed over, as `kuna test` does; a file none of whose
/// decompiles could be checked is an error.
pub fn check_datatest(path: &str, spec_roots: &[String], opts: &Options) -> FileResult {
    let start = Instant::now();
    let mut reports = Vec::new();
    let error = match session::load_datatest(path, spec_roots) {
        Ok((prog, commands)) => {
            let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                session::run(commands, Some(prog), spec_roots, true, None, |prog, fd| {
                    reports.push(check_function(prog, fd, None, opts));
                })
            }));
            match run {
                Ok(Ok((_, failures))) if reports.is_empty() && !failures.is_empty() => Some(failures[0].1.clone()),
                Ok(Ok(_)) => None,
                Ok(Err(e)) => Some(e),
                Err(_) => Some("panicked".to_string()),
            }
        }
        Err(e) => Some(e),
    };
    FileResult { reports, error, wall: start.elapsed() }
}

/// Plant mutants in each function a datatest decompiles (`mutate`), up to
/// `per_function` each, and hand each outcome to `emit`.
pub fn mutate_datatest(
    path: &str,
    spec_roots: &[String],
    opts: &Options,
    per_function: usize,
    mut emit: impl FnMut(mutate::Outcome),
) -> Result<(), String> {
    let (prog, commands) = session::load_datatest(path, spec_roots)?;
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        session::drive(commands, Some(prog), spec_roots, |prog, fd| {
            for o in mutate::mutate_function(prog, fd, opts, per_function) {
                emit(o);
            }
        })
    }));
    match run {
        Ok(r) => r,
        Err(_) => Err("panicked".to_string()),
    }
}

/// [`mutate_datatest`] for functions of an object file.
pub fn mutate_binary(
    path: &str,
    functions: &[String],
    options: &[(String, String)],
    spec_roots: &[String],
    opts: &Options,
    per_function: usize,
    mut emit: impl FnMut(mutate::Outcome),
) -> Result<(), String> {
    let script = session::binary_script(path, functions, options);
    let (_, failures) = session::run(script, None, spec_roots, true, None, |prog, fd| {
        for o in mutate::mutate_function(prog, fd, opts, per_function) {
            emit(o);
        }
    })?;
    if failures.is_empty() { Ok(()) } else {
        Err(failures.into_iter().map(|(_, msg)| msg).collect::<Vec<_>>().join("\n"))
    }
}
