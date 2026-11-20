//! FID archive ingestion through the built CLI, without an external archive tool.

use std::process::Command;

use crate::common;

fn archive_member(archive: &mut Vec<u8>, name: &str, bytes: &[u8]) {
    let header = format!(
        "{name:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
        0,
        0,
        0,
        "100644",
        bytes.len()
    );
    assert_eq!(header.len(), 60);
    archive.extend_from_slice(header.as_bytes());
    archive.extend_from_slice(bytes);
    if bytes.len() % 2 != 0 {
        archive.push(b'\n');
    }
}

#[test]
fn archive_members_keep_the_vendored_database_and_leave_no_staging_files() {
    let scratch = tempfile::tempdir().unwrap();
    let staging = scratch.path().join("staging");
    std::fs::create_dir(&staging).unwrap();
    let object = std::fs::read(common::fixture("fid_lib_x86_64.o")).unwrap();
    let expected = std::fs::read(common::fixture("fid_lib_x86_64.fid")).unwrap();
    let mut archive = b"!<arch>\n".to_vec();
    archive_member(&mut archive, "first.o/", &object);
    archive_member(&mut archive, "second.o/", &object);
    archive_member(&mut archive, "readme/", b"not an object");
    let input = scratch.path().join("library.a");
    let output = scratch.path().join("library.fid");
    std::fs::write(&input, archive).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "build"])
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .args(["--lang", "x86:LE:64:default", "--cspec", "gcc"])
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .env("TMPDIR", &staging)
        .env("TMP", &staging)
        .env("TEMP", &staging)
        .output()
        .expect("run kuna fid build");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8_lossy(&result.stderr).contains("6 record(s)"));
    assert_eq!(std::fs::read(&output).unwrap(), expected);
    let query = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "query"]).arg(common::repo_root().join("tests/fixtures/fid/prog"))
        .arg("--db").arg(&output).args(["--addr", "0x401810", "--json"])
        .env("SLEIGHHOME", common::repo_root().join("specs")).output().unwrap();
    assert!(query.status.success(), "{}", String::from_utf8_lossy(&query.stderr));
    let result: serde_json::Value = serde_json::from_slice(&query.stdout).unwrap();
    assert!(result["candidates"].as_array().unwrap().iter().any(|c| c["name"] == "kuna_strlen"));
    let records = kuna_analysis::fid::db::FidDb::load(&std::fs::read(output).unwrap()).unwrap();
    let mut names: Vec<_> = records.records().iter().map(|record| record.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(names, ["kuna_crc32", "kuna_memset", "kuna_strlen"]);
    assert!(std::fs::read_dir(staging).unwrap().next().is_none());
}

#[test]
fn an_archive_no_member_of_which_the_language_describes_is_refused() {
    let scratch = tempfile::tempdir().unwrap();
    let object = std::fs::read(common::fixture("fid_lib_x86_64.o")).unwrap();
    let mut archive = b"!<arch>\n".to_vec();
    archive_member(&mut archive, "first.o/", &object);
    archive_member(&mut archive, "second.o/", &object);
    let input = scratch.path().join("library.a");
    let output = scratch.path().join("library.fid");
    std::fs::write(&input, archive).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "build"])
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .args(["--lang", "AARCH64:LE:64:v8A", "--cspec", "default"])
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert_eq!(result.status.code(), Some(1), "{stderr}");
    assert_eq!(stderr.matches("skipped: --lang AARCH64:LE:64:v8A does not describe this object").count(), 2, "{stderr}");
    assert!(stderr.contains("--lang AARCH64:LE:64:v8A describes none of its object members (2 refused)"), "{stderr}");
    assert!(!output.exists());
}

#[test]
fn query_reports_candidates_collisions_and_invalid_databases() {
    use kuna_analysis::fid::db::FidDb;
    let root = common::repo_root();
    let binary = root.join("tests/fixtures/fid/prog");
    let fixture = root.join("tests/fixtures/fid/lib.fid");
    let scratch = tempfile::tempdir().unwrap();
    let invoke = |db: &std::path::Path| {
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["fid", "query"]).arg(&binary).arg("--db").arg(db)
            .args(["--addr", "0x4017c0", "--json"])
            .env("SLEIGHHOME", root.join("specs"))
            .output().unwrap()
    };
    let output = invoke(&fixture);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["status"], "single_candidate");
    assert_eq!(value["identity_verified"], false);
    assert_eq!(value["entry_source"], "discovered");
    assert_eq!(value["instruction_boundary"], "instruction_start");
    assert_eq!(value["match_basis"], "full_hash");
    assert_eq!(value["candidates"][0]["name"], "kuna_crc32");
    assert_eq!(value["candidates"][0]["specific_hash_matches"], false);
    assert_eq!(value["specific_hash_evidence"], "no_candidate_matches");
    assert_eq!(value["candidates"][0]["code_unit_size_matches"], true);
    let batch = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "query"]).arg(&binary).arg("--db").arg(&fixture)
        .args(["--addr", "0x4017c0", "--addr", "0x0", "--addr", "0x4017c0", "--json"])
        .env("SLEIGHHOME", root.join("specs")).output().unwrap();
    assert!(batch.status.success(), "{}", String::from_utf8_lossy(&batch.stderr));
    let batch: serde_json::Value = serde_json::from_slice(&batch.stdout).unwrap();
    assert_eq!(batch["queries"][0], value);
    assert_eq!(batch["queries"][2], value);
    let addresses = ["0x4017c0", "0x401810", "0x401840", "0x401870", "0x4017c1"];
    let mut multi = Command::new(env!("CARGO_BIN_EXE_kuna"));
    multi.args(["fid", "query"]).arg(&binary).arg("--db").arg(&fixture);
    for address in addresses { multi.args(["--addr", address]); }
    let output = multi.arg("--json").env("SLEIGHHOME", root.join("specs")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    for (index, address) in addresses.iter().enumerate() {
        let single = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["fid", "query"]).arg(&binary).arg("--db").arg(&fixture)
            .args(["--addr", address, "--json"]).env("SLEIGHHOME", root.join("specs"))
            .output().unwrap();
        assert!(single.status.success());
        let single: serde_json::Value = serde_json::from_slice(&single.stdout).unwrap();
        assert_eq!(result["queries"][index], single);
        if *address == "0x4017c1" {
            assert_eq!(single["entry_source"], "user_supplied");
            assert_eq!(single["instruction_boundary"], "inside_instruction");
        }
    }
    assert!(batch["queries"][1]["status"] == "error" || batch["queries"][1]["status"] == "unavailable");
    assert_eq!(batch["queries"][1]["entry_source"], "user_supplied");
    assert_eq!(batch["queries"][1]["instruction_boundary"], "non_executable");
    assert_eq!(batch["queries"][1]["identity_verified"], false);


    let db = FidDb::load(&std::fs::read(fixture).unwrap()).unwrap();
    let mut records: Vec<_> = db.records().into_iter().cloned().collect();
    let mut collision = records.iter().find(|r| r.name == "kuna_crc32").unwrap().clone();
    collision.name = "other_candidate".into();
    records.push(collision);
    let path = scratch.path().join("collision.fid");
    std::fs::write(&path, FidDb::from_records("x86:LE:64:default", "gcc", records).serialize()).unwrap();
    let output = invoke(&path);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["status"], "ambiguous");
    assert_eq!(value["candidates"].as_array().unwrap().len(), 2);
    assert_eq!(value["candidates"][1]["name"], "other_candidate");

    std::fs::write(&path, FidDb::from_records("ARM:LE:32:v7", "gcc", vec![]).serialize()).unwrap();
    let output = invoke(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("database language"));

    std::fs::write(&path, FidDb::from_records("x86:LE:64:default", "gcc", vec![]).serialize()).unwrap();
    let output = invoke(&path);
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["status"], "no_match");
    assert_eq!(value["candidates"].as_array().unwrap().len(), 0);

    std::fs::write(&path, b"not a FID database").unwrap();
    let output = invoke(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid database"));
}

#[test]
fn thumb_pointers_use_the_discovered_mode_and_isolate_batch_context() {
    use kuna_analysis::fid::db::FidDb;
    use object::{Object, ObjectSection};
    let root = common::repo_root();
    let scratch = tempfile::tempdir().unwrap();
    let mut bytes = std::fs::read(common::fixture("arm_interwork_le32")).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let text = file.section_by_name(".text").unwrap();
    let text_offset = text.file_range().unwrap().0 as usize;
    bytes[text_offset + 0x40..text_offset + 0x4a]
        .copy_from_slice(&[0x07, 0x20, 0x01, 0x30, 0x01, 0x30, 0x01, 0x30, 0x70, 0x47]);
    let binary = scratch.path().join("thumb.elf");
    std::fs::write(&binary, bytes).unwrap();
    let db = scratch.path().join("arm.fid");
    let object = common::fixture("fid_thumb_probe.o");
    let build = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "build"]).arg(&object).arg("-o").arg(&db)
        .args(["--lang", "ARM:LE:32:v8", "--cspec", "default"])
        .env("SLEIGHHOME", root.join("specs")).output().unwrap();
    assert!(build.status.success(), "{}", String::from_utf8_lossy(&build.stderr));
    let parsed = FidDb::load(&std::fs::read(&db).unwrap()).unwrap();
    assert!(parsed.records().iter().any(|record| record.name == "thumb_probe"));
    let call = |addresses: &[&str]| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.args(["fid", "query"]).arg(&binary).arg("--db").arg(&db);
        for address in addresses { command.args(["--addr", address]); }
        let output = command.arg("--json").env("SLEIGHHOME", root.join("specs")).output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
    };
    let even = call(&["0x2000040"]);
    let odd = call(&["0x2000041"]);
    assert_eq!(even["status"], "single_candidate");
    assert_eq!(even["candidates"][0]["name"], "thumb_probe");
    assert_eq!(even["entry_source"], "discovered");
    assert_eq!(even["code_unit_size"], 5);
    assert_eq!(even["full_hash"], odd["full_hash"]);
    assert_eq!(even["specific_hash"], odd["specific_hash"]);
    assert_eq!(odd["address"], "0x2000040");
    assert_eq!(odd["requested_address"], "0x2000041");
    let batch = call(&["0x2000040", "0x2000041", "0x2000080", "0x2000043", "0x2000040"]);
    assert_eq!(batch["queries"][0], even);
    assert_eq!(batch["queries"][1], odd);
    assert_eq!(batch["queries"][2], call(&["0x2000080"]));
    assert_eq!(batch["queries"][3], call(&["0x2000043"]));
    assert_eq!(batch["queries"][3]["entry_source"], "user_supplied");
    assert_eq!(batch["queries"][4], even);

    std::fs::write(&binary, std::fs::read(common::fixture("arm_interwork_le32")).unwrap()).unwrap();
    let too_short = call(&["0x2000040", "0x2000041"]);
    assert_eq!(too_short["queries"][0]["reason"], "too_short");
    assert_eq!(too_short["queries"][1]["reason"], "too_short");
}

#[test]
fn query_keeps_the_cli_contract_for_addresses_usage_errors_and_closed_pipes() {
    let root = common::repo_root();
    let binary = root.join("tests/fixtures/fid/prog");
    let db = root.join("tests/fixtures/fid/lib.fid");
    let kuna = |args: &[&str]| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.args(["fid", "query"]).arg(&binary).args(args).env("SLEIGHHOME", root.join("specs"));
        command
    };
    let query = |address: &str| {
        let out = kuna(&["--db", db.to_str().unwrap(), "--addr", address, "--json"]).output().unwrap();
        assert!(out.status.success(), "{address}: {}", String::from_utf8_lossy(&out.stderr));
        serde_json::from_slice::<serde_json::Value>(&out.stdout).unwrap()
    };
    let bare = query("4017c0");
    assert_eq!(bare["status"], "single_candidate");
    assert_eq!(bare, query("0x4017c0"));

    for args in [
        &["--db", db.to_str().unwrap(), "--json"][..],
        &["--db", db.to_str().unwrap(), "--addr", "0x4017c0"],
        &["--db", db.to_str().unwrap(), "--addr", "zz", "--json"],
        &["--addr", "0x4017c0", "--json", "--bogus"],
    ] {
        let out = kuna(args).output().unwrap();
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(String::from_utf8_lossy(&out.stderr).contains("usage: kuna fid query"));
    }

    let mut child = kuna(&["--db", db.to_str().unwrap(), "--addr", "0x4017c0", "--addr", "0x401810", "--json"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let closed = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&closed.stderr);
    assert!(!stderr.contains("panicked") && !stderr.contains("Broken pipe"), "{stderr}");
    assert!(closed.status.success(), "{:?}: {stderr}", closed.status.code());
}
