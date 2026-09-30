//! (kuna `fieldtype`) A synthesized field some access holds as a pointer is
//! declared as that pointer, not as the integer its first access carried.
//!
//! # The gap
//!
//! `structsynth` types a field from the widest access it measured, and among
//! accesses of one width the first one it met decides. At `-O0` a field is
//! reloaded for every use, so the first load of `sort`'s `buffer.buf` is the
//! one that feeds an integer add, and the field is declared `long` although
//! the next loads hand it to `memmove` and compare it with a `char *`:
//!
//! ```text
//! struct struct_5 { long field_0x0; ... };
//!   memmove((void *)a0->field_0x0,(void *)((a0->field_0x8 - a0->field_0x20) + a0->field_0x0),a0->field_0x20);
//!   v12 = (char *)a0->field_0x0;
//! ```
//!
//! Every read of the field then pays a cast back to what the program holds in
//! it, and the record's declaration says the wrong thing about the object.
//!
//! # The rule
//!
//! A value the program dereferences, calls, hands to a declared pointer
//! parameter, or receives from a function declared to return a pointer is a
//! pointer; an integer add or compare says nothing a pointer could not also do,
//! and neither does the type a value picks up by being merged with another one
//! (a phi, or a local it is copied into), which is where a register reused for
//! a string and a number gets its `char *`. So the accesses of the field's width
//! whose value is *used* as a pointer ([`held_as_pointer`]) are the evidence,
//! and when some of them carry a pointer type and no access carries a float,
//! the field is declared as the most specific pointer they carry:
//!
//! * a pointee that is a record, a named aggregate or code outranks one that is
//!   a scalar, which outranks an undefined pointee, which outranks `void`;
//! * the winner must be the only pointer of its rank -- two different records,
//!   or `char *` beside `unsigned char *`, is not agreement -- unless exactly one
//!   of them is the type of a declared parameter the loaded value is passed to;
//! * otherwise the field keeps the type the first access gave it.
//!
//! A field some access treats as a number no pointer is -- divided, shifted,
//! multiplied, compared with sign ([`used_as_number`]) -- keeps its type
//! whatever else holds it: a hash of an address, or an index once passed
//! through a `void *`, is still a number.
//!
//! A float beside the pointer keeps the field raw bytes, as it already does for
//! a float beside an integer. The integer accesses are unchanged: the printer
//! casts the pointer where an operation needs a number, which is the cast the
//! program's own arithmetic on an address is.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, TypeField};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// (kuna) Parse `option fieldtype on|off`; the caller writes the live field.
pub struct OptionFieldType;

impl OptionFieldType {
    /// The option name.
    pub const NAME: &'static str = "fieldtype";

    /// Parse + validate the `on`/`off` value.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Pointer field typing turned {prop}")))
    }
}

/// How much a pointer says about what it points at.
fn pointee_rank(t: &Datatype) -> Option<u8> {
    let p = t.get_ptr_to()?;
    Some(match p.get_metatype() {
        type_metatype::TYPE_VOID => 0,
        type_metatype::TYPE_UNKNOWN => 1,
        type_metatype::TYPE_STRUCT
        | type_metatype::TYPE_UNION
        | type_metatype::TYPE_ARRAY
        | type_metatype::TYPE_CODE => 3,
        _ => 2,
    })
}

/// Is `a` the same type as `b`, pointer chain included?
fn same(a: &Datatype, b: &Datatype) -> bool {
    if a.get_metatype() != b.get_metatype() || a.get_size() != b.get_size() || a.get_name() != b.get_name() {
        return false;
    }
    let (mut x, mut y) = (a.get_ptr_to(), b.get_ptr_to());
    for _ in 0..4 {
        match (x, y) {
            (Some(p), Some(q)) => {
                if p.get_metatype() != q.get_metatype() || p.get_size() != q.get_size() || p.get_name() != q.get_name()
                {
                    return false;
                }
                x = p.get_ptr_to();
                y = q.get_ptr_to();
            }
            (None, None) => return true,
            _ => return false,
        }
    }
    true
}

/// Is `v` passed to a declared parameter of type `t`?
fn declared_as(data: &Funcdata, v: VarnodeId, t: &Datatype) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    node.descend_iter().any(|r| {
        let Some(o) = data.obank().get(r) else { return false };
        if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            return false;
        }
        let Some(fc) = data.get_call_specs_index(r).map(|i| data.get_call_specs(i)) else { return false };
        (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).any(|s| {
            fc.proto()
                .get_param(s - 1)
                .is_some_and(|p| p.is_type_locked() && p.get_type().is_some_and(|pt| same(&pt, t)))
        })
    })
}

/// One field's evidence: its width, the type the first access gave it, and
/// every access of that width (the access type, and the value it loaded or
/// stored when known).
pub type FieldAccesses<'a> = (int4, Option<&'a Rc<Datatype>>, &'a [(Option<Rc<Datatype>>, Option<VarnodeId>)]);

/// The pointer each field of one record is declared as, in the order given, or
/// `None` for a field the rule leaves alone.
///
/// An access whose value is an address formed from `base` itself is left out
/// of the choice ([`addresses_the_record`]).
///
/// A field no access holds as a pointer is still one when a value loaded from
/// it is compared with a value loaded from a field the rule declared `T *`, and
/// no access uses it as a number: C compares a pointer only with a compatible
/// pointer, and `sort`'s `mergelines_node` walks `lo` down to `end_lo` with
/// nothing but that comparison saying what `end_lo` is.
///
/// A field the first access already gave a pointer whose pointee outranks the
/// chosen one keeps it: `fillbuf` at `-O2` reads its buffer as the `char *` it
/// walks, and only `memmove`'s `void *` proves it, which is no reason to say less.
pub fn pointer_fields(data: &Funcdata, base: VarnodeId, fields: &[FieldAccesses<'_>]) -> Vec<Option<Rc<Datatype>>> {
    let settled: Vec<bool> = fields
        .iter()
        .map(|(_, _, a)| floats(a) || a.iter().any(|(_, v)| v.is_some_and(|v| used_as_number(data, v))))
        .collect();
    let mut out: Vec<Option<Rc<Datatype>>> = fields
        .iter()
        .zip(&settled)
        .map(|((w, _, a), &settled)| {
            if settled {
                return None;
            }
            let own: Vec<_> = a
                .iter()
                .filter(|(_, v)| {
                    v.is_some_and(|v| !addresses_the_record(data, v, base) && held_as_pointer(data, v))
                })
                .cloned()
                .collect();
            pointer_field(data, *w, &own)
        })
        .collect();
    for _ in 0..fields.len() {
        let mut changed = false;
        for i in 0..fields.len() {
            if out[i].is_some() || settled[i] {
                continue;
            }
            let mine: Vec<VarnodeId> = fields[i].2.iter().filter_map(|(_, v)| *v).collect();
            let mut found: Option<Rc<Datatype>> = None;
            let mut clash = false;
            for (j, pt) in out.iter().enumerate() {
                let Some(pt) = pt else { continue };
                if j == i || pt.get_size() != fields[i].0 {
                    continue;
                }
                let theirs: Vec<VarnodeId> = fields[j].2.iter().filter_map(|(_, v)| *v).collect();
                if compared(data, &mine, &theirs) {
                    match &found {
                        Some(f) if !same(f, pt) => clash = true,
                        Some(_) => {}
                        None => found = Some(Rc::clone(pt)),
                    }
                }
            }
            if let (Some(f), false) = (found, clash) {
                out[i] = Some(f);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for (o, (_, first, _)) in out.iter_mut().zip(fields) {
        if o.as_deref().is_some_and(|chosen| outranks(*first, chosen)) {
            *o = None;
        }
    }
    out
}

/// Is the type the first access gave a field a pointer that says more about
/// its pointee than `chosen` does?
fn outranks(first: Option<&Rc<Datatype>>, chosen: &Datatype) -> bool {
    first.is_some_and(|f| f.get_metatype() == type_metatype::TYPE_PTR && pointee_rank(f) > pointee_rank(chosen))
}

/// `v` and the copies and casts of it, up to a few steps: the values that are
/// `v` itself. A phi or an indirect effect joins `v` with other values, so it
/// is not followed.
fn copies(data: &Funcdata, v: VarnodeId) -> Vec<VarnodeId> {
    let mut all = vec![v];
    let mut i = 0;
    while i < all.len() && all.len() < 16 {
        let Some(node) = data.vbank().get(all[i]) else { break };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
                if let Some(out) = o.get_out().filter(|out| !all.contains(out)) {
                    all.push(out);
                }
            }
        }
        i += 1;
    }
    all
}

/// The value `v` is a copy or cast of, a few steps back.
fn origin(data: &Funcdata, mut v: VarnodeId) -> VarnodeId {
    for _ in 0..4 {
        let Some(op) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else { break };
        match (op.code(), op.get_in(0)) {
            (OpCode::CPUI_COPY | OpCode::CPUI_CAST, Some(src)) => v = src,
            _ => break,
        }
    }
    v
}

/// Does the program use `v` as an address: a load or store reads through it,
/// at most after a constant offset or a scaled index is added to it, an
/// indirect call jumps to it, or a declared pointer parameter takes it; or is
/// it what a function declared to return a pointer returned?
fn held_as_pointer(data: &Funcdata, v: VarnodeId) -> bool {
    let mut work = copies(data, v);
    let mut i = 0;
    while i < work.len() && work.len() < 24 {
        let x = work[i];
        i += 1;
        let Some(node) = data.vbank().get(x) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            match o.code() {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE if o.get_in(1) == Some(x) => return true,
                OpCode::CPUI_CALLIND if o.get_in(0) == Some(x) => return true,
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    let Some(fc) = data.get_call_specs_index(r).map(|i| data.get_call_specs(i)) else { continue };
                    let declared = (1..o.num_input()).filter(|&s| o.get_in(s) == Some(x)).any(|s| {
                        fc.proto().get_param(s - 1).is_some_and(|p| {
                            p.is_type_locked()
                                && p.get_type().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_PTR)
                        })
                    });
                    if declared {
                        return true;
                    }
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB if offsets(data, o, x) => {
                    for c in o.get_out().map(|out| copies(data, out)).unwrap_or_default() {
                        if !work.contains(&c) {
                            work.push(c);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let src = origin(data, v);
    let Some(def) = data.vbank().get(src).and_then(|n| n.get_def()) else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    if !matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return false;
    }
    let Some(fc) = data.get_call_specs_index(def).map(|i| data.get_call_specs(i)) else { return false };
    fc.proto().is_output_locked()
        && fc.proto().get_output_type().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_PTR)
}

/// Is `x` the base `op` adds a constant or a scaled index to?
fn offsets(data: &Funcdata, op: &crate::op::PcodeOp, x: VarnodeId) -> bool {
    let konst = |w: Option<VarnodeId>| w.and_then(|w| data.vbank().get(w)).is_some_and(|w| w.is_constant());
    match op.code() {
        OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => op.get_in(0) == Some(x),
        OpCode::CPUI_INT_ADD => {
            let other = if op.get_in(0) == Some(x) { op.get_in(1) } else { op.get_in(0) };
            if konst(other) {
                return true;
            }
            let Some(def) = other.and_then(|w| data.vbank().get(w)).and_then(|w| w.get_def()) else { return false };
            let Some(d) = data.obank().get(def) else { return false };
            matches!(d.code(), OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_LEFT)
                && konst(d.get_in(1))
                && !is_minus_one(data, d.get_in(1))
        }
        _ => false,
    }
}

/// Is `w` the constant -1 at its size?
fn is_minus_one(data: &Funcdata, w: Option<VarnodeId>) -> bool {
    w.and_then(|w| data.vbank().get(w)).is_some_and(|w| {
        w.is_constant() && w.get_offset() == kuna_base::address::calc_mask(w.get_size())
    })
}

/// Does the program treat `v` as a number no pointer is: divide it, take a
/// remainder of it, shift it, multiply it by anything but -1 (a pointer
/// difference negates), compare it with sign, sign-extend it or convert it to a
/// float -- or compute it by one of those?
fn used_as_number(data: &Funcdata, v: VarnodeId) -> bool {
    let numeric = |c: OpCode| {
        matches!(
            c,
            OpCode::CPUI_INT_SDIV
                | OpCode::CPUI_INT_SREM
                | OpCode::CPUI_INT_DIV
                | OpCode::CPUI_INT_REM
                | OpCode::CPUI_INT_SRIGHT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL
                | OpCode::CPUI_INT_SBORROW
                | OpCode::CPUI_INT_SCARRY
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_FLOAT_INT2FLOAT
        )
    };
    let multiplies = |o: &crate::op::PcodeOp| o.code() == OpCode::CPUI_INT_MULT && !(0..2).any(|s| is_minus_one(data, o.get_in(s)));
    for x in copies(data, v) {
        let Some(node) = data.vbank().get(x) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            if numeric(o.code()) || multiplies(o) {
                return true;
            }
        }
    }
    let src = origin(data, v);
    let Some(def) = data.vbank().get(src).and_then(|n| n.get_def()) else { return false };
    data.obank().get(def).is_some_and(|o| {
        (numeric(o.code()) && !matches!(o.code(), OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL))
            || multiplies(o)
    })
}

/// Is `v` an address formed from the record's own base, `&rec[1]` or
/// `&rec->field_0x10`?  Its pointee is whatever the base carried before the
/// record was measured -- a guess from these very accesses -- so it says the
/// field is a pointer and nothing about what at.  `gnulib`'s scratch buffer
/// points its `void *data` at the inline storage just past the record.
fn addresses_the_record(data: &Funcdata, mut v: VarnodeId, base: VarnodeId) -> bool {
    for _ in 0..8 {
        if v == base {
            return true;
        }
        let Some(op) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else {
            return false;
        };
        let next = match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => op.get_in(0),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => {
                let konst = |s: int4| op.get_in(s).and_then(|x| data.vbank().get(x)).is_some_and(|x| x.is_constant());
                if konst(1) && (op.code() != OpCode::CPUI_PTRADD || konst(2)) {
                    op.get_in(0)
                } else {
                    None
                }
            }
            _ => None,
        };
        match next {
            Some(n) => v = n,
            None => return false,
        }
    }
    false
}

/// Does some access carry a float?
fn floats(accesses: &[(Option<Rc<Datatype>>, Option<VarnodeId>)]) -> bool {
    accesses.iter().any(|(t, _)| t.as_ref().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_FLOAT))
}

/// Is a value of `a` compared for equality or unsigned order with a value of `b`?
fn compared(data: &Funcdata, a: &[VarnodeId], b: &[VarnodeId]) -> bool {
    a.iter().any(|&v| {
        let Some(node) = data.vbank().get(v) else { return false };
        node.descend_iter().any(|r| {
            let Some(o) = data.obank().get(r) else { return false };
            if !matches!(
                o.code(),
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL
            ) {
                return false;
            }
            let other = if o.get_in(0) == Some(v) { o.get_in(1) } else { o.get_in(0) };
            other.is_some_and(|w| b.contains(&w))
        })
    })
}

/// The pointer a field of `width` bytes is declared as, given every access of
/// that width (its type, and the value it loaded or stored when known), or
/// `None` when the accesses do not settle on one.
pub fn pointer_field(
    data: &Funcdata,
    width: int4,
    accesses: &[(Option<Rc<Datatype>>, Option<VarnodeId>)],
) -> Option<Rc<Datatype>> {
    choose(width, accesses, |t, v| v.is_some_and(|v| declared_as(data, v, t)))
}

/// [`pointer_field`]'s decision, with `declared(t, value)` answering whether the
/// value is handed to a declared parameter of type `t`.
fn choose(
    width: int4,
    accesses: &[(Option<Rc<Datatype>>, Option<VarnodeId>)],
    declared: impl Fn(&Datatype, Option<VarnodeId>) -> bool,
) -> Option<Rc<Datatype>> {
    if floats(accesses) {
        return None;
    }
    let ptrs: Vec<(&Rc<Datatype>, Option<VarnodeId>, u8)> = accesses
        .iter()
        .filter_map(|(t, v)| {
            let t = t.as_ref()?;
            if t.get_metatype() != type_metatype::TYPE_PTR || t.get_size() != width {
                return None;
            }
            Some((t, *v, pointee_rank(t)?))
        })
        .collect();
    let top = ptrs.iter().map(|p| p.2).max()?;
    let mut kinds: Vec<&Rc<Datatype>> = Vec::new();
    for (t, _, r) in ptrs.iter() {
        if *r == top && !kinds.iter().any(|k| same(k, t)) {
            kinds.push(t);
        }
    }
    if kinds.len() == 1 {
        return Some(Rc::clone(kinds[0]));
    }
    let named: Vec<&&Rc<Datatype>> = kinds
        .iter()
        .filter(|k| ptrs.iter().any(|(t, v, _)| same(t, k) && declared(k, *v)))
        .collect();
    match named.as_slice() {
        [one] => Some(Rc::clone(one)),
        _ => None,
    }
}

/// Is the `KUNA_FIELDTYPE_TRACE` dump on?  One line per synthesized field:
/// function, base, offset, width, declared type, and every access of that width
/// with what the program does with its value (`+` held as a pointer, `#` used
/// as a number).  Changes nothing.
pub(crate) fn trace_on() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("KUNA_FIELDTYPE_TRACE").is_some())
}

fn spell(t: Option<&Datatype>) -> String {
    let Some(t) = t else { return "-".into() };
    if !t.get_name().is_empty() {
        return t.get_name().to_string();
    }
    match t.get_ptr_to() {
        Some(p) if !p.get_name().is_empty() => format!("{}*", p.get_name()),
        Some(p) => format!("{:?}*", p.get_metatype()),
        None => format!("{:?}{}", t.get_metatype(), t.get_size()),
    }
}

fn uses(data: &Funcdata, v: VarnodeId) -> String {
    let Some(node) = data.vbank().get(v) else { return String::new() };
    let tags: Vec<String> = node
        .descend_iter()
        .filter_map(|r| data.obank().get(r).map(|o| (r, o)))
        .map(|(r, o)| match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if o.get_in(0) == Some(v) => "callee".into(),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let slot = (1..o.num_input()).find(|&s| o.get_in(s) == Some(v));
                let fc = data.get_call_specs_index(r).map(|i| data.get_call_specs(i));
                match (slot, fc) {
                    (Some(s), Some(fc)) => match fc.proto().get_param(s - 1) {
                        Some(p) => format!("call{}:{}", if p.is_type_locked() { "L" } else { "" }, spell(p.get_type().map(|t| &**t))),
                        None => "call:?".into(),
                    },
                    _ => "call".into(),
                }
            }
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE if o.get_in(1) == Some(v) => {
                let w = if o.code() == OpCode::CPUI_LOAD { o.get_out() } else { o.get_in(2) };
                format!("deref{}", w.and_then(|w| data.vbank().get(w)).map(|w| w.get_size()).unwrap_or(0))
            }
            c => format!("{c:?}").trim_start_matches("CPUI_").to_lowercase(),
        })
        .collect();
    tags.join(",")
}

/// Print one [`trace_on`] line per field of the record synthesized over `base`.
pub(crate) fn trace(
    data: &Funcdata,
    base: VarnodeId,
    fields: &[(&TypeField, int4, &[(Option<Rc<Datatype>>, Option<VarnodeId>)])],
) {
    let fa = data.get_address().get_offset();
    for (f, width, accesses) in fields {
        let acc: Vec<String> = accesses
            .iter()
            .map(|(t, v)| {
                let mark = match v {
                    Some(v) if used_as_number(data, *v) => "#",
                    Some(v) if held_as_pointer(data, *v) => "+",
                    _ => "",
                };
                format!("{mark}{}[{}]", spell(t.as_deref()), v.map(|v| uses(data, v)).unwrap_or_default())
            })
            .collect();
        eprintln!(
            "FT\t{fa:#x}\t{base:?}\t{:#x}\t{width}\t{}\t{}",
            f.offset,
            spell(Some(&f.field_type)),
            acc.join(" | ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{flags, sub_metatype, DatatypeKind};

    fn ptr_to(ptrto: &Rc<Datatype>) -> Rc<Datatype> {
        let mut p = Datatype::new(8, type_metatype::TYPE_PTR);
        p.submeta = sub_metatype::SUB_PTR;
        p.kind = DatatypeKind::Pointer { ptrto: Rc::clone(ptrto), spaceid: None, truncate: None, wordsize: 1 };
        Rc::new(p)
    }

    fn named(size: i32, meta: type_metatype, name: &str) -> Rc<Datatype> {
        let mut t = Datatype::new(size, meta);
        t.name = name.to_string();
        Rc::new(t)
    }

    fn char_t() -> Rc<Datatype> {
        let mut c = Datatype::new(1, type_metatype::TYPE_INT);
        c.flags |= flags::chartype;
        c.name = "char".to_string();
        Rc::new(c)
    }

    fn acc(ts: &[&Rc<Datatype>]) -> Vec<(Option<Rc<Datatype>>, Option<VarnodeId>)> {
        ts.iter().map(|t| (Some(Rc::clone(t)), None)).collect()
    }

    fn no_decl(_: &Datatype, _: Option<VarnodeId>) -> bool {
        false
    }

    #[test]
    fn option_parses_on_and_off_and_rejects_anything_else() {
        assert!(OptionFieldType.apply("on").unwrap().0);
        assert!(!OptionFieldType.apply("off").unwrap().0);
        assert!(OptionFieldType.apply("pointer").is_err());
    }

    /// `sort`'s `fillbuf`: an integer add first, then `memmove`'s `void *` and a
    /// compare with a `char *`.  The character pointer is the field.
    #[test]
    fn a_pointer_beside_an_integer_wins_and_a_scalar_pointee_outranks_void() {
        let long = named(8, type_metatype::TYPE_INT, "long");
        let void = named(0, type_metatype::TYPE_VOID, "void");
        let (charp, voidp) = (ptr_to(&char_t()), ptr_to(&void));
        let got = choose(8, &acc(&[&long, &voidp, &long, &charp]), no_decl).unwrap();
        assert!(same(&got, &charp));
        let got = choose(8, &acc(&[&long, &voidp]), no_decl).unwrap();
        assert!(same(&got, &voidp), "a void pointer still beats an integer");
        assert!(choose(8, &acc(&[&long, &long]), no_decl).is_none(), "no pointer, no decision");
    }

    /// A called field is code; a record or named aggregate outranks a scalar.
    #[test]
    fn code_and_records_outrank_scalars_and_undefined_pointees_outrank_void() {
        let code = named(1, type_metatype::TYPE_CODE, "code");
        let file = named(0xd8, type_metatype::TYPE_STRUCT, "FILE");
        let undef = named(8, type_metatype::TYPE_UNKNOWN, "undefined8");
        let void = named(0, type_metatype::TYPE_VOID, "void");
        let ulong = named(8, type_metatype::TYPE_UINT, "ulong");
        let (codep, filep, undefp, voidp) = (ptr_to(&code), ptr_to(&file), ptr_to(&undef), ptr_to(&void));
        assert!(same(&choose(8, &acc(&[&ulong, &codep]), no_decl).unwrap(), &codep));
        assert!(same(&choose(8, &acc(&[&ptr_to(&char_t()), &filep]), no_decl).unwrap(), &filep));
        assert!(same(&choose(8, &acc(&[&voidp, &undefp]), no_decl).unwrap(), &undefp));
    }

    /// Two different pointers of the same rank are not agreement, unless exactly
    /// one of them is what a declared parameter the value is passed to names.
    #[test]
    fn two_pointers_of_one_rank_settle_only_on_a_declared_parameter() {
        let uchar = named(1, type_metatype::TYPE_UINT, "uchar");
        let (charp, ucharp) = (ptr_to(&char_t()), ptr_to(&uchar));
        let a = acc(&[&ucharp, &charp]);
        assert!(choose(8, &a, no_decl).is_none());
        let got = choose(8, &a, |t, _| same(t, &charp)).unwrap();
        assert!(same(&got, &charp));
        assert!(choose(8, &a, |_, _| true).is_none(), "both declared is still two answers");
    }

    /// A pointer the first access already carries is kept over a proven one that
    /// says less about the pointee, and replaced by one that says more.
    #[test]
    fn a_first_pointer_is_kept_over_a_proven_pointer_of_lower_rank() {
        let void = named(0, type_metatype::TYPE_VOID, "void");
        let file = named(0xd8, type_metatype::TYPE_STRUCT, "FILE");
        let long = named(8, type_metatype::TYPE_INT, "long");
        let (charp, voidp, filep) = (ptr_to(&char_t()), ptr_to(&void), ptr_to(&file));
        assert!(outranks(Some(&charp), &voidp), "fillbuf -O2: char * is not replaced by memmove's void *");
        assert!(!outranks(Some(&charp), &filep));
        assert!(!outranks(Some(&charp), &charp));
        assert!(!outranks(Some(&long), &voidp), "an integer first access is replaced by any proven pointer");
        assert!(!outranks(None, &voidp));
    }

    /// A float beside a pointer is a union member: the field stays raw bytes.
    /// A pointer of another width is not this field's.
    #[test]
    fn a_float_or_a_pointer_of_another_width_declines() {
        let dbl = named(8, type_metatype::TYPE_FLOAT, "double");
        let charp = ptr_to(&char_t());
        assert!(choose(8, &acc(&[&dbl, &charp]), no_decl).is_none());
        assert!(choose(4, &acc(&[&charp]), no_decl).is_none());
    }
}
