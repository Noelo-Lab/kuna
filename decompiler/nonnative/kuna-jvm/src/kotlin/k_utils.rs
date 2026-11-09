//! Port of `org/vineflower/kotlin/util/KUtils.java`.

#![allow(dead_code, unused_variables)]

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::kt_proto::Visibility;
use crate::text_buffer::TextBuffer;
use crate::unported;
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
        unported!("KUtils.replaceExprent")
    }

    /// `null` (an unknown enum number) throws like the Java `switch`.
    pub fn append_visibility(buf: &mut TextBuffer, visibility: Option<Visibility>) {
        unported!("KUtils.appendVisibility")
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
        unported!("KUtils.removeArguments")
    }

    /// Whether the JVM runs with `-ea`; Vineflower runs without.
    pub fn assertions_enabled() -> bool {
        false
    }
}
