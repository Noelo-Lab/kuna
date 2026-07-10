//! Port of `decompiler/cpp/slghpattern.{hh,cc}` (item `w2-sleigh-pattern`)
//! — instruction/context match patterns.
//!
//! ## Shape of the port
//!
//! The C++ class hierarchy
//!
//! ```text
//! Pattern
//!  +- DisjointPattern            (a pattern with no ORs in it)
//!  |   +- InstructionPattern     (matches the instruction bitstream)
//!  |   +- ContextPattern         (matches the context bitstream)
//!  |   +- CombinePattern         (a context piece and an instruction piece)
//!  +- OrPattern
//! ```
//!
//! maps onto the enums [`Pattern`] / [`DisjointPattern`] over the three
//! concrete structs, with [`PatternBlock`] as the shared mask/value pair.
//! Virtual dispatch becomes `match`; the C++ `dynamic_cast` ladders in the
//! algebra (`doAnd`/`doOr`/`commonSubPattern`) are transcribed in the exact
//! C++ cast order, and the C-style downcasts that would be UB on a type
//! confusion become panics (ADR 0004).  The C++ two-phase
//! construct-then-`decode` protocol becomes `decode` factory functions, so a
//! null `maskvalue` (only possible pre-decode upstream) cannot exist here.
//! `PatternBlock::clone()` is the derived [`Clone`].
//!
//! ## The `OrPattern::doOr` mutation quirk
//!
//! Upstream `OrPattern::doOr` is declared `const` but, when `sa < 0`, shifts
//! the elements of the receiver's OWN `orlist` (through the element
//! pointers) while pushing the *unshifted* clones into the result; with
//! `sa > 0` it shifts EVERY element of the result, the receiver's clones
//! included.  Both behaviors are transcribed exactly, which is why
//! [`Pattern::do_or`] takes `&mut self` and `&mut Pattern` (the C++ `const`
//! is a lie there, and delegation can make either operand the mutated
//! receiver).
//!
//! ## Walker boundary
//!
//! `isMatch`/`isInstructionMatch`/`isContextMatch` evaluate against a
//! `ParserWalker`, which does not exist yet in the port DAG; they take the
//! [`PatternExpressionContext`] boundary trait defined in
//! [`crate::slghpatexpress`] (see its module docs).
//!
//! ## sla format ids ([`sla`])
//!
//! The `.sla` ElementIds/AttributeIds live in C++ `slaformat.{hh,cc}` under
//! `FORMAT_SCOPE = 1`.  Scoped ids are never registered in the global
//! name->id registry (the C++ constructors skip registration for
//! `scope != 0`, and `.sla` streams are packed/id-numeric), so there is no
//! `IdRegistry` registration function here.  Only the ids used by
//! slghpattern/slghpatexpress are defined; they should migrate to
//! `slaformat.rs` when item `w2-sleigh-slaformat` ports that file.

use kuna_base::error::KunaResult;
use kuna_base::marshal::{Decoder, Encoder};
use kuna_base::types::Wrap;

use crate::slghpatexpress::PatternExpressionContext;

/// SLA-format ids used by the pattern/pattern-expression modules (C++
/// `slaformat.cc`, `FORMAT_SCOPE = 1`); see the module docs for why these
/// are not registered in any [`kuna_base::marshal::IdRegistry`].
pub mod sla {
    use kuna_base::marshal::{AttributeId, ElementId};

    pub const ATTRIB_VAL: AttributeId = AttributeId::new("val", 2);
    pub const ATTRIB_OFF: AttributeId = AttributeId::new("off", 6);
    pub const ATTRIB_MASK: AttributeId = AttributeId::new("mask", 8);
    pub const ATTRIB_INDEX: AttributeId = AttributeId::new("index", 9);
    pub const ATTRIB_NONZERO: AttributeId = AttributeId::new("nonzero", 10);
    pub const ATTRIB_STARTBIT: AttributeId = AttributeId::new("startbit", 14);
    pub const ATTRIB_TABLE: AttributeId = AttributeId::new("table", 16);
    pub const ATTRIB_CT: AttributeId = AttributeId::new("ct", 17);
    pub const ATTRIB_SHIFT: AttributeId = AttributeId::new("shift", 29);
    pub const ATTRIB_ENDBIT: AttributeId = AttributeId::new("endbit", 30);
    pub const ATTRIB_SIGNBIT: AttributeId = AttributeId::new("signbit", 31);
    pub const ATTRIB_ENDBYTE: AttributeId = AttributeId::new("endbyte", 32);
    pub const ATTRIB_STARTBYTE: AttributeId = AttributeId::new("startbyte", 33);
    pub const ATTRIB_BIGENDIAN: AttributeId = AttributeId::new("bigendian", 35);

    pub const ELEM_MASK_WORD: ElementId = ElementId::new("mask_word", 6);
    pub const ELEM_PAT_BLOCK: ElementId = ElementId::new("pat_block", 7);
    pub const ELEM_CONTEXT_PAT: ElementId = ElementId::new("context_pat", 10);
    pub const ELEM_OPERAND_EXP: ElementId = ElementId::new("operand_exp", 12);
    pub const ELEM_INSTRUCT_PAT: ElementId = ElementId::new("instruct_pat", 18);
    pub const ELEM_COMBINE_PAT: ElementId = ElementId::new("combine_pat", 19);
    pub const ELEM_TOKENFIELD: ElementId = ElementId::new("tokenfield", 27);
    pub const ELEM_CONTEXTFIELD: ElementId = ElementId::new("contextfield", 29);
    pub const ELEM_AND_EXP: ElementId = ElementId::new("and_exp", 47);
    pub const ELEM_DIV_EXP: ElementId = ElementId::new("div_exp", 48);
    pub const ELEM_LSHIFT_EXP: ElementId = ElementId::new("lshift_exp", 49);
    pub const ELEM_MINUS_EXP: ElementId = ElementId::new("minus_exp", 50);
    pub const ELEM_MULT_EXP: ElementId = ElementId::new("mult_exp", 51);
    pub const ELEM_NOT_EXP: ElementId = ElementId::new("not_exp", 52);
    pub const ELEM_OR_EXP: ElementId = ElementId::new("or_exp", 53);
    pub const ELEM_PLUS_EXP: ElementId = ElementId::new("plus_exp", 54);
    pub const ELEM_RSHIFT_EXP: ElementId = ElementId::new("rshift_exp", 55);
    pub const ELEM_SUB_EXP: ElementId = ElementId::new("sub_exp", 56);
    pub const ELEM_XOR_EXP: ElementId = ElementId::new("xor_exp", 57);
    pub const ELEM_INTB: ElementId = ElementId::new("intb", 58);
    pub const ELEM_END_EXP: ElementId = ElementId::new("end_exp", 59);
    pub const ELEM_NEXT2_EXP: ElementId = ElementId::new("next2_exp", 60);
    pub const ELEM_START_EXP: ElementId = ElementId::new("start_exp", 61);
    pub const ELEM_OR_PAT: ElementId = ElementId::new("or_pat", 78);
}

// ---------------------------------------------------------------------------
// PatternBlock
// ---------------------------------------------------------------------------

/// C++ `PatternBlock`: a mask/value pair viewed as two bitstreams.
#[derive(Debug, Clone)]
pub struct PatternBlock {
    /// Offset to non-zero byte of mask
    offset: i32,
    /// Last byte(+1) containing nonzero mask
    nonzerosize: i32,
    /// Mask
    maskvec: Vec<u32>,
    /// Value
    valvec: Vec<u32>,
}

impl PatternBlock {
    /// C++ `PatternBlock(int4 off,uintm msk,uintm val)`: define mask and
    /// value pattern, confined to one uintm.
    pub fn new(off: i32, msk: u32, val: u32) -> PatternBlock {
        let mut res = PatternBlock {
            offset: off,
            // Assume all non-zero bytes before normalization (sizeof(uintm))
            nonzerosize: 4,
            maskvec: vec![msk],
            valvec: vec![val],
        };
        res.normalize();
        res
    }

    /// C++ `PatternBlock(bool tf)`: an always-true or always-false block.
    pub fn new_always(tf: bool) -> PatternBlock {
        PatternBlock {
            offset: 0,
            nonzerosize: if tf { 0 } else { -1 },
            maskvec: Vec::new(),
            valvec: Vec::new(),
        }
    }

    /// C++ `PatternBlock(const PatternBlock *a,const PatternBlock *b)`:
    /// construct by ANDing two blocks together.
    pub fn new_and(a: &PatternBlock, b: &PatternBlock) -> PatternBlock {
        a.intersect(b)
    }

    /// C++ `PatternBlock(vector<PatternBlock *> &list)`: AND several blocks
    /// together to construct a new block (ownership replaces the C++
    /// delete-as-you-fold protocol).
    pub fn new_and_list(list: &[PatternBlock]) -> PatternBlock {
        if list.is_empty() {
            // If not ANDing anything, make constructed block always true
            return PatternBlock::new_always(true);
        }
        let mut res = list[0].clone();
        for next in &list[1..] {
            res = res.intersect(next);
        }
        res
    }

    /// C++ `PatternBlock::normalize`.
    fn normalize(&mut self) {
        if self.nonzerosize <= 0 {
            // Check if alwaystrue or alwaysfalse, in which case we don't
            // need mask and value
            self.offset = 0;
            self.maskvec.clear();
            self.valvec.clear();
            return;
        }

        // Cut zeros from beginning of mask
        let mut lead = 0usize;
        while lead < self.maskvec.len() && self.maskvec[lead] == 0 {
            lead += 1;
            self.offset += 4; // sizeof(uintm)
        }
        self.maskvec.drain(..lead);
        self.valvec.drain(..lead);

        if !self.maskvec.is_empty() {
            // Cut off unaligned zeros from beginning of mask
            let mut suboff: i32 = 0;
            let mut tmp = self.maskvec[0];
            while tmp != 0 {
                suboff += 1;
                tmp >>= 8;
            }
            suboff = 4 - suboff; // sizeof(uintm) - suboff
            if suboff != 0 {
                self.offset += suboff; // Slide up maskvec by suboff bytes
                let n = self.maskvec.len();
                for i in 0..n - 1 {
                    // suboff in [1,3] here (maskvec[0] != 0): shifts in [8,24]
                    let mut t = self.maskvec[i].wshl((suboff * 8) as u32);
                    t |= self.maskvec[i + 1].wshr(((4 - suboff) * 8) as u32);
                    self.maskvec[i] = t;
                }
                self.maskvec[n - 1] = self.maskvec[n - 1].wshl((suboff * 8) as u32);
                for i in 0..n - 1 {
                    // Slide up valvec by suboff bytes
                    let mut t = self.valvec[i].wshl((suboff * 8) as u32);
                    t |= self.valvec[i + 1].wshr(((4 - suboff) * 8) as u32);
                    self.valvec[i] = t;
                }
                self.valvec[n - 1] = self.valvec[n - 1].wshl((suboff * 8) as u32);
            }

            // Cut zeros from end of mask: walk iter1 back to the last
            // non-zero word (or begin()), then advance past it
            let mut i = self.maskvec.len();
            loop {
                if i == 0 {
                    break; // iter1 == maskvec.begin()
                }
                i -= 1;
                if self.maskvec[i] != 0 {
                    break; // Find last non-zero
                }
            }
            if i != self.maskvec.len() {
                i += 1; // Find first zero, in last zero chain
            }
            self.maskvec.truncate(i);
            self.valvec.truncate(i);
        }

        if self.maskvec.is_empty() {
            self.offset = 0;
            self.nonzerosize = 0; // Always true
            return;
        }
        // size_t -> int4: pattern masks are a handful of words
        self.nonzerosize = (self.maskvec.len() as i32) * 4;
        let mut tmp = *self.maskvec.last().unwrap(); // tmp must be nonzero
        while (tmp & 0xff) == 0 {
            self.nonzerosize -= 1;
            tmp >>= 8;
        }
    }

    /// C++ `PatternBlock::commonSubPattern`: the resulting pattern has a
    /// 1-bit in the mask only if the two pieces have a 1-bit and the values
    /// agree.
    pub fn common_sub_pattern(&self, b: &PatternBlock) -> PatternBlock {
        let mut res = PatternBlock::new_always(true);
        let maxlength = if self.get_length() > b.get_length() {
            self.get_length()
        } else {
            b.get_length()
        };

        res.offset = 0;
        let mut offset: i32 = 0; // local cursor (C++ shadows the member)
        while offset < maxlength {
            let mask1 = self.get_mask(offset * 8, 32);
            let val1 = self.get_value(offset * 8, 32);
            let mask2 = b.get_mask(offset * 8, 32);
            let val2 = b.get_value(offset * 8, 32);
            let resmask = mask1 & mask2 & !(val1 ^ val2);
            let resval = val1 & val2 & resmask;
            res.maskvec.push(resmask);
            res.valvec.push(resval);
            offset += 4;
        }
        res.nonzerosize = maxlength;
        res.normalize();
        res
    }

    /// C++ `PatternBlock::intersect`: construct the intersecting pattern.
    pub fn intersect(&self, b: &PatternBlock) -> PatternBlock {
        if self.always_false() || b.always_false() {
            return PatternBlock::new_always(false);
        }
        let mut res = PatternBlock::new_always(true);
        let maxlength = if self.get_length() > b.get_length() {
            self.get_length()
        } else {
            b.get_length()
        };

        res.offset = 0;
        let mut offset: i32 = 0; // local cursor (C++ shadows the member)
        while offset < maxlength {
            let mask1 = self.get_mask(offset * 8, 32);
            let val1 = self.get_value(offset * 8, 32);
            let mask2 = b.get_mask(offset * 8, 32);
            let val2 = b.get_value(offset * 8, 32);
            let commonmask = mask1 & mask2; // Bits in mask shared by both patterns
            if (commonmask & val1) != (commonmask & val2) {
                res.nonzerosize = -1; // Impossible pattern
                res.normalize();
                return res;
            }
            let resmask = mask1 | mask2;
            let resval = (mask1 & val1) | (mask2 & val2);
            res.maskvec.push(resmask);
            res.valvec.push(resval);
            offset += 4;
        }
        res.nonzerosize = maxlength;
        res.normalize();
        res
    }

    /// C++ `PatternBlock::specializes`: does every masked bit in `self`
    /// match the corresponding masked bit in `op2`.
    pub fn specializes(&self, op2: &PatternBlock) -> bool {
        let length = 8 * op2.get_length();
        let mut sbit: i32 = 0;
        while sbit < length {
            let mut tmplength = length - sbit;
            // C++ `tmplength > 8*sizeof(uintm)` compares int4 vs size_t:
            // tmplength converts to 64-bit unsigned (it is positive here)
            if (tmplength as i64 as u64) > 32 {
                tmplength = 32;
            }
            let mask1 = self.get_mask(sbit, tmplength);
            let value1 = self.get_value(sbit, tmplength);
            let mask2 = op2.get_mask(sbit, tmplength);
            let value2 = op2.get_value(sbit, tmplength);
            if (mask1 & mask2) != mask2 {
                return false;
            }
            if (value1 & mask2) != (value2 & mask2) {
                return false;
            }
            sbit += tmplength;
        }
        true
    }

    /// C++ `PatternBlock::identical`: do the mask and value match exactly.
    pub fn identical(&self, op2: &PatternBlock) -> bool {
        let mut length = 8 * op2.get_length();
        let tmplen = 8 * self.get_length();
        if tmplen > length {
            length = tmplen; // Maximum of two lengths
        }
        let mut sbit: i32 = 0;
        while sbit < length {
            let mut tmplength = length - sbit;
            // C++ int4 vs size_t mixed comparison (positive here)
            if (tmplength as i64 as u64) > 32 {
                tmplength = 32;
            }
            let mask1 = self.get_mask(sbit, tmplength);
            let value1 = self.get_value(sbit, tmplength);
            let mask2 = op2.get_mask(sbit, tmplength);
            let value2 = op2.get_value(sbit, tmplength);
            if mask1 != mask2 {
                return false;
            }
            if (mask1 & value1) != (mask2 & value2) {
                return false;
            }
            sbit += tmplength;
        }
        true
    }

    /// C++ `PatternBlock::shift`.
    pub fn shift(&mut self, sa: i32) {
        self.offset += sa;
        self.normalize();
    }

    /// C++ `PatternBlock::getLength`.
    pub fn get_length(&self) -> i32 {
        self.offset + self.nonzerosize
    }

    /// C++ `PatternBlock::getMask`.
    pub fn get_mask(&self, startbit: i32, size: i32) -> u32 {
        let startbit = startbit - 8 * self.offset;
        // Note the division and remainder here is unsigned (C++ divides the
        // int4 by `8*sizeof(uintm)`, a size_t, promoting the dividend to
        // 64-bit unsigned).  Then it is recast to signed.  If startbit is
        // negative, then wordnum1 is either negative or very big; in either
        // case, shift comes out between 0 and 31.
        let ustart = startbit as i64 as u64; // int4 -> size_t: sign-extend then reinterpret
        let wordnum1 = (ustart / 32) as i32; // size_t -> int4 truncation
        let shift = (ustart % 32) as i32; // in [0,31]
        let wordnum2 = ((startbit + size - 1) as i64 as u64 / 32) as i32; // same int4 -> size_t -> int4 path

        // (wordnum1<0)||(wordnum1>=maskvec.size()): the second compare is
        // int4 vs size_t in C++ but is only reached with wordnum1 >= 0
        let mut res: u32 = if wordnum1 < 0 || wordnum1 as usize >= self.maskvec.len() {
            0
        } else {
            self.maskvec[wordnum1 as usize] // wordnum1 >= 0 here
        };

        res = res.wshl(shift as u32); // shift in [0,31]
        if wordnum1 != wordnum2 {
            let tmp: u32 = if wordnum2 < 0 || wordnum2 as usize >= self.maskvec.len() {
                0
            } else {
                self.maskvec[wordnum2 as usize] // wordnum2 >= 0 here
            };
            // 32-shift: shift==0 cannot reach this branch for the size<=32
            // call sites (it would be shift-by-32 UB in C++); out-of-range
            // counts resolve x86-masked (ADR 0003)
            res |= tmp.wshr((32 - shift) as u32);
        }
        // size in (0,32] for all C++ call sites; x86-masked otherwise
        res = res.wshr((32 - size) as u32);

        res
    }

    /// C++ `PatternBlock::getValue`.
    pub fn get_value(&self, startbit: i32, size: i32) -> u32 {
        let startbit = startbit - 8 * self.offset;
        // Same unsigned division/remainder transcription as get_mask
        let ustart = startbit as i64 as u64; // int4 -> size_t: sign-extend then reinterpret
        let wordnum1 = (ustart / 32) as i32; // size_t -> int4 truncation
        let shift = (ustart % 32) as i32; // in [0,31]
        let wordnum2 = ((startbit + size - 1) as i64 as u64 / 32) as i32; // same int4 -> size_t -> int4 path

        let mut res: u32 = if wordnum1 < 0 || wordnum1 as usize >= self.valvec.len() {
            0
        } else {
            self.valvec[wordnum1 as usize] // wordnum1 >= 0 here
        };
        res = res.wshl(shift as u32); // shift in [0,31]
        if wordnum1 != wordnum2 {
            let tmp: u32 = if wordnum2 < 0 || wordnum2 as usize >= self.valvec.len() {
                0
            } else {
                self.valvec[wordnum2 as usize] // wordnum2 >= 0 here
            };
            // see get_mask for the shift-count notes
            res |= tmp.wshr((32 - shift) as u32);
        }
        res = res.wshr((32 - size) as u32);

        res
    }

    /// C++ `PatternBlock::alwaysTrue`.
    pub fn always_true(&self) -> bool {
        self.nonzerosize == 0
    }

    /// C++ `PatternBlock::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        self.nonzerosize == -1
    }

    /// C++ `PatternBlock::isInstructionMatch`.
    pub fn is_instruction_match(
        &self,
        walker: &dyn PatternExpressionContext,
    ) -> KunaResult<bool> {
        if self.nonzerosize <= 0 {
            return Ok(self.nonzerosize == 0);
        }
        let mut off = self.offset;
        for i in 0..self.maskvec.len() {
            let data = walker.get_instruction_bytes(off, 4)?;
            if (self.maskvec[i] & data) != self.valvec[i] {
                return Ok(false);
            }
            off += 4;
        }
        Ok(true)
    }

    /// C++ `PatternBlock::isContextMatch`.
    pub fn is_context_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        if self.nonzerosize <= 0 {
            return Ok(self.nonzerosize == 0);
        }
        let mut off = self.offset;
        for i in 0..self.maskvec.len() {
            let data = walker.get_context_bytes(off, 4)?;
            if (self.maskvec[i] & data) != self.valvec[i] {
                return Ok(false);
            }
            off += 4;
        }
        Ok(true)
    }

    /// C++ `PatternBlock::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_PAT_BLOCK);
        encoder.write_signed_integer(&sla::ATTRIB_OFF, i64::from(self.offset));
        encoder.write_signed_integer(&sla::ATTRIB_NONZERO, i64::from(self.nonzerosize));
        for i in 0..self.maskvec.len() {
            encoder.open_element(&sla::ELEM_MASK_WORD);
            encoder.write_unsigned_integer(&sla::ATTRIB_MASK, u64::from(self.maskvec[i]));
            encoder.write_unsigned_integer(&sla::ATTRIB_VAL, u64::from(self.valvec[i]));
            encoder.close_element(&sla::ELEM_MASK_WORD);
        }
        encoder.close_element(&sla::ELEM_PAT_BLOCK);
    }

    /// C++ `PatternBlock::decode` (decode factory; the C++ default-construct
    /// + `decode()` protocol collapses into this).
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<PatternBlock> {
        let el = decoder.open_element_id(&sla::ELEM_PAT_BLOCK)?;
        let offset = decoder.read_signed_integer_id(&sla::ATTRIB_OFF)? as i32; // C++ implicit intb -> int4
        let nonzerosize = decoder.read_signed_integer_id(&sla::ATTRIB_NONZERO)? as i32; // C++ implicit intb -> int4
        let mut maskvec = Vec::new();
        let mut valvec = Vec::new();
        while decoder.peek_element()? != 0 {
            let subel = decoder.open_element_id(&sla::ELEM_MASK_WORD)?;
            let mask = decoder.read_unsigned_integer_id(&sla::ATTRIB_MASK)? as u32; // C++ implicit uintb -> uintm
            let val = decoder.read_unsigned_integer_id(&sla::ATTRIB_VAL)? as u32; // C++ implicit uintb -> uintm
            maskvec.push(mask);
            valvec.push(val);
            decoder.close_element(subel)?;
        }
        let mut res = PatternBlock {
            offset,
            nonzerosize,
            maskvec,
            valvec,
        };
        res.normalize();
        decoder.close_element(el)?;
        Ok(res)
    }
}

// ---------------------------------------------------------------------------
// Concrete pattern types
// ---------------------------------------------------------------------------

/// C++ `InstructionPattern`: matches the instruction bitstream.
#[derive(Debug, Clone)]
pub struct InstructionPattern {
    maskvalue: PatternBlock,
}

impl InstructionPattern {
    /// C++ `InstructionPattern(PatternBlock *mv)`.
    pub fn new(mv: PatternBlock) -> InstructionPattern {
        InstructionPattern { maskvalue: mv }
    }

    /// C++ `InstructionPattern(bool tf)`.
    pub fn new_always(tf: bool) -> InstructionPattern {
        InstructionPattern {
            maskvalue: PatternBlock::new_always(tf),
        }
    }

    /// C++ `InstructionPattern::getBlock`.
    pub fn get_block(&self) -> &PatternBlock {
        &self.maskvalue
    }

    /// C++ `InstructionPattern::simplifyClone` (typed: always another
    /// InstructionPattern).
    pub fn simplify_clone_typed(&self) -> InstructionPattern {
        InstructionPattern {
            maskvalue: self.maskvalue.clone(),
        }
    }

    /// C++ `InstructionPattern::shiftInstruction`.
    pub fn shift_instruction(&mut self, sa: i32) {
        self.maskvalue.shift(sa);
    }

    /// C++ `InstructionPattern::isMatch`.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        self.maskvalue.is_instruction_match(walker)
    }

    /// C++ `InstructionPattern::alwaysTrue`.
    pub fn always_true(&self) -> bool {
        self.maskvalue.always_true()
    }

    /// C++ `InstructionPattern::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        self.maskvalue.always_false()
    }

    /// C++ `InstructionPattern::alwaysInstructionTrue`.
    pub fn always_instruction_true(&self) -> bool {
        self.maskvalue.always_true()
    }

    /// C++ `InstructionPattern::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_INSTRUCT_PAT);
        self.maskvalue.encode(encoder);
        encoder.close_element(&sla::ELEM_INSTRUCT_PAT);
    }

    /// C++ `InstructionPattern::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<InstructionPattern> {
        let el = decoder.open_element_id(&sla::ELEM_INSTRUCT_PAT)?;
        let maskvalue = PatternBlock::decode(decoder)?;
        decoder.close_element(el)?;
        Ok(InstructionPattern { maskvalue })
    }
}

/// C++ `ContextPattern`: matches the context bitstream.
#[derive(Debug, Clone)]
pub struct ContextPattern {
    maskvalue: PatternBlock,
}

impl ContextPattern {
    /// C++ `ContextPattern(PatternBlock *mv)`.
    pub fn new(mv: PatternBlock) -> ContextPattern {
        ContextPattern { maskvalue: mv }
    }

    /// C++ `ContextPattern::getBlock`.
    pub fn get_block(&self) -> &PatternBlock {
        &self.maskvalue
    }

    /// C++ `ContextPattern::simplifyClone` (typed).
    pub fn simplify_clone_typed(&self) -> ContextPattern {
        ContextPattern {
            maskvalue: self.maskvalue.clone(),
        }
    }

    /// C++ `ContextPattern::shiftInstruction`: do nothing.
    pub fn shift_instruction(&mut self, _sa: i32) {}

    /// C++ `ContextPattern::isMatch`.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        self.maskvalue.is_context_match(walker)
    }

    /// C++ `ContextPattern::alwaysTrue`.
    pub fn always_true(&self) -> bool {
        self.maskvalue.always_true()
    }

    /// C++ `ContextPattern::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        self.maskvalue.always_false()
    }

    /// C++ `ContextPattern::alwaysInstructionTrue`: always true.
    pub fn always_instruction_true(&self) -> bool {
        true
    }

    /// C++ `ContextPattern::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_CONTEXT_PAT);
        self.maskvalue.encode(encoder);
        encoder.close_element(&sla::ELEM_CONTEXT_PAT);
    }

    /// C++ `ContextPattern::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<ContextPattern> {
        let el = decoder.open_element_id(&sla::ELEM_CONTEXT_PAT)?;
        let maskvalue = PatternBlock::decode(decoder)?;
        decoder.close_element(el)?;
        Ok(ContextPattern { maskvalue })
    }
}

/// C++ `CombinePattern`: a pattern with a context piece and an instruction
/// piece.
#[derive(Debug, Clone)]
pub struct CombinePattern {
    /// Context piece
    context: ContextPattern,
    /// Instruction piece
    instr: InstructionPattern,
}

impl CombinePattern {
    /// C++ `CombinePattern(ContextPattern *con,InstructionPattern *in)`.
    pub fn new(con: ContextPattern, instr: InstructionPattern) -> CombinePattern {
        CombinePattern {
            context: con,
            instr,
        }
    }

    /// The context piece (C++ accesses the member directly).
    pub fn context(&self) -> &ContextPattern {
        &self.context
    }

    /// The instruction piece (C++ accesses the member directly).
    pub fn instr(&self) -> &InstructionPattern {
        &self.instr
    }

    /// C++ `CombinePattern::shiftInstruction`.
    pub fn shift_instruction(&mut self, sa: i32) {
        self.instr.shift_instruction(sa);
    }

    /// C++ `CombinePattern::isMatch`.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        if !self.instr.is_match(walker)? {
            return Ok(false);
        }
        if !self.context.is_match(walker)? {
            return Ok(false);
        }
        Ok(true)
    }

    /// C++ `CombinePattern::alwaysTrue`.
    pub fn always_true(&self) -> bool {
        self.context.always_true() && self.instr.always_true()
    }

    /// C++ `CombinePattern::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        self.context.always_false() || self.instr.always_false()
    }

    /// C++ `CombinePattern::alwaysInstructionTrue`.
    pub fn always_instruction_true(&self) -> bool {
        self.instr.always_instruction_true()
    }

    /// C++ `CombinePattern::simplifyClone`: we should only have to think at
    /// "our" level.
    pub fn simplify_clone(&self) -> Pattern {
        if self.context.always_true() {
            return Pattern::Disjoint(DisjointPattern::Instruction(
                self.instr.simplify_clone_typed(),
            ));
        }
        if self.instr.always_true() {
            return Pattern::Disjoint(DisjointPattern::Context(
                self.context.simplify_clone_typed(),
            ));
        }
        if self.context.always_false() || self.instr.always_false() {
            return Pattern::Disjoint(DisjointPattern::Instruction(
                InstructionPattern::new_always(false),
            ));
        }
        Pattern::Disjoint(DisjointPattern::Combine(CombinePattern {
            context: self.context.simplify_clone_typed(),
            instr: self.instr.simplify_clone_typed(),
        }))
    }

    /// C++ `CombinePattern::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_COMBINE_PAT);
        self.context.encode(encoder);
        self.instr.encode(encoder);
        encoder.close_element(&sla::ELEM_COMBINE_PAT);
    }

    /// C++ `CombinePattern::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<CombinePattern> {
        let el = decoder.open_element_id(&sla::ELEM_COMBINE_PAT)?;
        let context = ContextPattern::decode(decoder)?;
        let instr = InstructionPattern::decode(decoder)?;
        decoder.close_element(el)?;
        Ok(CombinePattern { context, instr })
    }
}

// ---------------------------------------------------------------------------
// DisjointPattern — a pattern with no ORs in it
// ---------------------------------------------------------------------------

/// The C++ abstract `DisjointPattern` over its three concrete subclasses.
#[derive(Debug, Clone)]
pub enum DisjointPattern {
    Instruction(InstructionPattern),
    Context(ContextPattern),
    Combine(CombinePattern),
}

/// C++ static `resolveIntersectBlock(PatternBlock*,PatternBlock*,PatternBlock*)`.
fn resolve_intersect_block(
    bl1: Option<&PatternBlock>,
    bl2: Option<&PatternBlock>,
    thisblock: Option<&PatternBlock>,
) -> bool {
    let inter_owned;
    let inter: Option<&PatternBlock> = match (bl1, bl2) {
        (None, _) => bl2,
        (_, None) => bl1,
        (Some(a), Some(b)) => {
            inter_owned = a.intersect(b);
            Some(&inter_owned)
        }
    };
    match inter {
        None => thisblock.is_none(), // C++: res = false only if thisblock != 0
        Some(ib) => match thisblock {
            None => false,
            Some(tb) => tb.identical(ib),
        },
    }
}

impl DisjointPattern {
    /// C++ virtual `getBlock(bool context)`: the mask/value block for the
    /// requested stream, or None (C++ null) when this pattern has no piece
    /// for it.
    pub fn get_block(&self, context: bool) -> Option<&PatternBlock> {
        match self {
            DisjointPattern::Instruction(p) => {
                if context {
                    None
                } else {
                    Some(&p.maskvalue)
                }
            }
            DisjointPattern::Context(p) => {
                if context {
                    Some(&p.maskvalue)
                } else {
                    None
                }
            }
            DisjointPattern::Combine(p) => {
                if context {
                    Some(&p.context.maskvalue)
                } else {
                    Some(&p.instr.maskvalue)
                }
            }
        }
    }

    /// C++ `DisjointPattern::getMask`.
    pub fn get_mask(&self, startbit: i32, size: i32, context: bool) -> u32 {
        match self.get_block(context) {
            Some(block) => block.get_mask(startbit, size),
            None => 0,
        }
    }

    /// C++ `DisjointPattern::getValue`.
    pub fn get_value(&self, startbit: i32, size: i32, context: bool) -> u32 {
        match self.get_block(context) {
            Some(block) => block.get_value(startbit, size),
            None => 0,
        }
    }

    /// C++ `DisjointPattern::getLength`.
    pub fn get_length(&self, context: bool) -> i32 {
        match self.get_block(context) {
            Some(block) => block.get_length(),
            None => 0,
        }
    }

    /// C++ `DisjointPattern::specializes`: return true if everywhere this's
    /// mask is non-zero, op2's mask is non-zero and op2's value matches.
    pub fn specializes(&self, op2: &DisjointPattern) -> bool {
        let a = self.get_block(false);
        let b = op2.get_block(false);
        if let Some(b) = b {
            if !b.always_true() {
                // a must match existing block
                match a {
                    None => return false,
                    Some(a) => {
                        if !a.specializes(b) {
                            return false;
                        }
                    }
                }
            }
        }
        let a = self.get_block(true);
        let b = op2.get_block(true);
        if let Some(b) = b {
            if !b.always_true() {
                // a must match existing block
                match a {
                    None => return false,
                    Some(a) => {
                        if !a.specializes(b) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// C++ `DisjointPattern::identical`: return true if patterns match
    /// exactly.
    pub fn identical(&self, op2: &DisjointPattern) -> bool {
        let a = self.get_block(false);
        let b = op2.get_block(false);
        match b {
            Some(b) => {
                // a must match existing block
                match a {
                    None => {
                        if !b.always_true() {
                            return false;
                        }
                    }
                    Some(a) => {
                        if !a.identical(b) {
                            return false;
                        }
                    }
                }
            }
            None => {
                if let Some(a) = a {
                    if !a.always_true() {
                        return false;
                    }
                }
            }
        }
        let a = self.get_block(true);
        let b = op2.get_block(true);
        match b {
            Some(b) => {
                // a must match existing block
                match a {
                    None => {
                        if !b.always_true() {
                            return false;
                        }
                    }
                    Some(a) => {
                        if !a.identical(b) {
                            return false;
                        }
                    }
                }
            }
            None => {
                if let Some(a) = a {
                    if !a.always_true() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// C++ `DisjointPattern::resolvesIntersect`: is this pattern equal to
    /// the intersection of `op1` and `op2`.
    pub fn resolves_intersect(&self, op1: &DisjointPattern, op2: &DisjointPattern) -> bool {
        if !resolve_intersect_block(
            op1.get_block(false),
            op2.get_block(false),
            self.get_block(false),
        ) {
            return false;
        }
        resolve_intersect_block(op1.get_block(true), op2.get_block(true), self.get_block(true))
    }

    /// C++ `DisjointPattern::decodeDisjoint`: DisjointPattern factory.
    /// Faithful to upstream, anything that is neither `instruct_pat` nor
    /// `context_pat` is decoded as a `combine_pat`.
    pub fn decode_disjoint(decoder: &mut dyn Decoder) -> KunaResult<DisjointPattern> {
        let el = decoder.peek_element()?;
        if el == sla::ELEM_INSTRUCT_PAT {
            Ok(DisjointPattern::Instruction(InstructionPattern::decode(
                decoder,
            )?))
        } else if el == sla::ELEM_CONTEXT_PAT {
            Ok(DisjointPattern::Context(ContextPattern::decode(decoder)?))
        } else {
            Ok(DisjointPattern::Combine(CombinePattern::decode(decoder)?))
        }
    }

    /// C++ virtual `isMatch` dispatch.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        match self {
            DisjointPattern::Instruction(p) => p.is_match(walker),
            DisjointPattern::Context(p) => p.is_match(walker),
            DisjointPattern::Combine(p) => p.is_match(walker),
        }
    }

    /// C++ virtual `shiftInstruction` dispatch.
    pub fn shift_instruction(&mut self, sa: i32) {
        match self {
            DisjointPattern::Instruction(p) => p.shift_instruction(sa),
            DisjointPattern::Context(p) => p.shift_instruction(sa),
            DisjointPattern::Combine(p) => p.shift_instruction(sa),
        }
    }

    /// C++ virtual `alwaysTrue` dispatch.
    pub fn always_true(&self) -> bool {
        match self {
            DisjointPattern::Instruction(p) => p.always_true(),
            DisjointPattern::Context(p) => p.always_true(),
            DisjointPattern::Combine(p) => p.always_true(),
        }
    }

    /// C++ virtual `alwaysFalse` dispatch.
    pub fn always_false(&self) -> bool {
        match self {
            DisjointPattern::Instruction(p) => p.always_false(),
            DisjointPattern::Context(p) => p.always_false(),
            DisjointPattern::Combine(p) => p.always_false(),
        }
    }

    /// C++ virtual `alwaysInstructionTrue` dispatch.
    pub fn always_instruction_true(&self) -> bool {
        match self {
            DisjointPattern::Instruction(p) => p.always_instruction_true(),
            DisjointPattern::Context(p) => p.always_instruction_true(),
            DisjointPattern::Combine(p) => p.always_instruction_true(),
        }
    }

    /// C++ virtual `simplifyClone` dispatch (a CombinePattern may collapse
    /// to one of its pieces, hence the `Pattern` return).
    pub fn simplify_clone(&self) -> Pattern {
        match self {
            DisjointPattern::Instruction(p) => {
                Pattern::Disjoint(DisjointPattern::Instruction(p.simplify_clone_typed()))
            }
            DisjointPattern::Context(p) => {
                Pattern::Disjoint(DisjointPattern::Context(p.simplify_clone_typed()))
            }
            DisjointPattern::Combine(p) => p.simplify_clone(),
        }
    }

    /// C++ virtual `encode` dispatch.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        match self {
            DisjointPattern::Instruction(p) => p.encode(encoder),
            DisjointPattern::Context(p) => p.encode(encoder),
            DisjointPattern::Combine(p) => p.encode(encoder),
        }
    }
}

// ---------------------------------------------------------------------------
// OrPattern
// ---------------------------------------------------------------------------

/// C++ `OrPattern`.
#[derive(Debug, Clone)]
pub struct OrPattern {
    orlist: Vec<DisjointPattern>,
}

impl OrPattern {
    /// C++ `OrPattern(DisjointPattern *a,DisjointPattern *b)`.
    pub fn new(a: DisjointPattern, b: DisjointPattern) -> OrPattern {
        OrPattern { orlist: vec![a, b] }
    }

    /// C++ `OrPattern(const vector<DisjointPattern *> &list)`.
    pub fn from_list(list: Vec<DisjointPattern>) -> OrPattern {
        OrPattern { orlist: list }
    }

    /// C++ `OrPattern::shiftInstruction`.
    pub fn shift_instruction(&mut self, sa: i32) {
        for pat in self.orlist.iter_mut() {
            pat.shift_instruction(sa);
        }
    }

    /// C++ `OrPattern::isMatch`.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        for pat in &self.orlist {
            if pat.is_match(walker)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// C++ `OrPattern::numDisjoint`.
    pub fn num_disjoint(&self) -> i32 {
        self.orlist.len() as i32 // size_t -> int4: or-lists are tiny
    }

    /// C++ `OrPattern::getDisjoint` (C++ indexes unchecked; an out-of-range
    /// index panics here, matching the UB boundary).
    pub fn get_disjoint(&self, i: i32) -> Option<&DisjointPattern> {
        Some(&self.orlist[i as usize]) // i >= 0 expected (C++ UB otherwise)
    }

    /// C++ `OrPattern::alwaysTrue`.  This isn't quite right because
    /// different branches may cover the entire gamut (upstream comment).
    pub fn always_true(&self) -> bool {
        for pat in &self.orlist {
            if pat.always_true() {
                return true;
            }
        }
        false
    }

    /// C++ `OrPattern::alwaysFalse`.
    pub fn always_false(&self) -> bool {
        for pat in &self.orlist {
            if !pat.always_false() {
                return false;
            }
        }
        true
    }

    /// C++ `OrPattern::alwaysInstructionTrue`.
    pub fn always_instruction_true(&self) -> bool {
        for pat in &self.orlist {
            if !pat.always_instruction_true() {
                return false;
            }
        }
        true
    }

    /// C++ `OrPattern::simplifyClone`: look for alwaysTrue, eliminate
    /// alwaysFalse.
    pub fn simplify_clone(&self) -> Pattern {
        for pat in &self.orlist {
            // Look for alwaysTrue
            if pat.always_true() {
                return Pattern::Disjoint(DisjointPattern::Instruction(
                    InstructionPattern::new_always(true),
                ));
            }
        }

        let mut newlist: Vec<DisjointPattern> = Vec::new();
        for pat in &self.orlist {
            // Look for alwaysFalse
            if !pat.always_false() {
                newlist.push(expect_disjoint(
                    pat.simplify_clone(),
                    "OrPattern::simplifyClone",
                ));
            }
        }

        if newlist.is_empty() {
            Pattern::Disjoint(DisjointPattern::Instruction(
                InstructionPattern::new_always(false),
            ))
        } else if newlist.len() == 1 {
            Pattern::Disjoint(newlist.pop().unwrap())
        } else {
            Pattern::Or(OrPattern { orlist: newlist })
        }
    }

    /// C++ `OrPattern::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_OR_PAT);
        for pat in &self.orlist {
            pat.encode(encoder);
        }
        encoder.close_element(&sla::ELEM_OR_PAT);
    }

    /// C++ `OrPattern::decode`.
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<OrPattern> {
        let el = decoder.open_element_id(&sla::ELEM_OR_PAT)?;
        let mut orlist = Vec::new();
        while decoder.peek_element()? != 0 {
            orlist.push(DisjointPattern::decode_disjoint(decoder)?);
        }
        decoder.close_element(el)?;
        Ok(OrPattern { orlist })
    }
}

// ---------------------------------------------------------------------------
// Pattern — the hierarchy root, and the doAnd/doOr/commonSubPattern algebra
// ---------------------------------------------------------------------------

/// The C++ abstract `Pattern` over the whole hierarchy.
#[derive(Debug, Clone)]
pub enum Pattern {
    Disjoint(DisjointPattern),
    Or(OrPattern),
}

/// Borrowed view used to transcribe the C++ `dynamic_cast` ladders of the
/// algebra without cloning (`this`/`b` are `const Pattern*` in C++).
#[derive(Clone, Copy)]
enum PatView<'a> {
    Instruction(&'a InstructionPattern),
    Context(&'a ContextPattern),
    Combine(&'a CombinePattern),
    Or(&'a OrPattern),
}

impl<'a> PatView<'a> {
    fn of_pattern(p: &'a Pattern) -> PatView<'a> {
        match p {
            Pattern::Disjoint(d) => PatView::of_disjoint(d),
            Pattern::Or(o) => PatView::Or(o),
        }
    }

    fn of_disjoint(d: &'a DisjointPattern) -> PatView<'a> {
        match d {
            DisjointPattern::Instruction(p) => PatView::Instruction(p),
            DisjointPattern::Context(p) => PatView::Context(p),
            DisjointPattern::Combine(p) => PatView::Combine(p),
        }
    }

    /// C++ virtual `numDisjoint` (0 for any DisjointPattern).
    fn num_disjoint(self) -> i32 {
        match self {
            PatView::Or(o) => o.num_disjoint(),
            _ => 0,
        }
    }

    /// C++ virtual `simplifyClone` through the view.
    fn simplify_clone(self) -> Pattern {
        match self {
            PatView::Instruction(p) => {
                Pattern::Disjoint(DisjointPattern::Instruction(p.simplify_clone_typed()))
            }
            PatView::Context(p) => {
                Pattern::Disjoint(DisjointPattern::Context(p.simplify_clone_typed()))
            }
            PatView::Combine(p) => p.simplify_clone(),
            PatView::Or(p) => p.simplify_clone(),
        }
    }
}

/// The C++ `(DisjointPattern *)` C-style downcast of an algebra result: a
/// type confusion is UB upstream, a panic here (ADR 0004).
fn expect_disjoint(p: Pattern, what: &str) -> DisjointPattern {
    match p {
        Pattern::Disjoint(d) => d,
        Pattern::Or(_) => panic!("{what}: result is not a DisjointPattern (C++ UB cast)"),
    }
}

/// The C++ `(ContextPattern *)` downcast of an algebra result.
fn expect_context(p: Pattern, what: &str) -> ContextPattern {
    match p {
        Pattern::Disjoint(DisjointPattern::Context(c)) => c,
        _ => panic!("{what}: result is not a ContextPattern (C++ UB cast)"),
    }
}

/// The C++ `(InstructionPattern *)` downcast of an algebra result.
fn expect_instruction(p: Pattern, what: &str) -> InstructionPattern {
    match p {
        Pattern::Disjoint(DisjointPattern::Instruction(i)) => i,
        _ => panic!("{what}: result is not an InstructionPattern (C++ UB cast)"),
    }
}

/// The virtual `doAnd` dispatch: each arm transcribes the corresponding C++
/// override, with the `dynamic_cast` ladder in the exact C++ order.
fn do_and_view(a: PatView<'_>, b: PatView<'_>, sa: i32) -> Pattern {
    match a {
        PatView::Instruction(a_ip) => {
            // InstructionPattern::doAnd
            if b.num_disjoint() > 0 {
                return do_and_view(b, a, -sa);
            }
            if matches!(b, PatView::Combine(_)) {
                return do_and_view(b, a, -sa);
            }
            if let PatView::Context(b3) = b {
                let mut newpat = a_ip.simplify_clone_typed();
                if sa < 0 {
                    newpat.shift_instruction(-sa);
                }
                return Pattern::Disjoint(DisjointPattern::Combine(CombinePattern::new(
                    b3.simplify_clone_typed(),
                    newpat,
                )));
            }
            // C++ falls through to `(const InstructionPattern *)b`
            let b4 = match b {
                PatView::Instruction(x) => x,
                _ => panic!("InstructionPattern::doAnd: operand is not an InstructionPattern (C++ UB cast)"),
            };
            let respattern = if sa < 0 {
                let mut a_block = a_ip.maskvalue.clone();
                a_block.shift(-sa);
                a_block.intersect(&b4.maskvalue)
            } else {
                let mut c = b4.maskvalue.clone();
                c.shift(sa);
                a_ip.maskvalue.intersect(&c)
            };
            Pattern::Disjoint(DisjointPattern::Instruction(InstructionPattern::new(
                respattern,
            )))
        }
        PatView::Context(a_cp) => {
            // ContextPattern::doAnd
            if let PatView::Context(b2) = b {
                let resblock = a_cp.maskvalue.intersect(&b2.maskvalue);
                Pattern::Disjoint(DisjointPattern::Context(ContextPattern::new(resblock)))
            } else {
                do_and_view(b, a, -sa)
            }
        }
        PatView::Combine(a_cb) => {
            // CombinePattern::doAnd
            if b.num_disjoint() != 0 {
                return do_and_view(b, a, -sa);
            }
            if let PatView::Combine(b2) = b {
                let c = expect_context(
                    do_and_view(PatView::Context(&a_cb.context), PatView::Context(&b2.context), 0),
                    "CombinePattern::doAnd",
                );
                let i = expect_instruction(
                    do_and_view(PatView::Instruction(&a_cb.instr), PatView::Instruction(&b2.instr), sa),
                    "CombinePattern::doAnd",
                );
                Pattern::Disjoint(DisjointPattern::Combine(CombinePattern::new(c, i)))
            } else if let PatView::Instruction(b3) = b {
                let i = expect_instruction(
                    do_and_view(PatView::Instruction(&a_cb.instr), PatView::Instruction(b3), sa),
                    "CombinePattern::doAnd",
                );
                Pattern::Disjoint(DisjointPattern::Combine(CombinePattern::new(
                    a_cb.context.simplify_clone_typed(),
                    i,
                )))
            } else {
                // Must be a ContextPattern
                let c = expect_context(
                    do_and_view(PatView::Context(&a_cb.context), b, 0),
                    "CombinePattern::doAnd",
                );
                let mut newpat = a_cb.instr.simplify_clone_typed();
                if sa < 0 {
                    newpat.shift_instruction(-sa);
                }
                Pattern::Disjoint(DisjointPattern::Combine(CombinePattern::new(c, newpat)))
            }
        }
        PatView::Or(a_or) => {
            // OrPattern::doAnd
            let mut newlist: Vec<DisjointPattern> = Vec::new();
            if let PatView::Or(b2) = b {
                for pat in &a_or.orlist {
                    for pat2 in &b2.orlist {
                        let tmp = expect_disjoint(
                            do_and_view(PatView::of_disjoint(pat), PatView::of_disjoint(pat2), sa),
                            "OrPattern::doAnd",
                        );
                        newlist.push(tmp);
                    }
                }
            } else {
                for pat in &a_or.orlist {
                    let tmp = expect_disjoint(
                        do_and_view(PatView::of_disjoint(pat), b, sa),
                        "OrPattern::doAnd",
                    );
                    newlist.push(tmp);
                }
            }
            Pattern::Or(OrPattern::from_list(newlist))
        }
    }
}

/// The virtual `commonSubPattern` dispatch, transcribed like [`do_and_view`].
fn common_sub_view(a: PatView<'_>, b: PatView<'_>, sa: i32) -> Pattern {
    match a {
        PatView::Instruction(a_ip) => {
            // InstructionPattern::commonSubPattern
            if b.num_disjoint() > 0 {
                return common_sub_view(b, a, -sa);
            }
            if matches!(b, PatView::Combine(_)) {
                return common_sub_view(b, a, -sa);
            }
            if matches!(b, PatView::Context(_)) {
                return Pattern::Disjoint(DisjointPattern::Instruction(
                    InstructionPattern::new_always(true),
                ));
            }
            let b4 = match b {
                PatView::Instruction(x) => x,
                _ => panic!("InstructionPattern::commonSubPattern: operand is not an InstructionPattern (C++ UB cast)"),
            };
            let respattern = if sa < 0 {
                let mut a_block = a_ip.maskvalue.clone();
                a_block.shift(-sa);
                a_block.common_sub_pattern(&b4.maskvalue)
            } else {
                let mut c = b4.maskvalue.clone();
                c.shift(sa);
                a_ip.maskvalue.common_sub_pattern(&c)
            };
            Pattern::Disjoint(DisjointPattern::Instruction(InstructionPattern::new(
                respattern,
            )))
        }
        PatView::Context(a_cp) => {
            // ContextPattern::commonSubPattern
            if let PatView::Context(b2) = b {
                let resblock = a_cp.maskvalue.common_sub_pattern(&b2.maskvalue);
                Pattern::Disjoint(DisjointPattern::Context(ContextPattern::new(resblock)))
            } else {
                common_sub_view(b, a, -sa)
            }
        }
        PatView::Combine(a_cb) => {
            // CombinePattern::commonSubPattern
            if b.num_disjoint() != 0 {
                return common_sub_view(b, a, -sa);
            }
            if let PatView::Combine(b2) = b {
                let c = expect_context(
                    common_sub_view(
                        PatView::Context(&a_cb.context),
                        PatView::Context(&b2.context),
                        0,
                    ),
                    "CombinePattern::commonSubPattern",
                );
                let i = expect_instruction(
                    common_sub_view(
                        PatView::Instruction(&a_cb.instr),
                        PatView::Instruction(&b2.instr),
                        sa,
                    ),
                    "CombinePattern::commonSubPattern",
                );
                Pattern::Disjoint(DisjointPattern::Combine(CombinePattern::new(c, i)))
            } else if matches!(b, PatView::Instruction(_)) {
                // returned raw (no cast) in C++
                common_sub_view(PatView::Instruction(&a_cb.instr), b, sa)
            } else {
                // Must be a ContextPattern; returned raw (no cast) in C++
                common_sub_view(PatView::Context(&a_cb.context), b, 0)
            }
        }
        PatView::Or(a_or) => {
            // OrPattern::commonSubPattern (an empty orlist is C++ UB:
            // *orlist.begin() on an empty vector; indexing panics here)
            let mut res = common_sub_view(PatView::of_disjoint(&a_or.orlist[0]), b, sa);
            let mut sa = sa;
            if sa > 0 {
                sa = 0;
            }
            for pat in &a_or.orlist[1..] {
                let next = common_sub_view(PatView::of_disjoint(pat), PatView::of_pattern(&res), sa);
                res = next;
            }
            res
        }
    }
}

impl Pattern {
    /// C++ virtual `simplifyClone`.
    pub fn simplify_clone(&self) -> Pattern {
        PatView::of_pattern(self).simplify_clone()
    }

    /// C++ virtual `shiftInstruction`.
    pub fn shift_instruction(&mut self, sa: i32) {
        match self {
            Pattern::Disjoint(d) => d.shift_instruction(sa),
            Pattern::Or(o) => o.shift_instruction(sa),
        }
    }

    /// C++ virtual `doAnd`.
    pub fn do_and(&self, b: &Pattern, sa: i32) -> Pattern {
        do_and_view(PatView::of_pattern(self), PatView::of_pattern(b), sa)
    }

    /// C++ virtual `commonSubPattern`.
    pub fn common_sub_pattern(&self, b: &Pattern, sa: i32) -> Pattern {
        common_sub_view(PatView::of_pattern(self), PatView::of_pattern(b), sa)
    }

    /// C++ virtual `doOr`.
    ///
    /// Takes `&mut self` AND `&mut b` to transcribe the upstream
    /// `OrPattern::doOr` quirk exactly (see module docs): when the receiver
    /// resolving the call is an OrPattern and the (possibly negated) shift
    /// is negative, the receiver's OWN orlist elements are shifted while the
    /// result keeps the unshifted clones.  Delegation (`b->doOr(this,-sa)`)
    /// can make either operand that receiver.
    pub fn do_or(&mut self, b: &mut Pattern, sa: i32) -> Pattern {
        match self {
            Pattern::Disjoint(DisjointPattern::Instruction(_)) => {
                // InstructionPattern::doOr
                if b.num_disjoint() > 0 {
                    return b.do_or(self, -sa);
                }
                if matches!(b, Pattern::Disjoint(DisjointPattern::Combine(_))) {
                    return b.do_or(self, -sa);
                }
                let mut res1 = expect_disjoint(self.simplify_clone(), "InstructionPattern::doOr");
                let mut res2 = expect_disjoint(b.simplify_clone(), "InstructionPattern::doOr");
                if sa < 0 {
                    res1.shift_instruction(-sa);
                } else {
                    res2.shift_instruction(sa);
                }
                Pattern::Or(OrPattern::new(res1, res2))
            }
            Pattern::Disjoint(DisjointPattern::Context(_)) => {
                // ContextPattern::doOr
                if !matches!(b, Pattern::Disjoint(DisjointPattern::Context(_))) {
                    return b.do_or(self, -sa);
                }
                // NOTE: no shifting here, exactly as upstream
                let res1 = expect_disjoint(self.simplify_clone(), "ContextPattern::doOr");
                let res2 = expect_disjoint(b.simplify_clone(), "ContextPattern::doOr");
                Pattern::Or(OrPattern::new(res1, res2))
            }
            Pattern::Disjoint(DisjointPattern::Combine(_)) => {
                // CombinePattern::doOr
                if b.num_disjoint() != 0 {
                    return b.do_or(self, -sa);
                }
                let mut res1 = expect_disjoint(self.simplify_clone(), "CombinePattern::doOr");
                let mut res2 = expect_disjoint(b.simplify_clone(), "CombinePattern::doOr");
                if sa < 0 {
                    res1.shift_instruction(-sa);
                } else {
                    res2.shift_instruction(sa);
                }
                Pattern::Or(OrPattern::new(res1, res2))
            }
            Pattern::Or(_) => {
                // OrPattern::doOr — transcribed exactly, including the
                // upstream quirk: the method is declared const but, when
                // sa < 0, shifts the ORIGINAL orlist elements (through the
                // element pointers) while the result keeps the unshifted
                // clones; when sa > 0 it shifts EVERY element of the result
                // (the receiver's clones included).
                let b_is_or = matches!(b, Pattern::Or(_)); // b2 = dynamic_cast<OrPattern>(b)
                let or = match self {
                    Pattern::Or(o) => o,
                    _ => unreachable!(),
                };
                let mut newlist: Vec<DisjointPattern> = Vec::new();
                for pat in &or.orlist {
                    newlist.push(expect_disjoint(pat.simplify_clone(), "OrPattern::doOr"));
                }
                if sa < 0 {
                    for pat in or.orlist.iter_mut() {
                        pat.shift_instruction(-sa);
                    }
                }
                if !b_is_or {
                    newlist.push(expect_disjoint(b.simplify_clone(), "OrPattern::doOr"));
                } else if let Pattern::Or(b2) = b {
                    for pat in &b2.orlist {
                        newlist.push(expect_disjoint(pat.simplify_clone(), "OrPattern::doOr"));
                    }
                }
                if sa > 0 {
                    for pat in newlist.iter_mut() {
                        pat.shift_instruction(sa);
                    }
                }
                Pattern::Or(OrPattern::from_list(newlist))
            }
        }
    }

    /// C++ virtual `isMatch`: does this pattern match context.
    pub fn is_match(&self, walker: &dyn PatternExpressionContext) -> KunaResult<bool> {
        match self {
            Pattern::Disjoint(d) => d.is_match(walker),
            Pattern::Or(o) => o.is_match(walker),
        }
    }

    /// C++ virtual `numDisjoint` (0 for any DisjointPattern).
    pub fn num_disjoint(&self) -> i32 {
        match self {
            Pattern::Disjoint(_) => 0,
            Pattern::Or(o) => o.num_disjoint(),
        }
    }

    /// C++ virtual `getDisjoint` (null — `None` — for any DisjointPattern).
    pub fn get_disjoint(&self, i: i32) -> Option<&DisjointPattern> {
        match self {
            Pattern::Disjoint(_) => None,
            Pattern::Or(o) => o.get_disjoint(i),
        }
    }

    /// C++ virtual `alwaysTrue`.
    pub fn always_true(&self) -> bool {
        match self {
            Pattern::Disjoint(d) => d.always_true(),
            Pattern::Or(o) => o.always_true(),
        }
    }

    /// C++ virtual `alwaysFalse`.
    pub fn always_false(&self) -> bool {
        match self {
            Pattern::Disjoint(d) => d.always_false(),
            Pattern::Or(o) => o.always_false(),
        }
    }

    /// C++ virtual `alwaysInstructionTrue`.
    pub fn always_instruction_true(&self) -> bool {
        match self {
            Pattern::Disjoint(d) => d.always_instruction_true(),
            Pattern::Or(o) => o.always_instruction_true(),
        }
    }

    /// C++ virtual `encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        match self {
            Pattern::Disjoint(d) => d.encode(encoder),
            Pattern::Or(o) => o.encode(encoder),
        }
    }
}

#[cfg(test)]
mod tests {
    use kuna_base::address::Address;
    use kuna_base::error::KunaError;
    use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
    use kuna_base::space::{AddrSpaceManager, ConstantSpace};
    use std::rc::Rc;

    use super::*;

    /// Minimal synthetic walker: instruction bytes packed big-endian,
    /// context provided as 32-bit words (mirrors ParserContext semantics).
    struct ByteWalker {
        instr: Vec<u8>,
        ctx: Vec<u32>,
    }

    impl ByteWalker {
        fn instr(bytes: &[u8]) -> ByteWalker {
            ByteWalker {
                instr: bytes.to_vec(),
                ctx: Vec::new(),
            }
        }

        fn ctx(words: &[u32]) -> ByteWalker {
            ByteWalker {
                instr: Vec::new(),
                ctx: words.to_vec(),
            }
        }
    }

    impl PatternExpressionContext for ByteWalker {
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
            unimplemented!("pattern tests never read addresses")
        }

        fn get_naddr(&self) -> Address {
            unimplemented!("pattern tests never read addresses")
        }

        fn get_n2addr(&self) -> KunaResult<Address> {
            unimplemented!("pattern tests never read addresses")
        }

        fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
            unimplemented!("pattern tests never evaluate operands")
        }
    }

    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
    }

    /// instruction pattern requiring `byte[pos] == val`
    fn ip_byte(pos: i32, val: u8) -> InstructionPattern {
        let shift_words = pos / 4;
        let inner = pos % 4;
        InstructionPattern::new(PatternBlock::new(
            shift_words * 4,
            0xff00_0000u32 >> (inner * 8),
            u32::from(val) << 24 >> (inner * 8),
        ))
    }

    // -- PatternBlock -------------------------------------------------------

    #[test]
    fn patternblock_normalize_offsets_and_length() {
        // mask 0x00ff0000 at word offset 0 normalizes to byte offset 1
        let blk = PatternBlock::new(0, 0x00ff_0000, 0x00ab_0000);
        assert_eq!(blk.get_length(), 2);
        assert_eq!(blk.get_mask(8, 8), 0xff);
        assert_eq!(blk.get_value(8, 8), 0xab);
        // bits before the block read as zero
        assert_eq!(blk.get_mask(0, 8), 0);
        assert_eq!(blk.get_value(0, 8), 0);
    }

    #[test]
    fn patternblock_getmask_negative_startbit_is_zero() {
        // exercises the C++ unsigned division path: startbit - 8*offset < 0
        let blk = PatternBlock::new(4, 0xff00_0000, 0x1200_0000);
        assert_eq!(blk.get_mask(0, 8), 0); // startbit -32 relative to block
        assert_eq!(blk.get_value(0, 8), 0);
        assert_eq!(blk.get_mask(16, 32), 0x0000_ff00);
        assert_eq!(blk.get_value(32, 8), 0x12);
    }

    #[test]
    fn patternblock_always_true_false() {
        let t = PatternBlock::new_always(true);
        assert!(t.always_true());
        assert!(!t.always_false());
        assert_eq!(t.get_length(), 0);
        let f = PatternBlock::new_always(false);
        assert!(f.always_false());
        assert!(!f.always_true());
        // a fully-zero mask normalizes to always true
        let z = PatternBlock::new(0, 0, 0x1234_5678);
        assert!(z.always_true());
    }

    #[test]
    fn patternblock_intersect_and_impossible() {
        let a = PatternBlock::new(0, 0xff00_0000, 0xab00_0000);
        let b = PatternBlock::new(0, 0x00ff_0000, 0x00cd_0000);
        let res = a.intersect(&b);
        assert_eq!(res.get_length(), 2);
        assert_eq!(res.get_mask(0, 16), 0xffff);
        assert_eq!(res.get_value(0, 16), 0xabcd);
        // conflicting values on a shared mask bit -> impossible (alwaysFalse)
        let c = PatternBlock::new(0, 0xff00_0000, 0x1200_0000);
        let d = PatternBlock::new(0, 0xff00_0000, 0x3400_0000);
        assert!(c.intersect(&d).always_false());
        // new_and / new_and_list agree with intersect
        assert!(PatternBlock::new_and(&a, &b).identical(&res));
        assert!(PatternBlock::new_and_list(&[a.clone(), b.clone()]).identical(&res));
        assert!(PatternBlock::new_and_list(&[]).always_true());
    }

    #[test]
    fn patternblock_common_sub_pattern() {
        // byte0 agrees (0xab), byte1 disagrees on bits 0x26
        let a = PatternBlock::new(0, 0xffff_0000, 0xab12_0000);
        let b = PatternBlock::new(0, 0xffff_0000, 0xab34_0000);
        let res = a.common_sub_pattern(&b);
        assert_eq!(res.get_mask(0, 16), 0xffd9);
        assert_eq!(res.get_value(0, 16), 0xab10);
    }

    #[test]
    fn patternblock_specializes_and_identical() {
        let fine = PatternBlock::new(0, 0xffff_0000, 0xab12_0000);
        let coarse = PatternBlock::new(0, 0xff00_0000, 0xab00_0000);
        assert!(fine.specializes(&coarse));
        assert!(!coarse.specializes(&fine));
        assert!(fine.identical(&fine.clone()));
        assert!(!fine.identical(&coarse));
    }

    #[test]
    fn patternblock_shift_and_multiword() {
        let mut a = PatternBlock::new(0, 0xff00_0000, 0xab00_0000);
        a.shift(2);
        assert_eq!(a.get_length(), 3);
        assert_eq!(a.get_value(16, 8), 0xab);
        // intersect across word boundary: byte 0 and byte 4 requirements
        let b = PatternBlock::new(0, 0xff00_0000, 0x1100_0000);
        let c = PatternBlock::new(4, 0xff00_0000, 0xcd00_0000);
        let m = b.intersect(&c);
        assert_eq!(m.get_length(), 5);
        assert_eq!(m.get_value(0, 8), 0x11);
        assert_eq!(m.get_value(32, 8), 0xcd);
    }

    #[test]
    fn patternblock_instruction_and_context_match() {
        let blk = PatternBlock::new(0, 0xff00_0000, 0xab00_0000);
        assert!(blk
            .is_instruction_match(&ByteWalker::instr(&[0xab, 1, 2, 3]))
            .unwrap());
        assert!(!blk
            .is_instruction_match(&ByteWalker::instr(&[0xac, 1, 2, 3]))
            .unwrap());
        assert!(blk
            .is_context_match(&ByteWalker::ctx(&[0xab12_3456]))
            .unwrap());
        assert!(!blk.is_context_match(&ByteWalker::ctx(&[0x0b12_3456])).unwrap());
        // alwaysTrue/alwaysFalse short-circuit without touching the walker
        assert!(PatternBlock::new_always(true)
            .is_instruction_match(&ByteWalker::instr(&[]))
            .unwrap());
        assert!(!PatternBlock::new_always(false)
            .is_instruction_match(&ByteWalker::instr(&[]))
            .unwrap());
        // walker errors propagate (C++ BadDataError)
        assert!(blk.is_instruction_match(&ByteWalker::instr(&[0xab])).is_err());
    }

    #[test]
    fn patternblock_encode_decode_roundtrip() {
        let manager = test_manager();
        let blk = PatternBlock::new(0, 0xffff_0000, 0xab12_0000);
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            blk.encode(&mut enc);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let rt = PatternBlock::decode(&mut dec).unwrap();
        assert!(rt.identical(&blk));
        let mut buf2 = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf2);
            rt.encode(&mut enc);
        }
        assert_eq!(buf2, buf);
    }

    // -- pattern matching ------------------------------------------------------

    #[test]
    fn instruction_context_combine_or_matching() {
        let ip = ip_byte(0, 0xab);
        assert!(ip.is_match(&ByteWalker::instr(&[0xab, 0, 0, 0])).unwrap());
        assert!(!ip.is_match(&ByteWalker::instr(&[0xcd, 0, 0, 0])).unwrap());

        let cp = ContextPattern::new(PatternBlock::new(0, 0xf000_0000, 0x3000_0000));
        assert!(cp.is_match(&ByteWalker::ctx(&[0x3abc_0000])).unwrap());
        assert!(!cp.is_match(&ByteWalker::ctx(&[0x4abc_0000])).unwrap());
        assert!(cp.always_instruction_true());

        // Combine: both pieces must match; the walker carries both streams
        let comb = CombinePattern::new(cp.clone(), ip.simplify_clone_typed());
        let mut walker = ByteWalker::instr(&[0xab, 0, 0, 0]);
        walker.ctx = vec![0x3abc_0000];
        assert!(comb.is_match(&walker).unwrap());
        walker.ctx = vec![0x4abc_0000];
        assert!(!comb.is_match(&walker).unwrap());

        // Or: second branch matches
        let or = OrPattern::new(
            DisjointPattern::Instruction(ip_byte(0, 0x11)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        );
        assert!(or.is_match(&ByteWalker::instr(&[0x22, 0, 0, 0])).unwrap());
        assert!(!or.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0])).unwrap());
        assert_eq!(or.num_disjoint(), 2);
    }

    // -- algebra -----------------------------------------------------------------

    #[test]
    fn do_and_instruction_with_context_builds_combine() {
        let ip = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0xab)));
        let cp = Pattern::Disjoint(DisjointPattern::Context(ContextPattern::new(
            PatternBlock::new(0, 0xf000_0000, 0x3000_0000),
        )));
        let combined = ip.do_and(&cp, 0);
        assert!(matches!(
            combined,
            Pattern::Disjoint(DisjointPattern::Combine(_))
        ));
        let mut walker = ByteWalker::instr(&[0xab, 0, 0, 0]);
        walker.ctx = vec![0x3000_0000];
        assert!(combined.is_match(&walker).unwrap());
        walker.instr[0] = 0xac;
        assert!(!combined.is_match(&walker).unwrap());
        // symmetric direction delegates through ContextPattern::doAnd
        let combined2 = cp.do_and(&ip, 0);
        assert!(matches!(
            combined2,
            Pattern::Disjoint(DisjointPattern::Combine(_))
        ));
    }

    #[test]
    fn do_and_instruction_shift_alignment() {
        // a requires byte0==0xab; b requires byte0==0xcd; with sa=1, b's
        // requirement lands on byte 1 of the result
        let a = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0xab)));
        let b = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0xcd)));
        let res = a.do_and(&b, 1);
        assert!(res.is_match(&ByteWalker::instr(&[0xab, 0xcd, 0, 0, 0])).unwrap());
        assert!(!res.is_match(&ByteWalker::instr(&[0xab, 0xab, 0, 0, 0])).unwrap());
        // negative sa shifts self instead
        let res2 = a.do_and(&b, -1);
        assert!(res2.is_match(&ByteWalker::instr(&[0xcd, 0xab, 0, 0, 0])).unwrap());
    }

    #[test]
    fn do_and_or_pattern_distributes() {
        let or = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(ip_byte(0, 0x11)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        ));
        let req = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(1, 0x33)));
        let res = or.do_and(&req, 0);
        assert_eq!(res.num_disjoint(), 2);
        assert!(res.is_match(&ByteWalker::instr(&[0x11, 0x33, 0, 0, 0])).unwrap());
        assert!(res.is_match(&ByteWalker::instr(&[0x22, 0x33, 0, 0, 0])).unwrap());
        assert!(!res.is_match(&ByteWalker::instr(&[0x11, 0x44, 0, 0, 0])).unwrap());
    }

    #[test]
    fn do_or_builds_or_pattern() {
        let mut a = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x11)));
        let mut b = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x22)));
        let res = a.do_or(&mut b, 0);
        assert_eq!(res.num_disjoint(), 2);
        assert!(res.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0])).unwrap());
        assert!(res.is_match(&ByteWalker::instr(&[0x22, 0, 0, 0])).unwrap());
        assert!(!res.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0])).unwrap());
        // positive sa shifts the b clone
        let mut a2 = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x11)));
        let mut b2 = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x22)));
        let res2 = a2.do_or(&mut b2, 1);
        assert!(res2.is_match(&ByteWalker::instr(&[0x00, 0x22, 0, 0, 0])).unwrap());
    }

    #[test]
    fn or_do_or_quirk_shifts_original_orlist() {
        // Upstream OrPattern::doOr with sa<0 shifts the RECEIVER's own
        // orlist (through const) and keeps the unshifted clones in the
        // result.  Document that exact behavior.
        let mut a = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(ip_byte(0, 0x11)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        ));
        let mut b = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x33)));
        let res = a.do_or(&mut b, -1);
        // result holds the UNSHIFTED clones plus b's clone
        assert_eq!(res.num_disjoint(), 3);
        assert!(res.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0, 0])).unwrap());
        assert!(res.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0, 0])).unwrap());
        // the ORIGINAL OrPattern was mutated: its byte-0 requirements moved
        // to byte 1
        assert!(a.is_match(&ByteWalker::instr(&[0x00, 0x11, 0, 0, 0])).unwrap());
        assert!(!a.is_match(&ByteWalker::instr(&[0x11, 0x00, 0, 0, 0])).unwrap());
    }

    #[test]
    fn common_sub_pattern_of_instructions() {
        let a = Pattern::Disjoint(DisjointPattern::Instruction(InstructionPattern::new(
            PatternBlock::new(0, 0xffff_0000, 0xab12_0000),
        )));
        let b = Pattern::Disjoint(DisjointPattern::Instruction(InstructionPattern::new(
            PatternBlock::new(0, 0xffff_0000, 0xab34_0000),
        )));
        let res = a.common_sub_pattern(&b, 0);
        match &res {
            Pattern::Disjoint(DisjointPattern::Instruction(ip)) => {
                assert_eq!(ip.get_block().get_mask(0, 16), 0xffd9);
                assert_eq!(ip.get_block().get_value(0, 16), 0xab10);
            }
            other => panic!("expected InstructionPattern, got {other:?}"),
        }
        // instruction vs context: alwaysTrue InstructionPattern
        let cp = Pattern::Disjoint(DisjointPattern::Context(ContextPattern::new(
            PatternBlock::new(0, 0xff00_0000, 0x1200_0000),
        )));
        let res2 = a.common_sub_pattern(&cp, 0);
        assert!(res2.always_true());
    }

    #[test]
    fn simplify_clone_collapses() {
        // Or with an alwaysTrue branch -> InstructionPattern(true)
        let or = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(InstructionPattern::new_always(true)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        ));
        assert!(or.simplify_clone().always_true());
        // Or with one alwaysFalse branch -> the surviving disjoint alone
        let or2 = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(InstructionPattern::new_always(false)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        ));
        let simp = or2.simplify_clone();
        assert!(matches!(
            simp,
            Pattern::Disjoint(DisjointPattern::Instruction(_))
        ));
        // Or with all alwaysFalse branches -> InstructionPattern(false)
        let or3 = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(InstructionPattern::new_always(false)),
            DisjointPattern::Instruction(InstructionPattern::new_always(false)),
        ));
        assert!(or3.simplify_clone().always_false());
        // Combine with alwaysTrue context collapses to its instruction piece
        let comb = CombinePattern::new(
            ContextPattern::new(PatternBlock::new_always(true)),
            ip_byte(0, 0xab),
        );
        assert!(matches!(
            comb.simplify_clone(),
            Pattern::Disjoint(DisjointPattern::Instruction(_))
        ));
        // Combine with alwaysFalse instruction collapses to false
        let comb2 = CombinePattern::new(
            ContextPattern::new(PatternBlock::new(0, 0xf000_0000, 0x3000_0000)),
            InstructionPattern::new_always(false),
        );
        assert!(comb2.simplify_clone().always_false());
    }

    #[test]
    fn disjoint_specializes_identical_resolves_intersect() {
        let fine = DisjointPattern::Instruction(InstructionPattern::new(PatternBlock::new(
            0,
            0xffff_0000,
            0xab12_0000,
        )));
        let coarse = DisjointPattern::Instruction(ip_byte(0, 0xab));
        assert!(fine.specializes(&coarse));
        assert!(!coarse.specializes(&fine));
        assert!(fine.identical(&fine.clone()));
        assert!(!fine.identical(&coarse));

        // a Combine specializes its own instruction piece
        let ctx = DisjointPattern::Context(ContextPattern::new(PatternBlock::new(
            0,
            0xf000_0000,
            0x3000_0000,
        )));
        let comb = match Pattern::Disjoint(coarse.clone())
            .do_and(&Pattern::Disjoint(ctx.clone()), 0)
        {
            Pattern::Disjoint(d) => d,
            other => panic!("expected disjoint, got {other:?}"),
        };
        assert!(comb.specializes(&coarse));
        assert!(comb.specializes(&ctx));
        // and resolves the intersection of its two pieces
        assert!(comb.resolves_intersect(&coarse, &ctx));
        assert!(!coarse.resolves_intersect(&coarse, &ctx));
        // an instruction pattern resolves the intersection of itself with
        // an (absent) context side
        assert!(coarse.resolves_intersect(&coarse, &coarse));
    }

    #[test]
    fn get_disjoint_accessors() {
        let or = Pattern::Or(OrPattern::new(
            DisjointPattern::Instruction(ip_byte(0, 0x11)),
            DisjointPattern::Instruction(ip_byte(0, 0x22)),
        ));
        assert!(or.get_disjoint(0).is_some());
        assert!(or.get_disjoint(1).is_some());
        let single = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x11)));
        assert!(single.get_disjoint(0).is_none());
        assert_eq!(single.num_disjoint(), 0);
    }

    // -- encode/decode round trips ---------------------------------------------------

    fn decode_disjoint_bytes(buf: &[u8]) -> DisjointPattern {
        let manager = test_manager();
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(buf).unwrap();
        DisjointPattern::decode_disjoint(&mut dec).unwrap()
    }

    fn encode_pattern(d: &DisjointPattern) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            d.encode(&mut enc);
        }
        buf
    }

    #[test]
    fn decode_disjoint_roundtrips_all_types() {
        // instruction
        let ip = DisjointPattern::Instruction(ip_byte(1, 0x7f));
        let buf = encode_pattern(&ip);
        let rt = decode_disjoint_bytes(&buf);
        assert!(matches!(rt, DisjointPattern::Instruction(_)));
        assert!(rt.identical(&ip));
        assert_eq!(encode_pattern(&rt), buf);
        // context
        let cp = DisjointPattern::Context(ContextPattern::new(PatternBlock::new(
            0,
            0x0ff0_0000,
            0x0120_0000,
        )));
        let buf = encode_pattern(&cp);
        let rt = decode_disjoint_bytes(&buf);
        assert!(matches!(rt, DisjointPattern::Context(_)));
        assert!(rt.identical(&cp));
        assert_eq!(encode_pattern(&rt), buf);
        // combine (the factory's fall-through branch)
        let comb = DisjointPattern::Combine(CombinePattern::new(
            ContextPattern::new(PatternBlock::new(0, 0xf000_0000, 0x3000_0000)),
            ip_byte(0, 0xab),
        ));
        let buf = encode_pattern(&comb);
        let rt = decode_disjoint_bytes(&buf);
        assert!(matches!(rt, DisjointPattern::Combine(_)));
        assert!(rt.identical(&comb));
        assert_eq!(encode_pattern(&rt), buf);
    }

    #[test]
    fn or_pattern_decode_roundtrip() {
        let manager = test_manager();
        let or = OrPattern::new(
            DisjointPattern::Instruction(ip_byte(0, 0x11)),
            DisjointPattern::Combine(CombinePattern::new(
                ContextPattern::new(PatternBlock::new(0, 0xf000_0000, 0x3000_0000)),
                ip_byte(0, 0x22),
            )),
        );
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            or.encode(&mut enc);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let rt = OrPattern::decode(&mut dec).unwrap();
        assert_eq!(rt.num_disjoint(), 2);
        assert!(rt.get_disjoint(0).unwrap().identical(or.get_disjoint(0).unwrap()));
        assert!(rt.get_disjoint(1).unwrap().identical(or.get_disjoint(1).unwrap()));
        let mut buf2 = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf2);
            rt.encode(&mut enc);
        }
        assert_eq!(buf2, buf);
    }
}
