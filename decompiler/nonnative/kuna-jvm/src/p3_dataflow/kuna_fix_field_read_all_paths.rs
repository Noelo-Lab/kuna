//! kuna fix `field_read_all_paths` (spec 10.16): the Kotlin inliner moves a
//! field read only to a use that no path from the read to it crosses
//! something that may change the field.
//!
//! `SSAUConstructorSparseEx` tracks each field read as a variable (a negative
//! index) that is live from the read until a call, an object creation, a field
//! store or an exception edge removes it (`removeAllFields`), and
//! `StackVarsProcessor` moves a local's field value to a later use only while
//! that variable is live there. Liveness has to hold on every path, but the
//! maps are merged by union, which keeps a field live after a join where one
//! predecessor removed it: past an `if` whose branch calls or a condition
//! whose second operand calls (`VarMapHolder`'s merges), into a catch or
//! finally body (the try node's own edge to its combined catch carries the
//! state from before the protected code), and past a finally body that calls
//! (the normal exit never sees the body). The Kotlin plugin inlines every
//! local (`INLINE_ALL_VARS`), so `val v = h.count; try { work() } finally {
//! use(v) }` printed `use(h.count)`, read after `work()`.
//!
//! With the fix, the SSAU forms of that pass keep a field variable at a merge
//! only if every predecessor whose state is known has it (a `VarMapHolder`
//! merge, only if both states have it), and drop every field variable on the
//! normal exit of a try-finally whose body may change fields; a value without
//! effects that reads a field whose variable is not live where it is
//! assigned (a read on one branch of a conditional) is inlined only into the
//! next statement, ahead of anything there that may change the heap.
//! The passes that inline only stack variables keep Vineflower's merge: a
//! stack value live across a join was pushed before a conditional
//! expression's operands (or is the `x!!` of Kotlin 1.3, whose other branch
//! never returns), and once the expression is rebuilt it is read before the
//! conditional part.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::p3_dataflow::kuna_fix_kt_inline_evaluation_order::{
    KtInlineEvaluationOrder, Movability,
};
use crate::rt::Shared;
use crate::s_forms_fast_map_direct::SFormsFastMapDirect;
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::statement::Statement;
use crate::var_version_pair::VarVersionPair;

pub struct FieldReadAllPaths;

impl FieldReadAllPaths {
    /// Whether, in an SSAU form built with `field_reads_all_paths`, `value`
    /// (assigned to `left`), which has no effect, reads a field whose variable
    /// is not live where `left` is assigned: a read on one branch of a
    /// conditional, which the merge after it drops. The liveness check at a
    /// use cannot see such a read, so the value is inlined only into the next
    /// statement, ahead of anything there that may change the heap. (A value
    /// with effects drops the variables it read itself; fix
    /// `kt_inline_evaluation_order` keeps it where nothing comes between.)
    pub fn untracked_read(
        left: VarVersionPair,
        value: Exprent,
        ssau: &SSAUConstructorSparseEx,
    ) -> bool {
        if !ssau.field_reads_all_paths
            || KtInlineEvaluationOrder::movability(value) == Movability::Effects
        {
            return false;
        }
        let Some(live) = ssau.get_live_var_versions_map(left) else {
            return false;
        };
        let fields = ssau.get_map_field_vars();
        value
            .get_all_exprents_rec_self(true, true)
            .into_iter()
            .filter(|e| e.is_field())
            .filter_map(|e| fields.get_cloned(&e.get_id()))
            .any(|index| !live.borrow().contains_key(index))
    }

    /// The field variables of `map`.
    pub fn fields(map: &Shared<SFormsFastMapDirect>) -> Vec<i32> {
        map.borrow()
            .entry_list()
            .into_iter()
            .map(|(key, _)| key)
            .filter(|key| *key < 0)
            .collect()
    }

    /// Removes from `merged` every field variable that one of `preds` (the
    /// predecessors' states) lacks.
    pub fn keep_common_fields(
        merged: &Shared<SFormsFastMapDirect>,
        preds: &[Shared<SFormsFastMapDirect>],
    ) {
        for key in Self::fields(merged) {
            if preds.iter().any(|pred| !pred.borrow().contains_key(key)) {
                merged.borrow_mut().remove(key);
            }
        }
    }

    /// Whether `body` holds an exprent after which the SSAU form drops every
    /// field variable: a call, an object creation, a field store.
    pub fn changes_fields(body: Statement) -> bool {
        let exprents = match body.get_exprents() {
            Some(list) => list.to_vec(),
            None => body.get_stat_exprents(),
        };
        if exprents
            .into_iter()
            .flat_map(|e| e.get_all_exprents_rec_self(true, true))
            .any(Self::drops_fields)
        {
            return true;
        }
        let stats = body.get_stats().borrow().to_vec();
        stats.into_iter().any(Self::changes_fields)
    }

    fn drops_fields(expr: Exprent) -> bool {
        if expr.is_invocation() {
            return true;
        }
        if let Some(new) = expr.try_as_new() {
            return new.get_new_type().type_ == CodeType::Object;
        }
        if let Some(assignment) = expr.try_as_assignment() {
            return assignment.get_left().is_field();
        }
        if let Some(function) = expr.try_as_function() {
            return matches!(
                function.get_func_type(),
                FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi
            ) && function.get_lst_operands().get(0).is_field();
        }
        false
    }
}
