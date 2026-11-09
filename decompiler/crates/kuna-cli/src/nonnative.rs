//! Inputs the native engine does not decompile, routed to the engine that does.
//!
//! [`dispatch`] runs on every file-taking subcommand before any native work:
//! it finds the input operand, sniffs it (one `stat`, one 8-byte read) and
//! hands a JVM input (a class file, a jar, a directory) to the JVM engine.
//! Anything else returns `None` and takes the native path unchanged, as does
//! every invocation with `--raw-image` or `--jobs-worker`.

use std::path::Path;

mod jvm;

pub(crate) use jvm::catalog;
pub(crate) use jvm::corpus;

/// The subcommands that take an input file.
const INPUT_SUBCOMMANDS: &[&str] = &[
    "decompile",
    "decompile-all",
    "decompile-project",
    "decompile-graph",
    "functions",
    "disassemble",
    "raw-pcode",
    "read",
    "xrefs",
    "strings",
    "crypto",
    "unpack",
    "fid",
];

/// Every flag of any file-taking subcommand that consumes the next argument.
const VALUE_FLAGS: &[&str] = &[
    "-o",
    "--output",
    "--label",
    "--to",
    "--from",
    "--addr",
    "--entry",
    "--count",
    "--bytes",
    "--lang",
    "--cspec",
    "--lib",
    "--base",
    "--slice",
    "--isa",
    "--target",
    "--language",
    "--mode",
    "--kassert",
    "--define-function",
    "--assert",
    "--decomp-dbg",
    "--max-fn-memory",
    "--max-fn-seconds",
    "--engine",
    "--sleighpath",
    "--timeout",
    "--omit",
    "--functions",
    "--jobs",
    "--jobs-chunk",
    "--jobs-worker",
    "--jobs-synth",
    "--filter",
    "--min-size",
    "--max-size",
    "--reachable-from",
    "--limit",
    "--sort",
    "--kind",
    "--min-length",
    "--encoding",
    "--section",
    "--algorithm",
    "--as",
];

/// The JVM engine's exit status for `kuna <sub> <rest..>`, or `None` when the
/// input is not a JVM input.
pub(crate) fn dispatch(sub: &str, rest: &[String]) -> Option<i32> {
    if !INPUT_SUBCOMMANDS.contains(&sub)
        || rest.iter().any(|a| a == "--raw-image" || a == "--jobs-worker")
    {
        return None;
    }
    let input = input_operand(sub, rest)?;
    if !kuna_jvm::kuna_input::sniff(Path::new(input)) {
        return None;
    }
    Some(jvm::run(sub, rest, input))
}

/// The first argument that is neither a flag nor a flag's value; `fid`'s
/// `build` verb is skipped.
pub(crate) fn input_operand<'a>(sub: &str, rest: &'a [String]) -> Option<&'a str> {
    let mut verb_pending = sub == "fid";
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a == "--option" {
            i += 3;
        } else if VALUE_FLAGS.contains(&a) {
            i += 2;
        } else if a.starts_with('-') && a.len() > 1 {
            i += 1;
        } else if verb_pending {
            verb_pending = false;
            i += 1;
        } else {
            return Some(a);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::input_operand;

    fn argv(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_operand_skips_flags_and_their_values() {
        let rest = argv(&["--option", "jvm_x", "on", "--mode", "fast", "-o", "out", "app.jar", "Main"]);
        assert_eq!(input_operand("decompile", &rest), Some("app.jar"));
        let rest = argv(&["--json", "app.jar"]);
        assert_eq!(input_operand("functions", &rest), Some("app.jar"));
        let rest = argv(&["build", "lib.a", "-o", "x.fid"]);
        assert_eq!(input_operand("fid", &rest), Some("lib.a"));
        assert_eq!(input_operand("decompile", &argv(&["--json"])), None);
    }
}
