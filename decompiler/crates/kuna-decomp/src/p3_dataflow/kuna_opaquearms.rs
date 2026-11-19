//! A conditional branch whose two arms are the same code (option
//! `opaquepredicate`).
//!
//! `if (c) A else A` runs `A` whatever `c` is. Obfuscators clone a block and
//! select between the copies with a predicate that is hard to decide; the
//! predicate need not be decided when the copies are equal. The two arms are
//! equal when each is one basic block entered only from the branch, both
//! continue to the same block, their operations match pairwise in order, and
//! every value the join merges from them is the same on both edges.
//!
//! Two operations match when they have the same opcode and their operands
//! match: the same Varnode, the same constant, the same callee, or the outputs
//! of an earlier matched pair. Outputs must be the same storage, except
//! temporaries, which only need the same size. The branch condition then
//! becomes a constant and constant-branch removal deletes one copy.

use std::collections::BTreeMap;

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_ARM_OPS: usize = 256;

/// The operations of `block` other than its closing jump, which one arm of a
/// branch has and the arm that falls through does not.
fn block_ops(data: &Funcdata, block: BlockId) -> Option<Vec<OpId>> {
    let mut ops = Vec::new();
    let mut cur = data.bb_op_head(block);
    while let Some(op) = cur {
        if ops.len() >= MAX_ARM_OPS {
            return None;
        }
        if data.obank().get(op).is_none_or(|o| o.code() != OpCode::CPUI_BRANCH) {
            ops.push(op);
        }
        cur = data.bb_op_next(op);
    }
    Some(ops)
}

struct Pairing<'a> {
    data: &'a Funcdata,
    values: BTreeMap<VarnodeId, VarnodeId>,
    ops: BTreeMap<OpId, OpId>,
}

impl Pairing<'_> {
    fn same_value(&self, a: VarnodeId, b: VarnodeId) -> bool {
        if a == b {
            return true;
        }
        if self.values.get(&a) == Some(&b) {
            return true;
        }
        let bank = self.data.vbank();
        let (Some(va), Some(vb)) = (bank.get(a), bank.get(b)) else { return false };
        va.is_free() && vb.is_free() && va.get_size() == vb.get_size() && va.get_addr() == vb.get_addr()
    }

    fn same_output(&self, a: Option<VarnodeId>, b: Option<VarnodeId>) -> bool {
        let bank = self.data.vbank();
        match (a.and_then(|a| bank.get(a)), b.and_then(|b| bank.get(b))) {
            (None, None) => a.is_none() && b.is_none(),
            (Some(va), Some(vb)) => {
                let temporary = |v: &crate::varnode::Varnode| v.get_space().get_type() == spacetype::IPTR_INTERNAL;
                va.get_size() == vb.get_size() && (temporary(va) && temporary(vb) || va.get_addr() == vb.get_addr())
            }
            _ => false,
        }
    }

    fn same_op(&mut self, a: OpId, b: OpId) -> bool {
        let bank = self.data.obank();
        let (Some(oa), Some(ob)) = (bank.get(a), bank.get(b)) else { return false };
        let code = oa.code();
        if code != ob.code() || oa.num_input() != ob.num_input() {
            return false;
        }
        if matches!(
            code,
            OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN | OpCode::CPUI_MULTIEQUAL
        ) {
            return false;
        }
        for slot in 0..oa.num_input() {
            let (Some(ia), Some(ib)) = (oa.get_in(slot), ob.get_in(slot)) else { return false };
            let same = if code == OpCode::CPUI_CALL && slot == 0 {
                let callee = |op: OpId| {
                    let index = self.data.get_call_specs_index(op)?;
                    Some(self.data.get_call_specs(index).get_entry_address().clone())
                };
                callee(a).zip(callee(b)).is_some_and(|(ta, tb)| ta == tb)
            } else if code == OpCode::CPUI_INDIRECT && slot == 1 {
                let target = |vn: VarnodeId| {
                    self.data.vbank().get(vn).map(|v| crate::funcdata_varnode::op_iop_decode(v.get_offset()))
                };
                target(ia).zip(target(ib)).is_some_and(|(ta, tb)| self.ops.get(&ta) == Some(&tb))
            } else {
                self.same_value(ia, ib)
            };
            if !same {
                return false;
            }
        }
        if !self.same_output(oa.get_out(), ob.get_out()) {
            return false;
        }
        if let (Some(out_a), Some(out_b)) = (oa.get_out(), ob.get_out()) {
            self.values.insert(out_a, out_b);
        }
        self.ops.insert(a, b);
        true
    }
}

/// Whether the two arms of conditional branch `op` are equal code that
/// continues to one block with equal values.
pub(crate) fn identical(data: &Funcdata, op: OpId) -> bool {
    let Some(block) = data.obank().get(op).and_then(|o| o.get_parent()) else { return false };
    let graph = data.bblocks_ref();
    let head = graph.block(block);
    if head.size_out() != 2 {
        return false;
    }
    let (left, right) = (head.get_out(0), head.get_out(1));
    let arm = |id: BlockId| {
        let b = graph.block(id);
        (b.size_in() == 1 && b.get_in(0) == block && b.size_out() == 1).then(|| b.get_out(0))
    };
    let (Some(join), Some(other)) = (arm(left), arm(right)) else { return false };
    if left == right || join != other || join == left || join == right || join == block {
        return false;
    }
    let (Some(left_ops), Some(right_ops)) = (block_ops(data, left), block_ops(data, right)) else { return false };
    if left_ops.len() != right_ops.len() {
        return false;
    }
    // An INDIRECT precedes the operation it guards, so pair the operations
    // that are not INDIRECTs first.
    let mut pairing = Pairing { data, values: BTreeMap::new(), ops: BTreeMap::new() };
    let indirect = |op: &OpId| data.obank().get(*op).is_some_and(|o| o.code() == OpCode::CPUI_INDIRECT);
    for (&a, &b) in left_ops.iter().zip(&right_ops) {
        if indirect(&a) != indirect(&b) {
            return false;
        }
        if !indirect(&a) {
            pairing.ops.insert(a, b);
        }
    }
    if !left_ops.iter().zip(&right_ops).all(|(&a, &b)| pairing.same_op(a, b)) {
        return false;
    }
    let (left_slot, right_slot) = (graph.block(join).get_in_index(left), graph.block(join).get_in_index(right));
    let mut cur = data.bb_op_head(join);
    while let Some(phi) = cur {
        let Some(o) = data.obank().get(phi) else { return false };
        if o.code() != OpCode::CPUI_MULTIEQUAL {
            break;
        }
        match (o.get_in(left_slot), o.get_in(right_slot)) {
            (Some(a), Some(b)) if pairing.same_value(a, b) => {}
            _ => return false,
        }
        cur = data.bb_op_next(phi);
    }
    true
}
