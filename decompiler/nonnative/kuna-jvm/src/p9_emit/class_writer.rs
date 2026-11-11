//! Port of `org/jetbrains/java/decompiler/main/ClassWriter.java`.
//!
//! `ClassWriter` implements the deferred `api/plugin/StatementWriter`; its
//! `writeClass`/`writeClassHeader` are plain methods here. The Fabric javadoc
//! hook (`net/fabricmc/fernflower/api`) is reduced to [`IFabricJavadocProvider`]
//! and [`FabricJavadocStyle`]; the provider object Vineflower reads from the
//! `fabric:javadoc` property is `DecompilerContext::get_javadoc_provider`.
//! A [`Caught`] has no stack trace or cause: `collectErrorLines` reports its
//! throw location as the only frame.
//!
//! kuna fix `statement_as_expression` (`kuna_fixes`): in
//! `class_lambda_to_java`, a lambda whose lone exprent is statement-only (an
//! `assert`) gets a block body, where Vineflower writes it as an expression body.
//! kuna fix `enum_body_separator` (`kuna_fixes`): in `write_class`, an
//! enum whose members have no constant before them gets the `;` Java requires
//! (`kuna_fix_enum_body_separator`).
//! kuna fix `override_inaccessible_method` (`kuna_fixes`): `@Override` is
//! written only for a supertype method the class can override
//! (`kuna_fix_override_inaccessible_method`).
//! kuna fix `sealed_private_permits`: a top-level sealed class whose permitted
//! subclasses are its own members, one of them private, is written without its
//! `permits` clause (`kuna_fix_sealed_private_permits`).
//! kuna fix `nested_param_rename`: a parameter whose uses are written by its
//! local-variable-table name is declared under it (`kuna_fix_nested_param_rename`).
//! kuna fix `captured_var_clash_name`: in `invoke_processors`, the captured
//! variables of the class's methods take the clashing names of the variables
//! they hold (`kuna_fix_captured_var_clash_name`).
//!
//! kuna fix `disjoint_generic_cast`: the value written here is marked as
//! one with a declared target type (`kuna_fix_disjoint_generic_cast`).

use std::collections::VecDeque;
use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::p2_lift::kuna_fix_disjoint_generic_cast;
use crate::cancelation_manager::CanceledException;
use crate::class_reference14_processor::ClassReference14Processor;
use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{
    ACC_ABSTRACT, ACC_ANNOTATION, ACC_BRIDGE, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_MANDATED,
    ACC_MODULE, ACC_NATIVE, ACC_OPEN, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
    ACC_STATIC_PHASE, ACC_STRICT, ACC_SYNCHRONIZED, ACC_SYNTHETIC, ACC_TRANSIENT, ACC_TRANSITIVE,
    ACC_VARARGS, ACC_VOLATILE, CLINIT_NAME, CONSTANT_CLASS, CONSTANT_DYNAMIC,
    CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL, CONSTANT_STRING, GROUP_FIELDACCESS,
    GROUP_INVOCATION, GROUP_JUMP, INIT_NAME, OPC_CHECKCAST, OPC_INSTANCEOF, OPC_INVOKEDYNAMIC,
    OPC_LDC, OPC_LDC2_W, OPC_LDC_W, OPC_NEW,
};
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::decompile_record::DecompileRecord;
use crate::decompiler_context::DecompilerContext;
use crate::enum_processor::EnumProcessor;
use crate::exit_exprent::Type as ExitType;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::function_exprent::FunctionType;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::generics_checker::GenericsChecker;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_helper::IfHelper;
use crate::import_collector::ImportCollector;
use crate::initializer_processor::InitializerProcessor;
use crate::interpreter_util::InterpreterUtil;
use crate::java_pass_location::JavaPassLocation;
use crate::kuna_fix_statement_as_expression;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_captured_var_clash_name;
use crate::kuna_spans;
use crate::link_constant::LinkConstant;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::p9_emit::kuna_fix_enum_body_separator::EnumBodySeparator;
use crate::p9_emit::kuna_fix_override_inaccessible_method;
use crate::p9_emit::kuna_fix_sealed_private_permits;
use crate::p9_emit::kuna_fix_nested_param_rename;
use crate::pair::Pair;
use crate::pass_context::PassContext;
use crate::pool_interceptor::PoolInterceptor;
use crate::pooled_constant::PooledConstant;
use crate::record_helper::RecordHelper;
use crate::rt::jchar::{is_java_identifier_part, is_java_identifier_start};
use crate::rt::jnum::int_to_hex_string;
use crate::rt::jstring::from_utf16;
use crate::rt::{rethrow, try_catch, Caught, JHashSet, JavaStr};
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
use crate::switch_helper::SwitchHelper;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::type_annotation::{PathValue, TypeAnnotation};
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;

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

    fn get_class_javadoc_style(&self, _struct_class: StructClass) -> FabricJavadocStyle {
        FabricJavadocStyle::Html
    }

    fn get_field_javadoc_style(
        &self,
        struct_class: StructClass,
        _struct_field: StructField,
    ) -> FabricJavadocStyle {
        self.get_class_javadoc_style(struct_class)
    }

    fn get_method_javadoc_style(
        &self,
        struct_class: StructClass,
        _struct_method: StructMethod,
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

#[allow(dead_code)]
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

/// A thrown `java.lang.Error` (which `catch (Exception e)` does not catch).
fn is_java_error(caught: &Caught) -> bool {
    caught
        .exception
        .as_ref()
        .is_some_and(|e| e.class.ends_with("Error"))
}

/// `finally { DecompilerContext.setProperty(CURRENT_CLASS_NODE, outer) }`.
struct RestoreClassNode(Option<ClassNode>);

impl Drop for RestoreClassNode {
    fn drop(&mut self) {
        if DecompilerContext::get_current_context().is_some() {
            DecompilerContext::set_current_class_node(self.0);
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

fn lambda_information(node: ClassNode) -> crate::classes_processor::LambdaInformation {
    node.with(|d| d.lambda_information.clone())
        .expect("NullPointerException: lambdaInformation")
}

/// Writes classes, fields and methods as Java source.
pub struct ClassWriter {
    interceptor: Option<PoolInterceptor>,
    javadoc_provider: Option<Rc<dyn IFabricJavadocProvider>>,
}

impl Default for ClassWriter {
    fn default() -> Self {
        ClassWriter::new()
    }
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
            javadoc_provider: DecompilerContext::get_javadoc_provider(),
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
        let Some(wrapper) = node.get_wrapper() else {
            let qualified_name = node.with(|d| d.class_struct).qualified_name();
            kuna_spans::class_failed(&qualified_name, &format!("Class {qualified_name} wasn't processed yet"));
            buffer.append(&format!(
                "/* $VF: Couldn't be decompiled. Class {qualified_name} wasn't processed yet! */"
            ));
            let mut lines: Vec<String> = Vec::new();
            lines.extend(ClassWriter::get_error_comment());
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

        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let (method_struct, class_struct, varproc) =
                method.with(|d| (d.method_struct, d.class_struct, d.varproc));
            let rec = DecompileRecord::new(method_struct);
            DecompilerContext::reset_method(method);

            if let Some(root) = method.with(|d| d.root) {
                let result = try_catch(|| {
                    if SwitchHelper::simplify_switches(root.0, method_struct, root) {
                        rec.add("FinalSimplifySwitches", Some(root));
                    }

                    if DecompilerContext::get_option(IFernflowerPreferences::PRETTIFY_IFS)
                        && IfHelper::prettify_ifs(root.0)
                    {
                        rec.add("FinalPrettifyIfs", Some(root));
                    }

                    let mut pctx = PassContext::new(
                        Some(root),
                        None,
                        method_struct,
                        class_struct,
                        varproc,
                        rec,
                    );
                    DecompilerContext::get_struct_context()
                        .get_plugin_context()
                        .run_passes(JavaPassLocation::LateProcessing, &mut pctx);
                });
                if let Err(e) = result {
                    if CanceledException::is(&e) {
                        rethrow(e);
                    }
                    let qualified_name = node.with(|d| d.class_struct).qualified_name();
                    DecompilerContext::get_logger().write_message_ex(
                        &format!(
                            "Method {} {} in class {} couldn't be written.",
                            method_struct.get_name(),
                            method_struct.get_descriptor(),
                            qualified_name
                        ),
                        Severity::Warn,
                        &e,
                    );
                    method.with_mut(|d| d.decompile_error = Some(e));
                }
            }
        }

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
                EnumProcessor::clear_enum(wrapper);
            }

            let methods = wrapper.get_methods().borrow().to_vec();
            for mw in methods {
                DecompilerContext::reset_method(mw);
                RecordHelper::fixup_canonical_constructor(mw, cl);
                if let Some(root) = mw.with(|d| d.root) {
                    mw.with(|d| d.varproc).rerun_clashing(root);
                }
            }
            if kuna_fixes::enabled(KunaFix::CapturedVarClashName) {
                kuna_fix_captured_var_clash_name::inherit_clashing_names(wrapper);
            }
        });
        if let Err(t) = result {
            if CanceledException::is(&t) {
                rethrow(t);
            }
            let simple_name = node.with(|d| d.simple_name.clone());
            DecompilerContext::get_logger().write_message_ex(
                &format!(
                    "Class {} couldn't be written.",
                    simple_name.as_deref().unwrap_or("null")
                ),
                Severity::Warn,
                &t,
            );
            kuna_spans::class_failed(&cl.qualified_name(), &t.message);
            ClassWriter::write_exception(buffer, &t);

            return false;
        }

        true
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
        let Some(wrapper) = node.get_wrapper() else {
            return;
        };

        let lambda_to_anonymous =
            DecompilerContext::get_option(IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS);

        let outer_node = DecompilerContext::get_current_class_node();
        DecompilerContext::set_current_class_node(Some(node));

        {
            let _finally = RestoreClassNode(outer_node);
            let cl = wrapper.get_class_struct();

            let simple_name = node.with(|d| d.simple_name.clone());
            DecompilerContext::get_logger()
                .start_write_class(simple_name.as_deref().unwrap_or("null"));

            let li = lambda_information(node);
            if li.is_method_reference {
                match method_object.filter(|_| !li.is_content_method_static) {
                    Some(method_object) => {
                        method_object.get_inferred_expr_type(Some(&VarType::new_value(
                            CodeType::Object,
                            0,
                            li.content_class_name.as_str(),
                        )));
                        let instance = method_object.to_java(indent);
                        if method_object.try_as_function().is_some_and(|f| {
                            f.get_func_type() == FunctionType::Cast && f.does_cast()
                        }) {
                            buffer.append("(").append_buffer(&instance).append(")");
                        } else {
                            buffer.append_buffer(&instance);
                        }
                    }
                    None => {
                        buffer.append_cast_type_name(&VarType::new_signature_cl(
                            &li.content_class_name,
                            true,
                        ));
                    }
                }

                let value = if INIT_NAME == li.content_method_name {
                    "new"
                } else {
                    li.content_method_name.as_str()
                };
                buffer.append("::").append_method(
                    value,
                    false,
                    &li.content_class_name,
                    &li.content_method_name,
                    &li.content_method_descriptor,
                );
            } else {
                let mt = cl
                    .get_method_key(&li.content_method_key)
                    .expect("NullPointerException: lambda content method");
                let method_wrapper =
                    wrapper.get_method_wrapper(&mt.get_name(), &mt.get_descriptor());
                let md_content = MethodDescriptor::parse_descriptor(&li.content_method_descriptor);
                let md_lambda = MethodDescriptor::parse_descriptor(&li.method_descriptor);

                let mut simple_lambda = false;
                let mut written = false;

                if !lambda_to_anonymous {
                    let root = wrapper
                        .get_method_wrapper(&mt.get_name(), &mt.get_descriptor())
                        .expect("NullPointerException: lambda method wrapper")
                        .with(|d| d.root);
                    if DecompilerContext::get_option(
                        IFernflowerPreferences::MARK_CORRESPONDING_SYNTHETICS,
                    ) {
                        buffer
                            .append("/* ")
                            .append_method(
                                &li.content_method_name,
                                true,
                                &li.content_class_name,
                                &li.content_method_name,
                                &li.content_method_descriptor,
                            )
                            .append(" */ ");
                    }
                    if md_lambda.params.len() == 1
                        && md_lambda.params[0].equals(&VarType::VARTYPE_INT())
                        && md_lambda.ret.array_dim > 0
                    {
                        let first = root.expect("NullPointerException: root").get_first();
                        if first.is_some_and(|f| f.is_basic_block())
                            && first.unwrap().get_exprents().expect("exprents").len() == 1
                        {
                            let exp = first.unwrap().get_exprents().expect("exprents").get(0);
                            if let Some(exit) = exp.try_as_exit() {
                                let return_value = exit.get_value();
                                if let Some(new_exp) = return_value.and_then(|v| v.try_as_new()) {
                                    if new_exp.get_new_type().array_dim > 0
                                        && !new_exp.is_direct_array_init()
                                        && new_exp.get_lst_array_elements().is_empty()
                                        && !new_exp.get_lst_dims().is_empty()
                                    {
                                        let dims = new_exp.get_lst_dims();
                                        let size = dims.get(dims.len() - 1);
                                        if let Some(size_var) = size.try_as_var() {
                                            let expected =
                                                if li.is_content_method_static { 0 } else { 1 };
                                            if size_var.get_index() == expected {
                                                let return_type = md_lambda.ret.clone();
                                                buffer.append_cast_type_name(&return_type);
                                                buffer.append("::new");
                                                written = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !written {
                        let lambda_parameters_need_parentheses = md_lambda.params.len() != 1;

                        if lambda_parameters_need_parentheses {
                            buffer.append("(");
                        }

                        let mut first_parameter = true;
                        let mut index: i32 = if li.is_content_method_static { 0 } else { 1 };
                        let start_index =
                            md_content.params.len() as i32 - md_lambda.params.len() as i32;

                        for i in 0..md_content.params.len() {
                            if i as i32 >= start_index {
                                if !first_parameter {
                                    buffer.append(", ");
                                }
                                let type_ = &md_content.params[i];

                                let mw =
                                    method_wrapper.expect("NullPointerException: methodWrapper");
                                let (varproc, method_struct) =
                                    mw.with(|d| (d.varproc, d.method_struct));
                                let clashing_name =
                                    varproc.get_clashing_name(VarVersionPair::new(index, 0));
                                let mut parameter_name =
                                    varproc.get_var_name(VarVersionPair::new(index, 0));
                                if parameter_name.is_none() {
                                    parameter_name = Some(format!("param{index}"));
                                }
                                if clashing_name.is_some() {
                                    parameter_name = clashing_name;
                                }
                                let parameter_name = method_struct
                                    .get_variable_namer()
                                    .rename_parameter(
                                        mt.get_access_flags(),
                                        type_,
                                        parameter_name.as_deref(),
                                        index,
                                    )
                                    .expect("NullPointerException: parameter name");
                                buffer.append_variable_descriptor(
                                    &parameter_name,
                                    true,
                                    true,
                                    &li.content_class_name,
                                    &li.content_method_name,
                                    &md_content,
                                    index,
                                    &parameter_name,
                                );

                                first_parameter = false;
                            }

                            index += md_content.params[i].stack_size;
                        }

                        if lambda_parameters_need_parentheses {
                            buffer.append(")");
                        }
                        buffer.append(" ->");

                        let inline = DecompilerContext::get_option(
                            IFernflowerPreferences::INLINE_SIMPLE_LAMBDAS,
                        ) && method_wrapper
                            .expect("NullPointerException: methodWrapper")
                            .with(|d| d.decompile_error.is_none());
                        if let Some(root) = root.filter(|_| inline) {
                            let first_stat = root.get_first();
                            let is_basic_block = first_stat.is_some_and(|s| s.is_basic_block());
                            let exprents = if is_basic_block {
                                first_stat.unwrap().get_exprents()
                            } else {
                                None
                            };
                            if is_basic_block && exprents.as_ref().is_some_and(|e| e.len() == 1) {
                                let first_expr = exprents.unwrap().get(0);
                                let is_var_definition =
                                    first_expr.try_as_assignment().is_some_and(|a| {
                                        a.get_left().try_as_var().is_some_and(|v| v.is_definition())
                                    });

                                let is_throw = first_expr
                                    .try_as_exit()
                                    .is_some_and(|e| e.get_exit_type() == ExitType::Throw);

                                let is_expression =
                                    if kuna_fixes::enabled(KunaFix::StatementAsExpression) {
                                        kuna_fix_statement_as_expression::is_lambda_expression_body(
                                            first_expr,
                                        )
                                    } else {
                                        !is_var_definition && !is_throw
                                    };

                                if is_expression {
                                    simple_lambda = true;
                                    let mw = method_wrapper.unwrap();
                                    let outer_wrapper =
                                        DecompilerContext::get_current_method_wrapper();
                                    DecompilerContext::set_current_method_wrapper(Some(mw));
                                    let _finally = RestoreMethodWrapper(outer_wrapper);
                                    let node_class_name =
                                        node.with(|d| d.class_struct).qualified_name();
                                    let result = try_catch(|| {
                                        let mut code_buffer = first_expr.to_java(indent);

                                        if first_expr.is_exit() {
                                            code_buffer.set_start(6);
                                        } else {
                                            code_buffer.prepend(" ");
                                        }

                                        code_buffer.add_bytecode_mapping(
                                            root.get_dummy_exit().bytecode().as_ref(),
                                        );
                                        let method_struct = mw.with(|d| d.method_struct);
                                        buffer.append_buffer_mapped(
                                            &code_buffer,
                                            Some(&node_class_name),
                                            Some(&InterpreterUtil::make_unique_key(
                                                &method_struct.get_name(),
                                                &method_struct.get_descriptor(),
                                            )),
                                        );
                                    });
                                    if let Err(ex) = result {
                                        if CanceledException::is(&ex) {
                                            rethrow(ex);
                                        }
                                        DecompilerContext::get_logger().write_message_ex(
                                            &format!(
                                                "Method {} {} in class {} couldn't be written.",
                                                mt.get_name(),
                                                mt.get_descriptor(),
                                                node_class_name
                                            ),
                                            Severity::Warn,
                                            &ex,
                                        );
                                        mw.with_mut(|d| d.decompile_error = Some(ex));
                                        kuna_spans::method_failed(mw);
                                        buffer.append(" // $VF: Couldn't be decompiled");
                                    }
                                }
                            } else if is_basic_block
                                && exprents.as_ref().is_some_and(|e| e.is_empty())
                            {
                                buffer.append(" {}");
                                simple_lambda = true;
                            }
                        }
                    }
                }

                if (!simple_lambda && !written) || lambda_to_anonymous {
                    buffer.append(" {").append_line_separator();

                    ClassWriter::method_lambda_to_java(
                        node,
                        wrapper,
                        mt,
                        buffer,
                        indent + 1,
                        !lambda_to_anonymous,
                    );

                    buffer.append_indent(indent).append("}");
                }
            }
        }

        DecompilerContext::get_logger().end_write_class();
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
        let outer_node = DecompilerContext::get_current_class_node();
        DecompilerContext::set_current_class_node(Some(node));

        {
            let _finally = RestoreClassNode(outer_node);
            let span = kuna_spans::open_class(buffer, node.with(|d| d.class_struct));

            let ok = ClassWriter::invoke_processors(buffer, node);

            if !ok {
                kuna_spans::close_failed(buffer, span);
                return;
            }

            let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
            let cl = wrapper.get_class_struct();

            DecompilerContext::get_logger().start_write_class(&cl.qualified_name());

            if DecompilerContext::get_option(IFernflowerPreferences::SOURCE_FILE_COMMENTS) {
                let class_struct = node.with(|d| d.class_struct);
                let source_file_attr =
                    class_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_SOURCE_FILE);

                if let Some(source_file_attr) = source_file_attr {
                    let pool = class_struct.get_pool();
                    let source_file = source_file_attr.get_source_file(pool);

                    buffer
                        .append_indent(indent)
                        .append(&format!("// $VF: Compiled from {source_file}"))
                        .append_line_separator();
                }
            }

            self.write_class_definition(node, buffer, indent);

            let mut has_content = false;
            let mut enum_separator = EnumBodySeparator::new(node, indent);
            let have_content = |buffer: &mut TextBuffer, has_content: &mut bool| {
                if !*has_content && node.with(|d| d.type_) == ClassNodeType::Anonymous {
                    buffer.append_line_separator();
                }
                *has_content = true;
            };

            let method_to_decompile =
                DecompilerContext::get_property(IFernflowerPreferences::METHOD_TO_DECOMPILE)
                    .expect("NullPointerException: method-to-decompile");

            if method_to_decompile.is_empty() {
                let components = cl.get_record_components();

                let mut enum_fields: Vec<StructField> = Vec::new();
                let mut non_enum_fields: Vec<StructField> = Vec::new();

                for fd in cl.get_fields().to_vec() {
                    let is_enum = fd.has_modifier(ACC_ENUM)
                        && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);
                    if is_enum {
                        enum_fields.push(fd);
                    } else {
                        non_enum_fields.push(fd);
                    }
                }

                let mut enums = false;
                for fd in enum_fields {
                    if enums {
                        buffer.append(",").append_line_separator();
                    }
                    enums = true;
                    enum_separator.constant_written();

                    have_content(buffer, &mut has_content);
                    self.write_field_to_buffer(buffer, indent, fd, wrapper);
                }

                if enums {
                    buffer.append(";").append_line_separator();
                }

                for fd in non_enum_fields {
                    let hide = (fd.is_synthetic()
                        && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
                        || wrapper.get_hidden_members().borrow().contains(
                            &InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor()),
                        );
                    if hide {
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

                    if enum_separator.before_member(buffer) {
                        has_content = true;
                        enums = true;
                    }

                    if enums {
                        buffer.append_line_separator();
                        enums = false;
                    }

                    have_content(buffer, &mut has_content);
                    self.write_field_to_buffer(buffer, indent, fd, wrapper);
                }
            }

            let methods = cl.get_methods();
            for i in 0..methods.size() {
                let mt = methods.get(i);
                let node_type = node.with(|d| d.type_);
                let hide = if method_to_decompile.is_empty()
                    || (node_type != ClassNodeType::Root && node_type != ClassNodeType::Member)
                {
                    (mt.is_synthetic()
                        && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
                        || (mt.has_modifier(ACC_BRIDGE)
                            && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_BRIDGE))
                        || wrapper.get_hidden_members().borrow().contains(
                            &InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor()),
                        )
                } else {
                    method_to_decompile
                        != format!(
                            "{}.{}{}",
                            cl.qualified_name(),
                            mt.get_name(),
                            mt.get_descriptor()
                        )
                        && (node_type != ClassNodeType::Root
                            || method_to_decompile
                                != format!("{}{}", mt.get_name(), mt.get_descriptor()))
                };
                if hide {
                    continue;
                }

                let mut method_buffer = TextBuffer::new();
                let span = kuna_spans::open_method(&mut method_buffer, cl, mt, wrapper);
                let method_skipped =
                    !self.write_method(node, mt, i as i32, &mut method_buffer, indent + 1);
                kuna_spans::close(&mut method_buffer, span);
                if !method_skipped {
                    if enum_separator.before_member(buffer) {
                        has_content = true;
                    }
                    if has_content {
                        buffer.append_line_separator();
                    }
                    have_content(buffer, &mut has_content);
                    buffer.append_buffer(&method_buffer);
                }
            }

            let nested = node.with(|d| d.nested.to_vec());
            for inner in nested {
                if inner.with(|d| d.type_) == ClassNodeType::Member {
                    let (inner_cl, inner_access) = inner.with(|d| (d.class_struct, d.access));
                    let hide = if method_to_decompile.is_empty() {
                        let is_synthetic =
                            (inner_access & ACC_SYNTHETIC) != 0 || inner_cl.is_synthetic();
                        (is_synthetic
                            && DecompilerContext::get_option(
                                IFernflowerPreferences::REMOVE_SYNTHETIC,
                            ))
                            || wrapper
                                .get_hidden_members()
                                .borrow()
                                .contains(&inner_cl.qualified_name())
                    } else {
                        !method_to_decompile.starts_with(&format!("{}.", inner_cl.qualified_name()))
                    };
                    if hide {
                        continue;
                    }

                    if enum_separator.before_member(buffer) {
                        has_content = true;
                    }
                    if has_content {
                        buffer.append_line_separator();
                    }
                    let mut cls_buffer = TextBuffer::new();
                    self.write_class(inner, &mut cls_buffer, indent + 1);
                    have_content(buffer, &mut has_content);
                    buffer.append_buffer(&cls_buffer);
                }
            }

            let node_type = node.with(|d| d.type_);
            if has_content || node_type != ClassNodeType::Anonymous {
                buffer.append_indent(indent);
            }
            buffer.append("}");

            if node_type != ClassNodeType::Anonymous {
                buffer.append_line_separator();
            }
            kuna_spans::close(buffer, span);
        }

        DecompilerContext::get_logger().end_write_class();
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
        ClassWriter::append_annotations(buffer, 0, cl.0, -1);

        let module_attribute = cl
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_MODULE)
            .expect("NullPointerException: Module attribute");

        if (module_attribute.module_flags() & ACC_OPEN) != 0 {
            buffer.append("open ");
        }

        buffer
            .append("module ")
            .append(&module_attribute.module_name())
            .append(" {")
            .append_line_separator();

        ClassWriter::write_module_info_body(buffer, module_attribute);

        buffer.append("}").append_line_separator();
    }

    fn write_module_info_body(buffer: &mut TextBuffer, module_attribute: StructModuleAttribute) {
        let mut new_line_needed = false;

        let requires_entries = module_attribute.requires();
        if !requires_entries.is_empty() {
            for requires in &requires_entries {
                if !ClassWriter::is_generated(requires.flags) {
                    buffer.append_indent(1).append("requires ");
                    if (requires.flags & ACC_TRANSITIVE) != 0 {
                        buffer.append("transitive ");
                    }
                    if (requires.flags & ACC_STATIC_PHASE) != 0 {
                        buffer.append("static ");
                    }
                    buffer
                        .append(&requires.module_name.replace('/', "."))
                        .append(";")
                        .append_line_separator();
                    new_line_needed = true;
                }
            }
        }

        let exports_entries = module_attribute.exports();
        if !exports_entries.is_empty() {
            if new_line_needed {
                buffer.append_line_separator();
            }
            for exports in &exports_entries {
                if !ClassWriter::is_generated(exports.flags) {
                    buffer
                        .append_indent(1)
                        .append("exports ")
                        .append(&exports.package_name.replace('/', "."));
                    let export_to_modules = &exports.export_to_modules;
                    if !export_to_modules.is_empty() {
                        buffer.append(" to").append_line_separator();
                        ClassWriter::append_fq_class_names(buffer, export_to_modules);
                    }
                    buffer.append(";").append_line_separator();
                    new_line_needed = true;
                }
            }
        }

        let opens_entries = module_attribute.opens();
        if !opens_entries.is_empty() {
            if new_line_needed {
                buffer.append_line_separator();
            }
            for opens in &opens_entries {
                if !ClassWriter::is_generated(opens.flags) {
                    buffer
                        .append_indent(1)
                        .append("opens ")
                        .append(&opens.package_name.replace('/', "."));
                    let opens_to_modules = &opens.opens_to_modules;
                    if !opens_to_modules.is_empty() {
                        buffer.append(" to").append_line_separator();
                        ClassWriter::append_fq_class_names(buffer, opens_to_modules);
                    }
                    buffer.append(";").append_line_separator();
                    new_line_needed = true;
                }
            }
        }

        let uses_entries = module_attribute.uses();
        if !uses_entries.is_empty() {
            if new_line_needed {
                buffer.append_line_separator();
            }
            for uses in &uses_entries {
                buffer
                    .append_indent(1)
                    .append("uses ")
                    .append(&ExprProcessor::build_java_class_name(uses))
                    .append(";")
                    .append_line_separator();
            }
            new_line_needed = true;
        }

        let provides_entries = module_attribute.provides();
        if !provides_entries.is_empty() {
            if new_line_needed {
                buffer.append_line_separator();
            }
            for provides in &provides_entries {
                buffer
                    .append_indent(1)
                    .append("provides ")
                    .append(&ExprProcessor::build_java_class_name(
                        &provides.interface_name,
                    ))
                    .append(" with")
                    .append_line_separator();
                let names: Vec<String> = provides
                    .implementation_names
                    .iter()
                    .map(|n| ExprProcessor::build_java_class_name(n))
                    .collect();
                ClassWriter::append_fq_class_names(buffer, &names);
                buffer.append(";").append_line_separator();
            }
        }
    }

    fn is_generated(flags: i32) -> bool {
        (flags & (ACC_SYNTHETIC | ACC_MANDATED)) != 0
    }

    fn write_class_definition(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        let mark_synthetics =
            DecompilerContext::get_option(IFernflowerPreferences::MARK_CORRESPONDING_SYNTHETICS);
        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();
        let node_type = node.with(|d| d.type_);

        if node_type == ClassNodeType::Anonymous {
            if mark_synthetics {
                ClassWriter::append_synthetic_class_comment(cl, buffer);
            }
            buffer.append(" {");
            return;
        }

        let mut flags = if node_type == ClassNodeType::Root {
            cl.get_access_flags()
        } else {
            node.with(|d| d.access)
        };
        let is_deprecated = cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED);
        let is_synthetic = (flags & ACC_SYNTHETIC) != 0
            || cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_SYNTHETIC);
        let is_enum = DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
            && (flags & ACC_ENUM) != 0;
        let is_interface = (flags & ACC_INTERFACE) != 0;
        let is_annotation = (flags & ACC_ANNOTATION) != 0;
        let is_module_info =
            (flags & ACC_MODULE) != 0 && cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_MODULE);
        let permitted_sub_classes_attr =
            cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_PERMITTED_SUBCLASSES);
        let permitted_sub_classes: Vec<String> = match permitted_sub_classes_attr {
            Some(attr) => attr.get_classes(),
            None => Vec::new(),
        };
        let is_sealed = permitted_sub_classes_attr.is_some() && !permitted_sub_classes.is_empty();
        let is_final = (flags & ACC_FINAL) != 0;
        let is_non_sealed = !is_sealed
            && !is_final
            && cl.get_version().has_sealed_classes()
            && ClassWriter::is_super_class_sealed(cl);

        if is_deprecated && !ClassWriter::contains_deprecated_annotation(cl.0) {
            ClassWriter::append_deprecation(buffer, indent);
        }

        if node_type == ClassNodeType::Member
            && (node
                .with(|d| d.parent)
                .expect("NullPointerException: parent")
                .get_wrapper()
                .expect("NullPointerException: parent wrapper")
                .get_class_struct()
                .get_access_flags()
                & ACC_INTERFACE)
                != 0
        {
            flags &= !ACC_PUBLIC;
            flags &= !ACC_STATIC;
        }

        if let Some(interceptor) = self.interceptor {
            let old_name = interceptor.get_old_name(&cl.qualified_name());
            ClassWriter::append_rename_comment(buffer, old_name.as_deref(), MType::Class, indent);
        }

        if is_synthetic {
            ClassWriter::append_comment(buffer, "synthetic class", indent);
        }

        if let Some(javadoc_provider) = &self.javadoc_provider {
            ClassWriter::append_javadoc(
                buffer,
                javadoc_provider.get_class_javadoc_style(cl),
                javadoc_provider.get_class_doc(cl).as_deref(),
                indent,
            );
        }

        ClassWriter::append_annotations(buffer, indent, cl.0, -1);

        buffer.append_indent(indent);

        if is_enum {
            flags &= !ACC_ABSTRACT;
            flags &= !ACC_FINAL;

            if node_type == ClassNodeType::Member || node_type == ClassNodeType::Local {
                flags &= !ACC_STATIC;
            }
        }

        let components = cl.get_record_components();

        if components.is_some() {
            flags &= !ACC_FINAL;
            flags &= !ACC_STATIC;
        }

        ClassWriter::append_modifiers(
            buffer,
            flags,
            ClassWriter::CLASS_ALLOWED,
            is_interface,
            ClassWriter::CLASS_EXCLUDED,
        );

        if !is_enum && is_sealed {
            buffer.append("sealed ");
        } else if is_non_sealed {
            buffer.append("non-sealed ");
        }
        if is_enum {
            buffer.append("enum ");
        } else if is_interface {
            if is_annotation {
                buffer.append("@");
            }
            buffer.append("interface ");
        } else if is_module_info {
            let module_attribute = cl
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_MODULE)
                .expect("NullPointerException: Module attribute");

            if (module_attribute.module_flags() & ACC_OPEN) != 0 {
                buffer.append("open ");
            }

            buffer.append("module ");
            buffer.append(&module_attribute.module_name());
        } else if components.is_some() {
            buffer.append("record ");
        } else {
            buffer.append("class ");
        }
        let simple_name = node
            .with(|d| d.simple_name.clone())
            .expect("NullPointerException: simpleName");
        buffer.append_class(&simple_name, true, &cl.qualified_name());

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
                ClassWriter::append_type_parameters(buffer, &fparameters, &fbounds);
            }
        }

        if let Some(components) = &components {
            buffer.append("(");
            RecordHelper::append_record_components(buffer, cl, components, indent);
            buffer.append(")");
        }

        buffer.push_newline_group(indent, 1);

        if !is_enum && !is_interface && components.is_none() {
            if let Some(super_class) = cl.super_class() {
                let supertype = VarType::new_signature_cl(&super_class.get_string(), true);
                if !VarType::VARTYPE_OBJECT().equals(&supertype) {
                    buffer.append_possible_newline_alt(" ");
                    buffer.append("extends ");
                    let type_ = match &descriptor {
                        None => supertype,
                        Some(descriptor) => descriptor.superclass(),
                    };
                    buffer.append_cast_type_name(&type_);
                }
            }
        }

        if !is_annotation {
            let interfaces = cl.get_interfaces();
            if !interfaces.is_empty() {
                buffer.append_possible_newline_alt(" ");
                buffer.append(if is_interface {
                    "extends "
                } else {
                    "implements "
                });
                for i in 0..interfaces.len() {
                    if i > 0 {
                        buffer.append(",");
                        buffer.append_possible_newline_alt(" ");
                    }

                    let type_ = match &descriptor {
                        None => Some(VarType::new_signature_cl(&cl.get_interface(i as i32), true)),
                        Some(descriptor) => descriptor.superinterfaces().get(i).cloned(),
                    };
                    if let Some(type_) = type_ {
                        buffer.append_cast_type_name(&type_);
                    }
                }
            }
        }

        let omit_permits = kuna_fixes::enabled(KunaFix::SealedPrivatePermits)
            && kuna_fix_sealed_private_permits::omit_permits(node, &permitted_sub_classes);
        if !is_enum && is_sealed && !omit_permits {
            buffer.append_possible_newline_alt(" ");
            buffer.append("permits ");
            for (i, permitted) in permitted_sub_classes.iter().enumerate() {
                if i > 0 {
                    buffer.append(",");
                    buffer.append_possible_newline_alt(" ");
                }
                buffer.append_cast_type_name(&VarType::new_signature_cl(permitted, true));
            }
        }

        buffer.pop_newline_group();

        if mark_synthetics && node_type == ClassNodeType::Local {
            ClassWriter::append_synthetic_class_comment(cl, buffer);
        }

        buffer.append(" {").append_line_separator();
    }

    fn is_super_class_sealed(cl: StructClass) -> bool {
        if let Some(super_class) = cl.super_class() {
            let super_class =
                DecompilerContext::get_struct_context().get_class(&super_class.get_string());
            if super_class.is_some_and(|s| {
                s.has_attribute(StructGeneralAttribute::ATTRIBUTE_PERMITTED_SUBCLASSES)
            }) {
                return true;
            }
        }
        for iface in cl.get_interface_names() {
            let iface_class = DecompilerContext::get_struct_context().get_class(&iface);
            if iface_class.is_some_and(|c| {
                c.has_attribute(StructGeneralAttribute::ATTRIBUTE_PERMITTED_SUBCLASSES)
            }) {
                return true;
            }
        }
        false
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
            indent + 1,
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
        if RecordHelper::is_hidden_record_field(cl.get_record_components().as_deref(), fd) {
            return;
        }

        let is_interface = cl.has_modifier(ACC_INTERFACE);
        let is_deprecated = fd.has_attribute(StructGeneralAttribute::ATTRIBUTE_DEPRECATED);
        let is_enum = fd.has_modifier(ACC_ENUM)
            && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);

        if is_deprecated && !ClassWriter::contains_deprecated_annotation(fd.0) {
            ClassWriter::append_deprecation(buffer, indent);
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
            ClassWriter::append_rename_comment(buffer, old_name.as_deref(), MType::Field, indent);
        }

        if fd.is_synthetic() {
            ClassWriter::append_comment(buffer, "synthetic field", indent);
        }

        if let Some(javadoc_provider) = &self.javadoc_provider {
            ClassWriter::append_javadoc(
                buffer,
                javadoc_provider.get_field_javadoc_style(cl, fd),
                javadoc_provider.get_field_doc(cl, fd).as_deref(),
                indent,
            );
        }
        let written_annotations =
            ClassWriter::append_annotations(buffer, indent, fd.0, TypeAnnotation::FIELD);

        buffer.append_indent(indent);

        if !is_enum {
            ClassWriter::append_modifiers(
                buffer,
                fd.get_access_flags(),
                ClassWriter::FIELD_ALLOWED,
                is_interface,
                ClassWriter::FIELD_EXCLUDED,
            );
        }

        let (field_type, descriptor) = ClassWriter::get_field_type_data(fd);
        if let Some(descriptor) = &descriptor {
            descriptor.verify_type(cl.get_signature().as_ref(), &field_type);
        }
        let declared_type = |descriptor: &Option<GenericFieldDescriptor>| match descriptor {
            None => field_type.clone(),
            Some(descriptor) => descriptor.type_(),
        };

        if !is_enum {
            let mut annos = ClassWriter::get_type_annotations(fd.0, TypeAnnotation::FIELD, -1);
            if !annos.is_empty() {
                buffer.append_cast_type_name_annotations(
                    &declared_type(&descriptor),
                    &mut annos,
                    &written_annotations,
                );
            } else {
                buffer.append_cast_type_name(&declared_type(&descriptor));
            }

            buffer.append(" ");
        }

        buffer.append_field(
            &name,
            true,
            &cl.qualified_name(),
            &name,
            &fd.get_descriptor(),
        );

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
                let expr = initializer.as_new();
                expr.set_enum_const(true);
                buffer.append_buffer(&expr.0.to_java(indent));
            } else {
                buffer.append(" = ");

                if let Some(const_exprent) = initializer.try_as_const() {
                    const_exprent.adjust_const_type(&field_type);
                }

                kuna_fix_disjoint_generic_cast::mark_declared_target();
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

    fn method_lambda_to_java(
        lambda_node: ClassNode,
        class_wrapper: ClassWrapper,
        mt: StructMethod,
        buffer: &mut TextBuffer,
        indent: i32,
        code_only: bool,
    ) {
        let mut indent = indent;
        let method_wrapper = class_wrapper.get_method_wrapper(&mt.get_name(), &mt.get_descriptor());

        let outer_wrapper = DecompilerContext::get_current_method_wrapper();
        DecompilerContext::set_current_method_wrapper(method_wrapper);

        let _finally = RestoreMethodWrapper(outer_wrapper);
        let li = lambda_information(lambda_node);
        let method_name = li.method_name.clone();
        let md_content = MethodDescriptor::parse_descriptor(&li.content_method_descriptor);
        let md_lambda = MethodDescriptor::parse_descriptor(&li.method_descriptor);
        let class_name = class_wrapper.get_class_struct().qualified_name();

        if !code_only {
            buffer.append_indent(indent);
            buffer.append("public ");
            buffer.append(&method_name);
            buffer.append("(");

            let mut first_parameter = true;
            let mut index: i32 = if li.is_content_method_static { 0 } else { 1 };
            let start_index = md_content.params.len() as i32 - md_lambda.params.len() as i32;

            for i in 0..md_content.params.len() {
                if i as i32 >= start_index {
                    if !first_parameter {
                        buffer.append(", ");
                    }

                    let type_ = &md_content.params[i];
                    let mut type_name = ExprProcessor::get_cast_type_name(type_);
                    if ExprProcessor::UNDEFINED_TYPE_STRING == type_name
                        && DecompilerContext::get_option(
                            IFernflowerPreferences::UNDEFINED_PARAM_TYPE_OBJECT,
                        )
                    {
                        type_name = ExprProcessor::get_cast_type_name(&VarType::VARTYPE_OBJECT());
                    }

                    buffer.append_cast_type_name_str(&type_name, type_);
                    buffer.append(" ");

                    let mw = method_wrapper.expect("NullPointerException: methodWrapper");
                    let (varproc, method_struct) = mw.with(|d| (d.varproc, d.method_struct));
                    let parameter_name = varproc
                        .get_var_name(VarVersionPair::new(index, 0))
                        .unwrap_or_else(|| format!("param{index}"));
                    let parameter_name = method_struct
                        .get_variable_namer()
                        .rename_parameter(
                            mt.get_access_flags(),
                            type_,
                            Some(&parameter_name),
                            index,
                        )
                        .expect("NullPointerException: parameter name");
                    buffer.append_variable_descriptor(
                        &parameter_name,
                        true,
                        true,
                        &class_name,
                        &method_name,
                        &md_content,
                        index,
                        &parameter_name,
                    );

                    first_parameter = false;
                }

                index += md_content.params[i].stack_size;
            }

            buffer.append(") {").append_line_separator();

            indent += 1;
        }

        let root = class_wrapper
            .get_method_wrapper(&mt.get_name(), &mt.get_descriptor())
            .expect("NullPointerException: lambda method wrapper")
            .with(|d| d.root);
        let mw = method_wrapper.expect("NullPointerException: methodWrapper");
        if mw.with(|d| d.decompile_error.is_none()) {
            if let Some(root) = root {
                let result = try_catch(|| {
                    let mut child_buf = root.to_java(indent);
                    child_buf.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                    buffer.append_buffer_mapped(
                        &child_buf,
                        Some(&class_name),
                        Some(&InterpreterUtil::make_unique_key(
                            &mt.get_name(),
                            &mt.get_descriptor(),
                        )),
                    );
                });
                if let Err(t) = result {
                    if CanceledException::is(&t) {
                        rethrow(t);
                    }
                    let message = format!(
                        "Method {} {} in class {} couldn't be written.",
                        mt.get_name(),
                        mt.get_descriptor(),
                        lambda_node.with(|d| d.class_struct).qualified_name()
                    );
                    DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);
                    mw.with_mut(|d| d.decompile_error = Some(t));
                }
            }
        }

        if mw.with(|d| d.decompile_error.is_some()) {
            ClassWriter::dump_error(buffer, mw, indent);
        }

        if !code_only {
            indent -= 1;
            buffer
                .append_indent(indent)
                .append("}")
                .append_line_separator();
        }
    }

    pub fn to_valid_java_identifier(name: Option<&str>) -> Option<String> {
        let name = name?;
        if name.is_empty() {
            return Some(name.to_string());
        }

        let mut changed = false;
        let units = name.utf16();
        let mut res: Vec<u16> = Vec::with_capacity(units.len());
        for (i, &c) in units.iter().enumerate() {
            if (i == 0 && !is_java_identifier_start(c)) || (i > 0 && !is_java_identifier_part(c)) {
                changed = true;
                res.push(b'_' as u16);
            } else {
                res.push(c);
            }
        }
        if !changed {
            return Some(name.to_string());
        }
        Some(from_utf16(&res))
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
        let wrapper = node.get_wrapper().expect("NullPointerException: wrapper");
        let cl = wrapper.get_class_struct();
        let method_wrapper = wrapper.get_method_wrapper_index(method_index);

        let mut hide_method = false;

        let outer_wrapper = DecompilerContext::get_current_method_wrapper();
        DecompilerContext::set_current_method_wrapper(Some(method_wrapper));

        {
            let _finally = RestoreMethodWrapper(outer_wrapper);
            let node_type = node.with(|d| d.type_);
            let node_class_struct = node.with(|d| d.class_struct);

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

            if is_deprecated && !ClassWriter::contains_deprecated_annotation(mt.0) {
                ClassWriter::append_deprecation(buffer, indent);
            }

            let mut name = mt.get_name();
            if let Some(interceptor) = self.interceptor {
                let new_name = interceptor.get_name(&format!(
                    "{} {} {}",
                    cl.qualified_name(),
                    mt.get_name(),
                    mt.get_descriptor()
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
                    mt.get_descriptor()
                ));
                ClassWriter::append_rename_comment(
                    buffer,
                    old_name.as_deref(),
                    MType::Method,
                    indent,
                );
            }

            let is_synthetic = (flags & ACC_SYNTHETIC) != 0
                || mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_SYNTHETIC);
            let is_bridge = (flags & ACC_BRIDGE) != 0;
            if is_synthetic {
                ClassWriter::append_comment(buffer, "synthetic method", indent);
            }
            if is_bridge {
                ClassWriter::append_comment(buffer, "bridge method", indent);
            }

            let (add_error_comment, has_comment_lines) =
                method_wrapper.with(|d| (d.add_error_comment, d.comment_lines.is_some()));
            if (DecompilerContext::get_option(IFernflowerPreferences::DECOMPILER_COMMENTS)
                && add_error_comment)
                || has_comment_lines
            {
                if add_error_comment {
                    for s in ClassWriter::get_error_comment() {
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
                ClassWriter::append_javadoc(
                    buffer,
                    javadoc_provider.get_method_javadoc_style(cl, mt),
                    javadoc_provider.get_method_doc(cl, mt).as_deref(),
                    indent,
                );
            }

            let written_annotations = ClassWriter::append_annotations(
                buffer,
                indent,
                mt.0,
                TypeAnnotation::METHOD_RETURN_TYPE,
            );

            let annotation_attribute =
                mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS);
            let should_apply_override =
                DecompilerContext::get_option(IFernflowerPreferences::OVERRIDE_ANNOTATION)
                    && mt.get_bytecode_version().has_override()
                    && INIT_NAME != mt.get_name()
                    && CLINIT_NAME != mt.get_name()
                    && !mt.has_modifier(ACC_STATIC)
                    && !mt.has_modifier(ACC_PRIVATE);

            if should_apply_override {
                let is_override = if kuna_fixes::enabled(KunaFix::OverrideInaccessibleMethod) {
                    kuna_fix_override_inaccessible_method::overrides(cl, &mt.get_name(), &md)
                } else {
                    ClassWriter::search_for_method(Some(cl), &mt.get_name(), &md, false)
                };
                let already_has_override = annotation_attribute.is_some_and(|a| {
                    a.get_annotations()
                        .iter()
                        .any(|annotation| "java/lang/Override" == annotation.get_class_name())
                });

                if is_override && !already_has_override {
                    buffer.append_indent(indent);
                    buffer.append("@Override");
                    buffer.append_line_separator();
                }
            }

            buffer.append_indent(indent);

            if INIT_NAME == name {
                if is_enum {
                    flags &= !ACC_PRIVATE;
                }

                if node_type == ClassNodeType::Anonymous {
                    name = String::new();
                    d_init = true;
                } else {
                    name = node
                        .with(|d| d.simple_name.clone())
                        .expect("NullPointerException: simpleName");
                    init = true;
                }
            } else if CLINIT_NAME == name {
                name = String::new();
                cl_init = true;
            }

            if !d_init {
                ClassWriter::append_modifiers(
                    buffer,
                    flags,
                    ClassWriter::METHOD_ALLOWED,
                    is_interface,
                    ClassWriter::METHOD_EXCLUDED,
                );
            }

            if is_interface
                && !mt.has_modifier(ACC_STATIC)
                && mt.contains_code()
                && (flags & ACC_PRIVATE) == 0
            {
                buffer.append("default ");
            }

            let descriptor = md.generic_info();
            if let Some(descriptor) = &descriptor {
                let mut params: Vec<VarType> = md.params.clone();

                if init && node_class_struct.has_modifier(ACC_ENUM) {
                    params.remove(0);
                    params.remove(0);
                }

                if params.len() != descriptor.parameter_types().len() {
                    let mask = method_wrapper.with(|d| d.synth_parameters.clone());
                    if let Some(mask) = mask {
                        let mut j: i32 = 0;
                        for item in &mask {
                            if item.is_some() {
                                params.remove(j as usize);
                                j -= 1;
                            }
                            j += 1;
                        }
                    }
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

                let mut checker = GenericsChecker::new();

                let mut current_node = Some(node);
                'outer: while let Some(current) = current_node {
                    let (enclosing_method_key, parent) =
                        current.with(|d| (d.enclosing_method.clone(), d.parent));
                    if let (Some(enclosing_method_key), Some(parent)) =
                        (&enclosing_method_key, parent)
                    {
                        let enclosing_method = parent
                            .with(|d| d.class_struct)
                            .get_method_key(enclosing_method_key);
                        if let Some(enclosing_method) = enclosing_method {
                            if let Some(signature) = enclosing_method.get_signature() {
                                let type_parameters = signature.type_parameters().clone();
                                let type_parameter_bounds =
                                    signature.type_parameter_bounds().clone();
                                checker = checker.copy(&type_parameters, &type_parameter_bounds);
                            }

                            let parent_nested = parent.with(|d| d.nested.to_vec());
                            for child in parent_nested {
                                if child.with(|d| d.type_) != ClassNodeType::Lambda {
                                    continue;
                                }

                                let child_li = lambda_information(child);
                                if *enclosing_method_key != child_li.content_method_key {
                                    continue;
                                }

                                if child_li.content_method_invocation_type
                                    == CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL
                                    && parent.with(|d| d.class_struct).qualified_name()
                                        != child_li.content_class_name
                                {
                                    continue;
                                }

                                current_node = Some(child);
                                continue 'outer;
                            }
                        }
                    }

                    let parent_signature = current.with(|d| d.class_struct).get_signature();
                    if let Some(parent_signature) = parent_signature {
                        checker = checker.copy_parent(&parent_signature.get_checker());
                    }

                    current_node = current.with(|d| d.parent);
                }

                descriptor.verify_types(
                    Some(&checker),
                    &params,
                    &mt.method_descriptor().ret,
                    &exceptions,
                );
            }

            let mut throws_exceptions = false;
            let mut param_count = 0;

            if !cl_init && !d_init {
                let this_var = !mt.has_modifier(ACC_STATIC);

                if let Some(descriptor) = &descriptor {
                    if !descriptor.type_parameters().is_empty() {
                        let type_parameters = descriptor.type_parameters().clone();
                        let type_parameter_bounds = descriptor.type_parameter_bounds().clone();
                        ClassWriter::append_type_parameters(
                            buffer,
                            &type_parameters,
                            &type_parameter_bounds,
                        );
                        buffer.append(" ");
                    }
                }

                if !init {
                    let return_type = match &descriptor {
                        None => md.ret.clone(),
                        Some(descriptor) => descriptor
                            .return_type()
                            .expect("NullPointerException: returnType"),
                    };
                    let mut annos = ClassWriter::get_type_annotations(
                        mt.0,
                        TypeAnnotation::METHOD_RETURN_TYPE,
                        -1,
                    );
                    if !annos.is_empty() {
                        buffer.append_cast_type_name_annotations(
                            &return_type,
                            &mut annos,
                            &written_annotations,
                        );
                    } else {
                        buffer.append_cast_type_name(&return_type);
                    }
                    buffer.append(" ");
                }

                let valid_name = ClassWriter::to_valid_java_identifier(Some(&name))
                    .expect("toValidJavaIdentifier of a non-null name");
                buffer.append_method_descriptor(
                    &valid_name,
                    true,
                    &cl.qualified_name(),
                    &mt.get_name(),
                    &md,
                );
                if valid_name != name {
                    buffer.append("/* $VF was: ").append(&name).append(" */");
                }

                if !method_wrapper.with(|d| d.is_compact_record_constructor) {
                    param_count = ClassWriter::write_method_parameter_header(
                        mt,
                        buffer,
                        indent,
                        method_wrapper,
                        &md,
                        is_enum,
                        init,
                        this_var,
                        descriptor.as_ref(),
                        param_count,
                        is_interface,
                        flags,
                        cl,
                    );
                }

                let attr = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS);
                let descriptor_exceptions = descriptor
                    .as_ref()
                    .is_some_and(|d| !d.exception_types().is_empty());
                if descriptor_exceptions || attr.is_some() {
                    throws_exceptions = true;
                    buffer.append(" throws ");

                    let use_descriptor = descriptor_exceptions;
                    let count = match attr {
                        None => descriptor
                            .as_ref()
                            .expect("NullPointerException: descriptor")
                            .exception_types()
                            .len(),
                        Some(attr) => attr.get_throws_exceptions().len(),
                    };
                    for i in 0..count {
                        if i > 0 {
                            buffer.append(", ");
                        }
                        let type_ = if use_descriptor {
                            descriptor.as_ref().unwrap().exception_types()[i].clone()
                        } else {
                            VarType::new_signature_cl(
                                &attr.unwrap().get_exc_classname(i as i32, cl.get_pool()),
                                true,
                            )
                        };
                        buffer.append_cast_type_name(&type_);
                    }
                }
            }

            if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0 {
                if is_annotation {
                    let attr =
                        mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_ANNOTATION_DEFAULT);
                    if let Some(attr) = attr {
                        buffer.append(" default ");
                        let default_value = attr
                            .get_default_value()
                            .expect("NullPointerException: default value");
                        buffer.append_buffer(&default_value.to_java(0));
                    }
                }

                buffer.append(";");
                buffer.append_line_separator();
            } else {
                if !cl_init && !d_init {
                    buffer.append(" ");
                }

                buffer.append("{").append_line_separator();

                let root = method_wrapper.with(|d| d.root);

                if let Some(root) =
                    root.filter(|_| method_wrapper.with(|d| d.decompile_error.is_none()))
                {
                    let result = try_catch(|| {
                        if RecordHelper::is_hidden_record_method(cl, mt, root) {
                            hide_method = true;
                        } else {
                            let mut code = root.to_java(indent + 1);
                            code.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                            hide_method = code.length() == 0
                                && (cl_init
                                    || d_init
                                    || ClassWriter::hide_constructor(
                                        node,
                                        init,
                                        throws_exceptions,
                                        param_count,
                                        flags,
                                        mt,
                                    ));
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
                        if CanceledException::is(&t) {
                            rethrow(t);
                        }
                        let message = format!(
                            "Method {} {} in class {} couldn't be written.",
                            mt.get_name(),
                            mt.get_descriptor(),
                            node_class_struct.qualified_name()
                        );
                        DecompilerContext::get_logger().write_message_ex(
                            &message,
                            Severity::Warn,
                            &t,
                        );
                        method_wrapper.with_mut(|d| d.decompile_error = Some(t));
                    }
                }

                if method_wrapper.with(|d| d.decompile_error.is_some()) {
                    ClassWriter::dump_error(buffer, method_wrapper, indent + 1);
                }
                buffer
                    .append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
        }

        !hide_method
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
        let mut param_count = param_count;
        buffer.append("(");

        let (mask, varproc, method_struct) =
            method_wrapper.with(|d| (d.synth_parameters.clone(), d.varproc, d.method_struct));

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
            let attr = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS);
            if let Some(attr) = attr {
                method_parameters = Some(attr.get_entries());
            }
        }

        let mut index: i32 = if is_enum && init {
            3
        } else if this_var {
            1
        } else {
            0
        };
        let start = if is_enum && init { 2 } else { 0 };
        let has_descriptor = descriptor.is_some();

        buffer.push_newline_group(indent, 0);
        for i in start..md.params.len() {
            let real = mask.as_ref().is_none_or(|m| m[i].is_none());
            let mut parameter_type = if real
                && has_descriptor
                && (param_count as usize) < descriptor.unwrap().parameter_types().len()
            {
                descriptor.unwrap().parameter_types()[param_count as usize].clone()
            } else {
                md.params[i].clone()
            };
            if real {
                if param_count > 0 {
                    buffer.append(",");
                    buffer.append_possible_newline_alt(" ");
                }

                let written_annotations =
                    ClassWriter::append_parameter_annotations(buffer, mt, param_count);

                match &method_parameters {
                    Some(method_parameters) if i < method_parameters.len() => {
                        ClassWriter::append_modifiers(
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

                let mut is_var_arg = i as i32 == last_visible_parameter_index
                    && mt.has_modifier(ACC_VARARGS)
                    && parameter_type.array_dim > 0;
                if is_var_arg {
                    parameter_type = parameter_type.decrease_array_dim();
                }
                let mut type_name = ExprProcessor::get_cast_type_name(&parameter_type);

                if ExprProcessor::UNDEFINED_TYPE_STRING == type_name
                    && DecompilerContext::get_option(
                        IFernflowerPreferences::UNDEFINED_PARAM_TYPE_OBJECT,
                    )
                {
                    type_name = ExprProcessor::get_cast_type_name(&VarType::VARTYPE_OBJECT());
                }
                let mut annos = ClassWriter::get_type_annotations(
                    mt.0,
                    TypeAnnotation::METHOD_PARAMETER,
                    param_count,
                );
                if !annos.is_empty() {
                    let mut temp_param = parameter_type.clone();
                    if is_var_arg {
                        temp_param = temp_param.resize_array_dim(temp_param.array_dim + 1);
                    }
                    buffer.append_cast_type_name_annotations_vararg(
                        &temp_param,
                        &mut annos,
                        &written_annotations,
                        is_var_arg,
                        true,
                    );
                    is_var_arg = false;
                } else {
                    buffer.append_cast_type_name_str(&type_name, &parameter_type);
                }

                if is_var_arg {
                    buffer.append("...");
                }

                buffer.append(" ");

                let clashing_name = varproc.get_clashing_name(VarVersionPair::new(index, 0));
                let parameter_name: Option<String> = if clashing_name.is_some() {
                    clashing_name
                } else if let Some(name) = method_parameters
                    .as_ref()
                    .filter(|p| i < p.len())
                    .and_then(|p| p[i].my_name.clone())
                {
                    Some(name)
                } else {
                    let name = varproc.get_var_name(VarVersionPair::new(index, 0));
                    let table_name = kuna_fixes::enabled(KunaFix::NestedParamRename)
                        .then(|| {
                            kuna_fix_nested_param_rename::table_name(
                                varproc,
                                VarVersionPair::new(index, 0),
                                name.as_deref(),
                            )
                        })
                        .flatten();
                    table_name.or(name)
                };

                let mut new_parameter_name = method_struct.get_variable_namer().rename_parameter(
                    flags,
                    &parameter_type,
                    parameter_name.as_deref(),
                    index,
                );
                if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0
                    && new_parameter_name == parameter_name
                {
                    new_parameter_name = DecompilerContext::get_struct_context()
                        .rename_abstract_parameter(
                            &method_struct.get_class_qualified_name(),
                            &mt.get_name(),
                            &mt.get_descriptor(),
                            index - if (flags & ACC_STATIC) == 0 { 1 } else { 0 },
                            parameter_name.as_deref(),
                        );
                }
                let parameter_name = new_parameter_name;

                let value = match &parameter_name {
                    None => format!("param{index}"),
                    Some(parameter_name) => parameter_name.clone(),
                };
                buffer.append_variable_descriptor(
                    &value,
                    true,
                    true,
                    &cl.qualified_name(),
                    &mt.get_name(),
                    md,
                    index,
                    parameter_name.as_deref().unwrap_or("null"),
                );

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
        param_count
    }

    fn dump_error(buffer: &mut TextBuffer, wrapper: MethodWrapper, indent: i32) {
        kuna_spans::method_failed(wrapper);
        let mut lines: Vec<String> = vec!["$VF: Couldn't be decompiled".to_string()];
        let exceptions =
            DecompilerContext::get_option(IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR);
        let bytecode =
            DecompilerContext::get_option(IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR);
        if exceptions {
            lines.extend(ClassWriter::get_error_comment());
            let error = wrapper
                .with(|d| d.decompile_error.clone())
                .expect("NullPointerException: decompileError");
            ClassWriter::collect_error_lines(&error, &mut lines);
            if bytecode {
                lines.push(String::new());
            }
        }
        if bytecode {
            let method_struct = wrapper.with(|d| d.method_struct);
            let result = try_catch(|| {
                lines.push("Bytecode:".to_string());
                ClassWriter::collect_bytecode(wrapper, &mut lines);
            });
            if let Err(e) = result {
                if is_java_error(&e) {
                    method_struct.release_resources();
                    rethrow(e);
                }
                lines.push("Error collecting bytecode:".to_string());
                ClassWriter::collect_error_lines(&e, &mut lines);
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

    /// The `DUMP_EXCEPTION_ON_ERROR` lines describing `error`: its text and its
    /// throw location as the single stack frame (nothing without a location).
    pub fn collect_error_lines(error: &Caught, lines: &mut Vec<String>) {
        let mut filtered_stack: Vec<&str> = Vec::new();
        if !error.location.is_empty() {
            filtered_stack.push(&error.location);
        }
        if filtered_stack.is_empty() {
            return;
        }
        lines.push(error.message.clone());
        for e in filtered_stack {
            lines.push(format!("  at {e}"));
        }
    }

    fn collect_bytecode(wrapper: MethodWrapper, lines: &mut Vec<String>) {
        let class_node = DecompilerContext::get_current_class_node()
            .expect("NullPointerException: CURRENT_CLASS_NODE");
        let class_struct = class_node.with(|d| d.class_struct);
        let method = wrapper.with(|d| d.method_struct);
        let mut instructions = method.get_instruction_sequence();
        if instructions.is_none() {
            method.expand_data(class_struct);
            instructions = method.get_instruction_sequence();
        }
        let instructions = instructions.expect("NullPointerException: instructions");
        let last_offset = instructions.get_last().start_offset();
        let digits = 8 - (last_offset as u32).leading_zeros() as i32 / 4;
        let pool = class_struct.get_pool();
        let bootstrap =
            class_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS);

        for instr in instructions.iter() {
            let mut sb = String::new();
            let off_hex = int_to_hex_string(instr.start_offset());
            let mut i = off_hex.len() as i32;
            while i < digits {
                sb.push('0');
                i += 1;
            }
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
                            ClassWriter::append_bootstrap_call(&mut sb, &target, bootstrap);
                        }
                        None => {
                            ClassWriter::append_constant(
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
                    ClassWriter::append_constant(
                        &mut sb,
                        pool.get_constant(instr.operand(0)).as_ref(),
                    );
                }
                GROUP_JUMP => {
                    sb.push(' ');
                    let dest = instr.start_offset().wrapping_add(instr.operand(0));
                    let dest_hex = int_to_hex_string(dest);
                    let mut i = dest_hex.len() as i32;
                    while i < digits {
                        sb.push('0');
                        i += 1;
                    }
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
                                ClassWriter::append_bootstrap_call(
                                    &mut sb,
                                    &constant.as_link(),
                                    bootstrap,
                                );
                            }
                            None => ClassWriter::append_constant(&mut sb, Some(&constant)),
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

        for handler in instructions.exception_table().get_handlers() {
            lines.push(format!(
                "try ({} -> {}): {} {}",
                handler.from(),
                handler.to(),
                handler.handler(),
                handler.exception_class().as_deref().unwrap_or("null")
            ));
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
        ClassWriter::append_constant(sb, bsm.map(PooledConstant::Link).as_ref());
        sb.push_str(" args=[ ");
        let mut first = true;
        for arg in &bsm_args {
            if !first {
                sb.push_str(", ");
            }
            first = false;
            ClassWriter::append_constant(sb, arg.as_ref());
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
                sb.push_str(link_constant.classname.as_deref().unwrap_or("null"));
                sb.push('.');
                sb.push_str(&link_constant.elementname);
                sb.push(' ');
                sb.push_str(&link_constant.descriptor);
            }
        }
    }

    fn hide_constructor(
        node: ClassNode,
        init: bool,
        throws_exceptions: bool,
        param_count: i32,
        method_access_flags: i32,
        struct_method: StructMethod,
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
        let is_record = cl.get_record_components().is_some();

        if !is_enum
            && !is_record
            && ((class_access_flags & ClassWriter::ACCESSIBILITY_FLAGS)
                != (method_access_flags & ClassWriter::ACCESSIBILITY_FLAGS))
        {
            return false;
        }

        if is_record && RecordHelper::has_annotations(struct_method) {
            return false;
        }

        if !is_record {
            let mut count = 0;
            for mt in cl.get_methods().to_vec() {
                if INIT_NAME == mt.get_name() {
                    count += 1;
                    if count > 1 {
                        return false;
                    }
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
        for key in ClassWriter::ANNOTATION_ATTRIBUTES {
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
                buffer.append(&ClassWriter::get_type_print_out(&fd.type_));
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
                    buffer.append(&ClassWriter::get_type_print_out(param_type));
                }
                buffer.append(") ");
                buffer.append(&ClassWriter::get_type_print_out(&md.ret));
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
        let mut filter: JHashSet<String> = JHashSet::new();

        for key in ClassWriter::ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    annotation.set_did_write_already(true);
                    buffer.append_indent(indent);
                    filter.add(
                        annotation
                            .to_java(-1)
                            .convert_to_string_and_allow_data_discard(),
                    );
                    buffer.append_buffer(&annotation.to_java(indent));
                    if indent < 0 {
                        buffer.append(" ");
                    } else {
                        buffer.append_line_separator();
                    }
                }
            }
        }

        if target_type == TypeAnnotation::FIELD || target_type == TypeAnnotation::METHOD_RETURN_TYPE
        {
            return filter;
        }
        ClassWriter::append_top_level_type_annotations(
            buffer,
            indent,
            mb,
            target_type,
            -1,
            &filter,
        );

        filter
    }

    /// Whether a non-static method `name`/`md` exists in `cl`'s supertypes (and `cl` itself when `search`).
    fn search_for_method(
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

            let found_in_super_class = ClassWriter::search_for_method(super_class, name, md, true);

            if found_in_super_class {
                return true;
            }
        }

        for iface_name in cl.get_interface_names() {
            let iface = DecompilerContext::get_struct_context().get_class(&iface_name);

            let found_in_iface = ClassWriter::search_for_method(iface, name, md, true);

            if found_in_iface {
                return true;
            }
        }

        false
    }

    fn append_parameter_annotations(
        buffer: &mut TextBuffer,
        mt: StructMethod,
        param: i32,
    ) -> JHashSet<String> {
        let mut filter: JHashSet<String> = JHashSet::new();

        for key in ClassWriter::PARAMETER_ANNOTATION_ATTRIBUTES {
            let attribute = mt.get_attribute(key);
            if let Some(attribute) = attribute {
                let annotations = attribute.get_param_annotations();
                if (param as usize) < annotations.len() {
                    for annotation in &annotations[param as usize] {
                        filter.add(
                            annotation
                                .to_java(-1)
                                .convert_to_string_and_allow_data_discard(),
                        );
                        buffer.append_buffer(&annotation.to_java(-1)).append(" ");
                    }
                }
            }
        }

        filter
    }

    fn append_top_level_type_annotations(
        buffer: &mut TextBuffer,
        indent: i32,
        mb: StructMember,
        target_type: i32,
        index: i32,
        filter: &JHashSet<String>,
    ) {
        for key in ClassWriter::TYPE_ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    if annotation.is_top_level()
                        && annotation.get_target_type() == target_type
                        && (index < 0 || annotation.get_index() == index)
                    {
                        let text = annotation
                            .get_annotation()
                            .to_java(-1)
                            .convert_to_string_and_allow_data_discard();
                        if !filter.contains(&text) {
                            buffer.append_indent(indent);
                            buffer.append_buffer(&annotation.get_annotation().to_java(indent));
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

    pub fn get_type_annotations(
        mb: StructMember,
        target_type: i32,
        index: i32,
    ) -> Vec<Pair<VecDeque<PathValue>, AnnotationExprent>> {
        let mut list: Vec<Pair<VecDeque<PathValue>, AnnotationExprent>> = Vec::new();
        for key in ClassWriter::TYPE_ANNOTATION_ATTRIBUTES {
            let attribute = mb.get_attribute(key);
            if let Some(attribute) = attribute {
                for annotation in attribute.get_annotations() {
                    if annotation.get_target_type() == target_type
                        && (index < 0 || annotation.get_index() == index)
                    {
                        let q = annotation.as_queue();
                        list.push(Pair::of(q, annotation.get_annotation()));
                    }
                }
            }
        }

        list
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
                buffer.append_cast_type_name(&parameter_bounds[0]);
                for bound in &parameter_bounds[1..] {
                    buffer.append(" & ");
                    buffer.append_cast_type_name(bound);
                }
            }
        }

        buffer.append(">");
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
