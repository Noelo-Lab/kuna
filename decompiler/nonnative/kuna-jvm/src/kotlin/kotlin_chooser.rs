//! Port of `org/vineflower/kotlin/KotlinChooser.java`. The decoding half of
//! `parseMetadataFor` is `kt_metadata`; the `EXTENSIONS` registry has no
//! counterpart (`kt_proto` always reads the JVM extensions).

#![allow(dead_code, unused_variables)]

use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::language_chooser::LanguageChooser;
use crate::struct_annotation_attribute::{AnnotationElement, StructAnnotationAttribute};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::{AttributeKey, StructGeneralAttribute};
use crate::unported;

#[derive(Clone, Copy, Debug, Default)]
pub struct KotlinChooser;

impl KotlinChooser {
    const ANNOTATION_ATTRIBUTES: [AttributeKey<StructAnnotationAttribute>; 2] = [
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS,
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS,
    ];

    pub fn new() -> KotlinChooser {
        KotlinChooser
    }

    pub fn is_language(&self, cl: StructClass) -> bool {
        unported!("KotlinChooser.isLanguage")
    }

    pub fn parse_metadata_for(cl: StructClass) {
        unported!("KotlinChooser.parseMetadataFor")
    }

    /// `getDataFromExpr((NewExprent) value)` over the session-scope annotation value.
    fn get_data_from_expr(d2: &AnnotationElement) -> Vec<String> {
        KotlinMetadataAnnotation::data_from_expr(d2)
    }
}

impl LanguageChooser for KotlinChooser {
    fn is_language(&self, cl: StructClass) -> bool {
        KotlinChooser::is_language(self, cl)
    }
}
