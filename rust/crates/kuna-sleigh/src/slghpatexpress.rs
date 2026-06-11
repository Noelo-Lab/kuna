//! Port of `decompiler/cpp/slghpatexpress.{hh,cc}` (item `w2-sleigh-pattern`)
//! — the SLEIGH pattern-expression tree.
//!
//! ## What is ported
//!
//! The full runtime `PatternExpression` tree: the `PatternValue` leaves
//! ([`TokenField`], [`ContextField`], [`ConstantValue`], [`OperandValue`],
//! [`StartInstructionValue`], [`EndInstructionValue`],
//! [`Next2InstructionValue`]) and the operator nodes (Plus/Sub/Mult/
//! LeftShift/RightShift/And/Or/Xor/Div binary, Minus/Not unary), with
//!
//! - `getValue` evaluation against parser-walker state,
//! - `minValue` / `maxValue`,
//! - `listValues` / `getMinMax` / `getSubValue`,
//! - `encode`, the per-class `decode` methods, and the
//!   [`PatternExpression::decode_expression`] factory keyed by sla
//!   ElementIds (defined in [`crate::slghpattern::sla`]).
//!
//! The C++ class hierarchy (virtual dispatch + `dynamic_cast`) maps onto two
//! enums mirroring the C++ split: [`PatternValue`] (the `PatternValue`
//! subclasses) wrapped by [`PatternExpression`] (`PatternValue` plus the
//! `BinaryExpression`/`UnaryExpression` operators).  The C++ intrusive
//! refcount (`refcount`/`layClaim`/`release`) is replaced by plain ownership
//! (`Box` children): decoded expression trees are never shared in the
//! consumer-side code paths this crate ports.
//!
//! ## What is NOT ported (SLEIGH compiler side)
//!
//! `TokenPattern` (the Token-aligned pattern builder), the whole
//! `PatternEquation` hierarchy (`OperandEquation`, `UnconstrainedEquation`,
//! `ValExpressEquation` and its comparison subclasses, `EquationAnd`/`Or`/
//! `Cat`, the ellipsis equations, `OperandResolve`), the pattern-generation
//! virtuals `genPattern` / `genMinPattern`, and the static helpers
//! `buildPattern` / `advance_combo`.  All of these exist only to *compile* a
//! `.slaspec`; the Rust port reads compiled `.sla` files and the compiler
//! stays C++ (see the crate docs).  `Token` itself is unported; the one
//! ported constructor that needs it, `TokenField(Token*,bool,int4,int4)`,
//! becomes [`TokenField::new`] taking the two Token properties the C++
//! constructor reads (`getSize()`, `isBigEndian()`).
//!
//! ## The ParserWalker seam ([`PatternExpressionContext`])
//!
//! Evaluation in C++ runs against a `ParserWalker` (context.hh/sleigh.hh),
//! which does not exist yet at this point in the port DAG.
//! [`PatternExpressionContext`] is the minimal trait mirroring exactly the
//! `ParserWalker` surface that slghpatexpress.cc and slghpattern.cc touch:
//! `getInstructionBytes`, `getContextBytes`, `getAddr`, `getNaddr`,
//! `getN2addr`, plus one method standing in for the body of
//! `OperandValue::getValue` (see [`PatternExpressionContext::operand_value`]).
//! The sleigh-core wave implements this trait for its `ParserWalker`.
//!
//! ## The decode seam ([`OperandValueResolver`])
//!
//! `OperandValue::decode` in C++ resolves its cached `Constructor*` through
//! the `Translate*` (really `SleighBase*`) passed to `decodeExpression`:
//! `findSymbol(tabid)` -> `SubtableSymbol` -> `getNumConstructors()` /
//! `getConstructor(ctid)`.  The symbol table is not ported yet, so the Rust
//! [`OperandValue`] stores the raw `(table_id, ct_id)` pair and the decode
//! validation goes through the [`OperandValueResolver`] seam, implemented by
//! the slghsymbol/sleighbase wave.  Two C++ `OperandValue` methods that
//! consult the symbol table at runtime — `isConstructorRelative()` and
//! `getName()` (used by `ContextOp::validate` in slghsymbol.cc) — are left
//! to that wave, which can reach them through the exposed
//! [`OperandValue::index`]/[`OperandValue::table_id`]/[`OperandValue::ct_id`]
//! accessors.  `OperandValue::getSubValue` (which evaluates the operand's
//! *defining expression*, again through the symbol table) is only reachable
//! from the unported compiler equations and returns a `Sleigh` error here.

use kuna_base::address::{byte_swap_inplace, sign_extend, zero_extend, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

use crate::slghpattern::sla;

// ---------------------------------------------------------------------------
// PatternExpressionContext — the ParserWalker seam
// ---------------------------------------------------------------------------

/// The minimal `ParserWalker` surface needed to evaluate patterns and
/// pattern expressions (see module docs).  Implemented by the sleigh-core
/// wave's `ParserWalker`; tests implement it over synthetic byte/context
/// providers.
pub trait PatternExpressionContext {
    /// C++ `ParserWalker::getInstructionBytes(int4 byteoff,int4 numbytes)`:
    /// packed big-endian instruction bytes, `byteoff` relative to the
    /// current constructor's starting offset within the instruction (the
    /// `point->offset` plumbing lives in the implementor).  The C++
    /// `ParserContext::getInstructionBytes` throws `BadDataError` when the
    /// read runs past `MAX_INSTRUCTION_LEN`; implementors surface that as
    /// `Err(KunaError::BadData)`.
    fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32>;

    /// C++ `ParserWalker::getContextBytes(int4 byteoff,int4 numbytes)`:
    /// packed bytes from the local context-word array.
    fn get_context_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32>;

    /// C++ `ParserWalker::getAddr()`: starting address of the instruction.
    fn get_addr(&self) -> Address;

    /// C++ `ParserWalker::getNaddr()`: address of the next instruction.
    fn get_naddr(&self) -> Address;

    /// C++ `ParserWalker::getN2addr()`: address of the instruction after the
    /// next.  `ParserContext::getN2addr` throws `LowlevelError` ("inst_next2
    /// not available in this context") when it cannot be computed, hence the
    /// `Result`.
    fn get_n2addr(&self) -> KunaResult<Address>;

    /// Stand-in for the body of C++ `OperandValue::getValue`
    /// (slghpatexpress.cc), which needs the symbol table and an out-of-band
    /// walker.  The implementor (sleigh-core wave) must transcribe exactly:
    ///
    /// ```text
    /// OperandSymbol *sym = ct->getOperand(index);     // ct from (table_id, ct_id)
    /// PatternExpression *patexp = sym->getDefiningExpression();
    /// if (patexp == 0) {
    ///   TripleSymbol *defsym = sym->getDefiningSymbol();
    ///   if (defsym != 0) patexp = defsym->getPatternExpression();
    ///   if (patexp == 0) return 0;
    /// }
    /// ConstructState tempstate;
    /// ParserWalker newwalker(walker.getParserContext());
    /// newwalker.setOutOfBandState(ct,index,&tempstate,walker);
    /// return patexp->getValue(newwalker);
    /// ```
    fn operand_value(&self, index: i32, table_id: u32, ct_id: u32) -> KunaResult<i64>;
}

/// Decode-time seam standing in for the `Translate*` (really `SleighBase*`)
/// argument of `PatternExpression::decodeExpression`; see module docs.
pub trait OperandValueResolver {
    /// C++ `OperandValue::decode`:
    /// `SubtableSymbol *tab =
    /// dynamic_cast<SubtableSymbol*>(((SleighBase*)trans)->findSymbol(tabid))`
    /// followed by `tab->getNumConstructors()`.  A failed lookup/downcast is
    /// a null dereference (UB) in C++; implementors may error or panic.
    fn num_constructors(&self, table_id: u32) -> KunaResult<i32>;
}

// ---------------------------------------------------------------------------
// Static helpers (slghpatexpress.cc file-local functions)
// ---------------------------------------------------------------------------

/// C++ static `getInstructionBytes(ParserWalker&,int4,int4,bool)`:
/// build an intb from the instruction bytes.
fn instruction_bytes(
    walker: &dyn PatternExpressionContext,
    bytestart: i32,
    byteend: i32,
    bigendian: bool,
) -> KunaResult<i64> {
    let mut res: i64 = 0;
    let mut bytestart = bytestart;
    let size = byteend - bytestart + 1;
    let mut tmpsize = size;
    // C++ `tmpsize >= sizeof(uintm)` compares int4 against size_t: tmpsize
    // converts to 64-bit unsigned (sign-extended), so a negative tmpsize
    // keeps looping until the walker errors out (the C++ loop runs until
    // getInstructionBytes throws BadDataError).  Replicated explicitly.
    while (tmpsize as i64 as u64) >= 4 {
        let tmp = walker.get_instruction_bytes(bytestart, 4)?;
        res = res.wshl(32); // intb << 32: high bits drop intentionally
        res |= i64::from(tmp); // uintm -> intb: zero-extending u32 -> i64
        bytestart += 4;
        tmpsize -= 4;
    }
    if tmpsize > 0 {
        let tmp = walker.get_instruction_bytes(bytestart, tmpsize)?;
        // tmpsize in (0,4) here: shift in (0,32)
        res = res.wshl((8 * tmpsize) as u32);
        res |= i64::from(tmp); // uintm -> intb: zero-extending u32 -> i64
    }
    if !bigendian {
        byte_swap_inplace(&mut res, size);
    }
    Ok(res)
}

/// C++ static `getContextBytes(ParserWalker&,int4,int4)`:
/// build an intb from the context bytes.
fn context_bytes(
    walker: &dyn PatternExpressionContext,
    bytestart: i32,
    byteend: i32,
) -> KunaResult<i64> {
    let mut res: i64 = 0;
    let mut bytestart = bytestart;
    let mut size = byteend - bytestart + 1;
    // C++ `size >= sizeof(uintm)` is the same int4 vs size_t mixed
    // comparison as in instruction_bytes; replicated explicitly.
    while (size as i64 as u64) >= 4 {
        let tmp = walker.get_context_bytes(bytestart, 4)?;
        res = res.wshl(32); // intb << 32: high bits drop intentionally
        res |= i64::from(tmp); // uintm -> intb: zero-extending u32 -> i64
        bytestart += 4;
        size = byteend - bytestart + 1;
    }
    if size > 0 {
        let tmp = walker.get_context_bytes(bytestart, size)?;
        // size in (0,4) here: shift in (0,32)
        res = res.wshl((8 * size) as u32);
        res |= i64::from(tmp); // uintm -> intb: zero-extending u32 -> i64
    }
    Ok(res)
}

// ---------------------------------------------------------------------------
// PatternValue leaves
// ---------------------------------------------------------------------------

/// C++ `TokenField`: a value extracted from a bit range of an instruction
/// token.  The C++ `Token *tok` member is only consulted by the unported
/// compiler-side `genPattern`/`genMinPattern` (and is nulled by the C++
/// decode anyway), so it is dropped here.
#[derive(Debug, Clone)]
pub struct TokenField {
    bigendian: bool,
    signbit: bool,
    /// Bits within the token, 0 bit is LEAST significant
    bitstart: i32,
    bitend: i32,
    /// Bytes to read to get value
    bytestart: i32,
    byteend: i32,
    /// Amount to shift to align value (bitstart % 8)
    shift: i32,
}

impl TokenField {
    /// C++ `TokenField(Token *tk,bool s,int4 bstart,int4 bend)`.  `Token` is
    /// SLEIGH-compiler state that is not ported; the two properties the C++
    /// constructor reads from it (`tok->getSize()`, `tok->isBigEndian()`)
    /// are passed directly.
    pub fn new(tok_size: i32, tok_bigendian: bool, s: bool, bstart: i32, bend: i32) -> TokenField {
        let (bytestart, byteend) = if tok_bigendian {
            ((tok_size * 8 - bend - 1) / 8, (tok_size * 8 - bstart - 1) / 8)
        } else {
            (bstart / 8, bend / 8)
        };
        TokenField {
            bigendian: tok_bigendian,
            signbit: s,
            bitstart: bstart,
            bitend: bend,
            bytestart,
            byteend,
            shift: bstart % 8,
        }
    }

    /// C++ `TokenField::getValue`: construct value given specific
    /// instruction stream.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        let mut res = instruction_bytes(walker, self.bytestart, self.byteend, self.bigendian)?;
        // intb >> int4: shift is bitstart%8 from the constructor, but comes
        // raw from decode; out-of-range counts resolve x86-masked (ADR 0003)
        res = res.wshr(self.shift as u32); // count masked mod 64 anyway
        if self.signbit {
            res = sign_extend(res, self.bitend - self.bitstart);
        } else {
            res = zero_extend(res, self.bitend - self.bitstart);
        }
        Ok(res)
    }

    /// C++ `TokenField::minValue`.
    pub fn min_value(&self) -> i64 {
        0
    }

    /// C++ `TokenField::maxValue`: `zero_extend(~(intb)0, bitend-bitstart)`.
    pub fn max_value(&self) -> i64 {
        let res: i64 = 0;
        zero_extend(!res, self.bitend - self.bitstart)
    }

    /// C++ `TokenField::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_TOKENFIELD);
        encoder.write_bool(&sla::ATTRIB_BIGENDIAN, self.bigendian);
        encoder.write_bool(&sla::ATTRIB_SIGNBIT, self.signbit);
        encoder.write_signed_integer(&sla::ATTRIB_STARTBIT, i64::from(self.bitstart));
        encoder.write_signed_integer(&sla::ATTRIB_ENDBIT, i64::from(self.bitend));
        encoder.write_signed_integer(&sla::ATTRIB_STARTBYTE, i64::from(self.bytestart));
        encoder.write_signed_integer(&sla::ATTRIB_ENDBYTE, i64::from(self.byteend));
        encoder.write_signed_integer(&sla::ATTRIB_SHIFT, i64::from(self.shift));
        encoder.close_element(&sla::ELEM_TOKENFIELD);
    }

    /// C++ `TokenField::decode` (sets `tok = (Token*)0`, which the port
    /// drops entirely).
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<TokenField> {
        let el = decoder.open_element_id(&sla::ELEM_TOKENFIELD)?;
        let bigendian = decoder.read_bool_id(&sla::ATTRIB_BIGENDIAN)?;
        let signbit = decoder.read_bool_id(&sla::ATTRIB_SIGNBIT)?;
        let bitstart = decoder.read_signed_integer_id(&sla::ATTRIB_STARTBIT)? as i32; // C++ implicit intb -> int4
        let bitend = decoder.read_signed_integer_id(&sla::ATTRIB_ENDBIT)? as i32; // C++ implicit intb -> int4
        let bytestart = decoder.read_signed_integer_id(&sla::ATTRIB_STARTBYTE)? as i32; // C++ implicit intb -> int4
        let byteend = decoder.read_signed_integer_id(&sla::ATTRIB_ENDBYTE)? as i32; // C++ implicit intb -> int4
        let shift = decoder.read_signed_integer_id(&sla::ATTRIB_SHIFT)? as i32; // C++ implicit intb -> int4
        decoder.close_element(el)?;
        Ok(TokenField {
            bigendian,
            signbit,
            bitstart,
            bitend,
            bytestart,
            byteend,
            shift,
        })
    }
}

/// C++ `ContextField`: a value extracted from a bit range of the context.
#[derive(Debug, Clone)]
pub struct ContextField {
    startbit: i32,
    endbit: i32,
    startbyte: i32,
    endbyte: i32,
    shift: i32,
    signbit: bool,
}

impl ContextField {
    /// C++ `ContextField(bool s,int4 sbit,int4 ebit)`.
    pub fn new(s: bool, sbit: i32, ebit: i32) -> ContextField {
        ContextField {
            signbit: s,
            startbit: sbit,
            endbit: ebit,
            startbyte: sbit / 8,
            endbyte: ebit / 8,
            shift: 7 - (ebit % 8),
        }
    }

    /// C++ `ContextField::getStartBit`.
    pub fn get_start_bit(&self) -> i32 {
        self.startbit
    }

    /// C++ `ContextField::getEndBit`.
    pub fn get_end_bit(&self) -> i32 {
        self.endbit
    }

    /// C++ `ContextField::getSignBit`.
    pub fn get_sign_bit(&self) -> bool {
        self.signbit
    }

    /// C++ `ContextField::getValue`.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        let mut res = context_bytes(walker, self.startbyte, self.endbyte)?;
        // intb >> int4: same x86-masked transcription as TokenField (ADR 0003)
        res = res.wshr(self.shift as u32); // count masked mod 64 anyway
        if self.signbit {
            res = sign_extend(res, self.endbit - self.startbit);
        } else {
            res = zero_extend(res, self.endbit - self.startbit);
        }
        Ok(res)
    }

    /// C++ `ContextField::minValue`.
    pub fn min_value(&self) -> i64 {
        0
    }

    /// C++ `ContextField::maxValue`: `zero_extend(~(intb)0, endbit-startbit)`.
    pub fn max_value(&self) -> i64 {
        let res: i64 = 0;
        zero_extend(!res, self.endbit - self.startbit)
    }

    /// C++ `ContextField::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_CONTEXTFIELD);
        encoder.write_bool(&sla::ATTRIB_SIGNBIT, self.signbit);
        encoder.write_signed_integer(&sla::ATTRIB_STARTBIT, i64::from(self.startbit));
        encoder.write_signed_integer(&sla::ATTRIB_ENDBIT, i64::from(self.endbit));
        encoder.write_signed_integer(&sla::ATTRIB_STARTBYTE, i64::from(self.startbyte));
        encoder.write_signed_integer(&sla::ATTRIB_ENDBYTE, i64::from(self.endbyte));
        encoder.write_signed_integer(&sla::ATTRIB_SHIFT, i64::from(self.shift));
        encoder.close_element(&sla::ELEM_CONTEXTFIELD);
    }

    /// C++ `ContextField::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<ContextField> {
        let el = decoder.open_element_id(&sla::ELEM_CONTEXTFIELD)?;
        let signbit = decoder.read_bool_id(&sla::ATTRIB_SIGNBIT)?;
        let startbit = decoder.read_signed_integer_id(&sla::ATTRIB_STARTBIT)? as i32; // C++ implicit intb -> int4
        let endbit = decoder.read_signed_integer_id(&sla::ATTRIB_ENDBIT)? as i32; // C++ implicit intb -> int4
        let startbyte = decoder.read_signed_integer_id(&sla::ATTRIB_STARTBYTE)? as i32; // C++ implicit intb -> int4
        let endbyte = decoder.read_signed_integer_id(&sla::ATTRIB_ENDBYTE)? as i32; // C++ implicit intb -> int4
        let shift = decoder.read_signed_integer_id(&sla::ATTRIB_SHIFT)? as i32; // C++ implicit intb -> int4
        decoder.close_element(el)?;
        Ok(ContextField {
            startbit,
            endbit,
            startbyte,
            endbyte,
            shift,
            signbit,
        })
    }
}

/// C++ `ConstantValue`.
#[derive(Debug, Clone)]
pub struct ConstantValue {
    val: i64,
}

impl ConstantValue {
    /// C++ `ConstantValue(intb v)`.
    pub fn new(v: i64) -> ConstantValue {
        ConstantValue { val: v }
    }

    /// C++ `ConstantValue::getValue` (ignores the walker).
    pub fn get_value(&self) -> i64 {
        self.val
    }

    /// C++ `ConstantValue::minValue`.
    pub fn min_value(&self) -> i64 {
        self.val
    }

    /// C++ `ConstantValue::maxValue`.
    pub fn max_value(&self) -> i64 {
        self.val
    }

    /// C++ `ConstantValue::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_INTB);
        encoder.write_signed_integer(&sla::ATTRIB_VAL, self.val);
        encoder.close_element(&sla::ELEM_INTB);
    }

    /// C++ `ConstantValue::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<ConstantValue> {
        let el = decoder.open_element_id(&sla::ELEM_INTB)?;
        let val = decoder.read_signed_integer_id(&sla::ATTRIB_VAL)?;
        decoder.close_element(el)?;
        Ok(ConstantValue { val })
    }
}

/// Shared body of the three instruction-address values: C++
/// `(intb)AddrSpace::byteToAddress(addr.getOffset(),
/// addr.getSpace()->getWordSize())`.
fn address_value(addr: &Address) -> i64 {
    let spc = addr
        .get_space()
        .expect("instruction address has no space (C++ dereferences the space unconditionally)");
    // (intb) cast of the uintb result: two's-complement reinterpret
    AddrSpace::byte_to_address(addr.get_offset(), spc.get_word_size()) as i64
}

/// C++ `StartInstructionValue`: the address of the current instruction.
#[derive(Debug, Clone, Default)]
pub struct StartInstructionValue;

impl StartInstructionValue {
    /// C++ `StartInstructionValue::getValue`.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        Ok(address_value(&walker.get_addr()))
    }

    /// C++ `StartInstructionValue::minValue`.
    pub fn min_value(&self) -> i64 {
        0
    }

    /// C++ `StartInstructionValue::maxValue`.
    pub fn max_value(&self) -> i64 {
        0
    }

    /// C++ `StartInstructionValue::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_START_EXP);
        encoder.close_element(&sla::ELEM_START_EXP);
    }

    /// C++ `StartInstructionValue::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<StartInstructionValue> {
        let el = decoder.open_element_id(&sla::ELEM_START_EXP)?;
        decoder.close_element(el)?;
        Ok(StartInstructionValue)
    }
}

/// C++ `EndInstructionValue`: the address of the next instruction.
#[derive(Debug, Clone, Default)]
pub struct EndInstructionValue;

impl EndInstructionValue {
    /// C++ `EndInstructionValue::getValue`.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        Ok(address_value(&walker.get_naddr()))
    }

    /// C++ `EndInstructionValue::minValue`.
    pub fn min_value(&self) -> i64 {
        0
    }

    /// C++ `EndInstructionValue::maxValue`.
    pub fn max_value(&self) -> i64 {
        0
    }

    /// C++ `EndInstructionValue::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_END_EXP);
        encoder.close_element(&sla::ELEM_END_EXP);
    }

    /// C++ `EndInstructionValue::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<EndInstructionValue> {
        let el = decoder.open_element_id(&sla::ELEM_END_EXP)?;
        decoder.close_element(el)?;
        Ok(EndInstructionValue)
    }
}

/// C++ `Next2InstructionValue`: the address of the instruction after the
/// next.  NOTE: like upstream, the [`PatternExpression::decode_expression`]
/// factory does NOT recognize `next2_exp` (the C++ factory omits it); the
/// symbol-table wave constructs this value directly when decoding a
/// `Next2Symbol`.
#[derive(Debug, Clone, Default)]
pub struct Next2InstructionValue;

impl Next2InstructionValue {
    /// C++ `Next2InstructionValue::getValue`.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        Ok(address_value(&walker.get_n2addr()?))
    }

    /// C++ `Next2InstructionValue::minValue`.
    pub fn min_value(&self) -> i64 {
        0
    }

    /// C++ `Next2InstructionValue::maxValue`.
    pub fn max_value(&self) -> i64 {
        0
    }

    /// C++ `Next2InstructionValue::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_NEXT2_EXP);
        encoder.close_element(&sla::ELEM_NEXT2_EXP);
    }

    /// C++ `Next2InstructionValue::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<Next2InstructionValue> {
        let el = decoder.open_element_id(&sla::ELEM_NEXT2_EXP)?;
        decoder.close_element(el)?;
        Ok(Next2InstructionValue)
    }
}

/// C++ `OperandValue`: the value of another operand of the same
/// constructor, used inside an expression.  C++ caches a `Constructor *ct`;
/// the port stores the `(table_id, ct_id)` pair the C++ decode resolves the
/// pointer from (see module docs for the seam).
#[derive(Debug, Clone)]
pub struct OperandValue {
    /// This is the defining field of expression (C++ `index`)
    index: i32,
    /// Id of the SubtableSymbol owning the constructor (C++
    /// `ct->getParent()->getId()`)
    table_id: u32,
    /// Id of the constructor within its table (C++ `ct->getId()`)
    ct_id: u32,
}

impl OperandValue {
    /// C++ `OperandValue(int4 ind,Constructor *c)`, with the constructor
    /// identified by ids instead of a pointer.
    pub fn new(index: i32, table_id: u32, ct_id: u32) -> OperandValue {
        OperandValue {
            index,
            table_id,
            ct_id,
        }
    }

    /// C++ `OperandValue::changeIndex`.
    pub fn change_index(&mut self, newind: i32) {
        self.index = newind;
    }

    /// The operand index within the constructor.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// The id of the SubtableSymbol owning the constructor.
    pub fn table_id(&self) -> u32 {
        self.table_id
    }

    /// The id of the constructor within its table.
    pub fn ct_id(&self) -> u32 {
        self.ct_id
    }

    /// C++ `OperandValue::getValue` — the body lives behind the
    /// [`PatternExpressionContext::operand_value`] seam (see its docs for
    /// the exact C++ code the implementor transcribes).
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        walker.operand_value(self.index, self.table_id, self.ct_id)
    }

    /// C++ `OperandValue::minValue`: `throw SleighError(...)`.
    pub fn min_value(&self) -> KunaResult<i64> {
        Err(KunaError::sleigh("Operand used in pattern expression"))
    }

    /// C++ `OperandValue::maxValue`: `throw SleighError(...)`.
    pub fn max_value(&self) -> KunaResult<i64> {
        Err(KunaError::sleigh("Operand used in pattern expression"))
    }

    /// C++ `OperandValue::getSubValue` evaluates the operand's *defining
    /// expression* (`sym->getDefiningExpression()->getSubValue(...)`), which
    /// requires the symbol table.  It is only reachable from the unported
    /// SLEIGH-compiler equations, so the port reports an error instead.
    pub fn get_sub_value(&self, _replace: &[i64], _listpos: &mut i32) -> KunaResult<i64> {
        Err(KunaError::sleigh(
            "OperandValue::getSubValue requires the SLEIGH compiler symbol table (not ported)",
        ))
    }

    /// C++ `OperandValue::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_OPERAND_EXP);
        encoder.write_signed_integer(&sla::ATTRIB_INDEX, i64::from(self.index));
        encoder.write_unsigned_integer(&sla::ATTRIB_TABLE, u64::from(self.table_id));
        encoder.write_unsigned_integer(&sla::ATTRIB_CT, u64::from(self.ct_id)); // Save id of our constructor
        encoder.close_element(&sla::ELEM_OPERAND_EXP);
    }

    /// C++ `OperandValue::decode`, with the symbol-table lookup behind the
    /// [`OperandValueResolver`] seam.
    pub fn decode(
        decoder: &mut dyn Decoder,
        trans: &dyn OperandValueResolver,
    ) -> KunaResult<OperandValue> {
        let el = decoder.open_element_id(&sla::ELEM_OPERAND_EXP)?;
        let index = decoder.read_signed_integer_id(&sla::ATTRIB_INDEX)? as i32; // C++ implicit intb -> int4
        let tabid = decoder.read_unsigned_integer_id(&sla::ATTRIB_TABLE)? as u32; // C++ implicit uintb -> uintm
        let ctid = decoder.read_unsigned_integer_id(&sla::ATTRIB_CT)? as u32; // C++ implicit uintb -> uintm
        let numct = trans.num_constructors(tabid)?;
        // C++ `ctid >= tab->getNumConstructors()` compares uintm against
        // int4: the int4 converts to uintm (usual arithmetic conversions)
        if ctid >= numct as u32 {
            return Err(KunaError::decoder("Invalid constructor id"));
        }
        decoder.close_element(el)?;
        Ok(OperandValue {
            index,
            table_id: tabid,
            ct_id: ctid,
        })
    }
}

// ---------------------------------------------------------------------------
// PatternValue — the C++ `PatternValue` abstract class
// ---------------------------------------------------------------------------

/// The C++ `PatternValue` subclasses as an enum (see module docs).
#[derive(Debug, Clone)]
pub enum PatternValue {
    TokenField(TokenField),
    ContextField(ContextField),
    ConstantValue(ConstantValue),
    OperandValue(OperandValue),
    StartInstructionValue(StartInstructionValue),
    EndInstructionValue(EndInstructionValue),
    Next2InstructionValue(Next2InstructionValue),
}

impl PatternValue {
    /// C++ virtual `getValue` dispatch over the `PatternValue` subclasses.
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        match self {
            PatternValue::TokenField(v) => v.get_value(walker),
            PatternValue::ContextField(v) => v.get_value(walker),
            PatternValue::ConstantValue(v) => Ok(v.get_value()),
            PatternValue::OperandValue(v) => v.get_value(walker),
            PatternValue::StartInstructionValue(v) => v.get_value(walker),
            PatternValue::EndInstructionValue(v) => v.get_value(walker),
            PatternValue::Next2InstructionValue(v) => v.get_value(walker),
        }
    }

    /// C++ virtual `minValue` (Result because `OperandValue::minValue`
    /// throws `SleighError`).
    pub fn min_value(&self) -> KunaResult<i64> {
        match self {
            PatternValue::TokenField(v) => Ok(v.min_value()),
            PatternValue::ContextField(v) => Ok(v.min_value()),
            PatternValue::ConstantValue(v) => Ok(v.min_value()),
            PatternValue::OperandValue(v) => v.min_value(),
            PatternValue::StartInstructionValue(v) => Ok(v.min_value()),
            PatternValue::EndInstructionValue(v) => Ok(v.min_value()),
            PatternValue::Next2InstructionValue(v) => Ok(v.min_value()),
        }
    }

    /// C++ virtual `maxValue` (Result because `OperandValue::maxValue`
    /// throws `SleighError`).
    pub fn max_value(&self) -> KunaResult<i64> {
        match self {
            PatternValue::TokenField(v) => Ok(v.max_value()),
            PatternValue::ContextField(v) => Ok(v.max_value()),
            PatternValue::ConstantValue(v) => Ok(v.max_value()),
            PatternValue::OperandValue(v) => v.max_value(),
            PatternValue::StartInstructionValue(v) => Ok(v.max_value()),
            PatternValue::EndInstructionValue(v) => Ok(v.max_value()),
            PatternValue::Next2InstructionValue(v) => Ok(v.max_value()),
        }
    }

    /// C++ `PatternValue::getSubValue`: `return replace[listpos++]`
    /// (overridden by `OperandValue`).
    pub fn get_sub_value(&self, replace: &[i64], listpos: &mut i32) -> KunaResult<i64> {
        match self {
            PatternValue::OperandValue(v) => v.get_sub_value(replace, listpos),
            _ => {
                // C++ replace[listpos++]: out-of-range indexing is UB in
                // C++, an indexing panic here (ADR 0004)
                let v = replace[*listpos as usize]; // listpos >= 0 by construction
                *listpos += 1;
                Ok(v)
            }
        }
    }

    /// C++ virtual `encode` dispatch.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        match self {
            PatternValue::TokenField(v) => v.encode(encoder),
            PatternValue::ContextField(v) => v.encode(encoder),
            PatternValue::ConstantValue(v) => v.encode(encoder),
            PatternValue::OperandValue(v) => v.encode(encoder),
            PatternValue::StartInstructionValue(v) => v.encode(encoder),
            PatternValue::EndInstructionValue(v) => v.encode(encoder),
            PatternValue::Next2InstructionValue(v) => v.encode(encoder),
        }
    }
}

// ---------------------------------------------------------------------------
// Binary / Unary expressions
// ---------------------------------------------------------------------------

/// C++ `BinaryExpression` base: owns its two child expressions (the C++
/// refcount/layClaim protocol is replaced by ownership).
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    left: Box<PatternExpression>,
    right: Box<PatternExpression>,
}

impl BinaryExpression {
    /// C++ `BinaryExpression(PatternExpression *l,PatternExpression *r)`.
    pub fn new(l: PatternExpression, r: PatternExpression) -> BinaryExpression {
        BinaryExpression {
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    /// C++ `BinaryExpression::getLeft`.
    pub fn get_left(&self) -> &PatternExpression {
        &self.left
    }

    /// C++ `BinaryExpression::getRight`.
    pub fn get_right(&self) -> &PatternExpression {
        &self.right
    }

    /// C++ `BinaryExpression::encode` (outer tag is generated by the
    /// dispatching variant).
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        self.left.encode(encoder);
        self.right.encode(encoder);
    }

    /// C++ `BinaryExpression::decode`: generic `openElement()` (the outer
    /// tag was already identified by the factory peek), two child
    /// expressions, close.
    pub fn decode(
        decoder: &mut dyn Decoder,
        trans: &dyn OperandValueResolver,
    ) -> KunaResult<BinaryExpression> {
        let el = decoder.open_element()?;
        let left = PatternExpression::decode_expression(decoder, trans)?;
        let right = PatternExpression::decode_expression(decoder, trans)?;
        decoder.close_element(el)?;
        Ok(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

/// C++ `UnaryExpression` base: owns its child expression.
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    unary: Box<PatternExpression>,
}

impl UnaryExpression {
    /// C++ `UnaryExpression(PatternExpression *u)`.
    pub fn new(u: PatternExpression) -> UnaryExpression {
        UnaryExpression { unary: Box::new(u) }
    }

    /// C++ `UnaryExpression::getUnary`.
    pub fn get_unary(&self) -> &PatternExpression {
        &self.unary
    }

    /// C++ `UnaryExpression::encode` (outer tag is generated by the
    /// dispatching variant).
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        self.unary.encode(encoder);
    }

    /// C++ `UnaryExpression::decode`.
    pub fn decode(
        decoder: &mut dyn Decoder,
        trans: &dyn OperandValueResolver,
    ) -> KunaResult<UnaryExpression> {
        let el = decoder.open_element()?;
        let unary = PatternExpression::decode_expression(decoder, trans)?;
        decoder.close_element(el)?;
        Ok(UnaryExpression {
            unary: Box::new(unary),
        })
    }
}

// ---------------------------------------------------------------------------
// PatternExpression — the C++ class hierarchy root
// ---------------------------------------------------------------------------

/// The C++ `PatternExpression` hierarchy as an enum: a `PatternValue` leaf
/// or one of the operator nodes.
#[derive(Debug, Clone)]
pub enum PatternExpression {
    Value(PatternValue),
    Plus(BinaryExpression),
    Sub(BinaryExpression),
    Mult(BinaryExpression),
    LeftShift(BinaryExpression),
    RightShift(BinaryExpression),
    And(BinaryExpression),
    Or(BinaryExpression),
    Xor(BinaryExpression),
    Div(BinaryExpression),
    Minus(UnaryExpression),
    Not(UnaryExpression),
}

impl PatternExpression {
    /// C++ virtual `getValue(ParserWalker&)`.
    ///
    /// Arithmetic notes (ADR 0003): the C++ operates on `intb` where signed
    /// overflow is UB but the oracle hardware wraps; the port uses the
    /// explicit wrapping helpers.  Shift counts are evaluated expression
    /// values and resolve x86-masked; `Div` panics on a zero divisor (C++
    /// SIGFPE/UB, an internal invariant violation per ADR 0004).
    pub fn get_value(&self, walker: &dyn PatternExpressionContext) -> KunaResult<i64> {
        match self {
            PatternExpression::Value(v) => v.get_value(walker),
            PatternExpression::Plus(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval.wadd(rightval))
            }
            PatternExpression::Sub(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval.wsub(rightval))
            }
            PatternExpression::Mult(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval.wmul(rightval))
            }
            PatternExpression::LeftShift(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                // intb << intb: count truncated then masked mod 64 (x86)
                Ok(leftval.wshl(rightval as u32))
            }
            PatternExpression::RightShift(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                // intb >> intb: arithmetic shift; count truncated then
                // masked mod 64 (x86)
                Ok(leftval.wshr(rightval as u32))
            }
            PatternExpression::And(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval & rightval)
            }
            PatternExpression::Or(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval | rightval)
            }
            PatternExpression::Xor(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval ^ rightval)
            }
            PatternExpression::Div(b) => {
                let leftval = b.get_left().get_value(walker)?;
                let rightval = b.get_right().get_value(walker)?;
                Ok(leftval.wdiv(rightval))
            }
            PatternExpression::Minus(u) => {
                let val = u.get_unary().get_value(walker)?;
                Ok(val.wneg())
            }
            PatternExpression::Not(u) => {
                let val = u.get_unary().get_value(walker)?;
                Ok(!val)
            }
        }
    }

    /// C++ virtual `listValues`: collect the `PatternValue` leaves in
    /// depth-first, left-to-right order.
    pub fn list_values<'a>(&'a self, list: &mut Vec<&'a PatternValue>) {
        match self {
            PatternExpression::Value(v) => list.push(v),
            // BinaryExpression::listValues: left then right
            PatternExpression::Plus(b)
            | PatternExpression::Sub(b)
            | PatternExpression::Mult(b)
            | PatternExpression::LeftShift(b)
            | PatternExpression::RightShift(b)
            | PatternExpression::And(b)
            | PatternExpression::Or(b)
            | PatternExpression::Xor(b)
            | PatternExpression::Div(b) => {
                b.get_left().list_values(list);
                b.get_right().list_values(list);
            }
            // UnaryExpression::listValues
            PatternExpression::Minus(u) | PatternExpression::Not(u) => {
                u.get_unary().list_values(list);
            }
        }
    }

    /// C++ virtual `getMinMax`: push min/max for every leaf in `listValues`
    /// order.
    pub fn get_min_max(&self, minlist: &mut Vec<i64>, maxlist: &mut Vec<i64>) -> KunaResult<()> {
        match self {
            PatternExpression::Value(v) => {
                minlist.push(v.min_value()?);
                maxlist.push(v.max_value()?);
                Ok(())
            }
            // BinaryExpression::getMinMax: left then right
            PatternExpression::Plus(b)
            | PatternExpression::Sub(b)
            | PatternExpression::Mult(b)
            | PatternExpression::LeftShift(b)
            | PatternExpression::RightShift(b)
            | PatternExpression::And(b)
            | PatternExpression::Or(b)
            | PatternExpression::Xor(b)
            | PatternExpression::Div(b) => {
                b.get_left().get_min_max(minlist, maxlist)?;
                b.get_right().get_min_max(minlist, maxlist)
            }
            // UnaryExpression::getMinMax
            PatternExpression::Minus(u) | PatternExpression::Not(u) => {
                u.get_unary().get_min_max(minlist, maxlist)
            }
        }
    }

    /// C++ virtual `getSubValue(const vector<intb>&,int4&)`: re-evaluate the
    /// expression substituting `replace[..]` for the leaves in `listValues`
    /// order.  Arithmetic transcription matches [`Self::get_value`].
    pub fn get_sub_value(&self, replace: &[i64], listpos: &mut i32) -> KunaResult<i64> {
        match self {
            PatternExpression::Value(v) => v.get_sub_value(replace, listpos),
            PatternExpression::Plus(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval.wadd(rightval))
            }
            PatternExpression::Sub(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval.wsub(rightval))
            }
            PatternExpression::Mult(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval.wmul(rightval))
            }
            PatternExpression::LeftShift(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                // intb << intb: count truncated then masked mod 64 (x86)
                Ok(leftval.wshl(rightval as u32))
            }
            PatternExpression::RightShift(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                // intb >> intb: arithmetic; count truncated, masked mod 64
                Ok(leftval.wshr(rightval as u32))
            }
            PatternExpression::And(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval & rightval)
            }
            PatternExpression::Or(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval | rightval)
            }
            PatternExpression::Xor(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval ^ rightval)
            }
            PatternExpression::Div(b) => {
                let leftval = b.get_left().get_sub_value(replace, listpos)?; // Must be left first
                let rightval = b.get_right().get_sub_value(replace, listpos)?;
                Ok(leftval.wdiv(rightval))
            }
            PatternExpression::Minus(u) => {
                let val = u.get_unary().get_sub_value(replace, listpos)?;
                Ok(val.wneg())
            }
            PatternExpression::Not(u) => {
                let val = u.get_unary().get_sub_value(replace, listpos)?;
                Ok(!val)
            }
        }
    }

    /// C++ non-virtual `getSubValue(const vector<intb>&)`: start the leaf
    /// cursor at 0.
    pub fn get_sub_value_root(&self, replace: &[i64]) -> KunaResult<i64> {
        let mut listpos: i32 = 0;
        self.get_sub_value(replace, &mut listpos)
    }

    /// C++ virtual `encode` dispatch (each operator writes its own outer
    /// tag, then the Binary/Unary base encodes the children).
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        match self {
            PatternExpression::Value(v) => v.encode(encoder),
            PatternExpression::Plus(b) => {
                encoder.open_element(&sla::ELEM_PLUS_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_PLUS_EXP);
            }
            PatternExpression::Sub(b) => {
                encoder.open_element(&sla::ELEM_SUB_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_SUB_EXP);
            }
            PatternExpression::Mult(b) => {
                encoder.open_element(&sla::ELEM_MULT_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_MULT_EXP);
            }
            PatternExpression::LeftShift(b) => {
                encoder.open_element(&sla::ELEM_LSHIFT_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_LSHIFT_EXP);
            }
            PatternExpression::RightShift(b) => {
                encoder.open_element(&sla::ELEM_RSHIFT_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_RSHIFT_EXP);
            }
            PatternExpression::And(b) => {
                encoder.open_element(&sla::ELEM_AND_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_AND_EXP);
            }
            PatternExpression::Or(b) => {
                encoder.open_element(&sla::ELEM_OR_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_OR_EXP);
            }
            PatternExpression::Xor(b) => {
                encoder.open_element(&sla::ELEM_XOR_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_XOR_EXP);
            }
            PatternExpression::Div(b) => {
                encoder.open_element(&sla::ELEM_DIV_EXP);
                b.encode(encoder);
                encoder.close_element(&sla::ELEM_DIV_EXP);
            }
            PatternExpression::Minus(u) => {
                encoder.open_element(&sla::ELEM_MINUS_EXP);
                u.encode(encoder);
                encoder.close_element(&sla::ELEM_MINUS_EXP);
            }
            PatternExpression::Not(u) => {
                encoder.open_element(&sla::ELEM_NOT_EXP);
                u.encode(encoder);
                encoder.close_element(&sla::ELEM_NOT_EXP);
            }
        }
    }

    /// C++ `PatternExpression::decodeExpression`: factory keyed by the
    /// peeked ElementId.  NOTE (faithful to upstream): `next2_exp` is NOT
    /// recognized here — the C++ factory omits it; see
    /// [`Next2InstructionValue`].
    pub fn decode_expression(
        decoder: &mut dyn Decoder,
        trans: &dyn OperandValueResolver,
    ) -> KunaResult<PatternExpression> {
        let el = decoder.peek_element()?;
        if el == sla::ELEM_TOKENFIELD {
            Ok(PatternExpression::Value(PatternValue::TokenField(
                TokenField::decode(decoder)?,
            )))
        } else if el == sla::ELEM_CONTEXTFIELD {
            Ok(PatternExpression::Value(PatternValue::ContextField(
                ContextField::decode(decoder)?,
            )))
        } else if el == sla::ELEM_INTB {
            Ok(PatternExpression::Value(PatternValue::ConstantValue(
                ConstantValue::decode(decoder)?,
            )))
        } else if el == sla::ELEM_OPERAND_EXP {
            Ok(PatternExpression::Value(PatternValue::OperandValue(
                OperandValue::decode(decoder, trans)?,
            )))
        } else if el == sla::ELEM_START_EXP {
            Ok(PatternExpression::Value(
                PatternValue::StartInstructionValue(StartInstructionValue::decode(decoder)?),
            ))
        } else if el == sla::ELEM_END_EXP {
            Ok(PatternExpression::Value(PatternValue::EndInstructionValue(
                EndInstructionValue::decode(decoder)?,
            )))
        } else if el == sla::ELEM_PLUS_EXP {
            Ok(PatternExpression::Plus(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_SUB_EXP {
            Ok(PatternExpression::Sub(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_MULT_EXP {
            Ok(PatternExpression::Mult(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_LSHIFT_EXP {
            Ok(PatternExpression::LeftShift(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_RSHIFT_EXP {
            Ok(PatternExpression::RightShift(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_AND_EXP {
            Ok(PatternExpression::And(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_OR_EXP {
            Ok(PatternExpression::Or(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_XOR_EXP {
            Ok(PatternExpression::Xor(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_DIV_EXP {
            Ok(PatternExpression::Div(BinaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_MINUS_EXP {
            Ok(PatternExpression::Minus(UnaryExpression::decode(
                decoder, trans,
            )?))
        } else if el == sla::ELEM_NOT_EXP {
            Ok(PatternExpression::Not(UnaryExpression::decode(
                decoder, trans,
            )?))
        } else {
            Err(KunaError::decoder("Invalid pattern expression element"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace};

    use super::*;

    // -- synthetic walker ----------------------------------------------------

    /// Synthetic byte/context provider mirroring the documented
    /// `ParserContext::getInstructionBytes`/`getContextBytes` semantics.
    struct TestWalker {
        instr: Vec<u8>,
        ctx: Vec<u32>,
        addr: Option<Address>,
        naddr: Option<Address>,
        n2addr: Option<Address>,
        /// canned answers for operand_value, keyed by (index, table, ct)
        operands: Vec<((i32, u32, u32), i64)>,
    }

    impl TestWalker {
        fn from_bytes(instr: &[u8]) -> TestWalker {
            TestWalker {
                instr: instr.to_vec(),
                ctx: Vec::new(),
                addr: None,
                naddr: None,
                n2addr: None,
                operands: Vec::new(),
            }
        }

        fn from_context(ctx: &[u32]) -> TestWalker {
            TestWalker {
                instr: Vec::new(),
                ctx: ctx.to_vec(),
                addr: None,
                naddr: None,
                n2addr: None,
                operands: Vec::new(),
            }
        }
    }

    impl PatternExpressionContext for TestWalker {
        fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
            // mirror ParserContext::getInstructionBytes with point->offset=0:
            // big-endian packing, error past the buffer (BadDataError analog)
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
            // mirror ParserContext::getContextBytes over the ctx word array
            let intstart = (bytestart / 4) as usize;
            let mut res: u32 = *self.ctx.get(intstart).unwrap_or(&0);
            let byte_offset = bytestart % 4;
            let unused_bytes = 4 - size;
            res = res.wshl((byte_offset * 8) as u32);
            res = res.wshr((unused_bytes * 8) as u32);
            let remaining = size - 4 + byte_offset;
            if remaining > 0 && intstart + 1 < self.ctx.len() {
                let mut res2 = self.ctx[intstart + 1];
                let unused2 = 4 - remaining;
                res2 = res2.wshr((unused2 * 8) as u32);
                res |= res2;
            }
            Ok(res)
        }

        fn get_addr(&self) -> Address {
            self.addr.clone().expect("test walker addr unset")
        }

        fn get_naddr(&self) -> Address {
            self.naddr.clone().expect("test walker naddr unset")
        }

        fn get_n2addr(&self) -> KunaResult<Address> {
            match &self.n2addr {
                Some(a) => Ok(a.clone()),
                None => Err(KunaError::lowlevel(
                    "inst_next2 not available in this context",
                )),
            }
        }

        fn operand_value(&self, index: i32, table_id: u32, ct_id: u32) -> KunaResult<i64> {
            for ((i, t, c), v) in &self.operands {
                if *i == index && *t == table_id && *c == ct_id {
                    return Ok(*v);
                }
            }
            Err(KunaError::sleigh("unknown operand in test walker"))
        }
    }

    /// Permissive resolver: every table reports the given constructor count.
    struct FixedResolver(i32);

    impl OperandValueResolver for FixedResolver {
        fn num_constructors(&self, _table_id: u32) -> KunaResult<i32> {
            Ok(self.0)
        }
    }

    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        // wordsize 2 so byteToAddress is observable (offset/2)
        let spc = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            2,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(spc)).unwrap();
        manager
    }

    fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("ram").unwrap())
    }

    fn cval(v: i64) -> PatternExpression {
        PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(v)))
    }

    // -- TokenField ------------------------------------------------------------

    #[test]
    fn tokenfield_little_endian_byte_select() {
        // LE token of 2 bytes: bits [0,7] live in byte 0, bits [8,15] in byte 1
        let walker = TestWalker::from_bytes(&[0xab, 0xcd]);
        let lo = TokenField::new(2, false, false, 0, 7);
        let hi = TokenField::new(2, false, false, 8, 15);
        assert_eq!(lo.get_value(&walker).unwrap(), 0xab);
        assert_eq!(hi.get_value(&walker).unwrap(), 0xcd);
    }

    #[test]
    fn tokenfield_big_endian_byte_select() {
        // BE token of 2 bytes: bits [0,7] are the LEAST significant -> byte 1
        let walker = TestWalker::from_bytes(&[0xab, 0xcd]);
        let lo = TokenField::new(2, true, false, 0, 7);
        let hi = TokenField::new(2, true, false, 8, 15);
        assert_eq!(lo.get_value(&walker).unwrap(), 0xcd);
        assert_eq!(hi.get_value(&walker).unwrap(), 0xab);
    }

    #[test]
    fn tokenfield_subbyte_shift_and_signextend() {
        // byte 0 = 0b1111_0110: field bits [1,3] (LE) = 0b011 = 3 unsigned
        let walker = TestWalker::from_bytes(&[0b1111_0110]);
        let f = TokenField::new(1, false, false, 1, 3);
        assert_eq!(f.get_value(&walker).unwrap(), 3);
        // bits [4,7] = 0b1111 -> signed = -1
        let s = TokenField::new(1, false, true, 4, 7);
        assert_eq!(s.get_value(&walker).unwrap(), -1);
        // same field unsigned = 15
        let u = TokenField::new(1, false, false, 4, 7);
        assert_eq!(u.get_value(&walker).unwrap(), 15);
    }

    #[test]
    fn tokenfield_multiword_value() {
        // 6-byte big-endian field crosses the 4-byte uintm packing boundary
        let walker = TestWalker::from_bytes(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        let f = TokenField::new(6, true, false, 0, 47);
        assert_eq!(f.get_value(&walker).unwrap(), 0x0102_0304_0506);
        // little-endian: bytes swapped over the full 6-byte size
        let g = TokenField::new(6, false, false, 0, 47);
        assert_eq!(g.get_value(&walker).unwrap(), 0x0605_0403_0201);
    }

    #[test]
    fn tokenfield_minmax() {
        let f = TokenField::new(2, false, false, 3, 10);
        assert_eq!(f.min_value(), 0);
        assert_eq!(f.max_value(), 0xff); // 8-bit field: zero_extend(~0, 7)
    }

    #[test]
    fn tokenfield_error_propagates_past_buffer() {
        let walker = TestWalker::from_bytes(&[0xab]);
        let f = TokenField::new(8, true, false, 0, 63);
        assert!(f.get_value(&walker).is_err());
    }

    // -- ContextField ------------------------------------------------------------

    #[test]
    fn contextfield_extraction() {
        let walker = TestWalker::from_context(&[0xabcd_1234]);
        // bits [0,7]: most significant byte of the first context word
        let f = ContextField::new(false, 0, 7);
        assert_eq!(f.get_value(&walker).unwrap(), 0xab);
        // bits [8,15]
        let g = ContextField::new(false, 8, 15);
        assert_eq!(g.get_value(&walker).unwrap(), 0xcd);
        // sub-byte bits [4,7] = low nibble of 0xab = 0xb
        let h = ContextField::new(false, 4, 7);
        assert_eq!(h.get_value(&walker).unwrap(), 0xb);
        // signed nibble 0xb -> -5
        let s = ContextField::new(true, 4, 7);
        assert_eq!(s.get_value(&walker).unwrap(), -5);
    }

    #[test]
    fn contextfield_accessors_and_minmax() {
        let f = ContextField::new(true, 4, 11);
        assert_eq!(f.get_start_bit(), 4);
        assert_eq!(f.get_end_bit(), 11);
        assert!(f.get_sign_bit());
        assert_eq!(f.min_value(), 0);
        assert_eq!(f.max_value(), 0xff); // 8-bit field [4,11]: zero_extend(~0, 7)
    }

    // -- instruction address values ----------------------------------------------

    #[test]
    fn start_end_next2_instruction_values() {
        let manager = test_manager();
        let mut walker = TestWalker::from_bytes(&[]);
        walker.addr = Some(Address::new(ram(&manager), 0x1000));
        walker.naddr = Some(Address::new(ram(&manager), 0x1004));
        walker.n2addr = Some(Address::new(ram(&manager), 0x1008));
        // wordsize 2: byteToAddress halves the byte offset
        assert_eq!(StartInstructionValue.get_value(&walker).unwrap(), 0x800);
        assert_eq!(EndInstructionValue.get_value(&walker).unwrap(), 0x802);
        assert_eq!(Next2InstructionValue.get_value(&walker).unwrap(), 0x804);
        // min/max are all zero
        assert_eq!(StartInstructionValue.min_value(), 0);
        assert_eq!(EndInstructionValue.max_value(), 0);
        // unavailable inst_next2 surfaces the C++ LowlevelError
        walker.n2addr = None;
        assert!(Next2InstructionValue.get_value(&walker).is_err());
    }

    // -- OperandValue ------------------------------------------------------------

    #[test]
    fn operand_value_via_seam() {
        let mut walker = TestWalker::from_bytes(&[]);
        walker.operands.push(((2, 7, 3), 0x42));
        let ov = OperandValue::new(2, 7, 3);
        assert_eq!(ov.get_value(&walker).unwrap(), 0x42);
        // min/max throw SleighError in C++
        assert!(ov.min_value().is_err());
        assert!(ov.max_value().is_err());
        let mut pos = 0;
        assert!(ov.get_sub_value(&[1, 2], &mut pos).is_err());
    }

    // -- expression evaluation -----------------------------------------------------

    #[test]
    fn expression_arithmetic_evaluation() {
        let walker = TestWalker::from_bytes(&[]);
        let plus = PatternExpression::Plus(BinaryExpression::new(cval(5), cval(7)));
        assert_eq!(plus.get_value(&walker).unwrap(), 12);
        let sub = PatternExpression::Sub(BinaryExpression::new(cval(5), cval(7)));
        assert_eq!(sub.get_value(&walker).unwrap(), -2);
        let mult = PatternExpression::Mult(BinaryExpression::new(cval(6), cval(-7)));
        assert_eq!(mult.get_value(&walker).unwrap(), -42);
        let lsh = PatternExpression::LeftShift(BinaryExpression::new(cval(3), cval(4)));
        assert_eq!(lsh.get_value(&walker).unwrap(), 48);
        let rsh = PatternExpression::RightShift(BinaryExpression::new(cval(-16), cval(2)));
        assert_eq!(rsh.get_value(&walker).unwrap(), -4); // arithmetic shift
        let and = PatternExpression::And(BinaryExpression::new(cval(0xff0), cval(0x0ff)));
        assert_eq!(and.get_value(&walker).unwrap(), 0x0f0);
        let or = PatternExpression::Or(BinaryExpression::new(cval(0xf00), cval(0x00f)));
        assert_eq!(or.get_value(&walker).unwrap(), 0xf0f);
        let xor = PatternExpression::Xor(BinaryExpression::new(cval(0xff), cval(0x0f)));
        assert_eq!(xor.get_value(&walker).unwrap(), 0xf0);
        let div = PatternExpression::Div(BinaryExpression::new(cval(-42), cval(5)));
        assert_eq!(div.get_value(&walker).unwrap(), -8); // C++ truncating division
        let minus = PatternExpression::Minus(UnaryExpression::new(cval(13)));
        assert_eq!(minus.get_value(&walker).unwrap(), -13);
        let not = PatternExpression::Not(UnaryExpression::new(cval(0)));
        assert_eq!(not.get_value(&walker).unwrap(), -1);
    }

    #[test]
    fn expression_wrapping_matches_two_complement() {
        let walker = TestWalker::from_bytes(&[]);
        let plus = PatternExpression::Plus(BinaryExpression::new(cval(i64::MAX), cval(1)));
        assert_eq!(plus.get_value(&walker).unwrap(), i64::MIN);
        let minus = PatternExpression::Minus(UnaryExpression::new(cval(i64::MIN)));
        assert_eq!(minus.get_value(&walker).unwrap(), i64::MIN);
        // shift count 64 resolves x86-masked to 0
        let lsh = PatternExpression::LeftShift(BinaryExpression::new(cval(3), cval(64)));
        assert_eq!(lsh.get_value(&walker).unwrap(), 3);
    }

    #[test]
    fn list_values_minmax_subvalue_walk() {
        // (tokA + tokB*2): leaves in left-to-right order
        let tok_a = PatternExpression::Value(PatternValue::TokenField(TokenField::new(
            1, false, false, 0, 3,
        )));
        let tok_b = PatternExpression::Value(PatternValue::TokenField(TokenField::new(
            1, false, false, 4, 7,
        )));
        let expr = PatternExpression::Plus(BinaryExpression::new(
            tok_a,
            PatternExpression::Mult(BinaryExpression::new(tok_b, cval(2))),
        ));
        let mut values = Vec::new();
        expr.list_values(&mut values);
        assert_eq!(values.len(), 3); // tokA, tokB, const 2
        let mut minlist = Vec::new();
        let mut maxlist = Vec::new();
        expr.get_min_max(&mut minlist, &mut maxlist).unwrap();
        assert_eq!(minlist, vec![0, 0, 2]);
        assert_eq!(maxlist, vec![15, 15, 2]);
        // substitute tokA=3, tokB=5, const leaf reads its own slot (2)
        assert_eq!(expr.get_sub_value_root(&[3, 5, 2]).unwrap(), 13);
    }

    #[test]
    fn get_min_max_errors_on_operand_value() {
        let expr = PatternExpression::Plus(BinaryExpression::new(
            PatternExpression::Value(PatternValue::OperandValue(OperandValue::new(0, 1, 0))),
            cval(1),
        ));
        let mut minlist = Vec::new();
        let mut maxlist = Vec::new();
        assert!(expr.get_min_max(&mut minlist, &mut maxlist).is_err());
    }

    // -- decode / encode round trips -------------------------------------------------

    fn roundtrip(expr_bytes: &[u8]) -> (PatternExpression, Vec<u8>) {
        let manager = test_manager();
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(expr_bytes).unwrap();
        let expr = PatternExpression::decode_expression(&mut dec, &FixedResolver(16)).unwrap();
        let mut reenc = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut reenc);
            expr.encode(&mut enc);
        }
        (expr, reenc)
    }

    #[test]
    fn decode_roundtrip_handencoded_plus_tree() {
        // hand-encode plus_exp{ tokenfield, intb } with PackedEncode
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_PLUS_EXP);
            enc.open_element(&sla::ELEM_TOKENFIELD);
            enc.write_bool(&sla::ATTRIB_BIGENDIAN, false);
            enc.write_bool(&sla::ATTRIB_SIGNBIT, false);
            enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
            enc.write_signed_integer(&sla::ATTRIB_ENDBIT, 7);
            enc.write_signed_integer(&sla::ATTRIB_STARTBYTE, 0);
            enc.write_signed_integer(&sla::ATTRIB_ENDBYTE, 0);
            enc.write_signed_integer(&sla::ATTRIB_SHIFT, 0);
            enc.close_element(&sla::ELEM_TOKENFIELD);
            enc.open_element(&sla::ELEM_INTB);
            enc.write_signed_integer(&sla::ATTRIB_VAL, 0x10);
            enc.close_element(&sla::ELEM_INTB);
            enc.close_element(&sla::ELEM_PLUS_EXP);
        }
        let (expr, reenc) = roundtrip(&buf);
        // decode -> re-encode is byte identical
        assert_eq!(reenc, buf);
        // and evaluates: instruction byte 0x2a + 0x10
        let walker = TestWalker::from_bytes(&[0x2a, 0, 0, 0]);
        assert_eq!(expr.get_value(&walker).unwrap(), 0x3a);
    }

    #[test]
    fn decode_roundtrip_nested_operators() {
        // not_exp{ div_exp{ intb, intb } }: unary over nested binary
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_NOT_EXP);
            enc.open_element(&sla::ELEM_DIV_EXP);
            enc.open_element(&sla::ELEM_INTB);
            enc.write_signed_integer(&sla::ATTRIB_VAL, 100);
            enc.close_element(&sla::ELEM_INTB);
            enc.open_element(&sla::ELEM_INTB);
            enc.write_signed_integer(&sla::ATTRIB_VAL, 7);
            enc.close_element(&sla::ELEM_INTB);
            enc.close_element(&sla::ELEM_DIV_EXP);
            enc.close_element(&sla::ELEM_NOT_EXP);
        }
        let (expr, reenc) = roundtrip(&buf);
        assert_eq!(reenc, buf);
        let walker = TestWalker::from_bytes(&[]);
        assert_eq!(expr.get_value(&walker).unwrap(), !(100i64 / 7));
    }

    #[test]
    fn decode_roundtrip_contextfield_and_start() {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_SUB_EXP);
            enc.open_element(&sla::ELEM_CONTEXTFIELD);
            enc.write_bool(&sla::ATTRIB_SIGNBIT, true);
            enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 4);
            enc.write_signed_integer(&sla::ATTRIB_ENDBIT, 7);
            enc.write_signed_integer(&sla::ATTRIB_STARTBYTE, 0);
            enc.write_signed_integer(&sla::ATTRIB_ENDBYTE, 0);
            enc.write_signed_integer(&sla::ATTRIB_SHIFT, 0);
            enc.close_element(&sla::ELEM_CONTEXTFIELD);
            enc.open_element(&sla::ELEM_START_EXP);
            enc.close_element(&sla::ELEM_START_EXP);
            enc.close_element(&sla::ELEM_SUB_EXP);
        }
        let (expr, reenc) = roundtrip(&buf);
        assert_eq!(reenc, buf);
        let manager = test_manager();
        let mut walker = TestWalker::from_context(&[0xab00_0000]);
        walker.addr = Some(Address::new(ram(&manager), 8));
        // ctx nibble [4,7] of 0xab signed = -5; start value = 8/2 = 4
        assert_eq!(expr.get_value(&walker).unwrap(), -9);
    }

    #[test]
    fn decode_roundtrip_end_exp() {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_END_EXP);
            enc.close_element(&sla::ELEM_END_EXP);
        }
        let (expr, reenc) = roundtrip(&buf);
        assert_eq!(reenc, buf);
        assert!(matches!(
            expr,
            PatternExpression::Value(PatternValue::EndInstructionValue(_))
        ));
    }

    #[test]
    fn decode_operand_exp_validates_constructor_id() {
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_OPERAND_EXP);
            enc.write_signed_integer(&sla::ATTRIB_INDEX, 1);
            enc.write_unsigned_integer(&sla::ATTRIB_TABLE, 5);
            enc.write_unsigned_integer(&sla::ATTRIB_CT, 9);
            enc.close_element(&sla::ELEM_OPERAND_EXP);
        }
        // resolver says the table has 10 constructors: ctid 9 is valid
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let expr = PatternExpression::decode_expression(&mut dec, &FixedResolver(10)).unwrap();
        match &expr {
            PatternExpression::Value(PatternValue::OperandValue(ov)) => {
                assert_eq!(ov.index(), 1);
                assert_eq!(ov.table_id(), 5);
                assert_eq!(ov.ct_id(), 9);
            }
            other => panic!("expected OperandValue, got {other:?}"),
        }
        // re-encode is byte identical
        let mut reenc = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut reenc);
            expr.encode(&mut enc);
        }
        assert_eq!(reenc, buf);
        // resolver says only 9 constructors: ctid 9 -> "Invalid constructor id"
        let mut dec2 = PackedDecode::new(&manager);
        dec2.ingest_stream(&buf).unwrap();
        let err = PatternExpression::decode_expression(&mut dec2, &FixedResolver(9))
            .expect_err("decode must fail");
        assert!(format!("{err}").contains("Invalid constructor id"));
    }

    #[test]
    fn decode_rejects_unknown_and_next2_elements() {
        let manager = test_manager();
        // next2_exp is deliberately NOT in the C++ factory
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_NEXT2_EXP);
            enc.close_element(&sla::ELEM_NEXT2_EXP);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let err = PatternExpression::decode_expression(&mut dec, &FixedResolver(1))
            .expect_err("decode must fail");
        assert!(format!("{err}").contains("Invalid pattern expression element"));
        // but the direct decode method works (used by Next2Symbol)
        let mut dec2 = PackedDecode::new(&manager);
        dec2.ingest_stream(&buf).unwrap();
        Next2InstructionValue::decode(&mut dec2).unwrap();
    }
}
