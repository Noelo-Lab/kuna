//! Port of `org/vineflower/kotlin/expr/KVarExprent.java`.
//!
//! kuna fix `kt_outer_this`: the variable that holds an enclosing class's
//! instance, named `Outer.this` (Java's qualified `this`), is written
//! `this@Outer`, Kotlin's; `Outer.this` is not a Kotlin expression.
//!
//! kuna fix `kt_extension_receiver`: an extension function's receiver
//! parameter is written `this` in its body
//! (`kuna_fix_kt_extension_receiver`).

use crate::exprent::Exprent;
use crate::k_dispatch::KExprentExt;
use crate::k_types::KTypes;
use crate::kotlin_exprent;
use crate::kotlin::kuna_fix_kt_extension_receiver;
use crate::kotlin_writer::KotlinWriter;
use crate::kuna_fixes::{self, KunaFix};
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
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            DeclarationType::Definition => "DEFINITION",
            DeclarationType::Usage => "USAGE",
            DeclarationType::ForLoopVariable => "FOR_LOOP_VARIABLE",
            DeclarationType::ExceptionType => "EXCEPTION_TYPE",
        }
    }
}

pub struct KVarExprentData {
    pub declaration_type: Option<DeclarationType>,
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
        })));
        KVarExprent(var)
    }

    pub fn get_name(self) -> String {
        if kuna_fixes::enabled(KunaFix::KtExtensionReceiver) {
            if let Some(name) = kuna_fix_kt_extension_receiver::name(self.0) {
                return name;
            }
        }
        let name = VarExprent::get_name_base(self.0);

        if name.starts_with("this@") || name == "this" {
            return name;
        }

        if kuna_fixes::enabled(KunaFix::KtOuterThis) {
            if let Some(class) = name.strip_suffix(".this").filter(|c| !c.is_empty()) {
                return KVarExprent::outer_this_label(class);
            }
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
            buffer.append(": ");
            buffer.append(&KTypes::get_kotlin_type(Some(
                &self.get_definition_var_type(),
            )));
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
