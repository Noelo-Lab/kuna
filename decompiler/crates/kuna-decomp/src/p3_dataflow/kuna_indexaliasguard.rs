//! P3 index-alias heritage guards — the `indexaliasguard` decision point.
//!
//! # The gap
//!
//! [`Heritage::guard`](crate::p3_dataflow::heritage::Heritage) ends with an arm
//! upstream Ghidra runs whenever a pointer can reach the range being heritaged
//! (`heritage.cc:1194`, gated on `Architecture::highPtrPossible`):
//! `Heritage::guardStores` (`heritage.cc:1538`) prepopulates data-flow across
//! every STORE that could alias the range, and `Heritage::guardLoads`
//! (`heritage.cc:1570`) puts an `addrforce` `CPUI_COPY` read of the range in
//! front of each indexed-stack LOAD whose discovered guard range covers it.
//! kuna shipped that arm behind a hard-coded `highPtrPossible == false`, so
//! neither ran.
//!
//! Without the LOAD guard a frame slot written by a direct `MOV [ESP+k],REG` and
//! read only through a `LEA`-derived pointer has no reader at all, so
//! `ActionDeadCode` deletes the store: the emitted C declares a stack array,
//! walks it with a pointer loop, and never initializes it.
//!
//! # The levels
//!
//! The two halves have very different footprints, so the option is a tri-state
//! rather than an on/off:
//!
//! * [`LEVEL_OFF`] — neither guard; byte-identical to what kuna shipped before
//!   this option.
//! * [`LEVEL_LOAD`] — the LOAD guard only.  This is the shipped default: it is
//!   what recovers the dropped initializing stores, and it moved 0/675 datatest
//!   and 0/714 stage assertions.
//! * [`LEVEL_FULL`] — LOAD plus STORE guards, i.e. upstream Ghidra's behavior.
//!   The STORE guard's `INDIRECT` chain survives into the output as write-backs
//!   of values a slot already held (`ttyperm._8_4_ = (unsigned int)v6;` beside
//!   `v6 = ttyperm._8_8_;`) and as globals hoisted into temporaries: measured
//!   over 60 decbench binaries it accounts for the whole +49-line growth of
//!   `libedit rl_init` and recovers nothing the LOAD guard does not.
//!
//! The guards themselves are upstream `Heritage` methods and live with their
//! siblings in [`crate::p3_dataflow::heritage`]; this module owns the option's
//! parse surface and the level vocabulary the gate reads.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;

/// `option indexaliasguard off` — neither index-alias guard runs.
pub const LEVEL_OFF: int4 = 0;

/// `option indexaliasguard load` — `Heritage::guardLoads` only.
pub const LEVEL_LOAD: int4 = 1;

/// `option indexaliasguard full` — `guardLoads` + `guardStores`, i.e. upstream
/// Ghidra's behavior.
pub const LEVEL_FULL: int4 = 2;

/// `option indexaliasguard off|load|full` (C++ has no equivalent — upstream is
/// unconditionally `full`).
pub struct OptionIndexAliasGuard;

impl OptionIndexAliasGuard {
    /// The option name.
    pub const NAME: &'static str = "indexaliasguard";

    /// Parse `off`/`load`/`full` into the guard level + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((LEVEL_OFF, "Index-alias heritage guards turned off".to_string())),
            // Empty parameter reads as the shipped default, matching `options::on_or_off`.
            "load" | "on" | "" => Ok((
                LEVEL_LOAD,
                "Index-alias heritage guards turned on, LOAD side only (guardLoads)".to_string(),
            )),
            "full" => Ok((
                LEVEL_FULL,
                "Index-alias heritage guards turned on, LOAD + STORE \
                 (guardLoads + guardStores)"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of off, load, full",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_is_registered() {
        assert!(crate::options::KUNA_OPTION_NAMES.contains(&OptionIndexAliasGuard::NAME));
    }

    #[test]
    fn levels_parse_and_reject() {
        let o = OptionIndexAliasGuard;
        assert_eq!(o.apply("off").unwrap().0, LEVEL_OFF);
        assert_eq!(o.apply("load").unwrap().0, LEVEL_LOAD);
        assert_eq!(o.apply("on").unwrap().0, LEVEL_LOAD);
        assert_eq!(o.apply("").unwrap().0, LEVEL_LOAD);
        assert_eq!(o.apply("full").unwrap().0, LEVEL_FULL);
        assert!(o.apply("sometimes").is_err());
    }
}
