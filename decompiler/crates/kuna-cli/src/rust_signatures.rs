//! Verified external Rust signature inputs and the image-wide summary report.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kuna_analysis::flirt::kuna_rustprofiles::{
    Bank, ImageReport, Library, Profile, ProfileSignature, Request, Selection, Source,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::jsonfmt::Json;

mod kuna_comment_profile;
mod kuna_installed_manifest;

pub(crate) use kuna_installed_manifest::run;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Corpus {
    identity: String,
    pin: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Candidate {
    version: String,
    optimization: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ManifestBank {
    Inline,
    NoInline,
}

impl ManifestBank {
    fn bank(self) -> Bank {
        match self {
            Self::Inline => Bank::Inline,
            Self::NoInline => Bank::NoInline,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignatureInput {
    path: PathBuf,
    sha256: String,
    version: String,
    optimization: u8,
    bank: ManifestBank,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    corpus: Corpus,
    candidates: Vec<Candidate>,
    signatures: Vec<SignatureInput>,
}

#[derive(Debug, Serialize)]
struct VerifiedSignature {
    path: String,
    sha256: String,
    version: String,
    optimization: u8,
    bank: ManifestBank,
}

#[derive(Debug, Serialize)]
pub(crate) struct ManifestInfo {
    manifest_path: String,
    manifest_sha256: String,
    corpus: Corpus,
    candidates: Vec<Candidate>,
    signatures: Vec<VerifiedSignature>,
}

pub(crate) fn load(path: &Path) -> Result<(Request, ManifestInfo), String> {
    let path = path
        .canonicalize()
        .map_err(|e| format!("Rust signature manifest {}: {e}", path.display()))?;
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("Rust signature manifest {}: {e}", path.display()))?;
    let manifest: Manifest = serde_json::from_slice(&bytes)
        .map_err(|e| format!("invalid Rust signature manifest {}: {e}", path.display()))?;
    if manifest.corpus.identity.trim().is_empty() || manifest.corpus.pin.trim().is_empty() {
        return Err("Rust signature corpus identity and pin must be nonempty".into());
    }
    if manifest.candidates.is_empty() {
        return Err("Rust signature manifest requires explicit candidates".into());
    }
    let mut candidates = Vec::new();
    let mut seen_candidates = BTreeSet::new();
    for candidate in &manifest.candidates {
        let profile = profile(&candidate.version, candidate.optimization)?;
        if !seen_candidates.insert(profile) {
            return Err(format!(
                "duplicate Rust signature candidate {} O{}",
                candidate.version, candidate.optimization
            ));
        }
        candidates.push(profile);
    }
    let mut signatures = Vec::new();
    let mut verified = Vec::new();
    let mut seen_banks = BTreeSet::new();
    let mut seen_paths = BTreeSet::new();
    for signature in manifest.signatures {
        let profile = profile(&signature.version, signature.optimization)?;
        if !seen_candidates.contains(&profile) {
            return Err(format!(
                "Rust signature {} does not belong to an explicit candidate",
                signature.path.display()
            ));
        }
        if !seen_banks.insert((profile, signature.bank.bank())) {
            return Err(format!(
                "duplicate Rust signature bank for {} O{} {:?}",
                signature.version, signature.optimization, signature.bank
            ));
        }
        if signature.sha256.len() != 64
            || !signature
                .sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(format!(
                "Rust signature {} requires a lowercase SHA-256 digest",
                signature.path.display()
            ));
        }
        if signature.path.as_os_str().is_empty() {
            return Err("Rust signature path must be nonempty".into());
        }
        let source_path = path
            .parent()
            .unwrap_or(Path::new("."))
            .join(&signature.path);
        let source_path = source_path
            .canonicalize()
            .map_err(|e| format!("Rust signature {}: {e}", source_path.display()))?;
        if !seen_paths.insert(source_path.clone()) {
            return Err(format!(
                "duplicate Rust signature path {}",
                source_path.display()
            ));
        }
        let source_bytes = std::fs::read(&source_path)
            .map_err(|e| format!("Rust signature {}: {e}", source_path.display()))?;
        let digest = sha256(&source_bytes);
        if digest != signature.sha256 {
            return Err(format!(
                "Rust signature SHA-256 mismatch: {}",
                source_path.display()
            ));
        }
        let identity = format!("sha256:{digest} {}", source_path.display());
        signatures.push(
            ProfileSignature::parse(
                Source {
                    profile,
                    bank: signature.bank.bank(),
                    identity,
                },
                &source_bytes,
            )
            .map_err(|e| format!("invalid Rust signature {}: {e:?}", source_path.display()))?,
        );
        verified.push(VerifiedSignature {
            path: source_path.to_string_lossy().into_owned(),
            sha256: digest,
            version: signature.version,
            optimization: signature.optimization,
            bank: signature.bank,
        });
    }
    Ok((
        Request {
            candidates,
            signatures,
        },
        ManifestInfo {
            manifest_path: path.to_string_lossy().into_owned(),
            manifest_sha256: sha256(&bytes),
            corpus: manifest.corpus,
            candidates: manifest.candidates,
            signatures: verified,
        },
    ))
}

fn profile(version: &str, optimization: u8) -> Result<Profile, String> {
    let fields: Vec<_> = version.split('.').collect();
    if fields.len() != 3
        || fields.iter().any(|field| {
            field.is_empty()
                || !field.bytes().all(|b| b.is_ascii_digit())
                || (field.len() > 1 && field.starts_with('0'))
        })
    {
        return Err(format!(
            "invalid Rust signature version {version:?}: expected MAJOR.MINOR.PATCH"
        ));
    }
    let mut numbers = [0; 3];
    for (number, field) in numbers.iter_mut().zip(fields) {
        *number = field
            .parse::<u16>()
            .map_err(|_| format!("Rust signature version out of range: {version:?}"))?;
    }
    if optimization > 3 {
        return Err(format!(
            "invalid Rust signature optimization {optimization}: expected 0 through 3"
        ));
    }
    Ok(Profile {
        version: (numbers[0], numbers[1], numbers[2]),
        optimization,
    })
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn profile_json(profile: Profile) -> Value {
    let (major, minor, patch) = profile.version;
    json!({"version": format!("{major}.{minor}.{patch}"), "optimization": profile.optimization})
}

fn bank_name(bank: Bank) -> &'static str {
    match bank {
        Bank::Inline => "inline",
        Bank::NoInline => "no-inline",
    }
}

pub(crate) fn fields(manifest: &ManifestInfo, image: Option<&ImageReport>) -> Vec<(String, Json)> {
    let compiler = image.map_or_else(
        || json!({"rustc_records": [], "source_commits": []}),
        |image| json!({"rustc_records": image.compiler.rustc_records, "source_commits": image.compiler.source_commits}),
    );
    let mut candidates = Vec::new();
    let mut selected = Value::Null;
    let mut tied = Vec::new();
    let mut comment_hint = None;
    let status = if let Some(image) = image {
        if let Some(error) = &image.error {
            if error == "unsupported_image" {
                "unsupported"
            } else {
                "unavailable"
            }
        } else if let Some(report) = &image.report {
            comment_hint = kuna_comment_profile::hint(&image.compiler, report);
            for candidate in &report.candidates {
                candidates.push(json!({
                    "profile": profile_json(candidate.profile),
                    "score": candidate.score,
                    "missing_banks": candidate.missing_banks.iter().copied().map(bank_name).collect::<Vec<_>>(),
                    "banks": candidate.banks.iter().map(|bank| json!({
                        "bank": bank_name(bank.source.bank), "source_identity": bank.source.identity,
                        "signature_library": bank.signature_library, "match_count": bank.matches.len(),
                        "matches": bank.matches.iter().map(|matched| json!({
                            "address": matched.address, "address_hex": format!("0x{:x}", matched.address),
                            "symbol": matched.symbol, "demangled": matched.demangled,
                            "library": match matched.library { Library::Core => "core", Library::Std => "std", Library::Alloc => "alloc" }
                        })).collect::<Vec<_>>()
                    })).collect::<Vec<_>>()
                }));
            }
            match &report.selection {
                Selection::Unique(profile) => {
                    selected = profile_json(*profile);
                    "unique"
                }
                Selection::Ambiguous(profiles) => {
                    tied = profiles.iter().copied().map(profile_json).collect();
                    "ambiguous"
                }
                Selection::Incomplete(_) => "incomplete",
                Selection::NoMatches => "no_matches",
                Selection::NoCandidates => "no_candidates",
            }
        } else {
            "unavailable"
        }
    } else {
        "unavailable"
    };
    let mut report = json!({
        "status": status, "selected_profile": selected, "tied_profiles": tied,
        "scope": {"listing": "final_accepted", "entries": "text_section_tails", "tail_bytes": 131072, "summary_filters_applied": false},
        "entry_count": image.map(|image| image.entry_count),
        "reason": image.and_then(|image| image.error.as_deref()),
        "provenance": manifest, "candidates": candidates,
    });
    if let Some(hint) = comment_hint {
        report["comment_profile_hint"] = hint;
    }
    vec![
        ("compiler_provenance".into(), ordered(compiler)),
        ("rust_signature_profile".into(), ordered(report)),
    ]
}

fn ordered(value: Value) -> Json {
    match value {
        Value::Null => Json::Null,
        Value::Bool(value) => Json::Bool(value),
        Value::Number(value) => Json::Number(value.to_string()),
        Value::String(value) => Json::Str(value),
        Value::Array(values) => Json::Array(values.into_iter().map(ordered).collect()),
        Value::Object(values) => Json::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, ordered(value)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests;

/// The application and its scoring must observe the same unmodified linked image.
pub(crate) fn validate_application(args: &crate::decompile_all::Args) -> Result<(), String> {
    use kuna_console::assertions::Body;
    if args.raw_image || args.slice.is_some() || args.target.is_some() || args.isa.is_some()
        || args.assertions.iter().any(|directive| matches!(directive.body, Body::Bytes { .. }))
        || args.options.iter().any(|(name, _)| matches!(name.as_str(), "dyldimage" | "kernelcache"))
    {
        return Err("--apply-rust-signatures requires an unmodified object image without raw, slice, target, ISA, image-selection or bytes overrides".into());
    }
    if args.jobs != 1 || args.jobs_auto || args.jobs_worker.is_some() {
        return Err("--apply-rust-signatures requires a single-process functions or decompile run".into());
    }
    Ok(())
}
