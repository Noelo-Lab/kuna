//! Retain physical stack-read locations through SSA copy propagation.

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct MachineRead {
    pub(crate) block: BlockId,
    pub(crate) space: i32,
    pub(crate) offset: u64,
    pub(crate) size: i32,
}

#[derive(Default)]
pub(crate) struct ReadScopes {
    pub(crate) lowered: std::collections::HashMap<OpId, MachineRead>,
    pub(crate) by_call: std::collections::HashMap<OpId, Vec<MachineRead>>,
    /// (kuna `loadguardreads`) The reads of LOAD guards that build no `COPY`.
    pub(crate) guarded: crate::kuna_fasthash::FxHashSet<MachineRead>,
}

impl ReadScopes {
    /// Every recorded read: lowered stack reads, then LOAD guard reads.
    pub(crate) fn reads(&self) -> impl Iterator<Item = &MachineRead> + '_ {
        self.lowered.values().chain(self.guarded.iter())
    }
}

pub(crate) fn mark_stack_read(data: &mut Funcdata, op: OpId) {
    if !data.get_arch().guarded_read_trim || data.num_calls() == 0 {
        return;
    }
    let Some(operation) = data.obank().get(op) else {
        return;
    };
    let Some(block) = operation.get_parent() else {
        return;
    };
    let Some(value) = operation.get_in(0).and_then(|v| data.vbank().get(v)) else {
        return;
    };
    if value.get_space().get_type() != spacetype::IPTR_SPACEBASE {
        return;
    }
    let read = MachineRead {
        block,
        space: value.get_space().get_index(),
        offset: value.get_offset(),
        size: value.get_size(),
    };
    data.kuna_guarded_reads.lowered.entry(op).or_insert(read);
    if let Some(operation) = data.obank_mut().get_mut(op) {
        operation.set_additional_flag(crate::op::pcodeop_addlflags::kuna_stack_read);
    }
}

/// (kuna `loadguardreads`) Record the read a LOAD guard `COPY` of the stack
/// range `addr`/`size` in front of `load` would have marked.
pub(crate) fn mark_guarded_read(data: &mut Funcdata, load: OpId, addr: &kuna_base::address::Address, size: i32) {
    if !data.get_arch().guarded_read_trim || data.num_calls() == 0 {
        return;
    }
    let Some(space) = addr.get_space().filter(|s| s.get_type() == spacetype::IPTR_SPACEBASE) else {
        return;
    };
    let Some(block) = data.obank().get(load).and_then(|op| op.get_parent()) else {
        return;
    };
    let read = MachineRead { block, space: space.get_index(), offset: addr.get_offset(), size };
    data.kuna_guarded_reads.guarded.insert(read);
}

#[inline]
pub(crate) fn record_propagated_read(data: &mut Funcdata, op: OpId, source: VarnodeId) {
    let Some(operation) = data.obank().get(op) else {
        return;
    };
    if operation.get_addlflags() & crate::op::pcodeop_addlflags::kuna_stack_read == 0 {
        return;
    }
    let Some(value) = data.vbank().get(source) else {
        return;
    };
    if value.get_space().get_type() != spacetype::IPTR_SPACEBASE {
        return;
    }
    let Some(definition) = value.get_def().and_then(|op| data.obank().get(op)) else {
        return;
    };
    if definition.code() != OpCode::CPUI_INDIRECT {
        return;
    }
    let Some(effect) = definition.get_in(1).and_then(|v| data.vbank().get(v)) else {
        return;
    };
    if effect.get_space().get_type() != spacetype::IPTR_IOP {
        return;
    }
    let effect = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
    if !data.obank().get(effect).is_some_and(|op| op.is_call()) {
        return;
    }
    let Some(read) = data.kuna_guarded_reads.lowered.get(&op).cloned() else {
        return;
    };
    let reads = data.kuna_guarded_reads.by_call.entry(effect).or_default();
    if !reads.contains(&read) {
        reads.push(read);
    }
}

pub(crate) fn record_current_read(data: &mut Funcdata, op: OpId) {
    let Some(operation) = data.obank().get(op) else {
        return;
    };
    if operation.get_addlflags() & crate::op::pcodeop_addlflags::kuna_stack_read == 0 {
        return;
    }
    let count = operation.num_input();
    for slot in 0..count {
        if let Some(source) = data.obank().get(op).and_then(|op| op.get_in(slot)) {
            record_propagated_read(data, op, source);
        }
    }
}
