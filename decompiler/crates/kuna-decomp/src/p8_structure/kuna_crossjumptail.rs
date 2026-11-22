//! S8 shared-run duplication: `crossjumprevert` past one block.
//!
//! [`crate::p8_structure::kuna_crossjumpreverter`] duplicates one cross-jump
//! tail block into a plain `goto`.  Cross-jumping also shares longer runs: a
//! statement or two, an `if`, a call, reached by `goto` from inside nested
//! arms, and the latch of a loop, reached from deep in its body.  On the final
//! tree, after `gotosink`, a goto `G` to the leaf `L` that starts the run `R` in
//! its list (or a whole `if` arm) is rewritten when flow after `R` is known:
//!
//! * **Converged.**  The flow after `G` (were it a fall-through) is the flow
//!   after `R`: `R` is copied after `G`'s body and the jump goes.  The end of
//!   a switch case that ends in `break;` is the switch's exit, and so is the
//!   end of the case `G` ends, which then ends in `break;` too.
//! * **Else arm.**  `G` is `if (c) goto L;`, or ends the only arm of an `if`
//!   standing at index `i` of a list, and flow after that list is the flow after
//!   `R`: the list's components after `i` become the `if`'s else arm and `R` is
//!   copied in place of the jump.  When `L` itself is a later member of that
//!   list, the members in between are the else arm and nothing is copied.
//! * **Latch** (`loopcontinue`).  `R` runs to the end of the body of the loop
//!   that is `G`'s innermost loop: `R` is copied before the jump, which prints
//!   `continue;` (for a do-while only where the language's `continue;` runs
//!   the test).
//! * **Break** (`loopbreak_recovery`).  `L` is where flow goes after `G`'s
//!   innermost loop, with no switch in between: the jump prints `break;`.
//!
//! `R` is the shortest run from `L` that converges, at most [`MAX_RUN_OPS`]
//! printed ops and [`MAX_RUN_CALLS`] calls, built of statements and `if`s (no
//! jump, loop, switch or `return`, no `for` header statement), and never holds
//! `G`.  The copies are fresh print-tree nodes over the same basic blocks, so the
//! printer re-emits their ops on the new path; no p-code changes.  A label no
//! plain goto reaches any more is released.

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{BlockGraph, BlockKind, BlockType, block_flags};
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;

/// The most printed p-code ops a duplicated run holds.
pub const MAX_RUN_OPS: usize = 12;
/// The most calls a duplicated run holds.
pub const MAX_RUN_CALLS: usize = 1;

fn is_list(graph: &BlockGraph, id: BlockId) -> bool {
    matches!(graph.block(id).get_type(), BlockType::Ls | BlockType::Graph)
}

fn is_loop(graph: &BlockGraph, id: BlockId) -> bool {
    matches!(
        graph.block(id).get_type(),
        BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop
    )
}

/// The target of a plain `goto` node or an `if (c) goto` node.
fn jump_target(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    let b = graph.block(id);
    match b.get_type() {
        BlockType::Goto if b.get_goto_type() == block_flags::f_goto_goto => b.get_goto_target(),
        BlockType::If if b.get_size() == 1 && b.get_if_goto_type() == block_flags::f_goto_goto => {
            b.get_if_goto_target()
        }
        _ => None,
    }
}

fn index_in(graph: &BlockGraph, list: BlockId, id: BlockId) -> Option<usize> {
    graph.block(list).get_list().iter().position(|&c| c == id)
}

fn contains(graph: &BlockGraph, root: BlockId, id: BlockId) -> bool {
    let mut cur = Some(id);
    while let Some(c) = cur {
        if c == root {
            return true;
        }
        cur = graph.block(c).get_parent();
    }
    false
}

/// `(list, index)` of the list member that starts with `leaf`, or `(if, arm)`
/// of the `if` arm that does.
fn run_start(graph: &BlockGraph, leaf: BlockId) -> Option<(BlockId, usize)> {
    let mut e = leaf;
    loop {
        let p = graph.block(e).get_parent()?;
        if is_list(graph, p) {
            return Some((p, index_in(graph, p, e)?));
        }
        if graph.block(p).get_type() == BlockType::If && graph.block(p).get_block(0) != e {
            return Some((p, index_in(graph, p, e)?));
        }
        if !matches!(
            graph.block(p).get_type(),
            BlockType::If | BlockType::Goto | BlockType::Condition
        ) || graph.get_front_leaf(p) != Some(leaf)
        {
            return None;
        }
        e = p;
    }
}

fn flow_after(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    graph.next_flow_after(graph.block(id).get_parent()?, id)
}

/// The ops a `for` header prints in place of their own statements.
fn header_ops(graph: &BlockGraph, root: BlockId) -> Vec<OpId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        out.extend(b.get_iterate_op());
        out.extend(b.get_initialize_op());
        stack.extend(b.get_list().iter().copied());
    }
    out
}

/// Printed ops and calls under `id`, or `None` when it cannot be copied.
fn run_cost(data: &Funcdata, id: BlockId, header: &[OpId]) -> Option<(usize, usize)> {
    let graph = data.sblocks_ref();
    let b = graph.block(id);
    match b.get_type() {
        BlockType::Copy => {
            let bb = b.get_copy()?;
            let (mut ops, mut calls) = (0, 0);
            let mut cur = data.bb_op_head(bb);
            while let Some(o) = cur {
                cur = data.bb_op_next(o);
                let op = data.obank().get(o)?;
                if header.contains(&o) || op.code() == OpCode::CPUI_RETURN {
                    return None;
                }
                if op.not_printed() || op.is_branch() {
                    continue;
                }
                ops += 1;
                if matches!(
                    op.code(),
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
                ) {
                    calls += 1;
                }
            }
            Some((ops, calls))
        }
        BlockType::Goto => None,
        BlockType::If if b.get_if_goto_target().is_some() => None,
        BlockType::Ls | BlockType::Graph | BlockType::If | BlockType::Condition => {
            let (mut ops, mut calls) = (0, 0);
            for &c in b.get_list() {
                let (o, k) = run_cost(data, c, header)?;
                ops += o;
                calls += k;
            }
            Some((ops, calls))
        }
        _ => None,
    }
}

/// Where flow goes: a leaf, or out of a switch through a case that ends in `break;`.
#[derive(Clone, Copy, PartialEq)]
enum Dest {
    Leaf(BlockId),
    SwitchExit(BlockId),
}

/// Where flow goes after `child` of `parent`, as [`BlockGraph::next_flow_after`],
/// except that the end of a case that exits its switch reaches that switch's
/// exit.  With `site`, the case that `child` ends (through last list members
/// only) counts as exiting: rewriting the jump that ends it makes it so.
/// Returns the switch and that case block alongside.
fn dest(
    graph: &BlockGraph,
    mut parent: BlockId,
    mut child: BlockId,
    site: bool,
) -> Option<(Dest, Option<BlockId>)> {
    loop {
        let p = graph.block(parent);
        match p.get_type() {
            BlockType::Switch => {
                let case = p.switch_caseblocks().iter().find(|c| c.block == child)?;
                return if case.isexit {
                    Some((Dest::SwitchExit(parent), None))
                } else if site && case.gototype == 0 {
                    Some((Dest::SwitchExit(parent), Some(child)))
                } else {
                    graph
                        .next_flow_after(parent, child)
                        .map(|l| (Dest::Leaf(l), None))
                };
            }
            BlockType::Ls | BlockType::Graph if p.get_list().last() == Some(&child) => {
                child = parent;
                parent = p.get_parent()?;
            }
            BlockType::If if !site && graph.block(parent).get_block(0) != child => {
                child = parent;
                parent = p.get_parent()?;
            }
            _ => {
                return graph
                    .next_flow_after(parent, child)
                    .map(|l| (Dest::Leaf(l), None));
            }
        }
    }
}

enum Plan {
    /// Copy `R` after `G`'s body; mark the case `G` ended as exiting its switch.
    Converged(usize, Option<(BlockId, BlockId)>),
    /// Move the members after the carrier into its else arm, then copy `R`.
    ElseArm {
        carrier: BlockId,
        list: BlockId,
        at: usize,
        run_end: Option<usize>,
    },
    /// Copy `R` (to the end of the body of loop `w`) and print `continue;`.
    Latch(BlockId),
    /// `L` is where flow goes after `G`'s innermost loop: print `break;`.
    Break,
}

/// The `if` whose only arm `G` ends, with its list and index; `G` itself when
/// it is an `if (c) goto`.
fn carrier(graph: &BlockGraph, g: BlockId) -> Option<(BlockId, BlockId, usize)> {
    let mut x = g;
    if graph.block(g).get_type() != BlockType::If {
        loop {
            let p = graph.block(x).get_parent()?;
            match graph.block(p).get_type() {
                BlockType::Ls | BlockType::Graph
                    if graph.block(p).get_list().last() == Some(&x) =>
                {
                    x = p
                }
                BlockType::If
                    if graph.block(p).get_size() == 2 && graph.block(p).get_block(1) == x =>
                {
                    x = p;
                    break;
                }
                _ => return None,
            }
        }
    }
    let list = graph.block(x).get_parent()?;
    is_list(graph, list).then_some(())?;
    Some((x, list, index_in(graph, list, x)?))
}

/// The innermost loop holding `id`.
fn innermost_loop(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    let mut cur = graph.block(id).get_parent();
    while let Some(c) = cur {
        if is_loop(graph, c) {
            return Some(c);
        }
        cur = graph.block(c).get_parent();
    }
    None
}

/// Is there a switch between `id` and its enclosing `w`?  A `break;` there
/// would leave the switch.
fn in_switch(graph: &BlockGraph, id: BlockId, w: BlockId) -> bool {
    let mut cur = graph.block(id).get_parent();
    while let Some(c) = cur.filter(|&c| c != w) {
        if graph.block(c).get_type() == BlockType::Switch {
            return true;
        }
        cur = graph.block(c).get_parent();
    }
    false
}

/// The body list of loop `w`.
fn loop_body(graph: &BlockGraph, w: BlockId) -> Option<BlockId> {
    let b = graph.block(w);
    match b.get_type() {
        BlockType::WhileDo if b.get_iterate_op().is_none() => Some(b.get_block(1)),
        BlockType::DoWhile | BlockType::InfLoop => Some(b.get_block(0)),
        _ => None,
    }
}

/// Which rewrites the options allow.
#[derive(Clone, Copy)]
struct Gates {
    /// `crossjumprevert`: copy a run.
    tails: bool,
    /// `crossjumprevert` and `loopcontinue`: copy a latch run and print `continue;`.
    latch: bool,
    /// `loopbreak_recovery`: print `break;`.
    brk: bool,
    /// A `continue;` in the output language's do-while runs its test.
    do_while_continue: bool,
}

fn plan(
    data: &Funcdata,
    g: BlockId,
    header: &[OpId],
    gates: Gates,
) -> Option<(BlockId, usize, Plan)> {
    let graph = data.sblocks_ref();
    let leaf = graph.get_front_leaf(jump_target(graph, g)?)?;
    if let Some(w) = innermost_loop(graph, g).filter(|_| gates.brk) {
        if !in_switch(graph, g, w) && flow_after(graph, w) == Some(leaf) {
            return Some((w, 0, Plan::Break));
        }
    }
    if !gates.tails {
        return None;
    }
    let (rl, k) = run_start(graph, leaf)?;
    let members = if is_list(graph, rl) {
        graph.block(rl).get_list().to_vec()
    } else {
        graph.block(rl).get_list()[..=k].to_vec()
    };
    let site = dest(graph, graph.block(g).get_parent()?, g, true);
    let carried = carrier(graph, g);
    if let Some((c, list, at)) = carried {
        if c != g && list == rl && k > at + 1 {
            return Some((
                rl,
                k,
                Plan::ElseArm {
                    carrier: c,
                    list,
                    at,
                    run_end: None,
                },
            ));
        }
    }
    let rest_flow = carried.and_then(|(_, list, _)| {
        dest(graph, list, *graph.block(list).get_list().last()?, false).map(|(d, _)| d)
    });
    let (mut ops, mut calls) = (0, 0);
    for m in k..members.len() {
        if contains(graph, members[m], g) {
            return None;
        }
        let (o, c) = run_cost(data, members[m], header)?;
        ops += o;
        calls += c;
        if ops > MAX_RUN_OPS || calls > MAX_RUN_CALLS {
            return None;
        }
        let after = dest(graph, rl, members[m], false).map(|(d, _)| d);
        if let Some((d, case)) = site.filter(|(d, _)| Some(*d) == after) {
            let mark = match (d, case) {
                (Dest::SwitchExit(s), Some(c)) => Some((s, c)),
                _ => None,
            };
            return Some((rl, k, Plan::Converged(m, mark)));
        }
        if let Some((c, list, at)) = carried {
            if after.is_some() && after == rest_flow && !contains(graph, list, members[k]) {
                return Some((
                    rl,
                    k,
                    Plan::ElseArm {
                        carrier: c,
                        list,
                        at,
                        run_end: Some(m),
                    },
                ));
            }
        }
    }
    let w = graph.block(rl).get_parent()?;
    let continues = graph.block(w).get_type() != BlockType::DoWhile || gates.do_while_continue;
    (gates.latch
        && continues
        && k > 0
        && is_loop(graph, w)
        && loop_body(graph, w) == Some(rl)
        && innermost_loop(graph, g) == Some(w))
    .then_some((rl, k, Plan::Latch(w)))
}

/// Copies of `members[k..=m]` of `rl`, in a new list.
fn copy_run(graph: &mut BlockGraph, rl: BlockId, k: usize, m: usize) -> BlockId {
    let members = graph.block(rl).get_list()[k..=m].to_vec();
    let list = graph.kuna_new_node(BlockKind::Ls);
    for c in members {
        let copy = graph.kuna_clone_subtree(c);
        graph.kuna_push_component(list, copy);
    }
    list
}

/// Put `body` where `g`'s jump was: after a `goto` node's body, or as an `if`'s arm.
fn replace_jump(graph: &mut BlockGraph, g: BlockId, body: BlockId) {
    if graph.block(g).get_type() == BlockType::If {
        graph.kuna_push_component(g, body);
        if let BlockKind::If { gototarget, .. } = graph.block_mut(g).kind_mut() {
            *gototarget = None;
        }
    } else {
        graph.kuna_push_component(g, body);
        *graph.block_mut(g).kind_mut() = BlockKind::Ls;
    }
}

fn apply_once(data: &mut Funcdata, gates: Gates) -> Option<BlockId> {
    let root = data.sblocks_root();
    let header = header_ops(data.sblocks_ref(), root);
    let mut stack = vec![root];
    let mut found = None;
    while let Some(id) = stack.pop() {
        let graph = data.sblocks_ref();
        stack.extend(graph.block(id).get_list().iter().copied());
        if jump_target(graph, id).is_some() {
            if let Some(p) = plan(data, id, &header, gates) {
                found = Some((id, p));
                break;
            }
        }
    }
    let (g, (rl, k, plan)) = found?;
    let graph = data.sblocks_mut();
    let leaf = graph.get_front_leaf(jump_target(graph, g)?)?;
    match plan {
        Plan::Converged(m, mark) => {
            let run = copy_run(graph, rl, k, m);
            replace_jump(graph, g, run);
            if let Some((s, c)) = mark {
                let case = graph
                    .block_mut(s)
                    .switch_caseblocks_mut()?
                    .iter_mut()
                    .find(|o| o.block == c)?;
                case.isexit = true;
            }
        }
        Plan::ElseArm {
            carrier,
            list,
            at,
            run_end,
        } => {
            let run = run_end.map(|m| copy_run(graph, rl, k, m));
            let len = graph.block(list).get_list().len();
            let rest_end = if run_end.is_none() { k } else { len };
            let rest = (at + 1 < rest_end)
                .then(|| graph.kuna_wrap_components(list, at + 1, rest_end - 1, BlockKind::Ls));
            match run {
                Some(run) => replace_jump(graph, g, run),
                None => *graph.block_mut(g).kind_mut() = BlockKind::Ls,
            }
            if let Some(rest) = rest {
                let at_rest = index_in(graph, list, rest)?;
                graph.kuna_move_component(list, at_rest, carrier);
            }
        }
        Plan::Break => {
            if graph.block(g).get_type() == BlockType::If {
                if let BlockKind::If { gototype, .. } = graph.block_mut(g).kind_mut() {
                    *gototype = block_flags::f_break_goto;
                }
            } else if let BlockKind::Goto { gototype, .. } = graph.block_mut(g).kind_mut() {
                *gototype = block_flags::f_break_goto;
            }
        }
        Plan::Latch(w) => {
            let end = graph.block(rl).get_list().len() - 1;
            let run = copy_run(graph, rl, k, end);
            let head = graph.get_front_leaf(w)?;
            if graph.block(g).get_type() == BlockType::If {
                let jump = graph.kuna_new_node(BlockKind::Goto {
                    gototarget: Some(head),
                    gototype: block_flags::f_continue_goto,
                });
                graph.kuna_push_component(jump, run);
                graph.kuna_push_component(g, jump);
                if let BlockKind::If { gototarget, .. } = graph.block_mut(g).kind_mut() {
                    *gototarget = None;
                }
            } else {
                let body = graph.block(g).get_block(0);
                let seq = graph.kuna_new_node(BlockKind::Ls);
                graph.kuna_set_component(g, 0, seq);
                graph.kuna_push_component(seq, body);
                graph.kuna_push_component(seq, run);
                if let BlockKind::Goto {
                    gototarget,
                    gototype,
                } = graph.block_mut(g).kind_mut()
                {
                    *gototarget = Some(head);
                    *gototype = block_flags::f_continue_goto;
                }
            }
        }
    }
    Some(leaf)
}

/// Rewrite gotos into shared runs to a fixpoint; returns how many were rewritten.
pub(crate) fn crossjumptail_apply(data: &mut Funcdata) -> i32 {
    let arch = data.get_arch();
    let gates = Gates {
        tails: arch.revert_cross_jumps,
        latch: arch.revert_cross_jumps && arch.loop_continue,
        brk: arch.recover_loop_break,
        do_while_continue: arch.do_while_continue,
    };
    if !(gates.tails || gates.brk) || data.sblocks_get_size() == 0 {
        return 0;
    }
    let root = data.sblocks_root();
    data.sblocks_mut().kuna_flatten_lists(root);
    let mut targets = Vec::new();
    for _ in 0..64 {
        match apply_once(data, gates) {
            Some(t) => targets.push(t),
            None => break,
        }
    }
    if !targets.is_empty() {
        crate::p8_structure::kuna_gotosink::release_labels(data, &targets);
    }
    targets.len() as i32
}

/// Runs [`crossjumptail_apply`].
pub struct ActionCrossJumpTail {
    base: ActionBase,
}

impl ActionCrossJumpTail {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionCrossJumpTail {
            base: ActionBase::new(0, "crossjumptail", g),
        })
    }
}

impl Action for ActionCrossJumpTail {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| {
            Box::new(ActionCrossJumpTail {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        self.base.count += crossjumptail_apply(data);
        0
    }
}
