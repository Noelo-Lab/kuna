//! Fix family `lambdas`: lambdas and method references: their inference,
//! overloads and recursion. In this file, `serializable_lambda` writes the
//! interfaces javac's `altMetafactory` adds to a lambda as an intersection
//! cast (`(Comparator & Serializable) (a, b) -> 1`), and `receiver_cast`
//! casts a lambda or method reference that is the receiver of a call.

pub(crate) mod explicit_lambda_parameters;
pub(crate) mod lambda_argument_overload;
pub(crate) mod lambda_pertinence;
pub(crate) mod lambda_return_type_argument;
pub(crate) mod lambda_type_variable_target;
pub(crate) mod raw_target_cast;
pub(crate) mod self_referencing_lambda;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::kuna_static_types::is_bare_var;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::pooled_constant::PooledConstant;
use crate::struct_method::StructMethod;
use crate::primitive_constant::ConstValue;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// The interfaces an `altMetafactory` bootstrap with arguments `args` adds to
/// a lambda of the functional interface `iface`: `Serializable` for
/// `FLAG_SERIALIZABLE`, the classes of `FLAG_MARKERS`; those `iface` is known
/// to extend are left out.
pub fn added_interfaces(args: &[Option<PooledConstant>], iface: &str) -> Vec<String> {
    let int_at = |i: usize| match args.get(i).and_then(|a| a.as_ref()).map(|a| a.as_primitive().value.clone()) {
        Some(Some(ConstValue::Integer(v))) => Some(v),
        _ => None,
    };
    let Some(flags) = int_at(3) else {
        return Vec::new();
    };
    let mut added = Vec::new();
    if flags & 1 != 0 {
        added.push("java/io/Serializable".to_string());
    }
    if flags & 2 != 0 {
        let count = int_at(4).unwrap_or(0).max(0) as usize;
        for i in 5..5 + count {
            if let Some(Some(marker)) = args.get(i).map(|a| a.as_ref().map(|a| a.as_primitive().get_string())) {
                added.push(marker.to_string());
            }
        }
    }
    let context = DecompilerContext::get_struct_context();
    added.retain(|m| !context.instance_of(iface, m));
    added
}

/// The intersection cast a lambda needs for the interfaces `added`, naming
/// its inferred parameterized type, or else its functional interface `iface`
/// when that is known not to be generic: none when nothing is added or
/// neither applies (a raw cast would type the lambda's parameters `Object`).
pub fn intersection_cast(added: &[String], iface: &VarType, inferred: Option<&VarType>) -> Option<TextBuffer> {
    if added.is_empty() {
        return None;
    }
    let target = match inferred {
        Some(t) if t.value == iface.value && t.is_generic() => t.clone(),
        _ => {
            let generic = iface
                .value
                .as_deref()
                .and_then(|n| DecompilerContext::get_struct_context().get_class(n))
                .is_none_or(|c| c.get_signature().is_some_and(|sig| !sig.fparameters().is_empty()));
            if generic {
                return None;
            }
            iface.clone()
        }
    };
    let mut buf = TextBuffer::new();
    buf.append("(").append_cast_type_name(&target);
    for marker in added {
        buf.append(" & ")
            .append_cast_type_name(&VarType::new_value(CodeType::Object, 0, marker));
    }
    buf.append(")");
    Some(buf)
}

/// The type of the cast [`receiver_cast`] asks for, for the receiver
/// `instance` of a method of the class `classname`: the parameterized type
/// the lambda was inferred to have, when it is of that class and names only
/// types in scope, since the call's own result is typed by it and loses its
/// cast (`String s = ((Supplier<String>) () -> "x").get()`); else the class.
pub fn receiver_cast_type(classname: &str, instance: Exprent) -> VarType {
    instance
        .try_as_new()
        .and_then(|n| n.d(|d| d.inferred_lambda_type.clone()))
        .filter(|t| t.is_generic() && t.array_dim == 0 && t.value.as_deref() == Some(classname))
        .or_else(|| {
            instance
                .try_as_new()
                .and_then(instantiated_parameterization)
                .filter(|t| t.value.as_deref() == Some(classname))
        })
        .unwrap_or_else(|| VarType::new_value(CodeType::Object, 0, classname))
}

/// Whether `instance`, the receiver of a call, is a lambda or method reference
/// with no cast of its own: Java gives it no type there, so it needs a cast
/// to the class of the invoked method (`((Func) () -> 1).get()`).
pub fn receiver_cast(instance: Exprent) -> bool {
    instance.try_as_new().is_some_and(|n| {
        n.is_lambda()
            && DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get(&n.get_new_type().value_str().to_string())
                .copied()
                .is_some_and(|node| {
                    node.with(|d| {
                        d.lambda_information.as_ref().is_some_and(|l| l.added_interfaces.is_empty())
                    })
                })
    })
}

/// The functional interface of `lambda`, a lambda or method reference whose
/// `invokedynamic` adds no interfaces, its instantiated method type, and the
/// interface method it implements when the interface declares it.
pub fn functional_method(lambda: NewExprent) -> Option<(VarType, String, StructMethod)> {
    if !lambda.is_lambda() {
        return None;
    }
    let node = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&lambda.get_new_type().value_str().to_string())
        .copied()?;
    let (iface, li) = node.with(|d| (d.anonymous_class_type.clone(), d.lambda_information.clone()));
    let (iface, li) = (iface?, li?);
    if !li.added_interfaces.is_empty() {
        return None;
    }
    let cls = DecompilerContext::get_struct_context().get_class(iface.value.as_deref()?)?;
    let sam = cls.get_method_key(&lambda.get_lambda_method_key())?;
    Some((iface, li.method_descriptor, sam))
}

/// The parameterization of the functional interface of `lambda`, a lambda or
/// method reference, that its `invokedynamic` determines: the instantiated
/// method type (the bootstrap's third argument) is the erasure of the
/// function type of the lambda's target (JLS 15.27.3), so each type variable
/// of the interface that is a parameter or the return of the method's
/// signature takes the instantiated type at that position. `None` unless
/// every type variable is determined, and unless a parameter is typed more
/// specifically than `Object`, which the raw interface would give it.
pub fn instantiated_parameterization(lambda: NewExprent) -> Option<VarType> {
    let (iface, instantiated, sam) = functional_method(lambda)?;
    let cls = DecompilerContext::get_struct_context().get_class(iface.value.as_deref()?)?;
    let fparams = cls.get_signature()?.fparameters().clone();
    if fparams.is_empty() {
        return None;
    }
    let sig = sam.get_signature()?;
    let instantiated = MethodDescriptor::parse_descriptor(&instantiated);
    let generic_params = sig.parameter_types().clone();
    if generic_params.len() != instantiated.params.len() {
        return None;
    }
    let mut positions: Vec<(VarType, VarType, bool)> = generic_params
        .into_iter()
        .zip(instantiated.params.iter().cloned())
        .map(|(g, t)| (g, t, true))
        .collect();
    positions.push((sig.return_type()?, instantiated.ret.clone(), false));
    let mut args = Vec::with_capacity(fparams.len());
    let mut specific = false;
    for name in &fparams {
        let mut found: Option<VarType> = None;
        for (generic, erased, is_param) in &positions {
            if !is_bare_var(generic) || generic.value_str() != name.as_str() {
                continue;
            }
            if (erased.type_ != CodeType::Object && erased.array_dim == 0)
                || found.as_ref().is_some_and(|f| !f.equals(erased))
            {
                return None;
            }
            specific |= *is_param && !erased.equals(&VarType::VARTYPE_OBJECT());
            found = Some(erased.clone());
        }
        args.push(descriptor_of(&found?)?);
    }
    if !specific {
        return None;
    }
    Some(GenericType::parse(&format!("L{}<{}>;", iface.value_str(), args.concat())))
}

fn descriptor_of(t: &VarType) -> Option<String> {
    let element = match t.type_ {
        CodeType::Object => format!("L{};", t.value.as_deref()?),
        _ => t.value.as_deref()?.to_string(),
    };
    Some(format!("{}{}", "[".repeat(t.array_dim.max(0) as usize), element))
}
