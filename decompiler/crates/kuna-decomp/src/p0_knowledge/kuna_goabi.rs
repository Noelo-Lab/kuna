//! (kuna) The `goabi` option: select the vendored golang compiler spec for a
//! Go image, and bind the `abi0` prototype model to `.abi0`-named functions
//! (`docs/multilang.md` W4, the Go ABI plane).
//!
//! ## What the option gates
//!
//! Go does not use the platform C ABI. Since 1.17 its register ABI
//! (`abi-internal`) passes arguments in RAX,RBX,RCX,RDI,RSI,R8,R9,R10,R11 on
//! x86-64 and returns multiple values in those same registers — so under the
//! default `gcc` cspec a Go function's parameters are invisible (RAX/RBX are
//! scratch to SysV), a second return value is dead-coded away, and the body
//! statements feeding it are DELETED. Ghidra vendors golang cspecs for
//! exactly this (`x86-64-golang.cspec`, `x86-32-golang.cspec`,
//! `AARCH64_golang.cspec` — `abi-internal` default, `abi0` the stack ABI,
//! plus the write-barrier/duff helper models), and kuna ships them compiled;
//! nothing selected them. With `goabi auto` (the default) an image whose
//! effective source language is Go composes the `:golang` compiler field in
//! its SLEIGH language id on the three architectures that have one; every
//! other architecture and every non-Go image is untouched, and the engine's
//! existing default-model retry covers a missing spec.
//!
//! ## Why the override is an env-var bridge
//!
//! The language id is composed inside `load file`, upstream of every
//! per-function `option` command — the same seam as `sourcelang`, and the
//! same [`GOABI_ENV`] bridge pattern (`rexthunk`/`relocobjects`).
//!
//! ## The `.abi0` binding that is deliberately NOT here
//!
//! Go's dual-ABI symbols spell a stack-ABI alias `name.abi0` in an unstripped
//! `.symtab` (115 in a go1.22 hello-world), and the golang cspec registers the
//! matching `abi0` model — but measured end to end, neither name source
//! reaches the engine: the go1.22 **pclntab carries no `.abi0` names at all**,
//! and the console's canonical entry inventory collapses the symtab alias so
//! zero `.abi0` spellings survive into funcdata or call-spec names. A
//! name-keyed model binding is therefore dead code on real inputs. Per-symbol
//! ABI0 needs alias-aware symbol plumbing first; until then every function of
//! a Go image takes the cspec default (`abi-internal`), which is also what
//! Ghidra does.

use crate::options::env_toggle;

/// Environment variable carrying the `goabi` choice into the load path.
/// Absent or any value other than the off-tokens means **enabled** (`auto`).
pub const GOABI_ENV: &str = "KUNA_GOABI";

/// Whether golang-cspec auto-selection is enabled for this process.
pub fn goabi_env_enabled() -> bool {
    env_toggle(GOABI_ENV, true)
}

/// Bridge a `set_kuna_option("goabi", val)` toggle to [`GOABI_ENV`] so a later
/// `load file` in the same process sees the choice.
pub fn set_goabi_env(on: bool) {
    std::env::set_var(GOABI_ENV, if on { "auto" } else { "off" });
}

/// Parse an `option goabi` value (`auto`/on-tokens enable, off-tokens disable).
pub fn parse_goabi(value: &str) -> Result<bool, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "auto" | "on" | "1" | "true" | "" => Ok(true),
        "off" | "0" | "false" => Ok(false),
        other => Err(format!("unknown goabi value {other:?} (expected auto or off)")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goabi_values_parse_and_refuse() {
        for on in ["auto", "on", "1", "true", "", " AUTO "] {
            assert_eq!(parse_goabi(on), Ok(true), "{on}");
        }
        for off in ["off", "0", "false", " OFF "] {
            assert_eq!(parse_goabi(off), Ok(false), "{off}");
        }
        assert!(parse_goabi("always").is_err());
    }

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(GOABI_ENV);
        assert!(goabi_env_enabled(), "unset env => default auto");
        std::env::set_var(GOABI_ENV, "off");
        assert!(!goabi_env_enabled());
        std::env::set_var(GOABI_ENV, "auto");
        assert!(goabi_env_enabled());
        std::env::remove_var(GOABI_ENV);
    }
}
