//! Fix `kt_explicit_type_arguments`: a call's explicit type arguments are
//! written after the function's name with Kotlin's type names
//! (`CollectionsKt.last<File>(list)`).

use crate::fixes::kt_util::writing_kotlin;
use crate::fixes::kt_java_syntax::kt_java_reference_cast;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// Whether the fix writes the calls of the Kotlin class being written.
pub fn active() -> bool {
    writing_kotlin()
}

/// Appends `<A, B>` for the type arguments `args`, if any and all known: a
/// type argument Vineflower does not know in full (Java's `Seq<>`) is left
/// to Kotlin's inference, as the source leaves it.
pub fn append(buf: &mut TextBuffer, args: &[VarType]) {
    if args.is_empty()
        || args
            .iter()
            .any(kt_java_reference_cast::has_unknown_argument)
    {
        return;
    }
    let names: Vec<String> = args
        .iter()
        .map(kt_java_reference_cast::type_name)
        .collect();
    buf.append("<").append(&names.join(", ")).append(">");
}
