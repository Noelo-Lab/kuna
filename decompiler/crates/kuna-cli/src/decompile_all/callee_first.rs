//! Callee-first execution and whole-program feedback rounds.
//!
//! Planning stays in callgraph; this owner applies the plan, collects caller
//! votes, parks callbacks and converges shared types while retaining target order.

use kuna_console::engine::{ConsoleProgram, FunctionEntry};
use kuna_console::project::FuncResult;
use std::collections::{BTreeMap, BTreeSet};

use super::Args;
use crate::callgraph::{callee_first_plan, CallGraph};

/// Execute the callee-first plan using decompile-all's output options.
/// Results retain target order. A missing call graph falls back to target order
/// without prototype parking and warns only for an explicit request.
pub(super) fn decompile_entries_callee_first(
    prog: &mut ConsoleProgram,
    args: &Args,
    targets: Vec<FunctionEntry>,
    explicit: bool,
) -> Vec<FuncResult> {
    let base = kuna_console::project::DecompileOptions {
        no_vars: args.no_vars,
        want_proto: false,
        want_provenance: args.json,
        want_callee_hints: false,
        header_carries_types: false,
        park_recovered_proto: false,
        single_target: targets.len() == 1,
        want_tokens: false,
        want_pointer_arguments: true,
    };
    decompile_callee_first(prog, args, targets, explicit, base)
}

/// (kuna `protoorder`) [`decompile_entries_callee_first`] for a driver that
/// renders something other than `decompile-all`'s own payload: the order, the
/// park and the convergence sweep are the same, only the per-function
/// [`kuna_console::project::DecompileOptions`] differ.
pub(crate) fn decompile_callee_first(
    prog: &mut ConsoleProgram,
    args: &Args,
    targets: Vec<FunctionEntry>,
    explicit: bool,
    base: kuna_console::project::DecompileOptions,
) -> Vec<FuncResult> {
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget = Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }
    let cycles = prog.arch().protoorder.states_in_cycles();
    let graph = CallGraph::build(prog, &args.binary, args.slice_pref());
    let plan = match &graph {
        Ok(graph) => callee_first_plan(graph, &targets, cycles),
        Err(e) => {
            if explicit {
                eprintln!(
                    "warning: --option protoorder: no call graph for {}: {e}",
                    args.binary
                );
            }
            (0..targets.len()).map(|i| (i, false)).collect()
        }
    };
    // (kuna `calleevote`, `callbacktype`) Both ask the image the same question
    // -- whose address does it put somewhere nobody can list? -- so the walk is
    // done once for whichever of them is on.
    let wants_open = prog.arch().calleevote.is_on() || prog.arch().callbacktype.is_on();
    let open = match &graph {
        Ok(graph) if wants_open => Some(graph.open_entries(&args.binary, args.slice_pref())),
        _ => None,
    };
    let expected = match (&graph, &open) {
        (Ok(graph), Some(open)) if prog.arch().calleevote.is_on() => {
            Some(open_callee_votes(prog, graph, open, &targets))
        }
        _ => None,
    };
    if prog.arch().callbacktype.is_on() && open.is_some() {
        let ledger = &mut prog.arch_mut().kuna_callbacktype;
        *ledger = kuna_decomp::kuna_callbacktype::Ledger::default();
        ledger.recording = true;
    }
    let mut slots: Vec<Option<FuncResult>> = (0..targets.len()).map(|_| None).collect();
    kuna_decomp::kuna_elemptr::start(prog.arch_mut(), true);
    prog.arch_mut().kuna_voidret = Default::default();
    let mut reads = VoidReads::default();
    for &(index, park) in &plan {
        slots[index] = Some(decompile_planned(prog, &targets[index], park, &base));
        if let Some(k) = vote_key(&targets[index]) {
            reads.decompiled(k);
        }
        reads.settle(prog, &targets, &plan, &base, &mut slots);
    }
    if let Some(expected) = expected {
        callee_vote_rounds(prog, &targets, &plan, &base, &mut slots, &expected);
    }
    converge_callee_first(prog, &targets, &plan, &base, &mut slots);
    prog.arch_mut().kuna_callbacktype.recording = false;
    if let (Ok(graph), Some(open)) = (&graph, &open) {
        if prog.arch().callbacktype.is_on() {
            callback_park_round(prog, graph, open, &targets, &plan, &base, &mut slots);
        }
    }
    converge_element_globals_callee_first(prog, &targets, &plan, &base, &mut slots);
    kuna_decomp::kuna_elemptr::stop(prog.arch_mut());
    let mut results: Vec<FuncResult> = slots.into_iter().flatten().collect();
    if base.want_pointer_arguments {
        kuna_console::project::reconcile_pointer_arguments(&mut results);
    }
    results
}

/// (kuna `voidret`) How many rounds of redos one settling takes at most. A
/// redone wrapper reads its own callee's result, so each round reaches one
/// function further down a chain of wrappers, and one caller further up it
/// once the wrapper returns.
const VOID_READ_ROUNDS: usize = 10;

/// (kuna `voidret`) When each function was last decompiled, and when a redo
/// last changed each function's recovered return, in decompiles counted from
/// the start of the run.
#[derive(Default)]
struct VoidReads {
    stamp: usize,
    stamp_of: BTreeMap<(i32, u64), usize>,
    changed: BTreeMap<(i32, u64), usize>,
}

impl VoidReads {
    fn decompiled(&mut self, k: (i32, u64)) {
        self.stamp += 1;
        self.stamp_of.insert(k, self.stamp);
    }

    /// Decompile again, in plan order, every function recovered `void` whose
    /// result a caller reads, returning in the storage the callers read
    /// (`kuna_decomp::kuna_voidret`), every function whose float return a
    /// reader keeps as another type, and every reader of a function whose
    /// return a redo changed after the reader was decompiled, until none is
    /// left. It runs after each function of the callee-first plan, so a
    /// wrapper is redone as soon as its first reader is decompiled: only that
    /// reader is decompiled again, and every later one reads the wrapper's
    /// return the first time. A redo that fails keeps the first body.
    fn settle(
        &mut self,
        prog: &mut ConsoleProgram,
        targets: &[FunctionEntry],
        plan: &[(usize, bool)],
        base: &kuna_console::project::DecompileOptions,
        slots: &mut [Option<FuncResult>],
    ) {
        for _ in 0..VOID_READ_ROUNDS {
            let mut redo = kuna_decomp::kuna_voidret::due(prog.arch_mut());
            redo.extend(kuna_decomp::kuna_voidret::withdrawals(prog.arch_mut()));
            redo.extend(kuna_decomp::kuna_voidret::stale_readers(prog.arch(), &self.stamp_of, &self.changed));
            if redo.is_empty() {
                break;
            }
            self.redo_in_plan_order(prog, targets, plan, base, slots, &redo);
        }
    }

    /// Decompile again, in plan order, every target keyed in `keys`, noting
    /// those whose recovered return the redo changed.
    fn redo_in_plan_order(
        &mut self,
        prog: &mut ConsoleProgram,
        targets: &[FunctionEntry],
        plan: &[(usize, bool)],
        base: &kuna_console::project::DecompileOptions,
        slots: &mut [Option<FuncResult>],
        keys: &BTreeSet<(i32, u64)>,
    ) {
        for &(index, park) in plan {
            let Some(k) = vote_key(&targets[index]).filter(|k| keys.contains(k)) else { continue };
            let stated = kuna_decomp::kuna_callrettype::statement(prog.arch(), k);
            let returns = kuna_decomp::kuna_voidret::returns(prog.arch(), k);
            let storage = kuna_decomp::kuna_voidret::return_storage(prog.arch(), k);
            let again = decompile_planned(prog, &targets[index], park, base);
            if slots[index].as_ref().is_none_or(|first| kuna_console::project::redo_replaces(first, &again)) {
                slots[index] = Some(again);
            } else {
                kuna_decomp::kuna_callrettype::restore(prog.arch_mut(), k, stated.clone());
                kuna_decomp::kuna_voidret::restore(prog.arch_mut(), k, returns, storage.clone());
            }
            self.decompiled(k);
            if kuna_decomp::kuna_voidret::returns(prog.arch(), k) != returns
                || (kuna_decomp::kuna_voidret::widened(prog.arch(), k)
                    && kuna_decomp::kuna_voidret::return_storage(prog.arch(), k) != storage)
                || !kuna_decomp::kuna_callrettype::same_statement(
                    stated.as_deref(),
                    kuna_decomp::kuna_callrettype::statement(prog.arch(), k).as_deref(),
                )
            {
                self.changed.insert(k, self.stamp);
            }
        }
    }
}

/// (kuna `elemptr`) [`kuna_console::project::converge_element_globals`] in plan
/// order, each target with its own park decision.
fn converge_element_globals_callee_first(
    prog: &mut ConsoleProgram,
    targets: &[FunctionEntry],
    plan: &[(usize, bool)],
    base: &kuna_console::project::DecompileOptions,
    slots: &mut [Option<FuncResult>],
) {
    for _ in 0..2 {
        let redo = kuna_decomp::kuna_elemptr::disagreements(prog.arch_mut());
        if redo.is_empty() {
            return;
        }
        for &(index, park) in plan {
            if !redo.contains(&targets[index].addr.get_offset()) {
                continue;
            }
            let again = decompile_planned(prog, &targets[index], park, base);
            match slots[index].as_ref() {
                Some(first) if !kuna_console::project::redo_replaces(first, &again) => {}
                _ => slots[index] = Some(again),
            }
        }
    }
}

/// (kuna `calleevote`) How many times the callers' statements are decided and
/// the functions they name decompiled again: a forwarding wrapper passes on the
/// type its own callers gave it only once it has been decompiled with it.
const CALLEE_VOTE_ROUNDS: usize = 3;

/// (kuna `calleevote`) The longest first decompile that is always done again.
/// A redo costs what the first decompile of that function cost (measured 1.01x
/// over `kmod -O2-noinline`'s 55 redos), and the redo pass is the option's whole
/// cost, so a function's printed length is what it charges. The shapes the vote
/// exists for -- a forwarder, a getter, a comparator -- print a few lines, and
/// those are redone on any binary.
const CALLEE_VOTE_MAX_LINES: usize = 32;

/// (kuna `calleevote`) What the redo pass may reprint, as a share of what the
/// first pass printed.
///
/// A flat refusal above [`CALLEE_VOTE_MAX_LINES`] costs the same nine perfect
/// functions on every binary, including the ones with room to spare: `fmt -O2`
/// spends 0.2% of its run on the redo pass and `ls`/`sort`/`bash` less, while
/// `kmod -O2-noinline` spends 3.4%. A share of the binary's own output spends
/// that room where it exists and stops where it does not: a binary whose short
/// functions already fill it buys no long ones, and one that barely uses the
/// vote redoes every candidate it has.
const CALLEE_VOTE_BUDGET_PCT: usize = 5;

/// (kuna `calleevote`) What redoing this function is charged: the lines its
/// first decompile printed.
fn redo_charge(code: Option<&str>) -> usize {
    code.map_or(0, |c| c.lines().count())
}

/// (kuna `calleevote`) The key the ledger files a function under.
fn vote_key(t: &FunctionEntry) -> Option<(i32, u64)> {
    Some((t.addr.get_space()?.get_index(), t.addr.get_offset()))
}

/// Park eligible callback prototypes, then redo only their planned targets.
/// Element-global convergence follows this round.
fn callback_park_round(
    prog: &mut ConsoleProgram,
    graph: &CallGraph,
    open: &BTreeSet<u64>,
    targets: &[FunctionEntry],
    plan: &[(usize, bool)],
    base: &kuna_console::project::DecompileOptions,
    slots: &mut [Option<FuncResult>],
) {
    use kuna_decomp::kuna_callbacktype as cb;
    let trace = cb::trace();
    let decided = prog.arch().kuna_callbacktype.decided();
    if decided.is_empty() {
        return;
    }
    let index: BTreeMap<u64, usize> = targets
        .iter()
        .enumerate()
        .filter_map(|(i, t)| vote_key(t).map(|k| (k.1, i)))
        .collect();
    let mut again: BTreeSet<u64> = BTreeSet::new();
    for (value, seen) in decided {
        let Some(&at) = index.get(&value) else {
            if trace {
                eprintln!(
                    "[callbacktype] decline 0x{value:x} not-a-target ({})",
                    seen.declared_by
                );
            }
            continue;
        };
        // Every instruction the image shows taking the address has to be
        // accounted for by a callback argument. The instruction itself cannot
        // be matched -- by the time the argument is read, type propagation has
        // rebuilt it as a `PTRSUB` sited at the call, not at the `lea` -- so
        // the two halves of the question are asked separately: the reference
        // must sit in a function that handed this address to a slot, and there
        // must not be more references than there were arguments. A function
        // holding one `lea` for a `qsort` call and another for a store fails
        // the second. One hoisted `lea` feeding two registrations passes both
        // -- it is ONE instruction -- which is why `park` also asks each body
        // whether it used the address anywhere the slots do not account for.
        let taken = graph.address_taken_refs(value);
        let escapes = open.contains(&value)
            || taken.len() > seen.args
            || !taken
                .iter()
                .all(|(_, owner)| owner.is_some_and(|o| seen.owners.contains(&o)));
        let entry = targets[at].addr.clone();
        let name = targets[at].name.clone();
        let image = cb::ImageFacts {
            escapes,
            is_target: true,
            called_directly: graph.called_directly(value),
            callers: graph.direct_callers(value),
        };
        let outcome = cb::park(prog.arch_mut(), &entry, &name, image);
        match outcome {
            Ok(pieces) => {
                if trace {
                    eprintln!(
                        "[callbacktype] park {name} @0x{value:x} from {} params={}",
                        seen.declared_by,
                        pieces.intypes.len(),
                    );
                }
                again.insert(value);
            }
            Err(reason) => {
                if trace {
                    eprintln!(
                        "[callbacktype] decline {name} @0x{value:x} {} ({}) taken={:x?} owners={:x?} args={}",
                        reason.as_str(),
                        seen.declared_by,
                        taken,
                        seen.owners,
                        seen.args
                    );
                }
            }
        }
    }
    if again.is_empty() {
        return;
    }
    for &(i, park) in plan {
        let Some(key) = vote_key(&targets[i]) else {
            continue;
        };
        if !again.contains(&key.1) {
            continue;
        }
        let redone = decompile_planned(prog, &targets[i], park, base);
        if slots[i]
            .as_ref()
            .is_none_or(|first| kuna_console::project::redo_replaces(first, &redone))
        {
            slots[i] = Some(redone);
        }
    }
}

/// (kuna `calleevote`) Start recording, mark the functions whose callers are all
/// known direct calls, and return every target's expected call sites. An image
/// that cannot be read again leaves every function open.
fn open_callee_votes(
    prog: &mut ConsoleProgram,
    graph: &CallGraph,
    open: &BTreeSet<u64>,
    targets: &[FunctionEntry],
) -> BTreeMap<(i32, u64), Vec<u64>> {
    let mut expected = BTreeMap::new();
    #[expect(clippy::disallowed_types, reason = "Membership only; the ledger reads closed through contains.")]
    let mut closed = std::collections::HashSet::new();
    for t in targets {
        let Some(key) = vote_key(t) else { continue };
        let Some(sites) = graph.direct_call_sites(key.1, open) else {
            continue;
        };
        if !sites.is_empty() {
            closed.insert(key);
        }
        expected.insert(key, sites);
    }
    let ledger = &mut prog.arch_mut().kuna_calleevote;
    *ledger = kuna_decomp::kuna_calleevote::Ledger::default();
    ledger.closed = closed;
    ledger.recording = true;
    expected
}

/// (kuna `calleevote`) Decide what every function's callers state about it and
/// decompile the functions whose statement is new, in plan order so a redone
/// callee states its types again before its redone callers read them; then
/// decide again over what the redone functions pass, up to
/// [`CALLEE_VOTE_ROUNDS`] times. A redo that fails keeps the first body.
///
/// Each round's redos are admitted shortest first, against one budget for the
/// whole pass: every redo is charged the lines it reprints,
/// [`CALLEE_VOTE_BUDGET_PCT`] of what the first pass printed is what there is to
/// spend, and a function printing at most [`CALLEE_VOTE_MAX_LINES`] lines is
/// admitted even once that is gone. A function the budget cannot reach leaves
/// the ledger with its statement withdrawn, so nothing is stated about it, no
/// later round proposes it again and the convergence sweep has nothing to apply
/// to it -- exactly what a function over the flat bound used to get, on the
/// binaries that have no room for it.
fn callee_vote_rounds(
    prog: &mut ConsoleProgram,
    targets: &[FunctionEntry],
    plan: &[(usize, bool)],
    base: &kuna_console::project::DecompileOptions,
    slots: &mut [Option<FuncResult>],
    expected: &BTreeMap<(i32, u64), Vec<u64>>,
) {
    let printed: usize = slots
        .iter()
        .map(|s| {
            s.as_ref()
                .and_then(|r| r.code.as_deref())
                .map_or(0, |c| c.lines().count())
        })
        .sum();
    let mut budget = printed * CALLEE_VOTE_BUDGET_PCT / 100;
    let opening = budget;
    let mut spent = 0usize;
    let mut redone = 0usize;
    if kuna_decomp::kuna_calleevote::trace() {
        eprintln!("[calleevote] {printed} lines printed, redo budget {budget} lines");
    }
    for round in 0..CALLEE_VOTE_ROUNDS {
        let changed =
            kuna_decomp::kuna_calleevote::decide(prog.arch_mut(), &|k| expected.get(&k).cloned());
        if kuna_decomp::kuna_calleevote::trace() {
            eprintln!(
                "[calleevote] round {}: {} functions decompiled again",
                round + 1,
                changed.len()
            );
        }
        if changed.is_empty() {
            break;
        }
        #[expect(clippy::disallowed_types, reason = "Membership only; the ordered plan determines redo order.")]
        let changed: std::collections::HashSet<(i32, u64)> = changed.into_iter().collect();
        let queue: Vec<(usize, (i32, u64))> = plan
            .iter()
            .filter_map(|&(index, _)| {
                let key = vote_key(&targets[index]).filter(|k| changed.contains(k))?;
                Some((
                    redo_charge(slots[index].as_ref().and_then(|r| r.code.as_deref())),
                    key,
                ))
            })
            .collect();
        let (admitted, declined) = admit_within_budget(queue, &mut budget, &mut spent);
        for (charge, key) in declined {
            if kuna_decomp::kuna_calleevote::trace() {
                eprintln!(
                    "[calleevote] 0x{:x} declined: {charge} lines, {budget} left",
                    key.1
                );
            }
            prog.arch_mut().kuna_calleevote.decline(key);
        }
        for &(index, park) in plan {
            let Some(key) = vote_key(&targets[index]).filter(|k| admitted.contains(k)) else {
                continue;
            };
            let stated = kuna_decomp::kuna_callrettype::statement(prog.arch(), key);
            let again = decompile_planned(prog, &targets[index], park, base);
            redone += 1;
            match slots[index].as_ref() {
                Some(first) if !same_arity(first, &again) => {
                    prog.arch_mut().kuna_calleevote.forget(key);
                    kuna_decomp::kuna_callrettype::restore(prog.arch_mut(), key, stated);
                }
                Some(first) if !kuna_console::project::redo_replaces(first, &again) => {
                    kuna_decomp::kuna_callrettype::restore(prog.arch_mut(), key, stated);
                }
                _ => {
                    prog.arch_mut().kuna_calleevote.keep(key);
                    slots[index] = Some(again);
                }
            }
        }
    }
    if kuna_decomp::kuna_calleevote::trace() {
        eprintln!("[calleevote] redo pass: {redone} decompiles, {spent} lines reprinted of {opening} budgeted");
    }
    prog.arch_mut().kuna_calleevote.recording = false;
}

#[expect(clippy::disallowed_types, reason = "Admitted membership is unordered; declined charges retain sorted order.")]
type BudgetAdmission = (
    std::collections::HashSet<(i32, u64)>,
    Vec<(usize, (i32, u64))>,
);

/// (kuna `calleevote`) Split this round's candidates -- one `(charge, key)` pair
/// per function the decision changed, charged the lines its first decompile
/// printed -- into the ones the pass decompiles again and the ones it declines.
///
/// Admitted: every function printing at most [`CALLEE_VOTE_MAX_LINES`] lines --
/// the shapes the vote exists for, which are redone whatever the budget says --
/// plus the longest prefix of the rest, shortest first, that `budget` still
/// covers. `budget` and `spent` carry across rounds, so what an early round
/// bought a later one cannot.
///
/// Ties go to the lower (space, address) key, so admission is a function of the
/// program and not of the order the plan visits it.
fn admit_within_budget(
    mut queue: Vec<(usize, (i32, u64))>,
    budget: &mut usize,
    spent: &mut usize,
) -> BudgetAdmission {
    queue.sort_unstable();
    #[expect(clippy::disallowed_types, reason = "Membership only; queue sorting determines admission and decline order.")]
    let mut admitted = std::collections::HashSet::new();
    let mut declined = Vec::new();
    for (charge, key) in queue {
        if charge <= CALLEE_VOTE_MAX_LINES || charge <= *budget {
            *budget = budget.saturating_sub(charge);
            *spent += charge;
            admitted.insert(key);
        } else {
            declined.push((charge, key));
        }
    }
    (admitted, declined)
}

/// (kuna `calleevote`) Does a redo keep the first decompile's parameters and
/// variables, by count? A caller's vote moves types only, so a redo that moves
/// either is not the vote's doing: decompiling a function a second time in one
/// session can recover a stack parameter the first decompile did not (bash
/// -O2 `sub_7ea70` gains an unused `unsigned int a6`), and such a redo keeps
/// the first body.
fn same_arity(first: &FuncResult, again: &FuncResult) -> bool {
    let params = |r: &FuncResult| r.variables.iter().filter(|v| v.is_param).count();
    let declared = |r: &FuncResult| {
        let head = r
            .code
            .as_deref()
            .and_then(|c| c.lines().next())
            .unwrap_or("");
        let inner = head
            .find('(')
            .and_then(|o| head.rfind(')').map(|c| &head[o + 1..c]))
            .unwrap_or("");
        if inner.trim().is_empty() || inner.trim() == "void" {
            0
        } else {
            inner.matches(',').count() + 1
        }
    };
    first.variables.len() == again.variables.len()
        && params(first) == params(again)
        && declared(first) == declared(again)
}

/// (kuna `protoorder` + `structsynth`) The batch's convergence sweep
/// (`converge_synthesized_structs`) for the callee-first order: decompile once
/// more, in plan order and with each target's own park decision, exactly the
/// results that name a superseded structure. A redone callee states its survivor
/// type again before its redone callers read it, and a statement naming a
/// superseded structure is forgotten before anything is redone, so no redo reads
/// one: not a recursive function's own, and not a cycle partner planned after it.
/// Not under `lock`, where a parked prototype is declared and a second decompile
/// would read its own.
fn converge_callee_first(
    prog: &mut ConsoleProgram,
    targets: &[FunctionEntry],
    plan: &[(usize, bool)],
    base: &kuna_console::project::DecompileOptions,
    slots: &mut [Option<FuncResult>],
) {
    if prog.arch().protoorder == kuna_decomp::kuna_protoorder::ProtoOrderMode::Lock {
        return;
    }
    let stale = kuna_console::project::superseded_struct_names(prog);
    if stale.is_empty() {
        return;
    }
    kuna_decomp::kuna_protoorder::forget_statements_naming(prog.arch_mut(), &stale);
    kuna_decomp::kuna_calleevote::forget_statements_naming(prog.arch_mut(), &stale);
    kuna_decomp::kuna_callrettype::forget_statements_naming(prog.arch_mut(), &stale);
    for &(index, park) in plan {
        if !slots[index]
            .as_ref()
            .is_some_and(|r| kuna_console::project::names_any_type(r, &stale))
        {
            continue;
        }
        let key = vote_key(&targets[index]);
        let stated = key.and_then(|k| kuna_decomp::kuna_callrettype::statement(prog.arch(), k));
        let again = decompile_planned(prog, &targets[index], park, base);
        if slots[index]
            .as_ref()
            .is_none_or(|first| kuna_console::project::redo_replaces(first, &again))
        {
            slots[index] = Some(again);
        } else if let Some(k) = key {
            kuna_decomp::kuna_callrettype::restore(prog.arch_mut(), k, stated);
        }
    }
}

/// Decompile one planned target with its prototype-parking decision.
fn decompile_planned(
    prog: &mut ConsoleProgram,
    target: &FunctionEntry,
    park: bool,
    base: &kuna_console::project::DecompileOptions,
) -> FuncResult {
    let opts = kuna_console::project::DecompileOptions {
        park_recovered_proto: park,
        ..*base
    };
    kuna_console::project::decompile_entry(prog, target.clone(), &opts)
}

#[cfg(test)]
mod calleevote_stored_tests {
    use super::*;

    #[test]
    fn budget_admission_ties_use_space_then_address() {
        let queue = vec![(40, (2, 0x1000)), (40, (1, 0x3000)), (40, (1, 0x2000))];
        for queue in [queue.clone(), queue.into_iter().rev().collect()] {
            let mut budget = 40;
            let mut spent = 0;
            let (admitted, declined) = admit_within_budget(queue, &mut budget, &mut spent);
            assert_eq!(admitted.len(), 1);
            assert!(admitted.contains(&(1, 0x2000)));
            assert_eq!(declined, [(40, (1, 0x3000)), (40, (2, 0x1000))]);
            assert_eq!((budget, spent), (0, 40));
        }
    }

    /// (kuna `calleevote`) What a redo is charged: the lines it reprints, and a
    /// function that failed has no body to measure.
    #[test]
    fn a_redo_is_charged_the_lines_it_reprints() {
        let getter = "long get_cap(void *a0)\n{\n  return *(long *)((long)a0 + 0x18);\n}\n";
        assert_eq!(redo_charge(Some(getter)), 4);
        let long: String = (0..=CALLEE_VOTE_MAX_LINES)
            .map(|i| format!("  v{i} = v{i} + 1;\n"))
            .collect();
        assert_eq!(redo_charge(Some(&long)), CALLEE_VOTE_MAX_LINES + 1);
        assert_eq!(redo_charge(None), 0);
    }

    /// (kuna `calleevote`) The budget is a share of what the binary printed:
    /// a long body is bought where the run has room for it and refused where it
    /// is not, while a short one is redone either way.
    #[test]
    fn the_budget_is_a_share_of_what_the_first_pass_printed() {
        /// One round through the real admission, over a binary that printed
        /// `printed` lines: the charges admitted and the charges declined (both
        /// sorted), what is left of the budget, and what was spent.
        fn spend(printed: usize, charges: &[usize]) -> (Vec<usize>, Vec<usize>, usize, usize) {
            let queue: Vec<(usize, (i32, u64))> = charges
                .iter()
                .enumerate()
                .map(|(i, &c)| (c, (1i32, 0x1000 + i as u64)))
                .collect();
            let by_key: std::collections::HashMap<(i32, u64), usize> =
                queue.iter().map(|&(c, k)| (k, c)).collect();
            let mut budget = printed * CALLEE_VOTE_BUDGET_PCT / 100;
            let mut spent = 0usize;
            let (admitted, declined) = admit_within_budget(queue, &mut budget, &mut spent);
            let mut taken: Vec<usize> = admitted.iter().map(|k| by_key[k]).collect();
            taken.sort_unstable();
            (
                taken,
                declined.iter().map(|&(c, _)| c).collect(),
                budget,
                spent,
            )
        }
        // A 20,000-line binary buys a 90-line and a 300-line body; a 2,000-line
        // one buys only the 90; a 1,000-line one buys neither.
        assert_eq!(spend(20_000, &[300, 90]).0, vec![90, 300]);
        assert_eq!(spend(2_000, &[300, 90]), (vec![90], vec![300], 10, 90));
        assert_eq!(spend(1_000, &[300, 90]), (vec![], vec![90, 300], 50, 0));
        // The short bodies the vote exists for are redone whatever is left, and
        // what they spend is what leaves no room for a long one. Once the budget
        // is gone it stays at zero rather than wrapping.
        assert_eq!(spend(400, &[30, 30, 90]), (vec![30, 30], vec![90], 0, 60));
        assert_eq!(spend(40, &[30, 30]), (vec![30, 30], vec![], 0, 60));
        // Shortest first is by charge, not by the order the plan lists them:
        // the 40-line body is bought and the 60-line one declined either way.
        let budgeted = spend(1_000, &[60, 40]);
        assert_eq!(budgeted, (vec![40], vec![60], 10, 40));
        assert_eq!(spend(1_000, &[40, 60]), budgeted);
    }

    /// (kuna `calleevote`) The budget is one pass-wide allowance: what an early
    /// round spends a later one no longer has.
    #[test]
    fn the_budget_carries_across_rounds() {
        let mut budget = 100usize;
        let mut spent = 0usize;
        let first = admit_within_budget(vec![(90, (1, 0x2000))], &mut budget, &mut spent);
        assert_eq!(
            (first.0.len(), first.1.len(), budget, spent),
            (1, 0, 10, 90)
        );
        let second = admit_within_budget(vec![(90, (1, 0x3000))], &mut budget, &mut spent);
        assert_eq!(
            (second.0.len(), second.1.len(), budget, spent),
            (0, 1, 10, 90)
        );
    }
}
