//! Fold a negated ordered comparison only when its NaN guards exclude unordered inputs.

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::expression::functional_equality;
use crate::funcdata::Funcdata;

fn copied_value(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..6 {
        let Some(op) = data
            .vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .and_then(|id| data.obank().get(id))
        else {
            break;
        };
        if op.code() != OpCode::CPUI_COPY {
            break;
        }
        let Some(input) = op.get_in(0) else { break };
        if data.vbank().get(vn).unwrap().get_size() != data.vbank().get(input).unwrap().get_size() {
            break;
        }
        vn = input;
    }
    vn
}

fn or_leaves(data: &Funcdata, vn: VarnodeId, depth: usize, leaves: &mut Vec<VarnodeId>) -> bool {
    if depth == 0 || leaves.len() == 8 {
        return false;
    }
    if data
        .vbank()
        .get(vn)
        .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
    {
        return true;
    }
    let Some(op) = data
        .vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|id| data.obank().get(id))
    else {
        leaves.push(vn);
        return true;
    };
    if op.code() == OpCode::CPUI_COPY {
        return op
            .get_in(0)
            .is_some_and(|input| or_leaves(data, input, depth - 1, leaves));
    }
    if matches!(op.code(), OpCode::CPUI_BOOL_OR | OpCode::CPUI_INT_OR) {
        return (0..2).all(|slot| {
            op.get_in(slot)
                .is_some_and(|input| or_leaves(data, input, depth - 1, leaves))
        });
    }
    leaves.push(vn);
    true
}

fn guarded_complement(data: &Funcdata, root: VarnodeId) -> Option<(OpCode, VarnodeId, VarnodeId)> {
    if !data
        .vbank()
        .get(root)
        .and_then(|v| v.get_def())
        .and_then(|id| data.obank().get(id))
        .is_some_and(|op| {
            matches!(
                op.code(),
                OpCode::CPUI_COPY | OpCode::CPUI_BOOL_OR | OpCode::CPUI_INT_OR
            )
        })
    {
        return None;
    }
    let mut leaves = Vec::new();
    if !or_leaves(data, root, 6, &mut leaves) || leaves.len() < 2 {
        return None;
    }
    let mut guards = Vec::new();
    let mut comparison = None;
    let mut equality = None;
    for vn in leaves {
        let Some(op) = data
            .vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .and_then(|id| data.obank().get(id))
        else {
            return None;
        };
        match op.code() {
            OpCode::CPUI_FLOAT_NAN => guards.push(op.get_in(0).unwrap()),
            OpCode::CPUI_FLOAT_LESS | OpCode::CPUI_FLOAT_LESSEQUAL if comparison.is_none() => {
                comparison = Some((op.code(), op.get_in(0).unwrap(), op.get_in(1).unwrap()));
            }
            OpCode::CPUI_FLOAT_EQUAL if equality.is_none() => {
                equality = Some((op.get_in(0).unwrap(), op.get_in(1).unwrap()));
            }
            _ => return None,
        }
    }
    let Some((mut code, a, b)) = comparison else {
        return None;
    };
    let same = |x, y| {
        functional_equality(
            copied_value(data, x),
            copied_value(data, y),
            data.vbank(),
            data.obank(),
        )
    };
    if let Some((x, y)) = equality {
        if code != OpCode::CPUI_FLOAT_LESS
            || !(same(a, x) && same(b, y) || same(a, y) && same(b, x))
        {
            return None;
        }
        code = OpCode::CPUI_FLOAT_LESSEQUAL;
    }
    if guards.is_empty() || guards.iter().any(|&g| !same(g, a) && !same(g, b)) {
        return None;
    }
    for input in [a, b] {
        let guarded = guards.iter().any(|&g| same(g, input));
        let non_nan_constant = data
            .vbank()
            .get(copied_value(data, input))
            .is_some_and(|v| {
                v.is_constant()
                    && v.get_size() <= 8
                    && data
                        .get_arch()
                        .get_float_format(v.get_size())
                        .is_some_and(|fmt| fmt.op_nan(v.get_offset()) == 0)
            });
        if !guarded && !non_nan_constant {
            return None;
        }
    }
    let complement = if code == OpCode::CPUI_FLOAT_LESS {
        OpCode::CPUI_FLOAT_LESSEQUAL
    } else {
        OpCode::CPUI_FLOAT_LESS
    };
    Some((complement, b, a))
}

/// `!(NAN(a) || NAN(b) || a <= b)` is exactly `b < a`, including NaNs.
pub(crate) fn fold_guarded_negate(data: &mut Funcdata, negate: OpId) -> bool {
    let Some(op) = data.obank().get(negate) else {
        return false;
    };
    if op.code() != OpCode::CPUI_BOOL_NEGATE {
        return false;
    }
    let Some(root) = op.get_in(0) else {
        return false;
    };
    let Some((code, a, b)) = guarded_complement(data, root) else {
        return false;
    };
    data.op_set_opcode(negate, crate::typeop::seam_type_op_for(code));
    data.op_set_all_input(negate, &[a, b])
        .expect("guarded float complement inputs");
    true
}

fn fold_guarded_branch(data: &mut Funcdata, branch: OpId) -> bool {
    let op = data.obank().get(branch).unwrap();
    let Some(root) = op.get_in(1) else {
        return false;
    };
    let Some((code, a, b)) = guarded_complement(data, root) else {
        return false;
    };
    let addr = op.get_addr().clone();
    let comparison = data.new_op(2, addr);
    data.op_set_opcode(comparison, crate::typeop::seam_type_op_for(code));
    let result = data
        .new_unique_out(1, comparison)
        .expect("guarded branch output");
    data.op_set_all_input(comparison, &[a, b])
        .expect("guarded branch inputs");
    data.op_insert_before(comparison, branch);
    data.op_set_input(branch, result, 1)
        .expect("guarded branch condition");
    data.op_flip_condition(branch);
    true
}

/// Preserve the exact guarded complement before `nanignore` discards its guards.
pub(crate) fn fold_nan_consumers(data: &mut Funcdata, nan: OpId) -> bool {
    let Some(out) = data.obank().get(nan).and_then(|op| op.get_out()) else {
        return false;
    };
    let mut pending = vec![(out, 0)];
    let mut changed = false;
    for _ in 0..32 {
        let Some((vn, depth)) = pending.pop() else {
            break;
        };
        if depth == 6 {
            continue;
        }
        let readers: Vec<_> = data
            .vbank()
            .get(vn)
            .unwrap()
            .descend_iter()
            .take(32)
            .collect();
        for reader in readers {
            let op = data.obank().get(reader).unwrap();
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_BOOL_OR | OpCode::CPUI_INT_OR => {
                    if let Some(out) = op.get_out() {
                        pending.push((out, depth + 1));
                    }
                }
                OpCode::CPUI_BOOL_NEGATE => changed |= fold_guarded_negate(data, reader),
                OpCode::CPUI_CBRANCH => changed |= fold_guarded_branch(data, reader),
                _ => {}
            }
        }
    }
    changed
}
