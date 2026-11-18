//! kuna fix `kt_not_null_assertion` (spec 10.16): kotlinc's code for `x!!` is
//! written `x!!`.
//!
//! kotlinc compiles `e!!` to `e; dup; ifnonnull L; Intrinsics.throwNpe(); L:`
//! (Kotlin 1.3) or `e; dup; Intrinsics.checkNotNull(e)` (1.4 and later), so the
//! decompiled method holds the value in a stack variable, then
//! `if (v == null) { Intrinsics.throwNpe(); }` or `Intrinsics.checkNotNull(v);`,
//! then the use. Vineflower's Kotlin plugin deletes `checkNotNull` and keeps
//! `throwNpe`; either way the expression loses the `!!` that gave it its
//! non-null type, and kotlinc rejects the use of the nullable value (or of
//! the stack variable declared non-null). Before the last stack-variable
//! pass, a check is folded back into the expression: `v = e` right before it
//! becomes `v = e!!` (which the pass then inlines into the use), and a check
//! of a value read again by the next statement (a local, or a field the
//! structurer copied into both places) puts the `!!` on that read when it is
//! the first thing the statement evaluates that may have an effect.

use crate::do_statement::Type as DoType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::k_function_exprent::{KFunctionExprent, KFunctionType};
use crate::root_statement::RootStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";

/// Folds every `!!` check under `root` into its expression.
pub fn recover(root: RootStatement) -> bool {
    let mut res = false;
    visit(root.0, &mut res);
    res
}

fn visit(stat: Statement, res: &mut bool) {
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats.into_iter().rev() {
        visit(st, res);
    }
    if let Some(exprents) = stat.get_exprents() {
        let mut i = exprents.len();
        while i > 0 {
            i -= 1;
            let Some(value) = check_not_null_operand(exprents.get(i)) else {
                continue;
            };
            let prev = if i > 0 { Some(exprents.get(i - 1)) } else { last_exprent_before(stat) };
            let next = if i + 1 < exprents.len() {
                Some(exprents.get(i + 1))
            } else {
                next_exprent(stat)
            };
            if fold(value, prev, next) {
                exprents.remove_at(i);
                *res = true;
            }
        }
    }
    let Some(ifstat) = stat.try_as_if() else {
        return;
    };
    let Some(value) = throw_npe_check(ifstat) else {
        return;
    };
    let first = ifstat.get_first().expect("if head");
    let prev = match first.get_exprents() {
        Some(e) if !e.is_empty() => Some(e.get(e.len() - 1)),
        _ => last_exprent_before(stat),
    };
    let next = next_exprent(stat);
    if fold(value, prev, next) {
        remove_check(ifstat, first);
        *res = true;
    }
}

/// `v` of a statement `Intrinsics.checkNotNull(v)`.
fn check_not_null_operand(expr: Exprent) -> Option<Exprent> {
    let inv = expr.try_as_invocation()?;
    (inv.is_static()
        && inv.get_classname() == INTRINSICS
        && inv.get_name() == "checkNotNull"
        && inv.get_string_descriptor() == "(Ljava/lang/Object;)V")
        .then(|| inv.get_lst_parameters().get(0))
}

/// `v` of `if (v == null) { Intrinsics.throwNpe(); }` (no else).
fn throw_npe_check(stat: IfStatement) -> Option<Exprent> {
    if stat.iftype() != IfStatement::IFTYPE_IF || stat.get_elsestat().is_some() {
        return None;
    }
    let body = stat.get_ifstat()?;
    if body.type_() != StatementType::BasicBlock {
        return None;
    }
    let exprents = body.get_exprents()?;
    let throws = exprents.len() == 1
        && exprents.get(0).try_as_invocation().is_some_and(|inv| {
            inv.is_static()
                && inv.get_classname() == INTRINSICS
                && inv.get_name() == "throwNpe"
                && inv.get_string_descriptor() == "()V"
        });
    if !throws {
        return None;
    }
    let cond = stat.get_headexprent().get_condition();
    let func = cond.try_as_function().filter(|f| f.get_func_type() == FunctionType::Eq)?;
    let ops = func.get_lst_operands();
    let is_null = |e: Exprent| e.try_as_const().is_some_and(|c| c.is_null());
    match (is_null(ops.get(0)), is_null(ops.get(1))) {
        (false, true) => Some(ops.get(0)),
        (true, false) => Some(ops.get(1)),
        _ => None,
    }
}

/// Puts the `!!` of a check of `value` on `prev` (`v = e`, `v` a stack
/// variable that is the checked value or holds the field read it copies) or on
/// the first read of `value` in `next`.
fn fold(value: Exprent, prev: Option<Exprent>, next: Option<Exprent>) -> bool {
    if !is_read(value) {
        return false;
    }
    let assign = prev.and_then(|p| p.try_as_assignment()).filter(|a| a.get_cond_type().is_none());
    let stored = assign.filter(|a| {
        a.get_left().try_as_var().is_some_and(|left| {
            left.is_stack()
                && match value.try_as_var() {
                    Some(var) => {
                        left.get_index() == var.get_index() && left.get_version() == var.get_version()
                    }
                    None => a.get_right().equals(value),
                }
        })
    });
    if let Some(assign) = stored {
        assign.set_right(not_null(assign.get_right()));
        return true;
    }
    let Some(next) = next else {
        return false;
    };
    let mut asserted = false;
    match find(next, value, None, &mut asserted) {
        Find::Found(read, Some(parent)) => {
            parent.replace_exprent(read, not_null(read));
            true
        }
        _ => asserted,
    }
}

/// Whether `expr` is a `!!`: it may throw, so it is neither copied nor moved.
pub fn is_not_null(expr: Exprent) -> bool {
    expr.try_as_k_function().is_some_and(|k| k.kd(|d| d.k_type) == KFunctionType::NotNull)
}

/// Whether `expr` only reads a value (a local, a field, an array element,
/// a cast of one), so the structurer may have copied it into the check.
pub fn is_read(expr: Exprent) -> bool {
    match expr.get_type() {
        ExprentType::Var | ExprentType::Const => true,
        ExprentType::Field => expr.as_field().get_instance().is_none_or(is_read),
        ExprentType::Array => is_read(expr.as_array().get_array()) && is_read(expr.as_array().get_index()),
        ExprentType::Function => {
            let func = expr.as_function();
            func.get_func_type() == FunctionType::Cast && is_read(func.get_lst_operands().get(0))
        }
        _ => false,
    }
}

fn not_null(expr: Exprent) -> Exprent {
    KFunctionExprent::new_not_null(expr).0 .0
}

enum Find {
    Found(Exprent, Option<Exprent>),
    Clean,
    Dirty,
}

/// The first read of `target` that `expr` evaluates, provided everything it
/// evaluates before has no effect and the read is not conditional. A read
/// that already has its `!!` is passed over and noted in `asserted`: a check
/// of the same value then has its `!!` there already.
fn find(expr: Exprent, target: Exprent, parent: Option<Exprent>, asserted: &mut bool) -> Find {
    if expr.equals(target) {
        return Find::Found(expr, parent);
    }
    let in_order = |operands: Vec<Exprent>, effect: bool, asserted: &mut bool| -> Find {
        for op in operands {
            match find(op, target, Some(expr), asserted) {
                Find::Clean => {}
                other => return other,
            }
        }
        if effect {
            Find::Dirty
        } else {
            Find::Clean
        }
    };
    match expr.get_type() {
        ExprentType::Var | ExprentType::Const => Find::Clean,
        ExprentType::Field => in_order(expr.as_field().get_instance().into_iter().collect(), false, asserted),
        ExprentType::Array => {
            let array = expr.as_array();
            in_order(vec![array.get_array(), array.get_index()], false, asserted)
        }
        ExprentType::Function => {
            let func = expr.as_function();
            let ops = func.get_lst_operands().to_vec();
            match func.get_func_type() {
                FunctionType::Ternary | FunctionType::BooleanAnd | FunctionType::BooleanOr => {
                    match find(ops[0], target, Some(expr), asserted) {
                        Find::Clean => Find::Dirty,
                        other => other,
                    }
                }
                FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi => {
                    in_order(ops, true, asserted)
                }
                FunctionType::Other if is_not_null(expr) => {
                    if ops[0].equals(target) {
                        *asserted = true;
                        return Find::Clean;
                    }
                    match find(ops[0], target, Some(expr), asserted) {
                        Find::Clean => Find::Dirty,
                        other => other,
                    }
                }
                FunctionType::Other => Find::Dirty,
                _ => in_order(ops, false, asserted),
            }
        }
        ExprentType::Invocation => {
            let inv = expr.as_invocation();
            let mut ops: Vec<Exprent> = inv.get_instance().into_iter().collect();
            ops.extend(inv.get_lst_parameters().to_vec());
            in_order(ops, true, asserted)
        }
        ExprentType::Assignment => {
            let assign = expr.as_assignment();
            let left = assign.get_left();
            let mut ops: Vec<Exprent> = Vec::new();
            if !left.is_var() {
                ops.extend(left.get_all_exprents());
            }
            ops.push(assign.get_right());
            in_order(ops, true, asserted)
        }
        ExprentType::Exit => in_order(expr.as_exit().get_value().into_iter().collect(), true, asserted),
        ExprentType::If => in_order(vec![expr.as_if().get_condition()], false, asserted),
        ExprentType::SwitchHead => in_order(vec![expr.as_switch_head().get_value()], false, asserted),
        ExprentType::Monitor => in_order(vec![expr.as_monitor().get_value()], true, asserted),
        _ => Find::Dirty,
    }
}

/// The last exprent run before `stat`: that of the block before it in its sequence.
fn last_exprent_before(stat: Statement) -> Option<Exprent> {
    let parent = stat.get_parent().filter(|p| p.type_() == StatementType::Sequence)?;
    let stats = parent.get_stats().borrow().to_vec();
    let pos = stats.iter().position(|s| *s == stat)?;
    let before = *stats.get(pos.checked_sub(1)?)?;
    if before.type_() != StatementType::BasicBlock {
        return None;
    }
    let exprents = before.get_exprents()?;
    (!exprents.is_empty()).then(|| exprents.get(exprents.len() - 1))
}

/// The exprent run right after `check` completes normally: the first of the
/// next statement of its sequence (or of an enclosing one it ends), or the
/// update or condition of the loop whose body it ends.
fn next_exprent(check: Statement) -> Option<Exprent> {
    let mut edges = check.get_all_successor_edges();
    let mut stat = check;
    loop {
        let parent = stat.get_parent()?;
        match parent.type_() {
            StatementType::Sequence => {
                let stats = parent.get_stats().borrow().to_vec();
                let pos = stats.iter().position(|s| *s == stat)?;
                let Some(&next) = stats.get(pos + 1) else {
                    stat = parent;
                    continue;
                };
                let falls = !edges.is_empty()
                    && edges.iter().all(|e| {
                        e.get_destination() == next
                            && (e.get_type() == StatEdge::TYPE_REGULAR
                                || e.get_type() == StatEdge::TYPE_BREAK)
                    });
                if !falls {
                    return None;
                }
                let empty = next.type_() == StatementType::BasicBlock
                    && next.get_exprents().is_some_and(|e| e.is_empty());
                if empty {
                    edges = next.get_all_successor_edges();
                    stat = next;
                    continue;
                }
                return first_evaluated(next);
            }
            StatementType::Do => {
                let continues = !edges.is_empty()
                    && edges.iter().all(|e| {
                        e.get_type() == StatEdge::TYPE_CONTINUE && e.get_destination() == parent
                    });
                let dostat = parent.try_as_do()?;
                return match dostat.get_looptype() {
                    _ if !continues => None,
                    DoType::For => dostat.get_inc_exprent(),
                    DoType::While | DoType::DoWhile => dostat.get_condition_exprent(),
                    _ => None,
                };
            }
            _ => return None,
        }
    }
}

/// The exprent `stat` evaluates first, when it runs it once on entry.
fn first_evaluated(stat: Statement) -> Option<Exprent> {
    match stat.type_() {
        StatementType::Sequence => {
            let first = stat.get_stats().borrow().to_vec().first().copied();
            first.and_then(first_evaluated)
        }
        StatementType::BasicBlock => {
            let exprents = stat.get_exprents()?;
            (!exprents.is_empty()).then(|| exprents.get(0))
        }
        StatementType::Do => {
            let dostat = stat.try_as_do()?;
            (dostat.get_looptype() == DoType::For).then(|| dostat.get_init_exprent()).flatten()
        }
        StatementType::If | StatementType::Switch | StatementType::Synchronized => {
            let first = stat.get_first()?;
            if let Some(exprents) = first.get_exprents().filter(|e| !e.is_empty()) {
                return Some(exprents.get(0));
            }
            let heads = if let Some(ifstat) = stat.try_as_if() {
                ifstat.get_headexprent_list()
            } else if let Some(switch) = stat.try_as_switch() {
                switch.get_headexprent_list()
            } else {
                stat.try_as_synchronized()?.get_headexprent_list()
            };
            (!heads.is_empty()).then(|| heads.get(0))
        }
        _ => None,
    }
}

/// Replaces the `if` of a folded check by its head block, which keeps the
/// exprents before the condition and takes the `if`'s place and edges.
fn remove_check(stat: IfStatement, first: Statement) {
    let body = stat.get_ifstat().expect("check body");
    for edge in body.get_all_successor_edges() {
        edge.remove();
    }
    for edge in first.get_all_successor_edges() {
        edge.remove();
    }
    first.get_var_definitions().add_all(stat.get_var_definitions().to_vec());
    stat.get_parent().expect("parent").replace_statement(stat.0, first);
}
