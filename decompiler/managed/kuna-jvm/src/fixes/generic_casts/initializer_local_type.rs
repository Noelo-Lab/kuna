//! Fix `initializer_local_type`: in a method compiled without a
//! LocalVariableTable, a local of a generic class with one assignment, whose
//! value's type gives that class known type arguments, is declared with them
//! when its every read is a call on it whose arguments do not depend on them,
//! or the iterable of a for-each whose variable takes the element type. A
//! local whose every value, and a for-each variable whose every element, has
//! the type of a type variable in scope that erases to the local's type is
//! declared with that variable.

use std::collections::{BTreeMap, BTreeSet};

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::fixes::generic_casts::mentions;
use crate::fixes::lambdas;
use crate::generic_type::GenericType;
use crate::kuna_static_types::{formals, param_type, supertype_args};
use crate::root_statement::RootStatement;
use crate::rt::JHashMap;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;
use crate::var_processor::{KunaLocalType, VarProcessor};
use crate::var_type::{get_class, instance_of, VarType};

/// Types the locals of `method` (body `root`, variables `processor`) the
/// module header describes.
pub fn type_locals(root: RootStatement, class: StructClass, method: StructMethod, processor: VarProcessor) {
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
    let scope = scope_type_variables(class, method);
    for (item, element) in for_each_elements(root) {
        if !defs.contains_key(&item.get_index()) {
            type_variable_local(processor, item, &element, &scope);
        }
    }
    for (index, values) in &defs {
        if rejected.contains(index) {
            continue;
        }
        let Some(var) = exprents.iter().filter_map(|e| e.try_as_var()).find(|v| v.get_index() == *index) else {
            continue;
        };
        let mut typed: Option<VarType> = None;
        if values.iter().all(|v| value_type(*v, *index, &mut typed, &scope)) {
            if let Some(typed) = typed {
                type_variable_local(processor, var, &typed, &scope);
            }
        }
    }
    for (index, values) in defs {
        if processor.kuna_typed(index).is_some() {
            continue;
        }
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

/// Declares `var` with `typed`, the type of every value it holds, when that
/// is a type variable in scope (or an array of one) whose erasure is the
/// variable's type: the variable then accepts every read the erased type
/// did, a bound's members included, and also the reads that require the
/// variable itself (`map.put(k, item)` for a `Map<String, N>`).
fn type_variable_local(
    processor: VarProcessor,
    var: VarExprent,
    typed: &VarType,
    scope: &BTreeMap<String, VarType>,
) {
    if typed.type_ != CodeType::Genvar
        || typed.try_as_generic().is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
        || param_type(var).is_some()
        || var.kuna_declared().is_some()
    {
        return;
    }
    let Some(erasure) = scope.get(typed.value_str()) else {
        return;
    };
    let declared = var.get_var_type();
    if declared.type_ != CodeType::Object
        || declared.array_dim != typed.array_dim
        || declared.value != erasure.value
    {
        return;
    }
    processor.set_kuna_typed(
        var.get_index(),
        KunaLocalType {
            type_: typed.clone(),
            mutable: false,
            nullable: Vec::new(),
            java: true,
        },
    );
}

/// The type variables `method` can name by a name that means one variable in
/// its body, its own and its class's (an instance method's only), each with
/// its erasure: its first bound's, through bounds that are type variables in
/// scope, or `Object`. A name both the method and the class declare is left
/// out, and so is, in a lambda's body, a name any method of the class
/// declares: the values typed by the class's variable are not of the type
/// the name denotes there.
fn scope_type_variables(class: StructClass, method: StructMethod) -> BTreeMap<String, VarType> {
    let mut bounds: BTreeMap<String, Option<VarType>> = BTreeMap::new();
    let mut hidden: BTreeSet<String> = BTreeSet::new();
    if !method.has_modifier(ACC_STATIC) {
        if let Some(sig) = class.get_signature() {
            for (name, b) in sig.fparameters().iter().zip(sig.fbounds().iter()) {
                bounds.insert(name.clone(), b.first().cloned());
            }
        }
        if method.is_synthetic() {
            for other in class.get_methods().to_vec() {
                if let Some(sig) = other.get_signature() {
                    hidden.extend(sig.type_parameters().iter().cloned());
                }
            }
        }
    }
    if let Some(sig) = method.get_signature() {
        for (name, b) in sig.type_parameters().iter().zip(sig.type_parameter_bounds().iter()) {
            if bounds.insert(name.clone(), b.first().cloned()).is_some() {
                hidden.insert(name.clone());
            }
        }
    }
    let mut out = BTreeMap::new();
    for name in bounds.keys().filter(|n| !hidden.contains(*n)) {
        let mut current = name.clone();
        for _ in 0..=bounds.len() {
            match bounds.get(&current).cloned().flatten() {
                None => {
                    out.insert(name.clone(), VarType::VARTYPE_OBJECT());
                    break;
                }
                Some(b) if b.type_ == CodeType::Genvar && b.array_dim == 0 => current = b.value_str().to_string(),
                Some(b) if b.type_ == CodeType::Object && b.array_dim == 0 => {
                    out.insert(name.clone(), VarType::new_value(CodeType::Object, 0, b.value.clone()));
                    break;
                }
                Some(_) => break,
            }
        }
    }
    out
}

/// Whether `value`, stored into the variable `index`, has the one type
/// `typed` gathers from the values so far: `null`, the variable itself and
/// each arm of a conditional add none, and javac's cast of a value of a type
/// variable to its erasure (`scope`'s) is a value of that variable.
fn value_type(value: Exprent, index: i32, typed: &mut Option<VarType>, scope: &BTreeMap<String, VarType>) -> bool {
    if value.try_as_const().is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
        || value.try_as_var().is_some_and(|v| v.get_index() == index)
    {
        return true;
    }
    if let Some(f) = value.try_as_function().filter(|f| f.get_func_type() == FunctionType::Ternary) {
        let ops = f.get_lst_operands();
        return value_type(ops.get(1), index, typed, scope) && value_type(ops.get(2), index, typed, scope);
    }
    let unchecked = value
        .try_as_function()
        .filter(|f| f.get_func_type() == FunctionType::Cast)
        .and_then(|f| {
            let ops = f.get_lst_operands();
            let target = ops.get(1).get_expr_type();
            let t = generic_type(ops.get(0))?;
            let erasure = scope.get(t.value_str())?;
            (t.type_ == CodeType::Genvar && target.array_dim == t.array_dim && target.value == erasure.value).then_some(t)
        });
    let Some(t) = unchecked.or_else(|| generic_type(value)) else {
        return false;
    };
    match typed {
        Some(prior) => prior.equals(&t) && prior.value == t.value,
        None => {
            *typed = Some(t);
            true
        }
    }
}

/// The type of `e` as its declarations give it, `None` where they do not
/// fix one: a parameter's from the method's signature, `this` as its class
/// with its own type variables, an element of such an array, a field's
/// (`field_type`), and the result of a call on one of these whose result
/// type is a type variable of the method's class, which the receiver's type
/// arguments give, when no argument is of a raw type where the parameter has
/// type arguments (an unchecked conversion erases the result, JLS
/// 15.12.2.6).
fn generic_type(e: Exprent) -> Option<VarType> {
    if let Some(var) = e.try_as_var() {
        if let Some(Some(t)) = param_type(var) {
            return Some(t);
        }
        if let Some(t) = var.kuna_typed() {
            return Some(t);
        }
        let class = var.get_processor()?.get_this_vars().borrow().get_cloned(&var.get_var_version_pair())?;
        return Some(get_class(&Some(class.as_str().into()))?.get_signature()?.generic_type().0.clone());
    }
    if let Some(array) = e.try_as_array() {
        let t = generic_type(array.get_array())?;
        return (t.array_dim > 0).then(|| t.decrease_array_dim());
    }
    if let Some(field) = e.try_as_field() {
        return field_type(field);
    }
    let inv = e.try_as_invocation()?;
    let m = inv.find_desc()?;
    let sig = m.get_signature()?;
    let ret = sig.return_type()?;
    if ret.type_ != CodeType::Genvar || ret.array_dim != 0 || sig.type_parameters().iter().any(|p| p == ret.value_str()) {
        return None;
    }
    let formals_ = sig.parameter_types().clone();
    let args = inv.get_lst_parameters().to_vec();
    if formals_.len() != args.len() {
        return None;
    }
    for (f, a) in formals_.iter().zip(&args) {
        let parameterized = f.try_as_generic().is_some_and(|g| !g.get_arguments().is_empty());
        if parameterized && generic_type(*a).is_none_or(|t| t.try_as_generic().is_none_or(|g| g.get_arguments().is_empty())) {
            return None;
        }
    }
    let owner = m.get_class_qualified_name();
    let class_formals = formals(&owner)?;
    let k = class_formals.iter().position(|f| f.value == ret.value)?;
    let receiver = generic_type(inv.get_instance()?)?;
    let args = supertype_args(&receiver, &owner)?;
    args.get(k).cloned().flatten()
}

/// The type of the field `field` reads: its signature when that names no
/// type variable, else with the type variables of the class declaring it
/// replaced by the arguments the type of its instance gives that class.
/// `None` when the instance's type is raw or not declared (the field's type
/// is then its erasure, JLS 4.8) or the signature names a type variable of
/// another declaration (an enclosing class's).
fn field_type(field: FieldExprent) -> Option<VarType> {
    let context = DecompilerContext::get_struct_context();
    let name = field.get_name();
    let descriptor = field.get_descriptor().descriptor_string.clone();
    let mut cls = context.get_class(&field.get_classname());
    let (owner, declared) = loop {
        let c = cls?;
        if let Some(f) = c.get_field(&name, &descriptor) {
            break (c.qualified_name(), f);
        }
        cls = c.super_class().and_then(|s| s.get_string_opt()).and_then(|s| context.get_class(&s));
    };
    let Some(sig) = declared.get_signature() else {
        return Some(field.get_descriptor().type_.clone());
    };
    let t = sig.type_();
    if names_only(&t, &[]) {
        return Some(t);
    }
    let owner_formals = formals(&owner)?;
    if !names_only(&t, &owner_formals) {
        return None;
    }
    let receiver = generic_type(field.get_instance()?)?;
    let args = supertype_args(&receiver, &owner)?;
    if args.len() != owner_formals.len() {
        return None;
    }
    let mut subst = JHashMap::new();
    for (formal, arg) in owner_formals.iter().zip(args) {
        subst.put(formal.clone(), arg?);
    }
    t.remap(&subst)
}

/// Whether every type variable `t` names is one of `vars`.
fn names_only(t: &VarType, vars: &[VarType]) -> bool {
    if t.type_ == CodeType::Genvar {
        return vars.iter().any(|v| v.value == t.value);
    }
    t.try_as_generic().is_none_or(|g| {
        g.get_arguments().iter().flatten().all(|a| names_only(a, vars))
            && g.get_parent().is_none_or(|p| names_only(&p, vars))
    })
}

/// The variable and element type of each for-each loop of `root` whose
/// iterable's type gives one: an array's component or the argument it gives
/// `Iterable`.
fn for_each_elements(root: RootStatement) -> Vec<(VarExprent, VarType)> {
    let mut loops: Vec<DoStatement> = Vec::new();
    collect_for_each(root.0, &mut loops);
    let mut out = Vec::new();
    for l in loops {
        let (Some(iterable), Some(item)) = (
            l.get_inc_exprent(),
            l.get_init_exprent().and_then(|e| e.try_as_var()),
        ) else {
            continue;
        };
        let Some(t) = generic_type(iterable) else {
            continue;
        };
        let element = if t.array_dim > 0 {
            Some(t.decrease_array_dim())
        } else if t.type_ == CodeType::Object {
            supertype_args(&t, "java/lang/Iterable").and_then(|a| a.first().cloned().flatten())
        } else {
            None
        };
        if let Some(element) = element {
            out.push((item, element));
        }
    }
    out
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
