//! kuna fix `type_annotation_qualified_name` (`kuna_fixes`): a class type
//! that carries a type annotation is written by `TypeAnnotation.appendCastTypeName`
//! one class name at a time, from the outermost class the class file names
//! down to the type, each by its simple name; a class outside the input is a
//! single name (`Map$Entry` is `Entry`). The import collector, which decides
//! how the same type is written without an annotation, may have to qualify it:
//! `Map.Entry` when another `Entry` is in scope, the full name when a class of
//! the same simple name is in the current package. Vineflower drops that
//! qualifier, so `Map.@Nullable Entry` becomes `@Nullable Entry`, which names
//! a different type or none. With the fix the first name is preceded by the
//! qualifier of the import collector's name for it, and the annotation goes
//! after the qualifier, where Java requires it (JLS 9.7.4).

use crate::decompiler_context::DecompilerContext;

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
