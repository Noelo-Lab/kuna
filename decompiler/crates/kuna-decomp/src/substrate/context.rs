//! Cross-wave stub placeholders for the W3 IR data-model.
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
    /// STUB(W3): the `PcodeOp` arena and its accessors are `op`/`funcdata`'s
    /// (`w3-ir-op`).  `varnode.rs` stores `OpId`s for `def` and `descend`
    /// links, exactly as the C++ stores `PcodeOp *`.
    pub struct OpId;

    /// Arena key for a `FlowBlock` (ADR 0001).
    ///
    /// STUB(W3): filled by `block` (`w3-ir-block`); declared here so the shared
    /// key set is in one place.
    pub struct BlockId;
}

/// HighVariable — the high-level variable an instance of which a Varnode is.
///
/// STUB(W7): filled by `merge`/`HighVariable` (`w7`).  `Varnode` holds an
/// `Option<HighVariableId>` where the C++ holds a `HighVariable *`; until W7
/// every varnode's high is `None` (the C++ `getHigh()` likewise throws until
/// merging builds it).  Modelled as an opaque arena key so the eventual
/// HighVariable arena can slot in without touching the Varnode layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HighVariableId(pub u32);

/// Cover — the def/use address coverage of a Varnode.
///
/// STUB(W7): filled by `cover`/`Cover` (`w7`).  The C++ `Varnode::cover` is a
/// lazily-built, mutable `Cover *`; the W3 data-model only needs to track the
/// presence/absence and the `coverdirty` flag (carried in `flags`), so a unit
/// placeholder suffices until W7 supplies the real geometry.
#[derive(Debug, Clone, Default)]
pub struct Cover;

/// TypeOp — the behavioral class (opcode) attached to a [`PcodeOp`].
///
/// STUB(W6): filled by `typeop`/`type` (`w6`).  The C++ `TypeOp` (`typeop.hh`)
/// bundles an [`OpCode`] value, a cached property-flag word
/// (`getFlags()`/`opflags`, transcribed as [`type_op_flags`]), a display
/// `name`, an `OpBehavior` for emulation, and the `TypeFactory`-backed local
/// type calculators (`getOutputLocal`/`getInputLocal`).
///
/// `op.cc` reaches only a thin slice of that surface: `PcodeOp::setOpcode`
/// caches `getFlags()` into the op's `flags`, `code()` returns `getOpcode()`,
/// the op-code lists key on `code()`, and the print/eval/type-local methods
/// dispatch through the `OpBehavior`/`TypeFactory`.  This boundary carries exactly
/// the first three (`opcode`/`flags`/`name`); the emulation+type-local methods
/// stay in W6 (the W3 `collapse`/`executeSimple`/`outputTypeLocal` paths defer
/// or take the behavior as an explicit argument — see `op.rs`).
#[derive(Debug, Clone)]
pub struct TypeOp {
    /// The op-code value (C++ `TypeOp::opcode`).  // STUB(W6)
    pub opcode: OpCode,
    /// Cached pcode-op properties for this op-code (C++ `TypeOp::opflags`,
    /// the `PcodeOp::*` flag bits `setOpcode` ORs in).  // STUB(W6)
    pub flags: uint4,
    /// Symbol denoting this operation (C++ `TypeOp::name`).  // STUB(W6)
    pub name: String,
}

impl TypeOp {
    /// Construct a minimal behavioral-class skeleton (STUB(W6)).
    pub fn new(opcode: OpCode, flags: uint4, name: impl Into<String>) -> TypeOp {
        TypeOp { opcode, flags, name: name.into() }
    }

    /// Get the op-code value (C++ `TypeOp::getOpcode`).  // STUB(W6)
    pub fn get_opcode(&self) -> OpCode {
        self.opcode
    }

    /// Get the properties associated with the op-code (C++ `TypeOp::getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }

    /// Get the display name of the op-code (C++ `TypeOp::getName`).  // STUB(W6)
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
    /// Display names of the owning Symbol's scope chain, innermost first, from the
    /// Symbol's own scope outward, EXCLUDING the global scope (C++ the scope chain
    /// `Symbol::getScope()` -> `getParent()` ... up to but not including global).
    /// Empty when the Symbol lives directly in the global scope.  Used by the
    /// printer's `pushSymbolScope`/`getResolutionDepth` namespace qualification
    /// (`printc.cc:203`, `database.cc:324`).
    pub scope_path: Vec<String>,
    /// Whether the owning Symbol is a `FunctionSymbol` (C++ the entry's
    /// `getSymbol()` is a `FunctionSymbol`, so `Scope::queryFunction(addr)` returns
    /// its `Funcdata`).  A bare loader function symbol carries a generic code type
    /// with NO recovered prototype, so `query_function` must still resolve it (with
    /// the default void proto) for `ActionDeindirect` to convert a CALLIND through
    /// its address into a direct CALL.
    pub is_function: bool,
    /// The owning FunctionSymbol's parked call-fixup inject id (C++
    /// `FunctionSymbol::getFunction()->getFuncProto().getInjectId()`, set by
    /// `IfcFixupApply` → `setInjectId`, which ALSO marks the proto inline).  `-1`
    /// when none / not a function.  `query_function` carries this onto the proto it
    /// hands `ActionDeindirect` so the deindirect sees an inline (inject-bearing)
    /// callee and restarts — the restart re-flows and the now-direct CALL injects.
    pub func_inject_id: int4,
    /// (kuna) The owning FunctionSymbol's no-return flag (C++
    /// `FunctionSymbol::getFunction()->getFuncProto().isNoReturn()`, set by
    /// `option noreturn <name>` or the no-return analysis pass).  `false` when not
    /// a function.  The C++ `Scope::queryFunction` hands `ActionDeindirect` the
    /// callee's live `Funcdata`, so its no-return flow effect comes for free; the
    /// kuna snapshot has to carry the bit explicitly, and without it a deindirected
    /// CALLIND to a no-return callee never schedules the restart that re-flows the
    /// now-direct CALL with an artificial halt.
    pub func_no_return: bool,
    /// The owning Symbol's id (C++ `SymbolEntry::getSymbol()->getId()`), for the
    /// Phase-4 `<high symref>` echo: ghidra-mode carries the REAL host database
    /// id delivered by getMappedSymbols (0 when the host sent none — the encode
    /// then omits `symref` rather than fabricate an id); the standalone console
    /// snapshot carries the internal `SYMBOL_ID_BASE`-range id, which is legal
    /// on the wire in the native-to-Java direction.
    pub symbol_id: u64,
}

/// A read-only snapshot of the global [`Scope`](crate::database::Scope)
/// sufficient to reproduce `Scope::queryProperties` (`database.cc:1268-1286`)
/// from the [`ArchHandle`] in [`Funcdata::set_varnode_properties`].
///
/// The C++ `glb` is the live `Architecture`, so `localmap->queryProperties` walks
/// the parent chain up to the live global scope.  The merged kuna `glb`
/// ([`ArchContext`]) is a separate IR-boundary skeleton, and the function's
/// `localmap` owns its own detached `Database`; this snapshot is the wire that
/// reconnects the global symbol table onto `glb` so global-mapped varnodes get
/// `persist`/`addrtied` painted and survive `ActionDeadCode`.  Built once per
/// `Funcdata` at [`build_arch_handle`](crate::architecture::Architecture::build_arch_handle).
#[derive(Debug, Clone, Default)]
pub struct GlobalQuery {
    /// Every whole-and-piece mapped storage in the global scope, stably grouped
    /// by address-space index by [`GlobalQuery::new`].
    entries: Vec<GlobalEntry>,
    /// Per-space offset indexes for mapped-storage containment queries.
    space_indexes: Vec<GlobalSpaceIndex>,
    /// The global scope's owned data ranges (C++ `Scope::rangetree`), for the
    /// `inScope` discovery branch of `queryProperties`.
    pub owned: kuna_base::address::RangeList,
    /// The boolean-property map (C++ `Database::flagbase`), for `getProperty`.
    pub flagbase: kuna_base::partmap::PartMap<Address, uint4>,
}

#[derive(Debug, Clone)]
struct GlobalSpaceIndex {
    space_index: int4,
    entry_start: usize,
    entry_end: usize,
    interval_order: Vec<usize>,
    max_last: Vec<u64>,
    leaf_base: usize,
}

impl GlobalSpaceIndex {
    fn new(
        space_index: int4,
        entry_start: usize,
        entry_end: usize,
        entries: &[GlobalEntry],
    ) -> Self {
        let mut interval_order: Vec<usize> = (entry_start..entry_end).collect();
        interval_order.sort_by_key(|&entry_index| entries[entry_index].first);

        let leaf_base = interval_order.len().next_power_of_two();
        let mut max_last = vec![0; leaf_base * 2];
        for (position, &entry_index) in interval_order.iter().enumerate() {
            max_last[leaf_base + position] = entries[entry_index].last;
        }
        for node in (1..leaf_base).rev() {
            max_last[node] = max_last[node * 2].max(max_last[node * 2 + 1]);
        }

        Self {
            space_index,
            entry_start,
            entry_end,
            interval_order,
            max_last,
            leaf_base,
        }
    }

    fn for_each_candidate(
        &self,
        entries: &[GlobalEntry],
        start: u64,
        end: u64,
        visit: &mut impl FnMut(usize),
    ) {
        let limit = self
            .interval_order
            .partition_point(|&entry_index| entries[entry_index].first <= start);
        self.visit_candidates(1, 0, self.leaf_base, limit, end, visit);
    }

    fn visit_candidates(
        &self,
        node: usize,
        node_start: usize,
        node_end: usize,
        limit: usize,
        end: u64,
        visit: &mut impl FnMut(usize),
    ) {
        if node_start >= limit || self.max_last[node] < end {
            return;
        }
        if node_end - node_start == 1 {
            visit(self.interval_order[node_start]);
            return;
        }
        let midpoint = node_start + (node_end - node_start) / 2;
        self.visit_candidates(node * 2, node_start, midpoint, limit, end, visit);
        self.visit_candidates(node * 2 + 1, midpoint, node_end, limit, end, visit);
    }
}

impl GlobalQuery {
    /// Build a snapshot whose entries are grouped by address space while
    /// retaining their original order within each space.
    pub fn new(
        mut entries: Vec<GlobalEntry>,
        owned: kuna_base::address::RangeList,
        flagbase: kuna_base::partmap::PartMap<Address, uint4>,
    ) -> Self {
        entries.sort_by_key(|entry| entry.space_index);
        let mut space_indexes = Vec::new();
        let mut entry_start = 0;
        while entry_start < entries.len() {
            let space_index = entries[entry_start].space_index;
            let entry_end = entry_start
                + entries[entry_start..]
                    .partition_point(|entry| entry.space_index == space_index);
            space_indexes.push(GlobalSpaceIndex::new(
                space_index,
                entry_start,
                entry_end,
                &entries,
            ));
            entry_start = entry_end;
        }
        Self {
            entries,
            space_indexes,
            owned,
            flagbase,
        }
    }

    /// Clone out the flattened entry records (the remote-provider merge input:
    /// the ghidra-mode [`RemoteScope`](crate::remote_provider::RemoteScope)
    /// rebuilds its merged snapshot from the base Database snapshot's entries
    /// plus its decoded ones).
    pub(crate) fn entries_cloned(&self) -> Vec<GlobalEntry> {
        self.entries.clone()
    }

    fn index_for_space(&self, space_index: int4) -> Option<&GlobalSpaceIndex> {
        let position = self
            .space_indexes
            .partition_point(|index| index.space_index < space_index);
        self.space_indexes
            .get(position)
            .filter(|index| index.space_index == space_index)
    }

    /// Return only entries belonging to one address space.
    fn entries_for_space(&self, space_index: int4) -> &[GlobalEntry] {
        let Some(index) = self.index_for_space(space_index) else {
            return &[];
        };
        &self.entries[index.entry_start..index.entry_end]
    }

    /// C++ `Database::getProperty(addr)` (`database.hh:949`): the boolean
    /// properties (read-only/volatile) painted on a memory range.
    fn get_property(&self, addr: &Address) -> uint4 {
        *self.flagbase.get_value(addr)
    }

    /// C++ `Scope::findContainer` (`database.cc:2278-2310`) restricted to the
    /// global scope: the smallest mapped storage containing `[addr, addr+size-1]`
    /// that is in-use at `usepoint`.  Returns its `getAllFlags()`.
    fn find_container_flags(&self, addr: &Address, size: int4, usepoint: &Address) -> Option<uint4> {
        self.find_container_entry(addr, size, usepoint)
            .map(|entry| entry.all_flags)
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
        let space_index = addr.get_space()?.get_index();
        let index = self.index_for_space(space_index)?;
        let start = addr.get_offset();
        // end = addr + size - 1 (uintb wrap), as in find_container.
        // cast: int4 -> u64, the C++ `uintb` widening of the non-negative byte
        // count; a storage size is never negative so sign-extension is irrelevant.
        let end = start.wrapping_add(size as u64).wrapping_sub(1);
        let mut best: Option<usize> = None;
        let mut earliest_exact: Option<usize> = None;
        let mut earliest_smaller: Option<usize> = None;
        index.for_each_candidate(&self.entries, start, end, &mut |entry_index| {
            let entry = &self.entries[entry_index];
            let in_use = if entry.addrtied {
                true
            } else if usepoint.is_invalid() {
                false
            } else {
                entry.uselimit.in_range(usepoint, 1)
            };
            if !in_use {
                return;
            }

            if entry.size == size
                && earliest_exact.is_none_or(|exact_index| entry_index < exact_index)
            {
                earliest_exact = Some(entry_index);
            } else if entry.size < size
                && earliest_smaller.is_none_or(|smaller_index| entry_index < smaller_index)
            {
                earliest_smaller = Some(entry_index);
            }

            let is_better = match best {
                Some(best_index) => {
                    let best_entry = &self.entries[best_index];
                    entry.size < best_entry.size
                        || (entry.size == best_entry.size && entry_index < best_index)
                }
                None => true,
            };
            if is_better {
                best = Some(entry_index);
            }
        });
        let selected = match earliest_exact {
            Some(exact_index)
                if earliest_smaller.is_none_or(|smaller_index| exact_index < smaller_index) =>
            {
                Some(exact_index)
            }
            _ => best,
        };
        selected.map(|entry_index| &self.entries[entry_index])
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
        self.name_for_varnode_scoped(addr, size, usepoint)
            .map(|(n, off, t, _)| (n, off, t))
    }

    /// Like [`name_for_varnode`](Self::name_for_varnode) but also returns the owning
    /// Symbol's scope-name chain (innermost first, global excluded) so the caller can
    /// apply the printer's namespace qualification (`pushSymbolScope`).  Empty chain
    /// for a Symbol that lives directly in the global scope.
    pub fn name_for_varnode_scoped(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(String, int4, Option<std::rc::Rc<crate::dtype::Datatype>>, Vec<String>)> {
        if addr.is_constant() {
            return None;
        }
        let e = self.find_container_entry(addr, size, usepoint)?;
        // sym_off = (access_addr - entry_addr) + entry_offset (C++
        // ScopeLocal::name_for_varnode / SymbolEntry geometry).  `e.first` is the
        // entry's starting offset (== entry addr offset).
        let sym_off = (addr.get_offset().wrapping_sub(e.first) as int4)
            .wrapping_add(e.symbol_offset);
        Some((e.symbol_name.clone(), sym_off, e.symbol_type.clone(), e.scope_path.clone()))
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

#[cfg(test)]
#[path = "context_tests.rs"]
mod tests;

/// Global configuration data for the program being decompiled (C++
/// `Architecture`, owned by `Funcdata` as `glb`).
///
/// STUB(W4): the full `decompiler/cpp/architecture.{hh,cc}` `Architecture` is a
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
/// stub-noted with an explicit `Err`/`None` or take the value as an argument.
pub struct ArchContext {
    /// The address-space manager (`Architecture` derives from
    /// `AddrSpaceManager` in C++).  // STUB(W4)
    ///
    /// Held as an [`Rc`] (LOSS-132): the **single** space set the SLEIGH engine
    /// lifted into is *shared* here, so `glb.manage()` returns the same
    /// `Rc<AddrSpace>` identities and indices the lifted varnodes carry and the
    /// analysis passes key state by.  Hand-built test fixtures still pass an
    /// owned [`AddrSpaceManager`] through [`ArchContext::new`] (wrapped here);
    /// the real lift+analyze path shares the engine's `Rc` via
    /// [`ArchContext::new_shared`].
    pub manage: Rc<AddrSpaceManager>,
    /// Minimum Varnode size to check as a laned register (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // STUB(W4)
    pub min_laned_register_size: int4,
    /// Vector registers that have preferred lane sizes (C++
    /// `Architecture::lanerecords`), built from the pspec `<register_data>`
    /// `vector_lane_sizes` attributes by `Architecture::decodeRegisterData`.
    /// Each [`LanedRegister`](crate::transform::LanedRegister) is keyed by its
    /// whole size (the records are sorted ascending by whole size, one per size),
    /// so [`ArchContext::get_laned_register`] can binary-search on size exactly
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
    /// The processor's floating-point formats (C++ `glb->translate->floatformats`),
    /// shared from the real [`crate::architecture::Architecture`]'s SLEIGH engine
    /// through `build_arch_handle`.  `SubfloatFlow` reaches them off `glb` to
    /// decide the precision-trace and convert constant encodings.  Empty for
    /// hand-built fixtures (those never run float-precision narrowing).
    pub floatformats: Vec<kuna_num::float::FloatFormat>,
    /// The default prototype model (C++ `Architecture::defaultfp`), shared from
    /// the real [`crate::architecture::Architecture`] registry through
    /// `build_arch_handle`.  The proto-recovery actions read it to set the
    /// function's model when the prototype is unrecovered.  `None` for hand-built
    /// fixtures (no proto registry).
    pub defaultfp: Option<Rc<crate::fspec::ProtoModel>>,
    /// The current-evaluation prototype model (C++ `evalfp_current`); falls back
    /// to `defaultfp` when unset.  Shared from the real architecture.
    pub evalfp_current: Option<Rc<crate::fspec::ProtoModel>>,
    /// Default storage location of a function's return address (C++
    /// `Architecture::defaultReturnAddr`), carried from the real architecture's
    /// cspec `<returnaddress>` decode.  `None` (== `space == 0`) for hand-built
    /// fixtures and cspecs without a `<returnaddress>`.  Read by
    /// `Funcdata::test_for_return_address` to detect a BRANCHIND that is really a
    /// tail return through the return-address register.
    pub default_return_addr: Option<kuna_num::pcoderaw::VarnodeData>,
    /// Maximum recursion depth for `Funcdata::ancestorOpUse` (C++
    /// `Architecture::trim_recurse_max`).  Drives `ActionReturnRecovery`'s
    /// ancestor-realism walk.
    pub trim_recurse_max: int4,
    /// Maximum number of references to an implied Varnode before it is forced
    /// explicit (C++ `Architecture::max_implied_ref`, default 2).  Drives
    /// `ActionMarkExplicit::baseExplicit`.
    pub max_implied_ref: int4,
    /// Maximum number of duplicating terms allowed in an implied expression
    /// before the multi-descendant Varnode is forced explicit (C++
    /// `Architecture::max_term_duplication`, default 2).  Drives
    /// `ActionMarkExplicit::processMultiplier`.
    pub max_term_duplication: int4,
    /// (kuna) GH-6990: keep only the first return register (C++ `return_single`).
    pub return_single: bool,
    /// (kuna GH-9218) When adjusting an unjustified input parameter container,
    /// also absorb an overlapping input that extends ABOVE the container's end
    /// (forward scan in `ActionUnjustifiedParams`).  C++ `input_varnode_adjust`.
    pub input_varnode_adjust: bool,
    /// (kuna) Keep a returned register half whose value is a formal input
    /// parameter, instead of dropping it as uncomputed leftover
    /// (`option retinputhalf`); read by [`crate::kuna_retinputhalf`].
    pub ret_input_half: bool,
    /// (kuna) Let a CALL on a block that ends in a no-return halt coexist with the
    /// RETURN's output trial (`option noreturnretuse`); read by
    /// [`crate::p4_calls::kuna_noreturnretuse`].
    pub noreturn_ret_use: bool,
    /// (kuna) `option rustabi` (0 off / 1 auto / 2 always): keep a rustc
    /// two-register `ScalarPair` return intact; read by [`crate::kuna_rustabi`].
    pub rust_abi: u8,
    /// (kuna) The loader's source-language verdict for this image (rustc or not);
    /// what `option rustabi auto` tests.  Copied from the engine `Architecture`
    /// in `build_arch_handle`.
    pub source_is_rust: bool,
    /// (kuna) angr-style default naming: an unknown callee / global prints as
    /// `sub_<addr>` / `dat_<addr>` rather than `func_<addr>` (C++
    /// `Architecture::name_style_angr`, default-on).  Read by the call-spec
    /// printed-name resolution ([`FuncCallSpecs::fspec_printed_name`]).
    pub name_style_angr: bool,
    /// (kuna, Phase 3) Ghidra-convention fallback naming (`FUN_`/`DAT_`/`LAB_`),
    /// set only by the ghidra-mode registerProgram — see
    /// `Architecture::name_style_ghidra`.  Wins over `name_style_angr` in
    /// [`Self::kuna_name_style`].
    pub name_style_ghidra: bool,
    /// (kuna) Collapse local-variable declarations whose fully-rendered line is
    /// identical (the scalar analogue of the composite-symbol decl collapse).  Read
    /// by [`PrintC::emit_local_var_decls`](crate::printc); `option dedupvardecls`,
    /// default-off (angr-inspired, S9).
    pub dedup_var_decls: bool,
    /// (kuna) GH-558: present canonicalized `INT_LESS(x, c+1)` comparisons in
    /// their original `x <= c` form (C++ `present_lessequal`, DIV-2 default-on).
    /// Read by [`ActionPresentCompareForm`](crate::kuna_compareform::ActionPresentCompareForm).
    pub present_lessequal: bool,
    /// (kuna) GH-1282: fold `(b<<k) s>> k` boolean sign-extension-mask idioms
    /// (C++ `fold_boolean_mask`, DIV-2 default-on).  Read by
    /// [`RuleBoolSignShift`](crate::kuna_booleanmask::RuleBoolSignShift).
    pub fold_boolean_mask: bool,
    /// (kuna) refuse to split a shared RETURN block that stores to GLOBALS
    /// (`retsplitglobal`).  Read by
    /// [`Funcdata::return_split_is_splittable`](crate::funcdata::Funcdata),
    /// the predicate both `ActionReturnSplit` and `ActionReturnDup` consult.
    pub ret_split_global: bool,
    /// (kuna) resolve a one-byte lane read of a CONSTANT-mask `pshufb` shuffle
    /// to the source lane it selects (`simdlane`).  Read by
    /// [`RuleSimdShuffleLane`](crate::kuna_simdlane::RuleSimdShuffleLane).
    pub simd_lane_fold: bool,
    /// (kuna) CALLOTHER user-op ids the architecture registered under a byte
    /// shuffle name ([`SHUFFLE_USEROP_NAMES`](crate::kuna_simdlane::SHUFFLE_USEROP_NAMES)),
    /// resolved once per program in `Architecture::build_arch_handle` so
    /// [`RuleSimdShuffleLane`](crate::kuna_simdlane::RuleSimdShuffleLane) can
    /// name a CALLOTHER without reaching the full `Architecture`.
    pub simd_shuffle_userops: Vec<kuna_base::types::uint4>,
    /// (kuna) GH-1276/8777: fold flag-modelled comparison idioms (C++
    /// `fold_flag_compare`, DIV-3 default-on).  Read by
    /// [`RuleBoolSignLess`](crate::kuna_flagcompare::RuleBoolSignLess) /
    /// [`RuleSborrowGe`](crate::kuna_flagcompare::RuleSborrowGe).
    pub fold_flag_compare: bool,
    /// (kuna) GH-8913: fuse 8-bit carry-chain 16-bit adds into one wide add (C++
    /// `add_carry_chain`, DIV-2 default-on).  Read by
    /// [`RuleAddCarryChain`](crate::kuna_addcarrychain::RuleAddCarryChain).
    pub add_carry_chain: bool,
    /// (kuna) GH-7190: collapse the OV-flag signed-less-than idiom to INT_SLESS
    /// (C++ `ov_less_simplify`, DIV-2 default-on).  Read by
    /// [`RuleOvLessSimplify`](crate::kuna_ovlesssimplify::RuleOvLessSimplify).
    pub ov_less_simplify: bool,
    /// (kuna) GH-8724: re-express a strided-induction offset as counter*stride
    /// (C++ `recover_array_stride`, DIV-3 default-on).  Read by
    /// [`RuleArrayStride`](crate::kuna_arraystride::RuleArrayStride).
    pub recover_array_stride: bool,
    /// (kuna) GH-9230/1537: recover constant-fill store/copy runs as
    /// `builtin_memset` (C++ `memset_recover`, DIV-2 default-on).  Read by
    /// [`RuleMemsetCopy`](crate::kuna_memsetsequence::RuleMemsetCopy).
    pub memset_recover: bool,
    /// (kuna `rodatastring`) Collapse a read-only string block copy into
    /// `builtin_strncpy`.  Read by
    /// [`RuleRodataStringCopy`](crate::kuna_rodatastring::RuleRodataStringCopy).
    pub rodata_string: bool,
    /// (kuna `ptrdepthcap`) Refuse to deepen an already unsatisfiable pointer
    /// equation while propagating types: a candidate `ptr(ptr(ptr(..)))` collapses
    /// to `ptr(undefined<N>)`, which is a fixed point.  Read by
    /// `ActionInferTypes::propagateTypeEdge` (`coreaction_infertypes`); the
    /// mechanism lives in `kuna_ptrdepth`.
    pub ptrdepthcap: bool,
    /// (kuna) GH-8017: resolve the gcc stack-probe loop SP MULTIEQUAL to a
    /// constant (C++ `model_stack_probe_loop`, DIV-3 default-on).  Read by
    /// [`RuleStackProbeLoop`](crate::kuna_stackprobeloop::RuleStackProbeLoop).
    pub model_stack_probe_loop: bool,
    /// (kuna) reconstruct a compiler-lowered comparison cascade into a switch
    /// (C++ `recover_lowered_switch`, default-on).  Read by the
    /// [`crate::kuna_loweredswitch`] detect/install actions.
    pub recover_lowered_switch: bool,
    /// (kuna) recover stack-passed call arguments (default-on; upstream-faithful).
    /// Read by [`check_input_trial_use`](crate::funcdata_callsite::check_input_trial_use)
    /// through [`crate::p4_calls::kuna_callsitestackargs::outside_caller_local_range`].
    pub callsite_stack_args: bool,
    /// (kuna) let a bounded decode of the callee's own body veto a register
    /// argument the callee provably never reads (`calleedeadarg`).  Read by
    /// [`check_input_trial_use`](crate::funcdata_callsite::check_input_trial_use)
    /// through [`crate::p4_calls::kuna_calleedeadarg::trial_is_dead_in_callee`].
    pub callee_dead_arg: bool,
    /// (kuna) in the function's OWN input recovery, tolerate a run of unused
    /// argument REGISTERS before a live-in register (`inputparamgap`).  Read by
    /// `ActionInputPrototype`, which stamps it onto the
    /// [`ParamActive`](crate::fspec::ParamActive) that
    /// [`ParamListStandard::fillin_map`](crate::fspec::ParamListStandard) then
    /// consults through
    /// [`crate::p4_calls::kuna_inputparamgap::gap_slot_is_exempt`].
    pub input_param_gap: bool,
    /// (kuna) score a variadic call's stack arguments as their own `fillinMap`
    /// resource section (`varargstackargs`).  Read by
    /// [`ParamListStandard::fillin_map`](crate::fspec::ParamListStandard) through
    /// [`crate::p4_calls::kuna_varargstackargs::stack_section_split`], via the
    /// flag `ActionActiveParam` writes onto the call's `ParamActive`.
    pub vararg_stack_args: bool,
    /// (kuna) reconcile a call's recovered argument list with a sibling call to
    /// the same callee (`calleearity`).  Read by
    /// [`build_input_from_trials`](crate::funcdata_callsite::build_input_from_trials)
    /// through [`crate::p4_calls::kuna_calleearity::unify_with_sibling_call`].
    pub callee_arity: bool,
    /// (kuna) retry that reconciliation against sibling calls that finalize
    /// LATER in the same `ActionActiveParam` pass (`calleearityfwd`).  Read by
    /// [`crate::p4_calls::kuna_calleearityfwd::capture_empty_call`]; inert
    /// unless `callee_arity` is also set.
    pub callee_arity_fwd: bool,
    /// (kuna) Extend a PARTIALLY recovered argument list to a sibling call's,
    /// when the callee's own body proves the missing registers are the ones it
    /// reads (`calleearitylive`).  Read by
    /// [`crate::p4_calls::kuna_calleearitylive::capture_partial_call`]; inert
    /// unless `callee_arity` is also set.
    pub callee_arity_live: bool,
    /// (kuna) completion level for the two upstream partial-range call-overlap
    /// guards (`calloverlap`): `0` = both stay inert (what kuna shipped before the
    /// option), `1` = `Heritage::guardCallOverlappingInput` only, `2` = that plus
    /// `Heritage::tryOutputOverlapGuard`, i.e. upstream Ghidra.  Read by
    /// [`Heritage::guard_calls`](crate::p3_dataflow::heritage::Heritage) at both
    /// `ContainedBy` arms; the level vocabulary lives in
    /// [`crate::p3_dataflow::kuna_calloverlap`].
    pub call_overlap: int4,
    /// (kuna) predicate strength for tolerating a caller-save spill among a
    /// trial Varnode's descendants (`spillargtrial`): `0` = upstream, every
    /// `CPUI_STORE` rejects, `1` = tolerate a store that is one half of a
    /// spill/reload pair, `2` = tolerate any caller-frame store of the value.
    /// Read by [`Funcdata::only_op_use`](crate::funcdata::Funcdata) through
    /// [`crate::p4_calls::kuna_spillargtrial::store_is_caller_save_spill`].
    pub spill_arg_trial: int4,
    /// (kuna) `option loadguardrange`: run the upstream ValueSet solver over
    /// new indexed-stack LOAD/STORE guard pointers at the end of each heritage
    /// pass (`Heritage::analyzeNewLoadGuards`, heritage.cc:834), refining each
    /// [`LoadGuard`](crate::p3_dataflow::heritage::LoadGuard)'s
    /// min/max/step so `MapState::addGuard` can supply real array extents in
    /// P6.  Off keeps every guard at the whole-space range with `step == 0`
    /// (the pre-port behavior).  Read by
    /// [`Heritage::heritage`](crate::p3_dataflow::heritage::Heritage::heritage).
    pub load_guard_range: bool,
    /// (kuna) `option tiedstorekeep` (default-on, DIV-105): refuse the
    /// `RulePropagateCopy` marker propagation that would leave an address-tied
    /// `COPY` output holding a call's return value with no readers, so a
    /// `local = f();` frame store survives dead-code elimination instead of
    /// vanishing from the emitted C.  Read by
    /// [`crate::p3_dataflow::kuna_tiedstorekeep::declines`].
    pub tied_store_keep: bool,
    /// (kuna) region-based (Phoenix/SAILR) structurer: structure the CFG by
    /// walking the [`KunaRegionIdentifier`](crate::p7_regions::kuna_regionid)
    /// region tree and matching Phoenix acyclic schemas instead of running
    /// Ghidra's `CollapseStructure` (`region_structure`, DIV-12 default-on: the
    /// primary structuring path; falls back to `CollapseStructure` on irreducible code).
    /// Read by [`ActionBlockStructure`](crate::blockaction::ActionBlockStructure).
    pub region_structure: bool,
    /// (kuna) source-layout tie-break for `ruleBlockIfNoExit`'s clause arm
    /// (`guard_arm`, option `guardarm`, opt-in default-off).  When set and BOTH
    /// out-arms are eligible clauses, the earlier-addressed arm becomes the `if`
    /// clause instead of out-index 0.  Read by
    /// [`ActionBlockStructure`](crate::blockaction::ActionBlockStructure).
    pub guard_arm: bool,
    /// (kuna) defer a live loop head in the deferred `ruleBlockIfNoExit` scan
    /// (`loop_cond_hoist`, option `loopcondhoist`, opt-in default-off) so
    /// `ruleBlockWhileDo` keeps the loop's head test.  Read by
    /// [`ActionBlockStructure`](crate::blockaction::ActionBlockStructure).
    pub loop_cond_hoist: bool,
    /// (kuna) region structurer cyclic loop-successor refinement
    /// (`region_loop_refine`, opt-in default-off).  When set (and
    /// `region_structure` is on), multi-exit / multi-latch / mid-entry loops are
    /// refined (secondary exits & latches virtualized to break/continue gotos) so
    /// they fold into structured loops instead of falling back to
    /// `CollapseStructure`.  Read by
    /// [`crate::p8_structure::region_structurer::run_region_structurer`].
    pub region_loop_refine: bool,
    /// (kuna) region structurer last-resort edge-virtualization ORDERING (SAILR P2,
    /// `region_edge_order`, opt-in default-off).  When set (and `region_structure`
    /// is on), the structurer orders the edges it virtualizes to gotos via angr's
    /// dominance-tiered bucketing (crossing / secondary / other) + the H2 post-
    /// dominator heuristic (capped), instead of the flat H1/H3 + address ordering.
    /// Only changes WHICH goto is chosen when virtualizing, so OFF is byte-identical.
    /// Read by [`crate::p8_structure::region_structurer::run_region_structurer`].
    pub region_edge_order: bool,
    /// (kuna) short-circuit condition folding across a COMPLEX sibling block
    /// (`cond_fold`, option `condfold`, opt-in default-off).  When set, the
    /// short-circuit schema of BOTH structurers accepts a *complex* sibling
    /// condition block when it fits the level's printed-width budget AND either
    /// admission rule takes it — Rule A, a `BlockCopy` of one branch-free,
    /// comment-free `BlockBasic` with at most one statement-root call; or Rule B, the
    /// statement-shape allowlist, which also admits a nested `BlockCondition` so a
    /// guard cascade can fold.  The absorbed statements then render inside the
    /// `&&`/`||` operand as a C comma expression (angr Phoenix's
    /// `MultiStatementExpression`).  Read by
    /// [`crate::p8_structure::kuna_condfold::compute_condfold_sets`] at both
    /// precompute sites.  The value is the shared printed-width budget, which
    /// doubles as the option's on/off sentinel: `0` = off, `5` = `on` (angr
    /// parity), `9` = `wide`.
    /// (kuna) `outline` region spec (empty = off), option `outline`.
    pub outline_spec: String,
    /// (kuna) `option cleanupcode`: delete the Rust drop/deallocate call sites
    /// (`core::ptr::drop_in_place`, `<T as core::ops::drop::Drop>::drop`,
    /// `alloc::raw_vec::RawVecInner::deallocate`, `__rust_dealloc`) from the
    /// pre-SSA op graph.  Read by
    /// [`ActionRemoveCleanupCode`](crate::p2_lift::kuna_cleanupcode::ActionRemoveCleanupCode).
    pub remove_cleanup_code: bool,
    /// (kuna) `option linuxsyscall`: rewrite a 32-bit Linux `int 0x80` from an
    /// indirect call through the `swi` userop into a named call on the syscall
    /// the number in `EAX` selects.  Read by
    /// [`ActionLinuxSyscall`](crate::p2_lift::kuna_linuxsyscall::ActionLinuxSyscall).
    pub linux_syscall: bool,
    /// (kuna) `option switchselector`: refuse a recovered lowered-switch record
    /// whose synthesized BRANCHIND would not get the switch value as its
    /// selector.  Read by
    /// [`install_selector_is_sound`](crate::p2_lift::kuna_loweredswitch::install_selector_is_sound).
    pub switch_selector_guard: bool,
    pub cond_fold: int4,
    /// (kuna) angr SAILR goto-reduction: duplicate a small return tail into a
    /// `goto` source (`reduce_return_gotos`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_gotoreduce`]'s `ActionGotoReduce`.
    pub reduce_return_gotos: bool,
    /// (kuna) angr `IfElseFlattener`: drop the `else` arm of a terminating-if
    /// 3-component `if` (`flatten_ifelse`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_ifelseflatten`]'s `ActionIfElseFlatten`.
    pub flatten_ifelse: bool,
    /// (kuna) angr SAILR `CrossJumpReverter`: duplicate a small *non-return*
    /// cross-jump tail into the `goto` source (`revert_cross_jumps`, opt-in
    /// default-off).  Read by
    /// [`crate::p8_structure::kuna_crossjumpreverter`]'s `ActionCrossJumpReverter`.
    pub revert_cross_jumps: bool,
    /// (kuna) angr SAILR `ReturnDuplicatorLow`: duplicate a small **return tail that
    /// contains a call** (e.g. `free(p); return;`) into the `goto` source
    /// (`dup_return_call_tails`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_taildup`]'s `ActionTailDup`.
    pub dup_return_call_tails: bool,
    /// (kuna) angr structurer ITE region-dedup: merge a duplicated `if/else` tail
    /// (shared prefix/suffix across both arms) by hoisting the shared blocks out of
    /// the `if` — the inverse of the SAILR duplication passes (`dedup_ite_tail`,
    /// opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_dedupitetail`]'s `ActionDedupIteTail`.
    pub dedup_ite_tail: bool,
    /// (kuna) angr `ITERegionConverter`: rewrite a two-arm assignment *diamond*
    /// (`if (c) v = A; else v = B;`) to a `?:` ternary (`v = c ? A : B;`)
    /// (`iteregion`, opt-in default-off — a **runtime choice**: matches source only
    /// when the source used a ternary).  Read by
    /// [`crate::p8_structure::kuna_iteregion`]'s `ActionIteRegion`.
    pub iteregion: bool,
    /// (kuna) `iteexpr`: extend iteregion to computed-expression diamond arms (see
    /// `Architecture::iteexpr`).  Per-function snapshot copy.
    pub iteexpr: bool,
    /// (kuna) `iteboolean`: re-roll a short-circuit `0`/`1` select diamond into a
    /// single boolean assignment (`x = a && b;`) — see `Architecture::iteboolean`.
    /// Read by [`crate::p8_structure::kuna_iteboolean`]'s `ActionIteBoolean`.
    pub iteboolean: bool,
    /// (kuna) `itecondlist`: let the `iteregion`/`iteboolean` diamond matchers descend
    /// a multi-component `BlockList` in the condition position to its last component —
    /// see `Architecture::itecondlist`.  Read by
    /// [`crate::p8_structure::kuna_itecondlist::cond_list_tail`].
    pub itecondlist: bool,
    /// (kuna) `orchain`: decline a `returndup` split whose shared RETURN block is the
    /// out-target two conditionals must keep in common for
    /// `CollapseStructure::rule_block_or` to fuse them — see
    /// `Architecture::returndup_orchain`.  Read by
    /// [`crate::p8_structure::kuna_orchain::shortcircuit_shared_targets`].
    pub returndup_orchain: bool,
    /// (kuna) `paramcopyhoist`: anchor an unmodified incoming parameter's trim COPY
    /// in the entry block rather than at the MULTIEQUAL slot's predecessor tail —
    /// see `Architecture::param_copy_hoist`.  Read by
    /// [`crate::p6_variables::kuna_paramcopyhoist`]'s `hoist_target`.
    pub param_copy_hoist: bool,
    /// (kuna) angr SAILR gotoless `ReturnDuplicatorHigh`: duplicate a shared
    /// **bare-epilogue** RETURN block into each predecessor but one so a
    /// `if (c) { body; return X; } return Y;` guard shape structures as per-predecessor
    /// early returns (`duplicate_shared_returns`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_returndup`]'s `ActionReturnDup`.
    pub duplicate_shared_returns: bool,
    /// (kuna) hoist a leading const-guard into an early return by peeling only the
    /// CONSTANT arm of a mixed return phi (`early_return`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_earlyreturn`]'s `ActionEarlyReturn`.
    pub early_return: bool,
    /// (kuna) continuation of `early_return` to the WIDE multi-way switch-phi return: peel
    /// each constant case arm to per-case `return K` past earlyreturn's 16-in-edge cap
    /// (`switch_return`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_switchreturn`]'s `ActionSwitchReturn`.
    pub switch_return: bool,
    /// (kuna) lower loop-exit `goto <successor>` edges to structured `break;`
    /// (a port of Ghidra `BlockGraph::scopeBreak`, DIV-10 default-on).  Read by
    /// [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure) to gate
    /// [`crate::kuna_loopbreak_recovery::kuna_scope_break`].
    pub recover_loop_break: bool,
    /// (kuna) fold an order-safe single-use call return into its use site
    /// (`fold_call_returns`, opt-in default-off).  Read by `base_explicit`
    /// (`ActionMarkExplicit`) via [`crate::kuna_callretfold::call_output_foldable`].
    pub fold_call_returns: bool,
    /// (kuna) strip the glibc -fstack-protector canary epilogue (C++
    /// `strip_stack_guard`, opt-in default-off).  Read by
    /// [`crate::kuna_stackguard`]'s `ActionStripStackGuard`.
    pub strip_stack_guard: bool,
    /// (kuna) strip rustc's bounds / slice / divide-by-zero panic branches
    /// (option `securitycheck`, DIV-82 default-on).  Read by
    /// [`crate::kuna_securitycheck`]'s `ActionRemoveSecurityCheck`.
    pub strip_security_check: bool,
    /// (kuna) flip negated-guard if/else branches for linearity (option
    /// `branchflip`, opt-in default-off).  Read by
    /// [`crate::p8_structure::kuna_branchflip`]'s `ActionBranchFlip`.
    pub branch_flip: bool,
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
    /// Ignore NaN operations entirely, assuming `nan()` always returns false
    /// (C++ `Architecture::nan_ignore_all`, default-off).  Read by
    /// [`RuleIgnoreNan`](crate::ruleaction_7::RuleIgnoreNan); set via the
    /// `nanignore all` option and shared from the real
    /// [`crate::architecture::Architecture`] through `build_arch_handle`.
    pub nan_ignore_all: bool,
    /// The data-type factory (C++ `glb->types`), shared from the real
    /// [`crate::architecture::Architecture`] through `build_arch_handle`.
    /// `ActionInferTypes` reaches `getBase`/`getTypePointer`/`down_chain` through
    /// it.  `None` for hand-built fixtures (no type factory registry).
    pub types: Option<Rc<crate::dtype::TypeFactoryImpl>>,
    /// The decoded-string manager (C++ `glb->stringManager`), shared from the
    /// real [`crate::architecture::Architecture`] through `build_arch_handle`.
    /// `Funcdata::getInternalString` (the `RuleStringStore`/`RuleStringCopy`
    /// transform) registers the assembled byte-array as an internal string here;
    /// the printer reads it back through the same shared instance on the real
    /// architecture.  `None` for hand-built fixtures (no internal strings).
    pub internal_strings:
        Option<Rc<std::cell::RefCell<crate::stringmanage::StringManagerUnicode>>>,
    /// Maximum number of entries in a single JumpTable (C++
    /// `Architecture::max_jumptable_size`), shared from the real architecture.
    /// Read by `JumpTable::recoverModel`.  `0` for hand-built fixtures.
    pub max_jumptable_size: uint4,
    /// Aliases blocked by 0=none,1=struct,2=array,3=all (C++
    /// `Architecture::alias_block_level`), shared from the real architecture.
    /// Read by `ScopeLocal::markUnaliased`.  Defaults to 2 (block structs and
    /// arrays), matching the real architecture's startup default.
    pub alias_block_level: int4,
    /// How many low bits of a recovered function pointer are forced to zero (C++
    /// `Architecture::funcptr_align`), shared from the real architecture.  Read
    /// by `JumpBasic::buildAddresses` to align recovered targets.  `0` (no
    /// alignment) for hand-built fixtures.
    pub funcptr_align: int4,
    /// (kuna) PE import-call binding (`option peimportcall`), shared from the real
    /// architecture.  Read by [`Self::query_function`]: with the gate on, a resolved
    /// callee's no-return flag rides the prototype handed to `ActionDeindirect`, so a
    /// deindirected `call [IAT slot]` to `ExitThread` restarts and re-flows with the
    /// artificial halt instead of falling through into the next function.  `false`
    /// for hand-built fixtures.
    pub peimportcall: bool,
    /// (kuna GH-8471) Keep a mode-bit-encoded (Thumb) function pointer symbolic
    /// (`PTRSUB(fn) + 1`) instead of letting `RulePtrsubUndo` collapse it back to
    /// raw hex (C++ `Architecture::preserve_thumb_funcptr`), shared from the real
    /// architecture.  Read by [`RulePtrsubUndo`](crate::ruleaction_6::RulePtrsubUndo)'s
    /// thumb-funcptr guard.  Default-on (DIV-2); `false` for hand-built fixtures.
    pub preserve_thumb_funcptr: bool,
    /// (kuna) Bound a LOAD-table jumptable by a modulo/and-mask on its index when
    /// the basic model fails to bound it (C++ `Architecture::switch_modulo_bound`,
    /// flipped by `option switchmodbound`, GH-9191), shared from the real
    /// architecture.  Read by `JumpBasic::recoverModel` before
    /// `kunaTryModuloBoundTable`.  `false` (default off / upstream byte-identical).
    pub switch_modulo_bound: bool,
    /// (kuna) Bound a LOAD-table jumptable by an out-of-band CBRANCH range guard
    /// when the basic model's guard analysis could not (C++
    /// `Architecture::switch_guard_bound`, flipped by `option switchguardbound`,
    /// angr `test_decompiling_missing_function_call`), shared from the real
    /// architecture.  Read by `JumpBasic::recoverModel` before
    /// `kuna_try_guard_bound_table`.  `false` (default off / upstream byte-identical).
    pub switch_guard_bound: bool,
    /// (kuna) Recover a GCC PIC relative-offset jump table whose base register is a
    /// loop-carried MULTIEQUAL, so the path-meld collapses and the CBRANCH range
    /// guard on the load index never bounds the table (C++
    /// `Architecture::switch_shared_case`, flipped by `option switchsharedcase`,
    /// angr `test_switch_case_shared_case_nodes_b2sum_digest`), shared from the
    /// real architecture.  Read by `JumpBasic::recoverModel` before
    /// `kuna_try_loop_carried_guard_table`.  `false` (default off / upstream
    /// byte-identical).
    pub switch_shared_case: bool,
    /// (kuna) Recover an image-base-relative jump table whose bound guard is
    /// "unrolled" / duplicated across multiple predecessors of the dispatch
    /// block (the `BRANCHIND` parent has `sizeIn() > 1`) (C++
    /// `Architecture::switch_multi_pred`, flipped by `option switchmultipred`,
    /// angr `test_decompiling_abnormal_switch_case_case3`), shared from the real
    /// architecture.  Read by `JumpBasic::checkUnrolledGuard` to gate the
    /// multi-predecessor unrolled-guard walk.  `false` (default off / upstream
    /// byte-identical).
    pub switch_multi_pred: bool,
    /// (kuna) Recover the interleaved jump tables of an MSVC optimized
    /// memcpy/memmove by making the jump-table-recovery partial clone's
    /// `collect_edges` SKIP a recovered sibling table's case-target edge whose
    /// op was not decoded into this partial's `visited` (C++
    /// `Architecture::unrolled_guard`, flipped by `option unrolledguard`, angr
    /// `test_decompiling_optimized_memcpy`), shared from the real architecture.
    /// Read by `FlowInfo::collectEdges`.  `false` (default off / upstream
    /// byte-identical).
    pub unrolled_guard: bool,
    /// (kuna) Share ONE jump-table partial sub-decompilation across every table
    /// recovered in a single `FlowInfo::recoverJumpTables` batch instead of
    /// re-cloning and re-analysing the function per table (C++
    /// `Funcdata::stageJumpTable`'s `if (!partial.isJumptableRecoveryOn())`
    /// guard), flipped by `option jtsharepartial`, shared from the real
    /// architecture.  Read by `Funcdata::stage_jump_table`.
    pub jumptable_share_partial: bool,
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
    /// (kuna `dynrelocs`) `[start, stop]` (inclusive) offsets whose read-only
    /// contents are foldable even with [`Self::readonlypropagate`] off: the
    /// dynamic-relocation slots the loader filled in that `PT_GNU_RELRO` freezes.
    /// The value is the linker's, the memory is `mprotect`ed read-only before
    /// `main`, and folding it is what turns a call through a GOT slot back into a
    /// named call — but `option readonly` is a program-wide switch, so these
    /// ranges carry the exception. Sorted ascending, disjoint, and EMPTY for
    /// every hand-built fixture and every XML `<binaryimage>` load, which is why
    /// the datatest parity oracle cannot see this field.
    pub dynreloc_const: Rc<Vec<(u64, u64)>>,
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
    /// (kuna, Phase 3) The ghidra-mode lazy symbol provider
    /// ([`RemoteScope`](crate::remote_provider::RemoteScope)); when installed,
    /// every global-scope read below queries through it (fetching unresolved
    /// addresses from the host) instead of the frozen `global_query`/
    /// `callee_protos` snapshots.  `None` on the standalone path, which stays
    /// byte-identical.
    pub remote_scope: Option<Rc<crate::remote_provider::RemoteScope>>,
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
    /// Snapshot of the engine's tracked-register database (C++ `glb->context`'s
    /// track base, populated by `set track <reg> <val> [start end]`).  The
    /// per-function `glb` skeleton does not hold the `ContextDatabase`, so
    /// `build_arch_handle` clones the part-map here; [`ActionConstbase`](crate::
    /// coreaction_early::ActionConstbase) queries it for the entry address to emit
    /// the `COPY #val -> reg` ops (`coreaction.cc:707`).  Empty for hand-built
    /// fixtures and when no register is tracked.
    pub tracked_sets: kuna_base::partmap::PartMap<Address, kuna_sleigh::globalcontext::TrackedSet>,
}

impl ArchContext {
    /// Construct the skeleton from an owned [`AddrSpaceManager`] (STUB(W4)).
    ///
    /// Used by hand-built test fixtures; the real path shares the engine's
    /// manager through [`Architecture::new_shared`].
    pub fn new(manage: AddrSpaceManager) -> ArchContext {
        ArchContext::new_shared(Rc::new(manage))
    }

    /// Construct the skeleton sharing the engine's single [`AddrSpaceManager`]
    /// (LOSS-132 keystone).  The `Rc` is the one the SLEIGH translator
    /// populated (with fspec/iop/join inserted by `Architecture::restoreFromSpec`),
    /// so the lifted varnodes and the analysis passes see the same spaces.
    pub fn new_shared(manage: Rc<AddrSpaceManager>) -> ArchContext {
        // C++ default: getMinimumLanedRegisterSize() returns the configured
        // minimum; the upstream default when unset is 4.
        ArchContext {
            manage,
            min_laned_register_size: 4,
            lanerecords: Vec::new(),
            opbehaviors: Vec::new(),
            floatformats: Vec::new(),
            defaultfp: None,
            evalfp_current: None,
            default_return_addr: None,
            // C++ Architecture default: trim_recurse_max = 5 (resetDefaults).
            trim_recurse_max: 5,
            // C++ Architecture default: max_implied_ref = 2 (resetDefaults).
            max_implied_ref: 2,
            // C++ Architecture default: max_term_duplication = 2 (resetDefaults).
            max_term_duplication: 2,
            return_single: false,
            // (kuna) GH-9218 DIV-3 default-on; the real value is copied from the
            // engine Architecture in `build_arch_handle`.
            input_varnode_adjust: true,
            // (kuna) `option retinputhalf` default-on; the real value is copied
            // from the engine Architecture in `build_arch_handle`.
            ret_input_half: true,
            // (kuna) `option noreturnretuse` default-on; the real value is copied
            // from the engine Architecture in `build_arch_handle`.
            noreturn_ret_use: true,
            // (kuna) `option rustabi` default-off; the real value is copied from
            // the engine Architecture in `build_arch_handle`.
            rust_abi: 0,
            source_is_rust: false,
            // (kuna) angr-style default naming is default-on (Architecture::reset).
            name_style_angr: true,
            // (kuna, Phase 3) ghidra-mode-only; never set on the standalone path.
            name_style_ghidra: false,
            // (kuna) default-off opt-in; the real value is copied from the engine
            // Architecture in `build_arch_handle` (`option dedupvardecls`).
            dedup_var_decls: false,
            // (kuna) DIV-2 default-on (GH-558): resetDefaults sets present_lessequal=true.
            present_lessequal: true,
            // (kuna) the real arch overwrites each of these in `build_arch_handle`;
            // hand-built fixtures (no `build_arch_handle`) get `false`, so a rule
            // registered `enabled=false` is inert there — matching the gate-off
            // unit tests (and the `infer_funcentry` stub-default convention).
            fold_boolean_mask: false,    // GH-1282 booleanmask
            ret_split_global: false,     // retsplitglobal (the ArchSeam carries the real default)
            simd_lane_fold: false,       // simdlane (the ArchSeam carries the real default)
            simd_shuffle_userops: Vec::new(),
            fold_flag_compare: false,    // GH-1276/8777 flagcompare
            add_carry_chain: false,      // GH-8913 addcarrychain
            ov_less_simplify: false,     // GH-7190 ovlesssimplify
            recover_array_stride: false, // GH-8724 arraystride
            memset_recover: false,       // GH-9230/1537 memsetrecover
            rodata_string: false,        // (kuna) rodatastring
            ptrdepthcap: false,          // (kuna) option ptrdepthcap
            model_stack_probe_loop: false, // GH-8017 stackprobeloop
            recover_lowered_switch: false, // loweredswitch
            // callsitestackargs is a correctness fix, not an opt-in transform, so the
            // hand-built-fixture seam carries the same default the real path does.
            callsite_stack_args: true,
            // calleedeadarg only ever REMOVES an argument, and only against a
            // decoded callee body; the fixture seam carries the real default.
            callee_dead_arg: true,
            input_param_gap: true,
            vararg_stack_args: true,     // varargstackargs (DIV-101 default-on)
            callee_arity: true,          // calleearity (DIV-102 default-on)
            callee_arity_fwd: true,      // calleearityfwd (default-on)
            callee_arity_live: true,     // calleearitylive (default-on)
            call_overlap: 0,             // calloverlap (0 = both overlap guards inert)
            spill_arg_trial: 0,          // spillargtrial (0 = upstream: every STORE rejects)
            load_guard_range: true,      // loadguardrange (upstream behavior, default-on)
            tied_store_keep: false,      // tiedstorekeep (Architecture::reset_defaults sets the shipped default: on)
            region_structure: false,     // regionstructure (opt-in default-off)
            guard_arm: false,            // guardarm (opt-in default-off)
            loop_cond_hoist: false,      // loopcondhoist (opt-in default-off)
            region_loop_refine: false,   // regionlooprefine (opt-in default-off)
            region_edge_order: false,    // regionedgeorder (opt-in default-off)
            outline_spec: String::new(), // outline (opt-in default-off; empty = off)
            remove_cleanup_code: true,   // cleanupcode (DIV-81 default-on; inert on a non-Rust binary)
            linux_syscall: false,        // linuxsyscall (opt-in default-off)
            switch_selector_guard: false, // switchselector (opt-in default-off)
            cond_fold: 0,                // condfold (opt-in default-off; 0 = off)
            reduce_return_gotos: false,  // gotoreduce (opt-in default-off)
            flatten_ifelse: false,  // ifelseflatten (opt-in default-off)
            revert_cross_jumps: false,   // crossjumprevert (opt-in default-off)
            dup_return_call_tails: false, // taildup (opt-in default-off)
            dedup_ite_tail: false,        // dedupitetail (opt-in default-off)
            iteregion: false,             // iteregion (opt-in default-off, runtime-choice)
            iteexpr: false,               // iteexpr (computed-arm ?: extension, default-off)
            iteboolean: false,            // iteboolean (0/1 select -> boolean assignment)
            itecondlist: false,           // itecondlist (condition-list tolerance, default-off)
            param_copy_hoist: false,      // paramcopyhoist (parameter copy-shadow -> entry block)
            returndup_orchain: false,     // orchain (short-circuit chain protection, default-off)
            duplicate_shared_returns: false, // returndup (opt-in default-off)
            early_return: false, // earlyreturn (opt-in default-off)
            switch_return: false, // switchreturn (opt-in default-off)
            recover_loop_break: false,   // loopbreak_recovery (opt-in default-off)
            fold_call_returns: false, // foldcallret (opt-in default-off)
            strip_stack_guard: false,    // stackguard (opt-in default-off)
            strip_security_check: false, // securitycheck (fixture default-off; the live gate rides build_arch_handle)
            branch_flip: false,          // branchflip (opt-in default-off)
            // (kuna) DIV-3 default-on (GH-9203): architecture.cc sets condexe_block_placement=true.
            condexe_block_placement: true,
            // C++ Architecture default: analyze_for_loops = true (architecture.cc).
            analyze_for_loops: true,
            // C++ Architecture default: nan_ignore_all = false (architecture.cc:1453);
            // only `nanignore all` flips it. nan_ignore_compare drives the rule's
            // enabled state at registration, not this flag.
            nan_ignore_all: false,
            types: None,
            internal_strings: None,
            max_jumptable_size: 0,
            alias_block_level: 2, // Architecture default: block structs and arrays
            funcptr_align: 0,
            peimportcall: false, // (kuna) the real arch overwrites this in build_arch_handle
            // (kuna GH-8471) DIV-2 default-on; the real arch overwrites this in
            // build_arch_handle.  A hand-built fixture has funcptr_align == 0, so
            // the thumb guard never fires regardless of this flag.
            preserve_thumb_funcptr: true,
            switch_modulo_bound: false, // (kuna) GH-9191 default off (upstream byte-identical)
            switch_guard_bound: false, // (kuna) angr opt-in default off (upstream byte-identical)
            switch_shared_case: false, // (kuna) angr opt-in default off (upstream byte-identical)
            switch_multi_pred: false, // (kuna) angr opt-in default off (upstream byte-identical)
            unrolled_guard: false, // (kuna) angr opt-in default off (upstream byte-identical)
            jumptable_share_partial: true, // (kuna) DIV: upstream stageJumpTable shape
            loader: None,
            // C++ Architecture default: readonlypropagate = false (resetDefaults);
            // `option readonly` flips it before the per-function build_arch_handle.
            readonlypropagate: false,
            // (kuna) No dynamic-relocation const slots until a real ELF load
            // installs them (see `Architecture::dynreloc_const`).
            dynreloc_const: Rc::new(Vec::new()),
            // Default volatile ops are non-functional (`getDisplay() != 0`), so the
            // volatile-read op's output is held; matches the unconfigured glb.
            volatile_display_functional: false,
            // C++ Architecture default: struct|array|pointer (resetDefaults).
            split_datatype_config: crate::options::split_datatype::OPTION_STRUCT
                | crate::options::split_datatype::OPTION_ARRAY
                | crate::options::split_datatype::OPTION_POINTER,
            global_query: None,
            remote_scope: None,
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
            // No tracked registers until build_arch_handle snapshots the context DB.
            tracked_sets: kuna_base::partmap::PartMap::default(),
        }
    }

    /// (kuna, Phase 3) The active fallback naming vocabulary (see
    /// [`KunaNameStyle`](crate::database::KunaNameStyle)): `Ghidra` wins over
    /// `Angr` wins over the upstream `Func` default.
    pub fn kuna_name_style(&self) -> crate::database::KunaNameStyle {
        if self.name_style_ghidra {
            crate::database::KunaNameStyle::Ghidra
        } else if self.name_style_angr {
            crate::database::KunaNameStyle::Angr
        } else {
            crate::database::KunaNameStyle::Func
        }
    }

    /// Tracked-register values valid at `addr` (C++ `glb->context->getTrackedSet`).
    /// Reads the [`tracked_sets`](Self::tracked_sets) snapshot taken at
    /// `build_arch_handle`.
    pub fn get_tracked_set(&self, addr: &Address) -> &kuna_sleigh::globalcontext::TrackedSet {
        self.tracked_sets.get_value(addr)
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

    /// (kuna `dynrelocs`) Whether `offset` lies in a dynamic-relocation slot that
    /// `PT_GNU_RELRO` freezes, so a read-only varnode there may be folded to its
    /// relocated value even with global read-only propagation off. Binary search
    /// over the sorted, disjoint [`Self::dynreloc_const`]; a plain `false` when
    /// that list is empty, which is every non-ELF path.
    pub fn dynreloc_const_contains(&self, offset: u64) -> bool {
        let r = &self.dynreloc_const;
        match r.binary_search_by(|(lo, _)| lo.cmp(&offset)) {
            Ok(_) => true,
            Err(0) => false,
            Err(i) => offset <= r[i - 1].1,
        }
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

    /// Get the floating-point format for the given size in bytes, or `None` if
    /// the processor has no format of that size (C++
    /// `glb->translate->getFloatFormat`).  Reads the [`floatformats`](Self::floatformats)
    /// table shared in `build_arch_handle`.
    pub fn get_float_format(&self, size: int4) -> Option<&kuna_num::float::FloatFormat> {
        self.floatformats.iter().find(|fmt| fmt.get_size() == size)
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
    /// `(space_index, offset)` — the ArchContext carries no `Rc<AddrSpace>` identities for
    /// the global scope, only the snapshotted indices.
    /// (kuna, Phase 3) The host-declared prototype MODEL of a locked callee
    /// signature (ghidra-mode `<prototype model=…>`), when the remote provider
    /// resolved one — the model `ActionDefaultParams` seeds the locked pieces
    /// under instead of `defaultfp`.  `None` on the standalone path (the CLI's
    /// `parse line extern` pieces are model-less), keeping it byte-identical.
    pub fn callee_proto_model(
        &self,
        addr: &Address,
    ) -> Option<std::rc::Rc<crate::fspec::ProtoModel>> {
        self.remote_scope.as_ref()?.callee_model_at(addr)
    }

    pub fn callee_proto_pieces(&self, addr: &Address) -> Option<crate::fspec::PrototypePieces> {
        if let Some(remote) = &self.remote_scope {
            // (kuna, Phase 3) ghidra-mode: the locked callee signatures decoded
            // from getMappedSymbols answers (query-through).
            return remote.callee_pieces_at(addr);
        }
        let space_index = addr.get_space()?.get_index();
        let offset = addr.get_offset();
        self.callee_protos
            .iter()
            .find(|(si, off, _)| *si == space_index && *off == offset)
            .map(|(_, _, pieces)| pieces.clone())
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
    /// `AddrSpaceManager`).  // STUB(W4)
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
        match self.effective_global_query(addr) {
            Some(gq) => gq.query_properties(addr, size, usepoint),
            None => 0,
        }
    }

    /// The global-scope read source for one address: the ghidra-mode
    /// [`RemoteScope`](crate::remote_provider::RemoteScope) (query-through +
    /// merged snapshot) when installed, else the frozen per-function
    /// [`GlobalQuery`] snapshot.  Every global read below resolves through
    /// this, so the remote seam is exactly the C++ `ScopeGhidra` overriding of
    /// the base `Scope` lookups.
    fn effective_global_query(&self, addr: &Address) -> Option<Rc<GlobalQuery>> {
        if let Some(remote) = &self.remote_scope {
            return Some(remote.query_snapshot(addr));
        }
        self.global_query.clone()
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
        self.effective_global_query(addr)
            .and_then(|gq| gq.name_for_varnode(addr, size, usepoint))
    }

    /// Like [`name_for_global_varnode`](Self::name_for_global_varnode) but also
    /// returns the owning Symbol's scope-name chain (innermost first, global
    /// excluded) for namespace-qualified rendering (`PrintC::pushSymbolScope`).
    pub fn name_for_global_varnode_scoped(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(String, int4, Option<std::rc::Rc<crate::dtype::Datatype>>, Vec<String>)> {
        self.effective_global_query(addr)
            .and_then(|gq| gq.name_for_varnode_scoped(addr, size, usepoint))
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
        self.effective_global_query(addr)
            .and_then(|gq| gq.sized_type_geometry(addr, size, usepoint))
    }

    /// Get the minimum laned-register size (C++
    /// `Architecture::getMinimumLanedRegisterSize`).  // STUB(W4)
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
    /// (C++ `AddrSpaceManager::getConstant`).  // STUB(W4)
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
        let gq = self.effective_global_query(addr)?;
        let e = gq.find_container_entry(addr, size, usepoint)?;
        let space = addr.get_space()?;
        Some(GlobalContainer {
            symbol_type: e.symbol_type.clone(),
            entry_addr: Address::new(Rc::clone(space), e.first),
            all_flags: e.all_flags,
            symbol_offset: e.symbol_offset,
            symbol_id: e.symbol_id,
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
        let gq = self.effective_global_query(entry)?;
        let space_index = entry.get_space()?.get_index();
        let start = entry.get_offset();
        for e in gq.entries_for_space(space_index) {
            if e.first != start {
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

    /// C++ `Scope::queryFunction(addr)` (`database.cc:1292`) restricted to the
    /// global-scope snapshot: resolve an address to the FunctionSymbol that
    /// *starts* there and report its display name, entry address, and recovered
    /// prototype.  A FunctionSymbol's storage is a `TYPE_CODE` Datatype whose
    /// `getPrototype()` is the callee `FuncProto` (the `Funcdata::getFuncProto()`
    /// the C++ `queryFunction` returns a `Funcdata *` for).  `None` when no global
    /// symbol table is shared (hand-built fixtures), no entry starts at `entry`, or
    /// the entry is not a code symbol with a prototype.
    ///
    /// This is the `ActionDeindirect::apply` lever: a CALLIND whose target is the
    /// constant `entry` deindirects to a direct CALL to this function.
    pub fn query_function(
        &self,
        entry: &Address,
    ) -> Option<(String, Address, std::rc::Rc<crate::fspec::FuncProto>)> {
        let gq = self.effective_global_query(entry)?;
        let space = entry.get_space()?;
        let space_index = space.get_index();
        let start = entry.get_offset();
        for e in gq.entries_for_space(space_index) {
            if e.first != start {
                continue;
            }
            // C++ `Scope::queryFunction(addr)` returns the `FunctionSymbol`'s
            // `Funcdata`, whose `getFuncProto()` is its recovered prototype — or,
            // for a bare loader symbol, the default (void, unlocked) proto.  A
            // FunctionSymbol carries a generic code data-type that usually holds NO
            // recovered `FuncProto`, so resolve on the function-ness of the symbol,
            // not on a code prototype being present.
            if !e.is_function {
                continue;
            }
            let entry_addr = Address::new(std::rc::Rc::clone(space), e.first);
            // Prefer a recovered code prototype when the type carries one (a
            // declared callee, e.g. `parse line extern void f(...)`); otherwise
            // hand back the default void proto the C++ funcsym's `getFuncProto`
            // would return for an undeclared loader symbol.
            if let Some(ct) = &e.symbol_type {
                if ct.get_metatype() == crate::dtype::type_metatype::TYPE_CODE {
                    if let Some(proto) = ct.get_code_prototype() {
                        // A declared callee already carries its model/params; but the
                        // parked inject id (call-fixup) lives on the FunctionSymbol,
                        // not the type — re-apply it so `deindirect` sees an inline
                        // (inject-bearing) callee even for a typed prototype.
                        if e.func_inject_id >= 0 && proto.get_inject_id() < 0 {
                            let mut p = crate::fspec::FuncProto::new();
                            p.copy(proto);
                            p.set_inject_id(e.func_inject_id);
                            if e.func_no_return && self.peimportcall {
                                p.set_no_return(true);
                            }
                            return Some((e.symbol_name.clone(), entry_addr, std::rc::Rc::new(p)));
                        }
                        if e.func_no_return && self.peimportcall && !proto.is_no_return() {
                            let mut p = crate::fspec::FuncProto::new();
                            p.copy(proto);
                            p.set_no_return(true);
                            return Some((e.symbol_name.clone(), entry_addr, std::rc::Rc::new(p)));
                        }
                        return Some((e.symbol_name.clone(), entry_addr, std::rc::Rc::clone(proto)));
                    }
                }
            }
            // Default (void, unlocked) proto on the default model — the shape the
            // C++ funcsym's `getFuncProto` carries after `setInternal(defaultfp,
            // void)`.  A model IS required: the deindirect proto-merge compares
            // models (`FuncProto::is_compatible` dereferences `op2->model()`).  If
            // the architecture has not shared its default model / type factory
            // (hand-built fixtures), fall back to a bare proto.
            let mut proto = crate::fspec::FuncProto::new();
            if let (Some(model), Some(types)) = (self.default_fp(), self.types()) {
                if let Ok(void) = types.get_type_void() {
                    proto.set_internal(std::rc::Rc::clone(model), void);
                }
            }
            // C++ `IfcFixupApply`'s `setInjectId` also marks the funcsym proto
            // inline; carry that so `deindirect`'s `isInline()` test triggers the
            // restart that re-flows and injects the call-fixup at the now-direct
            // call site.
            if e.func_inject_id >= 0 {
                proto.set_inject_id(e.func_inject_id);
            }
            // (kuna) The callee's no-return flow effect, gated by `peimportcall`:
            // `deindirect` skips its prototype merge for a no-return callee and
            // schedules the restart whose re-flow truncates at the call.
            if e.func_no_return && self.peimportcall {
                proto.set_no_return(true);
            }
            return Some((e.symbol_name.clone(), entry_addr, std::rc::Rc::new(proto)));
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
    /// `entry->getSymbol()->getId()` — see [`GlobalEntry::symbol_id`].
    pub symbol_id: u64,
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
/// STUB(W4): the symbol scope machinery (`decompiler/cpp/database.{hh,cc}`,
/// `varmap.{hh,cc}`) is a W4 subsystem.  `Funcdata` holds an `Option<Scope>`
/// where the C++ holds a `ScopeLocal *`; the W3 IR data-model never reads symbol
/// state, so the placeholder is empty.  The varnode-property look-ups
/// (`localmap->queryProperties`) that `setVarnodeProperties`/`newVarnode*` make
/// resolve to "no entry, no extra flags" until W4 fills this in.
#[derive(Debug, Clone, Default)]
pub struct Scope;

/// The recovered prototype of a function (C++ `FuncProto`, `Funcdata::funcp`).
///
/// STUB(W4): the prototype model subsystem (`decompiler/cpp/fspec.{hh,cc}`) is
/// W4.  `Funcdata` holds a `FuncProto` placeholder so the struct layout and the
/// `getFuncProto` accessor exist; the W3 IR construction never queries the
/// prototype.
#[derive(Debug, Clone, Default)]
pub struct FuncProto;

impl FuncProto {
    /// Is the output (return value) storage locked? (C++ `FuncProto::isOutputLocked`).
    ///
    /// STUB(W6): the proto-recovery passes are stubs and never lock the
    /// output, so this reports the un-recovered default (`false`).  `ActionDeadCode::
    /// gatherConsumedReturn` reads it to decide whether the return value is fully
    /// consumed; with no locked proto it falls through to the NZ-mask scan.
    pub fn is_output_locked(&self) -> bool {
        false
    }

    /// Number of bytes of the return value that are consumed, or 0 if unknown
    /// (C++ `FuncProto::getReturnBytesConsumed`).
    ///
    /// STUB(W6): no recovered proto, so 0 ("no restriction") — the faithful
    /// un-recovered default.
    pub fn get_return_bytes_consumed(&self) -> i32 {
        0
    }
}

// ===========================================================================
// DatabaseArch / TranslateAccess / TypeFactoryAccess on the ArchContext.
// ===========================================================================
//
// STUB(W4/W5/W6) closure: `database.rs` declares these traits (the slice of
// `glb` the symbol database needs) and the in-crate `TestArch` implements them
// for unit tests.  There was no NON-test impl, so the ghidra-style naming in
// `Database::build_variable_name` / `build_default_name` was unreachable in the
// live pipeline (`ActionNameVars` hardcoded `format!("v{base}")`).  Implementing
// the traits here on the `ArchContext` wires the live engine to that
// renderer so `option namestyle ghidra` produces `iVarN` locals.

impl crate::database::TranslateAccess for ArchContext {
    /// C++ `Translate::getRegisterName(spc,off,sz)` — forward to the engine's
    /// register lookup installed on the shared `AddrSpaceManager`.
    fn get_register_name(&self, space: &Rc<AddrSpace>, off: u64, size: int4) -> String {
        match self.manage.register_lookup() {
            Some(rl) => rl.get_register_name(space, off, size),
            None => String::new(),
        }
    }
}

impl crate::database::TypeFactoryAccess for ArchContext {
    /// C++ `TypeFactory::getBase(size,metatype)` — forward to the shared
    /// `TypeFactoryImpl`, falling back to a bare placeholder when no factory is
    /// shared (hand-built fixtures) or the lookup errs.
    fn get_base(
        &self,
        size: int4,
        meta: crate::dtype::type_metatype,
    ) -> Rc<crate::dtype::Datatype> {
        use crate::dtype::TypeFactory;
        match self.types.as_deref() {
            Some(tf) => tf
                .get_base(size, meta)
                .unwrap_or_else(|_| Rc::new(crate::dtype::Datatype::new(size, meta))),
            None => Rc::new(crate::dtype::Datatype::new(size, meta)),
        }
    }

    /// C++ `TypeFactory::getTypeCode()` — the "code" placeholder for function
    /// symbols, forwarded to the shared factory with a placeholder fallback.
    fn get_type_code(&self) -> Rc<crate::dtype::Datatype> {
        use crate::dtype::TypeFactory;
        match self.types.as_deref() {
            Some(tf) => tf.get_type_code().unwrap_or_else(|_| {
                Rc::new(crate::dtype::Datatype::new(1, crate::dtype::type_metatype::TYPE_CODE))
            }),
            None => Rc::new(crate::dtype::Datatype::new(1, crate::dtype::type_metatype::TYPE_CODE)),
        }
    }
}

impl crate::database::DatabaseArch for ArchContext {
    fn num_spaces(&self) -> int4 {
        self.manage.num_spaces()
    }
    fn types(&self) -> &dyn crate::database::TypeFactoryAccess {
        self
    }
    fn translate(&self) -> &dyn crate::database::TranslateAccess {
        self
    }
    fn min_funcsymbol_size(&self) -> int4 {
        // The ArchContext carries no `min_funcsymbol_size`; the symbol
        // database's only use of it is `FunctionSymbol` mapping (not the naming
        // path), and the engine default is 1.
        1
    }
    fn name_style_angr(&self) -> bool {
        self.name_style_angr
    }
    /// C++ `Datatype::printNameBase` (`type.hh:286`): the first char of the type
    /// name (the `iVar`/`uVar`/`fVar`/`pVar` prefix), empty for a nameless type.
    fn type_name_base(&self, dt: &crate::dtype::Datatype) -> String {
        match dt.get_name().chars().next() {
            Some(c) => c.to_string(),
            None => String::new(),
        }
    }
}

/// Shared handle to the [`Architecture`] (C++ `Funcdata::glb`, a borrowed
/// `Architecture *`).
///
/// STUB(W4): the C++ `glb` is a non-owning back-pointer to the long-lived
/// `Architecture`.  Modeled as `Rc<Architecture>` so multiple `Funcdata`
/// snapshots (ADR 0007) can share it; the W3 code only reads through it.
pub type ArchHandle = Rc<ArchContext>;
