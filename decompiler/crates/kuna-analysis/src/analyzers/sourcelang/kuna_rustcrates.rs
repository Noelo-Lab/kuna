//! Registry crates named by the source paths a Rust image carries.
//!
//! Panic locations and debug paths of a dependency spell its unpacked registry
//! directory, `<cargo home>/registry/src/<index>/<crate>-<version>/...`, where
//! `<index>` is the registry host and a 16-digit hash. The crate directory is
//! split at the first `-` that starts a `major.minor.patch` version.

use memchr::memmem;
use std::collections::BTreeSet;

const MAX_CRATES: usize = 4096;

fn is_sep(b: u8) -> bool {
    matches!(b, b'/' | b'\\')
}

/// The leading path component of `bytes` and the rest after its separator.
fn component(bytes: &[u8], max: usize) -> Option<(&[u8], &[u8])> {
    let end = bytes.iter().take(max + 1).position(|&b| is_sep(b))?;
    Some((&bytes[..end], &bytes[end + 1..]))
}

fn is_index(name: &[u8]) -> bool {
    let Some(dash) = name.iter().rposition(|&b| b == b'-') else {
        return false;
    };
    let (host, hash) = (&name[..dash], &name[dash + 1..]);
    !host.is_empty()
        && host.iter().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
        && hash.len() == 16
        && hash.iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_version(text: &[u8]) -> bool {
    let core = text.split(|b| matches!(b, b'-' | b'+')).next().unwrap_or(text);
    let mut fields = core.split(|&b| b == b'.');
    let numeric = |field: Option<&[u8]>| {
        field.is_some_and(|f| !f.is_empty() && f.len() <= 9 && f.iter().all(u8::is_ascii_digit))
    };
    numeric(fields.next())
        && numeric(fields.next())
        && numeric(fields.next())
        && fields.next().is_none()
        && text.iter().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'))
}

/// Split a registry directory name such as `sha-1-0.10.1` into name and version.
fn split_crate(dir: &[u8]) -> Option<(&str, &str)> {
    let at = (1..dir.len()).find(|&i| dir[i] == b'-' && is_version(&dir[i + 1..]))?;
    let name = &dir[..at];
    if !name.iter().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_')) {
        return None;
    }
    Some((std::str::from_utf8(name).ok()?, std::str::from_utf8(&dir[at + 1..]).ok()?))
}

/// The `(name, version)` pairs of the registry crates `bytes` names, sorted.
pub fn registry_crates(bytes: &[u8]) -> Vec<(String, String)> {
    let mut found = BTreeSet::new();
    for at in memmem::find_iter(bytes, b"registry") {
        let rest = &bytes[at + 8..];
        let [open, b's', b'r', b'c', close, rest @ ..] = rest else {
            continue;
        };
        if !is_sep(*open) || !is_sep(*close) {
            continue;
        }
        let Some((index, rest)) = component(rest, 96) else {
            continue;
        };
        let Some((dir, _)) = component(rest, 128) else {
            continue;
        };
        if !is_index(index) {
            continue;
        }
        if let Some((name, version)) = split_crate(dir) {
            found.insert((name.to_string(), version.to_string()));
            if found.len() == MAX_CRATES {
                break;
            }
        }
    }
    found.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{registry_crates, split_crate};

    #[test]
    fn crate_directories_split_at_the_version() {
        assert_eq!(split_crate(b"memchr-2.7.4"), Some(("memchr", "2.7.4")));
        assert_eq!(split_crate(b"sha-1-0.10.1"), Some(("sha-1", "0.10.1")));
        assert_eq!(split_crate(b"clap-4.0.0-rc.1"), Some(("clap", "4.0.0-rc.1")));
        assert_eq!(split_crate(b"libz-sys-1.1.8+zlib.1.2.12"), Some(("libz-sys", "1.1.8+zlib.1.2.12")));
        assert_eq!(split_crate(b"wasi-0.11.0+wasi-snapshot-preview1"), Some(("wasi", "0.11.0+wasi-snapshot-preview1")));
        for dir in [&b"memchr"[..], b"memchr-2.7", b"-1.0.0", b"mem chr-1.0.0", b"memchr-1.0.0 x", b""] {
            assert_eq!(split_crate(dir), None, "{dir:?}");
        }
    }

    #[test]
    fn registry_paths_list_each_crate_once() {
        let image = [
            &b"\0/home/u/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.7.4/src/memmem/mod.rs\0"[..],
            b"/usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/itoa-1.0.11/src/lib.rs\0",
            b"C:\\Users\\u\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\cfg-if-1.0.0\\src\\lib.rs\0",
            b"/home/u/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.7.4/src/lib.rs\0",
            b"/home/u/.cargo/registry/src/not-an-index/serde-1.0.0/src/lib.rs\0",
            b"/home/u/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/serde-1.0.0.crate\0",
            b"/home/u/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/truncated-1.0.0",
        ]
        .concat();
        let pair = |name: &str, version: &str| (name.to_string(), version.to_string());
        assert_eq!(
            registry_crates(&image),
            [pair("cfg-if", "1.0.0"), pair("itoa", "1.0.11"), pair("memchr", "2.7.4")]
        );
        assert!(registry_crates(b"registry/src/").is_empty());
        assert!(registry_crates(b"registry").is_empty());
    }
}
