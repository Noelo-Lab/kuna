//! (kuna `castsign`) Declare a variable signed when the program only ever reads it
//! signed, in the places the `signedness` walk leaves alone.
//!
//! `signedness auto` ([`crate::kuna_typeround`]) already re-declares a register
//! local `int` when every signedness-sensitive reader of it is signed.  Four
//! limits of that walk leave the same cast in place elsewhere:
//!
//! ```text
//!   unsigned long v1; // stack - 0x10
//!   v1 = strtoul(a0,NULL,0);
//!   if ((long)v1 <= -1)
//!     v1 = strtoul(a1,NULL,0);
//!   if (0 <= (long)v1)
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
//! **Arithmetic keeps the declaration unsigned** ([`can_overflow`]).  Declaring
//! `v` signed changes `v + 1`, `v - 1`, `v * k`, `-v` and `v << k` from unsigned
//! arithmetic, which wraps, to signed arithmetic, which is undefined on overflow,
//! and gcc and clang fold on that: `0 <= (long)(v - 1)` over a `long v` becomes
//! `0 < v`, and `v = v - 1, 0 <= v` exits at once for `v == LONG_MIN`, where the
//! binary loops.  So a high this option admits is left alone when any of those
//! operators reads it, directly or through the expression its value is printed
//! into.  (The walk `signedness` runs for register locals accepts that trade; this
//! option does not extend it.)
//!
//! **A flip must remove a cast** ([`drops_printed_cast`]).  A high this option
//! admits is re-declared only when a `(T)v` the new declaration makes a no-op is
//! printed today: its value lands in an expression, not in a call argument, an
//! assignment or a `return`, where `castimplied` already leaves it out.
//!
//! **A type-locked declaration is never touched** ([`symbol_type_locked`]).  A
//! `--assert type`, a DWARF local and a type committed from Ghidra lock the
//! Symbol, not the member varnodes the walk checks, so the lock is read on the
//! Symbol.
//!
//! **The option only ever declares a variable signed.**  A decision any of these
//! relaxations made is kept only when it declares the value `int`: the census
//! counts 20 signed-to-unsigned casts on locals against 526 the other way, and
//! IDA's opposite bias costs it 587.  A type-locked unsigned parameter and a store
//! through an unsigned pointer are deliberately not unsigned evidence: a
//! same-width conversion computes the same bits, gnulib's signed `idx_t` reaches
//! `size_t` parameters everywhere, and a stored-through pointer's pointee is
//! usually typed from the stored value itself.  C output only: Rust has no
//! implicit integer conversions, so a re-declared local would not accept its
//! unsigned definition.  With the option off, `signedness` is byte-identical to
//! what it was.

use std::collections::HashMap;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_base::address::Address;
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

/// Is `high` bound to a Symbol whose type is locked: a `--assert type`, a DWARF
/// local, or a type the host committed?  The lock lives on the Symbol, not on
/// the member varnodes the evidence walk checks, so it is read through the
/// naming pass's bind, a dynamic Symbol, and the Symbol containing each
/// address-tied member.
pub(crate) fn symbol_type_locked(fd: &Funcdata, high: HighVariableId) -> bool {
    let Some(h) = fd.high_bank().get(high) else { return true };
    let Some(lm) = fd.get_scope_local() else { return false };
    let db = lm.database();
    if [h.kuna_link_symbol(), h.kuna_dynamic_symbol()]
        .into_iter()
        .flatten()
        .any(|sid| db.symbol(sid).is_type_locked())
    {
        return true;
    }
    let invalid = Address::new_invalid();
    (0..h.num_instances()).any(|i| {
        fd.vbank()
            .get(h.get_instance(i))
            .filter(|v| !v.is_free() && v.is_addr_tied())
            .and_then(|v| lm.container_symbol_link(v.get_addr(), &invalid))
            .is_some_and(|(sid, ..)| db.symbol(sid).is_type_locked())
    })
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

/// Can `opc` overflow when its operand at `slot` is signed: `+ - *`, unary `-`,
/// and the shiftee of `<<`?  Over an unsigned operand each of these wraps, which
/// is defined; over a signed one each is undefined on overflow.
pub(crate) fn can_overflow(opc: OpCode, slot: int4) -> bool {
    match opc {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_2COMP => true,
        OpCode::CPUI_INT_LEFT => slot == 0,
        _ => false,
    }
}

/// Is one of `casts` (each a `CPUI_CAST` reading a declared member of the high)
/// a `(newty)` cast that prints today and that declaring the high `newty` makes a
/// no-op?  The cast prints when its value is inlined into an expression: a
/// value that only feeds a call argument, an assignment, a `return`, a store or
/// another conversion may already be left out by `castimplied`.
pub(crate) fn drops_printed_cast(fd: &Funcdata, casts: &[OpId], newty: &Datatype) -> bool {
    casts.iter().any(|&op| {
        let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()) else { return false };
        let Some(v) = fd.vbank().get(out) else { return false };
        let t = v.get_type_def_facing();
        if t.get_size() != newty.get_size()
            || t.get_metatype() != newty.get_metatype()
            || t.get_sub_meta() != newty.get_sub_meta()
            || t.get_name() != newty.get_name()
            || v.is_explicit()
        {
            return false;
        }
        v.descend_iter().any(|r| {
            fd.obank().get(r).is_some_and(|ro| {
                !matches!(
                    ro.code(),
                    OpCode::CPUI_CALL
                        | OpCode::CPUI_CALLIND
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_RETURN
                        | OpCode::CPUI_COPY
                        | OpCode::CPUI_CAST
                        | OpCode::CPUI_INT_SEXT
                        | OpCode::CPUI_INT_ZEXT
                        | OpCode::CPUI_SUBPIECE
                        | OpCode::CPUI_STORE
                        | OpCode::CPUI_MULTIEQUAL
                        | OpCode::CPUI_INDIRECT
                )
            })
        })
    })
}
