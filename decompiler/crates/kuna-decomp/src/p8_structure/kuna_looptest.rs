//! Where a bottom-tested loop prints its test (`option looptest binary|proven|assume`).
//!
//! GCC's loop header copying turns `while (c) body` into `if (c) { do body while (c); }`
//! and drops the `if` when it can prove `c` on entry.  kuna structures what the binary
//! holds, a `BlockDoWhile`.  This module plans, on the final structured tree, which of
//! those loops print as `while (c) { body }` and which guard `if` in front of such a loop
//! prints as nothing.  The plan is print-only: no p-code changes.
//!
//! A loop prints top-tested when the test at the top means the same thing as the test
//! at the bottom: every loop-variant leaf of the printed condition is the latch input of
//! a header MULTIEQUAL in the MULTIEQUAL's own HighVariable, and the condition reads no
//! call.  Its first evaluation must then agree with the binary.  `proven` accepts a loop
//! whose entry test `f_entry` (the condition with each latch input replaced by its entry
//! value) folds to true, or equals the condition of a guard that dominates the loop.  The
//! guard is dropped when nothing but pure statements sits between it and the loop, those
//! statements write only variables the skip path never reads, and the skip path reaches
//! the loop's own exit or an equal bare return.  `assume` also turns loops with no entry
//! proof.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::calc_mask;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::block::BlockType;
use crate::context::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::prettyprint::{Emit, MarkupRef, SyntaxHighlight};
use crate::printc::{tokens, PrintC};
use crate::printlanguage::OpToken;
use crate::printlanguage::modifiers;

/// The `looptest` option value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopTestMode {
    /// Print every loop where the binary tests it.
    Binary,
    /// Move the test to the top when the entry test is proven.
    Proven,
    /// Also move it when every loop-variant leaf has an entry value.
    Assume,
}

impl LoopTestMode {
    /// The option token.
    pub fn as_str(self) -> &'static str {
        match self {
            LoopTestMode::Binary => "binary",
            LoopTestMode::Proven => "proven",
            LoopTestMode::Assume => "assume",
        }
    }
}

/// Parse `binary|proven|assume`.
pub fn parse_loop_test(value: &str) -> KunaResult<(LoopTestMode, String)> {
    let mode = match value {
        "binary" => LoopTestMode::Binary,
        "proven" => LoopTestMode::Proven,
        "assume" => LoopTestMode::Assume,
        other => {
            return Err(KunaError::parse(format!(
                "Unknown looptest value: {other} (expected binary|proven|assume)"
            )))
        }
    };
    Ok((mode, format!("Loop test placement set to {value}")))
}

/// How a dropped guard `if` prints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Guard {
    /// `if (G) { pre; do … while (f); }`: print the condition's statements, then the body.
    Wrapped,
    /// `if (!G) return K;` ahead of `pre; do … while (f); return K;`: print only the
    /// condition's statements.
    Return,
    /// `if (G) { pre; do … while (f); return K; } return K;`: print the condition's
    /// statements and the body; the trailing return is hidden.
    Tail,
}

/// The print plan of one function.
#[derive(Default, Debug)]
pub struct LoopTests {
    /// `BlockDoWhile` nodes printed as `while (c) { body }`.
    pub top: BTreeSet<BlockId>,
    /// Guard `BlockIf` nodes printed without their test.
    pub guards: BTreeMap<BlockId, Guard>,
    /// Per loop, condition leaves that print as the header variable they are copied into.
    pub subst: BTreeMap<BlockId, Vec<(VarnodeId, VarnodeId)>>,
    /// The substitutions of the condition being printed.
    pub active: Vec<(VarnodeId, VarnodeId)>,
    /// Return leaves a `Tail` guard leaves unreachable.
    pub hidden: BTreeSet<BlockId>,
    /// A hidden return's RETURN op -> the RETURN that now ends the function.
    pub tail_returns: BTreeMap<OpId, OpId>,
    /// Per loop, the `==`/`!=` latch comparison that prints as an ordering test.
    pub respell: BTreeMap<BlockId, (OpId, &'static OpToken)>,
    /// The re-spelling of the condition being printed.
    pub active_respell: Option<(OpId, &'static OpToken)>,
}

impl LoopTests {
    /// The token a comparison op prints with, when the loop being printed re-spells it.
    pub fn token(&self, op: OpId) -> Option<&'static OpToken> {
        self.active_respell.filter(|&(o, _)| o == op).map(|(_, t)| t)
    }

    /// The varnode an explicit leaf prints as.
    pub fn leaf(&self, vn: VarnodeId) -> VarnodeId {
        self.active.iter().find(|(from, _)| *from == vn).map(|&(_, to)| to).unwrap_or(vn)
    }
}

/// The printer's declared type for a HighVariable, when it overrides the high's own.
pub type DeclType<'a> = &'a dyn Fn(HighVariableId) -> Option<std::rc::Rc<Datatype>>;

/// Plan every `BlockDoWhile` under `root`.
pub fn loop_tests(fd: &Funcdata, root: BlockId, mode: LoopTestMode, decl: DeclType) -> LoopTests {
    let mut plan = LoopTests::default();
    if mode == LoopTestMode::Binary {
        return plan;
    }
    let sb = fd.sblocks_ref();
    let mut leaves: BTreeMap<BlockId, BlockId> = BTreeMap::new();
    let mut loops = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = sb.block(id);
        match b.get_type() {
            BlockType::Copy => {
                if let Some(bb) = b.get_copy() {
                    leaves.insert(bb, id);
                }
            }
            BlockType::DoWhile => loops.push(id),
            _ => {}
        }
        stack.extend(b.get_list().iter().copied());
    }
    for dw in loops {
        if let Ok(top) = plan_loop(fd, &leaves, dw, mode, decl) {
            plan.top.insert(dw);
            if !top.subst.is_empty() {
                plan.subst.insert(dw, top.subst);
            }
            if let Some((g, f)) = top.guard {
                plan.guards.insert(g, f);
            }
            if let Some((leaf, from, to)) = top.hidden {
                plan.hidden.insert(leaf);
                plan.tail_returns.insert(from, to);
            }
            if let Some(r) = top.respell {
                plan.respell.insert(dw, r);
            }
        }
    }
    plan
}

struct Top {
    guard: Option<(BlockId, Guard)>,
    hidden: Option<(BlockId, OpId, OpId)>,
    subst: Vec<(VarnodeId, VarnodeId)>,
    respell: Option<(OpId, &'static OpToken)>,
}

type Verdict = Result<Top, &'static str>;

type Dropped = (BlockId, Guard, Option<(BlockId, OpId, OpId)>);

fn plan_loop(
    fd: &Funcdata,
    leaves: &BTreeMap<BlockId, BlockId>,
    dw: BlockId,
    mode: LoopTestMode,
    decl: DeclType,
) -> Verdict {
    let lp = Loop::find(fd, dw)?;
    let subst = lp.top_ok()?;
    let cont = lp.entry_cond(&BTreeSet::new());
    if cont.as_ref().is_some_and(|c| c.lit == Lit::True && !c.neg) {
        return Ok(Top { guard: None, hidden: None, subst, respell: None });
    }
    let guard = lp.guard(fd);
    let mut proven = guard.as_ref().is_some_and(|g| g.cond.is_some());
    let mut respell = None;
    let mut why = "guard differs";
    if !proven && lp.dominating_fact(cont.as_ref()) {
        return Ok(Top { guard: None, hidden: None, subst, respell: None });
    }
    if !proven {
        let fresh = guard.as_ref().map(|g| g.fresh.clone()).unwrap_or_default();
        match lp.respell(&fresh, decl) {
            Ok(r) => {
                if guard.as_ref().and_then(|g| g.enter.as_ref()) == Some(&r.entry) {
                    proven = true;
                    respell = Some((r.op, r.tok));
                } else if !r.entry.has_load() && lp.dominating_fact(Some(&r.entry)) {
                    return Ok(Top { guard: None, hidden: None, subst, respell: Some((r.op, r.tok)) });
                } else {
                    why = "respelled guard differs";
                }
            }
            Err(w) => why = w,
        }
    }
    if !proven {
        return match mode {
            LoopTestMode::Assume => Ok(Top { guard: None, hidden: None, subst, respell: None }),
            _ => Err(if guard.is_some() { why } else { "no guard" }),
        };
    }
    let g = guard.expect("proven implies a guard");
    match g.droppable(&lp, leaves) {
        Ok((node, form, hidden)) => Ok(Top { guard: Some((node, form)), hidden, subst, respell }),
        Err(_) if mode == LoopTestMode::Assume => Ok(Top { guard: None, hidden: None, subst, respell: None }),
        Err(why) => Err(why),
    }
}

/// A `BlockDoWhile` read back to its basic blocks.
struct Loop<'a> {
    fd: &'a Funcdata,
    dw: BlockId,
    body: BTreeSet<BlockId>,
    header: BlockId,
    cbranch: OpId,
    exit: BlockId,
    entry: BlockId,
    /// Latch input of each header MULTIEQUAL, and each loop value it copies -> its entry input.
    latch_inputs: BTreeMap<VarnodeId, VarnodeId>,
    /// The same values -> the MULTIEQUAL output they carry into the next iteration.
    carried: BTreeMap<VarnodeId, VarnodeId>,
    /// A loop value copied into a header MULTIEQUAL's input from another variable -> the MULTIEQUAL output.
    alias: BTreeMap<VarnodeId, VarnodeId>,
    /// The loop continues when the CBRANCH input xor this is true.
    cont_neg: bool,
}

impl<'a> Loop<'a> {
    fn find(fd: &'a Funcdata, dw: BlockId) -> Result<Loop<'a>, &'static str> {
        let sb = fd.sblocks_ref();
        let bodyn = sb.block(dw).get_block(0);
        let mut latch_leaf = bodyn;
        loop {
            let b = sb.block(latch_leaf);
            match b.get_type() {
                BlockType::Copy => break,
                BlockType::Ls if b.get_size() > 0 => latch_leaf = b.get_block(b.get_size() - 1),
                _ => return Err("latch not a leaf"),
            }
        }
        let head_leaf = sb.get_front_leaf(dw).ok_or("no head leaf")?;
        if sb.block(head_leaf).is_unstructured_target() || sb.block(dw).is_unstructured_target() {
            return Err("labelled head");
        }
        let header = sb.block(head_leaf).get_copy().ok_or("head not a copy")?;
        let latch = sb.block(latch_leaf).get_copy().ok_or("latch not a copy")?;
        let cbranch = fd.bb_op_tail(latch).ok_or("empty latch")?;
        if fd.obank().get(cbranch).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) {
            return Err("latch has no cbranch");
        }
        let mut body = BTreeSet::new();
        let mut stack = vec![dw];
        while let Some(id) = stack.pop() {
            let b = sb.block(id);
            if b.get_type() == BlockType::Copy {
                if let Some(bb) = b.get_copy() {
                    body.insert(bb);
                }
            }
            stack.extend(b.get_list().iter().copied());
        }
        let bb = fd.bblocks_ref();
        let lb = bb.block(latch);
        if lb.size_out() != 2 {
            return Err("latch out-degree");
        }
        let (back, exit) = (0..2).map(|i| lb.get_out(i)).partition::<Vec<_>, _>(|&t| t == header);
        if back.len() != 1 || exit.len() != 1 || body.contains(&exit[0]) {
            return Err("latch edges");
        }
        let exit = exit[0];
        let flip = fd.obank().get(cbranch).map(|o| o.is_boolean_flip()).unwrap_or(false);
        let cont_neg = if lb.get_out(1) == header { flip } else { !flip };
        let hb = bb.block(header);
        let mut entry_slot = None;
        let mut latch_slot = None;
        for i in 0..hb.size_in() {
            let p = hb.get_in(i);
            if body.contains(&p) {
                if p != latch || latch_slot.is_some() {
                    return Err("second back edge");
                }
                latch_slot = Some(i);
            } else {
                if entry_slot.is_some() {
                    return Err("second entry");
                }
                entry_slot = Some(i);
            }
        }
        let (Some(es), Some(ls)) = (entry_slot, latch_slot) else { return Err("header edges") };
        let entry = hb.get_in(es);
        let mut latch_inputs = BTreeMap::new();
        let mut carried = BTreeMap::new();
        let mut alias = BTreeMap::new();
        let mut op = fd.bb_op_head(header);
        while let Some(o) = op {
            op = fd.bb_op_next(o);
            let Some(p) = fd.obank().get(o) else { continue };
            if p.code() != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            let (Some(lv), Some(ev), Some(out)) = (p.get_in(ls), p.get_in(es), p.get_out()) else { continue };
            let same_high = fd.vbank().get(lv).and_then(|v| v.get_high()).is_some()
                && fd.vbank().get(lv).and_then(|v| v.get_high()) == fd.vbank().get(out).and_then(|v| v.get_high());
            if !same_high || latch_inputs.contains_key(&lv) {
                continue;
            }
            latch_inputs.insert(lv, ev);
            carried.insert(lv, out);
            let out_high = fd.vbank().get(out).and_then(|v| v.get_high());
            let mut c = lv;
            for _ in 0..4 {
                let Some(d) = fd.vbank().get(c).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else { break };
                if d.code() != OpCode::CPUI_COPY || !d.get_parent().is_some_and(|b| body.contains(&b)) {
                    break;
                }
                let Some(src) = d.get_in(0) else { break };
                let Some(sv) = fd.vbank().get(src) else { break };
                if !sv.is_written() || sv.is_implied() || latch_inputs.contains_key(&src) {
                    break;
                }
                if sv.get_high() != out_high {
                    let same_type = fd.vbank().get(out).is_some_and(|ov| {
                        std::rc::Rc::ptr_eq(ov.get_type(), sv.get_type())
                            || ov.get_type().compare(sv.get_type(), 10).map(|c| c == 0).unwrap_or(false)
                    });
                    if !same_type {
                        break;
                    }
                    alias.insert(src, out);
                }
                latch_inputs.insert(src, ev);
                carried.insert(src, out);
                c = src;
            }
        }
        Ok(Loop { fd, dw, body, header, cbranch, exit, entry, latch_inputs, carried, alias, cont_neg })
    }

    fn cond_vn(&self) -> VarnodeId {
        self.fd.obank().get(self.cbranch).and_then(|o| o.get_in(1)).expect("cbranch condition")
    }

    /// The printed condition reads, at the top, what it reads at the bottom.
    fn top_ok(&self) -> Result<Vec<(VarnodeId, VarnodeId)>, &'static str> {
        let mut budget = 256;
        let mut used = Vec::new();
        self.printed_ok(self.cond_vn(), &mut budget, &mut used)?;
        Ok(used)
    }

    fn printed_ok(&self, vn: VarnodeId, budget: &mut i32, used: &mut Vec<(VarnodeId, VarnodeId)>) -> Result<(), &'static str> {
        *budget -= 1;
        if *budget < 0 {
            return Err("condition too large");
        }
        let fd = self.fd;
        let v = fd.vbank().get(vn).ok_or("stale varnode")?;
        if v.is_constant() || !v.is_written() {
            return Ok(());
        }
        let def = v.get_def().ok_or("no def")?;
        let op = fd.obank().get(def).ok_or("stale op")?;
        if !op.get_parent().is_some_and(|p| self.body.contains(&p)) {
            return Ok(());
        }
        if v.is_implied() {
            if !pure_value_op(op.code()) {
                return Err("condition has a side effect");
            }
            let first = usize::from(op.code() == OpCode::CPUI_LOAD);
            for i in first as i32..op.num_input() {
                let input = op.get_in(i).ok_or("missing input")?;
                self.printed_ok(input, budget, used)?;
            }
            return Ok(());
        }
        if self.latch_inputs.contains_key(&vn) {
            if self.carried.get(&vn).is_some_and(|&out| self.on_stack(out)) {
                return Err("loop variable on the stack");
            }
            if let Some(&to) = self.alias.get(&vn) {
                if !used.iter().any(|&(f, _)| f == vn) {
                    used.push((vn, to));
                }
            }
            return Ok(());
        }
        Err("condition reads a body value")
    }

    /// Does the variable `out` belongs to live in the stack frame?  An optimizer that
    /// rotates loops keeps their counters in registers; one kept in memory is
    /// unoptimized code, where a bottom test is the source's own.
    fn on_stack(&self, out: VarnodeId) -> bool {
        let fd = self.fd;
        let Some(stack) = fd.get_arch().manage().get_stack_space() else { return false };
        let Some(h) = fd.vbank().get(out).and_then(|v| v.get_high()).and_then(|h| fd.high_bank().get(h)) else {
            return false;
        };
        (0..h.num_instances()).any(|i| {
            fd.vbank().get(h.get_instance(i)).is_some_and(|v| v.get_space().get_index() == stack.get_index())
        })
    }

    /// The continue condition evaluated at loop entry, as the top test would; LOADs in
    /// `fresh` read the memory the header sees.
    fn entry_cond(&self, fresh: &BTreeSet<OpId>) -> Option<Cond> {
        let b = Builder { fd: self.fd, subst: Some(&self.latch_inputs), body: &self.body, fresh };
        let mut budget = 128;
        let e = b.build(self.cond_vn(), 0, &mut budget)?;
        Some(cond_of(&e).negate_if(self.cont_neg))
    }

    /// The ordering test that agrees with the latch's `!=` on every iteration a guarded
    /// counted loop runs.  The latch compares a carried value `x` that steps by a constant
    /// `s` with a loop-invariant bound `b`; when the entry value starts strictly on the near
    /// side of `b` and `b - x_entry` is an exact multiple of `s`, `x` meets `b` exactly and
    /// never passes it, so `x != b` and `x < b` (`x > b` when `s` is negative) agree on
    /// every value `x` takes, and the arithmetic cannot wrap.  Returns the comparison op,
    /// the token it prints with, and the entry test `x_entry < b` the guard must equal.
    fn respell(&self, fresh: &BTreeSet<OpId>, decl: DeclType) -> Result<Respelled, &'static str> {
        let fd = self.fd;
        let cvn = self.cond_vn();
        let cv = fd.vbank().get(cvn).ok_or("stale")?;
        if !cv.is_implied() {
            return Err("condition is a variable");
        }
        let def = cv.get_def().ok_or("no def")?;
        let op = fd.obank().get(def).ok_or("stale")?;
        let ne = match op.code() {
            OpCode::CPUI_INT_NOTEQUAL if !self.cont_neg => true,
            OpCode::CPUI_INT_EQUAL if self.cont_neg => false,
            _ => return Err("not an equality latch"),
        };
        let (i0, i1) = (op.get_in(0).ok_or("input")?, op.get_in(1).ok_or("input")?);
        let (x_first, x, b) = if self.carried.contains_key(&i0) && self.invariant(i1, 0) {
            (true, i0, i1)
        } else if self.carried.contains_key(&i1) && self.invariant(i0, 0) {
            (false, i1, i0)
        } else {
            return Err("no carried value against an invariant bound");
        };
        let out = *self.carried.get(&x).ok_or("carried")?;
        let step = self.step(x, out).ok_or("no constant step")?;
        let bv = fd.vbank().get(b).ok_or("stale")?;
        let order = order_class(fd, self.alias.get(&x).copied().unwrap_or(x), decl).ok_or("counter type")?;
        let signed = order == Order::Signed;
        if bv.is_constant() {
            let v = bv.get_offset() & calc_mask(bv.get_size());
            let v = if signed { sext(v, bv.get_size()) } else { v as i64 };
            if !(0..1 << 31).contains(&v) {
                return Err("bound constant");
            }
        } else if bv.is_implied() {
            if class_of(bv.get_type()) != Some(order) || bv.get_size() < 4 {
                return Err("bound type");
            }
        } else if order_class(fd, b, decl) != Some(order) {
            return Err("bound type");
        }
        let builder = Builder { fd, subst: None, body: &self.body, fresh };
        let mut budget = 128;
        let ex = builder.build(*self.latch_inputs.get(&x).ok_or("entry")?, 0, &mut budget).ok_or("build")?;
        let eb = builder.build(b, 0, &mut budget).ok_or("build")?;
        let k = step.unsigned_abs();
        let exact = if step > 0 { multiple_of(&eb, &ex, k) } else { multiple_of(&ex, &eb, k) };
        if k != 1 && !exact {
            return Err("bound not a multiple of the step");
        }
        let code = if signed { OpCode::CPUI_INT_SLESS } else { OpCode::CPUI_INT_LESS };
        let (lo, hi) = if step > 0 { (ex, eb) } else { (eb, ex) };
        let entry = cond_of(&simplify(Expr::Op { code, size: 1, args: vec![lo, hi] }));
        let up = step > 0;
        let tok: &'static OpToken = match (ne, up == x_first) {
            (true, true) => &tokens::LESS_THAN,
            (true, false) => &tokens::GREATER_THAN,
            (false, true) => &tokens::GREATER_EQUAL,
            (false, false) => &tokens::LESS_EQUAL,
        };
        Ok(Respelled { op: def, tok, entry })
    }

    /// Is `vn` the same value on every iteration: defined before the loop, or an implied
    /// pure expression of such values?
    fn invariant(&self, vn: VarnodeId, depth: u32) -> bool {
        let fd = self.fd;
        let Some(v) = fd.vbank().get(vn) else { return false };
        if v.is_constant() || !v.is_written() {
            return true;
        }
        let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else { return false };
        if !op.get_parent().is_some_and(|p| self.body.contains(&p)) {
            return true;
        }
        if !v.is_implied() || depth > 8 || op.code() == OpCode::CPUI_LOAD || !pure_value_op(op.code()) {
            return false;
        }
        (0..op.num_input()).all(|i| op.get_in(i).is_some_and(|x| self.invariant(x, depth + 1)))
    }

    /// The constant `x` adds to the value `out` carried in from the previous iteration.
    fn step(&self, x: VarnodeId, out: VarnodeId) -> Option<i64> {
        let fd = self.fd;
        let is_out = |v: VarnodeId| {
            v == out
                || fd
                    .vbank()
                    .get(v)
                    .and_then(|v| v.get_def())
                    .and_then(|d| fd.obank().get(d))
                    .is_some_and(|d| d.code() == OpCode::CPUI_COPY && d.get_in(0) == Some(out))
        };
        let konst = |v: VarnodeId| {
            fd.vbank().get(v).filter(|v| v.is_constant()).map(|v| sext(v.get_offset(), v.get_size()))
        };
        let mut v = x;
        for _ in 0..4 {
            let op = fd.vbank().get(v)?.get_def().and_then(|d| fd.obank().get(d))?;
            if !op.get_parent().is_some_and(|p| self.body.contains(&p)) {
                return None;
            }
            let (a, b) = (op.get_in(0), op.get_in(1));
            let s = match op.code() {
                OpCode::CPUI_COPY => {
                    v = a?;
                    continue;
                }
                OpCode::CPUI_INT_ADD if is_out(a?) => konst(b?)?,
                OpCode::CPUI_INT_ADD if is_out(b?) => konst(a?)?,
                OpCode::CPUI_INT_SUB if is_out(a?) => konst(b?)?.checked_neg()?,
                OpCode::CPUI_PTRADD if is_out(a?) => konst(b?)?.checked_mul(konst(op.get_in(2)?)?)?,
                _ => return None,
            };
            return (s != 0).then_some(s);
        }
        None
    }

    /// The conditional edge that leads to the loop, walking back over straight-line blocks.
    fn guard(&self, fd: &Funcdata) -> Option<GuardSite> {
        let bb = fd.bblocks_ref();
        let mut chain = Vec::new();
        let mut cur = self.header;
        let mut p = self.entry;
        for _ in 0..4 {
            let pb = bb.block(p);
            if pb.size_out() == 2 {
                let other = if pb.get_out(0) == cur { pb.get_out(1) } else { pb.get_out(0) };
                if other == cur || self.body.contains(&other) {
                    return None;
                }
                let cb = fd.bb_op_tail(p)?;
                let o = fd.obank().get(cb)?;
                if o.code() != OpCode::CPUI_CBRANCH {
                    return None;
                }
                let enter_neg = if pb.get_out(1) == cur { o.is_boolean_flip() } else { !o.is_boolean_flip() };
                let mut site = GuardSite {
                    block: p,
                    cbranch: cb,
                    enter_neg,
                    pre: chain.iter().rev().copied().collect(),
                    skip: other,
                    cond: None,
                    enter: None,
                    fresh: BTreeSet::new(),
                };
                site.cond = site.compare(self);
                return Some(site);
            }
            if pb.size_out() != 1 || pb.size_in() != 1 {
                return None;
            }
            chain.push(p);
            cur = p;
            p = pb.get_in(0);
        }
        None
    }

    /// A conditional edge higher up the dominator tree whose condition equals the entry test.
    fn dominating_fact(&self, cont: Option<&Cond>) -> bool {
        let Some(cont) = cont else { return false };
        if cont.has_load() {
            return false;
        }
        let fd = self.fd;
        let bb = fd.bblocks_ref();
        let mut cur = self.header;
        for _ in 0..16 {
            let Some(d) = bb.block(cur).get_immed_dom() else { return false };
            let db = bb.block(d);
            if db.size_out() == 2 {
                if let Some(cb) = fd.bb_op_tail(d) {
                    if let Some(o) = fd.obank().get(cb).filter(|o| o.code() == OpCode::CPUI_CBRANCH) {
                        for side in 0..2 {
                            let t = db.get_out(side);
                            if bb.block(t).size_in() != 1 || !bb.dominates(t, Some(self.header)) {
                                continue;
                            }
                            let neg = if side == 1 { o.is_boolean_flip() } else { !o.is_boolean_flip() };
                            let b = Builder { fd, subst: None, body: &self.body, fresh: &BTreeSet::new() };
                            let mut budget = 128;
                            let Some(vn) = o.get_in(1) else { continue };
                            if let Some(e) = b.build(vn, 0, &mut budget) {
                                if &cond_of(&e).negate_if(neg) == cont {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            cur = d;
        }
        false
    }
}

/// The conditional block in front of a loop.
struct GuardSite {
    block: BlockId,
    cbranch: OpId,
    /// The loop is entered when the CBRANCH input xor this is true.
    enter_neg: bool,
    /// Straight-line blocks between the guard and the header, in execution order.
    pre: Vec<BlockId>,
    /// Where the guard goes when the loop is skipped.
    skip: BlockId,
    /// The guard's enter condition when it equals the loop's entry test.
    cond: Option<Cond>,
    /// The guard's enter condition.
    enter: Option<Cond>,
    /// LOADs that read the memory the loop header sees.
    fresh: BTreeSet<OpId>,
}

impl GuardSite {
    fn compare(&mut self, lp: &Loop) -> Option<Cond> {
        let fd = lp.fd;
        let mut fresh = BTreeSet::new();
        let mut pure_tail = true;
        let mut op = fd.bb_op_tail(self.block);
        while let Some(o) = op {
            let Some(p) = fd.obank().get(o) else { break };
            if p.code() == OpCode::CPUI_LOAD && pure_tail {
                fresh.insert(o);
            }
            if has_side_effect(p.code()) || p.is_call() {
                pure_tail = false;
            }
            op = p.basic_neighbours().0;
        }
        let pre_pure = self.pre.iter().all(|&b| block_ops(fd, b).iter().all(|&o| {
            fd.obank().get(o).is_some_and(|p| !has_side_effect(p.code()) && !p.is_call())
        }));
        if pre_pure {
            for &b in &self.pre {
                for o in block_ops(fd, b) {
                    if fd.obank().get(o).is_some_and(|p| p.code() == OpCode::CPUI_LOAD) {
                        fresh.insert(o);
                    }
                }
            }
        } else {
            fresh.clear();
        }
        let gvn = fd.obank().get(self.cbranch)?.get_in(1)?;
        let mut budget = 128;
        let gb = Builder { fd, subst: None, body: &lp.body, fresh: &fresh };
        let ge = gb.build(gvn, 0, &mut budget)?;
        let gc = cond_of(&ge).negate_if(self.enter_neg);
        self.enter = Some(gc.clone());
        self.fresh = fresh.clone();
        let lc = lp.entry_cond(&fresh)?;
        if gc == lc {
            Some(gc)
        } else {
            None
        }
    }

    /// The guard node to drop and how, when dropping it keeps the program.
    fn droppable(&self, lp: &Loop, leaves: &BTreeMap<BlockId, BlockId>) -> Result<Dropped, &'static str> {
        let fd = lp.fd;
        if !printed_pure(fd, self.cbranch) {
            return Err("guard test has a side effect");
        }
        let mut rets = None;
        let form = if self.skip == lp.exit {
            Guard::Wrapped
        } else {
            let a = bare_return(fd, self.skip).ok_or("skip is not a bare return")?;
            let b = bare_return(fd, lp.exit).ok_or("exit is not a bare return")?;
            if !same_return(fd, a, b) {
                return Err("returns differ");
            }
            if fd.bblocks_ref().block(self.skip).size_in() != 1 {
                return Err("shared skip return");
            }
            rets = Some((a, b));
            Guard::Return
        };
        let mut region: BTreeSet<BlockId> = lp.body.clone();
        region.extend(self.pre.iter().copied());
        for &b in &self.pre {
            for o in block_ops(fd, b) {
                let p = fd.obank().get(o).ok_or("stale op")?;
                if has_side_effect(p.code()) || p.is_call() || p.code() == OpCode::CPUI_LOAD {
                    return Err("pre not pure");
                }
                if p.not_printed() {
                    continue;
                }
                let Some(out) = p.get_out() else { continue };
                let v = fd.vbank().get(out).ok_or("stale varnode")?;
                if v.is_implied() {
                    continue;
                }
                if v.is_addr_tied() || v.is_persist() {
                    return Err("pre writes memory");
                }
                if form == Guard::Return {
                    continue;
                }
                let h = v.get_high().ok_or("no high")?;
                let hv = fd.high_bank().get(h).ok_or("stale high")?;
                for i in 0..hv.num_instances() {
                    let inst = fd.vbank().get(hv.get_instance(i)).ok_or("stale varnode")?;
                    let in_region = inst
                        .get_def()
                        .and_then(|d| fd.obank().get(d))
                        .and_then(|d| d.get_parent())
                        .is_some_and(|p| region.contains(&p));
                    if !in_region {
                        return Err("pre output live on skip path");
                    }
                }
            }
        }
        let sb = fd.sblocks_ref();
        let gleaf = *leaves.get(&self.block).ok_or("no guard leaf")?;
        let mut cond = gleaf;
        let ifn = loop {
            let parent = sb.block(cond).get_parent().ok_or("guard has no parent")?;
            let pb = sb.block(parent);
            match pb.get_type() {
                BlockType::Ls if pb.get_block(pb.get_size() - 1) == cond => cond = parent,
                BlockType::If if pb.get_block(0) == cond => break parent,
                _ => return Err("guard is not an if condition"),
            }
        };
        let ib = sb.block(ifn);
        if ib.get_size() != 2 || ib.get_if_goto_target().is_some() {
            return Err("guard if shape");
        }
        if let Some(parent) = ib.get_parent() {
            let pb = sb.block(parent);
            if pb.get_type() == BlockType::If && pb.get_size() == 3 && pb.get_block(2) == ifn {
                return Err("guard is an else arm");
            }
        }
        let pre_leaves: Vec<BlockId> =
            self.pre.iter().map(|b| leaves.get(b).copied().ok_or("no pre leaf")).collect::<Result<_, _>>()?;
        let then = flatten(fd, ib.get_block(1));
        let mut loop_seq = pre_leaves;
        loop_seq.push(lp.dw);
        if form == Guard::Wrapped {
            if then != loop_seq {
                return Err("guard body is not the loop");
            }
            return Ok((ifn, form, None));
        }
        let skip_leaf = *leaves.get(&self.skip).ok_or("no skip leaf")?;
        let exit_leaf = *leaves.get(&lp.exit).ok_or("no exit leaf")?;
        if sb.block(skip_leaf).is_unstructured_target() || sb.block(exit_leaf).is_unstructured_target() {
            return Err("labelled return");
        }
        let mut top = ifn;
        while let Some(p) = sb.block(top).get_parent() {
            if sb.block(p).get_type() != BlockType::Ls {
                break;
            }
            top = p;
        }
        let seq = flatten(fd, top);
        let at = seq.iter().position(|&x| x == ifn).ok_or("guard not in sequence")?;
        let follows = |want: &[BlockId]| seq.len() >= at + 1 + want.len() && seq[at + 1..at + 1 + want.len()] == *want;
        loop_seq.push(exit_leaf);
        if then == vec![skip_leaf] {
            if !follows(&loop_seq) {
                return Err("guard not followed by the loop");
            }
            return Ok((ifn, Guard::Return, None));
        }
        if then == loop_seq && follows(&[skip_leaf]) {
            let (a, b) = rets.ok_or("no returns")?;
            return Ok((ifn, Guard::Tail, Some((skip_leaf, a, b))));
        }
        Err("guard body is not the return")
    }
}

/// A comparison the latch can print as an ordering test.
struct Respelled {
    op: OpId,
    tok: &'static OpToken,
    entry: Cond,
}

/// How C orders values of a varnode's printed type.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    Signed,
    Unsigned,
    Pointer,
}

/// The ordering class of the type `vn` prints with: its high's declaration (the printer's
/// override first), else the varnode's own type.
fn order_class(fd: &Funcdata, vn: VarnodeId, decl: DeclType) -> Option<Order> {
    let v = fd.vbank().get(vn)?;
    let high = v.get_high()?;
    let ty = decl(high)
        .or_else(|| {
            crate::kuna_declhightype::type_representative(fd, high)
                .filter(|_| crate::kuna_declhightype::declares_from_high(fd, high))
                .and_then(|r| fd.vbank().get(r))
                .map(|r| r.get_type().clone())
        })
        .unwrap_or_else(|| v.get_type().clone());
    class_of(&ty)
}

fn class_of(ty: &Datatype) -> Option<Order> {
    use crate::dtype::type_metatype;
    match ty.get_metatype() {
        type_metatype::TYPE_INT => Some(Order::Signed),
        type_metatype::TYPE_UINT => Some(Order::Unsigned),
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => Some(Order::Pointer),
        _ => None,
    }
}

/// Is `hi` structurally `lo + n * k`: a pointer `&lo[n]` over `k`-byte elements (C pointer
/// arithmetic does not wrap), or `lo + n * k` / `lo + (n << j)` with `k` a power of two
/// (then the difference is a multiple of `k` modulo the word, hence exactly)?
fn multiple_of(hi: &Expr, lo: &Expr, k: u64) -> bool {
    if let (Expr::Op { code: OpCode::CPUI_PTRADD, args: h, .. }, Expr::Op { code: OpCode::CPUI_PTRADD, args: l, .. }) =
        (hi, lo)
    {
        if h.len() == 3 && l.len() == 3 && h[0] == l[0] && h[2] == l[2] && h[2].konst() == Some(k) {
            return true;
        }
    }
    let Expr::Op { code, args, .. } = hi else { return false };
    match code {
        OpCode::CPUI_PTRADD if args.len() == 3 => &args[0] == lo && args[2].konst() == Some(k),
        OpCode::CPUI_INT_ADD if args.len() == 2 && k.is_power_of_two() => {
            let scaled = if &args[0] == lo { &args[1] } else if &args[1] == lo { &args[0] } else { return false };
            match scaled {
                Expr::Op { code: OpCode::CPUI_INT_MULT, args: m, .. } if m.len() == 2 => {
                    m[0].konst().is_some_and(|c| c % k == 0) || m[1].konst().is_some_and(|c| c % k == 0)
                }
                Expr::Op { code: OpCode::CPUI_INT_LEFT, args: m, .. } if m.len() == 2 => {
                    m[1].konst().is_some_and(|j| j < 64 && (1u64 << j) % k == 0)
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The components of `id`, with nested lists spliced in.
fn flatten(fd: &Funcdata, id: BlockId) -> Vec<BlockId> {
    let sb = fd.sblocks_ref();
    let b = sb.block(id);
    if b.get_type() != BlockType::Ls {
        return vec![id];
    }
    b.get_list().iter().flat_map(|&c| flatten(fd, c)).collect()
}

fn block_ops(fd: &Funcdata, bb: BlockId) -> Vec<OpId> {
    let mut out = Vec::new();
    let mut op = fd.bb_op_head(bb);
    while let Some(o) = op {
        out.push(o);
        op = fd.bb_op_next(o);
    }
    out
}

/// The RETURN of a block that prints nothing else.
fn bare_return(fd: &Funcdata, bb: BlockId) -> Option<OpId> {
    let ops = crate::p8_structure::kuna_iteregion::printed_ops(fd, bb);
    if ops.len() != 1 {
        return None;
    }
    let o = fd.obank().get(ops[0])?;
    if o.code() != OpCode::CPUI_RETURN {
        return None;
    }
    if block_ops(fd, bb)
        .iter()
        .any(|&x| x != ops[0] && fd.obank().get(x).is_some_and(|p| p.is_call() || has_side_effect(p.code())))
    {
        return None;
    }
    Some(ops[0])
}

fn same_return(fd: &Funcdata, a: OpId, b: OpId) -> bool {
    let (Some(oa), Some(ob)) = (fd.obank().get(a), fd.obank().get(b)) else { return false };
    if oa.num_input() != ob.num_input() {
        return false;
    }
    if oa.num_input() < 2 {
        return true;
    }
    if oa.num_input() > 2 {
        return false;
    }
    let (Some(va), Some(vb)) = (oa.get_in(1), ob.get_in(1)) else { return false };
    if va == vb {
        return true;
    }
    let none = BTreeSet::new();
    let b = Builder { fd, subst: None, body: &none, fresh: &BTreeSet::new() };
    let (mut ba, mut bb) = (64, 64);
    match (b.build(va, 0, &mut ba), b.build(vb, 0, &mut bb)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// Does the printed expression of a CBRANCH call anything or write memory?
fn printed_pure(fd: &Funcdata, cbranch: OpId) -> bool {
    let Some(vn) = fd.obank().get(cbranch).and_then(|o| o.get_in(1)) else { return false };
    let mut stack = vec![vn];
    let mut budget = 256;
    while let Some(vn) = stack.pop() {
        budget -= 1;
        if budget < 0 {
            return false;
        }
        let Some(v) = fd.vbank().get(vn) else { return false };
        if !v.is_written() || !v.is_implied() {
            continue;
        }
        let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else { return false };
        if !pure_value_op(op.code()) {
            return false;
        }
        for i in 0..op.num_input() {
            if let Some(x) = op.get_in(i) {
                stack.push(x);
            }
        }
    }
    true
}

fn has_side_effect(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_STORE
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_NEW
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_RETURN
    )
}

fn pure_value_op(code: OpCode) -> bool {
    !has_side_effect(code)
        && !matches!(
            code,
            OpCode::CPUI_INDIRECT
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_CPOOLREF
                | OpCode::CPUI_SEGMENTOP
        )
}

// ---------------------------------------------------------------------------
// Expression matching
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Expr {
    Const { size: i32, val: u64 },
    Leaf(VarnodeId, i32),
    /// A memory read; `at` is `None` when it reads the memory the loop header sees.
    Load { addr: Box<Expr>, size: i32, at: Option<OpId> },
    Op { code: OpCode, size: i32, args: Vec<Expr> },
}

impl Expr {
    fn size(&self) -> i32 {
        match self {
            Expr::Const { size, .. } | Expr::Load { size, .. } | Expr::Op { size, .. } | Expr::Leaf(_, size) => *size,
        }
    }
    fn konst(&self) -> Option<u64> {
        match self {
            Expr::Const { val, .. } => Some(*val),
            _ => None,
        }
    }
    fn has_load(&self) -> bool {
        match self {
            Expr::Load { .. } => true,
            Expr::Op { args, .. } => args.iter().any(Expr::has_load),
            _ => false,
        }
    }
}

struct Builder<'a> {
    fd: &'a Funcdata,
    subst: Option<&'a BTreeMap<VarnodeId, VarnodeId>>,
    body: &'a BTreeSet<BlockId>,
    fresh: &'a BTreeSet<OpId>,
}

impl Builder<'_> {
    fn build(&self, vn: VarnodeId, depth: u32, budget: &mut i32) -> Option<Expr> {
        *budget -= 1;
        if *budget < 0 {
            return None;
        }
        let vn = self.subst.and_then(|m| m.get(&vn).copied()).unwrap_or(vn);
        let fd = self.fd;
        let v = fd.vbank().get(vn)?;
        let size = v.get_size();
        if v.is_constant() {
            return Some(Expr::Const { size, val: v.get_offset() & calc_mask(size) });
        }
        if !v.is_written() || depth > 12 {
            return Some(Expr::Leaf(vn, size));
        }
        let op = fd.obank().get(v.get_def()?)?;
        let code = op.code();
        let arg = |i: i32, budget: &mut i32| -> Option<Expr> { self.build(op.get_in(i)?, depth + 1, budget) };
        match code {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => arg(0, budget),
            OpCode::CPUI_LOAD => {
                let addr = arg(1, budget)?;
                let def = v.get_def()?;
                let in_body = op.get_parent().is_some_and(|p| self.body.contains(&p));
                let at = if in_body || self.fresh.contains(&def) { None } else { Some(def) };
                Some(Expr::Load { addr: Box::new(addr), size, at })
            }
            OpCode::CPUI_PTRSUB => {
                let a = arg(0, budget)?;
                let b = arg(1, budget)?;
                Some(simplify(Expr::Op { code: OpCode::CPUI_INT_ADD, size, args: vec![a, b] }))
            }
            OpCode::CPUI_PTRADD => {
                let a = arg(0, budget)?;
                let b = arg(1, budget)?;
                let c = arg(2, budget)?;
                Some(simplify(Expr::Op { code, size, args: vec![a, b, c] }))
            }
            _ if pure_value_op(code) && code != OpCode::CPUI_LOAD && op.num_input() <= 3 => {
                let mut args = Vec::new();
                for i in 0..op.num_input() {
                    args.push(arg(i, budget)?);
                }
                Some(simplify(Expr::Op { code, size, args }))
            }
            _ => Some(Expr::Leaf(vn, size)),
        }
    }
}

fn sext(val: u64, size: i32) -> i64 {
    let bits = size * 8;
    if bits >= 64 {
        return val as i64;
    }
    ((val << (64 - bits)) as i64) >> (64 - bits)
}

fn konst(size: i32, val: u64) -> Expr {
    Expr::Const { size, val: val & calc_mask(size) }
}

fn commutative(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
    )
}

/// Fold constants, order commutative operands, drop identities.
fn simplify(e: Expr) -> Expr {
    let Expr::Op { code, size, mut args } = e else { return e };
    if commutative(code) && args.len() == 2 && args[0] > args[1] {
        args.swap(0, 1);
    }
    let m = calc_mask(size);
    let ks: Vec<Option<u64>> = args.iter().map(Expr::konst).collect();
    if ks.iter().all(Option::is_some) && !ks.is_empty() {
        let a = ks[0].unwrap_or(0);
        let b = ks.get(1).copied().flatten().unwrap_or(0);
        let asz = args[0].size();
        let folded = match code {
            OpCode::CPUI_INT_ADD => Some(a.wrapping_add(b)),
            OpCode::CPUI_INT_SUB => Some(a.wrapping_sub(b)),
            OpCode::CPUI_INT_MULT => Some(a.wrapping_mul(b)),
            OpCode::CPUI_INT_AND => Some(a & b),
            OpCode::CPUI_INT_OR => Some(a | b),
            OpCode::CPUI_INT_XOR => Some(a ^ b),
            OpCode::CPUI_INT_NEGATE => Some(!a),
            OpCode::CPUI_INT_2COMP => Some(a.wrapping_neg()),
            OpCode::CPUI_INT_ZEXT => Some(a),
            OpCode::CPUI_INT_SEXT => Some(sext(a, asz) as u64),
            OpCode::CPUI_SUBPIECE if b < 8 => Some(a.checked_shr((b * 8) as u32).unwrap_or(0)),
            OpCode::CPUI_INT_LEFT if b < 64 => Some(a << b),
            OpCode::CPUI_INT_RIGHT if b < 64 => Some(a >> b),
            OpCode::CPUI_PTRADD if args.len() == 3 => {
                let c = ks[2].unwrap_or(0);
                Some(a.wrapping_add(b.wrapping_mul(c)))
            }
            _ => None,
        };
        if let Some(v) = folded {
            return Expr::Const { size, val: v & m };
        }
    }
    if code == OpCode::CPUI_SUBPIECE && ks.get(1) == Some(&Some(0)) {
        if let Expr::Op { code: OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, args: inner, .. } = &args[0] {
            if inner.len() == 1 && inner[0].size() == size {
                return inner[0].clone();
            }
        }
    }
    if args.len() == 2 {
        match code {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR if ks[0] == Some(0) => {
                return args.swap_remove(1);
            }
            OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT if ks[1] == Some(0) => {
                return args.swap_remove(0);
            }
            OpCode::CPUI_INT_MULT if ks[0] == Some(1) => return args.swap_remove(1),
            _ => {}
        }
    }
    Expr::Op { code, size, args }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Lit {
    True,
    Eq(Expr, Expr),
    Ltu(Expr, Expr),
    Lts(Expr, Expr),
    Other(Expr),
}

/// A literal, possibly negated.  `False` is a negated `True`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Cond {
    neg: bool,
    lit: Lit,
}

impl Cond {
    fn pos(lit: Lit) -> Cond {
        Cond { neg: false, lit }
    }
    fn negate(self) -> Cond {
        Cond { neg: !self.neg, lit: self.lit }
    }
    fn negate_if(self, n: bool) -> Cond {
        if n {
            self.negate()
        } else {
            self
        }
    }
    fn of_bool(b: bool) -> Cond {
        Cond { neg: !b, lit: Lit::True }
    }
    fn has_load(&self) -> bool {
        match &self.lit {
            Lit::True => false,
            Lit::Eq(a, b) | Lit::Ltu(a, b) | Lit::Lts(a, b) => a.has_load() || b.has_load(),
            Lit::Other(e) => e.has_load(),
        }
    }
}

fn cond_of(e: &Expr) -> Cond {
    if let Expr::Const { val, .. } = e {
        return Cond::of_bool(*val != 0);
    }
    if let Expr::Op { code, args, .. } = e {
        if args.len() == 2 {
            let (a, b) = (args[0].clone(), args[1].clone());
            match code {
                OpCode::CPUI_INT_EQUAL => return eq(a, b),
                OpCode::CPUI_INT_NOTEQUAL => return eq(a, b).negate(),
                OpCode::CPUI_INT_LESS => return ltu(a, b),
                OpCode::CPUI_INT_LESSEQUAL => return ltu(b, a).negate(),
                OpCode::CPUI_INT_SLESS => return lts(a, b),
                OpCode::CPUI_INT_SLESSEQUAL => return lts(b, a).negate(),
                _ => {}
            }
        }
        if *code == OpCode::CPUI_BOOL_NEGATE && args.len() == 1 {
            return cond_of(&args[0]).negate();
        }
    }
    if e.size() > 0 {
        return eq(e.clone(), konst(e.size(), 0)).negate();
    }
    Cond::pos(Lit::Other(e.clone()))
}

/// `&p[i]` and `&p[j]` over the same element size: two element pointers into one array
/// compare as their indices (C pointer arithmetic).
fn same_array(a: &Expr, b: &Expr) -> Option<(Expr, Expr)> {
    let (Expr::Op { code: OpCode::CPUI_PTRADD, args: x, .. }, Expr::Op { code: OpCode::CPUI_PTRADD, args: y, .. }) =
        (a, b)
    else {
        return None;
    };
    if x.len() != 3 || y.len() != 3 || x[0] != y[0] || x[2] != y[2] || x[1].size() != y[1].size() {
        return None;
    }
    Some((x[1].clone(), y[1].clone()))
}

/// `x == 0` with the conversions and scalings that cannot wrap removed.
fn zero_test(x: Expr) -> Expr {
    match x {
        Expr::Op { code: OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, ref args, .. } if args.len() == 1 => {
            zero_test(args[0].clone())
        }
        Expr::Op { code: OpCode::CPUI_INT_MULT, size, ref args } if args.len() == 2 => {
            let (k, n) = match (args[0].konst(), args[1].konst()) {
                (Some(k), None) => (k, &args[1]),
                (None, Some(k)) => (k, &args[0]),
                _ => return x.clone(),
            };
            if k != 0 && narrow_enough(n, size, k) {
                zero_test(n.clone())
            } else {
                x.clone()
            }
        }
        Expr::Op { code: OpCode::CPUI_INT_LEFT, size, ref args } if args.len() == 2 => match args[1].konst() {
            Some(s) if s < 64 && narrow_enough(&args[0], size, 1u64 << s) => zero_test(args[0].clone()),
            _ => x.clone(),
        },
        _ => x,
    }
}

/// Is `n * k` free of wrap-around in `size` bytes because `n` is extended from narrower?
fn narrow_enough(n: &Expr, size: i32, k: u64) -> bool {
    let Expr::Op { code: OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, args, .. } = n else { return false };
    let from_bits = args[0].size() * 8;
    let k_bits = 64 - k.leading_zeros() as i32;
    from_bits > 0 && from_bits + k_bits <= size * 8
}

fn eq(a: Expr, b: Expr) -> Cond {
    if let (Some(x), Some(y)) = (a.konst(), b.konst()) {
        return Cond::of_bool(x == y);
    }
    if let Some((i, j)) = same_array(&a, &b) {
        return eq(i, j);
    }
    for (p, q) in [(&a, &b), (&b, &a)] {
        if let Expr::Op { code, args, .. } = p {
            match code {
                OpCode::CPUI_PTRADD if args.len() == 3 && &args[0] == q => {
                    let n = zero_test(args[1].clone());
                    let sz = n.size();
                    return eq(n, konst(sz, 0));
                }
                OpCode::CPUI_INT_ADD if args.len() == 2 && (&args[0] == q || &args[1] == q) => {
                    let n = if &args[0] == q { args[1].clone() } else { args[0].clone() };
                    let sz = n.size();
                    return eq(n, konst(sz, 0));
                }
                _ => {}
            }
        }
    }
    let (a, b) = if a.konst().is_some() { (b, a) } else { (a, b) };
    if let Some(c) = b.konst() {
        if c == 0 {
            if let Expr::Op { code: OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_XOR, args, .. } = &a {
                if args.len() == 2 {
                    return eq(args[0].clone(), args[1].clone());
                }
            }
            let z = zero_test(a.clone());
            if z != a {
                let sz = z.size();
                return eq(z, konst(sz, 0));
            }
        }
        if let Expr::Op { code: OpCode::CPUI_INT_ADD, size, args } = &a {
            if args.len() == 2 {
                if let Some(k) = args[0].konst() {
                    return eq(args[1].clone(), konst(*size, c.wrapping_sub(k)));
                }
            }
        }
        return Cond::pos(Lit::Eq(a, b));
    }
    if a <= b {
        Cond::pos(Lit::Eq(a, b))
    } else {
        Cond::pos(Lit::Eq(b, a))
    }
}

fn ltu(a: Expr, b: Expr) -> Cond {
    if let Some((i, j)) = same_array(&a, &b) {
        return lts(i, j);
    }
    if let Expr::Op { code: OpCode::CPUI_PTRADD, args, .. } = &b {
        if args.len() == 3 && args[0] == a {
            let n = args[1].clone();
            let sz = n.size();
            return lts(konst(sz, 0), n);
        }
    }
    if let Expr::Op { code: OpCode::CPUI_PTRADD, args, .. } = &a {
        if args.len() == 3 && args[0] == b {
            let n = args[1].clone();
            let sz = n.size();
            return lts(n, konst(sz, 0));
        }
    }
    if let Some((x, c)) = narrowed(&a, &b, OpCode::CPUI_INT_ZEXT, false) {
        return ltu(x, c);
    }
    if let Some((c, x)) = narrowed(&b, &a, OpCode::CPUI_INT_ZEXT, false).map(|(x, c)| (c, x)) {
        return ltu(c, x);
    }
    let size = a.size().max(b.size());
    match (a.konst(), b.konst()) {
        (Some(x), Some(y)) => Cond::of_bool(x < y),
        (_, Some(0)) => Cond::of_bool(false),
        (Some(0), _) => {
            let sz = b.size();
            eq(b, konst(sz, 0)).negate()
        }
        (_, Some(1)) => {
            let sz = a.size();
            eq(a, konst(sz, 0))
        }
        (Some(c), _) if size > 0 && c < calc_mask(size) => ltu(b, konst(size, c + 1)).negate(),
        _ => Cond::pos(Lit::Ltu(a, b)),
    }
}

/// `ext(x)` compared with a constant that the narrower `x` can hold: the extension keeps
/// the order (zero extension for unsigned, sign extension for signed comparisons), so the
/// comparison is the narrow one.
fn narrowed(ext: &Expr, k: &Expr, code: OpCode, signed: bool) -> Option<(Expr, Expr)> {
    let Expr::Op { code: c, args, .. } = ext else { return None };
    if *c != code || args.len() != 1 {
        return None;
    }
    let x = &args[0];
    let (val, size) = match k {
        Expr::Const { val, size } => (*val, *size),
        _ => return None,
    };
    let narrow = x.size();
    if narrow <= 0 || narrow >= size {
        return None;
    }
    let fits = if signed {
        let v = sext(val, size);
        let half = 1i64 << (narrow * 8 - 1);
        (-half..half).contains(&v)
    } else {
        val <= calc_mask(narrow)
    };
    fits.then(|| (x.clone(), konst(narrow, val)))
}

fn lts(a: Expr, b: Expr) -> Cond {
    if let Some((x, c)) = narrowed(&a, &b, OpCode::CPUI_INT_SEXT, true) {
        return lts(x, c);
    }
    if let Some((c, x)) = narrowed(&b, &a, OpCode::CPUI_INT_SEXT, true).map(|(x, c)| (c, x)) {
        return lts(c, x);
    }
    let size = a.size().max(b.size());
    match (a.konst(), b.konst()) {
        (Some(x), Some(y)) if size > 0 => Cond::of_bool(sext(x, size) < sext(y, size)),
        (Some(c), _) if size > 0 && sext(c, size) < sext(calc_mask(size) >> 1, size) => {
            lts(b, konst(size, c.wrapping_add(1))).negate()
        }
        _ => Cond::pos(Lit::Lts(a, b)),
    }
}

// ---------------------------------------------------------------------------
// Printing
// ---------------------------------------------------------------------------

impl PrintC {
    /// A planned `BlockDoWhile` as `while (cond) { body }`: the same condition and body
    /// the do-while form prints, with the test first.
    pub(crate) fn emit_block_do_while_top(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        let body = fd.sblocks_ref().block(blk).get_block(0);
        self.emit.tag_line();
        self.emit.tag_op(self.lang().kw_while, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        self.emit.spaces(1, 0);
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH);
        self.loop_tests.active = self.loop_tests.subst.get(&blk).cloned().unwrap_or_default();
        self.loop_tests.active_respell = self.loop_tests.respell.get(&blk).copied();
        self.emit_block(fd, arch, body);
        self.loop_tests.active.clear();
        self.loop_tests.active_respell = None;
        self.context.pop_mod();
        let id = self.emit.open_brace_indent(self.lang().kw_open_curly, self.output_brace(self.options.brace_loop));
        self.flush_eol_warnings();
        self.context.set_mod(modifiers::NO_BRANCH);
        let id2 = self.emit.begin_block(0);
        self.emit_block(fd, arch, body);
        self.emit.end_block(id2);
        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
        self.flush_eol_warnings();
        self.context.pop_mod();
    }

    /// A planned guard `BlockIf`: its condition's statements, then (wrapped form) its body.
    pub(crate) fn emit_dropped_guard(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId, form: Guard) {
        let cond = fd.sblocks_ref().block(blk).get_block(0);
        self.context.push_mod();
        self.context.unset_mod(modifiers::ONLY_BRANCH | modifiers::PENDING_BRACE);
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, cond);
        self.emit_comment_block_tree(fd, cond);
        if form != Guard::Return {
            let id = self.emit.begin_block(0);
            self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
            self.emit.end_block(id);
        }
        self.context.pop_mod();
    }
}

#[cfg(test)]
#[path = "kuna_looptest/tests.rs"]
mod tests;
