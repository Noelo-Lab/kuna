//! A comparison decided by the branches that dominate it (option
//! `opaquepredicate`).
//!
//! Control that reaches a block through one out-edge of a conditional branch
//! knows the result of that branch's comparison. When a later comparison in a
//! block that edge dominates is over the same two values, or over the same
//! value and another constant, the earlier results may leave it only one
//! outcome: inside `if (x > 5)`, `x <= 5` is false and `x != 3` is true.
//! Obfuscators plant such pairs so that the second branch looks live.
//!
//! The values are SSA Varnodes, so a value named by both comparisons is the
//! same value at both. Every fact is kept as the set of orderings (less,
//! equal, greater) the first operand may have relative to the second, signed
//! and unsigned separately; against a constant the facts are kept as an
//! interval of the variable.

use kuna_base::address::calc_mask;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_DOMINATORS: usize = 64;
const LT: u8 = 1;
const EQ: u8 = 2;
const GT: u8 = 4;
const ALL: u8 = LT | EQ | GT;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    Signed,
    Unsigned,
    Either,
}

/// `a` compared with `b`: true exactly when the ordering of `a` relative to
/// `b` is in `truth`.
#[derive(Clone, Copy)]
struct Compare {
    a: VarnodeId,
    b: VarnodeId,
    truth: u8,
    order: Order,
    size: int4,
}

fn swap(mask: u8) -> u8 {
    (mask & EQ) | if mask & LT != 0 { GT } else { 0 } | if mask & GT != 0 { LT } else { 0 }
}

fn read_compare(data: &Funcdata, op: OpId) -> Option<Compare> {
    let o = data.obank().get(op)?;
    let (truth, order) = match o.code() {
        OpCode::CPUI_INT_EQUAL => (EQ, Order::Either),
        OpCode::CPUI_INT_NOTEQUAL => (LT | GT, Order::Either),
        OpCode::CPUI_INT_LESS => (LT, Order::Unsigned),
        OpCode::CPUI_INT_LESSEQUAL => (LT | EQ, Order::Unsigned),
        OpCode::CPUI_INT_SLESS => (LT, Order::Signed),
        OpCode::CPUI_INT_SLESSEQUAL => (LT | EQ, Order::Signed),
        _ => return None,
    };
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    let size = data.vbank().get(a)?.get_size();
    (1..=8).contains(&size).then_some(Compare { a, b, truth, order, size })
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset() & calc_mask(v.get_size()))
}

/// The comparison a conditional branch tests and whether it holds along the
/// branch's out-edge `edge`.
fn branch_fact(data: &Funcdata, block: BlockId, edge: int4) -> Option<(Compare, bool)> {
    let branch = data.obank().get(data.bb_op_tail(block)?)?;
    if branch.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let mut holds = (edge == 1) != branch.is_boolean_flip();
    let mut vn = branch.get_in(1)?;
    for _ in 0..8 {
        let def = data.vbank().get(vn)?.get_def()?;
        let o = data.obank().get(def)?;
        match o.code() {
            OpCode::CPUI_BOOL_NEGATE => holds = !holds,
            OpCode::CPUI_COPY => {}
            _ => return read_compare(data, def).map(|compare| (compare, holds)),
        }
        vn = o.get_in(0)?;
    }
    None
}

/// The comparisons known where `block` starts, with their results: one for
/// each dominating conditional branch that reaches it through a single edge.
/// A successor of a dominator that itself dominates `block` is the next block
/// down the dominator chain, so the chain is all that is walked.
fn dominating_facts(data: &Funcdata, block: BlockId) -> Vec<(Compare, bool)> {
    let graph = data.bblocks_ref();
    let mut facts = Vec::new();
    let mut cur = block;
    for _ in 0..MAX_DOMINATORS {
        let Some(dom) = graph.block(cur).get_immed_dom() else { break };
        let d = graph.block(dom);
        if d.size_out() == 2 && d.get_out(0) != d.get_out(1) && graph.block(cur).size_in() == 1 {
            let edge = if d.get_out(0) == cur { 0 } else { 1 };
            if d.get_out(edge) == cur && cur != dom {
                facts.extend(branch_fact(data, dom, edge));
            }
        }
        cur = dom;
    }
    facts
}

fn signed(value: u64, size: int4) -> i128 {
    let shift = 64 - 8 * size as u32;
    i128::from(((value << shift) as i64) >> shift)
}

/// The values a variable may take in one ordering, as an inclusive interval
/// with at most a few excluded points.
struct Interval {
    lo: i128,
    hi: i128,
    excluded: Vec<i128>,
}

impl Interval {
    fn narrow(&mut self, orderings: u8, c: i128) {
        match orderings {
            LT => self.hi = self.hi.min(c - 1),
            m if m == LT | EQ => self.hi = self.hi.min(c),
            GT => self.lo = self.lo.max(c + 1),
            m if m == GT | EQ => self.lo = self.lo.max(c),
            EQ => {
                self.lo = self.lo.max(c);
                self.hi = self.hi.min(c);
            }
            m if m == LT | GT && self.excluded.len() < 8 => self.excluded.push(c),
            _ => {}
        }
    }

    /// The orderings the variable may have relative to `c`.
    fn orderings(&self, c: i128) -> u8 {
        let mut mask = 0;
        if self.lo < c {
            mask |= LT;
        }
        if self.hi > c {
            mask |= GT;
        }
        if self.lo <= c && c <= self.hi && !self.excluded.contains(&c) {
            mask |= EQ;
        }
        mask
    }
}

fn decide(possible: u8, truth: u8) -> Option<bool> {
    if possible == 0 {
        None
    } else if possible & !truth == 0 {
        Some(true)
    } else if possible & truth == 0 {
        Some(false)
    } else {
        None
    }
}

/// What a fact in ordering `have` says in ordering `want`: all of it in the
/// same ordering, and whether the operands are equal in the other.
fn carry(orderings: u8, have: Order, want: Order) -> u8 {
    if have == Order::Either || have == want {
        orderings
    } else {
        (orderings & EQ) | if orderings & (LT | GT) != 0 { LT | GT } else { 0 }
    }
}

/// A dominating comparison over the target's operands, oriented like the
/// target: the orderings its first operand may have relative to `bound`, or
/// relative to the target's second operand when `bound` is `None`.
#[derive(Clone, Copy)]
struct Fact {
    known: u8,
    order: Order,
    bound: Option<u64>,
}

/// The result of a `size`-byte comparison that is true exactly for the
/// orderings `truth` of its first operand relative to `bound` (or to its
/// second operand), when `facts` leave it only one.
fn implied(size: int4, truth: u8, order: Order, bound: Option<u64>, facts: &[Fact]) -> Option<bool> {
    let orders: &[Order] = match order {
        Order::Either => &[Order::Signed, Order::Unsigned],
        Order::Signed => &[Order::Signed],
        Order::Unsigned => &[Order::Unsigned],
    };
    let full = calc_mask(size);
    for &order in orders {
        let value = |c: u64| if order == Order::Signed { signed(c, size) } else { i128::from(c) };
        let (lo, hi) = if order == Order::Signed { (full ^ (full >> 1), full >> 1) } else { (0, full) };
        let mut interval = Interval { lo: value(lo), hi: value(hi), excluded: Vec::new() };
        let mut pair = ALL;
        for fact in facts {
            let known = carry(fact.known, fact.order, order);
            match fact.bound {
                Some(c) if bound.is_some() => interval.narrow(known, value(c)),
                None if bound.is_none() => pair &= known,
                _ => {}
            }
        }
        let possible = match bound {
            Some(c) if interval.lo <= interval.hi => interval.orderings(value(c)),
            Some(_) => 0,
            None => pair,
        };
        if let Some(result) = decide(possible, truth) {
            return Some(result);
        }
    }
    None
}

/// The result of comparison `op` when the conditional branches that dominate
/// its block leave it only one.
pub(crate) fn prove(data: &Funcdata, op: OpId) -> Option<bool> {
    let target = read_compare(data, op)?;
    let block = data.obank().get(op)?.get_parent()?;
    let (variable, bound, truth) = match (constant(data, target.a), constant(data, target.b)) {
        (Some(_), Some(_)) => return None,
        (None, Some(c)) => (target.a, Some(c), target.truth),
        (Some(c), None) => (target.b, Some(c), swap(target.truth)),
        (None, None) => (target.a, None, target.truth),
    };
    let mut facts = Vec::new();
    for (fact, holds) in dominating_facts(data, block) {
        if fact.size != target.size {
            continue;
        }
        let known = if holds { fact.truth } else { ALL & !fact.truth };
        let oriented = match (constant(data, fact.a), constant(data, fact.b)) {
            (None, Some(c)) if fact.a == variable => Some((known, Some(c))),
            (Some(c), None) if fact.b == variable => Some((swap(known), Some(c))),
            (None, None) if fact.a == target.a && fact.b == target.b => Some((known, None)),
            (None, None) if fact.a == target.b && fact.b == target.a => Some((swap(known), None)),
            _ => None,
        };
        facts.extend(oriented.map(|(known, bound)| Fact { known, order: fact.order, bound }));
    }
    if facts.is_empty() {
        return None;
    }
    implied(target.size, truth, target.order, bound, &facts)
}

#[cfg(test)]
mod tests;
