//! (kuna `loopexit`) Which exit of a loop is its condition.
//!
//! A loop written `while (c) { … if (x) break; … }` has one natural exit, the
//! condition failing; every other way out is a `break`, a `return` or a `goto`.
//! Structuring keeps one exit edge per loop as the loop's own and turns the
//! others into jumps, and `ruleBlockIfNoExit` folds a test whose arm returns
//! into `if (…) return …;` whenever it gets there first.  When the edge it
//! loses is the condition's, the loop prints as `while (true)` with the
//! condition as an `if` inside it and the code written before a `break` after
//! the loop:
//!
//! ```c
//! while (true) {                 // while (p != NULL) {
//!   if (p == NULL) return v;     //   *c = fgetc(p);
//!   *c = fgetc(p);               //   if (*c != EOF) break;
//!   if (*c != -1) break;         //   ok &= next();
//!   ok &= next();                // }
//! }                              // return ok;
//! return v;
//! ```
//!
//! [`condition`] names the block whose test is the condition:
//!
//! * In unoptimized code ([`Rule::Layout`]) it reads the layout: a compiler
//!   that does not optimize enters a `while` or `for` loop by jumping to its
//!   condition, placed after the body, so a head laid out after every latch is
//!   the condition; a head laid out first is a `do … while` whose latch test is
//!   the condition, or a `for (;;)` when no latch tests.
//! * In optimized code ([`Rule::Prefer`]) a conditional latch is the condition
//!   (a `while` loop is rotated into a guard and a bottom test), else a test at
//!   the head.
//!
//! Three passes act on it.  `CollapseStructure` takes the condition's exit as
//! the loop exit and gives the condition block's arm to `ruleBlockIfNoExit`
//! last; the region structurer keeps the condition's exit edge structural; and
//! `ActionReturnSplit` does not copy a return block for a jump that is a
//! `break` to it, so both paths still meet there and the jump prints as
//! `break`.  Each is a different nesting of the same blocks.

use kuna_base::error::{KunaError, KunaResult};

use crate::block::{BlockGraph, BlockType};
use crate::context::BlockId;

/// The `loopexit` mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LoopExitMode {
    /// The structurers' own choice, as before this option.
    #[default]
    Structure,
    /// The condition, in every function.
    Head,
    /// The condition, in functions `kuna_unoptimized` calls unoptimized.
    Auto,
}

/// How a function's loop conditions are found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// From the unoptimized layout.
    Layout,
    /// A conditional latch, else a head test.
    Prefer,
}

/// The block of a loop whose test is its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
    /// The loop head.
    Head,
    /// The latch at this index.
    Latch(usize),
}

impl LoopExitMode {
    /// The rule for a function with this verdict, if any.
    pub fn rule(self, unoptimized: bool) -> Option<Rule> {
        match (self, unoptimized) {
            (LoopExitMode::Structure, _) | (LoopExitMode::Auto, false) => None,
            (_, true) => Some(Rule::Layout),
            (LoopExitMode::Head, false) => Some(Rule::Prefer),
        }
    }
}

/// The `loopexit structure|head|auto` ArchOption.
pub struct OptionLoopExit;

impl OptionLoopExit {
    /// The option name.
    pub const NAME: &'static str = "loopexit";

    /// Parse `structure`/`head`/`auto`, returning the mode + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(LoopExitMode, String)> {
        let mode = match p1 {
            "structure" | "off" => LoopExitMode::Structure,
            "head" | "on" => LoopExitMode::Head,
            "auto" | "" => LoopExitMode::Auto,
            _ => return Err(KunaError::parse("loopexit takes structure, head or auto")),
        };
        let prop = match mode {
            LoopExitMode::Structure => "structure (the structurer's choice)",
            LoopExitMode::Head => "head (every function)",
            LoopExitMode::Auto => "auto (unoptimized functions)",
        };
        Ok((mode, format!("Loop exit set to {prop}")))
    }
}

/// The condition block of a loop.  `head` is the head's start address and
/// whether it tests (one of its arms leaves the loop); `latches` the same for
/// each block with a back edge to the head.
pub fn condition(rule: Rule, head: (Option<u64>, bool), latches: &[(Option<u64>, bool)]) -> Option<Cond> {
    let (head_addr, head_tests) = head;
    match rule {
        Rule::Layout => {
            let h = head_addr?;
            let addrs: Option<Vec<u64>> = latches.iter().map(|l| l.0).collect();
            let addrs = addrs?;
            if addrs.is_empty() {
                return None;
            }
            if addrs.iter().all(|&a| a < h) {
                return head_tests.then_some(Cond::Head);
            }
            match latches {
                [(Some(a), true)] if *a > h => Some(Cond::Latch(0)),
                _ => None,
            }
        }
        Rule::Prefer => {
            let mut testing = latches.iter().enumerate().filter(|(_, l)| l.1);
            match (testing.next(), testing.next()) {
                (Some((i, _)), None) => Some(Cond::Latch(i)),
                (None, _) if head_tests => Some(Cond::Head),
                _ => None,
            }
        }
    }
}

/// Is the goto in structured block `bl` a `break`: does its target start the
/// code that follows the innermost loop around it?
pub fn is_loop_break(sb: &BlockGraph, bl: BlockId) -> bool {
    let target = match sb.block(bl).get_type() {
        BlockType::Goto => sb.block(bl).get_goto_target(),
        BlockType::If => sb.block(bl).get_if_goto_target(),
        _ => None,
    };
    let Some(target) = target.and_then(|t| sb.get_front_leaf(t)) else {
        return false;
    };
    let mut cur = bl;
    while let Some(p) = sb.block(cur).get_parent() {
        if matches!(sb.block(p).get_type(), BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop) {
            let Some(outer) = sb.block(p).get_parent() else {
                return false;
            };
            return sb.next_flow_after(outer, p) == Some(target);
        }
        cur = p;
    }
    false
}

#[cfg(test)]
#[path = "kuna_loopexit/tests.rs"]
mod tests;
