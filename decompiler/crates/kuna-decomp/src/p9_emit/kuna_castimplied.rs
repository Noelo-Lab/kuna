//! (kuna `castimplied`) Leave out a cast that C's own conversion rules already
//! perform on exactly those types.
//!
//! `CastStrategyC::isExtensionCastImplied` (cast.cc:249) hides an extension only
//! when the op reading it is integer arithmetic or a comparison against an
//! explicit operand of the same metatype.  Every other reader keeps the cast, so
//! kuna prints `memchr(v10,(int)v4,n)` where `memchr` takes an `int`,
//! `v2 = (long)v1;` into a `long`, `return (unsigned long)v1;` from a function
//! declared `unsigned long`, and `(long)(int)(unsigned int)(unsigned char)c` for
//! one byte widened to 64 bits.  C performs each of those conversions itself:
//!
//! - an argument is converted to its parameter's type when a prototype is in
//!   scope (C11 6.5.2.2p7),
//! - the right operand of `=` is converted to the type of the left (6.5.16.1p2),
//! - `return e;` converts `e` as if by assignment to the return type (6.8.6.4p3),
//! - and a conversion between integer types depends only on the VALUE converted
//!   (6.3.1.3), so a conversion that preserves the value can be left out from
//!   under another conversion that fixes the result type.
//!
//! This module decides, at print time, which of the casts `ActionSetCasts` put in
//! the IR can be left out on those grounds.  The IR is not changed.
//!
//! # The rule
//!
//! Only integer conversions are considered: a `CPUI_CAST` between integer types,
//! or an `INT_SEXT`/`INT_ZEXT` the printer renders as a cast.  A float
//! conversion, a pointer conversion, a conversion to `bool`, an enum, and a
//! truncation (`SUBPIECE`) always keep their cast.
//!
//! Consecutive conversions form a chain whose TOP is the first one read by
//! something that is not a conversion.  The top fixes the type of the whole
//! chain, so a conversion BELOW the top may be left out when it preserves the
//! value of its operand: the operand's printed C type has a range inside the
//! conversion's target range.  That is what makes
//! `(long)(int)(unsigned int)(unsigned char)c` print `(long)(unsigned char)c`:
//! a `unsigned char` value survives the trip through `unsigned int` and `int`
//! unchanged, and `(long)` then converts the same value.  A sign change or a
//! narrowing below the top keeps its cast, because it changes the value.
//!
//! The top itself may be left out when
//!
//! - it converts to the type its operand already has in C (an identity), or
//! - it is a value-preserving widening, and the value flows straight into
//!   * a call argument whose parameter type is trusted and spelled exactly the
//!     same (a declared or measured prototype, never a trial, never a varargs
//!     position, never a per-call-site override such as a resolved format
//!     string, never an argument `truncarg` narrowed on purpose),
//!   * the right side of a `lhs = e;` statement whose `lhs` is a local or a
//!     parameter declared with exactly that spelling (not an arm of `c ? a : b`,
//!     whose type is the two arms' common type), or
//!   * a `return` from a function declared to return exactly that spelling.
//!
//! "Spelled exactly the same" is judged on the text the printer writes for the
//! cast and for the declaration, so two types C treats as one conversion target
//! are the only ones matched.
//!
//! # What the operand's C type is
//!
//! The value-preservation test needs the C type of the printed operand, which is
//! not always the type the IR gives it: C promotes `a - b` over two
//! `unsigned char`s to `int`, and a byte loaded through a `char *` is a `char` in
//! C even when the IR types the loaded value `unsigned char`.  So the operand's
//! type is only taken as known when the printed text states it: a variable (its
//! declaration), a cast that stays (its target), a conversion this rule leaves
//! out (its own operand's type, which it preserves), and a truncation printed as
//! a cast.  Anything else is unknown, and an unknown operand never lets a
//! conversion below the top go.  For the top, an unknown operand falls back to
//! the IR type to judge "widening", which is safe because the destination then
//! performs the very same conversion the cast did.
//!
//! Output languages without implicit integer conversions (Rust) get nothing.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// What the printer knows about how an op prints, which the IR alone does not say.
pub(crate) trait PrintedForms {
    /// The text the printer writes for `ty` in a cast or a declaration.
    fn spell(&self, ty: &Rc<Datatype>) -> String;
    /// Does this `INT_SEXT`/`INT_ZEXT` print as a cast (not as `SEXT14(x)`)?
    fn extension_is_cast(&self, op: OpId) -> bool;
    /// Does upstream's integer-promotion rule already hide this extension?
    fn extension_hidden(&self, op: OpId, read_op: Option<OpId>) -> bool;
    /// Has `signedness` already dropped this `CPUI_CAST`?
    fn sign_dropped(&self, op: OpId) -> bool;
    /// Does this `SUBPIECE` print as a cast to its output type?
    fn truncation_is_cast(&self, op: OpId) -> bool;
    /// The name the printer writes for `high`.
    fn high_name(&self, high: HighVariableId) -> Option<String>;
    /// A declared type some other pass chose for `high` (`signedness`).
    fn planned_decl_type(&self, high: HighVariableId) -> Option<Rc<Datatype>>;
    /// Is `op` being printed as the statement `lhs = <op>;` (not, say, as one arm
    /// of a `c ? a : b`, whose type is the arms' common type)?
    fn is_statement(&self, op: OpId) -> bool;
}

/// The per-function state: the declarations the printer wrote, by spelling.
#[derive(Debug, Default)]
pub(crate) struct ImpliedCasts {
    enabled: bool,
    locals: HashMap<HighVariableId, String>,
    params: HashMap<String, String>,
    ret: Option<String>,
}

impl ImpliedCasts {
    /// Start a function: forget the previous one's declarations.
    pub(crate) fn begin(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.locals.clear();
        self.params.clear();
        self.ret = None;
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// The printer declared local `high` as `spelling`.
    pub(crate) fn record_local(&mut self, high: HighVariableId, spelling: String) {
        if self.enabled {
            self.locals.insert(high, spelling);
        }
    }

    /// The printer declared parameter `name` as `spelling`.
    pub(crate) fn record_param(&mut self, name: &str, spelling: String) {
        if self.enabled {
            self.params.insert(name.to_string(), spelling);
        }
    }

    /// The printer declared the function's return type as `spelling`.
    pub(crate) fn record_return(&mut self, spelling: String) {
        if self.enabled {
            self.ret = Some(spelling);
        }
    }

    /// Leave out the cast `op` prints, read by `read_op` (`None` at the top of a
    /// statement)?
    pub(crate) fn drops(
        &self,
        p: &dyn PrintedForms,
        fd: &Funcdata,
        op: OpId,
        read_op: Option<OpId>,
    ) -> bool {
        if !self.enabled {
            return false;
        }
        let below = read_op.is_some_and(|r| fixes_type(p, fd, r));
        self.decide(p, fd, op, read_op, below)
    }

    fn decide(
        &self,
        p: &dyn PrintedForms,
        fd: &Funcdata,
        op: OpId,
        read_op: Option<OpId>,
        below: bool,
    ) -> bool {
        let Some(o) = fd.obank().get(op) else { return false };
        let code = o.code();
        let (Some(invn), Some(outvn)) = (o.get_in(0), o.get_out()) else { return false };
        let Some(outv) = fd.vbank().get(outvn) else { return false };
        let target = outv.get_type_def_facing().clone();
        if !is_int_target(&target) {
            return false;
        }
        let Some(inv) = fd.vbank().get(invn) else { return false };
        let ir_src = inv.get_type_read_facing(op).clone();
        match code {
            OpCode::CPUI_CAST => {
                if p.sign_dropped(op) || int_range(&ir_src).is_none() {
                    return false;
                }
            }
            OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
                if !p.extension_is_cast(op) || p.extension_hidden(op, read_op) {
                    return false;
                }
            }
            _ => return false,
        }
        let src = self.operand_type(p, fd, invn, op);
        if below {
            return src.is_some_and(|s| preserves(&s, &target));
        }
        let want = p.spell(&target);
        if code == OpCode::CPUI_CAST
            && src.as_ref().is_some_and(|s| p.spell(s) == want)
        {
            return true;
        }
        if !preserves(src.as_ref().unwrap_or(&ir_src), &target) {
            return false;
        }
        match read_op {
            None => {
                p.is_statement(op)
                    && self.declared_spelling(p, fd, outvn).as_deref() == Some(want.as_str())
            }
            Some(r) => {
                let Some(ro) = fd.obank().get(r) else { return false };
                match ro.code() {
                    OpCode::CPUI_CALL => trusted_param(fd, r, outvn)
                        .is_some_and(|t| p.spell(&t) == want),
                    OpCode::CPUI_RETURN => {
                        ro.get_slot(outvn) == 1 && self.ret.as_deref() == Some(want.as_str())
                    }
                    OpCode::CPUI_COPY => p.is_statement(r) && ro.get_out().is_some_and(|lhs| {
                        fd.vbank().get(lhs).is_some_and(|v| v.is_explicit())
                            && self.declared_spelling(p, fd, lhs).as_deref()
                                == Some(want.as_str())
                    }),
                    _ => false,
                }
            }
        }
    }

    /// The C type of `vn` as printed where `reader` reads it, when the printed
    /// text states it.
    fn operand_type(
        &self,
        p: &dyn PrintedForms,
        fd: &Funcdata,
        vn: VarnodeId,
        reader: OpId,
    ) -> Option<Rc<Datatype>> {
        let v = fd.vbank().get(vn)?;
        if v.is_constant() || v.is_annotation() {
            return None;
        }
        if v.is_explicit() {
            return self.declared_type(p, fd, vn, reader);
        }
        let def = v.get_def()?;
        let d = fd.obank().get(def)?;
        let t = v.get_type_def_facing().clone();
        match d.code() {
            OpCode::CPUI_CAST => {
                int_range(&t)?;
                if p.sign_dropped(def) || !self.decide(p, fd, def, Some(reader), true) {
                    return Some(t);
                }
                self.operand_type(p, fd, d.get_in(0)?, def)
            }
            OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
                int_range(&t)?;
                if !p.extension_is_cast(def) || p.extension_hidden(def, Some(reader)) {
                    return None;
                }
                if !self.decide(p, fd, def, Some(reader), true) {
                    return Some(t);
                }
                self.operand_type(p, fd, d.get_in(0)?, def)
            }
            OpCode::CPUI_SUBPIECE if p.truncation_is_cast(def) => {
                int_range(&t)?;
                Some(t)
            }
            _ => None,
        }
    }

    /// The spelling of the declaration an explicit `vn` prints as, when that
    /// declaration is exactly `vn`'s own storage.
    fn declared_spelling(&self, p: &dyn PrintedForms, fd: &Funcdata, vn: VarnodeId) -> Option<String> {
        let v = fd.vbank().get(vn)?;
        if !v.is_explicit() || v.is_constant() || v.is_annotation() {
            return None;
        }
        let high = v.get_high()?;
        let h = fd.high_bank().get(high)?;
        if h.kuna_symbol_type().is_some_and(|st| st.get_size() != v.get_size()) {
            return None;
        }
        if let Some(s) = self.locals.get(&high) {
            return Some(s.clone());
        }
        let name = p.high_name(high)?;
        self.params.get(&name).cloned()
    }

    /// The type an explicit `vn` is declared with, as one of the types kuna
    /// holds for it whose spelling is the declaration's.
    fn declared_type(
        &self,
        p: &dyn PrintedForms,
        fd: &Funcdata,
        vn: VarnodeId,
        reader: OpId,
    ) -> Option<Rc<Datatype>> {
        let spelling = self.declared_spelling(p, fd, vn)?;
        let v = fd.vbank().get(vn)?;
        let planned = v.get_high().and_then(|h| p.planned_decl_type(h));
        [
            planned,
            Some(v.get_type_read_facing(reader).clone()),
            Some(v.get_type().clone()),
        ]
        .into_iter()
        .flatten()
        .find(|t| int_range(t).is_some() && p.spell(t) == spelling)
    }
}

/// Does the conversion `r` fix the C type of whatever it reads, so a
/// value-preserving conversion under it can go?
fn fixes_type(p: &dyn PrintedForms, fd: &Funcdata, r: OpId) -> bool {
    let Some(ro) = fd.obank().get(r) else { return false };
    let target_is_int = ro
        .get_out()
        .and_then(|o| fd.vbank().get(o))
        .is_some_and(|v| is_int_target(v.get_type_def_facing()));
    if !target_is_int {
        return false;
    }
    match ro.code() {
        OpCode::CPUI_CAST => true,
        OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
            if !p.extension_is_cast(r) {
                return false;
            }
            let Some(out) = ro.get_out().and_then(|o| fd.vbank().get(o)) else { return false };
            let reader = if out.is_explicit() {
                None
            } else {
                let mut it = out.descend_iter();
                match (it.next(), it.next()) {
                    (Some(only), None) => Some(only),
                    _ => return false,
                }
            };
            !p.extension_hidden(r, reader)
        }
        _ => false,
    }
}

/// The declared type of the parameter `arg` fills at the direct call `call`,
/// when that declaration is one C sees at this call.
fn trusted_param(fd: &Funcdata, call: OpId, arg: VarnodeId) -> Option<Rc<Datatype>> {
    let o = fd.obank().get(call)?;
    if o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let slot = o.get_slot(arg);
    if slot < 1 {
        return None;
    }
    let fc = fd.get_call_specs(fd.get_call_specs_index(call)?);
    if fc.format_arity().is_some() || fd.get_override().find_proto_override(o.get_addr()).is_some() {
        return None;
    }
    let size = fd.vbank().get(arg)?.get_size();
    if fc.is_zext_trimmed_input(slot, size, o.num_input()) {
        return None;
    }
    let param = fc.proto().get_param(slot - 1)?;
    if !param.is_type_locked() {
        return None;
    }
    param.get_type().cloned()
}

/// Is `t` an integer type a conversion here may target?  `bool` is not: a
/// conversion to `bool` tests the operand against zero.
fn is_int_target(t: &Datatype) -> bool {
    matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
        && int_range(t).is_some()
}

/// The values an integer type holds, or `None` for anything else.
pub(crate) fn int_range(t: &Datatype) -> Option<(i128, i128)> {
    if t.is_enum_type() {
        return None;
    }
    let size = t.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    let bits = 8 * size as u32;
    match t.get_metatype() {
        type_metatype::TYPE_INT => Some((-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)),
        type_metatype::TYPE_UINT => Some((0, (1i128 << bits) - 1)),
        type_metatype::TYPE_BOOL if size == 1 => Some((0, 1)),
        _ => None,
    }
}

/// Does converting any value of `src` to `dst` leave it unchanged?
pub(crate) fn preserves(src: &Datatype, dst: &Datatype) -> bool {
    if !is_int_target(dst) {
        return false;
    }
    match (int_range(src), int_range(dst)) {
        (Some((slo, shi)), Some((dlo, dhi))) => dlo <= slo && shi <= dhi,
        _ => false,
    }
}

#[cfg(test)]
#[path = "kuna_castimplied/tests.rs"]
mod tests;
