//! Port of `decompiler/cpp/slghsymbol.{hh,cc}` (item `w2-sleigh-symbol`) —
//! the SLEIGH symbol system as decoded from a compiled `.sla` file.
//!
//! ## What is ported
//!
//! - The `SleighSymbol` hierarchy as [`SleighSymbol`] (name/id/scope header)
//!   wrapping a [`SymbolKind`] enum mirroring the C++ subclasses:
//!   Space/Token/UserOp/Epsilon/Value/ValueMap/Name/Varnode/Context/
//!   VarnodeList/Operand/Start/End/Next2/FlowDest/FlowRef/Subtable.
//! - [`Constructor`] with print pieces (`print`/`print_mnemonic`/
//!   `print_body`, the flow-through rule) and context commands.
//! - [`DecisionNode`] runtime dispatch (`resolve`) and its decode.
//! - [`SymbolTable`] with [`SymbolScope`] chains and the two-pass `.sla`
//!   restore: header shells first (`decode_symbol_header`), then the
//!   per-kind content decode in stream order.
//! - The `encode`/`encodeHeader` writers for every decodable kind (used by
//!   the round-trip tests; the writer side of `.sla` otherwise belongs to
//!   the unported compiler, LOSS-001).
//!
//! ## What is NOT ported (SLEIGH compiler side, LOSS-001)
//!
//! Everything reachable only from `slgh_compile`: `SymbolTable::purge`/
//! `renumber`/`replaceSymbol`, `Constructor::buildPattern`/`orderOperands`/
//! `addEquation`/`setMainSection`/`setNamedSection`/
//! `markSubtableOperands`/`isRecursive`/`setError`/`isError`/
//! `collectLocalExports` (the pure print-piece builders `addOperand`/
//! `addInvisibleOperand`/`addSyntax`/`removeTrailingSpace`/`addContext`
//! ARE ported — tests build constructors with them),
//! `SubtableSymbol::buildPattern`/`buildDecisionTree`/
//! `collectLocalValues` (+ the `beingbuilt`/`errors` flags),
//! `DecisionNode::split`/`orderPatterns`/`chooseOptimalField`/`getScore`/
//! `getNumFixed`/`getMaximumLength`/`consistentValues`/`addConstructorPair`,
//! `DecisionProperties`, `OperandSymbol::setCodeAddress`/
//! `setOffsetIrrelevant`/mark handling (`defineOperand` IS ported),
//! and the Macro/Label/Section/Bitrange
//! symbol classes ([`SymbolType`] keeps their discriminants so `getType`
//! comparisons stay transcribable).  The `TokenPattern *pattern` and
//! `PatternEquation *pateq` members are compiler state and are dropped.
//! The `getVarnode()` virtuals (returning `VarnodeTpl`, semantics.hh) are
//! deferred with the semantics wave: their only callers are in the unported
//! pcode compiler.
//!
//! ## Pointer-web representation
//!
//! C++ resolves symbol ids to raw pointers at decode time
//! (`trans->findSymbol(id)`, unchecked casts).  The port stores the **ids**
//! (`u32`, C++ `uintm`) and resolves through the owning [`SymbolTable`] at
//! use time; methods that C++ ran on a bare pointer take a `&SymbolTable`
//! parameter.  Where a C++ unchecked downcast would be UB on a
//! wrongly-typed id, the port returns a `KunaError` (documented at each
//! site; same policy as the `OperandValueResolver` seam contract in
//! [`crate::slghpatexpress`]).  `DecisionNode::parent` is dropped: its only
//! consumer is the compiler-side `split()`.
//!
//! ## Seams
//!
//! - [`SymbolWalker`] extends the [`PatternExpressionContext`] seam from
//!   [`crate::slghpatexpress`] with exactly the additional `ParserWalker`
//!   surface slghsymbol.cc touches (operand push/pop, current constructor,
//!   fixed handles, spaces, flow addresses, bit reads).  Implemented by the
//!   sleigh decode-engine wave; tests implement it synthetically.
//! - [`SymbolWalkerChange`] mirrors `ParserWalkerChange` for the
//!   context-command `apply` path (`setContextWord`/`addCommit`).
//! - [`SleighBaseTrans`] stands in for the `SleighBase *trans` decode
//!   argument, reduced to what the symbol decode actually pulls from it
//!   beyond id storage: the constant space and `ConstructTpl`
//!   (semantics.hh) decode/encode, which belongs to the unported semantics
//!   wave.  Constructors hold opaque [`ConstructTplHandle`]s.
//! - [`SymbolTable`] itself implements
//!   [`crate::slghpatexpress::OperandValueResolver`], closing the loop the
//!   pattern wave left open (`OperandValue::decode` validation), and
//!   provides `OperandValue::isConstructorRelative`/`getName` (C++
//!   slghpatexpress.cc:800-812) as
//!   [`SymbolTable::operand_value_is_constructor_relative`] /
//!   [`SymbolTable::operand_value_name`].
//!
//! Names and print pieces are byte strings (marshal convention); printed
//! output goes to a `String` with names converted lossily (`.sla`
//! identifiers are ASCII).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::{FixedHandle, Token};
use crate::slghpatexpress::{
    ConstantValue, ContextField, EndInstructionValue, EqId, EquationArena, Next2InstructionValue,
    OperandResolve, OperandResolveSink, OperandValue, OperandValueResolver, PatternExpression,
    PatternExpressionContext, PatternValue, StartInstructionValue, TokenPattern,
};
use crate::slghpattern::{DisjointPattern, Pattern};

/// `.sla`-format ElementIds/AttributeIds used by the symbol system
/// (slaformat.cc, `FORMAT_SCOPE`).  Extends the set already defined by the
/// pattern wave in [`crate::slghpattern::sla`], which is re-exported here so
/// symbol code uses a single `sla::` namespace.
pub mod sla {
    use kuna_base::marshal::{AttributeId, ElementId};

    pub use crate::slghpattern::sla::*;

    pub const ATTRIB_ID: AttributeId = AttributeId::new("id", 3);
    pub const ATTRIB_SPACE: AttributeId = AttributeId::new("space", 4);
    pub const ATTRIB_CODE: AttributeId = AttributeId::new("code", 7);
    pub const ATTRIB_PIECE: AttributeId = AttributeId::new("piece", 11);
    pub const ATTRIB_NAME: AttributeId = AttributeId::new("name", 12);
    pub const ATTRIB_SCOPE: AttributeId = AttributeId::new("scope", 13);
    pub const ATTRIB_SIZE: AttributeId = AttributeId::new("size", 15);
    pub const ATTRIB_MINLEN: AttributeId = AttributeId::new("minlen", 18);
    pub const ATTRIB_BASE: AttributeId = AttributeId::new("base", 19);
    pub const ATTRIB_NUMBER: AttributeId = AttributeId::new("number", 20);
    pub const ATTRIB_CONTEXT: AttributeId = AttributeId::new("context", 21);
    pub const ATTRIB_PARENT: AttributeId = AttributeId::new("parent", 22);
    pub const ATTRIB_SUBSYM: AttributeId = AttributeId::new("subsym", 23);
    pub const ATTRIB_LINE: AttributeId = AttributeId::new("line", 24);
    pub const ATTRIB_SOURCE: AttributeId = AttributeId::new("source", 25);
    pub const ATTRIB_LENGTH: AttributeId = AttributeId::new("length", 26);
    pub const ATTRIB_FIRST: AttributeId = AttributeId::new("first", 27);
    pub const ATTRIB_SCOPESIZE: AttributeId = AttributeId::new("scopesize", 45);
    pub const ATTRIB_SYMBOLSIZE: AttributeId = AttributeId::new("symbolsize", 46);
    pub const ATTRIB_VARNODE: AttributeId = AttributeId::new("varnode", 47);
    pub const ATTRIB_LOW: AttributeId = AttributeId::new("low", 48);
    pub const ATTRIB_HIGH: AttributeId = AttributeId::new("high", 49);
    pub const ATTRIB_FLOW: AttributeId = AttributeId::new("flow", 50);
    pub const ATTRIB_I: AttributeId = AttributeId::new("i", 52);
    pub const ATTRIB_NUMCT: AttributeId = AttributeId::new("numct", 53);

    pub const ELEM_PRINT: ElementId = ElementId::new("print", 8);
    pub const ELEM_PAIR: ElementId = ElementId::new("pair", 9);
    pub const ELEM_NULL: ElementId = ElementId::new("null", 11);
    pub const ELEM_OPERAND_SYM: ElementId = ElementId::new("operand_sym", 13);
    pub const ELEM_OPERAND_SYM_HEAD: ElementId = ElementId::new("operand_sym_head", 14);
    pub const ELEM_OPER: ElementId = ElementId::new("oper", 15);
    pub const ELEM_DECISION: ElementId = ElementId::new("decision", 16);
    pub const ELEM_OPPRINT: ElementId = ElementId::new("opprint", 17);
    pub const ELEM_CONSTRUCTOR: ElementId = ElementId::new("constructor", 20);
    pub const ELEM_SCOPE: ElementId = ElementId::new("scope", 22);
    pub const ELEM_VARNODE_SYM: ElementId = ElementId::new("varnode_sym", 23);
    pub const ELEM_VARNODE_SYM_HEAD: ElementId = ElementId::new("varnode_sym_head", 24);
    pub const ELEM_USEROP: ElementId = ElementId::new("userop", 25);
    pub const ELEM_USEROP_HEAD: ElementId = ElementId::new("userop_head", 26);
    pub const ELEM_VAR: ElementId = ElementId::new("var", 28);
    pub const ELEM_CONTEXT_OP: ElementId = ElementId::new("context_op", 32);
    pub const ELEM_SYMBOL_TABLE: ElementId = ElementId::new("symbol_table", 38);
    pub const ELEM_VALUE_SYM: ElementId = ElementId::new("value_sym", 39);
    pub const ELEM_VALUE_SYM_HEAD: ElementId = ElementId::new("value_sym_head", 40);
    pub const ELEM_CONTEXT_SYM: ElementId = ElementId::new("context_sym", 41);
    pub const ELEM_CONTEXT_SYM_HEAD: ElementId = ElementId::new("context_sym_head", 42);
    pub const ELEM_END_SYM: ElementId = ElementId::new("end_sym", 43);
    pub const ELEM_END_SYM_HEAD: ElementId = ElementId::new("end_sym_head", 44);
    pub const ELEM_EPSILON_SYM: ElementId = ElementId::new("epsilon_sym", 62);
    pub const ELEM_EPSILON_SYM_HEAD: ElementId = ElementId::new("epsilon_sym_head", 63);
    pub const ELEM_NAME_SYM: ElementId = ElementId::new("name_sym", 64);
    pub const ELEM_NAME_SYM_HEAD: ElementId = ElementId::new("name_sym_head", 65);
    pub const ELEM_NAMETAB: ElementId = ElementId::new("nametab", 66);
    pub const ELEM_NEXT2_SYM: ElementId = ElementId::new("next2_sym", 67);
    pub const ELEM_NEXT2_SYM_HEAD: ElementId = ElementId::new("next2_sym_head", 68);
    pub const ELEM_START_SYM: ElementId = ElementId::new("start_sym", 69);
    pub const ELEM_START_SYM_HEAD: ElementId = ElementId::new("start_sym_head", 70);
    pub const ELEM_SUBTABLE_SYM: ElementId = ElementId::new("subtable_sym", 71);
    pub const ELEM_SUBTABLE_SYM_HEAD: ElementId = ElementId::new("subtable_sym_head", 72);
    pub const ELEM_VALUEMAP_SYM: ElementId = ElementId::new("valuemap_sym", 73);
    pub const ELEM_VALUEMAP_SYM_HEAD: ElementId = ElementId::new("valuemap_sym_head", 74);
    pub const ELEM_VALUETAB: ElementId = ElementId::new("valuetab", 75);
    pub const ELEM_VARLIST_SYM: ElementId = ElementId::new("varlist_sym", 76);
    pub const ELEM_VARLIST_SYM_HEAD: ElementId = ElementId::new("varlist_sym_head", 77);
    pub const ELEM_COMMIT: ElementId = ElementId::new("commit", 79);
}

// ---------------------------------------------------------------------------
// Seams
// ---------------------------------------------------------------------------

/// Identifies a [`Constructor`] without a pointer: the owning
/// `SubtableSymbol` id plus the constructor id within that table (the same
/// currency [`OperandValue`] uses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructorRef {
    /// Symbol id of the owning `SubtableSymbol`.
    pub table_id: u32,
    /// Constructor id within the subtable (C++ `Constructor::getId`).
    pub ct_id: u32,
}

/// The additional `ParserWalker` surface (context.hh/sleigh.hh) used by
/// slghsymbol.cc beyond the [`PatternExpressionContext`] seam: operand
/// traversal for printing, fixed-handle lookup, address spaces, flow
/// addresses, and the raw bit reads `DecisionNode::resolve` dispatches on.
/// Implemented by the sleigh decode-engine wave.
pub trait SymbolWalker: PatternExpressionContext {
    /// C++ `ParserWalker::pushOperand(int4 i)`.
    fn push_operand(&mut self, i: i32) -> KunaResult<()>;
    /// C++ `ParserWalker::popOperand()`.
    fn pop_operand(&mut self) -> KunaResult<()>;
    /// C++ `ParserWalker::getConstructor()`: the constructor resolved at the
    /// current point, identified by (table id, constructor id).
    fn get_constructor(&self) -> KunaResult<ConstructorRef>;
    /// C++ `ParserWalker::getFixedHandle(int4 i)` (the resolved handle of
    /// child operand `i`; returned by value instead of const reference).
    fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle>;
    /// C++ `ParserWalker::getConstSpace()`.
    fn get_const_space(&self) -> Rc<AddrSpace>;
    /// C++ `ParserWalker::getCurSpace()`.
    fn get_cur_space(&self) -> Rc<AddrSpace>;
    /// C++ `ParserWalker::getDestAddr()` (`ParserContext` may throw).
    fn get_dest_addr(&self) -> KunaResult<Address>;
    /// C++ `ParserWalker::getRefAddr()` (`ParserContext` may throw).
    fn get_ref_addr(&self) -> KunaResult<Address>;
    /// C++ `ParserWalker::getInstructionBits(int4 startbit,int4 size)`.
    fn get_instruction_bits(&self, startbit: i32, size: i32) -> KunaResult<u32>;
    /// C++ `ParserWalker::getContextBits(int4 startbit,int4 size)`.
    fn get_context_bits(&self, startbit: i32, size: i32) -> KunaResult<u32>;
}

/// Mirror of C++ `ParserWalkerChange` for the context-command apply path
/// (`ContextOp::apply` / `ContextCommit::apply`).
pub trait SymbolWalkerChange: SymbolWalker {
    /// C++ `walker.getParserContext()->setContextWord(i,val,mask)`.
    fn set_context_word(&mut self, i: i32, val: u32, mask: u32);
    /// C++ `walker.getParserContext()->addCommit(sym,num,mask,flow,point)`:
    /// `sym` is passed as its symbol id; the implementor supplies the
    /// current point (`walker.getPoint()`) itself.
    fn add_commit(&mut self, symbol_id: u32, num: i32, mask: u32, flow: bool);
}

/// Opaque handle to a `ConstructTpl` decoded through the
/// [`SleighBaseTrans`] seam (an index into the implementor's template
/// storage).
pub type ConstructTplHandle = usize;

/// Seam standing in for the `SleighBase *trans` argument of the symbol
/// decode/encode methods, reduced to what slghsymbol.cc pulls from it
/// beyond symbol-id storage: `trans->getConstantSpace()` and the
/// `ConstructTpl` (semantics.hh) decode/encode, which belongs to the
/// unported semantics wave.  Symbol-id resolution (`trans->findSymbol`)
/// stays inside [`SymbolTable`].
pub trait SleighBaseTrans {
    /// C++ `trans->getConstantSpace()`.
    fn get_constant_space(&self) -> Rc<AddrSpace>;
    /// C++ `ConstructTpl().decode(decoder)` inside `Constructor::decode`:
    /// the implementor decodes one `ConstructTpl` element from the stream,
    /// stores it, and returns `(section_id, handle)` where `section_id < 0`
    /// means the main section.
    fn decode_construct_tpl(
        &mut self,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<(i32, ConstructTplHandle)>;
    /// C++ `templ->encode(encoder, section_id)` inside `Constructor::encode`
    /// (`section_id == -1` for the main section).
    fn encode_construct_tpl(
        &self,
        handle: ConstructTplHandle,
        section_id: i32,
        encoder: &mut dyn Encoder,
    ) -> KunaResult<()>;
}

// ---------------------------------------------------------------------------
// SymbolType + small helpers
// ---------------------------------------------------------------------------

/// C++ `SleighSymbol::symbol_type`.  All discriminants are kept, including
/// the compiler-only classes that have no ported [`SymbolKind`] variant
/// (Macro/Section/Bitrange/Label/Dummy), so `getType()` comparisons
/// transcribe one-to-one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// C++ `space_symbol`
    Space,
    /// C++ `token_symbol`
    Token,
    /// C++ `userop_symbol`
    UserOp,
    /// C++ `value_symbol`
    Value,
    /// C++ `valuemap_symbol`
    ValueMap,
    /// C++ `name_symbol`
    Name,
    /// C++ `varnode_symbol`
    Varnode,
    /// C++ `varnodelist_symbol`
    VarnodeList,
    /// C++ `operand_symbol`
    Operand,
    /// C++ `start_symbol`
    Start,
    /// C++ `end_symbol`
    End,
    /// C++ `next2_symbol`
    Next2,
    /// C++ `subtable_symbol`
    Subtable,
    /// C++ `macro_symbol` (class not ported, LOSS-001)
    Macro,
    /// C++ `section_symbol` (class not ported, LOSS-001)
    Section,
    /// C++ `bitrange_symbol` (class not ported, LOSS-001)
    Bitrange,
    /// C++ `context_symbol`
    Context,
    /// C++ `epsilon_symbol`
    Epsilon,
    /// C++ `label_symbol` (class not ported, LOSS-001)
    Label,
    /// C++ `flowdest_symbol`
    FlowDest,
    /// C++ `flowref_symbol`
    FlowRef,
    /// C++ `dummy_symbol`
    Dummy,
}

/// Lossy text rendering of a byte-string symbol name for error messages and
/// printed output (`.sla` identifiers are ASCII in practice).
fn name_text(name: &[u8]) -> String {
    String::from_utf8_lossy(name).into_owned()
}

/// (WS4b renumber) remap the `OperandValue` subtable-id inside a `PatternValue`,
/// if any (the table-driven family symbols carry no subtable id; an
/// `OperandValue` does).
fn remap_patval_table(pv: Option<&mut PatternValue>, map: &impl Fn(u32) -> u32) {
    if let Some(PatternValue::OperandValue(ov)) = pv {
        ov.set_table_id(map(ov.table_id()));
    }
}

/// C++ `s << "0x" << hex << val` / `s << "-0x" << hex << -val` idiom shared
/// by the `print` methods (ValueSymbol, ValueMapSymbol, OperandSymbol).
fn push_hex_intb(s: &mut String, val: i64) {
    if val >= 0 {
        s.push_str(&format!("0x{val:x}"));
    } else {
        // C++ `-val`: UB on i64::MIN there, wrapping here; ostream then
        // prints the (possibly still negative) intb as its unsigned
        // representation, which is what {:x} on i64 produces.
        s.push_str(&format!("-0x{:x}", val.wrapping_neg()));
    }
}

/// C++ `s << "0x" << hex << (intb)offset` idiom of the address-printing
/// symbols (Start/End/Next2/FlowDest/FlowRef): ostream prints the signed
/// value as its unsigned 64-bit representation, i.e. the raw offset.
fn push_hex_offset(s: &mut String, offset: u64) {
    s.push_str(&format!("0x{offset:x}"));
}

/// C++ `(PatternValue *)PatternExpression::decodeExpression(decoder,trans)`:
/// the cast is unchecked in C++ (UB on a non-value expression); the port
/// reports a decoder error instead.
fn decode_pattern_value(
    decoder: &mut dyn Decoder,
    trans: &dyn OperandValueResolver,
) -> KunaResult<PatternValue> {
    match PatternExpression::decode_expression(decoder, trans)? {
        PatternExpression::Value(v) => Ok(v),
        _ => Err(KunaError::decoder(
            "Expecting pattern value (C++ casts unchecked)",
        )),
    }
}

/// C++ static `calc_maskword(int4 sbit,int4 ebit,int4 &num,int4 &shift,
/// uintm &mask)` (slghsymbol.cc): locate a context bit-field within one
/// machine int.  Returns `(num, shift, mask)`.
fn calc_maskword(mut sbit: i32, mut ebit: i32) -> KunaResult<(i32, i32, u32)> {
    let num = sbit / 32; // 8*sizeof(uintm)
    if num != ebit / 32 {
        return Err(KunaError::sleigh(
            "Context field not contained within one machine int",
        ));
    }
    sbit -= num * 32;
    ebit -= num * 32;
    let shift = 32 - ebit - 1;
    // uintm >> (sbit+shift), << shift: counts are in [0,31] for valid
    // fields; an invalid field is UB in C++ and resolves x86-masked here.
    // i32 -> u32 shift-count casts: non-negative for valid fields.
    let mut mask = (!0u32).wshr((sbit + shift) as u32);
    mask = mask.wshl(shift as u32);
    Ok((num, shift, mask))
}

// ---------------------------------------------------------------------------
// Symbol kind payloads (the C++ subclass data)
// ---------------------------------------------------------------------------

/// C++ `SpaceSymbol`: wraps an address space (never decoded from `.sla`;
/// built programmatically by the compiler/console sides).
#[derive(Debug, Clone)]
pub struct SpaceSymbol {
    space: Rc<AddrSpace>,
}

impl SpaceSymbol {
    /// C++ `SpaceSymbol::getSpace`.
    pub fn get_space(&self) -> &Rc<AddrSpace> {
        &self.space
    }
}

/// C++ `TokenSymbol`: wraps a [`Token`] (never decoded from `.sla`).
#[derive(Debug, Clone)]
pub struct TokenSymbol {
    tok: Token,
}

impl TokenSymbol {
    /// C++ `TokenSymbol::getToken`.
    pub fn get_token(&self) -> &Token {
        &self.tok
    }
}

/// C++ `UserOpSymbol`: a user-defined pcode-op.
#[derive(Debug, Clone, Default)]
pub struct UserOpSymbol {
    index: u32,
}

impl UserOpSymbol {
    /// C++ `UserOpSymbol::setIndex`.
    pub fn set_index(&mut self, ind: u32) {
        self.index = ind;
    }

    /// C++ `UserOpSymbol::getIndex`.
    pub fn get_index(&self) -> u32 {
        self.index
    }
}

/// C++ `EpsilonSymbol`: another name for the zero pattern/value.  The
/// `PatternlessSymbol` base's `ConstantValue(0)` pattern expression is
/// stateless and constructed on demand by
/// [`SleighSymbol::get_pattern_expression`].
#[derive(Debug, Clone, Default)]
pub struct EpsilonSymbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `ValueSymbol`: a value computed from a pattern (also the base data
/// of ValueMap/Name/Context/VarnodeList symbols, embedded by composition).
#[derive(Debug, Clone, Default)]
pub struct ValueSymbol {
    /// C++ `PatternValue *patval` (null until decoded).
    patval: Option<PatternValue>,
}

impl ValueSymbol {
    /// C++ `ValueSymbol::getPatternValue` (None mirrors the C++ null of an
    /// undecoded shell).
    pub fn get_pattern_value(&self) -> Option<&PatternValue> {
        self.patval.as_ref()
    }

    /// The C++ null-`patval` dereference sites (getValue paths) as an error.
    fn patval(&self) -> KunaResult<&PatternValue> {
        self.patval
            .as_ref()
            .ok_or_else(|| KunaError::sleigh("value symbol has no pattern value (not decoded)"))
    }
}

/// C++ `ValueMapSymbol`: maps the pattern value through a lookup table.
#[derive(Debug, Clone, Default)]
pub struct ValueMapSymbol {
    value: ValueSymbol,
    valuetable: Vec<i64>,
    tableisfilled: bool,
}

impl ValueMapSymbol {
    /// The decoded value table.
    pub fn get_value_table(&self) -> &[i64] {
        &self.valuetable
    }

    /// C++ `ValueMapSymbol::checkTableFill`.
    fn check_table_fill(&mut self) -> KunaResult<()> {
        let min = self.value.patval()?.min_value()?;
        let max = self.value.patval()?.max_value()?;
        // C++ `max < valuetable.size()`: intb vs size_t comparison — max
        // converts to 64-bit unsigned (negative max becomes huge => false).
        self.tableisfilled = (min >= 0) && ((max as u64) < self.valuetable.len() as u64);
        for i in 0..self.valuetable.len() {
            if self.valuetable[i] == 0xBADBEEF {
                self.tableisfilled = false;
            }
        }
        Ok(())
    }

    /// C++ `ValueMapSymbol::resolve` (always returns null Constructor).
    fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<()> {
        if !self.tableisfilled {
            let pw: &dyn PatternExpressionContext = walker;
            let ind = self.value.patval()?.get_value(pw)?;
            // C++ `(ind >= valuetable.size())||(ind<0)||...`: the first is
            // an intb vs size_t mixed comparison (ind converts to u64).
            if (ind as u64) >= self.valuetable.len() as u64
                || ind < 0
                || self.valuetable[ind as usize] == 0xBADBEEF
            {
                let mut s = String::new();
                s.push(walker.get_addr().get_shortcut());
                walker.get_addr().print_raw(&mut s)?;
                s.push_str(": No corresponding entry in valuetable");
                return Err(KunaError::bad_data(s));
            }
        }
        Ok(())
    }
}

/// C++ `NameSymbol`: maps the pattern value through a name table.
#[derive(Debug, Clone, Default)]
pub struct NameSymbol {
    value: ValueSymbol,
    nametable: Vec<Vec<u8>>,
    tableisfilled: bool,
}

impl NameSymbol {
    /// The decoded name table (a 1-byte TAB entry marks an illegal index).
    pub fn get_name_table(&self) -> &[Vec<u8>] {
        &self.nametable
    }

    /// C++ `NameSymbol::checkTableFill`.
    fn check_table_fill(&mut self) -> KunaResult<()> {
        let min = self.value.patval()?.min_value()?;
        let max = self.value.patval()?.max_value()?;
        // intb vs size_t mixed comparison as in ValueMapSymbol.
        self.tableisfilled = (min >= 0) && ((max as u64) < self.nametable.len() as u64);
        for i in 0..self.nametable.len() {
            if self.nametable[i] == b"_" || self.nametable[i] == b"\t" {
                self.nametable[i] = b"\t".to_vec(); // TAB indicates illegal index
                self.tableisfilled = false;
            }
        }
        Ok(())
    }

    /// C++ `NameSymbol::resolve` (always returns null Constructor).
    fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<()> {
        if !self.tableisfilled {
            let pw: &dyn PatternExpressionContext = walker;
            let ind = self.value.patval()?.get_value(pw)?;
            // C++ `(ind >= nametable.size())||(ind<0)||...`: intb vs size_t
            // mixed comparison first (ind converts to u64).
            if (ind as u64) >= self.nametable.len() as u64
                || ind < 0
                || (self.nametable[ind as usize].len() == 1
                    && self.nametable[ind as usize][0] == b'\t')
            {
                let mut s = String::new();
                s.push(walker.get_addr().get_shortcut());
                walker.get_addr().print_raw(&mut s)?;
                s.push_str(": No corresponding entry in nametable");
                return Err(KunaError::bad_data(s));
            }
        }
        Ok(())
    }
}

/// C++ `VarnodeSymbol`: a global varnode (e.g. a register).
#[derive(Debug, Clone, Default)]
pub struct VarnodeSymbol {
    fix: VarnodeData,
    /// Compiler-side flag (`markAsContext`); never set by decode.
    context_bits: bool,
}

impl VarnodeSymbol {
    /// C++ `VarnodeSymbol::markAsContext`.
    pub fn mark_as_context(&mut self) {
        self.context_bits = true;
    }

    /// C++ `VarnodeSymbol::getFixedVarnode`.
    pub fn get_fixed_varnode(&self) -> &VarnodeData {
        &self.fix
    }

    /// C++ `VarnodeSymbol::getSize`: `fix.size` (uint4 returned as int4).
    pub fn get_size(&self) -> i32 {
        self.fix.size as i32 // uint4 -> int4 implicit in C++
    }
}

/// C++ `ContextSymbol`: a value taken from the context bits of a varnode.
#[derive(Debug, Clone, Default)]
pub struct ContextSymbol {
    value: ValueSymbol,
    /// C++ `VarnodeSymbol *vn` as a symbol id (None for an undecoded shell).
    vn: Option<u32>,
    low: u32,
    high: u32,
    flow: bool,
}

impl ContextSymbol {
    /// C++ `ContextSymbol::getVarnode` (the backing varnode's symbol id).
    pub fn get_varnode(&self) -> Option<u32> {
        self.vn
    }

    /// C++ `ContextSymbol::getLow`.
    pub fn get_low(&self) -> u32 {
        self.low
    }

    /// C++ `ContextSymbol::getHigh`.
    pub fn get_high(&self) -> u32 {
        self.high
    }

    /// C++ `ContextSymbol::getFlow`.
    pub fn get_flow(&self) -> bool {
        self.flow
    }
}

/// C++ `VarnodeListSymbol`: selects one of a list of varnodes by pattern
/// value.
#[derive(Debug, Clone, Default)]
pub struct VarnodeListSymbol {
    value: ValueSymbol,
    /// C++ `vector<VarnodeSymbol *>` as symbol ids; `None` is the C++ null
    /// entry (illegal index).
    varnode_table: Vec<Option<u32>>,
    tableisfilled: bool,
}

impl VarnodeListSymbol {
    /// The decoded varnode table (symbol ids; `None` marks an illegal
    /// index).
    pub fn get_varnode_table(&self) -> &[Option<u32>] {
        &self.varnode_table
    }

    /// C++ `VarnodeListSymbol::checkTableFill`.
    fn check_table_fill(&mut self) -> KunaResult<()> {
        let min = self.value.patval()?.min_value()?;
        let max = self.value.patval()?.max_value()?;
        // intb vs size_t mixed comparison as in ValueMapSymbol.
        self.tableisfilled = (min >= 0) && ((max as u64) < self.varnode_table.len() as u64);
        for entry in &self.varnode_table {
            if entry.is_none() {
                self.tableisfilled = false;
            }
        }
        Ok(())
    }

    /// C++ `VarnodeListSymbol::resolve` (always returns null Constructor).
    fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<()> {
        if !self.tableisfilled {
            let pw: &dyn PatternExpressionContext = walker;
            let ind = self.value.patval()?.get_value(pw)?;
            // C++ `(ind<0)||(ind>=varnode_table.size())||(null entry)`: the
            // middle test is intb vs size_t (ind converts to u64), but a
            // negative ind is already caught by the first test.
            if ind < 0
                || (ind as u64) >= self.varnode_table.len() as u64
                || self.varnode_table[ind as usize].is_none()
            {
                let mut s = String::new();
                s.push(walker.get_addr().get_shortcut());
                walker.get_addr().print_raw(&mut s)?;
                s.push_str(": No corresponding entry in varnode list");
                return Err(KunaError::bad_data(s));
            }
        }
        Ok(())
    }

    /// C++ `VarnodeListSymbol::getSize` (`name` is the symbol name for the
    /// error message; the table resolves the varnode entries).
    fn get_size(&self, name: &[u8], table: &SymbolTable) -> KunaResult<i32> {
        // C++ loop returns at the first non-null entry (assume all are
        // same size)
        if let Some(id) = self.varnode_table.iter().flatten().next() {
            return Ok(table.varnode_symbol(*id)?.get_size());
        }
        Err(KunaError::sleigh(format!(
            "No register attached to: {}",
            name_text(name)
        )))
    }
}

/// C++ `OperandSymbol`: an operand of a constructor.
#[derive(Debug, Clone, Default)]
pub struct OperandSymbol {
    /// Relative offset (C++ `uint4 reloffset`).
    reloffset: u32,
    /// Base operand the offset is relative to (-1 = constructor start).
    offsetbase: i32,
    /// Minimum size of operand (within instruction tokens).
    minimumlength: i32,
    /// Handle index.
    hand: i32,
    /// C++ `OperandValue *localexp` (null until decoded).
    localexp: Option<OperandValue>,
    /// C++ `TripleSymbol *triple` (defining symbol) as a symbol id.
    triple: Option<u32>,
    /// C++ `PatternExpression *defexp` (OR defining expression).
    defexp: Option<PatternExpression>,
    flags: u32,
}

impl OperandSymbol {
    /// C++ `OperandSymbol::code_address` flag.
    pub const CODE_ADDRESS: u32 = 1;
    /// C++ `OperandSymbol::offset_irrel` flag.
    pub const OFFSET_IRREL: u32 = 2;
    /// C++ `OperandSymbol::variable_len` flag.
    pub const VARIABLE_LEN: u32 = 4;
    /// C++ `OperandSymbol::marked` flag.
    pub const MARKED: u32 = 8;

    /// C++ `OperandSymbol::getRelativeOffset`.
    pub fn get_relative_offset(&self) -> u32 {
        self.reloffset
    }

    /// C++ `OperandSymbol::getOffsetBase`.
    pub fn get_offset_base(&self) -> i32 {
        self.offsetbase
    }

    /// C++ `OperandSymbol::getMinimumLength`.
    pub fn get_minimum_length(&self) -> i32 {
        self.minimumlength
    }

    /// C++ `OperandSymbol::getDefiningExpression`.
    pub fn get_defining_expression(&self) -> Option<&PatternExpression> {
        self.defexp.as_ref()
    }

    /// C++ `OperandSymbol::getDefiningSymbol` (a TripleSymbol id).
    pub fn get_defining_symbol(&self) -> Option<u32> {
        self.triple
    }

    /// C++ `OperandSymbol::getIndex` (the handle index).
    pub fn get_index(&self) -> i32 {
        self.hand
    }

    /// C++ `OperandSymbol::isCodeAddress`.
    pub fn is_code_address(&self) -> bool {
        (self.flags & Self::CODE_ADDRESS) != 0
    }

    /// C++ `OperandSymbol::isOffsetIrrelevant`.
    pub fn is_offset_irrelevant(&self) -> bool {
        (self.flags & Self::OFFSET_IRREL) != 0
    }

    /// The local operand expression (C++ `getPatternExpression` returns it).
    pub fn get_local_expression(&self) -> Option<&OperandValue> {
        self.localexp.as_ref()
    }

    /// C++ `OperandSymbol::defineOperand(PatternExpression *pe)`.
    pub fn define_operand_expression(&mut self, pe: PatternExpression) -> KunaResult<()> {
        if self.defexp.is_some() || self.triple.is_some() {
            return Err(KunaError::sleigh("Redefining operand"));
        }
        self.defexp = Some(pe);
        Ok(())
    }

    /// C++ `OperandSymbol::defineOperand(TripleSymbol *tri)` (the defining
    /// symbol passed as its id).
    pub fn define_operand_symbol(&mut self, tri: u32) -> KunaResult<()> {
        if self.defexp.is_some() || self.triple.is_some() {
            return Err(KunaError::sleigh("Redefining operand"));
        }
        self.triple = Some(tri);
        Ok(())
    }

    // ---- build side (ws4a) ----

    /// C++ `OperandSymbol::isMarked`.
    pub fn is_marked(&self) -> bool {
        (self.flags & Self::MARKED) != 0
    }

    /// C++ `OperandSymbol::setMark`.
    pub fn set_mark(&mut self) {
        self.flags |= Self::MARKED;
    }

    /// C++ `OperandSymbol::clearMark`.
    pub fn clear_mark(&mut self) {
        self.flags &= !Self::MARKED;
    }

    /// C++ `OperandSymbol::isVariableLength`.
    pub fn is_variable_length(&self) -> bool {
        (self.flags & Self::VARIABLE_LEN) != 0
    }

    /// C++ `OperandSymbol::setVariableLength`.
    pub fn set_variable_length(&mut self) {
        self.flags |= Self::VARIABLE_LEN;
    }

    /// C++ direct member writes `sym->offsetbase = ...; sym->reloffset = ...`.
    pub fn set_offset(&mut self, offsetbase: i32, reloffset: u32) {
        self.offsetbase = offsetbase;
        self.reloffset = reloffset;
    }

    /// C++ direct member write `sym->offsetbase = ...`.
    pub fn set_offset_base(&mut self, offsetbase: i32) {
        self.offsetbase = offsetbase;
    }

    /// C++ direct member write `sym->minimumlength = ...`.
    pub fn set_minimum_length(&mut self, l: i32) {
        self.minimumlength = l;
    }

    /// C++ direct member write `newops[i]->hand = i`.
    pub fn set_hand(&mut self, hand: i32) {
        self.hand = hand;
    }

    /// C++ `newops[i]->localexp->changeIndex(i)`.
    pub fn change_local_index(&mut self, newind: i32) {
        if let Some(le) = self.localexp.as_mut() {
            le.change_index(newind);
        }
    }

    /// C++ `OperandSymbol::print`.
    fn print(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        walker.push_operand(self.get_index())?;
        if let Some(triple_id) = self.triple {
            let tsym = table.symbol(triple_id)?;
            if tsym.get_type() == SymbolType::Subtable {
                let ctref = walker.get_constructor()?;
                table.get_constructor(ctref)?.print(s, walker, table)?;
            } else {
                tsym.print(s, walker, table)?;
            }
        } else {
            // C++ dereferences defexp unconditionally here (null would be UB)
            let defexp = self
                .defexp
                .as_ref()
                .ok_or_else(|| KunaError::sleigh("operand has no defining expression"))?;
            let pw: &dyn PatternExpressionContext = &*walker;
            let val = defexp.get_value(pw)?;
            push_hex_intb(s, val);
        }
        walker.pop_operand()?;
        Ok(())
    }
}

/// C++ `StartSymbol`: the current instruction address.  Its
/// `StartInstructionValue` pattern expression is stateless and built on
/// demand.
#[derive(Debug, Clone, Default)]
pub struct StartSymbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `EndSymbol`: the next instruction address.
#[derive(Debug, Clone, Default)]
pub struct EndSymbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `Next2Symbol`: the address of the instruction after the next.
#[derive(Debug, Clone, Default)]
pub struct Next2Symbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `FlowDestSymbol`: the destination address of a flow override (never
/// decoded from `.sla`; constructed programmatically).
#[derive(Debug, Clone, Default)]
pub struct FlowDestSymbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `FlowRefSymbol`: the reference address of a flow override (never
/// decoded from `.sla`; constructed programmatically).
#[derive(Debug, Clone, Default)]
pub struct FlowRefSymbol {
    const_space: Option<Rc<AddrSpace>>,
}

/// C++ `SubtableSymbol`: a table of constructors with its decode-time
/// decision tree.  The compiler-only `TokenPattern *pattern` and
/// `beingbuilt`/`errors` flags are present for the build side (ws4a); the
/// decode path leaves them at their defaults.
#[derive(Debug, Clone, Default)]
pub struct SubtableSymbol {
    construct: Vec<Constructor>,
    decisiontree: Option<DecisionNode>,
    /// (kuna build side) C++ `TokenPattern *pattern`.
    pattern: Option<TokenPattern>,
    /// (kuna build side) C++ `bool beingbuilt` (recursion guard).
    beingbuilt: bool,
    /// (kuna build side) C++ `bool errors`.
    errors: bool,
}

impl SubtableSymbol {
    /// C++ `SubtableSymbol::addConstructor` (sets the constructor id).
    pub fn add_constructor(&mut self, mut ct: Constructor) {
        // size_t -> uintm id assignment as in C++ `ct->setId(construct.size())`
        ct.set_id(self.construct.len() as u32);
        self.construct.push(ct);
    }

    /// C++ `SubtableSymbol::isBeingBuilt`.
    pub fn is_being_built(&self) -> bool {
        self.beingbuilt
    }

    /// C++ `SubtableSymbol::isError`.
    pub fn is_error(&self) -> bool {
        self.errors
    }

    /// C++ `SubtableSymbol::getPattern` (the built [`TokenPattern`]).
    pub fn get_pattern(&self) -> Option<&TokenPattern> {
        self.pattern.as_ref()
    }

    /// Inject the subtable's built [`TokenPattern`] (golden tests / out-of-band
    /// build) so `build_decision_tree` sees a "fully formed" table.
    pub fn set_built_pattern_for_test(&mut self, tp: TokenPattern) {
        self.pattern = Some(tp);
    }

    /// C++ `SubtableSymbol::getNumConstructors` (`size_t` returned as
    /// `int4`).
    pub fn get_num_constructors(&self) -> i32 {
        self.construct.len() as i32 // size_t -> int4 as in C++
    }

    /// C++ `SubtableSymbol::getConstructor(uintm id)` — unchecked vector
    /// indexing in C++ (UB out of range); an error here.
    pub fn get_constructor(&self, id: u32) -> KunaResult<&Constructor> {
        self.construct
            .get(id as usize)
            .ok_or_else(|| KunaError::sleigh("constructor id out of range (C++ indexes unchecked)"))
    }

    /// Mutable `getConstructor` (build side).
    fn construct_mut(&mut self, id: u32) -> KunaResult<&mut Constructor> {
        self.construct
            .get_mut(id as usize)
            .ok_or_else(|| KunaError::sleigh("constructor id out of range (C++ indexes unchecked)"))
    }

    /// Mutable constructor access for the WS4b driver (`getConstructor`,
    /// build side).
    pub fn get_constructor_mut(&mut self, id: u32) -> Option<&mut Constructor> {
        self.construct.get_mut(id as usize)
    }

    /// The decoded decision tree (None for an undecoded shell).
    pub fn get_decision_tree(&self) -> Option<&DecisionNode> {
        self.decisiontree.as_ref()
    }

    /// C++ `SubtableSymbol::resolve`: `decisiontree->resolve(walker)`
    /// (a null tree dereference is UB in C++; an error here).
    pub fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<u32> {
        let tree = self
            .decisiontree
            .as_ref()
            .ok_or_else(|| KunaError::sleigh("subtable has no decision tree (not decoded)"))?;
        tree.resolve(walker)
    }
}

// ---------------------------------------------------------------------------
// Context commands (ContextChange / ContextOp / ContextCommit)
// ---------------------------------------------------------------------------

/// C++ `ContextOp`: sets a field of the local context to a computed value.
#[derive(Debug, Clone, Default)]
pub struct ContextOp {
    /// Expression determining the value (C++ `PatternExpression *patexp`,
    /// null until decoded).
    patexp: Option<PatternExpression>,
    /// Index of word containing context variable to set.
    num: i32,
    /// Mask off size of variable (C++ `uintm`).
    mask: u32,
    /// Number of bits to shift value into place.
    shift: i32,
}

impl ContextOp {
    /// C++ `ContextOp(int4 startbit,int4 endbit,PatternExpression *pe)`.
    pub fn new(startbit: i32, endbit: i32, pe: PatternExpression) -> KunaResult<ContextOp> {
        let (num, shift, mask) = calc_maskword(startbit, endbit)?;
        Ok(ContextOp {
            patexp: Some(pe),
            num,
            mask,
            shift,
        })
    }

    /// C++ `ContextOp::validate`: error if the expression uses an operand
    /// that is not constructor-relative (such operands are evaluated BEFORE
    /// their offset is recovered).
    pub fn validate(&self, table: &SymbolTable) -> KunaResult<()> {
        let patexp = self
            .patexp
            .as_ref()
            .ok_or_else(|| KunaError::sleigh("context op has no expression (not decoded)"))?;
        let mut values: Vec<&PatternValue> = Vec::new();
        patexp.list_values(&mut values); // Get all the expression tokens
        for value in values {
            let PatternValue::OperandValue(val) = value else {
                continue;
            };
            // Certain operands cannot be used in context expressions
            // because these are evaluated BEFORE the operand offset
            // has been recovered. If the offset is not relative to
            // the base constructor, then we throw an error
            if !table.operand_value_is_constructor_relative(val)? {
                return Err(KunaError::sleigh(format!(
                    "{}: cannot be used in context expression",
                    name_text(table.operand_value_name(val)?)
                )));
            }
        }
        Ok(())
    }

    /// C++ `ContextOp::apply`.
    pub fn apply(&self, walker: &mut dyn SymbolWalkerChange) -> KunaResult<()> {
        let patexp = self
            .patexp
            .as_ref()
            .ok_or_else(|| KunaError::sleigh("context op has no expression (not decoded)"))?;
        let pw: &dyn PatternExpressionContext = &*walker;
        // C++ `uintm val = patexp->getValue(walker)`: intb -> uintm
        // truncation
        let mut val = patexp.get_value(pw)? as u32;
        // uintm <<= int4: shift is in [0,31] from calc_maskword; a decoded
        // shift is unchecked (C++ UB >= 32, x86-masked here)
        val = val.wshl(self.shift as u32); // count masked mod 32 anyway
        walker.set_context_word(self.num, val, self.mask);
        Ok(())
    }

    /// C++ `ContextOp::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&sla::ELEM_CONTEXT_OP);
        encoder.write_signed_integer(&sla::ATTRIB_I, i64::from(self.num));
        encoder.write_signed_integer(&sla::ATTRIB_SHIFT, i64::from(self.shift));
        encoder.write_unsigned_integer(&sla::ATTRIB_MASK, u64::from(self.mask));
        self.patexp
            .as_ref()
            .ok_or_else(|| KunaError::sleigh("context op has no expression (not decoded)"))?
            .encode(encoder);
        encoder.close_element(&sla::ELEM_CONTEXT_OP);
        Ok(())
    }

    /// C++ `ContextOp::decode`.
    pub fn decode(
        decoder: &mut dyn Decoder,
        trans: &dyn OperandValueResolver,
    ) -> KunaResult<ContextOp> {
        let el = decoder.open_element_id(&sla::ELEM_CONTEXT_OP)?;
        let num = decoder.read_signed_integer_id(&sla::ATTRIB_I)? as i32; // C++ implicit intb -> int4
        let shift = decoder.read_signed_integer_id(&sla::ATTRIB_SHIFT)? as i32; // C++ implicit intb -> int4
        let mask = decoder.read_unsigned_integer_id(&sla::ATTRIB_MASK)? as u32; // C++ implicit uintb -> uintm
        let patexp = PatternExpression::decode_expression(decoder, trans)?;
        decoder.close_element(el)?;
        Ok(ContextOp {
            patexp: Some(patexp),
            num,
            mask,
            shift,
        })
    }
}

/// C++ `ContextCommit`: commits a span of context downstream.
#[derive(Debug, Clone, Default)]
pub struct ContextCommit {
    /// C++ `TripleSymbol *sym` as a symbol id.
    sym: u32,
    /// Index of word containing context commit.
    num: i32,
    /// Mask of bits in word being committed (C++ `uintm`).
    mask: u32,
    /// Whether the context "flows" from the point of change.
    flow: bool,
}

impl ContextCommit {
    /// C++ `ContextCommit(TripleSymbol *s,int4 sbit,int4 ebit,bool fl)`,
    /// with the symbol passed as its id.
    pub fn new(sym: u32, sbit: i32, ebit: i32, fl: bool) -> KunaResult<ContextCommit> {
        let (num, _shift, mask) = calc_maskword(sbit, ebit)?;
        Ok(ContextCommit {
            sym,
            num,
            mask,
            flow: fl,
        })
    }

    /// C++ `ContextCommit::apply`.
    pub fn apply(&self, walker: &mut dyn SymbolWalkerChange) {
        walker.add_commit(self.sym, self.num, self.mask, self.flow);
    }

    /// C++ `ContextCommit::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_COMMIT);
        encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.sym));
        encoder.write_signed_integer(&sla::ATTRIB_NUMBER, i64::from(self.num));
        encoder.write_unsigned_integer(&sla::ATTRIB_MASK, u64::from(self.mask));
        encoder.write_bool(&sla::ATTRIB_FLOW, self.flow);
        encoder.close_element(&sla::ELEM_COMMIT);
    }

    /// C++ `ContextCommit::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<ContextCommit> {
        let el = decoder.open_element_id(&sla::ELEM_COMMIT)?;
        let sym = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32; // C++ implicit uintb -> uintm
        let num = decoder.read_signed_integer_id(&sla::ATTRIB_NUMBER)? as i32; // C++ implicit intb -> int4
        let mask = decoder.read_unsigned_integer_id(&sla::ATTRIB_MASK)? as u32; // C++ implicit uintb -> uintm
        let flow = decoder.read_bool_id(&sla::ATTRIB_FLOW)?;
        decoder.close_element(el)?;
        Ok(ContextCommit {
            sym,
            num,
            mask,
            flow,
        })
    }
}

/// The C++ `ContextChange` virtual base over its two subclasses (the
/// `clone()` virtual becomes `derive(Clone)`).
#[derive(Debug, Clone)]
pub enum ContextChange {
    /// C++ `ContextOp`.
    Op(ContextOp),
    /// C++ `ContextCommit`.
    Commit(ContextCommit),
}

impl ContextChange {
    /// C++ virtual `validate` (`ContextCommit::validate` is a no-op).
    pub fn validate(&self, table: &SymbolTable) -> KunaResult<()> {
        match self {
            ContextChange::Op(op) => op.validate(table),
            ContextChange::Commit(_) => Ok(()),
        }
    }

    /// C++ virtual `apply`.
    pub fn apply(&self, walker: &mut dyn SymbolWalkerChange) -> KunaResult<()> {
        match self {
            ContextChange::Op(op) => op.apply(walker),
            ContextChange::Commit(c) => {
                c.apply(walker);
                Ok(())
            }
        }
    }

    /// C++ virtual `encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        match self {
            ContextChange::Op(op) => op.encode(encoder),
            ContextChange::Commit(c) => {
                c.encode(encoder);
                Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

/// C++ `Constructor` ("This is NOT a symbol").  Compiler-only members
/// (`pattern`, `pateq`, `inerror`; see module docs) are dropped; pointers
/// become symbol ids / template handles.
#[derive(Debug, Clone, Default)]
pub struct Constructor {
    /// Owning `SubtableSymbol` id (C++ null parent of a decode shell is
    /// `None`).
    parent: Option<u32>,
    /// `OperandSymbol` ids.
    operands: Vec<u32>,
    /// Print pieces; an operand placeholder is the 2-byte string
    /// `[b'\n', b'A' + index]` exactly as in C++.
    printpiece: Vec<Vec<u8>>,
    /// Context commands.
    context: Vec<ContextChange>,
    /// The main p-code section (held through the [`SleighBaseTrans`] seam).
    templ: Option<ConstructTplHandle>,
    /// Other named p-code sections.
    namedtempl: Vec<Option<ConstructTplHandle>>,
    /// Minimum length taken up by this constructor in bytes.
    minimumlength: i32,
    /// Unique id of constructor within subtable (C++ `uintm`).
    id: u32,
    /// Index of first whitespace piece in `printpiece` (-1 if none).
    firstwhitespace: i32,
    /// If >= 0 then print only a single operand, no markup.
    flowthruindex: i32,
    lineno: i32,
    /// Source file index.
    src_index: i32,
    /// (kuna build side) C++ `PatternEquation *pateq`: the constructor's
    /// pattern equation, as an arena id ([`EqId`]) into the driver-owned
    /// [`EquationArena`].  `None` for a decode shell.
    pateq: Option<EqId>,
    /// (kuna build side) C++ `TokenPattern *pattern`: the built pattern,
    /// `None` until [`Constructor::build_pattern`] runs.
    pattern: Option<TokenPattern>,
    /// (kuna build side) C++ `mutable bool inerror`.
    inerror: bool,
    /// (kuna build side, WS4b seam) the operand handle re-index map computed
    /// by [`SymbolTable::order_operands`] (`handmap[original_index] =
    /// new_index`).  C++ applies `templ->changeHandleIndex(handmap)` inline,
    /// but the kuna `ConstructTpl` arena is owned by the WS4b driver, so the
    /// map is stashed here for the driver to apply.  Empty until ordered.
    handmap: Vec<i32>,
}

impl Constructor {
    /// C++ `Constructor(void)` (decode shell): `firstwhitespace`/
    /// `flowthruindex` start at -1.
    pub fn new() -> Constructor {
        Constructor {
            firstwhitespace: -1,
            flowthruindex: -1,
            ..Default::default()
        }
    }

    /// C++ `Constructor::addEquation(PatternEquation *pe)`: set the pattern
    /// equation (as its arena id).
    pub fn add_equation(&mut self, pe: EqId) {
        self.pateq = Some(pe);
    }

    /// C++ `Constructor::getPatternEquation` (the arena id).
    pub fn get_pattern_equation(&self) -> Option<EqId> {
        self.pateq
    }

    /// (kuna build side, WS4b seam) the operand handle re-index map computed
    /// during [`SymbolTable::order_operands`].  `handmap[original_index] =
    /// new_index`.  The WS4b driver applies it to the constructor's
    /// `ConstructTpl` sections via `change_handle_index` (the C++ inline
    /// `templ->changeHandleIndex(handmap)`).  Empty if no reorder ran.
    pub fn get_handmap(&self) -> &[i32] {
        &self.handmap
    }

    /// C++ `Constructor::getPattern` (the built [`TokenPattern`]).
    pub fn get_pattern(&self) -> Option<&TokenPattern> {
        self.pattern.as_ref()
    }

    /// Inject a pre-built [`TokenPattern`] (golden tests / WS4b that build the
    /// pattern out of band).
    pub fn set_built_pattern_for_test(&mut self, tp: TokenPattern) {
        self.pattern = Some(tp);
    }

    /// C++ `Constructor::setError`.
    pub fn set_error(&mut self, val: bool) {
        self.inerror = val;
    }

    /// C++ `Constructor::isError`.
    pub fn is_error(&self) -> bool {
        self.inerror
    }

    /// C++ `Constructor::setMinimumLength`.
    pub fn set_minimum_length(&mut self, l: i32) {
        self.minimumlength = l;
    }

    /// C++ `Constructor::getMinimumLength`.
    pub fn get_minimum_length(&self) -> i32 {
        self.minimumlength
    }

    /// C++ `Constructor::setId`.
    pub fn set_id(&mut self, i: u32) {
        self.id = i;
    }

    /// C++ `Constructor::getId`.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// C++ `Constructor::setLineno`.
    pub fn set_lineno(&mut self, ln: i32) {
        self.lineno = ln;
    }

    /// C++ `Constructor::getLineno`.
    pub fn get_lineno(&self) -> i32 {
        self.lineno
    }

    /// C++ `Constructor::setSrcIndex`.
    pub fn set_src_index(&mut self, index: i32) {
        self.src_index = index;
    }

    /// C++ `Constructor::getSrcIndex`.
    pub fn get_src_index(&self) -> i32 {
        self.src_index
    }

    /// C++ `Constructor::getParent` (the owning subtable's symbol id).
    pub fn get_parent(&self) -> Option<u32> {
        self.parent
    }

    /// C++ `Constructor(SubtableSymbol *p)` parent wiring (split out since
    /// the port stores ids).
    pub fn set_parent(&mut self, parent: u32) {
        self.parent = Some(parent);
    }

    /// C++ `Constructor::addInvisibleOperand` (operand passed as its
    /// symbol id).
    pub fn add_invisible_operand(&mut self, sym: u32) {
        self.operands.push(sym);
    }

    /// C++ `Constructor::addOperand` (operand passed as its symbol id).
    pub fn add_operand(&mut self, sym: u32) {
        // C++ `operstring[1] = ('A'+operands.size())`: int -> char
        // truncation encodes the operand index
        let operstring = vec![b'\n', (i32::from(b'A') + self.operands.len() as i32) as u8];
        self.operands.push(sym);
        self.printpiece.push(operstring); // Placeholder for operand's string
    }

    /// C++ `Constructor::addSyntax`.
    pub fn add_syntax(&mut self, syn: &[u8]) {
        if syn.is_empty() {
            return;
        }
        let has_non_space = syn.iter().any(|&c| c != b' ');
        let syntrim: &[u8] = if has_non_space { syn } else { b" " };
        if self.firstwhitespace == -1 && syntrim == b" " {
            self.firstwhitespace = self.printpiece.len() as i32; // size_t -> int4 as in C++
        }
        if self.printpiece.is_empty() {
            self.printpiece.push(syntrim.to_vec());
        } else if self.printpiece.last().expect("non-empty") == b" " && syntrim == b" " {
            // Don't add more whitespace
        } else if self.printpiece.last().expect("non-empty").first() == Some(&b'\n')
            || self.printpiece.last().expect("non-empty") == b" "
            || syntrim == b" "
        {
            self.printpiece.push(syntrim.to_vec());
        } else {
            self.printpiece
                .last_mut()
                .expect("non-empty")
                .extend_from_slice(syntrim);
        }
    }

    /// C++ `Constructor::removeTrailingSpace`.
    pub fn remove_trailing_space(&mut self) {
        // Allow for user to force extra space at end of printing
        if self.printpiece.last().map(|p| p == b" ").unwrap_or(false) {
            self.printpiece.pop();
        }
    }

    /// C++ `Constructor::addContext`.
    pub fn add_context(&mut self, vec: Vec<ContextChange>) {
        self.context = vec;
    }

    /// C++ `Constructor::getNumOperands` (`size_t` returned as `int4`).
    pub fn get_num_operands(&self) -> i32 {
        self.operands.len() as i32 // size_t -> int4 as in C++
    }

    /// C++ `Constructor::getOperand(int4 i)` — the operand's symbol id
    /// (unchecked vector indexing in C++; an error here).
    pub fn get_operand(&self, i: i32) -> KunaResult<u32> {
        self.operands
            .get(i as usize)
            .copied()
            .ok_or_else(|| KunaError::sleigh("operand index out of range (C++ indexes unchecked)"))
    }

    /// C++ `Constructor::getTempl`.
    pub fn get_templ(&self) -> Option<ConstructTplHandle> {
        self.templ
    }

    /// C++ `Constructor::getNamedTempl(int4 secnum)`: null past the end of
    /// the vector.
    pub fn get_named_templ(&self, secnum: i32) -> Option<ConstructTplHandle> {
        // C++ `if (secnum < namedtempl.size())`: int4 vs size_t mixed
        // comparison — secnum sign-extends to 64-bit unsigned, so a
        // negative secnum fails the test and returns null, as in C++.
        if (secnum as i64 as u64) < self.namedtempl.len() as u64 {
            self.namedtempl[secnum as usize]
        } else {
            None
        }
    }

    /// C++ `Constructor::getNumSections` (`size_t` returned as `int4`).
    pub fn get_num_sections(&self) -> i32 {
        self.namedtempl.len() as i32 // size_t -> int4 as in C++
    }

    /// The raw print pieces (test/inspection surface).
    pub fn get_print_pieces(&self) -> &[Vec<u8>] {
        &self.printpiece
    }

    /// C++ `flowthruindex` (>= 0 when printing flows through one operand).
    pub fn get_flowthru_index(&self) -> i32 {
        self.flowthruindex
    }

    /// C++ `firstwhitespace`.
    pub fn get_first_whitespace(&self) -> i32 {
        self.firstwhitespace
    }

    /// The decoded context commands (test/inspection surface).
    pub fn get_context_changes(&self) -> &[ContextChange] {
        &self.context
    }

    /// C++ `Constructor::print`.
    pub fn print(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        for piece in &self.printpiece {
            // C++ `(*piter)[0] == '\n'`: indexing an empty std::string at
            // size() yields '\0', i.e. the else branch.
            if piece.first() == Some(&b'\n') {
                let index = i32::from(piece[1]) - i32::from(b'A');
                self.print_operand(s, walker, table, index)?;
            } else {
                s.push_str(&String::from_utf8_lossy(piece));
            }
        }
        Ok(())
    }

    /// Shared body of the three print loops: `operands[index]->print(s,
    /// walker)` (C++ indexes `operands` unchecked; an error here).
    fn print_operand(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
        index: i32,
    ) -> KunaResult<()> {
        let opid = self.get_operand(index)?;
        // C++ holds OperandSymbol* and virtual-dispatches print; the enum
        // dispatch through the symbol table is the same call.
        table.symbol(opid)?.print(s, walker, table)
    }

    /// C++ `Constructor::printMnemonic`.
    pub fn print_mnemonic(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        // C++ nested flowthruindex test + dynamic_cast<SubtableSymbol*>(
        // operands[flowthruindex]->getDefiningSymbol()) null test (&&
        // short-circuits identically)
        if self.flowthruindex != -1 && self.flowthru_subtable(table)?.is_some() {
            walker.push_operand(self.flowthruindex)?;
            let ctref = walker.get_constructor()?;
            table
                .get_constructor(ctref)?
                .print_mnemonic(s, walker, table)?;
            walker.pop_operand()?;
            return Ok(());
        }
        // C++ `(firstwhitespace==-1) ? printpiece.size() : firstwhitespace`
        // (size_t -> int4 as in C++)
        let endind = if self.firstwhitespace == -1 {
            self.printpiece.len() as i32 // size_t -> int4 as in C++
        } else {
            self.firstwhitespace
        };
        for i in 0..endind {
            let piece = &self.printpiece[i as usize];
            if piece.first() == Some(&b'\n') {
                let index = i32::from(piece[1]) - i32::from(b'A');
                self.print_operand(s, walker, table, index)?;
            } else {
                s.push_str(&String::from_utf8_lossy(piece));
            }
        }
        Ok(())
    }

    /// C++ `Constructor::printBody`.
    pub fn print_body(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        // C++ nested flowthruindex/dynamic_cast tests (&& short-circuits
        // identically)
        if self.flowthruindex != -1 && self.flowthru_subtable(table)?.is_some() {
            walker.push_operand(self.flowthruindex)?;
            let ctref = walker.get_constructor()?;
            table.get_constructor(ctref)?.print_body(s, walker, table)?;
            walker.pop_operand()?;
            return Ok(());
        }
        if self.firstwhitespace == -1 {
            return Ok(()); // Nothing to print after firstwhitespace
        }
        let mut i = self.firstwhitespace + 1;
        // C++ `i < printpiece.size()`: int4 vs size_t mixed comparison —
        // i sign-extends to 64-bit unsigned (i is >= 0 here in practice).
        while (i as i64 as u64) < self.printpiece.len() as u64 {
            let piece = &self.printpiece[i as usize];
            if piece.first() == Some(&b'\n') {
                let index = i32::from(piece[1]) - i32::from(b'A');
                self.print_operand(s, walker, table, index)?;
            } else {
                s.push_str(&String::from_utf8_lossy(piece));
            }
            i += 1;
        }
        Ok(())
    }

    /// The `dynamic_cast<SubtableSymbol*>(operands[flowthruindex]->
    /// getDefiningSymbol())` test shared by printMnemonic/printBody:
    /// `Some(subtable symbol id)` when the flow-through operand is defined
    /// by a subtable.
    fn flowthru_subtable(&self, table: &SymbolTable) -> KunaResult<Option<u32>> {
        let opid = self.get_operand(self.flowthruindex)?;
        let opsym = table.operand_symbol(opid)?;
        match opsym.get_defining_symbol() {
            Some(tid) if table.symbol(tid)?.get_type() == SymbolType::Subtable => Ok(Some(tid)),
            _ => Ok(None),
        }
    }

    /// C++ `Constructor::applyContext`.
    pub fn apply_context(&self, walker: &mut dyn SymbolWalkerChange) -> KunaResult<()> {
        for change in &self.context {
            change.apply(walker)?;
        }
        Ok(())
    }

    /// C++ `Constructor::printInfo`: identifying information for error
    /// messages.
    pub fn print_info(&self, s: &mut String, table: &SymbolTable) -> KunaResult<()> {
        // C++ dereferences parent unconditionally
        let parent = self
            .parent
            .ok_or_else(|| KunaError::sleigh("constructor has no parent table"))?;
        s.push_str(&format!(
            "table \"{}\" constructor starting at line {}",
            name_text(table.symbol(parent)?.get_name()),
            self.lineno
        ));
        Ok(())
    }

    /// C++ `Constructor::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder, trans: &dyn SleighBaseTrans) -> KunaResult<()> {
        encoder.open_element(&sla::ELEM_CONSTRUCTOR);
        // C++ dereferences parent unconditionally
        let parent = self
            .parent
            .ok_or_else(|| KunaError::sleigh("constructor has no parent table"))?;
        encoder.write_unsigned_integer(&sla::ATTRIB_PARENT, u64::from(parent));
        encoder.write_signed_integer(&sla::ATTRIB_FIRST, i64::from(self.firstwhitespace));
        encoder.write_signed_integer(&sla::ATTRIB_LENGTH, i64::from(self.minimumlength));
        encoder.write_signed_integer(&sla::ATTRIB_SOURCE, i64::from(self.src_index));
        encoder.write_signed_integer(&sla::ATTRIB_LINE, i64::from(self.lineno));
        for opid in &self.operands {
            encoder.open_element(&sla::ELEM_OPER);
            encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(*opid));
            encoder.close_element(&sla::ELEM_OPER);
        }
        for piece in &self.printpiece {
            if piece.first() == Some(&b'\n') {
                let index = i32::from(piece[1]) - i32::from(b'A');
                encoder.open_element(&sla::ELEM_OPPRINT);
                encoder.write_signed_integer(&sla::ATTRIB_ID, i64::from(index));
                encoder.close_element(&sla::ELEM_OPPRINT);
            } else {
                encoder.open_element(&sla::ELEM_PRINT);
                encoder.write_string(&sla::ATTRIB_PIECE, piece);
                encoder.close_element(&sla::ELEM_PRINT);
            }
        }
        for change in &self.context {
            change.encode(encoder)?;
        }
        if let Some(handle) = self.templ {
            trans.encode_construct_tpl(handle, -1, encoder)?;
        }
        for (i, named) in self.namedtempl.iter().enumerate() {
            if let Some(handle) = named {
                // Some sections may be NULL (skipped); usize -> int4 section
                // index as in the C++ loop variable
                trans.encode_construct_tpl(*handle, i as i32, encoder)?;
            }
        }
        encoder.close_element(&sla::ELEM_CONSTRUCTOR);
        Ok(())
    }

    /// C++ `Constructor::decode`.  `trans` resolves OperandValue validation
    /// (the [`OperandValueResolver`] seam) and decodes ConstructTpl
    /// sections (the [`SleighBaseTrans`] seam); symbol cross-references are
    /// stored as ids.
    pub fn decode(
        decoder: &mut dyn Decoder,
        resolver: &dyn OperandValueResolver,
        trans: &mut dyn SleighBaseTrans,
    ) -> KunaResult<Constructor> {
        let el = decoder.open_element_id(&sla::ELEM_CONSTRUCTOR)?;
        let mut ct = Constructor::new();
        let parent = decoder.read_unsigned_integer_id(&sla::ATTRIB_PARENT)? as u32; // C++ implicit uintb -> uintm
        ct.parent = Some(parent);
        ct.firstwhitespace = decoder.read_signed_integer_id(&sla::ATTRIB_FIRST)? as i32; // C++ implicit intb -> int4
        ct.minimumlength = decoder.read_signed_integer_id(&sla::ATTRIB_LENGTH)? as i32; // C++ implicit intb -> int4
        ct.src_index = decoder.read_signed_integer_id(&sla::ATTRIB_SOURCE)? as i32; // C++ implicit intb -> int4
        ct.lineno = decoder.read_signed_integer_id(&sla::ATTRIB_LINE)? as i32; // C++ implicit intb -> int4
        let mut subel = decoder.peek_element()?;
        while subel != 0 {
            if subel == sla::ELEM_OPER {
                decoder.open_element()?;
                let id = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32; // C++ implicit uintb -> uintm
                ct.operands.push(id);
                decoder.close_element(subel)?;
            } else if subel == sla::ELEM_PRINT {
                decoder.open_element()?;
                ct.printpiece
                    .push(decoder.read_string_id(&sla::ATTRIB_PIECE)?);
                decoder.close_element(subel)?;
            } else if subel == sla::ELEM_OPPRINT {
                decoder.open_element()?;
                let index = decoder.read_signed_integer_id(&sla::ATTRIB_ID)? as i32; // C++ implicit intb -> int4
                // C++ `operstring[1] = ('A' + index)`: int -> char
                // truncation
                let operstring = vec![b'\n', (i32::from(b'A') + index) as u8];
                ct.printpiece.push(operstring);
                decoder.close_element(subel)?;
            } else if subel == sla::ELEM_CONTEXT_OP {
                let c_op = ContextOp::decode(decoder, resolver)?;
                ct.context.push(ContextChange::Op(c_op));
            } else if subel == sla::ELEM_COMMIT {
                let c_op = ContextCommit::decode(decoder)?;
                ct.context.push(ContextChange::Commit(c_op));
            } else {
                // C++: ConstructTpl().decode(decoder) — through the seam
                let (sectionid, handle) = trans.decode_construct_tpl(decoder)?;
                if sectionid < 0 {
                    if ct.templ.is_some() {
                        return Err(KunaError::lowlevel("Duplicate main section"));
                    }
                    ct.templ = Some(handle);
                } else {
                    // C++ `while(namedtempl.size() <= sectionid)`: int4 vs
                    // size_t mixed comparison (sectionid is >= 0 here)
                    while self_namedtempl_len_i64(&ct.namedtempl) <= i64::from(sectionid) {
                        ct.namedtempl.push(None);
                    }
                    if ct.namedtempl[sectionid as usize].is_some() {
                        return Err(KunaError::lowlevel("Duplicate named section"));
                    }
                    ct.namedtempl[sectionid as usize] = Some(handle);
                }
            }
            subel = decoder.peek_element()?;
        }
        if ct.printpiece.len() == 1 && ct.printpiece[0].first() == Some(&b'\n') {
            ct.flowthruindex = i32::from(ct.printpiece[0][1]) - i32::from(b'A');
        } else {
            ct.flowthruindex = -1;
        }
        decoder.close_element(el)?;
        Ok(ct)
    }
}

/// Helper for the `namedtempl.size() <= sectionid` mixed comparison.
fn self_namedtempl_len_i64(v: &[Option<ConstructTplHandle>]) -> i64 {
    v.len() as i64 // size_t -> i64: section counts are tiny
}

// ---------------------------------------------------------------------------
// DecisionNode
// ---------------------------------------------------------------------------

/// C++ `DecisionNode`: a node in a subtable's decode decision tree.  The
/// `parent` back-pointer is dropped (only the compiler-side `split()` reads
/// it); constructors are stored by id within the owning subtable.
#[derive(Debug, Clone, Default)]
pub struct DecisionNode {
    /// (pattern, constructor id) pairs of a terminal node.
    list: Vec<(DisjointPattern, u32)>,
    /// Child nodes of a non-terminal node (`2^bitsize` of them).
    children: Vec<DecisionNode>,
    /// Total number of patterns we distinguish.
    num: i32,
    /// True if this is decision based on context.
    contextdecision: bool,
    /// Bits in the stream on which to base the decision.
    startbit: i32,
    /// Field size; 0 marks a terminal node.
    bitsize: i32,
}

impl DecisionNode {
    /// C++ `DecisionNode::resolve`: dispatch to a constructor id, or the
    /// C++ `BadDataError` when no terminal pattern matches.
    pub fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<u32> {
        if self.bitsize == 0 {
            // The node is terminal
            let pw: &dyn PatternExpressionContext = walker;
            for (pat, ct) in &self.list {
                if pat.is_match(pw)? {
                    return Ok(*ct);
                }
            }
            let mut s = String::new();
            s.push(walker.get_addr().get_shortcut());
            walker.get_addr().print_raw(&mut s)?;
            s.push_str(": Unable to resolve constructor");
            return Err(KunaError::bad_data(s));
        }
        let val = if self.contextdecision {
            walker.get_context_bits(self.startbit, self.bitsize)?
        } else {
            walker.get_instruction_bits(self.startbit, self.bitsize)?
        };
        // C++ `children[val]->resolve(walker)`: val < 2^bitsize by
        // construction; an undersized children vector (corrupt .sla) is UB
        // in C++ and an indexing panic here (ADR 0004).
        self.children[val as usize].resolve(walker)
    }

    /// The terminal pattern list (test/inspection surface).
    pub fn get_list(&self) -> &[(DisjointPattern, u32)] {
        &self.list
    }

    /// The child nodes (test/inspection surface).
    pub fn get_children(&self) -> &[DecisionNode] {
        &self.children
    }

    /// The decision field (test/inspection surface): `(startbit, bitsize,
    /// contextdecision)`.
    pub fn get_field(&self) -> (i32, i32, bool) {
        (self.startbit, self.bitsize, self.contextdecision)
    }

    // ---- build side (ws4a) ----

    /// C++ `DecisionNode(DecisionNode *p)` for the root (`p == 0`).
    fn new_root() -> DecisionNode {
        DecisionNode::default()
    }

    /// C++ `DecisionNode(DecisionNode *p)` child node.
    fn new_child() -> DecisionNode {
        DecisionNode::default()
    }

    /// C++ `DecisionNode::addConstructorPair`.
    fn add_constructor_pair(&mut self, pat: &DisjointPattern, ct: u32) {
        // C++ clones via simplifyClone so the node owns its pattern.
        let clone = expect_disjoint_clone(&pat.simplify_clone());
        self.list.push((clone, ct));
        self.num += 1;
    }

    /// C++ `DecisionNode::getMaximumLength(bool context)`.
    fn get_maximum_length(&self, context: bool) -> i32 {
        let mut max = 0;
        for (pat, _) in &self.list {
            let val = pat.get_length(context);
            if val > max {
                max = val;
            }
        }
        max
    }

    /// C++ `DecisionNode::getNumFixed(int4 low,int4 size,bool context)`.
    fn get_num_fixed(&self, low: i32, size: i32, context: bool) -> i32 {
        let mut count = 0;
        // m = (size==32) ? 0 : (1<<size); m = m-1
        let mut m: u32 = if size == 32 { 0 } else { 1u32 << size };
        m = m.wrapping_sub(1);
        for (pat, _) in &self.list {
            let mask = pat.get_mask(low, size, context);
            if (mask & m) == m {
                count += 1;
            }
        }
        count
    }

    /// C++ `DecisionNode::getScore(int4 low,int4 size,bool context)`.
    fn get_score(&self, low: i32, size: i32, context: bool) -> f64 {
        let num_bins = 1usize << size; // size is between 1 and 8
        let mut m: u32 = 1u32 << size;
        m = m.wrapping_sub(1);

        let mut total = 0i32;
        let mut count = vec![0i32; num_bins];

        for (pat, _) in &self.list {
            let mask = pat.get_mask(low, size, context);
            if (mask & m) != m {
                continue; // Skip if field not fully specified
            }
            let val = pat.get_value(low, size, context);
            total += 1;
            count[val as usize] += 1;
        }
        if total <= 0 {
            return -1.0;
        }
        let mut sc = 0.0f64;
        let listlen = self.list.len() as i32;
        for &c in count.iter().take(num_bins) {
            if c <= 0 {
                continue;
            }
            if c >= listlen {
                return -1.0;
            }
            let p = (c as f64) / (total as f64);
            sc -= p * p.ln();
        }
        sc / 2.0f64.ln()
    }

    /// C++ `DecisionNode::chooseOptimalField`.
    fn choose_optimal_field(&mut self) {
        let mut score = 0.0f64;
        let mut maxfixed = 1i32;

        // single-bit fields, context then instruction
        let mut context = true;
        loop {
            let maxlength = 8 * self.get_maximum_length(context);
            for sbit in 0..maxlength {
                let numfixed = self.get_num_fixed(sbit, 1, context);
                if numfixed < maxfixed {
                    continue;
                }
                let sc = self.get_score(sbit, 1, context);
                if numfixed > maxfixed && sc > 0.0 {
                    score = sc;
                    maxfixed = numfixed;
                    self.startbit = sbit;
                    self.bitsize = 1;
                    self.contextdecision = context;
                    continue;
                }
                if sc > score {
                    score = sc;
                    self.startbit = sbit;
                    self.bitsize = 1;
                    self.contextdecision = context;
                }
            }
            context = !context;
            if context {
                break;
            }
        }

        // multi-bit fields (2..=8), context then instruction
        let mut context = true;
        loop {
            let maxlength = 8 * self.get_maximum_length(context);
            for size in 2..=8 {
                let mut sbit = 0;
                while sbit < maxlength - size + 1 {
                    if self.get_num_fixed(sbit, size, context) >= maxfixed {
                        let sc = self.get_score(sbit, size, context);
                        if sc > score {
                            score = sc;
                            self.startbit = sbit;
                            self.bitsize = size;
                            self.contextdecision = context;
                        }
                    }
                    sbit += 1;
                }
            }
            context = !context;
            if context {
                break;
            }
        }
        if score <= 0.0 {
            self.bitsize = 0; // treat the node as terminal
        }
    }

    /// C++ `DecisionNode::consistentValues(vector<uint4> &bins,
    /// DisjointPattern *pat)`.
    fn consistent_values(&self, bins: &mut Vec<u32>, pat: &DisjointPattern) {
        let mut m: u32 = if self.bitsize == 32 { 0 } else { 1u32 << self.bitsize };
        m = m.wrapping_sub(1);
        let common_mask = m & pat.get_mask(self.startbit, self.bitsize, self.contextdecision);
        let common_value = common_mask & pat.get_value(self.startbit, self.bitsize, self.contextdecision);
        let dont_care_mask = m ^ common_mask;

        let mut i: u32 = 0;
        loop {
            if (i & dont_care_mask) == i {
                bins.push(common_value | i);
            }
            if i == dont_care_mask {
                break;
            }
            i += 1;
        }
    }

    /// C++ `DecisionNode::split(DecisionProperties &props)`.
    fn split(&mut self, props: &mut DecisionProperties) {
        if self.list.len() <= 1 {
            self.bitsize = 0; // Only one pattern, terminal node by default
            return;
        }
        self.choose_optimal_field();
        if self.bitsize == 0 {
            self.order_patterns(props);
            return;
        }
        // C++ guard: a child cannot keep as many patterns as the parent (the
        // recursion would not terminate).  The parent check uses parent->num;
        // here `self.num` already equals the parent's count at split time.
        let num_children = 1usize << self.bitsize;
        let mut children: Vec<DecisionNode> = Vec::with_capacity(num_children);
        for _ in 0..num_children {
            children.push(DecisionNode::new_child());
        }
        // Move each pattern into every consistent bin.
        let list = std::mem::take(&mut self.list);
        for (pat, ct) in &list {
            let mut vals: Vec<u32> = Vec::new();
            self.consistent_values(&mut vals, pat);
            for &v in &vals {
                children[v as usize].add_constructor_pair(pat, *ct);
            }
        }
        self.children = children;
        for child in self.children.iter_mut() {
            child.split(props);
        }
    }

    /// C++ `DecisionNode::orderPatterns(DecisionProperties &props)`.
    fn order_patterns(&mut self, props: &mut DecisionProperties) {
        let mut conflictlist: Vec<(usize, usize)> = Vec::new();

        // Check for identical patterns.
        for i in 0..self.list.len() {
            for j in 0..i {
                if self.list[i].0.identical(&self.list[j].0) {
                    props.identical_pattern(self.list[i].1, self.list[j].1);
                }
            }
        }

        // Insertion-sort by specialization (most specialized first), tracking
        // conflicts.  We mirror the C++ index dance on a working copy.
        let original = self.list.clone();
        let mut sorted: Vec<(DisjointPattern, u32)> = Vec::with_capacity(original.len());
        for i in 0..original.len() {
            let ipat = &original[i].0;
            let iconst = original[i].1;
            let mut j = 0usize;
            while j < i {
                let jpat = &original[j].0;
                let jconst = original[j].1;
                if ipat.specializes(jpat) {
                    break;
                }
                if !jpat.specializes(ipat) {
                    // potential conflict
                    if iconst != jconst {
                        conflictlist.push((i, j));
                    }
                }
                j += 1;
            }
            // insert original[i] at position j in sorted
            sorted.insert(j, original[i].clone());
        }
        self.list = sorted;

        // Check if intersection patterns resolve each conflict.
        let mut k = 0;
        while k < conflictlist.len() {
            let (i, j) = conflictlist[k];
            let pat1 = &original[i].0;
            let const1 = original[i].1;
            let pat2 = &original[j].0;
            let const2 = original[j].1;
            let mut resolved = false;
            for (tpat, tconst) in &self.list {
                if std::ptr::eq(tpat, pat1) {}
                // C++ compares pointer identity (tpat==pat1 && tconst==const1)
                // to detect "ran out of specializations".  After the sort the
                // patterns are clones, so identity is lost; mirror the C++
                // semantics by value+constructor identity instead.
                if tpat.identical(pat1) && *tconst == const1 {
                    break;
                }
                if tpat.identical(pat2) && *tconst == const2 {
                    break;
                }
                if tpat.resolves_intersect(pat1, pat2) {
                    resolved = true;
                    break;
                }
            }
            if !resolved {
                props.conflicting_pattern(const1, const2);
            }
            k += 1;
        }
    }

    /// C++ `DecisionNode::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_DECISION);
        encoder.write_signed_integer(&sla::ATTRIB_NUMBER, i64::from(self.num));
        encoder.write_bool(&sla::ATTRIB_CONTEXT, self.contextdecision);
        encoder.write_signed_integer(&sla::ATTRIB_STARTBIT, i64::from(self.startbit));
        encoder.write_signed_integer(&sla::ATTRIB_SIZE, i64::from(self.bitsize));
        for (pat, ct) in &self.list {
            encoder.open_element(&sla::ELEM_PAIR);
            // C++ writeSignedInteger(ATTRIB_ID, ct->getId()): uintm -> intb
            encoder.write_signed_integer(&sla::ATTRIB_ID, i64::from(*ct));
            pat.encode(encoder);
            encoder.close_element(&sla::ELEM_PAIR);
        }
        for child in &self.children {
            child.encode(encoder);
        }
        encoder.close_element(&sla::ELEM_DECISION);
    }

    /// C++ `DecisionNode::decode(decoder, parent, sub)`: `parent` is
    /// dropped (see struct docs) and `sub` reduces to the owning subtable's
    /// live constructor count, used to validate pair ids.
    pub fn decode(decoder: &mut dyn Decoder, numct: i32) -> KunaResult<DecisionNode> {
        let el = decoder.open_element_id(&sla::ELEM_DECISION)?;
        let mut node = DecisionNode {
            num: decoder.read_signed_integer_id(&sla::ATTRIB_NUMBER)? as i32, // C++ implicit intb -> int4
            contextdecision: decoder.read_bool_id(&sla::ATTRIB_CONTEXT)?,
            startbit: decoder.read_signed_integer_id(&sla::ATTRIB_STARTBIT)? as i32, // C++ implicit intb -> int4
            bitsize: decoder.read_signed_integer_id(&sla::ATTRIB_SIZE)? as i32, // C++ implicit intb -> int4
            ..Default::default()
        };
        let mut subel = decoder.peek_element()?;
        while subel != 0 {
            if subel == sla::ELEM_PAIR {
                decoder.open_element()?;
                // C++ `uintm id = decoder.readSignedInteger(ATTRIB_ID)`:
                // intb -> uintm truncation
                let id = decoder.read_signed_integer_id(&sla::ATTRIB_ID)? as u32;
                // C++ `id >= sub->getNumConstructors()`: uintm vs int4 —
                // the int4 converts to uintm (usual arithmetic conversions)
                if id >= numct as u32 {
                    return Err(KunaError::decoder("Invalid constructor id"));
                }
                let pat = DisjointPattern::decode_disjoint(decoder)?;
                node.list.push((pat, id));
                decoder.close_element(subel)?;
            } else if subel == sla::ELEM_DECISION {
                let subnode = DecisionNode::decode(decoder, numct)?;
                node.children.push(subnode);
            }
            subel = decoder.peek_element()?;
        }
        decoder.close_element(el)?;
        Ok(node)
    }
}

/// C++ `DecisionProperties`: collects the identical/conflicting constructor
/// pairs found by `DecisionNode::orderPatterns`.  C++ keys these by
/// `Constructor*` and flips `Constructor::setError` directly; here they are
/// recorded as `(constructor index in subtable)` pairs and the driver (WS4b)
/// reports them / flips the error flag.
#[derive(Debug, Clone, Default)]
pub struct DecisionProperties {
    identerrors: Vec<(u32, u32)>,
    conflicterrors: Vec<(u32, u32)>,
}

impl DecisionProperties {
    /// A fresh property collector.
    pub fn new() -> DecisionProperties {
        DecisionProperties::default()
    }

    /// C++ `DecisionProperties::identicalPattern` (records the pair once).
    fn identical_pattern(&mut self, a: u32, b: u32) {
        if !self.identerrors.contains(&(a, b)) && !self.identerrors.contains(&(b, a)) {
            self.identerrors.push((a, b));
        }
    }

    /// C++ `DecisionProperties::conflictingPattern`.
    fn conflicting_pattern(&mut self, a: u32, b: u32) {
        if !self.conflicterrors.contains(&(a, b)) && !self.conflicterrors.contains(&(b, a)) {
            self.conflicterrors.push((a, b));
        }
    }

    /// C++ `DecisionProperties::getIdentErrors`.
    pub fn get_ident_errors(&self) -> &[(u32, u32)] {
        &self.identerrors
    }

    /// C++ `DecisionProperties::getConflictErrors`.
    pub fn get_conflict_errors(&self) -> &[(u32, u32)] {
        &self.conflicterrors
    }
}

/// The C++ `(DisjointPattern *)pat->simplifyClone()` cast: a result that is
/// not disjoint is UB upstream, an error/clone-panic here (ADR 0004).
fn expect_disjoint(p: &Pattern) -> KunaResult<DisjointPattern> {
    match p {
        Pattern::Disjoint(d) => Ok(d.clone()),
        Pattern::Or(_) => Err(KunaError::sleigh(
            "decision tree: expected a DisjointPattern (C++ UB cast)",
        )),
    }
}

/// `(DisjointPattern *)pat->simplifyClone()` where the C++ result is known to
/// be disjoint (the input was a `DisjointPattern`); panics otherwise.
fn expect_disjoint_clone(p: &Pattern) -> DisjointPattern {
    match p {
        Pattern::Disjoint(d) => d.clone(),
        Pattern::Or(_) => {
            panic!("addConstructorPair: simplifyClone of a DisjointPattern is not disjoint (C++ UB)")
        }
    }
}

/// Bridges `OperandEquation::resolveOperandLeft`'s operand mutations to the
/// [`SymbolTable`] (the C++ `state.operands[index]` member writes).
struct ConstructorOperandSink<'a> {
    table: &'a mut SymbolTable,
    operand_ids: &'a [u32],
}

impl OperandResolveSink for ConstructorOperandSink<'_> {
    fn is_offset_irrelevant(&self, index: i32) -> bool {
        let opid = self.operand_ids[index as usize];
        // C++ dereferences unchecked; treat a bad symbol as not-irrelevant
        // (the build would already have errored earlier).
        self.table
            .operand_symbol(opid)
            .map(|op| op.is_offset_irrelevant())
            .unwrap_or(false)
    }

    fn set_offset(&mut self, index: i32, offsetbase: i32, reloffset: u32) {
        let opid = self.operand_ids[index as usize];
        if let Ok(op) = self.table.operand_symbol_mut(opid) {
            op.set_offset(offsetbase, reloffset);
        }
    }
}

// ---------------------------------------------------------------------------
// SleighSymbol
// ---------------------------------------------------------------------------

/// The C++ `SleighSymbol` subclass payloads as an enum (see module docs;
/// compiler-only classes have no variant).
#[derive(Debug, Clone)]
pub enum SymbolKind {
    /// C++ `SpaceSymbol`.
    Space(SpaceSymbol),
    /// C++ `TokenSymbol`.
    Token(TokenSymbol),
    /// C++ `UserOpSymbol`.
    UserOp(UserOpSymbol),
    /// C++ `EpsilonSymbol`.
    Epsilon(EpsilonSymbol),
    /// C++ `ValueSymbol`.
    Value(ValueSymbol),
    /// C++ `ValueMapSymbol`.
    ValueMap(ValueMapSymbol),
    /// C++ `NameSymbol`.
    Name(NameSymbol),
    /// C++ `VarnodeSymbol`.
    Varnode(VarnodeSymbol),
    /// C++ `ContextSymbol`.
    Context(ContextSymbol),
    /// C++ `VarnodeListSymbol`.
    VarnodeList(VarnodeListSymbol),
    /// C++ `OperandSymbol`.
    Operand(OperandSymbol),
    /// C++ `StartSymbol`.
    Start(StartSymbol),
    /// C++ `EndSymbol`.
    End(EndSymbol),
    /// C++ `Next2Symbol`.
    Next2(Next2Symbol),
    /// C++ `FlowDestSymbol`.
    FlowDest(FlowDestSymbol),
    /// C++ `FlowRefSymbol`.
    FlowRef(FlowRefSymbol),
    /// C++ `SubtableSymbol`.
    Subtable(SubtableSymbol),
}

/// C++ `SleighSymbol`: the name/id/scope header common to every symbol,
/// wrapping the subclass payload.
#[derive(Debug, Clone)]
pub struct SleighSymbol {
    name: Vec<u8>,
    /// Unique id across all symbols (C++ `uintm`).
    id: u32,
    /// Unique id of scope this symbol is in (C++ `uintm`).
    scopeid: u32,
    kind: SymbolKind,
}

impl SleighSymbol {
    /// C++ `SleighSymbol(const string &nm)` with an explicit payload
    /// (id/scopeid are filled in by [`SymbolTable::add_symbol`] or decode).
    pub fn new(nm: &[u8], kind: SymbolKind) -> SleighSymbol {
        SleighSymbol {
            name: nm.to_vec(),
            id: 0,
            scopeid: 0,
            kind,
        }
    }

    /// C++ `SpaceSymbol(AddrSpace *spc)` (named after the space).
    pub fn new_space(spc: Rc<AddrSpace>) -> SleighSymbol {
        let name = spc.get_name().as_bytes().to_vec();
        SleighSymbol::new(&name, SymbolKind::Space(SpaceSymbol { space: spc }))
    }

    /// C++ `TokenSymbol(Token *t)` (named after the token).
    pub fn new_token(tok: Token) -> SleighSymbol {
        let name = tok.get_name().to_vec();
        SleighSymbol::new(&name, SymbolKind::Token(TokenSymbol { tok }))
    }

    /// C++ `UserOpSymbol(const string &nm)`.
    pub fn new_userop(nm: &[u8]) -> SleighSymbol {
        SleighSymbol::new(nm, SymbolKind::UserOp(UserOpSymbol { index: 0 }))
    }

    /// C++ `EpsilonSymbol(const string &nm,AddrSpace *spc)`.
    pub fn new_epsilon(nm: &[u8], spc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Epsilon(EpsilonSymbol {
                const_space: Some(spc),
            }),
        )
    }

    /// C++ `ValueSymbol(const string &nm,PatternValue *pv)`.
    pub fn new_value(nm: &[u8], pv: PatternValue) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Value(ValueSymbol { patval: Some(pv) }),
        )
    }

    /// C++ `ValueMapSymbol(const string &nm,PatternValue *pv,const
    /// vector<intb> &vt)`.
    pub fn new_valuemap(nm: &[u8], pv: PatternValue, vt: Vec<i64>) -> KunaResult<SleighSymbol> {
        let mut vm = ValueMapSymbol {
            value: ValueSymbol { patval: Some(pv) },
            valuetable: vt,
            tableisfilled: false,
        };
        vm.check_table_fill()?;
        Ok(SleighSymbol::new(nm, SymbolKind::ValueMap(vm)))
    }

    /// C++ `NameSymbol(const string &nm,PatternValue *pv,const
    /// vector<string> &nt)`.
    pub fn new_name_symbol(
        nm: &[u8],
        pv: PatternValue,
        nt: Vec<Vec<u8>>,
    ) -> KunaResult<SleighSymbol> {
        let mut ns = NameSymbol {
            value: ValueSymbol { patval: Some(pv) },
            nametable: nt,
            tableisfilled: false,
        };
        ns.check_table_fill()?;
        Ok(SleighSymbol::new(nm, SymbolKind::Name(ns)))
    }

    /// C++ `VarnodeSymbol(const string &nm,AddrSpace *base,uintb offset,
    /// int4 size)`.
    pub fn new_varnode(nm: &[u8], base: Rc<AddrSpace>, offset: u64, size: i32) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Varnode(VarnodeSymbol {
                fix: VarnodeData {
                    space: Some(base),
                    offset,
                    size: size as u32, // C++ implicit int4 -> uint4 (VarnodeData::size)
                },
                context_bits: false,
            }),
        )
    }

    /// C++ `ContextSymbol(const string &nm,ContextField *pate,
    /// VarnodeSymbol *v,uint4 l,uint4 h,bool flow)` with the varnode passed
    /// as its symbol id.
    pub fn new_context(
        nm: &[u8],
        pate: ContextField,
        v: u32,
        l: u32,
        h: u32,
        flow: bool,
    ) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Context(ContextSymbol {
                value: ValueSymbol {
                    patval: Some(PatternValue::ContextField(pate)),
                },
                vn: Some(v),
                low: l,
                high: h,
                flow,
            }),
        )
    }

    /// C++ `VarnodeListSymbol(const string &nm,PatternValue *pv,const
    /// vector<SleighSymbol *> &vt)` with varnodes passed as symbol ids.
    pub fn new_varnodelist(
        nm: &[u8],
        pv: PatternValue,
        vt: Vec<Option<u32>>,
    ) -> KunaResult<SleighSymbol> {
        let mut vl = VarnodeListSymbol {
            value: ValueSymbol { patval: Some(pv) },
            varnode_table: vt,
            tableisfilled: false,
        };
        vl.check_table_fill()?;
        Ok(SleighSymbol::new(nm, SymbolKind::VarnodeList(vl)))
    }

    /// C++ `OperandSymbol(const string &nm,int4 index,Constructor *ct)`
    /// with the constructor identified by reference.
    pub fn new_operand(nm: &[u8], index: i32, ct: ConstructorRef) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Operand(OperandSymbol {
                hand: index,
                localexp: Some(OperandValue::new(index, ct.table_id, ct.ct_id)),
                ..Default::default()
            }),
        )
    }

    /// C++ `StartSymbol(const string &nm,AddrSpace *cspc)`.
    pub fn new_start(nm: &[u8], cspc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Start(StartSymbol {
                const_space: Some(cspc),
            }),
        )
    }

    /// C++ `EndSymbol(const string &nm,AddrSpace *cspc)`.
    pub fn new_end(nm: &[u8], cspc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::End(EndSymbol {
                const_space: Some(cspc),
            }),
        )
    }

    /// C++ `Next2Symbol(const string &nm,AddrSpace *cspc)`.
    pub fn new_next2(nm: &[u8], cspc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::Next2(Next2Symbol {
                const_space: Some(cspc),
            }),
        )
    }

    /// C++ `FlowDestSymbol(const string &nm,AddrSpace *cspc)`.
    pub fn new_flowdest(nm: &[u8], cspc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::FlowDest(FlowDestSymbol {
                const_space: Some(cspc),
            }),
        )
    }

    /// C++ `FlowRefSymbol(const string &nm,AddrSpace *cspc)`.
    pub fn new_flowref(nm: &[u8], cspc: Rc<AddrSpace>) -> SleighSymbol {
        SleighSymbol::new(
            nm,
            SymbolKind::FlowRef(FlowRefSymbol {
                const_space: Some(cspc),
            }),
        )
    }

    /// C++ `SubtableSymbol(const string &nm)`.
    pub fn new_subtable(nm: &[u8]) -> SleighSymbol {
        SleighSymbol::new(nm, SymbolKind::Subtable(SubtableSymbol::default()))
    }

    /// C++ `SleighSymbol::getName`.
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    /// C++ `SleighSymbol::getId`.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// The scope id (C++ private `scopeid`, exposed for inspection).
    pub fn get_scope_id(&self) -> u32 {
        self.scopeid
    }

    /// The subclass payload.
    pub fn kind(&self) -> &SymbolKind {
        &self.kind
    }

    /// `&SubtableSymbol` when this symbol is a subtable (C++
    /// `dynamic_cast<SubtableSymbol*>`), else `None`.
    pub fn as_subtable(&self) -> Option<&SubtableSymbol> {
        match &self.kind {
            SymbolKind::Subtable(v) => Some(v),
            _ => None,
        }
    }

    /// Mutable subtable access (WS4b build side).
    pub fn as_subtable_mut(&mut self) -> Option<&mut SubtableSymbol> {
        match &mut self.kind {
            SymbolKind::Subtable(v) => Some(v),
            _ => None,
        }
    }

    /// Mutable access to the subclass payload (compiler-style setup such as
    /// `UserOpSymbol::setIndex` / `VarnodeSymbol::markAsContext`).
    pub fn kind_mut(&mut self) -> &mut SymbolKind {
        &mut self.kind
    }

    /// (WS4b purge/renumber) re-point every encoded symbol-id cross-reference
    /// inside this symbol's content through `remap` (`old_id -> Some(new_id)`,
    /// `None` if the referenced symbol was purged).  C++ keeps these as
    /// pointers (immune to renumber); the kuna port stores ids and must remap.
    fn remap_symbol_refs(&mut self, remap: &[Option<u32>]) {
        let map = |id: u32| remap.get(id as usize).copied().flatten().unwrap_or(id);
        match &mut self.kind {
            SymbolKind::Context(c) => {
                if let Some(vn) = c.vn {
                    c.vn = Some(map(vn));
                }
                remap_patval_table(c.value.patval.as_mut(), &map);
            }
            SymbolKind::VarnodeList(v) => {
                for e in v.varnode_table.iter_mut() {
                    if let Some(id) = e {
                        *e = Some(map(*id));
                    }
                }
                remap_patval_table(v.value.patval.as_mut(), &map);
            }
            SymbolKind::Operand(o) => {
                if let Some(t) = o.triple {
                    o.triple = Some(map(t));
                }
                if let Some(le) = o.localexp.as_mut() {
                    le.set_table_id(map(le.table_id()));
                }
            }
            SymbolKind::Value(v) => remap_patval_table(v.patval.as_mut(), &map),
            SymbolKind::ValueMap(v) => remap_patval_table(v.value.patval.as_mut(), &map),
            SymbolKind::Name(v) => remap_patval_table(v.value.patval.as_mut(), &map),
            SymbolKind::Subtable(st) => {
                for ct in st.construct.iter_mut() {
                    if let Some(p) = ct.parent {
                        ct.parent = Some(map(p));
                    }
                }
            }
            _ => {}
        }
    }

    /// C++ virtual `getType`.
    pub fn get_type(&self) -> SymbolType {
        match &self.kind {
            SymbolKind::Space(_) => SymbolType::Space,
            SymbolKind::Token(_) => SymbolType::Token,
            SymbolKind::UserOp(_) => SymbolType::UserOp,
            SymbolKind::Epsilon(_) => SymbolType::Epsilon,
            SymbolKind::Value(_) => SymbolType::Value,
            SymbolKind::ValueMap(_) => SymbolType::ValueMap,
            SymbolKind::Name(_) => SymbolType::Name,
            SymbolKind::Varnode(_) => SymbolType::Varnode,
            SymbolKind::Context(_) => SymbolType::Context,
            SymbolKind::VarnodeList(_) => SymbolType::VarnodeList,
            SymbolKind::Operand(_) => SymbolType::Operand,
            SymbolKind::Start(_) => SymbolType::Start,
            SymbolKind::End(_) => SymbolType::End,
            SymbolKind::Next2(_) => SymbolType::Next2,
            SymbolKind::FlowDest(_) => SymbolType::FlowDest,
            SymbolKind::FlowRef(_) => SymbolType::FlowRef,
            SymbolKind::Subtable(_) => SymbolType::Subtable,
        }
    }

    /// C++ `FamilySymbol::getPatternValue` for the value-family kinds;
    /// `None` for everything else (no such virtual in C++) and for an
    /// undecoded shell (C++ null).
    pub fn get_pattern_value(&self) -> Option<&PatternValue> {
        match &self.kind {
            SymbolKind::Value(v) => v.get_pattern_value(),
            SymbolKind::ValueMap(v) => v.value.get_pattern_value(),
            SymbolKind::Name(v) => v.value.get_pattern_value(),
            SymbolKind::Context(v) => v.value.get_pattern_value(),
            SymbolKind::VarnodeList(v) => v.value.get_pattern_value(),
            _ => None,
        }
    }

    /// C++ virtual `TripleSymbol::getPatternExpression`.  `Ok(None)`
    /// mirrors a C++ null return (the engine checks for it, see the
    /// `operand_value` seam docs in [`crate::slghpatexpress`]); FlowDest/
    /// FlowRef/Subtable throw exactly the C++ `SleighError`s.  Returns an
    /// owned expression (C++ shares an interior pointer).
    pub fn get_pattern_expression(&self) -> KunaResult<Option<PatternExpression>> {
        match &self.kind {
            // PatternlessSymbol: the constant-0 expression
            SymbolKind::Epsilon(_) | SymbolKind::Varnode(_) => Ok(Some(PatternExpression::Value(
                PatternValue::ConstantValue(ConstantValue::new(0)),
            ))),
            SymbolKind::Value(v) => Ok(v.patval.clone().map(PatternExpression::Value)),
            SymbolKind::ValueMap(v) => Ok(v.value.patval.clone().map(PatternExpression::Value)),
            SymbolKind::Name(v) => Ok(v.value.patval.clone().map(PatternExpression::Value)),
            SymbolKind::Context(v) => Ok(v.value.patval.clone().map(PatternExpression::Value)),
            SymbolKind::VarnodeList(v) => Ok(v.value.patval.clone().map(PatternExpression::Value)),
            SymbolKind::Operand(op) => Ok(op
                .localexp
                .clone()
                .map(|ov| PatternExpression::Value(PatternValue::OperandValue(ov)))),
            SymbolKind::Start(_) => Ok(Some(PatternExpression::Value(
                PatternValue::StartInstructionValue(StartInstructionValue),
            ))),
            SymbolKind::End(_) => Ok(Some(PatternExpression::Value(
                PatternValue::EndInstructionValue(EndInstructionValue),
            ))),
            SymbolKind::Next2(_) => Ok(Some(PatternExpression::Value(
                PatternValue::Next2InstructionValue(Next2InstructionValue),
            ))),
            SymbolKind::FlowDest(_) | SymbolKind::FlowRef(_) => {
                Err(KunaError::sleigh("Cannot use symbol in pattern"))
            }
            SymbolKind::Subtable(_) => Err(KunaError::sleigh("Cannot use subtable in expression")),
            // Not a TripleSymbol in C++ (unreachable through ported paths)
            SymbolKind::Space(_) | SymbolKind::Token(_) | SymbolKind::UserOp(_) => Err(
                KunaError::sleigh("symbol has no pattern expression (not a TripleSymbol in C++)"),
            ),
        }
    }

    /// C++ virtual `TripleSymbol::resolve(ParserWalker&)`: `Some(ct_id)`
    /// only for a subtable; the table-driven family symbols validate their
    /// index and return `None` (C++ null), everything else is the base
    /// no-op.
    pub fn resolve(&self, walker: &dyn SymbolWalker) -> KunaResult<Option<u32>> {
        match &self.kind {
            SymbolKind::ValueMap(v) => {
                v.resolve(walker)?;
                Ok(None)
            }
            SymbolKind::Name(v) => {
                v.resolve(walker)?;
                Ok(None)
            }
            SymbolKind::VarnodeList(v) => {
                v.resolve(walker)?;
                Ok(None)
            }
            SymbolKind::Subtable(v) => Ok(Some(v.resolve(walker)?)),
            _ => Ok(None), // TripleSymbol::resolve base: null
        }
    }

    /// C++ virtual `TripleSymbol::getSize` (size out of context).
    pub fn get_size(&self, table: &SymbolTable) -> KunaResult<i32> {
        match &self.kind {
            SymbolKind::Varnode(v) => Ok(v.get_size()),
            SymbolKind::VarnodeList(v) => v.get_size(&self.name, table),
            SymbolKind::Operand(op) => match op.triple {
                Some(tid) => table.symbol(tid)?.get_size(table),
                None => Ok(0),
            },
            SymbolKind::Subtable(_) => Ok(-1),
            _ => Ok(0), // TripleSymbol::getSize base
        }
    }

    /// C++ virtual `TripleSymbol::getFixedHandle(FixedHandle&,
    /// ParserWalker&)`.  Only the C++-assigned fields of `hand` are
    /// touched (callers reuse handle storage).
    pub fn get_fixed_handle(
        &self,
        hand: &mut FixedHandle,
        walker: &dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        match &self.kind {
            SymbolKind::Epsilon(e) => {
                // C++ reads the (decode-initialized) const_space field
                hand.space = Some(e.const_space.clone().ok_or_else(|| {
                    KunaError::sleigh("epsilon symbol has no constant space (not decoded)")
                })?);
                hand.offset_space = None; // Not a dynamic value
                hand.offset_offset = 0;
                hand.size = 0; // Cannot provide size
            }
            // NameSymbol/ContextSymbol inherit ValueSymbol::getFixedHandle
            SymbolKind::Value(_) | SymbolKind::Name(_) | SymbolKind::Context(_) => {
                let v = match &self.kind {
                    SymbolKind::Value(v) => v,
                    SymbolKind::Name(n) => &n.value,
                    SymbolKind::Context(c) => &c.value,
                    _ => unreachable!("outer match restricts the kind"),
                };
                let pw: &dyn PatternExpressionContext = walker;
                hand.space = Some(walker.get_const_space());
                hand.offset_space = None;
                // C++ `(uintb) patval->getValue(walker)`: intb -> uintb
                // two's-complement reinterpret
                hand.offset_offset = v.patval()?.get_value(pw)? as u64;
                hand.size = 0; // Cannot provide size
            }
            SymbolKind::ValueMap(v) => {
                let pw: &dyn PatternExpressionContext = walker;
                // C++ `(uint4) patval->getValue(walker)`: intb -> uint4
                // truncation
                let ind = v.value.patval()?.get_value(pw)? as u32;
                // The resolve routine has checked that -ind- must be a
                // valid index (C++ indexes unchecked: panic on violation)
                hand.space = Some(walker.get_const_space());
                hand.offset_space = None; // Not a dynamic value
                // C++ `(uintb)valuetable[ind]`: intb -> uintb reinterpret
                hand.offset_offset = v.valuetable[ind as usize] as u64;
                hand.size = 0; // Cannot provide size
            }
            SymbolKind::Varnode(v) => {
                hand.space = v.fix.space.clone();
                hand.offset_space = None; // Not a dynamic symbol
                hand.offset_offset = v.fix.offset;
                hand.size = v.fix.size;
            }
            SymbolKind::VarnodeList(v) => {
                let pw: &dyn PatternExpressionContext = walker;
                // C++ `(uint4) patval->getValue(walker)`: intb -> uint4
                // truncation
                let ind = v.value.patval()?.get_value(pw)? as u32;
                // The resolve routine has checked that -ind- must be a
                // valid index; C++ dereferences the table entry unchecked
                let vnid = v.varnode_table[ind as usize].ok_or_else(|| {
                    KunaError::sleigh("null varnode list entry (C++ dereferences unchecked)")
                })?;
                let fix = table.varnode_symbol(vnid)?.get_fixed_varnode();
                hand.space = fix.space.clone();
                hand.offset_space = None; // Not a dynamic value
                hand.offset_offset = fix.offset;
                hand.size = fix.size;
            }
            SymbolKind::Operand(op) => {
                // C++ `hnd = walker.getFixedHandle(hand)`: whole-struct copy
                *hand = walker.get_fixed_handle(op.hand)?;
            }
            SymbolKind::Start(_) => {
                let spc = walker.get_cur_space();
                hand.size = spc.get_addr_size();
                hand.space = Some(spc);
                hand.offset_space = None;
                hand.offset_offset = walker.get_addr().get_offset();
            }
            SymbolKind::End(_) => {
                let spc = walker.get_cur_space();
                hand.size = spc.get_addr_size();
                hand.space = Some(spc);
                hand.offset_space = None;
                hand.offset_offset = walker.get_naddr().get_offset();
            }
            SymbolKind::Next2(_) => {
                let spc = walker.get_cur_space();
                hand.size = spc.get_addr_size();
                hand.space = Some(spc);
                hand.offset_space = None;
                hand.offset_offset = walker.get_n2addr()?.get_offset();
            }
            SymbolKind::FlowDest(f) => {
                let ref_addr = walker.get_dest_addr()?;
                hand.space = Some(f.const_space.clone().ok_or_else(|| {
                    KunaError::sleigh("flowdest symbol has no constant space")
                })?);
                hand.offset_space = None;
                hand.offset_offset = ref_addr.get_offset();
                hand.size = ref_addr.get_addr_size() as u32; // C++ implicit int4 -> uint4
            }
            SymbolKind::FlowRef(f) => {
                let ref_addr = walker.get_ref_addr()?;
                hand.space = Some(f.const_space.clone().ok_or_else(|| {
                    KunaError::sleigh("flowref symbol has no constant space")
                })?);
                hand.offset_space = None;
                hand.offset_offset = ref_addr.get_offset();
                hand.size = ref_addr.get_addr_size() as u32; // C++ implicit int4 -> uint4
            }
            SymbolKind::Subtable(_) => {
                return Err(KunaError::sleigh("Cannot use subtable in expression"));
            }
            // Not a TripleSymbol in C++ (unreachable through ported paths)
            SymbolKind::Space(_) | SymbolKind::Token(_) | SymbolKind::UserOp(_) => {
                return Err(KunaError::sleigh(
                    "symbol has no fixed handle (not a TripleSymbol in C++)",
                ));
            }
        }
        Ok(())
    }

    /// C++ virtual `TripleSymbol::print(ostream&,ParserWalker&)`.
    pub fn print(
        &self,
        s: &mut String,
        walker: &mut dyn SymbolWalker,
        table: &SymbolTable,
    ) -> KunaResult<()> {
        match &self.kind {
            SymbolKind::Epsilon(_) => {
                s.push('0');
                Ok(())
            }
            SymbolKind::Value(v) => {
                let pw: &dyn PatternExpressionContext = &*walker;
                let val = v.patval()?.get_value(pw)?;
                push_hex_intb(s, val);
                Ok(())
            }
            SymbolKind::ValueMap(v) => {
                let pw: &dyn PatternExpressionContext = &*walker;
                // C++ `(uint4)patval->getValue(walker)`: intb -> uint4
                // truncation; ind already checked by the resolve routine
                // (C++ indexes unchecked: panic on violation)
                let ind = v.value.patval()?.get_value(pw)? as u32;
                let val = v.valuetable[ind as usize];
                push_hex_intb(s, val);
                Ok(())
            }
            SymbolKind::Name(v) => {
                let pw: &dyn PatternExpressionContext = &*walker;
                // C++ `(uint4)patval->getValue(walker)`: intb -> uint4
                // truncation; ind already checked by the resolve routine
                let ind = v.value.patval()?.get_value(pw)? as u32;
                s.push_str(&String::from_utf8_lossy(&v.nametable[ind as usize]));
                Ok(())
            }
            SymbolKind::Varnode(_) => {
                // C++ `s << getName()`
                s.push_str(&name_text(&self.name));
                Ok(())
            }
            SymbolKind::Context(v) => {
                // ContextSymbol inherits ValueSymbol::print
                let pw: &dyn PatternExpressionContext = &*walker;
                let val = v.value.patval()?.get_value(pw)?;
                push_hex_intb(s, val);
                Ok(())
            }
            SymbolKind::VarnodeList(v) => {
                let pw: &dyn PatternExpressionContext = &*walker;
                // C++ `(uint4)patval->getValue(walker)`: intb -> uint4
                let ind = v.value.patval()?.get_value(pw)? as u32;
                // C++ `if (ind >= varnode_table.size())`: uint4 vs size_t
                // (ind zero-extends)
                if u64::from(ind) >= v.varnode_table.len() as u64 {
                    return Err(KunaError::sleigh("Value out of range for varnode table"));
                }
                // C++ dereferences the entry unchecked (null would be UB)
                let vnid = v.varnode_table[ind as usize].ok_or_else(|| {
                    KunaError::sleigh("null varnode list entry (C++ dereferences unchecked)")
                })?;
                s.push_str(&name_text(table.symbol(vnid)?.get_name()));
                Ok(())
            }
            SymbolKind::Operand(op) => op.print(s, walker, table),
            SymbolKind::Start(_) => {
                // C++ `(intb) walker.getAddr().getOffset()` printed in hex
                push_hex_offset(s, walker.get_addr().get_offset());
                Ok(())
            }
            SymbolKind::End(_) => {
                push_hex_offset(s, walker.get_naddr().get_offset());
                Ok(())
            }
            SymbolKind::Next2(_) => {
                push_hex_offset(s, walker.get_n2addr()?.get_offset());
                Ok(())
            }
            SymbolKind::FlowDest(_) => {
                push_hex_offset(s, walker.get_dest_addr()?.get_offset());
                Ok(())
            }
            SymbolKind::FlowRef(_) => {
                push_hex_offset(s, walker.get_ref_addr()?.get_offset());
                Ok(())
            }
            SymbolKind::Subtable(_) => Err(KunaError::sleigh("Cannot use subtable in expression")),
            // Not a TripleSymbol in C++ (unreachable through ported paths)
            SymbolKind::Space(_) | SymbolKind::Token(_) | SymbolKind::UserOp(_) => Err(
                KunaError::sleigh("symbol is not printable (not a TripleSymbol in C++)"),
            ),
        }
    }

    /// C++ virtual `encodeHeader`: the 13 decodable kinds wrap the base
    /// attributes in their `*_head` element; other kinds reproduce the base
    /// class behavior (bare attributes, no element — C++ only ever reaches
    /// it through a subclass wrapper).
    pub fn encode_header(&self, encoder: &mut dyn Encoder) {
        let head = match &self.kind {
            SymbolKind::UserOp(_) => Some(&sla::ELEM_USEROP_HEAD),
            SymbolKind::Epsilon(_) => Some(&sla::ELEM_EPSILON_SYM_HEAD),
            SymbolKind::Value(_) => Some(&sla::ELEM_VALUE_SYM_HEAD),
            SymbolKind::ValueMap(_) => Some(&sla::ELEM_VALUEMAP_SYM_HEAD),
            SymbolKind::Name(_) => Some(&sla::ELEM_NAME_SYM_HEAD),
            SymbolKind::Varnode(_) => Some(&sla::ELEM_VARNODE_SYM_HEAD),
            SymbolKind::Context(_) => Some(&sla::ELEM_CONTEXT_SYM_HEAD),
            SymbolKind::VarnodeList(_) => Some(&sla::ELEM_VARLIST_SYM_HEAD),
            SymbolKind::Operand(_) => Some(&sla::ELEM_OPERAND_SYM_HEAD),
            SymbolKind::Start(_) => Some(&sla::ELEM_START_SYM_HEAD),
            SymbolKind::End(_) => Some(&sla::ELEM_END_SYM_HEAD),
            SymbolKind::Next2(_) => Some(&sla::ELEM_NEXT2_SYM_HEAD),
            SymbolKind::Subtable(_) => Some(&sla::ELEM_SUBTABLE_SYM_HEAD),
            _ => None,
        };
        if let Some(elem) = head {
            encoder.open_element(elem);
        }
        // SleighSymbol::encodeHeader: the basic attributes of a symbol
        encoder.write_string(&sla::ATTRIB_NAME, &self.name);
        encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
        encoder.write_unsigned_integer(&sla::ATTRIB_SCOPE, u64::from(self.scopeid));
        if let Some(elem) = head {
            encoder.close_element(elem);
        }
    }

    /// C++ `SleighSymbol::decodeHeader`.
    pub fn decode_header(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.open_element()?;
        self.name = decoder.read_string_id(&sla::ATTRIB_NAME)?;
        self.id = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32; // C++ implicit uintb -> uintm
        self.scopeid = decoder.read_unsigned_integer_id(&sla::ATTRIB_SCOPE)? as u32; // C++ implicit uintb -> uintm
        decoder.close_element(el)?;
        Ok(())
    }

    /// C++ virtual `encode` (symbol content).  Kinds without a content
    /// encode throw the base `LowlevelError`; `SubtableSymbol::encode` is
    /// silently skipped when not fully formed, as upstream.
    pub fn encode(&self, encoder: &mut dyn Encoder, trans: &dyn SleighBaseTrans) -> KunaResult<()> {
        match &self.kind {
            SymbolKind::UserOp(v) => {
                encoder.open_element(&sla::ELEM_USEROP);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                encoder.write_signed_integer(&sla::ATTRIB_INDEX, i64::from(v.index));
                encoder.close_element(&sla::ELEM_USEROP);
            }
            SymbolKind::Epsilon(_) => {
                encoder.open_element(&sla::ELEM_EPSILON_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                encoder.close_element(&sla::ELEM_EPSILON_SYM);
            }
            SymbolKind::Value(v) => {
                encoder.open_element(&sla::ELEM_VALUE_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                v.patval()?.encode(encoder);
                encoder.close_element(&sla::ELEM_VALUE_SYM);
            }
            SymbolKind::ValueMap(v) => {
                encoder.open_element(&sla::ELEM_VALUEMAP_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                v.value.patval()?.encode(encoder);
                for val in &v.valuetable {
                    encoder.open_element(&sla::ELEM_VALUETAB);
                    encoder.write_signed_integer(&sla::ATTRIB_VAL, *val);
                    encoder.close_element(&sla::ELEM_VALUETAB);
                }
                encoder.close_element(&sla::ELEM_VALUEMAP_SYM);
            }
            SymbolKind::Name(v) => {
                encoder.open_element(&sla::ELEM_NAME_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                v.value.patval()?.encode(encoder);
                for nm in &v.nametable {
                    encoder.open_element(&sla::ELEM_NAMETAB);
                    if nm == b"\t" {
                        // TAB indicates an illegal index: tag with no name
                        // attribute
                    } else {
                        encoder.write_string(&sla::ATTRIB_NAME, nm);
                    }
                    encoder.close_element(&sla::ELEM_NAMETAB);
                }
                encoder.close_element(&sla::ELEM_NAME_SYM);
            }
            SymbolKind::Varnode(v) => {
                encoder.open_element(&sla::ELEM_VARNODE_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                let spc = v.fix.space.as_ref().ok_or_else(|| {
                    KunaError::sleigh("varnode symbol has no space (C++ dereferences unchecked)")
                })?;
                encoder.write_space(&sla::ATTRIB_SPACE, spc);
                encoder.write_unsigned_integer(&sla::ATTRIB_OFF, v.fix.offset);
                encoder.write_signed_integer(&sla::ATTRIB_SIZE, i64::from(v.fix.size));
                encoder.close_element(&sla::ELEM_VARNODE_SYM);
            }
            SymbolKind::Context(v) => {
                encoder.open_element(&sla::ELEM_CONTEXT_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                // C++ writes vn->getId() (deref unchecked)
                let vn = v.vn.ok_or_else(|| {
                    KunaError::sleigh("context symbol has no varnode (C++ dereferences unchecked)")
                })?;
                encoder.write_unsigned_integer(&sla::ATTRIB_VARNODE, u64::from(vn));
                encoder.write_signed_integer(&sla::ATTRIB_LOW, i64::from(v.low));
                encoder.write_signed_integer(&sla::ATTRIB_HIGH, i64::from(v.high));
                encoder.write_bool(&sla::ATTRIB_FLOW, v.flow);
                v.value.patval()?.encode(encoder);
                encoder.close_element(&sla::ELEM_CONTEXT_SYM);
            }
            SymbolKind::VarnodeList(v) => {
                encoder.open_element(&sla::ELEM_VARLIST_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                v.value.patval()?.encode(encoder);
                for entry in &v.varnode_table {
                    match entry {
                        None => {
                            encoder.open_element(&sla::ELEM_NULL);
                            encoder.close_element(&sla::ELEM_NULL);
                        }
                        Some(vnid) => {
                            encoder.open_element(&sla::ELEM_VAR);
                            encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(*vnid));
                            encoder.close_element(&sla::ELEM_VAR);
                        }
                    }
                }
                encoder.close_element(&sla::ELEM_VARLIST_SYM);
            }
            SymbolKind::Operand(op) => {
                encoder.open_element(&sla::ELEM_OPERAND_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                if let Some(triple) = op.triple {
                    encoder.write_unsigned_integer(&sla::ATTRIB_SUBSYM, u64::from(triple));
                }
                // C++ writeSignedInteger(ATTRIB_OFF, reloffset): uint4 ->
                // intb zero-extends
                encoder.write_signed_integer(&sla::ATTRIB_OFF, i64::from(op.reloffset));
                encoder.write_signed_integer(&sla::ATTRIB_BASE, i64::from(op.offsetbase));
                encoder.write_signed_integer(&sla::ATTRIB_MINLEN, i64::from(op.minimumlength));
                if op.is_code_address() {
                    encoder.write_bool(&sla::ATTRIB_CODE, true);
                }
                encoder.write_signed_integer(&sla::ATTRIB_INDEX, i64::from(op.hand));
                op.localexp
                    .as_ref()
                    .ok_or_else(|| {
                        KunaError::sleigh("operand has no local expression (not decoded)")
                    })?
                    .encode(encoder);
                if let Some(defexp) = &op.defexp {
                    defexp.encode(encoder);
                }
                encoder.close_element(&sla::ELEM_OPERAND_SYM);
            }
            SymbolKind::Start(_) => {
                encoder.open_element(&sla::ELEM_START_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                encoder.close_element(&sla::ELEM_START_SYM);
            }
            SymbolKind::End(_) => {
                encoder.open_element(&sla::ELEM_END_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                encoder.close_element(&sla::ELEM_END_SYM);
            }
            SymbolKind::Next2(_) => {
                encoder.open_element(&sla::ELEM_NEXT2_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                encoder.close_element(&sla::ELEM_NEXT2_SYM);
            }
            SymbolKind::Subtable(v) => {
                if v.decisiontree.is_none() {
                    return Ok(()); // Not fully formed
                }
                encoder.open_element(&sla::ELEM_SUBTABLE_SYM);
                encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(self.id));
                // C++ writeSignedInteger(ATTRIB_NUMCT, construct.size()):
                // size_t -> intb
                encoder.write_signed_integer(&sla::ATTRIB_NUMCT, v.construct.len() as i64);
                for ct in &v.construct {
                    ct.encode(encoder, trans)?;
                }
                v.decisiontree
                    .as_ref()
                    .expect("checked above")
                    .encode(encoder);
                encoder.close_element(&sla::ELEM_SUBTABLE_SYM);
            }
            // SleighSymbol::encode base: not directly encodable
            SymbolKind::Space(_) | SymbolKind::Token(_) | SymbolKind::FlowDest(_)
            | SymbolKind::FlowRef(_) => {
                return Err(KunaError::lowlevel(format!(
                    "Symbol {} cannot be encoded to stream directly",
                    name_text(&self.name)
                )));
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// SymbolScope / SymbolTable
// ---------------------------------------------------------------------------

/// C++ `SymbolScope`: one scope of the symbol table.  The C++
/// `SymbolTree` (`set<SleighSymbol*,SymbolCompare>` ordered by name) is a
/// `BTreeMap<name, symbol id>` — same lexicographic byte order, same
/// reject-on-duplicate insert semantics.
#[derive(Debug, Clone)]
pub struct SymbolScope {
    /// Parent scope id (`None` is the C++ null parent of the root).
    parent: Option<u32>,
    /// Name-ordered symbol ids (C++ `SymbolTree`).
    tree: BTreeMap<Vec<u8>, u32>,
    /// Scope id (C++ `uintm`).
    id: u32,
}

impl SymbolScope {
    /// C++ `SymbolScope(SymbolScope *p,uintm i)` with the parent as an id.
    pub fn new(parent: Option<u32>, id: u32) -> SymbolScope {
        SymbolScope {
            parent,
            tree: BTreeMap::new(),
            id,
        }
    }

    /// C++ `SymbolScope::getParent` (the parent scope id).
    pub fn get_parent(&self) -> Option<u32> {
        self.parent
    }

    /// C++ `SymbolScope::addSymbol`: `set::insert` keeps the existing
    /// entry on a name collision and returns it; the resident symbol id is
    /// returned either way.
    pub fn add_symbol(&mut self, name: Vec<u8>, id: u32) -> u32 {
        *self.tree.entry(name).or_insert(id)
    }

    /// C++ `SymbolScope::findSymbol` (the symbol id, or `None` for the C++
    /// null).
    pub fn find_symbol(&self, nm: &[u8]) -> Option<u32> {
        self.tree.get(nm).copied()
    }

    /// C++ `SymbolScope::getId`.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// C++ `SymbolScope::removeSymbol` (by name, which is the tree key).
    pub fn remove_symbol(&mut self, nm: &[u8]) {
        self.tree.remove(nm);
    }

    /// C++ `begin()`/`end()` iteration: symbol ids in `SymbolCompare`
    /// (name) order.
    pub fn symbol_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.tree.values().copied()
    }

    /// Whether the scope holds no symbols (C++ `tree.empty()`).
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

/// C++ `SymbolTable`: all symbols (`symbollist`, indexed by symbol id) and
/// all scopes (`table`, indexed by scope id).  Compiler-only maintenance
/// (`purge`/`renumber`/`replaceSymbol`) is not ported (LOSS-001).
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbollist: Vec<Option<SleighSymbol>>,
    table: Vec<Option<SymbolScope>>,
    /// Current scope id (C++ `SymbolScope *curscope`, null = `None`).
    curscope: Option<u32>,
}

/// C++ `SymbolTable::MAX_TABLES`.
const MAX_TABLES: i64 = 0x0010_0000;
/// C++ `SymbolTable::MAX_SYMBOLS`.
const MAX_SYMBOLS: i64 = 0x0100_0000;

impl SymbolTable {
    /// C++ `SymbolTable(void)`.
    pub fn new() -> SymbolTable {
        SymbolTable::default()
    }

    /// C++ `SymbolTable::getCurrentScope` (as a scope id).
    pub fn get_current_scope(&self) -> Option<u32> {
        self.curscope
    }

    /// C++ `SymbolTable::getGlobalScope` (`table[0]`; `None` when the
    /// table is empty, where C++ would index out of bounds).
    pub fn get_global_scope(&self) -> Option<&SymbolScope> {
        self.table.first().and_then(|s| s.as_ref())
    }

    /// C++ `SymbolTable::setCurrentScope` (by scope id).
    pub fn set_current_scope(&mut self, scope: Option<u32>) {
        self.curscope = scope;
    }

    /// Access a scope by id.
    pub fn get_scope(&self, id: u32) -> Option<&SymbolScope> {
        self.table.get(id as usize).and_then(|s| s.as_ref())
    }

    /// Number of scope slots (C++ `table.size()`).
    pub fn num_scopes(&self) -> usize {
        self.table.len()
    }

    /// Number of symbol slots (C++ `symbollist.size()`).
    pub fn num_symbols(&self) -> usize {
        self.symbollist.len()
    }

    /// C++ `SymbolTable::addScope`: add new scope off of current scope,
    /// make it current.
    pub fn add_scope(&mut self) {
        // size_t -> uintm scope id as in C++ `SymbolScope(curscope,
        // table.size())`
        let id = self.table.len() as u32;
        let scope = SymbolScope::new(self.curscope, id);
        self.table.push(Some(scope));
        self.curscope = Some(id);
    }

    /// C++ `SymbolTable::popScope`: make parent of current scope current.
    pub fn pop_scope(&mut self) {
        if let Some(cur) = self.curscope {
            self.curscope = self.get_scope(cur).and_then(|s| s.get_parent());
        }
    }

    /// C++ `SymbolTable::skipScope(int4 i)`.
    fn skip_scope(&self, mut i: i32) -> Option<u32> {
        let mut res = self.curscope;
        while i > 0 {
            let scope = self.get_scope(res?)?;
            match scope.get_parent() {
                None => return res,
                Some(p) => res = Some(p),
            }
            i -= 1;
        }
        res
    }

    /// C++ `SymbolTable::addGlobalSymbol`.  Returns the assigned symbol id.
    pub fn add_global_symbol(&mut self, mut sym: SleighSymbol) -> KunaResult<u32> {
        // size_t -> uintm symbol id as in C++ `a->id = symbollist.size()`
        let id = self.symbollist.len() as u32;
        sym.id = id;
        let scope = self
            .table
            .first_mut()
            .and_then(|s| s.as_mut())
            .ok_or_else(|| KunaError::sleigh("symbol table has no global scope"))?;
        sym.scopeid = scope.get_id();
        let name = sym.name.clone();
        // C++ pushes onto symbollist before the duplicate check throws
        self.symbollist.push(Some(sym));
        let scope = self
            .table
            .first_mut()
            .and_then(|s| s.as_mut())
            .expect("checked above");
        let res = scope.add_symbol(name.clone(), id);
        if res != id {
            return Err(KunaError::sleigh(format!(
                "Duplicate symbol name '{}'",
                name_text(&name)
            )));
        }
        Ok(id)
    }

    /// C++ `SymbolTable::addSymbol` (into the current scope).  Returns the
    /// assigned symbol id.
    pub fn add_symbol(&mut self, mut sym: SleighSymbol) -> KunaResult<u32> {
        // size_t -> uintm symbol id as in C++
        let id = self.symbollist.len() as u32;
        sym.id = id;
        let curid = self
            .curscope
            .ok_or_else(|| KunaError::sleigh("symbol table has no current scope"))?;
        sym.scopeid = curid;
        let name = sym.name.clone();
        // C++ pushes onto symbollist before the duplicate check throws
        self.symbollist.push(Some(sym));
        let scope = self
            .table
            .get_mut(curid as usize)
            .and_then(|s| s.as_mut())
            .ok_or_else(|| KunaError::sleigh("current scope is undefined"))?;
        let res = scope.add_symbol(name.clone(), id);
        if res != id {
            return Err(KunaError::sleigh(format!(
                "Duplicate symbol name: {}",
                name_text(&name)
            )));
        }
        Ok(id)
    }

    /// C++ `SymbolTable::findSymbolInternal`.
    fn find_symbol_internal(&self, scope: Option<u32>, nm: &[u8]) -> Option<&SleighSymbol> {
        let mut scope = scope;
        while let Some(sid) = scope {
            let sc = self.get_scope(sid)?;
            if let Some(symid) = sc.find_symbol(nm) {
                return self.symbollist.get(symid as usize).and_then(|s| s.as_ref());
            }
            scope = sc.get_parent(); // Try higher scope
        }
        None
    }

    /// C++ `SymbolTable::findSymbol(const string&)` (current scope chain).
    pub fn find_symbol(&self, nm: &[u8]) -> Option<&SleighSymbol> {
        self.find_symbol_internal(self.curscope, nm)
    }

    /// C++ `SymbolTable::findSymbol(const string&,int4 skip)`.
    pub fn find_symbol_skip(&self, nm: &[u8], skip: i32) -> Option<&SleighSymbol> {
        self.find_symbol_internal(self.skip_scope(skip), nm)
    }

    /// C++ `SymbolTable::findGlobalSymbol`.
    pub fn find_global_symbol(&self, nm: &[u8]) -> Option<&SleighSymbol> {
        self.find_symbol_internal(Some(0), nm)
    }

    /// C++ `SymbolTable::findSymbol(uintm id)` — `symbollist[id]`, indexed
    /// unchecked in C++ (UB out of range); `None` here for both an
    /// out-of-range id and an undecoded slot.
    pub fn find_symbol_by_id(&self, id: u32) -> Option<&SleighSymbol> {
        self.symbollist.get(id as usize).and_then(|s| s.as_ref())
    }

    /// Mutable `findSymbol(uintm id)` for the WS4b build side (`setIndex`,
    /// `markAsContext`, `addConstructor`, operand mutators, ...).
    pub fn find_symbol_by_id_mut(&mut self, id: u32) -> Option<&mut SleighSymbol> {
        self.symbollist.get_mut(id as usize).and_then(|s| s.as_mut())
    }

    /// `findSymbol(id)` at sites where C++ dereferences the result
    /// immediately (an error instead of UB).
    fn symbol(&self, id: u32) -> KunaResult<&SleighSymbol> {
        self.find_symbol_by_id(id).ok_or_else(|| {
            KunaError::sleigh(format!(
                "symbol id {id} is undefined (C++ dereferences unchecked)"
            ))
        })
    }

    /// Resolve a symbol id that C++ blind-casts to `VarnodeSymbol*`.
    fn varnode_symbol(&self, id: u32) -> KunaResult<&VarnodeSymbol> {
        match &self.symbol(id)?.kind {
            SymbolKind::Varnode(v) => Ok(v),
            _ => Err(KunaError::sleigh(format!(
                "symbol id {id} is not a varnode symbol (C++ casts unchecked)"
            ))),
        }
    }

    /// Resolve a symbol id that C++ blind-casts to `OperandSymbol*`.
    fn operand_symbol(&self, id: u32) -> KunaResult<&OperandSymbol> {
        match &self.symbol(id)?.kind {
            SymbolKind::Operand(v) => Ok(v),
            _ => Err(KunaError::sleigh(format!(
                "symbol id {id} is not an operand symbol (C++ casts unchecked)"
            ))),
        }
    }

    /// Resolve a symbol id that C++ blind-casts to `SubtableSymbol*`.
    fn subtable_symbol(&self, id: u32) -> KunaResult<&SubtableSymbol> {
        match &self.symbol(id)?.kind {
            SymbolKind::Subtable(v) => Ok(v),
            _ => Err(KunaError::sleigh(format!(
                "symbol id {id} is not a subtable symbol (C++ casts unchecked)"
            ))),
        }
    }

    /// Look up a [`Constructor`] by reference (C++ navigates
    /// `SubtableSymbol*`/`Constructor*` pointers).
    pub fn get_constructor(&self, ctref: ConstructorRef) -> KunaResult<&Constructor> {
        self.subtable_symbol(ctref.table_id)?
            .get_constructor(ctref.ct_id)
    }

    /// C++ `OperandValue::isConstructorRelative` (slghpatexpress.cc:800):
    /// `ct->getOperand(index)->getOffsetBase() == -1`, resolved through
    /// this table (see the module docs of [`crate::slghpatexpress`]).
    pub fn operand_value_is_constructor_relative(&self, ov: &OperandValue) -> KunaResult<bool> {
        let ct = self.get_constructor(ConstructorRef {
            table_id: ov.table_id(),
            ct_id: ov.ct_id(),
        })?;
        let sym = self.operand_symbol(ct.get_operand(ov.index())?)?;
        Ok(sym.get_offset_base() == -1)
    }

    /// C++ `OperandValue::getName` (slghpatexpress.cc:807):
    /// `ct->getOperand(index)->getName()`, resolved through this table.
    pub fn operand_value_name(&self, ov: &OperandValue) -> KunaResult<&[u8]> {
        let ct = self.get_constructor(ConstructorRef {
            table_id: ov.table_id(),
            ct_id: ov.ct_id(),
        })?;
        let opid = ct.get_operand(ov.index())?;
        Ok(self.symbol(opid)?.get_name())
    }

    // -----------------------------------------------------------------
    // Build side (ws4a): constructor / subtable / decision-tree build
    // -----------------------------------------------------------------

    /// Mutable `SubtableSymbol` by id (build side; C++ casts unchecked).
    fn subtable_symbol_mut(&mut self, id: u32) -> KunaResult<&mut SubtableSymbol> {
        let sym = self
            .symbollist
            .get_mut(id as usize)
            .and_then(|s| s.as_mut())
            .ok_or_else(|| KunaError::sleigh(format!("symbol id {id} is undefined")))?;
        match &mut sym.kind {
            SymbolKind::Subtable(v) => Ok(v),
            _ => Err(KunaError::sleigh(format!(
                "symbol id {id} is not a subtable symbol (C++ casts unchecked)"
            ))),
        }
    }

    /// Mutable `OperandSymbol` by id (build side; C++ casts unchecked).
    fn operand_symbol_mut(&mut self, id: u32) -> KunaResult<&mut OperandSymbol> {
        let sym = self
            .symbollist
            .get_mut(id as usize)
            .and_then(|s| s.as_mut())
            .ok_or_else(|| KunaError::sleigh(format!("symbol id {id} is undefined")))?;
        match &mut sym.kind {
            SymbolKind::Operand(v) => Ok(v),
            _ => Err(KunaError::sleigh(format!(
                "symbol id {id} is not an operand symbol (C++ casts unchecked)"
            ))),
        }
    }

    /// C++ `SubtableSymbol::buildPattern(ostream &s)` driven from the symbol
    /// table.  Builds each constructor's pattern, folds the subtable pattern
    /// as the common subpattern, sets `beingbuilt`/`errors`.  Errors are
    /// accumulated into `errs` (the C++ `ostream &s`) rather than thrown so a
    /// faulty constructor does not abort the whole table — exactly as C++.
    pub fn build_subtable_pattern(
        &mut self,
        table_id: u32,
        arena: &EquationArena,
        errs: &mut Vec<String>,
    ) -> KunaResult<()> {
        // Already built?
        if self.subtable_symbol(table_id)?.pattern.is_some() {
            return Ok(());
        }
        {
            let sub = self.subtable_symbol_mut(table_id)?;
            sub.errors = false;
            sub.beingbuilt = true;
            // C++ sets `pattern = new TokenPattern()` then overwrites it.
            sub.pattern = Some(TokenPattern::new_true());
        }
        let numct = self.subtable_symbol(table_id)?.construct.len();
        if numct == 0 {
            errs.push(format!(
                "Error: There are no constructors in table: {}",
                name_text(self.symbol(table_id)?.get_name())
            ));
            self.subtable_symbol_mut(table_id)?.errors = true;
            return Ok(());
        }
        // First constructor seeds the pattern.
        match self.build_constructor_pattern(table_id, 0, arena, errs) {
            Ok(()) => {}
            Err(e) => {
                let info = self.constructor_info(table_id, 0)?;
                errs.push(format!("Error: {}: for {}", e.explain(), info));
                self.subtable_symbol_mut(table_id)?.errors = true;
            }
        }
        let mut acc = self
            .subtable_symbol(table_id)?
            .construct[0]
            .pattern
            .clone()
            .unwrap_or_else(TokenPattern::new_true);
        for i in 1..numct {
            match self.build_constructor_pattern(table_id, i as u32, arena, errs) {
                Ok(()) => {}
                Err(e) => {
                    let info = self.constructor_info(table_id, i as u32)?;
                    errs.push(format!("Error: {}: for {}", e.explain(), info));
                    self.subtable_symbol_mut(table_id)?.errors = true;
                }
            }
            let ctpat = self
                .subtable_symbol(table_id)?
                .construct[i]
                .pattern
                .clone()
                .unwrap_or_else(TokenPattern::new_true);
            // C++: *pattern = construct[i]->getPattern()->commonSubPattern(*pattern)
            acc = ctpat.common_sub_pattern(&acc)?;
        }
        let sub = self.subtable_symbol_mut(table_id)?;
        sub.pattern = Some(acc);
        sub.beingbuilt = false;
        Ok(())
    }

    /// C++ `Constructor::printInfo` text without the double-borrow (reads
    /// the parent name + lineno).
    fn constructor_info(&self, table_id: u32, ct_id: u32) -> KunaResult<String> {
        let ct = self.subtable_symbol(table_id)?.get_constructor(ct_id)?;
        let lineno = ct.get_lineno();
        Ok(format!(
            "table \"{}\" constructor starting at line {}",
            name_text(self.symbol(table_id)?.get_name()),
            lineno
        ))
    }

    /// C++ `Constructor::buildPattern(ostream &s)` driven from the symbol
    /// table.  Generates a [`TokenPattern`] per operand, runs the
    /// constructor's pattern equation, resolves operand offsets, validates
    /// context, and orders the operands.
    fn build_constructor_pattern(
        &mut self,
        table_id: u32,
        ct_id: u32,
        arena: &EquationArena,
        errs: &mut Vec<String>,
    ) -> KunaResult<()> {
        // Already built?
        if self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .pattern
            .is_some()
        {
            return Ok(());
        }
        let operand_ids: Vec<u32> = self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .operands
            .clone();
        let mut oppattern: Vec<TokenPattern> = Vec::with_capacity(operand_ids.len());
        let mut recursion = false;

        for &opid in &operand_ids {
            // Read the operand's defining symbol / expression.
            let (triple, defexp) = {
                let sym = self.operand_symbol(opid)?;
                (sym.get_defining_symbol(), sym.get_defining_expression().cloned())
            };
            let sympat: TokenPattern = if let Some(tripid) = triple {
                let is_subtable =
                    self.symbol(tripid)?.get_type() == SymbolType::Subtable;
                if is_subtable {
                    if self.subtable_symbol(tripid)?.is_being_built() {
                        // Detected recursion
                        if recursion {
                            return Err(KunaError::sleigh("Illegal recursion"));
                        }
                        recursion = true;
                        TokenPattern::new_true()
                    } else {
                        self.build_subtable_pattern(tripid, arena, errs)?;
                        self.subtable_symbol(tripid)?
                            .get_pattern()
                            .cloned()
                            .unwrap_or_else(TokenPattern::new_true)
                    }
                } else {
                    // triple->getPatternExpression()->genMinPattern(oppattern)
                    let patexp = self.symbol(tripid)?.get_pattern_expression()?;
                    match patexp {
                        Some(pe) => pe.gen_min_pattern(&oppattern),
                        None => TokenPattern::new_true(),
                    }
                }
            } else if let Some(pe) = defexp {
                pe.gen_min_pattern(&oppattern)
            } else {
                let nm = name_text(self.symbol(opid)?.get_name()).to_string();
                return Err(KunaError::sleigh(format!("{nm}: operand is undefined")));
            };

            let min_len = sympat.get_minimum_length();
            let var_len = sympat.get_left_ellipsis() || sympat.get_right_ellipsis();
            {
                let op = self.operand_symbol_mut(opid)?;
                op.set_minimum_length(min_len);
                if var_len {
                    op.set_variable_length();
                }
            }
            oppattern.push(sympat);
        }

        let pateq = self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .get_pattern_equation()
            .ok_or_else(|| KunaError::sleigh("Missing equation"))?;

        // Build the entire pattern
        let mut pattern = arena.gen_pattern(pateq, &oppattern)?;
        if pattern.always_false() {
            return Err(KunaError::sleigh("Impossible pattern"));
        }
        if recursion {
            pattern.set_right_ellipsis(true);
        }
        let minimumlength = pattern.get_minimum_length();
        {
            let ct = self.subtable_symbol_mut(table_id)?.construct_mut(ct_id)?;
            ct.pattern = Some(pattern);
            ct.minimumlength = minimumlength;
        }

        // Resolve offsets of the operands.
        let mut resolve = OperandResolve::new();
        {
            let mut sink = ConstructorOperandSink {
                table: self,
                operand_ids: &operand_ids,
            };
            if !arena.resolve_operand_left(pateq, &mut resolve, &oppattern, &mut sink)? {
                return Err(KunaError::sleigh("Unable to resolve operand offsets"));
            }
        }

        // Unravel relative offsets to absolute (if possible).
        for &opid in &operand_ids {
            let (irrel, mut base, mut offset) = {
                let op = self.operand_symbol(opid)?;
                (
                    op.is_offset_irrelevant(),
                    op.get_offset_base(),
                    op.get_relative_offset() as i32,
                )
            };
            if irrel {
                self.operand_symbol_mut(opid)?.set_offset(-1, 0);
                continue;
            }
            while base >= 0 {
                let baseid = operand_ids[base as usize];
                let bop = self.operand_symbol(baseid)?;
                if bop.is_variable_length() {
                    break; // Cannot resolve to absolute
                }
                let bbase = bop.get_offset_base();
                offset += bop.get_minimum_length();
                offset += bop.get_relative_offset() as i32;
                base = bbase;
                if base < 0 {
                    self.operand_symbol_mut(opid)?.set_offset(base, offset as u32);
                }
            }
        }

        // Make sure context expressions are valid.
        let context = self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .context
            .clone();
        for change in &context {
            change.validate(self)?;
        }

        // Order the operands based on offset dependency.
        self.order_operands(table_id, ct_id, arena)?;
        Ok(())
    }

    /// C++ `Constructor::orderOperands` driven from the symbol table.
    fn order_operands(
        &mut self,
        table_id: u32,
        ct_id: u32,
        arena: &EquationArena,
    ) -> KunaResult<()> {
        let operand_ids: Vec<u32> = self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .operands
            .clone();
        let n = operand_ids.len();
        let pateq = self
            .subtable_symbol(table_id)?
            .get_constructor(ct_id)?
            .get_pattern_equation()
            .ok_or_else(|| KunaError::sleigh("Missing equation"))?;

        // pateq->operandOrder(this, patternorder) — returns operand indices.
        let mut patternorder: Vec<i32> = Vec::new();
        {
            let mut seen = vec![false; n];
            arena.operand_order(pateq, &mut patternorder, &mut seen);
            // Marking mirrors C++ setMark during operandOrder; reflect it.
            for (i, &s) in seen.iter().enumerate() {
                if s {
                    self.operand_symbol_mut(operand_ids[i])?.set_mark();
                }
            }
        }
        // Make sure patternorder contains all operands (mark the rest).
        for i in 0..n {
            if !self.operand_symbol(operand_ids[i])?.is_marked() {
                patternorder.push(i as i32);
                self.operand_symbol_mut(operand_ids[i])?.set_mark();
            }
        }

        // Build the new operand order honoring offset dependencies.
        let mut neworder: Vec<i32> = Vec::new();
        loop {
            let lastsize = neworder.len();
            for &idx in &patternorder {
                let op = self.operand_symbol(operand_ids[idx as usize])?;
                if !op.is_marked() {
                    continue; // already in neworder
                }
                if op.is_offset_irrelevant() {
                    continue; // expression operands come last
                }
                let base = op.get_offset_base();
                let base_marked = base != -1
                    && self
                        .operand_symbol(operand_ids[base as usize])?
                        .is_marked();
                if base == -1 || !base_marked {
                    neworder.push(idx);
                    self.operand_symbol_mut(operand_ids[idx as usize])?.clear_mark();
                }
            }
            if neworder.len() == lastsize {
                break;
            }
        }
        // Tack on expression operands.
        for &idx in &patternorder {
            if self
                .operand_symbol(operand_ids[idx as usize])?
                .is_offset_irrelevant()
            {
                neworder.push(idx);
                self.operand_symbol_mut(operand_ids[idx as usize])?.clear_mark();
            }
        }

        if neworder.len() != n {
            return Err(KunaError::sleigh(
                "Circular offset dependency between operands",
            ));
        }

        // Fix up operand indices: newops[i]->hand = i; localexp->changeIndex(i)
        for (i, &idx) in neworder.iter().enumerate() {
            let op = self.operand_symbol_mut(operand_ids[idx as usize])?;
            op.set_hand(i as i32);
            op.change_local_index(i as i32);
        }
        // handmap[i] = operands[i]->hand  (original index -> new index)
        let mut handmap: Vec<i32> = Vec::with_capacity(n);
        for &opid in &operand_ids {
            handmap.push(self.operand_symbol(opid)?.get_index());
        }
        // Fix up offsetbase through the handmap.
        for &idx in &neworder {
            let opid = operand_ids[idx as usize];
            let base = self.operand_symbol(opid)?.get_offset_base();
            if base == -1 {
                continue;
            }
            let newbase = handmap[base as usize];
            self.operand_symbol_mut(opid)?.set_offset_base(newbase);
        }

        // Reorder the constructor's operand id vector + fix printpiece refs.
        let new_operand_ids: Vec<u32> = neworder.iter().map(|&idx| operand_ids[idx as usize]).collect();
        let ct = self.subtable_symbol_mut(table_id)?.construct_mut(ct_id)?;
        // Fix up printpiece operand refs through the handmap.
        for piece in ct.printpiece.iter_mut() {
            if piece.first() == Some(&b'\n') {
                let index = i32::from(piece[1]) - i32::from(b'A');
                let newindex = handmap[index as usize];
                piece[1] = (i32::from(b'A') + newindex) as u8;
            }
        }
        ct.operands = new_operand_ids;
        // Stash the handmap for WS4b: ConstructTpl handle-index fix-up
        // (templ->changeHandleIndex) is performed by WS4b's driver, which
        // owns the ConstructTpl arena (freeze-interface for WS4b).
        ct.handmap = handmap;
        Ok(())
    }

    /// C++ `SubtableSymbol::buildDecisionTree(DecisionProperties &props)`
    /// driven from the symbol table.
    pub fn build_decision_tree(
        &mut self,
        table_id: u32,
        props: &mut DecisionProperties,
    ) -> KunaResult<()> {
        // Pattern not fully formed?
        if self.subtable_symbol(table_id)?.pattern.is_none() {
            return Ok(());
        }
        let numct = self.subtable_symbol(table_id)?.construct.len();
        let mut tree = DecisionNode::new_root();
        for i in 0..numct {
            // C++ uses construct[i]->getPattern()->getPattern() (the inner
            // Pattern of the constructor's TokenPattern).
            let pat: Pattern = match self.subtable_symbol(table_id)?.construct[i].get_pattern() {
                Some(tp) => tp.get_pattern().clone(),
                None => continue,
            };
            let ndisjoint = pat.num_disjoint();
            if ndisjoint == 0 {
                let dp = expect_disjoint(&pat)?;
                tree.add_constructor_pair(&dp, i as u32);
            } else {
                for j in 0..ndisjoint {
                    let dp = pat.get_disjoint(j).ok_or_else(|| {
                        KunaError::sleigh("decision tree: missing disjoint pattern")
                    })?;
                    tree.add_constructor_pair(dp, i as u32);
                }
            }
        }
        tree.split(props);
        self.subtable_symbol_mut(table_id)?.decisiontree = Some(tree);
        Ok(())
    }

    /// C++ `SymbolTable::replaceSymbol(SleighSymbol *a, SleighSymbol *b)`
    /// (slghsymbol.cc:123).  Replace the symbol named like `b` (assumed equal
    /// to the existing symbol `a`'s name) in place, preserving `a`'s id and
    /// scopeid.  Used by the `attach*` directives to swap a `ValueSymbol` for
    /// a `ValueMapSymbol`/`NameSymbol`/`VarnodeListSymbol`.
    pub fn replace_symbol(&mut self, a_id: u32, mut b: SleighSymbol) -> KunaResult<()> {
        let (name, scopeid) = {
            let a = self.symbol(a_id)?;
            (a.name.clone(), a.scopeid)
        };
        // C++ walks scopes from the back looking for the symbol by name; the
        // resident scope is `a`'s scopeid (the tree keys by name).
        if let Some(scope) = self.table.get_mut(scopeid as usize).and_then(|s| s.as_mut()) {
            scope.remove_symbol(&name);
            b.id = a_id;
            b.scopeid = scopeid;
            scope.add_symbol(name, a_id);
        }
        self.symbollist[a_id as usize] = Some(b);
        Ok(())
    }

    /// C++ `SymbolTable::purge` (slghsymbol.cc:281): get rid of unsavable
    /// symbols and scopes, then [`renumber`](Self::renumber) so the saved
    /// stream has no id gaps.  In the global scope only
    /// space/token/epsilon/section/bitrange/macro/subtable survive; in any
    /// child scope only operands survive.  Removing a macro or an unreferenced
    /// subtable also removes its operand locals.
    pub fn purge(&mut self) {
        for i in 0..self.symbollist.len() {
            // Decide whether to drop slot i (and collect any operand locals to
            // also drop), without holding an outstanding borrow.
            let (drop_self, scopeid, name): (bool, u32, Vec<u8>) = {
                let sym = match self.symbollist[i].as_ref() {
                    Some(s) => s,
                    None => continue,
                };
                let scopeid = sym.scopeid;
                let name = sym.name.clone();
                let ty = sym.get_type();
                if scopeid != 0 {
                    // Not in global scope: keep only operands.
                    if ty == SymbolType::Operand {
                        continue;
                    }
                    (true, scopeid, name)
                } else {
                    match ty {
                        SymbolType::Space
                        | SymbolType::Token
                        | SymbolType::Epsilon
                        | SymbolType::Section
                        | SymbolType::Bitrange => (true, scopeid, name),
                        SymbolType::Macro => {
                            // Macro symbols themselves are removed, plus their
                            // operand locals.  (Macro operand locals are not
                            // tracked here as a separate kind class — the
                            // MacroSymbol payload is not yet ported; when it
                            // is, drop its operands here.)
                            (true, scopeid, name)
                        }
                        SymbolType::Subtable => {
                            // Drop only an *unused* subtable (no built pattern),
                            // along with its constructors' operand locals.
                            let keep = self
                                .symbollist[i]
                                .as_ref()
                                .and_then(|s| s.as_subtable())
                                .map(|st| st.get_pattern().is_some())
                                .unwrap_or(true);
                            if keep {
                                continue;
                            }
                            // Collect this subtable's operand locals and drop
                            // them too.
                            let mut to_drop: Vec<(u32, u32, Vec<u8>)> = Vec::new();
                            if let Some(st) = self.symbollist[i].as_ref().and_then(|s| s.as_subtable()) {
                                for con in &st.construct {
                                    for &opid in &con.operands {
                                        if let Some(op) = self.symbollist.get(opid as usize).and_then(|s| s.as_ref()) {
                                            to_drop.push((opid, op.scopeid, op.name.clone()));
                                        }
                                    }
                                }
                            }
                            for (opid, opscope, opname) in to_drop {
                                if let Some(sc) = self.table.get_mut(opscope as usize).and_then(|s| s.as_mut()) {
                                    sc.remove_symbol(&opname);
                                }
                                self.symbollist[opid as usize] = None;
                            }
                            (true, scopeid, name)
                        }
                        _ => continue, // keep everything else? No: C++ default => continue (skip removal)
                    }
                }
            };
            if drop_self {
                if let Some(sc) = self.table.get_mut(scopeid as usize).and_then(|s| s.as_mut()) {
                    sc.remove_symbol(&name);
                }
                self.symbollist[i] = None;
            }
        }
        // Remove any empty scopes (except the global scope 0).
        for i in 1..self.table.len() {
            let empty = self.table[i].as_ref().map(|s| s.is_empty()).unwrap_or(false);
            if empty {
                self.table[i] = None;
            }
        }
        self.renumber();
    }

    /// C++ `SymbolTable::renumber` (slghsymbol.cc): compact `table` and
    /// `symbollist` so there are no id gaps, fixing up each survivor's
    /// `id`/`scopeid`.  Cross-references between symbols are by-pointer in
    /// C++ (immune to renumber); in the kuna port the by-id references that
    /// matter for the saved stream are the pattern/decision-tree data (already
    /// built into numeric form) and the scope parent ids (remapped here).
    fn renumber(&mut self) {
        // First renumber the scopes: new scope id = position in the compacted
        // table.  Build old-id -> new-id map.
        let mut scope_remap: Vec<Option<u32>> = vec![None; self.table.len()];
        let mut newtable: Vec<Option<SymbolScope>> = Vec::new();
        for i in 0..self.table.len() {
            if self.table[i].is_some() {
                let newid = newtable.len() as u32;
                scope_remap[i] = Some(newid);
                let mut scope = self.table[i].take().unwrap();
                scope.id = newid;
                newtable.push(Some(scope));
            }
        }
        // Fix parent ids through the remap (C++ keeps the pointer, whose id was
        // updated in place; we remap the stored parent id).
        for slot in newtable.iter_mut() {
            if let Some(scope) = slot.as_mut() {
                if let Some(p) = scope.parent {
                    scope.parent = scope_remap.get(p as usize).copied().flatten();
                }
            }
        }
        // Build the symbol-id remap (old id -> new id) first, so cross-refs
        // inside symbol contents can be rewritten.
        let mut sym_remap: Vec<Option<u32>> = vec![None; self.symbollist.len()];
        {
            let mut next = 0u32;
            for (i, slot) in self.symbollist.iter().enumerate() {
                if slot.is_some() {
                    sym_remap[i] = Some(next);
                    next += 1;
                }
            }
        }
        // Now renumber the symbols.
        let mut newsymbol: Vec<Option<SleighSymbol>> = Vec::new();
        for i in 0..self.symbollist.len() {
            if self.symbollist[i].is_some() {
                let newid = newsymbol.len() as u32;
                let mut sym = self.symbollist[i].take().unwrap();
                sym.scopeid = scope_remap
                    .get(sym.scopeid as usize)
                    .copied()
                    .flatten()
                    .unwrap_or(0);
                sym.id = newid;
                // Re-point every encoded symbol-id cross-reference.
                sym.remap_symbol_refs(&sym_remap);
                newsymbol.push(Some(sym));
            }
        }
        self.table = newtable;
        self.symbollist = newsymbol;
        // C++ leaves curscope pointing at table[0] implicitly via later use;
        // after purge the table is only read for encode.
        self.curscope = self.table.first().and_then(|s| s.as_ref()).map(|s| s.get_id());
    }

    /// C++ `SymbolTable::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder, trans: &dyn SleighBaseTrans) -> KunaResult<()> {
        encoder.open_element(&sla::ELEM_SYMBOL_TABLE);
        // size_t -> intb counts as in C++ writeSignedInteger(..., size())
        encoder.write_signed_integer(&sla::ATTRIB_SCOPESIZE, self.table.len() as i64);
        encoder.write_signed_integer(&sla::ATTRIB_SYMBOLSIZE, self.symbollist.len() as i64);
        for scope in &self.table {
            // C++ dereferences table[i] unconditionally (null after a
            // purge would be UB)
            let scope = scope
                .as_ref()
                .ok_or_else(|| KunaError::sleigh("undefined scope (C++ dereferences unchecked)"))?;
            encoder.open_element(&sla::ELEM_SCOPE);
            encoder.write_unsigned_integer(&sla::ATTRIB_ID, u64::from(scope.get_id()));
            let parent = match scope.get_parent() {
                None => 0,
                Some(p) => u64::from(p),
            };
            encoder.write_unsigned_integer(&sla::ATTRIB_PARENT, parent);
            encoder.close_element(&sla::ELEM_SCOPE);
        }
        // First save the headers
        for sym in &self.symbollist {
            let sym = sym.as_ref().ok_or_else(|| {
                KunaError::sleigh("undefined symbol (C++ dereferences unchecked)")
            })?;
            sym.encode_header(encoder);
        }
        // Now save the content of each symbol (must save IN ORDER)
        for sym in &self.symbollist {
            let sym = sym.as_ref().expect("checked in header loop");
            sym.encode(encoder, trans)?;
        }
        encoder.close_element(&sla::ELEM_SYMBOL_TABLE);
        Ok(())
    }

    /// C++ `SymbolTable::decode`: scopes, then the two-pass symbol restore
    /// (headers, then contents in stream order).
    pub fn decode(
        &mut self,
        decoder: &mut dyn Decoder,
        trans: &mut dyn SleighBaseTrans,
    ) -> KunaResult<()> {
        let el = decoder.open_element_id(&sla::ELEM_SYMBOL_TABLE)?;
        let table_size = decoder.read_signed_integer_id(&sla::ATTRIB_SCOPESIZE)?;
        let symbol_size = decoder.read_signed_integer_id(&sla::ATTRIB_SYMBOLSIZE)?;
        if table_size < 0 || symbol_size < 0 {
            return Err(KunaError::sleigh("Bad symbol table size"));
        }
        if table_size > MAX_TABLES {
            return Err(KunaError::sleigh("Maximum scopes exceeded"));
        }
        if symbol_size > MAX_SYMBOLS {
            return Err(KunaError::sleigh("Maximum symbols exceeded"));
        }
        self.table.resize_with(table_size as usize, || None); // bounded by MAX_TABLES
        self.symbollist.resize_with(symbol_size as usize, || None); // bounded by MAX_SYMBOLS
        for _ in 0..self.table.len() {
            // Decode the scopes
            let subel = decoder.open_element_id(&sla::ELEM_SCOPE)?;
            let id = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32; // C++ implicit uintb -> uintm
            // C++ `id >= table.size()`: uintm vs size_t (id zero-extends)
            if u64::from(id) >= self.table.len() as u64 {
                return Err(KunaError::sleigh(
                    "Bad symbol scope id: exceeds symbol scope table size",
                ));
            }
            let parent = decoder.read_unsigned_integer_id(&sla::ATTRIB_PARENT)? as u32; // C++ implicit uintb -> uintm
            if u64::from(parent) >= self.table.len() as u64 {
                return Err(KunaError::sleigh(
                    "Bad symbol scope parent id: exceeds symbol scope table size",
                ));
            }
            // C++ stores the table[parent] POINTER at this moment: a
            // not-yet-decoded parent slot is captured as null and stays
            // null, replicated via the is_some() check.
            let parscope = if parent == id {
                None
            } else if self.table[parent as usize].is_some() {
                Some(parent)
            } else {
                None
            };
            if self.table[id as usize].is_some() {
                // (C++ message quirk: complains about the parent id)
                return Err(KunaError::sleigh("Bad symbol scope parent id: not unique"));
            }
            self.table[id as usize] = Some(SymbolScope::new(parscope, id));
            decoder.close_element(subel)?;
        }
        // C++ `curscope = table[0]` (empty table would be an OOB read; the
        // null-slot case maps to None)
        self.curscope = match self.table.first() {
            Some(Some(scope)) => Some(scope.get_id()),
            _ => None,
        };
        // Now decode the symbol shells
        for _ in 0..self.symbollist.len() {
            self.decode_symbol_header(decoder)?;
        }
        // Now decode the symbol content
        while decoder.peek_element()? != 0 {
            decoder.open_element()?;
            let id = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32; // C++ implicit uintb -> uintm
            // C++ findSymbol(id)->decode(...): unchecked symbollist[id]
            self.decode_symbol_content(decoder, trans, id)?;
            // Tag closed by the per-kind decode
        }
        decoder.close_element(el)?;
        Ok(())
    }

    /// C++ `SymbolTable::decodeSymbolHeader`: put the shell of a symbol in
    /// the symbol table in order to allow recursion.
    pub fn decode_symbol_header(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.peek_element()?;
        let kind = if el == sla::ELEM_USEROP_HEAD {
            SymbolKind::UserOp(UserOpSymbol::default())
        } else if el == sla::ELEM_EPSILON_SYM_HEAD {
            SymbolKind::Epsilon(EpsilonSymbol::default())
        } else if el == sla::ELEM_VALUE_SYM_HEAD {
            SymbolKind::Value(ValueSymbol::default())
        } else if el == sla::ELEM_VALUEMAP_SYM_HEAD {
            SymbolKind::ValueMap(ValueMapSymbol::default())
        } else if el == sla::ELEM_NAME_SYM_HEAD {
            SymbolKind::Name(NameSymbol::default())
        } else if el == sla::ELEM_VARNODE_SYM_HEAD {
            SymbolKind::Varnode(VarnodeSymbol::default())
        } else if el == sla::ELEM_CONTEXT_SYM_HEAD {
            SymbolKind::Context(ContextSymbol::default())
        } else if el == sla::ELEM_VARLIST_SYM_HEAD {
            SymbolKind::VarnodeList(VarnodeListSymbol::default())
        } else if el == sla::ELEM_OPERAND_SYM_HEAD {
            SymbolKind::Operand(OperandSymbol::default())
        } else if el == sla::ELEM_START_SYM_HEAD {
            SymbolKind::Start(StartSymbol::default())
        } else if el == sla::ELEM_END_SYM_HEAD {
            SymbolKind::End(EndSymbol::default())
        } else if el == sla::ELEM_NEXT2_SYM_HEAD {
            SymbolKind::Next2(Next2Symbol::default())
        } else if el == sla::ELEM_SUBTABLE_SYM_HEAD {
            SymbolKind::Subtable(SubtableSymbol::default())
        } else {
            return Err(KunaError::sleigh("Bad symbol xml"));
        };
        let mut sym = SleighSymbol::new(b"", kind);
        sym.decode_header(decoder)?; // Restore basic elements of symbol
        // C++ `sym->id >= symbollist.size()`: uintm vs size_t (zero-extends)
        if u64::from(sym.id) >= self.symbollist.len() as u64 {
            return Err(KunaError::sleigh("Bad symbol id: exceeds symbollist size"));
        }
        if self.symbollist[sym.id as usize].is_some() {
            return Err(KunaError::sleigh("Bad symbol id: not unique"));
        }
        // C++ `sym->scopeid >= table.size()`: uintm vs size_t
        if u64::from(sym.scopeid) >= self.table.len() as u64 {
            return Err(KunaError::sleigh("Bad symbol scope id: too large"));
        }
        if self.table[sym.scopeid as usize].is_none() {
            return Err(KunaError::sleigh("Bad symbol scope id: undefined"));
        }
        let id = sym.id;
        let scopeid = sym.scopeid;
        let name = sym.name.clone();
        self.symbollist[id as usize] = Some(sym); // Put the basic symbol in the table
        self.table[scopeid as usize]
            .as_mut()
            .expect("checked above")
            .add_symbol(name, id); // to allow recursion
        Ok(())
    }

    /// The per-kind content decode dispatch (the C++ virtual
    /// `sym->decode(decoder,trans)` call in `SymbolTable::decode`).  The
    /// caller has already opened the content element and read `ATTRIB_ID`.
    fn decode_symbol_content(
        &mut self,
        decoder: &mut dyn Decoder,
        trans: &mut dyn SleighBaseTrans,
        id: u32,
    ) -> KunaResult<()> {
        let sym_type = self
            .find_symbol_by_id(id)
            .ok_or_else(|| {
                KunaError::decoder(format!(
                    "symbol content references undefined id {id} (C++ dereferences unchecked)"
                ))
            })?
            .get_type();
        match sym_type {
            SymbolType::UserOp => {
                // C++ UserOpSymbol::decode
                let index = decoder.read_signed_integer_id(&sla::ATTRIB_INDEX)? as u32; // C++ implicit intb -> uint4
                decoder.close_element(sla::ELEM_USEROP.get_id())?;
                if let SymbolKind::UserOp(v) = &mut self.kind_of_mut(id) {
                    v.index = index;
                }
            }
            SymbolType::Epsilon => {
                // C++ EpsilonSymbol::decode
                let cspace = trans.get_constant_space();
                decoder.close_element(sla::ELEM_EPSILON_SYM.get_id())?;
                if let SymbolKind::Epsilon(v) = &mut self.kind_of_mut(id) {
                    v.const_space = Some(cspace);
                }
            }
            SymbolType::Value => {
                // C++ ValueSymbol::decode
                if let SymbolKind::Value(v) = self.kind_of(id) {
                    if v.patval.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let patval = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    decode_pattern_value(decoder, &resolver)?
                };
                decoder.close_element(sla::ELEM_VALUE_SYM.get_id())?;
                if let SymbolKind::Value(v) = &mut self.kind_of_mut(id) {
                    v.patval = Some(patval);
                }
            }
            SymbolType::ValueMap => {
                // C++ ValueMapSymbol::decode
                if let SymbolKind::ValueMap(v) = self.kind_of(id) {
                    if v.value.patval.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let (patval, valuetable) = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    let patval = decode_pattern_value(decoder, &resolver)?;
                    let mut valuetable: Vec<i64> = Vec::new();
                    while decoder.peek_element()? != 0 {
                        let subel = decoder.open_element()?;
                        let val = decoder.read_signed_integer_id(&sla::ATTRIB_VAL)?;
                        valuetable.push(val);
                        decoder.close_element(subel)?;
                    }
                    (patval, valuetable)
                };
                decoder.close_element(sla::ELEM_VALUEMAP_SYM.get_id())?;
                if let SymbolKind::ValueMap(v) = &mut self.kind_of_mut(id) {
                    v.value.patval = Some(patval);
                    v.valuetable = valuetable;
                    v.check_table_fill()?;
                }
            }
            SymbolType::Name => {
                // C++ NameSymbol::decode
                if let SymbolKind::Name(v) = self.kind_of(id) {
                    if v.value.patval.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let (patval, nametable) = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    let patval = decode_pattern_value(decoder, &resolver)?;
                    let mut nametable: Vec<Vec<u8>> = Vec::new();
                    while decoder.peek_element()? != 0 {
                        let subel = decoder.open_element()?;
                        if decoder.get_next_attribute_id()? == sla::ATTRIB_NAME.get_id() {
                            nametable.push(decoder.read_string()?);
                        } else {
                            nametable.push(b"\t".to_vec()); // TAB indicates an illegal index
                        }
                        decoder.close_element(subel)?;
                    }
                    (patval, nametable)
                };
                decoder.close_element(sla::ELEM_NAME_SYM.get_id())?;
                if let SymbolKind::Name(v) = &mut self.kind_of_mut(id) {
                    v.value.patval = Some(patval);
                    v.nametable = nametable;
                    v.check_table_fill()?;
                }
            }
            SymbolType::Varnode => {
                // C++ VarnodeSymbol::decode
                let space = decoder.read_space_id(&sla::ATTRIB_SPACE)?;
                let offset = decoder.read_unsigned_integer_id(&sla::ATTRIB_OFF)?;
                let size = decoder.read_signed_integer_id(&sla::ATTRIB_SIZE)? as u32; // C++ implicit intb -> uint4
                // PatternlessSymbol does not need restoring
                decoder.close_element(sla::ELEM_VARNODE_SYM.get_id())?;
                if let SymbolKind::Varnode(v) = &mut self.kind_of_mut(id) {
                    v.fix = VarnodeData {
                        space: Some(space),
                        offset,
                        size,
                    };
                }
            }
            SymbolType::Context => {
                // C++ ContextSymbol::decode (header already filled in)
                let mut flow = false;
                let mut high_missing = true;
                let mut low_missing = true;
                let mut vn: Option<u32> = None;
                let mut low: u32 = 0;
                let mut high: u32 = 0;
                let mut attrib = decoder.get_next_attribute_id()?;
                while attrib != 0 {
                    if attrib == sla::ATTRIB_VARNODE {
                        // C++ (VarnodeSymbol*)trans->findSymbol(id): stored
                        // as the id (uintb -> uintm)
                        vn = Some(decoder.read_unsigned_integer()? as u32);
                    } else if attrib == sla::ATTRIB_LOW {
                        low = decoder.read_signed_integer()? as u32; // C++ implicit intb -> uint4
                        low_missing = false;
                    } else if attrib == sla::ATTRIB_HIGH {
                        high = decoder.read_signed_integer()? as u32; // C++ implicit intb -> uint4
                        high_missing = false;
                    } else if attrib == sla::ATTRIB_FLOW {
                        flow = decoder.read_bool()?;
                    }
                    attrib = decoder.get_next_attribute_id()?;
                }
                if low_missing || high_missing {
                    return Err(KunaError::decoder("Missing high/low attributes"));
                }
                if let SymbolKind::Context(v) = self.kind_of(id) {
                    if v.value.patval.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let patval = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    decode_pattern_value(decoder, &resolver)?
                };
                decoder.close_element(sla::ELEM_CONTEXT_SYM.get_id())?;
                if let SymbolKind::Context(v) = &mut self.kind_of_mut(id) {
                    v.value.patval = Some(patval);
                    v.vn = vn;
                    v.low = low;
                    v.high = high;
                    v.flow = flow;
                }
            }
            SymbolType::VarnodeList => {
                // C++ VarnodeListSymbol::decode
                if let SymbolKind::VarnodeList(v) = self.kind_of(id) {
                    if v.value.patval.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let (patval, varnode_table) = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    let patval = decode_pattern_value(decoder, &resolver)?;
                    let mut varnode_table: Vec<Option<u32>> = Vec::new();
                    while decoder.peek_element()? != 0 {
                        let subel = decoder.open_element()?;
                        if subel == sla::ELEM_VAR {
                            // C++ (VarnodeSymbol*)trans->findSymbol(id):
                            // stored as the id (uintb -> uintm)
                            let vnid = decoder.read_unsigned_integer_id(&sla::ATTRIB_ID)? as u32;
                            varnode_table.push(Some(vnid));
                        } else {
                            varnode_table.push(None);
                        }
                        decoder.close_element(subel)?;
                    }
                    (patval, varnode_table)
                };
                decoder.close_element(sla::ELEM_VARLIST_SYM.get_id())?;
                if let SymbolKind::VarnodeList(v) = &mut self.kind_of_mut(id) {
                    v.value.patval = Some(patval);
                    v.varnode_table = varnode_table;
                    v.check_table_fill()?;
                }
            }
            SymbolType::Operand => {
                // C++ OperandSymbol::decode
                if let SymbolKind::Operand(v) = self.kind_of(id) {
                    if v.defexp.is_some() || v.localexp.is_some() {
                        return Err(KunaError::decoder("Already decoded symbol"));
                    }
                }
                let mut reloffset: u32 = 0;
                let mut offsetbase: i32 = 0;
                let mut minimumlength: i32 = 0;
                let mut hand: i32 = 0;
                let mut triple: Option<u32> = None;
                let mut flags: u32 = 0;
                // C++ reads getNextAttributeId at the TOP of the loop body,
                // so the first peeked attribute (ATTRIB_ID, already consumed
                // by SymbolTable::decode) only feeds the loop condition.
                let mut attrib = decoder.get_next_attribute_id()?;
                while attrib != 0 {
                    attrib = decoder.get_next_attribute_id()?;
                    if attrib == sla::ATTRIB_INDEX {
                        hand = decoder.read_signed_integer()? as i32; // C++ implicit intb -> int4
                    } else if attrib == sla::ATTRIB_OFF {
                        reloffset = decoder.read_signed_integer()? as u32; // C++ implicit intb -> uint4
                    } else if attrib == sla::ATTRIB_BASE {
                        offsetbase = decoder.read_signed_integer()? as i32; // C++ implicit intb -> int4
                    } else if attrib == sla::ATTRIB_MINLEN {
                        minimumlength = decoder.read_signed_integer()? as i32; // C++ implicit intb -> int4
                    } else if attrib == sla::ATTRIB_SUBSYM {
                        // C++ (TripleSymbol*)trans->findSymbol(id): stored
                        // as the id (uintb -> uintm)
                        triple = Some(decoder.read_unsigned_integer()? as u32);
                    } else if attrib == sla::ATTRIB_CODE && decoder.read_bool()? {
                        flags |= OperandSymbol::CODE_ADDRESS;
                    }
                }
                let (localexp, defexp) = {
                    let resolver = TableResolver {
                        table: self,
                        current: None,
                    };
                    // C++ (OperandValue*)decodeExpression(...): unchecked
                    // cast (UB on a non-operand expression); an error here
                    let localexp =
                        match PatternExpression::decode_expression(decoder, &resolver)? {
                            PatternExpression::Value(PatternValue::OperandValue(ov)) => ov,
                            _ => {
                                return Err(KunaError::decoder(
                                    "Expecting operand expression (C++ casts unchecked)",
                                ))
                            }
                        };
                    let defexp = if decoder.peek_element()? != 0 {
                        Some(PatternExpression::decode_expression(decoder, &resolver)?)
                    } else {
                        None
                    };
                    (localexp, defexp)
                };
                decoder.close_element(sla::ELEM_OPERAND_SYM.get_id())?;
                if let SymbolKind::Operand(v) = &mut self.kind_of_mut(id) {
                    v.reloffset = reloffset;
                    v.offsetbase = offsetbase;
                    v.minimumlength = minimumlength;
                    v.hand = hand;
                    v.localexp = Some(localexp);
                    v.triple = triple;
                    v.defexp = defexp;
                    v.flags = flags;
                }
            }
            SymbolType::Start => {
                // C++ StartSymbol::decode
                let cspace = trans.get_constant_space();
                decoder.close_element(sla::ELEM_START_SYM.get_id())?;
                if let SymbolKind::Start(v) = &mut self.kind_of_mut(id) {
                    v.const_space = Some(cspace);
                }
            }
            SymbolType::End => {
                // C++ EndSymbol::decode
                let cspace = trans.get_constant_space();
                decoder.close_element(sla::ELEM_END_SYM.get_id())?;
                if let SymbolKind::End(v) = &mut self.kind_of_mut(id) {
                    v.const_space = Some(cspace);
                }
            }
            SymbolType::Next2 => {
                // C++ Next2Symbol::decode
                let cspace = trans.get_constant_space();
                decoder.close_element(sla::ELEM_NEXT2_SYM.get_id())?;
                if let SymbolKind::Next2(v) = &mut self.kind_of_mut(id) {
                    v.const_space = Some(cspace);
                }
            }
            SymbolType::Subtable => {
                // C++ SubtableSymbol::decode
                let (construct, decisiontree) = decode_subtable_content(decoder, self, trans, id)?;
                if let SymbolKind::Subtable(v) = &mut self.kind_of_mut(id) {
                    v.construct = construct;
                    v.decisiontree = decisiontree;
                }
            }
            // C++ SleighSymbol::decode base: no content decode for this kind
            _ => {
                return Err(KunaError::lowlevel(format!(
                    "Symbol {} cannot be decoded from stream directly",
                    name_text(self.symbol(id)?.get_name())
                )));
            }
        }
        Ok(())
    }

    /// Immutable kind access during content decode (slot existence was
    /// checked by the dispatch).
    fn kind_of(&self, id: u32) -> &SymbolKind {
        &self.symbollist[id as usize]
            .as_ref()
            .expect("checked by decode_symbol_content")
            .kind
    }

    /// Mutable kind access during content decode.
    fn kind_of_mut(&mut self, id: u32) -> &mut SymbolKind {
        &mut self.symbollist[id as usize]
            .as_mut()
            .expect("checked by decode_symbol_content")
            .kind
    }
}

impl OperandValueResolver for SymbolTable {
    /// C++ `OperandValue::decode` resolution:
    /// `dynamic_cast<SubtableSymbol*>(trans->findSymbol(tabid))
    /// ->getNumConstructors()` — a failed lookup/downcast is UB in C++ and
    /// an error here (per the seam contract).
    fn num_constructors(&self, table_id: u32) -> KunaResult<i32> {
        Ok(self.subtable_symbol(table_id)?.get_num_constructors())
    }
}

/// The resolver used while `SymbolTable::decode` is in progress: identical
/// to the post-decode [`SymbolTable`] resolver, except that the subtable
/// whose content is currently being decoded reports its live constructor
/// count (in C++ the `construct` vector grows in place during
/// `SubtableSymbol::decode`).
struct TableResolver<'a> {
    table: &'a SymbolTable,
    /// `(subtable symbol id, constructors decoded so far incl. current)`.
    current: Option<(u32, i32)>,
}

impl OperandValueResolver for TableResolver<'_> {
    fn num_constructors(&self, table_id: u32) -> KunaResult<i32> {
        if let Some((cur_id, cur_num)) = self.current {
            if cur_id == table_id {
                return Ok(cur_num);
            }
        }
        self.table.num_constructors(table_id)
    }
}

/// C++ `SubtableSymbol::decode` body: constructors (`addConstructor` then
/// `Constructor::decode`, so the live count includes the current one) and
/// the decision tree.  Runs against `&SymbolTable` so expression decode can
/// validate operand values mid-restore.
fn decode_subtable_content(
    decoder: &mut dyn Decoder,
    table: &SymbolTable,
    trans: &mut dyn SleighBaseTrans,
    self_id: u32,
) -> KunaResult<(Vec<Constructor>, Option<DecisionNode>)> {
    // C++ reads ATTRIB_NUMCT only to reserve()
    let _numct = decoder.read_signed_integer_id(&sla::ATTRIB_NUMCT)? as i32; // C++ implicit intb -> int4
    let mut construct: Vec<Constructor> = Vec::new();
    let mut decisiontree: Option<DecisionNode> = None;
    let mut subel = decoder.peek_element()?;
    while subel != 0 {
        if subel == sla::ELEM_CONSTRUCTOR {
            // C++ addConstructor(ct) BEFORE ct->decode: the table's live
            // count during this constructor's decode includes it.
            let ct_index = construct.len();
            let resolver = TableResolver {
                table,
                // usize -> i32 count: bounded by MAX_SYMBOLS-scale tables
                current: Some((self_id, ct_index as i32 + 1)),
            };
            let mut ct = Constructor::decode(decoder, &resolver, trans)?;
            ct.set_id(ct_index as u32); // C++ setId(construct.size())
            construct.push(ct);
        } else if subel == sla::ELEM_DECISION {
            // size_t -> int4 live count as C++ getNumConstructors()
            let tree = DecisionNode::decode(decoder, construct.len() as i32)?;
            decisiontree = Some(tree);
        }
        subel = decoder.peek_element()?;
    }
    decoder.close_element(sla::ELEM_SUBTABLE_SYM.get_id())?;
    Ok((construct, decisiontree))
}

#[cfg(test)]
mod tests {
    use kuna_base::marshal::{PackedDecode, PackedEncode};
    use kuna_base::space::{addrspace_flags, AddrSpaceManager, ConstantSpace};

    use crate::slghpatexpress::TokenField;
    use crate::slghpattern::{InstructionPattern, PatternBlock};

    use super::*;

    // -- fixtures --------------------------------------------------------------

    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let spc = AddrSpace::new(
            spacetype_processor(),
            "ram",
            false,
            4,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(spc)).unwrap();
        manager
    }

    fn spacetype_processor() -> kuna_base::space::spacetype {
        kuna_base::space::spacetype::IPTR_PROCESSOR
    }

    fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("ram").unwrap())
    }

    fn cspace(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_constant_space().unwrap())
    }

    /// [`SleighBaseTrans`] over the test manager; no ConstructTpl support
    /// (the symbol tests never include p-code sections).
    struct TestTrans {
        cspace: Rc<AddrSpace>,
    }

    impl TestTrans {
        fn new(manager: &AddrSpaceManager) -> TestTrans {
            TestTrans {
                cspace: cspace(manager),
            }
        }
    }

    impl SleighBaseTrans for TestTrans {
        fn get_constant_space(&self) -> Rc<AddrSpace> {
            Rc::clone(&self.cspace)
        }

        fn decode_construct_tpl(
            &mut self,
            _decoder: &mut dyn Decoder,
        ) -> KunaResult<(i32, ConstructTplHandle)> {
            Err(KunaError::sleigh("no ConstructTpl in symbol tests"))
        }

        fn encode_construct_tpl(
            &self,
            _handle: ConstructTplHandle,
            _section_id: i32,
            _encoder: &mut dyn Encoder,
        ) -> KunaResult<()> {
            Err(KunaError::sleigh("no ConstructTpl in symbol tests"))
        }
    }

    /// Synthetic [`SymbolWalker`]/[`SymbolWalkerChange`]: byte/context
    /// providers mirroring the documented ParserContext semantics, an
    /// operand-path -> constructor map for print recursion, and recorders
    /// for the context-command applies.
    struct TestWalker {
        instr: Vec<u8>,
        ctx: Vec<u32>,
        addr: Option<Address>,
        naddr: Option<Address>,
        const_space: Option<Rc<AddrSpace>>,
        cur_space: Option<Rc<AddrSpace>>,
        /// current operand path (push/pop_operand)
        path: std::cell::RefCell<Vec<i32>>,
        /// operand path -> constructor at that point
        ctmap: Vec<(Vec<i32>, ConstructorRef)>,
        /// canned fixed handles per operand index
        handles: Vec<(i32, FixedHandle)>,
        /// recorded set_context_word calls
        set_words: Vec<(i32, u32, u32)>,
        /// recorded add_commit calls
        commits: Vec<(u32, i32, u32, bool)>,
    }

    impl TestWalker {
        fn from_bytes(instr: &[u8]) -> TestWalker {
            TestWalker {
                instr: instr.to_vec(),
                ctx: Vec::new(),
                addr: None,
                naddr: None,
                const_space: None,
                cur_space: None,
                path: std::cell::RefCell::new(Vec::new()),
                ctmap: Vec::new(),
                handles: Vec::new(),
                set_words: Vec::new(),
                commits: Vec::new(),
            }
        }
    }

    impl PatternExpressionContext for TestWalker {
        fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
            if byteoff < 0 || (byteoff + numbytes) as usize > self.instr.len() {
                return Err(KunaError::bad_data(
                    "Instruction is using more than 16 bytes",
                ));
            }
            let mut res: u32 = 0;
            for i in 0..numbytes {
                res = res.wshl(8);
                res |= u32::from(self.instr[(byteoff + i) as usize]);
            }
            Ok(res)
        }

        fn get_context_bytes(&self, bytestart: i32, size: i32) -> KunaResult<u32> {
            let intstart = (bytestart / 4) as usize;
            let mut res: u32 = *self.ctx.get(intstart).unwrap_or(&0);
            let byte_offset = bytestart % 4;
            let unused_bytes = 4 - size;
            res = res.wshl((byte_offset * 8) as u32);
            res = res.wshr((unused_bytes * 8) as u32);
            Ok(res)
        }

        fn get_addr(&self) -> Address {
            self.addr.clone().expect("test walker addr unset")
        }

        fn get_naddr(&self) -> Address {
            self.naddr.clone().expect("test walker naddr unset")
        }

        fn get_n2addr(&self) -> KunaResult<Address> {
            Err(KunaError::lowlevel(
                "inst_next2 not available in this context",
            ))
        }

        fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
            Err(KunaError::sleigh("no operand values in symbol tests"))
        }
    }

    impl SymbolWalker for TestWalker {
        fn push_operand(&mut self, i: i32) -> KunaResult<()> {
            self.path.borrow_mut().push(i);
            Ok(())
        }

        fn pop_operand(&mut self) -> KunaResult<()> {
            self.path.borrow_mut().pop();
            Ok(())
        }

        fn get_constructor(&self) -> KunaResult<ConstructorRef> {
            let path = self.path.borrow();
            for (p, ctref) in &self.ctmap {
                if *p == *path {
                    return Ok(*ctref);
                }
            }
            Err(KunaError::sleigh("no constructor mapped for operand path"))
        }

        fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
            for (ind, hand) in &self.handles {
                if *ind == i {
                    return Ok(hand.clone());
                }
            }
            Err(KunaError::sleigh("no fixed handle for operand"))
        }

        fn get_const_space(&self) -> Rc<AddrSpace> {
            Rc::clone(self.const_space.as_ref().expect("const space unset"))
        }

        fn get_cur_space(&self) -> Rc<AddrSpace> {
            Rc::clone(self.cur_space.as_ref().expect("cur space unset"))
        }

        fn get_dest_addr(&self) -> KunaResult<Address> {
            Err(KunaError::lowlevel("no flowdest in symbol tests"))
        }

        fn get_ref_addr(&self) -> KunaResult<Address> {
            Err(KunaError::lowlevel("no flowref in symbol tests"))
        }

        fn get_instruction_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
            // bit 0 is the MSB of instruction byte 0
            let mut res: u32 = 0;
            for i in 0..size {
                let bit = startbit + i;
                let byte = (bit / 8) as usize;
                if byte >= self.instr.len() {
                    return Err(KunaError::bad_data("instruction bits past buffer"));
                }
                let b = (self.instr[byte] >> (7 - (bit % 8))) & 1;
                res = (res << 1) | u32::from(b);
            }
            Ok(res)
        }

        fn get_context_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
            // bit 0 is the MSB of context word 0
            let mut res: u32 = 0;
            for i in 0..size {
                let bit = startbit + i;
                let word = (bit / 32) as usize;
                if word >= self.ctx.len() {
                    return Err(KunaError::bad_data("context bits past buffer"));
                }
                let b = (self.ctx[word] >> (31 - (bit % 32))) & 1;
                res = (res << 1) | (b & 1);
            }
            Ok(res)
        }
    }

    impl SymbolWalkerChange for TestWalker {
        fn set_context_word(&mut self, i: i32, val: u32, mask: u32) {
            self.set_words.push((i, val, mask));
        }

        fn add_commit(&mut self, symbol_id: u32, num: i32, mask: u32, flow: bool) {
            self.commits.push((symbol_id, num, mask, flow));
        }
    }

    fn tokenfield(bstart: i32, bend: i32) -> TokenField {
        // 1-byte little-endian unsigned token field
        TokenField::new(1, false, false, bstart, bend)
    }

    // -- SymbolScope / SymbolTable basics ---------------------------------------

    #[test]
    fn symbol_scope_nested_lookup_and_shadowing() {
        let mut table = SymbolTable::new();
        table.add_scope(); // global scope (id 0)
        assert_eq!(table.get_current_scope(), Some(0));
        let outer = table
            .add_global_symbol(SleighSymbol::new_userop(b"thing"))
            .unwrap();
        table.add_scope(); // nested scope (id 1)
        assert_eq!(table.get_current_scope(), Some(1));
        let inner = table
            .add_symbol(SleighSymbol::new_subtable(b"thing"))
            .unwrap();
        assert_ne!(outer, inner);
        // current chain finds the inner (shadowing) symbol
        assert_eq!(table.find_symbol(b"thing").unwrap().get_id(), inner);
        assert_eq!(
            table.find_symbol(b"thing").unwrap().get_type(),
            SymbolType::Subtable
        );
        // skipping one scope finds the global one
        assert_eq!(table.find_symbol_skip(b"thing", 1).unwrap().get_id(), outer);
        // global lookup likewise
        assert_eq!(
            table.find_global_symbol(b"thing").unwrap().get_type(),
            SymbolType::UserOp
        );
        // popScope: current becomes the parent
        table.pop_scope();
        assert_eq!(table.get_current_scope(), Some(0));
        assert_eq!(table.find_symbol(b"thing").unwrap().get_id(), outer);
        // unknown name is None
        assert!(table.find_symbol(b"nope").is_none());
        // by-id lookup
        assert_eq!(table.find_symbol_by_id(inner).unwrap().get_name(), b"thing");
        // scope iteration is name-ordered (SymbolCompare)
        table.set_current_scope(Some(0));
        table
            .add_symbol(SleighSymbol::new_userop(b"alpha"))
            .unwrap();
        let ids: Vec<u32> = table.get_global_scope().unwrap().symbol_ids().collect();
        let names: Vec<&[u8]> = ids
            .iter()
            .map(|i| table.find_symbol_by_id(*i).unwrap().get_name())
            .collect();
        assert_eq!(names, vec![&b"alpha"[..], &b"thing"[..]]);
    }

    #[test]
    fn symbol_table_duplicate_names_error() {
        let mut table = SymbolTable::new();
        table.add_scope();
        table
            .add_global_symbol(SleighSymbol::new_userop(b"dup"))
            .unwrap();
        let err = table
            .add_global_symbol(SleighSymbol::new_userop(b"dup"))
            .expect_err("duplicate must fail");
        assert_eq!(
            format!("{err}"),
            format!("{}", KunaError::sleigh("Duplicate symbol name 'dup'"))
        );
        let err2 = table
            .add_symbol(SleighSymbol::new_userop(b"dup"))
            .expect_err("duplicate must fail");
        assert!(format!("{err2}").contains("Duplicate symbol name: dup"));
    }

    // -- calc_maskword / ContextOp / ContextCommit -------------------------------

    #[test]
    fn context_op_new_calc_maskword() {
        let op = ContextOp::new(
            4,
            7,
            PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(3))),
        )
        .unwrap();
        assert_eq!(op.num, 0);
        assert_eq!(op.shift, 24);
        assert_eq!(op.mask, 0x0f00_0000);
        // field straddling two machine ints
        let err = ContextOp::new(30, 33, PatternExpression::Value(PatternValue::ConstantValue(
            ConstantValue::new(0),
        )))
        .expect_err("straddling field must fail");
        assert!(format!("{err}").contains("Context field not contained within one machine int"));
    }

    #[test]
    fn context_op_decode_apply_roundtrip() {
        let manager = test_manager();
        // hand-encode context_op{i=1, shift=4, mask=0xf0} around intb 3
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_CONTEXT_OP);
            enc.write_signed_integer(&sla::ATTRIB_I, 1);
            enc.write_signed_integer(&sla::ATTRIB_SHIFT, 4);
            enc.write_unsigned_integer(&sla::ATTRIB_MASK, 0xf0);
            enc.open_element(&sla::ELEM_INTB);
            enc.write_signed_integer(&sla::ATTRIB_VAL, 3);
            enc.close_element(&sla::ELEM_INTB);
            enc.close_element(&sla::ELEM_CONTEXT_OP);
        }
        let table = SymbolTable::new();
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let op = ContextOp::decode(&mut dec, &table).unwrap();
        // re-encode is byte identical
        let mut reenc = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut reenc);
            op.encode(&mut enc).unwrap();
        }
        assert_eq!(reenc, buf);
        // apply: val = 3 << 4 into word 1 under mask 0xf0
        let mut walker = TestWalker::from_bytes(&[]);
        ContextChange::Op(op).apply(&mut walker).unwrap();
        assert_eq!(walker.set_words, vec![(1, 0x30, 0xf0)]);
    }

    #[test]
    fn context_commit_decode_apply_roundtrip() {
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_COMMIT);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 5);
            enc.write_signed_integer(&sla::ATTRIB_NUMBER, 1);
            enc.write_unsigned_integer(&sla::ATTRIB_MASK, 0xff);
            enc.write_bool(&sla::ATTRIB_FLOW, true);
            enc.close_element(&sla::ELEM_COMMIT);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let commit = ContextCommit::decode(&mut dec).unwrap();
        let mut reenc = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut reenc);
            commit.encode(&mut enc);
        }
        assert_eq!(reenc, buf);
        let mut walker = TestWalker::from_bytes(&[]);
        commit.apply(&mut walker);
        assert_eq!(walker.commits, vec![(5, 1, 0xff, true)]);
    }

    // -- full symbol-table decode / round-trip ----------------------------------

    fn enc_head(enc: &mut PackedEncode, elem: &kuna_base::marshal::ElementId, name: &[u8], id: u64) {
        enc.open_element(elem);
        enc.write_string(&sla::ATTRIB_NAME, name);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, id);
        enc.write_unsigned_integer(&sla::ATTRIB_SCOPE, 0);
        enc.close_element(elem);
    }

    /// Hand-encode (transcribing the C++ encode methods) a table with one
    /// scope and five symbols: subtable "instruction" (2 constructors +
    /// decision tree), operand "op0", varnode "r0", userop "syscall",
    /// value "imm8".
    fn encode_full_table(manager: &AddrSpaceManager) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&sla::ELEM_SYMBOL_TABLE);
        enc.write_signed_integer(&sla::ATTRIB_SCOPESIZE, 1);
        enc.write_signed_integer(&sla::ATTRIB_SYMBOLSIZE, 5);
        enc.open_element(&sla::ELEM_SCOPE);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
        enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
        enc.close_element(&sla::ELEM_SCOPE);
        // headers (in id order)
        enc_head(&mut enc, &sla::ELEM_SUBTABLE_SYM_HEAD, b"instruction", 0);
        enc_head(&mut enc, &sla::ELEM_OPERAND_SYM_HEAD, b"op0", 1);
        enc_head(&mut enc, &sla::ELEM_VARNODE_SYM_HEAD, b"r0", 2);
        enc_head(&mut enc, &sla::ELEM_USEROP_HEAD, b"syscall", 3);
        enc_head(&mut enc, &sla::ELEM_VALUE_SYM_HEAD, b"imm8", 4);
        // content id 0: subtable with two constructors + decision tree
        enc.open_element(&sla::ELEM_SUBTABLE_SYM);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
        enc.write_signed_integer(&sla::ATTRIB_NUMCT, 2);
        // ct0: "nop"
        enc.open_element(&sla::ELEM_CONSTRUCTOR);
        enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
        enc.write_signed_integer(&sla::ATTRIB_FIRST, -1);
        enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
        enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
        enc.write_signed_integer(&sla::ATTRIB_LINE, 5);
        enc.open_element(&sla::ELEM_PRINT);
        enc.write_string(&sla::ATTRIB_PIECE, b"nop");
        enc.close_element(&sla::ELEM_PRINT);
        enc.close_element(&sla::ELEM_CONSTRUCTOR);
        // ct1: "mov" ' ' opA
        enc.open_element(&sla::ELEM_CONSTRUCTOR);
        enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
        enc.write_signed_integer(&sla::ATTRIB_FIRST, 1);
        enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
        enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
        enc.write_signed_integer(&sla::ATTRIB_LINE, 7);
        enc.open_element(&sla::ELEM_OPER);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
        enc.close_element(&sla::ELEM_OPER);
        enc.open_element(&sla::ELEM_PRINT);
        enc.write_string(&sla::ATTRIB_PIECE, b"mov");
        enc.close_element(&sla::ELEM_PRINT);
        enc.open_element(&sla::ELEM_PRINT);
        enc.write_string(&sla::ATTRIB_PIECE, b" ");
        enc.close_element(&sla::ELEM_PRINT);
        enc.open_element(&sla::ELEM_OPPRINT);
        enc.write_signed_integer(&sla::ATTRIB_ID, 0);
        enc.close_element(&sla::ELEM_OPPRINT);
        enc.close_element(&sla::ELEM_CONSTRUCTOR);
        // decision tree: root splits on instruction bit 0
        enc.open_element(&sla::ELEM_DECISION);
        enc.write_signed_integer(&sla::ATTRIB_NUMBER, 2);
        enc.write_bool(&sla::ATTRIB_CONTEXT, false);
        enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
        enc.write_signed_integer(&sla::ATTRIB_SIZE, 1);
        // child 0: terminal, ct0 always-true
        enc.open_element(&sla::ELEM_DECISION);
        enc.write_signed_integer(&sla::ATTRIB_NUMBER, 1);
        enc.write_bool(&sla::ATTRIB_CONTEXT, false);
        enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
        enc.write_signed_integer(&sla::ATTRIB_SIZE, 0);
        enc.open_element(&sla::ELEM_PAIR);
        enc.write_signed_integer(&sla::ATTRIB_ID, 0);
        InstructionPattern::new_always(true).encode(&mut enc);
        enc.close_element(&sla::ELEM_PAIR);
        enc.close_element(&sla::ELEM_DECISION);
        // child 1: terminal, ct1 requires top two bits == 10
        enc.open_element(&sla::ELEM_DECISION);
        enc.write_signed_integer(&sla::ATTRIB_NUMBER, 1);
        enc.write_bool(&sla::ATTRIB_CONTEXT, false);
        enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
        enc.write_signed_integer(&sla::ATTRIB_SIZE, 0);
        enc.open_element(&sla::ELEM_PAIR);
        enc.write_signed_integer(&sla::ATTRIB_ID, 1);
        InstructionPattern::new(PatternBlock::new(0, 0xc000_0000, 0x8000_0000)).encode(&mut enc);
        enc.close_element(&sla::ELEM_PAIR);
        enc.close_element(&sla::ELEM_DECISION);
        enc.close_element(&sla::ELEM_DECISION);
        enc.close_element(&sla::ELEM_SUBTABLE_SYM);
        // content id 1: operand op0 (defexp = instruction byte 0)
        enc.open_element(&sla::ELEM_OPERAND_SYM);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
        enc.write_signed_integer(&sla::ATTRIB_OFF, 0);
        enc.write_signed_integer(&sla::ATTRIB_BASE, -1);
        enc.write_signed_integer(&sla::ATTRIB_MINLEN, 1);
        enc.write_signed_integer(&sla::ATTRIB_INDEX, 0);
        OperandValue::new(0, 0, 1).encode(&mut enc);
        tokenfield(0, 7).encode(&mut enc);
        enc.close_element(&sla::ELEM_OPERAND_SYM);
        // content id 2: varnode r0 = ram[0x100]:4
        enc.open_element(&sla::ELEM_VARNODE_SYM);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 2);
        enc.write_space(&sla::ATTRIB_SPACE, &ram(manager));
        enc.write_unsigned_integer(&sla::ATTRIB_OFF, 0x100);
        enc.write_signed_integer(&sla::ATTRIB_SIZE, 4);
        enc.close_element(&sla::ELEM_VARNODE_SYM);
        // content id 3: userop syscall index 2
        enc.open_element(&sla::ELEM_USEROP);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 3);
        enc.write_signed_integer(&sla::ATTRIB_INDEX, 2);
        enc.close_element(&sla::ELEM_USEROP);
        // content id 4: value imm8 (low nibble of byte 0)
        enc.open_element(&sla::ELEM_VALUE_SYM);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 4);
        tokenfield(0, 3).encode(&mut enc);
        enc.close_element(&sla::ELEM_VALUE_SYM);
        enc.close_element(&sla::ELEM_SYMBOL_TABLE);
        drop(enc);
        buf
    }

    fn decode_full_table(manager: &AddrSpaceManager) -> (SymbolTable, Vec<u8>) {
        let buf = encode_full_table(manager);
        let mut table = SymbolTable::new();
        let mut trans = TestTrans::new(manager);
        let mut dec = PackedDecode::new(manager);
        dec.ingest_stream(&buf).unwrap();
        table.decode(&mut dec, &mut trans).unwrap();
        (table, buf)
    }

    #[test]
    fn full_table_decode_and_reencode_roundtrip() {
        let manager = test_manager();
        let (table, buf) = decode_full_table(&manager);
        assert_eq!(table.num_scopes(), 1);
        assert_eq!(table.num_symbols(), 5);
        // subtable structure
        let inst = table.find_global_symbol(b"instruction").unwrap();
        assert_eq!(inst.get_type(), SymbolType::Subtable);
        let SymbolKind::Subtable(sub) = inst.kind() else {
            panic!("expected subtable");
        };
        assert_eq!(sub.get_num_constructors(), 2);
        let ct0 = sub.get_constructor(0).unwrap();
        assert_eq!(ct0.get_id(), 0);
        assert_eq!(ct0.get_parent(), Some(0));
        assert_eq!(ct0.get_lineno(), 5);
        assert_eq!(ct0.get_first_whitespace(), -1);
        assert_eq!(ct0.get_flowthru_index(), -1);
        assert_eq!(ct0.get_print_pieces(), &[b"nop".to_vec()]);
        let ct1 = sub.get_constructor(1).unwrap();
        assert_eq!(ct1.get_num_operands(), 1);
        assert_eq!(ct1.get_operand(0).unwrap(), 1);
        assert_eq!(ct1.get_first_whitespace(), 1);
        assert_eq!(ct1.get_flowthru_index(), -1);
        assert_eq!(
            ct1.get_print_pieces(),
            &[b"mov".to_vec(), b" ".to_vec(), b"\nA".to_vec()]
        );
        // operand
        let SymbolKind::Operand(op) = table.find_global_symbol(b"op0").unwrap().kind() else {
            panic!("expected operand");
        };
        assert_eq!(op.get_index(), 0);
        assert_eq!(op.get_offset_base(), -1);
        assert_eq!(op.get_relative_offset(), 0);
        assert_eq!(op.get_minimum_length(), 1);
        assert!(!op.is_code_address());
        assert!(op.get_defining_symbol().is_none());
        assert!(op.get_defining_expression().is_some());
        let lexp = op.get_local_expression().unwrap();
        assert_eq!((lexp.index(), lexp.table_id(), lexp.ct_id()), (0, 0, 1));
        // varnode
        let r0 = table.find_global_symbol(b"r0").unwrap();
        let SymbolKind::Varnode(vn) = r0.kind() else {
            panic!("expected varnode");
        };
        assert_eq!(vn.get_fixed_varnode().offset, 0x100);
        assert_eq!(vn.get_fixed_varnode().size, 4);
        assert_eq!(vn.get_size(), 4);
        assert_eq!(r0.get_size(&table).unwrap(), 4);
        // userop
        let SymbolKind::UserOp(uo) = table.find_global_symbol(b"syscall").unwrap().kind() else {
            panic!("expected userop");
        };
        assert_eq!(uo.get_index(), 2);
        // value symbol pattern
        let imm = table.find_global_symbol(b"imm8").unwrap();
        assert!(matches!(
            imm.get_pattern_value(),
            Some(PatternValue::TokenField(_))
        ));
        // re-encode is byte identical to the hand-built (C++-shaped) stream
        let trans = TestTrans::new(&manager);
        let mut reenc = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut reenc);
            table.encode(&mut enc, &trans).unwrap();
        }
        assert_eq!(reenc, buf);
    }

    #[test]
    fn decision_node_dispatch_and_failure() {
        let manager = test_manager();
        let (table, _) = decode_full_table(&manager);
        let inst = table.find_global_symbol(b"instruction").unwrap();
        // bit 0 clear -> child 0 -> always-true pattern -> ct0
        let mut walker = TestWalker::from_bytes(&[0x00, 0, 0, 0]);
        walker.addr = Some(Address::new(ram(&manager), 0x1000));
        assert_eq!(inst.resolve(&walker).unwrap(), Some(0));
        // bit 0 set, bit 1 clear -> child 1 -> pattern matches -> ct1
        let mut walker = TestWalker::from_bytes(&[0x80, 0, 0, 0]);
        walker.addr = Some(Address::new(ram(&manager), 0x1000));
        assert_eq!(inst.resolve(&walker).unwrap(), Some(1));
        // bits 11 -> child 1 -> pattern mismatch -> BadDataError
        let mut walker = TestWalker::from_bytes(&[0xc0, 0, 0, 0]);
        walker.addr = Some(Address::new(ram(&manager), 0x1000));
        let err = inst.resolve(&walker).expect_err("must fail to resolve");
        assert!(format!("{err}").contains("Unable to resolve constructor"));
    }

    #[test]
    fn constructor_print_mnemonic_and_body() {
        let manager = test_manager();
        let (table, _) = decode_full_table(&manager);
        let sub = match table.find_global_symbol(b"instruction").unwrap().kind() {
            SymbolKind::Subtable(s) => s,
            _ => panic!("expected subtable"),
        };
        let ct1 = sub.get_constructor(1).unwrap();
        let mut walker = TestWalker::from_bytes(&[0x8a, 0, 0, 0]);
        // full print: "mov" + " " + defexp value of byte 0 (0x8a)
        let mut s = String::new();
        ct1.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "mov 0x8a");
        // mnemonic stops at firstwhitespace
        let mut s = String::new();
        ct1.print_mnemonic(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "mov");
        // body starts after firstwhitespace
        let mut s = String::new();
        ct1.print_body(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "0x8a");
        // printInfo names the parent table
        let mut s = String::new();
        ct1.print_info(&mut s, &table).unwrap();
        assert_eq!(s, "table \"instruction\" constructor starting at line 7");
    }

    #[test]
    fn operand_print_recurses_through_subtable_and_varnode() {
        let manager = test_manager();
        let mut table = SymbolTable::new();
        table.add_scope();
        // id 0: subtable "sub" with one constructor printing "nop"
        let mut sub = SleighSymbol::new_subtable(b"sub");
        if let SymbolKind::Subtable(s) = sub.kind_mut() {
            let mut ct = Constructor::new();
            ct.set_parent(0);
            ct.add_syntax(b"nop");
            s.add_constructor(ct);
        }
        assert_eq!(table.add_global_symbol(sub).unwrap(), 0);
        // id 1: varnode r0
        let r0 = SleighSymbol::new_varnode(b"r0", ram(&manager), 0x100, 4);
        assert_eq!(table.add_global_symbol(r0).unwrap(), 1);
        // id 2: operand opA defined by the subtable
        let mut op_a = SleighSymbol::new_operand(b"opA", 0, ConstructorRef { table_id: 3, ct_id: 0 });
        if let SymbolKind::Operand(o) = op_a.kind_mut() {
            o.define_operand_symbol(0).unwrap();
            // redefinition is the C++ SleighError
            assert!(o.define_operand_symbol(1).is_err());
        }
        assert_eq!(table.add_global_symbol(op_a).unwrap(), 2);
        // id 3: top-level table "instruction" with ct: "do " opA " " opB
        let mut op_b = SleighSymbol::new_operand(b"opB", 1, ConstructorRef { table_id: 3, ct_id: 0 });
        if let SymbolKind::Operand(o) = op_b.kind_mut() {
            o.define_operand_symbol(1).unwrap();
        }
        let mut top = SleighSymbol::new_subtable(b"instruction");
        if let SymbolKind::Subtable(s) = top.kind_mut() {
            let mut ct = Constructor::new();
            ct.set_parent(3);
            ct.add_syntax(b"do ");
            ct.add_operand(2);
            ct.add_syntax(b" ");
            ct.add_operand(4);
            s.add_constructor(ct);
        }
        assert_eq!(table.add_global_symbol(top).unwrap(), 3);
        assert_eq!(table.add_global_symbol(op_b).unwrap(), 4);
        // the walker resolves operand path [0] to sub's constructor 0
        let mut walker = TestWalker::from_bytes(&[0, 0, 0, 0]);
        walker.ctmap.push((
            vec![0],
            ConstructorRef {
                table_id: 0,
                ct_id: 0,
            },
        ));
        let ct = table
            .get_constructor(ConstructorRef {
                table_id: 3,
                ct_id: 0,
            })
            .unwrap();
        assert_eq!(
            ct.get_print_pieces(),
            &[b"do ".to_vec(), b"\nA".to_vec(), b" ".to_vec(), b"\nB".to_vec()]
        );
        assert_eq!(ct.get_first_whitespace(), 2);
        let mut s = String::new();
        ct.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "do nop r0");
        // operand-path stack unwound
        assert!(walker.path.borrow().is_empty());
        // ContextOp::validate: opA has offsetbase 0 (not constructor
        // relative) -> the C++ SleighError
        let op = ContextOp::new(
            0,
            3,
            PatternExpression::Value(PatternValue::OperandValue(OperandValue::new(0, 3, 0))),
        )
        .unwrap();
        let err = op.validate(&table).expect_err("must reject operand");
        assert!(format!("{err}").contains("opA: cannot be used in context expression"));
    }

    // -- per-kind decode: table-fill validation, handles, guards ------------------

    /// Hand-encode a one-scope table whose symbols are written by `body`
    /// (headers first, then contents), then decode it.
    fn decode_mini_table(
        manager: &AddrSpaceManager,
        nsyms: i64,
        heads: impl FnOnce(&mut PackedEncode),
        contents: impl FnOnce(&mut PackedEncode),
    ) -> KunaResult<SymbolTable> {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_SYMBOL_TABLE);
            enc.write_signed_integer(&sla::ATTRIB_SCOPESIZE, 1);
            enc.write_signed_integer(&sla::ATTRIB_SYMBOLSIZE, nsyms);
            enc.open_element(&sla::ELEM_SCOPE);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
            enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
            enc.close_element(&sla::ELEM_SCOPE);
            heads(&mut enc);
            contents(&mut enc);
            enc.close_element(&sla::ELEM_SYMBOL_TABLE);
        }
        let mut table = SymbolTable::new();
        let mut trans = TestTrans::new(manager);
        let mut dec = PackedDecode::new(manager);
        dec.ingest_stream(&buf)?;
        table.decode(&mut dec, &mut trans)?;
        Ok(table)
    }

    #[test]
    fn name_symbol_check_table_fill_resolve_print() {
        let manager = test_manager();
        let table = decode_mini_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_NAME_SYM_HEAD, b"names", 0),
            |enc| {
                enc.open_element(&sla::ELEM_NAME_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                tokenfield(0, 0).encode(enc);
                enc.open_element(&sla::ELEM_NAMETAB);
                enc.write_string(&sla::ATTRIB_NAME, b"add");
                enc.close_element(&sla::ELEM_NAMETAB);
                enc.open_element(&sla::ELEM_NAMETAB);
                enc.write_string(&sla::ATTRIB_NAME, b"_");
                enc.close_element(&sla::ELEM_NAMETAB);
                enc.close_element(&sla::ELEM_NAME_SYM);
            },
        )
        .unwrap();
        let sym = table.find_global_symbol(b"names").unwrap();
        let SymbolKind::Name(ns) = sym.kind() else {
            panic!("expected name symbol");
        };
        // checkTableFill: "_" became the TAB illegal-index marker
        assert_eq!(ns.get_name_table(), &[b"add".to_vec(), b"\t".to_vec()]);
        // index 1 hits the illegal entry -> C++ BadDataError
        let mut walker = TestWalker::from_bytes(&[0x01]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        let err = sym.resolve(&walker).expect_err("illegal index");
        assert!(format!("{err}").contains("No corresponding entry in nametable"));
        // index 0 resolves (returning the C++ null constructor) and prints
        let mut walker = TestWalker::from_bytes(&[0x00]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        assert_eq!(sym.resolve(&walker).unwrap(), None);
        let mut s = String::new();
        sym.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "add");
    }

    #[test]
    fn valuemap_symbol_badbeef_resolve_handle_print() {
        let manager = test_manager();
        let table = decode_mini_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_VALUEMAP_SYM_HEAD, b"vmap", 0),
            |enc| {
                enc.open_element(&sla::ELEM_VALUEMAP_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                tokenfield(0, 0).encode(enc);
                enc.open_element(&sla::ELEM_VALUETAB);
                enc.write_signed_integer(&sla::ATTRIB_VAL, 16);
                enc.close_element(&sla::ELEM_VALUETAB);
                enc.open_element(&sla::ELEM_VALUETAB);
                enc.write_signed_integer(&sla::ATTRIB_VAL, 0xBADBEEF);
                enc.close_element(&sla::ELEM_VALUETAB);
                enc.close_element(&sla::ELEM_VALUEMAP_SYM);
            },
        )
        .unwrap();
        let sym = table.find_global_symbol(b"vmap").unwrap();
        // the 0xBADBEEF hole -> C++ BadDataError
        let mut walker = TestWalker::from_bytes(&[0x01]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        let err = sym.resolve(&walker).expect_err("hole index");
        assert!(format!("{err}").contains("No corresponding entry in valuetable"));
        // index 0: fixed handle carries valuetable[0] in the const space
        let mut walker = TestWalker::from_bytes(&[0x00]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        walker.const_space = Some(cspace(&manager));
        assert_eq!(sym.resolve(&walker).unwrap(), None);
        let mut hand = FixedHandle::default();
        sym.get_fixed_handle(&mut hand, &walker, &table).unwrap();
        assert_eq!(hand.offset_offset, 16);
        assert_eq!(hand.size, 0);
        assert!(hand.offset_space.is_none());
        assert_eq!(hand.space.as_ref().unwrap().get_name(), "const");
        let mut s = String::new();
        sym.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "0x10");
    }

    #[test]
    fn varnodelist_symbol_resolve_handle_print_size() {
        let manager = test_manager();
        let table = decode_mini_table(
            &manager,
            2,
            |enc| {
                enc_head(enc, &sla::ELEM_VARNODE_SYM_HEAD, b"r0", 0);
                enc_head(enc, &sla::ELEM_VARLIST_SYM_HEAD, b"reg", 1);
            },
            |enc| {
                enc.open_element(&sla::ELEM_VARNODE_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.write_space(&sla::ATTRIB_SPACE, &ram(&test_manager()));
                enc.write_unsigned_integer(&sla::ATTRIB_OFF, 0x100);
                enc.write_signed_integer(&sla::ATTRIB_SIZE, 4);
                enc.close_element(&sla::ELEM_VARNODE_SYM);
                enc.open_element(&sla::ELEM_VARLIST_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
                tokenfield(0, 0).encode(enc);
                enc.open_element(&sla::ELEM_VAR);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.close_element(&sla::ELEM_VAR);
                enc.open_element(&sla::ELEM_NULL);
                enc.close_element(&sla::ELEM_NULL);
                enc.close_element(&sla::ELEM_VARLIST_SYM);
            },
        )
        .unwrap();
        let sym = table.find_global_symbol(b"reg").unwrap();
        let SymbolKind::VarnodeList(vl) = sym.kind() else {
            panic!("expected varnodelist");
        };
        assert_eq!(vl.get_varnode_table(), &[Some(0), None]);
        // null entry -> C++ BadDataError
        let mut walker = TestWalker::from_bytes(&[0x01]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        let err = sym.resolve(&walker).expect_err("null entry");
        assert!(format!("{err}").contains("No corresponding entry in varnode list"));
        // entry 0 resolves to r0's varnode data and name
        let mut walker = TestWalker::from_bytes(&[0x00]);
        walker.addr = Some(Address::new(ram(&manager), 0x40));
        assert_eq!(sym.resolve(&walker).unwrap(), None);
        let mut hand = FixedHandle::default();
        sym.get_fixed_handle(&mut hand, &walker, &table).unwrap();
        assert_eq!(hand.offset_offset, 0x100);
        assert_eq!(hand.size, 4);
        assert_eq!(hand.space.as_ref().unwrap().get_name(), "ram");
        let mut s = String::new();
        sym.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "r0");
        // getSize: first non-null entry's size
        assert_eq!(sym.get_size(&table).unwrap(), 4);
    }

    #[test]
    fn start_end_epsilon_symbols_decode_handles_print() {
        let manager = test_manager();
        let table = decode_mini_table(
            &manager,
            3,
            |enc| {
                enc_head(enc, &sla::ELEM_START_SYM_HEAD, b"inst_start", 0);
                enc_head(enc, &sla::ELEM_END_SYM_HEAD, b"inst_next", 1);
                enc_head(enc, &sla::ELEM_EPSILON_SYM_HEAD, b"epsilon", 2);
            },
            |enc| {
                enc.open_element(&sla::ELEM_START_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.close_element(&sla::ELEM_START_SYM);
                enc.open_element(&sla::ELEM_END_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
                enc.close_element(&sla::ELEM_END_SYM);
                enc.open_element(&sla::ELEM_EPSILON_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 2);
                enc.close_element(&sla::ELEM_EPSILON_SYM);
            },
        )
        .unwrap();
        let mut walker = TestWalker::from_bytes(&[]);
        walker.addr = Some(Address::new(ram(&manager), 0x1000));
        walker.naddr = Some(Address::new(ram(&manager), 0x1004));
        walker.cur_space = Some(ram(&manager));
        // StartSymbol: current address in the current space
        let start = table.find_global_symbol(b"inst_start").unwrap();
        let mut hand = FixedHandle::default();
        start.get_fixed_handle(&mut hand, &walker, &table).unwrap();
        assert_eq!(hand.offset_offset, 0x1000);
        assert_eq!(hand.size, 4); // ram getAddrSize()
        assert_eq!(hand.space.as_ref().unwrap().get_name(), "ram");
        let mut s = String::new();
        start.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "0x1000");
        assert!(matches!(
            start.get_pattern_expression().unwrap(),
            Some(PatternExpression::Value(
                PatternValue::StartInstructionValue(_)
            ))
        ));
        // EndSymbol: next address
        let end = table.find_global_symbol(b"inst_next").unwrap();
        let mut s = String::new();
        end.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "0x1004");
        // EpsilonSymbol: prints '0', constant-0 pattern, const-space handle
        let eps = table.find_global_symbol(b"epsilon").unwrap();
        let mut s = String::new();
        eps.print(&mut s, &mut walker, &table).unwrap();
        assert_eq!(s, "0");
        let mut hand = FixedHandle::default();
        eps.get_fixed_handle(&mut hand, &walker, &table).unwrap();
        assert_eq!(hand.offset_offset, 0);
        assert_eq!(hand.size, 0);
        assert_eq!(hand.space.as_ref().unwrap().get_name(), "const");
        match eps.get_pattern_expression().unwrap() {
            Some(PatternExpression::Value(PatternValue::ConstantValue(c))) => {
                assert_eq!(c.get_value(), 0);
            }
            other => panic!("expected constant 0, got {other:?}"),
        }
    }

    #[test]
    fn operand_value_helpers_via_decoded_table() {
        let manager = test_manager();
        let (table, _) = decode_full_table(&manager);
        // op0 was decoded with BASE -1: constructor relative
        let ov = OperandValue::new(0, 0, 1);
        assert!(table.operand_value_is_constructor_relative(&ov).unwrap());
        assert_eq!(table.operand_value_name(&ov).unwrap(), b"op0");
        // SymbolTable is the OperandValueResolver for its own subtables
        assert_eq!(table.num_constructors(0).unwrap(), 2);
        assert!(table.num_constructors(2).is_err()); // varnode, not a subtable
        // ContextOp::validate accepts the constructor-relative operand
        let op = ContextOp::new(
            0,
            3,
            PatternExpression::Value(PatternValue::OperandValue(ov)),
        )
        .unwrap();
        op.validate(&table).unwrap();
    }

    #[test]
    fn decode_guards_and_bad_streams() {
        let manager = test_manager();
        // unknown header element -> "Bad symbol xml"
        let err = decode_mini_table(
            &manager,
            1,
            |enc| {
                enc.open_element(&sla::ELEM_VAR);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.close_element(&sla::ELEM_VAR);
            },
            |_| {},
        )
        .expect_err("bad header");
        assert!(format!("{err}").contains("Bad symbol xml"));
        // duplicate content for one symbol -> "Already decoded symbol"
        let err = decode_mini_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_VALUE_SYM_HEAD, b"v", 0),
            |enc| {
                for _ in 0..2 {
                    enc.open_element(&sla::ELEM_VALUE_SYM);
                    enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                    tokenfield(0, 0).encode(enc);
                    enc.close_element(&sla::ELEM_VALUE_SYM);
                }
            },
        )
        .expect_err("double decode");
        assert!(format!("{err}").contains("Already decoded symbol"));
        // header id out of range -> "Bad symbol id: exceeds symbollist size"
        let err = decode_mini_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_VALUE_SYM_HEAD, b"v", 9),
            |_| {},
        )
        .expect_err("bad id");
        assert!(format!("{err}").contains("Bad symbol id: exceeds symbollist size"));
        // ContextSymbol without low/high -> "Missing high/low attributes"
        let err = decode_mini_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_CONTEXT_SYM_HEAD, b"ctx", 0),
            |enc| {
                enc.open_element(&sla::ELEM_CONTEXT_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.write_unsigned_integer(&sla::ATTRIB_VARNODE, 0);
                enc.close_element(&sla::ELEM_CONTEXT_SYM);
            },
        )
        .expect_err("missing low/high");
        assert!(format!("{err}").contains("Missing high/low attributes"));
    }

    #[test]
    fn context_symbol_decode_attributes() {
        let manager = test_manager();
        let table = decode_mini_table(
            &manager,
            2,
            |enc| {
                enc_head(enc, &sla::ELEM_VARNODE_SYM_HEAD, b"ctxreg", 0);
                enc_head(enc, &sla::ELEM_CONTEXT_SYM_HEAD, b"mode", 1);
            },
            |enc| {
                enc.open_element(&sla::ELEM_VARNODE_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.write_space(&sla::ATTRIB_SPACE, &ram(&test_manager()));
                enc.write_unsigned_integer(&sla::ATTRIB_OFF, 0);
                enc.write_signed_integer(&sla::ATTRIB_SIZE, 4);
                enc.close_element(&sla::ELEM_VARNODE_SYM);
                // C++ ContextSymbol::encode order: id, varnode, low, high,
                // flow, patval
                enc.open_element(&sla::ELEM_CONTEXT_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
                enc.write_unsigned_integer(&sla::ATTRIB_VARNODE, 0);
                enc.write_signed_integer(&sla::ATTRIB_LOW, 4);
                enc.write_signed_integer(&sla::ATTRIB_HIGH, 7);
                enc.write_bool(&sla::ATTRIB_FLOW, true);
                ContextField::new(false, 4, 7).encode(enc);
                enc.close_element(&sla::ELEM_CONTEXT_SYM);
            },
        )
        .unwrap();
        let sym = table.find_global_symbol(b"mode").unwrap();
        assert_eq!(sym.get_type(), SymbolType::Context);
        let SymbolKind::Context(cs) = sym.kind() else {
            panic!("expected context symbol");
        };
        assert_eq!(cs.get_varnode(), Some(0));
        assert_eq!(cs.get_low(), 4);
        assert_eq!(cs.get_high(), 7);
        assert!(cs.get_flow());
        // buildXrefs-style access to the ContextField bits
        match sym.get_pattern_value() {
            Some(PatternValue::ContextField(f)) => {
                assert_eq!(f.get_start_bit(), 4);
                assert_eq!(f.get_end_bit(), 7);
            }
            other => panic!("expected context field, got {other:?}"),
        }
    }
}





