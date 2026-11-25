//! (kuna `retshape`) Re-spell return shapes of optimized functions on the
//! final structured tree.
//!
//! **Guards.** `if (a) return K; if (b) return K;` and `if (a || b) return K;`
//! are the same program: the second test runs only when the first fails, and
//! both paths run the same `return K;`.  An optimizing compiler emits the same
//! code for either, and whether the structurer recovers one guard or two
//! depends on whether the binary shares the return block, which the splitters
//! (`returndup`, `earlyreturn`) and the compiler's own epilogue duplication
//! decide, not the author.  [`GuardForm::Or`] folds every run of adjacent
//! guards whose arms print the same bare return and whose later tests print
//! nothing before the test into one guard joined by `||`;
//! [`GuardForm::Split`] (for measuring) prints each `||` operand of a guard as
//! a guard of its own.  Print-tree edits only: the conditions keep their
//! leaves, a folded guard's return leaf leaves the tree, a split guard's second
//! return is a `BlockCopy` of the same block.
//!
//! Functions [`crate::p3_dataflow::kuna_unoptimized`] flags keep the binary's
//! form.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{BlockKind, BlockType, FlowBlock};
use crate::context::{BlockId, HighVariableId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_looptest::LoopTestMode;
use crate::p8_structure::kuna_tailform::{self as tf, RetKey};

/// Bound on the guards folded or split in one function.
const MAX_EDITS: usize = 64;

/// How adjacent guards with the same return print.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardForm {
    /// As structuring left them.
    Binary,
    /// Adjacent guards with the same bare return fold into one `||` guard.
    Or,
    /// [`GuardForm::Or`], except that two tests that both call something stay
    /// separate guards when the return is void or a constant.
    OrCalls,
    /// Each `||` operand of a bare-return guard becomes its own guard.
    Split,
}

/// Which merged returns of a value split into one return per arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmForm {
    /// None: the binary's shared return.
    Merged,
    /// Every predecessor of the return block ends an arm (has one out-edge),
    /// entered from one place or from one short-circuit chain.
    Arms,
    /// [`ArmForm::Arms`], and every arm is entered from one place.
    ArmsSingle,
    /// Every shared return of a merged value (for measuring).
    All,
}

/// The `retshape` option value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetShapePolicy {
    pub guards: GuardForm,
    /// Which shared returns of a value the arms merge print as one return per
    /// arm (`returndup` splits the return block).
    pub arms: ArmForm,
    /// `tailform`'s merge drops an arm's `return v;` that prints the same
    /// value as the function's final return, not only the same SSA value.
    pub tail_values: bool,
    /// `tailform`'s void merge drops an arm's last `return;` also when the arm
    /// holds a guard the `looptest` plan drops.
    pub tail_void: bool,
    /// The `looptest` mode, whose plan owns the guard `if`s it drops.
    pub loop_test: LoopTestMode,
}

impl RetShapePolicy {
    pub fn off() -> Self {
        RetShapePolicy {
            guards: GuardForm::Binary,
            arms: ArmForm::Merged,
            tail_values: false,
            tail_void: false,
            loop_test: LoopTestMode::Binary,
        }
    }

    /// The shipped default.
    pub fn shipped() -> Self {
        RetShapePolicy::off()
    }

    fn is_off(&self) -> bool {
        self.guards == GuardForm::Binary && self.arms == ArmForm::Merged
    }
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
        let policy = data.get_arch().ret_shape;
        if policy.is_off() || data.sblocks_get_size() == 0 {
            return 0;
        }
        if crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data) {
            return 0;
        }
        let edits = match policy.guards {
            GuardForm::Or | GuardForm::OrCalls => fold_guards(data, &policy),
            GuardForm::Split => split_guards(data, &policy),
            GuardForm::Binary => 0,
        };
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

/// `list[i]` and `list[i + 1]` are guards that return the same value, and the
/// second one's test is all it prints.
fn foldable(
    data: &Funcdata,
    policy: &RetShapePolicy,
    list: &[BlockId],
    i: usize,
    conds: &[(BlockId, BlockId)],
    claimed: &[BlockId],
) -> bool {
    let (x, y) = (list[i], list[i + 1]);
    let Some(k) = guard_ret(data, x) else { return false };
    if guard_ret(data, y) != Some(k) || claimed.contains(&x) || claimed.contains(&y) {
        return false;
    }
    let (cx, cy) = (tf::cond_of(data, x, conds), tf::cond_of(data, y, conds));
    if policy.guards == GuardForm::OrCalls
        && matches!(k, Key::Void | Key::Const(..))
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

fn fold_guards(data: &mut Funcdata, policy: &RetShapePolicy) -> usize {
    let mut edits = 0;
    let mut claimed: Option<Vec<BlockId>> = None;
    for c in containers(data) {
        if !data.sblocks_ref().arena.contains_key(c) {
            continue;
        }
        let view = tf::build_view(data, c);
        let none: &[BlockId] = &[];
        if !(0..view.list.len().saturating_sub(1)).any(|i| foldable(data, policy, &view.list, i, &view.conds, none)) {
            continue;
        }
        let claimed = claimed.get_or_insert_with(|| tf::claimed_guards(data, policy.loop_test));
        if !(0..view.list.len().saturating_sub(1)).any(|i| foldable(data, policy, &view.list, i, &view.conds, claimed)) {
            continue;
        }
        if !tf::materialize(data, c, view) {
            continue;
        }
        while edits < MAX_EDITS {
            let list = data.sblocks_ref().block(c).get_list().to_vec();
            let Some(i) = (0..list.len().saturating_sub(1)).find(|&i| foldable(data, policy, &list, i, &[], claimed)) else {
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

/// Whether condition `c` prints as `x || y`.
fn is_or(data: &Funcdata, c: BlockId) -> bool {
    let b = data.sblocks_ref().block(c);
    b.get_type() == BlockType::Condition && b.get_condition_opcode() != Some(OpCode::CPUI_BOOL_AND)
}

fn split_guards(data: &mut Funcdata, policy: &RetShapePolicy) -> usize {
    let mut claimed: Option<Vec<BlockId>> = None;
    let mut edits = 0;
    while edits < MAX_EDITS {
        let cands: Vec<BlockId> = data
            .sblocks_ref()
            .arena
            .iter()
            .filter(|(_, b)| b.get_type() == BlockType::If)
            .map(|(id, _)| id)
            .filter(|&ifb| {
                guard_ret(data, ifb).is_some() && {
                    let c = data.sblocks_ref().block(ifb).get_block(0);
                    is_or(data, c) && quiet(data, data.sblocks_ref().block(c).get_block(1))
                }
            })
            .collect();
        if cands.is_empty() {
            break;
        }
        let claimed = claimed.get_or_insert_with(|| tf::claimed_guards(data, policy.loop_test));
        let Some(&ifb) = cands.iter().find(|b| !claimed.contains(b) && data.sblocks_ref().block(**b).get_parent().is_some()) else {
            break;
        };
        split(data, ifb);
        edits += 1;
    }
    edits
}

/// `if (a || b) return K;` -> `if (a) return K; if (b) return K;`.
fn split(data: &mut Funcdata, ifb: BlockId) {
    let (cond, ret, parent) = {
        let b = data.sblocks_ref().block(ifb);
        (b.get_block(0), b.get_block(1), b.get_parent().expect("split: parent"))
    };
    let (ca, cb) = (data.sblocks_ref().block(cond).get_block(0), data.sblocks_ref().block(cond).get_block(1));
    let mut leaf = ret;
    while data.sblocks_ref().block(leaf).get_copy().is_none() {
        leaf = data.sblocks_ref().block(leaf).get_block(0);
    }
    let rbb = data.sblocks_ref().block(leaf).get_copy().expect("split: return leaf");
    let ret2 = tf::new_leaf(data, rbb, 0);
    let g = data.sblocks_mut();
    let second = g.arena.insert(FlowBlock::new_kind(BlockKind::If {
        gototype: crate::block::block_flags::f_goto_goto,
        gototarget: None,
    }));
    g.kuna_set_components(second, vec![cb, ret2]);
    g.kuna_set_components(ifb, vec![ca, ret]);
    g.arena.remove(cond);
    let mut plist = g.block(parent).get_list().to_vec();
    let at = plist.iter().position(|&p| p == ifb).expect("split: child");
    if matches!(g.block(parent).get_type(), BlockType::Ls | BlockType::Graph) {
        plist.insert(at + 1, second);
        g.kuna_set_components(parent, plist);
    } else {
        let ls = g.arena.insert(FlowBlock::new_kind(BlockKind::Ls));
        g.kuna_set_components(ls, vec![ifb, second]);
        plist[at] = ls;
        g.kuna_set_components(parent, plist);
    }
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
    let form = fd.get_arch().ret_shape.arms;
    if form == ArmForm::Merged || crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return out;
    }
    let rets: Vec<_> = fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
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
        if form != ArmForm::All && preds.iter().any(|&p| g.block(p).size_out() != 1) {
            continue;
        }
        let shared: Vec<BlockId> = preds.iter().copied().filter(|&p| g.block(p).size_in() > 1).collect();
        if form != ArmForm::All && !shared.is_empty() {
            if form == ArmForm::ArmsSingle {
                continue;
            }
            let folded = folded.get_or_insert_with(|| crate::p8_structure::kuna_orchain::folded_in_degree(fd));
            if shared.iter().any(|p| folded.get(p).copied() != Some(1)) {
                continue;
            }
        }
        if join != b && fd.splice_block_basic(join).is_err() {
            continue;
        }
        if fd.get_arch().returndup_triangle && crate::p8_structure::kuna_rettriangle::is_triangle_join(fd, join) {
            continue;
        }
        if std::env::var_os("KUNA_RETSHAPE_TRACE").is_some() {
            let g = fd.bblocks_ref();
            let preds: Vec<String> = (0..g.block(join).size_in())
                .map(|i| {
                    let p = g.block(join).get_in(i);
                    format!("b{}/in{}out{}", g.block(p).get_index(), g.block(p).size_in(), g.block(p).size_out())
                })
                .collect();
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(std::env::var_os("KUNA_RETSHAPE_TRACE").unwrap()) {
                let _ = writeln!(f, "TRACE retshape arms {} b{} spliced={} preds {:?}", fd.get_name(), g.block(join).get_index(), join != b, preds);
            }
        }
        out.push(join);
    }
    out
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

/// Whether `tailform`'s void merge may drop a `return;` past a looptest guard.
pub(crate) fn void_tails(fd: &Funcdata) -> bool {
    fd.get_arch().ret_shape.tail_void && !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd)
}

/// Whether two returned values of different SSA identity print the same value
/// at the end of a function: equal constants, the same named variable, or the
/// same side-effect-free expression over such leaves.  Only `tailform`'s merge
/// asks, for a `return` that ends an arm right before the final `return`, where
/// no statement runs between the two.
pub(crate) fn same_return(fd: &Funcdata, a: RetKey, b: RetKey) -> bool {
    if !fd.get_arch().ret_shape.tail_values || crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return false;
    }
    match (a, b) {
        (RetKey::Vn(x), RetKey::Vn(y)) => same_value(fd, x, y, 6),
        _ => false,
    }
}

fn same_value(fd: &Funcdata, x: VarnodeId, y: VarnodeId, depth: u32) -> bool {
    if x == y {
        return true;
    }
    let (Some(vx), Some(vy)) = (fd.vbank().get(x), fd.vbank().get(y)) else { return false };
    if vx.get_size() != vy.get_size() {
        return false;
    }
    if vx.is_constant() || vy.is_constant() || vx.is_annotation() || vy.is_annotation() {
        return vx.is_constant() == vy.is_constant()
            && vx.is_annotation() == vy.is_annotation()
            && vx.get_addr() == vy.get_addr();
    }
    if !vx.is_implied() && !vy.is_implied() {
        return vx.get_high().is_some() && vx.get_high() == vy.get_high();
    }
    if !(vx.is_implied() && vy.is_implied()) || depth == 0 {
        return false;
    }
    let (Some(dx), Some(dy)) = (
        vx.get_def().and_then(|d| fd.obank().get(d)),
        vy.get_def().and_then(|d| fd.obank().get(d)),
    ) else {
        return false;
    };
    if dx.code() != dy.code() || dx.num_input() != dy.num_input() {
        return false;
    }
    if matches!(
        dx.code(),
        OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_NEW
            | OpCode::CPUI_STORE
    ) {
        return false;
    }
    (0..dx.num_input()).all(|i| match (dx.get_in(i), dy.get_in(i)) {
        (Some(a), Some(b)) => same_value(fd, a, b, depth - 1),
        _ => false,
    })
}

/// The `retshape` option.
pub struct OptionRetShape;

impl OptionRetShape {
    pub const NAME: &'static str = "retshape";

    pub fn apply(&self, p1: &str) -> KunaResult<(RetShapePolicy, String)> {
        let mut policy = RetShapePolicy::off();
        for tok in p1.split(',').map(str::trim) {
            match tok {
                "off" => {}
                "or" => policy.guards = GuardForm::Or,
                "orcalls" => policy.guards = GuardForm::OrCalls,
                "split" => policy.guards = GuardForm::Split,
                "arms" => policy.arms = ArmForm::Arms,
                "arms1" => policy.arms = ArmForm::ArmsSingle,
                "armsall" => policy.arms = ArmForm::All,
                "tailvalues" => policy.tail_values = true,
                "tailvoid" => policy.tail_void = true,
                _ => {
                    return Err(KunaError::parse(
                        "Must specify retshape as 'off' or a list of 'or', 'orcalls', 'split', 'arms', 'arms1', 'armsall'",
                    ))
                }
            }
        }
        Ok((policy, format!("Return shape policy: guards {:?}, arms {:?}", policy.guards, policy.arms)))
    }
}
