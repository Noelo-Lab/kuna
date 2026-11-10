//! kuna fix `kt_inline_lock_local` (spec 10.16): the Kotlin inliner keeps a
//! lock local whose value, evaluated again, may be another object.
//!
//! `synchronized (l) { .. }` holds the monitor of the object `l` holds. The
//! Kotlin plugin's last `SimplifyStack` (`INLINE_ALL_VARS`) copies a local's
//! value into the uses it may reach and keeps the definition for the others,
//! so `val l = this.lock; synchronized (l) { l.f() }` became
//! `val l = this.lock; synchronized (this.lock) { l.f() }`: the field is read
//! again for the lock, and another thread may have stored another object in
//! it between the two reads, so the body runs holding a monitor the bytecode
//! never takes while `l` names a different object. Array elements and calls
//! the inliner moves the same way.
//!
//! With the fix, a local whose value reaches a monitor operand (directly or
//! through the compiler's copies of it) is inlined only into its single use,
//! which moves the evaluation instead of repeating it, unless its value
//! evaluates to the same object wherever it is evaluated again: a constant, a
//! local (the inliner checks its version), a final field of such a value,
//! and casts and conditionals of those.

use std::collections::VecDeque;

use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::rt::JHashSet;
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;

pub struct KtInlineLockLocal;

impl KtInlineLockLocal {
    /// The SSAU versions whose value a monitor takes: the monitor operands'
    /// uses (a monitor is always a statement of its own), and the uses of
    /// the locals they copy.
    pub fn lock_uses(
        dgraph: DirectGraph,
        ssau: &SSAUConstructorSparseEx,
    ) -> JHashSet<VarVersionPair> {
        let mut uses: JHashSet<VarVersionPair> = JHashSet::new();
        dgraph.iterate_exprents(|ex: Exprent| {
            if let Some(mon) = ex.try_as_monitor() {
                if let Some(var) = mon.get_value().try_as_var() {
                    uses.add(var.get_var_version_pair());
                }
            }
            0
        });
        if uses.is_empty() {
            return uses;
        }

        let versions = ssau.get_ssu_versions();
        let copies = ssau.get_var_assignment_map();
        let mut work: VecDeque<VarVersionPair> = uses.to_vec().into_iter().collect();
        let mut seen: JHashSet<VarVersionNode> = JHashSet::new();
        while let Some(pair) = work.pop_front() {
            let Some(start) = versions.get_node_with_key(pair) else {
                continue;
            };
            let mut nodes: VecDeque<VarVersionNode> = VecDeque::from([start]);
            while let Some(node) = nodes.pop_front() {
                if !seen.add(node) {
                    continue;
                }
                if let Some(copied) = copies.get(&node.as_pair()).copied() {
                    if uses.add(copied) {
                        work.push_back(copied);
                    }
                }
                nodes.extend(node.predecessors());
            }
        }
        uses
    }

    /// Whether the local defined as `def` with value `value` must keep every
    /// use: `lock_uses` is reachable from it and `value` may evaluate to
    /// another object a second time.
    pub fn keeps_uses(
        def: VarVersionPair,
        value: Exprent,
        ssau: &SSAUConstructorSparseEx,
        lock_uses: &JHashSet<VarVersionPair>,
    ) -> bool {
        !lock_uses.is_empty() && !Self::same_object(value) && Self::reaches(def, ssau, lock_uses)
    }

    fn reaches(
        def: VarVersionPair,
        ssau: &SSAUConstructorSparseEx,
        lock_uses: &JHashSet<VarVersionPair>,
    ) -> bool {
        let Some(start) = ssau.get_ssu_versions().get_node_with_key(def) else {
            return false;
        };
        let mut seen: JHashSet<VarVersionNode> = JHashSet::new();
        let mut nodes: VecDeque<VarVersionNode> = VecDeque::from([start]);
        while let Some(node) = nodes.pop_front() {
            if !seen.add(node) {
                continue;
            }
            if node != start && lock_uses.contains(&node.as_pair()) {
                return true;
            }
            nodes.extend(node.successors());
        }
        false
    }

    /// Whether every evaluation of `value` gives the same object.
    pub fn same_object(value: Exprent) -> bool {
        if value.is_const() || value.is_var() {
            return true;
        }
        if let Some(field) = value.try_as_field() {
            let is_final = DecompilerContext::get_struct_context()
                .get_class(&field.get_classname())
                .and_then(|cl| {
                    cl.get_field(&field.get_name(), &field.get_descriptor().descriptor_string)
                })
                .is_some_and(|fd| fd.has_modifier(ACC_FINAL));
            return is_final && field.get_instance().is_none_or(Self::same_object);
        }
        if let Some(func) = value.try_as_function() {
            return matches!(
                func.get_func_type(),
                FunctionType::Cast | FunctionType::Ternary
            ) && func.get_lst_operands().iter().all(Self::same_object);
        }
        false
    }
}
