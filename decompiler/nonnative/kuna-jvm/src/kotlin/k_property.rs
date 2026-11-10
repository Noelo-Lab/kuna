//! Port of `org/vineflower/kotlin/struct/KProperty.java`.
//!
//! The record lives for the session (it is attached to members by `kt_attrs`)
//! while exprents are class-scope objects, so `annotations` records where each
//! annotation comes from ([`KPropertyAnnotation`]) and the exprent is fetched
//! in the scope of the class being written.
//!
//! kuna fix `kt_type_parameter_id`: each member's types are read in the
//! scope of its own type parameters (`kuna_fix_kt_type_parameter_id`).
//!
//! kuna fix `kt_default_accessor_flags`: an accessor whose flags the metadata
//! leaves out takes its property's default accessor flags.

use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_INTERFACE, ACC_STATIC};
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::exprent::Exprent;
use crate::interpreter_util::InterpreterUtil;
use crate::k_const_exprent::KConstExprent;
use crate::k_element::KElement;
use crate::k_function::{modality_name, MethodSupplier};
use crate::k_hidden_element::KHiddenElement;
use crate::k_property_accessor::KPropertyAccessor;
use crate::k_type::KType;
use crate::k_types::KTypes;
use crate::k_utils::KUtils;
use crate::key::Key;
use crate::kotlin::kuna_fix_kt_default_accessor_flags;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::{self, Modality, Visibility};
use crate::kuna_fix_kt_type_parameter_id;
use crate::kuna_fixes::{self, KunaFix};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::rt::jstring::java_string_hash;
use crate::rt::{JavaEq, JavaHash};
use crate::struct_annotation_attribute::StructAnnotationAttribute;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_member::StructMember;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

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
    /// The annotation exprent, in the current class scope (built anew for `Deprecated`).
    pub fn get(&self) -> AnnotationExprent {
        match self {
            KPropertyAnnotation::Attribute(attribute, index) => {
                attribute.get_annotations()[*index as usize]
            }
            KPropertyAnnotation::Deprecated => KProperty::DEPRECATED_ANNOTATION(),
        }
    }

    /// Java identity of the exprent this entry stands for.
    fn same_as(&self, o: &KPropertyAnnotation) -> bool {
        match (self, o) {
            (KPropertyAnnotation::Attribute(a, i), KPropertyAnnotation::Attribute(b, j)) => {
                a == b && i == j
            }
            (KPropertyAnnotation::Deprecated, KPropertyAnnotation::Deprecated) => true,
            _ => false,
        }
    }

    fn identity_hash(&self) -> i32 {
        match self {
            KPropertyAnnotation::Attribute(attribute, index) => (attribute.index() as i32)
                .wrapping_mul(31)
                .wrapping_add(*index),
            KPropertyAnnotation::Deprecated => -1,
        }
    }

    /// `annotations.addAll(attribute.getAnnotations())`.
    fn add_all(annotations: &mut Vec<KPropertyAnnotation>, attribute: StructAnnotationAttribute) {
        let count = attribute.get_records().len() as i32;
        for index in 0..count {
            annotations.push(KPropertyAnnotation::Attribute(attribute, index));
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
    /// Java builds one at class initialisation; here each use builds its own, so
    /// no class-scope exprent outlives the write that prints it (no arena
    /// reset or reclamation can strand a cached handle). Only its text is used.
    #[allow(non_snake_case)]
    pub fn DEPRECATED_ANNOTATION() -> AnnotationExprent {
        let class_name = VarType::new_signature("kotlin/Deprecated")
            .value
            .as_deref()
            .expect("VarType value")
            .to_string();
        let par_names = vec!["message".to_string()];
        let par_values = vec![Exprent::from(ConstExprent::new(
            VarType::VARTYPE_STRING(),
            Some(ConstValue::String("Deprecated by attribute.".to_string())),
            None,
        ))];
        AnnotationExprent::new(class_name, par_names, par_values)
    }

    /// The record accessor `annotations()`, as exprents of the current class scope.
    pub fn get_annotations(&self) -> Option<Vec<AnnotationExprent>> {
        self.annotations
            .as_ref()
            .map(|annotations| annotations.iter().map(KPropertyAnnotation::get).collect())
    }

    pub fn stringify(&self, class_wrapper: ClassWrapper, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let flags = self.flags;

        if Flags::HAS_ANNOTATIONS.get(flags) {
            if let Some(annotations) = self.get_annotations() {
                for anno in annotations {
                    buf.append_indent(indent)
                        .append_buffer(&anno.0.to_java(indent))
                        .append_line_separator();
                }
            } else {
                buf.append_indent(indent)
                    .append("// $VF: failed to identify property annotations")
                    .append_line_separator();
            }
        }

        buf.append_indent(indent);

        KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(flags));

        if Flags::IS_EXPECT_PROPERTY.get(flags) {
            buf.append("expect ");
        }

        if Flags::MODALITY.get(flags) == Some(Modality::Final) {
            buf.append(if Flags::IS_CONST.get(flags) {
                "const "
            } else {
                "final "
            });
        } else if !self.class_struct.has_modifier(ACC_INTERFACE)
            || Flags::MODALITY.get(flags) != Some(Modality::Abstract)
        {
            buf.append(&modality_name(Flags::MODALITY.get(flags)))
                .append_char(' ');
        }

        if Flags::IS_EXTERNAL_PROPERTY.get(flags) {
            buf.append("external ");
        }

        if Flags::IS_LATEINIT.get(flags) {
            buf.append("lateinit ");
        }

        buf.append(if Flags::IS_VAR.get(flags) {
            "var "
        } else {
            "val "
        })
        .append(&KotlinWriter::to_valid_kotlin_identifier(&self.name))
        .append(": ");
        let type_ = self.type_.as_ref().expect("NullPointerException: type");
        buf.append_buffer(&type_.stringify(indent));

        if let Some(initializer) = &self.initializer {
            let expr = initializer(class_wrapper);
            if let Some(expr) = expr {
                let initializer_buf = expr.to_java(indent);
                if Flags::IS_DELEGATED.get(flags) {
                    buf.append(" by ").append_buffer(&initializer_buf);
                } else {
                    buf.append(" =")
                        .push_newline_group(indent, 1)
                        .append_possible_newline_alt(" ")
                        .append_buffer(&initializer_buf)
                        .pop_newline_group();
                }
            }
        }

        match &self.getter {
            Some(getter) if Flags::IS_NOT_DEFAULT.get(getter.flags) => {
                buf.push_newline_group(indent, 1)
                    .append_line_separator()
                    .append_indent(indent + 1);

                KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(getter.flags));

                buf.append(&modality_name(Flags::MODALITY.get(getter.flags)))
                    .append_char(' ');

                if Flags::IS_EXTERNAL_ACCESSOR.get(getter.flags) {
                    buf.append("external ");
                }

                if Flags::IS_INLINE_ACCESSOR.get(getter.flags) {
                    buf.append("inline ");
                }

                buf.append("get() ");

                let method_wrapper = (getter.method_supplier)(class_wrapper)
                    .expect("NullPointerException: methodWrapper");
                KotlinWriter::write_method_body(
                    self.class_struct,
                    method_wrapper,
                    &mut buf,
                    indent + 1,
                    false,
                );

                buf.pop_newline_group();
            }
            Some(getter) if Flags::IS_EXTERNAL_ACCESSOR.get(getter.flags) => {
                buf.append_line_separator()
                    .append_indent(indent + 1)
                    .append("external get");
            }
            _ => {}
        }

        match &self.setter {
            Some(setter) if Flags::IS_NOT_DEFAULT.get(setter.flags) => {
                buf.push_newline_group(indent, 1)
                    .append_line_separator()
                    .append_indent(indent + 1);

                KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(setter.flags));

                buf.append(&modality_name(Flags::MODALITY.get(setter.flags)))
                    .append_char(' ');

                if Flags::IS_EXTERNAL_ACCESSOR.get(setter.flags) {
                    buf.append("external ");
                }

                if Flags::IS_INLINE_ACCESSOR.get(setter.flags) {
                    buf.append("inline ");
                }

                buf.append("set(")
                    .append(self.setter_param_name.as_deref().unwrap_or("null"))
                    .append(") ");

                let method_wrapper = (setter.method_supplier)(class_wrapper)
                    .expect("NullPointerException: methodWrapper");
                KotlinWriter::write_method_body(
                    self.class_struct,
                    method_wrapper,
                    &mut buf,
                    indent + 1,
                    false,
                );

                buf.pop_newline_group();
            }
            Some(setter)
                if Flags::IS_EXTERNAL_ACCESSOR.get(setter.flags)
                    || Flags::VISIBILITY.get(setter.flags) != Flags::VISIBILITY.get(flags)
                    || Flags::MODALITY.get(setter.flags) != Flags::MODALITY.get(flags) =>
            {
                buf.append_line_separator().append_indent(indent + 1);

                if Flags::VISIBILITY.get(setter.flags) != Flags::VISIBILITY.get(flags) {
                    KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(setter.flags));
                }

                if Flags::MODALITY.get(setter.flags) != Flags::MODALITY.get(flags) {
                    buf.append(&modality_name(Flags::MODALITY.get(setter.flags)))
                        .append_char(' ');
                }

                if Flags::IS_EXTERNAL_ACCESSOR.get(setter.flags) {
                    buf.append("external ");
                }

                buf.append("set");
            }
            None if Flags::IS_VAR.get(flags)
                && Flags::VISIBILITY.get(flags) != Some(Visibility::Private) =>
            {
                buf.append_line_separator()
                    .append_indent(indent + 1)
                    .append("private set");
            }
            _ => {}
        }

        buf
    }

    pub fn parse(
        class_struct: StructClass,
        proto_properties: &[kt_proto::Property],
        name_resolver: Option<&MetadataNameResolver>,
        companion_parent: Option<StructClass>,
    ) {
        let Some(name_resolver) = name_resolver else {
            return;
        };

        for property in proto_properties {
            let _type_parameters = kuna_fixes::enabled(KunaFix::KtTypeParameterId).then(|| {
                kuna_fix_kt_type_parameter_id::member_scope(
                    &property.get_type_parameter_list(),
                    name_resolver,
                )
            });
            let flags = property.get_flags();

            let jvm_prop = property.get_extension_property_signature();

            let mut annotations: Vec<KPropertyAnnotation> = Vec::new();
            if jvm_prop.has_synthetic_method() {
                let synthetic_method = jvm_prop.get_synthetic_method();
                let method_name = name_resolver.resolve(synthetic_method.get_name());
                let desc = name_resolver.resolve(synthetic_method.get_desc());
                let method = class_struct.get_method(&method_name, &desc);
                if let Some(method) = method {
                    KtAttrs::put(
                        method,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::Generic),
                    );
                    add_member_annotations(&mut annotations, method.into());
                }
            }

            let name = name_resolver.resolve(property.get_name());

            let mut prop_desc: Option<String> = None;
            let mut type_: Option<KType> = None;
            if property.has_return_type() {
                let return_type = KType::from(&property.get_return_type(), name_resolver);
                prop_desc = Some(KTypes::get_java_signature(
                    &return_type.k().kotlin_type,
                    property.get_return_type().get_nullable(),
                ));
                type_ = Some(return_type);
            }

            let mut delegate_supplier: Option<ExprentSupplier> = None;
            let mut delegate_field: Option<StructField> = None;
            if Flags::IS_DELEGATED.get(flags) {
                let delegate_field_name = name_resolver.resolve(jvm_prop.get_field().get_name());
                let delegate_desc = name_resolver.resolve(jvm_prop.get_field().get_desc());
                delegate_field = class_struct.get_field(&delegate_field_name, &delegate_desc);
                if let Some(delegate_field) = delegate_field {
                    KtAttrs::put(
                        delegate_field,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::Generic),
                    );
                    let key =
                        InterpreterUtil::make_unique_key(&delegate_field_name, &delegate_desc);
                    delegate_supplier = Some(field_initializer(
                        delegate_field.has_modifier(ACC_STATIC),
                        key,
                    ));
                }
            }

            let mut getter: Option<KPropertyAccessor> = None;
            let mut getter_method = None;
            if Flags::HAS_GETTER.get(flags) {
                let method_name = name_resolver.resolve(jvm_prop.get_getter().get_name());
                let desc = name_resolver.resolve(jvm_prop.get_getter().get_desc());
                getter_method = class_struct.get_method(&method_name, &desc);
                if let Some(getter_method) = getter_method {
                    let supplier = method_supplier(method_name.clone(), desc.clone());
                    let getter_flags = if kuna_fixes::enabled(KunaFix::KtDefaultAccessorFlags) {
                        kuna_fix_kt_default_accessor_flags::accessor_flags(
                            property.has_getter_flags(),
                            property.get_getter_flags(),
                            flags,
                        )
                    } else {
                        property.get_getter_flags()
                    };
                    getter = Some(KPropertyAccessor::new(getter_flags, supplier));

                    if prop_desc.is_none() {
                        let descriptor = getter_method.get_descriptor();
                        let start = descriptor.find(')').map_or(0, |i| i + 1);
                        prop_desc = Some(descriptor[start..].to_string());
                    }
                }

                if let Some(companion_parent) = companion_parent {
                    if let Some(in_parent) = companion_parent.get_method(&method_name, &desc) {
                        KtAttrs::put(
                            in_parent,
                            &KElement::KEY(),
                            KElement::HiddenElement(KHiddenElement::CompanionItem),
                        );
                    }
                }
            }

            let mut setter: Option<KPropertyAccessor> = None;
            let mut setter_method = None;
            let mut setter_param_name: Option<String> = None;
            if Flags::HAS_SETTER.get(flags) {
                let method_name = name_resolver.resolve(jvm_prop.get_setter().get_name());
                let desc = name_resolver.resolve(jvm_prop.get_setter().get_desc());
                setter_method = class_struct.get_method(&method_name, &desc);
                if setter_method.is_some() {
                    let supplier = method_supplier(method_name.clone(), desc.clone());
                    let setter_flags = if kuna_fixes::enabled(KunaFix::KtDefaultAccessorFlags) {
                        kuna_fix_kt_default_accessor_flags::accessor_flags(
                            property.has_setter_flags(),
                            property.get_setter_flags(),
                            flags,
                        )
                    } else {
                        property.get_setter_flags()
                    };
                    setter = Some(KPropertyAccessor::new(setter_flags, supplier));
                    setter_param_name = Some(
                        name_resolver.resolve(property.get_setter_value_parameter().get_name()),
                    );
                }

                if let Some(companion_parent) = companion_parent {
                    if let Some(in_parent) = companion_parent.get_method(&method_name, &desc) {
                        KtAttrs::put(
                            in_parent,
                            &KElement::KEY(),
                            KElement::HiddenElement(KHiddenElement::CompanionItem),
                        );
                    }
                }
            }

            let mut field: Option<StructField> = None;
            let mut field_container = class_struct;
            if jvm_prop.has_field() {
                let field_name = if jvm_prop.get_field().has_name() {
                    name_resolver.resolve(jvm_prop.get_field().get_name())
                } else {
                    name.clone()
                };
                let field_desc = if jvm_prop.get_field().has_desc() {
                    Some(name_resolver.resolve(jvm_prop.get_field().get_desc()))
                } else {
                    prop_desc.clone()
                };

                if let Some(field_desc) = field_desc {
                    field = class_struct.get_field(&field_name, &field_desc);

                    if let Some(companion_parent) = companion_parent {
                        let in_parent = companion_parent.get_field(&field_name, &field_desc);
                        if let Some(in_parent) = in_parent {
                            KtAttrs::put(
                                in_parent,
                                &KElement::KEY(),
                                KElement::HiddenElement(KHiddenElement::CompanionItem),
                            );

                            if field.is_none() {
                                field = Some(in_parent);
                                field_container = companion_parent;
                            }
                        }
                    }
                } else {
                    let fields = class_struct.get_fields();
                    for f in fields.to_vec() {
                        if f.get_name() == name {
                            field = Some(f);
                            prop_desc = Some(f.get_descriptor());
                            break;
                        }
                    }
                }
            }

            if Flags::HAS_ANNOTATIONS.get(flags) && annotations.is_empty() {
                if let Some(field) = field {
                    add_member_annotations(&mut annotations, field.into());
                    if field.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED) {
                        annotations.push(KPropertyAnnotation::Deprecated);
                    }
                }
            }

            let var_type = match &prop_desc {
                Some(prop_desc) => VarType::new_signature(prop_desc),
                None => VarType::VARTYPE_OBJECT(),
            };

            let key = InterpreterUtil::make_unique_key(&name, &var_type.to_string());
            let initializer: Option<ExprentSupplier>;

            if delegate_supplier.is_some() {
                initializer = delegate_supplier;
                field = delegate_field;
            } else if let Some(f) = field {
                if let Some(attr) =
                    f.get_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE)
                {
                    let constant = field_container
                        .get_pool()
                        .get_primitive_constant(attr.get_index());
                    let const_type: VarType = match &type_ {
                        Some(type_) => type_.0.clone(),
                        None => var_type.clone(),
                    };
                    initializer = Some(Rc::new(move |_ignored: ClassWrapper| {
                        let constant = constant
                            .as_ref()
                            .expect("NullPointerException: ConstantValue constant");
                        let value = constant.value.as_ref().map(Into::into);
                        Some(Exprent::from(KConstExprent::new(ConstExprent::new(
                            const_type.clone(),
                            value,
                            None,
                        ))))
                    }));
                } else {
                    initializer = Some(field_initializer(f.has_modifier(ACC_STATIC), key));
                }
            } else {
                initializer = None;
            }

            let kprop = Rc::new(KProperty::new(
                name,
                type_,
                flags,
                getter,
                setter,
                setter_param_name,
                field,
                initializer,
                Some(annotations),
                class_struct,
            ));

            if let Some(field) = field {
                KtAttrs::put(field, &KElement::KEY(), KElement::Property(kprop.clone()));
            }
            if let Some(getter_method) = getter_method {
                KtAttrs::put(
                    getter_method,
                    &KElement::KEY(),
                    KElement::Property(kprop.clone()),
                );
                KtAttrs::put(
                    getter_method,
                    &PropertyMethod::KEY(),
                    PropertyMethod::Getter,
                );
            }
            if let Some(setter_method) = setter_method {
                KtAttrs::put(
                    setter_method,
                    &KElement::KEY(),
                    KElement::Property(kprop.clone()),
                );
                KtAttrs::put(
                    setter_method,
                    &PropertyMethod::KEY(),
                    PropertyMethod::Setter,
                );
            }

            if class_struct != field_container {
                if let Some(field) = field {
                    let key = InterpreterUtil::make_unique_key(
                        &field.get_name(),
                        &field.get_descriptor(),
                    );
                    class_struct.add_field_with_key(field, key);
                }
                if let Some(getter_method) = getter_method {
                    let key = InterpreterUtil::make_unique_key(
                        &getter_method.get_name(),
                        &getter_method.get_descriptor(),
                    );
                    class_struct.add_method_with_key(getter_method, key);
                }
                if let Some(setter_method) = setter_method {
                    let key = InterpreterUtil::make_unique_key(
                        &setter_method.get_name(),
                        &setter_method.get_descriptor(),
                    );
                    class_struct.add_method_with_key(setter_method, key);
                }
            }
        }
    }

    /// The record's `equals`: the suppliers compare by identity, and so do the
    /// annotations (each entry is one exprent object per class scope).
    pub fn equals(&self, o: &KProperty) -> bool {
        self.name == o.name
            && match (&self.type_, &o.type_) {
                (None, None) => true,
                (Some(a), Some(b)) => a.equals(b),
                _ => false,
            }
            && self.flags == o.flags
            && accessor_equals(&self.getter, &o.getter)
            && accessor_equals(&self.setter, &o.setter)
            && self.setter_param_name == o.setter_param_name
            && self.underlying_field == o.underlying_field
            && match (&self.initializer, &o.initializer) {
                (None, None) => true,
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                _ => false,
            }
            && match (&self.annotations, &o.annotations) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.same_as(y))
                }
                _ => false,
            }
            && self.class_struct == o.class_struct
    }

    /// The record's `hashCode`; identity hashes are arena indices and addresses.
    pub fn hash_code(&self) -> i32 {
        let mut result: i32 = 0;
        result = result
            .wrapping_mul(31)
            .wrapping_add(java_string_hash(&self.name));
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.type_.as_ref().map_or(0, |t| t.hash_code()));
        result = result.wrapping_mul(31).wrapping_add(self.flags);
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.getter.as_ref().map_or(0, KPropertyAccessor::hash_code));
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.setter.as_ref().map_or(0, KPropertyAccessor::hash_code));
        result = result.wrapping_mul(31).wrapping_add(
            self.setter_param_name
                .as_deref()
                .map_or(0, java_string_hash),
        );
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.underlying_field.map_or(0, |f| f.index() as i32));
        result = result.wrapping_mul(31).wrapping_add(
            self.initializer
                .as_ref()
                .map_or(0, |i| Rc::as_ptr(i) as *const () as usize as i32),
        );
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.annotations.as_ref().map_or(0, |annotations| {
                let mut hash: i32 = 1;
                for annotation in annotations {
                    hash = hash
                        .wrapping_mul(31)
                        .wrapping_add(annotation.identity_hash());
                }
                hash
            }));
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.class_struct.index() as i32);
        result
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

fn accessor_equals(a: &Option<KPropertyAccessor>, b: &Option<KPropertyAccessor>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    }
}

/// The member's runtime visible, then invisible, annotations.
fn add_member_annotations(annotations: &mut Vec<KPropertyAnnotation>, member: StructMember) {
    if member.has_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS) {
        let attribute = member
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS)
            .expect("RuntimeVisibleAnnotations");
        KPropertyAnnotation::add_all(annotations, attribute);
    }
    if member.has_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS) {
        let attribute = member
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS)
            .expect("RuntimeInvisibleAnnotations");
        KPropertyAnnotation::add_all(annotations, attribute);
    }
}

/// `wrapper -> wrapper.getMethodWrapper(methodName, desc)`.
fn method_supplier(method_name: String, desc: String) -> MethodSupplier {
    Rc::new(move |wrapper: ClassWrapper| wrapper.get_method_wrapper(&method_name, &desc))
}

/// `wrapper -> wrapper.get{Static,Dynamic}FieldInitializers().getWithKey(key)`.
fn field_initializer(is_static: bool, key: String) -> ExprentSupplier {
    if is_static {
        Rc::new(move |wrapper: ClassWrapper| {
            wrapper
                .get_static_field_initializers()
                .borrow()
                .get_with_key(&key)
        })
    } else {
        Rc::new(move |wrapper: ClassWrapper| {
            wrapper
                .get_dynamic_field_initializers()
                .borrow()
                .get_with_key(&key)
        })
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
