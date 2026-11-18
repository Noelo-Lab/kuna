//! The analysis warnings a render writes, each as the engine recorded it.
//!
//! The printer records a [`Warning`] when it writes a `Funcdata::warning` or
//! `warning_header` comment (`emit_comment_group`, `emit_comment_func_header`,
//! `flush_eol_warnings`) and when it writes its own no-structure shell, so the
//! record carries the address the warning was indexed at and the line of the
//! render holding its comment. A comment of any other type (a user comment, a
//! storage or name comment) is never recorded, whatever it spells. Taken with
//! [`PrintC::take_warnings`](crate::printc::PrintC::take_warnings).

use crate::kuna_decodehalt::{
    BAD_INSTRUCTION_HEADER, BAD_INSTRUCTION_WARNING, UNIMPLEMENTED_HEADER,
    UNIMPLEMENTED_IGNORED_HEADER, UNIMPLEMENTED_WARNING,
};
use crate::printc::warning_slug;

/// Engine warning stems `warning_slug` keeps unabbreviated, with their kinds.
pub const STEMS: [(&str, &str); 19] = [
    (BAD_INSTRUCTION_WARNING, "bad-instruction"),
    (BAD_INSTRUCTION_HEADER, "bad-instruction"),
    (UNIMPLEMENTED_WARNING, "unimplemented-instruction"),
    (UNIMPLEMENTED_HEADER, "unimplemented-instruction"),
    (UNIMPLEMENTED_IGNORED_HEADER, "unimplemented-instruction"),
    ("Too many instructions -- truncating flow here", "instruction-limit"),
    ("Exceeded the ", "instruction-limit"),
    ("Function flow out of bounds", "out-of-bounds"),
    ("Function flows out of bounds", "out-of-bounds"),
    ("Function flow reaches unmapped memory", "unmapped-flow"),
    ("Function flows into unmapped memory", "unmapped-flow"),
    ("Could not recover jumptable", "jumptable"),
    ("Switch model does not reproduce", "jumptable"),
    ("Ignoring partial resolution of indirect", "jumptable"),
    ("Second-stage recovery error", "jumptable"),
    ("Sanity check requires truncation of jumptable", "jumptable"),
    ("Nullable weak ", "weak-relocation"),
    ("structured blocks unavailable", "structuring-declined"),
    ("decompilation failed", "decompilation-failed"),
];

/// One warning a render wrote: the byte address it was indexed at (the entry
/// for a header warning; `None` only for the printer's own shell), its stored
/// text and the 1-based line holding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    pub address: Option<u64>,
    pub text: String,
    pub line: usize,
}

impl Warning {
    pub(crate) fn new(address: Option<u64>, text: &str) -> Self {
        Self { address, text: text.to_string(), line: 0 }
    }

    /// The text less its `WARNING: ` marker; a jump-table recovery warning keeps
    /// `(jumptable): `.
    pub fn message(&self) -> String {
        match self.text.strip_prefix("WARNING (jumptable): ") {
            Some(body) => format!("(jumptable): {body}"),
            None => self.text.strip_prefix("WARNING: ").unwrap_or(&self.text).to_string(),
        }
    }

    /// A stable code: the kind of an unabbreviated engine stem, else the slug
    /// `warning_slug` gives the text (less its count), else the producer tag
    /// of a kuna warning, else `other`.
    pub fn kind(&self) -> String {
        let body = self.text.strip_prefix("WARNING (jumptable): ");
        let body = body.or_else(|| self.text.strip_prefix("WARNING: ")).unwrap_or(&self.text);
        if let Some((_, kind)) = STEMS.iter().find(|(stem, _)| body.starts_with(stem)) {
            return kind.to_string();
        }
        let slug = warning_slug(&self.text);
        let slug = slug.strip_prefix("jt: ").unwrap_or(&slug);
        if slug.starts_with("inlined: ") {
            return "inlined".to_string();
        }
        if !slug.starts_with("warn: ") {
            return match slug.rsplit_once(" x") {
                Some((base, n)) if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) => base,
                _ => slug,
            }
            .to_string();
        }
        match body.split_once(": ") {
            Some((tag, _))
                if tag.starts_with(|c: char| c.is_ascii_lowercase())
                    && tag.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()) =>
            {
                tag.to_string()
            }
            _ => "other".to_string(),
        }
    }
}

/// Rebase lines counted in `untrimmed` onto it with its leading newlines removed.
pub(crate) fn trim_lines(warnings: &mut [Warning], untrimmed: &str) {
    let skip = untrimmed.len() - untrimmed.trim_start_matches('\n').len();
    for warning in warnings {
        warning.line = warning.line.saturating_sub(skip);
    }
}

#[cfg(test)]
mod tests;
