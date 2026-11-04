//! (kuna `octeoneffects`) Memory effects and types of the Octeon atomic,
//! reservation and ordering user operations declared by `octeonAtomics.sinc`.
//! An atomic update, a store-conditional or a cache-block zero writes memory
//! through its address operand; `writes` (the default) guards writable memory
//! across those, `ordered` also across the load-linked and ordering operations,
//! and `off` leaves every user operation without a memory effect.
use std::rc::Rc;

use kuna_base::{
    address::Address,
    error::{KunaError, KunaResult},
    types::{int4, uint4},
};
use kuna_num::opcodes::OpCode;

use crate::{
    context::{OpId, VarnodeId},
    dtype::{type_metatype, Datatype},
    funcdata::Funcdata,
    userop::UserOpManage,
};

/// What `option octeoneffects` guards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OcteonEffectsMode {
    /// No memory effects or types: every user operation is opaque.
    Off,
    /// Atomic updates, store-conditionals and cache-block zeroing.
    #[default]
    Writes,
    /// Those, the load-linked and every ordering operation.
    Ordered,
}

impl OcteonEffectsMode {
    pub fn as_str(self) -> &'static str {
        match self {
            OcteonEffectsMode::Off => "off",
            OcteonEffectsMode::Writes => "writes",
            OcteonEffectsMode::Ordered => "ordered",
        }
    }

    /// Parse an `option octeoneffects` value.
    pub fn parse(p1: &str) -> KunaResult<(OcteonEffectsMode, String)> {
        let mode = match p1 {
            "off" => OcteonEffectsMode::Off,
            "writes" => OcteonEffectsMode::Writes,
            "ordered" => OcteonEffectsMode::Ordered,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown octeoneffects value: {other} (expected off|writes|ordered)"
                )))
            }
        };
        Ok((mode, format!("Octeon memory effects set to {}", mode.as_str())))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Fetch-and-add or exchange: reads and writes the addressed value.
    Update,
    /// Load-linked: reads the addressed value and opens a reservation.
    LoadLinked,
    /// Store-conditional: may write the addressed value.
    StoreConditional,
    /// Zeroes the cache block holding the address.
    ZeroBlock,
    /// Orders memory accesses; touches no address of its own.
    Order,
}

/// One operation's address-operand access width in bytes (0 for none),
/// whether its second payload input is a stored value, and its result width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effect {
    pub kind: Kind,
    pub width: int4,
    pub value: bool,
    pub result: int4,
}

impl Effect {
    const fn new(kind: Kind, width: int4, value: bool, result: int4) -> Effect {
        Effect { kind, width, value, result }
    }

    /// Does the operation write memory through its address operand?
    pub fn writes(self) -> bool {
        matches!(self.kind, Kind::Update | Kind::StoreConditional | Kind::ZeroBlock)
    }
}

const ORDER: Effect = Effect::new(Kind::Order, 0, false, 0);

const OPERATIONS: &[(&str, Effect)] = &[
    ("octeon_fetch_add32", Effect::new(Kind::Update, 4, true, 4)),
    ("octeon_fetch_add64", Effect::new(Kind::Update, 8, true, 8)),
    ("octeon_exchange32", Effect::new(Kind::Update, 4, true, 4)),
    ("octeon_exchange64", Effect::new(Kind::Update, 8, true, 8)),
    ("octeon_load_linked32", Effect::new(Kind::LoadLinked, 4, false, 4)),
    ("octeon_load_linked64", Effect::new(Kind::LoadLinked, 8, false, 8)),
    ("octeon_store_conditional32", Effect::new(Kind::StoreConditional, 4, true, 1)),
    ("octeon_store_conditional64", Effect::new(Kind::StoreConditional, 8, true, 1)),
    ("octeon_zero_cache_block", Effect::new(Kind::ZeroBlock, 1, false, 0)),
    ("octeon_zero_cache_block_transient", Effect::new(Kind::ZeroBlock, 1, false, 0)),
    ("octeon_sync", ORDER),
    ("octeon_syncio", ORDER),
    ("octeon_syncioall", ORDER),
    ("octeon_synciobdma", ORDER),
    ("octeon_syncw", ORDER),
    ("octeon_syncws", ORDER),
    ("octeon_syncs", ORDER),
    ("octeon_sync_unknown", ORDER),
];

/// The spec's uninjected Octeon operations, by user-op index. Only an Octeon
/// language declares these names; `off` resolves none.
pub fn userop_ids(userops: &UserOpManage, mode: OcteonEffectsMode) -> Vec<(u32, Effect)> {
    if mode == OcteonEffectsMode::Off {
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

/// Does `op` affect memory under the current mode?
pub fn guards(data: &Funcdata, op: OpId) -> bool {
    effect(data, op).is_some_and(|e| e.writes() || data.get_arch().octeon_effects == OcteonEffectsMode::Ordered)
}

/// Could `op` change the RAM or stack range at `addr` (heritage's
/// indirect-effect question for a CALLOTHER)?
pub fn affects(data: &Funcdata, op: OpId, addr: &Address) -> bool {
    guards(data, op) && crate::kuna_memoryguard::memory_space(data, addr)
}

/// Guard writable memory across the operations the mode selects.
pub fn guard_memory(data: &mut Funcdata, flags: uint4, addr: &Address, size: int4, write: &mut Vec<VarnodeId>) {
    if data.get_arch().octeon_memory_userops.is_empty() || !crate::kuna_memoryguard::guarded(data, flags, addr) {
        return;
    }
    let ops: Vec<OpId> = data
        .obank()
        .iter_code(OpCode::CPUI_CALLOTHER)
        .filter(|&op| guards(data, op))
        .collect();
    crate::kuna_memoryguard::guard(data, &ops, flags, addr, size, write);
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
        let value = types.get_base(effect.width, type_metatype::TYPE_UINT).ok()?;
        return types.get_type_pointer(input.get_size(), value, space.get_word_size()).ok();
    }
    if slot == 2 && effect.value {
        return types.get_base(input.get_size(), type_metatype::TYPE_UINT).ok();
    }
    None
}

pub fn output_type(data: &Funcdata, op: OpId) -> Option<Rc<Datatype>> {
    let effect = effect(data, op)?;
    let output = data.vbank().get(data.obank().get(op)?.get_out()?)?;
    (output.get_size() == effect.result)
        .then(|| data.get_arch().types()?.get_base(effect.result, type_metatype::TYPE_UINT).ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_parse_and_print() {
        for mode in [OcteonEffectsMode::Off, OcteonEffectsMode::Writes, OcteonEffectsMode::Ordered] {
            assert_eq!(OcteonEffectsMode::parse(mode.as_str()).unwrap().0, mode);
        }
        assert!(OcteonEffectsMode::parse("on").is_err());
        assert_eq!(OcteonEffectsMode::default(), OcteonEffectsMode::Writes);
    }

    #[test]
    fn only_updates_store_conditionals_and_block_zeroing_write() {
        let writers: Vec<&str> = OPERATIONS.iter().filter(|(_, e)| e.writes()).map(|(n, _)| *n).collect();
        assert_eq!(
            writers,
            [
                "octeon_fetch_add32",
                "octeon_fetch_add64",
                "octeon_exchange32",
                "octeon_exchange64",
                "octeon_store_conditional32",
                "octeon_store_conditional64",
                "octeon_zero_cache_block",
                "octeon_zero_cache_block_transient",
            ]
        );
        assert!(OPERATIONS.iter().filter(|(_, e)| e.kind == Kind::Order).all(|(_, e)| e.width == 0));
    }
}
