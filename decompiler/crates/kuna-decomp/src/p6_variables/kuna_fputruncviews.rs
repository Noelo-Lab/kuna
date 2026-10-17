//! Separate numerical integer results from the floating view transporting their bits.

use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId};
use crate::dtype::type_metatype as M;
use crate::funcdata::Funcdata;
use crate::merge::MergeContext;
use crate::op::pcodeop_addlflags;
use crate::options::on_or_off;

fn input_type(fd: &Funcdata, op: OpId, slot: i32) -> std::rc::Rc<crate::dtype::Datatype> {
    if slot > 0 {
        let proto = match fd.obank().get(op).map(|operation| operation.code()) {
            Some(OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) => fd
                .get_call_specs_index(op)
                .map(|index| fd.get_call_specs(index).proto()),
            Some(OpCode::CPUI_RETURN) => Some(fd.get_func_proto()),
            _ => None,
        };
        if let Some(proto) = proto.filter(|proto| proto.has_store()) {
            let ty = if fd.obank().get(op).unwrap().code() == OpCode::CPUI_RETURN {
                proto.get_output_type()
            } else {
                proto.get_param(slot - 1).and_then(|param| param.get_type())
            };
            if let Some(ty) = ty {
                return std::rc::Rc::clone(ty);
            }
        }
    }
    MergeContext::op_input_type_local(fd, op, slot)
}

fn marker_view(
    fd: &mut Funcdata,
    op: OpId,
    roots: &std::collections::BTreeSet<crate::context::VarnodeId>,
) -> bool {
    let Some(operation) = fd.obank().get(op) else {
        return false;
    };
    if !operation.is_marker() {
        return false;
    }
    let Some(out) = operation.get_out() else {
        return false;
    };
    let inputs: Vec<_> = (0..operation.num_input())
        .filter_map(|slot| operation.get_in(slot))
        .collect();
    let Some(value) = fd.vbank().get(out) else {
        return false;
    };
    let size = value.get_size();
    for vn in std::iter::once(out).chain(inputs) {
        if roots.contains(&vn) {
            continue;
        }
        let Some(value) = fd.vbank().get(vn) else {
            continue;
        };
        if value.get_size() != size {
            continue;
        }
        let high = value.get_high();
        let local = value.get_type().clone();
        let ty = high
            .map(|high| MergeContext::high_get_type(fd, high))
            .unwrap_or(local);
        if ty.get_size() == size && ty.get_metatype() == M::TYPE_FLOAT {
            return true;
        }
    }
    false
}

pub(crate) fn recover(fd: &mut Funcdata) -> KunaResult<()> {
    if !fd.get_arch().fpu_trunc_views {
        return Ok(());
    }
    let operations: Vec<_> = fd
        .obank()
        .iter_alive()
        .filter(|op| {
            fd.obank()
                .get(*op)
                .is_some_and(|operation| operation.code() == OpCode::CPUI_FLOAT_TRUNC)
        })
        .collect();
    let roots: Vec<_> = operations
        .iter()
        .filter_map(|op| {
            fd.obank()
                .get(*op)
                .and_then(|operation| operation.get_out())
        })
        .collect();
    let numeric_roots: std::collections::BTreeSet<_> = roots
        .into_iter()
        .filter(|out| {
            let Some(value) = fd.vbank().get(*out) else {
                return false;
            };
            if value.is_type_lock()
                || value.is_mapped()
                || value.is_addr_tied()
                || value.is_proto_partial()
                || !matches!(value.get_size(), 4 | 8)
            {
                return false;
            }
            let Some(high) = value.get_high() else {
                return false;
            };
            !fd.high_is_type_lock(high)
                && !fd.high_is_name_lock(high)
                && !fd.high_is_persist(high)
                && fd.high_piece(high).is_none()
                && fd.high_num_instances(high) == 1
        })
        .collect();
    for op in operations {
        let Some(operation) = fd.obank().get(op) else {
            continue;
        };
        let point = operation.get_addr().clone();
        let Some(out) = operation.get_out() else {
            continue;
        };
        let Some(value) = fd.vbank().get(out) else {
            continue;
        };
        let size = value.get_size();
        if value.is_type_lock()
            || value.is_mapped()
            || value.is_addr_tied()
            || value.is_proto_partial()
            || !matches!(size, 4 | 8)
        {
            continue;
        }
        let Some(high) = value.get_high() else {
            continue;
        };
        let current = value.get_type().clone();
        if fd.high_is_type_lock(high)
            || fd.high_is_name_lock(high)
            || fd.high_is_persist(high)
            || fd.high_piece(high).is_some()
            || fd.high_num_instances(high) != 1
        {
            continue;
        }
        let mut users: Vec<_> = fd.vbank().get(out).unwrap().descend_iter().collect();
        users.sort_unstable();
        users.dedup();
        if users.iter().any(|user| {
            fd.obank().get(*user).is_some_and(|operation| {
                operation.code() == OpCode::CPUI_COPY
                    && operation.get_in(0) == Some(out)
                    && operation.get_addlflags() & pcodeop_addlflags::kuna_fpu_view_copy != 0
            })
        }) {
            continue;
        }
        if users.iter().any(|user| {
            fd.obank().get(*user).is_some_and(|operation| {
                if operation.code() == OpCode::CPUI_PIECE {
                    return true;
                }
                operation
                    .get_out()
                    .and_then(|out| fd.vbank().get(out))
                    .is_some_and(|value| {
                        let ty = value.get_type();
                        operation.code() == OpCode::CPUI_SUBPIECE
                            && value.get_size() != size
                            && (ty.get_metatype() == M::TYPE_FLOAT
                                || ty
                                    .get_partial_base()
                                    .is_some_and(|base| base.get_metatype() == M::TYPE_FLOAT))
                    })
            })
        }) {
            continue;
        }
        if users.iter().any(|user| {
            let Some(operation) = fd.obank().get(*user) else {
                return false;
            };
            (0..operation.num_input()).any(|slot| {
                operation.get_in(slot) == Some(out) && {
                    let ty = input_type(fd, *user, slot);
                    ty.get_metatype() == M::TYPE_FLOAT && ty.get_size() != size
                }
            })
        }) {
            continue;
        }
        let mut mixed = current.get_metatype() == M::TYPE_FLOAT && current.get_size() == size;
        for user in &users {
            mixed |= marker_view(fd, *user, &numeric_roots);
            let Some(operation) = fd.obank().get(*user) else {
                continue;
            };
            if !operation.is_marker() && operation.code() != OpCode::CPUI_COPY {
                continue;
            }
            let Some(output) = operation.get_out() else {
                continue;
            };
            let Some(value) = fd.vbank().get(output) else {
                continue;
            };
            if value.get_size() != size {
                continue;
            }
            let Some(target) = value.get_high() else {
                continue;
            };
            let ty = MergeContext::high_get_type(fd, target);
            mixed |= ty.get_metatype() == M::TYPE_FLOAT && ty.get_size() == size;
        }
        if !mixed {
            continue;
        }
        let Some(factory) = fd.get_arch().types() else {
            continue;
        };
        let meta = if matches!(current.get_metatype(), M::TYPE_INT | M::TYPE_UINT) {
            current.get_metatype()
        } else {
            M::TYPE_INT
        };
        let (Ok(integer), Ok(float)) = (
            factory.get_base(size, meta),
            factory.get_base(size, M::TYPE_FLOAT),
        ) else {
            continue;
        };
        let mut uses = Vec::new();
        for user in users {
            let marker = marker_view(fd, user, &numeric_roots);
            let Some(operation) = fd.obank().get(user) else {
                continue;
            };
            let code = operation.code();
            let output = operation.get_out();
            let slots: Vec<_> = (0..operation.num_input())
                .filter(|slot| operation.get_in(*slot) == Some(out))
                .collect();
            let logical_copy = code == OpCode::CPUI_COPY
                && output.is_some_and(|output| {
                    let Some(value) = fd.vbank().get(output) else {
                        return false;
                    };
                    let Some(high) = value.get_high() else {
                        return false;
                    };
                    let ty = MergeContext::high_get_type(fd, high);
                    if ty.get_size() != size
                        || !matches!(ty.get_metatype(), M::TYPE_INT | M::TYPE_UINT)
                    {
                        return false;
                    }
                    fd.vbank()
                        .get(output)
                        .unwrap()
                        .descend_iter()
                        .all(|reader| {
                            let Some(read) = fd.obank().get(reader) else {
                                return false;
                            };
                            !read.is_marker()
                                && read.code() != OpCode::CPUI_COPY
                                && (0..read.num_input())
                                    .filter(|slot| read.get_in(*slot) == Some(output))
                                    .all(|slot| {
                                        matches!(
                                            input_type(fd, reader, slot).get_metatype(),
                                            M::TYPE_INT | M::TYPE_UINT
                                        )
                                    })
                        })
                });
            for slot in slots {
                let ty = input_type(fd, user, slot);
                let inferred_float_abi = slot > 0
                    && ty.get_metatype() == M::TYPE_UNKNOWN
                    && current.get_metatype() == M::TYPE_FLOAT
                    && match code {
                        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                            crate::kuna_floatreg::passed_in_a_float_register(fd, user, slot)
                        }
                        OpCode::CPUI_RETURN => fd.vbank().get(out).is_some_and(|node| {
                            crate::kuna_floatreg::returned_in_a_float_register(fd, out, node)
                        }),
                        _ => false,
                    };
                if marker
                    || (code == OpCode::CPUI_COPY && !logical_copy)
                    || ty.get_metatype() == M::TYPE_FLOAT
                    || inferred_float_abi
                {
                    uses.push((user, slot));
                }
            }
        }
        if uses.is_empty() {
            fd.vn_update_type(out, integer);
            continue;
        }
        let bridge = fd.new_op(1, point);
        fd.op_set_opcode_code(bridge, OpCode::CPUI_COPY);
        let view = fd.new_unique_out(size, bridge)?;
        fd.vn_update_type(view, float);
        fd.op_set_input(bridge, out, 0)?;
        fd.obank_mut()
            .get_mut(bridge)
            .unwrap()
            .set_additional_flag(pcodeop_addlflags::kuna_fpu_view_copy);
        fd.op_insert_after(bridge, op);
        fd.kuna_fpu_trunc_views.push(bridge);
        for (user, slot) in uses {
            fd.op_set_input(user, view, slot)?;
        }
        fd.vn_update_type(out, integer);
        if let Some(value) = fd.vbank_mut().get_mut(out) {
            value.clear_implied();
            value.set_explicit();
        }
    }
    Ok(())
}

pub(crate) fn keeps_apart(ctx: &dyn MergeContext, op: OpId) -> bool {
    ctx.fpu_trunc_views_enabled() && ctx.op_is_fpu_trunc_view(op)
}

pub(crate) fn keeps_highs_apart(
    ctx: &dyn MergeContext,
    a: HighVariableId,
    b: HighVariableId,
) -> bool {
    ctx.fpu_trunc_views_enabled() && ctx.fpu_trunc_view_highs(a, b)
}

/// A logical integer root never joins an unrelated variable speculatively.
pub(crate) fn keeps_root_alone(
    ctx: &dyn MergeContext,
    a: HighVariableId,
    b: HighVariableId,
) -> bool {
    ctx.fpu_trunc_views_enabled() && (ctx.fpu_trunc_root_high(a) || ctx.fpu_trunc_root_high(b))
}

pub struct OptionFpuTruncViews;
impl OptionFpuTruncViews {
    pub fn apply(&self, value: &str) -> KunaResult<(bool, String)> {
        let enabled = on_or_off(value)?;
        Ok((
            enabled,
            format!(
                "Numerical integer conversion views {}",
                if enabled { "on" } else { "off" }
            ),
        ))
    }
}

#[cfg(test)]
#[path = "kuna_fputruncviews/tests.rs"]
mod tests;
