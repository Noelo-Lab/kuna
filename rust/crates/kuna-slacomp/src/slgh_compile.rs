//! WS4b -- the `SleighCompile` driver and its subsystems (port of
//! `decompiler/cpp/slgh_compile.cc`, header `slgh_compile.hh`).
//!
//! This is the bulk of the compiler.  It owns the symbol-table build, the
//! parse-time builder methods the parser (WS2) calls, the post-parse `process()`
//! pipeline (consistency-check / pattern build / decision trees / unique
//! allocation), and the orchestration of the final `.sla` `encode` (WS5).
//!
//! It *composes* the already-ported `kuna_sleigh::SleighBase` and reuses the
//! `kuna_sleigh` symbol/pattern/template types throughout -- symbols are
//! referenced by their integer id in the `SymbolTable`, modelled as
//! [`SymbolId`]; pattern equations live in a driver-owned
//! [`kuna_sleigh::slghpatexpress::EquationArena`]; ConstructTpl sections live in
//! the `SleighBase` template arena.
//!
//! ## Lifecycle (slgh_compile.cc:3774, 2479)
//!
//! `run_compilation`: parse -> `process()` (consistency / patterns / decision
//! trees / unique allocation / purge) -> encode (WS5).
//!
//! ## Scope note (WS4b landed subset)
//!
//! The full definition half (spaces / tokens / contexts / varnodes / attaches /
//! subtables / constructors / pattern equations) plus the `process()` pattern/
//! decision-tree pipeline and the `.sla` encode are implemented and exercised
//! end-to-end (data-le-64 / data-be-64 byte-identical against C++ `sleigh_opt`).
//! The deep p-code *section* path (semantic RTL with `Constructor::
//! setMainSection` / `markSubtableOperands` / `ConstructTpl::fillinBuild`, which
//! were never ported to `kuna-sleigh`) is stubbed with errors/panics that carry
//! their `slgh_compile.cc`/`slghsymbol.cc` anchors; specs that exercise it are
//! not yet claimed.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;

use kuna_num::opcodes::OpCode;

use kuna_sleigh::pcodecompile::{
    ExprTree, Location, PcodeCompile, PcodeCompileSymbol, StarQuality,
};
use kuna_sleigh::semantics::{
    ConstTpl, ConstType, ConstructTpl, HandleTpl, OpTpl, VarnodeTpl, BUILD, CROSSBUILD, DELAY_SLOT,
    LABELBUILD, MACROBUILD,
};
use kuna_sleigh::slghpatexpress::{
    ConstantValue, ContextField, EquationArena, PatternEquation, PatternExpression, PatternValue,
};
use kuna_sleigh::slghsymbol::{
    ConstructTplHandle, ContextChange, ContextCommit, ContextOp, DecisionProperties,
    LabelTableSymbol, MacroSymbol, SectionSymbol, SleighSymbol, SymbolKind, SymbolType,
};
use kuna_sleigh::sleighbase::SleighBase;

use crate::pcodecompile_actions::{CompilerHost, SleighPcode};
use crate::slghparse::{ConstOp, FieldQual, ParserActions, PcodeOpc, SpaceQual};
use crate::slghscan::{ScannerHost, SymbolTokenKind};

/// A symbol's id in the `kuna_sleigh` `SymbolTable` (`symbollist` index).
/// Replaces the C++ `SleighSymbol *` / `*Symbol` return/param types.
pub type SymbolId = u32;

/// Sentinel returned by `resolve_symbol` for a name that does not resolve.
const NO_SYMBOL: SymbolId = u32::MAX;

// ---------------------------------------------------------------------------
// Compiler-only helper structs (slgh_compile.hh:42-246)
// ---------------------------------------------------------------------------

/// The heterogeneous bison semantic values of the p-code grammar
/// (`SLEIGHSTYPE`), unified into one tagged enum so the WS2 parser's `u32` ids
/// index a single arena and never alias across value kinds.
#[derive(Debug)]
enum RtlValue {
    /// `VarnodeTpl *` (varnode/jumpdest/intvn/lhsvarnode/exportvarnode).
    Varnode(VarnodeTpl),
    /// `ExprTree *` (`expr`).
    Expr(ExprTree),
    /// `vector<OpTpl *> *` (`statement`).
    OpList(Vec<OpTpl>),
    /// `StarQuality *` (`sizedstar`).
    Star(StarQuality),
    /// `ConstructTpl *` (an in-progress p-code section).
    Section(ConstructTpl),
    /// `SectionVector *` (`standaloneSection`/named sections).
    SecVec(SectionVector),
}

/// A named p-code section paired with its symbol scope (`RtlPair`,
/// slgh_compile.hh:42-47).
#[derive(Clone, Copy, Default, Debug)]
pub struct RtlPair {
    /// `ConstructTpl` handle in the base template arena (or `None`).
    pub section: Option<u32>,
    /// Symbol scope id associated with the section (or `None`).
    pub scope: Option<u32>,
}

/// The collection of named p-code sections for one Constructor (`SectionVector`,
/// slgh_compile.hh:58-72).
#[derive(Default, Debug)]
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
    pub fn new(rtl: u32, scope: Option<u32>) -> SectionVector {
        SectionVector {
            nextindex: -1,
            main: RtlPair {
                section: Some(rtl),
                scope,
            },
            named: Vec::new(),
        }
    }
    /// `SectionVector::getMaxId` (slgh_compile.hh:70).
    pub fn get_max_id(&self) -> i32 {
        self.named.len() as i32
    }
    /// `SectionVector::setNextIndex` (slgh_compile.hh:69).
    pub fn set_next_index(&mut self, i: i32) {
        self.nextindex = i;
    }
    /// `SectionVector::getMainPair` (slgh_compile.hh).
    pub fn get_main_pair(&self) -> RtlPair {
        self.main
    }
    /// `SectionVector::getNamedPair(int4 i)` (slgh_compile.hh).
    pub fn get_named_pair(&self, i: i32) -> RtlPair {
        self.named[i as usize]
    }
    /// `SectionVector::releaseMainSection` (slgh_compile.cc:55): take the main
    /// section id, leaving `None`.
    pub fn release_main_section(&mut self) -> Option<u32> {
        self.main.section.take()
    }
    /// `SectionVector::releaseNamedSection(int4 index)` (slgh_compile.cc:65).
    pub fn release_named_section(&mut self, index: i32) -> Option<u32> {
        self.named[index as usize].section.take()
    }
    /// `SectionVector::append(ConstructTpl *rtl,SymbolScope *scope)`
    /// (slgh_compile.cc:76): grow `named` to fit `nextindex`, store the pair.
    pub fn append(&mut self, rtl: u32, scope: Option<u32>) {
        // C++ `while(named.size() <= nextindex) named.emplace_back();`
        while (self.named.len() as i64) <= i64::from(self.nextindex) {
            self.named.push(RtlPair::default());
        }
        self.named[self.nextindex as usize] = RtlPair {
            section: Some(rtl),
            scope,
        };
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
    /// `SpaceQuality::SpaceQuality` (slgh_compile.cc:87).
    pub fn new(nm: &[u8]) -> SpaceQuality {
        SpaceQuality {
            name: nm.to_vec(),
            kind: SpaceType::Ram,
            size: 0,
            wordsize: 1,
            isdefault: false,
        }
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
    pub fn new(nm: &[u8], l: u64, h: u64) -> FieldQuality {
        FieldQuality {
            name: nm.to_vec(),
            low: l as u32,
            high: h as u32,
            signext: false,
            flow: true,
            hex: true,
        }
    }

    /// Build from the parser's [`FieldQual`].
    fn from_qual(q: FieldQual) -> FieldQuality {
        FieldQuality {
            name: q.name,
            low: q.low as u32,
            high: q.high as u32,
            signext: q.signext,
            flow: q.flow,
            hex: q.hex,
        }
    }
}

/// Header info applied across a `with` block (`WithBlock`, slgh_compile.hh:112-123).
#[derive(Default)]
pub struct WithBlock {
    /// Subtable each Constructor attaches to (`ss`), or `None` for the root table.
    pub ss: Option<SymbolId>,
    /// Pattern to prepend to each Constructor (`pateq`), an `EqId`, or `None`.
    pub pateq: Option<u32>,
    /// Context changes to associate with each Constructor (`contvec`).
    pub contvec: Vec<ContextChange>,
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
// The ConsistencyChecker (slgh_compile.cc:215-1776) lives in `consistency.rs`
// as inherent methods on SleighCompile (it mutates the template arena and reads
// the symbol table, exactly the C++ class's `compiler` back-pointer access).
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// SleighCompile -- the driver (slgh_compile.hh:302-484)
// ---------------------------------------------------------------------------

/// SLEIGH specification compiler: parses `.slaspec` and produces `.sla`
/// (`SleighCompile : public SleighBase`, slgh_compile.hh:302).
#[derive(Default)]
pub struct SleighCompile {
    /// The p-code parsing sub-engine (`SleighPcode pcode`, slgh_compile.hh:307).
    pub pcode: SleighPcode,

    /// The shared SLEIGH base (symbol table, address spaces, root, templates).
    pub base: SleighBase,

    /// The driver-owned pattern-equation arena (the WS2 `peq_*` ids index it).
    arena: EquationArena,
    /// The driver-owned pattern-expression arena (the WS2 `pexp_*` ids index it).
    patexp: Vec<PatternExpression>,

    // --- p-code section RTL arenas (WS4c) ---
    //
    // The WS2 parser threads `u32` ids for the heterogeneous bison semantic
    // values of the p-code grammar (`SLEIGHSTYPE`): `VarnodeTpl *`,
    // `ExprTree *`, `vector<OpTpl *> *`, `StarQuality *`, `ConstructTpl *`
    // (sections), and `SectionVector *`.  Each gets its own driver-owned arena
    // of `Option<T>` slots so an id can be *consumed* (the C++ pointer-move
    // semantics) by taking the slot.
    /// One arena for every heterogeneous p-code grammar semantic value, so ids
    /// are globally unique across kinds (see the accessor helpers).
    rtl_arena: Vec<Option<RtlValue>>,
    /// `ContextChange *` arena (`context_mod`/`context_set` vec elements).
    contextchange_arena: Vec<Option<ContextChange>>,
    /// ConsistencyChecker unnecessary-ext/trunc-to-COPY conversion count
    /// (the C++ `ConsistencyChecker::unnecessarypcode`; bumped by
    /// `deal_with_unnecessary_*`, read after `test_size_restrictions`).
    cc_unnecessary: i32,
    /// The macro bodies (`vector<ConstructTpl *> macrotable`); index = macro id.
    macro_bodies: Vec<Option<ConstructTpl>>,
    /// `maxdelayslotbytes` (slgh_compile.hh): largest delay slot seen.
    maxdelayslotbytes: u32,
    /// `unique_allocatemask` (slgh_compile.hh): set when a crossbuild needs the
    /// unique-space crossbuild region carved out.
    unique_allocatemask: u32,
    /// p-code compile defaultspace cache (C++ `SleighPcode` inherits the
    /// `defaultspace`/`constantspace`/`uniqspace` members of `PcodeCompile`).
    default_space: Option<Rc<AddrSpace>>,

    // --- parse-time state (slgh_compile.hh:309-334) ---
    preproc_defines: BTreeMap<Vec<u8>, Vec<u8>>,
    contexttable: Vec<FieldContext>,
    macrotable: Vec<u32>,
    /// Number of tokens defined so far (`tokentable.size()`).
    token_count: u32,
    /// Subtable symbol ids (`tables`).
    tables: Vec<SymbolId>,
    /// Section symbol ids (`sections`).
    sections: Vec<SymbolId>,
    /// Stack of `with` blocks (`withstack`).
    withstack: Vec<WithBlock>,
    /// (subtable_id, ct_index) for each driver-side constructor id.
    ctmap: Vec<(SymbolId, u32)>,
    /// Current Constructor id being defined (`curct`).
    curct: Option<u32>,
    /// Current macro symbol id being defined (`curmacro`).
    curmacro: Option<SymbolId>,
    /// Whether the context layout has been locked (`contextlock`).
    contextlock: bool,
    relpath: Vec<Vec<u8>>,
    filename: Vec<Vec<u8>>,
    lineno: Vec<i32>,
    symbol_loc: BTreeMap<SymbolId, Location>,
    ctor_loc: BTreeMap<u32, Location>,
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
    constant_space: Option<Rc<AddrSpace>>,
    unique_space: Option<Rc<AddrSpace>>,
}

impl SleighCompile {
    /// `SleighCompile::SleighCompile` (slgh_compile.cc:1960).
    pub fn new() -> SleighCompile {
        SleighCompile {
            lenientconflicterrors: true,
            failinsensitivedups: true,
            ..Default::default()
        }
    }

    // --- top-level entry (slgh_compile.cc:3774) ---

    /// Compile `filein` (a `.slaspec`) to `fileout` (a `.sla`)
    /// (`run_compilation`, slgh_compile.cc:3774).  Returns 0 = success, 2 = error.
    pub fn run_compilation(&mut self, filein: &str, fileout: &str) -> KunaResult<i32> {
        self.parse_from_new_file(filein.as_bytes());
        let src = match std::fs::read(filein) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Unable to open specfile: {filein}");
                return Ok(2);
            }
        };
        let mut scanner = crate::slghscan::SleighScanner::new();
        scanner.open(src);
        let mut parser = crate::slghparse::SleighParser::new(scanner);
        let parseres = match parser.parse(self) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Unrecoverable error: {}", e.explain());
                return Ok(2);
            }
        };
        if parseres == 0 {
            self.process()?;
        }
        if parseres == 0 && self.num_errors() == 0 {
            let bytes = crate::encode::encode_to_sla_bytes(&self.base)?;
            if std::fs::write(fileout, &bytes).is_err() {
                eprintln!("Unable to open output file: {fileout}");
                return Ok(2);
            }
            Ok(0)
        } else {
            eprintln!("No output produced");
            Ok(2)
        }
    }

    /// Post-parse pipeline (`process`, slgh_compile.cc:2479).
    pub fn process(&mut self) -> KunaResult<()> {
        self.check_nops();
        self.check_case_sensitivity();
        if self.base.default_code_space().is_none() {
            self.report_error("No default space specified");
        }
        if self.errors > 0 {
            return Ok(());
        }
        self.check_consistency();
        if self.errors > 0 {
            return Ok(());
        }
        self.check_local_collisions();
        if self.errors > 0 {
            return Ok(());
        }
        self.build_patterns()?;
        if self.errors > 0 {
            return Ok(());
        }
        self.build_decision_trees()?;
        if self.errors > 0 {
            return Ok(());
        }
        self.build_xrefs()?;
        if self.errors > 0 {
            return Ok(());
        }
        self.check_unique_allocation();
        self.base.symtab_mut().purge();
        Ok(())
    }

    /// Set all compiler options at once (`setAllOptions`, slgh_compile.cc:3901).
    #[allow(clippy::too_many_arguments)]
    pub fn set_all_options(
        &mut self,
        defines: &BTreeMap<Vec<u8>, Vec<u8>>,
        unnecessary_pcode_warning: bool,
        lenient_conflict: bool,
        all_collision_warning: bool,
        all_nop_warning: bool,
        dead_temp_warning: bool,
        enforce_local_key_word: bool,
        case_sensitive_register_names: bool,
        debug_output: bool,
    ) {
        for (k, v) in defines {
            self.set_preproc_value(k, v);
        }
        self.warnunnecessarypcode = unnecessary_pcode_warning;
        self.lenientconflicterrors = lenient_conflict;
        self.warnalllocalcollisions = all_collision_warning;
        self.warnallnops = all_nop_warning;
        self.warndeadtemps = dead_temp_warning;
        self.pcode.enforce_local_key = enforce_local_key_word;
        self.failinsensitivedups = !case_sensitive_register_names;
        self.debugoutput = debug_output;
    }

    /// `numErrors` (slgh_compile.hh:371).
    pub fn num_errors(&self) -> i32 {
        self.errors
    }

    // --- error/warning reporting (slgh_compile.cc:2403-2464) ---

    fn current_location(&self) -> Location {
        Location::new(
            self.filename.last().map(|f| f.as_slice()).unwrap_or(b""),
            self.lineno.last().copied().unwrap_or(0),
        )
    }

    fn format_status_message(loc: Option<&Location>, msg: &str) -> String {
        match loc {
            Some(l) => format!("{}: {}", l.format(), msg),
            None => msg.to_string(),
        }
    }

    /// `reportError(msg)` (slgh_compile.cc:2430).
    pub fn report_error(&mut self, msg: &str) {
        eprintln!(
            "{}:{} - ERROR {}",
            String::from_utf8_lossy(self.filename.last().map(|f| f.as_slice()).unwrap_or(b"")),
            self.lineno.last().copied().unwrap_or(0),
            msg
        );
        self.errors += 1;
    }

    fn report_error_loc(&mut self, loc: Option<&Location>, msg: &str) {
        let m = Self::format_status_message(loc, msg);
        self.report_error(&m);
    }

    /// `reportWarning(msg)` (slgh_compile.cc:2454).
    pub fn report_warning(&mut self, msg: &str) {
        eprintln!("WARN  {msg}");
    }

    fn report_warning_loc(&mut self, loc: Option<&Location>, msg: &str) {
        let m = Self::format_status_message(loc, msg);
        self.report_warning(&m);
    }

    // --- ConsistencyChecker glue (the `compiler` back-pointer callbacks) ---

    /// C++ `errors += 1` for a failed ConsistencyChecker pass.
    pub(crate) fn bump_error(&mut self) {
        self.errors += 1;
    }
    /// Plain (no-location) warning (`compiler->reportWarning(msg)`).
    pub(crate) fn report_warning_plain(&mut self, msg: &str) {
        self.report_warning(msg);
    }
    /// `compiler->reportError(compiler->getLocation(ct), msg)` keyed by ctor.
    pub(crate) fn cc_report_error_ct(&mut self, _sym: SymbolId, ctid_unused: u32, msg: &str) {
        let _ = ctid_unused;
        // The ctor location is keyed by the driver constructor id; the checker
        // navigates (table,ctidx) but the location map is keyed by the global
        // ctor id, so fall back to the current parse location (matches C++ when
        // the per-ctor location is unavailable).
        let loc = self.current_location();
        self.report_error_loc(Some(&loc), msg);
    }
    /// `compiler->reportWarning(compiler->getLocation(ct), msg)` keyed by ctor.
    pub(crate) fn cc_report_warning_ct(&mut self, _sym: SymbolId, _ctidx: u32, msg: &str) {
        let loc = self.current_location();
        self.report_warning_loc(Some(&loc), msg);
    }
    pub(crate) fn cc_warn_unnecessary(&self) -> bool {
        self.warnunnecessarypcode
    }
    pub(crate) fn cc_warn_deadtemps(&self) -> bool {
        self.warndeadtemps
    }
    pub(crate) fn cc_bump_unnecessary(&mut self) {
        self.cc_unnecessary += 1;
    }
    pub(crate) fn cc_take_unnecessary(&mut self) -> i32 {
        std::mem::take(&mut self.cc_unnecessary)
    }

    /// `addSymbol` (slgh_compile.cc:2355).  Returns the id (or `NO_SYMBOL` on a
    /// duplicate, which is reported as a parse error).
    fn add_sleigh_symbol(&mut self, sym: SleighSymbol) -> SymbolId {
        let loc = self.current_location();
        match self.base.symtab_mut().add_symbol(sym) {
            Ok(id) => {
                self.symbol_loc.insert(id, loc);
                id
            }
            Err(e) => {
                self.report_error(&e.explain());
                NO_SYMBOL
            }
        }
    }

    // --- lexer-facing hooks (slgh_compile.cc:2515-2638) ---

    /// `predefinedSymbols` (slgh_compile.cc:1982).
    fn predefined_symbols(&mut self) -> KunaResult<()> {
        self.base.symtab_mut().add_scope(); // global scope
        let root = self.add_sleigh_symbol(SleighSymbol::new_subtable(b"instruction"));
        self.base.set_root(root);
        let (constant, _other, unique) = self.base.create_predefined_spaces()?;
        self.constant_space = Some(Rc::clone(&constant));
        self.unique_space = Some(Rc::clone(&unique));
        self.add_sleigh_symbol(SleighSymbol::new_space(Rc::clone(&constant)));
        let other = self.base.space_by_name("OTHER").expect("OTHER inserted");
        self.add_sleigh_symbol(SleighSymbol::new_space(other));
        self.add_sleigh_symbol(SleighSymbol::new_space(Rc::clone(&unique)));
        self.add_sleigh_symbol(SleighSymbol::new_start(b"inst_start", Rc::clone(&constant)));
        self.add_sleigh_symbol(SleighSymbol::new_end(b"inst_next", Rc::clone(&constant)));
        self.add_sleigh_symbol(SleighSymbol::new_next2(b"inst_next2", Rc::clone(&constant)));
        self.add_sleigh_symbol(SleighSymbol::new_epsilon(b"epsilon", Rc::clone(&constant)));
        Ok(())
    }

    /// `calcContextLayout` (slgh_compile.cc:2515).
    pub fn calc_context_layout(&mut self) {
        if self.contextlock {
            return;
        }
        self.contextlock = true;
        let mut context_offset = 0;
        self.contexttable.sort_by(|a, b| {
            (a.sym, a.qual.low, a.qual.high).cmp(&(b.sym, b.qual.low, b.qual.high))
        });
        let mut begin = 0usize;
        while begin < self.contexttable.len() {
            let mut sz = 1usize;
            while begin + sz < self.contexttable.len()
                && self.contexttable[begin + sz].sym == self.contexttable[begin].sym
            {
                sz += 1;
            }
            context_offset = self.calc_context_var_layout(begin as i32, sz as i32, context_offset);
            begin += sz;
        }
        self.contexttable.clear();
    }

    /// `parseFromNewFile` (slgh_compile.cc:2558).
    pub fn parse_from_new_file(&mut self, fname: &[u8]) {
        let (path, base) = split_path(fname);
        self.filename.push(base);
        if self.relpath.is_empty() || is_absolute_path(&path) {
            self.relpath.push(path);
        } else {
            let mut total = self.relpath.last().cloned().unwrap_or_default();
            total.extend_from_slice(&path);
            self.relpath.push(total);
        }
        self.lineno.push(1);
    }

    /// `parsePreprocMacro` (slgh_compile.cc:2575).
    pub fn parse_preproc_macro(&mut self) {
        let mut fname = self.filename.last().cloned().unwrap_or_default();
        fname.extend_from_slice(b":macro");
        self.filename.push(fname);
        self.relpath
            .push(self.relpath.last().cloned().unwrap_or_default());
        self.lineno.push(self.lineno.last().copied().unwrap_or(1));
    }

    /// `parseFileFinished` (slgh_compile.cc:2585).
    pub fn parse_file_finished(&mut self) {
        self.filename.pop();
        self.relpath.pop();
        self.lineno.pop();
    }

    /// `nextLine` (slgh_compile.hh:421).
    pub fn next_line(&mut self) {
        if let Some(l) = self.lineno.last_mut() {
            *l += 1;
        }
    }

    /// `grabCurrentFilePath` (slgh_compile.cc:2546).
    fn grab_current_file_path(&self) -> Vec<u8> {
        if self.relpath.is_empty() {
            return Vec::new();
        }
        let mut p = self.relpath.last().cloned().unwrap_or_default();
        p.extend_from_slice(self.filename.last().map(|f| f.as_slice()).unwrap_or(b""));
        p
    }

    /// `getPreprocValue` (slgh_compile.cc:2597).
    pub fn get_preproc_value(&self, nm: &[u8]) -> Option<Vec<u8>> {
        self.preproc_defines.get(nm).cloned()
    }
    /// `setPreprocValue` (slgh_compile.cc:2609).
    pub fn set_preproc_value(&mut self, nm: &[u8], value: &[u8]) {
        self.preproc_defines.insert(nm.to_vec(), value.to_vec());
    }
    /// `undefinePreprocValue` (slgh_compile.cc:2618).
    pub fn undefine_preproc_value(&mut self, nm: &[u8]) -> bool {
        self.preproc_defines.remove(nm).is_some()
    }

    // --- parser-facing builder methods (slgh_compile.cc:2640-3759) ---

    /// `setEndian` (slgh_compile.cc:2761).
    pub fn set_endian(&mut self, end: i32) {
        self.base.set_big_endian(end == 1);
        if let Err(e) = self.predefined_symbols() {
            self.report_error(&e.explain());
        }
    }

    /// `setAlignment` (slgh_compile.hh:437).
    pub fn set_alignment(&mut self, val: i32) {
        self.base.set_alignment(val);
    }

    /// `newSpace` (slgh_compile.cc:2713).
    pub fn new_space(&mut self, qual: SpaceQuality) {
        if qual.size == 0 {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Space definition '{}' missing size attribute",
                    String::from_utf8_lossy(&qual.name)
                ),
            );
            return;
        }
        let is_register = qual.kind == SpaceType::Register;
        let nm = String::from_utf8_lossy(&qual.name).into_owned();
        let spc = match self
            .base
            .new_processor_space(&nm, qual.size, qual.wordsize, is_register)
        {
            Ok(s) => s,
            Err(e) => {
                self.report_error(&e.explain());
                return;
            }
        };
        if qual.isdefault {
            if self.base.default_code_space().is_some() {
                let loc = self.current_location();
                self.report_error_loc(Some(&loc), "Multiple default spaces");
            } else if let Err(e) = self.base.set_default_code_space(spc.get_index()) {
                self.report_error(&e.explain());
            } else {
                // C++ slgh_compile.cc:2731 `pcode.setDefaultSpace(spc)`.
                self.default_space = Some(Rc::clone(&spc));
            }
        }
        self.add_sleigh_symbol(SleighSymbol::new_space(spc));
    }

    /// `defineVarnodes` (slgh_compile.cc:2776).
    pub fn define_varnodes(&mut self, spacesym: SymbolId, off: u64, size: u64, names: Vec<Vec<u8>>) {
        let spc = match self.base.symtab().find_symbol_by_id(spacesym) {
            Some(s) => match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Space(sp) => Rc::clone(sp.get_space()),
                _ => return,
            },
            None => return,
        };
        let mut myoff = off;
        for nm in &names {
            if nm != b"_" {
                self.add_sleigh_symbol(SleighSymbol::new_varnode(
                    nm,
                    Rc::clone(&spc),
                    myoff,
                    size as i32,
                ));
            }
            myoff += size;
        }
    }

    /// `defineToken` (slgh_compile.cc:2640).
    pub fn define_token(&mut self, name: &[u8], sz: u64, endian: i32) -> SymbolId {
        let mut size = sz as u32;
        if size & 7 != 0 {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "'{}': token size must be multiple of 8",
                    String::from_utf8_lossy(name)
                ),
            );
            size = size / 8 + 1;
        } else {
            size /= 8;
        }
        let is_big = if endian == 0 {
            self.base.is_big_endian()
        } else {
            endian > 0
        };
        let index = self.token_count;
        self.token_count += 1;
        let tok = kuna_sleigh::context::Token::new(name, size as i32, is_big, index as i32);
        self.add_sleigh_symbol(SleighSymbol::new_token(tok))
    }

    /// `addTokenField` (slgh_compile.cc:2668).
    pub fn add_token_field(&mut self, sym: SymbolId, qual: FieldQuality) {
        if qual.high < qual.low {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Field '{}' starts at {} and ends at {}",
                    String::from_utf8_lossy(&qual.name),
                    qual.low,
                    qual.high
                ),
            );
        }
        let (tok_size, tok_big, tok_index) = match self.base.symtab().find_symbol_by_id(sym) {
            Some(s) => match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Token(t) => {
                    let tk = t.get_token();
                    (tk.get_size(), tk.is_big_endian(), tk.get_index())
                }
                _ => return,
            },
            None => return,
        };
        if tok_size * 8 <= qual.high as i32 {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Field '{}' high must be less than token size",
                    String::from_utf8_lossy(&qual.name)
                ),
            );
        }
        let field = kuna_sleigh::slghpatexpress::TokenField::new_for_build(
            tok_size,
            tok_big,
            tok_index,
            qual.signext,
            qual.low as i32,
            qual.high as i32,
        );
        self.add_sleigh_symbol(SleighSymbol::new_value(
            &qual.name,
            PatternValue::TokenField(field),
        ));
    }

    /// `addContextField` (slgh_compile.cc:2690).
    pub fn add_context_field(&mut self, sym: SymbolId, qual: FieldQuality) -> bool {
        if qual.high < qual.low {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Context field '{}' starts at {} and ends at {}",
                    String::from_utf8_lossy(&qual.name),
                    qual.low,
                    qual.high
                ),
            );
        }
        let vsize = match self.base.symtab().find_symbol_by_id(sym) {
            Some(s) => match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Varnode(v) => v.get_size(),
                _ => return false,
            },
            None => return false,
        };
        if vsize * 8 <= qual.high as i32 {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Context field '{}' high must be less than context size",
                    String::from_utf8_lossy(&qual.name)
                ),
            );
        }
        if self.contextlock {
            return false;
        }
        self.contexttable.push(FieldContext { sym, qual });
        true
    }

    /// `defineBitrange` (slgh_compile.cc:2800) -- needs `BitrangeSymbol` (not
    /// yet ported in kuna-sleigh).  Falls back to a plain varnode when the
    /// range is byte-aligned (the common case); a sub-byte bitrange errors.
    pub fn define_bitrange(&mut self, name: &[u8], sym: SymbolId, bitoffset: u32, numb: u32) {
        let (space, offset, vbytes) = match self.base.symtab().find_symbol_by_id(sym) {
            Some(s) => match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Varnode(v) => {
                    let fv = v.get_fixed_varnode();
                    (fv.space.clone(), fv.offset, v.get_size())
                }
                _ => return,
            },
            None => return,
        };
        let size = 8 * vbytes as u32;
        if numb == 0 {
            self.report_error(&format!(
                "'{}': size of bitrange is zero",
                String::from_utf8_lossy(name)
            ));
            return;
        }
        if bitoffset >= size || (bitoffset + numb) > size {
            self.report_error(&format!(
                "'{}': bad bitrange",
                String::from_utf8_lossy(name)
            ));
            return;
        }
        if bitoffset % 8 == 0 && numb % 8 == 0 {
            let newspace = space.expect("varnode has space");
            let mut newoffset = offset;
            let newsize = numb / 8;
            if self.base.is_big_endian() {
                newoffset += u64::from((size - bitoffset - numb) / 8);
            } else {
                newoffset += u64::from(bitoffset / 8);
            }
            self.add_sleigh_symbol(SleighSymbol::new_varnode(
                name,
                newspace,
                newoffset,
                newsize as i32,
            ));
        } else {
            self.report_error(
                "defineBitrange: sub-byte BitrangeSymbol not yet ported (slgh_compile.cc:2800)",
            );
        }
    }

    /// `addUserOp` (slgh_compile.cc:2833).
    pub fn add_user_op(&mut self, names: Vec<Vec<u8>>) {
        for nm in &names {
            let mut sym = SleighSymbol::new_userop(nm);
            if let kuna_sleigh::slghsymbol::SymbolKind::UserOp(u) = sym.kind_mut() {
                u.set_index(self.userop_count as u32);
            }
            self.userop_count += 1;
            self.add_sleigh_symbol(sym);
        }
    }

    /// `dedupSymbolList` (slgh_compile.cc:2849).
    fn dedup_symbol_list(&self, symlist: &mut [SymbolId]) -> Option<SymbolId> {
        let mut res = None;
        for i in 0..symlist.len() {
            let sym = symlist[i];
            if sym == NO_SYMBOL {
                continue;
            }
            for j in (i + 1)..symlist.len() {
                if symlist[j] == sym {
                    res = Some(sym);
                    symlist[j] = NO_SYMBOL;
                }
            }
        }
        res
    }

    /// `attachValues` (slgh_compile.cc:2872).
    pub fn attach_values(&mut self, mut symlist: Vec<SymbolId>, numlist: Vec<i64>) {
        if let Some(dup) = self.dedup_symbol_list(&mut symlist) {
            let nm = self.symbol_name(dup);
            let loc = self.current_location();
            self.report_warning_loc(
                Some(&loc),
                &format!(
                    "'attach values' list contains duplicate entries: {}",
                    String::from_utf8_lossy(&nm)
                ),
            );
        }
        for sym in symlist {
            if sym == NO_SYMBOL {
                continue;
            }
            let patval = match self.value_symbol_patval(sym) {
                Some(p) => p,
                None => continue,
            };
            let maxv = match patval.max_value() { Ok(m) => m, Err(e) => { self.report_error(&e.explain()); continue; } };
            self.check_attach_size(sym, maxv, numlist.len(), "value");
            let nm = self.symbol_name(sym);
            match SleighSymbol::new_valuemap(&nm, patval, numlist.clone()) {
                Ok(newsym) => {
                    let _ = self.base.symtab_mut().replace_symbol(sym, newsym);
                }
                Err(e) => self.report_error(&e.explain()),
            }
        }
    }

    /// `attachNames` (slgh_compile.cc:2900).
    pub fn attach_names(&mut self, mut symlist: Vec<SymbolId>, names: Vec<Vec<u8>>) {
        if let Some(dup) = self.dedup_symbol_list(&mut symlist) {
            let nm = self.symbol_name(dup);
            let loc = self.current_location();
            self.report_warning_loc(
                Some(&loc),
                &format!(
                    "'attach names' list contains duplicate entries: {}",
                    String::from_utf8_lossy(&nm)
                ),
            );
        }
        for sym in symlist {
            if sym == NO_SYMBOL {
                continue;
            }
            let patval = match self.value_symbol_patval(sym) {
                Some(p) => p,
                None => continue,
            };
            let maxv = match patval.max_value() { Ok(m) => m, Err(e) => { self.report_error(&e.explain()); continue; } };
            self.check_attach_size(sym, maxv, names.len(), "name");
            let nm = self.symbol_name(sym);
            match SleighSymbol::new_name_symbol(&nm, patval, names.clone()) {
                Ok(newsym) => { let _ = self.base.symtab_mut().replace_symbol(sym, newsym); }
                Err(e) => self.report_error(&e.explain()),
            }
        }
    }

    /// `attachVarnodes` (slgh_compile.cc:2928).
    pub fn attach_varnodes(&mut self, mut symlist: Vec<SymbolId>, varlist: Vec<SymbolId>) {
        if let Some(dup) = self.dedup_symbol_list(&mut symlist) {
            let nm = self.symbol_name(dup);
            let loc = self.current_location();
            self.report_warning_loc(
                Some(&loc),
                &format!(
                    "'attach variables' list contains duplicate entries: {}",
                    String::from_utf8_lossy(&nm)
                ),
            );
        }
        let var_ids: Vec<Option<u32>> = varlist
            .iter()
            .map(|&v| if v == NO_SYMBOL { None } else { Some(v) })
            .collect();
        let mut sz = 0i32;
        for &v in &varlist {
            if v == NO_SYMBOL {
                continue;
            }
            if let Some(s) = self.base.symtab().find_symbol_by_id(v) {
                if let kuna_sleigh::slghsymbol::SymbolKind::Varnode(vs) = s.kind() {
                    let vsz = vs.get_size();
                    if sz == 0 {
                        sz = vsz;
                    } else if sz != vsz {
                        let loc = self.current_location();
                        self.report_error_loc(
                            Some(&loc),
                            &format!(
                                "Attach statement contains varnodes of different sizes -- {sz} != {vsz}"
                            ),
                        );
                        break;
                    }
                }
            }
        }
        for sym in symlist {
            if sym == NO_SYMBOL {
                continue;
            }
            let patval = match self.value_symbol_patval(sym) {
                Some(p) => p,
                None => continue,
            };
            let maxv = match patval.max_value() { Ok(m) => m, Err(e) => { self.report_error(&e.explain()); continue; } };
            self.check_attach_size(sym, maxv, varlist.len(), "varnode");
            let nm = self.symbol_name(sym);
            match SleighSymbol::new_varnodelist(&nm, patval, var_ids.clone()) {
                Ok(newsym) => {
                    let _ = self.base.symtab_mut().replace_symbol(sym, newsym);
                }
                Err(e) => self.report_error(&e.explain()),
            }
        }
    }

    /// `newTable` (slgh_compile.cc:2968).
    pub fn new_table(&mut self, nm: &[u8]) -> SymbolId {
        let id = self.add_sleigh_symbol(SleighSymbol::new_subtable(nm));
        self.tables.push(id);
        id
    }

    /// `newOperand` (slgh_compile.cc:2984).
    pub fn new_operand(&mut self, ct: u32, nm: &[u8]) {
        let (table_id, ct_idx) = self.ctmap[ct as usize];
        let index = self.constructor(table_id, ct_idx).get_num_operands();
        let ctref = kuna_sleigh::slghsymbol::ConstructorRef { table_id, ct_id: ct_idx };
        let opid = self.add_sleigh_symbol(SleighSymbol::new_operand(nm, index, ctref));
        self.constructor_mut(table_id, ct_idx).add_operand(opid);
    }

    /// `createConstructor` (slgh_compile.cc:3364).
    pub fn create_constructor(&mut self, sym: Option<SymbolId>) -> u32 {
        let mut table_id = sym;
        if table_id.is_none() {
            table_id = self.with_block_current_subtable();
        }
        let table_id = table_id.unwrap_or_else(|| self.base.get_root().expect("root set"));
        self.curmacro = None;
        let lineno = self.lineno.last().copied().unwrap_or(0);
        let loc = self.current_location();
        let src_index = self.base.indexer_mut().index(loc.get_filename());
        let mut ct = kuna_sleigh::slghsymbol::Constructor::new();
        ct.set_parent(table_id);
        ct.set_lineno(lineno);
        ct.set_src_index(src_index);
        let ct_idx = self.constructor_count(table_id);
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(table_id) {
            if let kuna_sleigh::slghsymbol::SymbolKind::Subtable(st) = s.kind_mut() {
                st.add_constructor(ct);
            }
        }
        let id = self.ctmap.len() as u32;
        self.ctmap.push((table_id, ct_idx));
        self.ctor_loc.insert(id, loc);
        self.base.symtab_mut().add_scope();
        self.pcode.local_labelcount = 0; // C++ pcode.resetLabelCount() (cc:3377)
        self.curct = Some(id);
        id
    }

    /// `resetConstructors` (slgh_compile.cc:3384).
    pub fn reset_constructors(&mut self) {
        let global = self.base.symtab().get_global_scope().map(|s| s.get_id());
        self.base.symtab_mut().set_current_scope(global);
    }

    /// `addSyntax` on the current constructor (slghparse.y:275).
    pub fn add_syntax(&mut self, ct: u32, syntax: &[u8]) {
        let (table_id, ct_idx) = self.ctmap[ct as usize];
        self.constructor_mut(table_id, ct_idx).add_syntax(syntax);
    }

    /// `pushWith` (slgh_compile.cc:3676).
    pub fn push_with(&mut self, ss: Option<SymbolId>, pateq: Option<u32>, _contvec: Option<Vec<u32>>) {
        let mut block = WithBlock {
            ss,
            pateq,
            contvec: Vec::new(),
        };
        if block.ss.is_none() {
            block.ss = self.with_block_current_subtable();
        }
        self.withstack.push(block);
    }

    /// `popWith` (slgh_compile.cc:3684).
    pub fn pop_with(&mut self) {
        self.withstack.pop();
    }

    /// `WithBlock::getCurrentSubtable` (slgh_compile.cc:203).
    fn with_block_current_subtable(&self) -> Option<SymbolId> {
        for block in self.withstack.iter().rev() {
            if block.ss.is_some() {
                return block.ss;
            }
        }
        None
    }

    /// `buildConstructor` (slgh_compile.cc:3698).  In the landed subset `vec` is
    /// always `None` (no semantic section: `unimpl` or context-only).
    pub fn build_constructor(
        &mut self,
        big: u32,
        pateq: Option<u32>,
        _contvec: Option<Vec<u32>>,
        vec: Option<SectionVector>,
    ) {
        let (table_id, ct_idx) = self.ctmap[big as usize];
        let mut noerrors = true;
        if vec.is_some() {
            self.report_error(
                "buildConstructor: p-code section finalize not yet ported \
                 (slgh_compile.cc:3436 finalizeSections)",
            );
            noerrors = false;
        }
        if noerrors {
            let pateq = self.collect_and_prepend_pattern(pateq);
            if let Some(eq) = pateq {
                self.constructor_mut(table_id, ct_idx).add_equation(eq);
            } else {
                let eps = self.arena.alloc(PatternEquation::Unconstrained {
                    patex: PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(
                        0,
                    ))),
                });
                self.constructor_mut(table_id, ct_idx).add_equation(eps);
            }
            self.constructor_mut(table_id, ct_idx).remove_trailing_space();
        }
        self.base.symtab_mut().pop_scope();
    }

    /// `WithBlock::collectAndPrependPattern` (slgh_compile.cc:152).
    fn collect_and_prepend_pattern(&mut self, pateq: Option<u32>) -> Option<u32> {
        let mut res = pateq;
        let stack_pats: Vec<u32> = self.withstack.iter().rev().filter_map(|b| b.pateq).collect();
        for wpat in stack_pats {
            res = Some(match res {
                Some(r) => self.arena.alloc(PatternEquation::And {
                    left: wpat,
                    right: r,
                }),
                None => wpat,
            });
        }
        res
    }

    /// `getUniqueAddr` (slgh_compile.cc:2465).
    pub fn get_unique_addr(&mut self) -> u32 {
        let base = self.base.get_unique_base();
        // SleighBase::MAX_UNIQUE_SIZE == 256 (sleighbase.cc:20).
        self.base.set_unique_base(base + 256);
        base
    }

    // --- p-code RTL arena accessors (WS4c) ---
    //
    // A SINGLE arena (`rtl_arena`) of a tagged enum backs every heterogeneous
    // bison semantic value, so ids are globally unique across value kinds
    // (per-arena id counters would collide numerically — a `VarnodeTpl` id `n`
    // and an `ExprTree` id `n` would alias).  The `take_*`/`*_mut` helpers
    // assert the tag, catching any cross-kind threading bug immediately.

    fn alloc_rtl(&mut self, v: RtlValue) -> u32 {
        let id = self.rtl_arena.len() as u32;
        self.rtl_arena.push(Some(v));
        id
    }
    fn alloc_vntpl(&mut self, vn: VarnodeTpl) -> u32 {
        self.alloc_rtl(RtlValue::Varnode(vn))
    }
    fn take_vntpl(&mut self, id: u32) -> VarnodeTpl {
        match self.rtl_arena[id as usize].take() {
            Some(RtlValue::Varnode(v)) => v,
            other => panic!("rtl id {id} is not a VarnodeTpl: {:?}", other.is_some()),
        }
    }
    fn alloc_expr(&mut self, e: ExprTree) -> u32 {
        self.alloc_rtl(RtlValue::Expr(e))
    }
    fn take_expr(&mut self, id: u32) -> ExprTree {
        match self.rtl_arena[id as usize].take() {
            Some(RtlValue::Expr(e)) => e,
            _ => panic!("rtl id {id} is not an ExprTree"),
        }
    }
    fn alloc_oplist(&mut self, ops: Vec<OpTpl>) -> u32 {
        self.alloc_rtl(RtlValue::OpList(ops))
    }
    fn take_oplist(&mut self, id: u32) -> Vec<OpTpl> {
        match self.rtl_arena[id as usize].take() {
            Some(RtlValue::OpList(ops)) => ops,
            _ => panic!("rtl id {id} is not an OpTpl list"),
        }
    }
    fn alloc_star(&mut self, s: StarQuality) -> u32 {
        self.alloc_rtl(RtlValue::Star(s))
    }
    fn take_star(&mut self, id: u32) -> StarQuality {
        match self.rtl_arena[id as usize].take() {
            Some(RtlValue::Star(s)) => s,
            _ => panic!("rtl id {id} is not a StarQuality"),
        }
    }
    fn alloc_section(&mut self, ct: ConstructTpl) -> u32 {
        self.alloc_rtl(RtlValue::Section(ct))
    }
    fn section_mut(&mut self, id: u32) -> &mut ConstructTpl {
        match self.rtl_arena[id as usize].as_mut() {
            Some(RtlValue::Section(ct)) => ct,
            _ => panic!("rtl id {id} is not a section"),
        }
    }
    fn take_section(&mut self, id: u32) -> ConstructTpl {
        match self.rtl_arena[id as usize].take() {
            Some(RtlValue::Section(ct)) => ct,
            _ => panic!("rtl id {id} is not a section"),
        }
    }
    fn put_section(&mut self, id: u32, ct: ConstructTpl) {
        self.rtl_arena[id as usize] = Some(RtlValue::Section(ct));
    }
    fn section_ref(&self, id: u32) -> Option<&ConstructTpl> {
        match self.rtl_arena.get(id as usize).and_then(|s| s.as_ref()) {
            Some(RtlValue::Section(ct)) => Some(ct),
            _ => None,
        }
    }
    fn alloc_secvec(&mut self, v: SectionVector) -> u32 {
        self.alloc_rtl(RtlValue::SecVec(v))
    }
    fn secvec_ref(&self, id: u32) -> &SectionVector {
        match self.rtl_arena.get(id as usize).and_then(|s| s.as_ref()) {
            Some(RtlValue::SecVec(v)) => v,
            _ => panic!("rtl id {id} is not a section vector"),
        }
    }
    fn secvec_mut(&mut self, id: u32) -> &mut SectionVector {
        match self.rtl_arena.get_mut(id as usize).and_then(|s| s.as_mut()) {
            Some(RtlValue::SecVec(v)) => v,
            _ => panic!("rtl id {id} is not a section vector"),
        }
    }
    fn drop_secvec(&mut self, id: u32) {
        self.rtl_arena[id as usize] = None;
    }

    // --- post-parse subsystems ---

    /// `calcContextVarLayout` (slgh_compile.cc:2025).
    fn calc_context_var_layout(&mut self, start: i32, sz: i32, numbits: i32) -> i32 {
        let mut numbits = numbits;
        let sym = self.contexttable[start as usize].sym;
        let vsize = match self.base.symtab().find_symbol_by_id(sym) {
            Some(s) => match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Varnode(v) => v.get_size(),
                _ => 0,
            },
            None => 0,
        };
        if vsize % 4 != 0 {
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                "Invalid size of context register: must be a multiple of 4 bytes",
            );
        }
        let maxbits = vsize * 8 - 1;
        let mut i = 0i32;
        while i < sz {
            let (min, mut max) = {
                let q = &self.contexttable[(i + start) as usize].qual;
                (q.low as i32, q.high as i32)
            };
            if (max - min) > 32 {
                let loc = self.current_location();
                self.report_error_loc(Some(&loc), "Size of bitfield larger than 32 bits");
            }
            if max > maxbits {
                let loc = self.current_location();
                self.report_error_loc(
                    Some(&loc),
                    "Scope of bitfield extends beyond the size of context register",
                );
            }
            let mut j = i + 1;
            while j < sz {
                let qlow = self.contexttable[(j + start) as usize].qual.low as i32;
                let qhigh = self.contexttable[(j + start) as usize].qual.high as i32;
                if qlow <= max {
                    if qhigh > max {
                        max = qhigh;
                    }
                } else {
                    break;
                }
                j += 1;
            }
            let alloc = max - min + 1;
            let startword = numbits / 32;
            let endword = (numbits + alloc - 1) / 32;
            if startword != endword {
                numbits = endword * 32;
            }
            let low = numbits;
            numbits += alloc;
            while i < j {
                let (qlow, qhigh, qsign, qname, qflow) = {
                    let q = &self.contexttable[(i + start) as usize].qual;
                    (q.low as i32, q.high as i32, q.signext, q.name.clone(), q.flow)
                };
                let l = qlow - min + low;
                let h = numbits - 1 - (max - qhigh);
                let field = ContextField::new(qsign, l, h);
                self.add_sleigh_symbol(SleighSymbol::new_context(
                    &qname,
                    field,
                    sym,
                    qlow as u32,
                    qhigh as u32,
                    qflow,
                ));
                i += 1;
            }
        }
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let kuna_sleigh::slghsymbol::SymbolKind::Varnode(v) = s.kind_mut() {
                v.mark_as_context();
            }
        }
        numbits
    }

    /// `buildPatterns` (slgh_compile.cc:2122).
    fn build_patterns(&mut self) -> KunaResult<()> {
        let root = match self.base.get_root() {
            Some(r) => r,
            None => {
                self.report_error("No patterns to match.");
                return Ok(());
            }
        };
        let mut errs: Vec<String> = Vec::new();
        {
            let arena = std::mem::take(&mut self.arena);
            let r = self
                .base
                .symtab_mut()
                .build_subtable_pattern(root, &arena, &mut errs);
            self.arena = arena;
            r?;
        }
        let tables = self.tables.clone();
        for &t in &tables {
            let arena = std::mem::take(&mut self.arena);
            let r = self
                .base
                .symtab_mut()
                .build_subtable_pattern(t, &arena, &mut errs);
            self.arena = arena;
            r?;
        }
        self.apply_handmaps(root);
        for &t in &tables {
            self.apply_handmaps(t);
        }
        if self.subtable_is_error(root) {
            self.errors += 1;
        }
        for &t in &tables {
            if self.subtable_is_error(t) {
                self.errors += 1;
            }
            if self.subtable_pattern_none(t) {
                let loc = self.symbol_loc.get(&t).cloned();
                self.report_warning_loc(loc.as_ref(), "Unreferenced table");
            }
        }
        Ok(())
    }

    /// Apply each constructor's stashed handmap to its `ConstructTpl` sections.
    fn apply_handmaps(&mut self, table_id: SymbolId) {
        let numct = self.constructor_count(table_id);
        for ci in 0..numct {
            let handmap = self.constructor(table_id, ci).get_handmap().to_vec();
            if handmap.is_empty() {
                continue;
            }
            if let Some(h) = self.constructor(table_id, ci).get_templ() {
                if let Some(tpl) = self.base.template_mut(h) {
                    tpl.change_handle_index(&handmap);
                }
            }
            let nsec = self.constructor(table_id, ci).get_num_sections();
            for s in 0..nsec {
                if let Some(h) = self.constructor(table_id, ci).get_named_templ(s) {
                    if let Some(tpl) = self.base.template_mut(h) {
                        tpl.change_handle_index(&handmap);
                    }
                }
            }
        }
    }

    /// `buildDecisionTrees` (slgh_compile.cc:2086).
    fn build_decision_trees(&mut self) -> KunaResult<()> {
        let root = self.base.get_root().expect("root set");
        let mut props = DecisionProperties::new();
        self.base.symtab_mut().build_decision_tree(root, &mut props)?;
        let tables = self.tables.clone();
        for &t in &tables {
            self.base.symtab_mut().build_decision_tree(t, &mut props)?;
        }
        let ident = props.get_ident_errors().len();
        for _ in 0..ident {
            self.errors += 1;
            self.report_error("Constructor has identical pattern to another constructor");
        }
        if !self.lenientconflicterrors {
            let conflict = props.get_conflict_errors().len();
            for _ in 0..conflict {
                self.errors += 1;
                self.report_error(
                    "Constructor pattern cannot be distinguished from another constructor",
                );
            }
        }
        Ok(())
    }

    /// `checkConsistency` (slgh_compile.cc:2148) -- trivial in the landed subset.
    fn check_consistency(&mut self) {
        // The full ConsistencyChecker lives in `consistency.rs`.
        self.check_consistency_real();
    }

    /// `checkLocalCollisions` (slgh_compile.cc:2250) -- no exports in the landed subset.
    fn check_local_collisions(&mut self) {}

    /// `checkNops` (slgh_compile.cc:2277).
    fn check_nops(&mut self) {
        if !self.noplist.is_empty() {
            if self.warnallnops {
                let nops = self.noplist.clone();
                for n in &nops {
                    self.report_warning(&String::from_utf8_lossy(n));
                }
            }
            let count = self.noplist.len();
            self.report_warning(&format!("{count} NOP constructors found"));
            if !self.warnallnops {
                self.report_warning("Use -n switch to list each individually");
            }
        }
    }

    /// `checkCaseSensitivity` (slgh_compile.cc:2297).
    fn check_case_sensitivity(&mut self) {
        if !self.failinsensitivedups {
            return;
        }
        let mut register_map: BTreeMap<Vec<u8>, SymbolId> = BTreeMap::new();
        let global = match self.base.symtab().get_global_scope() {
            Some(g) => g.symbol_ids().collect::<Vec<_>>(),
            None => return,
        };
        let mut collisions: Vec<(SymbolId, SymbolId)> = Vec::new();
        for id in global {
            let (is_proc_varnode, name) = match self.base.symtab().find_symbol_by_id(id) {
                Some(s) => {
                    if s.get_type() != SymbolType::Varnode {
                        continue;
                    }
                    let proc = match s.kind() {
                        kuna_sleigh::slghsymbol::SymbolKind::Varnode(v) => v
                            .get_fixed_varnode()
                            .space
                            .as_ref()
                            .map(|sp| sp.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
                            .unwrap_or(false),
                        _ => false,
                    };
                    (proc, s.get_name().to_vec())
                }
                None => continue,
            };
            if !is_proc_varnode {
                continue;
            }
            let upper: Vec<u8> = name.iter().map(|c| c.to_ascii_uppercase()).collect();
            if let Some(&old) = register_map.get(&upper) {
                collisions.push((id, old));
            } else {
                register_map.insert(upper, id);
            }
        }
        for (id, old) in collisions {
            let n = self.symbol_name(id);
            let on = self.symbol_name(old);
            let loc = self.symbol_loc.get(&id).cloned();
            self.report_error_loc(
                loc.as_ref(),
                &format!(
                    "Name collision: {} --- Duplicate symbol {}",
                    String::from_utf8_lossy(&n),
                    String::from_utf8_lossy(&on)
                ),
            );
        }
    }

    /// `buildXrefs` -- the `.sla` encode does not depend on the varnode_xref
    /// map (rebuilt on decode), so the compile path can skip it.
    fn build_xrefs(&mut self) -> KunaResult<()> {
        Ok(())
    }

    /// `checkUniqueAllocation` (slgh_compile.cc:3638).
    // --- helpers over the symbol table / constructors ---

    fn symbol_name(&self, id: SymbolId) -> Vec<u8> {
        self.base
            .symtab()
            .find_symbol_by_id(id)
            .map(|s| s.get_name().to_vec())
            .unwrap_or_default()
    }

    fn value_symbol_patval(&self, id: SymbolId) -> Option<PatternValue> {
        self.base
            .symtab()
            .find_symbol_by_id(id)
            .and_then(|s| s.get_pattern_value().cloned())
    }

    fn check_attach_size(&mut self, sym: SymbolId, maxv: i64, listlen: usize, kind: &str) {
        if maxv + 1 != listlen as i64 {
            let nm = self.symbol_name(sym);
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "Attach {} '{}' (range 0..{}) is wrong size for list (of {} entries)",
                    kind,
                    String::from_utf8_lossy(&nm),
                    maxv,
                    listlen
                ),
            );
        }
    }

    fn constructor_count(&self, table_id: SymbolId) -> u32 {
        self.base
            .symtab()
            .find_symbol_by_id(table_id)
            .and_then(|s| s.as_subtable())
            .map(|st| st.get_num_constructors() as u32)
            .unwrap_or(0)
    }

    fn constructor(&self, table_id: SymbolId, idx: u32) -> &kuna_sleigh::slghsymbol::Constructor {
        self.base
            .symtab()
            .find_symbol_by_id(table_id)
            .and_then(|s| s.as_subtable())
            .and_then(|st| st.get_constructor(idx).ok())
            .expect("constructor exists")
    }

    fn constructor_mut(
        &mut self,
        table_id: SymbolId,
        idx: u32,
    ) -> &mut kuna_sleigh::slghsymbol::Constructor {
        self.base
            .symtab_mut()
            .find_symbol_by_id_mut(table_id)
            .and_then(|s| s.as_subtable_mut())
            .and_then(|st| st.get_constructor_mut(idx))
            .expect("constructor exists")
    }

    fn subtable_is_error(&self, table_id: SymbolId) -> bool {
        self.base
            .symtab()
            .find_symbol_by_id(table_id)
            .and_then(|s| s.as_subtable())
            .map(|st| st.is_error())
            .unwrap_or(false)
    }

    fn subtable_pattern_none(&self, table_id: SymbolId) -> bool {
        self.base
            .symtab()
            .find_symbol_by_id(table_id)
            .and_then(|s| s.as_subtable())
            .map(|st| st.get_pattern().is_none())
            .unwrap_or(true)
    }

    // --- pattern equation / expression builders ---

    fn patexp_get(&self, id: u32) -> PatternExpression {
        self.patexp[id as usize].clone()
    }

    fn family_patval(&self, fam: SymbolId) -> Option<PatternValue> {
        self.value_symbol_patval(fam)
    }

    fn build_cmp_equation(&mut self, fam: SymbolId, rhs: u32, mk: CmpKind) -> u32 {
        let lhs = match self.family_patval(fam) {
            Some(p) => p,
            None => {
                self.report_error("comparison on non-family symbol");
                PatternValue::ConstantValue(ConstantValue::new(0))
            }
        };
        let rhs = self.patexp_get(rhs);
        let eq = match mk {
            CmpKind::Equal => PatternEquation::Equal { lhs, rhs },
            CmpKind::NotEqual => PatternEquation::NotEqual { lhs, rhs },
            CmpKind::Less => PatternEquation::Less { lhs, rhs },
            CmpKind::LessEqual => PatternEquation::LessEqual { lhs, rhs },
            CmpKind::Greater => PatternEquation::Greater { lhs, rhs },
            CmpKind::GreaterEqual => PatternEquation::GreaterEqual { lhs, rhs },
        };
        self.arena.alloc(eq)
    }

    fn build_binexp(&mut self, l: u32, r: u32, mk: BinKind) -> u32 {
        let left = self.patexp_get(l);
        let right = self.patexp_get(r);
        let be = kuna_sleigh::slghpatexpress::BinaryExpression::new(left, right);
        let e = match mk {
            BinKind::Plus => PatternExpression::Plus(be),
            BinKind::Sub => PatternExpression::Sub(be),
            BinKind::Mult => PatternExpression::Mult(be),
            BinKind::LeftShift => PatternExpression::LeftShift(be),
            BinKind::RightShift => PatternExpression::RightShift(be),
            BinKind::And => PatternExpression::And(be),
            BinKind::Or => PatternExpression::Or(be),
            BinKind::Xor => PatternExpression::Xor(be),
            BinKind::Div => PatternExpression::Div(be),
        };
        let id = self.patexp.len() as u32;
        self.patexp.push(e);
        id
    }
}

#[derive(Clone, Copy)]
enum CmpKind {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Clone, Copy)]
enum BinKind {
    Plus,
    Sub,
    Mult,
    LeftShift,
    RightShift,
    And,
    Or,
    Xor,
    Div,
}

// ===========================================================================
// Path/file helpers (FileManage::splitPath / isAbsolutePath)
// ===========================================================================

/// C++ `FileManage::splitPath(full, path, base)`.
fn split_path(full: &[u8]) -> (Vec<u8>, Vec<u8>) {
    match full.iter().rposition(|&c| c == b'/') {
        Some(pos) => (full[..=pos].to_vec(), full[pos + 1..].to_vec()),
        None => (Vec::new(), full.to_vec()),
    }
}

/// C++ `FileManage::isAbsolutePath`.
fn is_absolute_path(p: &[u8]) -> bool {
    p.first() == Some(&b'/')
}

// ===========================================================================
// CompilerHost (WS3 seam: SleighPcode/MacroBuilder back-pointer)
// ===========================================================================

/// C++ `UNIQUE_CROSSBUILD_POSITION` / `UNIQUE_CROSSBUILD_NUMBITS`
/// (slgh_compile.cc): the bit field carved out of the unique-space offset for
/// the run-time crossbuild collision avoidance.
const UNIQUE_CROSSBUILD_POSITION: u32 = 16;
const UNIQUE_CROSSBUILD_NUMBITS: u32 = 8;

/// C++ `SleighCompile::insertCrossBuildRegion(uintb addr)` (slgh_compile.cc:3566).
fn insert_cross_build_region(addr: u64) -> u64 {
    let upperbits = (addr >> UNIQUE_CROSSBUILD_POSITION)
        << (UNIQUE_CROSSBUILD_POSITION + UNIQUE_CROSSBUILD_NUMBITS);
    let lowerbits =
        (addr << (64 - UNIQUE_CROSSBUILD_POSITION)) >> (64 - UNIQUE_CROSSBUILD_POSITION);
    upperbits | lowerbits
}

/// C++ `SleighCompile::shiftUniqueVn(VarnodeTpl *vn)` (slgh_compile.cc:3577).
fn shift_unique_vn(vn: &mut VarnodeTpl) {
    if vn.get_space().is_unique_space() && vn.get_offset().get_type() == ConstType::Real {
        let val = insert_cross_build_region(vn.get_offset().get_real());
        vn.set_offset(val);
    }
}

/// C++ `SleighCompile::shiftUniqueOp(OpTpl *op)` (slgh_compile.cc:3589).
fn shift_unique_op(op: &mut OpTpl) {
    if let Some(outvn) = op.get_out_mut() {
        shift_unique_vn(outvn);
    }
    for i in 0..op.num_input() {
        shift_unique_vn(op.get_in_mut(i));
    }
}

/// C++ `SleighCompile::shiftUniqueHandle(HandleTpl *hand)` (slgh_compile.cc:3602).
fn shift_unique_handle(hand: &mut HandleTpl) {
    if hand.get_space().is_unique_space()
        && hand.get_ptr_space().get_type() == ConstType::Real
        && hand.get_ptr_offset().get_type() == ConstType::Real
    {
        let val = insert_cross_build_region(hand.get_ptr_offset().get_real());
        hand.set_ptr_offset(val);
    } else if hand.get_ptr_space().is_unique_space()
        && hand.get_ptr_offset().get_type() == ConstType::Real
    {
        let val = insert_cross_build_region(hand.get_ptr_offset().get_real());
        hand.set_ptr_offset(val);
    }
    if hand.get_temp_space().is_unique_space()
        && hand.get_temp_offset().get_type() == ConstType::Real
    {
        let val = insert_cross_build_region(hand.get_temp_offset().get_real());
        hand.set_temp_offset(val);
    }
}

/// C++ `SleighCompile::shiftUniqueConstruct(ConstructTpl *tpl)` (slgh_compile.cc:3624).
fn shift_unique_construct(tpl: &mut ConstructTpl) {
    if let Some(result) = tpl.get_result_mut() {
        shift_unique_handle(result);
    }
    for op in tpl.get_opvec_mut().iter_mut() {
        shift_unique_op(op);
    }
}

/// C++ `SleighCompile::findSize(const ConstTpl &offset,const ConstructTpl *ct)`
/// (slgh_compile.cc:3512): find a local temporary varnode with the given offset,
/// returning a copy of its size const.
fn find_size(offset: &ConstTpl, ct: &ConstructTpl) -> Option<ConstTpl> {
    for op in ct.get_opvec() {
        if let Some(vn) = op.get_out() {
            if vn.is_local_temp() && vn.get_offset() == offset {
                return Some(vn.get_size().clone());
            }
        }
        for j in 0..op.num_input() {
            let vn = op.get_in(j);
            if vn.is_local_temp() && vn.get_offset() == offset {
                return Some(vn.get_size().clone());
            }
        }
    }
    None
}

/// C++ `contextMod` test: does the expression use `inst_next` (EndInstructionValue)
/// or `inst_next2` (Next2InstructionValue)?
fn pattern_expression_uses_end_or_next2(pe: &PatternExpression) -> bool {
    let mut list: Vec<&PatternValue> = Vec::new();
    pe.list_values(&mut list);
    list.iter().any(|v| {
        matches!(
            v,
            PatternValue::EndInstructionValue(_) | PatternValue::Next2InstructionValue(_)
        )
    })
}

/// Map a parser-level [`PcodeOpc`] to the `kuna_num` [`OpCode`] the
/// `PcodeCompile` builders expect (the C++ `CPUI_*` constant the bison action
/// passes to `pcode.createOp`).
fn pcode_opc_to_opcode(opc: PcodeOpc) -> OpCode {
    use OpCode::*;
    use PcodeOpc as P;
    match opc {
        P::IntAdd => CPUI_INT_ADD,
        P::IntSub => CPUI_INT_SUB,
        P::IntEqual => CPUI_INT_EQUAL,
        P::IntNotEqual => CPUI_INT_NOTEQUAL,
        P::IntLess => CPUI_INT_LESS,
        P::IntLessEqual => CPUI_INT_LESSEQUAL,
        P::IntSless => CPUI_INT_SLESS,
        P::IntSlessEqual => CPUI_INT_SLESSEQUAL,
        P::Int2Comp => CPUI_INT_2COMP,
        P::IntNegate => CPUI_INT_NEGATE,
        P::IntXor => CPUI_INT_XOR,
        P::IntAnd => CPUI_INT_AND,
        P::IntOr => CPUI_INT_OR,
        P::IntLeft => CPUI_INT_LEFT,
        P::IntRight => CPUI_INT_RIGHT,
        P::IntSright => CPUI_INT_SRIGHT,
        P::IntMult => CPUI_INT_MULT,
        P::IntDiv => CPUI_INT_DIV,
        P::IntSdiv => CPUI_INT_SDIV,
        P::IntRem => CPUI_INT_REM,
        P::IntSrem => CPUI_INT_SREM,
        P::BoolNegate => CPUI_BOOL_NEGATE,
        P::BoolXor => CPUI_BOOL_XOR,
        P::BoolAnd => CPUI_BOOL_AND,
        P::BoolOr => CPUI_BOOL_OR,
        P::FloatEqual => CPUI_FLOAT_EQUAL,
        P::FloatNotEqual => CPUI_FLOAT_NOTEQUAL,
        P::FloatLess => CPUI_FLOAT_LESS,
        P::FloatLessEqual => CPUI_FLOAT_LESSEQUAL,
        P::FloatAdd => CPUI_FLOAT_ADD,
        P::FloatSub => CPUI_FLOAT_SUB,
        P::FloatMult => CPUI_FLOAT_MULT,
        P::FloatDiv => CPUI_FLOAT_DIV,
        P::FloatNeg => CPUI_FLOAT_NEG,
        P::FloatAbs => CPUI_FLOAT_ABS,
        P::FloatSqrt => CPUI_FLOAT_SQRT,
        P::IntSext => CPUI_INT_SEXT,
        P::IntZext => CPUI_INT_ZEXT,
        P::IntCarry => CPUI_INT_CARRY,
        P::IntScarry => CPUI_INT_SCARRY,
        P::IntSborrow => CPUI_INT_SBORROW,
        P::FloatFloat2Float => CPUI_FLOAT_FLOAT2FLOAT,
        P::FloatInt2Float => CPUI_FLOAT_INT2FLOAT,
        P::FloatNan => CPUI_FLOAT_NAN,
        P::FloatTrunc => CPUI_FLOAT_TRUNC,
        P::FloatCeil => CPUI_FLOAT_CEIL,
        P::FloatFloor => CPUI_FLOAT_FLOOR,
        P::FloatRound => CPUI_FLOAT_ROUND,
        P::New => CPUI_NEW,
        P::Popcount => CPUI_POPCOUNT,
        P::Lzcount => CPUI_LZCOUNT,
        P::Subpiece => CPUI_SUBPIECE,
        P::Cpoolref => CPUI_CPOOLREF,
        P::Branch => CPUI_BRANCH,
        P::Cbranch => CPUI_CBRANCH,
        P::BranchInd => CPUI_BRANCHIND,
        P::Call => CPUI_CALL,
        P::CallInd => CPUI_CALLIND,
        P::Return => CPUI_RETURN,
    }
}

// ===========================================================================
// WS4c: the p-code section RTL build path
//
// Inherent methods backing the parser's p-code-section actions, plus the
// per-constructor section finalize (finalizeSections / forceExportSize /
// expandMacros), macro definition/use, and crossbuild unique allocation.
// ===========================================================================

impl SleighCompile {
    /// C++ `SleighCompile::recordNop()` (slgh_compile.cc:3758): record a NOP
    /// constructor at the current location for later reporting.
    pub fn record_nop(&mut self) {
        let loc = self.current_location();
        let msg = SleighCompile::format_status_message(Some(&loc), "NOP detected");
        self.noplist.push(msg.into_bytes());
    }

    /// C++ `SleighCompile::newSectionSymbol(const string &nm)`
    /// (slgh_compile.cc:2742): find or create the named section symbol.
    pub fn new_section_symbol(&mut self, nm: &[u8]) -> SymbolId {
        // C++: SectionSymbol *sym = (SectionSymbol*)symtab.findSymbol(nm);
        // create one if absent (using sections.size() as the template id).
        if let Some(existing) = self.base.symtab().find_symbol(nm) {
            let id = existing.get_id();
            let is_section = existing.get_type() == SymbolType::Section;
            if is_section {
                return id;
            }
            // C++ reports a parse error for a name clash.
            let loc = self.current_location();
            self.report_error_loc(
                Some(&loc),
                &format!(
                    "'{}' is already defined as a different type of symbol",
                    String::from_utf8_lossy(nm)
                ),
            );
            return id;
        }
        let templateid = self.sections.len() as i32;
        let sym = SleighSymbol::new(nm, SymbolKind::Section(SectionSymbol::new(templateid)));
        let id = self.add_sleigh_symbol(sym);
        self.sections.push(id);
        id
    }

    /// C++ `SleighCompile::enterSection()` (slgh_compile.cc:3351): a fresh empty
    /// ConstructTpl section; resets the label count.
    pub fn enter_section(&mut self) -> u32 {
        self.pcode.local_labelcount = 0; // C++ pcode.resetLabelCount()
        self.alloc_section(ConstructTpl::new())
    }

    /// C++ `rtl: rtlmid` finalize (slghparse.y:355): if the section produced
    /// neither ops nor a result, record a NOP.  Returns the section id.
    pub fn finish_main_rtl(&mut self, rtlmid: u32) -> u32 {
        let isnop = {
            let sec = self.section_mut(rtlmid);
            sec.get_opvec().is_empty() && sec.get_result().is_none()
        };
        if isnop {
            self.record_nop();
        }
        rtlmid
    }

    /// C++ `SleighCompile::setResultVarnode(ConstructTpl *ct,VarnodeTpl *vn)`
    /// (slgh_compile.cc:3099).
    pub fn set_result_varnode(&mut self, ct: u32, vn: u32) -> u32 {
        let vn = self.take_vntpl(vn);
        let res = HandleTpl::new_from_varnode(&vn);
        self.section_mut(ct).set_result(Some(res));
        ct
    }

    /// C++ `SleighCompile::setResultStarVarnode(ConstructTpl *ct,StarQuality *star,VarnodeTpl *vn)`
    /// (slgh_compile.cc:3115).
    pub fn set_result_star_varnode(&mut self, ct: u32, star: u32, vn: u32) -> u32 {
        let star = self.take_star(star);
        let vn = self.take_vntpl(vn);
        let uspace = self.get_unique_space_rc();
        let uaddr = self.get_unique_addr();
        let res = HandleTpl::new_ptr(
            &star.id,
            &ConstTpl::new_real(ConstType::Real, u64::from(star.size)),
            &vn,
            uspace,
            u64::from(uaddr),
        );
        self.section_mut(ct).set_result(Some(res));
        ct
    }

    /// C++ `rtlmid statement` (slghparse.y:362): append the statement's ops.
    /// Returns false on a multiple-delayslot error (C++ `addOpList`).
    pub fn rtl_add_oplist(&mut self, sec: u32, stmt: u32) -> bool {
        let ops = self.take_oplist(stmt);
        self.section_mut(sec).add_op_list(ops)
    }

    /// C++ `SleighCompile::standaloneSection(ConstructTpl *main)`
    /// (slgh_compile.cc:3257).
    pub fn standalone_section(&mut self, main: u32) -> u32 {
        let scope = self.base.symtab().get_current_scope();
        let sv = SectionVector::new(main, scope);
        self.alloc_secvec(sv)
    }

    /// C++ `SleighCompile::firstNamedSection(ConstructTpl *main,SectionSymbol *sym)`
    /// (slgh_compile.cc:3272).
    pub fn first_named_section(&mut self, main: u32, sym: SymbolId) -> u32 {
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let Some(sec) = s.as_section_mut() {
                sec.increment_define_count();
            }
        }
        let curscope = self.base.symtab().get_current_scope();
        self.base.symtab_mut().add_scope();
        let mut sv = SectionVector::new(main, curscope);
        let templateid = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_section())
            .map(|sec| sec.get_template_id())
            .unwrap_or(0);
        sv.set_next_index(templateid);
        self.alloc_secvec(sv)
    }

    /// C++ `SleighCompile::nextNamedSection(SectionVector *vec,ConstructTpl *section,SectionSymbol *sym)`
    /// (slgh_compile.cc:3295).
    pub fn next_named_section(&mut self, vec: u32, section: u32, sym: SymbolId) -> u32 {
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let Some(sec) = s.as_section_mut() {
                sec.increment_define_count();
            }
        }
        let curscope = self.base.symtab().get_current_scope();
        self.base.symtab_mut().pop_scope(); // Pop last named section scope
        self.base.symtab_mut().add_scope(); // New scope under the Constructor
        self.secvec_mut(vec).append(section, curscope);
        let templateid = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_section())
            .map(|sec| sec.get_template_id())
            .unwrap_or(0);
        self.secvec_mut(vec).set_next_index(templateid);
        vec
    }

    /// C++ `SleighCompile::finalNamedSection(SectionVector *vec,ConstructTpl *section)`
    /// (slgh_compile.cc:3317).
    pub fn final_named_section(&mut self, vec: u32, section: u32) -> u32 {
        let curscope = self.base.symtab().get_current_scope();
        self.secvec_mut(vec).append(section, curscope);
        self.base.symtab_mut().pop_scope(); // Pop the section scope
        vec
    }

    fn get_unique_space_rc(&self) -> Rc<AddrSpace> {
        self.unique_space
            .clone()
            .or_else(|| self.base.unique_space())
            .expect("unique space")
    }
    fn get_constant_space_rc(&self) -> Rc<AddrSpace> {
        self.constant_space
            .clone()
            .or_else(|| self.base.constant_space())
            .expect("constant space")
    }
    fn get_default_code_space_rc(&self) -> Rc<AddrSpace> {
        self.default_space
            .clone()
            .or_else(|| self.base.default_code_space())
            .expect("default code space")
    }

    // -----------------------------------------------------------------------
    // p-code statement / expression / varnode builders (the bison `expr` /
    // `statement` / `varnode` / `jumpdest` / `sizedstar` actions; all forward
    // to the inherited `PcodeCompile` machinery).
    // -----------------------------------------------------------------------

    /// `statement: lhsvarnode '=' expr ';'` (slghparse.y:366).
    pub fn stmt_assign(&mut self, lhs: u32, expr: u32) -> u32 {
        let lhsvn = self.take_vntpl(lhs);
        let mut e = self.take_expr(expr);
        if let Err(err) = e.set_output(lhsvn) {
            self.report_current_error(&err.explain());
        }
        let ops = ExprTree::to_vector(e);
        self.alloc_oplist(ops)
    }

    /// `expr: varnode` (slghparse.y:392): `new ExprTree($1)`.
    pub fn expr_from_varnode(&mut self, vn: u32) -> u32 {
        let v = self.take_vntpl(vn);
        self.alloc_expr(ExprTree::new(v))
    }

    /// `expr op expr` / unary `expr` (slghparse.y) -> `pcode.createOp`.
    pub fn pcode_create_op(&mut self, opc: PcodeOpc, a: u32, b: Option<u32>) -> u32 {
        let oc = pcode_opc_to_opcode(opc);
        let av = self.take_expr(a);
        let res = match b {
            None => self.create_op(oc, av),
            Some(bid) => {
                let bv = self.take_expr(bid);
                self.create_op2(oc, av, bv)
            }
        };
        self.alloc_expr(res)
    }

    /// `goto`/`call`/`return`/`cbranch` statements -> `pcode.createOpNoOut`.
    ///
    /// The `a` argument arrives as a `jumpdest` (a [`VarnodeTpl`] id, wrapped
    /// here in a fresh `ExprTree` as the C++ `new ExprTree($2)`) for the direct
    /// BRANCH/CALL forms and for the CBRANCH destination; for the indirect
    /// BRANCHIND/CALLIND/RETURN forms it is already an `ExprTree` id.
    pub fn pcode_create_op_noout(&mut self, opc: PcodeOpc, a: u32, cond: Option<u32>) -> u32 {
        let oc = pcode_opc_to_opcode(opc);
        let av = match opc {
            // Direct branch/call destinations and the CBRANCH dest are jumpdests.
            PcodeOpc::Branch | PcodeOpc::Call | PcodeOpc::Cbranch => {
                let vn = self.take_vntpl(a);
                ExprTree::new(vn)
            }
            // Indirect forms (BRANCHIND/CALLIND/RETURN) are already expressions.
            _ => self.take_expr(a),
        };
        let ops = match cond {
            None => self.create_op_no_out(oc, av),
            Some(cid) => {
                let cv = self.take_expr(cid);
                // C++ `createOpNoOut(CPUI_CBRANCH, dest, cond)`: dest then cond.
                self.create_op_no_out2(oc, av, cv)
            }
        };
        self.alloc_oplist(ops)
    }

    /// `BUILD`/`DELAY_SLOT` statements -> `pcode.createOpConst`.
    pub fn pcode_create_op_const(&mut self, op: ConstOp, val: u64) -> u32 {
        let oc = match op {
            ConstOp::Build => BUILD,
            ConstOp::DelaySlot => DELAY_SLOT,
        };
        let ops = self.create_op_const(oc, val);
        self.alloc_oplist(ops)
    }

    /// `*[space]:sz expr '=' expr` -> `pcode.createStore`.
    pub fn pcode_create_store(&mut self, star: u32, ptr: u32, val: u32) -> u32 {
        let s = self.take_star(star);
        let p = self.take_expr(ptr);
        let v = self.take_expr(val);
        let ops = match self.create_store(s, p, v) {
            Ok(ops) => ops,
            Err(err) => {
                self.report_current_error(&err.explain());
                Vec::new()
            }
        };
        self.alloc_oplist(ops)
    }

    /// `*[space]:sz expr` load -> `pcode.createLoad`.
    pub fn pcode_create_load(&mut self, star: u32, ptr: u32) -> u32 {
        let s = self.take_star(star);
        let p = self.take_expr(ptr);
        let res = match self.create_load(s, p) {
            Ok(e) => e,
            Err(err) => {
                self.report_current_error(&err.explain());
                ExprTree::default()
            }
        };
        self.alloc_expr(res)
    }

    /// `USEROPSYM '(' paramlist ')'` no-out statement.
    pub fn pcode_create_user_op_noout(&mut self, sym: SymbolId, params: Vec<u32>) -> u32 {
        let pv: Vec<ExprTree> = params.into_iter().map(|p| self.take_expr(p)).collect();
        let usym = self.userop_symbol_clone(sym);
        let ops = match usym {
            Some(u) => self.create_user_op_no_out(&u, pv),
            None => Vec::new(),
        };
        self.alloc_oplist(ops)
    }

    /// `USEROPSYM '(' paramlist ')'` expr.
    pub fn pcode_create_user_op(&mut self, sym: SymbolId, params: Vec<u32>) -> u32 {
        let pv: Vec<ExprTree> = params.into_iter().map(|p| self.take_expr(p)).collect();
        let usym = self.userop_symbol_clone(sym);
        let res = match usym {
            Some(u) => self.create_user_op(&u, pv),
            None => ExprTree::default(),
        };
        self.alloc_expr(res)
    }

    /// `OP_CPOOLREF '(' paramlist ')'` -> `pcode.createVariadic(CPUI_CPOOLREF,...)`.
    pub fn pcode_create_variadic_cpoolref(&mut self, params: Vec<u32>) -> u32 {
        let pv: Vec<ExprTree> = params.into_iter().map(|p| self.take_expr(p)).collect();
        let res = self.create_variadic(OpCode::CPUI_CPOOLREF, pv);
        self.alloc_expr(res)
    }

    /// `specificsymbol '(' integervarnode ')'` -> SUBPIECE
    /// (`createOp(CPUI_SUBPIECE, ExprTree(sym->getVarnode()), ExprTree(intvn))`).
    /// `off` is the raw `integervarnode: INTEGER` value; build its constant
    /// varnode here (C++ `new VarnodeTpl(constspace, real(off), real(0))`).
    pub fn pcode_create_subpiece(&mut self, spec: SymbolId, off: u32) -> u32 {
        let vn = self.symbol_varnode(spec);
        let cs = self.get_constant_space_rc();
        let offvn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, u64::from(off)),
            ConstTpl::new_real(ConstType::Real, 0),
        );
        let a = ExprTree::new(vn);
        let b = ExprTree::new(offvn);
        let res = self.create_op2(OpCode::CPUI_SUBPIECE, a, b);
        self.alloc_expr(res)
    }

    /// `specificsymbol ':' INTEGER` -> `createBitRange(sym,0,nbytes*8)`.
    pub fn pcode_create_bitrange_colon(&mut self, spec: SymbolId, nbytes: u64) -> u32 {
        // C++ `createBitRange($1,0,(uint4)(*$3 * 8))`.
        self.create_bit_range_action(spec, 0, (nbytes as u32).wrapping_mul(8))
    }

    /// `specificsymbol '[' INTEGER ',' INTEGER ']'` -> `createBitRange(sym,off,size)`.
    pub fn pcode_create_bitrange_idx(&mut self, spec: SymbolId, off: u32, size: u32) -> u32 {
        self.create_bit_range_action(spec, off, size)
    }

    /// `BITSYM` -> `createBitRange(bitsym->getParentSymbol(),bitoffset,numbits)`.
    pub fn pcode_create_bitrange_bitsym(&mut self, bitsym: SymbolId) -> u32 {
        let (parent, off, num) = self
            .base
            .symtab()
            .find_symbol_by_id(bitsym)
            .and_then(|s| s.as_bitrange())
            .map(|b| (b.get_parent_symbol(), b.get_bit_offset(), b.num_bits()))
            .expect("bitrange symbol");
        self.create_bit_range_action(parent, off, num)
    }

    fn create_bit_range_action(&mut self, spec: SymbolId, off: u32, size: u32) -> u32 {
        // C++ `pcode.createBitRange(sym, off, size)`: the trait takes the
        // symbol's varnode + its name (for error messages / temp naming).
        let vn = self.symbol_varnode(spec);
        let name = self.symbol_name(spec);
        let res = match self.create_bit_range(vn, &name, off, size) {
            Ok(e) => e,
            Err(err) => {
                self.report_current_error(&err.explain());
                ExprTree::default()
            }
        };
        self.alloc_expr(res)
    }

    /// `lhsvarnode lvalue '[' off ',' size ']' '=' expr` -> assignBitRange.
    pub fn pcode_assign_bitrange_idx(&mut self, lhs: u32, off: u32, size: u32, expr: u32) -> u32 {
        let vn = self.take_vntpl(lhs);
        let e = self.take_expr(expr);
        let ops = match self.assign_bit_range(vn, off, size, e) {
            Ok(ops) => ops,
            Err(err) => {
                self.report_current_error(&err.explain());
                Vec::new()
            }
        };
        self.alloc_oplist(ops)
    }

    /// `BITSYM '=' expr` -> assignBitRange via the bitrange's parent varnode.
    pub fn pcode_assign_bitrange_bitsym(&mut self, bitsym: SymbolId, expr: u32) -> u32 {
        let (parent, off, num) = self
            .base
            .symtab()
            .find_symbol_by_id(bitsym)
            .and_then(|s| s.as_bitrange())
            .map(|b| (b.get_parent_symbol(), b.get_bit_offset(), b.num_bits()))
            .expect("bitrange symbol");
        let vn = self.symbol_varnode(parent);
        let e = self.take_expr(expr);
        let ops = match self.assign_bit_range(vn, off, num, e) {
            Ok(ops) => ops,
            Err(err) => {
                self.report_current_error(&err.explain());
                Vec::new()
            }
        };
        self.alloc_oplist(ops)
    }

    /// `String '=' expr` / `String ':' INTEGER '=' expr` -> `pcode.newOutput`.
    pub fn pcode_new_output(
        &mut self,
        islocal: bool,
        expr: u32,
        name: &[u8],
        size: Option<u64>,
    ) -> u32 {
        let e = self.take_expr(expr);
        let sz = size.unwrap_or(0) as u32;
        let ops = match self.new_output(islocal, e, name, sz) {
            Ok(ops) => ops,
            Err(err) => {
                self.report_current_error(&err.explain());
                Vec::new()
            }
        };
        self.alloc_oplist(ops)
    }

    /// `LOCAL_KEY String [':' INTEGER]` declaration -> `pcode.newLocalDefinition`.
    pub fn pcode_new_local_definition(&mut self, name: &[u8], size: Option<u64>) {
        let sz = size.unwrap_or(0) as u32;
        self.new_local_definition(name, sz);
    }

    /// `sizedstar` builders (slghparse.y:485-488).
    pub fn sizedstar_space_sz(&mut self, spacesym: SymbolId, size: u64) -> u32 {
        let spc = self.space_symbol_space(spacesym);
        self.alloc_star(StarQuality {
            id: ConstTpl::new_space(spc),
            size: size as u32,
        })
    }
    pub fn sizedstar_space(&mut self, spacesym: SymbolId) -> u32 {
        let spc = self.space_symbol_space(spacesym);
        self.alloc_star(StarQuality {
            id: ConstTpl::new_space(spc),
            size: 0,
        })
    }
    pub fn sizedstar_default_sz(&mut self, size: u64) -> u32 {
        let spc = self.get_default_code_space_rc();
        self.alloc_star(StarQuality {
            id: ConstTpl::new_space(spc),
            size: size as u32,
        })
    }
    pub fn sizedstar_default(&mut self) -> u32 {
        let spc = self.get_default_code_space_rc();
        self.alloc_star(StarQuality {
            id: ConstTpl::new_space(spc),
            size: 0,
        })
    }

    /// `jumpdest` builders (slghparse.y:490-497).
    pub fn jumpdest_jumpsym(&mut self, sym: SymbolId) -> u32 {
        // `VarnodeTpl *sym = $1->getVarnode(); new VarnodeTpl(j_curspace,
        // sym->getOffset(), j_curspace_size); delete sym;`
        let symvn = self.symbol_varnode(sym);
        let vn = VarnodeTpl::new(
            ConstTpl::new_type(ConstType::JCurspace),
            symvn.get_offset().clone(),
            ConstTpl::new_type(ConstType::JCurspaceSize),
        );
        self.alloc_vntpl(vn)
    }
    pub fn jumpdest_integer(&mut self, val: u64) -> u32 {
        let vn = VarnodeTpl::new(
            ConstTpl::new_type(ConstType::JCurspace),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_type(ConstType::JCurspaceSize),
        );
        self.alloc_vntpl(vn)
    }
    pub fn jumpdest_operandsym(&mut self, sym: SymbolId) -> u32 {
        // `$$ = $1->getVarnode(); $1->setCodeAddress();`
        let vn = self.symbol_varnode(sym);
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let SymbolKind::Operand(op) = s.kind_mut() {
                op.set_code_address();
            }
        }
        self.alloc_vntpl(vn)
    }
    pub fn jumpdest_integer_space(&mut self, val: u64, spacesym: SymbolId) -> u32 {
        let spc = self.space_symbol_space(spacesym);
        let addr_size = spc.get_addr_size();
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(spc),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_real(ConstType::Real, u64::from(addr_size)),
        );
        self.alloc_vntpl(vn)
    }
    pub fn jumpdest_label(&mut self, label: SymbolId) -> u32 {
        // `new VarnodeTpl(constspace, j_relative(label->getIndex()),
        // real(sizeof(uintm))); label->incrementRefCount();`
        let idx = self
            .base
            .symtab()
            .find_symbol_by_id(label)
            .and_then(|s| s.as_label())
            .map(|l| l.get_index())
            .unwrap_or(0);
        let cs = self.get_constant_space_rc();
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::JRelative, u64::from(idx)),
            ConstTpl::new_real(ConstType::Real, 4), // sizeof(uintm) == 4
        );
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(label) {
            if let Some(l) = s.as_label_mut() {
                l.increment_ref_count();
            }
        }
        self.alloc_vntpl(vn)
    }

    /// `varnode: specificsymbol` / `lhsvarnode: specificsymbol` /
    /// `exportvarnode: specificsymbol` -> `sym->getVarnode()`.
    pub fn varnode_spec(&mut self, spec: SymbolId) -> u32 {
        let vn = self.symbol_varnode(spec);
        self.alloc_vntpl(vn)
    }
    pub fn lhsvarnode_spec(&mut self, spec: SymbolId) -> u32 {
        let vn = self.symbol_varnode(spec);
        self.alloc_vntpl(vn)
    }
    pub fn exportvarnode_spec(&mut self, spec: SymbolId) -> u32 {
        let vn = self.symbol_varnode(spec);
        self.alloc_vntpl(vn)
    }
    pub fn exportvarnode_integer_colon(&mut self, val: u64, size: u64) -> u32 {
        let cs = self.get_constant_space_rc();
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_real(ConstType::Real, size),
        );
        self.alloc_vntpl(vn)
    }

    /// `integervarnode: INTEGER` (slghparse.y:476).
    pub fn intvn_integer(&mut self, val: u64) -> u32 {
        let cs = self.get_constant_space_rc();
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_real(ConstType::Real, 0),
        );
        self.alloc_vntpl(vn)
    }
    /// `integervarnode: INTEGER ':' INTEGER` (slghparse.y:478).
    pub fn intvn_integer_colon(&mut self, val: u64, size: u64) -> u32 {
        let cs = self.get_constant_space_rc();
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_real(ConstType::Real, size),
        );
        self.alloc_vntpl(vn)
    }
    /// `integervarnode: '&' varnode` / `'&' ':' INTEGER varnode` -> addressOf.
    pub fn pcode_address_of(&mut self, vn: u32, size: u64) -> u32 {
        let v = self.take_vntpl(vn);
        let res = self.address_of(v, size as u32);
        self.alloc_vntpl(res)
    }

    /// `label: LABELSYM` -> the existing label symbol id.
    pub fn label_sym(&mut self, sym: SymbolId) -> u32 {
        sym
    }
    /// `label: '<' STRING '>'` -> `pcode.defineLabel`: create a LabelTableSymbol.
    pub fn pcode_define_label(&mut self, name: &[u8]) -> u32 {
        // C++ `PcodeCompile::defineLabel`: `new LabelSymbol(name, local_labelcount++)`.
        let count = self.pcode.local_labelcount;
        self.pcode.local_labelcount = count.wrapping_add(1);
        let sym = SleighSymbol::new(name, SymbolKind::Label(LabelTableSymbol::new(count)));
        self.add_sleigh_symbol(sym)
    }

    /// `statement: label` -> `pcode.placeLabel`: build the LABELBUILD op.
    pub fn pcode_place_label(&mut self, label: SymbolId) -> u32 {
        // C++ `PcodeCompile::placeLabel(LabelSymbol *)`.
        let placed = self
            .base
            .symtab()
            .find_symbol_by_id(label)
            .and_then(|s| s.as_label())
            .map(|l| l.is_placed())
            .unwrap_or(false);
        if placed {
            let loc = self.symbol_location(label);
            let nm = self.symbol_name(label);
            self.report_error_loc(
                loc.as_ref(),
                &format!("Label '{}' is placed more than once", String::from_utf8_lossy(&nm)),
            );
        }
        let idx = self
            .base
            .symtab()
            .find_symbol_by_id(label)
            .and_then(|s| s.as_label())
            .map(|l| l.get_index())
            .unwrap_or(0);
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(label) {
            if let Some(l) = s.as_label_mut() {
                l.set_placed();
            }
        }
        let cs = self.get_constant_space_rc();
        let mut op = OpTpl::new(LABELBUILD);
        op.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, u64::from(idx)),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        self.alloc_oplist(vec![op])
    }

    /// `MACROSYM '(' paramlist ')'` statement -> macro expansion placeholder.
    pub fn create_macro_use_stmt(&mut self, sym: SymbolId, param: Vec<u32>) -> u32 {
        let pv: Vec<ExprTree> = param.into_iter().map(|p| self.take_expr(p)).collect();
        let ops = self.create_macro_use(sym, pv);
        self.alloc_oplist(ops)
    }

    // ----- small symbol helpers -----

    fn symbol_varnode(&mut self, sym: SymbolId) -> VarnodeTpl {
        match self.base.symtab().get_varnode(sym) {
            Ok(v) => v,
            Err(err) => {
                self.report_current_error(&err.explain());
                VarnodeTpl::default()
            }
        }
    }
    fn symbol_varnode_size(&self, sym: SymbolId) -> i32 {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .map(|s| match s.kind() {
                SymbolKind::Varnode(v) => v.get_size(),
                _ => 0,
            })
            .unwrap_or(0)
    }
    fn space_symbol_space(&mut self, sym: SymbolId) -> Rc<AddrSpace> {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| match s.kind() {
                SymbolKind::Space(sp) => Some(Rc::clone(sp.get_space())),
                _ => None,
            })
            .unwrap_or_else(|| self.get_default_code_space_rc())
    }
    fn userop_symbol_clone(
        &mut self,
        sym: SymbolId,
    ) -> Option<kuna_sleigh::slghsymbol::UserOpSymbol> {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| match s.kind() {
                SymbolKind::UserOp(u) => Some(u.clone()),
                _ => None,
            })
    }
    fn symbol_location(&self, sym: SymbolId) -> Option<Location> {
        self.symbol_loc.get(&sym).cloned()
    }
    fn report_current_error(&mut self, msg: &str) {
        let loc = self.current_location();
        self.report_error_loc(Some(&loc), msg);
    }

    // -----------------------------------------------------------------------
    // operand / equation / context actions
    // -----------------------------------------------------------------------

    /// C++ `SleighCompile::constrainOperand(OperandSymbol *sym,PatternExpression *patexp)`
    /// (slgh_compile.cc:3000).
    pub fn constrain_operand(&mut self, sym: SymbolId, patexp: u32) -> Option<u32> {
        // If the operand is already defined as a family symbol, this must be a
        // constraint (an EqualEquation on the family's pattern value).
        let defining = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| match s.kind() {
                SymbolKind::Operand(op) => op.get_defining_symbol(),
                _ => None,
            });
        let fam_patval = defining.and_then(|d| self.family_patval(d));
        match fam_patval {
            Some(lhs) => {
                let rhs = self.patexp_get(patexp);
                Some(self.arena.alloc(PatternEquation::Equal { lhs, rhs }))
            }
            None => {
                // Operand currently undefined — cannot constrain.
                None
            }
        }
    }

    /// C++ `SleighCompile::defineOperand(OperandSymbol *sym,PatternExpression *patexp)`
    /// (slgh_compile.cc:3022).
    pub fn define_operand(&mut self, sym: SymbolId, patexp: u32) {
        let pe = self.patexp_get(patexp);
        let res = self
            .base
            .symtab_mut()
            .find_symbol_by_id_mut(sym)
            .and_then(|s| match s.kind_mut() {
                SymbolKind::Operand(op) => Some(op.define_operand_expression(pe)),
                _ => None,
            });
        match res {
            Some(Ok(())) => {
                // Offset is irrelevant: no pattern directly on this operand.
                if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
                    if let SymbolKind::Operand(op) = s.kind_mut() {
                        op.set_offset_irrelevant();
                    }
                }
            }
            Some(Err(err)) => self.report_current_error(&err.explain()),
            None => {}
        }
    }

    /// C++ `SleighCompile::defineInvisibleOperand(TripleSymbol *sym)`
    /// (slgh_compile.cc:3044).
    pub fn define_invisible_operand(&mut self, sym: SymbolId) -> Option<u32> {
        let curct = self.curct?;
        let (table_id, ct_idx) = self.ctmap[curct as usize];
        let index = self.constructor_mut(table_id, ct_idx).get_num_operands();
        let name = self.symbol_name(sym);
        // new OperandSymbol(name, index, curct)
        let opsym = SleighSymbol::new_operand(
            &name,
            index,
            kuna_sleigh::slghsymbol::ConstructorRef {
                table_id,
                ct_id: ct_idx,
            },
        );
        let opid = self.add_sleigh_symbol(opsym);
        self.constructor_mut(table_id, ct_idx).add_invisible_operand(opid);
        let res = self.arena.alloc(PatternEquation::Operand { index });
        // Define the operand from the triple symbol.
        let tp = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .map(|s| s.get_type());
        let defres = match tp {
            Some(SymbolType::Value) | Some(SymbolType::Context) => {
                let pe = self
                    .base
                    .symtab()
                    .find_symbol_by_id(sym)
                    .and_then(|s| s.get_pattern_expression().ok().flatten());
                pe.map(|pe| {
                    self.base
                        .symtab_mut()
                        .find_symbol_by_id_mut(opid)
                        .and_then(|s| match s.kind_mut() {
                            SymbolKind::Operand(op) => Some(op.define_operand_expression(pe)),
                            _ => None,
                        })
                })
            }
            _ => Some(
                self.base
                    .symtab_mut()
                    .find_symbol_by_id_mut(opid)
                    .and_then(|s| match s.kind_mut() {
                        SymbolKind::Operand(op) => Some(op.define_operand_symbol(sym)),
                        _ => None,
                    }),
            ),
        };
        if let Some(Some(Err(err))) = defres {
            self.report_current_error(&err.explain());
        }
        Some(res)
    }

    /// C++ `SleighCompile::selfDefine(OperandSymbol *sym)` (slgh_compile.cc:3072).
    pub fn self_define(&mut self, sym: SymbolId) {
        let name = self.symbol_name(sym);
        // glob = symtab.findSymbol(name, 1)  (skip the current scope)
        let glob = self.base.symtab().find_symbol_skip(&name, 1).map(|s| s.get_id());
        let glob = match glob {
            Some(g) => g,
            None => {
                self.report_current_error(&format!(
                    "No matching global symbol '{}'",
                    String::from_utf8_lossy(&name)
                ));
                return;
            }
        };
        let tp = self.base.symtab().find_symbol_by_id(glob).map(|s| s.get_type());
        let defres = match tp {
            Some(SymbolType::Value) | Some(SymbolType::Context) => {
                let pe = self
                    .base
                    .symtab()
                    .find_symbol_by_id(glob)
                    .and_then(|s| s.get_pattern_expression().ok().flatten());
                pe.map(|pe| {
                    self.base
                        .symtab_mut()
                        .find_symbol_by_id_mut(sym)
                        .and_then(|s| match s.kind_mut() {
                            SymbolKind::Operand(op) => Some(op.define_operand_expression(pe)),
                            _ => None,
                        })
                })
            }
            _ => Some(
                self.base
                    .symtab_mut()
                    .find_symbol_by_id_mut(sym)
                    .and_then(|s| match s.kind_mut() {
                        SymbolKind::Operand(op) => Some(op.define_operand_symbol(glob)),
                        _ => None,
                    }),
            ),
        };
        if let Some(Some(Err(err))) = defres {
            self.report_current_error(&err.explain());
        }
    }

    /// C++ `SleighCompile::contextMod(...)` (slgh_compile.cc:3137): a temporary
    /// context change.  Returns false if the value uses inst_next/inst_next2.
    pub fn context_mod(&mut self, vec: &mut Vec<u32>, sym: SymbolId, pe: u32) -> bool {
        let pexpr = self.patexp_get(pe);
        // The value expression must not use inst_next / inst_next2.
        if pattern_expression_uses_end_or_next2(&pexpr) {
            return false;
        }
        let (startbit, endbit) = self.context_field_bits(sym);
        let cop = match ContextOp::new(startbit, endbit, pexpr) {
            Ok(c) => c,
            Err(err) => {
                self.report_current_error(&err.explain());
                return true;
            }
        };
        let id = self.alloc_context_change(ContextChange::Op(cop));
        vec.push(id);
        true
    }

    /// C++ `SleighCompile::contextSet(...)` (slgh_compile.cc:3164): a permanent
    /// context commit.
    pub fn context_set(&mut self, vec: &mut Vec<u32>, sym: SymbolId, cvar: SymbolId) {
        let (startbit, endbit) = self.context_field_bits(cvar);
        let flow = self
            .base
            .symtab()
            .find_symbol_by_id(cvar)
            .and_then(|s| match s.kind() {
                SymbolKind::Context(c) => Some(c.get_flow()),
                _ => None,
            })
            .unwrap_or(true);
        let cc = match ContextCommit::new(sym, startbit, endbit, flow) {
            Ok(c) => c,
            Err(err) => {
                self.report_current_error(&err.explain());
                return;
            }
        };
        let id = self.alloc_context_change(ContextChange::Commit(cc));
        vec.push(id);
    }

    fn context_field_bits(&self, sym: SymbolId) -> (i32, i32) {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.get_pattern_value())
            .and_then(|pv| match pv {
                PatternValue::ContextField(cf) => Some((cf.get_start_bit(), cf.get_end_bit())),
                _ => None,
            })
            .unwrap_or((0, 0))
    }

    /// Driver-owned ContextChange arena (the `context_mod`/`context_set` vec
    /// threads `u32` ids the driver owns).
    fn alloc_context_change(&mut self, cc: ContextChange) -> u32 {
        let id = self.contextchange_arena.len() as u32;
        self.contextchange_arena.push(Some(cc));
        id
    }
    fn take_context_change(&mut self, id: u32) -> ContextChange {
        self.contextchange_arena[id as usize]
            .take()
            .expect("context change consumed")
    }

    // -----------------------------------------------------------------------
    // macros
    // -----------------------------------------------------------------------

    /// C++ `SleighCompile::createMacro(string *name,vector<string> *params)`
    /// (slgh_compile.cc:3180).
    pub fn create_macro(&mut self, name: &[u8], params: Vec<Vec<u8>>) -> SymbolId {
        self.curct = None; // Not currently defining a Constructor
        let macroindex = self.macro_bodies.len() as i32;
        let macsym = SleighSymbol::new(name, SymbolKind::Macro(MacroSymbol::new(macroindex)));
        let macid = self.add_sleigh_symbol(macsym);
        self.curmacro = Some(macid);
        self.base.symtab_mut().add_scope(); // New scope for the macro body
        self.pcode.local_labelcount = 0; // Macros have their own labels
        for (i, p) in params.iter().enumerate() {
            let oper = SleighSymbol::new_operand(
                p,
                i as i32,
                kuna_sleigh::slghsymbol::ConstructorRef {
                    table_id: u32::MAX,
                    ct_id: 0,
                },
            );
            let opid = self.add_sleigh_symbol(oper);
            if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(macid) {
                if let Some(m) = s.as_macro_mut() {
                    m.add_operand(opid);
                }
            }
        }
        macid
    }

    /// C++ `SleighCompile::buildMacro(MacroSymbol *sym,ConstructTpl *rtl)`
    /// (slgh_compile.cc:3737).
    pub fn build_macro(&mut self, sym: SymbolId, rtl: u32) {
        let scope = self.base.symtab().get_current_scope().unwrap_or(0);
        let errstring = self.base.symtab().check_symbols(scope);
        if !errstring.is_empty() {
            let name = self.symbol_name(sym);
            self.report_current_error(&format!(
                "In definition of macro '{}': {}",
                String::from_utf8_lossy(&name),
                errstring
            ));
            return;
        }
        let mut body = self.take_section(rtl);
        if !self.expand_macros(&mut body) {
            let name = self.symbol_name(sym);
            self.report_current_error(&format!(
                "Could not expand submacro in definition of macro '{}'",
                String::from_utf8_lossy(&name)
            ));
            return;
        }
        let _ = kuna_sleigh::pcodecompile::propagate_size(&mut body); // as much as possible
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let Some(m) = s.as_macro_mut() {
                m.set_construct(body.clone());
            }
        }
        self.base.symtab_mut().pop_scope(); // Pop local macro variables
        self.macro_bodies.push(Some(body));
    }

    /// C++ `SleighCompile::createMacroUse(MacroSymbol *sym,vector<ExprTree *> *param)`
    /// (slgh_compile.cc:3235).
    pub fn create_macro_use(&mut self, sym: SymbolId, param: Vec<ExprTree>) -> Vec<OpTpl> {
        let numops = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_macro())
            .map(|m| m.get_num_operands())
            .unwrap_or(0);
        let macroindex = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_macro())
            .map(|m| m.get_index())
            .unwrap_or(0);
        if numops != param.len() {
            let too_many = param.len() > numops;
            let name = self.symbol_name(sym);
            self.report_current_error(&format!(
                "Invocation of macro '{}' passes too {} parameters",
                String::from_utf8_lossy(&name),
                if too_many { "many" } else { "few" }
            ));
            return Vec::new();
        }
        self.compare_macro_params(sym, &param);
        let mut op = OpTpl::new(MACROBUILD);
        let cs = self.get_constant_space_rc();
        let idvn = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, macroindex as u64),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        op.add_input(idvn);
        ExprTree::append_params(op, param)
    }

    /// C++ `SleighCompile::compareMacroParams(MacroSymbol *sym,...)`
    /// (slgh_compile.cc:3204): pass `code_address` through to the parent operand.
    fn compare_macro_params(&mut self, sym: SymbolId, param: &[ExprTree]) {
        for (i, p) in param.iter().enumerate() {
            let outvn = match p.get_out() {
                Some(v) => v,
                None => continue,
            };
            if outvn.get_offset().get_type() != ConstType::Handle {
                continue;
            }
            let hand = outvn.get_offset().get_handle_index();
            // macroop = sym->getOperand(i)
            let macroop = self
                .base
                .symtab()
                .find_symbol_by_id(sym)
                .and_then(|s| s.as_macro())
                .map(|m| m.get_operand(i));
            let is_code = macroop
                .and_then(|mid| self.base.symtab().find_symbol_by_id(mid))
                .and_then(|s| match s.kind() {
                    SymbolKind::Operand(op) => Some(op.is_code_address()),
                    _ => None,
                })
                .unwrap_or(false);
            if !is_code {
                continue;
            }
            // parentop = (curct ? curct : curmacro)->getOperand(hand)
            let parentop = if let Some(curct) = self.curct {
                let (table_id, ct_idx) = self.ctmap[curct as usize];
                self.base
                    .symtab()
                    .get_constructor(kuna_sleigh::slghsymbol::ConstructorRef {
                        table_id,
                        ct_id: ct_idx,
                    })
                    .ok()
                    .and_then(|ct| ct.get_operand(hand).ok())
            } else {
                self.curmacro
                    .and_then(|mid| self.base.symtab().find_symbol_by_id(mid))
                    .and_then(|s| s.as_macro())
                    .map(|m| m.get_operand(hand as usize))
            };
            if let Some(pid) = parentop {
                if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(pid) {
                    if let SymbolKind::Operand(op) = s.kind_mut() {
                        op.set_code_address();
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // crossbuild
    // -----------------------------------------------------------------------

    /// C++ `SleighCompile::createCrossBuild(VarnodeTpl *addr,SectionSymbol *sym)`
    /// (slgh_compile.cc:3330).
    pub fn create_cross_build(&mut self, addr: u32, sym: SymbolId) -> u32 {
        self.unique_allocatemask = 1;
        let templateid = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_section())
            .map(|sec| sec.get_template_id())
            .unwrap_or(0);
        let addrvn = self.take_vntpl(addr);
        let cs = self.get_constant_space_rc();
        let sectionid = VarnodeTpl::new(
            ConstTpl::new_space(cs),
            ConstTpl::new_real(ConstType::Real, templateid as u64),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let mut op = OpTpl::new(CROSSBUILD);
        op.add_input(addrvn);
        op.add_input(sectionid);
        if let Some(s) = self.base.symtab_mut().find_symbol_by_id_mut(sym) {
            if let Some(sec) = s.as_section_mut() {
                sec.increment_ref_count();
            }
        }
        self.alloc_oplist(vec![op])
    }

    // -----------------------------------------------------------------------
    // per-constructor section finalize + the process()-time crossbuild shift
    // -----------------------------------------------------------------------

    /// C++ `SleighCompile::buildConstructor(...)` (slgh_compile.cc:3698).
    pub fn build_constructor_ws4c(
        &mut self,
        big: u32,
        pateq: Option<u32>,
        contvec: Option<Vec<u32>>,
        vec: Option<u32>,
    ) {
        let (table_id, ct_idx) = self.ctmap[big as usize];
        let mut noerrors = true;
        if let Some(secvec_id) = vec {
            noerrors = self.finalize_sections(big, secvec_id);
            if noerrors {
                // Attach sections to the Constructor.
                let main = self.secvec_mut(secvec_id).release_main_section();
                if let Some(secid) = main {
                    let ct = self.take_section(secid);
                    let handle = self.base.add_template(ct);
                    self.constructor_mut(table_id, ct_idx).set_main_section(handle);
                }
                let maxid = self.secvec_ref(secvec_id).get_max_id();
                for i in 0..maxid {
                    let named = self.secvec_mut(secvec_id).release_named_section(i);
                    if let Some(secid) = named {
                        let ct = self.take_section(secid);
                        let handle = self.base.add_template(ct);
                        self.constructor_mut(table_id, ct_idx).set_named_section(handle, i);
                    }
                }
            }
            // Drop the section vector (C++ delete vec).
            self.drop_secvec(secvec_id);
        }
        if noerrors {
            let pateq = self.collect_and_prepend_pattern(pateq);
            if let Some(eq) = pateq {
                self.constructor_mut(table_id, ct_idx).add_equation(eq);
            } else {
                let eps = self.arena.alloc(PatternEquation::Unconstrained {
                    patex: PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(
                        0,
                    ))),
                });
                self.constructor_mut(table_id, ct_idx).add_equation(eps);
            }
            self.constructor_mut(table_id, ct_idx).remove_trailing_space();
            // Context changes (prepended from the with-stack, then this ctor's).
            let mut contvec = self.collect_and_prepend_context(contvec);
            if !contvec.is_empty() {
                let changes: Vec<ContextChange> =
                    contvec.drain(..).map(|id| self.take_context_change(id)).collect();
                self.constructor_mut(table_id, ct_idx).add_context(changes);
            }
        }
        self.base.symtab_mut().pop_scope(); // In all cases pop scope
    }

    /// C++ `WithBlock::collectAndPrependContext` (slgh_compile.hh): prepend each
    /// with-block's context changes (outermost first) to this ctor's.
    fn collect_and_prepend_context(&mut self, contvec: Option<Vec<u32>>) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        // C++ iterates withstack front-to-back, prepending each block's context
        // (the stack is pushed inner-last, so iterate from the bottom).
        for block in &self.withstack {
            for cc in &block.contvec {
                let id = self.contextchange_arena.len() as u32;
                self.contextchange_arena.push(Some(cc.clone()));
                res.push(id);
            }
        }
        if let Some(v) = contvec {
            res.extend(v);
        }
        res
    }

    /// C++ `SleighCompile::finalizeSections(Constructor *big,SectionVector *vec)`
    /// (slgh_compile.cc:3436).
    fn finalize_sections(&mut self, big: u32, secvec_id: u32) -> bool {
        let (table_id, ct_idx) = self.ctmap[big as usize];
        let parent = self.constructor_parent(table_id, ct_idx);
        let root = self.base.get_root();
        let mut errors: Vec<String> = Vec::new();

        let mut cur = self.secvec_ref(secvec_id).get_main_pair();
        let mut i: i32 = -1;
        let mut sectionstring = String::from("   Main section: ");
        let max = self.secvec_ref(secvec_id).get_max_id();
        loop {
            let errstring = self
                .base
                .symtab()
                .check_symbols(cur.scope.unwrap_or(0));
            if !errstring.is_empty() {
                errors.push(format!("{sectionstring}{errstring}"));
            } else if let Some(secid) = cur.section {
                let mut body = self.take_section(secid);
                if !self.expand_macros(&mut body) {
                    errors.push(format!("{sectionstring}Could not expand macros"));
                }
                let operand_ids = self.constructor_operands(table_id, ct_idx);
                let mut check = self.base.symtab().mark_subtable_operands(&operand_ids);
                let cs = self.get_constant_space_rc();
                let res = body.fillin_build(&mut check, &cs);
                if res == 1 {
                    errors.push(format!("{sectionstring}Duplicate BUILD statements"));
                }
                if res == 2 {
                    errors.push(format!("{sectionstring}Unnecessary BUILD statements"));
                }
                if !kuna_sleigh::pcodecompile::propagate_size(&mut body).unwrap_or(false) {
                    errors.push(format!(
                        "{sectionstring}Could not resolve at least 1 variable size"
                    ));
                }
                // put the (mutated) section back
                self.put_section(secid, body);
            }
            if i < 0 {
                // Main-section-only potential errors.
                let has_result = cur
                    .section
                    .map(|sid| {
                        self.section_ref(sid)
                            .map(|s| s.get_result().is_some())
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);
                if has_result {
                    if parent == root {
                        errors.push("   Cannot have export statement in root constructor".into());
                    } else if !self.force_export_size(cur.section.unwrap()) {
                        errors.push("   Size of export is unknown".into());
                    }
                }
            }
            // Delay slot handling.
            if let Some(sid) = cur.section {
                let delay = self
                    .section_ref(sid)
                    .map(|s| s.delay_slot())
                    .unwrap_or(0);
                if delay != 0 {
                    if root != parent {
                        let loc = self.ctor_loc.get(&big).cloned();
                        self.report_warning_loc(
                            loc.as_ref(),
                            "Delay slot used in non-root constructor",
                        );
                    }
                    if delay > self.maxdelayslotbytes {
                        self.maxdelayslotbytes = delay;
                    }
                }
            }
            // Advance to the next non-null named section.
            loop {
                i += 1;
                if i >= max {
                    break;
                }
                cur = self.secvec_ref(secvec_id).get_named_pair(i);
                if cur.section.is_some() {
                    break;
                }
            }
            if i >= max {
                break;
            }
            let secsym = self.sections[i as usize];
            let nm = self.symbol_name(secsym);
            sectionstring = format!("   {} section: ", String::from_utf8_lossy(&nm));
        }
        if !errors.is_empty() {
            let loc = self.ctor_loc.get(&big).cloned();
            let mut info = String::from("in ");
            self.constructor_print_info(table_id, ct_idx, &mut info);
            self.report_error_loc(loc.as_ref(), &info);
            for e in &errors {
                self.report_error_loc(loc.as_ref(), e);
            }
            return false;
        }
        // Update the base maxdelayslotbytes (C++ stores it on SleighBase).
        self.base.set_max_delay_slot_bytes(self.maxdelayslotbytes);
        true
    }

    /// C++ `SleighCompile::forceExportSize(ConstructTpl *ct)` (slgh_compile.cc:3541).
    fn force_export_size(&mut self, secid: u32) -> bool {
        // Operate on the owned section in the arena.
        let mut body = self.take_section(secid);
        let ok = self.force_export_size_inner(&mut body);
        self.put_section(secid, body);
        ok
    }

    fn force_export_size_inner(&mut self, ct: &mut ConstructTpl) -> bool {
        let (ptr_uniq_zero, space_uniq_zero, offset) = match ct.get_result() {
            None => return true,
            Some(result) => (
                result.get_ptr_space().is_unique_space() && result.get_ptr_size().is_zero(),
                result.get_space().is_unique_space() && result.get_size().is_zero(),
                result.get_ptr_offset().clone(),
            ),
        };
        if ptr_uniq_zero {
            match find_size(&offset, ct) {
                Some(sz) => ct.get_result_mut().unwrap().set_ptr_size(sz),
                None => return false,
            }
        } else if space_uniq_zero {
            match find_size(&offset, ct) {
                Some(sz) => ct.get_result_mut().unwrap().set_size(sz),
                None => return false,
            }
        }
        true
    }

    /// C++ `SleighCompile::expandMacros(ConstructTpl *ctpl)` (slgh_compile.cc:3397).
    fn expand_macros(&mut self, ctpl: &mut ConstructTpl) -> bool {
        use kuna_sleigh::semantics::PcodeBuilder;
        let oldops = std::mem::take(ctpl.get_opvec_mut());
        let mut newvec: Vec<OpTpl> = Vec::new();
        for op in oldops {
            if op.get_opcode() == MACROBUILD {
                let index = op.get_in(0).get_offset().get_real() as usize;
                if index >= self.macro_bodies.len() {
                    *ctpl.get_opvec_mut() = newvec;
                    return false;
                }
                let macro_tpl = match &self.macro_bodies[index] {
                    Some(m) => m.clone(),
                    None => {
                        *ctpl.get_opvec_mut() = newvec;
                        return false;
                    }
                };
                let labelbase = ctpl.num_labels();
                let haserror = {
                    let mut builder = crate::pcodecompile_actions::MacroBuilder::new(
                        self,
                        &mut newvec,
                        labelbase,
                    );
                    if builder.set_macro_op(&op).is_err() {
                        true
                    } else {
                        let r = builder.build(Some(&macro_tpl), -1);
                        r.is_err() || builder.has_error()
                    }
                };
                ctpl.set_num_labels(ctpl.num_labels() + macro_tpl.num_labels());
                if haserror {
                    *ctpl.get_opvec_mut() = newvec;
                    return false;
                }
            } else {
                newvec.push(op);
            }
        }
        *ctpl.get_opvec_mut() = newvec;
        true
    }

    /// C++ `SleighCompile::checkUniqueAllocation` (slgh_compile.cc:3638).
    fn check_unique_allocation(&mut self) {
        if self.unique_allocatemask == 0 {
            return;
        }
        self.unique_allocatemask = 0xff; // 8 bits of free space
        // Gather every constructor's template handles (main + named).
        let mut handles: Vec<ConstructTplHandle> = Vec::new();
        let mut subtables: Vec<SymbolId> = Vec::new();
        if let Some(root) = self.base.get_root() {
            subtables.push(root);
        }
        subtables.extend(self.tables.iter().copied());
        for table_id in subtables {
            let numconst = self
                .base
                .symtab()
                .find_symbol_by_id(table_id)
                .and_then(|s| s.as_subtable())
                .map(|st| st.get_num_constructors())
                .unwrap_or(0);
            for j in 0..numconst {
                let (templ, named): (Option<ConstructTplHandle>, Vec<Option<ConstructTplHandle>>) =
                    {
                        let st = self
                            .base
                            .symtab()
                            .find_symbol_by_id(table_id)
                            .and_then(|s| s.as_subtable())
                            .expect("subtable");
                        let ct = st.get_constructor(j as u32).expect("constructor");
                        let named: Vec<Option<ConstructTplHandle>> = (0..ct.get_num_sections())
                            .map(|k| ct.get_named_templ(k))
                            .collect();
                        (ct.get_templ(), named)
                    };
                if let Some(h) = templ {
                    handles.push(h);
                }
                for n in named.into_iter().flatten() {
                    handles.push(n);
                }
            }
        }
        for h in handles {
            // Take the template out, shift, put it back (avoids aliasing).
            if let Some(tpl) = self.base.template_mut(h) {
                let mut owned = std::mem::take(tpl);
                shift_unique_construct(&mut owned);
                *self.base.template_mut(h).unwrap() = owned;
            }
        }
        let mut ubase = self.base.get_unique_base();
        ubase += 1 << UNIQUE_CROSSBUILD_POSITION;
        ubase <<= UNIQUE_CROSSBUILD_NUMBITS;
        self.base.set_unique_base(ubase);
    }

    // ---- small constructor helpers ----

    fn constructor_parent(&self, table_id: SymbolId, ct_idx: u32) -> Option<SymbolId> {
        self.base
            .symtab()
            .get_constructor(kuna_sleigh::slghsymbol::ConstructorRef {
                table_id,
                ct_id: ct_idx,
            })
            .ok()
            .and_then(|ct| ct.get_parent())
    }
    fn constructor_operands(&self, table_id: SymbolId, ct_idx: u32) -> Vec<SymbolId> {
        self.base
            .symtab()
            .get_constructor(kuna_sleigh::slghsymbol::ConstructorRef {
                table_id,
                ct_id: ct_idx,
            })
            .map(|ct| ct.get_operands().to_vec())
            .unwrap_or_default()
    }
    fn constructor_print_info(&self, table_id: SymbolId, ct_idx: u32, out: &mut String) {
        if let Ok(ct) = self
            .base
            .symtab()
            .get_constructor(kuna_sleigh::slghsymbol::ConstructorRef {
                table_id,
                ct_id: ct_idx,
            })
        {
            let _ = ct.print_info(out, self.base.symtab());
        }
    }
}

impl CompilerHost for SleighCompile {
    fn get_unique_addr(&mut self) -> u32 {
        SleighCompile::get_unique_addr(self)
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        self.unique_space
            .clone()
            .or_else(|| self.base.unique_space())
            .expect("unique space")
    }
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        self.constant_space
            .clone()
            .or_else(|| self.base.constant_space())
            .expect("constant space")
    }
    fn get_location(&self, symbol_name: &[u8]) -> Option<Location> {
        let id = self.base.symtab().find_symbol(symbol_name)?.get_id();
        self.symbol_loc.get(&id).cloned()
    }
    fn add_symbol(&mut self, sym: PcodeCompileSymbol) {
        // C++ `SleighCompile::addSymbol(SleighSymbol *)` (cc:2355): just
        // `symtab.addSymbol(sym)`.  `newOutput`/`newLocalDefinition` hand us a
        // VarnodeSymbol (the `Sleigh` variant).  The `Label` variant is never
        // produced here — the driver builds branch labels as `LabelTableSymbol`s
        // directly in `pcode_define_label` (see below), so `define_label`'s
        // `Rc<LabelSymbol>` path is unused by the compiler.
        match sym {
            PcodeCompileSymbol::Sleigh(boxed) => {
                self.add_sleigh_symbol(*boxed);
            }
            PcodeCompileSymbol::Label(_) => {
                // Not reached in the compiler path.
            }
        }
    }
    fn report_error(&mut self, loc: Option<&Location>, msg: &str) {
        self.report_error_loc(loc, msg);
    }
    fn report_warning(&mut self, loc: Option<&Location>, msg: &str) {
        self.report_warning_loc(loc, msg);
    }
}

// ===========================================================================
// PcodeCompile impl (WS4c): the driver IS the p-code compiler.
//
// In C++ `SleighPcode : public PcodeCompile` holds a back-pointer to the
// `SleighCompile`.  The Rust port collapses that into the driver: `SleighCompile`
// implements `PcodeCompile` directly, supplying the abstract hooks from its own
// state (the unique base / label count / enforce-local flag live in
// `self.pcode`; the spaces from the base; `addSymbol`/`getLocation`/reporting
// from the driver).  This gives the driver all the rich `create_op`/`create_store`/
// `assign_bit_range`/... machinery (pcodecompile.cc) for the section actions.
// ===========================================================================

impl PcodeCompile for SleighCompile {
    fn get_default_space(&self) -> Option<Rc<AddrSpace>> {
        self.default_space
            .clone()
            .or_else(|| self.base.default_code_space())
    }
    fn set_default_space(&mut self, spc: Rc<AddrSpace>) {
        self.default_space = Some(spc);
    }
    fn get_constant_space(&self) -> Option<Rc<AddrSpace>> {
        self.constant_space
            .clone()
            .or_else(|| self.base.constant_space())
    }
    fn set_constant_space(&mut self, spc: Rc<AddrSpace>) {
        self.constant_space = Some(spc);
    }
    fn get_unique_space(&self) -> Option<Rc<AddrSpace>> {
        self.unique_space.clone().or_else(|| self.base.unique_space())
    }
    fn set_unique_space(&mut self, spc: Rc<AddrSpace>) {
        self.unique_space = Some(spc);
    }
    fn is_enforce_local_key(&self) -> bool {
        self.pcode.enforce_local_key
    }
    fn set_enforce_local_key(&mut self, val: bool) {
        self.pcode.enforce_local_key = val;
    }
    fn local_label_count(&self) -> u32 {
        self.pcode.local_labelcount
    }
    fn set_local_label_count(&mut self, val: u32) {
        self.pcode.local_labelcount = val;
    }
    fn allocate_temp(&mut self) -> u32 {
        self.get_unique_addr()
    }
    fn add_symbol(&mut self, sym: PcodeCompileSymbol) {
        <Self as CompilerHost>::add_symbol(self, sym);
    }
    fn get_location(&self, symbol_name: &[u8]) -> Option<Location> {
        <Self as CompilerHost>::get_location(self, symbol_name)
    }
    fn report_error(&mut self, loc: Option<&Location>, msg: &str) {
        self.report_error_loc(loc, msg);
    }
    fn report_warning(&mut self, loc: Option<&Location>, msg: &str) {
        self.report_warning_loc(loc, msg);
    }
}

// ===========================================================================
// ScannerHost + ParserActions impls (WS2 driver seam)
// ===========================================================================

impl ScannerHost for SleighCompile {
    fn next_line(&mut self) {
        SleighCompile::next_line(self)
    }
    fn calc_context_layout(&mut self) {
        SleighCompile::calc_context_layout(self)
    }
    fn read_include(&mut self, fname: &[u8]) -> Option<Vec<u8>> {
        self.parse_from_new_file(fname);
        let path = self.grab_current_file_path();
        let contents = std::fs::read(String::from_utf8_lossy(&path).into_owned()).ok();
        if contents.is_none() {
            self.parse_file_finished();
        }
        contents
    }
    fn parse_file_finished(&mut self) {
        SleighCompile::parse_file_finished(self)
    }
    fn parse_preproc_macro(&mut self) {
        SleighCompile::parse_preproc_macro(self)
    }
    fn get_preproc_value(&self, name: &[u8]) -> Option<Vec<u8>> {
        SleighCompile::get_preproc_value(self, name)
    }
    fn set_preproc_value(&mut self, name: &[u8], value: &[u8]) {
        SleighCompile::set_preproc_value(self, name, value)
    }
    fn undefine_preproc_value(&mut self, name: &[u8]) -> bool {
        SleighCompile::undefine_preproc_value(self, name)
    }
    fn find_symbol_kind(&self, name: &[u8]) -> Option<SymbolTokenKind> {
        let sym = self.base.symtab().find_symbol(name)?;
        Some(match sym.get_type() {
            SymbolType::Space => SymbolTokenKind::Space,
            SymbolType::Token => SymbolTokenKind::Token,
            SymbolType::UserOp => SymbolTokenKind::Userop,
            SymbolType::Value => SymbolTokenKind::Value,
            SymbolType::ValueMap => SymbolTokenKind::ValueMap,
            SymbolType::Name => SymbolTokenKind::Name,
            SymbolType::Varnode => SymbolTokenKind::Varnode,
            SymbolType::VarnodeList => SymbolTokenKind::VarnodeList,
            SymbolType::Operand => SymbolTokenKind::Operand,
            SymbolType::Start | SymbolType::End | SymbolType::Next2 => SymbolTokenKind::Jump,
            SymbolType::FlowDest | SymbolType::FlowRef => SymbolTokenKind::Jump,
            SymbolType::Subtable => SymbolTokenKind::Subtable,
            SymbolType::Macro => SymbolTokenKind::Macro,
            SymbolType::Section => SymbolTokenKind::Section,
            SymbolType::Bitrange => SymbolTokenKind::Bitrange,
            SymbolType::Context => SymbolTokenKind::Context,
            SymbolType::Epsilon => SymbolTokenKind::Spec,
            SymbolType::Label => SymbolTokenKind::Label,
            SymbolType::Dummy => return None,
        })
    }
}

/// The unported p-code section path; reaching one means a spec with semantic RTL
/// hit the WS4b landed-subset boundary.
fn pcode_unported(name: &str) -> ! {
    panic!(
        "WS4b landed subset: p-code action `{name}` requires the unported \
         ConstructTpl/section path (slgh_compile.cc / pcodecompile.cc)"
    )
}

impl ParserActions for SleighCompile {
    fn set_endian(&mut self, big: i32) {
        SleighCompile::set_endian(self, big)
    }
    fn set_alignment(&mut self, val: i32) {
        SleighCompile::set_alignment(self, val)
    }
    fn define_token(&mut self, name: &[u8], sz: u64, endian: i32) -> SymbolId {
        SleighCompile::define_token(self, name, sz, endian)
    }
    fn add_token_field(&mut self, sym: SymbolId, qual: FieldQual) {
        SleighCompile::add_token_field(self, sym, FieldQuality::from_qual(qual))
    }
    fn context_prop_begin(&mut self, varsym: SymbolId) -> SymbolId {
        varsym
    }
    fn add_context_field(&mut self, sym: SymbolId, qual: FieldQual) -> bool {
        SleighCompile::add_context_field(self, sym, FieldQuality::from_qual(qual))
    }
    fn new_space(&mut self, qual: SpaceQual) {
        let kind = if qual.is_register {
            SpaceType::Register
        } else {
            SpaceType::Ram
        };
        SleighCompile::new_space(
            self,
            SpaceQuality {
                name: qual.name,
                kind,
                size: qual.size as u32,
                wordsize: qual.wordsize as u32,
                isdefault: qual.isdefault,
            },
        )
    }
    fn define_varnodes(&mut self, spacesym: SymbolId, off: u64, size: u64, names: Vec<Vec<u8>>) {
        SleighCompile::define_varnodes(self, spacesym, off, size, names)
    }
    fn define_bitrange(&mut self, name: &[u8], sym: SymbolId, bitoffset: u32, numb: u32) {
        SleighCompile::define_bitrange(self, name, sym, bitoffset, numb)
    }
    fn add_user_op(&mut self, names: Vec<Vec<u8>>) {
        SleighCompile::add_user_op(self, names)
    }
    fn attach_values(&mut self, symlist: Vec<SymbolId>, numlist: Vec<i64>) {
        SleighCompile::attach_values(self, symlist, numlist)
    }
    fn attach_names(&mut self, symlist: Vec<SymbolId>, names: Vec<Vec<u8>>) {
        SleighCompile::attach_names(self, symlist, names)
    }
    fn attach_varnodes(&mut self, symlist: Vec<SymbolId>, varlist: Vec<SymbolId>) {
        SleighCompile::attach_varnodes(self, symlist, varlist)
    }
    fn build_macro(&mut self, sym: SymbolId, rtl: u32) {
        SleighCompile::build_macro(self, sym, rtl)
    }
    fn create_macro(&mut self, name: &[u8], params: Vec<Vec<u8>>) -> SymbolId {
        SleighCompile::create_macro(self, name, params)
    }
    fn push_with(&mut self, ss: Option<SymbolId>, pateq: Option<u32>, contvec: Option<Vec<u32>>) {
        SleighCompile::push_with(self, ss, pateq, contvec)
    }
    fn pop_with(&mut self) {
        SleighCompile::pop_with(self)
    }
    fn new_table(&mut self, nm: &[u8]) -> SymbolId {
        SleighCompile::new_table(self, nm)
    }
    fn create_constructor(&mut self, sym: Option<SymbolId>) -> u32 {
        SleighCompile::create_constructor(self, sym)
    }
    fn reset_constructors(&mut self) {
        SleighCompile::reset_constructors(self)
    }
    fn add_syntax(&mut self, ct: u32, syntax: &[u8]) {
        SleighCompile::add_syntax(self, ct, syntax)
    }
    fn new_operand(&mut self, ct: u32, nm: &[u8]) {
        SleighCompile::new_operand(self, ct, nm)
    }
    fn is_in_root(&self, ct: u32) -> bool {
        let (table_id, _) = self.ctmap[ct as usize];
        self.base.get_root() == Some(table_id)
    }
    fn build_constructor(&mut self, big: u32, pateq: Option<u32>, contvec: Option<Vec<u32>>, vec: u32) {
        let v = if vec == u32::MAX { None } else { Some(vec) };
        SleighCompile::build_constructor_ws4c(self, big, pateq, contvec, v)
    }
    fn standalone_section(&mut self, main: u32) -> u32 {
        SleighCompile::standalone_section(self, main)
    }
    fn final_named_section(&mut self, vec: u32, section: u32) -> u32 {
        SleighCompile::final_named_section(self, vec, section)
    }
    fn first_named_section(&mut self, main: u32, sym: SymbolId) -> u32 {
        SleighCompile::first_named_section(self, main, sym)
    }
    fn next_named_section(&mut self, vec: u32, section: u32, sym: SymbolId) -> u32 {
        SleighCompile::next_named_section(self, vec, section, sym)
    }
    fn new_section_symbol(&mut self, nm: &[u8]) -> SymbolId {
        SleighCompile::new_section_symbol(self, nm)
    }
    fn enter_section(&mut self) -> u32 {
        SleighCompile::enter_section(self)
    }
    fn finish_main_rtl(&mut self, rtlmid: u32) -> u32 {
        SleighCompile::finish_main_rtl(self, rtlmid)
    }
    fn set_result_varnode(&mut self, ct: u32, vn: u32) -> u32 {
        SleighCompile::set_result_varnode(self, ct, vn)
    }
    fn set_result_star_varnode(&mut self, ct: u32, star: u32, vn: u32) -> u32 {
        SleighCompile::set_result_star_varnode(self, ct, star, vn)
    }
    fn rtl_add_oplist(&mut self, sec: u32, stmt: u32) -> bool {
        SleighCompile::rtl_add_oplist(self, sec, stmt)
    }
    fn peq_and(&mut self, l: u32, r: u32) -> u32 {
        self.arena.alloc(PatternEquation::And { left: l, right: r })
    }
    fn peq_or(&mut self, l: u32, r: u32) -> u32 {
        self.arena.alloc(PatternEquation::Or { left: l, right: r })
    }
    fn peq_cat(&mut self, l: u32, r: u32) -> u32 {
        self.arena.alloc(PatternEquation::Cat { left: l, right: r })
    }
    fn peq_left_ellipsis(&mut self, e: u32) -> u32 {
        self.arena.alloc(PatternEquation::LeftEllipsis { eq: e })
    }
    fn peq_right_ellipsis(&mut self, e: u32) -> u32 {
        self.arena.alloc(PatternEquation::RightEllipsis { eq: e })
    }
    fn peq_equal(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::Equal)
    }
    fn peq_notequal(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::NotEqual)
    }
    fn peq_less(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::Less)
    }
    fn peq_lessequal(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::LessEqual)
    }
    fn peq_greater(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::Greater)
    }
    fn peq_greaterequal(&mut self, fam: SymbolId, rhs: u32) -> u32 {
        self.build_cmp_equation(fam, rhs, CmpKind::GreaterEqual)
    }
    fn constrain_operand(&mut self, sym: SymbolId, patexp: u32) -> Option<u32> {
        SleighCompile::constrain_operand(self, sym, patexp)
    }
    fn peq_operand_equation(&mut self, sym: SymbolId) -> u32 {
        let index = self
            .base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Operand(o) => Some(o.get_index()),
                _ => None,
            })
            .unwrap_or(0);
        self.arena.alloc(PatternEquation::Operand { index })
    }
    fn self_define(&mut self, sym: SymbolId) {
        SleighCompile::self_define(self, sym)
    }
    fn peq_unconstrained(&mut self, spec: SymbolId) -> u32 {
        let patex = self
            .base
            .symtab()
            .find_symbol_by_id(spec)
            .and_then(|s| s.get_pattern_expression().ok().flatten())
            .unwrap_or(PatternExpression::Value(PatternValue::ConstantValue(
                ConstantValue::new(0),
            )));
        self.arena.alloc(PatternEquation::Unconstrained { patex })
    }
    fn define_invisible_operand(&mut self, sym: SymbolId) -> Option<u32> {
        SleighCompile::define_invisible_operand(self, sym)
    }
    fn pexp_constant(&mut self, val: i64) -> u32 {
        let id = self.patexp.len() as u32;
        self.patexp
            .push(PatternExpression::Value(PatternValue::ConstantValue(
                ConstantValue::new(val),
            )));
        id
    }
    fn pexp_family_value(&mut self, fam: SymbolId) -> u32 {
        let pv = self
            .family_patval(fam)
            .unwrap_or(PatternValue::ConstantValue(ConstantValue::new(0)));
        let id = self.patexp.len() as u32;
        self.patexp.push(PatternExpression::Value(pv));
        id
    }
    fn pexp_spec_expression(&mut self, spec: SymbolId) -> u32 {
        let pe = self
            .base
            .symtab()
            .find_symbol_by_id(spec)
            .and_then(|s| s.get_pattern_expression().ok().flatten())
            .unwrap_or(PatternExpression::Value(PatternValue::ConstantValue(
                ConstantValue::new(0),
            )));
        let id = self.patexp.len() as u32;
        self.patexp.push(pe);
        id
    }
    fn pexp_plus(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Plus)
    }
    fn pexp_sub(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Sub)
    }
    fn pexp_mult(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Mult)
    }
    fn pexp_leftshift(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::LeftShift)
    }
    fn pexp_rightshift(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::RightShift)
    }
    fn pexp_and(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::And)
    }
    fn pexp_or(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Or)
    }
    fn pexp_xor(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Xor)
    }
    fn pexp_div(&mut self, l: u32, r: u32) -> u32 {
        self.build_binexp(l, r, BinKind::Div)
    }
    fn pexp_minus(&mut self, e: u32) -> u32 {
        let u = kuna_sleigh::slghpatexpress::UnaryExpression::new(self.patexp_get(e));
        let id = self.patexp.len() as u32;
        self.patexp.push(PatternExpression::Minus(u));
        id
    }
    fn pexp_not(&mut self, e: u32) -> u32 {
        let u = kuna_sleigh::slghpatexpress::UnaryExpression::new(self.patexp_get(e));
        let id = self.patexp.len() as u32;
        self.patexp.push(PatternExpression::Not(u));
        id
    }
    fn context_mod(&mut self, vec: &mut Vec<u32>, sym: SymbolId, pe: u32) -> bool {
        SleighCompile::context_mod(self, vec, sym, pe)
    }
    fn context_set(&mut self, vec: &mut Vec<u32>, sym: SymbolId, cvar: SymbolId) {
        SleighCompile::context_set(self, vec, sym, cvar)
    }
    fn define_operand(&mut self, sym: SymbolId, patexp: u32) {
        SleighCompile::define_operand(self, sym, patexp)
    }
    fn pcode_new_local_definition(&mut self, name: &[u8], size: Option<u64>) {
        SleighCompile::pcode_new_local_definition(self, name, size)
    }
    fn pcode_new_output(&mut self, islocal: bool, expr: u32, name: &[u8], size: Option<u64>) -> u32 {
        SleighCompile::pcode_new_output(self, islocal, expr, name, size)
    }
    fn stmt_assign(&mut self, lhs: u32, expr: u32) -> u32 {
        SleighCompile::stmt_assign(self, lhs, expr)
    }
    fn pcode_create_store(&mut self, star: u32, ptr: u32, val: u32) -> u32 {
        SleighCompile::pcode_create_store(self, star, ptr, val)
    }
    fn pcode_create_user_op_noout(&mut self, sym: SymbolId, params: Vec<u32>) -> u32 {
        SleighCompile::pcode_create_user_op_noout(self, sym, params)
    }
    fn pcode_assign_bitrange_idx(&mut self, lhs: u32, off: u32, size: u32, expr: u32) -> u32 {
        SleighCompile::pcode_assign_bitrange_idx(self, lhs, off, size, expr)
    }
    fn pcode_assign_bitrange_bitsym(&mut self, bitsym: SymbolId, expr: u32) -> u32 {
        SleighCompile::pcode_assign_bitrange_bitsym(self, bitsym, expr)
    }
    fn pcode_create_op_const(&mut self, op: ConstOp, val: u64) -> u32 {
        SleighCompile::pcode_create_op_const(self, op, val)
    }
    fn create_cross_build(&mut self, addr: u32, sym: SymbolId) -> u32 {
        SleighCompile::create_cross_build(self, addr, sym)
    }
    fn pcode_create_op_noout(&mut self, opc: PcodeOpc, a: u32, cond: Option<u32>) -> u32 {
        SleighCompile::pcode_create_op_noout(self, opc, a, cond)
    }
    fn create_macro_use_stmt(&mut self, sym: SymbolId, param: Vec<u32>) -> u32 {
        SleighCompile::create_macro_use_stmt(self, sym, param)
    }
    fn pcode_place_label(&mut self, label: SymbolId) -> u32 {
        SleighCompile::pcode_place_label(self, label)
    }
    fn expr_from_varnode(&mut self, vn: u32) -> u32 {
        SleighCompile::expr_from_varnode(self, vn)
    }
    fn pcode_create_load(&mut self, star: u32, ptr: u32) -> u32 {
        SleighCompile::pcode_create_load(self, star, ptr)
    }
    fn pcode_create_op(&mut self, opc: PcodeOpc, a: u32, b: Option<u32>) -> u32 {
        SleighCompile::pcode_create_op(self, opc, a, b)
    }
    fn pcode_create_bitrange_colon(&mut self, spec: SymbolId, nbytes: u64) -> u32 {
        SleighCompile::pcode_create_bitrange_colon(self, spec, nbytes)
    }
    fn pcode_create_bitrange_idx(&mut self, spec: SymbolId, off: u32, size: u32) -> u32 {
        SleighCompile::pcode_create_bitrange_idx(self, spec, off, size)
    }
    fn pcode_create_bitrange_bitsym(&mut self, bitsym: SymbolId) -> u32 {
        SleighCompile::pcode_create_bitrange_bitsym(self, bitsym)
    }
    fn pcode_create_user_op(&mut self, sym: SymbolId, params: Vec<u32>) -> u32 {
        SleighCompile::pcode_create_user_op(self, sym, params)
    }
    fn pcode_create_variadic_cpoolref(&mut self, params: Vec<u32>) -> u32 {
        SleighCompile::pcode_create_variadic_cpoolref(self, params)
    }
    fn pcode_create_subpiece(&mut self, spec: SymbolId, off: u32) -> u32 {
        SleighCompile::pcode_create_subpiece(self, spec, off)
    }
    fn sizedstar_space_sz(&mut self, spacesym: SymbolId, size: u64) -> u32 {
        SleighCompile::sizedstar_space_sz(self, spacesym, size)
    }
    fn sizedstar_space(&mut self, spacesym: SymbolId) -> u32 {
        SleighCompile::sizedstar_space(self, spacesym)
    }
    fn sizedstar_default_sz(&mut self, size: u64) -> u32 {
        SleighCompile::sizedstar_default_sz(self, size)
    }
    fn sizedstar_default(&mut self) -> u32 {
        SleighCompile::sizedstar_default(self)
    }
    fn jumpdest_jumpsym(&mut self, sym: SymbolId) -> u32 {
        SleighCompile::jumpdest_jumpsym(self, sym)
    }
    fn jumpdest_integer(&mut self, val: u64) -> u32 {
        SleighCompile::jumpdest_integer(self, val)
    }
    fn jumpdest_operandsym(&mut self, sym: SymbolId) -> u32 {
        SleighCompile::jumpdest_operandsym(self, sym)
    }
    fn jumpdest_integer_space(&mut self, val: u64, spacesym: SymbolId) -> u32 {
        SleighCompile::jumpdest_integer_space(self, val, spacesym)
    }
    fn jumpdest_label(&mut self, label: SymbolId) -> u32 {
        SleighCompile::jumpdest_label(self, label)
    }
    fn varnode_spec(&mut self, spec: SymbolId) -> u32 {
        SleighCompile::varnode_spec(self, spec)
    }
    fn intvn_integer(&mut self, val: u64) -> u32 {
        SleighCompile::intvn_integer(self, val)
    }
    fn intvn_integer_colon(&mut self, val: u64, size: u64) -> u32 {
        SleighCompile::intvn_integer_colon(self, val, size)
    }
    fn pcode_address_of(&mut self, vn: u32, size: u64) -> u32 {
        SleighCompile::pcode_address_of(self, vn, size)
    }
    fn lhsvarnode_spec(&mut self, spec: SymbolId) -> u32 {
        SleighCompile::lhsvarnode_spec(self, spec)
    }
    fn exportvarnode_spec(&mut self, spec: SymbolId) -> u32 {
        SleighCompile::exportvarnode_spec(self, spec)
    }
    fn exportvarnode_integer_colon(&mut self, val: u64, size: u64) -> u32 {
        SleighCompile::exportvarnode_integer_colon(self, val, size)
    }
    fn label_sym(&mut self, sym: SymbolId) -> u32 {
        SleighCompile::label_sym(self, sym)
    }
    fn pcode_define_label(&mut self, name: &[u8]) -> u32 {
        SleighCompile::pcode_define_label(self, name)
    }
    fn resolve_symbol(&mut self, name: &[u8]) -> SymbolId {
        self.base
            .symtab()
            .find_symbol(name)
            .map(|s| s.get_id())
            .unwrap_or(NO_SYMBOL)
    }
    fn operand_index(&mut self, sym: SymbolId) -> u32 {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| match s.kind() {
                kuna_sleigh::slghsymbol::SymbolKind::Operand(o) => Some(o.get_index() as u32),
                _ => None,
            })
            .unwrap_or(0)
    }
    fn report_error(&mut self, msg: &str) {
        SleighCompile::report_error(self, msg)
    }
}
