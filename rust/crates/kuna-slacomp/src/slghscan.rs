//! WS1 -- the SLEIGH lexer (port of `decompiler/cpp/slghscan.l`).
//!
//! The C++ lexer is a flex scanner with **start-conditions** (`%x`) that switch
//! the active token set per syntactic region, plus a hand-written preprocessor
//! layer (`@include` / `@define` / `@ifdef` / `@if` ... `@endif`) that runs
//! *before* tokenization.  This module is the hand port of that scanner.
//!
//! ## Start conditions (flex `%x`, slghscan.l:483-488)
//!
//! - `INITIAL`    -- top level: `define`/`attach`/`macro`/`with`/subtable names.
//! - `defblock`   -- inside `define`/`attach` blocks (token/context/space/varnode).
//! - `macroblock` -- a macro's parameter list `( ... )`.
//! - `print`      -- a constructor's display (mnemonic) section, up to `is`.
//! - `pattern`    -- a constructor's pattern/context section, up to `{`.
//! - `sem`        -- a constructor's semantic (p-code) section `{ ... }`.
//! - `preproc`    -- transient state used while a preprocessor directive erases
//!   a region (`last_preproc` saves the state to return to).
//!
//! The scanner returns to `INITIAL` (or the saved state) on the structural
//! delimiters (`;`, `is`, `{`, `}`); `slgh->calcContextLayout()` is triggered as
//! a side effect when `attach`/`with` is scanned (slghscan.l:501-502).
//!
//! ## Module ownership: WS1 owns this file exclusively.

#![allow(dead_code)]

use kuna_base::error::KunaResult;

/// Lexer start-conditions, mirroring the flex `%x` states (slghscan.l:483-488).
///
/// `Preproc` corresponds to the `preproc` `%x` state used during directive
/// erasure; the state to resume is saved in [`SleighScanner::last_preproc`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanState {
    /// Top-level (flex `INITIAL`).
    Initial,
    /// `define`/`attach` blocks (flex `defblock`).
    DefBlock,
    /// Macro parameter list (flex `macroblock`).
    MacroBlock,
    /// Constructor display section (flex `print`).
    Print,
    /// Constructor pattern/context section (flex `pattern`).
    Pattern,
    /// Constructor semantic section (flex `sem`).
    Sem,
    /// Transient preprocessor-erasure state (flex `preproc`).
    Preproc,
}

/// The semantic value carried alongside a token.
///
/// This is the Rust analogue of the bison `union SLEIGHSTYPE` (slghparse.hh:197)
/// for the *lexer-produced* alternatives only -- the lexer only ever fills the
/// scalar/string/symbol-handle variants; the parser builds the AST variants.
/// The symbol-bearing tokens (`*SYM`) carry a handle the parser resolves against
/// the symbol table (so this enum stays lexer-side and dependency-light).
#[derive(Clone, Debug, Default)]
pub enum TokenValue {
    /// No semantic payload.
    #[default]
    None,
    /// `CHAR` (slghscan.l: a single character in a charstring).
    Char(u8),
    /// `INTEGER` -- an unsigned integer literal (bison `uintb *i`).
    Integer(u64),
    /// `INTB` -- a signed big integer literal (bison `intb *big`).
    Intb(i64),
    /// `STRING` / `SYMBOLSTRING` -- an identifier or quoted string (bison `string *str`).
    Str(Vec<u8>),
    /// A token that resolves to an existing symbol: carries the symbol name the
    /// parser looks up (bison `*sym` handle alternatives).  WS2 maps this to a
    /// concrete `SleighSymbol` reference during parsing.
    SymbolName(Vec<u8>),
}

/// Token kinds, transcribed from the bison `enum sleightokentype`
/// (slghparse.hh:74-191).  Numeric discriminants are intentionally *not* pinned
/// to bison's values -- the hand parser matches on these variants, not integers.
///
/// The `*_KEY` variants are keywords; the `OP_*` variants are p-code operators;
/// the `*SYM` variants are emitted when an identifier resolves to an existing
/// symbol of that kind (`find_symbol`, slghscan.l:389).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // End of input.
    Eof,

    // Boolean / bitwise / comparison / shift / arithmetic operators
    OpBoolOr, OpBoolAnd, OpBoolXor,
    OpOr, OpXor, OpAnd,
    OpEqual, OpNotEqual, OpFEqual, OpFNotEqual,
    OpGreatEqual, OpLessEqual, OpSless, OpSgreatEqual, OpSlessEqual, OpSgreat,
    OpFless, OpFgreat, OpFlessEqual, OpFgreatEqual,
    OpLeft, OpRight, OpSright,
    OpFadd, OpFsub, OpSdiv, OpSrem, OpFmult, OpFdiv,

    // Unary / intrinsic p-code operators
    OpZext, OpCarry, OpBorrow, OpSext, OpScarry, OpSborrow, OpNan, OpAbs,
    OpSqrt, OpCeil, OpFloor, OpRound, OpInt2Float, OpFloat2Float,
    OpTrunc, OpCpoolref, OpNew, OpPopcount, OpLzcount, OpUnimpl,

    BadInteger,

    // Statement keywords
    GotoKey, CallKey, ReturnKey, IfKey,

    // Definition keywords
    DefineKey, AttachKey, MacroKey, SpaceKey, TypeKey, RamKey, DefaultKey,
    RegisterKey, EndianKey, WithKey, AlignKey,
    TokenKey, SignedKey, NoflowKey, HexKey, DecKey, BigKey, LittleKey,
    SizeKey, WordsizeKey, OffsetKey, NamesKey, ValuesKey, VariablesKey, PcodeopKey,
    IsKey, LocalKey, DelayslotKey, CrossbuildKey, ExportKey, BuildKey, ContextKey,
    EllipsisKey, GlobalsetKey, BitrangeKey,

    // Literals
    Char, Integer, Intb, String, SymbolString,

    // Existing-symbol tokens (`find_symbol`)
    SpaceSym, SectionSym, TokenSym, UseropSym, ValueSym, ValueMapSym, ContextSym,
    NameSym, VarSym, BitSym, SpecSym, VarListSym, OperandSym, JumpSym, MacroSym,
    LabelSym, SubtableSym,

    // Single-character punctuation/operator tokens returned verbatim by the
    // flex `.` rule (e.g. `;` `:` `(` `)` `[` `]` `{` `}` `,` `=` `*` `+` ...).
    Char1(u8),
}

/// One scanned token: its kind plus any semantic payload.
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

/// State of a single open source file in the include stack
/// (`FileStreamState` / `filebuffers`, slghscan.l:35-44).
struct FileStreamState {
    /// Remaining bytes of the file being scanned.
    buffer: Vec<u8>,
    /// Cursor into `buffer`.
    pos: usize,
}

/// The SLEIGH lexer: a hand port of the flex scanner in `slghscan.l`.
///
/// Owns the include stack, the active start-condition, and the preprocessor's
/// `@if` nesting state.  It does **not** own the symbol table or the
/// [`crate::slgh_compile::SleighCompile`] driver; `find_symbol`-style lookups and
/// the `nextLine`/`calcContextLayout`/`parseFromNewFile` side effects are routed
/// back to the driver through the [`ScannerHost`] trait so this module stays
/// file-disjoint from WS4.
pub struct SleighScanner {
    /// Current flex start-condition.
    state: ScanState,
    /// Saved state to resume after a preprocessor erasure (`last_preproc`).
    last_preproc: ScanState,
    /// Whether `&`/`|`/`^` are "action-on" in the current pattern section
    /// (`actionon`, slghscan.l:42).
    actionon: bool,
    /// Whether we are between `with` and its `{` (`withsection`, slghscan.l:43).
    withsection: bool,
    /// The include stack (`filebuffers`).
    filebuffers: Vec<FileStreamState>,
    /// `@if`/`@ifdef` truth stack (`ifstack`, slghscan.l:45).
    ifstack: Vec<i32>,
    /// Depth at which a `@if` evaluated false (`negative_if`, slghscan.l:46).
    negative_if: i32,
}

/// Side-effect callbacks the scanner makes back into the compile driver.
///
/// Mirrors the `slgh->...` calls embedded in `slghscan.l` (e.g. `nextLine`,
/// `calcContextLayout`, `parseFromNewFile`/`parseFileFinished`, preproc value
/// get/set, `find_symbol` resolution).  WS4 implements this on `SleighCompile`;
/// keeping it a trait lets WS1 own `slghscan.rs` without touching `slgh_compile.rs`.
pub trait ScannerHost {
    /// Advance the current-file line counter (`slgh->nextLine()`).
    fn next_line(&mut self);
    /// Finalize the context layout (`slgh->calcContextLayout()`), triggered by
    /// `attach`/`with`.
    fn calc_context_layout(&mut self);
    /// Push a new include file onto the parse stack (`slgh->parseFromNewFile`).
    fn parse_from_new_file(&mut self, fname: &[u8]);
    /// Pop the current file/macro (`slgh->parseFileFinished`).
    fn parse_file_finished(&mut self);
    /// Mark start of an expanded preprocessor macro (`slgh->parsePreprocMacro`).
    fn parse_preproc_macro(&mut self);
    /// Look up a preprocessor variable (`slgh->getPreprocValue`).
    fn get_preproc_value(&self, name: &[u8]) -> Option<Vec<u8>>;
    /// Set a preprocessor variable (`slgh->setPreprocValue`).
    fn set_preproc_value(&mut self, name: &[u8], value: &[u8]);
    /// Remove a preprocessor variable (`slgh->undefinePreprocValue`).
    fn undefine_preproc_value(&mut self, name: &[u8]) -> bool;
    /// Resolve an identifier to an existing symbol's kind, for `find_symbol`
    /// (slghscan.l:389).  Returns `None` if the identifier is unknown (-> `STRING`).
    fn find_symbol_kind(&self, name: &[u8]) -> Option<TokenKind>;
}

impl SleighScanner {
    /// Construct a scanner with an empty include stack at the `INITIAL` state.
    pub fn new() -> SleighScanner {
        SleighScanner {
            state: ScanState::Initial,
            last_preproc: ScanState::Initial,
            actionon: false,
            withsection: false,
            filebuffers: Vec::new(),
            ifstack: Vec::new(),
            negative_if: -1,
        }
    }

    /// Begin scanning a new top-level source file (the lexer's `sleighin` open in
    /// `run_compilation`, slgh_compile.cc:3779).
    pub fn open(&mut self, _contents: Vec<u8>) {
        todo!("WS1: slghscan.l yy_scan_buffer / FileStreamState push (slghscan.l:35-44)")
    }

    /// Produce the next token, switching start-conditions and driving the
    /// preprocessor as the flex rules do.  Returns [`TokenKind::Eof`] at end of
    /// the outermost file.  `host` receives the embedded `slgh->...` side effects.
    pub fn next_token(&mut self, _host: &mut dyn ScannerHost) -> KunaResult<Token> {
        todo!("WS1: slghscan.l %% rule block (the main scanner loop, slghscan.l:489-672)")
    }

    /// Set the active start-condition (flex `BEGIN(state)`).
    pub fn begin(&mut self, state: ScanState) {
        self.state = state;
    }

    /// Reset the lexer between files (`sleighlex_destroy`, slgh_compile.cc:3788).
    pub fn destroy(&mut self) {
        todo!("WS1: sleighlex_destroy -- clear buffers/ifstack/state")
    }

    // --- preprocessor layer (slghscan.l:48-389) ---

    /// Run a preprocessor directive starting at the current `@`, returning the
    /// start-condition to resume in (`preprocess`, slghscan.l:232).
    fn preprocess(&mut self, _cur_state: ScanState, _blank_state: ScanState) -> ScanState {
        todo!("WS1: slghscan.l preprocess() (slghscan.l:232-357)")
    }

    /// Evaluate a preprocessor boolean clause (`preprocess_if`, slghscan.l:171).
    fn preprocess_if(&mut self) -> bool {
        todo!("WS1: slghscan.l preprocess_if() (slghscan.l:171-196)")
    }
}

impl Default for SleighScanner {
    fn default() -> Self {
        Self::new()
    }
}
