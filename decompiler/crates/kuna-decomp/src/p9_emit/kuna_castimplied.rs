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
//! out (its own operand's type, which it preserves), a truncation printed as a
//! cast, a load `*(T *)p` through a pointer printed with that cast or
//! declared `T *`, and a subscript `((T *)p)[k]` whose base is printed with that
//! cast.  An implied `COPY` prints as its operand, so it is looked
//! through.  Anything else is unknown, and an unknown operand never lets a
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

/// What is known about the C type of a printed operand.
#[derive(Debug)]
pub(crate) enum CType {
    /// The printed text states this type.
    Known(Rc<Datatype>),
    /// Nothing states it; C's type is at most a promotion of the IR's.
    Unknown,
    /// The text states a type that is none of the IR's (a pointer declaration
    /// over an integer read): refuse.
    Opaque,
}

impl CType {
    fn known(&self) -> Option<&Rc<Datatype>> {
        match self {
            CType::Known(t) => Some(t),
            _ => None,
        }
    }
}

/// The per-function state: the declarations the printer wrote, by spelling.
#[derive(Debug, Default)]
pub(crate) struct ImpliedCasts {
    enabled: bool,
    resigned: bool,
    arms: bool,
    arm_drops: Vec<OpId>,
    locals: HashMap<HighVariableId, String>,
    params: HashMap<String, String>,
    ret: Option<String>,
}

impl ImpliedCasts {
    /// Start a function: forget the previous one's declarations.  `arms` turns on
    /// `castternary` ([`crate::kuna_castternary`]).
    pub(crate) fn begin(&mut self, enabled: bool, resigned: bool, arms: bool) {
        self.enabled = enabled;
        self.resigned = enabled && resigned;
        self.arms = arms;
        self.arm_drops.clear();
        self.locals.clear();
        self.params.clear();
        self.ret = None;
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn arms_enabled(&self) -> bool {
        self.arms
    }

    /// The conversions `castternary` leaves out of the conditional being printed.
    pub(crate) fn set_arm_drops(&mut self, ops: Vec<OpId>) {
        self.arm_drops = ops;
    }

    /// Does `castternary` leave out the conversion `op` prints?
    pub(crate) fn arm_dropped(&self, op: OpId) -> bool {
        self.arm_drops.contains(&op)
    }

    fn tracking(&self) -> bool {
        self.enabled || self.arms
    }

    /// The printer declared local `high` as `spelling`.
    pub(crate) fn record_local(&mut self, high: HighVariableId, spelling: String) {
        if self.tracking() {
            self.locals.insert(high, spelling);
        }
    }

    /// The printer declared parameter `name` as `spelling`.
    pub(crate) fn record_param(&mut self, name: &str, spelling: String) {
        if self.tracking() {
            self.params.insert(name.to_string(), spelling);
        }
    }

    /// The printer declared the function's return type as `spelling`.
    pub(crate) fn record_return(&mut self, spelling: String) {
        if self.tracking() {
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
        let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()) else { return false };
        let (reader, _) = through_copies(fd, read_op, out);
        let below = reader.is_some_and(|r| fixes_type(p, fd, r));
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
            return src.known().is_some_and(|s| preserves(s, &target));
        }
        let want = p.spell(&target);
        if code == OpCode::CPUI_CAST
            && src.known().is_some_and(|s| int_range(s).is_some() && p.spell(s) == want)
        {
            return true;
        }
        let from = match &src {
            CType::Known(s) => s,
            CType::Unknown => &ir_src,
            CType::Opaque => return false,
        };
        if !preserves(from, &target) {
            return false;
        }
        let (reader, value) = through_copies(fd, read_op, outvn);
        match reader {
            None => p.is_statement(op) && self.assigns_to(p, fd, outvn, &want, &target),
            Some(r) => {
                let Some(ro) = fd.obank().get(r) else { return false };
                match ro.code() {
                    OpCode::CPUI_CALL => trusted_param(fd, r, value)
                        .is_some_and(|t| p.spell(&t) == want),
                    OpCode::CPUI_RETURN => {
                        ro.get_slot(value) == 1 && self.ret.as_deref() == Some(want.as_str())
                    }
                    OpCode::CPUI_COPY => p.is_statement(r) && ro.get_out().is_some_and(|lhs| {
                        fd.vbank().get(lhs).is_some_and(|v| v.is_explicit())
                            && self.assigns_to(p, fd, lhs, &want, &target)
                    }),
                    _ => false,
                }
            }
        }
    }

    /// The C type of `vn` as printed where `reader` reads it.
    pub(crate) fn operand_type(&self, p: &dyn PrintedForms, fd: &Funcdata, vn: VarnodeId, reader: OpId) -> CType {
        let Some(v) = fd.vbank().get(vn) else { return CType::Opaque };
        if v.is_annotation() {
            return CType::Opaque;
        }
        if v.is_constant() {
            return CType::Unknown;
        }
        if v.is_explicit() {
            return self.explicit_type(p, fd, vn, reader);
        }
        let Some(def) = v.get_def() else { return CType::Unknown };
        let t = v.get_type_def_facing().clone();
        self.def_type(p, fd, def, t, Some(reader))
    }

    /// The C type of the expression the op `def` prints, whose output has the IR
    /// type `t`, where `reader` (if any) reads it.
    pub(crate) fn def_type(
        &self,
        p: &dyn PrintedForms,
        fd: &Funcdata,
        def: OpId,
        t: Rc<Datatype>,
        reader: Option<OpId>,
    ) -> CType {
        let Some(d) = fd.obank().get(def) else { return CType::Unknown };
        let int = int_range(&t).is_some();
        let dropped_below =
            |r: Option<OpId>| self.enabled && r.is_some() && self.decide(p, fd, def, r, true);
        match d.code() {
            OpCode::CPUI_CAST if int => {
                if p.sign_dropped(def) || !dropped_below(reader) {
                    return CType::Known(t);
                }
                d.get_in(0).map_or(CType::Unknown, |i| self.operand_type(p, fd, i, def))
            }
            OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT if int => {
                if !p.extension_is_cast(def) || p.extension_hidden(def, reader) {
                    return CType::Unknown;
                }
                if !dropped_below(reader) {
                    return CType::Known(t);
                }
                d.get_in(0).map_or(CType::Unknown, |i| self.operand_type(p, fd, i, def))
            }
            OpCode::CPUI_SUBPIECE if int && p.truncation_is_cast(def) => CType::Known(t),
            OpCode::CPUI_COPY => {
                d.get_in(0).map_or(CType::Unknown, |i| self.operand_type(p, fd, i, def))
            }
            OpCode::CPUI_LOAD if int => {
                let Some(ptr) = d.get_in(1) else { return CType::Unknown };
                let Some(pv) = fd.vbank().get(ptr) else { return CType::Unknown };
                let ptr_ty = if pv.is_explicit() {
                    match self.explicit_type(p, fd, ptr, def) {
                        CType::Known(pt) => pt,
                        _ => return CType::Unknown,
                    }
                } else if let Some(c) = pv.get_def().and_then(|c| fd.obank().get(c)) {
                    match c.code() {
                        OpCode::CPUI_CAST => pv.get_type_def_facing().clone(),
                        OpCode::CPUI_PTRADD => match c
                            .get_in(0)
                            .and_then(|b| fd.vbank().get(b))
                            .filter(|b| !b.is_explicit())
                            .filter(|b| {
                                b.get_def()
                                    .and_then(|d| fd.obank().get(d))
                                    .is_some_and(|d| d.code() == OpCode::CPUI_CAST)
                            }) {
                            Some(b) => b.get_type_def_facing().clone(),
                            None => return CType::Unknown,
                        },
                        _ => return CType::Unknown,
                    }
                } else {
                    return CType::Unknown;
                };
                if ptr_ty.get_metatype() != type_metatype::TYPE_PTR {
                    return CType::Unknown;
                }
                match ptr_ty.get_ptr_to() {
                    Some(pointee) if p.spell(&pointee) == p.spell(&t) => CType::Known(t),
                    _ => CType::Unknown,
                }
            }
            _ => CType::Unknown,
        }
    }

    /// The C type of an explicit `vn`: the type its declaration line (or the
    /// signature) spells, else, for a global, its symbol's type.  A declaration that
    /// spells none of the types kuna holds for `vn` -- a merged variable declared
    /// `int *` whose member here reads as a `long` -- is `Opaque`: the text C
    /// sees is not the IR's type, so nothing can be concluded from the IR.
    fn explicit_type(&self, p: &dyn PrintedForms, fd: &Funcdata, vn: VarnodeId, reader: OpId) -> CType {
        let Some(v) = fd.vbank().get(vn) else { return CType::Opaque };
        let Some(high) = v.get_high() else { return CType::Opaque };
        if let Some(spelling) = self.declared_spelling(p, fd, vn) {
            let planned = p.planned_decl_type(high);
            return [
                planned,
                Some(v.get_type_read_facing(reader).clone()),
                Some(v.get_type().clone()),
            ]
            .into_iter()
            .flatten()
            .find(|t| p.spell(t) == spelling)
            .map_or(CType::Opaque, CType::Known);
        }
        let Some(h) = fd.high_bank().get(high) else { return CType::Opaque };
        let global = h.kuna_global()
            || (0..h.num_instances())
                .any(|i| fd.vbank().get(h.get_instance(i)).is_some_and(|m| m.is_persist()));
        if !global {
            return CType::Opaque;
        }
        match h.kuna_symbol_type().cloned() {
            Some(st) if st.get_size() == v.get_size() && int_range(&st).is_some() => CType::Known(st),
            _ => CType::Opaque,
        }
    }

    /// May `lhs = (want)e;` print as `lhs = e;`?  When `lhs` is declared `want`,
    /// always (the caller has already checked the conversion preserves `e`).  With
    /// `castsign` on and `lhs`'s declaration re-signed by `signedness`, also when
    /// the declared type is as wide as the cast's: a conversion to an N-bit integer
    /// depends only on the value modulo 2^N (C11 6.3.1.3, and gcc and clang define
    /// the signed case that way), so converting `e` straight to the declaration
    /// yields the bits the cast followed by the assignment's conversion would.
    fn assigns_to(&self, p: &dyn PrintedForms, fd: &Funcdata, lhs: VarnodeId, want: &str, target: &Datatype) -> bool {
        if self.declared_spelling(p, fd, lhs).as_deref() == Some(want) {
            return true;
        }
        self.resigned_type(p, fd, lhs).is_some_and(|d| d.get_size() == target.get_size())
    }

    /// The type `signedness` re-declared `vn`'s variable as, when `castsign` is on
    /// and that is the declaration the printer wrote.
    fn resigned_type(&self, p: &dyn PrintedForms, fd: &Funcdata, vn: VarnodeId) -> Option<Rc<Datatype>> {
        if !self.resigned {
            return None;
        }
        let spelled = self.declared_spelling(p, fd, vn)?;
        let high = fd.vbank().get(vn)?.get_high()?;
        let d = p.planned_decl_type(high)?;
        (is_int_target(&d) && p.spell(&d) == spelled).then_some(d)
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

}

/// The op that reads the value `vn` carries once `read_op` is known, looking
/// through implied `COPY`s (which print as their operand), with the varnode that
/// op reads it as.
fn through_copies(fd: &Funcdata, read_op: Option<OpId>, vn: VarnodeId) -> (Option<OpId>, VarnodeId) {
    let (mut reader, mut value) = (read_op, vn);
    for _ in 0..8 {
        let Some(r) = reader else { break };
        let Some(ro) = fd.obank().get(r) else { break };
        if ro.code() != OpCode::CPUI_COPY {
            break;
        }
        let Some(out) = ro.get_out() else { break };
        let Some(ov) = fd.vbank().get(out) else { break };
        if ov.is_explicit() {
            break;
        }
        let mut it = ov.descend_iter();
        match (it.next(), it.next()) {
            (Some(only), None) => {
                reader = Some(only);
                value = out;
            }
            _ => break,
        }
    }
    (reader, value)
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
