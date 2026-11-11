//! kuna fix `kt_constructor_accessor_call`: in Kotlin output, a call of the
//! synthetic constructor kotlinc generates to reach a constructor from
//! outside its class is written as the call of that constructor.
//!
//! A private constructor called from the companion object or a nested class,
//! and a constructor with an inline-class parameter, are reached through a
//! public synthetic `<init>(P1..Pn, DefaultConstructorMarker)` that only
//! passes its parameters on to `<init>(P1..Pn)`; the callers pass `null` for
//! the marker. Vineflower writes the call as it is, `CipherSuite(javaName,
//! null)`, a call the class does not declare. The accessor is recognised by
//! its code, as fix `kt_inline_class_constructor_accessor` recognises it
//! (`real_constructor`); the call then drops the `null` marker and names the
//! constructor the accessor forwards to.

use crate::decompiler_context::DecompilerContext;
use crate::kotlin::kuna_fix_kt_inline_class_constructor_accessor::real_constructor;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;

const MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

/// Rewrites the constructor call of `new` when it goes through an accessor.
pub fn resugar(new: NewExprent) {
    if new.is_anonymous() || new.is_lambda() || new.is_enum_const() {
        return;
    }
    let Some(invocation) = new.get_constructor() else {
        return;
    };
    let descriptor = invocation.get_string_descriptor();
    if !descriptor.ends_with(&format!("{MARKER})V")) {
        return;
    }
    let args = invocation.get_lst_parameters();
    let n = args.len();
    if n == 0
        || invocation.get_descriptor().params.len() != n
        || !args.get(n - 1).try_as_const().is_some_and(|c| c.is_null())
    {
        return;
    }
    let Some(cl) = DecompilerContext::get_struct_context().get_class(&invocation.get_classname())
    else {
        return;
    };
    let Some(accessor) = cl.get_method("<init>", &descriptor) else {
        return;
    };
    let Some(target) = real_constructor(cl, accessor) else {
        return;
    };
    let target_descriptor = target.get_descriptor();
    args.remove_at(n - 1);
    invocation.set_descriptor(MethodDescriptor::parse_descriptor(&target_descriptor));
    invocation.set_string_descriptor(target_descriptor);
}
