//! Driving a managed engine for one command line: opening the input, the
//! per-member memory guard, and the last-resort watchdog for a member no
//! deadline poll stops.
//!
//! The engine stops a member at `--max-fn-seconds` itself, at its poll sites.
//! A member stuck where nothing polls would hold the run forever, so a monitor
//! thread watches the members in progress and, once one has run `2N+30`
//! seconds, finishes the command with what was delivered (every unit not yet
//! delivered is reported as failed) and exits.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use kuna_decomp::kuna_fnmemory;
use kuna_managed::{MemoryGuard, Program, RunSettings, Runtime, Unit};

use super::args::Args;

/// Opens the input; `Err` is the command's status.
pub(crate) fn open(args: &Args) -> Result<Box<dyn Program>, i32> {
    args.engine.open(Path::new(&args.input), &args.options).map_err(|e| {
        eprintln!("error: {e}");
        1
    })
}

/// Opens what a run reads besides the input; `Err` is the command's status.
pub(crate) fn prepare(args: &Args, program: &mut dyn Program) -> Result<Option<Runtime>, i32> {
    program.prepare(&args.libs).map_err(|e| {
        eprintln!("error: {e}");
        1
    })
}

thread_local! {
    static ARMED: RefCell<Option<kuna_fnmemory::Armed>> = const { RefCell::new(None) };
}

/// `kuna_fnmemory` around each member, on the engine thread that runs it.
const MEMORY: MemoryGuard = MemoryGuard {
    arm: || ARMED.with(|armed| *armed.borrow_mut() = Some(kuna_fnmemory::Armed::enter())),
    disarm: || ARMED.with(|armed| drop(armed.borrow_mut().take())),
    over: || {
        kuna_fnmemory::over(Instant::now()).then(|| {
            format!(
                "the method exceeded the per-function memory limit ({} MiB; --max-fn-memory)",
                kuna_fnmemory::limit_mib()
            )
        })
    },
};

/// Arms `--max-fn-memory` (or the cgroup default); whether a limit is in force.
/// A member starts without handing free heap pages back: the engine threads
/// share one heap, and releasing it at every member start cost seconds of
/// system time on a large jar.
fn arm_memory_limit(args: &Args) -> bool {
    kuna_fnmemory::set_release_before_drives(false);
    match args.max_fn_memory {
        Some(mib) => {
            kuna_fnmemory::set_limit_mib(mib);
            mib > 0
        }
        None => kuna_fnmemory::use_cgroup_default(),
    }
}

/// Leaves a rendered outcome to the process exit that follows: its text was
/// built on the engine threads, and handing it back to their heaps while they
/// drop their engines costs a quarter second on a large jar.
pub(crate) fn abandon(outcome: Outcome) {
    std::mem::forget(outcome);
}

/// What a finished (or abandoned) run hands its renderer.
pub(crate) struct Outcome {
    /// Delivered units, in root order.
    pub(crate) results: Vec<Unit>,
    /// Roots asked for but never delivered, with the reason. A selection may
    /// also name a nested type in case the engine takes it for a root
    /// (`Inventory::chain`); one it does not is not listed.
    pub(crate) missing: Vec<(String, String)>,
    /// Why the run as a whole stopped early.
    pub(crate) error: Option<String>,
}

/// Runs the engine over `select` (every root for `None`) and hands the outcome
/// to `finish`, whose status is the command's; under a stall `finish` runs on
/// the monitor thread and the process exits with its status.
pub(crate) fn drive(
    program: &dyn Program,
    args: &Args,
    select: Option<BTreeSet<String>>,
    finish: &(dyn Fn(Outcome) -> i32 + Sync),
) -> i32 {
    let inventory = program.inventory();
    let top_level = |name: &str| inventory.root_of(name) == name;
    let expected: Vec<String> = match &select {
        Some(select) => select.iter().filter(|name| top_level(name)).cloned().collect(),
        None => inventory.types.iter().filter(|t| top_level(&t.name)).map(|t| t.name.clone()).collect(),
    };
    let memory = arm_memory_limit(args).then_some(MEMORY);
    let settings = RunSettings {
        jobs: args.jobs,
        select,
        member_seconds: args.max_fn_seconds,
        available_memory: kuna_decomp::kuna_hostmem::available().map(|a| a.bytes),
        watch: Arc::default(),
        memory,
    };
    let watch = &settings.watch;
    let results: Mutex<Vec<Unit>> = Mutex::new(Vec::new());
    let finishing = AtomicBool::new(false);
    let limit = (args.max_fn_seconds > 0)
        .then(|| Duration::from_secs(args.max_fn_seconds.saturating_mul(2).saturating_add(30)));
    let missing_of = |delivered: &[Unit], reason: &str| -> Vec<(String, String)> {
        let seen: BTreeSet<&str> = delivered.iter().map(|r| r.name.as_str()).collect();
        expected
            .iter()
            .filter(|name| !seen.contains(name.as_str()))
            .map(|name| (name.clone(), reason.to_string()))
            .collect()
    };
    let (done_tx, done_rx) = mpsc::channel::<()>();
    std::thread::scope(|scope| {
        if let Some(limit) = limit {
            let (results, finishing, missing_of) = (&results, &finishing, &missing_of);
            scope.spawn(move || loop {
                match done_rx.recv_timeout(Duration::from_secs(1)) {
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    _ => return,
                }
                let Some(member) = watch.stuck(limit) else {
                    continue;
                };
                if finishing.swap(true, Ordering::SeqCst) {
                    return;
                }
                let delivered = std::mem::take(&mut *results.lock().unwrap_or_else(|e| e.into_inner()));
                let reason = format!(
                    "method {member} ran {} s without reaching a deadline poll (--max-fn-seconds {}); \
                     the run was stopped",
                    limit.as_secs(),
                    args.max_fn_seconds
                );
                let missing = missing_of(&delivered, &reason);
                let status = finish(Outcome {
                    results: delivered,
                    missing,
                    error: Some(reason),
                });
                std::process::exit(status);
            });
        }
        let run = program.run(&settings, &mut |unit| {
            results.lock().unwrap_or_else(|e| e.into_inner()).push(unit);
        });
        drop(done_tx);
        if finishing.swap(true, Ordering::SeqCst) {
            loop {
                std::thread::park();
            }
        }
        let delivered = std::mem::take(&mut *results.lock().unwrap_or_else(|e| e.into_inner()));
        let outcome = match run {
            Ok(failed) => {
                let delivered_names: BTreeSet<&str> = delivered.iter().map(|r| r.name.as_str()).collect();
                let missing = failed
                    .iter()
                    .filter(|name| !delivered_names.contains(name.as_str()) && top_level(name))
                    .map(|name| (name.clone(), "not a top-level class of the input".to_string()))
                    .collect();
                Outcome {
                    results: delivered,
                    missing,
                    error: None,
                }
            }
            Err(reason) => {
                let missing = missing_of(&delivered, &reason);
                Outcome {
                    results: delivered,
                    missing,
                    error: Some(reason),
                }
            }
        };
        finish(outcome)
    })
}
