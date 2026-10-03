//! Keep an ordinary computed value separate from a global when joining them would move
//! the global's write across an observation or out of its basic block.

use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_indexaliasguard::LEVEL_GLOBAL;
use crate::merge::MergeContext;

pub fn keeps_apart(ctx: &mut dyn MergeContext, vn1: VarnodeId, vn2: VarnodeId) -> bool {
    let (Some(a), Some(b)) = (ctx.vn_high(vn1), ctx.vn_high(vn2)) else {
        return false;
    };
    if a == b {
        return false;
    }
    let (global, value) = match (ctx.high_is_persist(a), ctx.high_is_persist(b)) {
        (true, false) => (a, b),
        (false, true) => (b, a),
        _ => return false,
    };
    if ctx.high_is_addr_tied(value) {
        return false;
    }
    ctx.global_copy_moves_write(global, value)
}

pub fn copy_moves_write(data: &Funcdata, global: HighVariableId, value: HighVariableId) -> bool {
    if data.get_arch().index_alias_guard != LEVEL_GLOBAL {
        return false;
    }
    let Some(vh) = data.high_bank().get(value) else {
        return false;
    };
    if (0..vh.num_instances()).any(|i| {
        data.vbank()
            .get(vh.get_instance(i))
            .is_some_and(|v| v.is_global_load())
    }) {
        return false;
    }
    let stores: std::collections::BTreeSet<OpId> = (0..vh.num_instances())
        .filter_map(|i| data.vbank().get(vh.get_instance(i)))
        .flat_map(|v| v.descend_iter())
        .collect();
    for copy in stores {
        let Some(store) = data.obank().get(copy) else {
            continue;
        };
        if store.code() != OpCode::CPUI_COPY || store.is_dead() || store.is_return_copy() {
            continue;
        }
        let Some(global_vn) = store.get_out() else {
            continue;
        };
        let Some(g) = data.vbank().get(global_vn) else {
            continue;
        };
        if g.get_high() != Some(global)
            || !g.is_persist()
            || !data.global_store_guarded(g)
            || g.is_global_load()
        {
            continue;
        }
        let Some(source) = store.get_in(0) else {
            continue;
        };
        if write_moves(data, copy, source)
            && !crate::kuna_globalstorekeep::old_value_read_after(data, global_vn)
        {
            return true;
        }
    }
    false
}

pub fn write_moves(data: &Funcdata, copy: OpId, source: VarnodeId) -> bool {
    let Some(store) = data.obank().get(copy) else {
        return false;
    };
    let Some(global) = store.get_out().and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    let Some(def) = value_definition(data, source) else {
        return false;
    };
    let Some(start) = data.obank().get(def) else {
        return false;
    };
    if start.is_marker()
        || start.is_call()
        || matches!(
            start.code(),
            OpCode::CPUI_LOAD | OpCode::CPUI_COPY | OpCode::CPUI_CALLOTHER
        )
    {
        return false;
    }
    if start.get_parent() != store.get_parent() {
        return true;
    }
    let Some(block) = start.get_parent() else {
        return false;
    };
    let mut within = false;
    for op in data.bb_ops(block) {
        if op == def {
            within = true;
            continue;
        }
        if copy == op {
            break;
        }
        if within && observes(data, op, global) {
            return true;
        }
    }
    false
}

pub(crate) fn value_definition(data: &Funcdata, mut source: VarnodeId) -> Option<OpId> {
    let mut depth = 0;
    loop {
        let Some(v) = data.vbank().get(source) else {
            return None;
        };
        let Some(def) = v.get_def() else { return None };
        let Some(o) = data.obank().get(def) else {
            return None;
        };
        if o.code() != OpCode::CPUI_COPY || depth == 64 {
            return Some(def);
        }
        let Some(input) = o
            .get_in(0)
            .and_then(|v| data.vbank().get(v).map(|x| (v, x)))
        else {
            return Some(def);
        };
        if input.1.is_persist() || input.1.is_constant() || input.1.get_size() != v.get_size() {
            return Some(def);
        }
        source = input.0;
        depth += 1;
    }
}

fn observes(data: &Funcdata, op: OpId, global: &crate::varnode::Varnode) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if o.is_dead() || o.is_marker() {
        return false;
    }
    if o.is_call() || o.code() == OpCode::CPUI_CALLOTHER {
        return true;
    }
    if o.get_out()
        .and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.is_persist())
        && !o.is_return_copy()
    {
        return true;
    }
    if !matches!(o.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) {
        return false;
    }
    let Some(space) = o.get_in(0).and_then(|s| data.vbank().get(s)) else {
        return false;
    };
    if space.get_offset() != global.get_space().get_index() as u64 {
        return false;
    }
    let Some(pointer) = o.get_in(1).and_then(|p| data.vbank().get(p)) else {
        return false;
    };
    if !pointer.is_constant() || global.get_space().get_word_size() != 1 {
        return true;
    }
    let accessed = if o.code() == OpCode::CPUI_LOAD {
        o.get_out()
    } else {
        o.get_in(2)
    };
    let Some(size) = accessed
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.get_size())
    else {
        return true;
    };
    let Some(global_end) = global.get_offset().checked_add(global.get_size() as u64) else {
        return true;
    };
    let Some(access_end) = pointer.get_offset().checked_add(size as u64) else {
        return true;
    };
    pointer.get_offset() < global_end && global.get_offset() < access_end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{ArchContext, TypeOp};
    use crate::op::pcodeop_flags;
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use std::rc::Rc;

    // A global-state phi entering a cleanup loop is bookkeeping for prior
    // writes. Treating its outgoing COPY as a fresh computation made the
    // LS free-list loop print an extra byte-global assignment on every trip.
    #[test]
    fn a_loop_carried_global_state_copy_is_not_a_moved_write() {
        let mut spaces = AddrSpaceManager::new();
        spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        spaces
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        spaces.insert_space(Rc::clone(&ram)).unwrap();
        let arch = Rc::new(ArchContext::new(spaces));
        let mut data = Funcdata::new(
            "cleanup",
            "cleanup",
            arch,
            Address::new(Rc::clone(&ram), 0x1000),
            0x1000_0000,
            0x40,
        )
        .unwrap();
        let root = data.bblocks_ref().root.unwrap();
        let entry = data.bblocks_mut().new_block_basic(root);
        let cleanup = data.bblocks_mut().new_block_basic(root);
        let phi = data.new_op(2, Address::new(Rc::clone(&ram), 0x1000));
        data.op_set_opcode(
            phi,
            TypeOp::new(OpCode::CPUI_MULTIEQUAL, pcodeop_flags::marker, "MULTIEQUAL"),
        );
        for slot in 0..2 {
            let old = data.new_constant(1, slot as u64);
            data.op_set_input(phi, old, slot).unwrap();
        }
        let state = data.new_unique_out(1, phi).unwrap();
        data.op_insert(phi, entry, None);
        let copy = data.new_op(1, Address::new(Rc::clone(&ram), 0x1010));
        data.op_set_opcode(copy, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        data.op_set_input(copy, state, 0).unwrap();
        data.new_varnode_out(1, &Address::new(ram, 0x2000), copy)
            .unwrap();
        data.op_insert(copy, cleanup, None);
        assert!(!write_moves(&data, copy, state));
        // An independently computed byte, in the same block layout, really
        // would move its store onto an earlier path if merged with the global.
        data.op_set_opcode(phi, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
        assert!(write_moves(&data, copy, state));
    }
}
