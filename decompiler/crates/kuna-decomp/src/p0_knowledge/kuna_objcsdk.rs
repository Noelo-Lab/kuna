//! (kuna `objcsdk`) The load-time bridge for the Objective-C SDK header path.
//!
//! The `objc` pass runs while the loader snapshots the image, upstream of every
//! console `option`, so the directory it reads method signatures from travels
//! as a process environment variable ([`OBJCSDK_ENV`]) the way `sourcelang`
//! and `dynrelocs` do. Apple's headers are never vendored: the user points kuna
//! at an SDK on disk (`--option objcsdk /path/to/iPhoneOS.sdk`) or at any
//! directory of `.h` files, and nothing derived from them is written anywhere.

use std::path::PathBuf;

/// Environment variable carrying the SDK or header directory into the load path.
/// Unset, empty or `off` means no headers are read.
pub const OBJCSDK_ENV: &str = "KUNA_OBJCSDK";

/// The configured header root, if any.
pub fn sdk_path_from_env() -> Option<PathBuf> {
    let v = std::env::var(OBJCSDK_ENV).ok()?;
    parse_option_value(&v).ok().flatten()
}

/// Bridge a `set_kuna_option("objcsdk", val)` choice to [`OBJCSDK_ENV`] so a
/// later `load file` in the same process sees it (`off` clears it).
pub fn set_objcsdk_env(path: Option<&str>) {
    match path {
        Some(p) => std::env::set_var(OBJCSDK_ENV, p),
        None => std::env::remove_var(OBJCSDK_ENV),
    }
}

/// Parse an `objcsdk` option value: `off` (and empty) clear the path, anything
/// else is the path itself. A path that does not exist is refused.
pub fn parse_option_value(value: &str) -> Result<Option<PathBuf>, String> {
    let v = value.trim();
    if v.is_empty() || v.eq_ignore_ascii_case("off") {
        return Ok(None);
    }
    let path = PathBuf::from(v);
    if !path.is_dir() {
        return Err(format!("objcsdk: {v:?} is not a directory"));
    }
    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_and_empty_clear_a_missing_path_is_refused() {
        assert_eq!(parse_option_value("off").unwrap(), None);
        assert_eq!(parse_option_value("  ").unwrap(), None);
        assert!(parse_option_value("/definitely/not/here").is_err());
        let here = std::env::temp_dir();
        assert_eq!(parse_option_value(here.to_str().unwrap()).unwrap(), Some(here));
    }
}
