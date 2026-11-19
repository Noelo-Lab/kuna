//! P3 signed division and remainder by `2^k` written as a select (kuna `pow2select`).
//!
//! Signed `x / 2^k` rounds toward zero, so a compiler adds `2^k - 1` to a
//! negative `x` before the arithmetic shift.  Without a branchless sign mask it
//! selects the biased value with a conditional move (x86-64 `cmovns`, AArch64
//! `csel`), which lifts as a branch and a MULTIEQUAL:
//! `v = x + 3; if (0 <= x) v = x; return v >> 2;`.  Signed `x % 2^k` likewise
//! selects between `x & m` and `-(-x & m)` (AArch64 `csneg`).  A third form
//! needs no branch: `(x + ((x s>> (n-1)) >> (n-k))) s>> k` (ARM, x86 -O2 for
//! some widths).
//!
//! [`RuleSelectDiv`] rewrites the shift to `x / 2^k` (`INT_SDIV`) and replaces the
//! MULTIEQUAL of the remainder with `x % 2^k` (`INT_SREM`).  A select is only
//! taken when the branch feeding the MULTIEQUAL provably picks the unbiased
//! input for every `x > 0` and the other input for every `x < 0`: the branch
//! must be the MULTIEQUAL block's immediate dominator, each input must arrive
//! on its own out-edge (directly or through one block between), and the
//! condition must compare `x` or `-x` against a constant, whose `CircleRange`
//! pull-back gives the set of `x` sent down each edge.  At `x = 0` and at the
//! most negative `x` both inputs hold the same value, so either edge is fine.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::rangeutil::CircleRange;

/// Rewrite a signed division or remainder by `2^k` written as a select or a
/// sign-mask shift.
pub struct RuleSelectDiv {
    group: String,
}

impl RuleSelectDiv {
    pub fn new(group: impl Into<String>) -> RuleSelectDiv {
        RuleSelectDiv { group: group.into() }
    }
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

/// `vn` with any chain of `COPY`s that defined it skipped.
fn source(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut vn = vn;
    while let Some(op) = def_of(data, vn, OpCode::CPUI_COPY) {
        match input(data, op, 0) {
            Some(src) => vn = src,
            None => break,
        }
    }
    vn
}

/// Do `a` and `b` hold the same value through `COPY`s?
fn same(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    source(data, a) == source(data, b)
}

fn def_of(data: &Funcdata, vn: VarnodeId, opc: OpCode) -> Option<OpId> {
    let def = data.vbank().get(vn)?.get_def()?;
    (data.obank().get(def)?.code() == opc).then_some(def)
}

fn input(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op)?.get_in(slot)
}

fn size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map_or(0, |v| v.get_size())
}

/// `x` when `vn` is `x + c` (in either order).
fn added_to(data: &Funcdata, vn: VarnodeId, c: uintb) -> Option<VarnodeId> {
    let add = def_of(data, vn, OpCode::CPUI_INT_ADD)?;
    let (a, b) = (input(data, add, 0)?, input(data, add, 1)?);
    if constant(data, b) == Some(c) {
        Some(a)
    } else if constant(data, a) == Some(c) {
        Some(b)
    } else {
        None
    }
}

/// `x` when `vn` is `-x`, as `INT_2COMP` or a multiply by all-ones.
fn negation_of(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    if let Some(op) = def_of(data, vn, OpCode::CPUI_INT_2COMP) {
        return input(data, op, 0);
    }
    let mult = def_of(data, vn, OpCode::CPUI_INT_MULT)?;
    let mask = kuna_base::address::calc_mask(size(data, vn));
    (constant(data, input(data, mult, 1)?) == Some(mask)).then(|| input(data, mult, 0)).flatten()
}

fn last_op(data: &Funcdata, bl: BlockId) -> Option<OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_tail,
        _ => None,
    }
}

/// For a two-input MULTIEQUAL, the CBRANCH condition of its block's immediate
/// dominator and the condition's value on the edge each input arrives by.
fn select_condition(data: &Funcdata, phi: OpId) -> Option<(VarnodeId, [bool; 2])> {
    let graph = data.bblocks_ref();
    let bl = data.obank().get(phi)?.get_parent()?;
    if graph.block(bl).size_in() != 2 || data.obank().get(phi)?.num_input() != 2 {
        return None;
    }
    let dom = graph.block(bl).get_immed_dom()?;
    let branch = data.obank().get(last_op(data, dom)?)?;
    if branch.code() != OpCode::CPUI_CBRANCH || graph.block(dom).size_out() != 2 {
        return None;
    }
    let outs = [graph.block(dom).get_out(0), graph.block(dom).get_out(1)];
    if outs[0] == outs[1] {
        return None;
    }
    let mut values = [false; 2];
    let mut edges = [0usize; 2];
    for i in 0..2 {
        let pred = graph.block(bl).get_in(i as int4);
        let via = if pred == dom {
            bl
        } else {
            let p = graph.block(pred);
            if p.size_in() != 1 || p.get_in(0) != dom || p.size_out() != 1 {
                return None;
            }
            pred
        };
        edges[i] = outs.iter().position(|&o| o == via)?;
        values[i] = (edges[i] == 1) != branch.is_boolean_flip();
    }
    (edges[0] != edges[1]).then(|| branch.get_in(1)).flatten().map(|c| (c, values))
}

/// The values of `x` for which `cond` is `value`, when `cond` compares `x` or
/// `-x` against a constant.
fn x_set(data: &Funcdata, cond: VarnodeId, value: bool, x: VarnodeId) -> Option<CircleRange> {
    let mut cond = cond;
    let mut value = value;
    while let Some(neg) = def_of(data, cond, OpCode::CPUI_BOOL_NEGATE) {
        value = !value;
        cond = input(data, neg, 0)?;
    }
    let cmp = data.vbank().get(cond)?.get_def()?;
    let o = data.obank().get(cmp)?;
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    let (z, k, slot) = match (constant(data, a), constant(data, b)) {
        (None, Some(k)) => (a, k, 0),
        (Some(k), None) => (b, k, 1),
        _ => return None,
    };
    let n = size(data, x);
    let mut range = CircleRange::new_bool(value);
    if !range.pull_back_binary(o.code(), k, slot, n, 1) {
        return None;
    }
    if same(data, z, x) {
        return Some(range);
    }
    if negation_of(data, z).is_some_and(|y| same(data, y, x)) && range.pull_back_unary(OpCode::CPUI_INT_2COMP, n, n) {
        return Some(range);
    }
    None
}

/// Does the select send every positive `x` to input `pos` of `phi` and every
/// negative `x` (but the most negative) to the other input?
fn picks_by_sign(data: &Funcdata, phi: OpId, pos: usize, x: VarnodeId) -> bool {
    let Some((cond, values)) = select_condition(data, phi) else { return false };
    let n = size(data, x);
    let half: uintb = 1 << (n * 8 - 1);
    let positive = CircleRange::new(1, half, n, 1);
    let negative = CircleRange::new(half + 1, 0, n, 1);
    match (x_set(data, cond, values[pos], x), x_set(data, cond, values[1 - pos], x)) {
        (Some(p), Some(q)) => p.contains(&positive) && q.contains(&negative),
        _ => false,
    }
}

/// `x` and `k` when `op` is `INT_SRIGHT` computing `x / 2^k`.
fn division(data: &Funcdata, op: OpId) -> Option<(VarnodeId, int4)> {
    let shifted = input(data, op, 0)?;
    let k = constant(data, input(data, op, 1)?)?;
    let bits = size(data, shifted) as uintb * 8;
    if k == 0 || k >= bits - 1 {
        return None;
    }
    let bias = (1 << k) - 1;
    if let Some(phi) = def_of(data, shifted, OpCode::CPUI_MULTIEQUAL) {
        for pos in 0..2 {
            let x = input(data, phi, pos as int4)?;
            let biased = added_to(data, input(data, phi, 1 - pos as int4)?, bias);
            if biased.is_some_and(|b| same(data, b, x)) && picks_by_sign(data, phi, pos, x) {
                return Some((x, k as int4));
            }
        }
        return None;
    }
    let add = def_of(data, shifted, OpCode::CPUI_INT_ADD)?;
    for slot in 0..2 {
        let x = input(data, add, slot)?;
        let round = input(data, add, 1 - slot)?;
        let right = def_of(data, round, OpCode::CPUI_INT_RIGHT)?;
        if constant(data, input(data, right, 1)?) != Some(bits - k) {
            continue;
        }
        let sign = def_of(data, input(data, right, 0)?, OpCode::CPUI_INT_SRIGHT)?;
        if same(data, input(data, sign, 0)?, x) && constant(data, input(data, sign, 1)?) == Some(bits - 1) {
            return Some((x, k as int4));
        }
    }
    None
}

/// `x` and `m` when the MULTIEQUAL `phi` selects `x & m` or `-(-x & m)`, `m = 2^k - 1`.
fn remainder(data: &Funcdata, phi: OpId) -> Option<(VarnodeId, uintb)> {
    for pos in 0..2 {
        let Some(and) = def_of(data, input(data, phi, pos as int4)?, OpCode::CPUI_INT_AND) else { continue };
        let x = input(data, and, 0)?;
        let Some(m) = constant(data, input(data, and, 1)?) else { continue };
        if m == 0 || m & (m + 1) != 0 || m >= 1 << (size(data, x) * 8 - 1) {
            continue;
        }
        let Some(neg_and) = negation_of(data, input(data, phi, 1 - pos as int4)?) else { continue };
        let Some(and2) = def_of(data, neg_and, OpCode::CPUI_INT_AND) else { continue };
        if constant(data, input(data, and2, 1)?) != Some(m) {
            continue;
        }
        let y = input(data, and2, 0)?;
        let magnitude = negation_of(data, y).is_some_and(|v| same(data, v, x)) || (m == 1 && same(data, y, x));
        if magnitude && picks_by_sign(data, phi, pos, x) {
            return Some((x, m));
        }
    }
    None
}

impl Rule for RuleSelectDiv {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT, OpCode::CPUI_MULTIEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleSelectDiv { group: self.group.clone() }) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().pow2_select {
            return 0;
        }
        match data.obank().get(op).map(|o| o.code()) {
            Some(OpCode::CPUI_INT_SRIGHT) => {
                let Some((x, k)) = division(data, op) else { return 0 };
                let divisor = data.new_constant(size(data, x), 1 << k);
                data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_INT_SDIV));
                data.op_set_input(op, x, 0).expect("selectdiv: dividend");
                data.op_set_input(op, divisor, 1).expect("selectdiv: divisor");
                1
            }
            Some(OpCode::CPUI_MULTIEQUAL) => {
                let Some((x, m)) = remainder(data, op) else { return 0 };
                let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) else { return 0 };
                let bl = data.obank().get(op).and_then(|o| o.get_parent()).unwrap();
                let addr = data.obank().get(op).unwrap().get_addr().clone();
                let rem = data.new_op(2, addr);
                data.op_set_opcode(rem, crate::typeop::type_op_for(OpCode::CPUI_INT_SREM));
                let n = size(data, x);
                let rem_out = data.new_unique_out(n, rem).expect("selectdiv: output");
                let modulus = data.new_constant(n, m + 1);
                data.op_set_input(rem, x, 0).expect("selectdiv: dividend");
                data.op_set_input(rem, modulus, 1).expect("selectdiv: modulus");
                data.op_insert_begin(rem, bl);
                data.total_replace(out, rem_out).expect("selectdiv: replace");
                1
            }
            _ => 0,
        }
    }
}
