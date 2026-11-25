//! (kuna `tailform`) Choose the form of a function-tail `if` by its
//! conditional part: saved logistic weights over its features by default
//! (`model`), its printed size under `on` / `<n>`.
//!
//! At the end of a function `if (c) { S } return K;` and `if (!c) return K; S;
//! return K;` are the same program, and the binary does not record which one
//! the author wrote: whether the compiler shared the epilogue or gave a path
//! its own `ret` (or a tail call) decides the form the structurer recovers.
//! Authors wrap a short tail and guard a long one, so after the tree is final
//! this pass re-spells the function's top-level tail (shown here with the size
//! rule; the model makes the same two rewrites):
//!
//! * **wrap** -- `if (c) return K; S1 .. Sk; return K;` with a tail of at most
//!   `max` lines becomes `if (!c) { S1 .. Sk } return K;`.  `K` is nothing
//!   (void), or the same value on both returns (equal constants, or one SSA
//!   varnode).
//! * **guard** -- `if (c) { S1 .. Sk } return K;` (the `if` last before the
//!   function's final bare return, `K` void or a constant) with a body of more
//!   than `max` lines becomes `if (!c) return K; S1 .. Sk; return K;`.
//! * **merge** -- in a valued function, an arm of that last `if` that ends in
//!   the `return K;` the function ends with drops it and falls through.
//! * **no-return** -- when the top-level statements end in a call that does
//!   not return, `if (c) { X } T..; f();` with `X` leaving becomes
//!   `if (!c) { T..; f(); } X`, for any `X` and any size (`noreturn.rs`);
//!   it runs first, and the size rule then leaves such an arm wrapped.
//!
//! All are print-tree edits on the top-level statement list: the guard's bare
//! return leaf is dropped (wrap) or a `BlockCopy` of the final return leaf
//! becomes the guard's arm (guard), the condition is negated in place, and the
//! statements move into or out of the `if` in order.  No op is cloned,
//! reordered or removed.  A leaf flagged [`F_RET_SKIP`] prints without its tail
//! `RETURN` (a return that now falls off the function, or one split away from
//! the statements of its block) and one flagged [`F_RET_ONLY`] prints only it.
//!
//! [`decide`] is the whole policy: it maps a [`TailSite`] to wrap / guard /
//! leave, so a different decision function can replace it.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockKind, BlockType, FlowBlock};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_looptest::{loop_tests, LoopTestMode};

pub use features::TailFeatures;

#[path = "kuna_tailform/features.rs"]
mod features;
#[path = "kuna_tailform/noreturn.rs"]
mod noreturn;

/// Leaf flag: print the leaf without its tail `RETURN`.
pub const F_RET_SKIP: uint4 = 0x4000_0000;
/// Leaf flag: print only the leaf's tail `RETURN`.
pub const F_RET_ONLY: uint4 = 0x8000_0000;

/// The `on` threshold: a conditional tail of at most this many printed lines
/// is wrapped, a longer one guarded.  It also decides under `model` where the
/// weights do not apply.
pub const DEFAULT_MAX_LINES: u32 = 4;

/// Bound on the edits made in one function (each edit moves one `if`).
const MAX_EDITS: usize = 64;

/// How [`decide`] chooses between the two forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailRule {
    /// Wrap a tail of at most the threshold's lines.
    Threshold,
    /// Wrap every tail.
    Wrap,
    /// Guard every tail.
    Guard,
    /// The saved logistic weights of [`crate::p8_structure::kuna_tailform_weights`].
    Model,
}

/// The `tailform` option value: the rule, and the wrap threshold for void and
/// for valued tails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TailFormPolicy {
    pub rule: TailRule,
    pub void_max: u32,
    pub value_max: u32,
    /// `voidtailreturn` is on: a void path's final bare `return;` is elided.
    pub elide_void: bool,
    /// The `looptest` mode, whose plan owns the guard `if`s it drops.
    pub loop_test: LoopTestMode,
}

impl TailFormPolicy {
    /// The shipped default, `tailform model`.
    pub fn shipped() -> Self {
        TailFormPolicy { rule: TailRule::Model, ..Default::default() }
    }
}

/// The `on` policy: the 4-line threshold.
impl Default for TailFormPolicy {
    fn default() -> Self {
        TailFormPolicy {
            rule: TailRule::Threshold,
            void_max: DEFAULT_MAX_LINES,
            value_max: DEFAULT_MAX_LINES,
            elide_void: true,
            loop_test: LoopTestMode::Binary,
        }
    }
}

/// The form a site is printed in now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailShape {
    /// `if (c) return K; S;`
    Guard,
    /// `if (c) { S } return K;`
    Wrapped,
}

/// One function-tail site, as [`decide`] sees it.
#[derive(Debug, Clone, PartialEq)]
pub struct TailSite {
    pub shape: TailShape,
    /// The function returns void.
    pub void: bool,
    /// Printed lines of the conditional part `S`.
    pub lines: usize,
    /// The site's features, computed only when the rule or the dump reads them.
    pub feats: Option<TailFeatures>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailDecision {
    Wrap,
    Guard,
    Leave,
}

/// Choose the form of `site`: under [`TailRule::Threshold`] wrap a tail of at
/// most the threshold's lines and guard a longer one; under [`TailRule::Model`]
/// wrap when the saved weights score the site's features above zero (and an
/// empty tail always, so every choice is one some threshold makes).  The model
/// was fitted on optimized code: in a function `kuna_unoptimized` flags, and at
/// a site without features, the threshold decides.
pub fn decide(site: &TailSite, policy: &TailFormPolicy) -> TailDecision {
    let max = if site.void { policy.void_max } else { policy.value_max } as usize;
    let wrap = match policy.rule {
        TailRule::Threshold => site.lines <= max,
        TailRule::Wrap => true,
        TailRule::Guard => false,
        TailRule::Model => match site.feats.as_ref() {
            Some(f) if !f.unopt => site.lines == 0 || features::model_score(f) > 0.0,
            _ => site.lines <= max,
        },
    };
    match (site.shape, wrap) {
        (TailShape::Guard, true) => TailDecision::Wrap,
        (TailShape::Wrapped, false) => TailDecision::Guard,
        _ => TailDecision::Leave,
    }
}

pub struct ActionTailForm {
    base: ActionBase,
}

impl ActionTailForm {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionTailForm { base: ActionBase::new(0, "tailform", g) })
    }
}

impl Action for ActionTailForm {
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
        Some(Box::new(ActionTailForm { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let Some(policy) = data.get_arch().tail_form else {
            return 0;
        };
        if data.sblocks_get_size() == 0 {
            return 0;
        }
        let edits = run(data, &policy);
        self.base_mut().count += edits as int4;
        0
    }
}

/// The value a bare `return` hands back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetKey {
    Void,
    Const(int4, uintb),
    Vn(VarnodeId),
}

/// How the function's last top-level statement returns.
#[derive(Debug, Clone, Copy)]
enum Final {
    /// A leaf that prints only `return K;`.
    Bare(BlockId, RetKey),
    /// A leaf that prints statements and then `return K;`.
    Embedded(BlockId, RetKey),
    /// Anything else (void functions only).
    Open,
}

#[derive(Debug, Clone, Copy)]
enum Edit {
    /// Wrap the tail after the guard at this index.
    Wrap(usize),
    /// Turn the wrapped `if` at this index into a guard.
    Guard(usize),
    /// Drop the returns of this value from the ends of the arms of the `if` at
    /// this index, which the final return repeats.
    Merge(usize, RetKey),
}

/// The function's top-level statements with every print-neutral container
/// (`Ls`/`Graph`) and every condition-list prefix of a top-level `if` hoisted
/// into one sequence.
#[derive(Default)]
pub(crate) struct View {
    pub(crate) list: Vec<BlockId>,
    pub(crate) conds: Vec<(BlockId, BlockId)>,
    pub(crate) dropped: Vec<BlockId>,
}

struct Ctx {
    void: bool,
    elide: bool,
    policy: TailFormPolicy,
    top: BlockId,
    ours: Vec<BlockId>,
    /// Guard `if`s the `looptest` plan drops in front of a loop.
    claimed: Vec<BlockId>,
    /// The model or the training dump reads site features.
    feats: bool,
    /// `claimed` is computed, so a decision is final enough to dump.
    checked: bool,
    dumped: std::cell::RefCell<Vec<(u64, TailShape)>>,
}

fn run(data: &mut Funcdata, policy: &TailFormPolicy) -> usize {
    let Some(top) = top_container(data) else {
        return 0;
    };
    let void = returns_void(data);
    let mut cx = Ctx {
        void,
        elide: void && policy.elide_void,
        policy: *policy,
        top,
        ours: Vec::new(),
        claimed: Vec::new(),
        feats: policy.rule == TailRule::Model || features::dump_dir().is_some(),
        checked: false,
        dumped: std::cell::RefCell::new(Vec::new()),
    };
    let rewritten = noreturn::run(data, &mut cx);
    let view = build_view(data, top);
    if features::dump_dir().is_some() {
        cx.claimed = claimed_guards(data, policy.loop_test);
        cx.checked = true;
    }
    if next_edit(data, &cx, &view.list, &view.conds).is_none() {
        return rewritten;
    }
    cx.claimed = claimed_guards(data, policy.loop_test);
    cx.checked = true;
    let mut edits = 0;
    let start = next_edit(data, &cx, &view.list, &view.conds).is_some() && materialize(data, top, view);
    while start && edits < MAX_EDITS {
        let list = data.sblocks_ref().block(top).get_list().to_vec();
        let Some(edit) = next_edit(data, &cx, &list, &[]) else {
            break;
        };
        let done = match edit {
            Edit::Wrap(i) => wrap(data, &mut cx, &list, i),
            Edit::Guard(i) => guard(data, &mut cx, &list, i),
            Edit::Merge(i, key) => {
                let clause = data.sblocks_ref().block(list[i]).get_block(1);
                mark_terminal_returns(data, clause, key, true);
                true
            }
        };
        if !done {
            break;
        }
        edits += 1;
        let view = build_view(data, top);
        if !materialize(data, top, view) {
            break;
        }
        cx.claimed = claimed_guards(data, policy.loop_test);
    }
    rewritten + edits
}

/// The guard `if`s the `looptest` plan of the current tree drops; this pass
/// leaves them, and everything around them, to that plan.  The printer's sign
/// plan does not exist yet, so counters keep their high's own declared type.
pub(crate) fn claimed_guards(data: &Funcdata, mode: LoopTestMode) -> Vec<BlockId> {
    let Some(root) = data.sblocks_ref().root else { return Vec::new() };
    if mode == LoopTestMode::Binary {
        return Vec::new();
    }
    loop_tests(data, root, mode, &|_| None).guards.keys().copied().collect()
}

/// Whether `b` is, or holds, a guard the `looptest` plan claims.
fn holds_claimed(data: &Funcdata, cx: &Ctx, b: BlockId) -> bool {
    cx.claimed.contains(&b) || data.sblocks_ref().block(b).get_list().iter().any(|&c| holds_claimed(data, cx, c))
}

pub(crate) fn top_container(data: &Funcdata) -> Option<BlockId> {
    let g = data.sblocks_ref();
    let root = g.root?;
    let list = g.block(root).get_list();
    if list.len() == 1 && matches!(g.block(list[0]).get_type(), BlockType::Ls | BlockType::Graph) {
        Some(list[0])
    } else {
        Some(root)
    }
}

fn returns_void(data: &Funcdata) -> bool {
    data.get_func_proto()
        .get_output_type()
        .map(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_VOID)
        .unwrap_or(false)
}

pub(crate) fn build_view(data: &Funcdata, top: BlockId) -> View {
    let mut v = View::default();
    for &c in data.sblocks_ref().block(top).get_list() {
        push_item(data, c, &mut v);
    }
    v
}

fn push_item(data: &Funcdata, c: BlockId, v: &mut View) {
    let g = data.sblocks_ref();
    match g.block(c).get_type() {
        BlockType::Ls | BlockType::Graph => {
            v.dropped.push(c);
            for &e in g.block(c).get_list() {
                push_item(data, e, v);
            }
        }
        BlockType::If => {
            let mut cond = g.block(c).get_block(0);
            let mut hoisted = false;
            while matches!(g.block(cond).get_type(), BlockType::Ls | BlockType::Graph)
                && g.block(cond).get_size() > 0
            {
                v.dropped.push(cond);
                let l = g.block(cond).get_list();
                for &p in &l[..l.len() - 1] {
                    push_item(data, p, v);
                }
                cond = l[l.len() - 1];
                hoisted = true;
            }
            if hoisted {
                v.conds.push((c, cond));
            }
            v.list.push(c);
        }
        _ => v.list.push(c),
    }
}

/// Apply a view: the top container holds the flat sequence, each hoisted `if`
/// keeps only its condition's last component, and the emptied containers are
/// freed.  Declines (changing nothing) when a container it would free carries a
/// label or is named by a goto.
pub(crate) fn materialize(data: &mut Funcdata, top: BlockId, view: View) -> bool {
    if view.dropped.is_empty() && view.conds.is_empty() {
        return true;
    }
    let refs = goto_refs(data);
    const HELD: uint4 = block_flags::f_goto_goto
        | block_flags::f_break_goto
        | block_flags::f_continue_goto
        | block_flags::f_unstructured_targ
        | block_flags::f_interior_gotoout
        | block_flags::f_interior_gotoin
        | block_flags::f_label_bumpup;
    for &d in &view.dropped {
        if refs.contains(&d) || data.sblocks_ref().block(d).get_flags() & HELD != 0 {
            return false;
        }
    }
    let g = data.sblocks_mut();
    for &(ifb, cond) in &view.conds {
        let mut l = g.block(ifb).get_list().to_vec();
        l[0] = cond;
        g.kuna_set_components(ifb, l);
    }
    g.kuna_set_components(top, view.list);
    for d in view.dropped {
        g.arena.remove(d);
    }
    true
}

pub(crate) fn goto_refs(data: &Funcdata) -> Vec<BlockId> {
    let mut out = Vec::new();
    for blk in data.sblocks_ref().arena.values() {
        match blk.kind() {
            BlockKind::Goto { gototarget: Some(t), .. } | BlockKind::If { gototarget: Some(t), .. } => out.push(*t),
            BlockKind::MultiGoto { gotoedges, .. } => out.extend(gotoedges.iter().copied()),
            _ => {}
        }
    }
    out
}

fn next_edit(data: &Funcdata, cx: &Ctx, list: &[BlockId], conds: &[(BlockId, BlockId)]) -> Option<Edit> {
    let n = list.len();
    if cx.elide && n > 0 && merges(data, cx, list[n - 1], RetKey::Void) {
        return Some(Edit::Merge(n - 1, RetKey::Void));
    }
    if n < 2 {
        return None;
    }
    let fin = final_return(data, list[n - 1], cx.void);
    let body_end = match fin {
        Final::Bare(..) => n - 1,
        _ => n,
    };
    if !cx.void && matches!(fin, Final::Open) {
        return None;
    }
    if let Final::Bare(r, key) = fin {
        let i = n - 2;
        // (kuna `returndup auto`) the recorded returns of `if (c) { S; return; }
        // return;` are two statements: print the bare one as the guard.
        if crate::p8_structure::kuna_o0returns::decided(data)
            && !labelled(data, r)
            && returning_if(data, cx, list[i], conds)
        {
            return Some(Edit::Guard(i));
        }
        if (!cx.void || cx.elide) && merges(data, cx, list[i], key) {
            return Some(Edit::Merge(i, key));
        }
        if cx.void || matches!(key, RetKey::Const(..)) {
            if let Some(lines) = wrapped_site(data, cx, list[i], r, conds) {
                let clause = data.sblocks_ref().block(list[i]).get_block(1);
                let hidden = if cx.elide { terminal_returns(data, clause, RetKey::Void, true) } else { 0 };
                let part = [clause];
                let at = SiteAt { list, at: i, part: &part, key, hidden, conds };
                if site_decision(data, cx, TailShape::Wrapped, lines, &at) == TailDecision::Guard {
                    return Some(Edit::Guard(i));
                }
            }
        }
    }
    let fin_key = match fin {
        Final::Bare(_, k) | Final::Embedded(_, k) => Some(k),
        Final::Open => None,
    };
    for i in (0..body_end.saturating_sub(1)).rev() {
        let Some(key) = guard_key(data, list[i]) else {
            continue;
        };
        if cx.ours.contains(&list[i]) {
            return None;
        }
        if !cx.void && Some(key) != fin_key {
            continue;
        }
        if cx.void && key != RetKey::Void {
            return None;
        }
        let tail = &list[i + 1..body_end];
        if cx.claimed.contains(&list[i]) || tail.iter().any(|&t| labelled(data, t) || truncated(data, t) || holds_claimed(data, cx, t)) {
            return None;
        }
        if !cx.void && tail.iter().any(|&t| has_return(data, t, matches!(fin, Final::Embedded(e, _) if e == t))) {
            return None;
        }
        let mut lines: usize = tail.iter().map(|&t| count_lines(data, t, conds)).sum();
        let hidden = if let Final::Embedded(..) = fin {
            1
        } else if cx.elide {
            terminal_returns(data, list[body_end - 1], RetKey::Void, false)
        } else {
            0
        };
        lines = lines.saturating_sub(hidden);
        if !negatable(data, cond_of(data, list[i], conds)) {
            return None;
        }
        let at = SiteAt { list, at: i, part: tail, key, hidden, conds };
        return match site_decision(data, cx, TailShape::Guard, lines, &at) {
            TailDecision::Wrap => Some(Edit::Wrap(i)),
            _ => None,
        };
    }
    None
}

/// Where a site sits: its `if` in the statement list, its conditional part,
/// the returned value, and the returns in the part that the rewrite hides.
struct SiteAt<'a> {
    list: &'a [BlockId],
    at: usize,
    part: &'a [BlockId],
    key: RetKey,
    hidden: usize,
    conds: &'a [(BlockId, BlockId)],
}

/// [`decide`] for one site, with its features when the model or the training
/// dump reads them.  A part that ends in a call that does not return is the
/// no-return form's, not a decision site: it gets no features.
fn site_decision(data: &Funcdata, cx: &Ctx, shape: TailShape, lines: usize, at: &SiteAt) -> TailDecision {
    // (kuna `returndup auto`) the recorded return statements placed this tail.
    if crate::p8_structure::kuna_o0returns::decided(data) {
        return TailDecision::Leave;
    }
    let mut site = TailSite { shape, void: cx.void, lines, feats: None };
    let decision_site = at.part.last().is_some_and(|&l| !noreturn::noreturn_end(data, l));
    if cx.feats && decision_site {
        let parts = features::SiteParts {
            list: at.list,
            at: at.at,
            part: at.part,
            shape,
            void: cx.void,
            key: Some(at.key),
            lines,
            hidden_rets: at.hidden as u32,
            conds: at.conds,
            unopt: crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data),
        };
        site.feats = Some(features::compute(data, &parts));
    }
    let decision = decide(&site, &cx.policy);
    if let (Some(dir), Some(f), true) = (features::dump_dir(), site.feats.as_ref(), cx.checked) {
        let addr = features::site_addr(data, at.list[at.at], at.conds);
        let mut dumped = cx.dumped.borrow_mut();
        if !dumped.contains(&(addr, shape)) {
            features::dump(data, dir, dumped.len() as u32, addr, &cx.policy, decision, f);
            dumped.push((addr, shape));
        }
    }
    decision
}

/// Whether `b` is a one-arm `if` whose arm ends in the `return key` the
/// function ends with anyway (and is more than that return).
fn merges(data: &Funcdata, cx: &Ctx, b: BlockId, key: RetKey) -> bool {
    if !plain_if(data, b) || holds_claimed(data, cx, b) {
        return false;
    }
    // (kuna `returndup auto`) a recorded valued return is not merged away.
    if key != RetKey::Void && crate::p8_structure::kuna_o0returns::decided(data) {
        return false;
    }
    let clause = data.sblocks_ref().block(b).get_block(1);
    bare_return(data, clause).is_none() && terminal_returns(data, clause, key, true) > 0
}

/// A one-arm `if` whose arm is more than a bare return and ends in one.
fn returning_if(data: &Funcdata, cx: &Ctx, ifb: BlockId, conds: &[(BlockId, BlockId)]) -> bool {
    if !plain_if(data, ifb) || cx.ours.contains(&ifb) || holds_claimed(data, cx, ifb) {
        return false;
    }
    let clause = data.sblocks_ref().block(ifb).get_block(1);
    bare_return(data, clause).is_none()
        && !falls_through(data, clause)
        && has_return(data, clause, false)
        && !truncated(data, clause)
        && !noreturn::noreturn_end(data, clause)
        && negatable(data, cond_of(data, ifb, conds))
}

/// For a wrapped `if (c) { S }` right before the final bare return `r`, the
/// printed lines of `S`.
fn wrapped_site(data: &Funcdata, cx: &Ctx, ifb: BlockId, r: BlockId, conds: &[(BlockId, BlockId)]) -> Option<usize> {
    if !plain_if(data, ifb) || cx.ours.contains(&ifb) || holds_claimed(data, cx, ifb) {
        return None;
    }
    let g = data.sblocks_ref();
    let clause = g.block(ifb).get_block(1);
    if bare_return(data, clause).is_some() || truncated(data, clause) || noreturn::noreturn_end(data, clause) {
        return None;
    }
    if !cx.void && !falls_through(data, clause) {
        return None;
    }
    if cx.elide && labelled(data, r) {
        return None;
    }
    if g.block(r).get_copy().is_none() || !negatable(data, cond_of(data, ifb, conds)) {
        return None;
    }
    let lines = count_lines(data, clause, conds);
    Some(if cx.elide { lines.saturating_sub(terminal_returns(data, clause, RetKey::Void, true)) } else { lines })
}

pub(crate) fn cond_of(data: &Funcdata, ifb: BlockId, conds: &[(BlockId, BlockId)]) -> BlockId {
    conds
        .iter()
        .find(|&&(i, _)| i == ifb)
        .map(|&(_, c)| c)
        .unwrap_or_else(|| data.sblocks_ref().block(ifb).get_block(0))
}

/// An `if` with one arm and no `goto`.
fn plain_if(data: &Funcdata, b: BlockId) -> bool {
    let b = data.sblocks_ref().block(b);
    b.get_type() == BlockType::If && b.get_size() == 2 && b.get_if_goto_target().is_none()
}

/// `Some(K)` when `ifb` prints `if (c) return K;`.
pub(crate) fn guard_key(data: &Funcdata, ifb: BlockId) -> Option<RetKey> {
    if !plain_if(data, ifb) {
        return None;
    }
    let clause = data.sblocks_ref().block(ifb).get_block(1);
    if labelled(data, clause) {
        return None;
    }
    bare_return(data, clause)
}

fn final_return(data: &Funcdata, last: BlockId, void: bool) -> Final {
    if let Some(k) = bare_return(data, last) {
        return Final::Bare(last, k);
    }
    if !void && is_leaf(data, last) && data.sblocks_ref().block(last).get_flags() & (F_RET_SKIP | F_RET_ONLY) == 0 {
        if let Some(k) = leaf_return(data, last) {
            return Final::Embedded(last, k);
        }
    }
    Final::Open
}

fn is_leaf(data: &Funcdata, b: BlockId) -> bool {
    data.sblocks_ref().block(b).get_type() == BlockType::Copy && data.sblocks_ref().block(b).get_copy().is_some()
}

/// The value of the `RETURN` ending leaf `b`, if it ends in one.
fn leaf_return(data: &Funcdata, b: BlockId) -> Option<RetKey> {
    let bb = data.sblocks_ref().block(b).get_copy()?;
    let tail = data.bb_op_tail(bb)?;
    let op = data.obank().get(tail)?;
    if op.code() != OpCode::CPUI_RETURN || op.get_halt_type() != 0 {
        return None;
    }
    match op.num_input() {
        1 => Some(RetKey::Void),
        2 => value_key(data, op.get_in(1)?),
        _ => None,
    }
}

fn value_key(data: &Funcdata, vn: VarnodeId) -> Option<RetKey> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return Some(RetKey::Const(v.get_size(), v.get_offset()));
    }
    if v.is_written() && v.is_implied() {
        let def = data.obank().get(v.get_def()?)?;
        if def.code() == OpCode::CPUI_COPY {
            let src = data.vbank().get(def.get_in(0)?)?;
            if src.is_constant() {
                return Some(RetKey::Const(v.get_size(), src.get_offset()));
            }
        }
    }
    Some(RetKey::Vn(vn))
}

/// `Some(K)` when `b` (a leaf, or a one-element list of one) prints only
/// `return K;` and carries no label.
pub(crate) fn bare_return(data: &Funcdata, b: BlockId) -> Option<RetKey> {
    let g = data.sblocks_ref();
    let mut b = b;
    while matches!(g.block(b).get_type(), BlockType::Ls | BlockType::Graph) && g.block(b).get_size() == 1 {
        b = g.block(b).get_block(0);
    }
    if !is_leaf(data, b) || labelled(data, b) {
        return None;
    }
    let key = leaf_return(data, b)?;
    if g.block(b).get_flags() & F_RET_ONLY != 0 {
        return Some(key);
    }
    let bb = g.block(b).get_copy()?;
    let tail = data.bb_op_tail(bb)?;
    let mut cur = data.bb_op_head(bb);
    while let Some(o) = cur {
        cur = data.bb_op_next(o);
        if o != tail && op_prints(data, o) {
            return None;
        }
    }
    Some(key)
}

/// How many printed `return key`s [`mark_terminal_returns`] would elide in `b`.
fn terminal_returns(data: &Funcdata, b: BlockId, key: RetKey, alone: bool) -> usize {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    match blk.get_type() {
        BlockType::Ls | BlockType::Graph => {
            let list = blk.get_list();
            list.last().map_or(0, |&l| terminal_returns(data, l, key, alone && list.len() == 1))
        }
        BlockType::If if blk.get_if_goto_target().is_none() => {
            (1..blk.get_size()).map(|k| terminal_returns(data, blk.get_block(k), key, true)).sum()
        }
        BlockType::Copy => usize::from(elidable_return(data, b, key, alone)),
        _ => 0,
    }
}

fn elidable_return(data: &Funcdata, b: BlockId, key: RetKey, alone: bool) -> bool {
    data.sblocks_ref().block(b).get_flags() & (F_RET_ONLY | F_RET_SKIP) == 0
        && !labelled(data, b)
        && leaf_return(data, b) == Some(key)
        && !(alone && bare_return(data, b).is_some())
}

/// Whether `b` contains a `RETURN` other than the tail of `b` itself when
/// `skip_own_tail`.
fn has_return(data: &Funcdata, b: BlockId, skip_own_tail: bool) -> bool {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    if let Some(bb) = blk.get_copy() {
        if blk.get_flags() & F_RET_SKIP != 0 || skip_own_tail {
            return false;
        }
        return data.bb_op_tail(bb).and_then(|t| data.obank().get(t)).is_some_and(|o| o.code() == OpCode::CPUI_RETURN);
    }
    blk.get_list().iter().any(|&c| has_return(data, c, false))
}

/// Whether `b` holds a `RETURN` that marks flow kuna could not follow, where
/// the function does not really end: a decode failure, a `halt`, or a no-return
/// halt with no call before it (flow cut at the next function's entry).
fn truncated(data: &Funcdata, b: BlockId) -> bool {
    use crate::op::pcodeop_flags as f;
    let blk = data.sblocks_ref().block(b);
    let Some(bb) = blk.get_copy() else {
        return blk.get_list().iter().any(|&c| truncated(data, c));
    };
    let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else {
        return false;
    };
    let halt = op.get_halt_type();
    if op.code() != OpCode::CPUI_RETURN || halt == 0 {
        return false;
    }
    if halt & (f::halt | f::badinstruction | f::unimplemented | f::missing) != 0 {
        return true;
    }
    let mut cur = data.bb_op_head(bb);
    while let Some(o) = cur {
        cur = data.bb_op_next(o);
        if data.obank().get(o).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)) {
            return false;
        }
    }
    true
}

pub(crate) fn labelled(data: &Funcdata, b: BlockId) -> bool {
    let g = data.sblocks_ref();
    if g.block(b).is_unstructured_target() {
        return true;
    }
    g.get_front_leaf(b).is_some_and(|f| g.block(f).is_unstructured_target() && !g.block(f).is_label_bump_up())
}

/// Whether control can leave `b` at its bottom (not a return, a no-return call
/// or a goto).
fn falls_through(data: &Funcdata, b: BlockId) -> bool {
    let g = data.sblocks_ref();
    match g.block(b).get_type() {
        BlockType::Ls | BlockType::Graph => g.block(b).get_list().last().is_none_or(|&l| falls_through(data, l)),
        BlockType::Goto | BlockType::MultiGoto => false,
        BlockType::Copy => {
            let Some(bb) = g.block(b).get_copy() else { return false };
            let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else { return true };
            match op.code() {
                OpCode::CPUI_RETURN => false,
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
                    data.bblocks_ref().block(bb).size_out() != 0
                }
                _ => true,
            }
        }
        _ => true,
    }
}

/// The printer's statement test for one op of a leaf (see
/// `PrintC::emit_basic_block_ops`), resolving the ambiguous cases toward
/// "prints".
pub(crate) fn op_prints(data: &Funcdata, o: OpId) -> bool {
    let Some(op) = data.obank().get(o) else {
        return false;
    };
    if op.is_branch() || crate::p9_emit::kuna_stackmarkers::nonprinting(data, o) {
        return false;
    }
    let implied = op.get_out().and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_implied());
    if !op.not_printed() && !implied {
        return true;
    }
    crate::kuna_stackbytes::assignment(data, o).is_some()
}

/// Printed lines of `b`: statements, `if`/loop/`switch`/`case` headers, labels
/// and gotos (not lone braces).
fn count_lines(data: &Funcdata, b: BlockId, conds: &[(BlockId, BlockId)]) -> usize {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    let label = usize::from(blk.is_unstructured_target() && !blk.is_label_bump_up());
    match blk.get_type() {
        BlockType::Copy | BlockType::Basic => {
            let fl = blk.get_flags();
            if fl & F_RET_ONLY != 0 {
                return 1;
            }
            let Some(bb) = blk.get_copy() else { return label };
            let skip = if fl & F_RET_SKIP != 0 { data.bb_op_tail(bb) } else { None };
            let mut n = label;
            let mut cur = data.bb_op_head(bb);
            while let Some(o) = cur {
                cur = data.bb_op_next(o);
                if Some(o) != skip && op_prints(data, o) {
                    n += 1;
                }
            }
            n
        }
        BlockType::If => {
            let cond = cond_of(data, b, conds);
            let mut n = count_lines(data, cond, conds) + 1;
            if blk.get_if_goto_target().is_none() {
                for i in 1..blk.get_size() {
                    n += count_lines(data, blk.get_block(i), conds);
                }
                if blk.get_size() == 3 {
                    n += 1;
                }
            }
            n
        }
        BlockType::Condition => count_lines(data, blk.get_block(0), conds),
        BlockType::WhileDo => 1 + (1..blk.get_size()).map(|i| count_lines(data, blk.get_block(i), conds)).sum::<usize>(),
        BlockType::DoWhile | BlockType::InfLoop => {
            2 + blk.get_list().iter().map(|&c| count_lines(data, c, conds)).sum::<usize>()
        }
        BlockType::Switch => {
            let n = blk.get_size();
            let head = if n > 0 { count_lines(data, blk.get_block(0), conds) } else { 0 };
            head + 1 + (1..n).map(|i| 1 + count_lines(data, blk.get_block(i), conds)).sum::<usize>()
        }
        BlockType::Goto => label + 1 + blk.get_list().iter().map(|&c| count_lines(data, c, conds)).sum::<usize>(),
        BlockType::MultiGoto => {
            let extra = match blk.kind() {
                BlockKind::MultiGoto { gotoedges, .. } => gotoedges.len(),
                _ => 0,
            };
            extra + blk.get_list().iter().map(|&c| count_lines(data, c, conds)).sum::<usize>()
        }
        _ => label + blk.get_list().iter().map(|&c| count_lines(data, c, conds)).sum::<usize>(),
    }
}

/// Whether the condition `c` can be negated in place: its leaves' basic blocks
/// end in a `CBRANCH` and no other leaf of the tree prints them.
pub(crate) fn negatable(data: &Funcdata, c: BlockId) -> bool {
    let g = data.sblocks_ref();
    match g.block(c).get_type() {
        BlockType::Copy => {
            let Some(bb) = g.block(c).get_copy() else { return false };
            let cbranch = data
                .bb_op_tail(bb)
                .and_then(|t| data.obank().get(t))
                .is_some_and(|o| o.code() == OpCode::CPUI_CBRANCH);
            cbranch && data.bblocks_ref().block(bb).size_out() == 2 && copies_of(data, bb) == 1
        }
        BlockType::Ls | BlockType::Graph => g.block(c).get_list().last().is_some_and(|&l| negatable(data, l)),
        BlockType::Condition => negatable(data, g.block(c).get_block(0)) && negatable(data, g.block(c).get_block(1)),
        _ => false,
    }
}

fn copies_of(data: &Funcdata, bb: BlockId) -> usize {
    data.sblocks_ref().arena.values().filter(|b| b.get_copy() == Some(bb)).count()
}

/// Negate the printed condition `c` (C++ `FlowBlock::negateCondition`):
/// flip each leaf's `CBRANCH` and swap its edges, De Morgan through a
/// `BlockCondition`.
pub(crate) fn negate(data: &mut Funcdata, c: BlockId, top: bool) {
    match data.sblocks_ref().block(c).get_type() {
        BlockType::Copy => {
            if let Some(bb) = data.sblocks_ref().block(c).get_copy() {
                data.block_basic_negate_lastop(bb);
            }
        }
        BlockType::Ls | BlockType::Graph => {
            if let Some(&l) = data.sblocks_ref().block(c).get_list().last() {
                negate(data, l, false);
            }
        }
        BlockType::Condition => {
            let b0 = data.sblocks_ref().block(c).get_block(0);
            let b1 = data.sblocks_ref().block(c).get_block(1);
            negate(data, b0, false);
            negate(data, b1, false);
            data.sblocks_mut().block_mut(c).flip_condition_opcode();
        }
        _ => {}
    }
    if top && data.sblocks_ref().block(c).size_out() == 2 {
        data.sblocks_mut().swap_edges(c);
    }
}

fn new_leaf(data: &mut Funcdata, bb: BlockId, flags: uint4) -> BlockId {
    let mut leaf = FlowBlock::new_kind(BlockKind::Copy { copy: Some(bb) });
    leaf.set_flag(flags);
    data.sblocks_mut().arena.insert(leaf)
}

fn new_list(data: &mut Funcdata, items: Vec<BlockId>) -> BlockId {
    if items.len() == 1 {
        return items[0];
    }
    let ls = data.sblocks_mut().arena.insert(FlowBlock::new_kind(BlockKind::Ls));
    data.sblocks_mut().kuna_set_components(ls, items);
    ls
}

/// `if (c) return K; S..; return K;` -> `if (!c) { S.. } return K;`.
fn wrap(data: &mut Funcdata, cx: &mut Ctx, list: &[BlockId], i: usize) -> bool {
    let n = list.len();
    let g = list[i];
    let cond = data.sblocks_ref().block(g).get_block(0);
    let guard_ret = data.sblocks_ref().block(g).get_block(1);
    let refs = goto_refs(data);
    if refs.contains(&guard_ret) || data.sblocks_ref().block(guard_ret).get_list().iter().any(|c| refs.contains(c)) {
        return false;
    }
    let (body, after): (Vec<BlockId>, Vec<BlockId>) = match final_return(data, list[n - 1], cx.void) {
        Final::Bare(r, _) => (list[i + 1..n - 1].to_vec(), vec![r]),
        Final::Embedded(e, _) => {
            let Some(bb) = data.sblocks_ref().block(e).get_copy() else { return false };
            data.sblocks_mut().block_mut(e).set_flag(F_RET_SKIP);
            let only = new_leaf(data, bb, F_RET_ONLY);
            (list[i + 1..n].to_vec(), vec![only])
        }
        Final::Open => (list[i + 1..n].to_vec(), Vec::new()),
    };
    if body.is_empty() {
        return false;
    }
    negate(data, cond, true);
    let clause = new_list(data, body);
    data.sblocks_mut().kuna_set_components(g, vec![cond, clause]);
    let mut top = list[..=i].to_vec();
    top.extend(after);
    data.sblocks_mut().kuna_set_components(cx.top, top);
    free_subtree(data, guard_ret);
    if cx.elide {
        mark_terminal_returns(data, clause, RetKey::Void, true);
    }
    cx.ours.push(g);
    true
}

/// `if (c) { S.. } return K;` -> `if (!c) return K; S..; return K;`.
fn guard(data: &mut Funcdata, cx: &mut Ctx, list: &[BlockId], i: usize) -> bool {
    let g = list[i];
    let r = list[i + 1];
    let Some(rbb) = data.sblocks_ref().block(r).get_copy() else { return false };
    let cond = data.sblocks_ref().block(g).get_block(0);
    let clause = data.sblocks_ref().block(g).get_block(1);
    negate(data, cond, true);
    let ret = new_leaf(data, rbb, 0);
    data.sblocks_mut().kuna_set_components(g, vec![cond, ret]);
    if cx.elide {
        data.sblocks_mut().block_mut(r).set_flag(F_RET_SKIP);
        mark_terminal_returns(data, clause, RetKey::Void, false);
    } else if crate::p8_structure::kuna_o0returns::decided(data) && !falls_through(data, clause) {
        data.sblocks_mut().block_mut(r).set_flag(F_RET_SKIP);
    }
    let mut top = list[..=i].to_vec();
    top.push(clause);
    top.push(r);
    data.sblocks_mut().kuna_set_components(cx.top, top);
    cx.ours.push(g);
    true
}

pub(crate) fn free_subtree(data: &mut Funcdata, b: BlockId) {
    let kids = data.sblocks_ref().block(b).get_list().to_vec();
    for k in kids {
        free_subtree(data, k);
    }
    data.sblocks_mut().arena.remove(b);
}

/// Mark the `return key` ending each path of `b`, where control would reach
/// the function's own final `return key` (or its end) next.  An `if` arm that
/// is nothing but that `return` keeps it.
fn mark_terminal_returns(data: &mut Funcdata, b: BlockId, key: RetKey, alone: bool) {
    let ty = data.sblocks_ref().block(b).get_type();
    match ty {
        BlockType::Ls | BlockType::Graph => {
            let list = data.sblocks_ref().block(b).get_list();
            if let Some(&l) = list.last() {
                let alone = alone && list.len() == 1;
                mark_terminal_returns(data, l, key, alone);
            }
        }
        BlockType::If => {
            let blk = data.sblocks_ref().block(b);
            if blk.get_if_goto_target().is_none() {
                let arms: Vec<BlockId> = (1..blk.get_size()).map(|k| blk.get_block(k)).collect();
                for a in arms {
                    mark_terminal_returns(data, a, key, true);
                }
            }
        }
        BlockType::Copy => {
            if elidable_return(data, b, key, alone) {
                data.sblocks_mut().block_mut(b).set_flag(F_RET_SKIP);
            }
        }
        _ => {}
    }
}

/// For the printer: the op a `tailform` leaf leaves out (its tail `RETURN`),
/// or `None`.
pub fn skipped_return(data: &Funcdata, leaf: BlockId) -> Option<OpId> {
    let blk = data.sblocks_ref().block(leaf);
    if blk.get_flags() & F_RET_SKIP == 0 {
        return None;
    }
    let tail = data.bb_op_tail(blk.get_copy()?)?;
    (data.obank().get(tail)?.code() == OpCode::CPUI_RETURN).then_some(tail)
}

/// For the printer: the only op a `tailform` return leaf prints, or `None`.
pub fn only_return(data: &Funcdata, leaf: BlockId) -> Option<OpId> {
    let blk = data.sblocks_ref().block(leaf);
    if blk.get_flags() & F_RET_ONLY == 0 {
        return None;
    }
    let tail = data.bb_op_tail(blk.get_copy()?)?;
    (data.obank().get(tail)?.code() == OpCode::CPUI_RETURN).then_some(tail)
}

/// The `tailform off|on|wrap|guard|model|<n>|<void>,<value>` option.
pub struct OptionTailForm;

impl OptionTailForm {
    pub const NAME: &'static str = "tailform";

    pub fn apply(&self, p1: &str) -> KunaResult<(Option<TailFormPolicy>, String)> {
        let bad = || KunaError::parse("Must specify tailform as 'off', 'on', 'wrap', 'guard', 'model', <n> or <void>,<value>");
        let rule = |rule| TailFormPolicy { rule, ..Default::default() };
        let policy = match p1 {
            "off" => return Ok((None, "Function-tail form policy turned off".to_string())),
            "on" => TailFormPolicy::default(),
            "wrap" => return Ok((Some(rule(TailRule::Wrap)), "Function-tail form policy: wrap every tail".to_string())),
            "guard" => return Ok((Some(rule(TailRule::Guard)), "Function-tail form policy: guard every tail".to_string())),
            "model" => {
                return Ok((Some(TailFormPolicy::shipped()), "Function-tail form policy: the saved logistic model".to_string()))
            }
            _ => match p1.split_once(',') {
                Some((v, k)) => TailFormPolicy {
                    void_max: v.trim().parse().map_err(|_| bad())?,
                    value_max: k.trim().parse().map_err(|_| bad())?,
                    ..Default::default()
                },
                None => {
                    let n: u32 = p1.trim().parse().map_err(|_| bad())?;
                    TailFormPolicy { void_max: n, value_max: n, ..Default::default() }
                }
            },
        };
        Ok((
            Some(policy),
            format!(
                "Function-tail form policy: wrap void tails of at most {} lines, valued tails of at most {}",
                policy.void_max, policy.value_max
            ),
        ))
    }
}

#[cfg(test)]
#[path = "kuna_tailform/tests.rs"]
mod tests;
