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
//! (a join that found no common class, a `null`), the merge is kept. Where
//! the LocalVariableTable declares one local at every store to both, and that
//! local's type admits every value stored and every use that names a type (a
//! receiver, an argument, a returned or stored value), the merge is made with
//! that type instead: it is the source's variable (`Set<Modifier> m =
//! e.getModifiers(); ... m = new LinkedHashSet<>(m);`).

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::exit_exprent::Type as ExitType;
use crate::function_exprent::FunctionType;
use crate::rt::try_catch;
use crate::statement::Statement;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

/// What `mergeVars` does with a merge the fix was asked about.
pub enum Merge {
    /// Vineflower's merge, with `getMergedType`'s type.
    Plain,
    /// The merge, with the type of the local the LocalVariableTable declares.
    Widened(VarType),
    Refused,
}

type Def = (VarVersionPair, Exprent, VarExprent);

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
    let mut defs: Vec<Def> = Vec::new();
    collect_assignments(root, &mut defs);
    let mut seen = vec![from, to];
    [from, to]
        .iter()
        .any(|v| var_wider(*v, merged, &defs, lower, &mut seen, true))
}

fn collect_assignments(stat: Statement, out: &mut Vec<Def>) {
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
                    out.push((VarVersionPair::from_var(v), a.get_right(), v));
                }
            }
        }
    }
}

fn var_wider(
    var: VarVersionPair,
    merged: &VarType,
    defs: &[Def],
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
        .filter(|(v, _, _)| *v == var)
        .map(|(_, e, _)| *e)
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
    defs: &[Def],
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

/// The type of the one local the LocalVariableTable declares at every store
/// to `from` and to `to`, when that type admits every value stored to either
/// and every use of either that names a type (any, for `Object`, which the
/// writer casts wherever a narrower type is needed).
pub fn declared_type(
    root: Statement,
    varproc: VarProcessor,
    from: VarVersionPair,
    to: VarVersionPair,
) -> Option<VarType> {
    let slot = varproc.get_var_original_index(from.var)?;
    if varproc.get_var_original_index(to.var) != Some(slot) {
        return None;
    }
    let candidates = varproc.get_candidates(slot)?;
    let mut defs: Vec<Def> = Vec::new();
    collect_assignments(root, &mut defs);
    let mut local: Option<LocalVariable> = None;
    for var in [from, to] {
        let mut stored = false;
        for (_, _, left) in defs.iter().filter(|(v, _, _)| *v == var) {
            let entry = store_local(&candidates, *left)?;
            match &local {
                Some(l) if !same_local(l, &entry) => return None,
                Some(_) => {}
                None => local = Some(entry),
            }
            stored = true;
        }
        if !stored {
            return None;
        }
    }
    let declared = local?.get_var_type();
    if declared.type_ != CodeType::Object {
        return None;
    }
    let stored = defs
        .iter()
        .filter(|(v, _, _)| *v == from || *v == to)
        .all(|(_, value, _)| admits(&declared, *value));
    let used = is_object(&declared) || {
        let mut all: Vec<Exprent> = Vec::new();
        collect_exprents(root, &mut all);
        all.into_iter()
            .all(|e| operands_admit(e, [from, to], &declared))
    };
    (stored && used).then_some(declared)
}

fn collect_exprents(stat: Statement, out: &mut Vec<Exprent>) {
    let top: Vec<Exprent> = match stat.get_exprents() {
        Some(exprents) => exprents.iter().collect(),
        None => {
            for st in stat.get_stats().borrow().iter() {
                collect_exprents(*st, out);
            }
            stat.get_stat_exprents()
        }
    };
    for e in top {
        out.extend(e.get_all_exprents_rec(true));
        out.push(e);
    }
}

fn is_one_of(e: Exprent, vars: [VarVersionPair; 2]) -> bool {
    e.try_as_var()
        .is_some_and(|v| vars.contains(&VarVersionPair::from_var(v)))
}

/// Whether the operands of `e` that are one of `vars` accept a value of the
/// type `declared` where `e` names the type it needs: an invocation's
/// receiver and arguments, a field's object, a returned value, the value
/// stored to another variable, field or element. An arm of a conditional is
/// not accepted, its type being the conditional's.
fn operands_admit(e: Exprent, vars: [VarVersionPair; 2], declared: &VarType) -> bool {
    let fits = |operand: Option<Exprent>, needed: Option<VarType>| {
        !operand.is_some_and(|o| is_one_of(o, vars))
            || needed.is_some_and(|t| assignable(declared, &t))
    };
    let invocation = e
        .try_as_invocation()
        .or_else(|| e.try_as_new().and_then(|n| n.get_constructor()));
    if let Some(inv) = invocation {
        let owner = inv.get_classname();
        let owner = (!owner.starts_with('[')).then(|| VarType::new_signature_cl(&owner, true));
        if !fits(inv.get_instance(), owner) {
            return false;
        }
        let args = inv.get_lst_parameters().to_vec();
        if args.iter().any(|a| is_one_of(*a, vars)) {
            let Ok(descriptor) = try_catch(|| inv.get_descriptor()) else {
                return false;
            };
            let params = descriptor.params.clone();
            if params.len() != args.len()
                || !args.iter().zip(params).all(|(a, p)| fits(Some(*a), Some(p)))
            {
                return false;
            }
        }
        return true;
    }
    if let Some(field) = e.try_as_field() {
        let owner = VarType::new_signature_cl(&field.get_classname(), true);
        return fits(field.get_instance(), Some(owner));
    }
    if let Some(exit) = e.try_as_exit() {
        return exit.get_exit_type() != ExitType::Return || fits(exit.get_value(), exit.get_ret_type());
    }
    if let Some(a) = e.try_as_assignment() {
        return is_one_of(a.get_left(), vars) || fits(Some(a.get_right()), Some(a.get_left().get_expr_type()));
    }
    if let Some(f) = e.try_as_function() {
        if f.get_func_type() == FunctionType::Ternary {
            let ops = f.get_lst_operands();
            return !is_one_of(ops.get(1), vars) && !is_one_of(ops.get(2), vars);
        }
    }
    true
}

/// The entry of `candidates` whose scope a store by `left` initialises or
/// lies in: the store's instruction is `left`'s only bytecode.
fn store_local(candidates: &[LocalVariable], left: VarExprent) -> Option<LocalVariable> {
    let bytecode = left.0.get_bytecode()?;
    let first = usize::try_from(bytecode.next_set_bit(0)).ok()?;
    let next = bytecode.length();
    if bytecode.next_clear_bit(first) != next || next - first > 4 {
        return None;
    }
    let next = i32::try_from(next).ok()?;
    let starting: Vec<&LocalVariable> = candidates.iter().filter(|l| l.get_start() == next).collect();
    let found: Vec<&LocalVariable> = if starting.is_empty() {
        candidates
            .iter()
            .filter(|l| l.get_start() < next && next <= l.get_end())
            .collect()
    } else {
        starting
    };
    match found.as_slice() {
        [one] => Some((*one).clone()),
        _ => None,
    }
}

fn same_local(a: &LocalVariable, b: &LocalVariable) -> bool {
    a.compare_to(b) == 0 && a.name == b.name && a.descriptor == b.descriptor
}

/// Whether a value of `e` can be stored in a variable of the reference type
/// `declared` without a cast, following conditionals.
fn admits(declared: &VarType, e: Exprent) -> bool {
    if e.try_as_const().is_some_and(|c| c.is_null()) {
        return true;
    }
    if let Some(a) = e.try_as_assignment() {
        return admits(declared, a.get_right());
    }
    if let Some(f) = e.try_as_function() {
        if f.get_func_type() == FunctionType::Ternary {
            let ops = f.get_lst_operands();
            return admits(declared, ops.get(1)) && admits(declared, ops.get(2));
        }
    }
    assignable(&e.get_expr_type(), declared)
}

/// Whether a value of the reference type `t` is a `to` without a cast.
fn assignable(t: &VarType, to: &VarType) -> bool {
    if t.type_ != CodeType::Object || to.type_ != CodeType::Object {
        return false;
    }
    if t.equals(to) || is_object(to) {
        return true;
    }
    match (t.value.as_deref(), to.value.as_deref()) {
        (Some(tv), Some(dv)) if t.array_dim == to.array_dim => {
            DecompilerContext::get_struct_context().instance_of(tv, dv)
        }
        _ => false,
    }
}

fn is_object(t: &VarType) -> bool {
    t.type_ == CodeType::Object && t.array_dim == 0 && t.value.as_deref() == Some("java/lang/Object")
}
