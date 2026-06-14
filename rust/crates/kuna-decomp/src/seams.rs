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
    ///
    /// Held as an [`Rc`] (LOSS-132): the **single** space set the SLEIGH engine
    /// lifted into is *shared* here, so `glb.manage()` returns the same
    /// `Rc<AddrSpace>` identities and indices the lifted varnodes carry and the
    /// analysis passes key state by.  Hand-built test fixtures still pass an
    /// owned [`AddrSpaceManager`] through [`Architecture::new`] (wrapped here);
    /// the real lift+analyze path shares the engine's `Rc` via
    /// [`Architecture::new_shared`].
    pub manage: Rc<AddrSpaceManager>,
    /// Minimum Varnode size to check as a laned register (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // SEAM(W4)
    pub min_laned_register_size: int4,
    /// The p-code OpBehavior emulation table (C++ `glb->inst[opc]->getBehavior()`),
    /// indexed by op-code.
    ///
    /// The C++ `Architecture` IS-A `AddrSpaceManager` and owns the `TypeOp`
    /// table (with each op's `OpBehavior`); the W3 `glb` skeleton carries the
    /// behavior slice the IR-transform passes reach — `RuleCollapseConstants`
    /// drives `PcodeOp::collapse` through it for constant folding.  Empty for
    /// hand-built fixtures (those never fold constants); the real lift+analyze
    /// path populates it from the engine's table in
    /// `Architecture::build_arch_handle`.
    pub opbehaviors: Vec<Option<Rc<dyn kuna_num::opbehavior::OpBehavior>>>,
    /// The default prototype model (C++ `Architecture::defaultfp`), shared from
    /// the real [`crate::architecture::Architecture`] registry through
    /// `build_arch_handle`.  The proto-recovery actions read it to set the
    /// function's model when the prototype is unrecovered.  `None` for hand-built
    /// fixtures (no proto registry).
    pub defaultfp: Option<Rc<crate::fspec::ProtoModel>>,
    /// The current-evaluation prototype model (C++ `evalfp_current`); falls back
    /// to `defaultfp` when unset.  Shared from the real architecture.
    pub evalfp_current: Option<Rc<crate::fspec::ProtoModel>>,
    /// Maximum recursion depth for `Funcdata::ancestorOpUse` (C++
    /// `Architecture::trim_recurse_max`).  Drives `ActionReturnRecovery`'s
    /// ancestor-realism walk.
    pub trim_recurse_max: int4,
    /// Maximum number of references to an implied Varnode before it is forced
    /// explicit (C++ `Architecture::max_implied_ref`, default 2).  Drives
    /// `ActionMarkExplicit::baseExplicit`.
    pub max_implied_ref: int4,
    /// (kuna) GH-6990: keep only the first return register (C++ `return_single`).
    pub return_single: bool,
    /// (kuna) GH-558: present canonicalized `INT_LESS(x, c+1)` comparisons in
    /// their original `x <= c` form (C++ `present_lessequal`, DIV-2 default-on).
    /// Read by [`ActionPresentCompareForm`](crate::kuna_compareform::ActionPresentCompareForm).
    pub present_lessequal: bool,
    /// The data-type factory (C++ `glb->types`), shared from the real
    /// [`crate::architecture::Architecture`] through `build_arch_handle`.
    /// `ActionInferTypes` reaches `getBase`/`getTypePointer`/`down_chain` through
    /// it.  `None` for hand-built fixtures (no type factory registry).
    pub types: Option<Rc<crate::dtype::TypeFactoryImpl>>,
}

impl Architecture {
    /// Construct the skeleton from an owned [`AddrSpaceManager`] (SEAM(W4)).
    ///
    /// Used by hand-built test fixtures; the real path shares the engine's
    /// manager through [`Architecture::new_shared`].
    pub fn new(manage: AddrSpaceManager) -> Architecture {
        Architecture::new_shared(Rc::new(manage))
    }

    /// Construct the skeleton sharing the engine's single [`AddrSpaceManager`]
    /// (LOSS-132 keystone).  The `Rc` is the one the SLEIGH translator
    /// populated (with fspec/iop/join inserted by `Architecture::restoreFromSpec`),
    /// so the lifted varnodes and the analysis passes see the same spaces.
    pub fn new_shared(manage: Rc<AddrSpaceManager>) -> Architecture {
        // C++ default: getMinimumLanedRegisterSize() returns the configured
        // minimum; the upstream default when unset is 4.
        Architecture {
            manage,
            min_laned_register_size: 4,
            opbehaviors: Vec::new(),
            defaultfp: None,
            evalfp_current: None,
            // C++ Architecture default: trim_recurse_max = 5 (resetDefaults).
            trim_recurse_max: 5,
            // C++ Architecture default: max_implied_ref = 2 (resetDefaults).
            max_implied_ref: 2,
            return_single: false,
            // (kuna) DIV-2 default-on (GH-558): resetDefaults sets present_lessequal=true.
            present_lessequal: true,
            types: None,
        }
    }

    /// Borrow the data-type factory (C++ `glb->types`), if shared.
    pub fn types(&self) -> Option<&dyn crate::dtype::TypeFactory> {
        self.types.as_deref().map(|t| t as &dyn crate::dtype::TypeFactory)
    }

    /// Borrow the concrete data-type factory (for the `TypeFactoryImpl`-only
    /// builders, e.g. `down_chain`, the type-propagation engine needs).
    pub fn types_impl(&self) -> Option<&crate::dtype::TypeFactoryImpl> {
        self.types.as_deref()
    }

    /// Clone the shared data-type factory `Rc` (so a caller can hold a type
    /// factory handle across a `&mut Funcdata`/`&mut ScopeLocal` borrow that
    /// would otherwise alias the `&self` arch read).
    pub fn types_rc(&self) -> Option<Rc<crate::dtype::TypeFactoryImpl>> {
        self.types.clone()
    }

    /// The default prototype model (C++ `glb->defaultfp`), or `None`.
    pub fn default_fp(&self) -> Option<&Rc<crate::fspec::ProtoModel>> {
        self.defaultfp.as_ref()
    }

    /// The current-evaluation model (C++ `glb->evalfp_current`), falling back to
    /// `defaultfp` when unset.
    pub fn eval_fp_current(&self) -> Option<&Rc<crate::fspec::ProtoModel>> {
        self.evalfp_current.as_ref().or(self.defaultfp.as_ref())
    }

    /// Resolve an op-code to its emulation [`OpBehavior`](kuna_num::opbehavior::OpBehavior),
    /// or `None` (C++ `glb->inst[opc]->getBehavior()`).
    ///
    /// Drives `PcodeOp::collapse` (`RuleCollapseConstants`).  `None` for
    /// hand-built fixtures (empty table) or op-codes with no registered behavior.
    pub fn op_behavior(
        &self,
        opc: kuna_num::opcodes::OpCode,
    ) -> Option<&Rc<dyn kuna_num::opbehavior::OpBehavior>> {
        self.opbehaviors.get(opc as usize).and_then(|o| o.as_ref())
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

impl FuncProto {
    /// Is the output (return value) storage locked? (C++ `FuncProto::isOutputLocked`).
    ///
    /// SEAM(W6): the proto-recovery passes are seam stubs and never lock the
    /// output, so this reports the un-recovered default (`false`).  `ActionDeadCode::
    /// gatherConsumedReturn` reads it to decide whether the return value is fully
    /// consumed; with no locked proto it falls through to the NZ-mask scan.
    pub fn is_output_locked(&self) -> bool {
        false
    }

    /// Number of bytes of the return value that are consumed, or 0 if unknown
    /// (C++ `FuncProto::getReturnBytesConsumed`).
    ///
    /// SEAM(W6): no recovered proto, so 0 ("no restriction") — the faithful
    /// un-recovered default.
    pub fn get_return_bytes_consumed(&self) -> i32 {
        0
    }
}

/// Shared handle to the [`Architecture`] (C++ `Funcdata::glb`, a borrowed
/// `Architecture *`).
///
/// SEAM(W4): the C++ `glb` is a non-owning back-pointer to the long-lived
/// `Architecture`.  Modeled as `Rc<Architecture>` so multiple `Funcdata`
/// snapshots (ADR 0007) can share it; the W3 code only reads through it.
pub type ArchHandle = Rc<Architecture>;
