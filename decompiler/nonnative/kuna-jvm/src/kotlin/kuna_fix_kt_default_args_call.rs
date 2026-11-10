//! kuna fix `kt_default_args_call` (`kuna_fixes`): a call that leaves
//! arguments to their defaults is written as the call the source makes.
//!
//! kotlinc compiles `foo(a)` of `fun foo(a: Int, b: Int = 1)` to a call of
//! the synthetic `foo$default(a, 0, mask, null)`, which takes a placeholder
//! for every defaulted parameter, one `int` mask bit per value parameter
//! (set: use the default) and a marker; a member's bridge is static and takes
//! the receiver first (an interface's lives in its `DefaultImpls`), and a
//! constructor's is `<init>(.., mask, DefaultConstructorMarker)`. Vineflower
//! prints these calls as they are. A function call is resugared when its
//! bridge is synthetic and static, its marker `null`, its masks constants
//! naming value parameters only, each defaulted parameter declares a default
//! in the Kotlin metadata of the one function of that name and descriptor
//! the bridge stands for and is passed a constant. Defaulted arguments are
//! left out; passed ones after a defaulted one are written by name. When
//! another function of that name the call could resolve to might accept
//! what is left (by count and by type, or by names), the passed value
//! arguments are written by name, or the call is left as it is. A call in a
//! method of the target's name that has no metadata (an `@JvmOverloads`
//! overload, which delegates to the bridge) is left as it is: written as the
//! call of the function, it would read as the overload calling itself.
//! Constructor calls (`new`) go through `kt_default_args_super_call`'s
//! resugaring.

use std::cell::Cell;
use std::rc::Rc;

use crate::code_constants::{
    ACC_BRIDGE, ACC_FINAL, ACC_INTERFACE, ACC_STATIC, ACC_SYNTHETIC, ACC_VARARGS,
};
use crate::const_exprent::ConstValue;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kotlin::kuna_fix_kt_default_args_super_call::{self, defaulted_parameters};
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

const SUFFIX: &str = "$default";
const OBJECT: &str = "Ljava/lang/Object;";
const DEFAULT_IMPLS: &str = "$DefaultImpls";

thread_local! {
    static ENCLOSING: Cell<Option<StructMethod>> = const { Cell::new(None) };
}

/// Runs `f` with `method` as the method whose calls are resugared.
pub fn within<R>(method: StructMethod, f: impl FnOnce() -> R) -> R {
    struct Restore(Option<StructMethod>);
    impl Drop for Restore {
        fn drop(&mut self) {
            ENCLOSING.with(|e| e.set(self.0));
        }
    }
    let _restore = Restore(ENCLOSING.with(|e| e.replace(Some(method))));
    f()
}

/// Whether the call is in a method of the target's name without metadata, of
/// a class of the target's scope.
fn in_overload(target: &Target) -> bool {
    let Some(method) = ENCLOSING.with(Cell::get) else {
        return false;
    };
    method.get_name() == target.method.get_name()
        && KtAttrs::get(method, &KElement::KEY()).is_none()
        && target
            .scope
            .iter()
            .any(|c| c.qualified_name() == method.get_class_qualified_name())
}

/// The function a bridge stands for.
struct Target {
    owner: String,
    method: StructMethod,
    function: Rc<KFunction>,
    member: bool,
    interface: bool,
    /// The classes whose functions of the name a call could also resolve to.
    scope: Vec<StructClass>,
}

/// Rewrites `invocation` when it calls a function's default-arguments bridge
/// that resugars (see the module header); returns whether it did.
pub fn resugar_call(invocation: InvocationExprent) -> bool {
    if !invocation.is_static()
        || invocation.get_functype() != Type::General
        || invocation.get_invocation_type() != InvocationType::Static
    {
        return false;
    }
    let name = invocation.get_name();
    let Some(base) = name.strip_suffix(SUFFIX).filter(|b| !b.is_empty()) else {
        return false;
    };
    let descriptor = invocation.get_descriptor();
    let params: Vec<String> = descriptor.params.iter().map(VarType::to_string).collect();
    let args = invocation.get_lst_parameters().to_vec();
    if args.len() != params.len()
        || params.last().map(String::as_str) != Some(OBJECT)
        || !args.last().is_some_and(|a| is_null(*a))
    {
        return false;
    }
    let Some(target) = find_target(
        &invocation.get_classname(),
        &name,
        base,
        &invocation.get_string_descriptor(),
        &params,
        &descriptor.ret,
    ) else {
        return false;
    };
    if in_overload(&target) {
        return false;
    }
    let function = &target.function;
    let start = usize::from(target.member);
    let real: Vec<String> = target
        .method
        .method_descriptor()
        .params
        .iter()
        .map(VarType::to_string)
        .collect();
    let k = real.len();
    let n = function.parameters.len();
    let first_value = usize::from(function.receiver_type.is_some());
    let mask_count = n / 32 + 1;
    let Some(masks) = args[start + k..start + k + mask_count]
        .iter()
        .map(|a| int_constant(*a))
        .collect::<Option<Vec<i32>>>()
    else {
        return false;
    };
    let Some(defaulted) = defaulted_parameters(&masks, n) else {
        return false;
    };
    let real_args = &args[start..start + k];
    for (i, param) in function.parameters.iter().enumerate() {
        if defaulted[i]
            && (!Flags::DECLARES_DEFAULT_VALUE.get(param.flags)
                || !real_args[first_value + i].is_const())
        {
            return false;
        }
    }
    let Some(names) = argument_names(&target, &real, &defaulted, first_value) else {
        return false;
    };

    if target.member {
        invocation.set_static(false);
        invocation.set_instance(Some(args[0]));
        let invocation_type = if target.interface {
            InvocationType::Interface
        } else {
            InvocationType::Virtual
        };
        invocation.d_mut(|d| d.invocation_type = invocation_type);
        invocation.set_classname(target.owner.clone());
    }
    let real_descriptor = target.method.get_descriptor();
    invocation.set_name(target.method.get_name());
    invocation.set_descriptor(MethodDescriptor::parse_descriptor(&real_descriptor));
    invocation.set_string_descriptor(real_descriptor);
    invocation.set_lst_parameters(real_args.to_vec());
    invocation.set_kuna_arg_names(names.into());
    true
}

/// Resugars the constructor call of `new` when it goes through a class's
/// default-arguments constructor (`kt_default_args_super_call`).
pub fn resugar_new(new: NewExprent) {
    if new.is_anonymous() || new.is_lambda() || new.is_enum_const() {
        return;
    }
    if let Some(constructor) = new.get_constructor() {
        kuna_fix_kt_default_args_super_call::resugar(constructor);
    }
}

/// How each argument of the resugared call is written: `Some("")`
/// positionally, `Some(name)` by name, `None` not at all. Positional when
/// the defaults are trailing and no other candidate accepts the arguments
/// left; else every passed value argument by name, when the names select
/// the target alone and none of them is `vararg`.
fn argument_names(
    target: &Target,
    real: &[String],
    defaulted: &[bool],
    first_value: usize,
) -> Option<Vec<Option<String>>> {
    let params = &target.function.parameters;
    let n = params.len();
    let first_gap = defaulted.iter().position(|&d| d).unwrap_or(n);
    let trailing = defaulted[first_gap..].iter().all(|&d| d);
    let mut names: Vec<Option<String>> = vec![Some(String::new()); real.len()];
    for (i, &d) in defaulted.iter().enumerate() {
        if d {
            names[first_value + i] = None;
        }
    }
    if trailing
        && !others(target, |other, f| {
            positional_fit(real, first_value + first_gap, other, f)
        })
    {
        return Some(names);
    }
    let mut passed: Vec<(usize, &str)> = Vec::new();
    for (i, param) in params.iter().enumerate() {
        if defaulted[i] {
            continue;
        }
        if param.vararg_type.is_some() {
            return None;
        }
        passed.push((first_value + i, param.name.as_str()));
        names[first_value + i] = Some(KotlinWriter::to_valid_kotlin_identifier(&param.name));
    }
    if others(target, |other, f| {
        named_fit(real, first_value, &passed, other, f)
    }) {
        return None;
    }
    Some(names)
}

/// Whether a function other than the target, of its Kotlin name, visible
/// to Kotlin (in the metadata, or of a class without) and in the target's
/// scope, satisfies `fits` (given its function metadata, if any).
fn others(target: &Target, fits: impl Fn(StructMethod, Option<Rc<KFunction>>) -> bool) -> bool {
    let name = &target.function.name;
    let descriptor = target.method.get_descriptor();
    target.scope.iter().any(|&cl| {
        let kotlin = KtAttrs::get(cl, &KElement::KEY()).is_some();
        cl.get_methods().to_vec().into_iter().any(|mt| {
            if mt.get_descriptor() == descriptor && mt.get_name() == target.method.get_name()
                || mt.has_modifier(ACC_STATIC) == target.member
                || mt.has_modifier(ACC_SYNTHETIC)
                || mt.has_modifier(ACC_BRIDGE)
            {
                return false;
            }
            match KtAttrs::get(mt, &KElement::KEY()) {
                Some(KElement::Function(function)) => {
                    &function.name == name && fits(mt, Some(function))
                }
                _ => !kotlin && &mt.get_name() == name && fits(mt, None),
            }
        })
    })
}

/// Whether `other` might be chosen over the target for the first `kept`
/// arguments of a call of the target (JVM parameters `real`).
fn positional_fit(
    real: &[String],
    kept: usize,
    other: StructMethod,
    function: Option<Rc<KFunction>>,
) -> bool {
    let params = param_strings(other);
    let varargs = other.has_modifier(ACC_VARARGS);
    let rest_defaulted = match &function {
        None => true,
        Some(f) => {
            let first = params.len().saturating_sub(f.parameters.len());
            f.parameters.iter().enumerate().all(|(i, p)| {
                first + i < kept
                    || Flags::DECLARES_DEFAULT_VALUE.get(p.flags)
                    || p.vararg_type.is_some()
            })
        }
    };
    let count_fits = params.len() == kept || varargs || (params.len() > kept && rest_defaulted);
    if !count_fits {
        return false;
    }
    if varargs {
        let fixed = params.len().saturating_sub(1);
        return (0..kept.min(fixed)).all(|i| !excludes(&real[i], &params[i]));
    }
    let pairs: Vec<(&str, &str)> = (0..kept)
        .map(|i| (real[i].as_str(), params[i].as_str()))
        .collect();
    competes(&pairs)
}

/// Whether `other` might be chosen over the target for the call with the
/// leading `positional` arguments and the value arguments `named` (position,
/// name).
fn named_fit(
    real: &[String],
    positional: usize,
    named: &[(usize, &str)],
    other: StructMethod,
    function: Option<Rc<KFunction>>,
) -> bool {
    let params = param_strings(other);
    let Some(function) = function else {
        return true;
    };
    let first = usize::from(function.receiver_type.is_some());
    if first != positional || params.len() < first + function.parameters.len() {
        return true;
    }
    let mut pairs: Vec<(&str, &str)> = (0..positional)
        .map(|i| (real[i].as_str(), params[i].as_str()))
        .collect();
    for &(pos, name) in named {
        match function.parameters.iter().position(|p| p.name == name) {
            None => return false,
            Some(j) => pairs.push((real[pos].as_str(), params[first + j].as_str())),
        }
    }
    let accepts = function.parameters.iter().all(|p| {
        Flags::DECLARES_DEFAULT_VALUE.get(p.flags)
            || p.vararg_type.is_some()
            || named.iter().any(|&(_, n)| n == p.name)
    });
    accepts && competes(&pairs)
}

/// Whether a candidate whose parameters for the call's arguments have the
/// JVM types `pairs` (the target's, the candidate's) competes with the
/// target: it may accept the arguments and the target is not strictly more
/// specific.
fn competes(pairs: &[(&str, &str)]) -> bool {
    if pairs.iter().any(|(t, o)| excludes(t, o)) {
        return false;
    }
    let more_specific =
        pairs.iter().all(|(t, o)| is_subtype(t, o)) && pairs.iter().any(|(t, o)| t != o);
    !more_specific
}

/// Whether the JVM type `t` is known to be a subtype of `o` as Kotlin types
/// (arrays are invariant).
fn is_subtype(t: &str, o: &str) -> bool {
    if t == o || o == OBJECT {
        return true;
    }
    if t.len() == 1 {
        return boxed_supertypes(t).contains(&o);
    }
    if o.len() == 1 {
        return false;
    }
    if t.starts_with('[') {
        return ARRAY_SUPERTYPES.contains(&o);
    }
    let (Some(tn), Some(on)) = (class_name(t), class_name(o)) else {
        return false;
    };
    let context = DecompilerContext::get_struct_context();
    context.get_class(tn).is_some() && context.instance_of(tn, on)
}

/// Whether no argument whose static type is a subtype of the JVM type `t`
/// can be passed for a parameter of the JVM type `o`.
pub fn excludes(t: &str, o: &str) -> bool {
    if t == o {
        return false;
    }
    let primitive = |s: &str| s.len() == 1;
    match (primitive(t), primitive(o)) {
        (true, true) => true,
        (true, false) => !boxed_supertypes(t).contains(&o),
        (false, true) => !boxed_supertypes(o).contains(&t),
        (false, false) => match (t.strip_prefix('['), o.strip_prefix('[')) {
            (Some(te), Some(oe)) => {
                if primitive(te) || primitive(oe) {
                    te != oe
                } else {
                    excludes(te, oe)
                }
            }
            (Some(_), None) => !ARRAY_SUPERTYPES.contains(&o),
            (None, Some(_)) => !ARRAY_SUPERTYPES.contains(&t),
            (None, None) => unrelated_classes(t, o),
        },
    }
}

const ARRAY_SUPERTYPES: [&str; 3] = [
    "Ljava/lang/Object;",
    "Ljava/lang/Cloneable;",
    "Ljava/io/Serializable;",
];

/// The JVM types a value of the primitive type `p` can be passed as.
fn boxed_supertypes(p: &str) -> Vec<&'static str> {
    let boxed = match p {
        "Z" => "Ljava/lang/Boolean;",
        "C" => "Ljava/lang/Character;",
        "B" => "Ljava/lang/Byte;",
        "S" => "Ljava/lang/Short;",
        "I" => "Ljava/lang/Integer;",
        "J" => "Ljava/lang/Long;",
        "F" => "Ljava/lang/Float;",
        "D" => "Ljava/lang/Double;",
        _ => return vec![OBJECT],
    };
    vec![
        boxed,
        OBJECT,
        "Ljava/lang/Number;",
        "Ljava/lang/Comparable;",
        "Ljava/io/Serializable;",
        "Ljava/lang/constant/Constable;",
        "Ljava/lang/constant/ConstantDesc;",
    ]
}

/// Whether no class is a subtype of both classes: both are known, neither
/// is a subtype of the other, and both are classes or one is a final class.
fn unrelated_classes(t: &str, o: &str) -> bool {
    if t == OBJECT || o == OBJECT {
        return false;
    }
    let (Some(t), Some(o)) = (class_name(t), class_name(o)) else {
        return false;
    };
    let context = DecompilerContext::get_struct_context();
    let (Some(tc), Some(oc)) = (context.get_class(t), context.get_class(o)) else {
        return false;
    };
    if context.instance_of(t, o) || context.instance_of(o, t) {
        return false;
    }
    let interface = |c: StructClass| c.has_modifier(ACC_INTERFACE);
    let final_class = |c: StructClass| !interface(c) && c.has_modifier(ACC_FINAL);
    (!interface(tc) && !interface(oc)) || final_class(tc) || final_class(oc)
}

fn class_name(jvm: &str) -> Option<&str> {
    jvm.strip_prefix('L')?.strip_suffix(';')
}

fn param_strings(method: StructMethod) -> Vec<String> {
    method
        .method_descriptor()
        .params
        .iter()
        .map(VarType::to_string)
        .collect()
}

/// The one function the bridge `bridge` (`bridge_descriptor`, parameters
/// `params`) of `classname` stands for.
fn find_target(
    classname: &str,
    bridge: &str,
    base: &str,
    bridge_descriptor: &str,
    params: &[String],
    ret: &VarType,
) -> Option<Target> {
    let context = DecompilerContext::get_struct_context();
    let cl = context.get_class(classname)?;
    KotlinChooser::parse_metadata_for(cl);
    let mut classes = vec![cl];
    if let Some(KElement::MultifileFacade(facade)) = KtAttrs::get(cl, &KElement::KEY()) {
        classes.extend(facade.files.iter().filter_map(|f| context.get_class(f)));
    }
    let home = classes.iter().copied().find(|c| {
        c.get_method(bridge, bridge_descriptor)
            .is_some_and(|m| m.has_modifier(ACC_STATIC) && m.has_modifier(ACC_SYNTHETIC))
    })?;

    let home_name = home.qualified_name();
    let interface_owner = home_name.strip_suffix(DEFAULT_IMPLS).map(str::to_string);
    let owner_name = interface_owner.clone().unwrap_or_else(|| home_name.clone());
    let owner = if interface_owner.is_some() {
        context.get_class(&owner_name)?
    } else {
        home
    };

    let mut found: Vec<Target> = Vec::new();
    if params.first().map(String::as_str) == Some(&format!("L{owner_name};")) {
        if let Some((method, function)) = matching(owner, base, false, &params[1..], ret) {
            found.push(Target {
                owner: owner_name.clone(),
                method,
                function,
                member: true,
                interface: owner.has_modifier(ACC_INTERFACE),
                scope: supertypes(owner),
            });
        }
    }
    if interface_owner.is_none() {
        for &c in &classes {
            if let Some((method, function)) = matching(c, base, true, params, ret) {
                found.push(Target {
                    owner: classname.to_string(),
                    method,
                    function,
                    member: false,
                    interface: false,
                    scope: classes.clone(),
                });
            }
        }
    }
    if found.len() == 1 {
        found.pop()
    } else {
        None
    }
}

/// The function of `cl` named `base` whose bridge takes `params` (after the
/// receiver of a member) and returns `ret`, with its metadata.
fn matching(
    cl: StructClass,
    base: &str,
    is_static: bool,
    params: &[String],
    ret: &VarType,
) -> Option<(StructMethod, Rc<KFunction>)> {
    KotlinChooser::parse_metadata_for(cl);
    cl.get_methods().to_vec().into_iter().find_map(|mt| {
        if mt.get_name() != base || mt.has_modifier(ACC_STATIC) != is_static {
            return None;
        }
        let descriptor = mt.method_descriptor();
        let real = param_strings(mt);
        let k = real.len();
        if !descriptor.ret.equals(ret) || params.len() <= k || params[..k] != real[..] {
            return None;
        }
        let Some(KElement::Function(function)) = KtAttrs::get(mt, &KElement::KEY()) else {
            return None;
        };
        let n = function.parameters.len();
        let masks = n / 32 + 1;
        let shape = function.context_receiver_types.is_empty()
            && !Flags::IS_SUSPEND.get(function.flags)
            && k == usize::from(function.receiver_type.is_some()) + n
            && params.len() == k + masks + 1
            && params[k..k + masks].iter().all(|p| p == "I");
        shape.then_some((mt, function))
    })
}

/// `cl`, its superclasses and its superinterfaces, those that are loaded.
fn supertypes(cl: StructClass) -> Vec<StructClass> {
    let context = DecompilerContext::get_struct_context();
    let mut out: Vec<StructClass> = Vec::new();
    let mut work = vec![cl];
    while let Some(c) = work.pop() {
        if out.iter().any(|o| o.qualified_name() == c.qualified_name()) {
            continue;
        }
        out.push(c);
        if let Some(s) = c.super_class() {
            work.extend(context.get_class(&s.get_string()));
        }
        work.extend(
            c.get_interface_names()
                .iter()
                .filter_map(|i| context.get_class(i)),
        );
    }
    out
}

fn is_null(expr: Exprent) -> bool {
    expr.try_as_const()
        .is_some_and(|c| c.get_const_type().equals(&VarType::VARTYPE_NULL()))
}

fn int_constant(expr: Exprent) -> Option<i32> {
    match expr.try_as_const()?.get_value()? {
        ConstValue::Int(v) => Some(v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_and_array_parameters_exclude_each_other() {
        assert!(excludes("I", "J"));
        assert!(excludes("C", "Ljava/lang/String;"));
        assert!(excludes("Ljava/lang/String;", "C"));
        assert!(!excludes("I", "Ljava/lang/Object;"));
        assert!(!excludes("I", "Ljava/lang/Integer;"));
        assert!(!excludes("Ljava/lang/Number;", "J"));
        assert!(excludes("[B", "[C"));
        assert!(excludes("[I", "[Ljava/lang/Object;"));
        assert!(!excludes("[[I", "[Ljava/lang/Object;"));
        assert!(!excludes("[B", "Ljava/lang/Object;"));
        assert!(excludes("[B", "Ljava/lang/String;"));
        assert!(!excludes("[Ljava/lang/String;", "[Ljava/lang/Object;"));
        assert!(!excludes("Ljava/lang/String;", "Ljava/lang/String;"));
    }
}
