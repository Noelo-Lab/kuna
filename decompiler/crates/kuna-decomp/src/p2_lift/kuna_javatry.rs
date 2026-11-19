//! (kuna `javatry`) Exception edges for a DEX method's `try_item`s.
//!
//! A DEX method lists its protected ranges and their catch clauses beside the
//! code. Nothing in the instruction stream reaches a catch handler, so without
//! the table every handler is unreachable and the catch bodies vanish.
//!
//! With the option on, flow gives the handlers their incoming edges:
//!
//! - **The dispatch.** Each handler is entered once from the start of the first
//!   range that names it: `if (java_try(handler)) goto handler;` ahead of that
//!   instruction. A handler whose ranges cover more is dispatched first, so an
//!   enclosing `try` comes before the one it contains. The structurer sees the
//!   dispatch as an `if`, and the printer prints it as `try { } catch`.
//! - **The exceptional edges.** Every other instruction in a range that can
//!   throw is preceded by `if (java_throws()) goto handler;` for each of its
//!   handlers. These carry the register state at the throw to the handler, so a
//!   value the protected code assigns reaches the catch body. They are data
//!   flow only: structuring drops them ([`is_throws_edge`]).
//! - **The caught value.** `move-exception vA`, which the Dalvik spec leaves
//!   empty, becomes `vA = java_caught(handler)`.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;

use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::kuna_dex::{DexModel, TryBlock, REGISTER_START};
use crate::op::pcodeop_flags;
use crate::userop::{BUILTIN_JAVA_CAUGHT, BUILTIN_JAVA_THROWS, BUILTIN_JAVA_TRY};

/// Where a handler's `move-exception` (if any) leaves the exception, and where
/// the handler continues after it (through one `goto`, or falling through).
fn handler_stub(model: &DexModel, handler: u64) -> Option<(Option<u8>, u64)> {
    let head = model.bytes_at(handler, 2)?;
    let (register, at) = if head[0] == 0x0d { (Some(head[1]), handler + 2) } else { (None, handler) };
    let op = model.bytes_at(at, 1)?[0];
    let target = match op {
        0x28 => at.wrapping_add_signed(2 * i64::from(model.bytes_at(at + 1, 1)?[0] as i8)),
        0x29 => {
            let b = model.bytes_at(at + 2, 2)?;
            at.wrapping_add_signed(2 * i64::from(i16::from_le_bytes([b[0], b[1]])))
        }
        _ if register.is_some() => at,
        _ => return None,
    };
    Some((register, target))
}

/// The try items with each clause's handler replaced by the first handler of
/// the same item whose code is identical: D8 gives every type of a multi-type
/// catch its own `move-exception; goto` stub into the one catch body.
pub fn canonical_blocks(model: &DexModel, entry: u64) -> Vec<TryBlock> {
    let mut blocks = model.try_blocks(entry);
    for block in &mut blocks {
        let stubs: Vec<Option<(Option<u8>, u64)>> = block.handlers.iter().map(|&(_, h)| handler_stub(model, h)).collect();
        for k in 1..block.handlers.len() {
            let Some(stub) = &stubs[k] else { continue };
            if let Some(j) = (0..k).find(|&j| stubs[j].as_ref() == Some(stub)) {
                block.handlers[k].1 = block.handlers[j].1;
            }
        }
    }
    blocks
}

/// What flow adds ahead of the instructions of one method.
#[derive(Clone, Debug, Default)]
pub struct JavaTryPlan {
    blocks: Vec<TryBlock>,
    /// Range start -> the handlers dispatched there, outermost first.
    dispatch: BTreeMap<u64, Vec<u64>>,
}

impl JavaTryPlan {
    /// The plan for the method whose code starts at `entry`.
    pub fn new(model: &DexModel, entry: u64) -> JavaTryPlan {
        let mut blocks = canonical_blocks(model, entry);
        blocks.sort_by_key(|b| b.start);
        let mut spans: BTreeMap<u64, (u64, u64, usize)> = BTreeMap::new();
        let mut order = 0;
        for block in &blocks {
            for &(_, handler) in &block.handlers {
                let span = spans.entry(handler).or_insert((block.start, block.end, order));
                span.0 = span.0.min(block.start);
                span.1 = span.1.max(block.end);
                order += 1;
            }
        }
        let mut dispatch: BTreeMap<u64, Vec<(u64, u64, usize)>> = BTreeMap::new();
        for (&handler, &(start, end, first)) in &spans {
            dispatch.entry(start).or_default().push((handler, end, first));
        }
        let dispatch = dispatch
            .into_iter()
            .map(|(start, mut list)| {
                list.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
                (start, list.into_iter().map(|(h, _, _)| h).collect())
            })
            .collect();
        JavaTryPlan { blocks, dispatch }
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    fn block_at(&self, addr: u64) -> Option<&TryBlock> {
        self.blocks.iter().find(|b| b.start <= addr && addr < b.end)
    }

    /// The `try_item`s, sorted by start.
    pub fn blocks(&self) -> &[TryBlock] {
        &self.blocks
    }
}

/// Can the Dalvik instruction with opcode `op` throw? Moves, constants,
/// branches, compares and arithmetic other than integer division cannot.
pub fn can_throw(op: u8) -> bool {
    match op {
        0x00..=0x1b => false,
        0x28..=0x3d => false,
        0x7b..=0x92 | 0x95..=0x9d | 0xa0..=0xb2 | 0xb5..=0xbd | 0xc0..=0xd2 | 0xd5..=0xda | 0xdd..=0xe2 => false,
        _ => true,
    }
}

fn callother(data: &mut Funcdata, at: &Address, id: u32, args: &[u64]) -> KunaResult<OpId> {
    let op = data.new_op(1 + args.len() as i32, at.clone());
    data.op_set_opcode_code(op, OpCode::CPUI_CALLOTHER);
    if let Some(o) = data.obank_mut().get_mut(op) {
        o.clear_flag(pcodeop_flags::call);
    }
    let c = data.new_constant(4, u64::from(id));
    data.op_set_input(op, c, 0)?;
    for (slot, &arg) in args.iter().enumerate() {
        let v = data.new_constant(4, arg);
        data.op_set_input(op, v, slot as i32 + 1)?;
    }
    Ok(op)
}

fn edge(data: &mut Funcdata, at: &Address, id: u32, handler: u64) -> KunaResult<OpId> {
    let test = callother(data, at, id, &[handler])?;
    let flag = data.new_unique_out(1, test)?;
    let branch = data.new_op(2, at.clone());
    data.op_set_opcode_code(branch, OpCode::CPUI_CBRANCH);
    let dest = data.new_code_ref(&Address::new(at.get_space().expect("code space").clone(), handler));
    data.op_set_input(branch, dest, 0)?;
    data.op_set_input(branch, flag, 1)?;
    Ok(test)
}

/// Emit the ops that lead the instruction at `curaddr`, before it is
/// translated: the caught value at a handler, then the dispatches of the ranges
/// starting here, then the exceptional edges of a throwing instruction.
/// Returns the first op of the dispatches, which flow makes a basic block of
/// its own so the statements before the `try` stay out of its condition.
pub fn before_instruction(data: &mut Funcdata, plan: &JavaTryPlan, model: &DexModel, curaddr: &Address) -> KunaResult<Option<OpId>> {
    if plan.is_empty() {
        return Ok(None);
    }
    let off = curaddr.get_offset();
    let Some(&[opcode, operand]) = model.bytes_at(off, 2).map(|b| <&[u8; 2]>::try_from(b).unwrap()) else {
        return Ok(None);
    };
    let is_handler = plan.blocks.iter().any(|b| b.handlers.iter().any(|h| h.1 == off));
    if is_handler && opcode == 0x0d {
        let op = callother(data, curaddr, BUILTIN_JAVA_CAUGHT, &[off])?;
        let register = data
            .get_arch()
            .manage
            .get_space_by_name("register")
            .cloned()
            .ok_or_else(|| kuna_base::error::KunaError::lowlevel("Dalvik spec has no register space"))?;
        data.new_varnode_out(4, &Address::new(register, REGISTER_START + 4 * u64::from(operand)), op)?;
    }
    let dispatched: &[u64] = plan.dispatch.get(&off).map_or(&[], Vec::as_slice);
    let mut first = None;
    for &handler in dispatched {
        let test = edge(data, curaddr, BUILTIN_JAVA_TRY, handler)?;
        first.get_or_insert(test);
    }
    if let Some(block) = plan.block_at(off) {
        if can_throw(opcode) {
            let mut done: Vec<u64> = Vec::new();
            for &(_, handler) in &block.handlers {
                let skip = block.start == off && dispatched.contains(&handler);
                if !skip && !done.contains(&handler) {
                    edge(data, curaddr, BUILTIN_JAVA_THROWS, handler)?;
                    done.push(handler);
                }
            }
        }
    }
    Ok(first)
}

/// Does basic block `bb` hold nothing but `java_try` dispatches?
pub fn is_dispatch_only(data: &Funcdata, bb: crate::context::BlockId) -> bool {
    data.bb_op_tail(bb).is_some_and(|tail| dispatch_handler(data, tail).is_some())
        && data.bb_ops(bb).iter().all(|&op| {
            data.obank().get(op).is_some_and(|o| match o.code() {
                OpCode::CPUI_CBRANCH => true,
                OpCode::CPUI_CALLOTHER => data.vbank().get(o.get_in(0).unwrap_or_default()).is_some_and(|v| v.get_offset() == u64::from(BUILTIN_JAVA_TRY)),
                OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_COPY => true,
                _ => false,
            })
        })
}

/// The CALLOTHER builtin id feeding `op`'s condition, for a CBRANCH.
fn cbranch_builtin(data: &Funcdata, op: OpId) -> Option<(u64, OpId)> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let mut def_id = data.vbank().get(o.get_in(1)?)?.get_def()?;
    let mut def = data.obank().get(def_id)?;
    while def.code() == OpCode::CPUI_BOOL_NEGATE || def.code() == OpCode::CPUI_COPY {
        def_id = data.vbank().get(def.get_in(0)?)?.get_def()?;
        def = data.obank().get(def_id)?;
    }
    if def.code() != OpCode::CPUI_CALLOTHER {
        return None;
    }
    let id = data.vbank().get(def.get_in(0)?)?;
    id.is_constant().then(|| (id.get_offset(), def_id))
}

/// Is `op` the CBRANCH of an exceptional edge (structuring drops these)?
pub fn is_throws_edge(data: &Funcdata, op: OpId) -> bool {
    cbranch_builtin(data, op).is_some_and(|(id, _)| id == u64::from(BUILTIN_JAVA_THROWS))
}

/// The handler a CBRANCH dispatches to, when it is a `java_try` dispatch.
pub fn dispatch_handler(data: &Funcdata, op: OpId) -> Option<u64> {
    let (id, def) = cbranch_builtin(data, op)?;
    if id != u64::from(BUILTIN_JAVA_TRY) {
        return None;
    }
    let o = data.obank().get(def)?;
    data.vbank().get(o.get_in(1)?).map(|v| v.get_offset())
}

/// The catch clauses of one function, for the printer.
#[derive(Clone, Debug)]
pub struct CatchTable {
    /// Handler -> the ranges whose clauses name it, and the types it catches
    /// (`None` for the catch-all).
    handlers: BTreeMap<u64, (Vec<(u64, u64)>, Vec<Option<u32>>)>,
    model: std::rc::Rc<DexModel>,
}

impl CatchTable {
    /// The table of the function `data`, when it is a DEX method with try items
    /// and option `javatry` is on.
    pub fn new(data: &Funcdata) -> Option<CatchTable> {
        let arch = data.get_arch();
        let model = arch.dex.as_ref().filter(|_| arch.java_try)?;
        let mut handlers: BTreeMap<u64, (Vec<(u64, u64)>, Vec<Option<u32>>)> = BTreeMap::new();
        for block in canonical_blocks(model, data.get_address().get_offset()) {
            for &(type_idx, handler) in &block.handlers {
                let entry = handlers.entry(handler).or_default();
                if !entry.0.contains(&(block.start, block.end)) {
                    entry.0.push((block.start, block.end));
                }
                if !entry.1.contains(&type_idx) {
                    entry.1.push(type_idx);
                }
            }
        }
        // D8 leaves instructions that cannot throw out of a range, which splits
        // one `try` into several ranges; a gap of such instructions is still
        // inside the `try`.
        for (ranges, _) in handlers.values_mut() {
            ranges.sort_unstable();
            let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
            for &(start, end) in ranges.iter() {
                match merged.last_mut() {
                    Some(last) if start <= last.1 || !throws_between(model, last.1, start) => last.1 = last.1.max(end),
                    _ => merged.push((start, end)),
                }
            }
            *ranges = merged;
        }
        (!handlers.is_empty()).then(|| CatchTable { handlers, model: std::rc::Rc::clone(model) })
    }

    /// The ranges whose clauses name `handler`, sorted.
    pub fn ranges(&self, handler: u64) -> Vec<(u64, u64)> {
        let mut ranges = self.handlers.get(&handler).map(|h| h.0.clone()).unwrap_or_default();
        ranges.sort_unstable();
        ranges
    }

    /// Is `addr` inside a range whose clauses name `handler`?
    pub fn covers(&self, handler: u64, addr: u64) -> bool {
        self.handlers.get(&handler).is_some_and(|h| h.0.iter().any(|&(s, e)| s <= addr && addr < e))
    }

    /// The Java spelling of what `handler` catches: `IOException`, or
    /// `A | B` for a handler several clauses share.
    pub fn type_text(&self, handler: u64) -> String {
        let types = self.handlers.get(&handler).map(|h| h.1.clone()).unwrap_or_default();
        let names: Vec<String> = types.iter().map(|&t| self.model.catch_type_name(t)).collect();
        if names.is_empty() {
            "Throwable".to_string()
        } else {
            names.join(" | ")
        }
    }

    /// The descriptor of the exception `handler` receives: its one type, or
    /// `Throwable` when it catches several.
    pub fn caught_descriptor(&self, handler: u64) -> String {
        match self.handlers.get(&handler).map(|h| h.1.as_slice()) {
            Some([one]) => self.model.catch_descriptor(*one),
            _ => "Ljava/lang/Throwable;".to_string(),
        }
    }

    /// Can the instruction at `addr` throw?
    pub fn can_throw_at(&self, addr: u64) -> bool {
        self.model.bytes_at(addr, 1).is_some_and(|b| can_throw(b[0]))
    }
}

/// Can an instruction in `[from, to)` throw (or the walk not tell)?
fn throws_between(model: &DexModel, from: u64, to: u64) -> bool {
    let mut at = from;
    while at < to {
        let Some(op) = model.bytes_at(at, 1).map(|b| b[0]) else { return true };
        if can_throw(op) {
            return true;
        }
        let Some(size) = model.instruction_size(at) else { return true };
        at += size;
    }
    false
}

/// The exception handler a `java_caught` op receives, when `op` is one.
pub fn caught_handler(data: &Funcdata, op: OpId) -> Option<u64> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CALLOTHER || o.num_input() != 2 {
        return None;
    }
    let id = data.vbank().get(o.get_in(0)?)?;
    if !id.is_constant() || id.get_offset() != u64::from(BUILTIN_JAVA_CAUGHT) {
        return None;
    }
    data.vbank().get(o.get_in(1)?).map(|v| v.get_offset())
}

/// The basic blocks that end in a dispatch.
pub fn dispatch_blocks(data: &Funcdata) -> Vec<crate::context::BlockId> {
    (0..data.bblocks_get_size())
        .map(|i| data.bblocks_get_block(i))
        .filter(|&bb| data.bb_op_tail(bb).is_some_and(|op| dispatch_handler(data, op).is_some()))
        .collect()
}

/// The basic blocks this function's dispatches enter (their handlers).
pub fn dispatch_targets(data: &Funcdata) -> std::collections::BTreeSet<crate::context::BlockId> {
    dispatch_blocks(data)
        .into_iter()
        .filter_map(|bb| {
            let tail = data.bb_op_tail(bb)?;
            let taken = if data.obank().get(tail)?.is_fallthru_true() { 0 } else { 1 };
            let block = data.bblocks_ref().block(bb);
            (block.size_out() == 2).then(|| block.get_out(taken))
        })
        .collect()
}

/// Drop the exceptional edges from the structuring graph. They stay in the
/// control-flow graph, where they carry data flow to the handlers; the handler
/// is structured as the arm of its dispatch.
pub fn drop_throws_edges(data: &mut Funcdata) {
    let mut drops = Vec::new();
    for i in 0..data.bblocks_get_size() {
        let bb = data.bblocks_get_block(i);
        let Some(tail) = data.bb_op_tail(bb) else { continue };
        if !is_throws_edge(data, tail) {
            continue;
        }
        let taken = if data.obank().get(tail).is_some_and(|o| o.is_fallthru_true()) { 0 } else { 1 };
        let blocks = data.bblocks_ref();
        let block = blocks.block(bb);
        if block.size_out() != 2 {
            continue;
        }
        let target = block.get_out(taken);
        if let (Some(from), Some(to)) = (block.get_copy_map(), blocks.block(target).get_copy_map()) {
            drops.push((from, to));
        }
    }
    for (from, to) in drops {
        data.sblocks_mut().remove_edge(from, to);
    }
}

/// Remove the exceptional edges that carry nothing to their handler: an edge
/// whose register values at the handler's entry are also brought by another
/// edge. Each one split a basic block, so dropping them (the next `mainloop`
/// splices the halves) keeps the protected code's conditions whole.
pub struct ActionJavaThrowPrune {
    base: crate::action::ActionBase,
}

impl ActionJavaThrowPrune {
    pub fn boxed(group: &str) -> Box<dyn crate::action::Action> {
        Box::new(ActionJavaThrowPrune { base: crate::action::ActionBase::new(0, "javathrowprune", group) })
    }
}

/// The exceptional edges of the function, by the handler block they enter:
/// `(block, out-index)` of each.
fn throws_edges(data: &Funcdata) -> std::collections::BTreeMap<crate::context::BlockId, Vec<(crate::context::BlockId, i32)>> {
    let mut by_handler: std::collections::BTreeMap<_, Vec<_>> = std::collections::BTreeMap::new();
    for i in 0..data.bblocks_get_size() {
        let bb = data.bblocks_get_block(i);
        let Some(tail) = data.bb_op_tail(bb) else { continue };
        if !is_throws_edge(data, tail) || data.bblocks_ref().block(bb).size_out() != 2 {
            continue;
        }
        let taken = if data.obank().get(tail).is_some_and(|o| o.is_fallthru_true()) { 0 } else { 1 };
        by_handler.entry(data.bblocks_ref().block(bb).get_out(taken)).or_default().push((bb, taken));
    }
    by_handler
}

impl crate::action::Action for ActionJavaThrowPrune {
    fn base(&self) -> &crate::action::ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut crate::action::ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &crate::action::ActionGroupList) -> Option<Box<dyn crate::action::Action>> {
        grouplist.contains(self.get_group()).then(|| {
            Box::new(ActionJavaThrowPrune { base: self.base.clone() }) as Box<dyn crate::action::Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut crate::action::ActionContext) -> crate::action::ApplyResult {
        if !data.get_arch().java_try {
            return 0;
        }
        let mut removed = false;
        for (handler, edges) in throws_edges(data) {
            // How many in-edges bring each value, per MULTIEQUAL of the handler.
            let phis: Vec<OpId> = data
                .bb_ops(handler)
                .into_iter()
                .filter(|&op| data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_MULTIEQUAL))
                .collect();
            let mut counts: Vec<std::collections::HashMap<crate::context::VarnodeId, usize>> = phis
                .iter()
                .map(|&op| {
                    let mut map = std::collections::HashMap::new();
                    let o = data.obank().get(op).expect("phi");
                    for k in 0..o.num_input() {
                        if let Some(v) = o.get_in(k) {
                            *map.entry(v).or_insert(0) += 1;
                        }
                    }
                    map
                })
                .collect();
            for (bb, taken) in edges {
                let slot = data.bblocks_ref().block(handler).get_in_index(bb);
                if slot < 0 {
                    continue;
                }
                let brings: Vec<Option<crate::context::VarnodeId>> = phis
                    .iter()
                    .map(|&op| {
                        data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_MULTIEQUAL).and_then(|o| o.get_in(slot))
                    })
                    .collect();
                let redundant = brings
                    .iter()
                    .zip(&counts)
                    .all(|(v, map)| v.is_none_or(|v| map.get(&v).copied().unwrap_or(0) >= 2));
                if !redundant {
                    continue;
                }
                let test = data
                    .bb_op_tail(bb)
                    .and_then(|op| data.obank().get(op)?.get_in(1))
                    .and_then(|v| data.vbank().get(v)?.get_def());
                if data.branch_remove_internal(bb, taken).is_err() {
                    continue;
                }
                for (v, map) in brings.iter().zip(counts.iter_mut()) {
                    if let Some(c) = v.and_then(|v| map.get_mut(&v)) {
                        *c -= 1;
                    }
                }
                if let Some(test) = test {
                    let unused = data
                        .obank()
                        .get(test)
                        .and_then(|o| o.get_out())
                        .and_then(|v| data.vbank().get(v))
                        .is_some_and(|v| v.has_no_descend());
                    if unused {
                        data.op_destroy(test);
                    }
                }
                removed = true;
                self.base.count += 1;
            }
        }
        if removed {
            data.structure_reset();
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_division_and_object_access_throw() {
        for op in [0x6e, 0x52, 0x44, 0x22, 0x27, 0x93, 0xb4, 0xdb, 0x1d, 0x21] {
            assert!(can_throw(op), "{op:#x}");
        }
        for op in [0x01, 0x0d, 0x12, 0x28, 0x32, 0x90, 0xa6, 0xd0, 0x7b] {
            assert!(!can_throw(op), "{op:#x}");
        }
    }
}
