//! Port of `org/vineflower/kotlin/KotlinWriter.java`.
//!
//! A [`Caught`] has no stack trace or cause: `collectErrorLines` is
//! `ClassWriter::collect_error_lines` (its throw location is the only frame),
//! so `ERROR_DUMP_STOP_POINTS` is never consulted.

use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::cancelation_manager::{CanceledException, MethodDeadlineExceeded};
use crate::class_reference14_processor::ClassReference14Processor;
use crate::class_wrapper::ClassWrapper;
use crate::class_writer::{ClassWriter, IFabricJavadocProvider};
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{
    ACC_ABSTRACT, ACC_ANNOTATION, ACC_BRIDGE, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_NATIVE,
    ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED,
    ACC_SYNTHETIC, ACC_TRANSIENT, ACC_VARARGS, ACC_VOLATILE, CLINIT_NAME, CONSTANT_CLASS,
    CONSTANT_DYNAMIC, CONSTANT_STRING, GROUP_FIELDACCESS, GROUP_INVOCATION, GROUP_JUMP, INIT_NAME,
    OPC_CHECKCAST, OPC_INSTANCEOF, OPC_INVOKEDYNAMIC, OPC_LDC, OPC_LDC2_W, OPC_LDC_W, OPC_NEW,
};
use crate::const_exprent::ConstExprent;
use crate::constant_pool::ConstantPool;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::generics_checker::GenericsChecker;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::import_collector::ImportCollector;
use crate::initializer_processor::InitializerProcessor;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::k_annotation_exprent::{KAnnotationExprent, UseSiteTarget};
use crate::k_class::KClass;
use crate::k_constructor::KConstructor;
use crate::k_element::KElement;
use crate::k_enum_processor::KEnumProcessor;
use crate::k_function::KFunction;
use crate::k_property::KProperty;
use crate::k_type::KType;
use crate::k_types::KTypes;
use crate::k_utils::KUtils;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_import_collector::KotlinImportCollector;
use crate::kotlin_object_processors::KotlinObjectProcessors;
use crate::kotlin_options::KotlinOptions;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::{ClassKind, Modality, Visibility};
use crate::kuna_spans;
use crate::link_constant::LinkConstant;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::pool_interceptor::PoolInterceptor;
use crate::pooled_constant::PooledConstant;
use crate::record_helper::RecordHelper;
use crate::rt::jchar::{is_java_identifier_part, is_java_identifier_start};
use crate::rt::jnum::int_to_hex_string;
use crate::rt::{rethrow, try_catch, BitSet, Caught, JHashSet, JavaStr};
use crate::statement_writer::StatementWriter;
use crate::struct_annotation_attribute::StructAnnotationAttribute;
use crate::struct_annotation_parameter_attribute::StructAnnotationParameterAttribute;
use crate::struct_bootstrap_methods_attribute::StructBootstrapMethodsAttribute;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_general_attribute::{AttributeKey, StructGeneralAttribute};
use crate::struct_member::StructMember;
use crate::struct_method::StructMethod;
use crate::struct_type_annotation_attribute::StructTypeAnnotationAttribute;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::type_annotation::TypeAnnotation;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;

/// `KotlinWriter.MType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum MType {
    Class,
    Field,
    Method,
}

#[allow(dead_code)]
const ERROR_DUMP_STOP_POINTS: [&str; 3] = [
    "Fernflower.decompileContext",
    "MethodProcessor.codeToJava",
    "KotlinWriter.writeMethod",
];

const NOT_NULL_ANN_NAME: &str = "org/jetbrains/annotations/NotNull";
const NULLABLE_ANN_NAME: &str = "org/jetbrains/annotations/Nullable";
const REPEATABLE_ANN_NAME: &str = "java/lang/annotation/Repeatable";

const KT_HARD_KEYWORDS: [&str; 28] = [
    "as",
    "break",
    "class",
    "continue",
    "do",
    "else",
    "false",
    "for",
    "fun",
    "if",
    "in",
    "interface",
    "is",
    "null",
    "object",
    "package",
    "return",
    "super",
    "this",
    "throw",
    "true",
    "try",
    "typealias",
    "typeof",
    "val",
    "var",
    "when",
    "while",
];

/// The `LinkedHashMap` `MODIFIERS`, in insertion order.
const MODIFIERS: [(i32, &str); 5] = [
    (ACC_PUBLIC, "public"),
    (ACC_PROTECTED, "protected"),
    (ACC_PRIVATE, "private"),
    (ACC_ABSTRACT, "abstract"),
    (ACC_NATIVE, "native"),
];

/// `String.split(sep)` for a one-character literal: trailing empty strings are dropped.
fn java_split(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return vec![s];
    }
    let mut parts: Vec<&str> = s.split(sep).collect();
    while parts.last() == Some(&"") {
        parts.pop();
    }
    parts
}

/// A thrown `java.lang.Error` (which `catch (Exception e)` does not catch).
fn is_java_error(caught: &Caught) -> bool {
    caught
        .exception
        .as_ref()
        .is_some_and(|e| e.class.ends_with("Error"))
}

/// kuna's stop signals: a host cancel (`CanceledException`) or the per-method
/// deadline (`MethodDeadlineExceeded`). Vineflower never raises either while it
/// writes a Kotlin class (its checker runs in `codeToJava`, its watchdog only
/// around it), so the Kotlin writer's catch-alls pass them on, as `ClassWriter`
/// does, instead of recording a method or class that could not be written.
pub fn is_stop_signal(caught: &Caught) -> bool {
    CanceledException::is(caught) || caught.is(MethodDeadlineExceeded::CLASS)
}

/// A Java `String` that may be `null`, as `StringBuilder.append` prints it.
fn or_null(s: Option<&str>) -> &str {
    s.unwrap_or("null")
}

/// The record's `toString()`, for log messages only (the protobuf fields print no further).
fn k_element_to_string(el: &KElement) -> String {
    match el {
        KElement::Class(_) => "KClass[...]".to_string(),
        KElement::Constructor(_) => "KConstructor[...]".to_string(),
        KElement::File(_) => "KFile[...]".to_string(),
        KElement::Function(f) => format!("KFunction[name={}, ...]", f.name),
        KElement::FunctionReference(_) => "KFunctionReference[]".to_string(),
        KElement::HiddenElement(h) => h.name().to_string(),
        KElement::MultifileFacade(f) => format!("KMultifileFacade[files={:?}]", f.files),
        KElement::Property(p) => format!("KProperty[name={}, ...]", p.name),
    }
}

fn simple_name(node: ClassNode) -> Option<String> {
    node.with(|d| d.simple_name.clone())
}

/// `finally { DecompilerContext.setProperty(CURRENT_CLASS_NODE, outer); getLogger().endWriteClass(); }`.
struct WriteClassFinally(Option<ClassNode>);

impl Drop for WriteClassFinally {
    fn drop(&mut self) {
        if DecompilerContext::get_current_context().is_some() {
            DecompilerContext::set_current_class_node(self.0);
            DecompilerContext::get_logger().end_write_class();
        }
    }
}

/// `finally { DecompilerContext.setProperty(CURRENT_METHOD_WRAPPER, outer) }`.
struct RestoreMethodWrapper(Option<MethodWrapper>);

impl Drop for RestoreMethodWrapper {
    fn drop(&mut self) {
        if DecompilerContext::get_current_context().is_some() {
            DecompilerContext::set_current_method_wrapper(self.0);
        }
    }
}

/// Writes Kotlin classes, the `StatementWriter` of the Kotlin `LanguageSpec`.
pub struct KotlinWriter {
    interceptor: Option<PoolInterceptor>,
    javadoc_provider: Option<Rc<dyn IFabricJavadocProvider>>,
}

impl Default for KotlinWriter {
    fn default() -> Self {
        KotlinWriter::new()
    }
}

impl KotlinWriter {
    const CLASS_ALLOWED: i32 = ACC_PROTECTED | ACC_PRIVATE | ACC_ABSTRACT | ACC_STATIC | ACC_STRICT;
    const FIELD_ALLOWED: i32 = ACC_PUBLIC
        | ACC_PROTECTED
        | ACC_PRIVATE
        | ACC_STATIC
        | ACC_FINAL
        | ACC_TRANSIENT
        | ACC_VOLATILE;
    #[allow(dead_code)]
    const METHOD_ALLOWED: i32 = ACC_PUBLIC
        | ACC_PROTECTED
        | ACC_PRIVATE
        | ACC_ABSTRACT
        | ACC_STATIC
        | ACC_FINAL
        | ACC_SYNCHRONIZED
        | ACC_NATIVE
        | ACC_STRICT;

    const CLASS_EXCLUDED: i32 = ACC_ABSTRACT | ACC_STATIC;
    const FIELD_EXCLUDED: i32 = ACC_PUBLIC | ACC_STATIC | ACC_FINAL;
    #[allow(dead_code)]
    const METHOD_EXCLUDED: i32 = ACC_PUBLIC | ACC_ABSTRACT;

    const ACCESSIBILITY_FLAGS: i32 = ACC_PUBLIC | ACC_PROTECTED | ACC_PRIVATE;

    pub const ANNOTATION_ATTRIBUTES: [AttributeKey<StructAnnotationAttribute>; 2] = [
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS,
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS,
    ];

    pub const PARAMETER_ANNOTATION_ATTRIBUTES: [AttributeKey<StructAnnotationParameterAttribute>;
        2] = [
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS,
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS,
    ];

    pub const TYPE_ANNOTATION_ATTRIBUTES: [AttributeKey<StructTypeAnnotationAttribute>; 2] = [
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_TYPE_ANNOTATIONS,
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS,
    ];

    pub fn new() -> KotlinWriter {
        KotlinWriter {
            interceptor: DecompilerContext::get_pool_interceptor(),
            javadoc_provider: DecompilerContext::get_javadoc_provider(),
        }
    }

    pub fn ends_with_semicolon(&self, _expr: Exprent) -> bool {
        false
    }

    fn invoke_processors(&self, buffer: &mut TextBuffer, node: ClassNode) -> bool {
        let Some(wrapper) = node.get_wrapper() else {
            let qualified_name = node.with(|d| d.class_struct).qualified_name();
            kuna_spans::class_failed(&qualified_name, &format!("Class {qualified_name} wasn't processed yet"));
            buffer.append(&format!(
                "/* $VF: Couldn't be decompiled. Class {qualified_name} wasn't processed yet! */"
            ));
            let lines: Vec<String> = KotlinWriter::get_error_comment();
            for line in &lines {
                buffer.append("//");
                if !line.is_empty() {
                    buffer.append(" ").append(line);
                }
                buffer.append_line_separator();
            }
            return false;
        };
        let cl = wrapper.get_class_struct();

        let result = try_catch(|| {
            InitializerProcessor::extract_initializers(wrapper);
            InitializerProcessor::hide_initalizers(wrapper);

            if node.with(|d| d.type_) == ClassNodeType::Root
                && cl.get_version().has14_class_references()
                && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_CLASS_1_4)
            {
                ClassReference14Processor::process_class_references(node);
            }

            if cl.has_modifier(ACC_ENUM)
                && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
            {
                KEnumProcessor::clear_enum(wrapper);
            }

            KotlinObjectProcessors::parse_with_companion(node);
            KotlinObjectProcessors::mark_object_instance(wrapper);
        });
        if let Err(t) = result {
            if is_stop_signal(&t) {
                rethrow(t);
            }
            DecompilerContext::get_logger().write_message_ex(
                &format!(
                    "Class {} couldn't be written.",
                    or_null(simple_name(node).as_deref())
                ),
                Severity::Warn,
                &t,
            );
            kuna_spans::class_failed(&cl.qualified_name(), &t.message);
            buffer.append("// $VF: Couldn't be decompiled");
            buffer.append_line_separator();
            if DecompilerContext::get_option(IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR) {
                let mut lines: Vec<String> = KotlinWriter::get_error_comment();
                KotlinWriter::collect_error_lines(&t, &mut lines);
                for line in &lines {
                    buffer.append("//");
                    if !line.is_empty() {
                        buffer.append(" ").append(line);
                    }
                    buffer.append_line_separator();
                }
            }

            return false;
        }

        true
    }

    pub fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        let qualified_name = cl.qualified_name();
        let index = qualified_name.rfind('/');
        let has_package = index.is_some();

        let kt_data = KtAttrs::get(cl, &KElement::KEY());
        if let Some(KElement::File(file)) = &kt_data {
            if let Some(multifile_name) = &file.multifile_name {
                buffer
                    .append("@file:JvmMultifileClass")
                    .append_line_separator();

                if has_package {
                    let pkg = &qualified_name[..qualified_name.rfind('/').unwrap()];
                    buffer.append("@file:JvmName(\"");
                    let jvm_name = if multifile_name.starts_with(pkg) {
                        multifile_name.jsubstring_from(pkg.jlen() + 1)
                    } else {
                        multifile_name.clone()
                    };
                    buffer.append(&jvm_name);
                    buffer.append("\")").append_line_separator();
                } else {
                    buffer
                        .append("@file:JvmName(\"")
                        .append(multifile_name)
                        .append("\")")
                        .append_line_separator();
                }
            }

            for key in KotlinWriter::ANNOTATION_ATTRIBUTES {
                let attr = cl.get_attribute(key);
                if let Some(attr) = attr {
                    for expr in attr.get_annotations() {
                        if expr.get_class_name() == "kotlin/Metadata" {
                            continue;
                        }

                        let k_annotation_exprent =
                            KAnnotationExprent::new_target(expr, Some(UseSiteTarget::File));
                        buffer
                            .append_buffer(&KAnnotationExprent::to_java(k_annotation_exprent, 0))
                            .append_line_separator();
                    }
                }
            }

            if !buffer.contains_only_whitespaces() {
                buffer.append_line_separator();
            }
        }

        if let Some(index) = index {
            buffer.append("package ");

            let package_parts = java_split(&qualified_name[..index], '/');
            for (i, part) in package_parts.iter().enumerate() {
                if i > 0 {
                    buffer.append(".");
                }
                buffer.append(&KotlinWriter::to_valid_kotlin_identifier(part));
            }

            buffer.append_line_separator().append_line_separator();
        }

        let kotlin_import_collector = KotlinImportCollector::new(import_collector);
        kotlin_import_collector.write_imports(buffer, true);

        if let Some(KElement::Class(cls)) = &kt_data {
            if cls.proto.get_type_alias_count() > 0 {
                let type_aliases = cls.proto.get_type_alias_list();
                for type_alias in &type_aliases {
                    buffer.append("typealias ");
                    let name = KotlinWriter::class_resolver(cls).resolve(type_alias.get_name());
                    buffer.append(&KotlinWriter::to_valid_kotlin_identifier(&name));
                    buffer.append(" = ");
                    let underlying = KotlinWriter::class_resolver(cls)
                        .resolve(type_alias.get_underlying_type().get_class_name());
                    buffer.append(&KotlinWriter::to_valid_kotlin_identifier(&underlying));
                    buffer.append_line_separator();
                }

                buffer.append_line_separator();
            }
        }
    }

    /// `cls.resolver()`, dereferenced (a `null` resolver is a `NullPointerException`).
    fn class_resolver(cls: &KClass) -> &MetadataNameResolver {
        cls.resolver
            .as_ref()
            .expect("NullPointerException: KClass.resolver")
    }

    /// Not in Vineflower: the class and each method written get a kuna span
    /// (`kuna_spans`), as `ClassWriter` gives them; spans add no text.
    pub fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        let span = kuna_spans::open_class(buffer, node.with(|d| d.class_struct));
        if self.write_class_body(node, buffer, indent) {
            kuna_spans::close(buffer, span);
        } else {
            kuna_spans::close_failed(buffer, span);
        }
    }

    /// `writeClass` itself; false when the class could not be written.
    fn write_class_body(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) -> bool {
        let mut indent = indent;
        let outer_node = DecompilerContext::get_current_class_node();
        DecompilerContext::set_current_class_node(Some(node));
        DecompilerContext::set_import_collector(
            KotlinImportCollector::new(DecompilerContext::get_import_collector()).0,
        );

        let _finally = WriteClassFinally(outer_node);

        let ok = self.invoke_processors(buffer, node);

        if !ok {
            return false;
        }

        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();
        let pool = cl.get_pool();

        KotlinChooser::parse_metadata_for(cl);
        let kt_data = KtAttrs::get(cl, &KElement::KEY());

        DecompilerContext::get_logger().start_write_class(&cl.qualified_name());

        if let Some(KElement::MultifileFacade(facade)) = &kt_data {
            buffer
                .append("/*")
                .append_line_separator()
                .append(" * This is a multifile facade. The actual implementations of the functions are in the following files:")
                .append_line_separator();

            for file in &facade.files {
                buffer.append(" * ").append(file).append_line_separator();
            }

            buffer.append(" */").append_line_separator();

            if !DecompilerContext::get_option(KotlinOptions::DECOMPILE_FACADES) {
                return true;
            }
        }

        let kotlin_flags = match &kt_data {
            None => {
                KotlinWriter::append_comment(buffer, "Class flags could not be determined", indent);
                0
            }
            Some(KElement::Class(cls)) => cls.flags(),
            Some(_) => 0,
        };

        if DecompilerContext::get_option(IFernflowerPreferences::SOURCE_FILE_COMMENTS) {
            let source_file_attr = node
                .with(|d| d.class_struct)
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_SOURCE_FILE);

            if let Some(source_file_attr) = source_file_attr {
                let source_file = source_file_attr.get_source_file(pool);

                buffer
                    .append_indent(indent)
                    .append(&format!("// $VF: Compiled from {source_file}"))
                    .append_line_separator();
            }
        }

        if Flags::CLASS_KIND.get(kotlin_flags) == Some(ClassKind::AnnotationClass)
            || cl.has_modifier(ACC_ANNOTATION)
        {
            self.write_annotation_definition(node, buffer, indent, kt_data.as_ref());
            return true;
        }

        if let Some(KElement::Function(function)) = &kt_data {
            self.write_lambda(node, buffer, indent, function);
            return true;
        }

        let companion: Option<ClassNode> = match &kt_data {
            Some(KElement::Class(cls)) if cls.proto.has_companion_object_name() => {
                let name = KotlinWriter::class_resolver(cls)
                    .resolve(cls.proto.get_companion_object_name());
                let nested = node.with(|d| d.nested.to_vec());
                nested
                    .into_iter()
                    .find(|n| simple_name(*n).expect("NullPointerException: simpleName") == name)
            }
            _ => None,
        };

        if !matches!(kt_data, Some(KElement::File(_))) {
            self.write_class_definition(node, buffer, indent, kt_data.as_ref(), kotlin_flags);
            indent += 1;
        }

        let mut inner_buffer = TextBuffer::new();

        let mut has_content = false;

        let components = cl.get_record_components();

        let mut enum_fields: Vec<StructField> = Vec::new();

        let fields = cl.get_fields().to_vec();
        for &fd in &fields {
            let is_enum = fd.has_modifier(ACC_ENUM)
                && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);
            if is_enum {
                enum_fields.push(fd);
            }
        }

        let mut enums = false;
        for fd in enum_fields {
            if enums {
                inner_buffer.append(",").append_line_separator();
            }
            enums = true;

            self.write_field_to_buffer(&mut inner_buffer, indent, fd, wrapper);
            has_content = true;
        }

        let primary_constructor = KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR());
        let constructor_key = format!(
            "<init> {}",
            match &primary_constructor {
                Some(primary_constructor) => primary_constructor
                    .method_struct
                    .expect("NullPointerException: methodStruct")
                    .get_descriptor(),
                None => "()V".to_string(),
            }
        );

        let mut visited_properties: JHashSet<KProperty> = JHashSet::new();
        let mut has_newline = false;

        for &fd in &fields {
            if enums {
                inner_buffer
                    .append(";")
                    .append_line_separator()
                    .append_line_separator();
                enums = false;
                has_newline = true;
            }

            if fd.has_modifier(ACC_ENUM) {
                continue;
            }

            let el = KtAttrs::get(fd, &KElement::KEY());
            if let Some(KElement::Property(prop)) = &el {
                if prop.class_struct != cl || !visited_properties.add((**prop).clone()) {
                    continue;
                }

                let prop_buffer = prop.stringify(wrapper, indent);
                let is_multiline = prop_buffer.count_lines() > 0;

                if is_multiline && has_content && !has_newline {
                    inner_buffer.append_line_separator();
                }

                let initializer = if fd.has_modifier(ACC_STATIC) {
                    "<clinit> ()V"
                } else {
                    constructor_key.as_str()
                };
                let spans = KotlinWriter::open_property_spans(&mut inner_buffer, prop, wrapper);
                inner_buffer
                    .append_buffer_mapped(
                        &prop_buffer,
                        Some(&cl.qualified_name()),
                        Some(initializer),
                    )
                    .append_line_separator();
                KotlinWriter::close_spans(&mut inner_buffer, spans);

                if is_multiline {
                    inner_buffer.append_line_separator();
                    has_newline = true;
                } else {
                    has_newline = false;
                }

                has_content = true;
            } else if !matches!(el, Some(KElement::HiddenElement(_))) {
                if fd.is_synthetic()
                    && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC)
                {
                    continue;
                }

                if wrapper.get_hidden_members().borrow().contains(
                    &InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor()),
                ) {
                    continue;
                }

                if let Some(components) = &components {
                    if fd.get_access_flags() == (ACC_FINAL | ACC_PRIVATE)
                        && components.iter().any(|c| {
                            c.get_name() == fd.get_name()
                                && c.get_descriptor() == fd.get_descriptor()
                        })
                    {
                        continue;
                    }
                }

                match &el {
                    Some(el) => DecompilerContext::get_logger().write_message(
                        &format!(
                            "Unexpected Kotlin type for field {}: {}",
                            fd.get_name(),
                            k_element_to_string(el)
                        ),
                        Severity::Warn,
                    ),
                    None => DecompilerContext::get_logger().write_message(
                        &format!(
                            "Non-enum field {} is not associated with Kotlin property",
                            fd.get_name()
                        ),
                        Severity::Warn,
                    ),
                }

                let mut field_buffer = TextBuffer::new();
                self.write_field(
                    wrapper,
                    wrapper.get_class_struct(),
                    fd,
                    &mut field_buffer,
                    indent,
                );
                let initializer = if fd.has_modifier(ACC_STATIC) {
                    "<clinit> ()V"
                } else {
                    constructor_key.as_str()
                };
                inner_buffer.append_buffer_mapped(
                    &field_buffer,
                    Some(&wrapper.get_class_struct().qualified_name()),
                    Some(initializer),
                );
                has_newline = false;
                has_content = true;
            }
        }

        if has_newline {
            inner_buffer.set_length(
                inner_buffer.length() - DecompilerContext::get_new_line_separator().len() as i32,
            );
        }

        let methods = cl.get_methods();
        for i in 0..methods.size() {
            let mt = methods.get(i);
            let el = KtAttrs::get(mt, &KElement::KEY());
            let method_key =
                || InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor());

            let hide = (mt.is_synthetic()
                && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
                || (mt.has_modifier(ACC_BRIDGE)
                    && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_BRIDGE))
                || wrapper
                    .get_hidden_members()
                    .borrow()
                    .contains(&method_key())
                || (mt.get_name() == "<init>"
                    && mt.get_descriptor() == "(Lkotlin/jvm/internal/DefaultConstructorMarker;)V")
                || (companion
                    .map(|c| {
                        c.get_wrapper()
                            .expect("NullPointerException: companion wrapper")
                            .get_methods()
                            .borrow()
                            .contains_key(&method_key())
                    })
                    .unwrap_or(false)
                    && mt.has_modifier(ACC_STATIC))
                || mt.get_name().contains("$lambda$")
                || matches!(&el, Some(KElement::Property(prop)) if !visited_properties.add((**prop).clone()))
                || matches!(el, Some(KElement::HiddenElement(_)));
            if hide {
                continue;
            }

            if enums {
                inner_buffer.append(";").append_line_separator();
            }

            match &el {
                Some(KElement::Property(prop)) => {
                    if prop.class_struct != cl {
                        continue;
                    }

                    if has_content {
                        inner_buffer.append_line_separator();
                    }
                    has_content = true;
                    let spans = KotlinWriter::open_property_spans(&mut inner_buffer, prop, wrapper);
                    inner_buffer
                        .append_buffer_mapped(
                            &prop.stringify(wrapper, indent),
                            Some(&cl.qualified_name()),
                            Some(&method_key()),
                        )
                        .append_line_separator();
                    KotlinWriter::close_spans(&mut inner_buffer, spans);
                }
                Some(KElement::Function(function)) => {
                    if function.class_struct != Some(cl) {
                        continue;
                    }

                    if has_content {
                        inner_buffer.append_line_separator();
                    }
                    has_content = true;
                    let span = kuna_spans::open_method(&mut inner_buffer, cl, mt, wrapper);
                    inner_buffer.append_buffer_mapped(
                        &function.stringify(wrapper, indent),
                        Some(&cl.qualified_name()),
                        Some(&method_key()),
                    );
                    kuna_spans::close(&mut inner_buffer, span);
                }
                Some(KElement::Constructor(constructor)) => {
                    let mut ctor_buffer = TextBuffer::new();
                    let span = kuna_spans::open_method(&mut ctor_buffer, cl, mt, wrapper);
                    let ctor_skipped = !constructor.stringify(wrapper, &mut ctor_buffer, indent);
                    kuna_spans::close(&mut ctor_buffer, span);
                    if ctor_skipped {
                        ctor_buffer.clear_unassigned_bytecode_mapping_data();
                    } else {
                        if has_content {
                            inner_buffer.append_line_separator();
                        }
                        has_content = true;
                        inner_buffer.append_buffer_mapped(
                            &ctor_buffer,
                            Some(&cl.qualified_name()),
                            Some(&method_key()),
                        );
                    }
                }
                _ => {
                    match &el {
                        Some(el) => DecompilerContext::get_logger().write_message(
                            &format!(
                                "Unexpected Kotlin type for method {}: {}",
                                mt.get_name(),
                                k_element_to_string(el)
                            ),
                            Severity::Warn,
                        ),
                        None => DecompilerContext::get_logger().write_message(
                            &format!(
                                "Method {} is not associated with Kotlin property or function",
                                mt.get_name()
                            ),
                            Severity::Warn,
                        ),
                    }

                    let mut method_buffer = TextBuffer::new();
                    let span = kuna_spans::open_method(&mut method_buffer, cl, mt, wrapper);
                    let method_skipped =
                        !self.write_method(node, mt, i as i32, &mut method_buffer, indent);
                    kuna_spans::close(&mut method_buffer, span);
                    if !method_skipped {
                        if has_content {
                            inner_buffer.append_line_separator();
                        }
                        has_content = true;
                        inner_buffer.append_buffer(&method_buffer);
                    }
                }
            }
        }

        let nested = node.with(|d| d.nested.to_vec());
        for inner in nested {
            if inner.with(|d| d.type_) == ClassNodeType::Member {
                let (inner_cl, inner_access) = inner.with(|d| (d.class_struct, d.access));
                let is_synthetic = (inner_access & ACC_SYNTHETIC) != 0 || inner_cl.is_synthetic();
                let hide = (is_synthetic
                    && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
                    || wrapper
                        .get_hidden_members()
                        .borrow()
                        .contains(&inner_cl.qualified_name());
                if hide {
                    continue;
                }

                if has_content {
                    inner_buffer.append_line_separator();
                }
                self.write_class(inner, &mut inner_buffer, indent);

                has_content = true;
            }
        }

        if matches!(kt_data, Some(KElement::File(_))) {
            buffer.append_buffer(&inner_buffer);
            return true;
        }

        if !inner_buffer.contains_only_whitespaces() {
            indent -= 1;
            buffer
                .append(" {")
                .append_line_separator()
                .append_buffer(&inner_buffer)
                .append_indent(indent)
                .append("}");
        }

        if node.with(|d| d.type_) != ClassNodeType::Anonymous {
            buffer.append_line_separator();
        }
        true
    }

    /// Spans for the accessor methods of `prop` that this class declares,
    /// opened where its text starts.
    fn open_property_spans(
        buffer: &mut TextBuffer,
        prop: &KProperty,
        wrapper: ClassWrapper,
    ) -> Vec<kuna_spans::SpanId> {
        let cl = wrapper.get_class_struct();
        [&prop.getter, &prop.setter]
            .into_iter()
            .flatten()
            .filter_map(|accessor| (accessor.method_supplier)(wrapper))
            .map(|method| method.with(|d| d.method_struct))
            .map(|mt| kuna_spans::open_method(buffer, cl, mt, wrapper))
            .collect()
    }

    fn close_spans(buffer: &mut TextBuffer, spans: Vec<kuna_spans::SpanId>) {
        for span in spans {
            kuna_spans::close(buffer, span);
        }
    }

    /// `function` may be `KFunction::FAILED_LAMBDA()` (compared with `Rc::ptr_eq`).
    fn write_lambda(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        function: &Rc<KFunction>,
    ) {
        if Rc::ptr_eq(function, &KFunction::FAILED_LAMBDA()) {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "No Kotlin function information for lambda class {}",
                    or_null(simple_name(node).as_deref())
                ),
                Severity::Warn,
            );
            buffer.append("{").append_line_separator();
            kuna_spans::class_failed(
                &node.with(|d| d.class_struct).qualified_name(),
                "Could not decompile lambda - root function was not found. Is this a suspend lambda?",
            );
            KotlinWriter::append_comment(
                buffer,
                "Could not decompile lambda - root function was not found. Is this a suspend lambda?",
                indent + 1,
            );
            buffer.append_indent(indent).append("}");
            return;
        }
        let mt = function
            .method_struct
            .expect("NullPointerException: methodStruct");
        let cl = node.with(|d| d.class_struct);

        let span = match node.get_wrapper() {
            Some(wrapper) => kuna_spans::open_method(buffer, cl, mt, wrapper),
            None => None,
        };
        buffer.append("{");

        let mut index: i32 = 0;
        for param in function.parameters.iter() {
            if index > 0 {
                buffer.append(", ");
            } else {
                buffer.append(" ");
            }

            buffer.append_variable_descriptor(
                &param.name,
                true,
                true,
                &cl.qualified_name(),
                &mt.get_name(),
                &mt.method_descriptor(),
                index,
                &mt.get_name(),
            );
            buffer
                .append(": ")
                .append_buffer(&param.type_.stringify(indent + 1));
            index += param.type_.stack_size;
        }

        if !function.parameters.is_empty() {
            buffer.append(" ->");
        }

        buffer.append_line_separator();

        let method_wrapper = (function.method_supplier)(
            node.get_wrapper()
                .expect("NullPointerException: node.getWrapper()"),
        )
        .expect("NullPointerException: methodWrapper");
        let root = method_wrapper.with(|d| d.root);

        if let Some(root) = root.filter(|_| method_wrapper.with(|d| d.decompile_error.is_none())) {
            let result = try_catch(|| {
                let mut code = root.to_java(indent + 1);
                code.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                buffer.append_buffer_mapped(
                    &code,
                    Some(&cl.qualified_name()),
                    Some(&InterpreterUtil::make_unique_key(
                        &mt.get_name(),
                        &mt.get_descriptor(),
                    )),
                );
            });
            if let Err(t) = result {
                if is_stop_signal(&t) {
                    rethrow(t);
                }
                let message = format!(
                    "Method {} {} in class {} couldn't be written.",
                    mt.get_name(),
                    mt.get_descriptor(),
                    cl.qualified_name()
                );
                DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);
                method_wrapper.with_mut(|d| d.decompile_error = Some(t));
            }
        }

        if method_wrapper.with(|d| d.decompile_error.is_some()) {
            KotlinWriter::dump_error(buffer, method_wrapper, indent + 1);
        }

        buffer.append_indent(indent).append("}");
        kuna_spans::close(buffer, span);
    }

    pub fn stringify_reference(
        indent: i32,
        node: ClassNode,
        bytecode: Option<&BitSet>,
        receiver: Option<Exprent>,
    ) -> Option<TextBuffer> {
        let init = node
            .get_wrapper()
            .expect("NullPointerException: node.getWrapper()")
            .get_method_wrapper(
                "<init>",
                if receiver.is_some() {
                    "(Ljava/lang/Object;)V"
                } else {
                    "()V"
                },
            )?;

        let exprents = init
            .with(|d| d.root)
            .expect("NullPointerException: init.root")
            .get_first()
            .expect("NullPointerException: root.getFirst()")
            .get_exprents();
        let exprents = match exprents {
            Some(exprents) if exprents.len() == 1 => exprents,
            _ => return None,
        };

        let mut buf = TextBuffer::new();
        buf.add_bytecode_mapping(bytecode);

        let super_call = exprents.get(0);
        let parameters = super_call.as_invocation().get_lst_parameters();

        let const_string = |index: usize| -> String {
            parameters
                .get(index)
                .as_const()
                .get_value()
                .expect("NullPointerException: ConstExprent.value")
                .java_to_string()
        };
        let mut parameter_index: usize = if receiver.is_none() { 1 } else { 2 };
        let class_ref = const_string(parameter_index);
        parameter_index += 1;
        let method_ref = const_string(parameter_index);
        parameter_index += 1;
        let method_and_desc = const_string(parameter_index);

        let cl = DecompilerContext::get_struct_context().get_class(&class_ref);

        let desc = method_and_desc.jsubstring_from(method_ref.jlen());

        if method_ref == "<init>" {
            buf.append("::");
            let class_type = VarType::new_signature_cl(&class_ref, true);
            buf.append_method(
                &KTypes::get_kotlin_type(Some(&class_type)),
                false,
                &class_ref,
                &method_ref,
                &desc,
            );
            return Some(buf);
        }

        match receiver {
            Some(receiver) => {
                buf.append_buffer(&receiver.to_java(indent));
            }
            None => {
                let is_file = cl.is_some_and(|cl| {
                    matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::File(_)))
                });
                if !is_file {
                    let class_type = VarType::new_signature_cl(&class_ref, true);
                    buf.append_type_name_str(
                        &KTypes::get_kotlin_type(Some(&class_type)),
                        &class_type,
                    );
                }
            }
        }

        buf.append("::");
        buf.append_method(&method_ref, false, &class_ref, &method_ref, &desc);
        Some(buf)
    }

    fn write_annotation_definition(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        _kt_data: Option<&KElement>,
    ) {
        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();

        let runtime_annotations = cl
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS)
            .expect("NullPointerException: RuntimeVisibleAnnotations");
        let annotations = runtime_annotations.get_annotations();
        let repeatable_index = annotations
            .iter()
            .position(|a| REPEATABLE_ANN_NAME == a.get_class_name());

        let mut repeatable_container: Option<String> = None;

        if let Some(repeatable_index) = repeatable_index {
            let repeatable_annotation = annotations[repeatable_index];
            repeatable_container = Some(
                repeatable_annotation.get_par_values()[0]
                    .as_const()
                    .get_value()
                    .expect("NullPointerException: ConstExprent.value")
                    .java_to_string(),
            );
            KotlinWriter::remove_annotation(runtime_annotations, annotations, repeatable_index);
        }

        KotlinWriter::append_annotations(buffer, indent, cl.0, -1);
        KotlinWriter::append_jvm_annotations(
            buffer,
            indent,
            cl.0,
            true,
            false,
            cl.get_pool(),
            TypeAnnotation::CLASS_TYPE_PARAMETER,
        );

        buffer.append_indent(indent);
        KotlinWriter::append_modifiers(
            buffer,
            cl.get_access_flags(),
            KotlinWriter::CLASS_ALLOWED,
            true,
            KotlinWriter::CLASS_EXCLUDED,
        );

        let class_type = VarType::new_signature_cl(&cl.qualified_name(), true);

        buffer
            .append("annotation class ")
            .append(&KTypes::get_kotlin_type_outer(Some(&class_type), false))
            .append("(")
            .append_line_separator();

        if KtAttrs::has(cl, &KConstructor::PRIMARY_CONSTRUCTOR()) {
            let primary = KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR())
                .expect("NullPointerException: PRIMARY_CONSTRUCTOR");
            let map = (primary.default_args_supplier)(wrapper);

            let mut first = true;
            for param in primary.parameters.iter() {
                if !first {
                    buffer.append(",").append_line_separator();
                }
                first = false;
                buffer
                    .append_indent(indent + 1)
                    .append("val ")
                    .append(&KotlinWriter::to_valid_kotlin_identifier(&param.name))
                    .append(": ")
                    .append_buffer(&param.type_.stringify(indent + 1));

                if Flags::DECLARES_DEFAULT_VALUE.get(param.flags) {
                    buffer.append_buffer(&map.to_java(param, indent + 1));
                }
            }
        }

        buffer
            .append_line_separator()
            .append_indent(indent)
            .append(")");

        let mut appended = false;

        let mut inner_buffer = TextBuffer::new();

        let mut first = true;
        let nested = node.with(|d| d.nested.to_vec());
        for inner in nested {
            if inner.with(|d| d.type_) == ClassNodeType::Member {
                if repeatable_container.as_deref()
                    == Some(inner.with(|d| d.class_struct).qualified_name().as_str())
                {
                    continue;
                }

                if !first || appended {
                    inner_buffer.append_line_separator();
                }
                first = false;

                self.write_class(inner, &mut inner_buffer, indent + 1);
                appended = true;
            }
        }

        if appended {
            buffer
                .append(" {")
                .append_line_separator()
                .append_buffer(&inner_buffer)
                .append_indent(indent)
                .append("}");
        }

        buffer.append_line_separator();
    }

    /// `attribute.getAnnotations().remove(annotation)`: the attribute keeps the
    /// shortened list, in this class scope and (through its source) in later ones.
    fn remove_annotation(
        attribute: StructAnnotationAttribute,
        annotations: Vec<AnnotationExprent>,
        index: usize,
    ) {
        let mut annotations = annotations;
        annotations.remove(index);
        attribute.d_mut(|d| {
            if !d.pinned {
                d.source.remove(index);
            }
            d.annotations = Some(annotations);
        });
    }

    fn write_class_definition(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        kt_data: Option<&KElement>,
        kotlin_flags: i32,
    ) {
        let node_type = node.with(|d| d.type_);
        if node_type == ClassNodeType::Anonymous {
            let Some(KElement::Class(cls)) = kt_data else {
                jthrow!(IllegalStateException; "Anonymous class does not have Class Kotlin metadata");
            };

            buffer.append("object");

            let mut first = true;
            for supertype in cls.proto.get_supertype_list() {
                let k_type = KType::from(&supertype, KotlinWriter::class_resolver(cls));
                if VarType::VARTYPE_OBJECT().equals(&k_type) {
                    continue;
                }

                if first {
                    buffer.append(" : ");
                    first = false;
                } else {
                    buffer.append(", ");
                }

                buffer.append_buffer(&k_type.stringify(indent));
            }

            return;
        }

        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();

        let flags = if node_type == ClassNodeType::Root {
            cl.get_access_flags()
        } else {
            node.with(|d| d.access)
        };
        let is_deprecated = cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED);
        let is_enum = DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
            && (flags & ACC_ENUM) != 0;
        let is_interface = (flags & ACC_INTERFACE) != 0;
        let is_annotation = (flags & ACC_ANNOTATION) != 0;

        if is_deprecated && !KotlinWriter::contains_deprecated_annotation(cl.0) {
            KotlinWriter::append_deprecation(buffer, indent);
        }

        if let Some(interceptor) = self.interceptor {
            let old_name = interceptor.get_old_name(&cl.qualified_name());
            KotlinWriter::append_rename_comment(buffer, old_name.as_deref(), MType::Class, indent);
        }

        if let Some(javadoc_provider) = &self.javadoc_provider {
            KotlinWriter::append_javadoc(
                buffer,
                javadoc_provider.get_class_doc(cl).as_deref(),
                indent,
            );
        }

        KotlinWriter::append_annotations(buffer, indent, cl.0, -1);
        KotlinWriter::append_jvm_annotations(
            buffer,
            indent,
            cl.0,
            is_interface,
            false,
            cl.get_pool(),
            TypeAnnotation::CLASS_TYPE_PARAMETER,
        );

        buffer.append_indent(indent);

        if Flags::VISIBILITY.get(kotlin_flags) != Some(Visibility::Public)
            || DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY)
        {
            KUtils::append_visibility(buffer, Flags::VISIBILITY.get(kotlin_flags));
        }

        if Flags::IS_EXPECT_CLASS.get(kotlin_flags) {
            buffer.append("expect ");
        }

        buffer.append(
            match Flags::MODALITY
                .get(kotlin_flags)
                .expect("NullPointerException: modality")
            {
                Modality::Final => {
                    if is_interface {
                        "final "
                    } else {
                        ""
                    }
                }
                Modality::Open => {
                    if is_interface {
                        ""
                    } else {
                        "open "
                    }
                }
                Modality::Abstract => {
                    if is_interface {
                        ""
                    } else {
                        "abstract "
                    }
                }
                Modality::Sealed => "sealed ",
            },
        );

        if Flags::IS_EXTERNAL_CLASS.get(kotlin_flags) {
            buffer.append("external ");
        }
        if Flags::IS_INNER.get(kotlin_flags) {
            buffer.append("inner ");
        }
        if Flags::IS_FUN_INTERFACE.get(kotlin_flags) {
            buffer.append("fun ");
        }
        if Flags::IS_VALUE_CLASS.get(kotlin_flags) {
            buffer.append("value ");
        }
        if Flags::IS_DATA.get(kotlin_flags) {
            buffer.append("data ");
        }

        let class_kind = Flags::CLASS_KIND.get(kotlin_flags);
        if is_enum {
            buffer.append("enum class");
        } else if is_interface {
            buffer.append("interface");
        } else if is_annotation {
            buffer.append("annotation class");
        } else if class_kind == Some(ClassKind::Object) {
            buffer.append("object");
        } else if class_kind == Some(ClassKind::CompanionObject) {
            buffer.append("companion object");
        } else {
            buffer.append("class");
        }

        if class_kind != Some(ClassKind::CompanionObject)
            || simple_name(node).expect("NullPointerException: simpleName") != "Companion"
        {
            let name = simple_name(node);
            buffer.append(" ").append(or_null(
                name.as_deref()
                    .map(KotlinWriter::to_valid_kotlin_identifier)
                    .as_deref(),
            ));
        }

        let descriptor = cl.get_signature();
        if let Some(descriptor) = &descriptor {
            let superclass = VarType::new_signature_cl(
                &cl.super_class()
                    .expect("NullPointerException: superClass")
                    .get_string(),
                true,
            );
            let interfaces: Vec<VarType> = cl
                .get_interface_names()
                .iter()
                .map(|s| VarType::new_signature_cl(s, true))
                .collect();

            descriptor.verify_types(&superclass, &interfaces);
        }

        if let Some(descriptor) = &descriptor {
            if !descriptor.fparameters().is_empty() {
                let fparameters = descriptor.fparameters().clone();
                let fbounds = descriptor.fbounds().clone();
                KotlinWriter::append_type_parameters(buffer, &fparameters, &fbounds);
            }
        }

        buffer.push_newline_group(indent, 1);

        let mut appended_colon = false;
        let wrote_supertype = match KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR()) {
            Some(constructor) => constructor.write_primary_constructor(wrapper, buffer, indent),
            None => false,
        };
        if wrote_supertype {
            appended_colon = true;
        } else if !is_enum && !is_interface {
            if let Some(super_class) = cl.super_class() {
                let supertype = VarType::new_signature_cl(&super_class.get_string(), true);
                if !VarType::VARTYPE_OBJECT().equals(&supertype) {
                    buffer.append_possible_newline_alt(" ");
                    buffer.append(": ");
                    let type_ = match &descriptor {
                        None => supertype,
                        Some(descriptor) => descriptor.superclass(),
                    };
                    buffer.append(&ExprProcessor::get_cast_type_name(&type_));
                    appended_colon = true;
                }
            }
        }

        if !is_annotation {
            let interfaces = cl.get_interfaces();
            if !interfaces.is_empty() {
                for i in 0..interfaces.len() {
                    if !appended_colon {
                        buffer.append(" :");
                        appended_colon = true;
                    } else {
                        buffer.append(",");
                    }
                    buffer.append_possible_newline_alt(" ");

                    let type_ = match &descriptor {
                        None => Some(VarType::new_signature_cl(&cl.get_interface(i as i32), true)),
                        Some(descriptor) => descriptor.superinterfaces().get(i).cloned(),
                    };
                    if let Some(type_) = type_ {
                        buffer.append(&ExprProcessor::get_cast_type_name(&type_));
                    }
                }
            }
        }

        buffer.pop_newline_group();
    }

    /// Java: writeField(TextBuffer, int, StructField, ClassWrapper)
    fn write_field_to_buffer(
        &self,
        buffer: &mut TextBuffer,
        indent: i32,
        fd: StructField,
        wrapper: ClassWrapper,
    ) {
        let mut field_buffer = TextBuffer::new();
        self.write_field(
            wrapper,
            wrapper.get_class_struct(),
            fd,
            &mut field_buffer,
            indent,
        );
        let initializer = if fd.has_modifier(ACC_STATIC) {
            "<clinit> ()V"
        } else {
            "<init> ()V"
        };
        buffer.append_buffer_mapped(
            &field_buffer,
            Some(&wrapper.get_class_struct().qualified_name()),
            Some(initializer),
        );
    }

    /// Java: writeField(ClassWrapper, StructClass, StructField, TextBuffer, int)
    pub fn write_field(
        &self,
        wrapper: ClassWrapper,
        cl: StructClass,
        fd: StructField,
        buffer: &mut TextBuffer,
        indent: i32,
    ) {
        let is_interface = cl.has_modifier(ACC_INTERFACE);
        let is_deprecated = fd.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED);
        let is_enum = fd.has_modifier(ACC_ENUM)
            && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);

        if is_deprecated && !KotlinWriter::contains_deprecated_annotation(fd.0) {
            KotlinWriter::append_deprecation(buffer, indent);
        }

        let mut name = fd.get_name();
        if let Some(interceptor) = self.interceptor {
            let new_name = interceptor.get_name(&format!(
                "{} {} {}",
                cl.qualified_name(),
                fd.get_name(),
                fd.get_descriptor()
            ));

            if let Some(new_name) = new_name {
                name = java_split(&new_name, ' ')[1].to_string();
            }
        }

        if let Some(interceptor) = self.interceptor {
            let old_name = interceptor.get_old_name(&format!(
                "{} {} {}",
                cl.qualified_name(),
                name,
                fd.get_descriptor()
            ));
            KotlinWriter::append_rename_comment(buffer, old_name.as_deref(), MType::Field, indent);
        }

        if let Some(javadoc_provider) = &self.javadoc_provider {
            KotlinWriter::append_javadoc(
                buffer,
                javadoc_provider.get_field_doc(cl, fd).as_deref(),
                indent,
            );
        }
        KotlinWriter::append_annotations(buffer, indent, fd.0, TypeAnnotation::FIELD);

        let class_data = KtAttrs::get(cl, &KElement::KEY());
        let is_in_file = matches!(class_data, Some(KElement::File(_)));
        KotlinWriter::append_jvm_annotations(
            buffer,
            indent,
            fd.0,
            is_interface,
            is_in_file,
            cl.get_pool(),
            TypeAnnotation::FIELD,
        );

        buffer.append_indent(indent);

        if !fd.has_modifier(ACC_FINAL)
            && !fd.has_modifier(ACC_STATIC)
            && !fd.has_modifier(ACC_PRIVATE)
        {
            buffer.append("open ");
        }

        if !is_enum {
            KotlinWriter::append_modifiers(
                buffer,
                fd.get_access_flags(),
                KotlinWriter::FIELD_ALLOWED,
                is_interface,
                KotlinWriter::FIELD_EXCLUDED,
            );
        }

        let (field_type, descriptor) = KotlinWriter::get_field_type_data(fd);
        if let Some(descriptor) = &descriptor {
            descriptor.verify_type(cl.get_signature().as_ref(), &field_type);
        }
        let declared_type = |descriptor: &Option<GenericFieldDescriptor>| match descriptor {
            None => field_type.clone(),
            Some(descriptor) => descriptor.type_(),
        };

        if !is_enum {
            buffer.append(&ExprProcessor::get_cast_type_name(&declared_type(
                &descriptor,
            )));
            buffer.append(" ");
        }

        buffer.append(&name);

        let key = InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
        let initializer = if fd.has_modifier(ACC_STATIC) {
            wrapper
                .get_static_field_initializers()
                .borrow()
                .get_with_key(&key)
        } else {
            wrapper
                .get_dynamic_field_initializers()
                .borrow()
                .get_with_key(&key)
        };

        if let Some(initializer) = initializer {
            if is_enum && initializer.is_new() {
                let new_expr = initializer.as_new();
                new_expr.set_enum_const(true);
                buffer.append_buffer(&new_expr.0.to_java(indent));
            } else {
                buffer.append(" = ");

                if let Some(const_exprent) = initializer.try_as_const() {
                    const_exprent.adjust_const_type(&field_type);
                }

                ExprProcessor::get_casted_exprent(
                    initializer,
                    &declared_type(&descriptor),
                    buffer,
                    indent,
                    false,
                );
            }
        } else if fd.has_modifier(ACC_FINAL) && fd.has_modifier(ACC_STATIC) {
            let attr = fd.get_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE);
            if let Some(attr) = attr {
                let constant = cl
                    .get_pool()
                    .get_primitive_constant(attr.get_index())
                    .expect("NullPointerException: ConstantValue constant");
                buffer.append(" = ");
                let value = constant.value.as_ref().map(Into::into);
                buffer.append_buffer(
                    &ConstExprent::new(field_type.clone(), value, None)
                        .0
                        .to_java(indent),
                );
            }
        }

        if !is_enum {
            buffer.append(";").append_line_separator();
        }
    }

    /// `null` in gives `null` out in Java: map over the `Option` at the call site.
    pub fn to_valid_kotlin_identifier(name: &str) -> String {
        if name.is_empty() {
            return name.to_string();
        }

        if KT_HARD_KEYWORDS.contains(&name) {
            return format!("`{name}`");
        }

        let units = name.utf16();
        let mut requires_backticks = !is_java_identifier_start(units[0]) || units[0] == b'$' as u16;
        for &c in &units[1..] {
            if !is_java_identifier_part(c) || c == b'$' as u16 {
                requires_backticks = true;
                break;
            }
        }
        let mut name = name.to_string();
        let mut needs_comment = false;
        if name.contains('`') {
            name = name.replace('`', "_");
            needs_comment = true;
        }

        if requires_backticks {
            name = format!("`{name}`");
        }
        if needs_comment {
            format!("{name} /* $VF was: {name} */")
        } else {
            name
        }
    }

    pub fn write_method(
        &self,
        node: ClassNode,
        mt: StructMethod,
        method_index: i32,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();
        let method_wrapper = wrapper.get_method_wrapper_index(method_index);

        let mut hide_method = false;

        let outer_wrapper = DecompilerContext::get_current_method_wrapper();
        DecompilerContext::set_current_method_wrapper(Some(method_wrapper));

        {
            let _finally = RestoreMethodWrapper(outer_wrapper);
            let node_type = node.with(|d| d.type_);
            let (node_class_struct, node_access) = node.with(|d| (d.class_struct, d.access));

            let is_interface = cl.has_modifier(ACC_INTERFACE);
            let is_annotation = cl.has_modifier(ACC_ANNOTATION);
            let is_enum = cl.has_modifier(ACC_ENUM)
                && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);
            let is_deprecated = mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED);
            let mut cl_init = false;
            let mut init = false;
            let mut d_init = false;

            let md = MethodDescriptor::parse_descriptor_struct(mt, Some(node));

            let mut flags = mt.get_access_flags();
            if (flags & ACC_NATIVE) != 0 {
                flags &= !ACC_STRICT;
            }
            if CLINIT_NAME == mt.get_name() {
                flags &= ACC_STATIC;
            }

            if is_deprecated && !KotlinWriter::contains_deprecated_annotation(mt.0) {
                KotlinWriter::append_deprecation(buffer, indent);
            }

            let mut name: Option<String> = Some(mt.get_name());
            if let Some(interceptor) = self.interceptor {
                let new_name = interceptor.get_name(&format!(
                    "{} {} {}",
                    cl.qualified_name(),
                    mt.get_name(),
                    mt.get_descriptor()
                ));

                if let Some(new_name) = new_name {
                    name = Some(java_split(&new_name, ' ')[1].to_string());
                }
            }

            if let Some(interceptor) = self.interceptor {
                let old_name = interceptor.get_old_name(&format!(
                    "{} {} {}",
                    cl.qualified_name(),
                    or_null(name.as_deref()),
                    mt.get_descriptor()
                ));
                KotlinWriter::append_rename_comment(
                    buffer,
                    old_name.as_deref(),
                    MType::Method,
                    indent,
                );
            }

            let is_bridge = (flags & ACC_BRIDGE) != 0;
            if is_bridge {
                KotlinWriter::append_comment(buffer, "bridge methodSupplier", indent);
            }

            let (add_error_comment, has_comment_lines) =
                method_wrapper.with(|d| (d.add_error_comment, d.comment_lines.is_some()));
            if (DecompilerContext::get_option(IFernflowerPreferences::DECOMPILER_COMMENTS)
                && add_error_comment)
                || has_comment_lines
            {
                if add_error_comment {
                    for s in KotlinWriter::get_error_comment() {
                        method_wrapper.add_comment(&s);
                    }
                }

                let comment_lines = method_wrapper
                    .with(|d| d.comment_lines.as_ref().map(|c| c.to_vec()))
                    .expect("NullPointerException: commentLines");
                for s in comment_lines {
                    buffer
                        .append_indent(indent)
                        .append(&format!("// {s}"))
                        .append_line_separator();
                }
            }

            if let Some(javadoc_provider) = &self.javadoc_provider {
                KotlinWriter::append_javadoc(
                    buffer,
                    javadoc_provider.get_method_doc(cl, mt).as_deref(),
                    indent,
                );
            }

            KotlinWriter::append_annotations(
                buffer,
                indent,
                mt.0,
                TypeAnnotation::METHOD_RETURN_TYPE,
            );

            let class_data = KtAttrs::get(node_class_struct, &KElement::KEY());
            let is_in_file = matches!(class_data, Some(KElement::File(_)));
            KotlinWriter::append_jvm_annotations(
                buffer,
                indent,
                mt.0,
                is_interface,
                is_in_file,
                cl.get_pool(),
                TypeAnnotation::METHOD_RETURN_TYPE,
            );

            buffer.append_indent(indent);

            if name.as_deref() == Some(INIT_NAME) {
                if node_type == ClassNodeType::Anonymous {
                    name = Some(String::new());
                    d_init = true;
                } else {
                    name = simple_name(node);
                    init = true;
                }
            } else if name.as_deref() == Some(CLINIT_NAME) {
                name = Some(String::new());
                cl_init = true;
            }

            let mut did_override = false;
            if INIT_NAME != mt.get_name()
                && CLINIT_NAME != mt.get_name()
                && !mt.has_modifier(ACC_STATIC)
                && !mt.has_modifier(ACC_PRIVATE)
            {
                let is_override =
                    KotlinWriter::search_for_method(Some(cl), &mt.get_name(), &md, false);
                if is_override {
                    buffer.append("override ");
                    did_override = true;
                    if mt.has_modifier(ACC_ABSTRACT) {
                        buffer.append("abstract ");
                    }
                    if mt.has_modifier(ACC_FINAL) {
                        buffer.append("final ");
                    }
                }
            }

            if !did_override
                && !mt.has_modifier(ACC_FINAL)
                && !mt.has_modifier(ACC_PRIVATE)
                && !mt.has_modifier(ACC_STATIC)
                && !is_interface
                && !is_annotation
                && !is_enum
                && !cl.has_modifier(ACC_FINAL)
            {
                buffer.append(if mt.has_modifier(ACC_ABSTRACT) {
                    "abstract "
                } else {
                    "open "
                });
            }

            if !d_init {
                buffer.append("fun ");
            }

            let descriptor = mt.get_signature();
            if let Some(descriptor) = &descriptor {
                let mut params: Vec<VarType> = mt.method_descriptor().params.clone();
                if (node_access & ACC_ENUM) != 0 && init {
                    params.remove(0);
                    params.remove(0);
                }

                let attr = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS);
                let mut exceptions: Vec<VarType> = Vec::new();
                if let Some(attr) = attr {
                    for i in 0..attr.get_throws_exceptions().len() {
                        exceptions.push(VarType::new_signature_cl(
                            &attr.get_exc_classname(i as i32, node_class_struct.get_pool()),
                            true,
                        ));
                    }
                }

                let class_sig = if (mt.get_access_flags() & ACC_STATIC) == 0 {
                    node_class_struct.get_signature()
                } else {
                    None
                };
                let mut checker: Option<GenericsChecker> = None;
                if let Some(class_sig) = &class_sig {
                    let mut current = class_sig.get_checker();

                    let mut current_parent = node.with(|d| d.parent);
                    while let Some(parent) = current_parent {
                        let parent_signature = parent.with(|d| d.class_struct).get_signature();
                        if let Some(parent_signature) = parent_signature {
                            current = current.copy_parent(&parent_signature.get_checker());
                        }

                        current_parent = parent.with(|d| d.parent);
                    }
                    checker = Some(current);
                }

                descriptor.verify_types(
                    checker.as_ref(),
                    &params,
                    &mt.method_descriptor().ret,
                    &exceptions,
                );
            }

            let throws_exceptions = false;
            let mut param_count: i32 = 0;

            if !cl_init && !d_init {
                let this_var = !mt.has_modifier(ACC_STATIC);

                let mut index: i32 = if is_enum && init {
                    3
                } else if this_var {
                    1
                } else {
                    0
                };
                let mut start: usize = if is_enum && init { 2 } else { 0 };

                if let Some(descriptor) = &descriptor {
                    if !descriptor.type_parameters().is_empty() {
                        let type_parameters = descriptor.type_parameters().clone();
                        let type_parameter_bounds = descriptor.type_parameter_bounds().clone();
                        KotlinWriter::append_type_parameters(
                            buffer,
                            &type_parameters,
                            &type_parameter_bounds,
                        );
                        buffer.append(" ");
                    }
                }

                let (varproc, mask, method_struct) = method_wrapper
                    .with(|d| (d.varproc, d.synth_parameters.clone(), d.method_struct));

                let varproc_name = varproc.get_var_name(VarVersionPair::new(index, 0));
                let extension = varproc_name
                    .as_deref()
                    .is_some_and(|n| n.starts_with("$this$"));

                if extension {
                    let param_type = match &descriptor {
                        Some(descriptor) if !descriptor.parameter_types().is_empty() => {
                            descriptor.parameter_types()[0].clone()
                        }
                        _ => md.params[0].clone(),
                    };
                    let mut type_name = KTypes::get_kotlin_type(Some(&param_type));
                    let is_nullable = KotlinWriter::process_parameter_annotations(buffer, mt, 0);

                    if KTypes::is_function_type(&param_type) {
                        type_name = format!("({type_name})");
                    }

                    buffer.append(&type_name);

                    if is_nullable {
                        buffer.append("?");
                    }

                    buffer.append(".");

                    param_count += 1;
                    start += 1;
                    index += param_type.stack_size;
                }

                buffer.append(or_null(
                    name.as_deref()
                        .map(KotlinWriter::to_valid_kotlin_identifier)
                        .as_deref(),
                ));
                buffer.append("(");

                let mut last_visible_parameter_index: i32 = -1;
                for i in 0..md.params.len() {
                    if mask.as_ref().is_none_or(|m| m[i].is_none()) {
                        last_visible_parameter_index = i as i32;
                    }
                }
                if last_visible_parameter_index != -1 {
                    buffer.push_newline_group(indent, 1);
                    buffer.append_possible_newline();
                }

                let mut method_parameters = None;
                if DecompilerContext::get_option(IFernflowerPreferences::USE_METHOD_PARAMETERS) {
                    let attr =
                        mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS);
                    if let Some(attr) = attr {
                        method_parameters = Some(attr.get_entries());
                    }
                }

                let has_descriptor = descriptor.is_some();

                let mut first = true;
                buffer.push_newline_group(indent, 0);
                for i in start..md.params.len() {
                    let mut parameter_type = match &descriptor {
                        Some(descriptor)
                            if has_descriptor
                                && (param_count as usize) < descriptor.parameter_types().len() =>
                        {
                            descriptor.parameter_types()[param_count as usize].clone()
                        }
                        _ => md.params[i].clone(),
                    };
                    if mask.as_ref().is_none_or(|m| m[i].is_none()) {
                        if !first {
                            buffer.append(",");
                            buffer.append_possible_newline_alt(" ");
                        }
                        first = false;

                        let nullable =
                            KotlinWriter::process_parameter_annotations(buffer, mt, param_count);

                        let is_var_arg = i as i32 == last_visible_parameter_index
                            && mt.has_modifier(ACC_VARARGS)
                            && parameter_type.array_dim > 0;
                        if is_var_arg {
                            buffer.append("vararg ");
                        }

                        let mut parameter_name: Option<String> = match &method_parameters {
                            Some(method_parameters) if i < method_parameters.len() => {
                                method_parameters[i].my_name.clone()
                            }
                            _ => varproc.get_var_name(VarVersionPair::new(index, 0)),
                        };

                        if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0 {
                            let new_parameter_name = method_struct
                                .get_variable_namer()
                                .rename_abstract_parameter(parameter_name.as_deref(), index)
                                .expect("NullPointerException: renameAbstractParameter");
                            parameter_name = if parameter_name.as_deref()
                                != Some(new_parameter_name.as_str())
                            {
                                Some(new_parameter_name)
                            } else {
                                DecompilerContext::get_struct_context().rename_abstract_parameter(
                                    &method_struct.get_class_qualified_name(),
                                    &mt.get_name(),
                                    &mt.get_descriptor(),
                                    index - if (flags & ACC_STATIC) == 0 { 1 } else { 0 },
                                    parameter_name.as_deref(),
                                )
                            };
                        }

                        let parameter_name = parameter_name
                            .as_deref()
                            .map(KotlinWriter::to_valid_kotlin_identifier);

                        match &parameter_name {
                            None => buffer.append(&format!("param{index}")),
                            Some(parameter_name) => buffer.append(parameter_name),
                        };
                        buffer.append(": ");

                        match &method_parameters {
                            Some(method_parameters) if i < method_parameters.len() => {
                                KotlinWriter::append_modifiers(
                                    buffer,
                                    method_parameters[i].my_access_flags,
                                    ACC_FINAL,
                                    is_interface,
                                    0,
                                );
                            }
                            _ => {
                                if varproc.get_var_final(VarVersionPair::new(index, 0))
                                    == FinalType::ExplicitFinal
                                {
                                    buffer.append("final ");
                                }
                            }
                        }

                        if is_var_arg {
                            parameter_type = parameter_type.decrease_array_dim();
                        }
                        let mut type_name = KTypes::get_kotlin_type(Some(&parameter_type));

                        if KTypes::is_function_type(&parameter_type) && nullable {
                            type_name = format!("({type_name})");
                        }

                        buffer.append(&type_name);

                        if nullable {
                            buffer.append("?");
                        }

                        param_count += 1;
                    }

                    index += parameter_type.stack_size;
                }
                buffer.pop_newline_group();

                if last_visible_parameter_index != -1 {
                    buffer.append_possible_newline_dedent("", true);
                    buffer.pop_newline_group();
                }
                buffer.append(")");

                let ret_type = match &descriptor {
                    None => md.ret.clone(),
                    Some(descriptor) => descriptor
                        .return_type()
                        .expect("NullPointerException: returnType"),
                };
                if !init && !ret_type.higher_equal_in_lattice_than(&VarType::VARTYPE_VOID()) {
                    buffer.append(": ");
                    let mut ret = KTypes::get_kotlin_type(Some(&ret_type));
                    if KTypes::is_function_type(&ret_type) && KotlinWriter::is_nullable(mt.0) {
                        ret = format!("({ret})");
                    }
                    buffer.append(&ret);
                    if KotlinWriter::is_nullable(mt.0) {
                        buffer.append("?");
                    }
                }
            }

            if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0 {
                buffer.append_line_separator();
            } else {
                if !cl_init && !d_init {
                    buffer.append(" ");
                }

                let hide_if_init = cl_init
                    || d_init
                    || KotlinWriter::hide_constructor(
                        node,
                        init,
                        throws_exceptions,
                        param_count,
                        flags,
                    );

                hide_method = !KotlinWriter::write_method_body(
                    cl,
                    method_wrapper,
                    buffer,
                    indent,
                    hide_if_init,
                );
            }
        }

        !hide_method
    }

    pub fn write_method_body(
        cl: StructClass,
        method_wrapper: MethodWrapper,
        buffer: &mut TextBuffer,
        indent: i32,
        hide_if_init: bool,
    ) -> bool {
        let mut hide_method = true;
        let mt = method_wrapper.with(|d| d.method_struct);

        buffer.append("{").append_line_separator();

        let root = method_wrapper.with(|d| d.root);

        if let Some(root) = root.filter(|_| method_wrapper.with(|d| d.decompile_error.is_none())) {
            let result = try_catch(|| {
                if !RecordHelper::is_hidden_record_method(cl, mt, root) {
                    let mut code = root.to_java(indent + 1);
                    code.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                    hide_method = code.length() == 0 && hide_if_init;
                    buffer.append_buffer_mapped(
                        &code,
                        Some(&cl.qualified_name()),
                        Some(&InterpreterUtil::make_unique_key(
                            &mt.get_name(),
                            &mt.get_descriptor(),
                        )),
                    );
                }
            });
            if let Err(t) = result {
                if is_stop_signal(&t) {
                    rethrow(t);
                }
                let message = format!(
                    "Method {} {} in class {} couldn't be written.",
                    mt.get_name(),
                    mt.get_descriptor(),
                    cl.qualified_name()
                );
                DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);
                method_wrapper.with_mut(|d| d.decompile_error = Some(t));
            }
        }

        if method_wrapper.with(|d| d.decompile_error.is_some()) {
            KotlinWriter::dump_error(buffer, method_wrapper, indent + 1);
        }
        buffer
            .append_indent(indent)
            .append("}")
            .append_line_separator();
        !hide_method
    }

    pub fn dump_error(buffer: &mut TextBuffer, wrapper: MethodWrapper, indent: i32) {
        kuna_spans::method_failed(wrapper);
        let mut lines: Vec<String> = vec!["$VF: Couldn't be decompiled".to_string()];
        let exceptions =
            DecompilerContext::get_option(IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR);
        let bytecode =
            DecompilerContext::get_option(IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR);
        if exceptions {
            lines.extend(KotlinWriter::get_error_comment());
            let error = wrapper
                .with(|d| d.decompile_error.clone())
                .expect("NullPointerException: decompileError");
            KotlinWriter::collect_error_lines(&error, &mut lines);
            if bytecode {
                lines.push(String::new());
            }
        }
        if bytecode {
            let method_struct = wrapper.with(|d| d.method_struct);
            let result = try_catch(|| {
                lines.push("Bytecode:".to_string());
                KotlinWriter::collect_bytecode(wrapper, &mut lines);
            });
            if let Err(e) = result {
                if is_java_error(&e) {
                    method_struct.release_resources();
                    rethrow(e);
                }
                lines.push("Error collecting bytecode:".to_string());
                KotlinWriter::collect_error_lines(&e, &mut lines);
            }
            method_struct.release_resources();
        }
        for line in &lines {
            buffer.append_indent(indent);
            buffer.append("//");
            if !line.is_empty() {
                buffer.append(" ").append(line);
            }
            buffer.append_line_separator();
        }
    }

    /// The `DUMP_EXCEPTION_ON_ERROR` lines describing `error`, as `ClassWriter` writes them.
    pub fn collect_error_lines(error: &Caught, lines: &mut Vec<String>) {
        ClassWriter::collect_error_lines(error, lines);
    }

    fn collect_bytecode(wrapper: MethodWrapper, lines: &mut Vec<String>) {
        let class_node = DecompilerContext::get_current_class_node()
            .expect("NullPointerException: CURRENT_CLASS_NODE");
        let method = wrapper.with(|d| d.method_struct);
        let mut instructions = method.get_instruction_sequence();
        if instructions.is_none() {
            method.expand_data(class_node.with(|d| d.class_struct));
            instructions = method.get_instruction_sequence();
        }
        let instructions = instructions.expect("NullPointerException: instructions");
        let last_offset = instructions.get_last().start_offset();
        let digits = 8 - (last_offset as u32).leading_zeros() as i32 / 4;
        let class_struct = class_node.with(|d| d.class_struct);
        let pool = class_struct.get_pool();
        let bootstrap =
            class_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS);

        for instr in instructions.iter() {
            let mut sb = String::new();
            let off_hex = int_to_hex_string(instr.start_offset());
            sb.push_str(&"0".repeat((digits - off_hex.len() as i32).max(0) as usize));
            sb.push_str(&off_hex);
            sb.push_str(": ");
            if instr.wide() {
                sb.push_str("wide ");
            }
            sb.push_str(TextUtil::get_instruction_name(instr.opcode()));
            match instr.group() {
                GROUP_INVOCATION => {
                    sb.push(' ');
                    match bootstrap.filter(|_| instr.opcode() == OPC_INVOKEDYNAMIC) {
                        Some(bootstrap) => {
                            let target = pool
                                .get_link_constant(instr.operand(0))
                                .expect("NullPointerException: invokedynamic target");
                            KotlinWriter::append_bootstrap_call(&mut sb, &target, bootstrap);
                        }
                        None => {
                            KotlinWriter::append_constant(
                                &mut sb,
                                pool.get_constant(instr.operand(0)).as_ref(),
                            );
                        }
                    }
                    for i in 1..instr.operands_count() {
                        sb.push(' ');
                        sb.push_str(&instr.operand(i).to_string());
                    }
                }
                GROUP_FIELDACCESS => {
                    sb.push(' ');
                    KotlinWriter::append_constant(
                        &mut sb,
                        pool.get_constant(instr.operand(0)).as_ref(),
                    );
                }
                GROUP_JUMP => {
                    sb.push(' ');
                    let dest = instr.start_offset().wrapping_add(instr.operand(0));
                    let dest_hex = int_to_hex_string(dest);
                    sb.push_str(&"0".repeat((digits - dest_hex.len() as i32).max(0) as usize));
                    sb.push_str(&dest_hex);
                }
                _ => match instr.opcode() {
                    OPC_NEW | OPC_CHECKCAST | OPC_INSTANCEOF | OPC_LDC | OPC_LDC_W | OPC_LDC2_W => {
                        sb.push(' ');
                        let constant = pool
                            .get_constant(instr.operand(0))
                            .expect("NullPointerException: constant");
                        match bootstrap.filter(|_| constant.type_() == CONSTANT_DYNAMIC) {
                            Some(bootstrap) => {
                                KotlinWriter::append_bootstrap_call(
                                    &mut sb,
                                    &constant.as_link(),
                                    bootstrap,
                                );
                            }
                            None => KotlinWriter::append_constant(&mut sb, Some(&constant)),
                        }
                    }
                    _ => {
                        for i in 0..instr.operands_count() {
                            sb.push(' ');
                            sb.push_str(&instr.operand(i).to_string());
                        }
                    }
                },
            }
            lines.push(sb);
        }
    }

    fn append_bootstrap_call(
        sb: &mut String,
        target: &LinkConstant,
        bootstrap: StructBootstrapMethodsAttribute,
    ) {
        sb.push_str(&target.elementname);
        sb.push(' ');
        sb.push_str(&target.descriptor);

        let bsm = bootstrap.get_method_reference(target.index1);
        let bsm_args = bootstrap.get_method_arguments(target.index1);

        sb.push_str(" bsm=");
        KotlinWriter::append_constant(sb, bsm.map(PooledConstant::Link).as_ref());
        sb.push_str(" args=[ ");
        let mut first = true;
        for arg in &bsm_args {
            if !first {
                sb.push_str(", ");
            }
            first = false;
            KotlinWriter::append_constant(sb, arg.as_ref());
        }
        sb.push_str(" ]");
    }

    fn append_constant(sb: &mut String, constant: Option<&PooledConstant>) {
        let Some(constant) = constant else {
            sb.push_str("<null constant>");
            return;
        };
        match constant {
            PooledConstant::Primitive(prim) => {
                let string_value = match &prim.value {
                    None => "null".to_string(),
                    Some(value) => value.to_java_string(),
                };
                if prim.type_ == CONSTANT_CLASS {
                    sb.push_str(&string_value);
                } else if prim.type_ == CONSTANT_STRING {
                    sb.push('"');
                    sb.push_str(&ConstExprent::convert_string_to_java(&string_value, false));
                    sb.push('"');
                } else {
                    sb.push_str(&string_value);
                }
            }
            PooledConstant::Link(link_constant) => {
                sb.push_str(or_null(link_constant.classname.as_deref()));
                sb.push('.');
                sb.push_str(&link_constant.elementname);
                sb.push(' ');
                sb.push_str(&link_constant.descriptor);
            }
        }
    }

    pub fn hide_constructor(
        node: ClassNode,
        init: bool,
        throws_exceptions: bool,
        param_count: i32,
        method_access_flags: i32,
    ) -> bool {
        if !init
            || throws_exceptions
            || param_count > 0
            || !DecompilerContext::get_option(IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR)
        {
            return false;
        }

        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();

        let class_access_flags = if node.with(|d| d.type_) == ClassNodeType::Root {
            cl.get_access_flags()
        } else {
            node.with(|d| d.access)
        };
        let is_enum = cl.has_modifier(ACC_ENUM)
            && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);

        if !is_enum
            && ((class_access_flags & KotlinWriter::ACCESSIBILITY_FLAGS)
                != (method_access_flags & KotlinWriter::ACCESSIBILITY_FLAGS))
        {
            return false;
        }

        let mut count = 0;
        for mt in cl.get_methods().to_vec() {
            if INIT_NAME == mt.get_name() {
                count += 1;
                if count > 1 {
                    return false;
                }
            }
        }

        true
    }

    fn get_field_type_data(fd: StructField) -> (VarType, Option<GenericFieldDescriptor>) {
        let field_type = VarType::new_signature_cl(&fd.get_descriptor(), false);

        let descriptor = fd.get_signature();
        (field_type, descriptor)
    }

    fn contains_deprecated_annotation(mb: StructMember) -> bool {
        for key in KotlinWriter::ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    if annotation.get_class_name() == "java/lang/Deprecated" {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn append_deprecation(buffer: &mut TextBuffer, indent: i32) {
        buffer
            .append_indent(indent)
            .append("/** @deprecated */")
            .append_line_separator();
    }

    fn append_rename_comment(
        buffer: &mut TextBuffer,
        old_name: Option<&str>,
        type_: MType,
        indent: i32,
    ) {
        let Some(old_name) = old_name else {
            return;
        };

        buffer.append_indent(indent);
        buffer.append("// $VF: renamed from: ");

        match type_ {
            MType::Class => {
                buffer.append(&ExprProcessor::build_java_class_name(old_name));
            }
            MType::Field => {
                let f_parts = java_split(old_name, ' ');
                let fd = FieldDescriptor::parse_descriptor(f_parts[2]);
                buffer.append(f_parts[1]);
                buffer.append(" ");
                buffer.append(&KotlinWriter::get_type_print_out(&fd.type_));
            }
            MType::Method => {
                let m_parts = java_split(old_name, ' ');
                let md = MethodDescriptor::parse_descriptor(m_parts[2]);
                buffer.append(m_parts[1]);
                buffer.append(" (");
                let mut first = true;
                for param_type in &md.params {
                    if !first {
                        buffer.append(", ");
                    }
                    first = false;
                    buffer.append(&KotlinWriter::get_type_print_out(param_type));
                }
                buffer.append(") ");
                buffer.append(&KotlinWriter::get_type_print_out(&md.ret));
            }
        }

        buffer.append_line_separator();
    }

    fn get_type_print_out(type_: &VarType) -> String {
        let mut type_text = ExprProcessor::get_cast_type_name_short(type_, false);
        if ExprProcessor::UNDEFINED_TYPE_STRING == type_text
            && DecompilerContext::get_option(IFernflowerPreferences::UNDEFINED_PARAM_TYPE_OBJECT)
        {
            type_text = ExprProcessor::get_cast_type_name_short(&VarType::VARTYPE_OBJECT(), false);
        }
        type_text
    }

    /// The non-empty lines of the `ERROR_MESSAGE` property.
    pub fn get_error_comment() -> Vec<String> {
        let message = DecompilerContext::get_property(IFernflowerPreferences::ERROR_MESSAGE)
            .expect("NullPointerException: ERROR_MESSAGE");
        message
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect()
    }

    fn append_comment(buffer: &mut TextBuffer, comment: &str, indent: i32) {
        buffer
            .append_indent(indent)
            .append("// $VF: ")
            .append(comment)
            .append_line_separator();
    }

    #[allow(dead_code)]
    fn append_inline_comment(buffer: &mut TextBuffer, comment: &str) {
        buffer.append("/* $VF: ").append(comment).append(" */");
    }

    fn append_javadoc(buffer: &mut TextBuffer, java_doc: Option<&str>, indent: i32) {
        let Some(java_doc) = java_doc else {
            return;
        };
        buffer
            .append_indent(indent)
            .append("/**")
            .append_line_separator();
        for s in java_split(java_doc, '\n') {
            buffer
                .append_indent(indent)
                .append(" * ")
                .append(s)
                .append_line_separator();
        }
        buffer
            .append_indent(indent)
            .append(" */")
            .append_line_separator();
    }

    pub fn append_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
    ) {
        let mut filter: JHashSet<String> = JHashSet::new();

        for key in KotlinWriter::ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    let class_name = annotation.get_class_name();
                    if class_name == "kotlin/Metadata"
                        || class_name == NOT_NULL_ANN_NAME
                        || class_name == NULLABLE_ANN_NAME
                        || class_name == "kotlin/jvm/JvmStatic"
                    {
                        continue;
                    }

                    let k_annotation = KAnnotationExprent::new(annotation);

                    let text = KAnnotationExprent::to_java(k_annotation, indent)
                        .convert_to_string_and_allow_data_discard();
                    filter.add(text.clone());
                    buffer.append(&text);
                    if indent < 0 {
                        buffer.append(" ");
                    } else {
                        buffer.append_line_separator();
                    }
                }
            }
        }

        KotlinWriter::append_type_annotations(buffer, indent, mb, target_type, -1, &filter);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn append_jvm_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        is_interface: bool,
        is_in_file: bool,
        pool: ConstantPool,
        target_type: i32,
    ) {
        match target_type {
            TypeAnnotation::METHOD_RETURN_TYPE => {
                if is_interface && !mb.has_modifier(ACC_ABSTRACT) {
                    buffer
                        .append_indent(indent)
                        .append("@JvmDefault")
                        .append_line_separator();
                }
                if mb.has_modifier(ACC_SYNCHRONIZED) {
                    buffer
                        .append_indent(indent)
                        .append("@Synchronized")
                        .append_line_separator();
                }
                if mb.has_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS) {
                    let attrib = mb
                        .get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS)
                        .expect("Exceptions attribute");
                    buffer.append_indent(indent).append("@Throws(");
                    buffer.push_newline_group(indent, 1);
                    let mut first = true;
                    for i in attrib.get_throws_exceptions() {
                        if !first {
                            buffer.append(",").append_possible_newline_alt(" ");
                        }
                        first = false;
                        let name = pool
                            .get_primitive_constant(i)
                            .expect("NullPointerException: exception class constant")
                            .get_string()
                            .to_string();
                        buffer.append(&name).append("::class");
                    }
                    buffer.pop_newline_group();
                    buffer.append(")").append_line_separator();
                }
            }
            TypeAnnotation::FIELD => {
                if mb.has_modifier(ACC_TRANSIENT) {
                    buffer
                        .append_indent(indent)
                        .append("@Transient")
                        .append_line_separator();
                }
                if mb.has_modifier(ACC_VOLATILE) {
                    buffer
                        .append_indent(indent)
                        .append("@Volatile")
                        .append_line_separator();
                }
            }
            TypeAnnotation::CLASS_TYPE_PARAMETER => {
                if mb.has_attribute(StructGeneralAttribute::ATTRIBUTE_RECORD) {
                    buffer
                        .append_indent(indent)
                        .append("@JvmRecord")
                        .append_line_separator();
                }
            }
            _ => {}
        }

        if mb.has_modifier(ACC_STATIC)
            && target_type != TypeAnnotation::CLASS_TYPE_PARAMETER
            && !is_in_file
            && !mb.has_modifier(ACC_ENUM)
        {
            buffer
                .append_indent(indent)
                .append("@JvmStatic")
                .append_line_separator();
        }
        if mb.has_modifier(ACC_STRICT) {
            buffer
                .append_indent(indent)
                .append("@Strictfp")
                .append_line_separator();
        }
        if mb.has_modifier(ACC_SYNTHETIC) {
            buffer
                .append_indent(indent)
                .append("@JvmSynthetic")
                .append_line_separator();
        }
    }

    pub fn is_nullable(mb: StructMember) -> bool {
        for key in KotlinWriter::ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                return attribute
                    .get_annotations()
                    .iter()
                    .any(|annotation| annotation.get_class_name() == NULLABLE_ANN_NAME);
            }
        }
        false
    }

    pub fn search_for_method(
        cl: Option<StructClass>,
        name: &str,
        md: &MethodDescriptor,
        search: bool,
    ) -> bool {
        let Some(cl) = cl else {
            return false;
        };

        let methods = cl.get_methods();

        if search {
            for method in methods.to_vec() {
                if md.equals(&MethodDescriptor::parse_descriptor(
                    &method.get_descriptor(),
                )) && name == method.get_name()
                    && !method.has_modifier(ACC_STATIC)
                {
                    return true;
                }
            }
        }

        if let Some(super_class) = cl.super_class() {
            let super_class =
                DecompilerContext::get_struct_context().get_class(&super_class.get_string());

            let found_in_super_class = KotlinWriter::search_for_method(super_class, name, md, true);

            if found_in_super_class {
                return true;
            }
        }

        for iface_name in cl.get_interface_names() {
            let iface = DecompilerContext::get_struct_context().get_class(&iface_name);

            let found_in_iface = KotlinWriter::search_for_method(iface, name, md, true);

            if found_in_iface {
                return true;
            }
        }

        false
    }

    pub fn process_parameter_annotations(
        buffer: &mut TextBuffer,
        mt: StructMethod,
        param: i32,
    ) -> bool {
        let mut filter: JHashSet<String> = JHashSet::new();
        let mut ret = false;

        for key in KotlinWriter::PARAMETER_ANNOTATION_ATTRIBUTES {
            let attribute = mt.get_attribute(key);
            if let Some(attribute) = attribute {
                let annotations = attribute.get_param_annotations();
                if (param as usize) < annotations.len() {
                    for &annotation in &annotations[param as usize] {
                        let class_name = annotation.get_class_name();
                        if class_name == NOT_NULL_ANN_NAME {
                            continue;
                        } else if class_name == NULLABLE_ANN_NAME {
                            ret = true;
                            continue;
                        }
                        let text = annotation
                            .0
                            .to_java(-1)
                            .convert_to_string_and_allow_data_discard();
                        filter.add(text.clone());
                        buffer.append(&text).append(" ");
                    }
                }
            }
        }

        KotlinWriter::append_type_annotations(
            buffer,
            -1,
            mt.0,
            TypeAnnotation::METHOD_PARAMETER,
            param,
            &filter,
        );
        ret
    }

    fn append_type_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
        index: i32,
        filter: &JHashSet<String>,
    ) {
        for key in KotlinWriter::TYPE_ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    if annotation.is_top_level()
                        && annotation.get_target_type() == target_type
                        && (index < 0 || annotation.get_index() == index)
                    {
                        let text = annotation
                            .get_annotation()
                            .0
                            .to_java(indent)
                            .convert_to_string_and_allow_data_discard();
                        if !filter.contains(&text) {
                            buffer.append(&text);
                            if indent < 0 {
                                buffer.append(" ");
                            } else {
                                buffer.append_line_separator();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn append_modifiers(
        buffer: &mut TextBuffer,
        flags: i32,
        allowed: i32,
        is_interface: bool,
        excluded: i32,
    ) {
        let flags = flags & allowed;
        let excluded = if is_interface { excluded } else { 0 };
        for (modifier, name) in MODIFIERS {
            if (flags & modifier) == modifier && (modifier & excluded) == 0 {
                buffer.append(name).append(" ");
            }
        }
    }

    pub fn append_type_parameters(
        buffer: &mut TextBuffer,
        parameters: &[String],
        bounds: &[Vec<VarType>],
    ) {
        buffer.append("<");

        for i in 0..parameters.len() {
            if i > 0 {
                buffer.append(", ");
            }

            buffer.append(&parameters[i]);

            let parameter_bounds = &bounds[i];
            if parameter_bounds.len() > 1
                || parameter_bounds[0].value.as_deref() != Some("java/lang/Object")
            {
                buffer.append(" extends ");
                buffer.append(&ExprProcessor::get_cast_type_name(&parameter_bounds[0]));
                for bound in &parameter_bounds[1..] {
                    buffer.append(" & ");
                    buffer.append(&ExprProcessor::get_cast_type_name(bound));
                }
            }
        }

        buffer.append(">");
    }
}

impl StatementWriter for KotlinWriter {
    fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        KotlinWriter::write_class_header(self, cl, buffer, import_collector)
    }

    fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        KotlinWriter::write_class(self, node, buffer, indent)
    }

    fn write_field(
        &self,
        wrapper: ClassWrapper,
        cl: StructClass,
        fd: StructField,
        buffer: &mut TextBuffer,
        indent: i32,
    ) {
        KotlinWriter::write_field(self, wrapper, cl, fd, buffer, indent)
    }

    fn write_method(
        &self,
        node: ClassNode,
        mt: StructMethod,
        method_index: i32,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        KotlinWriter::write_method(self, node, mt, method_index, buffer, indent)
    }

    fn ends_with_semicolon(&self, expr: Exprent) -> bool {
        KotlinWriter::ends_with_semicolon(self, expr)
    }
}
