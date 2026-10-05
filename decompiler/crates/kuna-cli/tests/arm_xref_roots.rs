//! Synthetic ARM callers with unique prologues must retain their calls and owners.
mod common;

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

fn fixture(tag: &str, isa: &str, endian: &str, symbols: bool) -> PathBuf {
    fixture_variant(tag, isa, endian, symbols, &[])
}

fn fixture_variant(tag: &str, isa: &str, endian: &str, symbols: bool, options: &[&str]) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("arm-xref-{tag}-{}.elf", std::process::id()));
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/arm_xref_roots.py");
    let mut cmd = Command::new("python3");
    cmd.arg(generator)
        .arg(&path)
        .arg(isa)
        .arg(endian)
        .args(options);
    if symbols {
        cmd.arg("symbols");
    }
    assert!(cmd
        .status()
        .expect("run authored fixture generator")
        .success());
    path
}

fn query(
    path: &PathBuf,
    direction: &str,
    target: &str,
    isa: &str,
    endian: &str,
    extra: &[&str],
) -> Value {
    let language = if endian == "big" {
        "ARM:BE:32:v4t"
    } else {
        "ARM:LE:32:v4t"
    };
    query_language(path, direction, target, isa, language, extra)
}

fn query_language(
    path: &PathBuf,
    direction: &str,
    target: &str,
    isa: &str,
    language: &str,
    extra: &[&str],
) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "xrefs",
            path.to_str().unwrap(),
            direction,
            target,
            "--json",
            "--target",
            language,
            "--isa",
            isa,
            "--mode",
            "reliable",
            "--kind",
            "call",
        ])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("xref JSON")
}

fn calls(doc: &Value) -> Vec<(u64, u64, u64)> {
    doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["from_address"].as_u64().unwrap(),
                r["to_address"].as_u64().unwrap(),
                r["from_function"]["address"].as_u64().unwrap(),
            )
        })
        .collect()
}

#[test]
fn arm_loop_call_belongs_to_the_entry_before_the_early_return() {
    let path = fixture("boundary", "arm", "little", false);
    let doc = query(&path, "--to", "0x1100", "arm", "little", &[]);
    assert!(calls(&doc).contains(&(0x1418, 0x1100, 0x1400)), "{doc}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn arm_to_query_recovers_the_direct_call_in_an_unreferenced_caller() {
    let path = fixture("coverage", "arm", "little", false);
    let to = query(&path, "--to", "0x1100", "arm", "little", &[]);
    let from = query(&path, "--from", "0x1500", "arm", "little", &[]);
    assert!(calls(&from).contains(&(0x1508, 0x1100, 0x1500)), "{from}");
    assert!(calls(&to).contains(&(0x1508, 0x1100, 0x1500)), "{to}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn both_directions_keep_adjacent_and_unreferenced_callers_in_each_isa() {
    for isa in ["arm", "thumb"] {
        for endian in ["little", "big"] {
            for symbols in [false, true] {
                let path = fixture(
                    &format!("matrix-{isa}-{endian}-{symbols}"),
                    isa,
                    endian,
                    symbols,
                );
                let loop_site = if isa == "thumb" { 0x1414 } else { 0x1418 };
                let load_site = if isa == "thumb" { 0x1504 } else { 0x1508 };
                let doc = query(&path, "--to", "0x1100", isa, endian, &[]);
                let expected = vec![
                    (if isa == "thumb" { 0x1002 } else { 0x1004 }, 0x1100, 0x1000),
                    (loop_site, 0x1100, 0x1400),
                    (load_site, 0x1100, 0x1500),
                ];
                assert_eq!(
                    calls(&doc),
                    expected,
                    "{isa}/{endian}/symbols={symbols}: {doc}"
                );
                for (entry, site) in [(0x1400, loop_site), (0x1500, load_site)] {
                    let from = query(&path, "--from", &format!("0x{entry:x}"), isa, endian, &[]);
                    assert_eq!(calls(&from), vec![(site, 0x1100, entry)], "{from}");
                }
                for (entry, to, site) in [
                    (
                        0x1520,
                        if isa == "thumb" { 0x1108 } else { 0x1110 },
                        if isa == "thumb" { 0x1524 } else { 0x1528 },
                    ),
                    (
                        0x1700,
                        if isa == "thumb" { 0x1110 } else { 0x1120 },
                        if isa == "thumb" { 0x1704 } else { 0x1708 },
                    ),
                ] {
                    let doc = query(&path, "--to", &format!("0x{to:x}"), isa, endian, &[]);
                    assert!(calls(&doc).contains(&(site, to, entry)), "{doc}");
                }
                let negative = query(&path, "--to", "0x9000", isa, endian, &[]);
                assert!(
                    calls(&negative).is_empty(),
                    "invalid prologue admitted: {negative}"
                );
                std::fs::remove_file(path).unwrap();
            }
        }
    }
}

#[test]
fn prologue_recovery_needs_its_own_gate_but_not_aif() {
    let path = fixture("gates", "arm", "little", false);
    let no_aif = query(
        &path,
        "--to",
        "0x1100",
        "arm",
        "little",
        &["--option", "aif", "off"],
    );
    assert_eq!(calls(&no_aif).len(), 3, "{no_aif}");
    let off = query(
        &path,
        "--to",
        "0x1100",
        "arm",
        "little",
        &[
            "--option",
            "funcstart_patterns",
            "off",
            "--option",
            "aif",
            "off",
        ],
    );
    assert_eq!(calls(&off), vec![(0x1004, 0x1100, 0x1000)], "{off}");
    let explicit = query(
        &path,
        "--from",
        "0x1500",
        "arm",
        "little",
        &[
            "--option",
            "funcstart_patterns",
            "off",
            "--option",
            "aif",
            "off",
        ],
    );
    assert_eq!(
        calls(&explicit),
        vec![(0x1508, 0x1100, 0x1500)],
        "{explicit}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mapping_symbols_select_thumb_roots_and_explicit_arm_takes_precedence() {
    let path = fixture("mixed", "arm", "little", false);
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/arm_xref_roots.py");
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .arg("mixed")
        .status()
        .unwrap()
        .success());
    let auto = query(&path, "--to", "0x1740", "auto", "little", &[]);
    assert_eq!(calls(&auto), vec![(0x1704, 0x1740, 0x1700)], "{auto}");
    let arm = query(&path, "--to", "0x1740", "arm", "little", &[]);
    assert!(
        calls(&arm).is_empty(),
        "explicit ARM decoded Thumb bytes: {arm}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn focused_caller_before_its_frame_keeps_calls_without_claiming_other_roots() {
    for endian in ["little", "big"] {
        let path = fixture(&format!("focus-prefix-{endian}"), "arm", endian, false);
        let mut bytes = std::fs::read(&path).unwrap();
        let prefix = if endian == "big" {
            0xe1a0c00du32.to_be_bytes()
        } else {
            0xe1a0c00du32.to_le_bytes()
        };
        bytes[0x5fc..0x600].copy_from_slice(&prefix); // mov ip, sp at 0x14fc
        std::fs::write(&path, bytes).unwrap();
        for aif in ["on", "off"] {
            let options = ["--option", "aif", aif];
            let from = query(&path, "--from", "0x14fc", "arm", endian, &options);
            assert_eq!(calls(&from), vec![(0x1508, 0x1100, 0x14fc)], "{from}");
            let interior = query(&path, "--from", "0x1410", "arm", endian, &options);
            assert!(
                calls(&interior).is_empty(),
                "interior became a caller: {interior}"
            );
            let adjacent = query(&path, "--from", "0x1520", "arm", endian, &options);
            assert_eq!(
                calls(&adjacent),
                vec![(0x1528, 0x1110, 0x1520)],
                "{adjacent}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn thumb2_instruction_interior_does_not_become_a_frame_root() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(&format!("wide-{endian}"), "thumb", endian, false, &["wide"]);
        for aif in ["on", "off"] {
            let options = ["--option", "aif", aif];
            let from = query_language(&path, "--from", "0x1500", "thumb", language, &options);
            assert_eq!(calls(&from), vec![(0x1506, 0x1100, 0x1500)], "{from}");
            let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
            assert!(calls(&to).contains(&(0x1506, 0x1100, 0x1500)), "{to}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn aif_supported_prefix_remains_the_caller_in_both_query_directions() {
    for endian in ["little", "big"] {
        let path = fixture_variant(
            &format!("aif-prefix-{endian}"),
            "arm",
            endian,
            false,
            &["aifprefix"],
        );
        let options = ["--option", "aif", "on"];
        let to = query(&path, "--to", "0x1100", "arm", endian, &options);
        assert!(calls(&to).contains(&(0x1508, 0x1100, 0x14fc)), "{to}");
        let from = query(&path, "--from", "0x14fc", "arm", endian, &options);
        assert_eq!(calls(&from), vec![(0x1508, 0x1100, 0x14fc)], "{from}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn aif_uses_the_function_count_and_fingerprints_after_frames_are_walked() {
    for variant in ["aifcount", "aiffingerprint"] {
        for endian in ["little", "big"] {
            let path = fixture_variant(
                &format!("{variant}-{endian}"),
                "arm",
                endian,
                false,
                &[variant],
            );
            for patterns in ["off", "on"] {
                let to = query(
                    &path,
                    "--to",
                    "0x1500",
                    "arm",
                    endian,
                    &[
                        "--option",
                        "aif",
                        "on",
                        "--option",
                        "funcstart_patterns",
                        patterns,
                    ],
                );
                assert_eq!(
                    calls(&to),
                    vec![(0x155c, 0x1500, 0x1550)],
                    "{variant}/{patterns}: {to}"
                );
            }
            let no_aif = query(
                &path,
                "--to",
                "0x1500",
                "arm",
                endian,
                &["--option", "aif", "off"],
            );
            assert!(calls(&no_aif).is_empty(), "{no_aif}");
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn a_frame_in_a_gap_does_not_take_a_seeded_callers_distant_block() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            &format!("split-{endian}"),
            "arm",
            endian,
            false,
            &["splitbody"],
        );
        for aif in ["off", "on"] {
            let options = ["--option", "aif", aif];
            let from = query_language(&path, "--from", "0x1500", "arm", language, &options);
            assert_eq!(calls(&from), vec![(0x1750, 0x1100, 0x1500)], "{from}");
            let to = query_language(&path, "--to", "0x1100", "arm", language, &options);
            assert_eq!(calls(&to), vec![(0x1750, 0x1100, 0x1500)], "{to}");
            let unrelated = query_language(&path, "--from", "0x1600", "arm", language, &options);
            assert!(calls(&unrelated).is_empty(), "{unrelated}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn aif_rechecks_frame_prefixes_after_discovery_crosses_its_threshold() {
    for endian in ["little", "big"] {
        let path = fixture_variant(
            &format!("late-prefix-{endian}"),
            "arm",
            endian,
            false,
            &["aifprefix", "lateprefix"],
        );
        for patterns in ["off", "on"] {
            let options = [
                "--option",
                "aif",
                "on",
                "--option",
                "funcstart_patterns",
                patterns,
            ];
            let to = query(&path, "--to", "0x1700", "arm", endian, &options);
            assert_eq!(
                calls(&to),
                vec![(0x1508, 0x1700, 0x14fc)],
                "{patterns}: {to}"
            );
            let from = query(&path, "--from", "0x14fc", "arm", endian, &options);
            assert_eq!(calls(&from), vec![(0x1508, 0x1700, 0x14fc)], "{from}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn aif_prefix_claims_the_second_halfword_of_a_thumb2_instruction() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("wide-prefix", "thumb", endian, false, &["wideprefix"]);
        for patterns in ["off", "on"] {
            let options = [
                "--option",
                "aif",
                "on",
                "--option",
                "funcstart_patterns",
                patterns,
            ];
            let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
            assert!(
                calls(&to).contains(&(0x1508, 0x1100, 0x1500)),
                "{patterns}: {to}"
            );
            let from = query_language(&path, "--from", "0x1500", "thumb", language, &options);
            assert_eq!(calls(&from), vec![(0x1508, 0x1100, 0x1500)], "{from}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn replacing_a_late_frame_discards_its_phantom_call() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("stale-prefix", "thumb", endian, false, &["staleprefix"]);
        let options = [
            "--option",
            "aif",
            "on",
            "--option",
            "funcstart_patterns",
            "on",
        ];
        let to = query_language(&path, "--to", "0x1700", "thumb", language, &options);
        assert_eq!(to["xrefs"].as_array().unwrap().len(), 0, "{to}");
        let from = query_language(&path, "--from", "0x1500", "thumb", language, &options);
        assert_eq!(from["xrefs"].as_array().unwrap().len(), 0, "{from}");
        let live = query_language(&path, "--from", "0x1600", "thumb", language, &options);
        assert_eq!(calls(&live), vec![(0x1604, 0x1100, 0x1600)], "{live}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_rebuild_reconsiders_independently_validated_frame_callees() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "stale-callee",
            "thumb",
            endian,
            false,
            &["staleprefix", "stalecallee"],
        );
        for aif in ["off", "on"] {
            let options = [
                "--option",
                "aif",
                aif,
                "--option",
                "funcstart_patterns",
                "on",
            ];
            let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
            assert_eq!(
                calls(&to),
                vec![(0x1002, 0x1100, 0x1000), (0x1604, 0x1100, 0x1600)],
                "{to}"
            );
            let from = query_language(&path, "--from", "0x1600", "thumb", language, &options);
            assert_eq!(calls(&from), vec![(0x1604, 0x1100, 0x1600)], "{from}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn discarded_frame_blx_does_not_change_the_focused_thumb_isa() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variant in ["staleblx", "rejectedblx"] {
            let path = fixture_variant(
                "stale-mode",
                "thumb",
                endian,
                false,
                &["staleprefix", variant],
            );
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    "on",
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let from = query_language(&path, "--from", "0x1700", "thumb", language, &options);
                assert_eq!(
                    calls(&from),
                    vec![(0x1704, 0x1100, 0x1700)],
                    "{variant}/{patterns}: {from}"
                );
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn retried_frame_callees_reconcile_their_own_prefixes() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variant in ["chainprefix", "longchainprefix"] {
            let path = fixture_variant(
                "chained-prefixes",
                "thumb",
                endian,
                false,
                &["staleprefix", variant],
            );
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    "on",
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let to = query_language(&path, "--to", "0x1700", "thumb", language, &options);
                assert_eq!(calls(&to), vec![], "{variant}/{patterns}: {to}");
                let from = query_language(&path, "--from", "0x1700", "thumb", language, &options);
                assert_eq!(calls(&from), vec![(0x1704, 0x1100, 0x1700)], "{from}");
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn backward_speculative_blx_cannot_repaint_frame_prefixes() {
    for (endian, language) in [("big", "ARM:BE:32:v8"), ("little", "ARM:LE:32:v8")] {
        let path = fixture_variant(
            "backward-mode",
            "thumb",
            endian,
            false,
            &["staleprefix", "backwardblx"],
        );
        for patterns in ["off", "on"] {
            let options = [
                "--option",
                "aif",
                "on",
                "--option",
                "funcstart_patterns",
                patterns,
            ];
            let from = query_language(&path, "--from", "0x1700", "thumb", language, &options);
            assert_eq!(
                calls(&from),
                vec![(0x1704, 0x1100, 0x1700)],
                "{endian}/{patterns}: {from}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn speculative_blx_cannot_repaint_corpus_fingerprints() {
    for (endian, language) in [("big", "ARM:BE:32:v8"), ("little", "ARM:LE:32:v8")] {
        let path = fixture_variant(
            "corpus-mode",
            "thumb",
            endian,
            false,
            &["staleprefix", "corpusblx"],
        );
        for patterns in ["off", "on"] {
            let options = [
                "--option", "aif", "on", "--option", "funcstart_patterns", patterns,
            ];
            let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
            assert_eq!(
                calls(&to),
                if patterns == "on" {
                    vec![(0x1002, 0x1100, 0x1000), (0x1604, 0x1100, 0x1600)]
                } else {
                    vec![(0x1002, 0x1100, 0x1000)]
                },
                "{endian}/{patterns}: {to}"
            );
            let from = query_language(&path, "--from", "0x1700", "thumb", language, &options);
            assert_eq!(
                calls(&from),
                vec![(0x1704, 0x1100, 0x1700)],
                "{endian}/{patterns}: {from}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn speculative_call_cycles_do_not_protect_frame_interiors() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variant in ["chainprefix", "longchainprefix", "selfcycleprefix"] {
            let path = fixture_variant(
                "frame-cycle",
                "thumb",
                endian,
                false,
                &["staleprefix", variant, "cycleprefix"],
            );
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    "on",
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
                assert_eq!(
                    calls(&to),
                    vec![(0x1002, 0x1100, 0x1000)],
                    "{variant}/{patterns}: {to}"
                );
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn provisional_calls_to_prefixes_cannot_protect_frame_interiors() {
    for (endian, language) in [("big", "ARM:BE:32:v8"), ("little", "ARM:LE:32:v8")] {
        for variant in ["selfcycleprefix", "chainprefix", "longchainprefix"] {
            let path = fixture_variant(
                "called-prefix",
                "thumb",
                endian,
                false,
                &["staleprefix", variant, "cycleprefix", "calledprefix"],
            );
            for patterns in ["off", "on"] {
                let options = [
                    "--option", "aif", "on", "--option", "funcstart_patterns", patterns,
                ];
                let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
                assert_eq!(
                    calls(&to),
                    vec![(0x1002, 0x1100, 0x1000)],
                    "{variant}/{endian}/{patterns}: {to}"
                );
                let from = query_language(&path, "--from", "0x1500", "thumb", language, &options);
                assert!(calls(&from).is_empty(), "{variant}/{endian}/{patterns}: {from}");
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn an_independent_recovered_caller_keeps_its_recursive_callees() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "anchored-cycle",
            "thumb",
            endian,
            false,
            &["staleprefix", "chainprefix", "cycleprefix", "cycleanchor"],
        );
        let options = [
            "--option",
            "aif",
            "on",
            "--option",
            "funcstart_patterns",
            "on",
        ];
        let to = query_language(&path, "--to", "0x1100", "thumb", language, &options);
        assert_eq!(
            calls(&to),
            vec![
                (0x1002, 0x1100, 0x1000),
                (0x150e, 0x1100, 0x1506),
                (0x160e, 0x1100, 0x1606)
            ],
            "{to}"
        );
        let from = query_language(&path, "--from", "0x1700", "thumb", language, &options);
        assert_eq!(calls(&from), vec![(0x1704, 0x1506, 0x1700)], "{from}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn interworking_callees_supply_fingerprints_in_their_decoded_mode() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for (mapped, late) in [(true, false), (false, false), (false, true)] {
            let mut flags = vec!["interworkfp"];
            if mapped {
                flags.push("mappedcallee");
            }
            if late {
                flags.push("latecallee");
            }
            let path = fixture_variant("interwork-fingerprint", "arm", endian, false, &flags);
            let options = ["--option", "aif", "on", "--option", "funcstart_patterns", "on"];
            let to = query_language(&path, "--to", "0x1100", "auto", language, &options);
            assert_eq!(
                calls(&to),
                vec![(0x1004, 0x1100, 0x1000), (0x1704, 0x1100, 0x1700)],
                "{endian}/mapped={mapped}/late={late}: {to}"
            );
            let from = query_language(&path, "--from", "0x1600", "auto", language, &options);
            assert_eq!(
                calls(&from),
                vec![(0x1608, if late { 0x16f8 } else { 0x1700 }, 0x1600)],
                "{from}"
            );
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn later_interworking_discards_calls_from_an_obsolete_frame_decode() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("stale-frame-mode", "arm", endian, false, &["staleisa"]);
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    aif,
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let to = query_language(&path, "--to", "0x1700", "arm", language, &options);
                assert!(calls(&to).is_empty(), "{endian}/{aif}/{patterns}: {to}");
                let from = query_language(&path, "--from", "0x1650", "arm", language, &options);
                assert_eq!(calls(&from), vec![(0x1658, 0x1500, 0x1650)], "{from}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn interior_interworking_removes_phantom_calls_in_both_byte_orders() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "interior-frame-mode",
            "arm",
            endian,
            false,
            &["staleisa", "interiorisa"],
        );
        for aif in ["off", "on"] {
            let doc = query_language(
                &path,
                "--to",
                "0x1700",
                "arm",
                language,
                &["--option", "aif", aif],
            );
            assert!(calls(&doc).is_empty(), "{endian}/{aif}: {doc}");
            let from = query_language(
                &path,
                "--from",
                "0x1650",
                "arm",
                language,
                &["--option", "aif", aif],
            );
            assert_eq!(
                calls(&from),
                vec![(
                    0x1658,
                    if endian == "little" { 0x1502 } else { 0x1504 },
                    0x1650
                )]
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_discards_callees_invalidated_by_gap_interworking() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "gap-frame-inventory",
            "arm",
            endian,
            false,
            &["staleisa", "stalegap"],
        );
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "functions",
                path.to_str().unwrap(),
                "--json",
                "--target",
                language,
                "--isa",
                "arm",
                "--mode",
                "reliable",
                "--option",
                "listing",
                "on",
                "--option",
                "funcstart_patterns",
                "on",
                "--option",
                "aif",
                "on",
            ])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
        let entries: Vec<_> = doc["functions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["address"].as_u64().unwrap())
            .collect();
        assert!(entries.contains(&0x1400), "missing gap caller: {doc}");
        assert!(
            entries.contains(&0x1500),
            "missing interworking callee: {doc}"
        );
        assert!(!entries.contains(&0x1700), "obsolete callee: {doc}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_checks_gap_entries_before_rewalking_aif_candidates() {
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/cortexm_poolentry_le32.py");
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("arm-entry-checks-{}.elf", std::process::id()));
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .args(["--frame", "--pointer"])
        .status()
        .unwrap()
        .success());
    for (pool, pointers) in [("on", "off"), ("off", "on"), ("on", "on")] {
        let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "functions",
                path.to_str().unwrap(),
                "--json",
                "--option",
                "listing",
                "on",
                "--option",
                "funcstart_patterns",
                "on",
                "--option",
                "aif",
                "on",
                "--option",
                "aifstrict",
                "off",
                "--option",
                "aifcorroborate",
                "off",
                "--option",
                "poolentry",
                pool,
                "--option",
                "ptrentry",
                pointers,
            ])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let doc: Value = serde_json::from_slice(&result.stdout).unwrap();
        let entries: Vec<_> = doc["functions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["address"].as_u64().unwrap())
            .collect();
        assert!(
            entries.contains(&0x8000200),
            "the uncalled frame must be recovered"
        );
        assert!(
            entries.contains(&0x800014c),
            "real entry missing: pool={pool}, ptr={pointers}"
        );
        assert_eq!(
            entries.contains(&0x800014a),
            pool == "off",
            "pool-filtered entry was recommitted"
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn recovered_interworking_preserves_unrelated_focus_modes() {
    let isa = "arm";
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            &format!("focus-mode-{isa}-{endian}"),
            isa,
            endian,
            false,
            &["focusmode"],
        );
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    aif,
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let from = query_language(&path, "--from", "0x1600", isa, language, &options);
                assert_eq!(
                    calls(&from),
                    vec![(0x1604, 0x1200, 0x1600)],
                    "{isa}/{endian}/{aif}/{patterns}: {from}"
                );
            }
            let options = ["--option", "aif", aif];
            let caller = query_language(&path, "--from", "0x1500", isa, language, &options);
            assert_eq!(calls(&caller), vec![(0x1504, 0x1580, 0x1500)], "{caller}");
            let callee = query_language(&path, "--from", "0x1580", isa, language, &options);
            assert_eq!(
                calls(&callee),
                vec![(0x1584, 0x1300, 0x1580)],
                "a direct BLX must establish the focused callee's mode: {callee}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn stale_aif_modes_do_not_restart_inventory_forever() {
    use std::time::Duration;
    for (endian, language) in [("big", "ARM:BE:32:v8"), ("little", "ARM:LE:32:v8")] {
        let path = fixture_variant("stale-aif-mode", "arm", endian, false, &["staleaif"]);
        for patterns in ["off", "on"] {
            let out = common::process::output_with_timeout(
                Command::new(env!("CARGO_BIN_EXE_kuna")).args([
                    "functions", path.to_str().unwrap(), "--json", "--target", language,
                    "--mode", "reliable", "--option", "listing", "on", "--option", "aif", "on",
                    "--option", "funcstart_patterns", patterns,
                ]),
                Duration::from_secs(20), Duration::from_millis(10),
            ).unwrap_or_else(|| panic!("inventory did not terminate: {endian}/{patterns}"));
            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
            let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
            let entries: Vec<_> = doc["functions"].as_array().unwrap().iter()
                .map(|f| f["address"].as_u64().unwrap()).collect();
            assert!(entries.contains(&0x1300), "independent AIF entry lost: {doc}");
            if patterns == "on" {
                assert!(entries.contains(&0x160c), "direct Thumb callee lost: {doc}");
                assert!(!entries.contains(&0x1600), "stale AIF entry readmitted: {doc}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_xrefs_preserve_established_modes_for_callee_first_decompilation() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("xref-published-mode", "arm", endian, false,
            &["focusmode", "rootmode", "splitframe", "focusblocks", "xrefpublish"]);
        for protoorder in ["off", "types"] {
            for patterns in ["off", "on"] {
                let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args(["decompile-all", path.to_str().unwrap(), "--json", "--addr", "0x1500", "--addr", "0x1100",
                        "--target", language, "--mode", "reliable", "--option", "listing", "off",
                        "--option", "aif", "off", "--option", "protoorder", protoorder,
                        "--option", "funcstart_patterns", patterns])
                    .output().unwrap();
                assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
                let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
                let caller = doc["functions"].as_array().unwrap().iter()
                    .find(|f| f["address"].as_u64() == Some(0x1500)).unwrap();
                let code = caller["code"].as_str().unwrap_or_else(|| panic!("{caller}"));
                assert!(code.contains("sub_1100("), "{endian}/{protoorder}/{patterns}: {code}");
                assert!(!code.contains("halt_"), "{code}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_inventory_preserves_independent_aif_body_modes() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("aif-seeded-mode", "arm", endian, false, &["aifseedmode"]);
        for patterns in ["off", "on"] {
            for direction in ["--from", "--to"] {
                let address = if direction == "--from" { "0x1500" } else { "0x1200" };
                let doc = query_language(&path, direction, address, "auto", language,
                    &["--option", "listing", "on", "--option", "aif", "on",
                        "--option", "funcstart_patterns", patterns]);
                assert!(calls(&doc).contains(&(0x1750, 0x1200, 0x1500)),
                    "{endian}/{direction}/{patterns}: {doc}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_modes_do_not_escape_into_unclaimed_decompilation() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("unclaimed-mode", "arm", endian, false,
            &["focusmode", "rootmode", "splitframe", "unclaimedmode"]);
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args(["decompile", path.to_str().unwrap(), "0x1750", "--addr",
                        "--target", language, "--mode", "reliable", "--option", "listing", "on",
                        "--option", "funcstart_patterns", patterns, "--option", "aif", aif])
                    .output().unwrap();
                assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
                let code = String::from_utf8(out.stdout).unwrap();
                assert!(code.contains("sub_1100("), "{endian}/{aif}/{patterns}: {code}");
                assert!(!code.contains("halt_"), "{code}");
            }
            let callee = query_language(&path, "--to", "0x1300", "auto", language,
                &["--option", "listing", "on", "--option", "aif", aif]);
            assert_eq!(calls(&callee), vec![(0x1684, 0x1300, 0x1680)], "{callee}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_modes_preserve_unframed_focus_blocks() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("unframed-focus-mode", "arm", endian, false,
            &["focusmode", "rootmode", "splitframe", "focusblocks"]);
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                for isa in ["auto", "arm"] {
                    let doc = query_language(&path, "--from", "0x1500", isa, language,
                        &["--option", "listing", "off", "--option", "aif", aif,
                            "--option", "funcstart_patterns", patterns]);
                    assert_eq!(calls(&doc), vec![(0x1750, 0x1100, 0x1500)],
                        "{endian}/{aif}/{patterns}/{isa}: {doc}");
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_frame_modes_cover_discontiguous_blocks() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["focusmode", "rootmode", "splitframe"][..],
            &["focusmode", "rootmode", "splitframe", "armcallee"][..],
            &["focusmode", "rootmode", "splitframe", "backwardframe"][..],
            &["focusmode", "rootmode", "splitframe", "backwardframe", "armcallee"][..],
        ] {
            let path = fixture_variant("split-frame-mode", "arm", endian, false, variants);
            for aif in ["off", "on"] {
                for listing in ["off", "on"] {
                    let options = ["--option", "aif", aif, "--option", "listing", listing];
                    let explicit = query_language(&path, "--from", "0x1600", "arm", language, &options);
                    assert_eq!(calls(&explicit), vec![(0x1750, 0x1100, 0x1600)], "{explicit}");
                    for (direction, target, expected) in [
                        ("--from", "0x1600", (0x1750, 0x1100, 0x1600)),
                        ("--to", "0x1100", (0x1750, 0x1100, 0x1600)),
                        ("--to", "0x1300", (0x1684, 0x1300, 0x1680)),
                    ] {
                        let doc = query_language(&path, direction, target, "auto", language, &options);
                        assert_eq!(calls(&doc), vec![expected],
                            "{endian}/{aif}/{listing}/{variants:?}/{direction}/{target}: {doc}");
                    }
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn a_recursive_frame_retains_its_validated_distant_block_mode() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("recursive-split-frame-mode", "arm", endian, false,
            &["focusmode", "rootmode", "splitframe", "recursiveframe"]);
        for aif in ["off", "on"] {
            for listing in ["off", "on"] {
                let options = ["--option", "aif", aif, "--option", "listing", listing];
                let doc = query_language(&path, "--from", "0x1600", "arm", language, &options);
                let mut actual = calls(&doc);
                actual.sort_unstable();
                assert_eq!(actual, vec![(0x1604, 0x1600, 0x1600), (0x1750, 0x1100, 0x1600)],
                    "{endian}/{aif}/{listing}: {doc}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_restores_pending_frame_modes_after_backward_interworking() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["focusmode", "rootmode", "backwardframe"][..],
            &["focusmode", "rootmode", "backwardframe", "armcallee"][..],
        ] {
            let path = fixture_variant("inventory-backward-mode", "arm", endian, false, variants);
            for aif in ["off", "on"] {
                for listing in ["off", "on"] {
                    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args(["xrefs", path.to_str().unwrap(), "--from", "0x1500",
                            "--kind", "call", "--json", "--target", language, "--mode", "reliable",
                            "--option", "aif", aif, "--option", "listing", listing])
                        .output().unwrap();
                    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
                    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
                    assert_eq!(calls(&doc), vec![(0x1504, 0x1200, 0x1500)],
                        "{endian}/{aif}/{listing}/{variants:?}: {doc}");
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn inventory_seeds_retain_independent_frame_modes() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["focusmode", "rootmode"][..],
            &["focusmode", "rootmode", "armcallee"][..],
        ] {
            let path = fixture_variant("inventory-seed-mode", "arm", endian, false, variants);
            for aif in ["off", "on"] {
                for listing in ["off", "on"] {
                    let options = ["--option", "aif", aif, "--option", "listing", listing];
                    for (direction, target) in [("--from", "0x1600"), ("--to", "0x1200")] {
                        let doc = query_language(&path, direction, target, "arm", language, &options);
                        assert_eq!(
                            calls(&doc), vec![(0x1604, 0x1200, 0x1600)],
                            "{endian}/{aif}/{listing}/{variants:?}: {doc}"
                        );
                    }
                    let callee = query_language(&path, "--from", "0x1580", "arm", language, &options);
                    assert_eq!(
                        calls(&callee), vec![(0x1584, 0x1300, 0x1580)],
                        "direct interworking must override the saved seed mode: {callee}"
                    );
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn recovered_interworking_preserves_pending_frame_modes() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["focusmode", "rootmode"][..],
            &["focusmode", "rootmode", "armcallee"][..],
        ] {
            let path = fixture_variant("pending-frame-mode", "arm", endian, false, variants);
            for aif in ["off", "on"] {
                let options = ["--option", "aif", aif];
                let incoming = query_language(&path, "--to", "0x1200", "arm", language, &options);
                assert_eq!(
                    calls(&incoming),
                    vec![(0x1604, 0x1200, 0x1600)],
                    "{endian}/{aif}/{variants:?}: {incoming}"
                );
                let outgoing = query_language(&path, "--from", "0x1600", "arm", language, &options);
                assert_eq!(calls(&outgoing), calls(&incoming), "{outgoing}");
                let callee = query_language(&path, "--to", "0x1300", "arm", language, &options);
                assert_eq!(
                    calls(&callee),
                    vec![(0x1584, 0x1300, 0x1580)],
                    "direct-callee evidence must retain its mode: {callee}"
                );
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn recovered_interworking_preserves_unrelated_aif_body_modes() {
    for (endian, language) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for (variants, thumb_call) in [
            (&["gapmode"][..], 0x1584),
            (&["gapmode", "splitgapcallee"][..], 0x1680),
            (&["gapmode", "gapbranch"][..], 0x1584),
            (&["gapmode", "gapbranch", "splitgapcallee"][..], 0x1680),
            (&["gapmode", "gapbranch", "gapbound"][..], 0x1584),
        ] {
            let path = fixture_variant(
                &format!("gap-mode-{endian}-{thumb_call:x}"),
                "arm",
                endian,
                false,
                variants,
            );
            for patterns in ["off", "on"] {
                let options = [
                    "--option",
                    "aif",
                    "on",
                    "--option",
                    "funcstart_patterns",
                    patterns,
                ];
                let incoming = query_language(&path, "--to", "0x1200", "arm", language, &options);
                let mut expected = vec![(0x1044, 0x1200, 0x1000)];
                if !variants.contains(&"gapbound") {
                    expected.push((0x1600, 0x1200, 0x1400));
                }
                assert_eq!(
                    calls(&incoming),
                    expected,
                    "{endian}/{patterns}/{variants:?}: {incoming}"
                );
                if !variants.contains(&"gapbound") {
                    let outgoing =
                        query_language(&path, "--from", "0x1400", "arm", language, &options);
                    let expected = if variants.contains(&"gapbranch") {
                        vec![(0x1600, 0x1200, 0x1400)]
                    } else {
                        vec![(0x140c, 0x1100, 0x1400), (0x1600, 0x1200, 0x1400)]
                    };
                    assert_eq!(calls(&outgoing), expected, "{outgoing}");
                }
                if patterns == "on" {
                    let thumb = query_language(&path, "--to", "0x1300", "arm", language, &options);
                    assert_eq!(calls(&thumb), vec![(thumb_call, 0x1300, 0x1580)], "{thumb}");
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}
