//! Independent SSA value families do not become one local by reusing a register.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;

pub(crate) fn single_family(fd: &Funcdata, values: &[VarnodeId]) -> bool {
    if values.len() < 2 {
        return true;
    }
    let first = values[0];
    let mut missing: BTreeSet<_> = values.iter().copied().collect();
    let mut work = vec![first];
    let mut visited = BTreeSet::new();
    let mut budget = 8192usize;
    while let Some(id) = work.pop() {
        if !visited.insert(id) {
            continue;
        }
        let Some(left) = budget.checked_sub(1) else {
            return false;
        };
        budget = left;
        missing.remove(&id);
        if missing.is_empty() {
            return true;
        }
        let Some(value) = fd.vbank().get(id) else {
            return false;
        };
        let size = value.get_size();
        if let Some(def) = value.get_def().and_then(|op| fd.obank().get(op)) {
            if let Some(slots) = slots(def.code(), def.num_input(), def.is_indirect_creation()) {
                for slot in 0..slots {
                    if let Some(input) = def.get_in(slot) {
                        if fd
                            .vbank()
                            .get(input)
                            .is_some_and(|input| !input.is_constant() && input.get_size() == size)
                        {
                            work.push(input);
                        }
                    }
                }
            }
        }
        for read in value.descend_iter() {
            let Some(op) = fd.obank().get(read) else {
                continue;
            };
            let Some(slots) = slots(op.code(), op.num_input(), op.is_indirect_creation()) else {
                continue;
            };
            if !(0..slots).any(|slot| op.get_in(slot) == Some(id)) {
                continue;
            }
            if let Some(output) = op.get_out() {
                if fd
                    .vbank()
                    .get(output)
                    .is_some_and(|output| output.get_size() == size)
                {
                    work.push(output);
                }
            }
        }
    }
    false
}

fn slots(code: OpCode, count: i32, creation: bool) -> Option<i32> {
    match code {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => Some(1),
        OpCode::CPUI_MULTIEQUAL => Some(count),
        OpCode::CPUI_INDIRECT if !creation => Some(1),
        _ => None,
    }
}
