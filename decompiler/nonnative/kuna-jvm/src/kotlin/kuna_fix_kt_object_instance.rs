//! kuna fix `kt_object_instance`: the `INSTANCE` field of an `object` is
//! written as the object's name.
//!
//! kotlinc reaches an `object`'s members through its static `INSTANCE`
//! field (`getstatic SegmentPool.INSTANCE`), which Vineflower writes as such:
//! `SegmentPool.INSTANCE.recycle(s)`. Kotlin names the object itself, and has
//! no `INSTANCE`. With the fix the static `INSTANCE` field of a class whose
//! Kotlin metadata says `object` is written as the class's name
//! (`SegmentPool.recycle(s)`, `addTransformer(DebugProbesTransformer)`),
//! which kotlinc compiles to the same `getstatic`; both the plugin's field
//! expressions and the ones the writer prints in Java's form use it.

use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::field_exprent::FieldExprent;
use crate::k_element::KElement;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::ClassKind;
use crate::text_buffer::TextBuffer;

/// The object's name, when `field` is the `INSTANCE` field of an `object`.
pub fn object_name(field: FieldExprent) -> Option<TextBuffer> {
    if !field.is_static() || field.get_name() != "INSTANCE" {
        return None;
    }
    let classname = field.get_classname();
    let cl = DecompilerContext::get_struct_context().get_class(&classname)?;
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(cls)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    if Flags::CLASS_KIND.get(cls.flags()) != Some(ClassKind::Object) {
        return None;
    }
    let short_name = DecompilerContext::get_import_collector()
        .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(&classname));
    let mut buf = TextBuffer::new();
    buf.append_class(&short_name, false, &classname);
    buf.add_bytecode_mapping(field.get_bytecode().as_ref());
    Some(buf)
}
