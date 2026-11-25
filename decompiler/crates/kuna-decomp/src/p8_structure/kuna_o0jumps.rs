//! (kuna `loopcontinue auto`) Keep the tests an unoptimized function wrote
//! as separate jump statements apart.
//!
//! An unoptimizing compiler compiles a bare `continue;`, `break;` or `goto`
//! that is the whole arm of an `if` into a block of its own, kept so a
//! debugger can stop on the statement's line, while a structured `if` that
//! contains no jump statement branches straight to its join:
//!
//! ```text
//!     je   L1          ; c == NULL           for (...) {
//!     jne  L1          ; c->type != 7            if (c == NULL || c->type != 7)
//!     je   L2          ; c->addr == NULL              continue;
//!     jne  NEXT        ; strcmp(...) != 0        if (c->addr == NULL)
//!     ...body...                                      continue;
//! L1: nop; jmp NEXT                              if (strcmp(...) == 0) { ... }
//! L2: nop                                     }
//! NEXT: i++
//! ```
//!
//! gcc writes the statement as a `nop` the branch lands on, followed by a
//! jump to the statement's target or falling into it; `if (c) continue;` in a
//! `while` whose target is the loop test is instead `jcc L; jmp T; L:`, a
//! jump-only block the test falls into, and only the last test of a `||`
//! group goes through it (the earlier ones jump straight to the target).
//! Simplification removes those blocks, after which every test reaches the
//! same block and the short-circuit fold joins them into one `&&` chain.
//!
//! [`record`] reads the raw graph at the first heritage pass of a function
//! that [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized and labels
//! each edge into such a jump statement with the statement's id.  A removed
//! block's in-edges keep their labels when they are re-pointed at its
//! successor, so when the structurer folds two tests that share a successor
//! the labels still say whether the source wrote them as one statement:
//! [`may_join`] lets the fold join two edges into a loop's step or test (a
//! `continue;`) or out of a loop (a `break;`) only when they carry the same
//! id.  A recorded `break` into the function's one recorded return stays a
//! `break` ([`keeps_return`]), and after structuring a loop body's last
//! `if (c) { S }` whose test reaches the loop's continue point only through
//! recorded statements prints as `if (!c) continue; S` ([`print_continues`]).
//! When keeping the tests apart leaves more `goto`s than folding them, the
//! function is structured as before ([`gotos`]).

use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::block::{block_flags, BlockGraph, BlockType};
use crate::context::BlockId;
use crate::funcdata::Funcdata;

/// Edge-label bits that carry a jump-statement id (1-based); the ported edge
/// flags use bits 0..9 and `returndup auto` bits 16..30.
const ID_SHIFT: u32 = 10;
const ID_MASK: uint4 = 0x3f << ID_SHIFT;
const MAX_ID: u32 = 0x3f;
/// The widest run of instructions without p-code a branch may land on.
const MAX_GAP: u64 = 8;
/// The widest jump instruction a test may skip over.
const MAX_JUMP: u64 = 6;

/// What the first heritage pass recorded.
#[derive(Debug, Clone, Default)]
pub struct Record {
    /// Jump statements labelled.
    pub statements: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Key {
    /// A jump-only block.
    Block(BlockId),
    /// The address of a run of `nop`s that falls into its target.
    Landing(u64),
}

/// Read and label the jump statements; once, at the first heritage pass.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0jumps.is_some() {
        return;
    }
    let arch = fd.get_arch();
    if !arch.loop_continue || !arch.loop_continue_record {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let mut stmts: Vec<(Key, Vec<(BlockId, int4)>)> = Vec::new();
    let add = |stmts: &mut Vec<(Key, Vec<(BlockId, int4)>)>, key: Key, edge: (BlockId, int4)| {
        match stmts.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => {
                if !v.contains(&edge) {
                    v.push(edge)
                }
            }
            None => stmts.push((key, vec![edge])),
        }
    };
    let mut landings: Vec<(u64, BlockId)> = Vec::new();
    let n = fd.bblocks_get_size();
    for bi in 0..n {
        let p = fd.bblocks_get_block(bi);
        let Some(tail) = fd.bb_op_tail(p).and_then(|t| fd.obank().get(t)) else { continue };
        let code = tail.code();
        let taken = match code {
            OpCode::CPUI_CBRANCH if fd.bblocks_ref().block(p).size_out() == 2 => {
                if tail.is_boolean_flip() {
                    0
                } else {
                    1
                }
            }
            OpCode::CPUI_BRANCH if fd.bblocks_ref().block(p).size_out() == 1 => 0,
            _ => continue,
        };
        let Some(dest) = branch_target(fd, p) else { continue };
        let d = fd.bblocks_ref().block(p).get_out(taken);
        let Some(s) = start(fd, d) else { continue };
        if dest < s && s - dest <= MAX_GAP {
            let key = if jump_only(fd, d) {
                Key::Block(d)
            } else {
                landings.push((dest, d));
                Key::Landing(dest)
            };
            add(&mut stmts, key, (p, taken));
        }
        if code == OpCode::CPUI_CBRANCH {
            let m = fd.bblocks_ref().block(p).get_out(1 - taken);
            if dest == s && jump_only(fd, m) && fd.bblocks_ref().block(m).size_in() == 1 {
                let jmp = start(fd, m).unwrap_or(u64::MAX);
                if dest > jmp && dest - jmp <= MAX_JUMP {
                    add(&mut stmts, Key::Block(m), (p, 1 - taken));
                    forwarded_tests(fd, p, m, &mut stmts);
                }
            }
        }
    }
    // A fall-through runs the `nop`s between its block and the next one, so it
    // shares the first landing in that gap (a closing brace both reach).
    for bi in 0..n {
        let p = fd.bblocks_get_block(bi);
        let Some(tail) = fd.bb_op_tail(p).and_then(|t| fd.obank().get(t)) else { continue };
        let b = fd.bblocks_ref().block(p);
        let slot = match tail.code() {
            OpCode::CPUI_CBRANCH if b.size_out() == 2 => {
                if tail.is_boolean_flip() {
                    1
                } else {
                    0
                }
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => continue,
            _ if b.size_out() == 1 => 0,
            _ => continue,
        };
        let d = b.get_out(slot);
        let last = tail.get_addr().get_offset();
        let Some(s) = start(fd, d) else { continue };
        let hit = landings.iter().filter(|&&(a, t)| t == d && a > last && a < s).map(|&(a, _)| a).min();
        if let Some(a) = hit {
            add(&mut stmts, Key::Landing(a), (p, slot));
        }
    }
    // Every way into a jump-only statement block belongs to that statement.
    for (key, edges) in stmts.iter_mut() {
        if let Key::Block(m) = *key {
            let b = fd.bblocks_ref().block(m);
            for k in 0..b.size_in() {
                let e = (b.get_in(k), b.get_in_rev_index(k));
                if !edges.contains(&e) {
                    edges.push(e);
                }
            }
        }
    }
    stmts.sort_by_key(|(k, _)| match *k {
        Key::Block(b) => start(fd, b).unwrap_or(0),
        Key::Landing(a) => a,
    });
    let mut count = 0;
    for (id, (_, edges)) in stmts.iter().enumerate() {
        let id = id as u32 + 1;
        if id >= MAX_ID {
            break;
        }
        for &(src, slot) in edges {
            fd.bblocks_mut().clear_out_edge_flag(src, slot, ID_MASK);
            fd.bblocks_mut().set_out_edge_flag(src, slot, id << ID_SHIFT);
        }
        count += 1;
    }
    fd.kuna_o0jumps = Some(Box::new(Record { statements: count }));
}

/// gcc sends the earlier tests of a `||` group straight to the target of a
/// jump-only statement block: a test that runs only into `p` and otherwise
/// jumps to that target belongs to the statement too.
fn forwarded_tests(fd: &Funcdata, p: BlockId, m: BlockId, stmts: &mut Vec<(Key, Vec<(BlockId, int4)>)>) {
    let g = fd.bblocks_ref();
    if g.block(m).size_out() != 1 {
        return;
    }
    let t = g.block(m).get_out(0);
    let mut c = p;
    for _ in 0..8 {
        if g.block(c).size_in() != 1 {
            return;
        }
        let q = g.block(c).get_in(0);
        let qb = g.block(q);
        if qb.size_out() != 2 || fd.bb_op_tail(q).and_then(|o| fd.obank().get(o)).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) {
            return;
        }
        let k = if qb.get_out(0) == c { 1 } else { 0 };
        if qb.get_out(1 - k) != c || qb.get_out(k) != t {
            return;
        }
        if let Some((_, v)) = stmts.iter_mut().find(|(key, _)| *key == Key::Block(m)) {
            v.push((q, k));
        }
        c = q;
    }
}

fn branch_target(fd: &Funcdata, b: BlockId) -> Option<u64> {
    let op = fd.obank().get(fd.bb_op_tail(b)?)?;
    let v = fd.vbank().get(op.get_in(0)?)?;
    (v.get_space().get_type() != spacetype::IPTR_CONSTANT).then(|| v.get_offset())
}

fn start(fd: &Funcdata, b: BlockId) -> Option<u64> {
    fd.bb_op_head(b).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset())
}

/// A block holding nothing but one unconditional jump.
fn jump_only(fd: &Funcdata, b: BlockId) -> bool {
    let ops = fd.bb_ops(b);
    ops.len() == 1
        && fd.bblocks_ref().block(b).size_out() == 1
        && fd.obank().get(ops[0]).is_some_and(|o| o.code() == OpCode::CPUI_BRANCH)
}

/// Print the `continue;` statements the record holds: in each loop whose
/// `continue;` resumes at `points[loop]`, an `if (c) { S }` that is the last
/// statement before that point, and whose test leaves for it only through
/// recorded jump statements, becomes `if (!c) continue; S`.  Control reaches
/// the same place either way, so only the spelling changes.
pub(crate) fn print_continues(data: &mut Funcdata, points: &std::collections::BTreeMap<BlockId, BlockId>) {
    if data.kuna_o0jumps.is_none() {
        return;
    }
    for (&lp, &point) in points {
        let slot = match data.sblocks_ref().block(lp).get_type() {
            BlockType::WhileDo => 1,
            BlockType::DoWhile => 0,
            _ => continue,
        };
        for _ in 0..64 {
            if !continue_once(data, lp, slot, point) {
                break;
            }
        }
    }
}

/// One rewrite at the end of loop `lp`'s body (its component `slot`).
fn continue_once(data: &mut Funcdata, lp: BlockId, slot: int4, point: BlockId) -> bool {
    let Some(body) = data.sblocks_ref().sub_block(lp, slot) else { return false };
    let flat = matches!(data.sblocks_ref().block(body).get_type(), BlockType::Ls | BlockType::Graph);
    let list: Vec<BlockId> = if flat { data.sblocks_ref().block(body).get_list().to_vec() } else { vec![body] };
    let Some(&last) = list.last() else { return false };
    let i = if last == point && list.len() >= 2 { list.len() - 2 } else { list.len() - 1 };
    let ifb = list[i];
    let g = data.sblocks_ref();
    if g.block(ifb).get_type() != BlockType::If || g.block(ifb).get_size() != 2 || g.block(ifb).get_if_goto_target().is_some() {
        return false;
    }
    let cond = g.block(ifb).get_block(0);
    let clause = g.block(ifb).get_block(1);
    let Some(pbb) = g.block(point).get_copy() else { return false };
    if !leaves_through_record(data, cond, pbb) || !crate::p8_structure::kuna_tailform::negatable(data, cond) {
        return false;
    }
    crate::p8_structure::kuna_tailform::negate(data, cond, true);
    let sg = data.sblocks_mut();
    let jump = sg.arena.insert(crate::block::FlowBlock::new_kind(crate::block::BlockKind::If {
        gototype: block_flags::f_continue_goto,
        gototarget: Some(point),
    }));
    sg.kuna_set_components(jump, vec![cond]);
    let clause_items = if sg.block(clause).get_type() == BlockType::Ls { sg.block(clause).get_list().to_vec() } else { vec![clause] };
    let mut items = list[..i].to_vec();
    items.push(jump);
    items.extend(clause_items.iter().copied());
    items.extend(list[i + 1..].iter().copied());
    if flat {
        sg.kuna_set_components(body, items);
    } else {
        let ls = sg.arena.insert(crate::block::FlowBlock::new_kind(crate::block::BlockKind::Ls));
        sg.kuna_set_components(ls, items);
        let mut comps = sg.block(lp).get_list().to_vec();
        comps[slot as usize] = ls;
        sg.kuna_set_components(lp, comps);
    }
    if let Some(&first) = clause_items.first().filter(|&&c| c != clause) {
        sg.kuna_free_replaced(clause, first);
    }
    sg.kuna_free_replaced(ifb, jump);
    true
}

/// Whether every way the condition `cond` reaches basic block `pbb` is a
/// recorded jump statement, and there is one.
fn leaves_through_record(data: &Funcdata, cond: BlockId, pbb: BlockId) -> bool {
    let sg = data.sblocks_ref();
    let mut found = false;
    let mut stack = vec![cond];
    while let Some(b) = stack.pop() {
        if sg.block(b).get_type() == BlockType::Copy {
            let Some(bb) = sg.block(b).get_copy() else { return false };
            let blk = data.bblocks_ref().block(bb);
            for e in 0..blk.size_out() {
                if blk.get_out(e) == pbb {
                    if id_of(blk.get_out_label(e)) == 0 {
                        return false;
                    }
                    found = true;
                }
            }
        } else {
            for k in 0..sg.block(b).get_size() {
                stack.push(sg.block(b).get_block(k));
            }
        }
    }
    found
}

/// Whether `ActionReturnSplit` leaves in-edge `i` of RETURN block `b` to
/// the structured goto `goto_block`: in a function whose returns
/// `returndup auto` placed, an edge recorded as a jump statement that leaves
/// its innermost loop for the block after that loop is the source's `break`
/// out of the loop, not a second `return`.
pub(crate) fn keeps_return(fd: &Funcdata, b: BlockId, i: int4, goto_block: BlockId) -> bool {
    if fd.kuna_o0jumps.is_none() || !crate::p8_structure::kuna_o0returns::decided(fd) {
        return false;
    }
    if id_of(fd.bblocks_ref().block(b).get_in_label(i)) == 0 {
        return false;
    }
    let Some(leaf) = fd.bblocks_ref().block(b).get_copy_map() else { return false };
    let sg = fd.sblocks_ref();
    let mut cur = goto_block;
    while let Some(up) = sg.block(cur).get_parent() {
        match sg.block(up).get_type() {
            BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => {
                let Some(outer) = sg.block(up).get_parent() else { return false };
                return sg.next_flow_after(outer, up) == Some(leaf);
            }
            BlockType::Switch => return false,
            _ => cur = up,
        }
    }
    false
}

thread_local! {
    /// Whether [`may_join`] keeps recorded statements apart, and how many folds
    /// it refused since [`set_gate`].
    static GATE: std::cell::Cell<(bool, u32)> = const { std::cell::Cell::new((true, 0)) };
}

/// Turn the fold gate on or off for the next structuring and reset its count.
pub(crate) fn set_gate(on: bool) {
    GATE.with(|g| g.set((on, 0)));
}

/// Whether the gate refused a fold since [`set_gate`].
pub(crate) fn refused() -> bool {
    GATE.with(|g| g.get().1 > 0)
}

/// The out-edge order of every two-way block, to put the graph back after a
/// structuring that negated conditions.
pub(crate) fn orientation(data: &Funcdata) -> Vec<(BlockId, BlockId)> {
    (0..data.bblocks_get_size())
        .map(|i| data.bblocks_get_block(i))
        .filter(|&b| data.bblocks_ref().block(b).size_out() == 2)
        .map(|b| (b, data.bblocks_ref().block(b).get_out(0)))
        .collect()
}

/// Negate back every two-way block whose out-edge order differs from `saved`.
pub(crate) fn restore_orientation(data: &mut Funcdata, saved: &[(BlockId, BlockId)]) {
    for &(b, out0) in saved {
        let blk = data.bblocks_ref().block(b);
        if blk.size_out() == 2 && blk.get_out(0) != out0 && blk.get_out(1) == out0 {
            data.block_basic_negate_lastop(b);
        }
    }
}

/// The structured jumps that print as `goto`: every goto block, or `if` with
/// a goto target, except one that leaves its innermost loop for the block
/// after the loop (a `break`) or restarts an infinite loop (a `continue`).
pub(crate) fn gotos(data: &Funcdata) -> usize {
    let sg = data.sblocks_ref();
    let mut n = 0;
    let mut stack = vec![data.sblocks_root()];
    while let Some(b) = stack.pop() {
        for i in 0..sg.block(b).get_size() {
            stack.push(sg.block(b).get_block(i));
        }
        let target = match sg.block(b).get_type() {
            BlockType::Goto if sg.goto_prints(b) => sg.block(b).get_goto_target(),
            BlockType::If => sg.block(b).get_if_goto_target(),
            _ => None,
        };
        let Some(t) = target.and_then(|t| sg.get_front_leaf(t)) else { continue };
        if sg.block(b).get_goto_type() & (block_flags::f_break_goto | block_flags::f_continue_goto) != 0 {
            continue;
        }
        let mut cur = b;
        let mut structured = false;
        while let Some(up) = sg.block(cur).get_parent() {
            let ty = sg.block(up).get_type();
            if matches!(ty, BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop | BlockType::Switch) {
                let after = sg.block(up).get_parent().and_then(|o| sg.next_flow_after(o, up));
                structured = after == Some(t) || (ty == BlockType::InfLoop && sg.get_front_leaf(up) == Some(t));
                break;
            }
            cur = up;
        }
        if !structured {
            n += 1;
        }
    }
    n
}

/// The jump-statement id an edge label carries; 0 for none.
pub(crate) fn id_of(label: uint4) -> uint4 {
    (label & ID_MASK) >> ID_SHIFT
}

/// Whether a fold may join out-edge `ai` of `a` and out-edge `bj` of `b`,
/// which reach the same block, into one condition.  Two edges into a loop's
/// step or test (a latch: `continue;`) or out of a loop (`break;`) join only
/// when both carry the same recorded jump statement or neither carries one.
/// Edges into a loop head join as before: the loop rules need the head's
/// in-edges folded, and gcc sends all but the last test of a `||` group
/// straight to a `while` test.
pub(crate) fn may_join(graph: &BlockGraph, a: BlockId, ai: int4, b: BlockId, bj: int4) -> bool {
    if id_of(graph.block(a).get_out_label(ai)) == id_of(graph.block(b).get_out_label(bj)) || !GATE.with(|g| g.get().0) {
        return true;
    }
    let tb = graph.block(graph.block(a).get_out(ai));
    let loop_node = |x: &crate::block::FlowBlock| {
        (0..x.size_in()).any(|k| x.is_back_edge_in(k)) || (0..x.size_out()).any(|k| x.is_back_edge_out(k))
    };
    if (0..tb.size_in()).any(|k| tb.is_back_edge_in(k)) {
        return true;
    }
    let latch = (0..tb.size_out()).any(|k| tb.is_back_edge_out(k));
    let exit = (0..tb.size_in()).any(|k| loop_node(graph.block(tb.get_in(k))));
    if latch || exit {
        GATE.with(|g| g.set((true, g.get().1 + 1)));
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{edge_flags, BlockKind, FlowBlock};

    /// Two tests `a`, `b` that both reach `t`; `t` loops back to `head`.
    fn graph() -> (BlockGraph, [BlockId; 4]) {
        let mut g = BlockGraph::new();
        let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        let head = g.new_block(root);
        let a = g.new_block(root);
        let b = g.new_block(root);
        let t = g.new_block(root);
        g.add_edge(head, a);
        g.add_edge(a, b);
        g.add_edge(a, t);
        g.add_edge(b, t);
        g.add_edge(t, head);
        g.set_out_edge_flag(t, 0, edge_flags::f_back_edge);
        (g, [head, a, b, t])
    }

    #[test]
    fn tests_into_a_latch_join_only_through_one_statement() {
        let (mut g, [_, a, b, _]) = graph();
        set_gate(true);
        assert!(may_join(&g, a, 1, b, 0), "no statement on either edge");
        g.set_out_edge_flag(a, 1, 1 << ID_SHIFT);
        assert!(!may_join(&g, a, 1, b, 0), "a continue and a plain branch");
        assert!(refused());
        g.set_out_edge_flag(b, 0, 2 << ID_SHIFT);
        assert!(!may_join(&g, a, 1, b, 0), "two different continues");
        g.clear_out_edge_flag(b, 0, ID_MASK);
        g.set_out_edge_flag(b, 0, 1 << ID_SHIFT);
        assert!(may_join(&g, a, 1, b, 0), "one continue reached twice");
        g.clear_out_edge_flag(b, 0, ID_MASK);
        set_gate(false);
        assert!(may_join(&g, a, 1, b, 0), "gate off");
        assert!(!refused());
        set_gate(true);
    }

    #[test]
    fn tests_into_a_loop_head_join_as_before() {
        let (mut g, [head, a, b, t]) = graph();
        g.clear_out_edge_flag(t, 0, edge_flags::f_back_edge);
        g.add_edge(head, t);
        g.set_out_edge_flag(t, 0, edge_flags::f_back_edge);
        let tb = g.block(t);
        assert!((0..tb.size_in()).any(|k| tb.is_back_edge_in(k)) || (0..tb.size_out()).any(|k| tb.is_back_edge_out(k)));
        g.set_out_edge_flag(a, 1, 1 << ID_SHIFT);
        set_gate(true);
        assert!(!may_join(&g, a, 1, b, 0));
        let (mut g, [_, a, b, t]) = graph();
        let back = g.block(t).get_out(0);
        g.set_out_edge_flag(a, 1, 1 << ID_SHIFT);
        g.clear_out_edge_flag(t, 0, edge_flags::f_back_edge);
        g.add_edge(back, t);
        let n = g.block(back).size_out() - 1;
        g.set_out_edge_flag(back, n, edge_flags::f_back_edge);
        assert!(may_join(&g, a, 1, b, 0), "t is a loop head");
    }

    #[test]
    fn ids_live_in_their_own_label_bits() {
        let label = edge_flags::f_back_edge | edge_flags::f_goto_edge | (5 << ID_SHIFT) | (3 << 16);
        assert_eq!(id_of(label), 5);
        assert_eq!(id_of(edge_flags::f_immed_copy), 0);
        assert_eq!(ID_MASK & 0x3ff, 0);
        assert_eq!(ID_MASK & (0x7fff << 16), 0);
    }

    #[test]
    fn loopcontinue_auto_keeps_continue_recovery_and_adds_the_record() {
        let opt = crate::p8_structure::kuna_loopcontinue::OptionLoopContinue;
        assert!(matches!(opt.apply("auto"), Ok((true, true, _))));
        assert!(matches!(opt.apply("on"), Ok((true, false, _))));
        assert!(matches!(opt.apply("off"), Ok((false, false, _))));
        assert!(opt.apply("sometimes").is_err());
    }
}
