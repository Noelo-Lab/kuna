//! `--option NAME` name validation, over every subcommand that accepts one.
//!
//! A misspelled option name used to be accepted by `kuna decompile` with exit 0,
//! an empty stderr and output byte-identical to the run with no `--option` at
//! all (`docs/re-needs/unknown-option-name-silently.md`), so
//! "flipping X did not change anything" was not evidence that X is innocent.
//! The eight in-process surfaces did reject it, but only after loading the
//! binary; the check now happens in the parser, which is what lets these tests
//! run with no `.sla` and no fixture.

use std::process::{Command, Output};

/// One argv per subcommand that takes `--option`, with `--option` first so the
/// name is judged before any positional is missed. `read` shares
/// `disassemble`'s parser, and `decompile-project`/`decompile-graph`/`functions`
/// share `decompile-all`'s, but a template each keeps a future split honest.
const SURFACES: &[&[&str]] = &[
    &["decompile", "/nonexistent/kuna-binary", "main"],
    &["decompile-all", "/nonexistent/kuna-binary"],
    &["decompile-project", "/nonexistent/kuna-binary"],
    &["decompile-graph", "/nonexistent/kuna-binary"],
    &["functions", "/nonexistent/kuna-binary"],
    &["disassemble", "/nonexistent/kuna-binary", "main"],
    &["read", "/nonexistent/kuna-binary", "main"],
    &["xrefs", "/nonexistent/kuna-binary", "--to", "0x0"],
    &["strings", "/nonexistent/kuna-binary"],
];

fn run(argv: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(argv)
        .output()
        .expect("failed to spawn the kuna binary")
}

/// `<subcommand> --option <name> off <rest..>`.
fn with_option(surface: &[&str], name: &str) -> Vec<String> {
    let mut argv: Vec<String> = vec![surface[0].into()];
    argv.extend(["--option".to_string(), name.to_string(), "off".to_string()]);
    argv.extend(surface[1..].iter().map(|s| s.to_string()));
    argv
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// The need: an unrecognized name is a hard error on every surface, and the
/// message names the rejected spelling.
#[test]
fn an_unrecognized_option_name_is_rejected_everywhere() {
    for surface in SURFACES {
        let out = run(&with_option(surface, "zzzznotanoption"));
        let err = stderr_of(&out);
        assert_eq!(
            out.status.code(),
            Some(2),
            "kuna {} accepted a misspelled option\nstderr: {err}",
            surface[0]
        );
        assert!(
            err.contains("zzzznotanoption") && err.contains("Unknown option"),
            "kuna {} must name the rejected spelling, got: {err}",
            surface[0]
        );
        assert!(
            out.stdout.is_empty(),
            "kuna {} produced output for a rejected run",
            surface[0]
        );
    }
}

/// The three near-misses the need reported (`LOWEREDSWITCH`, `lowered_switch`,
/// `loweredswitc`) each name the option the caller meant.
#[test]
fn a_near_miss_suggests_the_catalogued_spelling() {
    for miss in ["LOWEREDSWITCH", "lowered_switch", "loweredswitc"] {
        let err = stderr_of(&run(&with_option(SURFACES[0], miss)));
        assert!(
            err.contains("did you mean \"loweredswitch\"?"),
            "{miss} should suggest loweredswitch, got: {err}"
        );
    }
}

/// The check is a *name* check: a catalogued name gets past the parser on every
/// surface and the run fails on the missing binary instead, not on the option.
#[test]
fn a_catalogued_name_still_gets_past_the_parser() {
    for surface in SURFACES {
        for name in ["loweredswitch", "listing", "readonly", "setlanguage", "relocobjects"] {
            let out = run(&with_option(surface, name));
            let err = stderr_of(&out);
            assert!(
                !err.contains("Unknown option"),
                "kuna {} rejected the catalogued name {name}: {err}",
                surface[0]
            );
        }
    }
}

/// The validation happens before anything is loaded — no specs, no fixture, and
/// no `decomp_dbg` spawn — so a bad name costs one parse rather than one load.
#[test]
fn a_bad_name_is_answered_without_specs() {
    let mut argv = with_option(SURFACES[0], "zzzznotanoption");
    argv.push("--sleighpath".into());
    argv.push("/nonexistent/specs".into());
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(&argv)
        .env_remove("SLEIGHHOME")
        .env_remove("KUNA_SPECS")
        .output()
        .expect("failed to spawn the kuna binary");
    assert_eq!(out.status.code(), Some(2), "{}", stderr_of(&out));
    assert!(stderr_of(&out).contains("zzzznotanoption"), "{}", stderr_of(&out));
}
