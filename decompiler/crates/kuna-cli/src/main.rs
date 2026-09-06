//! The user-facing `kuna` CLI — one Rust binary that reimplements the four
//! former Python entry points (`decompile`, `run_tests`, `catalog`, `slacomp`,
//! which lived in the old `kuna/` Python package and were removed after the port):
//!
//! ```text
//!   kuna decompile <binary> <func> [--addr] [--option NAME VALUE]... [--kassert ARGS]...
//!                                 [--define-function <start[-end][=name] | @file>]...
//!   kuna test [--all|--unittests|--datatests] [--name N]... [--baseline F]
//!             [--save-baseline F] [--json] [--binary P] [--sleighpath D]
//!   kuna catalog [--json|--markdown|--check] [--option NAME] [--tier T]
//!   kuna specs [-a <dir>] [<slaspec>...] [--diff]
//! ```
//!
//! Most subcommands shell out to the already-built engine binaries (the same
//! console command surface the Python drove), so their output is byte-identical;
//! `catalog --check` runs in-process against `kuna-decomp`.  Argument parsing is
//! hand-rolled (matching the workspace convention of avoiding a new dep) and
//! mirrors each module's argparse contract.

mod catalog;
mod decompile;
mod docs;
mod decompile_all;
mod decompile_project;
mod decompile_graph;
mod disassemble;
mod fid;
mod assertdecl;
mod funcdecl;
mod jsonfmt;
mod litpool;
mod optname;
mod output;
mod paths;
mod specs;
mod strings;
mod test;
mod unpack;
mod xrefs;

use std::process::ExitCode;

use test::{Mode, TestArgs};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        return ExitCode::from(2);
    }
    let sub = args[1].as_str();
    let rest = &args[2..];
    let code = match sub {
        "decompile" => decompile::main(rest),
        "decompile-all" => decompile_all::run(rest),
        "decompile-project" => decompile_project::run(rest),
        "decompile-graph" => decompile_graph::run(rest),
        "functions" => decompile_all::run_functions(rest),
        "test" => cmd_test(rest),
        "catalog" => cmd_catalog(rest),
        "docs" => docs::run(rest),
        "modes" => cmd_modes(rest),
        "specs" => specs::run(rest),
        "fid" => fid::run(rest),
        "unpack" => unpack::run(rest),
        "strings" => strings::run(rest),
        "disassemble" => disassemble::run(rest),
        "read" => disassemble::run_read(rest),
        "xrefs" => xrefs::run(rest),
        "-V" | "--version" | "version" => {
            // Release CI bakes the repo-derived MAJOR.MINOR (docs/release.md)
            // via KUNA_VERSION; dev builds report the workspace Cargo version.
            output::emit_with_status(
                &format!(
                    "kuna {}\n",
                    option_env!("KUNA_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
                ),
                0,
            )
        }
        "-h" | "--help" | "help" => {
            usage();
            0
        }
        other => {
            eprintln!("kuna: unknown subcommand {other:?}");
            usage();
            2
        }
    };
    ExitCode::from(code as u8)
}

fn usage() {
    eprintln!(
        "usage: kuna <decompile|decompile-all|decompile-project|decompile-graph|functions|disassemble|read|xrefs|strings|unpack|docs|test|catalog|modes|specs|fid> ...\n\
         \n\
         kuna decompile <binary> <func> [--addr] [--json] [--slice ARCH] [--language auto|c|rust] [--mode auto|reliable|aggressive|fast] [--option NAME VALUE]... [--kassert ARGS]... [--define-function S[-E][=N]|@FILE]... [--assert DIRECTIVE|@FILE]...\n\
         kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA]... [--no-vars] [--language auto|c|rust] [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] [--option N V]... [--define-function S[-E][=N]|@FILE]... [--assert DIRECTIVE|@FILE]...\n\
         kuna decompile-project <binary> [-o DIR] [--functions a,b,..] [--addr 0xVMA]... [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] [--option N V]... [--define-function S[-E][=N]|@FILE]... [--assert DIRECTIVE|@FILE]...\n\
         kuna decompile-graph <binary> [-o FILE] [--label TEXT] [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] [--option N V]... [--define-function S[-E][=N]|@FILE]...\n\
         kuna functions <binary> [--json] [--mode auto|reliable|aggressive|fast] [--define-function S[-E][=N]|@FILE]...\n\
         kuna xrefs <binary> (--to <name|0xaddr> | --from <name|0xaddr>) [--json] [--kind call,jump,data,read,write] [--mode auto|reliable|aggressive|fast]\n\
         kuna unpack <binary> [-o OUT] [--json]\n\
         kuna strings <binary> [--json] [--min-length N] [--filter REGEX] [--encoding ascii|utf16|all] [--section NAME] [--no-xrefs]\n\
         kuna disassemble <binary> <name|0xaddr|0xstart-0xend> [--addr] [--as code|data|auto] [--count N] [--bytes N] [--json] [--mode auto|reliable|aggressive|fast] [--option N V]... [--define-function S[-E][=N]|@FILE]... [--slice ARCH] [--target T] [--sleighpath D]\n\
         kuna read <binary> <name|0xaddr|0xstart-0xend> [--addr] [--bytes N] [--count N] [--json]   # the hexdump view of the same target\n\
         kuna docs [<topic>] [--json] [--all]\n\
         kuna test [--all|--unittests|--datatests] [--name N]... [--baseline F] [--save-baseline F] [--json]\n\
         kuna catalog [--json|--markdown|--check] [--option NAME] [--tier transform|analysis|core]\n\
         kuna modes [--json]\n\
         kuna specs [-a <dir>] [<slaspec>...] [--diff]\n\
         kuna fid build <lib.a|*.o ...> -o <out.fid> --lang <id> --cspec <id>\n\
         kuna --version"
    );
}

/// Honor `--engine cpp|rust` like the Python tools: set `KUNA_ENGINE`.  In the
/// Rust-only world `rust` is already the default and `cpp` would fail to resolve,
/// but the flag is accepted (and exported) for compatibility with existing
/// invocations / the pipeline.
fn apply_engine(engine: Option<&str>) {
    if let Some(e) = engine {
        std::env::set_var("KUNA_ENGINE", e);
    }
}

// --- test --------------------------------------------------------------------

fn cmd_test(argv: &[String]) -> i32 {
    let mut mode = Mode::All;
    let mut mode_set = false;
    let mut names: Vec<String> = Vec::new();
    let mut binary: Option<String> = None;
    let mut engine: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut datatests_dir: Option<String> = None;
    let mut baseline: Option<String> = None;
    let mut save_baseline: Option<String> = None;
    let mut json = false;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--all" => {
                mode = Mode::All;
                mode_set = true;
            }
            "--unittests" => {
                mode = Mode::Unittests;
                mode_set = true;
            }
            "--datatests" => {
                mode = Mode::Datatests;
                mode_set = true;
            }
            "--name" => {
                if let Some(v) = take_value(argv, &mut i, "--name") {
                    names.push(v);
                }
            }
            "--binary" => binary = take_value(argv, &mut i, "--binary"),
            "--engine" => engine = take_value(argv, &mut i, "--engine"),
            "--sleighpath" => sleighpath = take_value(argv, &mut i, "--sleighpath"),
            "--datatests-dir" => datatests_dir = take_value(argv, &mut i, "--datatests-dir"),
            "--baseline" => baseline = take_value(argv, &mut i, "--baseline"),
            "--save-baseline" => save_baseline = take_value(argv, &mut i, "--save-baseline"),
            "--json" => json = true,
            "-h" | "--help" => {
                usage_test();
                return 0;
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                return 2;
            }
            other => {
                eprintln!("error: unexpected argument {other:?}");
                return 2;
            }
        }
        i += 1;
    }
    let _ = mode_set;

    if !names.is_empty() && mode == Mode::All {
        eprintln!("error: --name requires --unittests or --datatests");
        return 2;
    }
    apply_engine(engine.as_deref());

    let targs = TestArgs {
        mode,
        names,
        binary,
        sleighpath,
        datatests_dir,
        baseline,
        save_baseline,
        json,
    };
    test::run_cmd(&targs)
}

fn usage_test() {
    eprintln!(
        "usage: kuna test [--all|--unittests|--datatests] [--name N].. [--json] \\\n\
         \x20                [--datatests-dir D] [--baseline F] [--save-baseline F] \\\n\
         \x20                [--binary B] [--sleighpath D]\n\
         \n\
         Run the parity gates.  Unit results are parsed off stderr and datatest\n\
         results off stdout, and the exit code is nonzero on any failure OR any\n\
         regression against --baseline.  Default is --all.\n\
         --name N (repeatable) narrows to named tests and requires --unittests or\n\
         --datatests.\n\
         \n\
         --datatests-dir D selects the corpus: tests/datatests (the default, = make\n\
         test) or tests/stages (the kuna-owned issue cases, = make test-stages).\n\
         --baseline F compares against a recorded result; --save-baseline F re-records\n\
         one.  Re-recording docs/baseline.json to absorb a regression is never\n\
         sanctioned -- see docs/agents.md."
    );
}

// --- catalog -----------------------------------------------------------------

fn cmd_catalog(argv: &[String]) -> i32 {
    let mut option: Option<String> = None;
    let mut tier: Option<String> = None;
    let mut json = false;
    let mut markdown = false;
    let mut check = false;
    let mut engine: Option<String> = None;
    // --decomp-dbg / --sleighpath are accepted (Python had them); they map to the
    // env overrides paths.rs already honors, so just set them.
    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--option" => option = take_value(argv, &mut i, "--option"),
            "--tier" => tier = take_value(argv, &mut i, "--tier"),
            "--json" => json = true,
            "--markdown" => markdown = true,
            "--check" => check = true,
            "--engine" => engine = take_value(argv, &mut i, "--engine"),
            "--decomp-dbg" => {
                if let Some(v) = take_value(argv, &mut i, "--decomp-dbg") {
                    std::env::set_var("KUNA_DECOMP_DBG", v);
                }
            }
            "--sleighpath" => {
                if let Some(v) = take_value(argv, &mut i, "--sleighpath") {
                    std::env::set_var("KUNA_SPECS", v);
                }
            }
            "-h" | "--help" => {
                usage_catalog();
                return 0;
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                return 2;
            }
            other => {
                eprintln!("error: unexpected argument {other:?}");
                return 2;
            }
        }
        i += 1;
    }
    // argparse mutually-exclusive group: at most one of json/markdown/check.
    if (json as u8) + (markdown as u8) + (check as u8) > 1 {
        eprintln!("error: --json, --markdown, --check are mutually exclusive");
        return 2;
    }
    apply_engine(engine.as_deref());

    if check {
        catalog::cmd_check()
    } else if json {
        catalog::cmd_json(option.as_deref())
    } else if markdown {
        catalog::cmd_markdown(option.as_deref())
    } else {
        catalog::cmd_text(option.as_deref(), tier.as_deref())
    }
}

fn usage_catalog() {
    eprintln!(
        "usage: kuna catalog [--json|--markdown|--check] [--option NAME] \\\n\
         \x20                   [--tier transform|analysis|core] [--sleighpath D]\n\
         \n\
         List the settable phase-model assertions -- the decisions `--option NAME VALUE`\n\
         flips on every decompiling surface.  Each row carries its phase, tier, values,\n\
         default, the symptoms it addresses and when to flip it.\n\
         \n\
         --json is the machine-readable form an agent selects options from.\n\
         --markdown regenerates docs/options.md (tier-grouped, symptom-indexed).\n\
         --check cross-checks the catalog against the engine's registered option\n\
         names and exits nonzero on drift; this is the CI gate.\n\
         --option NAME reports one row; --tier keeps one tier.\n\
         \n\
         Upstream OptionDatabase names (maxinstruction, errortoomanyinstructions, ..)\n\
         are reachable through --option but are NOT catalog rows, so they do not\n\
         appear here.  Mode presets are `kuna modes`."
    );
}

// --- modes -------------------------------------------------------------------

/// `kuna modes [--json]` — list the decompiler mode policies and presets.
/// Modes are *not* settable catalog rows (`kuna catalog` covers those);
/// concrete modes are option presets, while `auto` selects one using input
/// file size.
fn cmd_modes(argv: &[String]) -> i32 {
    let mut json = false;
    for a in argv {
        match a.as_str() {
            "--json" => json = true,
            "-h" | "--help" => {
                eprintln!("usage: kuna modes [--json]");
                return 0;
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                return 2;
            }
            other => {
                eprintln!("error: unexpected argument {other:?}");
                return 2;
            }
        }
    }

    use jsonfmt::Json;
    use std::fmt::Write as _;
    let mut text = String::new();
    if json {
        let modes: Vec<Json> = kuna_decomp::modes::MODE_TABLE
            .iter()
            .map(|m| {
                let overrides: Vec<Json> = m
                    .overrides
                    .iter()
                    .map(|(opt, val)| {
                        Json::Object(vec![
                            ("option".into(), Json::Str((*opt).into())),
                            ("value".into(), Json::Str((*val).into())),
                        ])
                    })
                    .collect();
                Json::Object(vec![
                    ("name".into(), Json::Str(m.name.into())),
                    ("summary".into(), Json::Str(m.summary.into())),
                    ("automatic".into(), Json::Bool(m.automatic)),
                    ("overrides".into(), Json::Array(overrides)),
                ])
            })
            .collect();
        let root = Json::Object(vec![("modes".into(), Json::Array(modes))]);
        let _ = writeln!(text, "{}", jsonfmt::dumps_indent2(&root));
    } else {
        for m in kuna_decomp::modes::MODE_TABLE {
            let _ = writeln!(text, "{}", m.name);
            let _ = writeln!(text, "  {}", m.summary);
            if m.automatic {
                let _ = writeln!(text, "  (automatic size policy; no direct overrides)");
            } else if m.overrides.is_empty() {
                let _ = writeln!(text, "  (no overrides — the shipped defaults)");
            } else {
                let joined: Vec<String> =
                    m.overrides.iter().map(|(o, v)| format!("{o}={v}")).collect();
                let _ = writeln!(text, "  overrides: {}", joined.join(", "));
            }
        }
    }
    output::emit_with_status(&text, 0)
}

// --- helpers -----------------------------------------------------------------

/// Consume the value following a flag at `argv[i]`, advancing `i` past it.
fn take_value(argv: &[String], i: &mut usize, flag: &str) -> Option<String> {
    if *i + 1 < argv.len() {
        *i += 1;
        Some(argv[*i].clone())
    } else {
        eprintln!("error: {flag} requires a value");
        None
    }
}
