//! Expand a `rep movs` that builds a by-value argument block, with a known
//! count, before the stack is heritaged.
//!
//! SLEIGH lifts a repeated string move as a loop inside the one instruction: a
//! head that leaves when the count register is zero, and a body that steps the
//! count down by one, moves one element and branches back. Heritage cannot see
//! which frame slots the walking pointer reaches. A by-value struct copied into
//! the outgoing area (`sub esp,N; mov edi,esp; rep movsd; call f`) then leaves
//! `f` without the arguments it builds, the slots it overwrites (an earlier
//! call's arguments and pushed return address) keep their old values, and a
//! pointer to the bottom of the frame makes every frame slot look aliased, so
//! earlier calls lose their pushed arguments too.
//!
//! On x86-32 (a 4-byte stack pointer), right after the first heritage pass,
//! which links the registers but not yet the stack, such a loop is replaced by
//! that many copies of its body, in
//! order, so the expansion is exact, overlapping moves included. A move
//! qualifies when its count enters as the same constant on every path, it
//! copies at most [`MAX_ELEMENTS`] elements and [`MAX_BYTES`] bytes, and its
//! destination is the stack pointer itself. When such a move reads the frame,
//! the moves that write the frame (the copy that built the struct) are
//! expanded with it, so its reads see their writes; it is left alone while any
//! other store reaches the frame through a pointer the stack pointer plus a
//! constant does not place, whose guards may not cover the slots the copies
//! would read directly. The copies' frame accesses become stack slots in the
//! same pass that turns pushes into them, before the stack's own first
//! heritage.

use std::collections::BTreeMap;

use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// The most elements a move may copy and still be expanded.
pub(crate) const MAX_ELEMENTS: u64 = 16;

/// The most bytes a move may copy and still be expanded.
pub(crate) const MAX_BYTES: u64 = 64;

/// The stack pointer width of the one target, x86-32. On x86-64 the copy
/// leaves `RCX` zero and `RSI` at the source's end, which the System V
/// register trials of a call that follows read as arguments.
const STACK_POINTER_BYTES: i32 = 4;

/// One `rep movs` loop: `head` tests the count and leaves when it is zero,
/// `body` moves one element and branches back.
struct RepMove {
    head: BlockId,
    body: BlockId,
    /// Each head phi: its output and the value the body hands back.
    phis: Vec<(VarnodeId, VarnodeId)>,
    count: u64,
    store: OpId,
    reads_frame: bool,
    writes_frame: bool,
    /// The destination is the stack pointer itself: an argument block.
    to_arguments: bool,
}

fn defining(fd: &Funcdata, vn: VarnodeId) -> Option<&crate::op::PcodeOp> {
    fd.vbank().get(vn)?.get_def().and_then(|d| fd.obank().get(d))
}

/// The constant `vn` holds, folding pure operations on constants.
fn constant(fd: &Funcdata, vn: VarnodeId, depth: u32) -> Option<u64> {
    let v = fd.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    if depth == 0 {
        return None;
    }
    let op = defining(fd, vn)?;
    let size = v.get_size();
    if op.code() == OpCode::CPUI_COPY {
        return constant(fd, op.get_in(0)?, depth - 1);
    }
    let behavior = fd.get_arch().op_behavior(op.code())?;
    if behavior.is_special() || size > 8 {
        return None;
    }
    let input = op.get_in(0)?;
    let input_size = fd.vbank().get(input)?.get_size();
    let a = constant(fd, input, depth - 1)?;
    let result = if behavior.is_unary() {
        behavior.evaluate_unary(size, input_size, a).ok()?
    } else {
        let b = constant(fd, op.get_in(1)?, depth - 1)?;
        behavior.evaluate_binary(size, input_size, a, b).ok()?
    };
    Some(result & kuna_base::address::calc_mask(size))
}

/// Where [`frame_offset`] leaves a value.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    /// The stack pointer plus this constant.
    At(u64),
    /// The stack pointer plus a constant a call's adjustment of the stack
    /// pointer, not yet solved, makes unknown.
    Shifted,
    /// The phi being resolved, unchanged around a loop.
    Cycle,
    Unplaced,
}

/// The constant `vn` adds to the stack pointer `sp`, through copies,
/// constant additions and phis whose inputs agree.
fn frame_offset(fd: &Funcdata, sp: VarnodeId, vn: VarnodeId, visiting: &mut Vec<VarnodeId>, budget: &mut u32) -> Place {
    if vn == sp {
        return Place::At(0);
    }
    if *budget == 0 {
        return Place::Unplaced;
    }
    *budget -= 1;
    let Some(op) = defining(fd, vn) else { return Place::Unplaced };
    let size = fd.vbank().get(vn).map_or(0, |v| v.get_size());
    let shift = |place: Place, by: u64| match place {
        Place::At(o) => Place::At(o.wrapping_add(by) & kuna_base::address::calc_mask(size)),
        Place::Shifted => Place::Shifted,
        Place::Cycle if by == 0 => Place::Cycle,
        _ => Place::Unplaced,
    };
    match (op.code(), op.get_in(0), op.get_in(1)) {
        (OpCode::CPUI_COPY, Some(a), _) => frame_offset(fd, sp, a, visiting, budget),
        (OpCode::CPUI_INT_ADD, Some(a), Some(b)) => match (constant(fd, a, 8), constant(fd, b, 8)) {
            (_, Some(c)) => shift(frame_offset(fd, sp, a, visiting, budget), c),
            (Some(c), _) => shift(frame_offset(fd, sp, b, visiting, budget), c),
            _ => Place::Unplaced,
        },
        (OpCode::CPUI_INT_SUB, Some(a), Some(b)) => match constant(fd, b, 8) {
            Some(c) => shift(frame_offset(fd, sp, a, visiting, budget), c.wrapping_neg()),
            None => Place::Unplaced,
        },
        (OpCode::CPUI_INDIRECT, Some(a), _)
            if fd.vbank().get(vn).is_some_and(|v| fd.vbank().get(sp).is_some_and(|s| v.get_addr() == s.get_addr() && v.get_size() == s.get_size())) =>
        {
            match frame_offset(fd, sp, a, visiting, budget) {
                Place::At(_) | Place::Shifted => Place::Shifted,
                other => other,
            }
        }
        (OpCode::CPUI_MULTIEQUAL, _, _) => {
            if visiting.contains(&vn) {
                return Place::Cycle;
            }
            visiting.push(vn);
            let mut found = Place::Cycle;
            for i in 0..op.num_input() {
                let Some(input) = op.get_in(i) else {
                    found = Place::Unplaced;
                    break;
                };
                match (frame_offset(fd, sp, input, visiting, budget), found) {
                    (Place::Cycle, _) => {}
                    (Place::At(o), Place::Cycle) => found = Place::At(o),
                    (Place::At(o), Place::At(p)) if o == p => {}
                    (Place::Shifted, Place::Cycle) => found = Place::Shifted,
                    _ => {
                        found = Place::Unplaced;
                        break;
                    }
                }
            }
            visiting.pop();
            found
        }
        _ => Place::Unplaced,
    }
}

/// Is `vn` the stack pointer `sp` plus a constant?
fn on_frame(fd: &Funcdata, sp: VarnodeId, vn: VarnodeId) -> bool {
    matches!(frame_offset(fd, sp, vn, &mut Vec::new(), &mut 256), Place::At(_) | Place::Shifted)
}

/// Is `vn` a value of the stack pointer register itself, through copies?
fn stack_pointer_value(fd: &Funcdata, sp: VarnodeId, mut vn: VarnodeId) -> bool {
    let Some(reg) = fd.vbank().get(sp).map(|v| (v.get_addr().clone(), v.get_size())) else { return false };
    for _ in 0..16 {
        let Some(v) = fd.vbank().get(vn) else { return false };
        if *v.get_addr() == reg.0 && v.get_size() == reg.1 {
            return true;
        }
        match defining(fd, vn) {
            Some(op) if op.code() == OpCode::CPUI_COPY => match op.get_in(0) {
                Some(i) => vn = i,
                None => return false,
            },
            _ => return false,
        }
    }
    false
}

/// Can `vn` be derived from the stack pointer `sp` at all?
fn from_frame(fd: &Funcdata, sp: VarnodeId, vn: VarnodeId) -> bool {
    let mut work = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some(v) = work.pop() {
        if v == sp {
            return true;
        }
        if seen.contains(&v) || seen.len() >= 256 {
            continue;
        }
        seen.push(v);
        let Some(op) = defining(fd, v) else { continue };
        if matches!(
            op.code(),
            OpCode::CPUI_COPY
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_PTRADD
                | OpCode::CPUI_PTRSUB
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
        ) {
            work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)));
        }
    }
    false
}

/// The `rep movs` loop headed by `head`, if it is one with a small constant
/// count that reads or writes the frame.
fn rep_move(fd: &Funcdata, sp: VarnodeId, head: BlockId) -> Option<RepMove> {
    let graph = fd.bblocks_ref();
    let h = graph.block(head);
    if h.size_in() < 2 || h.size_out() != 2 {
        return None;
    }
    let body = h.get_false_out();
    let exit = h.get_true_out();
    if body == head || exit == head || exit == body {
        return None;
    }
    let b = graph.block(body);
    if b.size_in() != 1 || b.size_out() != 1 || b.get_out(0) != head {
        return None;
    }
    let back = h.get_in_index(body);
    if back < 0 || (0..h.size_in()).filter(|&i| h.get_in(i) == body).count() != 1 {
        return None;
    }
    let head_ops = fd.bb_ops(head);
    let (&branch, rest) = head_ops.split_last()?;
    let (&test, phi_ops) = rest.split_last()?;
    let branch_op = fd.obank().get(branch)?;
    let test_op = fd.obank().get(test)?;
    if branch_op.code() != OpCode::CPUI_CBRANCH
        || branch_op.is_boolean_flip()
        || test_op.code() != OpCode::CPUI_INT_EQUAL
        || branch_op.get_in(1) != test_op.get_out()
    {
        return None;
    }
    let address = test_op.get_addr().clone();
    let mut phis = Vec::new();
    let mut entry: BTreeMap<VarnodeId, Vec<VarnodeId>> = BTreeMap::new();
    for &id in phi_ops {
        let op = fd.obank().get(id)?;
        if op.code() != OpCode::CPUI_MULTIEQUAL || op.num_input() != h.size_in() {
            return None;
        }
        let out = op.get_out()?;
        phis.push((out, op.get_in(back)?));
        let inputs = (0..op.num_input()).filter(|&i| i != back).map(|i| op.get_in(i)).collect::<Option<Vec<_>>>()?;
        entry.insert(out, inputs);
    }
    let zero = test_op.get_in(1).and_then(|v| fd.vbank().get(v))?;
    if !zero.is_constant() || zero.get_offset() != 0 {
        return None;
    }
    let counter = test_op.get_in(0)?;
    let &(_, stepped) = phis.iter().find(|p| p.0 == counter)?;
    let step = defining(fd, stepped)?;
    let size = fd.vbank().get(stepped)?.get_size();
    let decrement = match step.code() {
        OpCode::CPUI_INT_SUB => 1,
        OpCode::CPUI_INT_ADD => kuna_base::address::calc_mask(size),
        _ => return None,
    };
    if step.get_parent() != Some(body)
        || step.get_in(0) != Some(counter)
        || step.get_in(1).and_then(|v| constant(fd, v, 0)) != Some(decrement)
    {
        return None;
    }
    let counts: Vec<Option<u64>> = entry[&counter].iter().map(|&v| constant(fd, v, 8)).collect();
    let count = counts[0]?;
    if counts.iter().any(|&c| c != Some(count)) {
        return None;
    }
    let body_ops = fd.bb_ops(body);
    let (&last, ops) = body_ops.split_last()?;
    if fd.obank().get(last)?.code() != OpCode::CPUI_BRANCH || ops.len() > 64 {
        return None;
    }
    let sp_addr = fd.vbank().get(sp)?.get_addr().clone();
    let sp_size = fd.vbank().get(sp)?.get_size();
    let (mut loads, mut stores) = (Vec::new(), Vec::new());
    for &id in body_ops.iter() {
        let op = fd.obank().get(id)?;
        if *op.get_addr() != address {
            return None;
        }
        if id == last {
            continue;
        }
        match op.code() {
            OpCode::CPUI_LOAD => loads.push(id),
            OpCode::CPUI_STORE => stores.push(id),
            OpCode::CPUI_COPY => {}
            code if !op.is_call() && fd.get_arch().op_behavior(code).is_some_and(|b| !b.is_special()) => {}
            _ => return None,
        }
        if let Some(out) = op.get_out().and_then(|v| fd.vbank().get(v)) {
            if out.get_addr().overlap(0, &sp_addr, sp_size) >= 0 || sp_addr.overlap(0, out.get_addr(), out.get_size()) >= 0 {
                return None;
            }
        }
    }
    let (&[load], &[store]) = (loads.as_slice(), stores.as_slice()) else { return None };
    let element = fd.vbank().get(fd.obank().get(load)?.get_out()?)?.get_size() as u64;
    if fd.vbank().get(fd.obank().get(store)?.get_in(2)?)?.get_size() as u64 != element {
        return None;
    }
    if !(1..=MAX_ELEMENTS).contains(&count) || count * element > MAX_BYTES {
        return None;
    }
    let entering = |vn: VarnodeId| -> Option<&Vec<VarnodeId>> {
        let mut v = vn;
        for _ in 0..8 {
            if let Some(inputs) = entry.get(&v) {
                return Some(inputs);
            }
            match defining(fd, v) {
                Some(op) if op.code() == OpCode::CPUI_COPY && op.get_parent() == Some(body) => v = op.get_in(0)?,
                _ => return None,
            }
        }
        None
    };
    let sources = entering(fd.obank().get(load)?.get_in(1)?)?;
    let destinations = entering(fd.obank().get(store)?.get_in(1)?)?;
    let reads_frame = sources.iter().all(|&v| on_frame(fd, sp, v));
    let writes_frame = destinations.iter().all(|&v| on_frame(fd, sp, v));
    let to_arguments = destinations.iter().all(|&v| stack_pointer_value(fd, sp, v));
    if !reads_frame && !writes_frame {
        return None;
    }
    Some(RepMove { head, body, phis, count, store, reads_frame, writes_frame, to_arguments })
}

/// Does a STORE outside `moves` reach the frame through a pointer the stack
/// pointer plus a constant does not place?
fn walks_frame(fd: &Funcdata, sp: VarnodeId, moves: &[RepMove]) -> bool {
    fd.obank().iter_code(OpCode::CPUI_STORE).any(|id| {
        if moves.iter().any(|m| m.store == id) {
            return false;
        }
        let Some(ptr) = fd.obank().get(id).and_then(|op| op.get_in(1)) else { return false };
        !on_frame(fd, sp, ptr) && from_frame(fd, sp, ptr)
    })
}

fn expand(fd: &mut Funcdata, rep: &RepMove) -> kuna_base::error::KunaResult<()> {
    let body_ops = fd.bb_ops(rep.body);
    let body = &body_ops[..body_ops.len() - 1];
    let mut after: Vec<(VarnodeId, Vec<(OpId, i32)>)> = Vec::new();
    for &(out, _) in &rep.phis {
        let mut uses = Vec::new();
        for op in fd.vbank().get(out).map(|v| v.descend_iter().collect::<Vec<_>>()).unwrap_or_default() {
            let Some(o) = fd.obank().get(op) else { continue };
            if o.get_parent() == Some(rep.body) || o.get_parent() == Some(rep.head) {
                continue;
            }
            for slot in 0..o.num_input() {
                if o.get_in(slot) == Some(out) {
                    uses.push((op, slot));
                }
            }
        }
        after.push((out, uses));
    }
    let head = fd.bblocks_ref().block(rep.head);
    let to_body = (0..head.size_out()).find(|&i| head.get_out(i) == rep.body).unwrap();
    let test = fd
        .bb_op_tail(rep.head)
        .and_then(|b| fd.obank().get(b))
        .and_then(|b| b.get_in(1))
        .and_then(|t| fd.vbank().get(t))
        .and_then(|t| t.get_def());
    fd.branch_remove_internal(rep.head, to_body)?;
    if let Some(test) = test {
        fd.op_destroy(test);
    }
    fd.bblocks_mut().block_mut(rep.body).set_dead();
    fd.branch_remove_internal(rep.body, 0)?;
    let mut current: BTreeMap<VarnodeId, VarnodeId> = rep.phis.iter().map(|p| (p.0, p.0)).collect();
    for _ in 0..rep.count {
        let mut produced: BTreeMap<VarnodeId, VarnodeId> = BTreeMap::new();
        for &id in body {
            let op = fd.obank().get(id).unwrap();
            let code = op.code();
            let address = op.get_addr().clone();
            let inputs: Vec<VarnodeId> = (0..op.num_input()).filter_map(|slot| op.get_in(slot)).collect();
            let output = op.get_out();
            let copy = fd.new_op(inputs.len() as i32, address);
            fd.op_set_opcode_code(copy, code);
            for (slot, input) in inputs.into_iter().enumerate() {
                let value = if let Some(&v) = produced.get(&input).or_else(|| current.get(&input)) {
                    v
                } else if let Some((size, value)) =
                    fd.vbank().get(input).filter(|v| v.is_constant()).map(|v| (v.get_size(), v.get_offset()))
                {
                    fd.new_constant(size, value)
                } else {
                    input
                };
                fd.op_set_input(copy, value, slot as i32)?;
            }
            if let Some(output) = output {
                let (size, addr, unique) = {
                    let v = fd.vbank().get(output).unwrap();
                    (v.get_size(), v.get_addr().clone(), v.get_space().get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
                };
                let value = if unique { fd.new_unique_out(size, copy)? } else { fd.new_varnode_out(size, &addr, copy)? };
                produced.insert(output, value);
            }
            fd.op_insert_end(copy, rep.head);
        }
        current = rep
            .phis
            .iter()
            .map(|&(out, back)| (out, produced.get(&back).or_else(|| current.get(&back)).copied().unwrap_or(back)))
            .collect();
    }
    for (out, uses) in after {
        let last = current[&out];
        for (op, slot) in uses {
            fd.op_set_input(op, last, slot)?;
        }
    }
    fd.block_remove_internal(rep.body, true)?;
    Ok(())
}

/// Expand every frame `rep movs` with a small known count, right after the
/// first heritage pass; the number expanded.
pub(crate) fn expand_all(fd: &mut Funcdata) -> i32 {
    if fd.get_heritage_pass() != 1
        || !fd.stack_store_guard()
        || fd.get_arch().index_alias_guard == super::kuna_indexaliasguard::LEVEL_OFF
    {
        return 0;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space().cloned() else { return 0 };
    let Some(sp) = fd.find_spacebase_input(&stack) else { return 0 };
    if fd.vbank().get(sp).map_or(true, |v| v.get_size() != STACK_POINTER_BYTES) {
        return 0;
    }
    let found: Vec<RepMove> =
        (0..fd.bblocks_get_size()).filter_map(|i| rep_move(fd, sp, fd.bblocks_get_block(i))).collect();
    if !found.iter().any(|m| m.to_arguments) {
        return 0;
    }
    let feed = found.iter().any(|m| m.to_arguments && m.reads_frame);
    let mut moves: Vec<RepMove> = found.into_iter().filter(|m| m.to_arguments || (feed && m.writes_frame)).collect();
    while let Some(i) = moves.iter().position(|m| m.reads_frame && walks_frame(fd, sp, &moves)) {
        moves.remove(i);
        if !moves.iter().any(|m| m.to_arguments && m.reads_frame) {
            moves.retain(|m| m.to_arguments);
        }
    }
    let mut expanded = 0;
    for rep in &moves {
        if expand(fd, rep).is_err() {
            break;
        }
        expanded += 1;
    }
    if expanded > 0 {
        fd.structure_reset();
    }
    expanded
}
