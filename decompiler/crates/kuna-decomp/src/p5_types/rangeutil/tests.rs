//! Porter in-module tests for `rangeutil.rs`.
//!
//! These pre-validate the `CircleRange` core against ~15 of the trickiest cases
//! from `decompiler/unittests/testcirclerange.cc` (the harness item that ports
//! all 117 of those runs comes *after* this item).  Two flavors:
//!
//!   * **Explicit-value matrices** — the upstream `pullBack*` tests that assert
//!     exact `getMin()`/`getEnd()` results (`testcirclerange.cc:584-754`).  These
//!     are transcribed verbatim with the same expected numbers; they are the
//!     output-determining per-opcode matrices.
//!
//!   * **Brute-force oracle** — for `intersect`/`circleUnion`/`pushForward*`,
//!     where upstream compares the range operator's result against an explicit
//!     enumeration of set members re-fit into a stride.  The
//!     [`OracleRange::get_start_stop_step`] helper transcribes
//!     `CircleRangeTest::getStartStopStep` (`testcirclerange.cc:291`) and
//!     `testEqual` (`:182`) exactly; a few representative cases run through it.
//!
//! Plus: a `Widener` widening-semantics test and a Bourdoncle
//! topological-ordering test for the `ValueSetSolver` graph walk.

use super::*;
use kuna_base::address::{calc_mask, sign_extend_sized};

// ---------------------------------------------------------------------------
// Brute-force oracle (mirror of testcirclerange.cc CircleRangeTest)
// ---------------------------------------------------------------------------

/// Mirror of `CircleRangeTest`: holds the explicit member set + mask/byte-size,
/// and can re-derive a `(valid,start,stop,step)` description that a CircleRange
/// operator result must match.
struct OracleRange {
    elements: Vec<uintb>,
    mask: uintb,
    bytes: int4,
}

impl OracleRange {
    /// Build from a CircleRange by enumerating its members
    /// (`CircleRangeTest(const CircleRange&)`, `testcirclerange.cc:158`).
    fn from_range(range: &CircleRange) -> OracleRange {
        let mask = range.get_mask();
        let mut elements = Vec::new();
        if !range.is_empty() {
            let mut start = range.get_min();
            loop {
                elements.push(start);
                if !range.get_next(&mut start) {
                    break;
                }
            }
        }
        let mut temp = mask.wrapping_add(1);
        let bytes = if temp == 0 {
            8
        } else {
            let mut b: int4 = -1;
            while temp != 0 {
                temp >>= 1;
                b += 1;
            }
            b / 8
        };
        OracleRange { elements, mask, bytes }
    }

    /// `set_intersect` (`testcirclerange.cc:202`).
    fn set_intersect(&mut self, op2: &mut OracleRange) {
        let a: std::collections::BTreeSet<uintb> = self.elements.iter().copied().collect();
        let b: std::collections::BTreeSet<uintb> = op2.elements.iter().copied().collect();
        self.elements = a.intersection(&b).copied().collect();
    }

    /// `set_union` (`testcirclerange.cc:213`).
    fn set_union(&mut self, op2: &mut OracleRange) {
        let a: std::collections::BTreeSet<uintb> = self.elements.iter().copied().collect();
        let b: std::collections::BTreeSet<uintb> = op2.elements.iter().copied().collect();
        self.elements = a.union(&b).copied().collect();
    }

    /// `getStartStopStep` (`testcirclerange.cc:291`).  Returns `(valid, start,
    /// stop, step)`.
    fn get_start_stop_step(&mut self) -> (bool, uintb, uintb, int4) {
        if self.elements.is_empty() {
            return (true, 0, 0, 1);
        }
        self.elements.sort_unstable();
        // dedup
        let mut dedup = Vec::new();
        let mut lastel = self.elements[0];
        dedup.push(lastel);
        for &curel in &self.elements[1..] {
            if curel == lastel {
                continue;
            }
            dedup.push(curel);
            lastel = curel;
        }
        self.elements = dedup;

        if *self.elements.last().unwrap() > self.mask {
            return (false, 0, 0, 1);
        }

        if self.elements.len() == 1 {
            let start = self.elements[0];
            let stop = (start.wadd(1)) & self.mask;
            return (true, start, stop, 1);
        }
        if self.elements.len() == 2 {
            let diff = (self.elements[0].wsub(self.elements[1])) & self.mask;
            if diff == 1 || diff == 2 || diff == 4 || diff == 8 {
                let start = self.elements[1];
                let stop = (start.wadd(diff).wadd(diff)) & self.mask;
                return (true, start, stop, diff as int4);
            }
        }
        let mut bigpos: i64 = -1;
        let mut biggest1: uintb = 0;
        let mut biggest2: uintb = 0;
        for i in 1..self.elements.len() {
            let diff = self.elements[i].wsub(self.elements[i - 1]);
            if diff >= biggest1 {
                if diff > biggest1 {
                    biggest2 = biggest1;
                    biggest1 = diff;
                    bigpos = i as i64;
                }
            } else if diff > biggest2 {
                biggest2 = diff;
            }
        }

        if biggest1 == 0 {
            return (false, 0, 0, 1);
        }
        if biggest2 == 0 {
            let step = biggest1 as int4;
            let start = self.elements[0];
            let stop = (self.elements.last().unwrap().wadd(step as uintb)) & self.mask;
            return (true, start, stop, step);
        }
        let mut count1 = 0;
        let mut count3 = 0;
        for i in 1..self.elements.len() {
            let diff = self.elements[i].wsub(self.elements[i - 1]);
            if diff == biggest1 {
                count1 += 1;
            } else if diff == biggest2 {
                // count2 (unused beyond the original harness's bookkeeping)
            } else {
                count3 += 1;
            }
        }
        if count3 > 0 {
            return (false, 0, 0, 1);
        }
        if count1 > 1 {
            return (false, 0, 0, 1);
        }
        let step = biggest2 as int4;
        let mut tmp = self.elements.last().unwrap().wadd(step as uintb);
        if tmp <= self.mask {
            return (false, 0, 0, 1);
        }
        tmp = tmp.wsub(self.mask.wadd(1));
        if tmp != self.elements[0] {
            return (false, 0, 0, 1);
        }
        let start = self.elements[bigpos as usize];
        let stop = self.elements[(bigpos - 1) as usize].wadd(step as uintb);
        (true, start, stop, step)
    }

    /// `testEqual` (`testcirclerange.cc:182`).
    fn test_equal(&mut self, valid: bool, range: &CircleRange) -> bool {
        if self.elements.is_empty() {
            return range.is_empty();
        } else if range.is_empty() {
            return false;
        }
        let (testvalid, start, stop, step) = self.get_start_stop_step();
        if testvalid != valid {
            return false;
        }
        if !valid {
            return true;
        }
        if start != range.get_min() {
            return false;
        }
        if stop != range.get_end() {
            return false;
        }
        if step != range.get_step() {
            return false;
        }
        true
    }
}

/// Reproduce `testIntersect` (`testcirclerange.cc:132`).
fn test_intersect(
    start1: uintb,
    stop1: uintb,
    start2: uintb,
    stop2: uintb,
    step: int4,
    bytes: int4,
) -> bool {
    let mut range1 = CircleRange::new(start1, stop1, bytes, step);
    let range2 = CircleRange::new(start2, stop2, bytes, step);
    let mut o1 = OracleRange::from_range(&range1);
    let mut o2 = OracleRange::from_range(&range2);
    let code = range1.intersect(&range2);
    o1.set_intersect(&mut o2);
    o1.test_equal(code == 0, &range1)
}

/// Reproduce `testUnion` (`testcirclerange.cc:145`).
fn test_union(
    start1: uintb,
    stop1: uintb,
    start2: uintb,
    stop2: uintb,
    step: int4,
    bytes: int4,
) -> bool {
    let mut range1 = CircleRange::new(start1, stop1, bytes, step);
    let range2 = CircleRange::new(start2, stop2, bytes, step);
    let mut o1 = OracleRange::from_range(&range1);
    let mut o2 = OracleRange::from_range(&range2);
    let code = range1.circle_union(&range2);
    o1.set_union(&mut o2);
    o1.test_equal(code == 0, &range1)
}

// ---------------------------------------------------------------------------
// intersect / union (a representative subset of circlerange_intersect*/union*)
// ---------------------------------------------------------------------------

#[test]
fn circlerange_intersect_subset() {
    // testcirclerange.cc:376-406
    assert!(test_intersect(1, 20, 10, 30, 1, 4)); // intersect1
    assert!(test_intersect(200, 10, 250, 5, 1, 1)); // intersect2 (wrap)
    assert!(test_intersect(1, 250, 240, 5, 1, 1)); // intersect3
    assert!(test_intersect(4, 100, 248, 52, 4, 1)); // intersect4 (step 4)
    // intersect5: 8-byte, large offsets, step 2 (the trickiest)
    assert!(test_intersect(0x100000, 0x1000fe, 0xfffffffffffffff0, 0xfffffffffffffffe, 2, 8));
    assert!(test_intersect(0xffe0, 0x20, 0, 0x20, 2, 2)); // intersect7 (wrap, step 2)
    assert!(test_intersect(0x80, 0x8, 0xd0, 0x80, 1, 1)); // intersect8 (double wrap)
}

#[test]
fn circlerange_union_subset() {
    // testcirclerange.cc:408-438
    assert!(test_union(1, 20, 10, 30, 1, 4)); // union1
    assert!(test_union(200, 10, 250, 5, 1, 1)); // union2 (wrap)
    assert!(test_union(4, 100, 248, 52, 4, 1)); // union4 (step 4)
    assert!(test_union(0xffe0, 0x20, 0, 0x20, 2, 2)); // union7
    assert!(test_union(0x80, 0x8, 0xd0, 0x80, 1, 1)); // union8
}

// ---------------------------------------------------------------------------
// Explicit-value pullBackBinary matrices (testcirclerange.cc:584-754)
// ---------------------------------------------------------------------------

#[test]
fn circlerange_pullback_right() {
    // pullbackright1
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2));
    assert_eq!(range.get_min(), 0x100);
    assert_eq!(range.get_end(), 0xf00);

    // pullbackright2
    let mut range = CircleRange::new(0xf0, 0x10, 2, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2));
    assert_eq!(range.get_min(), 0xf000);
    assert_eq!(range.get_end(), 0x1000);

    // pullbackright3
    let mut range = CircleRange::new(0xf0, 0x10, 1, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 1, 0, 1, 1));
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x20);

    // pullbackright4: stride present -> not valid
    let mut range = CircleRange::new(0x01, 0x0f, 2, 2);
    assert!(!range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2));
}

#[test]
fn circlerange_pullback_sright() {
    // pullbacksright1
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 8, 0, 2, 2));
    assert_eq!(range.get_min(), 0x100);
    assert_eq!(range.get_end(), 0xf00);

    // pullbacksright2
    let mut range = CircleRange::new(0xf0, 0x10, 1, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 2, 0, 1, 1));
    assert_eq!(range.get_min(), 0xc0);
    assert_eq!(range.get_end(), 0x40);

    // pullbacksright3
    let mut range = CircleRange::new(0x10, 0x30, 1, 1);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 2, 0, 1, 1));
    assert_eq!(range.get_min(), 0x40);
    assert_eq!(range.get_end(), 0x80);

    // pullbacksright4
    let mut range = CircleRange::new(0x01, 0x0f, 2, 2);
    assert!(!range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 8, 0, 2, 2));
}

#[test]
fn circlerange_pullback_equal_notequal() {
    // pullbackequal1: true range -> [0x1234, 0x1235)
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_EQUAL, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x1235);

    // pullbackequal2: false range -> complement [0x1235, 0x1234)
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_EQUAL, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x1234);

    // pullbacknotequal1
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_NOTEQUAL, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x1235);

    // pullbacknotequal2
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_NOTEQUAL, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x1234);
}

#[test]
fn circlerange_pullback_carry() {
    // pullbackcarry1: true -> [0xedcc, 0)
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_CARRY, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0xedcc);
    assert_eq!(range.get_end(), 0);

    // pullbackcarry2: false -> [0, 0xedcc)
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_CARRY, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0xedcc);
}

#[test]
fn circlerange_pullback_less_lessequal() {
    // pullbackless1
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_LESS, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0);

    // pullbackless2
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_LESS, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x1234);

    // pullbacklessequal1
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_LESSEQUAL, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0);

    // pullbacklessequal2
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_LESSEQUAL, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x1235);
}

#[test]
fn circlerange_pullback_sless_slessequal() {
    // pullbacksless1: false, 4-byte -> [0x1234, 0x80000000)
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SLESS, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x80000000);

    // pullbacksless2: true, 2-byte -> [0x8000, 0x1234)
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SLESS, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0x8000);
    assert_eq!(range.get_end(), 0x1234);

    // pullbackslessequal1
    let mut range = CircleRange::new_bool(false);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SLESSEQUAL, 0x1234, 0, 4, 1));
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x80000000);

    // pullbackslessequal2
    let mut range = CircleRange::new_bool(true);
    assert!(range.pull_back_binary(OpCode::CPUI_INT_SLESSEQUAL, 0x1234, 0, 2, 1));
    assert_eq!(range.get_min(), 0x8000);
    assert_eq!(range.get_end(), 0x1235);
}

// ---------------------------------------------------------------------------
// pullBackUnary / pullBackBinary oracle coverage (NEGATE/2COMP, ADD/SUB)
// ---------------------------------------------------------------------------

/// `testPullbackUnary` (`testcirclerange.cc:87`); only ops with a closed-form
/// member recover are checked (NEGATE, 2COMP) where each output maps to exactly
/// one input.
fn test_pullback_unary_oracle(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    insize: int4,
) -> bool {
    let mut range = CircleRange::new(start, stop, bytes, step);
    let mut oracle = OracleRange::from_range(&range);
    let valid = range.pull_back_unary(opcode, insize, bytes);
    let mut res = Vec::new();
    for &e in &oracle.elements {
        let r = match opcode {
            OpCode::CPUI_INT_NEGATE => (!e) & oracle.mask, // recoverInputUnary NEGATE: ~out
            OpCode::CPUI_INT_2COMP => e.wrapping_neg() & oracle.mask, // 2COMP: -out
            _ => unreachable!("oracle only covers NEGATE/2COMP"),
        };
        res.push(r);
    }
    oracle.elements = res;
    if insize != bytes {
        oracle.bytes = insize;
        oracle.mask = calc_mask(insize);
    }
    oracle.test_equal(valid, &range)
}

#[test]
fn circlerange_pullback_unary_oracle() {
    // negate cases (testcirclerange.cc:440-450)
    assert!(test_pullback_unary_oracle(1, 20, 1, 4, OpCode::CPUI_INT_NEGATE, 4));
    assert!(test_pullback_unary_oracle(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_NEGATE, 1));
    assert!(test_pullback_unary_oracle(0x10, 0x30, 4, 4, OpCode::CPUI_INT_NEGATE, 4));
    // 2comp cases (testcirclerange.cc:464-474)
    assert!(test_pullback_unary_oracle(1, 20, 1, 4, OpCode::CPUI_INT_2COMP, 4));
    assert!(test_pullback_unary_oracle(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_2COMP, 1));
    assert!(test_pullback_unary_oracle(0x10, 0x30, 4, 4, OpCode::CPUI_INT_2COMP, 4));
}

/// `testPullbackBinary` for ADD/SUB (`testcirclerange.cc:97`).  recoverInputBinary
/// for ADD slot0 is `out - val`; for SUB slot0 is `out + val`.
fn test_pullback_binary_oracle(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    slot: int4,
    val: uintb,
) -> bool {
    let mut range = CircleRange::new(start, stop, bytes, step);
    let mut oracle = OracleRange::from_range(&range);
    let valid = range.pull_back_binary(opcode, val, slot, bytes, bytes);
    let mut res = Vec::new();
    for &e in &oracle.elements {
        let r = match opcode {
            OpCode::CPUI_INT_ADD => e.wsub(val) & oracle.mask,
            OpCode::CPUI_INT_SUB => e.wadd(val) & oracle.mask,
            _ => unreachable!("oracle only covers ADD/SUB"),
        };
        res.push(r);
    }
    oracle.elements = res;
    oracle.test_equal(valid, &range)
}

#[test]
fn circlerange_pullback_binary_oracle() {
    // add cases (testcirclerange.cc:536-546)
    assert!(test_pullback_binary_oracle(1, 20, 1, 4, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
    assert!(test_pullback_binary_oracle(0x10, 0x30, 4, 4, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
    // sub cases (testcirclerange.cc:560-570)
    assert!(test_pullback_binary_oracle(1, 20, 1, 4, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
    assert!(test_pullback_binary_oracle(0x10, 0x30, 4, 4, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

// ---------------------------------------------------------------------------
// pushForward oracle (ZEXT / SEXT)
// ---------------------------------------------------------------------------

/// `testPushUnary` (`testcirclerange.cc:107`) for ZEXT/SEXT where the member
/// behavior is a closed form.
fn test_push_unary_oracle(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    outsize: int4,
) -> bool {
    let range = CircleRange::new(start, stop, bytes, step);
    let mut res = CircleRange::new_empty();
    let valid = res.push_forward_unary(opcode, &range, bytes, outsize);
    let mut oracle = OracleRange::from_range(&range);
    for e in oracle.elements.iter_mut() {
        *e = match opcode {
            OpCode::CPUI_INT_ZEXT => *e, // evaluateUnary ZEXT: value unchanged in larger space
            OpCode::CPUI_INT_SEXT => sign_extend_sized(*e, bytes, outsize),
            _ => unreachable!(),
        };
    }
    if outsize != bytes {
        oracle.bytes = outsize;
        oracle.mask = calc_mask(outsize);
    }
    oracle.test_equal(valid, &res)
}

#[test]
fn circlerange_push_unary_oracle() {
    // pushzext1/3 (testcirclerange.cc:804-814)
    assert!(test_push_unary_oracle(1, 20, 1, 2, OpCode::CPUI_INT_ZEXT, 4));
    assert!(test_push_unary_oracle(0x10, 0x30, 4, 2, OpCode::CPUI_INT_ZEXT, 4));
    // pushsext1/3 (testcirclerange.cc:832-842)
    assert!(test_push_unary_oracle(1, 20, 1, 2, OpCode::CPUI_INT_SEXT, 4));
    assert!(test_push_unary_oracle(0x10, 0x30, 4, 2, OpCode::CPUI_INT_SEXT, 4));
    // pushzext7: single value 0 (testcirclerange.cc:828)
    assert!(test_push_unary_oracle(0, 0, 4, 1, OpCode::CPUI_INT_ZEXT, 2));
}

// ---------------------------------------------------------------------------
// translate2Op
// ---------------------------------------------------------------------------

#[test]
fn circlerange_translate2op() {
    let mut opc = OpCode::CPUI_MAX;
    let mut c = 0;
    let mut cslot = 0;

    // Single value -> INT_EQUAL
    let r = CircleRange::new_value(0x42, 4);
    assert_eq!(r.translate2_op(&mut opc, &mut c, &mut cslot), 0);
    assert_eq!(opc, OpCode::CPUI_INT_EQUAL);
    assert_eq!(c, 0x42);
    assert_eq!(cslot, 0);

    // [0, 10) -> INT_LESS with constant on slot 1
    let r = CircleRange::new(0, 10, 4, 1);
    assert_eq!(r.translate2_op(&mut opc, &mut c, &mut cslot), 0);
    assert_eq!(opc, OpCode::CPUI_INT_LESS);
    assert_eq!(cslot, 1);
    assert_eq!(c, 10);

    // full range -> all outputs possible (code 1)
    let mut full = CircleRange::new_empty();
    full.set_full(4);
    assert_eq!(full.translate2_op(&mut opc, &mut c, &mut cslot), 1);

    // empty -> code 3
    let empty = CircleRange::new_empty();
    assert_eq!(empty.translate2_op(&mut opc, &mut c, &mut cslot), 3);

    // stride present -> code 2 (not possible)
    let strided = CircleRange::new(0, 8, 4, 2);
    assert_eq!(strided.translate2_op(&mut opc, &mut c, &mut cslot), 2);
}

// ---------------------------------------------------------------------------
// getSize / isSingle / isFull semantics
// ---------------------------------------------------------------------------

#[test]
fn circlerange_size_and_predicates() {
    // [0,8) step 1 -> 8 values
    let r = CircleRange::new(0, 8, 4, 1);
    assert_eq!(r.get_size(), 8);
    assert!(!r.is_single());
    assert!(!r.is_full());

    // single value
    let s = CircleRange::new_value(5, 4);
    assert!(s.is_single());
    assert_eq!(s.get_size(), 1);

    // [n,n) step 1 is the FULL range in CircleRange semantics (NOT empty).
    let full = CircleRange::new(5, 5, 4, 1);
    assert!(full.is_full());
    assert_eq!(full.get_size(), 0x100000000); // mask+1 == 2^32

    // explicit empty
    let e = CircleRange::new_empty();
    assert!(e.is_empty());
    assert_eq!(e.get_size(), 0);

    // stepped range [0,16) step 2 -> 8 elements
    let stepped = CircleRange::new(0, 16, 4, 2);
    assert_eq!(stepped.get_size(), 8);
}

// ---------------------------------------------------------------------------
// Widener semantics (WidenerFull / WidenerNone)
// ---------------------------------------------------------------------------

#[test]
fn widener_full_landmark_widening() {
    // Build a ValueSet at the widen iteration with a landmark equation, and a
    // newRange contained in the landmark.  doWidening should widen toward the
    // landmark boundary.
    let widener = WidenerFull::new(); // widen=2 full=5
    let mut vs = ValueSet::new();
    vs.set_count(2); // exactly at widenIteration
    // landmark equation [0, 0x100) on a 4-byte domain, typeCode 0
    vs.add_equation(0, 0, CircleRange::new(0, 0x100, 4, 1));

    let mut range = CircleRange::new(0, 4, 4, 1); // previous
    let new_range = CircleRange::new(0, 8, 4, 1); // current (still inside landmark)
    let res = widener.do_widening(&vs, &mut range, &new_range);
    assert!(res);
    // left stable (both start at 0) -> right widened to landmark right (0x100).
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x100);
}

#[test]
fn widener_full_resets_and_freezes() {
    let widener = WidenerFull::with_iterations(2, 5);
    let mut vs = ValueSet::new();
    // determineIterationReset: count<widen -> 0 ; count>=widen -> widen
    vs.set_count(1);
    assert_eq!(widener.determine_iteration_reset(&vs), 0);
    vs.set_count(3);
    assert_eq!(widener.determine_iteration_reset(&vs), 2);

    // checkFreeze: a full range freezes
    let mut full = CircleRange::new_empty();
    full.set_full(4);
    vs.set_range(full);
    assert!(widener.check_freeze(&vs));

    // before widenIteration, doWidening just adopts newRange
    let mut vs2 = ValueSet::new();
    vs2.set_count(1);
    let mut range = CircleRange::new(0, 4, 4, 1);
    let new_range = CircleRange::new(0, 9, 4, 1);
    assert!(widener.do_widening(&vs2, &mut range, &new_range));
    assert!(range.equals(&new_range));
}

#[test]
fn widener_none_freezes_at_iteration() {
    let widener = WidenerNone::new(); // freeze=3
    let mut vs = ValueSet::new();
    vs.set_count(2);
    assert!(!widener.check_freeze(&vs));
    vs.set_count(3);
    assert!(widener.check_freeze(&vs));
    // determineIterationReset: count<freeze -> count ; count>=freeze -> freeze
    vs.set_count(1);
    assert_eq!(widener.determine_iteration_reset(&vs), 1);
    vs.set_count(5);
    assert_eq!(widener.determine_iteration_reset(&vs), 3);
    // doWidening always adopts newRange
    let mut range = CircleRange::new(0, 1, 4, 1);
    let new_range = CircleRange::new(0, 5, 4, 1);
    assert!(widener.do_widening(&vs, &mut range, &new_range));
    assert!(range.equals(&new_range));
}

// ---------------------------------------------------------------------------
// ValueSetSolver Bourdoncle weak-topological-ordering
// ---------------------------------------------------------------------------

/// A static adjacency-list graph for the ordering test.  Node `succ.len()` is
/// the simulated root; its successors are `root_succ`.
struct StaticGraph {
    succ: Vec<Vec<usize>>,
    root_succ: Vec<usize>,
}

impl ValueSetGraph for StaticGraph {
    fn successors(&self, node: usize) -> Vec<usize> {
        if node == self.succ.len() {
            self.root_succ.clone()
        } else {
            self.succ[node].clone()
        }
    }
}

#[test]
fn valuesetsolver_topological_order_linear() {
    // A simple chain root -> 0 -> 1 -> 2 (no cycles).  Bourdoncle ordering puts
    // them in dependency order 0,1,2 with no components.
    let mut solver = ValueSetSolver::new();
    let n0 = solver.alloc_value_set();
    let n1 = solver.alloc_value_set();
    let n2 = solver.alloc_value_set();
    solver.push_root(n0);

    let graph = StaticGraph { succ: vec![vec![n1], vec![n2], vec![]], root_succ: vec![n0] };
    solver.establish_topological_order(&graph);
    assert_eq!(solver.iteration_order(), vec![n0, n1, n2]);
    // no component heads in an acyclic chain
    assert!(solver.part_head_of(n0).is_none());
    assert!(solver.part_head_of(n1).is_none());
    assert!(solver.part_head_of(n2).is_none());
}

#[test]
fn valuesetsolver_topological_order_with_cycle() {
    // root -> 0 -> 1 -> 2, with a back-edge 2 -> 1 forming a loop {1,2}.
    // Bourdoncle: node 1 becomes a component head; order is 0,1,2.
    let mut solver = ValueSetSolver::new();
    let n0 = solver.alloc_value_set();
    let n1 = solver.alloc_value_set();
    let n2 = solver.alloc_value_set();
    solver.push_root(n0);

    let graph =
        StaticGraph { succ: vec![vec![n1], vec![n2], vec![n1]], root_succ: vec![n0] };
    solver.establish_topological_order(&graph);
    let order = solver.iteration_order();
    assert_eq!(order, vec![n0, n1, n2]);
    // The loop {1,2} has head 1 (a partition component).
    assert!(solver.part_head_of(n1).is_some());
    // node 0 is outside the loop.
    assert!(solver.part_head_of(n0).is_none());
}
