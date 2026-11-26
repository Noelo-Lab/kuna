//! (kuna `boolbranch return`) A function whose whole body is one test of a
//! short-circuit condition returning 1 or 0 prints one `return` of the
//! condition.
//!
//! `return p != NULL && p[0] == '/';` compiles to the same branches as
//! `if (p != NULL && p[0] == '/') return 1; return 0;`: an unoptimizing
//! compiler, and a branchy target at any level, materializes the value with a
//! jump to a `1` block and a `0` block, which structuring recovers as a guard
//! and a final return.  P3 `RuleConditionalMove` cannot fold such a select: the
//! `0` arm is entered from every short-circuit exit.  After the tree is final,
//! [`ActionBoolRet`] finds the function whose top-level statements are exactly
//! that guard and that return, the guard's condition a folded `&&`/`||` chain
//! whose leaves print nothing but their tests, and the two returns the
//! constants 1 and 0 of an integral return type.  The guard's `if` is flagged
//! [`F_BOOL_RET`] and the final return leaves the tree; the printer emits the
//! condition through the same renderer as the `if` header as the returned
//! value, behind a `!` when the guard returns 0 ([`F_BOOL_RET_NOT`]).  C's
//! `&&`, `||` and `!` yield exactly 0 or 1, so the value is the one the
//! constants gave, with the same operands evaluated in the same order and the
//! same short-circuit.  No p-code changes.
//!
//! A simple condition keeps its `if` (an unoptimizing compiler would have used
//! a flag-setting instruction for `return a == b;`), and so do a guard after
//! other statements, a chain of guards, and a condition that prints a
//! statement inside it (a comma expression).

use kuna_base::types::uint4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockType;
use crate::context::{BlockId, OpId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_tailform::{self as tf, RetKey};

/// Structured-block flag on the `if` that prints as `return <condition>;`.
pub const F_BOOL_RET: uint4 = 0x2000_0000;
/// With [`F_BOOL_RET`]: the guard returns 0, so the `if` prints
/// `return !<condition>;`.
pub const F_BOOL_RET_NOT: uint4 = 0x1000_0000;

/// How deep a condition chain the matcher walks.
const MAX_DEPTH: u32 = 16;

pub struct ActionBoolRet {
    base: ActionBase,
}

impl ActionBoolRet {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionBoolRet { base: ActionBase::new(0, "boolret", g) })
    }
}

impl Action for ActionBoolRet {
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
        Some(Box::new(ActionBoolRet { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().bool_ret || data.sblocks_get_size() == 0 {
            return 0;
        }
        if rewrite(data) {
            self.base_mut().count += 1;
        }
        0
    }
}

/// The guard `if` and the final return leaf of a function that is only
/// `if (C) return K; return !K;`.
struct Site {
    ifb: BlockId,
    ret: BlockId,
    /// The guard returns 0.
    negate: bool,
}

fn rewrite(data: &mut Funcdata) -> bool {
    let Some(top) = tf::top_container(data) else { return false };
    let view = tf::build_view(data, top);
    let Some(site) = find(data, &view.list, &view.conds) else { return false };
    if !tf::materialize(data, top, view) {
        return false;
    }
    let flags = if site.negate { F_BOOL_RET | F_BOOL_RET_NOT } else { F_BOOL_RET };
    data.sblocks_mut().block_mut(site.ifb).set_flag(flags);
    let rest: Vec<BlockId> = data.sblocks_ref().block(top).get_list().iter().copied().filter(|&b| b != site.ret).collect();
    data.sblocks_mut().kuna_set_components(top, rest);
    tf::free_subtree(data, site.ret);
    true
}

fn find(data: &Funcdata, list: &[BlockId], conds: &[(BlockId, BlockId)]) -> Option<Site> {
    if list.len() != 2 || !integral_return(data) {
        return None;
    }
    let (ifb, ret) = (list[0], list[1]);
    let RetKey::Const(_, k) = tf::guard_key(data, ifb)? else { return None };
    let RetKey::Const(_, last) = tf::bare_return(data, ret)? else { return None };
    if k > 1 || last != 1 - k {
        return None;
    }
    let refs = tf::goto_refs(data);
    let clause = data.sblocks_ref().block(ifb).get_block(1);
    if [ifb, clause, ret].iter().any(|b| refs.contains(b)) {
        return None;
    }
    let skipped = |b| data.sblocks_ref().block(leaf(data, b)).get_flags() & tf::F_RET_SKIP != 0;
    if skipped(clause) || skipped(ret) {
        return None;
    }
    let cond = tf::cond_of(data, ifb, conds);
    let chain = data.sblocks_ref().block(cond).get_type() == BlockType::Condition;
    if !chain || !quiet_chain(data, cond, &refs, 0) || !tf::negatable(data, cond) {
        return None;
    }
    Some(Site { ifb, ret, negate: k == 0 })
}

/// Whether every leaf of the condition chain `c` prints nothing but its test
/// and carries no label.
fn quiet_chain(data: &Funcdata, c: BlockId, refs: &[BlockId], depth: u32) -> bool {
    if depth > MAX_DEPTH {
        return false;
    }
    let g = data.sblocks_ref();
    match g.block(c).get_type() {
        BlockType::Condition => {
            g.block(c).get_size() == 2
                && quiet_chain(data, g.block(c).get_block(0), refs, depth + 1)
                && quiet_chain(data, g.block(c).get_block(1), refs, depth + 1)
        }
        BlockType::Copy => !refs.contains(&c) && !tf::labelled(data, c) && quiet_leaf(data, c),
        _ => false,
    }
}

fn quiet_leaf(data: &Funcdata, c: BlockId) -> bool {
    let Some(bb) = data.sblocks_ref().block(c).get_copy() else { return false };
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

/// The function returns an integer, a Boolean or an unknown scalar, the types
/// a Boolean expression stands in for 0 and 1 in.
fn integral_return(data: &Funcdata) -> bool {
    data.get_func_proto().get_output_type().is_some_and(|t| {
        matches!(
            t.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL | type_metatype::TYPE_UNKNOWN
        )
    })
}

/// For the printer: the `RETURN` whose statement the flagged `if` prints and
/// whether it returns the condition's negation, or `None` when `ifb` is not
/// flagged.
pub fn returned(data: &Funcdata, ifb: BlockId) -> Option<(OpId, bool)> {
    let g = data.sblocks_ref();
    let blk = g.block(ifb);
    if blk.get_flags() & F_BOOL_RET == 0 || blk.get_type() != BlockType::If || blk.get_size() != 2 {
        return None;
    }
    let tail = data.bb_op_tail(g.block(leaf(data, blk.get_block(1))).get_copy()?)?;
    (data.obank().get(tail)?.code() == OpCode::CPUI_RETURN).then_some((tail, blk.get_flags() & F_BOOL_RET_NOT != 0))
}

/// `b`, or the leaf a chain of one-element lists holds.
fn leaf(data: &Funcdata, b: BlockId) -> BlockId {
    let g = data.sblocks_ref();
    let mut b = b;
    while matches!(g.block(b).get_type(), BlockType::Ls | BlockType::Graph) && g.block(b).get_size() == 1 {
        b = g.block(b).get_block(0);
    }
    b
}
