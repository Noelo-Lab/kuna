//! (kuna `typelib`) Which prebuilt type library a load reads.
//!
//! A type library carries the declarations of a platform's C library: every
//! function with its prototype, and every record with the size, alignment and
//! member offsets the platform's compiler gives it. `auto` reads the library
//! built into kuna for the image's target (an x86-64 Linux ELF linked against
//! glibc or musl; no other target has one, so the option is inert there),
//! `off` reads none, and a path reads a library file instead of the built-in
//! one. The library is consulted when the analysis facts are committed, which
//! is after the `option` lines, so the choice is an ordinary field of the
//! [`Architecture`](crate::architecture::Architecture) and needs no load-time
//! bridge.

use std::io::Read;
use std::path::PathBuf;

/// The first line of every library file this build reads.
pub const MAGIC: &str = "kuna-typelib 1\n";

/// The value of the `typelib` option.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TypeLib {
    /// The library built in for the image's target, when there is one.
    #[default]
    Auto,
    /// No library.
    Off,
    /// The library in this file, for an image of the target it names.
    File(PathBuf),
}

impl TypeLib {
    /// Parse an option value. A path that is not a file is refused.
    pub fn parse(value: &str) -> Result<TypeLib, String> {
        let v = value.trim();
        if v.eq_ignore_ascii_case("auto") {
            return Ok(TypeLib::Auto);
        }
        if v.eq_ignore_ascii_case("off") {
            return Ok(TypeLib::Off);
        }
        let path = PathBuf::from(v);
        if !path.is_file() {
            return Err(format!("typelib: expected `auto`, `off` or a library file, got {v:?}"));
        }
        let mut magic = [0u8; MAGIC.len()];
        let read = std::fs::File::open(&path).and_then(|mut f| f.read_exact(&mut magic));
        if read.is_err() || magic != *MAGIC.as_bytes() {
            return Err(format!("typelib: {v} is not a type library (no `{}` first line)", MAGIC.trim_end()));
        }
        Ok(TypeLib::File(path))
    }

    /// The value as the option spells it.
    pub fn as_str(&self) -> String {
        match self {
            TypeLib::Auto => "auto".to_string(),
            TypeLib::Off => "off".to_string(),
            TypeLib::File(path) => path.display().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypeLib;

    #[test]
    fn a_value_is_auto_off_or_an_existing_file() {
        assert_eq!(TypeLib::parse("auto"), Ok(TypeLib::Auto));
        assert_eq!(TypeLib::parse(" OFF "), Ok(TypeLib::Off));
        assert!(TypeLib::parse("/nonexistent/library.ktl").is_err());
        let exe = std::env::current_exe().unwrap();
        assert!(TypeLib::parse(exe.to_str().unwrap()).unwrap_err().contains("not a type library"));
        let file = std::env::temp_dir().join(format!("kuna-typelib-{}.ktl", std::process::id()));
        std::fs::write(&file, "kuna-typelib 1\ntarget linux x86_64\nsource test 1\n").unwrap();
        let parsed = TypeLib::parse(file.to_str().unwrap());
        std::fs::remove_file(&file).unwrap();
        assert_eq!(parsed, Ok(TypeLib::File(file.clone())));
        assert_eq!(TypeLib::File(file.clone()).as_str(), file.display().to_string());
    }
}
