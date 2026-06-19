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

use crate::slghpattern::{
    sla, ContextPattern, DisjointPattern, InstructionPattern, Pattern, PatternBlock,
};

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
    /// (kuna build side) The owning `Token`'s byte size, retained so the
    /// SLEIGH-compiler `genPattern`/`genMinPattern` can build a
    /// `TokenPattern`.  C++ carries `Token *tok` here; the decode path nulls
    /// it (and never calls genPattern) so the decode factory leaves this -1.
    tok_size: i32,
    /// (kuna build side) The owning `Token`'s index (its identity in the
    /// pattern token list, replacing the C++ `Token *` pointer-identity).
    /// -1 after decode (no token).
    tok_index: i32,
}

impl TokenField {
    /// C++ `TokenField(Token *tk,bool s,int4 bstart,int4 bend)`.  `Token` is
    /// SLEIGH-compiler state; the three properties the C++ constructor reads
    /// from it (`tok->getSize()`, `tok->isBigEndian()`, and — for the build
    /// side — `tok->getIndex()` for token identity) are passed directly.
    /// `tok_index` defaults to -1 for callers that do not build patterns
    /// (use [`TokenField::new_for_build`] to carry the token identity).
    pub fn new(tok_size: i32, tok_bigendian: bool, s: bool, bstart: i32, bend: i32) -> TokenField {
        TokenField::new_for_build(tok_size, tok_bigendian, -1, s, bstart, bend)
    }

    /// C++ `TokenField(Token *tk,bool s,int4 bstart,int4 bend)` retaining the
    /// token index (its identity) for the SLEIGH-compiler build side.
    pub fn new_for_build(
        tok_size: i32,
        tok_bigendian: bool,
        tok_index: i32,
        s: bool,
        bstart: i32,
        bend: i32,
    ) -> TokenField {
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
            tok_size,
            tok_index,
        }
    }

    /// C++ `TokenField::genPattern(intb val)`: the basic instruction pattern
    /// `TokenPattern(tok, val, bitstart, bitend)`.
    pub fn gen_pattern(&self, val: i64) -> TokenPattern {
        TokenPattern::new_instruction_field(
            BuildToken {
                size: self.tok_size,
                bigendian: self.bigendian,
                index: self.tok_index,
            },
            val,
            self.bitstart,
            self.bitend,
        )
    }

    /// C++ `TokenField::genMinPattern`: `TokenPattern(tok)` (a TRUE pattern
    /// associated with the field's token).
    pub fn gen_min_pattern(&self) -> TokenPattern {
        TokenPattern::new_token(BuildToken {
            size: self.tok_size,
            bigendian: self.bigendian,
            index: self.tok_index,
        })
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
            // C++ decode nulls `tok` and never calls genPattern; -1 sentinel.
            tok_size: -1,
            tok_index: -1,
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

    /// C++ `ContextField::genPattern(intb val)`:
    /// `TokenPattern(val, startbit, endbit)` (a context pattern).
    pub fn gen_pattern(&self, val: i64) -> TokenPattern {
        TokenPattern::new_context_field(val, self.startbit, self.endbit)
    }

    /// C++ `ContextField::genMinPattern`: `TokenPattern()` (TRUE, no token).
    pub fn gen_min_pattern(&self) -> TokenPattern {
        TokenPattern::new_true()
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

    /// C++ virtual `PatternValue::genPattern(intb val)`: the pattern forcing
    /// this value to `val`.  `OperandValue::genPattern` throws (operands
    /// cannot appear in a pattern expression).
    pub fn gen_pattern(&self, val: i64) -> KunaResult<TokenPattern> {
        match self {
            PatternValue::TokenField(v) => Ok(v.gen_pattern(val)),
            PatternValue::ContextField(v) => Ok(v.gen_pattern(val)),
            // C++ `ConstantValue::genPattern`: `TokenPattern(val==v)`
            PatternValue::ConstantValue(v) => Ok(TokenPattern::new_bool(v.get_value() == val)),
            PatternValue::OperandValue(_) => {
                Err(KunaError::sleigh("Operand used in pattern expression"))
            }
            // Start/End/Next2 `genPattern` all return `TokenPattern()` (TRUE)
            PatternValue::StartInstructionValue(_)
            | PatternValue::EndInstructionValue(_)
            | PatternValue::Next2InstructionValue(_) => Ok(TokenPattern::new_true()),
        }
    }

    /// C++ virtual `PatternValue::genMinPattern(const vector<TokenPattern>&)`.
    /// `TokenField` returns its token; `OperandValue` returns `ops[index]`;
    /// the rest return TRUE.
    pub fn gen_min_pattern(&self, ops: &[TokenPattern]) -> TokenPattern {
        match self {
            PatternValue::TokenField(v) => v.gen_min_pattern(),
            PatternValue::ContextField(v) => v.gen_min_pattern(),
            // C++ `OperandValue::genMinPattern`: `return ops[index]`
            PatternValue::OperandValue(v) => ops[v.index() as usize].clone(),
            PatternValue::ConstantValue(_)
            | PatternValue::StartInstructionValue(_)
            | PatternValue::EndInstructionValue(_)
            | PatternValue::Next2InstructionValue(_) => TokenPattern::new_true(),
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

    /// C++ virtual `genMinPattern(const vector<TokenPattern> &ops)`.  Only
    /// `PatternValue` leaves do anything; every `BinaryExpression`/
    /// `UnaryExpression` returns `TokenPattern()` (TRUE).
    pub fn gen_min_pattern(&self, ops: &[TokenPattern]) -> TokenPattern {
        match self {
            PatternExpression::Value(v) => v.gen_min_pattern(ops),
            // BinaryExpression::genMinPattern / UnaryExpression::genMinPattern
            _ => TokenPattern::new_true(),
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

// ===========================================================================
// SLEIGH-compiler build side (ws4a): TokenPattern + PatternEquation arena
//
// Port of the compile-only half of slghpatexpress.{hh,cc} — the machinery the
// SLEIGH compiler drives to turn the parsed grammar into `Pattern`s.  This is
// ADDITIVE to the decode side above; the decoder never enters this code.
//
// Ownership / arena convention (consumed by WS4b's driver):
//   * `TokenPattern` is a value type (Clone) wrapping a `Pattern` plus its
//     token alignment list; the C++ `Pattern *pattern` + `simplifyClone`
//     ownership protocol becomes plain Rust ownership (`simplifyClone` IS the
//     Clone the assignment operator performed).
//   * `PatternEquation` is an `enum` stored in an `EquationArena` and
//     referenced by a `u32` arena id (`EqId`) — exactly the `u32` the WS2
//     parser actions thread.  The C++ refcounted `PatternEquation *` tree
//     (layClaim/release) becomes an arena of nodes whose children are `EqId`s.
//     The driver owns the arena; the parser returns the ids the arena indexes.
//   * Equation `genPattern` is non-const in C++ (it caches `resultpattern`);
//     here `gen_pattern` is a pure function returning the `TokenPattern` (no
//     mutable cache needed — the callers in slghsymbol read it back through
//     the return value).
// ===========================================================================

/// The three `Token` properties the build-side `TokenField`/`TokenPattern`
/// machinery reads (C++ `Token *`): byte size, endianness, and a unique index
/// standing in for pointer identity in the token-alignment list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildToken {
    /// C++ `tok->getSize()`.
    pub size: i32,
    /// C++ `tok->isBigEndian()`.
    pub bigendian: bool,
    /// C++ pointer identity, as the token's `getIndex()`.
    pub index: i32,
}

/// C++ `TokenPattern`: the token-aligned pattern builder.  Wraps a
/// [`Pattern`] together with the list of tokens it spans (for alignment) and
/// the two ellipsis flags.
#[derive(Debug, Clone)]
pub struct TokenPattern {
    pattern: Pattern,
    toklist: Vec<BuildToken>,
    leftellipsis: bool,
    rightellipsis: bool,
}

impl TokenPattern {
    /// C++ `TokenPattern(void)`: TRUE pattern unassociated with a token.
    pub fn new_true() -> TokenPattern {
        TokenPattern {
            pattern: Pattern::Disjoint(DisjointPattern::Instruction(
                InstructionPattern::new_always(true),
            )),
            toklist: Vec::new(),
            leftellipsis: false,
            rightellipsis: false,
        }
    }

    /// C++ `TokenPattern(bool tf)`: TRUE or FALSE pattern, no token.
    pub fn new_bool(tf: bool) -> TokenPattern {
        TokenPattern {
            pattern: Pattern::Disjoint(DisjointPattern::Instruction(
                InstructionPattern::new_always(tf),
            )),
            toklist: Vec::new(),
            leftellipsis: false,
            rightellipsis: false,
        }
    }

    /// C++ `TokenPattern(Token *tok)`: TRUE pattern associated with `tok`.
    pub fn new_token(tok: BuildToken) -> TokenPattern {
        TokenPattern {
            pattern: Pattern::Disjoint(DisjointPattern::Instruction(
                InstructionPattern::new_always(true),
            )),
            toklist: vec![tok],
            leftellipsis: false,
            rightellipsis: false,
        }
    }

    /// C++ `TokenPattern(Token *tok,intb value,int4 bitstart,int4 bitend)`:
    /// a basic instruction pattern.
    pub fn new_instruction_field(
        tok: BuildToken,
        value: i64,
        bitstart: i32,
        bitend: i32,
    ) -> TokenPattern {
        let block = if tok.bigendian {
            build_big_block(tok.size, bitstart, bitend, value)
        } else {
            build_little_block(tok.size, bitstart, bitend, value)
        };
        TokenPattern {
            pattern: Pattern::Disjoint(DisjointPattern::Instruction(InstructionPattern::new(block))),
            toklist: vec![tok],
            leftellipsis: false,
            rightellipsis: false,
        }
    }

    /// C++ `TokenPattern(intb value,int4 startbit,int4 endbit)`: a basic
    /// context pattern.
    pub fn new_context_field(value: i64, startbit: i32, endbit: i32) -> TokenPattern {
        let size = (endbit / 8) + 1;
        let block = build_big_block(size, size * 8 - 1 - endbit, size * 8 - 1 - startbit, value);
        TokenPattern {
            pattern: Pattern::Disjoint(DisjointPattern::Context(ContextPattern::new(block))),
            toklist: Vec::new(),
            leftellipsis: false,
            rightellipsis: false,
        }
    }

    /// The wrapped [`Pattern`] (C++ `getPattern`).
    pub fn get_pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Consume into the wrapped [`Pattern`].
    pub fn into_pattern(self) -> Pattern {
        self.pattern
    }

    /// C++ `TokenPattern::setLeftEllipsis`.
    pub fn set_left_ellipsis(&mut self, val: bool) {
        self.leftellipsis = val;
    }

    /// C++ `TokenPattern::setRightEllipsis`.
    pub fn set_right_ellipsis(&mut self, val: bool) {
        self.rightellipsis = val;
    }

    /// C++ `TokenPattern::getLeftEllipsis`.
    pub fn get_left_ellipsis(&self) -> bool {
        self.leftellipsis
    }

    /// C++ `TokenPattern::getRightEllipsis`.
    pub fn get_right_ellipsis(&self) -> bool {
        self.rightellipsis
    }

    /// C++ `TokenPattern::alwaysTrue`.
    pub fn always_true(&self) -> bool {
        self.pattern.always_true()
    }

    /// C++ `TokenPattern::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        self.pattern.always_false()
    }

    /// C++ `TokenPattern::alwaysInstructionTrue`.
    pub fn always_instruction_true(&self) -> bool {
        self.pattern.always_instruction_true()
    }

    /// C++ `TokenPattern::getMinimumLength`: sum of the spanned token sizes.
    pub fn get_minimum_length(&self) -> i32 {
        let mut length = 0;
        for tok in &self.toklist {
            length += tok.size;
        }
        length
    }

    /// C++ `TokenPattern::resolveTokens`: decide how `tok1`/`tok2` align,
    /// returning the shift `tok2` needs and storing the resulting token list
    /// and ellipses into `self`.
    fn resolve_tokens(
        &mut self,
        tok1: &TokenPattern,
        tok2: &TokenPattern,
    ) -> KunaResult<i32> {
        let mut reversedirection = false;
        self.leftellipsis = false;
        self.rightellipsis = false;
        let mut ressa: i32 = 0;
        let l1 = tok1.toklist.len();
        let l2 = tok2.toklist.len();
        let minsize = l1.min(l2);
        if minsize == 0 {
            // Check if pattern doesn't care about tokens
            if l1 == 0 && !tok1.leftellipsis && !tok1.rightellipsis {
                self.toklist = tok2.toklist.clone();
                self.leftellipsis = tok2.leftellipsis;
                self.rightellipsis = tok2.rightellipsis;
                return Ok(0);
            } else if l2 == 0 && !tok2.leftellipsis && !tok2.rightellipsis {
                self.toklist = tok1.toklist.clone();
                self.leftellipsis = tok1.leftellipsis;
                self.rightellipsis = tok1.rightellipsis;
                return Ok(0);
            }
            // If one of the ellipses is true then the pattern still cares
            // about tokens even though none are specified
        }

        if tok1.leftellipsis {
            reversedirection = true;
            if tok2.rightellipsis {
                return Err(KunaError::sleigh("Right/left ellipsis"));
            } else if tok2.leftellipsis {
                self.leftellipsis = true;
            } else if l1 != minsize {
                return Err(KunaError::sleigh(format!(
                    "Mismatched pattern sizes -- {} != {}",
                    l1, minsize
                )));
            } else if l1 == l2 {
                return Err(KunaError::sleigh("Pattern size cannot vary (missing '...'?)"));
            }
        } else if tok1.rightellipsis {
            if tok2.leftellipsis {
                return Err(KunaError::sleigh("Left/right ellipsis"));
            } else if tok2.rightellipsis {
                self.rightellipsis = true;
            } else if l1 != minsize {
                return Err(KunaError::sleigh(format!(
                    "Mismatched pattern sizes -- {} != {}",
                    l1, minsize
                )));
            } else if l1 == l2 {
                return Err(KunaError::sleigh("Pattern size cannot vary (missing '...'?)"));
            }
        } else if tok2.leftellipsis {
            reversedirection = true;
            if l2 != minsize {
                return Err(KunaError::sleigh(format!(
                    "Mismatched pattern sizes -- {} != {}",
                    l2, minsize
                )));
            } else if l1 == l2 {
                return Err(KunaError::sleigh("Pattern size cannot vary (missing '...'?)"));
            }
        } else if tok2.rightellipsis {
            if l2 != minsize {
                return Err(KunaError::sleigh(format!(
                    "Mismatched pattern sizes -- {} != {}",
                    l2, minsize
                )));
            } else if l1 == l2 {
                return Err(KunaError::sleigh("Pattern size cannot vary (missing '...'?)"));
            }
        } else if l2 != l1 {
            return Err(KunaError::sleigh(format!(
                "Mismatched pattern sizes -- {} != {}",
                l2, l1
            )));
        }

        if reversedirection {
            for i in 0..minsize {
                if tok1.toklist[l1 - 1 - i] != tok2.toklist[l2 - 1 - i] {
                    return Err(KunaError::sleigh(format!(
                        "Mismatched tokens when combining patterns -- {} != {}",
                        tok1.toklist[l1 - 1 - i].index,
                        tok2.toklist[l2 - 1 - i].index
                    )));
                }
            }
            if l1 <= l2 {
                for i in minsize..l2 {
                    ressa += tok2.toklist[l2 - 1 - i].size;
                }
            } else {
                for i in minsize..l1 {
                    ressa += tok1.toklist[l1 - 1 - i].size;
                }
            }
            if l1 < l2 {
                ressa = -ressa;
            }
        } else {
            for i in 0..minsize {
                if tok1.toklist[i] != tok2.toklist[i] {
                    return Err(KunaError::sleigh(format!(
                        "Mismatched tokens when combining patterns -- {} != {}",
                        tok1.toklist[i].index, tok2.toklist[i].index
                    )));
                }
            }
        }
        // Save the results into -self-
        if l1 <= l2 {
            self.toklist = tok2.toklist.clone();
        } else {
            self.toklist = tok1.toklist.clone();
        }
        Ok(ressa)
    }

    /// C++ `TokenPattern::doAnd`.
    pub fn do_and(&self, tokpat: &TokenPattern) -> KunaResult<TokenPattern> {
        let mut res = TokenPattern::new_true();
        res.toklist.clear();
        let sa = res.resolve_tokens(self, tokpat)?;
        // C++ returns `res` by value; the caller's TokenPattern copy/assign
        // runs `pattern->simplifyClone()`.  Apply it here so the stored
        // pattern is the simplified one (e.g. a trivial AND collapses).
        res.pattern = self.pattern.do_and(&tokpat.pattern, sa).simplify_clone();
        Ok(res)
    }

    /// C++ `TokenPattern::doOr`.
    pub fn do_or(&self, tokpat: &TokenPattern) -> KunaResult<TokenPattern> {
        let mut res = TokenPattern::new_true();
        res.toklist.clear();
        let sa = res.resolve_tokens(self, tokpat)?;
        // do_or takes &mut on both operands (the upstream const-cast quirk);
        // operate on clones since C++ `doOr` may mutate either receiver.
        let mut left = self.pattern.clone();
        let mut right = tokpat.pattern.clone();
        res.pattern = left.do_or(&mut right, sa).simplify_clone();
        Ok(res)
    }

    /// C++ `TokenPattern::doCat`: concatenation of `self` and `tokpat`.
    pub fn do_cat(&self, tokpat: &TokenPattern) -> KunaResult<TokenPattern> {
        let mut res = TokenPattern::new_true();
        res.toklist.clear();
        res.leftellipsis = self.leftellipsis;
        res.rightellipsis = self.rightellipsis;
        res.toklist = self.toklist.clone();
        let sa: i32;
        if self.rightellipsis || tokpat.leftellipsis {
            // Check for interior ellipsis
            if self.rightellipsis && !tokpat.always_instruction_true() {
                return Err(KunaError::sleigh("Interior ellipsis in pattern"));
            }
            if tokpat.leftellipsis {
                if !self.always_instruction_true() {
                    return Err(KunaError::sleigh("Interior ellipsis in pattern"));
                }
                res.leftellipsis = true;
            }
            sa = -1;
        } else {
            let mut acc = 0;
            for tok in &self.toklist {
                acc += tok.size;
            }
            sa = acc;
            for tok in &tokpat.toklist {
                res.toklist.push(*tok);
            }
            res.rightellipsis = tokpat.rightellipsis;
        }
        if res.rightellipsis && res.leftellipsis {
            return Err(KunaError::sleigh("Double ellipsis in pattern"));
        }
        if sa < 0 {
            res.pattern = self.pattern.do_and(&tokpat.pattern, 0).simplify_clone();
        } else {
            res.pattern = self.pattern.do_and(&tokpat.pattern, sa).simplify_clone();
        }
        Ok(res)
    }

    /// C++ `TokenPattern::commonSubPattern`.
    pub fn common_sub_pattern(&self, tokpat: &TokenPattern) -> KunaResult<TokenPattern> {
        let mut patres = TokenPattern::new_true();
        patres.toklist.clear();
        let mut reversedirection = false;

        if self.leftellipsis || tokpat.leftellipsis {
            if self.rightellipsis || tokpat.rightellipsis {
                return Err(KunaError::sleigh("Right/left ellipsis in commonSubPattern"));
            }
            reversedirection = true;
        }

        // Find common subset of tokens and ellipses
        patres.leftellipsis = self.leftellipsis || tokpat.leftellipsis;
        patres.rightellipsis = self.rightellipsis || tokpat.rightellipsis;
        let mut minnum = self.toklist.len();
        let mut maxnum = tokpat.toklist.len();
        if maxnum < minnum {
            std::mem::swap(&mut minnum, &mut maxnum);
        }
        let mut i = 0usize;
        if reversedirection {
            while i < minnum {
                let tok = self.toklist[self.toklist.len() - 1 - i];
                if tok == tokpat.toklist[tokpat.toklist.len() - 1 - i] {
                    patres.toklist.insert(0, tok);
                } else {
                    break;
                }
                i += 1;
            }
            if i < maxnum {
                patres.leftellipsis = true;
            }
        } else {
            while i < minnum {
                let tok = self.toklist[i];
                if tok == tokpat.toklist[i] {
                    patres.toklist.push(tok);
                } else {
                    break;
                }
                i += 1;
            }
            if i < maxnum {
                patres.rightellipsis = true;
            }
        }
        patres.pattern = self
            .pattern
            .common_sub_pattern(&tokpat.pattern, 0)
            .simplify_clone();
        Ok(patres)
    }
}

/// C++ static `TokenPattern::buildSingle(int4 startbit,int4 endbit,uintm
/// byteval)`: a mask/value pattern within a single word.  bit 0 is the MOST
/// significant bit of the word.
fn build_single(mut startbit: i32, mut endbit: i32, mut byteval: u32) -> PatternBlock {
    let mut offset = 0;
    let size = endbit - startbit + 1;
    while startbit >= 8 {
        offset += 1;
        startbit -= 8;
        endbit -= 8;
    }
    // mask = (~0) << (32 - size); shift count in [0,32) for valid fields
    let mut mask: u32 = (!0u32).wshl((32 - size) as u32);
    byteval = byteval.wshl((32 - size) as u32) & mask;
    mask = mask.wshr(startbit as u32);
    byteval = byteval.wshr(startbit as u32);
    PatternBlock::new(offset, mask, byteval)
}

/// C++ static `TokenPattern::buildBigBlock(int4 size,int4 bitstart,int4
/// bitend,intb value)`: pattern block for a bigendian contiguous bit range.
fn build_big_block(size: i32, bitstart: i32, bitend: i32, mut value: i64) -> PatternBlock {
    let startbit = 8 * size - 1 - bitend;
    let mut endbit = 8 * size - 1 - bitstart;

    let mut block: Option<PatternBlock> = None;
    while endbit >= startbit {
        let mut tmpstart = endbit - (endbit & 7);
        if tmpstart < startbit {
            tmpstart = startbit;
        }
        let tmpblock = build_single(tmpstart, endbit, value as u32);
        block = Some(match block {
            None => tmpblock,
            Some(b) => b.intersect(&tmpblock),
        });
        // value >>= (endbit-tmpstart+1): intb arithmetic shift, count in [1,8]
        value = value.wshr((endbit - tmpstart + 1) as u32);
        endbit = tmpstart - 1;
    }
    // C++ may return a null block when endbit < startbit on entry; that only
    // happens for an empty range, which the build call sites never produce.
    block.expect("build_big_block: empty bit range (C++ returns a null PatternBlock)")
}

/// C++ static `TokenPattern::buildLittleBlock(int4 size,int4 bitstart,int4
/// bitend,intb value)`: pattern block for a littleendian contiguous bit
/// range.
fn build_little_block(size: i32, mut bitstart: i32, mut bitend: i32, mut value: i64) -> PatternBlock {
    let _ = size; // C++ takes size but never reads it in this branch
    let mut startbit = (bitstart / 8) * 8;
    let endbit = (bitend / 8) * 8;
    bitend %= 8;
    bitstart %= 8;

    let block: PatternBlock;
    if startbit == endbit {
        startbit += 7 - bitend;
        let e = endbit + 7 - bitstart;
        block = build_single(startbit, e, value as u32);
    } else {
        let mut blk = build_single(startbit, startbit + (7 - bitstart), value as u32);
        value = value.wshr((8 - bitstart) as u32); // Cut off bits we just encoded
        startbit += 8;
        while startbit != endbit {
            let tmpblock = build_single(startbit, startbit + 7, value as u32);
            blk = blk.intersect(&tmpblock);
            value = value.wshr(8);
            startbit += 8;
        }
        let tmpblock = build_single(endbit + (7 - bitend), endbit + 7, value as u32);
        blk = blk.intersect(&tmpblock);
        block = blk;
    }
    block
}

/// C++ static `advance_combo`: increment the multi-dimensional counter `val`
/// (each digit ranges `[min, max]` inclusive); returns false when it wraps.
fn advance_combo(val: &mut [i64], min: &[i64], max: &[i64]) -> bool {
    let mut i = 0;
    while i < val.len() {
        val[i] += 1;
        if val[i] <= max[i] {
            // maximum is inclusive
            return true;
        }
        val[i] = min[i];
        i += 1;
    }
    false
}

/// C++ static `buildPattern(PatternValue *lhs,intb lhsval,vector<const
/// PatternValue*> &semval,vector<intb> &val)`: AND the lhs's pattern with the
/// forced pattern of every rhs leaf.
fn build_equation_pattern(
    lhs: &PatternValue,
    lhsval: i64,
    semval: &[&PatternValue],
    val: &[i64],
) -> KunaResult<TokenPattern> {
    let mut respattern = lhs.gen_pattern(lhsval)?;
    for (i, sv) in semval.iter().enumerate() {
        respattern = respattern.do_and(&sv.gen_pattern(val[i])?)?;
    }
    Ok(respattern)
}

// ---------------------------------------------------------------------------
// OperandResolve (slghpatexpress.hh:339)
// ---------------------------------------------------------------------------

/// C++ `OperandResolve`: the traversal state for `resolveOperandLeft`.  The
/// C++ struct holds a `vector<OperandSymbol*> &operands`; here the operand
/// updates flow through the [`OperandResolveSink`] seam so the equation code
/// stays independent of the symbol table.
pub struct OperandResolve {
    /// Current base operand (as we traverse left to right).
    pub base: i32,
    /// Bytes traversed from the LEFT edge of the current base.
    pub offset: i32,
    /// (resulting) rightmost operand in the pattern.
    pub cur_rightmost: i32,
    /// (resulting) bytes traversed from the LEFT edge of the rightmost.
    pub size: i32,
}

impl Default for OperandResolve {
    fn default() -> OperandResolve {
        OperandResolve {
            base: -1,
            offset: 0,
            cur_rightmost: -1,
            size: 0,
        }
    }
}

impl OperandResolve {
    /// C++ `OperandResolve(vector<OperandSymbol*> &ops)`.
    pub fn new() -> OperandResolve {
        OperandResolve::default()
    }
}

/// Seam for the operand mutations `OperandEquation::resolveOperandLeft`
/// performs on a `OperandSymbol` (C++ reaches through `state.operands[index]`
/// directly).  Implemented by the slghsymbol build side.
pub trait OperandResolveSink {
    /// C++ `OperandSymbol::isOffsetIrrelevant()` for operand `index`.
    fn is_offset_irrelevant(&self, index: i32) -> bool;
    /// C++ `sym->offsetbase = base; sym->reloffset = offset;` for `index`.
    fn set_offset(&mut self, index: i32, offsetbase: i32, reloffset: u32);
}

// ---------------------------------------------------------------------------
// PatternEquation arena (slghpatexpress.hh:351-487, slghpatexpress.cc)
// ---------------------------------------------------------------------------

/// Arena id of a [`PatternEquation`] node (the `u32` the WS2 parser threads).
pub type EqId = u32;

/// C++ `PatternEquation` hierarchy as an arena enum.  Children are stored as
/// [`EqId`]s into the owning [`EquationArena`] (replacing the C++ refcounted
/// `PatternEquation *` pointers); leaf payloads (the value/expression of a
/// comparison, the operand index, the unconstrained expression) are owned
/// inline.
#[derive(Debug, Clone)]
pub enum PatternEquation {
    /// C++ `OperandEquation(int4 index)`.
    Operand { index: i32 },
    /// C++ `UnconstrainedEquation(PatternExpression*)`.
    Unconstrained { patex: PatternExpression },
    /// C++ `EqualEquation(PatternValue*, PatternExpression*)`.
    Equal { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `NotEqualEquation`.
    NotEqual { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `LessEquation`.
    Less { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `LessEqualEquation`.
    LessEqual { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `GreaterEquation`.
    Greater { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `GreaterEqualEquation`.
    GreaterEqual { lhs: PatternValue, rhs: PatternExpression },
    /// C++ `EquationAnd(left,right)`.
    And { left: EqId, right: EqId },
    /// C++ `EquationOr(left,right)`.
    Or { left: EqId, right: EqId },
    /// C++ `EquationCat(left,right)`.
    Cat { left: EqId, right: EqId },
    /// C++ `EquationLeftEllipsis(eq)`.
    LeftEllipsis { eq: EqId },
    /// C++ `EquationRightEllipsis(eq)`.
    RightEllipsis { eq: EqId },
}

/// The driver-owned arena of [`PatternEquation`] nodes (the storage the WS2
/// `u32` equation ids index).
#[derive(Debug, Clone, Default)]
pub struct EquationArena {
    nodes: Vec<PatternEquation>,
}

impl EquationArena {
    /// A fresh empty arena.
    pub fn new() -> EquationArena {
        EquationArena { nodes: Vec::new() }
    }

    /// Store a node and return its id.
    pub fn alloc(&mut self, eq: PatternEquation) -> EqId {
        let id = self.nodes.len() as u32;
        self.nodes.push(eq);
        id
    }

    /// Borrow a node by id (panics on a bad id — an internal invariant
    /// violation, ADR 0004).
    pub fn get(&self, id: EqId) -> &PatternEquation {
        &self.nodes[id as usize]
    }

    /// Number of nodes (test surface).
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the arena is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// C++ `PatternEquation::genPattern(const vector<TokenPattern> &ops)`:
    /// build the [`TokenPattern`] for equation `id` (returned, not cached).
    pub fn gen_pattern(&self, id: EqId, ops: &[TokenPattern]) -> KunaResult<TokenPattern> {
        match self.get(id) {
            // OperandEquation::genPattern: resultpattern = ops[index]
            PatternEquation::Operand { index } => Ok(ops[*index as usize].clone()),
            // UnconstrainedEquation::genPattern: patex->genMinPattern(ops)
            PatternEquation::Unconstrained { patex } => Ok(patex.gen_min_pattern(ops)),
            PatternEquation::Equal { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::Equal)
            }
            PatternEquation::NotEqual { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::NotEqual)
            }
            PatternEquation::Less { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::Less)
            }
            PatternEquation::LessEqual { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::LessEqual)
            }
            PatternEquation::Greater { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::Greater)
            }
            PatternEquation::GreaterEqual { lhs, rhs } => {
                gen_comparison(lhs, rhs, ops, Comparison::GreaterEqual)
            }
            // EquationAnd::genPattern
            PatternEquation::And { left, right } => {
                let l = self.gen_pattern(*left, ops)?;
                let r = self.gen_pattern(*right, ops)?;
                l.do_and(&r)
            }
            // EquationOr::genPattern
            PatternEquation::Or { left, right } => {
                let l = self.gen_pattern(*left, ops)?;
                let r = self.gen_pattern(*right, ops)?;
                l.do_or(&r)
            }
            // EquationCat::genPattern
            PatternEquation::Cat { left, right } => {
                let l = self.gen_pattern(*left, ops)?;
                let r = self.gen_pattern(*right, ops)?;
                l.do_cat(&r)
            }
            // EquationLeftEllipsis::genPattern
            PatternEquation::LeftEllipsis { eq } => {
                let mut p = self.gen_pattern(*eq, ops)?;
                p.set_left_ellipsis(true);
                Ok(p)
            }
            // EquationRightEllipsis::genPattern
            PatternEquation::RightEllipsis { eq } => {
                let mut p = self.gen_pattern(*eq, ops)?;
                p.set_right_ellipsis(true);
                Ok(p)
            }
        }
    }

    /// C++ `PatternEquation::resolveOperandLeft(OperandResolve &state)`.  The
    /// equation needs each sub-equation's `resultpattern` (its
    /// [`TokenPattern`]'s length/ellipses); `ops` is the operand pattern list
    /// the caller already built (the same `ops` passed to [`Self::gen_pattern`]).
    pub fn resolve_operand_left(
        &self,
        id: EqId,
        state: &mut OperandResolve,
        ops: &[TokenPattern],
        sink: &mut dyn OperandResolveSink,
    ) -> KunaResult<bool> {
        match self.get(id) {
            PatternEquation::Operand { index } => {
                let index = *index;
                if sink.is_offset_irrelevant(index) {
                    sink.set_offset(index, -1, 0);
                    return Ok(true);
                }
                if state.base == -2 {
                    // We have no base
                    return Ok(false);
                }
                sink.set_offset(index, state.base, state.offset as u32);
                state.cur_rightmost = index;
                state.size = 0; // Distance from right edge
                Ok(true)
            }
            // UnconstrainedEquation / ValExpressEquation share resolveOperandLeft
            PatternEquation::Unconstrained { .. }
            | PatternEquation::Equal { .. }
            | PatternEquation::NotEqual { .. }
            | PatternEquation::Less { .. }
            | PatternEquation::LessEqual { .. }
            | PatternEquation::Greater { .. }
            | PatternEquation::GreaterEqual { .. } => {
                state.cur_rightmost = -1;
                let pat = self.gen_pattern(id, ops)?;
                if pat.get_left_ellipsis() || pat.get_right_ellipsis() {
                    state.size = -1; // don't know length
                } else {
                    state.size = pat.get_minimum_length();
                }
                Ok(true)
            }
            // EquationAnd / EquationOr share resolveOperandLeft
            PatternEquation::And { left, right } | PatternEquation::Or { left, right } => {
                let mut cur_rightmost = -1;
                let mut cur_size = -1;
                if !self.resolve_operand_left(*right, state, ops, sink)? {
                    return Ok(false);
                }
                if state.cur_rightmost != -1 && state.size != -1 {
                    cur_rightmost = state.cur_rightmost;
                    cur_size = state.size;
                }
                if !self.resolve_operand_left(*left, state, ops, sink)? {
                    return Ok(false);
                }
                if state.cur_rightmost == -1 || state.size == -1 {
                    state.cur_rightmost = cur_rightmost;
                    state.size = cur_size;
                }
                Ok(true)
            }
            PatternEquation::Cat { left, right } => {
                if !self.resolve_operand_left(*left, state, ops, sink)? {
                    return Ok(false);
                }
                let cur_base = state.base;
                let cur_offset = state.offset;
                let leftpat = self.gen_pattern(*left, ops)?;
                if !leftpat.get_left_ellipsis() && !leftpat.get_right_ellipsis() {
                    // Keep the same base
                    state.offset += leftpat.get_minimum_length();
                } else if state.cur_rightmost != -1 {
                    state.base = state.cur_rightmost;
                    state.offset = state.size;
                } else if state.size != -1 {
                    state.offset += state.size;
                } else {
                    state.base = -2; // We have no anchor
                }
                let cur_rightmost = state.cur_rightmost;
                let cur_size = state.size;
                if !self.resolve_operand_left(*right, state, ops, sink)? {
                    return Ok(false);
                }
                state.base = cur_base; // Restore base and offset
                state.offset = cur_offset;
                if state.cur_rightmost == -1
                    && state.size != -1
                    && cur_rightmost != -1
                    && cur_size != -1
                {
                    state.cur_rightmost = cur_rightmost;
                    state.size += cur_size;
                }
                Ok(true)
            }
            PatternEquation::LeftEllipsis { eq } => {
                let cur_base = state.base;
                state.base = -2;
                if !self.resolve_operand_left(*eq, state, ops, sink)? {
                    return Ok(false);
                }
                state.base = cur_base;
                Ok(true)
            }
            PatternEquation::RightEllipsis { eq } => {
                if !self.resolve_operand_left(*eq, state, ops, sink)? {
                    return Ok(false);
                }
                state.size = -1; // Cannot predict size
                Ok(true)
            }
        }
    }

    /// C++ `PatternEquation::operandOrder(Constructor*,vector<OperandSymbol*>
    /// &order)`: append the self-defining operand indices in left-to-right
    /// order, skipping ones already in `order` (the C++ `isMarked` set is the
    /// `seen` set here).
    pub fn operand_order(&self, id: EqId, order: &mut Vec<i32>, seen: &mut Vec<bool>) {
        match self.get(id) {
            PatternEquation::Operand { index } => {
                let index = *index;
                let idx = index as usize;
                if idx < seen.len() && !seen[idx] {
                    order.push(index);
                    seen[idx] = true;
                }
            }
            PatternEquation::And { left, right }
            | PatternEquation::Or { left, right }
            | PatternEquation::Cat { left, right } => {
                self.operand_order(*left, order, seen); // List operands left
                self.operand_order(*right, order, seen); //  to right
            }
            PatternEquation::LeftEllipsis { eq } | PatternEquation::RightEllipsis { eq } => {
                self.operand_order(*eq, order, seen);
            }
            // ValExpress/Unconstrained equations have no operandOrder override
            _ => {}
        }
    }
}

/// The six comparison flavors of `ValExpressEquation::genPattern`.
#[derive(Clone, Copy)]
enum Comparison {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// Shared body of the six `ValExpressEquation` subclasses' `genPattern`: for
/// every combination of rhs leaf values and every lhs value passing the
/// comparison, OR in `buildPattern(lhs,lhsval,semval,cur)`.
fn gen_comparison(
    lhs: &PatternValue,
    rhs: &PatternExpression,
    _ops: &[TokenPattern],
    cmp: Comparison,
) -> KunaResult<TokenPattern> {
    let lhsmin = lhs.min_value()?;
    let lhsmax = lhs.max_value()?;
    let mut semval: Vec<&PatternValue> = Vec::new();
    rhs.list_values(&mut semval);
    let mut min: Vec<i64> = Vec::new();
    let mut max: Vec<i64> = Vec::new();
    rhs.get_min_max(&mut min, &mut max)?;
    let mut cur = min.clone();

    let mut result: Option<TokenPattern> = None;
    loop {
        let val = rhs.get_sub_value_root(&cur)?;
        match cmp {
            Comparison::Equal => {
                if val >= lhsmin && val <= lhsmax {
                    let p = build_equation_pattern(lhs, val, &semval, &cur)?;
                    result = Some(match result {
                        None => p,
                        Some(acc) => acc.do_or(&p)?,
                    });
                }
            }
            _ => {
                let mut lhsval = lhsmin;
                while lhsval <= lhsmax {
                    let keep = match cmp {
                        Comparison::NotEqual => lhsval != val,
                        Comparison::Less => lhsval < val,
                        Comparison::LessEqual => lhsval <= val,
                        Comparison::Greater => lhsval > val,
                        Comparison::GreaterEqual => lhsval >= val,
                        Comparison::Equal => unreachable!(),
                    };
                    if keep {
                        let p = build_equation_pattern(lhs, lhsval, &semval, &cur)?;
                        result = Some(match result {
                            None => p,
                            Some(acc) => acc.do_or(&p)?,
                        });
                    }
                    lhsval += 1;
                }
            }
        }
        if !advance_combo(&mut cur, &min, &max) {
            break;
        }
    }
    match result {
        Some(p) => Ok(p),
        None => Err(KunaError::sleigh(match cmp {
            Comparison::Equal => "Equal constraint is impossible to match",
            Comparison::NotEqual => "Notequal constraint is impossible to match",
            Comparison::Less => "Less than constraint is impossible to match",
            Comparison::LessEqual => "Less than or equal constraint is impossible to match",
            Comparison::Greater => "Greater than constraint is impossible to match",
            Comparison::GreaterEqual => "Greater than or equal constraint is impossible to match",
        })),
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

    // -- build side (ws4a) ---------------------------------------------------

    fn op8_field(bstart: i32, bend: i32) -> PatternValue {
        // 1-byte little-endian token, index 0
        PatternValue::TokenField(TokenField::new_for_build(1, false, 0, false, bstart, bend))
    }

    fn cexpr(v: i64) -> PatternExpression {
        PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(v)))
    }

    #[test]
    fn token_pattern_do_and_two_fields() {
        // op8 high nibble == 0xa AND low nibble == 0x5 -> byte 0xa5
        let mut arena = EquationArena::new();
        let hi = arena.alloc(PatternEquation::Equal {
            lhs: op8_field(4, 7),
            rhs: cexpr(0xa),
        });
        let lo = arena.alloc(PatternEquation::Equal {
            lhs: op8_field(0, 3),
            rhs: cexpr(0x5),
        });
        let and = arena.alloc(PatternEquation::And { left: hi, right: lo });
        let tp = arena.gen_pattern(and, &[]).unwrap();
        // single instruction pattern with mask 0xff, value 0xa5
        let pat = tp.get_pattern();
        match pat {
            Pattern::Disjoint(DisjointPattern::Instruction(ip)) => {
                assert_eq!(ip.get_block().get_mask(0, 8), 0xff);
                assert_eq!(ip.get_block().get_value(0, 8), 0xa5);
            }
            other => panic!("expected instruction pattern, got {other:?}"),
        }
        assert_eq!(tp.get_minimum_length(), 1);
    }

    #[test]
    fn token_pattern_do_or_two_values() {
        // op8 == 0x10 OR op8 == 0x20
        let mut arena = EquationArena::new();
        let a = arena.alloc(PatternEquation::Equal {
            lhs: op8_field(0, 7),
            rhs: cexpr(0x10),
        });
        let b = arena.alloc(PatternEquation::Equal {
            lhs: op8_field(0, 7),
            rhs: cexpr(0x20),
        });
        let or = arena.alloc(PatternEquation::Or { left: a, right: b });
        let tp = arena.gen_pattern(or, &[]).unwrap();
        assert_eq!(tp.get_pattern().num_disjoint(), 2);
    }

    /// Records the offset sink writes for inspection.
    #[derive(Default)]
    struct RecordSink {
        irrelevant: Vec<i32>,
        offsets: Vec<(i32, i32, u32)>, // (index, base, reloffset)
    }

    impl OperandResolveSink for RecordSink {
        fn is_offset_irrelevant(&self, index: i32) -> bool {
            self.irrelevant.contains(&index)
        }
        fn set_offset(&mut self, index: i32, offsetbase: i32, reloffset: u32) {
            self.offsets.push((index, offsetbase, reloffset));
        }
    }

    #[test]
    fn resolve_operand_left_cat_offsets() {
        // EquationCat(Operand(0), Operand(1)) where operand 0 occupies a
        // 1-byte token (ops[0] is a token pattern) and operand 1 follows it.
        let mut arena = EquationArena::new();
        let op0 = arena.alloc(PatternEquation::Operand { index: 0 });
        let op1 = arena.alloc(PatternEquation::Operand { index: 1 });
        let cat = arena.alloc(PatternEquation::Cat {
            left: op0,
            right: op1,
        });
        // ops[0]: a 1-byte token pattern; ops[1]: empty (TRUE)
        let ops = vec![
            TokenPattern::new_token(BuildToken {
                size: 1,
                bigendian: false,
                index: 0,
            }),
            TokenPattern::new_true(),
        ];
        let mut state = OperandResolve::new();
        let mut sink = RecordSink::default();
        let ok = arena
            .resolve_operand_left(cat, &mut state, &ops, &mut sink)
            .unwrap();
        assert!(ok);
        // operand 0 anchored at base -1 offset 0; operand 1 at base -1 offset
        // 1 (after operand 0's 1-byte token).  Resolution runs left then
        // right: operand 0 sets (0,-1,0); the Cat advances offset by operand
        // 0's minimum length (1) before resolving operand 1.
        assert!(sink.offsets.contains(&(0, -1, 0)));
        assert!(sink.offsets.iter().any(|&(i, b, o)| i == 1 && b == -1 && o == 1));
    }

    #[test]
    fn operand_order_left_to_right() {
        // EquationCat(Operand(2), Operand(0)) lists operands in pattern order.
        let mut arena = EquationArena::new();
        let a = arena.alloc(PatternEquation::Operand { index: 2 });
        let b = arena.alloc(PatternEquation::Operand { index: 0 });
        let cat = arena.alloc(PatternEquation::Cat { left: a, right: b });
        let mut order = Vec::new();
        let mut seen = vec![false; 3];
        arena.operand_order(cat, &mut order, &mut seen);
        assert_eq!(order, vec![2, 0]);
        assert_eq!(seen, vec![true, false, true]);
    }

    #[test]
    fn equal_constraint_impossible_errors() {
        // op8 field [0,3] (max 15) == 100 is impossible.
        let mut arena = EquationArena::new();
        let eq = arena.alloc(PatternEquation::Equal {
            lhs: op8_field(0, 3),
            rhs: cexpr(100),
        });
        let err = arena.gen_pattern(eq, &[]).unwrap_err();
        assert!(format!("{err}").contains("impossible to match"));
    }
}
