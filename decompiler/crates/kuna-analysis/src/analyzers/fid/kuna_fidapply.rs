//! (kuna) Which `.fid` database applies to an image, and what became of the one
//! a run was given.
//!
//! A database applies when its header language has the image language's
//! processor, address size and instruction byte order, as the `.ldefs` define
//! them: Ghidra's `FidFile.canProcessLanguage` rule, under which the variant and
//! the compiler spec do not matter and a language no `.ldefs` defines applies to
//! nothing. Every configured database is reported as `loaded`, `missing`,
//! `unreadable`, `invalid` or `incompatible`; only a `loaded` one is matched.

use std::path::Path;

use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_langfamily::{language_id, LanguageFamily};

use super::db::{FidDatabase, FidDb};

/// What became of a configured database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FidDbStatus {
    Loaded,
    Missing,
    Unreadable,
    Invalid,
    Incompatible,
}

impl FidDbStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            FidDbStatus::Loaded => "loaded",
            FidDbStatus::Missing => "missing",
            FidDbStatus::Unreadable => "unreadable",
            FidDbStatus::Invalid => "invalid",
            FidDbStatus::Incompatible => "incompatible",
        }
    }
}

/// One configured database: its path as given, its status, and the header
/// facts when the header was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FidDbReport {
    pub path: String,
    pub status: FidDbStatus,
    pub records: Option<usize>,
    pub language: Option<String>,
    pub compiler_spec: Option<String>,
    pub reason: Option<String>,
}

impl FidDbReport {
    /// The sentence a run says about a database it did not match against;
    /// `None` for a loaded one.
    pub fn warning(&self) -> Option<String> {
        let reason = self.reason.as_deref().unwrap_or("");
        let what = match self.status {
            FidDbStatus::Loaded => return None,
            FidDbStatus::Missing => "no such file".to_string(),
            FidDbStatus::Unreadable => format!("cannot be read ({reason})"),
            FidDbStatus::Invalid => format!("not a kuna .fid database ({reason})"),
            FidDbStatus::Incompatible => reason.to_string(),
        };
        Some(format!("{WARNING_PREFIX}{}: {what}; no function was matched against it", self.path))
    }
}

/// How every [`FidDbReport::warning`] sentence starts.
pub const WARNING_PREFIX: &str = "FID database ";

/// Open the database at `path` for `arch`. The database is returned only when
/// it is `loaded`.
pub fn open(path: &Path, arch: &Architecture) -> (FidDbReport, Option<FidDb>) {
    let mut report = FidDbReport {
        path: path.to_string_lossy().into_owned(),
        status: FidDbStatus::Loaded,
        records: None,
        language: None,
        compiler_spec: None,
        reason: None,
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            report.status = if e.kind() == std::io::ErrorKind::NotFound {
                FidDbStatus::Missing
            } else {
                FidDbStatus::Unreadable
            };
            report.reason = Some(e.to_string());
            return (report, None);
        }
    };
    let db = match FidDb::load(&bytes) {
        Ok(db) => db,
        Err(e) => {
            report.status = FidDbStatus::Invalid;
            report.reason = Some(e.explain().to_string());
            return (report, None);
        }
    };
    report.records = Some(db.record_count());
    report.language = Some(db.lang_id().to_string());
    report.compiler_spec = Some(db.cspec_id().to_string());
    if let Err(reason) = applies(db.lang_id(), arch) {
        report.status = FidDbStatus::Incompatible;
        report.reason = Some(reason);
        return (report, None);
    }
    (report, Some(db))
}

/// `Ok` when a database built for `db_language` applies to `arch`'s image;
/// otherwise the reason it does not. An `Architecture` that recorded no
/// `.ldefs` languages falls back to exact language-id equality.
pub fn applies(db_language: &str, arch: &Architecture) -> Result<(), String> {
    let image_language = language_id(&arch.archid);
    if !arch.has_language_families() {
        return if db_language == image_language {
            Ok(())
        } else {
            Err(format!(
                "database language {db_language} differs from image language {image_language}"
            ))
        };
    }
    let Some(db) = arch.language_family(db_language) else {
        return Err(format!(
            "database language {db_language:?} is not a language the loaded specs define"
        ));
    };
    let Some(image) = arch.language_family(image_language) else {
        return if db_language == image_language {
            Ok(())
        } else {
            Err(format!(
                "database language {db_language} differs from image language {image_language}"
            ))
        };
    };
    let differs = differences(db, image);
    if differs.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "database language {db_language} ({}) does not apply to image language \
             {image_language} ({}): the {} differs",
            describe(db),
            describe(image),
            differs.join(" and the ")
        ))
    }
}

/// The facts on which `a` and `b` disagree; empty when one applies to the other.
fn differences(a: &LanguageFamily, b: &LanguageFamily) -> Vec<&'static str> {
    [
        (a.processor != b.processor, "processor"),
        (a.size != b.size, "address size"),
        (a.instruction_big_endian != b.instruction_big_endian, "instruction byte order"),
    ]
    .into_iter()
    .filter_map(|(differ, what)| differ.then_some(what))
    .collect()
}

fn describe(f: &LanguageFamily) -> String {
    let order = if f.instruction_big_endian { "big" } else { "little" };
    format!("{} {}-bit, {order}-endian instructions", f.processor, f.size)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn family(processor: &str, size: i32, big: bool) -> LanguageFamily {
        LanguageFamily { processor: processor.into(), size, instruction_big_endian: big }
    }

    #[test]
    fn processor_size_and_instruction_order_decide() {
        let x86_64 = family("x86", 64, false);
        assert!(differences(&x86_64, &family("x86", 64, false)).is_empty());
        assert_eq!(differences(&x86_64, &family("x86", 32, false)), ["address size"]);
        assert_eq!(differences(&x86_64, &family("AARCH64", 64, false)), ["processor"]);
        assert_eq!(
            differences(&family("ARM", 32, false), &family("PowerPC", 64, true)),
            ["processor", "address size", "instruction byte order"]
        );
    }

    #[test]
    fn only_a_loaded_database_is_silent() {
        let mut report = FidDbReport {
            path: "lib.fid".into(),
            status: FidDbStatus::Loaded,
            records: Some(2),
            language: Some("x86:LE:64:default".into()),
            compiler_spec: Some("gcc".into()),
            reason: None,
        };
        assert_eq!(report.warning(), None);
        report.status = FidDbStatus::Missing;
        assert_eq!(
            report.warning().as_deref(),
            Some("FID database lib.fid: no such file; no function was matched against it")
        );
    }
}
