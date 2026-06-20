//! WS2 golden-parse tests for the SLEIGH parser (`slghparse.rs`).
//!
//! Each `tests/golden/parse/<name>.trace` is a golden **action trace** captured
//! from an instrumented `/tmp` copy of the bison grammar (`slghparse.y`): the
//! genuine `sleigh_opt` parses the spec and every grammar action prints a `KT(...)`
//! token in reduction order (the throwaway harness lives in /tmp; the vendored
//! `decompiler/cpp/**` stays byte-untouched -- see
//! `docs/rust-port/sleigh-compiler/ws2-parser.md`).  This test runs the Rust
//! [`SleighParser`] over the SAME spec against a recording mock driver and asserts
//! the emitted trace matches token-for-token.
//!
//! The mock driver ([`MockDriver`]) implements both [`ScannerHost`] and
//! [`ParserActions`].  Because the lexer's `find_symbol` decides whether an
//! identifier becomes a `*SYM` token or a `STRING` -- and that decision depends on
//! the symbol table built *as parsing proceeds* -- the mock maintains a scoped
//! symbol table that mirrors the C++ `SleighCompile`'s symbol additions exactly:
//! every builder that does an `addSymbol` in `slgh_compile.cc` records the
//! (name -> kind) here at the same point, with the same constructor/macro scoping,
//! so the lexer resolves identifiers identically to the bison run.  WS4 fills the
//! real builders; this mock only needs the symbol-table side-effects (it returns
//! dummy arena ids for everything else).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use kuna_slacomp::slghparse::{
    ConstOp, FieldQual, ParserActions, PcodeOpc, SleighParser, SpaceQual, SymId,
};
use kuna_slacomp::slghscan::{ScannerHost, SleighScanner, SymbolTokenKind};

/// One symbol-table scope: name -> symbol kind.
type Scope = HashMap<Vec<u8>, SymbolTokenKind>;

/// The recording mock: a scoped symbol table mirroring `SleighCompile`'s additions
/// (so the lexer resolves identifiers like bison did), plus the action trace and
/// the preprocessor map / include resolver the lexer needs.
struct MockDriver {
    /// Scope stack; `scopes[0]` is the global scope.
    scopes: Vec<Scope>,
    /// Recorded action trace (the `trace(...)` tokens).
    trace: Vec<String>,
    /// Preprocessor values (`@define`).
    preproc: HashMap<Vec<u8>, Vec<u8>>,
    /// Base directory for `@include` resolution.
    base_dir: PathBuf,
    /// Pending context-field names awaiting `calc_context_layout` (the C++
    /// `contexttable`); each flushes to a ContextSymbol when the layout locks.
    pending_context: Vec<Vec<u8>>,
    /// Whether the context layout has locked (`contextlock`).
    context_locked: bool,
    /// Monotonic arena-id counter for the `u32` handles the parser threads.
    next_id: u32,
    /// For each constructor arena id, whether it belongs to the root table
    /// (drives `is_in_root`).
    ctor_in_root: HashMap<u32, bool>,
    /// The most recent `create_constructor`/`create_macro` arena id (so the
    /// matching `build_*` knows it pushed a scope).
    scoped_stack: Vec<()>,
    /// The arena id of the root `instruction` subtable (for `is_in_root`).
    root_subtable_id: Option<SymId>,
    /// Stable id per symbol name (so `create_constructor`'s `sym == root` check
    /// works against the recorded `instruction` id), and the reverse map (id ->
    /// name) the attach* ops use to replace symbol kinds.
    name_ids: HashMap<Vec<u8>, SymId>,
    id_names: HashMap<SymId, Vec<u8>>,
}

impl MockDriver {
    fn new(base_dir: PathBuf) -> MockDriver {
        let mut d = MockDriver {
            scopes: vec![Scope::new()],
            trace: Vec::new(),
            preproc: HashMap::new(),
            base_dir,
            pending_context: Vec::new(),
            context_locked: false,
            next_id: 1,
            ctor_in_root: HashMap::new(),
            scoped_stack: Vec::new(),
            root_subtable_id: None,
            name_ids: HashMap::new(),
            id_names: HashMap::new(),
        };
        d.add_predefined();
        d
    }

    /// A stable arena id for the given symbol name (interned).
    fn id_for_name(&mut self, name: &[u8]) -> SymId {
        if let Some(id) = self.name_ids.get(name) {
            return *id;
        }
        let id = self.fresh();
        self.name_ids.insert(name.to_vec(), id);
        self.id_names.insert(id, name.to_vec());
        id
    }

    /// The name for an interned symbol id (for the attach* replace).
    fn name_for_id(&self, id: SymId) -> Option<Vec<u8>> {
        self.id_names.get(&id).cloned()
    }

    /// Replace the kind of every symbol in `symlist` (the attach* targets).
    fn replace_symlist(&mut self, symlist: &[SymId], kind: SymbolTokenKind) {
        let names: Vec<Vec<u8>> = symlist.iter().filter_map(|id| self.name_for_id(*id)).collect();
        for name in names {
            self.replace(&name, kind);
        }
    }

    /// The C++ `predefinedSymbols()` (slgh_compile.cc:1982): the global scope's
    /// seed symbols the lexer must already resolve.
    fn add_predefined(&mut self) {
        use SymbolTokenKind::*;
        self.global_insert(b"instruction", Subtable);
        // constant / OTHER / unique spaces are SpaceSymbols.
        self.global_insert(b"const", Space);
        self.global_insert(b"OTHER", Space);
        self.global_insert(b"unique", Space);
        // inst_start / inst_next / inst_next2 -> JUMPSYM (start/end/next2 symbols).
        self.global_insert(b"inst_start", Jump);
        self.global_insert(b"inst_next", Jump);
        self.global_insert(b"inst_next2", Jump);
        // epsilon -> SPECSYM.
        self.global_insert(b"epsilon", Spec);
    }

    fn global_insert(&mut self, name: &[u8], kind: SymbolTokenKind) {
        self.scopes[0].insert(name.to_vec(), kind);
    }

    /// `addSymbol`: insert into the current (innermost) scope.
    fn add(&mut self, name: &[u8], kind: SymbolTokenKind) {
        self.scopes.last_mut().unwrap().insert(name.to_vec(), kind);
    }

    /// Replace a symbol's kind wherever it currently resolves (the attach* ops:
    /// `symtab.replaceSymbol`).  Searches innermost-first.
    fn replace(&mut self, name: &[u8], kind: SymbolTokenKind) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_vec(), kind);
                return;
            }
        }
        // Not found: insert globally (defensive; should not happen for valid specs).
        self.global_insert(name, kind);
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
        self.scoped_stack.push(());
    }

    fn pop_scope(&mut self) {
        if self.scoped_stack.pop().is_some() {
            self.scopes.pop();
        }
    }

    fn fresh(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// --- ScannerHost: drive the lexer's symbol resolution + preprocessor ---

impl ScannerHost for MockDriver {
    fn next_line(&mut self) {}
    fn calc_context_layout(&mut self) {
        // calcContextLayout (slgh_compile.cc:2515): flush the pending context
        // fields to ContextSymbols, then lock.
        if self.context_locked {
            return;
        }
        self.context_locked = true;
        let pending = std::mem::take(&mut self.pending_context);
        for name in pending {
            self.global_insert(&name, SymbolTokenKind::Context);
        }
    }
    fn read_include(&mut self, fname: &[u8]) -> Option<Vec<u8>> {
        let name = std::str::from_utf8(fname).ok()?;
        std::fs::read(self.base_dir.join(name)).ok()
    }
    fn parse_file_finished(&mut self) {}
    fn parse_preproc_macro(&mut self) {}
    fn get_preproc_value(&self, name: &[u8]) -> Option<Vec<u8>> {
        self.preproc.get(name).cloned()
    }
    fn set_preproc_value(&mut self, name: &[u8], value: &[u8]) {
        self.preproc.insert(name.to_vec(), value.to_vec());
    }
    fn undefine_preproc_value(&mut self, name: &[u8]) -> bool {
        self.preproc.remove(name).is_some()
    }
    fn find_symbol_kind(&self, name: &[u8]) -> Option<SymbolTokenKind> {
        // findSymbol searches all scopes (innermost first).
        for scope in self.scopes.iter().rev() {
            if let Some(k) = scope.get(name) {
                return Some(*k);
            }
        }
        None
    }
}

// --- ParserActions: record the trace + replicate the symbol-table additions ---

impl ParserActions for MockDriver {
    fn trace(&mut self, tok: &str) {
        self.trace.push(tok.to_string());
    }

    fn set_endian(&mut self, _big: i32) {}
    fn set_alignment(&mut self, _val: i32) {}

    fn define_token(&mut self, name: &[u8], _sz: u64, _endian: i32) -> SymId {
        self.add(name, SymbolTokenKind::Token);
        self.fresh()
    }
    fn add_token_field(&mut self, _sym: SymId, qual: FieldQual) {
        // addTokenField -> addSymbol(new ValueSymbol(qual->name,...)).
        self.add(&qual.name, SymbolTokenKind::Value);
    }
    fn context_prop_begin(&mut self, varsym: SymId) -> SymId {
        varsym
    }
    fn add_context_field(&mut self, _sym: SymId, qual: FieldQual) -> bool {
        // addContextField queues into contexttable; the ContextSymbol is created
        // at calcContextLayout.
        self.pending_context.push(qual.name.clone());
        true
    }
    fn new_space(&mut self, qual: SpaceQual) {
        // newSpace -> addSymbol(new SpaceSymbol(spc)).
        self.add(&qual.name, SymbolTokenKind::Space);
    }
    fn define_varnodes(&mut self, _spacesym: SymId, _off: u64, _size: u64, names: Vec<Vec<u8>>) {
        for n in names {
            if n != b"_" {
                self.add(&n, SymbolTokenKind::Varnode);
            }
        }
    }
    fn define_bitrange(&mut self, name: &[u8], _sym: SymId, _bitoffset: u32, _numb: u32) {
        self.add(name, SymbolTokenKind::Bitrange);
    }
    fn add_user_op(&mut self, names: Vec<Vec<u8>>) {
        for n in names {
            self.add(&n, SymbolTokenKind::Userop);
        }
    }
    fn attach_values(&mut self, symlist: Vec<SymId>, _numlist: Vec<i64>) {
        // attachValues replaces each value symbol with a ValueMapSymbol (so later
        // references still resolve to a familysymbol token -- VALUEMAPSYM).
        self.replace_symlist(&symlist, SymbolTokenKind::ValueMap);
    }
    fn attach_names(&mut self, symlist: Vec<SymId>, _names: Vec<Vec<u8>>) {
        self.replace_symlist(&symlist, SymbolTokenKind::Name);
    }
    fn attach_varnodes(&mut self, symlist: Vec<SymId>, _varlist: Vec<SymId>) {
        self.replace_symlist(&symlist, SymbolTokenKind::VarnodeList);
    }

    fn build_macro(&mut self, _sym: SymId, _rtl: u32) {
        self.pop_scope();
    }
    fn create_macro(&mut self, name: &[u8], params: Vec<Vec<u8>>) -> SymId {
        // createMacro: addSymbol(MacroSymbol) in current scope, then new scope, then
        // each param as an OperandSymbol in the macro scope.
        self.add(name, SymbolTokenKind::Macro);
        self.push_scope();
        for p in params {
            self.add(&p, SymbolTokenKind::Operand);
        }
        self.fresh()
    }
    fn push_with(&mut self, _ss: Option<SymId>, _pateq: Option<u32>, _contvec: Option<Vec<u32>>) {}
    fn pop_with(&mut self) {}
    fn new_table(&mut self, nm: &[u8]) -> SymId {
        self.add(nm, SymbolTokenKind::Subtable);
        self.fresh()
    }
    fn create_constructor(&mut self, sym: Option<SymId>) -> u32 {
        // createConstructor pushes a new scope.  Root membership: None or the root
        // "instruction" subtable -> in root.
        let id = self.fresh();
        let in_root = sym.is_none() || sym == self.root_subtable_id;
        self.ctor_in_root.insert(id, in_root);
        self.push_scope();
        id
    }
    fn reset_constructors(&mut self) {}
    fn add_syntax(&mut self, _ct: u32, _syntax: &[u8]) {}
    fn new_operand(&mut self, _ct: u32, nm: &[u8]) {
        self.add(nm, SymbolTokenKind::Operand);
    }
    fn is_in_root(&self, ct: u32) -> bool {
        *self.ctor_in_root.get(&ct).unwrap_or(&true)
    }
    fn build_constructor(&mut self, _big: u32, _pateq: Option<u32>, _contvec: Option<Vec<u32>>, _vec: u32) {
        self.pop_scope();
    }

    fn standalone_section(&mut self, _main: u32) -> u32 {
        self.fresh()
    }
    fn final_named_section(&mut self, _vec: u32, _section: u32) -> u32 {
        self.fresh()
    }
    fn first_named_section(&mut self, _main: u32, _sym: SymId) -> u32 {
        self.fresh()
    }
    fn next_named_section(&mut self, _vec: u32, _section: u32, _sym: SymId) -> u32 {
        self.fresh()
    }
    fn new_section_symbol(&mut self, nm: &[u8]) -> SymId {
        self.global_insert(nm, SymbolTokenKind::Section);
        self.fresh()
    }
    fn enter_section(&mut self) -> u32 {
        self.fresh()
    }
    fn finish_main_rtl(&mut self, rtlmid: u32) -> u32 {
        rtlmid
    }
    fn set_result_varnode(&mut self, _ct: u32, _vn: u32) -> u32 {
        self.fresh()
    }
    fn set_result_star_varnode(&mut self, _ct: u32, _star: u32, _vn: u32) -> u32 {
        self.fresh()
    }
    fn rtl_add_oplist(&mut self, _sec: u32, _stmt: u32) -> bool {
        true
    }

    fn peq_and(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn peq_or(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn peq_cat(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn peq_left_ellipsis(&mut self, _e: u32) -> u32 { self.fresh() }
    fn peq_right_ellipsis(&mut self, _e: u32) -> u32 { self.fresh() }
    fn peq_equal(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn peq_notequal(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn peq_less(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn peq_lessequal(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn peq_greater(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn peq_greaterequal(&mut self, _fam: SymId, _rhs: u32) -> u32 { self.fresh() }
    fn constrain_operand(&mut self, _sym: SymId, _patexp: u32) -> Option<u32> {
        Some(self.fresh())
    }
    fn peq_operand_equation(&mut self, _sym: SymId) -> u32 { self.fresh() }
    fn self_define(&mut self, _sym: SymId) {}
    fn peq_unconstrained(&mut self, _spec: SymId) -> u32 { self.fresh() }
    fn define_invisible_operand(&mut self, sym: SymId) -> Option<u32> {
        // defineInvisibleOperand adds an OperandSymbol named after the inner symbol
        // into the current constructor scope (slgh_compile.cc:3044), shadowing it,
        // so later references (e.g. `build instruction;`) resolve to OPERANDSYM.
        if let Some(name) = self.name_for_id(sym) {
            self.add(&name, SymbolTokenKind::Operand);
        }
        Some(self.fresh())
    }

    fn pexp_constant(&mut self, _val: i64) -> u32 { self.fresh() }
    fn pexp_family_value(&mut self, _fam: SymId) -> u32 { self.fresh() }
    fn pexp_spec_expression(&mut self, _spec: SymId) -> u32 { self.fresh() }
    fn pexp_plus(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_sub(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_mult(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_leftshift(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_rightshift(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_and(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_or(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_xor(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_div(&mut self, _l: u32, _r: u32) -> u32 { self.fresh() }
    fn pexp_minus(&mut self, _e: u32) -> u32 { self.fresh() }
    fn pexp_not(&mut self, _e: u32) -> u32 { self.fresh() }

    fn context_mod(&mut self, _vec: &mut Vec<u32>, _sym: SymId, _pe: u32) -> bool {
        true
    }
    fn context_set(&mut self, _vec: &mut Vec<u32>, _sym: SymId, _cvar: SymId) {}
    fn define_operand(&mut self, _sym: SymId, _patexp: u32) {}

    fn pcode_new_local_definition(&mut self, name: &[u8], _size: Option<u64>) {
        // newLocalDefinition adds a VarnodeSymbol (a temporary) into the current
        // constructor scope so later references resolve to VARSYM.
        self.add(name, SymbolTokenKind::Varnode);
    }
    fn pcode_new_output(&mut self, _islocal: bool, _expr: u32, name: &[u8], _size: Option<u64>) -> u32 {
        // newOutput defines the named temporary as a VarnodeSymbol (the local
        // declaration side-effect) so later references resolve to VARSYM.
        self.add(name, SymbolTokenKind::Varnode);
        self.fresh()
    }
    fn stmt_assign(&mut self, _lhs: u32, _expr: u32) -> u32 { self.fresh() }
    fn pcode_create_store(&mut self, _star: u32, _ptr: u32, _val: u32) -> u32 { self.fresh() }
    fn pcode_create_user_op_noout(&mut self, _sym: SymId, _params: Vec<u32>) -> u32 { self.fresh() }
    fn pcode_assign_bitrange_idx(&mut self, _lhs: u32, _off: u32, _size: u32, _expr: u32) -> u32 { self.fresh() }
    fn pcode_assign_bitrange_bitsym(&mut self, _bitsym: SymId, _expr: u32) -> u32 { self.fresh() }
    fn pcode_create_op_const(&mut self, _op: ConstOp, _val: u64) -> u32 { self.fresh() }
    fn create_cross_build(&mut self, _addr: u32, _sym: SymId) -> u32 { self.fresh() }
    fn pcode_create_op_noout(&mut self, _opc: PcodeOpc, _a: u32, _cond: Option<u32>) -> u32 { self.fresh() }
    fn create_macro_use_stmt(&mut self, _sym: SymId, _param: Vec<u32>) -> u32 { self.fresh() }
    fn pcode_place_label(&mut self, _label: SymId) -> u32 { self.fresh() }

    fn expr_from_varnode(&mut self, _vn: u32) -> u32 { self.fresh() }
    fn pcode_create_load(&mut self, _star: u32, _ptr: u32) -> u32 { self.fresh() }
    fn pcode_create_op(&mut self, _opc: PcodeOpc, _a: u32, _b: Option<u32>) -> u32 { self.fresh() }
    fn pcode_create_bitrange_colon(&mut self, _spec: SymId, _nbytes: u64) -> u32 { self.fresh() }
    fn pcode_create_bitrange_idx(&mut self, _spec: SymId, _off: u32, _size: u32) -> u32 { self.fresh() }
    fn pcode_create_bitrange_bitsym(&mut self, _bitsym: SymId) -> u32 { self.fresh() }
    fn pcode_create_user_op(&mut self, _sym: SymId, _params: Vec<u32>) -> u32 { self.fresh() }
    fn pcode_create_variadic_cpoolref(&mut self, _params: Vec<u32>) -> u32 { self.fresh() }
    fn pcode_create_subpiece(&mut self, _spec: SymId, _off: u32) -> u32 { self.fresh() }

    fn sizedstar_space_sz(&mut self, _spacesym: SymId, _size: u64) -> u32 { self.fresh() }
    fn sizedstar_space(&mut self, _spacesym: SymId) -> u32 { self.fresh() }
    fn sizedstar_default_sz(&mut self, _size: u64) -> u32 { self.fresh() }
    fn sizedstar_default(&mut self) -> u32 { self.fresh() }

    fn jumpdest_jumpsym(&mut self, _sym: SymId) -> u32 { self.fresh() }
    fn jumpdest_integer(&mut self, _val: u64) -> u32 { self.fresh() }
    fn jumpdest_operandsym(&mut self, _sym: SymId) -> u32 { self.fresh() }
    fn jumpdest_integer_space(&mut self, _val: u64, _spacesym: SymId) -> u32 { self.fresh() }
    fn jumpdest_label(&mut self, _label: SymId) -> u32 { self.fresh() }

    fn varnode_spec(&mut self, _spec: SymId) -> u32 { self.fresh() }
    fn intvn_integer(&mut self, _val: u64) -> u32 { self.fresh() }
    fn intvn_integer_colon(&mut self, _val: u64, _size: u64) -> u32 { self.fresh() }
    fn pcode_address_of(&mut self, _vn: u32, _size: u64) -> u32 { self.fresh() }
    fn lhsvarnode_spec(&mut self, _spec: SymId) -> u32 { self.fresh() }
    fn exportvarnode_spec(&mut self, _spec: SymId) -> u32 { self.fresh() }
    fn exportvarnode_integer_colon(&mut self, _val: u64, _size: u64) -> u32 { self.fresh() }

    fn label_sym(&mut self, _sym: SymId) -> u32 { self.fresh() }
    fn pcode_define_label(&mut self, name: &[u8]) -> u32 {
        // defineLabel adds a LabelSymbol into the constructor scope.
        self.add(name, SymbolTokenKind::Label);
        self.fresh()
    }

    fn resolve_symbol(&mut self, name: &[u8]) -> SymId {
        // Returns a stable id per distinct name so `create_constructor`'s root
        // check (sym == root) can compare against the recorded `instruction` id,
        // and the attach* ops can map ids back to names.
        self.id_for_name(name)
    }
    fn operand_index(&mut self, _sym: SymId) -> u32 {
        0
    }
    fn report_error(&mut self, _msg: &str) {}
}

/// Run the Rust parser over `spec` and return the recorded trace (one token/line).
fn parse_to_trace(spec: &Path) -> String {
    let contents = std::fs::read(spec).expect("read spec");
    let base_dir = spec.parent().unwrap().to_path_buf();
    let mut driver = MockDriver::new(base_dir);
    // Seed the `instruction` root-subtable id so is_in_root can compare.
    driver.root_subtable_id = Some(driver.id_for_name(b"instruction"));
    let mut scanner = SleighScanner::new();
    scanner.open(contents);
    let mut parser = SleighParser::new(scanner);
    parser.parse_driver(&mut driver).expect("parse");
    let mut out = driver.trace.join("\n");
    out.push('\n');
    out
}

/// Assert the Rust parser reproduces the golden trace for `name`.
fn check_golden(name: &str, spec_rel: &str) {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();
    let spec = repo_root.join(spec_rel);
    let gold_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden/parse")
        .join(format!("{name}.trace"));
    let gold = std::fs::read_to_string(&gold_path)
        .unwrap_or_else(|_| panic!("missing golden {}", gold_path.display()));
    let got = parse_to_trace(&spec);
    let gold_lines: Vec<&str> = gold.lines().collect();
    let got_lines: Vec<&str> = got.lines().collect();
    for (i, (g, r)) in gold_lines.iter().zip(got_lines.iter()).enumerate() {
        assert_eq!(
            g, r,
            "[{name}] action {i} mismatch:\n  golden: {g}\n  rust:   {r}\n  (context golden[{}..{}]: {:?})",
            i.saturating_sub(3),
            (i + 1).min(gold_lines.len()),
            &gold_lines[i.saturating_sub(3)..(i + 1).min(gold_lines.len())],
        );
    }
    assert_eq!(
        gold_lines.len(),
        got_lines.len(),
        "[{name}] action count mismatch: golden={} rust={}\n  golden tail: {:?}\n  rust tail:   {:?}",
        gold_lines.len(),
        got_lines.len(),
        &gold_lines[gold_lines.len().saturating_sub(5)..],
        &got_lines[got_lines.len().saturating_sub(5)..],
    );
}

// --- the simplest real spec, end-to-end (WS7 first byte-parity target) ---

#[test]
fn golden_data_le_64() {
    check_golden("data_le_64", "specs/Ghidra/Processors/DATA/data/languages/data-le-64.slaspec");
}

#[test]
fn golden_data_be_64() {
    check_golden("data_be_64", "specs/Ghidra/Processors/DATA/data/languages/data-be-64.slaspec");
}

// --- Toy builder: smallest real constructors/patterns/p-code, with-blocks,
//     macros, context, pcodeops, globalset, export, build (climbing complexity) ---

#[test]
fn golden_toy_builder_le() {
    check_golden("toy_builder_le", "specs/Ghidra/Processors/Toy/data/languages/toy_builder_le.slaspec");
}

#[test]
fn golden_toy_builder_be() {
    check_golden("toy_builder_be", "specs/Ghidra/Processors/Toy/data/languages/toy_builder_be.slaspec");
}

#[test]
fn golden_toy_builder_le_align2() {
    check_golden("toy_builder_le_align2", "specs/Ghidra/Processors/Toy/data/languages/toy_builder_le_align2.slaspec");
}

// --- full Toy ISA (attaches, with-blocks, macros, contexts) ---

#[test]
fn golden_toy_le() {
    check_golden("toy_le", "specs/Ghidra/Processors/Toy/data/languages/toy_le.slaspec");
}

#[test]
fn golden_toy64_be() {
    check_golden("toy64_be", "specs/Ghidra/Processors/Toy/data/languages/toy64_be.slaspec");
}

// --- small real ISAs ---

#[test]
fn golden_bpf_le() {
    check_golden("bpf_le", "specs/Ghidra/Processors/BPF/data/languages/BPF_le.slaspec");
}

#[test]
fn golden_m6502() {
    check_golden("m6502", "specs/Ghidra/Processors/6502/data/languages/6502.slaspec");
}
