//! kuna fix `type_switch_label_lookup` (`kuna_fixes::KunaFix::TypeSwitchLabelLookup`).
//!
//! A pattern switch's `lookupswitch` keys are indices into the bootstrap
//! arguments of `typeSwitch`/`enumSwitch`, and `SwitchPatternMatchProcessor`
//! replaces each index constant among the case values by the label it
//! stands for. Vineflower 1.12.0 finds the case of index `i` by looking for
//! an integer constant equal to `i` in the *first* label of each case, and
//! falls back to case number `i`. A case with several labels (`case 5, 7`,
//! `case E.X, E.Y`, `case Integer _, String _`) keeps the raw index of every
//! label after the first, and an integer label already substituted (`case 1`)
//! is taken for index 1. Here each index constant is found by identity in
//! whichever case holds it, so every label is replaced exactly once.
//!
//! A case with several type patterns (`case Integer _, String _`, Java 22+)
//! then needs two things Vineflower has no notion of. javac compiles it to a
//! guard that tests the selector against each pattern type again and
//! restarts the switch otherwise; that test is javac's, not the source's, and
//! is removed from the case's guard (`strip_multi_pattern_guards`). And such
//! a label cannot bind variables, so its patterns print unnamed, as one label
//! (`unnamed_pattern_types`).

use crate::code_constants::CONSTANT_STRING;
use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_type_switch_enum_label;
use crate::pooled_constant::PooledConstant;
use crate::rt::JList;
use crate::switch_pattern_match_processor::SwitchPatternMatchProcessor;
use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;
use crate::var_type::VarType;

/// Replaces every index constant of `stat`'s case values by its label.
pub fn replace_labels(
    stat: SwitchStatement,
    bootstrap_arguments: &[Option<PooledConstant>],
    real_selector: Exprent,
    is_enum_switch: bool,
) {
    let mut index_labels: Vec<(JList<Option<Exprent>>, Exprent, i32)> = Vec::new();
    for list in stat.get_case_values().iter() {
        for label in list.iter().flatten() {
            if let Some(c) = label.try_as_const() {
                if c.get_expr_type().type_family == TypeFamily::Integer {
                    if let Some(ConstValue::Int(index)) = c.get_value() {
                        index_labels.push((list.clone(), label, index));
                    }
                }
            }
        }
    }

    for (i, bsa) in bootstrap_arguments.iter().enumerate() {
        let targets: Vec<&(JList<Option<Exprent>>, Exprent, i32)> = index_labels
            .iter()
            .filter(|(list, label, index)| {
                *index == i as i32 && list.iter().any(|x| x == Some(*label))
            })
            .collect();
        let new_value = match bsa {
            Some(PooledConstant::Primitive(p)) => {
                if targets.is_empty() {
                    if p.type_ == CONSTANT_STRING {
                        jthrow!(IllegalStateException; "Unexpected case in switch pattern matching");
                    }
                    continue;
                }
                SwitchPatternMatchProcessor::label_value(p, real_selector, is_enum_switch, true)
            }
            Some(PooledConstant::Link(link))
                if !targets.is_empty() && kuna_fixes::enabled(KunaFix::TypeSwitchEnumLabel) =>
            {
                kuna_fix_type_switch_enum_label::resolve_label(link)
            }
            _ => None,
        };
        let Some(new_value) = new_value else {
            continue;
        };
        for (list, label, _) in targets {
            let replaced: Vec<Option<Exprent>> = list
                .to_vec()
                .into_iter()
                .map(|u| {
                    if u == Some(*label) {
                        Some(new_value)
                    } else {
                        u
                    }
                })
                .collect();
            list.set_all(replaced);
        }
    }
}

/// The types of a case's labels when it has several and every one is a type
/// pattern (`selector instanceof T var`).
pub fn unnamed_pattern_types(values: &JList<Option<Exprent>>) -> Option<Vec<VarType>> {
    if values.len() < 2 {
        return None;
    }
    let mut types = Vec::new();
    for value in values.iter() {
        let func = value?.try_as_function()?;
        let operands = func.get_lst_operands();
        if func.get_func_type() != FunctionType::Instanceof
            || operands.len() != 3
            || !operands.get(2).is_var()
        {
            return None;
        }
        types.push(operands.get(1).get_expr_type());
    }
    Some(types)
}

/// Writes a multi-pattern label's patterns, unnamed: `Integer _, String _`.
pub fn append_unnamed(buf: &mut TextBuffer, types: &[VarType]) {
    for (k, type_) in types.iter().enumerate() {
        if k > 0 {
            buf.append(", ");
        }
        buf.append_cast_type_name(type_).append(" _");
    }
}

/// Removes javac's re-test of a multi-pattern case's types from its guard:
/// a guard that is exactly `sel instanceof T1 || ... || sel instanceof Tn`
/// (the case's types, in order) goes, and one that is that test `&&` a rest
/// becomes the rest.
pub fn strip_multi_pattern_guards(stat: SwitchStatement) {
    let case_values = stat.get_case_values();
    let case_guards = stat.get_case_guards();
    for i in 0..case_values.len().min(case_guards.len()) {
        let values = case_values.get(i);
        let Some(types) = unnamed_pattern_types(&values) else {
            continue;
        };
        let Some(guard) = case_guards.get(i) else {
            continue;
        };
        let selector = values
            .get(0)
            .expect("pattern")
            .as_function()
            .get_lst_operands()
            .get(0);
        let is_retest = |c: &Cond| match c {
            Cond::Or(terms) => {
                terms.len() == types.len()
                    && terms
                        .iter()
                        .zip(&types)
                        .all(|(t, ty)| matches!(t, Cond::Atom(e) if is_type_test(*e, selector, ty)))
            }
            _ => false,
        };
        match norm(guard, false) {
            c if is_retest(&c) => {
                case_guards.set(i, None);
            }
            Cond::And(mut terms) if terms.len() >= 2 && is_retest(&terms[0]) => {
                terms.remove(0);
                case_guards.set(i, Some(rebuild(Cond::And(terms))));
            }
            _ => {}
        }
    }
}

enum Cond {
    Atom(Exprent),
    Not(Exprent),
    And(Vec<Cond>),
    Or(Vec<Cond>),
}

fn is_type_test(e: Exprent, selector: Exprent, type_: &VarType) -> bool {
    let Some(func) = e.try_as_function() else {
        return false;
    };
    let operands = func.get_lst_operands();
    func.get_func_type() == FunctionType::Instanceof
        && operands.len() == 2
        && operands.get(0).equals(selector)
        && operands.get(1).get_expr_type().equals(type_)
}

fn norm(e: Exprent, negated: bool) -> Cond {
    if let Some(func) = e.try_as_function() {
        let operands = func.get_lst_operands();
        match func.get_func_type() {
            FunctionType::BoolNot => return norm(operands.get(0), !negated),
            FunctionType::BooleanAnd | FunctionType::BooleanOr => {
                let conjunction = (func.get_func_type() == FunctionType::BooleanAnd) != negated;
                let mut terms = Vec::new();
                for operand in operands.iter() {
                    match (norm(operand, negated), conjunction) {
                        (Cond::And(inner), true) | (Cond::Or(inner), false) => terms.extend(inner),
                        (other, _) => terms.push(other),
                    }
                }
                return if conjunction {
                    Cond::And(terms)
                } else {
                    Cond::Or(terms)
                };
            }
            _ => {}
        }
    }
    if negated {
        Cond::Not(e)
    } else {
        Cond::Atom(e)
    }
}

fn rebuild(c: Cond) -> Exprent {
    match c {
        Cond::Atom(e) => e,
        Cond::Not(e) => FunctionExprent::new_single(FunctionType::BoolNot, e, None).0,
        Cond::And(terms) | Cond::Or(terms) if terms.len() == 1 => {
            rebuild(terms.into_iter().next().expect("term"))
        }
        Cond::And(terms) => fold(FunctionType::BooleanAnd, terms),
        Cond::Or(terms) => fold(FunctionType::BooleanOr, terms),
    }
}

fn fold(func_type: FunctionType, terms: Vec<Cond>) -> Exprent {
    let mut it = terms.into_iter().map(rebuild);
    let first = it.next().expect("term");
    it.fold(first, |acc, e| {
        FunctionExprent::new(func_type, vec![acc, e], None).0
    })
}
