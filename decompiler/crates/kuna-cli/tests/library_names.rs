//! `libcnames`: a local takes the name of the library call it meets, as a
//! second name for the number it keeps. Every local is numbered exactly as
//! with the option off, so a directive reaches the same variable in every
//! mode, by its library name or by its number, and a rename or retype changes
//! that local and no other.
//!
//! Fixtures: `kuna-analysis/tests/fixtures/libcnames_x86_64` and
//! `libcnames_O0_x86_64` (source beside them). `copy_file` prints `fd`,
//! `fd_2`, `n`, `v4` and `v5` under the `aggressive` preset `auto` selects for
//! it. Every case runs on both surfaces: the text one drives `decomp_dbg`,
//! `--json` runs in-process.

use std::collections::BTreeSet;
use std::process::Command;

use crate::common;
use common::process;

/// The C of `function` in `fixture` with `extra` arguments under `directives`,
/// after checking that every directive applied and both surfaces printed the
/// same C.
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
    for outcome in parsed["assertions"].as_array().into_iter().flatten() {
        assert_eq!(outcome["status"], "applied", "{directives:?}: {outcome}");
    }
    let json = parsed
        .pointer("/functions/0/code")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("no code: {stderr}\n{doc}"));
    assert_eq!(text.trim_end(), json.trim_end(), "{directives:?}: the two surfaces disagree");
    text
}

/// [`decompile_in`] of `copy_file` in the `-O2` fixture.
fn decompile(extra: &[&str], directives: &[&str]) -> String {
    decompile_in("libcnames_x86_64", "copy_file", extra, directives)
}

/// `code` with each `(from, to)` identifier swapped, as a sorted list of
/// lines (declarations are printed in name order, so a rename reorders them).
fn lines_renamed(code: &str, renames: &[(&str, &str)]) -> Vec<String> {
    let word = regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").unwrap();
    let mut lines: Vec<String> = code
        .lines()
        .map(|line| {
            word.replace_all(line, |m: &regex::Captures| {
                let name = &m[0];
                renames.iter().find(|(from, _)| *from == name).map_or(name, |(_, to)| *to).to_string()
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

fn names(list: &[&str]) -> BTreeSet<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// `directives` leave the C exactly as it was apart from `renames`.
fn renames_only(directives: &[&str], renames: &[(&str, &str)]) {
    let plain = decompile(&[], &[]);
    let renamed = decompile(&[], directives);
    assert_eq!(
        lines_renamed(&renamed, renames),
        lines_renamed(&plain, &[]),
        "{directives:?} renamed more than {renames:?}:\n{plain}\n---\n{renamed}"
    );
}

#[test]
fn the_preset_names_the_locals_and_numbers_them_as_the_shipped_default_does() {
    let named = decompile(&[], &[]);
    assert_eq!(declared(&named), names(&["fd", "fd_2", "n", "v4", "v5"]), "{named}");
    assert!(named.contains("  fd = open(a0,0);\n"), "{named}");
    assert!(named.contains("n = read(fd,v4,0x200);"), "{named}");
    let plain = decompile(&["--mode", "reliable"], &[]);
    assert_eq!(declared(&plain), names(&["v1", "v2", "v3", "v4", "v5"]), "{plain}");
    assert_eq!(lines_renamed(&named, &[("fd", "v1"), ("fd_2", "v2"), ("n", "v3")]).len(), plain.lines().count());
    let on = decompile(&["--mode", "reliable", "--option", "libcnames", "on"], &[]);
    assert_eq!(declared(&on), names(&["fd", "fd_2", "n", "v4", "v5"]), "{on}");
    let off = decompile(&["--option", "libcnames", "off"], &[]);
    assert_eq!(declared(&off), names(&["v1", "v2", "v3", "v4", "v5"]), "{off}");
}

#[test]
fn renaming_a_library_name_moves_no_other_local() {
    renames_only(&["name fd src_fd"], &[("src_fd", "fd")]);
    renames_only(&["name fd_2 dst_fd"], &[("dst_fd", "fd_2")]);
    renames_only(&["name n got"], &[("got", "n")]);
    renames_only(&["name fd src_fd", "name fd_2 dst_fd"], &[("src_fd", "fd"), ("dst_fd", "fd_2")]);
}

#[test]
fn renaming_a_numbered_local_moves_no_library_name() {
    renames_only(&["name v5 total"], &[("total", "v5")]);
    renames_only(&["name v4 block"], &[("block", "v4")]);
}

/// The name a rename frees is handed to no other local: `fd_2` does not
/// become `fd`.
#[test]
fn a_freed_library_name_goes_to_no_other_local() {
    let code = decompile(&[], &["name fd src_fd"]);
    assert_eq!(declared(&code), names(&["fd_2", "n", "src_fd", "v4", "v5"]), "{code}");
}

/// A retype that keeps the width names nothing anew, and is applied by the
/// library name or the number alike.
#[test]
fn a_retype_of_a_library_named_local_keeps_every_name() {
    for (directive, local) in [
        ("type fd unsigned int", "fd"),
        ("type v1 unsigned int", "fd"),
        ("type fd_2 unsigned int", "fd_2"),
        ("type v2 unsigned int", "fd_2"),
    ] {
        let code = decompile(&[], &[directive]);
        assert_eq!(declared(&code), names(&["fd", "fd_2", "n", "v4", "v5"]), "{directive}:\n{code}");
        assert!(code.contains(&format!("  unsigned int {local}; // ")), "{directive}:\n{code}");
    }
    let code = decompile(&[], &["type n unsigned long"]);
    assert!(code.contains("  unsigned long n; // rax\n"), "{code}");
    let code = decompile(&[], &["type fd unsigned int", "name fd src"]);
    assert!(code.contains("  unsigned int src; // ") && code.contains("  int fd_2; // "), "{code}");
}

/// A rename to a name shaped like a number is the caller's name, not a number
/// to print a library name over: `name fd v9` prints `v9`, which the next
/// directive reaches, and a numbered local renamed onto a library-named
/// local's number leaves that local its library name.
#[test]
fn a_number_shaped_rename_is_kept() {
    renames_only(&["name fd v9"], &[("v9", "fd")]);
    renames_only(&["name fd v9", "name v9 x"], &[("x", "fd")]);
    renames_only(&["name fd v9", "name fd_2 v8"], &[("v9", "fd"), ("v8", "fd_2")]);
    renames_only(&["name v4 v1"], &[("v1", "v4")]);
    renames_only(&["name fd v1"], &[("v1", "fd")]);
}

/// A local keeps its number whatever the mode prints, so a directive the
/// page recorded in one mode reaches the same variable after a switch to
/// another, by its number or by its library name.
#[test]
fn a_directive_reaches_the_same_local_in_every_mode() {
    for mode in ["auto", "aggressive", "reliable"] {
        let code = decompile(&["--mode", mode], &["name v4 block"]);
        assert!(code.contains("  char block [520]; // stack - 0x228\n"), "{mode}:\n{code}");
        let code = decompile(&["--mode", mode], &["name fd src"]);
        assert!(code.contains("  src = open(a0,0);\n"), "{mode}:\n{code}");
        let code = decompile(&["--mode", mode], &["name v1 src"]);
        assert!(code.contains("  src = open(a0,0);\n"), "{mode}:\n{code}");
    }
}

/// A rename that takes a library name leaves the local that printed it its
/// number, and both names stay addressable: the next directive reaches the
/// renamed local by the name it now has, and the other by its number.
#[test]
fn a_local_a_rename_took_its_library_name_from_prints_its_number() {
    let code = decompile(&[], &["name v5 n"]);
    assert_eq!(declared(&code), names(&["fd", "fd_2", "n", "v3", "v4"]), "{code}");
    assert!(code.contains("v3 = read(fd,v4,0x200);") && code.contains("n += write(fd_2,v4,v3);"), "{code}");
    let code = decompile(&[], &["name v5 n", "name v3 got"]);
    assert!(code.contains("got = read(fd,v4,0x200);") && code.contains("n += write(fd_2,v4,got);"), "{code}");
    let code = decompile(&[], &["name v5 n", "name n total"]);
    assert!(code.contains("n = read(fd,v4,0x200);") && code.contains("total += write(fd_2,v4,n);"), "{code}");
    renames_only(&["name v5 fd_2", "name v2 second"], &[("fd_2", "v5"), ("second", "fd_2")]);
}

/// A library name never repeats a global's, whether the function reads the
/// global (`note_pid`) or not (`self_pid`), and the suffixed name a clash
/// takes is addressable.
#[test]
fn a_library_name_takes_a_suffix_past_a_global() {
    for function in ["note_pid", "self_pid"] {
        let code = decompile_in("libcnames_x86_64", function, &[], &[]);
        assert!(code.contains("  pid_2 = getpid();\n"), "{function}:\n{code}");
        let code = decompile_in("libcnames_x86_64", function, &[], &["name pid_2 mine"]);
        assert!(code.contains("  mine = getpid();\n"), "{function}:\n{code}");
    }
    let code = decompile_in("libcnames_x86_64", "note_pid", &[], &[]);
    assert!(code.contains("pid = pid_2;"), "{code}");
}

/// `access` names its mode argument `type`, a Rust keyword, which no local
/// takes in either language.
#[test]
fn a_keyword_is_never_a_library_name() {
    for language in ["c", "rust"] {
        let code = decompile_in("libcnames_O0_x86_64", "can", &["--language", language], &[]);
        assert!(!regex::Regex::new(r"\btype\b").unwrap().is_match(&code), "{language}:\n{code}");
    }
}

/// `localtime`'s result is `tp`, and a library name is never a type the code
/// prints (`tm`): the printed `stamp`, compiled beside the source's own and a
/// layout of `struct tm`, computes what the source does under gcc and clang.
#[test]
fn a_library_name_never_shadows_a_type_and_the_printed_c_runs() {
    let text = decompile_in("libcnames_O0_x86_64", "stamp", &[], &[]);
    assert!(text.contains("  tm *tp; //") && text.contains("  tm *tp_2; //"), "{text}");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    const PRELUDE: &str = "struct tm { int field_0x0, field_0x4, field_0x8, field_0xc, field_0x10, field_0x14, \
                           field_0x18, field_0x1c, field_0x20; long field_0x28; const char *field_0x30; };\n\
                           typedef struct tm tm;\nstruct tm *localtime(const void *);\nint printf(const char *, ...);\n";
    const MAIN: &str = "int s_stamp(long, long);\nint main(void) {\n  static const long t[] = {0, 100000, \
                        86400 * 400L, 1700000000L, 951782400L};\n  int bad = 0;\n  for (int i = 0; i < 5; i++)\n    \
                        for (int j = 0; j < 5; j++)\n      if (stamp(t[i], t[j]) != s_stamp(t[i], t[j])) {\n        \
                        printf(\"%ld %ld\\n\", t[i], t[j]);\n        bad++;\n      }\n  return bad != 0;\n}\n";
    let source = common::repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/libcnames_x86_64.c");
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let printed = common::scratch_file(&format!("libcnames-stamp-{cc}{level}"), "c");
            let reference = printed.with_extension("o");
            let exe = printed.with_extension("exe");
            std::fs::write(&printed, format!("{PRELUDE}{text}\n{MAIN}")).unwrap();
            let out = Command::new(cc)
                .args(["-std=gnu11", "-w", level, "-c", "-o", reference.to_str().unwrap(), source.to_str().unwrap()])
                .args(["-Dstamp=s_stamp", "-Dmain=fixture_main"])
                .output()
                .expect("spawn the C compiler");
            assert!(out.status.success(), "{cc} {level} rejected the fixture source:\n{}", String::from_utf8_lossy(&out.stderr));
            let out = Command::new(cc)
                .args(["-std=gnu11", "-w", level, "-o", exe.to_str().unwrap(), printed.to_str().unwrap()])
                .arg(reference.to_str().unwrap())
                .args(common::CC_GCC15_DEMOTE)
                .output()
                .expect("spawn the C compiler");
            assert!(out.status.success(), "{cc} {level} rejected the printed C:\n{}\n{text}", String::from_utf8_lossy(&out.stderr));
            let run = Command::new(&exe).env("TZ", "UTC").output().expect("run the round trip");
            assert!(run.status.success(), "{cc} {level}: the printed stamp computes a different value:\n{}\n{text}", String::from_utf8_lossy(&run.stdout));
        }
    }
}
