//! Port of `decompiler/cpp/grammar.{hh,cc}` (generated from `grammar.y`) — the
//! C-declaration parser behind the console `parse line` / `parse file` commands
//! and the `parse_protopieces` / `parse_type` / `parse_C` entry points that the
//! `testfuncproto` / `testparamstore` unit tests use to build real function
//! prototypes.
//!
//! # What is ported (faithful)
//!
//! Per **LOSS-006** (bison/flex grammars -> hand-written recursive-descent
//! parsers; `docs/rust-port/losses.md`), the generated `grammar.cc` automaton is
//! **not** transliterated.  Instead this module hand-ports the *semantics* of
//! `grammar.y`:
//!
//! - [`GrammarToken`] / [`GrammarLexer`] — the byte-for-byte finite-state lexer
//!   (`grammar.cc:220-677`), including the `0x` hex / octal / decimal integer
//!   rules, char-constant escapes, `//` and `/* */` comments, the `...`
//!   (`dotdotdot`) and `::` (`scoperes`) multi-char tokens, and the
//!   "simulate a trailing space at EOF to resolve the final token" trick.
//! - [`CParse`] — a recursive-descent parser over the ISO/IEC-9899 subset
//!   encoded in `grammar.y` (declarations, declaration-specifiers, declarators,
//!   pointers, arrays, function prototypes, struct/union/enum specifiers,
//!   parameter lists with `...`).  The grammar's reduce actions are transcribed
//!   one-for-one as the `CParse::*` builder methods ([`CParse::merge_spec_dec`],
//!   [`CParse::merge_pointer`], [`CParse::new_func`], ...).
//! - [`TypeDeclarator`] / [`TypeSpecifiers`] / the [`TypeModifier`] family
//!   (`PointerModifier` / `ArrayModifier` / `FunctionModifier`) and
//!   `TypeDeclarator::{is_valid, build_type, get_prototype}` — the AST and the
//!   data-type assembly logic (`grammar.cc:679-859`).
//! - [`parse_type`] and [`parse_protopieces`] — the two entry points that need
//!   only the read-mostly [`TypeFactory`] surface (pointer/array construction,
//!   `findByName`, `getTypeVoid`).  These are exactly what `testfuncproto` /
//!   `testparamstore` drive (`parse_protopieces` then
//!   `ProtoModel::assignParameterStorage`).
//!
//! # What is stubbed (NOT yet ported — see LOSS-006 / LOSS-090 restoration)
//!
//! The struct/union/enum *construction* actions ([`CParse::new_struct`],
//! [`CParse::new_union`], [`CParse::new_enum`]) and the `parse_C` store-writes
//! (typedef -> `getTypedef`/`setName`; extern -> `setPrototype`) require
//! TypeFactory-*mutating* orchestrators (`assignRawFields`, `setEnumValues`,
//! `getTypedef`, `setName`) and an `Architecture` prototype-model registry
//! (`getModel` / `defaultfp` / `setPrototype`) that the W6/W4 ports left as
//! stubs (the kuna `Architecture` has **no** type-factory field or model
//! registry; `// STUB(w6-fspec-2)`).  The grammar *parses* these forms (the AST
//! is built and `is_valid` runs), but completing them errs with the same
//! C++-faithful explain text, deferred to the type-factory/model wave.  The
//! nested-function-pointer `buildType` path (`FunctionModifier::modType` ->
//! `getTypeCode(PrototypePieces)`) is likewise a stub for the same reason
//! (top-level prototypes go through [`TypeDeclarator::get_prototype`], which does
//! **not** need `getTypeCode`).
//!
//! The interface-command address parsers that share `grammar.cc`
//! (`parse_machaddr` / `parse_varnode` / `parse_op` / `parse_toseparator`) are
//! **not** part of the C-declaration grammar; they belong to the `ifacedecomp`
//! console surface and are out of scope for this item.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4, uintb};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeBitField, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;

// =============================================================================
// Default-data-space context (the `glb->getDefaultDataSpace()` info the C++
// pointer-modifier reads)
// =============================================================================

/// The pieces of `Architecture::getDefaultDataSpace()` the grammar needs when it
/// builds pointers (`PointerModifier::modType`, `grammar.cc:679-686` reads
/// `getAddrSize()` and `getWordSize()`).
///
/// The kuna `Architecture` (W4) does not yet own a [`TypeFactory`] field, so the
/// C++ `Architecture *glb` argument is split into the read-mostly factory the
/// callers pass plus this small data-organization record (the only `glb` state
/// the C-declaration grammar actually consults).  // STUB(w6-fspec-2)
#[derive(Debug, Clone, Copy)]
pub struct DataOrg {
    /// Byte size of an address in the default data space (C++
    /// `getDefaultDataSpace()->getAddrSize()`).
    pub addr_size: int4,
    /// Word size of the default data space (C++ `getWordSize()`).
    pub word_size: uint4,
}

// =============================================================================
// GrammarToken (grammar.hh:23-68, grammar.y:220-293)
// =============================================================================

/// Token kinds produced by [`GrammarLexer`] (C++ `GrammarToken` anonymous enum,
/// `grammar.hh:26-46`).  The single-character punctuation tokens keep their
/// ASCII codes so the parser can compare against `b'(' as u32` etc., exactly as
/// the C++ returns `(uint4)buffer[bufstart]`.
pub mod tok {
    use kuna_base::types::uint4;

    pub const OPENPAREN: uint4 = 0x28;
    pub const CLOSEPAREN: uint4 = 0x29;
    pub const STAR: uint4 = 0x2a;
    pub const COMMA: uint4 = 0x2c;
    pub const SEMICOLON: uint4 = 0x3b;
    pub const OPENBRACKET: uint4 = 0x5b;
    pub const CLOSEBRACKET: uint4 = 0x5d;
    pub const OPENBRACE: uint4 = 0x7b;
    pub const CLOSEBRACE: uint4 = 0x7d;

    pub const BADTOKEN: uint4 = 0x100;
    pub const ENDOFFILE: uint4 = 0x101;
    pub const DOTDOTDOT: uint4 = 0x102;
    pub const SCOPERES: uint4 = 0x103;

    pub const INTEGER: uint4 = 0x104;
    pub const CHARCONSTANT: uint4 = 0x105;
    pub const IDENTIFIER: uint4 = 0x106;
    pub const STRINGVAL: uint4 = 0x107;
}

/// The value carried by a [`GrammarToken`] (the C++ `union tokenvalue`).
#[derive(Debug, Clone)]
enum TokenValue {
    /// `value.integer` (C++ `uintb`).
    Integer(uintb),
    /// `value.stringval` (C++ `string *`).
    StringVal(String),
    /// No value (punctuation / structural tokens).
    None,
}

/// A single lexed token (C++ `GrammarToken`, `grammar.hh:23-68`).
#[derive(Debug, Clone)]
pub struct GrammarToken {
    /// Token kind (one of the [`tok`] constants).
    ty: uint4,
    /// Carried value.
    value: TokenValue,
    /// Line number containing this token (C++ `lineno`).
    lineno: int4,
    /// Column where this token starts (C++ `colno`).
    colno: int4,
    /// Which file we were in (C++ `filenum`).
    filenum: int4,
}

impl GrammarToken {
    /// C++ `GrammarToken::GrammarToken(void)` (`grammar.cc:285-293`).
    fn new() -> GrammarToken {
        GrammarToken { ty: 0, value: TokenValue::None, lineno: -1, colno: -1, filenum: -1 }
    }

    /// C++ `GrammarToken::set(uint4 tp)` (`grammar.cc:220-224`).
    fn set(&mut self, tp: uint4) {
        self.ty = tp;
    }

    /// C++ `GrammarToken::set(uint4 tp,char *ptr,int4 len)` (`grammar.cc:226-283`).
    ///
    /// `bytes` is the `ptr[0..len]` slice the C++ takes.
    fn set_with(&mut self, tp: uint4, bytes: &[u8]) -> KunaResult<()> {
        self.ty = tp;
        match tp {
            tok::INTEGER => {
                self.value = TokenValue::Integer(parse_c_integer(bytes));
            }
            tok::IDENTIFIER | tok::STRINGVAL => {
                // C++ `new string(ptr,len)`.  The lexer never feeds non-UTF-8
                // here (only ASCII identifier/quote bytes survive moveState).
                self.value = TokenValue::StringVal(String::from_utf8_lossy(bytes).into_owned());
            }
            tok::CHARCONSTANT => {
                let v: uintb = if bytes.len() == 1 {
                    bytes[0] as uintb
                } else {
                    // Backslash escape (C++ switch on ptr[1]).
                    match bytes[1] {
                        b'n' => 10,
                        b'0' => 0,
                        b'a' => 7,
                        b'b' => 8,
                        b't' => 9,
                        b'v' => 11,
                        b'f' => 12,
                        b'r' => 13,
                        other => other as uintb,
                    }
                };
                self.value = TokenValue::Integer(v);
            }
            _ => {
                return Err(KunaError::lowlevel("Bad internal grammar token set"));
            }
        }
        Ok(())
    }

    fn set_position(&mut self, file: int4, line: int4, col: int4) {
        self.filenum = file;
        self.lineno = line;
        self.colno = col;
    }

    /// C++ `getType` (`grammar.hh:62`).
    pub fn get_type(&self) -> uint4 {
        self.ty
    }

    /// C++ `getInteger` (`grammar.hh:63`).
    fn get_integer(&self) -> uintb {
        match self.value {
            TokenValue::Integer(v) => v,
            _ => 0,
        }
    }

    /// C++ `getString` (`grammar.hh:64`).
    fn get_string(&self) -> String {
        match &self.value {
            TokenValue::StringVal(s) => s.clone(),
            _ => String::new(),
        }
    }
}

/// Parse a C integer literal exactly the way the C++ `istringstream` with
/// `unsetf(dec|hex|oct)` does (`grammar.cc:1796-1804`): the `operator>>(intb&)`
/// num_get facet performs a `strtoll`-style *leading-prefix* scan — optional
/// sign, base auto-detected from the prefix (`0x`/`0X` -> hex, leading `0` ->
/// octal, else decimal), reading the longest run of digits valid for that base
/// and STOPPING at the first invalid character, then re-tagging unsigned
/// (`value.integer = (uintb)val`).
///
/// The lexer's `number` state (`grammar.cc:2094-2125`, mirrored at
/// [`LexState::Number`]) accretes ASCII letters and `_` into a NUMBER token, so
/// inputs like `1z`, `123abc`, `08`, `5_0`, `007_0` are single reachable tokens.
/// The C++ `>>` parses the valid prefix and discards the rest: `1z`->1,
/// `123abc`->123, `08`->0 (octal stops at the non-octal `8`), `5_0`->5,
/// `007_0`->7.  This must NOT full-string-parse (LOSS-124): a trailing letter /
/// underscore / out-of-base digit truncates the value, it does not invalidate
/// the literal.
///
/// Overflow is the signed saturation the C++11 facet performs on `intb`:
/// positive overflow leaves `val == INT64_MAX`, negative overflow leaves
/// `val == INT64_MIN` (e.g. `0xffffffffffffffff` -> `i64::MAX`).
fn parse_c_integer(bytes: &[u8]) -> uintb {
    // The lexer corpus is ASCII; treat the bytes directly (matches the C++
    // `istringstream` over the raw `char` buffer, which never sees a leading
    // space here — but `operator>>` would skip leading whitespace, so honor it).
    let mut i = 0usize;
    let n = bytes.len();
    while i < n && (bytes[i] as char).is_whitespace() {
        i += 1;
    }

    // Optional sign.
    let neg = if i < n && bytes[i] == b'-' {
        i += 1;
        true
    } else if i < n && bytes[i] == b'+' {
        i += 1;
        false
    } else {
        false
    };

    // Base auto-detection from the prefix (strtoll/num_get rules).
    let radix: u32 = if i < n && bytes[i] == b'0' {
        if i + 1 < n && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
            // `0x`/`0X`: hex.  The leading `0` is consumed as part of the
            // prefix; if no hex digit follows, only the `0` was parsed (value 0).
            i += 2;
            16
        } else {
            // Leading `0` (not `0x`): octal.  Leave the `0` in the digit run so
            // a bare `0` parses as 0 and `08`/`007_0` stop at the first invalid
            // octal char.
            8
        }
    } else {
        10
    };

    // Accumulate the longest valid-digit prefix with signed saturation, exactly
    // as the C++11 num_get facet leaves `val`.
    let mut val: i64 = 0;
    while i < n {
        // cast: ASCII byte to char for digit classification; bytes are the
        // lexer's `number` corpus (ASCII only).
        let digit = match (bytes[i] as char).to_digit(radix) {
            Some(d) => d,
            None => break,
        };
        // cast: `digit` is in `0..radix` (<= 16) and `radix` is one of 8/10/16,
        // both fit i64 exactly.
        let d = digit as i64;
        let base = radix as i64;
        if neg {
            val = val
                .checked_mul(base)
                .and_then(|v| v.checked_sub(d))
                .unwrap_or(i64::MIN);
        } else {
            val = val
                .checked_mul(base)
                .and_then(|v| v.checked_add(d))
                .unwrap_or(i64::MAX);
        }
        i += 1;
    }

    // cast: re-tag the signed `intb` as unsigned, matching `(uintb)val`.
    val as uintb
}

// =============================================================================
// GrammarLexer (grammar.hh:70-117, grammar.cc:295-677)
// =============================================================================

/// Finite-state-machine states of [`GrammarLexer`] (C++ anonymous enum,
/// `grammar.hh:83-101`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexState {
    Start,
    Slash,
    Dot1,
    Dot2,
    Dot3,
    Scoperes1,
    Scoperes2,
    Punctuation,
    EndoflineComment,
    CComment,
    Doublequote,
    Doublequoteend,
    Singlequote,
    Singlequoteend,
    Singlebackslash,
    Number,
    Identifier,
}

/// The byte-for-byte port of the C++ `GrammarLexer` (`grammar.cc:295-677`).
///
/// The C++ reads from an `istream` it `pushFile`s; the Rust port holds the full
/// input as a byte vector with a read cursor (`parse line` / `parse file` feed a
/// single string/file).  The single-file bookkeeping (`filenamemap` / `filestack`)
/// is collapsed to one filename, since the C-declaration grammar never `#include`s.
struct GrammarLexer {
    buffersize: int4,
    /// Current line being processed (C++ `buffer`); a growable byte buffer.
    buffer: Vec<u8>,
    /// Next character to process (C++ `bufstart`).
    bufstart: int4,
    /// Next open position in buffer (C++ `bufend`).
    bufend: int4,
    curlineno: int4,
    endoffile: bool,
    state: LexState,
    error: String,
    /// The single pushed file's name (C++ `filenamemap`/`filestack` collapsed).
    filename: String,
    /// `true` once a file has been pushed (so `filestack` is non-empty).
    has_file: bool,
    /// The remaining input bytes and a read cursor (C++ `in` / `istream`).
    input: Vec<u8>,
    input_pos: usize,
}

impl GrammarLexer {
    /// C++ `GrammarLexer::GrammarLexer(int4 maxbuffer)` (`grammar.cc:295-306`).
    fn new(maxbuffer: int4) -> GrammarLexer {
        GrammarLexer {
            buffersize: maxbuffer,
            // One extra slot for the EOF "simulate a space" write, which the C++
            // performs at buffer[bufend++] after the `bufend >= buffersize`
            // "Line too long" guard already bounded bufend to <= buffersize.
            buffer: vec![0u8; (maxbuffer.max(0) as usize) + 1],
            bufstart: 0,
            bufend: 0,
            curlineno: 0,
            endoffile: true,
            state: LexState::Start,
            error: String::new(),
            filename: String::new(),
            has_file: false,
            input: Vec::new(),
            input_pos: 0,
        }
    }

    fn set_error(&mut self, err: &str) {
        self.error = err.to_string();
    }

    /// C++ `getError` (`grammar.hh:116`).
    fn get_error(&self) -> &str {
        &self.error
    }

    /// C++ `GrammarLexer::clear(void)` (`grammar.cc:581-594`).
    fn clear(&mut self) {
        self.filename.clear();
        self.has_file = false;
        self.input.clear();
        self.input_pos = 0;
        self.bufstart = 0;
        self.bufend = 0;
        self.curlineno = 0;
        self.state = LexState::Start;
        self.endoffile = true;
        self.error.clear();
    }

    /// C++ `GrammarLexer::bumpLine(void)` (`grammar.cc:314-320`).
    fn bump_line(&mut self) {
        self.curlineno += 1;
        self.bufstart = 0;
        self.bufend = 0;
    }

    /// C++ `GrammarLexer::pushFile(const string &filename,istream *i)`
    /// (`grammar.cc:615-624`).  In the kuna port the stream content is passed by
    /// value as bytes.
    fn push_file(&mut self, filename: &str, content: Vec<u8>) {
        self.filename = filename.to_string();
        self.has_file = true;
        self.input = content;
        self.input_pos = 0;
        self.endoffile = false;
    }

    /// Read one byte from the current stream (C++ `in->get(nextchar)`); returns
    /// `None` at end of stream (the C++ `!(*in)`).
    fn stream_get(&mut self) -> Option<u8> {
        if self.input_pos < self.input.len() {
            let c = self.input[self.input_pos];
            self.input_pos += 1;
            Some(c)
        } else {
            None
        }
    }

    /// C++ `GrammarLexer::moveState(char lookahead)` (`grammar.cc:322-568`).
    ///
    /// Returns the resolved token kind (0 = keep accreting), or `badtoken` on a
    /// lexer error (with `error` set).  Faithfully replicates the FSM transition
    /// table, the `bufstart = bufend-1` "begin token here" sets, the newline
    /// handling, and the `0x`-hex guard in the `number` state.
    fn move_state(&mut self, lookahead_in: u8) -> uint4 {
        let mut lookahead = lookahead_in;
        let mut newline = false;

        if lookahead < 32 {
            if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 13 {
                lookahead = b' ';
            } else if lookahead == b'\n' {
                newline = true;
                lookahead = b' ';
            } else {
                self.set_error("Illegal character");
                return tok::BADTOKEN;
            }
        } else if lookahead >= 127 {
            self.set_error("Illegal character");
            return tok::BADTOKEN;
        }

        let mut res: uint4 = 0;
        let mut syntaxerror = false;
        match self.state {
            LexState::Start => match lookahead {
                b'/' => self.state = LexState::Slash,
                b'.' => self.state = LexState::Dot1,
                b'*' | b',' | b'(' | b')' | b'[' | b']' | b'{' | b'}' | b';' | b'=' => {
                    self.state = LexState::Punctuation;
                    self.bufstart = self.bufend - 1;
                }
                b':' => self.state = LexState::Scoperes1,
                b'-' | b'0'..=b'9' => {
                    self.state = LexState::Number;
                    self.bufstart = self.bufend - 1;
                }
                b' ' => {} // Ignore since we are already open
                b'"' => {
                    self.state = LexState::Doublequote;
                    self.bufstart = self.bufend - 1;
                }
                b'\'' => self.state = LexState::Singlequote,
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.state = LexState::Identifier;
                    self.bufstart = self.bufend - 1;
                }
                _ => {
                    self.set_error("Illegal character");
                    return tok::BADTOKEN;
                }
            },
            LexState::Slash => {
                if lookahead == b'*' {
                    self.state = LexState::CComment;
                } else if lookahead == b'/' {
                    self.state = LexState::EndoflineComment;
                } else {
                    syntaxerror = true;
                }
            }
            LexState::Dot1 => {
                if lookahead == b'.' {
                    self.state = LexState::Dot2;
                } else {
                    syntaxerror = true;
                }
            }
            LexState::Dot2 => {
                if lookahead == b'.' {
                    self.state = LexState::Dot3;
                } else {
                    syntaxerror = true;
                }
            }
            LexState::Dot3 => {
                self.state = LexState::Start;
                res = tok::DOTDOTDOT;
            }
            LexState::Scoperes1 => {
                if lookahead == b':' {
                    self.state = LexState::Scoperes2;
                } else {
                    self.state = LexState::Start;
                    res = b':' as uint4;
                }
            }
            LexState::Scoperes2 => {
                self.state = LexState::Start;
                res = tok::SCOPERES;
            }
            LexState::Punctuation => {
                self.state = LexState::Start;
                res = self.buffer[self.bufstart as usize] as uint4;
            }
            LexState::EndoflineComment => {
                if newline {
                    self.state = LexState::Start;
                }
                // Anything else is part of comment
            }
            LexState::CComment => {
                if lookahead == b'/'
                    && self.bufend > 1
                    && self.buffer[(self.bufend - 2) as usize] == b'*'
                {
                    self.state = LexState::Start;
                }
                // Anything else is part of comment
            }
            LexState::Doublequote => {
                if lookahead == b'"' {
                    self.state = LexState::Doublequoteend;
                }
                // Anything else is part of string
            }
            LexState::Doublequoteend => {
                self.state = LexState::Start;
                res = tok::STRINGVAL;
            }
            LexState::Singlequote => {
                if lookahead == b'\\' {
                    self.state = LexState::Singlebackslash;
                } else if lookahead == b'\'' {
                    self.state = LexState::Singlequoteend;
                }
                // Anything else is part of string
            }
            LexState::Singlequoteend => {
                self.state = LexState::Start;
                res = tok::CHARCONSTANT;
            }
            LexState::Singlebackslash => {
                self.state = LexState::Singlequote;
            }
            LexState::Number => {
                if lookahead == b'x' {
                    if (self.bufend - self.bufstart) != 2 || self.buffer[self.bufstart as usize] != b'0' {
                        syntaxerror = true; // x only allowed as 0x hex indicator
                    }
                } else if lookahead.is_ascii_digit()
                    || lookahead.is_ascii_uppercase()
                    || lookahead.is_ascii_lowercase()
                    || lookahead == b'_'
                {
                    // part of the number
                } else {
                    self.state = LexState::Start;
                    res = tok::INTEGER;
                }
            }
            LexState::Identifier => {
                if lookahead.is_ascii_digit()
                    || lookahead.is_ascii_uppercase()
                    || lookahead.is_ascii_lowercase()
                    || lookahead == b'_'
                {
                    // part of the identifier
                } else {
                    self.state = LexState::Start;
                    res = tok::IDENTIFIER;
                }
            }
        }

        if syntaxerror {
            self.set_error("Syntax error");
            return tok::BADTOKEN;
        }
        if newline {
            self.bump_line();
        }
        res
    }

    /// C++ `GrammarLexer::establishToken(GrammarToken &token,uint4 val)`
    /// (`grammar.cc:570-579`): for a structural token store the kind; for a
    /// value-bearing token slice `buffer[bufstart .. bufend-1]` (the trailing
    /// lookahead char is excluded) and decode it.
    fn establish_token(&self, token: &mut GrammarToken, val: uint4) -> KunaResult<()> {
        if val < tok::INTEGER {
            token.set(val);
        } else {
            let start = self.bufstart as usize;
            let end = (self.bufend - 1) as usize;
            let bytes = &self.buffer[start..end];
            token.set_with(val, bytes)?;
        }
        let filenum: int4 = if self.has_file { 0 } else { -1 };
        token.set_position(filenum, self.curlineno, self.bufstart);
        Ok(())
    }

    /// C++ `GrammarLexer::getNextToken(GrammarToken &token)`
    /// (`grammar.cc:638-677`).
    fn get_next_token(&mut self, token: &mut GrammarToken) -> KunaResult<()> {
        let mut tok_kind = tok::BADTOKEN;
        let mut firsttimethru = true;

        if self.endoffile {
            token.set(tok::ENDOFFILE);
            return Ok(());
        }

        loop {
            let nextchar: u8;
            if !firsttimethru || self.bufend == 0 {
                if self.bufend >= self.buffersize {
                    self.set_error("Line too long");
                    tok_kind = tok::BADTOKEN;
                    break;
                }
                match self.stream_get() {
                    Some(c) => {
                        nextchar = c;
                        self.buffer[self.bufend as usize] = nextchar;
                        self.bufend += 1;
                    }
                    None => {
                        self.endoffile = true;
                        break;
                    }
                }
            } else {
                nextchar = self.buffer[(self.bufend - 1) as usize]; // old lookahead
            }
            tok_kind = self.move_state(nextchar);
            firsttimethru = false;
            if tok_kind != 0 {
                break;
            }
        }

        if self.endoffile {
            self.buffer[self.bufend as usize] = b' '; // Simulate a space
            self.bufend += 1;
            tok_kind = self.move_state(b' '); // to let the final token resolve
            if tok_kind == 0
                && self.state != LexState::Start
                && self.state != LexState::EndoflineComment
            {
                self.set_error("Incomplete token");
                tok_kind = tok::BADTOKEN;
            }
        }

        self.establish_token(token, tok_kind)?;
        Ok(())
    }
}

// =============================================================================
// TypeModifier family (grammar.hh:121-166, grammar.cc:679-760)
// =============================================================================

/// C++ `TypeModifier` kind tags (`grammar.hh:123-129`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierKind {
    Pointer,
    Array,
    Function,
}

/// A single declarator modification (C++ `TypeModifier` subclasses,
/// `grammar.hh:121-166`).  Modelled as an enum rather than a virtual hierarchy.
#[derive(Debug, Clone)]
enum TypeModifier {
    /// `PointerModifier` (`grammar.hh:136-143`).  `flags` are the type-qualifier
    /// bits the C++ `PointerModifier` stores but never reads in `modType`
    /// (kept for structural fidelity with the C++ member).
    Pointer {
        #[allow(dead_code)] // C++ PointerModifier::flags: stored-but-unread in modType.
        flags: uint4,
    },
    /// `ArrayModifier` (`grammar.hh:145-153`).  `flags` likewise mirror the C++
    /// member (`modType` reads only `arraysize`).
    Array {
        #[allow(dead_code)] // C++ ArrayModifier::flags: stored-but-unread in modType.
        flags: uint4,
        arraysize: int4,
    },
    /// `FunctionModifier` (`grammar.hh:155-166`).  `paramlist` holds the parsed
    /// parameter declarators; a trailing `None` is **not** stored here (the
    /// `...` is captured in `dotdotdot`, exactly as `newFunc` pops the null).
    Function { paramlist: Vec<TypeDeclarator>, dotdotdot: bool },
}

impl TypeModifier {
    /// C++ `TypeModifier::getType` (`grammar.hh:131`).
    fn get_type(&self) -> ModifierKind {
        match self {
            TypeModifier::Pointer { .. } => ModifierKind::Pointer,
            TypeModifier::Array { .. } => ModifierKind::Array,
            TypeModifier::Function { .. } => ModifierKind::Function,
        }
    }

    /// C++ `TypeModifier::isValid` (`grammar.hh`/`grammar.cc:726-739`).
    fn is_valid(&self) -> bool {
        match self {
            // PointerModifier::isValid -> true (grammar.hh:141).
            TypeModifier::Pointer { .. } => true,
            // ArrayModifier::isValid -> arraysize>0 (grammar.hh:151).
            TypeModifier::Array { arraysize, .. } => *arraysize > 0,
            // FunctionModifier::isValid (grammar.cc:726-739).
            TypeModifier::Function { paramlist, .. } => {
                for decl in paramlist {
                    if !decl.is_valid_inner() {
                        return false;
                    }
                    if decl.mods.is_empty() {
                        if let Some(ct) = &decl.basetype {
                            if ct.get_metatype() == type_metatype::TYPE_VOID {
                                return false; // Extra void type
                            }
                        }
                    }
                }
                true
            }
        }
    }

    /// C++ `*Modifier::modType(base,decl,glb)` (`grammar.cc:679-760`).
    ///
    /// Builds the data-type one modification level produces.  `base` is the
    /// data-type the inner (already-applied) modifications produced.
    fn mod_type(
        &self,
        base: Option<Rc<Datatype>>,
        factory: &dyn TypeFactory,
        org: &DataOrg,
    ) -> KunaResult<Rc<Datatype>> {
        match self {
            TypeModifier::Pointer { .. } => {
                // PointerModifier::modType (grammar.cc:679-686).
                let base = base.ok_or_else(|| {
                    KunaError::lowlevel("grammar: pointer modifier requires a base type")
                })?;
                factory.get_type_pointer(org.addr_size, base, org.word_size)
            }
            TypeModifier::Array { arraysize, .. } => {
                // ArrayModifier::modType (grammar.cc:688-693).
                let base = base.ok_or_else(|| {
                    KunaError::lowlevel("grammar: array modifier requires a base type")
                })?;
                factory.get_type_array(*arraysize, base)
            }
            TypeModifier::Function { paramlist, .. } => {
                // FunctionModifier::modType (grammar.cc:2306-2325): build a
                // PrototypePieces describing the pointed-to function and intern a
                // TypeCode for it via glb->types->getTypeCode(proto).
                let outtype = match base {
                    Some(b) => b,
                    None => factory.get_type_void()?,
                };
                let intypes = function_get_in_types(paramlist, factory, org)?;
                // Varargs is encoded as an extra null pointer in paramlist; the
                // Rust `paramlist` never holds that null (CParse::newFunc popped it
                // into the modifier's `dotdotdot`, grammar.cc:2605-2618), so this
                // C++ `paramlist.back() == 0` check never fires — transcribed
                // faithfully (firstVarArgSlot stays -1 exactly as upstream).
                //
                // proto.model = decl->getModel(glb): a parsed function-pointer
                // field carries no model name, so getModel returns glb->defaultfp;
                // the factory supplies it inside getTypeCode(proto).  The kuna
                // PrototypePieces carries no `model` field (// STUB(w6-fspec-2)).
                let proto = PrototypePieces {
                    outtype: Some(outtype),
                    intypes,
                    first_var_arg_slot: -1,
                    ..PrototypePieces::default()
                };
                factory.get_type_code_proto(&proto)
            }
        }
    }
}

/// C++ `FunctionModifier::FunctionModifier` (`grammar.cc:695-708`): drop a lone
/// `void` parameter so `f(void)` becomes the empty parameter list.
fn function_modifier_new(mut paramlist: Vec<TypeDeclarator>, dtdtdt: bool) -> TypeModifier {
    if paramlist.len() == 1 {
        let decl = &paramlist[0];
        if decl.mods.is_empty() {
            if let Some(ct) = &decl.basetype {
                if ct.get_metatype() == type_metatype::TYPE_VOID {
                    paramlist.clear();
                }
            }
        }
    }
    TypeModifier::Function { paramlist, dotdotdot: dtdtdt }
}

/// C++ `FunctionModifier::getInTypes` (`grammar.cc:710-717`).
fn function_get_in_types(
    paramlist: &[TypeDeclarator],
    factory: &dyn TypeFactory,
    org: &DataOrg,
) -> KunaResult<Vec<Rc<Datatype>>> {
    let mut intypes = Vec::new();
    for p in paramlist {
        intypes.push(p.build_type(factory, org)?);
    }
    Ok(intypes)
}

/// C++ `FunctionModifier::getInNames` (`grammar.cc:719-724`).
fn function_get_in_names(paramlist: &[TypeDeclarator]) -> Vec<String> {
    paramlist.iter().map(|p| p.ident.clone()).collect()
}

// =============================================================================
// TypeDeclarator (grammar.hh:168-190, grammar.cc:762-859)
// =============================================================================

/// C-declaration specifier/qualifier flag bits (C++ `CParse` anonymous enum,
/// `grammar.hh:209-222`).
pub mod flags {
    use kuna_base::types::uint4;

    pub const F_TYPEDEF: uint4 = 1;
    pub const F_EXTERN: uint4 = 2;
    pub const F_STATIC: uint4 = 4;
    pub const F_AUTO: uint4 = 8;
    pub const F_REGISTER: uint4 = 16;
    pub const F_CONST: uint4 = 32;
    pub const F_RESTRICT: uint4 = 64;
    pub const F_VOLATILE: uint4 = 128;
    pub const F_INLINE: uint4 = 256;
    pub const F_STRUCT: uint4 = 512;
    pub const F_UNION: uint4 = 1024;
    pub const F_ENUM: uint4 = 2048;
}

/// A declarator: a base type plus an ordered list of modifications and an
/// identifier (C++ `TypeDeclarator`, `grammar.hh:168-190`).
#[derive(Debug, Clone)]
pub struct TypeDeclarator {
    /// Applied in reverse order of binding (C++ `mods`).
    mods: Vec<TypeModifier>,
    /// Base data-type (C++ `basetype`; `None` is the C++ null).
    basetype: Option<Rc<Datatype>>,
    /// Variable identifier associated with type (C++ `ident`).
    ident: String,
    /// Name of model associated with function pointer (C++ `model`).
    model: String,
    /// Specifier/qualifier flag bits (C++ `flags`).
    flags: uint4,
    /// Number of bits associated with declaration, 0=unspecified (C++ `numBits`).
    num_bits: int4,
}

impl TypeDeclarator {
    /// C++ `TypeDeclarator::TypeDeclarator(void)` (`grammar.hh:177`).
    fn new() -> TypeDeclarator {
        TypeDeclarator {
            mods: Vec::new(),
            basetype: None,
            ident: String::new(),
            model: String::new(),
            flags: 0,
            num_bits: 0,
        }
    }

    /// C++ `TypeDeclarator::TypeDeclarator(const string &nm)` (`grammar.hh:178`).
    fn with_ident(nm: &str) -> TypeDeclarator {
        let mut d = TypeDeclarator::new();
        d.ident = nm.to_string();
        d
    }

    /// C++ `getBaseType` (`grammar.hh:180`).
    pub fn get_base_type(&self) -> Option<&Rc<Datatype>> {
        self.basetype.as_ref()
    }

    /// C++ `numModifiers` (`grammar.hh:181`).
    pub fn num_modifiers(&self) -> usize {
        self.mods.len()
    }

    /// C++ `getNumBits` (`grammar.hh:182`).
    pub fn get_num_bits(&self) -> int4 {
        self.num_bits
    }

    /// C++ `setNumBits` (`grammar.hh:183`).
    fn set_num_bits(&mut self, val: int4) {
        self.num_bits = val;
    }

    /// C++ `getIdentifier` (`grammar.hh:184`).
    pub fn get_identifier(&self) -> &str {
        &self.ident
    }

    /// C++ `hasProperty` (`grammar.hh:187`).
    pub fn has_property(&self, mask: uint4) -> bool {
        (self.flags & mask) != 0
    }

    /// C++ `TypeDeclarator::buildType` (`grammar.cc:769-780`): apply the
    /// modifications to the basetype in **reverse** order of binding.
    pub fn build_type(&self, factory: &dyn TypeFactory, org: &DataOrg) -> KunaResult<Rc<Datatype>> {
        let mut restype = self.basetype.clone();
        for m in self.mods.iter().rev() {
            restype = Some(m.mod_type(restype, factory, org)?);
        }
        restype.ok_or_else(|| KunaError::lowlevel("grammar: declarator has no base type"))
    }

    /// C++ `TypeDeclarator::getPrototype` (`grammar.cc:794-822`).
    ///
    /// Returns `Ok(false)` if the leading modification is not a function
    /// modifier (the C++ `return false`); otherwise fills `pieces` with the
    /// function's input/return types and names.  The kuna [`PrototypePieces`]
    /// carries no `model` field (`// STUB(w6-fspec-2)`), so `pieces.model` is not
    /// set — the model is supplied separately by the proto-model wave.
    pub fn get_prototype(
        &self,
        pieces: &mut PrototypePieces,
        factory: &dyn TypeFactory,
        org: &DataOrg,
    ) -> KunaResult<bool> {
        let mod0 = self.mods.first();
        let fmod = match mod0 {
            Some(m) if m.get_type() == ModifierKind::Function => m,
            _ => return Ok(false),
        };
        let (paramlist, dotdotdot) = match fmod {
            TypeModifier::Function { paramlist, dotdotdot } => (paramlist, *dotdotdot),
            _ => unreachable!(),
        };

        // pieces.model = getModel(glb);  // STUB: no model registry on Architecture.
        pieces.name = self.ident.clone();
        pieces.intypes.clear();
        pieces.intypes = function_get_in_types(paramlist, factory, org)?;
        pieces.innames.clear();
        pieces.innames = function_get_in_names(paramlist);
        pieces.first_var_arg_slot =
            if dotdotdot { pieces.intypes.len() as int4 } else { -1 };

        // Construct the output type: apply every modification EXCEPT the leading
        // function modifier (grammar.cc:812-820).
        let mut outtype = self.basetype.clone();
        for m in self.mods.iter().skip(1).rev() {
            outtype = Some(m.mod_type(outtype, factory, org)?);
        }
        pieces.outtype = outtype;
        Ok(true)
    }

    /// The non-throwing half of C++ `TypeDeclarator::isValid` (the
    /// per-modifier/basetype checks) — see [`TypeDeclarator::is_valid`] for the
    /// throwing storage/qualifier-count checks.  Used inside
    /// `FunctionModifier::isValid` where the C++ calls `decl->isValid()` but the
    /// surrounding context only ever has zero/one specifier.
    fn is_valid_inner(&self) -> bool {
        if self.basetype.is_none() {
            return false;
        }
        for m in &self.mods {
            if !m.is_valid() {
                return false;
            }
        }
        true
    }

    /// C++ `TypeDeclarator::isValid` (`grammar.cc:824-859`).  Returns `Err`
    /// (C++ `throw ParseError`) on multiple storage specifiers / multiple type
    /// qualifiers, mirroring the C++ control flow exactly.
    pub fn is_valid(&self) -> KunaResult<bool> {
        if self.basetype.is_none() {
            return Ok(false); // No basetype
        }

        let mut count = 0;
        if (self.flags & flags::F_TYPEDEF) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_EXTERN) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_STATIC) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_AUTO) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_REGISTER) != 0 {
            count += 1;
        }
        if count > 1 {
            return Err(KunaError::parse("Multiple storage specifiers"));
        }

        count = 0;
        if (self.flags & flags::F_CONST) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_RESTRICT) != 0 {
            count += 1;
        }
        if (self.flags & flags::F_VOLATILE) != 0 {
            count += 1;
        }
        if count > 1 {
            return Err(KunaError::parse("Multiple type qualifiers"));
        }

        for m in &self.mods {
            if !m.is_valid() {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// C++ `TypeSpecifiers` (`grammar.hh:192-197`).
#[derive(Debug, Clone)]
struct TypeSpecifiers {
    type_specifier: Option<Rc<Datatype>>,
    function_specifier: String,
    flags: uint4,
}

impl TypeSpecifiers {
    fn new() -> TypeSpecifiers {
        TypeSpecifiers { type_specifier: None, function_specifier: String::new(), flags: 0 }
    }
}

/// C++ `Enumerator` (`grammar.hh:199-205`).
#[derive(Debug, Clone)]
struct Enumerator {
    enumconstant: String,
    constantassigned: bool,
    value: uintb,
}

impl Enumerator {
    fn new(nm: &str) -> Enumerator {
        Enumerator { enumconstant: nm.to_string(), constantassigned: false, value: 0 }
    }

    fn with_value(nm: &str, val: uintb) -> Enumerator {
        Enumerator { enumconstant: nm.to_string(), constantassigned: true, value: val }
    }
}

// =============================================================================
// Document type (grammar.hh:223-226)
// =============================================================================

/// C++ `CParse` document-type enum (`grammar.hh:223-226`): which top-level
/// production the parser should expect (the `firsttoken` injected before
/// `yyparse`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocType {
    /// `doc_declaration` — a full `declaration`.
    Declaration,
    /// `doc_parameter_declaration` — a single `parameter_declaration`.
    ParameterDeclaration,
}

// =============================================================================
// CParse — the recursive-descent parser (grammar.y productions + grammar.cc
// actions)
// =============================================================================

/// Token-class returned by [`CParse::lookup_identifier`], standing in for the
/// bison terminal an identifier reduces to (`grammar.cc:1236-1272`).
#[derive(Debug, Clone)]
enum IdentClass {
    StorageClassSpecifier,
    TypeQualifier,
    FunctionSpecifier,
    Struct,
    Union,
    Enum,
    TypeName(Rc<Datatype>),
    Identifier,
}

/// A lexer token lifted to the parser's terminal alphabet (the bison token plus
/// its semantic value), produced by [`CParse::lex`].
#[derive(Debug, Clone)]
enum PToken {
    /// End of input (C++ lex returns -1).
    Eof,
    /// A single-character punctuation token (`(`, `)`, `*`, `,`, `;`, `[`, `]`,
    /// `{`, `}`, `=`, `:`).
    Punct(u8),
    Number(uintb),
    Identifier(String),
    StorageClassSpecifier(String),
    TypeQualifier(String),
    FunctionSpecifier(String),
    TypeName(Rc<Datatype>),
    Struct,
    Union,
    Enum,
    Dotdotdot,
    Scoperes,
}

/// The C-declaration parser (C++ `CParse`, `grammar.hh:207-286` +
/// `grammar.cc:861-1387`).
///
/// Holds the lexer, the keyword table, the architecture context (factory + data
/// organization), the last-error string, and the result declarations.  The
/// `*_alloc` arena lists from the C++ are unnecessary in Rust (ownership moves
/// through the by-value AST), so they are dropped.
pub struct CParse<'a> {
    factory: &'a dyn TypeFactory,
    /// The data-organization record (the `glb` state `build_type`/`get_prototype`
    /// read).  In C++ this is part of the `Architecture *glb` the parser holds; the
    /// kuna split carries it alongside the factory.  // STUB(w6-fspec-2)
    org: DataOrg,
    keywords: BTreeMap<String, uint4>,
    lexer: GrammarLexer,
    /// Location of the last token (C++ `lineno`/`colno`/`filenum`).
    lineno: int4,
    colno: int4,
    filenum: int4,
    lasterror: String,
    lastdecls: Option<Vec<TypeDeclarator>>,
    /// One-token lookahead buffer (the parser pulls tokens via [`CParse::next`]).
    pushed: Option<PToken>,
}

impl<'a> CParse<'a> {
    /// C++ `CParse::CParse(Architecture *g,int4 maxbuf)` (`grammar.cc:861-882`).
    ///
    /// The C++ `Architecture *glb` is split into the [`TypeFactory`] (used by the
    /// `findByName` identifier classification) and — only at the `build_type` /
    /// `get_prototype` calls the entry points make — the [`DataOrg`].  The parser
    /// proper needs only the factory, so `CParse` holds just that.
    pub fn new(factory: &'a dyn TypeFactory, org: DataOrg, maxbuf: int4) -> CParse<'a> {
        let mut keywords: BTreeMap<String, uint4> = BTreeMap::new();
        keywords.insert("typedef".to_string(), flags::F_TYPEDEF);
        keywords.insert("extern".to_string(), flags::F_EXTERN);
        keywords.insert("static".to_string(), flags::F_STATIC);
        keywords.insert("auto".to_string(), flags::F_AUTO);
        keywords.insert("register".to_string(), flags::F_REGISTER);
        keywords.insert("const".to_string(), flags::F_CONST);
        keywords.insert("restrict".to_string(), flags::F_RESTRICT);
        keywords.insert("volatile".to_string(), flags::F_VOLATILE);
        keywords.insert("inline".to_string(), flags::F_INLINE);
        keywords.insert("struct".to_string(), flags::F_STRUCT);
        keywords.insert("union".to_string(), flags::F_UNION);
        keywords.insert("enum".to_string(), flags::F_ENUM);
        CParse {
            factory,
            org,
            keywords,
            lexer: GrammarLexer::new(maxbuf),
            lineno: -1,
            colno: -1,
            filenum: -1,
            lasterror: String::new(),
            lastdecls: None,
            pushed: None,
        }
    }

    /// C++ `getError` (`grammar.hh:283`).
    pub fn get_error(&self) -> &str {
        &self.lasterror
    }

    /// C++ `getResultDeclarations` (`grammar.hh:285`).
    pub fn get_result_declarations(&self) -> Option<&Vec<TypeDeclarator>> {
        self.lastdecls.as_ref()
    }

    /// Take the result declarations out (the kuna consumers move the AST).
    pub fn take_result_declarations(&mut self) -> Option<Vec<TypeDeclarator>> {
        self.lastdecls.take()
    }

    /// C++ `CParse::clear(void)` (`grammar.cc:890-898`).
    fn clear(&mut self) {
        self.lasterror.clear();
        self.lastdecls = None;
        self.lexer.clear();
        self.pushed = None;
    }

    /// C++ `CParse::setError(const string &msg)` (`grammar.cc:1318-1328`).
    ///
    /// Formats `<msg> at line <line> in <file>\n<source-line>\n<caret>`.  The
    /// kuna lexer keeps the single source line, so the location suffix is
    /// reproduced.  Errors only set `lasterror` once (the C++ `lex` short-circuits
    /// once `lasterror` is non-empty).
    fn set_error(&mut self, msg: &str) {
        let mut s = String::new();
        s.push_str(msg);
        // writeLocation (grammar.cc:596-601).
        s.push_str(&format!(" at line {}", self.lineno));
        s.push_str(&format!(" in {}", self.lexer.filename));
        s.push('\n');
        // writeTokenLocation (grammar.cc:603-613).
        if self.lineno == self.lexer.curlineno {
            for i in 0..self.lexer.bufend {
                s.push(self.lexer.buffer[i as usize] as char);
            }
            s.push('\n');
            for _ in 0..self.colno {
                s.push(' ');
            }
            s.push_str("^--\n");
        }
        self.lasterror = s;
    }

    /// C++ `CParse::lookupIdentifier(const string &nm)` (`grammar.cc:1236-1272`).
    fn lookup_identifier(&mut self, nm: &str) -> IdentClass {
        if let Some(&kw) = self.keywords.get(nm) {
            match kw {
                flags::F_TYPEDEF | flags::F_EXTERN | flags::F_STATIC | flags::F_AUTO
                | flags::F_REGISTER => return IdentClass::StorageClassSpecifier,
                flags::F_CONST | flags::F_RESTRICT | flags::F_VOLATILE => {
                    return IdentClass::TypeQualifier
                }
                flags::F_INLINE => return IdentClass::FunctionSpecifier,
                flags::F_STRUCT => return IdentClass::Struct,
                flags::F_UNION => return IdentClass::Union,
                flags::F_ENUM => return IdentClass::Enum,
                _ => {}
            }
        }
        if let Ok(Some(tp)) = self.factory.find_by_name(nm) {
            return IdentClass::TypeName(tp);
        }
        // glb->hasModel(nm) -> FUNCTION_SPECIFIER.  The kuna Architecture has no
        // model registry, so this never fires (it would classify e.g.
        // "__stdcall" as a function specifier).  // STUB(w6-fspec-2)
        IdentClass::Identifier
    }

    /// C++ `CParse::lex(void)` (`grammar.cc:1274-1316`): pull the next lexer
    /// token and lift it to a parser terminal.  Once `lasterror` is set the C++
    /// returns `BADTOKEN` forever; the Rust port surfaces that as a parse error
    /// at the call site.
    fn lex(&mut self) -> KunaResult<PToken> {
        if !self.lasterror.is_empty() {
            return Err(KunaError::parse(self.lasterror.clone()));
        }
        let mut t = GrammarToken::new();
        self.lexer.get_next_token(&mut t)?;
        self.lineno = t.lineno;
        self.colno = t.colno;
        self.filenum = t.filenum;
        match t.get_type() {
            tok::INTEGER | tok::CHARCONSTANT => Ok(PToken::Number(t.get_integer())),
            tok::IDENTIFIER => {
                let s = t.get_string();
                match self.lookup_identifier(&s) {
                    IdentClass::StorageClassSpecifier => Ok(PToken::StorageClassSpecifier(s)),
                    IdentClass::TypeQualifier => Ok(PToken::TypeQualifier(s)),
                    IdentClass::FunctionSpecifier => Ok(PToken::FunctionSpecifier(s)),
                    IdentClass::Struct => Ok(PToken::Struct),
                    IdentClass::Union => Ok(PToken::Union),
                    IdentClass::Enum => Ok(PToken::Enum),
                    IdentClass::TypeName(tp) => Ok(PToken::TypeName(tp)),
                    IdentClass::Identifier => Ok(PToken::Identifier(s)),
                }
            }
            tok::STRINGVAL => {
                self.set_error("Illegal string constant");
                Err(KunaError::parse(self.lasterror.clone()))
            }
            tok::DOTDOTDOT => Ok(PToken::Dotdotdot),
            tok::SCOPERES => Ok(PToken::Scoperes),
            tok::BADTOKEN => {
                let err = self.lexer.get_error().to_string();
                self.set_error(&err);
                Err(KunaError::parse(self.lasterror.clone()))
            }
            tok::ENDOFFILE => Ok(PToken::Eof),
            other => Ok(PToken::Punct(other as u8)),
        }
    }

    // -- one-token lookahead --------------------------------------------------

    /// Peek at the next terminal without consuming it.
    fn peek(&mut self) -> KunaResult<&PToken> {
        if self.pushed.is_none() {
            let t = self.lex()?;
            self.pushed = Some(t);
        }
        Ok(self.pushed.as_ref().unwrap())
    }

    /// Consume and return the next terminal.
    fn next(&mut self) -> KunaResult<PToken> {
        if let Some(t) = self.pushed.take() {
            return Ok(t);
        }
        self.lex()
    }

    /// Consume the next terminal, requiring it to be the given punctuation
    /// character; otherwise raise a syntax error (the bison `yyerror` path).
    fn expect_punct(&mut self, ch: u8) -> KunaResult<()> {
        match self.next()? {
            PToken::Punct(c) if c == ch => Ok(()),
            _ => self.syntax_error(),
        }
    }

    /// Raise the bison default "Syntax error" (`grammar.cc:1346-1347`).
    fn syntax_error<T>(&mut self) -> KunaResult<T> {
        if self.lasterror.is_empty() {
            self.set_error("Syntax error");
        }
        Err(KunaError::parse(self.lasterror.clone()))
    }

    // -- grammar.y reduce actions (grammar.cc:900-1190) -----------------------

    /// C++ `mergeSpecDec(TypeSpecifiers*,TypeDeclarator*)` (`grammar.cc:900-907`).
    fn merge_spec_dec(spec: &TypeSpecifiers, mut dec: TypeDeclarator) -> TypeDeclarator {
        dec.basetype = spec.type_specifier.clone();
        dec.model = spec.function_specifier.clone();
        dec.flags |= spec.flags;
        dec
    }

    /// C++ `mergeSpecDec(TypeSpecifiers*)` (`grammar.cc:909-915`).
    fn merge_spec_dec_empty(spec: &TypeSpecifiers) -> TypeDeclarator {
        Self::merge_spec_dec(spec, TypeDeclarator::new())
    }

    /// C++ `mergeSpecDecVec(TypeSpecifiers*,vector*)` (`grammar.cc:917-923`).
    fn merge_spec_dec_vec(spec: &TypeSpecifiers, declist: Vec<TypeDeclarator>) -> Vec<TypeDeclarator> {
        declist.into_iter().map(|d| Self::merge_spec_dec(spec, d)).collect()
    }

    /// C++ `mergeSpecDecVec(TypeSpecifiers*)` (`grammar.cc:925-935`).
    fn merge_spec_dec_vec_empty(spec: &TypeSpecifiers) -> Vec<TypeDeclarator> {
        vec![Self::merge_spec_dec_empty(spec)]
    }

    /// C++ `convertFlag(string*)` (`grammar.cc:937-947`).
    fn convert_flag(&mut self, s: &str) -> uint4 {
        if let Some(&v) = self.keywords.get(s) {
            return v;
        }
        self.set_error("Unknown qualifier");
        0
    }

    /// C++ `addSpecifier(TypeSpecifiers*,string*)` (`grammar.cc:949-955`).
    fn add_specifier(&mut self, spec: &mut TypeSpecifiers, s: &str) {
        let flag = self.convert_flag(s);
        spec.flags |= flag;
    }

    /// C++ `addTypeSpecifier(TypeSpecifiers*,Datatype*)` (`grammar.cc:957-964`).
    fn add_type_specifier(&mut self, spec: &mut TypeSpecifiers, tp: Rc<Datatype>) {
        if spec.type_specifier.is_some() {
            self.set_error("Multiple type specifiers");
        }
        spec.type_specifier = Some(tp);
    }

    /// C++ `addFuncSpecifier(TypeSpecifiers*,string*)` (`grammar.cc:966-980`).
    fn add_func_specifier(&mut self, spec: &mut TypeSpecifiers, s: &str) {
        if let Some(&v) = self.keywords.get(s) {
            spec.flags |= v; // A reserved specifier
        } else {
            if !spec.function_specifier.is_empty() {
                self.set_error("Multiple parameter models");
            }
            spec.function_specifier = s.to_string();
        }
    }

    /// C++ `mergePointer(vector<uint4>*,TypeDeclarator*)` (`grammar.cc:982-990`).
    fn merge_pointer(ptr: &[uint4], mut dec: TypeDeclarator) -> TypeDeclarator {
        for &fl in ptr {
            dec.mods.push(TypeModifier::Pointer { flags: fl });
        }
        dec
    }

    /// C++ `newArray(TypeDeclarator*,uint4,uintb*)` (`grammar.cc:1032-1038`).
    fn new_array(mut dec: TypeDeclarator, fl: uint4, num: uintb) -> TypeDeclarator {
        dec.mods.push(TypeModifier::Array { flags: fl, arraysize: num as int4 });
        dec
    }

    /// C++ `newFunc(TypeDeclarator*,vector*)` (`grammar.cc:1040-1053`): a trailing
    /// null (varargs marker) is popped and recorded as `dotdotdot`.
    fn new_func(mut dec: TypeDeclarator, mut declist: Vec<Option<TypeDeclarator>>) -> TypeDeclarator {
        let mut dotdotdot = false;
        if let Some(last) = declist.last() {
            if last.is_none() {
                dotdotdot = true;
                declist.pop();
            }
        }
        let params: Vec<TypeDeclarator> = declist.into_iter().map(|o| o.unwrap()).collect();
        dec.mods.push(function_modifier_new(params, dotdotdot));
        dec
    }

    // -- recursive-descent over grammar.y -------------------------------------

    /// `document: DECLARATION_RESULT declaration | PARAM_RESULT parameter_declaration`
    /// (`grammar.y:62-65`).  Drives the chosen top-level production then requires
    /// EOF.
    fn parse_document(&mut self, doctype: DocType) -> KunaResult<()> {
        match doctype {
            DocType::Declaration => {
                let decls = self.declaration()?;
                self.lastdecls = Some(decls);
            }
            DocType::ParameterDeclaration => {
                let d = self.parameter_declaration()?;
                self.lastdecls = Some(vec![d]);
            }
        }
        // bison requires the start symbol to consume the whole input.
        match self.next()? {
            PToken::Eof => Ok(()),
            _ => self.syntax_error(),
        }
    }

    /// `declaration: declaration_specifiers ';'
    ///             | declaration_specifiers init_declarator_list ';'`
    /// (`grammar.y:67-70`).
    fn declaration(&mut self) -> KunaResult<Vec<TypeDeclarator>> {
        let spec = self.declaration_specifiers()?;
        match self.peek()? {
            PToken::Punct(c) if *c == b';' => {
                self.next()?;
                Ok(Self::merge_spec_dec_vec_empty(&spec))
            }
            _ => {
                let declist = self.init_declarator_list()?;
                self.expect_punct(b';')?;
                Ok(Self::merge_spec_dec_vec(&spec, declist))
            }
        }
    }

    /// `declaration_specifiers` (`grammar.y:72-81`): a non-empty run of storage
    /// class / type / qualifier / function specifiers.
    fn declaration_specifiers(&mut self) -> KunaResult<TypeSpecifiers> {
        let mut spec = TypeSpecifiers::new();
        // First specifier is mandatory.
        if !self.declaration_specifier_one(&mut spec)? {
            return self.syntax_error();
        }
        // Subsequent specifiers continue while the lookahead begins one.
        while self.declaration_specifier_starts()? {
            self.declaration_specifier_one(&mut spec)?;
        }
        Ok(spec)
    }

    /// Does the lookahead begin a `declaration_specifiers` element?
    fn declaration_specifier_starts(&mut self) -> KunaResult<bool> {
        Ok(matches!(
            self.peek()?,
            PToken::StorageClassSpecifier(_)
                | PToken::TypeQualifier(_)
                | PToken::FunctionSpecifier(_)
                | PToken::TypeName(_)
                | PToken::Struct
                | PToken::Union
                | PToken::Enum
        ))
    }

    /// Consume one `declaration_specifiers` element into `spec`.  Returns `false`
    /// if the lookahead is not a specifier (so the caller can detect "no
    /// specifier at all").
    fn declaration_specifier_one(&mut self, spec: &mut TypeSpecifiers) -> KunaResult<bool> {
        match self.peek()? {
            PToken::StorageClassSpecifier(_) => {
                if let PToken::StorageClassSpecifier(s) = self.next()? {
                    self.add_specifier(spec, &s);
                }
                Ok(true)
            }
            PToken::TypeQualifier(_) => {
                if let PToken::TypeQualifier(s) = self.next()? {
                    self.add_specifier(spec, &s);
                }
                Ok(true)
            }
            PToken::FunctionSpecifier(_) => {
                if let PToken::FunctionSpecifier(s) = self.next()? {
                    self.add_func_specifier(spec, &s);
                }
                Ok(true)
            }
            PToken::TypeName(_) | PToken::Struct | PToken::Union | PToken::Enum => {
                let tp = self.type_specifier()?;
                self.add_type_specifier(spec, tp);
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// `init_declarator_list: init_declarator (',' init_declarator)*`
    /// (`grammar.y:83-86`).
    fn init_declarator_list(&mut self) -> KunaResult<Vec<TypeDeclarator>> {
        let mut v = vec![self.declarator()?];
        while matches!(self.peek()?, PToken::Punct(b',')) {
            self.next()?;
            v.push(self.declarator()?);
        }
        Ok(v)
    }

    /// `type_specifier: TYPE_NAME | struct_or_union_specifier | enum_specifier`
    /// (`grammar.y:93-97`).
    fn type_specifier(&mut self) -> KunaResult<Rc<Datatype>> {
        match self.peek()? {
            PToken::TypeName(_) => {
                if let PToken::TypeName(tp) = self.next()? {
                    Ok(tp)
                } else {
                    unreachable!()
                }
            }
            PToken::Struct | PToken::Union => self.struct_or_union_specifier(),
            PToken::Enum => self.enum_specifier(),
            _ => self.syntax_error(),
        }
    }

    /// `struct_or_union_specifier` (`grammar.y:99-106`).
    fn struct_or_union_specifier(&mut self) -> KunaResult<Rc<Datatype>> {
        let is_struct = matches!(self.next()?, PToken::Struct);
        // optional IDENTIFIER, then optional '{' struct_declaration_list '}'.
        let ident = if let PToken::Identifier(_) = self.peek()? {
            if let PToken::Identifier(s) = self.next()? {
                s
            } else {
                unreachable!()
            }
        } else {
            String::new()
        };
        if matches!(self.peek()?, PToken::Punct(b'{')) {
            self.next()?;
            let declist = self.struct_declaration_list()?;
            self.expect_punct(b'}')?;
            if is_struct {
                self.new_struct(&ident, declist)
            } else {
                self.new_union(&ident, declist)
            }
        } else {
            // `STRUCT IDENTIFIER` (no body) -> oldStruct/oldUnion.  An empty
            // identifier here is a syntax error (no production matches).
            if ident.is_empty() {
                return self.syntax_error();
            }
            if is_struct {
                self.old_struct(&ident)
            } else {
                self.old_union(&ident)
            }
        }
    }

    /// `struct_declaration_list: struct_declaration+` (`grammar.y:108-111`):
    /// concatenate the per-declaration declarator vectors.
    fn struct_declaration_list(&mut self) -> KunaResult<Vec<TypeDeclarator>> {
        let mut v = self.struct_declaration()?;
        // Continue while the lookahead begins a specifier_qualifier_list.
        while matches!(
            self.peek()?,
            PToken::TypeQualifier(_) | PToken::TypeName(_) | PToken::Struct | PToken::Union | PToken::Enum
        ) {
            let mut more = self.struct_declaration()?;
            v.append(&mut more);
        }
        Ok(v)
    }

    /// `struct_declaration: specifier_qualifier_list struct_declarator_list ';'`
    /// (`grammar.y:113-115`).
    fn struct_declaration(&mut self) -> KunaResult<Vec<TypeDeclarator>> {
        let spec = self.specifier_qualifier_list()?;
        let declist = self.struct_declarator_list()?;
        self.expect_punct(b';')?;
        Ok(Self::merge_spec_dec_vec(&spec, declist))
    }

    /// `specifier_qualifier_list` (`grammar.y:117-122`).
    fn specifier_qualifier_list(&mut self) -> KunaResult<TypeSpecifiers> {
        let mut spec = TypeSpecifiers::new();
        let mut any = false;
        loop {
            match self.peek()? {
                PToken::TypeName(_) | PToken::Struct | PToken::Union | PToken::Enum => {
                    let tp = self.type_specifier()?;
                    self.add_type_specifier(&mut spec, tp);
                    any = true;
                }
                PToken::TypeQualifier(_) => {
                    if let PToken::TypeQualifier(s) = self.next()? {
                        self.add_specifier(&mut spec, &s);
                    }
                    any = true;
                }
                _ => break,
            }
        }
        if !any {
            return self.syntax_error();
        }
        Ok(spec)
    }

    /// `struct_declarator_list: struct_declarator (',' struct_declarator)*`
    /// (`grammar.y:124-127`).
    fn struct_declarator_list(&mut self) -> KunaResult<Vec<TypeDeclarator>> {
        let mut v = vec![self.struct_declarator()?];
        while matches!(self.peek()?, PToken::Punct(b',')) {
            self.next()?;
            v.push(self.struct_declarator()?);
        }
        Ok(v)
    }

    /// `struct_declarator: declarator | declarator ':' NUMBER` (`grammar.y:129-132`).
    fn struct_declarator(&mut self) -> KunaResult<TypeDeclarator> {
        let mut dec = self.declarator()?;
        if matches!(self.peek()?, PToken::Punct(b':')) {
            self.next()?;
            match self.next()? {
                PToken::Number(n) => dec.set_num_bits(n as int4),
                _ => return self.syntax_error(),
            }
        }
        Ok(dec)
    }

    /// `enum_specifier` (`grammar.y:134-140`).
    fn enum_specifier(&mut self) -> KunaResult<Rc<Datatype>> {
        self.next()?; // ENUM
        let ident = if let PToken::Identifier(_) = self.peek()? {
            if let PToken::Identifier(s) = self.next()? {
                s
            } else {
                unreachable!()
            }
        } else {
            String::new()
        };
        if matches!(self.peek()?, PToken::Punct(b'{')) {
            self.next()?;
            let vecenum = self.enumerator_list()?;
            // optional trailing comma before '}'.
            if matches!(self.peek()?, PToken::Punct(b',')) {
                self.next()?;
            }
            self.expect_punct(b'}')?;
            self.new_enum(&ident, vecenum)
        } else {
            // `ENUM IDENTIFIER` -> oldEnum.
            if ident.is_empty() {
                return self.syntax_error();
            }
            self.old_enum(&ident)
        }
    }

    /// `enumerator_list: enumerator (',' enumerator)*` (`grammar.y:142-145`).
    /// The trailing-comma cases are handled by the caller (it consumes an
    /// optional `,` before `}`), exactly as the two `... ',' '}'` productions do.
    fn enumerator_list(&mut self) -> KunaResult<Vec<Enumerator>> {
        let mut v = vec![self.enumerator()?];
        while matches!(self.peek()?, PToken::Punct(b',')) {
            // A `,` may be the list separator OR the trailing comma before `}`.
            // Peek past it: if `}` follows, leave the `,` for the caller.
            self.next()?; // consume ','
            if matches!(self.peek()?, PToken::Punct(b'}')) {
                // Trailing comma: re-inject so the caller's `}` logic is uniform.
                self.pushed = Some(PToken::Punct(b'}'));
                break;
            }
            v.push(self.enumerator()?);
        }
        Ok(v)
    }

    /// `enumerator: IDENTIFIER | IDENTIFIER '=' NUMBER` (`grammar.y:147-150`).
    fn enumerator(&mut self) -> KunaResult<Enumerator> {
        let nm = match self.next()? {
            PToken::Identifier(s) => s,
            _ => return self.syntax_error(),
        };
        if matches!(self.peek()?, PToken::Punct(b'=')) {
            self.next()?;
            match self.next()? {
                PToken::Number(n) => Ok(Enumerator::with_value(&nm, n)),
                _ => self.syntax_error(),
            }
        } else {
            Ok(Enumerator::new(&nm))
        }
    }

    /// `declarator: direct_declarator | pointer direct_declarator`
    /// (`grammar.y:152-155`).
    fn declarator(&mut self) -> KunaResult<TypeDeclarator> {
        if matches!(self.peek()?, PToken::Punct(b'*')) {
            let ptr = self.pointer()?;
            let dec = self.direct_declarator()?;
            Ok(Self::merge_pointer(&ptr, dec))
        } else {
            self.direct_declarator()
        }
    }

    /// `var_identifier: IDENTIFIER (SCOPERES IDENTIFIER)*` (`grammar.y:157-160`):
    /// builds a `a::b::c` scoped name.
    fn var_identifier(&mut self, first: String) -> KunaResult<String> {
        let mut name = first;
        while matches!(self.peek()?, PToken::Scoperes) {
            self.next()?; // SCOPERES
            match self.next()? {
                PToken::Identifier(s) => {
                    name.push_str("::");
                    name.push_str(&s);
                }
                _ => return self.syntax_error(),
            }
        }
        Ok(name)
    }

    /// `direct_declarator` (`grammar.y:162-170`): the base name or
    /// parenthesised declarator, followed by array `[...]` and function `(...)`
    /// suffixes (left-recursion unrolled to a loop).
    fn direct_declarator(&mut self) -> KunaResult<TypeDeclarator> {
        let mut dec = match self.next()? {
            PToken::Identifier(s) => {
                let name = self.var_identifier(s)?;
                TypeDeclarator::with_ident(&name)
            }
            PToken::Punct(b'(') => {
                let inner = self.declarator()?;
                self.expect_punct(b')')?;
                inner
            }
            _ => return self.syntax_error(),
        };
        dec = self.direct_declarator_suffixes(dec)?;
        Ok(dec)
    }

    /// The left-recursive array/function suffixes shared by `direct_declarator`
    /// and `direct_abstract_declarator`.
    fn direct_declarator_suffixes(&mut self, mut dec: TypeDeclarator) -> KunaResult<TypeDeclarator> {
        loop {
            match self.peek()? {
                PToken::Punct(b'[') => {
                    self.next()?; // '['
                    // direct_declarator '[' type_qualifier_list assignment_expression ']'
                    // | direct_declarator '[' assignment_expression ']'
                    let qual = if matches!(self.peek()?, PToken::TypeQualifier(_)) {
                        self.type_qualifier_list()?
                    } else {
                        0
                    };
                    let n = self.assignment_expression()?;
                    self.expect_punct(b']')?;
                    dec = Self::new_array(dec, qual, n);
                }
                PToken::Punct(b'(') => {
                    self.next()?; // '('
                    let declist = self.parameter_type_list()?;
                    self.expect_punct(b')')?;
                    dec = Self::new_func(dec, declist);
                }
                _ => break,
            }
        }
        Ok(dec)
    }

    /// `pointer: ('*' type_qualifier_list?)+` (`grammar.y:172-177`).
    ///
    /// The C++ builds the qualifier vector inside-out (`'*' pointer` pushes a 0
    /// AFTER the recursion), so the qualifier for the **innermost** `*` ends up
    /// last in the vector and `mergePointer` applies pointers in vector order.
    /// We replicate the exact final vector by collecting the per-`*` qualifiers
    /// and reversing.
    fn pointer(&mut self) -> KunaResult<Vec<uint4>> {
        // Collect (in left-to-right `*` order) each star's qualifier flags.
        let mut stars: Vec<uint4> = Vec::new();
        loop {
            self.expect_punct(b'*')?;
            let q = if matches!(self.peek()?, PToken::TypeQualifier(_)) {
                self.type_qualifier_list()?
            } else {
                0
            };
            stars.push(q);
            if !matches!(self.peek()?, PToken::Punct(b'*')) {
                break;
            }
        }
        // grammar.y builds the vector via right recursion: the LAST '*' parsed
        // (innermost, deepest recursion) push_backs first, so the vector is the
        // reverse of left-to-right star order.
        stars.reverse();
        Ok(stars)
    }

    /// `type_qualifier_list: TYPE_QUALIFIER+` (`grammar.y:179-182`): OR the
    /// qualifier flag bits together.
    fn type_qualifier_list(&mut self) -> KunaResult<uint4> {
        let mut flags: uint4 = 0;
        let mut any = false;
        while matches!(self.peek()?, PToken::TypeQualifier(_)) {
            if let PToken::TypeQualifier(s) = self.next()? {
                flags |= self.convert_flag(&s);
            }
            any = true;
        }
        if !any {
            return self.syntax_error();
        }
        Ok(flags)
    }

    /// `parameter_type_list: parameter_list | parameter_list ',' DOTDOTDOT`
    /// (`grammar.y:184-187`).  Returns the parameter declarators with a trailing
    /// `None` for the `...` (the varargs marker `newFunc` pops).
    fn parameter_type_list(&mut self) -> KunaResult<Vec<Option<TypeDeclarator>>> {
        let mut v: Vec<Option<TypeDeclarator>> = Vec::new();
        // parameter_list: parameter_declaration (',' parameter_declaration)*
        v.push(Some(self.parameter_declaration()?));
        loop {
            if !matches!(self.peek()?, PToken::Punct(b',')) {
                break;
            }
            self.next()?; // ','
            if matches!(self.peek()?, PToken::Dotdotdot) {
                self.next()?;
                v.push(None); // varargs marker
                break;
            }
            v.push(Some(self.parameter_declaration()?));
        }
        Ok(v)
    }

    /// `parameter_declaration` (`grammar.y:194-198`):
    /// `declaration_specifiers (declarator | abstract_declarator)?`.
    fn parameter_declaration(&mut self) -> KunaResult<TypeDeclarator> {
        let spec = self.declaration_specifiers()?;
        match self.peek()? {
            // declarator: pointer? direct_declarator (starts with '*', IDENTIFIER, or '(').
            // abstract_declarator: pointer | direct_abstract_declarator
            //                    | pointer direct_abstract_declarator (starts with '*' or '(').
            PToken::Punct(b'*') => {
                // pointer, then optional direct_(abstract_)declarator.
                let ptr = self.pointer()?;
                match self.peek()? {
                    PToken::Identifier(_) | PToken::Punct(b'(') => {
                        // Could be a (possibly abstract) declarator continuation.
                        let inner = self.declarator_or_abstract_after_pointer()?;
                        let dec = Self::merge_pointer(&ptr, inner);
                        Ok(Self::merge_spec_dec(&spec, dec))
                    }
                    _ => {
                        // abstract_declarator: pointer  (grammar.y:201)
                        let dec = Self::merge_pointer(&ptr, TypeDeclarator::new());
                        Ok(Self::merge_spec_dec(&spec, dec))
                    }
                }
            }
            PToken::Identifier(_) => {
                let dec = self.declarator()?;
                Ok(Self::merge_spec_dec(&spec, dec))
            }
            PToken::Punct(b'(') | PToken::Punct(b'[') => {
                // direct_abstract_declarator (or a parenthesised concrete
                // declarator).  Try the (abstract-or-concrete) direct declarator.
                let dec = self.direct_abstract_or_concrete()?;
                Ok(Self::merge_spec_dec(&spec, dec))
            }
            _ => {
                // declaration_specifiers alone (grammar.y:196).
                Ok(Self::merge_spec_dec_empty(&spec))
            }
        }
    }

    /// After a leading `pointer`, parse the following declarator which may be
    /// concrete (`direct_declarator`) or abstract (`direct_abstract_declarator`).
    fn declarator_or_abstract_after_pointer(&mut self) -> KunaResult<TypeDeclarator> {
        self.direct_abstract_or_concrete()
    }

    /// Parse a `direct_declarator` or `direct_abstract_declarator`.  The two
    /// differ only in whether the leading atom is a name (concrete) or empty /
    /// parenthesised-abstract; both share the array/function suffix loop.
    fn direct_abstract_or_concrete(&mut self) -> KunaResult<TypeDeclarator> {
        let dec = match self.peek()? {
            PToken::Identifier(_) => {
                if let PToken::Identifier(s) = self.next()? {
                    let name = self.var_identifier(s)?;
                    TypeDeclarator::with_ident(&name)
                } else {
                    unreachable!()
                }
            }
            PToken::Punct(b'(') => {
                self.next()?; // '('
                // '(' declarator ')'  OR  '(' abstract_declarator ')'  OR
                // a function suffix '(' parameter_type_list ')' on an EMPTY
                // direct_abstract_declarator (grammar.y:206-211).
                match self.peek()? {
                    PToken::Punct(b'*') | PToken::Identifier(_) => {
                        // Parenthesised (abstract-or-concrete) declarator.
                        let inner = self.declarator_or_abstract_or_empty()?;
                        self.expect_punct(b')')?;
                        inner
                    }
                    _ => {
                        // '(' parameter_type_list ')' suffix on an empty
                        // abstract declarator: the leading '(' starts a function.
                        let declist = self.parameter_type_list()?;
                        self.expect_punct(b')')?;
                        Self::new_func(TypeDeclarator::new(), declist)
                    }
                }
            }
            PToken::Punct(b'[') => {
                // direct_abstract_declarator '[' assignment_expression ']' on an
                // empty base (grammar.y:209).
                TypeDeclarator::new()
            }
            _ => return self.syntax_error(),
        };
        self.direct_declarator_suffixes(dec)
    }

    /// Inside `(...)`: a declarator that may carry a leading pointer and may be
    /// abstract or concrete.
    fn declarator_or_abstract_or_empty(&mut self) -> KunaResult<TypeDeclarator> {
        if matches!(self.peek()?, PToken::Punct(b'*')) {
            let ptr = self.pointer()?;
            match self.peek()? {
                PToken::Identifier(_) | PToken::Punct(b'(') | PToken::Punct(b'[') => {
                    let inner = self.direct_abstract_or_concrete()?;
                    Ok(Self::merge_pointer(&ptr, inner))
                }
                _ => Ok(Self::merge_pointer(&ptr, TypeDeclarator::new())),
            }
        } else {
            self.direct_abstract_or_concrete()
        }
    }

    /// `assignment_expression: NUMBER` (`grammar.y:214-216`).
    fn assignment_expression(&mut self) -> KunaResult<uintb> {
        match self.next()? {
            PToken::Number(n) => Ok(n),
            _ => self.syntax_error(),
        }
    }

    // -- struct/union/enum construction actions (grammar.cc:1055-1189) --------

    /// C++ `CParse::newStruct` (`grammar.cc:1055-1085`).
    ///
    /// Builds the field list and would call `glb->types->assignRawFields(...)`.
    /// The mutating factory orchestrator (`assignRawFields` re-keys the interned
    /// stub) is a W6 type-factory boundary, so this errs after validating the
    /// declarators (preserving the C++ "Invalid structure declarator" message).
    /// // STUB(w6-fspec-2) — see LOSS-006 restoration.
    fn new_struct(&mut self, ident: &str, declist: Vec<TypeDeclarator>) -> KunaResult<Rc<Datatype>> {
        // Create the (incomplete) stub for recursion before any field references
        // it (C++ getTypeStruct).
        let res = self.factory.get_type_struct(ident)?;
        let is_big_endian = self.factory.is_big_endian();
        let mut sublist: Vec<TypeField> = Vec::new();
        let mut bitlist: Vec<TypeBitField> = Vec::new();
        for decl in &declist {
            if !decl.is_valid()? {
                self.set_error("Invalid structure declarator");
                self.factory.destroy_type(&res)?;
                return Err(KunaError::parse(self.lasterror.clone()));
            }
            let field_type = decl.build_type(self.factory, &self.org)?;
            if decl.get_num_bits() != 0 {
                bitlist.push(TypeBitField::new(
                    sublist.len() as int4,
                    decl.get_num_bits(),
                    is_big_endian,
                    decl.get_identifier(),
                    field_type,
                ));
            } else {
                sublist.push(TypeField::new(0, -1, decl.get_identifier(), field_type));
            }
        }
        // On a LowlevelError the C++ records setError + destroyType + returns
        // null (a parse failure).
        match self.factory.assign_raw_fields_struct(&res, sublist, bitlist) {
            Ok(completed) => Ok(completed),
            Err(err) => {
                self.set_error(err.explain());
                let _ = self.factory.destroy_type(&res);
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    /// C++ `CParse::oldStruct` (`grammar.cc:1087-1094`).
    fn old_struct(&mut self, ident: &str) -> KunaResult<Rc<Datatype>> {
        let res = self.factory.find_by_name(ident)?;
        match &res {
            Some(tp) if tp.get_metatype() == type_metatype::TYPE_STRUCT => Ok(tp.clone()),
            _ => {
                self.set_error("Identifier does not represent a struct as required");
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    /// C++ `CParse::newUnion` (`grammar.cc:1096-1121`).
    fn new_union(&mut self, ident: &str, declist: Vec<TypeDeclarator>) -> KunaResult<Rc<Datatype>> {
        // The (incomplete) stub (C++ getTypeUnion).
        let res = self.factory.get_type_union(ident)?;
        let mut sublist: Vec<TypeField> = Vec::new();
        for (i, decl) in declist.iter().enumerate() {
            if !decl.is_valid()? {
                self.set_error("Invalid union declarator");
                self.factory.destroy_type(&res)?;
                return Err(KunaError::parse(self.lasterror.clone()));
            }
            let field_type = decl.build_type(self.factory, &self.org)?;
            sublist.push(TypeField::new(i as int4, 0, decl.get_identifier(), field_type));
        }
        match self.factory.assign_raw_fields_union(&res, sublist) {
            Ok(completed) => Ok(completed),
            Err(err) => {
                self.set_error(err.explain());
                let _ = self.factory.destroy_type(&res);
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    /// C++ `CParse::oldUnion` (`grammar.cc:1123-1130`).
    fn old_union(&mut self, ident: &str) -> KunaResult<Rc<Datatype>> {
        let res = self.factory.find_by_name(ident)?;
        match &res {
            Some(tp) if tp.get_metatype() == type_metatype::TYPE_UNION => Ok(tp.clone()),
            _ => {
                self.set_error("Identifier does not represent a union as required");
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    /// C++ `CParse::newEnum` (`grammar.cc:1156-1180`).
    ///
    /// Ports the body up to the factory store-write: build the `(name,value,
    /// assigned)` lists from the parsed enumerators and run
    /// `TypeEnum::assignValues` (the duplicate-value check + free-value fill,
    /// `Datatype::assign_values`).  Installing the resulting value-map into the
    /// interned enum stub (`glb->types->setEnumValues`) is the W6 type-factory
    /// boundary, so this errs after the value computation.  // STUB(w6-fspec-2)
    fn new_enum(&mut self, ident: &str, vecenum: Vec<Enumerator>) -> KunaResult<Rc<Datatype>> {
        // An interned (incomplete) enum stub (C++ getTypeEnum).
        let res = self.factory.get_type_enum(ident)?;
        let mut namelist: Vec<String> = Vec::new();
        let mut vallist: Vec<uintb> = Vec::new();
        let mut assignlist: Vec<bool> = Vec::new();
        for e in &vecenum {
            namelist.push(e.enumconstant.clone());
            vallist.push(e.value);
            assignlist.push(e.constantassigned);
        }
        // Reports duplicate-value errors with the same explain text as the C++
        // (C++ TypeEnum::assignValues).
        match Datatype::assign_values(res.get_size(), res.get_name(), &namelist, &vallist, &assignlist)
        {
            Ok(namemap) => {
                match self.factory.set_enum_values(&res, namemap) {
                    Ok(completed) => Ok(completed),
                    Err(err) => {
                        self.set_error(err.explain());
                        let _ = self.factory.destroy_type(&res);
                        Err(KunaError::parse(self.lasterror.clone()))
                    }
                }
            }
            Err(err) => {
                // C++ catches LowlevelError, calls setError(err.explain), destroys
                // the stub, returns null.
                self.set_error(err.explain());
                let _ = self.factory.destroy_type(&res);
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    /// C++ `CParse::oldEnum` (`grammar.cc:1182-1189`).
    fn old_enum(&mut self, ident: &str) -> KunaResult<Rc<Datatype>> {
        let res = self.factory.find_by_name(ident)?;
        match &res {
            Some(tp) if tp.is_enum_type() => Ok(tp.clone()),
            _ => {
                self.set_error("Identifier does not represent an enum as required");
                Err(KunaError::parse(self.lasterror.clone()))
            }
        }
    }

    // -- top-level drivers (grammar.cc:1330-1375) -----------------------------

    /// C++ `CParse::runParse` + `parseStream` (`grammar.cc:1330-1375`).
    ///
    /// `Ok(true)` = parsed; `Ok(false)` = a *parse failure* recorded in
    /// `lasterror` (the C++ `yyparse()!=0` -> `runParse` returns false; the caller
    /// then `throw ParseError(getError())`).  `Err(e)` = a *thrown exception*
    /// (a `LowlevelError` from an action — e.g. the struct/enum construction
    /// boundary — that the C++ does **not** catch in `runParse`; it propagates past
    /// `parse_protopieces`).  The discriminator is whether the action recorded a
    /// `lasterror`: parse failures set it, thrown exceptions do not.
    fn parse_stream(&mut self, content: Vec<u8>, doctype: DocType) -> KunaResult<bool> {
        self.clear();
        self.lexer.push_file("stream", content);
        match self.parse_document(doctype) {
            Ok(()) => Ok(true),
            Err(e) => {
                if self.lasterror.is_empty() {
                    // No recorded parse error: this is a thrown exception (boundary /
                    // LowlevelError), which the C++ runParse never catches.
                    Err(e)
                } else {
                    Ok(false)
                }
            }
        }
    }
}

// =============================================================================
// Top-level entry points (grammar.cc:1389-1472)
// =============================================================================

/// C++ `parse_type(istream &s,string &name,Architecture *glb)`
/// (`grammar.cc:1389-1406`): parse a single C type, returning the data-type and
/// (via the returned `name`) any declared identifier.
///
/// Drives `doc_parameter_declaration`; the kuna `Architecture` info is supplied
/// as the [`TypeFactory`] plus the [`DataOrg`] (the only `glb` state the grammar
/// reads).  // STUB(w6-fspec-2)
pub fn parse_type(
    input: &str,
    factory: &dyn TypeFactory,
    org: DataOrg,
) -> KunaResult<(Rc<Datatype>, String)> {
    let mut parser = CParse::new(factory, org, 4096);
    if !parser.parse_stream(input.as_bytes().to_vec(), DocType::ParameterDeclaration)? {
        return Err(KunaError::parse(parser.get_error().to_string()));
    }
    let decls = parser
        .take_result_declarations()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| KunaError::parse("Did not parse a datatype"))?;
    if decls.len() > 1 {
        return Err(KunaError::parse("Parsed multiple declarations"));
    }
    let decl = &decls[0];
    if !decl.is_valid()? {
        return Err(KunaError::parse("Parsed type is invalid"));
    }
    let name = decl.get_identifier().to_string();
    let ty = decl.build_type(factory, &org)?;
    Ok((ty, name))
}

/// C++ `parse_protopieces(PrototypePieces &pieces,istream &s,Architecture *glb)`
/// (`grammar.cc:1408-1426`): parse a function declaration into its
/// [`PrototypePieces`].  This is the entry point `testfuncproto` /
/// `testparamstore` drive before `ProtoModel::assignParameterStorage`.
pub fn parse_protopieces(
    input: &str,
    factory: &dyn TypeFactory,
    org: DataOrg,
) -> KunaResult<PrototypePieces> {
    let mut parser = CParse::new(factory, org, 4096);
    if !parser.parse_stream(input.as_bytes().to_vec(), DocType::Declaration)? {
        return Err(KunaError::parse(parser.get_error().to_string()));
    }
    let decls = parser
        .take_result_declarations()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| KunaError::parse("Did not parse a datatype"))?;
    if decls.len() > 1 {
        return Err(KunaError::parse("Parsed multiple declarations"));
    }
    let decl = &decls[0];
    if !decl.is_valid()? {
        return Err(KunaError::parse("Parsed type is invalid"));
    }
    let mut pieces = PrototypePieces::default();
    if !decl.get_prototype(&mut pieces, factory, &org)? {
        return Err(KunaError::parse("Did not parse a prototype"));
    }
    Ok(pieces)
}

/// C++ `parse_C(Architecture *glb,istream &s)` (`grammar.cc:2993-3037`): load type
/// data straight into the data structures (a `parse line`/`parse file` body).
///
/// The C++ `Architecture *glb` is split into the [`TypeFactory`] (the construction
/// store-writes go through it), the [`DataOrg`], and `set_prototype` — a callback
/// standing in for `glb->setPrototype(pieces)` (which needs `symboltab`/`Funcdata`,
/// surfaces the console owns, not the grammar).  The construction itself (struct/
/// union/enum stub + `assignRawFields`/`setEnumValues`; typedef -> `setName`/
/// `getTypedef`) is fully wired into the factory.
///
/// Errors mirror the C++: a parse failure becomes a `ParseError`; the
/// "Did not parse a datatype"/"multiple declarations"/"invalid"/"Missing
/// identifier for typedef"/"Not sure what to do with this type" `ParseError`s and
/// the `LowlevelError` of the last clause are transcribed.
pub fn parse_c(
    input: &str,
    factory: &dyn TypeFactory,
    org: DataOrg,
    set_prototype: impl FnOnce(PrototypePieces) -> KunaResult<()>,
) -> KunaResult<()> {
    let mut parser = CParse::new(factory, org, 4096);
    if !parser.parse_stream(input.as_bytes().to_vec(), DocType::Declaration)? {
        return Err(KunaError::parse(parser.get_error().to_string()));
    }
    let decls = parser
        .take_result_declarations()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| KunaError::parse("Did not parse a datatype"))?;
    if decls.len() > 1 {
        return Err(KunaError::parse("Parsed multiple declarations"));
    }
    let decl = &decls[0];
    if !decl.is_valid()? {
        return Err(KunaError::parse("Parsed type is invalid"));
    }

    if decl.has_property(flags::F_EXTERN) {
        // extern: parse a prototype and hand it to glb->setPrototype(pieces).
        let mut pieces = PrototypePieces::default();
        if !decl.get_prototype(&mut pieces, factory, &org)? {
            return Err(KunaError::parse("Did not parse prototype as expected"));
        }
        set_prototype(pieces)?;
    } else if decl.has_property(flags::F_TYPEDEF) {
        let ct = decl.build_type(factory, &org)?;
        if decl.get_identifier().is_empty() {
            return Err(KunaError::parse("Missing identifier for typedef"));
        }
        if ct.get_metatype() == type_metatype::TYPE_STRUCT {
            factory.set_name(&ct, decl.get_identifier())?;
        } else {
            factory.get_typedef(&ct, decl.get_identifier(), 0, 0)?;
        }
    } else {
        // A bare struct/union/enum declaration is treated as a typedef: the stub
        // was already interned during the construction action (newStruct/...), so
        // the C++ does nothing further here.  Any other base meta-type is an error.
        let base = decl
            .get_base_type()
            .ok_or_else(|| KunaError::lowlevel("Not sure what to do with this type"))?;
        let meta = base.get_metatype();
        let recognised = meta == type_metatype::TYPE_STRUCT
            || meta == type_metatype::TYPE_UNION
            || base.is_enum_type();
        if !recognised {
            return Err(KunaError::lowlevel("Not sure what to do with this type"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
