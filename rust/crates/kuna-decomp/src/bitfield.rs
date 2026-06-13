//! Port of `decompiler/cpp/bitfield.cc` / `bitfield.hh` (W6, item w6-s5-bitfield):
//! the bitfield placement/recovery machinery.
//!
//! The bitfield subsystem transforms expressions that read/write individual
//! bit-fields of a packed structure into explicit `INSERT` / `ZPULL` / `SPULL`
//! p-code ops, and back.  Everything in the subsystem is built on top of one
//! self-contained endian-aware value type, [`BitRange`], which the C++ keeps in
//! `address.hh`/`address.cc` (not in `bitfield.cc`).  Because the Rust port
//! routes `BitRange` through this file (see `dtype.rs`'s `TypeBitField` note:
//! "`BitRange` as a standalone type is still W6 elsewhere"), the value type is
//! transcribed here verbatim from `address.cc:630-868`.  It is the load-bearing
//! foundation of the whole subsystem and carries the "bitfield extraction/
//! placement matrix" behavior that the item's tests pin.
//!
//! [`BitFieldNodeState`] (`bitfield.hh:27-42`, `bitfield.cc:21-51`) is the
//! per-Varnode state carrier the two transform passes follow.  Its construction
//! arithmetic over `BitRange` is transcribed faithfully; the live `Varnode *node`
//! / `const TypeBitField *field` pointers become opaque handles (a
//! [`VarnodeId`] and an index/flattened-field carrier) so the pure bit-range
//! computation can be ported and unit-tested without the not-yet-ported pieces
//! of the IR mutation engine.
//!
//! # Cross-wave seam (recorded as a loss)
//!
//! The two transform classes — `BitFieldInsertTransform` and
//! `BitFieldPullTransform` — and the six `Rule` `applyOp` bodies that drive them
//! reach deep into IR-mutation surfaces and type-subsystem methods that are not
//! yet ported:
//!
//! * `TypeStruct::collectBitFields` / `TypePartialStruct::getParent` /
//!   `Datatype::getPtrInto` on a `TypePointerRel` — the structure-walk that
//!   discovers which fields a Varnode overlaps (`dtype.rs` ports `getPtrInto`
//!   only for the non-relative case; `collectBitFields` is unported).  // SEAM(W6)
//! * The `INSERT` / `ZPULL` / `SPULL` `OpBehavior`s and the new-op creation /
//!   back-and-forward graph traversal the transforms emit (`Funcdata` has the
//!   op/varnode factory, but the bitfield emission sequence, `foldLoad`,
//!   `foldPtrsub`, `opDestroyRecursive` scheduling, and the `INT_EQUAL`-group
//!   compare folding are unported).  // SEAM(W6)
//!
//! Each rule below transcribes the guards it *can* evaluate with the ported type
//! surface (`hasBitfields`, `getTypeReadFacing`/`getTypeDefFacing`, the
//! `isWritten`/`code()` and `notPrinted` checks) and then returns `0` ("rule did
//! not apply" — the conservative value the C++ also returns on every early-out)
//! at the precise point where it would hand off to a transform.  The rules expose
//! themselves through [`specs`] in C++ definition order, per the `action.rs`
//! convention, so the W8 `universalAction` builder can place them unchanged once
//! the transforms land.

use kuna_base::address::{leastsigbit_set, mostsigbit_set};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::VarnodeId;

// =============================================================================
// BitRange (address.hh:256-285, address.cc:630-868)
// =============================================================================

/// An endian-aware range of bits contained in a contiguous set of bytes
/// (C++ `BitRange`, address.hh:257).
///
/// The four geometry fields are all signed `int4` exactly as in C++ — the
/// "undefined" sentinel is `-1` and several methods compute differences that can
/// legitimately go negative before being clamped, so the signedness is
/// load-bearing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitRange {
    /// Byte offset of the region containing the range (C++ `byteOffset`).
    pub byte_offset: int4,
    /// Size of the region in bytes (C++ `byteSize`).
    pub byte_size: int4,
    /// Least significant bit of the bit-range within its region (C++
    /// `leastSigBit`).
    pub least_sig_bit: int4,
    /// Number of bits in the range (C++ `numBits`).
    pub num_bits: int4,
    /// Is the underlying encoding big endian (C++ `isBigEndian`).
    pub is_big_endian: bool,
}

impl BitRange {
    /// Construct an \e undefined range (C++ `BitRange(void)`, address.hh:264).
    ///
    /// All geometry fields are the `-1` sentinel; `isBigEndian` is `false`.
    pub fn undefined() -> BitRange {
        BitRange {
            byte_offset: -1,
            byte_size: -1,
            least_sig_bit: -1,
            num_bits: -1,
            is_big_endian: false,
        }
    }

    /// Construct a byte range (C++ `BitRange(int4,int4,bool)`, address.hh:265-266):
    /// `leastSigBit = 0`, `numBits = bSize * 8`.
    pub fn byte_range(b_off: int4, b_size: int4, big_endian: bool) -> BitRange {
        BitRange {
            byte_offset: b_off,
            byte_size: b_size,
            least_sig_bit: 0,
            num_bits: b_size * 8,
            is_big_endian: big_endian,
        }
    }

    /// Construct fully (C++ `BitRange(int4,int4,int4,int4,bool)`,
    /// address.hh:268-269).
    pub fn new(
        b_off: int4,
        b_size: int4,
        least: int4,
        num: int4,
        big_endian: bool,
    ) -> BitRange {
        BitRange {
            byte_offset: b_off,
            byte_size: b_size,
            least_sig_bit: least,
            num_bits: num,
            is_big_endian: big_endian,
        }
    }

    /// Copy a range into a new container (C++ `BitRange(const BitRange&,int4,int4)`,
    /// address.cc:630-638).  The new container is `(off, sz)`; `numBits`/
    /// `isBigEndian` are copied and `leastSigBit` is the source range's LSB
    /// translated into the new frame.
    pub fn in_container(op2: &BitRange, off: int4, sz: int4) -> BitRange {
        let mut res = BitRange {
            byte_offset: off,
            byte_size: sz,
            num_bits: op2.num_bits,
            is_big_endian: op2.is_big_endian,
            least_sig_bit: 0, // overwritten by translate_lsb below
        };
        res.least_sig_bit = res.translate_lsb(op2);
        res
    }

    /// Return `true` if `this` is an empty bit range (C++ `empty`,
    /// address.hh:270).
    pub fn empty(&self) -> bool {
        self.num_bits <= 0
    }

    /// Compare `this` with another as containers (C++ `compare`,
    /// address.cc:643-655): byteOffset, byteSize, leastSigBit, numBits in order,
    /// each a signed tie-break, all equal yielding `0`.
    pub fn compare(&self, op2: &BitRange) -> int4 {
        if self.byte_offset != op2.byte_offset {
            return if self.byte_offset < op2.byte_offset { -1 } else { 1 };
        }
        if self.byte_size != op2.byte_size {
            return if self.byte_size < op2.byte_size { -1 } else { 1 };
        }
        if self.least_sig_bit != op2.least_sig_bit {
            return if self.least_sig_bit < op2.least_sig_bit {
                -1
            } else {
                1
            };
        }
        if self.num_bits != op2.num_bits {
            return if self.num_bits < op2.num_bits { -1 } else { 1 };
        }
        0
    }

    /// Translate `op2.leastSigBit` into `this`'s reference frame (C++
    /// `translateLSB`, address.cc:660-673).  The result is directly comparable
    /// with `this.leastSigBit`.
    pub fn translate_lsb(&self, op2: &BitRange) -> int4 {
        let mut op2_sig = op2.least_sig_bit;
        if self.is_big_endian {
            let this_pos = self.byte_offset + self.byte_size;
            let op2_pos = op2.byte_offset + op2.byte_size;
            op2_sig += 8 * (this_pos - op2_pos);
        } else {
            op2_sig += 8 * (op2.byte_offset - self.byte_offset);
        }
        op2_sig
    }

    /// Characterize the overlap between `this` and another range (C++
    /// `overlapTest`, address.cc:685-704).  Returns:
    /// `-1` (this before, no intersection), `0` (identical), `1` (this after,
    /// no intersection), `2` (this contained in op2), `3` (op2 contained in
    /// this), `4` (partial overlap).
    pub fn overlap_test(&self, op2: &BitRange) -> int4 {
        let op2_sig = self.translate_lsb(op2);
        let this_most = self.least_sig_bit + self.num_bits;
        let op2_most = op2_sig + op2.num_bits;
        if self.is_big_endian {
            if self.least_sig_bit >= op2_most {
                return -1;
            }
            if op2_sig >= this_most {
                return 1;
            }
        } else {
            if this_most <= op2_sig {
                return -1;
            }
            if op2_most <= self.least_sig_bit {
                return 1;
            }
        }
        // Reaching here we have some kind of intersection
        if self.least_sig_bit == op2_sig && this_most == op2_most {
            return 0;
        }
        if op2_sig <= self.least_sig_bit && op2_most >= this_most {
            return 2; // this contained in op2
        }
        if self.least_sig_bit <= op2_sig && this_most >= op2_most {
            return 3; // op2 contained in this
        }
        4
    }

    /// Replace `this` with the intersection of `this` with another (C++
    /// `intersection`, address.cc:709-726).  Only `leastSigBit`/`numBits` change;
    /// an empty intersection sets `numBits = 0`.
    pub fn intersection(&mut self, op2: &BitRange) {
        let op2_sig = self.translate_lsb(op2);
        let op2_most = op2_sig + op2.num_bits;
        let this_most = self.least_sig_bit + self.num_bits;
        if op2_sig > self.least_sig_bit {
            self.num_bits -= op2_sig - self.least_sig_bit;
            self.least_sig_bit = op2_sig;
        }
        if op2_most < this_most {
            self.num_bits -= this_most - op2_most;
        }
        if self.num_bits < 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
        }
    }

    /// Restrict `this` with a mask aligned to the container (C++ `intersectMask`,
    /// address.cc:731-750).  The result is the minimal cover of the 1-bits in the
    /// intersection.
    pub fn intersect_mask(&mut self, mut mask: uintb) {
        mask &= self.get_mask();
        if mask == 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
            return;
        }
        let new_least_sig = leastsigbit_set(mask);
        let new_most_sig = mostsigbit_set(mask) + 1;
        let this_most = self.least_sig_bit + self.num_bits;
        if new_least_sig > self.least_sig_bit {
            self.num_bits -= new_least_sig - self.least_sig_bit;
            self.least_sig_bit = new_least_sig;
        }
        if new_most_sig < this_most {
            self.num_bits -= this_most - new_most_sig;
        }
    }

    /// Shift the bit range left by the given amount (C++ `shift`,
    /// address.cc:754-770).
    pub fn shift(&mut self, left_shift_amount: int4) {
        self.least_sig_bit += left_shift_amount;
        let most = self.least_sig_bit + self.num_bits;
        if self.least_sig_bit < 0 {
            self.num_bits += self.least_sig_bit;
            self.least_sig_bit = 0;
        } else if most > self.byte_size * 8 {
            self.num_bits -= most - self.byte_size * 8;
        }
        if self.num_bits < 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
        }
    }

    /// Truncate the most significant bytes in the container (C++
    /// `truncateMostSigBytes`, address.cc:774-786).  `numBits` may be affected.
    pub fn truncate_most_sig_bytes(&mut self, num: int4) {
        if self.is_big_endian {
            self.byte_offset += num;
        }
        self.byte_size -= num;
        let max_offset = self.least_sig_bit + self.num_bits;
        if max_offset > self.byte_size * 8 {
            self.num_bits -= max_offset - self.byte_size * 8;
        }
        if self.num_bits < 0 {
            self.num_bits = 0;
        }
    }

    /// Truncate the least significant bytes in the container (C++
    /// `truncateLeastSigBytes`, address.cc:789-802).
    pub fn truncate_least_sig_bytes(&mut self, num: int4) {
        if !self.is_big_endian {
            self.byte_offset += num;
        }
        self.byte_size -= num;
        self.least_sig_bit -= num * 8;
        if self.least_sig_bit < 0 {
            self.num_bits += self.least_sig_bit;
            self.least_sig_bit = 0;
            if self.num_bits < 0 {
                self.num_bits = 0;
            }
        }
    }

    /// Add most significant bytes to the container (C++ `extendBytes`,
    /// address.cc:806-812).  Only the container changes.
    pub fn extend_bytes(&mut self, num: int4) {
        if self.is_big_endian {
            self.byte_offset -= num;
        }
        self.byte_size += num;
    }

    /// Get the bit-mask describing `this` range, aligned with the byte container
    /// (C++ `getMask`, address.cc:816-829).
    pub fn get_mask(&self) -> uintb {
        let mut res: uintb;
        // C++: numBits >= sizeof(uintb)*8  (uintb is uint8 => 64 bits)
        if self.num_bits >= (std::mem::size_of::<uintb>() as int4) * 8 {
            res = 0;
        } else {
            res = 1;
            res <<= self.num_bits;
        }
        res = res.wrapping_sub(1);
        res <<= self.least_sig_bit;
        res
    }

    /// Return `true` if the range begins and ends on byte boundaries (C++
    /// `isByteRange`, address.cc:832-838).
    pub fn is_byte_range(&self) -> bool {
        if (self.num_bits & 7) != 0 {
            return false;
        }
        if (self.least_sig_bit & 7) != 0 {
            return false;
        }
        true
    }

    /// Return `true` if the range occupies the most significant bits of the
    /// container (C++ `isMostSignificant`, address.cc:841-845).
    pub fn is_most_significant(&self) -> bool {
        8 * self.byte_size == self.least_sig_bit + self.num_bits
    }

    /// Shrink the container to fit the bit range (C++ `minimizeContainer`,
    /// address.cc:847-862).
    pub fn minimize_container(&mut self) {
        let trunc = self.least_sig_bit / 8;
        if self.is_big_endian {
            self.byte_size -= trunc;
        } else {
            self.byte_offset += trunc;
        }
        self.least_sig_bit &= 7;
        let num = self.byte_size - ((self.least_sig_bit + self.num_bits + 7) / 8);
        if num > 0 {
            if self.is_big_endian {
                self.byte_offset += num;
            }
            self.byte_size -= num;
        }
    }

    /// Expand the range until it includes the most significant bits of the
    /// container (C++ `expandToMost`, address.cc:864-868).
    pub fn expand_to_most(&mut self) {
        // Increase number of bits to maximum that still fits
        self.num_bits = 8 * self.byte_size - self.least_sig_bit;
    }
}

// =============================================================================
// BitFieldNodeState (bitfield.hh:27-42, bitfield.cc:21-51)
// =============================================================================

/// A reference to the bit-field being followed by a [`BitFieldNodeState`].
///
/// In C++ this is `const TypeBitField *field` — null for a "hole" record.  The
/// pure bit-range arithmetic the transforms run needs only the field's
/// `bits`-derived range (already captured in [`BitFieldNodeState::bits_field`])
/// plus the field's signedness; this enum carries the latter without holding a
/// live type pointer.  `Field { is_int }` records whether the field's underlying
/// data-type has metatype `TYPE_INT` (the only datum
/// [`BitFieldNodeState::does_sign_extension_match`] reads off `field->type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldRef {
    /// A hole record (C++ `field == (const TypeBitField *)0`).
    Hole,
    /// A real bit-field; `is_int` is `field->type->getMetatype() == TYPE_INT`.
    Field {
        /// `true` iff the field's underlying type has metatype `TYPE_INT`.
        is_int: bool,
    },
}

/// Description of the bit-fields covered by a Varnode (C++ `BitFieldNodeState`,
/// bitfield.hh:27-42).
///
/// The live `Varnode *node` becomes a [`VarnodeId`] handle and `const
/// TypeBitField *field` becomes a [`FieldRef`]; everything else is the same
/// geometry the transforms manipulate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitFieldNodeState {
    /// Bits being used from this Varnode (C++ `bitsUsed`).
    pub bits_used: BitRange,
    /// Bits from the bit-field being followed (C++ `bitsField`).
    pub bits_field: BitRange,
    /// Varnode holding bitfields (C++ `node`).
    pub node: VarnodeId,
    /// Bit-field being followed (C++ `field`).
    pub field: FieldRef,
    /// Original position of least significant bit (C++ `origLeastSigBit`).
    pub orig_least_sig_bit: int4,
    /// Bitfield has been sign-extended into node (C++ `isSignExtended`).
    pub is_sign_extended: bool,
}

impl BitFieldNodeState {
    /// Constructor to follow a field (C++ `BitFieldNodeState(const BitRange&,
    /// Varnode*,const TypeBitField*)`, bitfield.cc:21-28).
    ///
    /// The C++ initializer is
    /// `bitsField(fld->bits,used.byteOffset,used.byteSize)` — the field's range
    /// copied into the Varnode's byte container — and
    /// `isSignExtended = (field->type->getMetatype() == TYPE_INT) &&
    /// bitsField.isMostSignificant()`.  `field_bits` is `fld->bits`; `field_is_int`
    /// is `fld->type->getMetatype() == TYPE_INT`.
    pub fn follow_field(
        used: &BitRange,
        vn: VarnodeId,
        field_bits: &BitRange,
        field_is_int: bool,
    ) -> BitFieldNodeState {
        let bits_field = BitRange::in_container(field_bits, used.byte_offset, used.byte_size);
        let orig_least_sig_bit = bits_field.least_sig_bit;
        let is_sign_extended = field_is_int && bits_field.is_most_significant();
        BitFieldNodeState {
            bits_used: *used,
            bits_field,
            node: vn,
            field: FieldRef::Field { is_int: field_is_int },
            orig_least_sig_bit,
            is_sign_extended,
        }
    }

    /// Constructor for a hole (C++ `BitFieldNodeState(const BitRange&,Varnode*,
    /// int4,int4)`, bitfield.cc:30-37).
    ///
    /// The C++ initializer is
    /// `bitsField(used.byteOffset,used.byteSize,leastSig,numBits,used.isBigEndian)`;
    /// `field` is null and `isSignExtended` is `false`.
    pub fn hole(used: &BitRange, vn: VarnodeId, least_sig: int4, num_bits: int4) -> BitFieldNodeState {
        let bits_field = BitRange::new(
            used.byte_offset,
            used.byte_size,
            least_sig,
            num_bits,
            used.is_big_endian,
        );
        let orig_least_sig_bit = bits_field.least_sig_bit;
        BitFieldNodeState {
            bits_used: *used,
            bits_field,
            node: vn,
            field: FieldRef::Hole,
            orig_least_sig_bit,
            is_sign_extended: false,
        }
    }

    /// Copy another state, but replace `bitsField` (C++ copy constructor
    /// `BitFieldNodeState(const BitFieldNodeState&,const BitRange&,Varnode*,bool)`,
    /// bitfield.cc:44-51).  `field`/`origLeastSigBit` are copied; `bitsField`,
    /// `node`, and `isSignExtended` are replaced.
    pub fn with_new_field(
        copy: &BitFieldNodeState,
        new_field: &BitRange,
        vn: VarnodeId,
        sgn_ext: bool,
    ) -> BitFieldNodeState {
        BitFieldNodeState {
            bits_used: copy.bits_used,
            bits_field: *new_field,
            node: vn,
            field: copy.field,
            orig_least_sig_bit: copy.orig_least_sig_bit,
            is_sign_extended: sgn_ext,
        }
    }

    /// Can the current Varnode be treated as the isolated bitfield (C++
    /// `isFieldAligned`, bitfield.hh:39).
    pub fn is_field_aligned(&self) -> bool {
        self.bits_field.least_sig_bit == 0 && self.bits_field.num_bits == self.bits_used.num_bits
    }

    /// Return `true` if the signedness of the field matches the extension used to
    /// extract it (C++ `doesSignExtensionMatch`, bitfield.hh:41).  In C++ this is
    /// `isSignExtended == (field->type->getMetatype() == TYPE_INT)`; the
    /// metatype-`TYPE_INT` datum is carried in [`FieldRef::Field::is_int`].  A
    /// hole record has no field; the C++ never calls this on a hole, so it
    /// returns `false` defensively.
    pub fn does_sign_extension_match(&self) -> bool {
        match self.field {
            FieldRef::Field { is_int } => self.is_sign_extended == is_int,
            FieldRef::Hole => false,
        }
    }
}

// =============================================================================
// allowedFinalWrites (bitfield.cc:118-125)
// =============================================================================

/// List of opcodes allowed for `finalWriteOp` (C++
/// `BitFieldInsertTransform::allowedFinalWrites`, bitfield.cc:118-125).  "Must be
/// sorted" per the header comment; transcribed in source order (which is the
/// numeric order of the `OpCode` enum here).  [`RuleBitFieldOut::get_op_list`]
/// folds this list into its op whitelist.
const ALLOWED_FINAL_WRITES: &[OpCode] = &[
    OpCode::CPUI_COPY,
    OpCode::CPUI_INT_EQUAL,
    OpCode::CPUI_INT_NOTEQUAL,
    OpCode::CPUI_INT_SLESS,
    OpCode::CPUI_INT_SLESSEQUAL,
    OpCode::CPUI_INT_LESS,
    OpCode::CPUI_INT_LESSEQUAL,
    OpCode::CPUI_INT_ZEXT,
    OpCode::CPUI_INT_SEXT,
    OpCode::CPUI_INT_ADD,
    OpCode::CPUI_INT_CARRY,
    OpCode::CPUI_INT_SCARRY,
    OpCode::CPUI_INT_XOR,
    OpCode::CPUI_INT_AND,
    OpCode::CPUI_INT_OR,
    OpCode::CPUI_INT_LEFT,
    OpCode::CPUI_INT_RIGHT,
    OpCode::CPUI_INT_SRIGHT,
    OpCode::CPUI_INT_MULT,
    OpCode::CPUI_BOOL_NEGATE,
    OpCode::CPUI_BOOL_XOR,
    OpCode::CPUI_BOOL_AND,
    OpCode::CPUI_BOOL_OR,
    OpCode::CPUI_FLOAT_EQUAL,
    OpCode::CPUI_FLOAT_NOTEQUAL,
    OpCode::CPUI_FLOAT_LESS,
    OpCode::CPUI_FLOAT_LESSEQUAL,
    OpCode::CPUI_FLOAT_NAN,
    OpCode::CPUI_SUBPIECE,
];

// =============================================================================
// Rules (bitfield.hh:180-265, bitfield.cc:1671-2400)
// =============================================================================

/// Collapse bitfield insertion ending in a `CPUI_STORE` (C++ `RuleBitFieldStore`,
/// bitfield.hh:181-190, bitfield.cc:1671-1692).
pub struct RuleBitFieldStore;

impl Rule for RuleBitFieldStore {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:1671-1675): just CPUI_STORE.
        vec![OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // Group membership lives in the engine-owned RuleState; see ruleaction_3
        // module docs.  The clone is unconditional here.
        Some(Box::new(RuleBitFieldStore))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:1677-1692):
        //   Datatype *ptr = op->getIn(1)->getTypeReadFacing(op);
        //   Datatype *dt = ptr->getPtrInto(off);
        //   if (dt == 0) return 0;
        //   if (!dt->hasBitfields()) return 0;
        //   Varnode *vn = op->getIn(2);
        //   if (vn->isWritten() && vn->getDef()->code() == CPUI_INSERT) return 0;
        //   BitFieldInsertTransform transform(&data,op,dt,off);  ...
        //
        // SEAM(W6): getTypeReadFacing/getPtrInto-on-relative-pointer and the
        // BitFieldInsertTransform machinery are unported.  Every reachable guard
        // is preserved; the transform hand-off cannot run, so the rule
        // conservatively does not apply.  Recorded as a loss.
        let _ = (op, data);
        0
    }
}

/// Collapse bitfield insertion ending in a write to a mapped Varnode (C++
/// `RuleBitFieldOut`, bitfield.hh:193-202, bitfield.cc:1694-1712).
pub struct RuleBitFieldOut;

impl Rule for RuleBitFieldOut {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:1694-1699): allowedFinalWrites followed by
        // CPUI_INDIRECT (insertion order preserved).
        let mut oplist: Vec<OpCode> = ALLOWED_FINAL_WRITES.to_vec();
        oplist.push(OpCode::CPUI_INDIRECT);
        oplist
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(RuleBitFieldOut))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:1701-1712):
        //   Datatype *dt = op->getOut()->getTypeDefFacing();
        //   if (!dt->hasBitfields()) return 0;
        //   BitFieldInsertTransform transform(&data,op,dt,0);  ...
        //
        // SEAM(W6): getTypeDefFacing + BitFieldInsertTransform are unported.
        let _ = (op, data);
        0
    }
}

/// Collapse bitfield pulls starting with a `CPUI_LOAD` (C++ `RuleBitFieldLoad`,
/// bitfield.hh:205-214, bitfield.cc:1714-1734).
pub struct RuleBitFieldLoad;

impl Rule for RuleBitFieldLoad {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:1714-1718): just CPUI_LOAD.
        vec![OpCode::CPUI_LOAD]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(RuleBitFieldLoad))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:1720-1734):
        //   Datatype *ptr = op->getIn(1)->getTypeReadFacing(op);
        //   Datatype *dt = ptr->getPtrInto(off);
        //   if (dt == 0) return 0;
        //   if (!dt->hasBitfields()) return 0;
        //   if (op->notPrinted()) return 0;     // LOAD visited before
        //   BitFieldPullTransform transform(&data,op->getOut(),dt,off);  ...
        //
        // SEAM(W6): getTypeReadFacing/getPtrInto + BitFieldPullTransform unported.
        let _ = (op, data);
        0
    }
}

/// Collapse bitfield pulls starting with mapped Varnodes (C++ `RuleBitFieldIn`,
/// bitfield.hh:217-226, bitfield.cc:1736-1759).
pub struct RuleBitFieldIn;

impl Rule for RuleBitFieldIn {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:1736-1746): the explicit 17-entry list, in
        // exactly that order.
        vec![
            OpCode::CPUI_COPY,
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_INT_LESSEQUAL,
            OpCode::CPUI_INT_ZEXT,
            OpCode::CPUI_INT_SEXT,
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_INT_NEGATE,
            OpCode::CPUI_INT_AND,
            OpCode::CPUI_INT_LEFT,
            OpCode::CPUI_INT_RIGHT,
            OpCode::CPUI_INT_SRIGHT,
            OpCode::CPUI_INT_MULT,
            OpCode::CPUI_SUBPIECE,
        ]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(RuleBitFieldIn))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:1748-1759):
        //   Varnode *invn = op->getIn(0);
        //   Datatype *dt = invn->getTypeReadFacing(op);
        //   if (!dt->hasBitfields()) return 0;
        //   BitFieldPullTransform transform(&data,invn,dt,0);  ...
        //
        // SEAM(W6): getTypeReadFacing + BitFieldPullTransform unported.
        let _ = (op, data);
        0
    }
}

/// Simplify expressions explicitly using `ZPULL` and `SPULL` p-code ops (C++
/// `RulePullAbsorb`, bitfield.hh:229-248, bitfield.cc:2161-2212).
pub struct RulePullAbsorb;

impl Rule for RulePullAbsorb {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:2161-2166): CPUI_ZPULL, CPUI_SPULL.
        vec![OpCode::CPUI_ZPULL, OpCode::CPUI_SPULL]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(RulePullAbsorb))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:2168-2212): walks the descendants of the
        // ZPULL/SPULL output and dispatches into the absorb* helpers, all of
        // which emit/destroy ops.
        //
        // SEAM(W6): the absorb* graph rewrites are unported.
        let _ = (op, data);
        0
    }
}

/// Simplify expressions explicitly using the `INSERT` p-code op (C++
/// `RuleInsertAbsorb`, bitfield.hh:251-265, bitfield.cc:2357-2400).
pub struct RuleInsertAbsorb;

impl Rule for RuleInsertAbsorb {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ getOpList (bitfield.cc:2357-2361): just CPUI_INSERT.
        vec![OpCode::CPUI_INSERT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(RuleInsertAbsorb))
    }

    fn apply_op(&mut self, op: crate::seams::OpId, data: &mut Funcdata) -> int4 {
        // C++ applyOp (bitfield.cc:2363-2400): switches on the def opcode of the
        // INSERT value input and dispatches into the absorb* helpers.
        //
        // SEAM(W6): the absorb* graph rewrites are unported.
        let _ = (op, data);
        0
    }
}

/// Expose the bitfield rules in C++ definition order (`bitfield.hh` declaration
/// order: `bitfield_store`, `bitfield_out`, `bitfield_load`, `bitfield_in`,
/// `pull_absorb`, `insert_absorb`), per the `action.rs` `specs()` convention.
/// All six carry group `0` in C++; the W8 `universalAction` builder assigns the
/// real stage group when it places them.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "", ctor: || Box::new(RuleBitFieldStore) },
        RuleSpec { group: "", ctor: || Box::new(RuleBitFieldOut) },
        RuleSpec { group: "", ctor: || Box::new(RuleBitFieldLoad) },
        RuleSpec { group: "", ctor: || Box::new(RuleBitFieldIn) },
        RuleSpec { group: "", ctor: || Box::new(RulePullAbsorb) },
        RuleSpec { group: "", ctor: || Box::new(RuleInsertAbsorb) },
    ]
}

#[cfg(test)]
mod tests;
