//! kuna fix `type_annotation_argument_scope` (`kuna_fixes`): a type
//! annotation's `type_path` leads from the outermost type to the annotated
//! part, one step per level (JVMS 4.7.20.2). `TypeAnnotation.appendCastTypeName`
//! walks the type and, at each generic argument, takes the next step of every
//! annotation whose next step names that argument, then recurses into the
//! argument with all of the method's annotations. An annotation meant for a
//! later argument then takes its steps inside an earlier one: in
//! `Function<Map<String, @A Object>, @B Map<String, @C Object>>` the steps
//! `[argument 1, argument 1]` of `@C` match inside the first argument, and it
//! is written `Function<Map<String, @A @C Object>, @B Map<String, Object>>`.
//! With the fix the recursion into an argument sees only the annotations
//! whose path enters it.

use crate::type_annotation::{PathKind, TypeAnnotationEntry};

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
