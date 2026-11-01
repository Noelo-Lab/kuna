//! Octeon user operations observe memory independently of their scalar outputs.
use std::rc::Rc;

use kuna_base::{address::Address, space::spacetype, types::int4};
use kuna_num::opcodes::OpCode;

use crate::{
    context::{OpId, VarnodeId},
    dtype::{type_metatype, Datatype},
    funcdata::Funcdata,
    userop::UserOpManage,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effect {
    pub width: int4,
    pub value: bool,
    pub result: int4,
}

const OPERATIONS: &[(&str, Effect)] = &[
    (
        "octeon_fetch_add32",
        Effect {
            width: 4,
            value: true,
            result: 4,
        },
    ),
    (
        "octeon_fetch_add64",
        Effect {
            width: 8,
            value: true,
            result: 8,
        },
    ),
    (
        "octeon_exchange32",
        Effect {
            width: 4,
            value: true,
            result: 4,
        },
    ),
    (
        "octeon_exchange64",
        Effect {
            width: 8,
            value: true,
            result: 8,
        },
    ),
    (
        "octeon_load_linked32",
        Effect {
            width: 4,
            value: false,
            result: 4,
        },
    ),
    (
        "octeon_load_linked64",
        Effect {
            width: 8,
            value: false,
            result: 8,
        },
    ),
    (
        "octeon_store_conditional32",
        Effect {
            width: 4,
            value: true,
            result: 1,
        },
    ),
    (
        "octeon_store_conditional64",
        Effect {
            width: 8,
            value: true,
            result: 1,
        },
    ),
    (
        "octeon_zero_cache_block",
        Effect {
            width: 1,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_zero_cache_block_transient",
        Effect {
            width: 1,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_sync",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_syncio",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_syncioall",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_synciobdma",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_syncw",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_syncws",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_syncs",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
    (
        "octeon_sync_unknown",
        Effect {
            width: 0,
            value: false,
            result: 0,
        },
    ),
];

/// Resolve only this target's uninjected operations, once per architecture.
pub fn userop_ids(userops: &UserOpManage, archid: &str) -> Vec<(u32, Effect)> {
    let mut parts = archid.split(':');
    if parts.next() != Some("MIPS")
        || parts.next() != Some("BE")
        || parts.next() != Some("64")
        || !matches!(
            parts.next(),
            Some(
                "octeon"
                    | "octeonplus"
                    | "octeon2"
                    | "octeon3"
                    | "octeon-32addr"
                    | "octeonplus-32addr"
                    | "octeon2-32addr"
                    | "octeon3-32addr"
            )
        )
    {
        return Vec::new();
    }
    OPERATIONS
        .iter()
        .filter_map(|(name, effect)| {
            userops
                .get_op_by_name(name.as_bytes())
                .filter(|u| u.get_inject_id().is_none())
                .map(|u| (u.get_index() as u32, *effect))
        })
        .collect()
}

pub fn effect(data: &Funcdata, op: OpId) -> Option<Effect> {
    let known = &data.get_arch().octeon_memory_userops;
    if known.is_empty() {
        return None;
    }
    let op = data.obank().get(op)?;
    if op.code() != OpCode::CPUI_CALLOTHER || op.is_dead() {
        return None;
    }
    let index = data.vbank().get(op.get_in(0)?)?;
    if !index.is_constant() {
        return None;
    }
    known
        .iter()
        .find_map(|(id, effect)| (u64::from(*id) == index.get_offset()).then_some(*effect))
}

pub fn memory_space(data: &Funcdata, addr: &Address) -> bool {
    addr.get_space().is_some_and(|space| {
        space.get_type() == spacetype::IPTR_SPACEBASE
            || data
                .get_arch()
                .manage()
                .get_default_data_space()
                .is_some_and(|ram| ram.get_index() == space.get_index())
    })
}

pub fn memory_calls(data: &Funcdata) -> Vec<OpId> {
    if data.get_arch().octeon_memory_userops.is_empty() {
        return Vec::new();
    }
    data.obank()
        .iter_code(OpCode::CPUI_CALLOTHER)
        .filter(|&op| effect(data, op).is_some())
        .collect()
}

/// These operations may observe preceding stores and invalidate later reads.
pub fn guard_memory(data: &mut Funcdata, addr: &Address, size: int4, write: &mut Vec<VarnodeId>) {
    if data.get_arch().octeon_memory_userops.is_empty() || !memory_space(data, addr) {
        return;
    }
    for op in memory_calls(data) {
        let guard = data.new_indirect_op(op, addr, size, 0);
        data.obank_mut()
            .get_mut(guard)
            .expect("memory guard")
            .set_no_indirect_collapse();
        if let Some(input) = data.obank().get(guard).and_then(|o| o.get_in(0)) {
            data.vbank_mut()
                .get_mut(input)
                .expect("memory guard input")
                .set_active_heritage();
        }
        if let Some(output) = data.obank().get(guard).and_then(|o| o.get_out()) {
            let vn = data
                .vbank_mut()
                .get_mut(output)
                .expect("memory guard output");
            vn.set_active_heritage();
            vn.set_addr_force();
            write.push(output);
        }
    }
}

/// A potentially modified image value cannot be used as a compile-time constant.
pub fn blocks_readonly(data: &Funcdata, vn: VarnodeId) -> bool {
    !data.get_arch().octeon_memory_userops.is_empty()
        && data
            .vbank()
            .get(vn)
            .is_some_and(|v| memory_space(data, v.get_addr()))
        && data
            .obank()
            .iter_code(OpCode::CPUI_CALLOTHER)
            .any(|op| effect(data, op).is_some())
}

pub fn input_type(data: &Funcdata, op: OpId, slot: int4) -> Option<Rc<Datatype>> {
    let effect = effect(data, op)?;
    let input = data.vbank().get(data.obank().get(op)?.get_in(slot)?)?;
    let types = data.get_arch().types()?;
    if slot == 1 && effect.width != 0 {
        let space = data.get_arch().manage().get_default_data_space()?;
        if input.get_size() != space.get_addr_size() as int4 {
            return None;
        }
        let value = types
            .get_base(effect.width, type_metatype::TYPE_UINT)
            .ok()?;
        return types
            .get_type_pointer(input.get_size(), value, space.get_word_size())
            .ok();
    }
    if slot == 2 && effect.value {
        return types
            .get_base(input.get_size(), type_metatype::TYPE_UINT)
            .ok();
    }
    None
}

pub fn output_type(data: &Funcdata, op: OpId) -> Option<Rc<Datatype>> {
    let effect = effect(data, op)?;
    let output = data.vbank().get(data.obank().get(op)?.get_out()?)?;
    (output.get_size() == effect.result)
        .then(|| {
            data.get_arch()
                .types()?
                .get_base(effect.result, type_metatype::TYPE_UINT)
                .ok()
        })
        .flatten()
}
