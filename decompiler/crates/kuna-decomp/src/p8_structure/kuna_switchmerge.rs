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
//! An arm moves or goes away only when the arm printed before it cannot fall
//! into it.  The goto census reads [`SwitchPlans::hosts`], so a `break;` inside
//! a merged switch is placed for the switch that is printed.  The plan exists
//! only for the C switch form; other languages print every arm.

use crate::block::{block_flags, BlockGraph, BlockType, CaseOrder};
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet};

/// One printed arm: case `index` of the switch `owner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlannedCase {
    pub owner: BlockId,
    pub index: usize,
    /// The arm prints `default:` in place of its own labels.
    pub as_default: bool,
    /// The arm is a list whose first `skip` blocks were merged switches; it
    /// prints the rest of the list.
    pub skip: usize,
}

impl PlannedCase {
    fn new(owner: BlockId, index: usize) -> Self {
        PlannedCase { owner, index, as_default: false, skip: 0 }
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
    let start = if e.skip > 0 { *arm_list(graph, blk).0.get(e.skip)? } else { blk };
    graph.get_front_leaf(start)
}

/// Does the arm print as code that runs on into the next printed arm?
fn falls_through(fd: &Funcdata, e: PlannedCase) -> bool {
    let c = case_of(fd, e);
    c.gototype == 0 && !c.isexit && fd.sblocks_ref().block(c.block).size_out() > 0
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
                    || sb.switch_caseblocks().iter().any(|c| c.isdefault || c.isexit)
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
    let high = fd.vbank().get(sel).and_then(|v| v.get_high());
    let is_selector = |vn: VarnodeId| {
        vn == sel || fd.vbank().get(vn).is_some_and(|v| !v.is_implied() && high.is_some() && v.get_high() == high)
    };
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
        if out < 0 || out >= blk.size_out() {
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
        let arms = fold_default(fd, facts, arms, &mut out.dropped);
        let identity = arms.len() == n && arms.iter().enumerate().all(|(i, &e)| e == PlannedCase::new(sw, i));
        if !identity {
            out.plans.insert(sw, arms);
        }
    }
    out.hosts = planner.hosts;
    out
}
