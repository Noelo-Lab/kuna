//! `kuna bsim` — build a function-similarity corpus and query it.
//!
//! BSim answers a question FID cannot: "which functions in this corpus *look
//! like* this one?". kuna decompiles a function, turns the decompiler's feature
//! vector into a weighted LSH vector, and compares it with a stored corpus.
//!
//! ```text
//!   kuna bsim build <binary> -o <corpus.kbsm> [--name N] [--max-functions N]
//!   kuna bsim query <binary> --corpus <corpus.kbsm> [--address A | --name N]
//!                    [--min-similarity S] [--max-matches N] [--json]
//!   kuna bsim info <corpus.kbsm> [--json]
//! ```
//!
//! # Weights
//!
//! Scoring needs Ghidra's weight tables (`lshweights_*.xml`). They are looked up
//! where Ghidra keeps them, in `--weights <dir>` if given, else
//! `KUNA_BSIM_WEIGHTS`, else beside the corpus. A run without them reports the
//! missing file rather than scoring against an invented scale: similarity numbers
//! are only meaningful against the table they were defined with.
//!
//! # What a match says
//!
//! Similarity measures **shape**, not identity. A 1.0 match is the same vector;
//! two unrelated functions that compile alike also score high. Both similarity and
//! significance are reported so the evidence travels with the answer.

mod corpus_io;

use crate::args::take_value;
use crate::paths;

/// Dispatch `kuna bsim <subcommand>`.
pub fn run(argv: &[String]) -> i32 {
    let Some(sub) = argv.first() else {
        eprintln!("kuna bsim: missing subcommand (try `kuna bsim build ...`)");
        usage();
        return 2;
    };
    match sub.as_str() {
        "build" => corpus_io::cmd_build(&argv[1..]),
        "query" => corpus_io::cmd_query(&argv[1..]),
        "info" => corpus_io::cmd_info(&argv[1..]),
        "-h" | "--help" | "help" => {
            usage();
            0
        }
        other => {
            eprintln!("kuna bsim: unknown subcommand {other:?}");
            usage();
            2
        }
    }
}

fn usage() {
    eprintln!(
        "usage: kuna bsim build <binary> -o <corpus.kbsm> [--name N] [--max-functions N]\n\
         \x20      kuna bsim query <binary> --corpus <corpus.kbsm> [--address A | --name N]\n\
         \x20                       [--min-similarity S] [--max-matches N] [--json]\n\
         \x20      kuna bsim info <corpus.kbsm> [--json]\n\
         \n\
         Build a function-similarity corpus from one binary and look functions up\n\
         in it. A match is a similarity measure over compiled shape, not proof of\n\
         the same source: every result carries both similarity and significance.\n\
         --min-similarity defaults to 0.7 and is applied before --max-matches\n\
         (default 10), so a strong match is never crowded out by a weak one.\n\
         Scoring needs Ghidra's weight tables (lshweights_*.xml), resolved from\n\
         --weights <dir>, else KUNA_BSIM_WEIGHTS, else beside the corpus.\n\
         query --json emits {{binary, corpus, function, features, min_similarity,\n\
         considered, skipped_settings, truncated, matches, reason}}; info --json\n\
         emits {{corpus, settings, language, executables, functions}}."
    );
}

/// Where the weight tables live, and the failure a run reports without them.
///
/// The tables define what a similarity number means, so a run without them is an
/// error rather than a run against a different scale.
pub(crate) fn resolve_weights(explicit: Option<&str>, corpus: Option<&str>) -> Result<String, String> {
    if let Some(dir) = explicit {
        return Ok(dir.to_string());
    }
    if let Ok(dir) = std::env::var("KUNA_BSIM_WEIGHTS") {
        if !dir.trim().is_empty() {
            return Ok(dir);
        }
    }
    if let Some(path) = corpus {
        if let Some(dir) = std::path::Path::new(path).parent() {
            if dir.join("lshweights_64.xml").exists() {
                return Ok(dir.to_string_lossy().into_owned());
            }
        }
    }
    Err("no weight tables: pass --weights <dir> or set KUNA_BSIM_WEIGHTS (the directory \
         holding Ghidra's lshweights_*.xml)"
        .into())
}

/// The SLEIGH spec roots (an explicit `SLEIGHHOME` then the repo `specs/`),
/// matching `kuna fid`'s resolution.
pub(crate) fn spec_roots() -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() {
            roots.push(home);
        }
    }
    let specs = paths::specs_dir().to_string_lossy().into_owned();
    if !roots.contains(&specs) {
        roots.push(specs);
    }
    roots
}

/// Read a `--flag value` integer, with a named error.
pub(crate) fn parse_usize(args: &[String], i: &mut usize, flag: &str) -> Result<usize, String> {
    *i += 1;
    let raw = args.get(*i).ok_or_else(|| format!("{flag} needs a value"))?;
    raw.parse().map_err(|_| format!("{flag} needs a number, got {raw:?}"))
}

/// Read a `--flag value` float, with a named error.
pub(crate) fn parse_f64(args: &[String], i: &mut usize, flag: &str) -> Result<f64, String> {
    *i += 1;
    let raw = args.get(*i).ok_or_else(|| format!("{flag} needs a value"))?;
    raw.parse().map_err(|_| format!("{flag} needs a number, got {raw:?}"))
}

/// Parse a hex address (`0x` optional).
pub(crate) fn parse_address(value: &str) -> Result<u64, String> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u64::from_str_radix(digits, 16).map_err(|_| format!("invalid address {value:?}"))
}
