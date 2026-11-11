//! Port of `org/jetbrains/java/decompiler/struct/attr/StructAnnDefaultAttribute.java`.
//!
//! The default value is kept as a session-scope element and built into a
//! class-scope exprent on first use (see `struct_annotation_attribute`).

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::exprent::Exprent;
use crate::struct_annotation_attribute::{AnnotationElement, StructAnnotationAttribute};
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructAnnDefaultAttributeData {
    pub source: Option<AnnotationElement>,
    pub default_value: Option<Exprent>,
    pub pinned: bool,
}

attribute_view!(
    StructAnnDefaultAttribute,
    AnnDefault,
    StructAnnDefaultAttributeData,
    is_ann_default,
    as_ann_default,
    try_as_ann_default
);

impl StructAnnDefaultAttribute {
    pub fn new() -> StructAnnDefaultAttribute {
        StructAnnDefaultAttribute::alloc_view(StructAnnDefaultAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let source = StructAnnotationAttribute::read_annotation_element(data, pool);
        self.d_mut(|d| {
            d.source = Some(source);
            d.default_value = None;
            d.pinned = false;
        });
    }

    pub fn get_default_value(self) -> Option<Exprent> {
        let (cached, pinned) = self.d(|d| (d.default_value, d.pinned));
        if let Some(cached) = cached {
            if pinned || cached.is_live() {
                return Some(cached);
            }
        }
        if pinned {
            return None;
        }
        let built = self.d(|d| d.source.clone()).map(|s| s.to_exprent());
        crate::infra::kuna_reclaim::note_cached_exprents(built);
        self.d_mut(|d| d.default_value = built);
        built
    }

    /// The parsed default value, without building an exprent.
    pub fn get_record(self) -> Option<AnnotationElement> {
        self.d(|d| d.source.clone())
    }

    pub fn set_default_value(self, default_value: Option<Exprent>) {
        self.d_mut(|d| {
            d.default_value = default_value;
            d.pinned = true;
        })
    }
}
