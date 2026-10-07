//! (kuna `floatbits`) A float helper that works on the bits is still a float (P5).
//!
//! `fabsf` in newlib is `GET_FLOAT_WORD(ix, x); SET_FLOAT_WORD(x, ix &
//! 0x7fffffff); return x;`, which hard-float ARM compiles to `vmov r3,s0; bic
//! r3,r3,#0x80000000; vmov s0,r3; bx lr`.  The only ops on the value are
//! integer ops, so the fold typed the function `unsigned int (unsigned int)`,
//! a prototype that would pass and return in `r0`, and every caller printed
//! each call as a reinterpretation of its result.  The convention passes the
//! value in `s0` and takes it back there, which only a `float` does.
//!
//! [`Plan::of`] finds, once per inference pass, the inputs of the function in
//! float-class registers whose every use is one the bits of a float have:
//! copied, merged, read by a float op, handed to a call in a float register
//! that reads a float of the width, masked, flipped or shifted by a constant
//! (`&`, `|`, `^`, `~`, a shift, adding the sign bit), split into words or
//! joined from them, or compared.  What those bit ops make may only be bit-opped
//! again, compared, or returned.  An `==` or `!=` prints as a float comparison
//! once its operand is a float, so it passes only against an infinity or a
//! normal number its printed literal spells exactly ([`compares_as_a_float`]).
//! An 8-byte input must look like one
//! double ([`Shape`], [`half_of_a_wider_register`]), and a constant merged into
//! the value must spell as a float literal ([`bits_of`]).  Any other use --
//! arithmetic, an address, a store, a call argument made of bits, a register no
//! float is returned in -- keeps the integer.  A value returned in a float-class register of the
//! input's width takes the float with the input, when every value the function
//! returns there is built by those ops from such inputs and constants, or is a
//! constant a float literal spells; otherwise neither does, so the function
//! never prints a float on one side of a helper whose other side the binary
//! treats the same way.  A classifier that returns its answer in an integer
//! register (`isnanf`, `signbit`) types only its input.  The bit ops in the
//! body then read a float, and print as the reinterpretation of its bits the
//! cast printer already spells.
//!
//! Such a return is not a guess the function's readers can overrule
//! (`voidret`): a reader that holds the result as an integer reinterprets the
//! bits, in the register the helper returns in through `callrettype`'s
//! `float_held_as_bits`, and in a register the call only left behind through
//! [`held_bits_token`].

use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

const LIMIT: usize = 128;

/// The inputs and returned values the rule types as floats in one pass.
#[derive(Default)]
pub(crate) struct Plan {
    floats: Vec<VarnodeId>,
    returns: bool,
}

impl Plan {
    /// The plan for `data`'s current IR, empty when the option is off or no
    /// input arrives in a float-class register.
    pub(crate) fn of(data: &Funcdata) -> Plan {
        if !data.get_arch().float_bits {
            return Plan::default();
        }
        let proto = data.get_func_proto();
        if !proto.has_model() || proto.is_input_locked() {
            return Plan::default();
        }
        let list = proto.model().input_opt();
        let mut candidates: Vec<VarnodeId> = data
            .vbank()
            .iter_def_flag(varnode_flags::input)
            .filter(|&v| {
                data.vbank().get(v).is_some_and(|n| {
                    matches!(n.get_size(), 4 | 8)
                        && !n.is_type_lock()
                        && crate::kuna_floatreg::float_class(data, list, n)
                })
            })
            .collect();
        let all = candidates.clone();
        candidates.retain(|&v| !data.vbank().get(v).is_some_and(|n| n.get_size() == 8 && half_of_a_wider_register(data, n, &all)));
        if candidates.is_empty() {
            return Plan::default();
        }
        let mut walks: Vec<(VarnodeId, Walk)> =
            candidates.iter().filter_map(|&c| walk(data, c, &candidates).map(|w| (c, w))).collect();
        loop {
            let accepted: Vec<VarnodeId> = walks.iter().map(|(c, _)| *c).collect();
            let mut floats = accepted.clone();
            let mut refused = Vec::new();
            for &r in walks.iter().flat_map(|(_, w)| w.returned.iter()) {
                match returned_beside_ok(data, r, &accepted) {
                    Some(beside) => floats.extend(beside),
                    None => refused.push(r),
                }
            }
            if refused.is_empty() {
                let returns = walks.iter().any(|(_, w)| !w.returned.is_empty());
                floats.sort();
                floats.dedup();
                return Plan { floats, returns };
            }
            walks.retain(|(_, w)| !w.returned.iter().any(|r| refused.contains(r)));
        }
    }

    /// Does the plan type a value the function returns?
    pub(crate) fn types_a_return(&self) -> bool {
        self.returns
    }

    /// The float the plan gives `vn`, where the fold says no more than an
    /// integer or raw bytes.
    pub(crate) fn vote(&self, data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
        if self.floats.binary_search(&vn).is_err()
            || !matches!(ct.get_metatype(), type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
        {
            return None;
        }
        let size = data.vbank().get(vn)?.get_size();
        data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
    }
}

/// The output token of the call `op` when the caller holds, as an integer of
/// the width its callee returns the bits of a float-register input in, a
/// register the callee does not return in: the callee's float, so
/// `ActionSetCasts` hands it to the integer through a cast, printed as a
/// reinterpretation of its bits where C would convert the value.  That output
/// is what the call left in the register, which `voidret` does not count
/// against the float (`kuna_voidret.rs (elsewhere_than_return)`): betaflight's
/// `bl fabsf; vcmpe.f32 s0, s15; .. pop {r4, pc}` returns the `r0` the call left.
pub(crate) fn held_bits_token(data: &mut Funcdata, op: crate::context::OpId) -> Option<Rc<Datatype>> {
    if !data.get_arch().float_bits {
        return None;
    }
    let (outvn, float) = {
        let fc = data.get_call_specs(data.get_call_specs_index(op)?);
        if fc.proto().is_output_locked() {
            return None;
        }
        let entry = fc.get_entry_address();
        let key = (entry.get_space()?.get_index(), entry.get_offset());
        if !data.kuna_callee_float_bits(key) {
            return None;
        }
        let stated = data.kuna_callret_stated(key)?;
        let outvn = data.obank().get(op)?.get_out()?;
        let out = data.vbank().get(outvn)?;
        let float = stated.ct.get_metatype() == type_metatype::TYPE_FLOAT && stated.ct.get_size() == stated.size;
        let elsewhere = out.get_addr() != &stated.addr && out.get_size() == stated.size;
        (float && elsewhere).then_some(())?;
        (outvn, Rc::clone(&stated.ct))
    };
    let held = data.high_get_type(outvn)?;
    let integer = matches!(held.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT);
    (integer && held.get_size() == float.get_size()).then_some(float)
}

/// What the uses of one input reach: the values returned in a float-class
/// register of its width.
struct Walk {
    returned: Vec<VarnodeId>,
}

/// Walk every use of the input `input` and of what bit ops make of it; `None`
/// where one is not a use the bits of a float have, or no bit op touches it.
fn walk(data: &Funcdata, input: VarnodeId, candidates: &[VarnodeId]) -> Option<Walk> {
    let size = data.vbank().get(input)?.get_size();
    let mut seen: HashSet<VarnodeId> = HashSet::from([input]);
    let mut work = vec![(input, true)];
    let mut returned = Vec::new();
    let mut bits = false;
    let mut shape = Shape::default();
    while let Some((v, raw)) = work.pop() {
        if seen.len() >= LIMIT {
            return None;
        }
        let node = data.vbank().get(v)?;
        if node.is_type_lock() || node.is_persist() || in_data_space(data, node) {
            return None;
        }
        for r in node.descend_iter() {
            let o = data.obank().get(r)?;
            let other = |k: int4| o.get_in(k).filter(|&i| i != v);
            if size == 8 {
                shape.note(data, o);
            }
            let next = match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => Some(raw),
                OpCode::CPUI_MULTIEQUAL => {
                    let inputs: Vec<VarnodeId> = (0..o.num_input()).filter_map(|k| o.get_in(k)).filter(|&i| i != v).collect();
                    if !inputs.iter().all(|&i| seen.contains(&i) || bits_of(data, i, candidates, true)) {
                        return None;
                    }
                    Some(raw && inputs.iter().all(|i| seen.contains(i)))
                }
                OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                    if !(0..2).filter_map(other).all(|i| bits_of(data, i, candidates, false)) {
                        return None;
                    }
                    bits = true;
                    Some(false)
                }
                OpCode::CPUI_INT_NEGATE => {
                    bits = true;
                    Some(false)
                }
                OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    if o.get_in(0) != Some(v) || !is_constant(data, o.get_in(1)) {
                        return None;
                    }
                    bits = true;
                    Some(false)
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                    if o.get_in(0) != Some(v) && o.code() == OpCode::CPUI_INT_SUB || !flips_the_sign(data, o) {
                        return None;
                    }
                    bits = true;
                    Some(false)
                }
                OpCode::CPUI_SUBPIECE => (o.get_in(0) == Some(v)).then_some(false),
                OpCode::CPUI_PIECE => {
                    if !(0..2).filter_map(other).all(|i| bits_of(data, i, candidates, false)) {
                        return None;
                    }
                    Some(false)
                }
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                    if !(0..2).filter_map(other).next().is_some_and(|c| compares_as_a_float(data, c)) {
                        return None;
                    }
                    bits = true;
                    None
                }
                OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => {
                    if !(0..2).filter_map(other).all(|i| bits_of(data, i, candidates, false)) {
                        return None;
                    }
                    bits = true;
                    None
                }
                OpCode::CPUI_RETURN => {
                    if crate::kuna_floatreg::float_class(data, data.get_func_proto().model().output_list(), node) {
                        if node.get_size() != size || !crate::kuna_floatreg::returned_in_a_float_register(data, v, node) {
                            return None;
                        }
                        returned.push(v);
                    } else if raw {
                        return None;
                    }
                    None
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    let float_argument = raw
                        && o.get_in(0) != Some(v)
                        && (1..o.num_input())
                            .filter(|&s| o.get_in(s) == Some(v))
                            .all(|s| crate::kuna_protoorder::float_read_width(data, r, s) == Some(node.get_size()));
                    if !float_argument {
                        return None;
                    }
                    None
                }
                code if crate::kuna_floatreg::reads_a_float(code) => None,
                _ => return None,
            };
            if let Some(raw) = next {
                let out = o.get_out()?;
                if seen.insert(out) {
                    work.push((out, raw));
                }
            }
        }
    }
    (bits && (size != 8 || shape.double && !shape.lanes)).then_some(Walk { returned })
}

/// What the bit ops on an 8-byte input say about its layout.
#[derive(Default)]
struct Shape {
    /// An op works on the exponent or mantissa field of one double: a 64-bit
    /// magnitude, exponent or mantissa mask, or a shift by 52.  Bit 63 alone
    /// (its mask, a flip of it, a shift by 63) is also the sign of the second
    /// float of a `struct { float, float }`, or a bit of a `_Float128`'s low
    /// word, and says nothing.
    double: bool,
    /// A mask repeats one 32-bit pattern in both halves: two float or integer
    /// lanes (`0x7fffffff7fffffff`), not one double.
    lanes: bool,
}

impl Shape {
    fn note(&mut self, data: &Funcdata, o: &crate::op::PcodeOp) {
        let constant = |k: int4| {
            o.get_in(k).and_then(|c| data.vbank().get(c)).filter(|c| c.is_constant() && c.get_size() == 8).map(|c| c.get_offset() as u64)
        };
        match o.code() {
            OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL => {
                for c in (0..2).filter_map(constant) {
                    self.double |= DOUBLE_FIELDS.contains(&c);
                    self.lanes |= c >> 32 == c & 0xffff_ffff && c != 0 && c != u64::MAX;
                }
            }
            OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                let by = o.get_in(1).and_then(|c| data.vbank().get(c)).filter(|c| c.is_constant()).map(|c| c.get_offset());
                let wide = o.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| i.get_size() == 8);
                self.double |= wide && by == Some(52);
            }
            _ => {}
        }
    }
}

/// The 64-bit masks that cut a double at its exponent field: magnitude,
/// exponent, sign and exponent, mantissa, all but the exponent, the quiet bit
/// with the exponent, and the exponent shifted past the sign (`u << 1 ==
/// 0xffe0000000000000`).
const DOUBLE_FIELDS: [u64; 7] = [
    0x7fff_ffff_ffff_ffff,
    0x7ff0_0000_0000_0000,
    0xfff0_0000_0000_0000,
    0x000f_ffff_ffff_ffff,
    0x800f_ffff_ffff_ffff,
    0x7ff8_0000_0000_0000,
    0xffe0_0000_0000_0000,
];

/// Does `c` keep its meaning when an `==` or `!=` with it prints as a float
/// comparison?  Only for a constant that is a normal number or an infinity
/// whose printed literal, which C reads as a `double`, is the constant's own
/// value: a zero compares equal to the other zero, a NaN to nothing, a
/// denormal to zero where the FPU flushes them, and `0.1`, the shortest
/// spelling of `0.1f`, is another double, while the binary compares bits.
fn compares_as_a_float(data: &Funcdata, c: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(c).filter(|n| n.is_constant()) else { return false };
    let bits = node.get_offset() as u64;
    let (exponent, mantissa) = match node.get_size() {
        4 => ((bits >> 23) & 0xff, bits & 0x7f_ffff),
        8 => ((bits >> 52) & 0x7ff, bits & 0xf_ffff_ffff_ffff),
        _ => return false,
    };
    let top = if node.get_size() == 4 { 0xff } else { 0x7ff };
    if exponent == 0 || exponent == top && mantissa != 0 {
        return false;
    }
    if exponent == top {
        return true;
    }
    let Some(format) = data.get_arch().get_float_format(node.get_size()) else { return false };
    let (value, _) = format.get_host_float(bits);
    [false, true].into_iter().all(|sci| format.print_decimal(value, sci).parse::<f64>() == Ok(value))
}

/// Is the 8-byte input `node` half of a 16-byte register another input of the
/// function reads apart from the `candidates`: the low word of an `_Float128`
/// in `xmm0` beside `XMM0_Qb`, or `d0` beside the rest of `q0`?
fn half_of_a_wider_register(data: &Funcdata, node: &crate::varnode::Varnode, candidates: &[VarnodeId]) -> bool {
    let Some(space) = node.get_addr().get_space() else { return false };
    let Some(lookup) = data.get_arch().manage().register_lookup() else { return false };
    let off = node.get_offset();
    let Some(base) = [off, off.wrapping_sub(8)].into_iter().find(|&b| !lookup.get_exact_register_name(space, b, 16).is_empty())
    else {
        return false;
    };
    data.vbank().iter_def_flag(varnode_flags::input).any(|w| {
        !candidates.contains(&w)
            && data.vbank().get(w).is_some_and(|n| {
                n.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, space))
                    && n.get_offset() < base + 16
                    && base < n.get_offset() + n.get_size() as u64
            })
    })
}

/// The values a RETURN hands back beside `vn`, when every one is built by bit
/// ops from the inputs `accepted` and constants, or is a constant a float
/// literal spells, at `vn`'s width; `None` otherwise.
fn returned_beside_ok(data: &Funcdata, vn: VarnodeId, accepted: &[VarnodeId]) -> Option<Vec<VarnodeId>> {
    let node = data.vbank().get(vn)?;
    if !crate::kuna_floatreg::returned_in_a_float_register(data, vn, node) {
        return None;
    }
    let size = node.get_size();
    let beside = crate::kuna_floatreg::returned_beside(data, vn);
    beside
        .iter()
        .all(|&w| {
            data.vbank().get(w).is_some_and(|n| {
                n.get_size() == size
                    && !n.is_type_lock()
                    && if n.is_constant() { crate::kuna_floatreg::spells(data, w) } else { bits_of(data, w, accepted, true) }
            })
        })
        .then_some(beside)
}

/// Is `vn` a constant, or built by copies, merges and bit ops from constants
/// and the inputs `inputs`?  A constant `vn` takes as a value (`value` at
/// `vn`, through copies and merges) must be one a float literal spells: a
/// merged `0x7fc00001` or erased-flash `0xffffffff` would print as `NAN`.  The
/// operands of a bit op are bits, and print as such.
fn bits_of(data: &Funcdata, vn: VarnodeId, inputs: &[VarnodeId], value: bool) -> bool {
    let mut seen: HashSet<(VarnodeId, bool)> = HashSet::new();
    let mut work = vec![(vn, value)];
    while let Some((v, value)) = work.pop() {
        if !seen.insert((v, value)) {
            continue;
        }
        if seen.len() >= LIMIT {
            return false;
        }
        let Some(node) = data.vbank().get(v) else { return false };
        if node.is_constant() {
            if value && !crate::kuna_floatreg::spells(data, v) {
                return false;
            }
            continue;
        }
        if inputs.contains(&v) {
            continue;
        }
        let Some(o) = node.get_def().and_then(|d| data.obank().get(d)) else { return false };
        let (operands, value): (Vec<VarnodeId>, bool) = match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => (o.get_in(0).into_iter().collect(), value),
            OpCode::CPUI_INDIRECT if !o.is_indirect_creation() => (o.get_in(0).into_iter().collect(), value),
            OpCode::CPUI_MULTIEQUAL => ((0..o.num_input()).filter_map(|k| o.get_in(k)).collect(), value),
            OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_NEGATE => (o.get_in(0).into_iter().collect(), false),
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_PIECE => {
                ((0..o.num_input()).filter_map(|k| o.get_in(k)).collect(), false)
            }
            OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT if is_constant(data, o.get_in(1)) => {
                (o.get_in(0).into_iter().collect(), false)
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if flips_the_sign(data, o) => (o.get_in(0).into_iter().collect(), false),
            _ => return false,
        };
        work.extend(operands.into_iter().map(|w| (w, value)));
    }
    true
}

/// Is the INT_ADD or INT_SUB `o` its first input plus or minus the sign bit of
/// its width, which flips that bit and no other?
fn flips_the_sign(data: &Funcdata, o: &crate::op::PcodeOp) -> bool {
    let Some(c) = o.get_in(1).and_then(|c| data.vbank().get(c)).filter(|c| c.is_constant()) else { return false };
    let bits = 8 * c.get_size() as u32;
    bits <= 64 && c.get_offset() as u64 == 1u64 << (bits - 1)
}

fn is_constant(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
    vn.and_then(|v| data.vbank().get(v)).is_some_and(|n| n.is_constant())
}

fn in_data_space(data: &Funcdata, node: &crate::varnode::Varnode) -> bool {
    let data_space = data.get_arch().manage().get_default_data_space();
    data_space.is_some_and(|d| node.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, d)))
}
