//! The kuna fixes `twr_unresolved_resource` and `twr_desugaring_shape`
//! (`kuna_fixes`), called from
//! `TryWithResourcesProcessor::make_try_with_resource_j11`.
//!
//! Vineflower re-sugars javac 11+'s try-with-resources desugaring only when
//! the class of the `close()` call is known to be `AutoCloseable`, which fails
//! when the class, or the supertype that makes it `AutoCloseable`, is outside
//! the input; and when it is known it checks the shape only loosely, so
//! hand-written code that skips a `close()`, reorders the resource's
//! initializer or does something else than `addSuppressed` in the handler is
//! printed as try-with-resources too. [`javac_shape`] is javac's desugaring
//! (`Lower.makeTwrTry`) exactly: the resource is assigned by the last
//! statement before the `try`, which catches `Throwable` `t` with the handler
//! `[if (r != null)] { try { r.close(); } catch (Throwable x) {
//! t.addSuppressed(x); } } throw t;`, every normal exit runs
//! `[if (r != null)] r.close();` and nothing else leaves the body except a
//! `throw`; the null checks are absent exactly when the initializer is a
//! `new`, and the variable is used nowhere else. With `twr_unresolved_resource`
//! a class whose hierarchy kuna cannot resolve is accepted when the shape is
//! javac's; with `twr_desugaring_shape` a known `AutoCloseable` must have
//! javac's shape too. A class whose whole hierarchy is known and does not
//! reach `AutoCloseable` is never accepted. With `twr_desugaring_shape` the
//! resource is written as the field it was copied from (`try (this.f)`) only
//! when the field is final, as Java requires, and the copy is javac's (no
//! name in the local variable table).

use crate::assignment_exprent::AssignmentExprent;
use crate::catch_statement::CatchStatement;
use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

const THROWABLE: &str = "java/lang/Throwable";

/// Whether the resource variable `declaration`, assigned `exp`, can be
/// written as `exp` itself: a variable, or a field declared final when the
/// resource variable has no name in the local variable table (javac's copy
/// for `try (this.f)`).
pub fn is_resource_access(declaration: VarExprent, exp: Exprent) -> bool {
    let Some(field) = exp.try_as_field() else {
        return exp.is_var();
    };
    declaration.get_lvt().is_none()
        && DecompilerContext::get_struct_context()
            .get_class(&field.get_classname())
            .and_then(|cl| {
                cl.get_field(&field.get_name(), &field.get_descriptor().descriptor_string)
            })
            .is_some_and(|fd| fd.has_modifier(ACC_FINAL))
}

/// Whether a class is `AutoCloseable`, as far as its known hierarchy tells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Closeable {
    Yes,
    No,
    Unknown,
}

/// The status of the class of `exp` when it is a `close()V` call on a
/// variable, the calls `is_closeable` looks at; `None` for anything else.
pub fn close_call_status(exp: Exprent) -> Option<Closeable> {
    let invocation = exp.try_as_invocation()?;
    if invocation.get_name() != "close"
        || invocation.get_string_descriptor() != "()V"
        || !invocation.get_instance().is_some_and(|i| i.is_var())
    {
        return None;
    }
    Some(class_status(&invocation.get_classname(), &mut Vec::new()))
}

fn class_status(name: &str, seen: &mut Vec<String>) -> Closeable {
    if name == "java/lang/AutoCloseable" {
        return Closeable::Yes;
    }
    if name == "java/lang/Object" || seen.iter().any(|s| s == name) {
        return Closeable::No;
    }
    seen.push(name.to_string());
    let Some(cl) = DecompilerContext::get_struct_context().get_class(name) else {
        return Closeable::Unknown;
    };
    let mut supers: Vec<String> = cl
        .super_class()
        .map(|s| s.get_string().to_string())
        .into_iter()
        .collect();
    for iface in cl.get_interfaces() {
        let iface = cl
            .get_pool()
            .get_primitive_constant(iface)
            .expect("getPrimitiveConstant(iface)");
        supers.push(iface.get_string().to_string());
    }
    let mut status = Closeable::No;
    for s in supers {
        match class_status(&s, seen) {
            Closeable::Yes => return Closeable::Yes,
            Closeable::Unknown => status = Closeable::Unknown,
            Closeable::No => {}
        }
    }
    status
}

/// Whether the `try_statement` Vineflower is about to turn into
/// try-with-resources on `resource` (assigned by `def` in `def_block`) is
/// javac's desugaring, with the given normal-exit `destinations`.
pub fn javac_shape(
    try_statement: CatchStatement,
    resource: VarExprent,
    nullable: bool,
    def: AssignmentExprent,
    def_block: Statement,
    destinations: &[Statement],
) -> bool {
    let catch_var = try_statement.get_vars().get(0);
    let stats = try_statement.get_stats().borrow().to_vec();
    if stats.len() < 2 || !catches_throwable_only(try_statement, 0) {
        return false;
    }
    let def_exprents = def_block.get_exprents().expect("basic block");
    if def_exprents.is_empty() || def_exprents.get(def_exprents.len() - 1) != def.0 {
        return false;
    }
    if nullable
        == def
            .get_right()
            .try_as_new()
            .is_some_and(|n| !n.is_lambda() && n.get_new_type().array_dim == 0)
    {
        return false;
    }
    if !exceptional_close(stats[1], resource, nullable, catch_var) {
        return false;
    }
    if !destinations
        .iter()
        .all(|d| normal_close(*d, resource, nullable))
    {
        return false;
    }
    if leaves_without_close(stats[0]) {
        return false;
    }
    let mut skip = vec![try_statement.0];
    skip.extend_from_slice(destinations);
    let def_var = def.get_left().as_var();
    !referenced_outside(try_statement.get_top_parent().0, def_var, &skip)
}

fn catches_throwable_only(catch: CatchStatement, index: usize) -> bool {
    let exct = catch.get_exct_strings();
    index < exct.len() && exct.get(index) == vec![THROWABLE.to_string()]
}

fn same_var(exp: Option<Exprent>, var: VarExprent) -> bool {
    exp.and_then(|e| e.try_as_var())
        .is_some_and(|v| v.get_var_version_pair() == var.get_var_version_pair())
}

fn is_close_of(exp: Exprent, resource: VarExprent) -> bool {
    exp.try_as_invocation().is_some_and(|i| {
        !i.is_static()
            && i.get_name() == "close"
            && i.get_string_descriptor() == "()V"
            && i.get_lst_parameters().is_empty()
            && same_var(i.get_instance(), resource)
    })
}

fn single_exprent(stat: Statement) -> Option<Exprent> {
    if !stat.is_basic_block() {
        return None;
    }
    let exprents = stat.get_exprents()?;
    (exprents.len() == 1).then(|| exprents.get(0))
}

/// Whether `cond` is `resource != null` (under any number of `!`).
fn is_non_null_check(cond: Exprent, resource: VarExprent) -> bool {
    let mut negated = false;
    let mut cond = cond;
    loop {
        let Some(func) = cond.try_as_function() else {
            return false;
        };
        match func.get_func_type() {
            FunctionType::BoolNot => {
                negated = !negated;
                cond = func.get_lst_operands().get(0);
            }
            t @ (FunctionType::Ne | FunctionType::Eq) => {
                let operands = func.get_lst_operands();
                return operands.len() == 2
                    && (t == FunctionType::Ne) != negated
                    && same_var(Some(operands.get(0)), resource)
                    && operands.get(1).is_const()
                    && operands
                        .get(1)
                        .get_expr_type()
                        .equals(&VarType::VARTYPE_NULL());
            }
            _ => return false,
        }
    }
}

/// The body of `if (resource != null) { .. }` without an `else` and with no
/// statements before the condition; `stat` itself when `nullable` is false.
fn under_null_check(stat: Statement, resource: VarExprent, nullable: bool) -> Option<Statement> {
    if !nullable {
        return Some(stat);
    }
    let if_stat = stat.try_as_if()?;
    if if_stat.iftype() != IfStatement::IFTYPE_IF || if_stat.get_elsestat().is_some() {
        return None;
    }
    let head = if_stat.get_first()?;
    if !head.get_exprents().is_some_and(|e| e.is_empty()) {
        return None;
    }
    if !is_non_null_check(if_stat.get_headexprent().get_condition(), resource) {
        return None;
    }
    if_stat.get_ifstat()
}

/// `[if (r != null)] { try { r.close(); } catch (Throwable x) { t.addSuppressed(x); } } throw t;`
fn exceptional_close(
    handler: Statement,
    resource: VarExprent,
    nullable: bool,
    t: VarExprent,
) -> bool {
    if !handler.is_sequence() {
        return false;
    }
    let parts = handler.get_stats().borrow().to_vec();
    if parts.len() != 2 {
        return false;
    }
    let rethrows = single_exprent(parts[1])
        .and_then(|e| e.try_as_exit())
        .is_some_and(|e| {
            e.get_exit_type() == exit_exprent::Type::Throw && same_var(e.get_value(), t)
        });
    if !rethrows {
        return false;
    }
    let Some(inner) = under_null_check(parts[0], resource, nullable) else {
        return false;
    };
    let Some(inner) = inner.try_as_catch() else {
        return false;
    };
    let inner_stats = inner.get_stats().borrow().to_vec();
    if inner_stats.len() != 2
        || !catches_throwable_only(inner, 0)
        || !inner.get_resources().is_empty()
        || !single_exprent(inner_stats[0]).is_some_and(|e| is_close_of(e, resource))
    {
        return false;
    }
    let x = inner.get_vars().get(0);
    single_exprent(inner_stats[1])
        .and_then(|e| e.try_as_invocation())
        .is_some_and(|i| {
            !i.is_static()
                && i.get_name() == "addSuppressed"
                && i.get_string_descriptor() == "(Ljava/lang/Throwable;)V"
                && same_var(i.get_instance(), t)
                && i.get_lst_parameters().len() == 1
                && same_var(Some(i.get_lst_parameters().get(0)), x)
        })
}

/// `[if (r != null)] r.close();` at a normal exit of the body.
fn normal_close(stat: Statement, resource: VarExprent, nullable: bool) -> bool {
    if nullable {
        under_null_check(stat, resource, true)
            .and_then(single_exprent)
            .is_some_and(|e| is_close_of(e, resource))
    } else {
        stat.is_basic_block()
            && stat
                .get_exprents()
                .is_some_and(|e| !e.is_empty() && is_close_of(e.get(0), resource))
    }
}

/// Whether a statement of `stat` exits the method other than by `throw` (or
/// a `finally` handler's rethrow).
fn leaves_without_close(stat: Statement) -> bool {
    for edge in stat.get_all_successor_edges() {
        if edge.get_destination().is_dummy_exit() && edge.get_type() != StatEdge::TYPE_FINALLYEXIT {
            let throws = stat.is_basic_block()
                && stat.get_exprents().is_some_and(|e| {
                    !e.is_empty()
                        && e.get(e.len() - 1)
                            .try_as_exit()
                            .is_some_and(|x| x.get_exit_type() == exit_exprent::Type::Throw)
                });
            if !throws {
                return true;
            }
        }
    }
    let children = stat.get_stats().borrow().to_vec();
    children.into_iter().any(leaves_without_close)
}

fn referenced_outside(stat: Statement, var: VarExprent, skip: &[Statement]) -> bool {
    if skip.contains(&stat) {
        return false;
    }
    match stat.get_exprents() {
        Some(exprents) => exprents
            .to_vec()
            .into_iter()
            .any(|e| var.is_var_referenced_exp(e, &[])),
        None => {
            let children = stat.get_stats().borrow().to_vec();
            children
                .into_iter()
                .any(|s| referenced_outside(s, var, skip))
                || stat
                    .get_stat_exprents()
                    .into_iter()
                    .any(|e| var.is_var_referenced_exp(e, &[]))
        }
    }
}
