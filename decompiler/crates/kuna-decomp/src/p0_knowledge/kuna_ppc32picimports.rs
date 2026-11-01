//! Recovery policy for PPC32 secure-PLT imports. Only `abi` permits global
//! loader identities; `on` uses the caller-local flow proof.
pub const ENV: &str = "KUNA_PPC32PICIMPORTS";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Off,
    On,
    Abi,
}

impl Mode {
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
}

pub fn abi_enabled() -> bool {
    std::env::var(ENV).is_ok_and(|s| s == "abi")
}
