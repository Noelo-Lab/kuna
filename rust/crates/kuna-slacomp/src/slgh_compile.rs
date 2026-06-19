//! WS4 -- the `SleighCompile` driver and its subsystems (port of
//! `decompiler/cpp/slgh_compile.cc`, header `slgh_compile.hh`).
//!
//! This is the bulk of the compiler.  It owns the symbol-table build, the
//! parse-time builder methods the parser (WS2) calls, the post-parse `process()`
//! pipeline (consistency-check / pattern build / decision trees / unique
//! allocation), and the orchestration of the final `.sla` `encode` (WS5).
//!
//! It *extends* (via composition) the already-ported `kuna_sleigh::SleighBase`
//! and reuses the `kuna_sleigh` symbol/pattern/template types throughout --
//! symbols are referenced by their integer id in the `SymbolTable`
//! (`symbollist` index), modelled here as [`SymbolId`].
//!
//! ## Lifecycle (slgh_compile.cc:3774, 2479)
//!
//! ```text
//! run_compilation(filein, fileout):
//!     parseFromNewFile(filein)            // set up file/line stacks
//!     SleighParser::parse(self)           // WS2 drives builder methods below
//!     process():                          // post-parse pipeline
//!         checkNops; checkCaseSensitivity
//!         checkConsistency  (ConsistencyChecker passes)
//!         checkLocalCollisions
//!         buildPatterns
//!         buildDecisionTrees
//!         buildXrefs
//!         checkUniqueAllocation
//!         symtab.purge
//!     encode(FormatEncode)                // WS5: write the .sla
//! ```
//!
//! ## Module ownership: WS4 owns this file exclusively.
//!
//! `SectionVector`/`SpaceQuality`/`FieldQuality`/`WithBlock`/`ConsistencyChecker`/
//! `FieldContext` are compiler-only helpers (slgh_compile.hh:42-246) and live
//! here.  The `.sla` `encode` *orchestration* is split out to
//! [`crate::encode`] (WS5) so the writer side is a separate fleet task; the
//! per-symbol `encode(...)` methods it calls already exist in `kuna-sleigh`.

#![allow(dead_code)]

use std::collections::BTreeMap;

use kuna_base::error::KunaResult;

use crate::pcodecompile_actions::SleighPcode;

/// A symbol's id in the `kuna_sleigh` `SymbolTable` (`symbollist` index).
/// Replaces the C++ `SleighSymbol *` / `*Symbol` return/param types.
pub type SymbolId = u32;

// ---------------------------------------------------------------------------
// Compiler-only helper structs (slgh_compile.hh:42-246)
// ---------------------------------------------------------------------------

/// A named p-code section paired with its symbol scope (`RtlPair`,
/// slgh_compile.hh:42-47).
#[derive(Clone, Copy, Default)]
pub struct RtlPair {
    /// Index of the `ConstructTpl` section in the driver's section arena (or `None`).
    pub section: Option<u32>,
    /// Symbol scope id associated with the section (or `None`).
    pub scope: Option<u32>,
}

/// The collection of named p-code sections for one Constructor (`SectionVector`,
/// slgh_compile.hh:58-72).
#[derive(Default)]
pub struct SectionVector {
    /// Index of the section currently being parsed (`nextindex`).
    pub nextindex: i32,
    /// The main section (`main`).
    pub main: RtlPair,
    /// Named sections, by index (`named`).
    pub named: Vec<RtlPair>,
}

impl SectionVector {
    /// `SectionVector::SectionVector` (slgh_compile.cc:34).
    pub fn new(_rtl: u32, _scope: u32) -> SectionVector {
        todo!("WS4: SectionVector::SectionVector (slgh_compile.cc:34-40)")
    }
    /// `SectionVector::releaseMainSection` (slgh_compile.cc:55).
    pub fn release_main_section(&mut self) -> Option<u32> {
        todo!("WS4: SectionVector::releaseMainSection (slgh_compile.cc:55-63)")
    }
    /// `SectionVector::releaseNamedSection` (slgh_compile.cc:65).
    pub fn release_named_section(&mut self, _index: i32) -> Option<u32> {
        todo!("WS4: SectionVector::releaseNamedSection (slgh_compile.cc:65-74)")
    }
    /// `SectionVector::append` (slgh_compile.cc:76).
    pub fn append(&mut self, _rtl: u32, _scope: u32) {
        todo!("WS4: SectionVector::append (slgh_compile.cc:76-85)")
    }
    /// `SectionVector::getMaxId` (slgh_compile.hh:70).
    pub fn get_max_id(&self) -> i32 {
        self.named.len() as i32
    }
    /// `SectionVector::setNextIndex` (slgh_compile.hh:69).
    pub fn set_next_index(&mut self, i: i32) {
        self.nextindex = i;
    }
}

/// Address-space type, as parsed (`SpaceQuality::{ramtype,registertype}`,
/// slgh_compile.hh:80-83).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpaceType {
    /// Normal indexed memory (`ramtype`).
    Ram,
    /// Register space (`registertype`).
    Register,
}

/// Parsed qualities of an address space prior to allocation (`SpaceQuality`,
/// slgh_compile.hh:78-90).
pub struct SpaceQuality {
    pub name: Vec<u8>,
    pub kind: SpaceType,
    pub size: u32,
    pub wordsize: u32,
    pub isdefault: bool,
}

impl SpaceQuality {
    /// `SpaceQuality::SpaceQuality` (slgh_compile.cc:87) -- defaults: ramtype,
    /// size 0, wordsize 1, not default.
    pub fn new(_nm: &[u8]) -> SpaceQuality {
        todo!("WS4: SpaceQuality::SpaceQuality (slgh_compile.cc:87-100)")
    }
}

/// Parsed qualities of a token/context field prior to allocation (`FieldQuality`,
/// slgh_compile.hh:97-105).
pub struct FieldQuality {
    pub name: Vec<u8>,
    pub low: u32,
    pub high: u32,
    pub signext: bool,
    pub flow: bool,
    pub hex: bool,
}

impl FieldQuality {
    /// `FieldQuality::FieldQuality` (slgh_compile.cc:102).
    pub fn new(_nm: &[u8], _l: u64, _h: u64) -> FieldQuality {
        todo!("WS4: FieldQuality::FieldQuality (slgh_compile.cc:102-118)")
    }
}

/// Header info applied across a `with` block (`WithBlock`, slgh_compile.hh:112-123).
#[derive(Default)]
pub struct WithBlock {
    /// Subtable each Constructor attaches to (`ss`), or `None` for the root table.
    pub ss: Option<SymbolId>,
    /// Pattern to prepend to each Constructor (`pateq`), or `None`.  Index into
    /// the driver's pattern-equation arena.
    pub pateq: Option<u32>,
    /// Context changes to associate with each Constructor (`contvec`).
    pub contvec: Vec<u32>,
}

impl WithBlock {
    /// `WithBlock::set` (slgh_compile.cc:120).
    pub fn set(&mut self, _s: Option<SymbolId>, _pq: Option<u32>, _cvec: Option<Vec<u32>>) {
        todo!("WS4: WithBlock::set (slgh_compile.cc:120-132)")
    }
    /// `WithBlock::collectAndPrependPattern` (slgh_compile.cc:152).
    pub fn collect_and_prepend_pattern(_stack: &[WithBlock], _pateq: Option<u32>) -> Option<u32> {
        todo!("WS4: WithBlock::collectAndPrependPattern (slgh_compile.cc:152-170)")
    }
    /// `WithBlock::collectAndPrependContext` (slgh_compile.cc:172).
    pub fn collect_and_prepend_context(_stack: &[WithBlock], _contvec: Option<Vec<u32>>) -> Option<Vec<u32>> {
        todo!("WS4: WithBlock::collectAndPrependContext (slgh_compile.cc:172-202)")
    }
    /// `WithBlock::getCurrentSubtable` (slgh_compile.cc:203).
    pub fn get_current_subtable(_stack: &[WithBlock]) -> Option<SymbolId> {
        todo!("WS4: WithBlock::getCurrentSubtable (slgh_compile.cc:203-213)")
    }
}

/// A context field's storage + qualities prior to layout (`FieldContext`,
/// slgh_compile.hh:241-246).  Sorts by least-significant-bit boundary.
pub struct FieldContext {
    /// Varnode symbol id backing the field's physical storage (`sym`).
    pub sym: SymbolId,
    /// The parsed field qualities (`qual`).
    pub qual: FieldQuality,
}

// ---------------------------------------------------------------------------
// ConsistencyChecker (slgh_compile.hh:142-235)
// ---------------------------------------------------------------------------

/// How a temporary register is used within a Constructor (`OptimizeRecord`,
/// slgh_compile.hh:149-175).
#[derive(Clone)]
pub struct OptimizeRecord {
    pub offset: u64,
    pub size: i32,
    pub writeop: i32,
    pub readop: i32,
    pub inslot: i32,
    pub writecount: i32,
    pub readcount: i32,
    pub writesection: i32,
    pub readsection: i32,
    pub opttype: i32,
}

impl OptimizeRecord {
    /// `OptimizeRecord::OptimizeRecord(offset,size)` (slgh_compile.hh:165).
    pub fn new(offset: u64, size: i32) -> OptimizeRecord {
        OptimizeRecord {
            offset,
            size,
            writeop: -1,
            readop: -1,
            inslot: -1,
            writecount: 0,
            readcount: 0,
            writesection: -2,
            readsection: -2,
            opttype: -1,
        }
    }
    /// `OptimizeRecord::updateRead` (slgh_compile.cc:243).
    pub fn update_read(&mut self, _i: i32, _inslot: i32, _sec_num: i32) {
        todo!("WS4: OptimizeRecord::updateRead (slgh_compile.cc:243-253)")
    }
    /// `OptimizeRecord::updateWrite` (slgh_compile.cc:254).
    pub fn update_write(&mut self, _i: i32, _sec_num: i32) {
        todo!("WS4: OptimizeRecord::updateWrite (slgh_compile.cc:254-261)")
    }
}

/// Container of overlapping temporary records (`ConsistencyChecker::UniqueState`,
/// slgh_compile.hh:178-188).
#[derive(Default)]
pub struct UniqueState {
    recs: BTreeMap<u64, OptimizeRecord>,
}

/// Derives Varnode sizes, optimizes p-code, and checks validity in Constructor
/// p-code (`ConsistencyChecker`, slgh_compile.hh:142-235).
///
/// Holds a back-reference to the driver (modelled as a method receiver -- the
/// passes take `&mut SleighCompile`) plus the post-order table list and the
/// per-table export-size map.
#[derive(Default)]
pub struct ConsistencyChecker {
    pub unnecessarypcode: i32,
    pub readnowrite: i32,
    pub writenoread: i32,
    pub printextwarning: bool,
    pub printdeadwarning: bool,
    /// Root subtable symbol id.
    pub root_symbol: SymbolId,
    /// Subtables in post order.
    pub postorder: Vec<SymbolId>,
    /// Export sizes by subtable id.
    pub sizemap: BTreeMap<SymbolId, i32>,
}

impl ConsistencyChecker {
    /// `ConsistencyChecker::ConsistencyChecker` (slgh_compile.cc:297).
    pub fn new(_sleigh: &SleighCompile, _rt: SymbolId, _unnecessary: bool, _warndead: bool) -> ConsistencyChecker {
        todo!("WS4: ConsistencyChecker::ConsistencyChecker (slgh_compile.cc:297-319)")
    }
    /// `ConsistencyChecker::testSizeRestrictions` (slgh_compile.cc:1687).
    pub fn test_size_restrictions(&mut self, _sleigh: &mut SleighCompile) -> bool {
        todo!("WS4: ConsistencyChecker::testSizeRestrictions (slgh_compile.cc:1687-1703)")
    }
    /// `ConsistencyChecker::testTruncations` (slgh_compile.cc:1704).
    pub fn test_truncations(&mut self, _sleigh: &mut SleighCompile) -> bool {
        todo!("WS4: ConsistencyChecker::testTruncations (slgh_compile.cc:1704-1734)")
    }
    /// `ConsistencyChecker::testLargeTemporary` (slgh_compile.cc:1735).
    pub fn test_large_temporary(&mut self, _sleigh: &mut SleighCompile) {
        todo!("WS4: ConsistencyChecker::testLargeTemporary (slgh_compile.cc:1735-1759)")
    }
    /// `ConsistencyChecker::optimizeAll` (slgh_compile.cc:1760).
    pub fn optimize_all(&mut self, _sleigh: &mut SleighCompile) {
        todo!("WS4: ConsistencyChecker::optimizeAll (slgh_compile.cc:1760-1776)")
    }
    /// `ConsistencyChecker::setPostOrder` (slgh_compile.cc:1187).
    pub fn set_post_order(&mut self, _sleigh: &SleighCompile, _root: SymbolId) {
        todo!("WS4: ConsistencyChecker::setPostOrder (slgh_compile.cc:1187-1238)")
    }
    /// `getNumUnnecessaryPcode` (slgh_compile.hh:232).
    pub fn get_num_unnecessary_pcode(&self) -> i32 {
        self.unnecessarypcode
    }
    /// `getNumReadNoWrite` (slgh_compile.hh:233).
    pub fn get_num_read_no_write(&self) -> i32 {
        self.readnowrite
    }
    /// `getNumWriteNoRead` (slgh_compile.hh:234).
    pub fn get_num_write_no_read(&self) -> i32 {
        self.writenoread
    }
}

// ---------------------------------------------------------------------------
// SleighCompile -- the driver (slgh_compile.hh:302-484)
// ---------------------------------------------------------------------------

/// SLEIGH specification compiler: parses `.slaspec` and produces `.sla`
/// (`SleighCompile : public SleighBase`, slgh_compile.hh:302).
///
/// Composes the already-ported `kuna_sleigh::SleighBase` (symbol table, spaces,
/// constructor tables) rather than inheriting it; the `SleighBase` value is held
/// in [`SleighCompile::base`].  The compiler builds into that base via the parser
/// builder methods, runs [`SleighCompile::process`], then orchestrates the `.sla`
/// `encode` via [`crate::encode`].
#[derive(Default)]
pub struct SleighCompile {
    /// The p-code parsing sub-engine (`SleighPcode pcode`, slgh_compile.hh:307).
    pub pcode: SleighPcode,

    /// The shared SLEIGH base (symbol table, address spaces, root table).  Held
    /// as an opaque arena slot here so the skeleton is decoupled from the exact
    /// `kuna_sleigh::SleighBase` ownership; WS4 binds this to the real type.
    /// (Interface-freeze note: WS4 replaces `()` with `kuna_sleigh::SleighBase`.)
    pub base: (),

    // --- parse-time state (slgh_compile.hh:309-334) ---
    preproc_defines: BTreeMap<Vec<u8>, Vec<u8>>,
    contexttable: Vec<FieldContext>,
    /// Macro definition section ids (`macrotable`).
    macrotable: Vec<u32>,
    /// Token definition ids (`tokentable`).
    tokentable: Vec<u32>,
    /// Subtable symbol ids (`tables`).
    tables: Vec<SymbolId>,
    /// Section symbol ids (`sections`).
    sections: Vec<SymbolId>,
    /// Stack of `with` blocks (`withstack`).
    withstack: Vec<WithBlock>,
    /// Current Constructor id being defined (`curct`).
    curct: Option<u32>,
    /// Current macro symbol id being defined (`curmacro`).
    curmacro: Option<SymbolId>,
    /// Whether the context layout has been locked (`contextlock`).
    contextlock: bool,
    relpath: Vec<Vec<u8>>,
    filename: Vec<Vec<u8>>,
    lineno: Vec<i32>,
    userop_count: i32,
    warnunnecessarypcode: bool,
    warndeadtemps: bool,
    lenientconflicterrors: bool,
    warnalllocalcollisions: bool,
    warnallnops: bool,
    failinsensitivedups: bool,
    debugoutput: bool,
    noplist: Vec<Vec<u8>>,
    /// Number of fatal errors (`errors`).
    errors: i32,
}

impl SleighCompile {
    /// `SleighCompile::SleighCompile` (slgh_compile.cc:1960).
    pub fn new() -> SleighCompile {
        todo!("WS4: SleighCompile::SleighCompile (slgh_compile.cc:1960-1981)")
    }

    // --- top-level entry (slgh_compile.cc:3774) ---

    /// Compile `filein` (a `.slaspec`) to `fileout` (a `.sla`).  The single
    /// entry point: parse -> process -> encode (`run_compilation`,
    /// slgh_compile.cc:3774).  Returns the C++ exit code (0 = success, 2 = error).
    pub fn run_compilation(&mut self, _filein: &str, _fileout: &str) -> KunaResult<i32> {
        todo!("WS4: SleighCompile::run_compilation (slgh_compile.cc:3774-3820)")
    }

    /// Post-parse pipeline (`process`, slgh_compile.cc:2479): checkNops ->
    /// checkCaseSensitivity -> checkConsistency -> checkLocalCollisions ->
    /// buildPatterns -> buildDecisionTrees -> buildXrefs -> checkUniqueAllocation
    /// -> symtab.purge.
    pub fn process(&mut self) {
        todo!("WS4: SleighCompile::process (slgh_compile.cc:2479-2508)")
    }

    /// Set all compiler options at once (`setAllOptions`, slgh_compile.cc:3901).
    #[allow(clippy::too_many_arguments)]
    pub fn set_all_options(
        &mut self,
        _defines: &BTreeMap<Vec<u8>, Vec<u8>>,
        _unnecessary_pcode_warning: bool,
        _lenient_conflict: bool,
        _all_collision_warning: bool,
        _all_nop_warning: bool,
        _dead_temp_warning: bool,
        _enforce_local_key_word: bool,
        _case_sensitive_register_names: bool,
        _debug_output: bool,
    ) {
        todo!("WS4: SleighCompile::setAllOptions (slgh_compile.cc:3901-3918)")
    }

    /// `numErrors` (slgh_compile.hh:371).
    pub fn num_errors(&self) -> i32 {
        self.errors
    }

    // --- error/warning reporting (slgh_compile.cc:2403-2464) ---

    /// `reportError(msg)` (slgh_compile.cc:2430).
    pub fn report_error(&mut self, _msg: &str) {
        todo!("WS4: SleighCompile::reportError(msg) (slgh_compile.cc:2430-2444)")
    }
    /// `reportWarning(msg)` (slgh_compile.cc:2454).
    pub fn report_warning(&mut self, _msg: &str) {
        todo!("WS4: SleighCompile::reportWarning(msg) (slgh_compile.cc:2454-2464)")
    }

    // --- lexer-facing hooks (slgh_compile.cc:2515-2638) ---

    /// `calcContextLayout` (slgh_compile.cc:2515): lock + lay out context fields.
    pub fn calc_context_layout(&mut self) {
        todo!("WS4: SleighCompile::calcContextLayout (slgh_compile.cc:2515-2545)")
    }
    /// `parseFromNewFile` (slgh_compile.cc:2558).
    pub fn parse_from_new_file(&mut self, _fname: &[u8]) {
        todo!("WS4: SleighCompile::parseFromNewFile (slgh_compile.cc:2558-2574)")
    }
    /// `parsePreprocMacro` (slgh_compile.cc:2575).
    pub fn parse_preproc_macro(&mut self) {
        todo!("WS4: SleighCompile::parsePreprocMacro (slgh_compile.cc:2575-2584)")
    }
    /// `parseFileFinished` (slgh_compile.cc:2585).
    pub fn parse_file_finished(&mut self) {
        todo!("WS4: SleighCompile::parseFileFinished (slgh_compile.cc:2585-2596)")
    }
    /// `nextLine` (slgh_compile.hh:421).
    pub fn next_line(&mut self) {
        if let Some(l) = self.lineno.last_mut() {
            *l += 1;
        }
    }
    /// `getPreprocValue` (slgh_compile.cc:2597).
    pub fn get_preproc_value(&self, _nm: &[u8]) -> Option<Vec<u8>> {
        todo!("WS4: SleighCompile::getPreprocValue (slgh_compile.cc:2597-2608)")
    }
    /// `setPreprocValue` (slgh_compile.cc:2609).
    pub fn set_preproc_value(&mut self, _nm: &[u8], _value: &[u8]) {
        todo!("WS4: SleighCompile::setPreprocValue (slgh_compile.cc:2609-2617)")
    }
    /// `undefinePreprocValue` (slgh_compile.cc:2618).
    pub fn undefine_preproc_value(&mut self, _nm: &[u8]) -> bool {
        todo!("WS4: SleighCompile::undefinePreprocValue (slgh_compile.cc:2618-2639)")
    }

    // --- parser-facing builder methods (slgh_compile.cc:2640-3759) ---
    // These are the actions the WS2 parser invokes.  Each returns/accepts symbol
    // ids (`SymbolId`) in place of the C++ `*Symbol`.

    /// `defineToken` (slgh_compile.cc:2640).
    pub fn define_token(&mut self, _name: &[u8], _sz: u64, _endian: i32) -> SymbolId {
        todo!("WS4: SleighCompile::defineToken (slgh_compile.cc:2640-2667)")
    }
    /// `addTokenField` (slgh_compile.cc:2668).
    pub fn add_token_field(&mut self, _sym: SymbolId, _qual: FieldQuality) {
        todo!("WS4: SleighCompile::addTokenField (slgh_compile.cc:2668-2689)")
    }
    /// `addContextField` (slgh_compile.cc:2690).
    pub fn add_context_field(&mut self, _sym: SymbolId, _qual: FieldQuality) -> bool {
        todo!("WS4: SleighCompile::addContextField (slgh_compile.cc:2690-2712)")
    }
    /// `newSpace` (slgh_compile.cc:2713).
    pub fn new_space(&mut self, _qual: SpaceQuality) {
        todo!("WS4: SleighCompile::newSpace (slgh_compile.cc:2713-2741)")
    }
    /// `newSectionSymbol` (slgh_compile.cc:2742).
    pub fn new_section_symbol(&mut self, _nm: &[u8]) -> SymbolId {
        todo!("WS4: SleighCompile::newSectionSymbol (slgh_compile.cc:2742-2760)")
    }
    /// `setEndian` (slgh_compile.cc:2761).
    pub fn set_endian(&mut self, _end: i32) {
        todo!("WS4: SleighCompile::setEndian (slgh_compile.cc:2761-2775)")
    }
    /// `setAlignment` (slgh_compile.hh:437).
    pub fn set_alignment(&mut self, _val: i32) {
        todo!("WS4: SleighCompile::setAlignment (slgh_compile.hh:437)")
    }
    /// `defineVarnodes` (slgh_compile.cc:2776).
    pub fn define_varnodes(&mut self, _spacesym: SymbolId, _off: u64, _size: u64, _names: Vec<Vec<u8>>) {
        todo!("WS4: SleighCompile::defineVarnodes (slgh_compile.cc:2776-2799)")
    }
    /// `defineBitrange` (slgh_compile.cc:2800).
    pub fn define_bitrange(&mut self, _name: &[u8], _sym: SymbolId, _bitoffset: u32, _numb: u32) {
        todo!("WS4: SleighCompile::defineBitrange (slgh_compile.cc:2800-2832)")
    }
    /// `addUserOp` (slgh_compile.cc:2833).
    pub fn add_user_op(&mut self, _names: Vec<Vec<u8>>) {
        todo!("WS4: SleighCompile::addUserOp (slgh_compile.cc:2833-2848)")
    }
    /// `attachValues` (slgh_compile.cc:2872).
    pub fn attach_values(&mut self, _symlist: Vec<SymbolId>, _numlist: Vec<i64>) {
        todo!("WS4: SleighCompile::attachValues (slgh_compile.cc:2872-2899)")
    }
    /// `attachNames` (slgh_compile.cc:2900).
    pub fn attach_names(&mut self, _symlist: Vec<SymbolId>, _names: Vec<Vec<u8>>) {
        todo!("WS4: SleighCompile::attachNames (slgh_compile.cc:2900-2927)")
    }
    /// `attachVarnodes` (slgh_compile.cc:2928).
    pub fn attach_varnodes(&mut self, _symlist: Vec<SymbolId>, _varlist: Vec<SymbolId>) {
        todo!("WS4: SleighCompile::attachVarnodes (slgh_compile.cc:2928-2967)")
    }
    /// `newTable` (slgh_compile.cc:2968).
    pub fn new_table(&mut self, _nm: &[u8]) -> SymbolId {
        todo!("WS4: SleighCompile::newTable (slgh_compile.cc:2968-2983)")
    }
    /// `newOperand` (slgh_compile.cc:2984).
    pub fn new_operand(&mut self, _ct: u32, _nm: &[u8]) {
        todo!("WS4: SleighCompile::newOperand (slgh_compile.cc:2984-2999)")
    }
    /// `constrainOperand` (slgh_compile.cc:3000) -> a pattern-equation id.
    pub fn constrain_operand(&mut self, _sym: SymbolId, _patexp: u32) -> Option<u32> {
        todo!("WS4: SleighCompile::constrainOperand (slgh_compile.cc:3000-3021)")
    }
    /// `defineOperand` (slgh_compile.cc:3022).
    pub fn define_operand(&mut self, _sym: SymbolId, _patexp: u32) {
        todo!("WS4: SleighCompile::defineOperand (slgh_compile.cc:3022-3043)")
    }
    /// `defineInvisibleOperand` (slgh_compile.cc:3044) -> a pattern-equation id.
    pub fn define_invisible_operand(&mut self, _sym: SymbolId) -> Option<u32> {
        todo!("WS4: SleighCompile::defineInvisibleOperand (slgh_compile.cc:3044-3071)")
    }
    /// `selfDefine` (slgh_compile.cc:3072).
    pub fn self_define(&mut self, _sym: SymbolId) {
        todo!("WS4: SleighCompile::selfDefine (slgh_compile.cc:3072-3098)")
    }
    /// `setResultVarnode` (slgh_compile.cc:3099) -> a ConstructTpl id.
    pub fn set_result_varnode(&mut self, _ct: u32, _vn: u32) -> u32 {
        todo!("WS4: SleighCompile::setResultVarnode (slgh_compile.cc:3099-3114)")
    }
    /// `setResultStarVarnode` (slgh_compile.cc:3115) -> a ConstructTpl id.
    pub fn set_result_star_varnode(&mut self, _ct: u32, _star: u32, _vn: u32) -> u32 {
        todo!("WS4: SleighCompile::setResultStarVarnode (slgh_compile.cc:3115-3136)")
    }
    /// `contextMod` (slgh_compile.cc:3137).
    pub fn context_mod(&mut self, _vec: &mut Vec<u32>, _sym: SymbolId, _pe: u32) -> bool {
        todo!("WS4: SleighCompile::contextMod (slgh_compile.cc:3137-3163)")
    }
    /// `contextSet` (slgh_compile.cc:3164).
    pub fn context_set(&mut self, _vec: &mut Vec<u32>, _sym: SymbolId, _cvar: SymbolId) {
        todo!("WS4: SleighCompile::contextSet (slgh_compile.cc:3164-3179)")
    }
    /// `createMacro` (slgh_compile.cc:3180).
    pub fn create_macro(&mut self, _name: &[u8], _params: Vec<Vec<u8>>) -> SymbolId {
        todo!("WS4: SleighCompile::createMacro (slgh_compile.cc:3180-3203)")
    }
    /// `createMacroUse` (slgh_compile.cc:3235) -> an op-template list id.
    pub fn create_macro_use(&mut self, _sym: SymbolId, _param: Vec<u32>) -> u32 {
        todo!("WS4: SleighCompile::createMacroUse (slgh_compile.cc:3235-3256)")
    }
    /// `standaloneSection` (slgh_compile.cc:3257).
    pub fn standalone_section(&mut self, _main: u32) -> SectionVector {
        todo!("WS4: SleighCompile::standaloneSection (slgh_compile.cc:3257-3271)")
    }
    /// `firstNamedSection` (slgh_compile.cc:3272).
    pub fn first_named_section(&mut self, _main: u32, _sym: SymbolId) -> SectionVector {
        todo!("WS4: SleighCompile::firstNamedSection (slgh_compile.cc:3272-3294)")
    }
    /// `nextNamedSection` (slgh_compile.cc:3295).
    pub fn next_named_section(&mut self, _vec: SectionVector, _section: u32, _sym: SymbolId) -> SectionVector {
        todo!("WS4: SleighCompile::nextNamedSection (slgh_compile.cc:3295-3316)")
    }
    /// `finalNamedSection` (slgh_compile.cc:3317).
    pub fn final_named_section(&mut self, _vec: SectionVector, _section: u32) -> SectionVector {
        todo!("WS4: SleighCompile::finalNamedSection (slgh_compile.cc:3317-3329)")
    }
    /// `createCrossBuild` (slgh_compile.cc:3330) -> an op-template list id.
    pub fn create_cross_build(&mut self, _addr: u32, _sym: SymbolId) -> u32 {
        todo!("WS4: SleighCompile::createCrossBuild (slgh_compile.cc:3330-3350)")
    }
    /// `enterSection` (slgh_compile.cc:3351) -> a ConstructTpl id.
    pub fn enter_section(&mut self) -> u32 {
        todo!("WS4: SleighCompile::enterSection (slgh_compile.cc:3351-3363)")
    }
    /// `createConstructor` (slgh_compile.cc:3364) -> a Constructor id.
    pub fn create_constructor(&mut self, _sym: Option<SymbolId>) -> u32 {
        todo!("WS4: SleighCompile::createConstructor (slgh_compile.cc:3364-3383)")
    }
    /// `resetConstructors` (slgh_compile.cc:3384).
    pub fn reset_constructors(&mut self) {
        todo!("WS4: SleighCompile::resetConstructors (slgh_compile.cc:3384-3396)")
    }
    /// `pushWith` (slgh_compile.cc:3676).
    pub fn push_with(&mut self, _ss: Option<SymbolId>, _pateq: Option<u32>, _contvec: Option<Vec<u32>>) {
        todo!("WS4: SleighCompile::pushWith (slgh_compile.cc:3676-3683)")
    }
    /// `popWith` (slgh_compile.cc:3684).
    pub fn pop_with(&mut self) {
        todo!("WS4: SleighCompile::popWith (slgh_compile.cc:3684-3697)")
    }
    /// `buildConstructor` (slgh_compile.cc:3698).
    pub fn build_constructor(&mut self, _big: u32, _pateq: Option<u32>, _contvec: Option<Vec<u32>>, _vec: SectionVector) {
        todo!("WS4: SleighCompile::buildConstructor (slgh_compile.cc:3698-3736)")
    }
    /// `buildMacro` (slgh_compile.cc:3737).
    pub fn build_macro(&mut self, _sym: SymbolId, _rtl: u32) {
        todo!("WS4: SleighCompile::buildMacro (slgh_compile.cc:3737-3757)")
    }
    /// `recordNop` (slgh_compile.cc:3758).
    pub fn record_nop(&mut self) {
        todo!("WS4: SleighCompile::recordNop (slgh_compile.cc:3758-3773)")
    }
    /// `getUniqueAddr` (slgh_compile.cc:2465).
    pub fn get_unique_addr(&mut self) -> u32 {
        todo!("WS4: SleighCompile::getUniqueAddr (slgh_compile.cc:2465-2478)")
    }

    // --- post-parse subsystems (slgh_compile.cc:2025-2333, 3638) ---

    /// `buildPatterns` (slgh_compile.cc:2122).
    fn build_patterns(&mut self) {
        todo!("WS4: SleighCompile::buildPatterns (slgh_compile.cc:2122-2147)")
    }
    /// `buildDecisionTrees` (slgh_compile.cc:2086).
    fn build_decision_trees(&mut self) {
        todo!("WS4: SleighCompile::buildDecisionTrees (slgh_compile.cc:2086-2121)")
    }
    /// `checkConsistency` (slgh_compile.cc:2148).
    fn check_consistency(&mut self) {
        todo!("WS4: SleighCompile::checkConsistency (slgh_compile.cc:2148-2192)")
    }
    /// `checkLocalCollisions` (slgh_compile.cc:2250).
    fn check_local_collisions(&mut self) {
        todo!("WS4: SleighCompile::checkLocalCollisions (slgh_compile.cc:2250-2276)")
    }
    /// `checkNops` (slgh_compile.cc:2277).
    fn check_nops(&mut self) {
        todo!("WS4: SleighCompile::checkNops (slgh_compile.cc:2277-2296)")
    }
    /// `checkCaseSensitivity` (slgh_compile.cc:2297).
    fn check_case_sensitivity(&mut self) {
        todo!("WS4: SleighCompile::checkCaseSensitivity (slgh_compile.cc:2297-2333)")
    }
    /// `checkUniqueAllocation` (slgh_compile.cc:3638).
    fn check_unique_allocation(&mut self) {
        todo!("WS4: SleighCompile::checkUniqueAllocation (slgh_compile.cc:3638-3675)")
    }
    /// `calcContextVarLayout` (slgh_compile.cc:2025).
    fn calc_context_var_layout(&mut self, _start: i32, _sz: i32, _numbits: i32) -> i32 {
        todo!("WS4: SleighCompile::calcContextVarLayout (slgh_compile.cc:2025-2085)")
    }
}
