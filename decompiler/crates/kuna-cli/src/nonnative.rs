//! Inputs the native engine does not decompile, routed to the managed engine
//! that does.
//!
//! [`dispatch`] runs on every file-taking subcommand before any native work:
//! it finds the input operand and asks each engine of [`ENGINES`] to sniff it
//! (one `stat`, a short read); the first that claims it runs the command.
//! Anything else returns `None` and takes the native path unchanged, as does
//! every invocation with `--raw-image` or `--jobs-worker`. An engine meets the
//! CLI through the contract of `kuna_managed`, and everything here is the same
//! for each: `decompile`, `decompile-all`, `decompile-project` and `functions`
//! run the engine, every other file-taking subcommand answers with a
//! capability error. Each subcommand has its own parser ([`args`]) that
//! refuses the native-only flags; driving a run is [`run`], the text and JSON
//! shapes [`render`], what is reported about failed and hidden code
//! [`failures`], the source-tree export [`project`].

use std::path::Path;

use kuna_managed::{Engine, Kind};

mod args;
pub(crate) mod catalog;
mod failures;
mod heap;
mod project;
mod render;
mod run;
#[cfg(test)]
mod toy;

#[cfg(feature = "jvm")]
#[path = "nonnative/jvm/corpus.rs"]
pub(crate) mod corpus;

/// The managed engines compiled into this kuna.
pub(crate) static ENGINES: &[&dyn Engine] = &[
    #[cfg(feature = "jvm")]
    &kuna_jvm::kuna_engine::Jvm,
];

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
    "--image",
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

/// The exit status of `kuna <sub> <rest..>` run by the first of `engines`
/// that claims the input, or `None` when none does.
pub(crate) fn dispatch(engines: &[&'static dyn Engine], sub: &str, rest: &[String]) -> Option<i32> {
    if !INPUT_SUBCOMMANDS.contains(&sub)
        || rest.iter().any(|a| a == "--raw-image" || a == "--jobs-worker")
    {
        return None;
    }
    let input = input_operand(sub, rest)?;
    let engine = engines.iter().find(|e| e.sniff(Path::new(input)))?;
    Some(run(*engine, sub, rest, input))
}

fn run(engine: &'static dyn Engine, sub: &str, rest: &[String], input: &str) -> i32 {
    if rest.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!("{}", engine.usage(sub));
        return 0;
    }
    let kind = match engine.classify(Path::new(input)) {
        Ok(kind) => kind,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let Some(sub) = args::Sub::parse(sub) else {
        eprintln!("error: {}", capability_error(engine, sub, input, &kind));
        return 1;
    };
    let parsed = match args::parse(engine, sub, rest, input, &kind) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    if sub != args::Sub::Functions {
        heap::tune();
    }
    match sub {
        args::Sub::Decompile => render::decompile(&parsed),
        args::Sub::DecompileAll => render::decompile_all(&parsed),
        args::Sub::Project => project::run(&parsed),
        args::Sub::Functions => render::functions(&parsed),
    }
}

/// The input as the error messages name it: `JVM input (jar)`.
fn input_label(engine: &dyn Engine, kind: &Kind) -> String {
    format!("{} input ({})", engine.label(), kind.name)
}

/// The engine's manual: `kuna docs jvm`.
fn docs(engine: &dyn Engine) -> String {
    format!("kuna docs {}", engine.name())
}

fn capability_error(engine: &dyn Engine, sub: &str, input: &str, kind: &Kind) -> String {
    let what = match sub {
        "strings" => "has no string inventory yet",
        "xrefs" => "has no cross-reference index yet",
        "disassemble" | "read" => "has no bytecode listing yet",
        "decompile-graph" => "has no program graph yet",
        "crypto" => "has no crypto-constant scan yet",
        "raw-pcode" => "does not lift bytecode to p-code",
        "unpack" => "reads archives directly; there is nothing to unpack",
        "fid" => "has no function-ID databases",
        _ => "does not support this subcommand",
    };
    let label = engine.label();
    format!(
        "{input}: {} is decompiled by the non-native {label} engine, which {what}; \
         {label} support covers decompile, decompile-all, decompile-project and functions ({})",
        input_label(engine, kind),
        docs(engine)
    )
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
