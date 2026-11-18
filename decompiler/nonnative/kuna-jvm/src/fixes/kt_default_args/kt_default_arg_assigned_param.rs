//! kuna fix `kt_default_arg_assigned_param` (`kuna_fixes`), called from
//! `DefaultArgsMap::from`: a parameter's default value is what its
//! default-arguments branch assigns to the parameter. Vineflower takes the
//! right side of the branch's first statement whatever that statement assigns,
//! so a default computed through a local printed the local's initial value.
//! Two branch shapes give a default: a single assignment to a parameter slot,
//! and kotlinc's checked temporary `tmp = E; Intrinsics.check..(E or tmp, ..); param = tmp`,
//! whose default is `E`. Any other branch gives none (the unknown-default
//! placeholder).

use crate::exprent::Exprent;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const NULL_CHECKS: &[&str] = &[
    "checkNotNull",
    "checkNotNullExpressionValue",
    "checkExpressionValueIsNotNull",
];

/// The default the branch statements `exprents` assign to a parameter, whose
/// slots are those below `first_mask_slot`.
pub fn default_value(exprents: &[Exprent], first_mask_slot: i32) -> Option<Exprent> {
    let (last, before) = exprents.split_last()?;
    let assignment = last.try_as_assignment()?;
    let param = assignment.get_left().try_as_var()?;
    if param.get_index() < 0 || param.get_index() >= first_mask_slot {
        return None;
    }
    let right = assignment.get_right();
    let Some((first, checks)) = before.split_first() else {
        return Some(right);
    };

    let temp = right.try_as_var()?.get_index();
    if temp < first_mask_slot {
        return None;
    }
    let init = first.try_as_assignment()?;
    if init.get_left().try_as_var()?.get_index() != temp {
        return None;
    }
    let value = init.get_right();
    let checks_temp = |check: &Exprent| {
        check.try_as_invocation().is_some_and(|call| {
            call.get_classname() == INTRINSICS
                && NULL_CHECKS.contains(&call.get_name().as_str())
                && call
                    .get_lst_parameters()
                    .to_vec()
                    .first()
                    .is_some_and(|&arg| {
                        arg.try_as_var().is_some_and(|arg| arg.get_index() == temp)
                            || arg.equals(value)
                    })
        })
    };
    if !checks.iter().all(checks_temp) {
        return None;
    }
    Some(value)
}
