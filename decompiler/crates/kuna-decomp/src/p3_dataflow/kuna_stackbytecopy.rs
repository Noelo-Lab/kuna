//! Expand short, statically terminating frame byte-copy loops before heritage.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::calc_mask;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::{pcodeop_addlflags, pcodeop_flags};

type Storage = (i32, u64, i32);

#[derive(Clone, Copy)]
enum Value {
    Number(u64),
    Frame(u64),
    Unknown,
}

struct State {
    values: BTreeMap<Storage, Value>,
    results: BTreeMap<VarnodeId, Value>,
    spaces: [i32; 2],
}

fn storage(fd: &Funcdata, id: VarnodeId) -> Option<Storage> {
    let value = fd.vbank().get(id)?;
    Some((
        value.get_space().get_index(),
        value.get_offset(),
        value.get_size(),
    ))
}

impl State {
    fn read(&self, fd: &Funcdata, id: VarnodeId) -> Value {
        let value = fd.vbank().get(id).unwrap();
        if value.is_constant() {
            Value::Number(value.get_offset())
        } else if value.is_written() {
            self.results.get(&id).copied().unwrap_or(Value::Unknown)
        } else {
            storage(fd, id)
                .and_then(|key| self.values.get(&key).copied())
                .unwrap_or(Value::Unknown)
        }
    }

    fn write(&mut self, id: VarnodeId, key: Storage, value: Value) {
        self.results.insert(id, value);
        self.values.retain(|&(space, offset, size), _| {
            space != key.0
                || offset as u128 + size as u128 <= key.1 as u128
                || key.1 as u128 + key.2 as u128 <= offset as u128
        });
        self.values.insert(key, value);
    }

    fn step(&mut self, fd: &Funcdata, id: OpId) -> Option<()> {
        let op = fd.obank().get(id)?;
        let Some(output) = op.get_out() else {
            return Some(());
        };
        let key = storage(fd, output)?;
        if !self.spaces.contains(&key.0) {
            return Some(());
        }
        let size = fd.vbank().get(output)?.get_size();
        if !(1..=8).contains(&size) {
            return None;
        }
        let a = self.read(fd, op.get_in(0)?);
        let b = if op.num_input() > 1 {
            self.read(fd, op.get_in(1)?)
        } else {
            Value::Unknown
        };
        let value = match (op.code(), a, b) {
            (OpCode::CPUI_COPY, value, _) => value,
            (
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB,
                Value::Frame(base),
                Value::Number(offset),
            )
            | (OpCode::CPUI_INT_ADD, Value::Number(offset), Value::Frame(base)) => {
                Value::Frame(base.wrapping_add(offset) & calc_mask(size))
            }
            (OpCode::CPUI_INT_SUB, Value::Frame(base), Value::Number(offset)) => {
                Value::Frame(base.wrapping_sub(offset) & calc_mask(size))
            }
            (_, Value::Number(a), b) => {
                let behavior = fd.get_arch().op_behavior(op.code())?;
                let input_size = fd.vbank().get(op.get_in(0)?)?.get_size();
                let result = if behavior.is_special() {
                    None
                } else if behavior.is_unary() {
                    behavior.evaluate_unary(size, input_size, a).ok()
                } else if let Value::Number(b) = b {
                    behavior.evaluate_binary(size, input_size, a, b).ok()
                } else {
                    None
                };
                result.map_or(Value::Unknown, |n| Value::Number(n & calc_mask(size)))
            }
            _ => Value::Unknown,
        };
        self.write(output, key, value);
        Some(())
    }
}

fn trip_count(fd: &Funcdata, block: BlockId) -> Option<usize> {
    let graph = fd.bblocks_ref();
    let header = graph.block(block);
    if header.size_in() != 2 || header.size_out() != 2 {
        return None;
    }
    let entry = (0..2)
        .map(|slot| header.get_in(slot))
        .find(|&b| b != block)?;
    if !(0..2).any(|slot| header.get_in(slot) == block)
        || !(0..2).any(|slot| header.get_out(slot) == block)
        || graph.block(entry).size_out() != 1
    {
        return None;
    }
    let operations = fd.bb_ops(block);
    let branch = *operations.last()?;
    let branch_op = fd.obank().get(branch)?;
    if operations.len() > 64
        || branch_op.code() != OpCode::CPUI_CBRANCH
        || branch_op.is_boolean_flip()
    {
        return None;
    }
    let mut loads = 0;
    let mut stores = 0;
    for &id in &operations[..operations.len() - 1] {
        let op = fd.obank().get(id)?;
        match op.code() {
            OpCode::CPUI_LOAD => {
                if fd.vbank().get(op.get_out()?)?.get_size() != 1 {
                    return None;
                }
                loads += 1;
            }
            OpCode::CPUI_STORE => {
                if fd.vbank().get(op.get_in(2)?)?.get_size() != 1 {
                    return None;
                }
                stores += 1;
            }
            _ if !op.is_call()
                && fd
                    .get_arch()
                    .op_behavior(op.code())
                    .is_some_and(|behavior| !behavior.is_special()) =>
            {
                ()
            }
            _ => return None,
        }
    }
    if loads != 1 || stores != 1 {
        return None;
    }
    let stack = fd.get_arch().manage().get_stack_space()?;
    let base = stack.get_spacebase(0).ok()?;
    let base_key = (
        base.space.as_ref()?.get_index(),
        base.offset,
        base.size as i32,
    );
    if operations.iter().any(|&id| {
        fd.obank()
            .get(id)
            .and_then(|op| op.get_out())
            .and_then(|v| storage(fd, v))
            .is_some_and(|key| {
                key.0 == base_key.0
                    && (key.1 as u128) < base_key.1 as u128 + base_key.2 as u128
                    && (base_key.1 as u128) < key.1 as u128 + key.2 as u128
            })
    }) {
        return None;
    }
    let unique = fd.get_arch().manage().get_unique_space()?.get_index();
    if operations.iter().any(|&id| {
        fd.obank()
            .get(id)
            .and_then(|op| op.get_out())
            .and_then(|v| storage(fd, v))
            .is_some_and(|key| key.0 != base_key.0 && key.0 != unique)
    }) {
        return None;
    }
    let container = stack.get_contain()?.get_index() as u64;
    if operations.iter().any(|&id| {
        let op = fd.obank().get(id).unwrap();
        matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
            && op
                .get_in(0)
                .and_then(|v| fd.vbank().get(v))
                .is_none_or(|v| !v.is_constant() || v.get_offset() != container)
    }) {
        return None;
    }
    let mut state = State {
        values: BTreeMap::from([(base_key, Value::Frame(0))]),
        results: BTreeMap::new(),
        spaces: [base_key.0, unique],
    };
    let mut initialized = BTreeSet::new();
    let word_size = stack.get_contain()?.get_word_size();
    let setup = fd.bb_ops(entry);
    if setup.len() > 128 {
        return None;
    }
    for id in setup {
        let op = fd.obank().get(id)?;
        if op.is_call() || matches!(op.code(), OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCHIND) {
            return None;
        }
        if op.code() == OpCode::CPUI_STORE {
            if let Value::Frame(offset) = state.read(fd, op.get_in(1)?) {
                let value = op.get_in(2)?;
                let size = fd.vbank().get(value)?.get_size();
                if !(1..=8).contains(&size) {
                    return None;
                }
                let offset = kuna_base::space::AddrSpace::address_to_byte(offset, word_size);
                for byte in 0..size as u64 {
                    let address = offset.wrapping_add(byte);
                    if matches!(state.read(fd, value), Value::Number(_)) {
                        initialized.insert(address);
                    } else {
                        initialized.remove(&address);
                    }
                }
            } else {
                initialized.clear();
            }
        }
        state.step(fd, id)?;
    }
    let mut constant_buffer = true;
    for iteration in 1..=4 {
        for &id in &operations[..operations.len() - 1] {
            let op = fd.obank().get(id)?;
            if op.code() == OpCode::CPUI_STORE {
                let Value::Frame(offset) = state.read(fd, op.get_in(1)?) else {
                    return None;
                };
                let offset = kuna_base::space::AddrSpace::address_to_byte(offset, word_size);
                constant_buffer &= initialized.contains(&offset);
            }
            state.step(fd, id)?;
        }
        let Value::Number(condition) = state.read(fd, branch_op.get_in(1)?) else {
            return None;
        };
        let successor = if condition != 0 {
            header.get_true_out()
        } else {
            header.get_false_out()
        };
        if successor != block {
            return (iteration > 1 && !constant_buffer).then_some(iteration);
        }
    }
    None
}

fn expand(fd: &mut Funcdata, block: BlockId, count: usize) -> KunaResult<()> {
    let operations = fd.bb_ops(block);
    let branch = *operations.last().unwrap();
    let body = &operations[..operations.len() - 1];
    let mut final_outputs = BTreeMap::new();
    for _ in 0..count {
        let mut outputs = BTreeMap::new();
        for &id in body {
            let op = fd.obank().get(id).unwrap();
            let code = op.code();
            let address = op.get_addr().clone();
            let flags = op.get_flags() & pcodeop_flags::startmark;
            let additional = op.get_addlflags() & pcodeop_addlflags::kuna_exactfloat;
            let inputs: Vec<_> = (0..op.num_input())
                .map(|slot| op.get_in(slot).unwrap())
                .collect();
            let output = op.get_out();
            let copy = fd.new_op(inputs.len() as i32, address);
            fd.op_set_opcode_code(copy, code);
            let cloned = fd.obank_mut().get_mut(copy).unwrap();
            cloned.set_flag(flags);
            cloned.set_additional_flag(additional);
            if matches!(code, OpCode::CPUI_LOAD | OpCode::CPUI_STORE) {
                fd.note_stack_byte_copy_op(copy);
            }
            for (slot, input) in inputs.into_iter().enumerate() {
                let value = if fd.vbank().get(input).unwrap().is_written() {
                    outputs.get(&input).copied().unwrap_or(input)
                } else {
                    fd.clone_varnode(input)
                };
                fd.op_set_input(copy, value, slot as i32)?;
            }
            if let Some(output) = output {
                let value = fd.clone_varnode(output);
                fd.op_set_output(copy, value)?;
                outputs.insert(output, value);
            }
            fd.op_insert_before(copy, branch);
        }
        final_outputs = outputs;
    }
    for (old, new) in final_outputs {
        fd.total_replace(old, new)?;
    }
    let self_edge = (0..2)
        .find(|&slot| fd.bblocks_ref().block(block).get_out(slot) == block)
        .unwrap();
    fd.branch_remove_internal(block, self_edge)?;
    for &id in body {
        fd.op_destroy(id);
    }
    fd.structure_reset();
    Ok(())
}

pub struct ActionStackByteCopy {
    base: ActionBase,
}

impl ActionStackByteCopy {
    pub fn boxed() -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "stackbytecopy", "stackvars"),
        })
    }
}

impl Action for ActionStackByteCopy {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(|| Self::boxed())
    }
    fn apply(&mut self, fd: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !fd.stack_store_guard()
            || fd.get_arch().index_alias_guard == super::kuna_indexaliasguard::LEVEL_OFF
            || fd.get_heritage_pass() != 0
        {
            return 0;
        }
        let mut changed = 0;
        for index in 0..fd.bblocks_get_size() {
            let block = fd.bblocks_get_block(index);
            if let Some(count) = trip_count(fd, block) {
                expand(fd, block, count).expect("validated frame byte-copy loop");
                changed += 1;
            }
        }
        changed
    }
}
