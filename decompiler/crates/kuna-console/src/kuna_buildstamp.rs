//! The build handshake between `kuna` and the engine binaries it spawns.
//!
//! `kuna decompile` and `kuna catalog` run `decomp_dbg`, and `kuna test` runs
//! `decomp_test_dbg`. Either can come from a different build than the `kuna`
//! that spawned it -- `KUNA_DECOMP_DBG` pointing at another install, or a sibling
//! left behind when only `kuna` was rebuilt -- and nothing in the render says so.
//!
//! The parent exports its [`identity`] as [`PARENT_ENV`]. The child compares it
//! with its own at startup ([`answer_parent`]) and, only when they differ, writes
//! one [`REPLY`] line carrying its own identity to stderr. The parent strips that
//! line from the stderr it captured and names both builds in one warning
//! ([`take_reply`]). Matching builds exchange nothing but the variable; a child
//! that predates the handshake ignores it and so cannot warn. Stdout is never
//! written: it carries the console transcript and the datatest grammar.
//!
//! The variable name and the reply prefix are read by builds that differ by
//! definition, so neither may change.
//!
//! An identity is the version `kuna --version` prints plus a fingerprint of the
//! engine sources this crate links (`build.rs`). The version alone cannot tell
//! two source builds apart: every one reports the workspace Cargo version.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

/// The environment variable the parent sets to its own [`identity`].
pub const PARENT_ENV: &str = "KUNA_PARENT_BUILD";

/// The stderr line prefix a mismatched child answers with.
pub const REPLY: &str = "kuna-build-mismatch: ";

/// The release `MAJOR.MINOR` baked by release CI, else the workspace Cargo version.
pub const VERSION: &str = match option_env!("KUNA_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};

/// This build's identity: `<version> (source <fingerprint>)`.
pub fn identity() -> &'static str {
    static ID: OnceLock<String> = OnceLock::new();
    ID.get_or_init(|| format!("{VERSION} (source {})", env!("KUNA_SOURCE_FINGERPRINT")))
}

/// The child half: answer a parent whose identity differs from ours.
pub fn answer_parent() {
    if let Some(parent) = std::env::var_os(PARENT_ENV) {
        if parent != identity() {
            eprintln!("{REPLY}{}", identity());
        }
    }
}

/// Remove every reply line from a child's stderr, returning the remaining text
/// and the identity the child answered with, if it answered.
pub fn take_reply(stderr: &str) -> (String, Option<String>) {
    if !stderr.contains(REPLY) {
        return (stderr.to_string(), None);
    }
    let mut rest = String::with_capacity(stderr.len());
    let mut theirs = None;
    for line in stderr.split_inclusive('\n') {
        match line.strip_prefix(REPLY) {
            Some(id) => theirs = Some(id.trim_end_matches(['\r', '\n']).to_string()),
            None => rest.push_str(line),
        }
    }
    (rest, theirs)
}

/// The warning for a child that answered `theirs`. `pinned_by` names the flag or
/// variable that chose the child, when one did.
pub fn mismatch_warning(
    child_name: &str,
    child: &Path,
    theirs: &str,
    pinned_by: Option<&str>,
) -> String {
    let kuna = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "kuna".to_string());
    let width = child_name.len().max("kuna".len()) + 1;
    let pinned = pinned_by
        .map(|p| format!(", chosen by {p}"))
        .unwrap_or_default();
    format!(
        "warning: {child_name} is a different build from this kuna\n\
         \x20 {:<width$} {} {kuna}\n\
         \x20 {:<width$} {theirs} {}{pinned}\n",
        "kuna:",
        identity(),
        format!("{child_name}:"),
        child.display(),
    )
}

/// The parent half, after the child exited: strip its reply from `stderr` and,
/// the first time in this process that a child answered, print the warning.
pub fn report(stderr: String, child_name: &str, child: &Path, pinned_by: Option<&str>) -> String {
    static WARNED: AtomicBool = AtomicBool::new(false);
    let (rest, theirs) = take_reply(&stderr);
    if let Some(theirs) = theirs {
        if !WARNED.swap(true, Ordering::Relaxed) {
            eprint!(
                "{}",
                mismatch_warning(child_name, child, &theirs, pinned_by)
            );
        }
    }
    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reply_is_stripped_and_the_rest_kept_verbatim() {
        let stderr = format!("{REPLY}1.329 (source 00ff)\nUnable to write x\r\ntail");
        let (rest, theirs) = take_reply(&stderr);
        assert_eq!(theirs.as_deref(), Some("1.329 (source 00ff)"));
        assert_eq!(rest, "Unable to write x\r\ntail");
    }

    #[test]
    fn stderr_without_a_reply_is_untouched() {
        let (rest, theirs) = take_reply("  passed.\n0/0 tests passed.\n");
        assert_eq!(theirs, None);
        assert_eq!(rest, "  passed.\n0/0 tests passed.\n");
        assert_eq!(take_reply(""), (String::new(), None));
    }

    #[test]
    fn a_prefix_inside_a_line_is_not_a_reply() {
        let stderr = format!("note: {REPLY}x\n");
        assert_eq!(take_reply(&stderr), (stderr.clone(), None));
    }

    #[test]
    fn the_warning_names_both_builds_and_the_override() {
        let text = mismatch_warning(
            "decomp_dbg",
            Path::new("/opt/kuna-v1.329/decomp_dbg"),
            "1.329 (source 00ff)",
            Some("KUNA_DECOMP_DBG"),
        );
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 3, "{text}");
        assert_eq!(
            lines[0],
            "warning: decomp_dbg is a different build from this kuna"
        );
        assert!(
            lines[1].starts_with("  kuna:       ") && lines[1].contains(identity()),
            "{text}"
        );
        assert_eq!(
            lines[2],
            "  decomp_dbg: 1.329 (source 00ff) /opt/kuna-v1.329/decomp_dbg, chosen by KUNA_DECOMP_DBG"
        );
    }

    #[test]
    fn the_identity_carries_the_version_and_a_fingerprint() {
        let id = identity();
        assert!(id.starts_with(VERSION), "{id}");
        let fp = id
            .strip_prefix(&format!("{VERSION} (source "))
            .and_then(|s| s.strip_suffix(')'));
        assert!(
            fp.is_some_and(|f| f.len() == 16 && f.chars().all(|c| c.is_ascii_hexdigit())),
            "{id}"
        );
    }
}
