//! Explicit Rust signature comparisons; no compiler or layout facts are installed.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use flirt_sig::{Endian, Matcher, Signature};
use object::{BinaryFormat, Object};

use super::{accepted_matches, applies_to, Image};
use crate::listing::Listing;

mod kuna_application;
mod kuna_image;
pub use kuna_image::{CompilerProvenance, ImageReport, Request};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Profile {
    pub version: (u16, u16, u16),
    pub optimization: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bank {
    Inline,
    NoInline,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub profile: Profile,
    pub bank: Bank,
    /// Caller-supplied manifest identity, such as a path and verified content hash.
    pub identity: String,
}

pub struct ProfileSignature {
    source: Source,
    signature: Signature,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportError {
    UnsupportedImage,
    InvalidOptimization(u8),
    InapplicableSignature(String),
    MalformedSignature(String),
    DuplicateBank(Profile, Bank),
}

impl ProfileSignature {
    /// Parse an explicitly identified x86 ELF bank; this does not infer provenance.
    pub fn parse(source: Source, bytes: &[u8]) -> Result<Self, ReportError> {
        if source.profile.optimization > 3 {
            return Err(ReportError::InvalidOptimization(
                source.profile.optimization,
            ));
        }
        if !applies_to(bytes, Some(18)) || bytes[7] != 0 {
            return Err(ReportError::InapplicableSignature(source.identity));
        }
        let signature = Signature::parse(bytes)
            .map_err(|_| ReportError::MalformedSignature(source.identity.clone()))?;
        Ok(Self { source, signature })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Library {
    Core,
    Std,
    Alloc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LibraryMatch {
    pub address: u64,
    pub symbol: String,
    pub demangled: String,
    pub library: Library,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankScore {
    pub source: Source,
    pub signature_library: String,
    pub matches: Vec<LibraryMatch>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateScore {
    pub profile: Profile,
    pub banks: Vec<BankScore>,
    pub missing_banks: Vec<Bank>,
    /// Best bank count; incomplete candidates have no comparable score.
    pub score: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    NoCandidates,
    Incomplete(Vec<Profile>),
    NoMatches,
    Ambiguous(Vec<Profile>),
    Unique(Profile),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub candidates: Vec<CandidateScore>,
    pub selection: Selection,
}

/// Compare only the requested profiles on a caller-selected, common Listing.
/// Both banks must be present for every candidate before a winner can be reported.
pub fn report(
    file: &object::File<'_>,
    listing: &Listing,
    entries: &[u64],
    known: &HashMap<u64, String>,
    candidates: &[Profile],
    signatures: &[ProfileSignature],
) -> Result<Report, ReportError> {
    if file.format() != BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(
            file.kind(),
            object::ObjectKind::Executable | object::ObjectKind::Dynamic
        )
    {
        return Err(ReportError::UnsupportedImage);
    }
    score(
        &Image::new(file),
        listing,
        entries,
        known,
        candidates,
        signatures,
    )
}

fn score(
    image: &Image<'_>,
    listing: &Listing,
    entries: &[u64],
    known: &HashMap<u64, String>,
    candidates: &[Profile],
    signatures: &[ProfileSignature],
) -> Result<Report, ReportError> {
    let candidates: BTreeSet<_> = candidates.iter().copied().collect();
    let mut sources = BTreeMap::new();
    for signature in signatures {
        let source = &signature.source;
        if !candidates.contains(&source.profile) {
            continue;
        }
        if sources
            .insert((source.profile, source.bank), signature)
            .is_some()
        {
            return Err(ReportError::DuplicateBank(source.profile, source.bank));
        }
    }
    let mut entries = entries.to_vec();
    entries.sort_unstable();
    entries.dedup();
    let mut scores = Vec::new();
    for profile in candidates {
        if profile.optimization > 3 {
            return Err(ReportError::InvalidOptimization(profile.optimization));
        }
        let mut banks = Vec::new();
        let mut missing_banks = Vec::new();
        for bank in [Bank::Inline, Bank::NoInline] {
            let Some(signature) = sources.get(&(profile, bank)) else {
                missing_banks.push(bank);
                continue;
            };
            let matcher = Matcher::new(&signature.signature, Endian::Little);
            let matches =
                accepted_matches(&[matcher], image, listing, &entries, known.clone(), true)
                    .into_iter()
                    .filter_map(|(address, symbol)| library_match(address, symbol))
                    .collect();
            banks.push(BankScore {
                source: signature.source.clone(),
                signature_library: signature.signature.header.library_name.clone(),
                matches,
            });
        }
        let score = missing_banks
            .is_empty()
            .then(|| banks.iter().map(|b| b.matches.len()).max().unwrap_or(0));
        scores.push(CandidateScore {
            profile,
            banks,
            missing_banks,
            score,
        });
    }
    let selection = select(&scores);
    Ok(Report {
        candidates: scores,
        selection,
    })
}

fn library_match(address: u64, symbol: String) -> Option<LibraryMatch> {
    let demangled = crate::demangle::demangle_raw(&symbol).unwrap_or_else(|| symbol.clone());
    let path = demangled.trim_start_matches(['<', '&']);
    let library = if path.starts_with("core::") {
        Library::Core
    } else if path.starts_with("std::") {
        Library::Std
    } else if path.starts_with("alloc::") {
        Library::Alloc
    } else {
        return None;
    };
    Some(LibraryMatch {
        address,
        symbol,
        demangled,
        library,
    })
}

fn select(scores: &[CandidateScore]) -> Selection {
    if scores.is_empty() {
        return Selection::NoCandidates;
    }
    let incomplete: Vec<_> = scores
        .iter()
        .filter(|s| s.score.is_none())
        .map(|s| s.profile)
        .collect();
    if !incomplete.is_empty() {
        return Selection::Incomplete(incomplete);
    }
    let best = scores.iter().filter_map(|s| s.score).max().unwrap_or(0);
    if best == 0 {
        return Selection::NoMatches;
    }
    let best: Vec<_> = scores
        .iter()
        .filter(|s| s.score == Some(best))
        .map(|s| s.profile)
        .collect();
    match best.as_slice() {
        [profile] => Selection::Unique(*profile),
        _ => Selection::Ambiguous(best),
    }
}

#[cfg(test)]
mod tests;
