//! Raw control flow for a dispatch whose recovered cases leave an indirect exit.
use super::{FlowEnvironment, FlowInfo};
use crate::context::OpId;
use crate::jumptable::RecoveryMode;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;

impl<E: FlowEnvironment> FlowInfo<'_, E> {
    /// Instruction entries that bypass a memory table's destination load.
    /// A back edge through the load still dispatches through the table, even
    /// if it supplies a new index. Stop at an existing basic-block boundary:
    /// other predecessors there may have supplied a register target already.
    pub(super) fn table_load_suffix(&self, jump: OpId) -> Option<Vec<OpId>> {
        let bank = self.data.obank();
        let storage = |vn| {
            let value = self.data.vbank().get(vn)?;
            Some((value.get_addr().clone(), value.get_size()))
        };
        let branch = bank.get(jump)?;
        let mut needed = BTreeSet::from([storage(branch.get_in(0)?)?]);
        let mut entries = Vec::new();
        let mut cursor = jump;
        loop {
            let op = bank.get(cursor)?;
            if let Some(out) = op.get_out() {
                if needed.remove(&storage(out)?) {
                    match op.code() {
                        OpCode::CPUI_LOAD => return Some(entries),
                        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_SEXT
                        | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB
                        | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_LEFT | OpCode::CPUI_SUBPIECE => {
                            for slot in 0..op.num_input() {
                                let input = op.get_in(slot)?;
                                if !self.data.vbank().get(input)?.is_constant() {
                                    needed.insert(storage(input)?);
                                }
                            }
                        }
                        _ => return None,
                    }
                }
            }
            if op.is_instruction_start() {
                entries.push(cursor);
            }
            if op.is_block_start() || (cursor != jump && (op.is_branch() || op.is_call())) {
                return None;
            }
            cursor = bank.dead_prev(cursor)?;
        }
    }

    /// A re-entered selector with unproved values still transfers on those paths.
    /// Compare the actual branch operand with the destinations already followed
    /// before applying the ordinary indirect-call fallback.
    pub(super) fn open_dispatch(&mut self, op: OpId, idx: usize) -> KunaResult<()> {
        let table = self.data.get_jump_table(idx as i32);
        let targets: BTreeSet<_> = table
            .address_table()
            .iter()
            .filter(|addr| Some(*addr) != table.kuna_return_default())
            .cloned()
            .collect();
        let site = table.get_op_address().clone();
        let mask = table.kuna_target_encoding();
        self.data.remove_jump_table(idx);
        self.data.op_set_opcode_code(op, OpCode::CPUI_COPY);
        let input = self.data.obank().get(op).unwrap().get_in(0).unwrap();
        let size = self.data.vbank().get(input).unwrap().get_size();
        let selector = self.data.new_unique_out(size, op)?;
        let storage = self.data.vbank().get(selector).unwrap().get_addr().clone();
        let mut prev = op;
        if self.data.get_arch().indirect_return_addr.is_some()
            || self.data.get_arch().default_return_addr.is_some()
        {
            let guard = self.data.new_op(2, site.clone());
            self.data.op_set_opcode_code(guard, OpCode::CPUI_CBRANCH);
            let unproved = self.data.new_constant(1, 0);
            self.data.op_set_input(guard, unproved, 1)?;
            let ret = self.artificial_halt(&site, 0)?;
            let delta = self
                .data
                .obank()
                .get(ret)
                .unwrap()
                .get_time()
                .wrapping_sub(self.data.obank().get(guard).unwrap().get_time());
            let target = self.data.new_constant(4, delta as u64);
            self.data.op_set_input(guard, target, 0)?;
            self.data.op_dead_insert_after(guard, op);
            self.op_mark_start_basic(ret);
            self.op_mark_start_instruction(ret);
            self.data
                .open_dispatch_returns
                .push(crate::p2_lift::kuna_opendispatch::ReturnGuard {
                    selector: op,
                    branch: guard,
                    mask,
                });
            prev = guard;
        }
        for dest in targets {
            let cmp = self.data.new_op(2, site.clone());
            self.data.op_set_opcode_code(cmp, OpCode::CPUI_INT_EQUAL);
            let value = self.data.new_varnode(size, &storage, None);
            self.data.op_set_input(cmp, value, 0)?;
            let offset = AddrSpace::byte_to_address(
                dest.get_offset(),
                dest.get_space().unwrap().get_word_size(),
            );
            let constant = self.data.new_constant(size, offset);
            self.data.op_set_input(cmp, constant, 1)?;
            let matches = self.data.new_unique_out(1, cmp)?;
            let cond_addr = self.data.vbank().get(matches).unwrap().get_addr().clone();
            let branch = self.data.new_op(2, site.clone());
            self.data.op_set_opcode_code(branch, OpCode::CPUI_CBRANCH);
            let target = self.data.new_code_ref(&dest);
            self.data.op_set_input(branch, target, 0)?;
            let cond = self.data.new_varnode(1, &cond_addr, None);
            self.data.op_set_input(branch, cond, 1)?;
            self.data.op_dead_insert_after(cmp, prev);
            self.data.op_dead_insert_after(branch, cmp);
            if prev != op {
                self.op_mark_start_basic(cmp);
            }
            prev = branch;
        }
        let fallback = self.data.new_op(1, site);
        let value = self.data.new_varnode(size, &storage, None);
        self.data.op_set_input(fallback, value, 0)?;
        self.data.op_dead_insert_after(fallback, prev);
        self.op_mark_start_basic(fallback);
        self.truncate_indirect_jump(fallback, RecoveryMode::FailNormal)
    }
}
