//! The CLI over the JVM engine (`kuna-jvm`): class files, jars and directories.
//!
//! `decompile`, `decompile-all`, `decompile-project` and `functions` run the
//! engine; every other file-taking subcommand answers with a capability error.
//! Each subcommand has its own parser ([`args`]) that refuses the native-only
//! flags; the selection grammar is [`select`], the driver glue [`run`], the
//! text and JSON shapes [`render`], the source-tree export [`project`].

mod args;
pub(crate) mod catalog;
pub(crate) mod corpus;
mod project;
mod render;
mod run;
mod select;

use std::path::Path;

use kuna_jvm::kuna_input::{self, InputKind};

pub(crate) const DOCS_POINTER: &str = "kuna docs jvm";

/// Runs `kuna <sub> <rest..>` on the JVM input `input`.
pub(crate) fn run(sub: &str, rest: &[String], input: &str) -> i32 {
    if rest.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!("{}", args::usage(sub));
        return 0;
    }
    let kind = match kuna_input::classify(Path::new(input)) {
        Ok(kind) => kind,
        Err(e) => {
            eprintln!("error: {input}: {e}");
            return 1;
        }
    };
    if !kind.is_supported() {
        let error = kuna_input::InputError::Unsupported(input.into(), kind);
        eprintln!("error: {error} ({DOCS_POINTER})");
        return 1;
    }
    let Some(sub) = args::Sub::parse(sub) else {
        eprintln!("error: {}", capability_error(sub, input, kind));
        return 1;
    };
    let parsed = match args::parse(sub, rest, input, kind) {
        Ok(parsed) => parsed,
        Err(args::Refusal::Usage(e)) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    match sub {
        args::Sub::Decompile => render::decompile(&parsed),
        args::Sub::DecompileAll => render::decompile_all(&parsed),
        args::Sub::Project => project::run(&parsed),
        args::Sub::Functions => render::functions(&parsed),
    }
}

/// The input as the error messages name it: `JVM input (jar)`.
pub(crate) fn input_label(kind: InputKind) -> String {
    format!("JVM input ({})", kind.name())
}

fn capability_error(sub: &str, input: &str, kind: InputKind) -> String {
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
    format!(
        "{input}: {} is decompiled by the non-native JVM engine, which {what}; \
         JVM support covers decompile, decompile-all, decompile-project and functions ({DOCS_POINTER})",
        input_label(kind)
    )
}
