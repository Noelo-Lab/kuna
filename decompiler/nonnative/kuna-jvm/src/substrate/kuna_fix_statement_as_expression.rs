//! kuna fix `statement_as_expression` (`kuna_fixes`): the exprents
//! Java accepts only as statements.
//!
//! Vineflower models `assert`, `return`, `throw`, `yield`, `synchronized` and
//! local variable declarations as exprents, like expressions. Two places put a
//! lone exprent where Java requires an expression: a lambda's expression body
//! (`ClassWriter.classLambdaToJava`, which excludes only a declaration and a
//! `throw`) and a `for` loop's update (`MergeHelper.matchFor`, which excludes
//! only `return` and `throw`). Both let an `assert` through and print text no
//! Java parser accepts; with the fix both ask this module.

use crate::code_type::CodeType;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};

/// Whether `expr` is written as a statement that Java accepts in no
/// expression position: an `assert`, a `return` or `throw`, a `yield`, a
/// monitor (`synchronized`), a statement head, or a local variable declaration.
pub fn is_statement_only(expr: Exprent) -> bool {
    match expr.get_type() {
        ExprentType::Assert
        | ExprentType::Exit
        | ExprentType::Yield
        | ExprentType::Monitor
        | ExprentType::If
        | ExprentType::SwitchHead => true,
        ExprentType::Assignment => expr
            .try_as_assignment()
            .and_then(|a| a.get_left().try_as_var())
            .is_some_and(|v| v.is_definition()),
        ExprentType::Var => expr.try_as_var().is_some_and(|v| v.is_definition()),
        _ => false,
    }
}

/// Whether a lambda whose body is the lone exprent `expr` can be written as
/// `params -> body`: a `return` that has a value is written as that value;
/// every other statement-only exprent needs a block body.
pub fn is_lambda_expression_body(expr: Exprent) -> bool {
    if let Some(exit) = expr.try_as_exit() {
        return exit.get_exit_type() == ExitType::Return
            && exit.get_ret_type().is_none_or(|t| t.type_ != CodeType::Void);
    }
    !is_statement_only(expr)
}
