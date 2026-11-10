//! kuna fix `kt_inline_heap_read` (spec 10.16): the Kotlin inliner does not
//! copy a read of a non-final field or of an array element into several
//! uses.
//!
//! The Kotlin plugin's last `SimplifyStack` (`INLINE_ALL_VARS`) copies a
//! variable's value into every use its SSAU checks pass: a field read passes
//! while no call or store to a field lies between, so
//! `val s = this.state; if (s is Node) return s.value` became
//! `if (this.state is Node) return (this.state as Node).value`, and
//! `stream?.close()` (a `dup` of one read) `if (this.stream != null)
//! this.stream.close()`. One thread sees the same value twice; another
//! thread may store between the reads, so the test passes on one object and
//! the body uses another (`ClassCastException`, `NullPointerException`, a
//! value no single read gave). On a `@Volatile` field each read is a
//! separate synchronizing action; on a plain field the Java memory model
//! does not even promise that a second read sees the first one's value or a
//! later one. Array elements are the same kind of shared state.
//!
//! With the fix such a value stays in its variable when it has more than one
//! use (or a use the definition does not dominate, where the definition is
//! kept anyway): the bytecode reads the location once and so does the
//! output. A single use still moves the read; a final field of a value that
//! does not change (a constant, a local, `this`) is still copied, as it
//! reads the same value wherever it is read again; and so are the two uses
//! of kotlinc's `x as? T` (`dup`, `instanceof`, `checkcast`), which the
//! Kotlin writer prints as the one read `x as? T`.

use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::var_type::VarType;
use crate::var_version_node::VarVersionNode;

pub struct KtInlineHeapRead;

impl KtInlineHeapRead {
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
