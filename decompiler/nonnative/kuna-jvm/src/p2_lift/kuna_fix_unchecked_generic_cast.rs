//! kuna fix `unchecked_generic_cast` (`kuna_fixes`): a cast between two
//! parameterizations of one class, `(ReferenceEntry<K, V>) NullEntry.INSTANCE`
//! or `(CacheBuilder<K1, V1>) this`, is unchecked and compiles to no
//! instruction, so the bytecode keeps no trace of it. Vineflower writes a cast
//! where the erased types call for one or where a few generic shapes do
//! (`doGenericTypesCast`), and otherwise none, so a returned or assigned value
//! whose declared generic type is not assignable to its target is written
//! bare, which javac rejects (`incompatible types`). With the fix, when the
//! value is `this`, a field or a variable, whose static type the program
//! declares, and the target is a declared parameterization (a method's return
//! type, a variable's or a field's type), a target whose type arguments the
//! value's type cannot match gets the cast: to the target when the two are not
//! provably distinct (JLS 4.5), else to its raw class, which javac converts
//! unchecked. Kotlin output is left alone.

use crate::classes_processor::ClassNodeType;
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::kuna_fix_unchecked_call_result_cast;
use crate::kuna_fixes::{self, KunaFix};
use crate::kuna_static_types::{arg_equal, field_declared, param_type, supertype_args};
use crate::var_type::{instance_of, VarType};

/// The cast `exprent` needs to be assigned to `left`, when its declared type
/// is a parameterization `left`'s type arguments do not accept.
pub fn cast_for(exprent: Exprent, left: &VarType) -> Option<VarType> {
    if exprent.is_kotlin() {
        return None;
    }
    let target = left.try_as_generic()?;
    if left.type_ != CodeType::Object
        || left.array_dim != 0
        || target.get_parent().is_some()
        || target.get_arguments().is_empty()
    {
        return None;
    }
    let right = declared_type(exprent)?;
    let call = exprent.try_as_invocation().is_some();
    if right.type_ != CodeType::Object
        || right.array_dim != 0
        || !instance_of(&right.value, &left.value)
    {
        return None;
    }
    let view = supertype_args(&right, left.value_str())?;
    let wanted = target.get_arguments();
    if view.len() != wanted.len() {
        return None;
    }
    let mut mismatch = false;
    let mut distinct = false;
    for (w, v) in wanted.iter().zip(&view) {
        let Some(w) = w.as_ref().filter(|w| is_exact(w)) else {
            continue;
        };
        if arg_equal(&Some(w.clone()), v) {
            continue;
        }
        mismatch = true;
        if v.as_ref().is_some_and(|v| {
            is_exact(v) && (v.type_ != CodeType::Genvar || call) && w.type_ != CodeType::Genvar
        }) {
            distinct = true;
        }
    }
    if !mismatch {
        return None;
    }
    Some(if distinct {
        VarType::new_value(CodeType::Object, 0, left.value_str())
    } else {
        left.clone()
    })
}

/// The type the program declares for `exprent`: `this` (not of an inner
/// class), a static field or a field of `this` of a known class, or a
/// variable with a local variable table entry or a parameter's declared type;
/// with the fix `unchecked_call_result_cast`, also a call's declared result
/// (`kuna_fix_unchecked_call_result_cast`).
fn declared_type(exprent: Exprent) -> Option<VarType> {
    if let Some(inv) = exprent.try_as_invocation() {
        return kuna_fixes::enabled(KunaFix::UncheckedCallResultCast)
            .then(|| kuna_fix_unchecked_call_result_cast::declared_result(inv, declared_type))
            .flatten();
    }
    if let Some(var) = exprent.try_as_var() {
        let processor = var.get_processor()?;
        let pair = var.get_var_version_pair();
        if let Some(class) = processor.get_this_vars().borrow().get_cloned(&pair) {
            return (!is_inner_class(&class)).then(|| var.get_var_type());
        }
        if processor.get_external_vars().borrow().contains(&pair)
            || processor.get_method().get_name().starts_with("lambda$")
        {
            return None;
        }
        if let Some(param) = param_type(var) {
            return param;
        }
        return var.get_lvt().map(|_| var.get_inferred_expr_type(None));
    }
    let field = exprent.try_as_field()?;
    let instance_declared = field.get_instance().is_none_or(|i| {
        i.try_as_var()
            .and_then(|v| v.get_processor().map(|p| (v, p)))
            .is_some_and(|(v, p)| {
                p.get_this_vars()
                    .borrow()
                    .contains_key(&v.get_var_version_pair())
            })
    });
    if field.get_name().starts_with("this$") || !instance_declared {
        return None;
    }
    field_declared(field).then(|| field.get_inferred_expr_type(None))
}

/// A non-static class nested in another, whose `this` takes type arguments
/// from the enclosing instance that its own type does not show.
fn is_inner_class(class: &str) -> bool {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&class.to_string())
        .copied()
        .is_some_and(|node| {
            node.with(|d| d.type_) != ClassNodeType::Root
                && node.with(|d| d.access) & CodeConstants::ACC_STATIC == 0
        })
}

/// A type argument that is a type, not a wildcard.
fn is_exact(t: &VarType) -> bool {
    t.try_as_generic()
        .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}
