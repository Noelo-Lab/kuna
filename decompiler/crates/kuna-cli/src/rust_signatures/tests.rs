use super::*;
use kuna_analysis::flirt::kuna_rustprofiles::{CompilerProvenance, Report};

const TINY: &str = "49444153474e0600000000000000000000000000000000000000000000000000000007000001000000746573746c6962010400554889e50004bc1628006d7966756e6300";

fn inputs() -> (tempfile::TempDir, PathBuf, Value) {
    let temp = tempfile::tempdir().unwrap();
    let bytes: Vec<u8> = (0..TINY.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&TINY[i..i + 2], 16).unwrap())
        .collect();
    std::fs::write(temp.path().join("inline.sig"), &bytes).unwrap();
    std::fs::write(temp.path().join("no-inline.sig"), &bytes).unwrap();
    let value = json!({
        "corpus": {"identity": "authored-static", "pin": "test-v1"},
        "candidates": [{"version": "1.88.0", "optimization": 3}],
        "signatures": [
            {"path": "inline.sig", "sha256": sha256(&bytes), "version": "1.88.0", "optimization": 3, "bank": "inline"},
            {"path": "no-inline.sig", "sha256": sha256(&bytes), "version": "1.88.0", "optimization": 3, "bank": "no-inline"}
        ]
    });
    let path = temp.path().join("manifest.json");
    std::fs::write(&path, value.to_string()).unwrap();
    (temp, path, value)
}

fn rejected(path: &Path, value: &Value, expected: &str) {
    std::fs::write(path, value.to_string()).unwrap();
    let error = load(path).err().expect("invalid manifest was accepted");
    assert!(error.contains(expected), "{error}");
}

#[test]
fn verifies_relative_sources_and_retains_exact_hashes() {
    let (_temp, path, _) = inputs();
    let (request, info) = load(&path).unwrap();
    assert_eq!(request.candidates, vec![profile("1.88.0", 3).unwrap()]);
    assert_eq!(request.signatures.len(), 2);
    assert_eq!(info.manifest_sha256, sha256(&std::fs::read(&path).unwrap()));
    assert!(Path::new(&info.signatures[0].path).is_absolute());
    assert_eq!(
        info.signatures[0].sha256,
        sha256(&std::fs::read(&info.signatures[0].path).unwrap())
    );
}

#[test]
fn rejects_unknown_and_duplicate_json_fields() {
    let (_temp, path, mut value) = inputs();
    value["unexpected"] = json!(true);
    rejected(&path, &value, "unknown field");
    value.as_object_mut().unwrap().remove("unexpected");
    value["candidates"][0]["unexpected"] = json!(true);
    rejected(&path, &value, "unknown field");
    let text = r#"{"corpus":{"identity":"x","pin":"a","pin":"b"},"candidates":[],"signatures":[]}"#;
    std::fs::write(&path, text).unwrap();
    assert!(load(&path).err().unwrap().contains("duplicate field"));
}

#[test]
fn rejects_unrequested_and_duplicate_manifest_entries() {
    let (_temp, path, value) = inputs();
    let mut changed = value.clone();
    changed["signatures"][0]["version"] = json!("1.89.0");
    rejected(&path, &changed, "explicit candidate");
    changed = value.clone();
    changed["candidates"]
        .as_array_mut()
        .unwrap()
        .push(value["candidates"][0].clone());
    rejected(&path, &changed, "duplicate Rust signature candidate");
    changed = value.clone();
    changed["signatures"][1]["bank"] = json!("inline");
    rejected(&path, &changed, "duplicate Rust signature bank");
    changed = value;
    changed["signatures"][1]["path"] = json!("./inline.sig");
    rejected(&path, &changed, "duplicate Rust signature path");
}

#[test]
fn rejects_bad_hashes_versions_banks_and_unreadable_sources() {
    let (_temp, path, value) = inputs();
    let mut changed = value.clone();
    changed["signatures"][0]["sha256"] = json!("0".repeat(64));
    rejected(&path, &changed, "SHA-256 mismatch");
    changed["signatures"][0]["sha256"] = json!("A".repeat(64));
    rejected(&path, &changed, "lowercase SHA-256");
    changed = value.clone();
    changed["signatures"][0]["path"] = json!("missing.sig");
    rejected(&path, &changed, "missing.sig");
    changed = value;
    changed["signatures"][0]["bank"] = json!("unknown");
    rejected(&path, &changed, "unknown variant");
    for version in [
        "1.88",
        "1.88.0-nightly",
        "1.088.0",
        "1.88.65536",
        "unknown",
        "1.-1.0",
    ] {
        assert!(profile(version, 3).is_err(), "{version}");
    }
    assert!(profile("1.88.0", 4).is_err());
}

#[test]
fn omitted_banks_are_reportable_but_candidates_must_be_explicit() {
    let (_temp, path, mut value) = inputs();
    value["signatures"] = json!([]);
    std::fs::write(&path, value.to_string()).unwrap();
    assert!(load(&path).unwrap().0.signatures.is_empty());
    value["candidates"] = json!([]);
    rejected(&path, &value, "requires explicit candidates");
}

fn rendered(info: &ManifestInfo, image: Option<&ImageReport>) -> Value {
    serde_json::from_str(&crate::jsonfmt::dumps_indent2(&Json::Object(fields(
        info, image,
    ))))
    .unwrap()
}

#[test]
fn compiler_records_and_profile_selection_are_independent() {
    let (_temp, path, _) = inputs();
    let (_, info) = load(&path).unwrap();
    let old = profile("1.88.0", 3).unwrap();
    let new = profile("1.89.0", 3).unwrap();
    let mut image = ImageReport {
        compiler: CompilerProvenance {
            rustc_records: vec!["rustc version 1.89.0-nightly (test 2025-05-21)".into()],
            source_commits: vec!["bf64d66bd58719fac2585eae5e546e5e1d9947f5".into()],
        },
        report: Some(Report {
            candidates: Vec::new(),
            selection: Selection::Ambiguous(vec![old, new]),
        }),
        error: None,
        entry_count: 7,
    };
    let value = rendered(&info, Some(&image));
    assert_eq!(value["rust_signature_profile"]["status"], "ambiguous");
    assert!(value["rust_signature_profile"]["selected_profile"].is_null());
    assert_eq!(
        value["rust_signature_profile"]["tied_profiles"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        value["compiler_provenance"]["rustc_records"][0],
        image.compiler.rustc_records[0]
    );
    image.report.as_mut().unwrap().selection = Selection::Unique(old);
    let value = rendered(&info, Some(&image));
    assert_eq!(
        value["rust_signature_profile"]["selected_profile"]["version"],
        "1.88.0"
    );
    assert_eq!(
        value["compiler_provenance"]["rustc_records"][0],
        image.compiler.rustc_records[0]
    );
    assert_eq!(
        value["rust_signature_profile"]["provenance"]["manifest_sha256"],
        info.manifest_sha256
    );
}

#[test]
fn reports_missing_zero_and_unsupported_results_without_a_selected_profile() {
    let (_temp, path, _) = inputs();
    let (_, info) = load(&path).unwrap();
    let mut image = ImageReport {
        compiler: CompilerProvenance {
            rustc_records: Vec::new(),
            source_commits: Vec::new(),
        },
        report: None,
        error: Some("unsupported_image".into()),
        entry_count: 0,
    };
    let value = rendered(&info, Some(&image));
    assert_eq!(value["rust_signature_profile"]["status"], "unsupported");
    assert!(value["rust_signature_profile"]["selected_profile"].is_null());
    for (selection, status) in [
        (Selection::NoCandidates, "no_candidates"),
        (Selection::NoMatches, "no_matches"),
        (
            Selection::Incomplete(vec![profile("1.88.0", 3).unwrap()]),
            "incomplete",
        ),
    ] {
        image.error = None;
        image.report = Some(Report {
            candidates: Vec::new(),
            selection,
        });
        let value = rendered(&info, Some(&image));
        assert_eq!(value["rust_signature_profile"]["status"], status);
        assert!(value["rust_signature_profile"]["selected_profile"].is_null());
    }
    assert_eq!(
        rendered(&info, None)["rust_signature_profile"]["status"],
        "unavailable"
    );
}
