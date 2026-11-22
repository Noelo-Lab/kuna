//! Fold a comparison that has the same result for every input (option
//! `opaquepredicate`, default off).
//!
//! Only exact proofs are accepted:
//!
//! - **Low bits.** Bit `i` of a sum, difference, product, left shift or bitwise
//!   operation depends only on bits `0..=i` of its operands. A comparison whose
//!   sides are constants or values masked to their low `k <= 8` bits is
//!   therefore decided by the low `k` bits of at most two inputs, and all
//!   `2^k` (or `4^k`) assignments are evaluated.
//! - **Linear combination of bitwise terms.** A sum of constant multiples of
//!   `AND`/`OR`/`XOR`/`NOT` terms over the same inputs has one value for every
//!   input exactly when it has that value with every input all zeros or all
//!   ones. An equality whose two sides are such sums is decided on those
//!   assignments.
//!
//! An equality is also never true when the low four bits of its sides differ
//! for every assignment of the low four bits of the inputs.
//!
//! A proven comparison becomes a `COPY` of the constant; constant branch
//! removal then deletes the arm that never runs.
//!
//! The rule stands down inside jump-table recovery: that copy of the function
//! lacks the edges back from the table's targets, so a switch index set by
//! `xor ebx,ebx` reads as zero there and folding its guard would drop the
//! table's last case.

use std::collections::BTreeMap;

use kuna_base::address::calc_mask;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_INPUTS: usize = 2;
const MAX_NODES: usize = 48;
const MAX_LOW_BITS: u32 = 8;
const DIFFER_BITS: u32 = 4;

#[derive(Clone, Copy)]
enum Step {
    Input(usize),
    Const(u64),
    Unary(OpCode, usize),
    Binary(OpCode, usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Layer {
    LowBits,
    Linear,
    Bitwise,
}

/// A straight-line program over the inputs of one comparison.
struct Program<'a> {
    data: &'a Funcdata,
    steps: Vec<Step>,
    inputs: Vec<VarnodeId>,
    memo: BTreeMap<(VarnodeId, Layer), usize>,
}

impl<'a> Program<'a> {
    fn new(data: &'a Funcdata) -> Program<'a> {
        Program { data, steps: Vec::new(), inputs: Vec::new(), memo: BTreeMap::new() }
    }

    fn push(&mut self, step: Step) -> Option<usize> {
        if self.steps.len() >= MAX_NODES {
            return None;
        }
        self.steps.push(step);
        Some(self.steps.len() - 1)
    }

    fn constant(&self, vn: VarnodeId) -> Option<u64> {
        let v = self.data.vbank().get(vn)?;
        (v.is_constant() && (1..=8).contains(&v.get_size())).then(|| v.get_offset() & calc_mask(v.get_size()))
    }

    fn uniform_constant(&self, vn: VarnodeId) -> bool {
        let Some(v) = self.data.vbank().get(vn) else { return false };
        !v.is_constant() || v.get_offset() == 0 || v.get_offset() == calc_mask(v.get_size())
    }

    /// Which operations `layer` reads through; everything else is an input.
    fn expands(&self, op: OpId, layer: Layer) -> Option<Layer> {
        let o = self.data.obank().get(op)?;
        let constant_at = |slot: int4| o.get_in(slot).and_then(|v| self.constant(v)).is_some();
        let bitwise = |uniform: bool| match o.code() {
            OpCode::CPUI_INT_NEGATE => true,
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                !uniform || (0..2).all(|slot| o.get_in(slot).is_some_and(|v| self.uniform_constant(v)))
            }
            _ => false,
        };
        match layer {
            Layer::LowBits => match o.code() {
                OpCode::CPUI_COPY
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_INT_2COMP
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_MULT => Some(layer),
                OpCode::CPUI_INT_LEFT => constant_at(1).then_some(layer),
                OpCode::CPUI_SUBPIECE => {
                    o.get_in(1).and_then(|v| self.constant(v)).is_some_and(|offset| offset == 0).then_some(layer)
                }
                _ => bitwise(false).then_some(layer),
            },
            Layer::Linear => match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                    Some(layer)
                }
                OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_LEFT => constant_at(1).then_some(layer),
                _ => bitwise(true).then_some(Layer::Bitwise),
            },
            Layer::Bitwise => (o.code() == OpCode::CPUI_COPY || bitwise(true)).then_some(layer),
        }
    }

    fn compile(&mut self, vn: VarnodeId, layer: Layer) -> Option<usize> {
        if let Some(value) = self.constant(vn) {
            return (layer != Layer::Bitwise || self.uniform_constant(vn)).then(|| self.push(Step::Const(value)))?;
        }
        let v = self.data.vbank().get(vn)?;
        if v.is_constant() || v.is_free() || !(1..=8).contains(&v.get_size()) {
            return None;
        }
        let expanded = v.get_def().and_then(|def| Some((def, self.expands(def, layer)?)));
        let Some((def, layer)) = expanded else {
            let slot = match self.inputs.iter().position(|&input| input == vn) {
                Some(slot) => slot,
                None if self.inputs.len() < MAX_INPUTS => {
                    self.inputs.push(vn);
                    self.inputs.len() - 1
                }
                None => return None,
            };
            return self.push(Step::Input(slot));
        };
        if let Some(&index) = self.memo.get(&(vn, layer)) {
            return Some(index);
        }
        let o = self.data.obank().get(def)?;
        let (code, first, second) = (o.code(), o.get_in(0)?, (o.num_input() > 1).then(|| o.get_in(1)).flatten());
        let a = self.compile(first, layer)?;
        let index = match (code, second) {
            (OpCode::CPUI_SUBPIECE, _) => a,
            (_, Some(second)) => {
                let b = self.compile(second, layer)?;
                self.push(Step::Binary(code, a, b))?
            }
            (_, None) => self.push(Step::Unary(code, a))?,
        };
        self.memo.insert((vn, layer), index);
        Some(index)
    }

    /// Run the program with `values[i]` as input `i`; arithmetic wraps at 64 bits.
    fn run(&self, values: &[u64; MAX_INPUTS], slots: &mut Vec<u64>) {
        slots.clear();
        for step in &self.steps {
            let value = match *step {
                Step::Input(slot) => values[slot],
                Step::Const(value) => value,
                Step::Unary(OpCode::CPUI_INT_2COMP, a) => slots[a].wrapping_neg(),
                Step::Unary(OpCode::CPUI_INT_NEGATE, a) => !slots[a],
                Step::Unary(_, a) => slots[a],
                Step::Binary(code, a, b) => {
                    let (a, b) = (slots[a], slots[b]);
                    match code {
                        OpCode::CPUI_INT_ADD => a.wrapping_add(b),
                        OpCode::CPUI_INT_SUB => a.wrapping_sub(b),
                        OpCode::CPUI_INT_MULT => a.wrapping_mul(b),
                        OpCode::CPUI_INT_LEFT => u32::try_from(b).ok().and_then(|b| a.checked_shl(b)).unwrap_or(0),
                        OpCode::CPUI_INT_AND => a & b,
                        OpCode::CPUI_INT_OR => a | b,
                        _ => a ^ b,
                    }
                }
            };
            slots.push(value);
        }
    }
}

/// One side of a low-bits comparison. Its value is `slot & mask`, except that
/// after a signed remainder (`zero_only`) a nonzero `slot & mask` only says
/// the value is not zero.
struct Side {
    slot: usize,
    mask: u64,
    constant: Option<u64>,
    zero_only: bool,
}

/// A constant, or a value reduced to low bits by `INT_AND` or a remainder by a
/// power of two, seen through zero extensions.
fn low_side(program: &mut Program, mut vn: VarnodeId) -> Option<Side> {
    let data = program.data;
    if let Some(value) = program.constant(vn) {
        let slot = program.push(Step::Const(value))?;
        return Some(Side { slot, mask: u64::MAX, constant: Some(value), zero_only: false });
    }
    loop {
        let o = data.obank().get(data.vbank().get(vn)?.get_def()?)?;
        let code = o.code();
        let mask = match code {
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_COPY => {
                vn = o.get_in(0)?;
                continue;
            }
            OpCode::CPUI_INT_AND => program.constant(o.get_in(1)?)?,
            OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SREM => {
                let modulus = program.constant(o.get_in(1)?)?;
                modulus.is_power_of_two().then(|| modulus - 1)?
            }
            _ => return None,
        };
        if mask >> MAX_LOW_BITS != 0 {
            return None;
        }
        let slot = program.compile(o.get_in(0)?, Layer::LowBits)?;
        return Some(Side { slot, mask, constant: None, zero_only: code == OpCode::CPUI_INT_SREM });
    }
}

fn compare(code: OpCode, a: u64, b: u64) -> bool {
    match code {
        OpCode::CPUI_INT_EQUAL => a == b,
        OpCode::CPUI_INT_NOTEQUAL => a != b,
        OpCode::CPUI_INT_LESS => a < b,
        _ => a <= b,
    }
}

/// The comparison's result when the low bits of its inputs decide it and every
/// assignment of them agrees.
fn prove_low_bits(data: &Funcdata, code: OpCode, in0: VarnodeId, in1: VarnodeId) -> Option<bool> {
    let mut program = Program::new(data);
    let left = low_side(&mut program, in0)?;
    let right = low_side(&mut program, in1)?;
    if program.inputs.is_empty() {
        return None;
    }
    let zero_test = matches!(code, OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL)
        && (left.constant == Some(0) || right.constant == Some(0));
    let bits = [&left, &right]
        .iter()
        .filter(|side| side.constant.is_none())
        .map(|side| 64 - side.mask.leading_zeros())
        .max()?;
    let span = 1u64 << bits;
    let second = if program.inputs.len() > 1 { span } else { 1 };
    let mut slots = Vec::with_capacity(program.steps.len());
    let mut verdict = None;
    for x in 0..span {
        for y in 0..second {
            program.run(&[x, y], &mut slots);
            let (a, b) = (slots[left.slot] & left.mask, slots[right.slot] & right.mask);
            if !zero_test && (left.zero_only && a != 0 || right.zero_only && b != 0) {
                return None;
            }
            let result = compare(code, a, b);
            if *verdict.get_or_insert(result) != result {
                return None;
            }
        }
    }
    verdict
}

/// `Some(false)` when the low [`DIFFER_BITS`] bits of the two sides differ for
/// every assignment of the low bits of the inputs, so the sides are never equal.
fn prove_never_equal(data: &Funcdata, in0: VarnodeId, in1: VarnodeId) -> Option<bool> {
    let mut program = Program::new(data);
    let left = program.compile(in0, Layer::LowBits)?;
    let right = program.compile(in1, Layer::LowBits)?;
    if program.inputs.is_empty() {
        return None;
    }
    let span = 1u64 << DIFFER_BITS;
    let second = if program.inputs.len() > 1 { span } else { 1 };
    let mut slots = Vec::with_capacity(program.steps.len());
    for x in 0..span {
        for y in 0..second {
            program.run(&[x, y], &mut slots);
            if (slots[left] ^ slots[right]) & (span - 1) == 0 {
                return None;
            }
        }
    }
    Some(false)
}

/// Whether the two sides are equal for every input, when both are linear
/// combinations of bitwise terms whose difference is one constant.
fn prove_linear(data: &Funcdata, in0: VarnodeId, in1: VarnodeId) -> Option<bool> {
    let size = data.vbank().get(in0)?.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    let mut program = Program::new(data);
    let left = program.compile(in0, Layer::Linear)?;
    let right = program.compile(in1, Layer::Linear)?;
    if program.inputs.is_empty() {
        return None;
    }
    let mut slots = Vec::with_capacity(program.steps.len());
    let mut difference = None;
    for corner in 0..1u64 << program.inputs.len() {
        let values = [0u64.wrapping_sub(corner & 1), 0u64.wrapping_sub(corner >> 1 & 1)];
        program.run(&values, &mut slots);
        let delta = slots[left].wrapping_sub(slots[right]) & calc_mask(size);
        if *difference.get_or_insert(delta) != delta {
            return None;
        }
    }
    difference.map(|delta| delta == 0)
}

/// The constant value of comparison `op`, if one of the two proofs finds it.
fn constant_result(data: &Funcdata, op: OpId) -> Option<bool> {
    let o = data.obank().get(op)?;
    let (code, in0, in1) = (o.code(), o.get_in(0)?, o.get_in(1)?);
    if matches!(code, OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL) {
        if let Some(equal) = prove_linear(data, in0, in1).or_else(|| prove_never_equal(data, in0, in1)) {
            return Some(equal == (code == OpCode::CPUI_INT_EQUAL));
        }
    }
    prove_low_bits(data, code, in0, in1)
}

/// Replace a comparison that is constant for every input by that constant.
pub struct RuleOpaquePredicate {
    group: String,
}

impl RuleOpaquePredicate {
    pub fn new(group: &str) -> RuleOpaquePredicate {
        RuleOpaquePredicate { group: group.to_string() }
    }
}

impl Rule for RuleOpaquePredicate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_INT_LESSEQUAL,
        ]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleOpaquePredicate::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().opaque_predicate || data.is_jumptable_recovery_on() {
            return 0;
        }
        let Some(result) = constant_result(data, op) else { return 0 };
        let out = data.obank().get(op).and_then(|o| o.get_out());
        let Some(size) = out.and_then(|out| data.vbank().get(out)).map(|out| out.get_size()) else { return 0 };
        let constant = data.new_constant(size, u64::from(result));
        data.op_remove_input(op, 1);
        data.op_set_input(op, constant, 0).expect("RuleOpaquePredicate: opSetInput");
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        1
    }
}
