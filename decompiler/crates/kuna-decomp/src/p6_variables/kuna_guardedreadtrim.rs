//! Keep a call-written stack read inside the branch where it is evaluated.

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p3_dataflow::kuna_stackreadscope::MachineRead;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

#[cfg(test)]
use crate::p3_dataflow::kuna_stackreadscope::{mark_stack_read, record_propagated_read};

pub(crate) fn guarded_block(data: &Funcdata, vn: VarnodeId, reads: &[OpId]) -> Option<BlockId> {
    if !data.get_arch().guarded_read_trim {
        return None;
    }
    let value = data.vbank().get(vn)?;
    if value.get_space().get_type() != spacetype::IPTR_SPACEBASE
        || !value.is_addr_tied()
        || reads.is_empty()
    {
        return None;
    }
    let def = value.get_def()?;
    let definition = data.obank().get(def)?;
    if definition.code() != OpCode::CPUI_INDIRECT {
        return None;
    }
    let effect = definition.get_in(1).and_then(|v| data.vbank().get(v))?;
    if effect.get_space().get_type() != spacetype::IPTR_IOP {
        return None;
    }
    let effect = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
    let call = data.obank().get(effect)?;
    if !call.is_call() {
        return None;
    }
    let origin = call.get_parent()?;
    let graph = data.bblocks_ref();
    let origin_block = graph.block(origin);
    if origin_block.size_out() != 2 {
        return None;
    }
    let mut after_call = false;
    for op in data.bb_ops(origin) {
        if op == effect {
            after_call = true;
            continue;
        }
        if !after_call {
            continue;
        }
        let op = data.obank().get(op)?;
        if op.is_call() || matches!(op.code(), OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER) {
            return None;
        }
        if let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) {
            if value.characterize_overlap(out) != 0 {
                return None;
            }
        }
    }
    if !after_call {
        return None;
    }
    let machine_reads = data.kuna_guarded_reads.by_call.get(&effect)?;
    let space = value.get_space();
    if value.get_size() <= 0 {
        return None;
    }
    let end = value
        .get_offset()
        .checked_add(value.get_size() as u64 - 1)?;
    if end > space.get_highest() {
        return None;
    }
    for read in data
        .kuna_guarded_reads
        .reads()
        .filter(|read| read.space == space.get_index())
    {
        if read.size <= 0
            || read
                .offset
                .checked_add(read.size as u64 - 1)
                .is_none_or(|end| end > space.get_highest())
        {
            return None;
        }
    }
    let overlaps = |read: &&MachineRead| {
        read.space == space.get_index()
            && read.offset <= end
            && value.get_offset() <= read.offset + (read.size as u64 - 1)
    };
    if !machine_reads.iter().any(|read| overlaps(&read))
        || !data
            .kuna_guarded_reads
            .reads()
            .any(|read| overlaps(&read))
    {
        return None;
    }
    (0..origin_block.size_out())
        .map(|slot| origin_block.get_out(slot))
        .find(|&candidate| {
            candidate != origin
                && data
                    .kuna_guarded_reads
                    .reads()
                    .filter(overlaps)
                    .all(|read| {
                        read.block != origin
                            && graph.arena.get(read.block).is_some_and(|block| {
                                block.get_type() == crate::block::BlockType::Basic
                                    && block.get_parent() == graph.root
                            })
                    })
                && graph.block(candidate).size_in() == 1
                && graph.block(candidate).get_in(0) == origin
                && reads.iter().all(|&read| {
                    let Some(op) = data.obank().get(read) else {
                        return false;
                    };
                    if op.get_slot(vn) < 0 {
                        return false;
                    }
                    let Some(parent) = op.get_parent() else {
                        return false;
                    };
                    if op.code() == OpCode::CPUI_MULTIEQUAL {
                        (0..op.num_input())
                            .filter(|&slot| op.get_in(slot) == Some(vn))
                            .all(|slot| {
                                graph.dominates(candidate, Some(graph.block(parent).get_in(slot)))
                            })
                    } else {
                        graph.dominates(candidate, Some(parent))
                    }
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{ArchContext, TypeOp};
    use crate::op::pcodeop_flags;
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, SpacebaseSpace,
        UniqueSpace,
    };
    use std::rc::Rc;

    struct Fixture {
        data: Funcdata,
        origin: BlockId,
        success: BlockId,
        failure: BlockId,
        join: BlockId,
        vn: VarnodeId,
        read: OpId,
        call: OpId,
        stack: Rc<AddrSpace>,
        ram: Rc<AddrSpace>,
    }

    fn fixture() -> Fixture {
        fixture_with_option(true)
    }

    fn fixture_with_option(enabled: bool) -> Fixture {
        let mut spaces = AddrSpaceManager::new();
        spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        spaces
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        spaces.insert_space(Rc::clone(&ram)).unwrap();
        let stack = Rc::new(SpacebaseSpace::new("stack", 3, 8, &ram, 1, true, false));
        spaces.insert_space(Rc::clone(&stack)).unwrap();
        spaces.insert_space(Rc::new(IopSpace::new(4))).unwrap();
        let mut arch = ArchContext::new(spaces);
        arch.guarded_read_trim = enabled;
        let mut data = Funcdata::new(
            "guard",
            "guard",
            Rc::new(arch),
            Address::new(Rc::clone(&ram), 0x1000),
            0x1000_0000,
            0x40,
        )
        .unwrap();
        let root = data.bblocks_ref().root.unwrap();
        let origin = data.bblocks_mut().new_block_basic(root);
        let success = data.bblocks_mut().new_block_basic(root);
        let failure = data.bblocks_mut().new_block_basic(root);
        let join = data.bblocks_mut().new_block_basic(root);
        for (from, to) in [
            (origin, success),
            (origin, failure),
            (success, join),
            (failure, join),
        ] {
            data.bblocks_mut().add_edge(from, to);
        }
        data.bblocks_mut().calc_forward_dominator(root, &[origin]);
        let call = data.new_op(0, Address::new(Rc::clone(&ram), 0x1000));
        data.op_set_opcode(
            call,
            TypeOp::new(OpCode::CPUI_CALL, pcodeop_flags::call, "CALL"),
        );
        data.op_insert(call, origin, None);
        data.push_call_specs(crate::fspec::FuncCallSpecs::new(
            call,
            Address::new(Rc::clone(&ram), 0x2000),
        ));
        let indirect =
            data.new_indirect_op(call, &Address::new(Rc::clone(&stack), u64::MAX - 3), 4, 0);
        let vn = data.obank().get(indirect).unwrap().get_out().unwrap();
        data.vbank_mut()
            .get_mut(vn)
            .unwrap()
            .set_flags_pub(crate::varnode::varnode_flags::addrtied);
        let read = data.new_op(1, Address::new(Rc::clone(&ram), 0x1010));
        data.op_set_opcode_code(read, OpCode::CPUI_COPY);
        data.op_set_input(read, vn, 0).unwrap();
        data.new_unique_out(4, read).unwrap();
        data.op_insert(read, success, None);
        mark_stack_read(&mut data, read);
        record_propagated_read(&mut data, read, vn);
        Fixture {
            data,
            origin,
            success,
            failure,
            join,
            vn,
            read,
            call,
            stack,
            ram,
        }
    }

    #[test]
    fn option_off_preserves_legacy_placement_without_tracking() {
        let f = fixture_with_option(false);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
        assert!(f.data.kuna_guarded_reads.lowered.is_empty());
        assert!(f.data.kuna_guarded_reads.by_call.is_empty());
    }

    #[test]
    fn stack_read_stays_in_the_dominating_success_branch() {
        let f = fixture();
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), Some(f.success));
    }

    #[test]
    fn an_unguarded_read_prevents_delaying_the_capture() {
        let mut f = fixture();
        f.data.op_uninsert(f.read);
        f.data.op_insert_after(f.read, f.call);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }

    #[test]
    fn reads_on_both_successors_prevent_delaying_the_capture() {
        let mut f = fixture();
        let other = f.data.new_op(1, Address::new(Rc::clone(&f.ram), 0x1020));
        f.data.op_set_opcode_code(other, OpCode::CPUI_COPY);
        f.data.op_set_input(other, f.vn, 0).unwrap();
        f.data.new_unique_out(4, other).unwrap();
        f.data.op_insert(other, f.failure, None);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read, other]), None);
    }

    #[test]
    fn a_phi_read_is_evaluated_on_its_incoming_edge() {
        let mut f = fixture();
        let phi = f.data.new_op(2, Address::new(Rc::clone(&f.ram), 0x1030));
        f.data.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
        f.data.op_set_input(phi, f.vn, 0).unwrap();
        let zero = f.data.new_constant(4, 0);
        f.data.op_set_input(phi, zero, 1).unwrap();
        f.data.new_unique_out(4, phi).unwrap();
        f.data.op_insert(phi, f.join, None);
        assert_eq!(guarded_block(&f.data, f.vn, &[phi]), Some(f.success));
        f.data.op_set_input(phi, f.vn, 1).unwrap();
        assert_eq!(guarded_block(&f.data, f.vn, &[phi]), None);
    }

    #[test]
    fn a_write_before_the_branch_preserves_the_earlier_capture() {
        let mut f = fixture();
        let write = f.data.new_op(1, Address::new(Rc::clone(&f.ram), 0x1008));
        f.data.op_set_opcode_code(write, OpCode::CPUI_COPY);
        let c = f.data.new_constant(4, 23);
        f.data.op_set_input(write, c, 0).unwrap();
        f.data
            .new_varnode_out(4, &Address::new(Rc::clone(&f.stack), u64::MAX - 3), write)
            .unwrap();
        f.data.op_insert_after(write, f.call);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }
    #[test]
    fn intervening_unknown_effects_do_not_move_the_read() {
        for code in [
            OpCode::CPUI_CALL,
            OpCode::CPUI_STORE,
            OpCode::CPUI_CALLOTHER,
        ] {
            let mut f = fixture();
            let effect = f.data.new_op(0, Address::new(Rc::clone(&f.ram), 0x1008));
            let flags = if code == OpCode::CPUI_CALL {
                pcodeop_flags::call
            } else {
                0
            };
            f.data
                .op_set_opcode(effect, TypeOp::new(code, flags, "effect"));
            f.data.op_insert_after(effect, f.call);
            assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None, "{code:?}");
        }
    }

    #[test]
    fn a_partial_overlapping_write_keeps_the_prior_capture() {
        let mut f = fixture();
        let write = f.data.new_op(1, Address::new(Rc::clone(&f.ram), 0x1008));
        f.data.op_set_opcode_code(write, OpCode::CPUI_COPY);
        let c = f.data.new_constant(1, 23);
        f.data.op_set_input(write, c, 0).unwrap();
        f.data
            .new_varnode_out(1, &Address::new(Rc::clone(&f.stack), u64::MAX - 2), write)
            .unwrap();
        f.data.op_insert_after(write, f.call);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }
    #[test]
    fn an_indirect_output_after_the_call_is_an_intervening_write() {
        let mut f = fixture();
        let write = f.data.new_indirect_op(
            f.call,
            &Address::new(Rc::clone(&f.stack), u64::MAX - 3),
            4,
            0,
        );
        f.data.op_uninsert(write);
        f.data.op_insert_after(write, f.call);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }

    #[test]
    fn non_stack_call_effects_keep_the_definition_placement() {
        let mut f = fixture();
        let indirect =
            f.data
                .new_indirect_op(f.call, &Address::new(Rc::clone(&f.ram), 0x3000), 4, 0);
        let value = f.data.obank().get(indirect).unwrap().get_out().unwrap();
        f.data.op_set_input(f.read, value, 0).unwrap();
        assert_eq!(guarded_block(&f.data, value, &[f.read]), None);
    }
    #[test]
    fn a_successor_with_another_predecessor_keeps_definition_placement() {
        let mut f = fixture();
        f.data.bblocks_mut().add_edge(f.failure, f.success);
        let root = f.data.bblocks_ref().root.unwrap();
        f.data
            .bblocks_mut()
            .calc_forward_dominator(root, &[f.origin]);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }
    #[test]
    fn an_unconditional_machine_read_cannot_be_delayed_to_its_conditional_use() {
        let mut f = fixture();
        f.data
            .kuna_guarded_reads
            .lowered
            .get_mut(&f.read)
            .unwrap()
            .block = f.origin;
        f.data.kuna_guarded_reads.by_call.clear();
        record_propagated_read(&mut f.data, f.read, f.vn);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }

    #[test]
    fn missing_machine_read_provenance_keeps_the_original_placement() {
        let mut f = fixture();
        f.data.kuna_guarded_reads = Default::default();
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }
    #[test]
    fn stale_or_non_basic_machine_read_evidence_keeps_original_placement() {
        let mut f = fixture();
        let root = f.data.bblocks_ref().root.unwrap();
        f.data
            .kuna_guarded_reads
            .lowered
            .get_mut(&f.read)
            .unwrap()
            .block = root;
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }

    #[test]
    fn a_wrapping_machine_read_range_cannot_be_ignored() {
        let mut f = fixture();
        let mut read = f.data.kuna_guarded_reads.by_call[&f.call][0].clone();
        read.offset = u64::MAX;
        read.size = 2;
        read.block = f.origin;
        let extra = f.data.new_op(0, Address::new(Rc::clone(&f.ram), 0x1044));
        f.data.kuna_guarded_reads.lowered.insert(extra, read);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }
    #[test]
    fn an_unbound_machine_read_on_failure_does_not_feed_the_capture() {
        let mut f = fixture();
        let mut read = f.data.kuna_guarded_reads.lowered[&f.read].clone();
        read.block = f.failure;
        let extra = f.data.new_op(0, Address::new(Rc::clone(&f.ram), 0x1044));
        f.data.kuna_guarded_reads.lowered.insert(extra, read);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), Some(f.success));
    }

    #[test]
    fn removed_original_read_blocks_cannot_be_replaced_by_new_evidence() {
        let mut f = fixture();
        let root = f.data.bblocks_ref().root.unwrap();
        let old = f.data.bblocks_mut().new_block_basic(root);
        f.data
            .kuna_guarded_reads
            .lowered
            .get_mut(&f.read)
            .unwrap()
            .block = old;
        f.data.bblocks_mut().remove_block(root, old);
        let replacement = f.data.bblocks_mut().new_block_basic(root);
        assert_ne!(old, replacement);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), None);
    }

    #[test]
    fn a_join_reload_of_later_storage_does_not_feed_the_capture() {
        let mut f = fixture();
        let store = f.data.new_op(1, Address::new(Rc::clone(&f.ram), 0x1040));
        f.data.op_set_opcode_code(store, OpCode::CPUI_COPY);
        let zero = f.data.new_constant(4, 7);
        f.data.op_set_input(store, zero, 0).unwrap();
        let later = f
            .data
            .new_varnode_out(4, &Address::new(Rc::clone(&f.stack), u64::MAX - 3), store)
            .unwrap();
        f.data.op_insert(store, f.join, None);
        let read = f.data.new_op(1, Address::new(Rc::clone(&f.ram), 0x1044));
        f.data.op_set_opcode_code(read, OpCode::CPUI_COPY);
        f.data.op_set_input(read, later, 0).unwrap();
        f.data.new_unique_out(4, read).unwrap();
        f.data.op_insert(read, f.join, None);
        mark_stack_read(&mut f.data, read);
        assert_eq!(guarded_block(&f.data, f.vn, &[f.read]), Some(f.success));
    }
}
