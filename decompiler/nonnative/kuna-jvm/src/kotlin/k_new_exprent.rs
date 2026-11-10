//! Port of `org/vineflower/kotlin/expr/KNewExprent.java`.
//!
//! kuna fix `kt_property_reference`: a property-reference class is
//! written as the reference (`kuna_fix_kt_property_reference`).
//!
//! kuna fix `kt_synthetic_anonymous_object` (`kuna_fixes`): an
//! anonymous class is written with its `new`'s constructor at hand, so the
//! fix can tell whether the object's captured names mean what this `new`
//! passes.
//!
//! kuna fix `kt_object_super_call` (`kuna_fixes`): an object's
//! superclass arguments are computed here, where the `new` stands, before its
//! class is written (`kuna_fix_kt_object_super_call`).
//! kuna fix `kt_constructor_diamond`: a constructor call whose type arguments
//! are left to inference (Java's diamond, a single `GenericType::DUMMY_VAR`)
//! is written without type arguments, which is how Kotlin leaves them to
//! inference; Vineflower writes `<>`, which is not Kotlin.
//! kuna fix `kt_partial_array_dims` (`kuna_fixes`): in `to_java`, an array
//! creation that sizes fewer dimensions than its type (`anewarray` of an array
//! class) ends in `arrayOfNulls`, where Vineflower reads a length for every
//! dimension of the type.
//! kuna fix `kt_default_args_call` (`kuna_fixes`): a constructor call through
//! a default-arguments constructor is written as the call it stands for
//! (`kuna_fix_kt_default_args_call::resugar_new`).

use crate::class_writer::ClassWriter;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::generic_type::GenericType;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_element::KElement;
use crate::k_exit_exprent::KExitExprent;
use crate::k_types::KTypes;
use crate::k_utils::KUtils;
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_exprent;
use crate::kotlin_writer::{is_stop_signal, KotlinWriter};
use crate::kt_attrs::KtAttrs;
use crate::kuna_fix_kt_object_super_call;
use crate::kuna_fix_kt_property_reference;
use crate::kuna_fix_kt_synthetic_anonymous_object;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{rethrow, try_catch};
use crate::stat_edge::StatEdge;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct KNewExprentData {
    pub in_annotation: bool,
}

kotlin_exprent!(pub KNewExprent: NewExprent, New(KNewExprentData),
                is_k_new, as_k_new, try_as_k_new);

fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

/// `DecompilerContext.getClassProcessor().getMapRootClasses().get(name)`; a null name finds nothing.
fn root_class_node(name: Option<&str>) -> Option<ClassNode> {
    let name = name?;
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// The `finally` that puts the outer method wrapper back.
struct RestoreMethodWrapper(Option<MethodWrapper>);

impl Drop for RestoreMethodWrapper {
    fn drop(&mut self) {
        if DecompilerContext::get_current_context().is_some() {
            DecompilerContext::set_current_method_wrapper(self.0);
        }
    }
}

impl KNewExprent {
    #[allow(dead_code)]
    const REFERENCE_CTOR_DESC: &'static str = "(Ljava/lang/Object;)V";

    /// Java: KNewExprent(NewExprent)
    pub fn new(expr: NewExprent) -> KNewExprent {
        let bytecode = expr.get_bytecode();
        let new = NewExprent::new(expr.get_new_type(), expr.get_lst_dims(), bytecode.as_ref());
        new.0.set_kt(Some(KExprentExt::New(KNewExprentData {
            in_annotation: false,
        })));
        new.set_constructor(expr.get_constructor());
        new.set_lst_array_elements(expr.get_lst_array_elements());
        new.set_direct_array_init(expr.is_direct_array_init());
        new.set_var_arg_param(expr.is_var_arg_param());
        new.set_anonymous(expr.is_anonymous());
        if kuna_fixes::enabled(KunaFix::KtDefaultArgsCall) {
            kuna_fix_kt_default_args_call::resugar_new(expr);
        }

        if KUtils::assertions_enabled() && expr.is_lambda() != new.is_lambda() {
            jthrow!(AssertionError; "");
        }
        KNewExprent(new)
    }

    pub fn get_precedence(self) -> i32 {
        0
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let new_type = self.get_new_type();
        let node = root_class_node(new_type.value.as_deref());
        if let Some(node) = node {
            let class_struct = node.with(|d| d.class_struct);
            if KotlinChooser::new().is_language(class_struct) {
                let kt_data = KtAttrs::get(class_struct, &KElement::KEY());
                if let Some(KElement::FunctionReference(_)) = kt_data {
                    let receiver = self
                        .get_constructor()
                        .unwrap_or_else(|| npe("KNewExprent constructor"))
                        .get_lst_parameters()
                        .get(0);
                    let reference = KotlinWriter::stringify_reference(
                        indent,
                        node,
                        self.get_bytecode().as_ref(),
                        Some(receiver),
                    );
                    if let Some(reference) = reference {
                        return reference;
                    }
                }

                if kt_data.is_none() && kuna_fixes::enabled(KunaFix::KtPropertyReference) {
                    let receiver = self
                        .get_constructor()
                        .and_then(|c| c.get_lst_parameters().to_vec().into_iter().next());
                    let reference = kuna_fix_kt_property_reference::stringify(
                        indent,
                        node,
                        self.get_bytecode().as_ref(),
                        receiver,
                    );
                    if let Some(reference) = reference {
                        return reference;
                    }
                }

                if node.with(|d| d.type_) == ClassNodeType::Local
                    && matches!(kt_data, Some(KElement::Class(_)))
                {
                    node.with_mut(|d| d.type_ = ClassNodeType::Anonymous);
                    buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                    if kuna_fixes::enabled(KunaFix::KtObjectSuperCall) {
                        kuna_fix_kt_object_super_call::write_at_site(
                            node,
                            self.get_constructor(),
                            indent,
                            || KotlinWriter::new().write_class(node, &mut buf, indent),
                        );
                    } else {
                        KotlinWriter::new().write_class(node, &mut buf, indent);
                    }
                    return buf;
                }
            }
        }

        if self.is_lambda() {
            let outer_wrapper = DecompilerContext::get_current_method_wrapper();
            let _finally = RestoreMethodWrapper(outer_wrapper);

            let node = node.unwrap_or_else(|| npe("KNewExprent lambda node"));
            let lambda_info = node
                .with(|d| d.lambda_information.clone())
                .unwrap_or_else(|| npe("lambdaInformation"));
            let name = lambda_info.content_method_name.clone();
            if name.contains("$lambda$") {
                let referenced_desc =
                    MethodDescriptor::parse_descriptor(&lambda_info.method_descriptor);
                let real_desc =
                    MethodDescriptor::parse_descriptor(&lambda_info.content_method_descriptor);

                let wrapper = node
                    .get_wrapper()
                    .unwrap_or_else(|| npe("ClassNode.getWrapper()"))
                    .get_method_wrapper(&name, &lambda_info.content_method_descriptor);
                let wrapper_or_npe = || wrapper.unwrap_or_else(|| npe("lambda method wrapper"));
                buf.append("{ ");

                if !referenced_desc.params.is_empty() {
                    let mut index: i32 = if lambda_info.is_content_method_static {
                        0
                    } else {
                        1
                    };
                    let start_index =
                        real_desc.params.len() as i32 - referenced_desc.params.len() as i32;
                    for i in 0..real_desc.params.len() as i32 {
                        if i < start_index {
                            index += real_desc.params[i as usize].stack_size;
                            continue;
                        } else if i > start_index {
                            buf.append(", ");
                        }

                        let type_ = &real_desc.params[i as usize];
                        let (varproc, method_struct) =
                            wrapper_or_npe().with(|d| (d.varproc, d.method_struct));
                        let clashing_name =
                            varproc.get_clashing_name(VarVersionPair::new(index, 0));
                        let mut parameter_name =
                            varproc.get_var_name(VarVersionPair::new(index, 0));
                        if clashing_name.is_some() {
                            parameter_name = clashing_name;
                        } else if parameter_name.is_none() {
                            parameter_name = Some(format!("param{index}"));
                        }

                        let parameter_name = method_struct
                            .get_variable_namer()
                            .rename_parameter(
                                method_struct.get_access_flags(),
                                type_,
                                parameter_name.as_deref(),
                                index,
                            )
                            .unwrap_or_else(|| npe("parameter name"));
                        buf.append_variable_descriptor(
                            &parameter_name,
                            true,
                            true,
                            &lambda_info.content_class_name,
                            &lambda_info.content_method_name,
                            &real_desc,
                            index,
                            &parameter_name,
                        )
                        .append(": ")
                        .append_type_name_str(&KTypes::get_kotlin_type(Some(type_)), type_);

                        index += type_.stack_size;
                    }

                    buf.append(" ->");
                }

                buf.append_line_separator();

                let wrapper = wrapper_or_npe();
                if wrapper.with(|d| d.decompile_error.is_none()) {
                    DecompilerContext::set_current_method_wrapper(Some(wrapper));
                    let root = wrapper.with(|d| d.root);
                    if let Some(root) = root {
                        let class_name = node.with(|d| d.class_struct).qualified_name();
                        let result = try_catch(|| {
                            let mut exits: Vec<KExitExprent> = Vec::new();
                            let edges = root
                                .get_dummy_exit()
                                .0
                                .get_predecessor_edges(StatEdge::TYPE_BREAK);
                            for edge in edges {
                                let exprents = edge
                                    .get_source()
                                    .get_exprents()
                                    .unwrap_or_else(|| npe("Statement.getExprents()"));
                                for expr in exprents.iter() {
                                    if let Some(kexpr) = expr.try_as_k_exit() {
                                        if kexpr.get_exit_type() == ExitType::Return {
                                            exits.push(kexpr);
                                        }
                                    }
                                }
                            }

                            if exits.len() > 1 {
                                let at = name.find("$lambda$").expect("contains $lambda$");
                                let lambda_name = name[at + 1..].replace('$', "_");
                                buf.prepend(&format!("{lambda_name}@"));

                                for exit in &exits {
                                    exit.set_lambda_name(Some(lambda_name.clone()));
                                }
                            }

                            let mut child = root.0.to_java(indent + 1);
                            child.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                            buf.append_buffer_mapped(
                                &child,
                                Some(&class_name),
                                Some(&InterpreterUtil::make_unique_key(
                                    &name,
                                    &lambda_info.content_method_descriptor,
                                )),
                            );
                        });
                        if let Err(t) = result {
                            if is_stop_signal(&t) {
                                rethrow(t);
                            }
                            let message = format!(
                                "Lambda {} {} in class {} couldn't be written.",
                                name, lambda_info.content_method_descriptor, class_name
                            );
                            DecompilerContext::get_logger().write_message_ex(
                                &message,
                                Severity::Warn,
                                &t,
                            );
                            wrapper.with_mut(|d| d.decompile_error = Some(t));
                        }
                    }
                }

                if let Some(decompile_error) = wrapper.with(|d| d.decompile_error.clone()) {
                    crate::kuna_spans::method_failed(wrapper);
                    let mut lines: Vec<String> = Vec::new();
                    if DecompilerContext::get_option(
                        IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR,
                    ) {
                        lines.extend(ClassWriter::get_error_comment());
                        ClassWriter::collect_error_lines(&decompile_error, &mut lines);
                    }

                    for line in &lines {
                        buf.append_indent(indent + 1)
                            .append("// ")
                            .append(line)
                            .append_line_separator();
                    }
                }

                buf.append_indent(indent).append("}");

                return buf;
            }
        } else if self.is_anonymous() {
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            let node = node.unwrap_or_else(|| npe("KNewExprent anonymous node"));
            let write = |buf: &mut TextBuffer| {
                if kuna_fixes::enabled(KunaFix::KtSyntheticAnonymousObject) {
                    kuna_fix_kt_synthetic_anonymous_object::write_at_site(
                        node,
                        self.get_constructor(),
                        || KotlinWriter::new().write_class(node, buf, indent),
                    );
                } else {
                    KotlinWriter::new().write_class(node, buf, indent);
                }
            };
            if kuna_fixes::enabled(KunaFix::KtObjectSuperCall) {
                kuna_fix_kt_object_super_call::write_at_site(
                    node,
                    self.get_constructor(),
                    indent,
                    || write(&mut buf),
                );
            } else {
                write(&mut buf);
            }
            return buf;
        }

        let constructor = self.get_constructor();
        if new_type.array_dim == 0 {
            if !self.is_enum_const() {
                let mut type_name = Some(KTypes::get_kotlin_type(Some(&new_type)));

                if let Some(constructor) = constructor {
                    let enclosing = new_type.value.as_deref().and_then(|value| {
                        NewExprent::get_qualified_new_instance(
                            value,
                            &constructor.get_lst_parameters().to_vec(),
                            indent,
                        )
                    });
                    if let Some(enclosing) = enclosing {
                        buf.append_buffer(&enclosing).append_char('.');
                        let new_node = root_class_node(new_type.value.as_deref());
                        type_name = match new_node {
                            Some(new_node) => new_node.with(|d| d.simple_name.clone()),
                            None => type_name.map(|t| match t.rfind('.') {
                                Some(dot) => t[dot + 1..].to_string(),
                                None => t,
                            }),
                        };
                    }
                }

                buf.append_type_name_str(&type_name.unwrap_or_else(|| npe("typeName")), &new_type);
            }

            if let Some(constructor) = constructor {
                if !self.is_enum_const() || constructor.get_lst_parameters().len() > 2 {
                    let generic_args = constructor.get_generic_args();
                    if !(kuna_fixes::enabled(KunaFix::KtConstructorDiamond)
                        && is_diamond(&generic_args))
                    {
                        self.0 .0.append_parameters(&mut buf, &generic_args);
                    }
                    buf.append("(")
                        .append_buffer(&constructor.append_param_list(indent))
                        .append_char(')');
                }
            }
        } else if self.is_var_arg_param() {
            let mut first = true;
            for element in self.get_lst_array_elements().iter() {
                if !first {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                buf.append_buffer(&element.to_java(indent));
                first = false;
            }
        } else if self.kd(|d| d.in_annotation) {
            buf.append_char('[');
            buf.push_newline_group(indent, 1);
            buf.append_possible_newline();
            let mut first = true;
            for element in self.get_lst_array_elements().iter() {
                if !first {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                if let Some(knew) = element.try_as_k_new() {
                    knew.set_in_annotation(true);
                }
                buf.append_buffer(&element.to_java(indent));
                first = false;
            }
            buf.append_possible_newline_dedent("", true);
            buf.pop_newline_group();
            buf.append_char(']');
        } else if self.get_lst_array_elements().is_empty()
            && !self.get_lst_dims().is_empty()
            && (self.get_lst_dims().len() as i32) < new_type.array_dim
            && kuna_fixes::enabled(KunaFix::KtPartialArrayDims)
        {
            self.partial_dims_to_java(&mut buf, indent);
        } else if self.get_lst_array_elements().is_empty() {
            for i in 0..new_type.array_dim - 1 {
                buf.append("Array(");
                let dim = self.get_lst_dims().get(i as usize);
                if dim.get_type() == ExprentType::Const {
                    dim.as_const().adjust_const_type(&VarType::VARTYPE_INT());
                }
                buf.append_buffer(&dim.to_java(indent));
                buf.append(") {");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline_alt(" ");
            }

            buf.append(match new_type.resize_array_dim(0).type_ {
                CodeType::Boolean => "BooleanArray",
                CodeType::Byte => "ByteArray",
                CodeType::Short => "ShortArray",
                CodeType::Char => "CharArray",
                CodeType::Int => "IntArray",
                CodeType::Float => "FloatArray",
                CodeType::Long => "LongArray",
                CodeType::Double => "DoubleArray",
                _ => "arrayOfNulls",
            })
            .append_char('(');

            let dim = self.get_lst_dims().get((new_type.array_dim - 1) as usize);
            if dim.get_type() == ExprentType::Const {
                dim.as_const().adjust_const_type(&VarType::VARTYPE_INT());
            }
            buf.append_buffer(&dim.to_java(indent)).append_char(')');

            for _ in 0..new_type.array_dim - 1 {
                buf.append_possible_newline_dedent(" ", true);
                buf.pop_newline_group();
                buf.append_char('}');
            }
        } else {
            buf.append(match new_type.decrease_array_dim().type_ {
                CodeType::Boolean => "booleanArrayOf",
                CodeType::Byte => "byteArrayOf",
                CodeType::Short => "shortArrayOf",
                CodeType::Char => "charArrayOf",
                CodeType::Int => "intArrayOf",
                CodeType::Float => "floatArrayOf",
                CodeType::Long => "longArrayOf",
                CodeType::Double => "doubleArrayOf",
                _ => "arrayOf",
            });
            buf.append_char('(');
            buf.push_newline_group(indent, 1);
            buf.append_possible_newline();
            let mut first = true;
            for element in self.get_lst_array_elements().iter() {
                if !first {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                buf.append_buffer(&element.to_java(indent));
                first = false;
            }
            buf.append_possible_newline_dedent("", true);
            buf.pop_newline_group();
            buf.append_char(')');
        }

        buf
    }

    pub fn copy(self) -> Exprent {
        KNewExprent::new(NewExprent::copy(self.0).as_new()).0 .0
    }

    /// `kt_partial_array_dims`: `Array(d0) { .. arrayOfNulls(dn) }` for an
    /// array creation whose sized dimensions `d0..dn` stop short of its type's,
    /// so the innermost sized level holds null arrays.
    fn partial_dims_to_java(self, buf: &mut TextBuffer, indent: i32) {
        let dims = self.get_lst_dims();
        let sized = dims.len();
        for i in 0..sized {
            let dim = dims.get(i);
            if dim.get_type() == ExprentType::Const {
                dim.as_const().adjust_const_type(&VarType::VARTYPE_INT());
            }
            if i + 1 < sized {
                buf.append("Array(");
                buf.append_buffer(&dim.to_java(indent));
                buf.append(") {");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline_alt(" ");
            } else {
                buf.append("arrayOfNulls(");
                buf.append_buffer(&dim.to_java(indent)).append_char(')');
            }
        }
        for _ in 1..sized {
            buf.append_possible_newline_dedent(" ", true);
            buf.pop_newline_group();
            buf.append_char('}');
        }
    }

    pub fn set_in_annotation(self, in_annotation: bool) {
        self.kd_mut(|d| d.in_annotation = in_annotation)
    }
}

/// Java's diamond: the one placeholder argument `InvocationExprent` records
/// when the arguments are left to inference.
fn is_diamond(generic_args: &[VarType]) -> bool {
    matches!(generic_args, [arg] if GenericType::DUMMY_VAR().equals(arg))
}
