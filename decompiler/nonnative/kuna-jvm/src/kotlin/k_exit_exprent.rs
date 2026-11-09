//! Port of `org/vineflower/kotlin/expr/KExitExprent.java`.
//!
//! kuna fix `jvm_fix_kt_synthetic_anonymous_object` (`kuna_fixes`): in a
//! method of an object that fix writes, the current method wrapper is the
//! method itself, not a lambda's, and a `return` is printed.

use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::exprent::Exprent;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_type::KType;
use crate::kotlin_exprent;
use crate::kuna_fix_kt_synthetic_anonymous_object;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

pub struct KExitExprentData {
    pub lambda_name: Option<String>,
}

kotlin_exprent!(pub KExitExprent: ExitExprent, Exit(KExitExprentData),
                is_k_exit, as_k_exit, try_as_k_exit);

impl KExitExprent {
    /// Java: KExitExprent(ExitExprent)
    pub fn new(expr: ExitExprent) -> KExitExprent {
        let bytecode = expr.get_bytecode();
        let exit = ExitExprent::new(
            expr.get_exit_type(),
            expr.get_value(),
            expr.get_ret_type(),
            bytecode.as_ref(),
            expr.get_method_descriptor(),
        );
        exit.0.set_kt(Some(KExprentExt::Exit(KExitExprentData {
            lambda_name: None,
        })));
        KExitExprent(exit)
    }

    pub fn set_lambda_name(self, name: Option<String>) {
        self.kd_mut(|d| d.lambda_name = name)
    }

    /// A value-less `return` outside a named lambda throws `NullPointerException`, as in Vineflower.
    pub fn to_java(self, indent: i32) -> TextBuffer {
        if self.get_exit_type() == ExitType::Throw {
            return ExitExprent::to_java(self.0, indent);
        }

        let outer_method = DecompilerContext::get_current_method_wrapper().filter(|m| {
            !(kuna_fixes::enabled(KunaFix::KtSyntheticAnonymousObject)
                && kuna_fix_kt_synthetic_anonymous_object::is_declaration(*m))
        });
        if outer_method.is_some() {
            let mut buf = TextBuffer::new();

            if let Some(lambda_name) = self.kd(|d| d.lambda_name.clone()) {
                buf.append("return@").append(&lambda_name);
                let ret_type = self.get_ret_type();
                if VarType::VARTYPE_VOID().equals_opt(ret_type.as_ref())
                    || KType::UNIT().equals_opt(ret_type.as_ref())
                {
                    return buf;
                }
                buf.append(" ");
            }

            let value = self.get_value().unwrap_or_else(
                || jthrow!(NullPointerException; "KExitExprent.getValue() is null"),
            );
            buf.append_buffer(&value.to_java(indent));
            return buf;
        }

        ExitExprent::to_java(self.0, indent)
    }

    pub fn copy(self) -> Exprent {
        KExitExprent::new(ExitExprent::copy(self.0).as_exit()).0 .0
    }
}
