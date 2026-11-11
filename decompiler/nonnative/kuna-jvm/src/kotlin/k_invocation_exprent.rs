//! Port of `org/vineflower/kotlin/expr/KInvocationExprent.java`.
//!
//! kuna fix `kt_default_args_call`: a call of a default-arguments bridge is
//! rewritten into the call it stands for (`kuna_fix_kt_default_args_call`).
//! kuna fix `kt_operator_precedence`: an infix call's operands are
//! parenthesised by Kotlin's precedence (`kuna_fix_kt_operator_precedence`).
//! kuna fix `kt_super_property_access`: an accessor called with
//! `invokespecial` on `this` is written on `super`
//! (`kuna_fix_kt_super_property_access`).
//! kuna fix `kt_synthetic_accessor_call`: a call of a synthetic accessor is
//! the access it makes (`kuna_fix_kt_synthetic_accessor_call`).
//! kuna fixes `kt_facade_call`, `kt_extension_call` and `kt_jvm_name_call`:
//! a call of a top-level, extension or renamed function or property is
//! written as Kotlin calls it (`kuna_fix_kt_facade_call`,
//! `kuna_fix_kt_extension_call`, `kuna_fix_kt_jvm_name_call`).
//! kuna fix `kt_inlined_stdlib_call`: a call rebuilt from kotlinc's inlined
//! copy of a standard-library function is written as Kotlin calls it
//! (`kuna_fix_kt_inlined_stdlib_call`).
//! kuna fix `kt_object_monitor_call`: `wait`, `notify` and `notifyAll` are
//! called on the receiver cast to `java.lang.Object`
//! (`kuna_fix_kt_object_monitor_call`).

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_element::KElement;
use crate::k_property::PropertyMethod;
use crate::k_types::KTypes;
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kotlin::kuna_fix_kt_extension_call;
use crate::kotlin::kuna_fix_kt_facade_call;
use crate::kotlin::kuna_fix_kt_inlined_stdlib_call;
use crate::kotlin::kuna_fix_kt_jvm_name_call;
use crate::kotlin::kuna_fix_kt_object_monitor_call;
use crate::kotlin::kuna_fix_kt_synthetic_accessor_call;
use crate::kotlin::kuna_fix_kt_operator_precedence;
use crate::kotlin::kuna_fix_kt_super_property_access;
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

        let method = cl.get_method(&expr.get_name(), &expr.get_string_descriptor());
        let mangled = expr.get_name().contains('-');
        let method = if kuna_fixes::enabled(KunaFix::KtFacadeCall)
            && !mangled
            && method.is_none_or(|m| !KtAttrs::has(m, &KElement::KEY()))
        {
            kuna_fix_kt_facade_call::part_method(
                cl,
                &expr.get_name(),
                &expr.get_string_descriptor(),
            )
            .or(method)
        } else {
            method
        };
        let method = if kuna_fixes::enabled(KunaFix::KtJvmNameCall) && !mangled && method.is_none() {
            kuna_fix_kt_jvm_name_call::inherited(cl, &expr.get_name(), &expr.get_string_descriptor())
        } else {
            method
        };
        let Some(method) = method else {
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
        if let Some(buf) = kuna_fix_kt_inlined_stdlib_call::write(self.0, indent) {
            return buf;
        }
        let (calling_data, property_method) =
            self.kd(|d| (d.calling_data.clone(), d.property_method));

        if calling_data.is_none() && kuna_fixes::enabled(KunaFix::KtSyntheticAccessorCall) {
            if let Some(access) = kuna_fix_kt_synthetic_accessor_call::resugar(self.0) {
                self.kd_mut(|d| {
                    d.calling_data = Some(access.calling_data.clone());
                    d.property_method = access.property_method;
                });
                let buf = self.to_java(indent);
                self.kd_mut(|d| {
                    d.calling_data = None;
                    d.property_method = None;
                });
                drop(access);
                return buf;
            }
        }

        if let Some(buf) = self.kuna_call(&calling_data, property_method, indent) {
            return buf;
        }

        if let Some(KElement::Property(property)) = &calling_data {
            let property_method =
                property_method.unwrap_or_else(|| npe("KInvocationExprent.propertyMethod"));

            let top_level_prefix = (kuna_fixes::enabled(KunaFix::KtFacadeCall)
                && self.is_static())
            .then(|| kuna_fix_kt_facade_call::property_prefix(self.0, property))
            .flatten();
            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            let super_qualifier = kuna_fixes::enabled(KunaFix::KtSuperPropertyAccess)
                .then(|| kuna_fix_kt_super_property_access::qualifier(self.0))
                .flatten();
            if let Some(super_qualifier) = super_qualifier {
                buf.append(&super_qualifier).append_char('.');
            } else if let Some(instance) = self.get_instance() {
                let mut instance_buf = instance.to_java(indent);
                if instance.get_precedence() > self.0 .0.get_precedence() {
                    instance_buf.enclose("(", ")");
                }
                buf.append_buffer(&instance_buf).append_char('.');
            }
            if let Some(prefix) = top_level_prefix.as_deref() {
                buf.append(prefix);
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
                if kuna_fixes::enabled(KunaFix::KtExtensionCall) {
                    kuna_fix_kt_extension_call::import_infix(self.0, function);
                }
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

        if kuna_fixes::enabled(KunaFix::KtObjectMonitorCall) {
            if let Some(buf) = kuna_fix_kt_object_monitor_call::write(self.0, indent) {
                return buf;
            }
        }

        let kotlin_name = match &calling_data {
            Some(KElement::Function(function))
                if kuna_fixes::enabled(KunaFix::KtJvmNameCall)
                    && function.receiver_type.is_none()
                    && kuna_fix_kt_facade_call::top_level_package(&self.get_classname())
                        .is_none() =>
            {
                kuna_fix_kt_jvm_name_call::kotlin_name(self.0, function)
            }
            _ => None,
        };
        if kotlin_name.is_none() {
            return InvocationExprent::to_java(self.0, indent);
        }
        self.d_mut(|d| d.kuna_kotlin_name = kotlin_name);
        let buf = InvocationExprent::to_java(self.0, indent);
        self.d_mut(|d| d.kuna_kotlin_name = None);
        buf
    }

    /// The call as the fixes `kt_extension_call` and `kt_facade_call` write
    /// it, when they do.
    fn kuna_call(
        self,
        calling_data: &Option<KElement>,
        property_method: Option<PropertyMethod>,
        indent: i32,
    ) -> Option<TextBuffer> {
        let precedence = self.get_precedence();
        match calling_data {
            Some(KElement::Function(function)) => {
                if function.receiver_type.is_some() {
                    return kuna_fixes::enabled(KunaFix::KtExtensionCall)
                        .then(|| {
                            kuna_fix_kt_extension_call::write_function(
                                self.0, function, precedence, indent,
                            )
                        })
                        .flatten();
                }
                (kuna_fixes::enabled(KunaFix::KtFacadeCall) && self.is_static())
                    .then(|| kuna_fix_kt_facade_call::write_function(self.0, function, indent))
                    .flatten()
            }
            Some(KElement::Property(property)) => {
                let method = property_method?;
                if !kuna_fixes::enabled(KunaFix::KtExtensionCall)
                    || !kuna_fix_kt_extension_call::is_extension_accessor(self.0, method)
                {
                    return None;
                }
                kuna_fix_kt_extension_call::write_property(
                    self.0, property, method, precedence, indent,
                )
                .or_else(|| Some(InvocationExprent::to_java(self.0, indent)))
            }
            _ => None,
        }
    }

    pub fn copy(self) -> Exprent {
        KInvocationExprent::new(self.0).0 .0
    }
}
