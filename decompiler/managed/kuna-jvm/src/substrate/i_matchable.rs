//! Port of `org/jetbrains/java/decompiler/struct/match/IMatchable.java`.

use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum MatchProperties {
    StatementType,
    StatementRet,
    StatementStatsize,
    StatementExprsize,
    StatementPosition,
    StatementIftype,

    ExprentType,
    ExprentRet,
    ExprentPosition,
    ExprentFunctype,
    ExprentExittype,
    ExprentConsttype,
    ExprentConstvalue,
    ExprentInvocationClass,
    ExprentInvocationSignature,
    ExprentParameter,
    ExprentVarIndex,
    ExprentName,
}

impl MatchProperties {
}

impl JavaHash for MatchProperties {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for MatchProperties {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Implemented by `Statement` and `Exprent`.
pub trait IMatchable {
    fn find_object(&self, match_node: &MatchNode, index: i32) -> Option<Box<dyn IMatchable>>;

    fn match_(&self, match_node: &MatchNode, engine: &MatchEngine) -> bool;
}
