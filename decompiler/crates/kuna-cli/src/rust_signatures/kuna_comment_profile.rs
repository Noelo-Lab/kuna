//! Derive an O3 bank hint from stable compiler metadata and bank availability.

use kuna_analysis::flirt::kuna_rustprofiles::{CompilerProvenance, Report};
use serde_json::{json, Value};

use super::{profile, profile_json};

pub(super) fn hint(compiler: &CompilerProvenance, report: &Report) -> Option<Value> {
    let mut versions = compiler.rustc_records.iter().map(|record| {
        let token = record
            .strip_prefix("rustc version ")?
            .split_ascii_whitespace()
            .next()?;
        Some(profile(token, 3).ok()?.version)
    });
    let version = versions.next()??;
    if versions.any(|other| other != Some(version)) {
        return None;
    }

    let requested = report.candidates.iter().filter(|candidate| {
        candidate.profile.optimization == 3 && candidate.profile.version.0 == version.0
    });
    let complete = requested
        .clone()
        .filter(|candidate| candidate.missing_banks.is_empty())
        .map(|candidate| candidate.profile);
    let same_minor = complete
        .clone()
        .filter(|candidate| candidate.version.1 == version.1 && candidate.version <= version)
        .max();

    let (candidate, basis) = if let Some(candidate) = same_minor {
        let basis = if candidate.version == version {
            "exact_version"
        } else {
            "same_minor"
        };
        (candidate, basis)
    } else if version.1
        > requested
            .map(|candidate| candidate.profile.version.1)
            .max()?
    {
        (complete.max()?, "newer_than_available")
    } else {
        return None;
    };
    Some(json!({"profile": profile_json(candidate), "basis": basis}))
}
