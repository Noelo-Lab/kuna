//! (kuna) A narrow LOAD keeps its own width unless a declared record attests the
//! whole field it would be widened to.
//!
//! Upstream's `RuleExpandLoad` (`ruleaction.cc:10889`) rewrites a LOAD narrower
//! than its pointer's pointee as a LOAD of the whole pointee and takes the
//! loaded bytes back out: as a truncation (`SUBPIECE`), or inside the
//! `(V & C) == D` compares that are its only uses. Either way the printed C
//! reads bytes the program never reads. A record parameter whose 4-byte field at
//! `+8` is read whole on one path and by its low two bytes (`movzwl 8(%rdi)`) on
//! another printed the second read as `(unsigned short)a0->field_0x8`, and a
//! byte test of `+9` as `(a0->field_0x8 & 0x2000) != 0`; compiled, both read four
//! bytes and fault when the object the caller passes ends at `+10` at the end of
//! a page. Through a pointer a callee reads as `unsigned int *`, the two bytes
//! at `p + 4` printed `(unsigned short)a0[1]` and faulted the same way.
//!
//! The truncation form is never taken: the narrow read prints at its own width
//! and offset (`*(unsigned short *)&a0->field_0x8`) with the one cast the
//! truncation had. The AND form survives only through
//! [`widens_into_declared_field`], because there it prints a flag word's enum
//! names (`(p->flagfield & (HIGH_2|HIGH_1)) != 0`), which the byte the program
//! reads cannot carry, and the declaration attests the whole field.
//! A STORE is never widened: `TypeOpStore::getInputCast` casts the pointer of a
//! store narrower than its pointee (`((char *)&a0->field_0x8)[1] = b`).

use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;

/// Is `ptr` the address of a field of a record or union that a declaration
/// laid out (DWARF, a parsed header, a libc layout)? A bare pointer's target is
/// attested only at the bytes the program reads, even when declared, and a
/// record `structsynth` minted attests only the accesses it was built from.
pub fn widens_into_declared_field(data: &Funcdata, ptr: VarnodeId) -> bool {
    let Some(def) = data.vbank().get(ptr).and_then(|v| v.get_def()) else {
        return false;
    };
    let Some(op) = data.obank().get(def) else {
        return false;
    };
    if op.code() != OpCode::CPUI_PTRSUB {
        return false;
    }
    let Some(base) = op.get_in(0).and_then(|b| data.vbank().get(b)) else {
        return false;
    };
    let base_type = base.get_type_read_facing(def);
    let Some(record) = base_type.get_ptr_to() else {
        return false;
    };
    matches!(record.get_metatype(), type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION)
        && !crate::kuna_structsynth::points_at_synthesized_record(base_type)
}
