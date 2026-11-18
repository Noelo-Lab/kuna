//! Fix family `type_annotations`: `type_annotation_argument_scope` recurses
//! into a generic argument with only the annotations whose path enters it;
//! `type_annotation_outer_arguments` keeps the type arguments of the
//! enclosing classes of an annotated inner type; `type_annotation_qualified_name`
//! keeps the import collector's qualifier before the annotation.

use crate::decompiler_context::DecompilerContext;
use crate::type_annotation::{PathKind, TypeAnnotationEntry};
use crate::var_type::VarType;

/// The indices of the annotations whose next step enters generic argument
/// `index`, taken before that step is taken.
pub fn entering(type_annotations: &[TypeAnnotationEntry], index: i32) -> Vec<usize> {
    type_annotations
        .iter()
        .enumerate()
        .filter(|(_, anno)| {
            anno.a
                .front()
                .is_some_and(|v| v.kind() == PathKind::GenericArgument && v.type_arg() == index)
        })
        .map(|(i, _)| i)
        .collect()
}

/// Copies of the annotations at `indices`, for the recursion into the argument.
pub fn take(type_annotations: &[TypeAnnotationEntry], indices: &[usize]) -> Vec<TypeAnnotationEntry> {
    indices.iter().map(|&i| type_annotations[i].clone()).collect()
}

/// Writes the state the recursion left the annotations in back to their places.
pub fn put_back(
    type_annotations: &mut [TypeAnnotationEntry],
    indices: &[usize],
    sub: Vec<TypeAnnotationEntry>,
) {
    for (&i, anno) in indices.iter().zip(sub) {
        type_annotations[i] = anno;
    }
}

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

/// The qualifier (with its trailing `.`) of the import collector's name for
/// `clazz`, the outermost class `appendCastTypeName` writes, in Java form:
/// empty when the collector writes it by its simple name.
pub fn qualifier(clazz: &str) -> String {
    let Some(short) = DecompilerContext::get_import_collector().get_short_name(clazz) else {
        return String::new();
    };
    match short.rfind('.') {
        Some(dot) => short[..=dot].to_string(),
        None => String::new(),
    }
}
