//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IfPatternMatchProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::assignment_exprent::AssignmentExprent;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::if_helper::IfHelper;
use crate::if_statement::IfStatement;
use crate::pattern_exprent::PatternExprent;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JavaEq, JavaHash};
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_record_component::StructRecordComponent;
use crate::unported;
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
        unported!("IfPatternMatchProcessor.handleIf")
    }

    fn check_branch(
        exprent: Exprent,
        statement: IfStatement,
        branch: Statement,
        root: RootStatement,
    ) -> bool {
        unported!("IfPatternMatchProcessor.checkBranch")
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
        unported!("IfPatternMatchProcessor.findPatternMatchingInstanceof")
    }

    fn find_vars_in_predecessors(vvs: &mut Vec<VarVersionPair>, root: Statement) {
        unported!("IfPatternMatchProcessor.findVarsInPredecessors")
    }

    fn identify_if_record_pattern_match(
        stat: IfStatement,
        branch: Statement,
        inst_of: FunctionExprent,
        head: AssignmentExprent,
    ) -> bool {
        unported!("IfPatternMatchProcessor.identifyIfRecordPatternMatch")
    }

    pub fn identify_record_pattern_match(
        parent: Statement,
        branch: Statement,
        store_variable: Exprent,
        type_: &VarType,
        simulate: bool,
    ) -> Option<PatternExprent> {
        unported!("IfPatternMatchProcessor.identifyRecordPatternMatch")
    }

    /// `remove` is keyed by the `BasicBlockStatement` (identity), held as its base handle.
    #[allow(clippy::too_many_arguments)]
    fn get_child_pattern(
        cl: Option<StructClass>,
        store_variable: Exprent,
        type_: &VarType,
        branch: Statement,
        st_idx: i32,
        to_destroy: &mut Vec<Statement>,
        remove: &mut JHashMap<Statement, Vec<Exprent>>,
        remove_edge: &mut Vec<StatEdge>,
    ) -> Option<PatternData> {
        unported!("IfPatternMatchProcessor.getChildPattern")
    }

    pub fn is_statement_match_throw(st: Statement) -> bool {
        if st.is_basic_block() && st.get_exprents().unwrap().len() == 1 {
            if let Some(exit) = st.get_exprents().unwrap().get(0).try_as_exit() {
                if exit.get_exit_type() == ExitType::Throw {
                    if let Some(new_ex) = exit.get_value().and_then(|v| v.try_as_new()) {
                        return new_ex.get_new_type().value.as_deref().unwrap()
                            == "java/lang/MatchException";
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
        unported!("IfPatternMatchProcessor.getLastExprentWhen")
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
        unported!("IfPatternMatchProcessor.improvePatternType")
    }
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
