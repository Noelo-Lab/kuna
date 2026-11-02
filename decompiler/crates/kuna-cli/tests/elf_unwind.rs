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
        let c = common::scratch_file("elf-pc-read", "c");
        let exe = common::scratch_file("elf-pc-read", "out");
        std::fs::write(&c, format!("{code}\nint main(void) {{ for(int x=-10;x<=10;x++) if(bump(x)!=x*3+7) return 1; return 0; }}\n")).unwrap();
        let out = Command::new("cc")
            .args(["-O2", c.to_str().unwrap(), "-o", exe.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(Command::new(&exe).status().unwrap().success());
        for file in [path, c, exe] {
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
