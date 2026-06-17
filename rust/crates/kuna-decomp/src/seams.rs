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
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
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

/// One mapped global [`SymbolEntry`] storage record, flattened for the read-only
/// `queryProperties` walk the [`ArchHandle`] performs in
/// [`Funcdata::set_varnode_properties`](crate::funcdata::Funcdata::set_varnode_properties).
///
/// Carries exactly the fields `Scope::findContainer` / `SymbolEntry::inUse` /
/// `SymbolEntry::getAllFlags` read (`database.cc:2278-2310`, `1268-1286`); a flat
/// per-space record set rather than the live `rangemap<SymbolEntry>` because the
/// global scope is frozen by the time the `Funcdata`'s `glb` is built (every
/// `map addr` ran before `load function`).
#[derive(Debug, Clone)]
pub struct GlobalEntry {
    /// Address-space index (C++ `addr.getSpace()->getIndex()`).
    pub space_index: int4,
    /// First offset of the storage (C++ `SymbolEntry::getFirst`).
    pub first: u64,
    /// Last offset of the storage (C++ `SymbolEntry::getLast`, `uintb` wrap).
    pub last: u64,
    /// Number of bytes consumed by this storage (C++ `SymbolEntry::getSize`).
    pub size: int4,
    /// `extraflags | symbol->getFlags()` (C++ `SymbolEntry::getAllFlags`).
    pub all_flags: uint4,
    /// `(symbol->getFlags() & addrtied) != 0` — drives `SymbolEntry::inUse`'s
    /// "valid throughout scope" branch without re-deriving from `all_flags`.
    pub addrtied: bool,
    /// Code-address ranges where this storage is valid (C++ `uselimit`); empty
    /// for the address-tied `map addr` globals.
    pub uselimit: kuna_base::address::RangeList,
    /// The owning Symbol's display name (C++ `SymbolEntry::getSymbol()->getDisplayName()`),
    /// for the naming pass — `Funcdata::linkSymbol`'s reach into the global scope
    /// returns the Symbol whose display name a global-mapped HighVariable carries.
    pub symbol_name: String,
    /// Byte offset of this storage piece within the owning Symbol (C++
    /// `SymbolEntry::getOffset`), for the `getSymbolOffset` member-access render.
    pub symbol_offset: int4,
    /// The owning Symbol's data-type (C++ `SymbolEntry::getSymbol()->getType()`),
    /// for the HighVariable's symbol type after a global name binds.
    pub symbol_type: Option<std::rc::Rc<crate::dtype::Datatype>>,
}

/// A read-only snapshot of the global [`Scope`](crate::database::Scope)
/// sufficient to reproduce `Scope::queryProperties` (`database.cc:1268-1286`)
/// from the [`ArchHandle`] in [`Funcdata::set_varnode_properties`].
///
/// The C++ `glb` is the live `Architecture`, so `localmap->queryProperties` walks
/// the parent chain up to the live global scope.  The merged kuna `glb`
/// ([`Architecture`]) is a separate IR-boundary skeleton, and the function's
/// `localmap` owns its own detached `Database`; this snapshot is the wire that
/// reconnects the global symbol table onto `glb` so global-mapped varnodes get
/// `persist`/`addrtied` painted and survive `ActionDeadCode`.  Built once per
/// `Funcdata` at [`build_arch_handle`](crate::architecture::Architecture::build_arch_handle).
#[derive(Debug, Clone, Default)]
pub struct GlobalQuery {
    /// Every whole-and-piece mapped storage in the global scope, all spaces.
    pub entries: Vec<GlobalEntry>,
    /// The global scope's owned data ranges (C++ `Scope::rangetree`), for the
    /// `inScope` discovery branch of `queryProperties`.
    pub owned: kuna_base::address::RangeList,
    /// The boolean-property map (C++ `Database::flagbase`), for `getProperty`.
    pub flagbase: kuna_base::partmap::PartMap<Address, uint4>,
}

impl GlobalQuery {
    /// C++ `Database::getProperty(addr)` (`database.hh:949`): the boolean
    /// properties (read-only/volatile) painted on a memory range.
    fn get_property(&self, addr: &Address) -> uint4 {
        *self.flagbase.get_value(addr)
    }

    /// C++ `Scope::findContainer` (`database.cc:2278-2310`) restricted to the
    /// global scope: the smallest mapped storage containing `[addr, addr+size-1]`
    /// that is in-use at `usepoint`.  Returns its `getAllFlags()`.
    fn find_container_flags(&self, addr: &Address, size: int4, usepoint: &Address) -> Option<uint4> {
        let space = addr.get_space()?;
        let space_index = space.get_index();
        let start = addr.get_offset();
        // end = addr + size - 1 (uintb wrap), as in find_container.
        // cast: int4 -> u64, reproducing the C++ `uintb` widening of the
        // (non-negative) byte count `size`; sign-extension is irrelevant since a
        // storage size is never negative.
        let end = start.wrapping_add(size as u64).wrapping_sub(1);
        let mut best: Option<&GlobalEntry> = None;
        let mut oldsize: int4 = -1;
        for e in &self.entries {
            if e.space_index != space_index {
                continue;
            }
            // Containment: first <= addr (entries are storage at >= first) and
            // last >= end.  The rangemap subsort walk already filters to
            // first <= addr; here check both bounds explicitly.
            if e.first > start || e.last < end {
                continue;
            }
            if e.size < oldsize || oldsize == -1 {
                // SymbolEntry::inUse(usepoint): addr-tied is valid throughout; an
                // invalid usepoint never matches a use-limited entry; otherwise
                // the usepoint must fall in the uselimit ranges.
                let in_use = if e.addrtied {
                    true
                } else if usepoint.is_invalid() {
                    false
                } else {
                    e.uselimit.in_range(usepoint, 1)
                };
                if in_use {
                    best = Some(e);
                    if e.size == size {
                        break;
                    }
                    oldsize = e.size;
                }
            }
        }
        best.map(|e| e.all_flags)
    }

    /// C++ `Scope::findContainer` (`database.cc:2278-2310`) restricted to the
    /// global scope, returning the *smallest containing entry* itself (not just its
    /// flags) so the naming pass can read its Symbol.  Identical containment /
    /// `inUse` / smallest-size logic as [`find_container_flags`](Self::find_container_flags),
    /// factored out so both queries share one walk.
    fn find_container_entry(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<&GlobalEntry> {
        let space = addr.get_space()?;
        let space_index = space.get_index();
        let start = addr.get_offset();
        // end = addr + size - 1 (uintb wrap), as in find_container.
        // cast: int4 -> u64, the C++ `uintb` widening of the non-negative byte
        // count; a storage size is never negative so sign-extension is irrelevant.
        let end = start.wrapping_add(size as u64).wrapping_sub(1);
        let mut best: Option<&GlobalEntry> = None;
        let mut oldsize: int4 = -1;
        for e in &self.entries {
            if e.space_index != space_index {
                continue;
            }
            if e.first > start || e.last < end {
                continue;
            }
            if e.size < oldsize || oldsize == -1 {
                let in_use = if e.addrtied {
                    true
                } else if usepoint.is_invalid() {
                    false
                } else {
                    e.uselimit.in_range(usepoint, 1)
                };
                if in_use {
                    best = Some(e);
                    if e.size == size {
                        break;
                    }
                    oldsize = e.size;
                }
            }
        }
        best
    }

    /// Resolve the global Symbol covering a Varnode for the naming pass — the C++
    /// `Funcdata::linkSymbol`'s reach into the global scope (`funcdata_varnode.cc:1190`
    /// `localmap->queryProperties` walks up to the global scope, returns the covering
    /// `SymbolEntry`, and `high->getSymbol()` then carries that Symbol's display name).
    ///
    /// Returns `(display_name, symbol_offset, symbol_type)` where `symbol_offset` is
    /// the byte offset of the access within the Symbol (C++ `(addr - entry_addr) +
    /// entry_offset`; 0 for a whole-symbol/scalar global, > 0 for an array/struct
    /// member).  `None` when no global Symbol covers `[addr, addr+size)`.
    pub fn name_for_varnode(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(String, int4, Option<std::rc::Rc<crate::dtype::Datatype>>)> {
        if addr.is_constant() {
            return None;
        }
        let e = self.find_container_entry(addr, size, usepoint)?;
        // sym_off = (access_addr - entry_addr) + entry_offset (C++
        // ScopeLocal::name_for_varnode / SymbolEntry geometry).  `e.first` is the
        // entry's starting offset (== entry addr offset).
        let sym_off = (addr.get_offset().wrapping_sub(e.first) as int4)
            .wrapping_add(e.symbol_offset);
        Some((e.symbol_name.clone(), sym_off, e.symbol_type.clone()))
    }

    /// The type-locked covering Symbol's sized geometry for a Varnode — the input
    /// to C++ `SymbolEntry::updateType`/`getSizedType` (`database.cc:136,151`).
    ///
    /// `setVarnodeProperties` (funcdata_varnode.cc:31) calls `setSymbolProperties`
    /// (varnode.cc:429) -> `entry->updateType(this)`, which type-locks the Varnode
    /// to `getSizedType(addr,size) = getExactPiece(symbol->getType(), off, size)`
    /// ONLY when `(symbol->getFlags() & typelock) != 0`.  This returns
    /// `(symbol_type, off)` for the covering entry when it is type-locked, so the
    /// caller can run `getExactPiece` against the shared `TypeFactory` and apply
    /// `Varnode::updateType(dt, lock=true, override=true)`.  `None` when no global
    /// Symbol covers `[addr, addr+size)` or its Symbol is not type-locked.
    pub fn sized_type_geometry(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(std::rc::Rc<crate::dtype::Datatype>, int4)> {
        use crate::varnode::varnode_flags;
        if addr.is_constant() {
            return None;
        }
        let e = self.find_container_entry(addr, size, usepoint)?;
        // SymbolEntry::updateType: only a type-locked Symbol forces a type.
        if (e.all_flags & varnode_flags::typelock) == 0 {
            return None;
        }
        let ct = e.symbol_type.clone()?;
        // getSizedType: off = (inaddr - addr) + offset (non-dynamic entry).
        let off = (addr.get_offset().wrapping_sub(e.first) as int4).wrapping_add(e.symbol_offset);
        Some((ct, off))
    }

    /// C++ `Scope::queryProperties` (`database.cc:1268-1286`) for the parentless
    /// global scope: the Varnode boolean properties of the memory range, whether
    /// or not a covering Symbol exists.  Constant addresses never match (the
    /// `stackContainer` `addr.isConstant()` guard).
    pub fn query_properties(&self, addr: &Address, size: int4, usepoint: &Address) -> uint4 {
        use crate::varnode::varnode_flags;
        if addr.is_constant() {
            return 0;
        }
        if let Some(flags) = self.find_container_flags(addr, size, usepoint) {
            // res != 0: use the entry's flags.
            flags
        } else if self.owned.in_range(addr, size) {
            // finalscope != 0 (the global scope owns the range, no symbol).
            varnode_flags::mapped
                | varnode_flags::addrtied
                | varnode_flags::persist
                | self.get_property(addr)
        } else {
            // No symbol, no owning scope: just the range's boolean properties.
            self.get_property(addr)
        }
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
    /// Vector registers that have preferred lane sizes (C++
    /// `Architecture::lanerecords`), built from the pspec `<register_data>`
    /// `vector_lane_sizes` attributes by `Architecture::decodeRegisterData`.
    /// Each [`LanedRegister`](crate::transform::LanedRegister) is keyed by its
    /// whole size (the records are sorted ascending by whole size, one per size),
    /// so [`Architecture::get_laned_register`] can binary-search on size exactly
    /// as the C++ `Architecture::getLanedRegister` does.  Empty for hand-built
    /// fixtures and non-vector architectures.
    pub lanerecords: Vec<crate::transform::LanedRegister>,
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
    /// (kuna) angr-style default naming: an unknown callee / global prints as
    /// `sub_<addr>` / `dat_<addr>` rather than `func_<addr>` (C++
    /// `Architecture::name_style_angr`, default-on).  Read by the call-spec
    /// printed-name resolution ([`FuncCallSpecs::fspec_printed_name`]).
    pub name_style_angr: bool,
    /// (kuna) GH-558: present canonicalized `INT_LESS(x, c+1)` comparisons in
    /// their original `x <= c` form (C++ `present_lessequal`, DIV-2 default-on).
    /// Read by [`ActionPresentCompareForm`](crate::kuna_compareform::ActionPresentCompareForm).
    pub present_lessequal: bool,
    /// (kuna) GH-9203: when set, `ActionConditionalConst::handlePhiNodes` declines
    /// to materialize a propagated constant as a COPY inside a loop predecessor
    /// block (which would render as a spurious `= 0` in the do/while body).  C++
    /// `Architecture::condexe_block_placement`, DIV-3 default-on.  Read by
    /// [`crate::condconst`].
    pub condexe_block_placement: bool,
    /// Attempt conversion of `whiledo` loops to `for` loops (C++
    /// `Architecture::analyze_for_loops`, default-on).  Read by
    /// `Funcdata::finalize_forloop_transform` (the BlockWhileDo for-loop reroll).
    pub analyze_for_loops: bool,
    /// The data-type factory (C++ `glb->types`), shared from the real
    /// [`crate::architecture::Architecture`] through `build_arch_handle`.
    /// `ActionInferTypes` reaches `getBase`/`getTypePointer`/`down_chain` through
    /// it.  `None` for hand-built fixtures (no type factory registry).
    pub types: Option<Rc<crate::dtype::TypeFactoryImpl>>,
    /// Maximum number of entries in a single JumpTable (C++
    /// `Architecture::max_jumptable_size`), shared from the real architecture.
    /// Read by `JumpTable::recoverModel`.  `0` for hand-built fixtures.
    pub max_jumptable_size: uint4,
    /// How many low bits of a recovered function pointer are forced to zero (C++
    /// `Architecture::funcptr_align`), shared from the real architecture.  Read
    /// by `JumpBasic::buildAddresses` to align recovered targets.  `0` (no
    /// alignment) for hand-built fixtures.
    pub funcptr_align: int4,
    /// The program load image (C++ `Architecture::loader`), shared from the
    /// engine through `build_arch_handle`.  Read by jump-table emulation
    /// (`EmulateFunction::executeLoad` -> `get_load_image_value`) to fetch the
    /// read-only switch table.  `None` for hand-built fixtures (no loader).
    pub loader: Option<Rc<std::cell::RefCell<Box<dyn kuna_sleigh::loadimage::LoadImage>>>>,
    /// Treat read-only values as constants (C++ `Architecture::readonlypropagate`,
    /// flipped by `option readonly`), carried into the per-function `glb` so
    /// `ActionVarnodeProps::apply` reaches `glb->readonlypropagate` to gate
    /// [`Funcdata::fillin_read_only`](crate::funcdata::Funcdata::fillin_read_only).
    /// `false` by default (C++ `Architecture::resetDefaults`).
    pub readonlypropagate: bool,
    /// Whether the volatile read/write userops display \e functionally (C++
    /// `VolatileReadOp`/`VolatileWriteOp` ctor `functional` flag — the `<volatile
    /// format="functional">` spec attribute).  Drives the `setHoldOutput` branch
    /// in [`Funcdata::replace_volatile`](crate::funcdata::Funcdata::replace_volatile):
    /// `getDisplay() != 0` is equivalent to `!functional` for the volatile-read op.
    /// `false` (non-functional) by default, matching the unconfigured `glb`.
    pub volatile_display_functional: bool,
    /// Data-type-splitting toggle bits (C++ `Architecture::split_datatype_config`,
    /// `option_struct`/`option_array`/`option_pointer`), carried into the
    /// per-function `glb` so [`SplitDatatype`](crate::subflow::SplitDatatype) and
    /// the `RuleSplit{Copy,Load,Store}` rules reach `glb->split_datatype_config`.
    /// Default `struct|array|pointer` (C++ `Architecture::resetDefaults`).
    pub split_datatype_config: uint4,
    /// Read-only snapshot of the global symbol table (C++ `glb->symboltab`'s
    /// global scope + property map), the wire for `localmap->queryProperties`'s
    /// walk up to the global scope.  Built at `build_arch_handle` (after every
    /// `map addr`); read by [`Funcdata::set_varnode_properties`](crate::funcdata::
    /// Funcdata::set_varnode_properties) to paint `persist`/`addrtied` on
    /// global-mapped varnodes so their stores survive `ActionDeadCode`.  `None`
    /// for hand-built fixtures (no symbol table).
    pub global_query: Option<Rc<GlobalQuery>>,
    /// Infer pointers from likely-address constants (C++ `glb->infer_pointers`),
    /// shared from the real [`crate::architecture::Architecture`] through
    /// `build_arch_handle`.  Read by [`ActionConstantPtr`](crate::coreaction_render::
    /// ActionConstantPtr)'s `isPointer`/`checkCopy` (coreaction.cc:1056-1144) to gate
    /// the various pointer-inference arms.  Default-on (C++ `resetDefaults`); `false`
    /// for hand-built fixtures (no constant-pointer recovery exercised).
    pub infer_pointers: bool,
    /// (kuna GH-6930) Infer single-bit constants matching an exact function entry
    /// as pointers (C++ `glb->infer_funcentry`), shared from the real architecture.
    /// Read by `ActionConstantPtr::isPointer`'s `bit_transitions < 3` guard
    /// (coreaction.cc:1158-1159) via [`kuna_is_function_entry`](crate::
    /// kuna_inferfuncentry::kuna_is_function_entry).  Default-on (DIV-2); `false`
    /// for hand-built fixtures.
    pub infer_funcentry: bool,
    /// Ordered list of address spaces in which a constant pointer can be inferred
    /// (C++ `Architecture::inferPtrSpaces`, built by `cacheAddrSpaceProperties` +
    /// the cspec `<global>` push), shared from the real architecture through
    /// `build_arch_handle`.  Iterated by `ActionConstantPtr::selectInferSpace`
    /// (coreaction.cc:1020-1047).  Empty for hand-built fixtures.
    pub infer_ptr_spaces: Vec<Rc<AddrSpace>>,
    /// Source-declared callee prototypes, keyed by `(space_index, entry_offset)`
    /// (C++ the callee `Funcdata`'s lazily-built locked `FuncProto`).  Snapshotted
    /// from the global FunctionSymbols at `build_arch_handle`; read back by
    /// `ActionDefaultParams::apply` via [`Architecture::callee_proto_pieces`] to
    /// `fc->copy(otherfunc->getFuncProto())` (`coreaction.cc:2385`).  Empty for
    /// hand-built fixtures and undeclared callees.
    pub callee_protos: Vec<(int4, kuna_base::types::uintb, crate::fspec::PrototypePieces)>,
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
            lanerecords: Vec::new(),
            opbehaviors: Vec::new(),
            defaultfp: None,
            evalfp_current: None,
            // C++ Architecture default: trim_recurse_max = 5 (resetDefaults).
            trim_recurse_max: 5,
            // C++ Architecture default: max_implied_ref = 2 (resetDefaults).
            max_implied_ref: 2,
            return_single: false,
            // (kuna) angr-style default naming is default-on (Architecture::reset).
            name_style_angr: true,
            // (kuna) DIV-2 default-on (GH-558): resetDefaults sets present_lessequal=true.
            present_lessequal: true,
            // (kuna) DIV-3 default-on (GH-9203): architecture.cc sets condexe_block_placement=true.
            condexe_block_placement: true,
            // C++ Architecture default: analyze_for_loops = true (architecture.cc).
            analyze_for_loops: true,
            types: None,
            max_jumptable_size: 0,
            funcptr_align: 0,
            loader: None,
            // C++ Architecture default: readonlypropagate = false (resetDefaults);
            // `option readonly` flips it before the per-function build_arch_handle.
            readonlypropagate: false,
            // Default volatile ops are non-functional (`getDisplay() != 0`), so the
            // volatile-read op's output is held; matches the unconfigured glb.
            volatile_display_functional: false,
            // C++ Architecture default: struct|array|pointer (resetDefaults).
            split_datatype_config: crate::options::split_datatype::OPTION_STRUCT
                | crate::options::split_datatype::OPTION_ARRAY
                | crate::options::split_datatype::OPTION_POINTER,
            global_query: None,
            // C++ Architecture default: infer_pointers = true (resetDefaults);
            // a hand-built fixture never exercises constant-pointer recovery, but
            // the real lift+analyze path overwrites this from the real arch.
            infer_pointers: false,
            // (kuna) DIV-2 default-on (GH-6930): the real arch overwrites this.
            infer_funcentry: false,
            // No inferable spaces until cacheAddrSpaceProperties runs on the real
            // arch and build_arch_handle shares the result.
            infer_ptr_spaces: Vec::new(),
            // No declared callee prototypes until build_arch_handle snapshots them.
            callee_protos: Vec::new(),
        }
    }

    /// Read a `sz`-byte value out of the program load image at `addr` (C++
    /// `EmulatePcodeOp::getLoadImageValue`, `emulateutil.cc:30`): `loadFill` a
    /// `uintb` worth of bytes, byte-swap to host order if the space endianness
    /// differs from the host, then mask/shift to `sz`.  Drives jump-table LOAD
    /// emulation.  Returns an error if no loader is shared (hand-built fixtures).
    pub fn get_load_image_value(&self, addr: &Address, sz: int4) -> KunaResult<u64> {
        use kuna_base::address::{byte_swap, calc_mask};
        use kuna_base::types::HOST_ENDIAN;
        let loader = self
            .loader
            .as_ref()
            .ok_or_else(|| KunaError::lowlevel("getLoadImageValue: no load image shared"))?;
        let mut buf = [0u8; 8]; // sizeof(uintb)
        loader.borrow_mut().load_fill(&mut buf, addr)?;
        let big = addr.is_big_endian();
        let mut res = if HOST_ENDIAN == 1 {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        };
        if (HOST_ENDIAN == 1) != big {
            res = byte_swap(res, 8);
        }
        if big && (sz as usize) < 8 {
            res >>= (8 - sz as u32) * 8;
        } else {
            res &= calc_mask(sz);
        }
        Ok(res)
    }

    /// Borrow the data-type factory (C++ `glb->types`), if shared.
    /// Whether read-only values are treated as constants (C++
    /// `glb->readonlypropagate`).  Gates `ActionVarnodeProps`'s call into
    /// `Funcdata::fillinReadOnly`.
    pub fn readonly_propagate(&self) -> bool {
        self.readonlypropagate
    }

    /// Whether the volatile-read op's output must be held (C++
    /// `vr_op->getDisplay() != 0`).  For the default (non-functional) volatile
    /// read this is `true`, so `replaceVolatile` calls `setHoldOutput`.
    pub fn volatile_read_holds_output(&self) -> bool {
        !self.volatile_display_functional
    }

    /// Read `sz` bytes out of the program load image at `addr` into `buf`,
    /// mirroring C++ `glb->loader->loadFill(bytes, sz, addr)`.  Returns an error
    /// (the C++ `DataUnavailError` analogue) when no loader is shared or the
    /// region is unavailable.  Used by [`Funcdata::fillin_read_only`](crate::
    /// funcdata::Funcdata::fillin_read_only) to fetch a read-only Varnode's value.
    pub fn loader_fill(&self, buf: &mut [u8], addr: &Address) -> KunaResult<()> {
        let loader = self
            .loader
            .as_ref()
            .ok_or_else(|| KunaError::lowlevel("loadFill: no load image shared"))?;
        loader.borrow_mut().load_fill(buf, addr)
    }

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

    /// The source-declared prototype pieces for the callee whose entry is `addr`
    /// (C++ the callee `Funcdata`'s locked `FuncProto`), or `None` if that callee
    /// had no `parse line extern` declaration.  Read by `ActionDefaultParams::apply`
    /// to copy the locked callee prototype into the call site.  Matched by
    /// `(space_index, offset)` — the seam carries no `Rc<AddrSpace>` identities for
    /// the global scope, only the snapshotted indices.
    pub fn callee_proto_pieces(&self, addr: &Address) -> Option<&crate::fspec::PrototypePieces> {
        let space_index = addr.get_space()?.get_index();
        let offset = addr.get_offset();
        self.callee_protos
            .iter()
            .find(|(si, off, _)| *si == space_index && *off == offset)
            .map(|(_, _, pieces)| pieces)
    }

    /// The current-evaluation model (C++ `glb->evalfp_current`), falling back to
    /// `defaultfp` when unset.
    pub fn eval_fp_current(&self) -> Option<&Rc<crate::fspec::ProtoModel>> {
        self.evalfp_current.as_ref().or(self.defaultfp.as_ref())
    }

    /// The called-function evaluation model (C++ `glb->evalfp_called`), falling
    /// back to `defaultfp` when unset (`ActionStackPtrFlow::analyzeExtraPop` reads
    /// `evalfp_called ?: defaultfp`).  The merged arch-handle carries no distinct
    /// `evalfp_called` field; absent that it is the same as `defaultfp`, which is
    /// the C++ fallback the `?:` would take anyway.
    pub fn eval_fp_called(&self) -> Option<&Rc<crate::fspec::ProtoModel>> {
        self.defaultfp.as_ref()
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

    /// Query the global symbol table for the Varnode boolean properties of a
    /// storage range (C++ `localmap->queryProperties`'s reach into the global
    /// scope, `database.cc:1268-1286`).  Returns `0` when no global symbol table
    /// is shared (hand-built fixtures) or the range carries no global property.
    ///
    /// This is the global-scope half of `Funcdata::setVarnodeProperties`: a
    /// global-mapped Varnode picks up `mapped | addrtied | persist` (and any
    /// `readonly`/`volatile` on the range) here, so `ActionDeadCode` keeps its
    /// store alive.
    pub fn query_global_properties(&self, addr: &Address, size: int4, usepoint: &Address) -> uint4 {
        match &self.global_query {
            Some(gq) => gq.query_properties(addr, size, usepoint),
            None => 0,
        }
    }

    /// Resolve the global Symbol covering a storage range for the naming pass (C++
    /// `Funcdata::linkSymbol`'s reach into the global scope; see
    /// [`GlobalQuery::name_for_varnode`]).  Returns `(display_name, symbol_offset,
    /// symbol_type)`, or `None` when no global symbol table is shared (hand-built
    /// fixtures) or no global Symbol covers the range.
    pub fn name_for_global_varnode(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(String, int4, Option<std::rc::Rc<crate::dtype::Datatype>>)> {
        self.global_query
            .as_ref()
            .and_then(|gq| gq.name_for_varnode(addr, size, usepoint))
    }

    /// The type-locked covering global Symbol's `(symbol_type, in_symbol_offset)`
    /// for a Varnode storage range — the geometry C++ `SymbolEntry::getSizedType`
    /// (`database.cc:151`) feeds `getExactPiece`.  Drives the type-force half of
    /// `Funcdata::setVarnodeProperties` -> `Varnode::setSymbolProperties` ->
    /// `entry->updateType` (varnode.cc:429, database.cc:136): a global-mapped store
    /// to a type-locked Symbol (`map addr r0x301018 octint4 globaloct`) picks up
    /// that Symbol's data-type, so `ActionInferTypes` seeds the store + its COPY
    /// input from it and the forced display format (`oct`) reaches the constant.
    /// `None` when no global symbol table is shared or the covering Symbol is not
    /// type-locked.
    pub fn sized_type_for_global_varnode(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(std::rc::Rc<crate::dtype::Datatype>, int4)> {
        self.global_query
            .as_ref()
            .and_then(|gq| gq.sized_type_geometry(addr, size, usepoint))
    }

    /// Get the minimum laned-register size (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // SEAM(W4)
    pub fn get_minimum_laned_register_size(&self) -> int4 {
        self.min_laned_register_size
    }

    /// Look up the laned-register record for a storage location (C++
    /// `Architecture::getLanedRegister`, `architecture.cc:291`).
    ///
    /// As in the C++, the record is associated only with the *size* of the
    /// storage, not its address; `loc` is accepted to match the signature but is
    /// unused.  The records are sorted ascending by whole size, so this is a
    /// faithful transcription of the C++ binary search; returns `None` for the
    /// C++ `(const LanedRegister *)0` when no record matches the size.
    pub fn get_laned_register(
        &self,
        _loc: &Address,
        size: int4,
    ) -> Option<&crate::transform::LanedRegister> {
        // int4 min = 0; int4 max = lanerecords.size() - 1;
        let mut min: int4 = 0;
        let mut max: int4 = self.lanerecords.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let sz = self.lanerecords[mid as usize].get_whole_size();
            if sz < size {
                min = mid + 1;
            } else if size < sz {
                max = mid - 1;
            } else {
                return Some(&self.lanerecords[mid as usize]);
            }
        }
        None
    }

    /// Create a constant Varnode storage address in the constant space
    /// (C++ `AddrSpaceManager::getConstant`).  // SEAM(W4)
    pub fn get_constant(&self, val: u64) -> Address {
        self.manage.get_constant(val)
    }

    /// Infer pointers from likely-address constants (C++ `glb->infer_pointers`).
    /// Read by `ActionConstantPtr::isPointer`/`checkCopy`.
    pub fn infer_pointers(&self) -> bool {
        self.infer_pointers
    }

    /// (kuna GH-6930) Infer single-bit constants matching an exact function entry
    /// as pointers (C++ `glb->infer_funcentry`).  Read by `ActionConstantPtr::
    /// isPointer`'s `bit_transitions < 3` guard.
    pub fn infer_funcentry(&self) -> bool {
        self.infer_funcentry
    }

    /// The ordered inferable-pointer spaces (C++ `glb->inferPtrSpaces`), iterated
    /// by `ActionConstantPtr::selectInferSpace`.
    pub fn infer_ptr_spaces(&self) -> &[Rc<AddrSpace>] {
        &self.infer_ptr_spaces
    }

    /// Interpret a constant as a pointer into `spc` (C++ `glb->resolveConstant`,
    /// `AddrSpaceManager::resolveConstant`, `space.cc`).  A thin wrapper over the
    /// shared manager: for an un-segmented space it is just
    /// `Address(spc, byteToAddress(wrap(val)))` with `fullEncoding = val`; a
    /// near-pointer space routes through its registered `SegmentedResolver`.
    pub fn resolve_constant(
        &self,
        spc: &Rc<AddrSpace>,
        val: u64,
        sz: int4,
        point: &Address,
        full_encoding: &mut u64,
    ) -> KunaResult<Address> {
        self.manage.resolve_constant(spc, val, sz, point, full_encoding)
    }

    /// C++ `data.getScopeLocal()->getParent()->queryContainer(addr,size,usepoint)`
    /// restricted to the global scope (the merged kuna `glb` carries the global
    /// scope as the read-only [`GlobalQuery`] snapshot, not a live `Database`
    /// parent chain).  Returns the smallest covering Symbol's
    /// [`GlobalContainer`] (its data-type, entry address, and `getAllFlags()`), or
    /// `None` when no global symbol table is shared (hand-built fixtures) or no
    /// global Symbol covers `[addr, addr+size)`.
    ///
    /// This is the routing wire `ActionConstantPtr::isPointer` (coreaction.cc:1167)
    /// needs: the entry's `getSymbol()->getType()` (`symbol_type`), `getAddr()`
    /// (`entry_addr`), and — for `spacebase_constant`'s `sym->isTypeLocked()` —
    /// the `typelock` bit in `all_flags`.
    pub fn query_container_global(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<GlobalContainer> {
        let gq = self.global_query.as_ref()?;
        let e = gq.find_container_entry(addr, size, usepoint)?;
        let space = addr.get_space()?;
        Some(GlobalContainer {
            symbol_type: e.symbol_type.clone(),
            entry_addr: Address::new(Rc::clone(space), e.first),
            all_flags: e.all_flags,
            symbol_offset: e.symbol_offset,
        })
    }

    /// C++ `data.getScopeLocal()->getParent()->queryFunction(entry)->getFuncProto()`
    /// restricted to the global-scope snapshot: resolve the callee's recovered
    /// prototype for a direct-call entry address.
    ///
    /// The frozen [`GlobalQuery`] carries every global SymbolEntry's `symbol_type`;
    /// for a FunctionSymbol that is a `TypeCode` whose `getPrototype()` returns the
    /// `FuncProto` the callee body would expose (a declared callee parks the locked
    /// signature there — `TypeFactory::getTypeCode(PrototypePieces)`).  We match the
    /// entry whose storage *starts* at `entry` (an exact function-entry hit) and
    /// borrow its code prototype.  Generic over the callee signature: keyed only by
    /// address, no name/value keying.  `None` when no global symbol table is shared
    /// (hand-built fixtures), no entry starts at `entry`, or the entry's type is not
    /// a code type with a prototype (an unknown callee).
    pub fn query_callee_proto(
        &self,
        entry: &Address,
    ) -> Option<std::rc::Rc<crate::fspec::FuncProto>> {
        let gq = self.global_query.as_ref()?;
        let space_index = entry.get_space()?.get_index();
        let start = entry.get_offset();
        for e in &gq.entries {
            if e.space_index != space_index || e.first != start {
                continue;
            }
            if let Some(ct) = &e.symbol_type {
                if let Some(proto) = ct.get_code_prototype() {
                    return Some(std::rc::Rc::clone(proto));
                }
            }
        }
        None
    }
}

/// The fields of a global [`SymbolEntry`] that C++ `ActionConstantPtr::isPointer` /
/// `Funcdata::spacebaseConstant` read off the result of `queryContainer`
/// (`coreaction.cc:1167-1180`, `funcdata.cc:411-416`).  Flattened from a
/// [`GlobalEntry`] by [`Architecture::query_container_global`] because the merged
/// `glb` holds the frozen global scope as a snapshot, not the live `Scope`.
#[derive(Debug, Clone)]
pub struct GlobalContainer {
    /// `entry->getSymbol()->getType()` — the covering Symbol's declared data-type
    /// (e.g. the `int4[3][5]` of a `map addr r... int4 myarray[3][5]`).  `None` for
    /// an entry whose snapshot carried no type (degrade as the C++ never would).
    pub symbol_type: Option<std::rc::Rc<crate::dtype::Datatype>>,
    /// `entry->getAddr()` — the entry's starting address (`Address(space, first)`),
    /// for the `needexacthit && entry->getAddr() != rampoint` exact-hit test.
    pub entry_addr: Address,
    /// `entry->getAllFlags()` — used for the `typelock` bit
    /// (`sym->isTypeLocked()`).
    pub all_flags: uint4,
    /// `entry->getOffset()` — the byte offset of this storage piece within the
    /// owning Symbol, for the `TypeSpacebase::getSubType` residual-offset
    /// computation (`funcdata.cc` / `type.cc:3431`).
    pub symbol_offset: int4,
}

impl GlobalContainer {
    /// `entry->getSymbol()->isTypeLocked()` (C++ `funcdata.cc:414`): the covering
    /// Symbol's type is locked.
    pub fn is_type_locked(&self) -> bool {
        (self.all_flags & crate::varnode::varnode_flags::typelock) != 0
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
