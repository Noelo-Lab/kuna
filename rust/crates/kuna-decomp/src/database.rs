//! Port of `decompiler/cpp/database.{cc,hh}` (W4): the decompiler symbol table.
//!
//! This is the main symbol database: [`Symbol`] objects (and the
//! [`FunctionSymbol`]/[`EquateSymbol`]/[`UnionFacetSymbol`]/[`LabSymbol`]/
//! [`ExternRefSymbol`] specializations) live in named [`Scope`]s organized into
//! a namespace hierarchy under a [`Database`].  Symbols are mapped to storage by
//! [`SymbolEntry`] objects that ride on kuna-base's `rangemap` ([`EntryMap`],
//! one per address space) keyed by storage offset and sub-sorted by *use-limit*
//! (the C++ `EntrySubsort`).  Code can look a Symbol up by name
//! ([`Scope::query_by_name`]), by storage address ([`Scope::query_by_addr`]),
//! containing range ([`Scope::query_container`]), or boolean properties
//! ([`Scope::query_properties`]); each query walks the scope hierarchy through
//! the parent relationship in the exact order the C++ `stack*` helpers do.
//!
//! ## Transcription model (ADR 0001/0002)
//!
//! The C++ database is a graph of raw pointers (`Symbol *`, `Scope *`) with
//! `std::list<SymbolEntry>::iterator` handles.  This port keeps every object in
//! a [`Database`]-owned arena and replaces pointers with generational ids:
//!
//!   - [`SymbolId`] keys the per-scope symbol arena (the C++ `nametree`, a
//!     `set<Symbol *,SymbolCompareName>`, becomes a `BTreeSet<NameKey>` whose
//!     comparator transcribes `SymbolCompareName` — name then `nameDedup`).
//!   - [`ScopeId`] keys the [`Database`] scope arena (the C++ `ScopeMap`,
//!     `map<uint8,Scope *>`, becomes a `BTreeMap<u8 id,ScopeId>` per parent).
//!   - [`EntryId`] keys a SymbolEntry inside its [`EntryMap`]/dynamic list; the
//!     C++ `list<SymbolEntry>::iterator` stored in `Symbol::mapentry` becomes an
//!     [`EntryRef`] (which map + which handle).
//!
//! ## Seams (W4/W5/W6 — this porter owns only `database.rs`)
//!
//! `database.cc` reaches deep into subsystems this porter does not own:
//! `Architecture` (`glb`), the `TypeFactory` (`glb->types`), `Funcdata`,
//! `Varnode`, `Translate` (register names), and `JoinRecord`.  Per the SEAM
//! rule, these are satisfied by **local seam traits** defined here
//! ([`DatabaseArch`], [`TypeFactorySeam`], [`TranslateSeam`]) plus the
//! [`crate::dtype::Datatype`] seam; the W4 architecture porter / W6 type porter
//! implement them in their own modules.  Where a path needs a fully-wired
//! subsystem that no seam can stand in for (the `Funcdata`/`Varnode`-dependent
//! `buildDefaultName` Varnode arm, `getSizedType`, `updateType`), the method
//! takes the dependency as an explicit argument or returns a seam-noted result.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::crc32::crc_update;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::partmap::PartMap;
use kuna_base::rangemap::{RangeMap, RangeRecord, RecordIdx, Subsort};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uint8, uintb};

use crate::dtype::{type_metatype, Datatype};

// Varnode boolean property bits live in `varnode.rs` (W3); the Symbol/SymbolEntry
// flag words are exactly the same bit space (`database.hh` documents the
// `typelock,namelock,readonly,addrtied,persist,...` overlap).
use crate::varnode::varnode_flags;

// ===========================================================================
// Local seams (W4/W5/W6) — see module docs.
// ===========================================================================

/// Seam for the `TypeFactory` (`glb->types`, C++ `type.hh`, W6).
///
/// SEAM(W6): `database.cc` reaches `glb->types` to fabricate placeholder
/// data-types for symbols (`getBase`, `getTypeCode`, `getExactPiece`).  Only the
/// thin slice the database touches is declared here; W6 implements it on the
/// real `TypeFactory`.
pub trait TypeFactorySeam {
    /// C++ `TypeFactory::getBase(size,metatype)` — a base data-type of the given
    /// size and meta-type.
    fn get_base(&self, size: int4, meta: type_metatype) -> Rc<Datatype>;
    /// C++ `TypeFactory::getTypeCode()` — the "code" placeholder data-type used
    /// for function symbols.
    fn get_type_code(&self) -> Rc<Datatype>;
}

/// Seam for the `Translate`/`Architecture` register-name lookup
/// (`glb->translate->getRegisterName`, W5).
///
/// SEAM(W5): `buildVariableName` and the kuna storage-comment helper render
/// register-backed storage by name.  W5 routes this onto the real `Translate`.
pub trait TranslateSeam {
    /// C++ `Translate::getRegisterName(spc,off,sz)` — the register name covering
    /// the given storage, or empty if none.
    fn get_register_name(&self, space: &Rc<AddrSpace>, off: uintb, size: int4) -> String;
}

/// Seam for the slice of `Architecture` (C++ `glb`) the symbol database needs.
///
/// SEAM(W4/W5): the full `Architecture` is a separate W4 item (owned by the
/// fw-architecture serial porter).  `database.cc` reaches it for: the address
/// space count (`numSpaces`), the type factory (`types`), the translator
/// (`translate`), the minimum function-symbol size (`min_funcsymbol_size`), and
/// the kuna angr-naming toggle (`name_style_angr`).  This local seam carries
/// exactly that slice so `database.rs` compiles and tests standalone; the
/// architecture porter implements it on the real `Architecture`.
pub trait DatabaseArch {
    /// Number of address spaces (C++ `AddrSpaceManager::numSpaces`).
    fn num_spaces(&self) -> int4;
    /// Borrow the type factory (C++ `Architecture::types`).  // SEAM(W6)
    fn types(&self) -> &dyn TypeFactorySeam;
    /// Borrow the translator (C++ `Architecture::translate`).  // SEAM(W5)
    fn translate(&self) -> &dyn TranslateSeam;
    /// Minimum bytes a FunctionSymbol map consumes (C++
    /// `Architecture::min_funcsymbol_size`).
    fn min_funcsymbol_size(&self) -> int4;
    /// C++ (kuna) `Architecture::name_style_angr` — is angr-style naming active?
    /// SEAM(W4-p0pack): namestyle.  Default angr (per the `settableTable`
    /// registration in `kuna_stages.cc`); the architecture porter wires this to
    /// the `option namestyle` toggle.
    fn name_style_angr(&self) -> bool;
    /// C++ `Datatype::printNameBase` (`type.hh:286`): the (recursive) data-type
    /// name prefix `buildVariableName` prepends in the ghidra naming style
    /// (`iVar`/`<type>Ram`, etc).  SEAM(W6): the real per-type renderer lives in
    /// the type subsystem; the architecture seam forwards to it.  The base
    /// renderer is "first char of the type name", empty for a nameless type.
    fn type_name_base(&self, dt: &Datatype) -> String;
}

// ===========================================================================
// Kuna angr-style naming (DIV-5; port of decompiler/cpp/kuna_naming.cc).
// ===========================================================================
//
// This is a kuna-owned policy file in the C++ tree; its address-rendering
// helpers are pure and self-contained, so they are ported inline here (the
// database is their only consumer at the boundary this porter owns).

/// Render an address as lowercase hex with no leading zeros and no "0x" prefix
/// (C++ `kunaPrintBareAddr`): the angr address form used by `dat_`/`sub_`/`label_`.
fn kuna_bare_addr(addr: &Address) -> String {
    let space = addr.get_space().expect("kuna_bare_addr on invalid address");
    // byteToAddress(off, wordsize): the word-addressed offset.
    let off = AddrSpace::byte_to_address(addr.get_offset(), space.get_word_size());
    format!("{off:x}")
}

/// C++ `kunaGlobalDataName`: `dat_<addr>`.
pub fn kuna_global_data_name(addr: &Address) -> String {
    format!("dat_{}", kuna_bare_addr(addr))
}

/// C++ `kunaFunctionName`: `sub_<addr>`.
pub fn kuna_function_name(addr: &Address) -> String {
    format!("sub_{}", kuna_bare_addr(addr))
}

/// C++ `kunaLabelName`: `label_<addr>`.
pub fn kuna_label_name(addr: &Address) -> String {
    format!("label_{}", kuna_bare_addr(addr))
}

/// C++ `kunaArgName`: `a<catindex>` (0-indexed; negative clamps to 0).
pub fn kuna_arg_name(mut catindex: int4) -> String {
    if catindex < 0 {
        catindex = 0;
    }
    format!("a{catindex}")
}

// ===========================================================================
// SymbolEntry  (database.hh:75-164)
// ===========================================================================

/// Class for sub-sorting different SymbolEntry objects at the same address.
///
/// Transcription of the C++ `SymbolEntry::EntrySubsort` (`database.hh:107-134`):
/// pulled from the SymbolEntry `uselimit` (see [`SymbolEntry::get_subsort`]).
/// The derived lexicographic `Ord` is exactly `EntrySubsort::operator<` (compare
/// `useindex`, then `useoffset`).  The `Subsort` trait's `minimal`/`maximal`
/// transcribe the `EntrySubsort(bool)` constructor (`false` → earliest,
/// `true` → `useindex=0xffff` greater than any real index).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntrySubsort {
    /// Index of the sub-sorting address space (C++ `useindex`, an `int4`).
    pub useindex: int4,
    /// Offset into the sub-sorting address space (C++ `useoffset`, a `uintb`).
    pub useoffset: uintb,
}

impl EntrySubsort {
    /// C++ `EntrySubsort(const Address &addr)`.
    pub fn from_addr(addr: &Address) -> EntrySubsort {
        let space = addr.get_space().expect("EntrySubsort from invalid address");
        EntrySubsort { useindex: space.get_index(), useoffset: addr.get_offset() }
    }
}

impl Subsort for EntrySubsort {
    /// C++ `EntrySubsort(false)` — less than any real value.
    fn minimal() -> Self {
        EntrySubsort { useindex: 0, useoffset: 0 }
    }
    /// C++ `EntrySubsort(true)` — `useindex=0xffff`, greater than any real index.
    fn maximal() -> Self {
        EntrySubsort { useindex: 0xffff, useoffset: 0 }
    }
}

/// Initialization data for a (non-dynamic) [`SymbolEntry`] in a rangemap.
///
/// C++ `SymbolEntry::EntryInitData` (`database.hh:91-101`): all the raw pieces
/// except the main-address offset and size, which the rangemap supplies via the
/// `(inittype, linetype, linetype)` constructor.
#[derive(Debug, Clone)]
pub struct EntryInitData {
    /// The address space of the main SymbolEntry starting address.
    pub space: Rc<AddrSpace>,
    /// The symbol being mapped (arena id).
    pub symbol: SymbolId,
    /// Varnode flags specific to the storage location.
    pub extraflags: uint4,
    /// Starting offset of the portion of the Symbol being covered.
    pub offset: int4,
    /// The range of code addresses for which the storage is valid.
    pub uselimit: RangeList,
}

/// A storage location for a particular [`Symbol`] (C++ `SymbolEntry`,
/// `database.hh:75-163`).
///
/// Where a Symbol is stored (byte address + size) is of particular importance
/// to the decompiler; a Symbol split across multiple storage locations is
/// supported by the `offset`/`size` fields.  The `hash` field supports *dynamic*
/// storage (a constant or temporary register tied to specific p-code ops via the
/// DynamicHash mechanism, `addr` invalid).  `uselimit` defines the range of
/// instruction addresses over which a particular memory address represents this
/// Symbol (an empty `uselimit` ⇒ the storage holds the Symbol across all code).
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    /// Symbol object being mapped (arena id; C++ `symbol`).
    pub symbol: SymbolId,
    /// Varnode flags specific to this storage location (C++ `extraflags`).
    pub extraflags: uint4,
    /// Starting address of the storage location (C++ `addr`).
    pub addr: Address,
    /// A dynamic storage address, alternative to `addr` (C++ `hash`).
    pub hash: uint8,
    /// Offset into the Symbol that this covers (C++ `offset`).
    pub offset: int4,
    /// Number of bytes consumed by this (piece of) storage (C++ `size`).
    pub size: int4,
    /// Code address ranges where this storage is valid (C++ `uselimit`).
    pub uselimit: RangeList,
}

impl SymbolEntry {
    /// C++ `SymbolEntry(Symbol *sym)` — an unintegrated mapping with no address.
    pub fn new_unintegrated(sym: SymbolId) -> SymbolEntry {
        SymbolEntry {
            symbol: sym,
            extraflags: 0,
            addr: Address::new_invalid(),
            hash: 0,
            offset: 0,
            size: -1,
            uselimit: RangeList::new(),
        }
    }

    /// C++ `SymbolEntry(Symbol *,uint4,uint8,int4,int4,const RangeList &)` — a
    /// dynamic SymbolEntry (storage tied to a temporary register / constant).
    pub fn new_dynamic(
        sym: SymbolId,
        exfl: uint4,
        h: uint8,
        off: int4,
        sz: int4,
        rnglist: RangeList,
    ) -> SymbolEntry {
        SymbolEntry {
            symbol: sym,
            extraflags: exfl,
            addr: Address::new_invalid(),
            hash: h,
            offset: off,
            size: sz,
            uselimit: rnglist,
        }
    }

    /// Is this a high or low piece of the whole Symbol (C++ `isPiece`).
    pub fn is_piece(&self) -> bool {
        (self.extraflags & (varnode_flags::precislo | varnode_flags::precishi)) != 0
    }

    /// Is the storage *dynamic* (C++ `isDynamic`).
    pub fn is_dynamic(&self) -> bool {
        self.addr.is_invalid()
    }

    /// Is the storage *invalid* (C++ `isInvalid`).
    pub fn is_invalid(&self) -> bool {
        self.addr.is_invalid() && (self.hash == 0)
    }

    /// Offset of this within the Symbol (C++ `getOffset`).
    pub fn get_offset(&self) -> int4 {
        self.offset
    }

    /// First offset of this storage location (C++ `getFirst`).
    pub fn get_first(&self) -> uintb {
        self.addr.get_offset()
    }

    /// Last offset of this storage location (C++ `getLast`).
    ///
    /// Transcribes `addr.getOffset()+size-1` with `uintb` wraparound (the C++
    /// `uintb` arithmetic).
    pub fn get_last(&self) -> uintb {
        // C++: addr.getOffset() + size - 1 (uintb); size is int4 widened.
        self.addr
            .get_offset()
            .wrapping_add(self.size as uintb)
            .wrapping_sub(1)
    }

    /// Get the sub-sort object (C++ `getSubsort`).
    ///
    /// `database.cc:98-110`: a minimal subsort for an address-tied symbol;
    /// otherwise the first `uselimit` range's space index + first offset.  An
    /// empty `uselimit` on a non-tied symbol is a `LowlevelError`.
    pub fn get_subsort(&self, sym_flags: uint4) -> KunaResult<EntrySubsort> {
        let mut res = EntrySubsort::minimal(); // Minimal subsort
        if (sym_flags & varnode_flags::addrtied) == 0 {
            let range = self
                .uselimit
                .get_first_range()
                .ok_or_else(|| KunaError::lowlevel("Map entry with empty uselimit"))?;
            res.useindex = range.get_space().get_index();
            res.useoffset = range.get_first();
        }
        Ok(res)
    }

    /// Hash used to identify dynamic storage (C++ `getHash`).
    pub fn get_hash(&self) -> uint8 {
        self.hash
    }

    /// Number of bytes consumed by this storage (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }

    /// Starting address of this storage (C++ `getAddr`).
    pub fn get_addr(&self) -> &Address {
        &self.addr
    }

    /// The set of valid code addresses (C++ `getUseLimit`).
    pub fn get_use_limit(&self) -> &RangeList {
        &self.uselimit
    }

    /// Is this storage valid for the given code address (C++ `inUse`).
    ///
    /// `database.cc:115-121`.  Requires the owning Symbol's flags (for the
    /// address-tied short-circuit), passed explicitly since the Symbol lives in a
    /// separate arena.
    pub fn in_use(&self, usepoint: &Address, sym_flags: uint4) -> bool {
        if (sym_flags & varnode_flags::addrtied) != 0 {
            return true; // Valid throughout scope
        }
        if usepoint.is_invalid() {
            return false;
        }
        self.uselimit.in_range(usepoint, 1)
    }

    /// First code address where this storage is valid (C++ `getFirstUseAddress`).
    pub fn get_first_use_address(&self) -> Address {
        match self.uselimit.get_first_range() {
            None => Address::new_invalid(),
            Some(rng) => rng.get_first_addr(),
        }
    }

    /// Replace the range of valid code addresses (C++ `setUseLimit`).
    pub fn set_use_limit(&mut self, uselim: RangeList) {
        self.uselimit = uselim;
    }
}

/// A SymbolEntry record as it sits in a per-space [`RangeMap`] (C++
/// `typedef rangemap<SymbolEntry> EntryMap`).
///
/// The C++ rangemap stores `SymbolEntry` directly; the `(EntryInitData, a, b)`
/// constructor (`database.cc:85-94`) sets `addr = Address(space, a)`,
/// `size = (b-a)+1`, and copies the symbol/flags/offset/uselimit.  Because
/// kuna's [`RangeMap`] requires the subsort to be computed from the *record*
/// without external state, but the C++ `getSubsort` consults the owning Symbol's
/// `addrtied` flag, the record caches that flag (`addrtied_hint`) at insert time
/// (the flag is set on the Symbol before `addMap` runs — `database.cc:1131-1185`).
#[derive(Debug, Clone)]
pub struct EntryRecord {
    /// The wrapped SymbolEntry.
    pub entry: SymbolEntry,
    /// Cached `symbol->getFlags() & addrtied` for the subsort (see above).
    addrtied_hint: bool,
}

impl RangeRecord for EntryRecord {
    type LineType = uintb;
    type SubsortType = EntrySubsort;
    type InitType = (EntryInitData, bool); // (data, addrtied_hint)

    fn create(data: Self::InitType, a: uintb, b: uintb) -> EntryRecord {
        let (data, addrtied_hint) = data;
        // C++ SymbolEntry(const EntryInitData &data,uintb a,uintb b)
        let addr = Address::new(data.space, a);
        let size = (b.wrapping_sub(a)).wrapping_add(1) as int4;
        let entry = SymbolEntry {
            addr,
            size,
            symbol: data.symbol,
            extraflags: data.extraflags,
            offset: data.offset,
            uselimit: data.uselimit,
            hash: 0,
        };
        EntryRecord { entry, addrtied_hint }
    }

    fn get_first(&self) -> uintb {
        self.entry.get_first()
    }

    fn get_last(&self) -> uintb {
        self.entry.get_last()
    }

    fn get_subsort(&self) -> EntrySubsort {
        // C++ getSubsort consults symbol->getFlags()&addrtied; we cached it.
        let mut res = EntrySubsort::minimal();
        if !self.addrtied_hint {
            // Non-tied: derived from uselimit first range. Empty uselimit is a
            // C++ LowlevelError, but RangeRecord::get_subsort is infallible; we
            // mirror the C++ invariant that a non-tied entry always has a
            // non-empty uselimit by the time it is inserted (addMap sets
            // addrtied when uselimit is empty — database.cc:1154-1159).
            if let Some(range) = self.entry.uselimit.get_first_range() {
                res.useindex = range.get_space().get_index();
                res.useoffset = range.get_first();
            }
        }
        res
    }
}

/// A rangemap of [`SymbolEntry`] for one address space (C++ `EntryMap`).
pub type EntryMap = RangeMap<EntryRecord>;

// ===========================================================================
// Symbol  (database.hh:166-357)
// ===========================================================================

/// Display (dispflag) properties for a [`Symbol`] (C++ `Symbol` anon enum,
/// `database.hh:199-209`).
pub mod symbol_dispflags {
    use kuna_base::types::uint4;
    /// Force hexadecimal printing of constant symbol.
    pub const FORCE_HEX: uint4 = 1;
    /// Force decimal printing of constant symbol.
    pub const FORCE_DEC: uint4 = 2;
    /// Force octal printing of constant symbol.
    pub const FORCE_OCT: uint4 = 3;
    /// Force binary printing of constant symbol.
    pub const FORCE_BIN: uint4 = 4;
    /// Force integer to be printed as a character constant.
    pub const FORCE_CHAR: uint4 = 5;
    /// Only the size of the symbol is typelocked.
    pub const SIZE_TYPELOCK: uint4 = 8;
    /// Symbol should not speculatively merge automatically.
    pub const ISOLATE: uint4 = 16;
    /// Set if some SymbolEntrys did not get merged.
    pub const MERGE_PROBLEMS: uint4 = 32;
    /// We are the "this" symbol for a class method.
    pub const IS_THIS_PTR: uint4 = 64;
}

/// The possible specialized Symbol *categories* (C++ `Symbol` anon enum,
/// `database.hh:212-218`).
pub mod symbol_category {
    use kuna_base::types::int4;
    /// Symbol is not in a special category.
    pub const NO_CATEGORY: int4 = -1;
    /// The Symbol is a parameter to a function.
    pub const FUNCTION_PARAMETER: int4 = 0;
    /// The Symbol holds equate information about a constant.
    pub const EQUATE: int4 = 1;
    /// Symbol holding read or write facing union field information.
    pub const UNION_FACET: int4 = 2;
    /// Temporary placeholder for an input symbol prior to formalizing params.
    pub const FAKE_INPUT: int4 = 3;
}

/// Base of internal symbol ID's (C++ `Symbol::ID_BASE`, `database.cc:46`).
pub const SYMBOL_ID_BASE: uint8 = 0x4000_0000_0000_0000;

/// The subclass discriminant for a [`Symbol`] (C++ uses virtual dispatch /
/// `dynamic_cast`; the arena model uses an explicit tag in the same object).
#[derive(Debug, Clone)]
pub enum SymbolKind {
    /// A plain `Symbol` (C++ `Symbol`).
    Plain,
    /// `FunctionSymbol` — owns a function's meta-data (C++ `FunctionSymbol`).
    Function {
        /// Minimum bytes a SymbolEntry consumes (C++ `consumeSize`).
        consume_size: int4,
        /// Has an associated Funcdata been built?  SEAM(W5): the C++ owns a
        /// `Funcdata *fd`; the function-data subsystem is a later wave, so the
        /// arena stores only whether one exists.
        has_funcdata: bool,
        /// `FuncProto::is_inline` for this function (C++ `getFuncProto().isInline()`).
        /// The C++ stores this flag on the `FuncProto` owned by the FunctionSymbol's
        /// lazily-built `Funcdata`; with the `Funcdata`-on-symbol subsystem deferred
        /// (W5), the per-function inline property — set by `option inline <name>`
        /// (`OptionInline`) and read at flow time (`FlowInfo::queryCall`) — is parked
        /// here so the function's inline disposition survives between the two.
        inline_func: bool,
        /// `FuncProto::is_no_return` for this function (C++ `getFuncProto().isNoReturn()`).
        /// Set by `option noreturn <name>` (`OptionNoReturn`); parked here for the
        /// same reason as `inline_func`.
        no_return: bool,
        /// `FuncProto::injectid` for this function (C++ `getFuncProto().setInjectId(id)`):
        /// the call-fixup injection id applied by `fixup apply <fixup> <name>`
        /// (`IfcFixupApply`).  `-1` = no fixup.  Parked here for the same reason as
        /// `inline_func` (the lazily-built `Funcdata`/`FuncProto` is W5).
        inject_id: int4,
    },
    /// `EquateSymbol` — labels a constant (C++ `EquateSymbol`).
    Equate {
        /// Value of the constant being equated (C++ `value`).
        value: uintb,
    },
    /// `UnionFacetSymbol` — forces a union field interpretation (C++
    /// `UnionFacetSymbol`).
    UnionFacet {
        /// Particular field to associate (C++ `fieldNum`).
        field_num: int4,
        /// True if facet matches any PcodeOp at the address (C++ `addrBased`).
        addr_based: bool,
    },
    /// `LabSymbol` — labels code internal to a function (C++ `LabSymbol`).
    Lab,
    /// `ExternRefSymbol` — labels an external location (C++ `ExternRefSymbol`).
    ExternRef {
        /// The placeholder address for meta-data (C++ `refaddr`).
        refaddr: Address,
    },
}

/// A reference to a stored [`SymbolEntry`] — the arena analogue of the C++
/// `list<SymbolEntry>::iterator` kept in `Symbol::mapentry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryRef {
    /// An entry in the per-space rangemap (which space index, which record).
    Mapped {
        /// Address-space index into `ScopeInternal::maptable`.
        space_index: usize,
        /// Handle into that space's [`RangeMap`].
        idx: RecordIdx,
    },
    /// An entry in the dynamic-entry list (index into `ScopeInternal::dynamicentry`).
    Dynamic(usize),
}

/// The base class for a symbol in a symbol table or scope (C++ `Symbol`,
/// `database.hh:166-258`).
///
/// At its most basic, a Symbol is a `name` and a `type`.  It also knows its
/// `scope`, how it should be displayed (`dispflags`), and its `category`.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The scope that owns this symbol (C++ `scope`, an arena id).
    pub scope: ScopeId,
    /// The local name of the symbol (C++ `name`).
    pub name: String,
    /// Name to use when displaying symbol in output (C++ `displayName`).
    pub display_name: String,
    /// The symbol's data-type (C++ `type`).  `None` only transiently during
    /// decode-style construction (C++ `(Datatype *)0`).
    pub dtype: Option<Rc<Datatype>>,
    /// Id to distinguish symbols with the same name (C++ `nameDedup`).
    pub name_dedup: uint4,
    /// Varnode-like properties of the symbol (C++ `flags`).
    pub flags: uint4,
    /// Flags affecting the display of this symbol (C++ `dispflags`).
    pub dispflags: uint4,
    /// Special category (C++ `category`, an `int2`).
    pub category: int4,
    /// Index within category (C++ `catindex`, a `uint2`).
    pub catindex: u16,
    /// Unique id, 0 = unassigned (C++ `symbolId`).
    pub symbol_id: uint8,
    /// List of storage locations labeled with this Symbol (C++ `mapentry`).
    pub mapentry: Vec<EntryRef>,
    /// Number of SymbolEntries that map to the whole Symbol (C++ `wholeCount`).
    pub whole_count: uint4,
    /// The subclass-specific payload (C++ virtual subclass).
    pub kind: SymbolKind,
}

impl Symbol {
    /// C++ `Symbol(Scope *sc,const string &nm,Datatype *ct)` (`database.hh:963`).
    pub fn new(scope: ScopeId, nm: &str, ct: Option<Rc<Datatype>>) -> Symbol {
        Symbol {
            scope,
            name: nm.to_string(),
            display_name: nm.to_string(),
            dtype: ct,
            name_dedup: 0,
            flags: 0,
            dispflags: 0,
            category: symbol_category::NO_CATEGORY,
            catindex: 0,
            symbol_id: 0,
            mapentry: Vec::new(),
            whole_count: 0,
            kind: SymbolKind::Plain,
        }
    }

    /// C++ `Symbol(Scope *sc)` (`database.hh:982`) — for use with decode.
    pub fn new_empty(scope: ScopeId) -> Symbol {
        Symbol {
            scope,
            name: String::new(),
            display_name: String::new(),
            dtype: None,
            name_dedup: 0,
            flags: 0,
            dispflags: 0,
            category: symbol_category::NO_CATEGORY,
            catindex: 0,
            symbol_id: 0,
            mapentry: Vec::new(),
            whole_count: 0,
            kind: SymbolKind::Plain,
        }
    }

    /// Get the local name of the symbol (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the name to display in output (C++ `getDisplayName`).
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Get a unique id for the symbol (C++ `getId`).
    pub fn get_id(&self) -> uint8 {
        self.symbol_id
    }

    /// Get the boolean properties of the Symbol (C++ `getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }

    /// Get the format to display the Symbol in (C++ `getDisplayFormat`).
    pub fn get_display_format(&self) -> uint4 {
        self.dispflags & 7
    }

    /// Get the Symbol category (C++ `getCategory`).
    pub fn get_category(&self) -> int4 {
        self.category
    }

    /// Get the position of the Symbol within its category (C++ `getCategoryIndex`).
    pub fn get_category_index(&self) -> u16 {
        self.catindex
    }

    /// Is the Symbol type-locked (C++ `isTypeLocked`).
    pub fn is_type_locked(&self) -> bool {
        (self.flags & varnode_flags::typelock) != 0
    }

    /// Is the Symbol name-locked (C++ `isNameLocked`).
    pub fn is_name_locked(&self) -> bool {
        (self.flags & varnode_flags::namelock) != 0
    }

    /// Is the Symbol size type-locked (C++ `isSizeTypeLocked`).
    pub fn is_size_type_locked(&self) -> bool {
        (self.dispflags & symbol_dispflags::SIZE_TYPELOCK) != 0
    }

    /// Is the Symbol volatile (C++ `isVolatile`).
    pub fn is_volatile(&self) -> bool {
        (self.flags & varnode_flags::volatil) != 0
    }

    /// Is this the "this" pointer (C++ `isThisPointer`).
    pub fn is_this_pointer(&self) -> bool {
        (self.dispflags & symbol_dispflags::IS_THIS_PTR) != 0
    }

    /// Does this have an undefined name (C++ `isNameUndefined`,
    /// `database.cc:247-251`)?  True iff the name is exactly 15 chars and starts
    /// with `$$undef`.
    pub fn is_name_undefined(&self) -> bool {
        self.name.len() == 15 && self.name.starts_with("$$undef")
    }

    /// Does this have more than one entire mapping (C++ `isMultiEntry`).
    pub fn is_multi_entry(&self) -> bool {
        self.whole_count > 1
    }

    /// Return true if this is isolated from speculative merging (C++ `isIsolated`).
    pub fn is_isolated(&self) -> bool {
        (self.dispflags & symbol_dispflags::ISOLATE) != 0
    }

    /// Return the number of SymbolEntrys (C++ `numEntries`).
    pub fn num_entries(&self) -> int4 {
        self.mapentry.len() as int4
    }

    /// Force a specific display format for constant symbols (C++
    /// `setDisplayFormat`, `database.hh:262`).
    pub fn set_display_format(&mut self, val: uint4) {
        self.dispflags &= 0xffff_fff8;
        self.dispflags |= val;
    }

    /// Toggle whether this is the "this" pointer (C++ `setThisPointer`,
    /// `database.cc:236-243`).
    pub fn set_this_pointer(&mut self, val: bool) {
        if val {
            self.dispflags |= symbol_dispflags::IS_THIS_PTR;
        } else {
            self.dispflags &= !symbol_dispflags::IS_THIS_PTR;
        }
    }

    /// Calculate if the `size_typelock` property is on (C++ `checkSizeTypeLock`,
    /// `database.cc:227-233`).
    pub fn check_size_type_lock(&mut self) {
        self.dispflags &= !symbol_dispflags::SIZE_TYPELOCK;
        if self.is_type_locked() {
            if let Some(t) = &self.dtype {
                if t.get_metatype() == type_metatype::TYPE_UNKNOWN {
                    self.dispflags |= symbol_dispflags::SIZE_TYPELOCK;
                }
            }
        }
    }

    /// Set whether this Symbol should be speculatively merged (C++ `setIsolated`,
    /// `database.cc:256-266`).
    pub fn set_isolated(&mut self, val: bool) {
        if val {
            self.dispflags |= symbol_dispflags::ISOLATE;
            self.flags |= varnode_flags::typelock; // Isolated Symbol must be typelocked
            self.check_size_type_lock();
        } else {
            self.dispflags &= !symbol_dispflags::ISOLATE;
        }
    }

    /// Get number of bytes consumed within the address→symbol map (C++
    /// `getBytesConsumed`).  For most symbols this is the data-type size; a
    /// FunctionSymbol overrides it with `consumeSize`.
    pub fn get_bytes_consumed(&self) -> int4 {
        match &self.kind {
            SymbolKind::Function { consume_size, .. } => *consume_size,
            _ => self.dtype.as_ref().map(|t| t.get_size()).unwrap_or(0),
        }
    }
}

// EquateSymbol::isValueClose (database.cc:641-658) — pure, ported standalone.

/// C++ `EquateSymbol::isValueClose` (`database.cc:641-658`): is the given value
/// a transform (negation, two's-complement, ±1) of the equate's value?
pub fn equate_is_value_close(value: uintb, op2_value: uintb, size: int4) -> bool {
    use kuna_base::address::{calc_mask, sign_extend_sized};
    if value == op2_value {
        return true;
    }
    let mask = calc_mask(size);
    let mask_value = value & mask;
    if mask_value != value {
        // If '1' bits are getting masked off, make sure only sign-extension is.
        // C++ value != sign_extend(maskValue,size,sizeof(uintb))
        if value != sign_extend_sized(mask_value, size, 8) {
            return false;
        }
    }
    if mask_value == (op2_value & mask) {
        return true;
    }
    if mask_value == ((!op2_value) & mask) {
        return true;
    }
    // -op2Value with uintb wraparound
    if mask_value == (op2_value.wrapping_neg() & mask) {
        return true;
    }
    if mask_value == (op2_value.wrapping_add(1) & mask) {
        return true;
    }
    if mask_value == (op2_value.wrapping_sub(1) & mask) {
        return true;
    }
    false
}

// ===========================================================================
// Scope arena id newtypes
// ===========================================================================

slotmap::new_key_type! {
    /// Arena key for a [`Symbol`] (replaces the C++ `Symbol *`).
    pub struct SymbolId;
    /// Arena key for a [`Scope`] (replaces the C++ `Scope *`).
    pub struct ScopeId;
}

/// The ordering key used by the C++ `SymbolNameTree`
/// (`set<Symbol *,SymbolCompareName>`): name then `nameDedup`.
///
/// `SymbolCompareName::operator()` (`database.hh:360-373`) compares the symbol
/// names with `string::compare`, breaking ties by `nameDedup`.  We materialize
/// the (name, dedup) pair in the key so a `BTreeSet` can transcribe the order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct NameKey {
    name: String,
    name_dedup: uint4,
}

// ===========================================================================
// Scope  (database.hh:464-870, merging Scope + ScopeInternal)
// ===========================================================================
//
// The C++ has an abstract `Scope` base and a concrete `ScopeInternal`; kuna only
// ever instantiates `ScopeInternal` (`buildSubScope` returns `new
// ScopeInternal`).  The arena model merges the two into one `Scope` struct
// holding both the base members (rangetree, parent/children, glb/name/fd/
// uniqueId) and the ScopeInternal members (nametree, maptable, category,
// dynamicentry, multiEntrySet, nextUniqueId).  Methods that the C++ split across
// base/derived live on `Database` (which owns the scope and symbol arenas) when
// they touch shared state, or on `Scope` when they are self-contained.

/// A collection of [`Symbol`] objects within a single (namespace or functional)
/// scope (C++ `Scope`/`ScopeInternal`, `database.hh:464-870`).
///
/// (No `Debug` derive: the per-space [`EntryMap`] rangemaps do not implement
/// `Debug`, matching the C++ container, which also has no debug print.)
pub struct Scope {
    // --- C++ Scope base members ---
    /// Range of data addresses owned by this scope (C++ `rangetree`).
    pub rangetree: RangeList,
    /// The parent scope (C++ `parent`; `None` for the global scope).
    pub parent: Option<ScopeId>,
    /// Sorted list of child scopes, keyed by child uniqueId (C++ `children`,
    /// a `ScopeMap = map<uint8,Scope *>`).
    pub children: BTreeMap<uint8, ScopeId>,
    /// Name of this scope (C++ `name`).
    pub name: String,
    /// Name to display in output (C++ `displayName`).
    pub display_name: String,
    /// True if this is the local scope for a function (C++ `fd != null`).
    ///
    /// SEAM(W5): the C++ holds a `Funcdata *fd`; the function-data subsystem is a
    /// later wave.  The arena tracks only whether this scope is functional
    /// (`isGlobal() == !is_functional`), which is all `database.cc` reads of `fd`
    /// (the `clearResolve`/`fillResolve` "does not apply to functional scopes"
    /// guard, and `isGlobal`).
    pub is_functional: bool,
    /// Unique id for the scope (C++ `uniqueId`).
    pub unique_id: uint8,

    // --- C++ ScopeInternal members ---
    /// The set of Symbol ids, sorted by name (C++ `nametree`).
    nametree: BTreeMap<NameKey, SymbolId>,
    /// Rangemaps of SymbolEntry, one per address space (C++ `maptable`).
    /// `None` for an unallocated space (the C++ `(EntryMap *)0` sentinel).
    maptable: Vec<Option<EntryMap>>,
    /// References to Symbol ids organized by category (C++ `category`).
    /// `None` is the C++ `(Symbol *)0` placeholder slot.
    category: Vec<Vec<Option<SymbolId>>>,
    /// Dynamic symbol entries (C++ `dynamicentry`).  Slot-based: `None` is an
    /// erased entry (the C++ erases from a `std::list`; ids stay stable).
    dynamicentry: Vec<Option<SymbolEntry>>,
    /// Set of symbols with multiple entries (C++ `multiEntrySet`, sorted by name).
    multi_entry_set: BTreeMap<NameKey, SymbolId>,
    /// Next available symbol id within this scope (C++ `nextUniqueId`).
    next_unique_id: uint8,
}

impl Scope {
    /// C++ `Scope(uint8 id,const string &nm,Architecture *g,Scope *own)` plus the
    /// `ScopeInternal` constructor (`maptable.resize(numSpaces, null)`).
    fn new(id: uint8, nm: &str, num_spaces: int4) -> Scope {
        let n = if num_spaces < 0 { 0 } else { num_spaces as usize };
        Scope {
            rangetree: RangeList::new(),
            parent: None,
            children: BTreeMap::new(),
            name: nm.to_string(),
            display_name: nm.to_string(),
            is_functional: false,
            unique_id: id,
            nametree: BTreeMap::new(),
            maptable: (0..n).map(|_| None).collect(),
            category: Vec::new(),
            dynamicentry: Vec::new(),
            multi_entry_set: BTreeMap::new(),
            next_unique_id: 0,
        }
    }

    /// Get the name of the Scope (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get name displayed in output (C++ `getDisplayName`).
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Get the globally unique id (C++ `getId`).
    pub fn get_id(&self) -> uint8 {
        self.unique_id
    }

    /// Return true if this scope is global (C++ `isGlobal`: `fd == null`).
    pub fn is_global(&self) -> bool {
        !self.is_functional
    }

    /// Get the parent Scope (C++ `getParent`).
    pub fn get_parent(&self) -> Option<ScopeId> {
        self.parent
    }

    /// Access the address ranges owned by this Scope (C++ `getRangeTree`).
    pub fn get_range_tree(&self) -> &RangeList {
        &self.rangetree
    }

    /// Change name displayed in output (C++ `setDisplayName`).
    pub fn set_display_name(&mut self, nm: &str) {
        self.display_name = nm.to_string();
    }

    /// Query if the given range is owned by this Scope (C++ `inScope`).
    ///
    /// `database.hh:599-600`: all bytes in the range must be in `rangetree`.
    pub fn in_scope(&self, addr: &Address, size: int4, _usepoint: &Address) -> bool {
        self.rangetree.in_range(addr, size)
    }

    /// C++ `Scope::hashScopeName` (`database.cc:885-900`): a globally unique id
    /// from a scope name and its parent's id, via the CRC32 update primitive.
    pub fn hash_scope_name(base_id: uint8, nm: &str) -> uint8 {
        let mut reg1 = (base_id >> 32) as uint4;
        let mut reg2 = base_id as uint4;
        reg1 = crc_update(reg1, 0xa9);
        reg2 = crc_update(reg2, reg1);
        for b in nm.bytes() {
            // C++ `uint4 val = nm[i];` — char promoted; bytes are 0..=255.
            let val = b as uint4;
            reg1 = crc_update(reg1, val);
            reg2 = crc_update(reg2, reg1);
        }
        let mut res = reg1 as uint8;
        res = (res << 32) | (reg2 as uint8);
        res
    }
}


// ===========================================================================
// ScopeResolve  (database.hh:877-903): address -> owning namespace Scope.
// ===========================================================================
//
// The C++ `ScopeResolve` is a `rangemap<ScopeMapper>` with `linetype = Address`
// and a `NullSubsort` (no sub-sorting of overlapping ranges).  kuna's `RangeMap`
// keys on integer `Line` types, and `Address` is a W1 type this porter does not
// own, so the resolve-map is transcribed directly as a sorted set of
// (first,last,scope) ranges.  `find(addr)` returns the first range that contains
// `addr` (the C++ `resolvemap.find(addr).first` semantics that `mapScope` reads).
#[derive(Debug, Default)]
struct ScopeResolve {
    /// Ranges keyed by their first address; value is (last, scope).  Namespace
    /// ranges in this map are disjoint (they are derived from a scope's owned
    /// rangetree), so a `BTreeMap<Address,...>` suffices for containment search.
    ranges: BTreeMap<Address, (Address, ScopeId)>,
}

impl ScopeResolve {
    fn empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// C++ `resolvemap.insert(scope, first, last)`.
    fn insert(&mut self, first: Address, last: Address, scope: ScopeId) {
        self.ranges.insert(first, (last, scope));
    }

    /// C++ `resolvemap.find(addr).first` → owning scope, if any.
    ///
    /// Returns the scope whose range `[first,last]` contains `addr`.  Searches
    /// the last range whose `first <= addr` (the candidate), as the C++ rangemap
    /// `find` walks refinement cells containing the point.
    fn find(&self, addr: &Address) -> Option<ScopeId> {
        use std::ops::Bound::{Included, Unbounded};
        let (first, (last, scope)) = self.ranges.range((Unbounded, Included(addr))).next_back()?;
        // `first`/`last` are within one space (a Range never straddles spaces),
        // and `Address`'s `Ord` orders by space then offset, so `first <= addr
        // <= last` already implies "same space, offset in range".
        if addr >= first && addr <= last {
            Some(*scope)
        } else {
            None
        }
    }

    /// Remove the range owned by `scope` starting at `first` (C++ clearResolve
    /// erases the matching `(first, scope)` pair).
    fn erase_at(&mut self, first: &Address, scope: ScopeId) {
        if let Some((_, s)) = self.ranges.get(first) {
            if *s == scope {
                self.ranges.remove(first);
            }
        }
    }
}

// ===========================================================================
// DuplicateFunctionError (database.hh:434-439)
// ===========================================================================
//
// The C++ `DuplicateFunctionError : public RecovError` carries the address and
// name of the duplicate function so a handler can recover and pick up the
// original symbol.  `kuna-base::error::KunaError` does not own this variant
// (error.rs is not this porter's file; its module docs explicitly defer
// `DuplicateFunctionError` to this wave).  Until error.rs grows the variant, the
// payload lives here and maps onto `KunaError::Recov` at the seam.

/// C++ `DuplicateFunctionError` — a function added more than once to the
/// database.  Local mapping (see module note); convertible to [`KunaError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateFunctionError {
    /// Address of function causing the error (C++ `address`).
    pub address: Address,
    /// Name of the function (C++ `functionName`).
    pub function_name: String,
}

impl DuplicateFunctionError {
    /// C++ constructor `DuplicateFunctionError(addr, nm)`.
    pub fn new(address: Address, nm: &str) -> DuplicateFunctionError {
        DuplicateFunctionError { address, function_name: nm.to_string() }
    }
}

impl From<DuplicateFunctionError> for KunaError {
    /// Maps onto `RecovError("Duplicate Function")` (the C++ base explanation).
    fn from(_e: DuplicateFunctionError) -> KunaError {
        KunaError::recov("Duplicate Function")
    }
}

// ===========================================================================
// Database  (database.hh:919-958)
// ===========================================================================

/// A manager for symbol scopes for a whole executable (C++ `Database`,
/// `database.hh:919-958`).
///
/// The highest-level container for [`Scope`]/[`Symbol`] objects; the formal
/// *symbol table* for the decompiler.  Scopes are registered via
/// [`Database::attach_scope`] and looked up by id/name; a cross-scope
/// search-by-address map ([`ScopeResolve`]) covers *namespace* scopes (the
/// global and function scopes are not entered, since they terminate / default
/// the hierarchical walk).  A property map ([`PartMap`]) labels memory ranges
/// with boolean properties (`read-only`, `volatile`) independent of symbols.
pub struct Database {
    /// All scopes, owned by the database (replaces the C++ scope `delete` graph).
    scopes: slotmap::SlotMap<ScopeId, Scope>,
    /// All symbols, owned by the database (the C++ `nametree` owns `Symbol *`;
    /// here the arena owns them and scopes reference ids).
    symbols: slotmap::SlotMap<SymbolId, Symbol>,
    /// Quick reference to the global Scope (C++ `globalscope`).
    globalscope: Option<ScopeId>,
    /// Address-to-namespace map (C++ `resolvemap`).
    resolvemap: ScopeResolve,
    /// Map from id to Scope (C++ `idmap`).
    idmap: BTreeMap<uint8, ScopeId>,
    /// Map of global properties (C++ `flagbase`).
    flagbase: PartMap<Address, uint4>,
    /// True if scope ids are built from hash of name (C++ `idByNameHash`).
    id_by_name_hash: bool,
}

impl Database {
    /// C++ `Database(Architecture *g,bool idByName)` (`database.cc:2954-2961`).
    pub fn new(id_by_name: bool) -> Database {
        Database {
            scopes: slotmap::SlotMap::with_key(),
            symbols: slotmap::SlotMap::with_key(),
            globalscope: None,
            resolvemap: ScopeResolve::default(),
            idmap: BTreeMap::new(),
            flagbase: PartMap::new(0), // C++ flagbase.defaultValue()=0
            id_by_name_hash: id_by_name,
        }
    }

    /// Get the global Scope id (C++ `getGlobalScope`).
    pub fn get_global_scope(&self) -> Option<ScopeId> {
        self.globalscope
    }

    /// Let scopes adjust after configuration is finished (C++
    /// `Database::adjustCaches`, `database.cc:3001`, which calls
    /// `ScopeInternal::adjustCaches`, `database.cc:2139`:
    /// `maptable.resize(glb->numSpaces(), null)`).
    ///
    /// Resizes every scope's per-space `maptable` to `num_spaces`.  Called from
    /// `Architecture::init_post_engine` after the spec decode created new spaces
    /// (`<stackpointer>` adds the stack `SpacebaseSpace`, fspec/iop/join are
    /// appended) — without it, a `map addr s0x…` into the (newly higher-indexed)
    /// stack space indexes past the maptable end.  Growing only (C++ `resize`
    /// never shrinks a populated table here, since spaces are only added).
    pub fn adjust_caches(&mut self, num_spaces: int4) {
        let n = if num_spaces < 0 { 0 } else { num_spaces as usize };
        for (_id, scope) in self.scopes.iter_mut() {
            if scope.maptable.len() < n {
                scope.maptable.resize_with(n, || None);
            }
        }
    }

    /// Borrow a scope by id.
    pub fn scope(&self, id: ScopeId) -> &Scope {
        &self.scopes[id]
    }

    /// Mutably borrow a scope by id.
    pub fn scope_mut(&mut self, id: ScopeId) -> &mut Scope {
        &mut self.scopes[id]
    }

    /// Borrow a symbol by id.
    pub fn symbol(&self, id: SymbolId) -> &Symbol {
        &self.symbols[id]
    }

    /// Mutably borrow a symbol by id.
    pub fn symbol_mut(&mut self, id: SymbolId) -> &mut Symbol {
        &mut self.symbols[id]
    }

    /// Get boolean properties at the given address (C++ `getProperty`).
    pub fn get_property(&self, addr: &Address) -> uint4 {
        *self.flagbase.get_value(addr)
    }

    /// Get the entire property map (C++ `getProperties`).
    pub fn get_properties(&self) -> &PartMap<Address, uint4> {
        &self.flagbase
    }

    /// Replace the property map (C++ `setProperties`).
    pub fn set_properties(&mut self, newflags: PartMap<Address, uint4>) {
        self.flagbase = newflags;
    }

    /// Build an unattached child Scope object (C++ `Scope::buildSubScope` →
    /// `new ScopeInternal(id,nm,glb)`); the id/name/space-count come from the
    /// caller.  Inserted into the arena; *not* yet attached.
    fn build_sub_scope(&mut self, id: uint8, nm: &str, num_spaces: int4) -> ScopeId {
        self.scopes.insert(Scope::new(id, nm, num_spaces))
    }

    /// Register a new Scope (C++ `Database::attachScope`, `database.cc:2976-2997`).
    ///
    /// The new Scope must be initially empty.  Passing `None` for `parent`
    /// registers the global Scope (which must have an empty name and be unique).
    pub fn attach_scope(&mut self, newscope: ScopeId, parent: Option<ScopeId>) -> KunaResult<()> {
        let unique_id = self.scopes[newscope].unique_id;
        let name_empty = self.scopes[newscope].name.is_empty();
        match parent {
            None => {
                if self.globalscope.is_some() {
                    return Err(KunaError::lowlevel("Multiple global scopes"));
                }
                if !name_empty {
                    return Err(KunaError::lowlevel("Global scope does not have empty name"));
                }
                self.globalscope = Some(newscope);
                self.idmap.insert(unique_id, newscope);
                Ok(())
            }
            Some(par) => {
                if name_empty {
                    return Err(KunaError::lowlevel("Non-global scope has empty name"));
                }
                if self.idmap.contains_key(&unique_id) {
                    // C++ RecovError("Duplicate scope id: " + getFullName())
                    let full = self.get_full_name(newscope);
                    return Err(KunaError::recov(format!("Duplicate scope id: {full}")));
                }
                self.idmap.insert(unique_id, newscope);
                // C++ parent->attachScope: child->parent = this; children[uniqueId] = child
                self.scopes[newscope].parent = Some(par);
                self.scopes[par].children.insert(unique_id, newscope);
                Ok(())
            }
        }
    }

    /// Look up a Scope by id (C++ `Database::resolveScope`).
    pub fn resolve_scope(&self, id: uint8) -> Option<ScopeId> {
        self.idmap.get(&id).copied()
    }

    /// Look for a Scope by id; if absent, create it (C++ `findCreateScope`,
    /// `database.cc:3104-3113`).
    pub fn find_create_scope(
        &mut self,
        id: uint8,
        nm: &str,
        parent: Option<ScopeId>,
        num_spaces: int4,
    ) -> KunaResult<ScopeId> {
        if let Some(res) = self.resolve_scope(id) {
            return Ok(res);
        }
        let res = self.build_sub_scope(id, nm, num_spaces);
        self.attach_scope(res, parent)?;
        Ok(res)
    }

    /// Get the full name of a Scope (C++ `Scope::getFullName`,
    /// `database.cc:1448-1459`): the `::`-joined ancestor names (global excluded).
    pub fn get_full_name(&self, scope: ScopeId) -> String {
        let s = &self.scopes[scope];
        let mut parent = match s.parent {
            None => return String::new(),
            Some(p) => p,
        };
        let mut fname = s.name.clone();
        // while(scope->parent != null) prepend scope->name
        while let Some(pp) = self.scopes[parent].parent {
            fname = format!("{}::{}", self.scopes[parent].name, fname);
            parent = pp;
        }
        fname
    }

    /// Get the ordered list of scopes up to the given one, starting at global
    /// (C++ `Scope::getScopePath`, `database.cc:1463-1479`).
    pub fn get_scope_path(&self, scope: ScopeId) -> Vec<ScopeId> {
        let mut count = 0;
        let mut cur = Some(scope);
        while let Some(c) = cur {
            count += 1;
            cur = self.scopes[c].parent;
        }
        let mut vec = vec![scope; count];
        cur = Some(scope);
        while let Some(c) = cur {
            count -= 1;
            vec[count] = c;
            cur = self.scopes[c].parent;
        }
        vec
    }

    /// Is `scope` a sub-scope of `scp` (C++ `Scope::isSubScope`,
    /// `database.cc:1437-1446`)?
    pub fn is_sub_scope(&self, scope: ScopeId, scp: ScopeId) -> bool {
        let mut tmp = Some(scope);
        while let Some(t) = tmp {
            if t == scp {
                return true;
            }
            tmp = self.scopes[t].parent;
        }
        false
    }

    /// Find first ancestor of `scope` not shared by `op2` (C++
    /// `Scope::findDistinguishingScope`, `database.cc:1486-1509`).  Returns
    /// `None` when `scope` is an ancestor of `op2` (no distinguishing scope).
    pub fn find_distinguishing_scope(&self, scope: ScopeId, op2: ScopeId) -> Option<ScopeId> {
        if scope == op2 {
            return None;
        }
        let sp = self.scopes[scope].parent;
        let op2p = self.scopes[op2].parent;
        if sp == Some(op2) {
            return Some(scope);
        }
        if op2p == Some(scope) {
            return None;
        }
        if sp == op2p {
            return Some(scope);
        }
        let this_path = self.get_scope_path(scope);
        let op2_path = self.get_scope_path(op2);
        let min = this_path.len().min(op2_path.len());
        for i in 0..min {
            if this_path[i] != op2_path[i] {
                return Some(this_path[i]);
            }
        }
        if min < this_path.len() {
            return Some(this_path[min]); // thisPath matches op2Path but is longer
        }
        if min < op2_path.len() {
            return None; // op2Path matches thisPath but is longer
        }
        Some(scope) // ancestor paths identical (only base scopes differ)
    }

    /// Look for the immediate child of `scope` with a given name (C++
    /// `Scope::resolveScope`, `database.cc:1320-1350`).
    pub fn resolve_child_scope(&self, scope: ScopeId, nm: &str, strategy: bool) -> Option<ScopeId> {
        let s = &self.scopes[scope];
        if strategy {
            let key = Scope::hash_scope_name(s.unique_id, nm);
            let child = *s.children.get(&key)?;
            if self.scopes[child].name == nm {
                return Some(child);
            }
            None
        } else if !nm.is_empty() && nm.as_bytes()[0].is_ascii_digit() {
            // Allow the string to directly specify the id (C++ istringstream >>).
            // C++ `s.unsetf(dec|hex|oct)` => auto-base: 0x.. hex, 0.. oct, else dec.
            let key = parse_auto_base_u64(nm)?;
            s.children.get(&key).copied()
        } else {
            // Linear scan in id order (C++ iterates children in ScopeMap order).
            for (_, &child) in s.children.iter() {
                if self.scopes[child].name == nm {
                    return Some(child);
                }
            }
            None
        }
    }
}

/// Parse an integer the way C++ `istringstream` with `unsetf(dec|hex|oct)` does:
/// a `0x`/`0X` prefix is hex, a leading `0` is octal, otherwise decimal.  Returns
/// `None` if the string is not a valid integer prefix-token (C++ leaves the value
/// unread, but resolveScope only consults it when parse succeeds in practice).
fn parse_auto_base_u64(s: &str) -> Option<uint8> {
    let t = s.trim();
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        uint8::from_str_radix(hex, 16).ok()
    } else if t.len() > 1 && t.starts_with('0') {
        uint8::from_str_radix(&t[1..], 8).ok()
    } else {
        t.parse::<uint8>().ok()
    }
}

// ===========================================================================
// Database: symbol construction, name tree, mapping  (ScopeInternal methods)
// ===========================================================================

impl Database {
    /// Make the [`NameKey`] for a symbol id (its current name + dedup).
    fn name_key(&self, sym: SymbolId) -> NameKey {
        let s = &self.symbols[sym];
        NameKey { name: s.name.clone(), name_dedup: s.name_dedup }
    }

    /// C++ `ScopeInternal::findFirstByName` (`database.cc:2763-2772`): the first
    /// Symbol in `scope`'s nametree with the given name, or `None`.
    fn find_first_by_name(&self, scope: ScopeId, nm: &str) -> Option<SymbolId> {
        use std::ops::Bound::{Included, Unbounded};
        // lower_bound on Symbol(name, dedup=0)
        let lo = NameKey { name: nm.to_string(), name_dedup: 0 };
        let nametree = &self.scopes[scope].nametree;
        let (k, &sid) = nametree.range((Included(lo), Unbounded)).next()?;
        if k.name != nm {
            return None;
        }
        Some(sid)
    }

    /// C++ `ScopeInternal::insertNameTree` (`database.cc:2742-2757`): insert a
    /// Symbol into the nametree, establishing a dedup id on name collision.
    fn insert_name_tree(&mut self, scope: ScopeId, sym: SymbolId) -> KunaResult<()> {
        use std::collections::btree_map::Entry;
        use std::ops::Bound::Included;
        self.symbols[sym].name_dedup = 0;
        let key0 = self.name_key(sym);
        match self.scopes[scope].nametree.entry(key0) {
            Entry::Vacant(slot) => {
                slot.insert(sym); // C++ insert(sym) succeeded (no collision)
                return Ok(());
            }
            Entry::Occupied(_) => {} // fall through to dedup
        }
        // Collision (C++ database.cc:2748-2755): set nameDedup=0xffffffff,
        // upper_bound(sym), --iter == last symbol with this name; the new dedup
        // is that one's + 1.  The last entry sharing the name is the one with the
        // greatest dedup, found by ranging [name,0]..=[name,0xffffffff].
        let name = self.symbols[sym].name.clone();
        let lo = NameKey { name: name.clone(), name_dedup: 0 };
        let hi = NameKey { name: name.clone(), name_dedup: 0xffff_ffff };
        let last_dedup = self.scopes[scope]
            .nametree
            .range((Included(lo), Included(hi)))
            .next_back()
            .map(|(k, _)| k.name_dedup)
            .expect("name collision implies an existing entry with this name");
        self.symbols[sym].name_dedup = last_dedup.wrapping_add(1);
        let key = self.name_key(sym);
        match self.scopes[scope].nametree.entry(key) {
            Entry::Vacant(slot) => {
                slot.insert(sym);
                Ok(())
            }
            Entry::Occupied(_) => Err(KunaError::lowlevel(format!(
                "Could  not deduplicate symbol: {}",
                self.symbols[sym].name
            ))),
        }
    }

    /// C++ `ScopeInternal::addSymbolInternal` (`database.cc:1838-1869`): assign a
    /// symbol id, fill an undefined name, validate the type, and enter the symbol
    /// into the nametree and category tables.
    fn add_symbol_internal(&mut self, scope: ScopeId, sym: SymbolId) -> KunaResult<()> {
        if self.symbols[sym].symbol_id == 0 {
            let unique_id = self.scopes[scope].unique_id;
            let next = self.scopes[scope].next_unique_id;
            self.symbols[sym].symbol_id =
                SYMBOL_ID_BASE + ((unique_id & 0xffff) << 40) + next;
            self.scopes[scope].next_unique_id += 1;
        }
        // C++ wraps the rest in try/catch that deletes sym on LowlevelError; here
        // a returned Err leaves the arena holding the symbol (caller discards id).
        if self.symbols[sym].name.is_empty() {
            let nm = self.build_undefined_name(scope)?;
            self.symbols[sym].name = nm.clone();
            self.symbols[sym].display_name = nm;
        }
        match &self.symbols[sym].dtype {
            None => {
                return Err(KunaError::lowlevel(format!(
                    "{} symbol created with no type",
                    self.symbols[sym].get_name()
                )));
            }
            Some(t) => {
                if t.get_size() < 1 {
                    return Err(KunaError::lowlevel(format!(
                        "{} symbol created with zero size type",
                        self.symbols[sym].get_name()
                    )));
                }
            }
        }
        self.insert_name_tree(scope, sym)?;
        let cat = self.symbols[sym].category;
        if cat >= 0 {
            let cat = cat as usize;
            while self.scopes[scope].category.len() <= cat {
                self.scopes[scope].category.push(Vec::new());
            }
            if self.symbols[sym].category > 0 {
                self.symbols[sym].catindex = self.scopes[scope].category[cat].len() as u16;
            }
            let catindex = self.symbols[sym].catindex as usize;
            let list = &mut self.scopes[scope].category[cat];
            while list.len() <= catindex {
                list.push(None);
            }
            list[catindex] = Some(sym);
        }
        Ok(())
    }

    /// C++ `ScopeInternal::setCategory` (`database.cc:2854`): move `sym` into the
    /// `cat`/`ind` slot of `scope`'s category table (removing it from any prior
    /// category slot first).  For `cat > 0` the index is forced to append; for
    /// `cat == 0` (function parameters) the caller's `ind` is honored so a recovered
    /// parameter lands at its 0-based slot.
    pub fn set_category(&mut self, scope: ScopeId, sym: SymbolId, cat: int4, ind: int4) {
        let old_cat = self.symbols[sym].category;
        if old_cat >= 0 {
            let oc = old_cat as usize;
            let old_idx = self.symbols[sym].catindex as usize;
            if let Some(list) = self.scopes[scope].category.get_mut(oc) {
                if old_idx < list.len() {
                    list[old_idx] = None;
                }
                while matches!(list.last(), Some(None)) {
                    list.pop();
                }
            }
        }
        self.symbols[sym].category = cat;
        self.symbols[sym].catindex = ind as u16;
        if cat < 0 {
            return;
        }
        let cat = cat as usize;
        while self.scopes[scope].category.len() <= cat {
            self.scopes[scope].category.push(Vec::new());
        }
        if (cat as int4) > 0 {
            self.symbols[sym].catindex = self.scopes[scope].category[cat].len() as u16;
        }
        let catindex = self.symbols[sym].catindex as usize;
        let list = &mut self.scopes[scope].category[cat];
        while list.len() <= catindex {
            list.push(None);
        }
        list[catindex] = Some(sym);
    }

    /// C++ `ScopeInternal::buildUndefinedName` (`database.cc:2550-2581`): the next
    /// `$$undefXXXXXXXX` name not yet used in `scope`.
    fn build_undefined_name(&self, scope: ScopeId) -> KunaResult<String> {
        use std::ops::Bound::{Included, Unbounded};
        // C++: lower_bound("$$undefz"); if not begin, --iter; inspect that name.
        let probe = NameKey { name: "$$undefz".to_string(), name_dedup: 0 };
        let nametree = &self.scopes[scope].nametree;
        // iter = lower_bound(probe); if iter != begin --iter; (last name < probe
        // in most cases, since 'z' > any hex digit)
        let prev = nametree.range((Unbounded, Included(&probe))).next_back();
        if let Some((k, _)) = prev {
            let symname = &k.name;
            if symname.len() == 15 && symname.starts_with("$$undef") {
                let hexpart = &symname[7..15];
                let uniq = uint4::from_str_radix(hexpart, 16)
                    .map_err(|_| KunaError::lowlevel("Error creating undefined name"))?;
                if uniq == u32::MAX {
                    return Err(KunaError::lowlevel("Error creating undefined name"));
                }
                let uniq = uniq + 1;
                return Ok(format!("$$undef{uniq:08x}"));
            }
        }
        Ok("$$undef00000000".to_string())
    }

    /// C++ `ScopeInternal::makeNameUnique` (`database.cc:2583-2644`): produce a
    /// version of `nm` that won't collide with other names in `scope`.
    fn make_name_unique(&self, scope: ScopeId, nm: &str) -> KunaResult<String> {
        use std::ops::Bound::{Included, Unbounded};
        let first = self.find_first_by_name(scope, nm);
        if first.is_none() {
            return Ok(nm.to_string()); // already unique
        }
        let first_key = {
            let fid = first.unwrap();
            self.name_key(fid)
        };
        let nametree = &self.scopes[scope].nametree;
        // boundsym = Symbol(nm + "_x99999", dedup=0xffffffff); lower_bound(boundsym)
        let bound = NameKey { name: format!("{nm}_x99999"), name_dedup: 0xffff_ffff };
        // Walk backwards from lower_bound(bound) over names beginning with nm.
        let mut iter: Vec<&NameKey> = nametree
            .range((Unbounded, Included(&bound)))
            .map(|(k, _)| k)
            .collect();
        let mut uniqid: uint4;
        loop {
            uniqid = 0xffff_ffff;
            // --iter2: last symbol whose name starts with nm
            let bkey = match iter.pop() {
                None => break,
                Some(k) => k,
            };
            if *bkey == first_key {
                break;
            }
            let bname = &bkey.name;
            let mut is_x_form = false;
            let mut dig_count = 0;
            let bbytes = bname.as_bytes();
            if bname.len() >= nm.len() + 3 && bbytes[nm.len()] == b'_' {
                let mut i = nm.len() + 1;
                if bbytes[i] == b'x' {
                    i += 1; // 5 digit form
                    is_x_form = true;
                }
                uniqid = 0;
                while i < bname.len() {
                    let dig = bbytes[i];
                    if !dig.is_ascii_digit() {
                        // Everything after '_' must be a digit, else not our form.
                        uniqid = 0xffff_ffff;
                        break;
                    }
                    uniqid = uniqid.wrapping_mul(10).wrapping_add((dig - b'0') as uint4);
                    dig_count += 1;
                    i += 1;
                }
            }
            // C++ database.cc:2620-2623: an x-form needs exactly 5 digits, a
            // plain form exactly 2; anything else resets uniqid to the sentinel.
            if (is_x_form && dig_count != 5) || (!is_x_form && dig_count != 2) {
                uniqid = 0xffff_ffff;
            }
            if uniqid != 0xffff_ffff {
                break;
            }
        }

        let res_string = if uniqid == 0xffff_ffff {
            format!("{nm}_00") // start a new sequence
        } else {
            let uniqid = uniqid + 1;
            if uniqid < 100 {
                format!("{nm}_{uniqid:02}")
            } else {
                format!("{nm}_x{uniqid:05}")
            }
        };
        if self.find_first_by_name(scope, &res_string).is_some() {
            return Err(KunaError::lowlevel(format!("Unable to uniquify name: {res_string}")));
        }
        Ok(res_string)
    }

    /// Public wrapper around `make_name_unique` (C++ `Scope::makeNameUnique` is
    /// `protected`, but `ScopeLocal::buildVariableName` — a subclass method —
    /// calls it; the Rust `ScopeLocal` owns a separate [`Database`], so it needs
    /// the entry point exposed).
    pub fn public_make_name_unique(&self, scope: ScopeId, nm: &str) -> String {
        // C++ makeNameUnique throws only when it cannot uniquify; the stack
        // naming convention always can (the offset is part of the base), so the
        // fall-back to the raw name is never reached on well-formed input.
        self.make_name_unique(scope, nm).unwrap_or_else(|_| nm.to_string())
    }
}

// ===========================================================================
// Database: SymbolEntry mapping  (addMapInternal / addDynamicMapInternal / addMap)
// ===========================================================================

impl Database {
    /// C++ `ScopeInternal::addMapInternal` (`database.cc:1871-1900`): create a
    /// rangemap SymbolEntry for `sym` over `[addr, addr+sz-1]`.
    #[allow(clippy::too_many_arguments)] // mirrors the C++ addMapInternal signature
    fn add_map_internal(
        &mut self,
        scope: ScopeId,
        sym: SymbolId,
        exfl: uint4,
        addr: &Address,
        off: int4,
        sz: int4,
        uselim: &RangeList,
    ) -> KunaResult<EntryRef> {
        let space = addr
            .get_space()
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("addMapInternal on invalid address"))?;
        let space_index = space.get_index() as usize;
        // Find or create the appropriate rangemap.
        if self.scopes[scope].maptable[space_index].is_none() {
            self.scopes[scope].maptable[space_index] = Some(EntryMap::new());
        }
        // lastaddress = addr + (sz-1); detect address-space overflow.  C++
        // `Address lastaddress = addr + (sz-1)` uses `Address::operator+`, which
        // wraps the offset at the *space mask* (`base->wrapOffset`), not at a raw
        // 64-bit boundary — `&Address + i64` reproduces that.  This matters for
        // sub-64-bit spaces: a symbol straddling the top of a 4-byte space wraps
        // below its start and is rejected (database.cc:1883-1889).
        let last_offset = (addr + (sz - 1) as i64).get_offset();
        if last_offset < addr.get_offset() {
            return Err(KunaError::lowlevel(format!(
                "Symbol {} extends beyond the end of the address space",
                self.symbols[sym].get_name()
            )));
        }
        let sym_addrtied = (self.symbols[sym].flags & varnode_flags::addrtied) != 0;
        let initdata = EntryInitData {
            space,
            symbol: sym,
            extraflags: exfl,
            offset: off,
            uselimit: uselim.clone(),
        };
        let idx = self.scopes[scope].maptable[space_index]
            .as_mut()
            .expect("maptable slot present")
            .insert((initdata, sym_addrtied), addr.get_offset(), last_offset);
        let eref = EntryRef::Mapped { space_index, idx };
        self.symbols[sym].mapentry.push(eref);
        // wholeCount bookkeeping (database.cc:1894-1898).
        if Some(sz) == self.symbols[sym].dtype.as_ref().map(|t| t.get_size()) {
            self.symbols[sym].whole_count += 1;
            if self.symbols[sym].whole_count == 2 {
                let key = self.name_key(sym);
                self.scopes[scope].multi_entry_set.insert(key, sym);
            }
        }
        Ok(eref)
    }

    /// C++ `ScopeInternal::addDynamicMapInternal` (`database.cc:1902-1915`).
    #[allow(clippy::too_many_arguments)] // mirrors the C++ addDynamicMapInternal signature
    fn add_dynamic_map_internal(
        &mut self,
        scope: ScopeId,
        sym: SymbolId,
        exfl: uint4,
        hash: uint8,
        off: int4,
        sz: int4,
        uselim: &RangeList,
    ) -> EntryRef {
        let entry = SymbolEntry::new_dynamic(sym, exfl, hash, off, sz, uselim.clone());
        let slot = self.scopes[scope].dynamicentry.len();
        self.scopes[scope].dynamicentry.push(Some(entry));
        let eref = EntryRef::Dynamic(slot);
        self.symbols[sym].mapentry.push(eref);
        if Some(sz) == self.symbols[sym].dtype.as_ref().map(|t| t.get_size()) {
            self.symbols[sym].whole_count += 1;
            if self.symbols[sym].whole_count == 2 {
                let key = self.name_key(sym);
                self.scopes[scope].multi_entry_set.insert(key, sym);
            }
        }
        eref
    }

    /// Read the [`SymbolEntry`] behind an [`EntryRef`] in `scope`.
    pub fn entry(&self, scope: ScopeId, eref: EntryRef) -> &SymbolEntry {
        match eref {
            EntryRef::Mapped { space_index, idx } => {
                &self.scopes[scope].maptable[space_index]
                    .as_ref()
                    .expect("mapped entry in present maptable")
                    .record(idx)
                    .entry
            }
            EntryRef::Dynamic(slot) => self.scopes[scope].dynamicentry[slot]
                .as_ref()
                .expect("dynamic entry not erased"),
        }
    }

    /// C++ `Scope::addMap` (`database.cc:1131-1185`): integrate an unintegrated
    /// SymbolEntry (mapping the whole Symbol) into the range maps, marking
    /// persistence / address-tying and unraveling join addresses.
    ///
    /// SEAM(W5): the join-address arm (`addr.isJoin()` → split into pieces via
    /// `glb->findJoin`) needs the `JoinRecord` table, a W5 subsystem.  Until W5
    /// supplies it, a join address is mapped as a single entry and a
    /// [`KunaError`]-free note is returned via `joined_pieces == false`.
    fn add_map(
        &mut self,
        scope: ScopeId,
        mut entry: SymbolEntry,
    ) -> KunaResult<EntryRef> {
        let sym = entry.symbol;
        // First set persistence based on scope (database.cc:1136-1147).
        if self.scopes[scope].is_global() {
            self.symbols[sym].flags |= varnode_flags::persist;
        } else if !entry.addr.is_invalid() {
            // If a non-global scope but the address is in the global discovery
            // range, still mark persistent.
            if let Some(gid) = self.globalscope {
                let in_global = {
                    let addr = entry.addr.clone();
                    self.scopes[gid].in_scope(&addr, 1, &Address::new_invalid())
                };
                if in_global {
                    self.symbols[sym].flags |= varnode_flags::persist;
                    entry.uselimit.clear(); // FIXME (kuna upstream): kludge for bad XML
                }
            }
        }

        let consume_size = self.symbols[sym].get_bytes_consumed();
        if entry.addr.is_invalid() {
            Ok(self.add_dynamic_map_internal(
                scope,
                sym,
                varnode_flags::mapped,
                entry.hash,
                0,
                consume_size,
                &entry.uselimit,
            ))
        } else {
            if entry.uselimit.empty() {
                self.symbols[sym].flags |= varnode_flags::addrtied;
                // Global properties (readonly/volatile) only when use is not
                // limited (database.cc:1156-1158).
                let prop = self.get_property(&entry.addr);
                self.symbols[sym].flags |= prop;
            }
            let addr = entry.addr.clone();
            let res = self.add_map_internal(
                scope,
                sym,
                varnode_flags::mapped,
                &addr,
                0,
                consume_size,
                &entry.uselimit,
            )?;
            // SEAM(W5): join-address piece expansion deferred (see doc above).
            Ok(res)
        }
    }

    /// C++ `Scope::addMapPoint` (`database.cc:1553-1562`): map a Symbol to a
    /// specific address (and optional usepoint).
    fn add_map_point(
        &mut self,
        scope: ScopeId,
        sym: SymbolId,
        addr: &Address,
        usepoint: &Address,
    ) -> KunaResult<EntryRef> {
        let mut entry = SymbolEntry::new_unintegrated(sym);
        if !usepoint.is_invalid() {
            let space = usepoint
                .get_space()
                .cloned()
                .expect("non-invalid usepoint has a space");
            entry.uselimit.insert_range(space, usepoint.get_offset(), usepoint.get_offset());
        }
        entry.addr = addr.clone();
        self.add_map(scope, entry)
    }
}

// ===========================================================================
// Database: high-level symbol factories  (Scope::addSymbol/addFunction/...)
// ===========================================================================

impl Database {
    /// C++ `Scope::addSymbol(nm,ct)` (`database.cc:1515-1523`): add a Symbol
    /// without mapping it to an address.  Returns the new symbol id.
    pub fn add_symbol(
        &mut self,
        scope: ScopeId,
        nm: &str,
        ct: Rc<Datatype>,
    ) -> KunaResult<SymbolId> {
        let sym = self.symbols.insert(Symbol::new(scope, nm, Some(ct)));
        self.add_symbol_internal(scope, sym)?;
        Ok(sym)
    }

    /// C++ `Scope::addSymbol(nm,ct,addr,usepoint)` (`database.cc:1535-1546`): add
    /// a Symbol with a single storage mapping.
    pub fn add_symbol_mapped(
        &mut self,
        scope: ScopeId,
        nm: &str,
        ct: Rc<Datatype>,
        addr: &Address,
        usepoint: &Address,
    ) -> KunaResult<(SymbolId, EntryRef)> {
        // C++ strips a "stripped" type first; the W6 Datatype seam has no
        // hasStripped yet, so it is a no-op here (SEAM(W6)).
        let sym = self.symbols.insert(Symbol::new(scope, nm, Some(ct)));
        self.add_symbol_internal(scope, sym)?;
        let eref = self.add_map_point(scope, sym, addr, usepoint)?;
        Ok((sym, eref))
    }

    /// C++ `Scope::addFunction` (`database.cc:1620-1637`): create a FunctionSymbol
    /// at `addr`.  `min_funcsymbol_size` comes from the architecture seam.
    pub fn add_function(
        &mut self,
        scope: ScopeId,
        addr: &Address,
        nm: &str,
        min_funcsymbol_size: int4,
        type_code: Rc<Datatype>,
    ) -> KunaResult<SymbolId> {
        // C++ warns (printMessage) on overlap; the message channel is W5, so the
        // overlap query is performed but the warning is dropped (SEAM(W5)).
        let mut sym = Symbol::new_empty(scope);
        sym.dtype = Some(type_code);
        sym.flags |= varnode_flags::namelock | varnode_flags::typelock; // FunctionSymbol::buildType
        sym.name = nm.to_string();
        sym.display_name = nm.to_string();
        sym.kind = SymbolKind::Function {
            consume_size: min_funcsymbol_size,
            has_funcdata: false,
            inline_func: false,
            no_return: false,
            inject_id: -1,
        };
        let sid = self.symbols.insert(sym);
        self.add_symbol_internal(scope, sid)?;
        self.add_map_point(scope, sid, addr, &Address::new_invalid())?;
        Ok(sid)
    }

    /// C++ `ScopeInternal::setAttribute` (`database.cc:2228-2235`): OR the
    /// lockable boolean attributes onto a symbol (the rest are masked out), then
    /// recompute its `size_typelock`.  The console `map` commands lock the symbols
    /// they create as name/type-locked.
    pub fn set_attribute(&mut self, sym: SymbolId, attr: uint4) {
        let mask = varnode_flags::typelock
            | varnode_flags::namelock
            | varnode_flags::readonly
            | varnode_flags::incidental_copy
            | varnode_flags::nolocalalias
            | varnode_flags::volatil
            | varnode_flags::indirectstorage
            | varnode_flags::hiddenretparm;
        self.symbols[sym].flags |= attr & mask;
        self.symbols[sym].check_size_type_lock();
    }

    /// C++ `Scope::addCodeLabel` (`database.cc:1669-1684`): create a LabSymbol.
    pub fn add_code_label(
        &mut self,
        scope: ScopeId,
        addr: &Address,
        nm: &str,
        lab_type: Rc<Datatype>,
    ) -> KunaResult<SymbolId> {
        let mut sym = Symbol::new_empty(scope);
        sym.dtype = Some(lab_type); // LabSymbol::buildType -> getBase(1, TYPE_UNKNOWN)
        sym.name = nm.to_string();
        sym.display_name = nm.to_string();
        sym.kind = SymbolKind::Lab;
        let sid = self.symbols.insert(sym);
        self.add_symbol_internal(scope, sid)?;
        self.add_map_point(scope, sid, addr, &Address::new_invalid())?;
        Ok(sid)
    }

    /// C++ `Scope::addDynamicSymbol` (`database.cc:1695-1707`): a Symbol attached
    /// to a dynamic hash and a code address.
    pub fn add_dynamic_symbol(
        &mut self,
        scope: ScopeId,
        nm: &str,
        ct: Rc<Datatype>,
        caddr: &Address,
        hash: uint8,
    ) -> KunaResult<SymbolId> {
        let sz = ct.get_size();
        let sym = self.symbols.insert(Symbol::new(scope, nm, Some(ct)));
        self.add_symbol_internal(scope, sym)?;
        let mut rnglist = RangeList::new();
        if !caddr.is_invalid() {
            let space = caddr.get_space().cloned().expect("valid caddr has space");
            rnglist.insert_range(space, caddr.get_offset(), caddr.get_offset());
        }
        self.add_dynamic_map_internal(scope, sym, varnode_flags::mapped, hash, 0, sz, &rnglist);
        Ok(sym)
    }

    /// C++ `Scope::addEquateSymbol` (`database.cc:1717-1729`).
    #[allow(clippy::too_many_arguments)] // mirrors the C++ addEquateSymbol signature (+ type seam arg)
    pub fn add_equate_symbol(
        &mut self,
        scope: ScopeId,
        nm: &str,
        format: uint4,
        value: uintb,
        addr: &Address,
        hash: uint8,
        base1_unknown: Rc<Datatype>,
    ) -> KunaResult<SymbolId> {
        // C++ EquateSymbol ctor: category=equate, type=getBase(1,TYPE_UNKNOWN),
        // dispflags |= format.
        let mut sym = Symbol::new(scope, nm, Some(base1_unknown));
        sym.category = symbol_category::EQUATE;
        sym.dispflags |= format;
        sym.kind = SymbolKind::Equate { value };
        let sid = self.symbols.insert(sym);
        self.add_symbol_internal(scope, sid)?;
        let mut rnglist = RangeList::new();
        if !addr.is_invalid() {
            let space = addr.get_space().cloned().expect("valid addr has space");
            rnglist.insert_range(space, addr.get_offset(), addr.get_offset());
        }
        self.add_dynamic_map_internal(scope, sid, varnode_flags::mapped, hash, 0, 1, &rnglist);
        Ok(sid)
    }

    /// C++ `Scope::addUnionFacetSymbol` (`database.cc:1742-1752`).
    pub fn add_union_facet_symbol(
        &mut self,
        scope: ScopeId,
        nm: &str,
        dt: Rc<Datatype>,
        field_num: int4,
        addr: &Address,
        hash: uint8,
    ) -> KunaResult<SymbolId> {
        let mut sym = Symbol::new(scope, nm, Some(dt));
        sym.category = symbol_category::UNION_FACET;
        sym.kind = SymbolKind::UnionFacet { field_num, addr_based: false };
        let sid = self.symbols.insert(sym);
        self.add_symbol_internal(scope, sid)?;
        let mut rnglist = RangeList::new();
        if !addr.is_invalid() {
            let space = addr.get_space().cloned().expect("valid addr has space");
            rnglist.insert_range(space, addr.get_offset(), addr.get_offset());
        }
        self.add_dynamic_map_internal(scope, sid, varnode_flags::mapped, hash, 0, 1, &rnglist);
        Ok(sid)
    }
}

// ===========================================================================
// Database: find/query within a single Scope  (ScopeInternal::find*)
// ===========================================================================

impl Database {
    /// Resolve a [`RecordIdx`] in a given space's rangemap to its [`SymbolEntry`].
    fn mapped_entry(&self, scope: ScopeId, space_index: usize, idx: RecordIdx) -> &SymbolEntry {
        &self.scopes[scope].maptable[space_index]
            .as_ref()
            .expect("present maptable")
            .record(idx)
            .entry
    }

    /// The `(start_offset, symbol_type, type_locked)` hints for every Symbol
    /// mapped into a scope's address space, in EntryMap list order (C++
    /// `MapState::gatherSymbols`, `varmap.cc:1046`: iterate `rangemap->begin_list`,
    /// emit `(getAddr().getOffset(), sym->getType(), sym->isTypeLocked())`).
    pub fn scope_space_symbol_hints(
        &self,
        scope: ScopeId,
        space_index: usize,
    ) -> Vec<(uintb, Rc<Datatype>, bool)> {
        let mut out = Vec::new();
        let rangemap = match self.scopes[scope].maptable.get(space_index).and_then(|m| m.as_ref()) {
            Some(rm) => rm,
            None => return out,
        };
        for (_, rec) in rangemap.records() {
            let entry = &rec.entry;
            let sym = entry.symbol;
            let symbol = &self.symbols[sym];
            let ct = match &symbol.dtype {
                Some(c) => Rc::clone(c),
                None => continue,
            };
            let type_locked = (symbol.flags & varnode_flags::typelock) != 0;
            out.push((entry.get_addr().get_offset(), ct, type_locked));
        }
        out
    }

    /// The `(name, type, addr, all_flags)` specs for every Symbol mapped into a
    /// scope's space (the console-mapped `map addr` symbols), so they can be
    /// re-created in a freshly-built `Funcdata` (see `ScopeLocal::mapped_symbol_specs`).
    pub fn scope_space_symbol_specs(
        &self,
        scope: ScopeId,
        space_index: usize,
    ) -> Vec<(String, Rc<Datatype>, Address, uint4)> {
        let mut out = Vec::new();
        let rangemap = match self.scopes[scope].maptable.get(space_index).and_then(|m| m.as_ref()) {
            Some(rm) => rm,
            None => return out,
        };
        for (_, rec) in rangemap.records() {
            let entry = &rec.entry;
            // Only the whole-symbol starting entry (offset 0); pieces are rebuilt
            // by re-mapping the whole symbol.
            if entry.get_offset() != 0 {
                continue;
            }
            let sym = entry.symbol;
            let symbol = &self.symbols[sym];
            let ct = match &symbol.dtype {
                Some(c) => Rc::clone(c),
                None => continue,
            };
            out.push((symbol.name.clone(), ct, entry.get_addr().clone(), symbol.flags));
        }
        out
    }

    /// C++ `ScopeInternal::findAddr` (`database.cc:2252-2276`): find a Symbol at
    /// exactly `addr`, valid at `usepoint`.
    pub fn find_addr(&self, scope: ScopeId, addr: &Address, usepoint: &Address) -> Option<EntryRef> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let sub2 = if usepoint.is_invalid() {
            EntrySubsort::maximal()
        } else {
            EntrySubsort::from_addr(usepoint)
        };
        let mut it = rangemap.find_subsorts(addr.get_offset(), EntrySubsort::minimal(), sub2);
        // C++ walks --res.second (reverse).
        while let Some(idx) = it.next_back() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_addr().get_offset() == addr.get_offset() {
                let symflags = self.symbols[entry.symbol].flags;
                if entry.in_use(usepoint, symflags) {
                    return Some(EntryRef::Mapped { space_index, idx });
                }
            }
        }
        None
    }

    /// C++ `ScopeInternal::findContainer` (`database.cc:2278-2310`): the smallest
    /// Symbol containing `[addr, addr+size-1]`, valid at `usepoint`.
    pub fn find_container(
        &self,
        scope: ScopeId,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<EntryRef> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let sub2 = if usepoint.is_invalid() {
            EntrySubsort::maximal()
        } else {
            EntrySubsort::from_addr(usepoint)
        };
        let mut it = rangemap.find_subsorts(addr.get_offset(), EntrySubsort::minimal(), sub2);
        let mut bestentry: Option<EntryRef> = None;
        let mut oldsize: int4 = -1;
        let end = addr.get_offset().wrapping_add(size as uintb).wrapping_sub(1);
        while let Some(idx) = it.next_back() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_last() >= end {
                // We contain the range.
                if entry.get_size() < oldsize || oldsize == -1 {
                    let symflags = self.symbols[entry.symbol].flags;
                    if entry.in_use(usepoint, symflags) {
                        bestentry = Some(EntryRef::Mapped { space_index, idx });
                        if entry.get_size() == size {
                            break;
                        }
                        oldsize = entry.get_size();
                    }
                }
            }
        }
        bestentry
    }

    /// C++ `ScopeInternal::findClosestFit` (`database.cc:2312-2347`).
    pub fn find_closest_fit(
        &self,
        scope: ScopeId,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<EntryRef> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let sub2 = if usepoint.is_invalid() {
            EntrySubsort::maximal()
        } else {
            EntrySubsort::from_addr(usepoint)
        };
        let mut it = rangemap.find_subsorts(addr.get_offset(), EntrySubsort::minimal(), sub2);
        let mut bestentry: Option<EntryRef> = None;
        let mut olddiff: int4 = -10000;
        while let Some(idx) = it.next_back() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_last() >= addr.get_offset() {
                // We contain start.
                let newdiff = entry.get_size() - size;
                if (olddiff < 0 && newdiff > olddiff)
                    || (olddiff >= 0 && newdiff >= 0 && newdiff < olddiff)
                {
                    let symflags = self.symbols[entry.symbol].flags;
                    if entry.in_use(usepoint, symflags) {
                        bestentry = Some(EntryRef::Mapped { space_index, idx });
                        if newdiff == 0 {
                            break;
                        }
                        olddiff = newdiff;
                    }
                }
            }
        }
        bestentry
    }

    /// C++ `ScopeInternal::findOverlap` (`database.cc:2420-2431`): first Symbol
    /// overlapping `[addr, addr+size-1]`.
    pub fn find_overlap(&self, scope: ScopeId, addr: &Address, size: int4) -> Option<EntryRef> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let end = addr.get_offset().wrapping_add(size as uintb).wrapping_sub(1);
        let idx = rangemap.find_overlap(addr.get_offset(), end)?;
        Some(EntryRef::Mapped { space_index, idx })
    }

    /// C++ `ScopeInternal::findFunction` (`database.cc:2349-2368`): the
    /// FunctionSymbol starting exactly at `addr` (forward walk).
    pub fn find_function(&self, scope: ScopeId, addr: &Address) -> Option<SymbolId> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let mut it = rangemap.find(addr.get_offset());
        for idx in it.by_ref() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_addr().get_offset() == addr.get_offset()
                && matches!(self.symbols[entry.symbol].kind, SymbolKind::Function { .. })
            {
                return Some(entry.symbol);
            }
        }
        None
    }

    /// C++ `Scope::queryFunction(const string&)` (`database.cc:1217-1230`): walk the
    /// scope-name lookup (`queryByName`, which climbs to parent scopes) and return
    /// the first `FunctionSymbol`.  In C++ this returns `funcsym->getFunction()` (a
    /// `Funcdata*`); the function-data-on-symbol subsystem is W5, so this returns the
    /// `SymbolId` of the FunctionSymbol — the stable handle whose parked
    /// `inline_func`/`no_return` flags stand in for the lazily-built `FuncProto`.
    pub fn query_function_by_name(&self, scope: ScopeId, nm: &str) -> Option<SymbolId> {
        for sid in self.query_by_name(scope, nm) {
            if matches!(self.symbols[sid].kind, SymbolKind::Function { .. }) {
                return Some(sid);
            }
        }
        None
    }

    /// Set the `FuncProto::is_inline` flag for a FunctionSymbol (C++
    /// `infd->getFuncProto().setInline(val)` reached via `OptionInline::apply`).
    /// No-op on a non-Function symbol.
    pub fn set_function_inline(&mut self, sid: SymbolId, val: bool) {
        if let SymbolKind::Function { inline_func, .. } = &mut self.symbols[sid].kind {
            *inline_func = val;
        }
    }

    /// Set the `FuncProto::is_no_return` flag for a FunctionSymbol (C++
    /// `infd->getFuncProto().setNoReturn(val)` reached via `OptionNoReturn::apply`).
    /// No-op on a non-Function symbol.
    pub fn set_function_no_return(&mut self, sid: SymbolId, val: bool) {
        if let SymbolKind::Function { no_return, .. } = &mut self.symbols[sid].kind {
            *no_return = val;
        }
    }

    /// Set the `FuncProto::injectid` for a FunctionSymbol (C++
    /// `fd->getFuncProto().setInjectId(injectid)` reached via `IfcFixupApply`).
    /// No-op on a non-Function symbol.
    pub fn set_function_inject_id(&mut self, sid: SymbolId, injectid: int4) {
        if let SymbolKind::Function { inject_id, .. } = &mut self.symbols[sid].kind {
            *inject_id = injectid;
        }
    }

    /// Is the FunctionSymbol at `addr` (in `scope`) marked \e inline (C++
    /// `queryFunction(addr)->getFuncProto().isInline()`)?  `false` if no function
    /// symbol starts at `addr`.  Read at flow time by `FlowInfo::queryCall`.
    pub fn function_is_inline(&self, scope: ScopeId, addr: &Address) -> bool {
        match self.find_function(scope, addr) {
            Some(sid) => {
                matches!(self.symbols[sid].kind, SymbolKind::Function { inline_func: true, .. })
            }
            None => false,
        }
    }

    /// The \e injection id parked on the FunctionSymbol at `addr` (in `scope`)
    /// (C++ `queryFunction(addr)->getFuncProto().getInjectId()`), or `-1` if no
    /// function symbol starts at `addr` (or it has no inject id).  Read at flow
    /// time by `FlowInfo::queryCall` to route inline vs payload injection.
    pub fn function_inject_id(&self, scope: ScopeId, addr: &Address) -> int4 {
        match self.find_function(scope, addr) {
            Some(sid) => match self.symbols[sid].kind {
                SymbolKind::Function { inject_id, .. } => inject_id,
                _ => -1,
            },
            None => -1,
        }
    }

    /// Is the FunctionSymbol at `addr` (in `scope`) marked \e noreturn (C++
    /// `queryFunction(addr)->getFuncProto().isNoReturn()`)?  `false` if no function
    /// symbol starts at `addr`.
    pub fn function_is_no_return(&self, scope: ScopeId, addr: &Address) -> bool {
        match self.find_function(scope, addr) {
            Some(sid) => {
                matches!(self.symbols[sid].kind, SymbolKind::Function { no_return: true, .. })
            }
            None => false,
        }
    }

    /// C++ `ScopeInternal::findExternalRef` (`database.cc:2370-2388`).
    pub fn find_external_ref(&self, scope: ScopeId, addr: &Address) -> Option<SymbolId> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        let mut it = rangemap.find(addr.get_offset());
        for idx in it.by_ref() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_addr().get_offset() == addr.get_offset() {
                if matches!(self.symbols[entry.symbol].kind, SymbolKind::ExternRef { .. }) {
                    return Some(entry.symbol);
                }
                // C++ breaks on the first entry at this exact address regardless.
                break;
            }
        }
        None
    }

    /// C++ `ScopeInternal::findCodeLabel` (`database.cc:2396-2418`).
    pub fn find_code_label(&self, scope: ScopeId, addr: &Address) -> Option<SymbolId> {
        let space = addr.get_space()?;
        let space_index = space.get_index() as usize;
        let rangemap = self.scopes[scope].maptable.get(space_index)?.as_ref()?;
        // C++ find(offset, subsort(false), subsort(addr))
        let sub2 = EntrySubsort::from_addr(addr);
        let mut it = rangemap.find_subsorts(addr.get_offset(), EntrySubsort::minimal(), sub2);
        while let Some(idx) = it.next_back() {
            let entry = self.mapped_entry(scope, space_index, idx);
            if entry.get_addr().get_offset() == addr.get_offset() {
                let symflags = self.symbols[entry.symbol].flags;
                if entry.in_use(addr, symflags) {
                    if matches!(self.symbols[entry.symbol].kind, SymbolKind::Lab) {
                        return Some(entry.symbol);
                    }
                    break;
                }
            }
        }
        None
    }

    /// C++ `ScopeInternal::findByName` (`database.cc:2433-2443`): all Symbols in
    /// `scope` with the given name.
    pub fn find_by_name(&self, scope: ScopeId, nm: &str) -> Vec<SymbolId> {
        use std::ops::Bound::{Included, Unbounded};
        let mut res = Vec::new();
        let lo = NameKey { name: nm.to_string(), name_dedup: 0 };
        for (k, &sid) in self.scopes[scope].nametree.range((Included(lo), Unbounded)) {
            if k.name != nm {
                break;
            }
            res.push(sid);
        }
        res
    }

    /// C++ `ScopeInternal::isNameUsed` (`database.cc:2445-2460`): is `nm` used in
    /// `scope` or an ancestor up to (but not including) `op2`?
    pub fn is_name_used(&self, scope: ScopeId, nm: &str, op2: Option<ScopeId>) -> bool {
        use std::ops::Bound::{Included, Unbounded};
        let lo = NameKey { name: nm.to_string(), name_dedup: 0 };
        if let Some((k, _)) = self.scopes[scope].nametree.range((Included(lo), Unbounded)).next() {
            if k.name == nm {
                return true;
            }
        }
        let par = match self.scopes[scope].parent {
            None => return false,
            Some(p) => p,
        };
        if Some(par) == op2 {
            return false;
        }
        if self.scopes[par].parent.is_none() {
            return false; // Never recurse into global scope
        }
        self.is_name_used(par, nm, op2)
    }
}

// ===========================================================================
// Database: scope-hierarchy walks  (Scope::stack* / query* / mapScope)
// ===========================================================================

impl Database {
    /// C++ `Scope::mapScope` (`database.cc:3211-3238`): the lowest namespace
    /// Scope that might own `addr`, defaulting to `qpoint`.
    pub fn map_scope(&self, qpoint: ScopeId, addr: &Address, _usepoint: &Address) -> ScopeId {
        if self.resolvemap.empty() {
            return qpoint;
        }
        self.resolvemap.find(addr).unwrap_or(qpoint)
    }

    /// C++ `Scope::queryByName` (`database.cc:1203-1211`): look up symbols by
    /// name, recursing into parents only when this scope has none.
    pub fn query_by_name(&self, scope: ScopeId, nm: &str) -> Vec<SymbolId> {
        let res = self.find_by_name(scope, nm);
        if !res.is_empty() {
            return res;
        }
        match self.scopes[scope].parent {
            Some(p) => self.query_by_name(p, nm),
            None => Vec::new(),
        }
    }

    /// C++ `Scope::stackAddr` (`database.cc:914-933`): walk scopes `[scope1, scope2)`
    /// for a Symbol at exactly `addr`; returns (owning scope, matching entry).
    fn stack_addr(
        &self,
        mut scope1: Option<ScopeId>,
        scope2: Option<ScopeId>,
        addr: &Address,
        usepoint: &Address,
    ) -> (Option<ScopeId>, Option<(ScopeId, EntryRef)>) {
        if addr.is_constant() {
            return (None, None);
        }
        while let Some(s1) = scope1 {
            if scope2 == Some(s1) {
                break;
            }
            if let Some(eref) = self.find_addr(s1, addr, usepoint) {
                return (Some(s1), Some((s1, eref)));
            }
            if self.scopes[s1].in_scope(addr, 1, usepoint) {
                return (Some(s1), None); // Discovery of new variable
            }
            scope1 = self.scopes[s1].parent;
        }
        (None, None)
    }

    /// C++ `Scope::stackContainer` (`database.cc:948-967`).
    fn stack_container(
        &self,
        mut scope1: Option<ScopeId>,
        scope2: Option<ScopeId>,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> (Option<ScopeId>, Option<(ScopeId, EntryRef)>) {
        if addr.is_constant() {
            return (None, None);
        }
        while let Some(s1) = scope1 {
            if scope2 == Some(s1) {
                break;
            }
            if let Some(eref) = self.find_container(s1, addr, size, usepoint) {
                return (Some(s1), Some((s1, eref)));
            }
            if self.scopes[s1].in_scope(addr, size, usepoint) {
                return (Some(s1), None);
            }
            scope1 = self.scopes[s1].parent;
        }
        (None, None)
    }

    /// C++ `Scope::queryByAddr` (`database.cc:1236-1243`).
    pub fn query_by_addr(
        &self,
        scope: ScopeId,
        addr: &Address,
        usepoint: &Address,
    ) -> Option<(ScopeId, EntryRef)> {
        let basescope = self.map_scope(scope, addr, usepoint);
        let (_, res) = self.stack_addr(Some(basescope), None, addr, usepoint);
        res
    }

    /// C++ `Scope::queryContainer` (`database.cc:1251-1258`).
    pub fn query_container(
        &self,
        scope: ScopeId,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<(ScopeId, EntryRef)> {
        let basescope = self.map_scope(scope, addr, usepoint);
        let (_, res) = self.stack_container(Some(basescope), None, addr, size, usepoint);
        res
    }

    /// C++ `Scope::queryProperties` (`database.cc:1268-1286`): the smallest
    /// containing Symbol *and* the boolean properties of the memory range.
    /// Returns `(matching_entry, flags)`.
    pub fn query_properties(
        &self,
        scope: ScopeId,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> (Option<(ScopeId, EntryRef)>, uint4) {
        let basescope = self.map_scope(scope, addr, usepoint);
        let (finalscope, res) = self.stack_container(Some(basescope), None, addr, size, usepoint);
        let flags;
        if let Some((sc, eref)) = &res {
            // use the entry's flags (extraflags | symbol flags) = getAllFlags
            let entry = self.entry(*sc, *eref);
            flags = entry.extraflags | self.symbols[entry.symbol].flags;
        } else if let Some(fs) = finalscope {
            flags = {
                let mut f = varnode_flags::mapped | varnode_flags::addrtied;
                if self.scopes[fs].is_global() {
                    f |= varnode_flags::persist;
                }
                f | self.get_property(addr)
            };
        } else {
            flags = self.get_property(addr);
        }
        (res, flags)
    }

    /// C++ `Scope::isReadOnly` (`database.cc:1824-1830`).
    pub fn is_read_only(
        &self,
        scope: ScopeId,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> bool {
        let (_, flags) = self.query_properties(scope, addr, size, usepoint);
        (flags & varnode_flags::readonly) != 0
    }

    /// Flatten the global scope's mapped storage + owned ranges + the property map
    /// into a read-only [`GlobalQuery`](crate::seams::GlobalQuery) the per-function
    /// `glb` [`ArchHandle`](crate::seams::ArchHandle) carries.
    ///
    /// This is the wire for `localmap->queryProperties`'s walk up to the global
    /// scope: the C++ `glb` reaches the live `Database`, but the merged kuna `glb`
    /// is a separate skeleton, so the global symbol/property data is snapshot onto
    /// it at [`Architecture::build_arch_handle`](crate::architecture::Architecture::
    /// build_arch_handle).  Faithful because the global scope is frozen by the time
    /// a function is loaded (every `map addr` ran first).  Each `GlobalEntry`
    /// carries exactly what `findContainer`/`inUse`/`getAllFlags` read; the owned
    /// `RangeList` is the global `Scope::rangetree`; the flagbase is the C++
    /// `Database::flagbase`.
    pub fn build_global_query(&self) -> crate::seams::GlobalQuery {
        use crate::seams::{GlobalEntry, GlobalQuery};
        let gid = match self.globalscope {
            Some(g) => g,
            None => return GlobalQuery::default(),
        };
        let scope = &self.scopes[gid];
        let mut entries: Vec<GlobalEntry> = Vec::new();
        // Walk every per-space rangemap of the global scope, flattening each
        // SymbolEntry (C++ iterates `maptable[i]->begin_list..end_list`).
        for (space_index, slot) in scope.maptable.iter().enumerate() {
            let rangemap = match slot.as_ref() {
                Some(rm) => rm,
                None => continue,
            };
            for (_, rec) in rangemap.records() {
                let entry = &rec.entry;
                // Dynamic (hash-only) entries have no address: queryProperties for
                // a Varnode address never matches them (the `addr.isInvalid` skip).
                if entry.is_dynamic() {
                    continue;
                }
                let sym_flags = self.symbols[entry.symbol].flags;
                entries.push(GlobalEntry {
                    space_index: space_index as int4,
                    first: entry.get_first(),
                    last: entry.get_last(),
                    size: entry.get_size(),
                    // getAllFlags() = extraflags | symbol->getFlags().
                    all_flags: entry.extraflags | sym_flags,
                    addrtied: (sym_flags & varnode_flags::addrtied) != 0,
                    uselimit: entry.uselimit.clone(),
                });
            }
        }
        crate::seams::GlobalQuery {
            entries,
            owned: scope.rangetree.clone(),
            flagbase: self.flagbase.clone(),
        }
    }

    /// C++ `Scope::discoverScope` (`database.cc:1358-1370`): the sub/containing
    /// Scope that owns `[addr, addr+sz-1]` at `usepoint`.
    pub fn discover_scope(
        &self,
        scope: ScopeId,
        addr: &Address,
        sz: int4,
        usepoint: &Address,
    ) -> Option<ScopeId> {
        if addr.is_constant() {
            return None;
        }
        let mut basescope = Some(self.map_scope(scope, addr, usepoint));
        while let Some(bs) = basescope {
            if self.scopes[bs].in_scope(addr, sz, usepoint) {
                return Some(bs);
            }
            basescope = self.scopes[bs].parent;
        }
        None
    }
}

// ===========================================================================
// Database: ownership ranges + property ranges + resolve map
// ===========================================================================

impl Database {
    /// C++ `Database::clearResolve` (`database.cc:2900-2919`): remove a namespace
    /// scope's owned ranges from the resolve map.
    fn clear_resolve(&mut self, scope: ScopeId) {
        if Some(scope) == self.globalscope {
            return;
        }
        if self.scopes[scope].is_functional {
            return;
        }
        let firsts: Vec<Address> = self.scopes[scope]
            .rangetree
            .iter()
            .map(|rng| rng.get_first_addr())
            .collect();
        for first in firsts {
            self.resolvemap.erase_at(&first, scope);
        }
    }

    /// C++ `Database::fillResolve` (`database.cc:2938-2949`).
    fn fill_resolve(&mut self, scope: ScopeId) {
        if Some(scope) == self.globalscope {
            return;
        }
        if self.scopes[scope].is_functional {
            return;
        }
        let ranges: Vec<(Address, Address)> = self.scopes[scope]
            .rangetree
            .iter()
            .map(|rng| (rng.get_first_addr(), rng.get_last_addr()))
            .collect();
        for (first, last) in ranges {
            self.resolvemap.insert(first, last, scope);
        }
    }

    /// C++ `Database::setRange` (`database.cc:3062-3068`).
    pub fn set_range(&mut self, scope: ScopeId, rlist: RangeList) {
        self.clear_resolve(scope);
        self.scopes[scope].rangetree = rlist;
        self.fill_resolve(scope);
    }

    /// C++ `Database::addRange` (`database.cc:3076-3082`).
    pub fn add_range(&mut self, scope: ScopeId, spc: Rc<AddrSpace>, first: uintb, last: uintb) {
        self.clear_resolve(scope);
        self.scopes[scope].rangetree.insert_range(spc, first, last);
        self.fill_resolve(scope);
    }

    /// C++ `Database::removeRange` (`database.cc:3090-3096`).
    pub fn remove_range(&mut self, scope: ScopeId, spc: Rc<AddrSpace>, first: uintb, last: uintb) {
        self.clear_resolve(scope);
        self.scopes[scope].rangetree.remove_range(spc, first, last);
        self.fill_resolve(scope);
    }

    /// C++ `Database::setPropertyRange` (`database.cc:3246-3265`): OR boolean
    /// properties over a memory range.  `last_open` is `range.getLastAddrOpen(glb)`
    /// (the architecture-relative one-past-the-end address; the caller supplies it
    /// since the AddrSpaceManager lives in the architecture seam).
    pub fn set_property_range(&mut self, flags: uint4, addr1: &Address, addr2: &Address) {
        self.flagbase.split(addr1);
        if !addr2.is_invalid() {
            self.flagbase.split(addr2);
        }
        // Update bits across [addr1, addr2) (or [addr1, end) if addr2 invalid).
        for (k, v) in self.flagbase.iter_from_mut(addr1) {
            if !addr2.is_invalid() && k >= addr2 {
                break;
            }
            *v |= flags;
        }
    }

    /// C++ `Database::clearPropertyRange` (`database.cc:3271-3291`).
    pub fn clear_property_range(&mut self, flags: uint4, addr1: &Address, addr2: &Address) {
        self.flagbase.split(addr1);
        if !addr2.is_invalid() {
            self.flagbase.split(addr2);
        }
        let inv = !flags;
        for (k, v) in self.flagbase.iter_from_mut(addr1) {
            if !addr2.is_invalid() && k >= addr2 {
                break;
            }
            *v &= inv;
        }
    }
}

// ===========================================================================
// Database: variable naming  (ScopeInternal::buildVariableName, buildDefaultName)
// ===========================================================================

/// Capitalize the first ASCII char of a space name (C++ `spacename[0]=toupper(..)`).
fn capitalize_first(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if let Some(c) = chars.first_mut() {
        *c = c.to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

impl Database {
    /// C++ `ScopeInternal::buildVariableName` (`database.cc:2462-2548`): given an
    /// address, usepoint, data-type, index and flags, build a variable name.
    ///
    /// `arch` supplies the translator (register names), the angr toggle, and the
    /// type-name-base renderer (the W5/W6 seams).
    #[allow(clippy::too_many_arguments)] // mirrors the C++ buildVariableName signature (+ arch seam)
    pub fn build_variable_name(
        &self,
        scope: ScopeId,
        addr: &Address,
        _pc: &Address,
        ct: Option<&Datatype>,
        index: &mut int4,
        flags: uint4,
        arch: &dyn DatabaseArch,
    ) -> KunaResult<String> {
        use std::fmt::Write;
        let sz = ct.map(|c| c.get_size()).unwrap_or(1);
        let mut s = String::new();

        if (flags & varnode_flags::unaffected) != 0 {
            if (flags & varnode_flags::return_address) != 0 {
                s.push_str("unaff_retaddr");
            } else {
                let space = addr.get_space().expect("named varnode has a space");
                let unaffname = arch.translate().get_register_name(space, addr.get_offset(), sz);
                if unaffname.is_empty() {
                    let _ = write!(s, "unaff_{:08x}", addr.get_offset());
                } else {
                    let _ = write!(s, "unaff_{unaffname}");
                }
            }
        } else if (flags & varnode_flags::persist) != 0 {
            let space = addr.get_space().expect("named varnode has a space");
            let spacename = arch.translate().get_register_name(space, addr.get_offset(), sz);
            if !spacename.is_empty() {
                s.push_str(&spacename);
            } else {
                if arch.name_style_angr() {
                    // (kuna) angr-style: global data -> dat_<addr>
                    return self.make_name_unique(scope, &kuna_global_data_name(addr));
                }
                if let Some(c) = ct {
                    s.push_str(&arch.type_name_base(c));
                }
                let mut spacename = capitalize_first(space.get_name());
                let word_off = AddrSpace::byte_to_address(addr.get_offset(), space.get_word_size());
                let width = (2 * addr.get_addr_size()) as usize;
                let _ = write!(spacename, "{word_off:0width$x}");
                s.push_str(&spacename);
            }
        } else if (flags & varnode_flags::input) != 0 && *index < 0 {
            // Irregular input.
            let space = addr.get_space().expect("named varnode has a space");
            let regname = arch.translate().get_register_name(space, addr.get_offset(), sz);
            if regname.is_empty() {
                let _ = write!(s, "in_{}_{:08x}", space.get_name(), addr.get_offset());
            } else {
                let _ = write!(s, "in_{regname}");
            }
        } else if (flags & varnode_flags::input) != 0 {
            // Regular parameter.
            let _ = write!(s, "param_{index}");
        } else if (flags & varnode_flags::addrtied) != 0 {
            if let Some(c) = ct {
                s.push_str(&arch.type_name_base(c));
            }
            let space = addr.get_space().expect("named varnode has a space");
            let mut spacename = capitalize_first(space.get_name());
            let word_off = AddrSpace::byte_to_address(addr.get_offset(), space.get_word_size());
            let width = (2 * addr.get_addr_size()) as usize;
            let _ = write!(spacename, "{word_off:0width$x}");
            s.push_str(&spacename);
        } else if (flags & varnode_flags::indirect_creation) != 0 {
            s.push_str("extraout_");
            let space = addr.get_space().expect("named varnode has a space");
            let regname = arch.translate().get_register_name(space, addr.get_offset(), sz);
            if !regname.is_empty() {
                s.push_str(&regname);
            } else {
                s.push_str("var");
            }
        } else {
            // Some sort of local variable.
            if let Some(c) = ct {
                s.push_str(&arch.type_name_base(c));
            }
            let _ = write!(s, "Var{index}");
            *index += 1;
            if self.find_first_by_name(scope, &s).is_some() {
                // Try bumping the index a few times before makeNameUnique.
                for _ in 0..10 {
                    let mut s2 = String::new();
                    if let Some(c) = ct {
                        s2.push_str(&arch.type_name_base(c));
                    }
                    let _ = write!(s2, "Var{index}");
                    *index += 1;
                    if self.find_first_by_name(scope, &s2).is_none() {
                        return Ok(s2);
                    }
                }
            }
        }
        self.make_name_unique(scope, &s)
    }

    /// C++ `Scope::buildDefaultName` (`database.cc:1761-1814`) — the kuna anchor
    /// (DIV-5) with the angr-style branch (aN / dat_ / vN) and the upstream
    /// branch.  The Varnode-representative arm of the upstream branch needs the
    /// Funcdata/Varnode/HighVariable subsystems (W5/W7); it is taken via the
    /// `vn` argument as a precomputed (addr, flags, is_input, is_constant) tuple
    /// when available, or `None` to use the symbol's first map entry.
    ///
    /// `base` may be updated (the running uniquification index).
    pub fn build_default_name(
        &self,
        scope: ScopeId,
        sym: SymbolId,
        base: &mut int4,
        vn: Option<&VnRepresentative>,
        arch: &dyn DatabaseArch,
    ) -> KunaResult<String> {
        use std::fmt::Write;
        let category = self.symbols[sym].category;
        if arch.name_style_angr() {
            // (kuna) angr-style default naming: aN / dat_ / vN
            if category == symbol_category::FUNCTION_PARAMETER {
                let nm = kuna_arg_name(self.symbols[sym].get_category_index() as int4);
                return self.make_name_unique(scope, &nm);
            }
            let mut naddr = Address::new_invalid();
            let mut nflags: uint4 = 0;
            if let Some(rep) = vn {
                if !rep.is_constant {
                    naddr = rep.addr.clone();
                    nflags = rep.flags;
                }
            } else if self.symbols[sym].num_entries() != 0 {
                let eref = self.symbols[sym].mapentry[0];
                let entry = self.entry(scope, eref);
                naddr = entry.get_addr().clone();
                nflags = if entry.get_first_use_address().is_invalid() {
                    varnode_flags::addrtied
                } else {
                    0
                };
            }
            if !naddr.is_invalid() && (nflags & varnode_flags::persist) != 0 {
                let sz = self.symbols[sym].dtype.as_ref().map(|t| t.get_size()).unwrap_or(1);
                let space = naddr.get_space().expect("persist addr has a space");
                if arch
                    .translate()
                    .get_register_name(space, naddr.get_offset(), sz)
                    .is_empty()
                {
                    // global data -> dat_<addr>
                    return self.make_name_unique(scope, &kuna_global_data_name(&naddr));
                }
            }
            // everything else -> vN
            let mut s = String::new();
            let _ = write!(s, "v{base}");
            *base += 1;
            return self.make_name_unique(scope, &s);
        }

        // ---- upstream (ghidra) branch ----
        if let Some(rep) = vn {
            if !rep.is_constant {
                // C++ uses vn->getUsePoint(*fd) / vn->getHigh()->isInput(); the
                // representative carries the usepoint and is_input it precomputed.
                let usepoint = rep.usepoint.clone();
                let dt = self.symbols[sym].dtype.clone();
                if category == symbol_category::FUNCTION_PARAMETER || rep.is_input {
                    let mut index: int4 = -1;
                    if category == symbol_category::FUNCTION_PARAMETER {
                        index = self.symbols[sym].get_category_index() as int4 + 1;
                    }
                    return self.build_variable_name(
                        scope,
                        &rep.addr,
                        &usepoint,
                        dt.as_deref(),
                        &mut index,
                        rep.flags | varnode_flags::input,
                        arch,
                    );
                }
                let mut idx = *base;
                let nm = self.build_variable_name(
                    scope,
                    &rep.addr,
                    &usepoint,
                    dt.as_deref(),
                    &mut idx,
                    rep.flags,
                    arch,
                )?;
                *base = idx;
                return Ok(nm);
            }
        }
        if self.symbols[sym].num_entries() != 0 {
            let eref = self.symbols[sym].mapentry[0];
            let (addr, usepoint) = {
                let entry = self.entry(scope, eref);
                (entry.get_addr().clone(), entry.get_first_use_address())
            };
            let mut flags = if usepoint.is_invalid() { varnode_flags::addrtied } else { 0 };
            let dt = self.symbols[sym].dtype.clone();
            if category == symbol_category::FUNCTION_PARAMETER {
                flags |= varnode_flags::input;
                let mut index = self.symbols[sym].get_category_index() as int4 + 1;
                return self.build_variable_name(
                    scope,
                    &addr,
                    &usepoint,
                    dt.as_deref(),
                    &mut index,
                    flags,
                    arch,
                );
            }
            let mut idx = *base;
            let nm = self.build_variable_name(
                scope,
                &addr,
                &usepoint,
                dt.as_deref(),
                &mut idx,
                flags,
                arch,
            )?;
            *base = idx;
            return Ok(nm);
        }
        // Should never reach here (C++).
        let dt = self.symbols[sym].dtype.clone();
        let mut idx = *base;
        let nm = self.build_variable_name(
            scope,
            &Address::new_invalid(),
            &Address::new_invalid(),
            dt.as_deref(),
            &mut idx,
            0,
            arch,
        )?;
        *base = idx;
        Ok(nm)
    }
}

/// A precomputed representative-Varnode view for [`Database::build_default_name`].
///
/// SEAM(W5/W7): the C++ `buildDefaultName(sym, base, Varnode *vn)` reads
/// `vn->getAddr()`, `vn->getFlags()`, `vn->isConstant()`, `vn->getUsePoint(*fd)`,
/// and `vn->getHigh()->isInput()` — all from the function-data / merge
/// subsystems (later waves).  The caller in those waves materializes those
/// reads into this struct so `database.rs` stays decoupled from them.
#[derive(Debug, Clone)]
pub struct VnRepresentative {
    /// `vn->getAddr()`.
    pub addr: Address,
    /// `vn->getFlags()`.
    pub flags: uint4,
    /// `vn->isConstant()`.
    pub is_constant: bool,
    /// `vn->getUsePoint(*fd)` (invalid if address-tied or no function).
    pub usepoint: Address,
    /// `vn->getHigh()->isInput()`.
    pub is_input: bool,
}

// ===========================================================================
// Database: resolution depth + qualified-name scope resolution
// ===========================================================================

impl Database {
    /// C++ `Symbol::getResolutionDepth` (`database.cc:324-361`): how many
    /// namespace-path elements are needed to distinguish `sym` from a use scope.
    ///
    /// (The C++ caches the result on the Symbol via `mutable depthScope`/
    /// `depthResolution`; the arena does the same computation freshly each call —
    /// the cache is a pure optimization with no observable effect on the result.)
    pub fn get_resolution_depth(&self, sym: SymbolId, use_scope: Option<ScopeId>) -> int4 {
        let sym_scope = self.symbols[sym].scope;
        if Some(sym_scope) == use_scope {
            return 0; // Symbol is in scope where it is used
        }
        let use_scope = match use_scope {
            None => {
                // Treat null useScope as resolving the full path.
                let mut point = Some(sym_scope);
                let mut count = 0;
                while let Some(p) = point {
                    count += 1;
                    point = self.scopes[p].parent;
                }
                return count - 1; // Don't print global scope
            }
            Some(u) => u,
        };
        let distinguish_scope = self.find_distinguishing_scope(sym_scope, use_scope);
        let mut depth_resolution = 0;
        let distinguish_name: String;
        let terminating_scope: Option<ScopeId>;
        match distinguish_scope {
            None => {
                // Symbol scope is ancestor of use scope.
                distinguish_name = self.symbols[sym].name.clone();
                terminating_scope = Some(sym_scope);
            }
            Some(ds) => {
                distinguish_name = self.scopes[ds].name.clone();
                let mut current = sym_scope;
                while current != ds {
                    depth_resolution += 1; // Print its name
                    current = self.scopes[current].parent.expect("path reaches distinguish");
                }
                depth_resolution += 1; // Also print the distinguishing scope name
                terminating_scope = self.scopes[ds].parent;
            }
        }
        if self.is_name_used(use_scope, &distinguish_name, terminating_scope) {
            depth_resolution += 1; // Name overridden, need one more
        }
        depth_resolution
    }

    /// C++ `Database::resolveScopeFromSymbolName` (`database.cc:3139-3163`): the
    /// Scope (and base name) for a qualified Symbol name, using `delim`.
    /// Returns `(scope, basename)`; `scope` is `None` if a path element was bad.
    pub fn resolve_scope_from_symbol_name(
        &self,
        fullname: &str,
        delim: &str,
        start: Option<ScopeId>,
    ) -> (Option<ScopeId>, String) {
        let mut start = start.or(self.globalscope);
        let mut mark = 0usize;
        loop {
            let endmark = match fullname[mark..].find(delim) {
                None => break,
                Some(rel) => mark + rel,
            };
            if endmark == 0 {
                // Path is "absolute"
                start = self.globalscope;
            } else {
                let scopename = &fullname[mark..endmark];
                start = match start {
                    Some(s) => self.resolve_child_scope(s, scopename, self.id_by_name_hash),
                    None => None,
                };
                if start.is_none() {
                    return (None, String::new());
                }
            }
            mark = endmark + delim.len();
        }
        let basename = fullname[mark..].to_string();
        (start, basename)
    }

    /// C++ `Database::findCreateScopeFromSymbolName` (`database.cc:3177-3197`):
    /// like `resolve_scope_from_symbol_name` but creates missing namespaces.
    /// Requires `id_by_name_hash` (else a `LowlevelError`).
    pub fn find_create_scope_from_symbol_name(
        &mut self,
        fullname: &str,
        delim: &str,
        start: Option<ScopeId>,
        num_spaces: int4,
    ) -> KunaResult<(ScopeId, String)> {
        let mut start = start.or(self.globalscope).expect("a global scope exists");
        let mut mark = 0usize;
        loop {
            let endmark = match fullname[mark..].find(delim) {
                None => break,
                Some(rel) => mark + rel,
            };
            if !self.id_by_name_hash {
                return Err(KunaError::lowlevel("Scope name hashes not allowed"));
            }
            let scopename = fullname[mark..endmark].to_string();
            let name_id = Scope::hash_scope_name(self.scopes[start].unique_id, &scopename);
            start = self.find_create_scope(name_id, &scopename, Some(start), num_spaces)?;
            mark = endmark + delim.len();
        }
        let basename = fullname[mark..].to_string();
        Ok((start, basename))
    }
}

// ===========================================================================
// Database: rename / remove / assignDefaultNames
// ===========================================================================

impl Database {
    /// C++ `ScopeInternal::renameSymbol` (`database.cc:2180-2192`): rename `sym`
    /// within its scope, re-keying the name tree (and the multi-entry set).
    pub fn rename_symbol(&mut self, sym: SymbolId, newname: &str) -> KunaResult<()> {
        let scope = self.symbols[sym].scope;
        let oldkey = self.name_key(sym);
        self.scopes[scope].nametree.remove(&oldkey);
        let multi = self.symbols[sym].whole_count > 1;
        if multi {
            self.scopes[scope].multi_entry_set.remove(&oldkey);
        }
        self.symbols[sym].name = newname.to_string();
        self.symbols[sym].display_name = newname.to_string();
        self.insert_name_tree(scope, sym)?;
        if multi {
            let newkey = self.name_key(sym);
            self.scopes[scope].multi_entry_set.insert(newkey, sym);
        }
        Ok(())
    }

    /// C++ `ScopeInternal::retypeSymbol(Symbol *sym,Datatype *ct)`
    /// (`database.cc`): change the data-type of `sym`.  If the new type has the
    /// same size as the old (or the symbol has no storage mappings) the type is
    /// swapped in place and the size/type lock recomputed; otherwise, for a
    /// single address-tied mapping, the mapping is rebuilt at the new size.
    pub fn retype_symbol(&mut self, sym: SymbolId, ct: Rc<Datatype>) -> KunaResult<()> {
        // if (ct->hasStripped()) ct = ct->getStripped();
        let ct = if ct.has_stripped() { ct.get_stripped().unwrap_or(ct) } else { ct };
        let old_size = self.symbols[sym].dtype.as_ref().map(|t| t.get_size()).unwrap_or(0);
        let mapentry_len = self.symbols[sym].mapentry.len();
        // if ((sym->type->getSize() == ct->getSize()) || (sym->mapentry.empty()))
        if old_size == ct.get_size() || mapentry_len == 0 {
            self.symbols[sym].dtype = Some(ct);
            self.symbols[sym].check_size_type_lock();
            return Ok(());
        }
        // else if (sym->mapentry.size()==1)
        if mapentry_len == 1 {
            let eref = self.symbols[sym].mapentry[0];
            let scope = self.symbols[sym].scope;
            // C++ `SymbolEntry::isAddrTied()` reads `symbol->getFlags() &
            // Varnode::addrtied`.
            let addr_tied = (self.symbols[sym].get_flags() & varnode_flags::addrtied) != 0;
            let addr = self.entry(scope, eref).get_addr().clone();
            if addr_tied {
                // Remove the single map entry (C++ erases the rangemap iterator,
                // pops mapentry, zeroes wholeCount), swap the type, then re-add the
                // map point at the new size.
                self.erase_mapentry(scope, sym, eref);
                self.symbols[sym].mapentry.clear();
                self.symbols[sym].whole_count = 0;
                self.symbols[sym].dtype = Some(ct);
                self.symbols[sym].check_size_type_lock();
                self.add_map_point(scope, sym, &addr, &Address::new_invalid())?;
                return Ok(());
            }
        }
        Err(KunaError::recov(format!(
            "Unable to retype symbol: {}",
            self.symbols[sym].name
        )))
    }

    /// Erase one SymbolEntry mapping of `sym` from its scope rangemap / dynamic
    /// list (the per-entry half of [`remove_symbol_mappings`]).
    fn erase_mapentry(&mut self, scope: ScopeId, _sym: SymbolId, eref: EntryRef) {
        match eref {
            EntryRef::Dynamic(slot) => {
                self.scopes[scope].dynamicentry[slot] = None;
            }
            EntryRef::Mapped { space_index, idx } => {
                if let Some(map) = self.scopes[scope].maptable[space_index].as_mut() {
                    map.erase(idx);
                }
            }
        }
    }

    /// C++ `ScopeInternal::removeSymbolMappings` (`database.cc:2145-2164`): drop
    /// all SymbolEntry mappings of `sym`.
    fn remove_symbol_mappings(&mut self, sym: SymbolId) {
        let scope = self.symbols[sym].scope;
        if self.symbols[sym].whole_count > 1 {
            let key = self.name_key(sym);
            self.scopes[scope].multi_entry_set.remove(&key);
        }
        let entries = std::mem::take(&mut self.symbols[sym].mapentry);
        for eref in entries {
            match eref {
                EntryRef::Dynamic(slot) => {
                    self.scopes[scope].dynamicentry[slot] = None;
                }
                EntryRef::Mapped { space_index, idx } => {
                    if let Some(map) = self.scopes[scope].maptable[space_index].as_mut() {
                        map.erase(idx);
                    }
                }
            }
        }
        self.symbols[sym].whole_count = 0;
    }

    /// C++ `ScopeInternal::removeSymbol` (`database.cc:2166-2178`): remove `sym`
    /// entirely (category slot, mappings, name tree, and the object).
    pub fn remove_symbol(&mut self, sym: SymbolId) {
        let scope = self.symbols[sym].scope;
        let cat = self.symbols[sym].category;
        if cat >= 0 {
            let cat = cat as usize;
            let catindex = self.symbols[sym].catindex as usize;
            let list = &mut self.scopes[scope].category[cat];
            list[catindex] = None;
            while matches!(list.last(), Some(None)) {
                list.pop();
            }
        }
        self.remove_symbol_mappings(sym);
        let key = self.name_key(sym);
        self.scopes[scope].nametree.remove(&key);
        self.symbols.remove(sym);
    }

    /// C++ `ScopeInternal::clearUnlockedCategory` (`database.cc:2099`) for the
    /// `cat < 0` case (the only one `ScopeLocal::restructureVarnode` reaches with
    /// `-1`).  Every symbol with `category < 0` (an uncategorized local — the
    /// auto-recovered stack variables) is removed unless it is type-locked, in
    /// which case its unlocked name is reset to an undefined default but the symbol
    /// is kept.
    ///
    /// This is the per-pass reset that lets `restructureVarnode` re-derive the
    /// stack-frame layout from scratch each time it runs: without it, a spurious
    /// open-array Symbol created on an early pass (before `RuleStoreVarnode` has
    /// folded the STORE into a sized stack COPY) persists and `gatherSymbols`
    /// re-injects it as a competing fixed-array `RangeHint`, overriding the scalar
    /// hint the converted Varnode then supplies.
    pub fn clear_unlocked_category_negative(&mut self, scope: ScopeId) -> KunaResult<()> {
        // C++ iterates `nametree` advancing the iterator before acting on the
        // symbol (the act mutates the tree).  Collect first, then act — the arena
        // equivalent of "advance before remove".
        let candidates: Vec<SymbolId> = self.scopes[scope]
            .nametree
            .values()
            .copied()
            .filter(|&sid| self.symbols[sid].get_category() < 0)
            .collect();
        for sid in candidates {
            if self.symbols[sid].is_type_locked() {
                // Only hold if TYPE locked; clear an unlocked, defined name.
                if !self.symbols[sid].is_name_locked() && !self.symbols[sid].is_name_undefined() {
                    let newname = self.build_undefined_name(scope)?;
                    self.rename_symbol(sid, &newname)?;
                }
            } else {
                self.remove_symbol(sid);
            }
        }
        Ok(())
    }

    /// C++ `ScopeInternal::assignDefaultNames` (`database.cc:2880-2895`): rename
    /// every symbol with an undefined name to a generated default.
    pub fn assign_default_names(
        &mut self,
        scope: ScopeId,
        base: &mut int4,
        arch: &dyn DatabaseArch,
    ) -> KunaResult<()> {
        use std::ops::Bound::{Excluded, Unbounded};
        // C++: iter = nametree.upper_bound(Symbol("$$undef")); walk while names
        // are still "undefined".  Collect the undefined-named ids first (the
        // rename mutates the tree under iteration in C++ via "advance before
        // rename"; collecting up-front is the safe arena equivalent).
        // C++ probe is Symbol("$$undef") with nameDedup=0; upper_bound is the
        // first key strictly greater than ("$$undef", 0).
        let probe = NameKey { name: "$$undef".to_string(), name_dedup: 0 };
        let undefined: Vec<SymbolId> = self.scopes[scope]
            .nametree
            .range((Excluded(probe), Unbounded))
            .take_while(|(k, _)| k.name.len() == 15 && k.name.starts_with("$$undef"))
            .map(|(_, &sid)| sid)
            .collect();
        for sid in undefined {
            let nm = self.build_default_name(scope, sid, base, None, arch)?;
            self.rename_symbol(sid, &nm)?;
        }
        Ok(())
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace,
        UniqueSpace};

    /// A simple test data-type of the given size (TYPE_UNKNOWN unless given).
    fn dt(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    /// Build a manager: const(0), unique(1), ram(2), register(3).
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2,
            addrspace_flags::hasphysical, 1, 1,
        )))
        .unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0,
        )))
        .unwrap();
        m.set_default_code_space(2).unwrap();
        m
    }

    fn space(m: &AddrSpaceManager, idx: i32) -> Rc<AddrSpace> {
        Rc::clone(m.get_space(idx).unwrap())
    }

    /// A test architecture seam (register names + namestyle toggle).
    struct TestArch {
        num_spaces: int4,
        angr: bool,
        /// Map of (space_index, offset, size) -> register name.
        regnames: BTreeMap<(int4, uintb, int4), String>,
    }

    struct TestTypes;
    impl TypeFactorySeam for TestTypes {
        fn get_base(&self, size: int4, meta: type_metatype) -> Rc<Datatype> {
            Rc::new(Datatype::new(size, meta))
        }
        fn get_type_code(&self) -> Rc<Datatype> {
            Rc::new(Datatype::new(1, type_metatype::TYPE_CODE))
        }
    }

    impl TranslateSeam for TestArch {
        fn get_register_name(&self, sp: &Rc<AddrSpace>, off: uintb, size: int4) -> String {
            self.regnames
                .get(&(sp.get_index(), off, size))
                .cloned()
                .unwrap_or_default()
        }
    }

    impl DatabaseArch for TestArch {
        fn num_spaces(&self) -> int4 {
            self.num_spaces
        }
        fn types(&self) -> &dyn TypeFactorySeam {
            &TestTypes
        }
        fn translate(&self) -> &dyn TranslateSeam {
            self
        }
        fn min_funcsymbol_size(&self) -> int4 {
            1
        }
        fn name_style_angr(&self) -> bool {
            self.angr
        }
        fn type_name_base(&self, _dt: &Datatype) -> String {
            String::new() // base Datatype seam has no name (SEAM(W6))
        }
    }

    fn test_arch(num_spaces: int4, angr: bool) -> TestArch {
        TestArch { num_spaces, angr, regnames: BTreeMap::new() }
    }

    /// Build a database with an attached global scope.  Returns (db, global id).
    fn db_with_global(num_spaces: int4) -> (Database, ScopeId) {
        let mut db = Database::new(true);
        // Global scope: empty name, id 0.
        let g = db.build_sub_scope(0, "", num_spaces);
        db.attach_scope(g, None).unwrap();
        (db, g)
    }

    // --- hashScopeName / crc ---

    #[test]
    fn hash_scope_name_is_deterministic_and_namesensitive() {
        let a = Scope::hash_scope_name(0, "foo");
        let b = Scope::hash_scope_name(0, "foo");
        let c = Scope::hash_scope_name(0, "bar");
        let d = Scope::hash_scope_name(5, "foo");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    // --- scope tree resolution ---

    #[test]
    fn scope_tree_resolution_matrix() {
        let (mut db, g) = db_with_global(4);
        // Build global::ns1::ns2 and global::ns3.
        let id1 = Scope::hash_scope_name(0, "ns1");
        let ns1 = db.find_create_scope(id1, "ns1", Some(g), 4).unwrap();
        let id2 = Scope::hash_scope_name(id1, "ns2");
        let ns2 = db.find_create_scope(id2, "ns2", Some(ns1), 4).unwrap();
        let id3 = Scope::hash_scope_name(0, "ns3");
        let ns3 = db.find_create_scope(id3, "ns3", Some(g), 4).unwrap();

        // resolve_child_scope by name (strategy = hash).
        assert_eq!(db.resolve_child_scope(g, "ns1", true), Some(ns1));
        assert_eq!(db.resolve_child_scope(ns1, "ns2", true), Some(ns2));
        assert_eq!(db.resolve_child_scope(g, "ns3", true), Some(ns3));
        assert_eq!(db.resolve_child_scope(g, "nope", true), None);

        // full name.
        assert_eq!(db.get_full_name(g), "");
        assert_eq!(db.get_full_name(ns1), "ns1");
        assert_eq!(db.get_full_name(ns2), "ns1::ns2");
        assert_eq!(db.get_full_name(ns3), "ns3");

        // scope path (global-first).
        assert_eq!(db.get_scope_path(ns2), vec![g, ns1, ns2]);
        assert_eq!(db.get_scope_path(g), vec![g]);

        // sub-scope.
        assert!(db.is_sub_scope(ns2, ns1));
        assert!(db.is_sub_scope(ns2, g));
        assert!(!db.is_sub_scope(ns1, ns2));
        assert!(!db.is_sub_scope(ns3, ns1));

        // distinguishing scope.
        assert_eq!(db.find_distinguishing_scope(ns2, ns2), None); // same
        assert_eq!(db.find_distinguishing_scope(ns1, g), Some(ns1)); // parent==op2
        assert_eq!(db.find_distinguishing_scope(g, ns1), None); // op2.parent==this
        // ns2 vs ns3: paths [g,ns1,ns2] vs [g,ns3]; first differing index 1 -> ns1.
        assert_eq!(db.find_distinguishing_scope(ns2, ns3), Some(ns1));
    }

    #[test]
    fn resolve_scope_from_symbol_name_walks_path() {
        let (mut db, g) = db_with_global(4);
        let id1 = Scope::hash_scope_name(0, "ns1");
        let ns1 = db.find_create_scope(id1, "ns1", Some(g), 4).unwrap();
        let id2 = Scope::hash_scope_name(id1, "ns2");
        let ns2 = db.find_create_scope(id2, "ns2", Some(ns1), 4).unwrap();

        let (sc, base) = db.resolve_scope_from_symbol_name("ns1::ns2::foo", "::", Some(g));
        assert_eq!(sc, Some(ns2));
        assert_eq!(base, "foo");

        // absolute (leading delim) resets to global.
        let (sc, base) = db.resolve_scope_from_symbol_name("::ns1::bar", "::", Some(ns2));
        assert_eq!(sc, Some(ns1));
        assert_eq!(base, "bar");

        // bad path element.
        let (sc, _) = db.resolve_scope_from_symbol_name("ns1::nope::x", "::", Some(g));
        assert_eq!(sc, None);

        // no delimiter: scope unchanged, whole name is base.
        let (sc, base) = db.resolve_scope_from_symbol_name("plain", "::", Some(ns1));
        assert_eq!(sc, Some(ns1));
        assert_eq!(base, "plain");
    }

    #[test]
    fn find_create_scope_from_symbol_name_creates_missing() {
        let (mut db, g) = db_with_global(4);
        let (sc, base) = db
            .find_create_scope_from_symbol_name("a::b::c::sym", "::", Some(g), 4)
            .unwrap();
        assert_eq!(base, "sym");
        // a::b::c should now resolve.
        assert_eq!(db.get_full_name(sc), "a::b::c");
        // Idempotent: a second call returns the same scope.
        let (sc2, _) = db
            .find_create_scope_from_symbol_name("a::b::c::other", "::", Some(g), 4)
            .unwrap();
        assert_eq!(sc, sc2);
    }

    // --- name dedup + uniquify ---

    #[test]
    fn name_dedup_assigns_increasing_ids() {
        let (mut db, g) = db_with_global(4);
        let a = db.add_symbol(g, "dup", dt(4)).unwrap();
        let b = db.add_symbol(g, "dup", dt(4)).unwrap();
        let c = db.add_symbol(g, "dup", dt(4)).unwrap();
        assert_eq!(db.symbol(a).name_dedup, 0);
        assert_eq!(db.symbol(b).name_dedup, 1);
        assert_eq!(db.symbol(c).name_dedup, 2);
        // findByName returns all three in dedup order.
        let res = db.find_by_name(g, "dup");
        assert_eq!(res, vec![a, b, c]);
    }

    #[test]
    fn make_name_unique_sequences() {
        let (mut db, g) = db_with_global(4);
        // No collision -> unchanged.
        assert_eq!(db.make_name_unique(g, "fresh").unwrap(), "fresh");
        // One symbol named base -> first unique is base_00.
        db.add_symbol(g, "base", dt(4)).unwrap();
        assert_eq!(db.make_name_unique(g, "base").unwrap(), "base_00");
        // Add base_00, next is base_01.
        db.add_symbol(g, "base_00", dt(4)).unwrap();
        assert_eq!(db.make_name_unique(g, "base").unwrap(), "base_01");
    }

    #[test]
    fn build_undefined_name_sequence() {
        let (mut db, g) = db_with_global(4);
        // No undefined names yet.
        assert_eq!(db.build_undefined_name(g).unwrap(), "$$undef00000000");
        // A symbol created with empty name gets the undefined name.
        let s = db.add_symbol(g, "", dt(4)).unwrap();
        assert_eq!(db.symbol(s).name, "$$undef00000000");
        // Next undefined increments.
        assert_eq!(db.build_undefined_name(g).unwrap(), "$$undef00000001");
    }

    // --- SymbolEntry range queries ---

    #[test]
    fn symbol_entry_range_queries() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());

        // Symbol A: 4 bytes at ram:0x1000.
        let a_addr = Address::new(Rc::clone(&ram), 0x1000);
        let (a, _) = db
            .add_symbol_mapped(g, "A", dt(4), &a_addr, &Address::new_invalid())
            .unwrap();
        // Symbol B: 8 bytes at ram:0x2000.
        let b_addr = Address::new(Rc::clone(&ram), 0x2000);
        let (b, _) = db
            .add_symbol_mapped(g, "B", dt(8), &b_addr, &Address::new_invalid())
            .unwrap();

        // findAddr: exact hit for A and B; miss in the gap.
        let inv = Address::new_invalid();
        let ea = db.find_addr(g, &a_addr, &inv).expect("A found");
        assert_eq!(db.entry(g, ea).symbol, a);
        let eb = db.find_addr(g, &b_addr, &inv).expect("B found");
        assert_eq!(db.entry(g, eb).symbol, b);
        assert!(db.find_addr(g, &Address::new(Rc::clone(&ram), 0x1500), &inv).is_none());
        // mid-symbol address is not an exact-address hit for findAddr.
        assert!(db
            .find_addr(g, &Address::new(Rc::clone(&ram), 0x1002), &inv)
            .is_none());

        // findContainer: a 2-byte access at 0x1000 is contained in A.
        let ec = db
            .find_container(g, &a_addr, 2, &inv)
            .expect("container for 2-byte access");
        assert_eq!(db.entry(g, ec).symbol, a);
        // access fully past A is not contained.
        assert!(db
            .find_container(g, &Address::new(Rc::clone(&ram), 0x1003), 4, &inv)
            .is_none());

        // findOverlap: an access straddling A's start overlaps A.
        let eo = db
            .find_overlap(g, &Address::new(Rc::clone(&ram), 0x0ffe), 4)
            .expect("overlap with A");
        assert_eq!(db.entry(g, eo).symbol, a);
        assert!(db
            .find_overlap(g, &Address::new(Rc::clone(&ram), 0x1004), 4)
            .is_none());
    }

    #[test]
    fn find_closest_fit_picks_tightest() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());
        let base = Address::new(Rc::clone(&ram), 0x100);
        // Two overlapping symbols starting at 0x100: 8 bytes and 4 bytes.
        let (big, _) = db.add_symbol_mapped(g, "big", dt(8), &base, &Address::new_invalid()).unwrap();
        let (small, _) =
            db.add_symbol_mapped(g, "small", dt(4), &base, &Address::new_invalid()).unwrap();
        let inv = Address::new_invalid();
        // Closest fit to a 4-byte access at 0x100 is the 4-byte symbol.
        let e = db.find_closest_fit(g, &base, 4, &inv).expect("closest fit");
        assert_eq!(db.entry(g, e).symbol, small);
        // Closest fit to a 6-byte access: small (size 4) is too small (newdiff
        // negative), big (size 8) gives newdiff +2 which wins over -2.
        let e = db.find_closest_fit(g, &base, 6, &inv).expect("closest fit 6");
        assert_eq!(db.entry(g, e).symbol, big);
    }

    #[test]
    fn entry_subsort_ordering() {
        // EntrySubsort orders by useindex then useoffset; minimal < real < maximal.
        let mn = EntrySubsort::minimal();
        let mx = EntrySubsort::maximal();
        let lo = EntrySubsort { useindex: 2, useoffset: 0x10 };
        let hi = EntrySubsort { useindex: 2, useoffset: 0x20 };
        assert!(mn < lo);
        assert!(lo < hi);
        assert!(hi < mx);
        assert!(EntrySubsort { useindex: 1, useoffset: 0xffff } < lo);
    }

    // --- query walk through scope hierarchy ---

    #[test]
    fn query_by_name_walks_to_parent() {
        let (mut db, g) = db_with_global(4);
        let id1 = Scope::hash_scope_name(0, "ns1");
        let ns1 = db.find_create_scope(id1, "ns1", Some(g), 4).unwrap();
        let gx = db.add_symbol(g, "x", dt(4)).unwrap();
        // Looking up "x" from ns1 (which has no "x") recurses to global.
        assert_eq!(db.query_by_name(ns1, "x"), vec![gx]);
        // A local "x" in ns1 shadows the global one.
        let nx = db.add_symbol(ns1, "x", dt(4)).unwrap();
        assert_eq!(db.query_by_name(ns1, "x"), vec![nx]);
    }

    #[test]
    fn is_name_used_respects_terminator_and_global_cutoff() {
        let (mut db, g) = db_with_global(4);
        let id1 = Scope::hash_scope_name(0, "ns1");
        let ns1 = db.find_create_scope(id1, "ns1", Some(g), 4).unwrap();
        let id2 = Scope::hash_scope_name(id1, "ns2");
        let ns2 = db.find_create_scope(id2, "ns2", Some(ns1), 4).unwrap();
        db.add_symbol(ns1, "name", dt(4)).unwrap();

        // From ns2, "name" is used in ancestor ns1.
        assert!(db.is_name_used(ns2, "name", None));
        // With ns1 as terminator, the ns1 scope is not consulted.
        assert!(!db.is_name_used(ns2, "name", Some(ns1)));
        // A name only in global is never found (global is the recursion cutoff).
        db.add_symbol(g, "globalonly", dt(4)).unwrap();
        assert!(!db.is_name_used(ns2, "globalonly", None));
    }

    // --- default name generation: both namestyle branches ---

    #[test]
    fn build_default_name_angr_branch() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());
        let arch = test_arch(m.num_spaces(), true); // angr

        // A function parameter -> a<catindex>.
        let p = db.add_symbol(g, "p", dt(4)).unwrap();
        db.symbol_mut(p).category = symbol_category::FUNCTION_PARAMETER;
        db.symbol_mut(p).catindex = 2;
        let mut base = 0;
        assert_eq!(db.build_default_name(g, p, &mut base, None, &arch).unwrap(), "a2");

        // A persistent global at ram with no register name -> dat_<addr>.
        let a_addr = Address::new(Rc::clone(&ram), 0x4010);
        let (gd, _) = db.add_symbol_mapped(g, "gd", dt(4), &a_addr, &Address::new_invalid()).unwrap();
        // add_map set persist (global scope) and addrtied (empty uselimit).
        assert!(db.symbol(gd).flags & varnode_flags::persist != 0);
        let rep = VnRepresentative {
            addr: a_addr.clone(),
            flags: varnode_flags::persist,
            is_constant: false,
            usepoint: Address::new_invalid(),
            is_input: false,
        };
        let mut base = 0;
        let nm = db.build_default_name(g, gd, &mut base, Some(&rep), &arch).unwrap();
        assert_eq!(nm, "dat_4010");

        // A plain local (no persist) -> vN, advancing base.
        let loc = db.add_symbol(g, "loc", dt(4)).unwrap();
        let mut base = 7;
        let nm = db.build_default_name(g, loc, &mut base, None, &arch).unwrap();
        assert_eq!(nm, "v7");
        assert_eq!(base, 8);
    }

    #[test]
    fn clear_unlocked_category_negative_drops_unlocked_locals_keeps_locked_and_categorized() {
        // C++ `ScopeInternal::clearUnlockedCategory(-1)` (database.cc:2099, cat<0):
        // the per-pass reset at the head of `ScopeLocal::restructureVarnode` that
        // lets the stack-frame layout be re-derived from scratch each pass.
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());

        // An unlocked auto-recovered local (category < 0, no type/name lock): the
        // spurious open-array kind that must be dropped each pass.
        let a_addr = Address::new(Rc::clone(&ram), 0x100);
        let (local, _) = db
            .add_symbol_mapped(g, "", dt(4), &a_addr, &Address::new_invalid())
            .unwrap();
        // A type+name-locked local (e.g. a `map addr` symbol): must survive.
        let b_addr = Address::new(Rc::clone(&ram), 0x200);
        let (locked, _) = db
            .add_symbol_mapped(g, "kept", dt(4), &b_addr, &Address::new_invalid())
            .unwrap();
        db.set_attribute(locked, varnode_flags::typelock | varnode_flags::namelock);
        // A categorized parameter (category >= 0): the cat<0 walk must skip it.
        let c_addr = Address::new(Rc::clone(&ram), 0x300);
        let (param, _) = db
            .add_symbol_mapped(g, "p0", dt(4), &c_addr, &Address::new_invalid())
            .unwrap();
        db.set_category(g, param, symbol_category::FUNCTION_PARAMETER, 0);

        assert!(db.symbols.contains_key(local));
        db.clear_unlocked_category_negative(g).unwrap();

        // The unlocked local is gone; the locked local and the parameter survive.
        assert!(!db.symbols.contains_key(local), "unlocked local must be removed");
        assert!(db.symbols.contains_key(locked), "type/name-locked symbol kept");
        assert!(db.symbols.contains_key(param), "categorized parameter kept");
        // The mapping of the removed local is gone too (no stale EntryMap range).
        assert!(db.find_overlap(g, &a_addr, 4).is_none());
        assert!(db.find_overlap(g, &b_addr, 4).is_some());
    }

    // VERIFIER adversarial (w10-sized-stackvn): clearUnlockedCategory(-1) C++
    // database.cc:2120-2136 — the type-locked arm.  A TYPE-locked but NOT
    // name-locked symbol is KEPT; if its name is *defined* it is reset to an
    // undefined default; if already undefined it is left untouched.  A
    // type+name-locked symbol keeps its (defined) name.  Mirrors the three C++
    // sub-cases the porter's single test did not separate.
    #[test]
    fn verify_w10_sized_stackvn_clear_unlocked_typelocked_name_handling() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());
        let inv = Address::new_invalid();

        // (a) type-locked, NOT name-locked, DEFINED name -> kept, name reset to
        //     an undefined default ($$undef...).
        let a_addr = Address::new(Rc::clone(&ram), 0x100);
        let (a, _) = db.add_symbol_mapped(g, "defined_a", dt(4), &a_addr, &inv).unwrap();
        db.set_attribute(a, varnode_flags::typelock); // type lock only

        // (b) type-locked, NOT name-locked, ALREADY-undefined name -> kept, name
        //     unchanged (the C++ `if (!isNameUndefined())` skip).
        let b_addr = Address::new(Rc::clone(&ram), 0x200);
        let (b, _) = db.add_symbol_mapped(g, "tmpB", dt(4), &b_addr, &inv).unwrap();
        db.rename_symbol(b, "$$undef00000000").unwrap();
        db.set_attribute(b, varnode_flags::typelock);
        assert!(db.symbol(b).is_name_undefined());

        // (c) type+name-locked, DEFINED name -> kept, name unchanged.
        let c_addr = Address::new(Rc::clone(&ram), 0x300);
        let (c, _) = db.add_symbol_mapped(g, "named_c", dt(4), &c_addr, &inv).unwrap();
        db.set_attribute(c, varnode_flags::typelock | varnode_flags::namelock);

        db.clear_unlocked_category_negative(g).unwrap();

        // All three survive (type-locked are never removed by the cat<0 arm).
        assert!(db.symbols.contains_key(a), "(a) type-locked symbol must be kept");
        assert!(db.symbols.contains_key(b), "(b) type-locked symbol must be kept");
        assert!(db.symbols.contains_key(c), "(c) type+name-locked symbol must be kept");
        // (a) defined name was reset to an undefined default.
        assert!(
            db.symbol(a).is_name_undefined(),
            "(a) an unlocked, defined name on a type-locked symbol must be reset to \
             $$undef..., got {:?}",
            db.symbol(a).get_name()
        );
        // (b) already-undefined name untouched (still the SAME $$undef00000000).
        assert_eq!(
            db.symbol(b).get_name(),
            "$$undef00000000",
            "(b) an already-undefined name must not be re-bumped"
        );
        // (c) name-locked defined name preserved verbatim.
        assert_eq!(db.symbol(c).get_name(), "named_c", "(c) name-locked name preserved");
    }

    // VERIFIER adversarial (w10-sized-stackvn): the collect-then-act ("advance
    // iterator before remove") faithfulness.  Removing MANY unlocked cat<0
    // symbols in a single pass must visit every one (no skip from mutating the
    // name-tree mid-walk) and leave the type-locked ones standing — the property
    // that breaks if a naive `for x in &tree { remove(x) }` were used.
    #[test]
    fn verify_w10_sized_stackvn_clear_unlocked_removes_all_in_one_pass() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, g) = db_with_global(m.num_spaces());
        let inv = Address::new_invalid();

        // Five unlocked locals (cat<0) at distinct stack offsets, interleaved with
        // a type-locked keeper in the middle so a "remove shifts the iterator"
        // bug would skip a neighbour.
        let mut unlocked = Vec::new();
        for i in 0..5u64 {
            let addr = Address::new(Rc::clone(&ram), 0x100 + i * 0x10);
            let (s, _) =
                db.add_symbol_mapped(g, &format!("loc{i}"), dt(4), &addr, &inv).unwrap();
            unlocked.push((s, addr));
        }
        let keep_addr = Address::new(Rc::clone(&ram), 0x108); // between loc0 and loc1
        let (keeper, _) = db.add_symbol_mapped(g, "keeper", dt(2), &keep_addr, &inv).unwrap();
        db.set_attribute(keeper, varnode_flags::typelock | varnode_flags::namelock);

        db.clear_unlocked_category_negative(g).unwrap();

        // EVERY unlocked local is gone (none skipped); the keeper stands.
        for (s, addr) in &unlocked {
            assert!(!db.symbols.contains_key(*s), "every unlocked local must be removed");
            assert!(db.find_overlap(g, addr, 4).is_none(), "its mapping must be cleared");
        }
        assert!(db.symbols.contains_key(keeper), "the type-locked keeper survives");
        assert!(db.find_overlap(g, &keep_addr, 2).is_some(), "keeper mapping intact");
    }

    #[test]
    fn build_default_name_ghidra_branch() {
        let m = build_manager();
        let ram = space(&m, 2);
        let reg = space(&m, 3);
        let (mut db, g) = db_with_global(m.num_spaces());
        let mut arch = test_arch(m.num_spaces(), false); // ghidra
        // Register at reg:0x0 size 4 -> "rax".
        arch.regnames.insert((3, 0, 4), "rax".to_string());

        // function_parameter -> param_<index+1>.
        let p = db.add_symbol(g, "p", dt(4)).unwrap();
        db.symbol_mut(p).category = symbol_category::FUNCTION_PARAMETER;
        db.symbol_mut(p).catindex = 0;
        // Map it so build_default_name has an entry to read.
        let p_addr = Address::new(Rc::clone(&reg), 0x0);
        db.add_map_point(g, p, &p_addr, &Address::new_invalid()).unwrap();
        let mut base = 0;
        // param flag + index = catindex+1 = 1 -> "param_1".
        assert_eq!(db.build_default_name(g, p, &mut base, None, &arch).unwrap(), "param_1");

        // A persistent global with a register name renders the register name.
        let addr = Address::new(Rc::clone(&reg), 0x0);
        let (gd, _) = db.add_symbol_mapped(g, "gd", dt(4), &addr, &Address::new_invalid()).unwrap();
        let rep = VnRepresentative {
            addr,
            flags: varnode_flags::persist,
            is_constant: false,
            usepoint: Address::new_invalid(),
            is_input: false,
        };
        let mut base = 0;
        let nm = db.build_default_name(g, gd, &mut base, Some(&rep), &arch).unwrap();
        assert_eq!(nm, "rax");

        // A persistent global at ram (no register) -> capitalized space + hex.
        let ram_addr = Address::new(Rc::clone(&ram), 0x1234);
        let (rd, _) =
            db.add_symbol_mapped(g, "rd", dt(4), &ram_addr, &Address::new_invalid()).unwrap();
        let rep = VnRepresentative {
            addr: ram_addr,
            flags: varnode_flags::persist,
            is_constant: false,
            usepoint: Address::new_invalid(),
            is_input: false,
        };
        let mut base = 0;
        let nm = db.build_default_name(g, rd, &mut base, Some(&rep), &arch).unwrap();
        // capitalize_first("ram")="Ram", width=2*8=16 hex digits of 0x1234.
        assert_eq!(nm, format!("Ram{:016x}", 0x1234));
    }

    // --- property ranges ---

    #[test]
    fn property_range_set_and_clear() {
        let m = build_manager();
        let ram = space(&m, 2);
        let (mut db, _g) = db_with_global(m.num_spaces());
        let a1 = Address::new(Rc::clone(&ram), 0x100);
        let a2 = Address::new(Rc::clone(&ram), 0x200); // one-past-end
        db.set_property_range(varnode_flags::readonly, &a1, &a2);
        // Inside the range -> readonly.
        assert_eq!(
            db.get_property(&Address::new(Rc::clone(&ram), 0x180)) & varnode_flags::readonly,
            varnode_flags::readonly
        );
        // At/after a2 -> not set.
        assert_eq!(db.get_property(&Address::new(Rc::clone(&ram), 0x200)), 0);
        // Below a1 -> not set.
        assert_eq!(db.get_property(&Address::new(Rc::clone(&ram), 0x0ff)), 0);

        // Clear it back.
        db.clear_property_range(varnode_flags::readonly, &a1, &a2);
        assert_eq!(db.get_property(&Address::new(Rc::clone(&ram), 0x180)), 0);
    }

    // --- equate value-close ---

    #[test]
    fn equate_value_close_transforms() {
        // identity, complement, negation, +/-1.
        assert!(equate_is_value_close(5, 5, 4));
        assert!(equate_is_value_close(5, !5u64, 4)); // complement
        assert!(equate_is_value_close(5, 5u64.wrapping_neg(), 4)); // negation
        assert!(equate_is_value_close(5, 4, 4)); // value-1
        assert!(equate_is_value_close(5, 6, 4)); // value+1
        assert!(!equate_is_value_close(5, 100, 4));
    }

    // --- symbol flags / dispflags ---

    #[test]
    fn symbol_flag_helpers() {
        let mut s = Symbol::new(ScopeId::default(), "s", Some(dt(4)));
        assert!(!s.is_type_locked());
        s.flags |= varnode_flags::typelock;
        assert!(s.is_type_locked());
        // size_typelock is set only when typelocked AND type is unknown.
        s.check_size_type_lock();
        assert!(s.is_size_type_locked());
        s.dtype = Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT)));
        s.check_size_type_lock();
        assert!(!s.is_size_type_locked());

        // display format.
        s.set_display_format(symbol_dispflags::FORCE_HEX);
        assert_eq!(s.get_display_format(), symbol_dispflags::FORCE_HEX);

        // isolate forces typelock.
        let mut s2 = Symbol::new(ScopeId::default(), "s2", Some(dt(4)));
        s2.set_isolated(true);
        assert!(s2.is_isolated());
        assert!(s2.is_type_locked());

        // undefined-name detection.
        let mut s3 = Symbol::new(ScopeId::default(), "$$undef00000000", Some(dt(4)));
        assert!(s3.is_name_undefined());
        s3.name = "real".to_string();
        assert!(!s3.is_name_undefined());
    }

    #[test]
    fn resolution_depth() {
        let (mut db, g) = db_with_global(4);
        let id1 = Scope::hash_scope_name(0, "ns1");
        let ns1 = db.find_create_scope(id1, "ns1", Some(g), 4).unwrap();
        let sym = db.add_symbol(ns1, "y", dt(4)).unwrap();
        // Used in its own scope -> 0.
        assert_eq!(db.get_resolution_depth(sym, Some(ns1)), 0);
        // null useScope -> full path minus global. Path is [g, ns1]: count=2-1=1.
        assert_eq!(db.get_resolution_depth(sym, None), 1);
        // Used from global: distinguishing scope is ns1 (sym's scope); the name
        // "ns1" isn't used in global, so depth is 1.
        assert_eq!(db.get_resolution_depth(sym, Some(g)), 1);
    }

    /// The per-function inline / noreturn / inject-id flags round-trip through the
    /// FunctionSymbol kind (the parked-FuncProto facts `OptionInline` /
    /// `OptionNoReturn` / `IfcFixupApply` set and `FlowInfo::queryCall` reads).
    #[test]
    fn function_inline_noreturn_injectid_roundtrip() {
        let m = build_manager();
        let (mut db, g) = db_with_global(m.num_spaces());
        let addr = Address::new(space(&m, 2), 0x1000);
        let sid = db.add_function(g, &addr, "callee", 1, dt(1)).unwrap();
        // Resolve by name (queryFunction(name)) finds the FunctionSymbol.
        assert_eq!(db.query_function_by_name(g, "callee"), Some(sid));
        // Defaults: not inline, not noreturn.
        assert!(!db.function_is_inline(g, &addr));
        assert!(!db.function_is_no_return(g, &addr));
        // OptionInline / OptionNoReturn set them; queryFunction(addr).isInline()
        // reads them back at flow time.
        db.set_function_inline(sid, true);
        db.set_function_no_return(sid, true);
        assert!(db.function_is_inline(g, &addr));
        assert!(db.function_is_no_return(g, &addr));
        db.set_function_inline(sid, false);
        assert!(!db.function_is_inline(g, &addr));
        // Inject id parks the call-fixup id (IfcFixupApply -> setInjectId).
        db.set_function_inject_id(sid, 7);
        match db.symbol(sid).kind {
            SymbolKind::Function { inject_id, .. } => assert_eq!(inject_id, 7),
            _ => panic!("expected a FunctionSymbol"),
        }
        // An unknown name resolves to None (the "Unknown function name" path).
        assert_eq!(db.query_function_by_name(g, "nope"), None);
    }
}
