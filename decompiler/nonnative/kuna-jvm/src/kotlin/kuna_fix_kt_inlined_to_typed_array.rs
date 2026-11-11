//! kuna fix `kt_inlined_to_typed_array`: kotlinc's inlined `toTypedArray()`
//! is written `toTypedArray()`.
//!
//! `Collection<T>.toTypedArray()` is an inline function whose body calls
//! `java.util.Collection.toArray(arrayOfNulls<T>(0))` and casts the result to
//! `Array<T>`; Kotlin 1.3's copy also checks the result for null and throws
//! `TypeCastException("null cannot be cast to non-null type
//! kotlin.Array<T>")`. Vineflower writes the inlined body: `toArray`, which
//! a Kotlin collection does not have ("unresolved reference"), into a
//! stack variable typed `Array<Any>`, the `if (v == null) throw ..` around
//! the code after it, and the cast at each use. Before the last
//! stack-variable pass, each `invokeinterface Collection.toArray` of a new
//! zero-length array of `T` is made a call of `toTypedArray()` typed
//! `Array<T>`, and Kotlin 1.3's null check of the variable it was just stored
//! into is removed (an `else` becomes the code after it); after the pass, a
//! cast to `Array<T>` of that call, or of a variable that holds it, is left
//! out and the variable is typed `Array<T>`, and the call is written with
//! its type argument, `toTypedArray<T>()`, unless the receiver's type names
//! `T` for its elements (or `T` is a generic class, which has no raw name):
//! the reified `T` is the class of the array the call creates. kotlinc
//! compiles `toTypedArray()` to the same call and cast again.

use std::collections::BTreeSet;

use crate::const_exprent::ConstValue;
use crate::exprent::{Exprent, ExprentType};
use crate::exit_exprent::Type as ExitType;
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationType;
use crate::kotlin::kuna_fix_kt_raw_generic_type;
use crate::p8_structure::kuna_javac_layout;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::statement::{Statement, StatementType};
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

const COLLECTION: &str = "java/util/Collection";
const TO_ARRAY_DESCRIPTOR: &str = "([Ljava/lang/Object;)[Ljava/lang/Object;";
const TO_TYPED_ARRAY: &str = "toTypedArray";
const TYPE_CAST_EXCEPTION: &str = "kotlin/TypeCastException";
const ARRAY_CAST_MESSAGE: &str = "null cannot be cast to non-null type kotlin.Array<T>";

/// Before the last stack-variable pass: writes the `toArray` calls under
/// `root` as `toTypedArray()` and removes Kotlin 1.3's null checks of them.
pub fn resugar(root: RootStatement) -> bool {
    let mut res = false;
    rename(root.0, &mut res);
    remove_checks(root.0, &mut res);
    res
}

/// After the last stack-variable pass: leaves out the casts of the
/// `toTypedArray()` calls under `root` to their own type and types the
/// variables that hold them.
pub fn retype(root: RootStatement, processor: VarProcessor) {
    let mut holders: Vec<(VarVersionPair, VarType)> = Vec::new();
    collect_holders(root.0, &mut holders);
    loop {
        let before = holders.len();
        copy_holders(root.0, &mut holders);
        if holders.len() == before {
            break;
        }
    }
    for (pair, typed) in &holders {
        processor.set_var_type(*pair, typed);
    }
    let pairs: BTreeSet<VarVersionPair> = holders.iter().map(|(p, _)| *p).collect();
    for_each_exprent(root.0, &mut |e| strip_casts(e, &pairs));
}

/// Once the Kotlin plugin has replaced the exprents: gives the
/// `toTypedArray()` calls under `root` their type arguments.
pub fn type_arguments(root: RootStatement) {
    for_each_exprent(root.0, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(typed) = to_typed_array(e) {
                type_argument(e, typed);
            }
        }
    });
}

/// Writes the call's type argument, `toTypedArray<T>()`, unless the
/// receiver's type has `T` for its element type, which Kotlin infers, or
/// `T` is a generic class, which Kotlin cannot name raw.
fn type_argument(call: Exprent, typed: VarType) {
    let inv = call.as_invocation();
    let element = typed.decrease_array_dim();
    let inferred = inv.get_instance().and_then(element_type).is_some_and(|a| {
        a.type_ == element.type_ && a.array_dim == element.array_dim && a.value == element.value
    });
    if inferred || kuna_fix_kt_raw_generic_type::arity(&element).is_some() {
        return;
    }
    inv.d_mut(|d| d.kuna_chain_type_args = Some(vec![element]));
}

fn for_each_exprent(stat: Statement, f: &mut dyn FnMut(Exprent)) {
    let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
    lists.extend(stat.get_stat_exprents());
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    for e in lists {
        f(e);
    }
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        for_each_exprent(child, f);
    }
}

fn rename(stat: Statement, res: &mut bool) {
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(typed) = to_array_of_new(e) {
                let inv = e.as_invocation();
                let descriptor = format!("(){typed}");
                inv.set_name(TO_TYPED_ARRAY.to_string());
                inv.set_string_descriptor(descriptor.clone());
                inv.set_descriptor(MethodDescriptor::parse_descriptor(&descriptor));
                inv.set_lst_parameters(Vec::new());
                *res = true;
            }
        }
    });
}

/// The array type `T[]` of `Collection.toArray(new T[0])`.
fn to_array_of_new(e: Exprent) -> Option<VarType> {
    let inv = e.try_as_invocation()?;
    if inv.is_static()
        || inv.get_invocation_type() != InvocationType::Interface
        || inv.get_classname() != COLLECTION
        || inv.get_name() != "toArray"
        || inv.get_string_descriptor() != TO_ARRAY_DESCRIPTOR
        || inv.get_instance().is_none()
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    let [arg] = params.as_slice() else {
        return None;
    };
    let new = arg.try_as_new()?;
    let typed = new.get_new_type();
    let dims = new.get_lst_dims().to_vec();
    let zero = dims.len() == 1
        && dims[0]
            .try_as_const()
            .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::Int(0))));
    (typed.array_dim == 1 && zero && new.get_lst_array_elements().is_empty()).then_some(typed)
}

/// The array type of a `toTypedArray()` call this fix wrote.
fn to_typed_array(e: Exprent) -> Option<VarType> {
    let inv = e.try_as_invocation()?;
    if inv.get_classname() != COLLECTION || inv.get_name() != TO_TYPED_ARRAY {
        return None;
    }
    let descriptor = inv.get_string_descriptor();
    Some(VarType::new_signature(descriptor.strip_prefix("()")?))
}

fn remove_checks(stat: Statement, res: &mut bool) {
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        remove_checks(child, res);
    }
    let Some(ifstat) = stat.try_as_if() else {
        return;
    };
    let Some(first) = ifstat.get_first() else {
        return;
    };
    if !is_array_check(ifstat, first) {
        return;
    }
    if ifstat.iftype() == IfStatement::IFTYPE_IFELSE {
        if ifstat.get_elsestat().is_none() || ifstat.get_else_edge().is_none() {
            return;
        }
        kuna_javac_layout::split_else(ifstat);
    }
    let body = ifstat.get_ifstat().expect("check body");
    for edge in body.get_all_successor_edges() {
        edge.remove();
    }
    for edge in first.get_all_successor_edges() {
        edge.remove();
    }
    first.get_var_definitions().add_all(ifstat.get_var_definitions().to_vec());
    ifstat.get_parent().expect("parent").replace_statement(ifstat.0, first);
    *res = true;
}

/// Whether `stat` is `if (v == null) { throw TypeCastException("null cannot be
/// cast to non-null type kotlin.Array<T>") }` right after `v = c.toTypedArray()`
/// ends its head block `first`.
fn is_array_check(stat: IfStatement, first: Statement) -> bool {
    if first.type_() != StatementType::BasicBlock || stat.is_pattern_matched() {
        return false;
    }
    let Some(body) = stat.get_ifstat() else {
        return false;
    };
    let throws = body.type_() == StatementType::BasicBlock
        && body.get_exprents().is_some_and(|list| {
            list.len() == 1
                && list.get(0).try_as_exit().is_some_and(|exit| {
                    exit.get_exit_type() == ExitType::Throw
                        && exit.get_value().and_then(|v| v.try_as_new()).is_some_and(|new| {
                            new.get_new_type().value.as_deref() == Some(TYPE_CAST_EXCEPTION)
                                && new.get_constructor().is_some_and(|ctor| {
                                    let args = ctor.get_lst_parameters().to_vec();
                                    args.len() == 1
                                        && args[0].try_as_const().is_some_and(|c| {
                                            matches!(c.get_value(), Some(ConstValue::String(m)) if m == ARRAY_CAST_MESSAGE)
                                        })
                                })
                        })
                })
        });
    if !throws {
        return false;
    }
    let Some(func) = stat.get_headexprent().get_condition().try_as_function() else {
        return false;
    };
    if func.get_func_type() != FunctionType::Eq {
        return false;
    }
    let ops = func.get_lst_operands().to_vec();
    let checked = match (ops[0].try_as_const().is_some_and(|c| c.is_null()), ops[1].try_as_const().is_some_and(|c| c.is_null())) {
        (false, true) => ops[0],
        (true, false) => ops[1],
        _ => return false,
    };
    let Some(var) = checked.try_as_var() else {
        return false;
    };
    let Some(exprents) = first.get_exprents().filter(|l| !l.is_empty()) else {
        return false;
    };
    exprents.get(exprents.len() - 1).try_as_assignment().is_some_and(|a| {
        a.get_cond_type().is_none()
            && to_typed_array(a.get_right()).is_some()
            && a.get_left().try_as_var().is_some_and(|left| {
                left.get_index() == var.get_index() && left.get_version() == var.get_version()
            })
    })
}

fn collect_holders(stat: Statement, holders: &mut Vec<(VarVersionPair, VarType)>) {
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            let Some(a) = e.try_as_assignment().filter(|a| a.get_cond_type().is_none()) else {
                continue;
            };
            let (Some(left), Some(typed)) = (a.get_left().try_as_var(), to_typed_array(a.get_right())) else {
                continue;
            };
            let pair = VarVersionPair::from_var(left);
            if !holders.iter().any(|(p, _)| *p == pair) {
                holders.push((pair, typed));
            }
        }
    });
}

/// Adds the variables a holder is copied into (`w = v`) as holders.
fn copy_holders(stat: Statement, holders: &mut Vec<(VarVersionPair, VarType)>) {
    let mut found: Vec<(VarVersionPair, VarType)> = Vec::new();
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            let Some(a) = e.try_as_assignment().filter(|a| a.get_cond_type().is_none()) else {
                continue;
            };
            let (Some(left), Some(right)) = (a.get_left().try_as_var(), a.get_right().try_as_var()) else {
                continue;
            };
            let from = VarVersionPair::from_var(right);
            let to = VarVersionPair::from_var(left);
            if let Some((_, typed)) = holders.iter().find(|(p, _)| *p == from) {
                if !holders.iter().chain(found.iter()).any(|(p, _)| *p == to) {
                    found.push((to, typed.clone()));
                }
            }
        }
    });
    holders.extend(found);
}

/// The element type of the collection `e`, by its generic type: a local's
/// as the Kotlin plugin typed it, a cast's operand's, else the inferred one.
fn element_type(e: Exprent) -> Option<VarType> {
    let typed = match e.get_type() {
        ExprentType::Var => e
            .as_var()
            .kuna_declared()
            .map(|k| k.type_)
            .unwrap_or_else(|| e.get_inferred_expr_type(None)),
        ExprentType::Function if e.as_function().get_func_type() == FunctionType::Cast => {
            return element_type(e.as_function().get_lst_operands().get(0));
        }
        _ => e.get_inferred_expr_type(None),
    };
    typed.try_as_generic()?.get_arguments().first().cloned().flatten()
}

/// Leaves out, under `e`, each cast to `T[]` of a `toTypedArray()` call or a
/// holder of type `T[]`.
fn strip_casts(e: Exprent, holders: &BTreeSet<VarVersionPair>) {
    for child in e.get_all_exprents() {
        strip_casts(child, holders);
        let Some(func) = child.try_as_function().filter(|f| f.get_func_type() == FunctionType::Cast) else {
            continue;
        };
        let ops = func.get_lst_operands().to_vec();
        let target = ops[1].get_expr_type();
        let operand = ops[0];
        let typed = match operand.get_type() {
            ExprentType::Invocation => to_typed_array(operand),
            ExprentType::Var if holders.contains(&VarVersionPair::from_var(operand.as_var())) => {
                Some(operand.get_expr_type())
            }
            _ => None,
        };
        if typed.is_some_and(|t| t.equals(&target)) {
            e.replace_exprent(child, operand);
        }
    }
}
