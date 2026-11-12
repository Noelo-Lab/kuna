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
    assert_eq!(std::fs::read(output).unwrap(), expected);
    assert!(std::fs::read_dir(staging).unwrap().next().is_none());
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
    assert_eq!(value["status"], "unique");
    assert_eq!(value["match_basis"], "full_hash");
    assert_eq!(value["candidates"][0]["name"], "kuna_crc32");
    assert_eq!(value["candidates"][0]["specific_hash_matches"], false);
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
    }
    assert!(batch["queries"][1]["status"] == "error" || batch["queries"][1]["status"] == "unavailable");


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
fn query_reads_user_supplied_flirt_sig_and_reports_collisions() {
    use object::{Object, ObjectSection};
    use std::io::Write;
    let root = common::repo_root();
    let binary = root.join("tests/fixtures/fid/prog");
    let bytes = std::fs::read(&binary).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let text = file.section_by_name(".text").unwrap();
    let offset = (0x4017c0 - text.address()) as usize;
    let function_bytes = &text.data().unwrap()[offset..offset + 64];
    let prefix = &function_bytes[..32];
    let crc = {
        let mut value: u16 = 0xffff;
        for byte in &function_bytes[32..36] {
            value ^= u16::from(*byte);
            for _ in 0..8 {
                value = if value & 1 == 1 { (value >> 1) ^ 0x8408 } else { value >> 1 };
            }
        }
        (!value).swap_bytes()
    };
    let mut header = vec![0u8; 45];
    header[..6].copy_from_slice(b"IDASGN");
    header[6] = 10;
    header[41] = 32;
    let mut body = vec![1, 32, 0];
    body.extend_from_slice(prefix);
    body.extend_from_slice(&[0, 4]);
    body.extend_from_slice(&crc.to_be_bytes());
    body.extend_from_slice(&[32, 0]);
    body.extend_from_slice(b"first_candidate");
    body.extend_from_slice(&[14, 1, 1, function_bytes[37], 1, 2, 3]);
    body.extend_from_slice(b"foo");
    body.extend_from_slice(&[32, 0]);
    body.extend_from_slice(b"second_candidate");
    body.push(0);
    let scratch = tempfile::tempdir().unwrap();
    let path = scratch.path().join("test.sig");
    let invoke = || Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "query"]).arg(&binary).arg("--db").arg(&path)
        .args(["--addr", "0x4017c0", "--json"]).output().unwrap();
    let mut plain = header.clone();
    plain.extend_from_slice(&body);
    std::fs::write(&path, &plain).unwrap();
    let output = invoke();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["database_format"], "flirt_sig");
    assert_eq!(value["status"], "ambiguous");
    assert_eq!(value["candidates"][0]["name"], "first_candidate");
    assert_eq!(value["candidates"][1]["name"], "second_candidate");
    assert_eq!(value["reference_validation"], "not_checked");
    assert_eq!(value["candidates"][0]["reference_constraints"][0]["name"], "foo");
    assert_eq!(value["architecture_validation"], "not_checked");
    let batch = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "query"]).arg(&binary).arg("--db").arg(&path)
        .args(["--addr", "0x4017c0", "--addr", "0x0", "--addr", "0x4017c0", "--json"])
        .output().unwrap();
    assert!(batch.status.success(), "{}", String::from_utf8_lossy(&batch.stderr));
    let batch: serde_json::Value = serde_json::from_slice(&batch.stdout).unwrap();
    assert_eq!(batch["queries"][0], value);
    assert_eq!(batch["queries"][1]["status"], "error");
    assert_eq!(batch["queries"][2], value);


    let mut bad_crc = plain.clone();
    bad_crc[45 + 3 + 32 + 2] ^= 1;
    std::fs::write(&path, bad_crc).unwrap();
    let output = invoke();
    assert!(output.status.success());
    let no_match: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(no_match["status"], "no_match");

    let mut bad_tail = plain.clone();
    let tail_value = bad_tail.windows(b"first_candidate".len()).position(|w| w == b"first_candidate").unwrap() + b"first_candidate".len() + 3;
    bad_tail[tail_value] ^= 1;
    std::fs::write(&path, bad_tail).unwrap();
    let output = invoke();
    assert!(output.status.success());
    let tail_filtered: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(tail_filtered["status"], "unique");
    assert_eq!(tail_filtered["candidates"][0]["name"], "second_candidate");

    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&body).unwrap();
    header[16] = 0x10;
    header.extend(encoder.finish().unwrap());
    std::fs::write(&path, header).unwrap();
    let compressed = invoke();
    assert!(compressed.status.success(), "{}", String::from_utf8_lossy(&compressed.stderr));
    let compressed: serde_json::Value = serde_json::from_slice(&compressed.stdout).unwrap();
    assert_eq!(compressed["candidates"], value["candidates"]);

    plain[6] = 11;
    std::fs::write(&path, plain).unwrap();
    let output = invoke();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported FLIRT version"));
}

#[test]
fn flirt_lookup_reports_a_truncated_byte_window() {
    use object::write::{Object, StandardSection};
    let scratch = tempfile::tempdir().unwrap();
    let mut object = Object::new(object::BinaryFormat::Elf, object::Architecture::X86_64, object::Endianness::Little);
    let section = object.section_id(StandardSection::Text);
    object.append_section_data(section, &vec![0x90; 1024 * 1024 + 1], 1);
    let mut binary = object.write().unwrap();
    binary[16] = 2;
    let path = scratch.path().join("large.elf");
    std::fs::write(&path, binary).unwrap();
    let mut sig = vec![0u8; 45];
    sig[..6].copy_from_slice(b"IDASGN");
    sig[6] = 10;
    sig[41] = 32;
    sig.extend_from_slice(&[1, 32, 0]);
    sig.extend_from_slice(&[0x90; 32]);
    sig.extend_from_slice(&[0, 0, 0, 0, 32, 0]);
    sig.extend_from_slice(b"candidate");
    sig.push(0);
    let db = scratch.path().join("test.sig");
    std::fs::write(&db, sig).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "query"]).arg(path).arg("--db").arg(db)
        .args(["--addr", "0x0", "--json"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["byte_window_limited"], true);
    assert_eq!(value["status"], "incomplete");
    assert_eq!(value["candidates"][0]["name"], "candidate");
}
