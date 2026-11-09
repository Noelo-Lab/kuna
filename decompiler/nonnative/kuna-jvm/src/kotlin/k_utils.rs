//! Port of `org/vineflower/kotlin/util/KUtils.java`.

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::jthrow;
use crate::k_annotation_exprent::KAnnotationExprent;
use crate::k_const_exprent::KConstExprent;
use crate::k_exit_exprent::KExitExprent;
use crate::k_field_exprent::KFieldExprent;
use crate::k_function_exprent::KFunctionExprent;
use crate::k_invocation_exprent::KInvocationExprent;
use crate::k_new_exprent::KNewExprent;
use crate::k_switch_head_exprent::KSwitchHeadExprent;
use crate::k_var_exprent::KVarExprent;
use crate::kotlin_options::KotlinOptions;
use crate::kt_proto::Visibility;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

pub struct KUtils;

impl KUtils {
    pub fn replace_exprents(exprs: &[Exprent]) -> Vec<Exprent> {
        let mut res: Vec<Exprent> = Vec::new();

        for &expr in exprs {
            let map = KUtils::replace_exprent(expr);
            res.push(map.unwrap_or(expr));
        }

        res
    }

    /// The Kotlin object for `ex`: `ex` itself when it is one, `null` when its class has none.
    pub fn replace_exprent(ex: Exprent) -> Option<Exprent> {
        if ex.is_kotlin() {
            return Some(ex);
        }

        if ex.is_function() {
            return Some(KFunctionExprent::new(ex.as_function()).into());
        } else if ex.is_var() {
            return Some(KVarExprent::new(ex.as_var()).into());
        } else if ex.is_invocation() {
            return Some(KInvocationExprent::new(ex.as_invocation()).into());
        } else if ex.is_const() {
            return Some(KConstExprent::new(ex.as_const()).into());
        } else if ex.is_field() {
            return Some(KFieldExprent::new(ex.as_field()).into());
        } else if ex.is_annotation() {
            return Some(KAnnotationExprent::new(ex.as_annotation()).into());
        } else if ex.is_switch_head() {
            return Some(KSwitchHeadExprent::new(ex.as_switch_head()).into());
        } else if ex.is_new() {
            return Some(KNewExprent::new(ex.as_new()).into());
        } else if ex.is_exit() {
            return Some(KExitExprent::new(ex.as_exit()).into());
        }

        None
    }

    /// `null` (an unknown enum number) throws like the Java `switch`.
    pub fn append_visibility(buf: &mut TextBuffer, visibility: Option<Visibility>) {
        let Some(visibility) = visibility else {
            jthrow!(NullPointerException; "visibility is null")
        };
        match visibility {
            Visibility::Local => {
                buf.append("// $VF: local visibility outside of methodSupplier")
                    .append_line_separator()
                    .append("internal ");
            }
            Visibility::PrivateToThis => {
                buf.append("private ");
            }
            Visibility::Public => {
                if DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY) {
                    buf.append("public ");
                }
            }
            _ => {
                buf.append(&visibility.name().to_lowercase())
                    .append_char(' ');
            }
        }
    }

    /// Java: removeArguments(InvocationExprent, VarType)
    pub fn remove_arguments(expr: InvocationExprent, to_remove: &VarType) {
        KUtils::remove_arguments_replace(expr, to_remove, None)
    }

    /// Java: removeArguments(InvocationExprent, VarType, VarType)
    pub fn remove_arguments_replace(
        expr: InvocationExprent,
        to_remove: &VarType,
        replace_return_type: Option<&VarType>,
    ) {
        if expr.get_lst_parameters().is_empty() {
            return;
        }

        if expr.get_lst_parameters().len() == 1 {
            let descriptor = expr.get_descriptor();
            let Some(arg_type) = descriptor.params.first() else {
                jthrow!(ArrayIndexOutOfBoundsException; "Index 0 out of bounds for length 0")
            };
            if arg_type.equals(to_remove) {
                expr.get_lst_parameters().clear();
            }
            return;
        }

        let params = expr.get_descriptor().params.clone();
        let lst = expr.get_lst_parameters();
        let mut removed = vec![false; lst.len()];
        for (i, arg_type) in params.iter().enumerate() {
            if arg_type.equals(to_remove) {
                if i >= removed.len() {
                    jthrow!(IndexOutOfBoundsException; "Index {} out of bounds for length {}", i, removed.len());
                }
                removed[i] = true;
            }
        }
        let kept: Vec<Exprent> = lst
            .to_vec()
            .into_iter()
            .zip(removed)
            .filter(|(_, removed)| !removed)
            .map(|(e, _)| e)
            .collect();
        lst.set_all(kept);

        let mut new_desc = expr
            .get_string_descriptor()
            .replace(&to_remove.to_string(), "");

        if new_desc.ends_with(')') {
            let Some(replace_return_type) = replace_return_type else {
                jthrow!(IllegalStateException; "Invalid descriptor: {}", new_desc)
            };

            new_desc.push_str(&replace_return_type.to_string());
        }

        expr.set_string_descriptor(new_desc);
    }

    /// Whether the JVM runs with `-ea`; Vineflower runs without.
    pub fn assertions_enabled() -> bool {
        false
    }
}
