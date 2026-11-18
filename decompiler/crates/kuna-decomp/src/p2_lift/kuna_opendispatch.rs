//! Preserve caller-return provenance separately from the address of an open dispatch.
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;
use std::collections::{HashMap, HashSet};

/// Flow installs a conditional return beside the address comparisons and the
/// indirect call. Once heritage links definitions, only the incoming return
/// paths enable it. An opaque input can have any bit pattern, so a reserved
/// address cannot distinguish these paths.
pub(crate) struct ReturnGuard {
    pub selector: OpId,
    pub branch: OpId,
    pub mask: Option<(i32, u64)>,
}

pub(crate) fn resolve_returns(data: &mut Funcdata) -> KunaResult<()> {
    if data.open_dispatch_returns.is_empty() {
        return Ok(());
    }
    let stack_returns = entry_return_loads(data).unwrap_or_default();
    for guard in std::mem::take(&mut data.open_dispatch_returns) {
        let Some(root) = data
            .obank()
            .get(guard.selector)
            .filter(|op| !op.is_dead() && op.code() == OpCode::CPUI_COPY)
            .and_then(|op| op.get_in(0))
        else {
            continue;
        };
        let Some(branch) = data.obank().get(guard.branch) else {
            continue;
        };
        if branch.is_dead() || branch.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let mut parents: HashMap<VarnodeId, Vec<VarnodeId>> = HashMap::new();
        let mut projections = HashMap::new();
        let mut returns = Vec::new();
        let mut seen = HashSet::new();
        let mut work = vec![root];
        while let Some(vn) = work.pop() {
            if !seen.insert(vn) {
                continue;
            }
            if stack_returns.contains(&vn) || crate::kuna_indirectreturn::incoming_return(data, vn)
            {
                returns.push(vn);
            } else if let Some((def, slots)) = projection(data, vn, &guard) {
                let producer = data.obank().get(def).unwrap();
                for &slot in &slots {
                    let source = producer.get_in(slot).unwrap();
                    parents.entry(source).or_default().push(vn);
                    work.push(source);
                }
                projections.insert(vn, (def, slots));
            }
        }
        if returns.is_empty() {
            continue;
        }

        // Clone only selector expressions that can carry the return. Allocate
        // all outputs first so loop-carried PHIs can reference each other.
        let mut values = HashMap::new();
        for &vn in &returns {
            let size = data.vbank().get(vn).unwrap().get_size();
            values.insert(vn, (data.new_constant(size, 0), data.new_constant(1, 1)));
        }
        let mut pending = Vec::new();
        while let Some(child) = returns.pop() {
            for &vn in parents.get(&child).into_iter().flatten() {
                if values.contains_key(&vn) {
                    continue;
                }
                let (def, slots) = &projections[&vn];
                let op = data.obank().get(*def).unwrap();
                let code = op.code();
                let block = op.get_parent().unwrap();
                let site = op.get_addr().clone();
                let inputs: Vec<_> = (0..op.num_input()).map(|i| op.get_in(i).unwrap()).collect();
                let size = data.vbank().get(vn).unwrap().get_size();
                let value = data.new_op(inputs.len() as i32, site.clone());
                data.op_set_opcode_code(value, code);
                let tag = data.new_op(slots.len() as i32, site);
                let phi = code == OpCode::CPUI_MULTIEQUAL;
                data.op_set_opcode_code(tag, if phi { code } else { OpCode::CPUI_COPY });
                values.insert(
                    vn,
                    (
                        data.new_unique_out(size, value)?,
                        data.new_unique_out(1, tag)?,
                    ),
                );
                if phi {
                    data.op_insert_begin(value, block);
                    data.op_insert_begin(tag, block);
                } else {
                    data.op_insert_before(value, *def);
                    data.op_insert_before(tag, *def);
                }
                pending.push((value, tag, inputs, slots));
                returns.push(vn);
            }
        }
        for (value, tag, inputs, slots) in pending {
            for (i, &slot) in slots.iter().enumerate() {
                let source = inputs[slot as usize];
                let bit = values
                    .get(&source)
                    .map(|pair| pair.1)
                    .unwrap_or_else(|| data.new_constant(1, 0));
                data.op_set_input(tag, bit, i as i32)?;
            }
            for (i, input) in inputs.into_iter().enumerate() {
                let replacement = values.get(&input).map_or(input, |pair| pair.0);
                data.op_set_input(value, replacement, i as i32)?;
            }
        }
        let (value, tag) = values[&root];
        data.op_set_input(guard.selector, value, 0)?;
        data.op_set_input(guard.branch, tag, 1)?;
    }
    Ok(())
}

/// Before stack heritage turns a LOAD into a stack input, an entry-block
/// read at the incoming stack pointer can already prove the saved return.
/// Any preceding call or possible memory overwrite ends that proof.
fn entry_return_loads(data: &Funcdata) -> Option<HashSet<VarnodeId>> {
    let arch = data.get_arch();
    let ret = arch
        .indirect_return_addr
        .as_ref()
        .or(arch.default_return_addr.as_ref())?;
    let stack = arch.manage().get_stack_space()?;
    if ret.offset != 0 || ret.space.as_ref()?.get_index() != stack.get_index() {
        return None;
    }
    let base = stack.get_spacebase(0).ok()?;
    let memory = stack.get_contain()?.get_index();
    let block = data
        .bblocks_ref()
        .get_start_block(data.bblocks_root_pub())
        .ok()?;
    if data.bblocks_ref().block(block).size_in() != 0 {
        return None;
    }
    let mut found = HashSet::new();
    let mut next = data.bb_op_head(block);
    while let Some(id) = next {
        let op = data.obank().get(id)?;
        if op.is_call() || matches!(op.code(), OpCode::CPUI_CALLOTHER | OpCode::CPUI_STORE) {
            break;
        }
        if let Some(out) = op.get_out() {
            let value = data.vbank().get(out)?;
            let space = value.get_space().get_index();
            if space == stack.get_index() || space == memory {
                break;
            }
            if op.code() == OpCode::CPUI_LOAD && value.get_size() as u32 == ret.size {
                let segment = data.vbank().get(op.get_in(0)?)?;
                let ptr = data.vbank().get(op.get_in(1)?)?;
                if segment.is_constant()
                    && segment.get_offset() == memory as u64
                    && ptr.is_input()
                    && ptr.get_space().get_index() == base.space.as_ref()?.get_index()
                    && ptr.get_offset() == base.offset
                    && ptr.get_size() as u32 == base.size
                {
                    found.insert(out);
                }
            }
        }
        next = op.basic_neighbours().1;
    }
    Some(found)
}

fn projection(data: &Funcdata, vn: VarnodeId, guard: &ReturnGuard) -> Option<(OpId, Vec<i32>)> {
    let value = data.vbank().get(vn)?;
    let def = value.get_def()?;
    let op = data.obank().get(def)?;
    let source = |slot| data.vbank().get(op.get_in(slot)?);
    let constant = |slot| {
        let bits = source(slot)?;
        if bits.is_constant() {
            return Some(bits.get_offset());
        }
        let neg = data.obank().get(bits.get_def()?)?;
        let operand = data.vbank().get(neg.get_in(0)?)?;
        (neg.code() == OpCode::CPUI_INT_2COMP && operand.is_constant()).then(|| {
            operand.get_offset().wrapping_neg() & kuna_base::address::calc_mask(bits.get_size())
        })
    };
    let slots = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => vec![0],
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).collect(),
        OpCode::CPUI_SUBPIECE
            if source(1)?.is_constant()
                && source(1)?.get_offset() == 0
                && value.get_size() as u32 == data.get_address().get_space()?.get_addr_size() =>
        {
            vec![0]
        }
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD => {
            let zero = (0..2).find(|&slot| constant(slot) == Some(0))?;
            vec![1 - zero]
        }
        OpCode::CPUI_INT_AND if data.obank().get(guard.selector)?.get_addr() == op.get_addr() => {
            let (size, bits) = guard.mask?;
            if value.get_size() != size {
                return None;
            }
            let mask = (0..2).find(|&slot| constant(slot).is_some_and(|v| v & bits == bits))?;
            vec![1 - mask]
        }
        _ => return None,
    };
    Some((def, slots))
}
