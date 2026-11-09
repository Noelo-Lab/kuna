//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchPatternMatchProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::assignment_exprent::AssignmentExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_pattern_match_processor::IfPatternMatchProcessor;
use crate::invocation_exprent::InvocationType;
use crate::pair::Pair;
use crate::pattern_exprent::PatternExprent;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JList};
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::switch_statement::SwitchStatement;
use crate::unported;
use crate::validation_helper::ValidationHelper;

pub struct SwitchPatternMatchProcessor;

impl SwitchPatternMatchProcessor {
    pub fn process_pattern_matching(root: Statement) -> bool {
        let ret = Self::process_pattern_matching_rec(root, root);

        if ret {
            SequenceHelper::condense_sequences(root);
        }

        ret
    }

    fn process_pattern_matching_rec(stat: Statement, root: Statement) -> bool {
        ValidationHelper::validate_statement(root.as_root());

        let mut ret = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            ret |= Self::process_pattern_matching_rec(st, root);
            ValidationHelper::validate_statement(root.as_root());
        }

        if let Some(switch_stat) = stat.try_as_switch() {
            ret |= Self::process_statement(switch_stat, root);
            ValidationHelper::validate_statement(root.as_root());
        }

        ret
    }

    fn process_statement(stat: SwitchStatement, root: Statement) -> bool {
        unported!("SwitchPatternMatchProcessor.processStatement")
    }

    fn get_index(check: i32, values: &JList<JList<Option<Exprent>>>) -> i32 {
        unported!("SwitchPatternMatchProcessor.getIndex")
    }

    /// `guards` is Java's `Map<List<Exprent>, Exprent>`, keyed by the content of a case-value
    /// list (`List.equals`/`hashCode`), held here as a snapshot of that list.
    fn eliminate_guard_ref(
        stat: SwitchStatement,
        guards: &mut JHashMap<Vec<Option<Exprent>>, Exprent>,
        reference: &Pair<Statement, Exprent>,
        simulate: bool,
    ) -> bool {
        unported!("SwitchPatternMatchProcessor.eliminateGuardRef")
    }

    fn identify_switch_record_pattern_match(
        stat: SwitchStatement,
        statement: Statement,
        simulate: bool,
    ) -> Option<PatternExprent> {
        let head = statement.get_basichead();
        let exprents = head.get_exprents()?;
        if exprents.is_empty() {
            return None;
        }

        let first = exprents.get(0);

        let assignment = first.try_as_assignment()?;

        let assigned = assignment.get_left().try_as_var()?;
        let pattern = IfPatternMatchProcessor::identify_record_pattern_match(
            stat.0,
            statement,
            assigned.0,
            &assigned.get_expr_type(),
            simulate,
        );
        if pattern.is_some() && !simulate {
            head.get_exprents().unwrap().remove_at(0);
        }
        pattern
    }

    fn is_switch_pattern_match(head: SwitchHeadExprent) -> bool {
        let value = head.get_value();

        if let Some(invoc) = value.try_as_invocation() {
            return invoc.get_invocation_type() == InvocationType::Dynamic
                && (invoc.get_name() == "typeSwitch" || invoc.get_name() == "enumSwitch");
        }

        false
    }

    pub fn has_pattern_match(root: RootStatement) -> bool {
        root.mt().get_bytecode_version().has_switch_pattern_match()
            && DecompilerContext::get_option(IFernflowerPreferences::PATTERN_MATCHING)
    }

    fn is_pattern_matching_cast_assignment(
        switch_head: SwitchHeadExprent,
        assignment: AssignmentExprent,
    ) -> bool {
        let right = assignment.get_right();
        if let Some(switch_head_ref) = right.try_as_var() {
            if switch_head.contains_var(switch_head_ref.get_var_version_pair()) {
                return true;
            }
        } else if let Some(function_exprent) = right.try_as_function() {
            if function_exprent.get_func_type() != FunctionType::Cast {
                return false;
            }

            let lst_operands = function_exprent.get_lst_operands();
            if lst_operands.len() < 2 {
                return false;
            }
            let Some(switch_head_ref) = lst_operands.get(0).try_as_var() else {
                return false;
            };
            if !lst_operands.get(1).is_const() {
                return false;
            }
            if !switch_head.contains_var(switch_head_ref.get_var_version_pair()) {
                return false;
            }
            return true;
        }
        false
    }
}
