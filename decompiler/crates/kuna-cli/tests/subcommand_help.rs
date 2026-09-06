//! `kuna <subcommand> --help` — the discovery surface, over every subcommand at
//! once.
//!
//! Asking a command to describe itself is an agent's first reflex, and four of
//! the sixteen answered it with `error: unknown option --help` and exit 2
//! (`docs/re-needs/decompile-rejects-subcommand-help.md`). `decompile` is the
//! one the need was filed against; `test`, `catalog` and `specs` sat one command
//! away with the same defect, so the test is written over the whole dispatch
//! table rather than over the reported case — a seventeenth subcommand that
//! forgets its help arm fails here the day it lands.
//!
//! No `.sla` and no fixture: help is answered before anything is loaded, which
//! is itself part of the contract ([`help_needs_no_binary_and_no_specs`]).

use std::process::{Command, Output};

/// Every subcommand `main.rs` dispatches, in its dispatch order. `read` shares
/// `disassemble`'s parser and prints its block, which is why both are listed.
const SUBCOMMANDS: &[&str] = &[
    "decompile",
    "decompile-all",
    "decompile-project",
    "decompile-graph",
    "functions",
    "disassemble",
    "read",
    "xrefs",
    "strings",
    "unpack",
    "docs",
    "test",
    "catalog",
    "modes",
    "specs",
    "fid",
];

fn run(argv: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(argv)
        .output()
        .expect("failed to spawn the kuna binary")
}

fn help_text(out: &Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

/// The need: exit 0 and a usage block, for both spellings, on all sixteen.
#[test]
fn every_subcommand_answers_help() {
    for sub in SUBCOMMANDS {
        for flag in ["--help", "-h"] {
            let out = run(&[sub, flag]);
            let text = help_text(&out);
            assert_eq!(
                out.status.code(),
                Some(0),
                "`kuna {sub} {flag}` exited {:?}:\n{text}",
                out.status.code()
            );
            assert!(
                text.contains("usage: kuna "),
                "`kuna {sub} {flag}` printed no usage block:\n{text}"
            );
            assert!(
                !text.contains("unknown option") && !text.contains("unexpected argument"),
                "`kuna {sub} {flag}` treated the help flag as an argument:\n{text}"
            );
        }
    }
}

/// The block a subcommand prints is its own, not the top-level one: an agent
/// that asks about `strings` must not be handed the whole dispatch table.
#[test]
fn each_block_names_its_own_subcommand() {
    for sub in SUBCOMMANDS {
        // `read` shares `disassemble`'s parser and prints the shared block.
        let expect = if *sub == "read" { "usage: kuna disassemble|read" } else { &format!("usage: kuna {sub}") };
        let text = help_text(&run(&[sub, "--help"]));
        assert!(text.starts_with(expect), "`kuna {sub} --help` opened with:\n{text}");
    }
}

/// Help is answered by the argument parser, so it needs neither an input binary
/// nor a compiled `.sla` — the state an agent meeting the tool is actually in.
#[test]
fn help_needs_no_binary_and_no_specs() {
    for sub in ["decompile", "test", "catalog", "specs"] {
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([sub, "--help"])
            .env_remove("KUNA_SPECS")
            .env_remove("SLEIGHHOME")
            .env_remove("KUNA_DECOMP_DBG")
            .output()
            .expect("failed to spawn the kuna binary");
        assert_eq!(out.status.code(), Some(0), "`kuna {sub} --help` needed an environment");
    }
}

/// The four blocks this need added carry the flags the tester went looking for.
/// A usage line that omits the interesting half is the defect one step removed.
#[test]
fn the_repaired_blocks_document_their_flags() {
    let expected: &[(&str, &[&str])] = &[
        ("decompile", &["--addr", "--json", "--option", "--kassert", "--define-function", "--assert", "--mode", "--language"]),
        ("test", &["--all", "--unittests", "--datatests", "--baseline", "--save-baseline", "--name"]),
        ("catalog", &["--json", "--markdown", "--check", "--option", "--tier"]),
        ("specs", &["-a", "--diff"]),
    ];
    for (sub, flags) in expected {
        let text = help_text(&run(&[sub, "--help"]));
        for flag in *flags {
            assert!(text.contains(flag), "`kuna {sub} --help` never mentions {flag}:\n{text}");
        }
    }
}

/// Adding the help arm must not have swallowed a real usage error: an unknown
/// option is still exit 2, and `--help` is not a way to smuggle one past.
#[test]
fn an_unknown_option_is_still_a_usage_error() {
    for sub in ["decompile", "test", "catalog"] {
        let out = run(&[sub, "--no-such-flag"]);
        assert_eq!(out.status.code(), Some(2), "`kuna {sub} --no-such-flag` should be exit 2");
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("unknown option"),
            "`kuna {sub} --no-such-flag` lost its diagnostic"
        );
    }
}
