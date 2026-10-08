//! Memory SSA reconstruction does not perform additional writes to frame bytes.

use std::collections::{HashSet, VecDeque};

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub(crate) fn prepare(fd: &mut Funcdata) {
    if !fd.get_arch().stack_views {
        return;
    }
    let mut versions = HashSet::new();
    let mut copies = HashSet::new();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        if physical_version(fd, id) {
            versions.insert(id);
        } else if matches!(
            op.code(),
            OpCode::CPUI_COPY
                | OpCode::CPUI_CAST
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_PIECE
        ) && op
            .get_out()
            .and_then(|v| fd.vbank().get(v))
            .is_some_and(|v| !v.is_addr_tied() && !v.is_persist() && !v.is_stack_store())
        {
            copies.insert(id);
        }
    }
    let mut related = HashSet::new();
    let mut ancestors: Vec<_> = versions.iter().copied().collect();
    while let Some(id) = ancestors.pop() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        for slot in 0..op.num_input() {
            let Some(def) = op
                .get_in(slot)
                .and_then(|v| fd.vbank().get(v))
                .and_then(|v| v.get_def())
            else {
                continue;
            };
            if copies.contains(&def) && related.insert(def) {
                ancestors.push(def);
            }
        }
    }
    copies = related;
    let mut live = HashSet::new();
    let mut queue = VecDeque::new();
    for &id in &copies {
        let Some(output) = fd
            .obank()
            .get(id)
            .and_then(|op| op.get_out())
            .and_then(|v| fd.vbank().get(v))
        else {
            continue;
        };
        if output.descend_iter().any(|consumer| {
            !copies.contains(&consumer)
                && !versions.contains(&consumer)
                && fd.obank().get(consumer).is_some_and(|op| !op.not_printed())
        }) {
            live.insert(id);
            queue.push_back(id);
        }
    }
    while let Some(id) = queue.pop_front() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        for slot in 0..op.num_input() {
            let Some(def) = op
                .get_in(slot)
                .and_then(|v| fd.vbank().get(v))
                .and_then(|v| v.get_def())
            else {
                continue;
            };
            if copies.contains(&def) && live.insert(def) {
                queue.push_back(def);
            }
        }
    }
    let unused_set: HashSet<_> = copies.difference(&live).copied().collect();
    let mut unused: Vec<_> = unused_set.iter().copied().collect();
    unused.sort_by_key(|id| fd.obank().get(*id).map(|op| op.get_time()));
    for &id in &unused {
        let Some(output) = fd.obank().get(id).and_then(|op| op.get_out()) else {
            continue;
        };
        let Some(high) = fd.vbank().get(output).and_then(|v| v.get_high()) else {
            continue;
        };
        let shares_value = fd.high_bank().get(high).is_some_and(|h| {
            (0..h.num_instances()).any(|i| {
                fd.vbank().get(h.get_instance(i)).is_some_and(|v| {
                    v.is_input()
                        || v.get_def().is_some_and(|def| {
                            !versions.contains(&def)
                                && !unused_set.contains(&def)
                                && fd
                                    .obank()
                                    .get(def)
                                    .is_some_and(|op| !op.is_marker() && !op.not_printed())
                        })
                })
            })
        });
        if shares_value {
            fd.high_remove_member(high, output);
            let _ = fd.assign_high_var(output);
        }
    }
    for id in versions.into_iter().chain(unused) {
        fd.op_mark_non_printing_pub(id);
    }
}

fn physical_version(fd: &Funcdata, id: OpId) -> bool {
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    let Some(output) = op.get_out().and_then(|v| fd.vbank().get(v)) else {
        return false;
    };
    if output.is_stack_store() || op.is_call() {
        return false;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    if output.get_space().get_index() != stack.get_index() {
        return false;
    }
    if op.code() == OpCode::CPUI_COPY
        && op
            .get_in(0)
            .and_then(|v| fd.vbank().get(v))
            .is_some_and(|v| {
                v.is_input()
                    && v.get_addr() == output.get_addr()
                    && v.get_size() == output.get_size()
            })
    {
        return false;
    }
    fd.get_scope_local()
        .and_then(|scope| {
            scope.query_container_for_link_width(
                output.get_addr(),
                output.get_size(),
                &Address::new_invalid(),
            )
        })
        .and_then(|info| info.sym_type)
        .is_some_and(|ty| ty.get_name().starts_with("stack_views_"))
}
