//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/Pattern.java`.
//!
//! The interface has exactly two implementations, both exprents, so it is a
//! closed enum: `(Pattern) e` is [`Pattern::cast`], `e instanceof Pattern p` is
//! [`Pattern::try_cast`].

use crate::exprent::Exprent;
use crate::pattern_exprent::PatternExprent;
use crate::var_exprent::VarExprent;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pattern {
    Var(VarExprent),
    Pattern(PatternExprent),
}

impl Pattern {
    /// `e instanceof Pattern`.
    pub fn instance_of(e: Exprent) -> bool {
        e.is_var() || e.is_pattern()
    }

    /// `e instanceof Pattern p ? p : null`.
    pub fn try_cast(e: Exprent) -> Option<Pattern> {
        if let Some(v) = e.try_as_var() {
            Some(Pattern::Var(v))
        } else {
            e.try_as_pattern().map(Pattern::Pattern)
        }
    }

    /// `(Pattern) e`.
    #[track_caller]
    pub fn cast(e: Exprent) -> Pattern {
        match Pattern::try_cast(e) {
            Some(p) => p,
            None => crate::rt::exception::throw("ClassCastException", "Exprent is not a Pattern"),
        }
    }

    pub fn as_exprent(self) -> Exprent {
        match self {
            Pattern::Var(v) => v.0,
            Pattern::Pattern(p) => p.0,
        }
    }

    pub fn get_pattern_vars(self) -> Vec<VarExprent> {
        match self {
            Pattern::Var(v) => VarExprent::get_pattern_vars(v),
            Pattern::Pattern(p) => PatternExprent::get_pattern_vars(p),
        }
    }
}
