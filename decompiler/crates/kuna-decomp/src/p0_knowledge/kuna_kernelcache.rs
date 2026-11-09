//! (kuna) The `kernelcache` option: whether `load file` unwraps Apple firmware
//! containers (IMG4 / IM4P, LZFSE, `complzss`) and which `LC_FILESET_ENTRY` of an
//! `MH_FILESET` kernel collection it loads.
//!
//! Loading happens before any per-function `option` command, so the live gate is
//! the [`KERNELCACHE_ENV`] environment variable, exported by the `kuna` CLI and
//! bridged from the console `option` command, the same shape as `relocrebase`.

use kuna_base::error::{KunaError, KunaResult};

pub const KERNELCACHE_ENV: &str = "KUNA_KERNELCACHE";

/// `auto` unwraps containers and loads the kernel (`MH_EXECUTE`) entry of a
/// fileset; a bundle id loads that entry instead; `off` passes bytes through.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum KernelcacheChoice {
    Off,
    #[default]
    Auto,
    Entry(String),
}

impl KernelcacheChoice {
    pub fn parse(value: &str) -> Option<Self> {
        let token = value.trim();
        match token.to_ascii_lowercase().as_str() {
            "" | "auto" | "on" => Some(Self::Auto),
            "off" | "0" | "false" => Some(Self::Off),
            _ if token.chars().any(|c| c.is_whitespace() || c.is_control()) => None,
            _ => Some(Self::Entry(token.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::Auto => "auto",
            Self::Entry(id) => id,
        }
    }

    /// The choice in force for this process; an unset or unparsable value is `auto`.
    pub fn from_env() -> Self {
        std::env::var(KERNELCACHE_ENV).ok().and_then(|v| Self::parse(&v)).unwrap_or_default()
    }
}

pub fn set_kernelcache_env(choice: &KernelcacheChoice) {
    std::env::set_var(KERNELCACHE_ENV, choice.as_str());
}

pub fn parse_kernelcache(value: &str) -> KunaResult<(KernelcacheChoice, String)> {
    let choice = KernelcacheChoice::parse(value).ok_or_else(|| {
        KunaError::parse(format!(
            "kernelcache expects auto, off, or a fileset entry id, got {value:?}"
        ))
    })?;
    let msg = format!("Apple kernelcache loading set to {}", choice.as_str());
    Ok((choice, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_auto_off_and_entry_ids() {
        assert_eq!(KernelcacheChoice::parse(""), Some(KernelcacheChoice::Auto));
        assert_eq!(KernelcacheChoice::parse(" ON "), Some(KernelcacheChoice::Auto));
        assert_eq!(KernelcacheChoice::parse("Off"), Some(KernelcacheChoice::Off));
        assert_eq!(
            KernelcacheChoice::parse("com.apple.iokit.IOSurface"),
            Some(KernelcacheChoice::Entry("com.apple.iokit.IOSurface".into()))
        );
        assert_eq!(KernelcacheChoice::parse("two words"), None);
        assert!(parse_kernelcache("a\tb").is_err());
    }
}
