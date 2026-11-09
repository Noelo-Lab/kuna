//! Port of `org/vineflower/kotlin/KotlinWriter.java`.

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::class_writer::IFabricJavadocProvider;
use crate::classes_processor::ClassNode;
use crate::code_constants::{
    ACC_ABSTRACT, ACC_FINAL, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
    ACC_STRICT, ACC_SYNCHRONIZED, ACC_TRANSIENT, ACC_VOLATILE,
};
use crate::constant_pool::ConstantPool;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::import_collector::ImportCollector;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::link_constant::LinkConstant;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::pool_interceptor::PoolInterceptor;
use crate::pooled_constant::PooledConstant;
use crate::rt::{BitSet, Caught, JHashSet};
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
use crate::unported;
use crate::var_type::VarType;

/// `KotlinWriter.MType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum MType {
    Class,
    Field,
    Method,
}

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

    pub fn ends_with_semicolon(&self, expr: Exprent) -> bool {
        false
    }

    fn invoke_processors(&self, buffer: &mut TextBuffer, node: ClassNode) -> bool {
        unported!("KotlinWriter.invokeProcessors")
    }

    pub fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        unported!("KotlinWriter.writeClassHeader")
    }

    pub fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        unported!("KotlinWriter.writeClass")
    }

    /// `function` may be `KFunction::FAILED_LAMBDA()` (compared with `Rc::ptr_eq`).
    fn write_lambda(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        function: &Rc<KFunction>,
    ) {
        unported!("KotlinWriter.writeLambda")
    }

    pub fn stringify_reference(
        indent: i32,
        node: ClassNode,
        bytecode: Option<&BitSet>,
        receiver: Option<Exprent>,
    ) -> Option<TextBuffer> {
        unported!("KotlinWriter.stringifyReference")
    }

    fn write_annotation_definition(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        kt_data: Option<&KElement>,
    ) {
        unported!("KotlinWriter.writeAnnotationDefinition")
    }

    fn write_class_definition(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        indent: i32,
        kt_data: Option<&KElement>,
        kotlin_flags: i32,
    ) {
        unported!("KotlinWriter.writeClassDefinition")
    }

    /// Java: writeField(TextBuffer, int, StructField, ClassWrapper)
    fn write_field_to_buffer(
        &self,
        buffer: &mut TextBuffer,
        indent: i32,
        fd: StructField,
        wrapper: ClassWrapper,
    ) {
        unported!("KotlinWriter.writeField")
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
        unported!("KotlinWriter.writeField")
    }

    /// `null` in gives `null` out in Java: map over the `Option` at the call site.
    pub fn to_valid_kotlin_identifier(name: &str) -> String {
        unported!("KotlinWriter.toValidKotlinIdentifier")
    }

    pub fn write_method(
        &self,
        node: ClassNode,
        mt: StructMethod,
        method_index: i32,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        unported!("KotlinWriter.writeMethod")
    }

    pub fn write_method_body(
        cl: StructClass,
        method_wrapper: MethodWrapper,
        buffer: &mut TextBuffer,
        indent: i32,
        hide_if_init: bool,
    ) -> bool {
        unported!("KotlinWriter.writeMethodBody")
    }

    pub fn dump_error(buffer: &mut TextBuffer, wrapper: MethodWrapper, indent: i32) {
        unported!("KotlinWriter.dumpError")
    }

    pub fn collect_error_lines(error: &Caught, lines: &mut Vec<String>) {
        unported!("KotlinWriter.collectErrorLines")
    }

    fn collect_bytecode(wrapper: MethodWrapper, lines: &mut Vec<String>) {
        unported!("KotlinWriter.collectBytecode")
    }

    fn append_bootstrap_call(
        sb: &mut String,
        target: &LinkConstant,
        bootstrap: StructBootstrapMethodsAttribute,
    ) {
        unported!("KotlinWriter.appendBootstrapCall")
    }

    fn append_constant(sb: &mut String, constant: Option<&PooledConstant>) {
        unported!("KotlinWriter.appendConstant")
    }

    pub fn hide_constructor(
        node: ClassNode,
        init: bool,
        throws_exceptions: bool,
        param_count: i32,
        method_access_flags: i32,
    ) -> bool {
        unported!("KotlinWriter.hideConstructor")
    }

    fn get_field_type_data(fd: StructField) -> (VarType, Option<GenericFieldDescriptor>) {
        let field_type = VarType::new_signature_cl(&fd.get_descriptor(), false);

        let descriptor = fd.get_signature();
        (field_type, descriptor)
    }

    fn contains_deprecated_annotation(mb: StructMember) -> bool {
        unported!("KotlinWriter.containsDeprecatedAnnotation")
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
        unported!("KotlinWriter.appendRenameComment")
    }

    fn get_type_print_out(type_: &VarType) -> String {
        unported!("KotlinWriter.getTypePrintOut")
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

    fn append_inline_comment(buffer: &mut TextBuffer, comment: &str) {
        buffer.append("/* $VF: ").append(comment).append(" */");
    }

    fn append_javadoc(buffer: &mut TextBuffer, java_doc: Option<&str>, indent: i32) {
        unported!("KotlinWriter.appendJavadoc")
    }

    pub fn append_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
    ) {
        unported!("KotlinWriter.appendAnnotations")
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
        unported!("KotlinWriter.appendJvmAnnotations")
    }

    pub fn is_nullable(mb: StructMember) -> bool {
        unported!("KotlinWriter.isNullable")
    }

    pub fn search_for_method(
        cl: Option<StructClass>,
        name: &str,
        md: &MethodDescriptor,
        search: bool,
    ) -> bool {
        unported!("KotlinWriter.searchForMethod")
    }

    pub fn process_parameter_annotations(
        buffer: &mut TextBuffer,
        mt: StructMethod,
        param: i32,
    ) -> bool {
        unported!("KotlinWriter.processParameterAnnotations")
    }

    fn append_type_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
        index: i32,
        filter: &JHashSet<String>,
    ) {
        unported!("KotlinWriter.appendTypeAnnotations")
    }

    fn append_modifiers(
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
        unported!("KotlinWriter.appendTypeParameters")
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
