//! Port of `org/jetbrains/java/decompiler/struct/attr/StructAnnotationParameterAttribute.java`.
//!
//! Parameter annotations are kept in session scope and built into class-scope
//! exprents on first use (see `struct_annotation_attribute`).

use crate::annotation_exprent::AnnotationExprent;
use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_annotation_attribute::{AnnotationRecord, StructAnnotationAttribute};
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructAnnotationParameterAttributeData {
    pub source: Vec<Vec<AnnotationRecord>>,
    pub param_annotations: Option<Vec<Vec<AnnotationExprent>>>,
    pub pinned: bool,
}

attribute_view!(
    StructAnnotationParameterAttribute,
    AnnotationParameter,
    StructAnnotationParameterAttributeData,
    is_annotation_parameter,
    as_annotation_parameter,
    try_as_annotation_parameter
);

impl StructAnnotationParameterAttribute {
    pub fn new() -> StructAnnotationParameterAttribute {
        StructAnnotationParameterAttribute::alloc_view(
            StructAnnotationParameterAttributeData::default(),
        )
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_byte();
        let mut source = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let annotations = StructAnnotationAttribute::read_annotations(pool, data);
            source.push(annotations);
        }
        self.d_mut(|d| {
            d.source = source;
            d.param_annotations = None;
            d.pinned = false;
        });
    }

    pub fn get_param_annotations(self) -> Vec<Vec<AnnotationExprent>> {
        let (cached, pinned) = self.d(|d| (d.param_annotations.clone(), d.pinned));
        if let Some(cached) = cached {
            if pinned || cached.iter().flatten().all(|a| a.is_live()) {
                return cached;
            }
        }
        let source = self.d(|d| d.source.clone());
        let built: Vec<Vec<AnnotationExprent>> = source
            .iter()
            .map(|param| param.iter().map(AnnotationRecord::to_exprent).collect())
            .collect();
        crate::infra::kuna_reclaim::note_cached_exprents(built.iter().flatten().map(|a| a.0));
        self.d_mut(|d| d.param_annotations = Some(built.clone()));
        built
    }

    /// The parsed parameter annotations, without building exprents.
    pub fn get_records(self) -> Vec<Vec<AnnotationRecord>> {
        self.d(|d| d.source.clone())
    }

    pub fn set_param_annotations(self, param_annotations: Vec<Vec<AnnotationExprent>>) {
        self.d_mut(|d| {
            d.param_annotations = Some(param_annotations);
            d.pinned = true;
        })
    }
}
