//! (kuna `castsign`) Declare a variable signed when the program only ever reads it
//! signed, in the places the `signedness` walk leaves alone.
//!
//! `signedness auto` ([`crate::kuna_typeround`]) already re-declares a register
//! local `int` when every signedness-sensitive reader of it is signed.  Four
//! limits of that walk leave the same cast in place elsewhere:
//!
//! ```text
//!   unsigned long v1; // stack - 0x10
//!   v1 = strlen(a0);
//!   while ((v1 = v1 - 1, 0 <= (long)v1 && (a0[v1] == ' '))) {
//! ```
//!
//! - **A frame local is never considered.**  Every high mapped onto a Symbol is
//!   skipped, and a stack local is mapped onto its frame Symbol at offset 0.  This
//!   option admits a high that covers its Symbol whole ([`whole_slot_local`]):
//!   offset 0, a Symbol type (if any) that is a plain integer of the declaration's
//!   width, and no other high in the function printing the same name.  The last
//!   condition keeps an address-taken local out: `&v1` is a high of its own bound
//!   to the same name, and re-signing the declaration would change the C type of
//!   `&v1` under a pointer typed for the old one.
//! - **A pointer index vetoes.**  `p[v]` with a pointer-width `v` computes
//!   `p + v * size` modulo the address width under either declaration, and the
//!   signed reading keeps a negative index defined ([`neutral_reader`]).
//! - **A same-width conversion to a pointer vetoes.**  gcc and clang convert an
//!   integer to a pointer of its own width by keeping the bits.
//! - **A body local with an input member is skipped** as if it were a parameter.
//!   The printer declares only body locals, and `SignPlan::retain_sole_named`
//!   drops every planned high it does not declare, signature parameters included.
//!
//! **The option only ever declares a variable signed.**  A decision any of these
//! relaxations made is kept only when it declares the value `int`: the census
//! counts 20 signed-to-unsigned casts on locals against 526 the other way, and
//! IDA's opposite bias costs it 587.  A type-locked unsigned parameter and a store
//! through an unsigned pointer are deliberately not unsigned evidence: a
//! same-width conversion computes the same bits, gnulib's signed `idx_t` reaches
//! `size_t` parameters everywhere, and a stored-through pointer's pointee is
//! usually typed from the stored value itself.  With the option off,
//! `signedness` is byte-identical to what it was.

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
