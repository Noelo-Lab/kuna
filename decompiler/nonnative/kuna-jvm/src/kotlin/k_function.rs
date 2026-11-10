//! Port of `org/vineflower/kotlin/struct/KFunction.java`.
//!
//! The record lives for the session (it is attached to members by `kt_attrs`);
//! its suppliers take the writing class's `ClassWrapper` and build what they
//! return in that class's scope.
//!
//! kuna fix `kt_type_parameter_id`: each member's types are read in the
//! scope of its own type parameters (`kuna_fix_kt_type_parameter_id`).
//!
//! kuna fix `kt_abstract_function_body`: an abstract or external function
//! (no code in the class file) is written without a body, where Vineflower
//! writes an empty one, which Kotlin rejects.
//!
//! kuna fix `kt_missed_override`: a function that a bridge method of its
//! class calls is an override (`kuna_fix_kt_missed_override`).
//!
//! kuna fix `kt_extension_receiver`: an extension function's body writes
//! its receiver `this` (`kuna_fix_kt_extension_receiver`).

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_ABSTRACT, ACC_NATIVE, ACC_STATIC};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::default_args_map::{DefaultArgsMap, DefaultArgsSupplier};
use crate::i_fernflower_logger::Severity;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::k_contract::KContract;
use crate::k_element::KElement;
use crate::k_hidden_element::KHiddenElement;
use crate::k_parameter::KParameter;
use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::k_utils::KUtils;
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_writer::{is_stop_signal, KotlinWriter};
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::{self, Modality, Visibility};
use crate::kotlin::kuna_fix_kt_extension_receiver;
use crate::kotlin::kuna_fix_kt_missed_override;
use crate::kuna_fix_kt_java_reference_cast;
use crate::kuna_fix_kt_type_parameter_id;
use crate::kuna_fixes::{self, KunaFix};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::method_wrapper::MethodWrapper;
use crate::rt::{rethrow, try_catch};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::type_annotation::TypeAnnotation;

/// `Function<ClassWrapper, MethodWrapper>`; `getMethodWrapper` may answer `null`.
pub type MethodSupplier = Rc<dyn Fn(ClassWrapper) -> Option<MethodWrapper>>;

pub struct KFunction {
    pub name: String,
    pub parameters: Rc<[KParameter]>,
    pub type_parameters: Vec<KTypeParameter>,
    pub return_type: KType,
    pub flags: i32,
    pub context_receiver_types: Vec<KType>,
    pub method_supplier: MethodSupplier,
    pub receiver_type: Option<KType>,
    pub contract: Option<KContract>,
    pub known_override: bool,
    pub default_args_supplier: DefaultArgsSupplier,
    /// `null` only in `FAILED_LAMBDA`.
    pub class_struct: Option<StructClass>,
    /// `null` only in `FAILED_LAMBDA`.
    pub method_struct: Option<StructMethod>,
}

thread_local! {
    static FAILED_LAMBDA: Rc<KFunction> = Rc::new(KFunction::new(
        "failed-lambda".to_string(),
        Rc::from(Vec::new()),
        Vec::new(),
        KType::NOTHING(),
        0,
        Vec::new(),
        Rc::new(|_: ClassWrapper| None),
        None,
        None,
        false,
        Rc::new(|_: ClassWrapper| -> DefaultArgsMap {
            jthrow!(IllegalStateException; "Attempted to read default arguments for failed lambda")
        }),
        None,
        None,
    ));
}

impl KFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        parameters: Rc<[KParameter]>,
        type_parameters: Vec<KTypeParameter>,
        return_type: KType,
        flags: i32,
        context_receiver_types: Vec<KType>,
        method_supplier: MethodSupplier,
        receiver_type: Option<KType>,
        contract: Option<KContract>,
        known_override: bool,
        default_args_supplier: DefaultArgsSupplier,
        class_struct: Option<StructClass>,
        method_struct: Option<StructMethod>,
    ) -> KFunction {
        KFunction {
            name,
            parameters,
            type_parameters,
            return_type,
            flags,
            context_receiver_types,
            method_supplier,
            receiver_type,
            contract,
            known_override,
            default_args_supplier,
            class_struct,
            method_struct,
        }
    }

    /// `KFunction.FAILED_LAMBDA`; compare with `Rc::ptr_eq` (Java `==`).
    #[allow(non_snake_case)]
    pub fn FAILED_LAMBDA() -> Rc<KFunction> {
        FAILED_LAMBDA.with(Rc::clone)
    }

    pub fn parse(
        class_struct: StructClass,
        resolver: Option<&MetadataNameResolver>,
        type_table: &kt_proto::TypeTable,
        proto_functions: &[kt_proto::Function],
        is_synthetic_function: bool,
        companion_parent: Option<StructClass>,
    ) {
        let Some(resolver) = resolver else {
            return;
        };

        for function in proto_functions {
            let _type_parameters = kuna_fixes::enabled(KunaFix::KtTypeParameterId).then(|| {
                kuna_fix_kt_type_parameter_id::member_scope(
                    &function.get_type_parameter_list(),
                    resolver,
                )
            });
            let jvm_data = function.get_extension_method_signature();

            let flags = function.get_flags();

            let name = resolver.resolve(function.get_name());

            let mut parameters: Vec<KParameter> =
                Vec::with_capacity(function.get_value_parameter_count() as usize);
            for i in 0..function.get_value_parameter_count() {
                let parameter = function.get_value_parameter(i);
                let param_flags = parameter.get_flags();
                let param_name = resolver.resolve(parameter.get_name());
                let param_type = KType::from(&parameter.get_type(), resolver);
                let vararg_type = if parameter.has_vararg_element_type() {
                    Some(KType::from(&parameter.get_vararg_element_type(), resolver))
                } else {
                    None
                };
                let type_id = parameter.get_type_id();
                parameters.push(KParameter::new(
                    param_flags,
                    param_name,
                    param_type,
                    vararg_type,
                    type_id,
                ));
            }
            let parameters: Rc<[KParameter]> = Rc::from(parameters);

            let mut receiver_type: Option<KType> = None;
            if function.has_receiver_type() {
                receiver_type = Some(KType::from(&function.get_receiver_type(), resolver));
            }

            let return_type = KType::from(&function.get_return_type(), resolver);

            let mut method_lookup: Option<StructMethod> = None;
            let lookup_name = if jvm_data.has_name() {
                resolver.resolve(jvm_data.get_name())
            } else {
                name.clone()
            };
            if jvm_data.has_desc() {
                method_lookup =
                    class_struct.get_method(&lookup_name, &resolver.resolve(jvm_data.get_desc()));
            }

            if method_lookup.is_none() {
                let mut desc = String::from("(");
                if let Some(receiver_type) = &receiver_type {
                    desc.push_str(&receiver_type.0.to_string());
                }

                for parameter in parameters.iter() {
                    desc.push_str(&parameter.type_.0.to_string());
                }

                desc.push(')');
                desc.push_str(&return_type.0.to_string());

                method_lookup = class_struct.get_method(&lookup_name, &desc);

                if method_lookup.is_none() {
                    if Flags::IS_SUSPEND.get(flags) && "<anonymous>" == name {
                        continue;
                    }
                    DecompilerContext::get_logger().write_message(
                        &format!(
                            "Couldn't find method {} {} in class {}",
                            name,
                            desc,
                            class_struct.qualified_name()
                        ),
                        Severity::Error,
                    );
                    continue;
                }

                if let Some(companion_parent) = companion_parent {
                    if let Some(in_parent) = companion_parent.get_method(&lookup_name, &desc) {
                        KtAttrs::put(
                            in_parent,
                            &KElement::KEY(),
                            KElement::HiddenElement(KHiddenElement::CompanionItem),
                        );
                    }
                }
            }
            let method = method_lookup.expect("method lookup");

            let type_parameters: Vec<KTypeParameter> = function
                .get_type_parameter_list()
                .iter()
                .map(|type_parameter| KTypeParameter::from(type_parameter, resolver))
                .collect();

            let context_receiver_types: Vec<KType> = function
                .get_context_receiver_type_list()
                .iter()
                .map(|ctx_type| KType::from(ctx_type, resolver))
                .collect();

            let is_static = method.has_modifier(ACC_STATIC);
            let default_args_name = format!("{}$default", name);
            let mut default_args_desc = String::from("(");
            if !is_static {
                default_args_desc.push('L');
                default_args_desc.push_str(&class_struct.qualified_name());
                default_args_desc.push(';');
            }
            if let Some(receiver_type) = &receiver_type {
                default_args_desc.push_str(&receiver_type.0.to_string());
            }
            for (i, parameter) in parameters.iter().enumerate() {
                let named = !kuna_fixes::enabled(KunaFix::KtTypeParameterId)
                    || function
                        .get_value_parameter(i as i32)
                        .get_type()
                        .has_type_parameter_name();
                if let Some(type_parameter_name) = parameter
                    .type_
                    .k()
                    .type_parameter_name
                    .as_ref()
                    .filter(|_| named)
                {
                    let bound = type_parameters
                        .iter()
                        .find(|type_parameter| &type_parameter.name == type_parameter_name)
                        .map(|type_parameter| &type_parameter.upper_bounds)
                        .filter(|bounds| bounds.len() == 1)
                        .map(|bounds| &bounds[0]);
                    match bound {
                        Some(bound) => default_args_desc.push_str(&bound.0.to_string()),
                        None => default_args_desc.push_str("Ljava/lang/Object;"),
                    }
                } else {
                    default_args_desc.push_str(&parameter.type_.0.to_string());
                }
            }

            default_args_desc.push_str(&"I".repeat(parameters.len() / 32 + 1));
            default_args_desc.push_str("Ljava/lang/Object;)");
            default_args_desc.push_str(&return_type.0.to_string());

            let default_method = class_struct.get_method(&default_args_name, &default_args_desc);
            if let Some(default_method) = default_method {
                KtAttrs::put(
                    default_method,
                    &KElement::KEY(),
                    KElement::HiddenElement(KHiddenElement::DefaultImpl),
                );
            }

            if let Some(companion_parent) = companion_parent {
                let in_parent = companion_parent.get_method(&default_args_name, &default_args_desc);
                if let Some(in_parent) = in_parent {
                    KtAttrs::put(
                        in_parent,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::DefaultImpl),
                    );
                }
            }

            let default_args: DefaultArgsSupplier = {
                let parameters = parameters.clone();
                Rc::new(move |wrapper: ClassWrapper| {
                    DefaultArgsMap::from(
                        wrapper.get_method_wrapper(&default_args_name, &default_args_desc),
                        wrapper.get_method_wrapper(&method.get_name(), &method.get_descriptor()),
                        &parameters,
                    )
                })
            };

            let visibility = Flags::VISIBILITY.get(flags);

            let known_override = visibility != Some(Visibility::Private)
                && visibility != Some(Visibility::PrivateToThis)
                && visibility != Some(Visibility::Local)
                && (KotlinWriter::search_for_method(
                    Some(class_struct),
                    &method.get_name(),
                    &method.method_descriptor(),
                    false,
                ) || kuna_fixes::enabled(KunaFix::KtMissedOverride)
                    && kuna_fix_kt_missed_override::bridged(class_struct, method));

            let contract = if function.has_contract() {
                Some(KContract::from(
                    &function.get_contract(),
                    &parameters,
                    resolver,
                    type_table,
                ))
            } else {
                None
            };

            let method_supplier: MethodSupplier = Rc::new(move |wrapper: ClassWrapper| {
                wrapper.get_method_wrapper(&method.get_name(), &method.get_descriptor())
            });

            let k_function = Rc::new(KFunction::new(
                name,
                parameters,
                type_parameters,
                return_type,
                flags,
                context_receiver_types,
                method_supplier,
                receiver_type,
                contract,
                known_override,
                default_args,
                Some(class_struct),
                Some(method),
            ));

            KtAttrs::put(
                method,
                &KElement::KEY(),
                KElement::Function(k_function.clone()),
            );

            if is_synthetic_function {
                KtAttrs::put(
                    class_struct,
                    &KElement::KEY(),
                    KElement::Function(k_function),
                );
            }
        }
    }

    pub fn stringify(&self, wrapper: ClassWrapper, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let method_struct = self
            .method_struct
            .expect("NullPointerException: methodStruct");
        KotlinWriter::append_annotations(
            &mut buf,
            indent,
            method_struct.into(),
            TypeAnnotation::METHOD_RETURN_TYPE,
        );
        let class_struct = self
            .class_struct
            .expect("NullPointerException: classStruct");
        let class_data = KtAttrs::get(class_struct, &KElement::KEY());
        let is_in_file = matches!(class_data, Some(KElement::File(_)));
        KotlinWriter::append_jvm_annotations(
            &mut buf,
            indent,
            method_struct.into(),
            false,
            is_in_file,
            class_struct.get_pool(),
            TypeAnnotation::METHOD_RETURN_TYPE,
        );

        let method_key = InterpreterUtil::make_unique_key(
            &method_struct.get_name(),
            &method_struct.get_descriptor(),
        );

        buf.append_indent(indent);

        if !self.context_receiver_types.is_empty() {
            buf.append("context(");
            let mut first = true;
            for context_receiver_type in &self.context_receiver_types {
                if !first {
                    buf.append(", ");
                }

                buf.append_buffer(&context_receiver_type.stringify(indent + 1));
                first = false;
            }
            buf.append(")")
                .append_line_separator()
                .append_indent(indent);
        }

        let flags = self.flags;
        if Flags::VISIBILITY.get(flags) != Some(Visibility::Public)
            || DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY)
        {
            KUtils::append_visibility(&mut buf, Flags::VISIBILITY.get(flags));
        }

        if Flags::IS_EXPECT_FUNCTION.get(flags) {
            buf.append("expect ");
        }

        if Flags::MODALITY.get(flags) != Some(Modality::Final)
            && (!self.known_override || Flags::MODALITY.get(flags) != Some(Modality::Open))
        {
            buf.append(&modality_name(Flags::MODALITY.get(flags)))
                .append_char(' ');
        }

        if Flags::IS_EXTERNAL_FUNCTION.get(flags) {
            buf.append("external ");
        }

        if self.known_override {
            buf.append("override ");
        }

        if Flags::IS_TAILREC.get(flags) {
            buf.append("tailrec ");
        }

        if Flags::IS_SUSPEND.get(flags) {
            buf.append("suspend ");
        }

        if Flags::IS_INLINE.get(flags) {
            buf.append("inline ");
        }

        if Flags::IS_INFIX.get(flags) {
            buf.append("infix ");
        }

        if Flags::IS_OPERATOR.get(flags) {
            buf.append("operator ");
        }

        buf.append("fun ");

        let complex_type_params: Vec<&KTypeParameter> = self
            .type_parameters
            .iter()
            .filter(|type_parameter| type_parameter.upper_bounds.len() > 1)
            .collect();

        if !self.type_parameters.is_empty() {
            buf.append_char('<');

            for (i, type_parameter) in self.type_parameters.iter().enumerate() {
                if type_parameter.reified {
                    buf.append("reified ");
                }

                buf.append(&KotlinWriter::to_valid_kotlin_identifier(
                    &type_parameter.name,
                ));

                if type_parameter.upper_bounds.len() == 1 {
                    buf.append(" : ")
                        .append_buffer(&type_parameter.upper_bounds[0].stringify(indent + 1));
                }

                if i + 1 < self.type_parameters.len() {
                    buf.append(", ");
                }
            }

            buf.append("> ");
        }

        if let Some(receiver_type) = &self.receiver_type {
            let wrap = !receiver_type.k().is_nullable
                && receiver_type.k().kotlin_type.starts_with("kotlin/Function");
            if wrap {
                buf.append("(");
            }
            buf.append_buffer(&receiver_type.stringify(indent + 1));
            if wrap {
                buf.append(")");
            }

            buf.append(".");
        }

        buf.append(&KotlinWriter::to_valid_kotlin_identifier(&self.name))
            .append_char('(')
            .push_newline_group(indent, 1)
            .append_possible_newline_alt("");

        let qualified_name = class_struct.qualified_name();
        let mut first = true;
        for parameter in self.parameters.iter() {
            if !first {
                buf.append(",").append_possible_newline_alt(" ");
            }

            first = false;

            parameter.stringify(indent + 1, &mut buf);
            if Flags::DECLARES_DEFAULT_VALUE.get(parameter.flags) {
                let default_value =
                    (self.default_args_supplier)(wrapper).to_java(parameter, indent + 1);
                buf.append_buffer_mapped(&default_value, Some(&qualified_name), Some(&method_key));
            }
        }

        buf.append_possible_newline_dedent("", true)
            .pop_newline_group()
            .append_char(')');

        if self.return_type.type_ != CodeType::Void {
            buf.append(": ")
                .append_buffer(&self.return_type.stringify(indent + 1));
        }

        let no_body = kuna_fixes::enabled(KunaFix::KtAbstractFunctionBody)
            && (method_struct.get_access_flags() & (ACC_ABSTRACT | ACC_NATIVE)) != 0
            && (Flags::MODALITY.get(flags) == Some(Modality::Abstract)
                || Flags::IS_EXTERNAL_FUNCTION.get(flags));

        if complex_type_params.is_empty() {
            if !no_body {
                buf.append_char(' ');
            }
        } else {
            buf.push_newline_group(indent, 1)
                .append_possible_newline_alt(" ")
                .append("where ");

            first = true;
            for type_parameter in &complex_type_params {
                for upper_bound in &type_parameter.upper_bounds {
                    if !first {
                        buf.append_possible_newline_alt(",")
                            .append_possible_newline_alt(" ");
                    }

                    buf.append(&KotlinWriter::to_valid_kotlin_identifier(
                        &type_parameter.name,
                    ))
                    .append(" : ")
                    .append_buffer(&upper_bound.stringify(indent + 1));

                    first = false;
                }
            }

            buf.append_possible_newline_dedent(if no_body { "" } else { " " }, true)
                .pop_newline_group();
        }

        if no_body {
            buf.append_line_separator();
            return buf;
        }

        buf.append_char('{').append_line_separator();

        if let Some(contract) = &self.contract {
            buf.append_buffer(&contract.stringify(indent + 1));
        }

        let method_wrapper =
            (self.method_supplier)(wrapper).expect("NullPointerException: methodWrapper");
        let _receiver = (kuna_fixes::enabled(KunaFix::KtExtensionReceiver)
            && self.receiver_type.is_some())
        .then(|| {
            kuna_fix_kt_extension_receiver::enter(
                class_struct,
                method_wrapper,
                method_struct.has_modifier(ACC_STATIC),
            )
        })
        .flatten();
        let (root, decompile_error) =
            method_wrapper.with(|d| (d.root, d.decompile_error.is_some()));
        if let (Some(root), false) = (root, decompile_error) {
            let result = try_catch(|| {
                let mut body = kuna_fix_kt_java_reference_cast::with_nullable_parameters(
                    nullable_parameters(&self.parameters),
                    || root.to_java(indent + 1),
                );
                body.add_bytecode_mapping(root.get_dummy_exit().bytecode().as_ref());
                if body.length() != 0 && self.contract.is_some() {
                    buf.append_line_separator();
                }

                buf.append_buffer_mapped(&body, Some(&qualified_name), Some(&method_key));
            });
            if let Err(t) = result {
                if is_stop_signal(&t) {
                    rethrow(t);
                }
                let message = format!(
                    "Method {} {} in class {} couldn't be written.",
                    method_struct.get_name(),
                    method_wrapper.desc(),
                    qualified_name
                );
                DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);
                method_wrapper.with_mut(|d| d.decompile_error = Some(t));
            }
        }

        if method_wrapper.with(|d| d.decompile_error.is_some()) {
            KotlinWriter::dump_error(&mut buf, method_wrapper, indent + 1);
        }

        buf.append_indent(indent)
            .append_char('}')
            .append_line_separator();

        buf
    }
}

/// `MODALITY.get(flags).name().toLowerCase()`; an unknown modality is the Java NPE.
pub(crate) fn modality_name(modality: Option<Modality>) -> String {
    modality
        .expect("NullPointerException: modality")
        .name()
        .to_ascii_lowercase()
}

/// The names of the parameters declared nullable (fix `kt_java_reference_cast`).
pub fn nullable_parameters(parameters: &[KParameter]) -> Vec<String> {
    parameters
        .iter()
        .filter(|p| p.type_.k().is_nullable)
        .map(|p| p.name.clone())
        .collect()
}
