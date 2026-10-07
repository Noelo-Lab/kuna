//! ARM discovery changes modes only on reached instruction spans.
use kuna_analysis::listing::xrefs::{self, XrefKind};
use kuna_base::address::Address;
use kuna_console::engine::bootstrap_from_object_with_isa;
use kuna_console::project::{decompile_entry, DecompileOptions};
use std::path::PathBuf;
use std::process::Command;

fn bootstrap_frames(
    path: &str,
    target: &str,
    specs: &[String],
    isa: Option<kuna_console::engine::ArmIsa>,
) -> kuna_base::error::KunaResult<kuna_console::engine::ConsoleProgram> {
    let mut prog = bootstrap_from_object_with_isa(path, target, specs, isa)?;
    prog.arch_mut().set_kuna_option("armframes", "on")?;
    Ok(prog)
}

fn fixture(name: &str, generator: &str, args: &[&str]) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("arm-context-{name}-{}.elf", std::process::id()));
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures")
        .join(generator);
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .args(args)
        .status()
        .unwrap()
        .success());
    path
}

#[test]
fn ordinary_aif_does_not_publish_unreachable_interworking_modes() {
    use kuna_console::engine::ArmIsa;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture("ordinary-aif", "arm_xref_roots.py", &["thumb", endian, "staleprefix", "backwardblx"]);
        for frames in ["off", "on"] {
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], Some(ArmIsa::Thumb),
            ).unwrap();
            for (name, value) in [("listing", "off"), ("funcstart_patterns", "off"), ("armframes", frames), ("aif", "on")] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let space = prog.arch().manage().get_default_code_space().unwrap().clone();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            for _ in 0..2 {
                let index = xrefs::build_with_focus(&file, prog.arch(), prog.arch().translate(), &[0x1000], &[0x1700]);
                let calls: Vec<_> = index.refs_from_function(0x1700).into_iter()
                    .filter(|r| r.kind == XrefKind::Call).map(|r| (r.from, r.to)).collect();
                assert_eq!(calls, vec![(0x1704, 0x1100)]);
                for at in [0x14f0, 0x1500, 0x1600, 0x1700] {
                    assert_eq!(prog.arch().with_context_db_mut(|db| {
                        db.get_variable_value(b"TMode", &Address::new(space.clone(), at)).unwrap()
                    }), 1, "{endian}/frames={frames} at {at:x}");
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn ordinary_decompilation_preserves_a_backward_blx_callers_mode() {
    use kuna_console::engine::ArmIsa;
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture("ordinary-backward-call", "arm_xref_roots.py", &["arm", endian, "focusmode", "rootmode", "backwardframe"]);
        for isa in [None, Some(ArmIsa::Arm)] {
            for listing in ["off", "on"] {
                for aif in ["off", "on"] {
                    for frames in ["off", "on"] {
                        let mut prog = bootstrap_from_object_with_isa(
                            path.to_str().unwrap(), target, &[specs.to_str().unwrap().into()], isa,
                        ).unwrap();
                        for (name, value) in [("listing", listing), ("aif", aif), ("funcstart_patterns", "on"), ("armframes", frames)] {
                            prog.arch_mut().set_kuna_option(name, value).unwrap();
                        }
                        prog.commit_pending_analysis().unwrap();
                        let space = prog.arch().manage().get_default_code_space().unwrap().clone();
                        for iteration in 0..2 {
                            let entry = prog.resolve_address(&Address::new(space.clone(), 0x1600)).unwrap();
                            let result = decompile_entry(&mut prog, entry, &DecompileOptions::default());
                            assert!(result.error.is_none(), "{:?}", result.error);
                            let code = result.code.unwrap();
                            assert!(code.contains("sub_1380(") && !code.lines().any(|line| line.trim_start().starts_with('*')),
                                "{endian}/{isa:?}/listing={listing}/aif={aif}/frames={frames}/iteration={iteration}: {code}");
                            for at in [0x1600, 0x1604, 0x1608] {
                                assert_eq!(prog.arch().with_context_db_mut(|db| db.get_variable_value(b"TMode", &Address::new(space.clone(), at))).unwrap(), 0);
                            }
                            assert_eq!(prog.arch().with_context_db_mut(|db| db.get_variable_value(b"TMode", &Address::new(space.clone(), 0x1380))).unwrap(), 1);
                        }
                    }
                }
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn recovered_xrefs_leave_unclaimed_modes_and_later_c_unchanged() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for corpus in [false, true] {
            let path = fixture(
                "unclaimed",
                "arm_xref_roots.py",
                &[
                    "arm",
                    endian,
                    "focusmode",
                    "rootmode",
                    "splitframe",
                    "unclaimedmode",
                ],
            );
            if !corpus {
                let mut bytes = std::fs::read(&path).unwrap();
                let ret = if endian == "big" {
                    0xE12FFF1Eu32.to_be_bytes()
                } else {
                    0xE12FFF1Eu32.to_le_bytes()
                };
                bytes[0x100..0x104].copy_from_slice(&ret);
                std::fs::write(&path, bytes).unwrap();
            }
            for aif in ["off", "on"] {
                let mut prog = bootstrap_frames(
                    path.to_str().unwrap(),
                    target,
                    &[specs.to_str().unwrap().into()],
                    None,
                )
                .unwrap();
                for (key, value) in [
                    ("listing", "off"),
                    ("aif", aif),
                    ("funcstart_patterns", "on"),
                ] {
                    prog.arch_mut().set_kuna_option(key, value).unwrap();
                }
                prog.commit_pending_analysis().unwrap();
                let bytes = std::fs::read(&path).unwrap();
                let file = object::File::parse(&*bytes).unwrap();
                let index = xrefs::build_with_focus(
                    &file,
                    prog.arch(),
                    prog.arch().translate(),
                    &[0x1000],
                    &[],
                );
                assert!(index
                    .refs_from_function(0x1680)
                    .iter()
                    .any(|r| r.kind == XrefKind::Call && r.from == 0x1684 && r.to == 0x1300));
                let space = prog
                    .arch()
                    .manage()
                    .get_default_code_space()
                    .unwrap()
                    .clone();
                for at in 0x1750..0x1800 {
                    assert_eq!(
                        prog.arch()
                            .with_context_db_mut(|db| db
                                .get_variable_value(b"TMode", &Address::new(space.clone(), at)))
                            .unwrap(),
                        0,
                        "{endian}/{aif}/corpus={corpus}: {at:x}"
                    );
                }
                let entry = prog.resolve_address(&Address::new(space, 0x1750)).unwrap();
                let result = decompile_entry(&mut prog, entry, &DecompileOptions::default());
                assert!(result.error.is_none(), "{:?}", result.error);
                let code = result.code.unwrap();
                assert!(
                    code.contains("sub_1100(") && !code.contains("halt_"),
                    "{code}"
                );
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn queued_thumb_callee_keeps_its_distant_block_across_an_arm_seed() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture(
            "queued",
            "arm_context_graph.py",
            &[endian, "0", "thumb", "0"],
        );
        for patterns in ["off", "on"] {
            let mut prog = bootstrap_frames(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                None,
            )
            .unwrap();
            for (key, value) in [
                ("listing", "off"),
                ("aif", "off"),
                ("funcstart_patterns", patterns),
            ] {
                prog.arch_mut().set_kuna_option(key, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            let index = xrefs::build_with_focus(
                &file,
                prog.arch(),
                prog.arch().translate(),
                &[0x1000, 0x1600],
                &[],
            );
            let calls: Vec<_> = index
                .refs_to(0x1200)
                .iter()
                .filter(|r| r.kind == XrefKind::Call)
                .map(|r| r.from)
                .collect();
            assert_eq!(calls, [0x1750], "{endian}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn generated_graphs_preserve_calls_modes_and_c_across_layouts_and_reuse() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let layouts = [
        (0x1000, 0x1600, 0x1580, 0x1750),
        (0x1700, 0x1600, 0x1380, 0x1500),
        (0x1400, 0x1500, 0x1680, 0x1780),
        (0x1800, 0x1400, 0x1580, 0x1380),
    ];
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        for (layout, (entry, seed, callee, tail)) in layouts.into_iter().enumerate() {
            for isa in ["thumb", "arm"] {
                for leaves in [0, 14, 15, 16] {
                    for discovery in ["seed", "frame"] {
                        let path = fixture(
                            "matrix",
                            "arm_context_graph.py",
                            &[
                                endian,
                                &layout.to_string(),
                                isa,
                                &leaves.to_string(),
                                discovery,
                            ],
                        );
                        let bytes = std::fs::read(&path).unwrap();
                        let file = object::File::parse(&*bytes).unwrap();
                        let mut fresh = bootstrap_frames(
                            path.to_str().unwrap(),
                            target,
                            &[specs.to_str().unwrap().into()],
                            None,
                        )
                        .unwrap();
                        for key in ["listing", "aif", "funcstart_patterns"] {
                            fresh.arch_mut().set_kuna_option(key, "off").unwrap();
                        }
                        fresh.commit_pending_analysis().unwrap();
                        let space = fresh
                            .arch()
                            .manage()
                            .get_default_code_space()
                            .unwrap()
                            .clone();
                        let unrelated =
                            fresh.resolve_address(&Address::new(space, 0x1b00)).unwrap();
                        let baseline =
                            decompile_entry(&mut fresh, unrelated, &DecompileOptions::default());
                        assert!(baseline.error.is_none(), "{:?}", baseline.error);
                        let baseline = baseline.code.unwrap();
                        // Initial Listing interworking follows the separate flowmode policy.
                        // Inventory cases here introduce interworking through frame recovery.
                        let listings: &[&str] = if discovery == "frame" {
                            &["off", "on"]
                        } else {
                            &["off"]
                        };
                        for &listing in listings {
                            for aif in ["off", "on"] {
                                let label = format!("{endian}/layout={layout}/{isa}/leaves={leaves}/listing={listing}/aif={aif}/{discovery}");
                                let mut prog = bootstrap_frames(
                                    path.to_str().unwrap(),
                                    target,
                                    &[specs.to_str().unwrap().into()],
                                    None,
                                )
                                .unwrap();
                                for (key, value) in [
                                    ("listing", listing),
                                    ("aif", aif),
                                    ("funcstart_patterns", "on"),
                                ] {
                                    prog.arch_mut().set_kuna_option(key, value).unwrap();
                                }
                                prog.commit_pending_analysis().unwrap();
                                let mut expected =
                                    vec![(entry + 4, callee), (tail, 0x1200), (0x1a00, 0x1100)];
                                expected.extend(
                                    (0..leaves).map(|i| (entry + 8 + 4 * i, 0x2000 + 16 * i)),
                                );
                                let initial = if discovery == "frame" { 0x1900 } else { entry };
                                if discovery == "frame" {
                                    expected.extend(
                                        (0..leaves).map(|i| (initial + 4 + 4 * i, 0x2000 + 16 * i)),
                                    );
                                }
                                expected.sort_unstable();
                                for iteration in 0..3 {
                                    let seeds = if iteration == 1 {
                                        [seed, initial]
                                    } else {
                                        [initial, seed]
                                    };
                                    let focus = if iteration == 2 {
                                        vec![callee + if isa == "thumb" { 2 } else { 4 }]
                                    } else {
                                        Vec::new()
                                    };
                                    let index = xrefs::build_with_focus(
                                        &file,
                                        prog.arch(),
                                        prog.arch().translate(),
                                        &seeds,
                                        &focus,
                                    );
                                    let mut calls: Vec<_> = (0x1000..0x2400)
                                        .flat_map(|at| index.refs_from_instruction(at))
                                        .filter(|r| r.kind == XrefKind::Call)
                                        .map(|r| (r.from, r.to))
                                        .collect();
                                    calls.sort_unstable();
                                    assert_eq!(calls, expected, "{label}/iteration={iteration}");
                                    if iteration < 2 {
                                        assert_eq!(
                                            index.function_containing(tail),
                                            Some(callee),
                                            "{label}"
                                        );
                                    }
                                }
                                let space = prog
                                    .arch()
                                    .manage()
                                    .get_default_code_space()
                                    .unwrap()
                                    .clone();
                                for at in 0x1000..0x2400 {
                                    let thumb = isa == "thumb"
                                        && ((callee..callee + 4).contains(&at)
                                            || (tail..tail + 6).contains(&at)
                                            || (0x1200..0x1202).contains(&at));
                                    let mode = prog
                                        .arch()
                                        .with_context_db_mut(|db| {
                                            db.get_variable_value(
                                                b"TMode",
                                                &Address::new(space.clone(), at),
                                            )
                                        })
                                        .unwrap();
                                    assert_eq!(mode, u32::from(thumb), "{label}: mode at {at:x}");
                                }
                                let entry =
                                    prog.resolve_address(&Address::new(space, 0x1b00)).unwrap();
                                let result =
                                    decompile_entry(&mut prog, entry, &DecompileOptions::default());
                                assert!(result.error.is_none(), "{label}: {:?}", result.error);
                                let code = result.code.unwrap();
                                assert_eq!(code, baseline, "{label}: unrelated C changed");
                                assert!(
                                    code.contains("sub_1100(") && !code.contains("halt_"),
                                    "{label}: {code}"
                                );
                            }
                        }
                        std::fs::remove_file(path).unwrap();
                    }
                }
            }
        }
    }
}

#[test]
fn failed_callee_decodes_preserve_context_values_boundaries_and_write_policy() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        let path = fixture(
            "failed",
            "arm_context_graph.py",
            &[endian, "0", "thumb", "0"],
        );
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x680..0x690].fill(0xff);
        std::fs::write(&path, &bytes).unwrap();
        let mut prog = bootstrap_frames(
            path.to_str().unwrap(),
            target,
            &[specs.to_str().unwrap().into()],
            None,
        )
        .unwrap();
        for (key, value) in [
            ("listing", "off"),
            ("aif", "off"),
            ("funcstart_patterns", "on"),
        ] {
            prog.arch_mut().set_kuna_option(key, value).unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let space = prog
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone();
        let bounds = || {
            prog.arch().with_context_db_mut(|db| {
                let (words, first, last) =
                    db.get_context_bounds(&Address::new(space.clone(), 0x1580));
                (words.to_vec(), first, last)
            })
        };
        let before = bounds();
        for _ in 0..2 {
            let index = xrefs::build_measured(
                &file,
                prog.arch(),
                prog.arch().translate(),
                &[0x1000, 0x1600],
            );
            assert_eq!(
                index.function_instruction_counts(&[0x1580], 100),
                [0],
                "fixture must fail decoding"
            );
            assert_eq!(bounds(), before, "{endian}: failed decode changed context");
        }
        let _probe = prog.arch().translate().context_scope();
        kuna_analysis::listing::decode::decode_one(
            prog.arch().translate(),
            0x1004,
            &space,
            false,
            false,
        )
        .unwrap();
        assert_eq!(
            prog.arch()
                .with_context_db_mut(
                    |db| db.get_variable_value(b"TMode", &Address::new(space, 0x1580))
                )
                .unwrap(),
            1,
            "write mask was not restored"
        );
        std::fs::remove_file(path).unwrap();
    }
}
