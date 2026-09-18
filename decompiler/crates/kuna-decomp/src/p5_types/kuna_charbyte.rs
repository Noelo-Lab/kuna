//! A byte loaded through a `char *` stays `char` (kuna `charbyte`, P5).
//!
//! x86 loads a byte with `movzx` whether the program meant it signed or not,
//! so the p-code for `*p` is often `LOAD` followed by `INT_ZEXT`.
//! `TypeOpIntZext::getInputLocal` votes `getBase(1, TYPE_UINT)` for the byte,
//! and in `getLocalType`'s fold `SUB_UINT_PLAIN` (16) beats `SUB_INT_CHAR`
//! (19), so the byte is seeded `unsigned char` before propagation starts.  The
//! `char *` that reaches the LOAD's address afterwards cannot displace it --
//! `propagateTypeEdge` only ever moves a Varnode to a MORE specific type --
//! and the byte then pushes `unsigned char *` back onto the pointer, which it
//! does win.  A string walk ends up spelled in the wrong vocabulary:
//!
//! ```text
//!   v1 = (unsigned char *)*dat_c100;               // coreutils fmt -O2 get_line
//!   v7 = strchr("([\'`\"",(int)(char)*v1);
//! ```
//!
//! The rule changes the byte's SEED, never an edge.  While `ActionInferTypes`
//! propagates, [`note_edge`] records each LOAD whose pointer carries `char *`
//! and whose byte holds `uint1` only because of the zero-extension's vote --
//! seen from whichever end the propagation order reaches first.
//! If any were recorded, the pass starts over: the local types are rebuilt,
//! [`seed_char`] seeds those bytes `char` -- the fold they would have had
//! without the zero-extension's vote -- and propagation runs again, by the
//! unchanged upstream rules, from the new seeds.  [`keep_or_restore`] then
//! keeps the second propagation only where it moved nothing but `uint1` to
//! `char` (through pointers and arrays), and otherwise puts the first one's
//! types back.  Everything kept is a propagation upstream itself could
//! produce: a character constant compared
//! with the byte takes `char` from it (`v1 == '-'`, and `v1 == '\xe9'` rather
//! than a `char` compared with `0xe9`), the pointer keeps `char *`, and a
//! `uint1` that reaches the byte from anywhere else still wins it.
//!
//! "Only because of the zero-extension" is checked on the readers: an
//! `INT_ZEXT` must read the byte, and no other reader may vote `TYPE_UINT` --
//! a mask, an unsigned comparison, a logical shift, an unsigned division or an
//! `unsigned char` call argument is the program treating the byte as a
//! number, and that byte keeps `uint1`.  The widened value is never claimed:
//! the `INT_ZEXT` output keeps its own type, and the printer shows the
//! zero-extension of a `char` as the `(unsigned char)` cast it is.  A byte
//! read through an `unsigned char *` never meets a `char *`, so it stays
//! `unsigned char`.
//!
//! Two kinds of byte never become `char`, recorded or not.  A byte that
//! reaches a call or user-op argument, a return or a switch, directly or
//! through copies, joins and one-byte arithmetic ([`read_without_cast`]):
//! kuna shrinks a call argument to the byte when the `movzx` that widened it
//! folds away, and C would sign-extend a `char` there with no cast to show it
//! (`logit("%d",c)` passes -128 for 0x80 where the binary passes 128; a
//! `switch` on it never reaches `case 0x80:`).  And
//! a counter ([`is_counter`]), whose own sum or difference is stored back
//! through the address it was loaded from (`*p = c + 1`): the `char *` it
//! meets can be nothing but that sum's default signed vote.  Either one in the
//! second propagation -- a sibling `a0[1]` read through the retyped pointer
//! included -- restores the first.
//!
//! Gated by [`Architecture::char_byte`](crate::architecture::Architecture)
//! (option `charbyte on|off`); with the option off nothing is recorded and the
//! pass runs once, exactly as upstream.

use std::cell::RefCell;
use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

thread_local! {
    /// The bytes recorded by [`note_edge`] during the current `ActionInferTypes`
    /// propagation; `None` while no propagation is recording.
    static NOTED: RefCell<Option<Vec<VarnodeId>>> = const { RefCell::new(None) };
}

/// Is `t` the one-byte ASCII `char` core type?
pub fn is_char(t: &Datatype) -> bool {
    t.get_size() == 1 && t.get_metatype() == type_metatype::TYPE_INT && t.is_ascii()
}

/// Is `t` the plain one-byte unsigned integer (`uint1`)?
pub fn is_uint1(t: &Datatype) -> bool {
    t.get_size() == 1
        && t.get_metatype() == type_metatype::TYPE_UINT
        && !t.is_char_print()
        && !t.is_enum_type()
}

/// Is the unsigned vote on the loaded byte `vn` a zero-extension and nothing
/// else?  An `INT_ZEXT` must read it, and no other reader may vote `TYPE_UINT`.
pub fn zext_is_only_unsigned_vote(data: &Funcdata, vn: VarnodeId) -> bool {
    let v = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    let mut zext = false;
    for r in v.descend_iter() {
        let o = match data.obank().get(r) {
            Some(o) => o,
            None => return false,
        };
        if o.code() == OpCode::CPUI_INT_ZEXT {
            zext = true;
            continue;
        }
        let vote = crate::p5_types::coreaction_infertypes::input_type_local(data, r, o.get_slot(vn));
        if vote.get_metatype() == type_metatype::TYPE_UINT {
            return false;
        }
    }
    zext
}

/// Does `vn` reach, directly or through copies, joins and one-byte
/// arithmetic, a call or user-op argument, a returned value or a switch?
/// C promotes each of those with no cast to show it, so a `char` there is
/// sign-extended where the binary's folded-away `movzx` zero-extended, and a
/// `case 0x80:` label never matches it.
pub fn read_without_cast(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut seen = vec![vn];
    let mut work = vec![vn];
    while let Some(cur) = work.pop() {
        let readers: Vec<OpId> = match data.vbank().get(cur) {
            Some(v) => v.descend_iter().collect(),
            None => return true,
        };
        for r in readers {
            let o = match data.obank().get(r) {
                Some(o) => o,
                None => return true,
            };
            let through = match o.code() {
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_BRANCHIND => return true,
                OpCode::CPUI_INDIRECT => o.get_slot(cur) == 0,
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_2COMP
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_LEFT => true,
                _ => false,
            };
            if let Some(w) = o.get_out().filter(|_| through) {
                let one_byte = data.vbank().get(w).map(|x| x.get_size() == 1).unwrap_or(false);
                if one_byte && !seen.contains(&w) && seen.len() < 64 {
                    seen.push(w);
                    work.push(w);
                }
            }
        }
    }
    false
}

/// Follow `vn` back through COPYs.
fn strip_copies(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..8 {
        let def = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
        match def {
            Some(o) if o.code() == OpCode::CPUI_COPY => match o.get_in(0) {
                Some(i) => vn = i,
                None => break,
            },
            _ => break,
        }
    }
    vn
}

/// Do `a` and `b` compute the same address: one Varnode up to COPYs, equal
/// constants, or the same op over the same inputs?
fn same_address(data: &Funcdata, a: VarnodeId, b: VarnodeId, depth: u32) -> bool {
    let (a, b) = (strip_copies(data, a), strip_copies(data, b));
    if a == b {
        return true;
    }
    let (va, vb) = match (data.vbank().get(a), data.vbank().get(b)) {
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };
    if va.get_size() != vb.get_size() {
        return false;
    }
    if va.is_constant() || vb.is_constant() {
        return va.is_constant() && vb.is_constant() && va.get_offset() == vb.get_offset();
    }
    let (oa, ob) = match (va.get_def().and_then(|d| data.obank().get(d)), vb.get_def().and_then(|d| data.obank().get(d))) {
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };
    depth > 0
        && oa.code() == ob.code()
        && !matches!(oa.code(), OpCode::CPUI_LOAD | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
        && oa.num_input() == ob.num_input()
        && (0..oa.num_input()).all(|i| match (oa.get_in(i), ob.get_in(i)) {
            (Some(x), Some(y)) => same_address(data, x, y, depth - 1),
            _ => false,
        })
}

/// Is the loaded byte `vn` a counter: is its own one-byte sum or difference
/// stored back through the address it was loaded from (`*p = c + 1`),
/// directly or through copies and joins?
pub fn is_counter(data: &Funcdata, vn: VarnodeId) -> bool {
    let from = match data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) {
        Some(o) if o.code() == OpCode::CPUI_LOAD => match o.get_in(1) {
            Some(a) => a,
            None => return false,
        },
        _ => return false,
    };
    let mut seen = vec![vn];
    let mut work = vec![(vn, false)];
    while let Some((cur, summed)) = work.pop() {
        let readers: Vec<OpId> = match data.vbank().get(cur) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for r in readers {
            let o = match data.obank().get(r) {
                Some(o) => o,
                None => continue,
            };
            let next = match o.code() {
                OpCode::CPUI_STORE if summed && o.get_slot(cur) == 2 => {
                    if o.get_in(1).map(|to| same_address(data, to, from, 3)).unwrap_or(false) {
                        return true;
                    }
                    None
                }
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => o.get_out().map(|w| (w, summed)),
                OpCode::CPUI_INDIRECT if o.get_slot(cur) == 0 => o.get_out().map(|w| (w, summed)),
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if !summed => o.get_out().map(|w| (w, true)),
                _ => None,
            };
            if let Some((w, s)) = next {
                let one_byte = data.vbank().get(w).map(|x| x.get_size() == 1).unwrap_or(false);
                if one_byte && !seen.contains(&w) && seen.len() < 64 {
                    seen.push(w);
                    work.push((w, s));
                }
            }
        }
    }
    false
}

/// May the loaded byte `vn` be re-seeded `char`?  Its only unsigned vote is
/// the zero-extension, and it is neither read without a cast nor a counter.
pub fn is_candidate(data: &Funcdata, vn: VarnodeId) -> bool {
    zext_is_only_unsigned_vote(data, vn) && !read_without_cast(data, vn) && !is_counter(data, vn)
}

/// Start recording for one `ActionInferTypes` propagation of `data`.
pub fn begin(data: &Funcdata) {
    let on = data.get_arch().char_byte;
    NOTED.with(|n| *n.borrow_mut() = if on { Some(Vec::new()) } else { None });
}

/// Stop recording and return the bytes to re-seed, if any were recorded.
pub fn take_noted() -> Option<Vec<VarnodeId>> {
    let mut bytes = NOTED.with(|n| n.borrow_mut().take())?;
    if bytes.is_empty() {
        return None;
    }
    bytes.sort();
    bytes.dedup();
    Some(bytes)
}

fn points_to(t: &Datatype, pred: fn(&Datatype) -> bool) -> bool {
    t.get_metatype() == type_metatype::TYPE_PTR && t.get_ptr_to().map(|p| pred(&p)).unwrap_or(false)
}

/// Record the byte of a LOAD edge `inslot -> outslot` of `op` (ends `invn` and
/// `outvn`, pushing `newtype` over `cur`) when the byte and its `char *`
/// pointer disagree only because a zero-extension reads the byte: the pointer
/// pushing `char` onto a `uint1` byte, or the `uint1` byte pushing
/// `unsigned char *` over the `char *` pointer, whichever the propagation
/// order reaches first.
#[allow(clippy::too_many_arguments)]
pub fn note_edge(
    data: &Funcdata,
    op: OpId,
    inslot: int4,
    outslot: int4,
    invn: VarnodeId,
    outvn: VarnodeId,
    newtype: &Rc<Datatype>,
    cur: &Rc<Datatype>,
) {
    let byte = if inslot == 1 && outslot == -1 {
        if !is_char(newtype) || !is_uint1(cur) {
            return;
        }
        outvn
    } else if inslot == -1 && outslot == 1 {
        if !points_to(newtype, is_uint1) || !points_to(cur, is_char) {
            return;
        }
        invn
    } else {
        return;
    };
    let recording = NOTED.with(|n| n.borrow().is_some());
    if !recording || data.obank().get(op).map(|o| o.code()) != Some(OpCode::CPUI_LOAD) {
        return;
    }
    let byte_is_uint1 =
        data.vbank().get(byte).and_then(|v| v.get_temp_type()).map(|t| is_uint1(t)).unwrap_or(false);
    if byte_is_uint1 && is_candidate(data, byte) {
        NOTED.with(|n| {
            if let Some(v) = n.borrow_mut().as_mut() {
                v.push(byte);
            }
        });
    }
}

/// Seed each recorded byte `char` in place of the `uint1` its local-type fold
/// gave it.  Called between rebuilding the local types and propagating again.
/// A byte whose type is locked, or whose storage a type-locked symbol covers,
/// keeps its seed.
pub fn seed_char(data: &mut Funcdata, bytes: &[VarnodeId]) {
    let arch = Rc::clone(data.get_arch());
    let tlst = match arch.types() {
        Some(t) => t,
        None => return,
    };
    let c = match tlst.get_base(1, type_metatype::TYPE_INT) {
        Ok(c) if is_char(&c) => c,
        _ => return,
    };
    for &vn in bytes {
        let (addr, size) = match data.vbank().get(vn) {
            Some(v) if !v.is_type_lock() && v.get_temp_type().map(|t| is_uint1(t)).unwrap_or(false) => {
                (v.get_addr().clone(), v.get_size())
            }
            _ => continue,
        };
        let locked = data
            .get_scope_local()
            .and_then(|lm| lm.build_localtype_seed(&addr, size, tlst))
            .is_some();
        if !locked {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.set_temp_type(Rc::clone(&c));
            }
        }
    }
}

/// Every Varnode's temporary type after the first propagation, so a second
/// one that strays beyond `uint1` -> `char` can be undone.
pub type Snapshot = Vec<(VarnodeId, Rc<Datatype>)>;

/// Record every Varnode's temporary type.
pub fn snapshot(data: &Funcdata) -> Snapshot {
    data.vbank()
        .iter_loc()
        .filter_map(|vn| data.vbank().get(vn).and_then(|v| v.get_temp_type()).map(|t| (vn, Rc::clone(t))))
        .collect()
}

/// Is `now` the type `up` with `uint1` read as `char`, through any depth of
/// pointers and arrays?
pub fn same_but_char(up: &Datatype, now: &Datatype) -> bool {
    if std::ptr::eq(up, now) || (up.get_metatype() == now.get_metatype() && up.type_order(now).ok() == Some(0)) {
        return true;
    }
    if is_uint1(up) && is_char(now) {
        return true;
    }
    let meta = up.get_metatype();
    if meta != now.get_metatype()
        || up.get_size() != now.get_size()
        || !(meta == type_metatype::TYPE_PTR || meta == type_metatype::TYPE_ARRAY)
    {
        return false;
    }
    match (up.get_depend(0), now.get_depend(0)) {
        (Some(a), Some(b)) => same_but_char(&a, &b),
        _ => false,
    }
}

/// Is `vn` loaded through a pointer that does not point to `char`?  Such a
/// byte prints as a cast of its own load (`(char)v2[1]`,
/// `*(char *)((long)v2 + 1)`), which is what the rule exists to remove.
fn loaded_through_other(data: &Funcdata, vn: VarnodeId) -> bool {
    let def = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    match def {
        Some(o) if o.code() == OpCode::CPUI_LOAD => !o
            .get_in(1)
            .and_then(|p| data.vbank().get(p))
            .and_then(|p| p.get_temp_type())
            .map(|t| points_to(t, is_char))
            .unwrap_or(false),
        _ => false,
    }
}

/// Keep the second propagation only if every Varnode ends where the first
/// left it or at the same type with `uint1` read as `char`, and no byte that
/// moved is read without a cast, is a counter or is loaded through a pointer
/// that is not `char *`; otherwise put the first propagation's types back.  A re-seed can tip an unrelated contest -- a
/// `char *` that loses to an `int *` at a join where `unsigned char *` had
/// won -- and it retypes every byte read through the pointer, recorded or not.
pub fn keep_or_restore(data: &mut Funcdata, upstream: Snapshot) -> bool {
    let strays = upstream.iter().any(|(vn, up)| {
        let now = match data.vbank().get(*vn).and_then(|v| v.get_temp_type()) {
            Some(t) => t,
            None => return false,
        };
        if !same_but_char(up, now) {
            return true;
        }
        is_uint1(up)
            && is_char(now)
            && (read_without_cast(data, *vn) || is_counter(data, *vn) || loaded_through_other(data, *vn))
    });
    if strays {
        for (vn, up) in upstream {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.set_temp_type(up);
            }
        }
    }
    !strays
}

#[cfg(test)]
mod tests;
