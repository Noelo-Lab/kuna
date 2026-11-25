//! (kuna `loopcondhoist`) Which exit of a loop is its condition.
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
//! An arm into a block that ends in a call that does not return is not an exit
//! for this purpose, so `for (;;) { …; if (n != len) error(); }` stays one.
//!
//! Three passes act on it.  `CollapseStructure` takes the condition's exit as
//! the loop exit and gives the condition block's arm to `ruleBlockIfNoExit`
//! last; the region structurer keeps the condition's exit edge structural; and
//! `ActionReturnSplit` does not copy a return block for a jump that is a
//! `break` to it, so both paths still meet there and the jump prints as
//! `break`.  Each is a different nesting of the same blocks.  A structuring
//! that prints more `goto`s ([`printed_gotos`]) than the structurers' own
//! choice is not kept.

use kuna_base::error::{KunaError, KunaResult};

use crate::block::{BlockGraph, BlockType};
use crate::context::BlockId;

/// The `loopcondhoist` mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LoopCondHoist {
    /// The structurers' own choice, as upstream.
    #[default]
    Off,
    /// The condition, in functions `kuna_unoptimized` calls unoptimized.
    Auto,
    /// The condition, in every function.
    On,
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

impl LoopCondHoist {
    /// The rule for a function with this verdict, if any.
    pub fn rule(self, unoptimized: bool) -> Option<Rule> {
        match (self, unoptimized) {
            (LoopCondHoist::Off, _) | (LoopCondHoist::Auto, false) => None,
            (_, true) => Some(Rule::Layout),
            (LoopCondHoist::On, false) => Some(Rule::Prefer),
        }
    }
}

/// The `loopcondhoist off|auto|on` ArchOption.
pub struct OptionLoopCondHoist;

impl OptionLoopCondHoist {
    /// The option name.
    pub const NAME: &'static str = "loopcondhoist";

    /// Parse `off`/`auto`/`on`, returning the mode + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(LoopCondHoist, String)> {
        let mode = match p1 {
            "off" => LoopCondHoist::Off,
            "auto" => LoopCondHoist::Auto,
            "on" | "" => LoopCondHoist::On,
            _ => return Err(KunaError::parse("loopcondhoist takes off, auto or on")),
        };
        let prop = match mode {
            LoopCondHoist::Off => "off",
            LoopCondHoist::Auto => "auto (unoptimized functions)",
            LoopCondHoist::On => "on",
        };
        Ok((mode, format!("Loop condition hoist turned {prop}")))
    }
}

/// The rule for `fd`, if its loops' conditions are followed.
pub fn rule_for(fd: &crate::funcdata::Funcdata) -> Option<Rule> {
    fd.get_arch().loop_cond_hoist.rule(crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd))
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
/// code that follows the innermost loop around it, with no `switch` between?
pub fn is_loop_break(sb: &BlockGraph, bl: BlockId) -> bool {
    let target = match sb.block(bl).get_type() {
        BlockType::Goto => sb.block(bl).get_goto_target(),
        BlockType::If => sb.block(bl).get_if_goto_target(),
        _ => None,
    };
    let Some(target) = target.and_then(|t| sb.get_front_leaf(t)) else {
        return false;
    };
    let Some(l) = enclosing_loop(sb, bl) else {
        return false;
    };
    let mut cur = bl;
    while let Some(p) = sb.block(cur).get_parent() {
        if p == l {
            break;
        }
        if sb.block(p).get_type() == BlockType::Switch {
            return false;
        }
        cur = p;
    }
    sb.block(l).get_parent().is_some_and(|outer| sb.next_flow_after(outer, l) == Some(target))
}

/// The innermost loop block around `bl`, if any.
fn enclosing_loop(sb: &BlockGraph, bl: BlockId) -> Option<BlockId> {
    let mut cur = bl;
    while let Some(p) = sb.block(cur).get_parent() {
        if matches!(sb.block(p).get_type(), BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop) {
            return Some(p);
        }
        cur = p;
    }
    None
}

/// How many jumps the structured tree under `root` prints as `goto`: a goto
/// that is not the next statement, not a `break` of its innermost loop and not
/// a `continue` to that loop's start.
pub fn printed_gotos(sb: &BlockGraph, root: BlockId) -> usize {
    use crate::block::{block_flags, BlockKind};
    let mut n = 0;
    let mut stack = vec![root];
    while let Some(bl) = stack.pop() {
        let b = sb.block(bl);
        let target = match b.get_type() {
            BlockType::Goto if sb.goto_prints(bl) && b.get_goto_type() == block_flags::f_goto_goto => {
                b.get_goto_target()
            }
            BlockType::If if b.get_if_goto_type() == block_flags::f_goto_goto => b.get_if_goto_target(),
            BlockType::MultiGoto => {
                if let BlockKind::MultiGoto { gotoedges, .. } = b.kind() {
                    n += gotoedges.len();
                }
                None
            }
            _ => None,
        };
        if let Some(t) = target.and_then(|t| sb.get_front_leaf(t)) {
            let cont = enclosing_loop(sb, bl).and_then(|l| sb.get_front_leaf(l)) == Some(t);
            if !cont && !is_loop_break(sb, bl) {
                n += 1;
            }
        }
        for k in 0..sb.block(bl).get_size() {
            stack.push(sb.block(bl).get_block(k));
        }
    }
    n
}

/// The in-edges of RETURN block `b` that enter a return statement another
/// in-edge enters too, by the unoptimized function's record
/// ([`crate::p8_structure::kuna_o0returns`]).
pub(crate) fn shared_return(fd: &crate::funcdata::Funcdata, b: BlockId) -> Vec<kuna_base::types::int4> {
    use crate::p8_structure::kuna_o0returns::{decided, groups};
    if decided(fd) {
        return (0..fd.bblocks_ref().block(b).size_in()).collect();
    }
    groups(fd, b).map_or_else(Vec::new, |g| g.into_iter().filter(|p| p.len() > 1).flatten().collect())
}

#[cfg(test)]
#[path = "kuna_loopcond/tests.rs"]
mod tests;
