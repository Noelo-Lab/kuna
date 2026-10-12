//! (kuna) `condstmts off|N` — the most statements a basic block may print ahead of
//! its test when it is folded into a condition operand.
//!
//! `ruleBlockOr` and `ruleBlockWhileDo` absorb a condition block into a `&&`/`||`
//! operand or a `while (...)` header only when `BlockBasic::isComplex` says it is
//! not complex.  That verdict is a `calc_explicit` approximation taken when the
//! function is structured inside the main loop, before variables are merged, so a
//! block of four loads feeding one compare can later print as a five-element comma
//! chain inside the condition.  Which values end up explicit is only known after
//! `ActionMarkImplied`, and the structure is not rebuilt after it.
//!
//! So the check runs at the final `ActionBlockStructure`: every block in a
//! comma-printed position (the right operand of a `BlockCondition`, or the
//! condition of a `while (...)` header) is measured with the printer's own walk,
//! `kuna_condfold::printed_shape`, and a block printing more than N statements
//! before its test is recorded on the `Funcdata`.  The decompile drive then
//! analyzes the function again with those block starts seeded as complex from the
//! first structuring on, so the fold is declined (or a loop head takes the
//! `while (true) { ...; if (c) break; }` form) and every later pass sees the same
//! structure it would have built had the block been complex all along.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::block::BlockType;
use crate::context::BlockId;
use crate::funcdata::Funcdata;

pub const ELEM_CONDSTMTS: ElementId = ElementId::new("condstmts", 4178);

/// The shipped cap.
pub const DEFAULT_CAP: int4 = 3;

/// The largest cap accepted; anything wider is indistinguishable from `off`.
pub const MAX_CAP: int4 = 1_000;

/// How many times the drive re-analyzes one function for the cap.
pub const MAX_REATTEMPTS: usize = 2;

/// Blocks the structurers must treat as complex for the cap: the seeded block
/// starts, plus (once implied flags exist) every block printing more than the cap.
pub fn complex_blocks(data: &Funcdata) -> BTreeSet<BlockId> {
    let mut out = BTreeSet::new();
    let cap = data.get_arch().cond_stmts;
    if cap < 0 {
        return out;
    }
    let seed = data.condstmts_seed();
    let measure = data.is_high_on();
    if seed.is_empty() && !measure {
        return out;
    }
    for i in 0..data.bblocks_get_size() {
        let bb = data.bblocks_get_block(i);
        if (!seed.is_empty() && seed.contains(&data.bblocks_block_start(bb)))
            || (measure && is_overlong(data, bb, cap))
        {
            out.insert(bb);
        }
    }
    out
}

fn is_overlong(data: &Funcdata, bb: BlockId, cap: int4) -> bool {
    crate::p8_structure::kuna_condfold::printed_shape(data, &data.bb_ops(bb))
        .is_some_and(|shape| shape.stmts - 1 > cap)
}

/// Record every block the existing structure prints over the cap in a
/// comma-printed position.  Only meaningful once implied flags exist.
pub fn record_hits(data: &mut Funcdata) {
    let cap = data.get_arch().cond_stmts;
    if cap < 0 || !data.is_high_on() || data.sblocks_get_size() == 0 {
        return;
    }
    let mut hits: Vec<Address> = Vec::new();
    {
        let g = data.sblocks_ref();
        let mut stack = vec![data.sblocks_root()];
        let mut comma: Vec<BlockId> = Vec::new();
        while let Some(bl) = stack.pop() {
            let b = g.block(bl);
            match b.get_type() {
                BlockType::Condition => comma.push(b.get_block(1)),
                BlockType::WhileDo if !b.has_overflow_syntax() => comma.push(b.get_block(0)),
                _ => {}
            }
            for i in 0..b.get_size() {
                stack.push(b.get_block(i));
            }
        }
        while let Some(bl) = comma.pop() {
            let b = g.block(bl);
            match b.get_type() {
                BlockType::Copy => {
                    if let Some(bb) = b.get_copy() {
                        if is_overlong(data, bb, cap) {
                            hits.push(data.bblocks_block_start(bb));
                        }
                    }
                }
                BlockType::Condition => {
                    comma.push(b.get_block(0));
                    comma.push(b.get_block(1));
                }
                _ => {}
            }
        }
    }
    for a in hits {
        data.add_condstmts_hit(a);
    }
}

/// The seed for another analysis of `fd`, or `None` when it recorded nothing new.
pub fn next_seed(fd: &Funcdata) -> Option<BTreeSet<Address>> {
    let hits = fd.condstmts_hits();
    let seed = fd.condstmts_seed();
    if hits.iter().all(|a| seed.contains(a)) {
        return None;
    }
    Some(seed.union(hits).cloned().collect())
}

/// `condfold`'s printed-width budget bounded so it never admits a block the cap
/// would refuse (that budget counts the terminal `CBRANCH`).
pub fn condfold_budget(cond_fold: int4, cond_stmts: int4) -> int4 {
    if cond_fold <= 0 || cond_stmts < 0 {
        return cond_fold;
    }
    cond_fold.min(cond_stmts.saturating_add(1))
}

/// The `condstmts off|N` ArchOption; the parsed value is `-1` for `off`.
pub struct OptionCondStmts;

impl OptionCondStmts {
    pub const NAME: &'static str = "condstmts";

    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        let field = p1.trim();
        if field == "off" {
            return Ok((-1, "Condition-operand statement cap turned off".to_string()));
        }
        let val = match field.parse::<int4>() {
            Ok(v) if !field.is_empty() && field.bytes().all(|b| b.is_ascii_digit()) => v,
            _ => {
                return Err(KunaError::parse(
                    "Must specify off or a decimal statement count",
                ))
            }
        };
        if val > MAX_CAP {
            return Err(KunaError::parse(format!(
                "Bad statement cap: {val} exceeds the {MAX_CAP} ceiling"
            )));
        }
        Ok((
            val,
            format!("Condition operands print at most {val} statements before their test"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_id() {
        assert_eq!(ELEM_CONDSTMTS.get_id(), 4178);
        assert_eq!(ELEM_CONDSTMTS.get_name(), "condstmts");
    }

    #[test]
    fn parses_off_and_counts() {
        assert_eq!(OptionCondStmts.apply("off").unwrap().0, -1);
        assert_eq!(OptionCondStmts.apply("0").unwrap().0, 0);
        assert_eq!(OptionCondStmts.apply("2").unwrap().0, 2);
        assert_eq!(OptionCondStmts.apply(" 7 ").unwrap().0, 7);
        for bad in ["", "on", "-1", "2.5", "1001"] {
            assert!(OptionCondStmts.apply(bad).is_err(), "{bad:?} must be refused");
        }
    }

    #[test]
    fn condfold_budget_is_bounded_by_the_cap() {
        assert_eq!(condfold_budget(0, 2), 0);
        assert_eq!(condfold_budget(5, -1), 5);
        assert_eq!(condfold_budget(5, 2), 3);
        assert_eq!(condfold_budget(9, 0), 1);
        assert_eq!(condfold_budget(5, 10), 5);
    }
}
