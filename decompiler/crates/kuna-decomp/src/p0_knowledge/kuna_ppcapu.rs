//! (kuna) The `ppcapu` option gate: select a compatible embedded language for a 32-bit
//! PowerPC ELF image from its APU requirements and embedded ABI flag.
//!
//! The language is chosen while the image loads, before any `option` command
//! runs, so the gate is the process environment variable [`PPCAPU_ENV`], set by
//! the CLI before loading. `Architecture::analysis_ppcapu` exists only for the
//! catalog's live `current` field. The default `isel` extends the generic model
//! with ISEL; `on` also selects e500 for an SPE requirement, whose model does not
//! yet recover every scalar call argument held in its 64-bit registers.

/// Environment variable read by `kuna-analysis::loader::kuna_ppcaputarget`.
pub const PPCAPU_ENV: &str = "KUNA_PPCAPU";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    Off,
    #[default]
    Isel,
    On,
}

impl Mode {
    /// The console vocabulary: `isel`, `on` (or empty) and `off`.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "off" => Some(Self::Off),
            "isel" => Some(Self::Isel),
            "on" | "" => Some(Self::On),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Isel => "isel",
            Self::On => "on",
        }
    }

    pub fn set_env(self) {
        std::env::set_var(PPCAPU_ENV, self.as_str());
    }

    /// `isel`, the on-tokens of the other load-time gates, and `off` for
    /// anything else.
    pub fn from_env_value(value: &str) -> Self {
        if value.trim().eq_ignore_ascii_case("isel") {
            Self::Isel
        } else if crate::options::parse_env_toggle(value, false) {
            Self::On
        } else {
            Self::Off
        }
    }
}

/// The APU-based PowerPC language selection in force for this process.
pub fn ppcapu_mode() -> Mode {
    crate::kuna_loadoptions::scoped(|o| o.ppcapu).unwrap_or_else(crate::kuna_loadoptions::env::ppcapu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_isel_and_tokens() {
        std::env::remove_var(PPCAPU_ENV);
        assert_eq!(ppcapu_mode(), Mode::Isel, "unset env ⇒ default isel");
        for (token, mode) in [
            ("isel", Mode::Isel),
            (" ISEL ", Mode::Isel),
            ("on", Mode::On),
            ("1", Mode::On),
            ("true", Mode::On),
            ("ON", Mode::On),
            ("", Mode::On),
            ("off", Mode::Off),
            ("0", Mode::Off),
            ("false", Mode::Off),
            ("invalid", Mode::Off),
        ] {
            std::env::set_var(PPCAPU_ENV, token);
            assert_eq!(ppcapu_mode(), mode, "`{token}`");
        }
        for mode in [Mode::On, Mode::Off, Mode::Isel] {
            mode.set_env();
            assert_eq!(ppcapu_mode(), mode);
            assert_eq!(Mode::parse(mode.as_str()), Some(mode));
        }
        assert_eq!(Mode::parse("yes"), None);
        std::env::remove_var(PPCAPU_ENV);
    }
}
