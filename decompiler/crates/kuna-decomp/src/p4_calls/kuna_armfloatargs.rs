//! Scalar VFP input recovery from typed reads and proven callee contracts.

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::{
    context::VarnodeId,
    dtype::{type_class, type_metatype, Datatype},
    fspec::{FuncCallSpecs, ParameterPieces, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
    varnode::varnode_flags,
};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub struct ActionArmFloatArgs {
    base: ActionBase,
}

impl ActionArmFloatArgs {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "armfloatargs", group),
        })
    }
}

impl Action for ActionArmFloatArgs {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups
            .contains(self.get_group())
            .then(|| Self::boxed(self.get_group()))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        prepare_inputs(data);
        0
    }
}

/// The option is on over `armfloatreturn`'s widened model, whose positional
/// rules keep an unused leading VFP parameter in its slot.
pub fn applies(arch: &Architecture) -> bool {
    arch.arm_float_args && crate::kuna_armfloatreturn::applies(arch)
}

/// Keep a proven double argument independent of an overlapping float call result.
/// Run after passthrough claims, which must inspect the function's original reads.
pub fn link_call_inputs(data: &mut Funcdata, index: i32) {
    if !data.get_arch().arm_float_args {
        return;
    }
    let call = data.get_call_specs(index);
    if call.is_input_locked() || call.is_dotdotdot() {
        return;
    }
    let op = call.get_op();
    if !data
        .obank()
        .get(op)
        .is_some_and(|o| o.code() == OpCode::CPUI_CALL)
    {
        return;
    }
    let Some(stated) = data.kuna_protoorder_types(call.get_entry_address()) else {
        return;
    };
    if !stated.arity_sound {
        return;
    }
    let Some((result_addr, result_size, result_type)) = &stated.output else {
        return;
    };
    if *result_size != 4 || result_type.get_metatype() != type_metatype::TYPE_FLOAT {
        return;
    }
    let inputs: Vec<_> = stated
        .inputs
        .iter()
        .filter(|(a, s, t)| {
            *s == 8
                && t.get_metatype() == type_metatype::TYPE_FLOAT
                && a == result_addr
                && call.proto().model().input().get_entry().iter().any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(a, *s) == 0
                })
        })
        .map(|(a, s, _)| (a.clone(), *s))
        .collect();
    for (addr, size) in inputs {
        if data
            .get_call_specs(index)
            .active_input()
            .which_trial(&addr, size)
            >= 0
        {
            continue;
        }
        data.get_call_specs_mut(index)
            .get_active_input()
            .register_trial(&addr, size);
        let vn = data.new_varnode(size, &addr, None);
        let slot = data.obank().get(op).map_or(1, |o| o.num_input());
        let _ = data.op_insert_input(op, vn, slot);
    }
}

/// The s-register words the callee's recovered prototype states.
fn stated_words(data: &Funcdata, call: &FuncCallSpecs) -> Vec<Address> {
    if !data.get_arch().arm_float_args || call.is_input_locked() || !call.proto().has_model() {
        return Vec::new();
    }
    let Some(stated) = data
        .kuna_protoorder_types(call.get_entry_address())
        .filter(|s| s.arity_sound)
    else {
        return Vec::new();
    };
    let entries = call.proto().model().input().get_entry();
    stated
        .inputs
        .iter()
        .filter(|(a, s, _)| {
            *s == 4
                && entries.iter().any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT
                        && e.get_size() == 4
                        && e.justified_contain(a, 4) == 0
                })
        })
        .map(|(a, _, _)| a.clone())
        .collect()
}

/// The callee's stated s-register parameters inside a d-register range the call
/// may not take whole, so the call reads each word it states.
pub fn stated_singles(
    data: &Funcdata,
    call: &FuncCallSpecs,
    addr: &Address,
    size: i32,
) -> Vec<Address> {
    if size != 8 {
        return Vec::new();
    }
    stated_words(data, call)
        .into_iter()
        .filter(|a| addr.justified_contain(size, a, 4, false) >= 0)
        .collect()
}

/// Keep a hole the callee states as s-register words from taking a whole
/// d-register, and a slot the callee's body reads from being a back-fill hole.
pub fn mark_single_floats(call: &mut FuncCallSpecs, data: &Funcdata) {
    if data.get_arch().arm_float_args {
        let words = stated_words(data, call);
        let read = call.proto().has_model().then(|| {
            read_singles(
                call.proto().model().input().get_entry(),
                data.kuna_callee_entry_dead(call.get_entry_address()),
            )
        });
        call.get_active_input().set_single_floats(words);
        call.get_active_input()
            .set_read_singles(read.unwrap_or_default());
    }
}

/// The s-register parameter slots a body reads before writing, by its probe.
fn read_singles(
    entries: &[crate::fspec::ParamEntry],
    probe: Option<&crate::kuna_calleedeadarg::CalleeEntryDead>,
) -> Vec<Address> {
    let Some(probe) = probe else {
        return Vec::new();
    };
    entries
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 4)
        .map(|e| Address::new(e.get_space().clone(), e.get_base()))
        .filter(|a| probe.proves_read(a, 4))
        .collect()
}

/// A slot the function's own body reads on entry is one of its parameters,
/// even where it would be a back-fill slot.
pub fn mark_own_reads(active: &mut crate::fspec::ParamActive, data: &Funcdata) {
    if data.get_arch().arm_float_args && data.get_func_proto().has_model() {
        active.set_read_singles(read_singles(
            data.get_func_proto().model().input().get_entry(),
            data.kuna_own_entry_dead(),
        ));
    }
}

/// The VFP input trials the caller's own data flow scored as arguments, taken
/// before the positional rules run.
pub fn written_inputs(call: &FuncCallSpecs, data: &Funcdata) -> Vec<(Address, i32)> {
    if !data.get_arch().arm_float_args {
        return Vec::new();
    }
    let active = call.active_input();
    (0..active.get_num_trials())
        .map(|i| active.get_trial(i))
        .filter(|t| t.is_active())
        .map(|t| (t.get_address().clone(), t.get_size()))
        .collect()
}

/// Bind a call's VFP inputs to its callee's arity-sound contract, up to the
/// last stated input the caller wrote or the callee provably reads: stated
/// inputs in front of it become arguments, a stated double the call holds as
/// its two words becomes one argument, and a filler the contract skips is
/// dropped. Another unstated input goes only where the callee neither reads
/// nor forwards it. A stated input the scoring released and the callee reads,
/// or one held in part, leaves the call as recovered.
pub fn cap_stated_inputs(call: &mut FuncCallSpecs, data: &mut Funcdata, written: &[(Address, i32)]) {
    if !data.get_arch().arm_float_args || call.is_input_locked() || !call.proto().has_model() {
        return;
    }
    let Some(plan) = plan_stated_inputs(call, data, written) else {
        return;
    };
    let active = call.get_active_input();
    for &i in &plan.drop {
        active.get_trial_mut(i).mark_no_use();
    }
    if plan.partial {
        return;
    }
    for join in &plan.joins {
        join_words(call, data, join);
    }
    for &i in &plan.zeros {
        let (slot, size) = {
            let t = call.active_input().get_trial(i);
            (t.get_slot(), t.get_size())
        };
        if slot >= 1 {
            let zero = data.new_constant(size, 0);
            let _ = data.op_set_input(call.get_op(), zero, slot);
        }
    }
    let active = call.get_active_input();
    for &i in &plan.bind {
        let t = active.get_trial_mut(i);
        t.mark_active();
        t.mark_used();
    }
    if plan.missing.is_empty() {
        return;
    }
    let entries = call.proto().model().input().get_entry().to_vec();
    let active = call.get_active_input();
    for (addr, size, e) in &plan.missing {
        let at = active.get_num_trials();
        active.register_trial(addr, *size);
        let t = active.get_trial_mut(at);
        t.mark_unref();
        t.set_entry(Some(*e), 0);
        t.mark_active();
        t.mark_used();
    }
    active.sort_trials(&entries);
}

/// What [`cap_stated_inputs`] does to one call's trials.
struct StatedPlan {
    drop: Vec<i32>,
    bind: Vec<i32>,
    zeros: Vec<i32>,
    joins: Vec<WordJoin>,
    missing: Vec<(Address, i32, usize)>,
    partial: bool,
}

/// A stated double the call holds as its low and high word trials, passed as
/// their PIECE or, where the callee ignores it, as zero.
struct WordJoin {
    lo: i32,
    hi: i32,
    addr: Address,
    size: i32,
    entry: usize,
    zero: bool,
}

fn plan_stated_inputs(
    call: &FuncCallSpecs,
    data: &Funcdata,
    written: &[(Address, i32)],
) -> Option<StatedPlan> {
    let entry = call.get_entry_address().clone();
    let stated = data.kuna_protoorder_types(&entry).filter(|s| s.arity_sound)?;
    let entries = call.proto().model().input().get_entry();
    let vfp = |addr: &Address, size: i32| {
        entries.iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
        })
    };
    let entry_of = |addr: &Address, size: i32| {
        entries.iter().position(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT
                && e.get_size() == size
                && e.justified_contain(addr, size) == 0
        })
    };
    let wanted: Vec<(Address, i32)> = stated
        .inputs
        .iter()
        .filter(|(a, s, _)| vfp(a, *s))
        .map(|(a, s, _)| (a.clone(), *s))
        .collect();
    let dead = data.kuna_callee_entry_dead(&entry);
    let forward = data.kuna_callee_forward(&entry);
    let overlaps = |a: &Address, s: i32, b: &Address, t: i32| {
        a.overlap(0, b, t) >= 0 || b.overlap(0, a, s) >= 0
    };
    let active = call.active_input();
    let trials: Vec<_> = (0..active.get_num_trials())
        .map(|i| {
            let t = active.get_trial(i);
            (
                i,
                t.get_address().clone(),
                t.get_size(),
                t.is_used(),
                t.is_unref(),
            )
        })
        .collect();
    if wanted.len() < stated.inputs.len() && !trials.iter().any(|(_, _, _, used, _)| *used) {
        return None;
    }
    let reads = data
        .kuna_callee_entry_through(&entry)
        .or_else(|| data.kuna_callee_entry_dead(&entry));
    let passed = |i: i32| {
        data.obank()
            .get(call.get_op())
            .and_then(|op| op.get_in(active.get_trial(i).get_slot()))
    };
    let widens = |parts: &[Option<VarnodeId>]| {
        let parts: Option<Vec<VarnodeId>> = parts.iter().copied().collect();
        parts.map_or(true, |parts| widens_entry(data, &parts, call.get_op()))
    };
    let words = |addr: &Address, size: i32| -> Option<WordJoin> {
        if trials.iter().any(|(_, a, s, _, _)| a == addr && *s == size) {
            return None;
        }
        let parts: Vec<_> = trials
            .iter()
            .filter(|(_, a, s, _, _)| overlaps(addr, size, a, *s))
            .collect();
        let [first, second] = parts.as_slice() else {
            return None;
        };
        let word = |(i, a, s, _, unref): &&(i32, Address, i32, bool, bool)| {
            let t = active.get_trial(*i);
            (*s * 2 == size && !*unref && !t.is_definitely_not_used() && t.get_slot() >= 1)
                .then(|| (*i, addr.justified_contain(size, a, *s, false)))
        };
        let (lo, hi) = match (word(first)?, word(second)?) {
            ((lo, 0), (hi, off)) | ((hi, off), (lo, 0)) if off * 2 == size => (lo, hi),
            _ => return None,
        };
        let half = size / 2;
        let read_whole = reads.is_some_and(|r| {
            r.proves_input(addr, half) && r.proves_input(&(addr + i64::from(half)), half)
        });
        let widened = widens(&[passed(lo), passed(hi)]);
        let zero = if dead.is_some_and(|d| d.proves_dead(addr, size)) {
            widened
        } else if read_whole && !widened {
            false
        } else {
            return None;
        };
        Some(WordJoin { lo, hi, addr: addr.clone(), size, entry: entry_of(addr, size)?, zero })
    };
    let pairs: Vec<WordJoin> = wanted.iter().filter_map(|(a, s)| words(a, *s)).collect();
    let taken = |i: i32| {
        let (_, a, s, used, _) = &trials[i as usize];
        *used || written.iter().any(|(w, z)| w == a && z == s)
    };
    let reached = wanted
        .iter()
        .filter(|(addr, size)| {
            written.iter().any(|(a, s)| a == addr && s == size)
                || trials
                    .iter()
                    .any(|(_, a, s, used, _)| *used && a == addr && s == size)
                || pairs
                    .iter()
                    .any(|j| j.addr == *addr && j.size == *size && (taken(j.lo) || taken(j.hi)))
                || reads.is_some_and(|r| r.proves_input(addr, *size))
        })
        .map(|(a, _)| a)
        .fold(None::<&Address>, |last, a| match last {
            Some(l) if !above(a, l) => Some(l),
            _ => Some(a),
        });
    let in_reach = |addr: &Address| reached.is_some_and(|last| last == addr || above(last, addr));
    let joins: Vec<WordJoin> = pairs.into_iter().filter(|j| in_reach(&j.addr)).collect();
    let joined = |i: &i32| joins.iter().any(|j| j.lo == *i || j.hi == *i);
    let mut drop = Vec::new();
    let mut kept = Vec::new();
    for (i, addr, size, used, unref) in &trials {
        if !used
            || !vfp(addr, *size)
            || joined(i)
            || wanted.iter().any(|(a, s)| a == addr && s == size)
        {
            continue;
        }
        let filler = *unref && wanted.iter().any(|(a, _)| above(a, addr));
        let unreached = dead.is_some_and(|d| {
            !d.proves_read(addr, *size)
                && (d.returns_untouched(addr, *size)
                    || forward.is_some_and(|f| f.transfer_free(addr, *size)))
        });
        if filler || unreached {
            drop.push(*i);
        } else {
            kept.push((addr.clone(), *size));
        }
    }
    let ignored = |addr: &Address, size: i32| dead.is_some_and(|d| d.proves_dead(addr, size));
    let mut bind = Vec::new();
    let mut zeros = Vec::new();
    let mut missing = Vec::new();
    let mut partial = false;
    for (addr, size) in &wanted {
        if !in_reach(addr) {
            continue;
        }
        let exact = trials.iter().find(|(_, a, s, _, _)| a == addr && s == size);
        partial |= kept.iter().any(|(a, s)| overlaps(addr, *size, a, *s));
        if let Some((i, ..)) = exact {
            let parts = passed(*i).map_or(vec![None], |vn| pieces(data, vn));
            if ignored(addr, *size) && widens(&parts) {
                zeros.push(*i);
            }
        }
        match exact {
            Some((i, _, _, false, _)) if active.get_trial(*i).is_definitely_not_used() => {
                if ignored(addr, *size) {
                    bind.push(*i);
                } else {
                    partial = true;
                }
            }
            Some((i, _, _, false, _)) => bind.push(*i),
            Some(_) => {}
            None if joins.iter().any(|j| j.addr == *addr && j.size == *size) => {}
            None if trials
                .iter()
                .any(|(_, a, s, _, _)| overlaps(addr, *size, a, *s)) =>
            {
                partial = true
            }
            None => match entry_of(addr, *size) {
                Some(e) => missing.push((addr.clone(), *size, e)),
                None => {}
            },
        }
    }
    Some(StatedPlan { drop, bind, zeros, joins, missing, partial })
}

/// Pass a stated double held as two word trials as one argument: a PIECE of
/// the two words, or zero for a double the callee ignores, takes the low word's
/// slot, and the high word is released.
fn join_words(call: &mut FuncCallSpecs, data: &mut Funcdata, join: &WordJoin) {
    let op = call.get_op();
    let (lo_slot, hi_slot) = {
        let active = call.active_input();
        (active.get_trial(join.lo).get_slot(), active.get_trial(join.hi).get_slot())
    };
    let Some(call_op) = data.obank().get(op) else {
        return;
    };
    let (Some(lo), Some(hi)) = (call_op.get_in(lo_slot), call_op.get_in(hi_slot)) else {
        return;
    };
    let whole = if join.zero {
        data.new_constant(join.size, 0)
    } else {
        let pc = call_op.get_addr().clone();
        let piece = data.new_op(2, pc);
        data.op_set_opcode_code(piece, OpCode::CPUI_PIECE);
        let Ok(whole) = data.new_unique_out(join.size, piece) else {
            return;
        };
        let _ = data.op_set_input(piece, hi, 0);
        let _ = data.op_set_input(piece, lo, 1);
        data.op_insert_before(piece, op);
        whole
    };
    let _ = data.op_set_input(op, whole, lo_slot);
    let active = call.get_active_input();
    active.get_trial_mut(join.hi).mark_no_use();
    let t = active.get_trial_mut(join.lo);
    *t = crate::fspec::ParamTrial::new(join.addr.clone(), join.size, lo_slot);
    t.set_entry(Some(join.entry), 0);
    t.mark_active();
    t.mark_used();
}

/// The value `vn` as its low and high parts when it is a PIECE, else itself.
fn pieces(data: &Funcdata, vn: VarnodeId) -> Vec<Option<VarnodeId>> {
    let piece = data
        .vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .filter(|op| op.code() == OpCode::CPUI_PIECE);
    match piece {
        Some(op) if op.get_addr().is_big_endian() => vec![op.get_in(0), op.get_in(1)],
        Some(op) => vec![op.get_in(1), op.get_in(0)],
        None => vec![Some(vn)],
    }
}

/// Would passing `parts`, low to high, whole to `op` read more of the
/// function's entry registers than the function itself does? Not for values the
/// function computed, nor for entry registers it only forwards or also reads
/// whole elsewhere, so that the function takes them as one parameter anyway.
fn widens_entry(data: &Funcdata, parts: &[VarnodeId], op: crate::context::OpId) -> bool {
    let entry = |vn: VarnodeId| data.vbank().get(vn).is_some_and(|v| v.is_input());
    if !parts.iter().any(|&p| entry(p)) {
        return false;
    }
    if !parts.iter().all(|&p| entry(p)) {
        return true;
    }
    let readers = |vn: VarnodeId| -> Vec<crate::context::OpId> {
        data.descend_snapshot(vn).into_iter().filter(|&r| r != op).collect()
    };
    if parts.iter().all(|&p| readers(p).is_empty()) {
        return false;
    }
    let whole = match parts {
        [one] => {
            let size = data.vbank().get(*one).map_or(0, |v| v.get_size());
            readers(*one).into_iter().any(|r| {
                data.obank().get(r).is_some_and(|o| {
                    o.code() != OpCode::CPUI_SUBPIECE
                        || o.get_out()
                            .and_then(|out| data.vbank().get(out))
                            .is_some_and(|out| out.get_size() >= size)
                })
            })
        }
        [lo, hi] => readers(*lo).into_iter().any(|r| {
            data.obank()
                .get(r)
                .is_some_and(|o| (0..o.num_input()).any(|k| o.get_in(k) == Some(*hi)))
        }),
        _ => false,
    };
    !whole
}

/// Is `a` a later slot than `b` in the same register file?
fn above(a: &Address, b: &Address) -> bool {
    a.get_space().map(|s| s.get_index()) == b.get_space().map(|s| s.get_index())
        && a.get_offset() > b.get_offset()
}

/// A word of a stated floating parameter is not that parameter.
pub fn partial_float(data: &Funcdata, stated_size: i32, stated: &Datatype, size: i32) -> bool {
    data.get_arch().arm_float_args
        && size < stated_size
        && stated.get_metatype() == type_metatype::TYPE_FLOAT
}

fn single_entry(data: &Funcdata, addr: &Address) -> bool {
    data.get_func_proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT
                && e.get_size() == 4
                && e.justified_contain(addr, 4) == 0
        })
}

/// Replace an eight-byte VFP input by its two s-register words.
fn split_input(data: &mut Funcdata, vn: VarnodeId) -> Option<()> {
    let addr = data.vbank().get(vn)?.get_addr().clone();
    let high = &addr + 4;
    let (lo_addr, hi_addr) = if addr.is_big_endian() {
        (high, addr.clone())
    } else {
        (addr.clone(), high)
    };
    let block = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(block);
    let piece = data.new_op(2, start);
    data.op_set_opcode_code(piece, OpCode::CPUI_PIECE);
    let whole = data.new_varnode_out(8, &addr, piece).ok()?;
    data.op_insert_begin(piece, block);
    data.total_replace(vn, whole).ok()?;
    data.vbank_mut().destroy(vn).ok()?;
    let lo = data.new_varnode(4, &lo_addr, None);
    let lo = data.set_input_varnode(lo).ok()?;
    let hi = data.new_varnode(4, &hi_addr, None);
    let hi = data.set_input_varnode(hi).ok()?;
    data.op_set_input(piece, hi, 0).ok()?;
    data.op_set_input(piece, lo, 1).ok()?;
    read_words(data, whole, [lo, hi], 0, 4);
    if data.vbank().get(whole).is_some_and(|w| w.has_no_descend()) {
        data.op_destroy(piece);
    }
    Some(())
}

/// Point each word read of `vn` (shifted right by `shift` bytes) at its input.
fn read_words(data: &mut Funcdata, vn: VarnodeId, words: [VarnodeId; 2], shift: u64, depth: u32) {
    if depth == 0 {
        return;
    }
    for reader in data.descend_snapshot(vn) {
        let Some(op) = data.obank().get(reader) else {
            continue;
        };
        let Some(out) = op.get_out() else {
            continue;
        };
        let constant = op
            .get_in(1)
            .and_then(|c| data.vbank().get(c))
            .filter(|c| c.is_constant())
            .map(|c| c.get_offset());
        let out_size = data.vbank().get(out).map_or(0, |o| o.get_size());
        match (op.code(), constant) {
            (OpCode::CPUI_SUBPIECE, Some(offset)) if out_size == 4 => {
                let word = match offset + shift {
                    0 => words[0],
                    4 => words[1],
                    _ => continue,
                };
                data.op_remove_input(reader, 1);
                if data.op_set_input(reader, word, 0).is_ok() {
                    data.op_set_opcode_code(reader, OpCode::CPUI_COPY);
                }
                continue;
            }
            (OpCode::CPUI_INT_RIGHT, Some(32)) if shift == 0 && op.get_in(0) == Some(vn) => {
                read_words(data, out, words, 4, depth - 1);
            }
            (OpCode::CPUI_COPY | OpCode::CPUI_CAST, _) => {
                read_words(data, out, words, shift, depth - 1);
            }
            _ => continue,
        }
        if data.vbank().get(out).is_some_and(|o| o.has_no_descend()) {
            data.op_destroy(reader);
        }
    }
}

/// A d-register input read only as two words that floating operations consume
/// holds two s-register inputs. Words read only as integers stay one input: they
/// may be a double handed to an integer helper.
fn split_word_inputs(data: &mut Funcdata, typed: &[(Address, i32)]) {
    let floating = |addr: &Address| typed.iter().any(|(a, s)| *s == 4 && a == addr);
    let inputs: Vec<VarnodeId> = data
        .vbank()
        .iter_def_flag(varnode_flags::input)
        .filter(|&vn| {
            data.vbank().get(vn).is_some_and(|v| {
                let high = v.get_addr() + 4;
                v.get_size() == 8
                    && !v.has_no_descend()
                    && single_entry(data, v.get_addr())
                    && single_entry(data, &high)
                    && floating(v.get_addr())
                    && floating(&high)
            }) && crate::kuna_armfloatreturn::halves_only(data, vn, 4)
        })
        .collect();
    for vn in inputs {
        split_input(data, vn);
    }
}

/// A right shift by whole bytes: its shifted value and the byte count, so a
/// truncation of it is a truncation of that value further up.
fn byte_shift(data: &Funcdata, vn: VarnodeId) -> (VarnodeId, u64) {
    let shifted = data
        .vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|id| data.obank().get(id))
        .filter(|op| matches!(op.code(), OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT))
        .and_then(|op| {
            let amount = data.vbank().get(op.get_in(1)?)?;
            (amount.is_constant() && amount.get_offset() % 8 == 0)
                .then(|| (op.get_in(0), amount.get_offset() / 8))
        });
    match shifted {
        Some((Some(source), bytes)) => (source, bytes),
        _ => (vn, 0),
    }
}

fn input_storage(data: &Funcdata, vn: VarnodeId, budget: &mut usize) -> Option<(Address, i32)> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let value = data.vbank().get(vn)?;
    if value.is_input() {
        return Some((value.get_addr().clone(), value.get_size()));
    }
    let op = data.obank().get(value.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            let source = input_storage(data, op.get_in(0)?, budget)?;
            (source.1 == value.get_size()).then_some(source)
        }
        OpCode::CPUI_SUBPIECE => {
            let offset = data.vbank().get(op.get_in(1)?)?;
            if !offset.is_constant() {
                return None;
            }
            let (whole, shift) = byte_shift(data, op.get_in(0)?);
            let offset = offset.get_offset().checked_add(shift)?;
            let source = input_storage(data, whole, budget)?;
            if offset.checked_add(value.get_size() as u64)? > source.1 as u64 {
                return None;
            }
            let displacement = if source.0.is_big_endian() {
                source.1 as u64 - offset - value.get_size() as u64
            } else {
                offset
            };
            Some((&source.0 + displacement as i64, value.get_size()))
        }
        OpCode::CPUI_PIECE => {
            let hi = input_storage(data, op.get_in(0)?, budget)?;
            let lo = input_storage(data, op.get_in(1)?, budget)?;
            let (first, second) = if lo.0.is_big_endian() {
                (hi, lo)
            } else {
                (lo, hi)
            };
            (first.1 + second.1 == value.get_size() && &first.0 + i64::from(first.1) == second.0)
                .then_some((first.0, value.get_size()))
        }
        _ => None,
    }
}

/// Heritage can split an incoming d-register around an overlapping s-register result.
pub fn is_forwarded_input(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    data.get_arch().arm_float_args
        && size == 8
        && data.get_func_proto().has_model()
        && data
            .get_func_proto()
            .model()
            .input()
            .get_entry()
            .iter()
            .any(|e| {
                e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) == 0
            })
        && input_storage(data, vn, &mut 32).is_some_and(|(a, s)| a == *addr && s == size)
}

fn typed_inputs(data: &Funcdata) -> Vec<(Address, i32)> {
    let mut out = Vec::new();
    for op in data
        .obank()
        .iter_alive()
        .filter_map(|id| data.obank().get(id))
    {
        let floating = matches!(
            op.code(),
            OpCode::CPUI_FLOAT_EQUAL
                | OpCode::CPUI_FLOAT_NOTEQUAL
                | OpCode::CPUI_FLOAT_LESS
                | OpCode::CPUI_FLOAT_LESSEQUAL
                | OpCode::CPUI_FLOAT_NAN
                | OpCode::CPUI_FLOAT_ADD
                | OpCode::CPUI_FLOAT_DIV
                | OpCode::CPUI_FLOAT_MULT
                | OpCode::CPUI_FLOAT_SUB
                | OpCode::CPUI_FLOAT_NEG
                | OpCode::CPUI_FLOAT_ABS
                | OpCode::CPUI_FLOAT_SQRT
                | OpCode::CPUI_FLOAT_FLOAT2FLOAT
                | OpCode::CPUI_FLOAT_TRUNC
                | OpCode::CPUI_FLOAT_CEIL
                | OpCode::CPUI_FLOAT_FLOOR
                | OpCode::CPUI_FLOAT_ROUND
        );
        if floating {
            for slot in 0..op.num_input() {
                if let Some(storage) = op
                    .get_in(slot)
                    .and_then(|vn| input_storage(data, vn, &mut 32))
                {
                    out.push(storage);
                }
            }
        }
    }
    for i in 0..data.num_calls() {
        let call = data.get_call_specs(i);
        let Some(op) = data.obank().get(call.get_op()).filter(|o| !o.is_dead()) else {
            continue;
        };
        for slot in 1..op.num_input() {
            if let Some(storage) = op
                .get_in(slot)
                .and_then(|vn| input_storage(data, vn, &mut 32))
            {
                let declared = call.proto().get_param(slot - 1).is_some_and(|p| {
                    p.get_size() == storage.1
                        && p.get_type()
                            .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_FLOAT)
                });
                let passed = call
                    .final_input_storage()
                    .get((slot - 1) as usize)
                    .filter(|_| call.final_input_storage().len() as i32 + 1 == op.num_input())
                    .cloned()
                    .or_else(|| {
                        call.proto()
                            .get_param(slot - 1)
                            .map(|p| (p.get_address(), p.get_size()))
                    });
                let recovered = data
                    .kuna_protoorder_types(call.get_entry_address())
                    .is_some_and(|s| {
                        s.arity_sound
                            && s.inputs.iter().any(|(a, z, t)| {
                                *z == storage.1
                                    && t.get_metatype() == type_metatype::TYPE_FLOAT
                                    && (a == &storage.0
                                        || passed.as_ref().is_some_and(|(p, n)| a == p && *n == *z))
                            })
                    });
                if declared || recovered {
                    out.push(storage);
                }
            }
        }
    }
    out
}

/// Rejoin word-sized input pieces only when a floating reader consumes their whole value.
pub fn prepare_inputs(data: &mut Funcdata) {
    if !data.get_arch().arm_float_args
        || data.get_func_proto().is_input_locked()
        || !data.get_func_proto().has_model()
    {
        return;
    }
    let candidates = typed_inputs(data);
    split_word_inputs(data, &candidates);
    for (addr, size) in candidates {
        if !matches!(size, 4 | 8)
            || !data
                .get_func_proto()
                .model()
                .input()
                .get_entry()
                .iter()
                .any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT
                        && e.justified_contain(&addr, size) == 0
                })
        {
            continue;
        }
        let mut input = data.find_varnode_input(size, &addr);
        if input.is_none() && size == 8 {
            let high = &addr + 4;
            if let (Some(first), Some(second)) = (
                data.find_varnode_input(4, &addr),
                data.find_varnode_input(4, &high),
            ) {
                let (hi, lo) = if addr.is_big_endian() {
                    (first, second)
                } else {
                    (second, first)
                };
                if data.combine_input_varnodes(hi, lo).is_ok() {
                    input = data.find_varnode_input(size, &addr);
                }
            }
        }
        if let Some(vn) = input {
            if let Some(ty) = data
                .get_arch()
                .types()
                .and_then(|t| t.get_base(size, type_metatype::TYPE_FLOAT).ok())
            {
                data.vn_update_type(vn, ty);
            }
        }
    }
}

/// A resolved variadic format call states its parameters in base AAPCS storage.
pub fn base_inputs(arch: &Architecture, pieces: &mut PrototypePieces) -> bool {
    if !applies(arch) {
        return false;
    }
    let Some(model) = arch.get_model("__stdcall_softfp") else {
        return false;
    };
    let mut storage: Vec<ParameterPieces> = Vec::new();
    if model
        .assign_parameter_storage(pieces, &mut storage, true, arch.types(), arch.manage())
        .is_err()
        || storage.len() != pieces.intypes.len() + 1
    {
        return false;
    }
    for parameter in storage.iter_mut().skip(1) {
        if parameter
            .addr
            .get_space()
            .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_JOIN)
        {
            let Ok(join) = arch.manage().find_join(parameter.addr.get_offset()) else {
                return false;
            };
            if join.num_pieces() != 2 {
                return false;
            }
            let high = join.get_piece(0);
            let low = join.get_piece(1);
            let Some(space) = high
                .space
                .as_ref()
                .filter(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
            else {
                return false;
            };
            if low.space.as_ref().map(|s| s.get_index()) != Some(space.get_index())
                || high.size != 4
                || low.size != 4
            {
                return false;
            }
            let (first, second) = if space.is_big_endian() {
                (high, low)
            } else {
                (low, high)
            };
            if first.offset.checked_add(4) != Some(second.offset) {
                return false;
            }
            parameter.addr = Address::new(space.clone(), first.offset);
        }
        parameter.flags |= crate::fspec::parameter_pieces_flags::CUSTOM_STORAGE;
    }
    pieces.input_storage = storage
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(i, p)| (i as i32, p))
        .collect();
    true
}
