//! (kuna `armfloatargs`) An ARM hard-float double handed on whole in a core
//! register pair is a double parameter.
//!
//! `vmov r2,r3,d0` moves a double into the base convention, where a variadic
//! callee or a base-PCS helper takes it: `show2(int n, double d)` forwarding
//! `d` to `consume("%d %lf", n, d)` is `mov r1,r0; ...; vmov r2,r3,d0; b
//! consume`. Its p-code reads `d0` whole (`r3 = SUBPIECE(d0:8, 4); r2 = COPY
//! s0`), but heritage refines `d0` into the two words `s0` and `s1`, and copy
//! propagation leaves the call reading them as its `r2` and `r3` arguments.
//! Two floats moved one by one (`vmov r0,s0; vmov r1,s1`) end in the same IR,
//! so [`note_moves`] notes each doubleword move from the raw p-code, before
//! heritage runs. When the two words of a noted d register are read only as
//! the low and high registers of an aligned pair (`r0:r1`, `r2:r3`) of one
//! call, the registers that move wrote, the words are one double. The call
//! keeps passing its two words: whether the callee takes a double or a 64-bit
//! integer holding its bits is not known from the call. Little-endian only.

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

/// Are `lo` and `hi`, the low and high words of one d register, a double the
/// function hands on whole in a core register pair?
pub fn forwarded_words(data: &Funcdata, lo: VarnodeId, hi: VarnodeId) -> bool {
    let (Some(l), Some(h)) = (data.vbank().get(lo), data.vbank().get(hi)) else {
        return false;
    };
    let d = l.get_addr().clone();
    if l.get_size() != 4 || h.get_size() != 4 || &d + 4 != *h.get_addr() {
        return false;
    }
    let (Some((call, lo_slot)), Some((other, hi_slot))) = (only_reader(data, lo), only_reader(data, hi)) else {
        return false;
    };
    let (Some((r_lo, 4)), Some((r_hi, 4))) = (slot_storage(data, call, lo_slot), slot_storage(data, call, hi_slot)) else {
        return false;
    };
    call == other && &r_lo + 4 == r_hi && moved(data, call, &d, &r_lo)
}

/// Is the 8-byte input `vn` such a double, read only as its two words? That is
/// the shape [`forwarded_words`]' words take once `armfloatargs` joins them.
pub fn forwarded_double(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn).filter(|v| v.get_size() == 8 && v.is_input()) else {
        return false;
    };
    let mut words = [None, None];
    for id in v.descend_iter() {
        let Some(op) = data.obank().get(id).filter(|o| o.code() == OpCode::CPUI_SUBPIECE) else {
            return false;
        };
        let offset = op.get_in(1).and_then(|c| data.vbank().get(c)).filter(|c| c.is_constant()).map(|c| c.get_offset());
        let slot = match offset {
            Some(0) => 0,
            Some(4) => 1,
            _ => return false,
        };
        if words[slot].is_some() {
            return false;
        }
        words[slot] = op.get_out();
    }
    matches!(words, [Some(lo), Some(hi)] if forwarded_words(data, lo, hi))
}

/// Did a noted instruction, earlier in `call`'s block, move the d register at
/// `d` whole into the aligned core pair of `call` starting at `r_lo`?
fn moved(data: &Funcdata, call: OpId, d: &Address, r_lo: &Address) -> bool {
    let (Some(space), Some(op)) = (d.get_space(), data.obank().get(call)) else { return false };
    let Some(block) = op.get_parent() else { return false };
    let (start, at) = (data.bblocks_block_start(block).get_offset(), op.get_addr().get_offset());
    !d.is_big_endian()
        && d_register(data, d)
        && aligned(data, call, r_lo)
        && data
            .kuna_vfp_core_moves((space.get_index(), d.get_offset(), r_lo.get_offset()))
            .any(|insn| (start..at).contains(&insn))
}

fn d_register(data: &Funcdata, d: &Address) -> bool {
    let (Some(space), Some(lookup)) = (d.get_space(), data.get_arch().manage().register_lookup()) else {
        return false;
    };
    let name = lookup.get_register_name(space, d.get_offset(), 8);
    name.len() > 1 && name.starts_with('d') && name[1..].bytes().all(|b| b.is_ascii_digit())
}

/// The call that is `vn`'s only reader, and the one input slot it reads it in.
fn only_reader(data: &Funcdata, vn: VarnodeId) -> Option<(OpId, i32)> {
    let mut readers = data.vbank().get(vn)?.descend_iter();
    let id = readers.next()?;
    if readers.next().is_some() {
        return None;
    }
    let op = data.obank().get(id).filter(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))?;
    let mut slots = (1..op.num_input()).filter(|&s| op.get_in(s) == Some(vn));
    let slot = slots.next()?;
    slots.next().is_none().then_some((id, slot))
}

/// The storage `call` passes its input `slot` in.
fn slot_storage(data: &Funcdata, call: OpId, slot: i32) -> Option<(Address, i32)> {
    let fc = data.get_call_specs(data.get_call_specs_index(call)?);
    let finals = fc.final_input_storage();
    if finals.len() as i32 + 1 == data.obank().get(call)?.num_input() {
        return finals.get((slot - 1) as usize).cloned();
    }
    fc.proto().get_param(slot - 1).map(|p| (p.get_address(), p.get_size()))
}

/// Does an aligned core register pair of `call`'s model (`r0:r1`, `r2:r3`)
/// start at `r_lo`?
fn aligned(data: &Funcdata, call: OpId, r_lo: &Address) -> bool {
    let Some(fc) = data.get_call_specs_index(call).map(|i| data.get_call_specs(i)) else { return false };
    if !fc.proto().has_model() {
        return false;
    }
    let Some(space) = r_lo.get_space().map(|s| s.get_index()) else { return false };
    let mut cores: Vec<u64> = fc
        .proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_GENERAL && e.get_size() == 4)
        .filter(|e| e.get_space().get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
        .filter(|e| e.get_space().get_index() == space)
        .map(|e| e.get_base())
        .collect();
    cores.sort_unstable();
    cores.dedup();
    let lo = r_lo.get_offset();
    cores.first().is_some_and(|&first| lo >= first && (lo - first) % 8 == 0) && cores.contains(&lo) && cores.contains(&(lo + 4))
}

/// Note, before heritage refines the d register into its words, each
/// instruction whose p-code writes a core register's word pair from one d
/// register read whole: `r_hi = SUBPIECE(dN:8, 4)` beside `r_lo = COPY sN` or
/// `r_lo = SUBPIECE(dN:8, 0)`, `r_hi` four bytes above `r_lo`. The two-singles
/// `vmov rX,rY,sN,sM` reads the words apart and is not noted.
pub(crate) fn note_moves(data: &mut Funcdata) {
    if !data.get_arch().arm_float_args {
        return;
    }
    let storage = |vn: Option<VarnodeId>| {
        data.vbank().get(vn?).filter(|v| !v.is_constant()).map(|v| {
            let space = v.get_addr().get_space().map(|s| (s.get_index(), s.get_type(), s.is_big_endian()));
            (space, v.get_offset(), v.get_size())
        })
    };
    let constant = |vn: Option<VarnodeId>| data.vbank().get(vn?).filter(|c| c.is_constant()).map(|c| c.get_offset());
    let mut writes: Vec<(Address, (i32, u64), (i32, u64), u64)> = Vec::new();
    for id in data.obank().iter_alive() {
        let Some(op) = data.obank().get(id) else { continue };
        let (Some((Some((out_space, out_type, false)), out_off, 4)), Some((Some((in_space, _, false)), in_off, in_size))) =
            (storage(op.get_out()), storage(op.get_in(0)))
        else {
            continue;
        };
        if out_type != kuna_base::space::spacetype::IPTR_PROCESSOR {
            continue;
        }
        let word = match (op.code(), in_size, constant(op.get_in(1))) {
            (OpCode::CPUI_SUBPIECE, 8, Some(k @ (0 | 4))) => k,
            (OpCode::CPUI_COPY, 4, _) => 0,
            _ => continue,
        };
        let whole = if op.code() == OpCode::CPUI_SUBPIECE { 8 } else { 4 };
        writes.push((op.get_addr().clone(), (out_space, out_off), (in_space, in_off), word | whole << 8));
    }
    writes.sort_by_key(|w| w.0.get_offset());
    let mut found = Vec::new();
    for insn in writes.chunk_by(|a, b| a.0 == b.0) {
        for (at, (space, r_hi), (d_space, d), tag) in insn {
            if *tag != (4 | 8 << 8) || *r_hi < 4 {
                continue;
            }
            let low = insn.iter().any(|(_, (s, r), (ds, o), t)| {
                *s == *space && *r == r_hi - 4 && *ds == *d_space && *o == *d && (*t == (8 << 8) || *t == (4 << 8))
            });
            if low {
                found.push(((*d_space, *d, r_hi - 4), at.get_offset()));
            }
        }
    }
    for (key, at) in found {
        data.kuna_note_vfp_core_move(key, at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use std::rc::Rc;

    fn function() -> (Funcdata, Rc<AddrSpace>) {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 2, 0, 0, 0));
        let reg = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0));
        manager.insert_space(ram.clone()).unwrap();
        manager.insert_space(reg.clone()).unwrap();
        let mut arch = ArchContext::new(manager);
        arch.arm_float_args = true;
        let data = Funcdata::new("f", "f", Rc::new(arch), Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap();
        (data, reg)
    }

    /// An op at `at` writing the 4-byte `out` from `inputs`.
    fn write(data: &mut Funcdata, at: u64, code: OpCode, inputs: &[VarnodeId], out: &Address) {
        let pc = Address::new(data.get_address().get_space().unwrap().clone(), at);
        let op = data.new_op(inputs.len() as i32, pc);
        data.op_set_opcode_code(op, code);
        for (i, &vn) in inputs.iter().enumerate() {
            data.op_set_input(op, vn, i as i32).unwrap();
        }
        data.new_varnode_out(4, out, op).unwrap();
        data.obank_mut().mark_alive(op);
    }

    /// `vmov r2,r3,d0` is `r3 = SUBPIECE(d0:8, 4); r2 = COPY s0` at one address
    /// and is noted; the two-singles `vmov r2,r3,s0,s1` (two COPYs) and a move
    /// whose high register is not the word above the low one are not.
    #[test]
    fn only_a_doubleword_move_is_noted() {
        let (mut data, reg) = function();
        let (d0, s0, s1) = (Address::new(reg.clone(), 0x300), Address::new(reg.clone(), 0x300), Address::new(reg.clone(), 0x304));
        let (r2, r3, r5) = (Address::new(reg.clone(), 0x28), Address::new(reg.clone(), 0x2c), Address::new(reg.clone(), 0x34));
        let whole = data.new_varnode(8, &d0, None);
        let four = data.new_constant(4, 4);
        write(&mut data, 0x1008, OpCode::CPUI_SUBPIECE, &[whole, four], &r3);
        let low = data.new_varnode(4, &s0, None);
        write(&mut data, 0x1008, OpCode::CPUI_COPY, &[low], &r2);
        let low = data.new_varnode(4, &s0, None);
        write(&mut data, 0x1010, OpCode::CPUI_COPY, &[low], &r2);
        let high = data.new_varnode(4, &s1, None);
        write(&mut data, 0x1010, OpCode::CPUI_COPY, &[high], &r3);
        let whole = data.new_varnode(8, &d0, None);
        let four = data.new_constant(4, 4);
        write(&mut data, 0x1018, OpCode::CPUI_SUBPIECE, &[whole, four], &r5);
        let low = data.new_varnode(4, &s0, None);
        write(&mut data, 0x1018, OpCode::CPUI_COPY, &[low], &r2);
        note_moves(&mut data);
        let index = reg.get_index();
        assert_eq!(data.kuna_vfp_core_moves((index, 0x300, 0x28)).collect::<Vec<_>>(), [0x1008]);
        assert_eq!(data.kuna_vfp_core_moves_len(), 1);
    }
}
