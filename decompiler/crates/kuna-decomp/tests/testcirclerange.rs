//! Port of `decompiler/unittests/testcirclerange.cc` (117 TESTs).
//!
//! Faithful 1:1 transcription of the upstream Ghidra `CircleRangeTest`
//! harness and every `TEST(...)`.  `CircleRange` lives in
//! `kuna_decomp::rangeutil`; the OpBehavior emulation surface the harness uses
//! to compute its brute-force reference set comes from `kuna_num::opbehavior`
//! (the `glb->inst[opcode]->getBehavior()` of the C++ environment).
//!
//! Porting notes (C++ is the spec):
//!   - `glb->inst[opcode]->getBehavior()` becomes a `Vec<Option<Rc<dyn
//!     OpBehavior>>>` built once by `register_instructions` (the
//!     `CircleRangeTestEnvironment`).  The C++ test only ever exercises integer
//!     behaviors, so the `NoFormats` float provider is sufficient.
//!   - `catch(EvaluationError &err)` (the "output is not in range" skip in
//!     `pullbackUnary`/`pullbackBinary`) becomes matching on
//!     `KunaError::Evaluation`; any other `Err` would be a genuine porting bug
//!     and is re-surfaced via `panic`.
//!   - mixed-sign / wrapping arithmetic in `getStartStopStep` is replicated with
//!     the `Wrap` helpers (ADR 0003); the `& mask` truncations are verbatim.

use std::cmp::Ordering;
use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_base::error::KunaError;
use kuna_base::types::{int4, uintb, Wrap};
use kuna_decomp::rangeutil::CircleRange;
use kuna_num::float::FloatFormat;
use kuna_num::opbehavior::{register_instructions, FloatFormatProvider, OpBehavior};
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// CircleRangeTestEnvironment
//
// The C++ environment lazily builds an Architecture solely to reach
// `glb->inst[opcode]->getBehavior()`.  Here we build the equivalent OpBehavior
// table directly (no Architecture needed for integer behaviors).
// ---------------------------------------------------------------------------

/// A float-format provider with no formats registered (the integer behaviors
/// the harness uses never consult it).
struct NoFormats;
impl FloatFormatProvider for NoFormats {
    fn get_float_format(&self, _size: i32) -> Option<&FloatFormat> {
        None
    }
}

/// Build the per-opcode OpBehavior table (C++ `glb->inst`).
fn behavior_table() -> Vec<Option<Rc<dyn OpBehavior>>> {
    let mut inst: Vec<Option<Rc<dyn OpBehavior>>> = Vec::new();
    let trans: Rc<dyn FloatFormatProvider> = Rc::new(NoFormats);
    register_instructions(&mut inst, &trans);
    inst
}

/// Look up the behavior for `opcode` (C++ `glb->inst[opcode]->getBehavior()`).
fn behavior_for(inst: &[Option<Rc<dyn OpBehavior>>], opcode: OpCode) -> &dyn OpBehavior {
    inst[opcode as usize].as_deref().expect("behavior registered")
}

// ---------------------------------------------------------------------------
// CircleRangeTest
// ---------------------------------------------------------------------------

struct CircleRangeTest {
    elements: Vec<uintb>,
    mask: uintb,
    bytes: int4,
}

impl CircleRangeTest {
    /// C++ `CircleRangeTest(int4 b)`.
    fn with_bytes(b: int4) -> CircleRangeTest {
        CircleRangeTest { elements: Vec::new(), mask: calc_mask(b), bytes: b }
    }

    /// C++ `CircleRangeTest(const CircleRange &range)`.
    fn from_range(range: &CircleRange) -> CircleRangeTest {
        let mask = range.get_mask();
        let mut elements: Vec<uintb> = Vec::new();
        if !range.is_empty() {
            let mut start = range.get_min();
            loop {
                elements.push(start);
                if !range.get_next(&mut start) {
                    break;
                }
            }
        }
        let mut temp = mask.wadd(1);
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
        CircleRangeTest { elements, mask, bytes }
    }

    /// C++ `set_intersect`.
    fn set_intersect(&mut self, op2: &mut CircleRangeTest) {
        // std::set_intersection requires both inputs sorted.
        self.elements.sort_unstable();
        op2.elements.sort_unstable();
        let res = set_intersection(&self.elements, &op2.elements);
        self.elements = res;
    }

    /// C++ `set_union`.
    fn set_union(&mut self, op2: &mut CircleRangeTest) {
        self.elements.sort_unstable();
        op2.elements.sort_unstable();
        let res = set_union(&self.elements, &op2.elements);
        self.elements = res;
    }

    /// C++ `pushUnary`.
    fn push_unary(&mut self, inst: &[Option<Rc<dyn OpBehavior>>], opcode: OpCode, outsize: int4) {
        let behave = behavior_for(inst, opcode);
        for el in self.elements.iter_mut() {
            *el = behave.evaluate_unary(outsize, self.bytes, *el).unwrap();
        }
        if outsize != self.bytes {
            self.bytes = outsize;
            self.mask = calc_mask(outsize);
        }
    }

    /// C++ `pushBinary`.
    fn push_binary(
        &mut self,
        inst: &[Option<Rc<dyn OpBehavior>>],
        opcode: OpCode,
        outsize: int4,
        in1: &CircleRangeTest,
        in2: &CircleRangeTest,
    ) {
        let behave = behavior_for(inst, opcode);
        self.elements.clear();
        for &a in in1.elements.iter() {
            for &b in in2.elements.iter() {
                self.elements
                    .push(behave.evaluate_binary(outsize, in1.bytes, a, b).unwrap());
            }
        }
        if outsize != self.bytes {
            self.bytes = outsize;
            self.mask = calc_mask(outsize);
        }
    }

    /// C++ `pullbackUnary`.
    fn pullback_unary(&mut self, inst: &[Option<Rc<dyn OpBehavior>>], opcode: OpCode, insize: int4) {
        let behave = behavior_for(inst, opcode);
        let mut res: Vec<uintb> = Vec::new();
        for &el in self.elements.iter() {
            match behave.recover_input_unary(self.bytes, el, insize) {
                Ok(v) => res.push(v),
                Err(KunaError::Evaluation { .. }) => {
                    // output is not in range
                }
                Err(e) => panic!("unexpected error in recover_input_unary: {e:?}"),
            }
        }
        self.elements = res;
        if insize != self.bytes {
            self.bytes = insize;
            self.mask = calc_mask(insize);
        }
    }

    /// C++ `pullbackBinary`.
    fn pullback_binary(
        &mut self,
        inst: &[Option<Rc<dyn OpBehavior>>],
        opcode: OpCode,
        slot: int4,
        val: uintb,
    ) {
        let behave = behavior_for(inst, opcode);
        let mut res: Vec<uintb> = Vec::new();
        for &el in self.elements.iter() {
            match behave.recover_input_binary(slot, self.bytes, el, self.bytes, val) {
                Ok(v) => res.push(v),
                Err(KunaError::Evaluation { .. }) => {
                    // output is not in range
                }
                Err(e) => panic!("unexpected error in recover_input_binary: {e:?}"),
            }
        }
        self.elements = res;
    }

    /// C++ `testEqual`.
    fn test_equal(&mut self, valid: bool, range: &CircleRange) -> bool {
        if self.elements.is_empty() {
            return range.is_empty();
        } else if range.is_empty() {
            return false;
        }
        let mut start: uintb = 0;
        let mut stop: uintb = 0;
        let mut step: int4 = 0;
        let testvalid = self.get_start_stop_step(&mut start, &mut stop, &mut step);
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

    /// C++ `getStartStopStep`.
    fn get_start_stop_step(&mut self, start: &mut uintb, stop: &mut uintb, step: &mut int4) -> bool {
        if self.elements.is_empty() {
            *start = 0;
            *stop = 0;
            *step = 1;
            return true;
        }
        self.elements.sort_unstable();
        let mut dedup: Vec<uintb> = Vec::new();
        let mut lastel = self.elements[0];
        dedup.push(lastel);
        for i in 1..self.elements.len() {
            // Deduplicate the values
            let curel = self.elements[i];
            if curel == lastel {
                continue;
            }
            dedup.push(curel);
            lastel = curel;
        }
        std::mem::swap(&mut self.elements, &mut dedup);

        if *self.elements.last().unwrap() > self.mask {
            return false;
        }

        if self.elements.len() == 1 {
            *start = self.elements[0];
            *stop = start.wadd(1) & self.mask;
            *step = 1;
            return true;
        }
        if self.elements.len() == 2 {
            let diff = self.elements[0].wsub(self.elements[1]) & self.mask;
            if diff == 1 || diff == 2 || diff == 4 || diff == 8 {
                *start = self.elements[1];
                *stop = start.wadd(diff).wadd(diff) & self.mask;
                *step = diff as int4;
                return true;
            }
        }
        let mut bigpos: int4 = -1;
        let mut biggest1: uintb = 0;
        let mut biggest2: uintb = 0;
        for i in 1..self.elements.len() {
            let diff = self.elements[i].wsub(self.elements[i - 1]);
            if diff >= biggest1 {
                if diff > biggest1 {
                    biggest2 = biggest1;
                    biggest1 = diff;
                    bigpos = i as int4;
                }
            } else if diff > biggest2 {
                biggest2 = diff;
            }
        }

        if biggest1 == 0 {
            return false;
        }
        if biggest2 == 0 {
            *step = biggest1 as int4;
            *start = self.elements[0];
            *stop = self.elements.last().unwrap().wadd(*step as uintb) & self.mask;
            return true;
        }
        let mut count1: int4 = 0;
        let mut count2: int4 = 0;
        let mut count3: int4 = 0;
        for i in 1..self.elements.len() {
            let diff = self.elements[i].wsub(self.elements[i - 1]);
            if diff == biggest1 {
                count1 += 1;
            } else if diff == biggest2 {
                count2 += 1;
            } else {
                count3 += 1;
            }
        }
        let _ = count2;
        if count3 > 0 {
            return false;
        }
        if count1 > 1 {
            return false;
        }
        *step = biggest2 as int4;
        let mut tmp = self.elements.last().unwrap().wadd(*step as uintb);
        if tmp <= self.mask {
            return false;
        }
        tmp = tmp.wsub(self.mask.wadd(1));
        if tmp != self.elements[0] {
            return false;
        }
        *start = self.elements[bigpos as usize];
        *stop = self.elements[(bigpos - 1) as usize].wadd(*step as uintb);
        true
    }
}

// ---------------------------------------------------------------------------
// Static test-drivers (C++ `CircleRangeTest::test*`)
// ---------------------------------------------------------------------------

fn test_pullback_unary(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    insize: int4,
) -> bool {
    let inst = behavior_table();
    let mut range = CircleRange::new(start, stop, bytes, step);
    let mut testrange = CircleRangeTest::from_range(&range);
    let valid = range.pull_back_unary(opcode, insize, bytes);
    testrange.pullback_unary(&inst, opcode, insize);
    testrange.test_equal(valid, &range)
}

fn test_pullback_binary(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    slot: int4,
    val: uintb,
) -> bool {
    let inst = behavior_table();
    let mut range = CircleRange::new(start, stop, bytes, step);
    let mut testrange = CircleRangeTest::from_range(&range);
    let valid = range.pull_back_binary(opcode, val, slot, bytes, bytes);
    testrange.pullback_binary(&inst, opcode, slot, val);
    testrange.test_equal(valid, &range)
}

fn test_push_unary(
    start: uintb,
    stop: uintb,
    step: int4,
    bytes: int4,
    opcode: OpCode,
    outsize: int4,
) -> bool {
    let inst = behavior_table();
    let range = CircleRange::new(start, stop, bytes, step);
    let mut res = CircleRange::new_empty();
    let mut testrange = CircleRangeTest::from_range(&range);
    let valid = res.push_forward_unary(opcode, &range, bytes, outsize);
    testrange.push_unary(&inst, opcode, outsize);
    testrange.test_equal(valid, &res)
}

#[allow(clippy::too_many_arguments)]
fn test_push_binary(
    start1: uintb,
    stop1: uintb,
    step1: int4,
    start2: uintb,
    stop2: uintb,
    step2: int4,
    bytes: int4,
    opcode: OpCode,
    outsize: int4,
) -> bool {
    let inst = behavior_table();
    let range1 = CircleRange::new(start1, stop1, bytes, step1);
    let range2 = CircleRange::new(start2, stop2, bytes, step2);
    let testrange1 = CircleRangeTest::from_range(&range1);
    let testrange2 = CircleRangeTest::from_range(&range2);
    let mut res = CircleRange::new_empty();
    let valid = res.push_forward_binary(opcode, &range1, &range2, bytes, outsize, 32);
    let mut testres = CircleRangeTest::with_bytes(outsize);
    testres.push_binary(&inst, opcode, outsize, &testrange1, &testrange2);
    testres.test_equal(valid, &res)
}

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
    let mut testrange1 = CircleRangeTest::from_range(&range1);
    let mut testrange2 = CircleRangeTest::from_range(&range2);

    let code = range1.intersect(&range2);
    testrange1.set_intersect(&mut testrange2);
    testrange1.test_equal(code == 0, &range1)
}

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
    let mut testrange1 = CircleRangeTest::from_range(&range1);
    let mut testrange2 = CircleRangeTest::from_range(&range2);

    let code = range1.circle_union(&range2);
    testrange1.set_union(&mut testrange2);
    testrange1.test_equal(code == 0, &range1)
}

// ---------------------------------------------------------------------------
// std::set_intersection / std::set_union over sorted sequences (multiset
// semantics, matching the C++ harness exactly).
// ---------------------------------------------------------------------------

/// `std::set_intersection(first1,last1,first2,last2,res)` over sorted inputs.
fn set_intersection(a: &[uintb], b: &[uintb]) -> Vec<uintb> {
    let mut res: Vec<uintb> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                res.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    res
}

/// `std::set_union(first1,last1,first2,last2,res)` over sorted inputs.
fn set_union(a: &[uintb], b: &[uintb]) -> Vec<uintb> {
    let mut res: Vec<uintb> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less => {
                res.push(a[i]);
                i += 1;
            }
            Ordering::Greater => {
                res.push(b[j]);
                j += 1;
            }
            Ordering::Equal => {
                res.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    while i < a.len() {
        res.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        res.push(b[j]);
        j += 1;
    }
    res
}

// ===========================================================================
// TESTs
// ===========================================================================

#[test]
fn circlerange_intersect1() {
    assert!(test_intersect(1, 20, 10, 30, 1, 4));
}

#[test]
fn circlerange_intersect2() {
    assert!(test_intersect(200, 10, 250, 5, 1, 1));
}

#[test]
fn circlerange_intersect3() {
    assert!(test_intersect(1, 250, 240, 5, 1, 1));
}

#[test]
fn circlerange_intersect4() {
    assert!(test_intersect(4, 100, 248, 52, 4, 1));
}

#[test]
fn circlerange_intersect5() {
    assert!(test_intersect(
        0x100000,
        0x1000fe,
        0xfffffffffffffff0,
        0xfffffffffffffffe,
        2,
        8
    ));
}

#[test]
fn circlerange_intersect6() {
    assert!(test_intersect(0x100, 0x110, 0x110, 0x130, 4, 2));
}

#[test]
fn circlerange_intersect7() {
    assert!(test_intersect(0xffe0, 0x20, 0, 0x20, 2, 2));
}

#[test]
fn circlerange_intersect8() {
    assert!(test_intersect(0x80, 0x8, 0xd0, 0x80, 1, 1));
}

#[test]
fn circlerange_union1() {
    assert!(test_union(1, 20, 10, 30, 1, 4));
}

#[test]
fn circlerange_union2() {
    assert!(test_union(200, 10, 250, 5, 1, 1));
}

#[test]
fn circlerange_union3() {
    assert!(test_union(1, 250, 240, 5, 1, 1));
}

#[test]
fn circlerange_union4() {
    assert!(test_union(4, 100, 248, 52, 4, 1));
}

#[test]
fn circlerange_union5() {
    assert!(test_union(
        0x100000,
        0x1000fe,
        0xfffffffffffffff0,
        0xfffffffffffffffe,
        2,
        8
    ));
}

#[test]
fn circlerange_union6() {
    assert!(test_union(0x100, 0x110, 0x110, 0x130, 4, 2));
}

#[test]
fn circlerange_union7() {
    assert!(test_union(0xffe0, 0x20, 0, 0x20, 2, 2));
}

#[test]
fn circlerange_union8() {
    assert!(test_union(0x80, 0x8, 0xd0, 0x80, 1, 1));
}

#[test]
fn circlerange_pullbacknegate1() {
    assert!(test_pullback_unary(1, 20, 1, 4, OpCode::CPUI_INT_NEGATE, 4));
}

#[test]
fn circlerange_pullbacknegate2() {
    assert!(test_pullback_unary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pullbacknegate3() {
    assert!(test_pullback_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_NEGATE, 4));
}

#[test]
fn circlerange_pullbacknegate4() {
    assert!(test_pullback_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_NEGATE, 2));
}

#[test]
fn circlerange_pullbacknegate5() {
    assert!(test_pullback_unary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pullbacknegate6() {
    assert!(test_pullback_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pullbackminus1() {
    assert!(test_pullback_unary(1, 20, 1, 4, OpCode::CPUI_INT_2COMP, 4));
}

#[test]
fn circlerange_pullbackminus2() {
    assert!(test_pullback_unary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pullbackminus3() {
    assert!(test_pullback_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_2COMP, 4));
}

#[test]
fn circlerange_pullbackminus4() {
    assert!(test_pullback_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_2COMP, 2));
}

#[test]
fn circlerange_pullbackminus5() {
    assert!(test_pullback_unary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pullbackminus6() {
    assert!(test_pullback_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pullbackzext1() {
    assert!(test_pullback_unary(1, 20, 1, 4, OpCode::CPUI_INT_ZEXT, 2));
}

#[test]
fn circlerange_pullbackzext2() {
    assert!(test_pullback_unary(0xfff0, 0xff10, 1, 2, OpCode::CPUI_INT_ZEXT, 1));
}

#[test]
fn circlerange_pullbackzext3() {
    assert!(test_pullback_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_ZEXT, 1));
}

#[test]
fn circlerange_pullbackzext4() {
    assert!(test_pullback_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_ZEXT, 1));
}

#[test]
fn circlerange_pullbackzext5() {
    assert!(test_pullback_unary(0xffd1, 0x11, 4, 2, OpCode::CPUI_INT_ZEXT, 1));
}

#[test]
fn circlerange_pullbackzext6() {
    assert!(test_pullback_unary(0, 0x30, 4, 4, OpCode::CPUI_INT_ZEXT, 2));
}

#[test]
fn circlerange_pullbacksext1() {
    assert!(test_pullback_unary(1, 20, 1, 4, OpCode::CPUI_INT_SEXT, 2));
}

#[test]
fn circlerange_pullbacksext2() {
    assert!(test_pullback_unary(0xfff0, 0x10, 1, 2, OpCode::CPUI_INT_SEXT, 1));
}

#[test]
fn circlerange_pullbacksext3() {
    assert!(test_pullback_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_SEXT, 2));
}

#[test]
fn circlerange_pullbacksext4() {
    assert!(test_pullback_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_SEXT, 1));
}

#[test]
fn circlerange_pullbacksext5() {
    assert!(test_pullback_unary(0xffd1, 0x11, 4, 2, OpCode::CPUI_INT_SEXT, 1));
}

#[test]
fn circlerange_pullbacksext6() {
    assert!(test_pullback_unary(0, 0x30, 4, 2, OpCode::CPUI_INT_SEXT, 1));
}

#[test]
fn circlerange_pullbackadd1() {
    assert!(test_pullback_binary(1, 20, 1, 4, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackadd2() {
    assert!(test_pullback_binary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackadd3() {
    assert!(test_pullback_binary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackadd4() {
    assert!(test_pullback_binary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackadd5() {
    assert!(test_pullback_binary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackadd6() {
    assert!(test_pullback_binary(0, 0x30, 4, 1, OpCode::CPUI_INT_ADD, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub1() {
    assert!(test_pullback_binary(1, 20, 1, 4, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub2() {
    assert!(test_pullback_binary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub3() {
    assert!(test_pullback_binary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub4() {
    assert!(test_pullback_binary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub5() {
    assert!(test_pullback_binary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbacksub6() {
    assert!(test_pullback_binary(0, 0x30, 4, 1, OpCode::CPUI_INT_SUB, 0, 0xfffffffd));
}

#[test]
fn circlerange_pullbackright1() {
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2);
    assert!(valid);
    assert_eq!(range.get_min(), 0x100);
    assert_eq!(range.get_end(), 0xf00);
}

#[test]
fn circlerange_pullbackright2() {
    let mut range = CircleRange::new(0xf0, 0x10, 2, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2);
    assert!(valid);
    assert_eq!(range.get_min(), 0xf000);
    assert_eq!(range.get_end(), 0x1000);
}

#[test]
fn circlerange_pullbackright3() {
    let mut range = CircleRange::new(0xf0, 0x10, 1, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 1, 0, 1, 1);
    assert!(valid);
    assert_eq!(0, range.get_min());
    assert_eq!(0x20, range.get_end());
}

#[test]
fn circlerange_pullbackright4() {
    let mut range = CircleRange::new(0x01, 0x0f, 2, 2);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_RIGHT, 8, 0, 2, 2);
    assert!(!valid);
}

#[test]
fn circlerange_pullbacksright1() {
    let mut range = CircleRange::new(0x01, 0x0f, 2, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 8, 0, 2, 2);
    assert!(valid);
    assert_eq!(range.get_min(), 0x100);
    assert_eq!(range.get_end(), 0xf00);
}

#[test]
fn circlerange_pullbacksright2() {
    let mut range = CircleRange::new(0xf0, 0x10, 1, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 2, 0, 1, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0xc0);
    assert_eq!(range.get_end(), 0x40);
}

#[test]
fn circlerange_pullbacksright3() {
    let mut range = CircleRange::new(0x10, 0x30, 1, 1);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 2, 0, 1, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x40);
    assert_eq!(range.get_end(), 0x80);
}

#[test]
fn circlerange_pullbacksright4() {
    let mut range = CircleRange::new(0x01, 0x0f, 2, 2);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SRIGHT, 8, 0, 2, 2);
    assert!(!valid);
}

#[test]
fn circlerange_pullbackequal1() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_EQUAL, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x1235);
}

#[test]
fn circlerange_pullbackequal2() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_EQUAL, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x1234);
}

#[test]
fn circlerange_pullbacknotequal1() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_NOTEQUAL, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x1235);
}

#[test]
fn circlerange_pullbacknotequal2() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_NOTEQUAL, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x1234);
}

#[test]
fn circlerange_pullbackcarry1() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_CARRY, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0xedcc);
    assert_eq!(range.get_end(), 0);
}

#[test]
fn circlerange_pullbackcarry2() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_CARRY, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0xedcc);
}

#[test]
fn circlerange_pullbackless1() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_LESS, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0);
}

#[test]
fn circlerange_pullbackless2() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_LESS, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x1234);
}

#[test]
fn circlerange_pullbacklessequal1() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_LESSEQUAL, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0);
}

#[test]
fn circlerange_pullbacklessequal2() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_LESSEQUAL, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0);
    assert_eq!(range.get_end(), 0x1235);
}

#[test]
fn circlerange_pullbacksless1() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SLESS, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1234);
    assert_eq!(range.get_end(), 0x80000000);
}

#[test]
fn circlerange_pullbacksless2() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SLESS, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x8000);
    assert_eq!(range.get_end(), 0x1234);
}

#[test]
fn circlerange_pullbackslessequal1() {
    let mut range = CircleRange::new_bool(false);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SLESSEQUAL, 0x1234, 0, 4, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x1235);
    assert_eq!(range.get_end(), 0x80000000);
}

#[test]
fn circlerange_pullbackslessequal2() {
    let mut range = CircleRange::new_bool(true);
    let valid = range.pull_back_binary(OpCode::CPUI_INT_SLESSEQUAL, 0x1234, 0, 2, 1);
    assert!(valid);
    assert_eq!(range.get_min(), 0x8000);
    assert_eq!(range.get_end(), 0x1235);
}

#[test]
fn circlerange_pushnegate1() {
    assert!(test_push_unary(1, 20, 1, 4, OpCode::CPUI_INT_NEGATE, 4));
}

#[test]
fn circlerange_pushnegate2() {
    assert!(test_push_unary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pushnegate3() {
    assert!(test_push_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_NEGATE, 4));
}

#[test]
fn circlerange_pushnegate4() {
    assert!(test_push_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_NEGATE, 2));
}

#[test]
fn circlerange_pushnegate5() {
    assert!(test_push_unary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pushnegate6() {
    assert!(test_push_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_NEGATE, 1));
}

#[test]
fn circlerange_pushminus1() {
    assert!(test_push_unary(1, 20, 1, 4, OpCode::CPUI_INT_2COMP, 4));
}

#[test]
fn circlerange_pushminus2() {
    assert!(test_push_unary(0xf0, 0x10, 1, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pushminus3() {
    assert!(test_push_unary(0x10, 0x30, 4, 4, OpCode::CPUI_INT_2COMP, 4));
}

#[test]
fn circlerange_pushminus4() {
    assert!(test_push_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_2COMP, 2));
}

#[test]
fn circlerange_pushminus5() {
    assert!(test_push_unary(0xd1, 0x11, 4, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pushminus6() {
    assert!(test_push_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_2COMP, 1));
}

#[test]
fn circlerange_pushzext1() {
    assert!(test_push_unary(1, 20, 1, 2, OpCode::CPUI_INT_ZEXT, 4));
}

#[test]
fn circlerange_pushzext2() {
    assert!(test_push_unary(0xfff0, 0xff10, 1, 2, OpCode::CPUI_INT_ZEXT, 4));
}

#[test]
fn circlerange_pushzext3() {
    assert!(test_push_unary(0x10, 0x30, 4, 2, OpCode::CPUI_INT_ZEXT, 4));
}

#[test]
fn circlerange_pushzext4() {
    assert!(test_push_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_ZEXT, 4));
}

#[test]
fn circlerange_pushzext5() {
    assert!(test_push_unary(0xffd1, 0xfff1, 4, 2, OpCode::CPUI_INT_ZEXT, 4));
}

#[test]
fn circlerange_pushzext6() {
    assert!(test_push_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_ZEXT, 2));
}

#[test]
fn circlerange_pushzext7() {
    assert!(test_push_unary(0, 0, 4, 1, OpCode::CPUI_INT_ZEXT, 2));
}

#[test]
fn circlerange_pushsext1() {
    assert!(test_push_unary(1, 20, 1, 2, OpCode::CPUI_INT_SEXT, 4));
}

#[test]
fn circlerange_pushsext2() {
    assert!(test_push_unary(0xfff0, 0xff10, 1, 2, OpCode::CPUI_INT_SEXT, 4));
}

#[test]
fn circlerange_pushsext3() {
    assert!(test_push_unary(0x10, 0x30, 4, 2, OpCode::CPUI_INT_SEXT, 4));
}

#[test]
fn circlerange_pushsext4() {
    assert!(test_push_unary(0xfff0, 0x0, 4, 2, OpCode::CPUI_INT_SEXT, 4));
}

#[test]
fn circlerange_pushsext5() {
    assert!(test_push_unary(0xffd1, 0xfff1, 4, 2, OpCode::CPUI_INT_SEXT, 4));
}

#[test]
fn circlerange_pushsext6() {
    assert!(test_push_unary(0, 0x30, 4, 1, OpCode::CPUI_INT_SEXT, 2));
}

#[test]
fn circlerange_pushsext7() {
    assert!(test_push_unary(0, 0, 4, 1, OpCode::CPUI_INT_SEXT, 2));
}

#[test]
fn circlerange_pushadd1() {
    assert!(test_push_binary(10, 15, 1, 30, 35, 1, 1, OpCode::CPUI_INT_ADD, 1));
}

#[test]
fn circlerange_pushadd2() {
    assert!(test_push_binary(1, 10, 1, 0xfffffffe, 5, 1, 4, OpCode::CPUI_INT_ADD, 4));
}

#[test]
fn circlerange_pushadd3() {
    assert!(test_push_binary(0, 20, 4, 0xfff0, 6, 2, 2, OpCode::CPUI_INT_ADD, 2));
}

#[test]
fn circlerange_pushadd4() {
    assert!(test_push_binary(1, 250, 1, 20, 30, 1, 1, OpCode::CPUI_INT_ADD, 1));
}

#[test]
fn circlerange_pushmult1() {
    assert!(test_push_binary(0x1000, 0x1010, 1, 2, 3, 1, 4, OpCode::CPUI_INT_MULT, 4));
}

#[test]
fn circlerange_pushmult2() {
    assert!(test_push_binary(0xfffc, 8, 2, 4, 5, 1, 2, OpCode::CPUI_INT_MULT, 2));
}

#[test]
fn circlerange_pushmult3() {
    assert!(test_push_binary(5, 133, 1, 2, 3, 1, 1, OpCode::CPUI_INT_MULT, 1));
}

#[test]
fn circlerange_pushleft1() {
    assert!(test_push_binary(1, 5, 1, 1, 2, 1, 4, OpCode::CPUI_INT_LEFT, 4));
}

#[test]
fn circlerange_pushleft2() {
    assert!(test_push_binary(8, 72, 4, 2, 3, 1, 1, OpCode::CPUI_INT_LEFT, 1));
}

#[test]
fn circlerange_pushsubpiece1() {
    assert!(test_push_binary(0xfffe, 0x10005, 1, 0, 1, 1, 4, OpCode::CPUI_SUBPIECE, 1));
}

#[test]
fn circlerange_pushsubpiece2() {
    assert!(test_push_binary(0xfffe, 0x10005, 1, 1, 2, 1, 4, OpCode::CPUI_SUBPIECE, 1));
}

#[test]
fn circlerange_pushsubpiece3() {
    assert!(test_push_binary(0x10f0, 0x1200, 1, 0, 1, 1, 4, OpCode::CPUI_SUBPIECE, 1));
}

#[test]
fn circlerange_pushright1() {
    assert!(test_push_binary(0x30a6, 0x30c0, 2, 4, 5, 1, 2, OpCode::CPUI_INT_RIGHT, 2));
}

#[test]
fn circlerange_pushright2() {
    assert!(test_push_binary(0xfe00, 0xffc0, 0x20, 9, 10, 1, 2, OpCode::CPUI_INT_RIGHT, 2));
}

#[test]
fn circlerange_pushright3() {
    assert!(test_push_binary(7, 10, 1, 4, 5, 1, 4, OpCode::CPUI_INT_RIGHT, 4));
}

#[test]
fn circlerange_pushsright1() {
    assert!(test_push_binary(0x3000, 0x3064, 4, 3, 4, 1, 2, OpCode::CPUI_INT_SRIGHT, 2));
}

#[test]
fn circlerange_pushsright2() {
    assert!(test_push_binary(0xfff0, 0x24, 4, 3, 4, 1, 2, OpCode::CPUI_INT_SRIGHT, 2));
}
