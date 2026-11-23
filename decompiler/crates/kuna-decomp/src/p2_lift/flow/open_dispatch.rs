//! Raw control flow for a dispatch whose recovered cases leave an indirect exit.
use super::{FlowEnvironment, FlowInfo};
use crate::context::OpId;
use crate::jumptable::RecoveryMode;
use crate::kuna_returnbranch::Bypass;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;

impl<E: FlowEnvironment> FlowInfo<'_, E> {
    /// Entries into the dispatch block after its incoming range check. An edge
    /// can change the base or index before the load, or the target after it.
    pub(super) fn table_load_suffix(&self, jump: OpId) -> Option<Vec<(OpId, bool)>> {
        let bank = self.data.obank();
        let storage = |vn| {
            let value = self.data.vbank().get(vn)?;
            Some((value.get_space().get_index(), value.get_offset(), value.get_offset() + value.get_size() as u64))
        };
        let overlaps = |a: &(i32, u64, u64), b: &(i32, u64, u64)| a.0 == b.0 && a.1 < b.2 && b.1 < a.2;
        let covers = |outer: &(i32, u64, u64), inner: &(i32, u64, u64)| {
            outer.0 == inner.0 && outer.1 <= inner.1 && inner.2 <= outer.2
        };
        let branch = bank.get(jump)?;
        let mut needed = vec![storage(branch.get_in(0)?)?];
        let mut entries = Vec::new();
        let mut cursor = jump;
        let mut found_load = false;
        loop {
            let op = bank.get(cursor)?;
            if cursor != jump && (op.is_branch() || op.is_call()) {
                return found_load.then_some(entries);
            }
            if let Some(out) = op.get_out() {
                let out = storage(out)?;
                if !found_load && needed.iter().any(|need| overlaps(need, &out)) {
                    needed.retain(|need| !covers(&out, need));
                    match op.code() {
                        OpCode::CPUI_LOAD => found_load = true,
                        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_SEXT
                        | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB
                        | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_LEFT | OpCode::CPUI_SUBPIECE => {
                            for slot in 0..op.num_input() {
                                let input = op.get_in(slot)?;
                                if !self.data.vbank().get(input)?.is_constant() {
                                    needed.push(storage(input)?);
                                }
                            }
                        }
                        _ => return None,
                    }
                }
            }
            if op.is_instruction_start() {
                entries.push((cursor, found_load));
            }
            if op.is_block_start() {
                return found_load.then_some(entries);
            }
            let Some(prev) = bank.dead_prev(cursor) else { return found_load.then_some(entries) };
            cursor = prev;
        }
    }

    /// What the edge `from` -> `entry` brings to the loaded table of `jump`:
    /// see [`crate::kuna_returnbranch::classify_indexed_edge`]. A conditional
    /// edge counts only the values that take it.
    pub(super) fn classify_bypass(
        &self, from: OpId, entry: OpId, jump: OpId, idx: usize, budget: &mut usize,
    ) -> Bypass {
        let Some(branch) = self.data.obank().get(from) else { return Bypass::Unknown };
        let taken = if branch.code() == OpCode::CPUI_CBRANCH {
            let Some(dest) = branch.get_in(0).and_then(|vn| self.data.vbank().get(vn)) else {
                return Bypass::Unknown;
            };
            if dest.is_constant() {
                return Bypass::Unknown;
            }
            let to_dest = self.target(dest.get_addr()).ok() == Some(entry);
            let to_fall = self.fallthru_op(from) == Some(entry);
            match (to_dest, to_fall) {
                (true, false) => Some(true),
                (false, true) => Some(false),
                _ => return Bypass::Unknown,
            }
        } else {
            None
        };
        let table = self.data.get_jump_table(idx as i32);
        let stub = table.kuna_return_default();
        let known = |target: u64| {
            table.address_table().iter().any(|dest| {
                Some(dest) != stub
                    && AddrSpace::byte_to_address(dest.get_offset(), dest.get_space().unwrap().get_word_size()) == target
            })
        };
        crate::kuna_returnbranch::classify_indexed_edge(
            &self.data, from, jump, entry, taken, |index| self.env.is_isa_mode_switch(index),
            |loads| table.kuna_reads_recovered_rows(&self.data, loads), known, budget,
        )
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
