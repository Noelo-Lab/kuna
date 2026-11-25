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
    /// Each `||` operand of a bare-return guard becomes its own guard.
    Split,
}

/// The `retshape` option value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetShapePolicy {
    pub guards: GuardForm,
    /// The `looptest` mode, whose plan owns the guard `if`s it drops.
    pub loop_test: LoopTestMode,
}

impl RetShapePolicy {
    pub fn off() -> Self {
        RetShapePolicy { guards: GuardForm::Binary, loop_test: LoopTestMode::Binary }
    }

    /// The shipped default.
    pub fn shipped() -> Self {
        RetShapePolicy::off()
    }

    fn is_off(&self) -> bool {
        self.guards == GuardForm::Binary
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
            GuardForm::Or => fold_guards(data, &policy),
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
fn foldable(data: &Funcdata, list: &[BlockId], i: usize, conds: &[(BlockId, BlockId)], claimed: &[BlockId]) -> bool {
    let (x, y) = (list[i], list[i + 1]);
    let Some(k) = guard_ret(data, x) else { return false };
    if guard_ret(data, y) != Some(k) || claimed.contains(&x) || claimed.contains(&y) {
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
    quiet(data, tf::cond_of(data, y, conds))
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
        if !(0..view.list.len().saturating_sub(1)).any(|i| foldable(data, &view.list, i, &view.conds, none)) {
            continue;
        }
        let claimed = claimed.get_or_insert_with(|| tf::claimed_guards(data, policy.loop_test));
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

/// The `retshape off|or|split` option.
pub struct OptionRetShape;

impl OptionRetShape {
    pub const NAME: &'static str = "retshape";

    pub fn apply(&self, p1: &str) -> KunaResult<(RetShapePolicy, String)> {
        let guards = match p1 {
            "off" => GuardForm::Binary,
            "or" => GuardForm::Or,
            "split" => GuardForm::Split,
            _ => return Err(KunaError::parse("Must specify retshape as 'off', 'or' or 'split'")),
        };
        Ok((RetShapePolicy { guards, ..RetShapePolicy::off() }, format!("Return shape policy: guards {guards:?}")))
    }
}
