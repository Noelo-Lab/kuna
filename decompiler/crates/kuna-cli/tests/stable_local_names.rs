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

/// `v13` is two values the analysis merged in one register; the rename names
/// the variable once, under one declaration.
#[test]
fn renaming_the_last_local_keeps_every_other_number() {
    renames_only(&["name v13 acc"], &[("acc", "v13")]);
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

/// A name-only rename names the whole printed variable: `v5` spans four
/// registers and used to split, leaving the `rax` part behind as a new `v14`;
/// `v7` used to come back retyped `unsigned long` with a cast at its compare.
#[test]
fn a_rename_neither_splits_nor_retypes_the_variable() {
    renames_only(&["name v5 hash"], &[("hash", "v5")]);
    renames_only(&["name v7 best"], &[("best", "v7")]);
}

/// Two such renames in one batch, and a rename of a renamed local.
#[test]
fn successive_renames_name_the_whole_variable() {
    renames_only(&["name v5 hash", "name v7 best"], &[("hash", "v5"), ("best", "v7")]);
    renames_only(&["name v5 hash", "name hash h"], &[("h", "v5")]);
}

/// The console carries a rename across every later decompile: one rename per
/// decompile, the second naming a local by the number the first left alone.
#[test]
fn console_renames_survive_later_decompiles() {
    let script = "load function mix\ndecompile\nprint C\nrename v5 hash\ndecompile\nprint C\n\
                  rename v7 best\ndecompile\nprint C\nrename hash h\ndecompile\nprint C";
    let out = console(script);
    let printed: Vec<&str> = out
        .split(" mix(")
        .skip(1)
        .map(|block| block.find("\n}\n").map_or(block, |end| &block[..end + 2]))
        .collect();
    assert_eq!(printed.len(), 4, "{out}");
    let plain = format!(" mix({}", printed[0]);
    let steps: [&[(&str, &str)]; 3] =
        [&[("hash", "v5")], &[("hash", "v5"), ("best", "v7")], &[("h", "v5"), ("best", "v7")]];
    for (step, renames) in printed[1..].iter().zip(steps) {
        let code = format!(" mix({step}");
        assert_eq!(lines_renamed(&code, renames), lines_renamed(&plain, &[]), "{renames:?}:\n{plain}\n---\n{code}");
    }
}

/// Drive `decomp_dbg` with `script` on the fixture.
fn console(script: &str) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let engine = std::env::var_os("KUNA_DECOMP_DBG").map(std::path::PathBuf::from).unwrap_or_else(|| {
        std::path::Path::new(env!("CARGO_BIN_EXE_kuna")).parent().unwrap().join("decomp_dbg")
    });
    let specs = std::env::var_os("SLEIGHHOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"));
    let mut child = Command::new(engine)
        .arg("-s")
        .arg(specs)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run console");
    let script = format!(
        "load file \"{}\"\nread symbols\n{script}\nquit\n",
        common::fixture("stablenames_x86_64")
    );
    child.stdin.take().unwrap().write_all(script.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
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

/// Retyping leaves the C as it was apart from the retyped declaration
/// (`decl`: as retyped, as printed before) and the `casts` at its uses.
fn retypes_only_in(
    fixture: &str,
    function: &str,
    directives: &[&str],
    renames: &[(&str, &str)],
    decl: (&str, &str),
    casts: &[&str],
) {
    let plain = decompile_in(fixture, function, &[], &[]);
    let retyped = decompile_in(fixture, function, &[], directives);
    assert!(retyped.contains(decl.0), "{directives:?}:\n{retyped}");
    let uncast = |code: &str| casts.iter().fold(code.to_string(), |code, cast| code.replace(cast, ""));
    assert_eq!(
        lines_renamed(&uncast(&retyped.replacen(decl.0, decl.1, 1)), renames),
        lines_renamed(&uncast(&plain), &[]),
        "{directives:?} changed more than the declaration and its casts:\n{plain}\n---\n{retyped}"
    );
}

/// A retype that keeps a local's width types the whole printed variable:
/// `type v5 unsigned long` used to split `v5`'s `rax` part off as a new
/// `v14`, `type v13 long` merged `v12` and `v5` differently and added a
/// `v14`, and in `handle_intel` `type v6 unsigned long` and `type v9 int` each
/// added a `v11`.
#[test]
fn a_same_width_retype_types_the_whole_variable() {
    retypes_only_in(
        "stablenames_x86_64",
        "mix",
        &["type v5 unsigned long"],
        &[],
        ("  unsigned long v5;", "  long v5;"),
        &["(unsigned long)", "(long)"],
    );
    retypes_only_in("stablenames_x86_64", "mix", &["type v13 long"], &[], ("  long v13;", "  unsigned long v13;"), &[]);
    retypes_only_in("stablenames_x86_64", "mix", &["type v8 unsigned long"], &[], ("  unsigned long v8;", "  long v8;"), &[]);
    let function = "handle_intel.constprop.0";
    retypes_only_in("mcount_x86_64", function, &["type v6 unsigned long"], &[], ("  unsigned long v6;", "  long v6;"), &[]);
    retypes_only_in("mcount_x86_64", function, &["type v9 int"], &[], ("  int v9;", "  unsigned int v9;"), &["(unsigned int)"]);
}

/// The page's order: name a local, then retype it under the new name.
#[test]
fn a_rename_then_a_retype_types_the_whole_variable() {
    retypes_only_in(
        "stablenames_x86_64",
        "mix",
        &["name v5 hash", "type hash unsigned long hash"],
        &[("hash", "v5")],
        ("  unsigned long hash;", "  long hash;"),
        &["(unsigned long)", "(long)"],
    );
}

/// The console carries a retype across every later decompile, also once the
/// retyped local is renamed.
#[test]
fn console_retypes_survive_later_decompiles() {
    let script = "load function mix\ndecompile\nprint C\nretype v5 unsigned long\ndecompile\nprint C\n\
                  decompile\nprint C\nrename v5 hash\ndecompile\nprint C";
    let out = console(script);
    let printed: Vec<String> = out
        .split(" mix(")
        .skip(1)
        .map(|block| format!(" mix({}", block.find("\n}\n").map_or(block, |end| &block[..end + 2])))
        .collect();
    assert_eq!(printed.len(), 4, "{out}");
    let uncast = |code: &str| code.replace("(uint8)", "").replace("(int8)", "");
    let plain = lines_renamed(&uncast(&printed[0]), &[]);
    for (code, decl, renames) in [
        (&printed[1], "  uint8 v5;", &[][..]),
        (&printed[2], "  uint8 v5;", &[][..]),
        (&printed[3], "  uint8 hash;", &[("hash", "v5")][..]),
    ] {
        assert!(code.contains(decl), "{decl}:\n{code}");
        let code = uncast(&code.replacen(decl, &decl.replace("uint8", "int8"), 1));
        assert_eq!(lines_renamed(&code, renames), plain, "{decl}:\n{}\n---\n{code}", printed[0]);
    }
}

/// A retype to a pointer to a structure still drives type inference, which
/// is what prints the fields read through it.
#[test]
fn a_structure_pointer_retype_prints_fields() {
    let fixture = "globalpointee_gcc_O2_x86_64";
    let typedef = "typedef struct rec { int a; short b; short pad; long x; long y; int c; } rec;";
    let code = decompile_in(fixture, "r_rec", &[], &[typedef, "type v3 rec *"]);
    assert!(code.contains("  rec *v3;") && code.contains("v1 = v3->b;"), "{code}");
    assert_eq!(declared(&code), declared(&decompile_in(fixture, "r_rec", &[], &[])));
}

/// Such a retype types every definition of the variable: `name_len`'s `v1`
/// holds either call's result, and the type used to reach only one of them,
/// splitting the other off as `char *v2`.
#[test]
fn a_structure_pointer_retype_keeps_the_variable_whole() {
    let fixture = "callrettype_gcc_O2_x86_64";
    let typedef = "typedef struct S { long f0; long f1; } S;";
    let code = decompile_in(fixture, "name_len", &[], &[typedef, "type v1 S *"]);
    assert_eq!(declared(&code), declared(&decompile_in(fixture, "name_len", &[], &[])), "{code}");
    assert_eq!(code.matches("v1 = (S *)").count(), 2, "{code}");
}

/// The retyped variable keeps the name a directive pins on it while the type
/// reshapes the code around it: `walk`'s pointer is found by its lock once the
/// first decompile's keys miss it (it printed as `v3`), and `squeeze`'s stays
/// declared where the reshaped loop could inline it (it vanished).
#[test]
fn a_structure_pointer_retype_keeps_its_name() {
    let fixture = "retypeptr_x86_64";
    let typedef = "typedef struct S { long a; long b; } S;";
    for (function, directive, decl) in
        [("walk", "type v2 S *v2", "  S *v2;"), ("walk", "type v2 S *", "  S *v2;"), ("squeeze", "type v1 S *v1", "  S *v1;")]
    {
        let code = decompile_in(fixture, function, &[], &[typedef, directive]);
        assert!(code.contains(decl), "{function} {directive}:\n{code}");
        assert_eq!(declared(&code), declared(&decompile_in(fixture, function, &[], &[])), "{code}");
    }
}

/// The type is locked only on what the variable's own operations define:
/// `mix`'s `v10` points into the stack buffer `v4`, and a lock that also
/// took varnodes of the same storage and instruction re-laid the frame
/// around the pointer, losing `v4`.
#[test]
fn a_structure_pointer_retype_locks_only_the_variable() {
    let typedef = "typedef struct S { long a; long b; int c; int d; char *e; long f[4]; } S;";
    let code = decompile(&[typedef, "type v10 S *v10"]);
    assert!(code.contains("  S *v10;"), "{code}");
    assert_eq!(declared(&code), declared(&decompile(&[])), "{code}");
}

/// A retype to an enumeration also reaches type inference, which is what
/// names the constants the variable meets: applied after the merge alone,
/// `v1 = RUN` printed `v1 = 3`.
#[test]
fn an_enumeration_retype_names_its_constants() {
    let fixture = "retypeenum_x86_64";
    let state = "typedef enum state { IDLE = 0, RUN = 3, DONE = 9 } state;";
    let code = decompile_in(fixture, "step", &[], &[state, "type v1 state v1"]);
    for line in ["  state v1;", "v1 = IDLE;", "(v1 == RUN)", "v1 = DONE;"] {
        assert!(code.contains(line), "{line}:\n{code}");
    }
    assert_eq!(declared(&code), declared(&decompile_in(fixture, "step", &[], &[])), "{code}");
    let color = "typedef enum color { RED = 0, GREEN = 1, BLUE = 7 } color;";
    let code = decompile_in(fixture, "pick", &[], &[color, "type v2 color"]);
    for line in ["  color v2;", "v2 = RED;", "v2 = BLUE;", "v2 = GREEN;"] {
        assert!(code.contains(line), "{line}:\n{code}");
    }
}

/// A local the function only passes by address (`read`'s buffer) has no
/// instance to carry a retype to, so its Symbol takes the type: the retype
/// used to leave `char v1` in place while reporting itself applied.
#[test]
fn an_address_only_local_takes_its_retype() {
    let fixture = "retypeoutparam_x86_64";
    let e4 = "typedef enum E4 { E_ZERO = 0, E_ONE = 1 } E4;";
    for (function, directives, decl) in [
        ("rd", &["type v1 unsigned char v1"][..], "  unsigned char v1;"),
        ("rd", &["type v1 bool v1"][..], "  bool v1;"),
        ("rd", &["type v1 unsigned char"][..], "  unsigned char v1;"),
        ("rd", &["name v1 ch", "type ch unsigned char ch"][..], "  unsigned char ch;"),
        ("rdn", &["type v1 int v1"][..], "  int v1;"),
        ("rdn", &[e4, "type v1 E4 v1"][..], "  E4 v1;"),
    ] {
        let code = decompile_in(fixture, function, &[], directives);
        assert!(code.contains(decl), "{function} {directives:?}:\n{code}");
    }
}

/// A stack local read whole and by halves is three locals under one Symbol,
/// printed by one declaration, and the retype goes to the whole one: a half
/// that tied with it took the type, and the declaration kept its old one.
#[test]
fn a_stack_local_read_by_halves_is_retyped_whole() {
    let fixture = "retypehalves_x86_64";
    let plain = decompile_in(fixture, "control", &[], &[]);
    assert!(plain.contains("  unsigned long v6; // rdx | stack - 0x38"), "{plain}");
    let code = decompile_in(fixture, "control", &[], &["type v6 long"]);
    assert!(code.contains("  long v6; // rdx | stack - 0x38"), "{code}");
    assert_eq!(declared(&code), declared(&plain), "{code}");
}

/// A DWARF-named local takes a rename and a retype on both surfaces: the
/// console re-seeded the DWARF local over the Symbol the directive had
/// renamed, so the text surface kept printing `n`.
#[test]
fn a_dwarf_local_takes_its_rename_and_retype() {
    let fixture = "castsign_dwarf_gcc_O0_x86_64";
    for (directive, decl) in [("name n count", "  unsigned long count;"), ("type n long count", "  long count;")] {
        let code = decompile_in(fixture, "sign_of", &[], &[directive]);
        assert!(code.contains(decl), "{directive}:\n{code}");
    }
}

/// The `--json` outcome of each directive on `function`, after checking that
/// the text surface exits 0 on them.
fn outcomes(fixture: &str, function: &str, directives: &[&str]) -> (String, Vec<(String, Option<String>)>) {
    let binary = common::fixture(fixture);
    let mut args = vec!["decompile", binary.as_str(), function];
    for directive in directives {
        args.extend(["--assert", directive]);
    }
    let (_, stderr, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{directives:?}: {stderr}");
    args.push("--json");
    let (doc, stderr, _) = common::run_kuna(&args);
    let parsed: serde_json::Value = serde_json::from_str(&doc).unwrap_or_else(|e| panic!("{e}: {stderr}\n{doc}"));
    let code = parsed.pointer("/functions/0/code").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
    let rows = parsed["assertions"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|row| (row["status"].as_str().unwrap_or_default().to_string(), row["detail"].as_str().map(str::to_string)))
                .collect()
        })
        .unwrap_or_default();
    (code, rows)
}

/// A rename the printer has to suffix, because the name already belongs to
/// another identifier (a function, a parameter, another renamed local), took
/// effect: it is applied, and says what it was printed as. The check that
/// reads the printed C finds the local by its declaration, so a parameter or
/// another local that merely carries the requested name proves nothing.
#[test]
fn a_suffixed_rename_is_applied_as_printed() {
    let applied_as = |name: &str| Some(format!("applied as `{name}`: `{}` is already another identifier here", name.trim_end_matches("_1")));
    for (directive, decl, printed) in [
        ("name v6 is_vowel", "  int is_vowel_1;", "is_vowel_1"),
        ("name v6 mix", "  int mix_1;", "mix_1"),
        ("name v6 a0", "  int a0_1;", "a0_1"),
    ] {
        let (code, rows) = outcomes("stablenames_x86_64", "mix", &[directive]);
        assert!(code.contains(decl), "{directive}:\n{code}");
        assert_eq!(rows, vec![("applied".to_string(), applied_as(printed))], "{directive}");
    }
    let (code, rows) = outcomes("stablenames_x86_64", "mix", &["name v6 dup", "name v7 dup"]);
    assert!(code.contains("  int dup_1; // r8d") && code.contains(" dup; // r9 | rsi"), "{code}");
    assert_eq!(rows, vec![("applied".to_string(), applied_as("dup_1")), ("applied".to_string(), None)]);
}

/// A local cannot take a C keyword or a type's name: `name v6 int` used to
/// print `int int;`.
#[test]
fn a_keyword_or_type_name_is_refused() {
    let binary = common::fixture("stablenames_x86_64");
    for (directive, reason) in [
        ("name v6 int", "`int` is a C keyword"),
        ("name v6 uint4", "`uint4` names a type"),
        ("type v6 unsigned int while", "`while` is a C keyword"),
    ] {
        let (text, stderr, code) = common::run_kuna(&["decompile", binary.as_str(), "mix", "--assert", directive]);
        assert_eq!(code, 0, "{directive}: {stderr}");
        assert!(stderr.contains("rejected") && stderr.contains(reason), "{directive}: {stderr}");
        assert!(text.contains(" v6;"), "{directive}:\n{text}");
    }
    let out = console("load function mix\ndecompile\nrename v6 int\nretype v6 unsigned int while");
    assert!(out.contains("`int` is a C keyword") && out.contains("`while` is a C keyword"), "{out}");
}
