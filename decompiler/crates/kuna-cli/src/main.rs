//! The user-facing `kuna` CLI — one Rust binary that reimplements the four
//! former Python entry points (`decompile`, `run_tests`, `catalog`, `slacomp`,
//! which lived in the old `kuna/` Python package and were removed after the port):
//!
//! ```text
//!   kuna decompile <binary> <func> [--addr] [--option NAME VALUE]... [--kassert ARGS]...
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
mod decompile_all;
mod decompile_project;
mod fid;
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
        "decompile-all" => decompile_all::run(rest),
        "decompile-project" => decompile_project::run(rest),
        "functions" => decompile_all::run_functions(rest),
        "test" => cmd_test(rest),
        "catalog" => cmd_catalog(rest),
        "modes" => cmd_modes(rest),
        "specs" => specs::run(rest),
        "fid" => fid::run(rest),
        "-V" | "--version" | "version" => {
            // Release CI bakes the repo-derived MAJOR.MINOR (docs/release.md)
            // via KUNA_VERSION; dev builds report the workspace Cargo version.
            println!("kuna {}", option_env!("KUNA_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")));
            0
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
        "usage: kuna <decompile|decompile-all|decompile-project|functions|test|catalog|modes|specs|fid> ...\n\
         \n\
         kuna decompile <binary> <func> [--addr] [--slice ARCH] [--mode reliable|aggressive|fast] [--option NAME VALUE]... [--kassert ARGS]...\n\
         kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA]... [--no-vars] [--max-fn-seconds N] [--mode reliable|aggressive|fast] [--option N V]...\n\
         kuna decompile-project <binary> [-o DIR] [--functions a,b,..] [--addr 0xVMA]... [--max-fn-seconds N] [--mode reliable|aggressive|fast] [--option N V]...\n\
         kuna functions <binary> [--json] [--mode reliable|aggressive|fast]\n\
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

// --- decompile ---------------------------------------------------------------

fn cmd_decompile(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut target: Option<String> = None;
    let mut addr = false;
    let mut bfd_target: Option<String> = None;
    let mut raw = false;
    let mut regions = false;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut mode: Option<String> = None;
    let mut kasserts: Vec<String> = Vec::new();
    let mut decomp_dbg: Option<String> = None;
    let mut engine: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut slice: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--addr" => addr = true,
            "--raw" => raw = true,
            "--regions" => regions = true,
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
            "--mode" => mode = take_value(argv, &mut i, "--mode"),
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

    // Expand a selected `--mode` into its option overrides, PREPENDED so an
    // explicit `--option` still wins (later `option NAME VALUE` console lines
    // overwrite in `build_script`; the load-time env gates + `listing`
    // auto-inject already key off the merged `options` list).
    if let Some(m) = mode.as_deref() {
        match decompile_all::mode_override_pairs(m) {
            Ok(mut pairs) => {
                pairs.extend(options);
                options = pairs;
            }
            Err(e) => {
                eprintln!("error: {e}");
                return 2;
            }
        }
    }

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

// --- modes -------------------------------------------------------------------

/// `kuna modes [--json]` — list the decompiler mode presets (`reliable`,
/// `aggressive`, `fast`) and the `(option → value)` overrides each applies. Modes are
/// *not* settable catalog rows (`kuna catalog` covers those); they are presets
/// over the option surface, applied via `--mode` / the console `mode` command.
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
                    ("overrides".into(), Json::Array(overrides)),
                ])
            })
            .collect();
        let root = Json::Object(vec![("modes".into(), Json::Array(modes))]);
        println!("{}", jsonfmt::dumps_indent2(&root));
    } else {
        for m in kuna_decomp::modes::MODE_TABLE {
            println!("{}", m.name);
            println!("  {}", m.summary);
            if m.overrides.is_empty() {
                println!("  (no overrides — the shipped defaults)");
            } else {
                let joined: Vec<String> =
                    m.overrides.iter().map(|(o, v)| format!("{o}={v}")).collect();
                println!("  overrides: {}", joined.join(", "));
            }
        }
    }
    0
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
