//! A `name`/`type` directive renames the local it names and no other.
//!
//! The directive is read against the first decompile and applied by a second,
//! which used to number the locals from scratch: the renamed local took no
//! number, so every later one moved down (`name v6 x` printed the old `v7` as
//! `v6`), and a second directive naming `v6` reached the wrong variable.
//!
//! Fixture: `kuna-analysis/tests/fixtures/stablenames_x86_64` (source beside
//! it), whose `mix` prints thirteen numbered locals, and the static glibc
//! `mcount_x86_64`, whose `handle_intel.constprop.0` keeps many same-width
//! temporaries at one instruction. Every case runs on both surfaces: the text
//! one drives `decomp_dbg` through a console script, `--json` runs in-process.

use std::collections::BTreeSet;

use crate::common;

/// The C of `mix` under `directives`, after checking that every directive
/// applied and both surfaces printed the same C.
fn decompile(directives: &[&str]) -> String {
    decompile_with(&[], directives)
}

/// [`decompile`] with `extra` command-line arguments.
fn decompile_with(extra: &[&str], directives: &[&str]) -> String {
    decompile_in("stablenames_x86_64", "mix", extra, directives)
}

/// [`decompile`] of `function` in `fixture`.
fn decompile_in(fixture: &str, function: &str, extra: &[&str], directives: &[&str]) -> String {
    let binary = common::fixture(fixture);
    let mut args = vec!["decompile", binary.as_str(), function];
    args.extend(extra);
    for directive in directives {
        args.extend(["--assert", directive]);
    }
    let (text, stderr, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{directives:?}: {stderr}");
    assert!(!stderr.contains("rejected"), "{directives:?}: {stderr}");
    args.push("--json");
    let (doc, stderr, _) = common::run_kuna(&args);
    let parsed: serde_json::Value =
        serde_json::from_str(&doc).unwrap_or_else(|e| panic!("{e}: {stderr}\n{doc}"));
    let json = parsed
        .pointer("/functions/0/code")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("no code: {stderr}\n{doc}"));
    assert_eq!(text.trim_end(), json.trim_end(), "{directives:?}: the two surfaces disagree");
    text
}

/// `code` with each `(from, to)` identifier swapped, as a sorted list of
/// lines (declarations are printed in name order, so a rename reorders them).
fn lines_renamed(code: &str, renames: &[(&str, &str)]) -> Vec<String> {
    let mut lines: Vec<String> = code
        .lines()
        .map(|line| {
            regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b")
                .unwrap()
                .replace_all(line, |m: &regex::Captures| {
                    let word = &m[0];
                    renames
                        .iter()
                        .find(|(from, _)| *from == word)
                        .map_or(word, |(_, to)| *to)
                        .to_string()
                })
                .into_owned()
        })
        .collect();
    lines.sort();
    lines
}

/// The names `code` declares locals under.
fn declared(code: &str) -> BTreeSet<String> {
    let decl = regex::Regex::new(r"(?m)^  [^ ].*?([A-Za-z_][A-Za-z0-9_]*)( \[[0-9]+\])?; // ").unwrap();
    decl.captures_iter(code).map(|c| c[1].to_string()).collect()
}

/// Renaming leaves the C exactly as it was apart from the renamed names.
fn renames_only(directives: &[&str], renames: &[(&str, &str)]) {
    renames_only_with(&[], directives, renames);
}

fn renames_only_with(extra: &[&str], directives: &[&str], renames: &[(&str, &str)]) {
    renames_only_in("stablenames_x86_64", "mix", extra, directives, renames);
}

fn renames_only_in(fixture: &str, function: &str, extra: &[&str], directives: &[&str], renames: &[(&str, &str)]) {
    let plain = decompile_in(fixture, function, extra, &[]);
    let renamed = decompile_in(fixture, function, extra, directives);
    assert_eq!(
        lines_renamed(&renamed, renames),
        lines_renamed(&plain, &[]),
        "{directives:?} renamed more than {renames:?}:\n{plain}\n---\n{renamed}"
    );
}

#[test]
fn the_fixture_numbers_thirteen_locals() {
    let names = declared(&decompile(&[]));
    let expected: BTreeSet<String> = (1..=13).map(|i| format!("v{i}")).collect();
    assert_eq!(names, expected);
}

#[test]
fn renaming_a_middle_local_keeps_every_other_number() {
    renames_only(&["name v6 vowels"], &[("vowels", "v6")]);
}

#[test]
fn renaming_the_first_local_keeps_every_other_number() {
    renames_only(&["name v1 c"], &[("c", "v1")]);
}

/// `v13` is two values in one register, and the rename names both; no other
/// local's number moves.
#[test]
fn renaming_the_last_local_keeps_every_other_number() {
    let numbered: BTreeSet<String> =
        declared(&decompile(&["name v13 acc"])).into_iter().filter(|n| !n.starts_with("acc")).collect();
    assert_eq!(numbered, (1..=12).map(|i| format!("v{i}")).collect());
}

#[test]
fn two_renames_keep_every_other_number() {
    renames_only(&["name v6 vowels", "name v11 digits"], &[("vowels", "v6"), ("digits", "v11")]);
}

#[test]
fn renaming_an_argument_keeps_every_local_number() {
    renames_only(&["name a1 n"], &[("n", "a1")]);
}

#[test]
fn renaming_back_to_the_printed_name_changes_nothing() {
    assert_eq!(decompile(&["name v6 t", "name t v6"]), decompile(&[]));
}

/// `v7` is taken, so the local printed as `v7` takes a number past the last;
/// the number `v3` left stays unused, and nothing else moves.
#[test]
fn renaming_onto_another_locals_number_gives_that_local_a_new_one() {
    renames_only(&["name v3 v7"], &[("v7", "v3"), ("v14", "v7")]);
    renames_only(&["name v1 c", "name v6 v9"], &[("c", "v1"), ("v9", "v6"), ("v14", "v9")]);
}

/// The rename binds the instance at the variable's representative, which
/// splits `v5` on the second decompile. The part left over is a local the
/// first decompile did not print: it takes a new number, and no printed
/// local's number moves.
#[test]
fn a_split_remainder_takes_a_new_number() {
    let plain = declared(&decompile(&[]));
    let renamed = declared(&decompile(&["name v5 hash"]));
    let kept: BTreeSet<String> = plain.iter().filter(|n| *n != "v5").cloned().collect();
    let added: BTreeSet<String> = renamed.difference(&kept).cloned().collect();
    assert!(kept.is_subset(&renamed), "{plain:?} -> {renamed:?}");
    assert!(!renamed.contains("v5"), "{renamed:?}");
    assert_eq!(added, ["hash", "v14"].iter().map(|s| s.to_string()).collect());
}

/// Under Ghidra names the register locals read `iVar6` and the stack array
/// still reads `v4`; both keep their numbers.
#[test]
fn ghidra_names_keep_their_numbers() {
    renames_only_with(&["--option", "namestyle", "ghidra"], &["name iVar6 vowels"], &[("vowels", "iVar6")]);
}

/// Distinct locals hold temporaries of one width defined by one instruction,
/// so a temporary is identified by the operation that defines it: keyed by
/// its instruction alone, `name v5 x` printed the old `v3` as `v8`, so a
/// second rename naming `v3` reached another local than the one shown.
#[test]
fn locals_sharing_temporaries_at_an_instruction_keep_their_numbers() {
    let function = "handle_intel.constprop.0";
    renames_only_in("mcount_x86_64", function, &[], &["name v5 x"], &[("x", "v5")]);
    renames_only_in("mcount_x86_64", function, &[], &["name v5 x", "name v3 y"], &[("x", "v5"), ("y", "v3")]);
}

/// A retype that changes which storage the analysis merges keeps every
/// printed number.
#[test]
fn a_retype_that_changes_merging_keeps_every_number() {
    let plain = declared(&decompile(&[]));
    assert_eq!(declared(&decompile(&["type v8 unsigned long"])), plain);
}
