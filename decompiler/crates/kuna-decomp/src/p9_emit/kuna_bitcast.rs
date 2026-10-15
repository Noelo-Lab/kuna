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
/// printed C declares: a named variable or field, else the value's own type.
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
    let slot = operation.get_slot(input);
    let from_ty = crate::kuna_stackviews::access_type(fd, input, op, slot)
        .or_else(|| declared_member_type(fd, decl_high_type, input))
        .unwrap_or_else(|| fd.vn_type_read_facing(input, op));
    let to_ty = crate::kuna_stackviews::access_type(fd, out, op, -1)
        .or_else(|| declared_member_type(fd, decl_high_type, out))
        .unwrap_or_else(|| fd.vn_type_def_facing(out));
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

/// A detached frame snapshot keeps its bits when a native float op consumes it.
pub(crate) fn stack_float_input(
    fd: &Funcdata,
    decl_high_type: bool,
    op: OpId,
    input: VarnodeId,
) -> Option<Transfer> {
    if !fd.get_arch().stack_views {
        return None;
    }
    let code = fd.obank().get(op)?.code();
    if !matches!(
        code,
        OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_NAN
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_ROUND
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_TRUNC
    ) {
        return None;
    }
    let value = fd.vbank().get(input)?;
    if value.is_implied() || !matches!(value.get_size(), 4 | 8) {
        return None;
    }
    if !fd.get_func_proto().has_store() {
        return None;
    }
    let emitted_type = crate::printc::declared_variable_type(fd, decl_high_type, input)?;
    if emitted_type.get_size() != value.get_size()
        || !matches!(
            emitted_type.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
    {
        return None;
    }
    let high = value.get_high().and_then(|id| fd.high_bank().get(id))?;
    if !high.is_kuna_frame_snapshot() {
        return None;
    }
    let factory = fd.get_arch().types()?;
    Some(Transfer {
        input,
        target: factory
            .get_base(value.get_size(), type_metatype::TYPE_FLOAT)
            .ok()?,
        size: value.get_size(),
        to_float: true,
    })
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

fn declared_member_type(
    fd: &Funcdata,
    decl_high_type: bool,
    id: VarnodeId,
) -> Option<Rc<Datatype>> {
    let declared = crate::printc::declared_variable_type(fd, decl_high_type, id)?;
    if !fd.get_arch().stack_views {
        return Some(declared);
    }
    let value = fd.vbank().get(id)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    let high = value.get_high().and_then(|id| fd.high_bank().get(id));
    let frame_storage = value.get_space().get_index() == stack.get_index()
        || high.is_some_and(|high| {
            (0..high.num_instances()).any(|index| {
                fd.vbank()
                    .get(high.get_instance(index))
                    .is_some_and(|value| value.get_space().get_index() == stack.get_index())
            })
        });
    if !frame_storage {
        return Some(declared);
    }
    let offset = high
        .map(|high| high.get_symbol_offset().max(0))
        .unwrap_or(0);
    crate::kuna_stackviews::scalar_piece(
        fd.get_arch().types()?,
        Rc::clone(&declared),
        offset,
        value.get_size(),
    )
    .or(Some(declared))
}
