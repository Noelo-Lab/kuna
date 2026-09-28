//! Serial naming and convergence of structures recorded by subprocess workers.
//!
//! Reuse compatible results, replay changed answers, and fall back to an ordered
//! worker when replay cannot settle. The parent module owns process lifetime
//! and chunk scheduling.

#[expect(clippy::disallowed_types, reason = "These are lookup caches; observable output order is explicit.")]
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use kuna_console::project::FuncResult;
use kuna_decomp::kuna_structsynth::shard::{self, FunctionRecord, Replay, SynthRequest};

use super::{
    plan_chunks, run_planned, spec_name, Ending, Faults, Phase, PlannedRun, PoolConfig,
    Session, SynthWorker, TargetSpec, Worker, JOBS_FAULT_ENV, SPAWN_FAILED,
};
use super::wire::encode_spec;

/// How a sharded run's synthesized structures were named: the kind of worker
/// whose type blocks speak for the document, and the replayed table when the
/// names are the replay's.
pub(super) struct Named {
    pub(super) kind: SynthWorker,
    pub(super) table: Option<Vec<u8>>,
}

/// Replay recorded requests in serial order, renaming compatible results and
/// re-decompiling changed answers. Non-portable or unsettled requests fall back
/// to one ordered worker; the full contract is in `docs/spec/00-overview.md`.
pub(super) fn name_structs_serially(
    cfg: &PoolConfig,
    session: &Session,
    jobs: usize,
    targets: &[TargetSpec],
    base: Replay,
    run: &mut PlannedRun,
) -> Result<Named, String> {
    let records: Vec<Option<FunctionRecord>> = run.results.iter_mut().map(|r| r.synth.take()).collect();
    let mut asked: Vec<Vec<SynthRequest>> =
        records.iter().map(|r| r.as_ref().map(|r| r.requests.clone()).unwrap_or_default()).collect();
    let askers: Vec<usize> = (0..asked.len()).filter(|&i| !asked[i].is_empty()).collect();
    if askers.is_empty() {
        return Ok(Named { kind: SynthWorker::Record, table: None });
    }
    let fallback = |run: &mut PlannedRun, why: &str| serial_fallback(cfg, session, targets, &askers, run, why);
    let faults = Faults::from_env();
    if faults.synth_serial {
        return fallback(run, &format!("{JOBS_FAULT_ENV} asked for the serial path"));
    }
    // A function the watchdog or a dead worker cut short asks a different
    // number of questions each time it runs; its record is taken as it comes.
    let cut_short: Vec<bool> = run.results.iter().map(|r| r.error.is_some()).collect();
    // What each function's own worker answered, while its record stands.
    let mut own: Vec<Option<Vec<shard::Answer>>> = records
        .iter()
        .zip(&cut_short)
        .map(|(r, &cut)| if cut || faults.synth_force { None } else { r.as_ref()?.own_answers() })
        .collect();
    let held = base.held();

    // Every kept first and sweep decompile, with the answers it was given.
    #[expect(clippy::disallowed_types, reason = "Iteration writes distinct target slots and sums an order-independent count.")]
    let mut firsts: HashMap<usize, KeptFirst> = HashMap::new();
    #[expect(clippy::disallowed_types, reason = "Lookup only; the target sequence orders the sweep.")]
    let mut sweeps: HashMap<usize, (AnswerKey, FuncResult)> = HashMap::new();
    let mut forced = 0usize;
    let mut round = 0;
    let plan = loop {
        let plan = match SynthPlan::replay(base.clone(), &asked) {
            Ok(plan) => plan,
            Err(why) => return fallback(run, &why),
        };
        for &i in &askers {
            if firsts.get(&i).is_none_or(|kept| !plan.matches(&plan.first[i], &kept.key)) {
                let first = plan.first_key(i);
                if let Some(r) = rename_result(&run.results[i], own[i].as_deref(), &first, &held) {
                    firsts.insert(i, KeptFirst { key: first, result: r, renamed: true });
                }
            }
            let Some(answers) = plan.sweep[i].as_ref().filter(|_| plan.redo_predicted(i)) else { continue };
            if sweeps.get(&i).is_none_or(|(kept, _)| !plan.matches(answers, kept)) {
                let key = plan.key(answers);
                if let Some(r) = rename_result(&run.results[i], own[i].as_deref(), &key, &held) {
                    sweeps.insert(i, (key, r));
                }
            }
        }
        let stale_first = |i: usize| {
            firsts.get(&i).is_none_or(|kept| !plan.matches(&plan.first[i], &kept.key))
        };
        let stale_sweep = |i: usize| {
            plan.redo_predicted(i) && sweeps.get(&i).is_none_or(|(key, _)| {
                plan.sweep[i].as_ref().is_none_or(|answers| !plan.matches(answers, key))
            })
        };
        let list: Vec<(usize, bool)> = askers
            .iter()
            .filter(|&&i| stale_first(i))
            .map(|&i| (i, false))
            .chain(askers.iter().filter(|&&i| stale_sweep(i)).map(|&i| (i, true)))
            .collect();
        if list.is_empty() {
            break plan;
        }
        if round == SYNTH_ROUNDS {
            return fallback(run, "the replayed questions did not settle");
        }
        round += 1;
        forced += list.len();
        for (i, sweep, r, record) in run_forced(cfg, session, jobs, targets, &plan, &list)? {
            if r.error.is_some() && !cut_short[i] {
                return fallback(run, "a function failed when decompiled again with the serial names");
            }
            let agrees = record.as_ref().is_some_and(|rec| asks_the_same(rec, &asked[i]));
            if agrees || cut_short[i] {
                if sweep {
                    sweeps.insert(i, (plan.sweep_key(i).unwrap_or_default(), r));
                } else {
                    firsts.insert(i, KeptFirst { key: plan.first_key(i), result: r, renamed: false });
                }
                continue;
            }
            match record {
                Some(rec) if !sweep => {
                    asked[i] = rec.requests;
                    own[i] = None;
                    firsts.remove(&i);
                }
                _ => return fallback(run, "a function asked the ledger something new in the sweep"),
            }
        }
    };
    let mut renamed = 0;
    for (i, kept) in firsts {
        renamed += usize::from(kept.renamed);
        run.results[i] = kept.result;
    }

    // The serial sweep: decided on the first-pass text, then each redo in
    // target order.
    let mut again: Vec<(usize, FuncResult)> = Vec::new();
    let mut leftover: Vec<(usize, bool)> = Vec::new();
    for i in (0..run.results.len())
        .filter(|&i| kuna_console::project::names_any_type(&run.results[i], &plan.stale))
    {
        if asked[i].is_empty() {
            continue;
        }
        match &plan.sweep[i] {
            None => return fallback(run, "the convergence sweep would mint a structure"),
            Some(s) if *s == plan.first[i] => {}
            Some(_) => match sweeps.remove(&i) {
                Some((key, r)) if plan.sweep[i].as_ref().is_some_and(|answers| plan.matches(answers, &key)) => {
                    again.push((i, r));
                }
                _ => leftover.push((i, true)),
            },
        }
    }
    if !leftover.is_empty() {
        forced += leftover.len();
        for (i, _, r, record) in run_forced(cfg, session, jobs, targets, &plan, &leftover)? {
            if r.error.is_some() && !cut_short[i] {
                return fallback(run, "a function failed when decompiled again with the serial names");
            }
            let agrees = record.as_ref().is_some_and(|rec| asks_the_same(rec, &asked[i]));
            if !(agrees || cut_short[i]) {
                return fallback(run, "a function asked the ledger something new in the sweep");
            }
            again.push((i, r));
        }
    }
    // The `.h` renders the structures from the workers that hold them, each
    // with the other types its own functions interned.
    if cfg.want_types && !plan.minted.is_empty() {
        if let Err(why) = install_on_idle_workers(cfg, session, &plan.table) {
            return fallback(run, &why);
        }
    }
    again.sort_by_key(|(i, _)| *i);
    for (i, r) in again {
        if kuna_console::project::redo_replaces(&run.results[i], &r) {
            run.results[i] = r;
        }
    }
    eprintln!(
        "[kuna --jobs] structsynth: {} function(s) with synthesized structures named as {} names \
         them: {renamed} renamed, {forced} decompile(s) again",
        askers.len(),
        serial_run(cfg)
    );
    Ok(Named { kind: SynthWorker::Force, table: Some(plan.table) })
}

/// Whether every name `map` covers in `r` is a TYPE name.
///
/// The substitution is textual, so a function, alias or variable literally
/// called `struct_3` would be rewritten along with the structure of that name.
/// Nothing kuna names spells one today; a binary whose symbols do takes the
/// second decompile rather than a wrong rename.
fn only_types_are_renamed(r: &FuncResult, map: &[(String, String)]) -> bool {
    let covered = |name: &str| map.iter().any(|(own, _)| own == name);
    !covered(&r.name)
        && !r.aliases.iter().any(|a| covered(a))
        && !r.variables.iter().any(|v| covered(&v.name))
}

/// `r` with its structures renamed from `own` to `serial`, or `None` when the
/// two answer lists do not name structures with the same members, or the
/// function's text names one the renaming does not cover.
fn rename_result(
    r: &FuncResult,
    own: Option<&[shard::Answer]>,
    serial: &[shard::Answer],
    held: &[String],
) -> Option<FuncResult> {
    let map = shard::renaming(own?, serial)?;
    if !only_types_are_renamed(r, &map) {
        return None;
    }
    let text = |s: &str| shard::rename_identifiers(s, &map, held);
    let mut out = r.clone();
    out.code = match r.code.as_deref() {
        Some(code) => Some(text(code)?),
        None => None,
    };
    out.proto = match r.proto.as_deref() {
        Some(proto) => Some(text(proto)?),
        None => None,
    };
    for v in &mut out.variables {
        v.type_name = text(&v.type_name)?;
    }
    for t in &mut out.types {
        t.name = text(&t.name)?;
        t.definition = text(&t.definition)?;
    }
    Some(out)
}

/// Did a forced decompile ask what the replay was told it asks, repeats aside,
/// and take exactly the answers it was given?
fn asks_the_same(record: &FunctionRecord, asked: &[SynthRequest]) -> bool {
    !record.off_script && shard::distinct(&record.requests) == shard::distinct(asked)
}

/// How many times the replay takes a function's corrected questions and runs
/// again before the run falls back to one ordered worker.
const SYNTH_ROUNDS: usize = 4;

/// What a function's lookups were answered with, each name paired with the
/// definition the replay minted under it (a name held before the run stands for
/// itself), so a kept decompile is reused only when both still hold.
type AnswerKey = Vec<shard::Answer>;

struct KeptFirst {
    key: AnswerKey,
    result: FuncResult,
    renamed: bool,
}

/// The replayed ledger of one run: the answers each function's first decompile
/// gets, the answers its redo would get, the superseded names, and the table
/// every forced worker installs.
struct SynthPlan {
    first: Vec<Vec<Option<String>>>,
    /// `None` where the sweep's lookup would mint, which the serial sweep never
    /// does for a function that asks what it asked the first time.
    sweep: Vec<Option<Vec<Option<String>>>>,
    stale: Vec<String>,
    table: Vec<u8>,
    #[expect(clippy::disallowed_types, reason = "Lookup only; table serialization precedes conversion to this index.")]
    minted: HashMap<String, SynthRequest>,
}

impl SynthPlan {
    fn replay(mut replay: Replay, asked: &[Vec<SynthRequest>]) -> Result<SynthPlan, String> {
        let mut first = Vec::with_capacity(asked.len());
        for requests in asked {
            // Every asking goes through the ledger, as it does serially; a
            // forced worker answers a repeat as it answered the first asking,
            // so only the distinct questions carry answers.
            let mut answers: Vec<(&SynthRequest, Option<String>)> = Vec::new();
            for q in requests {
                let before = replay.table().len();
                let answer = replay.lookup_or_mint(q);
                if replay.table().len() > before && !q.portable() {
                    return Err("a synthesized structure has a field type another process \
                                cannot rebuild"
                        .into());
                }
                match answers.iter().find(|(asked, _)| *asked == q) {
                    Some((_, earlier)) if *earlier != answer => {
                        return Err("a function's repeated question got another answer".into())
                    }
                    Some(_) => {}
                    None => answers.push((q, answer)),
                }
            }
            // Whether the function repeats a question can depend on what its
            // process decompiled before, so every question must get the same
            // answer at the end of the function as when it was first asked.
            if answers.iter().any(|(q, a)| replay.lookup(q) != Ok(a.clone())) {
                return Err("a function's own structures would answer its earlier question \
                            differently"
                    .into());
            }
            first.push(answers.into_iter().map(|(_, a)| a).collect());
        }
        let sweep = asked
            .iter()
            .map(|requests| {
                shard::distinct(requests).iter().map(|q| replay.lookup(q)).collect::<Result<_, _>>().ok()
            })
            .collect();
        Ok(SynthPlan {
            first,
            sweep,
            stale: replay.superseded_names(),
            table: shard::encode_table(replay.table()),
            minted: replay.into_table().into_iter().collect(),
        })
    }

    fn key(&self, answers: &[Option<String>]) -> AnswerKey {
        answers
            .iter()
            .map(|a| a.as_ref().map(|n| (n.clone(), self.minted.get(n).cloned())))
            .collect()
    }

    fn first_key(&self, i: usize) -> AnswerKey {
        self.key(&self.first[i])
    }

    fn matches(&self, answers: &[Option<String>], key: &[shard::Answer]) -> bool {
        answers
            .iter()
            .map(|answer| answer.as_deref().map(|name| (name, self.minted.get(name))))
            .eq(key.iter().map(|answer| {
                answer.as_ref().map(|(name, request)| (name.as_str(), request.as_ref()))
            }))
    }

    fn sweep_key(&self, i: usize) -> Option<AnswerKey> {
        self.sweep[i].as_ref().map(|s| self.key(s))
    }

    /// Will the sweep, as far as the answers tell, decompile `i` again with
    /// different answers? The text decides whether it really does.
    fn redo_predicted(&self, i: usize) -> bool {
        self.first[i].iter().flatten().any(|n| self.stale.contains(n))
            && self.sweep[i].as_ref().is_some_and(|s| *s != self.first[i])
    }
}

/// One forced decompile: its target slot, whether it was the sweep's, its
/// result, and what it asked the ledger.
type ForcedRun = (usize, bool, FuncResult, Option<FunctionRecord>);

/// Decompile each `(slot, sweep)` of `list` again on workers holding the
/// replayed table, each lookup answered from `plan`.
fn run_forced(
    cfg: &PoolConfig,
    session: &Session,
    jobs: usize,
    targets: &[TargetSpec],
    plan: &SynthPlan,
    list: &[(usize, bool)],
) -> Result<Vec<ForcedRun>, String> {
    let specs: Vec<TargetSpec> = list
        .iter()
        .map(|&(i, sweep)| {
            let answers = if sweep { plan.sweep[i].clone() } else { Some(plan.first[i].clone()) };
            TargetSpec { synth: answers, ..targets[i].clone() }
        })
        .collect();
    // A chunk never holds one function twice: records are matched back by
    // address, and a function the sweep redoes is here twice.
    let split = list.iter().position(|&(_, sweep)| sweep).unwrap_or(list.len());
    let mut chunks = plan_chunks(&specs[..split], cfg.chunk, jobs);
    chunks.extend(
        plan_chunks(&specs[split..], cfg.chunk, jobs)
            .into_iter()
            .map(|c| c.into_iter().map(|k| k + split).collect()),
    );
    let n = specs.len();
    let banner = |plan: &[Vec<usize>], workers: usize| {
        format!(
            "[kuna --jobs] structsynth: {n} decompile(s) of the functions with synthesized \
             structures again, with the serial names, {} chunk(s), {workers} worker process(es)",
            plan.len()
        )
    };
    let phase = Phase { synth: SynthWorker::Force, table: Some(&plan.table) };
    let run = run_planned(cfg, session, jobs, &specs, chunks, &phase, &banner)?;
    Ok(run
        .results
        .into_iter()
        .zip(list)
        .map(|(mut r, &(i, sweep))| {
            let record = r.synth.take();
            (i, sweep, r, record)
        })
        .collect())
}

/// Make every idle worker hold `table` -- a new one, when no worker is left
/// that could -- and wait until each says it installed it: the `.h` renders
/// the replayed structures from the workers that hold them, and a worker whose
/// every function kept its first decompile never took the table.
fn install_on_idle_workers(cfg: &PoolConfig, session: &Session, table: &[u8]) -> Result<(), String> {
    let id = session.write_table(table)?;
    let idle = std::mem::take(&mut *session.idle.lock().unwrap_or_else(|e| e.into_inner()));
    let (ready, mut stale): (Vec<Worker>, Vec<Worker>) =
        idle.into_iter().partition(|w| w.synth == SynthWorker::Force && w.table == id);
    if ready.is_empty() && stale.is_empty() && !session.holds_table(table) {
        let exe = std::env::current_exe().map_err(|e| format!("cannot locate the kuna binary: {e}"))?;
        let mut w = Worker::spawn(cfg, SynthWorker::Force, &exe, session.path())
            .map_err(|e| format!("{SPAWN_FAILED}: {e}"))?;
        w.table = id;
        stale.push(w);
    }
    session.idle.lock().unwrap_or_else(|e| e.into_inner()).extend(ready);
    let failed = AtomicBool::new(false);
    std::thread::scope(|scope| {
        for mut w in stale {
            let failed = &failed;
            scope.spawn(move || {
                let idx = session.chunk_ids.fetch_add(1, Ordering::SeqCst);
                if std::fs::write(session.path().join(spec_name(idx)), encode_spec(&[])).is_err() {
                    session.retire(cfg, w);
                    failed.store(true, Ordering::SeqCst);
                    return;
                }
                // An empty chunk acknowledged after the `synth` line is the
                // table installed; a worker that died instead is gone.
                if (w.stale_for(id) && !w.force(id)) || w.run_chunk(cfg, session.path(), idx, 0).2 != Ending::Finished {
                    failed.store(true, Ordering::SeqCst);
                    return;
                }
                session.idle.lock().unwrap_or_else(|e| e.into_inner()).push(w);
            });
        }
    });
    if failed.load(Ordering::SeqCst) {
        return Err("a worker could not install the synthesized structures".into());
    }
    Ok(())
}

/// The serial run a sharded one is replaying, spelled as a command line.
///
/// (kuna `protoorder`) On `decompile-all` the serial default decompiles callees
/// first, which decides what a function mints as much as the ledger does; a pool
/// cannot take that order ([`crate::decompile_all`] says so on its own line), so
/// the run whose names these are is the one with the order turned off.
fn serial_run(cfg: &PoolConfig) -> &'static str {
    if cfg.serial_callee_first {
        "--jobs 1 --option protoorder off"
    } else {
        "--jobs 1"
    }
}

/// The functions that asked the ledger, decompiled again in target order by one
/// worker running the ledger and the convergence sweep itself.
fn serial_fallback(
    cfg: &PoolConfig,
    session: &Session,
    targets: &[TargetSpec],
    askers: &[usize],
    run: &mut PlannedRun,
    why: &str,
) -> Result<Named, String> {
    eprintln!(
        "[kuna --jobs] note: {why}, so the {} function(s) with synthesized structures are \
         decompiled again in order by one worker process, as {} would.",
        askers.len(),
        serial_run(cfg)
    );
    let specs: Vec<TargetSpec> = askers.iter().map(|&i| targets[i].clone()).collect();
    let n = specs.len();
    let banner = |_: &[Vec<usize>], _: usize| {
        format!("[kuna --jobs] structsynth: {n} function(s) in one ordered chunk, 1 worker process")
    };
    let phase = Phase { synth: SynthWorker::Serial, table: None };
    let serial = run_planned(cfg, session, 1, &specs, vec![(0..n).collect()], &phase, &banner)?;
    for (&i, r) in askers.iter().zip(serial.results) {
        run.results[i] = r;
    }
    run.retries.recovered += serial.retries.recovered;
    run.retries.failed_alone += serial.retries.failed_alone;
    Ok(Named { kind: SynthWorker::Serial, table: None })
}

#[cfg(test)]
mod tests;
