//! Image-level reports keep observed compiler text separate from signature scores.

use std::collections::{BTreeSet, HashMap};

use object::{Object, ObjectSection, ObjectSymbol};

use super::{report, Profile, ProfileSignature, Report};
use crate::listing::Listing;

pub struct Request {
    pub candidates: Vec<Profile>,
    pub signatures: Vec<ProfileSignature>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompilerProvenance {
    pub rustc_records: Vec<String>,
    pub source_commits: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageReport {
    pub compiler: CompilerProvenance,
    pub report: Option<Report>,
    pub error: Option<String>,
    pub entry_count: usize,
}

impl ImageReport {
    pub fn unavailable() -> Self {
        Self {
            compiler: CompilerProvenance::default(),
            report: None,
            error: Some("no accepted object Listing was produced".into()),
            entry_count: 0,
        }
    }

    pub fn unavailable_for(bytes: &[u8]) -> Self {
        let mut result = Self::unavailable();
        if let Ok(file) = object::File::parse(bytes) {
            result.compiler = compiler_provenance(&file);
            if file.format() != object::BinaryFormat::Elf
                || file.architecture() != object::Architecture::X86_64
                || !file.is_64()
                || !file.is_little_endian()
                || !matches!(
                    file.kind(),
                    object::ObjectKind::Executable | object::ObjectKind::Dynamic
                )
            {
                result.error = Some("unsupported_image".into());
            }
        }
        result
    }

    pub fn reject_unsupported(bytes: &[u8]) -> Option<Self> {
        let file = object::File::parse(bytes).ok()?;
        if file.format() == object::BinaryFormat::Elf
            && file.architecture() == object::Architecture::X86_64
            && file.is_64()
            && file.is_little_endian()
            && matches!(
                file.kind(),
                object::ObjectKind::Executable | object::ObjectKind::Dynamic
            )
        {
            None
        } else {
            Some(Self::unavailable_for(bytes))
        }
    }
}

impl Request {
    /// Observe a completed Listing without changing names, options or layout facts.
    pub fn evaluate(&self, file: &object::File<'_>, listing: &Listing) -> ImageReport {
        let tails: Vec<_> = file
            .sections()
            .filter(|section| section.kind() == object::SectionKind::Text)
            .map(|section| {
                let end = section.address().saturating_add(section.size());
                (end.saturating_sub(section.size().min(128 * 1024)), end)
            })
            .collect();
        let entries: Vec<_> = listing
            .functions()
            .map(|(&address, _)| address)
            .filter(|address| {
                tails
                    .iter()
                    .any(|&(lo, hi)| lo <= *address && *address < hi)
            })
            .collect();
        let mut known = HashMap::new();
        let mut conflicting = BTreeSet::new();
        for symbol in file
            .symbols()
            .chain(file.dynamic_symbols())
            .filter(|symbol| symbol.is_definition() && symbol.kind() == object::SymbolKind::Text)
        {
            if let Ok(name) = symbol.name() {
                if known
                    .insert(symbol.address(), name.to_owned())
                    .is_some_and(|old| old != name)
                {
                    conflicting.insert(symbol.address());
                }
            }
        }
        known.retain(|address, _| !conflicting.contains(address));
        let mut result = ImageReport {
            compiler: compiler_provenance(file),
            report: None,
            error: None,
            entry_count: entries.len(),
        };
        match report(
            file,
            listing,
            &entries,
            &known,
            &self.candidates,
            &self.signatures,
        ) {
            Ok(report) => result.report = Some(report),
            Err(super::ReportError::UnsupportedImage) => {
                result.error = Some("unsupported_image".into())
            }
            Err(error) => result.error = Some(format!("{error:?}")),
        }
        result
    }
}

fn compiler_provenance(file: &object::File<'_>) -> CompilerProvenance {
    let comment = file
        .section_by_name(".comment")
        .and_then(|s| s.data().ok())
        .unwrap_or_default();
    let rodata = file
        .section_by_name(".rodata")
        .and_then(|s| s.data().ok())
        .unwrap_or_default();
    from_sections(comment, rodata)
}

fn from_sections(comment: &[u8], rodata: &[u8]) -> CompilerProvenance {
    let rustc_records = comment
        .split(|&byte| byte == 0)
        .filter(|record| record.starts_with(b"rustc version "))
        .map(|record| String::from_utf8_lossy(record).into_owned())
        .collect();
    let mut commits = BTreeSet::new();
    for offset in memchr::memmem::find_iter(rodata, b"/rustc/") {
        let Some(suffix) = rodata.get(offset + 7..) else {
            continue;
        };
        let Some(hash) = suffix.get(..40) else {
            continue;
        };
        if hash
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
            && suffix
                .get(40..)
                .is_some_and(|tail| tail.starts_with(b"/library/"))
        {
            commits.insert(String::from_utf8_lossy(hash).into_owned());
        }
    }
    CompilerProvenance {
        rustc_records,
        source_commits: commits.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_preserves_nightly_and_conflicting_records_without_selecting_a_version() {
        let record = "rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)";
        let comments = format!("GCC\0{record}\0rustc version 1.90.0\0");
        let result = from_sections(comments.as_bytes(), b"");
        assert_eq!(result.rustc_records, [record, "rustc version 1.90.0"]);
        assert!(result.source_commits.is_empty());
    }

    #[test]
    fn only_complete_rust_library_source_commits_are_reported() {
        let hash = "9d871b0617a4b3d6610b7cee0ab5310dcb542c62";
        let text = format!("/rustc/{hash}/library/core/src/lib.rs\0/rustc/{hash}/library/std/src/lib.rs\0/rustc/{hash}/user/payload\0/rustc/9d871b061/library/core/src/lib.rs");
        let result = from_sections(b"GCC\0not rustc version 1.71.0\0", text.as_bytes());
        assert!(result.rustc_records.is_empty());
        assert_eq!(result.source_commits, [hash]);
    }
}
