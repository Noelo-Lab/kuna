//! Cross-wave seam placeholders for the W3 IR data-model.
//!
//! Per ADR 0001 the IR is three `Funcdata`-owned slotmap arenas keyed by the
//! newtypes [`VarnodeId`], [`OpId`], [`BlockId`].  The keys live here (the one
//! place every W3 serial-chain file can name them); the arenas and the
//! `Funcdata`-mediated mutation API are filled by `funcdata`/`op`/`block`.
//!
//! Everything else in this file is a forward-reference placeholder for a type
//! that `varnode.hh` mentions but that belongs to a later wave, annotated with
//! the wave that fills it.  These let `varnode.rs` transcribe the member layout
//! and link structure faithfully without pulling in the unported subsystems.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use slotmap::new_key_type;

new_key_type! {
    /// Arena key for a `Varnode` (ADR 0001).  Generational, so a stale handle
    /// is a caught error, not a use-after-free.
    pub struct VarnodeId;

    /// Arena key for a `PcodeOp` (ADR 0001).
    ///
    /// SEAM(W3): the `PcodeOp` arena and its accessors are `op`/`funcdata`'s
    /// (`w3-ir-op`).  `varnode.rs` stores `OpId`s for `def` and `descend`
    /// links, exactly as the C++ stores `PcodeOp *`.
    pub struct OpId;

    /// Arena key for a `FlowBlock` (ADR 0001).
    ///
    /// SEAM(W3): filled by `block` (`w3-ir-block`); declared here so the shared
    /// key set is in one place.
    pub struct BlockId;
}

/// HighVariable — the high-level variable an instance of which a Varnode is.
///
/// SEAM(W7): filled by `merge`/`HighVariable` (`w7`).  `Varnode` holds an
/// `Option<HighVariableId>` where the C++ holds a `HighVariable *`; until W7
/// every varnode's high is `None` (the C++ `getHigh()` likewise throws until
/// merging builds it).  Modelled as an opaque arena key so the eventual
/// HighVariable arena can slot in without touching the Varnode layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HighVariableId(pub u32);

/// Cover — the def/use address coverage of a Varnode.
///
/// SEAM(W7): filled by `cover`/`Cover` (`w7`).  The C++ `Varnode::cover` is a
/// lazily-built, mutable `Cover *`; the W3 data-model only needs to track the
/// presence/absence and the `coverdirty` flag (carried in `flags`), so a unit
/// placeholder suffices until W7 supplies the real geometry.
#[derive(Debug, Clone, Default)]
pub struct Cover;

/// TypeOp — the behavioral class (opcode) attached to a [`PcodeOp`].
///
/// SEAM(W6): filled by `typeop`/`type` (`w6`).  The C++ `TypeOp` (`typeop.hh`)
/// bundles an [`OpCode`] value, a cached property-flag word
/// (`getFlags()`/`opflags`, transcribed as [`type_op_flags`]), a display
/// `name`, an `OpBehavior` for emulation, and the `TypeFactory`-backed local
/// type calculators (`getOutputLocal`/`getInputLocal`).
///
/// `op.cc` reaches only a thin slice of that surface: `PcodeOp::setOpcode`
/// caches `getFlags()` into the op's `flags`, `code()` returns `getOpcode()`,
/// the op-code lists key on `code()`, and the print/eval/type-local methods
/// dispatch through the `OpBehavior`/`TypeFactory`.  This seam carries exactly
/// the first three (`opcode`/`flags`/`name`); the emulation+type-local methods
/// stay in W6 (the W3 `collapse`/`executeSimple`/`outputTypeLocal` paths defer
/// or take the behavior as an explicit argument — see `op.rs`).
#[derive(Debug, Clone)]
pub struct TypeOp {
    /// The op-code value (C++ `TypeOp::opcode`).  // SEAM(W6)
    pub opcode: OpCode,
    /// Cached pcode-op properties for this op-code (C++ `TypeOp::opflags`,
    /// the `PcodeOp::*` flag bits `setOpcode` ORs in).  // SEAM(W6)
    pub flags: uint4,
    /// Symbol denoting this operation (C++ `TypeOp::name`).  // SEAM(W6)
    pub name: String,
}

impl TypeOp {
    /// Construct a minimal behavioral-class skeleton (SEAM(W6)).
    pub fn new(opcode: OpCode, flags: uint4, name: impl Into<String>) -> TypeOp {
        TypeOp { opcode, flags, name: name.into() }
    }

    /// Get the op-code value (C++ `TypeOp::getOpcode`).  // SEAM(W6)
    pub fn get_opcode(&self) -> OpCode {
        self.opcode
    }

    /// Get the properties associated with the op-code (C++ `TypeOp::getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }

    /// Get the display name of the op-code (C++ `TypeOp::getName`).  // SEAM(W6)
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// Global configuration data for the program being decompiled (C++
/// `Architecture`, owned by `Funcdata` as `glb`).
///
/// SEAM(W4): the full `decompiler/cpp/architecture.{hh,cc}` `Architecture` is a
/// large W4 subsystem (the address-space manager, the `TypeFactory`, the symbol
/// table, the loader, prototype models, user-op table, the action database,
/// p-code injection).  This skeleton carries only the slice the W3 `Funcdata`
/// boot and its `funcdata_block`/`funcdata_op`/`funcdata_varnode` siblings reach
/// at the IR-construction boundary:
///
///   - the [`AddrSpaceManager`] (`glb` *is-a* `AddrSpaceManager` in C++): the
///     constant / unique / iop / fspec spaces and `getConstant`, needed by the
///     varnode-creation factories (`newConstant`, `newUnique`, `newVarnodeIop`,
///     `newVarnodeSpace`, `newCodeRef`) and by `VarnodeBank::new`;
///   - `getMinimumLanedRegisterSize` (`minLanedSize` is initialized from it in
///     the `Funcdata` constructor and reset in `clear`).
///
/// The `TypeFactory` (`glb->types`, W6 / [`crate::dtype`]), the symbol table /
/// `ScopeLocal` (W4 / [`Scope`]), the loader, the prototype models, the user-op
/// table, and the `ActionDatabase` (`glb->allacts`, used by `stageJumpTable`)
/// are **not** part of this skeleton; the W3 callers that need them are either
/// seam-noted with an explicit `Err`/`None` or take the value as an argument.
pub struct Architecture {
    /// The address-space manager (`Architecture` derives from
    /// `AddrSpaceManager` in C++).  // SEAM(W4)
    pub manage: AddrSpaceManager,
    /// Minimum Varnode size to check as a laned register (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // SEAM(W4)
    pub min_laned_register_size: int4,
}

impl Architecture {
    /// Construct the skeleton from an [`AddrSpaceManager`] (SEAM(W4)).
    pub fn new(manage: AddrSpaceManager) -> Architecture {
        // C++ default: getMinimumLanedRegisterSize() returns the configured
        // minimum; the upstream default when unset is 4.
        Architecture { manage, min_laned_register_size: 4 }
    }

    /// Borrow the address-space manager (C++ `glb` viewed as an
    /// `AddrSpaceManager`).  // SEAM(W4)
    pub fn manage(&self) -> &AddrSpaceManager {
        &self.manage
    }

    /// Get the minimum laned-register size (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // SEAM(W4)
    pub fn get_minimum_laned_register_size(&self) -> int4 {
        self.min_laned_register_size
    }

    /// Create a constant Varnode storage address in the constant space
    /// (C++ `AddrSpaceManager::getConstant`).  // SEAM(W4)
    pub fn get_constant(&self, val: u64) -> Address {
        self.manage.get_constant(val)
    }
}

/// The local-variable scope of a function (C++ `ScopeLocal`, `Funcdata::localmap`).
///
/// SEAM(W4): the symbol scope machinery (`decompiler/cpp/database.{hh,cc}`,
/// `varmap.{hh,cc}`) is a W4 subsystem.  `Funcdata` holds an `Option<Scope>`
/// where the C++ holds a `ScopeLocal *`; the W3 IR data-model never reads symbol
/// state, so the placeholder is empty.  The varnode-property look-ups
/// (`localmap->queryProperties`) that `setVarnodeProperties`/`newVarnode*` make
/// resolve to "no entry, no extra flags" until W4 fills this in.
#[derive(Debug, Clone, Default)]
pub struct Scope;

/// The recovered prototype of a function (C++ `FuncProto`, `Funcdata::funcp`).
///
/// SEAM(W4): the prototype model subsystem (`decompiler/cpp/fspec.{hh,cc}`) is
/// W4.  `Funcdata` holds a `FuncProto` placeholder so the struct layout and the
/// `getFuncProto` accessor exist; the W3 IR construction never queries the
/// prototype.
#[derive(Debug, Clone, Default)]
pub struct FuncProto;

/// Shared handle to the [`Architecture`] (C++ `Funcdata::glb`, a borrowed
/// `Architecture *`).
///
/// SEAM(W4): the C++ `glb` is a non-owning back-pointer to the long-lived
/// `Architecture`.  Modeled as `Rc<Architecture>` so multiple `Funcdata`
/// snapshots (ADR 0007) can share it; the W3 code only reads through it.
pub type ArchHandle = Rc<Architecture>;
