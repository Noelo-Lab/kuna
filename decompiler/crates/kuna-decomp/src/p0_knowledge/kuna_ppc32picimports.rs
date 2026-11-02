//! Recovery policy for PPC32 secure-PLT imports. Only `abi` permits global
//! loader identities; `on` uses the caller-local flow proof.
//!
//! `abi` is a load-time property: the loader reads [`ENV`] before an
//! `Architecture` exists, so a new `Architecture` starts in `abi` exactly when
//! its image was loaded with it ([`Mode::loaded`]). `on` and `off` are live and
//! set per `Architecture`.
pub const ENV: &str = "KUNA_PPC32PICIMPORTS";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Off,
    On,
    Abi,
}

impl Mode {
    /// The console vocabulary: `off`, `on` (or empty) and `abi`.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "off" => Some(Self::Off),
            "on" | "" => Some(Self::On),
            "abi" => Some(Self::Abi),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::Abi => "abi",
        }
    }

    pub fn scoped(self) -> bool {
        self == Self::On
    }

    pub fn set_env(self) {
        std::env::set_var(ENV, self.as_str());
    }

    /// The environment vocabulary of the other default-off load-time gates,
    /// plus `abi`; an unrecognized value is `off`.
    pub fn from_env_value(value: &str) -> Self {
        if value.trim().eq_ignore_ascii_case("abi") {
            Self::Abi
        } else if crate::options::parse_env_toggle(value, false) {
            Self::On
        } else {
            Self::Off
        }
    }

    pub fn from_env() -> Self {
        std::env::var(ENV).map_or(Self::Off, |value| Self::from_env_value(&value))
    }

    /// The mode a new `Architecture` starts in.
    pub fn loaded() -> Self {
        if abi_enabled() { Self::Abi } else { Self::Off }
    }
}

pub fn abi_enabled() -> bool {
    Mode::from_env() == Mode::Abi
}

#[cfg(test)]
mod tests {
    use super::Mode;

    #[test]
    fn environment_values_share_the_load_time_gate_vocabulary() {
        for (value, mode) in [
            ("abi", Mode::Abi), (" ABI ", Mode::Abi), ("on", Mode::On), ("1", Mode::On),
            ("true", Mode::On), (" TRUE ", Mode::On), ("", Mode::On), ("off", Mode::Off),
            ("0", Mode::Off), ("false", Mode::Off), ("yes", Mode::Off), ("bogus", Mode::Off),
        ] {
            assert_eq!(Mode::from_env_value(value), mode, "{value:?}");
        }
        for mode in [Mode::Off, Mode::On, Mode::Abi] {
            assert_eq!(Mode::from_env_value(mode.as_str()), mode);
            assert_eq!(Mode::parse(mode.as_str()), Some(mode));
        }
        assert_eq!(Mode::parse("true"), None, "the console keeps its on/off vocabulary");
    }
}
