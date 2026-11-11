//! kuna fix `kt_partial_inline` (spec 10.16): in Kotlin output, a variable
//! that stays is read at each of its uses.
//!
//! `StackVarsProcessor` replaces each use of a variable it can by the
//! variable's value and deletes the variable when no use is left; the
//! Kotlin plugin runs it over every local (`INLINE_ALL_VARS`). When a use
//! cannot take the value (the variable meets another assignment at a join,
//! a later use is past a change of what the value reads), the variable
//! stays and only some uses read it: `var r = args; if (args == null) r =
//! ..`, `val snapshot = entry; ..; closeQuietly(entry)`,
//! `val sink = s as Sink; (s as Sink).write(..)`. The value is computed
//! again or read from another variable than the one the bytecode reads, and
//! a null check of the copy no longer smart-casts the variable, so kotlinc
//! rejects its uses. With the fix a variable that stays (a use the value
//! cannot move to, or a definition that does not reach all of them) is
//! read at every use, as the bytecode reads it; a variable whose uses all
//! take the value is inlined as before.

use crate::kuna_fixes::{self, KunaFix};
use crate::p3_dataflow::kuna_fix_kt_field_read_reuse::is_kotlin_class;

/// Whether a variable that stays keeps all of its uses.
pub fn keeps_variable() -> bool {
    kuna_fixes::enabled(KunaFix::KtPartialInline) && is_kotlin_class()
}
