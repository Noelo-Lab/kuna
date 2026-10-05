//! The Listing and reference walk share mode-aware ARM frame roots.
use kuna_analysis::listing::xrefs::{self, XrefKind};
use kuna_console::engine::{bootstrap_from_object_with_isa, ArmIsa};
use std::path::PathBuf;
use std::process::Command;

fn fixture(tag: &str, isa: &str, endian: &str) -> PathBuf {
    fixture_variant(tag, isa, endian, &[])
}

fn fixture_variant(tag: &str, isa: &str, endian: &str, options: &[&str]) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("arm-root-index-{tag}-{}.elf", std::process::id()));
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/arm_xref_roots.py");
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .arg(isa)
        .arg(endian)
        .args(options)
        .status()
        .expect("generate authored ELF")
        .success());
    path
}

#[test]
fn frame_roots_preserve_owners_without_interior_or_invalid_entries() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (isa, mode) in [("arm", ArmIsa::Arm), ("thumb", ArmIsa::Thumb)] {
        for endian in ["little", "big"] {
            let path = fixture(&format!("{isa}-{endian}"), isa, endian);
            let target = if endian == "big" {
                "ARM:BE:32:v4t"
            } else {
                "ARM:LE:32:v4t"
            };
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(mode),
            )
            .expect("bootstrap synthetic ELF");
            prog.commit_pending_analysis().unwrap();
            prog.arch_mut()
                .set_kuna_option("funcstart_patterns", "on")
                .unwrap();
            prog.arch_mut().set_kuna_option("aif", "off").unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index =
                xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            let loop_site = if isa == "thumb" { 0x1414 } else { 0x1418 };
            let load_site = if isa == "thumb" { 0x1504 } else { 0x1508 };
            assert_eq!(
                index.function_instruction_counts(&[0x1400, 0x1500], 100),
                vec![if isa == "thumb" { 13 } else { 12 }, 4]
            );
            assert_eq!(index.function_containing(loop_site), Some(0x1400));
            assert_eq!(index.function_containing(load_site), Some(0x1500));
            let callers: Vec<_> = index
                .refs_to(0x1100)
                .iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| r.from)
                .collect();
            assert_eq!(
                callers,
                vec![
                    if isa == "thumb" { 0x1002 } else { 0x1004 },
                    loop_site,
                    load_site
                ]
            );
            for entry in [0x1400, 0x1500, 0x1520, 0x1700] {
                assert!(index.is_function_entry(entry), "missing root {entry:x}");
            }
            for interior in [0x140C, 0x1410, 0x1430, 0x1600] {
                assert!(
                    !index.is_function_entry(interior),
                    "false root {interior:x}"
                );
            }
            assert!(index.refs_to(0x9000).is_empty());
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn listing_inventory_recovers_the_same_arm_entries() {
    let path = fixture("inventory", "arm", "little");
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v4t",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    for (name, value) in [
        ("listing", "on"),
        ("funcstart_patterns", "on"),
        ("aif", "off"),
    ] {
        prog.arch_mut().set_kuna_option(name, value).unwrap();
    }
    prog.commit_pending_analysis().unwrap();
    let entries: Vec<_> = prog
        .function_entries_canonical()
        .iter()
        .map(|entry| entry.addr.get_offset())
        .collect();
    for entry in [0x1400, 0x1500, 0x1520, 0x1700] {
        assert!(entries.contains(&entry), "inventory omitted {entry:x}");
    }
    assert!(!entries.contains(&0x1410));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn be8_frames_use_little_endian_instructions_in_a_big_endian_elf() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (isa, mode) in [("thumb", ArmIsa::Thumb), ("arm", ArmIsa::Arm)] {
        let path = fixture(&format!("be8-{isa}"), isa, "be8");
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            "ARM:LEBE:32:v8LEInstruction",
            &[specs.to_str().unwrap().into()],
            Some(mode),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", "off"),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries: Vec<_> = prog
            .function_entries_canonical()
            .iter()
            .map(|entry| entry.addr.get_offset())
            .collect();
        assert!(
            entries.contains(&0x1500),
            "{isa}: missing BE8 frame: {entries:x?}"
        );
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
        let site = if isa == "thumb" { 0x1504 } else { 0x1508 };
        assert_eq!(index.function_containing(site), Some(0x1500));
        assert!(index
            .refs_to(0x1100)
            .iter()
            .any(|r| r.from == site && r.kind == XrefKind::Call));
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn listing_preserves_an_aif_supported_prefix_before_a_frame() {
    let path = fixture("aif-prefix", "arm", "aifprefix");
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v4t",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    for (name, value) in [
        ("listing", "on"),
        ("funcstart_patterns", "on"),
        ("aif", "on"),
    ] {
        prog.arch_mut().set_kuna_option(name, value).unwrap();
    }
    prog.commit_pending_analysis().unwrap();
    let entries: Vec<_> = prog
        .function_entries_canonical()
        .iter()
        .map(|entry| entry.addr.get_offset())
        .collect();
    assert!(entries.contains(&0x14fc), "missing prefix: {entries:x?}");
    assert!(
        !entries.contains(&0x1500),
        "interior frame became a root: {entries:x?}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn listing_reconciles_frames_inside_a_validated_prefix_instruction() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("wide-prefix", "thumb", endian, &["wideprefix"]);
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Thumb),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", "on"),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries: Vec<_> = prog
            .function_entries_canonical()
            .iter()
            .map(|entry| entry.addr.get_offset())
            .collect();
        assert!(entries.contains(&0x1500), "missing prefix: {entries:x?}");
        assert!(
            !entries.contains(&0x1506),
            "instruction interior became a root: {entries:x?}"
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn distant_body_ownership_agrees_for_direct_indirect_and_branch_queries() {
    let path = fixture("split-body", "arm", "splitbody");
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v8",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .unwrap();
    prog.arch_mut().set_kuna_option("aif", "off").unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
    assert!(index.is_function_entry(0x1600));
    assert_eq!(
        index.function_instruction_counts(&[0x1500, 0x1600], 100),
        vec![4, 3]
    );
    for at in [0x1750, 0x1754, 0x1758] {
        assert_eq!(index.function_containing(at), Some(0x1500));
    }
    assert_eq!(index.function_containing(0x1604), Some(0x1600));
    assert!(index.has_indirect_calls(0x1500));
    assert!(!index.has_indirect_calls(0x1600));
    let from = index.refs_from_function(0x1500);
    assert!(from
        .iter()
        .any(|r| r.from == 0x1750 && r.to == 0x1100 && r.kind == XrefKind::Call));
    assert!(!from.iter().any(|r| r.kind == XrefKind::Jump));
    assert!(index.refs_from_function(0x1600).is_empty());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_late_discovered_callee_owns_its_previously_decoded_body() {
    let path = fixture("late-callee", "arm", "splitbody");
    let mut bytes = std::fs::read(&path).unwrap();
    let call = 0xeb000000u32 | ((0x1750 - 0x1604 - 8) / 4);
    bytes[0x704..0x708].copy_from_slice(&call.to_le_bytes());
    std::fs::write(&path, &bytes).unwrap();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v8",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .unwrap();
    prog.arch_mut().set_kuna_option("aif", "off").unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let index = xrefs::build(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
    assert!(index.is_function_entry(0x1750));
    assert_eq!(index.function_containing(0x1754), Some(0x1750));
    assert!(index.has_indirect_calls(0x1750));
    assert!(!index.has_indirect_calls(0x1500));
    assert!(!index.has_indirect_calls(0x1600));
    assert!(index
        .refs_from_function(0x1750)
        .iter()
        .any(|r| r.from == 0x1750 && r.to == 0x1100 && r.kind == XrefKind::Call));
    assert!(index
        .refs_from_function(0x1600)
        .iter()
        .any(|r| r.from == 0x1604 && r.to == 0x1750 && r.kind == XrefKind::Call));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_predicated_return_keeps_the_nonreturning_path_in_the_same_frame() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for aif in ["off", "on"] {
        let path = fixture(&format!("predreturn-{aif}"), "arm", "predreturn");
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            "ARM:LE:32:v4t",
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Arm),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", aif),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries = prog.function_entries_canonical();
        let caller = entries
            .iter()
            .find(|e| e.addr.get_offset() == 0x1400)
            .unwrap();
        assert!(
            caller.size >= 28,
            "the extent truncates the nonreturning path: {}",
            caller.size
        );
        assert!(!entries.iter().any(|e| e.addr.get_offset() == 0x140c));
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
        assert!(!index.is_function_entry(0x140c));
        assert_eq!(index.function_containing(0x1410), Some(0x1400));
        assert_eq!(index.function_instruction_counts(&[0x1400], 100), vec![7]);
        assert!(index
            .refs_from_function(0x1400)
            .iter()
            .any(|r| r.from == 0x1410 && r.to == 0x1100 && r.kind == XrefKind::Call));
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_call_that_never_returns_ends_the_frame_without_absorbing_the_next_one() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for aif in ["off", "on"] {
        let path = fixture(&format!("noreturnpool-{aif}"), "arm", "noreturnpool");
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            "ARM:LE:32:v4t",
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Arm),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", aif),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries: Vec<u64> = prog
            .function_entries_canonical()
            .iter()
            .map(|e| e.addr.get_offset())
            .collect();
        for frame in [0x1400, 0x1418, 0x1440, 0x1458] {
            assert!(entries.contains(&frame), "aif {aif}: missing {frame:#x} in {entries:x?}");
        }
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
        for (frame, call, callee) in [
            (0x1400, 0x140c, 0x14c0),
            (0x1418, 0x1420, 0x1100),
            (0x1440, 0x144c, 0x14c0),
            (0x1458, 0x1460, 0x1110),
        ] {
            assert!(index.is_function_entry(frame), "aif {aif}: {frame:#x}");
            assert_eq!(index.function_containing(call), Some(frame), "aif {aif}: {call:#x}");
            assert!(
                index
                    .refs_from_function(frame)
                    .iter()
                    .any(|r| r.from == call && r.to == callee && r.kind == XrefKind::Call),
                "aif {aif}: {frame:#x} lost its call"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn gap_validators_still_stop_at_a_conditional_return() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let path = fixture("noreturnpool-aif-only", "arm", "noreturnpool");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v4t",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    for (name, value) in [
        ("listing", "on"),
        ("funcstart_patterns", "off"),
        ("aif", "on"),
    ] {
        prog.arch_mut().set_kuna_option(name, value).unwrap();
    }
    prog.commit_pending_analysis().unwrap();
    let entries: Vec<u64> = prog
        .function_entries_canonical()
        .iter()
        .map(|e| e.addr.get_offset())
        .collect();
    assert!(entries.contains(&0x1400), "{entries:x?}");
    assert!(entries.contains(&0x1440), "{entries:x?}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn inventory_rechecks_prefixes_after_frames_expand_the_corpus() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v4t"), ("big", "ARM:BE:32:v4t")] {
        let path = fixture_variant(
            &format!("late-prefix-{endian}"),
            "arm",
            endian,
            &["aifprefix", "lateprefix"],
        );
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Arm),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", "on"),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries = prog.function_entries_canonical();
        assert!(entries.iter().any(|e| e.addr.get_offset() == 0x14fc));
        assert!(!entries.iter().any(|e| e.addr.get_offset() == 0x1500));
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_conditional_return_does_not_hide_an_invalid_successor() {
    let path = fixture("invalid-predreturn", "arm", "predreturn");
    let mut bytes = std::fs::read(&path).unwrap();
    let branch = 0xea000000u32 | ((0x1900 - 0x140c - 8) / 4);
    bytes[0x50c..0x510].copy_from_slice(&branch.to_le_bytes());
    std::fs::write(&path, &bytes).unwrap();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v4t",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    for (name, value) in [
        ("listing", "on"),
        ("funcstart_patterns", "on"),
        ("aif", "off"),
    ] {
        prog.arch_mut().set_kuna_option(name, value).unwrap();
    }
    prog.commit_pending_analysis().unwrap();
    assert!(!prog
        .function_entries_canonical()
        .iter()
        .any(|e| e.addr.get_offset() == 0x1400));
    let file = object::File::parse(&*bytes).unwrap();
    let index = xrefs::build(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
    assert!(!index.is_function_entry(0x1400));
    assert!(index.refs_from_function(0x1400).is_empty());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn explicit_focus_contributes_to_the_prefix_fingerprint_corpus() {
    let path = fixture("focus-fingerprint", "arm", "aifprefix");
    let mut bytes = std::fs::read(&path).unwrap();
    let mut word = |at: usize, value: u32| {
        let off = at - 0x1000 + 0x100;
        bytes[off..off + 4].copy_from_slice(&value.to_le_bytes());
    };
    for i in 3..20 {
        let at = 0x1100 + 16 * i;
        word(at, 0xe3a00000);
        word(at + 4, 0xe3a01000);
        word(at + 8, 0xe12fff1e);
    }
    word(0x1700, 0xe1a0c00d);
    word(0x1704, 0xe92d4000);
    word(0x1708, 0xe8bd8000);
    word(0x170c, 0xe12fff1e);
    std::fs::write(&path, &bytes).unwrap();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_from_object_with_isa(
        path.to_str().unwrap(),
        "ARM:LE:32:v4t",
        &[specs.to_str().unwrap().into()],
        Some(ArmIsa::Arm),
    )
    .unwrap();
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .unwrap();
    prog.arch_mut().set_kuna_option("aif", "on").unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let unfocused = xrefs::build(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
    assert_eq!(unfocused.function_containing(0x1508), Some(0x1500));
    let focused = xrefs::build_with_focus(
        &file,
        prog.arch(),
        prog.arch().translate(),
        &[0x1000],
        &[0x1700],
    );
    assert_eq!(focused.function_containing(0x1508), Some(0x14fc));
    assert!(!focused.is_function_entry(0x1500));
    assert!(focused.is_function_entry(0x1700));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn replaced_frames_leave_no_decoded_body_or_discovered_callees() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("stale-prefix", "thumb", endian, &["staleprefix"]);
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Thumb),
        )
        .unwrap();
        for (name, value) in [("funcstart_patterns", "on"), ("aif", "on")] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
        assert!(index.is_function_entry(0x1500));
        assert!(
            index.refs_to(0x1700).is_empty(),
            "{:?}",
            index.refs_to(0x1700)
        );
        for at in [0x1506, 0x150a, 0x150e, 0x1510, 0x1700, 0x1704] {
            assert!(!index.is_function_entry(at), "obsolete root {at:x}");
            assert_eq!(index.function_containing(at), None, "obsolete body {at:x}");
            assert!(
                index.refs_from_instruction(at).is_empty(),
                "obsolete refs {at:x}"
            );
        }
        assert_eq!(
            index.function_instruction_counts(&[0x1500, 0x1600, 0x1700], 100),
            vec![4, 4, 0]
        );
        assert!(!index.has_indirect_calls(0x1500));
        let callers: Vec<_> = index
            .refs_to(0x1100)
            .iter()
            .filter(|r| r.kind == XrefKind::Call)
            .map(|r| r.from)
            .collect();
        assert_eq!(callers, vec![0x1002, 0x1604]);
        let focused = xrefs::build_with_focus(
            &file,
            prog.arch(),
            prog.arch().translate(),
            &[0x1000],
            &[0x1700],
        );
        assert!(focused.is_function_entry(0x1700));
        assert_eq!(focused.function_containing(0x1704), Some(0x1700));
        assert!(focused.refs_to(0x1700).is_empty());
        let seeded = xrefs::build_measured(
            &file,
            prog.arch(),
            prog.arch().translate(),
            &[0x1000, 0x1700],
        );
        assert_eq!(seeded.function_containing(0x1704), Some(0x1700));
        assert!(seeded.refs_to(0x1700).is_empty());
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn accepted_frames_keep_it_successors_and_real_blx_mode_changes() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("live-interwork", "thumb", endian, &["staleprefix"]);
        let mut bytes = std::fs::read(&path).unwrap();
        let mut half = |at: usize, value: u16| {
            let off = at - 0x1000 + 0x100;
            bytes[off..off + 2].copy_from_slice(&if endian == "big" {
                value.to_be_bytes()
            } else {
                value.to_le_bytes()
            });
        };
        for (at, value) in [
            (0x1500, 0xb510),
            (0x1502, 0x2800),
            (0x1504, 0xbf08),
            (0x1506, 0xbd10),
            (0x1508, 0xf000),
            (0x150a, 0xe8fa),
            (0x150c, 0xbd10),
        ] {
            half(at, value);
        }
        for (at, value) in [
            (0x1700, 0xe92d4000u32),
            (0x1704, 0xe3a00000),
            (0x1708, 0xeb00000c),
            (0x170c, 0xe8bd8000),
            (0x1740, 0xe12fff1e),
        ] {
            let off = at - 0x1000 + 0x100;
            bytes[off..off + 4].copy_from_slice(&if endian == "big" {
                value.to_be_bytes()
            } else {
                value.to_le_bytes()
            });
        }
        std::fs::write(&path, &bytes).unwrap();
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Thumb),
        )
        .unwrap();
        for (name, value) in [("funcstart_patterns", "on"), ("aif", "on")] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
        assert!(index
            .refs_from_function(0x1500)
            .iter()
            .any(|r| r.from == 0x1508 && r.to == 0x1700 && r.kind == XrefKind::Call));
        assert!(index
            .refs_from_function(0x1700)
            .iter()
            .any(|r| r.from == 0x1708 && r.to == 0x1740 && r.kind == XrefKind::Call));
        assert_eq!(
            index.function_instruction_counts(&[0x1500, 0x1700], 100),
            vec![6, 4]
        );
        let space = prog
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone();
        assert_eq!(
            prog.arch()
                .with_context_db_mut(
                    |db| db.get_variable_value(b"TMode", &Address::new(space, 0x1700))
                )
                .unwrap(),
            0
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn listing_rebuild_restores_context_from_discarded_blx_frames() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "listing-stale-mode",
            "thumb",
            endian,
            &["staleprefix", "staleblx"],
        );
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x146..0x148].copy_from_slice(&if endian == "big" {
            0xbd00u16.to_be_bytes()
        } else {
            0xbd00u16.to_le_bytes()
        });
        std::fs::write(&path, &bytes).unwrap();
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Thumb),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", "on"),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let space = prog
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone();
        assert_eq!(
            prog.arch()
                .with_context_db_mut(
                    |db| db.get_variable_value(b"TMode", &Address::new(space, 0x1700))
                )
                .unwrap(),
            1
        );
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_with_focus(
            &file,
            prog.arch(),
            prog.arch().translate(),
            &[0x1000],
            &[0x1700],
        );
        let calls: Vec<_> = index
            .refs_from_function(0x1700)
            .into_iter()
            .filter(|r| r.kind == XrefKind::Call)
            .map(|r| (r.from, r.to))
            .collect();
        assert_eq!(calls, vec![(0x1704, 0x1100)]);
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn repeated_reconciliation_cleans_inventory_and_bodies() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variant in ["chainprefix", "longchainprefix"] {
            let path = fixture_variant(
                "repeat-reconcile",
                "thumb",
                endian,
                &["staleprefix", variant],
            );
            for inventory in [false, true] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Thumb),
                )
                .unwrap();
                for (name, value) in [
                    ("listing", "on"),
                    ("funcstart_patterns", "on"),
                    ("aif", "on"),
                ] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                let prefixes: &[u64] = match variant {
                    "chainprefix" => &[0x1500, 0x1600],
                    _ => &[0x1500, 0x1580, 0x1600],
                };
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                if inventory {
                    prog.commit_pending_analysis().unwrap();
                    let entries: Vec<_> = prog
                        .function_entries_canonical()
                        .iter()
                        .map(|entry| entry.addr.get_offset())
                        .collect();
                    for &prefix in prefixes {
                        assert!(entries.contains(&prefix), "{variant}: missing {prefix:x}");
                        assert!(
                            !entries.contains(&(prefix + 6)),
                            "{variant}: interior {:x}",
                            prefix + 6
                        );
                    }
                } else {
                    let index = xrefs::build_measured(
                        &file,
                        prog.arch(),
                        prog.arch().translate(),
                        &[0x1000],
                    );
                    assert!(
                        index.refs_to(0x1700).is_empty(),
                        "{variant}: {:?}",
                        index.refs_to(0x1700)
                    );
                    assert_eq!(
                        index.function_instruction_counts(prefixes, 100),
                        vec![4; prefixes.len()]
                    );
                    for &prefix in prefixes {
                        assert!(index.is_function_entry(prefix));
                        assert!(!index.is_function_entry(prefix + 6));
                        assert_eq!(index.function_containing(prefix + 10), None);
                        assert!(index.refs_from_instruction(prefix + 10).is_empty());
                    }
                    assert!(!index.is_function_entry(0x1700));
                }
                let space = prog
                    .arch()
                    .manage()
                    .get_default_code_space()
                    .unwrap()
                    .clone();
                for at in [0x14f0, 0x1500, 0x1600, 0x1700] {
                    assert_eq!(
                        prog.arch()
                            .with_context_db_mut(|db| db
                                .get_variable_value(b"TMode", &Address::new(space.clone(), at)))
                            .unwrap(),
                        1,
                        "{variant}/{inventory}: ISA at {at:x}"
                    );
                }
                let focused = xrefs::build_with_focus(
                    &file,
                    prog.arch(),
                    prog.arch().translate(),
                    &[0x1000],
                    &[0x1700],
                );
                let calls: Vec<_> = focused
                    .refs_from_function(0x1700)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, r.to))
                    .collect();
                assert_eq!(calls, vec![(0x1704, 0x1100)], "{variant}/{inventory}");
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn cyclic_frame_support_requires_a_reachable_independent_caller() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variant in [
            "chainprefix",
            "longchainprefix",
            "selfcycleprefix",
            "cycleanchor",
        ] {
            let anchored = variant == "cycleanchor";
            let mut options = vec!["staleprefix", "cycleprefix", variant];
            if anchored {
                options.push("chainprefix");
            }
            let path = fixture_variant("cycle-support", "thumb", endian, &options);
            let prefixes: &[u64] = match variant {
                "selfcycleprefix" => &[0x1500],
                "longchainprefix" => &[0x1500, 0x1580, 0x1600],
                _ => &[0x1500, 0x1600],
            };
            for inventory in [false, true] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Thumb),
                )
                .unwrap();
                for (name, value) in [
                    ("listing", "on"),
                    ("funcstart_patterns", "on"),
                    ("aif", "on"),
                ] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                if inventory {
                    prog.commit_pending_analysis().unwrap();
                    let entries: Vec<_> = prog
                        .function_entries_canonical()
                        .iter()
                        .map(|entry| entry.addr.get_offset())
                        .collect();
                    for &prefix in prefixes {
                        assert_eq!(
                            entries.contains(&(prefix + 6)),
                            anchored,
                            "{variant}: {entries:x?}"
                        );
                        if !anchored {
                            assert!(entries.contains(&prefix), "missing {prefix:x}");
                        }
                    }
                } else {
                    let bytes = std::fs::read(&path).unwrap();
                    let file = object::File::parse(&*bytes).unwrap();
                    let index = xrefs::build_measured(
                        &file,
                        prog.arch(),
                        prog.arch().translate(),
                        &[0x1000],
                    );
                    let calls: Vec<_> = index
                        .refs_to(0x1100)
                        .iter()
                        .filter(|r| r.kind == XrefKind::Call)
                        .map(|r| (r.from, index.function_containing(r.from)))
                        .collect();
                    let mut expected = vec![(0x1002, Some(0x1000))];
                    if anchored {
                        expected.extend([(0x150e, Some(0x1506)), (0x160e, Some(0x1606))]);
                    }
                    assert_eq!(calls, expected, "{variant}/{endian}");
                    if !anchored {
                        assert_eq!(
                            index.function_instruction_counts(prefixes, 100),
                            vec![4; prefixes.len()]
                        );
                        for &prefix in prefixes {
                            assert!(!index.is_function_entry(prefix + 6));
                            assert_eq!(index.function_containing(prefix + 14), None);
                            assert!(index.refs_from_instruction(prefix + 14).is_empty());
                        }
                    }
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn provisional_prefix_callees_do_not_leave_inventory_entries_or_bodies() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("big", "ARM:BE:32:v8"), ("little", "ARM:LE:32:v8")] {
        for variant in ["selfcycleprefix", "chainprefix", "longchainprefix"] {
            let path = fixture_variant(
                "called-prefix-body",
                "thumb",
                endian,
                &["staleprefix", variant, "cycleprefix", "calledprefix"],
            );
            let prefixes: &[u64] = match variant {
                "selfcycleprefix" => &[0x1500],
                "chainprefix" => &[0x1500, 0x1600],
                _ => &[0x1500, 0x1580, 0x1600],
            };
            for inventory in [true, false] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Thumb),
                )
                .unwrap();
                for (name, value) in [
                    ("listing", "on"),
                    ("funcstart_patterns", "on"),
                    ("aif", "on"),
                ] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                if inventory {
                    prog.commit_pending_analysis().unwrap();
                    let entries: Vec<_> = prog
                        .function_entries_canonical()
                        .iter()
                        .map(|entry| entry.addr.get_offset())
                        .collect();
                    for &prefix in prefixes {
                        assert!(entries.contains(&prefix), "{variant}/{endian}: {entries:x?}");
                        assert!(
                            !entries.contains(&(prefix + 6)),
                            "{variant}/{endian}: {entries:x?}"
                        );
                    }
                } else {
                    let bytes = std::fs::read(&path).unwrap();
                    let file = object::File::parse(&*bytes).unwrap();
                    let index = xrefs::build_measured(
                        &file,
                        prog.arch(),
                        prog.arch().translate(),
                        &[0x1000],
                    );
                    assert_eq!(
                        index.function_instruction_counts(prefixes, 100),
                        vec![4; prefixes.len()]
                    );
                    for &prefix in prefixes {
                        assert!(index.is_function_entry(prefix));
                        assert!(!index.is_function_entry(prefix + 6));
                        for at in [prefix + 10, prefix + 14] {
                            assert_eq!(index.function_containing(at), None);
                            assert!(index.refs_from_instruction(at).is_empty());
                        }
                    }
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn fingerprint_rendering_retains_live_interworking_and_body_records() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for late in [false, true] {
            let mut flags = vec!["interworkfp"];
            if late {
                flags.push("latecallee");
            }
            let path = fixture_variant("interwork-corpus-context", "arm", endian, &flags);
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                None,
            )
            .unwrap();
            for (name, value) in [("funcstart_patterns", "on"), ("aif", "on")] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            assert_eq!(
                index.function_instruction_counts(&[0x1500, 0x1600, 0x1700], 100),
                vec![4, 4, 4]
            );
            assert!(!index.is_function_entry(0x1506));
            assert!(index.refs_from_instruction(0x150a).is_empty());
            let calls: Vec<_> = index.refs_from_function(0x1700).into_iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| (r.from, r.to))
                .collect();
            assert_eq!(calls, vec![(0x1704, 0x1100)]);
            let space = prog.arch().manage().get_default_code_space().unwrap().clone();
            for (at, mode) in [(0x1500, 1), (0x1600, 0), (0x1700, 1)] {
                assert_eq!(
                    prog.arch().with_context_db_mut(|db| {
                        db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
                    }).unwrap(),
                    mode,
                    "{endian}/late={late}: {at:x}"
                );
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn later_interworking_replaces_provisional_frames_in_the_wrong_mode() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("stale-frame-isa", "arm", endian, &["staleisa"]);
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Arm),
                )
                .unwrap();
                for (name, value) in [("funcstart_patterns", patterns), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build_with_focus(
                    &file,
                    prog.arch(),
                    prog.arch().translate(),
                    &[0x1000],
                    &[0x1650],
                );
                let calls: Vec<_> = index
                    .refs_from_function(0x1500)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, r.to))
                    .collect();
                assert!(calls.is_empty(), "{endian}/{aif}/{patterns}: {calls:x?}");
                assert!(
                    index.refs_to(0x1700).is_empty(),
                    "{endian}/{aif}/{patterns}"
                );
                assert!(!index.is_function_entry(0x1700), "obsolete callee survived");
                let caller: Vec<_> = index
                    .refs_from_function(0x1650)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, r.to))
                    .collect();
                assert_eq!(caller, vec![(0x1658, 0x1500)]);
                for at in if endian == "little" {
                    vec![0x1500, 0x1502, 0x1506, 0x150a]
                } else {
                    vec![0x1500, 0x1504, 0x1508]
                } {
                    assert_eq!(
                        index.function_containing(at),
                        Some(0x1500),
                        "missing Thumb instruction {at:x}"
                    );
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn gap_discovered_interworking_rebuilds_obsolete_frame_bodies() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("gap-frame-isa", "arm", endian, &["staleisa", "stalegap"]);
        for patterns in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [("funcstart_patterns", patterns), ("aif", "on")] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index =
                xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            assert!(index.refs_to(0x1700).is_empty(), "{endian}/{patterns}");
            assert!(!index.is_function_entry(0x1700), "obsolete callee survived");
            let caller: Vec<_> = index
                .refs_from_function(0x1400)
                .into_iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| (r.from, r.to))
                .collect();
            assert_eq!(caller, vec![(0x140c, 0x1500)], "{endian}/{patterns}, count={}", index.instruction_count());
            assert_eq!(
                index.function_instruction_counts(&[0x1500, 0x1400], 100),
                vec![if endian == "little" { 4 } else { 3 }, 5],
                "{endian}/{patterns}",
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn interior_interworking_replaces_overlapping_provisional_instructions() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "interior-frame-isa",
            "arm",
            endian,
            &["staleisa", "interiorisa"],
        );
        let callee = if endian == "little" { 0x1502 } else { 0x1504 };
        for aif in ["off", "on"] {
            for patterns in ["off", "on"] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Arm),
                )
                .unwrap();
                for (name, value) in [("funcstart_patterns", patterns), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build_with_focus(
                    &file,
                    prog.arch(),
                    prog.arch().translate(),
                    &[0x1000],
                    &[0x1650],
                );
                let calls: Vec<_> = index
                    .refs_from_function(0x1500)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, r.to))
                    .collect();
                assert!(calls.is_empty(), "{endian}/{aif}/{patterns}: {calls:x?}");
                assert!(
                    index.refs_to(0x1700).is_empty(),
                    "{endian}/{aif}/{patterns}"
                );
                assert!(!index.is_function_entry(0x1700), "obsolete callee survived");
                let caller: Vec<_> = index
                    .refs_from_function(0x1650)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, r.to))
                    .collect();
                assert_eq!(caller, vec![(0x1658, callee)]);
                for at in if endian == "little" {
                    vec![0x1502, 0x1506, 0x150a]
                } else {
                    vec![0x1504, 0x1508]
                } {
                    assert_eq!(
                        index.function_containing(at),
                        Some(callee),
                        "missing Thumb instruction {at:x}"
                    );
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_frames_preserve_inventory_aif_gap_bounds() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["gapmode", "gapbranch"][..],
            &["gapmode", "gapbranch", "armcallee"][..],
            &["gapmode", "gapbranch", "gapbound", "armcallee"][..],
        ] {
            let path = fixture_variant("inventory-gap-bounds", "arm", endian, variants);
            for patterns in ["off", "on"] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Arm),
                )
                .unwrap();
                for (name, value) in [
                    ("listing", "on"),
                    ("funcstart_patterns", patterns),
                    ("aif", "on"),
                ] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                prog.commit_pending_analysis().unwrap();
                let entries: Vec<_> = prog
                    .function_entries_canonical()
                    .into_iter()
                    .map(|f| f.addr.get_offset())
                    .collect();
                assert_eq!(
                    entries.contains(&0x1400),
                    !variants.contains(&"gapbound"),
                    "{endian}/{patterns}/{variants:?}: {entries:x?}"
                );
                if patterns == "on" {
                    assert!(entries.contains(&0x1500), "{entries:x?}");
                    assert!(entries.contains(&0x1580), "{entries:x?}");
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn gap_interworking_reconciles_inventory_before_publishing_callees() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "gap-mode-inventory",
            "arm",
            endian,
            &["staleisa", "stalegap"],
        );
        for patterns in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [
                ("listing", "on"),
                ("funcstart_patterns", patterns),
                ("aif", "on"),
            ] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let entries: Vec<_> = prog
                .function_entries_canonical()
                .into_iter()
                .map(|f| f.addr.get_offset())
                .collect();
            assert!(
                entries.contains(&0x1400),
                "missing AIF caller: {endian}/{patterns}"
            );
            assert!(
                !entries.contains(&0x1700),
                "obsolete callee: {endian}/{patterns}: {entries:x?}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn interworking_invalidates_a_caller_after_its_prefix_was_extended() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "prefix-frame-isa",
            "arm",
            endian,
            &["staleisa", "stalegap", "prefixisa"],
        );
        for patterns in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [("funcstart_patterns", patterns), ("aif", "on")] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index =
                xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            assert!(index.refs_to(0x1700).is_empty(), "{endian}/{patterns}");
            assert!(!index.is_function_entry(0x1700), "obsolete callee survived");
            let caller: Vec<_> = index
                .refs_from_function(0x1400)
                .into_iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| (r.from, r.to))
                .collect();
            assert_eq!(
                caller,
                vec![(0x140c, 0x1500)],
                "{endian}/{patterns}, count={}",
                index.instruction_count()
            );
            assert_eq!(
                index.function_instruction_counts(&[0x1500, 0x1400], 100),
                vec![if endian == "little" { 4 } else { 3 }, 5],
                "{endian}/{patterns}",
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn an_unfocused_frame_can_invalidate_a_prior_decode_through_an_interior_call() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "unfocused-interior-mode",
            "arm",
            endian,
            &["staleisa", "interiorisa"],
        );
        for aif in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [("funcstart_patterns", "on"), ("aif", aif)] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index =
                xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            assert!(index.refs_to(0x1700).is_empty(), "{endian}/{aif}");
            assert!(!index.is_function_entry(0x1700), "obsolete callee survived");
            let callee = if endian == "little" { 0x1502 } else { 0x1504 };
            assert!(index
                .refs_to(callee)
                .iter()
                .any(|r| r.from == 0x1658 && r.kind == XrefKind::Call));
            assert_eq!(index.function_containing(callee), Some(callee));
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn an_extended_prefix_recomputes_the_bodys_pc_relative_references() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for recursive in [false, true] {
            let path = fixture_variant("prefix-pool-reference", "arm", endian, &["prefixpool"]);
            if recursive {
                let mut bytes = std::fs::read(&path).unwrap();
                let call = if endian == "little" {
                    0xebfffffbu32.to_le_bytes()
                } else {
                    0xebfffffbu32.to_be_bytes()
                };
                bytes[0x608..0x60c].copy_from_slice(&call);
                std::fs::write(&path, bytes).unwrap();
            }
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for name in ["funcstart_patterns", "aif"] {
                prog.arch_mut().set_kuna_option(name, "on").unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index =
                xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
            let refs: Vec<_> = index
                .refs_to(0x1780)
                .into_iter()
                .filter(|r| r.kind == XrefKind::Data)
                .collect();
            assert_eq!(refs.len(), 1, "{endian}: {refs:?}");
            assert_eq!(refs[0].from, 0x1504);
            assert_eq!(index.function_containing(0x1504), Some(0x14fc));
            assert!(!index.is_function_entry(0x1500));
            if recursive {
                assert!(index
                    .refs_to(0x14fc)
                    .iter()
                    .any(|r| r.from == 0x1508 && r.kind == XrefKind::Call));
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn pointer_discovery_refreshes_the_frame_corpus_before_aif() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let path = fixture_variant("pointer-fingerprint", "thumb", "little", &[]);
    let mut bytes = std::fs::read(&path).unwrap();
    let mut half = |at: usize, value: u16| {
        let off = at - 0x1000 + 0x100;
        bytes[off..off + 2].copy_from_slice(&value.to_le_bytes());
    };
    for at in [0x1100, 0x1108, 0x1110, 0x1680] {
        for (off, word) in [0xb084, 0x466f, 0xb004, 0x4770].into_iter().enumerate() {
            half(at + 2 * off, word);
        }
    }
    for (off, word) in [0xb084, 0x466f, 0xf7ff, 0xfc6c, 0xb004, 0x4770]
        .into_iter()
        .enumerate()
    {
        half(0x1720 + 2 * off, word);
    }
    bytes[0x8e0..0x8e4].copy_from_slice(&0x1681u32.to_le_bytes());
    std::fs::write(&path, bytes).unwrap();
    for aif in ["off", "on"] {
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(),
            "ARM:LE:32:v8",
            &[specs.to_str().unwrap().into()],
            Some(ArmIsa::Thumb),
        )
        .unwrap();
        for (name, value) in [
            ("listing", "on"),
            ("funcstart_patterns", "on"),
            ("aif", aif),
        ] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let entries: Vec<_> = prog
            .function_entries_canonical()
            .into_iter()
            .map(|f| f.addr.get_offset())
            .collect();
        assert!(
            entries.contains(&0x1680),
            "missing pointer target: {entries:x?}"
        );
        assert_eq!(
            entries.contains(&0x1720),
            aif == "on",
            "AIF entry after pointer discovery: {entries:x?}"
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn recovered_interworking_preserves_interior_focus_modes() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("interior-focus-mode", "arm", endian, &["focusmode"]);
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        for aif in ["off", "on"] {
            for (patterns, focus) in [
                ("off", &[0x1500, 0x1584][..]),
                ("on", &[0x1500, 0x1584][..]),
                ("on", &[0x1500, 0x1582, 0x1584, 0x1586, 0x1600][..]),
            ] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    Some(ArmIsa::Arm),
                )
                .unwrap();
                for (name, value) in [("funcstart_patterns", patterns), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                let index = xrefs::build_with_focus(
                    &file,
                    prog.arch(),
                    prog.arch().translate(),
                    &[0x1000],
                    focus,
                );
                let calls: Vec<_> = index
                    .refs_to(0x1300)
                    .into_iter()
                    .filter(|r| r.kind == XrefKind::Call)
                    .map(|r| (r.from, index.function_containing(r.from)))
                    .collect();
                assert_eq!(
                    calls,
                    vec![(0x1584, Some(0x1580))],
                    "{endian}/{aif}/{patterns}/{focus:x?}"
                );
                for at in [0x1582, 0x1584, 0x1586] {
                    assert!(!index.is_function_entry(at), "interior entry {at:x}");
                }
                if focus.contains(&0x1600) {
                    let calls: Vec<_> = index
                        .refs_from_function(0x1600)
                        .into_iter()
                        .filter(|r| r.kind == XrefKind::Call)
                        .map(|r| (r.from, r.to))
                        .collect();
                    assert_eq!(
                        calls,
                        vec![(0x1604, 0x1200)],
                        "independent focus: {endian}/{aif}/{patterns}"
                    );
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn repeated_xrefs_preserve_recovered_body_modes() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("repeated-frame-mode", "arm", endian,
            &["focusmode", "rootmode", "splitframe", "recoveredpublish"]);
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
        ).unwrap();
        for (name, value) in [("listing", "off"), ("funcstart_patterns", "on"), ("aif", "off")] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        for iteration in 0..3 {
            let index = xrefs::build_with_focus(
                &file, prog.arch(), prog.arch().translate(), &[0x1000], &[0x1600],
            );
            for (root, at, to) in [(0x1600, 0x1750, 0x1100), (0x1680, 0x1684, 0x1300)] {
                let calls: Vec<_> = index.refs_from_function(root).into_iter()
                    .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                assert_eq!(calls, vec![(at, to)], "{endian}/{iteration}: {root:x}");
            }
        }
        let space = prog.arch().manage().get_default_code_space().unwrap().clone();
        for (at, mode) in [(0x1600, 0), (0x1750, 0), (0x1753, 0), (0x1680, 1), (0x1686, 1)] {
            assert_eq!(prog.arch().with_context_db_mut(|db| {
                db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
            }).unwrap(), mode, "{endian}: {at:x}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_xref_publication_preserves_later_decompilation() {
    use kuna_base::address::Address;
    use kuna_console::project::{decompile_entry, DecompileOptions};
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("decompiled-frame-mode", "arm", endian,
            &["focusmode", "rootmode", "splitframe", "recoveredpublish"]);
        for patterns in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
            ).unwrap();
            for (name, value) in [("listing", "off"), ("funcstart_patterns", patterns), ("aif", "off")] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index = xrefs::build_with_focus(
                &file, prog.arch(), prog.arch().translate(), &[0x1000], &[0x1600],
            );
            assert!(index.refs_from_function(0x1600).iter()
                .any(|r| r.kind == XrefKind::Call && r.from == 0x1750 && r.to == 0x1100));
            let space = prog.arch().manage().get_default_code_space().unwrap().clone();
            let entry = prog.resolve_address(&Address::new(space, 0x1600)).unwrap();
            let result = decompile_entry(&mut prog, entry, &DecompileOptions::default());
            assert!(result.error.is_none(), "{endian}/{patterns}: {:?}", result.error);
            let code = result.code.unwrap();
            assert!(code.contains("sub_1100(") && !code.contains("halt_"),
                "{endian}/{patterns}: {code}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn xref_publication_preserves_established_blocks_and_thumb_callees() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("xref-established-mode", "arm", endian,
            &["focusmode", "rootmode", "splitframe", "focusblocks", "xrefpublish"]);
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
        ).unwrap();
        for (name, value) in [("listing", "off"), ("funcstart_patterns", "on"), ("aif", "off")] {
            prog.arch_mut().set_kuna_option(name, value).unwrap();
        }
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let index = xrefs::build_with_focus(
            &file, prog.arch(), prog.arch().translate(), &[0x1000], &[],
        );
        assert!(index.refs_from_function(0x1500).iter()
            .any(|r| r.kind == XrefKind::Call && r.from == 0x1750 && r.to == 0x1100));
        let space = prog.arch().manage().get_default_code_space().unwrap().clone();
        for (at, mode) in [(0x1500, 0), (0x1750, 0), (0x1753, 0), (0x1758, 0),
            (0x1680, 1), (0x1684, 1), (0x1686, 1)] {
            assert_eq!(prog.arch().with_context_db_mut(|db| {
                db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
            }).unwrap(), mode, "{endian}: {at:x}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_preserves_aif_seed_modes_through_backward_interworking() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("aif-rebuilt-mode", "arm", endian, &["aifseedmode"]);
        let mut prog = bootstrap_from_object_with_isa(
            path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
        ).unwrap();
        for name in ["listing", "funcstart_patterns", "aif"] {
            prog.arch_mut().set_kuna_option(name, "on").unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        assert!(prog.function_entries_canonical().iter()
            .any(|entry| entry.addr.get_offset() == 0x1500));
        let space = prog.arch().manage().get_default_code_space().unwrap().clone();
        for (at, mode) in [(0x1500, 0), (0x1508, 0), (0x1750, 0), (0x1753, 0),
            (0x1450, 1), (0x1454, 1), (0x1456, 1)] {
            assert_eq!(prog.arch().with_context_db_mut(|db| {
                db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
            }).unwrap(), mode, "{endian}: {at:x}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn frame_publication_restores_gaps_and_preserves_thumb_instruction_spans() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("published-gap-mode", "arm", endian,
            &["focusmode", "rootmode", "splitframe", "unclaimedmode"]);
        for aif in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
            ).unwrap();
            for (name, value) in [("listing", "on"), ("funcstart_patterns", "on"), ("aif", aif)] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let entries: Vec<_> = prog.function_entries_canonical().iter()
                .map(|entry| entry.addr.get_offset()).collect();
            assert!(!entries.contains(&0x1750), "the caller must remain undiscovered");
            assert!(entries.contains(&0x1680));
            let space = prog.arch().manage().get_default_code_space().unwrap().clone();
            for (at, mode) in [(0x1750, 0), (0x1754, 0), (0x17fc, 0),
                (0x1680, 1), (0x1684, 1), (0x1686, 1), (0x1689, 1)] {
                assert_eq!(prog.arch().with_context_db_mut(|db| {
                    db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
                }).unwrap(), mode, "{endian}/{aif}: {at:x}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn an_unframed_focus_preserves_its_blocks_and_an_interior_callee_focus() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("unframed-focused-blocks", "arm", endian,
            &["focusmode", "rootmode", "splitframe", "focusblocks"]);
        for aif in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
            ).unwrap();
            for (name, value) in [("listing", "off"), ("funcstart_patterns", "on"), ("aif", aif)] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index = xrefs::build_with_focus(
                &file, prog.arch(), prog.arch().translate(), &[0x1000], &[0x1500, 0x1684],
            );
            for (root, at, callee) in [(0x1500, 0x1750, 0x1100), (0x1680, 0x1684, 0x1300)] {
                let calls: Vec<_> = index.refs_from_function(root).into_iter()
                    .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                assert_eq!(calls, vec![(at, callee)], "{endian}/{aif}: {root:x}");
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_retains_discontiguous_frame_and_intervening_callee_modes() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for backward in [false, true] {
            let mut flags = vec!["focusmode", "rootmode", "splitframe"];
            if backward { flags.push("backwardframe"); }
            let path = fixture_variant("inventory-split-frame-mode", "arm", endian, &flags);
            for aif in ["off", "on"] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
                ).unwrap();
                for (name, value) in [("listing", "on"), ("funcstart_patterns", "on"), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                prog.commit_pending_analysis().unwrap();
                let space = prog.arch().manage().get_default_code_space().unwrap().clone();
                for (at, mode) in [(0x1600, 0), (0x1750, 0), (0x1680, 1), (0x1684, 1)] {
                    assert_eq!(prog.arch().with_context_db_mut(|db| {
                        db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
                    }).unwrap(), mode, "{endian}/{aif}/backward={backward}: {at:x}");
                }
                let seeds: Vec<_> = prog.function_entries_canonical().iter()
                    .map(|entry| entry.addr.get_offset()).collect();
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &seeds);
                assert_eq!(index.function_instruction_counts(&[0x1600, 0x1680], 100), vec![4, 4]);
                for (root, at, target) in [(0x1600, 0x1750, 0x1100), (0x1680, 0x1684, 0x1300)] {
                    let calls: Vec<_> = index.refs_from_function(root).into_iter()
                        .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                    assert_eq!(calls, vec![(at, target)], "{endian}/{aif}/backward={backward}");
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn inventory_admits_frames_with_independent_or_directly_called_modes() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for (variants, expected_mode) in [
            (&["focusmode", "rootmode", "backwardframe"][..], 0),
            (&["staleisa"][..], 1),
        ] {
            let path = fixture_variant("inventory-admission-mode", "arm", endian, variants);
            for aif in ["off", "on"] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], None,
                ).unwrap();
                for (name, value) in [("listing", "on"), ("funcstart_patterns", "on"), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                prog.commit_pending_analysis().unwrap();
                let space = prog.arch().manage().get_default_code_space().unwrap().clone();
                let mode = prog.arch().with_context_db_mut(|db| {
                    db.get_variable_value(b"TMode", &Address::new(space.clone(), 0x1500))
                }).unwrap();
                assert_eq!(mode, expected_mode, "{endian}/{aif}/{variants:?}");
                let seeds: Vec<_> = prog.function_entries_canonical().iter()
                    .map(|entry| entry.addr.get_offset()).collect();
                assert!(seeds.contains(&0x1500));
                assert!(!seeds.contains(&0x1700), "obsolete ARM call must not publish a callee");
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build_measured(&file, prog.arch(), prog.arch().translate(), &seeds);
                let calls: Vec<_> = index.refs_from_function(0x1500).into_iter()
                    .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                if expected_mode == 0 {
                    assert_eq!(calls, vec![(0x1504, 0x1200)]);
                    assert!(index.refs_to(0x1300).iter().any(|r| r.from == 0x1384 && r.kind == XrefKind::Call));
                } else {
                    assert!(calls.is_empty(), "direct BLX must supersede the frame's saved ARM mode");
                    assert_eq!(index.function_instruction_counts(&[0x1500], 100), vec![if endian == "big" { 3 } else { 4 }]);
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn inventory_publishes_frame_modes_for_seeded_xrefs() {
    use kuna_base::address::Address;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant("published-frame-mode", "arm", endian, &["focusmode", "rootmode"]);
        for aif in ["off", "on"] {
            for isa in [None, Some(ArmIsa::Arm)] {
                let mut prog = bootstrap_from_object_with_isa(
                    path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], isa,
                ).unwrap();
                for (name, value) in [("listing", "on"), ("funcstart_patterns", "on"), ("aif", aif)] {
                    prog.arch_mut().set_kuna_option(name, value).unwrap();
                }
                prog.commit_pending_analysis().unwrap();
                let space = prog.arch().manage().get_default_code_space().unwrap().clone();
                for at in [0x1600, 0x1604, 0x1608] {
                    let mode = prog.arch().with_context_db_mut(|db| {
                        db.get_variable_value(b"TMode", &Address::new(space.clone(), at))
                    }).unwrap();
                    assert_eq!(mode, 0, "{endian}/{aif}: inventory changed frame mode at {at:x}");
                }
                let seeds: Vec<_> = prog.function_entries_canonical().iter()
                    .map(|entry| entry.addr.get_offset()).collect();
                assert!(seeds.contains(&0x1600));
                assert!(seeds.contains(&0x1580));
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds);
                for (entry, site, callee) in [(0x1600, 0x1604, 0x1200), (0x1580, 0x1584, 0x1300)] {
                    let calls: Vec<_> = index.refs_from_function(entry).into_iter()
                        .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                    assert_eq!(calls, vec![(site, callee)], "{endian}/{aif}: {entry:x}");
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn inventory_frame_rebuild_preserves_established_isa_modes() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for variants in [
            &["focusmode", "establishedmode"][..],
            &["focusmode", "establishedmode", "establishedsplit"][..],
        ] {
            let path = fixture_variant("established-frame-mode", "arm", endian, variants);
            for aif in ["off", "on"] {
                for patterns in ["off", "on"] {
                    let mut prog = bootstrap_from_object_with_isa(
                        path.to_str().unwrap(),
                        target,
                        &[specs.to_str().unwrap().into()],
                        Some(ArmIsa::Arm),
                    )
                    .unwrap();
                    for (name, value) in [
                        ("listing", "on"),
                        ("funcstart_patterns", patterns),
                        ("aif", aif),
                    ] {
                        prog.arch_mut().set_kuna_option(name, value).unwrap();
                    }
                    prog.commit_pending_analysis().unwrap();
                    let entries: Vec<_> = prog
                        .function_entries_canonical()
                        .into_iter()
                        .map(|f| f.addr.get_offset())
                        .collect();
                    if patterns == "on" {
                        assert!(
                            entries.contains(&0x1200),
                            "{endian}/{aif}/{patterns}/{variants:?}: {entries:x?}"
                        );
                    }
                    let bytes = std::fs::read(&path).unwrap();
                    let file = object::File::parse(&*bytes).unwrap();
                    let index =
                        xrefs::build(&file, prog.arch(), prog.arch().translate(), &[0x1000]);
                    let calls: Vec<_> = index
                        .refs_to(0x1200)
                        .iter()
                        .filter(|r| r.kind == XrefKind::Call)
                        .map(|r| r.from)
                        .collect();
                    assert_eq!(
                        calls,
                        vec![0x1604],
                        "{endian}/{aif}/{patterns}/{variants:?}"
                    );
                    if patterns == "on" {
                        assert!(entries.contains(&0x1500));
                        assert!(entries.contains(&0x1580));
                        assert!(index
                            .refs_to(0x1300)
                            .iter()
                            .any(|r| r.from == 0x1584 && r.kind == XrefKind::Call));
                    }
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn replacement_prefix_roots_retain_original_isa_modes() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture_variant(
            "replacement-prefix-mode",
            "arm",
            endian,
            &["prefixmode"],
        );
        for focus in [&[0x1200][..], &[0x1600][..]] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("aif", "on").unwrap();
            prog.arch_mut()
                .set_kuna_option("funcstart_patterns", "on")
                .unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index = xrefs::build_with_focus(
                &file,
                prog.arch(),
                prog.arch().translate(),
                &[0x1000],
                focus,
            );
            let calls: Vec<_> = index
                .refs_from_function(0x1600)
                .into_iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| (r.from, r.to))
                .collect();
            assert_eq!(calls, vec![(0x1608, 0x1200)], "{endian}/{focus:x?}");
            assert!(
                index.refs_to(0x1946).is_empty(),
                "phantom Thumb branch: {endian}"
            );
            assert!(!index.is_function_entry(0x1604));
            for at in [0x1580, 0x1582, 0x1584] {
                assert_eq!(index.function_containing(at), Some(0x1580));
            }
            assert!(index.refs_to(0x1580).iter()
                .any(|r| r.from == 0x1508 && r.kind == XrefKind::Call));
        }
        std::fs::remove_file(path).unwrap();
    }
}
