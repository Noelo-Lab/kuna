//! kuna fix `kt_explicit_type_arguments`: in Kotlin output, a call's
//! explicit type arguments, which Vineflower writes in Java's syntax before
//! the function's name (`CollectionsKt.<File>last(list)`), are written after
//! it, as Kotlin writes them, with Kotlin's type names
//! (`CollectionsKt.last<File>(list)`).

use crate::kuna_fix_kt_java_cast_syntax;
use crate::kuna_fix_kt_java_reference_cast;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// Whether the fix writes the calls of the Kotlin class being written.
pub fn active() -> bool {
    kuna_fixes::enabled(KunaFix::KtExplicitTypeArguments)
        && kuna_fix_kt_java_cast_syntax::writing_kotlin()
}

/// Appends `<A, B>` for the type arguments `args`, if any and all known: a
/// type argument Vineflower does not know in full (Java's `Seq<>`) is left
/// to Kotlin's inference, as the source leaves it.
pub fn append(buf: &mut TextBuffer, args: &[VarType]) {
    if args.is_empty()
        || args
            .iter()
            .any(kuna_fix_kt_java_reference_cast::has_unknown_argument)
    {
        return;
    }
    let names: Vec<String> = args
        .iter()
        .map(kuna_fix_kt_java_reference_cast::type_name)
        .collect();
    buf.append("<").append(&names.join(", ")).append(">");
}
