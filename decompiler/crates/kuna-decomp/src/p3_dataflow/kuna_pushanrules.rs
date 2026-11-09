//! Arithmetic residue left in a devirtualized (pushan) body.
//!
//! VMProtect builds every logic operation from NOR gates (`NOT a = NOR(a,a)`),
//! spells `x - y` as `~(~x + y)`, and a value its handlers split across two
//! stack cells comes back as a PIECE of slices of one value. The rules here undo
//! those spellings, in the manner of angr's Pushan simplifiers (the signature
//! search of `MBATemplateSimplifier`/`LinearMBASolver`, and the peepholes
//! `XorSimplifications`, `RemoveRedundantNegations`, `VMPSubAsNot`,
//! `NegPlusConstToSub`, `VMInsertSimplifier`, `AndMaskPruning`,
//! `DisjointAddToOr`). They act only inside a devirtualized
//! function ([`Funcdata::kuna_devirtualized`]) and every rewrite is an exact
//! bitvector identity that makes the expression strictly smaller.
//!
//! * [`RulePushanLogic`] evaluates a tree of AND/OR/XOR/NOT (and additions of
//!   operands with disjoint non-zero masks, through truncations and zero
//!   extensions) bit-parallel over every 0/1
//!   assignment of its distinct leaves, groups bit positions by the boolean
//!   function they compute, and replaces the tree with the cheapest equivalent
//!   built from minimal formulas of at most three leaves and constant masks.
//! * [`RulePushanArith`] rewrites a sum that contains a bitwise NOT as its linear
//!   form (`~x = -x - 1`), cancelling equal terms.
//! * [`RulePushanPiece`] folds a PIECE of two slices of one value back into the
//!   value, its shift or its truncation.
//! * [`RulePushanCarry`] clears the carry and signed overflow of an addition
//!   whose operands share no set bit, such as `x + ~x`.

use std::sync::OnceLock;

use kuna_base::address::calc_mask;
use kuna_num::opcodes::OpCode::{self, *};

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_NODES: usize = 64;
const MAX_VARS: usize = 6;
const SAME_DEPTH: u32 = 5;

fn size(data: &Funcdata, vn: VarnodeId) -> i32 {
    data.vbank().get(vn).unwrap().get_size()
}

fn constant_value(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn).unwrap();
    v.is_constant().then(|| v.get_offset())
}

fn def_of(data: &Funcdata, vn: VarnodeId) -> Option<(OpCode, Vec<VarnodeId>)> {
    let op = data.vbank().get(vn)?.get_def()?;
    let o = data.obank().get(op)?;
    let inputs = (0..o.num_input()).map(|i| o.get_in(i)).collect::<Option<Vec<_>>>()?;
    Some((o.code(), inputs))
}

fn unmovable(data: &Funcdata, vn: VarnodeId) -> bool {
    let v = data.vbank().get(vn).unwrap();
    v.is_free() && !v.is_constant()
}

fn set_op(data: &mut Funcdata, op: OpId, code: OpCode, inputs: &[VarnodeId]) {
    data.op_set_all_input(op, inputs).expect("pushan rule inputs");
    data.op_set_opcode(op, crate::typeop::seam_type_op_for(code));
}

fn insert(data: &mut Funcdata, before: OpId, code: OpCode, width: i32, inputs: &[VarnodeId]) -> VarnodeId {
    let address = data.obank().get(before).unwrap().get_addr().clone();
    let op = data.new_op(inputs.len() as i32, address);
    set_op(data, op, code, inputs);
    let out = data.new_unique_out(width, op).expect("pushan rule output");
    data.op_insert_before(op, before);
    out
}

fn pure(code: OpCode) -> bool {
    matches!(
        code,
        CPUI_COPY
            | CPUI_INT_ADD
            | CPUI_INT_SUB
            | CPUI_INT_MULT
            | CPUI_INT_AND
            | CPUI_INT_OR
            | CPUI_INT_XOR
            | CPUI_INT_NEGATE
            | CPUI_INT_2COMP
            | CPUI_INT_LEFT
            | CPUI_INT_RIGHT
            | CPUI_INT_SRIGHT
            | CPUI_INT_ZEXT
            | CPUI_INT_SEXT
            | CPUI_PIECE
            | CPUI_SUBPIECE
            | CPUI_INT_EQUAL
            | CPUI_INT_NOTEQUAL
            | CPUI_INT_LESS
            | CPUI_INT_LESSEQUAL
            | CPUI_INT_SLESS
            | CPUI_INT_SLESSEQUAL
            | CPUI_INT_CARRY
            | CPUI_INT_SCARRY
            | CPUI_INT_SBORROW
            | CPUI_BOOL_NEGATE
            | CPUI_BOOL_AND
            | CPUI_BOOL_OR
            | CPUI_BOOL_XOR
            | CPUI_POPCOUNT
            | CPUI_LZCOUNT
            | CPUI_CAST
    )
}

fn commutative(code: OpCode) -> bool {
    matches!(
        code,
        CPUI_INT_ADD
            | CPUI_INT_MULT
            | CPUI_INT_AND
            | CPUI_INT_OR
            | CPUI_INT_XOR
            | CPUI_INT_EQUAL
            | CPUI_INT_NOTEQUAL
            | CPUI_INT_CARRY
            | CPUI_INT_SCARRY
            | CPUI_BOOL_AND
            | CPUI_BOOL_OR
            | CPUI_BOOL_XOR
    )
}

/// Do `a` and `b` hold the same value? Structural comparison through pure ops,
/// `depth` levels deep; the interpreter recomputes one value from separate loads
/// of the same VM cell, so equal operands are rarely the same varnode.
fn same_value(data: &Funcdata, a: VarnodeId, b: VarnodeId, depth: u32) -> bool {
    if a == b {
        return true;
    }
    let (va, vb) = (data.vbank().get(a).unwrap(), data.vbank().get(b).unwrap());
    if va.get_size() != vb.get_size() {
        return false;
    }
    if va.is_constant() || vb.is_constant() {
        return va.is_constant() && vb.is_constant() && va.get_offset() == vb.get_offset();
    }
    if depth == 0 {
        return false;
    }
    let (Some((ca, ia)), Some((cb, ib))) = (def_of(data, a), def_of(data, b)) else {
        return false;
    };
    if ca != cb || !pure(ca) || ia.len() != ib.len() {
        return false;
    }
    if ia.iter().zip(&ib).all(|(&x, &y)| same_value(data, x, y, depth - 1)) {
        return true;
    }
    commutative(ca)
        && ia.len() == 2
        && same_value(data, ia[0], ib[1], depth - 1)
        && same_value(data, ia[1], ib[0], depth - 1)
}

fn devirtualized_only(data: &Funcdata) -> bool {
    data.kuna_devirtualized()
}

macro_rules! pushan_rule {
    ($name:ident) => {
        pub struct $name {
            group: String,
        }

        impl $name {
            pub fn new(group: impl Into<String>) -> Self {
                Self { group: group.into() }
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Minimal formulas of up to three variables
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Form {
    Zero,
    Ones,
    Var(u8),
    Not(u16),
    Bin(OpCode, u16, u16),
}

/// The minimal NOT/AND/OR/XOR formula (each operator costing one) of every
/// boolean function of `n` variables, by truth table; bit `s` of a truth table is
/// the value at the assignment whose bit `j` is variable `j`.
fn formulas(n: usize) -> &'static [Form] {
    static TABLES: [OnceLock<Vec<Form>>; 4] = [OnceLock::new(), OnceLock::new(), OnceLock::new(), OnceLock::new()];
    TABLES[n].get_or_init(|| {
        let rows = 1usize << n;
        let full: u16 = ((1u32 << rows) - 1) as u16;
        let mut best: Vec<Option<Form>> = vec![None; 1 << rows];
        let mut levels: Vec<Vec<u16>> = vec![Vec::new()];
        let seed = |tt: u16, f: Form, best: &mut Vec<Option<Form>>, level: &mut Vec<u16>| {
            if best[tt as usize].is_none() {
                best[tt as usize] = Some(f);
                level.push(tt);
            }
        };
        let mut level0 = Vec::new();
        seed(0, Form::Zero, &mut best, &mut level0);
        seed(full, Form::Ones, &mut best, &mut level0);
        for j in 0..n {
            let tt = (0..rows).filter(|s| s >> j & 1 == 1).fold(0u16, |t, s| t | 1 << s);
            seed(tt, Form::Var(j as u8), &mut best, &mut level0);
        }
        levels[0] = level0;
        let mut found = levels[0].len();
        let mut cost = 1;
        while found < best.len() && cost < 16 {
            let mut level = Vec::new();
            for &tt in &levels[cost - 1] {
                seed(!tt & full, Form::Not(tt), &mut best, &mut level);
            }
            for c1 in 0..cost {
                let c2 = cost - 1 - c1;
                if c2 < c1 {
                    break;
                }
                for i in 0..levels[c1].len() {
                    let a = levels[c1][i];
                    for j in 0..levels[c2].len() {
                        let b = levels[c2][j];
                        seed(a & b, Form::Bin(CPUI_INT_AND, a, b), &mut best, &mut level);
                        seed(a | b, Form::Bin(CPUI_INT_OR, a, b), &mut best, &mut level);
                        seed(a ^ b, Form::Bin(CPUI_INT_XOR, a, b), &mut best, &mut level);
                    }
                }
            }
            found += level.len();
            levels.push(level);
            cost += 1;
        }
        best.into_iter().map(|f| f.expect("every boolean function has a formula")).collect()
    })
}

// ---------------------------------------------------------------------------
// Expressions over leaves (candidate rewrites)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum Expr {
    Leaf(usize),
    Const(u64),
    Not(Box<Expr>),
    Bin(OpCode, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn from_form(n: usize, tt: u16, vars: &[usize]) -> Expr {
        match formulas(n)[tt as usize] {
            Form::Zero => Expr::Const(0),
            Form::Ones => Expr::Const(u64::MAX),
            Form::Var(j) => Expr::Leaf(vars[j as usize]),
            Form::Not(a) => Expr::Not(Box::new(Expr::from_form(n, a, vars))),
            Form::Bin(op, a, b) => {
                Expr::Bin(op, Box::new(Expr::from_form(n, a, vars)), Box::new(Expr::from_form(n, b, vars)))
            }
        }
    }

    fn bin(op: OpCode, a: Expr, b: Expr) -> Expr {
        Expr::Bin(op, Box::new(a), Box::new(b))
    }

    fn is_const(&self) -> bool {
        match self {
            Expr::Const(_) => true,
            Expr::Not(a) => a.is_const(),
            Expr::Bin(_, a, b) => a.is_const() && b.is_const(),
            Expr::Leaf(_) => false,
        }
    }

    /// Cost in half-operator units, constant subexpressions folding for free;
    /// `leaf` is what reading each leaf costs (a truncation, or nothing).
    fn cost(&self, leaf: &[u32]) -> u32 {
        if self.is_const() {
            return 0;
        }
        match self {
            Expr::Leaf(i) => leaf[*i],
            Expr::Const(_) => 0,
            Expr::Not(a) => 2 + a.cost(leaf),
            Expr::Bin(_, a, b) => 2 + a.cost(leaf) + b.cost(leaf),
        }
    }

    fn eval(&self, leaf: &[u64], mask: u64) -> u64 {
        match self {
            Expr::Leaf(i) => leaf[*i],
            Expr::Const(c) => c & mask,
            Expr::Not(a) => !a.eval(leaf, mask) & mask,
            Expr::Bin(op, a, b) => apply(*op, a.eval(leaf, mask), b.eval(leaf, mask)),
        }
    }
}

fn apply(op: OpCode, a: u64, b: u64) -> u64 {
    match op {
        CPUI_INT_AND => a & b,
        CPUI_INT_XOR => a ^ b,
        _ => a | b,
    }
}

// ---------------------------------------------------------------------------
// RulePushanLogic
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Node {
    Leaf(usize),
    Not(usize, u64),
    Bin(OpCode, usize, usize),
}

/// A leaf, seen through the truncations and zero extensions above it: only its
/// low `seen` bytes reach the root, zero-extended to the root's width.
struct LeafVal {
    vn: VarnodeId,
    constant: Option<u64>,
    nz: u64,
    size: i32,
    seen: i32,
}

struct LogicTree {
    nodes: Vec<Node>,
    leaves: Vec<LeafVal>,
    mask: u64,
    width: i32,
    cost: u32,
}

/// Is `vn` an operator of the logic tree: AND/OR/XOR/NOT, or an addition
/// merging two masked values that can never carry into each other?
fn logic_def(data: &Funcdata, vn: VarnodeId) -> Option<(OpCode, Vec<VarnodeId>)> {
    let (code, inputs) = def_of(data, vn)?;
    if size(data, vn) > 8 {
        return None;
    }
    match code {
        CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_NEGATE => Some((code, inputs)),
        CPUI_INT_ADD => {
            let nz = |v: VarnodeId| data.vbank().get(v).unwrap().get_nz_mask();
            let logic = |v: VarnodeId| {
                def_of(data, v).is_some_and(|(c, _)| matches!(c, CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_NEGATE))
            };
            let merge = inputs.iter().all(|&v| constant_value(data, v).is_none())
                && inputs.iter().any(|&v| logic(v))
                && nz(inputs[0]) & nz(inputs[1]) == 0;
            merge.then_some((code, inputs))
        }
        _ => None,
    }
}

impl LogicTree {
    fn leaf(&mut self, data: &Funcdata, vn: VarnodeId, seen: i32) -> Option<usize> {
        let seen = seen.min(size(data, vn));
        let index = match constant_value(data, vn) {
            Some(c) => {
                let c = c & calc_mask(seen);
                match self.leaves.iter().position(|l| l.constant == Some(c)) {
                    Some(i) => i,
                    None => {
                        self.leaves.push(LeafVal { vn, constant: Some(c), nz: c, size: seen, seen });
                        self.leaves.len() - 1
                    }
                }
            }
            None => {
                if unmovable(data, vn) {
                    return None;
                }
                let same = |l: &LeafVal| l.constant.is_none() && l.seen == seen && same_value(data, l.vn, vn, SAME_DEPTH);
                match self.leaves.iter().position(same) {
                    Some(i) => i,
                    None => {
                        let nz = data.vbank().get(vn).unwrap().get_nz_mask() & calc_mask(seen);
                        self.leaves.push(LeafVal { vn, constant: None, nz, size: size(data, vn), seen });
                        self.leaves.len() - 1
                    }
                }
            }
        };
        self.nodes.push(Node::Leaf(index));
        Some(self.nodes.len() - 1)
    }

    fn operator(&mut self, data: &Funcdata, code: OpCode, inputs: &[VarnodeId], seen: i32) -> Option<usize> {
        let a = self.build(data, inputs[0], seen)?;
        let node = if code == CPUI_INT_NEGATE {
            Node::Not(a, calc_mask(seen))
        } else {
            let b = self.build(data, inputs[1], seen)?;
            let code = if code == CPUI_INT_ADD { CPUI_INT_OR } else { code };
            Node::Bin(code, a, b)
        };
        self.cost += if code == CPUI_INT_ADD { 3 } else { 2 };
        self.nodes.push(node);
        Some(self.nodes.len() - 1)
    }

    /// Bitwise operators commute with truncation, and with zero extension once
    /// a NOT below it is taken at the narrower width, so the tree follows both.
    fn build(&mut self, data: &Funcdata, vn: VarnodeId, seen: i32) -> Option<usize> {
        let seen = seen.min(size(data, vn));
        if self.nodes.len() < MAX_NODES && constant_value(data, vn).is_none() {
            if let Some((code, inputs)) = logic_def(data, vn) {
                return self.operator(data, code, &inputs, seen);
            }
            let through = match def_of(data, vn) {
                Some((CPUI_SUBPIECE, inputs)) if constant_value(data, inputs[1]) == Some(0) => Some(inputs[0]),
                Some((CPUI_INT_ZEXT, inputs)) => Some(inputs[0]),
                _ => None,
            };
            if let Some(x) = through.filter(|&x| logic_def(data, x).is_some()) {
                self.cost += 1;
                return self.build(data, x, seen);
            }
        }
        self.leaf(data, vn, seen)
    }

    fn leaf_costs(&self) -> Vec<u32> {
        self.leaves
            .iter()
            .map(|l| u32::from(l.size != l.seen) + u32::from(l.seen != self.width))
            .collect()
    }

    fn read_leaf(&self, data: &mut Funcdata, before: OpId, i: usize) -> VarnodeId {
        let leaf = &self.leaves[i];
        let (mut vn, seen) = (leaf.vn, leaf.seen);
        if size(data, vn) != seen {
            let zero = data.new_constant(4, 0);
            vn = insert(data, before, CPUI_SUBPIECE, seen, &[vn, zero]);
        }
        if seen != self.width {
            vn = insert(data, before, CPUI_INT_ZEXT, self.width, &[vn]);
        }
        vn
    }

    fn vars(&self) -> Vec<usize> {
        (0..self.leaves.len()).filter(|&i| self.leaves[i].constant.is_none()).collect()
    }

    fn leaf_values(&self, vars: &[usize], row: usize) -> Vec<u64> {
        let mut values: Vec<u64> = self.leaves.iter().map(|l| l.constant.unwrap_or(0)).collect();
        for (j, &v) in vars.iter().enumerate() {
            if row >> j & 1 == 1 {
                values[v] = self.leaves[v].nz;
            }
        }
        values
    }

    fn eval(&self, leaf: &[u64]) -> u64 {
        let mut value = vec![0u64; self.nodes.len()];
        for (i, node) in self.nodes.iter().enumerate() {
            value[i] = match *node {
                Node::Leaf(l) => leaf[l],
                Node::Not(a, mask) => !value[a] & mask,
                Node::Bin(op, a, b) => apply(op, value[a], value[b]),
            };
        }
        value[self.nodes.len() - 1]
    }
}

/// The cheapest expression equal to the tree on every row, or `None` when
/// nothing cheaper than the tree itself exists.
fn simplest(tree: &LogicTree) -> Option<Expr> {
    let vars = tree.vars();
    if vars.len() > MAX_VARS {
        return None;
    }
    let rows = 1usize << vars.len();
    let leaf_rows: Vec<Vec<u64>> = (0..rows).map(|r| tree.leaf_values(&vars, r)).collect();
    let results: Vec<u64> = leaf_rows.iter().map(|l| tree.eval(l)).collect();
    let all_rows: u64 = if rows == 64 { u64::MAX } else { (1u64 << rows) - 1 };

    let mut groups: Vec<(u64, u64)> = Vec::new();
    for bit in 0..(tree.width as u32 * 8) {
        let column = results.iter().enumerate().fold(0u64, |c, (r, v)| c | ((v >> bit) & 1) << r);
        match groups.iter_mut().find(|g| g.0 == column) {
            Some(g) => g.1 |= 1 << bit,
            None => groups.push((column, 1 << bit)),
        }
    }
    let ones = groups.iter().filter(|g| g.0 == all_rows).fold(0, |m, g| m | g.1);
    let mut terms: Vec<(Expr, u64, u64)> = Vec::new();
    for &(column, bits) in groups.iter().filter(|g| g.0 != 0 && g.0 != all_rows) {
        let relevant: Vec<usize> = (0..vars.len())
            .filter(|&j| (0..rows).any(|r| (column >> r & 1) != (column >> (r ^ (1 << j)) & 1)))
            .collect();
        if relevant.len() > 3 {
            return None;
        }
        let n = relevant.len();
        let tt = (0..1usize << n).fold(0u16, |t, s| {
            let row = (0..n).filter(|&k| s >> k & 1 == 1).fold(0usize, |r, k| r | 1 << relevant[k]);
            t | (((column >> row) & 1) as u16) << s
        });
        let leaves: Vec<usize> = relevant.iter().map(|&j| vars[j]).collect();
        terms.push((Expr::from_form(n, tt, &leaves), bits, column));
    }

    if terms.len() > 4 {
        return None;
    }
    let span = terms.iter().fold(0, |m, t| m | t.1);
    let mut candidates: Vec<Expr> = vec![Expr::Const(ones)];
    let mut close = |e: Expr| {
        let masked = (span != tree.mask).then(|| Expr::bin(CPUI_INT_AND, e.clone(), Expr::Const(span)));
        for e in std::iter::once(e).chain(masked) {
            if ones != 0 {
                candidates.push(Expr::bin(CPUI_INT_OR, e.clone(), Expr::Const(ones)));
            }
            candidates.push(e);
        }
    };
    for (f, _, _) in &terms {
        close(f.clone());
    }
    if terms.len() == 2 && terms[0].2 ^ terms[1].2 == all_rows {
        close(Expr::bin(CPUI_INT_XOR, terms[0].0.clone(), Expr::Const(terms[1].1)));
    }
    for keep in 0..1u32 << terms.len() {
        let mut acc: Option<Expr> = None;
        for (i, (f, bits, _)) in terms.iter().enumerate() {
            let term = if keep >> i & 1 == 1 && *bits != tree.mask {
                Expr::bin(CPUI_INT_AND, f.clone(), Expr::Const(*bits))
            } else {
                f.clone()
            };
            acc = Some(match acc {
                None => term,
                Some(a) => Expr::bin(CPUI_INT_OR, a, term),
            });
        }
        if let Some(acc) = acc {
            candidates.push(if ones != 0 { Expr::bin(CPUI_INT_OR, acc, Expr::Const(ones)) } else { acc });
        }
    }

    let leaf_costs = tree.leaf_costs();
    candidates
        .into_iter()
        .filter(|e| e.cost(&leaf_costs) < tree.cost)
        .filter(|e| leaf_rows.iter().zip(&results).all(|(l, &r)| e.eval(l, tree.mask) == r))
        .min_by_key(|e| e.cost(&leaf_costs))
}

fn materialize(data: &mut Funcdata, before: OpId, e: &Expr, tree: &LogicTree) -> VarnodeId {
    if e.is_const() {
        let value = e.eval(&[], tree.mask);
        return data.new_constant(tree.width, value);
    }
    match e {
        Expr::Leaf(i) => tree.read_leaf(data, before, *i),
        Expr::Const(c) => data.new_constant(tree.width, c & tree.mask),
        Expr::Not(a) => {
            let a = materialize(data, before, a, tree);
            insert(data, before, CPUI_INT_NEGATE, tree.width, &[a])
        }
        Expr::Bin(op, a, b) => {
            let a = materialize(data, before, a, tree);
            let b = materialize(data, before, b, tree);
            insert(data, before, *op, tree.width, &[a, b])
        }
    }
}

fn rewrite(data: &mut Funcdata, op: OpId, e: &Expr, tree: &LogicTree) {
    if e.is_const() {
        let c = data.new_constant(tree.width, e.eval(&[], tree.mask));
        set_op(data, op, CPUI_COPY, &[c]);
        return;
    }
    match e {
        Expr::Leaf(i) => {
            let vn = tree.read_leaf(data, op, *i);
            set_op(data, op, CPUI_COPY, &[vn]);
        }
        Expr::Not(a) => {
            let a = materialize(data, op, a, tree);
            set_op(data, op, CPUI_INT_NEGATE, &[a]);
        }
        Expr::Bin(code, a, b) => {
            let a = materialize(data, op, a, tree);
            let b = materialize(data, op, b, tree);
            set_op(data, op, *code, &[a, b]);
        }
        Expr::Const(_) => unreachable!(),
    }
}

pushan_rule!(RulePushanLogic);

impl Rule for RulePushanLogic {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_XOR, CPUI_INT_NEGATE, CPUI_INT_ADD]
    }

    fn clone_rule(&self, groups: &ActionGroupList) -> Option<Box<dyn Rule>> {
        groups.contains(&self.group).then(|| Box::new(Self::new(self.group.clone())) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        if !devirtualized_only(data) {
            return 0;
        }
        let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) else {
            return 0;
        };
        let width = size(data, out);
        if width > 8 {
            return 0;
        }
        let Some((code, inputs)) = logic_def(data, out) else {
            return 0;
        };
        let mut tree = LogicTree { nodes: Vec::new(), leaves: Vec::new(), mask: calc_mask(width), width, cost: 0 };
        if tree.operator(data, code, &inputs, width).is_none() {
            return 0;
        }
        let operators = tree.nodes.iter().filter(|n| !matches!(n, Node::Leaf(_))).count();
        let leaf_nodes = tree.nodes.len() - operators;
        if operators == 1 && leaf_nodes == tree.leaves.len() && tree.leaves.iter().all(|l| l.constant.is_none()) {
            return 0;
        }
        let Some(best) = simplest(&tree) else {
            return 0;
        };
        rewrite(data, op, &best, &tree);
        1
    }
}

// ---------------------------------------------------------------------------
// RulePushanArith
// ---------------------------------------------------------------------------

struct Linear {
    terms: Vec<(VarnodeId, u64)>,
    constant: u64,
    mask: u64,
    width: i32,
    ops: u32,
    nots: u32,
}

impl Linear {
    fn add_leaf(&mut self, data: &Funcdata, vn: VarnodeId, scale: u64) -> Option<()> {
        if unmovable(data, vn) {
            return None;
        }
        match self.terms.iter_mut().find(|t| same_value(data, t.0, vn, SAME_DEPTH)) {
            Some(t) => t.1 = t.1.wrapping_add(scale) & self.mask,
            None => self.terms.push((vn, scale & self.mask)),
        }
        Some(())
    }

    fn operator(&mut self, data: &Funcdata, code: OpCode, inputs: &[VarnodeId], scale: u64) -> Option<()> {
        self.ops += 1;
        match code {
            CPUI_INT_ADD => {
                self.walk(data, inputs[0], scale)?;
                self.walk(data, inputs[1], scale)
            }
            CPUI_INT_SUB => {
                self.walk(data, inputs[0], scale)?;
                self.walk(data, inputs[1], scale.wrapping_neg())
            }
            CPUI_INT_2COMP => self.walk(data, inputs[0], scale.wrapping_neg()),
            CPUI_INT_NEGATE => {
                self.nots += 1;
                self.constant = self.constant.wrapping_sub(scale) & self.mask;
                self.walk(data, inputs[0], scale.wrapping_neg())
            }
            _ => {
                let (x, c) = match (constant_value(data, inputs[0]), constant_value(data, inputs[1])) {
                    (_, Some(c)) => (inputs[0], c),
                    (Some(c), _) => (inputs[1], c),
                    _ => return None,
                };
                self.walk(data, x, scale.wrapping_mul(c))
            }
        }
    }

    fn walk(&mut self, data: &Funcdata, vn: VarnodeId, scale: u64) -> Option<()> {
        if let Some(c) = constant_value(data, vn) {
            self.constant = self.constant.wrapping_add(scale.wrapping_mul(c)) & self.mask;
            return Some(());
        }
        if self.ops < MAX_NODES as u32 {
            if let Some((code, inputs)) = linear_def(data, vn, self.width) {
                return self.operator(data, code, &inputs, scale);
            }
        }
        self.add_leaf(data, vn, scale)
    }

    fn live_terms(&self) -> Vec<(VarnodeId, u64)> {
        let mut terms: Vec<(VarnodeId, u64)> = self.terms.iter().copied().filter(|t| t.1 != 0).collect();
        terms.sort_by_key(|t| t.1 != 1);
        terms
    }

    fn cost(&self) -> u32 {
        let terms = self.live_terms();
        if terms.is_empty() {
            return 0;
        }
        if terms.len() == 1 && terms[0].1 == self.mask && self.constant == self.mask {
            return 1;
        }
        let scaled = terms.iter().filter(|t| t.1 != 1).count() as u32;
        scaled + terms.len() as u32 - 1 + u32::from(self.constant != 0)
    }
}

fn linear_def(data: &Funcdata, vn: VarnodeId, width: i32) -> Option<(OpCode, Vec<VarnodeId>)> {
    let (code, inputs) = def_of(data, vn)?;
    if size(data, vn) != width {
        return None;
    }
    match code {
        CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_2COMP | CPUI_INT_NEGATE => Some((code, inputs)),
        CPUI_INT_MULT if inputs.iter().any(|&v| constant_value(data, v).is_some()) => Some((code, inputs)),
        _ => None,
    }
}

pushan_rule!(RulePushanArith);

impl Rule for RulePushanArith {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![CPUI_INT_NEGATE, CPUI_INT_ADD, CPUI_INT_SUB, CPUI_INT_MULT, CPUI_INT_2COMP]
    }

    fn clone_rule(&self, groups: &ActionGroupList) -> Option<Box<dyn Rule>> {
        groups.contains(&self.group).then(|| Box::new(Self::new(self.group.clone())) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        if !devirtualized_only(data) {
            return 0;
        }
        let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) else {
            return 0;
        };
        let width = size(data, out);
        if width > 8 {
            return 0;
        }
        let Some((code, inputs)) = linear_def(data, out, width) else {
            return 0;
        };
        let mask = calc_mask(width);
        let mut form = Linear { terms: Vec::new(), constant: 0, mask, width, ops: 0, nots: 0 };
        if form.operator(data, code, &inputs, 1).is_none() || form.nots == 0 || form.cost() >= form.ops {
            return 0;
        }
        let terms = form.live_terms();
        if terms.len() == 1 && terms[0].1 == mask && form.constant == mask {
            set_op(data, op, CPUI_INT_NEGATE, &[terms[0].0]);
            return 1;
        }
        let mut operands: Vec<(OpCode, Vec<VarnodeId>)> = Vec::new();
        for &(vn, coef) in &terms {
            if coef == 1 {
                operands.push((CPUI_COPY, vec![vn]));
            } else {
                let c = data.new_constant(width, coef);
                operands.push((CPUI_INT_MULT, vec![vn, c]));
            }
        }
        if form.constant != 0 || operands.is_empty() {
            let c = data.new_constant(width, form.constant);
            operands.push((CPUI_COPY, vec![c]));
        }
        let (last_code, last_inputs) = operands.pop().unwrap();
        if operands.is_empty() {
            set_op(data, op, last_code, &last_inputs);
            return 1;
        }
        let value = |data: &mut Funcdata, (code, inputs): (OpCode, Vec<VarnodeId>)| {
            if code == CPUI_COPY {
                inputs[0]
            } else {
                insert(data, op, code, width, &inputs)
            }
        };
        let mut acc: Option<VarnodeId> = None;
        for operand in operands {
            let v = value(data, operand);
            acc = Some(match acc {
                None => v,
                Some(a) => insert(data, op, CPUI_INT_ADD, width, &[a, v]),
            });
        }
        let last = value(data, (last_code, last_inputs));
        set_op(data, op, CPUI_INT_ADD, &[acc.unwrap(), last]);
        1
    }
}

// ---------------------------------------------------------------------------
// RulePushanPiece
// ---------------------------------------------------------------------------

/// `vn` equals bits `[shift, shift + 8*size(vn))` of `base`, zero above `base`.
#[derive(Clone, Copy, Debug)]
struct Slice {
    base: VarnodeId,
    shift: u32,
}

/// Every base `vn` is a slice of, nearest first.
fn slices(data: &Funcdata, vn: VarnodeId) -> Vec<Slice> {
    let width = size(data, vn) as u32 * 8;
    let mut chain = vec![Slice { base: vn, shift: 0 }];
    while chain.len() < 16 {
        let cur = *chain.last().unwrap();
        let Some((code, inputs)) = def_of(data, cur.base) else { break };
        let base_bits = size(data, cur.base) as u32 * 8;
        let next = match code {
            CPUI_COPY | CPUI_INT_ZEXT => Slice { base: inputs[0], shift: cur.shift },
            CPUI_SUBPIECE => {
                let Some(k) = constant_value(data, inputs[1]) else { break };
                let from_bits = size(data, inputs[0]) as u32 * 8;
                if cur.shift + width > base_bits && k as u32 * 8 + base_bits != from_bits {
                    break;
                }
                Slice { base: inputs[0], shift: cur.shift + k as u32 * 8 }
            }
            CPUI_INT_RIGHT if size(data, inputs[0]) as u32 * 8 == base_bits => {
                let Some(c) = constant_value(data, inputs[1]) else { break };
                if c >= base_bits as u64 {
                    break;
                }
                Slice { base: inputs[0], shift: cur.shift + c as u32 }
            }
            _ => break,
        };
        if unmovable(data, next.base) {
            break;
        }
        chain.push(next);
    }
    chain
}

pushan_rule!(RulePushanPiece);

impl Rule for RulePushanPiece {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![CPUI_PIECE]
    }

    fn clone_rule(&self, groups: &ActionGroupList) -> Option<Box<dyn Rule>> {
        groups.contains(&self.group).then(|| Box::new(Self::new(self.group.clone())) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        if !devirtualized_only(data) {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let (Some(hi), Some(lo), Some(out)) = (o.get_in(0), o.get_in(1), o.get_out()) else {
            return 0;
        };
        let lo_bits = size(data, lo) as u32 * 8;
        let out_size = size(data, out);
        let (lo_chain, hi_chain) = (slices(data, lo), slices(data, hi));
        let Some((base, shift)) = lo_chain.iter().find_map(|l| {
            hi_chain
                .iter()
                .any(|h| h.base == l.base && h.shift == l.shift + lo_bits)
                .then_some((l.base, l.shift))
        }) else {
            return 0;
        };
        if base == hi || base == lo {
            return 0;
        }
        let base_size = size(data, base);
        let base_bits = base_size as u32 * 8;
        let out_bits = out_size as u32 * 8;
        let existing = lo_chain.iter().chain(&hi_chain).map(|s| s.base).find(|&v| {
            size(data, v) == out_size && slices(data, v).iter().any(|s| s.base == base && s.shift == shift)
        });
        if let Some(v) = existing {
            set_op(data, op, CPUI_COPY, &[v]);
            return 1;
        }
        if shift == 0 && out_size == base_size {
            set_op(data, op, CPUI_COPY, &[base]);
        } else if shift % 8 == 0 && shift + out_bits <= base_bits {
            let k = data.new_constant(4, (shift / 8) as u64);
            set_op(data, op, CPUI_SUBPIECE, &[base, k]);
        } else if out_size == base_size {
            let k = data.new_constant(4, shift as u64);
            set_op(data, op, CPUI_INT_RIGHT, &[base, k]);
        } else {
            let shifted = if shift == 0 {
                base
            } else {
                let k = data.new_constant(4, shift as u64);
                insert(data, op, CPUI_INT_RIGHT, base_size, &[base, k])
            };
            if out_size > base_size {
                set_op(data, op, CPUI_INT_ZEXT, &[shifted]);
            } else {
                let zero = data.new_constant(4, 0);
                set_op(data, op, CPUI_SUBPIECE, &[shifted, zero]);
            }
        }
        1
    }
}

// ---------------------------------------------------------------------------
// RulePushanCarry
// ---------------------------------------------------------------------------

pushan_rule!(RulePushanCarry);

impl Rule for RulePushanCarry {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![CPUI_INT_CARRY, CPUI_INT_SCARRY]
    }

    fn clone_rule(&self, groups: &ActionGroupList) -> Option<Box<dyn Rule>> {
        groups.contains(&self.group).then(|| Box::new(Self::new(self.group.clone())) as Box<dyn Rule>)
    }

    /// An addition of two values with no set bit in common never carries, so
    /// neither its carry nor its signed overflow can be set; `x + ~x` (a flags
    /// word pushed after a NOR-gate NOT) is the common case.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        if !devirtualized_only(data) {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let (Some(a), Some(b), Some(out)) = (o.get_in(0), o.get_in(1), o.get_out()) else {
            return 0;
        };
        let nz = |v: VarnodeId| data.vbank().get(v).unwrap().get_nz_mask();
        let complement = |x: VarnodeId, y: VarnodeId| {
            matches!(def_of(data, x), Some((CPUI_INT_NEGATE, ref i)) if same_value(data, i[0], y, SAME_DEPTH))
        };
        if nz(a) & nz(b) != 0 && !complement(a, b) && !complement(b, a) {
            return 0;
        }
        let zero = data.new_constant(size(data, out), 0);
        set_op(data, op, CPUI_COPY, &[zero]);
        1
    }
}

#[cfg(test)]
mod tests;
