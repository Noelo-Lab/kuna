//! kuna fix `kt_supertype_type_arguments` (`kuna_fixes`): the superclass
//! call in a Kotlin class header is written with the superclass's type
//! arguments.
//!
//! Kotlin requires a supertype's type arguments in a class header
//! (`class Coroutine<T>(..) : AbstractCoroutine<T>(..)`); it does not infer
//! them. `KConstructor.writePrimaryConstructor` writes the superclass call it
//! takes from the primary constructor's first statement with the invoked
//! class's short name only, `: AbstractCoroutine(parentContext, true, active)`.
//! The type arguments are in the class's Kotlin metadata, in its supertype
//! list; they are written after the invoked class's name, with the class's
//! type parameters in scope (fix `kt_type_parameter_id`), so that an inner
//! class's `BufferedChannel<E>` names its outer class's `E`.

use crate::k_element::KElement;
use crate::k_type::KType;
use crate::kotlin::kuna_fix_kt_type_parameter_id;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;

/// The type arguments, `<A, B>`, of the supertype of the Kotlin class `cl`
/// that names `classname` (a JVM internal name), when it has any.
pub fn type_arguments(cl: StructClass, classname: &str, indent: i32) -> Option<TextBuffer> {
    let Some(KElement::Class(cls)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    let resolver = cls.resolver.as_ref()?;
    let _scope = kuna_fixes::enabled(KunaFix::KtTypeParameterId)
        .then(|| kuna_fix_kt_type_parameter_id::class_scope(cl, &cls.proto, resolver));
    let supertype = cls
        .proto
        .get_supertype_list()
        .iter()
        .map(|t| KType::from(t, resolver))
        .find(|t| t.value.as_deref() == Some(classname))?;
    let arguments = supertype.k().type_arguments.clone()?;
    if arguments.is_empty() {
        return None;
    }
    let mut buf = TextBuffer::new();
    buf.append("<");
    for (i, argument) in arguments.iter().enumerate() {
        if i > 0 {
            buf.append(", ");
        }
        buf.append_buffer(&argument.stringify(indent + 1));
    }
    buf.append(">");
    Some(buf)
}
