//! A `.fid` record's prototype through the built CLI: written by `fid build`
//! from debug information or an `--assert`, applied to the stripped function
//! it names, and withdrawn when the body reads past it.

use std::path::Path;
use std::process::Command;

use kuna_analysis::fid::db::{FidDatabase, FidDb};

use crate::common;

const DECLARED: &str = "int parse_header(unsigned char *buf,unsigned int len,hdr *out)";
const RECOVERED: &str = "int parse_header(int *a0,unsigned int a1,int *a2)";

fn kuna() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.env("SLEIGHHOME", common::repo_root().join("specs")).env_remove("kuna_fid_db");
    command
}

fn build(object: &Path, out: &Path, extra: &[&str]) -> String {
    let result = kuna()
        .args(["fid", "build"])
        .arg(object)
        .arg("-o")
        .arg(out)
        .args(["--lang", "x86:LE:64:default", "--cspec", "gcc"])
        .args(extra)
        .output()
        .expect("run kuna fid build");
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    assert!(result.status.success(), "{stderr}");
    stderr
}

fn decompile(db: &Path, extra: &[&str]) -> String {
    let result = kuna()
        .arg("decompile-all")
        .arg(common::repo_root().join("tests/fixtures/fid/protos/prog"))
        .args(["--option", "fid", "on"])
        .args(extra)
        .env("kuna_fid_db", db)
        .output()
        .expect("run kuna decompile-all");
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    String::from_utf8(result.stdout).unwrap()
}

#[test]
fn a_record_built_from_debug_information_declares_the_stripped_function() {
    let fixtures = common::repo_root().join("tests/fixtures/fid/protos");
    let scratch = tempfile::tempdir().unwrap();
    let db = scratch.path().join("lib.fid");
    let said = build(&fixtures.join("lib.o"), &db, &[]);
    assert!(said.contains("1 record(s), 1 with a prototype"), "{said}");
    assert_eq!(std::fs::read(&db).unwrap(), std::fs::read(fixtures.join("lib.fid")).unwrap());

    let code = decompile(&db, &[]);
    assert!(code.contains(DECLARED), "{code}");
    assert!(code.contains("out->magic = "), "{code}");
    assert!(code.contains("parse_header(a0,a1,&v3)"), "{code}");

    let asserted = decompile(&db, &["--assert", "prototype parse_header long parse_header(char *p,int n,void *o)"]);
    assert!(asserted.contains("long parse_header(char *p,int n,void *o)"), "{asserted}");
}

#[test]
fn a_version_1_database_names_the_function_and_nothing_more() {
    let fixtures = common::repo_root().join("tests/fixtures/fid/protos");
    let loaded = FidDb::load(&std::fs::read(fixtures.join("lib.fid")).unwrap()).unwrap();
    let names_only: Vec<_> = loaded
        .records()
        .into_iter()
        .cloned()
        .map(|mut record| {
            assert!(record.prototype.take().is_some());
            record.typedefs.clear();
            record
        })
        .collect();
    let bytes = FidDb::from_records(loaded.lang_id(), loaded.cspec_id(), names_only).serialize();
    assert_eq!(bytes[4..8], 1u32.to_le_bytes());
    let scratch = tempfile::tempdir().unwrap();
    let db = scratch.path().join("v1.fid");
    std::fs::write(&db, bytes).unwrap();
    let code = decompile(&db, &[]);
    assert!(code.contains(RECOVERED), "{code}");
    assert!(!code.contains(".fid prototype"), "{code}");
}

#[test]
fn a_stated_prototype_the_body_reads_past_is_dropped_and_reported() {
    let fixtures = common::repo_root().join("tests/fixtures/fid/protos");
    let scratch = tempfile::tempdir().unwrap();
    let db = scratch.path().join("short.fid");
    let stated = scratch.path().join("stated.kuna");
    std::fs::write(
        &stated,
        "# one parameter short\nprototype parse_header int parse_header(unsigned char *buf,unsigned int len)\n",
    )
    .unwrap();
    build(&fixtures.join("lib.o"), &db, &["--assert", &format!("@{}", stated.display())]);
    let code = decompile(&db, &[]);
    assert!(code.contains(RECOVERED), "{code}");
    assert!(
        code.contains("the .fid prototype was dropped: the body reads parameter storage it does not declare"),
        "{code}"
    );

    let unknown = scratch.path().join("unknown.fid");
    build(
        &fixtures.join("lib.o"),
        &unknown,
        &["--assert", "prototype parse_header int parse_header(missing_t *buf,unsigned int len,void *out)"],
    );
    let code = decompile(&unknown, &[]);
    assert!(code.contains(RECOVERED), "{code}");
    assert!(code.contains("the .fid prototype `int parse_header(missing_t *buf,unsigned int len,void *out)` was dropped"), "{code}");
}
