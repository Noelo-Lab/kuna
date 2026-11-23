//! (kuna `switchmerge`) Print-time plan for the arms of a C `switch`.
//!
//! A compiler lowers one sparse source `switch` into a search tree of jump
//! tables: each table's range guard sends the values it does not cover on to
//! the next table, and the table's holes go to the source's `default:` code.
//! The structurer nests the tables (`default: switch (v) { ... }`, or a
//! `default:` arm that runs one switch after another) and prints every hole as
//! an explicit `case N:` of a `goto` arm.  [`switch_plans`] undoes both at print
//! time, on the final structured tree, without changing where any value goes:
//!
//! * The `default:` arm of a switch is replaced by the arms of a switch on the
//!   same selector that it starts with, when that inner switch prints nothing
//!   before its `switch`, carries no label, lists no value the merged switch
//!   already lists, and leaves the same way the arm does.  An inner switch
//!   followed by more code in the arm must have no `default:` and no arm that
//!   leaves it, so its unlisted values reach that code, which stays the
//!   `default:` arm (also when the arm is that code plus a trailing `goto`).
//! * An arm that only jumps to the code the `default:` arm runs prints no
//!   labels, since `default:` covers its values; when `default:` is that jump
//!   and another arm holds the code, the arm holding the code prints `default:`.
//!   An arm that jumps or breaks elsewhere prints no labels either when the
//!   `default:` code sends each of its values there before printing anything
//!   ([`default_reaches`]: its tests and switches on the selector, decided
//!   with the value).
//!
//! * An `if (v == c) S` with no else that starts the `default:` arm prints as
//!   `case c: S` before it ([`fold_guards`]).
//! * An `if (v relop K) { switch A } else { switch B }` on the selector prints
//!   as one switch when every value of `A` and `B` takes its own side and the
//!   two defaults do the same ([`merge_range_splits`]).
//!
//! An arm moves or goes away only when the arm printed before it cannot fall
//! into it.  The goto census reads [`SwitchPlans::hosts`], so a `break;` inside
//! a merged switch is placed for the switch that is printed.  The plan exists
//! only for the C switch form; other languages print every arm.

use crate::block::{block_flags, BlockGraph, BlockType, CaseOrder};
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet};

/// One printed arm: case `index` of the switch `owner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlannedCase {
    pub owner: BlockId,
    pub index: usize,
    /// The arm prints `default:` in place of its own labels.
    pub as_default: bool,
    /// The arm is a list whose first `skip` blocks were merged switches or
    /// guards; it prints the rest of the list (nothing for an arm that is
    /// not a list).
    pub skip: usize,
    /// The arm prints `case value:` and the then-branch of this `if (v ==
    /// value)`, which started the `default:` arm of `owner`.
    pub guard: Option<(BlockId, uintb)>,
}

impl PlannedCase {
    pub(crate) fn new(owner: BlockId, index: usize) -> Self {
        PlannedCase { owner, index, as_default: false, skip: 0, guard: None }
    }
}

#[derive(Default)]
pub(crate) struct SwitchPlans {
    /// The arms of each switch whose printed arms differ from its own cases.
    pub plans: BTreeMap<BlockId, Vec<PlannedCase>>,
    /// Goto arms left out: (owner switch, case block).
    pub dropped: BTreeSet<(BlockId, BlockId)>,
    /// Each merged switch, mapped to the switch it prints inside.
    pub hosts: BTreeMap<BlockId, BlockId>,
    /// Each `if` that prints as a switch, mapped to the switch whose plan it
    /// prints.
    pub ifs: BTreeMap<BlockId, BlockId>,
}

/// The label values one switch's non-default arms print, with the byte size
/// and signedness every label prints with.
pub(crate) type LabelSet = (BTreeSet<uintb>, int4, bool);

/// What the printer tells the planner about a switch.
pub(crate) struct PrintFacts<'a> {
    /// Does a switch's header block print no statement before its `switch`?
    pub prints_nothing: &'a dyn Fn(BlockId) -> bool,
    /// The labels a switch prints; `None` when two arms print the same value.
    pub labels: &'a dyn Fn(BlockId) -> Option<LabelSet>,
    /// The label values case `index` of a switch prints.
    pub arm_labels: &'a dyn Fn(BlockId, usize) -> Vec<uintb>,
    /// Does a basic block print no statement besides its branch?
    pub block_prints_nothing: &'a dyn Fn(BlockId) -> bool,
}

struct Planner<'a> {
    fd: &'a Funcdata,
    facts: &'a PrintFacts<'a>,
    hosts: BTreeMap<BlockId, BlockId>,
}

fn case_of(fd: &Funcdata, e: PlannedCase) -> &CaseOrder {
    &fd.sblocks_ref().block(e.owner).switch_caseblocks()[e.index]
}

/// The blocks an arm runs in order, when it is a list or a `BlockGoto` around
/// one, and whether the list is wrapped in a `BlockGoto`.
pub(crate) fn arm_list(graph: &BlockGraph, blk: BlockId) -> (&[BlockId], bool) {
    let b = graph.block(blk);
    match b.get_type() {
        BlockType::Ls => (b.get_list(), false),
        BlockType::Goto if graph.block(b.get_block(0)).get_type() == BlockType::Ls => {
            (graph.block(b.get_block(0)).get_list(), true)
        }
        _ => (&[], false),
    }
}

/// The first block an arm prints.
fn arm_start(fd: &Funcdata, e: PlannedCase) -> Option<BlockId> {
    let graph = fd.sblocks_ref();
    let blk = case_of(fd, e).block;
    let start = match e.guard {
        Some((guard, _)) => graph.block(guard).get_block(1),
        None if e.skip > 0 => *arm_list(graph, blk).0.get(e.skip)?,
        None => blk,
    };
    graph.get_front_leaf(start)
}

/// Can the arm's printed code run on into the next printed arm?
fn falls_through(fd: &Funcdata, e: PlannedCase) -> bool {
    let c = case_of(fd, e);
    c.gototype == 0 && !c.isexit && runs_past_end(fd, c.block)
}

/// Can the printed code of `blk` run past its end?  It cannot when it ends in
/// a leaf whose basic block has no successor, or in a `BlockGoto` whose jump
/// prints (the goto census leaves out a jump to the block printed next).  The
/// leaf's own block is read, not its structured edges, since a duplicated
/// tail leaf carries none.
fn runs_past_end(fd: &Funcdata, blk: BlockId) -> bool {
    let graph = fd.sblocks_ref();
    let b = graph.block(blk);
    match b.get_type() {
        BlockType::Ls => b.get_list().last().is_none_or(|&l| runs_past_end(fd, l)),
        BlockType::Goto => {
            let target = b.get_goto_target().and_then(|t| graph.get_front_leaf(t));
            target.is_none() || b.get_parent().and_then(|p| graph.next_printed_flow_after(p, blk)) == target
        }
        BlockType::Copy => b.get_copy().is_none_or(|bb| fd.bblocks_ref().block(bb).size_out() > 0),
        _ => true,
    }
}

/// The selector varnode the switch header prints: in0 of its BRANCHIND.
fn selector(fd: &Funcdata, sw: BlockId) -> Option<VarnodeId> {
    let jt = fd.get_jump_table(fd.sblocks_ref().block(sw).switch_jt_index()? as int4);
    if jt.kuna_lowered_var().is_some() {
        return None;
    }
    fd.obank().get(jt.get_indirect_op()?)?.get_in(0)
}

fn same_selector(fd: &Funcdata, a: BlockId, b: BlockId) -> bool {
    let (Some(va), Some(vb)) = (selector(fd, a), selector(fd, b)) else {
        return false;
    };
    if va == vb {
        return true;
    }
    let (Some(x), Some(y)) = (fd.vbank().get(va), fd.vbank().get(vb)) else {
        return false;
    };
    !x.is_implied() && !y.is_implied() && x.get_high().is_some() && x.get_high() == y.get_high()
}

/// Does some jump-table entry of `sw` go to the switch's exit?  Such a value
/// prints no label and leaves the switch, so it cannot be told apart from a
/// value the merged `default:` takes.
fn has_hidden_exit(fd: &Funcdata, sw: BlockId) -> bool {
    let blk = fd.sblocks_ref().block(sw);
    let Some(j) = blk.switch_jt_index() else {
        return true;
    };
    let jt = fd.get_jump_table(j as int4);
    let mut listed = 0;
    for case in blk.switch_caseblocks() {
        let Some(bb) = case.basicblock else {
            return true;
        };
        listed += jt.num_indices_by_block(fd, bb).unwrap_or(0);
    }
    listed != jt.num_entries()
}

/// Does a `break;` inside one of the arms of `sw` leave `sw`?
fn arm_breaks(graph: &BlockGraph, sw: BlockId) -> bool {
    let mut stack: Vec<BlockId> = graph.block(sw).switch_caseblocks().iter().map(|c| c.block).collect();
    while let Some(id) = stack.pop() {
        let blk = graph.block(id);
        if matches!(blk.get_type(), BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop | BlockType::Switch) {
            continue;
        }
        let goto = match blk.get_type() {
            BlockType::Goto => blk.get_goto_type(),
            _ => blk.get_if_goto_type(),
        };
        if goto & block_flags::f_break_goto != 0 {
            return true;
        }
        for i in 0..blk.get_size() {
            stack.push(blk.get_block(i));
        }
    }
    false
}

impl Planner<'_> {
    /// Can `inner` print its arms in place of the default arm of `host`?
    fn mergeable(&self, host: BlockId, inner: BlockId, listed: &LabelSet) -> Option<LabelSet> {
        let graph = self.fd.sblocks_ref();
        let blk = graph.block(inner);
        if blk.get_type() != BlockType::Switch || blk.is_label_bump_up() {
            return None;
        }
        if graph.block(graph.get_front_leaf(inner)?).is_unstructured_target() {
            return None;
        }
        if !(self.facts.prints_nothing)(blk.get_block(0)) || !same_selector(self.fd, host, inner) {
            return None;
        }
        let (mut values, sz, signed) = listed.clone();
        let (more, msz, msigned) = (self.facts.labels)(inner)?;
        if msz != sz || msigned != signed || !values.is_disjoint(&more) {
            return None;
        }
        values.extend(more);
        Some((values, sz, signed))
    }

    /// The printed arms of `sw`, with the default arm replaced by the switches
    /// it starts with.  `listed` collects every label printed so far.
    fn splice(&mut self, host: BlockId, sw: BlockId, out: &mut Vec<PlannedCase>, listed: &mut LabelSet) {
        let graph = self.fd.sblocks_ref();
        let cases = graph.block(sw).switch_caseblocks();
        for (index, case) in cases.iter().enumerate() {
            let arm = PlannedCase::new(sw, index);
            if !case.isdefault || case.gototype != 0 || out.last().is_some_and(|&p| falls_through(self.fd, p)) {
                out.push(arm);
                continue;
            }
            let (list, wrapped) = arm_list(graph, case.block);
            let mut skip = 0;
            while skip + 1 < list.len() {
                let s = list[skip];
                let sb = graph.block(s);
                if sb.get_type() != BlockType::Switch
                    || sb.switch_caseblocks().iter().any(|c| c.isdefault || c.isexit || c.gototype == block_flags::f_break_goto)
                    || arm_breaks(graph, s)
                {
                    break;
                }
                let Some(next) = self.mergeable(host, s, listed) else {
                    break;
                };
                *listed = next;
                self.hosts.insert(s, host);
                self.splice(host, s, out, listed);
                skip += 1;
            }
            let last = match list.len() {
                0 => Some(case.block),
                n => (skip + 1 == n && !wrapped).then(|| list[skip]),
            };
            let leaves_alike = graph.block(case.block).size_out() == 0 || case.isexit;
            if let Some(last) = last.filter(|&l| leaves_alike && !has_hidden_exit(self.fd, l)) {
                if let Some(next) = self.mergeable(host, last, listed) {
                    *listed = next;
                    self.hosts.insert(last, host);
                    self.splice(host, last, out, listed);
                    continue;
                }
            }
            out.push(PlannedCase { skip, ..arm });
        }
    }
}

/// The value of `vn` as the printed code computes it when the selector
/// (`is_selector`) holds `value`, through the expression the printer inlines.
fn eval_with(fd: &Funcdata, vn: VarnodeId, is_selector: &dyn Fn(VarnodeId) -> bool, value: uintb, depth: u32) -> Option<uintb> {
    let v = fd.vbank().get(vn)?;
    let mask = calc_mask(v.get_size());
    if is_selector(vn) {
        return Some(value & mask);
    }
    if v.is_constant() {
        return Some(v.get_offset() & mask);
    }
    if !v.is_implied() || depth == 0 {
        return None;
    }
    let op = fd.obank().get(v.get_def()?)?;
    let input = |slot| eval_with(fd, op.get_in(slot)?, is_selector, value, depth - 1);
    let in_size = fd.vbank().get(op.get_in(0)?)?.get_size();
    let out = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => input(0)?,
        opc => {
            let behave = fd.get_arch().op_behavior(opc)?;
            let eval = op.get_eval_type();
            if eval & pcodeop_flags::unary != 0 && op.num_input() == 1 {
                behave.evaluate_unary(v.get_size(), in_size, input(0)?).ok()?
            } else if eval & pcodeop_flags::binary != 0 && op.num_input() == 2 {
                behave.evaluate_binary(v.get_size(), in_size, input(0)?, input(1)?).ok()?
            } else {
                return None;
            }
        }
    };
    Some(out & mask)
}

/// Does the code the `default:` arm prints, entered with the selector of `sw`
/// holding `value`, reach the basic block `target` before it prints a
/// statement?  Each branch on the way is decided with `value`: a test of the
/// selector, or a switch on it.
fn default_reaches(fd: &Funcdata, facts: &PrintFacts<'_>, sw: BlockId, start: BlockId, target: BlockId, value: uintb) -> bool {
    const MAX_STEPS: usize = 256;
    let Some(sel) = selector(fd, sw).filter(|&s| fd.vbank().get(s).is_some_and(|v| !v.is_implied())) else {
        return false;
    };
    let is_selector = |vn: VarnodeId| is_selector(fd, sel, vn);
    let graph = fd.bblocks_ref();
    let mut bb = start;
    let mut seen = BTreeSet::new();
    for _ in 0..MAX_STEPS {
        if bb == target {
            return true;
        }
        if !seen.insert(bb) || !(facts.block_prints_nothing)(bb) {
            return false;
        }
        let blk = graph.block(bb);
        let tail = fd.bb_op_tail(bb).and_then(|op| Some((op, fd.obank().get(op)?)));
        let out = match tail {
            Some((_, op)) if op.code() == OpCode::CPUI_CBRANCH => {
                let Some(taken) = op.get_in(1).and_then(|c| eval_with(fd, c, &is_selector, value, 8)) else {
                    return false;
                };
                if (taken != 0) != op.is_boolean_flip() { 1 } else { 0 }
            }
            Some((id, op)) if op.code() == OpCode::CPUI_BRANCHIND => {
                let Some(jt) = fd.find_jump_table_index(id).map(|j| fd.get_jump_table(j as int4)) else {
                    return false;
                };
                let Some(x) = op.get_in(0).filter(|_| jt.kuna_lowered_var().is_none()).and_then(|i| eval_with(fd, i, &is_selector, value, 8)) else {
                    return false;
                };
                let mask = calc_mask(fd.vbank().get(op.get_in(0).unwrap()).map_or(0, |v| v.get_size()));
                match jt.kuna_out_for_label(x, mask) {
                    Some(o) => o,
                    None => return false,
                }
            }
            _ if blk.size_out() == 1 => 0,
            _ => return false,
        };
        if out < 0 || out >= blk.size_out() || blk.is_back_edge_out(out) {
            return false;
        }
        bb = blk.get_out(out);
    }
    false
}

/// Does every value arm `e` prints go, through the `default:` arm starting at
/// the leaf `dstart`, to the code the arm jumps to?
fn values_reach_default(fd: &Funcdata, facts: &PrintFacts<'_>, e: PlannedCase, dstart: BlockId) -> bool {
    let graph = fd.sblocks_ref();
    let (Some(target), Some(start)) = (arm_start(fd, e).and_then(|t| graph.block(t).get_copy()), graph.block(dstart).get_copy()) else {
        return false;
    };
    let values = (facts.arm_labels)(e.owner, e.index);
    !values.is_empty() && values.iter().all(|&h| default_reaches(fd, facts, e.owner, start, target, h))
}

/// Leave out the arms that only repeat what `default:` does: an arm that jumps
/// where `default:` starts, or whose every value the `default:` code sends,
/// before it prints anything, to where the arm jumps.
fn fold_default(
    fd: &Funcdata,
    facts: &PrintFacts<'_>,
    arms: Vec<PlannedCase>,
    dropped: &mut BTreeSet<(BlockId, BlockId)>,
) -> Vec<PlannedCase> {
    let Some(d) = arms.iter().position(|&e| case_of(fd, e).isdefault) else {
        return arms;
    };
    let dc = case_of(fd, arms[d]);
    let target = arm_start(fd, arms[d]);
    if target.is_none() || (dc.gototype != 0 && dc.gototype != block_flags::f_goto_goto) {
        return arms;
    }
    let is_jump = |e: PlannedCase| {
        let c = case_of(fd, e);
        !c.isdefault
            && (c.gototype == block_flags::f_goto_goto && arm_start(fd, e) == target
                || (c.gototype == block_flags::f_goto_goto || c.gototype == block_flags::f_break_goto)
                    && target.is_some_and(|t| values_reach_default(fd, facts, e, t)))
    };
    let code_arm = if dc.gototype != 0 {
        arms.iter().position(|&e| {
            let c = case_of(fd, e);
            !c.isdefault && c.gototype == 0 && arm_start(fd, e) == target
        })
    } else {
        None
    };
    let mut out: Vec<PlannedCase> = Vec::with_capacity(arms.len());
    let mut default_dropped = false;
    for (i, &e) in arms.iter().enumerate() {
        let droppable = is_jump(e) || (i == d && code_arm.is_some());
        if droppable && out.last().is_none_or(|&p| !falls_through(fd, p)) {
            dropped.insert((e.owner, case_of(fd, e).block));
            default_dropped |= i == d;
            continue;
        }
        out.push(e);
    }
    if let Some(c) = code_arm.filter(|_| default_dropped) {
        if let Some(e) = out.iter_mut().find(|e| **e == arms[c]) {
            e.as_default = true;
        }
    }
    out
}

/// Is `vn` the selector `sel` as the printed code reads it: `sel` itself, or
/// an explicit varnode of the same HighVariable?
fn is_selector(fd: &Funcdata, sel: VarnodeId, vn: VarnodeId) -> bool {
    let high = fd.vbank().get(sel).and_then(|v| v.get_high());
    vn == sel || fd.vbank().get(vn).is_some_and(|v| !v.is_implied() && high.is_some() && v.get_high() == high)
}

/// The label values the arms of `arms` print.
fn plan_labels(facts: &PrintFacts<'_>, fd: &Funcdata, arms: &[PlannedCase]) -> Vec<uintb> {
    let mut values = Vec::new();
    for &e in arms {
        match e.guard {
            Some((_, value)) => values.push(value),
            None if !case_of(fd, e).isdefault && !e.as_default => values.extend((facts.arm_labels)(e.owner, e.index)),
            None => {}
        }
    }
    values
}

/// The basic block a structured block starts with, when it carries no label.
fn unlabelled_start(graph: &BlockGraph, blk: BlockId) -> Option<BlockId> {
    let leaf = graph.get_front_leaf(blk)?;
    let b = graph.block(leaf);
    if b.is_unstructured_target() || b.is_label_bump_up() {
        return None;
    }
    b.get_copy()
}

/// The constant `c` of an `if (v == c)` with no else, whose condition block
/// prints only its branch and whose then-arm runs exactly for `c`.
fn guard_value(fd: &Funcdata, facts: &PrintFacts<'_>, sw: BlockId, blk: BlockId) -> Option<uintb> {
    let graph = fd.sblocks_ref();
    let b = graph.block(blk);
    if b.get_type() != BlockType::If || b.get_size() != 2 || b.get_if_goto_target().is_some() {
        return None;
    }
    let cond = b.get_block(0);
    graph.block(cond).get_copy()?;
    let bb = unlabelled_start(graph, cond).filter(|&bb| (facts.block_prints_nothing)(bb))?;
    let then_bb = unlabelled_start(graph, b.get_block(1))?;
    let branch = fd.obank().get(fd.bb_op_tail(bb)?)?;
    if branch.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let test = fd.vbank().get(branch.get_in(1)?)?;
    if !test.is_implied() {
        return None;
    }
    let op = fd.obank().get(test.get_def()?)?;
    if !matches!(op.code(), OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL) {
        return None;
    }
    let (a, b) = (op.get_in(0)?, op.get_in(1)?);
    let (var, constant) = match (fd.vbank().get(a)?.is_constant(), fd.vbank().get(b)?.is_constant()) {
        (false, true) => (a, fd.vbank().get(b)?.get_offset()),
        (true, false) => (b, fd.vbank().get(a)?.get_offset()),
        _ => return None,
    };
    let sel = selector(fd, sw)?;
    if !is_selector(fd, sel, var) {
        return None;
    }
    let mask = calc_mask(fd.vbank().get(sel)?.get_size());
    let value = constant & mask;
    default_reaches(fd, facts, sw, bb, then_bb, value).then_some(value)
}

/// Print each `if (v == c) S` that starts the `default:` arm as `case c: S`
/// before it, which runs `S` for `c` and then the rest of `default:`, as the
/// `if` did.  A second `case` follows only an `S` that never runs past its
/// end, and nothing moves under an arm that falls into `default:`.
fn fold_guards(fd: &Funcdata, facts: &PrintFacts<'_>, mut arms: Vec<PlannedCase>) -> Vec<PlannedCase> {
    let graph = fd.sblocks_ref();
    let mut listed: BTreeSet<uintb> = plan_labels(facts, fd, &arms).into_iter().collect();
    loop {
        let Some(d) = arms.iter().position(|&e| e.guard.is_none() && !e.as_default && case_of(fd, e).isdefault) else {
            return arms;
        };
        let e = arms[d];
        let case = case_of(fd, e);
        if case.gototype != 0 {
            return arms;
        }
        match d.checked_sub(1).map(|p| arms[p]) {
            Some(prev) if prev.guard.is_some_and(|(g, _)| runs_past_end(fd, graph.block(g).get_block(1))) => return arms,
            Some(prev) if prev.guard.is_none() && falls_through(fd, prev) => return arms,
            _ => {}
        }
        let (list, _) = arm_list(graph, case.block);
        let first = match list.len() {
            0 if e.skip == 0 => case.block,
            0 => return arms,
            _ => match list.get(e.skip) {
                Some(&f) => f,
                None => return arms,
            },
        };
        let Some(value) = guard_value(fd, facts, e.owner, first).filter(|v| !listed.contains(v)) else {
            return arms;
        };
        listed.insert(value);
        arms[d].skip += 1;
        arms.insert(d, PlannedCase { guard: Some((first, value)), ..e });
        let rest_empty = list.len() <= e.skip + 1;
        if rest_empty && d + 2 == arms.len() && arm_list(graph, case.block).1 == false {
            arms.pop();
            return arms;
        }
    }
}

/// The switch a branch of an `if` prints as: a switch the plan has not merged
/// into another, or an `if` already printed as a switch.
fn branch_switch(graph: &BlockGraph, out: &SwitchPlans, blk: BlockId) -> Option<BlockId> {
    match graph.block(blk).get_type() {
        BlockType::Switch if !out.hosts.contains_key(&blk) => Some(blk),
        BlockType::If => out.ifs.get(&blk).copied(),
        _ => None,
    }
}

/// The arms `sw` prints under `out`.
fn planned(fd: &Funcdata, out: &SwitchPlans, sw: BlockId) -> Vec<PlannedCase> {
    out.plans.get(&sw).cloned().unwrap_or_else(|| {
        (0..fd.sblocks_ref().block(sw).switch_caseblocks().len()).map(|i| PlannedCase::new(sw, i)).collect()
    })
}

/// Where an arm that is only a jump sends control: its goto type and target.
fn jump_of(fd: &Funcdata, e: PlannedCase) -> Option<(uint4, Option<BlockId>)> {
    let c = case_of(fd, e);
    let jump = e.guard.is_none() && !e.as_default && e.skip == 0
        && (c.gototype == block_flags::f_goto_goto || c.gototype == block_flags::f_break_goto);
    jump.then(|| (c.gototype, arm_start(fd, e)))
}

/// Print `if (v relop K) { switch (v) A } else { switch (v) B }` as one switch:
/// every value of `A` takes the then-branch and every value of `B` the else,
/// and the two `default:` arms do the same (both absent or a `break;`, or the
/// same jump), so every value runs the same code.  Inner `if`s go first, so a
/// tree of range tests folds into one switch.
fn merge_range_splits(fd: &Funcdata, facts: &PrintFacts<'_>, root: BlockId, out: &mut SwitchPlans) {
    let graph = fd.sblocks_ref();
    let mut order = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        order.push(id);
        let b = graph.block(id);
        stack.extend((0..b.get_size()).map(|i| b.get_block(i)));
    }
    for &ib in order.iter().rev() {
        let b = graph.block(ib);
        if b.get_type() != BlockType::If || b.get_size() != 3 || b.get_if_goto_target().is_some() {
            continue;
        }
        let cond = b.get_block(0);
        let (then_blk, else_blk) = (b.get_block(1), b.get_block(2));
        let Some(cond_bb) = graph.block(cond).get_copy().and(unlabelled_start(graph, cond)) else {
            continue;
        };
        let (Some(a), Some(bsw)) = (branch_switch(graph, out, then_blk), branch_switch(graph, out, else_blk)) else {
            continue;
        };
        let (Some(then_bb), Some(else_bb)) = (unlabelled_start(graph, then_blk), unlabelled_start(graph, else_blk)) else {
            continue;
        };
        if !(facts.block_prints_nothing)(cond_bb) || !same_selector(fd, a, bsw) {
            continue;
        }
        let (Some(fa), Some(fb)) = ((facts.labels)(a), (facts.labels)(bsw)) else {
            continue;
        };
        if (fa.1, fa.2) != (fb.1, fb.2) {
            continue;
        }
        let (plan_a, plan_b) = (planned(fd, out, a), planned(fd, out, bsw));
        let owners: BTreeSet<BlockId> = plan_a.iter().chain(&plan_b).map(|e| e.owner).collect();
        if owners.iter().any(|&o| has_hidden_exit(fd, o) || !(facts.prints_nothing)(graph.block(o).get_block(0))) {
            continue;
        }
        let (va, vb) = (plan_labels(facts, fd, &plan_a), plan_labels(facts, fd, &plan_b));
        let set_a: BTreeSet<uintb> = va.iter().copied().collect();
        if vb.iter().any(|v| set_a.contains(v))
            || !va.iter().all(|&h| default_reaches(fd, facts, a, cond_bb, then_bb, h))
            || !vb.iter().all(|&h| default_reaches(fd, facts, bsw, cond_bb, else_bb, h))
        {
            continue;
        }
        let is_default = |e: &PlannedCase| e.as_default || (e.guard.is_none() && case_of(fd, *e).isdefault);
        let (da, db) = (plan_a.iter().position(is_default), plan_b.iter().position(is_default));
        let leaves = |plan: &[PlannedCase], d: Option<usize>| match d {
            None => true,
            Some(i) => jump_of(fd, plan[i]).is_some_and(|(t, _)| t == block_flags::f_break_goto),
        };
        let same_default = match (da, db) {
            (Some(i), Some(j)) => jump_of(fd, plan_a[i]).is_some() && jump_of(fd, plan_a[i]) == jump_of(fd, plan_b[j]),
            _ => leaves(&plan_a, da) && leaves(&plan_b, db),
        };
        if !same_default
            || plan_a.last().is_some_and(|&e| e.guard.is_some() || falls_through(fd, e))
            || db.is_some_and(|j| j > 0 && falls_through(fd, plan_b[j - 1]))
        {
            continue;
        }
        let mut merged = plan_a;
        for (j, &e) in plan_b.iter().enumerate() {
            if Some(j) == db {
                out.dropped.insert((e.owner, case_of(fd, e).block));
            } else {
                merged.push(e);
            }
        }
        out.plans.remove(&bsw);
        for host in out.hosts.values_mut().filter(|h| **h == bsw) {
            *host = a;
        }
        out.hosts.insert(bsw, a);
        out.plans.insert(a, merged);
        out.ifs.insert(ib, a);
    }
}

/// The arm plans for every switch under `root`.
pub(crate) fn switch_plans(fd: &Funcdata, root: BlockId, facts: &PrintFacts<'_>) -> SwitchPlans {
    let graph = fd.sblocks_ref();
    let mut planner = Planner { fd, facts, hosts: BTreeMap::new() };
    let mut out = SwitchPlans::default();
    // Preorder: an outer switch is planned before the switches inside it.
    let mut stack = vec![root];
    while let Some(sw) = stack.pop() {
        let blk = graph.block(sw);
        for i in (0..blk.get_size()).rev() {
            stack.push(blk.get_block(i));
        }
        if blk.get_type() != BlockType::Switch || planner.hosts.contains_key(&sw) {
            continue;
        }
        let mut arms = Vec::new();
        match (facts.labels)(sw) {
            Some(mut listed) if !has_hidden_exit(fd, sw) => planner.splice(sw, sw, &mut arms, &mut listed),
            _ => arms.extend((0..blk.switch_caseblocks().len()).map(|i| PlannedCase::new(sw, i))),
        }
        let n = blk.switch_caseblocks().len();
        let arms = fold_guards(fd, facts, fold_default(fd, facts, arms, &mut out.dropped));
        let identity = arms.len() == n && arms.iter().enumerate().all(|(i, &e)| e == PlannedCase::new(sw, i));
        if !identity {
            out.plans.insert(sw, arms);
        }
    }
    out.hosts = planner.hosts;
    merge_range_splits(fd, facts, root, &mut out);
    out
}
