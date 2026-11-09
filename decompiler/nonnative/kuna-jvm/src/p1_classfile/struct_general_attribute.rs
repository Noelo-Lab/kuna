//! Port of `org/jetbrains/java/decompiler/struct/attr/StructGeneralAttribute.java`.
//!
//! Every attribute is one session-scope arena object; the Java subclasses are
//! views over [`StructGeneralAttributeKind`]. The `Key<T>` constants are
//! [`AttributeKey<T>`]s, and `StructMember::get_attribute(key)` returns the
//! typed view `T` (the Java unchecked cast is a checked `as_*` here).
//! `ClassAttributeRegistry` is not ported: `create_attribute` is the registry
//! `init()` builds.

use std::marker::PhantomData;

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_ann_default_attribute::{
    StructAnnDefaultAttribute, StructAnnDefaultAttributeData,
};
use crate::struct_annotation_attribute::{
    StructAnnotationAttribute, StructAnnotationAttributeData,
};
use crate::struct_annotation_parameter_attribute::{
    StructAnnotationParameterAttribute, StructAnnotationParameterAttributeData,
};
use crate::struct_bootstrap_methods_attribute::{
    StructBootstrapMethodsAttribute, StructBootstrapMethodsAttributeData,
};
use crate::struct_code_attribute::{StructCodeAttribute, StructCodeAttributeData};
use crate::struct_constant_value_attribute::{
    StructConstantValueAttribute, StructConstantValueAttributeData,
};
use crate::struct_enclosing_method_attribute::{
    StructEnclosingMethodAttribute, StructEnclosingMethodAttributeData,
};
use crate::struct_exceptions_attribute::{
    StructExceptionsAttribute, StructExceptionsAttributeData,
};
use crate::struct_generic_signature_attribute::{
    StructGenericSignatureAttribute, StructGenericSignatureAttributeData,
};
use crate::struct_inner_classes_attribute::{
    StructInnerClassesAttribute, StructInnerClassesAttributeData,
};
use crate::struct_line_number_table_attribute::{
    StructLineNumberTableAttribute, StructLineNumberTableAttributeData,
};
use crate::struct_local_variable_table_attribute::{
    StructLocalVariableTableAttribute, StructLocalVariableTableAttributeData,
};
use crate::struct_local_variable_type_table_attribute::{
    StructLocalVariableTypeTableAttribute, StructLocalVariableTypeTableAttributeData,
};
use crate::struct_method_parameters_attribute::{
    StructMethodParametersAttribute, StructMethodParametersAttributeData,
};
use crate::struct_module_attribute::{StructModuleAttribute, StructModuleAttributeData};
use crate::struct_nest_host_attribute::{StructNestHostAttribute, StructNestHostAttributeData};
use crate::struct_permitted_subclasses_attribute::{
    StructPermittedSubclassesAttribute, StructPermittedSubclassesAttributeData,
};
use crate::struct_record_attribute::{StructRecordAttribute, StructRecordAttributeData};
use crate::struct_source_file_attribute::{
    StructSourceFileAttribute, StructSourceFileAttributeData,
};
use crate::struct_type_annotation_attribute::{
    StructTypeAnnotationAttribute, StructTypeAnnotationAttributeData,
};
use crate::{arena_identity, arena_object};

/// Java: `Key<T extends StructGeneralAttribute>`, the typed name of an attribute.
pub struct AttributeKey<T> {
    pub name: &'static str,
    view: PhantomData<fn() -> T>,
}

impl<T> AttributeKey<T> {
    pub const fn of(name: &'static str) -> AttributeKey<T> {
        AttributeKey {
            name,
            view: PhantomData,
        }
    }
}

impl<T> Clone for AttributeKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AttributeKey<T> {}

impl<T> PartialEq for AttributeKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> std::fmt::Debug for AttributeKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key({})", self.name)
    }
}

/// The class an [`AttributeKey`] names: the cast `get_attribute` applies.
pub trait AttributeView: Copy {
    fn from_attribute(attribute: StructGeneralAttribute) -> Self;
}

impl AttributeView for StructGeneralAttribute {
    fn from_attribute(attribute: StructGeneralAttribute) -> Self {
        attribute
    }
}

pub struct StructGeneralAttributeData {
    pub kind: StructGeneralAttributeKind,
}

pub enum StructGeneralAttributeKind {
    General,
    Code(StructCodeAttributeData),
    InnerClasses(StructInnerClassesAttributeData),
    GenericSignature(StructGenericSignatureAttributeData),
    AnnDefault(StructAnnDefaultAttributeData),
    Exceptions(StructExceptionsAttributeData),
    EnclosingMethod(StructEnclosingMethodAttributeData),
    Annotation(StructAnnotationAttributeData),
    AnnotationParameter(StructAnnotationParameterAttributeData),
    TypeAnnotation(StructTypeAnnotationAttributeData),
    LocalVariableTable(StructLocalVariableTableAttributeData),
    LocalVariableTypeTable(StructLocalVariableTypeTableAttributeData),
    ConstantValue(StructConstantValueAttributeData),
    BootstrapMethods(StructBootstrapMethodsAttributeData),
    LineNumberTable(StructLineNumberTableAttributeData),
    MethodParameters(StructMethodParametersAttributeData),
    Module(StructModuleAttributeData),
    Record(StructRecordAttributeData),
    PermittedSubclasses(StructPermittedSubclassesAttributeData),
    SourceFile(StructSourceFileAttributeData),
    NestHost(StructNestHostAttributeData),
}

arena_object!(pub StructGeneralAttribute => StructGeneralAttributeData, Session);
arena_identity!(StructGeneralAttribute);

impl Default for StructGeneralAttribute {
    fn default() -> Self {
        StructGeneralAttribute::new()
    }
}

impl StructGeneralAttribute {
    pub const ATTRIBUTE_CODE: AttributeKey<StructCodeAttribute> = AttributeKey::of("Code");
    pub const ATTRIBUTE_INNER_CLASSES: AttributeKey<StructInnerClassesAttribute> =
        AttributeKey::of("InnerClasses");
    pub const ATTRIBUTE_SIGNATURE: AttributeKey<StructGenericSignatureAttribute> =
        AttributeKey::of("Signature");
    pub const ATTRIBUTE_ANNOTATION_DEFAULT: AttributeKey<StructAnnDefaultAttribute> =
        AttributeKey::of("AnnotationDefault");
    pub const ATTRIBUTE_EXCEPTIONS: AttributeKey<StructExceptionsAttribute> =
        AttributeKey::of("Exceptions");
    pub const ATTRIBUTE_ENCLOSING_METHOD: AttributeKey<StructEnclosingMethodAttribute> =
        AttributeKey::of("EnclosingMethod");
    pub const ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS: AttributeKey<StructAnnotationAttribute> =
        AttributeKey::of("RuntimeVisibleAnnotations");
    pub const ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS: AttributeKey<StructAnnotationAttribute> =
        AttributeKey::of("RuntimeInvisibleAnnotations");
    pub const ATTRIBUTE_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: AttributeKey<
        StructAnnotationParameterAttribute,
    > = AttributeKey::of("RuntimeVisibleParameterAnnotations");
    pub const ATTRIBUTE_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: AttributeKey<
        StructAnnotationParameterAttribute,
    > = AttributeKey::of("RuntimeInvisibleParameterAnnotations");
    pub const ATTRIBUTE_RUNTIME_VISIBLE_TYPE_ANNOTATIONS: AttributeKey<
        StructTypeAnnotationAttribute,
    > = AttributeKey::of("RuntimeVisibleTypeAnnotations");
    pub const ATTRIBUTE_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: AttributeKey<
        StructTypeAnnotationAttribute,
    > = AttributeKey::of("RuntimeInvisibleTypeAnnotations");
    pub const ATTRIBUTE_LOCAL_VARIABLE_TABLE: AttributeKey<StructLocalVariableTableAttribute> =
        AttributeKey::of("LocalVariableTable");
    pub const ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE: AttributeKey<
        StructLocalVariableTypeTableAttribute,
    > = AttributeKey::of("LocalVariableTypeTable");
    pub const ATTRIBUTE_CONSTANT_VALUE: AttributeKey<StructConstantValueAttribute> =
        AttributeKey::of("ConstantValue");
    pub const ATTRIBUTE_BOOTSTRAP_METHODS: AttributeKey<StructBootstrapMethodsAttribute> =
        AttributeKey::of("BootstrapMethods");
    pub const ATTRIBUTE_SYNTHETIC: AttributeKey<StructGeneralAttribute> =
        AttributeKey::of("Synthetic");
    pub const ATTRIBUTE_DEPRECATED: AttributeKey<StructGeneralAttribute> =
        AttributeKey::of("Deprecated");
    pub const ATTRIBUTE_LINE_NUMBER_TABLE: AttributeKey<StructLineNumberTableAttribute> =
        AttributeKey::of("LineNumberTable");
    pub const ATTRIBUTE_METHOD_PARAMETERS: AttributeKey<StructMethodParametersAttribute> =
        AttributeKey::of("MethodParameters");
    pub const ATTRIBUTE_MODULE: AttributeKey<StructModuleAttribute> = AttributeKey::of("Module");
    pub const ATTRIBUTE_RECORD: AttributeKey<StructRecordAttribute> = AttributeKey::of("Record");
    pub const ATTRIBUTE_PERMITTED_SUBCLASSES: AttributeKey<StructPermittedSubclassesAttribute> =
        AttributeKey::of("PermittedSubclasses");
    pub const ATTRIBUTE_SOURCE_FILE: AttributeKey<StructSourceFileAttribute> =
        AttributeKey::of("SourceFile");
    pub const ATTRIBUTE_NEST_HOST: AttributeKey<StructNestHostAttribute> =
        AttributeKey::of("NestHost");

    /// Java: `new StructGeneralAttribute()` (`Synthetic`, `Deprecated`).
    pub fn new() -> StructGeneralAttribute {
        StructGeneralAttribute::alloc(StructGeneralAttributeData {
            kind: StructGeneralAttributeKind::General,
        })
    }

    /// Java: `createAttribute(Key.of(name))`; `None` for an unknown attribute.
    pub fn create_attribute(name: &str) -> Option<StructGeneralAttribute> {
        Some(match name {
            "Code" => StructCodeAttribute::new().0,
            "InnerClasses" => StructInnerClassesAttribute::new().0,
            "ConstantValue" => StructConstantValueAttribute::new().0,
            "Signature" => StructGenericSignatureAttribute::new().0,
            "AnnotationDefault" => StructAnnDefaultAttribute::new().0,
            "Exceptions" => StructExceptionsAttribute::new().0,
            "EnclosingMethod" => StructEnclosingMethodAttribute::new().0,
            "RuntimeVisibleAnnotations" | "RuntimeInvisibleAnnotations" => {
                StructAnnotationAttribute::new().0
            }
            "RuntimeVisibleParameterAnnotations" | "RuntimeInvisibleParameterAnnotations" => {
                StructAnnotationParameterAttribute::new().0
            }
            "RuntimeVisibleTypeAnnotations" | "RuntimeInvisibleTypeAnnotations" => {
                StructTypeAnnotationAttribute::new().0
            }
            "LocalVariableTable" => StructLocalVariableTableAttribute::new().0,
            "LocalVariableTypeTable" => StructLocalVariableTypeTableAttribute::new().0,
            "BootstrapMethods" => StructBootstrapMethodsAttribute::new().0,
            "Synthetic" | "Deprecated" => StructGeneralAttribute::new(),
            "LineNumberTable" => StructLineNumberTableAttribute::new().0,
            "MethodParameters" => StructMethodParametersAttribute::new().0,
            "Module" => StructModuleAttribute::new().0,
            "Record" => StructRecordAttribute::new().0,
            "PermittedSubclasses" => StructPermittedSubclassesAttribute::new().0,
            "SourceFile" => StructSourceFileAttribute::new().0,
            "NestHost" => StructNestHostAttribute::new().0,
            _ => return None,
        })
    }

    /// Java: `init()`; the registry it fills is built into `create_attribute`.
    pub fn init() {}

    pub fn init_content(self, data: &mut ByteCursor, pool: ConstantPool, version: BytecodeVersion) {
        if let Some(a) = self.try_as_code() {
            StructCodeAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_inner_classes() {
            StructInnerClassesAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_generic_signature() {
            StructGenericSignatureAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_ann_default() {
            StructAnnDefaultAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_exceptions() {
            StructExceptionsAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_enclosing_method() {
            StructEnclosingMethodAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_annotation() {
            StructAnnotationAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_annotation_parameter() {
            StructAnnotationParameterAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_type_annotation() {
            StructTypeAnnotationAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_local_variable_table() {
            StructLocalVariableTableAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_local_variable_type_table() {
            StructLocalVariableTypeTableAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_constant_value() {
            StructConstantValueAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_bootstrap_methods() {
            StructBootstrapMethodsAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_line_number_table() {
            StructLineNumberTableAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_method_parameters() {
            StructMethodParametersAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_module() {
            StructModuleAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_record() {
            StructRecordAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_permitted_subclasses() {
            StructPermittedSubclassesAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_source_file() {
            StructSourceFileAttribute::init_content(a, data, pool, version)
        } else if let Some(a) = self.try_as_nest_host() {
            StructNestHostAttribute::init_content(a, data, pool, version)
        }
    }
}

/// Declares an attribute subclass view and its [`AttributeView`] cast; the
/// caller imports `StructGeneralAttribute` and `StructGeneralAttributeKind`.
macro_rules! attribute_view {
    ($view:ident, $variant:ident, $data:ty, $is:ident, $as_:ident, $try_as:ident) => {
        $crate::java_subclass!(pub $view: StructGeneralAttribute,
            StructGeneralAttributeKind::$variant($data), $is, $as_, $try_as);

        impl $crate::struct_general_attribute::AttributeView for $view {
            fn from_attribute(attribute: StructGeneralAttribute) -> Self {
                attribute.$as_()
            }
        }

        impl Default for $view {
            fn default() -> Self {
                $view::new()
            }
        }

        impl $view {
            fn alloc_view(data: $data) -> $view {
                $view(StructGeneralAttribute::alloc(
                    $crate::struct_general_attribute::StructGeneralAttributeData {
                        kind: StructGeneralAttributeKind::$variant(data),
                    },
                ))
            }
        }
    };
}
pub(crate) use attribute_view;
