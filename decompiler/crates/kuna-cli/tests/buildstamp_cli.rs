//! The build handshake as `kuna` users see it: an engine binary from another
//! build is named once on stderr, a matching or pre-handshake one is silent, and
//! the warning never reaches stdout, where `--json` documents go.
//!
//! The children are shell stubs speaking the child half of the protocol
//! (`kuna_console::kuna_buildstamp`): compare `KUNA_PARENT_BUILD` with a baked
//! identity and answer on stderr only when they differ. The real binaries' half
//! is pinned by `kuna-console/tests/verify_buildstamp.rs` and
//! `kuna-harness/tests/verify_buildstamp.rs`.

#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use kuna_console::kuna_buildstamp::{identity, ChildKind, PARENT_ENV, REPLY};

const OTHER_BUILD: &str = "1.329 (source 0123456789abcdef)";

fn fauxware() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/fauxware")
        .canonicalize()
        .expect("the fauxware fixture")
}

/// An executable stub at a fresh temp path. `answers_as` is the identity it
/// compares the parent's against; `None` is a child that predates the handshake.
fn stub(tag: &str, answers_as: Option<&str>, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("kuna_buildstamp_{tag}_{}", std::process::id()));
    let handshake = answers_as
        .map(|id| {
            format!(
                "if [ -n \"${{{PARENT_ENV}+x}}\" ] && [ \"${PARENT_ENV}\" != '{id}' ]; then \
                 printf '%s%s\\n' '{REPLY}' '{id}' >&2; fi\n"
            )
        })
        .unwrap_or_default();
    std::fs::write(&path, format!("#!/bin/sh\n{handshake}{body}")).expect("write the stub");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("chmod the stub");
    path
}

/// A `decomp_dbg` that writes a fixed body to the script's `openfile write` target.
fn decomp_dbg(tag: &str, answers_as: Option<&str>) -> PathBuf {
    stub(
        tag,
        answers_as,
        "out=\n\
         while IFS= read -r line; do\n\
         \x20 case \"$line\" in 'openfile write '*) out=${line#openfile write } ;; esac\n\
         done\n\
         [ -n \"$out\" ] && printf 'int main(void)\\n{\\n  return 0;\\n}\\n' > \"$out\"\n\
         printf '[decomp]> decompile\\nDecompiling main\\n'\n",
    )
}

fn kuna(args: &[&str], envs: &[(&str, &Path)]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args(args)
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove(PARENT_ENV);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn kuna")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// The reporter's two-install setup: `KUNA_DECOMP_DBG` pins an engine from an
/// older release. The render still succeeds, and stderr names both builds.
#[test]
fn a_decomp_dbg_from_another_build_is_named_once_on_stderr() {
    let child = decomp_dbg("other", Some(OTHER_BUILD));
    let out = kuna(
        &["decompile", fauxware().to_str().unwrap(), "main"],
        &[("KUNA_DECOMP_DBG", &child)],
    );
    let (stdout, stderr) = (text(&out.stdout), text(&out.stderr));
    let _ = std::fs::remove_file(&child);

    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert_eq!(
        stdout, "int main(void)\n{\n  return 0;\n}\n",
        "stdout must carry only the C"
    );
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 3, "one three-line warning, got:\n{stderr}");
    assert_eq!(
        lines[0],
        "warning: decomp_dbg is a different build from this kuna"
    );
    assert!(
        lines[1].starts_with("  kuna:") && lines[1].contains(identity(ChildKind::Engine)),
        "{stderr}"
    );
    assert!(
        lines[1].ends_with("/kuna"),
        "the kuna line must name kuna's path: {stderr}"
    );
    assert_eq!(
        lines[2],
        format!(
            "  decomp_dbg: {OTHER_BUILD} {}, chosen by KUNA_DECOMP_DBG",
            child.display()
        )
    );
}

#[test]
fn a_decomp_dbg_from_this_build_prints_nothing() {
    let child = decomp_dbg("same", Some(identity(ChildKind::Engine)));
    let out = kuna(
        &[
            "decompile",
            fauxware().to_str().unwrap(),
            "main",
            "--decomp-dbg",
            child.to_str().unwrap(),
        ],
        &[],
    );
    let _ = std::fs::remove_file(&child);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(text(&out.stderr), "");
    assert_eq!(text(&out.stdout), "int main(void)\n{\n  return 0;\n}\n");
}

/// A child that predates the handshake cannot warn, and must not break.
#[test]
fn a_decomp_dbg_that_predates_the_handshake_prints_nothing() {
    let child = decomp_dbg("old", None);
    let out = kuna(
        &["decompile", fauxware().to_str().unwrap(), "main"],
        &[("KUNA_DECOMP_DBG", &child)],
    );
    let _ = std::fs::remove_file(&child);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(text(&out.stderr), "");
    assert_eq!(text(&out.stdout), "int main(void)\n{\n  return 0;\n}\n");
}

#[test]
fn the_warning_stays_out_of_catalog_json() {
    let child = stub(
        "catalog",
        Some(OTHER_BUILD),
        "cat > /dev/null\nprintf '[decomp]> phase catalog\\n[{\"option\": \"loweredswitch\"}]\\n'\n",
    );
    let out = kuna(
        &["catalog", "--json", "--option", "loweredswitch"],
        &[("KUNA_DECOMP_DBG", &child)],
    );
    let (stdout, stderr) = (text(&out.stdout), text(&out.stderr));
    let _ = std::fs::remove_file(&child);
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert!(
        stdout.trim_start().starts_with('[') && stdout.trim_end().ends_with(']'),
        "{stdout}"
    );
    assert!(
        !stdout.contains("warning") && !stdout.contains(REPLY),
        "{stdout}"
    );
    assert!(
        stderr.starts_with("warning: decomp_dbg is a different build"),
        "{stderr}"
    );
}

#[test]
fn a_catalog_flag_is_named_as_the_binary_selector() {
    let child = stub(
        "catalog_flag",
        Some(OTHER_BUILD),
        "cat > /dev/null\nprintf '[decomp]> phase catalog\\n[{\"option\": \"loweredswitch\"}]\\n'\n",
    );
    let out = kuna(
        &[
            "catalog",
            "--json",
            "--option",
            "loweredswitch",
            "--decomp-dbg",
            child.to_str().unwrap(),
        ],
        &[],
    );
    let stderr = text(&out.stderr);
    let _ = std::fs::remove_file(&child);
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert!(stderr.contains("chosen by --decomp-dbg"), "{stderr}");
    assert!(!stderr.contains("chosen by KUNA_DECOMP_DBG"), "{stderr}");
}

/// `decomp_test_dbg` reports unit tests on stderr, the stream the reply shares:
/// the reply must be stripped before that grammar is parsed.
#[test]
fn a_decomp_test_dbg_from_another_build_is_named_and_its_results_still_parse() {
    let child = stub(
        "harness",
        Some(OTHER_BUILD),
        "printf 'testing : alpha ...\\n  passed.\\n1/1 tests passed.\\n' >&2\n",
    );
    let out = kuna(
        &[
            "test",
            "--unittests",
            "--json",
            "--binary",
            child.to_str().unwrap(),
        ],
        &[],
    );
    let (stdout, stderr) = (text(&out.stdout), text(&out.stderr));
    let _ = std::fs::remove_file(&child);
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert!(stdout.contains("\"unit:alpha\""), "{stdout}");
    assert!(
        !stdout.contains("warning") && !stdout.contains(REPLY),
        "{stdout}"
    );
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 3, "{stderr}");
    assert_eq!(
        lines[0],
        "warning: decomp_test_dbg is a different build from this kuna"
    );
    assert!(
        lines[2].ends_with(&format!(
            "{OTHER_BUILD} {}, chosen by --binary",
            child.display()
        )),
        "{stderr}"
    );
}
