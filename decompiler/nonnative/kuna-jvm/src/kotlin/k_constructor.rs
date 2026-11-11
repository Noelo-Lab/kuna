//! Port of `org/vineflower/kotlin/struct/KConstructor.java`.
//!
//! kuna fix `kt_default_args_call` (`kuna_fixes`): a secondary constructor's
//! delegation through a default-arguments constructor is resugared
//! (`kuna_fix_kt_default_args_call::resugar_delegation`).
//!
//! kuna fix `kt_default_args_super_call` (`kuna_fixes`): in
//! `write_primary_constructor`, a supertype call through a default-arguments
//! constructor is written as a call of the constructor it stands for
//! (`kuna_fix_kt_default_args_super_call`), where Vineflower removes only the
//! marker argument and names a constructor that does not exist.
//!
//! kuna fix `kt_vararg_from_metadata`: in `parse`, a parameter gets a vararg
//! element type only when its metadata has one; Vineflower gives every
//! constructor parameter one, so every array parameter printed as `vararg`.
//!
//! kuna fix `kt_default_args_ctor_descriptor`: in `parse`, the
//! default-arguments constructor's descriptor is the constructor's own
//! parameters, then the masks and the marker; Vineflower spells the
//! parameters from their Kotlin types (and the enum prefix), which misses an
//! inner class's outer instance, so its defaults were never found.
//!
//! kuna fix `kt_abstract_class_constructor`: `parse` reads an abstract
//! class's constructors too; Vineflower returns before them, so they were
//! written as functions named after the class.
//!
//! kuna fix `kt_constructor_body_after_first_block`: in `stringify`, a
//! constructor is empty only when its whole body writes nothing
//! (`kuna_fix_kt_constructor_body_after_first_block`).
//!
//! kuna fix `kt_inline_class_constructor_accessor`: `parse` takes the
//! constructor an inline-class parameter's synthetic accessor forwards to
//! (`kuna_fix_kt_inline_class_constructor_accessor`).
//!
//! kuna fix `kt_constructor_default_signature`: `parse` computes a
//! constructor's JVM descriptor from its parameters when the metadata leaves
//! it out (`object Unit`), where Vineflower resolves the absent index 0.
//!
//! kuna fix `kt_secondary_constructor_line`: in `stringify`, a secondary
//! constructor without a body ends its line, where Vineflower leaves the
//! next member or the class's closing brace on it (`: this(n, "n")}`).
//!
//! kuna fix `kt_supertype_type_arguments`: `write_primary_constructor` writes
//! the superclass call's class with its type arguments from the metadata
//! (`kuna_fix_kt_supertype_type_arguments`).
//!
//! kuna fix `kt_hidden_empty_super_call`: a constructor whose empty
//! superclass call `hideEmptySuper` removed writes that call: `()` in the
//! primary constructor's header, nothing in a secondary one (where Vineflower
//! takes the next statement for the call, or drops an empty constructor).
//!
//! kuna fix `kt_empty_primary_constructor`: `write_primary_constructor`
//! writes `()` for a primary constructor without parameters when the class
//! has secondary constructors; without it Kotlin reads the class as having no
//! primary constructor, and a secondary one's `this()` names none.

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::decompiler_context::DecompilerContext;
use crate::default_args_map::{DefaultArgsMap, DefaultArgsSupplier};
use crate::direct_node::DirectNode;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::k_element::KElement;
use crate::k_function::{self, MethodSupplier};
use crate::k_hidden_element::KHiddenElement;
use crate::k_parameter::KParameter;
use crate::k_type::KType;
use crate::k_utils::KUtils;
use crate::key::Key;
use crate::kotlin::kuna_fix_kt_constructor_body_after_first_block;
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kotlin::kuna_fix_kt_default_args_super_call;
use crate::kotlin::kuna_fix_kt_hidden_empty_super_call;
use crate::kotlin::kuna_fix_kt_inline_class_constructor_accessor;
use crate::kotlin::kuna_fix_kt_supertype_type_arguments;
use crate::kuna_fix_kt_java_reference_cast;
use crate::kuna_fixes::{self, KunaFix};
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::{self, ClassKind, Modality, Visibility};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::root_statement::RootStatement;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::type_annotation::TypeAnnotation;
use crate::var_type::VarType;

#[derive(Clone)]
pub struct KConstructor {
    pub parameters: Rc<[KParameter]>,
    pub flags: i32,
    /// `null` for an annotation class's constructor (it has no `<init>`).
    pub method_supplier: Option<MethodSupplier>,
    pub is_primary: bool,
    pub default_args_supplier: DefaultArgsSupplier,
    pub class_struct: StructClass,
    /// `null` for an annotation class's constructor.
    pub method_struct: Option<StructMethod>,
    pub class_flags: i32,
}

thread_local! {
    static DEFAULT_CONSTRUCTOR_MARKER: VarType =
        VarType::new_signature_cl("kotlin/jvm/internal/DefaultConstructorMarker", true);
}

impl KConstructor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parameters: Rc<[KParameter]>,
        flags: i32,
        method_supplier: Option<MethodSupplier>,
        is_primary: bool,
        default_args_supplier: DefaultArgsSupplier,
        class_struct: StructClass,
        method_struct: Option<StructMethod>,
        class_flags: i32,
    ) -> KConstructor {
        KConstructor {
            parameters,
            flags,
            method_supplier,
            is_primary,
            default_args_supplier,
            class_struct,
            method_struct,
            class_flags,
        }
    }

    /// `KConstructor.PRIMARY_CONSTRUCTOR`: the class attribute holding its
    /// primary constructor, the same object as the `KElement` of its `<init>`.
    #[allow(non_snake_case)]
    pub fn PRIMARY_CONSTRUCTOR() -> Key<Rc<KConstructor>> {
        Key::of("primary-constructor")
    }

    #[allow(non_snake_case)]
    fn DEFAULT_CONSTRUCTOR_MARKER() -> VarType {
        DEFAULT_CONSTRUCTOR_MARKER.with(VarType::clone)
    }

    pub fn parse(
        class_struct: StructClass,
        proto_class: &kt_proto::Class,
        resolver: Option<&MetadataNameResolver>,
    ) {
        let Some(resolver) = resolver else {
            return;
        };

        let class_flags = proto_class.get_flags();
        if Flags::MODALITY.get(class_flags) == Some(Modality::Abstract)
            && !kuna_fixes::enabled(KunaFix::KtAbstractClassConstructor)
        {
            return;
        }

        let proto_constructors = proto_class.get_constructor_list();
        if proto_constructors.is_empty() {
            return;
        }

        for constructor in &proto_constructors {
            let mut parameters: Vec<KParameter> =
                Vec::with_capacity(constructor.get_value_parameter_count() as usize);
            for i in 0..constructor.get_value_parameter_count() {
                let proto_parameter = constructor.get_value_parameter(i);
                let flags = proto_parameter.get_flags();
                let name = resolver.resolve(proto_parameter.get_name());
                let type_ = KType::from(&proto_parameter.get_type(), resolver);
                let vararg_type = if !kuna_fixes::enabled(KunaFix::KtVarargFromMetadata)
                    || proto_parameter.has_vararg_element_type()
                {
                    Some(KType::from(&proto_parameter.get_vararg_element_type(), resolver))
                } else {
                    None
                };
                let type_id = proto_parameter.get_type_id();
                parameters.push(KParameter::new(flags, name, type_, vararg_type, type_id));
            }
            let parameters: Rc<[KParameter]> = Rc::from(parameters);

            let flags = constructor.get_flags();

            let signature = constructor.get_extension_constructor_signature();
            let desc = if kuna_fixes::enabled(KunaFix::KtConstructorDefaultSignature)
                && !signature.has_desc()
            {
                let mut desc = String::from("(");
                for parameter in parameters.iter() {
                    desc.push_str(&parameter.type_.0.to_string());
                }
                desc + ")V"
            } else {
                resolver.resolve(signature.get_desc())
            };
            let Some(method) = class_struct.get_method("<init>", &desc) else {
                if Flags::CLASS_KIND.get(class_flags) == Some(ClassKind::AnnotationClass) {
                    let k_constructor = KConstructor::new(
                        parameters,
                        flags,
                        None,
                        true,
                        Rc::new(move |_ignored: ClassWrapper| {
                            DefaultArgsMap::from_annotation(class_struct)
                        }),
                        class_struct,
                        None,
                        class_flags,
                    );
                    KtAttrs::put(
                        class_struct,
                        &KConstructor::PRIMARY_CONSTRUCTOR(),
                        Rc::new(k_constructor),
                    );
                    return;
                }

                DecompilerContext::get_logger().write_message(
                    &format!(
                        "Method <init>{} not found in {}",
                        desc,
                        class_struct.qualified_name()
                    ),
                    Severity::Warn,
                );
                continue;
            };

            let method = match kuna_fixes::enabled(KunaFix::KtInlineClassConstructorAccessor)
                .then(|| {
                    kuna_fix_kt_inline_class_constructor_accessor::real_constructor(
                        class_struct,
                        method,
                    )
                })
                .flatten()
            {
                Some(real) => {
                    KtAttrs::put(
                        method,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::Generic),
                    );
                    real
                }
                None => method,
            };

            let is_primary = !Flags::IS_SECONDARY.get(flags);

            let mut default_args_desc = String::from("(");
            if kuna_fixes::enabled(KunaFix::KtDefaultArgsCtorDescriptor) {
                for param in method.method_descriptor().params.iter() {
                    default_args_desc.push_str(&param.to_string());
                }
            } else {
                if Flags::CLASS_KIND.get(class_flags) == Some(ClassKind::EnumClass) {
                    default_args_desc.push_str("Ljava/lang/String;");
                    default_args_desc.push('I');
                }

                for parameter in parameters.iter() {
                    default_args_desc.push_str(&parameter.type_.0.to_string());
                }
            }

            default_args_desc.push_str(&"I".repeat(parameters.len() / 32 + 1));
            default_args_desc.push_str("Lkotlin/jvm/internal/DefaultConstructorMarker;)V");

            let default_method = class_struct.get_method("<init>", &default_args_desc);
            if let Some(default_method) = default_method {
                KtAttrs::put(
                    default_method,
                    &KElement::KEY(),
                    KElement::HiddenElement(KHiddenElement::DefaultImpl),
                );
            }

            let default_args_supplier: DefaultArgsSupplier = {
                let parameters = parameters.clone();
                Rc::new(move |wrapper: ClassWrapper| {
                    DefaultArgsMap::from(
                        wrapper.get_method_wrapper("<init>", &default_args_desc),
                        wrapper.get_method_wrapper(&method.get_name(), &method.get_descriptor()),
                        &parameters,
                    )
                })
            };

            let method_supplier: MethodSupplier = Rc::new(move |wrapper: ClassWrapper| {
                wrapper.get_method_wrapper(&method.get_name(), &method.get_descriptor())
            });

            let k_constructor = Rc::new(KConstructor::new(
                parameters,
                flags,
                Some(method_supplier),
                is_primary,
                default_args_supplier,
                class_struct,
                Some(method),
                class_flags,
            ));
            KtAttrs::put(
                method,
                &KElement::KEY(),
                KElement::Constructor(k_constructor.clone()),
            );

            if is_primary {
                KtAttrs::put(
                    class_struct,
                    &KConstructor::PRIMARY_CONSTRUCTOR(),
                    k_constructor,
                );
            }
        }
    }

    fn should_hide_constructor(&self) -> bool {
        if !self.is_primary
            || !self.parameters.is_empty()
            || !DecompilerContext::get_option(IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR)
        {
            return false;
        }

        if Flags::VISIBILITY.get(self.flags) != Flags::VISIBILITY.get(self.class_flags)
            && Flags::CLASS_KIND.get(self.class_flags) != Some(ClassKind::EnumClass)
        {
            return false;
        }

        true
    }

    /// Whether the metadata gives the class a secondary constructor.
    fn has_secondary_constructor(&self) -> bool {
        self.class_struct.get_methods().to_vec().into_iter().any(|mt| {
            matches!(KtAttrs::get(mt, &KElement::KEY()),
                Some(KElement::Constructor(c)) if !c.is_primary)
        })
    }

    /// `methodSupplier.apply(wrapper)`, dereferenced like the Java does.
    fn method_wrapper(&self, wrapper: ClassWrapper) -> MethodWrapper {
        let supplier = self
            .method_supplier
            .as_ref()
            .expect("NullPointerException: methodSupplier");
        supplier(wrapper).expect("NullPointerException: methodWrapper")
    }

    /// `methodWrapper.getOrBuildGraph().first`.
    fn first_node(method_wrapper: MethodWrapper) -> DirectNode {
        method_wrapper
            .get_or_build_graph()
            .expect("NullPointerException: graph")
            .first()
            .expect("NullPointerException: graph.first")
    }

    /// Appends each parameter and its default value, as both writers do.
    fn append_parameters(
        &self,
        wrapper: ClassWrapper,
        buf: &mut TextBuffer,
        indent: i32,
        method_key: &str,
    ) {
        let qualified_name = self.class_struct.qualified_name();
        let mut first = true;
        for parameter in self.parameters.iter() {
            if !first {
                buf.append(",").append_possible_newline_alt(" ");
            }

            first = false;

            parameter.stringify(indent + 1, buf);

            if Flags::DECLARES_DEFAULT_VALUE.get(parameter.flags) {
                let default_value =
                    (self.default_args_supplier)(wrapper).to_java(parameter, indent + 1);
                buf.append_buffer_mapped(&default_value, Some(&qualified_name), Some(method_key));
            }
        }
    }

    pub fn stringify(&self, wrapper: ClassWrapper, buffer: &mut TextBuffer, indent: i32) -> bool {
        if self.should_hide_constructor() {
            return false;
        }

        let mut buf = TextBuffer::new();
        let method_wrapper = self.method_wrapper(wrapper);
        let (root, wrapper_method, wrapper_class) =
            method_wrapper.with(|d| (d.root, d.method_struct, d.class_struct));

        let method_key = InterpreterUtil::make_unique_key(
            &wrapper_method.get_name(),
            &wrapper_method.get_descriptor(),
        );

        if !self.is_primary {
            if Flags::HAS_ANNOTATIONS.get(self.flags) {
                KotlinWriter::append_annotations(
                    &mut buf,
                    indent,
                    wrapper_method.into(),
                    TypeAnnotation::METHOD_RETURN_TYPE,
                );
                KotlinWriter::append_jvm_annotations(
                    &mut buf,
                    indent,
                    wrapper_method.into(),
                    false,
                    false,
                    wrapper_class.get_pool(),
                    TypeAnnotation::METHOD_RETURN_TYPE,
                );
            }

            buf.append_indent(indent);

            if Flags::VISIBILITY.get(self.flags) != Some(Visibility::Public)
                || DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY)
            {
                KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(self.flags));
            }

            buf.append("constructor");

            buf.append("(").push_newline_group(indent, 1);

            self.append_parameters(wrapper, &mut buf, indent, &method_key);

            buf.append_possible_newline_dedent("", true)
                .pop_newline_group();

            let method_struct = self
                .method_struct
                .expect("NullPointerException: methodStruct");
            let method_descriptor = format!(
                "{}{}",
                method_struct.get_name(),
                method_struct.get_descriptor()
            );
            let containing_class = self.class_struct.qualified_name();

            let exprents = KConstructor::first_node(method_wrapper).exprents();
            let hidden_super = kuna_fixes::enabled(KunaFix::KtHiddenEmptySuperCall)
                && kuna_fix_kt_hidden_empty_super_call::hidden_call(method_wrapper).is_some();
            if hidden_super {
                buf.append(")");
            } else {
                if exprents.is_empty() {
                    DecompilerContext::get_logger().write_message(
                        &format!(
                            "Unexpected empty constructor body in {} {}",
                            containing_class, method_descriptor
                        ),
                        Severity::Warn,
                    );
                    return true;
                }

                buf.append(") ");

                let first_expr = exprents.get(0);
                if !first_expr.is_invocation() {
                    DecompilerContext::get_logger().write_message(
                        &format!(
                            "Unexpected missing super/this constructor call in {} {}",
                            containing_class, method_descriptor
                        ),
                        Severity::Warn,
                    );
                } else {
                    buf.append(": ");

                    if kuna_fixes::enabled(KunaFix::KtDefaultArgsCall) {
                        if let Some(invocation) = first_expr.try_as_invocation() {
                            kuna_fix_kt_default_args_call::resugar_delegation(invocation);
                        }
                    }
                    buf.append_buffer_mapped(
                        &first_expr.to_java(indent + 1),
                        Some(&containing_class),
                        Some(&method_key),
                    );

                    KConstructor::first_node(method_wrapper)
                        .exprents()
                        .remove_at(0);
                }
            }
        }

        let mut rendered = None;
        if let Some(root) = root {
            if KConstructor::first_node(method_wrapper)
                .exprents()
                .is_empty()
                && kuna_fixes::enabled(KunaFix::KtConstructorBodyAfterFirstBlock)
            {
                rendered = kuna_fix_kt_constructor_body_after_first_block::body(|| {
                    self.render_body(root, indent)
                });
            }
        }

        if rendered.is_none()
            && KConstructor::first_node(method_wrapper)
                .exprents()
                .is_empty()
        {
            if self.is_primary {
                return false;
            }

            if kuna_fixes::enabled(KunaFix::KtSecondaryConstructorLine) {
                buf.append_line_separator();
            }
            buffer.append_buffer(&buf);
            return true;
        }

        if self.is_primary {
            buf.append_indent(indent).append("init");
        }

        buf.append(" {").append_line_separator();

        let root = root.expect("NullPointerException: root");
        let mut body = rendered.unwrap_or_else(|| self.render_body(root, indent));
        body.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());

        let qualified_name = self.class_struct.qualified_name();
        buf.append_buffer_mapped(&body, Some(&qualified_name), Some(&method_key));

        buf.append_indent(indent)
            .append("}")
            .append_line_separator();

        buffer.append_buffer(&buf);
        true
    }

    /// `root.toJava(indent + 1)`, with the parameters' nullability in scope.
    fn render_body(&self, root: RootStatement, indent: i32) -> TextBuffer {
        kuna_fix_kt_java_reference_cast::with_nullable_parameters(
            k_function::nullable_parameters(&self.parameters),
            || root.to_java(indent + 1),
        )
    }

    pub fn write_primary_constructor(
        &self,
        wrapper: ClassWrapper,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        if !self.is_primary {
            return false;
        }

        let mut buf = TextBuffer::new();
        let mut appended = false;

        let method_struct = self
            .method_struct
            .expect("NullPointerException: methodStruct");
        let method_key = InterpreterUtil::make_unique_key(
            &method_struct.get_name(),
            &method_struct.get_descriptor(),
        );

        let class_kind = Flags::CLASS_KIND.get(self.class_flags);
        if class_kind != Some(ClassKind::Object) && class_kind != Some(ClassKind::CompanionObject) {
            if Flags::HAS_ANNOTATIONS.get(self.flags) {
                buf.append(" ");
                KotlinWriter::append_annotations(
                    &mut buf,
                    -1,
                    method_struct.into(),
                    TypeAnnotation::METHOD_RETURN_TYPE,
                );
                KotlinWriter::append_jvm_annotations(
                    &mut buf,
                    -1,
                    method_struct.into(),
                    false,
                    false,
                    self.class_struct.get_pool(),
                    TypeAnnotation::METHOD_RETURN_TYPE,
                );
                appended = true;
            }

            if (Flags::VISIBILITY.get(self.flags) != Some(Visibility::Public)
                || (appended
                    && DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY)))
                && class_kind != Some(ClassKind::EnumClass)
            {
                buf.append(" ");
                KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(self.flags));
                appended = true;
            }

            if appended {
                buf.append("constructor");
            }

            let secondary = kuna_fixes::enabled(KunaFix::KtEmptyPrimaryConstructor)
                && self.has_secondary_constructor();
            if !self.parameters.is_empty() || appended || secondary {
                buf.append("(").push_newline_group(indent, 1);

                self.append_parameters(wrapper, &mut buf, indent, &method_key);

                buf.append_possible_newline_dedent("", true)
                    .pop_newline_group()
                    .append(")");
            }
        }

        let method_wrapper = self.method_wrapper(wrapper);

        let hidden = kuna_fixes::enabled(KunaFix::KtHiddenEmptySuperCall)
            .then(|| kuna_fix_kt_hidden_empty_super_call::hidden_call(method_wrapper))
            .flatten();
        let invocation = match hidden {
            Some(invocation) => invocation,
            None => {
                if KConstructor::first_node(method_wrapper)
                    .exprents()
                    .is_empty()
                {
                    buffer.append_buffer(&buf);
                    return false;
                }

                let first_expr = KConstructor::first_node(method_wrapper).exprents().get(0);
                match first_expr.try_as_invocation() {
                    Some(invocation) if invocation.get_name() == "<init>" => invocation,
                    _ => {
                        buffer.append_buffer(&buf);
                        return false;
                    }
                }
            }
        };

        if invocation.get_classname() == "java/lang/Object"
            || class_kind == Some(ClassKind::EnumClass)
        {
            buffer.append_buffer(&buf);
            return false;
        }

        let imports = DecompilerContext::get_import_collector();
        let super_class = imports.get_short_name(&invocation.get_classname().replace('/', "."));
        buf.append(" : ");

        buf.append(super_class.as_deref().unwrap_or("null"));
        if kuna_fixes::enabled(KunaFix::KtSupertypeTypeArguments) {
            if let Some(arguments) = kuna_fix_kt_supertype_type_arguments::type_arguments(
                self.class_struct,
                &invocation.get_classname(),
                indent,
            ) {
                buf.append_buffer(&arguments);
            }
        }
        buf.append_char('(');

        if hidden.is_none() {
            if !(kuna_fixes::enabled(KunaFix::KtDefaultArgsSuperCall)
                && kuna_fix_kt_default_args_super_call::resugar(invocation))
            {
                KUtils::remove_arguments(invocation, &KConstructor::DEFAULT_CONSTRUCTOR_MARKER());
            }

            buf.append_buffer(&invocation.append_param_list(indent + 1));
        }
        buf.append(")");

        buf.add_bytecode_mapping(invocation.get_bytecode().as_ref());

        if hidden.is_none() {
            KConstructor::first_node(method_wrapper)
                .exprents()
                .remove_at(0);
        }

        let qualified_name = self.class_struct.qualified_name();
        buffer.append_buffer_mapped(&buf, Some(&qualified_name), Some(&method_key));
        true
    }
}
