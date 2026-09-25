//! (kuna `castternary`) Leave out the integer widening on an arm of
//! `dest = c ? a : b;` that the conditional operator performs itself.
//!
//! `iteregion` prints an assignment diamond as one conditional.  Each arm keeps
//! the cast the IR put on it, so a table-driven decoder reads
//!
//! ```c
//! v2 = (*(char *)(v8 + a0) != '=') ? (int)*(char *)(v8 + t) : 0;
//! ```
//!
//! where C converts the `char` arm to `int` without being told: the second and
//! third operands of `?:` undergo the usual arithmetic conversions, integer
//! promotion first (C11 6.5.15p5, 6.3.1.8).  `castimplied` refuses these arms,
//! because leaving a cast out of one arm can change the type of the whole
//! conditional: `c ? (long)i : u` is a `long`, and `c ? i : u` is an
//! `unsigned int` for an `int i` and an `unsigned int u`.
//!
//! # The rule
//!
//! Let `R` be the conditional's type as printed today, and `T` the type the arm's
//! cast converts to.  The cast is left out when
//!
//! - it is an integer widening that keeps its operand's value (the conversions
//!   `castimplied` considers: never a narrowing, a sign change, a conversion
//!   from or to `bool`, an enum, a float or a pointer), and
//! - the usual arithmetic conversions of the two arms give `T` whether or not
//!   the cast is printed: `R` is `T`, and so is the type of the conditional
//!   with the cast left out.
//!
//! The conditional then converts the arm to `T` itself, which is the conversion
//! the cast spelled, and its own type and value are the ones it had with the
//! cast; so is every value computed from it, whatever reads it.  When both arms
//! carry a cast, the rule is applied to the pair: both casts go only when the
//! conditional over both bare operands is still `T`, else the first arm's cast
//! alone, else the second's.
//!
//! # The arms' C types
//!
//! Types are compared the way the conversions see them: after integer promotion
//! every arm is one of `int`, `unsigned int`, `long`, `unsigned long`, and each
//! arm's C type is carried as the SET of those it may be.  A declared variable,
//! a cast that stays, a load through a pointer printed with its pointee type
//! (what `castimplied` knows of an operand) is exactly one.  An expression
//! nothing states the type of may be any promotion its width allows: `int` or
//! `unsigned int` up to 4 bytes, any of the four at 8.  A literal is typed by
//! C's rules for an unsuffixed constant of its printed magnitude: `int` up to
//! 0x7fffffff, `unsigned int` or `long` up to 0xffffffff (hexadecimal or
//! decimal), `long` or `unsigned long` beyond (a type that forces a display
//! format may print a character literal, an `int`); one with a `U`/`L` suffix,
//! an equate, or a magnitude above `LONG_MAX` is not typed.  The conversions must give the one
//! type `T` for every combination of the two sets.  An arm of any other type
//! (a pointer, a float, an enum) leaves the conditional alone.
//!
//! Only a target whose `int` is 4 bytes, and only output languages with C's
//! integer conversions: Rust's `if c { a } else { b }` needs both arms of one
//! type and gets nothing.

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_castimplied::{int_range, preserves, CType, ImpliedCasts, PrintedForms};

/// Promoted C integer types, as bits of a set.
const INT: u8 = 1;
const UINT: u8 = 2;
const LONG: u8 = 4;
const ULONG: u8 = 8;

/// One arm of the conditional.
#[derive(Debug, Clone, Copy)]
struct Arm {
    /// The promoted types the arm may have as printed today.
    kept: u8,
    /// A conversion at the top of the arm this rule may leave out: the op, the
    /// type it converts to, and the promoted types its operand may have.
    conv: Option<(OpId, u8, u8)>,
}

/// The conversions to leave out of the arms `arms` (the ops `iteregion` prints
/// as the `?` and `:` values) of one conditional.
pub(crate) fn arm_drops(
    implied: &ImpliedCasts,
    p: &dyn PrintedForms,
    fd: &Funcdata,
    arms: [OpId; 2],
) -> Vec<OpId> {
    if !implied.arms_enabled() {
        return Vec::new();
    }
    let (Some(a), Some(b)) = (arm(implied, p, fd, arms[0]), arm(implied, p, fd, arms[1])) else {
        return Vec::new();
    };
    choose(a, b)
}

/// The conversions the pair `a`, `b` leaves out, both first.
fn choose(a: Arm, b: Arm) -> Vec<OpId> {
    let Some(r) = common(a.kept, b.kept) else { return Vec::new() };
    let options = [(a.conv, b.conv), (a.conv, None), (None, b.conv)];
    for (ca, cb) in options {
        if ca.is_none() && cb.is_none() {
            continue;
        }
        let ta = ca.map_or(a.kept, |(_, _, operand)| operand);
        let tb = cb.map_or(b.kept, |(_, _, operand)| operand);
        let targets_r = [ca, cb].iter().flatten().all(|&(_, t, _)| t == r);
        if targets_r && common(ta, tb) == Some(r) {
            return [ca, cb].iter().flatten().map(|&(op, _, _)| op).collect();
        }
    }
    Vec::new()
}

/// The type of `x ? a : b` for every `a` in the set `x` and `b` in the set `y`,
/// when that is one type.
fn common(x: u8, y: u8) -> Option<u8> {
    let mut out = 0u8;
    for a in [INT, UINT, LONG, ULONG] {
        for b in [INT, UINT, LONG, ULONG] {
            if x & a != 0 && y & b != 0 {
                out |= usual(a, b);
            }
        }
    }
    (out.count_ones() == 1).then_some(out)
}

/// The usual arithmetic conversions of two promoted types (C11 6.3.1.8): the
/// wider type wins; at one width the unsigned type wins; a signed type wider than
/// the unsigned one holds all its values and wins.
fn usual(a: u8, b: u8) -> u8 {
    let wide = |t: u8| t & (LONG | ULONG) != 0;
    let unsigned = |t: u8| t & (UINT | ULONG) != 0;
    match (wide(a), wide(b)) {
        (true, false) => a,
        (false, true) => b,
        _ if unsigned(a) => a,
        _ => b,
    }
}

/// The promoted type of `t`, for an integer (or `bool`) of 1, 2, 4 or 8 bytes.
fn promote(t: &Datatype) -> Option<u8> {
    let (lo, _) = int_range(t)?;
    match (t.get_size(), lo < 0) {
        (1 | 2, _) => Some(INT),
        (4, true) => Some(INT),
        (4, false) => Some(UINT),
        (8, true) => Some(LONG),
        (8, false) => Some(ULONG),
        _ => None,
    }
}

/// The promoted types an operand of known or unknown C type may have.
fn value_set(c: &CType, ir: &Datatype) -> Option<u8> {
    match c {
        CType::Known(t) => promote(t),
        CType::Unknown => {
            let int = matches!(
                ir.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL
            );
            if !int || ir.is_enum_type() {
                return None;
            }
            match ir.get_size() {
                1..=4 => Some(INT | UINT),
                8 => Some(INT | UINT | LONG | ULONG),
                _ => None,
            }
        }
        CType::Opaque => None,
    }
}

/// The C type of the literal the constant `vn` prints as where `op` reads it.
fn literal(fd: &Funcdata, vn: VarnodeId, op: OpId) -> Option<u8> {
    let v = fd.vbank().get(vn)?;
    let ct = v.get_type_read_facing(op);
    if ct.is_enum_type()
        || !matches!(ct.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
        || v.is_unsigned_print()
        || v.is_long_print()
    {
        return None;
    }
    let size = v.get_size();
    if ct.is_char_print() {
        return (size == 1).then_some(INT);
    }
    if !(1..=8).contains(&size) {
        return None;
    }
    let bits = 8 * size as u32;
    let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
    let val = v.get_offset() & mask;
    let negative = ct.get_metatype() == type_metatype::TYPE_INT && (val >> (bits - 1)) & 1 == 1;
    let magnitude = if negative { (mask - val) as u128 + 1 } else { val as u128 };
    if fd.vn_high_equate_symbol(vn).is_some() {
        return None;
    }
    let custom = ct.get_display_format() != 0;
    let set = match magnitude {
        0..=0x7fff_ffff => INT,
        0x8000_0000..=0xffff_ffff => UINT | LONG,
        0x1_0000_0000..=0x7fff_ffff_ffff_ffff => LONG | ULONG,
        _ => return None,
    };
    Some(if custom { set | INT } else { set })
}

/// Is `op` a conversion `castimplied` weighs?
fn is_conversion(fd: &Funcdata, op: OpId) -> bool {
    fd.obank().get(op).is_some_and(|o| {
        matches!(o.code(), OpCode::CPUI_CAST | OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT)
    })
}

/// The arm the op `op` prints (a `COPY` into the destination, or under
/// `iteexpr` any op writing it).
fn arm(implied: &ImpliedCasts, p: &dyn PrintedForms, fd: &Funcdata, op: OpId) -> Option<Arm> {
    let o = fd.obank().get(op)?;
    match o.code() {
        OpCode::CPUI_COPY => {
            let vn = o.get_in(0)?;
            let v = fd.vbank().get(vn)?;
            if v.is_constant() {
                return Some(Arm { kept: literal(fd, vn, op)?, conv: None });
            }
            if !v.is_explicit() && !v.is_annotation() {
                if let Some(def) = v.get_def().filter(|&d| is_conversion(fd, d)) {
                    return conversion_arm(implied, p, fd, def, Some(op));
                }
            }
            let t = v.get_type_read_facing(op).clone();
            let kept = value_set(&implied.operand_type(p, fd, vn, op), &t)?;
            Some(Arm { kept, conv: None })
        }
        OpCode::CPUI_CAST | OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
            conversion_arm(implied, p, fd, op, None)
        }
        _ => {
            let t = fd.vbank().get(o.get_out()?)?.get_type_def_facing().clone();
            let kept = value_set(&implied.def_type(p, fd, op, t.clone(), None), &t)?;
            Some(Arm { kept, conv: None })
        }
    }
}

/// The arm whose value is the conversion `op`, printed where `read_op` reads it.
fn conversion_arm(
    implied: &ImpliedCasts,
    p: &dyn PrintedForms,
    fd: &Funcdata,
    op: OpId,
    read_op: Option<OpId>,
) -> Option<Arm> {
    let o = fd.obank().get(op)?;
    let invn = o.get_in(0)?;
    let target = fd.vbank().get(o.get_out()?)?.get_type_def_facing().clone();
    let ir_src = fd.vbank().get(invn)?.get_type_read_facing(op).clone();
    let src = implied.operand_type(p, fd, invn, op);
    let bare = || value_set(&src, &ir_src);
    match o.code() {
        OpCode::CPUI_CAST if p.sign_dropped(op) => return Some(Arm { kept: promote(&target)?, conv: None }),
        OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
            if !p.extension_is_cast(op) {
                return None;
            }
            if p.extension_hidden(op, read_op) {
                return Some(Arm { kept: bare()?, conv: None });
            }
        }
        _ => {}
    }
    if implied.drops(p, fd, op, read_op) {
        return Some(Arm { kept: bare()?, conv: None });
    }
    let kept = promote(&target).filter(|_| is_int(&target))?;
    let exact = matches!(target.get_size(), 4 | 8);
    let from = match &src {
        CType::Known(s) => s.clone(),
        CType::Unknown => ir_src.clone(),
        CType::Opaque => return Some(Arm { kept, conv: None }),
    };
    let widening = exact && is_int(&ir_src) && is_int(&from) && preserves(&from, &target);
    let conv = if widening { bare().map(|operand| (op, kept, operand)) } else { None };
    Some(Arm { kept, conv })
}

/// An integer that is neither `bool` nor an enum.
fn is_int(t: &Datatype) -> bool {
    matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
        && !t.is_enum_type()
        && int_range(t).is_some()
}

#[cfg(test)]
#[path = "kuna_castternary/tests.rs"]
mod tests;
