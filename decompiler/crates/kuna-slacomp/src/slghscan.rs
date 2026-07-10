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
//! ## Porting strategy: longest-match per start-condition
//!
//! flex picks, at each input position, the rule with the **longest match**
//! (ties broken by rule order).  This module reproduces that explicitly: for
//! the current [`ScanState`] it tries each rule's matcher in the flex source
//! order, keeps the longest match, then runs that rule's action.  An action
//! either returns a [`Token`] or loops (whitespace / comment / preprocessor /
//! `$(...)` macro expansion produce no token).
//!
//! Two flex features are honored exactly:
//! - The `^@[^\n]*\n?` preprocessor rule and the `<preproc>^.*\n` erasure rule
//!   are **column-0 anchored** (the leading `^`): an `@` mid-line falls to the
//!   `.` rule instead.  We track whether the cursor is at the start of a line.
//! - The number rules and the operator rules interact via longest-match: e.g.
//!   `0xZZ` lexes as `INTEGER 0` then identifier `xZZ`, because `0x[0-9a-fA-F]+`
//!   needs at least one hex digit and otherwise loses to `[0-9]+` matching `0`.
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TokenValue {
    /// No semantic payload.
    #[default]
    None,
    /// `CHAR` (slghscan.l: a single character in a charstring; bison `char ch`).
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Token {
    /// A token with no semantic payload.
    fn bare(kind: TokenKind) -> Token {
        Token { kind, value: TokenValue::None }
    }
}

/// State of a single open source file in the include stack
/// (`FileStreamState` / `filebuffers`, slghscan.l:35-44).
///
/// In flex this saves the `YY_BUFFER_STATE` of the *outer* file; we keep the
/// outer buffer's remaining bytes + cursor + line-start flag so it resumes
/// exactly where it left off.
struct FileStreamState {
    /// The outer file/macro buffer being scanned when this one was pushed.
    buffer: Vec<u8>,
    /// Cursor into `buffer`.
    pos: usize,
    /// Whether the outer cursor sat at the start of a line (`^` anchor).
    at_line_start: bool,
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
    /// The current file/macro buffer (the flex "current buffer").
    buffer: Vec<u8>,
    /// Cursor into `buffer`.
    pos: usize,
    /// Whether the cursor sits at the start of a line (for the `^` anchors).
    at_line_start: bool,
    /// The include/macro stack (`filebuffers`): outer buffers to resume on EOF.
    filebuffers: Vec<FileStreamState>,
    /// `@if`/`@ifdef` truth stack (`ifstack`, slghscan.l:45).  Each entry is the
    /// flex `int4` truth/flags word (bit0=current-clause-true, bit1=seen-else,
    /// bit2=seen-true-elif).
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
    /// Resolve and read an `@include` file.  Mirrors the C++
    /// `slgh->parseFromNewFile(fname); fname = slgh->grabCurrentFilePath();
    /// sleighin = fopen(fname,...)` sequence in `preprocess()`: the driver
    /// resolves `fname` against the include path and records the new file as
    /// current, and the lexer reads its bytes.  Returns the file contents, or
    /// `None` if the file could not be opened (C++ aborts; WS4 reports the
    /// error).  **(WS1 freeze addition -- see docs/rust-port/sleigh-compiler/
    /// ws1-lexer.md; replaces the bare `parse_from_new_file` the skeleton
    /// declared.)**
    fn read_include(&mut self, fname: &[u8]) -> Option<Vec<u8>>;
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
    fn find_symbol_kind(&self, name: &[u8]) -> Option<SymbolTokenKind>;
    /// Report a fatal preprocessor error (`preproc_error`, slghscan.l:48): the
    /// C++ prints and `exit(1)`s.  The default panics, matching the abort; WS4
    /// may route it through its error machinery.
    fn preproc_error(&mut self, msg: &str) -> ! {
        panic!("SLEIGH preprocessor error: {msg}");
    }
}

/// The symbol-kind a resolved identifier maps to (the `find_symbol` switch,
/// slghscan.l:397-455).  A reduced projection: only the token kind matters to
/// the lexer (the parser re-resolves the actual symbol).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolTokenKind {
    Section,
    Space,
    Token,
    Userop,
    Value,
    ValueMap,
    Name,
    Varnode,
    Bitrange,
    VarnodeList,
    Operand,
    /// start/end/next2/flowdest/flowref -> JUMPSYM.
    Jump,
    Subtable,
    Macro,
    Label,
    /// epsilon_symbol -> SPECSYM.
    Spec,
    Context,
}

impl SymbolTokenKind {
    /// The `TokenKind` the `find_symbol` switch returns for this symbol kind.
    fn token_kind(self) -> TokenKind {
        match self {
            SymbolTokenKind::Section => TokenKind::SectionSym,
            SymbolTokenKind::Space => TokenKind::SpaceSym,
            SymbolTokenKind::Token => TokenKind::TokenSym,
            SymbolTokenKind::Userop => TokenKind::UseropSym,
            SymbolTokenKind::Value => TokenKind::ValueSym,
            SymbolTokenKind::ValueMap => TokenKind::ValueMapSym,
            SymbolTokenKind::Name => TokenKind::NameSym,
            SymbolTokenKind::Varnode => TokenKind::VarSym,
            SymbolTokenKind::Bitrange => TokenKind::BitSym,
            SymbolTokenKind::VarnodeList => TokenKind::VarListSym,
            SymbolTokenKind::Operand => TokenKind::OperandSym,
            SymbolTokenKind::Jump => TokenKind::JumpSym,
            SymbolTokenKind::Subtable => TokenKind::SubtableSym,
            SymbolTokenKind::Macro => TokenKind::MacroSym,
            SymbolTokenKind::Label => TokenKind::LabelSym,
            SymbolTokenKind::Spec => TokenKind::SpecSym,
            SymbolTokenKind::Context => TokenKind::ContextSym,
        }
    }
}

impl SleighScanner {
    /// Construct a scanner with an empty include stack at the `INITIAL` state.
    pub fn new() -> SleighScanner {
        SleighScanner {
            state: ScanState::Initial,
            last_preproc: ScanState::Initial,
            actionon: false,
            withsection: false,
            buffer: Vec::new(),
            pos: 0,
            at_line_start: true,
            filebuffers: Vec::new(),
            ifstack: Vec::new(),
            negative_if: -1,
        }
    }

    /// Begin scanning a new top-level source file (the lexer's `sleighin` open in
    /// `run_compilation`, slgh_compile.cc:3779).
    pub fn open(&mut self, contents: Vec<u8>) {
        self.buffer = contents;
        self.pos = 0;
        self.at_line_start = true;
    }

    /// Set the active start-condition (flex `BEGIN(state)`).
    pub fn begin(&mut self, state: ScanState) {
        self.state = state;
    }

    /// Reset the lexer between files (`sleighlex_destroy`, slgh_compile.cc:3788).
    pub fn destroy(&mut self) {
        self.state = ScanState::Initial;
        self.last_preproc = ScanState::Initial;
        self.actionon = false;
        self.withsection = false;
        self.buffer.clear();
        self.pos = 0;
        self.at_line_start = true;
        self.filebuffers.clear();
        self.ifstack.clear();
        self.negative_if = -1;
    }

    // --- low-level buffer cursor ---

    /// Remaining unread bytes of the current buffer.
    fn rest(&self) -> &[u8] {
        &self.buffer[self.pos..]
    }

    /// Whether the current buffer is exhausted.
    fn at_eof(&self) -> bool {
        self.pos >= self.buffer.len()
    }

    /// Consume `n` bytes, maintaining the line-start flag (`^` anchors track
    /// the last consumed character being a newline).
    fn consume(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.buffer.len());
        if n == 0 {
            return;
        }
        let last = self.buffer[self.pos + n - 1];
        self.pos += n;
        self.at_line_start = last == b'\n';
    }

    /// Produce the next token, switching start-conditions and driving the
    /// preprocessor as the flex rules do.  Returns [`TokenKind::Eof`] at end of
    /// the outermost file.  `host` receives the embedded `slgh->...` side effects.
    pub fn next_token(&mut self, host: &mut dyn ScannerHost) -> KunaResult<Token> {
        loop {
            // End-of-buffer: flex's <<EOF>> rule -- pop an include/macro buffer
            // or terminate at the outermost file.
            if self.at_eof() {
                match self.filebuffers.pop() {
                    Some(fs) => {
                        self.buffer = fs.buffer;
                        self.pos = fs.pos;
                        self.at_line_start = fs.at_line_start;
                        host.parse_file_finished();
                        continue;
                    }
                    None => return Ok(Token::bare(TokenKind::Eof)),
                }
            }
            // Try the rule set for the current start-condition; an action that
            // returns Some(token) yields it, None means "matched but no token"
            // (whitespace / comment / preprocessor / macro expansion) -> loop.
            if let Some(tok) = self.scan_one(host) {
                return Ok(tok);
            }
        }
    }

    /// Scan exactly one flex rule match in the current state, run its action,
    /// and return the token it produces (or `None` for a token-less rule).
    fn scan_one(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        match self.state {
            ScanState::Initial => self.scan_initial(host),
            ScanState::MacroBlock => self.scan_macroblock(host),
            ScanState::DefBlock => self.scan_defblock(host),
            ScanState::Print => self.scan_print(host),
            ScanState::Pattern => self.scan_pattern(host),
            ScanState::Sem => self.scan_sem(host),
            ScanState::Preproc => self.scan_preproc(host),
        }
    }

    // -----------------------------------------------------------------------
    // Shared rule fragments (the rules that every non-preproc state repeats):
    //   ^@[^\n]*\n?                  -> preprocess directive
    //   \$\([..]\)                   -> macro expansion
    // Each returns Some(()) if it consumed input (caller loops via None token).
    // -----------------------------------------------------------------------

    /// Try the `^@[^\n]*\n?` preprocessor rule at the cursor.  Column-0 anchored.
    /// On match, runs `preprocess(cur_state, Preproc)` and re-`BEGIN`s.  Returns
    /// `true` if it matched (the caller continues with no token).
    fn try_preproc_directive(&mut self, host: &mut dyn ScannerHost, cur_state: ScanState) -> bool {
        if !self.at_line_start {
            return false;
        }
        if self.rest().first() != Some(&b'@') {
            return false;
        }
        // `@[^\n]*\n?`: the directive text up to (and including) the newline.
        let rest = self.rest();
        let line_end = rest.iter().position(|&c| c == b'\n');
        let (text, total) = match line_end {
            Some(i) => (&rest[..i], i + 1), // include the trailing \n in the match
            None => (rest, rest.len()),
        };
        let directive = text.to_vec();
        self.consume(total);
        host.next_line();
        let next = self.preprocess(host, &directive, cur_state, ScanState::Preproc);
        self.begin(next);
        true
    }

    /// Try the `\$\([a-zA-Z0-9_.][a-zA-Z0-9_.]*\)` macro-expansion rule.  On
    /// match, expands the macro by pushing its value as a new buffer
    /// (`preproc_macroexpand`, slghscan.l:373).  Returns `true` if matched.
    fn try_macro_expand(&mut self, host: &mut dyn ScannerHost) -> bool {
        let rest = self.rest();
        if rest.len() < 2 || rest[0] != b'$' || rest[1] != b'(' {
            return false;
        }
        // Match $( [macrochar]+ ) ; macrochar = [a-zA-Z0-9_.]
        let start = 2;
        let mut i = start;
        while i < rest.len() && is_macro_char(rest[i]) {
            i += 1;
        }
        if i == start {
            return false; // need at least one macro char (the `+` in the regex)
        }
        if i >= rest.len() || rest[i] != b')' {
            return false; // unterminated -> not a $(...) match; fall through
        }
        let name = rest[start..i].to_vec();
        let total = i + 1; // include the ')'
        self.consume(total);
        // preproc_macroexpand: push current buffer, switch to the macro value.
        let value = match host.get_preproc_value(&name) {
            Some(v) => v,
            None => {
                host.preproc_error(&format!(
                    "Unknown preprocessing macro {}",
                    String::from_utf8_lossy(&name)
                ));
            }
        };
        self.push_buffer(value, false);
        host.parse_preproc_macro();
        true
    }

    /// Push `contents` as the new current buffer, saving the current one on the
    /// include/macro stack (flex `sleigh_switch_to_buffer`).  `line_start` is the
    /// `^`-anchor state the new buffer begins in (macro expansion is mid-line, so
    /// `false`; an `@include` file begins at line start, `true`).
    fn push_buffer(&mut self, contents: Vec<u8>, line_start: bool) {
        let outer = std::mem::replace(&mut self.buffer, contents);
        let outer_pos = std::mem::replace(&mut self.pos, 0);
        let outer_ls = std::mem::replace(&mut self.at_line_start, line_start);
        self.filebuffers.push(FileStreamState {
            buffer: outer,
            pos: outer_pos,
            at_line_start: outer_ls,
        });
    }

    // -----------------------------------------------------------------------
    // Per-state scanners.  Each mirrors the flex rules in source order; the
    // longest-match is computed where rules overlap (numbers vs operators).
    // -----------------------------------------------------------------------

    /// `INITIAL` rules (slghscan.l:491-504).
    fn scan_initial(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::Initial) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        // [(),\-]  -> return the char (sleighlval.ch = c)
        if matches!(c, b'(' | b')' | b',' | b'-') {
            self.consume(1);
            return Some(char1(c));
        }
        if c == b':' {
            self.consume(1);
            self.begin(ScanState::Print);
            host.calc_context_layout();
            return Some(char1(c));
        }
        if c == b'{' {
            self.consume(1);
            self.begin(ScanState::Sem);
            return Some(char1(c));
        }
        if c == b'#' {
            // #.*  (comment to end of line, the \n is left for the \n rule)
            self.consume_comment();
            return None;
        }
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return None;
        }
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return None;
        }
        // Keyword/identifier rules: macro/define/attach/with then identifier.
        if let Some((kw, len)) = match_keyword_initial(rest) {
            self.consume(len);
            match kw {
                InitKw::Macro => {
                    self.begin(ScanState::MacroBlock);
                    return Some(Token::bare(TokenKind::MacroKey));
                }
                InitKw::Define => {
                    self.begin(ScanState::DefBlock);
                    return Some(Token::bare(TokenKind::DefineKey));
                }
                InitKw::Attach => {
                    self.begin(ScanState::DefBlock);
                    host.calc_context_layout();
                    return Some(Token::bare(TokenKind::AttachKey));
                }
                InitKw::With => {
                    self.begin(ScanState::Pattern);
                    self.withsection = true;
                    host.calc_context_layout();
                    return Some(Token::bare(TokenKind::WithKey));
                }
            }
        }
        // [a-zA-Z_.][a-zA-Z0-9_.]*  -> find_symbol()
        if let Some(len) = match_identifier(rest) {
            let name = rest[..len].to_vec();
            self.consume(len);
            return Some(self.find_symbol(host, name));
        }
        // .  -> return the char (no sleighlval set in INITIAL's `.`)
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `macroblock` rules (slghscan.l:506-513).
    fn scan_macroblock(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::MacroBlock) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        if matches!(c, b'(' | b')' | b',') {
            self.consume(1);
            return Some(char1(c));
        }
        if c == b'{' {
            self.consume(1);
            self.begin(ScanState::Sem);
            return Some(char1_no_val(c));
        }
        if let Some(len) = match_identifier(rest) {
            // [a-zA-Z_.][a-zA-Z0-9_.]*  -> STRING (NOT find_symbol)
            let name = rest[..len].to_vec();
            self.consume(len);
            return Some(Token { kind: TokenKind::String, value: TokenValue::Str(name) });
        }
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return None;
        }
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return None;
        }
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `defblock` rules (slghscan.l:515-550).
    fn scan_defblock(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::DefBlock) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        if matches!(c, b'(' | b')' | b',' | b'=' | b':' | b'[' | b']') {
            self.consume(1);
            return Some(char1(c));
        }
        if c == b';' {
            self.consume(1);
            self.begin(ScanState::Initial);
            return Some(char1(c));
        }
        if c == b'#' {
            self.consume_comment();
            return None;
        }
        // keyword table (defblock keywords, slghscan.l:519-541)
        if let Some((kind, len)) = match_keyword_defblock(rest) {
            self.consume(len);
            return Some(Token::bare(kind));
        }
        // Capture the identifier match now (owned) so the &mut number/string
        // scans below don't conflict with the `rest` borrow.
        let ident = match_identifier(rest).map(|len| rest[..len].to_vec());
        // numbers (only when the identifier rule cannot apply: digits never
        // start an identifier).  Tried after keywords, before the identifier.
        if let Some(tok) = self.try_number(/*signed=*/ false) {
            return Some(tok);
        }
        // string literal  \"([^\"[:cntrl:]]|\"\")*\"
        if let Some(tok) = self.try_string_literal() {
            return Some(tok);
        }
        if let Some(name) = ident {
            self.consume(name.len());
            return Some(self.find_symbol(host, name));
        }
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return None;
        }
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return None;
        }
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `print` rules (slghscan.l:553-562).
    fn scan_print(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::Print) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        // The "is" keyword (flex lists `is` before the SYMBOLSTRING identifier
        // rule; on the 2-char "is" token both match length 2 and rule order
        // picks `is`).  `match_word` ensures it is a *whole* identifier, not a
        // prefix of e.g. `island`.
        if let Some(len) = match_word(rest, b"is") {
            self.consume(len);
            self.begin(ScanState::Pattern);
            self.actionon = false;
            return Some(Token::bare(TokenKind::IsKey));
        }
        // [~!@#$%&*()\-=+\[\]{}|;:<>?,/0-9]  -> CHAR (sleighlval.ch = c)
        if is_print_char(c) {
            self.consume(1);
            return Some(Token { kind: TokenKind::Char, value: TokenValue::Char(c) });
        }
        // \^  -> '^' (returns the char '^', sleighlval.ch='^')
        if c == b'^' {
            self.consume(1);
            return Some(Token { kind: TokenKind::Char1(b'^'), value: TokenValue::Char(b'^') });
        }
        // [a-zA-Z_.][a-zA-Z0-9_.]*  -> SYMBOLSTRING
        if let Some(len) = match_identifier(rest) {
            let name = rest[..len].to_vec();
            self.consume(len);
            return Some(Token { kind: TokenKind::SymbolString, value: TokenValue::Str(name) });
        }
        // \"([^\"[:cntrl:]]|\"\")*\"  -> STRING
        if let Some(tok) = self.try_string_literal() {
            return Some(tok);
        }
        // [\r \t\v]+  -> ' ' (a single space token, sleighlval.ch=' ')
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return Some(Token { kind: TokenKind::Char1(b' '), value: TokenValue::Char(b' ') });
        }
        // \n  -> nextLine() + ' '
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return Some(Token { kind: TokenKind::Char1(b' '), value: TokenValue::Char(b' ') });
        }
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `pattern` rules (slghscan.l:564-591).
    fn scan_pattern(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::Pattern) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        if c == b'{' {
            self.consume(1);
            let next = if self.withsection { ScanState::Initial } else { ScanState::Sem };
            self.begin(next);
            self.withsection = false;
            return Some(char1(c));
        }
        // word keywords: unimpl, globalset
        if let Some(len) = match_word(rest, b"unimpl") {
            self.consume(len);
            self.begin(ScanState::Initial);
            return Some(Token::bare(TokenKind::OpUnimpl));
        }
        if let Some(len) = match_word(rest, b"globalset") {
            self.consume(len);
            return Some(Token::bare(TokenKind::GlobalsetKey));
        }
        // multi-char operators (must beat single chars by longest match)
        if rest.starts_with(b">>") { self.consume(2); return Some(Token::bare(TokenKind::OpRight)); }
        if rest.starts_with(b"<<") { self.consume(2); return Some(Token::bare(TokenKind::OpLeft)); }
        if rest.starts_with(b"!=") { self.consume(2); return Some(Token::bare(TokenKind::OpNotEqual)); }
        if rest.starts_with(b"<=") { self.consume(2); return Some(Token::bare(TokenKind::OpLessEqual)); }
        if rest.starts_with(b">=") { self.consume(2); return Some(Token::bare(TokenKind::OpGreatEqual)); }
        // $and $or $xor (the dollar-prefixed action operators)
        if rest.starts_with(b"$and") { self.consume(4); return Some(Token::bare(TokenKind::OpAnd)); }
        if rest.starts_with(b"$or") { self.consume(3); return Some(Token::bare(TokenKind::OpOr)); }
        if rest.starts_with(b"$xor") { self.consume(4); return Some(Token::bare(TokenKind::OpXor)); }
        // ...  (ellipsis)
        if rest.starts_with(b"...") { self.consume(3); return Some(Token::bare(TokenKind::EllipsisKey)); }
        if c == b'[' {
            self.consume(1);
            self.actionon = true;
            return Some(char1(c));
        }
        if c == b']' {
            self.consume(1);
            self.actionon = false;
            return Some(char1(c));
        }
        if c == b'&' {
            self.consume(1);
            return Some(if self.actionon {
                Token { kind: TokenKind::OpAnd, value: TokenValue::Char(c) }
            } else {
                char1(c)
            });
        }
        if c == b'|' {
            self.consume(1);
            return Some(if self.actionon {
                Token { kind: TokenKind::OpOr, value: TokenValue::Char(c) }
            } else {
                char1(c)
            });
        }
        if c == b'^' {
            self.consume(1);
            return Some(Token::bare(TokenKind::OpXor));
        }
        // [=(),:;+\-*/~<>]  -> char
        if matches!(c, b'=' | b'(' | b')' | b',' | b':' | b';' | b'+' | b'-' | b'*' | b'/' | b'~' | b'<' | b'>') {
            self.consume(1);
            return Some(char1(c));
        }
        if c == b'#' {
            self.consume_comment();
            return None;
        }
        if let Some(len) = match_identifier(rest) {
            let name = rest[..len].to_vec();
            self.consume(len);
            return Some(self.find_symbol(host, name));
        }
        if let Some(tok) = self.try_number(/*signed=*/ true) {
            return Some(tok);
        }
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return None;
        }
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return None;
        }
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `sem` rules (slghscan.l:593-658).
    fn scan_sem(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        if self.try_preproc_directive(host, ScanState::Sem) {
            return None;
        }
        if self.try_macro_expand(host) {
            return None;
        }
        let rest = self.rest();
        let c = rest[0];
        if c == b'}' {
            self.consume(1);
            self.begin(ScanState::Initial);
            return Some(char1(c));
        }
        // 2/3-char operators and s-/f-prefixed operators, then word keywords.
        if let Some((kind, len)) = match_sem_operator(rest) {
            self.consume(len);
            return Some(Token::bare(kind));
        }
        if let Some((kind, len)) = match_keyword_sem(rest) {
            self.consume(len);
            return Some(Token::bare(kind));
        }
        // [=(),:\[\];!&|^+\-*/%~<>]  -> char
        if matches!(c,
            b'=' | b'(' | b')' | b',' | b':' | b'[' | b']' | b';' | b'!' | b'&' | b'|'
            | b'^' | b'+' | b'-' | b'*' | b'/' | b'%' | b'~' | b'<' | b'>'
        ) {
            self.consume(1);
            return Some(char1(c));
        }
        if c == b'#' {
            self.consume_comment();
            return None;
        }
        if let Some(len) = match_identifier(rest) {
            let name = rest[..len].to_vec();
            self.consume(len);
            return Some(self.find_symbol(host, name));
        }
        if let Some(tok) = self.try_number(/*signed=*/ false) {
            return Some(tok);
        }
        if matches!(c, b'\r' | b' ' | b'\t' | 0x0b) {
            self.consume_hspace();
            return None;
        }
        if c == b'\n' {
            self.consume(1);
            host.next_line();
            return None;
        }
        self.consume(1);
        Some(char1_no_val(c))
    }

    /// `preproc` erasure rules (slghscan.l:660-661): consume whole lines until a
    /// directive (`^@`) flips the state back.
    fn scan_preproc(&mut self, host: &mut dyn ScannerHost) -> Option<Token> {
        // <preproc>^@.*\n?   -> nextLine(); BEGIN(preprocess(preproc,preproc))
        if self.at_line_start && self.rest().first() == Some(&b'@') {
            // `@.*\n?` : the `.` here is flex's (no-newline) dot, so up to \n.
            let rest = self.rest();
            let line_end = rest.iter().position(|&c| c == b'\n');
            let (text, total) = match line_end {
                Some(i) => (&rest[..i], i + 1),
                None => (rest, rest.len()),
            };
            let directive = text.to_vec();
            self.consume(total);
            host.next_line();
            let next = self.preprocess(host, &directive, ScanState::Preproc, ScanState::Preproc);
            self.begin(next);
            return None;
        }
        // <preproc>^.*\n   -> nextLine()  (erase a whole line)
        // The `^.*\n` requires a terminating newline; a final line without a
        // trailing \n is matched by flex's default rule (skip) -- in practice the
        // erased region is always inside a file that continues, so we consume to
        // the newline (inclusive) or to EOF (guarding against an infinite loop).
        let rest = self.rest();
        match rest.iter().position(|&c| c == b'\n') {
            Some(i) => {
                self.consume(i + 1);
                host.next_line();
            }
            None => {
                let n = rest.len();
                self.consume(n);
            }
        }
        None
    }

    // --- helpers shared by the state scanners ---

    /// Consume a `#.*` comment (to but not including the newline).
    fn consume_comment(&mut self) {
        let rest = self.rest();
        let n = rest.iter().position(|&c| c == b'\n').unwrap_or(rest.len());
        self.consume(n);
    }

    /// Consume a run of horizontal whitespace `[\r \t\v]+`.
    fn consume_hspace(&mut self) {
        let rest = self.rest();
        let mut n = 0;
        while n < rest.len() && matches!(rest[n], b'\r' | b' ' | b'\t' | 0x0b) {
            n += 1;
        }
        self.consume(n);
    }

    /// Try the number rules:
    ///   `0x[0-9a-fA-F]+`  (hex), `0b[01]+` (binary), `[0-9]+` (decimal),
    /// honoring flex longest-match (a `0x`/`0b` with no following digit loses to
    /// `[0-9]+` matching just the `0`).  `signed_num` selects INTB vs INTEGER
    /// (the pattern state uses signed; defblock/sem use unsigned).
    fn try_number(&mut self, signed_num: bool) -> Option<Token> {
        let rest = &self.buffer[self.pos..];
        if rest.is_empty() || !rest[0].is_ascii_digit() {
            return None;
        }
        // Candidate longest matches.
        let dec_len = count_while(rest, |c| c.is_ascii_digit());
        // hex: 0x then >=1 hex digit
        let hex_len = if rest.len() >= 3 && rest[0] == b'0' && rest[1] == b'x' {
            let n = count_while(&rest[2..], |c| c.is_ascii_hexdigit());
            if n > 0 { Some(2 + n) } else { None }
        } else {
            None
        };
        // bin: 0b then >=1 binary digit
        let bin_len = if rest.len() >= 3 && rest[0] == b'0' && rest[1] == b'b' {
            let n = count_while(&rest[2..], |c| c == b'0' || c == b'1');
            if n > 0 { Some(2 + n) } else { None }
        } else {
            None
        };
        // flex longest-match, ties by source order (hex, bin, dec listed in that
        // order in each state).  hex/bin always start `0x`/`0b` so they only tie
        // with dec at length 1 (the `0`), where they are longer when present.
        let (radix, len, digit_start) = match (hex_len, bin_len) {
            (Some(h), _) if h >= dec_len => (16u32, h, 2usize),
            (_, Some(b)) if b >= dec_len => (2u32, b, 2usize),
            _ => (10u32, dec_len, 0usize),
        };
        let digits = rest[digit_start..len].to_vec();
        self.consume(len);
        // scan_number: std::stoul(digits, 0, radix) -> BADINTEGER on overflow/err
        match parse_radix_u64(&digits, radix) {
            Some(v) => {
                if signed_num {
                    Some(Token { kind: TokenKind::Intb, value: TokenValue::Intb(v as i64) })
                } else {
                    Some(Token { kind: TokenKind::Integer, value: TokenValue::Integer(v) })
                }
            }
            None => Some(Token::bare(TokenKind::BadInteger)),
        }
    }

    /// Try the string-literal rule `\"([^\"[:cntrl:]]|\"\")*\"`.  Returns a
    /// `STRING` token with the inner text.  Note the C++ stores
    /// `string(sleightext+1, len-2)` -- it keeps any doubled `""` verbatim
    /// (no un-escaping), so we copy the raw inner bytes.
    fn try_string_literal(&mut self) -> Option<Token> {
        let rest = &self.buffer[self.pos..];
        if rest.first() != Some(&b'"') {
            return None;
        }
        // Scan the body: any non-control non-quote, OR a doubled quote "".
        let mut i = 1;
        loop {
            if i >= rest.len() {
                return None; // unterminated: flex would not match this rule
            }
            let ch = rest[i];
            if ch == b'"' {
                if i + 1 < rest.len() && rest[i + 1] == b'"' {
                    i += 2; // "" stays inside the string
                    continue;
                }
                i += 1; // closing quote
                break;
            }
            if is_cntrl(ch) {
                return None; // [:cntrl:] excluded from the body; rule fails
            }
            i += 1;
        }
        // raw inner text = sleightext+1 .. sleightext+len-1 (verbatim, "" kept)
        let raw = rest[1..i - 1].to_vec();
        let total = i;
        self.consume(total);
        Some(Token { kind: TokenKind::String, value: TokenValue::Str(raw) })
    }

    /// `find_symbol()` (slghscan.l:389): an identifier resolves to its symbol's
    /// kind token, or `STRING` if unknown.
    fn find_symbol(&self, host: &mut dyn ScannerHost, name: Vec<u8>) -> Token {
        match host.find_symbol_kind(&name) {
            Some(kind) => Token {
                kind: kind.token_kind(),
                value: TokenValue::SymbolName(name),
            },
            None => Token { kind: TokenKind::String, value: TokenValue::Str(name) },
        }
    }

    // -----------------------------------------------------------------------
    // Preprocessor layer (slghscan.l:48-371).
    // -----------------------------------------------------------------------

    /// Port of `preprocess(cur_state, blank_state)` (slghscan.l:232): handle one
    /// `@`-directive (`directive` is the line text *without* the trailing `\n`),
    /// returning the start-condition to resume in.
    fn preprocess(
        &mut self,
        host: &mut dyn ScannerHost,
        directive: &[u8],
        cur_state: ScanState,
        blank_state: ScanState,
    ) -> ScanState {
        // strip a trailing `#` comment.
        let mut line: &[u8] = directive;
        if let Some(p) = line.iter().position(|&c| c == b'#') {
            line = &line[..p];
        }
        let mut s = ByteStream::new(line);

        if cur_state != blank_state {
            self.last_preproc = cur_state;
        }

        s.get(); // skip the preprocessor marker '@'
        let dtype = s.read_word();

        match dtype.as_slice() {
            b"include" => {
                if self.negative_if == -1 {
                    s.skip_ws();
                    let fname_raw = preprocess_string(host, &mut s);
                    let fname = expand_preprocmacros(host, &fname_raw);
                    match host.read_include(&fname) {
                        Some(contents) => self.push_buffer(contents, true),
                        None => host.preproc_error(&format!(
                            "Could not open included file {}",
                            String::from_utf8_lossy(&fname)
                        )),
                    }
                    check_to_endofline(host, &mut s);
                }
            }
            b"define" => {
                if self.negative_if == -1 {
                    let varname = s.read_identifier();
                    s.skip_ws();
                    let value = if s.peek() == Some(b'"') {
                        preprocess_string(host, &mut s)
                    } else {
                        s.read_identifier()
                    };
                    if varname.is_empty() {
                        host.preproc_error("Error in preprocessor definition");
                    }
                    host.set_preproc_value(&varname, &value);
                    check_to_endofline(host, &mut s);
                }
            }
            b"undef" => {
                if self.negative_if == -1 {
                    let varname = s.read_identifier();
                    if varname.is_empty() {
                        host.preproc_error("Error in preprocessor undef");
                    }
                    host.undefine_preproc_value(&varname);
                    check_to_endofline(host, &mut s);
                }
            }
            b"ifdef" => {
                let varname = s.read_identifier();
                if varname.is_empty() {
                    host.preproc_error("Error in preprocessor ifdef");
                }
                let truth = i32::from(host.get_preproc_value(&varname).is_some());
                self.ifstack.push(truth);
                check_to_endofline(host, &mut s);
            }
            b"ifndef" => {
                let varname = s.read_identifier();
                if varname.is_empty() {
                    host.preproc_error("Error in preprocessor ifndef");
                }
                let truth = i32::from(host.get_preproc_value(&varname).is_none());
                self.ifstack.push(truth);
                check_to_endofline(host, &mut s);
            }
            b"if" => {
                let truth = preprocess_if(host, &mut s);
                if !s.eof() {
                    host.preproc_error("Unbalanced parentheses");
                }
                self.ifstack.push(truth);
            }
            b"elif" => {
                if self.ifstack.is_empty() {
                    host.preproc_error("elif without preceding if");
                }
                let top = *self.ifstack.last().unwrap();
                if (top & 2) != 0 {
                    host.preproc_error("elif follows else");
                }
                // slghscan.l:323-326 has two separate arms both setting `= 4`
                // ((top&4): already a true elif; (top&1): last clause was a true
                // if).  Collapsed here -- identical effect, identical for parity.
                if (top & 4) != 0 || (top & 1) != 0 {
                    *self.ifstack.last_mut().unwrap() = 4;
                } else {
                    let truth = preprocess_if(host, &mut s);
                    if !s.eof() {
                        host.preproc_error("Unbalanced parentheses");
                    }
                    *self.ifstack.last_mut().unwrap() = if truth == 0 { 0 } else { 5 };
                }
            }
            b"endif" => {
                if self.ifstack.is_empty() {
                    host.preproc_error("preprocessing endif without matching if");
                }
                self.ifstack.pop();
                check_to_endofline(host, &mut s);
            }
            b"else" => {
                if self.ifstack.is_empty() {
                    host.preproc_error("preprocessing else without matching if");
                }
                let top = *self.ifstack.last().unwrap();
                if (top & 2) != 0 {
                    host.preproc_error("second else for one if");
                }
                if (top & 4) != 0 {
                    *self.ifstack.last_mut().unwrap() = 6;
                } else if top == 0 {
                    *self.ifstack.last_mut().unwrap() = 3;
                } else {
                    *self.ifstack.last_mut().unwrap() = 2;
                }
                check_to_endofline(host, &mut s);
            }
            other => {
                host.preproc_error(&format!(
                    "Unknown preprocessing directive: {}",
                    String::from_utf8_lossy(other)
                ));
            }
        }

        // Tail logic (slghscan.l:359-370): recompute the resume state.
        if self.negative_if >= 0 {
            if (self.negative_if as usize) + 1 < self.ifstack.len() {
                return blank_state;
            } else {
                self.negative_if = -1;
            }
        }
        if self.ifstack.is_empty() {
            return self.last_preproc;
        }
        if (*self.ifstack.last().unwrap() & 1) == 0 {
            self.negative_if = (self.ifstack.len() - 1) as i32;
            return blank_state;
        }
        self.last_preproc
    }
}

impl Default for SleighScanner {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Free helpers (the static functions and character classes of slghscan.l).
// ---------------------------------------------------------------------------

/// A single-char token returned verbatim, carrying its char in `sleighlval.ch`
/// (the flex rules that do `sleighlval.ch = sleightext[0]; return sleightext[0]`).
fn char1(c: u8) -> Token {
    Token { kind: TokenKind::Char1(c), value: TokenValue::Char(c) }
}

/// A single-char token returned by a `.` rule that does NOT set `sleighlval`
/// (the bare `. { return sleightext[0]; }` rules).
fn char1_no_val(c: u8) -> Token {
    Token { kind: TokenKind::Char1(c), value: TokenValue::None }
}

/// flex identifier-start char class `[a-zA-Z_.]`.
fn is_ident_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c == b'.'
}
/// flex identifier-continue char class `[a-zA-Z0-9_.]`.
fn is_ident_cont(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'.'
}
/// The `$(...)` macro-name char class `[a-zA-Z0-9_.]`.
fn is_macro_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'.'
}
/// C `iscntrl`: control chars (the `[:cntrl:]` excluded from string bodies).
fn is_cntrl(c: u8) -> bool {
    c < 0x20 || c == 0x7f
}
/// The `print`-state CHAR class `[~!@#$%&*()\-=+\[\]{}|;:<>?,/0-9]`.
fn is_print_char(c: u8) -> bool {
    matches!(
        c,
        b'~' | b'!' | b'@' | b'#' | b'$' | b'%' | b'&' | b'*' | b'(' | b')' | b'-' | b'='
            | b'+' | b'[' | b']' | b'{' | b'}' | b'|' | b';' | b':' | b'<' | b'>' | b'?'
            | b',' | b'/' | b'0'..=b'9'
    )
}

/// Count the leading bytes of `s` satisfying `pred`.
fn count_while(s: &[u8], pred: impl Fn(u8) -> bool) -> usize {
    s.iter().take_while(|&&c| pred(c)).count()
}

/// Length of a `[a-zA-Z_.][a-zA-Z0-9_.]*` identifier at the start of `rest`, or
/// `None` if it doesn't start with an identifier char.
fn match_identifier(rest: &[u8]) -> Option<usize> {
    if rest.is_empty() || !is_ident_start(rest[0]) {
        return None;
    }
    let mut i = 1;
    while i < rest.len() && is_ident_cont(rest[i]) {
        i += 1;
    }
    Some(i)
}

/// Match the bareword `word` at the start of `rest` *as a maximal identifier*
/// (flex would not let `is` match the prefix of `island`).  Returns the match
/// length only if the identifier token equals `word` exactly.
fn match_word(rest: &[u8], word: &[u8]) -> Option<usize> {
    let len = match_identifier(rest)?;
    if &rest[..len] == word {
        Some(len)
    } else {
        None
    }
}

/// `std::stoul(digits, 0, radix)` semantics: parse the whole `digits` slice in
/// the given radix, returning `None` on empty/invalid/overflow (-> BADINTEGER).
/// Note `std::stoul` returns `unsigned long` (64-bit here); it requires every
/// char be a valid digit for the radix and the value fit in 64 bits.
fn parse_radix_u64(digits: &[u8], radix: u32) -> Option<u64> {
    if digits.is_empty() {
        return None;
    }
    let mut val: u64 = 0;
    for &b in digits {
        let d = (b as char).to_digit(radix)?;
        val = val.checked_mul(radix as u64)?.checked_add(d as u64)?;
    }
    Some(val)
}

/// INITIAL-state keyword set (slghscan.l:499-502): the only keywords are
/// `macro`/`define`/`attach`/`with`; everything else goes through `find_symbol`.
enum InitKw {
    Macro,
    Define,
    Attach,
    With,
}

fn match_keyword_initial(rest: &[u8]) -> Option<(InitKw, usize)> {
    if let Some(l) = match_word(rest, b"macro") {
        return Some((InitKw::Macro, l));
    }
    if let Some(l) = match_word(rest, b"define") {
        return Some((InitKw::Define, l));
    }
    if let Some(l) = match_word(rest, b"attach") {
        return Some((InitKw::Attach, l));
    }
    if let Some(l) = match_word(rest, b"with") {
        return Some((InitKw::With, l));
    }
    None
}

/// defblock keyword table (slghscan.l:519-541).
fn match_keyword_defblock(rest: &[u8]) -> Option<(TokenKind, usize)> {
    const TABLE: &[(&[u8], TokenKind)] = &[
        (b"space", TokenKind::SpaceKey),
        (b"type", TokenKind::TypeKey),
        (b"ram_space", TokenKind::RamKey),
        (b"default", TokenKind::DefaultKey),
        (b"register_space", TokenKind::RegisterKey),
        (b"token", TokenKind::TokenKey),
        (b"context", TokenKind::ContextKey),
        (b"bitrange", TokenKind::BitrangeKey),
        (b"signed", TokenKind::SignedKey),
        (b"noflow", TokenKind::NoflowKey),
        (b"hex", TokenKind::HexKey),
        (b"dec", TokenKind::DecKey),
        (b"endian", TokenKind::EndianKey),
        (b"alignment", TokenKind::AlignKey),
        (b"big", TokenKind::BigKey),
        (b"little", TokenKind::LittleKey),
        (b"size", TokenKind::SizeKey),
        (b"wordsize", TokenKind::WordsizeKey),
        (b"offset", TokenKind::OffsetKey),
        (b"names", TokenKind::NamesKey),
        (b"values", TokenKind::ValuesKey),
        (b"variables", TokenKind::VariablesKey),
        (b"pcodeop", TokenKind::PcodeopKey),
    ];
    let len = match_identifier(rest)?;
    let word = &rest[..len];
    for &(kw, kind) in TABLE {
        if word == kw {
            return Some((kind, len));
        }
    }
    None
}

/// sem-state word keywords (slghscan.l:622-649): the named p-code intrinsics and
/// statement keywords.  (The operator forms are handled by `match_sem_operator`.)
fn match_keyword_sem(rest: &[u8]) -> Option<(TokenKind, usize)> {
    const TABLE: &[(&[u8], TokenKind)] = &[
        (b"zext", TokenKind::OpZext),
        (b"carry", TokenKind::OpCarry),
        (b"borrow", TokenKind::OpBorrow),
        (b"sext", TokenKind::OpSext),
        (b"scarry", TokenKind::OpScarry),
        (b"sborrow", TokenKind::OpSborrow),
        (b"nan", TokenKind::OpNan),
        (b"abs", TokenKind::OpAbs),
        (b"sqrt", TokenKind::OpSqrt),
        (b"ceil", TokenKind::OpCeil),
        (b"floor", TokenKind::OpFloor),
        (b"round", TokenKind::OpRound),
        (b"int2float", TokenKind::OpInt2Float),
        (b"float2float", TokenKind::OpFloat2Float),
        (b"trunc", TokenKind::OpTrunc),
        (b"cpool", TokenKind::OpCpoolref),
        (b"newobject", TokenKind::OpNew),
        (b"popcount", TokenKind::OpPopcount),
        (b"lzcount", TokenKind::OpLzcount),
        (b"if", TokenKind::IfKey),
        (b"goto", TokenKind::GotoKey),
        (b"call", TokenKind::CallKey),
        (b"return", TokenKind::ReturnKey),
        (b"delayslot", TokenKind::DelayslotKey),
        (b"crossbuild", TokenKind::CrossbuildKey),
        (b"export", TokenKind::ExportKey),
        (b"build", TokenKind::BuildKey),
        (b"local", TokenKind::LocalKey),
    ];
    let len = match_identifier(rest)?;
    let word = &rest[..len];
    for &(kw, kind) in TABLE {
        if word == kw {
            return Some((kind, len));
        }
    }
    None
}

/// sem-state multi-character operators (slghscan.l:596-621).  Ordered so the
/// longest forms win (`s>>`/`s<=`/`s>=`/`f==`/`f!=`/`f<=`/`f>=` before the 2-char
/// forms).  These are flex rules with explicit literal text, so the match is the
/// literal bytes (no maximal-munch identifier rule applies -- `s`/`f` here are
/// the operator-prefix characters of the rule, e.g. `s\/`).
fn match_sem_operator(rest: &[u8]) -> Option<(TokenKind, usize)> {
    // 3-char forms first.
    const THREE: &[(&[u8], TokenKind)] = &[
        (b"s>>", TokenKind::OpSright),
        (b"s<=", TokenKind::OpSlessEqual),
        (b"s>=", TokenKind::OpSgreatEqual),
        (b"f==", TokenKind::OpFEqual),
        (b"f!=", TokenKind::OpFNotEqual),
        (b"f<=", TokenKind::OpFlessEqual),
        (b"f>=", TokenKind::OpFgreatEqual),
    ];
    for &(op, kind) in THREE {
        if rest.starts_with(op) {
            return Some((kind, op.len()));
        }
    }
    const TWO: &[(&[u8], TokenKind)] = &[
        (b"||", TokenKind::OpBoolOr),
        (b"&&", TokenKind::OpBoolAnd),
        (b"^^", TokenKind::OpBoolXor),
        (b">>", TokenKind::OpRight),
        (b"<<", TokenKind::OpLeft),
        (b"==", TokenKind::OpEqual),
        (b"!=", TokenKind::OpNotEqual),
        (b"<=", TokenKind::OpLessEqual),
        (b">=", TokenKind::OpGreatEqual),
        (b"s/", TokenKind::OpSdiv),
        (b"s%", TokenKind::OpSrem),
        (b"s<", TokenKind::OpSless),
        (b"s>", TokenKind::OpSgreat),
        (b"f+", TokenKind::OpFadd),
        (b"f-", TokenKind::OpFsub),
        (b"f*", TokenKind::OpFmult),
        (b"f/", TokenKind::OpFdiv),
        (b"f<", TokenKind::OpFless),
        (b"f>", TokenKind::OpFgreat),
    ];
    for &(op, kind) in TWO {
        if rest.starts_with(op) {
            return Some((kind, op.len()));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Preprocessor stream helpers (the `istream &s` functions of slghscan.l).
// ---------------------------------------------------------------------------

/// A minimal `istream`-like cursor over a byte slice, supporting the operations
/// the preprocessor functions use (`get`, `peek`, `>> ws`, `>> word`, eof).
struct ByteStream<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteStream<'a> {
    fn new(buf: &'a [u8]) -> ByteStream<'a> {
        ByteStream { buf, pos: 0 }
    }
    /// `s.eof()`: true once all bytes are consumed.
    fn eof(&self) -> bool {
        self.pos >= self.buf.len()
    }
    /// `s.peek()`: the next byte without consuming (or `None` at eof).
    fn peek(&self) -> Option<u8> {
        self.buf.get(self.pos).copied()
    }
    /// `s.get()`: consume and return the next byte (or `None`/-1 at eof).
    fn get(&mut self) -> Option<u8> {
        let c = self.buf.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }
    /// `s >> ws`: skip whitespace (C `isspace`: space/tab/nl/vt/ff/cr).
    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
    /// `s >> word`: skip leading whitespace, then read up to the next
    /// whitespace (the C++ `istream >> string`).
    fn read_word(&mut self) -> Vec<u8> {
        self.skip_ws();
        let mut out = Vec::new();
        while let Some(c) = self.peek() {
            if matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
                break;
            }
            out.push(c);
            self.pos += 1;
        }
        out
    }
    /// `read_identifier(s)` (slghscan.l:65): skip ws, then read `[A-Za-z0-9_]*`.
    fn read_identifier(&mut self) -> Vec<u8> {
        self.skip_ws();
        let mut out = Vec::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                out.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        out
    }
}

/// `check_to_endofline(s)` (slghscan.l:56): after `>> ws`, the rest must be empty
/// or start a `#` comment.
fn check_to_endofline(host: &mut dyn ScannerHost, s: &mut ByteStream) {
    s.skip_ws();
    if !s.eof() && s.peek() != Some(b'#') {
        host.preproc_error("Extra characters in preprocessor directive");
    }
}

/// `preprocess_string(s)` (slghscan.l:82): read a double-quoted string.
fn preprocess_string(host: &mut dyn ScannerHost, s: &mut ByteStream) -> Vec<u8> {
    s.skip_ws();
    if s.get() != Some(b'"') {
        host.preproc_error("Expecting double quoted string");
    }
    let mut res = Vec::new();
    loop {
        match s.get() {
            Some(b'"') => return res,
            Some(c) => res.push(c),
            None => host.preproc_error("Missing terminating double quote"),
        }
    }
}

/// `read_defined_operator(s)` (slghscan.l:102): `defined ( NAME )`, 1 if NAME is
/// a defined macro.
fn read_defined_operator(host: &mut dyn ScannerHost, s: &mut ByteStream) -> i32 {
    s.skip_ws();
    if s.get() != Some(b'(') {
        host.preproc_error("Badly formed \"defined\" operator");
    }
    let macroname = s.read_identifier();
    let res = i32::from(host.get_preproc_value(&macroname).is_some());
    s.skip_ws();
    if s.get() != Some(b')') {
        host.preproc_error("Badly formed \"defined\" operator");
    }
    res
}

/// `read_boolean_clause(s)` (slghscan.l:120): a parenthetical or a `lhs CMP rhs`
/// comparison of macro values / string literals.
fn read_boolean_clause(host: &mut dyn ScannerHost, s: &mut ByteStream) -> i32 {
    s.skip_ws();
    if s.peek() == Some(b'(') {
        s.get();
        let res = preprocess_if(host, s);
        s.skip_ws();
        if s.get() != Some(b')') {
            host.preproc_error("Unbalanced parentheses");
        }
        return res;
    }
    // lhs
    let lhs: Vec<u8> = if s.peek() == Some(b'"') {
        preprocess_string(host, s)
    } else {
        let id = s.read_identifier();
        if id == b"defined" {
            return read_defined_operator(host, s);
        }
        match host.get_preproc_value(&id) {
            Some(v) => v,
            None => host.preproc_error(&format!(
                "Could not find preprocessor macro {}",
                String::from_utf8_lossy(&id)
            )),
        }
    };
    // comparison operator: two chars via `s >> tok` (whitespace-skipping).
    let comp = [read_token_char(s), read_token_char(s)];
    // rhs
    s.skip_ws();
    let rhs: Vec<u8> = if s.peek() == Some(b'"') {
        preprocess_string(host, s)
    } else {
        let id = s.read_identifier();
        match host.get_preproc_value(&id) {
            Some(v) => v,
            None => host.preproc_error(&format!(
                "Could not find preprocessor macro {}",
                String::from_utf8_lossy(&id)
            )),
        }
    };
    match &comp {
        b"==" => i32::from(lhs == rhs),
        b"!=" => i32::from(lhs != rhs),
        _ => host.preproc_error("Syntax error in condition"),
    }
}

/// `preprocess_if(s)` (slghscan.l:171): a boolean clause possibly joined by
/// `&&`/`||`/`^^` operators, up to `)` or eof.
fn preprocess_if(host: &mut dyn ScannerHost, s: &mut ByteStream) -> i32 {
    let mut res = read_boolean_clause(host, s);
    s.skip_ws();
    while !s.eof() && s.peek() != Some(b')') {
        let boolop = [read_token_char(s), read_token_char(s)];
        let res2 = read_boolean_clause(host, s);
        match &boolop {
            b"&&" => res &= res2,
            b"||" => res |= res2,
            b"^^" => res ^= res2,
            _ => host.preproc_error("Syntax error in expression"),
        }
        s.skip_ws();
    }
    res
}

/// `s >> tok` for a single non-whitespace char (skips leading whitespace).
/// Used by the boolean-operator reads (`s >> tok; comp += tok;`).
fn read_token_char(s: &mut ByteStream) -> u8 {
    s.skip_ws();
    s.get().unwrap_or(b'\0')
}

/// `expand_preprocmacros(str)` (slghscan.l:197): replace every `$(NAME)` in
/// `str` with the macro value.  Returns the expanded string.
fn expand_preprocmacros(host: &mut dyn ScannerHost, str_: &[u8]) -> Vec<u8> {
    if find_subslice(str_, b"$(").is_none() {
        return str_.to_vec();
    }
    let mut res = Vec::new();
    let mut lastpos = 0usize;
    let mut search_from = 0usize;
    loop {
        match find_subslice(&str_[search_from..], b"$(") {
            None => {
                res.extend_from_slice(&str_[lastpos..]);
                return res;
            }
            Some(rel) => {
                let pos = search_from + rel;
                res.extend_from_slice(&str_[lastpos..pos]);
                let after = pos + 2;
                let endpos = match find_subslice(&str_[after..], b")") {
                    Some(e) => after + e,
                    None => host.preproc_error("Unterminated macro in string"),
                };
                let macro_name = &str_[after..endpos];
                match host.get_preproc_value(macro_name) {
                    Some(v) => res.extend_from_slice(&v),
                    None => host.preproc_error(&format!(
                        "Unknown preprocessing macro {}",
                        String::from_utf8_lossy(macro_name)
                    )),
                }
                lastpos = endpos + 1;
                search_from = lastpos;
            }
        }
    }
}

/// `string::find(needle, from)` for byte slices.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}
