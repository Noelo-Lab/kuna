//! Fix family `kt_jumps` (Kotlin output): `break`, `continue` and `return` in
//! Kotlin's terms: a `return` printed in a declaration but not in a lambda, the
//! exits of a singleton lambda labelled, a jump out of a `when` branch or a
//! sequence written as Kotlin can, and jumps to a label that is no loop
//! (`kt_jump_to_non_loop_label`).

pub(crate) mod kt_jump_to_non_loop_label;

use std::cell::Cell;

use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use self::kt_jump_to_non_loop_label::Lambda;
use crate::k_exit_exprent::KExitExprent;
use crate::method_wrapper::MethodWrapper;
use crate::root_statement::RootStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::text_buffer::TextBuffer;

thread_local! {
    static DECLARATION: Cell<Option<MethodWrapper>> = const { Cell::new(None) };
}

/// Restores the enclosing declaration's wrapper when dropped.
pub struct DeclarationScope(Option<MethodWrapper>);

impl Drop for DeclarationScope {
    fn drop(&mut self) {
        DECLARATION.with(|c| c.set(self.0));
    }
}

/// Opens a declaration boundary: the method wrapper current now is the
/// declaration's own, until the returned guard is dropped.
pub fn enter_declaration() -> DeclarationScope {
    let current = DecompilerContext::get_current_method_wrapper();
    DeclarationScope(DECLARATION.with(|c| c.replace(current)))
}

/// Whether `current`, the installed method wrapper, was installed by a lambda
/// writer inside the innermost declaration rather than by the declaration.
pub fn is_lambda_body(current: MethodWrapper) -> bool {
    DECLARATION.with(Cell::get) != Some(current)
}

/// The `return` exits of a lambda body: the exit statements that break to
/// its dummy exit, the set `KNewExprent` labels.
fn exits(root: RootStatement) -> Vec<(KExitExprent, Statement)> {
    let mut exits = Vec::new();
    for edge in root
        .get_dummy_exit()
        .0
        .get_predecessor_edges(StatEdge::TYPE_BREAK)
    {
        let source = edge.get_source();
        let Some(exprents) = source.get_exprents() else {
            continue;
        };
        for expr in exprents.iter() {
            if let Some(exit) = expr.try_as_k_exit() {
                if exit.get_exit_type() == ExitType::Return {
                    exits.push((exit, source));
                }
            }
        }
    }
    exits
}

/// The label of the lambda class `class_name`: `lambda_` and its name's last
/// `$` segment (`lambda_1` for `Foo$bar$1`).
fn label(class_name: &str) -> String {
    let simple = class_name.rsplit(['/', '$']).next().unwrap_or_default();
    let simple: String = simple
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("lambda_{simple}")
}

/// Whether the value of an exit in `stat` is the lambda's result when
/// written bare: `stat` ends the body, through sequences that end with it and
/// `if`-`else` statements, the value of the branch it ends.
pub(crate) fn in_tail_position(stat: Statement) -> bool {
    let mut current = stat;
    loop {
        let Some(parent) = current.get_parent() else {
            return false;
        };
        match parent.type_() {
            StatementType::Root => return true,
            StatementType::Sequence => {
                if parent.get_stats().borrow().get_last() != current {
                    return false;
                }
            }
            StatementType::If => {
                let if_stat = parent.as_if();
                if if_stat.get_elsestat().is_none()
                    || (if_stat.get_ifstat() != Some(current)
                        && if_stat.get_elsestat() != Some(current))
                {
                    return false;
                }
            }
            _ => return false,
        }
        current = parent;
    }
}

/// Labels the lambda whose header is in `buf` and whose body is `root` when
/// an exit of the body is not in tail position, so each exit writes
/// `return@label`.
pub fn label_exits(buf: &mut TextBuffer, root: RootStatement, class_name: &str) {
    let exits = exits(root);
    if exits.iter().any(|(_, stat)| !in_tail_position(*stat)) {
        let label = label(class_name);
        buf.prepend(&format!("{label}@"));
        for (exit, _) in exits {
            exit.set_lambda_name(Some(label.clone()));
        }
    }
}

/// How the jump along `edge`, written after `site`, is written when it is an
/// unlabelled `break` out of a `when`: `Some("")` when it is left out,
/// `Some("return@labelN")` when the `when` is written in `run labelN@{ .. }`.
pub fn when_jump(edge: StatEdge, site: Statement) -> Option<String> {
    let switch = when_closure(edge)?;
    if ends_branch(edge, site, switch) {
        return Some(String::new());
    }
    kt_jump_to_non_loop_label::written_as(switch, Lambda::Run)
        .then(|| format!("return@label{}", switch.id()))
}

/// Whether the unlabelled `when` `switch` must be written in a
/// `run labelN@{ .. }` for a `break` out of it that does not end its branch.
pub fn needs_run(switch: Statement) -> bool {
    let mut stack: Vec<(Statement, bool)> = switch
        .as_k_switch()
        .get_case_statements()
        .to_vec()
        .into_iter()
        .map(|c| (c, true))
        .collect();
    while let Some((stat, wrapped)) = stack.pop() {
        let mut jumps = Vec::new();
        if wrapped {
            let succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            if succs.len() == 1 && !succs[0].get_destination().is_dummy_exit() {
                jumps.push(succs[0]);
            }
        }
        if let Some(stat_if) = stat.try_as_if() {
            if stat_if.get_ifstat().is_none() {
                jumps.extend(stat_if.get_if_edge());
            }
        }
        if jumps
            .into_iter()
            .any(|edge| when_closure(edge) == Some(switch) && !ends_branch(edge, stat, switch))
        {
            return true;
        }
        let head = matches!(
            stat.type_(),
            StatementType::If | StatementType::Switch | StatementType::Synchronized
        )
        .then(|| stat.get_first())
        .flatten();
        let children = stat.get_stats().borrow().to_vec();
        stack.extend(children.into_iter().map(|c| (c, Some(c) != head)));
    }
    false
}

fn when_closure(edge: StatEdge) -> Option<Statement> {
    if edge.get_type() != StatEdge::TYPE_BREAK || edge.labeled() || !edge.explicit() {
        return None;
    }
    edge.closure().filter(|c| c.is_k_switch())
}

/// Whether `edge` is the jump `KIfStatement` writes as the only statement of
/// the `if` `stat`.
fn is_if_edge(edge: StatEdge, stat: Statement) -> bool {
    stat.try_as_if()
        .is_some_and(|stat| stat.get_if_edge() == Some(edge) && stat.get_ifstat().is_none())
}

/// Whether control at the jump along `edge`, written after `site` (or as the
/// only statement of the `if` `site`), reaches the end of its branch of
/// `switch` and nothing else when the jump is left out.
fn ends_branch(edge: StatEdge, site: Statement, switch: Statement) -> bool {
    let cases = switch.as_k_switch().get_case_statements().to_vec();
    if is_if_edge(edge, site) && writes_jump(site, cases.contains(&site)) {
        return false;
    }
    let mut stack: Vec<(Statement, bool)> = cases.into_iter().map(|c| (c, true)).collect();
    while let Some((stat, case)) = stack.pop() {
        if stat == site {
            return true;
        }
        if writes_jump(stat, case) {
            continue;
        }
        match stat.type_() {
            StatementType::Sequence => {
                let stats = stat.get_stats();
                let last = stats.borrow().to_vec().last().copied();
                stack.extend(last.map(|s| (s, false)));
            }
            StatementType::If => {
                let stat_if = stat.try_as_if().expect("if");
                stack.extend(stat_if.get_ifstat().map(|s| (s, false)));
                stack.extend(stat_if.get_elsestat().map(|s| (s, false)));
            }
            StatementType::TryCatch => {
                stack.extend(
                    stat.get_stats()
                        .borrow()
                        .to_vec()
                        .into_iter()
                        .map(|s| (s, false)),
                );
            }
            StatementType::CatchAll => stack.extend(stat.get_first().map(|s| (s, false))),
            _ => {}
        }
    }
    false
}

/// Whether `jmpWrapper` writes a jump after `stat` (in a case, an unlabelled
/// `break` is not written).
fn writes_jump(stat: Statement, case: bool) -> bool {
    let succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    succs.len() == 1 && {
        let edge = succs[0];
        edge.get_type() != StatEdge::TYPE_REGULAR
            && edge.explicit()
            && !edge.get_destination().is_dummy_exit()
            && !(case && edge.get_type() == StatEdge::TYPE_BREAK && !edge.labeled())
    }
}

/// Whether the `break` along `edge` is written `return@run`/`return@labelN`
/// from the sequence it closes.
pub fn returns_from_sequence(edge: StatEdge) -> bool {
    edge.closure().is_some_and(|c| c.is_k_sequence()) && edge.labeled()
}
