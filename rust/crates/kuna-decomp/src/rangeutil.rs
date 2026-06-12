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
//! # The value-set solver layer (IR-coupled; SEAM)
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
//!     constraint generation, `establishValueSets`, `solve`) requires a
//!     `Varnode -> ValueSet` back-pointer field on `Varnode` plus `FlowBlock`
//!     dominance / `restrictedByConditional` / `getImmedDom` and the
//!     `OpBehavior` emulation surface.  None of those are exposed by the W3–W5
//!     IR surface yet (and `Varnode` is not a file this item owns, so the
//!     back-pointer cannot be added here).  The faithful **Bourdoncle
//!     weak-topological-ordering** walk (`visit`/`component`/
//!     `establishTopologicalOrder`/`solve`) is therefore ported against an
//!     injected edge abstraction ([`ValueSetGraph`]) so the iteration *order*
//!     and partition structure are exact and self-tested, while the methods
//!     that bind a `ValueSet` to a live `Varnode`/`PcodeOp` (`set_varnode`,
//!     `iterate`'s push-forward, all constraint generation,
//!     `establish_value_sets`) are `// SEAM(W5/W7-IR)` `Err` shells recorded as
//!     losses.

use kuna_base::address::{
    bit_transitions, calc_mask, count_leading_zeros, leastsigbit_set, mostsigbit_set, sign_extend,
    sign_extend_sized,
};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, intb, uint4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

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
                    // if ((intb)right < (intb)left)
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
                    val_left = val_right + 1; // Min negative
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
// The graph machinery is IR-coupled (Varnode<->ValueSet back-pointer, FlowBlock
// dominance, OpBehavior emulation), none of which the W3-W5 IR surface exposes
// and none of which lives in a file this item owns.  The structures below carry
// the lattice state faithfully (so the Widener trait above operates on a real
// ValueSet), and the methods that bind a ValueSet to a live Varnode/PcodeOp or
// run constraint generation are SEAM(W5/W7-IR) shells (recorded as losses).
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
/// The graph fields (`vn`, `next`, `partHead`) that the C++ uses as raw
/// pointers into the IR / partition list are not modeled here; this struct
/// holds the lattice state (`type_code`, `count`, stability flags, `range`,
/// `equations`) that the [`Widener`] strategy reads.  Binding to a live Varnode
/// is a SEAM (see module docs).
// `op_code` is set faithfully by the seamed `set_varnode`/`iterate` (which read
// it to dispatch the per-opcode push-forward); dead until that IR binding lands.
#[allow(dead_code)]
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

    /// Mark value set as possibly containing any value (C++ `setFull`).
    ///
    /// SEAM(W5/W7-IR): upstream reads `vn->getSize()` for the range size; with
    /// no bound Varnode the caller must supply the size.
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
/// Holds the value set at a specific read (PcodeOp + slot).  The `op`/`slot`
/// binding and `compute()` (which reads `vn->getValueSet()`) are SEAM; the
/// `addEquation`/range storage is faithful.
#[derive(Debug, Clone)]
pub struct ValueSetRead {
    type_code: int4,
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
    pub fn set_pcode_op(&mut self, slt: int4) {
        self.type_code = 0;
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

    /// Compute this value set from the bound Varnode's value set
    /// (C++ `compute`, `rangeutil.cc:1804`).
    ///
    /// SEAM(W5/W7-IR): the upstream method reads `op->getIn(slot)->getValueSet()`
    /// — the Varnode->ValueSet back-pointer that the IR surface does not expose.
    /// The constraint-application arithmetic is transcribed in
    /// [`ValueSetRead::compute_from`].
    pub fn compute(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "ValueSetRead::compute: needs Varnode->ValueSet back-pointer (W5/W7 IR seam)",
        ))
    }

    /// The IR-independent core of [`ValueSetRead::compute`]: given the bound
    /// Varnode's value set fields, apply the equation constraint.  Transcribed
    /// faithfully so the solver can call it once a back-pointer exists.
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
/// SEAM(W5/W7-IR): upstream walks the descendant PcodeOps of a node's Varnode
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
/// The full establish/solve pipeline is IR-coupled (see module docs); what is
/// ported faithfully here is the **Bourdoncle weak-topological-ordering** walk
/// (`visit`/`component`/`establish_topological_order`) over an injected
/// [`ValueSetGraph`], plus the partition-prepend/surround bookkeeping and the
/// `solve` driving loop's *structure*.  `establish_value_sets`, constraint
/// generation, and the push-forward `iterate` are SEAM shells (losses).
// `root_nodes`/`max_iterations` are consumed by the seamed
// `establish_value_sets`/`solve` (which need IR binding); dead until then.
#[allow(dead_code)]
pub struct ValueSetSolver {
    /// Storage for all the current value sets (C++ `valueNodes`).
    value_nodes: Vec<ValueSet>,
    /// `next` pointer of each node, as an index into `value_nodes` (C++
    /// `ValueSet::next`).  `None` terminates the iteration list.
    next: Vec<Option<usize>>,
    /// Partition head index of each node, or `None` (C++ `ValueSet::partHead`,
    /// here indexing into `record_storage`).
    part_head: Vec<Option<usize>>,
    /// Value sets in iteration order (C++ `orderPartition`).
    order_partition: Partition,
    /// Storage for the Partitions establishing components (C++ `recordStorage`).
    record_storage: Vec<Partition>,
    /// Values treated as inputs (C++ `rootNodes`) — node indices.
    root_nodes: Vec<usize>,
    /// Stack used to generate the topological ordering (C++ `nodeStack`).
    node_stack: Vec<usize>,
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
            order_partition: Partition::new(),
            record_storage: Vec::new(),
            root_nodes: Vec::new(),
            node_stack: Vec::new(),
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

    /// Allocate storage for a new ValueSet, returning its arena index
    /// (mirrors C++ `newValueSet`, sans the IR `setVarnode` binding which is a
    /// seam).  Used by the topological-ordering tests to build a graph.
    pub fn new_value_set(&mut self) -> usize {
        self.value_nodes.push(ValueSet::new());
        self.next.push(None);
        self.part_head.push(None);
        self.value_nodes.len() - 1
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

    // --- IR-coupled pipeline (SEAM) --------------------------------------

    /// Build value sets for a data-flow system (C++ `establishValueSets`,
    /// `rangeutil.cc:2416`).
    ///
    /// SEAM(W5/W7-IR): needs the Varnode->ValueSet back-pointer, descendant-op
    /// walking, `FlowBlock` dominance, and the constraint-generation pipeline.
    pub fn establish_value_sets(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "ValueSetSolver::establishValueSets: needs IR Varnode/PcodeOp/FlowBlock binding (W5/W7 seam)",
        ))
    }

    /// Iterate the ValueSet system until it stabilizes (C++ `solve`,
    /// `rangeutil.cc:2524`).
    ///
    /// SEAM(W5/W7-IR): the per-node `iterate()` push-forward reads input
    /// Varnodes' value sets through the IR back-pointer.  The partition-walk
    /// ordering that `solve` drives is ported faithfully in
    /// [`ValueSetSolver::establish_topological_order`] (exercised by the
    /// in-module ordering tests); the public `solve` remains a seam until
    /// `iterate` can run against live ops.
    pub fn solve(&mut self, _max: int4, _widener: &dyn Widener) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "ValueSetSolver::solve: per-node iterate() needs IR push-forward (W5/W7 seam)",
        ))
    }
}

#[cfg(test)]
mod tests;
