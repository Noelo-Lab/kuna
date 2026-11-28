//! Port of `org/vineflower/kotlin/expr/KVarExprent.java`.

use crate::exprent::Exprent;
use crate::k_dispatch::KExprentExt;
use crate::k_types::KTypes;
use crate::kotlin_exprent;
use crate::fixes::kt_default_args;
use crate::fixes::kt_extensions;
use crate::fixes::kt_names::kt_function_parameter_name;
use crate::fixes::kt_null_checks::kt_nullable_local;
use crate::fixes::kt_types::kt_raw_generic_type;
use crate::fixes::kt_properties;
use crate::fixes::kt_suspend::kt_suspend_lambda;
use crate::fixes::kt_suspend::kt_suspend_impl;
use crate::fixes::kt_anonymous_objects::kt_unrepresentable_declared_type;
use crate::kotlin_writer::KotlinWriter;
use crate::rt::BitSet;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;

/// `KVarExprent.DeclarationType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum DeclarationType {
    Definition,
    Usage,
    ForLoopVariable,
    ExceptionType,
}

impl DeclarationType {
}

pub struct KVarExprentData {
    pub declaration_type: Option<DeclarationType>,
    /// Fix `kt_nullable_local`: the local may hold null.
    pub nullable: bool,
}

kotlin_exprent!(pub KVarExprent: VarExprent, Var(KVarExprentData),
                is_k_var, as_k_var, try_as_k_var);

impl KVarExprent {
    /// Java: KVarExprent(VarExprent)
    pub fn new(ex: VarExprent) -> KVarExprent {
        let bytecode = ex.get_bytecode();
        let var = KVarExprent::new_index(
            ex.get_index(),
            Some(ex.get_var_type()),
            ex.get_processor(),
            bytecode.as_ref(),
        );
        var.set_stack(ex.is_stack());
        var.set_class_def(ex.is_class_def());
        var.set_version(ex.get_version());
        var.set_effectively_final(ex.is_effectively_final());
        var.set_definition(ex.is_definition());
        let declaration_type = match ex.0.try_as_k_var() {
            Some(k_var_exprent) => k_var_exprent.kd(|d| d.declaration_type),
            None => Some(if ex.is_definition() {
                DeclarationType::Definition
            } else {
                DeclarationType::Usage
            }),
        };
        var.set_declaration_type(declaration_type);
        if let Some(k) = ex.0.try_as_k_var() {
            let nullable = k.kd(|d| d.nullable);
            var.kd_mut(|d| d.nullable = nullable);
        }
        var
    }

    /// Java: KVarExprent(int, VarType, VarProcessor, BitSet)
    pub fn new_index(
        index: i32,
        var_type: Option<VarType>,
        processor: Option<VarProcessor>,
        bytecode: Option<&BitSet>,
    ) -> KVarExprent {
        let var = VarExprent::new_bytecode(index, var_type, processor, bytecode);
        var.0.set_kt(Some(KExprentExt::Var(KVarExprentData {
            declaration_type: None,
            nullable: false,
        })));
        KVarExprent(var)
    }

    pub fn get_name(self) -> String {
        if let Some(name) = kt_extensions::name(self.0) {
            return name;
        }
        if let Some(name) = kt_properties::name_in_setter(self.0) {
            return name;
        }
        if let Some(name) = kt_default_args::name_in_defaults(self.0) {
            return name;
        }
        if let Some(name) = kt_suspend_impl::name(self.0) {
            return name;
        }
        if let Some(name) = kt_function_parameter_name::name(self.0) {
            return name;
        }
        let name = VarExprent::get_name_base(self.0);

        if let Some(reference) = kt_suspend_lambda::receiver_reference(self.0) {
            return reference;
        }

        if name.starts_with("this@") || name == "this" {
            return name;
        }

        if let Some(class) = name.strip_suffix(".this").filter(|c| !c.is_empty()) {
            return KVarExprent::outer_this_label(class);
        }

        KotlinWriter::to_valid_kotlin_identifier(&name)
    }

    /// Fix `kt_outer_this`: `this@Class`, the instance of the enclosing class `class`.
    pub fn outer_this_label(class: &str) -> String {
        format!("this@{}", KotlinWriter::to_valid_kotlin_identifier(class))
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        let mut buffer = TextBuffer::new();

        buffer.add_bytecode_mapping(self.get_bytecode().as_ref());

        let declaration_type = self.kd(|d| d.declaration_type);
        if declaration_type == Some(DeclarationType::Definition) {
            let processor = self.get_processor();

            let is_final = self.is_effectively_final()
                || processor.is_some_and(|processor| {
                    processor.get_var_final(self.get_var_version_pair()) != FinalType::NonFinal
                });

            buffer.append(if is_final { "val " } else { "var " });
        }

        buffer.append(&self.0.get_name());

        if declaration_type == Some(DeclarationType::Definition)
            || declaration_type == Some(DeclarationType::ExceptionType)
        {
            let written = match self.0.kuna_declared() {
                Some(typed) => kt_raw_generic_type::declared_type(&typed),
                None => KTypes::get_kotlin_type(Some(&self.get_definition_var_type())),
            };
            if !(declaration_type == Some(DeclarationType::Definition)
                && kt_unrepresentable_declared_type::omits(&written))
            {
                buffer.append(": ");
                if declaration_type == Some(DeclarationType::Definition) && self.kd(|d| d.nullable)
                {
                    buffer.append(&kt_nullable_local::nullable(&written));
                } else {
                    buffer.append(&written);
                }
            }
        }

        buffer
    }

    pub fn set_declaration_type(self, declaration_type: Option<DeclarationType>) {
        self.kd_mut(|d| d.declaration_type = declaration_type)
    }

    pub fn copy(self) -> Exprent {
        KVarExprent::new(VarExprent::copy(self.0).as_var()).0 .0
    }
}
