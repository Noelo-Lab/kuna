//! Fix family `stack_var_inlining`: which values the stack-variable inliner
//! copies into their uses, and when. In this file, `field_read_reuse` keeps a
//! field read javac `dup`s for two uses in its stack variable;
//! `kt_field_read_reuse` keeps a read of a non-final field in its variable in
//! a Kotlin class; `kt_partial_inline` reads a variable the Kotlin inliner
//! keeps at every one of its uses.

pub(crate) mod field_read_all_paths;
pub(crate) mod kt_inline_evaluation_order;
pub(crate) mod kt_inline_heap_read;
pub(crate) mod kt_inline_lock_local;

use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::kotlin_options::KotlinOptions;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::rt::JList;
use crate::struct_class::StructClass;
use crate::var_exprent::VarExprent;
use crate::var_version_node::VarVersionNode;

/// Whether the field read `value`, assigned to a stack variable with the uses
/// `used`, must stay in that variable: it has several uses and no statement
/// after `index` stores into a field of the same name and class (a
/// read-modify-write, whose copies `PPandMMHelper` folds into one `x++`; its
/// receiver may still be a stack variable here, so receivers are not compared).
pub fn keeps_single_read(
    value: Exprent,
    used: &[VarVersionNode],
    exprents: &JList<Exprent>,
    index: usize,
) -> bool {
    used.len() > 1
        && !exprents.to_vec().into_iter().skip(index + 1).any(|e| {
            e.try_as_assignment()
                .is_some_and(|store| same_field(store.get_left(), value))
        })
}

/// Whether a statement after `index` stores into a field of the object the
/// stack variable `left` holds, directly or through a stack variable that
/// copies it (a read-modify-write `x.f.g++`, `dup` before `getfield`, whose
/// copies `PPandMMHelper` folds into one read of `x.f`).
pub fn stores_through(left: VarExprent, exprents: &JList<Exprent>, index: usize) -> bool {
    let mut holders = vec![left.get_index()];
    exprents.to_vec().into_iter().skip(index + 1).any(|e| {
        let Some(assignment) = e.try_as_assignment() else {
            return false;
        };
        let target = assignment.get_left();
        if let (Some(copy), Some(source)) =
            (target.try_as_var(), assignment.get_right().try_as_var())
        {
            if copy.is_stack() && holders.contains(&source.get_index()) {
                holders.push(copy.get_index());
            }
            return false;
        }
        target
            .try_as_field()
            .and_then(|f| f.get_instance())
            .and_then(|i| i.try_as_var())
            .is_some_and(|v| holders.contains(&v.get_index()))
    })
}

fn same_field(a: Exprent, b: Exprent) -> bool {
    let (Some(a), Some(b)) = (a.try_as_field(), b.try_as_field()) else {
        return false;
    };
    a.get_name() == b.get_name()
        && a.get_classname() == b.get_classname()
        && a.is_static() == b.is_static()
}

/// Whether the class being decompiled is written as Kotlin: the Kotlin plugin
/// is on and the class has Kotlin metadata.
pub fn written_as_kotlin() -> bool {
    DecompilerContext::get_option(KotlinOptions::DECOMPILE_KOTLIN)
        && DecompilerContext::get_current_class()
            .is_some_and(|cl| KotlinMetadataAnnotation::find(cl).is_some())
}

/// Whether the field read `value`, assigned to a variable with the uses
/// `used`, stays in that variable in a Kotlin class (`kotlin`, [`is_kotlin_class`]).
pub fn keeps_read(
    value: Exprent,
    used: &[VarVersionNode],
    exprents: &JList<Exprent>,
    index: usize,
    kotlin: bool,
) -> bool {
    value.is_field()
        && kotlin
        && keeps_single_read(value, used, exprents, index)
        && !is_final(value)
}

/// Whether the field `value` reads is final (a `val` property's), which keeps its value once set.
fn is_final(value: Exprent) -> bool {
    let field = value.as_field();
    DecompilerContext::get_struct_context()
        .get_class(&field.get_classname())
        .and_then(|cl| cl.get_field(&field.get_name(), &field.get_descriptor().descriptor_string))
        .is_some_and(|fd| fd.has_modifier(ACC_FINAL))
}

/// Whether `StackVarsProcessor` simplifies a method of `cl` as Kotlin: a fix
/// that asks is on, the Kotlin plugin is on and `cl` has Kotlin metadata.
pub fn is_kotlin_class(cl: StructClass) -> bool {
    DecompilerContext::get_option(KotlinOptions::DECOMPILE_KOTLIN)
        && KotlinMetadataAnnotation::find(cl).is_some()
}

/// Whether a variable that stays keeps all of its uses, in a method of a class
/// written as Kotlin (`kotlin`).
pub fn keeps_variable(kotlin: bool) -> bool {
    kotlin
}
