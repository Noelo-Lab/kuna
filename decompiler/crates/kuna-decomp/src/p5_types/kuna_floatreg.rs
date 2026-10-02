//! (kuna) A value a function returns in a floating-point register is a float (P5).
//!
//! A calling convention that reserves registers for floating point (`xmm0` on
//! x86-64, `s0`/`d0` on hard-float ARM and AArch64, `ST0` on i386) makes the
//! register itself the declaration of what a function returns there: a float of
//! the register's width. The type fold hears nothing of it when the value is only
//! moved -- a constant, or a float callee's result handed back -- so
//! `float qnanf_(void) { return __builtin_nanf(""); }` printed as
//! `unsigned int qnanf_(void) { return 0x7fc00000; }`, and every caller that used
//! the result as a float printed `(float)qnanf_()`, a value conversion of the
//! bits.
//!
//! [`float_register_vote`] offers the float for a value a RETURN reads from a
//! float-class output register, where the fold says no more than an integer or
//! raw bytes. It is refused wherever `protoorder` refuses a callee's float vote:
//! an integer op computes with the value, it is stored, pieced or handed on
//! outside a float register, it is read from or written to a global, or it is
//! loaded through a pointer the function also moves integers through at that
//! width. It is refused where the value is one of the function's own inputs,
//! whose type its callers decide: `float pass(float x) { return x; }` called as
//! `pass(p[1])` of an `int *` would print a conversion by value where the binary
//! hands the bits on. It is refused where the value is handed to a call whose
//! parameter no declaration or recovery makes a float, or is the result of a call
//! whose return none does -- `f2u(a0)` beside `unsigned int f2u(unsigned int)`
//! converts. And it is refused for a NaN constant `NAN` does not spell exactly,
//! and for one half of an ARM register pair the function uses whole (a `double`
//! in `d0`). The function returns one type in the register, so a refusal for
//! any value a RETURN hands back there refuses them all: `if (k) return g.f;
//! return 0.0f;`, with a `ret` on each path, reads a global on one of them.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_class, type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::p4_calls::fspec::ParamListStandard;
use crate::varnode::Varnode;

/// The float `vn` is, when the function returns it in a float register and
/// nothing the function does with it says otherwise; `ct` is the fold's type
/// for it.
pub(crate) fn float_register_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    vote(data, vn, ct, None)
}

/// [`float_register_vote`], taking the result of the call `jump` for whatever
/// the function returns: an import stub's jump through its slot.
fn vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>, jump: Option<crate::context::OpId>) -> Option<Rc<Datatype>> {
    if !matches!(ct.get_metatype(), type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_INT | type_metatype::TYPE_UINT) {
        return None;
    }
    let node = data.vbank().get(vn)?;
    let size = node.get_size();
    if !matches!(size, 4 | 8 | 10) || node.is_type_lock() || !returned_in_a_float_register(data, vn, node) {
        return None;
    }
    let float = data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()?;
    if returned_beside(data, vn).into_iter().any(|r| refuses(data, r, &float, jump)) {
        return None;
    }
    Some(float)
}

/// Does anything about the value `vn`, returned in a float register, refuse
/// the float `float` for it?
fn refuses(data: &Funcdata, vn: VarnodeId, float: &Rc<Datatype>, jump: Option<crate::context::OpId>) -> bool {
    if data.vbank().get(vn).is_some_and(|n| n.is_constant()) {
        return !spells(data, vn);
    }
    let family = crate::kuna_protoorder::value_family(data, vn);
    family.iter().any(|&v| data.vbank().get(v).is_some_and(|n| n.is_input()))
        || crate::kuna_protoorder::input_refuses(data, vn, float)
        || family.iter().any(|&v| crosses_a_call_as_other_than_a_float(data, v, jump))
        || family.iter().any(|&v| loaded_beside_integers(data, v))
        || !spells_exactly(data, vn)
}

/// `vn` and every other value a live RETURN hands back in a slot `vn` is
/// returned in: the function returns one type there, so a float is refused for
/// all of them when it is refused for one. `if (ready) return packed.f; return
/// 0.0f;` returns a global on one path and a constant on the other, and the
/// constant alone would make the global a float.
fn returned_beside(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let Some(node) = data.vbank().get(vn) else { return vec![vn] };
    let slots: Vec<int4> = node
        .descend_iter()
        .filter_map(|r| data.obank().get(r).filter(|o| o.code() == OpCode::CPUI_RETURN))
        .flat_map(|o| (1..o.num_input()).filter(|&s| o.get_in(s) == Some(vn)).collect::<Vec<_>>())
        .collect();
    let mut out = vec![vn];
    for r in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
        for &s in &slots {
            if let Some(v) = o.get_in(s).filter(|v| !out.contains(v)) {
                out.push(v);
            }
        }
    }
    out
}

/// Does the value `v` cross a call as something no declaration or recovery
/// makes a float: handed to a parameter that is not one, or produced by a call
/// (its output, or the register a call leaves behind) whose return is not one?
fn crosses_a_call_as_other_than_a_float(data: &Funcdata, v: VarnodeId, jump: Option<crate::context::OpId>) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    let handed = node.descend_iter().any(|r| {
        data.obank().get(r).is_some_and(|o| {
            matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                && (1..o.num_input())
                    .filter(|&s| o.get_in(s) == Some(v))
                    .any(|s| !crate::kuna_protoorder::reads_a_float(data, r, s))
        })
    });
    if handed {
        return true;
    }
    let Some(def) = node.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o))) else { return false };
    match def.1.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => Some(def.0) != jump && !call_returns_a_float(data, def.0),
        OpCode::CPUI_INDIRECT if def.1.is_indirect_creation() => {
            let call = def
                .1
                .get_in(1)
                .and_then(|i| data.vbank().get(i))
                .map(|i| crate::context::OpId::from(slotmap::KeyData::from_ffi(i.get_offset())));
            call.is_none_or(|c| data.obank().get(c).is_none() || !call_returns_a_float(data, c))
        }
        _ => false,
    }
}

/// Is `v` loaded through a pointer the function also moves integers through at
/// its width?  The float would type the pointer, and those accesses with it.
fn loaded_beside_integers(data: &Funcdata, v: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    node.get_def().is_some_and(|d| {
        data.obank().get(d).is_some_and(|o| o.code() == OpCode::CPUI_LOAD)
            && crate::kuna_protoorder::moves_integers_beside(data, d, node.get_size())
    })
}

/// Does the call `op`'s callee return a float: a locked output of one, a
/// recovered return its own decompile stated, or one the float-register vote
/// gave it?
fn call_returns_a_float(data: &Funcdata, op: crate::context::OpId) -> bool {
    let Some(fc) = data.get_call_specs_index(op).map(|i| data.get_call_specs(i)) else { return false };
    let proto = fc.proto();
    let float = |t: &Datatype| t.get_metatype() == type_metatype::TYPE_FLOAT;
    if proto.is_output_locked() {
        return proto.get_output_type().is_some_and(|t| float(t));
    }
    let entry = fc.get_entry_address();
    let Some(k) = entry.get_space().map(|s| (s.get_index(), entry.get_offset())) else { return false };
    match data.kuna_callret_stated(k) {
        Some(stated) => float(&stated.ct),
        None => data.kuna_callee_returns(k) == Some(crate::kuna_voidret::Returns::Float),
    }
}

/// The type an import stub's jump through its slot hands back: the stub's own
/// return, when that is a float -- declared (`double strtod(..)`) or the float
/// its register makes it -- and the stub returns the jump's result.  Otherwise
/// the result is untyped and the stub prints `v1 = (float)(*dat_4018)()`,
/// converting what the target handed back.
pub(crate) fn jump_result_type(data: &Funcdata, op: crate::context::OpId, size: int4) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let out = o.get_out()?;
    let at = o.get_addr().clone();
    let returned = data.vbank().get(out)?.descend_iter().any(|r| {
        data.obank().get(r).is_some_and(|ret| {
            ret.code() == OpCode::CPUI_RETURN && !ret.is_dead() && *ret.get_addr() == at && (1..ret.num_input()).any(|s| ret.get_in(s) == Some(out))
        })
    });
    if !returned {
        return None;
    }
    let proto = data.get_func_proto();
    if proto.is_output_locked() {
        let t = proto.get_output_type()?;
        return (t.get_metatype() == type_metatype::TYPE_FLOAT && t.get_size() == size).then(|| Rc::clone(t));
    }
    let unknown = data.get_arch().types()?.get_base(size, type_metatype::TYPE_UNKNOWN).ok()?;
    vote(data, out, &unknown, Some(op))
}

/// Is `node` in a float-class entry of `list`, and a whole float of it?  An
/// entry narrower than a `double` (ARM hard-float's `s0`..`s15`) is also half
/// of one (`d0` over `s0` and `s1`), so a value there is a `float` only when
/// the function never uses the pair whole: `third()` loads `1.0 / 3.0` into
/// `d0`, and the `s0` left once the load is folded is not a float.
fn float_class(data: &Funcdata, list: Option<&ParamListStandard>, node: &Varnode) -> bool {
    let Some((l, i)) = list.and_then(|l| l.find_entry(node.get_addr(), node.get_size(), true).map(|i| (l, i))) else {
        return false;
    };
    let entry = &l.get_entry()[i];
    if entry.get_type() != type_class::TYPECLASS_FLOAT {
        return false;
    }
    if entry.get_size() == 10 {
        return node.get_size() == 10;
    }
    if entry.get_size() >= 8 {
        return true;
    }
    let Some(space) = node.get_addr().get_space() else { return false };
    !data.kuna_float_pair_half((space.get_index(), node.get_offset())) && !pair_used_whole(data, node.get_addr(), node.get_size())
}

/// Does an op of the function read or write one value spanning the whole pair
/// of registers `[addr, addr+size)` belongs to, a `double` in `d0` over `s0`
/// and `s1`?  The trials a call or a return carries for every register it may
/// pass or return in say nothing, and `s1` read on its own is a second float
/// (`clamp(float x, float lo, float hi)` receives `lo` there).
fn pair_used_whole(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(space) = addr.get_space() else { return false };
    let half = size as u64;
    let pair = addr.get_offset() & !(2 * half - 1);
    let lo = Address::new(Rc::clone(space), pair.saturating_sub(64));
    let hi = Address::new(Rc::clone(space), pair + 2 * half);
    let computes = |op: crate::context::OpId| {
        data.obank().get(op).is_some_and(|o| {
            !matches!(
                o.code(),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL
            )
        })
    };
    data.vbank().iter_loc_addr_range(&lo, &hi).any(|id| {
        data.vbank().get(id).is_some_and(|v| {
            let (off, end) = (v.get_offset(), v.get_offset().wrapping_add(v.get_size() as u64));
            off <= pair && pair + 2 * half <= end && (v.get_def().is_some_and(computes) || v.descend_iter().any(computes))
        })
    })
}

/// Note, before any fold can remove it, each narrow float-class register of
/// the function's model (ARM hard-float's `s0`..`s15`) whose pair the
/// function's code uses whole: `vldr d0, [pc]` loads a `double` whose low half
/// is all that is left in `s0` once the constant is folded.
pub(crate) fn note_float_pairs(data: &mut Funcdata) {
    let proto = data.get_func_proto();
    if !proto.has_model() {
        return;
    }
    let lists = [proto.model().input_opt(), proto.model().output_list()];
    let halves: Vec<(Address, int4)> = lists
        .into_iter()
        .flatten()
        .flat_map(|l| l.get_entry().iter())
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() < 8 && e.get_size() > 0)
        .map(|e| (Address::new(Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect();
    for (addr, size) in halves {
        if pair_used_whole(data, &addr, size) {
            let Some(space) = addr.get_space() else { continue };
            data.kuna_note_float_pair_half((space.get_index(), addr.get_offset()));
        }
    }
}

fn returned_in_a_float_register(data: &Funcdata, vn: VarnodeId, node: &Varnode) -> bool {
    let proto = data.get_func_proto();
    if !proto.has_model() || proto.is_output_locked() || data.kuna_float_return_withdrawn() {
        return false;
    }
    let returned = node.descend_iter().any(|r| {
        data.obank()
            .get(r)
            .is_some_and(|o| o.code() == OpCode::CPUI_RETURN && (1..o.num_input()).any(|s| o.get_in(s) == Some(vn)))
    });
    returned && float_class(data, proto.model().output_list(), node)
}

/// Does every constant in `vn`'s value family print as a float literal that
/// compiles back to the same bits?  The constants are the inputs of the copies
/// and joins the family is made of (a literal-pool load folds to `s0 =
/// COPY #0x7fc00123`).  `NAN` and `-NAN` are the canonical quiet NaNs; any
/// other payload, and a signalling NaN, has no spelling.
fn spells_exactly(data: &Funcdata, vn: VarnodeId) -> bool {
    crate::kuna_protoorder::value_family(data, vn).into_iter().all(|v| {
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else {
            return true;
        };
        let inputs = match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => 1,
            OpCode::CPUI_MULTIEQUAL => def.num_input(),
            _ => 0,
        };
        (0..inputs).filter_map(|k| def.get_in(k)).all(|c| spells(data, c))
    })
}

/// Does `c`, when it is a constant, print as a float literal that compiles back
/// to the same bits?
fn spells(data: &Funcdata, c: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(c).filter(|n| n.is_constant()) else { return true };
    let Some(format) = data.get_arch().get_float_format(node.get_size()) else { return false };
    let bits = node.get_offset() as u64;
    if format.get_host_float(bits).1 != kuna_num::float::floatclass::nan {
        return true;
    }
    node.get_size() <= 8 && (bits == format.get_encoding(f64::NAN) || bits == format.get_encoding(-f64::NAN))
}
