//! Port of `org/vineflower/kotlin/expr/KInvocationExprent.java`.
//!
//! kuna fix `kt_default_args_call`: a call of a default-arguments bridge is
//! rewritten into the call it stands for (`kuna_fix_kt_default_args_call`).
//! kuna fix `kt_operator_precedence`: an infix call's operands are
//! parenthesised by Kotlin's precedence (`kuna_fix_kt_operator_precedence`).

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_element::KElement;
use crate::k_property::PropertyMethod;
use crate::k_types::KTypes;
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kotlin::kuna_fix_kt_operator_precedence;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_exprent;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

pub struct KInvocationExprentData {
    pub calling_data: Option<KElement>,
    pub property_method: Option<PropertyMethod>,
}

kotlin_exprent!(pub KInvocationExprent: InvocationExprent, Invocation(KInvocationExprentData),
                is_k_invocation, as_k_invocation, try_as_k_invocation);

fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

impl KInvocationExprent {
    /// Java: KInvocationExprent(InvocationExprent)
    pub fn new(expr: InvocationExprent) -> KInvocationExprent {
        let inv = InvocationExprent::new_copy(expr);
        inv.0
            .set_kt(Some(KExprentExt::Invocation(KInvocationExprentData {
                calling_data: None,
                property_method: None,
            })));
        let kinv = KInvocationExprent(inv);
        let expr = if kuna_fixes::enabled(KunaFix::KtDefaultArgsCall)
            && kuna_fix_kt_default_args_call::resugar_call(inv)
        {
            inv
        } else {
            expr
        };

        let Some(cl) = DecompilerContext::get_struct_context().get_class(&expr.get_classname())
        else {
            return kinv;
        };

        let Some(method) = cl.get_method(&expr.get_name(), &expr.get_string_descriptor()) else {
            return kinv;
        };

        KotlinChooser::parse_metadata_for(cl);
        let calling_data = KtAttrs::get(method, &KElement::KEY());
        let property_method = KtAttrs::get(method, &PropertyMethod::KEY());
        kinv.kd_mut(|d| {
            d.calling_data = calling_data;
            d.property_method = property_method;
        });
        kinv
    }

    pub fn get_precedence(self) -> i32 {
        if let Some(KElement::Function(function)) = self.kd(|d| d.calling_data.clone()) {
            return if Flags::IS_INFIX.get(function.flags) {
                9
            } else {
                Exprent::get_precedence_base(self.0 .0)
            };
        }
        Exprent::get_precedence_base(self.0 .0)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let (calling_data, property_method) =
            self.kd(|d| (d.calling_data.clone(), d.property_method));

        if let Some(KElement::Property(property)) = &calling_data {
            let property_method =
                property_method.unwrap_or_else(|| npe("KInvocationExprent.propertyMethod"));

            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            if let Some(instance) = self.get_instance() {
                let mut instance_buf = instance.to_java(indent);
                if instance.get_precedence() > self.0 .0.get_precedence() {
                    instance_buf.enclose("(", ")");
                }
                buf.append_buffer(&instance_buf).append_char('.');
            }
            buf.append(&property.name);
            if property_method == PropertyMethod::Setter {
                buf.append(" = ");
                let new_value = self.get_lst_parameters().get(0);
                buf.append_buffer(&new_value.to_java(indent));
            }
            return buf;
        }

        if let Some(KElement::Function(function)) = &calling_data {
            if Flags::IS_INFIX.get(function.flags) {
                let mut buf = TextBuffer::new();
                buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                let is_extension_function = function.receiver_type.is_some();
                let instance = if is_extension_function {
                    Some(self.get_lst_parameters().get(0))
                } else {
                    self.get_instance()
                };
                let arg = if is_extension_function {
                    self.get_lst_parameters().get(1)
                } else {
                    self.get_lst_parameters().get(0)
                };
                let instance = instance.unwrap_or_else(|| npe("KInvocationExprent instance"));
                let mut instance_buf = instance.to_java(indent);
                if instance.get_precedence() > self.0 .0.get_precedence()
                    || kuna_fix_kt_operator_precedence::infix_call_operand(instance, false)
                {
                    instance_buf.enclose("(", ")");
                }
                buf.append_buffer(&instance_buf)
                    .append(" ")
                    .append(&function.name)
                    .append(" ");
                let mut arg_buf = arg.to_java(indent);
                if kuna_fix_kt_operator_precedence::infix_call_operand(arg, true) {
                    arg_buf.enclose("(", ")");
                }
                buf.append_buffer(&arg_buf);
                return buf;
            }
        }

        if KTypes::is_function_type(&VarType::new_signature_cl(&self.get_classname(), true)) {
            let mut buf = TextBuffer::new();
            let instance = self
                .get_instance()
                .unwrap_or_else(|| npe("KInvocationExprent instance"));
            let mut instance_buf = instance.to_java(indent);
            if instance.get_precedence() > self.0 .0.get_precedence() {
                instance_buf.enclose("(", ")");
            }
            buf.append_buffer(&instance_buf);
            let classname = self.get_classname();
            let name = self.get_name();
            if self.get_lst_parameters().is_empty() {
                buf.append_method_descriptor(
                    "()",
                    false,
                    &classname,
                    &name,
                    &self.get_descriptor(),
                );
                buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                return buf;
            }

            buf.append_method_descriptor("(", false, &classname, &name, &self.get_descriptor());
            buf.append_buffer(&self.append_param_list(indent));
            buf.append_method_descriptor(")", false, &classname, &name, &self.get_descriptor());
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            return buf;
        }

        InvocationExprent::to_java(self.0, indent)
    }

    pub fn copy(self) -> Exprent {
        KInvocationExprent::new(self.0).0 .0
    }
}
