//! `kuna bsim` — build a function-similarity corpus and query it.
//!
//! BSim answers a question FID cannot: "which functions in this corpus *look
//! like* this one?". kuna decompiles a function, turns the decompiler's feature
//! vector into a weighted LSH vector, and compares it with a stored corpus.
//!
//! Local queries use Ghidra's XML weight tables. PostgreSQL queries use the
//! connected BSim database's signature settings, weights and LSH index.
//!
//! # What a match says
//!
//! Similarity measures **shape**, not identity. A 1.0 match is the same vector;
//! two unrelated functions that compile alike also score high. Both similarity and
//! significance are reported so the evidence travels with the answer.

mod corpus_io;
mod postgres;
mod push;
mod reference;
mod server_client;

struct TerminalText<'a>(&'a str);

impl std::fmt::Display for TerminalText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut start = 0;
        for (offset, ch) in self.0.char_indices() {
            if ch.is_control() || matches!(ch, '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}') {
                f.write_str(&self.0[start..offset])?;
                for escaped in ch.escape_default() {
                    f.write_char(escaped)?;
                }
                start = offset + ch.len_utf8();
            }
        }
        f.write_str(&self.0[start..])
    }
}

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
        "compare" => reference::cmd_compare(&argv[1..]),
        "push" => push::run(&argv[1..]),
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
         \x20                       [--profile nosize] [--weights DIR]\n\
         \x20      kuna bsim query <binary> (--corpus <corpus.kbsm> [--weights DIR] | --postgres |\n\
         \x20                       --server URL [--profile PROFILE])\n\
         \x20                       (--address A | --name N) [--min-similarity S] [--max-matches N] [--json]\n\
         \x20      kuna bsim compare <binary> (--address A | --name N)\n\
         \x20                       (--reference BIN --reference-address A |\n\
         \x20                        --match-id ID (--reference BIN | --server URL)) [--json]\n\
         \x20      kuna bsim push <binary> --server URL (--address A | --name N)\n\
         \x20                       [--rename NAME] [--profile PROFILE] [--json]\n\
         \x20      kuna bsim info <corpus.kbsm> [--json]\n\
         \n\
         Build a function-similarity corpus from one binary and look functions up\n\
         in it. A match is a similarity measure over compiled shape, not proof of\n\
         the same source: every result carries both similarity and significance.\n\
         --min-similarity defaults to 0.7 and is applied before --max-matches\n\
         (default 10), so a strong match is never crowded out by a weak one.\n\
         Each local corpus pins one signing/scoring profile. Choose --profile\n\
         nosize at creation to mix native inputs with different address sizes.\n\
         Local build and query use bundled weights by default. Overrides, in\n\
         order: --weights, KUNA_BSIM_WEIGHTS, then matching XML beside the corpus.\n\
         A capped or incomplete build can be retried with a higher total cap or no cap.\n\
         PostgreSQL reads KUNA_BSIM_POSTGRES and uses an existing server-side BSim\n\
         index and weights. No local bridge is needed.\n\
         --server uses the native Kuna BSim server and fetches an exact binary\n\
         for compare. A PostgreSQL match ID instead requires a local --reference.\n\
         Push signs and publishes one named entry and its original image. It uses\n\
         KUNA_BSIM_SERVER_WRITE_TOKEN and never writes to the input binary.\n\
         Local query --json emits {{binary, corpus, function, features, min_similarity,\n\
         profile, settings, considered, truncated, matches, reason}}. Remote query\n\
         emits {{binary, backend, function, features, min_similarity, truncated,\n\
         matches, reason}}. Compare emits {{binary, source, match, reference, diff}}.\n\
         Push emits {{match_id, profile, sha256, address, name, outcome}}.\n\
         Info emits {{corpus, settings, profile, executables, functions}}."
    );
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
    raw.parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("{flag} needs a finite number, got {raw:?}"))
}

/// Parse a hex address (`0x` optional).
pub(crate) fn parse_address(value: &str) -> Result<u64, String> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u64::from_str_radix(digits, 16).map_err(|_| format!("invalid address {value:?}"))
}
