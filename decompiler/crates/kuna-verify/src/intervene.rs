//! CSIC: counterfactual interventions on a run that differs.
//!
//! The idea is Fission RFC #1101's (Causal Structural Intervention Calculus);
//! the code is kuna's own. A `DIFFERS` run is replayed with one change made to
//! kuna's side at a time, and each change after which that run agrees is a
//! counterfactual witness. Two families of change:
//!
//! - **value**: every execution of one op takes the value the machine holds,
//!   after that op's instruction, in the storage the op writes. It needs the
//!   op's output to name machine state (a register, a stack slot, memory), and
//!   holds only for the recorded run, whose machine values it uses.
//! - **structural**: one op's opcode becomes another of its family, its two
//!   inputs trade places, its branch goes the other way, or one constant moves
//!   by one. It holds for every run, so a witness on the recorded run is
//!   replayed over all of them.
//!
//! A witness says the run depends on that op; it does not say the op is the
//! only cause, nor the smallest one. No joint interventions or contingencies
//! are searched, so minimality is never claimed.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Instant;

use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::{get_opname, OpCode};

use crate::check::Options;
use crate::mutate::Mutation;
use crate::program::{Exec, Kind, Operand, Program};
use crate::report::{Report, Verdict};
use crate::state::State;
use crate::storage::{hex, Storage};
use crate::value::mask;

/// The machine side's view of kuna's outputs: after each execution of an
/// instruction kuna's side writes storage at, the values of that storage.
#[derive(Default)]
pub struct Probe {
    want: BTreeMap<u64, Vec<Storage>>,
    got: BTreeMap<(u64, u32), Vec<Option<u128>>>,
    counts: BTreeMap<u64, u32>,
    open: Vec<Option<(u64, u32)>>,
}

impl Probe {
    /// An instruction at `addr` starts at call depth `depth`: the one before
    /// it at that depth has finished.
    pub(crate) fn start(&mut self, state: &mut State, depth: usize, addr: u64) {
        self.open.truncate(depth + 1);
        self.open.resize(depth + 1, None);
        if let Some((a, n)) = self.open[depth].take() {
            self.capture(state, a, n);
        }
        if self.want.contains_key(&addr) {
            let c = self.counts.entry(addr).or_default();
            self.open[depth] = Some((addr, *c));
            *c += 1;
        }
    }

    fn capture(&mut self, state: &mut State, addr: u64, nth: u32) {
        let Some(want) = self.want.get(&addr) else { return };
        let live = state.live.take();
        let reads = state.frame_reads.take();
        let flags = (state.read_uninitialized, state.read_saved);
        let vals = want.iter().map(|s| state.read_storage(&s.space, s.off, s.size).ok()).collect();
        (state.live, state.frame_reads) = (live, reads);
        (state.read_uninitialized, state.read_saved) = flags;
        self.got.insert((addr, nth), vals);
    }
}

/// Storage the machine can hold: a register, a stack slot, memory.
fn machine_state(s: &Storage) -> bool {
    matches!(s.space.get_type(), spacetype::IPTR_PROCESSOR | spacetype::IPTR_SPACEBASE | spacetype::IPTR_JOIN)
}

/// The storage kuna's ops write, by the instruction they come from.
fn wanted(p: &Program) -> BTreeMap<u64, Vec<Storage>> {
    let mut want: BTreeMap<u64, Vec<Storage>> = BTreeMap::new();
    for op in p.blocks.iter().flat_map(|b| &b.ops) {
        let Some(st) = op.out.as_ref().and_then(|o| o.storage.clone()).filter(machine_state) else { continue };
        let v = want.entry(op.at).or_default();
        if !v.iter().any(|s| same(s, &st)) {
            v.push(st);
        }
    }
    want
}

fn same(a: &Storage, b: &Storage) -> bool {
    a.space.get_index() == b.space.get_index() && a.off == b.off && a.size == b.size
}

/// One change to kuna's side.
#[derive(Clone, Debug)]
pub enum Do {
    /// Every execution of the op takes the machine's value (`None` leaves
    /// that execution alone). `first` is the first execution that differed.
    Value { block: usize, op: usize, values: Vec<Option<u128>>, first: usize, kuna: u128, machine: u128, storage: String },
    Opcode { block: usize, op: usize, to: OpCode },
    Swap { block: usize, op: usize },
    Branch { block: usize, op: usize },
    Constant { block: usize, op: usize, input: usize, to: u128 },
}

impl Do {
    pub fn place(&self) -> (usize, usize) {
        match *self {
            Do::Value { block, op, .. }
            | Do::Opcode { block, op, .. }
            | Do::Swap { block, op }
            | Do::Branch { block, op }
            | Do::Constant { block, op, .. } => (block, op),
        }
    }

    pub fn family(&self) -> &'static str {
        match self {
            Do::Value { .. } => "value",
            _ => "structural",
        }
    }

    fn describe(&self, p: &Program) -> String {
        let op = &p.blocks[self.place().0].ops[self.place().1];
        match self {
            Do::Value { values, first, kuna, machine, storage, .. } => {
                let n = values.iter().filter(|v| v.is_some()).count();
                let size = op.out.as_ref().map_or(8, |o| o.size);
                format!(
                    "do({storage} := machine's value) at each of {n} executions; execution {} had kuna {} machine {}",
                    first + 1,
                    hex(*kuna, size),
                    hex(*machine, size)
                )
            }
            Do::Opcode { to, .. } => format!("do({} -> {})", name(op), get_opname(*to)),
            Do::Swap { .. } => format!("do({} inputs swapped)", name(op)),
            Do::Branch { .. } => "do(branch inverted)".to_string(),
            Do::Constant { input, to, .. } => {
                let was = match op.ins.get(*input).map(|a| &a.op) {
                    Some(Operand::Const(c)) => *c,
                    _ => 0,
                };
                format!("do(constant {was:#x} -> {to:#x}, input {input} of {})", name(op))
            }
        }
    }
}

fn name(op: &crate::program::Op) -> String {
    match &op.kind {
        Kind::Eval(o) => get_opname(*o).to_string(),
        Kind::Load(_) => "LOAD".into(),
        Kind::Store(_) => "STORE".into(),
        Kind::CBranch { .. } => "CBRANCH".into(),
        Kind::Call { .. } => "CALL".into(),
        Kind::CallOther { name, .. } => format!("CALLOTHER {name}"),
        Kind::Indirect(_) | Kind::IndirectBefore => "INDIRECT".into(),
        other => format!("{other:?}").split([' ', '{', '(']).next().unwrap_or("op").to_uppercase(),
    }
}

/// Make the change on `p`; false when it does not fit the op.
pub(crate) fn apply(p: &mut Program, d: &Do) -> bool {
    let (b, k) = d.place();
    if let Do::Value { values, .. } = d {
        p.force = Some((b, k, values.clone()));
        return true;
    }
    let Some(op) = p.blocks.get_mut(b).and_then(|bl| bl.ops.get_mut(k)) else { return false };
    match (d, &mut op.kind) {
        (Do::Opcode { to, .. }, Kind::Eval(opc)) => *opc = *to,
        (Do::Swap { .. }, Kind::Eval(_)) if op.ins.len() == 2 => op.ins.swap(0, 1),
        (Do::Branch { .. }, Kind::CBranch { flip }) => *flip = !*flip,
        (Do::Constant { input, to, .. }, Kind::Eval(_)) => match op.ins.get_mut(*input) {
            Some(a) if matches!(a.op, Operand::Const(_)) => a.op = Operand::Const(*to),
            _ => return false,
        },
        _ => return false,
    }
    true
}

/// What `check_runs` takes from an intervention search and hands back.
#[derive(Default)]
pub struct Hook {
    /// Make only this run.
    pub run: Option<u32>,
    pub apply: Option<Do>,
    /// Record both sides of the run: kuna's values op by op, and the
    /// machine's at the same instructions.
    pub observe: bool,
    pub observed: Option<Observed>,
}

pub struct Observed {
    pub program: Program,
    pub execs: Vec<Exec>,
    want: BTreeMap<u64, Vec<Storage>>,
    machine: BTreeMap<(u64, u32), Vec<Option<u128>>>,
}

impl Hook {
    pub(crate) fn prepare(&self, p: &mut Program) -> Option<Rc<RefCell<Vec<Exec>>>> {
        let rec = self.observe.then(|| Rc::new(RefCell::new(Vec::new())));
        p.record = rec.clone();
        rec
    }

    pub(crate) fn probe(&self, p: &Program) -> Option<Probe> {
        self.observe.then(|| Probe { want: wanted(p), ..Probe::default() })
    }

    pub(crate) fn keep(&mut self, p: &Program, probe: Option<Probe>, rec: Option<&Rc<RefCell<Vec<Exec>>>>) {
        if self.observed.is_some() {
            return;
        }
        if let (Some(probe), Some(rec)) = (probe, rec) {
            let execs = std::mem::take(&mut *rec.borrow_mut());
            self.observed = Some(Observed { program: p.clone(), execs, want: probe.want, machine: probe.got });
        }
    }
}

/// A change after which the recorded run agrees.
#[derive(Clone, Debug)]
pub struct Witness {
    pub change: Do,
    pub at: u64,
    pub op: String,
    pub what: String,
    /// Where the op first ran in the recorded run, among all executed ops.
    pub order: usize,
    /// A structural change replayed over every run: the verdict.
    pub all_runs: Option<String>,
    /// The rule application `localize` names at this address, when known.
    pub rule: Option<String>,
}

/// A search's result for one decompile.
pub struct Csic {
    pub function: String,
    pub base: Report,
    pub run: Option<u32>,
    pub first_mismatch: Option<String>,
    pub witnesses: Vec<Witness>,
    pub value_candidates: usize,
    pub structural_candidates: usize,
    pub replays: usize,
    /// Candidates left untried when the replay budget ran out.
    pub untried: usize,
    pub ms: f64,
}

/// Replays of the recorded run a search makes at most.
pub const MAX_REPLAYS: usize = 400;

/// Structural witnesses replayed over every run.
const MAX_CONFIRMS: usize = 8;

const FAMILIES: &[&[OpCode]] = {
    use OpCode::*;
    &[
        &[CPUI_INT_ADD, CPUI_INT_SUB, CPUI_INT_MULT, CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_XOR],
        &[CPUI_INT_LESS, CPUI_INT_SLESS, CPUI_INT_LESSEQUAL, CPUI_INT_SLESSEQUAL],
        &[CPUI_INT_EQUAL, CPUI_INT_NOTEQUAL],
        &[CPUI_INT_LEFT, CPUI_INT_RIGHT, CPUI_INT_SRIGHT],
        &[CPUI_INT_ZEXT, CPUI_INT_SEXT],
        &[CPUI_INT_DIV, CPUI_INT_SDIV],
        &[CPUI_INT_REM, CPUI_INT_SREM],
    ]
};

fn structural(p: &Program, b: usize, k: usize) -> Vec<Do> {
    let op = &p.blocks[b].ops[k];
    let mut out = Vec::new();
    match &op.kind {
        Kind::Eval(opc) => {
            if let Some(fam) = FAMILIES.iter().find(|f| f.contains(opc)) {
                out.extend(fam.iter().filter(|o| *o != opc).map(|&to| Do::Opcode { block: b, op: k, to }));
            }
            if crate::mutate::ordered(*opc) && op.ins.len() == 2 && op.ins[0].size == op.ins[1].size {
                out.push(Do::Swap { block: b, op: k });
            }
            for (i, a) in op.ins.iter().enumerate() {
                let Operand::Const(c) = a.op else { continue };
                let m = mask(a.size.max(1));
                let mut tos = vec![(c ^ 1) & m, c.wrapping_add(1) & m, c.wrapping_sub(1) & m];
                tos.sort_unstable();
                tos.dedup();
                out.extend(tos.into_iter().filter(|&t| t != c).map(|to| Do::Constant { block: b, op: k, input: i, to }));
            }
        }
        Kind::CBranch { .. } => out.push(Do::Branch { block: b, op: k }),
        _ => {}
    }
    out
}

/// Where an op's output first differed: the execution among all ops, the
/// execution of this op, kuna's value and the machine's.
type FirstDiff = (usize, usize, u128, u128);

/// Each op's machine value per execution, and where it first differed.
type PerOp = BTreeMap<(usize, usize), (Vec<Option<u128>>, Option<FirstDiff>)>;

/// Value candidates: each op whose output differs from the machine's value
/// after its instruction, in the order its first difference ran.
fn values(o: &Observed) -> Vec<(usize, Do)> {
    let mut nth: BTreeMap<(u64, i32, u64, u32), u32> = BTreeMap::new();
    let mut per_op: PerOp = BTreeMap::new();
    for (i, e) in o.execs.iter().enumerate() {
        let op = &o.program.blocks[e.block].ops[e.op];
        let Some(out) = op.out.as_ref() else { continue };
        let Some(st) = out.storage.as_ref().filter(|s| machine_state(s)) else { continue };
        let Some(slot) = o.want.get(&op.at).and_then(|w| w.iter().position(|s| same(s, st))) else { continue };
        let n = nth.entry((op.at, st.space.get_index(), st.off, st.size)).or_default();
        let machine = o.machine.get(&(op.at, *n)).and_then(|v| v.get(slot).copied().flatten()).map(|v| v & mask(out.size));
        *n += 1;
        let entry = per_op.entry((e.block, e.op)).or_default();
        let j = entry.0.len();
        entry.0.push(machine);
        if let (Some(m), Some(k)) = (machine, e.value.map(|v| v & mask(out.size))) {
            if m != k && entry.1.is_none() {
                entry.1 = Some((i, j, k, m));
            }
        }
    }
    let mut out: Vec<(usize, Do)> = per_op
        .into_iter()
        .filter_map(|((block, op), (values, first))| {
            let (i, first, kuna, machine) = first?;
            let storage = o.program.blocks[block].ops[op].out.as_ref()?.storage.as_ref()?.clone();
            let storage = format!("{}:{:#x}:{}", storage.space.get_name(), storage.off, storage.size);
            Some((i, Do::Value { block, op, values, first, kuna, machine, storage }))
        })
        .collect();
    out.sort_by_key(|(i, _)| *i);
    out
}

fn first_mismatch(r: &Report) -> Option<String> {
    let skip = ["input", "contract", "concolic", "note", "trace"];
    let i = r.lines.iter().position(|l| !skip.iter().any(|s| l.starts_with(s)))?;
    let squeeze = |l: &str| l.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut s = squeeze(&r.lines[i]);
    if s.starts_with("machine") {
        if let Some(k) = r.lines.get(i + 1).filter(|l| l.starts_with("kuna")) {
            s = format!("{s}; {}", squeeze(k));
        }
    }
    Some(s)
}

/// Search one decompile, with `mutation` planted when given.
pub fn search(prog: &ConsoleProgram, fd: &Funcdata, opts: &Options, mutation: Option<Mutation>) -> Csic {
    let started = Instant::now();
    let opts = Options { concolic: 0, ..*opts };
    let mut observe = Hook { observe: true, ..Hook::default() };
    let base = crate::check::check_hooked(prog, fd, &opts, mutation, None);
    let mut csic = Csic {
        function: fd.get_name().to_string(),
        first_mismatch: first_mismatch(&base),
        run: None,
        base,
        witnesses: Vec::new(),
        value_candidates: 0,
        structural_candidates: 0,
        replays: 0,
        untried: 0,
        ms: 0.0,
    };
    let Verdict::Differs { run, .. } = csic.base.verdict else {
        csic.ms = started.elapsed().as_secs_f64() * 1e3;
        return csic;
    };
    csic.run = Some(run);
    observe.run = Some(run);
    crate::check::check_hooked(prog, fd, &opts, mutation, Some(&mut observe));
    let Some(obs) = observe.observed else {
        csic.ms = started.elapsed().as_secs_f64() * 1e3;
        return csic;
    };
    let mut order: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (i, e) in obs.execs.iter().enumerate() {
        order.entry((e.block, e.op)).or_insert(i);
    }
    let value = values(&obs);
    let mut ran: Vec<(usize, usize)> = order.keys().copied().collect();
    ran.sort_by_key(|p| order[p]);
    let structs: Vec<Do> = ran.iter().flat_map(|&(b, k)| structural(&obs.program, b, k)).collect();
    csic.value_candidates = value.len();
    csic.structural_candidates = structs.len();
    let candidates: Vec<Do> = value.into_iter().map(|(_, d)| d).chain(structs).collect();
    let mut confirms = 0usize;
    for (n, d) in candidates.iter().enumerate() {
        if csic.replays >= MAX_REPLAYS {
            csic.untried = candidates.len() - n;
            break;
        }
        csic.replays += 1;
        let mut hook = Hook { run: Some(run), apply: Some(d.clone()), ..Hook::default() };
        let r = crate::check::check_hooked(prog, fd, &opts, mutation, Some(&mut hook));
        if !matches!(r.verdict, Verdict::Agrees(_)) {
            continue;
        }
        let all_runs = (d.family() == "structural" && confirms < MAX_CONFIRMS).then(|| {
            confirms += 1;
            let mut every = Hook { apply: Some(d.clone()), ..Hook::default() };
            let r = crate::check::check_hooked(prog, fd, &opts, mutation, Some(&mut every));
            match r.verdict {
                Verdict::Differs { run, of } => format!("DIFFERS (run {run} of {of})"),
                v => v.label(),
            }
        });
        let (b, k) = d.place();
        let op = &obs.program.blocks[b].ops[k];
        csic.witnesses.push(Witness {
            change: d.clone(),
            at: op.at,
            op: name(op),
            what: d.describe(&obs.program),
            order: order.get(&(b, k)).copied().unwrap_or(usize::MAX),
            all_runs,
            rule: None,
        });
    }
    csic.witnesses.sort_by_key(|w| w.order);
    csic.ms = started.elapsed().as_secs_f64() * 1e3;
    csic
}

impl Csic {
    pub fn causal_status(&self) -> &'static str {
        if self.witnesses.is_empty() {
            "NONE"
        } else {
            "WITNESS_FOUND"
        }
    }

    pub fn render(&self, head: &str) -> String {
        let Some(run) = self.run else {
            return format!("{head}  does not differ: {}\n", self.base.verdict.label());
        };
        let mut s = format!("{head}  DIFFERS (run {run})\n");
        s.push_str(&format!("  first_observable_mismatch  {}\n", self.first_mismatch.as_deref().unwrap_or("?")));
        s.push_str(&format!(
            "  searched  {} value and {} structural candidates; {} replays of run {run}{}\n",
            self.value_candidates,
            self.structural_candidates,
            self.replays,
            if self.untried > 0 { format!(", {} left untried (budget)", self.untried) } else { String::new() }
        ));
        s.push_str(&format!("  causal_status  {}\n", self.causal_status()));
        s.push_str("  minimality  NOT_PROVEN\n");
        for (i, w) in self.witnesses.iter().enumerate() {
            let (b, k) = w.change.place();
            let rule = w.rule.as_ref().map(|r| format!("  [rule {r}]")).unwrap_or_default();
            s.push_str(&format!("  witness {}  ({})\n", i + 1, w.change.family()));
            s.push_str(&format!("    candidate_origin  {:#x} {} (block {b}, op {k}){rule}\n", w.at, w.op));
            s.push_str(&format!("    intervention  {}\n", w.what));
            let all = w.all_runs.as_ref().map(|a| format!("; over every run: {a}")).unwrap_or_default();
            s.push_str(&format!("    counterfactual_replay  AGREES (run {run}){all}\n"));
        }
        s
    }

    pub fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "function": self.function,
            "verdict": self.base.verdict.label(),
            "run": self.run,
            "first_observable_mismatch": self.first_mismatch,
            "causal_status": self.causal_status(),
            "minimality": "NOT_PROVEN",
            "value_candidates": self.value_candidates,
            "structural_candidates": self.structural_candidates,
            "replays": self.replays,
            "untried": self.untried,
            "ms": self.ms,
            "witnesses": self.witnesses.iter().map(|w| {
                let (b, k) = w.change.place();
                serde_json::json!({
                    "family": w.change.family(),
                    "candidate_origin": {"address": format!("{:#x}", w.at), "op": w.op, "block": b, "index": k, "rule": w.rule},
                    "intervention": w.what,
                    "counterfactual_replay": "AGREES",
                    "all_runs": w.all_runs,
                })
            }).collect::<Vec<_>>(),
        })
    }
}

/// Search the `pass`-th decompile (from 1) of `function`, or by default the
/// first of its decompiles that differs: its index from 0, and the search.
pub fn search_source(
    src: &crate::localize::Source,
    roots: &[String],
    function: &str,
    pass: Option<usize>,
    opts: &Options,
) -> Result<(usize, Csic), String> {
    let mut seen = 0usize;
    let mut found: Option<(usize, Csic)> = None;
    let mut first: Option<(usize, Csic)> = None;
    let each = |prog: &ConsoleProgram, fd: &Funcdata| {
        if fd.get_name() != function {
            return;
        }
        let i = seen;
        seen += 1;
        if found.is_some() || pass.is_some_and(|p| p != i + 1) {
            return;
        }
        let c = search(prog, fd, opts, None);
        if pass.is_some() || c.run.is_some() {
            found = Some((i, c));
        } else if first.is_none() {
            first = Some((i, c));
        }
    };
    match src {
        crate::localize::Source::Datatest(path) => {
            let (prog, commands) = crate::session::load_datatest(path, roots)?;
            crate::session::drive(commands, Some(prog), roots, each)?;
        }
        crate::localize::Source::Binary { path, functions, options, .. } => {
            crate::session::run(crate::session::binary_script(path, functions, options), None, roots, true, None, each)?;
        }
    }
    found.or(first).ok_or_else(|| format!("no decompile of {function}"))
}

/// Name, on each witness at an address `localize` blames, the rule
/// application it names there.
pub fn attribute(c: &mut Csic, l: &crate::localize::Localized) {
    let crate::localize::Finding::Rules { culprits, .. } = &l.finding else { return };
    for w in &mut c.witnesses {
        if let Some(r) = culprits.iter().find(|r| r.addr == Some(w.at)) {
            w.rule = Some(format!("{} application {} of {}", r.rule, r.k, r.total));
        }
    }
}
