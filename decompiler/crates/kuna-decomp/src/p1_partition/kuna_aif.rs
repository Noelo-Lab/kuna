//! (kuna `aif`) The four-valued gate on the Aggressive Instruction Finder gap walk.
//!
//! The walk itself lives in the analyzer tier (`kuna_analysis::aif`); what lives
//! here is the *decision* the option names, because the option is not a plain
//! on/off. Each value is a cumulative setting of the walk:
//!
//! | value | gap cursor | accept |
//! |---|---|---|
//! | `off` | no walk | — |
//! | `on` | next byte | prologue shared by 4 discovered functions + a valid subroutine |
//! | `strict` | next 4-byte boundary, or the first byte of a hole | as `on` |
//! | `precise` | as `strict` | as `on`, and the routine adds information or its prologue is shared by 50 |

use kuna_base::error::{KunaError, KunaResult};

/// The resolved `option aif off|on|strict|precise` mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AifMode {
    #[default]
    Off,
    On,
    Strict,
    Precise,
}

impl AifMode {
    /// Does the gap walk run at all?
    pub fn enabled(self) -> bool {
        !matches!(self, AifMode::Off)
    }

    /// Does the gap cursor take the aligned slide (`kuna_aifstrict`)?
    pub fn strict(self) -> bool {
        matches!(self, AifMode::Strict | AifMode::Precise)
    }

    /// Must an accept be corroborated (`kuna_aifcorroborate`)?
    pub fn corroborate(self) -> bool {
        matches!(self, AifMode::Precise)
    }

    /// The `option aif <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            AifMode::Off => "off",
            AifMode::On => "on",
            AifMode::Strict => "strict",
            AifMode::Precise => "precise",
        }
    }
}

/// Parse the `option aif off|on|strict|precise` argument, producing the resolved
/// mode plus the confirmation message.
pub fn parse_aif_mode(p1: &str) -> KunaResult<(AifMode, String)> {
    let mode = match p1 {
        "off" => AifMode::Off,
        "on" => AifMode::On,
        "strict" => AifMode::Strict,
        "precise" => AifMode::Precise,
        _ => return Err(KunaError::parse("Must specify off, on, strict or precise")),
    };
    Ok((mode, format!("Aggressive Instruction Finder gap walk set to {p1}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_value_is_a_cumulative_setting_of_the_walk() {
        let table = [
            ("off", false, false, false),
            ("on", true, false, false),
            ("strict", true, true, false),
            ("precise", true, true, true),
        ];
        for (token, enabled, strict, corroborate) in table {
            let (mode, _) = parse_aif_mode(token).unwrap();
            assert_eq!(mode.as_str(), token);
            assert_eq!((mode.enabled(), mode.strict(), mode.corroborate()), (enabled, strict, corroborate));
        }
        assert!(parse_aif_mode("corroborate").is_err());
        assert_eq!(AifMode::default(), AifMode::Off);
    }
}
