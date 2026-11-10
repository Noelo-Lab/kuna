//! kuna fix `kt_delegation_call_arguments`: a constructor's delegation call
//! whose arguments kotlinc computed before a null check is written with
//! them in the call.
//!
//! kotlinc checks a value of a platform type passed where a non-null one is
//! expected right after computing it (`dup`, then
//! `Intrinsics.checkNotNullExpressionValue`, Kotlin 1.3's
//! `checkExpressionValueIsNotNull`). In a constructor that computes an
//! argument of its `this(..)`/`super(..)` call that way, the check ends a
//! statement with the call's receiver and arguments on the stack, so
//! `ExprProcessor` stores them in stack variables, and `SimplifyStack`,
//! which sees two uses of the checked one, leaves them: `val var10001 =
//! Pattern.compile(pattern); this(var10001)`. `ResugarKotlinMethodsPass`
//! then drops the check (Kotlin 1.4's name; 1.3's stays) and the variable
//! stays. Kotlin has no statements before a constructor's delegation:
//! `KConstructor` takes the first statement for the call, a secondary
//! constructor is written `constructor(pattern: String)  {` with the call in
//! its body, a primary one's call goes into an `init` block. Before the pass
//! drops the checks, the stack variables defined before the delegation call
//! in the constructor's first block are put back into the call when that
//! computes the same values in the same order: each is used once apart from
//! the checks, as an argument of the call, the arguments in the order of
//! their definitions; every argument before one of them is a variable or a
//! constant, which the computations do not assign; each check follows its
//! variable's definition, and the parameter checks come first. The checks
//! of those variables are dropped: kotlinc makes the same check of the
//! expression passed in the call. What precedes the call afterwards is
//! parameter checks only, which the pass drops (fix
//! `kt_legacy_constructor_parameter_checks` for Kotlin 1.3's).

use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::pass_context::PassContext;
use crate::statements::Statements;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const CHECK_DESCRIPTOR: &str = "(Ljava/lang/Object;Ljava/lang/String;)V";
const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];
const EXPRESSION_CHECKS: [&str; 2] = [
    "checkNotNullExpressionValue",
    "checkExpressionValueIsNotNull",
];

/// What a statement before the delegation call is.
enum Before {
    ParameterCheck,
    Definition(VarVersionPair, Exprent),
    ExpressionCheck(VarVersionPair),
}

/// Puts the stack variables the delegation call of the constructor `ctx`
/// decompiles is passed back into the call; whether it changed anything.
pub fn inline(ctx: &PassContext) -> bool {
    if ctx.get_method().get_name() != "<init>" {
        return false;
    }
    let Some(root) = ctx.get_root() else {
        return false;
    };
    let Some(exprents) = Statements::find_first_data(root.0).and_then(|s| s.get_exprents()) else {
        return false;
    };
    let this_vars = ctx.get_var_proc().get_this_vars();
    let list = exprents.to_vec();
    let Some(at) = list.iter().position(|e| {
        e.try_as_invocation().is_some_and(|inv| {
            inv.get_name() == "<init>"
                && inv
                    .get_instance()
                    .and_then(|i| i.try_as_var())
                    .is_some_and(|v| {
                        this_vars
                            .borrow()
                            .contains_key(&VarVersionPair::from_var(v))
                    })
        })
    }) else {
        return false;
    };
    let call = list[at].as_invocation();

    let mut before = Vec::new();
    for &e in &list[..at] {
        match classify(e) {
            Some(b) => before.push(b),
            None => return false,
        }
    }
    let mut definitions: Vec<(VarVersionPair, Exprent)> = Vec::new();
    for (i, b) in before.iter().enumerate() {
        match b {
            Before::ParameterCheck => {
                if !definitions.is_empty() {
                    return false;
                }
            }
            Before::Definition(pair, value) => definitions.push((*pair, *value)),
            Before::ExpressionCheck(pair) => {
                let follows = i > 0
                    && matches!(&before[i - 1], Before::Definition(p, _) | Before::ExpressionCheck(p) if p == pair);
                if !follows {
                    return false;
                }
            }
        }
    }
    if definitions.is_empty() {
        return false;
    }

    let args = call.get_lst_parameters().to_vec();
    let position = |pair: VarVersionPair| {
        args.iter().position(|a| {
            a.try_as_var()
                .is_some_and(|v| v.get_var_version_pair() == pair)
        })
    };
    let mut positions = Vec::new();
    for (pair, value) in &definitions {
        if assigns(*value) {
            return false;
        }
        let Some(p) = position(*pair) else {
            return false;
        };
        if positions.last().is_some_and(|&q| q >= p) {
            return false;
        }
        positions.push(p);
    }
    let last = *positions.last().expect("definitions");
    for (p, a) in args.iter().enumerate().take(last) {
        if !positions.contains(&p) && !stable(*a) {
            return false;
        }
    }
    let pairs: Vec<VarVersionPair> = definitions.iter().map(|(p, _)| *p).collect();
    let checked = before
        .iter()
        .filter(|b| matches!(b, Before::ExpressionCheck(_)))
        .count();
    if uses(ctx, &pairs) != pairs.len() * 2 + checked {
        return false;
    }

    for ((_, value), p) in definitions.iter().zip(&positions) {
        call.get_lst_parameters().set(*p, *value);
    }
    let keep: Vec<Exprent> = list[..at]
        .iter()
        .zip(&before)
        .filter(|(_, b)| matches!(b, Before::ParameterCheck))
        .map(|(e, _)| *e)
        .collect();
    for _ in 0..at {
        exprents.remove_at(0);
    }
    for (i, e) in keep.into_iter().enumerate() {
        exprents.add_at(i, e);
    }
    true
}

fn classify(e: Exprent) -> Option<Before> {
    if let Some(assignment) = e.try_as_assignment() {
        let var = assignment.get_left().try_as_var()?;
        return (is_stack(var) && assignment.get_cond_type().is_none())
            .then(|| Before::Definition(var.get_var_version_pair(), assignment.get_right()));
    }
    let inv = e.try_as_invocation()?;
    if !inv.is_static()
        || inv.get_classname() != INTRINSICS
        || inv.get_string_descriptor() != CHECK_DESCRIPTOR
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    if params.len() != 2
        || !params[1]
            .try_as_const()
            .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::String(_))))
    {
        return None;
    }
    let var = params[0].try_as_var()?;
    let name = inv.get_name();
    if PARAMETER_CHECKS.contains(&name.as_str()) && !is_stack(var) {
        Some(Before::ParameterCheck)
    } else if EXPRESSION_CHECKS.contains(&name.as_str()) && is_stack(var) {
        Some(Before::ExpressionCheck(var.get_var_version_pair()))
    } else {
        None
    }
}

fn is_stack(var: VarExprent) -> bool {
    var.get_index() >= VarExprent::STACK_BASE
}

/// Whether evaluating `e` may assign a variable.
fn assigns(e: Exprent) -> bool {
    std::iter::once(e)
        .chain(e.get_all_exprents_rec(true))
        .any(|x| {
            x.is_assignment()
                || x.try_as_function().is_some_and(|f| {
                    matches!(
                        f.get_func_type(),
                        FunctionType::Imm
                            | FunctionType::Mmi
                            | FunctionType::Ipp
                            | FunctionType::Ppi
                    )
                })
        })
}

/// A variable or a constant: its value does not depend on when it is read
/// among computations that assign no variable.
fn stable(e: Exprent) -> bool {
    e.try_as_var().is_some_and(|v| !is_stack(v)) || e.try_as_const().is_some()
}

/// The number of reads and writes of `pairs` in the method.
fn uses(ctx: &PassContext, pairs: &[VarVersionPair]) -> usize {
    let digraph = FlattenStatementsHelper::build(ctx.get_root().expect("root"));
    let mut n = 0;
    for node in digraph.nodes() {
        for e in node.with(|d| d.exprents.clone()).iter() {
            for x in std::iter::once(e).chain(e.get_all_exprents_rec(true)) {
                if x.try_as_var()
                    .is_some_and(|v| pairs.contains(&v.get_var_version_pair()))
                {
                    n += 1;
                }
            }
        }
    }
    n
}
