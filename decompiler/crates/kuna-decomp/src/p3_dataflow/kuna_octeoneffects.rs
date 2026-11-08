//! (kuna `octeoneffects`) Memory effects and types of the Octeon atomic,
//! reservation and ordering user operations declared by `octeonAtomics.sinc`.
//! An atomic update, load-linked, store-conditional or cache-block zero accesses
//! memory through its address operand; `atomic` (the default) guards writable
//! memory across those, `ordered` also across every ordering operation, and
//! `off` lifts the instructions upstream's MIPS spec also decodes (`ll`, `lld`,
//! `sc`, `scd`, `sync`) as upstream does, through the fixups in [`UPSTREAM`].
//! The option governs decompilation only: raw p-code and the analysis tier see
//! the user operations in every mode.
use std::rc::Rc;

use kuna_base::{
    address::Address,
    error::{KunaError, KunaResult},
    space::spacetype,
    types::{int4, uint4},
};
use kuna_num::opcodes::OpCode;

use crate::{
    context::OpId,
    dtype::{type_metatype, Datatype},
    funcdata::Funcdata,
    userop::UserOpManage,
};

/// What `option octeoneffects` guards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OcteonEffectsMode {
    /// Upstream's lifting: `ll`/`sc` as a plain load and an always-successful
    /// store, `sync` as `SYNC(n)`, and no memory effects or types for the rest.
    Off,
    /// Atomic updates, load-linked, store-conditionals and cache-block zeroing.
    #[default]
    Atomic,
    /// Those and every ordering operation.
    Ordered,
}

impl OcteonEffectsMode {
    pub fn as_str(self) -> &'static str {
        match self {
            OcteonEffectsMode::Off => "off",
            OcteonEffectsMode::Atomic => "atomic",
            OcteonEffectsMode::Ordered => "ordered",
        }
    }

    /// Parse an `option octeoneffects` value.
    pub fn parse(p1: &str) -> KunaResult<(OcteonEffectsMode, String)> {
        let mode = match p1 {
            "off" => OcteonEffectsMode::Off,
            "atomic" => OcteonEffectsMode::Atomic,
            "ordered" => OcteonEffectsMode::Ordered,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown octeoneffects value: {other} (expected off|atomic|ordered)"
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

/// One operation's kind, the width in bytes its address operand's pointer is
/// typed to (0 leaves it untyped), whether its second payload input is a
/// stored value, and its result width.
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

    /// Does the operation access memory through an address operand?
    pub fn addressed(self) -> bool {
        self.kind != Kind::Order
    }
}

const ORDER: Effect = Effect::new(Kind::Order, 0, false, 0);

/// A cache-block zero clears a whole block, not a value of any width, so its
/// address stays untyped.
const OPERATIONS: &[(&str, Effect)] = &[
    ("octeon_fetch_add32", Effect::new(Kind::Update, 4, true, 4)),
    ("octeon_fetch_add64", Effect::new(Kind::Update, 8, true, 8)),
    ("octeon_exchange32", Effect::new(Kind::Update, 4, true, 4)),
    ("octeon_exchange64", Effect::new(Kind::Update, 8, true, 8)),
    ("octeon_load_linked32", Effect::new(Kind::LoadLinked, 4, false, 4)),
    ("octeon_load_linked64", Effect::new(Kind::LoadLinked, 8, false, 8)),
    ("octeon_store_conditional32", Effect::new(Kind::StoreConditional, 4, true, 1)),
    ("octeon_store_conditional64", Effect::new(Kind::StoreConditional, 8, true, 1)),
    ("octeon_zero_cache_block", Effect::new(Kind::ZeroBlock, 0, false, 0)),
    ("octeon_zero_cache_block_transient", Effect::new(Kind::ZeroBlock, 0, false, 0)),
    ("octeon_sync", ORDER),
    ("octeon_synciobdma", ORDER),
    ("octeon_syncw", ORDER),
    ("octeon_syncws", ORDER),
    ("octeon_syncs", ORDER),
    ("octeon_sync_unknown", ORDER),
];

/// One callother fixup: the user operation, its output and input operand names,
/// and the p-code upstream's `mips32Instructions.sinc`/`mips64Instructions.sinc`
/// lift the instruction to.
pub struct Fixup {
    pub name: &'static [u8],
    pub output: &'static [u8],
    pub inputs: &'static [&'static [u8]],
    pub body: &'static [u8],
}

/// Upstream's lifting of each Octeon operation that replaced a generic MIPS
/// constructor. Registered at architecture bootstrap, before any option is
/// read, and applied only under `off` (`decompile_drive::is_injected_userop`).
pub const UPSTREAM: &[Fixup] = &[
    Fixup { name: b"octeon_load_linked32", output: b"result", inputs: &[b"addr"], body: b"result = *[ram]:4 addr;\n" },
    Fixup { name: b"octeon_load_linked64", output: b"result", inputs: &[b"addr"], body: b"result = *[ram]:8 addr;\n" },
    Fixup {
        name: b"octeon_store_conditional32",
        output: b"success",
        inputs: &[b"addr", b"value"],
        body: b"*[ram]:4 addr = value;\nsuccess = 1:1;\n",
    },
    Fixup {
        name: b"octeon_store_conditional64",
        output: b"success",
        inputs: &[b"addr", b"value"],
        body: b"*[ram]:8 addr = value;\nsuccess = 1:1;\n",
    },
    Fixup { name: b"octeon_sync", output: b"", inputs: &[], body: b"SYNC(0:1);\n" },
    Fixup { name: b"octeon_synciobdma", output: b"", inputs: &[], body: b"SYNC(2:1);\n" },
    Fixup { name: b"octeon_syncw", output: b"", inputs: &[], body: b"SYNC(4:1);\n" },
    Fixup { name: b"octeon_syncws", output: b"", inputs: &[], body: b"SYNC(5:1);\n" },
    Fixup { name: b"octeon_syncs", output: b"", inputs: &[], body: b"SYNC(6:1);\n" },
    Fixup { name: b"octeon_sync_unknown", output: b"", inputs: &[b"kind"], body: b"SYNC(kind);\n" },
];

/// The spec's Octeon operations, by user-op index. An operation counts if it
/// is uninjected or carries only this module's own [`UPSTREAM`] fixup (`ours`);
/// a compiler spec's own fixup wins. Only an Octeon language declares these
/// names. `off` resolves only a load-linked or store-conditional that has no
/// fixup at all: where [`UPSTREAM`] could not be compiled (ghidra mode has no
/// local `.sla`), it keeps its memory effect instead of turning opaque.
pub fn userop_ids(userops: &UserOpManage, mode: OcteonEffectsMode, ours: &[int4]) -> Vec<(u32, Effect)> {
    let off = mode == OcteonEffectsMode::Off;
    OPERATIONS
        .iter()
        .filter(|(_, effect)| !off || matches!(effect.kind, Kind::LoadLinked | Kind::StoreConditional))
        .filter_map(|(name, effect)| {
            userops
                .get_op_by_name(name.as_bytes())
                .filter(|u| match u.get_inject_id() {
                    None => true,
                    Some(id) => !off && ours.contains(&(id as int4)),
                })
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

/// Does `op` take a guard under the current mode?
pub fn guards(data: &Funcdata, op: OpId) -> bool {
    effect(data, op).is_some_and(|e| e.addressed() || data.get_arch().octeon_effects == OcteonEffectsMode::Ordered)
}

/// Selected operations that a memory observation must not move across.
pub fn memory_ops(data: &Funcdata) -> Vec<OpId> {
    if data.get_arch().octeon_memory_userops.is_empty() {
        return Vec::new();
    }
    data.obank().iter_code(OpCode::CPUI_CALLOTHER).filter(|&op| guards(data, op)).collect()
}

/// The bytes a cache-block zero clears: the data-cache block of every Octeon
/// generation.
const CACHE_BLOCK: i64 = 128;

/// Must the guard of `op` over the stack local `[addr, addr+size)` survive even
/// though no alias of that local escapes? Only at a cache-block zero, which
/// clears the whole aligned block holding its address and so reaches neighbors
/// of the byte it names. An address formed in this frame may reach any local.
/// One fixed at entry, a constant or an incoming value, points at or above the
/// entry stack pointer if into the stack at all, and its block starts less than
/// a block below that: only a local overlapping the top [`CACHE_BLOCK`] bytes
/// of the frame, or the caller's area above it, is in reach.
pub fn keeps_unaliased_guard(data: &Funcdata, op: OpId, addr: &Address, size: int4) -> bool {
    if !effect(data, op).is_some_and(|e| e.kind == Kind::ZeroBlock) {
        return false;
    }
    let Some(space) = addr.get_space().filter(|s| s.get_type() == spacetype::IPTR_SPACEBASE) else {
        return true;
    };
    formed_in_frame(data, op) || near_entry(addr.get_offset(), size, space.get_addr_size())
}

/// Does the stack range at `offset` (from the entry stack pointer, in an
/// `addr_size`-byte space) end less than [`CACHE_BLOCK`] bytes below that pointer?
fn near_entry(offset: u64, size: int4, addr_size: uint4) -> bool {
    let shift = 64 - 8 * addr_size.min(8);
    let signed = ((offset << shift) as i64) >> shift;
    signed.saturating_add(i64::from(size)) > -CACHE_BLOCK
}

/// Was the address operand of `op` possibly formed after this frame was
/// allocated: built from the stack pointer, loaded, or returned by a call (an
/// escaped frame address)? Constants and incoming values were not.
fn formed_in_frame(data: &Funcdata, op: OpId) -> bool {
    let Some(address) = data.obank().get(op).and_then(|o| o.get_in(1)) else {
        return true;
    };
    let stack_pointer = data.get_arch().manage().get_stack_space().and_then(|s| s.get_spacebase(0).ok());
    let mut pending = vec![address];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        let Some(vn) = data.vbank().get(id).filter(|_| seen.len() <= 64) else {
            return true;
        };
        if vn.is_constant() {
            continue;
        }
        if vn.is_spacebase() {
            return true;
        }
        if vn.is_input() {
            let sp = stack_pointer
                .as_ref()
                .filter(|sp| sp.space.as_ref().is_some_and(|s| vn.get_space().get_index() == s.get_index()));
            if sp.is_some_and(|sp| {
                vn.get_offset() < sp.offset.wrapping_add(u64::from(sp.size))
                    && sp.offset < vn.get_offset().wrapping_add(vn.get_size() as u64)
            }) {
                return true;
            }
            continue;
        }
        let Some(def) = vn.get_def().and_then(|d| data.obank().get(d)) else {
            return true;
        };
        let inputs = match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => 1,
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE => def.num_input(),
            _ => return true,
        };
        pending.extend((0..inputs).filter_map(|slot| def.get_in(slot)));
    }
    false
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
        .then(|| data.get_arch().types()?.get_base(effect.result, result_metatype(effect.result)).ok())
        .flatten()
}

/// A word result is `int`, as every constructor sign-extends it into its 64-bit
/// register; a doubleword result or a success byte is unsigned. Addresses and
/// stored values stay unsigned, so an `unsigned int *` operand needs no cast.
fn result_metatype(size: int4) -> type_metatype {
    if size == 4 {
        type_metatype::TYPE_INT
    } else {
        type_metatype::TYPE_UINT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_parse_and_print() {
        for mode in [OcteonEffectsMode::Off, OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            assert_eq!(OcteonEffectsMode::parse(mode.as_str()).unwrap().0, mode);
        }
        assert!(OcteonEffectsMode::parse("writes").is_err());
        assert_eq!(OcteonEffectsMode::default(), OcteonEffectsMode::Atomic);
    }

    #[test]
    fn every_operation_but_an_ordering_one_is_addressed() {
        let ordering: Vec<&str> = OPERATIONS.iter().filter(|(_, e)| !e.addressed()).map(|(n, _)| *n).collect();
        assert!(ordering.iter().all(|n| n.starts_with("octeon_sync")));
        assert_eq!(ordering.len(), 6);
        assert!(OPERATIONS.iter().filter(|(_, e)| e.kind == Kind::Order).all(|(_, e)| e.width == 0));
        assert!(OPERATIONS.iter().filter(|(_, e)| e.kind == Kind::ZeroBlock).all(|(_, e)| e.width == 0));
    }

    #[test]
    fn off_keeps_only_reservations_that_upstream_lifting_could_not_replace() {
        use crate::userop::UserPcodeOp;
        let mut userops = UserOpManage::new();
        userops.register_op(UserPcodeOp::new_unspecialized(b"octeon_load_linked32", 0)).unwrap();
        userops.register_op(UserPcodeOp::new_injected(b"octeon_store_conditional32", 1, 7)).unwrap();
        userops.register_op(UserPcodeOp::new_unspecialized(b"octeon_syncw", 2)).unwrap();
        userops.register_op(UserPcodeOp::new_unspecialized(b"octeon_fetch_add32", 3)).unwrap();
        let ids = |mode, ours: &[int4]| -> Vec<u32> {
            let mut ids: Vec<u32> = userop_ids(&userops, mode, ours).into_iter().map(|(id, _)| id).collect();
            ids.sort();
            ids
        };
        assert_eq!(ids(OcteonEffectsMode::Off, &[7]), vec![0]);
        assert_eq!(ids(OcteonEffectsMode::Off, &[]), vec![0]);
        assert_eq!(ids(OcteonEffectsMode::Atomic, &[7]), vec![0, 1, 2, 3]);
        assert_eq!(ids(OcteonEffectsMode::Atomic, &[]), vec![0, 2, 3]);
        assert_eq!(ids(OcteonEffectsMode::Ordered, &[7]), vec![0, 1, 2, 3]);
    }

    #[test]
    fn a_block_fixed_at_entry_reaches_only_the_top_cache_block_of_the_frame() {
        let at = |offset: i64| offset as u64;
        assert!(near_entry(at(-8), 8, 8));
        assert!(near_entry(at(-128), 8, 8));
        assert!(!near_entry(at(-136), 8, 8));
        assert!(!near_entry(at(-248), 8, 8));
        assert!(near_entry(at(16), 8, 8));
        assert!(near_entry(at(-128) & 0xffff_ffff, 4, 4));
        assert!(!near_entry(at(-132) & 0xffff_ffff, 4, 4));
    }

    #[test]
    fn upstream_fixups_cover_only_reservations_and_barriers() {
        for fixup in UPSTREAM {
            let name = std::str::from_utf8(fixup.name).unwrap();
            let (_, effect) = OPERATIONS.iter().find(|(n, _)| *n == name).expect("fixup names an operation");
            assert!(matches!(effect.kind, Kind::LoadLinked | Kind::StoreConditional | Kind::Order), "{name}");
        }
        assert_eq!(UPSTREAM.len(), 10);
    }
}
