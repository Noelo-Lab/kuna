//! Byte stores retain a user-declared layout across incompatible object views.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(crate) struct Assignment {
    pub output: VarnodeId,
    pub operand: Option<VarnodeId>,
    pub raw_constant: bool,
    pub datatype: Rc<Datatype>,
    pub size: i32,
    pub name: String,
    pub offset: i32,
    pub pointer_size: i32,
}

pub(crate) fn assignment(fd: &Funcdata, op: OpId) -> Option<Assignment> {
    if !fd.get_arch().stack_views {
        return None;
    }
    let operation = fd.obank().get(op)?;
    if operation.is_marker() {
        return None;
    }
    let output = operation.get_out()?;
    let value = fd.vbank().get(output)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    let size = value.get_size();
    if !matches!(size, 1 | 2 | 4 | 8 | 16) {
        return None;
    }
    let scope = fd.get_scope_local()?;
    let (address, linked_symbol) = if value.get_space().get_index() == stack.get_index() {
        (value.get_addr().clone(), None)
    } else {
        let high = fd.high_bank().get(value.get_high()?)?;
        let symbol = high.kuna_link_symbol()?;
        if scope.database().symbol(symbol).scope != scope.scope_id() {
            return None;
        }
        let offset = high.kuna_symbol_offset();
        if offset < 0 {
            return None;
        }
        let mut storage = scope
            .symbol_storage(symbol)
            .into_iter()
            .filter(|(address, width)| {
                address
                    .get_space()
                    .is_some_and(|space| space.get_index() == stack.get_index())
                    && offset.checked_add(size).is_some_and(|end| end <= *width)
            });
        let (base, _) = storage.next()?;
        if storage.next().is_some() {
            return None;
        }
        (
            Address::new(
                Rc::clone(&stack),
                stack.wrap_offset(base.get_offset().wrapping_add(offset as u64)),
            ),
            Some(symbol),
        )
    };
    let info = scope.query_container_for_link_width(&address, size, &Address::new_invalid())?;
    if linked_symbol.is_some_and(|symbol| symbol != info.symbol) {
        return None;
    }
    let root = info.sym_type.as_ref()?;
    let factory = fd.get_arch().types()?;
    let byte_member = factory
        .get_exact_piece(Rc::clone(root), info.sym_off, size)
        .ok()
        .flatten()
        .is_some_and(|piece| {
            let storage = piece.get_partial_base().unwrap_or(piece);
            super::kuna_bytearrayc::byte_array(&storage)
        });
    if linked_symbol.is_some() && !byte_member {
        return None;
    }
    if !scope.database().symbol(info.symbol).is_type_locked()
        || info.is_name_undefined
        || root.get_name().starts_with("stack_views_")
        || (!byte_member && !incompatible_views(fd, &info.entry_addr, root))
    {
        return None;
    }
    let mut operand = matches!(operation.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST)
        .then(|| operation.get_in(0))
        .flatten();
    let mut array_cast = operation.code() == OpCode::CPUI_CAST
        && super::kuna_bytearrayc::byte_array(value.get_type());
    if byte_member {
        // The inserted array CAST describes storage, not a numeric conversion.
        // Copy the scalar representation underneath it into the declared member.
        while let Some(input) = operand {
            let value = fd.vbank().get(input)?;
            if !value.is_implied() || !super::kuna_bytearrayc::byte_array(value.get_type()) {
                break;
            }
            let Some(cast) = value.get_def().and_then(|id| fd.obank().get(id)) else {
                break;
            };
            if cast.code() != OpCode::CPUI_CAST {
                break;
            }
            let source = cast.get_in(0)?;
            if fd.vbank().get(source)?.get_size() != size {
                return None;
            }
            operand = Some(source);
            array_cast = true;
        }
    }
    if linked_symbol.is_some() && !array_cast {
        return None;
    }
    let raw_constant = operand.is_some_and(|id| {
        fd.vbank().get(id).is_some_and(|value| value.is_constant())
            && !crate::kuna_stackviews::pointer_origin(fd, id)
    });
    let memory_source = byte_member
        && operand
            .and_then(|input| loaded_bytes(fd, input))
            .or_else(|| load_pointer(fd, op))
            .is_some();
    let datatype = if memory_source {
        // This is a raw representation, not a scalar C value. The array type
        // selects memory-source emission without imposing the member's type.
        factory
            .get_type_array(size, factory.get_base(1, type_metatype::TYPE_UINT).ok()?)
            .ok()?
    } else if raw_constant {
        factory.get_base(size, type_metatype::TYPE_UINT).ok()?
    } else if let Some(input) = operand {
        let ty = if crate::kuna_stackviews::pointer_origin(fd, input) {
            let read_type = fd.vn_type_read_facing(input, op);
            if read_type.get_metatype() == type_metatype::TYPE_PTR {
                read_type
            } else {
                factory
                    .get_type_pointer(size, factory.get_type_void().ok()?, 1)
                    .ok()?
            }
        } else {
            crate::printc::declared_variable_type(fd, fd.get_arch().decl_high_type, input)
                .unwrap_or_else(|| fd.vn_type_read_facing(input, op))
        };
        if scalar(&ty) && ty.get_size() == size {
            ty
        } else {
            return None;
        }
    } else if crate::kuna_stackviews::pointer_origin(fd, output) {
        factory
            .get_type_pointer(size, factory.get_type_void().ok()?, 1)
            .ok()?
    } else {
        let ty = crate::typeop::type_op_info(operation.code())
            .get_output_local(factory, size)
            .ok()?;
        if !scalar(&ty) {
            return None;
        }
        ty
    };
    Some(Assignment {
        output,
        operand,
        raw_constant,
        datatype,
        size,
        name: info.display_name,
        offset: info.sym_off,
        pointer_size: stack.get_addr_size() as i32,
    })
}

fn incompatible_views(fd: &Funcdata, address: &Address, root: &Rc<Datatype>) -> bool {
    let Some(factory) = fd.get_arch().types() else {
        return false;
    };
    crate::kuna_stackobjects::objects(fd).iter().any(|object| {
        if object.address.get_space().map(|space| space.get_index())
            != address.get_space().map(|space| space.get_index())
        {
            return false;
        }
        let offset = object
            .address
            .get_offset()
            .wrapping_sub(address.get_offset());
        if offset
            .checked_add(object.size as u64)
            .is_none_or(|end| end > root.get_size() as u64)
        {
            return false;
        }
        object.uses.iter().any(|use_| {
            let ty = crate::kuna_stackviews::scalar_piece(
                factory,
                Rc::clone(root),
                offset as i32,
                object.size,
            )
            .or_else(|| {
                factory
                    .get_exact_piece(Rc::clone(root), offset as i32, object.size)
                    .ok()
                    .flatten()
            });
            ty.is_none_or(|ty| {
                !Rc::ptr_eq(&ty, &use_.datatype)
                    && !(integer(&ty)
                        && integer(&use_.datatype)
                        && ty.get_size() == use_.datatype.get_size())
            })
        })
    })
}

fn integer(datatype: &Datatype) -> bool {
    matches!(
        datatype.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    )
}

fn scalar(datatype: &Datatype) -> bool {
    matches!(
        datatype.get_metatype(),
        type_metatype::TYPE_BOOL
            | type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_FLOAT
            | type_metatype::TYPE_PTR
    )
}

/// Follow implied representation-only wrappers to the original memory read.
/// Numeric conversions and computations must still be evaluated as scalars.
pub(super) fn loaded_bytes(fd: &Funcdata, mut input: VarnodeId) -> Option<(VarnodeId, OpId, i32)> {
    let size = fd.vbank().get(input)?.get_size();
    for _ in 0..32 {
        let value = fd.vbank().get(input)?;
        if !value.is_implied() || value.is_volatile() || value.get_size() != size {
            return None;
        }
        let definition = value.get_def()?;
        let operation = fd.obank().get(definition)?;
        match operation.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => input = operation.get_in(0)?,
            OpCode::CPUI_LOAD => return load_pointer(fd, definition),
            _ => return None,
        }
    }
    None
}

/// A LOAD can itself define the frame member after SSA coalescing.
pub(super) fn load_pointer(fd: &Funcdata, definition: OpId) -> Option<(VarnodeId, OpId, i32)> {
    let operation = fd.obank().get(definition)?;
    if operation.code() != OpCode::CPUI_LOAD || fd.vbank().get(operation.get_out()?)?.is_volatile()
    {
        return None;
    }
    let space_id = fd.vbank().get(operation.get_in(0)?)?;
    if !space_id.is_constant() {
        return None;
    }
    let space = fd
        .get_arch()
        .manage()
        .get_space(i32::try_from(space_id.get_offset()).ok()?)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    if space.get_word_size() != 1 || space.is_big_endian() != stack.is_big_endian() {
        return None;
    }
    Some((
        operation.get_in(1)?,
        definition,
        space.get_addr_size() as i32,
    ))
}

/// The memmove source operand: implied pointer CASTs are peeled, and the
/// `unsigned char *` conversion is requested only for a non-pointer expression.
pub(super) fn memmove_source(
    fd: &Funcdata,
    mut pointer: VarnodeId,
    mut reader: OpId,
) -> Option<(VarnodeId, OpId, bool)> {
    let typed_pointer = |vn: VarnodeId, op: OpId| {
        let value = fd.vbank().get(vn)?;
        if value.is_constant() {
            return Some(false);
        }
        let ty = crate::printc::declared_variable_type(fd, fd.get_arch().decl_high_type, vn)
            .unwrap_or_else(|| fd.vn_type_read_facing(vn, op));
        Some(
            ty.get_metatype() == type_metatype::TYPE_PTR
                && ty
                    .get_ptr_to()
                    .is_some_and(|to| to.get_metatype() != type_metatype::TYPE_CODE),
        )
    };
    for _ in 0..8 {
        let value = fd.vbank().get(pointer)?;
        let Some(cast) = value
            .is_implied()
            .then(|| value.get_def())
            .flatten()
            .and_then(|id| fd.obank().get(id).map(|op| (id, op)))
        else {
            break;
        };
        if cast.1.code() != OpCode::CPUI_CAST {
            break;
        }
        let input = cast.1.get_in(0)?;
        if fd.vbank().get(input)?.get_size() != value.get_size()
            || !typed_pointer(input, cast.0)?
        {
            break;
        }
        pointer = input;
        reader = cast.0;
    }
    Some((pointer, reader, !typed_pointer(pointer, reader)?))
}
