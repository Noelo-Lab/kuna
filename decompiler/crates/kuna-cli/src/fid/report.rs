//! What became of each `.fid` database a run was configured with, on stderr and
//! in the `fid_databases` field of the JSON documents.

use kuna_analysis::fid::kuna_fidapply::FidDbReport;
use kuna_console::engine::ConsoleProgram;

use crate::jsonfmt::Json;

/// Say once on stderr, as a `warning:`, why each configured database that was
/// not loaded did not take part.
pub(crate) fn warn(prog: &ConsoleProgram, binary: &str) {
    for warning in warnings(prog.fid_databases()) {
        kuna_base::notes::say_once(binary, &format!("warning: {warning}"));
    }
}

/// The sentences [`warn`] prints, without the `warning: ` prefix.
pub(crate) fn warnings(reports: &[FidDbReport]) -> impl Iterator<Item = String> + '_ {
    reports.iter().filter_map(FidDbReport::warning)
}

/// The `warning:` lines a `decomp_dbg` transcript's `read symbols` printed for
/// its databases, as [`warn`] would print them.
pub(crate) fn from_transcript(transcript: &str) -> impl Iterator<Item = &str> {
    transcript
        .lines()
        .map(|line| {
            let line = line.trim();
            line.strip_prefix("[decomp]>").unwrap_or(line).trim()
        })
        .filter(|line| {
            line.strip_prefix("warning: ")
                .is_some_and(|rest| rest.starts_with(kuna_analysis::fid::kuna_fidapply::WARNING_PREFIX))
        })
}

/// The `fid_databases` field, absent when no database was configured or FID
/// did not run.
pub(crate) fn field(reports: &[FidDbReport]) -> Option<(String, Json)> {
    (!reports.is_empty()).then(|| ("fid_databases".into(), Json::Array(reports.iter().map(record).collect())))
}

/// The `--summary` text form's line for each report.
pub(crate) fn text_lines(reports: &[FidDbReport]) -> impl Iterator<Item = String> + '_ {
    reports.iter().map(|r| {
        let records = r.records.map_or_else(|| "-".to_string(), |n| n.to_string());
        format!("fid database\t{}\t{records}\t{}", r.status.as_str(), r.path)
    })
}

fn record(r: &FidDbReport) -> Json {
    let text = |value: &Option<String>| value.clone().map_or(Json::Null, Json::Str);
    Json::Object(vec![
        ("path".into(), Json::Str(r.path.clone())),
        ("status".into(), Json::Str(r.status.as_str().into())),
        ("records".into(), r.records.map_or(Json::Null, |n| Json::Number(n.to_string()))),
        ("language".into(), text(&r.language)),
        ("compiler_spec".into(), text(&r.compiler_spec)),
        ("reason".into(), text(&r.reason)),
    ])
}
