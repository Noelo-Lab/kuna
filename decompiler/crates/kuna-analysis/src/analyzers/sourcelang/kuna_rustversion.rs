//! Compiler-version evidence for versioned Rust runtime layouts.

//!
//! The primary record is the `rustc version` string in ELF `.comment`. An image
//! without one still names its compiler in every panic location path the
//! standard library carries (`/rustc/<commit>/library/...`); that commit is
//! mapped to a stable release through `kuna_rustreleases::RELEASES`.

use super::kuna_rustreleases::RELEASES;
use memchr::memmem;
use object::{BinaryFormat, Object, ObjectSection};

const PIECES_NIGHTLY: &[u8; 40] = b"bf64d66bd58719fac2585eae5e546e5e1d9947f5";

pub struct RustFmtPieces {
    pub relro: Option<(u64, u64)>,
}

/// `paths` allows the source-path fallback; callers pass whether the image
/// was detected as Rust, so other images are never scanned.
pub fn fmt_pieces_layout(bytes: &[u8], paths: bool) -> Option<RustFmtPieces> {
    let file = object::File::parse(bytes).ok()?;
    if file.format() != BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(
            file.kind(),
            object::ObjectKind::Executable | object::ObjectKind::Dynamic
        )
        || !match comment_records(&file) {
            Some(comment) => pieces_version(comment),
            None => paths && path_commit(bytes).is_some_and(|commit| {
                commit == PIECES_NIGHTLY || matches!(release(commit), Some((1, 89 | 90, 0)))
            }),
        }
    {
        return None;
    }
    let relro = crate::loader::kuna_dynrelocs::relro_range(bytes)
        .and_then(|(lo, hi)| (lo < (hi & !4095)).then_some((lo, hi & !4095)));
    Some(RustFmtPieces { relro })
}

fn pieces_version(bytes: &[u8]) -> bool {
    let mut found = None;
    for record in bytes.split(|&b| b == 0) {
        let Some(version) = record.strip_prefix(b"rustc version ") else {
            continue;
        };
        let identity = match version.split(|b| b.is_ascii_whitespace()).next() {
            Some(b"1.89.0") => 0,
            Some(b"1.90.0") => 1,
            Some(b"1.89.0-nightly") if version == b"1.89.0-nightly (bf64d66bd 2025-05-21)" => 2,
            _ => return false,
        };
        if found.is_some_and(|old| old != identity) {
            return false;
        }
        found = Some(identity);
    }
    found.is_some()
}

/// `paths` allows the source-path fallback, as in [`fmt_pieces_layout`].
pub fn stable_rustc_version(bytes: &[u8], paths: bool) -> Option<(u16, u16, u16)> {
    let file = object::File::parse(bytes).ok()?;
    if file.format() != BinaryFormat::Elf {
        return None;
    }
    match comment_records(&file) {
        Some(comment) => from_comment(comment),
        None if paths => release(path_commit(bytes)?),
        None => None,
    }
}

/// The compiler version to report for an image of any format: the `.comment`
/// version token, prerelease suffix included, or the release of the commit in
/// its standard library paths.
pub fn rustc_version_label(bytes: &[u8]) -> Option<String> {
    let file = object::File::parse(bytes).ok();
    if let Some(comment) = file.as_ref().and_then(comment_records) {
        return comment_label(comment);
    }
    let (major, minor, patch) = release(path_commit(bytes)?)?;
    Some(format!("{major}.{minor}.{patch}"))
}

/// The `.comment` data when it holds at least one `rustc version` record.
fn comment_records<'a>(file: &object::File<'a>) -> Option<&'a [u8]> {
    let comment = file.section_by_name(".comment")?.data().ok()?;
    comment
        .split(|&b| b == 0)
        .any(|record| record.starts_with(b"rustc version "))
        .then_some(comment)
}

fn comment_label(bytes: &[u8]) -> Option<String> {
    let mut found = None;
    for record in bytes.split(|&b| b == 0) {
        let Some(version) = record.strip_prefix(b"rustc version ") else {
            continue;
        };
        let token = version.split(|b| b.is_ascii_whitespace()).next()?;
        let valid = token.first().is_some_and(u8::is_ascii_digit)
            && token.len() <= 64
            && token
                .iter()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'));
        if !valid || found.is_some_and(|old| old != token) {
            return None;
        }
        found = Some(token);
    }
    found.map(|token| String::from_utf8_lossy(token).into_owned())
}

/// The one compiler commit named by `/rustc/<40 hex>/` paths, `None` when the
/// image has no such path or its paths disagree.
fn path_commit(bytes: &[u8]) -> Option<&[u8; 40]> {
    let mut found: Option<&[u8; 40]> = None;
    for at in memmem::find_iter(bytes, b"/rustc/") {
        let Some(rest) = bytes.get(at + 7..at + 48) else {
            break;
        };
        let (commit, end) = rest.split_first_chunk::<40>()?;
        if !matches!(end, [b'/' | b'\\'])
            || !commit.iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        {
            continue;
        }
        if found.is_some_and(|old| old != commit) {
            return None;
        }
        found = Some(commit);
    }
    found
}

fn release(commit: &[u8; 40]) -> Option<(u16, u16, u16)> {
    RELEASES
        .binary_search_by(|(known, _)| (*known).cmp(commit))
        .ok()
        .map(|at| RELEASES[at].1)
}

fn from_comment(bytes: &[u8]) -> Option<(u16, u16, u16)> {
    let mut found = None;
    for record in bytes.split(|&b| b == 0) {
        let Some(version) = record.strip_prefix(b"rustc version ") else {
            continue;
        };
        let token = std::str::from_utf8(version)
            .ok()?
            .split_ascii_whitespace()
            .next()?;
        let mut fields = token.split('.');
        let mut number = || {
            let field = fields.next()?;
            if field.is_empty() || !field.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            field.parse::<u16>().ok()
        };
        let version = (number()?, number()?, number()?);
        if fields.next().is_some() || found.is_some_and(|old| old != version) {
            return None;
        }
        found = Some(version);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::{comment_label, from_comment, path_commit, pieces_version, release, RELEASES};

    #[test]
    fn release_table_is_sorted_and_maps_commits() {
        assert!(RELEASES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        assert_eq!(release(b"59807616e1fa2540724bfbac14d7976d7e4a3860"), Some((1, 95, 0)));
        assert_eq!(release(b"01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf"), Some((1, 93, 1)));
        assert_eq!(release(b"a59de37e99060162a2674e3ff45409ac73595c0e"), Some((1, 0, 0)));
        assert_eq!(release(b"bf64d66bd58719fac2585eae5e546e5e1d9947f5"), None);
    }

    #[test]
    fn path_commit_requires_one_full_hash() {
        let one = b"x\0/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/fmt/mod.rs\0";
        assert_eq!(path_commit(one), Some(b"59807616e1fa2540724bfbac14d7976d7e4a3860"));
        let windows = b"/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860\\library\\std\\src\\io.rs";
        assert_eq!(path_commit(windows), Some(b"59807616e1fa2540724bfbac14d7976d7e4a3860"));
        let twice = [&one[..], &one[..]].concat();
        assert_eq!(path_commit(&twice), Some(b"59807616e1fa2540724bfbac14d7976d7e4a3860"));
        let mixed = [
            &one[..],
            b"/rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/alloc/src/vec/mod.rs",
        ]
        .concat();
        assert_eq!(path_commit(&mixed), None);
        for bytes in [
            &b"/rustc/59807616e/library/core/src/fmt/mod.rs"[..],
            b"/rustc/59807616E1FA2540724BFBAC14D7976D7E4A3860/library",
            b"/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860",
            b"/rustc/59807616e1fa2540724bfbac14d7976d7e4a38600/library",
            b"/rustc/",
            b"",
        ] {
            assert_eq!(path_commit(bytes), None, "{bytes:?}");
        }
    }

    #[test]
    fn comment_label_keeps_the_prerelease_suffix() {
        assert_eq!(
            comment_label(b"GCC: 12\0rustc version 1.95.0 (59807616e 2026-04-14)\0").as_deref(),
            Some("1.95.0")
        );
        assert_eq!(
            comment_label(b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)\0").as_deref(),
            Some("1.89.0-nightly")
        );
        for bytes in [
            &b"rustc version 1.93.1\0rustc version 1.92.0\0"[..],
            b"rustc version unknown\0",
            b"rustc version \0",
            b"GCC: 12\0",
        ] {
            assert_eq!(comment_label(bytes), None, "{bytes:?}");
        }
    }

    #[test]
    fn pieces_layout_requires_a_verified_compiler_build() {
        for bytes in [
            &b"rustc version 1.89.0 (abc 2025-08-07)\0"[..],
            b"rustc version 1.90.0\0rustc version 1.90.0\0",
            b"GCC\0rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)\0",
        ] {
            assert!(pieces_version(bytes), "{bytes:?}");
        }
        for bytes in [
            &b"rustc version 1.89.0-nightly\0"[..],
            b"rustc version 1.89.0-nightly (bc8215286 2025-05-20)\0",
            b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-22)\0",
            b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-21) extra\0",
            b"rustc version 1.89.0\0rustc version 1.90.0\0",
            b"rustc version 1.89.0\0rustc version unknown\0",
            b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)\0rustc version 1.89.0\0",
            b"rustc version 1.89.1\0",
            b"rustc version 1.93.1\0",
            b"GCC\0",
        ] {
            assert!(!pieces_version(bytes), "{bytes:?}");
        }
    }

    #[test]
    fn stable_version_requires_unambiguous_records() {
        assert_eq!(
            from_comment(b"GCC: 12\0rustc version 1.93.1 (abc 2026-02-11)\0"),
            Some((1, 93, 1))
        );
        assert_eq!(
            from_comment(b"rustc version 1.93.1\0rustc version 1.93.1\0"),
            Some((1, 93, 1))
        );
        for bytes in [
            &b"rustc version 1.93.0-nightly\0"[..],
            b"rustc version 1.93.1\0rustc version 1.92.0\0",
            b"rustc version 1.93.1\0rustc version unknown\0",
            b"rustc version 1.93\0",
            b"rustc version 1.93.1.0\0",
            b"rustc version 1.93.1+local\0",
            b"rustc version 1.93.65536\0",
            b"GCC: 12\0",
            b"rustc version \0",
        ] {
            assert_eq!(from_comment(bytes), None, "{bytes:?}");
        }
    }
}
