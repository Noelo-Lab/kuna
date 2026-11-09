//! Port of `org/jetbrains/java/decompiler/struct/attr/StructTypeAnnotationAttribute.java`.
//!
//! The parse (targets, validated paths, annotation) runs at load time; the
//! `TypeAnnotation`s, which hold class-scope exprents, are built on first use
//! (see `struct_annotation_attribute`).

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::jthrow;
use crate::struct_annotation_attribute::{AnnotationRecord, StructAnnotationAttribute};
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::type_annotation::{PathKind, PathValue, TypeAnnotation};

/// A parsed `type_annotation`, in session scope.
#[derive(Clone)]
pub struct TypeAnnotationRecord {
    pub target: i32,
    pub path: Vec<PathValue>,
    pub annotation: AnnotationRecord,
}

impl TypeAnnotationRecord {
    pub fn to_type_annotation(&self) -> TypeAnnotation {
        TypeAnnotation::new(self.target, self.path.clone(), self.annotation.to_exprent())
    }
}

#[derive(Default)]
pub struct StructTypeAnnotationAttributeData {
    pub source: Vec<TypeAnnotationRecord>,
    pub annotations: Option<Vec<TypeAnnotation>>,
    pub pinned: bool,
}

attribute_view!(
    StructTypeAnnotationAttribute,
    TypeAnnotation,
    StructTypeAnnotationAttributeData,
    is_type_annotation,
    as_type_annotation,
    try_as_type_annotation
);

impl StructTypeAnnotationAttribute {
    pub fn new() -> StructTypeAnnotationAttribute {
        StructTypeAnnotationAttribute::alloc_view(StructTypeAnnotationAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_short();
        let mut source = Vec::with_capacity(len as usize);
        for _ in 0..len {
            source.push(StructTypeAnnotationAttribute::parse(data, pool));
        }
        self.d_mut(|d| {
            d.source = source;
            d.annotations = None;
            d.pinned = false;
        });
    }

    fn parse(data: &mut ByteCursor, pool: ConstantPool) -> TypeAnnotationRecord {
        let target_type = data.read_unsigned_byte();
        let mut target = target_type.wrapping_shl(24);

        match target_type {
            TypeAnnotation::CLASS_TYPE_PARAMETER
            | TypeAnnotation::METHOD_TYPE_PARAMETER
            | TypeAnnotation::METHOD_PARAMETER => {
                target |= data.read_unsigned_byte();
            }

            TypeAnnotation::SUPER_TYPE_REFERENCE
            | TypeAnnotation::CLASS_TYPE_PARAMETER_BOUND
            | TypeAnnotation::METHOD_TYPE_PARAMETER_BOUND
            | TypeAnnotation::THROWS_REFERENCE
            | TypeAnnotation::CATCH_CLAUSE
            | TypeAnnotation::EXPR_INSTANCEOF
            | TypeAnnotation::EXPR_NEW
            | TypeAnnotation::EXPR_CONSTRUCTOR_REF
            | TypeAnnotation::EXPR_METHOD_REF => {
                target |= data.read_unsigned_short();
            }

            TypeAnnotation::TYPE_ARG_CAST
            | TypeAnnotation::TYPE_ARG_CONSTRUCTOR_CALL
            | TypeAnnotation::TYPE_ARG_METHOD_CALL
            | TypeAnnotation::TYPE_ARG_CONSTRUCTOR_REF
            | TypeAnnotation::TYPE_ARG_METHOD_REF => {
                data.skip_bytes(3);
            }

            TypeAnnotation::LOCAL_VARIABLE | TypeAnnotation::RESOURCE_VARIABLE => {
                let count = data.read_unsigned_short();
                data.skip_bytes(count * 6);
            }

            TypeAnnotation::FIELD
            | TypeAnnotation::METHOD_RETURN_TYPE
            | TypeAnnotation::METHOD_RECEIVER => {}

            _ => jthrow!("unknown target type: {}", target_type),
        }

        let path_length = data.read_unsigned_byte();
        let mut path = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            let kind = path_kind(data.read_byte());
            let type_arg = data.read_byte() as i32;
            path.push(PathValue::new(kind, type_arg));
        }

        let annotation = StructAnnotationAttribute::read_annotation(data, pool);

        TypeAnnotationRecord {
            target,
            path,
            annotation,
        }
    }

    pub fn get_annotations(self) -> Vec<TypeAnnotation> {
        let (cached, pinned) = self.d(|d| (d.annotations.clone(), d.pinned));
        if let Some(cached) = cached {
            if pinned || cached.iter().all(|a| a.get_annotation().is_live()) {
                return cached;
            }
        }
        let source = self.d(|d| d.source.clone());
        let built: Vec<TypeAnnotation> = source
            .iter()
            .map(TypeAnnotationRecord::to_type_annotation)
            .collect();
        crate::infra::kuna_reclaim::note_cached_exprents(built.iter().map(|a| a.get_annotation().0));
        self.d_mut(|d| d.annotations = Some(built.clone()));
        built
    }

    /// The parsed type annotations, without building exprents.
    pub fn get_records(self) -> Vec<TypeAnnotationRecord> {
        self.d(|d| d.source.clone())
    }

    pub fn set_annotations(self, annotations: Vec<TypeAnnotation>) {
        self.d_mut(|d| {
            d.annotations = Some(annotations);
            d.pinned = true;
        })
    }
}

/// `PathKind.VALUES[index]`.
fn path_kind(index: i8) -> PathKind {
    match index {
        0 => PathKind::DeeperArray,
        1 => PathKind::DeeperNested,
        2 => PathKind::OnWildcard,
        3 => PathKind::GenericArgument,
        _ => jthrow!(ArrayIndexOutOfBoundsException; "Index {} out of bounds for length 4", index),
    }
}
