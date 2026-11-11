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
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// Java `name()`.
    pub fn name(self) -> &'static str {
        match self {
            MatchProperties::StatementType => "STATEMENT_TYPE",
            MatchProperties::StatementRet => "STATEMENT_RET",
            MatchProperties::StatementStatsize => "STATEMENT_STATSIZE",
            MatchProperties::StatementExprsize => "STATEMENT_EXPRSIZE",
            MatchProperties::StatementPosition => "STATEMENT_POSITION",
            MatchProperties::StatementIftype => "STATEMENT_IFTYPE",
            MatchProperties::ExprentType => "EXPRENT_TYPE",
            MatchProperties::ExprentRet => "EXPRENT_RET",
            MatchProperties::ExprentPosition => "EXPRENT_POSITION",
            MatchProperties::ExprentFunctype => "EXPRENT_FUNCTYPE",
            MatchProperties::ExprentExittype => "EXPRENT_EXITTYPE",
            MatchProperties::ExprentConsttype => "EXPRENT_CONSTTYPE",
            MatchProperties::ExprentConstvalue => "EXPRENT_CONSTVALUE",
            MatchProperties::ExprentInvocationClass => "EXPRENT_INVOCATION_CLASS",
            MatchProperties::ExprentInvocationSignature => "EXPRENT_INVOCATION_SIGNATURE",
            MatchProperties::ExprentParameter => "EXPRENT_PARAMETER",
            MatchProperties::ExprentVarIndex => "EXPRENT_VAR_INDEX",
            MatchProperties::ExprentName => "EXPRENT_NAME",
        }
    }
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
