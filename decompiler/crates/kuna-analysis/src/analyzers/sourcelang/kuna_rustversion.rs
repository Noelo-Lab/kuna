//! Compiler-version evidence for versioned Rust runtime layouts.

use object::{BinaryFormat, Object, ObjectSection};

pub struct RustFmtPieces {
    pub relro: Option<(u64, u64)>,
}

pub fn fmt_pieces_layout(bytes: &[u8]) -> Option<RustFmtPieces> {
    let file = object::File::parse(bytes).ok()?;
    if file.format() != BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(
            file.kind(),
            object::ObjectKind::Executable | object::ObjectKind::Dynamic
        )
        || !pieces_version(file.section_by_name(".comment")?.data().ok()?)
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

pub fn stable_rustc_version(bytes: &[u8]) -> Option<(u16, u16, u16)> {
    let file = object::File::parse(bytes).ok()?;
    if file.format() != BinaryFormat::Elf {
        return None;
    }
    from_comment(file.section_by_name(".comment")?.data().ok()?)
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
    use super::{from_comment, pieces_version};

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
