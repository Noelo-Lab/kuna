//! Subcommand help and verification guidance without loading an image.
//! Exercise the real CLI's dispatch, statuses and advertised flags.

use std::process::{Command, Output};

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

fn subcommands() -> Vec<String> {
    let out = run(&["--help"]);
    assert_eq!(out.status.code(), Some(0), "{}", help_text(&out));
    assert!(out.stdout.is_empty(), "{}", help_text(&out));
    let text = String::from_utf8(out.stderr).expect("UTF-8 help");
    let names: Vec<String> = text.lines().next().expect("usage line")
        .strip_prefix("usage: kuna <").expect("command list prefix")
        .strip_suffix("> ...").expect("command list suffix")
        .split('|').map(str::to_owned).collect();
    for (index, name) in names.iter().enumerate() {
        assert!(!name.is_empty() && !names[..index].contains(name), "invalid command list: {names:?}");
    }
    names
}

/// The need: exit 0 and a usage block, for both spellings, on every subcommand.
#[test]
fn every_subcommand_answers_help() {
    for sub in subcommands() {
        for flag in ["--help", "-h"] {
            let out = run(&[&sub, flag]);
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
    for sub in subcommands() {
        // `read` shares `disassemble`'s parser and prints the shared block.
        let expect = if sub == "read" { "usage: kuna disassemble|read" } else { &format!("usage: kuna {sub}") };
        let text = help_text(&run(&[&sub, "--help"]));
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

/// Every `--json` names its root in `--help` (issue #1039): `catalog` and
/// `docs` emit arrays and everything else an object, and a consumer that assumed
/// one shape broke on the other. Where the document needs no input binary, the
/// stated root is checked against the one emitted.
#[test]
fn every_json_surface_names_its_root_in_help() {
    let stated: &[(&str, &str)] = &[
        ("decompile", "{binary,"),
        ("decompile-all", "{binary,"),
        ("functions", "{binary,"),
        ("disassemble", "{binary,"),
        ("read", "{binary,"),
        ("xrefs", "{binary,"),
        ("strings", "{binary,"),
        ("crypto", "{binary,"),
        ("unpack", "{binary,"),
        ("test", "{unit_footer,"),
        ("modes", "{modes:["),
        ("docs", "[{topic,"),
        ("catalog", "[{option,"),
    ];
    for sub in subcommands() {
        let text = help_text(&run(&[&sub, "--help"]));
        if !text.contains("--json") {
            continue;
        }
        let root = stated
            .iter()
            .find(|(name, _)| *name == sub)
            .unwrap_or_else(|| panic!("`kuna {sub}` takes --json but this table has no root for it"))
            .1;
        assert!(text.contains(root), "`kuna {sub} --help` never states its --json root {root}:\n{text}");
    }
    for argv in [
        &["catalog", "--json"][..],
        &["catalog", "--json", "--option", "jumptablemax"],
        &["docs", "--json"],
        &["modes", "--json"],
    ] {
        let out = run(argv);
        assert_eq!(out.status.code(), Some(0), "{argv:?}: {}", help_text(&out));
        let stdout = String::from_utf8_lossy(&out.stdout);
        let emitted = stdout.trim_start().chars().next();
        let root = stated.iter().find(|(name, _)| *name == argv[0]).unwrap().1;
        assert_eq!(emitted, root.chars().next(), "{argv:?} emits a different root than its help states");
    }
}

/// `--tier` keeps one tier in `--json` as it does in the table: it used to reach
/// the table only, so `catalog --json --tier core` answered every row.
#[test]
fn catalog_tier_filters_the_json_as_it_filters_the_table() {
    let rows = |argv: &[&str]| -> Vec<serde_json::Value> {
        let out = run(argv);
        assert_eq!(out.status.code(), Some(0), "{argv:?}: {}", help_text(&out));
        serde_json::from_slice(&out.stdout).expect("a JSON array")
    };
    let all = rows(&["catalog", "--json"]);
    let mut kept = 0;
    for tier in ["transform", "analysis", "core"] {
        let some = rows(&["catalog", "--json", "--tier", tier]);
        assert!(!some.is_empty() && some.iter().all(|r| r["tier"] == tier), "{tier}: {some:?}");
        kept += some.len();
    }
    assert_eq!(kept, all.len(), "the three tiers partition the catalog");
    assert!(rows(&["catalog", "--json", "--option", "jumptablemax", "--tier", "core"]).is_empty());
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

#[test]
fn top_level_aliases_and_usage_errors_keep_their_statuses_and_streams() {
    let help = run(&["--help"]);
    assert_eq!(help.status.code(), Some(0));
    assert!(help.stdout.is_empty());
    for alias in ["-h", "help"] {
        let out = run(&[alias]);
        assert_eq!(out.status.code(), Some(0));
        assert_eq!(out.stdout, help.stdout);
        assert_eq!(out.stderr, help.stderr);
    }
    let empty = run(&[]);
    assert_eq!(empty.status.code(), Some(2));
    assert_eq!(empty.stdout, help.stdout);
    assert_eq!(empty.stderr, help.stderr);
    let unknown = "unknown\ncommand";
    let out = run(&[unknown]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(out.stderr, format!("kuna: unknown subcommand {unknown:?}\n{}",
        String::from_utf8(help.stderr).unwrap()).into_bytes());

    let version = run(&["--version"]);
    assert_eq!(version.status.code(), Some(0));
    assert!(version.stdout.starts_with(b"kuna "));
    assert!(version.stderr.is_empty());
    for alias in ["-V", "version"] {
        let out = run(&[alias]);
        assert_eq!(out.status.code(), Some(0));
        assert_eq!(out.stdout, version.stdout);
        assert_eq!(out.stderr, version.stderr);
    }
}

#[test]
fn specs_diff_explains_independent_gates_without_running_a_compiler() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["specs", "--diff"])
        .env("KUNA_SLACOMP", concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .output()
        .expect("spawn kuna");
    assert_eq!(out.status.code(), Some(0), "{}", help_text(&out));
    assert!(out.stderr.is_empty(), "{}", help_text(&out));
    let text = String::from_utf8(out.stdout).expect("UTF-8 guidance");
    for expected in [
        "informational",
        "pinned Ghidra",
        "--test compiler_parity",
        "make test",
        "separate",
        "docs/history.md",
        "decompiler/crates/kuna-slacomp/tests/golden/README.md",
    ] {
        assert!(text.contains(expected), "missing {expected:?}: {text}");
    }
    assert!(!text.contains("no in-tree oracle"), "{text}");
    assert!(!text.contains("subsumed"), "{text}");
    assert!(!text.contains("docs/rust-port/"), "{text}");
}
