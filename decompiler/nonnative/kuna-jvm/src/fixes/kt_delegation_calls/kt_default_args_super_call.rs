//! kuna fix `kt_default_args_super_call` (`kuna_fixes`): a primary
//! constructor's supertype call that goes through the superclass's
//! default-arguments constructor (`<init>(P1..Pn, int mask.., DefaultConstructorMarker)`)
//! becomes a call of the constructor it stands for, `<init>(P1..Pn)`, with the
//! arguments the mask defaults left out. Vineflower removes only the marker, so
//! the call names a constructor the class does not have. The masks number
//! the value parameters, which for an inner superclass follow the outer
//! instance. Where leaving out trailing arguments is not enough (a default
//! before a passed argument, or another constructor that would take the
//! arguments left), the passed arguments are written by name.

use crate::code_constants::{ACC_SYNTHETIC, ACC_VARARGS};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_descriptor::MethodDescriptor;
use crate::struct_class::StructClass;
use crate::var_type::VarType;

const MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

/// The call of a default-arguments constructor, as the call it stands for.
#[derive(Debug, PartialEq, Eq)]
pub struct Resugared {
    /// The descriptor of the constructor whose defaults the call uses.
    pub descriptor: String,
    /// How many leading arguments the call passes; the rest are defaulted.
    pub passed: usize,
}

/// A value parameter of a constructor, from the class's Kotlin metadata.
struct MetaParam {
    name: String,
    default: bool,
    vararg: bool,
}

/// A call of a default-arguments constructor, taken apart.
struct DefaultsCall {
    args: Vec<Exprent>,
    masks: Vec<i32>,
    /// The parameters before the masks: the outer instance, if any, then
    /// the value parameters.
    n: usize,
    descriptor: String,
    cl: StructClass,
    /// Every constructor in the metadata, the called one's first.
    ctors: Vec<Vec<MetaParam>>,
}

/// Rewrites `invocation` (a supertype constructor call) when it calls the
/// superclass's default-arguments constructor with constant placeholders for
/// the defaulted arguments; returns whether it did. When the defaulted
/// arguments are trailing ones and no other constructor of the superclass
/// could take the arguments left, the call becomes a positional call of the
/// constructor the defaults belong to; otherwise, when naming the passed
/// arguments selects that constructor alone, every passed argument is
/// written by name.
pub fn resugar(invocation: InvocationExprent) -> bool {
    let Some(call) = take_apart(invocation) else {
        return false;
    };
    if let Some(plan) = plan(&call) {
        let args = invocation.get_lst_parameters();
        let kept: Vec<Exprent> = args.to_vec().into_iter().take(plan.passed).collect();
        args.set_all(kept);
        invocation.set_descriptor(MethodDescriptor::parse_descriptor(&plan.descriptor));
        invocation.set_string_descriptor(plan.descriptor);
        return true;
    }
    match named(&call) {
        Some(names) => {
            invocation.set_kuna_arg_names(names.into());
            true
        }
        None => false,
    }
}

fn take_apart(invocation: InvocationExprent) -> Option<DefaultsCall> {
    let params: Vec<String> = invocation
        .get_descriptor()
        .params
        .iter()
        .map(VarType::to_string)
        .collect();
    let args = invocation.get_lst_parameters().to_vec();
    if args.len() != params.len() {
        return None;
    }
    let masks: Vec<i32> = mask_args(&params)?
        .map(|i| int_constant(args[i]))
        .collect::<Option<_>>()?;
    let n = params.len() - masks.len() - 1;
    let descriptor = format!("({})V", params[..n].concat());

    let cl = DecompilerContext::get_struct_context().get_class(&invocation.get_classname())?;
    let default_ctor = cl.get_method("<init>", &invocation.get_string_descriptor())?;
    if !default_ctor.has_modifier(ACC_SYNTHETIC) || cl.get_method("<init>", &descriptor).is_none() {
        return None;
    }
    let ctors = metadata_constructors(cl, &descriptor)?;
    Some(DefaultsCall {
        args,
        masks,
        n,
        descriptor,
        cl,
        ctors,
    })
}

fn plan(call: &DefaultsCall) -> Option<Resugared> {
    let defaults: Vec<bool> = call.ctors[0].iter().map(|p| p.default).collect();
    let prefix = call.n.checked_sub(defaults.len()).filter(|&p| p <= 1)?;
    let passed_values = trailing_defaults(&call.masks, defaults.len())?;
    let passed = prefix + passed_values;
    if !defaults[passed_values..].iter().all(|&d| d)
        || !call.args[passed..call.n].iter().all(|a| a.is_const())
        || other_candidate(call.cl, &call.descriptor, passed)
    {
        return None;
    }
    Some(Resugared {
        descriptor: call.descriptor.clone(),
        passed,
    })
}

/// The argument names of the call written by name: the passed value
/// arguments named, the rest (defaulted ones, masks, marker) left out.
/// Only when the call has no outer instance, every defaulted parameter
/// declares a default, no passed one is `vararg`, and of the superclass's
/// constructors only the called one accepts those names.
fn named(call: &DefaultsCall) -> Option<Vec<Option<String>>> {
    let target = &call.ctors[0];
    if call.n != target.len() {
        return None;
    }
    let defaulted = defaulted_parameters(&call.masks, call.n)?;
    let mut names = vec![None; call.args.len()];
    for (i, param) in target.iter().enumerate() {
        if defaulted[i] {
            if !param.default || !call.args[i].is_const() {
                return None;
            }
        } else if param.vararg {
            return None;
        } else {
            names[i] = Some(param.name.clone());
        }
    }
    let passed: Vec<&str> = names.iter().flatten().map(String::as_str).collect();
    let accepting = call
        .ctors
        .iter()
        .filter(|ctor| accepts_names(ctor, &passed))
        .count();
    (accepting == 1 && accepts_names(target, &passed)).then(|| {
        names
            .into_iter()
            .map(|name| name.map(|n| KotlinWriter::to_valid_kotlin_identifier(&n)))
            .collect()
    })
}

/// Which of the `n` parameters the masks default, when they default at
/// least one and none beyond `n`.
pub fn defaulted_parameters(masks: &[i32], n: usize) -> Option<Vec<bool>> {
    let defaulted = |i: usize| masks[i / 32] as u32 & (1u32 << (i % 32)) != 0;
    let beyond = (n..masks.len() * 32).any(defaulted);
    let flags: Vec<bool> = (0..n).map(defaulted).collect();
    (!beyond && flags.contains(&true)).then_some(flags)
}

/// Whether a call naming exactly `names` can select `ctor`: it has every
/// name, and each parameter not named has a default or is `vararg`.
fn accepts_names(ctor: &[MetaParam], names: &[&str]) -> bool {
    names.iter().all(|n| ctor.iter().any(|p| p.name == *n))
        && ctor
            .iter()
            .all(|p| p.default || p.vararg || names.contains(&p.name.as_str()))
}

/// The indexes of the mask arguments when `params` has the default-arguments
/// shape: n >= 1 parameters, n / 32 + 1 `int` masks, then the marker.
pub fn mask_args(params: &[String]) -> Option<std::ops::Range<usize>> {
    let total = params.len();
    if params.last().map(String::as_str) != Some(MARKER) {
        return None;
    }
    let n = (1..total).find(|n| n + n / 32 + 2 == total)?;
    let masks = n..total - 1;
    params[masks.clone()]
        .iter()
        .all(|p| p == "I")
        .then_some(masks)
}

/// How many leading parameters are passed, when the masks default the other
/// parameters of `n` and only those (at least one).
pub fn trailing_defaults(masks: &[i32], n: usize) -> Option<usize> {
    let defaulted = |i: usize| masks[i / 32] as u32 & (1u32 << (i % 32)) != 0;
    let passed = (0..n).find(|&i| defaulted(i)).unwrap_or(n);
    let beyond = (n..masks.len() * 32).any(defaulted);
    (passed < n && (passed..n).all(defaulted) && !beyond).then_some(passed)
}

fn int_constant(expr: Exprent) -> Option<i32> {
    let value = expr.try_as_const()?.get_value()?;
    match value {
        crate::const_exprent::ConstValue::Int(v) => Some(v),
        _ => None,
    }
}

/// The value parameters of every constructor in the superclass's Kotlin
/// metadata, the one with the JVM descriptor `descriptor` first (`None`
/// without metadata or without that constructor). The value parameters are
/// the descriptor's last ones: an inner class's constructor takes the outer
/// instance first, and its masks number the value parameters only.
fn metadata_constructors(cl: StructClass, descriptor: &str) -> Option<Vec<Vec<MetaParam>>> {
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(kclass)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    let resolver = kclass.resolver.as_ref()?;
    let mut target = None;
    let mut others = Vec::new();
    for ctor in kclass.proto.get_constructor_list() {
        let params: Vec<MetaParam> = (0..ctor.get_value_parameter_count())
            .map(|i| {
                let param = ctor.get_value_parameter(i);
                MetaParam {
                    name: resolver.resolve(param.get_name()),
                    default: Flags::DECLARES_DEFAULT_VALUE.get(param.get_flags()),
                    vararg: param.has_vararg_element_type(),
                }
            })
            .collect();
        let signature = ctor.get_extension_constructor_signature();
        if target.is_none()
            && signature.has_desc()
            && resolver.resolve(signature.get_desc()) == descriptor
        {
            target = Some(params);
        } else {
            others.push(params);
        }
    }
    let mut ctors = vec![target?];
    ctors.extend(others);
    Some(ctors)
}

/// Whether another constructor of the superclass might also accept `passed`
/// positional arguments, so that leaving the defaults out could select it.
fn other_candidate(cl: StructClass, descriptor: &str, passed: usize) -> bool {
    cl.get_methods().to_vec().into_iter().any(|mt| {
        if mt.get_name() != "<init>"
            || mt.get_descriptor() == descriptor
            || mt.has_modifier(ACC_SYNTHETIC)
        {
            return false;
        }
        let params: Vec<String> = mt
            .method_descriptor()
            .params
            .iter()
            .map(VarType::to_string)
            .collect();
        let with_defaults = format!(
            "({}{}{})V",
            params.concat(),
            "I".repeat(params.len() / 32 + 1),
            MARKER
        );
        params.len() == passed
            || mt.has_modifier(ACC_VARARGS)
            || (params.len() > passed && cl.get_method("<init>", &with_defaults).is_some())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &[&str]) -> Vec<String> {
        s.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_default_arguments_shape_is_recognised() {
        assert_eq!(
            mask_args(&p(&["I", "Lkotlin/jvm/functions/Function1;", "I", MARKER])),
            Some(2..3)
        );
        assert_eq!(mask_args(&p(&["I", "I", MARKER])), Some(1..2));
        assert_eq!(mask_args(&p(&[MARKER])), None);
        assert_eq!(mask_args(&p(&["I", "Lpkg/X;", MARKER])), None);
        assert_eq!(
            mask_args(&p(&[
                "Z",
                "Z",
                "I",
                "Lkotlin/io/encoding/Base64$PaddingOption;",
                MARKER
            ])),
            None
        );
        let mut wide = vec!["J".to_string(); 33];
        wide.extend(p(&["I", "I", MARKER]));
        assert_eq!(mask_args(&wide), Some(33..35));
        let mut ambiguous = vec!["J".to_string(); 32];
        ambiguous.extend(p(&["I", MARKER]));
        assert_eq!(mask_args(&ambiguous), None);
    }

    #[test]
    fn only_trailing_defaults_are_left_out() {
        assert_eq!(trailing_defaults(&[0b10], 2), Some(1));
        assert_eq!(trailing_defaults(&[0b110], 3), Some(1));
        assert_eq!(trailing_defaults(&[0b111], 3), Some(0));
        assert_eq!(trailing_defaults(&[0b010], 3), None);
        assert_eq!(trailing_defaults(&[0], 3), None);
        assert_eq!(trailing_defaults(&[0b1000], 3), None);
        assert_eq!(trailing_defaults(&[0, 0b1], 33), Some(32));
        assert_eq!(trailing_defaults(&[1 << 31, 0b1], 33), Some(31));
        assert_eq!(trailing_defaults(&[1 << 31, 0b0], 33), None);
    }

    #[test]
    fn any_defaulted_parameters_are_found() {
        assert_eq!(
            defaulted_parameters(&[0b010], 3),
            Some(vec![false, true, false])
        );
        assert_eq!(
            defaulted_parameters(&[0b101], 3),
            Some(vec![true, false, true])
        );
        assert_eq!(defaulted_parameters(&[0], 3), None);
        assert_eq!(defaulted_parameters(&[0b1000], 3), None);
        assert_eq!(
            defaulted_parameters(&[0, 0b1], 33).map(|d| d[32]),
            Some(true)
        );
    }

    fn param(name: &str, default: bool, vararg: bool) -> MetaParam {
        MetaParam {
            name: name.to_string(),
            default,
            vararg,
        }
    }

    #[test]
    fn names_select_a_constructor_by_its_parameter_names_and_defaults() {
        let primary = [param("a", false, false), param("b", true, false)];
        let secondary = [param("n", false, false)];
        let spread = [param("a", false, false), param("rest", false, true)];
        assert!(accepts_names(&primary, &["a"]));
        assert!(!accepts_names(&secondary, &["a"]));
        assert!(accepts_names(&spread, &["a"]));
        assert!(!accepts_names(&primary, &["b"]));
        assert!(!accepts_names(&primary, &["a", "c"]));
    }
}
