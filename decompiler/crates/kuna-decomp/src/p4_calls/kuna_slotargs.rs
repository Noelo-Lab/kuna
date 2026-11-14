//! (kuna) The declaration a call through an import slot takes before heritage.
//!
//! x86's `call [slot]` reads its target into a temporary its own instruction
//! fills from the slot, so `ActionDefaultParams` finds the slot through that
//! instruction's load and gives the call the pieces a library table parked
//! there (`win32sigs`, the libc tables). Two limits keep that declaration to
//! what the caller's own code shows.
//!
//! * Only the parameters are locked ([`input_only`]). The result stays the one
//!   the caller's use recovers, as it was before the declaration reached the
//!   call: a locked result would make a function that returns `al` beside an
//!   import's pointer in the rest of `rax` return all of `rax`, and a 4-byte one
//!   would leave the upper half of a returned `rax` an unwritten register. A
//!   declared pointer result is still the type of a recovered result of its
//!   width ([`declared_result_type`]), as a vote, so `LoadLibraryW(a0)` needs
//!   no cast to the `void *` its uses give the handle.
//! * Every declared stack parameter has to be a slot the caller wrote for this
//!   call ([`backed`]). The walk runs back from the call over the raw p-code,
//!   through blocks entered from one predecessor, tracking the stack pointer:
//!   a push or a store through the stack pointer writes a slot, a call whose
//!   pop is stated (or a direct callee whose own returns pop nothing, which is
//!   also the solver's guess for it) moves the pointer by that pop and owns the
//!   slots it pops, a cleanup owns the slots it frees, and a call whose pop is
//!   a guess, any other write of the stack pointer or a join ends it. A walk
//!   that reaches the function's entry knows the slots from the return address
//!   up are the caller's, and that a callee-saved register pushed there before
//!   anything wrote it is a saved register, not an argument. A call
//!   pushing two of `VirtualAlloc`'s four arguments keeps the two it pushes, and
//!   `CompareStringW(LocaleNameToLCID(...), ...)`, whose inner callee's pop is
//!   only guessed, keeps its recovered arguments rather than reading the inner
//!   call's pushes as its own. A jump to the slot passes the function's own
//!   arguments and is not walked.

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::{FuncCallSpecs, FuncProto, PrototypePieces, EXTRAPOP_UNKNOWN};
use crate::funcdata::Funcdata;

const MAX_OPS: usize = 2048;
const MAX_BLOCKS: usize = 32;
const MAX_SLOT_BYTES: i64 = 512;

/// `pieces` with the result left to recovery.
pub fn input_only(mut pieces: PrototypePieces) -> PrototypePieces {
    pieces.outtype = None;
    pieces.output_storage = None;
    pieces
}

/// The pointer a library-table declaration says the call `op` returns, when
/// the call took only that declaration's parameters and recovered a register
/// result of the pointer's width.
pub fn declared_result_type(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, size: int4) -> Option<Rc<Datatype>> {
    let proto = fc.proto();
    if !proto.is_input_locked() || proto.is_output_locked() {
        return None;
    }
    let o = data.obank().get(op)?;
    let out = data.vbank().get(o.get_out()?)?;
    if o.code() != OpCode::CPUI_CALL || out.get_size() != size || out.get_space().get_type() != spacetype::IPTR_PROCESSOR {
        return None;
    }
    let arch = data.get_arch();
    let entry = fc.get_entry_address();
    arch.callee_library_names(entry)?;
    let ct = arch.callee_proto_pieces(entry)?.outtype?;
    (ct.get_metatype() == type_metatype::TYPE_PTR && ct.get_size() == size).then_some(ct)
}

struct Walk {
    sp_space: int4,
    sp_offset: u64,
    sp_size: int4,
    written: Vec<bool>,
    owned: Vec<bool>,
    saves: Vec<(i64, int4, Address, int4)>,
}

impl Walk {
    fn is_sp(&self, data: &Funcdata, vn: VarnodeId) -> bool {
        data.vbank().get(vn).is_some_and(|v| {
            v.get_space().get_index() == self.sp_space && v.get_offset() == self.sp_offset && v.get_size() == self.sp_size
        })
    }

    fn mark(&mut self, at: i64, size: int4, write: bool) -> bool {
        let mut fresh = false;
        for b in at.max(0)..(at + size as i64).min(self.written.len() as i64) {
            let b = b as usize;
            if !self.written[b] && !self.owned[b] {
                fresh = true;
                if write {
                    self.written[b] = true;
                } else {
                    self.owned[b] = true;
                }
            }
        }
        fresh
    }
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.get_space().map(|s| s.get_index()) == b.get_space().map(|s| s.get_index())
        && a.get_offset() < b.get_offset() + bsize as u64
        && b.get_offset() < a.get_offset() + asize as u64
}

fn signed(space: &AddrSpace, value: u64) -> i64 {
    let bits = (space.get_addr_size() as u32 * 8).min(64);
    if bits >= 64 {
        return value as i64;
    }
    let sign = 1u64 << (bits - 1);
    let mask = (1u64 << bits) - 1;
    let v = value & mask;
    if v & sign != 0 { (v | !mask) as i64 } else { v as i64 }
}

/// The constant `c` of `sp <op> c` when `base` is the stack pointer.
fn sp_offset(data: &Funcdata, walk: &Walk, stack: &AddrSpace, base: VarnodeId, c: VarnodeId) -> Option<i64> {
    if !walk.is_sp(data, base) {
        return None;
    }
    let c = data.vbank().get(c)?;
    c.is_constant().then(|| signed(stack, c.get_offset()))
}

/// The op that writes the temporary `vn` reads, within `op`'s instruction.
fn def_in_instruction(data: &Funcdata, op: OpId, vn: VarnodeId) -> Option<OpId> {
    let at = data.obank().get(op)?.get_addr().clone();
    let target = data.vbank().get(vn)?;
    let (addr, size) = (target.get_addr().clone(), target.get_size());
    let mut cur = op;
    loop {
        if data.obank().get(cur)?.is_instruction_start() {
            return None;
        }
        cur = data.obank().get(cur)?.basic_neighbours().0?;
        let prev = data.obank().get(cur)?;
        if prev.get_addr() != &at {
            return None;
        }
        let writes = prev.get_out().and_then(|o| data.vbank().get(o)).is_some_and(|o| o.get_addr() == &addr && o.get_size() == size);
        if writes {
            return Some(cur);
        }
    }
}

/// The register a stored value is a plain copy of, within the instruction.
fn stored_register(data: &Funcdata, op: OpId, value: VarnodeId) -> Option<(Address, int4)> {
    let v = data.vbank().get(value)?;
    let src = match v.get_space().get_type() {
        spacetype::IPTR_PROCESSOR => v,
        spacetype::IPTR_INTERNAL => {
            let def = data.obank().get(def_in_instruction(data, op, value)?)?;
            if def.code() != OpCode::CPUI_COPY {
                return None;
            }
            data.vbank().get(def.get_in(0)?)?
        }
        _ => return None,
    };
    (src.get_space().get_type() == spacetype::IPTR_PROCESSOR).then(|| (src.get_addr().clone(), src.get_size()))
}

/// Whether every stack parameter of `proto`, placed at the call `call_op`, is a
/// slot the caller writes for this call.
pub fn backed(data: &Funcdata, call_op: OpId, proto: &FuncProto) -> bool {
    let Some(stack) = data.get_arch().manage().get_stack_space().cloned() else { return true };
    let Some(call) = data.obank().get(call_op) else { return false };
    let call_addr: Address = call.get_addr().clone();
    let mut need: Vec<(i64, int4)> = Vec::new();
    for j in 0..proto.num_params() {
        let Some(param) = proto.get_param(j) else { continue };
        let addr = param.get_address();
        if addr.get_space().map(|s| s.get_index()) != Some(stack.get_index()) {
            continue;
        }
        need.push((signed(&stack, addr.get_offset()), param.get_size()));
    }
    if need.is_empty() {
        return true;
    }
    let Some(ret) = crate::p6_variables::kuna_calleepop::return_address_bytes(data, &stack, &call_addr) else {
        return true;
    };
    let need: Vec<(i64, int4)> = need.into_iter().map(|(off, size)| (off - ret as i64, size)).collect();
    let top = need.iter().map(|&(off, size)| off + size as i64).max().unwrap_or(0);
    if need.iter().any(|&(off, _)| off < 0) || top > MAX_SLOT_BYTES {
        return false;
    }
    let Ok(sp) = stack.get_spacebase(0) else { return true };
    let Some(sp_space) = sp.space.as_ref() else { return true };
    let mut walk = Walk {
        sp_space: sp_space.get_index(),
        sp_offset: sp.offset,
        sp_size: sp.size as int4,
        written: vec![false; top as usize],
        owned: vec![false; top as usize],
        saves: Vec::new(),
    };

    let mut first = call_op;
    while let Some(prev) = data.obank().get(first).and_then(|o| o.basic_neighbours().0) {
        if data.obank().get(prev).map(|o| o.get_addr()) != Some(&call_addr) {
            break;
        }
        first = prev;
    }
    let mut block = call.get_parent();
    let mut cur = data.obank().get(first).and_then(|o| o.basic_neighbours().0);
    let mut p: i64 = 0;
    let mut entry: Option<i64> = None;
    let (mut ops, mut blocks) = (0usize, 0usize);
    loop {
        let Some(op_id) = cur else {
            let Some(bl) = block else { break };
            let graph = data.bblocks_ref();
            let fb = graph.block(bl);
            if fb.size_in() == 0 {
                if fb.is_entry_point() {
                    entry = Some(p);
                }
                break;
            }
            blocks += 1;
            if fb.size_in() != 1 || blocks > MAX_BLOCKS {
                break;
            }
            let pred = fb.get_in(0);
            block = Some(pred);
            cur = data.bb_op_tail(pred);
            continue;
        };
        ops += 1;
        if ops > MAX_OPS {
            break;
        }
        let Some(op) = data.obank().get(op_id) else { break };
        cur = op.basic_neighbours().0;
        match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let Some(k) = data.get_call_specs_index(op_id) else { break };
                let fc = data.get_call_specs(k);
                let mut pop = fc.proto().get_extra_pop();
                if !fc.proto().has_model() {
                    break;
                }
                if pop == EXTRAPOP_UNKNOWN && op.code() == OpCode::CPUI_CALL && fc.callee_purge() == Some(ret) {
                    pop = ret;
                }
                if pop == EXTRAPOP_UNKNOWN {
                    break;
                }
                let before = p - pop as i64;
                walk.mark(before, (p - before) as int4, false);
                p = before;
                continue;
            }
            OpCode::CPUI_STORE => {
                let (Some(ptr), Some(value)) = (op.get_in(1), op.get_in(2)) else { continue };
                let size = data.vbank().get(value).map(|v| v.get_size()).unwrap_or(0);
                let slot = if walk.is_sp(data, ptr) {
                    Some(p)
                } else if data.vbank().get(ptr).is_some_and(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL) {
                    def_in_instruction(data, op_id, ptr).and_then(|d| {
                        let def = data.obank().get(d)?;
                        match def.code() {
                            OpCode::CPUI_COPY => walk.is_sp(data, def.get_in(0)?).then_some(p),
                            OpCode::CPUI_INT_ADD => sp_offset(data, &walk, &stack, def.get_in(0)?, def.get_in(1)?)
                                .or_else(|| sp_offset(data, &walk, &stack, def.get_in(1)?, def.get_in(0)?))
                                .map(|c| p + c),
                            _ => None,
                        }
                    })
                } else {
                    None
                };
                if let Some(slot) = slot {
                    if walk.mark(slot, size, true) {
                        if let Some((reg, reg_size)) = stored_register(data, op_id, value) {
                            if proto.has_effect(&reg, reg_size) == crate::fspec::effect_type::UNAFFECTED {
                                walk.saves.push((slot, size, reg, reg_size));
                            }
                        }
                    }
                }
                continue;
            }
            _ => {}
        }
        let Some(out) = op.get_out() else { continue };
        if walk.is_sp(data, out) {
            let (Some(a), Some(b)) = (op.get_in(0), op.get_in(1)) else { break };
            match op.code() {
                OpCode::CPUI_INT_SUB => match sp_offset(data, &walk, &stack, a, b) {
                    Some(c) => p += c,
                    None => break,
                },
                OpCode::CPUI_INT_ADD => {
                    match sp_offset(data, &walk, &stack, a, b).or_else(|| sp_offset(data, &walk, &stack, b, a)) {
                        Some(c) => {
                            if c > 0 {
                                walk.mark(p - c, c as int4, false);
                            }
                            p -= c;
                        }
                        None => break,
                    }
                }
                _ => break,
            }
            continue;
        }
        if let Some(o) = data.vbank().get(out) {
            if o.get_space().get_type() == spacetype::IPTR_PROCESSOR && !walk.saves.is_empty() {
                let (written, wsize) = (o.get_addr().clone(), o.get_size());
                walk.saves.retain(|(_, _, reg, size)| !overlaps(&written, wsize, reg, *size));
            }
        }
    }
    if let Some(at_entry) = entry {
        for &(slot, size, _, _) in &walk.saves {
            for b in slot.max(0)..(slot + size as i64).min(top) {
                walk.written[b as usize] = false;
            }
        }
        for b in at_entry.max(0)..top {
            walk.written[b as usize] = false;
        }
    }
    need.iter().all(|&(off, size)| (off..off + size as i64).all(|b| walk.written[b as usize]))
}
