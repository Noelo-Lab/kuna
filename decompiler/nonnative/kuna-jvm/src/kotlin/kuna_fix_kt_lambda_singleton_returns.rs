//! kuna fix `kt_lambda_singleton_returns`: a lambda compiled to a singleton
//! class (`Foo$bar$1.INSTANCE`, written by `KFieldExprent`) is labelled and
//! its exits written `return@label v`, as `KNewExprent` writes an
//! `invokedynamic` lambda with several exits, when one of its exits is not in
//! tail position. Vineflower writes every exit of such a lambda as its bare
//! value, which is the lambda's result only at the end of its body (or of
//! both branches of an `if`-`else` there); an early `return@label v` becomes
//! the expression statement `v` and the lambda goes on past it.

use crate::exit_exprent::Type as ExitType;
use crate::k_exit_exprent::KExitExprent;
use crate::root_statement::RootStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::text_buffer::TextBuffer;

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
fn in_tail_position(stat: Statement) -> bool {
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
