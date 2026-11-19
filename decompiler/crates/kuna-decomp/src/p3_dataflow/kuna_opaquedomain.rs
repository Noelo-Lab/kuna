//! Exhaustive evaluation of a comparison over narrow inputs (option
//! `opaquepredicate`).
//!
//! A comparison whose operands are computed only from constants and from
//! values that together are at most two bytes wide (two one-byte values or
//! one two-byte value) has at most 65,536 input assignments. Every p-code
//! operation in the tree is evaluated exactly, at its own width, for each of
//! them; the comparison folds when all assignments agree. This decides
//! predicates that depend on a remainder, a division, a signed comparison or
//! a right shift, which the low-bit proofs of `kuna_opaquepredicate` cannot
//! read: `(x * x * x - x) % 3 == 0` and `x * x >= 0` hold for every zero- or
//! sign-extended byte `x` and for no wider one.
//!
//! A read of a global the whole program never writes (`kuna_opaqueglobals`,
//! `opaquepredicate globals`) is the constant the image initializes it to.

use std::collections::BTreeMap;

use kuna_base::address::calc_mask;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_LEAVES: usize = 2;
const MAX_LEAF_BYTES: int4 = 2;
const MAX_NODES: usize = 64;

#[derive(Clone, Copy)]
enum Node {
    Leaf(usize),
    Const(u64),
    Unary { code: OpCode, out: int4, size: int4, a: usize },
    Binary { code: OpCode, out: int4, size: int4, a: usize, b: usize },
}

struct Tree<'a> {
    data: &'a Funcdata,
    nodes: Vec<Node>,
    leaves: Vec<(VarnodeId, int4)>,
    memo: BTreeMap<VarnodeId, usize>,
    globals: usize,
}

fn evaluable(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_SUBPIECE
    )
}

impl<'a> Tree<'a> {
    fn push(&mut self, node: Node) -> Option<usize> {
        if self.nodes.len() >= MAX_NODES {
            return None;
        }
        self.nodes.push(node);
        Some(self.nodes.len() - 1)
    }

    fn rollback(&mut self, nodes: usize, leaves: usize, globals: usize) {
        self.nodes.truncate(nodes);
        self.leaves.truncate(leaves);
        self.globals = globals;
        self.memo.retain(|_, index| *index < nodes);
    }

    fn compile(&mut self, vn: VarnodeId) -> Option<usize> {
        let v = self.data.vbank().get(vn)?;
        let size = v.get_size();
        if !(1..=8).contains(&size) {
            return None;
        }
        if v.is_constant() {
            return self.push(Node::Const(v.get_offset() & calc_mask(size)));
        }
        if v.is_free() {
            return None;
        }
        if let Some(&index) = self.memo.get(&vn) {
            return Some(index);
        }
        if let Some(value) = crate::kuna_opaqueglobals::constant_read(self.data, vn) {
            self.globals += 1;
            return self.push(Node::Const(value));
        }
        let mark = (self.nodes.len(), self.leaves.len(), self.globals);
        let index = match v.get_def().and_then(|def| self.compile_op(def)) {
            Some(index) => index,
            None => {
                self.rollback(mark.0, mark.1, mark.2);
                let slot = match self.leaves.iter().position(|&(leaf, _)| leaf == vn) {
                    Some(slot) => slot,
                    None => {
                        let held: int4 = self.leaves.iter().map(|&(_, held)| held).sum();
                        if self.leaves.len() >= MAX_LEAVES || held + size > MAX_LEAF_BYTES {
                            return None;
                        }
                        self.leaves.push((vn, size));
                        self.leaves.len() - 1
                    }
                };
                self.push(Node::Leaf(slot))?
            }
        };
        self.memo.insert(vn, index);
        Some(index)
    }

    fn compile_op(&mut self, op: OpId) -> Option<usize> {
        let o = self.data.obank().get(op)?;
        let code = o.code();
        if !evaluable(code) {
            return None;
        }
        let out = self.data.vbank().get(o.get_out()?)?.get_size();
        let first = o.get_in(0)?;
        let size = self.data.vbank().get(first)?.get_size();
        let a = self.compile(first)?;
        match o.num_input() {
            1 => self.push(Node::Unary { code, out, size, a }),
            2 => {
                let b = self.compile(o.get_in(1)?)?;
                self.push(Node::Binary { code, out, size, a, b })
            }
            _ => None,
        }
    }

    /// The value of the last node with `values[i]` in leaf `i`; `None` when an
    /// operation has no result there (a division by zero).
    fn run(&self, values: [u64; MAX_LEAVES], slots: &mut Vec<u64>) -> Option<u64> {
        slots.clear();
        let glb = self.data.get_arch();
        for node in &self.nodes {
            let value = match *node {
                Node::Leaf(slot) => values[slot],
                Node::Const(value) => value,
                Node::Unary { code, out, size, a } => glb.op_behavior(code)?.evaluate_unary(out, size, slots[a]).ok()?,
                Node::Binary { code, out, size, a, b } => {
                    if matches!(
                        code,
                        OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_SDIV | OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SREM
                    ) && slots[b] == 0
                    {
                        return None;
                    }
                    glb.op_behavior(code)?.evaluate_binary(out, size, slots[a], slots[b]).ok()?
                }
            };
            slots.push(value);
        }
        slots.last().copied()
    }

    /// The value of each leaf when the leaves together hold the bits of
    /// `index`.
    fn assign(&self, index: u64) -> [u64; MAX_LEAVES] {
        let mut values = [0; MAX_LEAVES];
        let mut rest = index;
        for (slot, &(_, size)) in self.leaves.iter().enumerate() {
            values[slot] = rest & calc_mask(size);
            rest >>= 8 * size as u32;
        }
        values
    }

    /// Leaf values tried first: the extremes of each width, and the
    /// neighbours of every constant in the tree, where a comparison that
    /// depends on its input most often changes.
    fn probes(&self) -> Vec<u64> {
        let mut probes = vec![0, 1, 0x7f, 0x80, 0xff, 0x7fff, 0x8000, 0xffff];
        for node in &self.nodes {
            if let Node::Const(value) = *node {
                for near in [value.wrapping_sub(1), value, value.wrapping_add(1)] {
                    if !probes.contains(&(near & 0xffff)) && probes.len() < 20 {
                        probes.push(near & 0xffff);
                    }
                }
            }
        }
        probes
    }
}

/// The result of comparison `op` when it is the same for every value of the
/// narrow inputs it is computed from.
pub(crate) fn prove(data: &Funcdata, op: OpId) -> Option<bool> {
    let mut tree = Tree { data, nodes: Vec::new(), leaves: Vec::new(), memo: BTreeMap::new(), globals: 0 };
    tree.compile_op(op)?;
    if tree.leaves.is_empty() && tree.globals == 0 {
        return None;
    }
    let mut slots = Vec::with_capacity(tree.nodes.len());
    let first = tree.run([0; MAX_LEAVES], &mut slots)?;
    if tree.leaves.is_empty() {
        return Some(first != 0);
    }
    let probes = tree.probes();
    for &x in &probes {
        for &y in &probes[..if tree.leaves.len() > 1 { probes.len() } else { 1 }] {
            let mut values = [x, y];
            for (value, &(_, size)) in values.iter_mut().zip(&tree.leaves) {
                *value &= calc_mask(size);
            }
            if tree.run(values, &mut slots)? != first {
                return None;
            }
        }
    }
    let bits: u32 = tree.leaves.iter().map(|&(_, size)| 8 * size as u32).sum();
    for index in 0..1u64 << bits {
        if tree.run(tree.assign(index), &mut slots)? != first {
            return None;
        }
    }
    Some(first != 0)
}
