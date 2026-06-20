//! WS2 -- the SLEIGH grammar (hand recursive-descent port of
//! `decompiler/cpp/slghparse.y`).
//!
//! The C++ parser is bison LALR(1); kuna ports it (as with the p-code `parse line`
//! grammar already in `kuna-sleigh/src/pcodeparse`) to a **hand recursive-descent
//! parser**.  Each grammar action calls a builder on the driver, which constructs
//! the symbol-table / constructor / template objects (the already-ported
//! `kuna-sleigh` types).
//!
//! ## The driver seam ([`ParserActions`])
//!
//! The bison grammar's actions are `slgh->...` calls on a `SleighCompile`.  In the
//! Rust port those land in WS4 (`slgh_compile.rs`), still `todo!()` while WS2 is
//! filled.  So WS2 drives the parse through the [`ParserActions`] trait: every
//! builder the grammar invokes is a trait method, and the parser threads the
//! arena ids (`u32`) those methods return (pattern equations, pattern
//! expressions, ConstructTpl sections, ExprTrees, VarnodeTpls, op lists, ...)
//! exactly as bison threads the `$$`/`$n` semantic values.  This mirrors the
//! interface-freeze decision in `docs/rust-port/sleigh-compiler/map.md` (WS4):
//! "Pattern equations / ConstructTpls / op-template lists are referenced by arena
//! ids (`u32`) the driver owns -- this keeps the parser from threading raw owned
//! trees through every production."
//!
//! [`crate::slgh_compile::SleighCompile`] implements [`ParserActions`] (and
//! [`crate::slghscan::ScannerHost`]); the recording mock used by the golden-parse
//! tests implements both too, so WS2 is verified independently of WS4.  Every
//! action emits a [`ParserActions::trace`] token; the golden traces are captured
//! from an instrumented `/tmp` copy of the bison grammar (the `KT(...)` calls --
//! see `docs/rust-port/sleigh-compiler/ws2-parser.md`).
//!
//! ## Precedence (slghparse.y:82-93), highest last
//!
//! ```text
//! %left  OP_BOOL_OR
//! %left  OP_BOOL_AND OP_BOOL_XOR
//! %left  '|' OP_OR
//! %left  ';'
//! %left  '^' OP_XOR
//! %left  '&' OP_AND
//! %left  OP_EQUAL OP_NOTEQUAL OP_FEQUAL OP_FNOTEQUAL
//! %nonassoc '<' '>' OP_GREATEQUAL OP_LESSEQUAL OP_SLESS OP_SGREATEQUAL ...
//! %left  OP_LEFT OP_RIGHT OP_SRIGHT
//! %left  '+' '-' OP_FADD OP_FSUB
//! %left  '*' '/' '%' OP_SDIV OP_SREM OP_FMULT OP_FDIV
//! %right '!' '~'
//! ```
//! Realized as precedence-climbing for both `pexpression` (pattern values) and
//! `expr` (p-code).
//!
//! ## Module ownership: WS2 owns this file exclusively.

#![allow(dead_code)]

use kuna_base::error::{KunaError, KunaResult};

use crate::slgh_compile::SleighCompile;
use crate::slghscan::{ScannerHost, SleighScanner, Token, TokenKind, TokenValue};

/// `actionon == 0`: parsing a pattern (the `actionon` global, slghparse.y:26).
pub const ACTIONON_PATTERN: i32 = 0;
/// `actionon == 1`: parsing inside a pattern action `[ ... ]` block.
pub const ACTIONON_ACTION: i32 = 1;

/// p-code opcodes the grammar names directly (the `CPUI_*` constants the bison
/// actions pass to `pcode.createOp`).  Kept as a small enum so the trait stays
/// free of a dependency on the full `kuna_sleigh` opcode set; WS4 maps these to
/// `kuna_sleigh::opcodes::OpCode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcodeOpc {
    IntAdd, IntSub, IntEqual, IntNotEqual, IntLess, IntLessEqual,
    IntSless, IntSlessEqual, Int2Comp, IntNegate, IntXor, IntAnd, IntOr,
    IntLeft, IntRight, IntSright, IntMult, IntDiv, IntSdiv, IntRem, IntSrem,
    BoolNegate, BoolXor, BoolAnd, BoolOr,
    FloatEqual, FloatNotEqual, FloatLess, FloatLessEqual,
    FloatAdd, FloatSub, FloatMult, FloatDiv, FloatNeg, FloatAbs, FloatSqrt,
    IntSext, IntZext, IntCarry, IntScarry, IntSborrow,
    FloatFloat2Float, FloatInt2Float, FloatNan, FloatTrunc, FloatCeil,
    FloatFloor, FloatRound, New, Popcount, Lzcount, Subpiece, Cpoolref,
    Branch, Cbranch, BranchInd, Call, CallInd, Return,
}

/// Constant-op constants the grammar passes to `pcode.createOpConst`
/// (`BUILD`, `DELAY_SLOT`; slghparse.y:378,381).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstOp {
    Build,
    DelaySlot,
}

/// A symbol id in the driver's `SymbolTable` (alias of
/// [`crate::slgh_compile::SymbolId`]).
pub type SymId = crate::slgh_compile::SymbolId;

/// The driver builder seam the parser drives (see module docs).  Every method
/// corresponds 1:1 to a `slgh->...` (or `slgh->pcode....`) call in `slghparse.y`;
/// the `u32` ids are the driver-owned arena handles that stand in for the C++
/// `*` semantic values.
///
/// Implemented by [`SleighCompile`] (WS4 bodies) and by the test recording mock.
pub trait ParserActions {
    /// Emit a golden-trace token for the action just taken (default no-op; the
    /// test mock records it).  Mirrors the instrumented bison `KT("...")`.
    fn trace(&mut self, _tok: &str) {}

    // --- definitions ---
    fn set_endian(&mut self, big: i32);
    fn set_alignment(&mut self, val: i32);
    fn define_token(&mut self, name: &[u8], sz: u64, endian: i32) -> SymId;
    fn add_token_field(&mut self, sym: SymId, qual: FieldQual);
    fn context_prop_begin(&mut self, varsym: SymId) -> SymId;
    fn add_context_field(&mut self, sym: SymId, qual: FieldQual) -> bool;
    fn new_space(&mut self, qual: SpaceQual);
    fn define_varnodes(&mut self, spacesym: SymId, off: u64, size: u64, names: Vec<Vec<u8>>);
    fn define_bitrange(&mut self, name: &[u8], sym: SymId, bitoffset: u32, numb: u32);
    fn add_user_op(&mut self, names: Vec<Vec<u8>>);
    fn attach_values(&mut self, symlist: Vec<SymId>, numlist: Vec<i64>);
    fn attach_names(&mut self, symlist: Vec<SymId>, names: Vec<Vec<u8>>);
    fn attach_varnodes(&mut self, symlist: Vec<SymId>, varlist: Vec<SymId>);

    // --- macros / with / constructors ---
    fn build_macro(&mut self, sym: SymId, rtl: u32);
    fn create_macro(&mut self, name: &[u8], params: Vec<Vec<u8>>) -> SymId;
    fn push_with(&mut self, ss: Option<SymId>, pateq: Option<u32>, contvec: Option<Vec<u32>>);
    fn pop_with(&mut self);
    fn new_table(&mut self, nm: &[u8]) -> SymId;
    fn create_constructor(&mut self, sym: Option<SymId>) -> u32;
    fn reset_constructors(&mut self);
    fn add_syntax(&mut self, ct: u32, syntax: &[u8]);
    fn new_operand(&mut self, ct: u32, nm: &[u8]);
    fn is_in_root(&self, ct: u32) -> bool;
    fn build_constructor(&mut self, big: u32, pateq: Option<u32>, contvec: Option<Vec<u32>>, vec: u32);

    // --- p-code sections ---
    fn standalone_section(&mut self, main: u32) -> u32;
    fn final_named_section(&mut self, vec: u32, section: u32) -> u32;
    fn first_named_section(&mut self, main: u32, sym: SymId) -> u32;
    fn next_named_section(&mut self, vec: u32, section: u32, sym: SymId) -> u32;
    fn new_section_symbol(&mut self, nm: &[u8]) -> SymId;
    fn enter_section(&mut self) -> u32;
    /// `rtl: rtlmid` (slghparse.y:355): finalize the main section, recording a NOP
    /// if it produced neither ops nor a result.  Returns the section id.
    fn finish_main_rtl(&mut self, rtlmid: u32) -> u32;
    fn set_result_varnode(&mut self, ct: u32, vn: u32) -> u32;
    fn set_result_star_varnode(&mut self, ct: u32, star: u32, vn: u32) -> u32;
    /// `rtlmid statement` (slghparse.y:362): append the statement's op list to the
    /// section.  Returns false on a multiple-delayslot error.
    fn rtl_add_oplist(&mut self, sec: u32, stmt: u32) -> bool;

    // --- pattern equations / expressions (arena builders) ---
    fn peq_and(&mut self, l: u32, r: u32) -> u32;
    fn peq_or(&mut self, l: u32, r: u32) -> u32;
    fn peq_cat(&mut self, l: u32, r: u32) -> u32;
    fn peq_left_ellipsis(&mut self, e: u32) -> u32;
    fn peq_right_ellipsis(&mut self, e: u32) -> u32;
    fn peq_equal(&mut self, fam: SymId, rhs: u32) -> u32;
    fn peq_notequal(&mut self, fam: SymId, rhs: u32) -> u32;
    fn peq_less(&mut self, fam: SymId, rhs: u32) -> u32;
    fn peq_lessequal(&mut self, fam: SymId, rhs: u32) -> u32;
    fn peq_greater(&mut self, fam: SymId, rhs: u32) -> u32;
    fn peq_greaterequal(&mut self, fam: SymId, rhs: u32) -> u32;
    fn constrain_operand(&mut self, sym: SymId, patexp: u32) -> Option<u32>;
    fn peq_operand_equation(&mut self, sym: SymId) -> u32;
    fn self_define(&mut self, sym: SymId);
    fn peq_unconstrained(&mut self, spec: SymId) -> u32;
    fn define_invisible_operand(&mut self, sym: SymId) -> Option<u32>;

    fn pexp_constant(&mut self, val: i64) -> u32;
    fn pexp_family_value(&mut self, fam: SymId) -> u32;
    fn pexp_spec_expression(&mut self, spec: SymId) -> u32;
    fn pexp_plus(&mut self, l: u32, r: u32) -> u32;
    fn pexp_sub(&mut self, l: u32, r: u32) -> u32;
    fn pexp_mult(&mut self, l: u32, r: u32) -> u32;
    fn pexp_leftshift(&mut self, l: u32, r: u32) -> u32;
    fn pexp_rightshift(&mut self, l: u32, r: u32) -> u32;
    fn pexp_and(&mut self, l: u32, r: u32) -> u32;
    fn pexp_or(&mut self, l: u32, r: u32) -> u32;
    fn pexp_xor(&mut self, l: u32, r: u32) -> u32;
    fn pexp_div(&mut self, l: u32, r: u32) -> u32;
    fn pexp_minus(&mut self, e: u32) -> u32;
    fn pexp_not(&mut self, e: u32) -> u32;

    // --- context block ---
    fn context_mod(&mut self, vec: &mut Vec<u32>, sym: SymId, pe: u32) -> bool;
    fn context_set(&mut self, vec: &mut Vec<u32>, sym: SymId, cvar: SymId);
    fn define_operand(&mut self, sym: SymId, patexp: u32);

    // --- statements / expr / varnodes (all arena builders via the driver) ---
    fn pcode_new_local_definition(&mut self, name: &[u8], size: Option<u64>);
    fn pcode_new_output(&mut self, islocal: bool, expr: u32, name: &[u8], size: Option<u64>) -> u32;
    fn stmt_assign(&mut self, lhs: u32, expr: u32) -> u32;
    fn pcode_create_store(&mut self, star: u32, ptr: u32, val: u32) -> u32;
    fn pcode_create_user_op_noout(&mut self, sym: SymId, params: Vec<u32>) -> u32;
    fn pcode_assign_bitrange_idx(&mut self, lhs: u32, off: u32, size: u32, expr: u32) -> u32;
    fn pcode_assign_bitrange_bitsym(&mut self, bitsym: SymId, expr: u32) -> u32;
    fn pcode_create_op_const(&mut self, op: ConstOp, val: u64) -> u32;
    fn create_cross_build(&mut self, addr: u32, sym: SymId) -> u32;
    fn pcode_create_op_noout(&mut self, opc: PcodeOpc, a: u32, cond: Option<u32>) -> u32;
    fn create_macro_use_stmt(&mut self, sym: SymId, param: Vec<u32>) -> u32;
    fn pcode_place_label(&mut self, label: SymId) -> u32;

    fn expr_from_varnode(&mut self, vn: u32) -> u32;
    fn pcode_create_load(&mut self, star: u32, ptr: u32) -> u32;
    fn pcode_create_op(&mut self, opc: PcodeOpc, a: u32, b: Option<u32>) -> u32;
    fn pcode_create_bitrange_colon(&mut self, spec: SymId, nbytes: u64) -> u32;
    fn pcode_create_bitrange_idx(&mut self, spec: SymId, off: u32, size: u32) -> u32;
    fn pcode_create_bitrange_bitsym(&mut self, bitsym: SymId) -> u32;
    fn pcode_create_user_op(&mut self, sym: SymId, params: Vec<u32>) -> u32;
    fn pcode_create_variadic_cpoolref(&mut self, params: Vec<u32>) -> u32;
    fn pcode_create_subpiece(&mut self, spec: SymId, off: u32) -> u32;

    fn sizedstar_space_sz(&mut self, spacesym: SymId, size: u64) -> u32;
    fn sizedstar_space(&mut self, spacesym: SymId) -> u32;
    fn sizedstar_default_sz(&mut self, size: u64) -> u32;
    fn sizedstar_default(&mut self) -> u32;

    fn jumpdest_jumpsym(&mut self, sym: SymId) -> u32;
    fn jumpdest_integer(&mut self, val: u64) -> u32;
    fn jumpdest_operandsym(&mut self, sym: SymId) -> u32;
    fn jumpdest_integer_space(&mut self, val: u64, spacesym: SymId) -> u32;
    fn jumpdest_label(&mut self, label: SymId) -> u32;

    fn varnode_spec(&mut self, spec: SymId) -> u32;
    fn intvn_integer(&mut self, val: u64) -> u32;
    fn intvn_integer_colon(&mut self, val: u64, size: u64) -> u32;
    fn pcode_address_of(&mut self, vn: u32, size: u64) -> u32;
    fn lhsvarnode_spec(&mut self, spec: SymId) -> u32;
    fn exportvarnode_spec(&mut self, spec: SymId) -> u32;
    fn exportvarnode_integer_colon(&mut self, val: u64, size: u64) -> u32;

    fn label_sym(&mut self, sym: SymId) -> u32;
    fn pcode_define_label(&mut self, name: &[u8]) -> u32;

    // --- symbol-handle resolution (the `*SYM` token -> SymId mapping) ---
    /// Resolve the symbol named `name` (the identifier the lexer attached to a
    /// `*SYM` token) to its `SymId`.  The lexer already decided the token *kind*
    /// (via `find_symbol`); this returns the symbol id for the builders.
    fn resolve_symbol(&mut self, name: &[u8]) -> SymId;
    /// The `getIndex()` of an operand symbol (slghparse.y:332,378): used to build
    /// an `OperandEquation` / a `BUILD` op-const.
    fn operand_index(&mut self, sym: SymId) -> u32;

    // --- error reporting ---
    fn report_error(&mut self, msg: &str);
}

/// Parsed token/context field qualities (the grammar's `FieldQuality`).  Mirrors
/// [`crate::slgh_compile::FieldQuality`] but stays parser-local so the parser
/// never depends on the driver's not-yet-built `FieldQuality::new` (WS4).
#[derive(Clone, Debug)]
pub struct FieldQual {
    pub name: Vec<u8>,
    pub low: u64,
    pub high: u64,
    pub signext: bool,
    pub flow: bool,
    pub hex: bool,
}

impl FieldQual {
    /// `new FieldQuality(name, low, high)` (slgh_compile.cc:102): defaults
    /// signext=false, flow=true, hex=true.
    fn new(name: Vec<u8>, low: u64, high: u64) -> FieldQual {
        FieldQual { name, low, high, signext: false, flow: true, hex: true }
    }
}

/// Parsed address-space qualities (the grammar's `SpaceQuality`).  Parser-local
/// for the same reason as [`FieldQual`].
#[derive(Clone, Debug)]
pub struct SpaceQual {
    pub name: Vec<u8>,
    /// false = ramtype (default), true = registertype.
    pub is_register: bool,
    pub size: u32,
    pub wordsize: u32,
    pub isdefault: bool,
}

impl SpaceQual {
    /// `new SpaceQuality(name)` (slgh_compile.cc:87): defaults ramtype, size 0,
    /// wordsize 1, not default.
    fn new(name: Vec<u8>) -> SpaceQual {
        SpaceQual { name, is_register: false, size: 0, wordsize: 1, isdefault: false }
    }
}

/// The combined driver trait the parser needs: both the lexer's [`ScannerHost`]
/// side effects and the grammar's [`ParserActions`] builders.  One object (the
/// `SleighCompile`, or the test mock) provides both.
pub trait ParseDriver: ScannerHost + ParserActions {
    /// Self-view as the actions sub-trait (for the precedence-climbing closures).
    fn as_actions(&mut self) -> &mut dyn ParserActions;
    /// Self-view as the scanner-host sub-trait (for the lexer call).
    fn as_host(&mut self) -> &mut dyn ScannerHost;
}
impl<T: ScannerHost + ParserActions> ParseDriver for T {
    fn as_actions(&mut self) -> &mut dyn ParserActions {
        self
    }
    fn as_host(&mut self) -> &mut dyn ScannerHost {
        self
    }
}

/// The recursive-descent parser.  Holds the token lookahead and the scanner; all
/// semantic actions are delegated to the driver (`&mut dyn ParseDriver`).
pub struct SleighParser {
    scanner: SleighScanner,
    /// One-token lookahead (LALR(1) needs only a single token here).
    lookahead: Option<Token>,
}

impl SleighParser {
    /// Construct a parser over the given (already-opened) scanner.
    pub fn new(scanner: SleighScanner) -> SleighParser {
        SleighParser { scanner, lookahead: None }
    }

    /// Parse a whole `.slaspec`, driving the builders on `compiler`.
    ///
    /// Entry point corresponding to bison's `sleighparse()`
    /// (slgh_compile.cc:3786).  Returns `Ok(0)` on success (matching the C++
    /// `parseres == 0` convention).
    pub fn parse(&mut self, compiler: &mut SleighCompile) -> KunaResult<i32> {
        self.parse_driver(compiler)
    }

    /// Parse against any [`ParseDriver`] (the real driver or the test mock).
    pub fn parse_driver(&mut self, d: &mut dyn ParseDriver) -> KunaResult<i32> {
        // `spec: endiandef | spec aligndef | spec definition | spec constructorlike`
        // (slghparse.y:163-167).
        self.spec(d)?;
        Ok(0)
    }

    // -----------------------------------------------------------------------
    // Token plumbing
    // -----------------------------------------------------------------------

    /// Peek the current lookahead token, fetching one if needed.
    fn peek(&mut self, d: &mut dyn ParseDriver) -> KunaResult<TokenKind> {
        if self.lookahead.is_none() {
            let tok = self.scanner.next_token(d.as_host())?;
            self.lookahead = Some(tok);
        }
        Ok(self.lookahead.as_ref().unwrap().kind)
    }

    /// Consume and return the current token.
    fn next(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Token> {
        if let Some(tok) = self.lookahead.take() {
            return Ok(tok);
        }
        self.scanner.next_token(d.as_host())
    }

    /// Consume a token, asserting it is `kind`.
    fn expect(&mut self, d: &mut dyn ParseDriver, kind: TokenKind) -> KunaResult<Token> {
        let tok = self.next(d)?;
        if tok.kind != kind {
            return Err(KunaError::parse(format!(
                "SLEIGH parse error: expected {kind:?}, got {:?}",
                tok.kind
            )));
        }
        Ok(tok)
    }

    /// Consume the next token and return it if its kind matches `kind`, else
    /// leave it in the lookahead and return `None`.
    fn accept(&mut self, d: &mut dyn ParseDriver, kind: TokenKind) -> KunaResult<Option<Token>> {
        if self.peek(d)? == kind {
            Ok(Some(self.next(d)?))
        } else {
            Ok(None)
        }
    }

    // -----------------------------------------------------------------------
    // spec / definition / constructorlike (slghparse.y:163-182)
    // -----------------------------------------------------------------------

    fn spec(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        // `endiandef` first (slghparse.y:163).
        self.endiandef(d)?;
        loop {
            let k = self.peek(d)?;
            if k == TokenKind::Eof {
                return Ok(());
            }
            // aligndef / definition all begin with DEFINE_KEY; constructorlike
            // begins with ATTACH/MACRO/WITH/a subtable-start (SUBTABLESYM / STRING
            // / ':').  Dispatch on the leading token.
            match k {
                TokenKind::DefineKey => self.after_define(d)?,
                TokenKind::AttachKey => self.attach(d)?,
                TokenKind::MacroKey => self.macrodef(d)?,
                TokenKind::WithKey => self.withblock(d)?,
                _ => self.constructor(d)?,
            }
        }
    }

    /// `endiandef: DEFINE_KEY ENDIAN_KEY '=' (BIG|LITTLE) ';'` (slghparse.y:184).
    fn endiandef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::DefineKey)?;
        self.expect(d, TokenKind::EndianKey)?;
        self.expect(d, TokenKind::Char1(b'='))?;
        self.endian_tail(d)
    }

    /// The `(BIG|LITTLE) ';'` tail shared by `endiandef` (the `=` already consumed).
    fn endian_tail(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        match self.next(d)?.kind {
            TokenKind::BigKey => {
                d.trace("setEndian(1)");
                d.set_endian(1);
            }
            TokenKind::LittleKey => {
                d.trace("setEndian(0)");
                d.set_endian(0);
            }
            other => {
                return Err(KunaError::parse(format!(
                    "SLEIGH: expected big/little endian, got {other:?}"
                )))
            }
        }
        self.expect(d, TokenKind::Char1(b';'))?;
        Ok(())
    }

    /// Dispatch the productions that start with `DEFINE_KEY` (after the leading
    /// `define`): aligndef, endiandef, tokendef, contextdef, spacedef,
    /// varnodedef, bitrangedef, pcodeopdef (slghparse.y:184-238).
    fn after_define(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::DefineKey)?;
        match self.peek(d)? {
            TokenKind::EndianKey => {
                self.expect(d, TokenKind::EndianKey)?;
                self.expect(d, TokenKind::Char1(b'='))?;
                self.endian_tail(d)
            }
            TokenKind::AlignKey => self.aligndef(d),
            TokenKind::TokenKey => self.tokendef(d),
            TokenKind::ContextKey => self.contextdef(d),
            TokenKind::SpaceKey => self.spacedef(d),
            TokenKind::SpaceSym => self.varnodedef(d),
            TokenKind::BitrangeKey => self.bitrangedef(d),
            TokenKind::PcodeopKey => self.pcodeopdef(d),
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token after `define`: {other:?}"
            ))),
        }
    }

    /// `aligndef: DEFINE_KEY ALIGN_KEY '=' INTEGER ';'` (slghparse.y:187).
    fn aligndef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::AlignKey)?;
        self.expect(d, TokenKind::Char1(b'='))?;
        let v = self.expect_integer(d)?;
        self.expect(d, TokenKind::Char1(b';'))?;
        d.trace("setAlignment");
        d.set_alignment(v as i32);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // tokens / contexts (slghparse.y:189-214)
    // -----------------------------------------------------------------------

    /// `tokendef: tokenprop ';'` (slghparse.y:189).  At `TOKEN_KEY`.
    fn tokendef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::TokenKey)?;
        let name = self.expect_string(d)?;
        self.expect(d, TokenKind::Char1(b'('))?;
        let sz = self.expect_integer(d)?;
        self.expect(d, TokenKind::Char1(b')'))?;
        // Optional `ENDIAN_KEY '=' (LITTLE|BIG)` (slghparse.y:192-193).
        let endian = if self.accept(d, TokenKind::EndianKey)?.is_some() {
            self.expect(d, TokenKind::Char1(b'='))?;
            match self.next(d)?.kind {
                TokenKind::LittleKey => -1,
                TokenKind::BigKey => 1,
                other => return Err(KunaError::parse(format!("SLEIGH: bad token endian {other:?}"))),
            }
        } else {
            0
        };
        d.trace(match endian {
            -1 => "defineToken/-1",
            1 => "defineToken/1",
            _ => "defineToken/0",
        });
        let tok = d.define_token(&name, sz, endian);
        // `tokenprop fielddef` repetition (slghparse.y:194).
        while self.peek(d)? != TokenKind::Char1(b';') {
            let qual = self.fielddef(d, /*context=*/ false)?;
            d.trace("addTokenField");
            d.add_token_field(tok, qual);
        }
        self.expect(d, TokenKind::Char1(b';'))?;
        Ok(())
    }

    /// `contextdef: contextprop ';'` (slghparse.y:197).  At `CONTEXT_KEY`.
    fn contextdef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::ContextKey)?;
        // `DEFINE_KEY CONTEXT_KEY VARSYM` (slghparse.y:199).
        let var = self.expect_symbol(d, TokenKind::VarSym)?;
        d.trace("contextprop/VARSYM");
        let sym = d.context_prop_begin(var);
        // `contextprop contextfielddef` repetition (slghparse.y:200).
        while self.peek(d)? != TokenKind::Char1(b';') {
            let qual = self.fielddef(d, /*context=*/ true)?;
            d.trace("addContextField");
            if !d.add_context_field(sym, qual) {
                d.report_error("All context definitions must come before constructors");
                return Err(KunaError::parse("context after constructors".to_string()));
            }
        }
        self.expect(d, TokenKind::Char1(b';'))?;
        Ok(())
    }

    /// `fielddef` / `contextfielddef` (slghparse.y:203-214): the base
    /// `STRING '=' '(' INTEGER ',' INTEGER ')'` plus a run of quality keywords.
    fn fielddef(&mut self, d: &mut dyn ParseDriver, context: bool) -> KunaResult<FieldQual> {
        let name = self.expect_string(d)?;
        self.expect(d, TokenKind::Char1(b'='))?;
        self.expect(d, TokenKind::Char1(b'('))?;
        let low = self.expect_integer(d)?;
        self.expect(d, TokenKind::Char1(b','))?;
        let high = self.expect_integer(d)?;
        self.expect(d, TokenKind::Char1(b')'))?;
        d.trace(if context { "FieldQuality/ctx" } else { "FieldQuality" });
        let mut qual = FieldQual::new(name, low, high);
        loop {
            match self.peek(d)? {
                TokenKind::SignedKey => { self.next(d)?; d.trace("field.signext"); qual.signext = true; }
                TokenKind::HexKey => { self.next(d)?; d.trace("field.hex"); qual.hex = true; }
                TokenKind::DecKey => { self.next(d)?; d.trace("field.dec"); qual.hex = false; }
                TokenKind::NoflowKey if context => {
                    self.next(d)?; d.trace("field.noflow"); qual.flow = false;
                }
                _ => break,
            }
        }
        Ok(qual)
    }

    // -----------------------------------------------------------------------
    // spaces / varnodes / bitranges / pcodeops (slghparse.y:216-239)
    // -----------------------------------------------------------------------

    /// `spacedef: spaceprop ';'` (slghparse.y:216).  At `SPACE_KEY`.
    fn spacedef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::SpaceKey)?;
        let name = self.expect_string(d)?;
        d.trace("SpaceQuality");
        let mut qual = SpaceQual::new(name);
        loop {
            match self.peek(d)? {
                TokenKind::TypeKey => {
                    self.next(d)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    match self.next(d)?.kind {
                        TokenKind::RamKey => { d.trace("space.type=ram"); qual.is_register = false; }
                        TokenKind::RegisterKey => { d.trace("space.type=register"); qual.is_register = true; }
                        other => return Err(KunaError::parse(format!("SLEIGH: bad space type {other:?}"))),
                    }
                }
                TokenKind::SizeKey => {
                    self.next(d)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let v = self.expect_integer(d)?;
                    d.trace("space.size");
                    qual.size = v as u32;
                }
                TokenKind::WordsizeKey => {
                    self.next(d)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let v = self.expect_integer(d)?;
                    d.trace("space.wordsize");
                    qual.wordsize = v as u32;
                }
                TokenKind::DefaultKey => {
                    self.next(d)?;
                    d.trace("space.default");
                    qual.isdefault = true;
                }
                _ => break,
            }
        }
        self.expect(d, TokenKind::Char1(b';'))?;
        d.trace("newSpace");
        d.new_space(qual);
        Ok(())
    }

    /// `varnodedef: DEFINE_KEY SPACESYM OFFSET_KEY '=' INTEGER SIZE_KEY '=' INTEGER
    /// stringlist ';'` (slghparse.y:226).  At `SPACESYM`.
    fn varnodedef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        let space = self.expect_symbol(d, TokenKind::SpaceSym)?;
        self.expect(d, TokenKind::OffsetKey)?;
        self.expect(d, TokenKind::Char1(b'='))?;
        if self.peek(d)? == TokenKind::BadInteger {
            self.next(d)?;
            d.report_error("Parsed integer is too big (overflow)");
            return Err(KunaError::parse("varnode offset overflow".to_string()));
        }
        let off = self.expect_integer(d)?;
        self.expect(d, TokenKind::SizeKey)?;
        self.expect(d, TokenKind::Char1(b'='))?;
        let size = self.expect_integer(d)?;
        let names = self.stringlist(d)?;
        self.expect(d, TokenKind::Char1(b';'))?;
        d.trace("defineVarnodes");
        d.define_varnodes(space, off, size, names);
        Ok(())
    }

    /// `bitrangedef: DEFINE_KEY BITRANGE_KEY bitrangelist ';'` (slghparse.y:230).
    fn bitrangedef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::BitrangeKey)?;
        loop {
            if self.peek(d)? == TokenKind::Char1(b';') {
                break;
            }
            let name = self.expect_string(d)?;
            self.expect(d, TokenKind::Char1(b'='))?;
            let var = self.expect_symbol(d, TokenKind::VarSym)?;
            self.expect(d, TokenKind::Char1(b'['))?;
            let lo = self.expect_integer(d)?;
            self.expect(d, TokenKind::Char1(b','))?;
            let hi = self.expect_integer(d)?;
            self.expect(d, TokenKind::Char1(b']'))?;
            d.trace("defineBitrange");
            d.define_bitrange(&name, var, lo as u32, hi as u32);
        }
        self.expect(d, TokenKind::Char1(b';'))?;
        Ok(())
    }

    /// `pcodeopdef: DEFINE_KEY PCODEOP_KEY stringlist ';'` (slghparse.y:238).
    fn pcodeopdef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::PcodeopKey)?;
        let names = self.stringlist(d)?;
        self.expect(d, TokenKind::Char1(b';'))?;
        d.trace("addUserOp");
        d.add_user_op(names);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // attaches (slghparse.y:240-245)
    // -----------------------------------------------------------------------

    /// `valueattach | nameattach | varattach` (slghparse.y:240-244).
    fn attach(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::AttachKey)?;
        match self.next(d)?.kind {
            TokenKind::ValuesKey => {
                let symlist = self.valuelist(d)?;
                let numlist = self.intblist(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("attachValues");
                d.attach_values(symlist, numlist);
            }
            TokenKind::NamesKey => {
                let symlist = self.valuelist(d)?;
                let names = self.anystringlist(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("attachNames");
                d.attach_names(symlist, names);
            }
            TokenKind::VariablesKey => {
                let symlist = self.valuelist(d)?;
                let varlist = self.varlist(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("attachVarnodes");
                d.attach_varnodes(symlist, varlist);
            }
            other => return Err(KunaError::parse(format!("SLEIGH: bad attach {other:?}"))),
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // macros (slghparse.y:246-267)
    // -----------------------------------------------------------------------

    /// `macrodef: macrostart '{' rtl '}'`; `macrostart: MACRO_KEY STRING '('
    /// oplist ')'` (slghparse.y:246,266).
    fn macrodef(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::MacroKey)?;
        let name = self.expect_string(d)?;
        self.expect(d, TokenKind::Char1(b'('))?;
        let params = self.oplist(d)?;
        self.expect(d, TokenKind::Char1(b')'))?;
        d.trace("createMacro");
        let sym = d.create_macro(&name, params);
        self.expect(d, TokenKind::Char1(b'{'))?;
        let rtl = self.rtl(d)?;
        self.expect(d, TokenKind::Char1(b'}'))?;
        d.trace("buildMacro");
        d.build_macro(sym, rtl);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // with-blocks (slghparse.y:249-265)
    // -----------------------------------------------------------------------

    /// `withblock: withblockmid '}'`; `withblockstart: WITH_KEY id_or_nil ':'
    /// bitpat_or_nil contextblock '{'` (slghparse.y:249-255).
    fn withblock(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        self.expect(d, TokenKind::WithKey)?;
        let ss = self.id_or_nil(d)?;
        self.expect(d, TokenKind::Char1(b':'))?;
        let pateq = self.bitpat_or_nil(d)?;
        let contblock = self.contextblock(d)?;
        self.expect(d, TokenKind::Char1(b'{'))?;
        d.trace("pushWith");
        d.push_with(ss, pateq, contblock);
        loop {
            match self.peek(d)? {
                TokenKind::Char1(b'}') => { self.next(d)?; break; }
                TokenKind::DefineKey => self.after_define(d)?,
                TokenKind::AttachKey => self.attach(d)?,
                TokenKind::MacroKey => self.macrodef(d)?,
                TokenKind::WithKey => self.withblock(d)?,
                TokenKind::Eof => return Err(KunaError::parse("SLEIGH: EOF inside with-block".to_string())),
                _ => self.constructor(d)?,
            }
        }
        d.trace("popWith");
        d.pop_with();
        Ok(())
    }

    /// `id_or_nil: /*empty*/ | SUBTABLESYM | STRING` (slghparse.y:257-260).
    fn id_or_nil(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Option<SymId>> {
        match self.peek(d)? {
            TokenKind::SubtableSym => {
                let sym = self.expect_symbol(d, TokenKind::SubtableSym)?;
                d.trace("id_or_nil/sub");
                Ok(Some(sym))
            }
            TokenKind::String => {
                let name = self.expect_string(d)?;
                d.trace("id_or_nil/newTable");
                Ok(Some(d.new_table(&name)))
            }
            _ => {
                d.trace("id_or_nil/nil");
                Ok(None)
            }
        }
    }

    /// `bitpat_or_nil: /*empty*/ | pequation` (slghparse.y:262-264).
    fn bitpat_or_nil(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Option<u32>> {
        match self.peek(d)? {
            TokenKind::Char1(b'[') | TokenKind::Char1(b'{') => {
                d.trace("bitpat_or_nil/nil");
                Ok(None)
            }
            _ => Ok(Some(self.pequation(d)?)),
        }
    }

    // -----------------------------------------------------------------------
    // constructors (slghparse.y:268-289)
    // -----------------------------------------------------------------------

    /// `constructor: (constructprint|subtablestart) IS_KEY pequation contextblock
    /// rtlbody` (slghparse.y:272-273).
    fn constructor(&mut self, d: &mut dyn ParseDriver) -> KunaResult<()> {
        let ct = self.subtablestart(d)?;
        let ct = self.constructprint(d, ct)?;
        self.expect(d, TokenKind::IsKey)?;
        let pateq = self.pequation(d)?;
        let contblock = self.contextblock(d)?;
        let vec = self.rtlbody(d)?;
        d.trace("buildConstructor");
        d.build_constructor(ct, Some(pateq), contblock, vec);
        Ok(())
    }

    /// `subtablestart: SUBTABLESYM ':' | STRING ':' | ':' | subtablestart ' '`
    /// (slghparse.y:285-288).  A leading run of print-state spaces is absorbed by
    /// `subtablestart ' '` (rule 288, traced) and does NOT consume the "first
    /// display piece" status -- the following piece is still the first
    /// `constructprint` piece (verified against the golden: `Rel8:  addr` yields
    /// `subtablestart/space` then `constructprint/SYMBOLSTRING`).
    fn subtablestart(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        let ct = match self.peek(d)? {
            TokenKind::SubtableSym => {
                let sym = self.expect_symbol(d, TokenKind::SubtableSym)?;
                self.expect(d, TokenKind::Char1(b':'))?;
                d.trace("createConstructor/sub");
                d.create_constructor(Some(sym))
            }
            TokenKind::String => {
                let name = self.expect_string(d)?;
                self.expect(d, TokenKind::Char1(b':'))?;
                d.trace("createConstructor/newTable");
                let sym = d.new_table(&name);
                d.create_constructor(Some(sym))
            }
            TokenKind::Char1(b':') => {
                self.next(d)?;
                d.trace("createConstructor/root");
                d.create_constructor(None)
            }
            other => {
                return Err(KunaError::parse(format!(
                    "SLEIGH: expected subtable start, got {other:?}"
                )))
            }
        };
        // `subtablestart ' '` (slghparse.y:288): absorb a leading run of
        // print-state spaces, tracing each (rule 288).
        while self.peek(d)? == TokenKind::Char1(b' ') {
            self.next(d)?;
            d.trace("subtablestart/space");
        }
        Ok(ct)
    }

    /// `constructprint` (slghparse.y:275-284): the rest of the display section --
    /// STRING / charstring / SYMBOLSTRING / '^' / ' ' pieces, until `is`.
    fn constructprint(&mut self, d: &mut dyn ParseDriver, ct: u32) -> KunaResult<u32> {
        let mut first = true;
        loop {
            match self.peek(d)? {
                TokenKind::IsKey => break,
                TokenKind::Char => {
                    // A run of CHAR tokens lumps into one `charstring`, appended as
                    // one syntax piece (`subtablestart|constructprint charstring`,
                    // slghparse.y:276,281,294,299 -> KT addSyntax/char).
                    let s = self.charstring(d)?;
                    d.trace("addSyntax/char");
                    d.add_syntax(ct, &s);
                }
                TokenKind::String => {
                    // A quoted STRING piece (`subtablestart|constructprint STRING`,
                    // slghparse.y:275,280,293,298 -> KT addSyntax/str).
                    let s = self.expect_string(d)?;
                    d.trace("addSyntax/str");
                    d.add_syntax(ct, &s);
                }
                TokenKind::SymbolString => {
                    let tok = self.next(d)?;
                    let name = token_str(&tok)?;
                    // Two grammar rules:
                    //  - `subtablestart SYMBOLSTRING` (slghparse.y:277, the FIRST
                    //    piece only): traces `constructprint/SYMBOLSTRING`; the
                    //    action is addSyntax if the constructor is in the root
                    //    table, else newOperand.
                    //  - `constructprint SYMBOLSTRING` (slghparse.y:283, any later
                    //    piece): traces `newOperand` and always makes an operand.
                    if first {
                        d.trace("constructprint/SYMBOLSTRING");
                        if d.is_in_root(ct) {
                            d.add_syntax(ct, &name);
                        } else {
                            d.new_operand(ct, &name);
                        }
                    } else {
                        d.trace("newOperand");
                        d.new_operand(ct, &name);
                    }
                }
                TokenKind::Char1(b'^') => {
                    self.next(d)?;
                    if first {
                        if !d.is_in_root(ct) {
                            d.report_error("Unexpected '^' at start of print pieces");
                            return Err(KunaError::parse("caret at start".to_string()));
                        }
                        d.trace("constructprint/caret0");
                    } else {
                        d.trace("constructprint/caret");
                    }
                }
                TokenKind::Char1(b' ') => {
                    self.next(d)?;
                    d.trace("addSyntax/space");
                    d.add_syntax(ct, b" ");
                }
                other => {
                    return Err(KunaError::parse(format!(
                        "SLEIGH: unexpected token in display section: {other:?}"
                    )))
                }
            }
            first = false;
        }
        Ok(ct)
    }

    // -----------------------------------------------------------------------
    // p-code sections (slghparse.y:268-365)
    // -----------------------------------------------------------------------

    /// `rtlbody: '{' rtl '}' | '{' rtlcontinue rtlmid '}' | OP_UNIMPL`
    /// (slghparse.y:268-271).
    fn rtlbody(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        if self.peek(d)? == TokenKind::OpUnimpl {
            self.next(d)?;
            d.trace("rtlbody/unimpl");
            // C++ returns a null SectionVector; the driver uses u32::MAX as the
            // "unimpl/no-sections" marker.
            return Ok(u32::MAX);
        }
        self.expect(d, TokenKind::Char1(b'{'))?;
        // The first chunk is always a full `rtl` (`rtlmid` plus an optional
        // `EXPORT ...;` tail), shared by both the standalone body (`'{' rtl '}'`)
        // and the named-section body (`rtlfirstsection: rtl section_def`,
        // slghparse.y:350).  bison reduces `rtl` (firing finish_main_rtl /
        // setResultVarnode) before deciding which `rtlbody` alternative applies,
        // so finish_rtl_tail must run on the first chunk regardless of whether a
        // `<<section>>` delimiter follows.
        let main_mid = self.rtlmid(d)?;
        let main = self.finish_rtl_tail(d, main_mid)?;
        if self.at_section_def(d)? {
            // rtlfirstsection: rtl section_def (slghparse.y:350).
            let firstsec = self.section_def(d)?;
            d.trace("firstNamedSection");
            let mut vec = d.first_named_section(main, firstsec);
            d.trace("rtlcontinue/first");
            loop {
                let mid = self.rtlmid(d)?;
                if self.at_section_def(d)? {
                    let sec = self.section_def(d)?;
                    d.trace("nextNamedSection");
                    vec = d.next_named_section(vec, mid, sec);
                } else {
                    self.expect(d, TokenKind::Char1(b'}'))?;
                    d.trace("finalNamedSection");
                    return Ok(d.final_named_section(vec, mid));
                }
            }
        } else {
            self.expect(d, TokenKind::Char1(b'}'))?;
            d.trace("standaloneSection");
            Ok(d.standalone_section(main))
        }
    }

    /// True if the lookahead begins a `section_def` (`<` rendered as OP_LEFT in
    /// the sem state).
    fn at_section_def(&mut self, d: &mut dyn ParseDriver) -> KunaResult<bool> {
        Ok(self.peek(d)? == TokenKind::OpLeft)
    }

    /// `section_def: OP_LEFT (STRING|SECTIONSYM) OP_RIGHT` (slghparse.y:347-348).
    fn section_def(&mut self, d: &mut dyn ParseDriver) -> KunaResult<SymId> {
        self.expect(d, TokenKind::OpLeft)?;
        let sym = match self.peek(d)? {
            TokenKind::SectionSym => {
                let s = self.expect_symbol(d, TokenKind::SectionSym)?;
                d.trace("section_def/sym");
                s
            }
            _ => {
                let name = self.expect_string(d)?;
                d.trace("newSectionSymbol");
                d.new_section_symbol(&name)
            }
        };
        self.expect(d, TokenKind::OpRight)?;
        Ok(sym)
    }

    /// `rtl: rtlmid` plus the optional EXPORT tail (slghparse.y:355-359).
    fn rtl(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        let mid = self.rtlmid(d)?;
        self.finish_rtl_tail(d, mid)
    }

    /// Finish an `rtl`: handle an optional `EXPORT ...;` tail, else finalize the
    /// main section (recording a NOP if empty) (slghparse.y:355-359).
    fn finish_rtl_tail(&mut self, d: &mut dyn ParseDriver, mid: u32) -> KunaResult<u32> {
        if self.accept(d, TokenKind::ExportKey)?.is_some() {
            if self.peek(d)? == TokenKind::Char1(b'*') {
                let star = self.sizedstar(d)?;
                let vn = self.lhsvarnode(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("setResultStarVarnode");
                Ok(d.set_result_star_varnode(mid, star, vn))
            } else {
                let vn = self.exportvarnode(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("setResultVarnode");
                Ok(d.set_result_varnode(mid, vn))
            }
        } else {
            d.trace("rtl/main");
            Ok(d.finish_main_rtl(mid))
        }
    }

    /// `rtlmid` (slghparse.y:361-365): the empty section, then a run of statements
    /// / local declarations.  Returns the (in-progress) section id.
    fn rtlmid(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        d.trace("enterSection");
        let sec = d.enter_section();
        loop {
            match self.peek(d)? {
                TokenKind::Char1(b'}') | TokenKind::ExportKey | TokenKind::OpLeft => break,
                TokenKind::LocalKey => {
                    self.local_form(d, sec)?;
                }
                _ => {
                    let stmt = self.statement(d)?;
                    d.trace("rtlmid/addOpList");
                    if !d.rtl_add_oplist(sec, stmt) {
                        d.report_error("Multiple delayslot declarations");
                        return Err(KunaError::parse("multiple delayslot".to_string()));
                    }
                }
            }
        }
        Ok(sec)
    }

    /// Handle a `local ...` form (slghparse.y:363-364 declarations, plus the
    /// `LOCAL_KEY ...` statement forms slghparse.y:367,369,371).
    fn local_form(&mut self, d: &mut dyn ParseDriver, sec: u32) -> KunaResult<()> {
        debug_assert_eq!(self.peek(d)?, TokenKind::LocalKey);
        self.next(d)?; // LOCAL_KEY
        if self.peek(d)? != TokenKind::String {
            // `LOCAL_KEY specificsymbol '='` -> redefinition error (slghparse.y:371).
            d.report_error("Redefinition of symbol");
            return Err(KunaError::parse("redefinition of symbol".to_string()));
        }
        let tok = self.next(d)?; // STRING
        let name = token_str(&tok)?;
        match self.peek(d)? {
            TokenKind::Char1(b';') => {
                self.next(d)?;
                d.trace("pcode.newLocalDefinition");
                d.pcode_new_local_definition(&name, None);
                Ok(())
            }
            TokenKind::Char1(b':') => {
                self.next(d)?; // ':'
                let sz = self.expect_integer(d)?;
                if self.peek(d)? == TokenKind::Char1(b';') {
                    self.next(d)?;
                    d.trace("pcode.newLocalDefinition/sz");
                    d.pcode_new_local_definition(&name, Some(sz));
                    Ok(())
                } else {
                    // `local NAME : INTEGER = expr ;` (slghparse.y:369).
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.newOutput/local.sz");
                    let stmt = d.pcode_new_output(true, e, &name, Some(sz));
                    d.trace("rtlmid/addOpList");
                    d.rtl_add_oplist(sec, stmt);
                    Ok(())
                }
            }
            TokenKind::Char1(b'=') => {
                // `local NAME = expr ;` (slghparse.y:367).
                self.next(d)?;
                let e = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.newOutput/local");
                let stmt = d.pcode_new_output(true, e, &name, None);
                d.trace("rtlmid/addOpList");
                d.rtl_add_oplist(sec, stmt);
                Ok(())
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token after `local {}`: {other:?}",
                String::from_utf8_lossy(&name)
            ))),
        }
    }

    // -----------------------------------------------------------------------
    // statements (slghparse.y:366-391)
    // -----------------------------------------------------------------------

    /// One `statement` (slghparse.y:366-391).  Returns the op-list id.
    fn statement(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            TokenKind::VarSym | TokenKind::SpecSym | TokenKind::OperandSym | TokenKind::JumpSym => {
                self.assign_or_bitrange_statement(d)
            }
            TokenKind::String => {
                let tok = self.next(d)?;
                let name = token_str(&tok)?;
                if self.peek(d)? == TokenKind::Char1(b':') {
                    self.next(d)?;
                    let sz = self.expect_integer(d)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.newOutput/sz");
                    Ok(d.pcode_new_output(true, e, &name, Some(sz)))
                } else {
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.newOutput");
                    Ok(d.pcode_new_output(false, e, &name, None))
                }
            }
            TokenKind::Char1(b'*') => {
                let star = self.sizedstar(d)?;
                let ptr = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b'='))?;
                let val = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.createStore");
                Ok(d.pcode_create_store(star, ptr, val))
            }
            TokenKind::UseropSym => {
                let sym = self.expect_symbol(d, TokenKind::UseropSym)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let params = self.paramlist(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.createUserOpNoOut");
                Ok(d.pcode_create_user_op_noout(sym, params))
            }
            TokenKind::BitSym => {
                let sym = self.expect_symbol(d, TokenKind::BitSym)?;
                self.expect(d, TokenKind::Char1(b'='))?;
                let e = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.assignBitRange/bitsym");
                Ok(d.pcode_assign_bitrange_bitsym(sym, e))
            }
            TokenKind::BuildKey => {
                self.next(d)?;
                let sym = self.expect_symbol(d, TokenKind::OperandSym)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.createOpConst/BUILD");
                let idx = d.operand_index(sym);
                Ok(d.pcode_create_op_const(ConstOp::Build, idx as u64))
            }
            TokenKind::CrossbuildKey => {
                self.next(d)?;
                let vn = self.varnode(d)?;
                self.expect(d, TokenKind::Char1(b','))?;
                if self.peek(d)? == TokenKind::SectionSym {
                    let sym = self.expect_symbol(d, TokenKind::SectionSym)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("createCrossBuild/sym");
                    Ok(d.create_cross_build(vn, sym))
                } else {
                    let name = self.expect_string(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("createCrossBuild/str");
                    let sym = d.new_section_symbol(&name);
                    Ok(d.create_cross_build(vn, sym))
                }
            }
            TokenKind::DelayslotKey => {
                self.next(d)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let v = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.createOpConst/DELAY");
                Ok(d.pcode_create_op_const(ConstOp::DelaySlot, v))
            }
            TokenKind::GotoKey => {
                self.next(d)?;
                if self.peek(d)? == TokenKind::Char1(b'[') {
                    self.next(d)?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b']'))?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.createOpNoOut/BRANCHIND");
                    Ok(d.pcode_create_op_noout(PcodeOpc::BranchInd, e, None))
                } else {
                    let j = self.jumpdest(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.createOpNoOut/BRANCH");
                    Ok(d.pcode_create_op_noout(PcodeOpc::Branch, j, None))
                }
            }
            TokenKind::IfKey => {
                self.next(d)?;
                let cond = self.expr(d)?;
                self.expect(d, TokenKind::GotoKey)?;
                let j = self.jumpdest(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.createOpNoOut/CBRANCH");
                Ok(d.pcode_create_op_noout(PcodeOpc::Cbranch, j, Some(cond)))
            }
            TokenKind::CallKey => {
                self.next(d)?;
                if self.peek(d)? == TokenKind::Char1(b'[') {
                    self.next(d)?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b']'))?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.createOpNoOut/CALLIND");
                    Ok(d.pcode_create_op_noout(PcodeOpc::CallInd, e, None))
                } else {
                    let j = self.jumpdest(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.createOpNoOut/CALL");
                    Ok(d.pcode_create_op_noout(PcodeOpc::Call, j, None))
                }
            }
            TokenKind::ReturnKey => {
                self.next(d)?;
                if self.peek(d)? == TokenKind::Char1(b'[') {
                    self.next(d)?;
                    let e = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b']'))?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("pcode.createOpNoOut/RETURN");
                    Ok(d.pcode_create_op_noout(PcodeOpc::Return, e, None))
                } else {
                    d.report_error("Must specify an indirect parameter for return");
                    Err(KunaError::parse("bare return".to_string()))
                }
            }
            TokenKind::MacroSym => {
                let sym = self.expect_symbol(d, TokenKind::MacroSym)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let params = self.paramlist(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("createMacroUse");
                Ok(d.create_macro_use_stmt(sym, params))
            }
            TokenKind::Char1(b'<') => {
                let label = self.label(d)?;
                d.trace("pcode.placeLabel");
                Ok(d.pcode_place_label(label))
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token starting statement: {other:?}"
            ))),
        }
    }

    /// `lhsvarnode '=' expr ';'` plus the bitrange/subpiece-LHS variants
    /// (slghparse.y:366,374,376,377).
    fn assign_or_bitrange_statement(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        let spec = self.specificsymbol(d)?;
        // The `:`/`(` LHS error forms (slghparse.y:376,377) reduce `varnode`, not
        // `lhsvarnode`, before erroring -- but they always fail, so the exact trace
        // is irrelevant.  All other forms reduce `lhsvarnode` from the
        // specificsymbol *now*, before the RHS (bison reduces leaves first).
        match self.peek(d)? {
            TokenKind::Char1(b':') => {
                self.next(d)?;
                let _ = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b'='))?;
                d.report_error("Illegal truncation on left-hand side of assignment");
                return Err(KunaError::parse("illegal truncation lhs".to_string()));
            }
            TokenKind::Char1(b'(') => {
                self.next(d)?;
                let _ = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                d.report_error("Illegal subpiece on left-hand side of assignment");
                return Err(KunaError::parse("illegal subpiece lhs".to_string()));
            }
            _ => {}
        }
        d.trace("lhsvarnode/spec");
        let lhs = d.lhsvarnode_spec(spec);
        match self.peek(d)? {
            TokenKind::Char1(b'[') => {
                // `lhsvarnode '[' INTEGER ',' INTEGER ']' '=' expr ';'`
                // (slghparse.y:374).
                self.next(d)?;
                let off = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b','))?;
                let size = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b']'))?;
                self.expect(d, TokenKind::Char1(b'='))?;
                let e = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("pcode.assignBitRange/idx");
                Ok(d.pcode_assign_bitrange_idx(lhs, off as u32, size as u32, e))
            }
            _ => {
                // `lhsvarnode '=' expr ';'` (slghparse.y:366).
                self.expect(d, TokenKind::Char1(b'='))?;
                let e = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b';'))?;
                d.trace("stmt/assign");
                Ok(d.stmt_assign(lhs, e))
            }
        }
    }

    // -----------------------------------------------------------------------
    // pattern equations / expressions (slghparse.y:290-346)
    // -----------------------------------------------------------------------

    /// `pequation` with the `& | ;` operators (slghparse.y:309-312).  Binding
    /// powers from the %left table (slghparse.y:82-93): `|`=1, `;`=2, `&`=3.
    fn pequation(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        self.pequation_bp(d, 0)
    }

    fn pequation_bp(&mut self, d: &mut dyn ParseDriver, min_bp: u8) -> KunaResult<u32> {
        let mut lhs = self.pequation_atom(d)?;
        loop {
            let (lbp, op): (u8, fn(&mut dyn ParserActions, u32, u32) -> u32) =
                match self.peek(d)? {
                    TokenKind::Char1(b'|') => (1, |a, l, r| { a.trace("peq/Or"); a.peq_or(l, r) }),
                    TokenKind::Char1(b';') => (2, |a, l, r| { a.trace("peq/Cat"); a.peq_cat(l, r) }),
                    TokenKind::Char1(b'&') => (3, |a, l, r| { a.trace("peq/And"); a.peq_and(l, r) }),
                    _ => break,
                };
            if lbp < min_bp {
                break;
            }
            self.next(d)?; // operator
            let rhs = self.pequation_bp(d, lbp + 1)?;
            lhs = op(d.as_actions(), lhs, rhs);
        }
        Ok(lhs)
    }

    /// `elleq`/`ellrt`/`atomic`/`constraint` (slghparse.y:314-336): the ellipsis
    /// wrappers around an atomic constraint.
    fn pequation_atom(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        if self.peek(d)? == TokenKind::EllipsisKey {
            self.next(d)?;
            let inner = self.ellrt(d)?;
            d.trace("peq/LeftEllipsis");
            return Ok(d.peq_left_ellipsis(inner));
        }
        self.ellrt(d)
    }

    /// `ellrt: atomic ELLIPSIS_KEY | atomic` (slghparse.y:317-319).
    fn ellrt(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        let a = self.atomic(d)?;
        if self.peek(d)? == TokenKind::EllipsisKey {
            self.next(d)?;
            d.trace("peq/RightEllipsis");
            Ok(d.peq_right_ellipsis(a))
        } else {
            Ok(a)
        }
    }

    /// `atomic: constraint | '(' pequation ')'` (slghparse.y:320-322).
    fn atomic(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        if self.peek(d)? == TokenKind::Char1(b'(') {
            self.next(d)?;
            let e = self.pequation(d)?;
            self.expect(d, TokenKind::Char1(b')'))?;
            d.trace("peq/paren");
            return Ok(e);
        }
        self.constraint(d)
    }

    /// `constraint` (slghparse.y:323-336).
    fn constraint(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            k if is_familysymbol(k) => {
                let fam = self.familysymbol(d)?;
                match self.peek(d)? {
                    TokenKind::Char1(b'=') => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/Equal");
                        Ok(d.peq_equal(fam, rhs))
                    }
                    TokenKind::OpNotEqual => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/NotEqual");
                        Ok(d.peq_notequal(fam, rhs))
                    }
                    TokenKind::Char1(b'<') => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/Less");
                        Ok(d.peq_less(fam, rhs))
                    }
                    TokenKind::OpLessEqual => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/LessEqual");
                        Ok(d.peq_lessequal(fam, rhs))
                    }
                    TokenKind::Char1(b'>') => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/Greater");
                        Ok(d.peq_greater(fam, rhs))
                    }
                    TokenKind::OpGreatEqual => {
                        self.next(d)?;
                        let rhs = self.pexpression(d)?;
                        d.trace("peq/GreaterEqual");
                        Ok(d.peq_greaterequal(fam, rhs))
                    }
                    _ => {
                        d.trace("defineInvisibleOperand/fam");
                        match d.define_invisible_operand(fam) {
                            Some(id) => Ok(id),
                            None => Err(KunaError::parse("invisible operand failed".to_string())),
                        }
                    }
                }
            }
            TokenKind::OperandSym => {
                let sym = self.expect_symbol(d, TokenKind::OperandSym)?;
                if self.peek(d)? == TokenKind::Char1(b'=') {
                    self.next(d)?;
                    let rhs = self.pexpression(d)?;
                    d.trace("constrainOperand");
                    match d.constrain_operand(sym, rhs) {
                        Some(id) => Ok(id),
                        None => {
                            d.report_error("Constraining currently undefined operand");
                            Err(KunaError::parse("constrain undefined operand".to_string()))
                        }
                    }
                } else {
                    d.trace("peq/OperandEquation+selfDefine");
                    let eq = d.peq_operand_equation(sym);
                    d.self_define(sym);
                    Ok(eq)
                }
            }
            TokenKind::SpecSym => {
                let sym = self.expect_symbol(d, TokenKind::SpecSym)?;
                d.trace("peq/Unconstrained");
                Ok(d.peq_unconstrained(sym))
            }
            TokenKind::SubtableSym => {
                let sym = self.expect_symbol(d, TokenKind::SubtableSym)?;
                d.trace("defineInvisibleOperand/sub");
                match d.define_invisible_operand(sym) {
                    Some(id) => Ok(id),
                    None => Err(KunaError::parse("invisible operand (subtable) failed".to_string())),
                }
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in pattern constraint: {other:?}"
            ))),
        }
    }

    /// `pexpression` with full operator precedence (slghparse.y:290-308).
    fn pexpression(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        self.pexpression_bp(d, 0)
    }

    fn pexpression_bp(&mut self, d: &mut dyn ParseDriver, min_bp: u8) -> KunaResult<u32> {
        let mut lhs = self.pexpression_atom(d)?;
        loop {
            // %left table (slghparse.y:82-93): `|`=1, `^`=2, `&`=3, shifts=4,
            // `+`/`-`=5, `*`/`/`=6.
            let (lbp, op): (u8, fn(&mut dyn ParserActions, u32, u32) -> u32) =
                match self.peek(d)? {
                    TokenKind::OpOr => (1, |a, l, r| { a.trace("pexp/Or"); a.pexp_or(l, r) }),
                    TokenKind::OpXor => (2, |a, l, r| { a.trace("pexp/Xor"); a.pexp_xor(l, r) }),
                    TokenKind::OpAnd => (3, |a, l, r| { a.trace("pexp/And"); a.pexp_and(l, r) }),
                    TokenKind::OpLeft => (4, |a, l, r| { a.trace("pexp/LeftShift"); a.pexp_leftshift(l, r) }),
                    TokenKind::OpRight => (4, |a, l, r| { a.trace("pexp/RightShift"); a.pexp_rightshift(l, r) }),
                    TokenKind::Char1(b'+') => (5, |a, l, r| { a.trace("pexp/Plus"); a.pexp_plus(l, r) }),
                    TokenKind::Char1(b'-') => (5, |a, l, r| { a.trace("pexp/Sub"); a.pexp_sub(l, r) }),
                    TokenKind::Char1(b'*') => (6, |a, l, r| { a.trace("pexp/Mult"); a.pexp_mult(l, r) }),
                    TokenKind::Char1(b'/') => (6, |a, l, r| { a.trace("pexp/Div"); a.pexp_div(l, r) }),
                    _ => break,
                };
            if lbp < min_bp {
                break;
            }
            self.next(d)?;
            let rhs = self.pexpression_bp(d, lbp + 1)?;
            lhs = op(d.as_actions(), lhs, rhs);
        }
        Ok(lhs)
    }

    /// `pexpression` atoms: literals, symbols, parens, and the unary `- ~`
    /// (slghparse.y:290-307).
    fn pexpression_atom(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            TokenKind::Intb => {
                let v = self.expect_intb(d)?;
                d.trace("pexp/ConstantValue");
                Ok(d.pexp_constant(v))
            }
            TokenKind::Char1(b'(') => {
                self.next(d)?;
                let e = self.pexpression(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                d.trace("pexp/paren");
                Ok(e)
            }
            // unary minus: `'-' pexpression %prec '!'` (slghparse.y:306).
            TokenKind::Char1(b'-') => {
                self.next(d)?;
                let e = self.pexpression_atom(d)?;
                d.trace("pexp/Minus");
                Ok(d.pexp_minus(e))
            }
            // unary not: `'~' pexpression` (slghparse.y:307).
            TokenKind::Char1(b'~') => {
                self.next(d)?;
                let e = self.pexpression_atom(d)?;
                d.trace("pexp/Not");
                Ok(d.pexp_not(e))
            }
            k if is_familysymbol(k) => {
                let fam = self.familysymbol(d)?;
                d.trace("pexp/familysym.getPatternValue");
                Ok(d.pexp_family_value(fam))
            }
            k if is_specificsymbol(k) => {
                let spec = self.specificsymbol(d)?;
                d.trace("pexp/specsym.getPatternExpression");
                Ok(d.pexp_spec_expression(spec))
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in pattern expression: {other:?}"
            ))),
        }
    }

    // -----------------------------------------------------------------------
    // context block (slghparse.y:337-346)
    // -----------------------------------------------------------------------

    /// `contextblock: /*empty*/ | '[' contextlist ']'` (slghparse.y:337-339).
    fn contextblock(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Option<Vec<u32>>> {
        if self.peek(d)? != TokenKind::Char1(b'[') {
            d.trace("contextblock/empty");
            return Ok(None);
        }
        self.next(d)?; // '['
        d.trace("contextlist/new");
        let mut vec: Vec<u32> = Vec::new();
        loop {
            match self.peek(d)? {
                TokenKind::Char1(b']') => { self.next(d)?; break; }
                TokenKind::ContextSym => {
                    let sym = self.expect_symbol(d, TokenKind::ContextSym)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let pe = self.pexpression(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("contextMod");
                    if !d.context_mod(&mut vec, sym, pe) {
                        d.report_error("Cannot use 'inst_next' or 'inst_next2' to set context variable");
                        return Err(KunaError::parse("bad context mod".to_string()));
                    }
                }
                TokenKind::GlobalsetKey => {
                    self.next(d)?;
                    self.expect(d, TokenKind::Char1(b'('))?;
                    let is_spec = is_specificsymbol(self.peek(d)?);
                    let sym = if is_spec {
                        self.specificsymbol(d)?
                    } else {
                        self.familysymbol(d)?
                    };
                    self.expect(d, TokenKind::Char1(b','))?;
                    let cvar = self.expect_symbol(d, TokenKind::ContextSym)?;
                    self.expect(d, TokenKind::Char1(b')'))?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace(if is_spec { "contextSet/spec" } else { "contextSet" });
                    d.context_set(&mut vec, sym, cvar);
                }
                TokenKind::OperandSym => {
                    let sym = self.expect_symbol(d, TokenKind::OperandSym)?;
                    self.expect(d, TokenKind::Char1(b'='))?;
                    let pe = self.pexpression(d)?;
                    self.expect(d, TokenKind::Char1(b';'))?;
                    d.trace("defineOperand");
                    d.define_operand(sym, pe);
                }
                other => {
                    d.report_error("Expecting context symbol");
                    return Err(KunaError::parse(format!(
                        "SLEIGH: bad context list entry {other:?}"
                    )));
                }
            }
        }
        d.trace("contextblock/list");
        Ok(Some(vec))
    }

    // -----------------------------------------------------------------------
    // expr (slghparse.y:392-459)
    // -----------------------------------------------------------------------

    /// `expr` with full p-code operator precedence (slghparse.y:392-459).
    fn expr(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        self.expr_bp(d, 0)
    }

    fn expr_bp(&mut self, d: &mut dyn ParseDriver, min_bp: u8) -> KunaResult<u32> {
        let mut lhs = self.expr_atom(d)?;
        loop {
            // Binding powers from the %left table (slghparse.y:82-93): OP_BOOL_OR=1;
            // OP_BOOL_AND/XOR=2; `|`/OP_OR=3; `^`/OP_XOR=5; `&`/OP_AND=6;
            // equality=7; comparisons(nonassoc)=8; shifts=9; add/sub=10;
            // mul/div/rem=11.
            let (lbp, opc, trace): (u8, PcodeOpc, &str) = match self.peek(d)? {
                TokenKind::OpBoolOr => (1, PcodeOpc::BoolOr, "pcode.createOp/CPUI_BOOL_OR"),
                TokenKind::OpBoolAnd => (2, PcodeOpc::BoolAnd, "pcode.createOp/CPUI_BOOL_AND"),
                TokenKind::OpBoolXor => (2, PcodeOpc::BoolXor, "pcode.createOp/CPUI_BOOL_XOR"),
                TokenKind::Char1(b'|') => (3, PcodeOpc::IntOr, "pcode.createOp/CPUI_INT_OR"),
                TokenKind::OpOr => (3, PcodeOpc::IntOr, "pcode.createOp/CPUI_INT_OR"),
                TokenKind::Char1(b'^') => (5, PcodeOpc::IntXor, "pcode.createOp/CPUI_INT_XOR"),
                TokenKind::OpXor => (5, PcodeOpc::IntXor, "pcode.createOp/CPUI_INT_XOR"),
                TokenKind::Char1(b'&') => (6, PcodeOpc::IntAnd, "pcode.createOp/CPUI_INT_AND"),
                TokenKind::OpAnd => (6, PcodeOpc::IntAnd, "pcode.createOp/CPUI_INT_AND"),
                TokenKind::OpEqual => (7, PcodeOpc::IntEqual, "pcode.createOp/CPUI_INT_EQUAL"),
                TokenKind::OpNotEqual => (7, PcodeOpc::IntNotEqual, "pcode.createOp/CPUI_INT_NOTEQUAL"),
                TokenKind::OpFEqual => (7, PcodeOpc::FloatEqual, "pcode.createOp/CPUI_FLOAT_EQUAL"),
                TokenKind::OpFNotEqual => (7, PcodeOpc::FloatNotEqual, "pcode.createOp/CPUI_FLOAT_NOTEQUAL"),
                TokenKind::Char1(b'<') => (8, PcodeOpc::IntLess, "pcode.createOp/CPUI_INT_LESS"),
                TokenKind::Char1(b'>') => (8, PcodeOpc::IntLess, "<>swap"),
                TokenKind::OpGreatEqual => (8, PcodeOpc::IntLessEqual, "<>swap"),
                TokenKind::OpLessEqual => (8, PcodeOpc::IntLessEqual, "pcode.createOp/CPUI_INT_LESSEQUAL"),
                TokenKind::OpSless => (8, PcodeOpc::IntSless, "pcode.createOp/CPUI_INT_SLESS"),
                TokenKind::OpSgreatEqual => (8, PcodeOpc::IntSlessEqual, "<>swap"),
                TokenKind::OpSlessEqual => (8, PcodeOpc::IntSlessEqual, "pcode.createOp/CPUI_INT_SLESSEQUAL"),
                TokenKind::OpSgreat => (8, PcodeOpc::IntSless, "<>swap"),
                TokenKind::OpFless => (8, PcodeOpc::FloatLess, "pcode.createOp/CPUI_FLOAT_LESS"),
                TokenKind::OpFgreat => (8, PcodeOpc::FloatLess, "<>swap"),
                TokenKind::OpFlessEqual => (8, PcodeOpc::FloatLessEqual, "pcode.createOp/CPUI_FLOAT_LESSEQUAL"),
                TokenKind::OpFgreatEqual => (8, PcodeOpc::FloatLessEqual, "<>swap"),
                TokenKind::OpLeft => (9, PcodeOpc::IntLeft, "pcode.createOp/CPUI_INT_LEFT"),
                TokenKind::OpRight => (9, PcodeOpc::IntRight, "pcode.createOp/CPUI_INT_RIGHT"),
                TokenKind::OpSright => (9, PcodeOpc::IntSright, "pcode.createOp/CPUI_INT_SRIGHT"),
                TokenKind::Char1(b'+') => (10, PcodeOpc::IntAdd, "pcode.createOp/CPUI_INT_ADD"),
                TokenKind::Char1(b'-') => (10, PcodeOpc::IntSub, "pcode.createOp/CPUI_INT_SUB"),
                TokenKind::OpFadd => (10, PcodeOpc::FloatAdd, "pcode.createOp/CPUI_FLOAT_ADD"),
                TokenKind::OpFsub => (10, PcodeOpc::FloatSub, "pcode.createOp/CPUI_FLOAT_SUB"),
                TokenKind::Char1(b'*') => (11, PcodeOpc::IntMult, "pcode.createOp/CPUI_INT_MULT"),
                TokenKind::Char1(b'/') => (11, PcodeOpc::IntDiv, "pcode.createOp/CPUI_INT_DIV"),
                TokenKind::Char1(b'%') => (11, PcodeOpc::IntRem, "pcode.createOp/CPUI_INT_REM"),
                TokenKind::OpSdiv => (11, PcodeOpc::IntSdiv, "pcode.createOp/CPUI_INT_SDIV"),
                TokenKind::OpSrem => (11, PcodeOpc::IntSrem, "pcode.createOp/CPUI_INT_SREM"),
                TokenKind::OpFmult => (11, PcodeOpc::FloatMult, "pcode.createOp/CPUI_FLOAT_MULT"),
                TokenKind::OpFdiv => (11, PcodeOpc::FloatDiv, "pcode.createOp/CPUI_FLOAT_DIV"),
                _ => break,
            };
            if lbp < min_bp {
                break;
            }
            let optok = self.next(d)?.kind;
            let rhs = self.expr_bp(d, lbp + 1)?;
            lhs = if trace == "<>swap" {
                let (t, a, b) = swap_compare(optok, lhs, rhs);
                d.trace(t);
                d.pcode_create_op(opc, a, Some(b))
            } else {
                d.trace(trace);
                d.pcode_create_op(opc, lhs, Some(rhs))
            };
        }
        Ok(lhs)
    }

    /// `expr` atoms (slghparse.y:392-458).
    fn expr_atom(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            // `sizedstar expr %prec '!'` -> load (slghparse.y:393).
            TokenKind::Char1(b'*') => {
                let star = self.sizedstar(d)?;
                let e = self.expr_atom(d)?;
                d.trace("pcode.createLoad");
                Ok(d.pcode_create_load(star, e))
            }
            TokenKind::Char1(b'(') => {
                self.next(d)?;
                let e = self.expr(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                d.trace("expr/paren");
                Ok(e)
            }
            // unary `'-' expr %prec '!'` (slghparse.y:407).
            TokenKind::Char1(b'-') => {
                self.next(d)?;
                let e = self.expr_atom(d)?;
                d.trace("pcode.createOp/CPUI_INT_2COMP");
                Ok(d.pcode_create_op(PcodeOpc::Int2Comp, e, None))
            }
            // unary `'~' expr` (slghparse.y:408).
            TokenKind::Char1(b'~') => {
                self.next(d)?;
                let e = self.expr_atom(d)?;
                d.trace("pcode.createOp/CPUI_INT_NEGATE");
                Ok(d.pcode_create_op(PcodeOpc::IntNegate, e, None))
            }
            // unary `'!' expr` (slghparse.y:420).
            TokenKind::Char1(b'!') => {
                self.next(d)?;
                let e = self.expr_atom(d)?;
                d.trace("pcode.createOp/CPUI_BOOL_NEGATE");
                Ok(d.pcode_create_op(PcodeOpc::BoolNegate, e, None))
            }
            // `OP_FSUB expr %prec '!'` -> float neg (slghparse.y:434).
            TokenKind::OpFsub => {
                self.next(d)?;
                let e = self.expr_atom(d)?;
                d.trace("pcode.createOp/CPUI_FLOAT_NEG");
                Ok(d.pcode_create_op(PcodeOpc::FloatNeg, e, None))
            }
            // intrinsic one-arg ops (slghparse.y:435-452).
            TokenKind::OpAbs => self.intrinsic1(d, PcodeOpc::FloatAbs, "pcode.createOp/CPUI_FLOAT_ABS"),
            TokenKind::OpSqrt => self.intrinsic1(d, PcodeOpc::FloatSqrt, "pcode.createOp/CPUI_FLOAT_SQRT"),
            TokenKind::OpSext => self.intrinsic1(d, PcodeOpc::IntSext, "pcode.createOp/CPUI_INT_SEXT"),
            TokenKind::OpZext => self.intrinsic1(d, PcodeOpc::IntZext, "pcode.createOp/CPUI_INT_ZEXT"),
            TokenKind::OpFloat2Float => self.intrinsic1(d, PcodeOpc::FloatFloat2Float, "pcode.createOp/CPUI_FLOAT_FLOAT2FLOAT"),
            TokenKind::OpInt2Float => self.intrinsic1(d, PcodeOpc::FloatInt2Float, "pcode.createOp/CPUI_FLOAT_INT2FLOAT"),
            TokenKind::OpNan => self.intrinsic1(d, PcodeOpc::FloatNan, "pcode.createOp/CPUI_FLOAT_NAN"),
            TokenKind::OpTrunc => self.intrinsic1(d, PcodeOpc::FloatTrunc, "pcode.createOp/CPUI_FLOAT_TRUNC"),
            TokenKind::OpCeil => self.intrinsic1(d, PcodeOpc::FloatCeil, "pcode.createOp/CPUI_FLOAT_CEIL"),
            TokenKind::OpFloor => self.intrinsic1(d, PcodeOpc::FloatFloor, "pcode.createOp/CPUI_FLOAT_FLOOR"),
            TokenKind::OpRound => self.intrinsic1(d, PcodeOpc::FloatRound, "pcode.createOp/CPUI_FLOAT_ROUND"),
            TokenKind::OpPopcount => self.intrinsic1(d, PcodeOpc::Popcount, "pcode.createOp/CPUI_POPCOUNT"),
            TokenKind::OpLzcount => self.intrinsic1(d, PcodeOpc::Lzcount, "pcode.createOp/CPUI_LZCOUNT"),
            // two-arg intrinsics (slghparse.y:439-441).
            TokenKind::OpCarry => self.intrinsic2(d, PcodeOpc::IntCarry, "pcode.createOp/CPUI_INT_CARRY"),
            TokenKind::OpScarry => self.intrinsic2(d, PcodeOpc::IntScarry, "pcode.createOp/CPUI_INT_SCARRY"),
            TokenKind::OpSborrow => self.intrinsic2(d, PcodeOpc::IntSborrow, "pcode.createOp/CPUI_INT_SBORROW"),
            // `OP_NEW '(' expr ')' | OP_NEW '(' expr ',' expr ')'`
            // (slghparse.y:449-450).
            TokenKind::OpNew => {
                self.next(d)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let a = self.expr(d)?;
                if self.peek(d)? == TokenKind::Char1(b',') {
                    self.next(d)?;
                    let b = self.expr(d)?;
                    self.expect(d, TokenKind::Char1(b')'))?;
                    d.trace("pcode.createOp/CPUI_NEW");
                    Ok(d.pcode_create_op(PcodeOpc::New, a, Some(b)))
                } else {
                    self.expect(d, TokenKind::Char1(b')'))?;
                    d.trace("pcode.createOp/CPUI_NEW");
                    Ok(d.pcode_create_op(PcodeOpc::New, a, None))
                }
            }
            // `OP_CPOOLREF '(' paramlist ')'` (slghparse.y:458).
            TokenKind::OpCpoolref => {
                self.next(d)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let params = self.paramlist(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                if params.len() < 2 {
                    d.report_error("Must be at least two inputs to cpool");
                    return Err(KunaError::parse("cpool < 2 inputs".to_string()));
                }
                d.trace("pcode.createVariadic/CPOOLREF");
                Ok(d.pcode_create_variadic_cpoolref(params))
            }
            // `USEROPSYM '(' paramlist ')'` (slghparse.y:457).
            TokenKind::UseropSym => {
                let sym = self.expect_symbol(d, TokenKind::UseropSym)?;
                self.expect(d, TokenKind::Char1(b'('))?;
                let params = self.paramlist(d)?;
                self.expect(d, TokenKind::Char1(b')'))?;
                d.trace("pcode.createUserOp");
                Ok(d.pcode_create_user_op(sym, params))
            }
            // `BITSYM` (slghparse.y:456).
            TokenKind::BitSym => {
                let sym = self.expect_symbol(d, TokenKind::BitSym)?;
                d.trace("pcode.createBitRange/bitsym");
                Ok(d.pcode_create_bitrange_bitsym(sym))
            }
            // specificsymbol followed by `(`, `:`, or `[` -> subpiece / bitrange
            // (slghparse.y:453-455); else a plain varnode expr.
            k if is_specificsymbol(k) => {
                let spec = self.specificsymbol(d)?;
                match self.peek(d)? {
                    TokenKind::Char1(b'(') => {
                        self.next(d)?;
                        let iv = self.expect_integer_value(d)?;
                        self.expect(d, TokenKind::Char1(b')'))?;
                        d.trace("pcode.createOp/CPUI_SUBPIECE");
                        Ok(d.pcode_create_subpiece(spec, iv as u32))
                    }
                    TokenKind::Char1(b':') => {
                        self.next(d)?;
                        let n = self.expect_integer(d)?;
                        d.trace("pcode.createBitRange/colon");
                        Ok(d.pcode_create_bitrange_colon(spec, n))
                    }
                    TokenKind::Char1(b'[') => {
                        self.next(d)?;
                        let off = self.expect_integer(d)?;
                        self.expect(d, TokenKind::Char1(b','))?;
                        let size = self.expect_integer(d)?;
                        self.expect(d, TokenKind::Char1(b']'))?;
                        d.trace("pcode.createBitRange/idx");
                        Ok(d.pcode_create_bitrange_idx(spec, off as u32, size as u32))
                    }
                    _ => {
                        d.trace("varnode/spec");
                        let vn = d.varnode_spec(spec);
                        d.trace("expr/varnode");
                        Ok(d.expr_from_varnode(vn))
                    }
                }
            }
            // integervarnode forms and `&` addressOf as a varnode -> expr.
            TokenKind::Integer | TokenKind::BadInteger | TokenKind::Char1(b'&') => {
                let vn = self.varnode(d)?;
                d.trace("expr/varnode");
                Ok(d.expr_from_varnode(vn))
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in expression: {other:?}"
            ))),
        }
    }

    fn intrinsic1(&mut self, d: &mut dyn ParseDriver, opc: PcodeOpc, trace: &str) -> KunaResult<u32> {
        self.next(d)?; // the OP_* keyword
        self.expect(d, TokenKind::Char1(b'('))?;
        let a = self.expr(d)?;
        self.expect(d, TokenKind::Char1(b')'))?;
        d.trace(trace);
        Ok(d.pcode_create_op(opc, a, None))
    }

    fn intrinsic2(&mut self, d: &mut dyn ParseDriver, opc: PcodeOpc, trace: &str) -> KunaResult<u32> {
        self.next(d)?;
        self.expect(d, TokenKind::Char1(b'('))?;
        let a = self.expr(d)?;
        self.expect(d, TokenKind::Char1(b','))?;
        let b = self.expr(d)?;
        self.expect(d, TokenKind::Char1(b')'))?;
        d.trace(trace);
        Ok(d.pcode_create_op(opc, a, Some(b)))
    }

    // -----------------------------------------------------------------------
    // varnode / integervarnode / sizedstar / jumpdest / label (slghparse.y:460-509)
    // -----------------------------------------------------------------------

    /// `sizedstar` (slghparse.y:460-464).
    fn sizedstar(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        self.expect(d, TokenKind::Char1(b'*'))?;
        match self.peek(d)? {
            TokenKind::Char1(b'[') => {
                self.next(d)?;
                let space = self.expect_symbol(d, TokenKind::SpaceSym)?;
                self.expect(d, TokenKind::Char1(b']'))?;
                if self.peek(d)? == TokenKind::Char1(b':') {
                    self.next(d)?;
                    let sz = self.expect_integer(d)?;
                    d.trace("sizedstar/space:sz");
                    Ok(d.sizedstar_space_sz(space, sz))
                } else {
                    d.trace("sizedstar/space");
                    Ok(d.sizedstar_space(space))
                }
            }
            TokenKind::Char1(b':') => {
                self.next(d)?;
                let sz = self.expect_integer(d)?;
                d.trace("sizedstar/:sz");
                Ok(d.sizedstar_default_sz(sz))
            }
            _ => {
                d.trace("sizedstar/bare");
                Ok(d.sizedstar_default())
            }
        }
    }

    /// `jumpdest` (slghparse.y:465-472).
    fn jumpdest(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            TokenKind::JumpSym => {
                let sym = self.expect_symbol(d, TokenKind::JumpSym)?;
                d.trace("jumpdest/JUMPSYM");
                Ok(d.jumpdest_jumpsym(sym))
            }
            TokenKind::Integer => {
                let v = self.expect_integer(d)?;
                if self.peek(d)? == TokenKind::Char1(b'[') {
                    self.next(d)?;
                    let space = self.expect_symbol(d, TokenKind::SpaceSym)?;
                    self.expect(d, TokenKind::Char1(b']'))?;
                    d.trace("jumpdest/INTEGER[SPACE]");
                    Ok(d.jumpdest_integer_space(v, space))
                } else {
                    d.trace("jumpdest/INTEGER");
                    Ok(d.jumpdest_integer(v))
                }
            }
            TokenKind::BadInteger => {
                self.next(d)?;
                d.report_error("Parsed integer is too big (overflow)");
                d.trace("jumpdest/BADINTEGER");
                Ok(d.jumpdest_integer(0))
            }
            TokenKind::OperandSym => {
                let sym = self.expect_symbol(d, TokenKind::OperandSym)?;
                d.trace("jumpdest/OPERANDSYM");
                Ok(d.jumpdest_operandsym(sym))
            }
            TokenKind::Char1(b'<') => {
                let label = self.label(d)?;
                d.trace("jumpdest/label");
                Ok(d.jumpdest_label(label))
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in jump destination: {other:?}"
            ))),
        }
    }

    /// `varnode: specificsymbol | integervarnode` (slghparse.y:473-477).
    fn varnode(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            k if is_specificsymbol(k) => {
                let spec = self.specificsymbol(d)?;
                d.trace("varnode/spec");
                Ok(d.varnode_spec(spec))
            }
            _ => {
                let iv = self.integervarnode(d)?;
                d.trace("varnode/int");
                Ok(iv)
            }
        }
    }

    /// `integervarnode` (slghparse.y:478-483).
    fn integervarnode(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            TokenKind::Integer => {
                let v = self.expect_integer(d)?;
                if self.peek(d)? == TokenKind::Char1(b':') {
                    self.next(d)?;
                    let sz = self.expect_integer(d)?;
                    d.trace("intvn/INTEGER:INTEGER");
                    Ok(d.intvn_integer_colon(v, sz))
                } else {
                    d.trace("intvn/INTEGER");
                    Ok(d.intvn_integer(v))
                }
            }
            TokenKind::BadInteger => {
                self.next(d)?;
                d.report_error("Parsed integer is too big (overflow)");
                d.trace("intvn/BADINTEGER");
                Ok(d.intvn_integer(0))
            }
            TokenKind::Char1(b'&') => {
                self.next(d)?;
                if self.peek(d)? == TokenKind::Char1(b':') {
                    self.next(d)?;
                    let sz = self.expect_integer(d)?;
                    let vn = self.varnode(d)?;
                    d.trace("pcode.addressOf/sz");
                    Ok(d.pcode_address_of(vn, sz))
                } else {
                    let vn = self.varnode(d)?;
                    d.trace("pcode.addressOf/0");
                    Ok(d.pcode_address_of(vn, 0))
                }
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in integer varnode: {other:?}"
            ))),
        }
    }

    /// `lhsvarnode: specificsymbol` (slghparse.y:484-487).
    fn lhsvarnode(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        let spec = self.specificsymbol(d)?;
        d.trace("lhsvarnode/spec");
        Ok(d.lhsvarnode_spec(spec))
    }

    /// `exportvarnode` (slghparse.y:491-497).
    fn exportvarnode(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        match self.peek(d)? {
            k if is_specificsymbol(k) => {
                let spec = self.specificsymbol(d)?;
                d.trace("exportvn/spec");
                Ok(d.exportvarnode_spec(spec))
            }
            TokenKind::Char1(b'&') => {
                self.next(d)?;
                if self.peek(d)? == TokenKind::Char1(b':') {
                    self.next(d)?;
                    let sz = self.expect_integer(d)?;
                    let vn = self.varnode(d)?;
                    d.trace("pcode.addressOf/sz:export");
                    Ok(d.pcode_address_of(vn, sz))
                } else {
                    let vn = self.varnode(d)?;
                    d.trace("pcode.addressOf/0:export");
                    Ok(d.pcode_address_of(vn, 0))
                }
            }
            TokenKind::Integer => {
                let v = self.expect_integer(d)?;
                self.expect(d, TokenKind::Char1(b':'))?;
                let sz = self.expect_integer(d)?;
                d.trace("exportvn/INTEGER:INTEGER");
                Ok(d.exportvarnode_integer_colon(v, sz))
            }
            other => Err(KunaError::parse(format!(
                "SLEIGH: unexpected token in export varnode: {other:?}"
            ))),
        }
    }

    /// `label: '<' LABELSYM '>' | '<' STRING '>'` (slghparse.y:488-490).
    fn label(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u32> {
        self.expect(d, TokenKind::Char1(b'<'))?;
        let id = match self.peek(d)? {
            TokenKind::LabelSym => {
                let sym = self.expect_symbol(d, TokenKind::LabelSym)?;
                d.trace("label/sym");
                d.label_sym(sym)
            }
            _ => {
                let name = self.expect_string(d)?;
                d.trace("pcode.defineLabel");
                d.pcode_define_label(&name)
            }
        };
        self.expect(d, TokenKind::Char1(b'>'))?;
        Ok(id)
    }

    // -----------------------------------------------------------------------
    // familysymbol / specificsymbol (slghparse.y:498-508)
    // -----------------------------------------------------------------------

    /// `familysymbol: VALUESYM | VALUEMAPSYM | CONTEXTSYM | NAMESYM | VARLISTSYM`
    /// (slghparse.y:498-503).
    fn familysymbol(&mut self, d: &mut dyn ParseDriver) -> KunaResult<SymId> {
        let tok = self.next(d)?;
        if !is_familysymbol(tok.kind) {
            return Err(KunaError::parse(format!(
                "SLEIGH: expected family symbol, got {:?}",
                tok.kind
            )));
        }
        Ok(self.sym_from_token(d, &tok))
    }

    /// `specificsymbol: VARSYM | SPECSYM | OPERANDSYM | JUMPSYM` (slghparse.y:504-508).
    fn specificsymbol(&mut self, d: &mut dyn ParseDriver) -> KunaResult<SymId> {
        let tok = self.next(d)?;
        if !is_specificsymbol(tok.kind) {
            return Err(KunaError::parse(format!(
                "SLEIGH: expected specific symbol, got {:?}",
                tok.kind
            )));
        }
        Ok(self.sym_from_token(d, &tok))
    }

    // -----------------------------------------------------------------------
    // list helpers (slghparse.y:509-582)
    // -----------------------------------------------------------------------

    /// `charstring: CHAR | charstring CHAR` (slghparse.y:509-511).
    fn charstring(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<u8>> {
        let mut s = Vec::new();
        while self.peek(d)? == TokenKind::Char {
            let tok = self.next(d)?;
            if let TokenValue::Char(c) = tok.value {
                s.push(c);
            }
        }
        Ok(s)
    }

    /// `intblist: '[' intbpart ']' | INTEGER | '-' INTEGER` (slghparse.y:512-524).
    fn intblist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<i64>> {
        if self.peek(d)? == TokenKind::Char1(b'[') {
            self.next(d)?;
            let mut v = Vec::new();
            loop {
                match self.peek(d)? {
                    TokenKind::Char1(b']') => { self.next(d)?; break; }
                    TokenKind::Char1(b'-') => {
                        self.next(d)?;
                        let n = self.expect_integer(d)?;
                        v.push(-(n as i64));
                    }
                    TokenKind::Integer => {
                        let n = self.expect_integer(d)?;
                        v.push(n as i64);
                    }
                    TokenKind::String => {
                        // `_` -> the 0xBADBEEF sentinel (slghparse.y:518).
                        let s = self.expect_string(d)?;
                        if s != b"_" {
                            d.report_error("Expecting integer");
                            return Err(KunaError::parse("intbpart non-_".to_string()));
                        }
                        v.push(0xBADBEEF);
                    }
                    other => return Err(KunaError::parse(format!("SLEIGH: bad intblist {other:?}"))),
                }
            }
            Ok(v)
        } else if self.peek(d)? == TokenKind::Char1(b'-') {
            self.next(d)?;
            let n = self.expect_integer(d)?;
            Ok(vec![-(n as i64)])
        } else {
            let n = self.expect_integer(d)?;
            Ok(vec![n as i64])
        }
    }

    /// `stringlist: '[' stringpart ']' | STRING` (slghparse.y:525-531).
    fn stringlist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<Vec<u8>>> {
        if self.peek(d)? == TokenKind::Char1(b'[') {
            self.next(d)?;
            let mut v = Vec::new();
            loop {
                match self.peek(d)? {
                    TokenKind::Char1(b']') => { self.next(d)?; break; }
                    TokenKind::String => v.push(self.expect_string(d)?),
                    other => {
                        if is_anysymbol(other) {
                            let tok = self.next(d)?;
                            let name = self.sym_name(&tok);
                            d.report_error(&format!("{}: redefined", String::from_utf8_lossy(&name)));
                            return Err(KunaError::parse("stringpart redefined".to_string()));
                        }
                        return Err(KunaError::parse(format!("SLEIGH: bad stringlist {other:?}")));
                    }
                }
            }
            Ok(v)
        } else {
            Ok(vec![self.expect_string(d)?])
        }
    }

    /// `anystringlist: '[' anystringpart ']'` (slghparse.y:532-538).
    fn anystringlist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<Vec<u8>>> {
        self.expect(d, TokenKind::Char1(b'['))?;
        let mut v = Vec::new();
        loop {
            match self.peek(d)? {
                TokenKind::Char1(b']') => { self.next(d)?; break; }
                TokenKind::String => v.push(self.expect_string(d)?),
                k if is_anysymbol(k) => {
                    let tok = self.next(d)?;
                    v.push(self.sym_name(&tok));
                }
                other => return Err(KunaError::parse(format!("SLEIGH: bad anystringlist {other:?}"))),
            }
        }
        Ok(v)
    }

    /// `valuelist: '[' valuepart ']' | VALUESYM | CONTEXTSYM` (slghparse.y:539-548).
    fn valuelist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<SymId>> {
        if self.peek(d)? == TokenKind::Char1(b'[') {
            self.next(d)?;
            let mut v = Vec::new();
            loop {
                match self.peek(d)? {
                    TokenKind::Char1(b']') => { self.next(d)?; break; }
                    TokenKind::ValueSym => v.push(self.expect_symbol(d, TokenKind::ValueSym)?),
                    TokenKind::ContextSym => v.push(self.expect_symbol(d, TokenKind::ContextSym)?),
                    TokenKind::String => {
                        let s = self.expect_string(d)?;
                        d.report_error(&format!("{}: is not a value pattern", String::from_utf8_lossy(&s)));
                        return Err(KunaError::parse("valuepart non-value".to_string()));
                    }
                    other => return Err(KunaError::parse(format!("SLEIGH: bad valuelist {other:?}"))),
                }
            }
            Ok(v)
        } else {
            match self.peek(d)? {
                TokenKind::ValueSym => Ok(vec![self.expect_symbol(d, TokenKind::ValueSym)?]),
                TokenKind::ContextSym => Ok(vec![self.expect_symbol(d, TokenKind::ContextSym)?]),
                other => Err(KunaError::parse(format!("SLEIGH: bad valuelist head {other:?}"))),
            }
        }
    }

    /// `varlist: '[' varpart ']' | VARSYM` (slghparse.y:549-558).
    fn varlist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<SymId>> {
        if self.peek(d)? == TokenKind::Char1(b'[') {
            self.next(d)?;
            let mut v = Vec::new();
            loop {
                match self.peek(d)? {
                    TokenKind::Char1(b']') => { self.next(d)?; break; }
                    TokenKind::VarSym => v.push(self.expect_symbol(d, TokenKind::VarSym)?),
                    TokenKind::String => {
                        let s = self.expect_string(d)?;
                        if s != b"_" {
                            d.report_error(&format!("{}: is not a varnode symbol", String::from_utf8_lossy(&s)));
                            return Err(KunaError::parse("varpart non-_".to_string()));
                        }
                        // `_` -> the null-varnode sentinel (slghparse.y:553).
                        v.push(SymId::MAX);
                    }
                    other => return Err(KunaError::parse(format!("SLEIGH: bad varlist {other:?}"))),
                }
            }
            Ok(v)
        } else {
            Ok(vec![self.expect_symbol(d, TokenKind::VarSym)?])
        }
    }

    /// `paramlist: /*EMPTY*/ | expr | paramlist ',' expr` (slghparse.y:559-562).
    fn paramlist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<u32>> {
        if self.peek(d)? == TokenKind::Char1(b')') {
            return Ok(Vec::new());
        }
        let mut v = vec![self.expr(d)?];
        while self.peek(d)? == TokenKind::Char1(b',') {
            self.next(d)?;
            v.push(self.expr(d)?);
        }
        Ok(v)
    }

    /// `oplist: /*EMPTY*/ | STRING | oplist ',' STRING` (slghparse.y:563-566).
    fn oplist(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<Vec<u8>>> {
        if self.peek(d)? == TokenKind::Char1(b')') {
            return Ok(Vec::new());
        }
        let mut v = vec![self.expect_string(d)?];
        while self.peek(d)? == TokenKind::Char1(b',') {
            self.next(d)?;
            v.push(self.expect_string(d)?);
        }
        Ok(v)
    }

    // -----------------------------------------------------------------------
    // token-value extraction helpers
    // -----------------------------------------------------------------------

    fn expect_integer(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u64> {
        let tok = self.expect(d, TokenKind::Integer)?;
        match tok.value {
            TokenValue::Integer(i) => Ok(i),
            _ => Err(KunaError::parse("SLEIGH: INTEGER without value".to_string())),
        }
    }

    /// An INTEGER value (the SUBPIECE arg comes through `integervarnode`'s INTEGER,
    /// slghparse.y:453).
    fn expect_integer_value(&mut self, d: &mut dyn ParseDriver) -> KunaResult<u64> {
        self.expect_integer(d)
    }

    fn expect_intb(&mut self, d: &mut dyn ParseDriver) -> KunaResult<i64> {
        let tok = self.expect(d, TokenKind::Intb)?;
        match tok.value {
            TokenValue::Intb(b) => Ok(b),
            _ => Err(KunaError::parse("SLEIGH: INTB without value".to_string())),
        }
    }

    fn expect_string(&mut self, d: &mut dyn ParseDriver) -> KunaResult<Vec<u8>> {
        let tok = self.expect(d, TokenKind::String)?;
        token_str(&tok)
    }

    /// Consume a token of the given `*SYM` kind and resolve it to a `SymId`.
    fn expect_symbol(&mut self, d: &mut dyn ParseDriver, kind: TokenKind) -> KunaResult<SymId> {
        let tok = self.expect(d, kind)?;
        Ok(self.sym_from_token(d, &tok))
    }

    /// Resolve a `*SYM` token's attached name to a `SymId` via the driver.
    fn sym_from_token(&mut self, d: &mut dyn ParseDriver, tok: &Token) -> SymId {
        let name = self.sym_name(tok);
        d.resolve_symbol(&name)
    }

    /// The identifier name a `*SYM` (or any-symbol) token carries.
    fn sym_name(&self, tok: &Token) -> Vec<u8> {
        match &tok.value {
            TokenValue::SymbolName(n) => n.clone(),
            TokenValue::Str(n) => n.clone(),
            _ => Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// free helpers
// ---------------------------------------------------------------------------

/// Extract a `STRING`/`SYMBOLSTRING` token's bytes.
fn token_str(tok: &Token) -> KunaResult<Vec<u8>> {
    match &tok.value {
        TokenValue::Str(s) => Ok(s.clone()),
        TokenValue::SymbolName(s) => Ok(s.clone()),
        _ => Err(KunaError::parse("SLEIGH: string token without value".to_string())),
    }
}

/// True for the `familysymbol` alternatives (slghparse.y:498-503).
fn is_familysymbol(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::ValueSym
            | TokenKind::ValueMapSym
            | TokenKind::ContextSym
            | TokenKind::NameSym
            | TokenKind::VarListSym
    )
}

/// True for the `specificsymbol` alternatives (slghparse.y:504-508).
fn is_specificsymbol(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::VarSym | TokenKind::SpecSym | TokenKind::OperandSym | TokenKind::JumpSym
    )
}

/// True for the `anysymbol` alternatives (slghparse.y:567-582).
fn is_anysymbol(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::SpaceSym
            | TokenKind::SectionSym
            | TokenKind::TokenSym
            | TokenKind::UseropSym
            | TokenKind::MacroSym
            | TokenKind::SubtableSym
            | TokenKind::ValueSym
            | TokenKind::ValueMapSym
            | TokenKind::ContextSym
            | TokenKind::NameSym
            | TokenKind::VarSym
            | TokenKind::VarListSym
            | TokenKind::OperandSym
            | TokenKind::JumpSym
            | TokenKind::BitSym
    )
}

/// The operand-swapping comparison forms (slghparse.y:400,402,404,406,427,429):
/// the C++ swaps `$1`/`$3` for `>`/`>=`/`s>`/`s>=`/`f>`/`f>=`.  Returns the trace
/// token (with the underlying op name) and the (a,b) operand order to pass.
fn swap_compare(optok: TokenKind, lhs: u32, rhs: u32) -> (&'static str, u32, u32) {
    match optok {
        TokenKind::Char1(b'>') => ("pcode.createOp/CPUI_INT_LESS", rhs, lhs),
        TokenKind::OpGreatEqual => ("pcode.createOp/CPUI_INT_LESSEQUAL", rhs, lhs),
        TokenKind::OpSgreatEqual => ("pcode.createOp/CPUI_INT_SLESSEQUAL", rhs, lhs),
        TokenKind::OpSgreat => ("pcode.createOp/CPUI_INT_SLESS", rhs, lhs),
        TokenKind::OpFgreat => ("pcode.createOp/CPUI_FLOAT_LESS", rhs, lhs),
        TokenKind::OpFgreatEqual => ("pcode.createOp/CPUI_FLOAT_LESSEQUAL", rhs, lhs),
        _ => ("pcode.createOp/UNKNOWN", lhs, rhs),
    }
}
