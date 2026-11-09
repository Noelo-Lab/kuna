//! Port of `org/jetbrains/java/decompiler/main/ClassWriter.java`.
//!
//! `ClassWriter` implements the deferred `api/plugin/StatementWriter`; its
//! `writeClass`/`writeClassHeader` are plain methods here. The Fabric javadoc
//! hook (`net/fabricmc/fernflower/api`) is reduced to [`IFabricJavadocProvider`]
//! and [`FabricJavadocStyle`]; Vineflower reads the provider from an object
//! property that kuna's string properties cannot carry, so it is always absent.

#![allow(dead_code, unused_variables)]

use std::collections::VecDeque;
use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::code_constants::{
    ACC_ABSTRACT, ACC_FINAL, ACC_MANDATED, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED, ACC_SYNTHETIC, ACC_TRANSIENT, ACC_VOLATILE,
};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::import_collector::ImportCollector;
use crate::link_constant::LinkConstant;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::pair::Pair;
use crate::pool_interceptor::PoolInterceptor;
use crate::pooled_constant::PooledConstant;
use crate::rt::{Caught, JHashSet};
use crate::struct_annotation_attribute::StructAnnotationAttribute;
use crate::struct_annotation_parameter_attribute::StructAnnotationParameterAttribute;
use crate::struct_bootstrap_methods_attribute::StructBootstrapMethodsAttribute;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_general_attribute::{AttributeKey, StructGeneralAttribute};
use crate::struct_member::StructMember;
use crate::struct_method::StructMethod;
use crate::struct_module_attribute::StructModuleAttribute;
use crate::struct_type_annotation_attribute::StructTypeAnnotationAttribute;
use crate::text_buffer::TextBuffer;
use crate::type_annotation::PathValue;
use crate::unported;
use crate::var_type::VarType;

/// `net.fabricmc.fernflower.api.FabricJavadocStyle`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum FabricJavadocStyle {
    Html,
    Markdown,
}

/// `net.fabricmc.fernflower.api.IFabricJavadocProvider`.
pub trait IFabricJavadocProvider {
    fn get_class_doc(&self, struct_class: StructClass) -> Option<String>;

    fn get_field_doc(&self, struct_class: StructClass, struct_field: StructField)
        -> Option<String>;

    fn get_method_doc(
        &self,
        struct_class: StructClass,
        struct_method: StructMethod,
    ) -> Option<String>;

    fn get_class_javadoc_style(&self, struct_class: StructClass) -> FabricJavadocStyle {
        FabricJavadocStyle::Html
    }

    fn get_field_javadoc_style(
        &self,
        struct_class: StructClass,
        struct_field: StructField,
    ) -> FabricJavadocStyle {
        self.get_class_javadoc_style(struct_class)
    }

    fn get_method_javadoc_style(
        &self,
        struct_class: StructClass,
        struct_method: StructMethod,
    ) -> FabricJavadocStyle {
        self.get_class_javadoc_style(struct_class)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum MType {
    Class,
    Field,
    Method,
}

const ERROR_DUMP_STOP_POINTS: [&str; 5] = [
    "Fernflower.decompileContext",
    "MethodProcessor.codeToJava",
    "ClassWriter.writeMethod",
    "ClassWriter.methodLambdaToJava",
    "ClassWriter.classLambdaToJava",
];

const MODIFIERS: [(i32, &str); 11] = [
    (ACC_PUBLIC, "public"),
    (ACC_PROTECTED, "protected"),
    (ACC_PRIVATE, "private"),
    (ACC_ABSTRACT, "abstract"),
    (ACC_STATIC, "static"),
    (ACC_FINAL, "final"),
    (ACC_STRICT, "strictfp"),
    (ACC_TRANSIENT, "transient"),
    (ACC_VOLATILE, "volatile"),
    (ACC_SYNCHRONIZED, "synchronized"),
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

/// Writes classes, fields and methods as Java source.
pub struct ClassWriter {
    interceptor: Option<PoolInterceptor>,
    javadoc_provider: Option<Rc<dyn IFabricJavadocProvider>>,
}

impl ClassWriter {
    const CLASS_ALLOWED: i32 = ACC_PUBLIC
        | ACC_PROTECTED
        | ACC_PRIVATE
        | ACC_ABSTRACT
        | ACC_STATIC
        | ACC_FINAL
        | ACC_STRICT;
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

    pub fn new() -> ClassWriter {
        ClassWriter {
            interceptor: DecompilerContext::get_pool_interceptor(),
            javadoc_provider: None,
        }
    }

    pub fn ends_with_semicolon(&self, expr: Exprent) -> bool {
        !(expr.is_switch_head()
            || expr.is_monitor()
            || expr.is_if()
            || expr.try_as_var().is_some_and(|var| var.is_class_def()))
    }

    /// Late per-class processing before writing; false (with an error comment
    /// written) when the class cannot be written.
    fn invoke_processors(buffer: &mut TextBuffer, node: ClassNode) -> bool {
        unported!("ClassWriter.invokeProcessors")
    }

    pub fn write_exception(buffer: &mut TextBuffer, t: &Caught) {
        buffer.append("// $VF: Couldn't be decompiled");
        buffer.append_line_separator();
        if DecompilerContext::get_option(IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR) {
            let mut lines: Vec<String> = Vec::new();
            lines.extend(ClassWriter::get_error_comment());
            ClassWriter::collect_error_lines(t, &mut lines);
            for line in &lines {
                buffer.append("//");
                if !line.is_empty() {
                    buffer.append(" ").append(line);
                }
                buffer.append_line_separator();
            }
        }
    }

    pub fn class_lambda_to_java(
        &self,
        node: ClassNode,
        buffer: &mut TextBuffer,
        method_object: Option<Exprent>,
        indent: i32,
    ) {
        unported!("ClassWriter.classLambdaToJava")
    }

    pub fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        let qualified_name = cl.qualified_name();
        if let Some(index) = qualified_name.rfind('/') {
            let package_name = qualified_name[..index].replace('/', ".");
            buffer
                .append("package ")
                .append(&package_name)
                .append(";")
                .append_line_separator()
                .append_line_separator();
        }

        import_collector.write_imports(buffer, true);
    }

    pub fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        unported!("ClassWriter.writeClass")
    }

    pub fn package_info_to_java(cl: StructClass, buffer: &mut TextBuffer) {
        ClassWriter::append_annotations(buffer, 0, cl.0, -1);

        let qualified_name = cl.qualified_name();
        let index = qualified_name.rfind('/').unwrap();
        let package_name = qualified_name[..index].replace('/', ".");
        buffer
            .append("package ")
            .append(&package_name)
            .append(";")
            .append_line_separator()
            .append_line_separator();
    }

    pub fn module_info_to_java(cl: StructClass, buffer: &mut TextBuffer) {
        unported!("ClassWriter.moduleInfoToJava")
    }

    fn write_module_info_body(buffer: &mut TextBuffer, module_attribute: StructModuleAttribute) {
        unported!("ClassWriter.writeModuleInfoBody")
    }

    fn is_generated(flags: i32) -> bool {
        (flags & (ACC_SYNTHETIC | ACC_MANDATED)) != 0
    }

    fn write_class_definition(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        unported!("ClassWriter.writeClassDefinition")
    }

    fn is_super_class_sealed(cl: StructClass) -> bool {
        unported!("ClassWriter.isSuperClassSealed")
    }

    /// Java: writeField(TextBuffer, int, StructField, ClassWrapper)
    fn write_field_to_buffer(
        &self,
        buffer: &mut TextBuffer,
        indent: i32,
        fd: StructField,
        wrapper: ClassWrapper,
    ) {
        unported!("ClassWriter.writeField")
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
        unported!("ClassWriter.writeField")
    }

    fn method_lambda_to_java(
        lambda_node: ClassNode,
        class_wrapper: ClassWrapper,
        mt: StructMethod,
        buffer: &mut TextBuffer,
        indent: i32,
        code_only: bool,
    ) {
        unported!("ClassWriter.methodLambdaToJava")
    }

    pub fn to_valid_java_identifier(name: Option<&str>) -> Option<String> {
        unported!("ClassWriter.toValidJavaIdentifier")
    }

    /// Writes one method; false when it was skipped (nothing written).
    pub fn write_method(
        &self,
        node: ClassNode,
        mt: StructMethod,
        method_index: i32,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        unported!("ClassWriter.writeMethod")
    }

    /// Writes `(params)` and returns the number of parameters written.
    #[allow(clippy::too_many_arguments)]
    fn write_method_parameter_header(
        mt: StructMethod,
        buffer: &mut TextBuffer,
        indent: i32,
        method_wrapper: MethodWrapper,
        md: &MethodDescriptor,
        is_enum: bool,
        init: bool,
        this_var: bool,
        descriptor: Option<&GenericMethodDescriptor>,
        param_count: i32,
        is_interface: bool,
        flags: i32,
        cl: StructClass,
    ) -> i32 {
        unported!("ClassWriter.writeMethodParameterHeader")
    }

    fn dump_error(buffer: &mut TextBuffer, wrapper: MethodWrapper, indent: i32) {
        unported!("ClassWriter.dumpError")
    }

    /// The `DUMP_EXCEPTION_ON_ERROR` lines describing `error`.
    pub fn collect_error_lines(error: &Caught, lines: &mut Vec<String>) {
        unported!("ClassWriter.collectErrorLines")
    }

    fn collect_bytecode(wrapper: MethodWrapper, lines: &mut Vec<String>) {
        unported!("ClassWriter.collectBytecode")
    }

    fn append_bootstrap_call(
        sb: &mut String,
        target: &LinkConstant,
        bootstrap: StructBootstrapMethodsAttribute,
    ) {
        unported!("ClassWriter.appendBootstrapCall")
    }

    fn append_constant(sb: &mut String, constant: Option<&PooledConstant>) {
        unported!("ClassWriter.appendConstant")
    }

    fn hide_constructor(
        node: ClassNode,
        init: bool,
        throws_exceptions: bool,
        param_count: i32,
        method_access_flags: i32,
        struct_method: StructMethod,
    ) -> bool {
        unported!("ClassWriter.hideConstructor")
    }

    fn get_field_type_data(fd: StructField) -> (VarType, Option<GenericFieldDescriptor>) {
        unported!("ClassWriter.getFieldTypeData")
    }

    fn contains_deprecated_annotation(mb: StructMember) -> bool {
        unported!("ClassWriter.containsDeprecatedAnnotation")
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
        unported!("ClassWriter.appendRenameComment")
    }

    fn get_type_print_out(type_: &VarType) -> String {
        unported!("ClassWriter.getTypePrintOut")
    }

    /// The non-empty lines of the `ERROR_MESSAGE` property.
    pub fn get_error_comment() -> Vec<String> {
        let message =
            DecompilerContext::get_property(IFernflowerPreferences::ERROR_MESSAGE).unwrap();
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

    fn append_javadoc(
        buffer: &mut TextBuffer,
        javadoc_style: FabricJavadocStyle,
        java_doc: Option<&str>,
        indent: i32,
    ) {
        let Some(java_doc) = java_doc else {
            return;
        };
        match javadoc_style {
            FabricJavadocStyle::Html => {
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
            FabricJavadocStyle::Markdown => {
                for s in java_split(java_doc, '\n') {
                    buffer
                        .append_indent(indent)
                        .append("/// ")
                        .append(s)
                        .append_line_separator();
                }
            }
        }
    }

    pub fn append_synthetic_class_comment(cl: StructClass, buffer: &mut TextBuffer) {
        let qualified_name = cl.qualified_name();
        let class_name = &qualified_name[qualified_name.rfind('/').map_or(0, |i| i + 1)..];
        buffer
            .append(" /* ")
            .append_class(class_name, true, &qualified_name)
            .append(" */");
    }

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

    /// Writes the declaration annotations of `mb`; returns their texts, for filtering type annotations.
    pub fn append_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
    ) -> JHashSet<String> {
        unported!("ClassWriter.appendAnnotations")
    }

    /// Whether a non-static method `name`/`md` exists in `cl`'s supertypes (and `cl` itself when `search`).
    fn search_for_method(
        cl: Option<StructClass>,
        name: &str,
        md: &MethodDescriptor,
        search: bool,
    ) -> bool {
        unported!("ClassWriter.searchForMethod")
    }

    fn append_parameter_annotations(
        buffer: &mut TextBuffer,
        mt: StructMethod,
        param: i32,
    ) -> JHashSet<String> {
        unported!("ClassWriter.appendParameterAnnotations")
    }

    fn append_top_level_type_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
        index: i32,
        filter: &JHashSet<String>,
    ) {
        unported!("ClassWriter.appendTopLevelTypeAnnotations")
    }

    pub fn get_type_annotations(
        mb: StructMember,
        target_type: i32,
        index: i32,
    ) -> Vec<Pair<VecDeque<PathValue>, AnnotationExprent>> {
        unported!("ClassWriter.getTypeAnnotations")
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

    /// The modifier keywords present in `flags`, space-separated.
    pub fn get_modifiers(flags: i32) -> String {
        MODIFIERS
            .iter()
            .filter(|(modifier, _)| (modifier & flags) != 0)
            .map(|(_, name)| *name)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn append_type_parameters(
        buffer: &mut TextBuffer,
        parameters: &[String],
        bounds: &[Vec<VarType>],
    ) {
        unported!("ClassWriter.appendTypeParameters")
    }

    fn append_fq_class_names(buffer: &mut TextBuffer, names: &[String]) {
        for (i, name) in names.iter().enumerate() {
            buffer.append_indent(2).append(name);
            if i + 1 < names.len() {
                buffer.append(",").append_line_separator();
            }
        }
    }
}
