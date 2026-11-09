//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchExpressionHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::JHashMap;
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;
use crate::unported;
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
        unported!("SwitchExpressionHelper.processStatement")
    }

    fn find_replacements(
        stat: Statement,
        var: VarVersionPair,
        replacements: &mut JHashMap<Exprent, YieldExprent>,
    ) {
        unported!("SwitchExpressionHelper.findReplacements")
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
        unported!("SwitchExpressionHelper.findNextData")
    }

    /// `None` when a block ends in a non-throw exit; a `None` value marks a block that throws.
    fn map_assignments(
        break_jumps: &[Statement],
    ) -> Option<JHashMap<Statement, Option<Vec<VarVersionPair>>>> {
        unported!("SwitchExpressionHelper.mapAssignments")
    }

    fn find_relevant_var(
        assignments: &JHashMap<Statement, Option<Vec<VarVersionPair>>>,
    ) -> Option<VarExprent> {
        unported!("SwitchExpressionHelper.findRelevantVar")
    }

    pub fn has_switch_expressions(statement: RootStatement) -> bool {
        statement
            .mt()
            .get_bytecode_version()
            .has_switch_expressions()
            && DecompilerContext::get_option(IFernflowerPreferences::SWITCH_EXPRESSIONS)
    }
}
