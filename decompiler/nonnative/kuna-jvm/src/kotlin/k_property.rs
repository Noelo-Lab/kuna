//! Port of `org/vineflower/kotlin/struct/KProperty.java`.
//!
//! The record lives for the session (it is attached to members by `kt_attrs`)
//! while exprents are class-scope objects, so `annotations` records where each
//! annotation comes from ([`KPropertyAnnotation`]) and the exprent is fetched
//! in the scope of the class being written.

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::class_wrapper::ClassWrapper;
use crate::exprent::Exprent;
use crate::k_property_accessor::KPropertyAccessor;
use crate::k_type::KType;
use crate::key::Key;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::rt::{JavaEq, JavaHash};
use crate::struct_annotation_attribute::StructAnnotationAttribute;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::text_buffer::TextBuffer;
use crate::unported;

/// `Function<ClassWrapper, Exprent>`: a property's initializer or delegate, or `null`.
pub type ExprentSupplier = Rc<dyn Fn(ClassWrapper) -> Option<Exprent>>;

/// One entry of `KProperty.annotations`.
#[derive(Clone)]
pub enum KPropertyAnnotation {
    /// `attribute.getAnnotations().get(index)`.
    Attribute(StructAnnotationAttribute, i32),
    /// `KProperty.DEPRECATED_ANNOTATION`.
    Deprecated,
}

impl KPropertyAnnotation {
    /// The annotation exprent, in the current class scope.
    pub fn get(&self) -> AnnotationExprent {
        match self {
            KPropertyAnnotation::Attribute(attribute, index) => {
                attribute.get_annotations()[*index as usize]
            }
            KPropertyAnnotation::Deprecated => KProperty::DEPRECATED_ANNOTATION(),
        }
    }
}

#[derive(Clone)]
pub struct KProperty {
    pub name: String,
    /// `null` when the metadata has no return type.
    pub type_: Option<KType>,
    pub flags: i32,
    pub getter: Option<KPropertyAccessor>,
    pub setter: Option<KPropertyAccessor>,
    pub setter_param_name: Option<String>,
    pub underlying_field: Option<StructField>,
    pub initializer: Option<ExprentSupplier>,
    pub annotations: Option<Vec<KPropertyAnnotation>>,
    pub class_struct: StructClass,
}

impl KProperty {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        type_: Option<KType>,
        flags: i32,
        getter: Option<KPropertyAccessor>,
        setter: Option<KPropertyAccessor>,
        setter_param_name: Option<String>,
        underlying_field: Option<StructField>,
        initializer: Option<ExprentSupplier>,
        annotations: Option<Vec<KPropertyAnnotation>>,
        class_struct: StructClass,
    ) -> KProperty {
        KProperty {
            name,
            type_,
            flags,
            getter,
            setter,
            setter_param_name,
            underlying_field,
            initializer,
            annotations,
            class_struct,
        }
    }

    /// `KProperty.DEPRECATED_ANNOTATION`: `@kotlin.Deprecated(message = "Deprecated by attribute.")`.
    #[allow(non_snake_case)]
    pub fn DEPRECATED_ANNOTATION() -> AnnotationExprent {
        unported!("KProperty.DEPRECATED_ANNOTATION")
    }

    /// The record accessor `annotations()`, as exprents of the current class scope.
    pub fn get_annotations(&self) -> Option<Vec<AnnotationExprent>> {
        self.annotations
            .as_ref()
            .map(|annotations| annotations.iter().map(KPropertyAnnotation::get).collect())
    }

    pub fn stringify(&self, class_wrapper: ClassWrapper, indent: i32) -> TextBuffer {
        unported!("KProperty.stringify")
    }

    pub fn parse(
        class_struct: StructClass,
        proto_properties: &[kt_proto::Property],
        name_resolver: Option<&MetadataNameResolver>,
        companion_parent: Option<StructClass>,
    ) {
        unported!("KProperty.parse")
    }

    /// The record's `equals` (the suppliers compare by identity).
    pub fn equals(&self, o: &KProperty) -> bool {
        unported!("KProperty.equals")
    }

    /// The record's `hashCode`.
    pub fn hash_code(&self) -> i32 {
        unported!("KProperty.hashCode")
    }
}

impl JavaHash for KProperty {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for KProperty {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

/// `KProperty.PropertyMethod`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum PropertyMethod {
    Getter,
    Setter,
}

impl PropertyMethod {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            PropertyMethod::Getter => "GETTER",
            PropertyMethod::Setter => "SETTER",
        }
    }

    /// `PropertyMethod.KEY` (`Key.of(PropertyMethod.class.getName())`).
    #[allow(non_snake_case)]
    pub fn KEY() -> Key<PropertyMethod> {
        Key::of("org.vineflower.kotlin.struct.KProperty$PropertyMethod")
    }
}
