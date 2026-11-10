//! kuna fix `kt_field_read_reuse` (spec 10.16): in Kotlin output, a field
//! value the bytecode reads once and uses several times is not read again at
//! each use.
//!
//! `StackVarsProcessor` copies a variable's value into its uses when the
//! value may be used several times (`MULTIPLE_USES`), as a field read on a
//! variable may. kotlinc reads a field once and `dup`s it for an elvis or a
//! safe call (`val h = head ?: throw ..`, `head?.let { h -> .. }`): the
//! null check and the store each use the value. Vineflower put the read into
//! both, `if (this.head == null) throw ..; val h = this.head`, and the Kotlin
//! plugin, which inlines every local (`INLINE_ALL_VARS`), also copies the
//! read a named local holds into each of its uses (`val s = src.head;
//! s!!.limit - s.pos` came out `src.head!!.limit - src.head.pos`). Each read
//! may see another value once another thread writes the field (`@Volatile`
//! fields of kotlinx.coroutines), and the check no longer guards the value
//! used, so kotlinc rejects the use of the nullable property. With the fix
//! such a value of a field that is not final stays in its variable (a final
//! field, a `val` property's, keeps its value once set, and Kotlin smart-casts
//! a `val` property). As for fix `field_read_reuse`, a
//! read followed in its block by a store into the same field (Kotlin 1.3's
//! `x++`) keeps the copies, which `PPandMMHelper` folds into one read.

use std::cell::Cell;

use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::kotlin_options::KotlinOptions;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_fixes::{self, KunaFix};
use crate::p3_dataflow::kuna_fix_field_read_reuse;
use crate::rt::JList;
use crate::struct_class::StructClass;
use crate::var_version_node::VarVersionNode;

/// Whether the field read `value`, assigned to a variable with the uses
/// `used`, stays in that variable in a Kotlin class.
pub fn keeps_read(
    value: Exprent,
    used: &[VarVersionNode],
    exprents: &JList<Exprent>,
    index: usize,
) -> bool {
    value.is_field()
        && kuna_fixes::enabled(KunaFix::KtFieldReadReuse)
        && is_kotlin_class()
        && kuna_fix_field_read_reuse::keeps_single_read(value, used, exprents, index)
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

thread_local! {
    static KOTLIN: Cell<bool> = const { Cell::new(false) };
}

/// While alive, `StackVarsProcessor` simplifies a method of the class it was entered with.
pub struct KotlinScope(bool);

impl Drop for KotlinScope {
    fn drop(&mut self) {
        KOTLIN.with(|k| k.set(self.0));
    }
}

/// Enters `StackVarsProcessor`'s run over a method of `cl`.
pub fn enter(cl: StructClass) -> KotlinScope {
    let kotlin = (kuna_fixes::enabled(KunaFix::KtFieldReadReuse)
        || kuna_fixes::enabled(KunaFix::KtPartialInline))
        && DecompilerContext::get_option(KotlinOptions::DECOMPILE_KOTLIN)
        && KotlinMetadataAnnotation::find(cl).is_some();
    KotlinScope(KOTLIN.with(|k| k.replace(kotlin)))
}

/// Whether the method being simplified is written as Kotlin: its class is a
/// Kotlin class (it has metadata) and the Kotlin plugin is on.
pub fn is_kotlin_class() -> bool {
    KOTLIN.with(|k| k.get())
}
