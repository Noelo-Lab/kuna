//! A register value cannot publish shared frame bytes before their native store.

use crate::context::HighVariableId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

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
    if (0..high.num_instances()).any(|i| {
        fd.vbank()
            .get(high.get_instance(i))
            .is_some_and(|v| v.is_addr_tied())
    }) {
        return false;
    }
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
