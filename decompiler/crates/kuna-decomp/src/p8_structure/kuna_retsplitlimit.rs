//! (kuna) `retsplitlimit` — `ActionReturnSplit` does not clone a return block
//! into more copies than a function can afford.
//!
//! `ActionReturnSplit` gives every goto into a shared RETURN block its own
//! copy of the block, so each path prints its own `return`. The block is
//! cloned whole: every op in it, once per split edge. A bare epilogue is a
//! handful of ops, but a RETURN block also carries one `glob = glob` guard COPY
//! per global the function reads (`Heritage::guardReturns`). serde-hkx's
//! 194 KB deserializer reads about 4,400 globals and has two return blocks of
//! 4,524 ops reached by 674 and 658 goto edges, so splitting them clones 6.0
//! million ops (8.75 million ops in the function right after, 7.19 million of
//! them guard COPYs), and every later pass and the variable merge walk them.
//!
//! With the option set, a return block whose op count times its split edges
//! exceeds the limit keeps its gotos instead. The limit is in cloned ops; a
//! block under it splits exactly as upstream, so the C changes only in the
//! functions that would have cloned more.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;

/// Marshalling element id for the option (kuna 4000+ range).
pub const ELEM_RETSPLITLIMIT: ElementId = ElementId::new("retsplitlimit", 4179);

/// The default limit: a return block is not split into more than this many
/// cloned ops.
pub const DEFAULT_LIMIT: u64 = 1 << 20;

/// The `retsplitlimit off|<n>` option (`Architecture::ret_split_limit`).
pub struct OptionRetSplitLimit;

impl OptionRetSplitLimit {
    /// The option name.
    pub const NAME: &'static str = "retsplitlimit";

    /// Resolve the limit (`None` for `off`) and its confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(Option<u64>, String)> {
        let field = p1.trim();
        if field == "off" {
            return Ok((None, "Return split clone limit turned off".to_string()));
        }
        let n = field
            .parse::<u64>()
            .ok()
            .filter(|_| !field.is_empty() && field.bytes().all(|b| b.is_ascii_digit()))
            .ok_or_else(|| KunaError::parse("Must specify off or a decimal op count"))?;
        Ok((Some(n), format!("A return block is split into at most {n} cloned ops")))
    }
}

/// Does splitting a block of `ops` ops along `edges` edges clone more than
/// `limit` (the resolved option) allows?
pub fn split_is_declined(limit: Option<u64>, ops: usize, edges: usize) -> bool {
    limit.is_some_and(|limit| (ops as u64).saturating_mul(edges as u64) > limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limit_counts_cloned_ops_and_off_never_declines() {
        assert!(!split_is_declined(None, 1 << 20, 1 << 20));
        assert!(!split_is_declined(Some(DEFAULT_LIMIT), 3000, 349));
        assert!(split_is_declined(Some(DEFAULT_LIMIT), 3000, 350));
        assert!(!split_is_declined(Some(0), 0, 10));
        assert_eq!(OptionRetSplitLimit.apply("off").unwrap().0, None);
        assert_eq!(OptionRetSplitLimit.apply("4096").unwrap().0, Some(4096));
        assert!(OptionRetSplitLimit.apply("on").is_err());
        assert!(OptionRetSplitLimit.apply("-1").is_err());
    }
}
