//! Fix `var_merge_narrowing`: `VarDefinitionHelper.mergeVars` does not merge two
//! variables when the merged type is narrower than a value one of them is
//! assigned.
//!
//! `getMergedType` may give the merged variable an upper bound (`fromMax`) or
//! the narrower of the two lower bounds. Every assignment whose value is not
//! of that type is then printed with a cast the bytecode does not have there.
//! A Kotlin suspend loop shows it: `Object v = src.next(c); if (v ==
//! SUSPENDED) return v; Integer w = (Integer)v;` merged `w` into `v` as an
//! `Integer`, printed `Integer v = (Integer)src.next(c)`, which throws exactly
//! where the bytecode returns the marker. A merge is refused when the merged
//! type is strictly below the type of a value assigned to either variable
//! (through conditionals and copies of other variables, whose own bounds may
//! be a loose join); two variables are always correct, the versions having
//! been split by SSA. Where the merged type only narrows a loose lower bound
//! (a join that found no common class, a `null`), the merge is kept.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::statement::Statement;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

/// True when `merged` is a strict subtype of `from_min` or of `to_min`.
pub fn narrows(merged: &VarType, from_min: &VarType, to_min: &VarType) -> bool {
    strictly_below(merged, from_min) || strictly_below(merged, to_min)
}

/// True when `t` is a reference type strictly below the reference type `bound`.
pub fn strictly_below(t: &VarType, bound: &VarType) -> bool {
    if t.type_ != CodeType::Object || bound.type_ != CodeType::Object || t.equals(bound) {
        return false;
    }
    if bound.array_dim == 0 && bound.value.as_deref() == Some("java/lang/Object") {
        return true;
    }
    match (t.value.as_deref(), bound.value.as_deref()) {
        (Some(tv), Some(bv)) if t.array_dim == bound.array_dim && tv != bv => {
            DecompilerContext::get_struct_context().instance_of(tv, bv)
        }
        _ => false,
    }
}

/// Whether a value assigned to `from` or `to` anywhere under `root` has a
/// type `merged` is strictly below; `lower` gives a variable's lower bound.
pub fn assigns_wider(
    root: Statement,
    from: VarVersionPair,
    to: VarVersionPair,
    merged: &VarType,
    lower: &dyn Fn(VarVersionPair) -> Option<VarType>,
) -> bool {
    let mut defs: Vec<(VarVersionPair, Exprent)> = Vec::new();
    collect_assignments(root, &mut defs);
    let mut seen = vec![from, to];
    [from, to]
        .iter()
        .any(|v| var_wider(*v, merged, &defs, lower, &mut seen, true))
}

fn collect_assignments(stat: Statement, out: &mut Vec<(VarVersionPair, Exprent)>) {
    let mut top: Vec<Exprent> = Vec::new();
    match stat.get_exprents() {
        Some(exprents) => top.extend(exprents.iter()),
        None => {
            top.extend(stat.get_stat_exprents());
            for st in stat.get_stats().borrow().iter() {
                collect_assignments(*st, out);
            }
        }
    }
    for e in top {
        let mut all = e.get_all_exprents_rec(true);
        all.push(e);
        for x in all {
            if let Some(a) = x.try_as_assignment() {
                if let Some(v) = a.get_left().try_as_var() {
                    out.push((VarVersionPair::from_var(v), a.get_right()));
                }
            }
        }
    }
}

fn var_wider(
    var: VarVersionPair,
    merged: &VarType,
    defs: &[(VarVersionPair, Exprent)],
    lower: &dyn Fn(VarVersionPair) -> Option<VarType>,
    seen: &mut Vec<VarVersionPair>,
    start: bool,
) -> bool {
    if !start {
        match lower(var) {
            Some(t) if strictly_below(merged, &t) => {}
            _ => return false,
        }
        if seen.contains(&var) {
            return false;
        }
        seen.push(var);
    }
    let values: Vec<Exprent> = defs
        .iter()
        .filter(|(v, _)| *v == var)
        .map(|(_, e)| *e)
        .collect();
    if values.is_empty() {
        return !start;
    }
    values
        .into_iter()
        .any(|e| value_wider(e, merged, defs, lower, seen))
}

fn value_wider(
    e: Exprent,
    merged: &VarType,
    defs: &[(VarVersionPair, Exprent)],
    lower: &dyn Fn(VarVersionPair) -> Option<VarType>,
    seen: &mut Vec<VarVersionPair>,
) -> bool {
    if e.try_as_const().is_some_and(|c| c.is_null()) {
        return false;
    }
    if let Some(v) = e.try_as_var() {
        return var_wider(
            VarVersionPair::from_var(v),
            merged,
            defs,
            lower,
            seen,
            false,
        );
    }
    if let Some(a) = e.try_as_assignment() {
        return value_wider(a.get_right(), merged, defs, lower, seen);
    }
    if let Some(f) = e.try_as_function() {
        if f.get_func_type() == FunctionType::Ternary {
            let ops = f.get_lst_operands();
            return value_wider(ops.get(1), merged, defs, lower, seen)
                || value_wider(ops.get(2), merged, defs, lower, seen);
        }
    }
    strictly_below(merged, &e.get_expr_type())
}
