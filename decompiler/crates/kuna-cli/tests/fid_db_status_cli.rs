//! Which `.fid` database a run matches against, and what it says about the one
//! it was given (GH #1475): a database applies only to an image whose language
//! has its header language's processor, address size and instruction byte
//! order, and every configured database is reported on stderr and in
//! `fid_databases`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use kuna_analysis::fid::db::{FidDatabase, FidDb};
use serde_json::Value;

use crate::common;

fn binary() -> String {
    common::fixture("fid_audit_x86_64")
}

fn database() -> String {
    common::fixture("fid_audit_x86_64.fid")
}

fn relabeled(lang: &str, cspec: &str) -> PathBuf {
    let db = FidDb::load(&std::fs::read(database()).unwrap()).unwrap();
    let records = db.records().into_iter().cloned().collect();
    let path = common::scratch_file("fid-relabeled", "fid");
    std::fs::write(&path, FidDb::from_records(lang, cspec, records).serialize()).unwrap();
    path
}

fn kuna(db: Option<&Path>, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args(args).env("SLEIGHHOME", common::repo_root().join("specs")).env_remove("kuna_fid_db");
    if let Some(db) = db {
        command.env("kuna_fid_db", db);
    }
    command.output().unwrap()
}

fn functions(db: Option<&Path>) -> (Value, String) {
    let out = kuna(db, &["functions", &binary(), "--option", "listing", "on", "--option", "fid", "on", "--json"]);
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    (serde_json::from_slice(&out.stdout).unwrap(), stderr)
}

fn names(doc: &Value) -> Vec<&str> {
    doc["functions"].as_array().unwrap().iter().map(|f| f["name"].as_str().unwrap()).collect()
}

fn only_report(doc: &Value) -> &Value {
    let reports = doc["fid_databases"].as_array().expect("fid_databases");
    assert_eq!(reports.len(), 1, "{reports:?}");
    &reports[0]
}

#[test]
fn a_matching_database_names_the_functions_and_reports_loaded() {
    let db = PathBuf::from(database());
    let (doc, stderr) = functions(Some(&db));
    assert_eq!(names(&doc), ["audit_mix", "audit_fold"]);
    let report = only_report(&doc);
    assert_eq!(report["path"], database());
    assert_eq!(report["status"], "loaded");
    assert_eq!(report["records"], 2);
    assert_eq!(report["language"], "x86:LE:64:default");
    assert_eq!(report["compiler_spec"], "gcc");
    assert_eq!(report["reason"], Value::Null);
    assert!(!stderr.contains("FID database"), "{stderr}");
}

#[test]
fn a_database_for_another_processor_or_size_names_nothing() {
    for lang in ["AARCH64:LE:64:v8A", "x86:LE:32:default", "x86:LE:64:nonesuch", "garbage"] {
        let db = relabeled(lang, "default");
        let (doc, stderr) = functions(Some(&db));
        assert_eq!(names(&doc), ["sub_401000", "sub_401020"], "{lang}");
        let report = only_report(&doc);
        assert_eq!(report["status"], "incompatible", "{lang}");
        assert_eq!(report["records"], 2);
        assert_eq!(report["language"], lang);
        assert!(report["reason"].as_str().unwrap().contains(lang), "{report}");
        assert_eq!(stderr.matches("warning: FID database").count(), 1, "{stderr}");
        assert!(stderr.contains("no function was matched against it"), "{stderr}");
    }
    let db = relabeled("AARCH64:LE:64:v8A", "default");
    let reason = only_report(&functions(Some(&db)).0)["reason"].as_str().unwrap().to_string();
    assert!(reason.contains("x86:LE:64:default"), "{reason}");
}

#[test]
fn the_variant_and_compiler_spec_do_not_decide() {
    let db = relabeled("x86:LE:64:compat32", "windows");
    let (doc, stderr) = functions(Some(&db));
    assert_eq!(names(&doc), ["audit_mix", "audit_fold"]);
    assert_eq!(only_report(&doc)["status"], "loaded");
    assert_eq!(only_report(&doc)["compiler_spec"], "windows");
    assert!(!stderr.contains("FID database"), "{stderr}");
}

#[test]
fn a_missing_unreadable_or_invalid_database_is_reported_and_the_run_succeeds() {
    let missing = common::scratch_file("fid-missing", "fid");
    let directory = common::scratch_file("fid-directory", "fid");
    std::fs::create_dir_all(&directory).unwrap();
    let truncated = common::scratch_file("fid-truncated", "fid");
    std::fs::write(&truncated, &std::fs::read(database()).unwrap()[..40]).unwrap();
    let garbage = common::scratch_file("fid-garbage", "fid");
    std::fs::write(&garbage, b"garbage not a fid db\n").unwrap();
    for (db, status, says) in [
        (&missing, "missing", "no such file"),
        (&directory, "unreadable", "cannot be read"),
        (&truncated, "invalid", "not a kuna .fid database"),
        (&garbage, "invalid", "not a kuna .fid database"),
    ] {
        let (doc, stderr) = functions(Some(db));
        assert_eq!(names(&doc), ["sub_401000", "sub_401020"], "{status}");
        let report = only_report(&doc);
        assert_eq!(report["path"], db.to_str().unwrap());
        assert_eq!(report["status"], status);
        assert_eq!(report["records"], Value::Null);
        assert_eq!(report["language"], Value::Null);
        assert!(report["reason"].is_string(), "{report}");
        assert!(stderr.contains(&format!("warning: FID database {}: {says}", db.display())), "{stderr}");
    }
    let (doc, _) = functions(Some(&garbage));
    assert_eq!(only_report(&doc)["reason"], "fid: bad magic [67, 61, 72, 62] (expected [4b, 46, 49, 44])");
}

#[test]
fn without_a_configured_database_the_documents_are_unchanged() {
    let (doc, stderr) = functions(None);
    assert!(doc.get("fid_databases").is_none());
    assert_eq!(names(&doc), ["sub_401000", "sub_401020"]);
    assert!(!stderr.contains("FID database"), "{stderr}");
    let out = kuna(None, &["decompile", &binary(), "--addr", "0x401020", "--json"]);
    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(doc.get("fid_databases").is_none());
}

#[test]
fn every_document_that_reports_a_run_carries_the_status() {
    let db = relabeled("AARCH64:LE:64:v8A", "default");
    let decompiled = kuna(Some(&db), &["decompile", &binary(), "--addr", "0x401020", "--json"]);
    assert_eq!(decompiled.status.code(), Some(0));
    let doc: Value = serde_json::from_slice(&decompiled.stdout).unwrap();
    assert_eq!(only_report(&doc)["status"], "incompatible");
    assert_eq!(doc["functions"][0]["name"], "sub_401020");
    let all = kuna(Some(&db), &["decompile-all", &binary(), "--json"]);
    assert_eq!(only_report(&serde_json::from_slice(&all.stdout).unwrap())["status"], "incompatible");

    let summary = kuna(Some(&db), &["functions", &binary(), "--summary", "--json"]);
    let doc: Value = serde_json::from_slice(&summary.stdout).unwrap();
    assert_eq!(only_report(&doc)["status"], "incompatible");
    let warnings = doc["warnings"].as_array().unwrap();
    assert!(warnings.iter().any(|w| w.as_str().unwrap().starts_with(&format!("FID database {}: ", db.display()))), "{warnings:?}");
    let text = kuna(Some(&db), &["functions", &binary(), "--summary"]);
    let text = String::from_utf8(text.stdout).unwrap();
    assert!(text.contains(&format!("fid database\tincompatible\t2\t{}\n", db.display())), "{text}");
}

#[test]
fn the_text_decompile_surface_warns_too() {
    let db = relabeled("AARCH64:LE:64:v8A", "default");
    let out = kuna(Some(&db), &["decompile", &binary(), "0x401020", "--addr"]);
    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert!(stdout.contains("sub_401020"), "{stdout}");
    assert_eq!(stderr.matches(&format!("warning: FID database {}: ", db.display())).count(), 1, "{stderr}");
    let matched = kuna(Some(Path::new(&database())), &["decompile", &binary(), "0x401020", "--addr"]);
    assert!(String::from_utf8(matched.stdout).unwrap().contains("audit_fold"));
    assert!(!String::from_utf8(matched.stderr).unwrap().contains("FID database"));
}

#[test]
fn build_refuses_a_language_that_does_not_describe_the_object() {
    let object = common::fixture("fid_audit_x86_64.o");
    let out = common::scratch_file("fid-build", "fid");
    let build = |lang: &str| {
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["fid", "build", &object, "-o"])
            .arg(&out)
            .args(["--lang", lang, "--cspec", "gcc"])
            .env("SLEIGHHOME", common::repo_root().join("specs"))
            .output()
            .unwrap()
    };
    let refused = build("AARCH64:LE:64:v8A");
    assert_eq!(refused.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(stderr.contains("--lang AARCH64:LE:64:v8A does not describe this object"), "{stderr}");
    assert!(!out.exists());
    let built = build("x86:LE:64:default");
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(database()).unwrap());
    let variant = build("x86:LE:64:compat32");
    assert!(variant.status.success(), "{}", String::from_utf8_lossy(&variant.stderr));
    assert_eq!(FidDb::load(&std::fs::read(&out).unwrap()).unwrap().lang_id(), "x86:LE:64:compat32");
}

#[test]
fn query_uses_the_same_rule() {
    let query = |db: &Path| {
        kuna(None, &["fid", "query", &binary(), "--db", db.to_str().unwrap(), "--addr", "0x401020", "--json"])
    };
    let variant = query(&relabeled("x86:LE:64:compat32", "gcc"));
    assert!(variant.status.success(), "{}", String::from_utf8_lossy(&variant.stderr));
    let doc: Value = serde_json::from_slice(&variant.stdout).unwrap();
    assert_eq!(doc["status"], "single_candidate");
    assert_eq!(doc["candidates"][0]["name"], "audit_fold");
    let other = query(&relabeled("AARCH64:LE:64:v8A", "default"));
    assert_eq!(other.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&other.stderr).contains("database language AARCH64:LE:64:v8A"));
}
