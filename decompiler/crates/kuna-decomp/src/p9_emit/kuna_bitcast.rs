//! Storage transfers preserve bits; FLOAT_* conversion operations change values.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(crate) struct Transfer {
    pub input: VarnodeId,
    pub target: Rc<Datatype>,
    pub size: i32,
    pub to_float: bool,
}

/// A same-width `CAST` between an integer and a float, judged by the types the
/// printed C declares: a named variable's declaration, else the value's own type.
pub(crate) fn storage_transfer(fd: &Funcdata, decl_high_type: bool, op: OpId) -> Option<Transfer> {
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_CAST {
        return None;
    }
    let input = operation.get_in(0)?;
    let out = operation.get_out()?;
    let src = fd.vbank().get(input)?;
    let dst = fd.vbank().get(out)?;
    let size = src.get_size();
    if !matches!(size, 4 | 8) || dst.get_size() != size {
        return None;
    }
    let from_ty = crate::printc::declared_variable_type(fd, decl_high_type, input)
        .unwrap_or_else(|| src.get_type_read_facing(op).clone());
    let to_ty = crate::printc::declared_variable_type(fd, decl_high_type, out)
        .unwrap_or_else(|| dst.get_type_def_facing().clone());
    let from = from_ty.get_metatype();
    let to = to_ty.get_metatype();
    if from == type_metatype::TYPE_UNKNOWN && untyped_call_value(fd, input, 0) {
        return None;
    }
    let integer = |t| {
        matches!(
            t,
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
    };
    let to_float = to == type_metatype::TYPE_FLOAT && integer(from);
    if !to_float && !(from == type_metatype::TYPE_FLOAT && integer(to)) {
        return None;
    }
    Some(Transfer {
        input,
        target: to_ty,
        size,
        to_float,
    })
}

/// Whether a same-width `CAST` of `input` to a float prints as a reinterpretation
/// of its bits: the integer side [`storage_transfer`] accepts, judged by the same
/// printed type.
pub(crate) fn reinterprets_to_float(fd: &Funcdata, decl_high_type: bool, input: VarnodeId) -> bool {
    let Some(src) = fd.vbank().get(input) else {
        return false;
    };
    let from = crate::printc::declared_variable_type(fd, decl_high_type, input)
        .unwrap_or_else(|| src.get_type().clone())
        .get_metatype();
    match from {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT => true,
        type_metatype::TYPE_UNKNOWN => !untyped_call_value(fd, input, 0),
        _ => false,
    }
}

/// An untyped call has no integer C return contract. Its ABI storage alone
/// cannot establish the type of the call expression that the printer emits.
/// A callee whose prototype is locked, whose own decompile stated what it
/// returns, or whose last decompile returned a value other than a float, prints
/// a declaration that gives the call that type.
fn untyped_call_value(fd: &Funcdata, vn: VarnodeId, depth: usize) -> bool {
    if depth == 16 {
        return true;
    }
    let Some(v) = fd.vbank().get(vn) else {
        return true;
    };
    if v.get_type().get_metatype() != type_metatype::TYPE_UNKNOWN {
        return false;
    }
    let Some((id, op)) = v.get_def().and_then(|id| fd.obank().get(id).map(|op| (id, op))) else {
        return false;
    };
    match op.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => !states_a_return(fd, id),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_SUBPIECE => op
            .get_in(0)
            .is_none_or(|input| untyped_call_value(fd, input, depth + 1)),
        _ => false,
    }
}

/// Does the callee of the call `op` declare what it returns: a locked output, a
/// return its own decompile stated, or a non-float value its last decompile
/// returned (raw bytes print as `unsigned int`)?  A statement counts only in
/// the storage and width the call's result is read from: a callee that returns
/// in `rax` says nothing of the `xmm0` its caller reads.
fn states_a_return(fd: &Funcdata, op: OpId) -> bool {
    let Some(fc) = fd.get_call_specs_index(op).map(|i| fd.get_call_specs(i)) else { return false };
    let proto = fc.proto();
    if proto.is_output_locked() {
        return proto.get_output_type().is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID);
    }
    let Some(out) = read_storage(fd, op) else { return false };
    let read_as = |addr: &kuna_base::address::Address, size: i32| out.get_addr() == addr && out.get_size() == size;
    let entry = fc.get_entry_address();
    entry.get_space().map(|s| (s.get_index(), entry.get_offset())).is_some_and(|k| {
        fd.kuna_callret_stated(k).is_some_and(|s| read_as(&s.addr, s.size))
            || (fd.kuna_callee_returns(k) == Some(crate::kuna_voidret::Returns::Other)
                && fd.kuna_callee_return_storage(k).is_some_and(|(a, n)| read_as(a, *n)))
    })
}

/// The Varnode the call `op`'s result is read from: its output, or, where the
/// cast pass gave the call a temporary, the storage the temporary is cast into.
fn read_storage(fd: &Funcdata, op: OpId) -> Option<&crate::varnode::Varnode> {
    let out = fd.vbank().get(fd.obank().get(op)?.get_out()?)?;
    let temporary = out.get_addr().get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL);
    if !temporary {
        return Some(out);
    }
    let mut readers = out.descend_iter();
    let cast = fd.obank().get(readers.next()?).filter(|o| o.code() == OpCode::CPUI_CAST)?;
    if readers.next().is_some() {
        return None;
    }
    fd.vbank().get(cast.get_out()?)
}
