//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchHelper.java`.
//!
//! The sealed `StringSwitch` interface is the closed enum [`StringSwitch`] over its four
//! permitted records; `instanceof Merged`/`NullableSplit` checks become matches on it.
#![allow(unused_variables, dead_code)]

use std::collections::VecDeque;

use crate::array_exprent::ArrayExprent;
use crate::assignment_exprent::AssignmentExprent;
use crate::code_constants::{ACC_FINAL, ACC_STATIC, ACC_SYNTHETIC};
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JList, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::switch_statement::SwitchStatement;
use crate::unported;
use crate::var_exprent::VarExprent;

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
        unported!("SwitchHelper.simplify")
    }

    fn try_simplify_string_switch(
        switch_stat: SwitchStatement,
        switch_head_value: Exprent,
    ) -> bool {
        unported!("SwitchHelper.trySimplifyStringSwitch")
    }

    fn unwrap_merged_switch_cases(switch_info: StringSwitch) -> bool {
        unported!("SwitchHelper.unwrapMergedSwitchCases")
    }

    fn merge_duplicate_case_stats(switch_info: StringSwitch) {
        unported!("SwitchHelper.mergeDuplicateCaseStats")
    }

    fn find_synthetic_dup_var(switch_info: StringSwitch) -> Option<SyntheticDupVarResult> {
        unported!("SwitchHelper.findSyntheticDupVar")
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
        unported!("SwitchHelper.getStringSwitchCaseMap")
    }

    fn get_string_switch_real_case_values(
        switch_info: StringSwitch,
        case_map: &JHashMap<i32, Vec<Exprent>>,
    ) -> Vec<JList<Option<Exprent>>> {
        unported!("SwitchHelper.getStringSwitchRealCaseValues")
    }

    fn simplify_switch_on_enum_j21(switch_st: SwitchStatement, root: RootStatement) -> bool {
        unported!("SwitchHelper.simplifySwitchOnEnumJ21")
    }

    fn is_enum_array(exprent: Exprent) -> bool {
        unported!("SwitchHelper.isEnumArray")
    }

    fn get_enum_array_exprent(switch_head: Exprent, root: RootStatement) -> Option<ArrayExprent> {
        unported!("SwitchHelper.getEnumArrayExprent")
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
        unported!("SwitchHelper.StringSwitch.isValid")
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
        unported!("SwitchHelper.NullableSplit.match")
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
