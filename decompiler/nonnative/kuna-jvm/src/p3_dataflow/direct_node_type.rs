//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectNodeType.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum DirectNodeType {
    Direct,
    Tail,
    Init,
    Condition,
    Increment,
    Try,
    Catch,
    CombinedCatch,
    Finally,
    FinallyEnd,
    ForeachVardef,
    Case,
}

impl DirectNodeType {
    pub const VALUES: [DirectNodeType; 12] = [
        DirectNodeType::Direct,
        DirectNodeType::Tail,
        DirectNodeType::Init,
        DirectNodeType::Condition,
        DirectNodeType::Increment,
        DirectNodeType::Try,
        DirectNodeType::Catch,
        DirectNodeType::CombinedCatch,
        DirectNodeType::Finally,
        DirectNodeType::FinallyEnd,
        DirectNodeType::ForeachVardef,
        DirectNodeType::Case,
    ];

    pub fn values() -> [DirectNodeType; 12] {
        Self::VALUES
    }

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// Java `Enum.name()`.
    pub fn name(self) -> &'static str {
        match self {
            DirectNodeType::Direct => "DIRECT",
            DirectNodeType::Tail => "TAIL",
            DirectNodeType::Init => "INIT",
            DirectNodeType::Condition => "CONDITION",
            DirectNodeType::Increment => "INCREMENT",
            DirectNodeType::Try => "TRY",
            DirectNodeType::Catch => "CATCH",
            DirectNodeType::CombinedCatch => "COMBINED_CATCH",
            DirectNodeType::Finally => "FINALLY",
            DirectNodeType::FinallyEnd => "FINALLY_END",
            DirectNodeType::ForeachVardef => "FOREACH_VARDEF",
            DirectNodeType::Case => "CASE",
        }
    }

    /// Java: the private `name` field (empty for `DIRECT`, else the lower-cased constant name).
    pub fn lower_name(self) -> &'static str {
        match self {
            DirectNodeType::Direct => "",
            DirectNodeType::Tail => "tail",
            DirectNodeType::Init => "init",
            DirectNodeType::Condition => "condition",
            DirectNodeType::Increment => "increment",
            DirectNodeType::Try => "try",
            DirectNodeType::Catch => "catch",
            DirectNodeType::CombinedCatch => "combined_catch",
            DirectNodeType::Finally => "finally",
            DirectNodeType::FinallyEnd => "finally_end",
            DirectNodeType::ForeachVardef => "foreach_vardef",
            DirectNodeType::Case => "case",
        }
    }

    pub fn make_id(self, stat_id: i32) -> String {
        match self {
            DirectNodeType::Direct => stat_id.to_string(),
            _ => format!("{}_{}", stat_id, self.lower_name()),
        }
    }
}

impl JavaHash for DirectNodeType {
    fn java_hash(&self) -> i32 {
        self.ordinal()
    }
}

impl JavaEq for DirectNodeType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl std::fmt::Display for DirectNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
