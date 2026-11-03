use crate::common;
use kuna_analysis::analyzers::entry::kuna_fdeinterior::{eligible_fde_bodies, scan_fde_bodies};
use kuna_analysis::analyzers::entry::{scan_eh_frame_landing_pads, scan_eh_frame_starts};
use std::process::Command;
#[path = "fixtures/elf_unwind_fixture.rs"]
mod fixture;

#[test]
fn bounds_follow_elf_class_and_byte_order() {
    for little in [false, true] {
        for wide in [false, true] {
            for pcrel in [false, true] {
                let bytes = fixture::image(little, wide, pcrel, false);
                let file = object::File::parse(bytes.as_slice()).unwrap();
                assert_eq!(
                    scan_eh_frame_starts(&file),
                    [0x1000],
                    "little={little} wide={wide} pcrel={pcrel}"
                );
                assert_eq!(
                    scan_fde_bodies(&file),
                    [(0x1000, 0x1024)],
                    "little={little} wide={wide} pcrel={pcrel}"
                );
                assert_eq!(eligible_fde_bodies(&file, &bytes), [(0x1000, 0x1024)]);
            }
        }
    }
}

#[test]
fn pc_read_keeps_the_complete_body_and_result() {
    for little in [false, true] {
        let path = common::scratch_file("elf-pc-read", "elf");
        std::fs::write(&path, fixture::image(little, false, true, false)).unwrap();
        let (code, err, status) = common::run_kuna(&[
            "decompile",
            path.to_str().unwrap(),
            "bump",
            "--mode",
            "fast",
        ]);
        assert_eq!(status, 0, "{code}\n{err}");
        assert!(code.contains("return a0 * 3 + 7;"), "{code}");
        assert!(
            !code.contains("sub_100c") && !code.contains("halt_baddata"),
            "{code}"
        );
        std::fs::remove_file(&path).unwrap();
        if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
            continue;
        }
        let c = common::scratch_file("elf-pc-read", "c");
        let exe = common::scratch_file("elf-pc-read", "out");
        std::fs::write(&c, format!("{code}\nint main(void) {{ for(int x=-10;x<=10;x++) if(bump(x)!=x*3+7) return 1; return 0; }}\n")).unwrap();
        common::process::required_output(
            Command::new("cc").args(["-O2", c.to_str().unwrap(), "-o", exe.to_str().unwrap()]),
        );
        assert!(Command::new(&exe).status().unwrap().success());
        for file in [c, exe] {
            std::fs::remove_file(file).unwrap();
        }
    }
}

#[test]
fn pc_read_needs_an_owned_body_without_a_named_inner_entry() {
    for little in [false, true] {
        for inner in [false, true] {
            let bytes = fixture::image(little, false, true, inner);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(eligible_fde_bodies(&file, &bytes).is_empty(), inner);
            let path = common::scratch_file("elf-pc-read-call", "elf");
            std::fs::write(&path, bytes).unwrap();
            let mut args = vec![
                "decompile",
                path.to_str().unwrap(),
                "bump",
                "--mode",
                "fast",
            ];
            if !inner {
                args.extend(["--option", "fdeinterior", "off"]);
            }
            let (code, err, status) = common::run_kuna(&args);
            assert_eq!(status, 0, "{code}\n{err}");
            assert!(
                code.contains(if inner { "inner(" } else { "sub_100c(" }),
                "{code}"
            );
            assert!(!code.contains("return a0 * 3 + 7;"), "{code}");
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn stage_image_matches_authored_bytes() {
    assert_eq!(
        include_bytes!("fixtures/ppc-pc-read.elf").as_slice(),
        fixture::image(false, false, true, false)
    );
}

#[test]
fn landing_pad_fields_follow_elf_class_and_byte_order() {
    for little in [false, true] {
        for wide in [false, true] {
            for pcrel in [false, true] {
                let bytes = fixture::lsda_image(little, wide, pcrel);
                let file = object::File::parse(bytes.as_slice()).unwrap();
                assert_eq!(scan_eh_frame_starts(&file), [0x1000]);
                assert_eq!(
                    scan_eh_frame_landing_pads(&file),
                    [0x100c],
                    "little={little} wide={wide} pcrel={pcrel}"
                );
            }
        }
    }
}

#[test]
fn fde_only_entries_keep_their_emitted_body() {
    for little in [false, true] {
        let mut bytes = fixture::image(little, false, true, false);
        let word = |v: u32| {
            if little {
                v.to_le_bytes()
            } else {
                v.to_be_bytes()
            }
        };
        let shoff = if little {
            u32::from_le_bytes(bytes[32..36].try_into().unwrap())
        } else {
            u32::from_be_bytes(bytes[32..36].try_into().unwrap())
        } as usize;
        bytes[24..28].copy_from_slice(&word(0));
        bytes[shoff + 3 * 40 + 4..shoff + 3 * 40 + 8].copy_from_slice(&word(1));
        let path = common::scratch_file("elf-fde-only", "elf");
        std::fs::write(&path, bytes).unwrap();
        let (out, err, status) = common::run_kuna(&[
            "decompile-all",
            path.to_str().unwrap(),
            "--json",
            "--mode",
            "fast",
        ]);
        assert_eq!(status, 0, "{out}\n{err}");
        let data: serde_json::Value = serde_json::from_str(&out).unwrap();
        let functions = data["functions"].as_array().unwrap();
        assert!(
            functions.iter().any(|f| f["address"] == 0x1000u64
                && f["code"]
                    .as_str()
                    .is_some_and(|c| c.contains("return a0 * 3 + 7;"))),
            "{out}"
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn shared_unwind_does_not_suppress_unnamed_pic_stubs_in_text() {
    use object::{Object, ObjectSection};
    for little in [false, true] {
        for code in [
            [0x817e0000u32, 0x7d6903a6, 0x4e800420, 0x60000000],
            [0x3d7e0000u32, 0x816b0000, 0x7d6903a6, 0x4e800420],
        ] {
            let mut bytes = fixture::image(little, false, true, false);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            let offset = file
                .section_by_name(".text")
                .unwrap()
                .file_range()
                .unwrap()
                .0 as usize;
            for i in 0..8 {
                let word = if little {
                    code[i % 4].to_le_bytes()
                } else {
                    code[i % 4].to_be_bytes()
                };
                bytes[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&word);
            }
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(scan_fde_bodies(&file), [(0x1000, 0x1024)]);
            assert!(eligible_fde_bodies(&file, &bytes).is_empty());
            for i in 0..8 {
                let nop = if little {
                    0x60000000u32.to_le_bytes()
                } else {
                    0x60000000u32.to_be_bytes()
                };
                bytes[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&nop);
            }
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(eligible_fde_bodies(&file, &bytes), [(0x1000, 0x1024)]);
        }
    }
}

fn with_text(little: bool, code: &[u32]) -> Vec<u8> {
    use object::{Object, ObjectSection};
    let mut bytes = fixture::image(little, false, true, false);
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let offset = file.section_by_name(".text").unwrap().file_range().unwrap().0 as usize;
    for (i, &w) in code.iter().enumerate() {
        let word = if little { w.to_le_bytes() } else { w.to_be_bytes() };
        bytes[offset + i * 4..offset + i * 4 + 4].copy_from_slice(&word);
    }
    bytes
}

/// A shared linker FDE begins with a stub; an ordinary function whose tail is
/// stub-shaped begins with its own prologue and keeps its FDE. Absolute stubs
/// mark a shared FDE as well as r30-relative ones.
#[test]
fn only_an_fde_that_starts_with_a_stub_is_a_shared_stub_range() {
    let nop = 0x6000_0000;
    let short = [0x817e_0000, 0x7d69_03a6, 0x4e80_0420, nop];
    let absolute = [0x3d60_0000, 0x816b_3000, 0x7d69_03a6, 0x4e80_0420];
    let prologue = [0x7c08_02a6, nop, nop, nop, nop];
    for little in [false, true] {
        for (code, shared) in [
            ([absolute, absolute].concat(), true),
            ([&prologue[..], &short[..]].concat(), false),
            ([&prologue[..], &absolute[..]].concat(), false),
        ] {
            let bytes = with_text(little, &code);
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert_eq!(scan_fde_bodies(&file), [(0x1000, 0x1024)]);
            assert_eq!(
                eligible_fde_bodies(&file, &bytes).is_empty(),
                shared,
                "little={little} code={code:x?}"
            );
        }
    }
}

/// A forward `bcl 20,31` to a helper that returns through LR is a local call,
/// not a PC read: the code after the call stays in the caller.
#[test]
fn bcl_to_a_returning_helper_remains_a_call() {
    for little in [false, true] {
        let code = [
            0x7c08_02a6, // mflr r0
            0x429f_0011, // bcl 20,31,0x1014
            0x3863_0001, // addi r3,r3,1
            0x7c08_03a6, // mtlr r0
            0x4e80_0020, // blr
            0x1c63_0003, // mulli r3,r3,3
            0x3863_0007, // addi r3,r3,7
            0x4e80_0020, // blr
            0x6000_0000,
        ];
        let path = common::scratch_file("elf-bcl-helper", "elf");
        std::fs::write(&path, with_text(little, &code)).unwrap();
        let (out, err, status) = common::run_kuna(&["decompile", path.to_str().unwrap(), "bump", "--mode", "fast"]);
        std::fs::remove_file(path).unwrap();
        assert_eq!(status, 0, "{out}\n{err}");
        assert!(out.contains("+ 1") && out.contains("1014"), "little={little}: {out}");
        assert!(!out.contains("return a0 * 3 + 7;"), "little={little}: {out}");
    }
}

/// A helper that reads LR but never loads a skipped word through that register is
/// a call, whether it returns through LR, restores LR first, tail-jumps away or
/// overwrites the register before loading through it.
#[test]
fn bcl_to_a_helper_without_a_literal_load_remains_a_call() {
    let helpers: [&[u32]; 5] = [
        &[0x7c88_02a6, 0x4e80_0020], // mflr r4; blr
        &[0x7c88_02a6, 0x7c88_03a6, 0x4e80_0020], // mflr r4; mtlr r4; blr
        &[
            0x7c08_02a6, // mflr r0
            0x9421_fff0, // stwu r1,-16(r1)
            0x9001_0014, // stw r0,20(r1)
            0x3860_0007, // li r3,7
            0x8001_0014, // lwz r0,20(r1)
            0x7c08_03a6, // mtlr r0
            0x3821_0010, // addi r1,r1,16
            0x4e80_0020, // blr
        ],
        &[0x7c88_02a6, 0x4800_0000], // mflr r4; b .
        &[0x7d28_02a6, 0x3920_0000, 0x8129_0000, 0x4e80_0020], // mflr r9; li r9,0; lwz r9,0(r9); blr
    ];
    for helper in helpers {
        let tail = 0x10 + 4 * helper.len() as u32;
        let code = [
            &[
                0x7c08_02a6, // mflr r0
                0x429f_000d, // bcl 20,31,0x1010
                0x3863_0001, // addi r3,r3,1
                0x4800_0000 | tail - 0x0c,
            ][..],
            helper,
            &[0x7c08_03a6, 0x4e80_0020], // mtlr r0; blr
        ]
        .concat();
        for little in [false, true] {
            let path = common::scratch_file("elf-bcl-lr-helper", "elf");
            std::fs::write(&path, fixture::image_with_code(little, &code)).unwrap();
            let (out, err, status) = common::run_kuna(&["decompile", path.to_str().unwrap(), "bump", "--mode", "fast"]);
            std::fs::remove_file(path).unwrap();
            assert_eq!(status, 0, "{out}\n{err}");
            assert!(out.contains("sub_1010(") && out.contains("+ 1"), "little={little} helper={helper:x?}: {out}");
        }
    }
}

/// The literal load may be scheduled after unrelated instructions.
#[test]
fn pc_read_literal_load_may_follow_other_instructions() {
    let code = [
        0x7c08_02a6, // mflr r0
        0x429f_0009, // bcl 20,31,0x100c
        7,
        0x7d28_02a6, // mflr r9
        0x93e1_fffc, // stw r31,-4(r1)
        0x7c3f_0b78, // mr r31,r1
        0x1c63_0003, // mulli r3,r3,3
        0x8129_0000, // lwz r9,0(r9)
        0x7c63_4a14, // add r3,r3,r9
        0x7c08_03a6, // mtlr r0
        0x4e80_0020, // blr
    ];
    for little in [false, true] {
        let path = common::scratch_file("elf-pc-read-scheduled", "elf");
        std::fs::write(&path, fixture::image_with_code(little, &code)).unwrap();
        let (out, err, status) = common::run_kuna(&["decompile", path.to_str().unwrap(), "bump", "--mode", "fast"]);
        std::fs::remove_file(path).unwrap();
        assert_eq!(status, 0, "{out}\n{err}");
        assert!(out.contains("return a0 * 3 + 7;") && !out.contains("halt_baddata"), "little={little}: {out}");
    }
}

/// GCC may branch from several PC reads to one block that reads LR; the read
/// still counts when it is reached by a direct branch inside the body.
#[test]
fn pc_read_through_a_branch_to_a_shared_lr_read() {
    let code = [
        0x7c08_02a6, // mflr r0
        0x429f_0009, // bcl 20,31,0x100c
        7,
        0x4800_0008, // b 0x1014
        0x7fe0_0008, // trap
        0x7d28_02a6, // mflr r9
        0x8129_0000, // lwz r9,0(r9)
        0x1c63_0003, // mulli r3,r3,3
        0x7c63_4a14, // add r3,r3,r9
        0x7c08_03a6, // mtlr r0
        0x4e80_0020, // blr
    ];
    for little in [false, true] {
        let path = common::scratch_file("elf-pc-read-branch", "elf");
        std::fs::write(&path, fixture::image_with_code(little, &code)).unwrap();
        let (out, err, status) = common::run_kuna(&["decompile", path.to_str().unwrap(), "bump", "--mode", "fast"]);
        std::fs::remove_file(path).unwrap();
        assert_eq!(status, 0, "{out}\n{err}");
        assert!(out.contains("return a0 * 3 + 7;") && !out.contains("halt_baddata"), "little={little}: {out}");
    }
}
