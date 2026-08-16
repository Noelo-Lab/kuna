//! Port of `decompiler/cpp/rangeutil.{cc,hh}` (W6, item `w6-s5-rangeutil`):
//! the circular-interval value-set domain used by jump-table / value-set
//! analysis.
//!
//! # Scope and faithfulness
//!
//! [`CircleRange`] — the circular range `[left, right)` over the integers mod
//! `2^n` with an explicit `step` — is ported **completely and faithfully**.
//! Every operator (`normalize`, `complement`, `convert_to_boolean`, the
//! `new_stride`/`new_domain` helpers, `intersect`, `circle_union`,
//! `minimal_container`, `set_nz_mask`, `set_stride`, the per-opcode
//! `pull_back_unary`/`pull_back_binary`/`push_forward_unary`/
//! `push_forward_binary`/`push_forward_trinary` matrices, `widen`, and
//! `translate2_op`) is transcribed with the same arithmetic, iteration order,
//! and tie-breakers as upstream.  This is the output-determining core that
//! `jumptable.rs` builds its guard/normalization analysis on and that
//! `testcirclerange.cc`'s 117 unit tests exercise; a representative subset of
//! those (including all the explicit-value `pullBack` matrices) is replicated
//! as in-module tests at the bottom of this file.
//!
//! Integer semantics follow ADR 0003: `uintb -> u64`, `intb -> i64`,
//! `int4 -> i32`, `uint4 -> u32`; all arithmetic that upstream lets wrap uses
//! the [`Wrap`] ops (`wadd`/`wsub`/`wmul`/`wshl`/`wshr`), and mixed-sign
//! comparisons are replicated with comments.
//!
//! # The value-set solver layer
//!
//! `ValueSet` / `ValueSetRead` / `Widener` / `WidenerFull` / `WidenerNone` /
//! `Partition` / `ValueSetSolver` are the lattice-iteration layer on top of
//! `CircleRange`.  Two distinct halves:
//!
//!   * The **widening strategy** ([`Widener`], [`WidenerFull`], [`WidenerNone`])
//!     and the `CircleRange`-only widening helpers ([`CircleRange::widen`], the
//!     `doWidening` landmark/invert logic) operate purely on `CircleRange` plus
//!     iteration counts.  These are ported faithfully and self-tested — the
//!     widening semantics are exactly upstream's.
//!
//!   * The **graph machinery** (`ValueSet::setVarnode`/`iterate`/
//!     `computeTypeCode`, the whole `ValueSetSolver` — topological ordering,
//!     constraint generation, `establishValueSets`, `solve`) is bound to the
//!     live IR: where the C++ threads a `Varnode -> ValueSet` back-pointer
//!     (`Varnode::setValueSet`) and per-object mark bits, this port keeps an
//!     explicit `VarnodeId -> node index` map on the solver (insertion and
//!     membership coincide exactly with upstream's `setValueSet` +
//!     `setMark`, which are always performed together) and side sets for the
//!     PcodeOp/FlowBlock marks, leaving the IR's own mark bits untouched.
//!     The **Bourdoncle weak-topological-ordering** walk (`visit`/`component`/
//!     `establishTopologicalOrder`) runs against the [`ValueSetGraph`] edge
//!     abstraction; the live system supplies the IR-backed
//!     `IrValueSetGraph` (descendant-op edges plus the simulated root), and
//!     the self-contained ordering tests keep exercising the walk directly.

use kuna_base::address::{
    bit_transitions, calc_mask, count_leading_zeros, leastsigbit_set, mostsigbit_set, sign_extend,
    sign_extend_sized,
};
use kuna_base::types::{int4, intb, uint4, uintb, Wrap};
use kuna_num::opcodes::OpCode;
use std::collections::{HashMap, HashSet};

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

// ===========================================================================
// CircleRange (rangeutil.hh:50, rangeutil.cc) — fully faithful
// ===========================================================================

/// Map from raw overlap codes to normalized overlap codes
/// (C++ `CircleRange::arrange`, `rangeutil.cc:21`).
///
/// The literal is exactly 64 printable chars; the 6-bit overlap code indexes
/// 0..=63 so no terminator byte is ever read.
const ARRANGE: &[u8; 64] =
    b"gcgbegdagggggggeggggcgbggggggggcdfgggggggegdggggbgggfggggcgbegda";

/// A class for manipulating integer value ranges (C++ `CircleRange`,
/// `rangeutil.hh:50`).
///
/// The representation is a circular range — the half-open interval
/// `[left, right)` over the integers mod `2^n`, where `mask = 2^n - 1`.  The
/// range can support a `step` if some of the least-significant bits of the mask
/// are set to zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleRange {
    /// Left boundary of the open range `[left, right)`.
    left: uintb,
    /// Right boundary of the open range `[left, right)`.
    right: uintb,
    /// Bit mask defining the size (modulus) and step of the range.
    mask: uintb,
    /// `true` if set is empty.
    isempty: bool,
    /// Explicit step size.
    step: int4,
}

impl Default for CircleRange {
    /// Construct an empty range (C++ `CircleRange(void) { isempty=true; }`).
    fn default() -> Self {
        CircleRange { left: 0, right: 0, mask: 0, isempty: true, step: 1 }
    }
}

impl CircleRange {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// Construct an empty range (C++ `CircleRange(void)`).
    pub fn new_empty() -> CircleRange {
        CircleRange::default()
    }

    /// Construct given specific boundaries and step
    /// (C++ `CircleRange(uintb lft,uintb rgt,int4 size,int4 stp)`,
    /// `rangeutil.cc:179`).
    pub fn new(lft: uintb, rgt: uintb, size: int4, stp: int4) -> CircleRange {
        CircleRange { mask: calc_mask(size), step: stp, left: lft, right: rgt, isempty: false }
    }

    /// Construct a boolean range (C++ `CircleRange(bool val)`, `rangeutil.cc:191`).
    pub fn new_bool(val: bool) -> CircleRange {
        // mask = 0xff; step = 1; left = val?1:0; right = val+1
        CircleRange {
            mask: 0xff,
            step: 1,
            left: if val { 1 } else { 0 },
            right: (val as uintb) + 1,
            isempty: false,
        }
    }

    /// Construct range with a single value (C++ `CircleRange(uintb val,int4 size)`,
    /// `rangeutil.cc:205`).
    pub fn new_value(val: uintb, size: int4) -> CircleRange {
        let mask = calc_mask(size);
        CircleRange { mask, step: 1, left: val, right: val.wadd(1) & mask, isempty: false }
    }

    // -----------------------------------------------------------------------
    // Setters (rangeutil.cc:219-253)
    // -----------------------------------------------------------------------

    /// Set directly to a specific range (C++ `setRange(uintb,uintb,int4,int4)`).
    pub fn set_range(&mut self, lft: uintb, rgt: uintb, size: int4, stp: int4) {
        self.mask = calc_mask(size);
        self.left = lft;
        self.right = rgt;
        self.step = stp;
        self.isempty = false;
    }

    /// Set range with a single value (C++ `setRange(uintb val,int4 size)`).
    pub fn set_range_value(&mut self, val: uintb, size: int4) {
        self.mask = calc_mask(size);
        self.step = 1;
        self.left = val;
        self.right = (val.wadd(1)) & self.mask;
        self.isempty = false;
    }

    /// Set a completely full range (C++ `setFull`).
    pub fn set_full(&mut self, size: int4) {
        self.mask = calc_mask(size);
        self.step = 1;
        self.left = 0;
        self.right = 0;
        self.isempty = false;
    }

    // -----------------------------------------------------------------------
    // Simple accessors (rangeutil.hh:71-82)
    // -----------------------------------------------------------------------

    /// Return `true` if the range is empty (C++ `isEmpty`).
    pub fn is_empty(&self) -> bool {
        self.isempty
    }

    /// Return `true` if the range contains all possible values (C++ `isFull`).
    pub fn is_full(&self) -> bool {
        (!self.isempty) && (self.step == 1) && (self.left == self.right)
    }

    /// Return `true` if the range contains a single value (C++ `isSingle`).
    pub fn is_single(&self) -> bool {
        (!self.isempty) && (self.right == (self.left.wadd(self.step as uintb)) & self.mask)
    }

    /// Get the left boundary of the range (C++ `getMin`).
    pub fn get_min(&self) -> uintb {
        self.left
    }

    /// Get the right-most integer contained in the range (C++ `getMax`).
    pub fn get_max(&self) -> uintb {
        self.right.wsub(self.step as uintb) & self.mask
    }

    /// Get the right boundary of the range (C++ `getEnd`).
    pub fn get_end(&self) -> uintb {
        self.right
    }

    /// Get the mask (C++ `getMask`).
    pub fn get_mask(&self) -> uintb {
        self.mask
    }

    /// Set the mask directly (C++ writes `mask` in `CircleRange::pullBack`'s
    /// SUBPIECE-zero special case — `mask = calc_mask(res->getSize())`).
    pub fn set_mask(&mut self, m: uintb) {
        self.mask = m;
    }

    /// Get the step for the range (C++ `getStep`).
    pub fn get_step(&self) -> int4 {
        self.step
    }

    /// Advance an integer within the range (C++ `getNext`):
    /// `val = (val+step)&mask; return val != right`.
    pub fn get_next(&self, val: &mut uintb) -> bool {
        *val = val.wadd(self.step as uintb) & self.mask;
        *val != self.right
    }

    /// Equals operator (C++ `operator==`, `rangeutil.hh:331`).
    pub fn equals(&self, op2: &CircleRange) -> bool {
        if self.isempty != op2.isempty {
            return false;
        }
        if self.isempty {
            return true;
        }
        self.left == op2.left
            && self.right == op2.right
            && self.mask == op2.mask
            && self.step == op2.step
    }

    // -----------------------------------------------------------------------
    // normalize / complement / convert_to_boolean (rangeutil.cc:25-90)
    // -----------------------------------------------------------------------

    /// Normalize the representation of full sets (C++ `normalize`,
    /// `rangeutil.cc:25`).
    fn normalize(&mut self) {
        if self.left == self.right {
            if self.step != 1 {
                self.left %= self.step as uintb;
            } else {
                self.left = 0;
            }
            self.right = self.left;
        }
    }

    /// Set `self` to the complement of itself.  Only works if `step` is 1
    /// (C++ `complement`, `rangeutil.cc:38`).
    fn complement(&mut self) {
        if self.isempty {
            self.left = 0;
            self.right = 0;
            self.isempty = false;
            return;
        }
        if self.left == self.right {
            self.isempty = true;
            return;
        }
        // C++ swaps left/right (rangeutil.cc:51-53).
        std::mem::swap(&mut self.left, &mut self.right);
    }

    /// Convert `self` to boolean (C++ `convertToBoolean`, `rangeutil.cc:63`).
    /// Returns `true` if the range contains both 0 and 1.
    fn convert_to_boolean(&mut self) -> bool {
        if self.isempty {
            return false;
        }
        let contains_zero = self.contains_val(0);
        let contains_one = self.contains_val(1);
        self.mask = 0xff;
        self.step = 1;
        if contains_zero && contains_one {
            self.left = 0;
            self.right = 2;
            self.isempty = false;
            return true;
        } else if contains_zero {
            self.left = 0;
            self.right = 1;
            self.isempty = false;
        } else if contains_one {
            self.left = 1;
            self.right = 2;
            self.isempty = false;
        } else {
            self.isempty = true;
        }
        false
    }

    // -----------------------------------------------------------------------
    // new_stride / new_domain static helpers (rangeutil.cc:103-168)
    // -----------------------------------------------------------------------

    /// Recalculate range based on new stride (C++ `newStride`, `rangeutil.cc:103`).
    /// Returns `true` if result is empty.
    fn new_stride(
        mask: uintb,
        step: int4,
        old_step: int4,
        rem: uint4,
        myleft: &mut uintb,
        myright: &mut uintb,
    ) -> bool {
        if old_step != 1 {
            // uint4 oldRem = (uint4)(myleft % oldStep)
            let old_rem = (*myleft % (old_step as uintb)) as uint4;
            if old_rem != (rem % (old_step as uint4)) {
                return true; // Step is completely off
            }
        }
        let orig_order = *myleft < *myright;
        let left_rem = (*myleft % (step as uintb)) as uint4;
        let right_rem = (*myright % (step as uintb)) as uint4;
        if left_rem > rem {
            *myleft = myleft.wadd((rem + (step as uint4) - left_rem) as uintb);
        } else {
            *myleft = myleft.wadd((rem - left_rem) as uintb);
        }

        if right_rem > rem {
            *myright = myright.wadd((rem + (step as uint4) - right_rem) as uintb);
        } else {
            *myright = myright.wadd((rem - right_rem) as uintb);
        }
        *myleft &= mask;
        *myright &= mask;

        let new_order = *myleft < *myright;
        if orig_order != new_order {
            return true;
        }

        false // not empty
    }

    /// Make a range fit in a new domain (C++ `newDomain`, `rangeutil.cc:143`).
    /// Returns `true` if the truncated domain is empty.
    fn new_domain(new_mask: uintb, new_step: int4, myleft: &mut uintb, myright: &mut uintb) -> bool {
        let rem = if new_step != 1 { *myleft % (new_step as uintb) } else { 0 };
        if *myleft > new_mask {
            if *myright > new_mask {
                // Both bounds out of range of newMask
                if *myleft < *myright {
                    return true; // Old range completely out of bounds of new mask
                }
                *myleft = rem;
                *myright = rem; // Old range contained everything in newMask
                return false;
            }
            *myleft = rem; // Take everything up to left edge of new range
        }
        if *myright > new_mask {
            *myright = rem; // Take everything up to right edge of new range
        }
        if *myleft == *myright {
            *myleft = rem; // Normalize the everything
            *myright = rem;
        }
        false // not empty
    }

    /// Calculate overlap code (C++ `encodeRangeOverlaps`, `rangeutil.hh:358`).
    fn encode_range_overlaps(
        op1left: uintb,
        op1right: uintb,
        op2left: uintb,
        op2right: uintb,
    ) -> u8 {
        let mut val: int4 = if op1left <= op1right { 0x20 } else { 0 };
        val |= if op1left <= op2left { 0x10 } else { 0 };
        val |= if op1left <= op2right { 0x8 } else { 0 };
        val |= if op1right <= op2left { 4 } else { 0 };
        val |= if op1right <= op2right { 2 } else { 0 };
        val |= if op2left <= op2right { 1 } else { 0 };
        ARRANGE[val as usize]
    }

    // -----------------------------------------------------------------------
    // getSize / getMaxInfo (rangeutil.cc:256-297)
    // -----------------------------------------------------------------------

    /// Get the number of integers contained in this range (C++ `getSize`,
    /// `rangeutil.cc:256`).
    pub fn get_size(&self) -> uintb {
        if self.isempty {
            return 0;
        }
        let val;
        if self.left < self.right {
            val = (self.right - self.left) / (self.step as uintb);
        } else {
            // C++ (rangeutil.cc:264): `(mask - (left-right) + step) / step` over
            // uintb. For a full range (left==right, 8-byte mask) `mask + step`
            // WRAPS to 0 so v==0 hits the documented overflow branch below; bare
            // `+`/`-` would panic in debug, so use the wrapping helpers (ADR 0003).
            let mut v = self
                .mask
                .wsub(self.left.wsub(self.right))
                .wadd(self.step as uintb)
                / (self.step as uintb);
            if v == 0 {
                // Overflow: all uintb values are in the range
                v = self.mask; // We lie by one, which shouldn't matter for jumptable use
                if self.step > 1 {
                    v /= self.step as uintb;
                    v += 1;
                }
            }
            val = v;
        }
        val
    }

    /// Get maximum information content of range (C++ `getMaxInfo`,
    /// `rangeutil.cc:280`).
    pub fn get_max_info(&self) -> int4 {
        let half_point = self.mask ^ (self.mask >> 1);
        if self.contains_val(half_point) {
            // 8*sizeof(uintb) - count_leading_zeros(halfPoint)
            return 8 * (std::mem::size_of::<uintb>() as int4) - count_leading_zeros(half_point);
        }
        let size_left = if (half_point & self.left) == 0 {
            count_leading_zeros(self.left)
        } else {
            count_leading_zeros(!self.left & self.mask)
        };
        let size_right = if (half_point & self.right) == 0 {
            count_leading_zeros(self.right)
        } else {
            count_leading_zeros(!self.right & self.mask)
        };
        8 * (std::mem::size_of::<uintb>() as int4)
            - (if size_right < size_left { size_right } else { size_left })
    }

    // -----------------------------------------------------------------------
    // contains (rangeutil.cc:301-352)
    // -----------------------------------------------------------------------

    /// Check containment of another range in `self` (C++
    /// `contains(const CircleRange &)`, `rangeutil.cc:301`).
    pub fn contains(&self, op2: &CircleRange) -> bool {
        if self.isempty {
            return op2.isempty;
        }
        if op2.isempty {
            return true;
        }
        if self.step > op2.step {
            // self must have a smaller or equal step to op2 or containment is
            // impossible, except where op2 is a single element.
            if !op2.is_single() {
                return false;
            }
        }
        if self.left == self.right {
            return true;
        }
        if op2.left == op2.right {
            return false;
        }
        if self.left % (self.step as uintb) != op2.left % (self.step as uintb) {
            return false; // Wrong phase
        }
        if self.left == op2.left && self.right == op2.right {
            return true;
        }

        let overlap_code = CircleRange::encode_range_overlaps(self.left, self.right, op2.left, op2.right);

        if overlap_code == b'c' {
            return true;
        }
        if overlap_code == b'b' && (self.right == op2.right) {
            return true;
        }
        false
    }

    /// Check containment of a specific integer (C++ `contains(uintb)`,
    /// `rangeutil.cc:334`).
    pub fn contains_val(&self, val: uintb) -> bool {
        if self.isempty {
            return false;
        }
        if self.step != 1 && (self.left % (self.step as uintb)) != (val % (self.step as uintb)) {
            return false; // Phase is wrong
        }
        if self.left < self.right {
            if val < self.left {
                return false;
            }
            if self.right <= val {
                return false;
            }
        } else if self.right < self.left {
            if val < self.right {
                return true;
            }
            if val >= self.left {
                return true;
            }
            return false;
        }
        true
    }

    // -----------------------------------------------------------------------
    // circleUnion (rangeutil.cc:360-444)
    // -----------------------------------------------------------------------

    /// Union two ranges (C++ `circleUnion`, `rangeutil.cc:360`).
    /// Returns 0 if the result is a single valid interval, 2 if the union is two
    /// pieces (in which case `self` is unmodified).
    pub fn circle_union(&mut self, op2: &CircleRange) -> int4 {
        if op2.isempty {
            return 0;
        }
        if self.isempty {
            *self = op2.clone();
            return 0;
        }
        if self.mask != op2.mask {
            return 2; // Cannot do proper union with different domains
        }
        let mut a_right = self.right;
        let mut b_right = op2.right;
        let mut new_step = self.step;
        if self.step < op2.step {
            if self.is_single() {
                new_step = op2.step;
                a_right = (self.left.wadd(new_step as uintb)) & self.mask;
            } else {
                return 2;
            }
        } else if op2.step < self.step {
            if op2.is_single() {
                new_step = self.step;
                b_right = (op2.left.wadd(new_step as uintb)) & self.mask;
            } else {
                return 2;
            }
        }
        let rem;
        if new_step != 1 {
            rem = self.left % (new_step as uintb);
            if rem != (op2.left % (new_step as uintb)) {
                return 2;
            }
        } else {
            rem = 0;
        }
        if (self.left == a_right) || (op2.left == b_right) {
            self.left = rem;
            self.right = rem;
            self.step = new_step;
            return 0;
        }

        let overlap_code = CircleRange::encode_range_overlaps(self.left, a_right, op2.left, b_right);
        match overlap_code {
            b'a' | b'f' => {
                // order (l r op2.l op2.r) | (op2.l op2.r l r)
                if a_right == op2.left {
                    self.right = b_right;
                    self.step = new_step;
                    return 0;
                }
                if self.left == b_right {
                    self.left = op2.left;
                    self.right = a_right;
                    self.step = new_step;
                    return 0;
                }
                2 // 2 pieces
            }
            b'b' => {
                // order (l op2.l r op2.r)
                self.right = b_right;
                self.step = new_step;
                0
            }
            b'c' => {
                // order (l op2.l op2.r r)
                self.right = a_right;
                self.step = new_step;
                0
            }
            b'd' => {
                // order (op2.l l r op2.r)
                self.left = op2.left;
                self.right = b_right;
                self.step = new_step;
                0
            }
            b'e' => {
                // order (op2.l l op2.r r)
                self.left = op2.left;
                self.right = a_right;
                self.step = new_step;
                0
            }
            b'g' => {
                // either impossible or covers whole circle
                self.left = rem;
                self.right = rem;
                self.step = new_step;
                0 // entire circle is covered
            }
            _ => -1, // Never reach here
        }
    }

    // -----------------------------------------------------------------------
    // minimalContainer (rangeutil.cc:454-529)
    // -----------------------------------------------------------------------

    /// Construct minimal range that contains both `self` and another range
    /// (C++ `minimalContainer`, `rangeutil.cc:454`).  Returns `true` if the
    /// container is everything (full).
    pub fn minimal_container(&mut self, op2: &CircleRange, max_step: int4) -> bool {
        if self.is_single() && op2.is_single() {
            let (min, max) = if self.get_min() < op2.get_min() {
                (self.get_min(), op2.get_min())
            } else {
                (op2.get_min(), self.get_min())
            };
            let diff = max.wsub(min);
            // C++ nests these (rangeutil.cc:468-469); collapsed (same semantics).
            if diff > 0 && diff <= (max_step as uintb) && leastsigbit_set(diff) == mostsigbit_set(diff)
            {
                self.step = diff as int4;
                self.left = min;
                self.right = (max.wadd(self.step as uintb)) & self.mask;
                return false;
            }
        }

        // Treat original ranges as having step=1
        let a_right = self.right.wsub(self.step as uintb).wadd(1);
        let b_right = op2.right.wsub(op2.step as uintb).wadd(1);
        self.step = 1;
        self.mask |= op2.mask;
        let vacant_size1;
        let vacant_size2;

        let overlap_code = CircleRange::encode_range_overlaps(self.left, a_right, op2.left, b_right);
        match overlap_code {
            b'a' => {
                // order (l r op2.l op2.r)
                vacant_size1 = self.left.wadd(self.mask.wsub(b_right)).wadd(1);
                vacant_size2 = op2.left.wsub(a_right);
                if vacant_size1 < vacant_size2 {
                    self.left = op2.left;
                    self.right = a_right;
                } else {
                    self.right = b_right;
                }
            }
            b'f' => {
                // order (op2.l op2.r l r)
                vacant_size1 = op2.left.wadd(self.mask.wsub(a_right)).wadd(1);
                vacant_size2 = self.left.wsub(b_right);
                if vacant_size1 < vacant_size2 {
                    self.right = b_right;
                } else {
                    self.left = op2.left;
                    self.right = a_right;
                }
            }
            b'b' => {
                // order (l op2.l r op2.r)
                self.right = b_right;
            }
            b'c' => {
                // order (l op2.l op2.r r)
                self.right = a_right;
            }
            b'd' => {
                // order (op2.l l r op2.r)
                self.left = op2.left;
                self.right = b_right;
            }
            b'e' => {
                // order (op2.l l op2.r r)
                self.left = op2.left;
                self.right = a_right;
            }
            b'g' => {
                // order (l op2.r op2.l r) — entire circle is covered
                self.left = 0;
                self.right = 0;
            }
            _ => {}
        }
        self.normalize();
        self.left == self.right
    }

    // -----------------------------------------------------------------------
    // invert / intersect / setNZMask / setStride (rangeutil.cc:533-722)
    // -----------------------------------------------------------------------

    /// Convert to complementary range.  The step is converted to 1 first
    /// (C++ `invert`, `rangeutil.cc:533`).  Returns the original step size.
    pub fn invert(&mut self) -> int4 {
        let res = self.step;
        self.step = 1;
        self.complement();
        res
    }

    /// Intersect `self` with another range (C++ `intersect`, `rangeutil.cc:549`).
    /// Returns 0 if the result is a single interval, 2 if it is two pieces (in
    /// which case `self` is unmodified).
    pub fn intersect(&mut self, op2: &CircleRange) -> int4 {
        let retval;
        let new_step;

        if self.isempty {
            return 0; // Intersection with empty is empty
        }
        if op2.isempty {
            self.isempty = true;
            return 0;
        }
        let mut myleft = self.left;
        let mut myright = self.right;
        let mut op2left = op2.left;
        let mut op2right = op2.right;
        if self.step < op2.step {
            new_step = op2.step;
            let rem = (op2left % (new_step as uintb)) as uint4;
            if CircleRange::new_stride(self.mask, new_step, self.step, rem, &mut myleft, &mut myright) {
                self.isempty = true;
                return 0;
            }
        } else if op2.step < self.step {
            new_step = self.step;
            let rem = (myleft % (new_step as uintb)) as uint4;
            if CircleRange::new_stride(op2.mask, new_step, op2.step, rem, &mut op2left, &mut op2right) {
                self.isempty = true;
                return 0;
            }
        } else {
            new_step = self.step;
        }
        let new_mask = self.mask & op2.mask;
        if self.mask != new_mask {
            if CircleRange::new_domain(new_mask, new_step, &mut myleft, &mut myright) {
                self.isempty = true;
                return 0;
            }
        } else if op2.mask != new_mask
            && CircleRange::new_domain(new_mask, new_step, &mut op2left, &mut op2right)
        {
            self.isempty = true;
            return 0;
        }
        if myleft == myright {
            // Intersect with this everything
            self.left = op2left;
            self.right = op2right;
            retval = 0;
        } else if op2left == op2right {
            // Intersect with op2 everything
            self.left = myleft;
            self.right = myright;
            retval = 0;
        } else {
            let overlap_code = CircleRange::encode_range_overlaps(myleft, myright, op2left, op2right);
            match overlap_code {
                b'a' | b'f' => {
                    // order (l r op2.l op2.r) | (op2.l op2.r l r)
                    self.isempty = true;
                    retval = 0; // empty set
                }
                b'b' => {
                    // order (l op2.l r op2.r)
                    self.left = op2left;
                    self.right = myright;
                    if self.left == self.right {
                        self.isempty = true;
                    }
                    retval = 0;
                }
                b'c' => {
                    // order (l op2.l op2.r r)
                    self.left = op2left;
                    self.right = op2right;
                    retval = 0;
                }
                b'd' => {
                    // order (op2.l l r op2.r)
                    self.left = myleft;
                    self.right = myright;
                    retval = 0;
                }
                b'e' => {
                    // order (op2.l l op2.r r)
                    self.left = myleft;
                    self.right = op2right;
                    if self.left == self.right {
                        self.isempty = true;
                    }
                    retval = 0;
                }
                b'g' => {
                    // order (l op2.r op2.l r)
                    if myleft == op2right {
                        self.left = op2left;
                        self.right = myright;
                        if self.left == self.right {
                            self.isempty = true;
                        }
                        retval = 0;
                    } else if op2left == myright {
                        self.left = myleft;
                        self.right = op2right;
                        if self.left == self.right {
                            self.isempty = true;
                        }
                        retval = 0;
                    } else {
                        retval = 2; // 2 pieces
                    }
                }
                _ => {
                    retval = 2; // Will never reach here
                }
            }
        }
        if retval != 0 {
            return retval;
        }
        self.mask = new_mask;
        self.step = new_step;
        0
    }

    /// Set the range based on a putative mask (C++ `setNZMask`, `rangeutil.cc:672`).
    /// Returns `true` if the mask is valid.
    pub fn set_nz_mask(&mut self, nzmask: uintb, size: int4) -> bool {
        let trans = bit_transitions(nzmask, size);
        if trans > 2 {
            return false; // Too many transitions to form a valid range
        }
        let hasstep = (nzmask & 1) == 0;
        if (!hasstep) && (trans == 2) {
            return false; // Two sections of non-zero bits
        }
        self.isempty = false;
        if trans == 0 {
            self.mask = calc_mask(size);
            if hasstep {
                // All zeros
                self.step = 1;
                self.left = 0;
                self.right = 1; // Range containing only zero
            } else {
                // All ones
                self.step = 1;
                self.left = 0;
                self.right = 0; // Everything
            }
            return true;
        }
        let shift = leastsigbit_set(nzmask);
        self.step = 1;
        self.step <<= shift; // step <<= shift (int4 shift)
        self.mask = calc_mask(size);
        self.left = 0;
        self.right = (nzmask.wadd(self.step as uintb)) & self.mask;
        true
    }

    /// Set a new step on `self` range (C++ `setStride`, `rangeutil.cc:707`).
    pub fn set_stride(&mut self, new_step: int4, rem: uintb) {
        let iseverything = (!self.isempty) && (self.left == self.right);
        if new_step == self.step {
            return;
        }
        let mut a_right = self.right.wsub(self.step as uintb);
        self.step = new_step;
        if self.step == 1 {
            return; // No remainder to fill in
        }
        let mut cur_rem = self.left % (self.step as uintb);
        self.left = self.left.wsub(cur_rem).wadd(rem);
        cur_rem = a_right % (self.step as uintb);
        a_right = a_right.wsub(cur_rem).wadd(rem);
        self.right = a_right.wadd(self.step as uintb);
        if (!iseverything) && (self.left == self.right) {
            self.isempty = true;
        }
    }

    // -----------------------------------------------------------------------
    // pullBackUnary / pullBackBinary (rangeutil.cc:728-1003)
    // -----------------------------------------------------------------------

    /// Pull-back `self` through the given unary operator (C++ `pullBackUnary`,
    /// `rangeutil.cc:728`).  Returns `true` if a valid range is formed.
    pub fn pull_back_unary(&mut self, opc: OpCode, in_size: int4, out_size: int4) -> bool {
        // If there is nothing in the output set, no input will map to it
        if self.isempty {
            return true;
        }

        match opc {
            OpCode::CPUI_BOOL_NEGATE => {
                if self.convert_to_boolean() {
                    // Both outputs possible => both inputs possible
                } else {
                    self.left ^= 1; // Flip the boolean range
                    self.right = self.left + 1;
                }
            }
            OpCode::CPUI_COPY => {
                // Identity transform on range
            }
            OpCode::CPUI_INT_2COMP => {
                let val = (!self.left).wadd(1).wadd(self.step as uintb) & self.mask;
                self.left = (!self.right).wadd(1).wadd(self.step as uintb) & self.mask;
                self.right = val;
            }
            OpCode::CPUI_INT_NEGATE => {
                let val = (!self.left).wadd(self.step as uintb) & self.mask;
                self.left = (!self.right).wadd(self.step as uintb) & self.mask;
                self.right = val;
            }
            OpCode::CPUI_INT_ZEXT => {
                let val = calc_mask(in_size); // (smaller) input mask
                let rem = self.left % (self.step as uintb);
                let mut zextrange = CircleRange::new_empty();
                zextrange.left = rem;
                zextrange.right = val.wadd(1).wadd(rem); // Biggest possible range of ZEXT
                zextrange.mask = self.mask;
                zextrange.step = self.step; // Keep the same stride
                zextrange.isempty = false;
                if 0 != self.intersect(&zextrange) {
                    return false;
                }
                self.left &= val;
                self.right &= val;
                self.mask &= val; // Preserve the stride
            }
            OpCode::CPUI_INT_SEXT => {
                let val = calc_mask(in_size); // (smaller) input mask
                let rem = self.left & (self.step as uintb);
                let mut sextrange = CircleRange::new_empty();
                sextrange.left = val ^ (val >> 1); // High order bit for (small) input space
                sextrange.left = sextrange.left.wadd(rem);
                sextrange.right = sign_extend_sized(sextrange.left, in_size, out_size);
                sextrange.mask = self.mask;
                sextrange.step = self.step; // Keep the same stride
                sextrange.isempty = false;
                // C++ (rangeutil.cc:782-792): fail if the sext-domain intersect
                // is two pieces, or if the resulting range is non-empty (the
                // input range extended outside the sext domain).  `||`
                // short-circuits so `is_empty()` is only read when intersect==0,
                // matching the nested C++ else branch exactly.
                if sextrange.intersect(self) != 0 || !sextrange.is_empty() {
                    return false;
                } else {
                    self.left &= val;
                    self.right &= val;
                    self.mask &= val; // Preserve the stride
                }
            }
            _ => return false,
        }
        true
    }

    /// Pull-back `self` through the given binary operator (C++ `pullBackBinary`,
    /// `rangeutil.cc:807`).  Returns `true` if a valid range is formed.
    pub fn pull_back_binary(
        &mut self,
        opc: OpCode,
        val: uintb,
        slot: int4,
        in_size: int4,
        _out_size: int4,
    ) -> bool {
        // If there is nothing in the output set, no input will map to it
        if self.isempty {
            return true;
        }

        match opc {
            OpCode::CPUI_INT_EQUAL => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true; // All possible outs => all possible ins
                }
                let yescomplement = self.left == 0;
                self.left = val;
                self.right = (val.wadd(1)) & self.mask;
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_NOTEQUAL => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                self.left = (val.wadd(1)) & self.mask;
                self.right = val;
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_LESS => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                if slot == 0 {
                    if val == 0 {
                        self.isempty = true; // X < 0 is always false
                    } else {
                        self.left = 0;
                        self.right = val;
                    }
                } else if val == self.mask {
                    self.isempty = true; // 0xffff < X is always false
                } else {
                    self.left = (val.wadd(1)) & self.mask;
                    self.right = 0;
                }
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_LESSEQUAL => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                if slot == 0 {
                    self.left = 0;
                    self.right = (val.wadd(1)) & self.mask;
                } else {
                    self.left = val;
                    self.right = 0;
                }
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_SLESS => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                if slot == 0 {
                    if val == (self.mask >> 1) + 1 {
                        self.isempty = true; // X < -infinity, always false
                    } else {
                        self.left = (self.mask >> 1) + 1; // -infinity
                        self.right = val;
                    }
                } else if val == (self.mask >> 1) {
                    self.isempty = true; // infinity < X, always false
                } else {
                    self.left = (val.wadd(1)) & self.mask;
                    self.right = (self.mask >> 1) + 1; // -infinity
                }
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_SLESSEQUAL => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                if slot == 0 {
                    self.left = (self.mask >> 1) + 1; // -infinity
                    self.right = (val.wadd(1)) & self.mask;
                } else {
                    self.left = val;
                    self.right = (self.mask >> 1) + 1; // -infinity
                }
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_CARRY => {
                let both_true_false = self.convert_to_boolean();
                self.mask = calc_mask(in_size);
                if both_true_false {
                    return true;
                }
                let yescomplement = self.left == 0;
                if val == 0 {
                    self.isempty = true; // Nothing carries adding zero
                } else {
                    self.left = (self.mask.wsub(val)).wadd(1) & self.mask;
                    self.right = 0;
                }
                if yescomplement {
                    self.complement();
                }
            }
            OpCode::CPUI_INT_ADD => {
                self.left = self.left.wsub(val) & self.mask;
                self.right = self.right.wsub(val) & self.mask;
            }
            OpCode::CPUI_INT_SUB => {
                if slot == 0 {
                    self.left = self.left.wadd(val) & self.mask;
                    self.right = self.right.wadd(val) & self.mask;
                } else {
                    self.left = val.wsub(self.left) & self.mask;
                    self.right = val.wsub(self.right) & self.mask;
                }
            }
            OpCode::CPUI_INT_RIGHT => {
                if self.step == 1 {
                    // val is the shift amount (passed in via `val`); cast to u32 for shift.
                    // C++ (rangeutil.cc:949): `(calc_mask(inSize) >> val) + 1`.
                    // `val` is a uintb shift count; x86 masks it mod 64 and the
                    // `+1` may wrap (val==0, 8-byte mask -> u64::MAX+1==0), so use
                    // wshr/wadd (matching the wshl already used for the LEFT shifts).
                    let right_bound = calc_mask(in_size).wshr(val as u32).wadd(1); // maximal right bound
                    if ((self.left >= right_bound)
                        && (self.right >= right_bound)
                        && (self.left >= self.right))
                        || ((self.left == 0) && (self.right >= right_bound))
                        || (self.left == self.right)
                    {
                        // covers everything in range of shift
                        self.left = 0;
                        self.right = 0;
                    } else {
                        if self.left > right_bound {
                            self.left = right_bound;
                        }
                        if self.right > right_bound {
                            self.right = 0;
                        }
                        self.left = (self.left.wshl(val as u32)) & self.mask;
                        self.right = (self.right.wshl(val as u32)) & self.mask;
                        if self.left == self.right {
                            self.isempty = true;
                        }
                    }
                } else {
                    return false;
                }
            }
            OpCode::CPUI_INT_SRIGHT => {
                if self.step == 1 {
                    let mut rightb = calc_mask(in_size);
                    // C++ (rangeutil.cc:975): `rightb >> (val + 1)`. `val` is a
                    // uintb; `val+1` may wrap and the x86 shift masks the count
                    // mod 64, so use wadd for the count and wshr for the shift.
                    let mut leftb = rightb.wshr(val.wadd(1) as u32);
                    rightb ^= leftb; // Smallest negative possible (rangeutil.cc:976)
                    leftb = leftb.wadd(1); // Biggest positive (+1) possible (rangeutil.cc:977; wraps for full mask)
                    if ((self.left >= leftb)
                        && (self.left <= rightb)
                        && (self.right >= leftb)
                        && (self.right <= rightb)
                        && (self.left >= self.right))
                        || (self.left == self.right)
                    {
                        // covers everything in range of shift
                        self.left = 0;
                        self.right = 0;
                    } else {
                        if (self.left > leftb) && (self.left < rightb) {
                            self.left = leftb;
                        }
                        if (self.right > leftb) && (self.right < rightb) {
                            self.right = rightb;
                        }
                        self.left = (self.left.wshl(val as u32)) & self.mask;
                        self.right = (self.right.wshl(val as u32)) & self.mask;
                        if self.left == self.right {
                            self.isempty = true;
                        }
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
        true
    }

    // -----------------------------------------------------------------------
    // pushForwardUnary / pushForwardBinary / pushForwardTrinary
    // (rangeutil.cc:1093-1389)
    // -----------------------------------------------------------------------

    /// Push the given input range forward through a unary operator (C++
    /// `pushForwardUnary`, `rangeutil.cc:1093`).  Returns `true` if the result
    /// forms a range.
    pub fn push_forward_unary(
        &mut self,
        opc: OpCode,
        in1: &CircleRange,
        in_size: int4,
        out_size: int4,
    ) -> bool {
        if in1.isempty {
            self.isempty = true;
            return true;
        }
        match opc {
            OpCode::CPUI_CAST | OpCode::CPUI_COPY => {
                *self = in1.clone();
            }
            OpCode::CPUI_INT_ZEXT => {
                self.isempty = false;
                self.step = in1.step;
                self.mask = calc_mask(out_size);
                if in1.left == in1.right {
                    self.left = in1.left % (self.step as uintb);
                    self.right = in1.mask.wadd(1).wadd(self.left);
                } else {
                    self.left = in1.left;
                    self.right = in1.right.wsub(in1.step as uintb) & in1.mask;
                    if self.right < self.left {
                        return false; // Extending causes 2 pieces
                    }
                    self.right = self.right.wadd(self.step as uintb); // Impossible to wrap with bigger mask
                }
            }
            OpCode::CPUI_INT_SEXT => {
                self.isempty = false;
                self.step = in1.step;
                self.mask = calc_mask(out_size);
                if in1.left == in1.right {
                    let rem = in1.left % (self.step as uintb);
                    self.right = calc_mask(in_size) >> 1;
                    self.left = (calc_mask(out_size) ^ self.right).wadd(rem);
                    self.right = self.right.wadd(1).wadd(rem);
                } else {
                    self.left = sign_extend_sized(in1.left, in_size, out_size);
                    self.right =
                        sign_extend_sized(in1.right.wsub(in1.step as uintb) & in1.mask, in_size, out_size);
                    if (self.right as intb) < (self.left as intb) {
                        return false; // Extending causes 2 pieces
                    }
                    self.right = self.right.wadd(self.step as uintb) & self.mask;
                }
            }
            OpCode::CPUI_INT_2COMP => {
                self.isempty = false;
                self.step = in1.step;
                self.mask = in1.mask;
                self.right = (!in1.left).wadd(1).wadd(self.step as uintb) & self.mask;
                self.left = (!in1.right).wadd(1).wadd(self.step as uintb) & self.mask;
                self.normalize();
            }
            OpCode::CPUI_INT_NEGATE => {
                self.isempty = false;
                self.step = in1.step;
                self.mask = in1.mask;
                self.left = (!in1.right).wadd(self.step as uintb) & self.mask;
                self.right = (!in1.left).wadd(self.step as uintb) & self.mask;
                self.normalize();
            }
            OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_FLOAT_NAN => {
                self.isempty = false;
                self.mask = 0xff;
                self.step = 1;
                self.left = 0;
                self.right = 2;
            }
            _ => return false,
        }
        true
    }

    /// Push the given input ranges forward through a binary operator (C++
    /// `pushForwardBinary`, `rangeutil.cc:1180`).  Returns `true` if the result
    /// forms a range.
    pub fn push_forward_binary(
        &mut self,
        opc: OpCode,
        in1: &CircleRange,
        in2: &CircleRange,
        _in_size: int4,
        out_size: int4,
        max_step: int4,
    ) -> bool {
        if in1.isempty || in2.isempty {
            self.isempty = true;
            return true;
        }
        match opc {
            OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => {
                self.isempty = false;
                self.mask = in1.mask | in2.mask;
                if in1.left == in1.right || in2.left == in2.right {
                    self.step = if in1.step < in2.step { in1.step } else { in2.step }; // Smaller step
                    self.left = in1.left.wadd(in2.left) % (self.step as uintb);
                    self.right = self.left;
                } else if in2.is_single() {
                    self.step = in1.step;
                    self.left = in1.left.wadd(in2.left) & self.mask;
                    self.right = in1.right.wadd(in2.left) & self.mask;
                } else if in1.is_single() {
                    self.step = in2.step;
                    self.left = in2.left.wadd(in1.left) & self.mask;
                    self.right = in2.right.wadd(in1.left) & self.mask;
                } else {
                    self.step = if in1.step < in2.step { in1.step } else { in2.step }; // Smaller step
                    let size1 = if in1.left < in1.right {
                        in1.right.wsub(in1.left)
                    } else {
                        in1.mask.wsub(in1.left.wsub(in1.right)).wadd(in1.step as uintb)
                    };
                    self.left = in1.left.wadd(in2.left) & self.mask;
                    self.right = in1
                        .right
                        .wsub(in1.step as uintb)
                        .wadd(in2.right)
                        .wsub(in2.step as uintb)
                        .wadd(self.step as uintb)
                        & self.mask;
                    let sizenew = if self.left < self.right {
                        self.right.wsub(self.left)
                    } else {
                        self.mask.wsub(self.left.wsub(self.right)).wadd(self.step as uintb)
                    };
                    if sizenew < size1 {
                        self.right = self.left; // Over-flow, we covered everything
                    }
                    self.normalize();
                }
            }
            OpCode::CPUI_INT_MULT => {
                self.isempty = false;
                self.mask = in1.mask | in2.mask;
                let const_val;
                if in1.is_single() {
                    const_val = in1.get_min();
                    self.step = in2.step;
                } else if in2.is_single() {
                    const_val = in2.get_min();
                    self.step = in1.step;
                } else {
                    return false;
                }
                let mut tmp = const_val as uint4;
                while self.step < max_step {
                    if (tmp & 1) != 0 {
                        break;
                    }
                    self.step <<= 1;
                    tmp >>= 1;
                }
                let whole_size =
                    8 * (std::mem::size_of::<uintb>() as int4) - count_leading_zeros(self.mask);
                if in1.get_max_info() + in2.get_max_info() > whole_size {
                    self.left = in1.left.wmul(in2.left) % (self.step as uintb);
                    self.right = self.left; // Covered everything
                    self.normalize();
                    return true;
                }
                if (const_val & (self.mask ^ (self.mask >> 1))) != 0 {
                    // Multiplying by a negative number
                    self.left = (in1.right.wsub(in1.step as uintb)).wmul(in2.right.wsub(in2.step as uintb))
                        & self.mask;
                    self.right = (in1.left.wmul(in2.left)).wadd(self.step as uintb) & self.mask;
                } else {
                    self.left = in1.left.wmul(in2.left) & self.mask;
                    self.right = (in1.right.wsub(in1.step as uintb))
                        .wmul(in2.right.wsub(in2.step as uintb))
                        .wadd(self.step as uintb)
                        & self.mask;
                }
            }
            OpCode::CPUI_INT_LEFT => {
                if !in2.is_single() {
                    return false;
                }
                self.isempty = false;
                self.mask = in1.mask;
                self.step = in1.step;
                let sa = in2.get_min() as uint4;
                let mut tmp = sa;
                while self.step < max_step && tmp > 0 {
                    self.step <<= 1;
                    tmp -= 1;
                }
                self.left = (in1.left.wshl(sa)) & self.mask;
                self.right = (in1.right.wshl(sa)) & self.mask;
                let whole_size =
                    8 * (std::mem::size_of::<uintb>() as int4) - count_leading_zeros(self.mask);
                // mixed: int4 getMaxInfo() + uint4 sa > int4 wholeSize
                if in1.get_max_info() + (sa as int4) > whole_size {
                    self.right = self.left; // Covered everything
                    self.normalize();
                    return true;
                }
            }
            OpCode::CPUI_SUBPIECE => {
                if !in2.is_single() {
                    return false;
                }
                self.isempty = false;
                // C++ (rangeutil.cc:1283): `(int4)in2.left * 8`. The i32
                // multiply wraps on x86 (in2.left is a non-negative single
                // constant); use wmul so a large offset cannot panic in debug.
                let sa = (in2.left as int4).wmul(8);
                self.mask = calc_mask(out_size);
                self.step = if sa == 0 { in1.step } else { 1 };
                let range =
                    if in1.left < in1.right { in1.right.wsub(in1.left) } else { in1.left.wsub(in1.right) };

                // sa is a non-negative int4; x86 masks the shift count mod 64
                // (rangeutil.cc:1288), so use wshr (a SUBPIECE offset 8 gives sa=64).
                if range == 0 || (range.wshr(sa as u32) > self.mask) {
                    self.left = 0;
                    self.right = 0; // We cover everything
                } else {
                    self.left = in1.left.wshr(sa as u32); // x86 shift mod 64 (rangeutil.cc:1292/1308)
                    self.right = (in1.right.wsub(in1.step as uintb)).wshr(sa as u32).wadd(self.step as uintb); // rangeutil.cc:1293 (x86 shift mod 64)
                    self.left &= self.mask;
                    self.right &= self.mask;
                    self.normalize();
                }
            }
            OpCode::CPUI_INT_RIGHT => {
                if !in2.is_single() {
                    return false;
                }
                self.isempty = false;
                let sa = in2.left as int4;
                self.mask = calc_mask(out_size);
                self.step = 1; // Lose any step
                if in1.left < in1.right {
                    self.left = in1.left.wshr(sa as u32); // x86 shift mod 64 (rangeutil.cc:1292/1308)
                    self.right = (in1.right.wsub(in1.step as uintb)).wshr(sa as u32).wadd(1); // rangeutil.cc:1309 (x86 shift mod 64)
                } else {
                    self.left = 0;
                    self.right = in1.mask.wshr(sa as u32); // rangeutil.cc:1313 (x86 shift mod 64)
                }
                if self.left == self.right {
                    // Don't truncate accidentally to everything
                    self.right = (self.left.wadd(1)) & self.mask;
                }
            }
            OpCode::CPUI_INT_SRIGHT => {
                if !in2.is_single() {
                    return false;
                }
                self.isempty = false;
                let sa = in2.left as int4;
                self.mask = calc_mask(out_size);
                self.step = 1; // Lose any step
                let bit_pos = 8 * _in_size - 1;
                let mut val_left = sign_extend(in1.left as intb, bit_pos);
                let mut val_right = sign_extend(in1.right as intb, bit_pos);
                if val_left >= val_right {
                    val_right = (self.mask >> 1) as intb; // Max positive
                    // C++ rangeutil.cc:1331 relies on two's-complement wrap
                    // (i64::MAX+1 -> i64::MIN) when mask==u64::MAX (out_size==8).
                    val_left = val_right.wadd(1); // Min negative
                    val_left = sign_extend(val_left, bit_pos);
                }
                // left = (valLeft >> sa) & mask  (intb arithmetic shift)
                self.left = ((val_left.wshr(sa as u32)) as uintb) & self.mask; // rangeutil.cc:1334 (arithmetic shr, x86 mod 64)
                // right = (((valRight - in1.step) >> sa) + 1) & mask
                self.right = (((val_right.wsub(in1.step as intb)).wshr(sa as u32).wadd(1)) as uintb) & self.mask; // rangeutil.cc:1335 (arithmetic shr, x86 mod 64)
                if self.left == self.right {
                    self.right = (self.left.wadd(1)) & self.mask;
                }
            }
            OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL => {
                // Ops with boolean outcome.  We don't try to eliminate outcomes here.
                self.isempty = false;
                self.mask = 0xff;
                self.step = 1;
                self.left = 0; // Both true and false are possible
                self.right = 2;
            }
            _ => return false,
        }
        true
    }

    /// Push the given input ranges forward through a trinary operator (only
    /// `CPUI_PTRADD`; C++ `pushForwardTrinary`, `rangeutil.cc:1381`).
    // Mirrors the upstream 7-parameter signature (+ `&mut self`); the arg count
    // is dictated by the C++ spec, not refactorable without diverging.
    #[allow(clippy::too_many_arguments)]
    pub fn push_forward_trinary(
        &mut self,
        opc: OpCode,
        in1: &CircleRange,
        in2: &CircleRange,
        in3: &CircleRange,
        in_size: int4,
        out_size: int4,
        max_step: int4,
    ) -> bool {
        if opc != OpCode::CPUI_PTRADD {
            return false;
        }
        let mut tmp_range = CircleRange::new_empty();
        if !tmp_range.push_forward_binary(OpCode::CPUI_INT_MULT, in2, in3, in_size, in_size, max_step) {
            return false;
        }
        self.push_forward_binary(OpCode::CPUI_INT_ADD, in1, &tmp_range, in_size, out_size, max_step)
    }

    // -----------------------------------------------------------------------
    // widen / translate2Op (rangeutil.cc:1395-1467)
    // -----------------------------------------------------------------------

    /// Widen `self` so at least one boundary matches the given containing range
    /// (C++ `widen`, `rangeutil.cc:1395`).
    pub fn widen(&mut self, op2: &CircleRange, left_is_stable: bool) {
        if left_is_stable {
            let lmod = self.left % (self.step as uintb);
            let m = op2.right % (self.step as uintb);
            if m <= lmod {
                self.right = op2.right.wadd(lmod - m);
            } else {
                self.right = op2.right.wsub(m - lmod);
            }
            self.right &= self.mask;
        } else {
            self.left = op2.left & self.mask;
        }
        self.normalize();
    }

    /// Translate range to a comparison op (C++ `translate2Op`, `rangeutil.cc:1424`).
    /// Returns: 0 on success, 1 if all inputs must return true, 2 if not
    /// possible, 3 if no inputs must return true.  On success `opc`/`c`/`cslot`
    /// hold the comparison op, the constant input, and the constant's slot.
    pub fn translate2_op(&self, opc: &mut OpCode, c: &mut uintb, cslot: &mut int4) -> int4 {
        if self.isempty {
            return 3;
        }
        if self.step != 1 {
            return 2; // Not possible with a stride
        }
        if self.right == ((self.left.wadd(1)) & self.mask) {
            // Single value
            *opc = OpCode::CPUI_INT_EQUAL;
            *cslot = 0;
            *c = self.left;
            return 0;
        }
        if self.left == ((self.right.wadd(1)) & self.mask) {
            // All but one value
            *opc = OpCode::CPUI_INT_NOTEQUAL;
            *cslot = 0;
            *c = self.right;
            return 0;
        }
        if self.left == self.right {
            return 1; // All outputs are possible
        }
        if self.left == 0 {
            *opc = OpCode::CPUI_INT_LESS;
            *cslot = 1;
            *c = self.right;
            return 0;
        }
        if self.right == 0 {
            *opc = OpCode::CPUI_INT_LESS;
            *cslot = 0;
            *c = (self.left.wsub(1)) & self.mask;
            return 0;
        }
        if self.left == (self.mask >> 1) + 1 {
            *opc = OpCode::CPUI_INT_SLESS;
            *cslot = 1;
            *c = self.right;
            return 0;
        }
        if self.right == (self.mask >> 1) + 1 {
            *opc = OpCode::CPUI_INT_SLESS;
            *cslot = 0;
            *c = (self.left.wsub(1)) & self.mask;
            return 0;
        }
        2 // Cannot represent
    }
}

// ===========================================================================
// Widener (rangeutil.hh:204-261) — CircleRange-only widening strategy
// ===========================================================================

/// A particular widening strategy for the `ValueSetSolver` iteration algorithm
/// (C++ `Widener`, `rangeutil.hh:204`).
///
/// This object decides when a value set is *frozen* (`check_freeze`) and when /
/// by how much value sets are artificially grown to accelerate stabilization
/// (`do_widening`).  The trait operates purely on a [`ValueSet`]'s count /
/// range plus its landmark, so it is portable independent of the IR.
pub trait Widener {
    /// Upon entering a fresh partition, determine how the given ValueSet count
    /// should be reset (C++ `determineIterationReset`).
    fn determine_iteration_reset(&self, value_set: &ValueSet) -> int4;

    /// Check if the given value set has been frozen for the rest of the
    /// iteration (C++ `checkFreeze`).
    fn check_freeze(&self, value_set: &ValueSet) -> bool;

    /// For an iteration that isn't stabilizing, attempt to widen the given
    /// ValueSet (C++ `doWidening`).  `range` is the previous form (and the
    /// storage for the widened result), `new_range` is the current iteration.
    /// Returns `true` if widening succeeded.
    fn do_widening(&self, value_set: &ValueSet, range: &mut CircleRange, new_range: &CircleRange)
        -> bool;
}

/// Normal widening (C++ `WidenerFull`, `rangeutil.hh:236`).
pub struct WidenerFull {
    widen_iteration: int4,
    full_iteration: int4,
}

impl Default for WidenerFull {
    /// C++ default constructor: `widenIteration = 2; fullIteration = 5;`.
    fn default() -> Self {
        WidenerFull { widen_iteration: 2, full_iteration: 5 }
    }
}

impl WidenerFull {
    /// Constructor with default iterations (C++ `WidenerFull(void)`).
    pub fn new() -> WidenerFull {
        WidenerFull::default()
    }

    /// Constructor specifying iterations (C++ `WidenerFull(int4,int4)`).
    pub fn with_iterations(wide: int4, full: int4) -> WidenerFull {
        WidenerFull { widen_iteration: wide, full_iteration: full }
    }
}

impl Widener for WidenerFull {
    fn determine_iteration_reset(&self, value_set: &ValueSet) -> int4 {
        if value_set.get_count() >= self.widen_iteration {
            return self.widen_iteration; // Reset to point just after any widening
        }
        0 // Delay widening, if we haven't performed it yet
    }

    fn check_freeze(&self, value_set: &ValueSet) -> bool {
        value_set.get_range().is_full()
    }

    fn do_widening(
        &self,
        value_set: &ValueSet,
        range: &mut CircleRange,
        new_range: &CircleRange,
    ) -> bool {
        if value_set.get_count() < self.widen_iteration {
            *range = new_range.clone();
            true
        } else if value_set.get_count() == self.widen_iteration {
            if let Some(landmark) = value_set.get_landmark() {
                let left_is_stable = range.get_min() == new_range.get_min();
                *range = new_range.clone(); // Preserve any new step information
                if landmark.contains(range) {
                    range.widen(landmark, left_is_stable);
                    return true;
                } else {
                    let mut constraint = landmark.clone();
                    constraint.invert();
                    if constraint.contains(range) {
                        range.widen(&constraint, left_is_stable);
                        return true;
                    }
                }
            }
            false
        } else if value_set.get_count() < self.full_iteration {
            *range = new_range.clone();
            true
        } else {
            false // Indicate that constrained widening failed (set to full)
        }
    }
}

/// Freeze value sets at a specific iteration (C++ `WidenerNone`,
/// `rangeutil.hh:254`).
pub struct WidenerNone {
    freeze_iteration: int4,
}

impl Default for WidenerNone {
    /// C++ default constructor: `freezeIteration = 3;`.
    fn default() -> Self {
        WidenerNone { freeze_iteration: 3 }
    }
}

impl WidenerNone {
    /// Constructor with default freeze iteration (C++ `WidenerNone(void)`).
    pub fn new() -> WidenerNone {
        WidenerNone::default()
    }
}

impl Widener for WidenerNone {
    fn determine_iteration_reset(&self, value_set: &ValueSet) -> int4 {
        if value_set.get_count() >= self.freeze_iteration {
            return self.freeze_iteration; // Reset to point just after any widening
        }
        value_set.get_count()
    }

    fn check_freeze(&self, value_set: &ValueSet) -> bool {
        if value_set.get_range().is_full() {
            return true;
        }
        value_set.get_count() >= self.freeze_iteration
    }

    fn do_widening(
        &self,
        _value_set: &ValueSet,
        range: &mut CircleRange,
        new_range: &CircleRange,
    ) -> bool {
        *range = new_range.clone();
        true
    }
}

// ===========================================================================
// ValueSet / Partition / ValueSetSolver (rangeutil.hh:113-327)
//
// The graph machinery is IR-coupled.  Where the C++ threads raw pointers
// (`ValueSet::vn`, `Varnode::setValueSet`, `partHead`, `next`) this port keeps
// arena indices on the solver plus a `VarnodeId -> node` map that plays the
// role of both the back-pointer and the Varnode mark bit (upstream always sets
// the two together and clears the marks when the map would die).
// ===========================================================================

/// An equation attached to a [`ValueSet`] (C++ `ValueSet::Equation`,
/// `rangeutil.hh:121`).
#[derive(Debug, Clone)]
pub struct Equation {
    /// The input parameter slot to which the constraint is attached.
    pub slot: int4,
    /// The constraint characteristic: 0=absolute, 1=relative to a spacebase.
    pub type_code: int4,
    /// The range constraint.
    pub range: CircleRange,
}

impl Equation {
    /// Construct an equation (C++ `Equation(int4,int4,const CircleRange &)`).
    pub fn new(slot: int4, type_code: int4, range: CircleRange) -> Equation {
        Equation { slot, type_code, range }
    }
}

/// A range of values attached to a Varnode within a data-flow subsystem
/// (C++ `ValueSet`, `rangeutil.hh:113`).
///
/// The linked-list graph fields (`next`, `partHead`) that the C++ threads as
/// raw pointers live as parallel index vectors on the [`ValueSetSolver`]; the
/// bound Varnode (`vn`) is carried here as an arena id (`None` for the
/// solver's simulated root and for the graph-test nodes).
#[derive(Debug, Clone)]
pub struct ValueSet {
    /// 0=pure constant, 1=stack relative.
    type_code: int4,
    /// Number of input parameters to the defining operation.
    num_params: int4,
    /// Depth-first numbering / widening count.
    count: int4,
    /// Op-code defining the Varnode.
    op_code: OpCode,
    /// `true` if left boundary of range didn't change (last iteration).
    left_is_stable: bool,
    /// `true` if right boundary of range didn't change (last iteration).
    right_is_stable: bool,
    /// The Varnode this set is attached to (C++ `vn`; `None` = simulated root).
    vn: Option<VarnodeId>,
    /// Range of values or offsets in this set.
    range: CircleRange,
    /// Any equations associated with this value set.
    equations: Vec<Equation>,
}

/// Maximum step inferred for a value set (C++ `ValueSet::MAX_STEP`,
/// `rangeutil.cc:1494`).
pub const VALUESET_MAX_STEP: int4 = 32;

impl Default for ValueSet {
    fn default() -> Self {
        ValueSet {
            type_code: 0,
            num_params: 0,
            count: 0,
            op_code: OpCode::CPUI_MAX,
            left_is_stable: false,
            right_is_stable: false,
            vn: None,
            range: CircleRange::new_empty(),
            equations: Vec::new(),
        }
    }
}

impl ValueSet {
    /// Construct an empty value set.
    pub fn new() -> ValueSet {
        ValueSet::default()
    }

    /// Get the current iteration count (C++ `getCount`).
    pub fn get_count(&self) -> int4 {
        self.count
    }

    /// Set the iteration count (used by the solver / tests).
    pub fn set_count(&mut self, c: int4) {
        self.count = c;
    }

    /// Return '0' for normal constant, '1' for spacebase relative
    /// (C++ `getTypeCode`).
    pub fn get_type_code(&self) -> int4 {
        self.type_code
    }

    /// Get the actual range of values (C++ `getRange`).
    pub fn get_range(&self) -> &CircleRange {
        &self.range
    }

    /// Set the range (test/solver helper; not an upstream public method).
    pub fn set_range(&mut self, range: CircleRange) {
        self.range = range;
    }

    /// `true` if the left boundary hasn't been changing (C++ `isLeftStable`).
    pub fn is_left_stable(&self) -> bool {
        self.left_is_stable
    }

    /// `true` if the right boundary hasn't been changing (C++ `isRightStable`).
    pub fn is_right_stable(&self) -> bool {
        self.right_is_stable
    }

    /// The Varnode attached to this value set (C++ `getVarnode`).
    pub fn get_varnode(&self) -> Option<VarnodeId> {
        self.vn
    }

    /// Mark value set as possibly containing any value (C++ `setFull`).
    ///
    /// Upstream reads `vn->getSize()` for the range size; the caller supplies
    /// the bound Varnode's size explicitly.
    pub fn set_full(&mut self, size: int4) {
        self.range.set_full(size);
        self.type_code = 0;
    }

    /// Insert an equation restricting this value set, ordered on slot
    /// (C++ `addEquation`, `rangeutil.cc:1549`).
    pub fn add_equation(&mut self, slot: int4, type_code: int4, constraint: CircleRange) {
        // Find the first equation whose slot exceeds `slot`, insert before it.
        let mut pos = 0;
        while pos < self.equations.len() {
            if self.equations[pos].slot > slot {
                break;
            }
            pos += 1;
        }
        self.equations.insert(pos, Equation::new(slot, type_code, constraint));
    }

    /// Add a widening landmark (C++ `addLandmark`).
    pub fn add_landmark(&mut self, type_code: int4, constraint: CircleRange) {
        self.add_equation(self.num_params, type_code, constraint);
    }

    /// Does the indicated equation apply for the given input slot
    /// (C++ `doesEquationApply`, `rangeutil.hh:375`).
    pub fn does_equation_apply(&self, num: int4, slot: int4) -> bool {
        if (num as usize) < self.equations.len() {
            let eq = &self.equations[num as usize];
            if eq.slot == slot && eq.type_code == self.type_code {
                return true;
            }
        }
        false
    }

    /// Get any landmark range (C++ `getLandMark`, `rangeutil.cc:1742`).
    ///
    /// Any equation can serve as a landmark.  Prefer the one restricting an
    /// input branch (`typeCode` matches), as these give a tighter approximation.
    pub fn get_landmark(&self) -> Option<&CircleRange> {
        for eq in &self.equations {
            if eq.type_code == self.type_code {
                return Some(&eq.range);
            }
        }
        None
    }
}

/// A range of nodes (within the weak topological ordering) iterated together
/// (C++ `Partition`, `rangeutil.hh:161`).
///
/// The C++ threads `startNode`/`stopNode` as raw `ValueSet *` through a linked
/// list; here partitions index into the solver's node arena.
#[derive(Debug, Clone, Default)]
pub struct Partition {
    /// Index of the starting node of the component (or `None` if empty).
    pub start_node: Option<usize>,
    /// Index of the ending node of the component (or `None` if empty).
    pub stop_node: Option<usize>,
    /// `true` if a node in this component has changed this iteration.
    pub is_dirty: bool,
}

impl Partition {
    /// Construct an empty partition (C++ `Partition(void)`).
    pub fn new() -> Partition {
        Partition::default()
    }
}

/// A special ValueSet for the *read point* of a Varnode (C++ `ValueSetRead`,
/// `rangeutil.hh:178`).
///
/// Holds the value set at a specific read (PcodeOp + slot), possibly modified
/// from the Varnode's own set by control-flow constraints at the read.
#[derive(Debug, Clone)]
pub struct ValueSetRead {
    type_code: int4,
    /// The PcodeOp at the point of the value set read (C++ `op`).
    op: Option<OpId>,
    slot: int4,
    range: CircleRange,
    equation_constraint: CircleRange,
    equation_type_code: int4,
    left_is_stable: bool,
    right_is_stable: bool,
}

impl Default for ValueSetRead {
    fn default() -> Self {
        ValueSetRead {
            type_code: 0,
            op: None,
            slot: 0,
            range: CircleRange::new_empty(),
            equation_constraint: CircleRange::new_empty(),
            equation_type_code: -1,
            left_is_stable: false,
            right_is_stable: false,
        }
    }
}

impl ValueSetRead {
    /// Construct an empty read value set.
    pub fn new() -> ValueSetRead {
        ValueSetRead::default()
    }

    /// Establish that this value set corresponds to read `(o, slt)`
    /// (C++ `setPcodeOp`, `rangeutil.cc:1781`).
    pub fn set_pcode_op(&mut self, o: OpId, slt: int4) {
        self.type_code = 0;
        self.op = Some(o);
        self.slot = slt;
        self.equation_type_code = -1;
    }

    /// Insert an equation restricting this value set (C++ `addEquation`,
    /// `rangeutil.cc:1793`).
    pub fn add_equation(&mut self, slt: int4, type_code: int4, constraint: CircleRange) {
        if self.slot == slt {
            self.equation_type_code = type_code;
            self.equation_constraint = constraint;
        }
    }

    /// Return '0' for normal constant, '1' for spacebase relative.
    pub fn get_type_code(&self) -> int4 {
        self.type_code
    }

    /// Get the actual range of values (C++ `getRange`).
    pub fn get_range(&self) -> &CircleRange {
        &self.range
    }

    /// `true` if the left boundary hasn't been changing.
    pub fn is_left_stable(&self) -> bool {
        self.left_is_stable
    }

    /// `true` if the right boundary hasn't been changing.
    pub fn is_right_stable(&self) -> bool {
        self.right_is_stable
    }

    /// The IR-independent core of C++ `ValueSetRead::compute`
    /// (`rangeutil.cc:1804`): given the bound Varnode's value set fields, apply
    /// the equation constraint.  The solver resolves `op->getIn(slot)` to that
    /// value set and calls this (see [`ValueSetSolver::solve`]).
    pub fn compute_from(&mut self, value_set: &ValueSet) {
        self.type_code = value_set.get_type_code();
        self.range = value_set.get_range().clone();
        self.left_is_stable = value_set.is_left_stable();
        self.right_is_stable = value_set.is_right_stable();
        if self.type_code == self.equation_type_code
            && 0 != self.range.intersect(&self.equation_constraint)
        {
            self.range = self.equation_constraint.clone();
        }
    }
}

/// Edge abstraction for the value-set graph (the `ValueSetEdge` iterator over
/// out-bound edges, C++ `ValueSetSolver::ValueSetEdge`).
///
/// STUB(W5/W7-IR): upstream walks the descendant PcodeOps of a node's Varnode
/// and yields the ValueSet of each output that is `isMark()`ed, plus a simulated
/// root that yields `rootNodes`.  Because that requires the Varnode->ValueSet
/// back-pointer, the solver here consumes a precomputed adjacency list (node
/// index -> successor node indices) that mirrors that edge set exactly.  This
/// keeps the Bourdoncle ordering algorithm faithful and self-testable.
pub trait ValueSetGraph {
    /// Successor node indices of `node` (in the exact order the C++
    /// `ValueSetEdge::getNext` would yield them: root edges first for the
    /// simulated root, then descendant-op outputs).
    fn successors(&self, node: usize) -> Vec<usize>;
}

/// Determines a ValueSet for each Varnode in a data-flow system
/// (C++ `ValueSetSolver`, `rangeutil.hh:274`).
///
/// The system is formed by providing a set of sink Varnodes via
/// [`establish_value_sets`](ValueSetSolver::establish_value_sets); running
/// [`solve`](ValueSetSolver::solve) iterates the sets to a fixed point in the
/// **Bourdoncle weak-topological-ordering** established by
/// `establish_topological_order`, and results are read back through
/// [`get_value_set_read`](ValueSetSolver::get_value_set_read) or the node
/// arena.
pub struct ValueSetSolver {
    /// Storage for all the current value sets (C++ `valueNodes`).
    value_nodes: Vec<ValueSet>,
    /// `next` pointer of each node, as an index into `value_nodes` (C++
    /// `ValueSet::next`).  `None` terminates the iteration list.
    next: Vec<Option<usize>>,
    /// Partition head index of each node, or `None` (C++ `ValueSet::partHead`,
    /// here indexing into `record_storage`).
    part_head: Vec<Option<usize>>,
    /// Additional, after iteration, add-on value sets at specific reads
    /// (C++ `readNodes`, keyed by the read op instead of its SeqNum).
    read_nodes: HashMap<OpId, ValueSetRead>,
    /// Value sets in iteration order (C++ `orderPartition`).
    order_partition: Partition,
    /// Storage for the Partitions establishing components (C++ `recordStorage`).
    record_storage: Vec<Partition>,
    /// Values treated as inputs (C++ `rootNodes`) — node indices.
    root_nodes: Vec<usize>,
    /// Stack used to generate the topological ordering (C++ `nodeStack`).
    node_stack: Vec<usize>,
    /// The Varnode -> node-index binding (C++ `Varnode::setValueSet` back-
    /// pointer).  Membership doubles as upstream's Varnode mark bit: the two
    /// are always established together and the marks die with the solver.
    value_set_map: HashMap<VarnodeId, usize>,
    /// (Global) depth-first numbering for topological ordering.
    depth_first_index: int4,
    /// Count of individual ValueSet iterations.
    num_iterations: int4,
    /// Maximum number of iterations before forcing termination.
    max_iterations: int4,
}

impl Default for ValueSetSolver {
    fn default() -> Self {
        ValueSetSolver {
            value_nodes: Vec::new(),
            next: Vec::new(),
            part_head: Vec::new(),
            read_nodes: HashMap::new(),
            order_partition: Partition::new(),
            record_storage: Vec::new(),
            root_nodes: Vec::new(),
            node_stack: Vec::new(),
            value_set_map: HashMap::new(),
            depth_first_index: 0,
            num_iterations: 0,
            max_iterations: 0,
        }
    }
}

impl ValueSetSolver {
    /// Construct an empty solver.
    pub fn new() -> ValueSetSolver {
        ValueSetSolver::default()
    }

    /// Get the current number of iterations (C++ `getNumIterations`).
    pub fn get_num_iterations(&self) -> int4 {
        self.num_iterations
    }

    /// Borrow the value-set nodes (C++ `begin/endValueSets`).
    pub fn value_sets(&self) -> &[ValueSet] {
        &self.value_nodes
    }

    /// Allocate storage for a new (unbound) ValueSet, returning its arena
    /// index.  The IR-bound [`new_value_set`](ValueSetSolver::new_value_set)
    /// builds on this; the topological-ordering tests use it directly to build
    /// a synthetic graph.
    pub fn alloc_value_set(&mut self) -> usize {
        self.value_nodes.push(ValueSet::new());
        self.next.push(None);
        self.part_head.push(None);
        self.value_nodes.len() - 1
    }

    /// Allocate a new ValueSet attached to the given Varnode and record the
    /// back-pointer (C++ `newValueSet`, `rangeutil.cc:1953`, plus
    /// `ValueSet::setVarnode`, `rangeutil.cc:1503`).
    ///
    /// The initial values are set based on the type of Varnode: a constant gets
    /// the single value, an input gets all possible values, other written
    /// Varnodes start with an empty set.
    fn new_value_set(&mut self, fd: &Funcdata, vn: VarnodeId, t_code: int4) {
        let idx = self.alloc_value_set();
        let (size, is_written, is_constant, offset, def) = {
            let v = fd.vbank().get(vn).expect("newValueSet: stale varnode");
            (v.get_size(), v.is_written(), v.is_constant(), v.get_offset(), v.get_def())
        };
        let (op_code, num_params) = match def {
            Some(d) if is_written => {
                let o = fd.obank().get(d).expect("newValueSet: stale def op");
                (o.code(), o.num_input())
            }
            _ => (OpCode::CPUI_MAX, 0),
        };
        let node = &mut self.value_nodes[idx];
        node.vn = Some(vn);
        node.type_code = t_code;
        if t_code != 0 {
            node.op_code = OpCode::CPUI_MAX;
            node.num_params = 0;
            // Treat as offset of 0 relative to the special value.
            node.range.set_range_value(0, size);
            node.left_is_stable = true;
            node.right_is_stable = true;
        } else if is_written {
            node.op_code = op_code;
            if node.op_code == OpCode::CPUI_INDIRECT {
                // Treat CPUI_INDIRECT as CPUI_COPY.
                node.num_params = 1;
                node.op_code = OpCode::CPUI_COPY;
            } else {
                node.num_params = num_params;
            }
            node.left_is_stable = false;
            node.right_is_stable = false;
        } else if is_constant {
            node.op_code = OpCode::CPUI_MAX;
            node.num_params = 0;
            node.range.set_range_value(offset, size);
            node.left_is_stable = true;
            node.right_is_stable = true;
        } else {
            // Some other form of input.
            node.op_code = OpCode::CPUI_MAX;
            node.num_params = 0;
            node.type_code = 0;
            node.range.set_full(size);
            node.left_is_stable = false;
            node.right_is_stable = false;
        }
        self.value_set_map.insert(vn, idx);
    }

    /// Register a node index as a root (C++ `rootNodes.push_back`).
    pub fn push_root(&mut self, node: usize) {
        self.root_nodes.push(node);
    }

    /// The root node indices, in registration order (C++ `rootNodes`).
    pub fn root_nodes(&self) -> &[usize] {
        &self.root_nodes
    }

    /// Walk the computed iteration order (`orderPartition.startNode` then
    /// `next` links), returning node indices in order.  Used to verify the
    /// Bourdoncle ordering in tests.
    pub fn iteration_order(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut cur = self.order_partition.start_node;
        while let Some(idx) = cur {
            out.push(idx);
            cur = self.next[idx];
        }
        out
    }

    /// Return the partition-head record index for a node, if it is a component
    /// head (C++ `ValueSet::partHead != 0`).
    pub fn part_head_of(&self, node: usize) -> Option<usize> {
        self.part_head[node]
    }

    /// Prepend a vertex to a partition (C++ `partitionPrepend(ValueSet*,...)`,
    /// `rangeutil.hh:389`).
    fn partition_prepend_vertex(&mut self, vertex: usize, part: &mut Partition) {
        self.next[vertex] = part.start_node; // Attach new vertex to beginning of list
        part.start_node = Some(vertex); // Change the first value set to be the new vertex
        if part.stop_node.is_none() {
            part.stop_node = Some(vertex);
        }
    }

    /// Prepend a full Partition to the given Partition (C++
    /// `partitionPrepend(const Partition&,...)`, `rangeutil.hh:400`).
    fn partition_prepend_part(&mut self, head: &Partition, part: &mut Partition) {
        if let Some(head_stop) = head.stop_node {
            self.next[head_stop] = part.start_node;
        }
        part.start_node = head.start_node;
        if part.stop_node.is_none() {
            part.stop_node = head.stop_node;
        }
    }

    /// Save a Partition to permanent storage and set its head's `partHead`
    /// (C++ `partitionSurround`, `rangeutil.cc:1963`).
    fn partition_surround(&mut self, part: &Partition) {
        self.record_storage.push(part.clone());
        if let Some(start) = part.start_node {
            self.part_head[start] = Some(self.record_storage.len() - 1);
        }
    }

    /// Generate a partition component given its head (C++ `component`,
    /// `rangeutil.cc:1974`).
    fn component(&mut self, vertex: usize, graph: &dyn ValueSetGraph, part: &mut Partition) {
        let succs = graph.successors(vertex);
        for succ in succs {
            if self.value_nodes[succ].count == 0 {
                self.visit(succ, graph, part);
            }
        }
        self.partition_prepend_vertex(vertex, part);
        self.partition_surround(part);
    }

    /// Recursively walk the data-flow graph finding partitions (C++ `visit`,
    /// `rangeutil.cc:1991`).  Returns the head index of the current partition.
    fn visit(&mut self, vertex: usize, graph: &dyn ValueSetGraph, part: &mut Partition) -> int4 {
        self.node_stack.push(vertex);
        self.depth_first_index += 1;
        self.value_nodes[vertex].count = self.depth_first_index;
        let mut head = self.depth_first_index;
        let mut loop_ = false;
        let succs = graph.successors(vertex);
        for succ in succs {
            let min = if self.value_nodes[succ].count == 0 {
                self.visit(succ, graph, part)
            } else {
                self.value_nodes[succ].count
            };
            if min <= head {
                head = min;
                loop_ = true;
            }
        }
        if head == self.value_nodes[vertex].count {
            self.value_nodes[vertex].count = 0x7fff_ffff; // "infinity"
            let mut element = self.node_stack.pop().unwrap();
            if loop_ {
                while element != vertex {
                    self.value_nodes[element].count = 0;
                    element = self.node_stack.pop().unwrap();
                }
                let mut comp_part = Partition::new(); // empty partition
                self.component(vertex, graph, &mut comp_part);
                self.partition_prepend_part(&comp_part, part);
            } else {
                self.partition_prepend_vertex(vertex, part);
            }
        }
        head
    }

    /// Find the optimal order for iterating through the ValueSets (C++
    /// `establishTopologicalOrder`, `rangeutil.cc:2042`).
    ///
    /// Upstream allocates a stack-local `rootNode` (a real `ValueSet` with
    /// `vn==NULL`) and visits it; its successors are `rootNodes`.  We mirror
    /// that by appending a transient root `ValueSet` at index `value_nodes.len()`
    /// (so the graph's `successors(root)` must yield the root edges), running the
    /// standard [`ValueSetSolver::visit`], then stripping the root from the order
    /// list and popping it.  This is the Bourdoncle "Efficient chaotic iteration
    /// strategies with widenings" ordering; the resulting
    /// `order_partition`/`part_head`/`next` describe the nested iteration
    /// components exactly as upstream.
    pub fn establish_topological_order(&mut self, graph: &dyn ValueSetGraph) {
        for i in 0..self.value_nodes.len() {
            self.value_nodes[i].count = 0;
            self.next[i] = None;
            self.part_head[i] = None;
        }
        // Append the transient simulated root node.
        let root = self.value_nodes.len();
        self.value_nodes.push(ValueSet::new());
        self.next.push(None);
        self.part_head.push(None);

        self.depth_first_index = 0;
        let mut order = std::mem::take(&mut self.order_partition);
        self.visit(root, graph, &mut order);
        // Remove simulated root: orderPartition.startNode = startNode->next
        if let Some(start) = order.start_node {
            debug_assert_eq!(start, root);
            order.start_node = self.next[start];
        }
        self.order_partition = order;

        // Pop the transient root node (its arena slot must not survive; any
        // partHead recorded for the root would have referenced it as a head,
        // which cannot happen since nothing edges into the root).
        self.value_nodes.pop();
        self.next.pop();
        self.part_head.pop();
    }

    // --- IR-coupled pipeline ----------------------------------------------

    /// Look up the ValueSet bound to a Varnode (C++ `Varnode::getValueSet`).
    pub fn get_value_set(&self, vn: VarnodeId) -> Option<&ValueSet> {
        self.value_set_map.get(&vn).map(|&i| &self.value_nodes[i])
    }

    /// Get the ValueSetRead calculated for the given read op (C++
    /// `getValueSetRead`, `rangeutil.hh:325` — keyed by op instead of SeqNum).
    pub fn get_value_set_read(&self, op: OpId) -> Option<&ValueSetRead> {
        self.read_nodes.get(&op)
    }

    /// Build value sets for a data-flow system (C++ `establishValueSets`,
    /// `rangeutil.cc:2416`).
    ///
    /// Given a set of sinks, find all the Varnodes that flow directly into them
    /// and set up their initial ValueSet objects.  `reads` are add-on PcodeOps
    /// where the input ValueSet at the point of read is wanted; `stack_reg`
    /// (if given) is the stack-pointer input Varnode (tracked as a relative
    /// offset); `indirect_as_copy` treats CPUI_INDIRECT as a COPY.
    pub fn establish_value_sets(
        &mut self,
        fd: &Funcdata,
        sinks: &[VarnodeId],
        reads: &[OpId],
        stack_reg: Option<VarnodeId>,
        indirect_as_copy: bool,
    ) {
        let mut worklist: Vec<VarnodeId> = Vec::new();
        let mut work_pos = 0usize;
        if let Some(sr) = stack_reg {
            self.new_value_set(fd, sr, 1); // Establish stack pointer as special
            worklist.push(sr);
            work_pos += 1;
            let idx = self.value_set_map[&sr];
            self.root_nodes.push(idx);
        }
        for &vn in sinks {
            self.new_value_set(fd, vn, 0);
            worklist.push(vn);
        }
        while work_pos < worklist.len() {
            let vn = worklist[work_pos];
            work_pos += 1;
            let (is_written, is_constant, is_spacebase, def, vn_size) = {
                let v = match fd.vbank().get(vn) {
                    Some(v) => v,
                    None => continue,
                };
                (v.is_written(), v.is_constant(), v.is_spacebase(), v.get_def(), v.get_size())
            };
            if !is_written {
                if is_constant {
                    // Constant inputs to binary ops should not be treated as
                    // root nodes as they get picked up during iteration by the
                    // other input, except in the case of a PTRSUB from a
                    // spacebase constant.
                    let lone_inputs = fd
                        .lone_descend(vn)
                        .and_then(|op| fd.obank().get(op))
                        .map(|o| o.num_input());
                    if is_spacebase || lone_inputs == Some(1) {
                        let idx = self.value_set_map[&vn];
                        self.root_nodes.push(idx);
                    }
                } else {
                    let idx = self.value_set_map[&vn];
                    self.root_nodes.push(idx);
                }
                continue;
            }
            let op = match def {
                Some(d) => d,
                None => continue,
            };
            let opc = match fd.obank().get(op) {
                Some(o) => o.code(),
                None => continue,
            };
            // Distinguish ops where we can never predict an integer range.
            match opc {
                OpCode::CPUI_INDIRECT => {
                    let ind_store =
                        fd.obank().get(op).map(|o| o.is_indirect_store()).unwrap_or(false);
                    if indirect_as_copy || ind_store {
                        if let Some(in_vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                            if !self.value_set_map.contains_key(&in_vn) {
                                self.new_value_set(fd, in_vn, 0);
                                worklist.push(in_vn);
                            }
                        }
                    } else {
                        let idx = self.value_set_map[&vn];
                        self.value_nodes[idx].set_full(vn_size);
                        self.root_nodes.push(idx);
                    }
                }
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_LOAD
                | OpCode::CPUI_NEW
                | OpCode::CPUI_SEGMENTOP
                | OpCode::CPUI_CPOOLREF
                | OpCode::CPUI_FLOAT_ADD
                | OpCode::CPUI_FLOAT_DIV
                | OpCode::CPUI_FLOAT_MULT
                | OpCode::CPUI_FLOAT_SUB
                | OpCode::CPUI_FLOAT_NEG
                | OpCode::CPUI_FLOAT_ABS
                | OpCode::CPUI_FLOAT_SQRT
                | OpCode::CPUI_FLOAT_INT2FLOAT
                | OpCode::CPUI_FLOAT_FLOAT2FLOAT
                | OpCode::CPUI_FLOAT_TRUNC
                | OpCode::CPUI_FLOAT_CEIL
                | OpCode::CPUI_FLOAT_FLOOR
                | OpCode::CPUI_FLOAT_ROUND => {
                    let idx = self.value_set_map[&vn];
                    self.value_nodes[idx].set_full(vn_size);
                    self.root_nodes.push(idx);
                }
                _ => {
                    let num = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                    for i in 0..num {
                        let in_vn = match fd.obank().get(op).and_then(|o| o.get_in(i)) {
                            Some(v) => v,
                            None => continue,
                        };
                        let is_annotation =
                            fd.vbank().get(in_vn).map(|v| v.is_annotation()).unwrap_or(true);
                        if self.value_set_map.contains_key(&in_vn) || is_annotation {
                            continue;
                        }
                        self.new_value_set(fd, in_vn, 0);
                        worklist.push(in_vn);
                    }
                }
            }
        }
        // Set up read sites (the C++ marks the read ops for the equation
        // generation stage; the mark set lives here and dies after it).
        let mut read_marked: HashSet<OpId> = HashSet::new();
        for &op in reads {
            let num = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
            for slot in 0..num {
                let vn = match fd.obank().get(op).and_then(|o| o.get_in(slot)) {
                    Some(v) => v,
                    None => continue,
                };
                if self.value_set_map.contains_key(&vn) {
                    let mut read = ValueSetRead::new();
                    read.set_pcode_op(op, slot);
                    self.read_nodes.insert(op, read);
                    read_marked.insert(op);
                    break; // Only 1 read allowed
                }
            }
        }
        self.generate_constraints(fd, &worklist, reads, &read_marked);
        // (read-op marks are cleared by dropping `read_marked`)

        let graph = IrValueSetGraph {
            fd,
            vn_of: self.value_nodes.iter().map(|n| n.vn).collect(),
            map: self.value_set_map.clone(),
            roots: self.root_nodes.clone(),
        };
        self.establish_topological_order(&graph);
        // (worklist Varnode marks die with the solver; the ValueSet
        //  back-pointers — the map — stay live for solve, as upstream.)
    }

    /// Generate an equation given a `true` constraint and the (output-node,
    /// read-op) it affects (C++ `generateTrueEquation`, `rangeutil.cc:2066`).
    fn generate_true_equation(
        &mut self,
        node: Option<usize>,
        op: OpId,
        slot: int4,
        type_code: int4,
        range: &CircleRange,
    ) {
        match node {
            Some(i) => self.value_nodes[i].add_equation(slot, type_code, range.clone()),
            // Special read site.
            None => self.read_nodes.entry(op).or_default().add_equation(
                slot,
                type_code,
                range.clone(),
            ),
        }
    }

    /// Generate the complementary equation given a `true` constraint (C++
    /// `generateFalseEquation`, `rangeutil.cc:2084`).
    fn generate_false_equation(
        &mut self,
        node: Option<usize>,
        op: OpId,
        slot: int4,
        type_code: int4,
        range: &CircleRange,
    ) {
        let mut false_range = range.clone();
        false_range.invert();
        match node {
            Some(i) => self.value_nodes[i].add_equation(slot, type_code, false_range),
            // Special read site.
            None => self.read_nodes.entry(op).or_default().add_equation(
                slot,
                type_code,
                false_range,
            ),
        }
    }

    /// Look for PcodeOps where the given constraint range applies and
    /// instantiate an equation (C++ `applyConstraints`, `rangeutil.cc:2105`).
    fn apply_constraints(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        type_code: int4,
        range: &CircleRange,
        cbranch: OpId,
        read_marked: &HashSet<OpId>,
    ) {
        let split_point = match fd.obank().get(cbranch).and_then(|o| o.get_parent()) {
            Some(b) => b,
            None => return,
        };
        let bb = fd.bblocks_ref();
        if bb.block(split_point).size_out() != 2 {
            return; // defensive: C++ guarantees a CBRANCH block has 2 out-edges
        }
        let flip = fd.obank().get(cbranch).map(|o| o.is_boolean_flip()).unwrap_or(false);
        let (true_block, false_block) = if flip {
            (bb.block(split_point).get_false_out(), bb.block(split_point).get_true_out())
        } else {
            (bb.block(split_point).get_true_out(), bb.block(split_point).get_false_out())
        };
        // Check if the only path to trueBlock or falseBlock is via a splitPoint
        // out-edge induced by the condition.
        let true_is_restricted = bb.restricted_by_conditional(true_block, split_point);
        let false_is_restricted = bb.restricted_by_conditional(false_block, split_point);

        if fd.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
            if let Some(&vidx) = self.value_set_map.get(&vn) {
                if self.value_nodes[vidx].op_code == OpCode::CPUI_MULTIEQUAL {
                    // Leave landmark for widening.
                    self.value_nodes[vidx].add_landmark(type_code, range.clone());
                }
            }
        }
        let descend: Vec<OpId> =
            fd.vbank().get(vn).map(|v| v.descend_iter().collect()).unwrap_or_default();
        for op in descend {
            // The output node in the system, or None for a special read site.
            let mut out_node: Option<usize> = None;
            if !read_marked.contains(&op) {
                // Not a special read site: make sure there is a Varnode in the system.
                let out_vn = match fd.obank().get(op).and_then(|o| o.get_out()) {
                    Some(v) => v,
                    None => continue,
                };
                match self.value_set_map.get(&out_vn) {
                    Some(&i) => out_node = Some(i),
                    None => continue,
                }
            }
            let mut cur_block = match fd.obank().get(op).and_then(|o| o.get_parent()) {
                Some(b) => b,
                None => continue,
            };
            let slot = match fd.obank().get(op) {
                Some(o) => o.get_slot(vn),
                None => continue,
            };
            if slot < 0 {
                continue; // defensive: C++ getSlot assumes membership
            }
            let opc = fd.obank().get(op).map(|o| o.code());
            if opc == Some(OpCode::CPUI_MULTIEQUAL) {
                if cur_block == true_block {
                    // If it's possible that both the true and false edges reach
                    // trueBlock then the only input we can restrict is a
                    // MULTIEQUAL input along the exact true edge.
                    if true_is_restricted || bb.block(true_block).get_in(slot) == split_point {
                        self.generate_true_equation(out_node, op, slot, type_code, range);
                    }
                    continue;
                } else if cur_block == false_block {
                    if false_is_restricted || bb.block(false_block).get_in(slot) == split_point {
                        self.generate_false_equation(out_node, op, slot, type_code, range);
                    }
                    continue;
                } else {
                    // A MULTIEQUAL input is really only from one in-block.
                    cur_block = bb.block(cur_block).get_in(slot);
                }
            }
            loop {
                if cur_block == true_block {
                    if true_is_restricted {
                        self.generate_true_equation(out_node, op, slot, type_code, range);
                    }
                    break;
                } else if cur_block == false_block {
                    if false_is_restricted {
                        self.generate_false_equation(out_node, op, slot, type_code, range);
                    }
                    break;
                } else if cur_block == split_point {
                    break;
                }
                match bb.block(cur_block).get_immed_dom() {
                    Some(d) => cur_block = d,
                    None => break,
                }
            }
        }
    }

    /// Lift a range along a known path to a system Varnode and apply it at
    /// reads (C++ `constraintsFromPath`, `rangeutil.cc:2185`).
    #[allow(clippy::too_many_arguments)]
    fn constraints_from_path(
        &mut self,
        fd: &Funcdata,
        type_code: int4,
        mut lift: CircleRange,
        mut start_vn: VarnodeId,
        end_vn: VarnodeId,
        cbranch: OpId,
        read_marked: &HashSet<OpId>,
    ) {
        while start_vn != end_vn {
            let def = match fd.vbank().get(start_vn).and_then(|v| v.get_def()) {
                Some(d) => d,
                None => return,
            };
            match crate::jumptable::circlerange_pull_back(fd, &mut lift, def, false) {
                Some(v) => start_vn = v,
                None => return, // Couldn't pull all the way back to our value set
            }
        }
        let mut end_vn = end_vn;
        loop {
            self.apply_constraints(fd, end_vn, type_code, &lift, cbranch, read_marked);
            let (is_written, def) = match fd.vbank().get(end_vn) {
                Some(v) => (v.is_written(), v.get_def()),
                None => break,
            };
            if !is_written {
                break;
            }
            let op = match def {
                Some(d) => d,
                None => break,
            };
            let stop = fd.obank().get(op).map(|o| o.is_call() || o.is_marker()).unwrap_or(true);
            if stop {
                break;
            }
            match crate::jumptable::circlerange_pull_back(fd, &mut lift, op, false) {
                Some(v) => end_vn = v,
                None => break,
            }
            if !self.value_set_map.contains_key(&end_vn) {
                break;
            }
        }
    }

    /// Generate constraints from a conditional branch (C++
    /// `constraintsFromCBranch`, `rangeutil.cc:2210`).
    fn constraints_from_cbranch(
        &mut self,
        fd: &Funcdata,
        cbranch: OpId,
        read_marked: &HashSet<OpId>,
    ) {
        // Get the Varnode deciding the condition.
        let mut vn = match fd.obank().get(cbranch).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
        while !self.value_set_map.contains_key(&vn) {
            let (is_written, def) = match fd.vbank().get(vn) {
                Some(v) => (v.is_written(), v.get_def()),
                None => return,
            };
            if !is_written {
                break;
            }
            let op = match def {
                Some(d) => d,
                None => break,
            };
            let (is_call, is_marker, num) = match fd.obank().get(op) {
                Some(o) => (o.is_call(), o.is_marker(), o.num_input()),
                None => break,
            };
            if is_call || is_marker {
                break;
            }
            if num == 0 || num > 2 {
                break;
            }
            vn = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => return,
            };
            if num == 2 {
                let vn_const = fd.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false);
                if vn_const {
                    vn = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
                        Some(v) => v,
                        None => return,
                    };
                } else {
                    let other_const = fd
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(1))
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.is_constant())
                        .unwrap_or(false);
                    if !other_const {
                        // Both inputs are non-constant.
                        self.generate_relative_constraint(fd, op, cbranch, read_marked);
                        return;
                    }
                    // vn is non-constant, other input is constant.
                }
            }
        }
        if self.value_set_map.contains_key(&vn) {
            let lift = CircleRange::new_bool(true);
            let start_vn = match fd.obank().get(cbranch).and_then(|o| o.get_in(1)) {
                Some(v) => v,
                None => return,
            };
            self.constraints_from_path(fd, 0, lift, start_vn, vn, cbranch, read_marked);
        }
    }

    /// Look for constraints on Varnodes in the system arising from branch
    /// conditions (C++ `generateConstraints`, `rangeutil.cc:2248`).
    fn generate_constraints(
        &mut self,
        fd: &Funcdata,
        worklist: &[VarnodeId],
        reads: &[OpId],
        read_marked: &HashSet<OpId>,
    ) {
        let mut block_list: Vec<BlockId> = Vec::new();
        let mut block_marks: HashSet<BlockId> = HashSet::new();
        // Collect all blocks that contain a system op (input) or dominate a
        // container.
        for &vn in worklist {
            let op = match fd.vbank().get(vn).and_then(|v| v.get_def()) {
                Some(o) => o,
                None => continue,
            };
            let bl = match fd.obank().get(op).and_then(|o| o.get_parent()) {
                Some(b) => b,
                None => continue,
            };
            let is_multi = fd.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_MULTIEQUAL);
            if is_multi {
                let size_in = fd.bblocks_ref().block(bl).size_in();
                for j in 0..size_in {
                    let mut cur = Some(fd.bblocks_ref().block(bl).get_in(j));
                    while let Some(c) = cur {
                        if block_marks.contains(&c) {
                            break;
                        }
                        block_marks.insert(c);
                        block_list.push(c);
                        cur = fd.bblocks_ref().block(c).get_immed_dom();
                    }
                }
            } else {
                let mut cur = Some(bl);
                while let Some(c) = cur {
                    if block_marks.contains(&c) {
                        break;
                    }
                    block_marks.insert(c);
                    block_list.push(c);
                    cur = fd.bblocks_ref().block(c).get_immed_dom();
                }
            }
        }
        for &rop in reads {
            let mut cur = fd.obank().get(rop).and_then(|o| o.get_parent());
            while let Some(c) = cur {
                if block_marks.contains(&c) {
                    break;
                }
                block_marks.insert(c);
                block_list.push(c);
                cur = fd.bblocks_ref().block(c).get_immed_dom();
            }
        }
        // (C++ clears the block marks here; the fresh set below replaces them.)
        let mut final_marks: HashSet<BlockId> = HashSet::new();
        // Now go through input blocks to the previously calculated blocks.
        for &bl in &block_list {
            let size_in = fd.bblocks_ref().block(bl).size_in();
            for j in 0..size_in {
                let split_point = fd.bblocks_ref().block(bl).get_in(j);
                if final_marks.contains(&split_point) {
                    continue;
                }
                if fd.bblocks_ref().block(split_point).size_out() != 2 {
                    continue;
                }
                let last_op = match fd.bblocks_ref().block(split_point).kind() {
                    crate::block::BlockKind::Basic(bd) => bd.op_tail,
                    _ => None,
                };
                if let Some(lo) = last_op {
                    if fd.obank().get(lo).map(|o| o.code()) == Some(OpCode::CPUI_CBRANCH) {
                        final_marks.insert(split_point);
                        // Try to generate constraints from this splitPoint.
                        self.constraints_from_cbranch(fd, lo, read_marked);
                    }
                }
            }
        }
    }

    /// Check if the given Varnode is a *relative* constant — produced from the
    /// system's base register by a straight line of COPYs / constant INT_ADDs
    /// (C++ `checkRelativeConstant`, `rangeutil.cc:2316`).  Returns the base
    /// register's type code and the additive value.
    fn check_relative_constant(&self, fd: &Funcdata, mut vn: VarnodeId) -> Option<(int4, uintb)> {
        let mut value: uintb = 0;
        loop {
            if let Some(&idx) = self.value_set_map.get(&vn) {
                let tc = self.value_nodes[idx].type_code;
                if tc != 0 {
                    return Some((tc, value));
                }
            }
            let v = fd.vbank().get(vn)?;
            if !v.is_written() {
                return None;
            }
            let op = v.get_def()?;
            let o = fd.obank().get(op)?;
            let opc = o.code();
            if opc == OpCode::CPUI_COPY || opc == OpCode::CPUI_INDIRECT {
                vn = o.get_in(0)?;
            } else if opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRSUB {
                let const_vn = o.get_in(1)?;
                let cv = fd.vbank().get(const_vn)?;
                if !cv.is_constant() {
                    return None;
                }
                value = value.wadd(cv.get_offset()) & calc_mask(cv.get_size());
                vn = o.get_in(0)?;
            } else {
                return None;
            }
        }
    }

    /// Try to generate a constraint relative to the system's base register
    /// from a two-non-constant comparison (C++ `generateRelativeConstraint`,
    /// `rangeutil.cc:2351`).
    fn generate_relative_constraint(
        &mut self,
        fd: &Funcdata,
        comp_op: OpId,
        cbranch: OpId,
        read_marked: &HashSet<OpId>,
    ) {
        let mut opc = match fd.obank().get(comp_op) {
            Some(o) => o.code(),
            None => return,
        };
        match opc {
            // Treat unsigned pointer comparisons as signed relative to the base
            // register.
            OpCode::CPUI_INT_LESS => opc = OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_LESSEQUAL => opc = OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL => {}
            _ => return,
        }
        let in_vn0 = match fd.obank().get(comp_op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return,
        };
        let in_vn1 = match fd.obank().get(comp_op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
        let mut lift = CircleRange::new_bool(true);
        let type_code: int4;
        let vn: VarnodeId;
        if let Some((tc, value)) = self.check_relative_constant(fd, in_vn0) {
            vn = in_vn1;
            let size = fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
            if !lift.pull_back_binary(opc, value, 1, size, 1) {
                return;
            }
            type_code = tc;
        } else if let Some((tc, value)) = self.check_relative_constant(fd, in_vn1) {
            vn = in_vn0;
            let size = fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
            if !lift.pull_back_binary(opc, value, 0, size, 1) {
                return;
            }
            type_code = tc;
        } else {
            return; // Neither side looks like a relative constant
        }

        let mut end_vn = vn;
        while !self.value_set_map.contains_key(&end_vn) {
            let (is_written, def) = match fd.vbank().get(end_vn) {
                Some(v) => (v.is_written(), v.get_def()),
                None => return,
            };
            if !is_written {
                return;
            }
            let op = match def {
                Some(d) => d,
                None => return,
            };
            let o = match fd.obank().get(op) {
                Some(o) => o,
                None => return,
            };
            let opc2 = o.code();
            if opc2 == OpCode::CPUI_COPY || opc2 == OpCode::CPUI_PTRSUB {
                end_vn = match o.get_in(0) {
                    Some(v) => v,
                    None => return,
                };
            } else if opc2 == OpCode::CPUI_INT_ADD {
                // Can pull-back through INT_ADD if second param is constant.
                let second_const = o
                    .get_in(1)
                    .and_then(|v| fd.vbank().get(v))
                    .map(|v| v.is_constant())
                    .unwrap_or(false);
                if !second_const {
                    return;
                }
                end_vn = match o.get_in(0) {
                    Some(v) => v,
                    None => return,
                };
            } else {
                return;
            }
        }
        self.constraints_from_path(fd, type_code, lift, vn, end_vn, cbranch, read_marked);
    }

    /// Examine the input value sets and decide if this node's set is relative
    /// (C++ `ValueSet::computeTypeCode`, `rangeutil.cc:1567`).  Returns `true`
    /// on an indeterminate combination.
    fn compute_type_code(&mut self, fd: &Funcdata, node: usize) -> bool {
        let mut rel_count = 0;
        let mut last_type_code = 0;
        let vn = match self.value_nodes[node].vn {
            Some(v) => v,
            None => return true,
        };
        let op = match fd.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(o) => o,
            None => return true,
        };
        let num_params = self.value_nodes[node].num_params;
        for i in 0..num_params {
            let in_vn = match fd.obank().get(op).and_then(|o| o.get_in(i)) {
                Some(v) => v,
                None => return true, // defensive: input missing => indeterminate
            };
            match self.value_set_map.get(&in_vn) {
                Some(&idx) => {
                    let tc = self.value_nodes[idx].type_code;
                    if tc != 0 {
                        rel_count += 1;
                        last_type_code = tc;
                    }
                }
                // defensive: an input outside the system (upstream would deref a
                // null back-pointer, which its establish invariants preclude).
                None => return true,
            }
        }
        if rel_count == 0 {
            self.value_nodes[node].type_code = 0;
            return false;
        }
        // Only certain operations can propagate a relative value set.
        match self.value_nodes[node].op_code {
            OpCode::CPUI_PTRSUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB => {
                if rel_count == 1 {
                    self.value_nodes[node].type_code = last_type_code;
                } else {
                    return true;
                }
            }
            OpCode::CPUI_CAST
            | OpCode::CPUI_COPY
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_MULTIEQUAL => {
                self.value_nodes[node].type_code = last_type_code;
            }
            _ => return true,
        }
        false
    }

    /// Recalculate one node by pushing its inputs' value sets forward through
    /// the defining operator (C++ `ValueSet::iterate`, `rangeutil.cc:1611`).
    /// Returns `true` if the node's value set changed.
    fn iterate_node(&mut self, fd: &Funcdata, node: usize, widener: &dyn Widener) -> bool {
        let vn = match self.value_nodes[node].vn {
            Some(v) => v,
            None => return false,
        };
        let (is_written, def, vn_size) = match fd.vbank().get(vn) {
            Some(v) => (v.is_written(), v.get_def(), v.get_size()),
            None => return false,
        };
        if !is_written {
            return false;
        }
        if widener.check_freeze(&self.value_nodes[node]) {
            return false;
        }
        if self.value_nodes[node].count == 0 && self.compute_type_code(fd, node) {
            self.value_nodes[node].set_full(vn_size);
            return true;
        }
        self.value_nodes[node].count += 1; // Count this iteration
        let op = match def {
            Some(d) => d,
            None => return false,
        };
        let op_code = self.value_nodes[node].op_code;
        let num_params = self.value_nodes[node].num_params;

        // Snapshot the input value sets (range, stability, size) up front; the
        // C++ reads them through live pointers, but nothing below mutates any
        // input node, so the snapshot is equivalent.
        let mut inputs: Vec<(CircleRange, bool, bool, int4)> = Vec::new();
        for i in 0..num_params {
            let in_vn = match fd.obank().get(op).and_then(|o| o.get_in(i)) {
                Some(v) => v,
                None => return false,
            };
            let idx = match self.value_set_map.get(&in_vn) {
                Some(&idx) => idx,
                None => {
                    // defensive: input outside the system (see compute_type_code)
                    self.value_nodes[node].set_full(vn_size);
                    return true;
                }
            };
            let size = fd.vbank().get(in_vn).map(|v| v.get_size()).unwrap_or(0);
            let n = &self.value_nodes[idx];
            inputs.push((n.range.clone(), n.left_is_stable, n.right_is_stable, size));
        }

        let mut res = CircleRange::new_empty();
        let mut eq_pos: usize = 0;
        if op_code == OpCode::CPUI_MULTIEQUAL {
            let mut pieces;
            for i in 0..num_params {
                let in_range = &inputs[i as usize].0;
                if self.value_nodes[node].does_equation_apply(eq_pos as int4, i) {
                    let mut range_copy = in_range.clone();
                    let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                    if 0 != range_copy.intersect(&eq_range) {
                        range_copy = eq_range;
                    }
                    pieces = res.circle_union(&range_copy);
                    eq_pos += 1; // Equation was used
                } else {
                    pieces = res.circle_union(in_range);
                }
                if pieces == 2 {
                    // Could not get clean union, force it.
                    if res.minimal_container(in_range, VALUESET_MAX_STEP) {
                        break;
                    }
                }
            }
            let prev_range = self.value_nodes[node].range.clone();
            // Union with the previous iteration's set.
            if 0 != res.circle_union(&prev_range) {
                res.minimal_container(&prev_range, VALUESET_MAX_STEP);
            }
            if !prev_range.is_empty() && !res.is_empty() {
                self.value_nodes[node].left_is_stable = prev_range.get_min() == res.get_min();
                self.value_nodes[node].right_is_stable = prev_range.get_end() == res.get_end();
            }
        } else if num_params == 1 {
            let (in1_range, in1_left, in1_right, in1_size) = inputs[0].clone();
            if self.value_nodes[node].does_equation_apply(eq_pos as int4, 0) {
                let mut range_copy = in1_range.clone();
                let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                if 0 != range_copy.intersect(&eq_range) {
                    range_copy = eq_range;
                }
                if !res.push_forward_unary(op_code, &range_copy, in1_size, vn_size) {
                    self.value_nodes[node].set_full(vn_size);
                    return true;
                }
                // (eq_pos += 1 would follow upstream; it is dead past this point)
            } else if !res.push_forward_unary(op_code, &in1_range, in1_size, vn_size) {
                self.value_nodes[node].set_full(vn_size);
                return true;
            }
            self.value_nodes[node].left_is_stable = in1_left;
            self.value_nodes[node].right_is_stable = in1_right;
        } else if num_params == 2 {
            let (in1_range, in1_left, in1_right, in1_size) = inputs[0].clone();
            let (in2_range, in2_left, in2_right, _in2_size) = inputs[1].clone();
            if self.value_nodes[node].equations.is_empty() {
                if !res.push_forward_binary(
                    op_code,
                    &in1_range,
                    &in2_range,
                    in1_size,
                    vn_size,
                    VALUESET_MAX_STEP,
                ) {
                    self.value_nodes[node].set_full(vn_size);
                    return true;
                }
            } else {
                let mut range1 = in1_range;
                let mut range2 = in2_range;
                if self.value_nodes[node].does_equation_apply(eq_pos as int4, 0) {
                    let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                    if 0 != range1.intersect(&eq_range) {
                        range1 = eq_range;
                    }
                    eq_pos += 1;
                }
                if self.value_nodes[node].does_equation_apply(eq_pos as int4, 1) {
                    let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                    if 0 != range2.intersect(&eq_range) {
                        range2 = eq_range;
                    }
                }
                if !res.push_forward_binary(
                    op_code,
                    &range1,
                    &range2,
                    in1_size,
                    vn_size,
                    VALUESET_MAX_STEP,
                ) {
                    self.value_nodes[node].set_full(vn_size);
                    return true;
                }
            }
            self.value_nodes[node].left_is_stable = in1_left && in2_left;
            self.value_nodes[node].right_is_stable = in1_right && in2_right;
        } else if num_params == 3 {
            let (in1_range, in1_left, in1_right, in1_size) = inputs[0].clone();
            let (in2_range, in2_left, in2_right, _in2_size) = inputs[1].clone();
            let (in3_range, _, _, _) = inputs[2].clone();
            let mut range1 = in1_range;
            let mut range2 = in2_range;
            if self.value_nodes[node].does_equation_apply(eq_pos as int4, 0) {
                let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                if 0 != range1.intersect(&eq_range) {
                    range1 = eq_range;
                }
                eq_pos += 1;
            }
            if self.value_nodes[node].does_equation_apply(eq_pos as int4, 1) {
                let eq_range = self.value_nodes[node].equations[eq_pos].range.clone();
                if 0 != range2.intersect(&eq_range) {
                    range2 = eq_range;
                }
            }
            if !res.push_forward_trinary(
                op_code,
                &range1,
                &range2,
                &in3_range,
                in1_size,
                vn_size,
                VALUESET_MAX_STEP,
            ) {
                self.value_nodes[node].set_full(vn_size);
                return true;
            }
            self.value_nodes[node].left_is_stable = in1_left && in2_left;
            self.value_nodes[node].right_is_stable = in1_right && in2_right;
        } else {
            return false; // No way to change this value set
        }

        if res.equals(&self.value_nodes[node].range) {
            return false;
        }
        if self.part_head[node].is_some() {
            let snapshot = self.value_nodes[node].clone();
            let mut cur_range = snapshot.range.clone();
            if !widener.do_widening(&snapshot, &mut cur_range, &res) {
                self.value_nodes[node].set_full(vn_size);
            } else {
                self.value_nodes[node].range = cur_range;
            }
        } else {
            self.value_nodes[node].range = res;
        }
        true
    }

    /// Iterate the ValueSet system until it stabilizes (C++ `solve`,
    /// `rangeutil.cc:2524`).
    ///
    /// The ValueSets are recalculated in the established topological ordering,
    /// with looping at various levels until a fixed point is reached.  `max`
    /// bounds the total number of node iterations; `widener` selects the
    /// strategy for accelerating stabilization.
    pub fn solve(&mut self, fd: &Funcdata, max: int4, widener: &dyn Widener) {
        self.max_iterations = max;
        self.num_iterations = 0;
        for n in self.value_nodes.iter_mut() {
            n.count = 0;
        }

        let mut component_stack: Vec<usize> = Vec::new(); // record_storage indices
        let mut cur_component: Option<usize> = None;
        let mut cur_set: Option<usize> = self.order_partition.start_node;

        while let Some(cur) = cur_set {
            self.num_iterations += 1;
            if self.num_iterations > self.max_iterations {
                break; // Quit if max iterations exceeded
            }
            if let Some(ph) = self.part_head[cur] {
                if Some(ph) != cur_component {
                    component_stack.push(ph);
                    cur_component = Some(ph);
                    self.record_storage[ph].is_dirty = false;
                    // Reset component counter upon entry.
                    if let Some(start) = self.record_storage[ph].start_node {
                        self.value_nodes[start].count =
                            widener.determine_iteration_reset(&self.value_nodes[start]);
                    }
                }
            }
            if let Some(comp) = cur_component {
                if self.iterate_node(fd, cur, widener) {
                    self.record_storage[comp].is_dirty = true;
                }
                if self.record_storage[comp].stop_node != Some(cur) {
                    cur_set = self.next[cur];
                } else {
                    loop {
                        let top = *component_stack.last().expect("solve: component stack head");
                        if self.record_storage[top].is_dirty {
                            self.record_storage[top].is_dirty = false;
                            cur_set = self.record_storage[top].start_node;
                            if component_stack.len() > 1 {
                                // Mark parent as dirty if we are restarting a
                                // dirty child.
                                let parent = component_stack[component_stack.len() - 2];
                                self.record_storage[parent].is_dirty = true;
                            }
                            break;
                        }

                        component_stack.pop();
                        if component_stack.is_empty() {
                            cur_component = None;
                            cur_set = self.next[cur];
                            break;
                        }
                        let new_top = *component_stack.last().expect("solve: new stack head");
                        cur_component = Some(new_top);
                        if self.record_storage[new_top].stop_node != Some(cur) {
                            cur_set = self.next[cur];
                            break;
                        }
                    }
                }
            } else {
                self.iterate_node(fd, cur, widener);
                cur_set = self.next[cur];
            }
        }
        // Calculate any follow-on value sets (C++ `ValueSetRead::compute`).
        let read_ops: Vec<OpId> = self.read_nodes.keys().copied().collect();
        for rop in read_ops {
            let slot = self.read_nodes[&rop].slot;
            let in_vn = fd.obank().get(rop).and_then(|o| o.get_in(slot));
            if let Some(vn) = in_vn {
                if let Some(&idx) = self.value_set_map.get(&vn) {
                    let node = self.value_nodes[idx].clone();
                    if let Some(read) = self.read_nodes.get_mut(&rop) {
                        read.compute_from(&node);
                    }
                }
            }
        }
    }
}

/// The live-IR edge source for the topological-ordering walk (C++
/// `ValueSetSolver::ValueSetEdge`, `rangeutil.cc:1910`).
///
/// Successors of a node are the ValueSets attached to the outputs of the
/// descendant ops of the node's Varnode — outputs outside the system are
/// skipped, matching the C++ `outVn->isMark()` test — and the simulated root
/// node (the index one past the arena) yields the root nodes.
struct IrValueSetGraph<'a> {
    fd: &'a Funcdata,
    /// Node index -> bound Varnode (snapshot of the arena's `vn` fields).
    vn_of: Vec<Option<VarnodeId>>,
    /// The Varnode -> node-index binding (snapshot of `value_set_map`).
    map: HashMap<VarnodeId, usize>,
    /// Root node indices, yielded by the simulated root.
    roots: Vec<usize>,
}

impl ValueSetGraph for IrValueSetGraph<'_> {
    fn successors(&self, node: usize) -> Vec<usize> {
        if node >= self.vn_of.len() {
            return self.roots.clone(); // The simulated root
        }
        let vn = match self.vn_of[node] {
            Some(v) => v,
            None => return Vec::new(),
        };
        let mut out = Vec::new();
        let descend: Vec<OpId> = match self.fd.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => return out,
        };
        for op in descend {
            if let Some(o) = self.fd.obank().get(op) {
                if let Some(out_vn) = o.get_out() {
                    if let Some(&idx) = self.map.get(&out_vn) {
                        out.push(idx);
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests;
