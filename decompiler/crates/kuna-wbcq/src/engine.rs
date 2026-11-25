//! The loop: learn a hypothesis from the machine (L#), refine the alphabet
//! until every path of every state has a symbol (concolic exploration with
//! z3), certify the hypothesis by path closure, and test it against the
//! machine on random words. A word the hypothesis gets wrong goes back to the
//! learner.
//!
//! **Path closure.** Let `C` be the context bytes that a branch condition or
//! the result depends on, closed under what those bytes' next values depend
//! on. If, from every state's context, the path enumeration is complete; if
//! on every path the result and the next value of every byte of `C` do not
//! depend on the input; and if every transition's next context agrees on `C`
//! with the context of the state it goes to, then every context that agrees
//! with a state's context on `C` takes, on every input, the path the state
//! takes, with the same result and the same next state. The hypothesis then
//! agrees with the machine on every word of every length from the initial
//! context. The proof rests on the p-code semantics, on dependence tracking
//! (an address or jump computed from data stops the step), and on z3's
//! `unsat` answers; an `unknown` answer withholds it.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use kuna_verify::concolic::{solve, Answer, Solver};
use kuna_verify::state::Env;
use serde_json::{json, Value};

use crate::guard;
use crate::learn::{Hypothesis, Oracle, Output, Tree};
use std::rc::Rc;

use kuna_verify::concolic::{Branch, Expr, Path};

use crate::machine::{step, Abi, StepRun, INPUT, INPUT_SPACE, PIN_AT, RESULT};

pub struct Config {
    pub entry: u64,
    pub initial: Vec<u8>,
    pub max_states: usize,
    pub max_paths: usize,
    pub max_rounds: usize,
    pub z3_millis: u32,
    pub words: usize,
    pub length: usize,
    pub seed: u64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            entry: 0,
            initial: vec![0; 8],
            max_states: 32,
            max_paths: 256,
            max_rounds: 64,
            z3_millis: 2000,
            words: 256,
            length: 8,
            seed: 0x2545_f491_4f6c_dd1d,
        }
    }
}

pub struct Machine<'e, 'p> {
    env: &'e Env<'p>,
    abi: &'e Abi,
    entry: u64,
    initial: Vec<u8>,
    pub steps: u64,
}

impl Machine<'_, '_> {
    pub fn run_word(&mut self, word: &[u64]) -> Result<(Vec<Output>, Vec<u8>), String> {
        let mut ctx = self.initial.clone();
        let mut out = Vec::with_capacity(word.len());
        for x in word {
            let r = step(self.env, self.abi, self.entry, &ctx, *x, false, 0)?;
            self.steps += 1;
            out.push(r.result);
            ctx = r.context;
        }
        Ok((out, ctx))
    }

    fn symbolic(&mut self, ctx: &[u8], x: u64, pin: u64) -> Result<StepRun, String> {
        self.steps += 1;
        step(self.env, self.abi, self.entry, ctx, x, true, pin)
    }
}

impl Oracle for Machine<'_, '_> {
    fn query(&mut self, word: &[u64]) -> Result<Vec<Output>, String> {
        Ok(self.run_word(word)?.0)
    }
}

#[derive(Default)]
struct Explored {
    paths: Vec<(u64, StepRun)>,
    queries: u32,
    unknowns: u32,
    capped: bool,
}

type Signature = Vec<(u64, u32, bool)>;

/// Every path of one step from `ctx`: run the seeds, then ask z3, for each
/// branch of each path found, for an input that keeps the branches before it
/// and flips it, once per prefix.
fn explore(
    m: &mut Machine,
    solver: &mut Solver,
    ctx: &[u8],
    seeds: &[u64],
    pin: u64,
    max_paths: usize,
) -> Result<Explored, String> {
    let mut ex = Explored::default();
    let mut todo: VecDeque<u64> = seeds.iter().copied().collect();
    let mut seen: BTreeSet<Signature> = BTreeSet::new();
    let mut tried: BTreeSet<Signature> = BTreeSet::new();
    // A value branch is complete only when no input gives a value outside
    // all those seen after its prefix, so its query excludes all of them.
    let mut values: BTreeMap<Signature, BTreeSet<u128>> = BTreeMap::new();
    while let Some(x) = todo.pop_front() {
        let r = m.symbolic(ctx, x, pin)?;
        if !seen.insert(r.signature.clone()) {
            continue;
        }
        if ex.paths.len() >= max_paths {
            ex.capped = true;
            break;
        }
        for k in 0..r.path.branches.len() {
            let mut key: Signature = r.path.branches[..k]
                .iter()
                .map(|b| (b.at, b.op, b.taken))
                .collect();
            let b = &r.path.branches[k];
            let answer = if b.at & PIN_AT == PIN_AT {
                key.push((b.at, 0, false));
                let Expr::Test(_, e, v) = &*b.cond else {
                    return Err("a value branch without its test".into());
                };
                let Expr::Const(v, w) = **v else {
                    return Err("a value branch without its value".into());
                };
                let seen = values.entry(key).or_default();
                if !seen.insert(v) {
                    continue;
                }
                let any = seen
                    .iter()
                    .map(|s| Rc::new(Expr::Test("=", Rc::clone(e), Rc::new(Expr::Const(*s, w)))))
                    .reduce(|a, b| Rc::new(Expr::Op("bvor", vec![a, b], 8)))
                    .expect("one value");
                let mut query = Path {
                    branches: r.path.branches[..k].to_vec(),
                    inputs: r.path.inputs.clone(),
                };
                query.branches.push(Branch {
                    at: b.at,
                    op: 0,
                    cond: any,
                    taken: true,
                });
                ex.queries += 1;
                solve(solver, &query, k)
            } else {
                key.push((b.at, b.op, !b.taken));
                if !tried.insert(key) {
                    continue;
                }
                ex.queries += 1;
                solve(solver, &r.path, k)
            };
            match answer {
                Answer::Model(model) => {
                    let mut x2 = x;
                    for ((space, off), byte) in model {
                        if space == INPUT_SPACE && off < 8 {
                            x2 = (x2 & !(0xff << (8 * off))) | (u64::from(byte) << (8 * off));
                        }
                    }
                    todo.push_back(x2);
                }
                Answer::Unsat => {}
                Answer::Unknown => ex.unknowns += 1,
                Answer::Broken => return Err("z3 stopped answering".into()),
            }
        }
        ex.paths.push((x, r));
    }
    Ok(ex)
}

/// What the certificate concluded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Closure {
    Proved,
    /// Not proved; the reason. The hypothesis may still be right.
    Withheld(String),
}

/// Per state: its context and each alphabet symbol's run from it.
pub struct StateRuns {
    pub context: Vec<u8>,
    pub runs: Vec<StepRun>,
    pub complete: Result<(), String>,
}

fn project(ctx: &[u8], control: u64) -> Vec<(usize, u8)> {
    ctx.iter()
        .enumerate()
        .filter(|(i, _)| control & (1 << i) != 0)
        .map(|(i, b)| (i, *b))
        .collect()
}

/// The control bytes: what a branch or the result reads, closed under what
/// their next values read.
pub fn control_bytes(states: &[StateRuns]) -> u64 {
    let runs = || states.iter().flat_map(|s| &s.runs);
    let mut c = runs().fold(0, |c, r| c | r.control_reads | (r.result_dep & !INPUT));
    loop {
        let mut next = c;
        for r in runs() {
            for (j, dep) in r.context_dep.iter().enumerate() {
                if c & (1 << j) != 0 {
                    next |= dep & !INPUT;
                }
            }
        }
        if next == c {
            return c;
        }
        c = next;
    }
}

/// Path closure of `h` over the runs of its states (module documentation).
pub fn closure(h: &Hypothesis, states: &[StateRuns]) -> Closure {
    for (q, s) in states.iter().enumerate() {
        if let Err(why) = &s.complete {
            return Closure::Withheld(format!("state {q}: {why}"));
        }
    }
    let c = control_bytes(states);
    for (q, s) in states.iter().enumerate() {
        for (a, r) in s.runs.iter().enumerate() {
            if r.result_dep & INPUT != 0 && r.pinned & RESULT == 0 {
                return Closure::Withheld(format!(
                    "state {q}, symbol {a}: the result depends on the input's value"
                ));
            }
            if let Some(j) = (0..r.context_dep.len()).find(|j| {
                c & (1 << j) != 0 && r.context_dep[*j] & INPUT != 0 && r.pinned & (1 << j) == 0
            }) {
                return Closure::Withheld(format!(
                    "state {q}, symbol {a}: control byte {j} takes a value from the input"
                ));
            }
            let (out, t) = h.delta[q][a];
            if out != r.result {
                return Closure::Withheld(format!(
                    "state {q}, symbol {a}: the hypothesis says {out:#x}, the machine {:#x}",
                    r.result
                ));
            }
            if project(&r.context, c) != project(&states[t].context, c) {
                return Closure::Withheld(format!(
                    "state {q}, symbol {a}: the next control bytes are not state {t}'s"
                ));
            }
        }
    }
    Closure::Proved
}

/// The alphabet symbol an input stands for in state `q`: the first whose
/// path condition it satisfies.
fn classify(states: &[StateRuns], q: usize, x: u64) -> Option<usize> {
    states[q]
        .runs
        .iter()
        .position(|r| guard::holds(&r.path.branches, x) == Some(true))
}

fn splitmix(z: &mut u64) -> u64 {
    *z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut x = *z;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Random words: uniform 32-bit inputs, or drawn from the alphabet.
pub fn words(cfg: &Config, alphabet: &[u64], from_alphabet: bool) -> Vec<Vec<u64>> {
    let mut z = cfg.seed ^ u64::from(from_alphabet);
    (0..cfg.words)
        .map(|_| {
            (0..cfg.length)
                .map(|_| {
                    let r = splitmix(&mut z);
                    if from_alphabet {
                        alphabet[(r % alphabet.len() as u64) as usize]
                    } else {
                        r & 0xffff_ffff
                    }
                })
                .collect()
        })
        .collect()
}

/// The first word on which the machine and `h` (inputs classified by its
/// states' path conditions) differ, and the shortest prefix that shows it.
pub fn conformance(
    m: &mut Machine,
    h: &Hypothesis,
    states: &[StateRuns],
    words: &[Vec<u64>],
) -> Result<Option<Vec<u64>>, String> {
    for w in words {
        let (real, _) = m.run_word(w)?;
        let mut q = 0;
        for (i, x) in w.iter().enumerate() {
            let agrees = match classify(states, q, *x) {
                Some(a) => {
                    let (o, t) = h.delta[q][a];
                    q = t;
                    o == real[i]
                }
                None => false,
            };
            if !agrees {
                return Ok(Some(w[..=i].to_vec()));
            }
        }
    }
    Ok(None)
}

/// How many control valuations the alphabet reaches from the initial context,
/// counting up to `limit`.
fn reachable_valuations(
    m: &mut Machine,
    tree: &Tree,
    control: u64,
    limit: usize,
) -> Result<usize, String> {
    let mut seen = BTreeSet::new();
    let mut todo = VecDeque::from([m.initial.clone()]);
    seen.insert(project(&m.initial, control));
    while let Some(ctx) = todo.pop_front() {
        for x in &tree.alphabet {
            let r = step(m.env, m.abi, m.entry, &ctx, *x, false, 0)?;
            m.steps += 1;
            if seen.insert(project(&r.context, control)) {
                if seen.len() >= limit {
                    return Ok(seen.len());
                }
                todo.push_back(r.context);
            }
        }
    }
    Ok(seen.len())
}

pub struct Learned {
    pub tree: Tree,
    pub hypothesis: Option<Hypothesis>,
    pub states: Vec<StateRuns>,
    pub closure: Closure,
    pub status: String,
    pub rounds: usize,
    pub z3_queries: u32,
    pub z3_unknowns: u32,
    pub paths: Vec<usize>,
    pub conformance_words: usize,
}

fn state_runs(
    m: &mut Machine,
    tree: &Tree,
    ex_complete: &[Result<(), String>],
    pin: u64,
) -> Result<Vec<StateRuns>, String> {
    let mut out = Vec::new();
    for (q, &b) in tree.basis.iter().enumerate() {
        let (_, context) = m.run_word(&tree.symbols(&tree.access(b)))?;
        let runs = tree
            .alphabet
            .iter()
            .map(|x| m.symbolic(&context, *x, pin))
            .collect::<Result<Vec<_>, _>>()?;
        out.push(StateRuns {
            context,
            runs,
            complete: ex_complete[q].clone(),
        });
    }
    Ok(out)
}

pub fn learn<'e, 'p>(
    env: &'e Env<'p>,
    abi: &'e Abi,
    cfg: &Config,
) -> Result<(Learned, Machine<'e, 'p>), String> {
    let mut m = Machine {
        env,
        abi,
        entry: cfg.entry,
        initial: cfg.initial.clone(),
        steps: 0,
    };
    let mut solver = Solver::start(cfg.z3_millis).ok_or("z3 could not be started")?;
    let mut tree = Tree::new(vec![0]);
    // What exploration pins: the result, and the control bytes as far as the
    // last round's runs show them.
    let mut pin = RESULT;
    let mut out = Learned {
        tree: Tree::new(Vec::new()),
        hypothesis: None,
        states: Vec::new(),
        closure: Closure::Withheld("not reached".into()),
        status: String::new(),
        rounds: 0,
        z3_queries: 0,
        z3_unknowns: 0,
        paths: Vec::new(),
        conformance_words: 0,
    };
    loop {
        out.rounds += 1;
        if out.rounds > cfg.max_rounds {
            out.status = "partial_frontier: round budget".into();
            break;
        }
        let Some(h) = tree.stabilize(&mut m, cfg.max_states)? else {
            out.status = format!("partial_frontier: more than {} states", cfg.max_states);
            break;
        };
        let mut new = Vec::new();
        let mut complete = Vec::new();
        out.paths.clear();
        // The basis contexts first, then every context one symbol past them:
        // a frontier context whose paths need a symbol the alphabet lacks is
        // where a state the alphabet cannot yet tell apart hides.
        let mut contexts: Vec<(bool, Vec<u8>)> = Vec::new();
        for &b in &tree.basis.clone() {
            let access = tree.symbols(&tree.access(b));
            contexts.insert(
                contexts.iter().filter(|(basis, _)| *basis).count(),
                (true, m.run_word(&access)?.1),
            );
            for x in tree.alphabet.clone() {
                let mut w = access.clone();
                w.push(x);
                let ctx = m.run_word(&w)?.1;
                if !contexts.iter().any(|(_, c)| *c == ctx) {
                    contexts.push((false, ctx));
                }
            }
        }
        let mut capped = false;
        for (basis, ctx) in contexts {
            let ex = explore(
                &mut m,
                &mut solver,
                &ctx,
                &tree.alphabet,
                pin,
                cfg.max_paths,
            )?;
            if ex.capped {
                capped = true;
                break;
            }
            out.z3_queries += ex.queries;
            out.z3_unknowns += ex.unknowns;
            out.paths.push(ex.paths.len());
            let covered: BTreeSet<Signature> = tree
                .alphabet
                .iter()
                .map(|x| m.symbolic(&ctx, *x, pin).map(|r| r.signature))
                .collect::<Result<_, _>>()?;
            for (x, r) in &ex.paths {
                if !covered.contains(&r.signature) && !new.contains(x) {
                    new.push(*x);
                }
            }
            if !basis {
                continue;
            }
            complete.push(if ex.capped {
                Err(format!("more than {} paths", cfg.max_paths))
            } else if ex.unknowns > 0 {
                Err(format!("z3 answered unknown {} times", ex.unknowns))
            } else if let Some(why) = ex.paths.iter().find_map(|(_, r)| r.incomplete.clone()) {
                Err(why)
            } else {
                Ok(())
            });
        }
        if capped {
            out.status = format!(
                "partial_frontier: more than {} paths in one step",
                cfg.max_paths
            );
            break;
        }
        if !new.is_empty() {
            for x in new {
                tree.symbol(x);
            }
            continue;
        }
        let states = state_runs(&mut m, &tree, &complete, pin)?;
        let control = control_bytes(&states) | RESULT;
        if control & !pin != 0 {
            pin |= control;
            continue;
        }
        let closure = closure(&h, &states);
        let mut cex = None;
        for from_alphabet in [false, true] {
            let ws = words(cfg, &tree.alphabet, from_alphabet);
            out.conformance_words += ws.len();
            if let Some(w) = conformance(&mut m, &h, &states, &ws)? {
                cex = Some(w);
                break;
            }
        }
        if let Some(w) = cex {
            if closure == Closure::Proved {
                return Err(format!(
                    "the closure proof and the machine disagree on {w:x?}: a bug in the prototype"
                ));
            }
            let known = w.iter().all(|x| tree.alphabet.contains(x));
            let word = w.iter().map(|x| tree.symbol(*x)).collect::<Vec<_>>();
            if known {
                tree.counterexample(&mut m, &h, &word)?;
            } else {
                tree.query(&mut m, &word)?;
            }
            continue;
        }
        out.status = match &closure {
            Closure::Proved => "proved_by_path_closure".into(),
            Closure::Withheld(why) => {
                match reachable_valuations(&mut m, &tree, control, cfg.max_states + 1)? {
                    n if n > cfg.max_states => format!(
                        "partial_frontier: more than {} control valuations reachable ({why})",
                        cfg.max_states
                    ),
                    _ => format!("empirically_consistent: {why}"),
                }
            }
        };
        out.closure = closure;
        out.states = states;
        out.hypothesis = Some(h);
        break;
    }
    out.tree = tree;
    Ok((out, m))
}

/// The learned machine as JSON: states, transitions with guards (symbols
/// with the same result and target merged), witnesses, control and data
/// bytes, evidence.
pub fn report(l: &Learned, m: &Machine) -> Value {
    let t = &l.tree;
    let Some(h) = &l.hypothesis else {
        return json!({ "status": l.status, "states_seen": t.basis.len(), "alphabet": t.alphabet,
            "membership_queries": t.queries, "machine_steps": m.steps, "rounds": l.rounds });
    };
    let control = control_bytes(&l.states);
    let written = l.states.iter().flat_map(|s| &s.runs).fold(0u64, |acc, r| {
        acc | r
            .context_dep
            .iter()
            .enumerate()
            .fold(0, |a, (j, d)| if *d != 1 << j { a | 1 << j } else { a })
    });
    let bytes = |set: u64| {
        (0..63)
            .filter(|j| set & (1 << j) != 0)
            .collect::<Vec<u32>>()
    };
    let mut transitions = Vec::new();
    for q in 0..h.states {
        let mut merged: BTreeMap<(Output, usize), Vec<String>> = BTreeMap::new();
        for (a, (o, to)) in h.delta[q].iter().enumerate() {
            merged
                .entry((*o, *to))
                .or_default()
                .push(guard::render(&l.states[q].runs[a].path.branches));
        }
        let whole = merged.len() == 1;
        for ((o, to), guards) in merged {
            let guards: BTreeSet<String> = guards.into_iter().collect();
            let guard = if whole {
                "true".to_string()
            } else {
                guards.into_iter().collect::<Vec<_>>().join(" || ")
            };
            transitions.push(json!({ "from": q, "to": to, "result": o, "guard": guard }));
        }
    }
    json!({
        "status": l.status,
        "states": (0..h.states).map(|q| json!({
            "id": q,
            "access": t.symbols(&t.access(t.basis[q])),
            "control_valuation": project(&l.states[q].context, control),
        })).collect::<Vec<_>>(),
        "transitions": transitions,
        "witnesses": t.witnesses().iter().map(|(a, b, w)| json!({ "a": a, "b": b, "word": t.symbols(w) })).collect::<Vec<_>>(),
        "control_bytes": bytes(control),
        "data_bytes": bytes(written & !control),
        "alphabet": t.alphabet,
        "evidence": {
            "rounds": l.rounds,
            "membership_queries": t.queries,
            "symbols_run": t.symbols_run,
            "machine_steps": m.steps,
            "z3_queries": l.z3_queries,
            "z3_unknowns": l.z3_unknowns,
            "paths_per_state": l.paths,
            "conformance_words": l.conformance_words,
        },
    })
}
