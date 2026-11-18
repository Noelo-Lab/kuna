//! kuna fix `kt_legacy_parameter_checks`: the Kotlin 1.3 parameter null
//! checks that open a function's body are dropped.
//!
//! kotlinc checks each non-null reference parameter of a non-private
//! function (an extension's receiver too) on entry, before any of the
//! function's own code. `ResugarKotlinMethodsPass` drops the check under its
//! Kotlin 1.4 name (`Intrinsics.checkNotNullParameter`), since the
//! parameter's non-null type says it and kotlinc emits it again; Kotlin 1.3
//! and earlier call `Intrinsics.checkParameterIsNotNull`, which it keeps.
//! The kept call is noise in every function, and in an inline function it
//! passes a lambda parameter as a value, which kotlinc rejects ("illegal
//! usage of inline parameter"). Here the checks that open the function's
//! first block are dropped, each of a parameter written with a non-null
//! type (the metadata's type, or, without metadata, the parameter not
//! `@Nullable`), up to the first statement that is not one; constructors are
//! `kt_legacy_constructor_parameter_checks`'. The first block is the one a
//! sequence, an `if`, a `switch` or a `synchronized` starts with, labeled or
//! not; not a loop's body, which may run again.

use std::collections::BTreeMap;

use crate::code_constants::ACC_STATIC;
use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::k_element::KElement;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::pass_context::PassContext;
use crate::statement::{Statement, StatementType};
use crate::struct_method::StructMethod;
use crate::type_family::TypeFamily;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const LEGACY_CHECK: &str = "checkParameterIsNotNull";
const CHECK_DESCRIPTOR: &str = "(Ljava/lang/Object;Ljava/lang/String;)V";
const NULLABLE: &str = "org/jetbrains/annotations/Nullable";

/// Drops the legacy checks that open the body of the function `ctx`
/// decompiles; whether it dropped any.
pub fn remove(ctx: &PassContext) -> bool {
    let method = ctx.get_method();
    if method.get_name().starts_with('<') {
        return false;
    }
    let Some(first) = ctx.get_root().and_then(|root| first_block(root.0)) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    let non_null = non_null_parameters(method);
    let mut count = 0;
    while count < exprents.len()
        && checked_parameter(exprents.get(count))
            .is_some_and(|slot| non_null.get(&slot) == Some(&true))
    {
        count += 1;
    }
    for _ in 0..count {
        exprents.remove_at(0);
    }
    count > 0
}

/// The block `stat` runs first: as `Statements::find_first_data`, also
/// under a statement something breaks out of.
fn first_block(stat: Statement) -> Option<Statement> {
    if stat.get_exprents().is_some() {
        return Some(stat);
    }
    match stat.type_() {
        StatementType::Sequence
        | StatementType::If
        | StatementType::Root
        | StatementType::Switch
        | StatementType::Synchronized => first_block(stat.get_first()?),
        _ => None,
    }
}

/// The slot of `p` in `Intrinsics.checkParameterIsNotNull(p, "..")`.
fn checked_parameter(exprent: Exprent) -> Option<i32> {
    let inv = exprent.try_as_invocation()?;
    if !inv.is_static()
        || inv.get_classname() != INTRINSICS
        || inv.get_name() != LEGACY_CHECK
        || inv.get_string_descriptor() != CHECK_DESCRIPTOR
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    if params.len() != 2
        || !params[1]
            .try_as_const()
            .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::String(_))))
    {
        return None;
    }
    params[0].try_as_var().map(|v| v.get_index())
}

/// Whether each reference parameter of `method` is written with a non-null
/// type, by slot.
fn non_null_parameters(method: StructMethod) -> BTreeMap<i32, bool> {
    let md = method.method_descriptor();
    let mut res = BTreeMap::new();
    let mut slot = if method.has_modifier(ACC_STATIC) { 0 } else { 1 };
    let written: Vec<Option<bool>> = match KtAttrs::get(method, &KElement::KEY()) {
        Some(KElement::Function(f)) => {
            let kotlin: Vec<Option<bool>> = f
                .receiver_type
                .iter()
                .chain(f.parameters.iter().map(|p| &p.type_))
                .map(|t| Some(!t.k().is_nullable))
                .collect();
            if kotlin.len() > md.params.len() {
                return res;
            }
            kotlin
        }
        _ => (0..md.params.len()).map(|i| Some(!annotated_nullable(method, i))).collect(),
    };
    for (i, param) in md.params.iter().enumerate() {
        if let Some(Some(non_null)) = written.get(i) {
            res.insert(slot, *non_null && param.type_family == TypeFamily::Object);
        }
        slot += param.stack_size;
    }
    res
}

/// Whether the parameter `param` of `method` is annotated `@Nullable`.
fn annotated_nullable(method: StructMethod, param: usize) -> bool {
    KotlinWriter::PARAMETER_ANNOTATION_ATTRIBUTES.iter().any(|key| {
        method.get_attribute(*key).is_some_and(|attribute| {
            attribute
                .get_param_annotations()
                .get(param)
                .is_some_and(|annotations| annotations.iter().any(|a| a.get_class_name() == NULLABLE))
        })
    })
}
