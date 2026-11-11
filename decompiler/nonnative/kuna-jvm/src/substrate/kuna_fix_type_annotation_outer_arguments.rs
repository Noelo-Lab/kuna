//! kuna fix `type_annotation_outer_arguments` (`kuna_fixes`), called from
//! `TypeAnnotation::append_cast_type_name`.
//!
//! A class type that carries a type annotation is written one class name at
//! a time, from the outermost class down to the type, and only the type's
//! own type arguments are written: an inner class of a generic class,
//! `CombinedFuture<V>.CombinedFutureInterruptibleTask<?>` with `@Nullable`
//! on the inner type, becomes `CombinedFuture.@Nullable
//! CombinedFutureInterruptibleTask<?>`, type arguments on a member of a raw
//! type, which javac rejects (Guava's `CombinedFuture`, lost with its field).
//! With the fix each enclosing class keeps the type arguments the generic
//! type gives it: `CombinedFuture<V>.@Nullable CombinedFutureInterruptibleTask<?>`.

use crate::var_type::VarType;

/// The type arguments `type_` gives its enclosing class `class` (internal
/// name), when it gives it any.
pub fn arguments_of(type_: &VarType, class: &str) -> Option<Vec<Option<VarType>>> {
    let mut parent = type_.try_as_generic()?.get_parent();
    while let Some(p) = parent {
        let generic = p.try_as_generic();
        if p.value.as_deref() == Some(class) {
            let args = generic?.get_arguments().to_vec();
            return (!args.is_empty()).then_some(args);
        }
        parent = generic.and_then(|g| g.get_parent());
    }
    None
}
