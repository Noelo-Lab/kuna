//! kuna fix `kt_inline_heap_read` (spec 10.16): the Kotlin inliner does not
//! copy a value that reads an array element, or a field that may change,
//! into several uses.
//!
//! The Kotlin plugin's last `SimplifyStack` (`INLINE_ALL_VARS`) copies a
//! variable's value into every use its SSAU checks pass and deletes the
//! variable: an array element read passes as a field read does,
//! `val e = slots[i]; if (e != null) e.value` became
//! `if (this.slots[i] != null) this.slots[i].value`, and so did arithmetic of
//! field reads (`val r = limit - count`) and a final field of an object a
//! changing field holds (`val n = this.head.next`). Another thread may store
//! between the reads, so the test passes on one value and the use reads
//! another (`NullPointerException`, a value no single read gave); on a
//! `@Volatile` field each read is a separate synchronizing action.
//!
//! With the fix such a value whose every use the inliner would copy, more
//! than one, stays in its variable: the bytecode reads it once and so does
//! the output. Not here: a read of a non-final field alone, which fix
//! `kt_field_read_reuse` keeps in its variable in every pass of a Kotlin
//! class, and a variable the inliner keeps for some uses, which fix
//! `kt_partial_inline` reads at all of them. A single use still moves the
//! read; a final field of an unchanging value (a constant, a local, `this`)
//! is still copied; and so are the two uses of kotlinc's `x as? T` (`dup`,
//! `instanceof`, `checkcast`), which the Kotlin writer prints as the one read
//! `x as? T`.

use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::p3_dataflow::kuna_fix_field_read_reuse;
use crate::rt::JList;
use crate::var_type::VarType;
use crate::var_version_node::VarVersionNode;

pub struct KtInlineHeapRead;

impl KtInlineHeapRead {
    /// Whether the value `value` of a variable with the uses `used`, all of
    /// which the inliner would copy, must stay in the variable.
    pub fn copies_heap_read(
        value: Exprent,
        used: &[VarVersionNode],
        exprents: &JList<Exprent>,
        index: usize,
        next: Option<Exprent>,
    ) -> bool {
        Self::rereads_heap(value)
            && !Self::kept_field_read(value, used, exprents, index)
            && !Self::safe_cast_operands(used, next)
    }

    /// Whether `value` is a read of a field not declared final in the class
    /// the reference names, with uses that fix `kt_field_read_reuse` keeps
    /// in its variable (no later store into the same field in the block).
    fn kept_field_read(
        value: Exprent,
        used: &[VarVersionNode],
        exprents: &JList<Exprent>,
        index: usize,
    ) -> bool {
        let Some(field) = value.try_as_field() else {
            return false;
        };
        let declared_final = DecompilerContext::get_struct_context()
            .get_class(&field.get_classname())
            .and_then(|cl| {
                cl.get_field(&field.get_name(), &field.get_descriptor().descriptor_string)
            })
            .is_some_and(|fd| fd.has_modifier(ACC_FINAL));
        !declared_final
            && kuna_fix_field_read_reuse::keeps_single_read(value, used, exprents, index)
    }

    /// Whether evaluating `value` again may give another value: it reads an
    /// array element or a field not known to be final.
    pub fn rereads_heap(value: Exprent) -> bool {
        value
            .get_all_exprents_rec_self(true, true)
            .into_iter()
            .any(|e| {
                e.try_as_array().is_some() || e.try_as_field().is_some_and(|f| !Self::is_final(f))
            })
    }

    /// Whether the uses `used` are the operands of one `v is T ? (T) v : null`
    /// in `next`, which the Kotlin writer prints `v as? T`.
    pub fn safe_cast_operands(used: &[VarVersionNode], next: Option<Exprent>) -> bool {
        let ([first, second], Some(next)) = (used, next) else {
            return false;
        };
        let var_of = |e: Exprent, func_type: FunctionType| {
            e.try_as_function()
                .filter(|f| f.get_func_type() == func_type)
                .and_then(|f| f.get_lst_operands().get(0).try_as_var())
                .map(|v| v.get_var_version_pair())
        };
        next.get_all_exprents_rec_self(true, true)
            .into_iter()
            .filter_map(|e| e.try_as_function())
            .filter(|f| f.get_func_type() == FunctionType::Ternary)
            .any(|f| {
                let operands = f.get_lst_operands();
                let tested = var_of(operands.get(0), FunctionType::Instanceof);
                let cast = var_of(operands.get(1), FunctionType::Cast);
                let null = operands
                    .get(2)
                    .get_expr_type_opt()
                    .is_some_and(|t| t.ptr_eq(&VarType::VARTYPE_NULL()));
                null && matches!((tested, cast), (Some(t), Some(c))
                    if (t, c) == (first.as_pair(), second.as_pair())
                        || (c, t) == (first.as_pair(), second.as_pair()))
            })
    }

    /// Whether `field` is declared final in its class or a superclass the
    /// decompiler knows.
    fn is_final(field: FieldExprent) -> bool {
        let context = DecompilerContext::get_struct_context();
        let name = field.get_name();
        let descriptor = field.get_descriptor().descriptor_string.clone();
        let mut owner = Some(field.get_classname());
        let mut depth = 0;
        while let Some(class_name) = owner.take() {
            let Some(cl) = context.get_class(&class_name) else {
                return false;
            };
            if let Some(fd) = cl.get_field(&name, &descriptor) {
                return fd.has_modifier(ACC_FINAL);
            }
            depth += 1;
            if depth > 64 {
                return false;
            }
            owner = cl.super_class().map(|s| s.get_string().to_string());
        }
        false
    }
}
