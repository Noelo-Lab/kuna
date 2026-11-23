//! Render a guarded one-byte load from a verified Go string input.

use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;

pub(super) struct StringIndex {
    pub data: VarnodeId,
    pub len: VarnodeId,
    pub index: VarnodeId,
}

fn same_width_source(fd: &Funcdata, mut id: VarnodeId, width: i32) -> Option<VarnodeId> {
    for _ in 0..4 {
        let value = fd.vbank().get(id)?;
        if value.get_size() != width || value.is_volatile() {
            return None;
        }
        let Some(definition) = value.get_def() else {
            return Some(id);
        };
        let op = fd.obank().get(definition)?;
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return Some(id);
        }
        id = op.get_in(0)?;
    }
    None
}

fn byte_return(fd: &Funcdata, block: BlockId) -> Option<VarnodeId> {
    let tail = fd.obank().get(fd.bb_op_tail(block)?)?;
    if tail.code() != OpCode::CPUI_RETURN || tail.num_input() != 2 {
        return None;
    }
    same_width_source(fd, tail.get_in(1)?, 1)
}

fn is_formal_input(fd: &Funcdata, id: VarnodeId) -> bool {
    let Some(node) = fd.vbank().get(id) else {
        return false;
    };
    let proto = fd.get_func_proto();
    node.is_input()
        && (0..proto.num_params()).any(|i| {
            proto.get_param(i).is_some_and(|param| {
                param.is_type_locked()
                    && param.get_size() == node.get_size()
                    && param.get_address() == *node.get_addr()
            })
        })
}

pub(super) fn prove(fd: &Funcdata, load_id: OpId) -> Option<StringIndex> {
    let formal = fd.go_string_input()?;
    let proto = fd.get_func_proto();
    if !formal.matches(proto) || fd.bblocks_ref().root.is_none() || fd.bblocks_get_size() != 3 {
        return None;
    }
    if (0..proto.num_params()).any(|i| {
        proto
            .get_param(i)
            .is_some_and(|param| super::kuna_langgo::identifier(param.get_name()) == "unsafe")
    }) {
        return None;
    }
    let [data, len] = formal
        .storage
        .each_ref()
        .map(|address| fd.vbank().find_input(8, address));
    let (data, len) = (data?, len?);
    if fd.vbank().get(data)?.is_volatile() {
        return None;
    }
    let load = fd.obank().get(load_id)?;
    let address = fd.vbank().get(load.get_in(1)?)?;
    let space = fd.vbank().get(load.get_in(0)?)?;
    let ram = fd.get_arch().manage().get_default_data_space()?;
    let output = load.get_out()?;
    let byte = fd.vbank().get(output)?;
    if load.code() != OpCode::CPUI_LOAD
        || !space.is_constant()
        || space.get_offset() != ram.get_index() as u64
        || byte.get_size() != 1
        || byte.is_volatile()
        || address.get_size() != 8
        || address.is_volatile()
    {
        return None;
    }
    let add_id = address.get_def()?;
    let add = fd.obank().get(add_id)?;
    if add.code() != OpCode::CPUI_PTRADD
        || add.get_in(0)? != data
        || !fd
            .vbank()
            .get(add.get_in(2)?)
            .is_some_and(|stride| stride.is_constant() && stride.get_offset() == 1)
        || !fd.vbank().get(data)?.descend_iter().eq([add_id])
        || !address.descend_iter().eq([load_id])
    {
        return None;
    }
    let index = add.get_in(1)?;
    let node = fd.vbank().get(index)?;
    if !is_formal_input(fd, index)
        || node.get_size() != 8
        || node.is_volatile()
        || index == len
        || !matches!(
            node.get_type().get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
    {
        return None;
    }

    let entry = fd.bblocks_get_block(0);
    let graph = fd.bblocks_ref();
    let root = graph.block(entry);
    if fd.bblocks_block_start(entry) != *fd.get_address()
        || root.size_in() != 0
        || root.size_out() != 2
    {
        return None;
    }
    let branch_id = fd.bb_op_tail(entry)?;
    let branch = fd.obank().get(branch_id)?;
    if branch.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let compare_id = fd.vbank().get(branch.get_in(1)?)?.get_def()?;
    let compare = fd.obank().get(compare_id)?;
    if compare.get_parent()? != entry {
        return None;
    }
    let left = same_width_source(fd, compare.get_in(0)?, 8)?;
    let right = same_width_source(fd, compare.get_in(1)?, 8)?;
    let passes_when_true = match compare.code() {
        OpCode::CPUI_INT_LESS if [left, right] == [index, len] => true,
        OpCode::CPUI_INT_LESSEQUAL if [left, right] == [len, index] => false,
        _ => return None,
    };
    let (passing, fallback) = if passes_when_true ^ branch.is_boolean_flip() {
        (root.get_true_out(), root.get_false_out())
    } else {
        (root.get_false_out(), root.get_true_out())
    };
    if passing == fallback
        || load.get_parent()? != passing
        || add.get_parent()? != passing
        || [passing, fallback].iter().any(|&id| {
            let block = graph.block(id);
            block.size_in() != 1 || block.get_in(0) != entry || block.size_out() != 0
        })
        || byte_return(fd, passing)? != output
        || !is_formal_input(fd, byte_return(fd, fallback)?)
    {
        return None;
    }

    let mut count = 0;
    for id in fd.obank().iter_alive() {
        count += 1;
        if count > 16 {
            return None;
        }
        let op = fd.obank().get(id)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                let result = fd.vbank().get(op.get_out()?)?;
                if result.is_volatile() || result.is_persist() {
                    return None;
                }
            }
            OpCode::CPUI_RETURN => {}
            _ if [load_id, add_id, branch_id, compare_id].contains(&id) => {}
            _ => return None,
        }
    }
    Some(StringIndex { data, len, index })
}
