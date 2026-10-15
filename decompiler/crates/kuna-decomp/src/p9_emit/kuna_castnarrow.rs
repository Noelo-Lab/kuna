//! (kuna `castnarrow`) A narrowing C's own assignment, argument or return
//! conversion performs keeps no cast.
//!
//! `castimplied` leaves out a widening that flows into a destination declared
//! with exactly its type, and keeps every truncation and sign change.  So kuna
//! prints `v4 = (unsigned int)a0;` into an `unsigned int v4` from a `long a0`,
//! `return (int)x;` from a function declared `int`, and `toupper((int)a0)`.
//! C performs that very conversion itself in the places it converts a value to
//! a declared type:
//!
//! - the right operand of `=` to the type of the left (C11 6.5.16.1p2),
//! - an argument to its parameter's type when a prototype is in scope
//!   (6.5.2.2p7),
//! - `return e;` to the function's return type (6.8.6.4p3),
//!
//! and a conversion between integer types depends only on the value converted
//! and the type converted to (6.3.1.3).  `T v = (T)e;` therefore stores what
//! `T v = e;` stores, whatever integer type `e` has.
//!
//! # The rule
//!
//! A narrowing is a low `SUBPIECE` printed as a cast, a `CPUI_CAST` between
//! integer types (a sign change, which castimplied keeps), or the `(T)name` the
//! printer writes for the low piece of a wider variable `name`.  Its cast goes
//! when
//!
//! - its value lands whole in a plain `lhs = (T)e;` statement whose `lhs` is a
//!   local or a parameter declared with exactly `T`'s spelling (not an arm of
//!   `c ? a : b`, not the compound literal of a stack-view `__builtin_memcpy`),
//!   a store `*p = (T)e;` through a pointer printed with pointee `T`, a direct
//!   call argument whose type-locked parameter is spelled `T` (never a trial
//!   prototype, a varargs position, a per-call-site override or an argument
//!   `truncarg` narrowed on purpose), or `return` from a function declared to
//!   return `T`; and
//! - `T` is a plain integer and the operand's printed C type is an integer too:
//!   no `bool`, no enum, no float, no pointer.  The operand's type is known when
//!   the text states it (a declared variable, including one of an unknown type
//!   the printer spells as an integer, a cast, a load through a typed pointer),
//!   or when it is integer arithmetic over such operands or a shift of one; a
//!   constant, a call or anything else keeps the cast.  For a piece of `name`,
//!   `name` must be declared with exactly its own type.
//!
//! The value read anywhere else keeps its cast: an operand of arithmetic or of
//! a comparison (`v = (unsigned int)a + b` converts `a` before the addition), a
//! conversion under another conversion, an untrusted argument, a destination
//! of another spelling.
//!
//! Only where `int` is 4 bytes, and only for output languages with C's integer
//! conversions.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_castimplied::{through_copies, trusted_param, CType, ImpliedCasts, PrintedForms};
use crate::kuna_castwiden::{operand_set, plain_int, store_pointee};

/// Leave out the truncation or sign change `op` prints, read by `read_op`
/// (`None` at the top of a statement)?
pub(crate) fn drops(ic: &ImpliedCasts, p: &dyn PrintedForms, fd: &Funcdata, op: OpId, read_op: Option<OpId>) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    let (Some(invn), Some(outvn)) = (o.get_in(0), o.get_out()) else { return false };
    let Some(outv) = fd.vbank().get(outvn) else { return false };
    let target = outv.get_type_def_facing().clone();
    if !plain_int(&target) {
        return false;
    }
    match o.code() {
        OpCode::CPUI_SUBPIECE if p.truncation_is_cast(op) => {}
        OpCode::CPUI_CAST if !p.sign_dropped(op) => {}
        _ => return false,
    }
    let dest = match through_copies(fd, read_op, outvn) {
        (None, _) => plain_statement(p, fd, op).then(|| ic.declared_spelling(p, fd, outvn)).flatten(),
        (Some(r), value) => destination(ic, p, fd, r, value),
    };
    dest.is_some_and(|d| d == p.spell(&target)) && integer_operand(ic, p, fd, invn, op)
}

/// Leave out the `(T)name` the printer writes for `vn`, the low piece `piece`
/// of the variable `name` whose whole type is `whole`, where `op` reads it?
/// The cast is the truncation of `name` to `T`, so it goes on the terms
/// [`drops`] sets: `T` is a plain integer, `name` is declared exactly `whole`,
/// an integer, and C converts the value to exactly `T` where it lands.
#[allow(clippy::too_many_arguments)]
pub(crate) fn drops_partial(
    ic: &ImpliedCasts,
    p: &dyn PrintedForms,
    fd: &Funcdata,
    vn: VarnodeId,
    op: OpId,
    piece: &Rc<Datatype>,
    whole: &Rc<Datatype>,
    name: &str,
) -> bool {
    if !plain_int(piece) || !c_integer(whole) || ic.declared_named(name) != Some(p.spell(whole).as_str()) {
        return false;
    }
    let (Some(r), value) = through_copies(fd, Some(op), vn) else { return false };
    destination(ic, p, fd, r, value).is_some_and(|d| d == p.spell(piece))
}

/// The spelling of the type C converts `value` to where `r` reads it, when that
/// is an assignment, a store, a trusted argument or a `return`.
fn destination(ic: &ImpliedCasts, p: &dyn PrintedForms, fd: &Funcdata, r: OpId, value: VarnodeId) -> Option<String> {
    let ro = fd.obank().get(r)?;
    match ro.code() {
        OpCode::CPUI_CALL => trusted_param(fd, r, value).map(|t| p.spell(&t)),
        OpCode::CPUI_RETURN if ro.get_slot(value) == 1 => ic.return_spelling().map(str::to_string),
        OpCode::CPUI_COPY if plain_statement(p, fd, r) => {
            let lhs = ro.get_out()?;
            fd.vbank().get(lhs)?.is_explicit().then(|| ic.declared_spelling(p, fd, lhs)).flatten()
        }
        OpCode::CPUI_STORE
            if ro.get_slot(value) == 2
                && !ro.does_special_printing()
                && crate::kuna_stackmem::access(fd, r).is_none() =>
        {
            store_pointee(ic, p, fd, r).filter(|t| c_integer(t)).map(|t| p.spell(&t))
        }
        _ => None,
    }
}

/// Is `op` printed as the statement `lhs = <op>;`, not as the compound literal
/// a stack view or a byte-wise store writes?
fn plain_statement(p: &dyn PrintedForms, fd: &Funcdata, op: OpId) -> bool {
    p.is_statement(op)
        && crate::kuna_stackassign::assignment(fd, op).is_none()
        && crate::kuna_stackbytes::assignment(fd, op).is_none()
}

/// Does `vn`, as printed where `op` reads it, have an integer C type?  A shift
/// has the promoted type of its left operand (C11 6.5.7p3).
fn integer_operand(ic: &ImpliedCasts, p: &dyn PrintedForms, fd: &Funcdata, vn: VarnodeId, op: OpId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if v.is_constant() {
        return false;
    }
    match ic.operand_type(p, fd, vn, op) {
        CType::Known(s) => c_integer(&s),
        CType::Opaque => false,
        CType::Unknown => {
            operand_set(ic, p, fd, vn, op, 0).is_some()
                || v.get_def().is_some_and(|def| {
                    fd.obank().get(def).is_some_and(|d| {
                        matches!(d.code(), OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)
                            && plain_int(v.get_type_def_facing())
                            && d.get_in(0).is_some_and(|x| operand_set(ic, p, fd, x, def, 1).is_some())
                    })
                })
        }
    }
}

/// Does the printer spell `t` as a C integer: a plain integer, or an unknown
/// type of an integer's width, which prints as `unsigned long` or `undefined8`?
fn c_integer(t: &Datatype) -> bool {
    plain_int(t)
        || (t.get_metatype() == type_metatype::TYPE_UNKNOWN && !t.is_enum_type() && matches!(t.get_size(), 1 | 2 | 4 | 8))
}
