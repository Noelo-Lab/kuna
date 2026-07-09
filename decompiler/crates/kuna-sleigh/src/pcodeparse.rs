//! Port of `decompiler/cpp/pcodeparse.{cc,hh}` (grammar `pcodeparse.y`)
//! (item `w2-sleigh-pcodeparse`): the runtime p-code **snippet** parser that
//! compiles the `<pcode>` injection bodies found in `.cspec`/`.pspec`
//! constructor/callotherfixup/callfixup definitions.
//!
//! This is part of the oracle runtime (`decomp_dbg`/`decomp_test_dbg` link
//! it), not the build-time SLEIGH compiler, so it must reproduce the C++
//! behavior exactly.
//!
//! ## Shape of the port
//!
//! - **Lexer** ([`PcodeLexer`]): the C++ hand-written `PcodeLexer` is a
//!   character-by-character state machine (`moveState`) with a two-character
//!   lookahead; it is transcribed state-for-state, including the `idents`
//!   binary-search keyword table and the `s`/`f`-prefixed multi-character
//!   operator recognition. The lexer reads from a byte slice (the C++
//!   `istream`); `'\0'` is the end-of-stream sentinel exactly as in C++.
//! - **Parser** ([`PcodeSnippet::parse_stream`]): the C++ parser is
//!   bison-generated (`pcodeparse.cc` LALR tables); per **LOSS-006** there is
//!   no bison in the Rust toolchain, so it is replaced by a hand-written
//!   recursive-descent parser. The grammar is `pcodeparse.y`; this port
//!   reproduces (a) the same `PcodeCompile` calls in the same order, (b) the
//!   operator precedence/associativity from the `%left`/`%right`/`%nonassoc`
//!   declarations, (c) the two documented shift/reduce conflict resolutions
//!   (`%expect 3`): `':'` binds to `INTEGER` (integervarnode) by shifting,
//!   and `STRING` after `=` is a temporary declaration by shifting, and
//!   (d) the error text emitted by the `yyerror(...)` grammar actions
//!   wherever it is observable through `firsterror`.
//! - **`getVarnode()` resolver** ([`get_varnode_tpl`]): the C++
//!   `SpecificSymbol::getVarnode()` virtuals (VarnodeSymbol / OperandSymbol /
//!   Start / End / Next2 / FlowDest / FlowRef) were deferred by the symbol
//!   wave (**LOSS-022**, refined by **LOSS-026**); this item implements them
//!   as a free function over the resolved [`SnippetSymbol`], matching the
//!   per-kind `(space, offset, size)` `ConstTpl` triples of slghsymbol.cc.
//!   The const space the address-symbols (Start/End/Next2/FlowDest/FlowRef)
//!   carry is, in every construction site (slgh_compile.cc and
//!   `PcodeSnippet`'s own ctor), `sleigh->getConstantSpace()` — the same
//!   space the snippet holds — so the resolver uses the snippet constant
//!   space, byte-equivalent to reading the (private) `const_space` member.
//!
//! ## The language boundary
//!
//! C++ `PcodeSnippet` holds a `const SleighBase *sleigh` and calls only
//! `sleigh->findSymbol(name)`, `sleigh->numSpaces()`, `sleigh->getSpace(i)`,
//! `sleigh->getDefaultCodeSpace()`, `sleigh->getConstantSpace()`,
//! `sleigh->getUniqueSpace()` on it. `sleighbase.rs` is still a stub, so —
//! following the W1/W2 boundary convention — those operations are abstracted
//! behind the [`SnippetLanguage`] trait, implemented by the decode-engine
//! wave (`SleighBase`) and synthetically by tests. `findSymbol` returns a
//! resolved [`SnippetSymbol`] (the parser-relevant projection of the C++
//! `SleighSymbol *` switch in `PcodeSnippet::lex`).
//!
//! ## Local symbol scope
//!
//! C++ keeps a `SymbolTree tree` of `SleighSymbol *` (temporaries, operands,
//! and `LabelSymbol`s) sorted by name. The port keeps a `BTreeMap<Vec<u8>,
//! SnippetLocal>` (the `SymbolTree` comparator is byte-string name order;
//! see [`crate::slghsymbol`]). It holds the snippet-local definitions: the
//! `VarnodeSymbol`s `newOutput`/`newLocalDefinition` create, the
//! `OperandSymbol`s `addOperand` adds, and the `LabelSymbol`s
//! `defineLabel` creates. `inst_dest`/`inst_ref` are added by the ctor.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::Wrap;
use kuna_num::opcodes::OpCode;

use crate::pcodecompile::{
    ExprTree, LabelSymbol, Location, PcodeCompile, PcodeCompileSymbol, StarQuality,
};
use crate::semantics::{ConstTpl, ConstType, ConstructTpl, OpTpl, VarnodeTpl};
use crate::slghsymbol::{SleighSymbol, SymbolType, UserOpSymbol, VarnodeSymbol};

// ---------------------------------------------------------------------------
// Tokens (mirror of the bison %token set, with carried values)
// ---------------------------------------------------------------------------

/// A lexed token. Mirrors the bison terminal set of `pcodeparse.y`: the
/// keyword/operator terminals are unit variants; `INTEGER`/`STRING` carry
/// their value; the symbol terminals (`SPACESYM`/`USEROPSYM`/`VARSYM`/
/// `OPERANDSYM`/`JUMPSYM`/`LABELSYM`) carry the resolved [`SnippetSymbol`].
/// `Endofstream` is the explicit `ENDOFSTREAM` terminal `rtl` ends on;
/// `Eof` is the bison "0 = end of file" the parser stops at.
#[derive(Debug, Clone)]
enum Token {
    // multi-character operators
    BoolOr,
    BoolAnd,
    BoolXor,
    Equal,
    NotEqual,
    Fequal,
    Fnotequal,
    Greatequal,
    Lessequal,
    Sless,
    Sgreatequal,
    Slessequal,
    Sgreat,
    Fless,
    Fgreat,
    Flessequal,
    Fgreatequal,
    Left,
    Right,
    Sright,
    Fadd,
    Fsub,
    Sdiv,
    Srem,
    Fmult,
    Fdiv,
    // named unary p-code functions
    Zext,
    Carry,
    Borrow,
    Sext,
    Scarry,
    Sborrow,
    Nan,
    Abs,
    Sqrt,
    Ceil,
    Floor,
    Round,
    Int2float,
    Float2float,
    Trunc,
    /// `OP_NEW`: the grammar has `OP_NEW '(' expr ...)` productions, but the
    /// C++ lexer's `idents[]` table has **no** `"new"` entry, so the token is
    /// never produced from a snippet (an upstream dead path, faithfully
    /// preserved: `new` lexes as a STRING/identifier instead). Kept so the
    /// grammar's OP_NEW reductions are transcribed; unreachable like upstream.
    #[allow(dead_code)]
    New,
    // keywords
    BadInteger,
    GotoKey,
    CallKey,
    ReturnKey,
    IfKey,
    Endofstream,
    LocalKey,
    // values
    Integer(u64),
    Str(Vec<u8>),
    // resolved symbols
    SpaceSym(SnippetSymbol),
    UserOpSym(SnippetSymbol),
    VarSym(SnippetSymbol),
    OperandSym(SnippetSymbol),
    JumpSym(SnippetSymbol),
    LabelSym(Rc<LabelSymbol>),
    // single-character punctuation (the bison literal-char terminals)
    Char(u8),
    // bison "0" end of file
    Eof,
}

// ---------------------------------------------------------------------------
// Resolved symbols (the projection PcodeSnippet::lex pulls from a SleighSymbol)
// ---------------------------------------------------------------------------

/// The varnode-bearing symbol kinds the snippet parser can encounter
/// (`PcodeSnippet::lex`'s `switch(sym->getType())`). Each carries exactly
/// what its `getVarnode()` template needs (slghsymbol.cc) and the symbol's
/// declared **name** (C++ `SleighSymbol::getName()`, used by the
/// `createBitRange`/`LOCAL` redefinition error strings and by
/// `PcodeSnippet::lex`'s STRING fall-through). `SpaceSym`/`UserOpSym` carry no
/// varnode (they're consumed as grammar terminals).
#[derive(Debug, Clone)]
pub enum SnippetSymbol {
    /// C++ `SpaceSymbol` (carries the address space).
    Space(Rc<AddrSpace>),
    /// C++ `UserOpSymbol`.
    UserOp(UserOpSymbol),
    /// C++ `VarnodeSymbol` (a global register), plus its name.
    Varnode(VarnodeSymbol, Vec<u8>),
    /// C++ `OperandSymbol`: only the handle index (`hand`) matters for the
    /// snippet's `getVarnode`; plus its name.
    Operand(i32, Vec<u8>),
    /// C++ `StartSymbol` (`inst_start`), plus its name.
    Start(Vec<u8>),
    /// C++ `EndSymbol` (`inst_next`), plus its name.
    End(Vec<u8>),
    /// C++ `Next2Symbol` (`inst_next2`), plus its name.
    Next2(Vec<u8>),
    /// C++ `FlowDestSymbol` (`inst_dest`), plus its name.
    FlowDest(Vec<u8>),
    /// C++ `FlowRefSymbol` (`inst_ref`), plus its name.
    FlowRef(Vec<u8>),
}

impl SnippetSymbol {
    /// C++ `SleighSymbol::getName()` over the resolved snippet symbol.
    fn name(&self) -> &[u8] {
        match self {
            SnippetSymbol::Space(spc) => {
                // SpaceSymbol's name is the space name; this arm is never used
                // by getName() call sites (Space never reaches them), so a
                // borrow of the space name is fine.
                spc.get_name().as_bytes()
            }
            SnippetSymbol::UserOp(_) => b"",
            SnippetSymbol::Varnode(_, nm)
            | SnippetSymbol::Operand(_, nm)
            | SnippetSymbol::Start(nm)
            | SnippetSymbol::End(nm)
            | SnippetSymbol::Next2(nm)
            | SnippetSymbol::FlowDest(nm)
            | SnippetSymbol::FlowRef(nm) => nm,
        }
    }
}

/// C++ `SpecificSymbol::getVarnode()` (the runtime virtuals), resolved over a
/// [`SnippetSymbol`]. `constant_space` stands in for the symbol's
/// `const_space` member (always the snippet/language constant space; see
/// module docs).
///
/// Returns `None` for the kinds that have no `getVarnode()` (Space/UserOp);
/// the grammar never calls `getVarnode()` on those.
pub fn get_varnode_tpl(
    sym: &SnippetSymbol,
    constant_space: &Rc<AddrSpace>,
) -> Option<VarnodeTpl> {
    match sym {
        // C++ VarnodeSymbol::getVarnode:
        //   new VarnodeTpl(ConstTpl(fix.space),ConstTpl(real,fix.offset),
        //                  ConstTpl(real,fix.size))
        SnippetSymbol::Varnode(v, _) => {
            let fix = v.get_fixed_varnode();
            let space = fix
                .space
                .as_ref()
                .expect("VarnodeSymbol has no space (C++ dereferences fix.space)");
            Some(VarnodeTpl::new(
                ConstTpl::new_space(Rc::clone(space)),
                ConstTpl::new_real(ConstType::Real, fix.offset),
                ConstTpl::new_real(ConstType::Real, u64::from(fix.size)),
            ))
        }
        // C++ OperandSymbol::getVarnode: in the snippet, operands are added
        // via addOperand() with no defexp and no triple, so the switch always
        // takes the final `else` branch:  new VarnodeTpl(hand,false).
        // (The defexp / SpecificSymbol-triple / valuemap-name branches only
        // arise inside the SLEIGH compiler, never the snippet parser.)
        SnippetSymbol::Operand(hand, _) => Some(VarnodeTpl::new_from_handle(*hand, false)),
        // C++ StartSymbol::getVarnode: ConstTpl(const_space), j_start, zero
        SnippetSymbol::Start(_) => Some(VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(constant_space)),
            ConstTpl::new_type(ConstType::JStart),
            ConstTpl::new(),
        )),
        // C++ EndSymbol::getVarnode: ConstTpl(const_space), j_next, zero
        SnippetSymbol::End(_) => Some(VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(constant_space)),
            ConstTpl::new_type(ConstType::JNext),
            ConstTpl::new(),
        )),
        // C++ Next2Symbol::getVarnode: ConstTpl(const_space), j_next2, zero
        SnippetSymbol::Next2(_) => Some(VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(constant_space)),
            ConstTpl::new_type(ConstType::JNext2),
            ConstTpl::new(),
        )),
        // C++ FlowDestSymbol::getVarnode: ConstTpl(const_space), j_flowdest
        SnippetSymbol::FlowDest(_) => Some(VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(constant_space)),
            ConstTpl::new_type(ConstType::JFlowdest),
            ConstTpl::new(),
        )),
        // C++ FlowRefSymbol::getVarnode: ConstTpl(const_space), j_flowref
        SnippetSymbol::FlowRef(_) => Some(VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(constant_space)),
            ConstTpl::new_type(ConstType::JFlowref),
            ConstTpl::new(),
        )),
        SnippetSymbol::Space(_) | SnippetSymbol::UserOp(_) => None,
    }
}

// ---------------------------------------------------------------------------
// The language boundary (SleighBase, reduced to what PcodeSnippet uses)
// ---------------------------------------------------------------------------

/// Stand-in for `const SleighBase *sleigh`, reduced to the surface
/// `PcodeSnippet` pulls from it (`sleighbase.rs` is still a stub; W1/W2 boundary
/// convention). Implemented by the decode-engine wave's `SleighBase` and by
/// tests.
pub trait SnippetLanguage {
    /// C++ `sleigh->findSymbol(name)`, classified into a [`SnippetSymbol`].
    /// Returns `None` for an unknown name, OR for a symbol whose type is not
    /// one the snippet parser keeps visible (the `dummy_symbol`/`default`
    /// arms of `PcodeSnippet::lex`'s switch fall through to `STRING`, i.e.
    /// "not a special symbol").
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<SnippetSymbol>;
    /// C++ `sleigh->getDefaultCodeSpace()`.
    fn get_default_code_space(&self) -> Rc<AddrSpace>;
    /// C++ `sleigh->getConstantSpace()`.
    fn get_constant_space(&self) -> Rc<AddrSpace>;
    /// C++ `sleigh->getUniqueSpace()`.
    fn get_unique_space(&self) -> Rc<AddrSpace>;
    /// C++ `sleigh->numSpaces()`.
    fn num_spaces(&self) -> i32;
    /// C++ `sleigh->getSpace(i)`.
    fn get_space(&self, i: i32) -> Option<Rc<AddrSpace>>;
}

// ---------------------------------------------------------------------------
// PcodeLexer (faithful transcription of the C++ state machine)
// ---------------------------------------------------------------------------

/// The lexer state machine result of `moveState` and the token classifier of
/// `getNextToken` (C++ `PcodeLexer::enum` and the token-kind dispatch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexState {
    Start,
    Special2,
    Special3,
    Special32,
    Comment,
    Punctuation,
    Identifier,
    Hexstring,
    Decstring,
    Endstream,
    Illegal,
}

/// A lexed lexeme kind out of `getNextToken` before symbol classification.
enum Lexeme {
    /// A keyword/operator identifier that matched the `idents` table (C++
    /// `getNextToken` returns the keyword token directly, before
    /// `PcodeSnippet::lex`'s symbol classification).
    Keyword(Token),
    /// A non-keyword identifier (`curtoken`); the snippet classifies it as a
    /// symbol or STRING.
    Identifier(Vec<u8>),
    /// A parsed integer.
    Integer(u64),
    /// A malformed integer literal (`BADINTEGER`).
    BadInteger,
    /// The explicit `ENDOFSTREAM` terminal.
    Endofstream,
    /// bison "0 = end of file" (also covers the illegal-char path, which C++
    /// `getNextToken` likewise reports as `0`).
    Eof,
    /// A single punctuation character.
    Char(u8),
}

/// C++ `IdentRec idents[]`: a keyword name and its token. Sorted by name; the
/// lexer binary-searches it.
struct IdentRec {
    nm: &'static [u8],
    tok: fn() -> Token,
}

/// C++ `PcodeLexer::idents[]` — the **sorted** keyword/operator table.
/// `findIdentifier` binary-searches it (the order must stay byte-sorted).
const IDENTS: &[IdentRec] = &[
    IdentRec { nm: b"!=", tok: || Token::NotEqual },
    IdentRec { nm: b"&&", tok: || Token::BoolAnd },
    IdentRec { nm: b"<<", tok: || Token::Left },
    IdentRec { nm: b"<=", tok: || Token::Lessequal },
    IdentRec { nm: b"==", tok: || Token::Equal },
    IdentRec { nm: b">=", tok: || Token::Greatequal },
    IdentRec { nm: b">>", tok: || Token::Right },
    IdentRec { nm: b"^^", tok: || Token::BoolXor },
    IdentRec { nm: b"||", tok: || Token::BoolOr },
    IdentRec { nm: b"abs", tok: || Token::Abs },
    IdentRec { nm: b"borrow", tok: || Token::Borrow },
    IdentRec { nm: b"call", tok: || Token::CallKey },
    IdentRec { nm: b"carry", tok: || Token::Carry },
    IdentRec { nm: b"ceil", tok: || Token::Ceil },
    IdentRec { nm: b"f!=", tok: || Token::Fnotequal },
    IdentRec { nm: b"f*", tok: || Token::Fmult },
    IdentRec { nm: b"f+", tok: || Token::Fadd },
    IdentRec { nm: b"f-", tok: || Token::Fsub },
    IdentRec { nm: b"f/", tok: || Token::Fdiv },
    IdentRec { nm: b"f<", tok: || Token::Fless },
    IdentRec { nm: b"f<=", tok: || Token::Flessequal },
    IdentRec { nm: b"f==", tok: || Token::Fequal },
    IdentRec { nm: b"f>", tok: || Token::Fgreat },
    IdentRec { nm: b"f>=", tok: || Token::Fgreatequal },
    IdentRec { nm: b"float2float", tok: || Token::Float2float },
    IdentRec { nm: b"floor", tok: || Token::Floor },
    IdentRec { nm: b"goto", tok: || Token::GotoKey },
    IdentRec { nm: b"if", tok: || Token::IfKey },
    IdentRec { nm: b"int2float", tok: || Token::Int2float },
    IdentRec { nm: b"local", tok: || Token::LocalKey },
    IdentRec { nm: b"nan", tok: || Token::Nan },
    IdentRec { nm: b"return", tok: || Token::ReturnKey },
    IdentRec { nm: b"round", tok: || Token::Round },
    IdentRec { nm: b"s%", tok: || Token::Srem },
    IdentRec { nm: b"s/", tok: || Token::Sdiv },
    IdentRec { nm: b"s<", tok: || Token::Sless },
    IdentRec { nm: b"s<=", tok: || Token::Slessequal },
    IdentRec { nm: b"s>", tok: || Token::Sgreat },
    IdentRec { nm: b"s>=", tok: || Token::Sgreatequal },
    IdentRec { nm: b"s>>", tok: || Token::Sright },
    IdentRec { nm: b"sborrow", tok: || Token::Sborrow },
    IdentRec { nm: b"scarry", tok: || Token::Scarry },
    IdentRec { nm: b"sext", tok: || Token::Sext },
    IdentRec { nm: b"sqrt", tok: || Token::Sqrt },
    IdentRec { nm: b"trunc", tok: || Token::Trunc },
    IdentRec { nm: b"zext", tok: || Token::Zext },
];

/// C++ `PcodeLexer`: the snippet lexer. Reads bytes from an in-memory buffer
/// (the C++ `istream`) with a two-character lookahead.
struct PcodeLexer<'a> {
    curstate: LexState,
    curchar: u8,
    lookahead1: u8,
    lookahead2: u8,
    curtoken: Vec<u8>,
    // C++ reads the stream via s->get(c); the port indexes a byte slice and
    // tracks an explicit cursor + end-of-stream flag (the istream's eof).
    buf: &'a [u8],
    pos: usize,
    endofstream: bool,
    endofstreamsent: bool,
}

impl<'a> PcodeLexer<'a> {
    /// C++ `initialize`: buffer the first two characters.
    fn initialize(buf: &'a [u8]) -> PcodeLexer<'a> {
        let mut lex = PcodeLexer {
            curstate: LexState::Start,
            curchar: 0,
            lookahead1: 0,
            lookahead2: 0,
            curtoken: Vec::new(),
            buf,
            pos: 0,
            endofstream: false,
            endofstreamsent: false,
        };
        // s->get(lookahead1)
        match lex.stream_get() {
            Some(c) => lex.lookahead1 = c,
            None => {
                lex.endofstream = true;
                lex.lookahead1 = 0;
                return lex;
            }
        }
        // s->get(lookahead2)
        match lex.stream_get() {
            Some(c) => lex.lookahead2 = c,
            None => {
                lex.endofstream = true;
                lex.lookahead2 = 0;
            }
        }
        lex
    }

    /// C++ `s->get(c)` returning success: read one byte from the buffer.
    fn stream_get(&mut self) -> Option<u8> {
        if self.pos < self.buf.len() {
            let c = self.buf[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    fn start_token(&mut self) {
        // C++ curtoken[0] = curchar; tokpos = 1;
        self.curtoken.clear();
        self.curtoken.push(self.curchar);
    }

    fn advance_token(&mut self) {
        // C++ curtoken[tokpos++] = curchar;
        self.curtoken.push(self.curchar);
    }

    // C++ isIdent / isHex / isDec (ctype on a char; C++ isalnum etc treat the
    // arg as unsigned char). We restrict to ASCII like the .sla identifiers.
    fn is_ident(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_' || c == b'.'
    }
    fn is_hex(c: u8) -> bool {
        c.is_ascii_hexdigit()
    }
    fn is_dec(c: u8) -> bool {
        c.is_ascii_digit()
    }

    /// C++ `findIdentifier`: binary search the sorted keyword table; returns
    /// the index, or `None` (`-1`).
    fn find_identifier(str_: &[u8]) -> Option<usize> {
        // C++ do/while binary search over int4 indices; replicated faithfully
        // (i32 low/high so the `targ = (low+high)/2` arithmetic matches).
        let mut low: i32 = 0;
        let mut high: i32 = (IDENTS.len() as i32) - 1;
        loop {
            let targ = (low + high) / 2;
            // C++ str.compare(idents[targ].nm): byte-lexicographic order
            let comp = str_.cmp(IDENTS[targ as usize].nm);
            match comp {
                std::cmp::Ordering::Less => high = targ - 1,
                std::cmp::Ordering::Greater => low = targ + 1,
                std::cmp::Ordering::Equal => return Some(targ as usize),
            }
            if low > high {
                break;
            }
        }
        None
    }

    /// C++ `PcodeLexer::moveState`: one step of the character state machine.
    /// Returns the next [`LexState`] (the C++ `int4` return = the state value
    /// that token classification keys on).
    fn move_state(&mut self) -> LexState {
        match self.curstate {
            LexState::Start => self.move_state_start(),
            LexState::Special2 => {
                self.advance_token();
                self.curstate = LexState::Start;
                LexState::Identifier
            }
            LexState::Special3 => {
                self.advance_token();
                self.curstate = LexState::Special32;
                LexState::Start
            }
            LexState::Special32 => {
                self.advance_token();
                self.curstate = LexState::Start;
                LexState::Identifier
            }
            LexState::Comment => {
                if self.curchar == b'\n' {
                    self.curstate = LexState::Start;
                } else if self.curchar == b'\0' {
                    self.curstate = LexState::Endstream;
                    return LexState::Endstream;
                }
                LexState::Start
            }
            LexState::Identifier => {
                self.advance_token();
                if Self::is_ident(self.lookahead1) {
                    return LexState::Start;
                }
                self.curstate = LexState::Start;
                LexState::Identifier
            }
            LexState::Hexstring => {
                self.advance_token();
                if Self::is_hex(self.lookahead1) {
                    return LexState::Start;
                }
                self.curstate = LexState::Start;
                LexState::Hexstring
            }
            LexState::Decstring => {
                self.advance_token();
                if Self::is_dec(self.lookahead1) {
                    return LexState::Start;
                }
                self.curstate = LexState::Start;
                LexState::Decstring
            }
            // C++ default: curstate = endstream; return endstream;
            LexState::Endstream | LexState::Punctuation | LexState::Illegal => {
                self.curstate = LexState::Endstream;
                LexState::Endstream
            }
        }
    }

    /// The `case start:` branch of `moveState` (the bulk of the machine).
    fn move_state_start(&mut self) -> LexState {
        match self.curchar {
            b'#' => {
                self.curstate = LexState::Comment;
                LexState::Start
            }
            b'|' => {
                if self.lookahead1 == b'|' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'&' => {
                if self.lookahead1 == b'&' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'^' => {
                if self.lookahead1 == b'^' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'>' => {
                if self.lookahead1 == b'>' || self.lookahead1 == b'=' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'<' => {
                if self.lookahead1 == b'<' || self.lookahead1 == b'=' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'=' => {
                if self.lookahead1 == b'=' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'!' => {
                if self.lookahead1 == b'=' {
                    self.start_token();
                    self.curstate = LexState::Special2;
                    return LexState::Start;
                }
                LexState::Punctuation
            }
            b'(' | b')' | b',' | b':' | b'[' | b']' | b';' | b'+' | b'-' | b'*' | b'/' | b'%'
            | b'~' => LexState::Punctuation,
            b's' | b'f' => self.move_state_sf(),
            // identifier-start characters (the C++ alpha/_/. cases that fall
            // through from 's'/'f'); 's'/'f' that did not match an operator
            // also fall through here via move_state_sf returning the
            // identifier path.
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'.' => self.start_identifier(),
            b'0' => {
                self.start_token();
                if self.lookahead1 == b'x' {
                    self.curstate = LexState::Hexstring;
                    return LexState::Start;
                }
                if Self::is_dec(self.lookahead1) {
                    self.curstate = LexState::Decstring;
                    return LexState::Start;
                }
                self.curstate = LexState::Start;
                LexState::Decstring
            }
            b'1'..=b'9' => {
                self.start_token();
                if Self::is_dec(self.lookahead1) {
                    self.curstate = LexState::Decstring;
                    return LexState::Start;
                }
                self.curstate = LexState::Start;
                LexState::Decstring
            }
            b'\n' | b' ' | b'\t' | 0x0b | b'\r' => LexState::Start, // whitespace
            b'\0' => {
                self.curstate = LexState::Endstream;
                LexState::Endstream
            }
            _ => {
                self.curstate = LexState::Illegal;
                LexState::Illegal
            }
        }
    }

    /// The `case 's': case 'f':` block of `moveState` (the multi-character
    /// signed/float operator recognition that falls through to identifier).
    fn move_state_sf(&mut self) -> LexState {
        if self.curchar == b's' {
            if self.lookahead1 == b'/' || self.lookahead1 == b'%' {
                self.start_token();
                self.curstate = LexState::Special2;
                return LexState::Start;
            } else if self.lookahead1 == b'<' {
                self.start_token();
                if self.lookahead2 == b'=' {
                    self.curstate = LexState::Special3;
                } else {
                    self.curstate = LexState::Special2;
                }
                return LexState::Start;
            } else if self.lookahead1 == b'>' {
                self.start_token();
                if self.lookahead2 == b'>' || self.lookahead2 == b'=' {
                    self.curstate = LexState::Special3;
                } else {
                    self.curstate = LexState::Special2;
                }
                return LexState::Start;
            }
        } else {
            // curchar == 'f'
            if self.lookahead1 == b'+'
                || self.lookahead1 == b'-'
                || self.lookahead1 == b'*'
                || self.lookahead1 == b'/'
            {
                self.start_token();
                self.curstate = LexState::Special2;
                return LexState::Start;
            } else if (self.lookahead1 == b'=' || self.lookahead1 == b'!')
                && self.lookahead2 == b'='
            {
                self.start_token();
                self.curstate = LexState::Special3;
                return LexState::Start;
            } else if self.lookahead1 == b'<' || self.lookahead1 == b'>' {
                self.start_token();
                if self.lookahead2 == b'=' {
                    self.curstate = LexState::Special3;
                } else {
                    self.curstate = LexState::Special2;
                }
                return LexState::Start;
            }
        }
        // fall through: treat 's'/'f' as ordinary identifier characters
        self.start_identifier()
    }

    /// The identifier-start tail shared by the alpha cases and the 's'/'f'
    /// fall-through.
    fn start_identifier(&mut self) -> LexState {
        self.start_token();
        if Self::is_ident(self.lookahead1) {
            self.curstate = LexState::Identifier;
            return LexState::Start;
        }
        self.curstate = LexState::Start;
        LexState::Identifier
    }

    /// C++ `PcodeLexer::getNextToken`: drive the state machine until a token
    /// boundary, then classify (identifier/keyword/number/end/eof/char).
    fn get_next_token(&mut self) -> Lexeme {
        let tok;
        loop {
            // curchar = lookahead1; lookahead1 = lookahead2; refill lookahead2
            self.curchar = self.lookahead1;
            self.lookahead1 = self.lookahead2;
            if self.endofstream {
                self.lookahead2 = b'\0';
            } else {
                match self.stream_get() {
                    Some(c) => self.lookahead2 = c,
                    None => {
                        self.endofstream = true;
                        self.lookahead2 = b'\0';
                    }
                }
            }
            let t = self.move_state();
            if t != LexState::Start {
                tok = t;
                break;
            }
        }
        match tok {
            LexState::Identifier => {
                // C++ getNextToken: findIdentifier(curidentifier); if a
                // keyword/operator, return its token, else STRING (here a raw
                // Identifier the snippet classifies as a symbol/STRING).
                let name = std::mem::take(&mut self.curtoken);
                match Self::find_identifier(&name) {
                    Some(idx) => Lexeme::Keyword((IDENTS[idx].tok)()),
                    None => Lexeme::Identifier(name),
                }
            }
            LexState::Hexstring | LexState::Decstring => {
                // C++ istringstream with unsetf(dec|hex|oct): parse by prefix
                // (0x => hex, leading 0 => octal, else decimal); the prefix is
                // captured in curtoken exactly as the C++ stream sees it.
                match parse_number(&self.curtoken) {
                    Some(n) => Lexeme::Integer(n),
                    None => Lexeme::BadInteger,
                }
            }
            LexState::Endstream => {
                if !self.endofstreamsent {
                    self.endofstreamsent = true;
                    Lexeme::Endofstream
                } else {
                    Lexeme::Eof
                }
            }
            LexState::Illegal => Lexeme::Eof, // C++ returns 0
            // punctuation: C++ `return (int4)curchar`
            _ => Lexeme::Char(self.curchar),
        }
    }
}

/// C++ `istringstream >> uintb` with `unsetf(dec|hex|oct)`.
///
/// The base is detected from the literal's prefix (`0x`/`0X` hex, leading `0`
/// octal, otherwise decimal), then the stream reads the **longest valid prefix**
/// of digits in that base and STOPS at the first digit invalid for the base
/// *without* failing — extraction only sets `failbit` (=> `None` => `BADINTEGER`)
/// when zero digits could be consumed or the parsed prefix overflows `uintb`.
///
/// This partial-parse matters for octal tokens that contain a non-octal digit:
/// the `decstring` lexer state admits any `0-9`, so `08`/`0789` reach this
/// function as single tokens. The C++ stream yields `08 -> 0`, `0789 -> 7`,
/// `019 -> 1`, `0178 -> 15` (verified against a standalone `istringstream`
/// program). `u64::from_str_radix(_, 8)` instead requires every digit valid and
/// would reject the whole token (F1).
fn parse_number(tok: &[u8]) -> Option<u64> {
    let s = std::str::from_utf8(tok).ok()?;
    // Detect base exactly as the unset stream does. For octal the leading `0`
    // is itself the first (value-0) digit, so `digits` keeps it; for hex the
    // `0x`/`0X` prefix is consumed and the digits follow it.
    let (radix, digits) = if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))
    {
        (16u64, rest)
    } else if s.len() > 1 && s.starts_with('0') {
        (8u64, s)
    } else {
        (10u64, s)
    };
    // Consume the longest prefix of valid digits in `radix`, accumulating with
    // overflow detection (the stream succeeds on a partial prefix but fails the
    // whole token on overflow, clamping to max + setting failbit).
    let mut value: u64 = 0;
    let mut consumed = 0usize;
    for &b in digits.as_bytes() {
        let digit = match b {
            b'0'..=b'9' => (b - b'0') as u64,
            b'a'..=b'f' => (b - b'a') as u64 + 10,
            b'A'..=b'F' => (b - b'A') as u64 + 10,
            _ => break, // non-alphanumeric: never reaches here (lexer-filtered)
        };
        if digit >= radix {
            break; // first digit invalid for the base: stream stops here
        }
        value = value.checked_mul(radix)?.checked_add(digit)?; // overflow => fail
        consumed += 1;
    }
    if consumed == 0 {
        // No valid digit consumed: e.g. an empty `0x` token => failbit => BADINTEGER.
        return None;
    }
    Some(value)
}

// ---------------------------------------------------------------------------
// PcodeSnippet local scope
// ---------------------------------------------------------------------------

/// A snippet-local symbol (the `SymbolTree` entries). Varnode/operand symbols
/// are kept as resolved [`SnippetSymbol`]s (they classify the same way a
/// language symbol does in `lex`); labels are `Rc<LabelSymbol>`.
#[derive(Debug, Clone)]
enum SnippetLocal {
    Symbol(SnippetSymbol),
    Label(Rc<LabelSymbol>),
}

// ---------------------------------------------------------------------------
// PcodeSnippet (PcodeCompile implementor + the parser driver)
// ---------------------------------------------------------------------------

/// C++ `PcodeSnippet`: compiles a standalone p-code snippet against an
/// existing SLEIGH language. Implements [`PcodeCompile`].
pub struct PcodeSnippet<'a> {
    sleigh: &'a dyn SnippetLanguage,
    // PcodeCompile data members
    defaultspace: Option<Rc<AddrSpace>>,
    constantspace: Option<Rc<AddrSpace>>,
    uniqspace: Option<Rc<AddrSpace>>,
    local_labelcount: u32,
    enforce_local_key: bool,
    // PcodeSnippet members
    tree: BTreeMap<Vec<u8>, SnippetLocal>,
    tempbase: u32,
    errorcount: i32,
    firsterror: String,
    result: Option<ConstructTpl>,
}

impl<'a> PcodeSnippet<'a> {
    /// C++ `PcodeSnippet(const SleighBase *slgh)`.
    pub fn new(slgh: &'a dyn SnippetLanguage) -> PcodeSnippet<'a> {
        let mut snip = PcodeSnippet {
            sleigh: slgh,
            defaultspace: Some(slgh.get_default_code_space()),
            constantspace: Some(slgh.get_constant_space()),
            uniqspace: Some(slgh.get_unique_space()),
            local_labelcount: 0,
            enforce_local_key: false,
            tree: BTreeMap::new(),
            tempbase: 0,
            errorcount: 0,
            firsterror: String::new(),
            result: None,
        };
        // Register every constant/processor/spacebase/internal space as a
        // SpaceSymbol (C++ loops numSpaces()).
        let num = slgh.num_spaces();
        for i in 0..num {
            if let Some(spc) = slgh.get_space(i) {
                let ty = spc.get_type();
                if ty == spacetype::IPTR_CONSTANT
                    || ty == spacetype::IPTR_PROCESSOR
                    || ty == spacetype::IPTR_SPACEBASE
                    || ty == spacetype::IPTR_INTERNAL
                {
                    // SpaceSymbol uses the space's own name.
                    snip.tree.insert(
                        get_space_name(&spc),
                        SnippetLocal::Symbol(SnippetSymbol::Space(spc)),
                    );
                }
            }
        }
        // C++ addSymbol(new FlowDestSymbol("inst_dest",...)) / FlowRefSymbol.
        snip.add_local(
            b"inst_dest",
            SnippetLocal::Symbol(SnippetSymbol::FlowDest(b"inst_dest".to_vec())),
        );
        snip.add_local(
            b"inst_ref",
            SnippetLocal::Symbol(SnippetSymbol::FlowRef(b"inst_ref".to_vec())),
        );
        snip
    }

    /// C++ `PcodeSnippet::addSymbol`: insert into the local tree, reporting a
    /// duplicate-name error (the C++ `addSymbol(SleighSymbol *)` overload).
    fn add_local(&mut self, name: &[u8], local: SnippetLocal) {
        if self.tree.contains_key(name) {
            self.report_error(
                None,
                &format!(
                    "Duplicate symbol name: {}",
                    String::from_utf8_lossy(name)
                ),
            );
            return;
        }
        self.tree.insert(name.to_vec(), local);
    }

    /// C++ `setUniqueBase`.
    pub fn set_unique_base(&mut self, val: u32) {
        self.tempbase = val;
    }

    /// C++ `getUniqueBase`.
    pub fn get_unique_base(&self) -> u32 {
        self.tempbase
    }

    /// C++ `hasErrors`.
    pub fn has_errors(&self) -> bool {
        self.errorcount != 0
    }

    /// C++ `getErrorMessage`.
    pub fn get_error_message(&self) -> &str {
        &self.firsterror
    }

    /// C++ `releaseResult`: take the compiled template.
    pub fn release_result(&mut self) -> Option<ConstructTpl> {
        self.result.take()
    }

    /// C++ `addOperand(const string &name,int4 index)`: add an operand symbol
    /// for this snippet (`OperandSymbol(name,index,(Constructor *)0)` — no
    /// triple, no defexp; only the handle index is consulted by getVarnode).
    pub fn add_operand(&mut self, name: &[u8], index: i32) {
        self.add_local(
            name,
            SnippetLocal::Symbol(SnippetSymbol::Operand(index, name.to_vec())),
        );
    }

    /// C++ `clear`: reset for a new parse against the same language. Keeps the
    /// SpaceSymbols (and inst_dest/inst_ref, which C++ leaves too — only the
    /// non-space symbols are erased), drops the rest.
    pub fn clear(&mut self) {
        // C++ erases every non-space symbol (this drops inst_dest/inst_ref,
        // operands, locals, labels — matching the SleighSymbol::space_symbol
        // guard).
        self.tree
            .retain(|_, v| matches!(v, SnippetLocal::Symbol(SnippetSymbol::Space(_))));
        self.result = None;
        // C++ leaves tempbase as-is (the commented-out `tempbase = 0`).
        self.errorcount = 0;
        self.firsterror.clear();
        self.reset_label_count();
    }

    /// C++ `PcodeSnippet::lex` symbol classification: turn a lexed identifier
    /// into the right token (local tree first, then the language). Mirrors the
    /// `switch(sym->getType())` of `PcodeSnippet::lex`.
    fn classify_identifier(&self, name: &[u8]) -> Token {
        // First check the local scope (C++ tree.find), else sleigh->findSymbol.
        if let Some(local) = self.tree.get(name) {
            match local {
                SnippetLocal::Label(lab) => return Token::LabelSym(Rc::clone(lab)),
                SnippetLocal::Symbol(sym) => return symbol_to_token(sym.clone()),
            }
        }
        if let Some(sym) = self.sleigh.find_snippet_symbol(name) {
            return symbol_to_token(sym);
        }
        // Not a special symbol: STRING.
        Token::Str(name.to_vec())
    }

    /// Resolve a label by name from the local tree (used after a label is
    /// `defineLabel`/`placeLabel`-created and later referenced).
    fn snippet_constant_space(&self) -> Rc<AddrSpace> {
        self.constantspace
            .clone()
            .expect("PcodeSnippet constant space not set")
    }

    /// C++ `parseStream`: lex + parse the snippet, then propagate sizes.
    /// `true` on success; on failure the first error is in `firsterror`.
    pub fn parse_stream(&mut self, buf: &[u8]) -> bool {
        let lexer = PcodeLexer::initialize(buf);
        let mut parser = Parser::new(self, lexer);
        let ok = parser.parse();
        if !ok {
            // C++ reportError(0,"Syntax error") when yyparse() != 0.
            self.report_error(None, "Syntax error");
            return false;
        }
        let propagated = match self.result.as_mut() {
            Some(ct) => crate::pcodecompile::propagate_size(ct),
            // C++ propagateSize(null) would dereference null; an empty/None
            // result only arises after an internal error, treated as failure.
            None => Ok(false),
        };
        match propagated {
            Ok(true) => true,
            Ok(false) | Err(_) => {
                self.report_error(None, "Could not resolve at least 1 variable size");
                false
            }
        }
    }
}

/// SpaceSymbol uses the address space's name (C++ `SpaceSymbol(AddrSpace *)`
/// takes the space's name as the symbol name).
fn get_space_name(spc: &Rc<AddrSpace>) -> Vec<u8> {
    spc.get_name().as_bytes().to_vec()
}

/// Map a resolved [`SnippetSymbol`] to its grammar terminal (the type switch
/// in `PcodeSnippet::lex`).
fn symbol_to_token(sym: SnippetSymbol) -> Token {
    match sym {
        SnippetSymbol::Space(_) => Token::SpaceSym(sym),
        SnippetSymbol::UserOp(_) => Token::UserOpSym(sym),
        SnippetSymbol::Varnode(..) => Token::VarSym(sym),
        SnippetSymbol::Operand(..) => Token::OperandSym(sym),
        // start/end/next2/flowdest/flowref all map to JUMPSYM (the
        // SpecificSymbol case in lex's switch).
        SnippetSymbol::Start(_)
        | SnippetSymbol::End(_)
        | SnippetSymbol::Next2(_)
        | SnippetSymbol::FlowDest(_)
        | SnippetSymbol::FlowRef(_) => Token::JumpSym(sym),
    }
}

impl PcodeCompile for PcodeSnippet<'_> {
    fn get_default_space(&self) -> Option<Rc<AddrSpace>> {
        self.defaultspace.clone()
    }
    fn set_default_space(&mut self, spc: Rc<AddrSpace>) {
        self.defaultspace = Some(spc);
    }
    fn get_constant_space(&self) -> Option<Rc<AddrSpace>> {
        self.constantspace.clone()
    }
    fn set_constant_space(&mut self, spc: Rc<AddrSpace>) {
        self.constantspace = Some(spc);
    }
    fn get_unique_space(&self) -> Option<Rc<AddrSpace>> {
        self.uniqspace.clone()
    }
    fn set_unique_space(&mut self, spc: Rc<AddrSpace>) {
        self.uniqspace = Some(spc);
    }
    fn is_enforce_local_key(&self) -> bool {
        self.enforce_local_key
    }
    fn set_enforce_local_key(&mut self, val: bool) {
        self.enforce_local_key = val;
    }
    fn local_label_count(&self) -> u32 {
        self.local_labelcount
    }
    fn set_local_label_count(&mut self, val: u32) {
        self.local_labelcount = val;
    }

    /// C++ `PcodeSnippet::allocateTemp`: advance the unique base by 16.
    fn allocate_temp(&mut self) -> u32 {
        let res = self.tempbase;
        self.tempbase = self.tempbase.wadd(16);
        res
    }

    /// C++ `PcodeSnippet::addSymbol`.
    fn add_symbol(&mut self, sym: PcodeCompileSymbol) {
        match sym {
            PcodeCompileSymbol::Label(lab) => {
                let name = lab.get_name().to_vec();
                self.add_local(&name, SnippetLocal::Label(lab));
            }
            PcodeCompileSymbol::Sleigh(boxed) => {
                // newOutput/newLocalDefinition build a VarnodeSymbol; keep it
                // as a SnippetSymbol so later lexes resolve it like any global.
                let name = boxed.get_name().to_vec();
                let local = match sleigh_symbol_to_snippet(&boxed) {
                    Some(s) => SnippetLocal::Symbol(s),
                    // C++ only ever adds VarnodeSymbols here; an unexpected
                    // kind is an internal invariant violation.
                    None => panic!("PcodeSnippet::addSymbol got a non-varnode SleighSymbol"),
                };
                self.add_local(&name, local);
            }
        }
    }

    fn get_location(&self, _symbol_name: &[u8]) -> Option<Location> {
        None // C++ PcodeSnippet::getLocation returns null
    }

    /// C++ `PcodeSnippet::reportError`: keep the first message, count errors.
    fn report_error(&mut self, _loc: Option<&Location>, msg: &str) {
        if self.errorcount == 0 {
            self.firsterror = msg.to_string();
        }
        self.errorcount += 1;
    }

    fn report_warning(&mut self, _loc: Option<&Location>, _msg: &str) {}
}

/// Convert a freshly-built `VarnodeSymbol` (the only kind `addSymbol`
/// receives) into a [`SnippetSymbol`]. Returns `None` for any other kind.
fn sleigh_symbol_to_snippet(sym: &SleighSymbol) -> Option<SnippetSymbol> {
    match sym.get_type() {
        SymbolType::Varnode => match sym.kind() {
            crate::slghsymbol::SymbolKind::Varnode(v) => {
                Some(SnippetSymbol::Varnode(v.clone(), sym.get_name().to_vec()))
            }
            _ => None,
        },
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Recursive-descent parser (the bison grammar, hand-written per LOSS-006)
// ---------------------------------------------------------------------------

/// The hand-written recursive-descent parser. Drives [`PcodeLexer`] for
/// tokens and the [`PcodeSnippet`] (`PcodeCompile`) for the semantic actions,
/// reproducing the bison grammar's reductions in order. A `bool` flag tracks
/// the bison `YYERROR`/`yyparse()!=0` path: on a grammar error the parse
/// aborts and `parse()` returns `false`, just as `yyparse` returns non-zero.
struct Parser<'p, 'l, 's> {
    pcode: &'p mut PcodeSnippet<'s>,
    lexer: PcodeLexer<'l>,
    cur: Token,
    failed: bool,
}

impl<'p, 'l, 's> Parser<'p, 'l, 's> {
    fn new(pcode: &'p mut PcodeSnippet<'s>, lexer: PcodeLexer<'l>) -> Parser<'p, 'l, 's> {
        let mut p = Parser {
            pcode,
            lexer,
            cur: Token::Eof,
            failed: false,
        };
        p.advance();
        p
    }

    /// C++ `pcodelex()` -> `PcodeSnippet::lex()`: pull the next token,
    /// classifying identifiers via the snippet's symbol scope.
    fn advance(&mut self) {
        self.cur = match self.lexer.get_next_token() {
            Lexeme::Keyword(t) => t,
            Lexeme::Identifier(name) => self.pcode.classify_identifier(&name),
            Lexeme::Integer(n) => Token::Integer(n),
            Lexeme::BadInteger => Token::BadInteger,
            Lexeme::Endofstream => Token::Endofstream,
            Lexeme::Eof => Token::Eof,
            Lexeme::Char(c) => Token::Char(c),
        };
    }

    /// C++ `yyerror(msg)`: report through the snippet, mark the parse failed
    /// (the grammar actions follow yyerror with `YYERROR`/abort).
    fn yyerror(&mut self, msg: &str) {
        self.pcode.report_error(None, msg);
        self.failed = true;
    }

    fn is_char(&self, c: u8) -> bool {
        matches!(self.cur, Token::Char(ch) if ch == c)
    }

    /// Expect a specific punctuation char; on mismatch, fail (bison syntax
    /// error). Consumes it on success.
    fn expect_char(&mut self, c: u8) -> bool {
        if self.is_char(c) {
            self.advance();
            true
        } else {
            self.failed = true;
            false
        }
    }

    /// `rtl: rtlmid ENDOFSTREAM`. Returns false on any grammar error
    /// (`yyparse() != 0`).
    fn parse(&mut self) -> bool {
        let mut result = ConstructTpl::new();
        self.parse_rtlmid(&mut result);
        if self.failed {
            return false;
        }
        // rtl requires the ENDOFSTREAM terminal next.
        if !matches!(self.cur, Token::Endofstream) {
            return false;
        }
        self.pcode.result = Some(result);
        true
    }

    /// `rtlmid: /*EMPTY*/ | rtlmid statement | rtlmid LOCAL STRING ';'
    ///        | rtlmid LOCAL STRING ':' INTEGER ';'`
    fn parse_rtlmid(&mut self, result: &mut ConstructTpl) {
        loop {
            if self.failed {
                return;
            }
            // rtlmid ends at ENDOFSTREAM / EOF (the only follow of rtlmid in
            // `rtl: rtlmid ENDOFSTREAM`).
            if matches!(self.cur, Token::Endofstream | Token::Eof) {
                return;
            }
            if matches!(self.cur, Token::LocalKey) {
                // Could be a `local STRING ;` / `local STRING : INTEGER ;`
                // declaration (rtlmid) OR a `local ...` statement. Disambiguate
                // by looking past the STRING. The grammar's `local` statement
                // forms start `LOCAL STRING '=' ...` or `LOCAL specificsymbol
                // '=' ...` or `LOCAL STRING ':' INTEGER '=' ...`.
                if self.try_parse_local_decl_or_stmt(result) {
                    continue;
                }
                return;
            }
            // Otherwise it must be a `statement`.
            match self.parse_statement() {
                Some(ops) => {
                    // rtlmid statement: addOpList
                    if !result.add_op_list(ops) {
                        self.yyerror("Multiple delayslot declarations");
                        return;
                    }
                }
                None => return, // failed or genuine end
            }
        }
    }

    /// Handle a token stream beginning with `LOCAL_KEY`. Distinguishes the
    /// four `local`-leading productions:
    ///   rtlmid LOCAL STRING ';'
    ///   rtlmid LOCAL STRING ':' INTEGER ';'
    ///   statement: LOCAL STRING '=' expr ';'
    ///   statement: LOCAL STRING ':' INTEGER '=' expr ';'
    ///   statement: LOCAL specificsymbol '='   (redefinition error)
    /// Returns true if it consumed a full production (caller continues),
    /// false on error.
    fn try_parse_local_decl_or_stmt(&mut self, result: &mut ConstructTpl) -> bool {
        // consume LOCAL_KEY
        self.advance();
        // Next is STRING (a fresh name) or a specificsymbol (redefinition).
        match self.cur.clone() {
            Token::Str(name) => {
                self.advance();
                if self.is_char(b':') {
                    self.advance();
                    // LOCAL STRING ':' INTEGER ...
                    let size = match self.expect_integer() {
                        Some(v) => v,
                        None => return false,
                    };
                    if self.is_char(b';') {
                        self.advance();
                        // rtlmid LOCAL STRING ':' INTEGER ';'
                        self.pcode.new_local_definition(&name, size as u32);
                        return true;
                    }
                    if self.expect_char(b'=') {
                        // statement: LOCAL STRING ':' INTEGER '=' expr ';'
                        let rhs = match self.parse_expr(PREC_LOWEST) {
                            Some(e) => e,
                            None => return false,
                        };
                        if !self.expect_char(b';') {
                            return false;
                        }
                        match self.pcode.new_output(true, rhs, &name, size as u32) {
                            Ok(ops) => {
                                if !result.add_op_list(ops) {
                                    self.yyerror("Multiple delayslot declarations");
                                    return false;
                                }
                                return true;
                            }
                            Err(_) => {
                                self.failed = true;
                                return false;
                            }
                        }
                    }
                    return false;
                }
                if self.is_char(b';') {
                    self.advance();
                    // rtlmid LOCAL STRING ';'
                    self.pcode.new_local_definition(&name, 0);
                    return true;
                }
                if self.expect_char(b'=') {
                    // statement: LOCAL STRING '=' expr ';'
                    let rhs = match self.parse_expr(PREC_LOWEST) {
                        Some(e) => e,
                        None => return false,
                    };
                    if !self.expect_char(b';') {
                        return false;
                    }
                    match self.pcode.new_output(true, rhs, &name, 0) {
                        Ok(ops) => {
                            if !result.add_op_list(ops) {
                                self.yyerror("Multiple delayslot declarations");
                                return false;
                            }
                            return true;
                        }
                        Err(_) => {
                            self.failed = true;
                            return false;
                        }
                    }
                }
                false
            }
            // LOCAL specificsymbol '=' : redefinition error.
            Token::VarSym(_) | Token::OperandSym(_) | Token::JumpSym(_) => {
                let sym = self.cur.clone();
                self.advance();
                if self.is_char(b'=') {
                    let name = specificsymbol_name(&sym);
                    self.yyerror(&format!(
                        "Redefinition of symbol: {}",
                        String::from_utf8_lossy(&name)
                    ));
                    return false;
                }
                self.failed = true;
                false
            }
            _ => {
                self.failed = true;
                false
            }
        }
    }

    /// Parse a `statement` (one of the many forms). Returns the produced op
    /// vector, or `None` on error/abort. The leading token has already been
    /// classified; this dispatches on it.
    fn parse_statement(&mut self) -> Option<Vec<OpTpl>> {
        match self.cur.clone() {
            // label : '<' LABELSYM '>'  |  '<' STRING '>'
            // statement: label  { placeLabel }
            Token::Char(b'<') => {
                let lab = self.parse_label()?;
                Some(self.pcode.place_label(&lab))
            }
            // sizedstar expr '=' expr ';'  (store) — starts with '*'
            Token::Char(b'*') => {
                let qual = self.parse_sizedstar()?;
                let ptr = self.parse_expr(PREC_LOWEST)?;
                if !self.expect_char(b'=') {
                    return None;
                }
                let val = self.parse_expr(PREC_LOWEST)?;
                if !self.expect_char(b';') {
                    return None;
                }
                match self.pcode.create_store(qual, ptr, val) {
                    Ok(ops) => Some(ops),
                    Err(_) => {
                        self.failed = true;
                        None
                    }
                }
            }
            // USEROPSYM '(' paramlist ')' ';'  (no out)
            Token::UserOpSym(sym) => {
                self.advance();
                if !self.expect_char(b'(') {
                    return None;
                }
                let params = self.parse_paramlist()?;
                if !self.expect_char(b')') {
                    return None;
                }
                if !self.expect_char(b';') {
                    return None;
                }
                let usym = useropsym(&sym);
                Some(self.pcode.create_user_op_no_out(&usym, params))
            }
            Token::GotoKey => self.parse_goto_statement(),
            Token::CallKey => self.parse_call_statement(),
            Token::ReturnKey => self.parse_return_statement(),
            Token::IfKey => self.parse_if_statement(),
            // lhsvarnode-leading statements:
            //   lhsvarnode '=' expr ';'
            //   lhsvarnode '[' INTEGER ',' INTEGER ']' '=' expr ';'
            // specificsymbol can also start `varnode ':' INTEGER '='`
            //   (illegal-truncation) and `varnode '(' INTEGER ')'`
            //   (illegal-subpiece) error productions.
            Token::VarSym(_) | Token::OperandSym(_) | Token::JumpSym(_) => self.parse_lhs_statement(),
            // STRING-leading statements (the temporary-declaration / unknown-
            // varnode forms):
            //   STRING '=' expr ';'
            //   STRING ':' INTEGER '=' expr ';'
            //   (an unknown lhs varnode is an error inside lhsvarnode)
            Token::Str(name) => self.parse_string_statement(name),
            _ => {
                self.failed = true;
                None
            }
        }
    }

    /// `GOTO_KEY jumpdest ';' | GOTO_KEY '[' expr ']' ';'`
    fn parse_goto_statement(&mut self) -> Option<Vec<OpTpl>> {
        self.advance(); // GOTO_KEY
        if self.is_char(b'[') {
            self.advance();
            let e = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b']') {
                return None;
            }
            if !self.expect_char(b';') {
                return None;
            }
            return Some(self.pcode.create_op_no_out(OpCode::CPUI_BRANCHIND, e));
        }
        let jd = self.parse_jumpdest()?;
        if !self.expect_char(b';') {
            return None;
        }
        // createOpNoOut(CPUI_BRANCH, new ExprTree(jumpdest))
        Some(self.pcode.create_op_no_out(OpCode::CPUI_BRANCH, ExprTree::new(jd)))
    }

    /// `CALL_KEY jumpdest ';' | CALL_KEY '[' expr ']' ';'`
    fn parse_call_statement(&mut self) -> Option<Vec<OpTpl>> {
        self.advance(); // CALL_KEY
        if self.is_char(b'[') {
            self.advance();
            let e = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b']') {
                return None;
            }
            if !self.expect_char(b';') {
                return None;
            }
            return Some(self.pcode.create_op_no_out(OpCode::CPUI_CALLIND, e));
        }
        let jd = self.parse_jumpdest()?;
        if !self.expect_char(b';') {
            return None;
        }
        Some(self.pcode.create_op_no_out(OpCode::CPUI_CALL, ExprTree::new(jd)))
    }

    /// `RETURN_KEY ';'  (error) | RETURN_KEY '[' expr ']' ';'`
    fn parse_return_statement(&mut self) -> Option<Vec<OpTpl>> {
        self.advance(); // RETURN_KEY
        if self.is_char(b';') {
            self.advance();
            self.yyerror("Must specify an indirect parameter for return");
            return None;
        }
        if !self.expect_char(b'[') {
            return None;
        }
        let e = self.parse_expr(PREC_LOWEST)?;
        if !self.expect_char(b']') {
            return None;
        }
        if !self.expect_char(b';') {
            return None;
        }
        Some(self.pcode.create_op_no_out(OpCode::CPUI_RETURN, e))
    }

    /// `IF_KEY expr GOTO_KEY jumpdest ';'`
    fn parse_if_statement(&mut self) -> Option<Vec<OpTpl>> {
        self.advance(); // IF_KEY
        let cond = self.parse_expr(PREC_LOWEST)?;
        if !matches!(self.cur, Token::GotoKey) {
            self.failed = true;
            return None;
        }
        self.advance(); // GOTO_KEY
        let jd = self.parse_jumpdest()?;
        if !self.expect_char(b';') {
            return None;
        }
        // createOpNoOut(CPUI_CBRANCH, new ExprTree(jumpdest), cond)
        Some(self.pcode.create_op_no_out2(OpCode::CPUI_CBRANCH, ExprTree::new(jd), cond))
    }

    /// lhsvarnode-leading statements. `lhsvarnode: specificsymbol`. Also the
    /// two error productions `varnode ':' INTEGER '='` and `varnode '('
    /// INTEGER ')'` whose lhs is a specificsymbol.
    fn parse_lhs_statement(&mut self) -> Option<Vec<OpTpl>> {
        let sym = self.cur.clone();
        self.advance();
        let constspace = self.pcode.snippet_constant_space();
        // The varnode of the specificsymbol.
        let vn = specificsymbol_varnode(&sym, &constspace);
        if self.is_char(b'=') {
            self.advance();
            let mut rhs = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b';') {
                return None;
            }
            // statement: lhsvarnode '=' expr ';'  -> rhs->setOutput(lhs)
            if rhs.set_output(vn).is_err() {
                self.failed = true;
                return None;
            }
            return Some(ExprTree::to_vector(rhs));
        }
        if self.is_char(b'[') {
            self.advance();
            // lhsvarnode '[' INTEGER ',' INTEGER ']' '=' expr ';'
            let off = self.expect_integer()?;
            if !self.expect_char(b',') {
                return None;
            }
            let nbits = self.expect_integer()?;
            if !self.expect_char(b']') {
                return None;
            }
            if !self.expect_char(b'=') {
                return None;
            }
            let rhs = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b';') {
                return None;
            }
            match self.pcode.assign_bit_range(vn, off as u32, nbits as u32, rhs) {
                Ok(ops) => return Some(ops),
                Err(_) => {
                    self.failed = true;
                    return None;
                }
            }
        }
        if self.is_char(b':') {
            // varnode ':' INTEGER '='  -> illegal truncation on lhs
            self.advance();
            self.expect_integer()?;
            if self.expect_char(b'=') {
                self.yyerror("Illegal truncation on left-hand side of assignment");
            }
            return None;
        }
        if self.is_char(b'(') {
            // varnode '(' INTEGER ')'  -> illegal subpiece on lhs
            self.advance();
            self.expect_integer()?;
            if self.expect_char(b')') {
                self.yyerror("Illegal subpiece on left-hand side of assignment");
            }
            return None;
        }
        self.failed = true;
        None
    }

    /// STRING-leading statements:
    ///   STRING '=' expr ';'             -> newOutput(false,...)
    ///   STRING ':' INTEGER '=' expr ';' -> newOutput(true,...,size)
    /// A STRING used as an lhsvarnode that isn't one of these is the
    /// "Unknown assignment varnode" error.
    fn parse_string_statement(&mut self, name: Vec<u8>) -> Option<Vec<OpTpl>> {
        self.advance(); // STRING
        if self.is_char(b'=') {
            self.advance();
            let rhs = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b';') {
                return None;
            }
            match self.pcode.new_output(false, rhs, &name, 0) {
                Ok(ops) => Some(ops),
                Err(_) => {
                    self.failed = true;
                    None
                }
            }
        } else if self.is_char(b':') {
            self.advance();
            let size = self.expect_integer()?;
            if !self.expect_char(b'=') {
                return None;
            }
            let rhs = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b';') {
                return None;
            }
            // statement: STRING ':' INTEGER '=' expr ';' -> newOutput(true,..)
            match self.pcode.new_output(true, rhs, &name, size as u32) {
                Ok(ops) => Some(ops),
                Err(_) => {
                    self.failed = true;
                    None
                }
            }
        } else {
            // Unknown assignment varnode (lhsvarnode: STRING error production)
            self.yyerror(&format!(
                "Unknown assignment varnode: {}",
                String::from_utf8_lossy(&name)
            ));
            None
        }
    }

    /// `sizedstar`:
    ///   '*' '[' SPACESYM ']' ':' INTEGER
    ///   '*' '[' SPACESYM ']'
    ///   '*' ':' INTEGER
    ///   '*'
    fn parse_sizedstar(&mut self) -> Option<StarQuality> {
        // consume '*'
        if !self.expect_char(b'*') {
            return None;
        }
        if self.is_char(b'[') {
            self.advance();
            let space = match self.cur.clone() {
                Token::SpaceSym(sym) => sym,
                _ => {
                    self.failed = true;
                    return None;
                }
            };
            self.advance(); // SPACESYM
            if !self.expect_char(b']') {
                return None;
            }
            let spc = spacesym_space(&space);
            if self.is_char(b':') {
                self.advance();
                let size = self.expect_integer()?;
                return Some(StarQuality {
                    id: ConstTpl::new_space(spc),
                    size: size as u32,
                });
            }
            return Some(StarQuality {
                id: ConstTpl::new_space(spc),
                size: 0,
            });
        }
        if self.is_char(b':') {
            self.advance();
            let size = self.expect_integer()?;
            let def = self
                .pcode
                .get_default_space()
                .expect("snippet default space not set");
            return Some(StarQuality {
                id: ConstTpl::new_space(def),
                size: size as u32,
            });
        }
        // bare '*'
        let def = self
            .pcode
            .get_default_space()
            .expect("snippet default space not set");
        Some(StarQuality {
            id: ConstTpl::new_space(def),
            size: 0,
        })
    }

    /// `jumpdest`:
    ///   JUMPSYM | INTEGER | BADINTEGER | INTEGER '[' SPACESYM ']'
    ///   | label | STRING (error)
    fn parse_jumpdest(&mut self) -> Option<VarnodeTpl> {
        let constspace = self.pcode.snippet_constant_space();
        match self.cur.clone() {
            Token::JumpSym(sym) => {
                self.advance();
                let symvn = get_varnode_tpl(&sym, &constspace)
                    .expect("JUMPSYM has no getVarnode (internal invariant)");
                Some(VarnodeTpl::new(
                    ConstTpl::new_type(ConstType::JCurspace),
                    symvn.get_offset().clone(),
                    ConstTpl::new_type(ConstType::JCurspaceSize),
                ))
            }
            Token::Integer(v) => {
                self.advance();
                if self.is_char(b'[') {
                    self.advance();
                    let space = match self.cur.clone() {
                        Token::SpaceSym(sym) => sym,
                        _ => {
                            self.failed = true;
                            return None;
                        }
                    };
                    self.advance();
                    if !self.expect_char(b']') {
                        return None;
                    }
                    // INTEGER '[' SPACESYM ']'
                    let spc = spacesym_space(&space);
                    let addrsize = spc.get_addr_size();
                    return Some(VarnodeTpl::new(
                        ConstTpl::new_space(spc),
                        ConstTpl::new_real(ConstType::Real, v),
                        ConstTpl::new_real(ConstType::Real, u64::from(addrsize)),
                    ));
                }
                // INTEGER : j_curspace, real(v), j_curspace_size
                Some(VarnodeTpl::new(
                    ConstTpl::new_type(ConstType::JCurspace),
                    ConstTpl::new_real(ConstType::Real, v),
                    ConstTpl::new_type(ConstType::JCurspaceSize),
                ))
            }
            Token::BadInteger => {
                self.advance();
                self.yyerror("Parsed integer is too big (overflow)");
                // C++ still builds a VarnodeTpl(j_curspace, real(0),
                // j_curspace_size) but the parse continues only as far as
                // the surrounding production allows; our `failed` flag will
                // abort the parse just like YYERROR is not used here (the
                // BADINTEGER jumpdest does NOT YYERROR, it returns a varnode).
                self.failed = false; // jumpdest BADINTEGER does not YYERROR
                Some(VarnodeTpl::new(
                    ConstTpl::new_type(ConstType::JCurspace),
                    ConstTpl::new_real(ConstType::Real, 0),
                    ConstTpl::new_type(ConstType::JCurspaceSize),
                ))
            }
            Token::Char(b'<') => {
                // jumpdest: label
                let lab = self.parse_label()?;
                // the relative offset's size is sizeof(uintm) = 4
                lab.increment_ref_count();
                Some(VarnodeTpl::new(
                    ConstTpl::new_space(constspace),
                    ConstTpl::new_real(ConstType::JRelative, u64::from(lab.get_index())),
                    ConstTpl::new_real(ConstType::Real, 4),
                ))
            }
            Token::Str(name) => {
                self.advance();
                self.yyerror(&format!(
                    "Unknown jump destination: {}",
                    String::from_utf8_lossy(&name)
                ));
                None
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }

    /// `label: '<' LABELSYM '>' | '<' STRING '>'`. The leading '<' is current.
    fn parse_label(&mut self) -> Option<Rc<LabelSymbol>> {
        // consume '<'
        if !self.expect_char(b'<') {
            return None;
        }
        match self.cur.clone() {
            Token::LabelSym(lab) => {
                self.advance();
                if !self.expect_char(b'>') {
                    return None;
                }
                Some(lab)
            }
            Token::Str(name) => {
                self.advance();
                if !self.expect_char(b'>') {
                    return None;
                }
                // '<' STRING '>' : defineLabel
                Some(self.pcode.define_label(&name))
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }

    /// `paramlist`: EMPTY | expr | paramlist ',' expr. Returns the parameter
    /// expressions in source order.
    fn parse_paramlist(&mut self) -> Option<Vec<ExprTree>> {
        let mut params: Vec<ExprTree> = Vec::new();
        // EMPTY: a ')' immediately
        if self.is_char(b')') {
            return Some(params);
        }
        let first = self.parse_expr(PREC_LOWEST)?;
        params.push(first);
        while self.is_char(b',') {
            self.advance();
            let e = self.parse_expr(PREC_LOWEST)?;
            params.push(e);
        }
        Some(params)
    }

    /// Expect an INTEGER terminal; consume and return its value, else fail.
    /// (BADINTEGER is NOT accepted where the grammar wants a literal INTEGER.)
    fn expect_integer(&mut self) -> Option<u64> {
        match self.cur {
            Token::Integer(v) => {
                self.advance();
                Some(v)
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }

    // -- expression precedence climbing -------------------------------------

    /// `expr` via precedence climbing. `min_prec` is the lowest binding level
    /// this call will accept; mirrors the bison precedence table. Returns the
    /// expression tree or `None` on error.
    fn parse_expr(&mut self, min_prec: u8) -> Option<ExprTree> {
        let mut lhs = self.parse_unary()?;
        loop {
            if self.failed {
                return None;
            }
            let (prec, info) = match self.peek_binop() {
                Some(x) => x,
                None => break,
            };
            if prec < min_prec {
                break;
            }
            // All the binary levels are %left except the comparison level
            // (PREC_CMP), which is %nonassoc. For %left we recurse at prec+1
            // (left-associative). For %nonassoc we also recurse at prec+1, and
            // after applying the op a second comparison at the same level is a
            // syntax error (bison rejects `a < b < c`).
            self.advance(); // consume the operator
            let rhs = self.parse_expr(prec + 1)?;
            lhs = self.apply_binop(info, lhs, rhs)?;
            if prec == PREC_CMP {
                // %nonassoc: do not allow another comparison operator to chain.
                if let Some((next_prec, _)) = self.peek_binop() {
                    if next_prec == PREC_CMP {
                        self.failed = true;
                        return None;
                    }
                }
            }
        }
        Some(lhs)
    }

    /// Peek the current token as a binary operator, returning its precedence
    /// and an action descriptor. `None` if the current token is not a binop.
    fn peek_binop(&self) -> Option<(u8, BinOp)> {
        let info = match &self.cur {
            Token::BoolOr => (PREC_BOOL_OR, BinOp::Op(OpCode::CPUI_BOOL_OR, false)),
            Token::BoolAnd => (PREC_BOOL_AND, BinOp::Op(OpCode::CPUI_BOOL_AND, false)),
            Token::BoolXor => (PREC_BOOL_AND, BinOp::Op(OpCode::CPUI_BOOL_XOR, false)),
            Token::Char(b'|') => (PREC_OR, BinOp::Op(OpCode::CPUI_INT_OR, false)),
            Token::Char(b'^') => (PREC_XOR, BinOp::Op(OpCode::CPUI_INT_XOR, false)),
            Token::Char(b'&') => (PREC_AND, BinOp::Op(OpCode::CPUI_INT_AND, false)),
            Token::Equal => (PREC_EQ, BinOp::Op(OpCode::CPUI_INT_EQUAL, false)),
            Token::NotEqual => (PREC_EQ, BinOp::Op(OpCode::CPUI_INT_NOTEQUAL, false)),
            Token::Fequal => (PREC_EQ, BinOp::Op(OpCode::CPUI_FLOAT_EQUAL, false)),
            Token::Fnotequal => (PREC_EQ, BinOp::Op(OpCode::CPUI_FLOAT_NOTEQUAL, false)),
            // comparison operators (%nonassoc)
            Token::Char(b'<') => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_LESS, false)),
            Token::Char(b'>') => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_LESS, true)),
            Token::Greatequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_LESSEQUAL, true)),
            Token::Lessequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_LESSEQUAL, false)),
            Token::Sless => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_SLESS, false)),
            Token::Sgreatequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_SLESSEQUAL, true)),
            Token::Slessequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_SLESSEQUAL, false)),
            Token::Sgreat => (PREC_CMP, BinOp::Op(OpCode::CPUI_INT_SLESS, true)),
            Token::Fless => (PREC_CMP, BinOp::Op(OpCode::CPUI_FLOAT_LESS, false)),
            Token::Fgreat => (PREC_CMP, BinOp::Op(OpCode::CPUI_FLOAT_LESS, true)),
            Token::Flessequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_FLOAT_LESSEQUAL, false)),
            Token::Fgreatequal => (PREC_CMP, BinOp::Op(OpCode::CPUI_FLOAT_LESSEQUAL, true)),
            // shifts (%left)
            Token::Left => (PREC_SHIFT, BinOp::Op(OpCode::CPUI_INT_LEFT, false)),
            Token::Right => (PREC_SHIFT, BinOp::Op(OpCode::CPUI_INT_RIGHT, false)),
            Token::Sright => (PREC_SHIFT, BinOp::Op(OpCode::CPUI_INT_SRIGHT, false)),
            // additive (%left)
            Token::Char(b'+') => (PREC_ADD, BinOp::Op(OpCode::CPUI_INT_ADD, false)),
            Token::Char(b'-') => (PREC_ADD, BinOp::Op(OpCode::CPUI_INT_SUB, false)),
            Token::Fadd => (PREC_ADD, BinOp::Op(OpCode::CPUI_FLOAT_ADD, false)),
            Token::Fsub => (PREC_ADD, BinOp::Op(OpCode::CPUI_FLOAT_SUB, false)),
            // multiplicative (%left)
            Token::Char(b'*') => (PREC_MUL, BinOp::Op(OpCode::CPUI_INT_MULT, false)),
            Token::Char(b'/') => (PREC_MUL, BinOp::Op(OpCode::CPUI_INT_DIV, false)),
            Token::Char(b'%') => (PREC_MUL, BinOp::Op(OpCode::CPUI_INT_REM, false)),
            Token::Sdiv => (PREC_MUL, BinOp::Op(OpCode::CPUI_INT_SDIV, false)),
            Token::Srem => (PREC_MUL, BinOp::Op(OpCode::CPUI_INT_SREM, false)),
            Token::Fmult => (PREC_MUL, BinOp::Op(OpCode::CPUI_FLOAT_MULT, false)),
            Token::Fdiv => (PREC_MUL, BinOp::Op(OpCode::CPUI_FLOAT_DIV, false)),
            _ => return None,
        };
        Some(info)
    }

    /// Apply a binary operator action. `swap` reverses operands (the `>` / `>=`
    /// productions that reduce to a `<`/`<=` with swapped args).
    fn apply_binop(&mut self, info: BinOp, lhs: ExprTree, rhs: ExprTree) -> Option<ExprTree> {
        match info {
            BinOp::Op(opc, swap) => {
                if swap {
                    Some(self.pcode.create_op2(opc, rhs, lhs))
                } else {
                    Some(self.pcode.create_op2(opc, lhs, rhs))
                }
            }
        }
    }

    /// Parse a unary / primary expression (everything that binds tighter than
    /// the binary operators): the prefix unary ops, `sizedstar expr` loads,
    /// parenthesized expressions, the named p-code functions, and the
    /// varnode/specificsymbol primaries with their postfix `:`/`[`/`(` forms.
    fn parse_unary(&mut self) -> Option<ExprTree> {
        match self.cur.clone() {
            // '-' expr  %prec '!'   -> CPUI_INT_2COMP
            Token::Char(b'-') => {
                self.advance();
                let e = self.parse_expr(PREC_UNARY)?;
                Some(self.pcode.create_op(OpCode::CPUI_INT_2COMP, e))
            }
            // '~' expr              -> CPUI_INT_NEGATE
            Token::Char(b'~') => {
                self.advance();
                let e = self.parse_expr(PREC_UNARY)?;
                Some(self.pcode.create_op(OpCode::CPUI_INT_NEGATE, e))
            }
            // '!' expr              -> CPUI_BOOL_NEGATE
            Token::Char(b'!') => {
                self.advance();
                let e = self.parse_expr(PREC_UNARY)?;
                Some(self.pcode.create_op(OpCode::CPUI_BOOL_NEGATE, e))
            }
            // OP_FSUB expr %prec '!'-> CPUI_FLOAT_NEG
            Token::Fsub => {
                self.advance();
                let e = self.parse_expr(PREC_UNARY)?;
                Some(self.pcode.create_op(OpCode::CPUI_FLOAT_NEG, e))
            }
            // sizedstar expr %prec '!'  -> createLoad
            Token::Char(b'*') => {
                let qual = self.parse_sizedstar()?;
                let ptr = self.parse_expr(PREC_UNARY)?;
                match self.pcode.create_load(qual, ptr) {
                    Ok(e) => Some(e),
                    Err(_) => {
                        self.failed = true;
                        None
                    }
                }
            }
            // '(' expr ')'
            Token::Char(b'(') => {
                self.advance();
                let e = self.parse_expr(PREC_LOWEST)?;
                if !self.expect_char(b')') {
                    return None;
                }
                Some(e)
            }
            // named unary p-code function calls: OP '(' expr ')' (and the
            // binary/ternary OP_CARRY/OP_SCARRY/OP_SBORROW/OP_NEW forms).
            Token::Abs => self.parse_unary_func(OpCode::CPUI_FLOAT_ABS),
            Token::Sqrt => self.parse_unary_func(OpCode::CPUI_FLOAT_SQRT),
            Token::Sext => self.parse_unary_func(OpCode::CPUI_INT_SEXT),
            Token::Zext => self.parse_unary_func(OpCode::CPUI_INT_ZEXT),
            Token::Float2float => self.parse_unary_func(OpCode::CPUI_FLOAT_FLOAT2FLOAT),
            Token::Int2float => self.parse_unary_func(OpCode::CPUI_FLOAT_INT2FLOAT),
            Token::Nan => self.parse_unary_func(OpCode::CPUI_FLOAT_NAN),
            Token::Trunc => self.parse_unary_func(OpCode::CPUI_FLOAT_TRUNC),
            Token::Ceil => self.parse_unary_func(OpCode::CPUI_FLOAT_CEIL),
            Token::Floor => self.parse_unary_func(OpCode::CPUI_FLOAT_FLOOR),
            Token::Round => self.parse_unary_func(OpCode::CPUI_FLOAT_ROUND),
            Token::Carry => self.parse_binary_func(OpCode::CPUI_INT_CARRY),
            Token::Scarry => self.parse_binary_func(OpCode::CPUI_INT_SCARRY),
            Token::Sborrow => self.parse_binary_func(OpCode::CPUI_INT_SBORROW),
            Token::Borrow => {
                // OP_BORROW is a token but the grammar has no rule using it as
                // a function; bison would syntax-error. Reproduce that.
                self.failed = true;
                None
            }
            // OP_NEW '(' expr ')' | OP_NEW '(' expr ',' expr ')'
            Token::New => self.parse_new(),
            // USEROPSYM '(' paramlist ')'  -> createUserOp (with out)
            Token::UserOpSym(sym) => {
                self.advance();
                if !self.expect_char(b'(') {
                    return None;
                }
                let params = self.parse_paramlist()?;
                if !self.expect_char(b')') {
                    return None;
                }
                let usym = useropsym(&sym);
                Some(self.pcode.create_user_op(&usym, params))
            }
            // specificsymbol / integervarnode / varnode primaries
            Token::VarSym(_) | Token::OperandSym(_) | Token::JumpSym(_) => {
                self.parse_specificsymbol_primary()
            }
            Token::Integer(_) | Token::BadInteger | Token::Char(b'&') => {
                self.parse_integervarnode_primary()
            }
            // STRING as a varnode is the "Unknown varnode parameter" error.
            Token::Str(name) => {
                self.advance();
                self.yyerror(&format!(
                    "Unknown varnode parameter: {}",
                    String::from_utf8_lossy(&name)
                ));
                None
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }

    /// `OP '(' expr ')'` for the unary named p-code functions.
    fn parse_unary_func(&mut self, opc: OpCode) -> Option<ExprTree> {
        self.advance(); // OP
        if !self.expect_char(b'(') {
            return None;
        }
        let e = self.parse_expr(PREC_LOWEST)?;
        if !self.expect_char(b')') {
            return None;
        }
        Some(self.pcode.create_op(opc, e))
    }

    /// `OP '(' expr ',' expr ')'` for OP_CARRY/OP_SCARRY/OP_SBORROW.
    fn parse_binary_func(&mut self, opc: OpCode) -> Option<ExprTree> {
        self.advance(); // OP
        if !self.expect_char(b'(') {
            return None;
        }
        let a = self.parse_expr(PREC_LOWEST)?;
        if !self.expect_char(b',') {
            return None;
        }
        let b = self.parse_expr(PREC_LOWEST)?;
        if !self.expect_char(b')') {
            return None;
        }
        Some(self.pcode.create_op2(opc, a, b))
    }

    /// `OP_NEW '(' expr ')' | OP_NEW '(' expr ',' expr ')'`.
    fn parse_new(&mut self) -> Option<ExprTree> {
        self.advance(); // OP_NEW
        if !self.expect_char(b'(') {
            return None;
        }
        let a = self.parse_expr(PREC_LOWEST)?;
        if self.is_char(b',') {
            self.advance();
            let b = self.parse_expr(PREC_LOWEST)?;
            if !self.expect_char(b')') {
                return None;
            }
            return Some(self.pcode.create_op2(OpCode::CPUI_NEW, a, b));
        }
        if !self.expect_char(b')') {
            return None;
        }
        Some(self.pcode.create_op(OpCode::CPUI_NEW, a))
    }

    /// specificsymbol-leading primaries:
    ///   specificsymbol                             -> varnode -> ExprTree
    ///   specificsymbol '(' integervarnode ')'      -> SUBPIECE
    ///   specificsymbol ':' INTEGER                 -> createBitRange(0, n*8)
    ///   specificsymbol '[' INTEGER ',' INTEGER ']' -> createBitRange(o, n)
    fn parse_specificsymbol_primary(&mut self) -> Option<ExprTree> {
        let sym = self.cur.clone();
        self.advance();
        let constspace = self.pcode.snippet_constant_space();
        if self.is_char(b'(') {
            self.advance();
            // specificsymbol '(' integervarnode ')'
            let iv = self.parse_integervarnode_primary()?;
            if !self.expect_char(b')') {
                return None;
            }
            let symvn = specificsymbol_varnode(&sym, &constspace);
            return Some(self.pcode.create_op2(
                OpCode::CPUI_SUBPIECE,
                ExprTree::new(symvn),
                iv,
            ));
        }
        if self.is_char(b':') {
            self.advance();
            let n = self.expect_integer()?;
            // createBitRange(sym, 0, (uint4)(n*8))
            let symvn = specificsymbol_varnode(&sym, &constspace);
            let name = specificsymbol_name(&sym);
            // C++ `*$3 * 8` on uintb, then cast to uint4
            let numbits = (n.wmul(8)) as u32;
            match self.pcode.create_bit_range(symvn, &name, 0, numbits) {
                Ok(e) => return Some(e),
                Err(_) => {
                    self.failed = true;
                    return None;
                }
            }
        }
        if self.is_char(b'[') {
            self.advance();
            let off = self.expect_integer()?;
            if !self.expect_char(b',') {
                return None;
            }
            let nbits = self.expect_integer()?;
            if !self.expect_char(b']') {
                return None;
            }
            let symvn = specificsymbol_varnode(&sym, &constspace);
            let name = specificsymbol_name(&sym);
            match self
                .pcode
                .create_bit_range(symvn, &name, off as u32, nbits as u32)
            {
                Ok(e) => return Some(e),
                Err(_) => {
                    self.failed = true;
                    return None;
                }
            }
        }
        // plain varnode: specificsymbol  -> new ExprTree(getVarnode())
        let symvn = specificsymbol_varnode(&sym, &constspace);
        Some(ExprTree::new(symvn))
    }

    /// integervarnode-leading primaries (also reachable as a `varnode`):
    ///   INTEGER | BADINTEGER | INTEGER ':' INTEGER
    ///   | '&' varnode | '&' ':' INTEGER varnode
    fn parse_integervarnode_primary(&mut self) -> Option<ExprTree> {
        let constspace = self.pcode.snippet_constant_space();
        match self.cur.clone() {
            Token::Integer(v) => {
                self.advance();
                if self.is_char(b':') {
                    self.advance();
                    let sz = self.expect_integer()?;
                    // INTEGER ':' INTEGER : const(real v, real sz)
                    return Some(ExprTree::new(VarnodeTpl::new(
                        ConstTpl::new_space(constspace),
                        ConstTpl::new_real(ConstType::Real, v),
                        ConstTpl::new_real(ConstType::Real, sz),
                    )));
                }
                // INTEGER : const(real v, real 0)
                Some(ExprTree::new(VarnodeTpl::new(
                    ConstTpl::new_space(constspace),
                    ConstTpl::new_real(ConstType::Real, v),
                    ConstTpl::new_real(ConstType::Real, 0),
                )))
            }
            Token::BadInteger => {
                self.advance();
                self.yyerror("Parsed integer is too big (overflow)");
                // integervarnode BADINTEGER does NOT YYERROR; it yields a
                // const(real 0, real 0) and the parse continues.
                self.failed = false;
                Some(ExprTree::new(VarnodeTpl::new(
                    ConstTpl::new_space(constspace),
                    ConstTpl::new_real(ConstType::Real, 0),
                    ConstTpl::new_real(ConstType::Real, 0),
                )))
            }
            // '&' varnode  |  '&' ':' INTEGER varnode
            Token::Char(b'&') => {
                self.advance();
                if self.is_char(b':') {
                    self.advance();
                    let size = self.expect_integer()?;
                    let vn = self.parse_varnode()?;
                    return Some(ExprTree::new(self.pcode.address_of(vn, size as u32)));
                }
                let vn = self.parse_varnode()?;
                Some(ExprTree::new(self.pcode.address_of(vn, 0)))
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }

    /// `varnode: specificsymbol | integervarnode | STRING (error)`. Returns a
    /// bare [`VarnodeTpl`] (the `&`-operator and SUBPIECE arg contexts want a
    /// varnode, not an ExprTree).
    fn parse_varnode(&mut self) -> Option<VarnodeTpl> {
        let constspace = self.pcode.snippet_constant_space();
        let tok = self.cur.clone();
        match &tok {
            Token::VarSym(_) | Token::OperandSym(_) | Token::JumpSym(_) => {
                self.advance();
                Some(specificsymbol_varnode(&tok, &constspace))
            }
            Token::Integer(_) | Token::BadInteger | Token::Char(b'&') => {
                // integervarnode reduces to a varnode; reuse the expr builder
                // and pull its (single) output varnode.
                let e = self.parse_integervarnode_primary()?;
                e.get_out().cloned()
            }
            Token::Str(name) => {
                let name = name.clone();
                self.advance();
                self.yyerror(&format!(
                    "Unknown varnode parameter: {}",
                    String::from_utf8_lossy(&name)
                ));
                None
            }
            _ => {
                self.failed = true;
                None
            }
        }
    }
}

/// Binary-operator action descriptor for the precedence climber.
#[derive(Debug, Clone, Copy)]
enum BinOp {
    /// `create_op2(opcode, a, b)`; if the bool is set, operands are swapped
    /// (the `>`/`>=`-style productions that reduce to `<`/`<=`).
    Op(OpCode, bool),
}

// -- precedence levels (bison %left/%right/%nonassoc order) ------------------
// Lower number = binds looser. Mirrors the declaration order in pcodeparse.y.
const PREC_LOWEST: u8 = 0;
const PREC_BOOL_OR: u8 = 1;
const PREC_BOOL_AND: u8 = 2; // OP_BOOL_AND, OP_BOOL_XOR
const PREC_OR: u8 = 3; // '|'
// ';' level (4) is statement-only, never an expr binop
const PREC_XOR: u8 = 5; // '^'
const PREC_AND: u8 = 6; // '&'
const PREC_EQ: u8 = 7; // ==, !=, f==, f!=
const PREC_CMP: u8 = 8; // <, >, comparisons (%nonassoc)
const PREC_SHIFT: u8 = 9; // <<, >>, s>>
const PREC_ADD: u8 = 10; // +, -, f+, f-
const PREC_MUL: u8 = 11; // *, /, %, s/, s%, f*, f/
const PREC_UNARY: u8 = 12; // !, ~ (%right) — unary prefix recursion level

// ---------------------------------------------------------------------------
// Symbol projection helpers (the grammar's $1->getVarnode()/getName()/etc.)
// ---------------------------------------------------------------------------

/// `$1->getVarnode()` for a specificsymbol terminal (VARSYM/OPERANDSYM/
/// JUMPSYM). The grammar always has a varnode here.
fn specificsymbol_varnode(tok: &Token, constspace: &Rc<AddrSpace>) -> VarnodeTpl {
    let sym = match tok {
        Token::VarSym(s) | Token::OperandSym(s) | Token::JumpSym(s) => s,
        _ => unreachable!("specificsymbol_varnode on a non-symbol token"),
    };
    get_varnode_tpl(sym, constspace)
        .expect("specificsymbol has no getVarnode (internal invariant)")
}

/// `$2->getName()` for a specificsymbol terminal — the declared symbol name,
/// used by the `LOCAL` redefinition error, the `createBitRange` error
/// messages, and the `createBitRange(sym,...)` calls. The resolved
/// [`SnippetSymbol`] carries it (C++ `SleighSymbol::getName()`).
fn specificsymbol_name(tok: &Token) -> Vec<u8> {
    let sym = match tok {
        Token::VarSym(s) | Token::OperandSym(s) | Token::JumpSym(s) => s,
        _ => unreachable!("specificsymbol_name on a non-symbol token"),
    };
    sym.name().to_vec()
}

/// `SPACESYM->getSpace()` (over the inner `SnippetSymbol::Space`).
fn spacesym_space(sym: &SnippetSymbol) -> Rc<AddrSpace> {
    match sym {
        SnippetSymbol::Space(spc) => Rc::clone(spc),
        _ => unreachable!("spacesym_space on a non-space symbol"),
    }
}

/// `USEROPSYM` -> the `UserOpSymbol` (over the inner `SnippetSymbol::UserOp`).
fn useropsym(sym: &SnippetSymbol) -> UserOpSymbol {
    match sym {
        SnippetSymbol::UserOp(u) => u.clone(),
        _ => unreachable!("useropsym on a non-userop symbol"),
    }
}

#[cfg(test)]
mod tests;
