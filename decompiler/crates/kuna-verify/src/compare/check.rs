//! Tier C: run both sides of one function and compare what they do.
//!
//! A run compares, in order: each call (its target, kuna's arguments, and the
//! memory the callee can read), whether the function returns at all, the return
//! value, and the memory left outside the stack. The return value is compared
//! under the physical ABI (the bytes kuna's claimed return occupies) and, when a
//! typed contract is given, at the contract's width with kuna's value converted
//! to the contract type by C's rules.

use std::time::Instant;

use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;

use crate::abi::{self, Claim};
use crate::contract::{CKind, CType, Contract};
use crate::divergence::{self, ReturnMismatch};
use crate::calls::{self, Record};
use crate::program::{self, Program};
use crate::report::{Report, Verdict};
use crate::state::{Env, State};
use crate::storage::{hex, Storage};
use crate::value::{mask, Halt};
use crate::{graph, machine};
use super::observe::*;
use super::setup::*;
#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub runs: u32,
    pub seed: u64,
    /// Ops per side per run.
    pub budget: u64,
    /// Runs that may exhaust the budget before the function is given up as
    /// `UNKNOWN(budget)`: a loop bounded by an argument exhausts it on almost
    /// every random input.
    pub budget_runs: u32,
    /// Add both sides' calls to a `DIFFERS` report.
    pub trace: bool,
    /// C5: z3 queries per function for concolic inputs after the random
    /// runs; 0 runs none.
    pub concolic: u32,
    /// Calls per run whose callee read an argument kuna passes otherwise
    /// that get a machine rerun with kuna's value. A run with more is not
    /// counted as agreeing.
    pub call_reruns: u32,
}

impl Default for Options {
    fn default() -> Self {
        Options { runs: 256, seed: 0x6b75_6e61, budget: 200_000, budget_runs: 4, trace: false, concolic: 0, call_reruns: 4 }
    }
}

/// Upper return registers the machine sets to the same value on every run while
/// kuna's claimed return leaves them out (#794's shape).
struct UpperWatch {
    extra: Vec<Storage>,
    value: Option<Vec<u128>>,
    every_run: bool,
}

impl UpperWatch {
    fn new(prog: &ConsoleProgram, fd: &Funcdata, claim: &Claim) -> Option<Self> {
        let out = claim.storage.as_ref()?;
        let wide = abi::return_storage(prog, fd, &CType { spelling: String::new(), size: out.size * 2, signed: false, kind: CKind::Integer }).ok()?;
        let pieces = wide.pieces(prog);
        let extra: Vec<Storage> = pieces.iter().filter(|p| !p.overlaps(out)).cloned().collect();
        (!extra.is_empty() && pieces.iter().any(|p| p.covers(out))).then_some(UpperWatch {
            extra,
            value: None,
            every_run: true,
        })
    }

    fn observe(&mut self, ms: &mut State) {
        let mut vals = Vec::with_capacity(self.extra.len());
        for p in &self.extra {
            if !ms.wrote(&p.space, p.off, p.size) {
                self.every_run = false;
                return;
            }
            match ms.read_storage(&p.space, p.off, p.size) {
                Ok(v) => vals.push(v),
                Err(_) => {
                    self.every_run = false;
                    return;
                }
            }
        }
        match &self.value {
            None => self.value = Some(vals),
            Some(prev) if *prev != vals => self.every_run = false,
            _ => {}
        }
    }

    fn suspects(self, prog: &ConsoleProgram) -> Vec<String> {
        let Some(vals) = self.value.filter(|_| self.every_run) else { return Vec::new() };
        self.extra
            .iter()
            .zip(vals)
            .map(|(p, v)| {
                format!(
                    "the machine sets {} to {} on every run; the claimed return leaves it out",
                    p.name(prog),
                    hex(v, p.size)
                )
            })
            .collect()
    }
}

/// The whole of x87's 80-bit `ST0` when `out` is it, where a `float` or
/// `double` return lives in the 80-bit format: the machine's value is rounded
/// to the claimed size before it is compared.
fn x87_register(prog: &ConsoleProgram, out: &Storage) -> Option<Storage> {
    if !(out.size == 4 || out.size == 8) || !prog.arch().archid.starts_with("x86") {
        return None;
    }
    let [piece] = out.pieces(prog).try_into().ok()?;
    (piece.name(prog) == "ST0").then(|| Storage::new(&piece.space, piece.off, 10))
}

/// One run's comparison: the report lines when the two sides differ.
struct Compare<'a> {
    prog: &'a ConsoleProgram,
    claim: &'a Claim,
    contract: Option<(&'a Contract, &'a Storage)>,
}

enum Outcome {
    Same,
    Differs(Vec<String>),
    Declared(&'static str),
}

impl Compare<'_> {
    fn run(&self, ms: &mut State, ret: Option<u128>) -> Result<Outcome, Halt> {
        let mut lines = Vec::new();
        if let Some(out) = &self.claim.storage {
            let k = ret.ok_or_else(|| Halt::Harness("RETURN without the prototype's value".into()))?;
            let mv = if let Some(st) = x87_register(self.prog, out).filter(|_| self.claim.float) {
                crate::x87::to_ieee(ms.read_storage(&st.space, st.off, st.size)?, out.size)
            } else {
                ms.read_storage(&out.space, out.off, out.size)?
            };
            if self.claim.float && divergence::nan_quieting(out.size, mv, k & mask(out.size)) {
                return Ok(Outcome::Declared(divergence::NAN_QUIETING.id));
            }
            if mv != k & mask(out.size) {
                lines.push(format!("machine  {} = {}", out.name(self.prog), hex(mv, out.size)));
                lines.push(format!("kuna     {} = {}", self.claim.text, hex(k, out.size)));
                return Ok(Outcome::Differs(lines));
            }
        }
        let Some((c, cs)) = self.contract else { return Ok(Outcome::Same) };
        let mv = ms.read_storage(&cs.space, cs.off, cs.size)?;
        let kuna = match (&self.claim.storage, ret) {
            (Some(out), Some(k)) => {
                let value = if c.ret.kind == CKind::Boolean {
                    let value = k & mask(out.size);
                    let nonzero = if self.claim.float {
                        match out.size {
                            4 => f32::from_bits(value as u32) != 0.0,
                            8 => f64::from_bits(value as u64) != 0.0,
                            _ => return Err(Halt::Unsupported(format!("{}-byte float conversion to _Bool", out.size))),
                        }
                    } else {
                        value != 0
                    };
                    u128::from(nonzero)
                } else {
                    abi::convert(k, out.size, self.claim.signed, cs.size)
                };
                Some((out.size, value))
            }
            _ => None,
        };
        if kuna.is_some_and(|(_, kv)| kv == mv) {
            return Ok(Outcome::Same);
        }
        if let Some((ks, kv)) = kuna {
            let m = ReturnMismatch { arch: &self.prog.arch().archid, contract_size: cs.size, kuna_size: ks, machine: mv, kuna: kv };
            if let Some(d) = divergence::find(&m) {
                return Ok(Outcome::Declared(d.id));
            }
        }
        lines.push(format!("contract {}  ({})", c.declaration, c.source));
        lines.push(format!("machine  {} = {}", cs.name(self.prog), hex(mv, cs.size)));
        match (&self.claim.storage, ret, kuna) {
            (Some(out), Some(k), Some((_, kv))) => lines.push(format!(
                "kuna     {} = {}; as {} {}",
                self.claim.text,
                hex(k, out.size),
                c.ret.spelling,
                hex(kv, cs.size)
            )),
            _ => lines.push(format!("kuna     {} returns no value", self.claim.text)),
        }
        Ok(Outcome::Differs(lines))
    }
}

/// C5: the random runs whose paths are recorded for z3 to branch from.
const RECORDED_RUNS: u32 = 16;

/// C5: z3's time per query.
const CONCOLIC_MILLIS: u32 = 300;

/// Runs that may stop on an operation the model cannot run before the
/// function is given up as `UNKNOWN(unsupported-op)`: such a function is
/// `UNKNOWN` whatever its other runs show, unless one of them differs, and
/// later runs mostly take the same path into the same operation.
const UNSUPPORTED_RUNS: u32 = 16;

/// Runs that are repeated with other values in the storage kuna's C never
/// assigns.
const PERTURBED_RUNS: u32 = 16;

/// Runs that may run past the op budget on both sides, counted apart, before
/// the function stops being run: each costs two full budgets. A run past it on
/// one side only still makes the function `UNKNOWN(budget)`: one side may loop
/// where the other does not.
const BUDGET_RUNS_APART: u32 = 64;

/// Once `budget_runs` runs have run past the budget on both sides, later runs
/// get this fraction of it: a run is only cut off sooner, never compared
/// differently. One that then runs past it on one side only is run again with
/// the whole budget before it counts as one side looping.
const SHORT_BUDGET: u64 = 8;

/// Runs of the pass that gives a parameter kuna's recovered prototype types
/// `bool` any value: the 0-or-1 domain is kuna's claim, not the program's.
const DOMAIN_RUNS: u32 = 64;

pub fn check_function(prog: &ConsoleProgram, fd: &Funcdata, contract: Option<&Contract>, opts: &Options) -> Report {
    let mut work = crate::concolic::Worklist::default();
    let (mut report, assumed) = check_runs(prog, fd, contract, opts, &mut work, None, &mut None, false);
    if assumed && matches!(report.verdict, Verdict::Agrees(_)) {
        let free = Options { runs: opts.runs.min(DOMAIN_RUNS), concolic: 0, ..*opts };
        let (wide, _) = check_runs(prog, fd, contract, &free, &mut crate::concolic::Worklist::default(), None, &mut None, true);
        if matches!(wide.verdict, Verdict::Differs { .. }) {
            report.verdict = Verdict::Unknown(
                "UNKNOWN(assumed-domain: agrees only while a parameter kuna's recovered prototype types bool holds 0 or 1)".into(),
            );
            report.lines = wide.lines;
            report.suspects = wide.suspects;
        } else if let Verdict::Agrees(n) = wide.verdict {
            report.lines.push(format!("note     a parameter kuna's recovered prototype types bool also took any byte, in {n} runs that agree"));
        } else {
            report.lines.push(format!("note     with any byte in a parameter kuna's recovered prototype types bool, the check ended {}", wide.verdict.label()));
            report.verdict = match wide.verdict {
                d @ Verdict::Diverges(_) => d,
                _ => Verdict::Unknown(
                    "UNKNOWN(assumed-domain: agrees while a parameter kuna's recovered prototype types bool holds 0 or 1; with any byte the check could not tell)".into(),
                ),
            };
        }
    }
    if opts.concolic > 0 {
        let s = work.summary();
        let why = match s.solver {
            crate::concolic::SolverState::Unavailable => "; z3 could not be started, so no concolic run was made",
            crate::concolic::SolverState::Broken => "; z3 stopped answering as expected and was not asked again",
            _ => "",
        };
        report.lines.push(format!("concolic {} z3 queries, {} inputs found{why}", s.queries, s.models));
        report.concolic = Some(s);
    }
    report
}

/// Check `fd` with mutant `m` planted on kuna's side: the report, what the
/// mutant changed, and whether a run reached it. `None` when it does not apply.
pub fn check_mutant(prog: &ConsoleProgram, fd: &Funcdata, opts: &Options, m: crate::mutate::Mutation) -> Option<(Report, String, bool)> {
    let mut work = crate::concolic::Worklist::default();
    let mut got = None;
    let reached = std::rc::Rc::new(std::cell::Cell::new(false));
    let (report, _) = check_runs(prog, fd, None, opts, &mut work, Some((m, std::rc::Rc::clone(&reached))), &mut got, false);
    got.map(|what| (report, what, reached.get()))
}

fn check_runs(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    contract: Option<&Contract>,
    opts: &Options,
    work: &mut crate::concolic::Worklist,
    mutation: Option<(crate::mutate::Mutation, std::rc::Rc<std::cell::Cell<bool>>)>,
    mutated: &mut Option<String>,
    free_domain: bool,
) -> (Report, bool) {
    let started = Instant::now();
    let assumed = std::cell::Cell::new(false);
    let finish = |verdict: Verdict, lines: Vec<String>, suspects: Vec<String>| {
        let r = Report { function: fd.get_name().to_string(), verdict, lines, suspects, elapsed: started.elapsed(), concolic: None };
        (r, assumed.get())
    };
    let mut env = match Env::new(prog, opts.seed) {
        Ok(e) => e,
        Err(h) => return finish(Verdict::from_halt(&h), Vec::new(), Vec::new()),
    };
    env.fd = Some(fd);
    let claim = abi::claim(prog, fd);
    if let Some(out) = claim
        .storage
        .as_ref()
        .filter(|o| matches!(o.space.get_type(), spacetype::IPTR_INTERNAL | spacetype::IPTR_CONSTANT))
    {
        let at = format!("{}:{:#x}", out.space.get_name(), out.off);
        return finish(Verdict::Unknown(format!("UNKNOWN(return storage {at} is not machine state)")), Vec::new(), Vec::new());
    }
    env.extensions = abi::extensions(fd, &env, prog);
    env.tracked = tracked(prog, fd, &mut env);
    env.booleans = booleans(fd, &env);
    assumed.set(!fd.get_func_proto().is_input_locked() && !env.booleans.is_empty());
    if free_domain {
        env.booleans.clear();
    }
    env.kernel.set(kernel_returns(prog));
    let contract_storage = match contract.filter(|c| !c.ret.is_void()) {
        Some(c) => match abi::return_storage(prog, fd, &c.ret) {
            Ok(s) => Some(s),
            Err(e) => return finish(Verdict::Unknown(format!("UNKNOWN(contract: {e})")), Vec::new(), Vec::new()),
        },
        None => None,
    };
    let compare = Compare { prog, claim: &claim, contract: contract.zip(contract_storage.as_ref()) };
    let mut upper = UpperWatch::new(prog, fd, &claim);
    let mut machine_calls = calls::sites(fd, &env);
    env.arg_regs = machine_calls.input_regs.clone();
    env.scratch_regs = veneer_registers(prog);
    let mut calls = machine_calls.clone();
    let mut program: Program = program::compile(prog, fd, &calls);
    if let Some((m, reached)) = mutation {
        program.reached = reached;
        match crate::mutate::apply(&mut program, &mut calls, &env, m) {
            Some(what) => {
                if m.operator == crate::mutate::Operator::NoReturn {
                    let at = program.blocks[m.block].ops[m.op].at;
                    if let Some(s) = machine_calls.by_addr.get_mut(&at) {
                        s.noreturn = true;
                    }
                }
                *mutated = Some(what)
            }
            None => return finish(Verdict::Unknown("UNKNOWN(mutant does not apply)".into()), Vec::new(), Vec::new()),
        }
    }
    let calls = calls;
    let machine_calls = machine_calls;
    let program = program;
    let entry = fd.get_address().get_offset();
    let left_cfg = |mo: &machine::Outcome| mo.indirect.iter().any(|t| !program.starts.contains(t));
    let mut agreed = 0u32;
    let mut unverified_runs = 0u32;
    let mut unrerun_runs = 0u32;
    let mut trapped_runs = 0u32;
    let mut trapped: Option<String> = None;
    let mut budget_runs = 0u32;
    let mut both_budget_runs = 0u32;
    let mut unsupported_runs = 0u32;
    let mut unknown: Option<Halt> = None;
    let mut diverged: Option<&'static str> = None;
    let mut call_suspects: Vec<String> = Vec::new();
    let perturb = unassigned(prog, &env, fd, &claim, &program, &calls);
    let mut perturbed_runs = 0u32;
    let mut kernel_shows = false;
    let mut queue: std::collections::VecDeque<(u32, Option<crate::concolic::Inputs>, Option<u64>)> =
        (0..opts.runs).map(|r| (r, None, None)).collect();
    loop {
        let (run, inputs, aim) = match queue.pop_front() {
            Some(x) => x,
            None => match (opts.concolic > 0).then(|| work.next(opts.concolic, CONCOLIC_MILLIS)).flatten() {
                Some((r, inputs, at)) => (r, Some(inputs), Some(at)),
                None => break,
            },
        };
        *env.concolic_inputs.borrow_mut() = inputs.clone();
        env.record_path.set(opts.concolic > 0 && (inputs.is_some() || run < RECORDED_RUNS));
        let short = both_budget_runs >= opts.budget_runs;
        let run_budget = if short { opts.budget / SHORT_BUDGET } else { opts.budget };
        let mut m = machine::run(&env, run, entry, run_budget, &machine_calls, None);
        env.record_path.set(false);
        if let Some(path) = m.as_ref().ok().and_then(|mo| mo.path.clone()).filter(|p| !p.branches.is_empty()) {
            work.add(run, inputs.clone(), path);
        }
        let mut g = graph::run(&env, &program, run, run_budget, &calls, m.as_ref().map(|mo| mo.calls.as_slice()).unwrap_or(&[]), None);
        if short && matches!(m, Err(Halt::Budget)) != matches!(g, Err(Halt::Budget)) {
            m = machine::run(&env, run, entry, opts.budget, &machine_calls, None);
            g = graph::run(&env, &program, run, opts.budget, &calls, m.as_ref().map(|mo| mo.calls.as_slice()).unwrap_or(&[]), None);
        }
        let overlap = g.as_ref().ok().and_then(|go| go.overlapping_copy);
        let rerun_machine = |with_args: Option<&machine::WithArgs>| match overlap {
            Some(stop) => machine::run_before_copy(&env, run, entry, opts.budget, &machine_calls, with_args, stop),
            None => machine::run(&env, run, entry, opts.budget, &machine_calls, with_args),
        };
        if overlap.is_some() && m.is_ok() {
            m = rerun_machine(None);
        }
        let (mo, go) = match (m, g) {
            (Err(Halt::Trap(t)), Err(Halt::Trap(_))) => {
                trapped.get_or_insert(t);
                trapped_runs += 1;
                continue;
            }
            (Ok(_), Err(Halt::Trap(t))) if env.divide_by_zero_is_zero && t.contains("ivide by 0") => {
                diverged.get_or_insert(divergence::DIVIDE_BY_ZERO.id);
                continue;
            }
            (Err(Halt::Trap(t)), Ok(_)) if t.starts_with(crate::state::READONLY_STORE) => {
                trapped.get_or_insert(t);
                trapped_runs += 1;
                continue;
            }
            (Ok(mo), Err(Halt::NoRow(_))) if left_cfg(&mo) => {
                diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                continue;
            }
            (Err(Halt::Budget), Err(Halt::Budget)) => {
                both_budget_runs += 1;
                if both_budget_runs >= BUDGET_RUNS_APART || (agreed == 0 && both_budget_runs >= 4 * opts.budget_runs) {
                    break;
                }
                continue;
            }
            (Err(h), _) | (_, Err(h)) => {
                match h {
                    Halt::Budget => budget_runs += 1,
                    Halt::Unsupported(_) | Halt::UnexpectedReturn(_) => unsupported_runs += 1,
                    _ => {}
                }
                unknown.get_or_insert(h);
                if budget_runs >= opts.budget_runs || unsupported_runs >= UNSUPPORTED_RUNS {
                    break;
                }
                continue;
            }
            (Ok(mo), Ok(go)) => {
                if diverged.is_none() && !env.kernel.get() && mo.events.iter().any(|e| calls::SYSTEM_USEROPS.contains(&e.name.as_str())) {
                    env.kernel.set(true);
                    let alt = rerun_machine(None);
                    env.kernel.set(false);
                    if alt.is_ok_and(|alt| machine_differs(prog, &env, run, &machine_calls, &claim, &mo, &alt).is_some()) {
                        kernel_shows = true;
                    }
                }
                if go.unmatched_switch && left_cfg(&mo) {
                    diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                    continue;
                }
                (mo, go)
            }
        };
        let entry_callee = |at: u64| mo.calls.iter().any(|r| r.at == at && r.target.is_some_and(|t| env.is_entry(t)));
        if let Some(&(at, o)) = mo
            .convention
            .iter()
            .find(|(at, o)| entry_callee(*at) && calls.site(*at).preserved.iter().any(|&(p, s)| *o >= p && *o < p + u64::from(s)))
        {
            env.keep_clobbers.set(true);
            let alt = rerun_machine(None);
            env.keep_clobbers.set(false);
            if let Some(d) = alt.map_or_else(|h| Some(Verdict::from_halt(&h).label()), |alt| machine_differs(prog, &env, run, &machine_calls, &claim, &mo, &alt)) {
                let reg = Storage::new(&env.register, o & !(u64::from(env.sp.1) - 1), env.sp.1).name(prog);
                unknown.get_or_insert(Halt::AbiViolation(format!(
                    "the callee at {at:#x} changes {reg}, which the calling convention says it preserves; with the change kept, the machine's {d}"
                )));
                continue;
            }
        }
        let differs = |mut lines: Vec<String>| {
            let at = usize::from(lines.first().is_some_and(|l| l.starts_with("contract")));
            lines.insert(at, input_line(prog, &env, run, &claim));
            if let Some(a) = aim {
                lines.insert(at + 1, format!("concolic an input z3 gave to take the branch at {a:#x} the other way"));
            }
            for (site, callee) in &mo.followed {
                lines.push(format!("note     the machine ran {callee:#x} from {site:#x} itself: kuna has no call there"));
            }
            if opts.trace {
                for (side, recs) in [("machine", &mo.calls), ("kuna   ", &go.calls)] {
                    for c in recs.iter() {
                        let known = &calls.site(c.at).args;
                        let args: Vec<String> = c
                            .args
                            .iter()
                            .enumerate()
                            .map(|(i, a)| if matches!(known.get(i), Some(Some(_))) { format!("{a:#x}") } else { "?".into() })
                            .collect();
                        lines.push(format!("trace    {side} call at {:#x} to {} ({})", c.at, target(c.target), args.join(", ")));
                    }
                }
            }
            lines
        };
        for &(at, o) in &mo.convention {
            if call_suspects.len() >= 4 {
                break;
            }
            let reg = Storage::new(&env.register, o & !(u64::from(env.sp.1) - 1), env.sp.1).name(prog);
            let s = format!("the callee at {at:#x} changes {reg}, which the calling convention says it preserves");
            if !call_suspects.contains(&s) {
                call_suspects.push(s);
            }
        }
        if ptrauth_trap(&mo.events, &go.events) {
            diverged.get_or_insert(divergence::PTRAUTH_TRAP.id);
            continue;
        }
        let mut cc = compare_calls(prog, &env, run, &calls, &mo.calls, &go.calls);
        let mut rerun_failed = None;
        let unchecked = go.call_inputs.len().saturating_sub(opts.call_reruns as usize) + go.unkept_inputs;
        for ci in go.call_inputs.iter().take(opts.call_reruns as usize) {
            if cc.differs.is_some() || cc.stop {
                break;
            }
            let alt = match rerun_machine(Some(&ci.with)) {
                Ok(alt) => alt,
                Err(h) => {
                    rerun_failed = Some(format!("{}; the machine could not be rerun with kuna's arguments: {}", ci.msg, Verdict::from_halt(&h).label()));
                    break;
                }
            };
            let callee = ci.with.call.and_then(|k| callee_differs(prog, &machine_calls, mo.calls.get(k), alt.calls.get(k)));
            if let Some(d) = machine_differs(prog, &env, run, &machine_calls, &claim, &mo, &alt).or(callee) {
                let l = format!("{}; given kuna's arguments, the machine's {d}", ci.msg);
                if ci.locked {
                    cc.suspects.push(format!("{l}; the callee's declared prototype does not take it"));
                } else {
                    cc.differs = Some(vec![l]);
                }
            }
        }
        if let Some(msg) = rerun_failed.filter(|_| cc.differs.is_none()) {
            unknown.get_or_insert(Halt::Doubtful(msg));
            continue;
        }
        if cc.differs.is_none() && !cc.stop {
            cc.differs = compare_events(prog, &mo.events, &go.events);
            if cc.differs.is_none() && overlap.is_some() {
                cc.differs = compare_writes(&env, run, &mo.state.writes(), &go.writes, None)
                    .map(|d| vec![format!("memory before overlapping copy: {d}")]);
            }
        }
        let canary = cc.suspects.iter().any(|s| s.starts_with("canary: "));
        if canary {
            unknown.get_or_insert(Halt::Undefined("a call the model let write the frame changed the stack canary".into()));
            continue;
        }
        let suspect = !cc.suspects.is_empty();
        let argument_doubt = cc.suspects.iter().find(|s| s.ends_with("may not take it")).cloned();
        let narrowing = mo.narrowed && {
            env.stub_recovered.set(true);
            let alt = rerun_machine(None);
            env.stub_recovered.set(false);
            alt.map_or(true, |alt| machine_differs(prog, &env, run, &machine_calls, &claim, &mo, &alt).is_some())
        };
        let doubtful = suspect || mo.guessed || narrowing;
        let doubt = |lines: &[String]| if suspect { Halt::Doubtful(lines.join("; ")) } else { Halt::Guessed(lines.join("; ")) };
        let undefined = (go.read_uninitialized && (mo.state.read_uninitialized || mo.state.read_saved)) || smashed(&env, &mo.state);
        for s in cc.suspects {
            if call_suspects.len() < 4 && !call_suspects.contains(&s) {
                call_suspects.push(s);
            }
        }
        if let Some(id) = cc.declared {
            diverged.get_or_insert(id);
            if cc.stop {
                continue;
            }
        }
        if let Some(lines) = cc.differs {
            if undefined {
                unknown.get_or_insert(Halt::Undefined(lines.join("; ")));
                continue;
            }
            if doubtful {
                unknown.get_or_insert(doubt(&lines));
                continue;
            }
            return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
        }
        if let Some(gap) = crate::compare::observe::event_gap(prog, &mo.events, &go.events) {
            unknown.get_or_insert(Halt::Unsupported(gap));
            continue;
        }
        if overlap.is_some() {
            match argument_doubt.clone() {
                Some(s) => {
                    unknown.get_or_insert(Halt::Doubtful(s));
                }
                None => {
                    diverged.get_or_insert(divergence::COPY_OVERLAP.id);
                }
            }
            continue;
        }
        if mo.noreturn != go.noreturn && (doubtful || undefined) {
            unknown.get_or_insert(doubt(&["one side returns".to_string()]));
            continue;
        }
        if mo.noreturn != go.noreturn {
            let which = if mo.noreturn { "the machine stops in a no-return call, kuna returns" } else { "kuna stops in a no-return call, the machine returns" };
            return finish(Verdict::Differs { run, of: opts.runs }, differs(vec![which.to_string()]), Vec::new());
        }
        if mo.noreturn && mo.unverified_noreturn {
            unverified_runs += 1;
            continue;
        }
        if let Some(s) = argument_doubt {
            unknown.get_or_insert(Halt::Doubtful(s));
            continue;
        }
        let mut ms = mo.state;
        let outcome = if mo.noreturn { Ok(Outcome::Same) } else { compare.run(&mut ms, go.ret) };
        let outcome = match outcome {
            Ok(o @ (Outcome::Same | Outcome::Declared(_))) => match compare_writes(&env, run, &ms.writes(), &go.writes, None) {
                Some(d) => Ok(Outcome::Differs(vec![format!("memory at exit: {d}")])),
                None => Ok(o),
            },
            other => other,
        };
        match outcome {
            Err(h) => {
                unknown.get_or_insert(h);
                continue;
            }
            Ok(Outcome::Declared(id)) => {
                diverged.get_or_insert(id);
            }
            Ok(Outcome::Differs(lines)) if undefined => {
                unknown.get_or_insert(Halt::Undefined(lines.join("; ")));
                continue;
            }
            Ok(Outcome::Differs(lines)) if doubtful => {
                unknown.get_or_insert(doubt(&lines));
                continue;
            }
            Ok(Outcome::Differs(lines)) => {
                return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
            }
            Ok(Outcome::Same) => {}
        }
        if kernel_shows {
            diverged.get_or_insert(divergence::SYSCALL_RESULT.id);
        }
        if let Some(w) = upper.as_mut().filter(|_| !mo.noreturn) {
            w.observe(&mut ms);
        }
        if perturbed_runs < PERTURBED_RUNS && !undefined && !perturb.is_empty() {
            perturbed_runs += 1;
            let replay: &[Record] = mo.calls.as_slice();
            for u in &perturb {
                let d = match graph::run(&env, &program, run, opts.budget, &calls, replay, Some(&u.perturb)) {
                    Ok(gp) => match depends(prog, &env, run, &calls, &claim, &go, &gp) {
                        Some(d) => d,
                        None => continue,
                    },
                    Err(Halt::Trap(t)) => format!("whether it traps ({t})"),
                    Err(h) => {
                        let why = format!("with other values in {}, kuna's side ends {}", u.names, Verdict::from_halt(&h).label());
                        unknown.get_or_insert(Halt::Undefined(why));
                        continue;
                    }
                };
                if u.shared {
                    let with = if u.entry.is_empty() {
                        machine::WithArgs { call: None, bytes: Vec::new(), kill_salt: 0x5a5a_5a5a }
                    } else {
                        machine::WithArgs { call: None, bytes: entry_bytes(&env, u, run), kill_salt: 0 }
                    };
                    let base = machine::run(&env, run, entry, opts.budget, &machine_calls, None);
                    let alt = machine::run(&env, run, entry, opts.budget, &machine_calls, Some(&with));
                    match (base, alt) {
                        (Ok(b), Ok(a)) if machine_differs(prog, &env, run, &machine_calls, &claim, &b, &a).is_none() => {}
                        _ => continue,
                    }
                }
                let lines = vec![format!("kuna's result depends on {}, which its C never assigns: {d}", u.names)];
                return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
            }
        }
        if unchecked > 0 {
            unrerun_runs += 1;
            continue;
        }
        agreed += 1;
    }
    *env.concolic_inputs.borrow_mut() = None;
    let mut suspects = abi::unclaimed_inputs(prog, &env, fd, &claim);
    suspects.extend(call_suspects);
    if agreed >= opts.runs {
        if let Some(w) = upper {
            suspects.extend(w.suspects(prog));
        }
    }
    let verdict = match (unknown, diverged, trapped) {
        (Some(h), _, _) => Verdict::from_halt(&h),
        (None, None, None) if agreed == 0 && both_budget_runs > 0 => Verdict::from_halt(&Halt::Budget),
        (None, None, None) if agreed == 0 && unverified_runs > 0 => Verdict::Unknown(
            "UNKNOWN(noreturn-unverified: every run ended at a call only kuna says does not return)".to_string(),
        ),
        (None, Some(d), _) => Verdict::Diverges(d),
        (None, None, Some(t)) if agreed == 0 => Verdict::from_halt(&Halt::Trap(t)),
        (None, None, None) if agreed == 0 && unrerun_runs > 0 => Verdict::Unknown(format!(
            "UNKNOWN(call-reruns: every run had more than {} calls whose callee reads an argument kuna passes otherwise)",
            opts.call_reruns
        )),
        (None, None, _) => Verdict::Agrees(agreed),
    };
    let mut lines = match contract {
        Some(c) if matches!(verdict, Verdict::Agrees(_)) => vec![format!("contract {}  ({})", c.declaration, c.source)],
        _ => Vec::new(),
    };
    if unverified_runs > 0 && matches!(verdict, Verdict::Agrees(_)) {
        lines.push(format!(
            "note     {unverified_runs} runs ended at a call only kuna says does not return, with no name, layout or run to show it; they are not counted"
        ));
    }
    if unrerun_runs > 0 && matches!(verdict, Verdict::Agrees(_)) {
        lines.push(format!(
            "note     {unrerun_runs} runs had more than {} calls whose callee reads an argument kuna passes otherwise; the machine was not rerun for the rest, so they are not counted",
            opts.call_reruns
        ));
    }
    if both_budget_runs > 0 && matches!(verdict, Verdict::Agrees(_)) {
        lines.push(format!(
            "note     {both_budget_runs} runs ran past the op budget on both sides; they are not compared"
        ));
    }
    if trapped_runs > 0 && matches!(verdict, Verdict::Agrees(_)) {
        lines.push(format!(
            "note     {trapped_runs} runs trapped on both sides or stored into read-only memory; C leaves them undefined, so they are not compared"
        ));
    }
    finish(verdict, lines, suspects)
}
