//! A register value cannot publish shared frame bytes before their native store.

use crate::context::HighVariableId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;

const MAX_GUARD_SCAN_BLOCKS: usize = 256;
const MAX_GUARD_SCAN_OPS: usize = 8192;

pub(crate) fn moves_write(fd: &Funcdata, storage: HighVariableId, value: HighVariableId) -> bool {
    if !fd.get_arch().stack_views || storage == value {
        return false;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    let Some(scope) = fd.get_scope_local() else {
        return false;
    };
    let Some(backing) = fd.high_bank().get(storage) else {
        return false;
    };
    if !(0..backing.num_instances()).any(|i| {
        fd.vbank().get(backing.get_instance(i)).is_some_and(|v| {
            v.is_addr_tied()
                && v.get_space().get_index() == stack.get_index()
                && scope
                    .query_container_for_link_width(
                        v.get_addr(),
                        v.get_size(),
                        &Address::new_invalid(),
                    )
                    .and_then(|info| info.sym_type)
                    .is_some_and(|ty| ty.get_name().starts_with("stack_views_"))
        })
    }) {
        return false;
    }
    let Some(high) = fd.high_bank().get(value) else {
        return false;
    };
    let has_addr_tied_value = (0..high.num_instances()).any(|i| {
        fd.vbank()
            .get(high.get_instance(i))
            .is_some_and(|v| v.is_addr_tied())
    });
    for i in 0..high.num_instances() {
        let Some(source) = fd.vbank().get(high.get_instance(i)) else {
            continue;
        };
        for id in source.descend_iter() {
            let Some(store) = fd.obank().get(id) else {
                continue;
            };
            if store.code() != OpCode::CPUI_COPY || store.is_dead() {
                continue;
            }
            let Some(output) = store.get_out().and_then(|v| fd.vbank().get(v)) else {
                continue;
            };
            if output.get_high() != Some(storage)
                || !output.is_stack_store()
                || output.get_space().get_index() != stack.get_index()
            {
                continue;
            }
            let Some(info) = scope.query_container_for_link_width(
                output.get_addr(),
                output.get_size(),
                &Address::new_invalid(),
            ) else {
                continue;
            };
            if !info
                .sym_type
                .is_some_and(|ty| ty.get_name().starts_with("stack_views_"))
            {
                continue;
            }
            // A call guard gives this frame range a second, post-call value.
            // If the source value is merged into the backing, this native store
            // becomes an internal COPY and can disappear from C even though the
            // callee receives the object by pointer. Keep the source separate so
            // the stack write remains an assignment before the call.
            // Byte and word stores carry scalar frame views. Wider address-tied
            // values, especially pointers, retain their existing merge policy.
            if output.get_size() <= 2
                && output.get_size() < stack.get_addr_size() as i32
                && later_call_guard(
                    fd,
                    id,
                    output.get_addr(),
                    output.get_size(),
                    info.symbol,
                    stack,
                )
            {
                return true;
            }
            if has_addr_tied_value {
                continue;
            }
            let Some(definition) =
                crate::p6_variables::kuna_globalorder::value_definition(fd, high.get_instance(i))
            else {
                continue;
            };
            let Some(start) = fd.obank().get(definition) else {
                continue;
            };
            if start.get_parent() != store.get_parent() {
                return true;
            }
            let Some(block) = start.get_parent() else {
                continue;
            };
            let mut within = false;
            for between in fd.bb_ops(block) {
                if between == definition {
                    within = true;
                    continue;
                }
                if between == id {
                    break;
                }
                let Some(op) = fd.obank().get(between) else {
                    continue;
                };
                if within
                    && (op.is_call()
                        || matches!(
                            op.code(),
                            OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER
                        )
                        || op
                            .get_out()
                            .and_then(|v| fd.vbank().get(v))
                            .is_some_and(|v| v.is_stack_store()))
                {
                    return true;
                }
            }
        }
    }
    false
}

fn later_call_guard(
    fd: &Funcdata,
    store: crate::context::OpId,
    address: &Address,
    size: i32,
    symbol: crate::database::SymbolId,
    stack: &std::rc::Rc<kuna_base::space::AddrSpace>,
) -> bool {
    let Some(block) = fd.obank().get(store).and_then(|op| op.get_parent()) else {
        return false;
    };
    let ops = fd.bb_ops(block);
    let Some(store_index) = ops.iter().position(|&id| id == store) else {
        return false;
    };
    let mut pending = vec![(block, store_index + 1)];
    let mut visited = BTreeSet::new();
    let mut scanned_ops = 0;
    while let Some((block, start)) = pending.pop() {
        if !visited.insert((block, start != 0)) {
            continue;
        }
        if visited.len() > MAX_GUARD_SCAN_BLOCKS {
            // Keep the store if a very large CFG prevents a cheap proof.
            return true;
        }
        for id in fd.bb_ops(block).iter().skip(start).copied() {
            scanned_ops += 1;
            if scanned_ops > MAX_GUARD_SCAN_OPS {
                return true;
            }
            if is_matching_call_guard(fd, id, address, size, symbol, stack) {
                return true;
            }
        }
        let graph = fd.bblocks_ref();
        let flow = graph.block(block);
        for i in 0..flow.size_out() {
            pending.push((flow.get_out(i), 0));
        }
    }
    false
}

fn is_matching_call_guard(
    fd: &Funcdata,
    id: crate::context::OpId,
    address: &Address,
    size: i32,
    symbol: crate::database::SymbolId,
    stack: &std::rc::Rc<kuna_base::space::AddrSpace>,
) -> bool {
    let Some(indirect) = fd
        .obank()
        .get(id)
        .filter(|op| !op.is_dead() && op.code() == OpCode::CPUI_INDIRECT)
    else {
        return false;
    };
    let Some(effect) = indirect.get_in(1).and_then(|vn| fd.vbank().get(vn)) else {
        return false;
    };
    if fd
        .get_arch()
        .manage()
        .get_iop_space()
        .is_none_or(|iop| effect.get_space().get_index() != iop.get_index())
    {
        return false;
    }
    let call = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
    if !fd.obank().get(call).is_some_and(|op| op.is_call()) {
        return false;
    }
    let Some(output) = indirect.get_out().and_then(|vn| fd.vbank().get(vn)) else {
        return false;
    };
    if output.get_space().get_index() != stack.get_index()
        || (output.get_addr().overlap(0, address, size) < 0
            && address.overlap(0, output.get_addr(), output.get_size()) < 0)
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
        .is_some_and(|guarded| guarded.symbol == symbol)
}
