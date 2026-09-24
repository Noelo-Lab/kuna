//! (kuna `castsign`) Declare a variable signed when the program only ever reads it
//! as signed, including the stack-frame locals and the readers the `signedness`
//! walk leaves alone.
//!
//! `signedness auto` ([`crate::kuna_typeround`]) already re-declares a register
//! local `int` when every signedness-sensitive reader of it is signed.  Three
//! limits of that walk leave the same cast in place elsewhere:
//!
//! ```text
//!   unsigned long v15; // stack - 0x60
//!   v15 = read(a0,v17,v11);
//!   if (0 <= (long)v15) {
//! ```
//!
//! - **A local that lives in the frame is never considered.**  Every high mapped
//!   onto a Symbol is skipped, and a stack local is mapped onto its frame Symbol
//!   at offset 0.  This option admits a high that covers its Symbol whole: offset
//!   0, a Symbol type (if any) that is a plain integer of the declaration's width,
//!   not a parameter, and no other high in the function printing the same name.
//!   The last condition is what keeps an address-taken local out: `&v15` is a
//!   high of its own bound to the same name, and re-signing the declaration would
//!   change the C type of `&v15` under a pointer that was typed for the old one.
//! - **A same-width conversion to a pointer vetoes.**  `(char *)v` converts the
//!   same bits whichever signedness `v` is declared with when the integer and the
//!   pointer are the same width (gcc and clang both define it that way), so it is
//!   neutral here.
//! - **A pointer index vetoes.**  `p[v]` with a pointer-width `v` computes
//!   `p + v * size` modulo the address width under either declaration; the signed
//!   reading is the one that stays inside C's defined behaviour for a negative
//!   index, so it is neutral for a flip to signed.
//!
//! Two uses that do not change what C computes still say what the value is, and
//! this option counts them as unsigned evidence so that a flip never goes against
//! a declared type: a direct call argument whose parameter is type-locked (a
//! declared or measured prototype) as an unsigned integer of the same width, and
//! a value stored through a pointer to an unsigned integer of the same width.
//!
//! **The option only ever declares a variable signed.**  Every relaxation above
//! is sound for the unsigned direction too except the pointer index, but IDA's
//! census shows the opposite bias costs as many casts as it saves, so a verdict
//! that would need any of these relaxations to declare something unsigned is left
//! exactly as `signedness` decided it.  With the option off, `signedness` is
//! byte-identical to what it was.

use std::collections::HashMap;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// How many highs in `fd` print each name.
pub(crate) fn name_counts(fd: &Funcdata) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    for (_, h) in fd.high_bank().iter() {
        if let Some(n) = h.kuna_name() {
            *out.entry(n.to_string()).or_default() += 1;
        }
    }
    out
}

/// Is `high` a frame local that covers its Symbol whole, with a plain integer
/// Symbol type of `width`, and the only high printing its name?
pub(crate) fn whole_slot_local(
    fd: &Funcdata,
    high: HighVariableId,
    width: int4,
    names: &HashMap<String, u32>,
    is_plain_integer: impl Fn(&Datatype) -> bool,
) -> bool {
    let Some(h) = fd.high_bank().get(high) else { return false };
    if h.kuna_symbol_offset() != 0 {
        return false;
    }
    if h.kuna_symbol_type().is_some_and(|t| !is_plain_integer(t) || t.get_size() != width) {
        return false;
    }
    h.kuna_name().is_some_and(|n| names.get(n).copied() == Some(1))
}

/// A reader `signedness` vetoes that this option takes as neutral for a flip to
/// signed: a same-width conversion to a pointer, or the index of a pointer-width
/// `PTRADD`.
pub(crate) fn neutral_reader(fd: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    let Some(size) = fd.vbank().get(vn).map(|v| v.get_size()) else { return false };
    match o.code() {
        OpCode::CPUI_CAST => o
            .get_out()
            .and_then(|out| fd.vbank().get(out))
            .is_some_and(|v| {
                let t = v.get_type_def_facing();
                t.get_metatype() == type_metatype::TYPE_PTR && t.get_size() == size
            }),
        OpCode::CPUI_PTRADD => {
            o.get_in(1) == Some(vn)
                && o.get_in(0) != Some(vn)
                && o.get_in(0)
                    .and_then(|b| fd.vbank().get(b))
                    .is_some_and(|b| b.get_size() == size)
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "kuna_castsign/tests.rs"]
mod tests;
