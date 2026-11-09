//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IfPatternMatchProcessor.java`.
//!
//! kuna fix `pattern_variable_scope` (`kuna_fixes`): `check_branch` makes a
//! pattern only when the variable is used nowhere outside the pattern's scope
//! (`kuna_fix_pattern_variable_scope`).

use std::collections::VecDeque;

use crate::assignment_exprent::AssignmentExprent;
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::if_helper::IfHelper;
use crate::if_statement::IfStatement;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_pattern_variable_scope;
use crate::pattern_exprent::PatternExprent;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JList, JavaEq, JavaHash};
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_record_component::StructRecordComponent;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct IfPatternMatchProcessor;

impl IfPatternMatchProcessor {
    pub fn match_instanceof(root: RootStatement) -> bool {
        let res = Self::match_instanceof_rec(root.0, root);

        if res {
            ValidationHelper::validate_statement(root);

            SequenceHelper::condense_sequences(root.0);

            if IfHelper::merge_all_ifs(root) {
                Self::improve_pattern_types(root.0);
            } else {
                SequenceHelper::condense_sequences(root.0);
            }
        }

        res
    }

    fn match_instanceof_rec(statement: Statement, root: RootStatement) -> bool {
        let mut res = false;
        let stats = statement.get_stats().borrow().to_vec();
        for stat in stats {
            if Self::match_instanceof_rec(stat, root) {
                res = true;
            }
        }

        if let Some(if_stat) = statement.try_as_if() {
            res |= Self::handle_if(if_stat, root);
        }

        res
    }

    fn handle_if(statement: IfStatement, root: RootStatement) -> bool {
        let condition = statement.get_headexprent().get_condition();

        let last_if_true = Self::get_last_exprent_when(condition, true, true);
        let last_if_false = Self::get_last_exprent_when(condition, false, true);

        let mut updated = false;
        if let Some(last_if_true) = last_if_true {
            let destination = statement.get_if_edge().expect("ifEdge").get_destination();
            if Self::check_branch(last_if_true, statement, destination, root) {
                updated = true;

                statement.fix_if_invariant_empty_if_branch();
            }
        }

        if !updated {
            if let Some(last_if_false) = last_if_false {
                if let Some(else_edge) = statement.get_else_edge() {
                    if Self::check_branch(
                        last_if_false,
                        statement,
                        else_edge.get_destination(),
                        root,
                    ) {
                        updated = true;

                        statement.fix_if_invariant_empty_else_branch();
                    }
                } else {
                    let all_suc = statement.get_all_successor_edges();
                    if all_suc.len() == 1
                        && Self::check_branch(
                            last_if_false,
                            statement,
                            all_suc[0].get_destination(),
                            root,
                        )
                    {
                        updated = true;
                    }
                }
            }
        }

        updated
    }

    fn check_branch(
        exprent: Exprent,
        statement: IfStatement,
        branch: Statement,
        root: RootStatement,
    ) -> bool {
        if !exprent.is_function() || branch.get_all_predecessor_edges().len() != 1 {
            return false;
        }

        let iof = exprent.as_function();

        if iof.get_func_type() != FunctionType::Instanceof || iof.get_lst_operands().len() != 2 {
            return false;
        }

        let source = iof.get_lst_operands().get(0);
        if (source.get_exprent_use() & Exprent::MULTIPLE_USES) == 0 {
            return false;
        }

        let target = iof.get_lst_operands().get(1);

        let head = branch.get_basichead().0;

        let first = match head.get_exprents() {
            Some(exprents) if !exprents.is_empty() => exprents.get(0),
            _ => return false,
        };

        if !first.is_assignment() {
            return false;
        }

        let sides = first.get_all_exprents();
        let left = sides[0];
        let right = sides[1];

        let out_of_scope = kuna_fixes::enabled(KunaFix::PatternVariableScope)
            && left.try_as_var().is_some_and(|var| {
                !kuna_fix_pattern_variable_scope::in_scope(statement, branch, var, root.0)
            });
        let mut result = !out_of_scope
            && Self::find_pattern_matching_instanceof(left, right, source, target, branch, iof, head);

        if let Some(exprents) = head.get_exprents() {
            if !exprents.is_empty() {
                if let Some(assignment) = exprents.get(0).try_as_assignment() {
                    let right = assignment.get_all_exprents()[1];

                    if !right.is_function() {
                        result |= Self::identify_if_record_pattern_match(
                            statement, branch, iof, assignment,
                        );
                    }
                }
            }
        }

        if result {
            statement.set_pattern_matched(true);

            let before = statement.get_basichead();
            if let Some(before_exprents) = before.get_exprents() {
                if !before_exprents.is_empty() {
                    let last = before_exprents.get(before_exprents.len() - 1);
                    if let (Some(assign), Some(checked)) =
                        (last.try_as_assignment(), source.try_as_var())
                    {
                        let stored = assign.get_left();
                        let method = assign.get_right();
                        if (!method.is_function()
                            || method.as_function().get_func_type() != FunctionType::Cast)
                            && checked.equals(stored)
                            && !checked.is_var_referenced(root.0, &[stored.as_var()])
                        {
                            iof.get_lst_operands().set(0, assign.get_right());
                            before_exprents.remove_at(before_exprents.len() - 1);
                        }
                    }
                }
            }
        }

        result
    }

    fn find_pattern_matching_instanceof(
        left: Exprent,
        right: Exprent,
        source: Exprent,
        target: Exprent,
        branch: Statement,
        iof: FunctionExprent,
        head: Statement,
    ) -> bool {
        let casted = if let Some(function) = right
            .try_as_function()
            .filter(|f| f.get_func_type() == FunctionType::Cast)
        {
            function.get_lst_operands().get(0)
        } else if let Some(variable) = right.try_as_var() {
            variable.0
        } else {
            return false;
        };

        if !source.equals(casted) {
            return false;
        }

        if !left.is_var() || !target.get_expr_type().equals(&left.get_expr_type()) {
            return false;
        }

        let mut vvs: Vec<VarVersionPair> = Vec::new();

        Self::find_vars_in_predecessors(&mut vvs, branch);

        let var = left.as_var().get_var_version_pair();

        for vv in &vvs {
            if var.var == vv.var {
                return false;
            }
        }

        let store_type = left.get_inferred_expr_type(None);

        iof.get_lst_operands().add_at(2, left);
        head.get_exprents().expect("exprents").remove_at(0);
        if store_type.is_generic() {
            let bytecode = iof.get_lst_operands().get(1).get_bytecode();
            iof.get_lst_operands()
                .set(1, ConstExprent::new(store_type, None, bytecode.as_ref()).0);
        }
        true
    }

    fn find_vars_in_predecessors(vvs: &mut Vec<VarVersionPair>, root: Statement) {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        let mut seen: JHashSet<Statement> = JHashSet::new();

        stack.push_back(root);

        while let Some(st) = stack.pop_front() {
            if !seen.add(st) {
                continue;
            }

            if st.get_parent().is_some_and(|p| p.is_if()) || st.is_if() {
                stack.push_back(st.get_parent().expect("NullPointerException: parent"));
            }

            for pred in st.get_all_predecessor_edges() {
                let stat = pred.get_source();
                stack.push_back(stat);
                if root.contains_statement(stat) {
                    continue;
                }

                if let Some(exprents) = stat.get_exprents() {
                    for exprent in exprents.iter() {
                        if let Some(assignment) = exprent.try_as_assignment() {
                            if let Some(left) = assignment.get_left().try_as_var() {
                                vvs.push(left.get_var_version_pair());
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_if_record_pattern_match(
        stat: IfStatement,
        branch: Statement,
        inst_of: FunctionExprent,
        head: AssignmentExprent,
    ) -> bool {
        if !stat
            .get_top_parent()
            .mt()
            .get_bytecode_version()
            .has_record_pattern_matching()
        {
            return false;
        }

        let head_right = head.get_right();

        let operands = inst_of.get_lst_operands();
        let checked = if operands.len() > 2 {
            operands.get(2)
        } else {
            operands.get(0)
        };
        if !checked.equals(head_right) {
            return false;
        }

        let type_ = inst_of.get_lst_operands().get(1).get_expr_type();

        let Some(exprent) =
            Self::identify_record_pattern_match(stat.0, branch, head_right, &type_, false)
        else {
            return false;
        };

        if inst_of.get_lst_operands().len() > 2 {
            inst_of.get_lst_operands().set(2, exprent.0);
        } else {
            inst_of.get_lst_operands().add_at(2, exprent.0);
        }

        stat.set_pattern_matched(true);
        true
    }

    pub fn identify_record_pattern_match(
        parent: Statement,
        branch: Statement,
        store_variable: Exprent,
        type_: &VarType,
        simulate: bool,
    ) -> Option<PatternExprent> {
        let cl = type_
            .value
            .as_deref()
            .and_then(|name| DecompilerContext::get_struct_context().get_class(name))?;

        match cl.get_record_components() {
            Some(components) if !components.is_empty() => {}
            _ => return None,
        }

        let mut remove: JHashMap<Statement, Vec<Exprent>> = JHashMap::new();
        let mut to_destroy: Vec<Statement> = Vec::new();
        let mut remove_edge: Vec<StatEdge> = Vec::new();

        let pattern = Self::get_child_pattern(
            Some(cl),
            store_variable,
            type_,
            branch,
            1,
            &mut to_destroy,
            &mut remove,
            &mut remove_edge,
        )?;

        if simulate {
            return Some(pattern.exp);
        }

        if branch != pattern.stat {
            parent.replace_statement(branch, pattern.stat);
            for e in pattern.stat.get_successor_edges(StatEdge::TYPE_BREAK) {
                e.change_closure(Some(parent));
            }
        }

        for st in to_destroy {
            st.replace_with_empty();
        }

        for (block, exprents) in remove.entry_vec() {
            remove_all(&block.get_exprents().expect("exprents"), &exprents);
        }

        for edge in remove_edge {
            edge.remove();
        }

        Some(pattern.exp)
    }

    /// `remove` is keyed by the `BasicBlockStatement` (identity), held as its base handle.
    #[allow(clippy::too_many_arguments)]
    fn get_child_pattern(
        cl: Option<StructClass>,
        store_variable: Exprent,
        type_: &VarType,
        mut branch: Statement,
        mut st_idx: i32,
        to_destroy: &mut Vec<Statement>,
        remove: &mut JHashMap<Statement, Vec<Exprent>>,
        remove_edge: &mut Vec<StatEdge>,
    ) -> Option<PatternData> {
        let cl = cl?;
        cl.get_record_components()?;

        let original = branch;

        let mut pattern_stores: Vec<PatternStore> = Vec::new();
        let comp = cl.get_record_components().expect("record components");

        let mut vars: JLinkedHashMap<StructRecordComponent, Exprent> = JLinkedHashMap::new();
        for c in comp {
            if stat_count(branch) <= st_idx {
                return None;
            }

            let mut next = stat_at(branch, st_idx);
            let catch_st = next.try_as_catch().filter(|catch_st| {
                catch_st.get_vars().len() == 1
                    && catch_st.get_vars().get(0).get_var_type().value_str()
                        == "java/lang/Throwable"
            });
            let catch_st = catch_st?;

            let mut found_var: Option<VarExprent> = None;
            if stat_count(catch_st.0) == 2 && Self::is_statement_match_throw(stat_at(catch_st.0, 1))
            {
                let inner = stat_at(catch_st.0, 0);
                if inner.is_basic_block() {
                    let inner_exprents = inner.get_exprents().expect("exprents");
                    if inner_exprents.len() == 1 {
                        if let Some(assign) = inner_exprents.get(0).try_as_assignment() {
                            if let (Some(var), Some(invok)) = (
                                assign.get_left().try_as_var(),
                                assign.get_right().try_as_invocation(),
                            ) {
                                if var.is_stack()
                                    && type_.value.as_deref()
                                        == Some(invok.get_classname().as_str())
                                    && invok.get_name() == c.get_name()
                                {
                                    found_var = Some(var);
                                }
                            }
                        }
                    }
                }
            }

            let found_var = found_var?;

            to_destroy.push(next);

            st_idx += 1;
            if stat_count(branch) > st_idx {
                next = stat_at(branch, st_idx);

                let mut ok = false;
                let head = next.get_basichead();
                let head_exprents = head.get_exprents().expect("exprents");
                if head_exprents.len() >= 2 {
                    if let Some(assign) = head_exprents.get(0).try_as_assignment() {
                        if let Some(ex_var) = assign.get_left().try_as_var() {
                            if assign.get_right().equals(found_var.0) {
                                if let Some(real_assign) = head_exprents.get(1).try_as_assignment()
                                {
                                    if let Some(real_var) = real_assign.get_left().try_as_var() {
                                        if ex_var.equals(real_assign.get_right())
                                            && !ex_var.is_var_referenced(
                                                branch,
                                                &[real_assign.get_right().as_var()],
                                            )
                                        {
                                            vars.put(c, real_var.0);

                                            ok = true;

                                            let mut destroyed = false;
                                            if head_exprents.len() == 3 {
                                                if let Some(n_assign) =
                                                    head_exprents.get(2).try_as_assignment()
                                                {
                                                    if n_assign.get_right().equals(store_variable) {
                                                        to_destroy.push(head.0);

                                                        destroyed = true;

                                                        for e in head.get_successor_edges(
                                                            StatEdge::TYPE_BREAK,
                                                        ) {
                                                            e.remove();
                                                        }
                                                    }
                                                }
                                            }

                                            if !destroyed {
                                                let to_remove =
                                                    remove.compute_if_absent(head.0, Vec::new);
                                                to_remove.push(assign.0);
                                                to_remove.push(real_assign.0);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(if_st) = next
                    .try_as_if()
                    .filter(|if_st| if_st.iftype() == IfStatement::IFTYPE_IF)
                {
                    let mut found = false;
                    let mut inverted = false;
                    let condition = if_st.get_headexprent().get_condition();
                    if let Some(func) = condition.try_as_function() {
                        let mut function: Option<FunctionExprent> = None;
                        if func.get_func_type() == FunctionType::Instanceof {
                            found = true;
                            function = Some(func);
                        } else if func.get_func_type() == FunctionType::BoolNot {
                            if let Some(inner) = func.get_lst_operands().get(0).try_as_function() {
                                if inner.get_func_type() == FunctionType::Instanceof {
                                    found = true;
                                    inverted = true;
                                    function = Some(inner);
                                }
                            }
                        }

                        if found {
                            let branch_head = branch.get_basichead();
                            let branch_head_exprents =
                                branch_head.get_exprents().expect("exprents");
                            if branch_head_exprents.len() == 1 {
                                if let Some(assign) =
                                    branch_head_exprents.get(0).try_as_assignment()
                                {
                                    if assign.get_left().is_var() && assign.get_right().is_var() {
                                        to_destroy.push(branch.get_basichead().0);
                                    }
                                }
                            }

                            let function = function.expect("function");
                            let operands = function.get_lst_operands();
                            let store = if operands.len() > 2 {
                                operands.get(2)
                            } else {
                                operands.get(0)
                            };
                            if let Some(variable) = store.try_as_var() {
                                let store_cl =
                                    variable.get_expr_type().value.as_deref().and_then(|name| {
                                        DecompilerContext::get_struct_context().get_class(name)
                                    });
                                pattern_stores.push(PatternStore::new(
                                    c,
                                    store_cl,
                                    &variable.get_expr_type(),
                                    variable,
                                ));
                                vars.put(c, variable.0);
                                ok = true;
                            } else if let Some(pattern) = store.try_as_pattern() {
                                vars.put(c, pattern.0);
                                ok = true;
                            }
                        }
                    } else if let Some(const_exp) =
                        condition.try_as_const().filter(|k| k.has_boolean_value())
                    {
                        let mut stat: Option<Statement> = None;
                        let mut exprents: Option<JList<Exprent>> = None;
                        if const_exp.get_int_value() == 1 {
                            let block = if_st.get_ifstat().expect("ifstat").get_basichead();
                            exprents = block.get_exprents();
                            stat = Some(block.0);
                        } else if st_idx + 1 < stat_count(branch) {
                            let block = stat_at(branch, st_idx + 1).get_basichead();
                            exprents = block.get_exprents();
                            stat = Some(block.0);
                            inverted = true;
                        }

                        if let Some(stat) = stat {
                            let exprents = exprents.expect("exprents");
                            if !exprents.is_empty() {
                                if let Some(assignment) = exprents.get(0).try_as_assignment() {
                                    if let Some(store) = assignment.get_left().try_as_var() {
                                        if assignment.get_right().is_var() {
                                            found = true;
                                            vars.put(c, store.0);
                                            ok = true;
                                            let to_remove =
                                                remove.compute_if_absent(stat, Vec::new);
                                            to_remove.push(assignment.0);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if found {
                        to_destroy.push(if_st.0);

                        if inverted {
                            st_idx += 1;
                        } else {
                            branch = if_st.get_ifstat().expect("ifstat");
                            st_idx = 0;
                        }
                    }
                }

                if !ok {
                    vars.put(c, found_var.0);
                }

                st_idx += 1;
            }
        }

        for st in to_destroy.clone() {
            if st.is_catch() {
                remove_edge.extend(stat_at(st, 0).get_successor_edges(StatEdge::TYPE_BREAK));
                remove_edge.extend(stat_at(st, 1).get_successor_edges(StatEdge::TYPE_BREAK));
            } else if st.is_if() {
                remove_edge.extend(st.get_successor_edges(StatEdge::TYPE_BREAK));
            }
        }

        if branch != original {
            if let Some(if_st) = branch.get_parent().and_then(|p| p.try_as_if()) {
                remove_edge.extend(if_st.get_successor_edges(StatEdge::TYPE_BREAK));
                remove_edge.extend(
                    if_st
                        .get_first()
                        .expect("first")
                        .get_successor_edges(StatEdge::TYPE_REGULAR),
                );
            }
        }

        for pattern_store in &pattern_stores {
            let mut tmp_to_destroy: Vec<Statement> = Vec::new();
            let mut tmp_remove: JHashMap<Statement, Vec<Exprent>> = JHashMap::new();
            let mut tmp_remove_edge: Vec<StatEdge> = Vec::new();
            let pattern_data = Self::get_child_pattern(
                pattern_store.cl,
                pattern_store.store.0,
                &pattern_store.type_,
                branch,
                st_idx,
                &mut tmp_to_destroy,
                &mut tmp_remove,
                &mut tmp_remove_edge,
            );
            if let Some(pattern_data) = pattern_data {
                vars.put(pattern_store.component, pattern_data.exp.0);
                branch = pattern_data.stat;
                st_idx = pattern_data.index;
                to_destroy.extend(tmp_to_destroy);
                remove.put_all(tmp_remove.entry_vec());
                remove_edge.extend(tmp_remove_edge);
            }
        }

        let pattern = PatternExprent::new(
            PatternExprent::record_data(cl),
            type_.clone(),
            vars.value_vec(),
        );
        Some(PatternData::new(pattern, branch, st_idx))
    }

    pub fn is_statement_match_throw(st: Statement) -> bool {
        if st.is_basic_block() && st.get_exprents().unwrap().len() == 1 {
            if let Some(exit) = st.get_exprents().unwrap().get(0).try_as_exit() {
                if exit.get_exit_type() == ExitType::Throw {
                    if let Some(new_ex) = exit.get_value().and_then(|v| v.try_as_new()) {
                        return new_ex.get_new_type().value_str() == "java/lang/MatchException";
                    }
                }
            }
        }

        false
    }

    pub fn get_last_exprent_when(
        base: Exprent,
        if_true: bool,
        only_if_true: bool,
    ) -> Option<Exprent> {
        if base.get_type() == ExprentType::Function {
            let func = base.as_function();
            match func.get_func_type() {
                FunctionType::BooleanAnd => {
                    if if_true {
                        return Self::get_last_exprent_when(
                            func.get_lst_operands().get(1),
                            true,
                            only_if_true,
                        );
                    }
                }
                FunctionType::BooleanOr => {
                    if !if_true {
                        return Self::get_last_exprent_when(
                            func.get_lst_operands().get(1),
                            false,
                            only_if_true,
                        );
                    }
                }
                FunctionType::BoolNot => {
                    return Self::get_last_exprent_when(
                        func.get_lst_operands().get(0),
                        !if_true,
                        only_if_true,
                    );
                }
                FunctionType::Eq => {
                    let rhs = func.get_lst_operands().get(1);
                    if rhs.get_type() == ExprentType::Const {
                        let const_exprent = rhs.as_const();
                        if const_exprent
                            .get_const_type()
                            .ptr_eq(&VarType::VARTYPE_BOOLEAN())
                        {
                            if const_exprent.get_int_value() == 0 {
                                return Self::get_last_exprent_when(
                                    func.get_lst_operands().get(0),
                                    !if_true,
                                    only_if_true,
                                );
                            } else {
                                return Self::get_last_exprent_when(
                                    func.get_lst_operands().get(0),
                                    if_true,
                                    only_if_true,
                                );
                            }
                        }
                    }
                }
                FunctionType::Ne => {
                    let rhs = func.get_lst_operands().get(1);
                    if rhs.get_type() == ExprentType::Const {
                        let const_exprent = rhs.as_const();
                        if const_exprent
                            .get_const_type()
                            .ptr_eq(&VarType::VARTYPE_BOOLEAN())
                        {
                            if const_exprent.get_int_value() == 0 {
                                return Self::get_last_exprent_when(
                                    func.get_lst_operands().get(0),
                                    if_true,
                                    only_if_true,
                                );
                            } else {
                                return Self::get_last_exprent_when(
                                    func.get_lst_operands().get(0),
                                    !if_true,
                                    only_if_true,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if only_if_true && !if_true {
            return None;
        }

        Some(base)
    }

    fn improve_pattern_types(stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::improve_pattern_types(st);
        }

        if let Some(if_st) = stat.try_as_if() {
            let cond = if_st.get_headexprent().get_condition();

            if Self::improve_pattern_type(if_st.get_headexprent().0, cond, if_st.get_ifstat()) {
                res = true;
            }
        }

        res
    }

    fn improve_pattern_type(parent: Exprent, ex: Exprent, st: Option<Statement>) -> bool {
        let mut res = false;
        for e in ex.get_all_exprents_rec_self(false, true) {
            if e != ex {
                res |= Self::improve_pattern_type(ex, e, st);
            }

            let Some(fn_) = e
                .try_as_function()
                .filter(|f| f.get_func_type() == FunctionType::BooleanAnd)
            else {
                continue;
            };
            let base = fn_.get_lst_operands().get(0);

            let Some(base_fn) = base.try_as_function() else {
                continue;
            };
            if base_fn.get_func_type() != FunctionType::Instanceof
                || base_fn.get_lst_operands().len() <= 2
            {
                continue;
            }
            let Some(pattern) = base_fn.get_lst_operands().get(2).try_as_pattern() else {
                continue;
            };
            if !matches!(
                pattern.get_data(),
                crate::pattern_exprent::PatternData::RecordPatternData(_)
            ) {
                continue;
            }

            let mut vars: Vec<Option<VarExprent>> = Vec::new();

            for pattern_ex in pattern.get_exprents().iter() {
                vars.push(pattern_ex.try_as_var());
            }

            for (j, var) in vars.iter().enumerate() {
                let Some(var) = *var else {
                    continue;
                };

                if var.is_var_referenced(st.expect("NullPointerException: statement"), &[]) {
                    continue;
                }

                'out: for exp in parent.get_all_exprents_rec_self(true, true) {
                    for inst in exp.get_all_exprents() {
                        let Some(inst_fun) = inst
                            .try_as_function()
                            .filter(|f| f.get_func_type() == FunctionType::BooleanAnd)
                        else {
                            continue;
                        };
                        for i in 0..2 {
                            let inner = inst_fun.get_lst_operands().get(i);
                            if let Some(inner_fun) = inner.try_as_function() {
                                if inner_fun.get_func_type() == FunctionType::Instanceof
                                    && inner_fun.get_lst_operands().len() > 2
                                    && inner_fun.get_lst_operands().get(0).equals(var.0)
                                {
                                    pattern
                                        .get_exprents()
                                        .set(j, inner_fun.get_lst_operands().get(2));
                                    pattern.get_bound_types().set(
                                        j,
                                        Some(inner_fun.get_lst_operands().get(1).get_expr_type()),
                                    );

                                    exp.replace_exprent(
                                        inst,
                                        inst_fun.get_lst_operands().get(i ^ 1),
                                    );

                                    break 'out;
                                }
                            }
                        }
                    }
                }
            }
        }

        res
    }
}

fn stat_count(stat: Statement) -> i32 {
    stat.get_stats().borrow().len() as i32
}

fn stat_at(stat: Statement, index: i32) -> Statement {
    let stats = stat.get_stats();
    let stats = stats.borrow();
    stats.get(index as usize)
}

/// `list.removeAll(c)`: drops every element `e` of `list` for which some `x` in `c` has `e.equals(x)`.
fn remove_all(list: &JList<Exprent>, c: &[Exprent]) -> bool {
    let current = list.to_vec();
    let kept: Vec<Exprent> = current
        .iter()
        .copied()
        .filter(|e| !c.iter().any(|x| e.equals(*x)))
        .collect();
    let modified = kept.len() != current.len();
    if modified {
        list.set_all(kept);
    }
    modified
}

/// `record PatternData(PatternExprent exp, Statement stat, int index)`.
#[derive(Clone, Copy, Debug)]
pub struct PatternData {
    exp: PatternExprent,
    stat: Statement,
    index: i32,
}

impl PatternData {
    fn new(exp: PatternExprent, stat: Statement, index: i32) -> PatternData {
        PatternData { exp, stat, index }
    }

    pub fn exp(&self) -> PatternExprent {
        self.exp
    }

    pub fn stat(&self) -> Statement {
        self.stat
    }

    pub fn index(&self) -> i32 {
        self.index
    }
}

impl JavaEq for PatternData {
    fn java_equals(&self, other: &Self) -> bool {
        self.exp.0.java_equals(&other.exp.0) && self.stat == other.stat && self.index == other.index
    }
}

impl JavaHash for PatternData {
    fn java_hash(&self) -> i32 {
        record_hash(&[self.exp.0.java_hash(), self.stat.java_hash(), self.index])
    }
}

/// The local `record PatternStore(StructRecordComponent component, StructClass cl, VarType type, VarExprent store)`
/// of `getChildPattern`; `cl` comes from `StructContext.getClass` and may be null.
#[derive(Clone)]
pub struct PatternStore {
    component: StructRecordComponent,
    cl: Option<StructClass>,
    type_: VarType,
    store: VarExprent,
}

impl PatternStore {
    pub fn new(
        component: StructRecordComponent,
        cl: Option<StructClass>,
        type_: &VarType,
        store: VarExprent,
    ) -> PatternStore {
        PatternStore {
            component,
            cl,
            type_: type_.clone(),
            store,
        }
    }

    pub fn component(&self) -> StructRecordComponent {
        self.component
    }

    pub fn cl(&self) -> Option<StructClass> {
        self.cl
    }

    pub fn type_(&self) -> VarType {
        self.type_.clone()
    }

    pub fn store(&self) -> VarExprent {
        self.store
    }
}

impl JavaEq for PatternStore {
    fn java_equals(&self, other: &Self) -> bool {
        self.component == other.component
            && self.cl == other.cl
            && self.type_.java_equals(&other.type_)
            && self.store.0.java_equals(&other.store.0)
    }
}

impl JavaHash for PatternStore {
    fn java_hash(&self) -> i32 {
        record_hash(&[
            self.component.java_hash(),
            self.cl.java_hash(),
            self.type_.java_hash(),
            self.store.0.java_hash(),
        ])
    }
}

/// `Record.hashCode()` as the JDK generates it: `31 * h + hash(c)` over the components, from 0.
fn record_hash(components: &[i32]) -> i32 {
    components
        .iter()
        .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(*c))
}
