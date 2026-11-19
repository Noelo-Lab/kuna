//! Authored ELF/signature data is read statically; only Kuna is executed.

use crate::common;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const COMMENT: &[u8] = b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)\0";
const TINY: &str = "49444153474e0600000000000000000000000000000000000000000000000000000007000001000000746573746c6962010400554889e50004bc1628006d7966756e6300";

fn fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let binary = temp.path().join("authored.elf");
    let mut bytes = vec![0u8; 0x300];
    bytes[..16].copy_from_slice(b"\x7fELF\x02\x01\x01\0\0\0\0\0\0\0\0\0");
    for (offset, value) in [
        (16, 2u16),
        (18, 62),
        (52, 64),
        (54, 56),
        (56, 1),
        (58, 64),
        (60, 4),
        (62, 3),
    ] {
        bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (offset, value) in [
        (20, 1u32),
        (64, 1),
        (68, 5),
        (0x240, 1),
        (0x244, 1),
        (0x280, 7),
        (0x284, 1),
        (0x2c0, 16),
        (0x2c4, 3),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let names = b"\0.text\0.comment\0.shstrtab\0";
    for (offset, value) in [
        (24, 0x400100u64),
        (32, 64),
        (40, 0x200),
        (80, 0x400000),
        (88, 0x400000),
        (96, 0x128),
        (104, 0x128),
        (112, 0x1000),
        (0x248, 6),
        (0x250, 0x400100),
        (0x258, 0x100),
        (0x260, 40),
        (0x270, 16),
        (0x298, 0x140),
        (0x2a0, COMMENT.len() as u64),
        (0x2b0, 1),
        (0x2d8, 0x180),
        (0x2e0, names.len() as u64),
        (0x2f0, 1),
    ] {
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    bytes[0x100..0x128].fill(0x90);
    bytes[0x100..0x104].copy_from_slice(&[0x55, 0x48, 0x89, 0xe5]);
    bytes[0x126..0x128].copy_from_slice(&[0x5d, 0xc3]);
    bytes[0x140..0x140 + COMMENT.len()].copy_from_slice(COMMENT);
    bytes[0x180..0x180 + names.len()].copy_from_slice(names);
    std::fs::write(&binary, bytes).unwrap();
    let mut signature: Vec<u8> = (0..TINY.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&TINY[i..i + 2], 16).unwrap())
        .collect();
    let at = signature
        .windows(7)
        .position(|window| window == b"myfunc\0")
        .unwrap();
    signature.splice(
        at..at + 7,
        b"_ZN4core3ptr4read17h0123456789abcdefE\0".iter().copied(),
    );
    std::fs::write(temp.path().join("inline.sig"), &signature).unwrap();
    std::fs::write(temp.path().join("no-inline.sig"), &signature).unwrap();
    let hash: String = Sha256::digest(&signature)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let manifest = temp.path().join("manifest.json");
    std::fs::write(&manifest, json!({
        "corpus": {"identity": "authored-static", "pin": "test-v1"},
        "candidates": [{"version": "1.88.0", "optimization": 3}],
        "signatures": [
            {"path": "inline.sig", "sha256": hash, "version": "1.88.0", "optimization": 3, "bank": "inline"},
            {"path": "no-inline.sig", "sha256": hash, "version": "1.88.0", "optimization": 3, "bank": "no-inline"}
        ]
    }).to_string()).unwrap();
    (temp, binary, manifest)
}

fn summary(binary: &std::path::Path, extra: &[&str]) -> Value {
    let mut args = vec![
        "functions",
        binary.to_str().unwrap(),
        "--summary",
        "--json",
        "--option",
        "listing",
        "on",
    ];
    args.extend_from_slice(extra);
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}");
    serde_json::from_str(&stdout).unwrap()
}

#[test]
fn requested_profiles_preserve_normal_summary_and_ignore_summary_filters() {
    let (_temp, binary, manifest) = fixture();
    for listing in ["on", "off"] {
        let plain = summary(&binary, &["--option", "listing", listing]);
        assert!(plain.get("compiler_provenance").is_none());
        assert!(plain.get("rust_signature_profile").is_none());
        let flag = ["--rust-signature-manifest", manifest.to_str().unwrap()];
        let mut requested = summary(&binary, &[flag[0], flag[1], "--option", "listing", listing]);
        assert_eq!(requested["rust_signature_profile"]["status"], "unique");
        assert_eq!(
            requested["rust_signature_profile"]["selected_profile"]["version"],
            "1.88.0"
        );
        assert_eq!(
            requested["compiler_provenance"]["rustc_records"][0],
            std::str::from_utf8(&COMMENT[..COMMENT.len() - 1]).unwrap()
        );
        let narrowed = summary(
            &binary,
            &[
                flag[0],
                flag[1],
                "--limit",
                "0",
                "--filter",
                "does_not_match",
                "--option",
                "listing",
                listing,
            ],
        );
        assert_eq!(
            requested["rust_signature_profile"],
            narrowed["rust_signature_profile"]
        );
        let object = requested.as_object_mut().unwrap();
        object.remove("compiler_provenance");
        object.remove("rust_signature_profile");
        assert_eq!(
            plain, requested,
            "report request changed inventory, names, or normal summary"
        );
    }
}

#[test]
fn image_report_survives_second_commit_without_changing_analysis_flags() {
    use kuna_analysis::flirt::kuna_rustprofiles::{Profile, Request, Selection};
    let (_temp, binary, _) = fixture();
    let specs = common::repo_root().join("specs").display().to_string();
    for listing in [false, true] {
        let mut program = kuna_console::engine::bootstrap_from_object(
            binary.to_str().unwrap(),
            "",
            &[specs.clone()],
        )
        .unwrap();
        program.arch_mut().analysis_listing = listing;
        program.arch_mut().analysis_flirt = false;
        program.request_rust_profiles(Request {
            candidates: vec![Profile {
                version: (1, 88, 0),
                optimization: 3,
            }],
            signatures: Vec::new(),
        });
        program.commit_pending_analysis().unwrap();
        let first = program
            .rust_profile_report()
            .cloned()
            .expect("requested image report");
        let report = first
            .report
            .as_ref()
            .expect("observer must have scored the accepted Listing");
        assert!(
            first.entry_count > 0,
            "observer saw no function entries with listing={listing}"
        );
        assert_eq!(
            report.selection,
            Selection::Incomplete(vec![Profile {
                version: (1, 88, 0),
                optimization: 3
            }])
        );
        assert!(first.error.is_none(), "{first:?}");
        assert_eq!(program.arch().analysis_listing, listing);
        assert!(!program.arch().analysis_flirt);
        program.commit_pending_analysis().unwrap();
        assert_eq!(program.rust_profile_report(), Some(&first));
        assert_eq!(program.arch().analysis_listing, listing);
        assert!(!program.arch().analysis_flirt);
    }
}
