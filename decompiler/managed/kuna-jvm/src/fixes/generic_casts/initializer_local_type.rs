//! Fix `initializer_local_type`: in a method compiled without a
//! LocalVariableTable, a local of a generic class with one assignment, whose
//! value's type gives that class known type arguments, is declared with them
//! when its every read is a call on it whose arguments do not depend on them,
//! or the iterable of a for-each whose variable takes the element type.

use std::collections::{BTreeMap, BTreeSet};

use crate::code_type::CodeType;
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exprent::Exprent;
use crate::fixes::generic_casts::mentions;
use crate::fixes::lambdas;
use crate::generic_type::GenericType;
use crate::kuna_static_types::{formals, param_type, supertype_args};
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::var_processor::{KunaLocalType, VarProcessor};
use crate::var_type::{instance_of, VarType};

/// Types the locals of `method` (body `root`, variables `processor`) the
/// module header describes.
pub fn type_locals(root: RootStatement, method: StructMethod, processor: VarProcessor) {
    if method.get_local_variable_attr().is_some() {
        return;
    }
    let mut defs: BTreeMap<i32, Vec<Exprent>> = BTreeMap::new();
    let mut total: BTreeMap<i32, usize> = BTreeMap::new();
    let mut approved: BTreeMap<i32, usize> = BTreeMap::new();
    let mut rejected: BTreeSet<i32> = BTreeSet::new();
    let exprents = crate::fixes::util::all_exprents(root.0);
    for e in &exprents {
        if let Some(var) = e.try_as_var() {
            *total.entry(var.get_index()).or_insert(0) += 1;
            if var.is_definition() {
                *approved.entry(var.get_index()).or_insert(0) += 1;
            }
        }
        for (i, child) in e.get_all_exprents().into_iter().enumerate() {
            let Some(var) = child.try_as_var() else {
                continue;
            };
            let index = var.get_index();
            if let Some(assign) = e.try_as_assignment() {
                if i == 0 {
                    if assign.get_cond_type().is_some() {
                        rejected.insert(index);
                    }
                    defs.entry(index).or_default().push(assign.get_right());
                    if !var.is_definition() {
                        *approved.entry(index).or_insert(0) += 1;
                    }
                    continue;
                }
            }
            let receiver = e.try_as_invocation().is_some_and(|inv| {
                inv.get_instance() == Some(child) && independent_arguments(inv)
            });
            if receiver && !var.is_definition() {
                *approved.entry(index).or_insert(0) += 1;
            }
        }
    }
    let mut loops: Vec<DoStatement> = Vec::new();
    collect_for_each(root.0, &mut loops);
    let mut iterated: BTreeMap<i32, Vec<VarType>> = BTreeMap::new();
    for l in loops {
        let (Some(var), Some(item)) = (
            l.get_inc_exprent().and_then(|e| e.try_as_var()),
            l.get_init_exprent().and_then(|e| e.try_as_var()),
        ) else {
            continue;
        };
        *approved.entry(var.get_index()).or_insert(0) += 1;
        iterated
            .entry(var.get_index())
            .or_default()
            .push(item.get_definition_var_type());
    }
    for (index, values) in defs {
        if values.len() != 1
            || rejected.contains(&index)
            || total.get(&index) != approved.get(&index)
        {
            continue;
        }
        let Some(var) = exprents
            .iter()
            .filter_map(|e| e.try_as_var())
            .find(|v| v.get_index() == index)
        else {
            continue;
        };
        if param_type(var).is_some() || var.kuna_declared().is_some() {
            continue;
        }
        let declared = var.get_var_type();
        let typed = values[0]
            .try_as_new()
            .and_then(lambdas::instantiated_parameterization)
            .unwrap_or_else(|| values[0].get_inferred_expr_type(None));
        if declared.type_ != CodeType::Object
            || declared.array_dim != 0
            || typed.array_dim != 0
            || typed.value != declared.value
            || !concrete(&typed)
            || formals(declared.value_str()).is_none_or(|f| f.is_empty())
        {
            continue;
        }
        let items = iterated.get(&index).cloned().unwrap_or_default();
        if !items.is_empty() {
            let element = supertype_args(&typed, "java/lang/Iterable");
            let fits = element
                .as_ref()
                .and_then(|a| a.first().cloned().flatten())
                .is_some_and(|e| {
                    items.iter().all(|t| {
                        t.type_ == CodeType::Object
                            && t.array_dim == 0
                            && instance_of(&e.value, &t.value)
                    })
                });
            if !fits {
                continue;
            }
        }
        processor.set_kuna_typed(
            index,
            KunaLocalType {
                type_: typed,
                mutable: false,
                nullable: Vec::new(),
                java: true,
            },
        );
    }
}

/// Whether no argument of `inv` is written against a type variable of its
/// method's class, which the receiver's type arguments would fix: an
/// argument whose parameter type in the method's signature names one is a
/// lambda or method reference, which the receiver's type types.
fn independent_arguments(inv: crate::invocation_exprent::InvocationExprent) -> bool {
    let Some(desc) = inv.find_desc() else {
        return false;
    };
    let Some(sig) = desc.get_signature() else {
        return true;
    };
    let Some(class_formals) = formals(&desc.get_class_qualified_name()) else {
        return false;
    };
    let params = sig.parameter_types();
    let args = inv.get_lst_parameters();
    if params.len() != args.len() {
        return false;
    }
    params.iter().zip(args.iter()).all(|(p, a)| {
        !class_formals.iter().any(|f| mentions(p, f.value_str()))
            || a.try_as_new().is_some_and(|n| n.is_lambda())
    })
}

/// Whether every type argument of `t`, to any depth, is a class type: no
/// wildcard, type variable or unknown argument.
fn concrete(t: &VarType) -> bool {
    let Some(g) = t.try_as_generic() else {
        return false;
    };
    !g.get_arguments().is_empty()
        && g.get_arguments().iter().all(|a| {
            a.as_ref().is_some_and(|a| {
                a.type_ == CodeType::Object
                    && a.array_dim == 0
                    && a.try_as_generic().is_none_or(|ag| {
                        ag.get_wildcard() == GenericType::WILDCARD_NO
                            && (ag.get_arguments().is_empty() || concrete(a))
                    })
            })
        })
}

fn collect_for_each(stat: Statement, out: &mut Vec<DoStatement>) {
    if let Some(d) = stat.try_as_do().filter(|d| d.get_looptype() == LoopType::ForEach) {
        out.push(d);
    }
    for st in stat.get_stats().borrow().iter() {
        collect_for_each(*st, out);
    }
}
