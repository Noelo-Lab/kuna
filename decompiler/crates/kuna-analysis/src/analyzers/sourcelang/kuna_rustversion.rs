//! Compiler-version evidence for versioned Rust runtime layouts.

use object::{BinaryFormat, Object, ObjectSection};

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
    use super::from_comment;

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
