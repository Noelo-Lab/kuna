//! (kuna) The `kernelcache-boot` option: where `load file` finds the kernel
//! collections an Auxiliary or System collection was linked against.
//!
//! Such a collection points into the boot collection (and an Auxiliary one into
//! the System collection) with chained fixups that are offsets from the other
//! file's base, so the loader has to read that file. Loading happens before any
//! per-function `option` command, so the live gate is the
//! [`KERNELCACHE_BOOT_ENV`] environment variable, exported by the `kuna` CLI
//! and bridged from the console `option` command.

use std::path::PathBuf;

use kuna_base::error::{KunaError, KunaResult};

pub const KERNELCACHE_BOOT_ENV: &str = "KUNA_KERNELCACHE_BOOT";

/// `auto` searches the directory of the input; a path names one collection file
/// or a directory to search instead; `off` reads no other file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum KernelcacheBoot {
    Off,
    #[default]
    Auto,
    Path(PathBuf),
}

impl KernelcacheBoot {
    pub fn parse(value: &str) -> Self {
        let token = value.trim();
        match token.to_ascii_lowercase().as_str() {
            "" | "auto" | "on" => Self::Auto,
            "off" | "0" | "false" => Self::Off,
            _ => Self::Path(PathBuf::from(token)),
        }
    }

    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::Off => "off".into(),
            Self::Auto => "auto".into(),
            Self::Path(path) => path.to_string_lossy(),
        }
    }

    /// The choice of the load in progress (`LoadOptions::kernelcache_boot`,
    /// else [`KERNELCACHE_BOOT_ENV`]).
    pub fn in_force() -> Self {
        crate::kuna_loadoptions::scoped(|o| o.kernelcache_boot.clone())
            .unwrap_or_else(crate::kuna_loadoptions::env::kernelcache_boot)
    }

    /// The choice [`KERNELCACHE_BOOT_ENV`] states; unset is `auto`. Reads only
    /// the environment; a load in progress reads [`Self::in_force`].
    pub fn from_env() -> Self {
        std::env::var(KERNELCACHE_BOOT_ENV).map(|v| Self::parse(&v)).unwrap_or_default()
    }
}

pub fn set_kernelcache_boot_env(choice: &KernelcacheBoot) {
    std::env::set_var(KERNELCACHE_BOOT_ENV, &*choice.as_str());
}

/// Parse a `kernelcache-boot` option value. A path that does not exist is
/// refused.
pub fn parse_kernelcache_boot(value: &str) -> KunaResult<(KernelcacheBoot, String)> {
    let choice = KernelcacheBoot::parse(value);
    if let KernelcacheBoot::Path(path) = &choice {
        if !path.exists() {
            return Err(KunaError::parse(format!(
                "kernelcache-boot expects auto, off, or the path of a kernel collection or its directory; {value:?} does not exist"
            )));
        }
    }
    let msg = format!("Linked kernel collections set to {}", choice.as_str());
    Ok((choice, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_auto_off_and_paths() {
        assert_eq!(KernelcacheBoot::parse(""), KernelcacheBoot::Auto);
        assert_eq!(KernelcacheBoot::parse(" Auto "), KernelcacheBoot::Auto);
        assert_eq!(KernelcacheBoot::parse("OFF"), KernelcacheBoot::Off);
        let here = std::env::temp_dir();
        let (choice, _) = parse_kernelcache_boot(here.to_str().unwrap()).unwrap();
        assert_eq!(choice, KernelcacheBoot::Path(here));
        assert!(parse_kernelcache_boot("/definitely/not/here.kc").is_err());
    }
}
