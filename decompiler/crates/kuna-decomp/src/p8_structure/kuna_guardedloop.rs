//! (kuna `guardedloop`) A guarded do-while loop prints as a while loop.
//!
//! clang rotates `while (c) B` into `if (g) { I; do B while (c); }`, where the
//! guard `g` is `c` on the values the assignments `I` give the loop.  On the
//! final tree, a do-while in an arm of an `if`, behind only assignments that
//! store nothing and call nothing, prints its test at the top, `while (c) B`,
//! when `g` implies `c` on entry: `g` is `c` with every variable an assignment
//! of `I` writes replaced by that assignment's right-hand side (expressions
//! compared op by op, variables by HighVariable, `<=` against a constant
//! written as `<` of the next constant, `==` against `!=` and `<` against `>=`
//! with the senses swapped), or `g` is `a < b` where `c` is `a != b`.  Both
//! forms run the same code: the first test holds, and every later test runs
//! where the bottom test ran, as nothing runs between a latch and its head.
//!
//! When the `if` has no else, the loop ends the arm, `I` reads no memory, no
//! variable `I` writes is read again after the guard's skip edge, and `g` is
//! `c` on entry (calls included: the loop's first test then makes the guard's
//! call), the guard goes too: the `if` becomes a list of its condition block's
//! statements and the arm.  No goto may reach an assignment of `I` or the
//! loop's first block.

use std::collections::{BTreeMap, BTreeSet};

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{BlockGraph, BlockKind, BlockType, block_flags};
use crate::context::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p3_dataflow::kuna_strlenscan::taken;

/// The basic block of the last leaf of `id`, through lists only.
fn last_basic(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    let b = graph.block(id);
    match b.get_type() {
        BlockType::Copy => b.get_copy(),
        BlockType::Ls | BlockType::Graph => last_basic(graph, *b.get_list().last()?),
        _ => None,
    }
}

/// Every leaf a plain goto under `root` lands on (`root` itself when a multi-goto
/// leaves the targets unknown).
fn goto_targets(graph: &BlockGraph, root: BlockId) -> BTreeSet<BlockId> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        let target = match b.get_type() {
            BlockType::Goto if b.get_goto_type() == block_flags::f_goto_goto => b.get_goto_target(),
            BlockType::If if b.get_if_goto_type() == block_flags::f_goto_goto => {
                b.get_if_goto_target()
            }
            BlockType::MultiGoto => return [root].into_iter().collect(),
            _ => None,
        };
        out.extend(target.and_then(|t| graph.get_front_leaf(t)));
        stack.extend(b.get_list().iter().copied());
    }
    out
}

fn basics_under(graph: &BlockGraph, root: BlockId, out: &mut BTreeSet<BlockId>) {
    let b = graph.block(root);
    if b.get_type() == BlockType::Copy {
        out.extend(b.get_copy());
    }
    for &c in b.get_list() {
        basics_under(graph, c, out);
    }
}

/// The CBRANCH condition of basic block `bb` and the sense in which it reaches `to`.
fn branch_to(data: &Funcdata, bb: BlockId, to: BlockId) -> Option<(VarnodeId, bool)> {
    let (t, f) = taken(data, bb)?;
    let sense = if t == to {
        true
    } else if f == to {
        false
    } else {
        return None;
    };
    let cond = data.obank().get(data.bb_op_tail(bb)?)?.get_in(1)?;
    Some((cond, sense))
}

struct Inits {
    /// The HighVariable each assignment writes, with the op that writes it last.
    writes: BTreeMap<HighVariableId, OpId>,
    /// Some assignment reads memory.
    loads: bool,
}

/// The assignments of the leaves `leaves`, or `None` when one calls, stores or
/// writes no variable.
fn inits(data: &Funcdata, graph: &BlockGraph, leaves: &[BlockId]) -> Option<Inits> {
    let mut writes = BTreeMap::new();
    let mut loads = false;
    for &leaf in leaves {
        let b = graph.block(leaf);
        if b.get_type() != BlockType::Copy {
            return None;
        }
        for op in data.bb_ops(b.get_copy()?) {
            let o = data.obank().get(op)?;
            match o.code() {
                OpCode::CPUI_BRANCH => continue,
                OpCode::CPUI_LOAD => loads = true,
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_STORE
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_RETURN => return None,
                _ => {}
            }
            if o.not_printed() {
                continue;
            }
            let out = data.vbank().get(o.get_out()?)?;
            if !out.is_implied() {
                writes.insert(out.get_high()?, op);
            }
        }
    }
    Some(Inits { writes, loads })
}

struct Compare<'a> {
    data: &'a Funcdata,
    writes: &'a BTreeMap<HighVariableId, OpId>,
    /// A call may stand on both sides: only when the guard goes, so the loop's
    /// first test makes the guard's one call.
    calls: bool,
}

/// One side of a compare: a varnode, or a constant (`(value, size)`) the
/// normalisation of `k <= x` into `k - 1 < x` made.
#[derive(Clone, Copy)]
enum Operand {
    Var(VarnodeId),
    Const(u64, i32),
}

#[derive(Clone, Copy, PartialEq)]
enum Rel {
    Eq,
    Ne,
    Less(bool),
    LessEq(bool),
}

/// A test as `a rel b`, its sense applied (a false sense negates it).
#[derive(Clone, Copy)]
struct Test {
    rel: Rel,
    a: Operand,
    b: Operand,
}

impl Compare<'_> {
    fn implied_def(&self, v: VarnodeId) -> Option<&crate::op::PcodeOp> {
        let n = self.data.vbank().get(v)?;
        if !n.is_implied() {
            return None;
        }
        self.data.obank().get(n.get_def()?)
    }

    fn operand(&self, v: VarnodeId) -> Operand {
        match self.data.vbank().get(v) {
            Some(n) if n.is_constant() => Operand::Const(n.get_offset(), n.get_size()),
            _ => Operand::Var(v),
        }
    }

    /// The assignment of the inits that `c` reads, when it reads one.
    fn init_of(&self, c: VarnodeId) -> Option<OpId> {
        let n = self.data.vbank().get(c)?;
        if n.is_implied() || n.is_constant() {
            return None;
        }
        self.writes.get(&n.get_high()?).copied()
    }

    /// Does the guard's `g` equal the loop test's `c` at the loop's entry?  Inside
    /// an assignment's right-hand side (`rhs`) nothing is substituted.
    fn same(&self, g: Operand, c: Operand, rhs: bool, depth: u32) -> bool {
        if depth > 12 {
            return false;
        }
        if let Operand::Var(cv) = c {
            if let Some(w) = self.init_of(cv) {
                return !rhs && self.same_op(g, w, depth + 1);
            }
        }
        let (g, c) = match (g, c) {
            (Operand::Const(x, xs), Operand::Const(y, ys)) => return x == y && xs == ys,
            (Operand::Var(g), Operand::Var(c)) => (g, c),
            _ => return false,
        };
        let (Some(gn), Some(cn)) = (self.data.vbank().get(g), self.data.vbank().get(c)) else {
            return false;
        };
        if !cn.is_implied() {
            return !gn.is_implied() && cn.get_high().is_some() && gn.get_high() == cn.get_high();
        }
        match (self.implied_def(g), self.implied_def(c)) {
            (Some(a), Some(b)) => self.same_ops(a, b, rhs, depth + 1),
            _ => false,
        }
    }

    /// Is `g` the value the assignment `w` computes?
    fn same_op(&self, g: Operand, w: OpId, depth: u32) -> bool {
        let Some(o) = self.data.obank().get(w) else {
            return false;
        };
        if o.code() == OpCode::CPUI_COPY {
            return o
                .get_in(0)
                .is_some_and(|x| self.same(g, self.operand(x), true, depth));
        }
        match g {
            Operand::Var(g) => self
                .implied_def(g)
                .is_some_and(|a| self.same_ops(a, o, true, depth)),
            Operand::Const(..) => false,
        }
    }

    fn same_ops(
        &self,
        a: &crate::op::PcodeOp,
        b: &crate::op::PcodeOp,
        rhs: bool,
        depth: u32,
    ) -> bool {
        use OpCode::*;
        let call = matches!(a.code(), CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER);
        if a.code() != b.code()
            || a.num_input() != b.num_input()
            || (call && !self.calls)
            || matches!(a.code(), CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_STORE)
        {
            return false;
        }
        let ins = |o: &crate::op::PcodeOp| {
            (0..o.num_input())
                .map(|i| o.get_in(i).map(|v| self.operand(v)))
                .collect::<Option<Vec<_>>>()
        };
        let (Some(x), Some(y)) = (ins(a), ins(b)) else {
            return false;
        };
        let pairwise = |x: &[Operand], y: &[Operand]| {
            x.iter().zip(y).all(|(&p, &q)| self.same(p, q, rhs, depth))
        };
        pairwise(&x, &y) || (commutative(a.code()) && x.len() == 2 && pairwise(&x, &[y[1], y[0]]))
    }

    /// `v` under `sense` as a compare, BOOL_NEGATEs folded into the sense and
    /// `k <= x` / `x <= k` against a constant written `k - 1 < x` / `x < k + 1`.
    fn test(&self, mut v: VarnodeId, mut sense: bool) -> Option<Test> {
        use OpCode::*;
        let o = loop {
            let o = self.implied_def(v)?;
            if o.code() != CPUI_BOOL_NEGATE {
                break o;
            }
            v = o.get_in(0)?;
            sense = !sense;
        };
        let (a, b) = (self.operand(o.get_in(0)?), self.operand(o.get_in(1)?));
        let rel = match o.code() {
            CPUI_INT_EQUAL => Rel::Eq,
            CPUI_INT_NOTEQUAL => Rel::Ne,
            CPUI_INT_LESS => Rel::Less(false),
            CPUI_INT_SLESS => Rel::Less(true),
            CPUI_INT_LESSEQUAL => Rel::LessEq(false),
            CPUI_INT_SLESSEQUAL => Rel::LessEq(true),
            _ => return None,
        };
        let t = if sense {
            Test { rel, a, b }
        } else {
            match rel {
                Rel::Eq => Test { rel: Rel::Ne, a, b },
                Rel::Ne => Test { rel: Rel::Eq, a, b },
                Rel::Less(s) => Test {
                    rel: Rel::LessEq(s),
                    a: b,
                    b: a,
                },
                Rel::LessEq(s) => Test {
                    rel: Rel::Less(s),
                    a: b,
                    b: a,
                },
            }
        };
        Some(normalise(t))
    }

    fn same_test(&self, g: Test, c: Test) -> bool {
        let pair = |x: Operand, y: Operand, p: Operand, q: Operand| {
            self.same(x, p, false, 0) && self.same(y, q, false, 0)
        };
        g.rel == c.rel
            && (pair(g.a, g.b, c.a, c.b)
                || (matches!(g.rel, Rel::Eq | Rel::Ne) && pair(g.a, g.b, c.b, c.a)))
    }

    /// Does the guard `(g, gs)` enter the arm exactly when `(c, cs)` continues the loop?
    fn equivalent(&self, (g, gs): (VarnodeId, bool), (c, cs): (VarnodeId, bool)) -> bool {
        match (self.test(g, gs), self.test(c, cs)) {
            (Some(gt), Some(ct)) => self.same_test(gt, ct),
            (None, None) => gs == cs && self.same(self.operand(g), self.operand(c), false, 0),
            _ => false,
        }
    }

    /// Does the guard imply that the loop test holds on entry: equivalence, or a
    /// strict `<` guard for a `!=` test on the same operands?
    fn implies(&self, g: (VarnodeId, bool), c: (VarnodeId, bool)) -> bool {
        if self.equivalent(g, c) {
            return true;
        }
        let (Some(gt), Some(ct)) = (self.test(g.0, g.1), self.test(c.0, c.1)) else {
            return false;
        };
        matches!(gt.rel, Rel::Less(_))
            && ct.rel == Rel::Ne
            && self.same_test(Test { rel: Rel::Ne, ..gt }, ct)
    }
}

/// `k <= x` as `k - 1 < x` and `x <= k` as `x < k + 1`, where the constant does
/// not wrap.
fn normalise(t: Test) -> Test {
    let Rel::LessEq(signed) = t.rel else { return t };
    let bounds = |size: i32| {
        let bits = (size.clamp(1, 8) * 8) as u32;
        let mask = if bits == 64 {
            u64::MAX
        } else {
            (1u64 << bits) - 1
        };
        let (min, max) = if signed {
            (1u64 << (bits - 1), (1u64 << (bits - 1)) - 1)
        } else {
            (0, mask)
        };
        (mask, min, max)
    };
    match (t.a, t.b) {
        (Operand::Const(k, size), _) => {
            let (mask, min, _) = bounds(size);
            if k & mask == min {
                return t;
            }
            Test {
                rel: Rel::Less(signed),
                a: Operand::Const(k.wrapping_sub(1) & mask, size),
                b: t.b,
            }
        }
        (_, Operand::Const(k, size)) => {
            let (mask, _, max) = bounds(size);
            if k & mask == max {
                return t;
            }
            Test {
                rel: Rel::Less(signed),
                a: t.a,
                b: Operand::Const(k.wrapping_add(1) & mask, size),
            }
        }
        _ => t,
    }
}

fn commutative(code: OpCode) -> bool {
    use OpCode::*;
    matches!(
        code,
        CPUI_INT_ADD
            | CPUI_INT_AND
            | CPUI_INT_OR
            | CPUI_INT_XOR
            | CPUI_INT_MULT
            | CPUI_INT_EQUAL
            | CPUI_INT_NOTEQUAL
            | CPUI_BOOL_AND
            | CPUI_BOOL_OR
            | CPUI_BOOL_XOR
    )
}

/// Every instance of `h` is written by an op in one of `blocks`.
/// Is no value of `h` that reaches the guard block `guard` read again after
/// the guard's skip edge to `join`?  Then the arm's assignments to `h` may run
/// on that path too.  A use counts when its block is reachable from `join`
/// before the value's definition runs again.
fn dead_after_skip(
    data: &Funcdata,
    h: HighVariableId,
    guard: BlockId,
    join: BlockId,
    arm: &BTreeSet<BlockId>,
) -> bool {
    let Some(high) = data.high_bank().get(h) else {
        return false;
    };
    let graph = data.bblocks_ref();
    (0..high.num_instances()).all(|i| {
        let Some(v) = data.vbank().get(high.get_instance(i)) else {
            return false;
        };
        let def = v
            .get_def()
            .and_then(|d| data.obank().get(d))
            .and_then(|o| o.get_parent());
        if def.is_some_and(|d| arm.contains(&d) || !graph.dominates(d, Some(guard))) {
            return true;
        }
        let mut seen = BTreeSet::new();
        let mut stack = vec![join];
        while let Some(b) = stack.pop() {
            if Some(b) == def || !seen.insert(b) {
                continue;
            }
            let bb = graph.block(b);
            stack.extend((0..bb.size_out()).map(|k| bb.get_out(k)));
        }
        v.descend_iter().all(|op| {
            data.obank()
                .get(op)
                .and_then(|o| o.get_parent())
                .is_some_and(|b| !seen.contains(&b))
        })
    })
}

/// The rewrite for the do-while `w`: `Some(Some(if))` when the guard `if` goes as
/// well, `Some(None)` when only the test moves to the top.
fn plan(data: &Funcdata, w: BlockId, targets: &BTreeSet<BlockId>) -> Option<Option<BlockId>> {
    let graph = data.sblocks_ref();
    let body = graph.block(w).get_block(0);
    let head_leaf = graph.get_front_leaf(body)?;
    let head = graph.block(head_leaf).get_copy()?;
    if targets.contains(&head_leaf) {
        return None;
    }
    let latch = last_basic(graph, body)?;
    let cond = branch_to(data, latch, head)?;
    let parent = graph.block(w).get_parent()?;
    let (arm, list, guard) = if matches!(
        graph.block(parent).get_type(),
        BlockType::Ls | BlockType::Graph
    ) {
        (
            parent,
            graph.block(parent).get_list().to_vec(),
            graph.block(parent).get_parent()?,
        )
    } else {
        (w, vec![w], parent)
    };
    let at = list.iter().position(|&m| m == w)?;
    let gb = graph.block(guard);
    if gb.get_type() != BlockType::If
        || !(1..gb.get_size()).any(|i| gb.get_block(i) == arm)
        || gb.get_if_goto_target().is_some()
    {
        return None;
    }
    let cond_leaf = gb.get_block(0);
    let gbb = last_basic(graph, cond_leaf)?;
    let first = graph.get_front_leaf(arm)?;
    let entry = graph.block(first).get_copy()?;
    let gcond = branch_to(data, gbb, entry)?;
    let join = taken(data, gbb).map(|(t, f)| if t == entry { f } else { t })?;
    let leaves = &list[..at];
    if leaves.iter().any(|l| targets.contains(l)) {
        return None;
    }
    let found = inits(data, graph, leaves)?;
    let mut blocks = BTreeSet::new();
    basics_under(graph, arm, &mut blocks);
    let droppable = gb.get_size() == 2
        && at + 1 == list.len()
        && !found.loads
        && found
            .writes
            .keys()
            .all(|&h| dead_after_skip(data, h, gbb, join, &blocks));
    let cmp = Compare {
        data,
        writes: &found.writes,
        calls: true,
    };
    if droppable && cmp.equivalent(gcond, cond) {
        return Some(Some(guard));
    }
    (Compare {
        calls: false,
        ..cmp
    })
    .implies(gcond, cond)
    .then_some(None)
}

/// Print each qualifying guarded do-while as a while loop (see the module docs);
/// returns how many loops changed.
pub(crate) fn guardedloop_apply(data: &mut Funcdata) -> i32 {
    if !data.get_arch().guarded_loop || data.sblocks_get_size() == 0 {
        return 0;
    }
    let root = data.sblocks_root();
    let targets = goto_targets(data.sblocks_ref(), root);
    let mut loops = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = data.sblocks_ref().block(id);
        stack.extend(b.get_list().iter().copied());
        if b.get_type() == BlockType::DoWhile && b.get_flags() & block_flags::f_kuna_top_test == 0 {
            loops.push(id);
        }
    }
    let mut count = 0;
    for w in loops {
        let Some(drop) = plan(data, w, &targets) else {
            continue;
        };
        let graph = data.sblocks_mut();
        graph.block_mut(w).set_flag(block_flags::f_kuna_top_test);
        if let Some(guard) = drop {
            *graph.block_mut(guard).kind_mut() = BlockKind::Ls;
        }
        count += 1;
    }
    count
}

/// Runs [`guardedloop_apply`].
pub struct ActionGuardedLoop {
    base: ActionBase,
}

impl ActionGuardedLoop {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionGuardedLoop {
            base: ActionBase::new(0, "guardedloop", g),
        })
    }
}

impl Action for ActionGuardedLoop {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| {
            Box::new(ActionGuardedLoop {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        self.base.count += guardedloop_apply(data);
        0
    }
}
