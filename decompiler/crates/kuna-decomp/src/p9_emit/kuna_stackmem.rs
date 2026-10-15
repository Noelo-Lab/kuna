//! C pointer casts do not make differently typed frame storage alias safely.
//! Copy bytes through reinterpreted pointers or indexed frame fields.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;
use std::rc::Rc;

pub(crate) struct ByteAddress {
    pub name: String,
    pub offset: i32,
    pub terms: Vec<crate::kuna_stackranges::IndexTerm>,
    pub pointer_size: i32,
}

pub(crate) struct Access {
    pub pointer: VarnodeId,
    pub value: Option<VarnodeId>,
    pub datatype: Rc<Datatype>,
    pub size: i32,
    pub address: Option<ByteAddress>,
}

pub(crate) fn access(fd: &Funcdata, op: OpId) -> Option<Access> {
    if !fd.get_arch().stack_views {
        return None;
    }
    let operation = fd.obank().get(op)?;
    let pointer = operation.get_in(1)?;
    let (value, datatype, size) = match operation.code() {
        OpCode::CPUI_STORE => {
            let value = operation.get_in(2)?;
            let size = fd.vbank().get(value)?.get_size();
            (Some(value), fd.vn_type_read_facing(value, op), size)
        }
        OpCode::CPUI_LOAD => {
            let output = operation.get_out()?;
            (
                None,
                fd.vn_type_def_facing(output),
                fd.vbank().get(output)?.get_size(),
            )
        }
        _ => return None,
    };
    let address = byte_address(fd, pointer, size);
    let indexed = address
        .as_ref()
        .is_some_and(|address| !address.terms.is_empty());
    if !matches!(size, 1 | 2 | 4 | 8)
        || (size == 1 && !indexed)
        || datatype.get_size() != size
        || !matches!(
            datatype.get_metatype(),
            type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_FLOAT
                | type_metatype::TYPE_PTR
                | type_metatype::TYPE_UNKNOWN
        )
        || (!indexed && !pointer_cast(fd, pointer))
    {
        return None;
    }
    Some(Access {
        pointer,
        value,
        datatype,
        size,
        address,
    })
}

fn byte_address(fd: &Funcdata, pointer: VarnodeId, size: i32) -> Option<ByteAddress> {
    use kuna_base::address::{calc_mask, Address};
    let address = crate::kuna_stackranges::frame_address(fd, pointer)?;
    let space = fd.get_arch().manage().get_stack_space()?;
    let at = Address::new(
        Rc::clone(space),
        address.offset as u64 & calc_mask(address.pointer_size),
    );
    let info = fd
        .get_scope_local()?
        .query_container_for_link(&at, &Address::new_invalid())?;
    let datatype = info.sym_type?;
    if info.is_name_undefined || !datatype.get_name().starts_with("stack_views_") {
        return None;
    }
    let first = i64::from(info.sym_off).checked_sub(address.offset.checked_sub(address.first)?)?;
    let last = i64::from(info.sym_off)
        .checked_add(address.last.checked_sub(address.offset)?)?
        .checked_add(i64::from(size))?;
    if first < 0 || last > i64::from(datatype.get_size()) {
        return None;
    }
    Some(ByteAddress {
        name: info.display_name,
        offset: info.sym_off,
        terms: address.terms,
        pointer_size: address.pointer_size,
    })
}

fn pointer_cast(fd: &Funcdata, root: VarnodeId) -> bool {
    let mut pending = vec![root];
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if visited.len() == 64 {
            return false;
        }
        if !visited.insert(id) {
            continue;
        }
        let Some(value) = fd.vbank().get(id) else {
            continue;
        };
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else {
            continue;
        };
        if op.code() == OpCode::CPUI_CAST
            && value.get_type().get_metatype() == type_metatype::TYPE_PTR
        {
            return true;
        }
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => {
                pending.extend(op.get_in(0))
            }
            OpCode::CPUI_MULTIEQUAL => {
                pending.extend((0..op.num_input()).filter_map(|i| op.get_in(i)))
            }
            _ => (),
        }
    }
    false
}
