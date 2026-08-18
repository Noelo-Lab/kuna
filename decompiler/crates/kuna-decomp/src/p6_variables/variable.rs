//! Port of `decompiler/cpp/variable.{hh,cc}` (W7, item `w7-s6-variable-cover`) —
//! the high-level variable model: [`HighVariable`] (a grouping of `Varnode`s
//! that hold one source-level variable across the function), its
//! flags/type/name/cover dirty-and-rederive machinery, the overlap model
//! ([`VariableGroup`]/[`VariablePiece`]), and the Cover-intersection cache
//! ([`HighEdge`]/[`HighIntersectTest`]).
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `HighVariable` is a heap object holding `vector<Varnode *> inst` and
//! reverse-referenced by each member `vn->high`.  Per ADR 0001 HighVariables
//! live in a [`HighVariableBank`] slotmap-style arena keyed by
//! [`HighVariableId`] (the arena key); `inst` becomes `Vec<VarnodeId>` and the
//! back-link is the `Varnode::high` `Option<HighVariableId>` already wired in
//! `varnode.rs`.  Every HighVariable method that reads member-Varnode state
//! (`getFlags`/`getType`/`getCover`/`isTypeLock`/…) takes a [`HighContext`]
//! (implemented by `Funcdata`) since the members live in a different arena.
//!
//! `VariableGroup`/`VariablePiece` (the overlapping-pieces model) similarly live
//! in arenas keyed by [`VariableGroupId`]/[`VariablePieceId`]; the C++ raw
//! `new`/`delete` of these become arena insert/remove, and the
//! `pieceSet`/`group`/`high`/`intersection` cross-pointers become ids.
//!
//! ## ADR 0002 (ordered containers) realization
//!
//! `VariableGroup::pieceSet` is a `BTreeSet<VariablePieceId>` ordered by
//! [`PieceCompareByOffset`] (offset then size) — transcribed from
//! `variable.cc:33-39`.  `HighIntersectTest::highedgemap` is a
//! `BTreeMap<HighEdge,bool>`; the C++ `HighEdge::operator<` orders by the raw
//! `HighVariable *` pointer pair, modelled here by the [`HighVariableId`] `Ord`
//! (ADR 0002 sanctions transcribing pointer-keyed maps to id-keyed BTreeMaps;
//! both `(a,b)` and `(b,a)` are always inserted/purged together, so the cache
//! semantics are order-independent under any consistent total order).
//!
//! ## Boundaries (W4 Symbol link, W7 merge)
//!
//! The current `Varnode` carries no `SymbolEntry` back-link (the W4 symbol-merge
//! wiring is not yet on the Varnode), and `Varnode::copyShadow`/`partialCopyShadow`
//! (`varnode.cc`, outside this item's ownership sliver) are not yet ported.  The
//! Symbol-resolution methods (`setSymbol`/`updateSymbol`/`getSymbolEntry`/
//! `finalizeDatatype`) and the copy-shadow refinement inside
//! `HighIntersectTest::testBlockIntersection` therefore route through
//! [`HighContext`] callbacks that the W4-symbol / merge waves fill; until then
//! `vn_symbol_entry` returns `None` and `copy_shadow` returns `false` (see
//! `docs/rust-port/losses.md`).

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::types::{int2, int4, uint4};

use crate::cover::{Cover, PcodeOpSet};
use crate::dtype::{type_metatype, Datatype};
use crate::context::{HighVariableId, VarnodeId};

// =============================================================================
// Wire marshaling ids owned by variable.cc (upstream numbers, variable.cc:23-27;
// DECOMPILER scope — written by numeric id, never registered on the SLEIGH
// registry; see the note in `substrate/funcdata_encode.rs`).
// =============================================================================

/// Marshaling attribute "class" (C++ `ATTRIB_CLASS`, variable.cc:23, id 66).
pub const ATTRIB_CLASS: kuna_base::marshal::AttributeId =
    kuna_base::marshal::AttributeId::new("class", 66);
/// Marshaling attribute "repref" (C++ `ATTRIB_REPREF`, variable.cc:24, id 67).
pub const ATTRIB_REPREF: kuna_base::marshal::AttributeId =
    kuna_base::marshal::AttributeId::new("repref", 67);
/// Marshaling attribute "symref" (C++ `ATTRIB_SYMREF`, variable.cc:25, id 68).
pub const ATTRIB_SYMREF: kuna_base::marshal::AttributeId =
    kuna_base::marshal::AttributeId::new("symref", 68);
/// Marshaling element `<high>` (C++ `ELEM_HIGH`, variable.cc:27, id 82).
pub const ELEM_HIGH: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("high", 82);

// =============================================================================
// Arena keys for the VariableGroup / VariablePiece overlap model
// =============================================================================

/// Arena key for a [`VariableGroup`] (C++ `VariableGroup *`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableGroupId(pub u32);

/// Arena key for a [`VariablePiece`] (C++ `VariablePiece *`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariablePieceId(pub u32);

// =============================================================================
// HighVariable dirtiness flags (variable.hh:119-131)
// =============================================================================

/// Dirtiness flags for a [`HighVariable`] (C++ anonymous `enum` in
/// `class HighVariable`, `variable.hh:119-131`).
pub mod high_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Boolean properties for the HighVariable are dirty
    pub const flagsdirty: uint4 = 1;
    /// The name representative for the HighVariable is dirty
    pub const namerepdirty: uint4 = 2;
    /// The data-type for the HighVariable is dirty
    pub const typedirty: uint4 = 4;
    /// The cover for the HighVariable is dirty
    pub const coverdirty: uint4 = 8;
    /// The symbol attachment is dirty
    pub const symboldirty: uint4 = 0x10;
    /// There exists at least 1 COPY into \b this HighVariable
    pub const copy_in1: uint4 = 0x20;
    /// There exists at least 2 COPYs into \b this HighVariable
    pub const copy_in2: uint4 = 0x40;
    /// Set if a final data-type is locked in and dirtying is disabled
    pub const type_finalized: uint4 = 0x80;
    /// Set if part of a multi-entry Symbol but did not get merged
    pub const unmerged: uint4 = 0x100;
    /// Set if intersections with other HighVariables need recompute
    pub const intersectdirty: uint4 = 0x200;
    /// Set if extended cover needs to be recomputed
    pub const extendcoverdirty: uint4 = 0x400;
}

// Mirror the few Varnode flag bits HighVariable reads (variable.hh inline `is*`).
use crate::varnode::varnode_flags;

// =============================================================================
// HighContext — cross-arena reads HighVariable needs from member Varnodes
// =============================================================================

/// A minimal read-only view of a member Varnode's state, as the C++
/// `HighVariable` reads off each `Varnode *`.
#[derive(Debug, Clone)]
pub struct VarnodeView {
    /// `vn->getFlags()`
    pub flags: uint4,
    /// `vn->getSize()`
    pub size: int4,
    /// `vn->getType()`
    pub type_: Rc<Datatype>,
    /// `vn->isTypeLock()`
    pub type_lock: bool,
    /// `vn->getMergeGroup()`
    pub merge_group: int2,
    /// `vn->isWritten()`
    pub written: bool,
    /// `vn->getDef()->getTime()` when written (the `compareName` earliest tiebreak)
    pub def_time: uint4,
    /// `vn->getSpace()->getType() == IPTR_INTERNAL`
    pub space_internal: bool,
    /// `vn->getCreateIndex()` (the `encode` repref; also stable identity)
    pub create_index: uint4,
}

/// Cross-arena access for [`HighVariable`] methods (the C++ reads these off the
/// held `Varnode *`/`Symbol *`/`TypeFactory *`).  Implemented by `Funcdata`.
pub trait HighContext {
    /// Read the [`VarnodeView`] of a member Varnode (`vn->get*`).
    fn vn_view(&self, vn: VarnodeId) -> VarnodeView;
    /// `vn->getCover()` — the member's Cover (already rebuilt by the Merge
    /// driver), or `None` if it has none.
    fn vn_cover(&self, vn: VarnodeId) -> Option<Cover>;
    /// `vn->hasCover()`.
    fn vn_has_cover(&self, vn: VarnodeId) -> bool;
    /// Read the [`CompareNameView`] of a member Varnode (the fields
    /// `HighVariable::compareName` reads).
    fn vn_name_view(&self, vn: VarnodeId) -> CompareNameView;
    /// Read the [`VarnodeViewLoc`] of a member Varnode (`vn->getAddr()`), for the
    /// `compareJustLoc` membership ordering.
    fn vn_loc_view(&self, vn: VarnodeId) -> VarnodeViewLoc;
}

// =============================================================================
// VariableGroup (variable.hh:44-65)
// =============================================================================

/// A collection of [`HighVariable`] pieces that mutually overlap (C++ `class
/// VariableGroup`, `variable.hh:44-65`).
#[derive(Debug, Clone)]
pub struct VariableGroup {
    /// The set of [`VariablePiece`]s making up \b this group, ordered by
    /// [`PieceCompareByOffset`] (C++ `pieceSet`).  The ordering key (offset,size)
    /// is cached here so the `BTreeSet` need not reach the piece arena.
    piece_set: BTreeSet<PieceKey>,
    /// Number of contiguous bytes covered by the whole group (C++ `size`).
    size: int4,
    /// Byte offset of \b this group within its containing Symbol (C++
    /// `symbolOffset`).
    symbol_offset: int4,
}

/// Ordering key for [`VariableGroup::piece_set`] (the C++
/// `PieceCompareByOffset`: offset then size, `variable.cc:33-39`).  Carries the
/// piece id as the final identity so distinct equal-offset/equal-size pieces
/// remain distinct set members (matching the C++ pointer-set membership).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PieceKey {
    offset: int4,
    size: int4,
    id: VariablePieceId,
}

impl PartialOrd for PieceKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PieceKey {
    /// C++ `PieceCompareByOffset::operator()`: offset, then size.  A final id
    /// tiebreak preserves distinctness of equal (offset,size) pieces (the C++
    /// `set<VariablePiece *>` distinguishes them by pointer; `find` of a piece
    /// uses the *same* key, so `mergeGroups`' `pieceSet.find(piece)` matches on
    /// (offset,size) — modelled by [`VariableGroup::find_by_offset_size`]).
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.offset != other.offset {
            return self.offset.cmp(&other.offset);
        }
        if self.size != other.size {
            return self.size.cmp(&other.size);
        }
        self.id.cmp(&other.id)
    }
}

impl Default for VariableGroup {
    fn default() -> Self {
        VariableGroup::new()
    }
}

impl VariableGroup {
    /// C++ `VariableGroup()`.
    pub fn new() -> VariableGroup {
        VariableGroup { piece_set: BTreeSet::new(), size: 0, symbol_offset: 0 }
    }

    /// Return \b true if \b this group has no pieces (C++ `empty`).
    pub fn empty(&self) -> bool {
        self.piece_set.is_empty()
    }

    /// Get the number of bytes \b this group covers (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }

    /// Cache the symbol offset for the group (C++ `setSymbolOffset`).
    pub fn set_symbol_offset(&mut self, val: int4) {
        self.symbol_offset = val;
    }

    /// Get offset of \b this group within its Symbol (C++ `getSymbolOffset`).
    pub fn get_symbol_offset(&self) -> int4 {
        self.symbol_offset
    }

    /// Find a piece in the set by its (offset,size) key (the C++
    /// `pieceSet.find(piece)` in `mergeGroups`, which keys on offset/size).
    fn find_by_offset_size(&self, offset: int4, size: int4) -> Option<VariablePieceId> {
        // Range over all pieces with this (offset,size); the C++ set treats
        // matching (offset,size) as equal keys (operator< returns false both
        // ways), so any one match is "the" duplicate.
        self.piece_set
            .iter()
            .find(|k| k.offset == offset && k.size == size)
            .map(|k| k.id)
    }
}

// =============================================================================
// VariablePiece (variable.hh:71-96)
// =============================================================================

/// Information about how a [`HighVariable`] fits into a larger overlap group
/// (C++ `class VariablePiece`, `variable.hh:71-96`).
#[derive(Debug, Clone)]
pub struct VariablePiece {
    /// Group to which \b this piece belongs (C++ `group`).
    group: VariableGroupId,
    /// HighVariable owning \b this piece (C++ `high`).
    high: HighVariableId,
    /// Byte offset of \b this piece within the group (C++ `groupOffset`).
    group_offset: int4,
    /// Number of bytes in \b this piece (C++ `size`).
    size: int4,
    /// Pieces \b this intersects with (C++ `intersection`).
    intersection: Vec<VariablePieceId>,
    /// Extended cover for the piece (C++ `cover`).
    cover: Cover,
}

impl VariablePiece {
    /// Get the HighVariable associated with \b this piece (C++ `getHigh`).
    pub fn get_high(&self) -> HighVariableId {
        self.high
    }
    /// Get the central group (C++ `getGroup`).
    pub fn get_group(&self) -> VariableGroupId {
        self.group
    }
    /// Get the offset of \b this within its group (C++ `getOffset`).
    pub fn get_offset(&self) -> int4 {
        self.group_offset
    }
    /// Return the number of bytes in \b this piece (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get the cover associated with \b this piece (C++ `getCover`).
    pub fn get_cover(&self) -> &Cover {
        &self.cover
    }
    /// Get number of pieces \b this intersects with (C++ `numIntersection`).
    pub fn num_intersection(&self) -> int4 {
        self.intersection.len() as int4
    }
    /// Get i-th piece \b this intersects with (C++ `getIntersection`).
    pub fn get_intersection(&self, i: int4) -> VariablePieceId {
        self.intersection[i as usize]
    }
}

// =============================================================================
// HighVariable (variable.hh:112-233)
// =============================================================================

/// A high-level variable modeled as a list of low-level Varnodes (C++ `class
/// HighVariable`, `variable.hh:112-233`).
#[derive(Debug, Clone)]
pub struct HighVariable {
    /// The member Varnode objects making up \b this (C++ `inst`).
    inst: Vec<VarnodeId>,
    /// Number of speculative merge classes (C++ `numMergeClasses`).
    num_merge_classes: int4,
    /// Dirtiness flags (C++ `highflags`).
    highflags: uint4,
    /// Boolean properties inherited from members (C++ `flags`).
    flags: uint4,
    /// The data-type for \b this (C++ `type`; `None` == null).
    type_: Option<Rc<Datatype>>,
    /// Storage location used to generate a Symbol name (C++ `nameRepresentative`).
    name_representative: Option<VarnodeId>,
    /// The internal cover (C++ `internalCover`).
    internal_cover: Cover,
    /// Additional overlap info, if any (C++ `piece`).
    piece: Option<VariablePieceId>,
    /// The Symbol \b this is tied to (C++ `symbol`; arena id, `None` == null).
    symbol: Option<crate::database::SymbolId>,
    /// -1 = perfect symbol match, >= 0 = offset (C++ `symboloffset`).
    symbol_offset: int4,
    /// (kuna) The name bound to this HighVariable by `ActionNameVars`, when the
    /// W4 ScopeLocal/`Symbol` layer is absent.  In C++ the name lives on the
    /// `Symbol` in the local scope (`Scope::buildDefaultName` -> `renameSymbol`);
    /// the merged tree has no ScopeLocal, so `ActionNameVars` binds the angr
    /// default name (`vN`) directly here and the printer reads it as the
    /// `getSymbol()->getDisplayName()` stand-in.  `None` == unnamed.
    kuna_name: Option<String>,
    /// (kuna) The data-type of the mapped Symbol this high renders through (the
    /// `getSymbol()->getType()` stand-in, used by the printer to render an array
    /// or struct member access `name[idx]`).  `None` for an unnamed/scalar high.
    kuna_symbol_type: Option<Rc<Datatype>>,
    /// (kuna) The local-scope EquateSymbol bound to this high by
    /// `Funcdata::build_dynamic_symbol` (C++ `vn->setSymbolEntry(sym->...)` whose
    /// effect is `high->getSymbol() == sym`).  The W4 `Symbol`/`SymbolEntry`
    /// varnode-link is not in the merged tree, so the `force varnode` console
    /// command parks the equate-Symbol id here and the printer reads its
    /// `getDisplayFormat()` — the `vn->getHigh()->getSymbol()->getDisplayFormat()`
    /// path of `PrintC::push_integer` (printc.cc:1370-1376).  `None` == no equate.
    kuna_equate_symbol: Option<crate::database::SymbolId>,
    /// (kuna) The name bound to this high names a Symbol in the GLOBAL scope (set
    /// when `ActionNameVars::linkSpacebaseSymbol` resolves the `&symbol` reference
    /// through `glb->getGlobalScope()` rather than `fd->getScopeLocal()`).  C++
    /// `PrintC::emitLocalVarDecls` declares only the LOCAL scope
    /// (`emitScopeVarDecls(fd->getScopeLocal(), ...)`, printc.cc:2336/2667), so a
    /// global Symbol is rendered by name in the body but never carries a local
    /// declaration.  `false` == a local-scope (or unscoped) high (declarable).
    kuna_global: bool,
}

impl HighVariable {
    /// Construct a HighVariable with a single member Varnode (C++
    /// `HighVariable(Varnode *vn)`, `variable.cc:220-235`).
    ///
    /// The C++ constructor also calls `vn->setHigh(this, numMergeClasses-1)` and,
    /// if the member has a SymbolEntry, `setSymbol(vn)`.  Those cross-arena steps
    /// are done by the [`HighVariableBank::new_high`] factory, which has the id.
    fn new(vn: VarnodeId) -> HighVariable {
        HighVariable {
            inst: vec![vn],
            num_merge_classes: 1,
            highflags: high_flags::flagsdirty
                | high_flags::namerepdirty
                | high_flags::typedirty
                | high_flags::coverdirty,
            flags: 0,
            type_: None,
            name_representative: None,
            internal_cover: Cover::new(),
            piece: None,
            symbol: None,
            symbol_offset: -1,
            kuna_name: None,
            kuna_symbol_type: None,
            kuna_equate_symbol: None,
            kuna_global: false,
        }
    }

    /// (kuna) Get the name bound by `ActionNameVars` (the ScopeLocal/`Symbol`
    /// name stand-in), or `None` if unnamed.
    pub fn kuna_name(&self) -> Option<&str> {
        self.kuna_name.as_deref()
    }

    /// (kuna) Bind the angr default name to this HighVariable.
    pub fn set_kuna_name(&mut self, name: impl Into<String>) {
        self.kuna_name = Some(name.into());
    }

    /// (kuna) Set the in-symbol byte offset for an array/struct member access
    /// (the printer renders `name[idx]` when this is > 0 and the symbol type is
    /// an array).  The C++ `HighVariable::symboloffset` (`variable.cc`).
    pub fn set_symbol_offset(&mut self, off: int4) {
        self.symbol_offset = off;
    }

    /// (kuna) The in-symbol byte offset bound by `name_for_varnode` (or -1).
    pub fn kuna_symbol_offset(&self) -> int4 {
        self.symbol_offset
    }

    /// (kuna LOSS-229) Bind the dynamic-mapping Symbol id to this high (C++
    /// `HighVariable::setSymbol` via `vn->setSymbolEntry`).  Read by
    /// `Merge::mergeTestRequired`'s symbol guard (merge.cc:157-164) so a dynamic
    /// temp does not get merged back into the storage it copies.
    pub fn set_kuna_dynamic_symbol(&mut self, sym: crate::database::SymbolId) {
        self.symbol = Some(sym);
    }

    /// (kuna LOSS-229) The dynamic-mapping Symbol id bound to this high, or `None`.
    pub fn kuna_dynamic_symbol(&self) -> Option<crate::database::SymbolId> {
        self.symbol
    }

    /// (kuna) Bind the mapped Symbol's data-type (for array/struct rendering).
    pub fn set_symbol_type(&mut self, ct: Rc<Datatype>) {
        self.kuna_symbol_type = Some(ct);
    }

    /// (kuna) The mapped Symbol's data-type, or `None`.
    pub fn kuna_symbol_type(&self) -> Option<&Rc<Datatype>> {
        self.kuna_symbol_type.as_ref()
    }

    /// (kuna) Mark that this high's name resolves to a GLOBAL-scope Symbol (the
    /// `linkSpacebaseSymbol` global-scope fallback).  See [`Self::kuna_global`].
    pub fn set_kuna_global(&mut self, global: bool) {
        self.kuna_global = global;
    }

    /// (kuna) Whether this high names a GLOBAL-scope Symbol (rendered by name but
    /// never declared in the function body — C++ `emitScopeVarDecls` walks only the
    /// local scope, printc.cc:2667).
    pub fn kuna_global(&self) -> bool {
        self.kuna_global
    }

    /// (kuna) Bind the local-scope EquateSymbol to this high (the merged-tree
    /// stand-in for `vn->setSymbolEntry` whose effect C++ exposes as
    /// `high->getSymbol()`).  Set by `Funcdata::build_dynamic_symbol`.
    pub fn set_kuna_equate_symbol(&mut self, sym: crate::database::SymbolId) {
        self.kuna_equate_symbol = Some(sym);
    }

    /// (kuna) The equate-Symbol id bound to this high, or `None` (the
    /// `high->getSymbol()` stand-in for the constant-format console path).
    pub fn kuna_equate_symbol(&self) -> Option<crate::database::SymbolId> {
        self.kuna_equate_symbol
    }

    // --- Dirty-flag setters (variable.hh:165-169, inline) -----------------

    /// Mark the boolean properties as \e dirty (C++ `flagsDirty`).
    pub fn flags_dirty(&mut self) {
        self.highflags |= high_flags::flagsdirty | high_flags::namerepdirty;
    }
    /// Mark the data-type as \e dirty (C++ `typeDirty`).
    pub fn type_dirty(&mut self) {
        self.highflags |= high_flags::typedirty;
    }
    /// Mark the symbol as \e dirty (C++ `symbolDirty`).
    pub fn symbol_dirty(&mut self) {
        self.highflags |= high_flags::symboldirty;
    }
    /// Mark \b this as having merge problems (C++ `setUnmerged`).
    pub fn set_unmerged(&mut self) {
        self.highflags |= high_flags::unmerged;
    }

    /// Mark the cover as \e dirty (C++ inline `coverDirty`, `variable.hh:276-282`).
    ///
    /// If \b this is a piece of a VariableGroup, the extended-cover dirtying of
    /// the intersecting pieces is done by [`HighVariableBank::cover_dirty`]
    /// (it needs the piece/group arenas); the internal-cover bit is set here.
    pub fn cover_dirty(&mut self) {
        self.highflags |= high_flags::coverdirty;
    }

    /// Is the cover returned by `getCover()` up-to-date (C++ inline
    /// `isCoverDirty`, `variable.hh:286-290`).
    pub fn is_cover_dirty(&self) -> bool {
        (self.highflags & (high_flags::coverdirty | high_flags::extendcoverdirty)) != 0
    }

    // --- COPY-in marks (variable.hh:153-157) -------------------------------

    /// Mark the existence of one COPY into \b this (C++ `setCopyIn1`).
    pub fn set_copy_in1(&mut self) {
        self.highflags |= high_flags::copy_in1;
    }
    /// Mark the existence of two COPYs into \b this (C++ `setCopyIn2`).
    pub fn set_copy_in2(&mut self) {
        self.highflags |= high_flags::copy_in2;
    }
    /// Clear marks indicating COPYs into \b this (C++ `clearCopyIns`).
    pub fn clear_copy_ins(&mut self) {
        self.highflags &= !(high_flags::copy_in1 | high_flags::copy_in2);
    }
    /// Is there at least one COPY into \b this (C++ `hasCopyIn1`).
    pub fn has_copy_in1(&self) -> bool {
        (self.highflags & high_flags::copy_in1) != 0
    }
    /// Is there at least two COPYs into \b this (C++ `hasCopyIn2`).
    pub fn has_copy_in2(&self) -> bool {
        (self.highflags & high_flags::copy_in2) != 0
    }

    // --- Simple accessors --------------------------------------------------

    /// Get the number of member Varnodes \b this has (C++ `numInstances`).
    pub fn num_instances(&self) -> int4 {
        self.inst.len() as int4
    }
    /// Get the i-th member Varnode (C++ `getInstance`).
    pub fn get_instance(&self, i: int4) -> VarnodeId {
        self.inst[i as usize]
    }
    /// Get the number of speculative merges for \b this (C++ `getNumMergeClasses`).
    pub fn get_num_merge_classes(&self) -> int4 {
        self.num_merge_classes
    }
    /// Get the Symbol offset associated with \b this (C++ `getSymbolOffset`).
    pub fn get_symbol_offset(&self) -> int4 {
        self.symbol_offset
    }
    /// Return \b true if \b this has no member Varnode (C++ `isUnattached`).
    pub fn is_unattached(&self) -> bool {
        self.inst.is_empty()
    }
    /// Return \b true if \b this has merge problems (C++ `isUnmerged`).
    pub fn is_unmerged(&self) -> bool {
        (self.highflags & high_flags::unmerged) != 0
    }
    /// Set the mark on this variable (C++ `setMark`).
    pub fn set_mark(&mut self) {
        self.flags |= varnode_flags::mark;
    }
    /// Clear the mark on this variable (C++ `clearMark`).
    pub fn clear_mark(&mut self) {
        self.flags &= !varnode_flags::mark;
    }
    /// Return \b true if \b this is marked (C++ `isMark`).
    pub fn is_mark(&self) -> bool {
        (self.flags & varnode_flags::mark) != 0
    }

    // --- updateFlags / flag predicates (variable.cc:352-368) ---------------

    /// (Re)derive boolean properties of \b this from the member Varnodes (C++
    /// `HighVariable::updateFlags`, `variable.cc:352-368`).
    fn update_flags(&mut self, ctx: &dyn HighContext) {
        if (self.highflags & high_flags::flagsdirty) == 0 {
            return; // flags are up to date
        }
        let mut fl: uint4 = 0;
        for &vn in &self.inst {
            fl |= ctx.vn_view(vn).flags;
        }
        // Keep these flags
        self.flags &= varnode_flags::mark | varnode_flags::typelock;
        // Update all but these
        self.flags |=
            fl & !(varnode_flags::mark | varnode_flags::directwrite | varnode_flags::typelock);
        self.highflags &= !high_flags::flagsdirty; // Clear the dirty flag
    }

    /// C++ `isMapped` (calls `updateFlags`).
    pub fn is_mapped(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::mapped) != 0
    }
    /// C++ `isPersist`.
    pub fn is_persist(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::persist) != 0
    }
    /// C++ `isAddrTied`.
    pub fn is_addr_tied(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::addrtied) != 0
    }
    /// C++ `isInput`.
    pub fn is_input(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::input) != 0
    }
    /// C++ `isImplied`.
    pub fn is_implied(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::implied) != 0
    }
    /// C++ `isSpacebase`.
    pub fn is_spacebase(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::spacebase) != 0
    }
    /// C++ `isConstant`.
    pub fn is_constant(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::constant) != 0
    }
    /// C++ `isUnaffected`.
    pub fn is_unaffected(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::unaffected) != 0
    }
    /// C++ `isExtraOut`.
    pub fn is_extra_out(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & (varnode_flags::indirect_creation | varnode_flags::addrtied))
            == varnode_flags::indirect_creation
    }
    /// C++ `isProtoPartial`.
    pub fn is_proto_partial(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::proto_partial) != 0
    }
    /// C++ `hasCover` (`variable.hh:218-220`).
    pub fn has_cover(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & (varnode_flags::constant | varnode_flags::annotation | varnode_flags::insert))
            == varnode_flags::insert
    }
    /// C++ `isNameLock`.
    pub fn is_name_lock(&mut self, ctx: &dyn HighContext) -> bool {
        self.update_flags(ctx);
        (self.flags & varnode_flags::namelock) != 0
    }

    // --- Type resolution (variable.cc:302-416) -----------------------------

    /// Take the stripped form of the current data-type (C++
    /// `HighVariable::stripType`, `variable.cc:302-320`).
    ///
    /// `ctx` supplies the lone member's constant-ness for the enum case;
    /// `symbol_submeta` supplies the backing-symbol metatype for the partial
    /// case (`None` if no backing symbol).
    fn strip_type(&mut self, ctx: &dyn HighContext, symbol_submeta: Option<type_metatype>) {
        let type_ = self.type_.clone().expect("stripType: null type (internal invariant)");
        if !type_.has_stripped() {
            return;
        }
        let meta = type_.get_metatype();
        if meta == type_metatype::TYPE_PARTIALUNION || meta == type_metatype::TYPE_PARTIALSTRUCT {
            if self.symbol.is_some() && self.symbol_offset != -1 {
                if let Some(submeta) = symbol_submeta {
                    if submeta == type_metatype::TYPE_STRUCT || submeta == type_metatype::TYPE_UNION
                    {
                        return; // Don't strip the partial union
                    }
                }
            }
        } else if type_.is_enum_type() {
            // Only preserve partial enum on a constant
            if self.inst.len() == 1 && ctx.vn_view(self.inst[0]).flags & varnode_flags::constant != 0
            {
                return;
            }
        }
        if let Some(stripped) = type_.get_stripped() {
            self.type_ = Some(stripped);
        }
    }

    /// Find the member Varnode with the most specialized data-type (C++
    /// `HighVariable::getTypeRepresentative`, `variable.cc:377-396`).
    pub fn get_type_representative(&self, ctx: &dyn HighContext) -> VarnodeId {
        let mut rep = self.inst[0];
        let mut rep_view = ctx.vn_view(rep);
        for &vn in self.inst.iter().skip(1) {
            let vn_view = ctx.vn_view(vn);
            if rep_view.type_lock != vn_view.type_lock {
                if vn_view.type_lock {
                    rep = vn;
                    rep_view = vn_view;
                }
            } else if 0 > vn_view.type_.type_order_formal(&rep_view.type_).unwrap_or(0) {
                rep = vn;
                rep_view = vn_view;
            }
        }
        rep
    }

    /// (Re)derive the data-type for \b this from the member Varnodes (C++
    /// `HighVariable::updateType`, `variable.cc:400-416`).
    ///
    /// `symbol_submeta` is forwarded to [`HighVariable::strip_type`].
    pub fn update_type(&mut self, ctx: &dyn HighContext, symbol_submeta: Option<type_metatype>) {
        if (self.highflags & high_flags::typedirty) == 0 {
            return; // Type is up to date
        }
        self.highflags &= !high_flags::typedirty; // Mark type as clean
        if (self.highflags & high_flags::type_finalized) != 0 {
            return; // Type has been finalized
        }
        let vn = self.get_type_representative(ctx);
        let vn_view = ctx.vn_view(vn);
        self.type_ = Some(vn_view.type_.clone());
        self.strip_type(ctx, symbol_submeta);
        // Update lock flags
        self.flags &= !varnode_flags::typelock;
        if vn_view.type_lock {
            self.flags |= varnode_flags::typelock;
        }
    }

    /// Get the data-type (C++ `getType`, `variable.hh:175`): runs `updateType`.
    pub fn get_type(&mut self, ctx: &dyn HighContext, symbol_submeta: Option<type_metatype>) -> Rc<Datatype> {
        self.update_type(ctx, symbol_submeta);
        self.type_.clone().expect("getType: null type after update (internal invariant)")
    }

    /// C++ `isTypeLock` (`variable.hh:223`): runs `updateType`.
    pub fn is_type_lock(&mut self, ctx: &dyn HighContext, symbol_submeta: Option<type_metatype>) -> bool {
        self.update_type(ctx, symbol_submeta);
        (self.flags & varnode_flags::typelock) != 0
    }

    // --- Internal/external cover (variable.cc:324-347) ---------------------

    /// (Re)derive the internal cover from member Varnodes (C++
    /// `HighVariable::updateInternalCover`, `variable.cc:324-335`).
    pub fn update_internal_cover(&mut self, ctx: &dyn HighContext) {
        if (self.highflags & high_flags::coverdirty) != 0 {
            self.internal_cover.clear();
            if ctx.vn_has_cover(self.inst[0]) {
                for &vn in &self.inst {
                    if let Some(c) = ctx.vn_cover(vn) {
                        self.internal_cover.merge(&c);
                    }
                }
            }
            self.highflags &= !high_flags::coverdirty;
        }
    }

    /// Borrow the internal cover directly (for the bank's piece-cover union).
    fn internal_cover(&self) -> &Cover {
        &self.internal_cover
    }

    // --- Name resolution (variable.cc:439-511) -----------------------------

    /// Compare two Varnodes based just on storage address (C++
    /// `HighVariable::compareJustLoc`, `variable.cc:439-443`).
    pub fn compare_just_loc(a: &VarnodeViewLoc, b: &VarnodeViewLoc) -> bool {
        a.addr < b.addr
    }

    /// Members are sorted on naming properties (C++ `HighVariable::compareName`,
    /// `variable.cc:456-488`).  Returns \b true if `vn2`'s name overrides `vn1`'s.
    pub fn compare_name(vn1: &CompareNameView, vn2: &CompareNameView) -> bool {
        if vn1.name_lock {
            return false; // Check for namelocks
        }
        if vn2.name_lock {
            return true;
        }
        if vn1.unaffected != vn2.unaffected {
            return vn2.unaffected; // Prefer unaffected
        }
        if vn1.persist != vn2.persist {
            return vn2.persist; // Prefer persistent
        }
        if vn1.input != vn2.input {
            return vn2.input; // Prefer an input
        }
        if vn1.addr_tied != vn2.addr_tied {
            return vn2.addr_tied; // Prefer address tied
        }
        if vn1.proto_partial != vn2.proto_partial {
            return vn2.proto_partial; // Prefer pieces
        }
        // Prefer NOT internal
        if !vn1.space_internal && vn2.space_internal {
            return false;
        }
        if vn1.space_internal && !vn2.space_internal {
            return true;
        }
        if vn1.written != vn2.written {
            return vn2.written; // Prefer written
        }
        if !vn1.written {
            return false;
        }
        // Prefer earlier
        if vn1.def_time != vn2.def_time {
            return vn2.def_time < vn1.def_time;
        }
        false
    }

    // --- Membership mutation (variable.cc:515-545) -------------------------

    /// Find the index that will retrieve the given Varnode member (C++
    /// `HighVariable::instanceIndex`, `variable.cc:821-830`): -1 if not a member.
    pub fn instance_index(&self, vn: VarnodeId) -> int4 {
        for (i, &m) in self.inst.iter().enumerate() {
            if m == vn {
                return i as int4;
            }
        }
        -1
    }

    /// Get a member Varnode that dictates the naming of \b this HighVariable
    /// (C++ `HighVariable::getNameRepresentative`, `variable.cc:492-511`).
    pub fn get_name_representative(&mut self, ctx: &dyn HighContext) -> VarnodeId {
        if (self.highflags & high_flags::namerepdirty) == 0 {
            return self.name_representative.expect("nameRep up to date but null");
        }
        self.highflags &= !high_flags::namerepdirty;
        let mut name_rep = self.inst[0];
        let mut rep_view = ctx.vn_name_view(name_rep);
        for &vn in self.inst.iter().skip(1) {
            let vn_view = ctx.vn_name_view(vn);
            if HighVariable::compare_name(&rep_view, &vn_view) {
                name_rep = vn;
                rep_view = vn_view;
            }
        }
        self.name_representative = Some(name_rep);
        name_rep
    }

    /// Determine if \b this can have a name (C++ `HighVariable::hasName`,
    /// `variable.cc:731-760`).
    ///
    /// Mirrors the C++ exactly; the `LowlevelError` throws become `KunaError`s
    /// (ADR 0004).  `getInputVarnode`'s illegal-input/spacebase reads come from
    /// the member [`VarnodeView`]s.
    pub fn has_name(&mut self, ctx: &dyn HighContext) -> kuna_base::error::KunaResult<bool> {
        let mut indirectonly = true;
        for i in 0..self.inst.len() {
            let vn = self.inst[i];
            let view = ctx.vn_view(vn);
            let has_cover = (view.flags
                & (varnode_flags::constant | varnode_flags::annotation | varnode_flags::insert))
                == varnode_flags::insert;
            if !has_cover {
                if self.inst.len() > 1 {
                    return Err(kuna_base::error::KunaError::lowlevel(
                        "Non-coverable varnode has been merged",
                    ));
                }
                return Ok(false);
            }
            if (view.flags & varnode_flags::implied) != 0 {
                if self.inst.len() > 1 {
                    return Err(kuna_base::error::KunaError::lowlevel(
                        "Implied varnode has been merged",
                    ));
                }
                return Ok(false);
            }
            if (view.flags & varnode_flags::indirectonly) == 0 {
                indirectonly = false;
            }
        }
        if self.is_unaffected(ctx) {
            if !self.is_input(ctx) {
                return Ok(false);
            }
            if indirectonly {
                return Ok(false);
            }
            let vn = self.get_input_varnode(ctx)?;
            let view = ctx.vn_view(vn);
            let illegal_input = (view.flags & (varnode_flags::input | varnode_flags::directwrite))
                == varnode_flags::input;
            if !illegal_input {
                // A legal input, unaff, gets named unless it is the stackpointer
                if (view.flags & varnode_flags::spacebase) != 0 {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Find the first address-tied member Varnode (C++
    /// `HighVariable::getTiedVarnode`, `variable.cc:765-775`).
    pub fn get_tied_varnode(&self, ctx: &dyn HighContext) -> kuna_base::error::KunaResult<VarnodeId> {
        for &vn in &self.inst {
            let view = ctx.vn_view(vn);
            if (view.flags & (varnode_flags::addrtied | varnode_flags::insert))
                == (varnode_flags::addrtied | varnode_flags::insert)
            {
                return Ok(vn);
            }
        }
        Err(kuna_base::error::KunaError::lowlevel("Could not find address-tied varnode"))
    }

    /// Find (the) input member Varnode (C++ `HighVariable::getInputVarnode`,
    /// `variable.cc:780-787`).
    pub fn get_input_varnode(&self, ctx: &dyn HighContext) -> kuna_base::error::KunaResult<VarnodeId> {
        for &vn in &self.inst {
            if (ctx.vn_view(vn).flags & varnode_flags::input) != 0 {
                return Ok(vn);
            }
        }
        Err(kuna_base::error::KunaError::lowlevel("Could not find input varnode"))
    }

    /// Search for the given Varnode and cut it out, marking all properties dirty
    /// (C++ `HighVariable::remove`, `variable.cc:515-532`).
    ///
    /// `vn_has_symbol_entry` is `vn->getSymbolEntry() != 0` (the symbol-dirty
    /// trigger); the piece extend-cover dirtying is the bank's (it has the piece
    /// arena), signalled by the returned `bool` (`true` if \b this is a piece).
    /// Membership ordering uses `compareJustLoc` over the supplied loc views.
    fn remove(&mut self, vn: VarnodeId, vn_has_symbol_entry: bool, ctx: &dyn HighContext) -> bool {
        // lower_bound(inst, vn, compareJustLoc) then linear scan for ==
        let vn_loc = ctx.vn_loc_view(vn);
        let start = self.lower_bound_loc(&vn_loc, ctx);
        for i in start..self.inst.len() {
            if self.inst[i] == vn {
                self.inst.remove(i);
                self.highflags |= high_flags::flagsdirty
                    | high_flags::namerepdirty
                    | high_flags::coverdirty
                    | high_flags::typedirty;
                if vn_has_symbol_entry {
                    self.highflags |= high_flags::symboldirty;
                }
                return self.piece.is_some();
            }
        }
        false
    }

    /// Directly insert a Varnode into \b this (C++ `HighVariable::insert`,
    /// `variable.cc:538-545`).  The cross-arena `newvn->setHigh(this,mergeGroup)`
    /// is done by the bank/caller (it owns the VarnodeBank).
    fn insert(&mut self, newvn: VarnodeId, ctx: &dyn HighContext) {
        let newvn_loc = ctx.vn_loc_view(newvn);
        let pos = self.lower_bound_loc(&newvn_loc, ctx);
        self.inst.insert(pos, newvn);
    }

    /// `lower_bound(inst, key, compareJustLoc)` — first index whose loc is not
    /// before `key`'s.
    fn lower_bound_loc(&self, key: &VarnodeViewLoc, ctx: &dyn HighContext) -> usize {
        // inst is kept sorted by compareJustLoc (address); binary-search it.
        let mut lo = 0usize;
        let mut hi = self.inst.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mid_loc = ctx.vn_loc_view(self.inst[mid]);
            // *mid < key  ->  search right
            if HighVariable::compare_just_loc(&mid_loc, key) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

// =============================================================================
// Lightweight views for the static comparators (compareJustLoc / compareName)
// =============================================================================

/// The single field `compareJustLoc` reads (`vn->getAddr()`).
#[derive(Debug, Clone)]
pub struct VarnodeViewLoc {
    /// `vn->getAddr()`
    pub addr: kuna_base::address::Address,
}

/// The fields `compareName` reads off each Varnode.
#[derive(Debug, Clone, Copy)]
pub struct CompareNameView {
    /// `vn->isNameLock()`
    pub name_lock: bool,
    /// `vn->isUnaffected()`
    pub unaffected: bool,
    /// `vn->isPersist()`
    pub persist: bool,
    /// `vn->isInput()`
    pub input: bool,
    /// `vn->isAddrTied()`
    pub addr_tied: bool,
    /// `vn->isProtoPartial()`
    pub proto_partial: bool,
    /// `vn->getSpace()->getType()==IPTR_INTERNAL`
    pub space_internal: bool,
    /// `vn->isWritten()`
    pub written: bool,
    /// `vn->getDef()->getTime()` (only meaningful when `written`)
    pub def_time: uint4,
}

// =============================================================================
// HighEdge / HighIntersectTest (variable.hh:239-272, variable.cc:1058-1217)
// =============================================================================

/// A record for caching a Cover intersection test between two HighVariables
/// (C++ `class HighEdge`, `variable.hh:239-247`).
///
/// The C++ `operator<` orders by the raw `HighVariable *` pointer pair; here the
/// derived `Ord` over the [`HighVariableId`] pair is that order (ADR 0002).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HighEdge {
    /// First HighVariable of the pair (C++ `a`).
    pub a: HighVariableId,
    /// Second HighVariable of the pair (C++ `b`).
    pub b: HighVariableId,
}

impl HighEdge {
    /// C++ `HighEdge(HighVariable *c,HighVariable *d)`.
    pub fn new(a: HighVariableId, b: HighVariableId) -> HighEdge {
        HighEdge { a, b }
    }
}

/// A cache of Cover intersection tests for HighVariables (C++ class
/// `HighIntersectTest`, `variable.hh:258-272`).
///
/// The block-intersection refinement (`copyShadow`/`partialCopyShadow`) and the
/// untied-call test (`affectsTest`) route through callbacks since the underlying
/// `Varnode::copyShadow` (varnode.cc) is not yet ported and `affectsTest` is the
/// `PcodeOpSet` owner's secondary test.
pub struct HighIntersectTest {
    /// A cache of intersection tests, sorted by HighVariable pair (C++
    /// `highedgemap`).
    highedgemap: BTreeMap<HighEdge, bool>,
    /// PcodeOps that may indirectly affect the intersection test (C++
    /// `affectingOps`).
    affecting_ops: PcodeOpSet,
}

impl HighIntersectTest {
    pub fn new(affecting_ops: PcodeOpSet) -> HighIntersectTest {
        HighIntersectTest { highedgemap: BTreeMap::new(), affecting_ops }
    }

    /// Clear any cached tests (C++ `clear`).
    pub fn clear(&mut self) {
        self.highedgemap.clear();
    }

    /// Remove cached intersection tests for a given HighVariable (C++
    /// `HighIntersectTest::purgeHigh`, `variable.cc:1058-1073`).
    pub fn purge_high(&mut self, high: HighVariableId) {
        // Collect the `b` endpoints of all (high, b) edges, then erase both
        // directions of each.  The C++ does the same with lower_bound ranges and
        // an erase that avoids invalidating the iterator; over a BTreeMap we
        // gather then remove, which is order-equivalent (both directions removed).
        let bs: Vec<HighVariableId> = self
            .highedgemap
            .range(HighEdge::new(high, HighVariableId(0))..)
            .take_while(|(e, _)| e.a == high)
            .map(|(e, _)| e.b)
            .collect();
        if bs.is_empty() {
            return;
        }
        for b in bs {
            self.highedgemap.remove(&HighEdge::new(b, high));
            self.highedgemap.remove(&HighEdge::new(high, b));
        }
    }

    /// Translate intersection tests for `high2` into tests for `high1` ahead of a
    /// merge (C++ `HighIntersectTest::moveIntersectTests`, `variable.cc:1104-1157`).
    ///
    /// `marks` is the bank's HighVariable mark surface (the C++ `b->setMark()` /
    /// `clearMark` / `isMark`), supplied as a closure pair so the bank's arena
    /// owns the mark bit.
    pub fn move_intersect_tests(
        &mut self,
        high1: HighVariableId,
        high2: HighVariableId,
        set_mark: &mut dyn FnMut(HighVariableId),
        clear_mark: &mut dyn FnMut(HighVariableId),
        is_mark: &dyn Fn(HighVariableId) -> bool,
    ) {
        let mut yesinter: Vec<HighVariableId> = Vec::new(); // Highs high2 intersects
        let mut nointer: Vec<HighVariableId> = Vec::new(); // Highs high2 does not

        // Walk the (high2, b) range.
        let high2_edges: Vec<(HighVariableId, bool)> = self
            .highedgemap
            .range(HighEdge::new(high2, HighVariableId(0))..)
            .take_while(|(e, _)| e.a == high2)
            .map(|(e, &v)| (e.b, v))
            .collect();
        for (b, v) in &high2_edges {
            if *b == high1 {
                continue;
            }
            if *v {
                // Save all high2's intersections; still valid for the merge
                yesinter.push(*b);
            } else {
                nointer.push(*b);
                set_mark(*b); // Mark that high2 did not intersect
            }
        }
        // Purge all high2 tests (both directions).
        for (b, _) in &high2_edges {
            self.highedgemap.remove(&HighEdge::new(*b, high2));
            self.highedgemap.remove(&HighEdge::new(high2, *b));
        }

        // Walk the (high1, b) range, deleting intersection==false tests that had
        // no corresponding high2 test (b not marked).
        let high1_bs: Vec<(HighVariableId, bool)> = self
            .highedgemap
            .range(HighEdge::new(high1, HighVariableId(0))..)
            .take_while(|(e, _)| e.a == high1)
            .map(|(e, &v)| (e.b, v))
            .collect();
        for (b, v) in high1_bs {
            if !v && !is_mark(b) {
                // Delete both edges of the test
                self.highedgemap.remove(&HighEdge::new(b, high1));
                self.highedgemap.remove(&HighEdge::new(high1, b));
            }
            // else keep
        }
        for &b in &nointer {
            clear_mark(b);
        }
        // Reinsert high2's intersection==true tests for high1.
        for &b in &yesinter {
            self.highedgemap.insert(HighEdge::new(high1, b), true);
            self.highedgemap.insert(HighEdge::new(b, high1), true);
        }
    }

    /// Look up a cached test result, if present (C++ `highedgemap.find`).
    pub fn cached(&self, a: HighVariableId, b: HighVariableId) -> Option<bool> {
        self.highedgemap.get(&HighEdge::new(a, b)).copied()
    }

    /// Cache a symmetric test result (C++ `highedgemap[HighEdge(a,b)] = res; …`).
    pub fn cache_result(&mut self, a: HighVariableId, b: HighVariableId, res: bool) {
        self.highedgemap.insert(HighEdge::new(a, b), res);
        self.highedgemap.insert(HighEdge::new(b, a), res);
    }

    /// Borrow the affecting-ops set (the untied-call secondary test owner).
    pub fn affecting_ops_mut(&mut self) -> &mut PcodeOpSet {
        &mut self.affecting_ops
    }
}

// =============================================================================
// HighVariableBank — the Funcdata-owned HighVariable / VariableGroup / piece
// arena (ADR 0001).  HighVariables are created/deleted as raw `new`/`delete` in
// C++; here they live in monotonically-keyed maps so HighVariableId order is a
// stable total order (the HighEdge comparator).
// =============================================================================

/// Owns the [`HighVariable`], [`VariableGroup`], and [`VariablePiece`] arenas
/// for one function (the part of `Funcdata` the C++ `new HighVariable`/`Merge`
/// allocate into).  Keyed monotonically so [`HighVariableId`] order is stable.
#[derive(Debug, Default)]
pub struct HighVariableBank {
    highs: BTreeMap<HighVariableId, HighVariable>,
    groups: BTreeMap<VariableGroupId, VariableGroup>,
    pieces: BTreeMap<VariablePieceId, VariablePiece>,
    next_high: u32,
    next_group: u32,
    next_piece: u32,
}

impl HighVariableBank {
    /// Construct an empty bank.
    pub fn new() -> HighVariableBank {
        HighVariableBank::default()
    }

    /// Clear all HighVariables/groups/pieces (C++ `Funcdata::clearDeadVarnodes`
    /// teardown of the high arena, and `clear`).
    pub fn clear(&mut self) {
        self.highs.clear();
        self.groups.clear();
        self.pieces.clear();
        // Note: ids are NOT reset, preserving stable monotonic identity within a
        // Funcdata lifetime (matches that C++ never reuses a freed pointer's
        // address within a meaningful window for the HighEdge cache).
    }

    /// Number of live HighVariables.
    pub fn num_highs(&self) -> usize {
        self.highs.len()
    }

    /// Borrow a HighVariable by id.
    pub fn get(&self, id: HighVariableId) -> Option<&HighVariable> {
        self.highs.get(&id)
    }

    /// Iterate `(id, &HighVariable)` over every live HighVariable.  Used by the
    /// printer's local declaration loop to find a piece's whole-symbol sibling (the
    /// `getFirstWholeMap()` root) without a dedicated group index.
    pub fn iter(&self) -> impl Iterator<Item = (HighVariableId, &HighVariable)> + '_ {
        self.highs.iter().map(|(id, h)| (*id, h))
    }

    /// Mutably borrow a HighVariable by id.
    pub fn get_mut(&mut self, id: HighVariableId) -> Option<&mut HighVariable> {
        self.highs.get_mut(&id)
    }

    /// Allocate a new HighVariable owning the single Varnode `vn` (C++ `new
    /// HighVariable(vn)`).  Returns the new id; the caller is responsible for the
    /// cross-arena `vn->setHigh(id, mergeGroup)` (the bank does not own the
    /// VarnodeBank).
    pub fn new_high(&mut self, vn: VarnodeId) -> HighVariableId {
        let id = HighVariableId(self.next_high);
        self.next_high += 1;
        self.highs.insert(id, HighVariable::new(vn));
        id
    }

    /// Mark a HighVariable's cover as dirty, propagating extended-cover dirtiness
    /// to intersecting pieces if it is part of a group (C++ inline `coverDirty` +
    /// `VariablePiece::markExtendCoverDirty`, `variable.hh:276-282`,
    /// `variable.cc:128-137`).
    pub fn cover_dirty(&mut self, id: HighVariableId) {
        if let Some(h) = self.highs.get_mut(&id) {
            h.highflags |= high_flags::coverdirty;
        }
        let piece = self.highs.get(&id).and_then(|h| h.piece);
        if let Some(p) = piece {
            self.mark_extend_cover_dirty(p);
        }
    }

    /// C++ `VariablePiece::markExtendCoverDirty` (`variable.cc:128-137`).
    fn mark_extend_cover_dirty(&mut self, piece_id: VariablePieceId) {
        let (high, intersection) = match self.pieces.get(&piece_id) {
            None => return,
            Some(p) => (p.high, p.intersection.clone()),
        };
        if let Some(h) = self.highs.get(&high) {
            if (h.highflags & high_flags::intersectdirty) != 0 {
                return; // intersection list dirty, covers recomputed anyway
            }
        }
        for inter in intersection {
            let ih = self.pieces.get(&inter).map(|p| p.high);
            if let Some(ih) = ih {
                if let Some(h) = self.highs.get_mut(&ih) {
                    h.highflags |= high_flags::extendcoverdirty;
                }
            }
        }
        if let Some(h) = self.highs.get_mut(&high) {
            h.highflags |= high_flags::extendcoverdirty;
        }
    }

    /// C++ `VariablePiece::markIntersectionDirty` (`variable.cc:119-126`): mark
    /// every piece in the group as needing intersection+extended-cover recompute.
    fn mark_intersection_dirty(&mut self, piece_id: VariablePieceId) {
        let group = match self.pieces.get(&piece_id) {
            None => return,
            Some(p) => p.group,
        };
        let members: Vec<VariablePieceId> = match self.groups.get(&group) {
            None => return,
            Some(g) => g.piece_set.iter().map(|k| k.id).collect(),
        };
        for pid in members {
            let high = self.pieces.get(&pid).map(|p| p.high);
            if let Some(high) = high {
                if let Some(h) = self.highs.get_mut(&high) {
                    h.highflags |= high_flags::intersectdirty | high_flags::extendcoverdirty;
                }
            }
        }
    }

    /// (Re)derive the external cover of a HighVariable (C++
    /// `HighVariable::updateCover`, `variable.cc:338-347`): for a grouped
    /// variable this updates intersections and the extended cover; otherwise the
    /// internal cover.  Driven by the Merge class.
    pub fn update_cover(&mut self, id: HighVariableId, ctx: &dyn HighContext) {
        let piece = self.highs.get(&id).and_then(|h| h.piece);
        match piece {
            None => {
                if let Some(h) = self.highs.get_mut(&id) {
                    h.update_internal_cover(ctx);
                }
            }
            Some(p) => {
                self.update_intersections(p);
                self.update_piece_cover(p, ctx);
            }
        }
    }

    /// C++ `VariablePiece::updateIntersections` (`variable.cc:140-157`).
    fn update_intersections(&mut self, piece_id: VariablePieceId) {
        let (high, group_offset, size, group) = match self.pieces.get(&piece_id) {
            None => return,
            Some(p) => (p.high, p.group_offset, p.size, p.group),
        };
        if let Some(h) = self.highs.get(&high) {
            if (h.highflags & high_flags::intersectdirty) == 0 {
                return;
            }
        }
        let end_offset = group_offset + size;
        let members: Vec<(VariablePieceId, int4, int4)> = match self.groups.get(&group) {
            None => return,
            Some(g) => g
                .piece_set
                .iter()
                .map(|k| (k.id, k.offset, k.size))
                .collect(),
        };
        let mut intersection = Vec::new();
        for (other_id, other_offset, other_size) in members {
            if other_id == piece_id {
                continue;
            }
            if end_offset <= other_offset {
                continue;
            }
            let other_end = other_offset + other_size;
            if group_offset >= other_end {
                continue;
            }
            intersection.push(other_id);
        }
        if let Some(p) = self.pieces.get_mut(&piece_id) {
            p.intersection = intersection;
        }
        if let Some(h) = self.highs.get_mut(&high) {
            h.highflags &= !high_flags::intersectdirty;
        }
    }

    /// C++ `VariablePiece::updateCover` (`variable.cc:160-172`): union the
    /// internal covers of all intersecting pieces into this piece's extended
    /// cover.
    fn update_piece_cover(&mut self, piece_id: VariablePieceId, ctx: &dyn HighContext) {
        let (high, intersection) = match self.pieces.get(&piece_id) {
            None => return,
            Some(p) => (p.high, p.intersection.clone()),
        };
        if let Some(h) = self.highs.get(&high) {
            if (h.highflags & (high_flags::coverdirty | high_flags::extendcoverdirty)) == 0 {
                return;
            }
        }
        if let Some(h) = self.highs.get_mut(&high) {
            h.update_internal_cover(ctx);
        }
        let mut cover = self
            .highs
            .get(&high)
            .map(|h| h.internal_cover().clone())
            .unwrap_or_default();
        for inter in intersection {
            let ih = self.pieces.get(&inter).map(|p| p.high);
            if let Some(ih) = ih {
                if let Some(h) = self.highs.get_mut(&ih) {
                    h.update_internal_cover(ctx);
                }
                if let Some(c) = self.highs.get(&ih).map(|h| h.internal_cover().clone()) {
                    cover.merge(&c);
                }
            }
        }
        if let Some(p) = self.pieces.get_mut(&piece_id) {
            p.cover = cover;
        }
        if let Some(h) = self.highs.get_mut(&high) {
            h.highflags &= !high_flags::extendcoverdirty;
        }
    }

    /// Get the cover associated with a HighVariable (C++ inline `getCover`,
    /// `variable.hh:295-301`): the internal cover, or the piece's extended cover.
    pub fn get_cover(&self, id: HighVariableId) -> Option<&Cover> {
        let h = self.highs.get(&id)?;
        match h.piece {
            None => Some(&h.internal_cover),
            Some(p) => self.pieces.get(&p).map(|piece| &piece.cover),
        }
    }

    /// Raw internal cover of a HighVariable (C++ `high->internalCover`,
    /// distinct from `getCover()` which returns the piece's extended cover when
    /// the high is grouped).  Used by `Merge::inflateTest` (`merge.cc:1622`).
    pub fn internal_cover(&self, id: HighVariableId) -> Option<&Cover> {
        self.highs.get(&id).map(|h| &h.internal_cover)
    }

    /// The HighVariable's `piece` id, if it belongs to a [`VariableGroup`] (C++
    /// `high->piece`).
    pub fn high_piece_id(&self, id: HighVariableId) -> Option<VariablePieceId> {
        self.highs.get(&id).and_then(|h| h.piece)
    }

    /// Byte offset of a piece within its group (C++ `VariablePiece::getOffset`).
    pub fn piece_offset(&self, piece: VariablePieceId) -> int4 {
        self.pieces.get(&piece).map(|p| p.group_offset).unwrap_or(0)
    }

    /// Number of bytes in a piece (C++ `VariablePiece::getSize`).
    pub fn piece_size(&self, piece: VariablePieceId) -> int4 {
        self.pieces.get(&piece).map(|p| p.size).unwrap_or(0)
    }

    /// The central group id of a piece (C++ `VariablePiece::getGroup`).
    pub fn piece_group(&self, piece: VariablePieceId) -> Option<VariableGroupId> {
        self.pieces.get(&piece).map(|p| p.group)
    }

    /// The HighVariable owning a piece (C++ `VariablePiece::getHigh`).
    pub fn piece_high(&self, piece: VariablePieceId) -> Option<HighVariableId> {
        self.pieces.get(&piece).map(|p| p.high)
    }

    /// Number of contiguous bytes covered by a whole group (C++
    /// `VariableGroup::getSize`).
    pub fn group_size(&self, group: VariableGroupId) -> int4 {
        self.groups.get(&group).map(|g| g.size).unwrap_or(0)
    }

    /// Number of pieces a piece intersects with (C++
    /// `VariablePiece::numIntersection`), after the intersection list is current.
    pub fn piece_num_intersection(&self, piece: VariablePieceId) -> int4 {
        self.pieces.get(&piece).map(|p| p.intersection.len() as int4).unwrap_or(0)
    }

    /// The i-th piece a piece intersects with (C++
    /// `VariablePiece::getIntersection`).
    pub fn piece_get_intersection(&self, piece: VariablePieceId, i: int4) -> Option<VariablePieceId> {
        self.pieces.get(&piece).and_then(|p| p.intersection.get(i as usize).copied())
    }

    /// Recompute a piece's intersection list (C++
    /// `VariablePiece::updateIntersections`, driven by `Merge::inflateTest` /
    /// `intersection`).  Public wrapper for the private `update_intersections`.
    pub fn update_piece_intersections(&mut self, piece: VariablePieceId) {
        self.update_intersections(piece);
    }

    /// Is a HighVariable part of the same VariableGroup as another (C++ inline
    /// `isSameGroup`, `variable.hh:306-312`).
    pub fn is_same_group(&self, a: HighVariableId, b: HighVariableId) -> bool {
        let pa = self.highs.get(&a).and_then(|h| h.piece);
        let pb = self.highs.get(&b).and_then(|h| h.piece);
        match (pa, pb) {
            (Some(pa), Some(pb)) => {
                let ga = self.pieces.get(&pa).map(|p| p.group);
                let gb = self.pieces.get(&pb).map(|p| p.group);
                ga.is_some() && ga == gb
            }
            _ => false,
        }
    }

    /// C++ `HighVariable::establishGroupSymbolOffset` (`variable.cc:623`): if `id`
    /// is part of a larger group and has had its `symboloffset` set, use it to
    /// compute the group's `symbolOffset` so every other HighVariable in the group
    /// can derive its own in-symbol offset.
    ///
    /// Returns `Err` on the `off < 0` invariant violation (the C++ `throw`), so the
    /// caller can fall back rather than abort.
    pub fn establish_group_symbol_offset(
        &mut self,
        id: HighVariableId,
    ) -> kuna_base::error::KunaResult<()> {
        let piece = self.highs.get(&id).and_then(|h| h.piece);
        let piece = match piece {
            Some(p) => p,
            // C++ derefs `piece` unconditionally; a non-group high never reaches
            // here in the call sites (guarded by `isSameGroup`), so a missing
            // piece is a no-op rather than a panic.
            None => return Ok(()),
        };
        let group = match self.pieces.get(&piece).map(|p| p.group) {
            Some(g) => g,
            None => return Ok(()),
        };
        let mut off = self.highs.get(&id).map(|h| h.symbol_offset).unwrap_or(-1);
        if off < 0 {
            off = 0;
        }
        off -= self.pieces.get(&piece).map(|p| p.group_offset).unwrap_or(0);
        if off < 0 {
            return Err(kuna_base::error::KunaError::lowlevel(
                "Symbol offset is incompatible with VariableGroup",
            ));
        }
        if let Some(g) = self.groups.get_mut(&group) {
            g.symbol_offset = off;
        }
        Ok(())
    }

    /// The group-relative symbol offset cached on a HighVariable's group (C++
    /// `piece->getGroup()->getSymbolOffset()`), or 0 when the high is groupless.
    pub fn group_symbol_offset(&self, id: HighVariableId) -> int4 {
        self.highs
            .get(&id)
            .and_then(|h| h.piece)
            .and_then(|p| self.pieces.get(&p))
            .and_then(|p| self.groups.get(&p.group))
            .map(|g| g.symbol_offset)
            .unwrap_or(0)
    }

    /// Remove a member Varnode from a HighVariable (C++ `HighVariable::remove`),
    /// propagating the piece extend-cover dirtiness (the C++
    /// `piece->markExtendCoverDirty()` at the end of `remove`).
    pub fn remove_member(
        &mut self,
        id: HighVariableId,
        vn: VarnodeId,
        vn_has_symbol_entry: bool,
        ctx: &dyn HighContext,
    ) {
        let is_piece = match self.highs.get_mut(&id) {
            None => return,
            Some(h) => h.remove(vn, vn_has_symbol_entry, ctx),
        };
        if is_piece {
            if let Some(p) = self.highs.get(&id).and_then(|h| h.piece) {
                self.mark_extend_cover_dirty(p);
            }
        }
    }

    /// Return \b true if the given HighVariable has no member Varnodes left
    /// (C++ `HighVariable::isUnattached`).  Absent ids count as unattached.
    pub fn is_unattached(&self, id: HighVariableId) -> bool {
        self.highs.get(&id).map(|h| h.is_unattached()).unwrap_or(true)
    }

    /// Delete an (unattached) HighVariable from the bank (C++ `delete high`
    /// in `Varnode::~Varnode`, run once the high has lost its last member).
    pub fn erase(&mut self, id: HighVariableId) {
        self.highs.remove(&id);
    }

    /// Swap `origvn` with `replacevn` between their respective HighVariables
    /// (C++ `Varnode::replaceInHigh`, `varnode.cc:353`).
    ///
    /// `replacevn` must be a singleton in its own HighVariable (the C++
    /// invariant); after the swap, `replacevn` takes `origvn`'s place in
    /// `origvn`'s old high, and `origvn` becomes the sole member of `replacevn`'s
    /// old high.  The cross-arena `vn->setHigh(...)` writes are performed via the
    /// `set_high` closure (the bank does not own the VarnodeBank).
    #[allow(clippy::too_many_arguments)] // the C++ Varnode::replaceInHigh data + cross-arena set_high closure
    pub fn replace_in_high(
        &mut self,
        origvn: VarnodeId,
        orig_high: HighVariableId,
        orig_mergegroup: int2,
        replacevn: VarnodeId,
        replace_high: HighVariableId,
        vn_has_symbol_entry: bool,
        ctx: &dyn HighContext,
        set_high: &mut dyn FnMut(VarnodeId, HighVariableId, int2),
    ) {
        // Remove origvn from origHigh.
        if let Some(h) = self.highs.get_mut(&orig_high) {
            h.remove(origvn, vn_has_symbol_entry, ctx);
        }
        // replaceHigh now holds origvn (singleton).
        if let Some(rh) = self.highs.get_mut(&replace_high) {
            // replacevn was the sole member; overwrite it with origvn.
            rh.inst.clear();
            rh.inst.push(origvn);
            rh.highflags |= high_flags::flagsdirty
                | high_flags::namerepdirty
                | high_flags::coverdirty
                | high_flags::typedirty;
        }
        // origHigh now holds replacevn.
        if let Some(h) = self.highs.get_mut(&orig_high) {
            h.insert(replacevn, ctx);
            h.highflags |= high_flags::flagsdirty
                | high_flags::namerepdirty
                | high_flags::coverdirty
                | high_flags::typedirty;
        }
        // replacevn->high = 0  was set in C++ before the inst swap; here the
        // cross-arena writes finalize both varnodes' high back-links:
        //   high = replaceHigh; mergegroup = 0;        (origvn)
        //   (replacevn keeps origvn's old mergegroup, now points to origHigh)
        set_high(origvn, replace_high, 0);
        set_high(replacevn, orig_high, orig_mergegroup);
    }

    /// Insert a Varnode into a HighVariable and wire the back-link (C++
    /// `HighVariable::insert` + the `newvn->setHigh(this,mergeGroup)`).
    pub fn insert_member(
        &mut self,
        id: HighVariableId,
        newvn: VarnodeId,
        merge_group: int2,
        ctx: &dyn HighContext,
        set_high: &mut dyn FnMut(VarnodeId, HighVariableId, int2),
    ) {
        if let Some(h) = self.highs.get_mut(&id) {
            h.insert(newvn, ctx);
        }
        set_high(newvn, id, merge_group);
    }

    /// Merge another HighVariable's members into \b this and delete it (C++
    /// `HighVariable::mergeInternal`, `variable.cc:639-679`).
    ///
    /// `set_high` wires each moved member's `vn->setHigh(this,...)`; the merged
    /// instance list is the `std::merge` of the two `compareJustLoc`-sorted
    /// lists, exactly as the C++.  Returns an error on the non-speculative-after-
    /// speculative invariant violation (ADR 0004).
    pub fn merge_internal(
        &mut self,
        id1: HighVariableId,
        id2: HighVariableId,
        isspeculative: bool,
        ctx: &dyn HighContext,
        set_high: &mut dyn FnMut(VarnodeId, HighVariableId, int2),
    ) -> kuna_base::error::KunaResult<()> {
        // highflags |= flagsdirty|namerepdirty|typedirty;
        if let Some(h) = self.highs.get_mut(&id1) {
            h.highflags |= high_flags::flagsdirty | high_flags::namerepdirty | high_flags::typedirty;
        }
        // Inherit Symbol if tv2 has one and is not symbol-dirty
        let (tv2_symbol, tv2_symboloffset, tv2_symdirty) = {
            let h2 = self.highs.get(&id2).expect("merge_internal: stale id2");
            (h2.symbol, h2.symbol_offset, (h2.highflags & high_flags::symboldirty) != 0)
        };
        if tv2_symbol.is_some() && !tv2_symdirty {
            if let Some(h1) = self.highs.get_mut(&id1) {
                h1.symbol = tv2_symbol;
                h1.symbol_offset = tv2_symboloffset;
                h1.highflags &= !high_flags::symboldirty;
            }
        }

        let num1 = self.highs.get(&id1).unwrap().num_merge_classes;
        let inst2: Vec<VarnodeId> = self.highs.get(&id2).unwrap().inst.clone();
        if isspeculative {
            for &vn in &inst2 {
                let mg = ctx.vn_view(vn).merge_group;
                set_high(vn, id1, mg + num1 as int2);
            }
            let num2 = self.highs.get(&id2).unwrap().num_merge_classes;
            self.highs.get_mut(&id1).unwrap().num_merge_classes += num2;
        } else {
            let num2 = self.highs.get(&id2).unwrap().num_merge_classes;
            if num1 != 1 || num2 != 1 {
                return Err(kuna_base::error::KunaError::lowlevel(
                    "Making a non-speculative merge after speculative merges have occurred",
                ));
            }
            for &vn in &inst2 {
                let mg = ctx.vn_view(vn).merge_group;
                set_high(vn, id1, mg);
            }
        }

        // std::merge(inst1, inst2) by compareJustLoc into the combined list.
        let inst1: Vec<VarnodeId> = self.highs.get(&id1).unwrap().inst.clone();
        let merged = self.merge_inst_lists(&inst1, &inst2, ctx);
        {
            let h1 = self.highs.get_mut(&id1).unwrap();
            h1.inst = merged;
        }
        // internalCover merge (only if neither is cover-dirty), else mark dirty.
        let (h1_cd, h2_cd) = {
            let h1 = self.highs.get(&id1).unwrap();
            let h2 = self.highs.get(&id2).unwrap();
            (
                (h1.highflags & high_flags::coverdirty) != 0,
                (h2.highflags & high_flags::coverdirty) != 0,
            )
        };
        if !h1_cd && !h2_cd {
            let cover2 = self.highs.get(&id2).unwrap().internal_cover.clone();
            self.highs.get_mut(&id1).unwrap().internal_cover.merge(&cover2);
        } else {
            self.highs.get_mut(&id1).unwrap().highflags |= high_flags::coverdirty;
        }

        // delete tv2
        self.highs.remove(&id2);
        Ok(())
    }

    /// `std::merge` of two `compareJustLoc`-sorted instance lists (the
    /// `mergeInternal` body, `variable.cc:668-671`).
    fn merge_inst_lists(
        &self,
        a: &[VarnodeId],
        b: &[VarnodeId],
        ctx: &dyn HighContext,
    ) -> Vec<VarnodeId> {
        let mut out = Vec::with_capacity(a.len() + b.len());
        let mut i = 0usize;
        let mut j = 0usize;
        while i < a.len() && j < b.len() {
            let la = ctx.vn_loc_view(a[i]);
            let lb = ctx.vn_loc_view(b[j]);
            // std::merge: take from the SECOND range only if it is strictly less
            // (the first range wins ties), i.e. push a unless b < a.
            if HighVariable::compare_just_loc(&lb, &la) {
                out.push(b[j]);
                j += 1;
            } else {
                out.push(a[i]);
                i += 1;
            }
        }
        out.extend_from_slice(&a[i..]);
        out.extend_from_slice(&b[j..]);
        out
    }

    // -----------------------------------------------------------------------
    // VariableGroup / VariablePiece overlap model (variable.cc:43-216, 584-725)
    // -----------------------------------------------------------------------

    /// Allocate a new [`VariableGroup`].
    fn new_group(&mut self) -> VariableGroupId {
        let id = VariableGroupId(self.next_group);
        self.next_group += 1;
        self.groups.insert(id, VariableGroup::new());
        id
    }

    /// C++ `VariableGroup::addPiece` (`variable.cc:43-52`): add a piece, set its
    /// group, extend the group size.  Errors on a duplicate (offset,size).
    fn group_add_piece(
        &mut self,
        group: VariableGroupId,
        piece: VariablePieceId,
    ) -> kuna_base::error::KunaResult<()> {
        let (offset, size) = {
            let p = self.pieces.get_mut(&piece).expect("group_add_piece: stale piece");
            p.group = group;
            (p.group_offset, p.size)
        };
        let g = self.groups.get_mut(&group).expect("group_add_piece: stale group");
        if !g.piece_set.insert(PieceKey { offset, size, id: piece }) {
            return Err(kuna_base::error::KunaError::lowlevel("Duplicate VariablePiece"));
        }
        let piece_max = offset + size;
        if piece_max > g.size {
            g.size = piece_max;
        }
        Ok(())
    }

    /// C++ `VariableGroup::adjustOffsets` (`variable.cc:56-65`): add `amt` to
    /// every piece offset and the group size.  Rebuilds `piece_set` keys.
    fn group_adjust_offsets(&mut self, group: VariableGroupId, amt: int4) {
        let members: Vec<VariablePieceId> = match self.groups.get(&group) {
            None => return,
            Some(g) => g.piece_set.iter().map(|k| k.id).collect(),
        };
        for pid in &members {
            if let Some(p) = self.pieces.get_mut(pid) {
                p.group_offset += amt;
            }
        }
        // Rebuild the ordered key set with the new offsets.
        let new_keys: BTreeSet<PieceKey> = members
            .iter()
            .map(|&pid| {
                let p = self.pieces.get(&pid).unwrap();
                PieceKey { offset: p.group_offset, size: p.size, id: pid }
            })
            .collect();
        let g = self.groups.get_mut(&group).unwrap();
        g.piece_set = new_keys;
        g.size += amt;
    }

    /// C++ `VariableGroup::removePiece` (`variable.cc:67-72`): drop a piece from
    /// the set (size is not adjusted, matching the C++ comment).
    fn group_remove_piece(&mut self, group: VariableGroupId, piece: VariablePieceId) {
        let key = self.pieces.get(&piece).map(|p| PieceKey {
            offset: p.group_offset,
            size: p.size,
            id: piece,
        });
        if let (Some(g), Some(key)) = (self.groups.get_mut(&group), key) {
            g.piece_set.remove(&key);
        }
    }

    /// C++ `VariablePiece::transferGroup` (`variable.cc:176-183`): move a piece to
    /// a new group, deleting the old group if it becomes empty.
    fn piece_transfer_group(
        &mut self,
        piece: VariablePieceId,
        new_group: VariableGroupId,
    ) -> kuna_base::error::KunaResult<()> {
        let old_group = self.pieces.get(&piece).expect("transferGroup: stale piece").group;
        self.group_remove_piece(old_group, piece);
        if self.groups.get(&old_group).map(|g| g.empty()).unwrap_or(false) {
            self.groups.remove(&old_group);
        }
        self.group_add_piece(new_group, piece)
    }

    /// C++ `VariableGroup::combineGroups` (`variable.cc:78-89`): move every piece
    /// of `op2` into `this_group`, then delete `op2`.
    fn group_combine(
        &mut self,
        this_group: VariableGroupId,
        op2: VariableGroupId,
    ) -> kuna_base::error::KunaResult<()> {
        let members: Vec<VariablePieceId> = match self.groups.get(&op2) {
            None => return Ok(()),
            Some(g) => g.piece_set.iter().map(|k| k.id).collect(),
        };
        for piece in members {
            self.piece_transfer_group(piece, this_group)?;
        }
        Ok(())
    }

    /// Allocate a new [`VariablePiece`] for `high` (C++ `VariablePiece` ctor,
    /// `variable.cc:96-107`).  `grp` (when set) shares the existing group; else a
    /// fresh group is allocated.  `size` is `h->getInstance(0)->getSize()`.
    fn new_piece(
        &mut self,
        high: HighVariableId,
        offset: int4,
        first_size: int4,
        grp: Option<HighVariableId>,
    ) -> kuna_base::error::KunaResult<VariablePieceId> {
        let group = match grp {
            Some(g) => self
                .highs
                .get(&g)
                .and_then(|h| h.piece)
                .and_then(|p| self.pieces.get(&p))
                .map(|p| p.group)
                .expect("new_piece: grp has no piece/group"),
            None => self.new_group(),
        };
        let id = VariablePieceId(self.next_piece);
        self.next_piece += 1;
        self.pieces.insert(
            id,
            VariablePiece {
                group,
                high,
                group_offset: offset,
                size: first_size,
                intersection: Vec::new(),
                cover: Cover::new(),
            },
        );
        self.group_add_piece(group, id)?;
        Ok(id)
    }

    /// Put two HighVariables in the same intersection group (C++
    /// `HighVariable::groupWith`, `variable.cc:584-618`).
    ///
    /// `first_size`/`hi2_first_size` are `getInstance(0)->getSize()` of the
    /// respective highs (the C++ reads them through the piece constructor).
    pub fn group_with(
        &mut self,
        id: HighVariableId,
        off: int4,
        hi2: HighVariableId,
        first_size: int4,
        hi2_first_size: int4,
    ) -> kuna_base::error::KunaResult<()> {
        let this_piece = self.highs.get(&id).and_then(|h| h.piece);
        let hi2_piece = self.highs.get(&hi2).and_then(|h| h.piece);
        match (this_piece, hi2_piece) {
            (None, None) => {
                let p2 = self.new_piece(hi2, 0, hi2_first_size, None)?;
                self.highs.get_mut(&hi2).unwrap().piece = Some(p2);
                let p1 = self.new_piece(id, off, first_size, Some(hi2))?;
                self.highs.get_mut(&id).unwrap().piece = Some(p1);
                self.mark_intersection_dirty(p2);
            }
            (None, Some(p2)) => {
                let hi2_dirty =
                    (self.highs.get(&hi2).unwrap().highflags & high_flags::intersectdirty) != 0;
                if !hi2_dirty {
                    self.mark_intersection_dirty(p2);
                }
                self.highs.get_mut(&id).unwrap().highflags |=
                    high_flags::intersectdirty | high_flags::extendcoverdirty;
                let off = off + self.pieces.get(&p2).unwrap().group_offset;
                let p1 = self.new_piece(id, off, first_size, Some(hi2))?;
                self.highs.get_mut(&id).unwrap().piece = Some(p1);
            }
            (Some(p1), None) => {
                let mut hi2_off = self.pieces.get(&p1).unwrap().group_offset - off;
                if hi2_off < 0 {
                    let group = self.pieces.get(&p1).unwrap().group;
                    self.group_adjust_offsets(group, -hi2_off);
                    hi2_off = 0;
                }
                let this_dirty =
                    (self.highs.get(&id).unwrap().highflags & high_flags::intersectdirty) != 0;
                if !this_dirty {
                    self.mark_intersection_dirty(p1);
                }
                self.highs.get_mut(&hi2).unwrap().highflags |=
                    high_flags::intersectdirty | high_flags::extendcoverdirty;
                let p2 = self.new_piece(hi2, hi2_off, hi2_first_size, Some(id))?;
                self.highs.get_mut(&hi2).unwrap().piece = Some(p2);
            }
            (Some(p1), Some(p2)) => {
                let off_diff = self.pieces.get(&p2).unwrap().group_offset + off
                    - self.pieces.get(&p1).unwrap().group_offset;
                if off_diff != 0 {
                    let group = self.pieces.get(&p1).unwrap().group;
                    self.group_adjust_offsets(group, off_diff);
                }
                let g1 = self.pieces.get(&p1).unwrap().group;
                let g2 = self.pieces.get(&p2).unwrap().group;
                self.group_combine(g2, g1)?;
                self.mark_intersection_dirty(p2);
            }
        }
        Ok(())
    }

    /// C++ `VariablePiece::mergeGroups` (`variable.cc:193-216`): combine the two
    /// groups so the pieces share offsets; returns the HighVariable pairs that
    /// must subsequently be merged (each `(kept, freed)`).  The `kept` piece stays
    /// in the combined group; the `freed` piece is detached and dropped.
    fn merge_groups(&mut self, p1: VariablePieceId, p2: VariablePieceId) -> Vec<(HighVariableId, HighVariableId)> {
        let diff = self.pieces.get(&p1).unwrap().group_offset
            - self.pieces.get(&p2).unwrap().group_offset;
        if diff > 0 {
            let g2 = self.pieces.get(&p2).unwrap().group;
            self.group_adjust_offsets(g2, diff);
        } else if diff < 0 {
            let g1 = self.pieces.get(&p1).unwrap().group;
            self.group_adjust_offsets(g1, -diff);
        }
        let g1 = self.pieces.get(&p1).unwrap().group;
        let g2 = self.pieces.get(&p2).unwrap().group;
        let op2_members: Vec<VariablePieceId> = self
            .groups
            .get(&g2)
            .map(|g| g.piece_set.iter().map(|k| k.id).collect())
            .unwrap_or_default();
        let mut merge_pairs = Vec::new();
        for piece in op2_members {
            let (offset, size, high) = {
                let p = self.pieces.get(&piece).unwrap();
                (p.group_offset, p.size, p.high)
            };
            if let Some(matched) = self.groups.get(&g1).and_then(|g| {
                // pieceSet.find(piece) keys on (offset,size)
                g.piece_set
                    .iter()
                    .find(|k| k.offset == offset && k.size == size)
                    .map(|k| k.id)
            }) {
                let matched_high = self.pieces.get(&matched).unwrap().high;
                merge_pairs.push((matched_high, high));
                // Detach the HighVariable from its original VariablePiece, drop it
                self.highs.get_mut(&high).unwrap().piece = None;
                self.group_remove_piece(g2, piece);
                self.pieces.remove(&piece);
            } else {
                let _ = self.piece_transfer_group(piece, g1);
            }
        }
        // free the now-empty op2 group if it is gone
        if self.groups.get(&g2).map(|g| g.empty()).unwrap_or(true) {
            self.groups.remove(&g2);
        }
        // Demonstrate the (offset,size) finder is the same the C++ relies on.
        debug_assert!(self.find_helper_consistent(g1));
        merge_pairs
    }

    /// Used only by a `debug_assert` in [`merge_groups`] to keep
    /// [`VariableGroup::find_by_offset_size`] (the C++ `pieceSet.find` key) wired
    /// into the build (it is the same lookup `merge_groups` performs inline).
    fn find_helper_consistent(&self, group: VariableGroupId) -> bool {
        match self.groups.get(&group) {
            None => true,
            Some(g) => g
                .piece_set
                .iter()
                .all(|k| g.find_by_offset_size(k.offset, k.size).is_some()),
        }
    }

    /// C++ `HighVariable::transferPiece` (`variable.cc:291-299`): move ownership of
    /// `tv2`'s VariablePiece to `this`.
    fn transfer_piece(&mut self, id1: HighVariableId, id2: HighVariableId) {
        let p = self.highs.get(&id2).and_then(|h| h.piece);
        let (intersectbits, _) = {
            let h2 = self.highs.get(&id2).unwrap();
            (
                h2.highflags & (high_flags::intersectdirty | high_flags::extendcoverdirty),
                (),
            )
        };
        self.highs.get_mut(&id1).unwrap().piece = p;
        self.highs.get_mut(&id2).unwrap().piece = None;
        if let Some(p) = p {
            self.pieces.get_mut(&p).unwrap().high = id1;
        }
        self.highs.get_mut(&id1).unwrap().highflags |= intersectbits;
        self.highs.get_mut(&id2).unwrap().highflags &=
            !(high_flags::intersectdirty | high_flags::extendcoverdirty);
    }

    /// Merge two HighVariables taking groups into account (C++
    /// `HighVariable::merge`, `variable.cc:688-725`).  `tv2` is deleted.
    ///
    /// `test_cache`, when supplied, has its cached intersection tests updated for
    /// the merge (the C++ `testCache->moveIntersectTests`); the mark surface is
    /// the same closure trio `move_intersect_tests` uses.
    #[allow(clippy::too_many_arguments)]
    pub fn merge(
        &mut self,
        id1: HighVariableId,
        id2: HighVariableId,
        isspeculative: bool,
        ctx: &dyn HighContext,
        set_high: &mut dyn FnMut(VarnodeId, HighVariableId, int2),
        test_cache: Option<&mut HighIntersectTest>,
        set_mark: &mut dyn FnMut(HighVariableId),
        clear_mark: &mut dyn FnMut(HighVariableId),
        is_mark: &dyn Fn(HighVariableId) -> bool,
    ) -> kuna_base::error::KunaResult<()> {
        if id2 == id1 {
            return Ok(());
        }
        let mut test_cache = test_cache;
        if let Some(tc) = test_cache.as_deref_mut() {
            tc.move_intersect_tests(id1, id2, set_mark, clear_mark, is_mark);
        }
        let p1 = self.highs.get(&id1).and_then(|h| h.piece);
        let p2 = self.highs.get(&id2).and_then(|h| h.piece);
        match (p1, p2) {
            (None, None) => self.merge_internal(id1, id2, isspeculative, ctx, set_high),
            (Some(p1), None) => {
                // Keep group that id1 is already in
                self.mark_extend_cover_dirty(p1);
                self.merge_internal(id1, id2, isspeculative, ctx, set_high)
            }
            (None, Some(_)) => {
                // Move ownership of the VariablePiece from the freed high
                self.transfer_piece(id1, id2);
                let p1 = self.highs.get(&id1).and_then(|h| h.piece).unwrap();
                self.mark_extend_cover_dirty(p1);
                self.merge_internal(id1, id2, isspeculative, ctx, set_high)
            }
            (Some(p1), Some(p2)) => {
                if isspeculative {
                    return Err(kuna_base::error::KunaError::lowlevel(
                        "Trying speculatively merge variables in separate groups",
                    ));
                }
                let merge_pairs = self.merge_groups(p1, p2);
                for (high1, high2) in merge_pairs {
                    if let Some(tc) = test_cache.as_deref_mut() {
                        tc.move_intersect_tests(high1, high2, set_mark, clear_mark, is_mark);
                    }
                    self.merge_internal(high1, high2, isspeculative, ctx, set_high)?;
                }
                let p1 = self.highs.get(&id1).and_then(|h| h.piece).unwrap();
                self.mark_intersection_dirty(p1);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::address::Address;
    use kuna_base::space::{spacetype, AddrSpace};
    use slotmap::KeyData;

    fn vid(n: u64) -> VarnodeId {
        VarnodeId::from(KeyData::from_ffi(n))
    }

    /// A simple in-memory HighContext over a fixed set of varnode views.
    struct MapCtx {
        views: BTreeMap<u64, VarnodeView>,
        covers: BTreeMap<u64, Cover>,
    }
    impl MapCtx {
        fn key(vn: VarnodeId) -> u64 {
            use slotmap::Key;
            vn.data().as_ffi()
        }
    }
    impl HighContext for MapCtx {
        fn vn_view(&self, vn: VarnodeId) -> VarnodeView {
            self.views.get(&MapCtx::key(vn)).expect("view").clone()
        }
        fn vn_cover(&self, vn: VarnodeId) -> Option<Cover> {
            self.covers.get(&MapCtx::key(vn)).cloned()
        }
        fn vn_has_cover(&self, vn: VarnodeId) -> bool {
            // has cover if insert & !constant & !annotation
            let fl = self.views.get(&MapCtx::key(vn)).map(|v| v.flags).unwrap_or(0);
            (fl & (varnode_flags::constant | varnode_flags::annotation | varnode_flags::insert))
                == varnode_flags::insert
        }
        fn vn_name_view(&self, vn: VarnodeId) -> CompareNameView {
            let v = self.vn_view(vn);
            CompareNameView {
                name_lock: v.flags & varnode_flags::namelock != 0,
                unaffected: v.flags & varnode_flags::unaffected != 0,
                persist: v.flags & varnode_flags::persist != 0,
                input: v.flags & varnode_flags::input != 0,
                addr_tied: v.flags & varnode_flags::addrtied != 0,
                proto_partial: v.flags & varnode_flags::proto_partial != 0,
                space_internal: v.space_internal,
                written: v.written,
                def_time: v.def_time,
            }
        }
        fn vn_loc_view(&self, _vn: VarnodeId) -> VarnodeViewLoc {
            // The membership tests in this module's unit tests don't exercise the
            // address-ordered insert/remove; a placeholder invalid address keeps
            // the trait satisfied (the dedicated compare_just_loc test builds real
            // addresses directly).
            VarnodeViewLoc { addr: kuna_base::address::Address::new_invalid() }
        }
    }

    fn dt(size: int4, meta: type_metatype) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, meta))
    }

    fn view(flags: uint4, ty: Rc<Datatype>, type_lock: bool) -> VarnodeView {
        VarnodeView {
            flags,
            size: ty.get_size(),
            type_: ty,
            type_lock,
            merge_group: 0,
            written: (flags & varnode_flags::written) != 0,
            def_time: 0,
            space_internal: false,
            create_index: 0,
        }
    }

    #[test]
    fn highvariable_construction_dirty() {
        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        let h = bank.get(id).unwrap();
        assert_eq!(h.num_instances(), 1);
        assert_eq!(h.get_instance(0), vid(1));
        assert_eq!(h.get_num_merge_classes(), 1);
        // all four dirty bits set at construction
        assert!(h.is_cover_dirty());
        assert_ne!(h.highflags & high_flags::flagsdirty, 0);
        assert_ne!(h.highflags & high_flags::namerepdirty, 0);
        assert_ne!(h.highflags & high_flags::typedirty, 0);
    }

    #[test]
    fn update_flags_inherits_member_props() {
        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        let mut views = BTreeMap::new();
        // member is input + addrtied + mapped + directwrite (directwrite must be
        // masked OUT by updateFlags)
        views.insert(
            MapCtx::key(vid(1)),
            view(
                varnode_flags::input
                    | varnode_flags::addrtied
                    | varnode_flags::mapped
                    | varnode_flags::directwrite
                    | varnode_flags::insert,
                dt(4, type_metatype::TYPE_UNKNOWN),
                false,
            ),
        );
        let ctx = MapCtx { views, covers: BTreeMap::new() };
        let h = bank.get_mut(id).unwrap();
        assert!(h.is_input(&ctx));
        assert!(h.is_addr_tied(&ctx));
        assert!(h.is_mapped(&ctx));
        // directwrite is masked out of the high flags
        assert_eq!(h.flags & varnode_flags::directwrite, 0);
        // flagsdirty cleared after updateFlags
        assert_eq!(h.highflags & high_flags::flagsdirty, 0);
    }

    #[test]
    fn update_type_picks_most_specialized() {
        // Two members: an UNKNOWN(4) and an INT(4). INT is more specialized
        // (lower typeOrderFormal), so it should win.
        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        // add second instance directly for the test
        bank.get_mut(id).unwrap().inst.push(vid(2));
        bank.get_mut(id).unwrap().highflags |= high_flags::typedirty;

        let mut views = BTreeMap::new();
        views.insert(
            MapCtx::key(vid(1)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_UNKNOWN), false),
        );
        views.insert(
            MapCtx::key(vid(2)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_INT), false),
        );
        let ctx = MapCtx { views, covers: BTreeMap::new() };
        let h = bank.get_mut(id).unwrap();
        let ty = h.get_type(&ctx, None);
        assert_eq!(ty.get_metatype(), type_metatype::TYPE_INT);
        // typedirty cleared
        assert_eq!(h.highflags & high_flags::typedirty, 0);
    }

    #[test]
    fn update_type_typelock_wins() {
        // A type-locked UNKNOWN beats an unlocked INT (typelock preferred).
        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        bank.get_mut(id).unwrap().inst.push(vid(2));
        bank.get_mut(id).unwrap().highflags |= high_flags::typedirty;
        let mut views = BTreeMap::new();
        views.insert(
            MapCtx::key(vid(1)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_INT), false),
        );
        views.insert(
            MapCtx::key(vid(2)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_UNKNOWN), true),
        );
        let ctx = MapCtx { views, covers: BTreeMap::new() };
        let h = bank.get_mut(id).unwrap();
        // representative is the typelocked one (vid 2, UNKNOWN)
        let rep = h.get_type_representative(&ctx);
        assert_eq!(rep, vid(2));
        assert!(h.is_type_lock(&ctx, None));
    }

    #[test]
    fn compare_name_prefers_input_over_temp() {
        let v1 = CompareNameView {
            name_lock: false,
            unaffected: false,
            persist: false,
            input: false,
            addr_tied: false,
            proto_partial: false,
            space_internal: true,
            written: true,
            def_time: 5,
        };
        let v2 = CompareNameView { input: true, space_internal: false, ..v1 };
        // vn2 is an input -> overrides
        assert!(HighVariable::compare_name(&v1, &v2));
        // namelock short-circuits
        let v1l = CompareNameView { name_lock: true, ..v1 };
        assert!(!HighVariable::compare_name(&v1l, &v2));
    }

    #[test]
    fn compare_name_earliest_def_wins() {
        let base = CompareNameView {
            name_lock: false,
            unaffected: false,
            persist: false,
            input: false,
            addr_tied: false,
            proto_partial: false,
            space_internal: false,
            written: true,
            def_time: 0,
        };
        let v1 = CompareNameView { def_time: 10, ..base };
        let v2 = CompareNameView { def_time: 3, ..base };
        // vn2 has earlier def (3 < 10) -> overrides
        assert!(HighVariable::compare_name(&v1, &v2));
        let v3 = CompareNameView { def_time: 20, ..base };
        // vn3 is later -> does not override
        assert!(!HighVariable::compare_name(&v1, &v3));
    }

    #[test]
    fn cover_dirty_propagation_no_piece() {
        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        // clear cover dirty first
        bank.get_mut(id).unwrap().highflags &= !high_flags::coverdirty;
        assert!(!bank.get(id).unwrap().is_cover_dirty());
        bank.cover_dirty(id);
        assert!(bank.get(id).unwrap().is_cover_dirty());
    }

    #[test]
    fn internal_cover_unions_members() {
        // Build a HighVariable with two members: member 1 has a def-point in
        // block 0, member 2 has a def-point in block 1.  The internal cover is
        // the union -> both blocks present.
        use crate::cover::{Cover, CoverPoint};

        let mut bank = HighVariableBank::new();
        let id = bank.new_high(vid(1));
        bank.get_mut(id).unwrap().inst.push(vid(2));
        bank.get_mut(id).unwrap().highflags |= high_flags::coverdirty;

        let op_pt = |u: u64| CoverPoint::Op {
            id: crate::context::OpId::from(KeyData::from_ffi(u + 1)),
            uindex: u as kuna_base::types::uintm,
            code: kuna_num::opcodes::OpCode::CPUI_COPY,
        };
        let mut c1 = Cover::new();
        c1.add_def_point(Some((0, op_pt(5))), false);
        let mut c2 = Cover::new();
        c2.add_def_point(Some((1, op_pt(9))), false);

        let mut covers = BTreeMap::new();
        covers.insert(MapCtx::key(vid(1)), c1);
        covers.insert(MapCtx::key(vid(2)), c2);
        let mut views = BTreeMap::new();
        views.insert(
            MapCtx::key(vid(1)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_UNKNOWN), false),
        );
        views.insert(
            MapCtx::key(vid(2)),
            view(varnode_flags::insert, dt(4, type_metatype::TYPE_UNKNOWN), false),
        );
        let ctx = MapCtx { views, covers };
        bank.get_mut(id).unwrap().update_internal_cover(&ctx);
        // both def-points carried into the internal cover
        let h = bank.get(id).unwrap();
        assert!(!h.internal_cover().get_cover_block(0).empty());
        assert!(!h.internal_cover().get_cover_block(1).empty());
        // coverdirty cleared
        assert_eq!(h.highflags & high_flags::coverdirty, 0);
    }

    #[test]
    fn high_edge_order_and_cache() {
        let populate = || Vec::new();
        let affects = |_: crate::context::OpId, _: &kuna_base::address::Address| false;
        let opset = PcodeOpSet::new(Box::new(populate), Box::new(affects));
        let mut test = HighIntersectTest::new(opset);
        let a = HighVariableId(1);
        let b = HighVariableId(2);
        assert_eq!(test.cached(a, b), None);
        test.cache_result(a, b, true);
        assert_eq!(test.cached(a, b), Some(true));
        assert_eq!(test.cached(b, a), Some(true)); // symmetric
                                                    // purge removes both directions
        test.purge_high(a);
        assert_eq!(test.cached(a, b), None);
        assert_eq!(test.cached(b, a), None);
    }

    #[test]
    fn move_intersect_tests_transfers_yes() {
        let populate = || Vec::new();
        let affects = |_: crate::context::OpId, _: &kuna_base::address::Address| false;
        let opset = PcodeOpSet::new(Box::new(populate), Box::new(affects));
        let mut test = HighIntersectTest::new(opset);
        let high1 = HighVariableId(10);
        let high2 = HighVariableId(20);
        let other = HighVariableId(30);
        // high2 intersects other (true)
        test.cache_result(high2, other, true);
        // marks: simple set
        let mut marks: BTreeSet<HighVariableId> = BTreeSet::new();
        let marked = std::cell::RefCell::new(BTreeSet::<HighVariableId>::new());
        {
            let mut set_mark = |h: HighVariableId| {
                marked.borrow_mut().insert(h);
            };
            let mut clear_mark = |h: HighVariableId| {
                marked.borrow_mut().remove(&h);
            };
            let is_mark = |h: HighVariableId| marked.borrow().contains(&h);
            test.move_intersect_tests(high1, high2, &mut set_mark, &mut clear_mark, &is_mark);
        }
        let _ = &mut marks;
        // high2's true test with `other` is reassigned to high1
        assert_eq!(test.cached(high1, other), Some(true));
        assert_eq!(test.cached(other, high1), Some(true));
        // high2's tests purged
        assert_eq!(test.cached(high2, other), None);
    }

    #[test]
    fn make_varnode_view_loc_orders() {
        // compareJustLoc orders purely by address
        let spc = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            0,
            0,
            0,
        ));
        let a = VarnodeViewLoc { addr: Address::new(Rc::clone(&spc), 0x10) };
        let b = VarnodeViewLoc { addr: Address::new(Rc::clone(&spc), 0x20) };
        assert!(HighVariable::compare_just_loc(&a, &b));
        assert!(!HighVariable::compare_just_loc(&b, &a));
    }
}
