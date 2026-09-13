//! (kuna) The `rexthunk` option gate: on an x86-64 PE, the `FF 25 <disp32>` one
//! byte into a `REX FF 25` tail jump is not an import thunk.
//!
//! ## The gap it closes
//!
//! The PE import resolver names every `FF 25 <disp32>` (`jmp [rip+disp]`) whose
//! target is an Import Address Table slot as a thunk entry. Every linker
//! (link.exe, lld-link, GNU ld) emits that thunk bare. A compiler-emitted tail
//! jump through `__imp_X` carries a REX.W prefix instead (`48 FF 25`), the epilogue
//! form the Windows x64 unwinder recognises. The byte scan matched the `FF 25` one
//! byte into that instruction, and because a RIP-relative displacement is measured
//! from the end of the instruction the slot still resolved, so each such tail jump
//! became an import-named function entry in the middle of an instruction. A match
//! after a REX byte is now dropped unless the image references its `FF` byte (a
//! branch, a RIP-relative `lea`, an eight-byte pointer, `.pdata` or an export) or a
//! thunk of the same table ends there (`kuna-analysis::loader::kuna_rexthunk`).
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The import names are resolved inside `load file`, upstream of every
//! per-function `option` command. The gate is therefore a process
//! **environment variable** ([`REXTHUNK_ENV`]) consulted by
//! `kuna-analysis::loader::kuna_rexthunk` and set by the `kuna` CLI before the
//! file is loaded, the same pattern as `pdatachained`. The companion
//! `Architecture::analysis_rexthunk` bool exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **on**; `--option rexthunk off` restores the previous inventory.

/// Environment variable that gates the REX-prefixed thunk rejection. Absent or
/// any value other than the off-tokens means **enabled**; `off`/`0`/`false`
/// means disabled.
pub const REXTHUNK_ENV: &str = "KUNA_REXTHUNK";

/// Whether the REX-prefixed thunk rejection is enabled for this process.
pub fn rexthunk_enabled() -> bool {
    match std::env::var(REXTHUNK_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true,
    }
}

/// Bridge a `set_kuna_option("rexthunk", val)` toggle to [`REXTHUNK_ENV`] so a
/// later `load file` in the same process sees the choice.
pub fn set_rexthunk_env(on: bool) {
    std::env::set_var(REXTHUNK_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(REXTHUNK_ENV);
        assert!(rexthunk_enabled(), "unset env => default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(REXTHUNK_ENV, off);
            assert!(!rexthunk_enabled(), "`{off}` => disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(REXTHUNK_ENV, on);
            assert!(rexthunk_enabled(), "`{on}` => enabled");
        }

        set_rexthunk_env(false);
        assert!(!rexthunk_enabled());
        set_rexthunk_env(true);
        assert!(rexthunk_enabled());

        std::env::remove_var(REXTHUNK_ENV);
    }
}
