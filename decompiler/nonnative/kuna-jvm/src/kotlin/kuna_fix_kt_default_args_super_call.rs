//! kuna fix `jvm_fix_kt_default_args_super_call` (`kuna_fixes`): a primary
//! constructor's supertype call that goes through the superclass's
//! default-arguments constructor (`<init>(P1..Pn, int mask.., DefaultConstructorMarker)`)
//! becomes a call of the constructor it stands for, `<init>(P1..Pn)`, with the
//! arguments the mask defaults left out. Vineflower removes only the marker, so
//! the call names a constructor the class does not have.

use crate::code_constants::{ACC_SYNTHETIC, ACC_VARARGS};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::kotlin_chooser::KotlinChooser;
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

/// Rewrites `invocation` (a supertype constructor call) when it calls the
/// superclass's default-arguments constructor, every defaulted argument is a
/// trailing one passed as a constant placeholder, and no other constructor
/// of the superclass could take the arguments left; returns whether it did.
pub fn resugar(invocation: InvocationExprent) -> bool {
    let Some(plan) = plan(invocation) else {
        return false;
    };
    let args = invocation.get_lst_parameters();
    let kept: Vec<Exprent> = args.to_vec().into_iter().take(plan.passed).collect();
    args.set_all(kept);
    invocation.set_descriptor(MethodDescriptor::parse_descriptor(&plan.descriptor));
    invocation.set_string_descriptor(plan.descriptor);
    true
}

fn plan(invocation: InvocationExprent) -> Option<Resugared> {
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
    let passed = trailing_defaults(&masks, n)?;
    if !args[passed..n].iter().all(|a| a.is_const()) {
        return None;
    }
    let descriptor = format!("({})V", params[..n].concat());

    let cl = DecompilerContext::get_struct_context().get_class(&invocation.get_classname())?;
    let default_ctor = cl.get_method("<init>", &invocation.get_string_descriptor())?;
    if !default_ctor.has_modifier(ACC_SYNTHETIC) || cl.get_method("<init>", &descriptor).is_none() {
        return None;
    }
    if !declares_defaults(cl, &descriptor, n, passed) || other_candidate(cl, &descriptor, passed) {
        return None;
    }
    Some(Resugared { descriptor, passed })
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

/// Whether the superclass's Kotlin metadata has the constructor `descriptor`
/// with `n` value parameters, every one from `passed` on declaring a default.
fn declares_defaults(cl: StructClass, descriptor: &str, n: usize, passed: usize) -> bool {
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(kclass)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return false;
    };
    let Some(resolver) = kclass.resolver.as_ref() else {
        return false;
    };
    kclass.proto.get_constructor_list().iter().any(|ctor| {
        let signature = ctor.get_extension_constructor_signature();
        signature.has_desc()
            && resolver.resolve(signature.get_desc()) == descriptor
            && ctor.get_value_parameter_count() as usize == n
            && (passed..n).all(|i| {
                Flags::DECLARES_DEFAULT_VALUE.get(ctor.get_value_parameter(i as i32).get_flags())
            })
    })
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
}
