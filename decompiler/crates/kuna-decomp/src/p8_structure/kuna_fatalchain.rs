//! (kuna `condfold noreturn|auto`) A chain of tests that all branch to one call
//! that does not return prints as one `if` over the `||` chain with the call as
//! its body and the rest of the function after it:
//!
//! ```c
//! if ((r = put(b, x)) != 0 || (r = put(b, y)) != 0) fatal(r, "compose");
//! rest;
//! ```
//!
//! The function is structured inside the main loop, while each call block still
//! holds the call's stack bookkeeping (on x86 the push of the return address),
//! so `BlockBasic::isComplex` counts the call, a store and the branch and
//! `ruleBlockOr` declines every later test: the chain prints as a ladder of
//! nested `if`s with the call after all of them.  [`relax`] drops the later
//! tests of such a chain from the complex set the structurer reads.  Under
//! `auto`, a function the unoptimized detector does not flag also relaxes a
//! chain into any block only the chain enters (`if (a() || b()) { note(); return
//! -1; }`).  A test that then prints more statements before its comparison than
//! the `condstmts` cap allows is caught by that option's final check, which
//! analyzes the function again with the test complex.  A structuring that
//! relaxed and prints a goto is not kept, and a function whose fold into an
//! ordinary block still prints a goto after the later passes is decompiled
//! again without such folds ([`late_gotos`]).
//!
//! After the tree is final, [`ActionFatalChain`] turns `if (chain) { R } f();`
//! (where `R` does not fall through) into `if (!chain) f(); R`, at any depth.
//! Neither form reaches past `f()`, so the two run the same statements on the
//! same condition.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockType;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_tailform as tf;

/// Which chains of tests fold across a complex later test.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChainFold {
    /// None.
    #[default]
    Off,
    /// Chains into a call that does not return.
    NoReturn,
    /// Those, and in a function the detector does not call unoptimized, a
    /// chain into any block only it enters.
    Auto,
}

/// A function of at least this many blocks whose fold into an ordinary block
/// fell back for gotos stops relaxing for the rest of its analysis.
pub const GIVE_UP_BLOCKS: int4 = 64;

fn enabled(data: &Funcdata) -> bool {
    data.get_arch().cond_fold_chain != ChainFold::Off
}

/// Whether a chain may fold into a block that is not a no-return call.
fn any_target(data: &Funcdata) -> bool {
    data.get_arch().cond_fold_chain == ChainFold::Auto
        && !data.chainfold_withdrawn()
        && !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data)
}

/// Whether basic block `bb` ends in the halt flow plants after a call to a
/// subroutine that does not return, and that call carries the warning the
/// printer shows as `// no-return` (a flow cut at the next function's entry
/// carries another one).
fn noreturn_tail(data: &Funcdata, bb: BlockId) -> bool {
    use crate::op::pcodeop_flags as f;
    let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else {
        return false;
    };
    let halt = op.get_halt_type();
    if op.code() != OpCode::CPUI_RETURN
        || halt & f::noreturn == 0
        || halt & (f::halt | f::badinstruction | f::unimplemented | f::missing) != 0
    {
        return false;
    }
    let mut prev = op.basic_neighbours().0;
    while let Some(p) = prev {
        let Some(o) = data.obank().get(p) else { return false };
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return data.kuna_noreturn_note(o.get_addr());
        }
        prev = o.basic_neighbours().0;
    }
    false
}

/// The chains `C1 -> C2 -> .. -> Ck` (k >= 2) of two-way blocks whose other
/// out-edge each reaches the same block `t`, where every later `Ci` is entered
/// only from `C(i-1)` and `t` is entered only from the chain; `t` ends in a
/// call that does not return unless `any`.
fn chains(data: &Funcdata, any: bool) -> Vec<(BlockId, Vec<BlockId>)> {
    let bg = data.bblocks_ref();
    let mut out = Vec::new();
    for i in 0..data.bblocks_get_size() {
        let t = data.bblocks_get_block(i);
        let tb = bg.block(t);
        if tb.size_in() < 2 || (!any && (tb.size_out() != 0 || !noreturn_tail(data, t))) {
            continue;
        }
        let preds: Vec<BlockId> = (0..tb.size_in()).map(|k| tb.get_in(k)).collect();
        let other = |p: BlockId| -> Option<BlockId> {
            let pb = bg.block(p);
            if pb.size_out() != 2 || pb.is_switch_out() {
                return None;
            }
            match (pb.get_out(0) == t, pb.get_out(1) == t) {
                (true, false) => Some(pb.get_out(1)),
                (false, true) => Some(pb.get_out(0)),
                _ => None,
            }
        };
        if preds.iter().any(|&p| other(p).is_none()) {
            continue;
        }
        let distinct: BTreeSet<BlockId> = preds.iter().copied().collect();
        if distinct.len() != preds.len() {
            continue;
        }
        let link = |p: BlockId| -> Option<BlockId> {
            let q = other(p)?;
            (q != p && distinct.contains(&q) && bg.block(q).size_in() == 1).then_some(q)
        };
        let linked: BTreeSet<BlockId> = preds.iter().filter_map(|&p| link(p)).collect();
        let heads: Vec<BlockId> = preds.iter().copied().filter(|p| !linked.contains(p)).collect();
        if heads.len() != 1 {
            continue;
        }
        let mut ch = vec![heads[0]];
        while let Some(q) = link(ch[ch.len() - 1]) {
            if ch.contains(&q) {
                break;
            }
            ch.push(q);
        }
        if ch.len() == preds.len() {
            out.push((t, ch));
        }
    }
    out
}

fn start(data: &Funcdata, bb: BlockId) -> Address {
    data.bblocks_block_start(bb)
}

/// Whether `bb` prints no more statements before its test than the
/// `condstmts` cap allows and carries no comment the condition printer would
/// drop.
fn fits(data: &Funcdata, bb: BlockId) -> bool {
    let ops = data.bb_ops(bb);
    let cap = data.get_arch().cond_stmts;
    !crate::p8_structure::kuna_condfold::block_carries_comment(data, &ops)
        && crate::p8_structure::kuna_condfold::printed_shape(data, &ops).is_some_and(|s| cap < 0 || s.stmts - 1 <= cap)
}

/// Whether a structuring of `data` now would relax a block.
pub(crate) fn pending(data: &Funcdata) -> bool {
    enabled(data) && !data.fatalchain_held() && !chains(data, any_target(data)).is_empty()
}

/// Drop the later tests of every chain into a no-return call from `complex`,
/// except a block the `condstmts` seed keeps complex.  Before implied flags
/// exist the printed width is checked later by `condstmts`; after, here.
pub(crate) fn relax(data: &mut Funcdata, complex: &mut BTreeSet<BlockId>) {
    if !enabled(data) || data.fatalchain_held() {
        return;
    }
    let high = data.is_high_on();
    let mut hits: Vec<Address> = Vec::new();
    let mut ordinary = false;
    for (t, ch) in chains(data, any_target(data)) {
        let before = hits.len();
        for &c in &ch[1..] {
            if !complex.contains(&c) {
                continue;
            }
            let a = start(data, c);
            let ok = if high {
                fits(data, c)
            } else {
                !crate::p8_structure::kuna_condfold::block_carries_comment(data, &data.bb_ops(c))
            };
            if !ok || data.condstmts_seed().contains(&a) {
                continue;
            }
            complex.remove(&c);
            hits.push(a);
        }
        ordinary |= hits.len() > before && !noreturn_tail(data, t);
    }
    for a in hits {
        data.add_fatalchain_relaxed(a);
    }
    if ordinary {
        data.set_chainfold_any(true);
    }
}

/// Whether the structure folds a block relaxed since `before` into a condition.
pub(crate) fn folded_any(data: &Funcdata, before: &BTreeSet<Address>) -> bool {
    folded_leaves(data).into_iter().any(|bb| {
        let a = start(data, bb);
        !before.contains(&a) && data.fatalchain_relaxed().contains(&a)
    })
}

/// The basic blocks printed as the right operand of a `&&` / `||`.
fn folded_leaves(data: &Funcdata) -> Vec<BlockId> {
    let g = data.sblocks_ref();
    let mut out = Vec::new();
    let mut stack = vec![data.sblocks_root()];
    while let Some(b) = stack.pop() {
        let blk = g.block(b);
        if blk.get_type() == BlockType::Condition {
            let mut ops = vec![blk.get_block(1)];
            while let Some(o) = ops.pop() {
                match g.block(o).get_type() {
                    BlockType::Copy => out.extend(g.block(o).get_copy()),
                    BlockType::Condition => ops.extend([g.block(o).get_block(0), g.block(o).get_block(1)]),
                    _ => {}
                }
            }
        }
        for i in 0..blk.get_size() {
            stack.push(blk.get_block(i));
        }
    }
    out
}

/// The gotos a function prints whose kept structuring folded a chain into an
/// ordinary block, or `None` when it prints none or folded no such chain.
pub(crate) fn late_gotos(fd: &Funcdata) -> Option<int4> {
    if !fd.chainfold_any() {
        return None;
    }
    let n = crate::p2_lift::kuna_loweredswitchkeep::printed_gotos(fd);
    (n > 0).then_some(n)
}

/// The basic blocks a condition prints.
fn cond_leaves(data: &Funcdata, c: BlockId, out: &mut Vec<BlockId>) {
    let g = data.sblocks_ref();
    match g.block(c).get_type() {
        BlockType::Copy => out.extend(g.block(c).get_copy()),
        _ => {
            for i in 0..g.block(c).get_size() {
                cond_leaves(data, g.block(c).get_block(i), out);
            }
        }
    }
}

/// The leaves of the tree that print basic block `bb`.
fn copies(data: &Funcdata, bb: BlockId) -> Vec<BlockId> {
    let g = data.sblocks_ref();
    let mut out = Vec::new();
    let mut stack = vec![data.sblocks_root()];
    while let Some(b) = stack.pop() {
        let blk = g.block(b);
        if blk.get_type() == BlockType::Copy && blk.get_copy() == Some(bb) {
            out.push(b);
        }
        for i in 0..blk.get_size() {
            stack.push(blk.get_block(i));
        }
    }
    out
}

fn is_container(data: &Funcdata, b: BlockId) -> bool {
    matches!(data.sblocks_ref().block(b).get_type(), BlockType::Ls | BlockType::Graph)
}

/// `[.., if (chain) { R }, f()]` -> `[.., if (!chain) f(), R]` for each chain.
fn orient(data: &mut Funcdata) -> usize {
    let chains = chains(data, false);
    if chains.is_empty() {
        return 0;
    }
    let mut edits = 0;
    for (t, ch) in chains {
        let leaves = copies(data, t);
        let [f] = leaves[..] else { continue };
        let g = data.sblocks_ref();
        let Some(p) = g.block(f).get_parent() else { continue };
        if !is_container(data, p) || g.block(p).get_list().last() != Some(&f) {
            continue;
        }
        let list = g.block(p).get_list().to_vec();
        if list.len() < 2 {
            continue;
        }
        let mut host = p;
        let mut ifb = list[list.len() - 2];
        while is_container(data, ifb) {
            let Some(&last) = g.block(ifb).get_list().last() else { break };
            host = ifb;
            ifb = last;
        }
        if !tf::plain_if(data, ifb) {
            continue;
        }
        let cond = g.block(ifb).get_block(0);
        let clause = g.block(ifb).get_block(1);
        let mut tested = Vec::new();
        cond_leaves(data, cond, &mut tested);
        if !ch[1..].iter().any(|c| tested.contains(c)) {
            continue;
        }
        let refs = tf::goto_refs(data);
        if tf::falls_through(data, clause)
            || tf::labelled(data, clause)
            || tf::labelled(data, f)
            || refs.contains(&f)
            || refs.contains(&clause)
            || !tf::negatable(data, cond)
        {
            continue;
        }
        tf::negate(data, cond, true);
        let sg = data.sblocks_mut();
        sg.kuna_set_components(ifb, vec![cond, f]);
        let mut outer = list[..list.len() - 1].to_vec();
        if host == p {
            outer.push(clause);
            sg.kuna_set_components(p, outer);
        } else {
            sg.kuna_set_components(p, outer);
            let mut inner = sg.block(host).get_list().to_vec();
            inner.push(clause);
            sg.kuna_set_components(host, inner);
        }
        edits += 1;
    }
    edits
}

/// The print-tree pass: [`orient`] over the final tree.
pub struct ActionFatalChain {
    base: ActionBase,
}

impl ActionFatalChain {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionFatalChain { base: ActionBase::new(0, "fatalchain", g) })
    }
}

impl Action for ActionFatalChain {
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
        Some(Box::new(ActionFatalChain { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !enabled(data) || data.sblocks_get_size() == 0 {
            return 0;
        }
        let edits = orient(data);
        self.base_mut().count += edits as int4;
        0
    }
}
