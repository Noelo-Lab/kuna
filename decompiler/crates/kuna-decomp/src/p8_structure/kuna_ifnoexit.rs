//! (kuna) The `rule_block_if_no_exit` decision surface: the shared per-block
//! facts the rule's arm tie-break and the deferred scan need, plus the
//! `KUNA_RS_DEBUG` decision trace that measures them.
//!
//! `CollapseStructure` runs against the `sblocks` `BlockCopy` mirror and has no
//! `Funcdata` back-pointer, so anything that must read the live op list (a
//! block's start address, and whether its trailing `RETURN` is an *artificial*
//! no-return halt rather than a genuine source `return`) is precomputed here by
//! `ActionBlockStructure::apply` and handed over as a side table — the same
//! shape as the existing `complex_blocks` / `switch_blocks` precomputes.

use std::collections::BTreeMap;

use kuna_base::types::uint4;

use crate::context::BlockId;

/// Per-`bblocks`-block facts the structuring graph cannot read for itself.
#[derive(Debug, Clone, Default)]
pub struct ExitLeafFacts {
    /// `PcodeOp::getHaltType()` of the block's trailing op, when that op is an
    /// artificial halt `RETURN` (`op_mark_halt`).  Absent for a block whose
    /// tail is a genuine `RETURN` or is not a `RETURN` at all.
    pub halt: BTreeMap<BlockId, uint4>,
    /// Start address offset of each block, for the trace only.
    pub addr: BTreeMap<BlockId, u64>,
}

impl ExitLeafFacts {
    /// The halt type of `bb`, 0 when it carries none.
    pub fn halt_type(&self, bb: BlockId) -> uint4 {
        self.halt.get(&bb).copied().unwrap_or(0)
    }

    /// Whether `bb` ends in an artificial halt marked `noreturn` — the
    /// `op_mark_halt` bit a no-return *callee* sets, as distinct from the
    /// `badinstruction` / `unimplemented` / `missing` / `halt` bits, which
    /// produce the identical `return(#0x1:4)` shape from a decode failure.
    pub fn is_noreturn_halt(&self, bb: BlockId) -> bool {
        (self.halt_type(bb) & crate::op::pcodeop_flags::noreturn) != 0
    }
}

/// Render a halt-type bitmask as the flag names, for the trace.
pub fn halt_str(flags: uint4) -> String {
    use crate::op::pcodeop_flags as f;
    if flags == 0 {
        return "-".to_string();
    }
    let mut v: Vec<&str> = Vec::new();
    if (flags & f::noreturn) != 0 {
        v.push("noreturn");
    }
    if (flags & f::halt) != 0 {
        v.push("halt");
    }
    if (flags & f::badinstruction) != 0 {
        v.push("badinstr");
    }
    if (flags & f::unimplemented) != 0 {
        v.push("unimpl");
    }
    if (flags & f::missing) != 0 {
        v.push("missing");
    }
    v.join("|")
}

/// Whether the `KUNA_RS_DEBUG` decision trace is enabled.
pub fn trace_enabled() -> bool {
    std::env::var_os("KUNA_RS_DEBUG").is_some()
}

//===========================================================================
// Option parse (`guardarm on|off`, `loopcondhoist on|off`).
//===========================================================================

/// The `guardarm on|off` ArchOption — source-layout tie-break for the
/// `ruleBlockIfNoExit` clause arm.
pub struct OptionGuardArm;

impl OptionGuardArm {
    /// The option name.
    pub const NAME: &'static str = "guardarm";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(bool, String)> {
        let val = crate::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("If-no-exit guard-arm layout tie-break turned {prop}")))
    }
}

/// The `loopcondhoist on|off` ArchOption - defer a loop head in the
/// `ruleBlockIfNoExit` scan so `ruleBlockWhileDo` keeps its head test.
pub struct OptionLoopCondHoist;

impl OptionLoopCondHoist {
    /// The option name.
    pub const NAME: &'static str = "loopcondhoist";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(bool, String)> {
        let val = crate::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("If-no-exit loop-head deferral turned {prop}")))
    }
}

/// Monotone counter of `ActionBlockStructure::apply` invocations, so a trace
/// line can be attributed to a particular `collapse_all` run (the action pool
/// re-runs the structuring action, and `take_pending_flips` re-orients the CFG
/// between runs, so the run index is load-bearing).
pub fn next_apply_index() -> u32 {
    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
