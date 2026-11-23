//! The no-return form of a function tail: when the function's top-level
//! statements end in a call that does not return, `if (c) { X } T..; f();`,
//! where `X` leaves, becomes `if (!c) { T..; f(); } X`.
//!
//! Neither path reaches the end of the function, so the two spellings run the
//! same statements on the same condition: `X` still runs only when `c` holds,
//! because `T..; f();` never reaches it.  `X` is anything that leaves: a
//! `return E` of any value, statements and a return, or a jump.  The call is
//! one flow marks as not returning: the halt it plants right after the call
//! and the warning the printer shows as `// no-return`.  No size
//! threshold applies: authors write the fatal arm as the `if` body
//! (`if (n < 0) xalloc_die(); return n;`) whatever its length.

use super::*;

/// Rewrite the no-return sites of the function's top level: the top
/// container, then each list nested in it with no statement in between (one
/// the flattened top level cannot absorb because a goto names it).
pub(super) fn run(data: &mut Funcdata, cx: &mut Ctx) -> usize {
    let mut edits = 0;
    let mut computed = false;
    for c in top_lists(data, cx.top) {
        if !data.sblocks_ref().arena.contains_key(c) {
            continue;
        }
        let view = build_view(data, c);
        if site(data, cx, &view.list, &view.conds).is_none() {
            continue;
        }
        if !computed {
            cx.claimed = claimed_guards(data, cx.policy.loop_test);
            computed = true;
        }
        if site(data, cx, &view.list, &view.conds).is_none() || !materialize(data, c, view) {
            continue;
        }
        let list = data.sblocks_ref().block(c).get_list().to_vec();
        if let Some(i) = site(data, cx, &list, &[]) {
            rewrite(data, cx, c, &list, i);
            edits += 1;
            cx.claimed = claimed_guards(data, cx.policy.loop_test);
        }
    }
    cx.claimed.clear();
    edits
}

/// `top` and the `Ls` blocks reachable from it through `Ls`/`Graph` blocks
/// only, outer first.
fn top_lists(data: &Funcdata, top: BlockId) -> Vec<BlockId> {
    let g = data.sblocks_ref();
    let mut out = vec![top];
    let mut walk = vec![top];
    let mut k = 0;
    while k < walk.len() {
        for &c in g.block(walk[k]).get_list() {
            match g.block(c).get_type() {
                BlockType::Ls => {
                    out.push(c);
                    walk.push(c);
                }
                BlockType::Graph => walk.push(c),
                _ => {}
            }
        }
        k += 1;
    }
    out
}

/// In `list`, ending in a call that does not return, the index of the last
/// one-armed `if` whose arm leaves; the statements after it become its arm.
fn site(data: &Funcdata, cx: &Ctx, list: &[BlockId], conds: &[(BlockId, BlockId)]) -> Option<usize> {
    let n = list.len();
    if n < 2 || !is_leaf(data, list[n - 1]) || !noreturn_end(data, list[n - 1]) {
        return None;
    }
    let g = data.sblocks_ref();
    for i in (0..n - 1).rev() {
        let b = list[i];
        if g.block(b).get_type() != BlockType::If {
            continue;
        }
        if !plain_if(data, b) {
            return None;
        }
        let clause = g.block(b).get_block(1);
        if falls_through(data, clause) {
            continue;
        }
        if noreturn_end(data, clause) || cx.claimed.contains(&b) {
            return None;
        }
        let refs = goto_refs(data);
        let pinned = |t: BlockId| labelled(data, t) || truncated(data, t) || holds_claimed(data, cx, t) || refs.contains(&t);
        if pinned(clause) || list[i + 1..].iter().any(|&t| pinned(t)) || !negatable(data, cond_of(data, b, conds)) {
            return None;
        }
        return Some(i);
    }
    None
}

/// `if (c) { X } T..` -> `if (!c) { T.. } X` in the list `c`.
fn rewrite(data: &mut Funcdata, cx: &mut Ctx, c: BlockId, list: &[BlockId], i: usize) {
    let g = list[i];
    let cond = data.sblocks_ref().block(g).get_block(0);
    let clause = data.sblocks_ref().block(g).get_block(1);
    negate(data, cond, true);
    let body = new_list(data, list[i + 1..].to_vec());
    data.sblocks_mut().kuna_set_components(g, vec![cond, body]);
    let mut items = list[..=i].to_vec();
    items.push(clause);
    data.sblocks_mut().kuna_set_components(c, items);
    cx.ours.push(g);
}

/// Whether basic block `bb` ends in the halt flow plants after a call to a
/// subroutine that does not return, with nothing printed between the two, and
/// the call carries the warning the printer shows as `// no-return` (a flow
/// cut at the next function's entry carries another one).
fn noreturn_block(data: &Funcdata, bb: BlockId) -> bool {
    use crate::op::pcodeop_flags as f;
    let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else {
        return false;
    };
    let halt = op.get_halt_type();
    if op.code() != OpCode::CPUI_RETURN || halt & f::noreturn == 0 {
        return false;
    }
    if halt & (f::halt | f::badinstruction | f::unimplemented | f::missing) != 0 {
        return false;
    }
    let mut prev = op.basic_neighbours().0;
    while let Some(p) = prev {
        let Some(o) = data.obank().get(p) else { return false };
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return data.kuna_noreturn_note(o.get_addr());
        }
        if op_prints(data, p) {
            return false;
        }
        prev = o.basic_neighbours().0;
    }
    false
}

/// Whether `b` ends, at its bottom, in a call that does not return.
pub(super) fn noreturn_end(data: &Funcdata, b: BlockId) -> bool {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    match blk.get_type() {
        BlockType::Ls | BlockType::Graph => blk.get_list().last().is_some_and(|&l| noreturn_end(data, l)),
        BlockType::Copy => blk.get_copy().is_some_and(|bb| noreturn_block(data, bb)),
        _ => false,
    }
}
