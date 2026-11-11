//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchHelper.java`.
//!
//! The sealed `StringSwitch` interface is the closed enum [`StringSwitch`] over its four
//! permitted records; `instanceof Merged`/`NullableSplit` checks become matches on it.

use std::collections::VecDeque;

use crate::array_exprent::ArrayExprent;
use crate::assignment_exprent::AssignmentExprent;
use crate::code_constants::{ACC_ENUM, ACC_FINAL, ACC_STATIC, ACC_SYNTHETIC, CLINIT_NAME};
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::i_fernflower_logger::Severity;
use crate::if_pattern_match_processor::IfPatternMatchProcessor;
use crate::if_statement::IfStatement;
use crate::jthrow;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JList, JavaEq, JavaHash, Shared};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::switch_statement::SwitchStatement;
use crate::type_family::TypeFamily;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

pub struct SwitchHelper;

impl SwitchHelper {
    pub const STATIC_FINAL_SYNTHETIC: i32 = ACC_STATIC | ACC_FINAL | ACC_SYNTHETIC;

    pub fn simplify_switches(stat: Statement, mt: StructMethod, root: RootStatement) -> bool {
        let mut ret = false;
        if let Some(switch_stat) = stat.try_as_switch() {
            ret = Self::simplify(switch_stat, mt, root);
        }

        let mut i = 0;
        while i < stat.get_stats().borrow().len() {
            let st = stat.get_stats().borrow().get(i);
            ret |= Self::simplify_switches(st, mt, root);
            i += 1;
        }

        ret
    }

    fn simplify(switch_statement: SwitchStatement, mt: StructMethod, root: RootStatement) -> bool {
        if Self::simplify_switch_on_enum_j21(switch_statement, root) {
            return true;
        }

        let switch_head_exprent = switch_statement.get_headexprent().as_switch_head();
        let value = switch_head_exprent.get_value();
        let array = Self::get_enum_array_exprent(value, root);
        if let Some(array) = array {
            let case_values = switch_statement.get_case_values();
            let mut mapping: JHashMap<Exprent, Option<Exprent>> =
                JHashMap::with_capacity(case_values.len());
            if let Some(array_field) = array.get_array().try_as_field() {
                let class_node = DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get_cloned(&array_field.get_classname());
                if let Some(class_node) = class_node {
                    if let Some(class_wrapper) = class_node.get_wrapper() {
                        let wrapper = class_wrapper.get_method_wrapper(CLINIT_NAME, "()V");
                        if let Some(wrapper) = wrapper {
                            if let Some(wrapper_root) = wrapper.with(|d| d.root) {
                                let mut field_assignments =
                                    Self::get_assignments_of_within_one_statement(
                                        wrapper_root.0,
                                        array_field.0,
                                    );
                                if field_assignments.len() > 1 {
                                    field_assignments.clear();
                                }

                                let mut field_assignment_encountered = false;

                                wrapper
                                    .get_or_build_graph()
                                    .expect("NullPointerException: graph")
                                    .iterate_exprents(|exprent: Exprent| {
                                        if let Some(assignment) = exprent.try_as_assignment() {
                                            let left = assignment.get_left();
                                            if let Some(left_array) = left.try_as_array() {
                                                let assignment_array = left_array.get_array();
                                                let mut targets_field =
                                                    assignment_array.equals(array_field.0);

                                                if !targets_field
                                                    && assignment_array.is_var()
                                                    && !field_assignment_encountered
                                                {
                                                    for field_assignment in &field_assignments {
                                                        if field_assignment
                                                            .get_right()
                                                            .equals(assignment_array)
                                                        {
                                                            targets_field = true;
                                                            break;
                                                        }
                                                    }
                                                }

                                                if targets_field
                                                    && left_array.get_index().is_invocation()
                                                {
                                                    mapping.put(
                                                        assignment.get_right(),
                                                        left_array
                                                            .get_index()
                                                            .as_invocation()
                                                            .get_instance(),
                                                    );
                                                }
                                            } else if field_assignments
                                                .iter()
                                                .any(|fa| exprent.equals(fa.0))
                                            {
                                                field_assignment_encountered = true;
                                            }
                                        }
                                        0
                                    });
                            }
                        }
                    }
                }
            } else {
                let invocation = array.get_array().as_invocation();
                let class_node = DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get_cloned(&invocation.get_classname());
                if let Some(class_node) = class_node {
                    if let Some(class_wrapper) = class_node.get_wrapper() {
                        let wrapper =
                            class_wrapper.get_method_wrapper(&invocation.get_name(), "()[I");
                        if let Some(wrapper) = wrapper {
                            if wrapper.with(|d| d.root.is_some()) {
                                wrapper
                                    .get_or_build_graph()
                                    .expect("NullPointerException: graph")
                                    .iterate_exprents(|exprent: Exprent| {
                                        if let Some(assignment) = exprent.try_as_assignment() {
                                            let left = assignment.get_left();
                                            if let Some(left_array) = left.try_as_array() {
                                                mapping.put(
                                                    assignment.get_right(),
                                                    left_array
                                                        .get_index()
                                                        .as_invocation()
                                                        .get_instance(),
                                                );
                                            }
                                        }
                                        0
                                    });
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }

            let mut nullable = false;
            let mut real_case_values: Vec<JList<Option<Exprent>>> =
                Vec::with_capacity(case_values.len());
            for case_value in case_values.iter() {
                let values: JList<Option<Exprent>> =
                    JList::from(Vec::with_capacity(case_value.len()));
                real_case_values.push(values.clone());
                'cases: for exprent in case_value.iter() {
                    let Some(exprent) = exprent else {
                        values.add(None);
                        continue;
                    };

                    let Some(real_const) = mapping.get(&exprent).copied().flatten() else {
                        if let Some(const_label) = exprent.try_as_const() {
                            if const_label.get_const_type().type_family == TypeFamily::Integer {
                                let int_label = const_label.get_int_value();
                                if int_label == -1 {
                                    values.add(Some(
                                        ConstExprent::new(VarType::VARTYPE_NULL(), None, None).0,
                                    ));
                                    nullable = true;
                                    continue;
                                }
                                for key in mapping.key_vec() {
                                    if key.try_as_const().is_some_and(|k| {
                                        k.get_const_type().type_family == TypeFamily::Integer
                                            && k.get_int_value() > int_label
                                    }) {
                                        values.add(Some(key.copy()));
                                        continue 'cases;
                                    }
                                }
                            }
                        }
                        root.add_comment_error("$VF: Unable to simplify switch on enum", true);
                        let message = format!(
                            "Unable to simplify switch on enum: {} not found, available: {} in method {} {}",
                            exprent.to_string(),
                            enum_mapping_to_string(&mapping),
                            mt.get_class_qualified_name(),
                            mt.get_name()
                        );
                        DecompilerContext::get_logger().write_message(&message, Severity::Error);
                        return false;
                    };
                    values.add(Some(real_const.copy()));
                }
            }
            case_values.set_all(real_case_values);
            let new_expr = array
                .get_index()
                .as_invocation()
                .get_instance()
                .expect("NullPointerException: instance")
                .copy();
            switch_head_exprent.replace_exprent(value, new_expr);
            new_expr.add_bytecode_offsets(value.get_bytecode().as_ref());

            if let Some(value_var) = value.try_as_var() {
                let mut references: Vec<Pair<Statement, Exprent>> = Vec::new();
                Self::find_exprents(
                    root.0,
                    Some,
                    |e| value_var.is_var_referenced_exp(e, &[]),
                    false,
                    |stat, expr| references.push(Pair::of(stat, expr)),
                );

                if references.len() == 1 {
                    let ref_ = &references[0];
                    if ref_
                        .b
                        .try_as_assignment()
                        .is_some_and(|a| a.get_left().equals(value))
                    {
                        ref_.a
                            .get_exprents()
                            .expect("NullPointerException: exprents")
                            .remove_item(&ref_.b);
                    }
                }
            }

            let head = switch_statement.get_basichead();
            if nullable {
                let head_exprents = head.get_exprents().expect("NullPointerException: exprents");
                if !head_exprents.is_empty() {
                    let last = head_exprents.get(head_exprents.len() - 1);
                    if let Some(assignment) = last.try_as_assignment() {
                        if let Some(temp_var) = assignment.get_left().try_as_var() {
                            if let Some(used_var) = switch_head_exprent.get_value().try_as_var() {
                                if temp_var.equals_versions(used_var.0)
                                    && !temp_var.is_var_referenced(root.0, &[used_var])
                                {
                                    head_exprents.remove_at(head_exprents.len() - 1);
                                    switch_head_exprent.set_value(assignment.get_right());
                                }
                            }
                        }
                    }
                }
            }

            return true;
        }

        Self::try_simplify_string_switch(switch_statement, value)
    }

    fn try_simplify_string_switch(
        switch_stat: SwitchStatement,
        switch_head_value: Exprent,
    ) -> bool {
        let Some(switch_info) = StringSwitch::match_(switch_stat) else {
            return false;
        };

        let target_val = switch_info
            .target()
            .get_headexprent()
            .as_switch_head()
            .get_value();
        switch_info.target().get_headexprent().replace_exprent(
            target_val,
            switch_head_value
                .as_invocation()
                .get_instance()
                .expect("NullPointerException: instance"),
        );

        if matches!(switch_info, StringSwitch::Merged(_)) {
            if !Self::unwrap_merged_switch_cases(switch_info) {
                return false;
            }

            Self::merge_duplicate_case_stats(switch_info);
            Self::remove_synthetic_dup_var(switch_info);
            return true;
        }

        let case_map = Self::get_string_switch_case_map(switch_info);
        let real_case_values = Self::get_string_switch_real_case_values(switch_info, &case_map);
        switch_info.target().get_case_values().clear();
        switch_info
            .target()
            .get_case_values()
            .add_all(real_case_values);

        let first_basic = switch_stat.get_basichead();
        let first_exprs = first_basic.get_exprents();
        if let Some(first_exprs) = &first_exprs {
            if !first_exprs.is_empty() {
                first_exprs.remove_at(first_exprs.len() - 1);
            }
        }

        for edge in first_basic.get_all_predecessor_edges() {
            first_basic.remove_predecessor(edge);
        }
        for edge in first_basic.get_all_successor_edges() {
            first_basic.remove_successor(edge);
        }

        let target_exprs = switch_info.target().get_basichead().get_exprents();
        if let (Some(target_exprs), Some(first_exprs)) = (&target_exprs, &first_exprs) {
            target_exprs.add_all(first_exprs.to_vec());
            first_exprs.clear();
        }

        let mut replacement = first_basic.0;
        if matches!(switch_info, StringSwitch::InlineSplit(_)) {
            replacement = match switch_info.target().get_parent() {
                Some(seq_stat) if seq_stat.is_sequence() => seq_stat,
                _ => switch_info.target().0,
            };
        }

        for edge in replacement.get_all_predecessor_edges() {
            replacement.remove_predecessor(edge);
        }
        switch_stat.replace_with(replacement);

        if let StringSwitch::NullableSplit(si) = switch_info {
            let replaced = si.null_check().replace_with_empty();
            hash_set_for_each(&replaced.get_label_edges(), |e| {
                switch_info.target().remove_predecessor(e);
                e.remove_closure();
            });
        }

        for e in switch_info.target().get_all_predecessor_edges() {
            if switch_stat.contains_statement(e.get_source()) && e.get_source() != first_basic.0 {
                e.get_source().remove_successor(e);
            }
        }

        Self::remove_synthetic_dup_var(switch_info);

        true
    }

    fn unwrap_merged_switch_cases(switch_info: StringSwitch) -> bool {
        let mut processed_cases_idx = 0;

        let case_stats = switch_info.first().get_case_statements();
        let mut i = 0;
        while i < case_stats.len() {
            let stat = case_stats.get(i);
            i += 1;
            let Some(root_if_stat) = stat.try_as_if() else {
                continue;
            };

            let exprs = root_if_stat
                .get_ifstat()
                .expect("NullPointerException: ifstat")
                .get_exprents();
            let Some(exprs) = exprs.filter(|e| e.len() == 1) else {
                continue;
            };

            let is_return = exprs
                .get(0)
                .try_as_exit()
                .is_some_and(|exit_expr| exit_expr.get_exit_type() == ExitType::Return);
            if !is_return {
                continue;
            }

            let mut curr_if = Some(root_if_stat);

            switch_info.first().remove_case_at(processed_cases_idx);

            if root_if_stat.has_successor(StatEdge::TYPE_REGULAR) {
                root_if_stat.get_successor_edges(StatEdge::TYPE_REGULAR)[0].remove();
            }

            while let Some(if_stat) = curr_if {
                let else_stat = if_stat.get_elsestat().map(|s| s.as_if());

                if if_stat.has_successor(StatEdge::TYPE_BREAK) {
                    if_stat.get_successor_edges(StatEdge::TYPE_BREAK)[0].remove();
                }

                for edge in if_stat.get_basichead().get_all_successor_edges() {
                    edge.remove();
                }

                let cases: JList<Option<Exprent>> = JList::new();
                let if_cond = if_stat.get_headexprent().get_condition();
                if let Some(cond_func) = if_cond.try_as_function() {
                    for oper in cond_func.get_lst_operands().iter() {
                        if let Some(cond_invoc) = oper.try_as_invocation() {
                            cases.add(Some(cond_invoc.get_lst_parameters().get(0)));
                        }
                    }
                } else if let Some(cond_invoc) = if_cond.try_as_invocation() {
                    cases.add(Some(cond_invoc.get_lst_parameters().get(0)));
                } else {
                    return false;
                }

                switch_info.first().add_case_at(
                    processed_cases_idx,
                    cases,
                    if_stat.get_ifstat().expect("NullPointerException: ifstat"),
                );

                processed_cases_idx += 1;
                curr_if = else_stat;
            }
        }

        processed_cases_idx > 0
    }

    fn merge_duplicate_case_stats(switch_info: StringSwitch) {
        let case_stats = switch_info.first().get_case_statements();
        let mut i = 0;
        while i < case_stats.len() {
            let stat = case_stats.get(i);

            if stat.get_exprents().is_some_and(|e| e.len() != 1) {
                i += 1;
                continue;
            }

            let mut j = i + 1;
            while j < case_stats.len() {
                let next = case_stats.get(j);
                let next_exprents = next.get_exprents();
                if next_exprents.as_ref().is_some_and(|e| !e.is_empty())
                    && stat
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .get(0)
                        .equals(next_exprents.unwrap().get(0))
                {
                    let first = switch_info.first();
                    let moved = first.get_case_values().get(j).to_vec();
                    first.get_case_values().get(i).add_all(moved);

                    let edge = first.get_case_edges().get(j).get(0);
                    edge.set_destination(first.get_case_statements().get(i));
                    let new_lst = first.get_case_edges().get(i).copy();
                    new_lst.add(edge);
                    first.get_case_edges().set(i, new_lst);

                    first.remove_case_at(j as i32);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    fn find_synthetic_dup_var(switch_info: StringSwitch) -> Option<SyntheticDupVarResult> {
        let switch_head = switch_info
            .target()
            .get_headexprent()
            .try_as_switch_head()?;

        let head = match switch_info {
            StringSwitch::NullableSplit(si) => si.null_check().get_basichead(),
            _ => switch_info.target().get_basichead(),
        };

        let head_exprs = head.get_exprents();
        let dup_var_idx: i32 = if matches!(switch_info, StringSwitch::NullableSplit(_)) {
            2
        } else {
            1
        };
        let head_exprs = head_exprs?;
        if head_exprs.len() < dup_var_idx as usize {
            return None;
        }

        let assignment = head_exprs
            .get(head_exprs.len() - dup_var_idx as usize)
            .try_as_assignment()?;
        let tmp_var = assignment.get_left().try_as_var()?;
        let real_expr = assignment.get_right();

        let head_value = switch_head.get_value();
        let head_var: Option<VarExprent> = match head_value.try_as_invocation() {
            Some(val) => val.get_instance().map(|i| i.as_var()),
            None => Some(head_value.as_var()),
        };
        let equals_head_var = || head_var.is_some_and(|h| tmp_var.equals_versions(h.0));
        let whitelist: Vec<VarExprent> = head_var.into_iter().collect();

        if !equals_head_var()
            && tmp_var.is_var_referenced(
                switch_info
                    .first()
                    .get_parent()
                    .expect("NullPointerException: parent"),
                &whitelist,
            )
        {
            let mut is_dup_var_used_in_all_case_ifs = true;
            for stat in switch_info.first().get_case_statements().iter() {
                let Some(if_stat) = stat.try_as_if() else {
                    continue;
                };

                let mut if_cond = if_stat.get_headexprent().get_condition();
                if let Some(cond_func) = if_cond.try_as_function() {
                    if cond_func.get_func_type() == FunctionType::Ne {
                        if_cond = cond_func.get_lst_operands().get(0);
                    }
                }

                let Some(cond_invoc) = if_cond.try_as_invocation() else {
                    continue;
                };

                is_dup_var_used_in_all_case_ifs = is_dup_var_used_in_all_case_ifs
                    && cond_invoc
                        .get_instance()
                        .is_some_and(|i| tmp_var.equals_versions(i));
            }

            if !equals_head_var() || !is_dup_var_used_in_all_case_ifs {
                return None;
            }
        }

        Some(SyntheticDupVarResult::new(
            switch_head,
            head_exprs,
            dup_var_idx,
            tmp_var,
            real_expr,
        ))
    }

    fn remove_synthetic_dup_var(switch_info: StringSwitch) {
        let Some(result) = Self::find_synthetic_dup_var(switch_info) else {
            return;
        };

        result
            .switch_head()
            .replace_exprent(result.switch_head().get_value(), result.real_var());
        if !matches!(switch_info, StringSwitch::NullableSplit(_)) {
            let head_exprs = result.head_exprs();
            head_exprs.remove_at(head_exprs.len() - result.dup_var_idx() as usize);
        }
    }

    fn is_const_return(expr: Exprent) -> bool {
        expr.try_as_exit().is_some_and(|exit_exprent| {
            exit_exprent.get_exit_type() == ExitType::Return
                && exit_exprent.get_value().is_some_and(|v| v.is_const())
        })
    }

    fn is_const_assign_with_var(expr: Exprent, var_expr: VarExprent) -> bool {
        expr.try_as_assignment().is_some_and(|assign_expr| {
            assign_expr.get_right().is_const() && assign_expr.get_left().equals(var_expr.0)
        })
    }

    fn get_string_switch_case_map(switch_info: StringSwitch) -> JHashMap<i32, Vec<Exprent>> {
        let mut case_map: JHashMap<i32, Vec<Exprent>> = JHashMap::new();
        let mut case_ret_map: JHashMap<i32, Vec<Exprent>> = JHashMap::new();

        let mut i = 0;
        while i < switch_info.first().get_case_statements().len() {
            let mut curr_stat = Some(switch_info.first().get_case_statements().get(i));
            while let Some(if_stat) = curr_stat.and_then(|s| s.try_as_if()) {
                let mut condition = if_stat.get_headexprent().get_condition();

                if let Some(cond_func) = condition.try_as_function() {
                    if cond_func.get_func_type() == FunctionType::Ne {
                        condition = cond_func.get_lst_operands().get(0);
                    }
                }

                if let Some(cond_invoc) = condition
                    .try_as_invocation()
                    .filter(|c| c.get_lst_parameters().len() == 1)
                {
                    let if_exprs = if_stat
                        .get_ifstat()
                        .expect("NullPointerException: ifstat")
                        .get_exprents();

                    let Some(if_exprs) = if_exprs else {
                        curr_stat = None;
                        continue;
                    };

                    let if_eq_first_expr = if_exprs.get(0);
                    let real_val = cond_invoc.get_lst_parameters().get(0);

                    if let Some(assign_expr) = if_eq_first_expr.try_as_assignment() {
                        let intermediate = assign_expr.get_right().as_const().get_int_value();
                        case_map
                            .compute_if_absent(intermediate, || new_array_list(intermediate))
                            .push(real_val);
                    } else if if_eq_first_expr.is_exit() {
                        let curr_case_val = switch_info.first().get_case_values().get(i);
                        for val in curr_case_val.iter() {
                            let val = val.expect("NullPointerException: case value");
                            let hash_code = val.as_const().get_int_value();
                            case_map
                                .compute_if_absent(hash_code, || new_array_list(hash_code))
                                .push(real_val);
                            case_ret_map
                                .compute_if_absent(hash_code, || new_array_list(hash_code))
                                .push(val);
                        }
                    }
                }

                curr_stat = if_stat.get_elsestat();
            }
            i += 1;
        }

        if let StringSwitch::NullableSplit(si) = switch_info {
            let old_val = si.expr().get_right().as_const().get_int_value();
            let val = ConstExprent::new(VarType::VARTYPE_NULL(), None, None);
            case_map
                .compute_if_absent(old_val, || new_array_list(old_val))
                .push(val.0);
        }

        case_map
    }

    fn get_string_switch_real_case_values(
        switch_info: StringSwitch,
        case_map: &JHashMap<i32, Vec<Exprent>>,
    ) -> Vec<JList<Option<Exprent>>> {
        let mut real_case_values: Vec<JList<Option<Exprent>>> = Vec::new();

        let mut i = 0;
        while i < switch_info.target().get_case_values().len() {
            let real_case_val: JList<Option<Exprent>> = JList::new();

            let target_case_val = switch_info.target().get_case_values().get(i);
            let target_case_edges = switch_info.target().get_case_edges().get(i);

            let mut j = 0;
            while j < target_case_val.len() {
                let target_val = target_case_val.get(j);
                let target_edge = target_case_edges.get(j);

                let intermediate: Option<i32> = target_val
                    .and_then(|v| v.try_as_const())
                    .map(|e| e.get_int_value());

                let real_val = intermediate.and_then(|k| case_map.get(&k));
                let Some(real_val) = real_val else {
                    let default_edge = switch_info.target().get_default_edge();
                    if default_edge.is_some_and(|default_edge| {
                        target_edge != default_edge
                            && target_edge.get_destination() == default_edge.get_destination()
                    }) {
                        target_case_edges.remove_at(j);
                        target_case_val.remove_at(j);
                    } else {
                        real_case_val.add(None);
                        j += 1;
                    }

                    continue;
                };

                real_case_val.add_all(real_val.iter().map(|v| Some(*v)));
                j += 1;
            }

            real_case_values.push(real_case_val);
            i += 1;
        }

        real_case_values
    }

    fn simplify_switch_on_enum_j21(switch_st: SwitchStatement, root: RootStatement) -> bool {
        let head = switch_st.get_headexprent().as_switch_head();
        let inner = head.get_value();

        let mut mapping: JHashMap<ConstExprent, Option<String>> = JHashMap::new();
        let values = switch_st.get_case_values();
        for list in values.iter() {
            for v in list.iter() {
                let Some(v) = v else {
                    continue;
                };

                if !v.is_const() {
                    return false;
                }

                if v.get_expr_type().type_family != TypeFamily::Integer {
                    return false;
                }

                mapping.put(v.as_const(), None);
            }
        }

        if let Some(inv_inner) = inner.try_as_invocation() {
            if inv_inner.get_name() == "ordinal" {
                let class_struct =
                    DecompilerContext::get_struct_context().get_class(&inv_inner.get_classname());
                let Some(class_struct) = class_struct else {
                    root.add_comment_error(
                        "$VF: Unable to simplify switch-on-enum, as the enum class was not able to be found.",
                        true,
                    );
                    return false;
                };

                if (class_struct.get_access_flags() & ACC_ENUM) == ACC_ENUM {
                    let mut enum_names: Vec<String> = Vec::new();

                    for fd in class_struct.get_fields().iter() {
                        if (fd.get_access_flags() & ACC_ENUM) == ACC_ENUM {
                            enum_names.push(fd.get_name());
                        }
                    }

                    for e in JHashSet::from_collection(mapping.key_vec()).to_vec() {
                        for lst in values.iter() {
                            let mut i = 0;
                            while i < lst.len() {
                                let ex = lst.get(i);

                                if ex == Some(e.0) {
                                    let idx = e.get_int_value();
                                    let name =
                                        enum_names[java_index(idx, enum_names.len())].clone();
                                    let classname = inv_inner.get_classname();
                                    let descriptor = FieldDescriptor::parse_descriptor(&format!(
                                        "L{classname};"
                                    ));
                                    lst.set(
                                        i,
                                        Some(
                                            FieldExprent::new(
                                                name, classname, true, None, descriptor, None,
                                            )
                                            .0,
                                        ),
                                    );
                                }
                                i += 1;
                            }
                        }
                    }

                    if switch_st.is_phantom() {
                        let mut i = 0;
                        while i < values.len() {
                            let list = values.get(i);

                            if list.len() == 1 && list.get(0).is_none() {
                                let st = switch_st.get_case_statements().get(i);
                                if IfPatternMatchProcessor::is_statement_match_throw(st) {
                                    st.replace_with_empty();
                                }
                            }
                            i += 1;
                        }
                    }

                    head.replace_exprent(
                        inner,
                        inv_inner
                            .get_instance()
                            .expect("NullPointerException: instance"),
                    );

                    return true;
                }
            }
        }

        false
    }

    fn is_enum_array(exprent: Exprent) -> bool {
        if let Some(arr) = exprent.try_as_array() {
            let tmp = arr.get_array();
            if let Some(field) = tmp.try_as_field() {
                let index = arr.get_index();
                let class_node = DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get_cloned(&field.get_classname());

                let Some(class_node) =
                    class_node.filter(|_| "[I" == field.get_descriptor().descriptor_string)
                else {
                    return field.get_name().starts_with("$SwitchMap")
                        || (index
                            .try_as_invocation()
                            .is_some_and(|i| i.get_name() == "ordinal")
                            && field.is_static());
                };

                let descriptor_string = field.get_descriptor().descriptor_string.clone();
                let st_field = match class_node.get_wrapper() {
                    None => class_node
                        .with(|d| d.class_struct)
                        .get_field(&field.get_name(), &descriptor_string),
                    Some(wrapper) => wrapper
                        .get_class_struct()
                        .get_field(&field.get_name(), &descriptor_string),
                }
                .expect("NullPointerException: field");

                if (st_field.get_access_flags() & Self::STATIC_FINAL_SYNTHETIC)
                    != Self::STATIC_FINAL_SYNTHETIC
                {
                    return false;
                }

                let is_synthetic_class = match class_node.get_wrapper() {
                    None => {
                        (class_node.with(|d| d.class_struct).get_access_flags() & ACC_SYNTHETIC)
                            == ACC_SYNTHETIC
                    }
                    Some(wrapper) => {
                        (wrapper.get_class_struct().get_access_flags() & ACC_SYNTHETIC)
                            == ACC_SYNTHETIC
                    }
                };

                if is_synthetic_class {
                    return true;
                }
            } else if let Some(inv) = tmp.try_as_invocation() {
                if inv.get_name().starts_with("$SWITCH_TABLE$") {
                    return true;
                }
            }
        }
        false
    }

    fn get_enum_array_exprent(switch_head: Exprent, root: RootStatement) -> Option<ArrayExprent> {
        let mut candidate = switch_head;

        if let Some(func) = switch_head.try_as_function() {
            if func.get_func_type() == FunctionType::Ternary && func.get_lst_operands().len() == 3 {
                let ops = func.get_lst_operands();
                if let Some(nn) = ops.get(0).try_as_function() {
                    if nn.get_func_type() == FunctionType::Ne
                        && nn.get_lst_operands().get(0).is_var()
                        && nn
                            .get_lst_operands()
                            .get(1)
                            .get_expr_type()
                            .equals(&VarType::VARTYPE_NULL())
                    {
                        if let Some(minus_one) = ops.get(2).try_as_const() {
                            if minus_one.get_const_type().equals(&VarType::VARTYPE_INT())
                                && minus_one.get_int_value() == -1
                            {
                                candidate = ops.get(1);
                            }
                        }
                    }
                }
            }
        }

        if let Some(var) = switch_head.try_as_var() {
            if "I" != var.get_var_type().to_string() {
                return None;
            }

            let assignments = Self::get_assignments_of_within_one_statement(root.0, var.0);

            if !assignments.is_empty() {
                if assignments.len() == 1 {
                    let assignment = assignments[0];
                    candidate = assignment.get_right();
                } else {
                    return None;
                }
            }
        }

        if Self::is_enum_array(candidate) {
            Some(candidate.as_array())
        } else {
            None
        }
    }

    fn get_assignments_of_within_one_statement(
        start: Statement,
        target: Exprent,
    ) -> Vec<AssignmentExprent> {
        let mut exprents = Vec::new();
        Self::find_exprents(
            start,
            Exprent::try_as_assignment,
            |assignment| assignment.get_left().equals(target),
            true,
            |_stat, expr| exprents.push(expr),
        );
        exprents
    }

    /// Java: findExprents(Statement, Class, Predicate, boolean, BiConsumer); `expr_class` is the
    /// `Class.isInstance` + cast pair (`Some` for `Exprent.class`, `Exprent::try_as_assignment`, ...).
    pub fn find_exprents<T: Copy>(
        start: Statement,
        expr_class: impl Fn(Exprent) -> Option<T>,
        predicate: impl Fn(T) -> bool,
        only_one_stat: bool,
        mut consumer: impl FnMut(Statement, T),
    ) {
        let mut stat_queue: VecDeque<Statement> = VecDeque::new();
        stat_queue.push_back(start);

        while let Some(stat) = stat_queue.pop_front() {
            stat_queue.extend(stat.get_stats().borrow().to_vec());

            if let Some(exprents) = stat.get_exprents() {
                let mut found_any = false;

                for expr in exprents.iter() {
                    if let Some(t) = expr_class(expr) {
                        if predicate(t) {
                            consumer(stat, t);
                            found_any = true;
                        }
                    }
                }

                if only_one_stat && found_any {
                    break;
                }
            }
        }
    }
}

/// `record SyntheticDupVarResult(...)`; `head_exprs` aliases the head block's exprent list.
#[derive(Clone)]
struct SyntheticDupVarResult {
    switch_head: SwitchHeadExprent,
    head_exprs: JList<Exprent>,
    dup_var_idx: i32,
    tmp_var: VarExprent,
    real_var: Exprent,
}

impl SyntheticDupVarResult {
    fn new(
        switch_head: SwitchHeadExprent,
        head_exprs: JList<Exprent>,
        dup_var_idx: i32,
        tmp_var: VarExprent,
        real_var: Exprent,
    ) -> SyntheticDupVarResult {
        SyntheticDupVarResult {
            switch_head,
            head_exprs,
            dup_var_idx,
            tmp_var,
            real_var,
        }
    }

    pub fn switch_head(&self) -> SwitchHeadExprent {
        self.switch_head
    }

    pub fn head_exprs(&self) -> JList<Exprent> {
        self.head_exprs.clone()
    }

    pub fn dup_var_idx(&self) -> i32 {
        self.dup_var_idx
    }

    pub fn tmp_var(&self) -> VarExprent {
        self.tmp_var
    }

    pub fn real_var(&self) -> Exprent {
        self.real_var
    }
}

impl JavaEq for SyntheticDupVarResult {
    fn java_equals(&self, other: &Self) -> bool {
        self.switch_head.0.java_equals(&other.switch_head.0)
            && self
                .head_exprs
                .to_vec()
                .java_equals(&other.head_exprs.to_vec())
            && self.dup_var_idx == other.dup_var_idx
            && self.tmp_var.0.java_equals(&other.tmp_var.0)
            && self.real_var.java_equals(&other.real_var)
    }
}

impl JavaHash for SyntheticDupVarResult {
    fn java_hash(&self) -> i32 {
        record_hash(&[
            self.switch_head.0.java_hash(),
            self.head_exprs.to_vec().java_hash(),
            self.dup_var_idx,
            self.tmp_var.0.java_hash(),
            self.real_var.java_hash(),
        ])
    }
}

/// `sealed interface StringSwitch permits Split, InlineSplit, NullableSplit, Merged`.
#[derive(Clone, Copy, Debug)]
enum StringSwitch {
    Split(Split),
    InlineSplit(InlineSplit),
    NullableSplit(NullableSplit),
    Merged(Merged),
}

impl StringSwitch {
    pub fn first(&self) -> SwitchStatement {
        match self {
            StringSwitch::Split(s) => s.first(),
            StringSwitch::InlineSplit(s) => s.first(),
            StringSwitch::NullableSplit(s) => s.first(),
            StringSwitch::Merged(s) => s.first(),
        }
    }

    pub fn target(&self) -> SwitchStatement {
        match self {
            StringSwitch::Split(s) => s.target(),
            StringSwitch::InlineSplit(s) => s.target(),
            StringSwitch::NullableSplit(s) => s.target(),
            StringSwitch::Merged(s) => s.target(),
        }
    }

    pub fn match_(stat: SwitchStatement) -> Option<StringSwitch> {
        let mut result = Split::match_(stat);
        if result.is_some() {
            return result;
        }

        result = InlineSplit::match_(stat);
        if result.is_some() {
            return result;
        }

        result = NullableSplit::match_(stat);
        if result.is_some() {
            return result;
        }

        Merged::match_(stat)
    }

    pub fn is_valid(sw: StringSwitch) -> bool {
        let first_head_val = sw.first().get_headexprent().as_switch_head().get_value();
        let Some(first_head_val_invoc) = first_head_val.try_as_invocation() else {
            return false;
        };
        if first_head_val_invoc.get_name() != "hashCode"
            || first_head_val_invoc.get_classname() != "java/lang/String"
            || !sw.first().get_case_statements().get(0).is_if()
        {
            return false;
        }

        let mut intermediate: Option<VarExprent> = None;
        if !matches!(sw, StringSwitch::Merged(_)) {
            let Some(var_expr) = sw
                .target()
                .get_headexprent()
                .as_switch_head()
                .get_value()
                .try_as_var()
            else {
                return false;
            };
            intermediate = Some(var_expr);

            if let StringSwitch::NullableSplit(s) = sw {
                if !s.expr().get_left().equals(var_expr.0) {
                    return false;
                }
            }
        }

        let possible_dup_var = SwitchHelper::find_synthetic_dup_var(sw);

        let first_instance = first_head_val_invoc.get_instance();
        let not_equals_on_head = |e: Exprent| -> bool {
            let Some(cond_invoc) = e.try_as_invocation() else {
                return true;
            };
            if cond_invoc.get_name() != "equals" {
                return true;
            }
            let instance = cond_invoc
                .get_instance()
                .expect("NullPointerException: instance");
            !first_instance.is_some_and(|f| instance.equals(f))
                && possible_dup_var
                    .as_ref()
                    .is_some_and(|dup| !instance.equals(dup.tmp_var().0))
        };

        let mut i = 0;
        while i < sw.first().get_case_statements().len() {
            let mut curr_stat = Some(sw.first().get_case_statements().get(i));
            let curr_edge = sw.first().get_case_edges().get(i);

            if sw
                .first()
                .get_default_edge()
                .is_some_and(|default_edge| curr_edge.contains(&default_edge))
            {
                i += 1;
                continue;
            }

            while let Some(stat) = curr_stat {
                let Some(if_stat) = stat.try_as_if() else {
                    return false;
                };

                let mut if_cond = if_stat.get_headexprent().get_condition();
                if let Some(cond_func) = if_cond.try_as_function() {
                    if cond_func.get_func_type() == FunctionType::Ne
                        || cond_func.get_func_type() == FunctionType::BoolNot
                    {
                        if_cond = cond_func.get_lst_operands().get(0);
                    }
                }

                if let Some(cond_func) = if_cond.try_as_function() {
                    for oper in cond_func.get_lst_operands().iter() {
                        if not_equals_on_head(oper) {
                            return false;
                        }
                    }
                } else if not_equals_on_head(if_cond) {
                    return false;
                }

                let is_if_break = if_stat.get_ifstat().is_none()
                    && if_stat.get_elsestat().is_none()
                    && if_stat.get_if_edge().is_some()
                    && if_stat.get_else_edge().is_none()
                    && if_stat
                        .get_basichead()
                        .has_successor(StatEdge::TYPE_REGULAR)
                    && if_stat
                        .get_basichead()
                        .get_successor_edges(StatEdge::TYPE_BREAK)
                        .len()
                        == 1;

                if !is_if_break {
                    let block = if_stat.get_ifstat().and_then(|s| s.get_exprents());
                    let Some(block) = block.filter(|b| b.len() == 1) else {
                        return false;
                    };

                    if matches!(sw, StringSwitch::Merged(_))
                        && !SwitchHelper::is_const_return(block.get(0))
                    {
                        return false;
                    }

                    if !matches!(sw, StringSwitch::Merged(_))
                        && !SwitchHelper::is_const_assign_with_var(
                            block.get(0),
                            intermediate.expect("intermediate"),
                        )
                    {
                        return false;
                    }
                } else if let Some(default_case) = sw.first().get_default_case() {
                    let default_break = default_case.get_all_successor_edges()[0];
                    let if_edge = if_stat.get_if_edge().expect("NullPointerException: ifEdge");
                    if if_edge.get_type() != StatEdge::TYPE_BREAK
                        || if_edge.get_destination() != default_break.get_destination()
                    {
                        return false;
                    }
                }

                curr_stat = if_stat.get_elsestat();
            }
            i += 1;
        }

        true
    }
}

/// `record Split(SwitchStatement first, SwitchStatement target)`.
#[derive(Clone, Copy, Debug)]
pub struct Split {
    first: SwitchStatement,
    target: SwitchStatement,
}

impl Split {
    pub fn new(first: SwitchStatement, target: SwitchStatement) -> Split {
        Split { first, target }
    }

    pub fn first(&self) -> SwitchStatement {
        self.first
    }

    pub fn target(&self) -> SwitchStatement {
        self.target
    }

    fn match_(stat: SwitchStatement) -> Option<StringSwitch> {
        let edges = stat.get_successor_edges(StatEdge::TYPE_REGULAR);
        if edges.len() != 1 {
            return None;
        }
        let found = edges[0].get_destination().try_as_switch()?;

        let matched = StringSwitch::Split(Split::new(stat, found));
        if StringSwitch::is_valid(matched) {
            Some(matched)
        } else {
            None
        }
    }
}

/// `record InlineSplit(SwitchStatement first, SwitchStatement target)`.
#[derive(Clone, Copy, Debug)]
pub struct InlineSplit {
    first: SwitchStatement,
    target: SwitchStatement,
}

impl InlineSplit {
    pub fn new(first: SwitchStatement, target: SwitchStatement) -> InlineSplit {
        InlineSplit { first, target }
    }

    pub fn first(&self) -> SwitchStatement {
        self.first
    }

    pub fn target(&self) -> SwitchStatement {
        self.target
    }

    fn match_(stat: SwitchStatement) -> Option<StringSwitch> {
        let default_edge = stat.get_default_edge()?;

        let default_dest = default_edge.get_destination();
        let target = if let Some(found) = default_dest.try_as_switch() {
            found
        } else if default_dest.is_sequence()
            && default_dest.get_first().is_some_and(|f| f.is_switch())
        {
            default_dest.get_first().unwrap().as_switch()
        } else {
            return None;
        };

        let matched = StringSwitch::InlineSplit(InlineSplit::new(stat, target));
        if StringSwitch::is_valid(matched) {
            Some(matched)
        } else {
            None
        }
    }
}

/// `record NullableSplit(SwitchStatement first, SwitchStatement target, AssignmentExprent expr, IfStatement nullCheck)`.
#[derive(Clone, Copy, Debug)]
pub struct NullableSplit {
    first: SwitchStatement,
    target: SwitchStatement,
    expr: AssignmentExprent,
    null_check: IfStatement,
}

impl NullableSplit {
    pub fn new(
        first: SwitchStatement,
        target: SwitchStatement,
        expr: AssignmentExprent,
        null_check: IfStatement,
    ) -> NullableSplit {
        NullableSplit {
            first,
            target,
            expr,
            null_check,
        }
    }

    pub fn first(&self) -> SwitchStatement {
        self.first
    }

    pub fn target(&self) -> SwitchStatement {
        self.target
    }

    pub fn expr(&self) -> AssignmentExprent {
        self.expr
    }

    pub fn null_check(&self) -> IfStatement {
        self.null_check
    }

    fn match_(stat: SwitchStatement) -> Option<StringSwitch> {
        let parent = stat.get_parent().and_then(|p| p.try_as_if());
        let parent = parent.filter(|_| !stat.has_successor(StatEdge::TYPE_REGULAR))?;

        let if_cond = parent.get_headexprent().get_condition();
        if parent.iftype() != IfStatement::IFTYPE_IFELSE {
            return None;
        }
        let func_expr = if_cond.try_as_function()?;
        if func_expr.get_func_type() != FunctionType::Ne || func_expr.get_lst_operands().len() != 2
        {
            return None;
        }

        let right = func_expr.get_lst_operands().get(1);
        if !right.is_const() || !right.get_expr_type().ptr_eq(&VarType::VARTYPE_NULL()) {
            return None;
        }

        let else_stat = parent.get_elsestat();
        let else_exprents = else_stat
            .filter(|s| s.is_basic_block())
            .and_then(|s| s.get_exprents())
            .filter(|e| e.len() == 1)?;
        let null_assign_expr = else_exprents.get(0).try_as_assignment()?;

        let edges = parent.get_successor_edges(StatEdge::TYPE_REGULAR);
        if edges.len() != 1 {
            return None;
        }
        let found = edges[0].get_destination().try_as_switch()?;

        let matched =
            StringSwitch::NullableSplit(NullableSplit::new(stat, found, null_assign_expr, parent));
        if StringSwitch::is_valid(matched) {
            Some(matched)
        } else {
            None
        }
    }
}

/// `record Merged(SwitchStatement first, SwitchStatement target)`.
#[derive(Clone, Copy, Debug)]
pub struct Merged {
    first: SwitchStatement,
    target: SwitchStatement,
}

impl Merged {
    pub fn new(first: SwitchStatement, target: SwitchStatement) -> Merged {
        Merged { first, target }
    }

    pub fn first(&self) -> SwitchStatement {
        self.first
    }

    pub fn target(&self) -> SwitchStatement {
        self.target
    }

    fn match_(stat: SwitchStatement) -> Option<StringSwitch> {
        let matched = StringSwitch::Merged(Merged::new(stat, stat));
        if StringSwitch::is_valid(matched) {
            Some(matched)
        } else {
            None
        }
    }
}

macro_rules! two_switch_record_eq_hash {
    ($($t:ident),*) => {$(
        impl JavaEq for $t {
            fn java_equals(&self, other: &Self) -> bool {
                self.first == other.first && self.target == other.target
            }
        }

        impl JavaHash for $t {
            fn java_hash(&self) -> i32 {
                record_hash(&[self.first.0.java_hash(), self.target.0.java_hash()])
            }
        }
    )*};
}
two_switch_record_eq_hash!(Split, InlineSplit, Merged);

impl JavaEq for NullableSplit {
    fn java_equals(&self, other: &Self) -> bool {
        self.first == other.first
            && self.target == other.target
            && self.expr.0.java_equals(&other.expr.0)
            && self.null_check == other.null_check
    }
}

impl JavaHash for NullableSplit {
    fn java_hash(&self) -> i32 {
        record_hash(&[
            self.first.0.java_hash(),
            self.target.0.java_hash(),
            self.expr.0.java_hash(),
            self.null_check.0.java_hash(),
        ])
    }
}

/// `Record.hashCode()` as the JDK generates it: `31 * h + hash(c)` over the components, from 0.
fn record_hash(components: &[i32]) -> i32 {
    components
        .iter()
        .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(*c))
}

/// `new ArrayList<>(initialCapacity)` (what `computeIfAbsent(k, ArrayList::new)` calls with the key).
fn new_array_list<T>(initial_capacity: i32) -> Vec<T> {
    if initial_capacity < 0 {
        jthrow!(IllegalArgumentException; "Illegal Capacity: {}", initial_capacity);
    }
    Vec::new()
}

/// `List.get(int)` index check.
fn java_index(index: i32, len: usize) -> usize {
    match usize::try_from(index) {
        Ok(i) if i < len => i,
        _ => jthrow!(IndexOutOfBoundsException; "Index {} out of bounds for length {}", index, len),
    }
}

/// `HashSet.forEach` (the iterator form): a structural change made by `action` throws
/// `ConcurrentModificationException` when the iteration then moves to a further element.
fn hash_set_for_each(set: &Shared<JHashSet<StatEdge>>, mut action: impl FnMut(StatEdge)) {
    let snapshot = set.borrow().to_vec();
    let len = snapshot.len();
    for (i, &e) in snapshot.iter().enumerate() {
        if i > 0 {
            let current = set.borrow();
            if current.len() != len || !snapshot.iter().all(|x| current.contains(x)) {
                jthrow!(ConcurrentModificationException; "");
            }
        }
        action(e);
    }
}

/// `AbstractMap.toString()` of the enum switch mapping.
fn enum_mapping_to_string(mapping: &JHashMap<Exprent, Option<Exprent>>) -> String {
    let entries: Vec<String> = mapping
        .entry_vec()
        .into_iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                k.to_string(),
                v.map_or_else(|| "null".to_string(), |v| v.to_string())
            )
        })
        .collect();
    format!("{{{}}}", entries.join(", "))
}
