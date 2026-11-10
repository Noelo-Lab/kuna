//! Port of `org/jetbrains/java/decompiler/modules/decompiler/StackVarsProcessor.java`.
//!
//! A Java `null` head slot is an empty `JList`, so the Java's `nd.exprents.add(null)`
//! on an emptied loop init/condition/increment list is a no-op here.
//!
//! kuna fixes (spec 10.16), both for the pass that inlines every local
//! (`inlineRegularVars`, the Kotlin plugin's): `kt_inline_evaluation_order`
//! inlines a local whose value calls, allocates or reads an array element only
//! into the next statement, ahead of anything there it could observe or be
//! observed by; `field_read_all_paths` builds the SSAU form with field reads
//! live only while live on every path; `field_read_reuse` keeps a field read
//! with several uses in its stack variable (`kuna_fix_field_read_reuse`);
//! `kt_inline_lock_local` keeps every use of a lock local whose value may be
//! another object when evaluated again (`kuna_fix_kt_inline_lock_local`).

use std::collections::VecDeque;

use crate::cancelation_manager::CancelationManager;
use crate::catch_statement::CatchStatement;
use crate::code_constants::INIT_NAME;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::do_statement::Type as DoType;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::method_processor::ScratchScope;
use crate::monitor_exprent::Type as MonitorType;
use crate::p3_dataflow::kuna_fix_field_read_all_paths::FieldReadAllPaths;
use crate::p3_dataflow::kuna_fix_field_read_reuse;
use crate::p3_dataflow::kuna_fix_kt_inline_evaluation_order::KtInlineEvaluationOrder;
use crate::p3_dataflow::kuna_fix_kt_inline_lock_local::KtInlineLockLocal;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::sequence_helper::SequenceHelper;
use crate::simplify_exprents_helper::SimplifyExprentsHelper;
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_graph::VarVersionsGraph;

pub struct StackVarsProcessor;

/// `{returnExprent, changed, isReplaceable}` of `iterateChildExprent`.
type ChildRet = (Option<Exprent>, bool, bool);

const DEFAULT_OPTIONS: StackSimplifyOptions = StackSimplifyOptions {
    inline_regular_vars: false,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct StackSimplifyOptions {
    inline_regular_vars: bool,
}

impl StackSimplifyOptions {
    pub fn new() -> StackSimplifyOptions {
        StackSimplifyOptions {
            inline_regular_vars: false,
        }
    }

    pub fn inline_regular_vars(mut self) -> StackSimplifyOptions {
        self.inline_regular_vars = true;
        self
    }
}

impl StackVarsProcessor {
    pub fn simplify_stack_vars(root: RootStatement, mt: StructMethod, cl: StructClass) {
        Self::simplify_stack_vars_options(root, mt, cl, DEFAULT_OPTIONS);
    }

    /// Java: simplifyStackVars(RootStatement, StructMethod, StructClass, StackSimplifyOptions)
    pub fn simplify_stack_vars_options(
        root: RootStatement,
        mt: StructMethod,
        cl: StructClass,
        options: StackSimplifyOptions,
    ) {
        let mut scratch = ScratchScope::open();
        let mut set_reordered_ifs: JHashSet<i32> = JHashSet::new();
        let mut ssau: Option<SSAUConstructorSparseEx> = None;
        loop {
            CancelationManager::check_canceled();
            let mut found = false;
            scratch.checkpoint();
            let first = ssau.is_none();

            let mut ssa = SSAConstructorSparseEx::new();
            ssa.split_variables(root, mt);

            while SimplifyExprentsHelper::simplify_stack_vars_statement(
                root.0,
                &mut set_reordered_ifs,
                Some(&ssa),
                cl,
                first,
            ) {
                ValidationHelper::validate_statement(root);
                found = true;
            }

            Self::set_versions_to_null(root.0);

            SequenceHelper::condense_sequences(root.0);
            ValidationHelper::validate_statement(root);

            scratch.checkpoint();
            let current = ssau.insert(SSAUConstructorSparseEx::new());
            current.field_reads_all_paths = Self::field_reads_all_paths(options);
            current.split_variables(root, mt);

            if first {
                Self::set_effectively_final_vars(root.0, current, &mut JHashMap::new());
                ValidationHelper::validate_statement(root);
            }

            if Self::iterate_statements(root, current, options) {
                ValidationHelper::validate_statement(root);
                found = true;
            }

            Self::set_versions_to_null(root.0);

            if !found {
                break;
            }
        }

        scratch.checkpoint();
        let mut ssau = SSAUConstructorSparseEx::new();
        ssau.field_reads_all_paths = Self::field_reads_all_paths(options);
        ssau.split_variables(root, mt);
        ValidationHelper::validate_statement(root);

        Self::iterate_statements(root, &mut ssau, options);

        Self::set_versions_to_null(root.0);
    }

    /// Whether this pass's SSAU forms keep a field read live only while it is
    /// live on every path (fix `field_read_all_paths`).
    fn field_reads_all_paths(options: StackSimplifyOptions) -> bool {
        options.inline_regular_vars && kuna_fixes::enabled(KunaFix::FieldReadAllPaths)
    }

    pub fn set_versions_to_null(stat: Statement) {
        match stat.get_exprents() {
            None => {
                let sub_stats = stat.get_stats().borrow().to_vec();
                for st in sub_stats {
                    Self::set_versions_to_null(st);
                }

                for exprent in stat.get_stat_exprents() {
                    Self::set_exprent_versions_to_null(exprent);
                }
            }
            Some(exprents) => {
                for exprent in exprents.iter() {
                    Self::set_exprent_versions_to_null(exprent);
                }
            }
        }
    }

    fn set_exprent_versions_to_null(exprent: Exprent) {
        let lst = exprent.get_all_exprents_rec_self(true, true);

        for expr in lst {
            if let Some(var) = expr.try_as_var() {
                var.set_version(0);
            }
        }
    }

    fn iterate_statements(
        root: RootStatement,
        ssa: &mut SSAUConstructorSparseEx,
        options: StackSimplifyOptions,
    ) -> bool {
        let mut flatthelper = FlattenStatementsHelper::new();
        let dgraph = flatthelper.build_direct_graph(root);

        let mut res = false;

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();
        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        let mut stack_maps: VecDeque<JHashMap<VarVersionPair, Exprent>> = VecDeque::new();

        stack.push_back(dgraph.first().expect("NullPointerException"));
        stack_maps.push_back(JHashMap::new());

        let mut lvts: JHashMap<VarVersionPair, LocalVariable> = JHashMap::new();

        let lock_uses =
            if options.inline_regular_vars && kuna_fixes::enabled(KunaFix::KtInlineLockLocal) {
                KtInlineLockLocal::lock_uses(dgraph, ssa)
            } else {
                JHashSet::new()
            };

        let mut ret = [0, 0];
        while let Some(mut nd) = stack.pop_front() {
            let mut map_var_values = stack_maps.pop_front().expect("NoSuchElementException");

            if set_visited.contains(&nd) {
                continue;
            }

            set_visited.add(nd);

            let mut lst_lists: Vec<JList<Exprent>> = Vec::new();

            if !nd.exprents().is_empty() {
                lst_lists.push(nd.exprents());
            }

            let succs = nd.get_successors(DirectEdgeType::Regular);
            let mut following = (succs.len() == 1)
                .then(|| succs.get(0).get_destination().exprents())
                .and_then(|list| (!list.is_empty()).then(|| list.get(0)));
            if succs.len() == 1 {
                let ndsucc = succs.get(0).get_destination();

                if ndsucc.type_() == DirectNodeType::Tail && !ndsucc.exprents().is_empty() {
                    lst_lists.push(succs.get(0).get_destination().exprents());
                    nd = ndsucc;
                    following = None;
                }
            }

            let mut stack_stage = 0;
            while stack_stage < 2 {
                if !DecompilerContext::get_option(
                    IFernflowerPreferences::SIMPLIFY_STACK_SECOND_PASS,
                ) {
                    stack_stage = 2;
                }

                for i in 0..lst_lists.len() {
                    let lst = lst_lists[i].clone();

                    let mut index: i32 = 0;
                    while index < lst.len() as i32 {
                        let mut next = None;
                        let mut evaluated_next = following;

                        if index == lst.len() as i32 - 1 {
                            if i < lst_lists.len() - 1 {
                                next = Some(lst_lists[i + 1].get(0));
                                evaluated_next = next;
                            }
                        } else {
                            next = Some(lst.get(index as usize + 1));
                            evaluated_next = next;
                        }

                        let simplify_across_stack = stack_stage == 1;

                        Self::iterate_exprent(
                            &lst,
                            nd.statement(),
                            &mut lvts,
                            index,
                            next,
                            evaluated_next,
                            &mut map_var_values,
                            ssa,
                            simplify_across_stack,
                            &mut ret,
                            options,
                            &lock_uses,
                        );

                        if ret[0] >= 0 {
                            index = ret[0];
                        } else {
                            index += 1;
                        }

                        let changed = ret[1] == 1;
                        res |= changed;

                        if changed {
                            stack_stage = 2;
                        }
                    }
                }

                stack_stage += 1;
            }

            for ndx in succs.iter() {
                stack.push_back(ndx.get_destination());
                stack_maps.push_back(map_var_values.clone());
            }

            if nd.exprents().is_empty()
                && (nd.type_() == DirectNodeType::Init
                    || nd.type_() == DirectNodeType::Condition
                    || nd.type_() == DirectNodeType::Increment)
            {
                if let Some(loop_) = nd.statement().try_as_do() {
                    if loop_.get_looptype() == DoType::For
                        && loop_.get_init_exprent().is_none()
                        && loop_.get_inc_exprent().is_none()
                    {
                        loop_.set_looptype(DoType::While);
                    }
                }
            }
        }

        dgraph.iterate_exprents_deep(|ex: Exprent| {
            if let Some(var) = ex.try_as_var() {
                if lvts.contains_key(&var.get_var_version_pair()) {
                    let lvt = lvts.get_cloned(&var.get_var_version_pair());
                    if var.get_lvt().is_none() {
                        var.set_lvt(lvt);
                    }
                }
            }

            0
        });

        res
    }

    fn is_replaceable_var(
        exprent: Exprent,
        map_var_values: &JHashMap<VarVersionPair, Exprent>,
    ) -> Option<Exprent> {
        let mut dest = None;

        if let Some(var) = exprent.try_as_var() {
            dest = map_var_values.get(&VarVersionPair::from_var(var)).copied();
        }

        dest
    }

    fn replace_single_var(
        parent: Exprent,
        var: VarExprent,
        dest: Exprent,
        ssau: &SSAUConstructorSparseEx,
    ) {
        parent.replace_exprent(var.0, dest);
        dest.add_bytecode_offsets(var.get_bytecode().as_ref());

        let live_map = ssau.get_live_var_versions_map(VarVersionPair::from_var(var));

        let set_vars = Self::get_all_versions(dest);

        for pair in set_vars.to_vec() {
            let node = ssau
                .get_ssu_versions()
                .get_node_with_key(pair)
                .expect("NullPointerException");

            let Some(live) = node.live() else {
                continue;
            };

            let entries = live.borrow().entry_list();
            for (key, set) in entries {
                let live_map = live_map.as_ref().expect("NullPointerException");

                if !live_map.borrow().contains_key(key) {
                    live.borrow_mut().remove(key);
                } else {
                    let other = live_map.borrow().get(key).expect("NullPointerException");

                    if set.ptr_eq(&other) {
                        let snapshot = other.borrow().clone();
                        set.borrow_mut().complement(&snapshot);
                    } else {
                        set.borrow_mut().complement(&other.borrow());
                    }

                    if set.borrow().is_empty() {
                        live.borrow_mut().remove(key);
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_exprent(
        lst_exprents: &JList<Exprent>,
        stat: Statement,
        lvts: &mut JHashMap<VarVersionPair, LocalVariable>,
        index: i32,
        next: Option<Exprent>,
        evaluated_next: Option<Exprent>,
        map_var_values: &mut JHashMap<VarVersionPair, Exprent>,
        ssau: &mut SSAUConstructorSparseEx,
        simplify_across_stack: bool,
        ret: &mut [i32; 2],
        options: StackSimplifyOptions,
        lock_uses: &JHashSet<VarVersionPair>,
    ) {
        let mut next = next;
        let exprent = lst_exprents.get(index as usize);

        let mut changed = 0;

        let mut arr: ChildRet = (None, false, false);
        for child in exprent.get_all_exprents() {
            let mut expr = child;
            loop {
                Self::iterate_child_exprent(
                    expr,
                    exprent,
                    next,
                    map_var_values,
                    ssau,
                    &mut arr,
                    options,
                );
                let retexpr = arr.0;
                changed |= if arr.1 { 1 } else { 0 };

                let is_replaceable = arr.2;
                if let Some(retexpr) = retexpr {
                    if is_replaceable {
                        Self::replace_single_var(exprent, expr.as_var(), retexpr, ssau);
                        expr = retexpr;
                    } else {
                        exprent.replace_exprent(expr, retexpr);
                        retexpr.add_bytecode_offsets(expr.get_bytecode().as_ref());
                    }

                    changed = 1;
                }

                if !is_replaceable {
                    break;
                }
            }
        }

        if exprent.is_var() {
            let res = Self::is_replaceable_var(exprent, map_var_values);

            let Some(res) = res else {
                Self::set_ret(ret, -1, 0);
                return;
            };

            lst_exprents.set(index as usize, res);
            Self::set_ret(ret, index + 1, 1);
            return;
        }

        let mut left: Option<VarExprent> = None;
        let mut right: Option<Exprent> = None;

        if let Some(as_) = exprent.try_as_assignment() {
            if as_.get_cond_type().is_none() {
                if let Some(left_var) = as_.get_left().try_as_var() {
                    left = Some(left_var);
                    right = Some(as_.get_right());
                }
            }
        }

        let left = match left {
            Some(left) if !left.is_effectively_final() => left,
            _ => {
                Self::set_ret(ret, -1, changed);
                return;
            }
        };
        let right = right.expect("NullPointerException");

        let left_var = VarVersionPair::from_var(left);

        let mut used_vers: Vec<VarVersionNode> = Vec::new();
        let notdom = Self::get_used_versions(ssau, left_var, &mut used_vers);

        if !notdom && used_vers.is_empty() {
            if left.is_stack() && (right.is_invocation() || right.is_assignment() || right.is_new())
            {
                if let Some(nexpr) = right.try_as_new() {
                    let new_type = nexpr.get_new_type();
                    if new_type.array_dim > 0 || new_type.type_ != CodeType::Object {
                        Self::set_ret(ret, -1, changed);
                        return;
                    }
                }

                lst_exprents.set(index as usize, right);
                Self::set_ret(ret, index + 1, 1);
                return;
            } else if right.is_var() {
                Self::on_deleted_var(stat, left, right, lvts, ssau);

                lst_exprents.remove_at(index as usize);
                Self::set_ret(ret, index, 1);
                return;
            } else if left.is_stack() && right.is_function() {
                let func = right.as_function();

                if func.get_func_type().is_postfix_ppmm() {
                    lst_exprents.set(index as usize, right);
                    Self::set_ret(ret, index, 1);
                    return;
                } else if func.get_func_type() == FunctionType::Cast
                    && !func.get_lst_operands().get(0).is_invocation()
                {
                    lst_exprents.remove_at(index as usize);
                    Self::set_ret(ret, index, 1);
                    return;
                }

                Self::set_ret(ret, -1, changed);

                return;
            } else if left.is_stack() && right.is_field() {
                lst_exprents.remove_at(index as usize);
                Self::set_ret(ret, index, 1);
                return;
            } else {
                Self::set_ret(ret, -1, changed);
                return;
            }
        }

        let useflags = right.get_exprent_use();

        if (!left.is_stack() && !options.inline_regular_vars)
            && right
                .try_as_var()
                .is_none_or(|variable| !variable.is_catch_temp_var())
        {
            Self::set_ret(ret, -1, changed);
            return;
        }

        if (useflags & Exprent::MULTIPLE_USES) == 0 && (notdom || used_vers.len() > 1) {
            Self::set_ret(ret, -1, changed);
            return;
        }

        if !left.is_stack()
            && (notdom || used_vers.len() > 1)
            && KtInlineLockLocal::keeps_uses(left_var, right, ssau, lock_uses)
        {
            Self::set_ret(ret, -1, changed);
            return;
        }

        let mut map_vars = Self::get_all_var_versions(left_var, right, ssau);

        let is_self_reference = map_vars.contains_key(&left_var.var);
        if is_self_reference && notdom {
            Self::set_ret(ret, -1, changed);
            return;
        }

        let untracked_read = FieldReadAllPaths::untracked_read(left_var, right, ssau);

        if simplify_across_stack {
            let simplified_across_stack =
                Self::simplify_across_stack_exprent(lst_exprents, index, next, right, left);

            if simplified_across_stack.is_some() {
                next = simplified_across_stack;
            }
        }

        let set_next_vars = next.map(Self::get_all_versions);

        if let Some(next_vars) = set_next_vars
            .as_ref()
            .filter(|_| !right.is_const() && !right.is_var())
            .filter(|_| map_vars.contains_key(&left_var.var))
        {
            for usedvar in &used_vers {
                if !next_vars.contains(&VarVersionPair::new(usedvar.var(), usedvar.version())) {
                    Self::set_ret(ret, -1, changed);
                    return;
                }
            }
        }

        map_vars.remove(&left_var.var);

        let mut vernotreplaced = false;
        let mut verreplaced = false;

        let mut set_temp_used_vers: JHashSet<VarVersionPair> = JHashSet::new();

        let keeps_single_read = left.is_stack()
            && !options.inline_regular_vars
            && right.is_field()
            && kuna_fixes::enabled(KunaFix::FieldReadReuse)
            && kuna_fix_field_read_reuse::keeps_single_read(
                right,
                &used_vers,
                lst_exprents,
                index as usize,
            );

        for usedvar in &used_vers {
            let usedver = VarVersionPair::new(usedvar.var(), usedvar.version());

            if Self::is_version_to_be_replaced(usedver, &map_vars, ssau, left_var)
                && (right.is_const()
                    || right.is_var()
                    || (right.is_field() && !keeps_single_read)
                    || set_next_vars
                        .as_ref()
                        .is_none_or(|next_vars| next_vars.contains(&usedver)))
                && (left.is_stack()
                    || !kuna_fixes::enabled(KunaFix::KtInlineEvaluationOrder)
                    || KtInlineEvaluationOrder::may_move(right, evaluated_next, usedver))
                && (!untracked_read
                    || KtInlineEvaluationOrder::heap_unchanged_before(right, evaluated_next, usedver))
                && !ssau.leaves_finally_body(left_var, usedver)
            {
                set_temp_used_vers.add(usedver);
                verreplaced = true;
            } else {
                vernotreplaced = true;
            }
        }

        if is_self_reference && vernotreplaced {
            Self::set_ret(ret, -1, changed);
            return;
        } else {
            for usedver in set_temp_used_vers.to_vec() {
                let copy = right.copy();

                if right.is_field() && ssau.get_map_field_vars().contains_key(&right.get_id()) {
                    let field_var = ssau
                        .get_map_field_vars()
                        .get_cloned(&right.get_id())
                        .expect("NullPointerException");
                    ssau.get_map_field_vars_mut().put(copy.get_id(), field_var);
                }

                map_var_values.put(usedver, copy);
            }
        }

        if !notdom && !vernotreplaced {
            Self::on_deleted_var(stat, left, right, lvts, ssau);
            lst_exprents.remove_at(index as usize);
            Self::set_ret(ret, index, 1);
        } else if verreplaced {
            Self::set_ret(ret, index + 1, changed);
        } else {
            Self::set_ret(ret, -1, changed);
        }
    }

    fn on_deleted_var(
        stat: Statement,
        left: VarExprent,
        right: Exprent,
        lvts: &mut JHashMap<VarVersionPair, LocalVariable>,
        ssau: &SSAUConstructorSparseEx,
    ) {
        if let Some(right_var) = right.try_as_var() {
            if right_var.get_lvt().is_none() && left.get_lvt().is_some() {
                let versions = ssau.get_ssu_versions();
                let node = versions
                    .get_node_with_key(left.get_var_version_pair())
                    .expect("NullPointerException");

                let mut nodes: VecDeque<VarVersionNode> = VecDeque::new();
                let mut seen: JHashSet<VarVersionNode> = JHashSet::new();
                nodes.push_back(node);

                while let Some(nd) = nodes.pop_front() {
                    seen.add(nd);

                    if nd.var() == right_var.get_index() {
                        lvts.put(nd.as_pair(), left.get_lvt().expect("NullPointerException"));
                    }

                    let preds = Self::predecessors(nd, versions, ssau.get_var_assignment_map());
                    for pred in preds.to_vec() {
                        let Some(pred) = pred else {
                            ValidationHelper::validate_true(false, "Predecessor is null!");
                            continue;
                        };

                        if !seen.contains(&pred) {
                            nodes.push_back(pred);
                        }
                    }
                }

                if let Some(catch_st) = Self::enclosing_catch(stat) {
                    for var in catch_st.get_vars().iter() {
                        if var.get_index() == right_var.get_index() {
                            var.set_lvt(left.get_lvt());
                        }
                    }
                }
            }
        }
    }

    fn predecessors(
        node: VarVersionNode,
        graph: VarVersionsGraph,
        assignments: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> JHashSet<Option<VarVersionNode>> {
        let mut res: JHashSet<Option<VarVersionNode>> = JHashSet::new();
        res.add_all(node.predecessors().into_iter().map(Some));

        let assign = assignments.get(&node.as_pair()).copied();
        if let Some(assign) = assign {
            res.add(graph.get_node_with_key(assign));
        }

        res
    }

    fn enclosing_catch(mut stat: Statement) -> Option<CatchStatement> {
        while let Some(parent) = stat.get_parent() {
            if let Some(st) = parent.try_as_catch() {
                return Some(st);
            }

            stat = parent;
        }

        None
    }

    fn simplify_across_stack_exprent(
        exprents: &JList<Exprent>,
        index: i32,
        next: Option<Exprent>,
        right: Exprent,
        left: VarExprent,
    ) -> Option<Exprent> {
        let mut ret = None;

        if let Some(next_assign) = next.and_then(Exprent::try_as_assignment) {
            if index < exprents.len() as i32 - 2 {
                let next_right = next_assign.get_right();

                let all_right = right.get_all_exprents_rec(true);
                let all_next_right = next_right.get_all_exprents_rec(true);

                if all_right.len() == all_next_right.len() && !all_right.is_empty() {
                    let ok = Self::are_trees_equal(left, &all_right, &all_next_right);

                    if ok {
                        ret = Some(exprents.get(index as usize + 2));
                    }
                } else if all_next_right.len() > all_right.len() {
                    for exprent in &all_next_right {
                        let subtree = exprent.get_all_exprents_rec(true);

                        if all_right.len() == subtree.len() && !all_right.is_empty() {
                            let ok = Self::are_trees_equal(left, &all_right, &subtree);

                            if ok {
                                ret = Some(exprents.get(index as usize + 2));

                                break;
                            }
                        }
                    }
                }
            }
        }

        ret
    }

    fn set_ret(ret: &mut [i32; 2], a: i32, b: i32) {
        ret[0] = a;
        ret[1] = b;
    }

    fn are_trees_equal(left: VarExprent, tree_a: &[Exprent], tree_b: &[Exprent]) -> bool {
        let mut ok = true;

        for i in 0..tree_a.len() {
            let a = tree_a[i];
            let b = tree_b[i];

            if a.get_type() != b.get_type() {
                ok = false;
                break;
            }

            if let (Some(va), Some(vb)) = (a.try_as_var(), b.try_as_var()) {
                if va.get_index() != vb.get_index() {
                    ok = false;
                    break;
                }

                if vb.get_index() == left.get_index() {
                    ok = false;
                    break;
                }
            }

            if let (Some(fa), Some(fb)) = (a.try_as_field(), b.try_as_field()) {
                if fa.get_name() != fb.get_name()
                    || fa.get_classname() != fb.get_classname()
                    || fa.is_static() != fb.is_static()
                    || !fa.get_descriptor().equals(&fb.get_descriptor())
                {
                    ok = false;
                    break;
                }
            }

            if a.is_const() && b.is_const() && !a.equals(b) {
                ok = false;
                break;
            }
        }

        ok
    }

    fn get_all_versions(exprent: Exprent) -> JHashSet<VarVersionPair> {
        let mut res = JHashSet::new();

        let mut exprents = exprent.get_all_exprents_rec(true);
        exprents.push(exprent);

        for expr in exprents {
            if let Some(var) = expr.try_as_var() {
                res.add(VarVersionPair::from_var(var));
            }
        }

        res
    }

    fn iterate_child_exprent(
        exprent: Exprent,
        parent: Exprent,
        next: Option<Exprent>,
        map_var_values: &mut JHashMap<VarVersionPair, Exprent>,
        ssau: &mut SSAUConstructorSparseEx,
        ret: &mut ChildRet,
        options: StackSimplifyOptions,
    ) {
        let mut changed = false;

        let mut arr: ChildRet = (None, false, false);
        for child in exprent.get_all_exprents() {
            let mut expr = child;
            loop {
                Self::iterate_child_exprent(
                    expr,
                    parent,
                    next,
                    map_var_values,
                    ssau,
                    &mut arr,
                    options,
                );
                let retexpr = arr.0;
                changed |= arr.1;

                let is_replaceable = arr.2;
                if let Some(retexpr) = retexpr {
                    if is_replaceable {
                        Self::replace_single_var(exprent, expr.as_var(), retexpr, ssau);
                        expr = retexpr;
                    } else {
                        exprent.replace_exprent(expr, retexpr);
                        retexpr.add_bytecode_offsets(expr.get_bytecode().as_ref());
                    }

                    changed = true;
                }

                if !is_replaceable {
                    break;
                }
            }
        }

        let dest = Self::is_replaceable_var(exprent, map_var_values);
        if dest.is_some() {
            Self::set_ret_child(ret, dest, true, true);
            return;
        }

        let mut left: Option<VarExprent> = None;
        let mut right: Option<Exprent> = None;

        if let Some(as_) = exprent.try_as_assignment() {
            if as_.get_cond_type().is_none() {
                if let Some(left_var) = as_.get_left().try_as_var() {
                    left = Some(left_var);
                    right = Some(as_.get_right());
                }
            }
        }

        let left = match left {
            Some(left) if !left.is_effectively_final() => left,
            _ => {
                Self::set_ret_child(ret, None, changed, false);
                return;
            }
        };
        let right = right.expect("NullPointerException");

        let mut is_head_synchronized = false;
        if next.is_none() {
            if let Some(monexpr) = parent.try_as_monitor() {
                if monexpr.get_mon_type() == MonitorType::Enter
                    && exprent.equals(monexpr.get_value())
                {
                    is_head_synchronized = true;
                }
            }
        }

        if (!left.is_stack() && !options.inline_regular_vars) && !is_head_synchronized {
            Self::set_ret_child(ret, None, changed, false);
            return;
        }

        let left_var = VarVersionPair::from_var(left);

        let mut used_vers: Vec<VarVersionNode> = Vec::new();
        let notdom = Self::get_used_versions(ssau, left_var, &mut used_vers);

        if !notdom && used_vers.is_empty() {
            Self::set_ret_child(ret, Some(right), changed, false);
            return;
        }

        if !left.is_stack() {
            Self::set_ret_child(ret, None, changed, false);
            return;
        }

        let useflags = right.get_exprent_use();

        if (useflags & Exprent::BOTH_FLAGS) != Exprent::BOTH_FLAGS {
            Self::set_ret_child(ret, None, changed, false);
            return;
        }

        let mut map_vars = Self::get_all_var_versions(left_var, right, ssau);
        if map_vars.contains_key(&left_var.var) && notdom {
            Self::set_ret_child(ret, None, changed, false);
            return;
        }

        if FieldReadAllPaths::untracked_read(left_var, right, ssau) {
            Self::set_ret_child(ret, None, changed, false);
            return;
        }

        map_vars.remove(&left_var.var);

        let mut set_allowed_vars = Self::get_all_versions(parent);
        if let Some(next) = next {
            set_allowed_vars.add_all(Self::get_all_versions(next).to_vec());
        }

        let mut vernotreplaced = false;

        let mut set_temp_used_vers: JHashSet<VarVersionPair> = JHashSet::new();

        for usedvar in &used_vers {
            let usedver = VarVersionPair::new(usedvar.var(), usedvar.version());
            if Self::is_version_to_be_replaced(usedver, &map_vars, ssau, left_var)
                && (right.is_var() || set_allowed_vars.contains(&usedver))
            {
                set_temp_used_vers.add(usedver);
            } else {
                vernotreplaced = true;
            }
        }

        if !notdom && !vernotreplaced {
            for usedver in set_temp_used_vers.to_vec() {
                let copy = right.copy();
                if right.is_field() && ssau.get_map_field_vars().contains_key(&right.get_id()) {
                    let field_var = ssau
                        .get_map_field_vars()
                        .get_cloned(&right.get_id())
                        .expect("NullPointerException");
                    ssau.get_map_field_vars_mut().put(copy.get_id(), field_var);
                }

                map_var_values.put(usedver, copy);
            }

            Self::set_ret_child(ret, Some(right), changed, false);
            return;
        }

        Self::set_ret_child(ret, None, changed, false);
    }

    /// Java: setRet(Object[], Object, boolean, boolean)
    fn set_ret_child(ret: &mut ChildRet, a: Option<Exprent>, b: bool, c: bool) {
        ret.0 = a;
        ret.1 = b;
        ret.2 = c;
    }

    fn get_used_versions(
        ssa: &SSAUConstructorSparseEx,
        var: VarVersionPair,
        res: &mut Vec<VarVersionNode>,
    ) -> bool {
        let ssu = ssa.get_ssu_versions();
        let node = ssu.get_node_with_key(var).expect("NullPointerException");

        let mut set_visited: JHashSet<VarVersionNode> = JHashSet::new();
        let mut set_not_doms: JHashSet<VarVersionNode> = JHashSet::new();

        let mut stack: VecDeque<VarVersionNode> = VecDeque::new();
        stack.push_back(node);

        while let Some(nd) = stack.pop_front() {
            set_visited.add(nd);

            if nd != node {
                res.push(nd);
            }

            for dest in nd.successors() {
                if set_visited.contains(&dest) {
                    continue;
                }

                let mut is_dominated = true;
                for source in dest.predecessors() {
                    if !set_visited.contains(&source) {
                        is_dominated = false;
                        break;
                    }
                }

                if is_dominated {
                    stack.push_back(dest);
                } else {
                    set_not_doms.add(dest);
                }
            }
        }

        set_not_doms.remove_all(&set_visited);

        !set_not_doms.is_empty()
    }

    fn is_version_to_be_replaced(
        usedvar: VarVersionPair,
        map_vars: &JHashMap<i32, Option<JHashSet<VarVersionPair>>>,
        ssau: &SSAUConstructorSparseEx,
        leftpaar: VarVersionPair,
    ) -> bool {
        let ssuversions = ssau.get_ssu_versions();

        if ssuversions
            .get_node_with_key(usedvar)
            .expect("NullPointerException")
            .phantom_node()
            .is_some()
        {
            return false;
        }

        let Some(map_live_vars) = ssau.get_live_var_versions_map(usedvar) else {
            return false;
        };

        if ssau.get_map_version_first_range().get_cloned(&leftpaar)
            != ssau.get_map_version_first_range().get_cloned(&usedvar)
        {
            return false;
        }

        for (key, value) in map_vars.iter() {
            let liveverset = map_live_vars.borrow().get(*key);
            let Some(liveverset) = liveverset else {
                return false;
            };

            let mut domset: JHashSet<VarVersionNode> = JHashSet::new();
            for verpaar in value.as_ref().expect("NullPointerException").iter() {
                domset.add(
                    ssuversions
                        .get_node_with_key(*verpaar)
                        .expect("NullPointerException"),
                );
            }

            let mut isdom = true;

            let livevers: Vec<i32> = liveverset.borrow().iter().collect();
            for livever in livevers {
                let node = ssuversions
                    .get_node_with_key(VarVersionPair::new(*key, livever))
                    .expect("NullPointerException");

                if !ssuversions.is_dominator_set(node, &domset) {
                    isdom = false;
                    break;
                }
            }

            if !isdom {
                return false;
            }
        }

        true
    }

    fn get_all_var_versions(
        leftvar: VarVersionPair,
        exprent: Exprent,
        ssau: &SSAUConstructorSparseEx,
    ) -> JHashMap<i32, Option<JHashSet<VarVersionPair>>> {
        let mut map: JHashMap<i32, Option<JHashSet<VarVersionPair>>> = JHashMap::new();
        let map_live_vars = ssau.get_live_var_versions_map(leftvar);

        let mut lst = exprent.get_all_exprents_rec(true);
        lst.push(exprent);

        for expr in lst {
            if let Some(var) = expr.try_as_var() {
                let varindex = var.get_index();

                if leftvar.var != varindex {
                    let live = map_live_vars.as_ref().expect("NullPointerException");
                    let versions = live.borrow().get(varindex);
                    if let Some(versions) = versions {
                        let mut verset = JHashSet::new();
                        for vers in versions.borrow().iter() {
                            verset.add(VarVersionPair::new(varindex, vers));
                        }

                        map.put(varindex, Some(verset));
                    } else {
                        jthrow!("inconsistent live map!");
                    }
                } else {
                    map.put(varindex, None);
                }
            } else if expr.is_field() && ssau.get_map_field_vars().contains_key(&expr.get_id()) {
                let varindex = ssau
                    .get_map_field_vars()
                    .get_cloned(&expr.get_id())
                    .expect("NullPointerException");

                let live = map_live_vars.as_ref().expect("NullPointerException");
                let versions = live.borrow().get(varindex);
                if let Some(versions) = versions {
                    let mut verset = JHashSet::new();

                    for vers in versions.borrow().iter() {
                        verset.add(VarVersionPair::new(varindex, vers));
                    }

                    map.put(varindex, Some(verset));
                }
            }
        }

        map
    }

    fn set_effectively_final_vars(
        stat: Statement,
        ssau: &SSAUConstructorSparseEx,
        var_lookup_map: &mut JHashMap<VarVersionPair, VarExprent>,
    ) {
        if let Some(exprents) = stat.get_exprents() {
            if !exprents.is_empty() {
                let mut i = 0;
                while i < exprents.len() {
                    Self::set_effectively_final_vars_exprent(
                        stat,
                        exprents.get(i),
                        ssau,
                        i as i32,
                        &exprents,
                        var_lookup_map,
                    );
                    i += 1;
                }
            }
        }

        let sub_stats = stat.get_stats().borrow().to_vec();

        for st in sub_stats {
            Self::set_effectively_final_vars(st, ssau, var_lookup_map);
        }
    }

    /// Java: setEffectivelyFinalVars(Statement, Exprent, SSAUConstructorSparseEx, int, List, Map)
    #[allow(clippy::only_used_in_recursion)]
    fn set_effectively_final_vars_exprent(
        stat: Statement,
        exprent: Exprent,
        ssau: &SSAUConstructorSparseEx,
        index: i32,
        list: &JList<Exprent>,
        var_lookup_map: &mut JHashMap<VarVersionPair, VarExprent>,
    ) {
        if let Some(assign) = exprent.try_as_assignment() {
            if let Some(var) = assign.get_left().try_as_var() {
                var_lookup_map.put(var.get_var_version_pair(), var);
            }
        } else if let Some(new_expr) = exprent.try_as_new() {
            if new_expr.is_anonymous() {
                let node = match new_expr.get_new_type().value.clone() {
                    Some(value) => DecompilerContext::get_class_processor()
                        .get_map_root_classes()
                        .borrow()
                        .get(&value.to_string())
                        .copied(),
                    None => None,
                };

                if let Some(node) = node {
                    if !new_expr.is_lambda() {
                        let methods = node.with(|d| d.class_struct).get_methods().to_vec();
                        for mt in methods {
                            if mt.get_name() == INIT_NAME {
                                let param_types: Vec<VarType> =
                                    MethodDescriptor::parse_descriptor(&mt.get_descriptor())
                                        .params
                                        .clone();

                                let mut i = std::cmp::max(0, index - param_types.len() as i32);
                                while i < index {
                                    let temp = list.get(i as usize);
                                    if let Some(temp_assign) = temp.try_as_assignment() {
                                        let left = temp_assign.get_left();
                                        if let Some(left_var) = left.try_as_var() {
                                            if let Some(lvt) = left_var.get_lvt() {
                                                let lvt_type = lvt.get_var_type();
                                                if param_types.iter().any(|p| lvt_type.equals(p)) {
                                                    left_var.set_effectively_final(true);
                                                }
                                            }
                                        }
                                    }
                                    i += 1;
                                }
                                break;
                            }
                        }
                    } else if !new_expr.is_method_reference() {
                        let lambda_information = node
                            .with(|d| d.lambda_information.clone())
                            .expect("NullPointerException");
                        let md_lambda = MethodDescriptor::parse_descriptor(
                            &lambda_information.method_descriptor,
                        );
                        let md_content = MethodDescriptor::parse_descriptor(
                            &lambda_information.content_method_descriptor,
                        );
                        let param_offset: i32 = if lambda_information.is_content_method_static {
                            0
                        } else {
                            1
                        };
                        let vars_count =
                            md_content.params.len() as i32 - md_lambda.params.len() as i32;

                        for i in 0..vars_count {
                            let param = new_expr
                                .get_constructor()
                                .expect("NullPointerException")
                                .get_lst_parameters()
                                .get((param_offset + i) as usize);
                            if let Some(param_var) = param.try_as_var() {
                                let mut vvp = param_var.get_var_version_pair();
                                let Some(mut vvnode) =
                                    ssau.get_ssu_versions().get_node_with_key(vvp)
                                else {
                                    continue;
                                };

                                loop {
                                    let mut next: Option<VarVersionNode> = None;
                                    if vvnode.var() >= VarExprent::STACK_BASE {
                                        vvnode = vvnode
                                            .predecessors()
                                            .into_iter()
                                            .next()
                                            .expect("NoSuchElementException");
                                        let next_vvp = ssau.get_var_assignment_map().get_cloned(
                                            &VarVersionPair::new(vvnode.var(), vvnode.version()),
                                        );
                                        next = next_vvp.and_then(|p| {
                                            ssau.get_ssu_versions().get_node_with_key(p)
                                        });

                                        if let Some(next_vvp) = next_vvp {
                                            if next_vvp.var < 0 {
                                                vvp = next_vvp;
                                                break;
                                            }
                                        }
                                    } else {
                                        let j = i as usize;
                                        let var_index = vvnode.var();
                                        let var_version = vvnode.version();
                                        let roots = Self::get_roots(vvnode);
                                        let all_roots: Vec<VarVersionNode> =
                                            Self::distinct(ssau.get_ssu_versions().nodes())
                                                .into_iter()
                                                .filter(|n| {
                                                    n.var() == var_index
                                                        && n.with(|d| d.predecessors.is_empty())
                                                })
                                                .filter(|n| {
                                                    if let Some(lvt) = n.lvt() {
                                                        return md_content.params[j].equals(
                                                            &VarType::new_signature(
                                                                &lvt.get_descriptor(),
                                                            ),
                                                        );
                                                    }
                                                    n.version() > var_version
                                                })
                                                .collect();

                                        if roots.len() >= all_roots.len() {
                                            if roots.len() == 1 {
                                                vvnode = roots[0];
                                                vvp = VarVersionPair::new(
                                                    vvnode.var(),
                                                    vvnode.version(),
                                                );
                                                let next_vvp =
                                                    ssau.get_var_assignment_map().get_cloned(&vvp);
                                                next = next_vvp.and_then(|p| {
                                                    ssau.get_ssu_versions().get_node_with_key(p)
                                                });
                                                if let Some(next_vvp) = next_vvp {
                                                    if next_vvp.var < 0 {
                                                        vvp = next_vvp;
                                                        break;
                                                    }
                                                }
                                            } else if roots.len() == 2 {
                                                let first = roots[0];
                                                let second = roots[1];

                                                if let (Some(first_lvt), Some(second_lvt)) =
                                                    (first.lvt(), second.lvt())
                                                {
                                                    if first_lvt.get_version()
                                                        == second_lvt.get_version()
                                                    {
                                                        vvp = first_lvt.get_version();
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    match next {
                                        None => break,
                                        Some(next) => vvnode = next,
                                    }
                                }

                                if let Some(var) = var_lookup_map.get(&vvp).copied() {
                                    var.set_effectively_final(true);
                                }
                            }
                        }
                    }
                }
            }
        }

        for ex in exprent.get_all_exprents() {
            Self::set_effectively_final_vars_exprent(stat, ex, ssau, index, list, var_lookup_map);
        }
    }

    /// `stream().distinct()` over identity-hashed nodes: first occurrences, in order.
    fn distinct(nodes: Vec<VarVersionNode>) -> Vec<VarVersionNode> {
        let mut seen: JHashSet<VarVersionNode> = JHashSet::new();
        nodes.into_iter().filter(|n| seen.add(*n)).collect()
    }

    fn get_roots(vvnode: VarVersionNode) -> Vec<VarVersionNode> {
        let mut ret = Vec::new();
        let mut visited: JHashSet<VarVersionNode> = JHashSet::new();
        let mut queue: VecDeque<VarVersionNode> = VecDeque::new();

        queue.push_back(vvnode);
        visited.add(vvnode);

        while let Some(next) = queue.pop_front() {
            let predecessors = next.predecessors();
            if predecessors.is_empty() {
                ret.push(next);
            } else {
                for source in predecessors {
                    if visited.add(source) {
                        queue.push_back(source);
                    }
                }
            }
        }

        ret
    }
}
