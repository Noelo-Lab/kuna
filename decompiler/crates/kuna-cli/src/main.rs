//! The user-facing `kuna` CLI — one Rust binary that reimplements the four
//! former Python entry points (`decompile`, `run_tests`, `catalog`, `slacomp`,
//! which lived in the old `kuna/` Python package and were removed after the port):
//!
//! ```text
//!   kuna decompile <binary> <func> [--addr] [--experimental-formats] [--option NAME VALUE]... [--kassert ARGS]...
//!   kuna test [--all|--unittests|--datatests] [--name N]... [--baseline F]
//!             [--save-baseline F] [--json] [--binary P] [--sleighpath D]
//!   kuna catalog [--json|--markdown|--check] [--option NAME]
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
mod jsonfmt;
mod paths;
mod specs;
mod test;

use std::process::ExitCode;

use decompile::DecompileArgs;
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
        "decompile" => cmd_decompile(rest),
        "test" => cmd_test(rest),
        "catalog" => cmd_catalog(rest),
        "specs" => specs::run(rest),
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
        "usage: kuna <decompile|test|catalog|specs> ...\n\
         \n\
         kuna decompile <binary> <func> [--addr] [--experimental-formats] [--slice ARCH] [--option NAME VALUE]... [--kassert ARGS]...\n\
         kuna test [--all|--unittests|--datatests] [--name N]... [--baseline F] [--save-baseline F] [--json]\n\
         kuna catalog [--json|--markdown|--check] [--option NAME]\n\
         kuna specs [-a <dir>] [<slaspec>...] [--diff]"
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

// --- decompile ---------------------------------------------------------------

fn cmd_decompile(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut target: Option<String> = None;
    let mut addr = false;
    let mut bfd_target: Option<String> = None;
    let mut raw = false;
    let mut regions = false;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut kasserts: Vec<String> = Vec::new();
    let mut decomp_dbg: Option<String> = None;
    let mut engine: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut experimental_formats = false;
    let mut slice: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--addr" => addr = true,
            "--raw" => raw = true,
            "--regions" => regions = true,
            "--experimental-formats" => experimental_formats = true,
            "--slice" => slice = take_value(argv, &mut i, "--slice"),
            "--target" => {
                bfd_target = take_value(argv, &mut i, "--target");
            }
            "--option" => {
                // nargs=2
                if i + 2 >= argv.len() {
                    eprintln!("error: --option requires NAME VALUE");
                    return 2;
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--kassert" => {
                if let Some(v) = take_value(argv, &mut i, "--kassert") {
                    kasserts.push(v);
                }
            }
            "--decomp-dbg" => decomp_dbg = take_value(argv, &mut i, "--decomp-dbg"),
            "--engine" => engine = take_value(argv, &mut i, "--engine"),
            "--sleighpath" => sleighpath = take_value(argv, &mut i, "--sleighpath"),
            "--timeout" => {
                // Accepted for compatibility; the in-process child has no timeout
                // wall (the Python timeout guarded a hung subprocess — out of scope
                // here, but we must consume the value so it isn't read as a positional).
                let _ = take_value(argv, &mut i, "--timeout");
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                return 2;
            }
            _ => {
                if binary.is_none() {
                    binary = Some(a.to_string());
                } else if target.is_none() {
                    target = Some(a.to_string());
                } else {
                    eprintln!("error: unexpected argument {a:?}");
                    return 2;
                }
            }
        }
        i += 1;
    }

    let (binary, target) = match (binary, target) {
        (Some(b), Some(t)) => (b, t),
        _ => {
            eprintln!("error: decompile requires <binary> and <func>");
            return 2;
        }
    };
    apply_engine(engine.as_deref());

    let dargs = DecompileArgs {
        binary,
        target,
        by_address: addr,
        bfd_target,
        raw,
        regions,
        options,
        kasserts,
        decomp_dbg,
        sleighpath,
        experimental_formats,
        slice,
    };
    decompile::run(&dargs)
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

// --- catalog -----------------------------------------------------------------

fn cmd_catalog(argv: &[String]) -> i32 {
    let mut option: Option<String> = None;
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
        catalog::cmd_text(option.as_deref())
    }
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
