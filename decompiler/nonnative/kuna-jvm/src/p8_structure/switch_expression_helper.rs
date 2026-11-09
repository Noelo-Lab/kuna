//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchExpressionHelper.java`.
//!
//! kuna fix `switch_expression_assignments` (`kuna_fixes`): a switch becomes
//! an expression only when every assignment it turns into a `yield` is the
//! last thing a case does before it breaks
//! (`kuna_fix_switch_expression_assignments`).

use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_switch_expression_assignments;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet};
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::switch_exprent::SwitchExprent;
use crate::switch_statement::SwitchStatement;
use crate::try_with_resources_processor::TryWithResourcesProcessor;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;
use crate::yield_exprent::YieldExprent;

pub struct SwitchExpressionHelper;

impl SwitchExpressionHelper {
    pub fn process_switch_expressions(root: Statement) -> bool {
        let ret = Self::process_switch_expressions_rec(root);

        if ret {
            SequenceHelper::condense_sequences(root);
        }

        ret
    }

    fn process_switch_expressions_rec(stat: Statement) -> bool {
        let mut ret = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            ret |= Self::process_switch_expressions_rec(st);
        }

        if let Some(switch_stat) = stat.try_as_switch() {
            ret |= Self::process_statement(switch_stat);
        }

        ret
    }

    fn process_statement(stat: SwitchStatement) -> bool {
        if stat.is_phantom() {
            return false;
        }

        let condition = stat.get_headexprent().as_switch_head().get_value();
        if let Some(invoc) = condition.try_as_invocation() {
            if invoc.get_name() == "hashCode" && invoc.get_classname() == "java/lang/String" {
                return false;
            }
        }

        let Some(next_data) = Self::find_next_data(stat) else {
            return false;
        };

        let break_jumps = next_data.b.clone();

        let mut check: JHashSet<Statement> = JHashSet::new();
        check.add_all(break_jumps.iter().copied());
        check.add_all(stat.get_case_statements().iter());

        for st in check.to_vec() {
            if !st.has_basic_succ_edge() {
                continue;
            }

            let breaks = st.get_successor_edges(StatEdge::TYPE_BREAK);

            if breaks.is_empty() {
                return false;
            }

            let first_break = breaks[0];

            if !first_break
                .closure()
                .is_some_and(|closure| stat.contains_statement(closure))
                && !first_break.get_destination().is_dummy_exit()
            {
                return false;
            }
        }

        let mut temp: JHashSet<StatEdge> = JHashSet::new();
        let case_statements = stat.get_case_statements();
        let mut i = 0;
        while i < case_statements.len() {
            let st = case_statements.get(i);
            temp.clear();

            TryWithResourcesProcessor::find_edges_leaving_exit(st, stat.0, &mut temp, true);

            let mut saw_break = false;
            for edge in temp.to_vec() {
                if edge.get_type() == StatEdge::TYPE_BREAK {
                    if edge.get_destination() != next_data.a
                        && !edge.get_destination().is_dummy_exit()
                    {
                        return false;
                    }

                    if !edge.explicit() && i != case_statements.len() - 1 {
                        return false;
                    }

                    saw_break = true;
                }
            }

            if !saw_break {
                return false;
            }
            i += 1;
        }

        let Some(assignments) = Self::map_assignments(&break_jumps) else {
            return false;
        };

        let found_default = stat
            .get_case_edges()
            .iter()
            .flat_map(|edges| edges.iter())
            .any(|e| Some(e) == stat.get_default_edge());

        if !found_default {
            return false;
        }

        let Some(relevant_var) = Self::find_relevant_var(&assignments) else {
            return false;
        };

        if kuna_fixes::enabled(KunaFix::SwitchExpressionAssignments)
            && !kuna_fix_switch_expression_assignments::every_assignment_yields(
                stat,
                &break_jumps,
                relevant_var.get_index(),
            )
        {
            return false;
        }

        let mut edges: JHashSet<StatEdge> = JHashSet::new();
        for case_stat in stat.get_case_statements().iter() {
            TryWithResourcesProcessor::find_edges_leaving(case_stat, case_stat, &mut edges);

            for edge in edges.to_vec() {
                if edge.get_type() == StatEdge::TYPE_CONTINUE {
                    return false;
                }
            }

            edges.clear();
        }

        let sucs = stat.get_successor_edges(StatEdge::TYPE_REGULAR);

        if !sucs.is_empty() {
            let mut suc = sucs[0].get_destination();
            if !suc.is_basic_block() {
                let old_suc = suc;

                suc = BasicBlockStatement::create().0;
                let seq = SequenceStatement::new_varargs(&[stat.0, suc]);

                seq.set_parent(stat.get_parent());

                stat.replace_with(seq.0);

                seq.set_all_parent();

                for edge in old_suc.get_all_predecessor_edges() {
                    if stat.contains_statement(edge.get_source()) {
                        let source = edge.get_source();
                        source.remove_successor(edge);

                        source.add_successor(StatEdge::new_closure(
                            edge.get_type(),
                            source,
                            suc,
                            Some(stat.0),
                        ));
                    }
                }

                suc.add_successor(StatEdge::new_closure(
                    StatEdge::TYPE_REGULAR,
                    suc,
                    old_suc,
                    Some(seq.0),
                ));
            }

            stat.set_phantom(true);

            for st in stat.get_case_statements().iter() {
                let mut replacements: JHashMap<Exprent, YieldExprent> = JHashMap::new();

                Self::find_replacements(st, relevant_var.get_var_version_pair(), &mut replacements);

                if !replacements.is_empty() {
                    Self::replace(st, &replacements);
                }
            }

            let exprents = suc.get_exprents().expect("NullPointerException: exprents");

            let v_expr = VarExprent::new(
                relevant_var.get_index(),
                Some(relevant_var.get_var_type()),
                relevant_var.get_processor(),
            );
            v_expr.set_stack(true);
            let switch_exprent =
                SwitchExprent::new(stat, relevant_var.get_expr_type(), false, false);
            let to_add = AssignmentExprent::new(v_expr.0, switch_exprent.0, None);

            exprents.add_at(0, to_add.0);

            let first_exprents = stat
                .get_first()
                .expect("NullPointerException: first")
                .get_exprents();
            if let Some(first_exprents) = first_exprents.filter(|e| !e.is_empty()) {
                let mut i = 0;
                let mut it = 0;
                while it < first_exprents.len() {
                    let ex = first_exprents.get(it);
                    let is_stack_assignment = ex
                        .try_as_assignment()
                        .and_then(|a| a.get_left().try_as_var())
                        .is_some_and(|v| v.is_stack());
                    if is_stack_assignment {
                        exprents.add_at(i, ex);
                        i += 1;
                        first_exprents.remove_at(it);
                    } else {
                        it += 1;
                    }
                }
            }

            return true;
        }

        false
    }

    fn find_replacements(
        stat: Statement,
        var: VarVersionPair,
        replacements: &mut JHashMap<Exprent, YieldExprent>,
    ) {
        if let Some(exprents) = stat.get_exprents() {
            for e in exprents.iter() {
                if let Some(assign) = e.try_as_assignment() {
                    if let Some(left) = assign.get_left().try_as_var() {
                        if left.get_index() == var.var {
                            let yield_exprent =
                                YieldExprent::new(assign.get_right(), assign.get_expr_type());
                            replacements.put(assign.0, yield_exprent);
                        }
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::find_replacements(st, var, replacements);
        }
    }

    fn replace(stat: Statement, replacements: &JHashMap<Exprent, YieldExprent>) {
        for (key, value) in replacements.entry_vec() {
            stat.replace_exprent(key, value.0);
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::replace(st, replacements);
        }
    }

    /// `<next statement, break sources>`, or `None` when no regular successor is found.
    fn find_next_data(stat: SwitchStatement) -> Option<Pair<Statement, Vec<Statement>>> {
        let mut edges = stat.get_successor_edges(StatEdge::TYPE_REGULAR);
        let mut check = stat.get_parent();
        while edges.is_empty() {
            let current = check.expect("NullPointerException: parent");
            edges = current.get_successor_edges(StatEdge::TYPE_REGULAR);
            check = current.get_parent();

            check?;
        }

        let edge = edges[0];
        let next = edge.get_destination();

        let mut breaks = next.get_predecessor_edges(StatEdge::TYPE_BREAK);

        breaks.extend(
            stat.get_top_parent()
                .get_dummy_exit()
                .get_predecessor_edges(StatEdge::TYPE_BREAK),
        );

        breaks.retain(|e| stat.contains_statement(e.get_source()));

        Some(Pair::of(
            next,
            breaks.iter().map(|e| e.get_source()).collect(),
        ))
    }

    /// `None` when a block ends in a non-throw exit; a `None` value marks a block that throws.
    fn map_assignments(
        break_jumps: &[Statement],
    ) -> Option<JHashMap<Statement, Option<Vec<VarVersionPair>>>> {
        let mut map: JHashMap<Statement, Option<Vec<VarVersionPair>>> = JHashMap::new();

        for &break_jump in break_jumps {
            let exprents = break_jump.get_exprents();

            if let Some(exprents) = exprents.filter(|e| !e.is_empty()) {
                if !exprents.is_empty() {
                    if let Some(exit) = exprents.get(exprents.len() - 1).try_as_exit() {
                        if exit.get_exit_type() == ExitType::Throw {
                            map.put(break_jump, None);
                            continue;
                        } else {
                            return None;
                        }
                    }
                }

                let mut list: Vec<VarVersionPair> = Vec::new();
                for i in (0..exprents.len()).rev() {
                    let exprent = exprents.get(i);
                    if let Some(assign) = exprent.try_as_assignment() {
                        if let Some(var) = assign.get_left().try_as_var() {
                            list.push(var.get_var_version_pair());

                            if var.get_index() >= VarExprent::STACK_BASE {
                                break;
                            }
                            continue;
                        }
                    }
                    break;
                }

                map.put(break_jump, Some(list));
            }
        }

        Some(map)
    }

    fn find_relevant_var(
        assignments: &JHashMap<Statement, Option<Vec<VarVersionPair>>>,
    ) -> Option<VarExprent> {
        let values: Vec<Option<Vec<VarVersionPair>>> = assignments.value_vec();

        let mut consistently_more_than1 = true;

        for value in &values {
            let Some(value) = value else {
                continue;
            };

            if value.is_empty() {
                return None;
            }

            if value.len() == 1 {
                consistently_more_than1 = false;
            }
        }

        if consistently_more_than1 {
            return None;
        }

        let mut first_not_null: Option<usize> = None;
        for (index, value) in values.iter().enumerate() {
            if value.is_none() {
                continue;
            }

            first_not_null = Some(index);
            break;
        }

        let first_not_null = first_not_null?;

        let check = values[first_not_null].as_ref().expect("first non-null")[0];

        for value in values.iter().flatten() {
            if value[0] != check {
                return None;
            }
        }

        for (index, (key, _)) in assignments.iter().enumerate() {
            if index == first_not_null {
                let exprents = key.get_exprents().expect("NullPointerException: exprents");
                let exprent = exprents.get(exprents.len() - 1);
                return Some(exprent.as_assignment().get_left().as_var());
            }
        }

        None
    }

    pub fn has_switch_expressions(statement: RootStatement) -> bool {
        statement
            .mt()
            .get_bytecode_version()
            .has_switch_expressions()
            && DecompilerContext::get_option(IFernflowerPreferences::SWITCH_EXPRESSIONS)
    }
}
