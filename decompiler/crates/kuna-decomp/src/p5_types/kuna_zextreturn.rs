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
//! nothing about its sign, and keep the fold's type. Where the rule
//! sign-extends a 32-bit return whatever its sign (RISC-V and LoongArch LP64), a
//! zero-extended word whose sign bit may be set is no 32-bit value at all, and
//! [`zero_extended_word`] makes the trim keep the whole register. Under the
//! other conventions only the callers can tell: one that computes with more of
//! the register relies on the zero-extension, and `voidret` has the function
//! decompiled again, keeping the whole register for a caller computing in 64
//! bits and otherwise typing the narrow value unsigned.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::ParamListStandard;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags::kuna_zextreturn;

/// Did trimming the RETURN input `vn` to the `size`-byte value under `mask`
/// drop only known-zero bits, where the convention extends that value by sign,
/// or where the function's callers compute with up to four bytes of the
/// register ([`read_width`])? `storage` is a non-constant input of a RETURN,
/// which names the register.
pub(crate) fn unsigned_trim(data: &Funcdata, vn: VarnodeId, storage: VarnodeId, mask: u64, size: i32) -> bool {
    if !crate::kuna_truncarg::drops_only_zero_bits(data, vn, mask) {
        return false;
    }
    let Some((narrow, list)) = narrow_output(data, storage, size) else { return false };
    crate::kuna_narrowext::extends_by_sign(data.get_arch().narrow_ext.output, list, &narrow, size)
        || read_width(data, &narrow, size).is_some_and(|w| w <= 4)
}

/// Would trimming the RETURN input `vn` to the `size`-byte value under `mask`
/// drop a zero-extension no narrow type extends as the binary does? Where the
/// `narrowext` rule sign-extends a return of that width whatever its type (a
/// 32-bit value on RISC-V and LoongArch LP64), one whose sign bit may be set.
/// Where the convention extends it neither so nor by its type's sign (x86-64,
/// AArch64), one whose callers compute with more than four bytes of the
/// register ([`read_width`]), whatever its sign: C would compute with the
/// narrow type where the binary computes in 64 bits. The RETURN must then keep
/// the whole register. Such a trim that no caller has yet computed with wider
/// is noted on `data`, so that `voidret` can ask the callers.
pub(crate) fn zero_extended_word(data: &mut Funcdata, vn: VarnodeId, storage: VarnodeId, mask: u64, size: i32) -> bool {
    if !(1..8).contains(&size) || !crate::kuna_truncarg::drops_only_zero_bits(data, vn, mask) {
        return false;
    }
    let sign = data.vbank().get(vn).is_some_and(|v| v.get_nz_mask() >> (size * 8 - 1) & 1 != 0);
    let rule = data.get_arch().narrow_ext.output;
    let Some((narrow, any, by_type)) = narrow_output(data, storage, size).map(|(narrow, list)| {
        let any = crate::kuna_narrowext::sign_extends_any(rule, list, &narrow, size);
        let by_type = crate::kuna_narrowext::extends_by_sign(rule, list, &narrow, size);
        (narrow, any, by_type)
    }) else {
        return false;
    };
    if any || by_type {
        return any && sign;
    }
    if read_width(data, &narrow, size).is_some_and(|w| w > 4) {
        return true;
    }
    data.kuna_note_zext_word(narrow, size, sign);
    false
}

/// How many bytes of the register holding the `size`-byte value at `narrow`
/// the function's callers compute with, when more than the value
/// ([`Funcdata::kuna_wide_return`]). Up to four, C's promotion of an unsigned
/// narrow type to `int` extends it as the binary does; beyond, only a 64-bit
/// type does.
fn read_width(data: &Funcdata, narrow: &Address, size: i32) -> Option<i32> {
    data.kuna_wide_return().filter(|(addr, width)| wider_over(addr, *width, narrow, size)).map(|(_, width)| *width)
}

/// Does `[addr, addr+width)` hold `[narrow, narrow+size)` and more?
pub(crate) fn wider_over(addr: &Address, width: i32, narrow: &Address, size: i32) -> bool {
    let (Some(a), Some(n)) = (addr.get_space(), narrow.get_space()) else { return false };
    a.get_index() == n.get_index()
        && width > size
        && addr.get_offset() <= narrow.get_offset()
        && narrow.get_offset() + size as u64 <= addr.get_offset() + width as u64
}

/// The low `size` bytes of the non-constant RETURN input `storage`, and the
/// prototype's output list they are read against.
fn narrow_output(data: &Funcdata, storage: VarnodeId, size: i32) -> Option<(Address, &ParamListStandard)> {
    let v = data.vbank().get(storage)?;
    let addr = v.get_addr();
    let space = addr.get_space()?;
    if v.is_constant() || addr.is_join() || size >= v.get_size() {
        return None;
    }
    let skip = if space.is_big_endian() { (v.get_size() - size) as u64 } else { 0 };
    let narrow = Address::new(Rc::clone(space), addr.get_offset() + skip);
    let proto = data.get_func_proto();
    let list = proto.has_model().then(|| proto.model().output_list()).flatten()?;
    Some((narrow, list))
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
/// Also when the function keeps the whole register for callers that compute
/// with it ([`zero_extended_word`]) and `vn`, returned in it, is a word its
/// upper half leaves zero.
pub(crate) fn unsigned_return_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if !matches!(ct.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UNKNOWN) {
        return None;
    }
    if v.is_type_lock() || data.get_func_proto().is_output_locked() {
        return None;
    }
    let returned = v.descend_iter().any(|op| {
        data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_RETURN && o.get_in(1) == Some(vn))
    });
    if !returned {
        return None;
    }
    let whole = data.kuna_wide_return().is_some_and(|(addr, width)| *width == size && addr == v.get_addr());
    if whole && size > 4 && v.get_nz_mask() >> 32 == 0 {
        return data.get_arch().types()?.get_base(size, type_metatype::TYPE_UINT).ok();
    }
    if !(1..8).contains(&size) || v.get_nz_mask() >> (size * 8 - 1) & 1 == 0 {
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
