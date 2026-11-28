//! Fix `var_merge_captured`: a variable a lambda captures must be
//! effectively final (JLS 4.12.4), so merging it with another variable of
//! its slot gives it a second assignment javac rejects: two blocks that
//! each declare an `o` and pass `() -> o.hashCode()` were printed as one
//! `Object o = ..; .. o = ..;`. A variable that is a captured argument of a
//! lambda in the method is not merged.

use crate::fixes::util::all_exprents;
use crate::statement::Statement;
use crate::var_version_pair::VarVersionPair;

/// Whether `from` or `to` is a captured argument of a lambda created in
/// `root`'s tree.
pub fn captured_apart(root: Statement, from: VarVersionPair, to: VarVersionPair) -> bool {
    all_exprents(root).iter().any(|e| {
        e.try_as_new().filter(|n| n.is_lambda() && !n.is_method_reference()).and_then(|n| n.get_constructor()).is_some_and(
            |constructor| {
                constructor.get_lst_parameters().iter().any(|p| {
                    p.try_as_var().map(VarVersionPair::from_var).is_some_and(|pair| pair.equals(&from) || pair.equals(&to))
                })
            },
        )
    })
}
