//! Tier C: run both sides of one function and compare what they do.
//!
//! A run compares, in order: each call (its target, kuna's arguments, and the
//! memory the callee can read), whether the function returns at all, the return
//! value, and the memory left outside the stack. The return value is compared
//! under the physical ABI (the bytes kuna's claimed return occupies) and, when a
//! typed contract is given, at the contract's width with kuna's value converted
//! to the contract type by C's rules.

use std::collections::BTreeSet;
use std::time::Instant;

use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;

use crate::abi::{self, Claim};
use crate::contract::{CType, Contract};
use crate::divergence::{self, ReturnMismatch};
use crate::calls::{self, Calls, Record};
use crate::program::{self, Program};
use crate::report::{Report, Verdict};
use crate::state::{Env, State, Writes};
use crate::storage::{hex, Storage};
use crate::value::{mask, Halt};
use crate::{graph, machine};

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
}

impl Default for Options {
    fn default() -> Self {
        Options { runs: 256, seed: 0x6b75_6e61, budget: 200_000, budget_runs: 4 }
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
        let wide = abi::return_storage(prog, fd, &CType { spelling: String::new(), size: out.size * 2, signed: false }).ok()?;
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
            let mv = ms.read_storage(&out.space, out.off, out.size)?;
            if mv != k & mask(out.size) {
                lines.push(format!("machine  {} = {}", out.name(self.prog), hex(mv, out.size)));
                lines.push(format!("kuna     {} = {}", self.claim.text, hex(k, out.size)));
                return Ok(Outcome::Differs(lines));
            }
        }
        let Some((c, cs)) = self.contract else { return Ok(Outcome::Same) };
        let mv = ms.read_storage(&cs.space, cs.off, cs.size)?;
        let kuna = match (&self.claim.storage, ret) {
            (Some(out), Some(k)) => Some((out.size, abi::convert(k, out.size, self.claim.signed, cs.size))),
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

/// The first byte the two sides left different, of the memory either wrote.
fn compare_writes(env: &Env, run: u32, machine: &Writes, kuna: &Writes) -> Option<String> {
    let manage = env.prog.arch().manage();
    let keys: BTreeSet<(i32, u64)> = machine.bytes.keys().chain(kuna.bytes.keys()).copied().collect();
    let mut first = None;
    let mut differ = 0usize;
    for (sp, off) in keys {
        let Some(space) = manage.get_space(sp) else { continue };
        let side = |w: &Writes| {
            w.bytes.get(&(sp, off)).copied().unwrap_or_else(|| env.base_byte(run, w.epoch, &w.escaped, space, off))
        };
        let (m, k) = (side(machine), side(kuna));
        if m != k {
            differ += 1;
            first.get_or_insert(format!("{}:{off:#x}: machine 0x{m:02x}, kuna 0x{k:02x}", space.get_name()));
        }
    }
    first.map(|f| if differ > 1 { format!("{f} ({differ} bytes differ)") } else { f })
}

/// A computed call target as an address in the code space: wrapped to its
/// size, and without the ARM instruction-set bit.
fn code_target(env: &Env, t: Option<u64>) -> Option<u64> {
    let arm = env.prog.arch().archid.starts_with("ARM");
    t.map(|t| env.code.wrap_offset(t) & if arm { !1 } else { !0 })
}

fn target(t: Option<u64>) -> String {
    t.map_or_else(|| "?".to_string(), |t| format!("{t:#x}"))
}

/// How the two sides' calls compare: the first difference, and the arguments a
/// recovered callee prototype may have wrong.
struct CallCompare {
    differs: Option<Vec<String>>,
    suspects: Vec<String>,
}

fn compare_calls(prog: &ConsoleProgram, env: &Env, run: u32, calls: &Calls, m: &[Record], g: &[Record]) -> CallCompare {
    let mut suspects = Vec::new();
    let differs = |lines: Vec<String>, suspects: Vec<String>| CallCompare { differs: Some(lines), suspects };
    for k in 0..m.len().max(g.len()) {
        let n = k + 1;
        let (mc, gc) = match (m.get(k), g.get(k)) {
            (Some(mc), Some(gc)) => (mc, gc),
            (Some(mc), None) => {
                let l = format!("call {n} at {:#x} to {}: the machine makes it, kuna does not", mc.at, target(mc.target));
                return differs(vec![l], suspects);
            }
            (None, Some(gc)) => {
                let l = format!("call {n} at {:#x} to {}: kuna makes it, the machine does not", gc.at, target(gc.target));
                return differs(vec![l], suspects);
            }
            (None, None) => break,
        };
        if mc.at != gc.at {
            return differs(vec![format!("call {n}: the machine calls from {:#x}, kuna from {:#x}", mc.at, gc.at)], suspects);
        }
        if mc.indirect && gc.indirect && code_target(env, mc.target) != code_target(env, gc.target) {
            let l = format!("call {n} at {:#x}: the machine calls {}, kuna calls {}", gc.at, target(mc.target), target(gc.target));
            return differs(vec![l], suspects);
        }
        let site = calls.site(gc.at);
        for (i, ((ma, ka), arg)) in mc.args.iter().zip(&gc.args).zip(&site.args).enumerate() {
            let Some(st) = arg.as_ref().map(|a| &a.storage) else { continue };
            if ma & mask(st.size) == ka & mask(st.size) {
                continue;
            }
            let l = format!(
                "call {n} to {}, argument {} ({}): machine {}, kuna {}",
                target(gc.target),
                i + 1,
                st.name(prog),
                hex(*ma & mask(st.size), st.size),
                hex(*ka & mask(st.size), st.size)
            );
            if site.locked {
                return differs(vec![l], suspects);
            }
            let s = format!("{l}; the callee's recovered prototype may not take it");
            if !suspects.contains(&s) {
                suspects.push(s);
            }
        }
        if let Some(d) = compare_writes(env, run, &mc.writes, &gc.writes) {
            return differs(vec![format!("memory before call {n} to {}: {d}", target(gc.target))], suspects);
        }
    }
    CallCompare { differs: None, suspects }
}

fn input_line(prog: &ConsoleProgram, env: &Env, run: u32, claim: &Claim) -> String {
    let mut init = State::new(env, run);
    let inputs: Vec<String> = claim
        .params
        .iter()
        .map(|(n, st)| match init.read_storage(&st.space, st.off, st.size) {
            Ok(v) => format!("{n}={} ({})", hex(v, st.size), st.name(prog)),
            Err(_) => format!("{n}=? ({})", st.name(prog)),
        })
        .collect();
    let input = if inputs.is_empty() { "no declared parameters".to_string() } else { inputs.join("  ") };
    format!("input    {input}")
}

/// Check one decompiled function. `contract`, when given, is the function's
/// real type.
pub fn check_function(prog: &ConsoleProgram, fd: &Funcdata, contract: Option<&Contract>, opts: &Options) -> Report {
    let started = Instant::now();
    let finish = |verdict: Verdict, lines: Vec<String>, suspects: Vec<String>| Report {
        function: fd.get_name().to_string(),
        verdict,
        lines,
        suspects,
        elapsed: started.elapsed(),
    };
    let mut env = match Env::new(prog, opts.seed) {
        Ok(e) => e,
        Err(h) => return finish(Verdict::from_halt(&h), Vec::new(), Vec::new()),
    };
    let claim = abi::claim(fd);
    if let Some(out) = claim.storage.as_ref().filter(|o| o.space.get_type() == spacetype::IPTR_INTERNAL) {
        let at = format!("{}:{:#x}", out.space.get_name(), out.off);
        return finish(Verdict::Unknown(format!("UNKNOWN(return storage {at} is not machine state)")), Vec::new(), Vec::new());
    }
    env.extensions = abi::extensions(fd, &env, prog);
    let contract_storage = match contract.filter(|c| !c.ret.is_void()) {
        Some(c) => match abi::return_storage(prog, fd, &c.ret) {
            Ok(s) => Some(s),
            Err(e) => return finish(Verdict::Unknown(format!("UNKNOWN(contract: {e})")), Vec::new(), Vec::new()),
        },
        None => None,
    };
    let compare = Compare { prog, claim: &claim, contract: contract.zip(contract_storage.as_ref()) };
    let mut upper = UpperWatch::new(prog, fd, &claim);
    let calls = calls::sites(fd, &env);
    let program: Program = program::compile(prog, fd, &calls);
    let entry = fd.get_address().get_offset();
    let left_cfg = |mo: &machine::Outcome| mo.indirect.iter().any(|t| !program.starts.contains(t));
    let mut agreed = 0u32;
    let mut budget_runs = 0u32;
    let mut unknown: Option<Halt> = None;
    let mut diverged: Option<&'static str> = None;
    let mut call_suspects: Vec<String> = Vec::new();
    for run in 0..opts.runs {
        let m = machine::run(&env, run, entry, opts.budget, &calls);
        let g = graph::run(&env, &program, run, opts.budget, &calls);
        let (mo, go) = match (m, g) {
            (Err(Halt::Trap(_)), Err(Halt::Trap(_))) => {
                agreed += 1;
                continue;
            }
            (Ok(mo), Err(Halt::NoRow(_))) if left_cfg(&mo) => {
                diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                continue;
            }
            (Err(h), _) | (_, Err(h)) => {
                if h == Halt::Budget {
                    budget_runs += 1;
                }
                unknown.get_or_insert(h);
                if budget_runs >= opts.budget_runs {
                    break;
                }
                continue;
            }
            (Ok(mo), Ok(go)) => {
                if go.unmatched_switch && left_cfg(&mo) {
                    diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                    continue;
                }
                (mo, go)
            }
        };
        let differs = |mut lines: Vec<String>| {
            let at = usize::from(lines.first().is_some_and(|l| l.starts_with("contract")));
            lines.insert(at, input_line(prog, &env, run, &claim));
            for (site, callee) in &mo.followed {
                lines.push(format!("note     the machine ran {callee:#x} from {site:#x} itself: kuna has no call there"));
            }
            lines
        };
        let cc = compare_calls(prog, &env, run, &calls, &mo.calls, &go.calls);
        for s in cc.suspects {
            if call_suspects.len() < 4 && !call_suspects.contains(&s) {
                call_suspects.push(s);
            }
        }
        if let Some(lines) = cc.differs {
            return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
        }
        if mo.noreturn != go.noreturn {
            let which = if mo.noreturn { "the machine stops in a no-return call, kuna returns" } else { "kuna stops in a no-return call, the machine returns" };
            return finish(Verdict::Differs { run, of: opts.runs }, differs(vec![which.to_string()]), Vec::new());
        }
        let mut ms = mo.state;
        let outcome = if mo.noreturn { Ok(Outcome::Same) } else { compare.run(&mut ms, go.ret) };
        let outcome = match outcome {
            Ok(Outcome::Same) => match compare_writes(&env, run, &ms.writes(), &go.writes) {
                Some(d) => Ok(Outcome::Differs(vec![format!("memory at exit: {d}")])),
                None => Ok(Outcome::Same),
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
            Ok(Outcome::Differs(lines)) => {
                return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), Vec::new());
            }
            Ok(Outcome::Same) => {}
        }
        if let Some(w) = upper.as_mut().filter(|_| !mo.noreturn) {
            w.observe(&mut ms);
        }
        agreed += 1;
    }
    let mut suspects = abi::unclaimed_inputs(prog, &env, fd, &claim);
    suspects.extend(call_suspects);
    if agreed == opts.runs {
        if let Some(w) = upper {
            suspects.extend(w.suspects(prog));
        }
    }
    let verdict = match (unknown, diverged) {
        (Some(h), _) => Verdict::from_halt(&h),
        (None, Some(d)) => Verdict::Diverges(d),
        (None, None) => Verdict::Agrees(agreed),
    };
    let lines = match contract {
        Some(c) if matches!(verdict, Verdict::Agrees(_)) => vec![format!("contract {}  ({})", c.declaration, c.source)],
        _ => Vec::new(),
    };
    finish(verdict, lines, suspects)
}
