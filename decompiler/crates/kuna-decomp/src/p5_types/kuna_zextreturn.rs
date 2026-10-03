//! (kuna) A narrow value a function returns zero-extended is unsigned where the
//! calling convention extends a return by the sign of its type (P5).
//!
//! Dead-code analysis counts only the possibly-nonzero bits of a RETURN as
//! consumed, so sub-variable flow trims a return the function zero-extends to
//! the narrow value it extends: ARM `ldrh r0,[r1]; bx lr` returns the 2-byte
//! load, and `add r1,r1,#1; and r0,r1,#0xffff` the 2-byte sum. The type fold then
//! types that value by its own ops, and an addition or a byte of unknown type
//! reads as signed (`short`, `char`). C promotes a signed return value by
//! sign-extension, so a caller of `short Next(void)` computed `-32768 * 100 >> 16`
//! where the binary multiplies 32768.
//!
//! The trim records, on each RETURN it narrows, whether the bits it dropped were
//! known zero and the convention extends a value of that width by its type's
//! sign: `inttype` in the compiler spec (ARM, PowerPC, SPARC), or the rule
//! `narrowext` states (RISC-V and LoongArch below 32 bits, and MIPS and Apple
//! arm64 under `compiler`). There a zero-extended return is unsigned, or never
//! sets its sign bit. [`unsigned_return_vote`] offers the unsigned integer of the
//! value's width for it, where the fold says no more than a signed or unknown
//! integer and the value's sign bit may be set. Every RETURN must carry the
//! record, since the function returns one type. Conventions that leave the bits
//! above a narrow return unspecified (x86, AArch64) or always zero-extend it say
//! nothing about its sign, and keep the fold's type.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags::kuna_zextreturn;

/// Did trimming the RETURN input `vn` to the `size`-byte value under `mask`
/// drop only known-zero bits, where the convention extends that value by sign?
/// `storage` is a non-constant input of a RETURN, which names the register.
pub(crate) fn unsigned_trim(data: &Funcdata, vn: VarnodeId, storage: VarnodeId, mask: u64, size: i32) -> bool {
    if !crate::kuna_truncarg::drops_only_zero_bits(data, vn, mask) {
        return false;
    }
    let Some(v) = data.vbank().get(storage) else { return false };
    let addr = v.get_addr();
    let Some(space) = addr.get_space() else { return false };
    if v.is_constant() || addr.is_join() || size >= v.get_size() {
        return false;
    }
    let skip = if space.is_big_endian() { (v.get_size() - size) as u64 } else { 0 };
    let narrow = Address::new(Rc::clone(space), addr.get_offset() + skip);
    let proto = data.get_func_proto();
    let Some(list) = proto.has_model().then(|| proto.model().output_list()).flatten() else { return false };
    crate::kuna_narrowext::extends_by_sign(data.get_arch().narrow_ext.output, list, &narrow, size)
}

/// Record on the RETURN `op`, just trimmed, whether the trim was [`unsigned_trim`].
pub(crate) fn note_trimmed_return(data: &mut Funcdata, op: OpId, unsigned: bool) {
    let Some(o) = data.obank_mut().get_mut(op) else { return };
    if o.code() != OpCode::CPUI_RETURN {
        return;
    }
    if unsigned {
        o.set_additional_flag(kuna_zextreturn);
    } else {
        o.clear_additional_flag(kuna_zextreturn);
    }
}

/// The unsigned integer `vn` is, when every RETURN hands it back as an
/// [`unsigned_trim`] and `ct`, the fold's type, is a signed or unknown integer.
pub(crate) fn unsigned_return_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if !matches!(ct.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UNKNOWN) {
        return None;
    }
    if v.is_type_lock() || !(1..8).contains(&size) || v.get_nz_mask() >> (size * 8 - 1) & 1 == 0 {
        return None;
    }
    if data.get_func_proto().is_output_locked() {
        return None;
    }
    let returned = v.descend_iter().any(|op| {
        data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_RETURN && o.get_in(1) == Some(vn))
    });
    if !returned {
        return None;
    }
    let mut any = false;
    for op in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let o = data.obank().get(op)?;
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() < 2 {
            continue;
        }
        let width = o.get_in(1).and_then(|r| data.vbank().get(r)).map(|r| r.get_size());
        if o.get_addlflags() & kuna_zextreturn == 0 || width != Some(size) {
            return None;
        }
        any = true;
    }
    if !any {
        return None;
    }
    data.get_arch().types()?.get_base(size, type_metatype::TYPE_UINT).ok()
}
