//! Authored ELF/signature data is read statically; only Kuna is executed.

use crate::common;
use kuna_console::engine::Bootstrap;
use kuna_decomp::kuna_loadoptions::LoadOptions;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const COMMENT: &[u8] = b"rustc version 1.89.0-nightly (bf64d66bd 2025-05-21)\0";
const TINY: &str = "49444153474e0600000000000000000000000000000000000000000000000000000007000001000000746573746c6962010400554889e50004bc1628006d7966756e6300";
const RECOVERED: &str = "_ZN4core3ptr4read17h0123456789abcdefE";
const ENTRY: u64 = 0x400100;

fn fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    fixture_with_comment(COMMENT)
}

fn fixture_with_comment(
    comment: &[u8],
) -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    assert!(comment.len() <= 0x40);
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
        (0x2a0, comment.len() as u64),
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
    bytes[0x140..0x140 + comment.len()].copy_from_slice(comment);
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
        "--option",
        "flirt",
        "off",
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
fn stable_comment_profile_hint_preserves_matches_selection_and_provenance() {
    let cases: &[(&[u8], bool, Option<&str>)] = &[
        (b"rustc version 1.93.0\0", true, Some("exact_version")),
        (b"rustc version 1.93.1\0", true, Some("same_minor")),
        (
            b"rustc version 1.98.1\0",
            true,
            Some("newer_than_available"),
        ),
        (b"rustc version 1.90.1\0", true, None),
        (b"rustc version 2.0.0\0", true, None),
        (b"rustc version 1.93.1\0rustc version 1.98.1\0", true, None),
        (COMMENT, true, None),
        (b"rustc version 1.93.1\0", false, None),
    ];
    for &(comment, pair, basis) in cases {
        let (temp, binary, manifest) = fixture_with_comment(comment);
        let mut inputs: Value = serde_json::from_slice(&std::fs::read(&manifest).unwrap()).unwrap();
        let candidate = json!({"version": "1.93.0", "optimization": 3});
        inputs["candidates"].as_array_mut().unwrap().push(candidate);
        for (index, bank) in ["inline", "no-inline"].iter().enumerate() {
            if !pair && index == 1 {
                break;
            }
            let mut signature = inputs["signatures"][index].clone();
            let name = format!("second-{bank}.sig");
            std::fs::copy(
                temp.path().join(format!("{bank}.sig")),
                temp.path().join(&name),
            )
            .unwrap();
            signature["path"] = json!(name);
            signature["version"] = json!("1.93.0");
            inputs["signatures"].as_array_mut().unwrap().push(signature);
        }
        std::fs::write(&manifest, inputs.to_string()).unwrap();

        let result = summary(
            &binary,
            &["--rust-signature-manifest", manifest.to_str().unwrap()],
        );
        let report = &result["rust_signature_profile"];
        assert!(report["selected_profile"].is_null());
        let records: Vec<_> = comment
            .split(|&byte| byte == 0)
            .filter(|record| !record.is_empty())
            .map(|record| std::str::from_utf8(record).unwrap())
            .collect();
        assert_eq!(
            result["compiler_provenance"]["rustc_records"],
            json!(records)
        );
        if pair {
            assert_eq!(report["status"], "ambiguous");
            assert_eq!(report["tied_profiles"].as_array().unwrap().len(), 2);
            assert_eq!(report["candidates"][0]["score"], 1);
            assert_eq!(report["candidates"][1]["score"], 1);
        } else {
            assert_eq!(report["status"], "incomplete");
        }
        match basis {
            Some(basis) => assert_eq!(
                report["comment_profile_hint"],
                json!({
                    "profile": {"version":"1.93.0", "optimization":3},
                    "basis": basis,
                })
            ),
            None => assert!(report.get("comment_profile_hint").is_none()),
        }
    }
}

#[test]
fn image_report_survives_second_commit_without_changing_analysis_flags() {
    use kuna_analysis::flirt::kuna_rustprofiles::{Profile, Request, Selection};
    let (_temp, binary, _) = fixture();
    let specs = common::repo_root().join("specs").display().to_string();
    for listing in [false, true] {
        let mut program = Bootstrap::new(&[specs.clone()])
            .options(&LoadOptions::default())
            .object(binary.to_str().unwrap())
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

fn invoke(args: &[&str]) -> (String, String, i32) {
    let mut args = args.to_vec();
    args.extend(["--option", "flirt", "off", "--language", "c"]);
    common::run_kuna(&args)
}

fn document(args: &[&str]) -> Value {
    let (stdout, stderr, status) = invoke(args);
    assert_eq!(status, 0, "{args:?}: {stderr}\n{stdout}");
    serde_json::from_str(&stdout).unwrap()
}

fn entry(document: &Value) -> &Value {
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["address"] == ENTRY)
        .expect("the authored entry remains in the inventory")
}

fn refuse(args: &[&str], diagnostic: &[&str]) {
    let (stdout, stderr, status) = invoke(args);
    assert_ne!(
        status, 0,
        "request was silently accepted: {args:?}\n{stdout}"
    );
    let message = format!("{stderr}\n{stdout}").to_ascii_lowercase();
    assert!(
        diagnostic.iter().any(|word| message.contains(word)),
        "missing refusal reason {diagnostic:?}: {args:?}\n{message}"
    );
    if let Ok(document) = serde_json::from_str::<Value>(&stdout) {
        assert!(
            document["functions"]
                .as_array()
                .is_none_or(|rows| { rows.iter().all(|row| row["name"] != RECOVERED) }),
            "a refused request emitted an applied name: {document}"
        );
    }
}

#[test]
fn application_names_the_same_existing_entry_on_every_single_function_surface() {
    let (_temp, binary, manifest) = fixture();
    let binary = binary.to_str().unwrap();
    let manifest = manifest.to_str().unwrap();
    let plain = document(&["functions", binary, "--json"]);
    let placeholder = entry(&plain)["name"].as_str().unwrap();
    assert!(
        ["sub_", "FUN_", "func_"]
            .iter()
            .any(|prefix| placeholder.starts_with(prefix)),
        "{plain}"
    );
    refuse(
        &["decompile", binary, RECOVERED, "--json"],
        &["no function", "not found"],
    );

    let applied = document(&[
        "functions",
        binary,
        "--json",
        "--apply-rust-signatures",
        manifest,
    ]);
    assert_eq!(entry(&applied)["name"], RECOVERED);
    let addresses = |doc: &Value| {
        doc["functions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["address"].as_u64().unwrap())
            .collect::<Vec<_>>()
    };
    assert_eq!(addresses(&plain), addresses(&applied));
    let (text, stderr, status) =
        invoke(&["functions", binary, "--apply-rust-signatures", manifest]);
    assert_eq!(status, 0, "{stderr}");
    assert!(text.contains(RECOVERED), "{text}");
    let summary = document(&[
        "functions",
        binary,
        "--summary",
        "--json",
        "--apply-rust-signatures",
        manifest,
    ]);
    assert!(
        summary["summary"]["largest"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["address"] == ENTRY && row["name"] == RECOVERED),
        "{summary}"
    );

    let mut first: Option<String> = None;
    for target in [RECOVERED, "0x400100"] {
        let result = document(&[
            "decompile",
            binary,
            target,
            "--json",
            "--apply-rust-signatures",
            manifest,
            "--max-fn-seconds",
            "120",
            "--max-fn-memory",
            "0",
        ]);
        let function = entry(&result);
        assert_eq!(function["name"], RECOVERED);
        assert!(function["error"].is_null(), "{result}");
        let code = function["code"].as_str().unwrap();
        if let Some(first) = first.as_deref() {
            assert_eq!(code, first);
        } else {
            first = Some(code.to_owned());
        }
        let (text, stderr, status) = invoke(&[
            "decompile",
            binary,
            target,
            "--apply-rust-signatures",
            manifest,
            "--max-fn-seconds",
            "120",
            "--max-fn-memory",
            "0",
        ]);
        assert_eq!(status, 0, "{target}: {stderr}");
        assert_eq!(
            text.trim(),
            code.trim(),
            "text and JSON differ for {target}"
        );
    }
}

#[test]
fn application_refuses_ambiguous_incomplete_disagreeing_and_unsupported_evidence() {
    for case in ["ambiguous", "incomplete", "disagreeing", "unsupported"] {
        let (temp, binary, manifest) = fixture();
        let mut data: Value = serde_json::from_slice(&std::fs::read(&manifest).unwrap()).unwrap();
        match case {
            "ambiguous" => {
                data["candidates"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"version": "1.87.0", "optimization": 3}));
                let mut signatures = data["signatures"].as_array().unwrap().clone();
                for (index, signature) in signatures.iter_mut().enumerate() {
                    let name = format!("ambiguous-{index}.sig");
                    std::fs::copy(
                        temp.path().join(signature["path"].as_str().unwrap()),
                        temp.path().join(&name),
                    )
                    .unwrap();
                    signature["path"] = json!(name);
                    signature["version"] = json!("1.87.0");
                }
                data["signatures"]
                    .as_array_mut()
                    .unwrap()
                    .extend(signatures);
            }
            "incomplete" => {
                data["signatures"].as_array_mut().unwrap().pop();
            }
            "disagreeing" => {
                let path = temp.path().join("no-inline.sig");
                let mut signature = std::fs::read(&path).unwrap();
                let offset = signature
                    .windows(4)
                    .position(|bytes| bytes == b"read")
                    .unwrap();
                signature[offset..offset + 4].copy_from_slice(b"copy");
                std::fs::write(path, &signature).unwrap();
                let hash: String = Sha256::digest(&signature)
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
                data["signatures"][1]["sha256"] = json!(hash);
            }
            "unsupported" => {
                let mut bytes = std::fs::read(&binary).unwrap();
                bytes[16..18].copy_from_slice(&1u16.to_le_bytes());
                std::fs::write(&binary, bytes).unwrap();
            }
            _ => unreachable!(),
        }
        std::fs::write(&manifest, serde_json::to_vec(&data).unwrap()).unwrap();
        let diagnostic = if case == "disagreeing" {
            vec!["agree", "consensus", "no accepted"]
        } else {
            vec![case]
        };
        let binary = binary.to_str().unwrap();
        let manifest = manifest.to_str().unwrap();
        refuse(
            &[
                "functions",
                binary,
                "--json",
                "--apply-rust-signatures",
                manifest,
            ],
            &diagnostic,
        );
        refuse(
            &[
                "decompile",
                binary,
                "0x400100",
                "--json",
                "--apply-rust-signatures",
                manifest,
            ],
            &diagnostic,
        );
        refuse(
            &[
                "decompile",
                binary,
                "0x400100",
                "--apply-rust-signatures",
                manifest,
            ],
            &diagnostic,
        );
    }
}

#[test]
fn application_refuses_changed_image_and_decoder_views() {
    let (_temp, binary, manifest) = fixture();
    let binary = binary.to_str().unwrap();
    let manifest = manifest.to_str().unwrap();
    let overrides: &[&[&str]] = &[
        &["--assert", "bytes 0x400100 90"],
        &[
            "--raw-image",
            "--base",
            "0x400000",
            "--target",
            "x86:LE:64:default",
        ],
        &["--slice", "x86_64"],
        &["--target", "x86:LE:64:default"],
        &["--isa", "arm"],
        &["--option", "dyldimage", "libSystem.B.dylib"],
        &["--option", "kernelcache", "kernel"],
    ];
    for override_args in overrides {
        for prefix in [
            vec!["functions", binary, "--json"],
            vec!["decompile", binary, "0x400100"],
            vec!["decompile", binary, "0x400100", "--json"],
        ] {
            let mut args = prefix;
            args.extend(["--apply-rust-signatures", manifest]);
            args.extend_from_slice(override_args);
            if args[0] == "functions" && override_args[0] == "--raw-image" {
                args.extend(["--addr", "0x400100"]);
            }
            refuse(&args, &["--apply-rust-signatures"]);
        }
    }
}

#[test]
fn application_refuses_batches_workers_and_console_only_controls() {
    let (temp, binary, manifest) = fixture();
    let binary = binary.to_str().unwrap();
    let manifest = manifest.to_str().unwrap();
    refuse(
        &[
            "functions",
            binary,
            "--apply-rust-signatures",
            manifest,
            "--apply-rust-signatures",
            manifest,
        ],
        &["duplicate --apply-rust-signatures"],
    );
    refuse(
        &[
            "functions",
            binary,
            "--summary",
            "--json",
            "--apply-rust-signatures",
            manifest,
            "--rust-signature-manifest",
            manifest,
        ],
        &["exclusive"],
    );
    for command in ["decompile-all", "decompile-project", "decompile-graph"] {
        refuse(
            &[command, binary, "--apply-rust-signatures", manifest],
            &["--apply-rust-signatures"],
        );
    }
    refuse(
        &[
            "decompile-all",
            binary,
            "--apply-rust-signatures",
            manifest,
            "--jobs-worker",
            temp.path().to_str().unwrap(),
        ],
        &["--apply-rust-signatures"],
    );
    let controls: &[&[&str]] = &[
        &["--raw"],
        &["--regions"],
        &["--kassert", "unused"],
        &["--decomp-dbg", "/not-an-engine"],
        &["--engine", "rust"],
    ];
    for controls in controls {
        let mut args = vec![
            "decompile",
            binary,
            "0x400100",
            "--apply-rust-signatures",
            manifest,
        ];
        args.extend_from_slice(controls);
        refuse(&args, &["--apply-rust-signatures"]);
    }
}

#[test]
fn caller_declaration_and_prototype_outrank_applied_signature_names() {
    let (_temp, binary, manifest) = fixture();
    let binary = binary.to_str().unwrap();
    let manifest = manifest.to_str().unwrap();
    let declare = ["--define-function", "0x400100=caller_selected"];
    let inventory = document(&[
        "functions",
        binary,
        "--json",
        "--apply-rust-signatures",
        manifest,
        declare[0],
        declare[1],
    ]);
    assert_eq!(entry(&inventory)["name"], "caller_selected");
    let extra = [
        "--apply-rust-signatures",
        manifest,
        declare[0],
        declare[1],
        "--assert",
        "prototype caller_selected void caller_selected(void)",
        "--assert-strict",
    ];
    let mut args = vec!["decompile", binary, "caller_selected", "--json"];
    args.extend(extra);
    let result = document(&args);
    let function = entry(&result);
    assert_eq!(function["name"], "caller_selected");
    assert!(function["error"].is_null(), "{result}");
    let code = function["code"].as_str().unwrap();
    assert!(
        code.lines()
            .any(|line| line.trim_start().starts_with("void caller_selected(")),
        "{code}"
    );
    let assertions = result["assertions"].as_array().unwrap();
    assert_eq!(assertions.len(), 1);
    assert_eq!(assertions[0]["status"], "applied");
    let mut args = vec!["decompile", binary, "caller_selected"];
    args.extend(extra);
    let (text, stderr, status) = invoke(&args);
    assert_eq!(status, 0, "{stderr}");
    assert_eq!(text.trim(), code.trim());
}

#[test]
fn application_consumes_one_observation_and_console_rejects_effective_overlays() {
    use kuna_analysis::flirt::kuna_rustprofiles::{
        Bank, Profile, ProfileSignature, Request, Source,
    };
    use kuna_console::assertions::{Body, Directive};
    let (temp, binary, _) = fixture();
    let specs = common::repo_root().join("specs").display().to_string();
    let profile = Profile {
        version: (1, 88, 0),
        optimization: 3,
    };
    for overlay in [false, true] {
        let mut program = Bootstrap::new(&[specs.clone()])
            .options(&LoadOptions::default())
            .object(binary.to_str().unwrap())
            .unwrap();
        program.arch_mut().analysis_flirt = false;
        program.arch_mut().analysis_listing = true;
        if overlay {
            program.set_assertions(vec![Directive {
                raw: "bytes 0x400100 90".into(),
                body: Body::Bytes {
                    addr: ENTRY,
                    data: vec![0x90],
                },
            }]);
            kuna_console::assertions::apply_image_scoped(&mut program);
            assert_eq!(program.assertion_outcomes()[0].status, "applied");
        }
        let signatures = [
            (Bank::Inline, "inline.sig"),
            (Bank::NoInline, "no-inline.sig"),
        ]
        .into_iter()
        .map(|(bank, name)| {
            ProfileSignature::parse(
                Source {
                    profile,
                    bank,
                    identity: name.into(),
                },
                &std::fs::read(temp.path().join(name)).unwrap(),
            )
            .unwrap()
        })
        .collect();
        program.request_rust_signature_names(Request {
            candidates: vec![profile],
            signatures,
        });
        let committed = program.commit_pending_analysis();
        if overlay {
            assert!(committed
                .unwrap_err()
                .explain()
                .contains("unmodified object image"));
            assert!(program.lookup_symbol(RECOVERED).is_none());
        } else {
            committed.unwrap();
            assert_eq!(
                program.lookup_symbol(RECOVERED).unwrap().get_offset(),
                ENTRY
            );
            let observed = program.rust_profile_report().cloned();
            program.commit_pending_analysis().unwrap();
            assert_eq!(program.rust_profile_report(), observed.as_ref());
            assert_eq!(
                program.lookup_symbol(RECOVERED).unwrap().get_offset(),
                ENTRY
            );
        }
    }
}

#[test]
fn application_preserves_independent_compiler_and_registry_crate_fields() {
    let (_temp, binary, manifest) = fixture();
    let mut bytes = std::fs::read(&binary).unwrap();
    bytes.extend_from_slice(b"\0/home/author/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/r01-proof-2.3.4/src/lib.rs\0");
    std::fs::write(&binary, bytes).unwrap();
    let plain = summary(&binary, &[]);
    let reported = summary(
        &binary,
        &["--rust-signature-manifest", manifest.to_str().unwrap()],
    );
    let applied = summary(
        &binary,
        &["--apply-rust-signatures", manifest.to_str().unwrap()],
    );
    let runtime = &plain["summary"]["runtime"];
    let rust = runtime
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "rust")
        .unwrap();
    assert_eq!(rust["version"], "1.89.0-nightly");
    assert_eq!(
        rust["crates"],
        json!([{"name": "r01-proof", "version": "2.3.4"}])
    );
    assert_eq!(reported["summary"]["runtime"], *runtime);
    assert_eq!(applied["summary"]["runtime"], *runtime);
    assert_eq!(
        reported["rust_signature_profile"]["selected_profile"]["version"],
        "1.88.0"
    );
    assert_eq!(
        reported["compiler_provenance"]["rustc_records"][0],
        std::str::from_utf8(&COMMENT[..COMMENT.len() - 1]).unwrap()
    );
    assert!(applied.get("rust_signature_profile").is_none());
    assert!(applied.get("compiler_provenance").is_none());
}

#[test]
fn application_refuses_archive_fanout_and_member_paths_before_loading() {
    let (temp, binary, manifest) = fixture();
    let image = std::fs::read(&binary).unwrap();
    let archive = temp.path().join("linked-images.a");
    let mut bytes = b"!<arch>\n".to_vec();
    for name in ["first.elf/", "second.elf/"] {
        let header = format!(
            "{name:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
            0,
            0,
            0,
            "100644",
            image.len()
        );
        assert_eq!(header.len(), 60);
        bytes.extend_from_slice(header.as_bytes());
        bytes.extend_from_slice(&image);
        if image.len() % 2 != 0 {
            bytes.push(b'\n');
        }
    }
    std::fs::write(&archive, bytes).unwrap();
    let archive = archive.to_str().unwrap();
    let manifest = manifest.to_str().unwrap();
    let plain = document(&["functions", archive, "--member", "--json"]);
    assert_eq!(plain["members"].as_array().unwrap().len(), 2);
    let selected = format!("{archive}:first.elf");
    for mut args in [
        vec!["functions", archive, "--json"],
        vec!["functions", archive, "--summary", "--json"],
        vec!["functions", archive, "--member", "--json"],
        vec!["functions", archive, "--member", "first.elf", "--json"],
        vec!["functions", archive, "--member=first.elf", "--json"],
        vec!["functions", &selected, "--json"],
        vec!["decompile", archive, "0x400100", "--member", "first.elf"],
        vec!["decompile", &selected, "0x400100", "--json"],
    ] {
        args.extend(["--apply-rust-signatures", manifest]);
        refuse(&args, &["standalone linked elf"]);
    }
}

#[test]
fn installed_corpus_check_validates_the_default_and_explicit_paths() {
    let (temp, image, manifest) = fixture();
    let data = temp.path().join("user-data");
    let install_base = if cfg!(target_os = "macos") {
        data.join("Library/Application Support")
    } else {
        data.clone()
    };
    let installed = install_base.join("kuna/rust-signatures/x86_64/linux/manifest.json");
    let specs = common::repo_root().join("specs");
    let run = |args: &[&str], code| {
        let result = std::process::Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("HOME", &data)
            .env("LOCALAPPDATA", &data)
            .env("XDG_DATA_HOME", &data)
            .env("SLEIGHHOME", &specs)
            .env("KUNA_SPECS", &specs)
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(code),
            "{args:?}: {:?}",
            result.stderr
        );
        serde_json::from_slice::<Value>(&result.stdout).unwrap()
    };
    let check = |path: Option<&std::path::Path>, code| {
        let mut args = vec!["rust-signatures", "check", "--json"];
        if let Some(path) = path {
            args.extend(["--manifest", path.to_str().unwrap()]);
        }
        run(&args, code)
    };
    let plain_args = [
        "functions",
        image.to_str().unwrap(),
        "--summary",
        "--json",
        "--option",
        "flirt",
        "off",
    ];
    let mut report_args = plain_args.to_vec();
    report_args.extend(["--rust-signature-manifest", manifest.to_str().unwrap()]);
    assert_eq!(check(None, 1)["status"], "absent");
    assert!(!data.exists());
    let plain = run(&plain_args, 0);
    let report = run(&report_args, 0);
    assert_eq!(report["rust_signature_profile"]["status"], "unique");

    let parent = installed.parent().unwrap();
    std::fs::create_dir_all(parent).unwrap();
    for bank in ["inline.sig", "no-inline.sig"] {
        std::fs::copy(temp.path().join(bank), parent.join(bank)).unwrap();
    }
    std::fs::copy(&manifest, &installed).unwrap();
    let ready = check(None, 0);
    assert_eq!(ready["status"], "ready");
    assert_eq!(
        ready["manifest_path"],
        installed.canonicalize().unwrap().to_string_lossy().as_ref()
    );
    assert_eq!(ready["inventory"]["profile_count"], 1);
    assert_eq!(ready["inventory"]["verified_bank_count"], 2);
    assert_eq!(
        ready["inventory"]["manifest_sha256"],
        report["rust_signature_profile"]["provenance"]["manifest_sha256"]
    );
    assert_eq!(run(&plain_args, 0), plain);
    assert_eq!(run(&report_args, 0), report);
    assert_eq!(
        check(Some(&manifest), 0)["manifest_path"],
        manifest.canonicalize().unwrap().to_string_lossy().as_ref()
    );
    let missing = temp.path().join("absent.json");
    let absent = check(Some(&missing), 1);
    assert_eq!(absent["status"], "absent");
    assert_eq!(absent["manifest_path"], missing.to_string_lossy().as_ref());

    let mut changed: Value = serde_json::from_slice(&std::fs::read(&manifest).unwrap()).unwrap();
    let bad = temp.path().join("bad.json");
    changed["signatures"][1]["sha256"] = json!("0".repeat(64));
    std::fs::write(&bad, changed.to_string()).unwrap();
    let invalid = check(Some(&bad), 1);
    assert_eq!(invalid["status"], "invalid");
    assert!(invalid["reason"]
        .as_str()
        .unwrap()
        .contains("SHA-256 mismatch"));
    changed["signatures"].as_array_mut().unwrap().pop();
    std::fs::write(&installed, changed.to_string()).unwrap();
    let partial = check(None, 1);
    assert_eq!(partial["status"], "incomplete");
    assert_eq!(partial["inventory"]["verified_bank_count"], 1);

    assert_eq!(run(&plain_args, 0), plain);
    assert_eq!(run(&report_args, 0), report);
}
