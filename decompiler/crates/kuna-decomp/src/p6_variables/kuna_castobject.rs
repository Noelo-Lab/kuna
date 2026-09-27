//! (kuna `castobject`) A frame object whose address is passed to a declared
//! `T *` parameter is declared `T` when its readers agree.
//!
//! The frame model types a stack local from the values its own loads and stores
//! carry, and when two readings of the same bytes differ only in signedness it
//! keeps the one the ported data-type ordering ranks more specific.  `unsigned
//! int` outranks `int` there, so the classic out-parameter
//!
//! ```text
//!   int status;
//!   waitpid(pid, &status, 0);
//!   if (WIFEXITED(status) && WEXITSTATUS(status) == 0) ...
//! ```
//!
//! comes back declared by its bit tests and passed at the callee's type:
//!
//! ```text
//!   unsigned int v5; // stack - 0x28
//!   waitpid(v2,(int *)&v5,0);
//!   if ((v5 & 0x7f) || ((int)v5 >> 8 & 0xffU)) ...
//! ```
//!
//! With the option on, the callee's declaration decides the object's
//! signedness when all of the following hold:
//!
//! - Every frame reference that lands on the object is a plain `&local` handed
//!   straight to a direct call, at a parameter whose type is locked (a declared
//!   or libc signature; never a trial, a format-string position or an
//!   override) and is `T *`, and every such parameter names the same `T`.  An
//!   address kept in a variable, stored, compared, offset or passed at another
//!   type is another reading of the object, and it declines.
//! - `T` is a plain integer of 4 or 8 bytes, the width C computes in without
//!   promotion.
//! - Every access the frame model sees inside the object covers the whole
//!   object, is not type-locked, and is an integer or unknown of `T`'s width.
//!   A byte read of one half, a wider copy or a float declines.  So does an
//!   object the frame model would stretch into an array because nothing
//!   follows it: the declaration would then retype elements no access reads.
//! - No reader asks for the other signedness ([`readers_agree`]).  The walk
//!   follows the value through copies, phis and the operators whose C result
//!   keeps the operand's type, and every operator whose C meaning depends on
//!   the operand's signedness (`<`, `/`, `%`, `>>`, a widening, and a piece
//!   above the lowest byte, which prints as a shift) must compute with `T`'s.
//!   A value re-declared signed is also left alone when `+`, `-`, `*`, unary
//!   `-` or `<<` reads it, directly or through the expression it is printed
//!   into, because signed overflow is undefined where the binary wraps, and
//!   when it meets a constant whose top bit is set in `==`, `!=`, `&`, `|` or
//!   `^`, which C would sign-extend.  A reader this does not model (an address,
//!   an index, a float conversion) declines.  A value stored into another
//!   stack slot is that slot's own variable, so the walk stops there.
//!
//! Only the declaration moves.  The variable's own type is still the one its
//! values give it, and an object stored from a register (`int st = init;`) can
//! keep the old signedness there, so the casts the cast pass computes are
//! against that type.  That is why one reader wanting the other signedness is
//! enough to decline: when every sign-dependent reader computes with `T`'s
//! signedness, each cast put in against the old type converts to the declared
//! type and prints as nothing, and the ones C performs by assignment, argument
//! passing and return need none.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::address::sign_extend;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::varmap::{MapState, RangeType};

/// (kuna) Declare a frame object at the pointee of the parameter its address fills:
/// `castobject on|off`.
pub struct OptionCastObject;

impl OptionCastObject {
    /// The option name.
    pub const NAME: &'static str = "castobject";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::cast_object`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Out-parameter object typing turned {prop}")))
    }
}

const WALK_DEPTH: u32 = 6;

/// The locked declared type of the parameter `arg` fills at the direct call `call`.
fn declared_param(fd: &Funcdata, call: OpId, arg: VarnodeId) -> Option<Rc<Datatype>> {
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
    let param = fc.proto().get_param(slot - 1)?;
    if !param.is_type_locked() {
        return None;
    }
    param.get_type().cloned()
}

fn signedness(t: &Datatype) -> Option<bool> {
    match t.get_metatype() {
        type_metatype::TYPE_INT => Some(true),
        type_metatype::TYPE_UINT => Some(false),
        _ => None,
    }
}

/// A pointee `T` this option declares: a plain integer C computes in unpromoted.
fn declarable(t: &Datatype) -> bool {
    signedness(t).is_some() && matches!(t.get_size(), 4 | 8)
}

/// Is a frame reading of type `h` the same kind of value as `want`?
fn same_kind(h: &Datatype, want: &Datatype) -> bool {
    h.get_size() == want.get_size()
        && matches!(
            h.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
}

/// A frame reference: its offset, and the pointee its only uses declare (or
/// `None` when it is read any other way).
struct FrameRef {
    off: uintb,
    want: Option<Rc<Datatype>>,
}

fn frame_refs(fd: &Funcdata, state: &mut MapState) -> Vec<FrameRef> {
    let checker = state.checker_mut();
    let alias: Vec<uintb> = checker.get_alias().to_vec();
    let bases: Vec<(Option<VarnodeId>, VarnodeId)> =
        checker.get_add_base().iter().map(|ab| (ab.index, ab.base)).collect();
    let mut out = Vec::with_capacity(bases.len());
    for (i, (index, base)) in bases.into_iter().enumerate() {
        let Some(&off) = alias.get(i) else { continue };
        let want = if index.is_some() {
            None
        } else {
            fd.vbank().get(base).and_then(|v| {
                let mut want: Option<Rc<Datatype>> = None;
                for d in v.descend_iter() {
                    let t = declared_param(fd, d, base)
                        .filter(|p| p.get_metatype() == type_metatype::TYPE_PTR)
                        .and_then(|p| p.get_ptr_to())
                        .filter(|t| declarable(t))?;
                    match &want {
                        Some(w) if !same_int(w, &t) => return None,
                        _ => want = Some(t),
                    }
                }
                want
            })
        };
        out.push(FrameRef { off, want });
    }
    out
}

/// Are `a` and `b` the same plain integer, whatever each is spelled?
fn same_int(a: &Datatype, b: &Datatype) -> bool {
    a.get_metatype() == b.get_metatype() && a.get_size() == b.get_size() && !a.is_enum_type() && !b.is_enum_type()
}

/// What one reader of the object's value asks of its signedness.
enum Read {
    /// The operator's C meaning depends on the operand's sign (`true` = signed);
    /// `carry` says the result keeps the operand's type into the expression
    /// around it.
    Demands { signed: bool, carry: bool },
    /// Sign-independent, and the result keeps the operand's type.
    Carries,
    /// Sign-independent, and the result prints a type of its own or no value.
    Opaque,
    /// Undefined on overflow once the operand is signed.
    Overflows,
    /// Not modelled.
    Veto,
}

fn classify(opc: OpCode, slot: int4) -> Read {
    use OpCode::*;
    use Read::*;
    match opc {
        CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL | CPUI_INT_SEXT | CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
            Demands { signed: true, carry: false }
        }
        CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_ZEXT | CPUI_INT_CARRY => {
            Demands { signed: false, carry: false }
        }
        CPUI_INT_SDIV | CPUI_INT_SREM => Demands { signed: true, carry: true },
        CPUI_INT_DIV | CPUI_INT_REM => Demands { signed: false, carry: true },
        CPUI_INT_SRIGHT if slot == 0 => Demands { signed: true, carry: true },
        CPUI_INT_RIGHT if slot == 0 => Demands { signed: false, carry: true },
        CPUI_INT_LEFT if slot == 0 => Overflows,
        CPUI_INT_SRIGHT | CPUI_INT_RIGHT | CPUI_INT_LEFT => Opaque,
        CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_2COMP => Overflows,
        CPUI_INT_AND
        | CPUI_INT_OR
        | CPUI_INT_XOR
        | CPUI_INT_NEGATE
        | CPUI_COPY
        | CPUI_MULTIEQUAL
        | CPUI_INDIRECT => Carries,
        CPUI_INT_EQUAL
        | CPUI_INT_NOTEQUAL
        | CPUI_SUBPIECE
        | CPUI_PIECE
        | CPUI_POPCOUNT
        | CPUI_LZCOUNT
        | CPUI_CBRANCH
        | CPUI_RETURN
        | CPUI_BOOL_NEGATE
        | CPUI_BOOL_AND
        | CPUI_BOOL_OR
        | CPUI_BOOL_XOR => Opaque,
        CPUI_CALL | CPUI_CALLIND if slot >= 1 => Opaque,
        CPUI_STORE if slot == 2 => Opaque,
        _ => Veto,
    }
}

/// Does the `SUBPIECE` `op` keep bytes above the lowest?  The printer spells it
/// as a right shift and a truncation, `(char)(v >> 8)`, and casts a signed `v`
/// to unsigned first.
fn high_piece(fd: &Funcdata, op: OpId) -> bool {
    fd.obank()
        .get(op)
        .and_then(|o| o.get_in(1))
        .and_then(|c| fd.vbank().get(c))
        .is_some_and(|c| c.is_constant() && c.get_offset() != 0)
}

/// Does a constant whose top bit is set meet the value in `op` (one C would
/// sign-extend once the value is signed)?
fn wide_constant(fd: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    if !matches!(
        o.code(),
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR
    ) {
        return false;
    }
    (0..o.num_input()).filter_map(|i| o.get_in(i)).filter(|&v| v != vn).any(|v| {
        fd.vbank().get(v).is_some_and(|c| {
            let bits = c.get_size() * 8;
            c.is_constant() && (1..=64).contains(&bits) && (c.get_offset() >> (bits - 1)) & 1 == 1
        })
    })
}

/// The live Varnodes stored at `(off, size)` in `space`.
fn storage_varnodes(fd: &Funcdata, space: &Rc<AddrSpace>, off: uintb, size: int4) -> Vec<VarnodeId> {
    fd.vbank()
        .iter_loc()
        .filter(|&vn| {
            fd.vbank().get(vn).is_some_and(|v| {
                !v.is_free()
                    && v.get_addr().get_space().map(|s| s.get_index()) == Some(space.get_index())
                    && v.get_offset() == off
                    && v.get_size() == size
            })
        })
        .collect()
}

/// Does every reader of the values in `start`, the object at `off` in `space`
/// and what it merges with, compute with signedness `signed`, directly or
/// through the expressions it is printed into, with nothing that makes the
/// declaration unsafe?  A value stored into another stack slot is that slot's
/// own variable, read at its own declaration, so the walk stops there.
fn readers_agree(fd: &Funcdata, start: &[VarnodeId], signed: bool, space: &AddrSpace, off: uintb) -> bool {
    let other_slot = |vn: VarnodeId| {
        fd.vbank().get(vn).is_some_and(|v| {
            v.get_addr().get_space().map(|s| s.get_index()) == Some(space.get_index()) && v.get_offset() != off
        })
    };
    let mut work: Vec<(VarnodeId, u32)> = start.iter().map(|&vn| (vn, 0)).collect();
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    while let Some((vn, depth)) = work.pop() {
        if !seen.insert(vn) {
            continue;
        }
        let Some(v) = fd.vbank().get(vn) else { return false };
        for r in v.descend_iter() {
            let Some(o) = fd.obank().get(r) else { return false };
            if signed && wide_constant(fd, r, vn) {
                return false;
            }
            let read = if o.code() == OpCode::CPUI_SUBPIECE && high_piece(fd, r) {
                Read::Demands { signed: false, carry: false }
            } else {
                classify(o.code(), o.get_slot(vn))
            };
            let carry = match read {
                Read::Veto => return false,
                Read::Overflows if signed => return false,
                Read::Demands { signed: s, .. } if s != signed => return false,
                Read::Demands { carry, .. } => carry,
                Read::Overflows | Read::Carries => true,
                Read::Opaque => false,
            };
            if carry {
                if depth >= WALK_DEPTH {
                    return false;
                }
                if let Some(out) = o.get_out().filter(|&out| !other_slot(out)) {
                    work.push((out, depth + 1));
                }
            }
        }
    }
    true
}

/// Re-declare every frame object whose address only ever fills declared `T *`
/// parameters, whose every access is the whole object, and whose readers all
/// compute with `T`'s signedness, at `T`.  Returns the objects re-declared, as
/// `(offset, size, T)`.
pub fn declare_out_params(fd: &Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) -> Vec<(uintb, int4, Rc<Datatype>)> {
    let mut declared = Vec::new();
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let signed_off = |o: uintb| sign_extend(o as intb, bits);
    let refs = frame_refs(fd, state);
    if refs.iter().all(|r| r.want.is_none()) {
        return declared;
    }
    let mut wants: HashMap<uintb, Option<Rc<Datatype>>> = HashMap::new();
    for r in &refs {
        let e = wants.entry(r.off).or_insert_with(|| r.want.clone());
        if !matches!((&*e, &r.want), (Some(a), Some(b)) if same_int(a, b)) {
            *e = None;
        }
    }
    let frame_end = state.end_sstart();
    for (off, want) in wants {
        let Some(want) = want else { continue };
        let Some(signed) = signedness(&want) else { continue };
        let size = want.get_size();
        let start = signed_off(off);
        let end: intb = start + size as intb;
        if refs.iter().any(|r| r.off != off && signed_off(r.off) > start && signed_off(r.off) < end) {
            continue;
        }
        let inside = |s: intb, sz: int4| s < end && start < s + sz as intb;
        let hints = state.hints_mut();
        let mut settled = true;
        let mut open = false;
        let clean = hints.iter().filter(|h| inside(h.sstart, h.size.max(1))).all(|h| {
            settled &= same_int(&h.type_, &want);
            open |= h.range_type == RangeType::Open;
            h.sstart == start
                && h.size == size
                && !h.is_type_lock()
                && h.range_type != RangeType::Endpoint
                && (h.range_type != RangeType::Open || h.highind < 0)
                && same_kind(&h.type_, &want)
        });
        if !clean || settled {
            continue;
        }
        if open {
            let next = hints.iter().map(|h| h.sstart).filter(|&s| s >= end).chain(frame_end).min();
            if next != Some(end) {
                continue;
            }
        }
        if !readers_agree(fd, &storage_varnodes(fd, space, off, size), signed, space, off) {
            continue;
        }
        for h in state.hints_mut().iter_mut().filter(|h| h.sstart == start) {
            h.type_ = Rc::clone(&want);
        }
        declared.push((off, size, want));
    }
    declared
}

/// Once the variables are merged, check each object [`declare_out_params`]
/// re-declared.  A merge can bring a value stored into the object, and that
/// value's own readers, into the object's variable, and the variable's type,
/// which the cast pass computes with, can keep the old signedness.  When some
/// reader of the merged variable wants the other signedness and the variable's
/// type is not the declared one, the object is declared at the variable's type
/// again, so the cast pass and the declaration agree.
pub fn reconcile(fd: &mut Funcdata) {
    let objects = fd.cast_objects();
    let Some(space) = fd.get_scope_local().map(|lm| Rc::clone(lm.get_space_id())) else { return };
    for (off, size, want) in objects {
        let Some(signed) = signedness(&want) else { continue };
        let slot = storage_varnodes(fd, &space, off, size);
        let mut highs: Vec<(HighVariableId, Rc<Datatype>)> = Vec::new();
        for &vn in &slot {
            let Some(h) = fd.vbank().get(vn).and_then(|v| v.get_high()) else { continue };
            if highs.iter().any(|(k, _)| *k == h) {
                continue;
            }
            let Some(t) = crate::kuna_declhightype::type_representative(fd, h)
                .and_then(|r| fd.vbank().get(r))
                .map(|r| Rc::clone(r.get_type()))
            else {
                continue;
            };
            highs.push((h, t));
        }
        if highs.iter().all(|(_, t)| same_int(t, &want)) {
            continue;
        }
        let members: Vec<VarnodeId> = highs
            .iter()
            .filter_map(|(h, _)| fd.high_bank().get(*h))
            .flat_map(|h| (0..h.num_instances()).map(|i| h.get_instance(i)).collect::<Vec<_>>())
            .collect();
        if readers_agree(fd, &members, signed, &space, off) {
            continue;
        }
        let mut others = highs.iter().map(|(_, t)| t).filter(|t| !same_int(t, &want));
        let Some(t) = others.next().cloned() else { continue };
        if others.any(|o| !same_int(o, &t)) {
            continue;
        }
        for vn in slot {
            let high = fd.vbank().get(vn).and_then(|v| v.get_high());
            let changed = fd.vbank_mut().get_mut(vn).is_some_and(|v| v.update_type(Rc::clone(&t)));
            if let (true, Some(h)) = (changed, high) {
                if let Some(hh) = fd.high_bank_mut().get_mut(h) {
                    hh.type_dirty();
                }
            }
        }
        let addr = kuna_base::address::Address::new(Rc::clone(&space), off);
        let sym = fd.get_scope_local().and_then(|lm| lm.containing_symbol_for_storage(&addr));
        if let (Some((sym, _)), Some(lm)) = (sym, fd.get_scope_local_mut()) {
            let _ = lm.retype_symbol(sym, t);
        }
    }
}

#[cfg(test)]
mod tests;
