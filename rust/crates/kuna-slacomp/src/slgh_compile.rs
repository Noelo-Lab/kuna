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

use kuna_sleigh::pcodecompile::{Location, PcodeCompileSymbol};
use kuna_sleigh::slghpatexpress::{
    ConstantValue, ContextField, EquationArena, PatternEquation, PatternExpression, PatternValue,
};
use kuna_sleigh::slghsymbol::{ContextChange, DecisionProperties, SleighSymbol, SymbolType};
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

/// A named p-code section paired with its symbol scope (`RtlPair`,
/// slgh_compile.hh:42-47).
#[derive(Clone, Copy, Default)]
pub struct RtlPair {
    /// `ConstructTpl` handle in the base template arena (or `None`).
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
// ConsistencyChecker (slgh_compile.hh:142-235) -- trivial in the landed subset.
// ---------------------------------------------------------------------------

/// Derives Varnode sizes, optimizes p-code, and checks validity in Constructor
/// p-code (`ConsistencyChecker`, slgh_compile.hh:142-235).
#[derive(Default)]
pub struct ConsistencyChecker {
    pub unnecessarypcode: i32,
    pub readnowrite: i32,
    pub writenoread: i32,
    pub printextwarning: bool,
    pub printdeadwarning: bool,
    pub root_symbol: SymbolId,
}

impl ConsistencyChecker {
    /// `ConsistencyChecker::ConsistencyChecker` (slgh_compile.cc:297).
    pub fn new(root: SymbolId, unnecessary: bool, warndead: bool) -> ConsistencyChecker {
        ConsistencyChecker {
            root_symbol: root,
            printextwarning: unnecessary,
            printdeadwarning: warndead,
            ..Default::default()
        }
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
        self.base.set_unique_base(base + 0x10000); // SleighBase::MAX_UNIQUE_SIZE
        base
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
        let root = self.base.get_root().expect("root set");
        let _checker = ConsistencyChecker::new(root, self.warnunnecessarypcode, self.warndeadtemps);
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
    fn check_unique_allocation(&mut self) {
        if self.base.get_unique_allocatemask() == 0 {
            return;
        }
        // Crossbuild unique re-allocation is part of the unported p-code path.
    }

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
    fn add_symbol(&mut self, _sym: PcodeCompileSymbol) {
        // p-code label/local symbol additions are part of the unported section path.
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
    fn build_macro(&mut self, _sym: SymbolId, _rtl: u32) {
        pcode_unported("build_macro")
    }
    fn create_macro(&mut self, _name: &[u8], _params: Vec<Vec<u8>>) -> SymbolId {
        pcode_unported("create_macro")
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
        let v = if vec == u32::MAX {
            None
        } else {
            Some(SectionVector::default())
        };
        SleighCompile::build_constructor(self, big, pateq, contvec, v)
    }
    fn standalone_section(&mut self, _main: u32) -> u32 {
        pcode_unported("standalone_section")
    }
    fn final_named_section(&mut self, _vec: u32, _section: u32) -> u32 {
        pcode_unported("final_named_section")
    }
    fn first_named_section(&mut self, _main: u32, _sym: SymbolId) -> u32 {
        pcode_unported("first_named_section")
    }
    fn next_named_section(&mut self, _vec: u32, _section: u32, _sym: SymbolId) -> u32 {
        pcode_unported("next_named_section")
    }
    fn new_section_symbol(&mut self, _nm: &[u8]) -> SymbolId {
        pcode_unported("new_section_symbol")
    }
    fn enter_section(&mut self) -> u32 {
        pcode_unported("enter_section")
    }
    fn finish_main_rtl(&mut self, _rtlmid: u32) -> u32 {
        pcode_unported("finish_main_rtl")
    }
    fn set_result_varnode(&mut self, _ct: u32, _vn: u32) -> u32 {
        pcode_unported("set_result_varnode")
    }
    fn set_result_star_varnode(&mut self, _ct: u32, _star: u32, _vn: u32) -> u32 {
        pcode_unported("set_result_star_varnode")
    }
    fn rtl_add_oplist(&mut self, _sec: u32, _stmt: u32) -> bool {
        pcode_unported("rtl_add_oplist")
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
    fn constrain_operand(&mut self, _sym: SymbolId, _patexp: u32) -> Option<u32> {
        pcode_unported("constrain_operand")
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
    fn self_define(&mut self, _sym: SymbolId) {
        pcode_unported("self_define")
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
    fn define_invisible_operand(&mut self, _sym: SymbolId) -> Option<u32> {
        pcode_unported("define_invisible_operand")
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
    fn context_mod(&mut self, _vec: &mut Vec<u32>, _sym: SymbolId, _pe: u32) -> bool {
        pcode_unported("context_mod")
    }
    fn context_set(&mut self, _vec: &mut Vec<u32>, _sym: SymbolId, _cvar: SymbolId) {
        pcode_unported("context_set")
    }
    fn define_operand(&mut self, _sym: SymbolId, _patexp: u32) {
        pcode_unported("define_operand")
    }
    fn pcode_new_local_definition(&mut self, _name: &[u8], _size: Option<u64>) {
        pcode_unported("pcode_new_local_definition")
    }
    fn pcode_new_output(&mut self, _islocal: bool, _expr: u32, _name: &[u8], _size: Option<u64>) -> u32 {
        pcode_unported("pcode_new_output")
    }
    fn stmt_assign(&mut self, _lhs: u32, _expr: u32) -> u32 {
        pcode_unported("stmt_assign")
    }
    fn pcode_create_store(&mut self, _star: u32, _ptr: u32, _val: u32) -> u32 {
        pcode_unported("pcode_create_store")
    }
    fn pcode_create_user_op_noout(&mut self, _sym: SymbolId, _params: Vec<u32>) -> u32 {
        pcode_unported("pcode_create_user_op_noout")
    }
    fn pcode_assign_bitrange_idx(&mut self, _lhs: u32, _off: u32, _size: u32, _expr: u32) -> u32 {
        pcode_unported("pcode_assign_bitrange_idx")
    }
    fn pcode_assign_bitrange_bitsym(&mut self, _bitsym: SymbolId, _expr: u32) -> u32 {
        pcode_unported("pcode_assign_bitrange_bitsym")
    }
    fn pcode_create_op_const(&mut self, _op: ConstOp, _val: u64) -> u32 {
        pcode_unported("pcode_create_op_const")
    }
    fn create_cross_build(&mut self, _addr: u32, _sym: SymbolId) -> u32 {
        pcode_unported("create_cross_build")
    }
    fn pcode_create_op_noout(&mut self, _opc: PcodeOpc, _a: u32, _cond: Option<u32>) -> u32 {
        pcode_unported("pcode_create_op_noout")
    }
    fn create_macro_use_stmt(&mut self, _sym: SymbolId, _param: Vec<u32>) -> u32 {
        pcode_unported("create_macro_use_stmt")
    }
    fn pcode_place_label(&mut self, _label: SymbolId) -> u32 {
        pcode_unported("pcode_place_label")
    }
    fn expr_from_varnode(&mut self, _vn: u32) -> u32 {
        pcode_unported("expr_from_varnode")
    }
    fn pcode_create_load(&mut self, _star: u32, _ptr: u32) -> u32 {
        pcode_unported("pcode_create_load")
    }
    fn pcode_create_op(&mut self, _opc: PcodeOpc, _a: u32, _b: Option<u32>) -> u32 {
        pcode_unported("pcode_create_op")
    }
    fn pcode_create_bitrange_colon(&mut self, _spec: SymbolId, _nbytes: u64) -> u32 {
        pcode_unported("pcode_create_bitrange_colon")
    }
    fn pcode_create_bitrange_idx(&mut self, _spec: SymbolId, _off: u32, _size: u32) -> u32 {
        pcode_unported("pcode_create_bitrange_idx")
    }
    fn pcode_create_bitrange_bitsym(&mut self, _bitsym: SymbolId) -> u32 {
        pcode_unported("pcode_create_bitrange_bitsym")
    }
    fn pcode_create_user_op(&mut self, _sym: SymbolId, _params: Vec<u32>) -> u32 {
        pcode_unported("pcode_create_user_op")
    }
    fn pcode_create_variadic_cpoolref(&mut self, _params: Vec<u32>) -> u32 {
        pcode_unported("pcode_create_variadic_cpoolref")
    }
    fn pcode_create_subpiece(&mut self, _spec: SymbolId, _off: u32) -> u32 {
        pcode_unported("pcode_create_subpiece")
    }
    fn sizedstar_space_sz(&mut self, _spacesym: SymbolId, _size: u64) -> u32 {
        pcode_unported("sizedstar_space_sz")
    }
    fn sizedstar_space(&mut self, _spacesym: SymbolId) -> u32 {
        pcode_unported("sizedstar_space")
    }
    fn sizedstar_default_sz(&mut self, _size: u64) -> u32 {
        pcode_unported("sizedstar_default_sz")
    }
    fn sizedstar_default(&mut self) -> u32 {
        pcode_unported("sizedstar_default")
    }
    fn jumpdest_jumpsym(&mut self, _sym: SymbolId) -> u32 {
        pcode_unported("jumpdest_jumpsym")
    }
    fn jumpdest_integer(&mut self, _val: u64) -> u32 {
        pcode_unported("jumpdest_integer")
    }
    fn jumpdest_operandsym(&mut self, _sym: SymbolId) -> u32 {
        pcode_unported("jumpdest_operandsym")
    }
    fn jumpdest_integer_space(&mut self, _val: u64, _spacesym: SymbolId) -> u32 {
        pcode_unported("jumpdest_integer_space")
    }
    fn jumpdest_label(&mut self, _label: SymbolId) -> u32 {
        pcode_unported("jumpdest_label")
    }
    fn varnode_spec(&mut self, _spec: SymbolId) -> u32 {
        pcode_unported("varnode_spec")
    }
    fn intvn_integer(&mut self, _val: u64) -> u32 {
        pcode_unported("intvn_integer")
    }
    fn intvn_integer_colon(&mut self, _val: u64, _size: u64) -> u32 {
        pcode_unported("intvn_integer_colon")
    }
    fn pcode_address_of(&mut self, _vn: u32, _size: u64) -> u32 {
        pcode_unported("pcode_address_of")
    }
    fn lhsvarnode_spec(&mut self, _spec: SymbolId) -> u32 {
        pcode_unported("lhsvarnode_spec")
    }
    fn exportvarnode_spec(&mut self, _spec: SymbolId) -> u32 {
        pcode_unported("exportvarnode_spec")
    }
    fn exportvarnode_integer_colon(&mut self, _val: u64, _size: u64) -> u32 {
        pcode_unported("exportvarnode_integer_colon")
    }
    fn label_sym(&mut self, _sym: SymbolId) -> u32 {
        pcode_unported("label_sym")
    }
    fn pcode_define_label(&mut self, _name: &[u8]) -> u32 {
        pcode_unported("pcode_define_label")
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
