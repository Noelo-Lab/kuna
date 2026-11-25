//! (kuna `returndup auto`) Two return shapes of optimized functions that the
//! binary's return layout decides, not the author.
//!
//! **Adjacent guards.** `if (a) return K; if (b) return K;` and
//! `if (a || b) return K;` are the same program, and an optimizing compiler
//! emits the same code for both; whether structuring recovers one guard or two
//! depends on whether the paths share the return block.  [`ActionRetShape`]
//! folds every run of adjacent guards on the final tree whose arms print the
//! same bare return and whose later test prints nothing in front of it into one
//! guard joined by `||` (left-nested, as `rule_block_or` builds a chain).  Two
//! tests that both call something stay apart when the return is void or a
//! constant: sources check one call's result per guard.  A print-tree edit: the
//! condition leaves keep their own polarity and the folded guard's return leaf
//! leaves the tree.
//!
//! **Merged returns.** `if (c) v = f(); else v = g(); return v;` is how a
//! shared return block of a value the arms compute prints; authors return in
//! each arm.  [`arm_returns`] hands `returndup` the shared return blocks whose
//! value is a phi of their in-edges (perhaps cast or extended, with nothing
//! else but the return's guard copies) when every in-edge ends an arm, so the
//! split gives each arm its own `return`.  A block that only runs into the
//! return (the join a stripped stack-protector check leaves) is spliced into it
//! first.  An arm entered from several places qualifies only when its entries
//! are one `if` or one short-circuit chain; otherwise its own return would need
//! a `goto`.  A one-block `if` (`v = a; if (c) v = b; return v;`) qualifies
//! when `b` does not read `a` (not the update `if (c) v += k;`).
//!
//! Functions [`crate::p3_dataflow::kuna_unoptimized`] flags, and functions
//! that address their frame through a frame pointer, keep the binary's form;
//! `returndup record` (and `on`) keep it everywhere.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{BlockKind, BlockType, FlowBlock};
use crate::context::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_looptest::LoopTestMode;
use crate::p8_structure::kuna_tailform::{self as tf, RetKey};

/// Bound on the guards folded in one function.
const MAX_EDITS: usize = 64;

/// The two re-spellings of `returndup auto`, as the Architecture carries them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetShapePolicy {
    /// Fold adjacent guards and split merged returns in optimized functions.
    pub on: bool,
    /// The `looptest` mode, whose plan owns the guard `if`s it drops.
    pub loop_test: LoopTestMode,
}

impl RetShapePolicy {
    pub fn off() -> Self {
        RetShapePolicy { on: false, loop_test: LoopTestMode::Binary }
    }
}

/// The shapes apply in a function the detector does not call unoptimized and
/// that does not address its frame through a frame pointer (small `-O0`
/// functions the detector misses keep one).
fn applies(fd: &Funcdata) -> bool {
    fd.get_arch().ret_shape.on
        && !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd)
        && !crate::p3_dataflow::kuna_unoptimized::frame_pointer(fd)
}

pub struct ActionRetShape {
    base: ActionBase,
}

impl ActionRetShape {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRetShape { base: ActionBase::new(0, "retshape", g) })
    }
}

impl Action for ActionRetShape {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionRetShape { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if data.sblocks_get_size() == 0 || !applies(data) {
            return 0;
        }
        let loop_test = data.get_arch().ret_shape.loop_test;
        let edits = fold_guards(data, loop_test);
        self.base_mut().count += edits as int4;
        0
    }
}

/// What a bare return hands back, compared the way the printer spells it: a
/// variable by its high (every SSA piece of it prints the same name), an
/// expression only as the same varnode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Void,
    Const(int4, uintb),
    High(HighVariableId),
    Vn(VarnodeId),
}

fn guard_ret(data: &Funcdata, ifb: BlockId) -> Option<Key> {
    Some(match tf::guard_key(data, ifb)? {
        RetKey::Void => Key::Void,
        RetKey::Const(s, v) => Key::Const(s, v),
        RetKey::Vn(vn) => {
            let v = data.vbank().get(vn)?;
            match v.get_high() {
                Some(h) if !v.is_implied() => Key::High(h),
                _ => Key::Vn(vn),
            }
        }
    })
}

/// Whether condition `c` prints nothing in front of its test: its front leaf
/// holds only ops the printer folds into the test.
fn quiet(data: &Funcdata, c: BlockId) -> bool {
    let g = data.sblocks_ref();
    match g.block(c).get_type() {
        BlockType::Copy => {
            let Some(bb) = g.block(c).get_copy() else { return false };
            let Some(tail) = data.bb_op_tail(bb) else { return false };
            if data.obank().get(tail).is_none_or(|o| o.code() != OpCode::CPUI_CBRANCH) {
                return false;
            }
            let mut cur = data.bb_op_head(bb);
            while let Some(o) = cur {
                cur = data.bb_op_next(o);
                if o != tail && tf::op_prints(data, o) {
                    return false;
                }
            }
            true
        }
        BlockType::Condition => quiet(data, g.block(c).get_block(0)),
        _ => false,
    }
}

/// The statement lists that no enclosing list flattens: the function's top
/// list and every `Ls`/`Graph` whose parent is not one, outside condition
/// positions.
fn containers(data: &Funcdata) -> Vec<BlockId> {
    let mut out = Vec::new();
    let Some(top) = tf::top_container(data) else { return out };
    let g = data.sblocks_ref();
    let mut stack = vec![(top, true)];
    while let Some((b, open)) = stack.pop() {
        let blk = g.block(b);
        let ty = blk.get_type();
        let list = matches!(ty, BlockType::Ls | BlockType::Graph);
        if list && open {
            out.push(b);
        }
        if ty == BlockType::Condition {
            continue;
        }
        let head = matches!(ty, BlockType::If | BlockType::WhileDo | BlockType::Switch);
        for (k, &c) in blk.get_list().iter().enumerate() {
            stack.push((c, !list && !(head && k == 0)));
        }
    }
    out
}

/// `list[i]` and `list[i + 1]` are guards that return the same value, the
/// second one's test is all it prints, and not both tests call something
/// when the value is void or a constant.
fn foldable(data: &Funcdata, list: &[BlockId], i: usize, conds: &[(BlockId, BlockId)], claimed: &[BlockId]) -> bool {
    let (x, y) = (list[i], list[i + 1]);
    let Some(k) = guard_ret(data, x) else { return false };
    if guard_ret(data, y) != Some(k) || claimed.contains(&x) || claimed.contains(&y) {
        return false;
    }
    let (cx, cy) = (tf::cond_of(data, x, conds), tf::cond_of(data, y, conds));
    if matches!(k, Key::Void | Key::Const(..))
        && test_calls(data, last_leaf(data, cx))
        && test_calls(data, first_leaf(data, cy))
    {
        return false;
    }
    if tf::labelled(data, y) {
        return false;
    }
    let refs = tf::goto_refs(data);
    let ry = data.sblocks_ref().block(y).get_block(1);
    if refs.contains(&y) || refs.contains(&ry) {
        return false;
    }
    quiet(data, cy)
}

fn first_leaf(data: &Funcdata, c: BlockId) -> BlockId {
    let g = data.sblocks_ref();
    let mut c = c;
    while g.block(c).get_type() == BlockType::Condition {
        c = g.block(c).get_block(0);
    }
    c
}

fn last_leaf(data: &Funcdata, c: BlockId) -> BlockId {
    let g = data.sblocks_ref();
    let mut c = c;
    while g.block(c).get_type() == BlockType::Condition {
        c = g.block(c).get_block(1);
    }
    c
}

/// Whether the test of condition leaf `c` calls something (a call the printer
/// folds into the test, not one printed as a statement before it).
fn test_calls(data: &Funcdata, c: BlockId) -> bool {
    let Some(bb) = data.sblocks_ref().block(c).get_copy() else { return false };
    let mut cur = data.bb_op_head(bb);
    while let Some(o) = cur {
        cur = data.bb_op_next(o);
        let call = data.obank().get(o).is_some_and(|op| matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND));
        if call && !tf::op_prints(data, o) {
            return true;
        }
    }
    false
}

fn fold_guards(data: &mut Funcdata, loop_test: LoopTestMode) -> usize {
    let mut edits = 0;
    let mut claimed: Option<Vec<BlockId>> = None;
    for c in containers(data) {
        if !data.sblocks_ref().arena.contains_key(c) {
            continue;
        }
        let view = tf::build_view(data, c);
        let none: &[BlockId] = &[];
        if !(0..view.list.len().saturating_sub(1)).any(|i| foldable(data, &view.list, i, &view.conds, none)) {
            continue;
        }
        let claimed = claimed.get_or_insert_with(|| tf::claimed_guards(data, loop_test));
        if !(0..view.list.len().saturating_sub(1)).any(|i| foldable(data, &view.list, i, &view.conds, claimed)) {
            continue;
        }
        if !tf::materialize(data, c, view) {
            continue;
        }
        while edits < MAX_EDITS {
            let list = data.sblocks_ref().block(c).get_list().to_vec();
            let Some(i) = (0..list.len().saturating_sub(1)).find(|&i| foldable(data, &list, i, &[], claimed)) else {
                break;
            };
            fold(data, c, &list, i);
            edits += 1;
        }
    }
    edits
}

/// `if (a) return K; if (b) return K;` -> `if (a || b) return K;`.
fn fold(data: &mut Funcdata, c: BlockId, list: &[BlockId], i: usize) {
    let (x, y) = (list[i], list[i + 1]);
    let g = data.sblocks_mut();
    let (cx, rx) = (g.block(x).get_block(0), g.block(x).get_block(1));
    let (cy, ry) = (g.block(y).get_block(0), g.block(y).get_block(1));
    let cond = g.arena.insert(FlowBlock::new_kind(BlockKind::Condition { opc: OpCode::CPUI_BOOL_OR }));
    g.kuna_set_components(cond, vec![cx, cy]);
    g.kuna_set_components(x, vec![cond, rx]);
    let mut rest = list.to_vec();
    rest.remove(i + 1);
    g.kuna_set_components(c, rest);
    g.arena.remove(y);
    tf::free_subtree(data, ry);
}

/// Most ops a value tail may hold.
const MAX_TAIL_OPS: usize = 16;

/// The shared return blocks of an optimized function whose returned value the
/// arms merge (`if (c) v = f(); else v = g(); return v;`), for `returndup` to
/// give every arm its own return.  The block may compute the value from the
/// merge (a cast or an extension) but do nothing else; a block that only runs
/// into the return (the join a stripped stack-protector check leaves) is
/// spliced into it first.
pub(crate) fn arm_returns(fd: &mut Funcdata, max_inedges: int4) -> Vec<BlockId> {
    let mut out = Vec::new();
    if !applies(fd) {
        return out;
    }
    let rets: Vec<OpId> = fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
    let mut folded: Option<std::collections::BTreeMap<BlockId, int4>> = None;
    for r in rets {
        let Some(op) = fd.obank().get(r) else { continue };
        if op.is_dead() || op.get_halt_type() != 0 || op.num_input() < 2 {
            continue;
        }
        let Some(b) = op.get_parent() else { continue };
        let g = fd.bblocks_ref();
        let join = if g.block(b).size_in() == 1 && g.block(g.block(b).get_in(0)).size_out() == 1 {
            g.block(b).get_in(0)
        } else {
            b
        };
        let n = g.block(join).size_in();
        if n < 2 || n > max_inedges || !value_tail(fd, b) || !value_tail(fd, join) || !merged_value(fd, b, join) {
            continue;
        }
        let preds: Vec<BlockId> = (0..n).map(|i| g.block(join).get_in(i)).collect();
        if preds.iter().any(|&p| g.block(p).size_out() != 1) && !fresh_triangle(fd, b, join) {
            continue;
        }
        let open: Vec<BlockId> = preds
            .iter()
            .copied()
            .filter(|&p| g.block(p).size_out() == 1 && g.block(p).size_in() > 1 && !if_join(fd, p))
            .collect();
        if !open.is_empty() {
            let folded = folded.get_or_insert_with(|| crate::p8_structure::kuna_orchain::folded_in_degree(fd));
            if open.iter().any(|p| folded.get(p).copied() != Some(1)) {
                continue;
            }
        }
        if join != b && fd.splice_block_basic(join).is_err() {
            continue;
        }
        if fd.get_arch().returndup_triangle && crate::p8_structure::kuna_rettriangle::is_triangle_join(fd, join) {
            continue;
        }
        out.push(join);
    }
    out
}

/// The two in-edges of `join` are a one-block `if` (a two-way block and the
/// one block it branches to), and the value the block computes does not read
/// the value the branch carries straight to the join: `v = a; if (c) v = b;`
/// rather than the update `if (c) v += k;`.
fn fresh_triangle(fd: &Funcdata, b: BlockId, join: BlockId) -> bool {
    let g = fd.bblocks_ref();
    let blk = g.block(join);
    if blk.size_in() != 2 {
        return false;
    }
    let (p0, p1) = (blk.get_in(0), blk.get_in(1));
    let arm_slot = if g.block(p0).size_out() == 2 && g.block(p1).size_out() == 1 && g.block(p1).size_in() == 1 && g.block(p1).get_in(0) == p0 {
        1
    } else if g.block(p1).size_out() == 2 && g.block(p0).size_out() == 1 && g.block(p0).size_in() == 1 && g.block(p0).get_in(0) == p1 {
        0
    } else {
        return false;
    };
    if g.block(if arm_slot == 1 { p0 } else { p1 }).is_switch_out() {
        return false;
    }
    let Some(phi) = phi_of(fd, b, join).and_then(|o| fd.obank().get(o)) else { return false };
    let (Some(arm), Some(carried)) = (phi.get_in(arm_slot), phi.get_in(1 - arm_slot)) else { return false };
    !reads(fd, arm, carried, 6)
}

/// The phi `merged_value` reaches.
fn phi_of(fd: &Funcdata, b: BlockId, join: BlockId) -> Option<OpId> {
    let ret = fd.obank().get(fd.bb_op_tail(b)?)?;
    let mut v = ret.get_in(1);
    for _ in 0..8 {
        let d = fd.vbank().get(v?)?.get_def()?;
        let def = fd.obank().get(d)?;
        match def.code() {
            OpCode::CPUI_MULTIEQUAL if def.get_parent() == Some(join) => return Some(d),
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_SUBPIECE => {
                v = def.get_in(0)
            }
            _ => return None,
        }
    }
    None
}

/// Whether the expression computing `v` reads `target`, or the value `target`
/// copies.
fn reads(fd: &Funcdata, v: VarnodeId, target: VarnodeId, depth: u32) -> bool {
    if v == target || copied(fd, target) == Some(v) {
        return true;
    }
    if depth == 0 {
        return false;
    }
    let Some(def) = fd.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| fd.obank().get(d)) else { return false };
    if def.code() == OpCode::CPUI_MULTIEQUAL {
        return false;
    }
    (0..def.num_input()).filter_map(|i| def.get_in(i)).any(|i| reads(fd, i, target, depth - 1))
}

/// The value `v` copies, when a `COPY` defines it.
fn copied(fd: &Funcdata, v: VarnodeId) -> Option<VarnodeId> {
    let def = fd.obank().get(fd.vbank().get(v)?.get_def()?)?;
    (def.code() == OpCode::CPUI_COPY).then(|| def.get_in(0)).flatten()
}

/// Block `p` joins the two paths of one `if`: a two-way block and the one
/// block it branches to, or two blocks entered only from the same two-way
/// block, each leaving only to `p`.
fn if_join(fd: &Funcdata, p: BlockId) -> bool {
    let g = fd.bblocks_ref();
    let blk = g.block(p);
    if blk.size_in() != 2 {
        return false;
    }
    let (a, b) = (blk.get_in(0), blk.get_in(1));
    let arm = |x: BlockId| g.block(x).size_in() == 1 && g.block(x).size_out() == 1;
    let head = |x: BlockId| g.block(x).size_out() == 2 && !g.block(x).is_switch_out();
    let triangle = |d: BlockId, x: BlockId| head(d) && arm(x) && g.block(x).get_in(0) == d;
    triangle(a, b) || triangle(b, a) || (arm(a) && arm(b) && g.block(a).get_in(0) == g.block(b).get_in(0) && head(g.block(a).get_in(0)))
}

/// Block `b` only computes a value: no memory access, call or branch, inputs
/// already defined, and no write to a global but the return's own
/// `glob = glob` guard copies.
fn value_tail(fd: &Funcdata, b: BlockId) -> bool {
    let ops = fd.bb_ops(b);
    ops.len() <= MAX_TAIL_OPS
        && ops.iter().all(|&o| {
            let Some(op) = fd.obank().get(o) else { return false };
            if matches!(
                op.code(),
                OpCode::CPUI_STORE
                    | OpCode::CPUI_LOAD
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_CALLOTHER
                    | OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_INDIRECT
                    | OpCode::CPUI_NEW
            ) {
                return false;
            }
            let settled = (0..op.num_input())
                .filter_map(|i| op.get_in(i))
                .all(|v| fd.vbank().get(v).is_some_and(|v| v.is_constant() || v.is_annotation() || !v.is_free()));
            let out = op.get_out().and_then(|v| fd.vbank().get(v));
            let local = out.is_none_or(|v| {
                !v.is_persist()
                    || (op.code() == OpCode::CPUI_COPY
                        && op.get_in(0).and_then(|i| fd.vbank().get(i)).is_some_and(|i| i.get_addr() == v.get_addr()))
            });
            settled && local
        })
}

/// The value `RETURN` hands back in block `b` is a merge of the in-edges of
/// `join` (`b` itself, or the block that only runs into it), perhaps cast or
/// extended.
fn merged_value(fd: &Funcdata, b: BlockId, join: BlockId) -> bool {
    let Some(ret) = fd.bb_op_tail(b).and_then(|t| fd.obank().get(t)) else { return false };
    if ret.code() != OpCode::CPUI_RETURN {
        return false;
    }
    let mut v = ret.get_in(1);
    for _ in 0..8 {
        let Some(def) = v.and_then(|v| fd.vbank().get(v)).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else {
            return false;
        };
        let at = def.get_parent();
        if at != Some(b) && at != Some(join) {
            return false;
        }
        match def.code() {
            OpCode::CPUI_MULTIEQUAL => return at == Some(join),
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_SUBPIECE => {
                v = def.get_in(0)
            }
            _ => return false,
        }
    }
    false
}
